#!/bin/bash

# LightNVR Installation Script

# Exit on error
set -e

# Default installation prefix
PREFIX="/usr/local"
CONFIG_DIR="/etc/lightnvr"
DATA_DIR="/var/lib/lightnvr"
LOG_DIR="/var/log/lightnvr"
RUN_DIR="/var/run/lightnvr"

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    key="$1"
    case $key in
        --prefix=*)
            PREFIX="${key#*=}"
            shift
            ;;
        --config-dir=*)
            CONFIG_DIR="${key#*=}"
            shift
            ;;
        --data-dir=*)
            DATA_DIR="${key#*=}"
            shift
            ;;
        --log-dir=*)
            LOG_DIR="${key#*=}"
            shift
            ;;
        --run-dir=*)
            RUN_DIR="${key#*=}"
            shift
            ;;
        --help)
            echo "Usage: $0 [options]"
            echo "Options:"
            echo "  --prefix=DIR       Installation prefix (default: /usr/local)"
            echo "  --config-dir=DIR   Configuration directory (default: /etc/lightnvr)"
            echo "  --data-dir=DIR     Data directory (default: /var/lib/lightnvr)"
            echo "  --log-dir=DIR      Log directory (default: /var/log/lightnvr)"
            echo "  --run-dir=DIR      Run directory (default: /var/run/lightnvr)"
            echo "  --help             Show this help message"
            exit 0
            ;;
        *)
            echo "Unknown option: $key"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Check if running as root
if [ "$(id -u)" -ne 0 ]; then
    echo "This script must be run as root"
    exit 1
fi

# Check if the binary exists
if [ ! -f "build/Release/lightnvr" ]; then
    echo "Binary not found. Please build the project first:"
    echo "./scripts/build.sh --release"
    exit 1
fi

# Create directories
echo "Creating directories..."
mkdir -p "$PREFIX/bin"
mkdir -p "$CONFIG_DIR"
mkdir -p "$DATA_DIR/recordings"
mkdir -p "$DATA_DIR/www"
mkdir -p "$LOG_DIR"
mkdir -p "$RUN_DIR"

# Install binary
echo "Installing binary..."
install -m 755 build/Release/lightnvr "$PREFIX/bin/lightnvr"

# Install configuration files
echo "Installing configuration files..."
if [ ! -f "$CONFIG_DIR/lightnvr.conf" ]; then
    install -m 644 config/lightnvr.conf.default "$CONFIG_DIR/lightnvr.conf"
    echo "Installed default configuration to $CONFIG_DIR/lightnvr.conf"
else
    echo "Configuration file already exists, not overwriting"
    install -m 644 config/lightnvr.conf.default "$CONFIG_DIR/lightnvr.conf.default"
fi

# Install web interface files
echo "Installing web interface files..."
cp -r web/* "$DATA_DIR/www/"

# Set permissions
echo "Setting permissions..."
chown -R root:root "$PREFIX/bin/lightnvr"
chown -R root:root "$CONFIG_DIR"
chown -R root:root "$DATA_DIR"
chown -R root:root "$LOG_DIR"
chown -R root:root "$RUN_DIR"

# Create systemd service file
echo "Creating systemd service file..."
cat > /etc/systemd/system/lightnvr.service << EOF
[Unit]
Description=LightNVR - Lightweight Network Video Recorder
After=network.target

[Service]
Type=forking
PIDFile=/var/run/lightnvr.pid
Environment="PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"
Environment="HOME=/root"
Environment="PWD=/var/lib/lightnvr"
WorkingDirectory=/var/lib/lightnvr
ExecStart=/usr/local/bin/lightnvr -c /etc/lightnvr/lightnvr.conf -d
Restart=on-failure
RestartSec=5
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
EOF

# Reload systemd
echo "Reloading systemd..."
systemctl daemon-reload

echo "Installation completed successfully!"
echo ""
echo "To start LightNVR:"
echo "  systemctl start lightnvr"
echo ""
echo "To enable LightNVR to start at boot:"
echo "  systemctl enable lightnvr"
echo ""
echo "To check LightNVR status:"
echo "  systemctl status lightnvr"
echo ""
echo "Configuration file: $CONFIG_DIR/lightnvr.conf"
echo "Log file: $LOG_DIR/lightnvr.log"
echo "Web interface: http://localhost:8080 (default port)"
