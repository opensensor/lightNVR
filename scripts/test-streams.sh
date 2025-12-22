#!/bin/bash
# test-streams.sh - Start test RTSP streams for development and CI/CD testing
#
# This script helps set up virtual test RTSP streams using go2rtc's built-in
# FFmpeg lavfi virtual source. These streams are useful for:
#   - Local development without real cameras
#   - CI/CD integration testing
#   - Debugging stream handling code
#
# Usage:
#   ./scripts/test-streams.sh [command]
#
# Commands:
#   config    - Show go2rtc configuration for test streams
#   verify    - Verify test streams are accessible
#   help      - Show this help message

set -e

# Configuration
# Test config uses different ports (11984/18554) to avoid conflicts with production
GO2RTC_API_PORT="${GO2RTC_API_PORT:-11984}"
GO2RTC_RTSP_PORT="${GO2RTC_RTSP_PORT:-18554}"
GO2RTC_HOST="${GO2RTC_HOST:-localhost}"

# Test stream definitions for go2rtc.yaml
# See config/go2rtc/go2rtc-test.yaml for the full configuration
TEST_STREAMS_CONFIG='# Use config/go2rtc/go2rtc-test.yaml for the complete test configuration
# Start with: ./go2rtc/go2rtc -config config/go2rtc/go2rtc-test.yaml

streams:
  # Test pattern with timestamp counter (720p)
  test_pattern: ffmpeg:virtual?video&size=720#video=h264

  # SMPTE color bars (720p)
  test_colorbars: ffmpeg:virtual?video=smptebars&size=720#video=h264

  # Solid colors (480p, low CPU)
  test_red: "exec:ffmpeg -re -f lavfi -i color=red:s=640x480:r=10 -c:v libx264 -preset ultrafast -tune zerolatency -g 30 -f rtsp {output}"
  test_blue: "exec:ffmpeg -re -f lavfi -i color=blue:s=640x480:r=10 -c:v libx264 -preset ultrafast -tune zerolatency -g 30 -f rtsp {output}"
  test_green: "exec:ffmpeg -re -f lavfi -i color=green:s=640x480:r=10 -c:v libx264 -preset ultrafast -tune zerolatency -g 30 -f rtsp {output}"

  # Moving pattern for motion detection testing (480p)
  test_mandelbrot: ffmpeg:virtual?video=mandelbrot&size=640x480#video=h264

  # High resolution test pattern (1080p)
  test_pattern2: ffmpeg:virtual?video=testsrc2&size=1080#video=h264

  # Low framerate stream (5 fps, 480p)
  test_lowfps: ffmpeg:virtual?video=testsrc2&size=640x480&rate=5#video=h264'

show_config() {
    echo "================================================================"
    echo "Add the following to your go2rtc.yaml configuration file:"
    echo "================================================================"
    echo ""
    echo "$TEST_STREAMS_CONFIG"
    echo ""
    echo "================================================================"
    echo "After adding, restart go2rtc and access streams at:"
    echo "================================================================"
    echo ""
    echo "  RTSP URLs:"
    echo "    rtsp://${GO2RTC_HOST}:${GO2RTC_RTSP_PORT}/test_pattern"
    echo "    rtsp://${GO2RTC_HOST}:${GO2RTC_RTSP_PORT}/test_colorbars"
    echo "    rtsp://${GO2RTC_HOST}:${GO2RTC_RTSP_PORT}/test_red"
    echo "    rtsp://${GO2RTC_HOST}:${GO2RTC_RTSP_PORT}/test_blue"
    echo "    rtsp://${GO2RTC_HOST}:${GO2RTC_RTSP_PORT}/test_green"
    echo "    rtsp://${GO2RTC_HOST}:${GO2RTC_RTSP_PORT}/test_mandelbrot"
    echo "    rtsp://${GO2RTC_HOST}:${GO2RTC_RTSP_PORT}/test_pattern2"
    echo "    rtsp://${GO2RTC_HOST}:${GO2RTC_RTSP_PORT}/test_lowfps"
    echo ""
    echo "  Web UI:"
    echo "    http://${GO2RTC_HOST}:${GO2RTC_API_PORT}/go2rtc/"
    echo ""
    echo "  API endpoints:"
    echo "    http://${GO2RTC_HOST}:${GO2RTC_API_PORT}/go2rtc/api/streams"
    echo ""
}

verify_streams() {
    echo "Verifying go2rtc test streams..."
    echo ""
    
    # Check if go2rtc API is accessible
    if ! curl -s --connect-timeout 5 "http://${GO2RTC_HOST}:${GO2RTC_API_PORT}/go2rtc/api/streams" > /dev/null 2>&1; then
        echo "❌ ERROR: Cannot connect to go2rtc API at http://${GO2RTC_HOST}:${GO2RTC_API_PORT}"
        echo "   Make sure go2rtc is running with the test stream configuration."
        exit 1
    fi
    
    echo "✓ go2rtc API is accessible"
    echo ""
    
    # Get list of streams
    STREAMS=$(curl -s "http://${GO2RTC_HOST}:${GO2RTC_API_PORT}/go2rtc/api/streams")
    
    # Check for test streams
    TEST_STREAM_NAMES=("test_pattern" "test_colorbars" "test_red" "test_blue" "test_green" "test_mandelbrot" "test_pattern2" "test_lowfps")
    
    FOUND=0
    MISSING=0
    
    for stream in "${TEST_STREAM_NAMES[@]}"; do
        if echo "$STREAMS" | grep -q "\"$stream\""; then
            echo "✓ Stream '$stream' is configured"
            FOUND=$((FOUND + 1))
        else
            echo "✗ Stream '$stream' is NOT configured"
            MISSING=$((MISSING + 1))
        fi
    done
    
    echo ""
    echo "Summary: $FOUND streams found, $MISSING streams missing"
    
    if [ $MISSING -gt 0 ]; then
        echo ""
        echo "To add missing streams, run: $0 config"
        exit 1
    fi
    
    echo ""
    echo "All test streams are configured! You can now:"
    echo "  1. Add streams to lightNVR via the web UI or API"
    echo "  2. Use RTSP URLs directly: rtsp://${GO2RTC_HOST}:${GO2RTC_RTSP_PORT}/<stream_name>"
}

show_help() {
    echo "test-streams.sh - Set up test RTSP streams for lightNVR development"
    echo ""
    echo "Usage: $0 [command]"
    echo ""
    echo "Commands:"
    echo "  config    Show go2rtc configuration for test streams"
    echo "  verify    Verify test streams are accessible via go2rtc API"
    echo "  help      Show this help message"
    echo ""
    echo "Environment variables:"
    echo "  GO2RTC_HOST       Host where go2rtc is running (default: localhost)"
    echo "  GO2RTC_API_PORT   go2rtc API port (default: 1984)"
    echo "  GO2RTC_RTSP_PORT  go2rtc RTSP port (default: 8554)"
    echo ""
    echo "Examples:"
    echo "  $0 config                    # Show configuration to add"
    echo "  $0 verify                    # Check if streams are working"
    echo "  GO2RTC_HOST=192.168.1.100 $0 verify  # Verify on remote host"
}

# Main
case "${1:-help}" in
    config)
        show_config
        ;;
    verify)
        verify_streams
        ;;
    help|--help|-h)
        show_help
        ;;
    *)
        echo "Unknown command: $1"
        echo ""
        show_help
        exit 1
        ;;
esac

