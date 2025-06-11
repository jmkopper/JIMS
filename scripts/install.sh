#!/bin/bash
set -e  # Exit on error

echo "==> Building Rust components..."
(
  cd webserver && cargo build --release
)
(
  cd stats-daemon && cargo build --release
)

echo "==> Setting up Python virtual environment..."
cd sensor-daemon
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
deactivate
cd ..

echo "==> Creating directories in /opt/jims..."
sudo mkdir -p /opt/jims/data

echo "==> Copying binaries and Python code..."
sudo cp webserver/target/release/webserver /opt/jims/
sudo cp stats-daemon/target/release/stats-daemon /opt/jims/
sudo cp -r sensor-daemon/ /opt/jims/sensor-daemon/

echo "==> Copying systemd service files..."
sudo cp systemd/jims-*.service /etc/systemd/system/

echo "==> Setting ownership to current user..."
sudo chown -R $(whoami):$(whoami) /opt/jims

echo "==> Reloading systemd daemon and enabling services..."
sudo systemctl daemon-reload
sudo systemctl enable jims-webserver.service
sudo systemctl enable jims-stats-daemon.service
sudo systemctl enable jims-sensor-daemon.service

echo "==> Starting services..."
sudo systemctl start jims-webserver.service
sudo systemctl start jims-stats-daemon.service
sudo systemctl start jims-sensor-daemon.service

echo "==> Cleaning up..."
sudo rm -r sensor-daemon/venv

echo "JIMS installation complete."
