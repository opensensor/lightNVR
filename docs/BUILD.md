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
    git \
    libsqlite3-dev \
    libavcodec-dev \
    libavformat-dev \
    libavutil-dev \
    libswscale-dev \
    libswresample-dev \
    libavfilter-dev \
    libuv1-dev \
    libcjson-dev \
    zlib1g-dev \
    libcurl4-openssl-dev \
    libmbedtls-dev \
    libmosquitto-dev \
    libyaml-dev \
    curl \
    wget
```

**Notes:**
- **FFmpeg 7.1 or newer is required** (`libavcodec` ≥ 61, `libavformat` ≥ 61, `libavutil` ≥ 59,
  `libswscale` ≥ 8, `libswresample` ≥ 5), tested up to FFmpeg 9.0. Debian 13 and Ubuntu 24.04+
  ship a compatible version; Debian 12 and Ubuntu 22.04 do not, so build FFmpeg from source
  there.
- `libmbedtls-dev` is **required** for ONVIF and authentication.
- `libmosquitto-dev` (≥ 2.0) and `libyaml-dev` are optional. Without them CMake prints a
  warning and builds without MQTT, or without go2rtc override validation.
- Building the web UI needs **Node.js 26** (the version in `.nvmrc`). Distribution packages
  are often older; use [NodeSource](https://github.com/nodesource/distributions) or
  [nvm](https://github.com/nvm-sh/nvm).

### Fedora/RHEL/CentOS

```bash
sudo dnf install -y \
    gcc \
    gcc-c++ \
    make \
    cmake \
    pkgconfig \
    git \
    sqlite-devel \
    ffmpeg-devel \
    mosquitto-devel \
    libyaml-devel \
    libcurl-devel \
    libuv-devel \
    cjson-devel \
    zlib-devel \
    mbedtls-devel \
    curl \
    wget
```

**Note**: `mbedtls-devel` is **required** for ONVIF support and authentication system (cryptographic functions).

### Arch Linux

```bash
sudo pacman -S \
    base-devel \
    cmake \
    git \
    sqlite \
    ffmpeg \
    mosquitto \
    libyaml \
    libuv \
    cjson \
    zlib \
    curl \
    wget \
    mbedtls
```

**Note**: `mbedtls` is **required** for ONVIF support and authentication system (cryptographic functions).

## Building

LightNVR includes a build script that simplifies the build process. To build the software:

```bash
# Clone the repository (if you haven't already)
git clone https://github.com/opensensor/lightnvr.git
cd lightnvr

# Initialize submodules (required for go2rtc)
git submodule update --init --recursive

# Build in debug mode (default)
./scripts/build.sh

# Or build in release mode
./scripts/build.sh --release

# Clean build directory before building
./scripts/build.sh --clean
```

### Building with or without SOD

LightNVR can be built with or without SOD (an embedded computer vision & machine learning library) support. By default, SOD is enabled. Use CMake options to control this:

```bash
# Build with SOD support (default)
./scripts/build.sh --release

# Build without SOD support
cd build/Release
cmake -DCMAKE_BUILD_TYPE=Release -DENABLE_SOD=OFF ../..
cmake --build . -- -j$(nproc)
```

When built without SOD, LightNVR will still function normally but will not have embedded object detection capabilities. You can still use external API-based detection via the `[api_detection]` configuration section.

For more information about SOD integration, see [SOD Integration](SOD_INTEGRATION.md).

### Voice enhancement (libavfilter)

The per-stream **Voice Enhancement** option filters recorded audio while it is
transcoded from PCM (G.711 μ-law/A-law, raw PCM) to AAC: two cascaded
`highpass=f=100` sections for hum and rumble, `lowpass=f=8000` when the source
rate carries content above 8 kHz, and the `afftdn` adaptive denoiser. It needs
`libavfilter` (`libavfilter-dev`):

- `ENABLE_AUDIO_FILTERS` (default `ON`) links libavfilter when pkg-config finds
  it. When it is missing or disabled, the build still succeeds and turning the
  option on logs that it is unavailable; audio records unfiltered.
- `ENABLE_AUDIO_DENOISE` (default `ON`, `OFF` in the embedded profile) includes
  `afftdn`, the CPU-heavy stage. With it off, only the highpass/lowpass run.

Audio that the camera already sends in an MP4-compatible codec (AAC, MP3,
Opus, AC-3) is passed through to the recording without re-encoding, and is
**not** filtered. Live audio is never filtered. If the filter graph cannot be
built, LightNVR logs a warning once and keeps recording unfiltered audio.

### HTTPS support

`ENABLE_SSL` (default `OFF`) builds TLS termination into the web server so it
can serve HTTPS without a reverse proxy. It uses mbedTLS, which is already a
required dependency, so no extra packages are needed. From the repository root,
configure and rebuild the release directory used by `scripts/build.sh`:

```bash
cmake -S . -B build/Release -DCMAKE_BUILD_TYPE=Release -DENABLE_SSL=ON
cmake --build build/Release --parallel
```

HTTPS is then turned on at runtime with `ssl_enabled`, `ssl_cert_path` and
`ssl_key_path` in `[web]`; see
[HTTPS without a reverse proxy](REVERSE_PROXY.md#https-without-a-reverse-proxy).
The former `USE_MBEDTLS` and `USE_WOLFSSL` options were removed; they only
linked libraries that nothing used.

### Build options

Pass these to CMake as `-D<OPTION>=ON|OFF`. "Profile" means `ON` in the default `full`
profile and `OFF` in the `embedded` profile.

| Option | Default | What it controls |
|---|---|---|
| `LIGHTNVR_PROFILE` | `full` | `full` or `embedded` (see below) |
| `ENABLE_GO2RTC` | `ON` | go2rtc integration for WebRTC, MSE and HLS live view |
| `GO2RTC_BINARY_PATH` | `/usr/local/bin/go2rtc` | Default go2rtc executable path (`[go2rtc] binary_path` overrides it at runtime) |
| `GO2RTC_CONFIG_DIR` | `/etc/lightnvr/go2rtc` | Default go2rtc configuration directory |
| `ENABLE_SSL` | `OFF` | HTTPS in the built-in web server (mbedTLS) |
| `ENABLE_SOD` | profile | Embedded SOD motion/object detection ([SOD](SOD_INTEGRATION.md)) |
| `SOD_DYNAMIC_LINK` | `OFF` | Link SOD as a shared library instead of statically |
| `ENABLE_LITERT` | profile | In-process LiteRT (TFLite) detection engine (`[detection_engine]` in [Configuration](CONFIGURATION.md#in-process-litert-engine)) |
| `LITERT_WITH_XNNPACK` | `ON` | Compile the XNNPACK CPU delegate into LiteRT |
| `LITERT_WITH_GPU` | `OFF` | Compile the OpenCL/GLES GPU delegate into LiteRT |
| `ENABLE_MQTT` | profile | MQTT event publishing and Home Assistant discovery (needs libmosquitto) |
| `ENABLE_S3_ARCHIVE` | `ON` | S3 archive targets (needs libcurl 7.75+; see [Storage & Archive](STORAGE_ARCHIVE.md)) |
| `ENABLE_AUDIO_FILTERS` | `ON` | libavfilter voice enhancement (below) |
| `ENABLE_AUDIO_DENOISE` | profile | The afftdn denoiser within voice enhancement |
| `ENABLE_SENTRY` | `OFF` | Optional Sentry native crash reporting |
| `BUILD_TESTS` | `OFF` | Build the C unit and integration tests |

### Embedded build profile

Resource-constrained targets can select the embedded profile without changing
the defaults used by server-class installations:

```bash
cmake -S . -B build/embedded \
  -DCMAKE_BUILD_TYPE=Release \
  -DLIGHTNVR_PROFILE=embedded
