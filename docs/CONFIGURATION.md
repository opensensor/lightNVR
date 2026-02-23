# LightNVR Configuration Guide

This document describes the configuration options for LightNVR.

## Configuration File

LightNVR supports two configuration file formats:

1. **INI format** (recommended): A structured configuration format with sections and key-value pairs.
2. **Legacy key-value format**: A simple key-value configuration format.

By default, LightNVR looks for a configuration file in the following locations (in order):

1. `./lightnvr.ini` (INI format in current directory)
2. `/etc/lightnvr/lightnvr.ini` (INI format in system directory)
3. `./lightnvr.conf` (legacy format in current directory)
4. `/etc/lightnvr/lightnvr.conf` (legacy format in system directory)

You can specify a different configuration file using the `-c` option:

```bash
./lightnvr -c /path/to/config.conf
```

When saving configuration changes through the web interface, LightNVR will prefer to use the INI format.

## INI Format

The INI format is a structured configuration format that organizes settings into sections. Here's an example of the same configuration in INI format:

```ini
; LightNVR Configuration File (INI format)

[general]
pid_file = /var/run/lightnvr.pid
log_file = /var/log/lightnvr.log
log_level = 2  ; 0=ERROR, 1=WARN, 2=INFO, 3=DEBUG
syslog_enabled = false  ; Enable logging to syslog
syslog_ident = lightnvr  ; Syslog identifier
syslog_facility = LOG_USER  ; Syslog facility

[storage]
path = /var/lib/lightnvr/data/recordings
max_size = 0  ; 0 means unlimited, otherwise bytes
retention_days = 30
auto_delete_oldest = true

; New recording format options
record_mp4_directly = false
mp4_path = /var/lib/lightnvr/data/recordings/mp4
mp4_segment_duration = 900
mp4_retention_days = 30

[database]
path = /var/lib/lightnvr/data/database/lightnvr.db

[web]
port = 8080
root = /var/lib/lightnvr/www
auth_enabled = true
username = admin
; Password is auto-generated on first run - check logs for the generated password
; password =
auth_timeout_hours = 24  ; Session timeout in hours (default: 24)
web_thread_pool_size = 8

[streams]
max_streams = 32

; Note: Stream configurations are stored in the database
; and managed via the API/web UI

[models]
path = /var/lib/lightnvr/data/models

[api_detection]
url = http://localhost:9001/api/v1/detect
backend = onnx  ; Detection backend: onnx (YOLOv8), tflite, or opencv
confidence_threshold = 0.35
filter_classes = car,motorcycle,truck,bus,bicycle  ; Comma-separated class filter

[memory]
buffer_size = 1024  ; Buffer size in KB
use_swap = true
swap_file = /var/lib/lightnvr/data/swap
swap_size = 134217728  ; 128MB in bytes

[hardware]
hw_accel_enabled = false
hw_accel_device =

[go2rtc]
webrtc_enabled = true
webrtc_listen_port = 8555
stun_enabled = true
stun_server = stun.l.google.com:19302
; external_ip =
; ice_servers =
; proxy_max_inflight = 16

[mqtt]
enabled = false
broker_host = localhost
broker_port = 1883
; username =
; password =
client_id = lightnvr
topic_prefix = lightnvr
tls_enabled = false
keepalive = 60
qos = 1
retain = false

[onvif]
discovery_enabled = false
discovery_interval = 300
discovery_network = auto
```

The INI format offers several advantages:
- Simple and widely used format
- Easy to read and edit
- Organized into sections
- Support for comments
- Lightweight parsing

## Configuration Options

The configuration file is divided into several sections:

### General Settings

```ini
[general]
pid_file = /var/run/lightnvr.pid
log_file = /var/log/lightnvr.log
log_level = 2  ; 0=ERROR, 1=WARN, 2=INFO, 3=DEBUG
syslog_enabled = false  ; Enable logging to syslog
syslog_ident = lightnvr  ; Syslog identifier (application name)
syslog_facility = LOG_USER  ; Syslog facility
```

- `pid_file`: Path to the PID file
- `log_file`: Path to the log file
- `log_level`: Logging level (0=ERROR, 1=WARN, 2=INFO, 3=DEBUG)
- `syslog_enabled`: Enable logging to syslog for easier system integration and centralized log management (default: false)
- `syslog_ident`: Syslog identifier/application name used in syslog messages (default: "lightnvr")
- `syslog_facility`: Syslog facility for categorizing messages. Valid values:
  - `LOG_USER` (default): User-level messages
  - `LOG_DAEMON`: System daemon messages
  - `LOG_LOCAL0` through `LOG_LOCAL7`: Local use facilities for custom applications

