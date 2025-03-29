#!/bin/bash

# Exit on error
set -e

# Default build type
BUILD_TYPE="Release"
ENABLE_SOD=1
ENABLE_TESTS=1

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
        --with-sod)
            ENABLE_SOD=1
            shift
            ;;
        --without-sod)
            ENABLE_SOD=0
            shift
            ;;
        --with-tests)
            ENABLE_TESTS=1
            shift
            ;;
        --without-tests)
            ENABLE_TESTS=0
            shift
            ;;
        --help)
            echo "Usage: $0 [options]"
            echo "Options:"
            echo "  --release          Build in release mode (default)"
            echo "  --debug            Build in debug mode"
            echo "  --clean            Clean build directory before building"
            echo "  --with-sod         Build with SOD support (default)"
            echo "  --without-sod      Build without SOD support"
            echo "  --with-tests       Build test suite (default)"
            echo "  --without-tests    Build without test suite"
            echo "  --help             Show this help message"
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

# Configure SOD and Tests options
SOD_OPTION=""
if [ "$ENABLE_SOD" -eq 1 ]; then
    SOD_OPTION="-DENABLE_SOD=ON -DSOD_DYNAMIC_LINK=OFF"
    echo "Building with SOD support (static linking)"
else
    SOD_OPTION="-DENABLE_SOD=OFF"
    echo "Building without SOD support"
fi

TEST_OPTION=""
if [ "$ENABLE_TESTS" -eq 1 ]; then
    TEST_OPTION="-DBUILD_TESTS=ON"
    echo "Building with test suite"
else
    TEST_OPTION="-DBUILD_TESTS=OFF"
    echo "Building without test suite"
fi

# Create a temporary CMake module to find the custom FFmpeg
mkdir -p cmake/modules
cat > cmake/modules/FindFFmpeg.cmake << 'EOF'
# Custom FindFFmpeg.cmake module to locate custom-built FFmpeg
# This will override any system module with the same name

# Find the include directories
find_path(FFMPEG_INCLUDE_DIR libavcodec/avcodec.h
    HINTS
    ${FFMPEG_DIR}
    ${FFMPEG_DIR}/include
    PATH_SUFFIXES ffmpeg
)

# Find each of the libraries
find_library(AVCODEC_LIBRARY
    NAMES avcodec
    HINTS ${FFMPEG_DIR}/lib
)

find_library(AVFORMAT_LIBRARY
    NAMES avformat
    HINTS ${FFMPEG_DIR}/lib
)

find_library(AVUTIL_LIBRARY
    NAMES avutil
    HINTS ${FFMPEG_DIR}/lib
)

find_library(SWSCALE_LIBRARY
    NAMES swscale
    HINTS ${FFMPEG_DIR}/lib
)

find_library(AVDEVICE_LIBRARY
    NAMES avdevice
    HINTS ${FFMPEG_DIR}/lib
)

find_library(SWRESAMPLE_LIBRARY
    NAMES swresample
    HINTS ${FFMPEG_DIR}/lib
)

# Set the FFMPEG_LIBRARIES variable
set(FFMPEG_LIBRARIES
    ${AVCODEC_LIBRARY}
    ${AVFORMAT_LIBRARY}
    ${AVUTIL_LIBRARY}
    ${SWSCALE_LIBRARY}
    ${AVDEVICE_LIBRARY}
    ${SWRESAMPLE_LIBRARY}
)

# Set the FFMPEG_INCLUDE_DIRS variable
set(FFMPEG_INCLUDE_DIRS ${FFMPEG_INCLUDE_DIR})

# Handle standard args
include(FindPackageHandleStandardArgs)
find_package_handle_standard_args(FFmpeg
    REQUIRED_VARS
    FFMPEG_INCLUDE_DIR
    AVCODEC_LIBRARY
    AVFORMAT_LIBRARY
    AVUTIL_LIBRARY
)

mark_as_advanced(
    FFMPEG_INCLUDE_DIR
    AVCODEC_LIBRARY
    AVFORMAT_LIBRARY
    AVUTIL_LIBRARY
    SWSCALE_LIBRARY
    AVDEVICE_LIBRARY
    SWRESAMPLE_LIBRARY
)
EOF

# Configure the build
cd "$BUILD_DIR"

# Use our custom module path
CMAKE_MODULE_PATH="$(pwd)/../../cmake/modules"
cmake -DCMAKE_BUILD_TYPE="$BUILD_TYPE" $SOD_OPTION $TEST_OPTION $FFMPEG_CMAKE_OPTIONS \
      -DCMAKE_MODULE_PATH="$CMAKE_MODULE_PATH" ../..

# Return to project root
cd ../..

# Build the project
echo "Building LightNVR against custom FFmpeg..."
cmake --build "$BUILD_DIR" -- -j$(nproc)

# Report success
echo "Build completed successfully!"
echo "Binary location: $BUILD_DIR/bin/lightnvr"

# Look for test binaries
if [ "$ENABLE_TESTS" -eq 1 ] && [ "$ENABLE_SOD" -eq 1 ]; then
    if [ -f "$BUILD_DIR/bin/test_sod_realnet" ]; then
        echo "SOD test binary: $BUILD_DIR/bin/test_sod_realnet"
        echo ""
        echo "Usage example:"
        echo "  $BUILD_DIR/bin/test_sod_realnet path/to/image.jpg path/to/face.realnet.sod output.jpg"
    else
        echo "Warning: SOD test binary was not built correctly."
        echo "Check CMake configuration and build logs."
    fi
fi

# Create a symbolic link to the binary in the project root
if [ -f "$BUILD_DIR/bin/lightnvr" ]; then
    ln -sf "$BUILD_DIR/bin/lightnvr" lightnvr
    echo "Created symbolic link: lightnvr -> $BUILD_DIR/bin/lightnvr"
fi
