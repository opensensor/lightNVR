#!/bin/bash

# Exit on error
set -e

# Default build type
BUILD_TYPE="Release"
ENABLE_SOD=1
ENABLE_TESTS=1
ENABLE_GO2RTC=1

# Default SOD linking mode
SOD_DYNAMIC=0

# Default go2rtc settings
GO2RTC_BINARY_PATH="/usr/local/bin/go2rtc"
GO2RTC_CONFIG_DIR="/etc/lightnvr/go2rtc"
GO2RTC_API_PORT=1984

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
        --sod-dynamic)
            SOD_DYNAMIC=1
            shift
            ;;
        --sod-static)
            SOD_DYNAMIC=0
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
        --with-go2rtc)
            ENABLE_GO2RTC=1
            shift
            ;;
        --without-go2rtc)
            ENABLE_GO2RTC=0
            shift
            ;;
        --go2rtc-binary=*)
            GO2RTC_BINARY_PATH="${key#*=}"
            shift
            ;;
        --go2rtc-config-dir=*)
            GO2RTC_CONFIG_DIR="${key#*=}"
            shift
            ;;
        --go2rtc-api-port=*)
            GO2RTC_API_PORT="${key#*=}"
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
            echo "  --sod-dynamic      Use dynamic linking for SOD (builds libsod.so)"
            echo "  --sod-static       Use static linking for SOD (default)"
            echo "  --with-tests       Build test suite (default)"
            echo "  --without-tests    Build without test suite"
            echo "  --with-go2rtc      Build with go2rtc integration (default)"
            echo "  --without-go2rtc   Build without go2rtc integration"
            echo "  --go2rtc-binary=PATH  Set go2rtc binary path (default: /usr/local/bin/go2rtc)"
            echo "  --go2rtc-config-dir=DIR  Set go2rtc config directory (default: /etc/lightnvr/go2rtc)"
            echo "  --go2rtc-api-port=PORT  Set go2rtc API port (default: 1984)"
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


# Add this to your build script before building
export LD_LIBRARY_PATH="$PWD/build/Release/src/sod:$LD_LIBRARY_PATH"

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
    if [ "$SOD_DYNAMIC" -eq 1 ]; then
        SOD_OPTION="-DENABLE_SOD=ON -DSOD_DYNAMIC_LINK=ON"
        echo "Building with SOD support as a shared library (dynamic linking)"
    else
        SOD_OPTION="-DENABLE_SOD=ON -DSOD_DYNAMIC_LINK=OFF"
        echo "Building with SOD support as a static library (static linking)"
    fi
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

# Configure go2rtc options
GO2RTC_OPTION=""
if [ "$ENABLE_GO2RTC" -eq 1 ]; then
    GO2RTC_OPTION="-DENABLE_GO2RTC=ON -DGO2RTC_BINARY_PATH=\"$GO2RTC_BINARY_PATH\" -DGO2RTC_CONFIG_DIR=\"$GO2RTC_CONFIG_DIR\" -DGO2RTC_API_PORT=$GO2RTC_API_PORT"
    echo "Building with go2rtc integration"
    echo "  go2rtc binary path: $GO2RTC_BINARY_PATH"
    echo "  go2rtc config directory: $GO2RTC_CONFIG_DIR"
    echo "  go2rtc API port: $GO2RTC_API_PORT"
else
    GO2RTC_OPTION="-DENABLE_GO2RTC=OFF"
    echo "Building without go2rtc integration"
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
cmake -DCMAKE_BUILD_TYPE="$BUILD_TYPE" $SOD_OPTION $TEST_OPTION $GO2RTC_OPTION $FFMPEG_CMAKE_OPTIONS \
      -DCMAKE_MODULE_PATH="$CMAKE_MODULE_PATH" ../..

# Return to project root
cd ../..

# Build the project
echo "Building LightNVR against custom FFmpeg..."
cmake --build "$BUILD_DIR" -- -j$(nproc)

# Report success
echo "Build completed successfully!"
echo "Binary location: $BUILD_DIR/bin/lightnvr"

# Check if the binary is linked to libsod.so when dynamic linking is enabled
if [ "$ENABLE_SOD" -eq 1 ] && [ "$SOD_DYNAMIC" -eq 1 ]; then
    echo "Checking if lightnvr is linked to libsod.so..."
    if ldd "$BUILD_DIR/bin/lightnvr" | grep -q "libsod.so"; then
        echo "SUCCESS: lightnvr is correctly linked to libsod.so"
    else
        echo "WARNING: lightnvr is not linked to libsod.so"
        echo "This might indicate a problem with the dynamic linking setup."

        # Check if libsod.so exists in the build output
        echo "Checking for libsod.so in the build directory..."
        if find "$BUILD_DIR" -name "libsod.so" | grep -q .; then
            echo "Found libsod.so in the build directory. It might not be in the library path."
            LIBSOD_PATH=$(find "$BUILD_DIR" -name "libsod.so" | head -1)

            # Try to load the library directly
            echo "You can run the application with:"
            echo "LD_LIBRARY_PATH=\"$(dirname \"$LIBSOD_PATH\"):$LD_LIBRARY_PATH\" $BUILD_DIR/bin/lightnvr"
        else
            echo "Could not find libsod.so in the build directory."
            echo "This suggests a problem with building the shared library."
        fi
    fi
fi

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
