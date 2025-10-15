#!/bin/bash
# Deploy new LightNVR binary and trigger recording sync

set -e

echo "=== Deploying new LightNVR binary ==="

# Stop service if running
echo "Stopping lightnvr service..."
sudo systemctl stop lightnvr || true

# Install new binary
echo "Installing new binary..."
sudo cp build/bin/lightnvr /usr/local/bin/lightnvr
sudo chmod +x /usr/local/bin/lightnvr

# Start service
echo "Starting lightnvr service..."
sudo systemctl start lightnvr

# Wait for service to start
echo "Waiting for service to start..."
sleep 5

# Check service status
echo "Checking service status..."
sudo systemctl status lightnvr --no-pager || true

# Wait a bit more for web server to be ready
sleep 2

# Trigger sync via API
echo ""
echo "=== Triggering recording sync via API ==="
curl -X POST http://admin:admin@localhost:8080/api/recordings/sync 2>&1 || {
    echo "Failed to call sync API, trying to check logs..."
    sudo journalctl -u lightnvr --since "1 minute ago" --no-pager | tail -20
}

echo ""
echo "=== Done ==="

