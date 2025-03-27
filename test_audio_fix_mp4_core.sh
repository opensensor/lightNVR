#!/bin/bash

# Test script to verify the audio handling fix in mp4_recording_core.c

echo "Building the project..."
make clean
make

echo "Creating test directories if they don't exist..."
mkdir -p /var/lib/lightnvr/recordings/mp4/face

echo "Running a test recording..."
# Use the same RTSP URL as in test_audio_fix.sh
RTSP_URL="rtsp://thingino:thingino@192.168.50.49:554/ch0"

# Start a recording for the 'face' stream
curl -X POST "http://localhost:8080/api/recordings/start?stream=face"

echo "Recording for 10 seconds..."
sleep 10

# Stop the recording
curl -X POST "http://localhost:8080/api/recordings/stop?stream=face"

echo "Checking the latest recording file..."
LATEST_FILE=$(ls -t /var/lib/lightnvr/recordings/mp4/face/*.mp4 | head -1)

if [ -f "$LATEST_FILE" ]; then
    echo "Success! Output file was created: $LATEST_FILE"
    echo "File information:"
    ffprobe -v error -select_streams v:0 -show_entries stream=codec_name,codec_type,width,height -of default=noprint_wrappers=1 "$LATEST_FILE"
    ffprobe -v error -select_streams a:0 -show_entries stream=codec_name,codec_type,sample_rate,channels -of default=noprint_wrappers=1 "$LATEST_FILE"
    echo "Test completed successfully!"
else
    echo "Error: No MP4 recording files found."
    exit 1
fi
