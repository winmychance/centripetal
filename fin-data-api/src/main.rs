
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
                date: row.get(2).expect("date Exists"),
                time: row.get(3).expect("time Exists"),
                open: row.get(4).expect("open Exists"),
                high: row.get(5).expect("high Exists"),
                low: row.get(6).expect("low Exists"),
                close: row.get(7).expect("close Exists"),
            })
        }).expect("Map rows").enumerate().map(|(_, m)| {
            return m.expect("Record exists")
        });
        let records: Vec<RawTickerData>  = ticker_data.collect();
        let res_json = json!(records);
        con.hset::<String, String, String, ()>(ticker.clone(), "data".to_string(), res_json.to_string().clone()).expect("set");
        res_json.to_string()
    }).await;
    
    Ok(warp::reply::with_status(
        format!("{}", res_json),
        http::StatusCode::OK
    ))
}

async fn get_conn() -> Connection {
    let current_dir = std::env::current_dir().expect("Current Directory Exists");
    let db_path_obj = current_dir.parent().expect("Parent exists").join("data-to-sql");
    let db_path = db_path_obj.to_str().expect("Directory exists");
    let db_full= db_path.to_string() + "/5_minute.db.sqlite";
    let conn = Connection::open(db_full.clone()).await.expect("Connection successs");
    conn
}

async fn all_ticker_route() -> std::result::Result<impl warp::Reply, Infallible> {
    let conn = get_conn().await;
    
    let res_json = conn
    .call(move |conn| {
        let mut stmt = conn.prepare(
        "SELECT distinct ticker from raw_data;",
    ).expect("Prepared select");
    let ticker_data = stmt.query_map([], |row| {
        Ok(Ticker {
            ticker: row.get(0).expect("ticker Name Exists"),
        })
    }).expect("Map rows").enumerate().map(|(_, m)| {
        return m.expect("Record exists")