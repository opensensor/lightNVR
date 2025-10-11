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
path = /var/lib/lightnvr/recordings
max_size = 0  ; 0 means unlimited, otherwise bytes
retention_days = 30
auto_delete_oldest = true

[database]
path = /var/lib/lightnvr/lightnvr.db

[models]
path = /var/lib/lightnvr/models

[web]
port = 8080
root = /var/lib/lightnvr/www
auth_enabled = true
username = admin
password = admin  ; IMPORTANT: Change this default password!

[streams]
max_streams = 16

; Note: Stream configurations are stored in the database
; This section is for reference only

; Example stream configuration:
; [stream_0]
; name = Front Door
; url = rtsp://192.168.1.100:554/stream1
; enabled = true
; width = 1920
; height = 1080
; fps = 15
; codec = h264
; priority = 10
; record = true
; segment_duration = 900  ; 15 minutes in seconds

[memory]
buffer_size = 1024  ; Buffer size in KB
use_swap = true
swap_file = /var/lib/lightnvr/swap
swap_size = 134217728  ; 128MB in bytes

[hardware]
hw_accel_enabled = false
hw_accel_device = 
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

```
# Storage Settings
storage_path=/var/lib/lightnvr/recordings
max_storage_size=0  # 0 means unlimited, otherwise bytes
retention_days=30
auto_delete_oldest=true
```

- `storage_path`: Directory where recordings are stored
- `max_storage_size`: Maximum storage size in bytes (0 means unlimited)
- `retention_days`: Number of days to keep recordings
- `auto_delete_oldest`: Whether to automatically delete the oldest recordings when storage is full

### Models Settings

```
# Models Settings
models_path=/var/lib/lightnvr/models
```

- `models_path`: Directory where detection models are stored

### Database Settings

```
# Database Settings
db_path=/var/lib/lightnvr/lightnvr.db
```

- `db_path`: Path to the SQLite database file

### Web Server Settings

```
# Web Server Settings
web_port=8080
web_root=/var/lib/lightnvr/www
web_auth_enabled=true
web_username=admin
web_password=admin  # IMPORTANT: Change this default password!
```

- `web_port`: Port for the web interface
- `web_root`: Directory containing web interface files
- `web_auth_enabled`: Whether to enable authentication for the web interface
- `web_username`: Username for web interface authentication
- `web_password`: Password for web interface authentication

### Stream Settings

```
# Stream Settings
max_streams=16
```

- `max_streams`: Maximum number of streams to support

### Memory Optimization

```
# Memory Optimization
buffer_size=1024  # Buffer size in KB
use_swap=true
swap_file=/var/lib/lightnvr/swap
swap_size=134217728  # 128MB in bytes
```

- `buffer_size`: Buffer size for video processing in KB
- `use_swap`: Whether to use a swap file for additional memory
- `swap_file`: Path to the swap file
- `swap_size`: Size of the swap file in bytes

### Hardware Acceleration

```
# Hardware Acceleration
hw_accel_enabled=false
hw_accel_device=
```

- `hw_accel_enabled`: Whether to enable hardware acceleration
- `hw_accel_device`: Device to use for hardware acceleration

### Stream Configurations

Each stream is configured with a set of parameters:

```
# Stream 0
stream.0.name=Front Door
stream.0.url=rtsp://192.168.1.100:554/stream1
stream.0.enabled=true
stream.0.width=1920
stream.0.height=1080
stream.0.fps=15
stream.0.codec=h264
stream.0.priority=10
stream.0.record=true
stream.0.segment_duration=900  # 15 minutes in seconds
```

- `stream.N.name`: Name of the stream
- `stream.N.url`: URL of the stream (RTSP or ONVIF)
- `stream.N.enabled`: Whether the stream is enabled
- `stream.N.width`: Width of the stream in pixels
- `stream.N.height`: Height of the stream in pixels
- `stream.N.fps`: Frame rate of the stream
- `stream.N.codec`: Codec of the stream (h264 or h265)
- `stream.N.priority`: Priority of the stream (1-10, higher = more important)
- `stream.N.record`: Whether to record the stream
- `stream.N.segment_duration`: Duration of each recording segment in seconds

## Example Configuration

Here's a complete example configuration file:

```
# LightNVR Configuration File

# General Settings
pid_file=/var/run/lightnvr.pid
log_file=/var/log/lightnvr.log
log_level=2  # 0=ERROR, 1=WARN, 2=INFO, 3=DEBUG

# Storage Settings
storage_path=/var/lib/lightnvr/recordings
max_storage_size=0  # 0 means unlimited, otherwise bytes
retention_days=30
auto_delete_oldest=true

# Database Settings
db_path=/var/lib/lightnvr/lightnvr.db

# Models Settings
models_path=/var/lib/lightnvr/models

# Web Server Settings
web_port=8080
web_root=/var/lib/lightnvr/www
web_auth_enabled=true
web_username=admin
web_password=admin  # IMPORTANT: Change this default password!

# Stream Settings
max_streams=16

# Memory Optimization
buffer_size=1024  # Buffer size in KB
use_swap=true
swap_file=/var/lib/lightnvr/swap
swap_size=134217728  # 128MB in bytes

# Hardware Acceleration
hw_accel_enabled=false
hw_accel_device=

# Stream Configurations
stream.0.name=Front Door
stream.0.url=rtsp://192.168.1.100:554/stream1
stream.0.enabled=true
stream.0.width=1920
stream.0.height=1080
stream.0.fps=15
stream.0.codec=h264
stream.0.priority=10
stream.0.record=true
stream.0.segment_duration=900  # 15 minutes in seconds

stream.1.name=Back Yard
stream.1.url=rtsp://192.168.1.101:554/stream1
stream.1.enabled=true
stream.1.width=1280
stream.1.height=720
stream.1.fps=10
stream.1.codec=h264
stream.1.priority=5
stream.1.record=true
stream.1.segment_duration=900  # 15 minutes in seconds
```

## Command Line Options

LightNVR supports the following command line options:

- `-c, --config FILE`: Use the specified configuration file
- `-d, --daemon`: Run as a daemon
- `-h, --help`: Show help message
- `-v, --version`: Show version information

## Memory Optimization for Ingenic A1

The Ingenic A1 SoC has limited memory (256MB), so it's important to optimize memory usage:

1. Set appropriate buffer sizes:
   ```
   buffer_size=512  # 512KB buffer size
   ```

2. Enable swap file for additional memory:
   ```
   use_swap=true
   swap_file=/var/lib/lightnvr/swap
   swap_size=134217728  # 128MB in bytes
   ```

3. Limit the number of streams and their resolution/frame rate:
   ```
   max_streams=8
   
   stream.0.width=1280
   stream.0.height=720
   stream.0.fps=10
   ```

4. Set stream priorities to ensure important streams get resources:
   ```
   stream.0.priority=10  # High priority
   stream.1.priority=5   # Medium priority
   stream.2.priority=1   # Low priority
   ```

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
