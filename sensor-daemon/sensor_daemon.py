import sqlite3
import sys
import time

import adafruit_sht4x
import board

READ_INTERVAL = 60  # seconds
BATCH_SIZE = 1


def init_db(path):
    conn = sqlite3.connect(path)
    with open("shared/schema.sql", "r") as f:
        conn.executescript(f.read())
    return conn


def main():
    if len(sys.argv) < 2:
        print("Usage: sensor_daemon <sqlite3 filename>")
        sys.exit(1)

    db_path = sys.argv[1]
    conn = init_db(db_path)
    cursor = conn.cursor()

    i2c = board.I2C()
    sht = adafruit_sht4x.SHT4x(i2c)
    print(f"Found SHT4x serial number {hex(sht.serial_number)}")

    readings = []

    try:
        while True:
            temperature, humidity = sht.measurements
            timestamp = int(time.time())
            readings.append((timestamp, temperature, humidity))

            if len(readings) >= BATCH_SIZE:
                cursor.executemany(
                    "INSERT INTO sensor_readings (timestamp, temperature, humidity) VALUES (?, ?, ?)",
                    readings,
                )
                conn.commit()
                readings.clear()

            time.sleep(READ_INTERVAL)
    except KeyboardInterrupt:
        print("Shutting down...")
    finally:
        conn.close()


if __name__ == "__main__":
    main()
