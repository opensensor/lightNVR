# LightNVR - Lightweight Network Video Recorder

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

LightNVR is a tiny, memory-optimized Network Video Recorder software written in C. While originally designed for resource-constrained devices like the Ingenic A1 SoC with only 256MB of RAM, it can run on any Linux system.

## Overview

LightNVR provides a lightweight yet powerful solution for recording and managing IP camera streams. It's designed to run efficiently on low-power, memory-constrained devices while still providing essential NVR functionality.

![Live Streams Interface](docs/images/live-streams.png)

### Key Features

- **Cross-Platform**: Runs on any Linux system, from embedded devices to full servers
- **Memory Efficient**: Optimized to run on devices with low memory (SBCs and certain SoCs)
- **Stream Support**: Handle up to 16 video streams (with memory-optimized buffering)
- **Protocol Support**: RTSP and ONVIF (basic profile)
- **Codec Support**: H.264 (primary), H.265 (if resources permit)
- **Object Detection**: Optional SOD integration for motion and object detection (supports both RealNet and CNN models)
- **ONVIF Motion Recording**: Automated recording triggered by ONVIF motion detection events
- **Resolution Support**: Up to 1080p per stream (configurable lower resolutions)
- **Frame Rate Control**: Configurable from 1-15 FPS per stream to reduce resource usage
- **Standard Formats**: Records in standard MP4/MKV containers with proper indexing
- **Modern Web Interface**: Responsive UI built with Tailwind CSS and Preact
- **Storage Management**: Automatic retention policies and disk space management
- **Reliability**: Automatic recovery after power loss or system failure
- **Resource Optimization**: Stream prioritization to manage limited RAM

## System Requirements

- **Processor**: Any Linux-compatible processor (ARM, x86, MIPS, etc.)
- **Memory**: unknown RAM minimum (more recommended for multiple streams)
- **Storage**: Any storage device accessible by the OS
- **Network**: Ethernet or WiFi connection
- **OS**: Linux with kernel 4.4 or newer

## Screenshots

| ![Stream Management](docs/images/stream-management.png) | ![Recording Management](docs/images/recording-management.png) |
|:-------------------------------------------------------:|:------------------------------------------------------------:|
| Stream Management                                       | Recording Management                                          |

| ![Settings Management](docs/images/settings-management.png) | ![System Information](docs/images/System.png) |
|:----------------------------------------------------------:|:--------------------------------------------:|
| Settings Management                                         | System Information                            |

## Quick Start

### Installation

1. **Build from source**:
   ```bash
   # Clone the repository
   git clone https://github.com/opensensor/lightnvr.git
   cd lightnvr

   # Build web assets (requires Node.js/npm)
   cd web
   npm install
   npm run build
   cd ..

   # Build the software
   ./scripts/build.sh --release

   # Install (requires root)
   sudo ./scripts/install.sh
   ```

2. **Configure**:
   ```bash
   # Edit the configuration file
   sudo nano /etc/lightnvr/lightnvr.conf
   ```

3. **Start the service**:
   ```bash
   sudo systemctl start lightnvr
   ```

4. **Verify the service is running**:
   ```bash
   sudo systemctl status lightnvr
   # Check that port 8080 is open
   netstat -tlnp | grep :8080
   ```

5. **Access the web interface**:
   Open a web browser and navigate to `http://your-device-ip:8080`

   Default credentials:
   - Username: `admin`
   - Password: `admin`

## Troubleshooting

### Blank Web Page

If you see a blank page after installation, the web assets may not have been installed:

```bash
# Diagnose the issue
sudo ./scripts/diagnose_web_issue.sh

# Install web assets
sudo ./scripts/install_web_assets.sh

# Restart service
sudo systemctl restart lightnvr
```

See [Web Interface Troubleshooting Guide](docs/TROUBLESHOOTING_WEB_INTERFACE.md) for detailed instructions.

### Daemon Mode Issues

If the systemd service starts but port 8080 is not accessible, see the [Daemon Troubleshooting Guide](docs/DAEMON_TROUBLESHOOTING.md).

Quick diagnosis:
```bash
# Run the diagnostic script
sudo ./scripts/diagnose_daemon.sh

# Test daemon mode functionality
sudo ./scripts/test_daemon_mode.sh

# Validate that fixes are working
sudo ./scripts/validate_daemon_fix.sh
```

### General Troubleshooting

For other issues, see the [General Troubleshooting Guide](docs/TROUBLESHOOTING.md).

### Docker Installation

#### Quick Start with Docker Compose (Recommended)

```bash
# Clone the repository
git clone https://github.com/opensensor/lightNVR.git
cd lightNVR

# Start the container
docker-compose up -d

# View logs
docker-compose logs -f
```

