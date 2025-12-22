#!/bin/bash
# run-integration-tests.sh - Run integration tests locally
#
# This script runs integration tests using go2rtc test streams and lightNVR.
# It can be used for local development or CI/CD testing.
#
# Usage:
#   ./scripts/run-integration-tests.sh [options]
#
# Options:
#   --start-go2rtc       Start go2rtc standalone with test config (for go2rtc-only tests)
#   --stop-go2rtc        Stop go2rtc after tests (standalone mode only)
#   --start-lightnvr     Start lightNVR with test config (lightNVR will start go2rtc)
#   --stop-lightnvr      Stop lightNVR after tests
#   --skip-build         Skip building lightNVR
#   --go2rtc-only        Only run go2rtc tests (skip lightNVR tests)
#   --full               Run full integration tests (lightNVR + go2rtc + test streams)
#   --verbose            Show verbose output

set -e

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
GO2RTC_BIN="${PROJECT_ROOT}/go2rtc/go2rtc"
GO2RTC_TEST_CONFIG="${PROJECT_ROOT}/config/go2rtc/go2rtc-test.yaml"
LIGHTNVR_BIN="${PROJECT_ROOT}/build/bin/lightnvr"
LIGHTNVR_TEST_CONFIG="${PROJECT_ROOT}/config/lightnvr-test.ini"
GO2RTC_API_PORT=11984
GO2RTC_RTSP_PORT=18554
LIGHTNVR_PORT=18080
TEST_OUTPUT_DIR="/tmp/lightnvr-integration-tests"
LIGHTNVR_TEST_DIR="/tmp/lightnvr-test"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Flags
START_GO2RTC=false
STOP_GO2RTC=false
START_LIGHTNVR=false
STOP_LIGHTNVR=false
SKIP_BUILD=false
GO2RTC_ONLY=false
FULL_TEST=false
VERBOSE=false

# PIDs for cleanup
GO2RTC_PID=""
LIGHTNVR_PID=""

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --start-go2rtc) START_GO2RTC=true; shift ;;
        --stop-go2rtc) STOP_GO2RTC=true; shift ;;
        --start-lightnvr) START_LIGHTNVR=true; shift ;;
        --stop-lightnvr) STOP_LIGHTNVR=true; shift ;;
        --skip-build) SKIP_BUILD=true; shift ;;
        --go2rtc-only) GO2RTC_ONLY=true; shift ;;
        --full) FULL_TEST=true; START_LIGHTNVR=true; STOP_LIGHTNVR=true; shift ;;
        --verbose) VERBOSE=true; shift ;;
        *) echo "Unknown option: $1"; exit 1 ;;
    esac
done

# Default behavior: if no flags provided, run go2rtc tests with auto start/stop
if [ "$START_GO2RTC" = false ] && [ "$START_LIGHTNVR" = false ] && [ "$FULL_TEST" = false ]; then
    START_GO2RTC=true
    STOP_GO2RTC=true
    GO2RTC_ONLY=true
fi

