#!/bin/bash
# Build and install the go2rtc fork pinned by this LightNVR checkout.
# Upstream binaries do not implement LightNVR's media authority protocol.

set -e

# Default values
INSTALL_DIR="/usr/local/bin"
CONFIG_DIR="/etc/lightnvr/go2rtc"
ARCH=$(uname -m)
SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
GO2RTC_SOURCE_DIR="$SCRIPT_DIR/../go2rtc"

# Display usage information
usage() {
    echo "Usage: $0 [options]"
    echo "Options:"
    echo "  -d, --install-dir DIR   Installation directory for go2rtc binary (default: $INSTALL_DIR)"
    echo "  -c, --config-dir DIR    Configuration directory for go2rtc (default: $CONFIG_DIR)"
    echo "  -a, --arch ARCH         Architecture (default: auto-detected, current: $ARCH)"
    echo "  Requires Go and an initialized go2rtc submodule."
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

# Map the requested target to Go's cross-compilation settings.
case $ARCH in
    x86_64|amd64)
        GO2RTC_GOARCH="amd64"
        ;;
    aarch64|arm64)
        GO2RTC_GOARCH="arm64"
        ;;
    arm|armv7|armv7l|armhf)
        GO2RTC_GOARCH="arm"
        GO2RTC_GOARM="7"
        ;;
    *)
        echo "Unsupported architecture: $ARCH"
        echo "Please specify a supported architecture with --arch"
        exit 1
        ;;
esac

if [ ! -f "$GO2RTC_SOURCE_DIR/go.mod" ] ||
   [ ! -f "$GO2RTC_SOURCE_DIR/internal/mediaauth/lease.go" ]; then
    echo "Compatible go2rtc source is missing. Run: git submodule update --init go2rtc" >&2
    exit 1
fi
if ! command -v go >/dev/null 2>&1; then
    echo "Go is required to build the pinned go2rtc fork." >&2
    exit 1
fi

echo "Building the pinned LightNVR go2rtc fork..."
echo "  Target: linux/$GO2RTC_GOARCH${GO2RTC_GOARM:+ (GOARM=$GO2RTC_GOARM)}"
echo "  Install directory: $INSTALL_DIR"
echo "  Config directory: $CONFIG_DIR"

GO2RTC_BUILD_DIR=$(mktemp -d)
GO2RTC_INSTALL_TEMP=""
trap 'rm -rf "$GO2RTC_BUILD_DIR"; if [ -n "$GO2RTC_INSTALL_TEMP" ]; then rm -f "$GO2RTC_INSTALL_TEMP"; fi' EXIT
(
    cd "$GO2RTC_SOURCE_DIR"
    GOOS=linux GOARCH="$GO2RTC_GOARCH" GOARM="${GO2RTC_GOARM:-}" CGO_ENABLED=0 \
        go build -trimpath -ldflags="-s -w" -o "$GO2RTC_BUILD_DIR/go2rtc" .
)
mkdir -p "$INSTALL_DIR" "$CONFIG_DIR"
GO2RTC_INSTALL_TEMP="$INSTALL_DIR/.go2rtc.$$.tmp"
install -m 0755 "$GO2RTC_BUILD_DIR/go2rtc" "$GO2RTC_INSTALL_TEMP"
mv -f "$GO2RTC_INSTALL_TEMP" "$INSTALL_DIR/go2rtc"
GO2RTC_INSTALL_TEMP=""

# Create basic configuration file if it doesn't exist
if [ ! -f "$CONFIG_DIR/go2rtc.yaml" ]; then
    cat > "$CONFIG_DIR/go2rtc.yaml" << EOF
# go2rtc configuration file
# See https://github.com/opensensor/go2rtc for documentation (fork with memory fixes)

api:
  listen: 127.0.0.1:1984
  base_path: /go2rtc

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
