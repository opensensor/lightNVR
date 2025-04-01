#!/bin/bash
# Script to build web assets using Snowpack and update HTML files

# Set directories
SCRIPT_DIR="$(dirname "$0")"
BUILD_SCRIPT="$SCRIPT_DIR/build_web_snowpack.sh"
UPDATE_SCRIPT="$SCRIPT_DIR/update_html_for_snowpack.sh"

# Check if the build script exists
if [ ! -f "$BUILD_SCRIPT" ]; then
    echo "Build script not found: $BUILD_SCRIPT"
    exit 1
fi

# Check if the update script exists
if [ ! -f "$UPDATE_SCRIPT" ]; then
    echo "Update script not found: $UPDATE_SCRIPT"
    exit 1
fi

# Run the build script
echo "Running Snowpack build script..."
"$BUILD_SCRIPT"

# Check if build was successful
if [ $? -ne 0 ]; then
    echo "Build failed. Aborting update."
    exit 1
fi

# Run the update script
echo "Running HTML update script..."
"$UPDATE_SCRIPT"

# Check if update was successful
if [ $? -ne 0 ]; then
    echo "Update failed."
    exit 1
fi

echo "Build and update completed successfully."
echo "You can now test the Snowpack-built application."
