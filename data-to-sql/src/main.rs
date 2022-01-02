use csv;
use rusqlite::{Connection, params};
use walkdir::WalkDir;

fn main() {
    let current_dir = std::env::current_dir().expect("Current Directo