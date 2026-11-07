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
mp4_directory_format = year_month_day
mp4_segment_duration = 900
mp4_retention_days = 30

[database]
path = /var/lib/lightnvr/data/database/lightnvr.db

[web]
port = 8080
bind_ip = 0.0.0.0
root = /var/lib/lightnvr/www
auth_enabled = true
username = admin
; Only read when the admin account is first created. Set it before the first start to
; avoid the default admin/admin ever being valid; afterwards manage users in the web UI.
; password =
auth_timeout_hours = 24  ; Session timeout in hours (default: 24)
audio_disabled = false  ; Instance-wide audio compliance policy
; trusted_proxy_cidrs = 127.0.0.1/32,::1/128  ; Only trust X-Forwarded-For from these reverse proxies
web_thread_pool_size = 8

[streams]
max_streams = 32

; Note: Stream configurations are stored in the database
; and managed via the API/web UI

[models]
path = /var/lib/lightnvr/data/models

[api_detection]
url = http://localhost:8000/api/v1/detect
format = light-object-detect  ; Request format: light-object-detect or doods2
backend = onnx  ; light-object-detect backend: onnx (YOLOv8), tflite, or opencv
detector_name = default  ; doods2 detector_name
detection_threshold = 50

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

### Multiple Recording Storage Targets

The `[storage]` path remains the compatibility/default recording root. After the
database initializes, lightNVR registers the effective MP4 recording root as a
stable default target and attaches existing recording metadata by relative key;
it does not move footage.

Additional local or administrator-mounted NFS/SMB roots are managed under
**Settings → Storage → Recording storage targets** or through
`/api/storage-targets`. Each target has its own stable UUID, enabled state,
performance class, reserved headroom, watermarks, cached capacity, and health.
Enabled roots must already be mounted and writable. lightNVR does not mount
remote shares or store share credentials.

**Adding a target does not by itself change where anything is recorded.** A
target is inventory: new segments keep going to the default target until a
camera placement policy routes cameras to the new target, either as the
policy's primary target (or a pool containing it) or as its archive target
with an age or pressure trigger. A passing **Test target** only proves the
directory is writable. Two more things keep a target idle even with a policy:
the target must be `healthy` (a target above its high watermark or inside its
reserve is `degraded` and is skipped), and footage recorded before the policy
existed is not attributed to it, so only new segments are affected. When a
policy's primary target is skipped, the log reports
`Storage policy "<name>": primary not eligible (...)` with the reason.

