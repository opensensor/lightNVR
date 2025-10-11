#!/bin/bash

# Test script for ONVIF detection functionality
# This script demonstrates how to use the ONVIF detection feature

# Set your credentials (leave empty for cameras without authentication)
USERNAME="thingino"
PASSWORD="thingino"

# Set camera IP
CAMERA_IP="192.168.50.49"

# Function to test ONVIF detection with authentication
test_onvif_detection_with_auth() {
    echo "========================================="
    echo "Testing ONVIF detection WITH authentication"
    echo "========================================="
    echo "Camera IP: $CAMERA_IP"
    echo "Username: $USERNAME"
    echo "Password: $PASSWORD"
    echo ""

    # Create a test stream configuration
    cat > test_stream_with_auth.json << EOF
{
  "name": "onvif_test",
  "url": "onvif://$USERNAME:$PASSWORD@$CAMERA_IP",
  "enabled": true,
  "width": 1920,
  "height": 1080,
  "fps": 15,
  "codec": "h264",
  "priority": 5,
  "record": true,
  "segment_duration": 60,
  "detection_based_recording": true,
  "detection_model": "onvif",
  "detection_interval": 10,
  "detection_threshold": 0.5,
  "pre_detection_buffer": 5,
  "post_detection_buffer": 10,
  "streaming_enabled": true,
  "protocol": 0,
  "record_audio": false,
  "onvif_username": "$USERNAME",
  "onvif_password": "$PASSWORD",
  "onvif_profile": "Profile_1",
  "onvif_discovery_enabled": true,
  "is_onvif": true
}
EOF

    echo "Created test stream configuration: test_stream_with_auth.json"
    echo ""
}

# Function to test ONVIF detection without authentication
test_onvif_detection_no_auth() {
    echo "========================================="
    echo "Testing ONVIF detection WITHOUT authentication"
    echo "========================================="
    echo "Camera IP: $CAMERA_IP"
    echo "Username: (empty)"
    echo "Password: (empty)"
    echo ""

    # Create a test stream configuration without credentials
    cat > test_stream_no_auth.json << EOF
{
  "name": "onvif_test_no_auth",
  "url": "rtsp://$CAMERA_IP:554/stream",
  "enabled": true,
  "width": 1920,
  "height": 1080,
  "fps": 15,
  "codec": "h264",
  "priority": 5,
  "record": true,
  "segment_duration": 60,
  "detection_based_recording": true,
  "detection_model": "onvif",
  "detection_interval": 10,
  "detection_threshold": 0.5,
  "pre_detection_buffer": 5,
  "post_detection_buffer": 10,
  "streaming_enabled": true,
  "protocol": 0,
  "record_audio": false,
  "onvif_username": "",
  "onvif_password": "",
  "onvif_profile": "Profile_1",
  "onvif_discovery_enabled": true,
  "is_onvif": true
}
EOF

    echo "Created test stream configuration: test_stream_no_auth.json"
    echo ""
}

# Main function
main() {
    echo "========================================="
    echo "ONVIF Detection Test Script"
    echo "========================================="
    echo ""
    echo "This script creates test configurations for ONVIF detection."
    echo "It demonstrates both authenticated and non-authenticated scenarios."
    echo ""

    # Test with authentication
    test_onvif_detection_with_auth

    # Test without authentication
    test_onvif_detection_no_auth

    echo "========================================="
    echo "Next Steps"
    echo "========================================="
    echo ""
    echo "To add these streams to your LightNVR instance:"
    echo "1. Use the web interface to add a new stream"
    echo "2. Or use the API to POST the JSON configuration"
    echo ""
    echo "The ONVIF detection will automatically start when the stream is added."
    echo ""
    echo "To manually test ONVIF detection, you can use the following commands:"
    echo "1. Initialize ONVIF detection system:"
    echo "   init_onvif_detection_system()"
    echo "2. Detect motion using ONVIF (with auth):"
    echo "   detect_motion_onvif(\"http://$CAMERA_IP\", \"$USERNAME\", \"$PASSWORD\", result, \"onvif_test\")"
    echo "3. Detect motion using ONVIF (without auth):"
    echo "   detect_motion_onvif(\"http://$CAMERA_IP\", \"\", \"\", result, \"onvif_test_no_auth\")"
    echo "4. Shutdown ONVIF detection system:"
    echo "   shutdown_onvif_detection_system()"
    echo ""
    echo "For more details, see:"
    echo "- docs/ONVIF_DETECTION.md"
    echo "- ONVIF_CREDENTIALS_FIX.md"
    echo "- include/video/onvif_detection.h"
    echo "- src/video/onvif_detection.c"
    echo ""
    echo "Check the logs for detailed information about the ONVIF detection process."
    echo "Look for messages like:"
    echo "  - 'Creating ONVIF request without authentication'"
    echo "  - 'Creating ONVIF request with WS-Security authentication'"
    echo "  - 'Using camera without authentication (empty credentials)'"
    echo "  - 'Using camera with authentication (username: ...)'"
}

# Run the main function
main
