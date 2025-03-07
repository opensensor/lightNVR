#!/bin/bash

# LightNVR Build Script

# Exit on error
set -e

# Default build type
BUILD_TYPE="Debug"

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    key="$1"
    case $key in
        --release)
            BUILD_TYPE="Release"
            shift
            ;;
        --debug)
            BUILD_TYPE="Debug"
            shift
            ;;
        --clean)
            CLEAN=1
            shift
            ;;
        --help)
            echo "Usage: $0 [options]"
            echo "Options:"
            echo "  --release    Build in release mode"
            echo "  --debug      Build in debug mode (default)"
            echo "  --clean      Clean build directory before building"
            echo "  --help       Show this help message"
            exit 0
            ;;
        *)
            echo "Unknown option: $key"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Set build directory
BUILD_DIR="build/$BUILD_TYPE"

# Clean build directory if requested
if [ -n "$CLEAN" ] && [ -d "$BUILD_DIR" ]; then
    echo "Cleaning build directory: $BUILD_DIR"
    rm -rf "$BUILD_DIR"
fi

# Create build directory if it doesn't exist
mkdir -p "$BUILD_DIR"

# Check for required dependencies
echo "Checking dependencies..."

# Check for CMake
if ! command -v cmake &> /dev/null; then
    echo "CMake is required but not installed. Please install CMake."
    exit 1
fi

# Check for GCC/G++
if ! command -v gcc &> /dev/null || ! command -v g++ &> /dev/null; then
    echo "GCC/G++ is required but not installed. Please install GCC/G++."
    exit 1
fi

# Check for SQLite3
if ! pkg-config --exists sqlite3; then
    echo "SQLite3 development files are required but not installed."
    echo "Please install libsqlite3-dev (Debian/Ubuntu) or sqlite-devel (Fedora/RHEL)."
    exit 1
fi

# Check for FFmpeg
if ! pkg-config --exists libavcodec libavformat libavutil libswscale; then
    echo "FFmpeg development files are required but not installed."
    echo "Please install libavcodec-dev libavformat-dev libavutil-dev libswscale-dev (Debian/Ubuntu)"
    echo "or ffmpeg-devel (Fedora/RHEL)."
    exit 1
fi

# Configure the build
echo "Configuring build..."
cd "$BUILD_DIR"
cmake -DCMAKE_BUILD_TYPE="$BUILD_TYPE" ../..

# Build the project
echo "Building LightNVR..."
cmake --build . -- -j$(nproc)

# Report success
echo "Build completed successfully!"
echo "Binary location: $BUILD_DIR/bin/lightnvr"

# Create a symbolic link to the binary in the project root
if [ -f "bin/lightnvr" ]; then
    cd ../..
    ln -sf "$BUILD_DIR/bin/lightnvr" lightnvr
    echo "Created symbolic link: lightnvr -> $BUILD_DIR/bin/lightnvr"
fi

echo "Run ./lightnvr to start the NVR software"
