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
INSTALL_GO2RTC=1
GO2RTC_CONFIG_DIR="/etc/lightnvr/go2rtc"
DO_LDCONFIG=1
INSTALL_SYSTEMD_SERVICE=1

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
        --with-ldconfig)
            DO_LDCONFIG=1
            shift
            ;;
        --without-ldconfig)
            DO_LDCONFIG=0
            shift
            ;;
        --with-systemd)
            INSTALL_SYSTEMD_SERVICE=1
            shift
            ;;
        --without-systemd)
            INSTALL_SYSTEMD_SERVICE=0
            shift
            ;;
        --with-go2rtc)
            INSTALL_GO2RTC=1
            shift
            ;;
        --without-go2rtc)
            INSTALL_GO2RTC=0
            shift
            ;;
        --go2rtc-config-dir=*)
            GO2RTC_CONFIG_DIR="${key#*=}"
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
            echo "  --with-ldconfig    Run ldconfig after installing SOD library (default)"
            echo "  --without-ldconfig Skip running ldconfig after installing SOD library"
            echo "  --with-systemd     Install systemd service (default)"
            echo "  --without-systemd  Skip installing systemd service"
            echo "  --with-go2rtc      Install with go2rtc support (default)"
            echo "  --without-go2rtc   Install without go2rtc support"
            echo "  --go2rtc-config-dir=DIR  Set go2rtc config directory (default: /etc/lightnvr/go2rtc)"
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

# Search for the lightnvr binary first
echo "Searching for LightNVR binary..."
BINARY_PATH=""
POSSIBLE_BINARY_PATHS=(
    "lightnvr"
    "build/Release/bin/lightnvr"
    "build/Debug/bin/lightnvr"
    "build/Release/lightnvr"
    "build/Debug/lightnvr"
    "build/bin/lightnvr"
    "build/lightnvr"
)

for path in "${POSSIBLE_BINARY_PATHS[@]}"; do
    if [ -f "$path" ]; then
        BINARY_PATH="$path"
        echo "Found binary at: $BINARY_PATH"
        break
    fi
done

if [ -z "$BINARY_PATH" ]; then
    echo "Binary not found. Please build the project first:"
    echo "./scripts/build.sh --release"
    exit 1
fi

# Search for the SOD library
echo "Searching for SOD library..."
LIB_PATH=""
SOD_LIB_NAME=""
POSSIBLE_LIB_PATHS=(
    "build/Release/src/sod"
    "build/Debug/src/sod"
    "build/Release/lib"
    "build/Debug/lib"
    "build/src/sod"
    "build/lib"
    "src/sod"
)

for path in "${POSSIBLE_LIB_PATHS[@]}"; do
    if [ -f "$path/libsod.so.1.1.9" ]; then
        LIB_PATH="$path"
        SOD_LIB_NAME="libsod.so.1.1.9"
        echo "Found SOD library at: $LIB_PATH/$SOD_LIB_NAME"
        break
    elif [ -f "$path/libsod.so.1" ]; then
        LIB_PATH="$path"
        SOD_LIB_NAME="libsod.so.1"
        echo "Found SOD library at: $LIB_PATH/$SOD_LIB_NAME"
        break
    elif [ -f "$path/libsod.so" ]; then
        LIB_PATH="$path"
        SOD_LIB_NAME="libsod.so"
        echo "Found SOD library at: $LIB_PATH/$SOD_LIB_NAME"
        break
    fi
done

# Print detailed file information for better debugging
if [ -n "$LIB_PATH" ]; then
    echo "SOD library details:"
    ls -la "$LIB_PATH"/libsod.so*

    # Check if it's a symlink and where it points
    if [ -L "$LIB_PATH/$SOD_LIB_NAME" ]; then
        echo "SOD library is a symlink pointing to: $(readlink -f "$LIB_PATH/$SOD_LIB_NAME")"
    else
        echo "SOD library is a regular file"

        # Check file type
        file "$LIB_PATH/$SOD_LIB_NAME"
    fi
else
    echo "No SOD library found in standard locations."
    echo "Performing deep search for libsod.so files..."

    # Do a more exhaustive search
    SOD_FILES=$(find . -name "libsod.so*" -type f -o -type l | sort)

    if [ -n "$SOD_FILES" ]; then
        echo "Found SOD library files:"
        echo "$SOD_FILES"

        # Use the first one found
        FIRST_SOD=$(echo "$SOD_FILES" | head -n 1)
        LIB_PATH=$(dirname "$FIRST_SOD")
        SOD_LIB_NAME=$(basename "$FIRST_SOD")
        echo "Using SOD library: $LIB_PATH/$SOD_LIB_NAME"
    fi
