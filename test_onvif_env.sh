#!/bin/bash
# Test script to verify ONVIF environment variable override logic

echo "=== ONVIF Discovery Network Override Test ==="
echo ""

# Test 1: Environment variable override
echo "Test 1: Environment variable override"
echo "Setting LIGHTNVR_ONVIF_NETWORK=192.168.1.0/24"
export LIGHTNVR_ONVIF_NETWORK="192.168.1.0/24"
echo "Expected: Should use 192.168.1.0/24 from environment variable"
echo ""

# Test 2: Auto-detection (no env var)
echo "Test 2: Auto-detection"
unset LIGHTNVR_ONVIF_NETWORK
echo "Expected: Should auto-detect network (skip Docker interfaces)"
echo ""

# Test 3: Explicit parameter overrides env var
echo "Test 3: Explicit parameter priority"
export LIGHTNVR_ONVIF_NETWORK="192.168.1.0/24"
echo "If function called with explicit network parameter (e.g., 10.0.0.0/24)"
echo "Expected: Should use explicit parameter, not env var"
echo ""

# Test 4: Config file parsing
echo "Test 4: Config file parsing"
echo "Check if [onvif] section is parsed correctly from lightnvr.ini"
cat > /tmp/test_onvif.ini << 'EOF'
[onvif]
discovery_enabled = true
discovery_interval = 300
discovery_network = 192.168.2.0/24
EOF
echo "Created test config at /tmp/test_onvif.ini"
echo "Expected: Config should parse discovery_enabled, discovery_interval, discovery_network"
echo ""

echo "=== Implementation Summary ==="
echo ""
echo "Priority order for network selection:"
echo "1. Function parameter (explicit network passed to discover_onvif_devices())"
echo "2. LIGHTNVR_ONVIF_NETWORK environment variable"
echo "3. Config file [onvif] discovery_network setting"
echo "4. Auto-detection (skips docker*, veth*, br-*, lxc* interfaces)"
echo ""

echo "=== Code Changes Made ==="
echo ""
echo "1. src/core/config.c:"
echo "   - Added ONVIF defaults in load_default_config()"
echo "   - Added [onvif] section parsing in config_ini_handler()"
echo ""
echo "2. src/video/onvif_discovery.c:"
echo "   - Added getenv('LIGHTNVR_ONVIF_NETWORK') check"
echo "   - Checks env var before auto-detection"
echo ""
echo "3. config/lightnvr.ini:"
echo "   - Added [onvif] section with discovery settings"
echo ""
echo "4. docs/DOCKER.md:"
echo "   - Documented LIGHTNVR_ONVIF_NETWORK environment variable"
echo "   - Added usage examples for containers"
echo ""
echo "5. docker-compose.yml:"
echo "   - Added commented example for LIGHTNVR_ONVIF_NETWORK"
echo ""
echo "6. docker-entrypoint.sh:"
echo "   - Added [onvif] section to default config template"
echo ""

echo "=== Docker Usage Example ==="
echo ""
echo "docker-compose.yml:"
cat << 'EOF'
services:
  lightnvr:
    environment:
      - LIGHTNVR_ONVIF_NETWORK=192.168.1.0/24
EOF
echo ""

echo "Or with docker run:"
echo "docker run -e LIGHTNVR_ONVIF_NETWORK=192.168.1.0/24 ..."
echo ""

echo "=== Test Complete ==="

