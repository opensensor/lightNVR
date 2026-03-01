#!/bin/bash
# RTSP Test Stream Setup for LightNVR Load Testing
# Streams 64 channels via MediaMTX + FFmpeg on 192.168.50.153
# Requires: ffmpeg, mediamtx (auto-downloaded if missing), sqlite3
# Override DB path: DB_PATH=/path/to/lightnvr.db ./setup_rtsp_streams.sh

set -e

MEDIAMTX_VERSION="v1.9.3"
MEDIAMTX_DIR="/opt/mediamtx"
MEDIAMTX_BIN="$MEDIAMTX_DIR/mediamtx"
MEDIAMTX_CONF="$MEDIAMTX_DIR/mediamtx.yml"
IP="192.168.50.153"
RTSP_PORT="8554"
LOG_DIR="/tmp/rtsp_streams"
DB_PATH="${DB_PATH:-/var/lib/lightnvr/lightnvr.db}"

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
# Format: "name|resolution|fps|bitrate|lavfi_source"
# cam01-cam08: varied patterns/resolutions (original)
# cam09-cam64: solid-color 640x480@10fps (low CPU, easy to identify)
declare -a STREAMS=(
    "cam01|1920x1080|15|1500k|testsrc2=size=1920x1080:rate=15"
    "cam02|1280x720|15|800k|testsrc2=size=1280x720:rate=15"
    "cam03|1920x1080|25|2000k|testsrc=size=1920x1080:rate=25"
    "cam04|1280x720|25|1000k|testsrc=size=1280x720:rate=25"
    "cam05|640x480|15|400k|testsrc2=size=640x480:rate=15"
    "cam06|640x480|25|500k|mandelbrot=size=640x480:rate=25"
    "cam07|1280x720|10|600k|smptebars=size=1280x720:rate=10"
    "cam08|1920x1080|10|1200k|smptehdbars=size=1920x1080:rate=10"
    "cam09|640x480|10|250k|color=c=yellow:size=640x480:rate=10"
    "cam10|640x480|10|250k|color=c=cyan:size=640x480:rate=10"
    "cam11|640x480|10|250k|color=c=magenta:size=640x480:rate=10"
    "cam12|640x480|10|250k|color=c=orange:size=640x480:rate=10"
    "cam13|640x480|10|250k|color=c=purple:size=640x480:rate=10"
    "cam14|640x480|10|250k|color=c=pink:size=640x480:rate=10"
    "cam15|640x480|10|250k|color=c=lime:size=640x480:rate=10"
    "cam16|640x480|10|250k|color=c=teal:size=640x480:rate=10"
    "cam17|640x480|10|250k|color=c=crimson:size=640x480:rate=10"
    "cam18|640x480|10|250k|color=c=royalblue:size=640x480:rate=10"
    "cam19|640x480|10|250k|color=c=limegreen:size=640x480:rate=10"
    "cam20|640x480|10|250k|color=c=hotpink:size=640x480:rate=10"
    "cam21|640x480|10|250k|color=c=darkturquoise:size=640x480:rate=10"
    "cam22|640x480|10|250k|color=c=darkviolet:size=640x480:rate=10"
    "cam23|640x480|10|250k|color=c=darkorange:size=640x480:rate=10"
    "cam24|640x480|10|250k|color=c=seagreen:size=640x480:rate=10"
    "cam25|640x480|10|250k|color=c=dodgerblue:size=640x480:rate=10"
    "cam26|640x480|10|250k|color=c=deeppink:size=640x480:rate=10"
    "cam27|640x480|10|250k|color=c=springgreen:size=640x480:rate=10"
    "cam28|640x480|10|250k|color=c=saddlebrown:size=640x480:rate=10"
    "cam29|640x480|10|250k|color=c=slateblue:size=640x480:rate=10"
    "cam30|640x480|10|250k|color=c=lightseagreen:size=640x480:rate=10"
    "cam31|640x480|10|250k|color=c=firebrick:size=640x480:rate=10"
    "cam32|640x480|10|250k|color=c=steelblue:size=640x480:rate=10"
    "cam33|640x480|10|250k|color=c=chocolate:size=640x480:rate=10"
    "cam34|640x480|10|250k|color=c=darkolivegreen:size=640x480:rate=10"
    "cam35|640x480|10|250k|color=c=darkred:size=640x480:rate=10"
    "cam36|640x480|10|250k|color=c=darkslateblue:size=640x480:rate=10"
    "cam37|640x480|10|250k|color=c=peru:size=640x480:rate=10"
    "cam38|640x480|10|250k|color=c=goldenrod:size=640x480:rate=10"
    "cam39|640x480|10|250k|color=c=cadetblue:size=640x480:rate=10"
    "cam40|640x480|10|250k|color=c=mediumseagreen:size=640x480:rate=10"
    "cam41|640x480|10|250k|color=c=rosybrown:size=640x480:rate=10"
    "cam42|640x480|10|250k|color=c=indigo:size=640x480:rate=10"
    "cam43|640x480|10|250k|color=c=skyblue:size=640x480:rate=10"
    "cam44|640x480|10|250k|color=c=sandybrown:size=640x480:rate=10"
    "cam45|640x480|10|250k|color=c=yellowgreen:size=640x480:rate=10"
    "cam46|640x480|10|250k|color=c=turquoise:size=640x480:rate=10"
    "cam47|640x480|10|250k|color=c=violet:size=640x480:rate=10"
    "cam48|640x480|10|250k|color=c=slategray:size=640x480:rate=10"
    "cam49|640x480|10|250k|color=c=darksalmon:size=640x480:rate=10"
    "cam50|640x480|10|250k|color=c=darkkhaki:size=640x480:rate=10"
    "cam51|640x480|10|250k|color=c=lawngreen:size=640x480:rate=10"
    "cam52|640x480|10|250k|color=c=salmon:size=640x480:rate=10"
    "cam53|640x480|10|250k|color=c=cornflowerblue:size=640x480:rate=10"
    "cam54|640x480|10|250k|color=c=burlywood:size=640x480:rate=10"
    "cam55|640x480|10|250k|color=c=forestgreen:size=640x480:rate=10"
    "cam56|640x480|10|250k|color=c=tomato:size=640x480:rate=10"
    "cam57|640x480|10|250k|color=c=khaki:size=640x480:rate=10"
    "cam58|640x480|10|250k|color=c=coral:size=640x480:rate=10"
    "cam59|640x480|10|250k|color=c=sienna:size=640x480:rate=10"
    "cam60|640x480|10|250k|color=c=plum:size=640x480:rate=10"
    "cam61|640x480|10|250k|color=c=darkseagreen:size=640x480:rate=10"
    "cam62|640x480|10|250k|color=c=mediumpurple:size=640x480:rate=10"
    "cam63|640x480|10|250k|color=c=lightsalmon:size=640x480:rate=10"
    "cam64|640x480|10|250k|color=c=mediumaquamarine:size=640x480:rate=10"
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
echo "  64 RTSP Test Streams Ready"
echo "════════════════════════════════════════════════════════════════"
for STREAM in "${STREAMS[@]}"; do
    IFS='|' read -r NAME RES FPS BITRATE LAVFI <<< "$STREAM"
    printf "  rtsp://%s:%s/%-6s  %s @ %sfps  %s\n" \
        "$IP" "$RTSP_PORT" "$NAME" "$RES" "$FPS" "$BITRATE"
done
echo ""
echo "  Logs: $LOG_DIR/"
echo "  Stop all: pkill -f mediamtx; pkill -f 'ffmpeg.*rtsp'"
echo "════════════════════════════════════════════════════════════════"

# ── 6. Register streams in LightNVR DB ───────────────────────────────────────
if [ ! -f "$DB_PATH" ]; then
    echo ""
    echo "[!] LightNVR DB not found at $DB_PATH — skipping registration."
    echo "    Re-run after LightNVR has initialised, or set DB_PATH=/path/to/lightnvr.db"
    exit 0
fi

echo ""
echo "[*] Registering streams in LightNVR DB: $DB_PATH"

SQL="BEGIN;"
for STREAM in "${STREAMS[@]}"; do
    IFS='|' read -r NAME RES FPS BITRATE LAVFI <<< "$STREAM"
    WIDTH=$(echo "$RES" | cut -dx -f1)
    HEIGHT=$(echo "$RES" | cut -dx -f2)
    URL="rtsp://${IP}:${RTSP_PORT}/${NAME}"
    SQL+="
INSERT OR IGNORE INTO streams
    (name, url, enabled, streaming_enabled, width, height, fps, codec,
     priority, record, segment_duration, record_audio, tags)
VALUES
    ('${NAME}','${URL}',1,1,${WIDTH},${HEIGHT},${FPS},'h264',5,0,60,0,'test');"
done
SQL+="
COMMIT;
SELECT COUNT(*) || ' test streams in DB' FROM streams WHERE tags='test';"

sudo sqlite3 "$DB_PATH" "$SQL"
echo "[+] Done. Restart LightNVR to pick up the new streams."