#### Syslog Integration

When `syslog_enabled` is set to `true`, LightNVR will send log messages to the system's syslog daemon in addition to the regular log file and console output. This provides several benefits:

- **Centralized Logging**: Integrate with system-wide log management tools
- **Remote Logging**: Forward logs to remote syslog servers for centralized monitoring
- **Log Rotation**: Leverage system log rotation policies
- **Integration with Monitoring Tools**: Use tools like `journalctl`, `rsyslog`, or `syslog-ng`

Example syslog configuration for production use:

```ini
syslog_enabled = true
syslog_ident = lightnvr
syslog_facility = LOG_LOCAL0
```

To view LightNVR logs via syslog on systemd-based systems:

```bash
# View all LightNVR logs
journalctl -t lightnvr

# Follow logs in real-time
journalctl -t lightnvr -f

# View logs from the last hour
journalctl -t lightnvr --since "1 hour ago"
```

On traditional syslog systems, logs will appear in `/var/log/syslog` or `/var/log/messages` depending on your syslog configuration.

### Storage Settings

```ini
[storage]
path = /var/lib/lightnvr/data/recordings
max_size = 0  ; 0 means unlimited, otherwise bytes
retention_days = 30
auto_delete_oldest = true
record_mp4_directly = false
mp4_path = /var/lib/lightnvr/data/recordings/mp4
mp4_segment_duration = 900
mp4_retention_days = 30
```

- `path`: Directory where recordings are stored
- `max_size`: Maximum storage size in bytes (0 means unlimited)
- `retention_days`: Number of days to keep recordings
- `auto_delete_oldest`: Whether to automatically delete the oldest recordings when storage is full
- `record_mp4_directly`: Enable direct MP4 recording (instead of HLS-to-MP4 conversion)
- `mp4_path`: Directory for direct MP4 recordings
- `mp4_segment_duration`: Duration of each MP4 segment in seconds
- `mp4_retention_days`: Number of days to keep MP4 recordings

### Database Settings

```ini
[database]
path = /var/lib/lightnvr/data/database/lightnvr.db
```

- `path`: Path to the SQLite database file

### Web Server Settings

```ini
[web]
port = 8080
root = /var/lib/lightnvr/www
auth_enabled = true
username = admin
; password is auto-generated on first run
auth_timeout_hours = 24
web_thread_pool_size = 8
```

- `port`: Port for the web interface
- `root`: Directory containing web interface files
- `auth_enabled`: Whether to enable authentication for the web interface
- `username`: Username for web interface authentication
- `password`: Password for web interface authentication (auto-generated on first run if not set)
- `auth_timeout_hours`: Session timeout in hours (default: 24)
- `web_thread_pool_size`: Number of worker threads for the web server (default: 8)

### Stream Settings

```ini
[streams]
max_streams = 32
```

- `max_streams`: Maximum number of streams to support (default: 32, max: 32)

**Note:** Stream configurations are stored in the SQLite database and managed via the API or web UI. They are no longer configured in the INI file.

### Models Settings

```ini
[models]
path = /var/lib/lightnvr/data/models
```

- `path`: Directory where detection models are stored

### API Detection Settings

```ini
[api_detection]
url = http://localhost:9001/api/v1/detect
backend = onnx
confidence_threshold = 0.35
filter_classes = car,motorcycle,truck,bus,bicycle
```

- `url`: URL of the external detection API
- `backend`: Detection backend to use: `onnx` (YOLOv8 - best accuracy), `tflite`, or `opencv`
- `confidence_threshold`: Minimum confidence threshold for detections (0.0-1.0)
- `filter_classes`: Comma-separated list of object classes to detect (empty = all classes)

### Memory Optimization

```ini
[memory]
buffer_size = 1024  ; Buffer size in KB
use_swap = true
swap_file = /var/lib/lightnvr/data/swap
swap_size = 134217728  ; 128MB in bytes
```

- `buffer_size`: Buffer size for video processing in KB
- `use_swap`: Whether to use a swap file for additional memory
- `swap_file`: Path to the swap file
- `swap_size`: Size of the swap file in bytes