fi

# Check if we found the SOD library when it's required
if [ "$INSTALL_SOD" -eq 1 ] && [ -z "$LIB_PATH" ]; then
    echo "SOD library not found. Did you build with SOD support?"
    echo "Try running: ./scripts/build.sh --release --with-sod"
    exit 1
fi

# Print paths for debugging
echo "Using binary: $BINARY_PATH"
if [ "$INSTALL_SOD" -eq 1 ]; then
    echo "Using SOD library: $LIB_PATH/$SOD_LIB_NAME"
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

    # Install all libsod.so files found in the directory
    for sod_file in "$LIB_PATH"/libsod.so*; do
        if [ -f "$sod_file" ]; then
            sod_filename=$(basename "$sod_file")
            echo "Installing $sod_filename..."
            install -m 755 "$sod_file" "$PREFIX/lib/$sod_filename"
        fi
    done

    # Create symlinks if needed (if we installed a versioned library but not the unversioned one)
    if [ -f "$PREFIX/lib/libsod.so.1.1.9" ] && [ ! -f "$PREFIX/lib/libsod.so" ]; then
        echo "Creating symlinks for libsod.so..."
        ln -sf "$PREFIX/lib/libsod.so.1.1.9" "$PREFIX/lib/libsod.so.1"
        ln -sf "$PREFIX/lib/libsod.so.1" "$PREFIX/lib/libsod.so"
    elif [ -f "$PREFIX/lib/libsod.so.1" ] && [ ! -f "$PREFIX/lib/libsod.so" ]; then
        echo "Creating symlink for libsod.so..."
        ln -sf "$PREFIX/lib/libsod.so.1" "$PREFIX/lib/libsod.so"
    elif [ -f "$PREFIX/lib/libsod.so" ] && [ ! -f "$PREFIX/lib/libsod.so.1.1.9" ]; then
        # If we only have libsod.so, create the versioned symlinks
        echo "Creating versioned symlinks for libsod.so..."
        ln -sf "$PREFIX/lib/libsod.so" "$PREFIX/lib/libsod.so.1"
        ln -sf "$PREFIX/lib/libsod.so.1" "$PREFIX/lib/libsod.so.1.1.9"
    fi

    # Run ldconfig to update the shared library cache
    # we can skip with --without-ldconfig
    if [ "$DO_LDCONFIG" -eq 1 ]; then
        echo "Running ldconfig..."
        ldconfig
    else
        echo "Skipping ldconfig (--without-ldconfig)"
    fi

    echo "SOD library installed to $PREFIX/lib"
    ls -la "$PREFIX/lib"/libsod.so*
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

# Install go2rtc configuration if enabled
if [ "$INSTALL_GO2RTC" -eq 1 ]; then
    echo "Setting up go2rtc configuration..."
    mkdir -p "$GO2RTC_CONFIG_DIR"
    
    # Create default go2rtc configuration if it doesn't exist
    if [ ! -f "$GO2RTC_CONFIG_DIR/go2rtc.yaml" ]; then
        cat > "$GO2RTC_CONFIG_DIR/go2rtc.yaml" << EOF
# go2rtc configuration file
# See https://github.com/AlexxIT/go2rtc for documentation

api:
  listen: :1984
  origin: "*"

webrtc:
  ice_servers:
    - urls: [stun:stun.l.google.com:19302]

log:
  level: info

streams:
  # Streams will be added dynamically by LightNVR
EOF
        echo "Created default go2rtc configuration at $GO2RTC_CONFIG_DIR/go2rtc.yaml"
    else
        echo "go2rtc configuration already exists, not overwriting"
    fi
    
    # Set permissions for go2rtc configuration
    chown -R root:root "$GO2RTC_CONFIG_DIR"
    chmod -R 755 "$GO2RTC_CONFIG_DIR"
    
    echo "go2rtc configuration directory: $GO2RTC_CONFIG_DIR"
    echo "Note: You need to install the go2rtc binary separately using scripts/install_go2rtc.sh"
else
    echo "Skipping go2rtc configuration (go2rtc support disabled)"
fi

