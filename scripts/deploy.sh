#!/bin/bash
set -e  # Exit on error

echo "==> Building Rust components..."
(
  cd webserver && cargo build --release
)
(
  cd stats-daemon && cargo build --release
)

echo "==> Syncing binaries and Python code to /opt/jims..."
sudo cp webserver/target/release/webserver /opt/jims/
sudo cp stats-daemon/target/release/stats-daemon /opt/jims/
sudo rsync -a --exclude 'venv' sensor-daemon/ /opt/jims/sensor-daemon/

echo "==> Restarting systemd services..."
sudo systemctl restart jims-webserver.service
sudo systemctl restart jims-stats-daemon.service
sudo systemctl restart jims-sensor-daemon.service

echo "JIMS deployment complete."