Storage pools and selector-driven lifecycle policies are managed on the same
page. A pool can allocate by most-free, round-robin, or explicit priority.
Policies may place new segments into a pool, require distinct verified copies,
move footage to an archive target after an age threshold, and set
minimum/desired/maximum retention. Lifecycle copies and moves are durable,
restart-safe, bandwidth-limited, and constrained to each destination target's
local-time archival window. The compliance panel uses the prior 30 days of
observed bytes to forecast target pressure and expected policy retention.

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
mp4_directory_format = year_month_day
mp4_segment_duration = 900
mp4_retention_days = 30
```

- `path`: Directory where recordings are stored
- `max_size`: Maximum storage size in bytes (0 means unlimited)
- `retention_days`: Number of days to keep recordings. Age-based retention
  applies to every configured camera, enabled or disabled (deleting a camera
  from the UI without "permanent" only disables it), and to recordings whose
  camera was permanently deleted; per-camera overrides still apply, and
  protected recordings are never expired by age.
- `auto_delete_oldest`: Whether to automatically delete the oldest recordings when storage is full
- `mp4_faststart`: Whether to rewrite each finished MP4 segment with its index (moov) at the front (default `true`). The rewrite re-reads and re-writes the whole segment, which roughly triples I/O on network storage (NFS/SMB) or a slow HDD; players that use HTTP range requests play either layout, so set `false` when recording to a NAS.
- `record_mp4_directly`: Enable direct MP4 recording (instead of HLS-to-MP4 conversion)
- `mp4_path`: Directory for direct MP4 recordings
- `mp4_directory_format`: Recording folder layout: `flat`, `year_month` (`YYYY/MM`), or `year_month_day` (`YYYY/MM/DD`, default)
- `mp4_segment_duration`: Duration of each MP4 segment in seconds
- `mp4_retention_days`: Number of days to keep MP4 recordings
- `path_hls`: Separate directory for live HLS segments (default: empty, meaning under `path`). Useful for putting short-lived segments on tmpfs or a faster disk.
- `min_free_pct`: Keep at least this percentage of the recording volume free; storage cleanup deletes recordings to restore it (default: 10, range 0-90, 0 disables the cap).
- `pressure_warning_pct`, `pressure_critical_pct`, `pressure_emergency_pct`: Free-space percentages below which the storage pressure level becomes warning, critical and emergency (defaults: 20, 10, 5). They must decrease strictly from warning to emergency; invalid values reset all three to the defaults.
- `generate_thumbnails`: Generate preview thumbnails for recordings (default: true)
- `thumbnails_per_recording`: `1` or `3` thumbnails per recording (default: 3; any other value rounds to the nearer of the two)

### Database Settings

```ini
[database]
path = /var/lib/lightnvr/data/database/lightnvr.db
startup_check = off
backup_interval_minutes = 0
backup_retention_count = 6
backup_verify = full
; post_backup_script = /usr/local/bin/ship-backup
```

- `path`: Path to the SQLite database file
- `startup_check`: Optional consistency check run against an existing database
  at boot (default: `off`). `quick` runs `PRAGMA quick_check`, which validates page
  structure. `full` runs `PRAGMA integrity_check`, which additionally
  cross-checks every index against its table — its cost scales with total index
  size, so on a large database it can take minutes, and it runs before the HTTP
  listener is bound, so that time appears to clients as a gateway error. `off`
  skips the check. Corruption seen in practice (truncated writes, torn pages)
  is caught by `quick`. Both checks scan the database before the HTTP listener
  binds, so run them during a maintenance window rather than on every boot.
- `backup_retention_count`: Number of timestamped backups to retain (default: 6).
  Each backup is a full copy of the database, so this multiplies disk usage by
  the database size.
- `backup_verify`: Scan applied to the copied file after each backup before it
  is published (default: `full`). `full` runs `PRAGMA integrity_check`, which
  also cross-checks every index against its table and seeks across the whole
  file; on a large database over a mechanical disk that scan can take longer
  than the copy and may hit the 30-minute stuck-backup safety valve, in which
  case the backup is discarded. `quick` runs `PRAGMA quick_check`, which
  validates page structure (torn or truncated pages, corrupted b-trees) with
  far fewer seeks; the backup is a page-level copy of a consistent snapshot, so
  index/table consistency is inherited from the source. `off` skips the scan.
  Every mode logs `Backup verification (<mode>) ... in N s`.
- `backup_interval_minutes`: Take a timestamped database backup every N minutes while running (default: 0, disabled).
- `post_backup_script`: Absolute path to an executable run after each successful backup, for example to copy it off the host. It receives three arguments: the backup file path, the live database path, and the backup directory. A non-zero exit is logged as an error.

### Web Server Settings

```ini
[web]
port = 8080
bind_ip = 0.0.0.0
root = /var/lib/lightnvr/www
; ssl_enabled = false
; ssl_cert_path = /etc/lightnvr/tls/cert.pem
; ssl_key_path = /etc/lightnvr/tls/key.pem
auth_enabled = true
username = admin
; password = ; only read when the admin account is first created
auth_timeout_hours = 24
audio_disabled = false
; trusted_proxy_cidrs = 127.0.0.1/32,::1/128
web_thread_pool_size = 8
```

- `port`: Port for the web interface
- `bind_ip`: IP address for the web interface
- `root`: Directory containing web interface files
- `ssl_enabled`: Serve the web interface over HTTPS instead of HTTP on `port` (default: false). Needs a build configured with `-DENABLE_SSL=ON`; a build without it logs a warning and keeps serving HTTP. See [HTTPS without a reverse proxy](REVERSE_PROXY.md#https-without-a-reverse-proxy).
- `ssl_cert_path`: PEM certificate file, leaf certificate first, followed by any intermediates
- `ssl_key_path`: PEM private key for that certificate (unencrypted). If either file is missing, unreadable or does not match the other, LightNVR does not start the web server; it never falls back to plain HTTP.
- `auth_enabled`: Whether to enable authentication for the web interface
- `username`: Username for web interface authentication
- `password`: Password used **only** when the admin account is first created. If unset, the account is created as `admin` / `admin` — see the warning below. Once the account exists this setting is ignored; passwords live in the database and are changed from the **Users** page.
- `auth_timeout_hours`: Session timeout in hours (default: 24)
- `audio_disabled`: Instance-wide compliance policy (default: false). When true,
  LightNVR prevents audio in new recordings, live listening, and two-way talk;
  the Live View audio controls are removed. Per-stream audio settings and
  custom go2rtc source/config overrides cannot bypass the policy. Existing
  recording files are not rewritten or deleted.
  Container deployments may set the equivalent
  `LIGHTNVR_AUDIO_DISABLED=true` environment variable.
- `trusted_proxy_cidrs`: Comma- or newline-separated list of IPv4/IPv6 CIDRs allowed to set `X-Forwarded-For` / `X-Real-IP` headers. Required when running behind a reverse proxy (nginx, Caddy, Traefik, Kubernetes ingress) — without it, the audit log, login allow-list, and rate limiter all see every request as coming from the proxy itself. See [Reverse Proxy & HTTPS](REVERSE_PROXY.md).
- `web_thread_pool_size`: Number of worker threads for HTTP handlers (default:
  twice the CPU core count, bounded to 2–128; requires restart). An explicit
  `UV_THREADPOOL_SIZE` environment variable takes precedence. API requests,
  including reads of stored detection results, share this pool. Camera detection
  and calls to an external object detector run on separate camera threads;
  go2rtc HTTP proxies also use separate threads. Increasing this value cannot
  resolve a shared database lock or increase the detector's processing capacity.
- `auth_absolute_timeout_hours`: Maximum session lifetime regardless of activity (default: 168, i.e. 7 days; minimum 1). `auth_timeout_hours` is the idle timeout.
- `trusted_device_days`: How long a device marked as trusted after MFA skips the TOTP prompt (default: 30, 0 disables trusted devices).
- `force_mfa_on_login`: Ask for the TOTP code on the same form as the password, so password guesses cannot be tested on their own (default: false).
- `login_rate_limit_enabled`: Lock out repeated failed logins (default: true).
- `login_rate_limit_max_attempts`: Failed attempts allowed per window before lockout (default: 5).
- `login_rate_limit_window_seconds`: Length of the rate-limit window (default: 300, minimum 10).
- `webrtc_disabled`, `hls_disabled`, `mse_disabled`: Hide that live-view transport on the dashboard (default: false for each).
- `auto_disabled`: Hide the **Auto** view and use the first enabled transport instead (default: false).
- `default_playback_transport`: Transport profile given to new streams: `auto` (default), `webrtc_only`, `mse_only` or `hls_only`.
- `demo_mode`: Give unauthenticated visitors **viewer** access (default: false). Intended for public demonstration instances; never enable it on a recorder with private cameras.

Slow-request logs distinguish `HTTP worker queue wait`, `handler execution`,
and `event loop completion wait`, using the request path and request ID. Endpoint
phases such as `stream configuration database read` identify delays inside an
already-running handler. Stream inventory and setup-status reads use private
read-only SQLite connections, so they can read committed data while the shared
writer connection is busy. Disk stalls or database locks outside WAL mode can
still delay reads; a setup-status database failure returns HTTP 503 rather than
incorrectly reporting that initial setup is incomplete.

> ⚠️ **Default credentials are `admin` / `admin`.** Combined with the default
> `bind_ip = 0.0.0.0`, anyone who can reach port 8080 can reach your cameras and
> recordings if an attacker completes first login before you do. Either set `password`
> before the very first start, or log in with `admin` / `admin` and complete the mandatory
> password-change screen. Until the change succeeds, that password-authenticated session
> can only verify its state and change its own password; MFA is evaluated after the new
> password is set and the user signs in again.
>
> The requirement is added only when a new administrator is created with the fallback
> password. Existing users are not flagged on upgrade, and API-key authentication and demo
> mode are unaffected.
>
> There is no password-reset flag. If you lose the admin password, stop LightNVR, delete
> the account row, and restart — it is recreated by the same first-run rules:
>
> ```bash
> sqlite3 /var/lib/lightnvr/data/database/lightnvr.db "DELETE FROM users WHERE username = 'admin';"
> ```

### Stream Settings

```ini
[streams]
max_streams = 32
```

- `max_streams`: Runtime stream capacity for one LightNVR instance (default: 32,
  maximum: 1024). Changing it requires a restart. Higher values reserve more
  memory; the number of cameras a host can actively record, relay, or analyze
  still depends on its CPU, memory, storage, network, and enabled workloads.

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
url = http://localhost:8000/api/v1/detect
format = light-object-detect  ; or doods2
backend = onnx                ; light-object-detect only
detector_name = default       ; doods2 only
detection_threshold = 50      ; Default confidence threshold (0-100%)
```

