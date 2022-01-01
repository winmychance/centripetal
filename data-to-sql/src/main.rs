use csv;
use rusqlite::{Connection, params};
use walkdir::WalkDir;

fn main() {
    let current_dir = std::env::curr