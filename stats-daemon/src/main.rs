mod db;
mod system_stats;
use rusqlite::Result;
use std::{env, error::Error, thread, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
    let db_path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: server <path_to_db>");
        std::process::exit(1);
    });

    let conn = db::init_db(&db_path)?;

    loop {
        let sys_stats = system_stats::SystemStats::current();
        if let Err(err) = db::write_system_stats(&conn, sys_stats) {
            eprintln!("Failed to write to DB: {:?}", err);
        }
        thread::sleep(Duration::from_secs(60));
    }
}
