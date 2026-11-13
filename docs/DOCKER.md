# Docker Deployment Guide

## Security migration: non-root runtime and private go2rtc API

Images run as UID:GID `10001:10001`. Port 1984 is internal and bound to loopback.
WebRTC, MSE, HLS and snapshots use LightNVR’s authenticated proxy on the dashboard port.
For HTTPS, follow [Reverse Proxy](REVERSE_PROXY.md). The gateway only needs access to
LightNVR's web port; it does not need to share go2rtc's network namespace.

The repository Compose file uses named volumes for config, database, and models,
and a bind mount only for `./data/recordings`. Before the first start, prepare that
recordings directory for UID:GID 10001:10001. Existing deployments must retain or
migrate their config, database, and models before switching volume layouts; otherwise
Docker creates empty volumes and the recorder appears unconfigured. Stop the recorder
and back up the data before migrating. You can keep existing mounts during migration,
provided they contain only LightNVR state and are writable by the runtime UID.

For existing bind mounts, change ownership of the specific LightNVR directories to
10001:10001 on the host, recursively (`chown -R 10001:10001 <host directory>`, which
keeps file modes). Releases before 0.44 ran as root, so subdirectories such as
`go2rtc/`, the database file, and per-camera recording directories are root-owned too;
changing only the top-level directory is not enough. Do not delete a bind-mounted
directory to start over: Docker recreates a missing one owned by root. The entrypoint
does not change ownership or permissions itself; it checks every path before
writing anything and lists each one that is not writable, with its current owner.
Preserve restrictive permissions on credentials and `override.yaml`.

