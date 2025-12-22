#!/bin/bash
# Run Playwright tests with proper output

set -e

echo "========================================"
echo "LightNVR Playwright Test Runner"
echo "========================================"
echo ""

# Parse arguments
HEADED=""
PROJECT="ui"
DEBUG=""

while [[ $# -gt 0 ]]; do
    case $1 in
        --headed)
            HEADED="--headed"
            export HEADED=1
            shift
            ;;
        --project=*)
            PROJECT="${1#*=}"
            shift
            ;;
        --debug)
            DEBUG="--debug"
            export PWDEBUG=1
            shift
            ;;
        --ui)
            # Playwright UI mode
            exec npx playwright test --ui
            ;;
        *)
            shift
            ;;
    esac
done

# Cleanup any existing processes
echo "[INFO] Cleaning up existing processes..."
pkill -9 -f lightnvr 2>/dev/null || true
pkill -9 -f go2rtc 2>/dev/null || true
rm -rf /tmp/lightnvr-test
sleep 2

# Create test-results directory
mkdir -p test-results

echo "[INFO] Starting Playwright tests..."
echo "[INFO] Project: $PROJECT"
echo "[INFO] Headed: ${HEADED:-headless}"
echo ""

# Run playwright
npx playwright test --project="$PROJECT" $HEADED $DEBUG

echo ""
echo "[INFO] Tests complete!"
echo "[INFO] Screenshots saved to: test-results/"
echo "[INFO] HTML report: npx playwright show-report"

