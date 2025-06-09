use std::error::Error;
use std::fs;
use std::time::SystemTime;

#[derive(Debug)]
pub struct SystemStats {
    pub timestamp: i64,
    pub cpu_temp: f64,
    pub cpu_usage: f64,
    pub memory_usage: f64,
}

impl SystemStats {
    pub fn current() -> Self {
        SystemStats {
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .map(|d| d.as_secs() as i64)
                .unwrap_or(0),
            cpu_temp: get_cpu_temp().unwrap_or(-1.0),
            cpu_usage: get_cpu_usage().unwrap_or(-1.0),
            memory_usage: get_memory_usage().unwrap_or(-1.0),
        }
    }
}

fn get_cpu_temp() -> Result<f64, Box<dyn Error>> {
    let temp_str = fs::read_to_string("/sys/class/thermal/thermal_zone0/temp")?;
    let temp: f64 = temp_str.trim().parse()?;
    Ok(temp / 1000.0)
}

fn get_cpu_usage() -> Result<f64, Box<dyn Error>> {
    let stat = fs::read_to_string("/proc/stat")?;
    let first_line = stat.lines().next().ok_or("No data in /proc/stat")?;
    let parts: Vec<u64> = first_line
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap_or(0))
        .collect();

    if parts.len() < 4 {
        return Err("Invalid data in /proc/stat".into());
    }

    let total_time = parts.iter().sum::<u64>();
    let idle_time = parts[3];
    let usage = 100.0 * (total_time - idle_time) as f64 / total_time as f64;
    Ok(usage)
}

fn get_memory_usage() -> Result<f64, Box<dyn Error>> {
    let meminfo = fs::read_to_string("/proc/meminfo")?;
    let mut total_memory = 0;
    let mut free_memory = 0;

    for line in meminfo.lines() {
        if line.starts_with("MemTotal:") {
            total_memory = line
                .split_whitespace()
                .nth(1)
                .ok_or("Unable to parse MemTotal")?
                .parse::<u64>()?;
        } else if line.starts_with("MemFree:") {
            free_memory = line
                .split_whitespace()
                .nth(1)
                .ok_or("Unable to parse MemFree")?
                .parse::<u64>()?;
        }
    }

    if total_memory == 0 {
        return Err("Invalid MemTotal value".into());
    }

    let used_memory = total_memory - free_memory;
    Ok((used_memory as f64 / total_memory as f64) * 100.0)
}