- `url`: URL of the external detection API. A stream's **Custom API Endpoint**
  overrides it for that stream. Query parameters in either URL are passed
  through to the server.
- `format`: Wire format used when posting snapshots: `light-object-detect`
  (default) or `doods2`. Both are described below.
- `backend`: `light-object-detect` only. Inference backend requested from the
  server: `onnx` (YOLOv8 - best accuracy), `tflite`, or `opencv`.
- `detector_name`: `doods2` only. The `detector_name` sent with each request;
  it must match one of the detectors the server lists at `GET /detectors`.
- `detection_threshold`: Default confidence threshold (0-100%) for streams that
  do not set their own.
- `pre_detection_buffer`: Default seconds of video kept before a detection for streams that do not set their own (default: 5, range 0-60; the ring buffer holds at most 30 seconds, so larger values behave as 30). See [Pre- and Post-Event Buffering](MOTION_BUFFER.md).
- `post_detection_buffer`: Default seconds recorded after the last detection (default: 10, range 0-300).

### Detection Behaviour

```ini
[detection]
grace_period = 2
```

- `grace_period`: Seconds after the last detection before the post-detection buffer starts counting down, so brief gaps in detections do not split one event into several (default: 2, range 0-60). Applies to every detection backend.

### In-process LiteRT Engine

