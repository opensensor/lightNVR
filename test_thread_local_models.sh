#!/bin/bash

# Test script for thread-local model approach
# This script builds and runs the application, then monitors memory usage

echo "Building application..."
make clean && make

if [ $? -ne 0 ]; then
    echo "Build failed!"
    exit 1
fi

echo "Starting application with multiple detection threads..."
./nvr_server -c config/test_multi_stream.json &
APP_PID=$!

echo "Application started with PID: $APP_PID"
echo "Monitoring memory usage for 60 seconds..."

# Monitor memory usage for 60 seconds
for i in {1..12}; do
    echo "Memory snapshot $i/12 (5 seconds interval):"
    ps -o pid,vsz,rss,command -p $APP_PID
    sleep 5
done

echo "Stopping application..."
kill -SIGTERM $APP_PID
wait $APP_PID

echo "Test completed!"