The container will automatically:
- Create default configuration files in `./config`
- Initialize the database in `./data/database`
- Set up web assets with working defaults
- Configure go2rtc with WebRTC/STUN support

Access the web UI at `http://localhost:8080` (default credentials: admin/admin)

#### Using Docker Run

```bash
docker pull ghcr.io/opensensor/lightnvr:latest

docker run -d \
  --name lightnvr \
  --restart unless-stopped \
  -p 8080:8080 \
  -p 8554:8554 \
  -p 8555:8555 \
  -p 8555:8555/udp \
  -p 1984:1984 \
  -v ./config:/etc/lightnvr \
  -v ./data:/var/lib/lightnvr/data \
  -e TZ=America/New_York \
  ghcr.io/opensensor/lightnvr:latest
```

#### Volume Mounts Explained

The container uses two volume mounts for persistence:

- **`/etc/lightnvr`** - Configuration files
  - `lightnvr.ini` - Main configuration
  - `go2rtc/go2rtc.yaml` - go2rtc WebRTC/RTSP configuration

- **`/var/lib/lightnvr/data`** - Persistent data
  - `database/` - SQLite database
  - `recordings/` - Video recordings (HLS and MP4)
  - `models/` - Object detection models

**⚠️ Important:** Do NOT mount `/var/lib/lightnvr` directly as it will overwrite web assets!

#### Exposed Ports

- **8080** - Web UI (HTTP)
- **8554** - RTSP streaming
- **8555** - WebRTC (TCP/UDP)
- **1984** - go2rtc API

#### Environment Variables

- `TZ` - Timezone (default: UTC)
- `GO2RTC_CONFIG_PERSIST` - Persist go2rtc config across restarts (default: true)
- `LIGHTNVR_AUTO_INIT` - Auto-initialize config files (default: true)

#### First Run

On first run, the container will:
1. Create default configuration files in `/etc/lightnvr`
2. Copy web assets to `/var/lib/lightnvr/web`
3. Initialize the database in `/var/lib/lightnvr/data/database`
4. Set up go2rtc with WebRTC/STUN configuration

The go2rtc configuration includes STUN servers for WebRTC NAT traversal, so WebRTC streaming should work out-of-the-box in most network environments.

#### Customizing Configuration

After first run, you can customize the configuration:

```bash
# Edit main configuration
nano ./config/lightnvr.ini

# Edit go2rtc configuration (WebRTC, RTSP settings)
nano ./config/go2rtc/go2rtc.yaml

# Restart to apply changes
docker-compose restart
```

The configuration files will persist across container restarts and updates.

## Documentation

- [Installation Guide](docs/INSTALLATION.md)
- [Build Instructions](docs/BUILD.md)
- [Configuration Guide](docs/CONFIGURATION.md)
- [API Documentation](docs/API.md)
- [Frontend Architecture](docs/FRONTEND.md)
- [Troubleshooting Guide](docs/TROUBLESHOOTING.md)
- [Architecture Overview](docs/ARCHITECTURE.md)
- [SOD Integration](docs/SOD_INTEGRATION.md)
- [SOD Unified Detection](docs/SOD_UNIFIED_DETECTION.md)
- [ONVIF Detection](docs/ONVIF_DETECTION.md)
- [ONVIF Motion Recording](docs/ONVIF_MOTION_RECORDING.md)
- [Motion Buffer System](docs/MOTION_BUFFER.md)
- [Release Process](docs/RELEASE_PROCESS.md) - For maintainers creating releases

## Project Structure

- `src/` - Source code
  - `core/` - Core system components
  - `video/` - Video processing and stream handling
  - `storage/` - Storage management
  - `web/` - Web interface and API handlers
  - `database/` - Database operations
  - `utils/` - Utility functions
- `include/` - Header files
- `scripts/` - Build and utility scripts
- `config/` - Configuration files
- `docs/` - Documentation
- `tests/` - Test suite
- `web/` - Web interface files
  - `css/` - Tailwind CSS stylesheets
  - `js/` - JavaScript and Preact components
  - `*.html` - HTML entry points

## Memory Optimization

LightNVR is specifically designed for memory-constrained environments:

- **Efficient Buffering**: Minimizes memory usage while maintaining reliable recording
- **Stream Prioritization**: Allocates resources based on stream importance
- **Staggered Initialization**: Prevents memory spikes during startup
- **Swap Support**: Optional swap file configuration for additional virtual memory
- **Resource Governors**: Prevents system crashes due to memory exhaustion

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- FFmpeg for video processing capabilities
- SQLite for efficient database storage
- Mongoose for the web server
- cJSON for JSON parsing
- Tailwind CSS for frontend styling
- Preact for frontend components
- All contributors who have helped with the project
