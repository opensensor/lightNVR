# LightNVR Installation Guide

*Verified against LightNVR 0.37.x.*

This document provides detailed instructions for installing LightNVR on various platforms.

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Installation Methods](#installation-methods)
   - [Building from Source](#building-from-source)
   - [Docker Installation](#docker-installation)
   - [Pre-built Packages](#pre-built-packages)
3. [Platform-Specific Instructions](#platform-specific-instructions)
   - [Debian/Ubuntu](#debianubuntu)
   - [Fedora/RHEL/CentOS](#fedorarhel-centos)
   - [Arch Linux](#arch-linux)
   - [Ingenic A1](#ingenic-a1)
   - [Raspberry Pi](#raspberry-pi)
   - [NAS (Synology, QNAP, unRAID, TrueNAS)](#nas-synology-qnap-unraid-truenas)
   - [Home Assistant](#home-assistant)
   - [Windows](#windows)
4. [Post-Installation Setup](#post-installation-setup)
5. [Upgrading](#upgrading)
6. [Uninstallation](#uninstallation)

## Prerequisites

Before installing LightNVR, ensure your system meets the following requirements:

- **Processor**: Any Linux-compatible processor (ARM, x86, MIPS, etc.)
- **Memory**: 256 MB minimum; 1 GB or more if you enable object detection — see below
- **Storage**: Any storage device accessible by the OS, sized for your retention policy
- **Network**: Ethernet or WiFi connection
- **OS**: Linux with kernel 4.4 or newer

#### Memory in practice

LightNVR is built for memory-constrained hardware, so the floor is low, but "minimum" and
"enough for your setup" are different numbers.

| Deployment | Memory |
|---|---|
| Container, started, no streams configured | ~75 MB measured (LightNVR ~33 MB, go2rtc ~16 MB) |
| A few streams, recording only, no detection | 256 MB is workable |
| Object detection enabled | 1 GB or more — models are held in memory |
| Many streams, or detection across several streams | 2 GB and up |

The idle figure is measured from the published `amd64` image on first start with no
cameras. Everything above it scales with what you actually run: each stream costs buffers
(`[memory] buffer_size`, 1024 KB per stream by default) plus whatever the decoder needs for
that resolution, and object detection adds the model's working set on top.

If you are sizing a box rather than checking a floor: recording alone is cheap and mostly
I/O-bound, while detection is what will decide your RAM. A swap file is supported and
helps small devices survive spikes, but do not plan to run detection out of swap.

## Installation Methods

### Building from Source

Building from source is the recommended method for most installations, as it ensures compatibility with your specific system.

#### 1. Clone the Repository

```bash
git clone https://github.com/opensensor/lightnvr.git
cd lightnvr

# Initialize submodules (required for go2rtc)
git submodule update --init --recursive
```

#### 2. Install Dependencies

See the [Platform-Specific Instructions](#platform-specific-instructions) section for dependency installation commands for your distribution.

#### 3. Build the Software

```bash
# Build in debug mode (default)
./scripts/build.sh

# Or build in release mode (recommended for production)
./scripts/build.sh --release
```

#### 4. Install the Software

```bash
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

### Docker Installation

Docker provides an easy way to run LightNVR without installing dependencies directly on your system.

#### Option 1: Using Docker Compose (Recommended)

Docker Compose simplifies the deployment and ensures proper volume configuration.

```bash
# Clone the repository
git clone https://github.com/opensensor/lightNVR.git
cd lightNVR

# Initialize submodules (required for go2rtc build)
git submodule update --init --recursive

# Start the container (first run will build the image)
docker compose up -d
```

The default `docker-compose.yml` creates two volumes:
- `./config` - Configuration files (mounted to `/etc/lightnvr`)
- `./data` - Persistent data including database, recordings, and models (mounted to `/var/lib/lightnvr/data`)

To customize the configuration:

```bash
# Edit the configuration file
nano config/lightnvr.ini

# Restart the container to apply changes
docker compose restart
```

#### Option 2: Using Docker Run

##### 1. Pull the Docker Image

```bash
docker pull ghcr.io/opensensor/lightnvr:latest
```

##### 2. Create Directories for Persistent Storage

```bash
mkdir -p /path/to/config
mkdir -p /path/to/data
```

**Important:** The data directory must be persisted to avoid losing the database and recordings on container restart.

##### 3. Run the Container

```bash
docker run -d \
  --name lightnvr \
  --restart unless-stopped \
  -p 8080:8080 \
  -p 1984:1984 \
  -v /path/to/config:/etc/lightnvr \
  -v /path/to/data:/var/lib/lightnvr/data \
  ghcr.io/opensensor/lightnvr:latest
```

##### 4. Create a Configuration File

```bash
# Copy the default configuration
docker cp lightnvr:/etc/lightnvr/lightnvr.ini /path/to/config/lightnvr.ini

# Edit the configuration
nano /path/to/config/lightnvr.ini
```

**Note:** Ensure the paths in `lightnvr.ini` point to `/var/lib/lightnvr/data` subdirectories:
- Database: `/var/lib/lightnvr/data/database/lightnvr.db`
- Recordings: `/var/lib/lightnvr/data/recordings`
- MP4 recordings: `/var/lib/lightnvr/data/recordings/mp4`
- Models: `/var/lib/lightnvr/data/models`

##### 5. Restart the Container

```bash
docker restart lightnvr
```

### Pre-built Packages

Pre-built packages are available from GitHub Releases.

#### Downloading from GitHub Releases

Visit the [LightNVR Releases page](https://github.com/opensensor/lightNVR/releases) and
download the package matching **both** your architecture and your Debian suite. Assets are
named `lightnvr_<version>_<suite>_<arch>.deb`, for example:

```
lightnvr_0.37.2_trixie_arm64.deb    # Debian 13 / Raspberry Pi OS (trixie), 64-bit
lightnvr_0.37.2_sid_amd64.deb       # Debian unstable, x86-64
lightnvr_0.37.2_trixie_armhf.deb    # 32-bit ARM
```

The suite matters: the packages link against that release's system libraries, so a `sid`
package on a `trixie` system will fail to satisfy its dependencies.

#### Debian/Ubuntu

```bash
# Substitute the version, suite and architecture you need
wget https://github.com/opensensor/lightNVR/releases/download/0.37.2/lightnvr_0.37.2_trixie_arm64.deb

# Install the package
sudo dpkg -i lightnvr_0.37.2_trixie_arm64.deb

# Install any missing dependencies
sudo apt-get install -f
```

#### Other Distributions

Only `.deb` packages are published. On non-Debian systems, use
[the container](DOCKER.md) or [build from source](#building-from-source).

## Platform-Specific Instructions

### Debian/Ubuntu

#### Install Dependencies

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
    libuv1-dev \
    libcurl4-openssl-dev \
    libmbedtls-dev \
    curl \
    wget
```

**Note**: `libuv1-dev` is required for the HTTP server. `libmbedtls-dev` is **required** for ONVIF support and authentication system (cryptographic functions). `llhttp` is downloaded and built automatically by CMake if not found.

### Fedora/RHEL/CentOS

#### Install Dependencies

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
    libuv-devel \
    libcurl-devel \
    mbedtls-devel \
    curl \
    wget
```

**Note**: `libuv-devel` is required for the HTTP server. `mbedtls-devel` is **required** for ONVIF support and authentication system (cryptographic functions).

### Arch Linux

#### Install Dependencies

```bash
sudo pacman -S \
    base-devel \
    cmake \
    git \
    sqlite \
    ffmpeg \
    libuv \
    curl \
    wget \
    mbedtls
```

**Note**: `libuv` is required for the HTTP server. `mbedtls` is **required** for ONVIF support and authentication system (cryptographic functions).

### Ingenic A1

The Ingenic A1 SoC requires cross-compilation. A detailed guide is provided below.

#### 1. Set Up Cross-Compilation Toolchain

```bash
# Download and extract the toolchain
wget https://github.com/Ingenic-community/mips-linux-toolchain/releases/download/latest/mips-linux-uclibc-toolchain.tar.gz
sudo mkdir -p /opt/mips-linux-toolchain
sudo tar -xzf mips-linux-uclibc-toolchain.tar.gz -C /opt/mips-linux-toolchain
```

#### 2. Install Dependencies for Cross-Compilation

```bash
# Install build tools
sudo apt-get update
sudo apt-get install -y build-essential cmake pkg-config

# Clone and build cross-compiled dependencies
git clone https://github.com/lightnvr/ingenic-dependencies.git
cd ingenic-dependencies
./build-all.sh
```

#### 3. Build LightNVR for Ingenic A1

```bash
# Clone the repository
git clone https://github.com/opensensor/lightnvr.git
cd lightnvr

# Initialize submodules (required for go2rtc)
git submodule update --init --recursive

# Cross-compile by pointing the build at a CMake toolchain file
CMAKE_TOOLCHAIN_FILE=/path/to/your/mips-ingenic.cmake ./scripts/build.sh --release
```

`scripts/build.sh` passes `CMAKE_TOOLCHAIN_FILE` straight through to CMake, which is how
all cross-compilation is driven. The repository ships
`cmake/toolchains/armv7-linux-gnueabihf.cmake` as a working example; **there is no MIPS
toolchain file in-repo**, so for Ingenic you supply one pointing at the toolchain you
extracted above.

The binary lands at `build/Release/bin/lightnvr`.

#### 4. Deploy to Ingenic A1 Device

```bash
# Copy the binary and configuration files to the device
scp build/Release/bin/lightnvr root@ingenic-device:/usr/local/bin/
scp config/lightnvr.ini root@ingenic-device:/etc/lightnvr/lightnvr.ini

# Create necessary directories on the device
ssh root@ingenic-device "mkdir -p /var/lib/lightnvr/data/recordings /var/lib/lightnvr/www /var/log/lightnvr"

# Copy the *built* web interface (web/ holds sources; web/dist/ is what gets served)
cd web && npm ci && npm run build && cd ..
scp -r web/dist/* root@ingenic-device:/var/lib/lightnvr/www/
```

### Raspberry Pi

Raspberry Pi is a Debian system, so everything in the [Debian/Ubuntu](#debianubuntu)
section applies. In order of least to most effort:

1. **Container** — `ghcr.io/opensensor/lightnvr` publishes `arm64` and `arm/v7`. See
   [DOCKER.md](DOCKER.md). This is the path most Pi users want.
2. **`.deb` package** — prebuilt for `arm64` and `armhf` on the
   [Releases](https://github.com/opensensor/lightNVR/releases) page, for both Debian
   trixie (stable) and sid. Match the package to your OS release, not just the
   architecture.
3. **From source** — below. Worth it if you are developing, or on an OS release we do not
   package for.

#### Before you start: do not record to the SD card

This is the mistake that kills Pi NVR builds. Continuous video writing will wear out a
microSD card, and when it fails it usually takes the SQLite database with it. Put
recordings — and ideally the whole data directory — on a USB SSD or HDD:

```ini
[storage]
path = /mnt/ssd/lightnvr/recordings

[database]
path = /mnt/ssd/lightnvr/database/lightnvr.db
```

Make sure the drive is mounted at boot (an `/etc/fstab` entry, not a desktop automount)
before pointing LightNVR at it, or the service will start against an empty mount point.

#### Which Pi?

A Pi 4 or 5 with 2 GB or more handles several streams comfortably. A Pi Zero or a
first-generation model will struggle with anything beyond one low-resolution stream. Object
detection wants 1 GB of headroom on top of whatever your streams use — see
[Memory in practice](#memory-in-practice).

Prefer a 64-bit OS. `arm/v7` builds exist, but 32-bit ARM is the least-tested target and
Home Assistant has already dropped it.

#### Building from source

##### Install dependencies

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
    libuv1-dev \
    libcurl4-openssl-dev \
    libmbedtls-dev \
    curl \
    wget
```

**Note**: `libuv1-dev` is required for the HTTP server. `libmbedtls-dev` is **required** for ONVIF support and authentication system (cryptographic functions).

##### Build and install

```bash
# Clone the repository
git clone https://github.com/opensensor/lightnvr.git
cd lightnvr

# Initialize submodules (required for go2rtc)
git submodule update --init --recursive

# Build (ARM architecture is detected automatically)
./scripts/build.sh --release

# Install
sudo ./scripts/install.sh
```

### NAS (Synology, QNAP, unRAID, TrueNAS)

A NAS is a natural home for an NVR — it already has the disks. All of these run Docker
under their own branding (Synology **Container Manager**, QNAP **Container Station**,
unRAID **Docker** tab, TrueNAS SCALE **Apps**), so [DOCKER.md](DOCKER.md) is the reference
and their own documentation covers the click-path for adding a container.

Four things are LightNVR-specific and are what people actually get wrong:

**Do not put the database on a network share.** Recordings on a share are fine; the SQLite
database is not. NFS and SMB do not implement the file locking SQLite expects, and the
result is corruption rather than an error message. Keep `[database] path` on local storage
— the same volume the container runs from — and point only `[storage] path` at the big
array. This applies to the NAS's own shares mounted back into the container, not just
remote ones.

**Bridge networking breaks ONVIF discovery.** WS-Discovery is multicast and will not cross
the default bridge. Either set `LIGHTNVR_ONVIF_NETWORK` to your camera subnet in CIDR form
(the unicast sweep works through NAT), or give the container host/macvlan networking, which
most NAS UIs expose. Host networking also makes WebRTC live view work without extra port
mapping.

**Check the UID the container runs as.** These platforms often run containers as a
non-root user of their choosing. The data directory must be writable by whatever that is,
or LightNVR will start and then fail to record.

**Mind the spin-down.** If the array parks its disks, continuous recording will keep waking
them, and event-triggered recording will lose the first seconds to spin-up latency. Either
disable spin-down for the recording volume or accept the trade.

Ports and volumes are identical to any other Docker deployment: map `/etc/lightnvr` and
`/var/lib/lightnvr/data`, never `/var/lib/lightnvr` itself.

### Home Assistant

On Home Assistant OS or Supervised, install LightNVR from the Add-on Store by adding the
repository `https://github.com/opensensor/lightnvr-hassio-addons`.

See **[LightNVR on Home Assistant](HOME_ASSISTANT.md)** for the walkthrough and for the
ways the add-on differs from a plain Docker deployment — it serves the web UI on port 7800,
uses host networking, and keeps recordings out of your Home Assistant backups.

Home Assistant Container and Core installs have no add-on store; use [DOCKER.md](DOCKER.md)
there.

### Windows

LightNVR is Linux software; there is no native Windows build and none is planned. On
Windows you run the published Linux container.

See **[Running LightNVR on Windows with Podman + WSL2](WINDOWS_PODMAN.md)** for the full
walkthrough, including the Windows-specific parts that are easy to get wrong: reaching the
web UI from other devices on your LAN, why the database must not live on a `C:\` path, and
what it takes to keep a recorder running through sleep and reboots.

Docker Desktop works too, and [DOCKER.md](DOCKER.md) applies unchanged if you prefer it.

## Post-Installation Setup

After installing LightNVR, follow these steps to complete the setup:

### 1. Configure LightNVR

Edit the configuration file:

```bash
sudo nano /etc/lightnvr/lightnvr.ini
```

At minimum, you should:
- Set a secure password for the web interface
- Configure storage paths
- Set up your camera streams

See [CONFIGURATION.md](CONFIGURATION.md) for detailed configuration options.

### 2. Start the Service

```bash
# Start the service
sudo systemctl start lightnvr

# Enable the service to start at boot
sudo systemctl enable lightnvr
```

### 3. Check the Status

```bash
sudo systemctl status lightnvr
```

### 4. Access the Web Interface

Open a web browser and navigate to:

```
http://your-device-ip:8080
```

Log in with the username and password configured in the configuration file.

## Upgrading

### Upgrading from Source

```bash
# Navigate to the repository
cd lightnvr

# Pull the latest changes
git pull

# Rebuild
./scripts/build.sh --release

# Stop the service
sudo systemctl stop lightnvr

# Install the new version
sudo ./scripts/install.sh

# Start the service
sudo systemctl start lightnvr
```

### Upgrading Docker Installation

#### Using Docker Compose

```bash
# Navigate to the repository
cd lightNVR

# Pull the latest changes
git pull

# Rebuild and restart
docker compose down
docker compose build
docker compose up -d
```

#### Using Docker Run

```bash
# Pull the latest image
docker pull ghcr.io/opensensor/lightnvr:latest

# Stop and remove the container
docker stop lightnvr
docker rm lightnvr

# Run a new container with the latest image
docker run -d \
  --name lightnvr \
  --restart unless-stopped \
  -p 8080:8080 \
  -p 1984:1984 \
  -v /path/to/config:/etc/lightnvr \
  -v /path/to/data:/var/lib/lightnvr/data \
  ghcr.io/opensensor/lightnvr:latest
```

**Note:** Your data is preserved in the volumes, so upgrading will not affect your database or recordings.

## Uninstallation

### Uninstalling Source Installation

```bash
# Stop the service
sudo systemctl stop lightnvr
sudo systemctl disable lightnvr

# Remove the service file
sudo rm /etc/systemd/system/lightnvr.service
sudo systemctl daemon-reload

# Remove the binary
sudo rm /usr/local/bin/lightnvr

# Remove configuration and data (optional)
sudo rm -rf /etc/lightnvr
sudo rm -rf /var/lib/lightnvr
sudo rm -rf /var/log/lightnvr
```

### Uninstalling Docker Installation

#### Using Docker Compose

```bash
# Navigate to the repository
cd lightNVR

# Stop and remove the container
docker compose down

# Remove the image
docker rmi lightnvr

# Remove volumes (optional - this will delete all data)
rm -rf ./config
rm -rf ./data
```

#### Using Docker Run

```bash
# Stop and remove the container
docker stop lightnvr
docker rm lightnvr

# Remove the image
docker rmi ghcr.io/opensensor/lightnvr:latest

# Remove volumes (optional - this will delete all data)
rm -rf /path/to/config
rm -rf /path/to/data
```
