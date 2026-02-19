#!/bin/bash
# RTSP Test Stream Setup for LightNVR Load Testing
# Streams 8 channels via MediaMTX + FFmpeg on 192.168.50.153
# Requires: ffmpeg, mediamtx (auto-downloaded if missing)

set -e

MEDIAMTX_VERSION="v1.9.3"
MEDIAMTX_DIR="/opt/mediamtx"
MEDIAMTX_BIN="$MEDIAMTX_DIR/mediamtx"
MEDIAMTX_CONF="$MEDIAMTX_DIR/mediamtx.yml"
IP="192.168.50.153"
RTSP_PORT="8554"
LOG_DIR="/tmp/rtsp_streams"

mkdir -p "$LOG_DIR"

# ── 1. Install ffmpeg if missing ──────────────────────────────────────────────
if ! command -v ffmpeg &>/dev/null; then
    echo "[*] Installing ffmpeg..."
    sudo apt-get update -qq && sudo apt-get install -y -qq ffmpeg
fi

# ── 2. Download MediaMTX if missing ──────────────────────────────────────────
if [ ! -f "$MEDIAMTX_BIN" ]; then
    echo "[*] Downloading MediaMTX $MEDIAMTX_VERSION..."
    ARCH=$(uname -m)
    case $ARCH in
        x86_64)  ARCH_LABEL="amd64" ;;
        aarch64) ARCH_LABEL="arm64v8" ;;
        armv7*)  ARCH_LABEL="armv7" ;;
        *)       echo "Unknown arch: $ARCH"; exit 1 ;;
    esac
    TMP=$(mktemp -d)
    URL="https://github.com/bluenviron/mediamtx/releases/download/${MEDIAMTX_VERSION}/mediamtx_${MEDIAMTX_VERSION}_linux_${ARCH_LABEL}.tar.gz"
    echo "[*] Fetching: $URL"
    wget -q --show-progress -O "$TMP/mediamtx.tar.gz" "$URL"
    sudo mkdir -p "$MEDIAMTX_DIR"
    sudo tar -xzf "$TMP/mediamtx.tar.gz" -C "$MEDIAMTX_DIR"
    rm -rf "$TMP"
    echo "[*] MediaMTX installed to $MEDIAMTX_DIR"
fi

# ── 3. Write MediaMTX config ──────────────────────────────────────────────────
sudo tee "$MEDIAMTX_CONF" > /dev/null <<EOF
logLevel: warn
rtsp: yes
rtspAddress: :${RTSP_PORT}
rtspsAddress: :8322
rtpAddress: :8000
rtcpAddress: :8001
hlsAddress: :8888
webrtcAddress: :8889
apiAddress: :9997

paths:
  all:
    source: publisher
EOF

# ── 4. Start MediaMTX ─────────────────────────────────────────────────────────
if pgrep -f mediamtx &>/dev/null; then
    echo "[*] MediaMTX already running, restarting..."
    pkill -f mediamtx || true
    sleep 1
fi

echo "[*] Starting MediaMTX..."
nohup sudo "$MEDIAMTX_BIN" "$MEDIAMTX_CONF" > "$LOG_DIR/mediamtx.log" 2>&1 &
sleep 2
echo "[+] MediaMTX PID: $(pgrep -f mediamtx)"

# ── 5. Stream definitions ─────────────────────────────────────────────────────
# Format: "name|resolution|fps|bitrate|pattern"
# Patterns use ffmpeg lavfi test sources for variety
declare -a STREAMS=(
    "cam01|1920x1080|15|1500k|testsrc2=size=1920x1080:rate=15"
    "cam02|1280x720|15|800k|testsrc2=size=1280x720:rate=15"
    "cam03|1920x1080|25|2000k|testsrc=size=1920x1080:rate=25"
    "cam04|1280x720|25|1000k|testsrc=size=1280x720:rate=25"
    "cam05|640x480|15|400k|testsrc2=size=640x480:rate=15"
    "cam06|640x480|25|500k|mandelbrot=size=640x480:rate=25"
    "cam07|1280x720|10|600k|smptebars=size=1280x720:rate=10"
    "cam08|1920x1080|10|1200k|smptehdbars=size=1920x1080:rate=10"
)

# Kill any old ffmpeg stream publishers
pkill -f "ffmpeg.*rtsp://${IP}" 2>/dev/null || true
sleep 1

echo ""
echo "[*] Starting FFmpeg publishers..."
echo ""

for STREAM in "${STREAMS[@]}"; do
    IFS='|' read -r NAME RES FPS BITRATE LAVFI <<< "$STREAM"

    nohup ffmpeg -hide_banner -loglevel error \
        -re \
        -f lavfi -i "${LAVFI},drawtext=text='${NAME} %{localtime}':fontsize=32:fontcolor=white:box=1:boxcolor=black@0.5:x=10:y=10" \
        -f lavfi -i "sine=frequency=440:sample_rate=44100" \
        -c:v libx264 -preset ultrafast -tune zerolatency \
        -b:v "$BITRATE" -maxrate "$BITRATE" -bufsize "$((${BITRATE%k} * 2))k" \
        -g "$((FPS * 2))" -sc_threshold 0 \
        -c:a aac -b:a 32k -ar 44100 \
        -f rtsp -rtsp_transport tcp \
        "rtsp://${IP}:${RTSP_PORT}/${NAME}" \
        > "$LOG_DIR/${NAME}.log" 2>&1 &

    echo "  [+] $NAME ($RES @ ${FPS}fps, ${BITRATE}bps) → rtsp://${IP}:${RTSP_PORT}/${NAME}"
done

echo ""
echo "════════════════════════════════════════════════════════════════"
echo "  8 RTSP Test Streams Ready"
echo "════════════════════════════════════════════════════════════════"
echo ""
echo "  rtsp://192.168.50.153:8554/cam01   1080p @ 15fps  ~1.5 Mbps  (testsrc2)"
echo "  rtsp://192.168.50.153:8554/cam02    720p @ 15fps  ~0.8 Mbps  (testsrc2)"
echo "  rtsp://192.168.50.153:8554/cam03   1080p @ 25fps  ~2.0 Mbps  (testsrc)"
echo "  rtsp://192.168.50.153:8554/cam04    720p @ 25fps  ~1.0 Mbps  (testsrc)"
echo "  rtsp://192.168.50.153:8554/cam05    480p @ 15fps  ~0.4 Mbps  (testsrc2)"
echo "  rtsp://192.168.50.153:8554/cam06    480p @ 25fps  ~0.5 Mbps  (mandelbrot)"
echo "  rtsp://192.168.50.153:8554/cam07    720p @ 10fps  ~0.6 Mbps  (SMPTEbars)"
echo "  rtsp://192.168.50.153:8554/cam08   1080p @ 10fps  ~1.2 Mbps  (SMPTEhd)"
echo ""
echo "  Total approx bandwidth: ~8 Mbps"
echo ""
echo "  Logs: $LOG_DIR/"
echo "  Stop all: pkill -f mediamtx; pkill -f 'ffmpeg.*rtsp'"
echo "════════════════════════════════════════════════════════════════"
