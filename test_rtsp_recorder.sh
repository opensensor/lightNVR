#!/bin/bash

# Test script for rtsp_recorder
# This script runs the rtsp_recorder with a short duration to test audio recording

# Default RTSP URL - replace with your camera's URL if needed
RTSP_URL="rtsp://thingino:thingino@192.168.50.49:554/ch0"

# Output file
OUTPUT_FILE="test_output.mp4"

# Duration in seconds
DURATION=10

echo "Running rtsp_recorder test..."
echo "RTSP URL: $RTSP_URL"
echo "Output file: $OUTPUT_FILE"
echo "Duration: $DURATION seconds"

# Run the recorder
./rtsp_recorder -i "$RTSP_URL" -o "$OUTPUT_FILE" -d "$DURATION"

# Check if the file was created
if [ -f "$OUTPUT_FILE" ]; then
    # Get file size
    SIZE=$(du -h "$OUTPUT_FILE" | cut -f1)
    echo "Recording successful! File size: $SIZE"
    
    # Use ffprobe to check if the file has audio
    echo "Checking for audio stream..."
    AUDIO_STREAM=$(ffprobe -v error -select_streams a -show_streams "$OUTPUT_FILE")
    
    if [ -n "$AUDIO_STREAM" ]; then
        echo "Audio stream found in the recording!"
    else
        echo "No audio stream found in the recording."
    fi
else
    echo "Recording failed! No output file was created."
fi

echo "Test complete."
