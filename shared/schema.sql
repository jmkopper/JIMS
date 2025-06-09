CREATE TABLE IF NOT EXISTS system_stats (
    id INTEGER PRIMARY KEY,
    timestamp INTEGER,
    cpu_temp FLOAT,
    cpu_usage FLOAT,
    mem_usage FLOAT
);

CREATE TABLE IF NOT EXISTS sensor_readings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp INTEGER,
    temperature REAL,
    humidity REAL
);

CREATE INDEX IF NOT EXISTS idx_sensor_time ON sensor_readings(timestamp);
CREATE INDEX IF NOT EXISTS idx_stats_time ON system_stats(timestamp);