### Hardware Acceleration

```ini
[hardware]
hw_accel_enabled = false
hw_accel_device =
```

- `hw_accel_enabled`: Whether to enable hardware acceleration
- `hw_accel_device`: Device to use for hardware acceleration

### go2rtc Settings

```ini
[go2rtc]
webrtc_enabled = true
webrtc_listen_port = 8555
stun_enabled = true
stun_server = stun.l.google.com:19302
; external_ip =
; ice_servers =
; proxy_max_inflight = 16
```

- `webrtc_enabled`: Enable WebRTC streaming (default: true)
- `webrtc_listen_port`: Port for WebRTC connections (default: 8555)
- `stun_enabled`: Enable STUN for NAT traversal (default: true)
- `stun_server`: Primary STUN server address
- `external_ip`: External IP for complex NAT scenarios (leave empty for auto-detection)
- `ice_servers`: Custom ICE servers, comma-separated (format: `stun:host:port` or `turn:host:port`)
- `proxy_max_inflight`: Maximum concurrent HLS/snapshot proxy requests (default: 16, range: 1-128)

### MQTT Settings

```ini
[mqtt]
enabled = false
broker_host = localhost
broker_port = 1883
; username =
; password =
client_id = lightnvr
topic_prefix = lightnvr
tls_enabled = false
keepalive = 60
qos = 1
retain = false
```

- `enabled`: Enable MQTT publishing of detection events (default: false)
- `broker_host`: MQTT broker hostname or IP address
- `broker_port`: MQTT broker port (default: 1883, use 8883 for TLS)
- `username`: MQTT authentication username (optional)
- `password`: MQTT authentication password (optional)
- `client_id`: MQTT client ID (default: lightnvr)
- `topic_prefix`: Topic prefix for detection events. Events are published to `{topic_prefix}/detections/{stream_name}`
- `tls_enabled`: Enable TLS for MQTT connection (default: false)
- `keepalive`: MQTT keepalive interval in seconds (default: 60)
- `qos`: MQTT QoS level: 0 (at most once), 1 (at least once), 2 (exactly once)
- `retain`: Retain detection messages on the broker (default: false)

See [MQTT_INTEGRATION.md](MQTT_INTEGRATION.md) for detailed MQTT documentation.

### ONVIF Settings

```ini
[onvif]
discovery_enabled = false
discovery_interval = 300
discovery_network = auto
```

- `discovery_enabled`: Enable automatic ONVIF camera discovery (default: false)
- `discovery_interval`: Interval in seconds between discovery scans (30-3600, default: 300)
- `discovery_network`: Network to scan in CIDR notation, or `auto` for automatic detection. For Docker containers, set `LIGHTNVR_ONVIF_NETWORK` environment variable instead.

## Example Configuration

See `config/lightnvr.ini` in the repository for a complete, annotated example configuration file. The example includes all available sections and settings with descriptive comments.

## Command Line Options

LightNVR supports the following command line options:

- `-c, --config FILE`: Use the specified configuration file
- `-d, --daemon`: Run as a daemon
- `-h, --help`: Show help message
- `-v, --version`: Show version information

## Memory Optimization for Ingenic A1

The Ingenic A1 SoC has limited memory (256MB), so it's important to optimize memory usage:

1. Set appropriate buffer sizes:
   ```ini
   [memory]
   buffer_size = 512  ; 512KB buffer size
   ```

2. Enable swap file for additional memory:
   ```ini
   [memory]
   use_swap = true
   swap_file = /var/lib/lightnvr/data/swap
   swap_size = 134217728  ; 128MB in bytes
   ```

3. Limit the number of streams via `[streams]` section and configure lower resolution/fps via the web UI.

4. Use stream priorities (1-10, higher = more important) to ensure critical streams get resources when memory is constrained.

## Troubleshooting

If you encounter issues with your configuration:

1. Check the log file for error messages:
   ```
   tail -f /var/log/lightnvr/lightnvr.log
   ```

2. Verify that all paths in the configuration file exist and have the correct permissions

3. Test stream URLs separately using a tool like VLC or ffmpeg:
   ```
   ffplay rtsp://192.168.1.100:554/stream1
   ```

4. Monitor memory usage to ensure you're not exceeding the available memory:
   ```
   top -p $(pgrep lightnvr)