If you cannot change ownership on the host (for example on Unraid or a NAS UI),
start the container once as root instead, with `user: "0:0"` in Compose or
`--user 0:0`. The entrypoint then changes ownership of the LightNVR volumes, only
where it differs and without changing modes, and drops to the runtime user before
doing anything else. LightNVR and go2rtc never run as root. Set `PUID`/`PGID` to use
an ID other than 10001:10001 (Unraid's convention is `PUID=99`, `PGID=100`); they
take effect only when the container starts as root. In recordings, only directories are
changed: segments written by root stay root-owned but remain readable and can be
deleted by retention. This run needs the default `CHOWN`, `DAC_OVERRIDE`, `FOWNER`,
`SETUID` and `SETGID` capabilities, so do not combine it with `cap_drop: ALL`. NFS exports
with `root_squash` must be fixed on the server. Afterwards, stop starting as root:
if you use the default runtime IDs, remove `user: "0:0"`; if you set custom
`PUID`/`PGID`, replace it with `user: "<PUID>:<PGID>"` (for example `user: "99:100"`).

Change an existing `pid_file = /var/run/lightnvr.pid` to
`pid_file = /var/run/lightnvr/lightnvr.pid` in `lightnvr.ini`.
Root-owned application binaries and web assets stay outside writable volumes.
Do not mount host roots, Docker sockets, or unrelated host data into the recorder.

The bundled go2rtc rejects `exec:`, `echo:`, and `expr:` sources, including config-file
streams and redirect results. FFmpeg sources accept network URLs or registered stream
names and supported codec/input presets; raw command fragments, local file/device
inputs, and arbitrary filter/query parameters are rejected. The supported synthetic
`virtual?video=...` test patterns accept only validated pattern parameters.

### Upgrade shows the setup wizard or no recordings

Images including 0.45.1 declared separate Docker `VOLUME` paths for
`/var/lib/lightnvr/data/database`, `recordings`, and `models`. If your existing
Compose file binds only the parent `/var/lib/lightnvr/data`, Docker creates
anonymous volumes at those child paths. They hide the original database and
recordings, so LightNVR sees an empty installation even after ownership is fixed.
The host files are not deleted by this mount behavior. See [discussion #946](https://github.com/opensensor/lightNVR/discussions/946).

Inspect the affected container (replace `lightnvr` with its name):

```bash
docker inspect lightnvr --format '{{range .Mounts}}{{println .Type .Source "->" .Destination}}{{end}}'
```

A parent `bind` plus child `volume` entries at those three paths confirms the
mount conflict. Stop the container and back up the original configuration and
database directory, including any SQLite `-wal` and `-shm` files, before recovery.
Preserve any new data written into the anonymous volumes as well; do not run
`docker compose down -v` or prune volumes during recovery.

For an affected image, explicitly bind the existing child directories. For
example, if `/mnt/user/nvr-recording` is your existing data root and contains
`database/lightnvr.db`, `recordings/`, and `models/`, use:

```yaml
volumes:
  - /mnt/user/appdata/lightnvr/config:/etc/lightnvr
  - /mnt/user/nvr-recording:/var/lib/lightnvr/data
  - /mnt/user/nvr-recording/database:/var/lib/lightnvr/data/database
  - /mnt/user/nvr-recording/recordings:/var/lib/lightnvr/data/recordings
  - /mnt/user/nvr-recording/models:/var/lib/lightnvr/data/models
```

Verify these host paths contain the original data before starting; binding a
missing path creates an empty directory. Keep the existing `[database] path`,
`[storage] path`, and `mp4_path` in `lightnvr.ini`, and check for a `DB_PATH`
environment override. Recreate the container to apply the mounts:

```bash
docker compose up -d --force-recreate lightnvr
```

A restart alone does not change mounts. Current source declares only the parent
data volume again, allowing both legacy parent binds and explicit child mounts.
Containers that already have anonymous child mounts must still be recreated
with the corrected mounts; updating the image alone may retain those volumes.

*Verified against LightNVR 0.37.x.*

This guide provides comprehensive information about deploying LightNVR using Docker.

> **On Windows?** Everything here applies, but the host setup has its own pitfalls.
> Start with [Running LightNVR on Windows with Podman + WSL2](WINDOWS_PODMAN.md).
>
> **On Home Assistant OS or Supervised?** Use the add-on instead — it handles networking,
> storage, and backups for you. See [LightNVR on Home Assistant](HOME_ASSISTANT.md).

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

No clone or build required — the compose file pulls the published image. Save the repository's [`docker-compose.yml`](../docker-compose.yml) into an empty directory (or create it from the example below):

```yaml
services:
  lightnvr:
    image: ghcr.io/opensensor/lightnvr:latest
    container_name: lightnvr
    restart: unless-stopped
    ports:
      - "8080:8080"     # Web UI
      - "8554:8554"     # RTSP
      - "8555:8555"     # WebRTC TCP
      - "8555:8555/udp" # WebRTC UDP
    volumes:
      - lightnvr-config:/etc/lightnvr
      - lightnvr-database:/var/lib/lightnvr/data/database
      - lightnvr-recordings:/var/lib/lightnvr/data/recordings
      - lightnvr-models:/var/lib/lightnvr/data/models
    security_opt:
      - no-new-privileges:true
    cap_drop: [ALL]
    environment:
      - TZ=UTC  # Change to your timezone (e.g., America/New_York)
      # ONVIF discovery network for containers, e.g. 192.168.1.0/24
      # - LIGHTNVR_ONVIF_NETWORK=192.168.1.0/24
volumes:
  lightnvr-config:
  lightnvr-database:
  lightnvr-recordings:
  lightnvr-models:
```

Then start it:

```bash
# Pull the image and start the container
docker compose up -d

# View logs
docker compose logs -f

# Access the web UI
# http://localhost:8080
# Default credentials: admin / admin - change immediately, see First Run Experience
```

To build the image from source instead, clone the repository (with `git submodule update --init --recursive`) and switch the `image:` line in `docker-compose.yml` to the commented-out `build:` block — see [Building from Source](#building-from-source).

### Using Docker Run

```bash
docker run -d \
  --name lightnvr \
  --restart unless-stopped \
  -p 8080:8080 \
  -p 8554:8554 \
  -p 8555:8555 \
  -p 8555:8555/udp \
  --security-opt no-new-privileges --cap-drop ALL \
  -v lightnvr-config:/etc/lightnvr \
  -v lightnvr-database:/var/lib/lightnvr/data/database \
  -v lightnvr-recordings:/var/lib/lightnvr/data/recordings \
  -v lightnvr-models:/var/lib/lightnvr/data/models \
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

2. **UID/GID mapping**: The container runs as UID:GID `10001:10001`. Grant that identity access to the dedicated LightNVR export, or map it to a dedicated NAS service account. Keep root squashing enabled.

3. **Directory pre-creation**: For better reliability with NFS, pre-create the directory structure on your NAS:
   ```bash
   # On your NAS or NFS server
   mkdir -p /volume1/docker/lightnvr/config/go2rtc
   mkdir -p /volume1/docker/lightnvr/data/recordings/mp4
   mkdir -p /volume1/docker/lightnvr/data/database
   mkdir -p /volume1/docker/lightnvr/data/models
   chown -R 10001:10001 /volume1/docker/lightnvr
   ```

#### Recording Issues on NFS

If recordings are not being created:

1. **Check write permissions**: The container logs will show write permission test results on startup
2. **Verify NFS mount**: Ensure the NFS volume is actually mounted inside the container:
   ```bash
   docker exec lightnvr ls -la /var/lib/lightnvr/data/recordings/mp4
   ```
3. **Check available space**: Ensure your NAS has sufficient free space
4. **Review logs**: Check for "Failed to open input" or "Operation not permitted" errors:
   ```bash
   docker logs lightnvr | grep -i error
   ```

#### Example NFS Configuration

Here's a complete example for Synology NAS with Traefik:

```yaml
volumes:
  config:
    driver_opts:
      type: "nfs"
      o: "nfsvers=4,addr=${IpAddressNFS},rw"
      device: ":/${NFSVolumePath}/${SystemId}/lightnvr-config"

  data:
    driver_opts:
      type: "nfs"
      o: "nfsvers=4,addr=${IpAddressNFS},rw"
      device: ":/${NFSVolumePath}/${SystemId}/lightnvr-data"
```

**Note**: `nolock` is not a fix for filesystem ownership or permission errors, and it does not disable NFSv4 locking. If you see permission issues, verify UID:GID mapping (`10001:10001`), NFS export permissions, and server-side ACLs on the NAS.

### Web Assets

Web assets are baked into the image at `/var/lib/lightnvr/www` during the build. The
entrypoint verifies `index.html` is present and refuses to start if it is not; there is no
copy-on-first-run step. This means:

- The web UI works immediately after container start
- Pulling a new image updates the web UI
- **Never mount a volume over `/var/lib/lightnvr`** — it would hide the assets and the
  container will fail to start. Mount `/var/lib/lightnvr/data` instead.

## Network Configuration

### Port Mapping

| Port | Protocol | Service | Description |
|------|----------|---------|-------------|
| 8080 | TCP | Web UI | Main web interface |
| 8554 | TCP | RTSP | RTSP streaming server |
| 8555 | TCP/UDP | WebRTC | WebRTC streaming |
| 1984 | TCP | Internal go2rtc API | Loopback only; do not publish |

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
| `LIGHTNVR_FFMPEG_MAX_RSS_MB` | `768` for audio, `1536` for video | Per-child RSS cutoff for ffmpeg started by managed go2rtc. Valid range: 128–2048 MiB. Audio-only jobs also receive an address-space limit 256 MiB above this value. |
| `LIGHTNVR_ONVIF_NETWORK` | (none) | Override ONVIF discovery network (e.g., `192.168.1.0/24`) |
| `LIGHTNVR_MIGRATIONS_DIR` | (none) | Override where database migrations are read from. Falls back to `./db/migrations`, then `/usr/share/lightnvr/migrations`, then a path relative to the binary. |

Archive, recovery and licence-plate variables (`LIGHTNVR_ARCHIVE_*`,
`LIGHTNVR_STORAGE_READ_ONLY`, `LIGHTNVR_LPR_MASTER_KEY_HEX`) are described with their
features in [STORAGE_ARCHIVE.md](STORAGE_ARCHIVE.md) and [ONVIF_DETECTION.md](ONVIF_DETECTION.md).

### Configuration overrides

These `LIGHTNVR_*` variables set `lightnvr.ini` values, which is convenient for
provisioning containers. **An override applies only while the setting is still at its
built-in default**: if `lightnvr.ini` sets the key to anything else, the file wins and
the variable is ignored (the log says so at debug level). Booleans accept
`true`/`1`/`yes`/`on`.

| Variable | `lightnvr.ini` setting |
|----------|------------------------|
| `LIGHTNVR_WEB_PORT` | `[web] port` |
| `LIGHTNVR_WEB_BIND_IP` | `[web] bind_ip` |
| `LIGHTNVR_WEB_SSL_ENABLED`, `LIGHTNVR_WEB_SSL_CERT_PATH`, `LIGHTNVR_WEB_SSL_KEY_PATH` | `[web] ssl_enabled`, `ssl_cert_path`, `ssl_key_path` |
| `LIGHTNVR_WEB_AUTH_ENABLED` | `[web] auth_enabled` |
| `LIGHTNVR_WEB_USERNAME` | `[web] username` |
| `LIGHTNVR_WEB_TRUSTED_PROXY_CIDRS` | `[web] trusted_proxy_cidrs` |
| `LIGHTNVR_DEMO_MODE` | `[web] demo_mode` — grants unauthenticated viewer access |
| `LIGHTNVR_AUDIO_DISABLED` | `[web] audio_disabled` |
| `LIGHTNVR_LOG_LEVEL` | `[general] log_level` (`0` error, `1` warn, `2` info, `3` debug) |
| `LIGHTNVR_STORAGE_PATH` | `[storage] path` |
| `LIGHTNVR_RETENTION_DAYS` | `[storage] retention_days` |
| `LIGHTNVR_DB_PATH` | `[database] path` |
| `LIGHTNVR_DB_BACKUP_INTERVAL_MINUTES`, `LIGHTNVR_DB_BACKUP_RETENTION_COUNT`, `LIGHTNVR_DB_POST_BACKUP_SCRIPT` | `[database] backup_interval_minutes`, `backup_retention_count`, `post_backup_script` |
| `LIGHTNVR_DB_STARTUP_CHECK`, `LIGHTNVR_DB_BACKUP_VERIFY` | `[database] startup_check`, `backup_verify`, as numbers: `0` off, `1` quick, `2` full |
| `LIGHTNVR_GO2RTC_ENABLED`, `LIGHTNVR_GO2RTC_WEBRTC_ENABLED`, `LIGHTNVR_GO2RTC_STUN_ENABLED` | `[go2rtc] enabled`, `webrtc_enabled`, `stun_enabled` |
| `LIGHTNVR_GO2RTC_STUN_SERVER`, `LIGHTNVR_GO2RTC_EXTERNAL_IP`, `LIGHTNVR_GO2RTC_ICE_SERVERS` | `[go2rtc] stun_server`, `external_ip`, `ice_servers` |
| `LIGHTNVR_GO2RTC_WEBRTC_LISTEN_PORT` | `[go2rtc] webrtc_listen_port` |
| `LIGHTNVR_WEBRTC_CONNECTION_TIMEOUT_MS`, `LIGHTNVR_WEBRTC_ICE_RECOVERY_TIMEOUT_MS` | `[go2rtc] webrtc_connection_timeout_ms`, `webrtc_ice_recovery_timeout_ms` |
| `LIGHTNVR_TURN_ENABLED`, `LIGHTNVR_TURN_SERVER_URL`, `LIGHTNVR_TURN_USERNAME`, `LIGHTNVR_TURN_PASSWORD` | `[go2rtc] turn_enabled`, `turn_server_url`, `turn_username`, `turn_password` |

> `LIGHTNVR_AUTO_INIT` and `LIGHTNVR_WEB_ROOT` are set by the image and by
> `docker-compose.yml`, but nothing reads them — neither the entrypoint nor LightNVR
> itself. Setting them has no effect. They are left in place because removing them from
> existing compose files would be a breaking-looking change for no gain; treat them as
> inert. Config initialization always runs when `/etc/lightnvr/lightnvr.ini` is absent,
> and the web root is fixed at `/var/lib/lightnvr/www`.

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
   /var/lib/lightnvr/data/database/
   /var/lib/lightnvr/data/recordings/
   /var/lib/lightnvr/data/models/
   ```

2. **Verifies Web Assets**
   - Checks that `/var/lib/lightnvr/www/index.html` exists — the assets are baked into
     the image at build time and are *not* copied at startup
   - Startup fails with an error if they are missing, which normally means
     `/var/lib/lightnvr` was mounted over. Mount `/var/lib/lightnvr/data`, never
     `/var/lib/lightnvr` itself.

3. **Creates Default Configuration**
   - `lightnvr.ini` with sensible defaults
   - `go2rtc.yaml` with WebRTC/STUN configuration

4. **Seeds Detection Models**
   - Copies `/usr/share/lightnvr/models/` into `/var/lib/lightnvr/data/models/`, only if
     the destination is empty

5. **Initializes Database**
   - Creates SQLite database on first access
   - Sets up default admin user

### Default Credentials

- **Username:** `admin`
- **Password:** `admin`

⚠️ The web server binds `0.0.0.0` inside the container, so once you publish port 8080
these credentials are the only thing in front of the first-login flow. LightNVR requires
the default password to be replaced immediately after login and blocks the rest of the UI
and password-authenticated APIs until that succeeds. You then sign in again with the new
password; MFA, when enabled, follows on that sign-in.

To avoid the default ever being valid, set `password` in the `[web]` section *before*
the first start. In the default Compose setup (named volumes), set it in
`/etc/lightnvr/lightnvr.ini` inside the mounted config volume, and make sure that file is
populated before the container is started for the first time (for example, by starting a
temporary container with the config volume mounted and writing `lightnvr.ini` there). If
you want to manage config from the host, first switch Compose to a `./config` bind mount,
then edit `./config/lightnvr.ini`. The admin account is created with that password
instead. The setting is only read when the account is created; afterwards users are
managed from the **Users** page.

Existing accounts are not flagged during an upgrade. API-key authentication and demo
mode remain available while a freshly bootstrapped administrator is awaiting a password
change.

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

Edit `/etc/lightnvr/go2rtc/go2rtc.yaml` in the container's named config volume (for example via `docker compose exec`):

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

If you intentionally switched Compose to use a prepared host `./config` bind mount, edit `./config/go2rtc/go2rtc.yaml` instead.

Restart the container to apply changes:
```bash
docker compose restart
```

### Port Forwarding for WebRTC

If running behind NAT/firewall, forward these ports:

- **8555/TCP** - WebRTC media (ICE-TCP fallback)
- **8555/UDP** - WebRTC media (preferred ICE transport)
- **Dashboard HTTP(S) port** (for example `8080`, or your reverse-proxy HTTPS port) - WebRTC signaling via LightNVR's authenticated proxy

## Troubleshooting

### Web UI Not Loading

**Symptom:** Accessing `http://localhost:8080` shows nothing or 404 error

**Causes:**
1. Mounted `/var/lib/lightnvr` directly (overwrote web assets)

**Solution:**
```bash
# Stop and remove container so mount changes apply cleanly
docker compose down

# Fix incorrect volume mount
# Edit docker-compose.yml to use /var/lib/lightnvr/data instead of /var/lib/lightnvr

# Recreate container with corrected mounts
docker compose up -d
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

**Symptom:** WebRTC streams fail to connect (ICE connection failures)

**Causes:**
1. ICE candidates not resolving to the correct host IP address
2. UDP port 8555 not forwarded
3. Firewall blocking WebRTC
4. STUN server not reachable

**Solution:**

The most common fix is setting your host machine's local IP as `external_ip` in `/etc/lightnvr/lightnvr.ini` (default Compose/named-volume setup):

```ini
[go2rtc]
external_ip = 192.168.1.100  ; Replace with your machine's local IP
```

If your deployment explicitly bind-mounts `./config` to `/etc/lightnvr`, edit `./config/lightnvr.ini` on the host instead.

To find your local IP:
```bash
# Linux
hostname -I | awk '{print $1}'

# macOS
ipconfig getifaddr en0    # or en1 for Wi-Fi
```

Then restart the container:
```bash
docker compose restart
```

If that doesn't resolve it, check go2rtc connectivity:
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

Before starting, create the bind-mount directories and set ownership for the non-root runtime user:

```bash
mkdir -p ./config-1 ./data-1 ./config-2 ./data-2
sudo chown -R 10001:10001 ./config-1 ./data-1 ./config-2 ./data-2
```

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

If you're upgrading from an older version that mounted `/var/lib/lightnvr` directly, change
the mount to `/var/lib/lightnvr/data` and move your existing content down one level:

```bash
docker compose down

# Your old volume held database/, recordings/ and models/ at its root, alongside
# the web assets. Only the first three should persist.
mkdir -p ./data
mv ./old-volume/database ./old-volume/recordings ./old-volume/models ./data/ 2>/dev/null

# Ensure migrated content is writable by the non-root runtime user (UID:GID 10001:10001)
sudo chown -R 10001:10001 ./data

# Then point the volume at the data subdirectory:
#   - ./data:/var/lib/lightnvr/data
docker compose up -d
```

Mounting `/var/lib/lightnvr` itself hides the web assets baked into the image, and the
container will refuse to start with a "Web assets not found" error.

## Building from Source

```bash
# Clone repository
git clone https://github.com/opensensor/lightNVR.git
cd lightNVR

# Initialize submodules (required for go2rtc)
git submodule update --init --recursive

# Build image
docker build --pull -t lightnvr:local .

# Prepare bind-mount directories for non-root runtime (UID:GID 10001:10001)
mkdir -p ./config ./data
sudo chown -R 10001:10001 ./config ./data

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
