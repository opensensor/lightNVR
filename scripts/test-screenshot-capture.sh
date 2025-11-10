#!/bin/bash

##
# Quick test script for screenshot capture
# Tests against a running LightNVR instance
##

set -e

URL="${1:-http://localhost:8080}"

echo "Testing screenshot capture against: $URL"
echo ""

# Check if LightNVR is accessible
echo "Checking LightNVR accessibility..."
if ! curl -s -o /dev/null -w "%{http_code}" "$URL/login.html" | grep -q "200"; then
  echo "Error: Cannot access LightNVR at $URL"
  echo "Make sure LightNVR is running first."
  exit 1
fi
echo "✓ LightNVR is accessible"
echo ""

# Run screenshot capture
echo "Running screenshot capture..."
node scripts/capture-screenshots.js --url "$URL"

echo ""
echo "✓ Test complete!"
echo ""
echo "Check docs/images/ for screenshots:"
ls -lh docs/images/*.png 2>/dev/null || echo "No screenshots found"

