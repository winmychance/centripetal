
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