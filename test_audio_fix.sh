#!/bin/bash

# Test script to verify audio recording in MP4 files
# This script will record a short clip from an RTSP stream and check if it contains audio

# Default values
RTSP_URL="rtsp://thingino:thingino@192.168.50.49:554/ch0"
OUTPUT_FILE="test_audio_recording.mp4"
DURATION=10

# Parse command line arguments
while [[ $# -gt 0 ]]; do
  case $1 in
    -i|--input)
      RTSP_URL="$2"
      shift 2
      ;;
    -o|--output)
      OUTPUT_FILE="$2"
      shift 2
      ;;
    -d|--duration)
      DURATION="$2"
      shift 2
      ;;
    -h|--help)
      echo "Usage: $0 [options]"
      echo "Options:"
      echo "  -i, --input URL     RTSP URL to record (default: $RTSP_URL)"
      echo "  -o, --output FILE   Output MP4 file (default: $OUTPUT_FILE)"
      echo "  -d, --duration SEC  Recording duration in seconds (default: $DURATION)"
      echo "  -h, --help          Show this help message"
      exit 0
      ;;
    *)
      echo "Unknown option: $1"
      exit 1
      ;;
  esac
done

echo "Testing audio recording with the following parameters:"
echo "RTSP URL: $RTSP_URL"
echo "Output file: $OUTPUT_FILE"
echo "Duration: $DURATION seconds"

# Step 1: Record a short clip using the standalone test program
echo "Recording test clip using rtsp_recorder_standalone..."
./rtsp_recorder_standalone -i "$RTSP_URL" -o "$OUTPUT_FILE" -d "$DURATION"

# Step 2: Check if the output file exists
if [ ! -f "$OUTPUT_FILE" ]; then
  echo "Error: Output file $OUTPUT_FILE was not created!"
  exit 1
fi

# Step 3: Use ffprobe to check if the file contains audio streams
echo "Analyzing recorded file for audio streams..."
AUDIO_STREAMS=$(ffprobe -v error -select_streams a -show_entries stream=codec_type -of csv=p=0 "$OUTPUT_FILE" | wc -l)

if [ "$AUDIO_STREAMS" -gt 0 ]; then
  echo "SUCCESS: File contains $AUDIO_STREAMS audio stream(s)"
  
  # Get more details about the audio stream
  echo "Audio stream details:"
  ffprobe -v error -select_streams a -show_entries stream=codec_name,sample_rate,channels -of default=noprint_wrappers=1 "$OUTPUT_FILE"
else
  echo "FAILURE: No audio streams found in the recorded file!"
fi

# Step 4: Print file information
echo "Complete file information:"
ffprobe -v error -show_format -show_streams "$OUTPUT_FILE"

echo "Test completed."
