use csv;
use rusqlite::{Connection, params};
use walkdir::WalkDir;

fn main() {
    let current_dir = std::env::current_dir().expect("Current Directory Exists");
    let parent_path_obj = current_dir.parent().expect("Parent exists").join("5-min-stocks/data/5_min/us/");
    let parent_path = parent_path_obj.to_str().expect("Directory exists");

    let conn = Connection::open("5_minute.db.sqlite").expect("Connection");
    conn.execute_batch(
        "PRAGMA journal_mode = OFF;
              PRAGMA synchronous = 0;
              PRAGMA cache_size = 1000000;
              PRAGMA locking_mode = EXCLUSIVE;
              PRAGMA temp_store = MEMORY;",
    )
    .expect("PRAGMA");
    let mut stmt = conn.prepare(
        "INSERT INTO raw_data (ticker, per, date, time, open, high, low, close) values (?1, ?2, ?3 , ?4, ?5, ?6, ?7, ?8)",
       ).expect("insert");
    for entry in WalkDir::new(parent_path.clone()) {
        let entry = entry.unwrap();
        if entry.file_type().is_dir() {
            continue
        }
        let nyse_full= entry.path().as_os_str().to_str().expect("Full path a string").to_owned() ;
        //let new_name = nyse_full.clone() + ".csv";
        //fs::rename(nyse_full.clone(), new_name.clone()).expect("fs rename");
        let mut reader = csv::Reader::from_path(nyse_full.clone()).expect("Read file");
        print!("reading {}", nyse_full);
        conn.execute(
            "Create table if not Exists raw_data (ticker text, per text, date real, time real, open real, high real, low real, close real)",
            [],
        ).expect("Table created");
        print!("read {}", nyse_full);
        for r in reader.records() {
            let r = r.expect("Record");
            stmt.execute(params![&r[0],
                &r[1],
                &r[2],
                &r[3],
                &r[4],
                &r[5],
                &r[6],
                &r[7]]).expect("INSERT");
        }
    }
}