cmake --build build/embedded -- -j$(nproc)
```

The embedded profile caps compile-time stream storage at 16 streams, optimizes
for size, and defaults SOD, LiteRT, MQTT, and the voice-enhancement denoiser
(`ENABLE_AUDIO_DENOISE`) off. The individual feature switches remain available
when an embedded target needs one of them. Full builds remain the default and
retain the 1024-stream ceiling.

`LIGHTNVR_MAX_STREAMS` can override the full-profile ceiling. Embedded builds
accept values from 1 through 16 only. The older `EMBEDDED_A1_DEVICE=ON` option
is retained as a deprecated alias for `LIGHTNVR_PROFILE=embedded`.

Firmware packagers can also reduce web assets independently:

```bash
cd web
LIGHTNVR_WEB_LEGACY=false LIGHTNVR_WEB_LOCALES=en npm run build
```

The first variable omits legacy-browser bundles. The second accepts a
comma-separated locale list; English is always retained as the fallback.

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
1. `./lightnvr.ini` (INI format in current directory)
2. `/etc/lightnvr/lightnvr.ini` (INI format in system directory)
3. `./lightnvr.conf` (legacy format in current directory)
4. `/etc/lightnvr/lightnvr.conf` (legacy format in system directory)

You can specify a different configuration file using the `-c` option:

```bash
./lightnvr -c /path/to/config.ini
```

## Building Web Assets

LightNVR includes a web interface built with Vite. By default, the web assets are built **without source maps** to reduce file size, which is especially important for embedded devices.

### Building Web Assets Without Source Maps (Default)

```bash
# Build web assets (no source maps)
./scripts/build_web_vite.sh
```

### Building Web Assets With Source Maps

If you need source maps for debugging, use the `-m` flag:

```bash
# Build web assets with source maps
./scripts/build_web_vite.sh -m
```

**Note**: Source maps can significantly increase the build size (often 2-3x larger). Only use the `-m` flag when you need to debug the web interface.

### Installing Web Assets

To install the web assets to the web root directory:

```bash
# Install without source maps (default)
sudo ./scripts/install_web_assets.sh

# Install with source maps (for debugging)
sudo ./scripts/install_web_assets.sh -m
```

The web assets will be installed to `/var/lib/lightnvr/www` by default, or to the path specified in `/etc/lightnvr/lightnvr.ini`.

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

### llhttp source fallback

When `libllhttp` is unavailable through pkg-config, CMake downloads generated
llhttp sources. The default `LLHTTP_DOWNLOAD_REF=release/v9.4.3` is a tag whose
files are checked against pinned SHA-256 hashes.

For alternate sources, set both `LLHTTP_DOWNLOAD_REF` and
`LLHTTP_DOWNLOAD_REF_TYPE` (`tag`, `branch`, or `commit`). For example:

```bash
cmake -S . -B build -DLLHTTP_DOWNLOAD_REF_TYPE=branch -DLLHTTP_DOWNLOAD_REF=release
```

Alternate refs are cached separately by type and value under the build tree's
`_deps` directory and are not checked against the default tag's hashes. To refresh
a moving branch, remove its `_deps/llhttp-*` cache directory before reconfiguring.
Use a full commit hash when a fixed revision is required. These options have no
effect when CMake finds a system llhttp package.