Available in builds configured with `-DENABLE_LITERT=ON` (see [BUILD.md](BUILD.md)). Streams whose model path ends in `.tflite` run through this engine instead of an external API. Labels are read from a `<model>.labels.txt` file next to the model.

```ini
[detection_engine]
enabled = false
threads = 1
delegate = xnnpack
```

- `enabled`: Enable the in-process engine (default: false)
- `threads`: Interpreter threads (default: 1, range 1-16)
- `delegate`: `xnnpack` (default), `gpu` (requires `-DLITERT_WITH_GPU=ON`) or `none`. Unknown values fall back to `xnnpack`.

A stream whose only detection engine is an `api` engine may override `format`,
`backend` and `detector_name` in that engine's `config` object via
`PUT /api/streams/{name}/detection-engines`; see
[Detection Engines](DETECTION_ENGINES.md).

#### Wire formats

LightNVR grabs one JPEG per detection interval and posts it to the API. It
never asks the detector to open the camera stream itself, so streaming request
modes offered by some servers are not used.

**light-object-detect** (default)

Request: `POST <url>?backend=<backend>&confidence_threshold=<0-1>&return_image=false`
as `multipart/form-data` with a single `file` part containing the JPEG.

Response:

```json
{"detections": [
  {"label": "person", "confidence": 0.91,
   "x_min": 0.10, "y_min": 0.20, "x_max": 0.40, "y_max": 0.80}
]}
```

Coordinates are normalized 0-1 and `confidence` is 0-1. The box may instead be
nested under a `bounding_box` object. Optional `track_id` (number) and
`zone_id` (string) are stored when present. Any server that implements this
contract works with the default format.

**doods2**

Request: `POST <url>` with `Content-Type: application/json`:

```json
{"id": "<stream name>", "detector_name": "<detector_name>",
 "detect": {"*": <threshold x 100>}, "data": "<base64 JPEG>"}
```

