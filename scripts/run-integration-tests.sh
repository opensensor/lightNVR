#!/bin/bash
# run-integration-tests.sh - Run integration tests locally
#
# This script runs integration tests using go2rtc test streams.
# It can be used for local development or CI/CD testing.
#
# Usage:
#   ./scripts/run-integration-tests.sh [options]
#
# Options:
#   --start-go2rtc    Start go2rtc with test config (if not already running)
#   --stop-go2rtc     Stop go2rtc after tests
#   --skip-build      Skip building lightNVR
#   --verbose         Show verbose output

set -e

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
GO2RTC_BIN="${PROJECT_ROOT}/go2rtc/go2rtc"
GO2RTC_TEST_CONFIG="${PROJECT_ROOT}/config/go2rtc/go2rtc-test.yaml"
GO2RTC_API_PORT=11984
GO2RTC_RTSP_PORT=18554
TEST_OUTPUT_DIR="/tmp/lightnvr-integration-tests"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Flags
START_GO2RTC=false
STOP_GO2RTC=false
SKIP_BUILD=false
VERBOSE=false

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --start-go2rtc) START_GO2RTC=true; shift ;;
        --stop-go2rtc) STOP_GO2RTC=true; shift ;;
        --skip-build) SKIP_BUILD=true; shift ;;
        --verbose) VERBOSE=true; shift ;;
        *) echo "Unknown option: $1"; exit 1 ;;
    esac
done

