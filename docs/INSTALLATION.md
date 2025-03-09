# LightNVR Installation Guide

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
4. [Post-Installation Setup](#post-installation-setup)
5. [Upgrading](#upgrading)
6. [Uninstallation](#uninstallation)

## Prerequisites

Before installing LightNVR, ensure your system meets the following requirements:

- **Processor**: Any Linux-compatible processor (ARM, x86, MIPS, etc.)
- **Memory**: unknown what the minimum is
- **Storage**: Any storage device accessible by the OS
- **Network**: Ethernet or WiFi connection
- **OS**: Linux with kernel 4.4 or newer

## Installation Methods

### Building from Source

Building from source is the recommended method for most installations, as it ensures compatibility with your specific system.

#### 1. Clone the Repository

```bash
git clone https://github.com/yourusername/lightnvr.git
cd lightnvr
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

#### 1. Pull the Docker Image

```bash
docker pull lightnvr/lightnvr:latest
```

#### 2. Create Directories for Persistent Storage

```bash
mkdir -p /path/to/config
mkdir -p /path/to/recordings
```

#### 3. Run the Container

```bash
docker run -d \
  --name lightnvr \
  --restart unless-stopped \
  -p 8080:8080 \
  -v /path/to/config:/etc/lightnvr \
  -v /path/to/recordings:/var/lib/lightnvr/recordings \
  lightnvr/lightnvr:latest
```

#### 4. Create a Configuration File

```bash
# Copy the default configuration
docker cp lightnvr:/etc/lightnvr/lightnvr.conf.default /path/to/config/lightnvr.conf

# Edit the configuration
nano /path/to/config/lightnvr.conf
```

#### 5. Restart the Container

```bash
docker restart lightnvr
```

### Pre-built Packages

Pre-built packages are available for some distributions.

#### Debian/Ubuntu

```bash
# Add the repository
echo "deb https://packages.lightnvr.org/debian stable main" | sudo tee /etc/apt/sources.list.d/lightnvr.list
wget -qO - https://packages.lightnvr.org/debian/pubkey.gpg | sudo apt-key add -

# Update package lists
sudo apt-get update

# Install LightNVR
sudo apt-get install lightnvr
```

#### Fedora/RHEL/CentOS

```bash
# Add the repository
sudo dnf config-manager --add-repo https://packages.lightnvr.org/fedora/lightnvr.repo

# Install LightNVR
sudo dnf install lightnvr
```

#### Arch Linux

LightNVR is available in the AUR:

```bash
# Using yay
yay -S lightnvr

# Or using manual AUR installation
git clone https://aur.archlinux.org/lightnvr.git
cd lightnvr
makepkg -si
```

## Platform-Specific Instructions

### Debian/Ubuntu

#### Install Dependencies

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

#### Install Dependencies

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

#### Install Dependencies

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
git clone https://github.com/yourusername/lightnvr.git
cd lightnvr

# Build using the cross-compilation script
./scripts/build-ingenic.sh
```

#### 4. Deploy to Ingenic A1 Device

```bash
# Copy the binary and configuration files to the device
scp build/Ingenic/bin/lightnvr root@ingenic-device:/usr/local/bin/
scp config/lightnvr.conf.default root@ingenic-device:/etc/lightnvr/lightnvr.conf

# Create necessary directories on the device
ssh root@ingenic-device "mkdir -p /var/lib/lightnvr/recordings /var/lib/lightnvr/www /var/log/lightnvr"

# Copy web interface files
scp -r web/* root@ingenic-device:/var/lib/lightnvr/www/
```

### Raspberry Pi

Raspberry Pi installation is similar to Debian/Ubuntu, with some optimizations.

#### 1. Install Dependencies

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

#### 2. Build and Install

```bash
# Clone the repository
git clone https://github.com/yourusername/lightnvr.git
cd lightnvr

# Build with Raspberry Pi optimizations
./scripts/build.sh --release --platform=raspberry-pi

# Install
sudo ./scripts/install.sh
```

## Post-Installation Setup

After installing LightNVR, follow these steps to complete the setup:

### 1. Configure LightNVR

Edit the configuration file:

```bash
sudo nano /etc/lightnvr/lightnvr.conf
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

```bash
# Pull the latest image
docker pull lightnvr/lightnvr:latest

# Stop and remove the container
docker stop lightnvr
docker rm lightnvr

# Run a new container with the latest image
docker run -d \
  --name lightnvr \
  --restart unless-stopped \
  -p 8080:8080 \
  -v /path/to/config:/etc/lightnvr \
  -v /path/to/recordings:/var/lib/lightnvr/recordings \
  lightnvr/lightnvr:latest
```

### Upgrading Package Installation

#### Debian/Ubuntu

```bash
sudo apt-get update
sudo apt-get upgrade lightnvr
```

#### Fedora/RHEL/CentOS

```bash
sudo dnf upgrade lightnvr
```

#### Arch Linux

```bash
# Using yay
yay -Syu lightnvr

# Or using pacman if installed from AUR
sudo pacman -Syu
```

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

```bash
# Stop and remove the container
docker stop lightnvr
docker rm lightnvr

# Remove the image
docker rmi lightnvr/lightnvr:latest

# Remove volumes (optional)
rm -rf /path/to/config
rm -rf /path/to/recordings
```

### Uninstalling Package Installation

#### Debian/Ubuntu

```bash
sudo apt-get remove --purge lightnvr
```

#### Fedora/RHEL/CentOS

```bash
sudo dnf remove lightnvr
```

#### Arch Linux

```bash
sudo pacman -Rs lightnvr
