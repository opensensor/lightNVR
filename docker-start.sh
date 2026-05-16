#!/bin/bash
set -e

cleanup() {
    echo "Cleaning up processes..."
}
trap cleanup EXIT

echo "Starting LightNVR; LightNVR will manage go2rtc"
/bin/lightnvr -c /etc/lightnvr/lightnvr.ini
