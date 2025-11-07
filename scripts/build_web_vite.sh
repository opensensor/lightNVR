#!/bin/bash
# Script to build web assets using Vite

# Parse command line arguments
BUILD_SOURCEMAPS=false
while [[ $# -gt 0 ]]; do
    case $1 in
        -m|--with-maps)
            BUILD_SOURCEMAPS=true
            shift
            ;;
        *)
            echo "Unknown option: $1"
            echo "Usage: $0 [-m|--with-maps]"
            echo "  -m, --with-maps    Build with source maps (increases build size)"
            exit 1
            ;;
    esac
done

# Set directories
WEB_DIR="$(dirname "$(dirname "$0")")/web"
BUILD_DIR="$WEB_DIR/dist"

# Check if Node.js and npm are installed
if ! command -v npm &> /dev/null; then
    echo "npm is not installed. Please install Node.js and npm first."
    exit 1
fi

# Extract version from CMakeLists.txt and generate version.js
echo "Extracting version from CMakeLists.txt..."
"$(dirname "$0")/extract_version.js"
if [ $? -ne 0 ]; then
    echo "Failed to extract version. Please check the error messages above."
    exit 1
fi

# Navigate to web directory
cd "$WEB_DIR" || exit 1

# Install dependencies if node_modules doesn't exist
if [ ! -d "node_modules" ]; then
    echo "Installing dependencies..."
    npm install
fi

# Build with Vite
if [ "$BUILD_SOURCEMAPS" = true ]; then
    echo "Building web assets with Vite (with source maps)..."
    BUILD_SOURCEMAPS=true npm run build
else
    echo "Building web assets with Vite (without source maps)..."
    npm run build
fi

# Check if build was successful
if [ $? -ne 0 ]; then
    echo "Build failed. Please check the error messages above."
    exit 1
fi

echo "Build completed successfully. Output is in $BUILD_DIR"

# Update web_optimization.json to use the new dist directory
CONFIG_FILE="$(dirname "$(dirname "$0")")/config/web_optimization.json"
if [ -f "$CONFIG_FILE" ]; then
    echo "Updating web_optimization.json to use Vite dist directory..."
    # Create a backup of the original file
    cp "$CONFIG_FILE" "${CONFIG_FILE}.bak"
    
    # Update the asset paths - ensure we use the correct paths for Vite's optimized output
    sed -i 's|"js": "web/dist/js"|"js": "web/dist/assets"|g' "$CONFIG_FILE"
    sed -i 's|"js": "web/js"|"js": "web/dist/assets"|g' "$CONFIG_FILE"
    sed -i 's|"css": "web/dist/css"|"css": "web/dist/assets"|g' "$CONFIG_FILE"
    sed -i 's|"css": "web/css"|"css": "web/dist/assets"|g' "$CONFIG_FILE"
    sed -i 's|"images": "web/dist/img"|"images": "web/dist/assets"|g' "$CONFIG_FILE"
    sed -i 's|"images": "web/img"|"images": "web/dist/assets"|g' "$CONFIG_FILE"
    sed -i 's|"fonts": "web/dist/fonts"|"fonts": "web/dist/assets"|g' "$CONFIG_FILE"
    sed -i 's|"fonts": "web/fonts"|"fonts": "web/dist/assets"|g' "$CONFIG_FILE"
    
    echo "Configuration updated."
else
    echo "Warning: Could not find web_optimization.json. You may need to update your server configuration manually."
fi

echo "Vite build process complete."
