# Docker Deployment Guide

This guide provides comprehensive information about deploying LightNVR using Docker.

## Table of Contents

- [Quick Start](#quick-start)
- [Container Architecture](#container-architecture)
- [Volume Management](#volume-management)
- [Network Configuration](#network-configuration)
- [Environment Variables](#environment-variables)
- [First Run Experience](#first-run-experience)
- [WebRTC Configuration](#webrtc-configuration)
- [Troubleshooting](#troubleshooting)
- [Advanced Configuration](#advanced-configuration)

## Quick Start

### Using Docker Compose (Recommended)

```bash
# Clone the repository
git clone https://github.com/opensensor/lightNVR.git
cd lightNVR

# Start the container
docker-compose up -d

# View logs
docker-compose logs -f

# Access the web UI
# http://localhost:8080
# Default credentials: admin/admin
```

### Using Docker Run

```bash
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

## Container Architecture

The LightNVR Docker container is built using a multi-stage build process:

1. **Builder Stage** - Compiles LightNVR and go2rtc from source
2. **Runtime Stage** - Minimal Debian-based image with only runtime dependencies

### Key Features

- **Automatic Initialization** - Creates default configs on first run
- **Persistent Configuration** - Config files survive container restarts
- **Protected Web Assets** - Web UI files stored in template location
- **Health Checks** - Built-in health monitoring
- **Multi-Architecture** - Supports amd64, arm64, and armv7

## Volume Management

### Volume Structure

The container uses two primary volume mounts:

```
/etc/lightnvr/              # Configuration files
├── lightnvr.ini            # Main configuration
└── go2rtc/
    └── go2rtc.yaml         # go2rtc configuration

/var/lib/lightnvr/data/     # Persistent data
├── database/
│   └── lightnvr.db         # SQLite database
├── recordings/
│   ├── hls/                # HLS recordings
│   └── mp4/                # MP4 recordings
└── models/                 # Object detection models
```

### Important Volume Notes

⚠️ **DO NOT mount `/var/lib/lightnvr` directly!**

Mounting the entire `/var/lib/lightnvr` directory will overwrite the web assets and break the web UI. Always mount only the subdirectories you need:

✅ **Correct:**
```yaml
volumes:
  - ./config:/etc/lightnvr
  - ./data:/var/lib/lightnvr/data
```

❌ **Incorrect:**
```yaml
volumes:
  - ./config:/etc/lightnvr
  - ./data:/var/lib/lightnvr  # This will break the web UI!
```

### NFS Volume Considerations

When using NFS volumes (common with NAS devices like Synology), there are some important considerations:

#### Permission Issues

NFS volumes may have different permission models than local filesystems. If you encounter "Operation not permitted" errors:

1. **Check NFS mount options**: Ensure your NFS share is mounted with appropriate permissions
   ```yaml
   volumes:
     config:
       driver_opts:
         type: "nfs"
         o: "nfsvers=4,addr=192.168.1.100,rw,nolock"  # Add nolock if needed
         device: ":/volume1/docker/lightnvr/config"
   ```

2. **UID/GID mapping**: The container runs as root by default. Ensure your NFS export allows root access or map UIDs appropriately:
   - On Synology NAS: Enable "Map all users to admin" or "Squash" options in NFS permissions
   - Or use `all_squash,anonuid=1000,anongid=1000` in NFS export options

3. **Directory pre-creation**: For better reliability with NFS, pre-create the directory structure on your NAS:
   ```bash
   # On your NAS or NFS server
   mkdir -p /volume1/docker/lightnvr/config/go2rtc
   mkdir -p /volume1/docker/lightnvr/data/recordings/mp4
   mkdir -p /volume1/docker/lightnvr/data/database
   mkdir -p /volume1/docker/lightnvr/data/models
   chmod -R 755 /volume1/docker/lightnvr
   ```

#### Recording Issues on NFS

If recordings are not being created:

1. **Check write permissions**: The container logs will show write permission test results on startup
2. **Verify NFS mount**: Ensure the NFS volume is actually mounted inside the container:
   ```bash
   docker exec lightnvr-latest ls -la /var/lib/lightnvr/data/recordings/mp4
   ```
3. **Check available space**: Ensure your NAS has sufficient free space
4. **Review logs**: Check for "Failed to open input" or "Operation not permitted" errors:
   ```bash
   docker logs lightnvr-latest | grep -i error
   ```

#### Example NFS Configuration

Here's a complete example for Synology NAS with Traefik:

```yaml
volumes:
  config:
    driver_opts:
      type: "nfs"
      o: "nfsvers=4,addr=${IpAddressNFS},rw,nolock"
      device: ":/${NFSVolumePath}/${SystemId}/lightnvr-config"

  data:
    driver_opts:
      type: "nfs"
      o: "nfsvers=4,addr=${IpAddressNFS},rw,nolock"
      device: ":/${NFSVolumePath}/${SystemId}/lightnvr-data"
```

**Note**: The `nolock` option can help with some NFS permission issues but may reduce file locking safety. Use with caution in production environments.

### Web Assets

Web assets are stored in `/var/lib/lightnvr/web` and are automatically copied from `/usr/share/lightnvr/web-template/` on first run. This ensures:

- Web UI works immediately after container start
- Updates to the container image update the web UI
- Web assets are not lost when mounting data volumes

## Network Configuration

### Port Mapping

| Port | Protocol | Service | Description |
|------|----------|---------|-------------|
| 8080 | TCP | Web UI | Main web interface |
| 8554 | TCP | RTSP | RTSP streaming server |
| 8555 | TCP/UDP | WebRTC | WebRTC streaming |
| 1984 | TCP | go2rtc API | go2rtc REST API |

### Network Modes

#### Bridge Mode (Default)

```yaml
services:
  lightnvr:
    ports:
      - "8080:8080"
      - "8554:8554"
      - "8555:8555"
      - "8555:8555/udp"
      - "1984:1984"
```

#### Host Mode (For Better Performance)

```yaml
services:
  lightnvr:
    network_mode: host
```

**Note:** Host mode provides better performance but exposes all ports directly on the host.

## Environment Variables

### Available Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `TZ` | `UTC` | Container timezone |
| `GO2RTC_CONFIG_PERSIST` | `true` | Persist go2rtc config across restarts |
| `LIGHTNVR_AUTO_INIT` | `true` | Auto-initialize config files on first run |
| `LIGHTNVR_WEB_ROOT` | `/var/lib/lightnvr/web` | Web assets directory |
| `LIGHTNVR_ONVIF_NETWORK` | (none) | Override ONVIF discovery network (e.g., `192.168.1.0/24`) |

### Example Usage

```yaml
environment:
  - TZ=America/New_York
  - GO2RTC_CONFIG_PERSIST=true
  - LIGHTNVR_AUTO_INIT=true
  - LIGHTNVR_ONVIF_NETWORK=192.168.1.0/24
```

### ONVIF Discovery in Containers

When running in a container, ONVIF auto-detection skips Docker bridge interfaces by default. To enable ONVIF camera discovery in containerized deployments, set the `LIGHTNVR_ONVIF_NETWORK` environment variable to specify which network to scan:

```yaml
services:
  lightnvr:
    environment:
      # Specify the network where your cameras are located
      - LIGHTNVR_ONVIF_NETWORK=192.168.1.0/24
```

**Network Priority:**
1. Explicit network parameter (API calls)
2. `LIGHTNVR_ONVIF_NETWORK` environment variable
3. `discovery_network` in config file (`[onvif]` section)
4. Auto-detection (skips Docker interfaces)

**Finding Your Network:**
```bash
# On the Docker host, find your camera network
ip addr show

# Example: If your host IP is 192.168.1.100 with netmask 255.255.255.0
# Use: LIGHTNVR_ONVIF_NETWORK=192.168.1.0/24
```

## First Run Experience

On first container start, the entrypoint script automatically:

1. **Creates Directory Structure**
   ```
   /etc/lightnvr/
   /var/lib/lightnvr/web/
   /var/lib/lightnvr/data/database/
   /var/lib/lightnvr/data/recordings/
   /var/lib/lightnvr/data/models/
   ```

2. **Copies Web Assets**
   - Copies from `/usr/share/lightnvr/web-template/` to `/var/lib/lightnvr/web/`
   - Only if web directory is empty

3. **Creates Default Configuration**
   - `lightnvr.ini` with sensible defaults
   - `go2rtc.yaml` with WebRTC/STUN configuration

4. **Initializes Database**
   - Creates SQLite database on first access
   - Sets up default admin user

### Default Credentials

- **Username:** `admin`
- **Password:** `admin`

⚠️ **Change these immediately after first login!**

## WebRTC Configuration

The container includes pre-configured WebRTC support with STUN servers for NAT traversal.

### Default go2rtc Configuration

```yaml
webrtc:
  listen: :8555
  ice_servers:
    - urls: [stun:stun.l.google.com:19302]
  candidates:
    - "*:8555"
    - stun:stun.l.google.com:19302
```

### Customizing WebRTC

Edit `./config/go2rtc/go2rtc.yaml`:

```yaml
webrtc:
  listen: :8555
  ice_servers:
    - urls: [stun:stun.l.google.com:19302]
    - urls: [turn:your-turn-server.com:3478]
      username: your-username
      credential: your-password
  candidates:
    - "YOUR_PUBLIC_IP:8555"
```

Restart the container to apply changes:
```bash
docker-compose restart
```

### Port Forwarding for WebRTC

If running behind NAT/firewall, forward these ports:

- **8555/TCP** - WebRTC signaling
- **8555/UDP** - WebRTC media

## Troubleshooting

### Web UI Not Loading

**Symptom:** Accessing `http://localhost:8080` shows nothing or 404 error

**Causes:**
1. Mounted `/var/lib/lightnvr` directly (overwrote web assets)
2. Web assets not copied during initialization

**Solution:**
```bash
# Stop container
docker-compose down

# Remove incorrect volume mount
# Edit docker-compose.yml to use /var/lib/lightnvr/data instead

# Remove web directory to force re-initialization
rm -rf ./data/web

# Start container
docker-compose up -d
```

### Database Lost on Restart

**Symptom:** All streams and settings disappear after container restart

**Cause:** Data volume not mounted correctly

**Solution:**
```bash
# Verify volume mounts
docker inspect lightnvr | grep -A 10 Mounts

# Should show:
# /etc/lightnvr
# /var/lib/lightnvr/data
```

### WebRTC Not Working

**Symptom:** WebRTC streams fail to connect

**Causes:**
1. UDP port 8555 not forwarded
2. Firewall blocking WebRTC
3. STUN server not reachable

**Solution:**
```bash
# Check if go2rtc is running
docker exec lightnvr ps aux | grep go2rtc

# Check go2rtc logs
docker exec lightnvr cat /var/log/lightnvr/go2rtc.log

# Test STUN connectivity
docker exec lightnvr nc -vzu stun.l.google.com 19302
```

### go2rtc Config Keeps Resetting

**Symptom:** Changes to `go2rtc.yaml` are lost on restart

**Cause:** `GO2RTC_CONFIG_PERSIST` set to `false`

**Solution:**
```yaml
environment:
  - GO2RTC_CONFIG_PERSIST=true
```

## Advanced Configuration

### Custom Configuration Path

Mount a custom config file:

```bash
docker run -d \
  --name lightnvr \
  -v /path/to/custom/lightnvr.ini:/etc/lightnvr/lightnvr.ini \
  -v ./data:/var/lib/lightnvr/data \
  ghcr.io/opensensor/lightnvr:latest
```

### Running Multiple Instances

```yaml
services:
  lightnvr-1:
    image: ghcr.io/opensensor/lightnvr:latest
    ports:
      - "8080:8080"
      - "8554:8554"
    volumes:
      - ./config-1:/etc/lightnvr
      - ./data-1:/var/lib/lightnvr/data

  lightnvr-2:
    image: ghcr.io/opensensor/lightnvr:latest
    ports:
      - "8081:8080"
      - "8555:8554"
    volumes:
      - ./config-2:/etc/lightnvr
      - ./data-2:/var/lib/lightnvr/data
```

### Resource Limits

```yaml
services:
  lightnvr:
    deploy:
      resources:
        limits:
          cpus: '2'
          memory: 2G
        reservations:
          cpus: '1'
          memory: 512M
```

### Logging Configuration

```yaml
services:
  lightnvr:
    logging:
      driver: "json-file"
      options:
        max-size: "10m"
        max-file: "3"
```

## Migration from Previous Versions

If you're upgrading from an older version that mounted `/var/lib/lightnvr` directly, see [DOCKER_MIGRATION_GUIDE.md](../DOCKER_MIGRATION_GUIDE.md) for detailed migration instructions.

## Building from Source

```bash
# Clone repository
git clone https://github.com/opensensor/lightNVR.git
cd lightNVR

# Build image
docker build -t lightnvr:local .

# Run locally built image
docker run -d \
  --name lightnvr \
  -p 8080:8080 \
  -v ./config:/etc/lightnvr \
  -v ./data:/var/lib/lightnvr/data \
  lightnvr:local
```

## Support

For issues and questions:
- GitHub Issues: https://github.com/opensensor/lightNVR/issues
- Documentation: https://github.com/opensensor/lightNVR/tree/main/docs

