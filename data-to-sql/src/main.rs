use csv;
use rusqlite::{Connection, params};
use walkdir::WalkDir;

fn main() {
    let current_dir = std::env::current_dir().expect("Current Directory Exists");
    let parent_path_obj = current_dir.parent().expect("Parent exists").join("5-min-stocks/data/5_min/us/");
    let parent_path = parent_path_obj.to_str().expect("Directory exists");

    let 