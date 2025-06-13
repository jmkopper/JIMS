# Jay's Incredible Monitoring Service (JIMS)

Monitoring service for a Raspberry Pi with an [Adafruit SHT41](https://www.adafruit.com/product/5776) temperature and humidity sensor. Builds a sqlite database and logs temperature, humidity, cpu temperature, cpu usage, and memory usage every minute. JIMS also runs a thin HTTP web server to display the most recent stats.

## Installation

JIMS is meant to be installed with the `install.sh` shell script. Manual installation involves building the rust binaries, copying everything to `/opt/jims/` and then manually creating `systemd` services.
