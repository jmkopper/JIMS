use rusqlite::Connection;
use std::{fs, str::FromStr};
use tiny_http::{Header, Request, Response, StatusCode};

type HTTPresponse = Response<std::io::Cursor<Vec<u8>>>;

pub fn handle(req: Request, conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    let res = match req.url() {
        "/" => html_response("static/index.html"),
        "/system_stats" => system_stats_response(conn),
        "/sensor_readings" => sensor_readings_response(conn),
        _ => Response::from_string("Not Found").with_status_code(404),
    };
    req.respond(res)?;
    Ok(())
}

fn html_response(path: &str) -> HTTPresponse {
    match fs::read_to_string(path) {
        Ok(body) => header_response(body, "text/html; charset=utf-8", 200),
        Err(_) => status(500, "Failed to read HTML"),
    }
}

fn system_stats_response(conn: &Connection) -> HTTPresponse {
    let sql = "SELECT timestamp, cpu_temp, cpu_usage, mem_usage FROM system_stats ORDER BY timestamp DESC LIMIT 1";
    let row = conn.query_row(sql, [], |r| {
        Ok((
            r.get::<_, i64>(0)?,
            r.get::<_, f64>(1)?,
            r.get::<_, f64>(2)?,
            r.get::<_, f64>(3)?,
        ))
    });

    match row {
        Ok((t, ct, cu, mu)) => {
            let json = format!(
                r#"{{"timestamp":{t},"cpu_temp":{ct:.2},"cpu_usage":{cu:.2},"mem_usage":{mu:.2}}}"#
            );
            header_response(json, "application/json", 200)
        }
        Err(_) => status(500, "DB query error"),
    }
}

fn sensor_readings_response(conn: &Connection) -> HTTPresponse {
    let sql = "SELECT timestamp, temperature, humidity FROM sensor_readings ORDER BY timestamp DESC LIMIT 1";
    let row = conn.query_row(sql, [], |r| {
        Ok((
            r.get::<_, i64>(0)?,
            r.get::<_, f64>(1)?,
            r.get::<_, f64>(2)?,
        ))
    });

    match row {
        Ok((t, temp, humidity)) => {
            let json =
                format!(r#"{{"timestamp":{t},"temperature":{temp:.2},"humidity":{humidity:.2}}}"#);
            header_response(json, "application/json", 200)
        }
        Err(_) => status(500, "DB query error"),
    }
}

fn header_response(body: impl Into<String>, mime: &str, code: u16) -> HTTPresponse {
    match Header::from_str(&format!("Content-Type: {mime}")) {
        Ok(h) => status(code, &body.into()).with_header(h),
        Err(_) => status(500, "Header error"),
    }
}

fn status(code: u16, msg: &str) -> HTTPresponse {
    Response::from_string(msg).with_status_code(StatusCode(code))
}
