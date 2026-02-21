#!/bin/bash
# Loop an MP4 file as an RTSP stream via MediaMTX
# Usage: ./stream_mp4_loop.sh <path-to-file.mp4> [stream-name]

IP="192.168.50.153"
RTSP_PORT="8554"

MP4="${1:-}"
NAME="${2:-cam_mp4}"

if [ -z "$MP4" ]; then
    echo "Usage: $0 <path-to-file.mp4> [stream-name]"
    echo "  e.g. $0 /home/matt/test.mp4 cam09"
    exit 1
fi

if [ ! -f "$MP4" ]; then
    echo "Error: file not found: $MP4"
    exit 1
fi

# Make sure MediaMTX is running (from the main setup script)
if ! pgrep -f mediamtx &>/dev/null; then
    echo "Warning: MediaMTX doesn't appear to be running."
    echo "Run setup_rtsp_streams.sh first, or start mediamtx manually."
    exit 1
fi

echo "[*] Streaming $MP4 → rtsp://${IP}:${RTSP_PORT}/${NAME}"
echo "    (looping forever, Ctrl+C to stop)"
echo ""

# -stream_loop -1  = loop forever
# -re              = read at native framerate (crucial for RTSP)
# -copyts          = preserve timestamps across loops
# ultrafast + zerolatency = low encode overhead on the host
exec ffmpeg -hide_banner -loglevel warning \
    -stream_loop -1 -re \
    -i "$MP4" \
    -c:v libx264 -preset ultrafast -tune zerolatency \
    -b:v 1500k -maxrate 1500k -bufsize 3000k \
    -c:a aac -b:a 64k \
    -f rtsp -rtsp_transport tcp \
    "rtsp://${IP}:${RTSP_PORT}/${NAME}"