# Check if web directory exists
if [ -d "web" ]; then
    # Install web interface files
    echo "Installing web interface files..."

    # Check if dist directory exists (prebuilt assets)
    if [ -d "web/dist" ] && [ -f "web/dist/index.html" ]; then
        echo "Found prebuilt web assets, installing from dist directory..."
        cp -r web/dist/* "$DATA_DIR/www/"
        echo "Web interface files installed to $DATA_DIR/www/"

        # Verify installation
        if [ -f "$DATA_DIR/www/index.html" ]; then
            echo "✓ Web assets successfully installed"
        else
            echo "⚠ Warning: Web assets may not have been installed correctly"
        fi
    else
        echo "⚠ WARNING: No prebuilt web assets found in web/dist/"
        echo ""
        echo "The web interface will NOT work without building the assets first!"
        echo ""
        echo "To build web assets, run:"
        echo "  cd web"
        echo "  npm install"
        echo "  npm run build"
        echo "  cd .."
        echo "  sudo bash scripts/install.sh"
        echo ""
        echo "Or use the dedicated script:"
        echo "  sudo bash scripts/install_web_assets.sh"
        echo ""
        read -p "Do you want to continue installation without web assets? (y/N) " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            echo "Installation cancelled. Please build web assets first."
            exit 1
        fi
        echo "Continuing without web assets..."
    fi
else
    echo "⚠ WARNING: Web interface directory not found, skipping web installation"
    echo "The web interface will NOT be available!"
fi

# Set permissions
echo "Setting permissions..."
chown -R root:root "$PREFIX/bin/lightnvr"
chown -R root:root "$CONFIG_DIR"
chown -R root:root "$DATA_DIR"
chown -R root:root "$LOG_DIR"
chown -R root:root "$RUN_DIR"

# skip systemd if --without-systemd flag set
if [ "$INSTALL_SYSTEMD_SERVICE" -eq 0 ]; then
    echo "Skipping systemd service installation (--without-systemd)"
    exit 0
else
    # Create systemd service file
    echo "Creating systemd service file..."
    cat > /etc/systemd/system/lightnvr.service << EOF
[Unit]
Description=LightNVR - Lightweight Network Video Recorder
After=network.target network-online.target
Wants=network-online.target

[Service]
Type=forking
PIDFile=/var/run/lightnvr.pid
User=root
Group=root
# Set environment variables for proper operation
Environment="PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"
Environment="HOME=/root"
Environment="LD_LIBRARY_PATH=/usr/local/lib:/usr/lib:/lib"
Environment="FFMPEG_DATADIR=/usr/share/ffmpeg"
# Do NOT set WorkingDirectory - daemon code handles this to avoid SQLite issues
# Do NOT set PWD environment variable - let the daemon determine the working directory
ExecStart=$PREFIX/bin/lightnvr -c $CONFIG_DIR/lightnvr.ini -d
ExecReload=/bin/kill -HUP \$MAINPID
KillMode=mixed
KillSignal=SIGTERM
TimeoutStartSec=30
TimeoutStopSec=30
Restart=on-failure
RestartSec=5
# Ensure proper permissions for directories
ExecStartPre=/bin/mkdir -p $DATA_DIR $LOG_DIR $RUN_DIR
ExecStartPre=/bin/chown root:root $DATA_DIR $LOG_DIR $RUN_DIR
ExecStartPre=/bin/chmod 755 $DATA_DIR $LOG_DIR $RUN_DIR
# Log to journal for better debugging
StandardOutput=journal
StandardError=journal
SyslogIdentifier=lightnvr

[Install]
WantedBy=multi-user.target
EOF

    # Reload systemd
    echo "Reloading systemd..."
    systemctl daemon-reload
fi

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

    # Verify if the library is properly linked to the binary
    echo ""
    echo "Checking if lightnvr is linked to libsod.so..."
    if ldd "$PREFIX/bin/lightnvr" | grep -q "libsod.so"; then
        echo "SUCCESS: lightnvr is correctly linked to libsod.so"
    else
        echo "WARNING: lightnvr is not linked to libsod.so"
        echo "This may indicate a problem with dynamic linking."
        echo "LightNVR will attempt to load the library at runtime."
    fi
else
    echo ""
    echo "SOD library not installed"
    echo "Object detection will be disabled unless SOD is installed separately"
    echo "See docs/SOD_INTEGRATION.md for more information"
fi

if [ "$INSTALL_GO2RTC" -eq 1 ]; then
    echo ""
    echo "go2rtc integration is enabled"
    echo "Configuration directory: $GO2RTC_CONFIG_DIR"
    echo ""
    echo "To install the go2rtc binary:"
    echo "  sudo ./scripts/install_go2rtc.sh"
    echo ""
    echo "For more information about go2rtc integration:"
    echo "  See docs/GO2RTC_INTEGRATION.md"
else
    echo ""
    echo "go2rtc integration is disabled"
    echo "WebRTC streaming will not be available"
fi
