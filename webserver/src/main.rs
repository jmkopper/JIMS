mod handlers;
use crate::handlers::handle;
use rusqlite::Connection;
use std::env;
use tiny_http::Server;

fn main() {
    let db_path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: server <path_to_db>");
        std::process::exit(1);
    });

    let conn = match Connection::open(db_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to open DB: {e}");
            return;
        }
    };

    let server = Server::http("0.0.0.0:8080").unwrap();
    println!("Listening on http://0.0.0.0:8080");

    for req in server.incoming_requests() {
        if let Err(e) = handle(req, &conn) {
            eprintln!("Request error: {e}");
        }
    }
}
