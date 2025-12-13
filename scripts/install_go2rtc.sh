#!/bin/bash
# Script to download and install go2rtc for WebRTC streaming
# Uses opensensor/go2rtc fork with memory leak fixes

set -e

# Default values
INSTALL_DIR="/usr/local/bin"
CONFIG_DIR="/etc/lightnvr/go2rtc"
VERSION="latest"
ARCH=$(uname -m)
# Use opensensor fork by default (includes memory leak fixes)
REPO="opensensor/go2rtc"

# Display usage information
usage() {
    echo "Usage: $0 [options]"
    echo "Options:"
    echo "  -d, --install-dir DIR   Installation directory for go2rtc binary (default: $INSTALL_DIR)"
    echo "  -c, --config-dir DIR    Configuration directory for go2rtc (default: $CONFIG_DIR)"
    echo "  -v, --version VERSION   Version to install (default: latest)"
    echo "  -a, --arch ARCH         Architecture (default: auto-detected, current: $ARCH)"
    echo "  -h, --help              Display this help message"
    exit 1
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -d|--install-dir)
            INSTALL_DIR="$2"
            shift 2
            ;;
        -c|--config-dir)
            CONFIG_DIR="$2"
            shift 2
            ;;
        -v|--version)
            VERSION="$2"
            shift 2
            ;;
        -a|--arch)
            ARCH="$2"
            shift 2
            ;;
        -h|--help)
            usage
            ;;
        *)
            echo "Unknown option: $1"
            usage
            ;;
    esac
done

# Map architecture to go2rtc naming convention
case $ARCH in
    x86_64|amd64)
        GO2RTC_ARCH="amd64"
        ;;
    aarch64|arm64)
        GO2RTC_ARCH="arm64"
        ;;
    armv7l|armhf)
        GO2RTC_ARCH="arm"
        ;;
    *)
        echo "Unsupported architecture: $ARCH"
        echo "Please specify a supported architecture with --arch"
        exit 1
        ;;
esac

echo "Installing go2rtc..."
echo "  Architecture: $GO2RTC_ARCH"
echo "  Version: $VERSION"
echo "  Install directory: $INSTALL_DIR"
echo "  Config directory: $CONFIG_DIR"

# Create directories if they don't exist
mkdir -p "$INSTALL_DIR"
mkdir -p "$CONFIG_DIR"

# Determine download URL
if [ "$VERSION" = "latest" ]; then
    # Get latest release URL from opensensor fork
    RELEASE_URL=$(curl -s https://api.github.com/repos/$REPO/releases/latest | grep "browser_download_url.*linux_$GO2RTC_ARCH" | cut -d '"' -f 4)
    if [ -z "$RELEASE_URL" ]; then
        echo "Failed to determine latest release URL from $REPO."
        echo "Falling back to upstream AlexxIT/go2rtc..."
        RELEASE_URL=$(curl -s https://api.github.com/repos/AlexxIT/go2rtc/releases/latest | grep "browser_download_url.*linux_$GO2RTC_ARCH" | cut -d '"' -f 4)
        if [ -z "$RELEASE_URL" ]; then
            echo "Failed to determine latest release URL. Please specify a version."
            exit 1
        fi
    fi
else
    # Construct URL for specific version from opensensor fork
    RELEASE_URL="https://github.com/$REPO/releases/download/$VERSION/go2rtc_linux_$GO2RTC_ARCH"
fi

echo "Downloading from: $RELEASE_URL"

# Download go2rtc binary
curl -L -o "$INSTALL_DIR/go2rtc" "$RELEASE_URL"

# Make binary executable
chmod +x "$INSTALL_DIR/go2rtc"

# Create basic configuration file if it doesn't exist
if [ ! -f "$CONFIG_DIR/go2rtc.yaml" ]; then
    cat > "$CONFIG_DIR/go2rtc.yaml" << EOF
# go2rtc configuration file
# See https://github.com/opensensor/go2rtc for documentation (fork with memory fixes)

api:
  listen: :1984
  base_path: /go2rtc/

webrtc:
  ice_servers:
    - urls: [stun:stun.l.google.com:19302]

log:
  level: info

streams:
  # Streams will be added dynamically by LightNVR
EOF
    echo "Created default configuration file: $CONFIG_DIR/go2rtc.yaml"
fi

echo "go2rtc installation complete!"
echo "Binary installed to: $INSTALL_DIR/go2rtc"
echo "Configuration directory: $CONFIG_DIR"
echo ""
echo "To use go2rtc with LightNVR, configure the following CMake options:"
echo "  -DGO2RTC_BINARY_PATH=$INSTALL_DIR/go2rtc"
echo "  -DGO2RTC_CONFIG_DIR=$CONFIG_DIR"
echo ""
echo "Note: You may need to run this script with sudo to install to system directories."