# Helper functions
log_info() { echo -e "${GREEN}[INFO]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }
log_test() { echo -e "  $1"; }

# Cleanup function
cleanup() {
    if [ "$STOP_GO2RTC" = true ] && [ -n "$GO2RTC_PID" ]; then
        log_info "Stopping go2rtc (PID: $GO2RTC_PID)..."
        kill $GO2RTC_PID 2>/dev/null || true
    fi
}
trap cleanup EXIT

# Check prerequisites
check_prerequisites() {
    log_info "Checking prerequisites..."

    local missing=0

    if ! command -v ffmpeg &> /dev/null; then
        log_error "ffmpeg is not installed"
        missing=1
    fi

    if ! command -v curl &> /dev/null; then
        log_error "curl is not installed"
        missing=1
    fi

    if ! command -v jq &> /dev/null; then
        log_error "jq is not installed"
        missing=1
    fi

    if [ ! -f "$GO2RTC_BIN" ]; then
        log_error "go2rtc binary not found at $GO2RTC_BIN"
        log_info "Build go2rtc first or download from releases"
        missing=1
    fi

    if [ $missing -eq 1 ]; then
        exit 1
    fi

    log_info "Prerequisites check passed"
}

# Start go2rtc if requested
start_go2rtc() {
    if [ "$START_GO2RTC" = true ]; then
        # Check if already running on test port
        if curl -s "http://localhost:${GO2RTC_API_PORT}/api/streams" > /dev/null 2>&1; then
            log_info "go2rtc already running on port ${GO2RTC_API_PORT}"
            return 0
        fi

        log_info "Starting go2rtc with test configuration..."
        "$GO2RTC_BIN" -config "$GO2RTC_TEST_CONFIG" &
        GO2RTC_PID=$!

        # Wait for go2rtc to start
        for i in {1..30}; do
            if curl -s "http://localhost:${GO2RTC_API_PORT}/api/streams" > /dev/null 2>&1; then
                log_info "go2rtc started successfully (PID: $GO2RTC_PID)"
                return 0
            fi
            sleep 1
        done

        log_error "go2rtc failed to start within 30 seconds"
        exit 1
    fi
}

# Check go2rtc is running
check_go2rtc() {
    log_info "Checking go2rtc connectivity..."

    if ! curl -s "http://localhost:${GO2RTC_API_PORT}/api/streams" > /dev/null 2>&1; then
        log_error "Cannot connect to go2rtc API on port ${GO2RTC_API_PORT}"
        log_info "Start go2rtc with: $GO2RTC_BIN -config $GO2RTC_TEST_CONFIG"
        log_info "Or run with --start-go2rtc flag"
        exit 1
    fi

    log_info "go2rtc is accessible"
}

# Test: Verify all test streams are configured
test_streams_configured() {
    log_info "Test: Verifying test streams are configured..."

    local streams=$(curl -s "http://localhost:${GO2RTC_API_PORT}/api/streams")
    local expected_streams=("test_pattern" "test_colorbars" "test_red" "test_blue" "test_green" "test_mandelbrot" "test_pattern2" "test_lowfps")
    local failed=0

    for stream in "${expected_streams[@]}"; do
        if echo "$streams" | jq -e ".[\"$stream\"]" > /dev/null 2>&1; then
            log_test "✓ Stream '$stream' configured"
        else
            log_test "✗ Stream '$stream' NOT configured"
            failed=1
        fi
    done

    return $failed
}

# Test: Capture frames from RTSP streams
test_rtsp_capture() {
    log_info "Test: Capturing frames from RTSP streams..."

    mkdir -p "$TEST_OUTPUT_DIR"
    local test_streams=("test_pattern" "test_colorbars" "test_red")
    local failed=0

    for stream in "${test_streams[@]}"; do
        local output_file="${TEST_OUTPUT_DIR}/frame_${stream}.jpg"

        # Remove old file if exists
        rm -f "$output_file"

        # Run ffmpeg with nostdin to prevent hanging on stdin
        # Redirect stderr to /dev/null but keep stdout for progress
        set +e
        timeout 30 ffmpeg -nostdin -y -rtsp_transport tcp \
            -i "rtsp://localhost:${GO2RTC_RTSP_PORT}/${stream}" \
            -frames:v 1 -update 1 "$output_file" </dev/null >/dev/null 2>&1
        local ffmpeg_exit=$?
        set -e

        if [ -f "$output_file" ]; then
            local size=$(stat -c%s "$output_file" 2>/dev/null || echo "0")
            if [ "$size" -gt 1000 ]; then
                log_test "✓ Captured frame from '$stream' (${size} bytes)"
            else
                log_test "✗ Frame too small from '$stream' (${size} bytes)"
                failed=1
            fi
        else
            log_test "✗ No output file for '$stream' (ffmpeg exit: $ffmpeg_exit)"
            failed=1
        fi
    done

    return $failed
}

# Test: Verify stream metadata via API
test_stream_metadata() {
    log_info "Test: Checking stream metadata..."

    local streams=$(curl -s "http://localhost:${GO2RTC_API_PORT}/api/streams")
    local failed=0

    # Check that streams have producers configured
    for stream in test_pattern test_colorbars; do
        local producers=$(echo "$streams" | jq -r ".[\"$stream\"].producers | length")
        if [ "$producers" -gt 0 ]; then
            log_test "✓ Stream '$stream' has $producers producer(s)"
        else
            log_test "✗ Stream '$stream' has no producers"
            failed=1
        fi
    done

    return $failed
}

# Test: Verify WebRTC endpoint is available
test_webrtc_endpoint() {
    log_info "Test: Checking WebRTC endpoint..."

    # Just check that the WebRTC port is listening
    if curl -s "http://localhost:${GO2RTC_API_PORT}/api/webrtc" > /dev/null 2>&1; then
        log_test "✓ WebRTC API endpoint accessible"
        return 0
    else
        log_test "✗ WebRTC API endpoint not accessible"
        return 1
    fi
}

# Run all tests
run_tests() {
    log_info "Running integration tests..."
    echo ""

    local total=0
    local passed=0
    local failed=0

    # Test 1: Streams configured
    total=$((total + 1))
    if test_streams_configured; then
        passed=$((passed + 1))
    else
        failed=$((failed + 1))
    fi
    echo ""

    # Test 2: RTSP capture
    total=$((total + 1))
    if test_rtsp_capture; then
        passed=$((passed + 1))
    else
        failed=$((failed + 1))
    fi
    echo ""

    # Test 3: Stream metadata
    total=$((total + 1))
    if test_stream_metadata; then
        passed=$((passed + 1))
    else
        failed=$((failed + 1))
    fi
    echo ""

    # Test 4: WebRTC endpoint
    total=$((total + 1))
    if test_webrtc_endpoint; then
        passed=$((passed + 1))
    else
        failed=$((failed + 1))
    fi
    echo ""

    # Summary
    echo "========================================"
    log_info "Test Summary: $passed/$total passed"
    if [ $failed -gt 0 ]; then
        log_error "$failed test(s) failed"
        return 1
    else
        log_info "All tests passed!"
        return 0
    fi
}

# Main
main() {
    echo "========================================"
    echo "lightNVR Integration Tests"
    echo "========================================"
    echo ""

    check_prerequisites
    start_go2rtc
    check_go2rtc
    echo ""

    run_tests
}

main "$@"
