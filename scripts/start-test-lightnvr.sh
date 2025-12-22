#!/bin/bash

##
# Start lightNVR with test configuration
# This script sets up temp directories and starts lightNVR for integration testing
##

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Test configuration
TEST_DIR="/tmp/lightnvr-test"
LIGHTNVR_BIN="${PROJECT_ROOT}/build/bin/lightnvr"
CONFIG_FILE="${PROJECT_ROOT}/config/lightnvr-test.ini"
LIGHTNVR_PORT=18080
GO2RTC_API_PORT=11984

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log_info() { echo -e "${GREEN}[INFO]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

usage() {
    echo "Usage: $0 [command] [options]"
    echo ""
    echo "Commands:"
    echo "  start     Start lightNVR with test configuration"
    echo "  stop      Stop the test lightNVR instance"
    echo "  status    Check if test lightNVR is running"
    echo "  setup     Set up test directories only"
    echo "  cleanup   Remove test directories and stop lightNVR"
    echo ""
    echo "Options:"
    echo "  --wait    Wait for lightNVR to be ready (for start command)"
    echo "  --verbose Enable verbose logging"
    echo ""
    exit 1
}

setup_test_dirs() {
    log_info "Setting up test directories..."
    mkdir -p "${TEST_DIR}/recordings"
    mkdir -p "${TEST_DIR}/recordings/mp4"
    mkdir -p "${TEST_DIR}/recordings/hls"
    mkdir -p "${TEST_DIR}/models"
    mkdir -p "${TEST_DIR}/go2rtc"
    log_info "Test directories created at ${TEST_DIR}"
}

cleanup_test_dirs() {
    log_info "Cleaning up test directories..."
    rm -rf "${TEST_DIR}"
    log_info "Test directories removed"
}

check_lightnvr_binary() {
    if [ ! -f "$LIGHTNVR_BIN" ]; then
        log_error "lightNVR binary not found at $LIGHTNVR_BIN"
        log_error "Please build lightNVR first: ./scripts/build.sh"
        exit 1
    fi
}

wait_for_lightnvr() {
    local max_wait=30
    local wait_time=0
    log_info "Waiting for lightNVR to be ready (port $LIGHTNVR_PORT)..."
    
    while [ $wait_time -lt $max_wait ]; do
        if curl -s -u admin:admin "http://localhost:${LIGHTNVR_PORT}/api/v1/system" >/dev/null 2>&1; then
            log_info "lightNVR is ready!"
            return 0
        fi
        sleep 1
        wait_time=$((wait_time + 1))
    done
    
    log_error "lightNVR failed to start within ${max_wait} seconds"
    return 1
}

start_lightnvr() {
    local wait_flag="$1"
    local verbose_flag="$2"
    
    check_lightnvr_binary
    setup_test_dirs
    
    # Check if already running
    if [ -f "${TEST_DIR}/lightnvr.pid" ]; then
        local pid=$(cat "${TEST_DIR}/lightnvr.pid")
        if kill -0 "$pid" 2>/dev/null; then
            log_warn "lightNVR already running (PID: $pid)"
            return 0
        fi
    fi
    
    log_info "Starting lightNVR with test configuration..."
    
    local cmd="$LIGHTNVR_BIN -c $CONFIG_FILE"
    if [ "$verbose_flag" = "true" ]; then
        cmd="$cmd --verbose"
    fi
    
    cd "$PROJECT_ROOT"
    $cmd &
    local pid=$!
    echo "$pid" > "${TEST_DIR}/lightnvr.pid"
    
    log_info "lightNVR started (PID: $pid)"
    
    if [ "$wait_flag" = "true" ]; then
        wait_for_lightnvr
    fi
}

stop_lightnvr() {
    if [ -f "${TEST_DIR}/lightnvr.pid" ]; then
        local pid=$(cat "${TEST_DIR}/lightnvr.pid")
        if kill -0 "$pid" 2>/dev/null; then
            log_info "Stopping lightNVR (PID: $pid)..."
            kill "$pid" 2>/dev/null || true
            sleep 2
            # Force kill if still running
            if kill -0 "$pid" 2>/dev/null; then
                kill -9 "$pid" 2>/dev/null || true
            fi
            log_info "lightNVR stopped"
        else
            log_info "lightNVR is not running"
        fi
        rm -f "${TEST_DIR}/lightnvr.pid"
    else
        log_info "No PID file found, lightNVR may not be running"
    fi
}

status_lightnvr() {
    if [ -f "${TEST_DIR}/lightnvr.pid" ]; then
        local pid=$(cat "${TEST_DIR}/lightnvr.pid")
        if kill -0 "$pid" 2>/dev/null; then
            log_info "lightNVR is running (PID: $pid)"
            # Check API
            if curl -s -u admin:admin "http://localhost:${LIGHTNVR_PORT}/api/v1/system" >/dev/null 2>&1; then
                log_info "API is responsive on port $LIGHTNVR_PORT"
            else
                log_warn "API is not responding"
            fi
            return 0
        fi
    fi
    log_info "lightNVR is not running"
    return 1
}

# Parse command and options
COMMAND="${1:-}"
shift || true

WAIT_FLAG="false"
VERBOSE_FLAG="false"

while [ $# -gt 0 ]; do
    case "$1" in
        --wait) WAIT_FLAG="true" ;;
        --verbose) VERBOSE_FLAG="true" ;;
        *) log_error "Unknown option: $1"; usage ;;
    esac
    shift
done

case "$COMMAND" in
    start) start_lightnvr "$WAIT_FLAG" "$VERBOSE_FLAG" ;;
    stop) stop_lightnvr ;;
    status) status_lightnvr ;;
    setup) setup_test_dirs ;;
    cleanup) stop_lightnvr; cleanup_test_dirs ;;
    *) usage ;;
esac