# Helper functions
log_info() { echo -e "${GREEN}[INFO]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }
log_section() { echo -e "${BLUE}[SECTION]${NC} $1"; }
log_test() { echo -e "  $1"; }

# Cleanup function
cleanup() {
    if [ "$STOP_GO2RTC" = true ] && [ -n "$GO2RTC_PID" ]; then
        log_info "Stopping go2rtc (PID: $GO2RTC_PID)..."
        kill $GO2RTC_PID 2>/dev/null || true
    fi
    if [ "$STOP_LIGHTNVR" = true ] && [ -n "$LIGHTNVR_PID" ]; then
        log_info "Stopping lightNVR (PID: $LIGHTNVR_PID)..."
        kill $LIGHTNVR_PID 2>/dev/null || true
        sleep 1
        kill -9 $LIGHTNVR_PID 2>/dev/null || true
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

    # Check lightNVR binary if we need to test it
    if [ "$LIGHTNVR_ONLY" = true ] || [ "$START_LIGHTNVR" = true ]; then
        if [ ! -f "$LIGHTNVR_BIN" ]; then
            log_error "lightNVR binary not found at $LIGHTNVR_BIN"
            log_info "Build lightNVR first: ./scripts/build.sh"
            missing=1
        fi
    fi

    if [ $missing -eq 1 ]; then
        exit 1
    fi

    log_info "Prerequisites check passed"
}

# Setup lightNVR test directories
setup_lightnvr_dirs() {
    log_info "Setting up lightNVR test directories..."
    mkdir -p "${LIGHTNVR_TEST_DIR}/recordings"
    mkdir -p "${LIGHTNVR_TEST_DIR}/recordings/mp4"
    mkdir -p "${LIGHTNVR_TEST_DIR}/recordings/hls"
    mkdir -p "${LIGHTNVR_TEST_DIR}/models"
    mkdir -p "${LIGHTNVR_TEST_DIR}/go2rtc"
    mkdir -p "$TEST_OUTPUT_DIR"
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

# Start lightNVR if requested
start_lightnvr() {
    if [ "$START_LIGHTNVR" = true ]; then
        setup_lightnvr_dirs

        # Check if already running
        if curl -s -u admin:admin "http://localhost:${LIGHTNVR_PORT}/api/v1/system" > /dev/null 2>&1; then
            log_info "lightNVR already running on port ${LIGHTNVR_PORT}"
            return 0
        fi

        log_info "Starting lightNVR with test configuration..."
        cd "$PROJECT_ROOT"
        "$LIGHTNVR_BIN" -c "$LIGHTNVR_TEST_CONFIG" &
        LIGHTNVR_PID=$!

        # Wait for lightNVR to start
        for i in {1..30}; do
            if curl -s -u admin:admin "http://localhost:${LIGHTNVR_PORT}/api/v1/system" > /dev/null 2>&1; then
                log_info "lightNVR started successfully (PID: $LIGHTNVR_PID)"
                return 0
            fi
            sleep 1
        done

        log_error "lightNVR failed to start within 30 seconds"
        # Show logs for debugging
        if [ -f "${LIGHTNVR_TEST_DIR}/lightnvr.log" ]; then
            log_error "Last 20 lines of log:"
            tail -20 "${LIGHTNVR_TEST_DIR}/lightnvr.log"
        fi
        exit 1
    fi
}

# Check lightNVR is running
check_lightnvr() {
    log_info "Checking lightNVR connectivity..."

    if ! curl -s -u admin:admin "http://localhost:${LIGHTNVR_PORT}/api/v1/system" > /dev/null 2>&1; then
        log_error "Cannot connect to lightNVR API on port ${LIGHTNVR_PORT}"
        log_info "Start lightNVR with: $LIGHTNVR_BIN -c $LIGHTNVR_TEST_CONFIG"
        log_info "Or run with --start-lightnvr flag"
        exit 1
    fi

    log_info "lightNVR is accessible"
}

# Register virtual test streams with go2rtc
# These are FFmpeg-generated test patterns used for integration testing
register_test_streams() {
    log_info "Registering virtual test streams with go2rtc..."

    # Wait for go2rtc to be ready (lightNVR starts it)
    local retries=30
    while [ $retries -gt 0 ]; do
        if curl -s "http://localhost:${GO2RTC_API_PORT}/api/streams" > /dev/null 2>&1; then
            break
        fi
        sleep 1
        retries=$((retries - 1))
    done

    if [ $retries -eq 0 ]; then
        log_error "go2rtc is not accessible on port ${GO2RTC_API_PORT}"
        return 1
    fi

    # Define test streams using FFmpeg virtual sources
    # Format: name|source
    # Note: Using simple virtual sources that work reliably with go2rtc API registration
    local test_streams=(
        "test_pattern|ffmpeg:virtual?video&size=720#video=h264"
        "test_colorbars|ffmpeg:virtual?video=smptebars&size=720#video=h264"
        "test_red|ffmpeg:virtual?video=color&color=red&size=640x480#video=h264"
        "test_blue|ffmpeg:virtual?video=color&color=blue&size=640x480#video=h264"
        "test_green|ffmpeg:virtual?video=color&color=green&size=640x480#video=h264"
        "test_mandelbrot|ffmpeg:virtual?video=mandelbrot&size=640x480#video=h264"
        "test_pattern2|ffmpeg:virtual?video=testsrc2&size=1080#video=h264"
        "test_lowfps|ffmpeg:virtual?video&size=480&fps=5#video=h264"
    )

    local registered=0
    for stream_def in "${test_streams[@]}"; do
        local name="${stream_def%%|*}"
        local source="${stream_def#*|}"

        # Check if stream already exists
        if curl -s "http://localhost:${GO2RTC_API_PORT}/api/streams" | jq -e ".[\"$name\"]" > /dev/null 2>&1; then
            log_test "  Stream '$name' already exists, skipping"
            registered=$((registered + 1))
            continue
        fi

        # Register stream via go2rtc API
        # URL encode the source
        local encoded_source=$(python3 -c "import urllib.parse; print(urllib.parse.quote('$source', safe=''))" 2>/dev/null || echo "$source")

        local response=$(curl -s -X PUT "http://localhost:${GO2RTC_API_PORT}/api/streams?src=${encoded_source}&name=${name}" 2>&1)

        # Verify registration
        if curl -s "http://localhost:${GO2RTC_API_PORT}/api/streams" | jq -e ".[\"$name\"]" > /dev/null 2>&1; then
            log_test "  ✓ Registered test stream: $name"
            registered=$((registered + 1))
        else
            log_test "  ✗ Failed to register: $name"
        fi
    done

    log_info "Registered $registered/${#test_streams[@]} test streams"
    return 0
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
        local producers=$(echo "$streams" | jq -r ".[\"$stream\"].producers | length" 2>/dev/null)
        # Handle empty or non-numeric values
        if [ -z "$producers" ] || ! [[ "$producers" =~ ^[0-9]+$ ]]; then
            log_test "✗ Stream '$stream' metadata not available"
            failed=1
        elif [ "$producers" -gt 0 ]; then
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

#############################################
# lightNVR Tests
#############################################

# Test: lightNVR system API endpoint
test_lightnvr_system_api() {
    log_info "Test: lightNVR system API..."

    local response=$(curl -s -u admin:admin "http://localhost:${LIGHTNVR_PORT}/api/v1/system")
    local failed=0

    # Check for version field
    if echo "$response" | jq -e '.version' > /dev/null 2>&1; then
        local version=$(echo "$response" | jq -r '.version')
        log_test "✓ System API returns version: $version"
    else
        log_test "✗ System API missing version field"
        failed=1
    fi

    # Check for uptime field
    if echo "$response" | jq -e '.uptime' > /dev/null 2>&1; then
        log_test "✓ System API returns uptime"
    else
        log_test "✗ System API missing uptime field"
        failed=1
    fi

    return $failed
}

# Test: lightNVR streams API - list streams
test_lightnvr_streams_list() {
    log_info "Test: lightNVR streams list API..."

    local response=$(curl -s -u admin:admin "http://localhost:${LIGHTNVR_PORT}/api/v1/streams")

    # Should return an object with streams array
    if echo "$response" | jq -e '.streams' > /dev/null 2>&1; then
        local count=$(echo "$response" | jq '.streams | length')
        log_test "✓ Streams API returns valid response ($count streams)"
        return 0
    else
        log_test "✗ Streams API did not return expected format"
        return 1
    fi
}

# Test: lightNVR add stream via API
test_lightnvr_add_stream() {
    log_info "Test: lightNVR add stream API..."

    # Add a test stream pointing to go2rtc test stream
    local stream_data='{
        "name": "test_integration_stream",
        "url": "rtsp://localhost:18554/test_pattern",
        "enabled": true,
        "width": 1280,
        "height": 720,
        "fps": 15,
        "codec": "h264",
        "priority": 5,
        "record": false
    }'

    local response=$(curl -s -u admin:admin -X POST \
        -H "Content-Type: application/json" \
        -d "$stream_data" \
        "http://localhost:${LIGHTNVR_PORT}/api/v1/streams")

    # Check if stream was created
    if echo "$response" | jq -e '.id' > /dev/null 2>&1; then
        local stream_id=$(echo "$response" | jq -r '.id')
        log_test "✓ Stream created with ID: $stream_id"
        # Store ID for cleanup
        echo "$stream_id" > "${TEST_OUTPUT_DIR}/test_stream_id"
        return 0
    elif echo "$response" | jq -e '.name' > /dev/null 2>&1; then
        # Some APIs return the stream without explicit id field
        log_test "✓ Stream created successfully"
        return 0
    else
        log_test "✗ Failed to create stream: $response"
        return 1
    fi
}

# Test: lightNVR settings API
test_lightnvr_settings() {
    log_info "Test: lightNVR settings API..."

    local response=$(curl -s -u admin:admin "http://localhost:${LIGHTNVR_PORT}/api/v1/settings")
    local failed=0

    # Check for storage_path field
    if echo "$response" | jq -e '.storage_path' > /dev/null 2>&1; then
        log_test "✓ Settings API returns storage_path"
    else
        log_test "✗ Settings API missing storage_path"
        failed=1
    fi

    # Check for web_port field
    if echo "$response" | jq -e '.web_port' > /dev/null 2>&1; then
        local port=$(echo "$response" | jq -r '.web_port')
        log_test "✓ Settings API returns web_port: $port"
    else
        log_test "✗ Settings API missing web_port"
        failed=1
    fi

    return $failed
}

# Test: lightNVR recordings API
test_lightnvr_recordings() {
    log_info "Test: lightNVR recordings API..."

    local response=$(curl -s -u admin:admin "http://localhost:${LIGHTNVR_PORT}/api/v1/recordings")

    # Should return an object with recordings array
    if echo "$response" | jq -e '.recordings' > /dev/null 2>&1; then
        local count=$(echo "$response" | jq '.recordings | length')
        log_test "✓ Recordings API returns valid response ($count recordings)"
        return 0
    else
        log_test "✗ Recordings API did not return expected format"
        return 1
    fi
}

# Test: lightNVR authentication
test_lightnvr_auth() {
    log_info "Test: lightNVR authentication..."
    local failed=0

    # Test with valid credentials
    local valid_response=$(curl -s -w "%{http_code}" -o /dev/null -u admin:admin "http://localhost:${LIGHTNVR_PORT}/api/v1/system")
    if [ "$valid_response" = "200" ]; then
        log_test "✓ Valid credentials accepted (HTTP 200)"
    else
        log_test "✗ Valid credentials rejected (HTTP $valid_response)"
        failed=1
    fi

    # Test with invalid credentials
    local invalid_response=$(curl -s -w "%{http_code}" -o /dev/null -u admin:wrongpassword "http://localhost:${LIGHTNVR_PORT}/api/v1/system")
    if [ "$invalid_response" = "401" ]; then
        log_test "✓ Invalid credentials rejected (HTTP 401)"
    else
        log_test "✗ Invalid credentials not properly rejected (HTTP $invalid_response)"
        failed=1
    fi

    return $failed
}

#############################################
# Run Tests
#############################################

# Run go2rtc tests
run_go2rtc_tests() {
    log_section "Running go2rtc tests..."
    echo ""

    local total=0
    local passed=0

    # Test 1: Streams configured
    total=$((total + 1))
    if test_streams_configured; then passed=$((passed + 1)); fi
    echo ""

    # Test 2: RTSP capture
    total=$((total + 1))
    if test_rtsp_capture; then passed=$((passed + 1)); fi
    echo ""

    # Test 3: Stream metadata
    total=$((total + 1))
    if test_stream_metadata; then passed=$((passed + 1)); fi
    echo ""

    # Test 4: WebRTC endpoint
    total=$((total + 1))
    if test_webrtc_endpoint; then passed=$((passed + 1)); fi
    echo ""

    echo "go2rtc tests: $passed/$total passed"
    echo "$passed $total"
}

# Run lightNVR tests
run_lightnvr_tests() {
    log_section "Running lightNVR tests..."
    echo ""

    local total=0
    local passed=0

    # Test 1: System API
    total=$((total + 1))
    if test_lightnvr_system_api; then passed=$((passed + 1)); fi
    echo ""

    # Test 2: Authentication
    total=$((total + 1))
    if test_lightnvr_auth; then passed=$((passed + 1)); fi
    echo ""

    # Test 3: Streams list
    total=$((total + 1))
    if test_lightnvr_streams_list; then passed=$((passed + 1)); fi
    echo ""

    # Test 4: Add stream
    total=$((total + 1))
    if test_lightnvr_add_stream; then passed=$((passed + 1)); fi
    echo ""

    # Test 5: Settings
    total=$((total + 1))
    if test_lightnvr_settings; then passed=$((passed + 1)); fi
    echo ""

    # Test 6: Recordings
    total=$((total + 1))
    if test_lightnvr_recordings; then passed=$((passed + 1)); fi
    echo ""

    echo "lightNVR tests: $passed/$total passed"
    echo "$passed $total"
}

# Run all tests
run_tests() {
    local total_passed=0
    local total_tests=0

    # Run go2rtc tests unless lightNVR-only mode
    if [ "$LIGHTNVR_ONLY" != true ]; then
        local go2rtc_result=$(run_go2rtc_tests | tail -1)
        local go2rtc_passed=$(echo "$go2rtc_result" | cut -d' ' -f1)
        local go2rtc_total=$(echo "$go2rtc_result" | cut -d' ' -f2)
        total_passed=$((total_passed + go2rtc_passed))
        total_tests=$((total_tests + go2rtc_total))
    fi

    # Run lightNVR tests if started or in lightNVR-only mode
    if [ "$START_LIGHTNVR" = true ] || [ "$LIGHTNVR_ONLY" = true ]; then
        if [ "$LIGHTNVR_ONLY" = true ]; then
            check_lightnvr
        fi

        local lightnvr_result=$(run_lightnvr_tests | tail -1)
        local lightnvr_passed=$(echo "$lightnvr_result" | cut -d' ' -f1)
        local lightnvr_total=$(echo "$lightnvr_result" | cut -d' ' -f2)
        total_passed=$((total_passed + lightnvr_passed))
        total_tests=$((total_tests + lightnvr_total))
    fi

    # Summary
    echo "========================================"
    log_info "Test Summary: $total_passed/$total_tests passed"
    local failed=$((total_tests - total_passed))
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

    # Show test mode
    if [ "$FULL_TEST" = true ]; then
        log_info "Mode: Full integration test (lightNVR manages go2rtc)"
    elif [ "$GO2RTC_ONLY" = true ]; then
        log_info "Mode: go2rtc standalone tests"
    elif [ "$START_LIGHTNVR" = true ]; then
        log_info "Mode: lightNVR + go2rtc tests"
    else
        log_info "Mode: go2rtc standalone tests (use --full for complete integration)"
    fi
    echo ""

    check_prerequisites

    # Start services based on mode
    if [ "$START_LIGHTNVR" = true ]; then
        # lightNVR mode: lightNVR will start and manage go2rtc
        start_lightnvr

        # Register test streams with go2rtc (lightNVR started it)
        register_test_streams

        # Wait a moment for streams to initialize
        sleep 2

        check_go2rtc
    elif [ "$GO2RTC_ONLY" = true ] || [ "$START_GO2RTC" = true ]; then
        # Standalone go2rtc mode
        start_go2rtc
        check_go2rtc
    fi
    echo ""

    run_tests
}

main "$@"
