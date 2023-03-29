
use std::convert::Infallible;

use tokio_rusqlite::Connection;
use serde::{Deserialize, Serialize};

use serde_json::json;
use warp::{Filter, http};
use redis::{ Commands};


#[derive(Debug, Serialize, Deserialize)]
struct Ticker {
    ticker: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct RawTickerData {
    ticker: String,
    per: String,
    date: f64,
    time: f64,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
}

async fn connect() -> redis::RedisResult<redis::Connection> {
    let client = redis::Client::open("redis://localhost:6379")?;
    let con = client.get_connection().expect("Get redis");
    Ok(con)
}

async fn data_route(ticker: String) -> std::result::Result<impl warp::Reply, Infallible> {
    let mut con = connect().await.expect("connected");
    let t = ticker.clone();
    if let Ok(json) = con.hget::<String, String, String>(ticker.clone(), "data".to_string()) {
        return Ok(warp::reply::with_status(
            format!("{}", json),
            http::StatusCode::OK
        ));
    }
    
    let conn = get_conn().await;
    let res_json = conn
        .call(move |conn| {
            let mut stmt = conn.prepare(
            "SELECT ticker, per, date, time, open, high, low, close from raw_data where ticker = ?1;",
        ).expect("Prepared select");
        let ticker_data = stmt.query_map([t.clone()], |row| {
            Ok(RawTickerData {
                ticker: row.get(0).expect("ticker Name Exists"),
                per: row.get(1).expect("per Exists"),