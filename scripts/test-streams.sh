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

# Path to the test config (relative to repo root)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TEST_CONFIG="${SCRIPT_DIR}/../config/go2rtc/go2rtc-test.yaml"

# All 64 test stream names: 8 named originals + test_cam_09..test_cam_64
_all_stream_names() {
    echo "test_pattern test_colorbars test_red test_blue test_green test_mandelbrot test_pattern2 test_lowfps"
    seq -f "test_cam_%02g" 9 64
}

show_config() {
    echo "================================================================"
    echo "Start go2rtc with the 64-stream test configuration:"
    echo "================================================================"
    echo ""
    echo "  go2rtc -config ${TEST_CONFIG}"
    echo ""
    echo "Or copy the streams section into your existing go2rtc.yaml:"
    echo ""
    echo "  ${TEST_CONFIG}"
    echo ""
    echo "================================================================"
    echo "Once running, streams are available at:"
    echo "================================================================"
    echo ""
    echo "  RTSP:   rtsp://${GO2RTC_HOST}:${GO2RTC_RTSP_PORT}/<stream_name>"
    echo "  Web UI: http://${GO2RTC_HOST}:${GO2RTC_API_PORT}/"
    echo "  API:    http://${GO2RTC_HOST}:${GO2RTC_API_PORT}/api/streams"
    echo ""
    echo "Stream names (64 total):"
    _all_stream_names | tr ' ' '\n' | sed 's/^/    /'
    echo ""
}

verify_streams() {
    echo "Verifying go2rtc test streams (64 total)..."
    echo ""

    # Check if go2rtc API is accessible
    if ! curl -s --connect-timeout 5 "http://${GO2RTC_HOST}:${GO2RTC_API_PORT}/api/streams" > /dev/null 2>&1; then
        echo "❌ ERROR: Cannot connect to go2rtc API at http://${GO2RTC_HOST}:${GO2RTC_API_PORT}"
        echo "   Start go2rtc with: go2rtc -config ${TEST_CONFIG}"
        exit 1
    fi

    echo "✓ go2rtc API is accessible"
    echo ""

    STREAMS=$(curl -s "http://${GO2RTC_HOST}:${GO2RTC_API_PORT}/api/streams")
    FOUND=0
    MISSING=0

    for stream in $(_all_stream_names); do
        if echo "$STREAMS" | grep -q "\"$stream\""; then
            echo "✓ $stream"
            FOUND=$((FOUND + 1))
        else
            echo "✗ $stream  (MISSING)"
            MISSING=$((MISSING + 1))
        fi
    done

    echo ""
    echo "Summary: ${FOUND}/64 streams found, ${MISSING} missing"

    if [ "$MISSING" -gt 0 ]; then
        echo ""
        echo "To start go2rtc with all 64 streams:"
        echo "  go2rtc -config ${TEST_CONFIG}"
        exit 1
    fi

    echo ""
    echo "All 64 test streams are configured!"
    echo "  RTSP base: rtsp://${GO2RTC_HOST}:${GO2RTC_RTSP_PORT}/<stream_name>"
}

show_help() {
    echo "test-streams.sh - Set up 64 test RTSP streams for lightNVR development"
    echo ""
    echo "Usage: $0 [command]"
    echo ""
    echo "Commands:"
    echo "  config    Show how to start go2rtc with all 64 test streams"
    echo "  verify    Verify all 64 streams are accessible via go2rtc API"
    echo "  help      Show this help message"
    echo ""
    echo "Environment variables:"
    echo "  GO2RTC_HOST       Host where go2rtc is running (default: localhost)"
    echo "  GO2RTC_API_PORT   go2rtc API port (default: 11984)"
    echo "  GO2RTC_RTSP_PORT  go2rtc RTSP port (default: 18554)"
    echo ""
    echo "Examples:"
    echo "  $0 config                              # Show how to start go2rtc"
    echo "  $0 verify                              # Check all 64 streams"
    echo "  GO2RTC_HOST=192.168.1.100 $0 verify   # Verify on remote host"
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

