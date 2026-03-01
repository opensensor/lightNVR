#!/bin/bash
# HD RTSP Test Streams – LightNVR Load Testing
# Downloads 8 real sample video files then publishes 64 looped RTSP streams
# via MediaMTX + FFmpeg at higher resolutions than the lavfi-based script.
#
#   cam01–cam08  →  1920×1080 @ 25 fps  (3 000 kbit/s)
#   cam09–cam64  →  1280×720  @ 15 fps  (1 500 kbit/s)
#
# All streams loop real video content and overlay the camera name + timestamp.
#
# Overridable env-vars:
#   RTSP_IP   – IP to bind / advertise  (default: 192.168.50.153)
#   RTSP_PORT – MediaMTX RTSP port      (default: 8554)
#   VIDEO_DIR – where videos are cached (default: /opt/lightnvr/sample-videos)
#   DB_PATH   – LightNVR SQLite DB      (default: /var/lib/lightnvr/lightnvr.db)
#   SKIP_DB   – set to 1 to skip DB registration

set -euo pipefail

# ── Config ────────────────────────────────────────────────────────────────────
MEDIAMTX_VERSION="v1.9.3"
MEDIAMTX_DIR="/opt/mediamtx"
MEDIAMTX_BIN="$MEDIAMTX_DIR/mediamtx"
MEDIAMTX_CONF="$MEDIAMTX_DIR/mediamtx.yml"
IP="${RTSP_IP:-192.168.50.153}"
RTSP_PORT="${RTSP_PORT:-8554}"
LOG_DIR="/tmp/rtsp_streams"
VIDEO_DIR="${VIDEO_DIR:-/opt/lightnvr/sample-videos}"
DB_PATH="${DB_PATH:-/var/lib/lightnvr/lightnvr.db}"

mkdir -p "$LOG_DIR"
sudo mkdir -p "$VIDEO_DIR"

# ── 1. Ensure ffmpeg and wget ─────────────────────────────────────────────────
for BIN in ffmpeg wget; do
    if ! command -v "$BIN" &>/dev/null; then
        echo "[*] Installing $BIN..."
        sudo apt-get update -qq && sudo apt-get install -y -qq "$BIN"
    fi
done

# ── 2. Download MediaMTX if missing ──────────────────────────────────────────
if [ ! -f "$MEDIAMTX_BIN" ]; then
    echo "[*] Downloading MediaMTX $MEDIAMTX_VERSION..."
    ARCH=$(uname -m)
    case $ARCH in
        x86_64)  ARCH_LABEL="amd64"    ;;
        aarch64) ARCH_LABEL="arm64v8"  ;;
        armv7*)  ARCH_LABEL="armv7"    ;;
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

# ── 4. Start / restart MediaMTX ──────────────────────────────────────────────
if pgrep -f mediamtx &>/dev/null; then
    echo "[*] MediaMTX already running – restarting..."
    pkill -f mediamtx || true
    sleep 1
fi
echo "[*] Starting MediaMTX..."
nohup sudo "$MEDIAMTX_BIN" "$MEDIAMTX_CONF" > "$LOG_DIR/mediamtx.log" 2>&1 &
sleep 2
echo "[+] MediaMTX PID: $(pgrep -f mediamtx)"

# ── 5. Sample-video catalogue ─────────────────────────────────────────────────
# Format: "local-filename|URL|Human description"
# Source: Google's public test-video CDN (gtv-videos-bucket) – stable, no auth.
declare -a VIDEO_CATALOGUE=(
    "big_buck_bunny.mp4|https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4|Big Buck Bunny (1080p 30fps ~158 MB)"
    "elephants_dream.mp4|https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/ElephantsDream.mp4|Elephants Dream (1080p ~156 MB)"
    "tears_of_steel.mp4|https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/TearsOfSteel.mp4|Tears of Steel (1080p ~738 MB)"
    "subaru_outback.mp4|https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/SubaruOutbackOnStreetAndDirt.mp4|Subaru Outback Street & Dirt (HD ~66 MB)"
    "volkswagen_gti.mp4|https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/VolkswagenGTIReview.mp4|Volkswagen GTI Review (HD ~50 MB)"
    "we_are_going_on_bullrun.mp4|https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/WeAreGoingOnBullrun.mp4|We Are Going On Bullrun (HD ~48 MB)"
    "for_bigger_blazes.mp4|https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/ForBiggerBlazes.mp4|For Bigger Blazes (HD ~11 MB)"
    "for_bigger_escapes.mp4|https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/ForBiggerEscapes.mp4|For Bigger Escapes (HD ~21 MB)"
)

# ── 6. Download videos (resumable) ───────────────────────────────────────────
echo ""
echo "[*] Downloading sample videos → $VIDEO_DIR"
echo "    (Tears of Steel is ~738 MB; total ~1.3 GB – resumable with -c)"
echo ""
declare -a VIDEO_FILES=()
for ENTRY in "${VIDEO_CATALOGUE[@]}"; do
    IFS='|' read -r FNAME URL DESC <<< "$ENTRY"
    DEST="$VIDEO_DIR/$FNAME"
    if [ -f "$DEST" ] && [ "$(stat -c%s "$DEST")" -gt 1048576 ]; then
        echo "  [✓] Already present: $FNAME"
    else
        echo "  [↓] $DESC"
        if ! sudo wget -q --show-progress -c -O "$DEST" "$URL"; then
            echo "  [!] WARNING: download failed for $FNAME – skipping"
            sudo rm -f "$DEST"
            continue
        fi
    fi
    VIDEO_FILES+=("$DEST")
