#!/bin/bash
set -e  # Exit on error

echo "==> Stopping services..."
sudo systemctl stop jims-sensor-daemon.service
sudo systemctl stop jims-stats-daemon.service
sudo systemctl stop jims-webserver.service
sudo systemctl disable jims-sensor-daemon.service
sudo systemctl disable jims-stats-daemon.service
sudo systemctl disable jims-webserver.service


echo "==> Removing files..."
sudo rm /etc/systemd/system/jims-sensor-daemon.service
sudo rm /etc/systemd/system/jims-stats-daemon.service
sudo rm /etc/systemd/system/jims-webserver.service

sudo rm -r /opt/jims/

echo "==> Updating systemd..."
sudo systemctl daemon-reload
sudo systemctl reset-failed


systemctl disable [servicename]
rm /etc/systemd/system/[servicename]
rm /etc/systemd/system/[servicename] # and symlinks that might be related
rm /usr/lib/systemd/system/[servicename] 
rm /usr/lib/systemd/system/[servicename] # and symlinks that might be related
systemctl daemon-reload
systemctl reset-failed

echo "JIMS uninstalled."
