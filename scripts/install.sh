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
INSTALL_SOD=1

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
        --with-sod)
            INSTALL_SOD=1
            shift
            ;;
        --without-sod)
            INSTALL_SOD=0
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
            echo "  --with-sod         Install with SOD support (default)"
            echo "  --without-sod      Install without SOD support"
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

# Find the lightnvr binary and SOD library
if [ -f "build/Release/lightnvr" ]; then
    BINARY_PATH="build/Release/lightnvr"
    if [ -f "build/Release/src/sod/libsod.so.1.1.9" ]; then
        LIB_PATH="build/Release/src/sod"
    fi
elif [ -f "build/Debug/lightnvr" ]; then
    BINARY_PATH="build/Debug/lightnvr"
    if [ -f "build/Debug/src/sod/libsod.so.1.1.9" ]; then
        LIB_PATH="build/Debug/src/sod"
    fi
# Also check the original expected paths
elif [ -f "build/Release/bin/lightnvr" ]; then
    BINARY_PATH="build/Release/bin/lightnvr"
    if [ -f "build/Release/lib/libsod.so.1.1.9" ]; then
        LIB_PATH="build/Release/lib"
    elif [ -f "build/Release/src/sod/libsod.so.1.1.9" ]; then
        LIB_PATH="build/Release/src/sod"
    fi
elif [ -f "build/Debug/bin/lightnvr" ]; then
    BINARY_PATH="build/Debug/bin/lightnvr"
    if [ -f "build/Debug/lib/libsod.so.1.1.9" ]; then
        LIB_PATH="build/Debug/lib"
    elif [ -f "build/Debug/src/sod/libsod.so.1.1.9" ]; then
        LIB_PATH="build/Debug/src/sod"
    fi
else
    echo "Binary not found. Please build the project first:"
    echo "./scripts/build.sh --release"
    exit 1
fi

# Check if we found the SOD library
if [ "$INSTALL_SOD" -eq 1 ] && [ -z "$LIB_PATH" ]; then
    echo "SOD library not found. Did you build with SOD support?"
    echo "Try running: ./scripts/build.sh --release --with-sod"
    exit 1
fi

# Print paths for debugging
echo "Using binary: $BINARY_PATH"
if [ "$INSTALL_SOD" -eq 1 ]; then
    echo "Using SOD library: $LIB_PATH/libsod.so.1.1.9"
fi

# Create directories
echo "Creating directories..."
mkdir -p "$PREFIX/bin"
mkdir -p "$PREFIX/lib"
mkdir -p "$CONFIG_DIR"
mkdir -p "$DATA_DIR/recordings"
mkdir -p "$DATA_DIR/www"
mkdir -p "$LOG_DIR"
mkdir -p "$RUN_DIR"

# Install binary
echo "Installing binary..."
install -m 755 "$BINARY_PATH" "$PREFIX/bin/lightnvr"

# Install SOD library if enabled
if [ "$INSTALL_SOD" -eq 1 ]; then
    echo "Installing SOD library..."
    if [ -f "$LIB_PATH/libsod.so.1.1.9" ]; then
        install -m 755 "$LIB_PATH/libsod.so.1.1.9" "$PREFIX/lib/libsod.so.1.1.9"
        ln -sf "$PREFIX/lib/libsod.so.1.1.9" "$PREFIX/lib/libsod.so.1"
        ln -sf "$PREFIX/lib/libsod.so.1" "$PREFIX/lib/libsod.so"

        # Run ldconfig to update the shared library cache
        ldconfig

        echo "SOD library installed to $PREFIX/lib/libsod.so.1.1.9"
    else
        echo "SOD library not found. Did you build with SOD support?"
        echo "Try running: ./scripts/build.sh --release --with-sod"
        exit 1
    fi
else
    echo "Skipping SOD library installation (SOD support disabled)"
fi

# Install configuration files
echo "Installing configuration files..."
if [ ! -f "$CONFIG_DIR/lightnvr.ini" ]; then
    install -m 644 config/lightnvr.ini "$CONFIG_DIR/lightnvr.ini"
    echo "Installed default configuration to $CONFIG_DIR/lightnvr.ini"
else
    echo "Configuration file already exists, not overwriting"
    install -m 644 config/lightnvr.ini "$CONFIG_DIR/lightnvr.ini.default"
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
ExecStart=$PREFIX/bin/lightnvr -c $CONFIG_DIR/lightnvr.ini -d
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
echo "Configuration file: $CONFIG_DIR/lightnvr.ini"
echo "Log file: $LOG_DIR/lightnvr.log"
echo "Web interface: http://localhost:8080 (default port)"

if [ "$INSTALL_SOD" -eq 1 ]; then
    echo ""
    echo "SOD library installed: $PREFIX/lib/libsod.so"
    echo "Object detection is enabled"
else
    echo ""
    echo "SOD library not installed"
    echo "Object detection will be disabled unless SOD is installed separately"
    echo "See docs/SOD_INTEGRATION.md for more information"
fi