done

NUM_VIDEOS=${#VIDEO_FILES[@]}
if [ "$NUM_VIDEOS" -eq 0 ]; then
    echo "[✗] No videos downloaded – cannot continue."
    exit 1
fi
echo ""
echo "[+] $NUM_VIDEOS video file(s) ready (round-robin across 64 streams)."



# ── 7. Kill old FFmpeg publishers ─────────────────────────────────────────────
pkill -f "ffmpeg.*rtsp://${IP}" 2>/dev/null || true
sleep 1

# ── 8. Launch 64 looped-video FFmpeg publishers ───────────────────────────────
# cam01-cam08  → 1920×1080 @ 25 fps (3 000 kbit/s)
# cam09-cam64  → 1280×720  @ 15 fps (1 500 kbit/s)
# Video files are assigned round-robin so all source footage is represented.
echo "[*] Starting 64 FFmpeg publishers..."

for IDX in $(seq 0 63); do
    CAM_NUM=$(( IDX + 1 ))
    NAME=$(printf "cam%02d" "$CAM_NUM")
    VID="${VIDEO_FILES[$(( IDX % NUM_VIDEOS ))]}"

    if [ "$CAM_NUM" -le 8 ]; then
        W=1920; H=1080; FPS=25; BR=3000
    else
        W=1280; H=720;  FPS=15; BR=1500
    fi

    SCALE="scale=${W}:${H}:force_original_aspect_ratio=decrease,pad=${W}:${H}:(ow-iw)/2:(oh-ih)/2"
    TEXT="drawtext=text='${NAME} %{localtime\}':fontsize=28:fontcolor=white:box=1:boxcolor=black@0.5:x=10:y=10"

    nohup ffmpeg -hide_banner -loglevel error \
        -stream_loop -1 -re \
        -i "$VID" \
        -vf "${SCALE},${TEXT}" \
        -c:v libx264 -preset ultrafast -tune zerolatency \
        -b:v "${BR}k" -maxrate "${BR}k" -bufsize "$(( BR * 2 ))k" \
        -r "$FPS" -g "$(( FPS * 2 ))" -sc_threshold 0 \
        -c:a aac -b:a 64k -ar 44100 \
        -f rtsp -rtsp_transport tcp \
        "rtsp://${IP}:${RTSP_PORT}/${NAME}" \
        > "$LOG_DIR/${NAME}.log" 2>&1 &

    printf "  [+] %-6s  %dx%d @ %dfps  %dk  <- %s\n" \
        "$NAME" "$W" "$H" "$FPS" "$BR" "$(basename "$VID")"
done

# ── 9. Summary ────────────────────────────────────────────────────────────────
echo ""
echo "════════════════════════════════════════════════════════════════════════"
echo "  64 HD RTSP Streams Ready  (real video files, infinite loop)"
echo "════════════════════════════════════════════════════════════════════════"
for IDX in $(seq 0 63); do
    CAM_NUM=$(( IDX + 1 ))
    NAME=$(printf "cam%02d" "$CAM_NUM")
    VID_BASE="$(basename "${VIDEO_FILES[$(( IDX % NUM_VIDEOS ))]}")"
    if [ "$CAM_NUM" -le 8 ]; then RES="1920x1080"; FPS=25; else RES="1280x720"; FPS=15; fi
    printf "  rtsp://%s:%s/%-6s  %-11s @ %dfps  %s\n" \
        "$IP" "$RTSP_PORT" "$NAME" "$RES" "$FPS" "$VID_BASE"
done
echo ""
echo "  Video cache : $VIDEO_DIR/"
echo "  FFmpeg logs : $LOG_DIR/"
echo "  Stop all    : pkill -f mediamtx; pkill -f 'ffmpeg.*rtsp'"
echo "════════════════════════════════════════════════════════════════════════"

# ── 10. Register streams in LightNVR DB ──────────────────────────────────────
if [ "${SKIP_DB:-0}" = "1" ]; then
    echo ""
    echo "[*] SKIP_DB=1 – skipping database registration."
    exit 0
fi

if [ ! -f "$DB_PATH" ]; then
    echo ""
    echo "[!] LightNVR DB not found at $DB_PATH – skipping registration."
    echo "    Re-run after LightNVR has initialised, or:"
    echo "      DB_PATH=/path/to/lightnvr.db $0"
    exit 0
fi

echo ""
echo "[*] Registering streams in LightNVR DB: $DB_PATH"

SQL="BEGIN;"
for IDX in $(seq 0 63); do
    CAM_NUM=$(( IDX + 1 ))
    NAME=$(printf "cam%02d" "$CAM_NUM")
    if [ "$CAM_NUM" -le 8 ]; then W=1920; H=1080; FPS=25; else W=1280; H=720; FPS=15; fi
    URL="rtsp://${IP}:${RTSP_PORT}/${NAME}"
    SQL+="
INSERT OR REPLACE INTO streams
    (name, url, enabled, streaming_enabled, width, height, fps, codec,
     priority, record, segment_duration, record_audio, tags)
VALUES
    ('${NAME}','${URL}',1,1,${W},${H},${FPS},'h264',5,0,60,0,'test-hd');"
done
SQL+="
COMMIT;
SELECT COUNT(*) || ' HD test streams registered' FROM streams WHERE tags='test-hd';"

sudo sqlite3 "$DB_PATH" "$SQL"
echo "[+] Done. Restart LightNVR to activate the new streams."