Response: the standard [DOODS2](https://github.com/snowzach/doods2) detect
response. `top`/`left`/`bottom`/`right` are normalized 0-1 and `confidence` is
0-100; LightNVR converts both to its internal 0-1 scale. A non-empty `error`
field fails the request.

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
- `binary_path`: go2rtc executable (default: `/usr/local/bin/go2rtc`, set at build time by `-DGO2RTC_BINARY_PATH`)
- `config_dir`: Directory for the generated go2rtc configuration (default: `/etc/lightnvr/go2rtc`)
- `api_port`: go2rtc API port (default: 1984). It is bound to loopback; never publish it.
- `rtsp_port`: go2rtc RTSP restream port (default: 8554), also bound to loopback
- `force_native_hls`: Make the web UI use LightNVR's FFmpeg-based HLS instead of go2rtc's, even while go2rtc is running (default: false)
- `turn_enabled`, `turn_server_url`, `turn_username`, `turn_password`: Relay WebRTC through a TURN server when direct and STUN-assisted connections fail, for example for remote viewers behind symmetric NAT (default: disabled)
- `webrtc_connection_timeout_ms`, `webrtc_ice_recovery_timeout_ms`: Browser-side WebRTC timeouts; see [Troubleshooting](TROUBLESHOOTING.md)

To change go2rtc's API prefix, set **Config Override** in the go2rtc settings:

```yaml
api:
  base_path: /foo
```

LightNVR reads the prefix from the applied override and uses it for API calls,
health checks, and direct browser playback. Use an absolute path without a
trailing slash; `base_path: ""` serves the API at the root. Clearing or
quarantining the override restores `/go2rtc`. Automatic prefix detection requires
libyaml, which is included in the standard builds.

LightNVR's built-in HTTP proxy keeps its public `/go2rtc/*` routes and translates
requests to the configured upstream prefix. For HTTPS playback through an external
reverse proxy, forward browser requests (including WebSocket upgrades) to
LightNVR, which performs authorization before contacting go2rtc.

#### Media authority deployment requirements

Use the maintained OpenSensor go2rtc fork shipped with this LightNVR version.
Its media authority protocol enforces camera access and separate `audio.listen`
and `audio.talk` permissions for WebRTC, MSE, and go2rtc HLS. Older binaries and
upstream builds without this protocol fail closed for these playback requests.

The fork renews an opaque media lease through LightNVR at `127.0.0.1` on
`web.port`, using LightNVR's HTTP or HTTPS mode. **LightNVR's web listener must be
reachable on IPv4 loopback in the same network namespace as go2rtc.** The default
`web.bind_ip = 0.0.0.0` satisfies this requirement; `127.0.0.1` also works behind
a reverse proxy in that namespace. Binding only to a specific non-loopback
address, or placing go2rtc in a separate container network namespace, prevents
renewal: new media requests are denied and established media is closed. LightNVR
does not change your bind setting automatically. Keep `/etc/lightnvr` as the
default configuration directory.

For an independently managed go2rtc service, use the compatible fork, keep its
API and RTSP listeners on loopback, and set `api.media_authority_port` to
LightNVR's `web.port`. Both processes must share loopback reachability as above.
LightNVR supplies this setting after overrides for children it launches; a saved
override cannot disable it. Separate-host go2rtc deployment is not supported by
this local authority protocol.

Established fork media checks permission every five seconds with a two-second
callback timeout, giving a seven-second revocation bound when the services are
scheduled normally. A failed callback also closes media. Native `/hls/...`
requests instead use the existing authorization cache, which can retain access
for up to 30 seconds. Previously delivered or player-buffered media cannot be
recalled. See [media access security](../SECURITY.md#media-access-and-revocation)
for the scope of these guarantees.

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
- `ha_discovery`: Publish Home Assistant MQTT discovery messages, so each camera appears in Home Assistant with its detection sensors (default: false; `ha_discovery_enabled` is accepted as an alias)
- `ha_discovery_prefix`: Home Assistant discovery prefix (default: `homeassistant`)
- `ha_snapshot_interval`: Seconds between JPEG snapshots published for each camera's Home Assistant camera entity (default: 30, 0 disables)

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
- `--verbose`: Log at debug level, overriding `log_level`
- `--generate-go2rtc-config`: Write `go2rtc.yaml` from the saved settings and exit (the container entrypoint uses this before starting go2rtc)
- `-h, --help`: Show help message
- `-v, --version`: Print the version and exit

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
