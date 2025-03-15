# Building LightNVR

This document provides instructions for building the LightNVR software from source.

## Prerequisites

Before building LightNVR, you need to install the following dependencies:

### Debian/Ubuntu

```bash
sudo apt-get update
sudo apt-get install -y \
    build-essential \
    cmake \
    pkg-config \
    libsqlite3-dev \
    libavcodec-dev \
    libavformat-dev \
    libavutil-dev \
    libswscale-dev \
    libmicrohttpd-dev \
    libcurl4-openssl-dev \
    libssl-dev
```

### Fedora/RHEL/CentOS

```bash
sudo dnf install -y \
    gcc \
    gcc-c++ \
    make \
    cmake \
    pkgconfig \
    sqlite-devel \
    ffmpeg-devel \
    libmicrohttpd-devel \
    libcurl-devel \
    openssl-devel
```

### Arch Linux

```bash
sudo pacman -S \
    base-devel \
    cmake \
    sqlite \
    ffmpeg \
    libmicrohttpd \
    curl \
    openssl
```

## Building

LightNVR includes a build script that simplifies the build process. To build the software:

```bash
# Clone the repository (if you haven't already)
git clone https://github.com/opensensor/lightnvr.git
cd lightnvr

# Build in debug mode (default)
./scripts/build.sh

# Or build in release mode
./scripts/build.sh --release

# Clean build directory before building
./scripts/build.sh --clean
```

### Building with or without SOD

LightNVR can be built with or without SOD (an embedded computer vision & machine learning library) support. By default, SOD is enabled.

```bash
# Build with SOD support (default)
./scripts/build_with_sod.sh

# Build without SOD support
./scripts/build_without_sod.sh
```

When built without SOD, LightNVR will still function normally but will not have object detection capabilities unless the SOD library is installed separately and available at runtime.

For more information about SOD integration, see [SOD Integration](SOD_INTEGRATION.md).

The build script will:
1. Check for required dependencies
2. Configure the build using CMake
3. Build the software
4. Create a symbolic link to the binary in the project root

## Manual Build

If you prefer to build manually without using the build script:

```bash
# Create build directory
mkdir -p build/Release
cd build/Release

# Configure (with SOD enabled)
cmake -DCMAKE_BUILD_TYPE=Release -DENABLE_SOD=ON ../..

# Or configure without SOD
# cmake -DCMAKE_BUILD_TYPE=Release -DENABLE_SOD=OFF ../..

# Build
cmake --build . -- -j$(nproc)

# Return to project root
cd ../..
```

## Running

After building, you can run LightNVR directly:

```bash
# If you used the build script
./lightnvr

# Or run the binary directly
./build/Release/bin/lightnvr
```

By default, LightNVR will look for a configuration file in the following locations:
1. `./lightnvr.conf` (current directory)
2. `/etc/lightnvr/lightnvr.conf`

You can specify a different configuration file using the `-c` option:

```bash
./lightnvr -c /path/to/config.conf
```

## Installation

To install LightNVR system-wide, you can use the provided installation script:

```bash
# Build in release mode first
./scripts/build.sh --release

# Install (requires root privileges)
sudo ./scripts/install.sh
```

The installation script will:
1. Install the binary to `/usr/local/bin/lightnvr`
2. Install configuration files to `/etc/lightnvr/`
3. Create data directories in `/var/lib/lightnvr/`
4. Create a systemd service file

You can customize the installation paths using options:

```bash
sudo ./scripts/install.sh --prefix=/opt --config-dir=/etc/custom/lightnvr
```

See `./scripts/install.sh --help` for all available options.

## Cross-Compiling for Ingenic A1

To cross-compile LightNVR for the Ingenic A1 SoC, you need to set up a cross-compilation toolchain. Detailed instructions for cross-compiling will be provided in a separate document.

## Troubleshooting

### Build Errors

If you encounter build errors, try the following:

1. Make sure all dependencies are installed
2. Clean the build directory: `./scripts/build.sh --clean`
3. Check the CMake output for specific error messages

### Runtime Errors

If LightNVR fails to start or crashes:

1. Check the log file for error messages: `/var/log/lightnvr/lightnvr.log`
2. Verify that the configuration file is valid
3. Ensure that all required directories exist and have the correct permissions
