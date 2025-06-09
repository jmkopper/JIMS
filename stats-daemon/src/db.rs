use crate::system_stats::SystemStats;
use rusqlite::{Connection, Result, params};
use std::error::Error;

pub fn write_system_stats(conn: &Connection, stats: SystemStats) -> Result<()> {
    let sql = "INSERT INTO system_stats (timestamp, cpu_temp, cpu_usage, mem_usage) VALUES (?1, ?2, ?3, ?4)";
    conn.execute(
        sql,
        params![
            stats.timestamp,
            stats.cpu_temp,
            stats.cpu_usage,
            stats.memory_usage
        ],
    )?;
    Ok(())
}

pub fn init_db(fp: &str) -> Result<Connection, Box<dyn Error>> {
    let conn = Connection::open(fp)?;
    let schema_sql = std::fs::read_to_string("shared/schema.sql")?;
    conn.execute_batch(&schema_sql)?;
    Ok(conn)
}
