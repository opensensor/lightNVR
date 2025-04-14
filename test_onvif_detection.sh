#!/bin/bash

# Test script for ONVIF detection functionality
# This script demonstrates how to use the ONVIF detection feature

# Set your credentials
USERNAME="thingino"
PASSWORD="thingino"

# Set camera IP
CAMERA_IP="192.168.50.49"

# Function to test ONVIF detection
test_onvif_detection() {
    echo "Testing ONVIF detection with camera at $CAMERA_IP"
    echo "Username: $USERNAME"
    echo "Password: $PASSWORD"
    
    # Create a test stream configuration
    cat > test_stream.json << EOF
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
    
    echo "Created test stream configuration"
    echo "To add this stream to your LightNVR instance, use the web interface or API"
    echo "The ONVIF detection will automatically start when the stream is added"
    
    echo "To manually test ONVIF detection, you can use the following commands:"
    echo "1. Initialize ONVIF detection system:"
    echo "   init_onvif_detection_system()"
    echo "2. Detect motion using ONVIF:"
    echo "   detect_motion_onvif(\"http://$CAMERA_IP\", \"$USERNAME\", \"$PASSWORD\", result, \"onvif_test\")"
    echo "3. Shutdown ONVIF detection system:"
    echo "   shutdown_onvif_detection_system()"
    
    echo "For more details, see the implementation in:"
    echo "- include/video/onvif_detection.h"
    echo "- include/video/onvif_detection_integration.h"
    echo "- src/video/onvif_detection.c"
    echo "- src/video/onvif_detection_integration.c"
}

# Run the test
test_onvif_detection
