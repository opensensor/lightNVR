#!/bin/sh
set -e

# LightNVR Docker Entrypoint Script
# This script handles initialization and setup for the LightNVR container

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log_info() {
    echo "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo "${RED}[ERROR]${NC} $1"
}

report_unwritable() {
    owner=$(stat -c 'owner %u:%g, mode %a' "$1" 2>/dev/null || echo "missing")
    log_error "Not writable by UID:GID $(id -u):$(id -g): $1 ($owner)"
    permission_errors=$((permission_errors + 1))
}

# Verify every path the entrypoint and LightNVR write to. All problems are
# collected first so the operator can fix them in one pass. Mounted operator
# files are never chowned or chmodded here; in particular, 0600 credentials
# and go2rtc overrides keep their permissions.
check_volume_permissions() {
    log_info "Checking volume permissions for UID:GID $(id -u):$(id -g)..."
    permission_errors=0
    recordings=/var/lib/lightnvr/data/recordings

    for dir in \
        "/etc/lightnvr" \
        "/etc/lightnvr/go2rtc" \
        "/var/lib/lightnvr/data" \
        "$recordings" \
        "$recordings/mp4" \
        "/var/lib/lightnvr/data/database" \
        "/var/lib/lightnvr/data/models" \
        "/var/log/lightnvr"; do
        test_file="$dir/.write_test_$$"
        if [ -d "$dir" ] && touch "$test_file" 2>/dev/null; then
            rm -f "$test_file"
        else
            report_unwritable "$dir"
        fi
    done

    # SQLite writes the database and its WAL/SHM files in place.
    for file in \
        "/var/lib/lightnvr/data/database/lightnvr.db" \
        "/var/lib/lightnvr/data/database/lightnvr.db-wal" \
        "/var/lib/lightnvr/data/database/lightnvr.db-shm"; do
        if [ -e "$file" ] && [ ! -w "$file" ]; then
            report_unwritable "$file"
        fi
    done

    # Per-camera directories created by root-run releases before 0.44 would
    # let LightNVR start but break new segments and retention later. This is
    # bounded by camera count, not by recording file count (see #368).
    for parent in "$recordings" "$recordings/mp4"; do
        [ -d "$parent" ] || continue
        unwritable=$(find "$parent" -mindepth 1 -maxdepth 1 -type d \
            ! -path "$recordings/mp4" ! -writable 2>/dev/null)
        if [ -n "$unwritable" ]; then
            count=$(printf '%s\n' "$unwritable" | wc -l)
            report_unwritable "$(printf '%s\n' "$unwritable" | head -n 1)"
            if [ "$count" -gt 1 ]; then
                log_error "...and $((count - 1)) more camera directories under $parent"
            fi
        fi
    done

    # A read-only lightnvr.ini is a valid choice for managed deployments, but
    # settings saved from the web UI will not persist.
    if [ -e /etc/lightnvr/lightnvr.ini ] && [ ! -w /etc/lightnvr/lightnvr.ini ]; then
        log_warn "/etc/lightnvr/lightnvr.ini is read-only; settings saved from the web UI will not persist"
    fi

    if [ "$permission_errors" -gt 0 ]; then
        log_error ""
        log_error "LightNVR runs as UID:GID $(id -u):$(id -g) and cannot change ownership of existing volumes."
        log_error "Releases before 0.44 ran as root, so their files are usually owned by root."
        log_error "Stop the container, back up its data, and run on the host for each LightNVR bind mount:"
        log_error "  chown -R $(id -u):$(id -g) <host directory>"
        log_error "Deleting the host directory does not help: Docker recreates it owned by root."
        log_error "See docs/DOCKER.md, 'Security migration: non-root runtime and private go2rtc API':"
        log_error "  https://github.com/opensensor/lightNVR/blob/main/docs/DOCKER.md#security-migration-non-root-runtime-and-private-go2rtc-api"
        exit 1
    fi
    log_info "Volume permissions OK"
}

# Function to copy default files if they don't exist
init_config() {
    log_info "Initializing LightNVR configuration..."
    
    # Create directories with error handling for NFS volumes
    log_info "Creating directory structure..."

    # Create directories one by one with error checking
    for dir in \
        "/etc/lightnvr" \
        "/etc/lightnvr/go2rtc" \
        "/var/lib/lightnvr/data" \
        "/var/lib/lightnvr/data/recordings" \
        "/var/lib/lightnvr/data/recordings/mp4" \
        "/var/lib/lightnvr/data/database" \
        "/var/lib/lightnvr/data/models" \
        "/var/log/lightnvr"; do

        if [ ! -d "$dir" ]; then
            if mkdir -p "$dir" 2>/dev/null; then
                log_info "Created directory: $dir"
            else
                log_warn "Could not create directory: $dir (may already exist or be on NFS)"
            fi
        else
            log_info "Directory already exists: $dir"
        fi
    done

    # Fail before writing anything, so an ownership problem is reported as
    # one actionable message instead of a bare shell error (#672).
    check_volume_permissions

    # Verify web assets exist at the expected location; no fallback copy
    log_info "Verifying web assets at /var/lib/lightnvr/www"
    if [ ! -f /var/lib/lightnvr/www/index.html ]; then
        log_error "Web assets not found at /var/lib/lightnvr/www (missing index.html). The image must include built assets in this location."
        exit 1
    else
        log_info "Web assets present."
    fi

    # Create default config if it doesn't exist
    if [ ! -f /etc/lightnvr/lightnvr.ini ]; then
        log_info "Creating default configuration file..."
        cat > /etc/lightnvr/lightnvr.ini << 'EOF'
; LightNVR Configuration File (INI format)

[general]
pid_file = /var/run/lightnvr/lightnvr.pid
log_file = /var/log/lightnvr/lightnvr.log
log_level = 2  ; 0=ERROR, 1=WARN, 2=INFO, 3=DEBUG
syslog_enabled = false
syslog_ident = lightnvr
syslog_facility = LOG_USER

[health]
enabled = true
profile = balanced
fast_interval_seconds = 10
normal_interval_seconds = 60
slow_interval_seconds = 300
device_interval_seconds = 900
write_probe_enabled = true
hardware_provider = auto
presence_interval_seconds = 60
incident_retention_days = 90

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

[database]
path = /var/lib/lightnvr/data/database/lightnvr.db
; startup_check: consistency check run against an existing database at boot.
;   quick           - PRAGMA quick_check: page-level structural validation
;   full            - PRAGMA integrity_check: also cross-checks every index
;                     against its table. Cost scales with total index size and
;                     can take minutes on a large database -- minutes with no
;                     HTTP listener bound, which a proxy reports as a gateway
;                     error. Prefer running it from maintenance, not at boot.
;   off (default)   - skip the boot check; run it during maintenance instead
; startup_check = off
; backup_retention_count = 6  ; timestamped backups to keep (each is a full copy)

[web]
port = 8080
bind_ip = 0.0.0.0
root = /var/lib/lightnvr/www
auth_enabled = true
username = admin
password = admin
; web_thread_pool_size: UV_THREADPOOL_SIZE. Default is 2x online CPU cores (clamped [2,128]).
; Uncomment to override. Requires restart.
; web_thread_pool_size = 8

[streams]
; max_streams: runtime stream slot limit (default 32, ceiling 1024). Requires restart.
max_streams = ${MAX_STREAMS:-32}

[models]
path = /var/lib/lightnvr/data/models

[api_detection]
url = http://localhost:9001/detect

[memory]
buffer_size = 1024
use_swap = true
swap_file = /var/lib/lightnvr/data/swap
swap_size = 134217728

[hardware]
hw_accel_enabled = false
hw_accel_device =

[go2rtc]
; go2rtc binary and configuration paths
binary_path = /bin/go2rtc
config_dir = /etc/lightnvr/go2rtc
api_port = 1984

; WebRTC configuration for NAT/firewall traversal
webrtc_enabled = true
webrtc_ice_servers = stun:stun.l.google.com:19302

[mqtt]
; MQTT settings for detection event streaming
; Enable MQTT publishing of detection events (default: false)
enabled = false
; MQTT broker hostname or IP address
broker_host = localhost
; MQTT broker port (default: 1883, use 8883 for TLS)
broker_port = 1883
; MQTT authentication (optional)
; username =
; password =
; MQTT client ID (default: lightnvr)
client_id = lightnvr
; Topic prefix for detection events (default: lightnvr)
topic_prefix = lightnvr
; Enable TLS for MQTT connection (default: false)
tls_enabled = false
; MQTT keepalive interval in seconds (default: 60)
keepalive = 60
; MQTT QoS level: 0 (at most once), 1 (at least once), 2 (exactly once)
qos = 1
; Retain detection messages on the broker (default: false)
retain = false

[onvif]
; ONVIF camera discovery settings
discovery_enabled = false
discovery_interval = 300
; For containers, use LIGHTNVR_ONVIF_NETWORK environment variable instead
discovery_network = auto
EOF
        log_info "Default configuration created at /etc/lightnvr/lightnvr.ini"
    else
        log_info "Configuration file already exists at /etc/lightnvr/lightnvr.ini"
    fi
    
    # Always regenerate go2rtc.yaml fresh at startup to avoid stale/corrupted
    # configs from prior versions causing stream errors (see issue #165).
    # The entrypoint will then ask LightNVR to rewrite this file from the saved
    # runtime settings before go2rtc itself is launched.
    #
    # Note: any user override at /etc/lightnvr/go2rtc/override.yaml (and its
    # quarantined counterpart) is deliberately preserved — go2rtc is launched
    # with `--config go2rtc.yaml --config override.yaml` and merges both (#394);
    # the override is user-owned and must survive container recreates.
    if [ -f /etc/lightnvr/go2rtc/go2rtc.yaml ]; then
        log_info "Removing old go2rtc configuration to regenerate fresh..."
        rm -f /etc/lightnvr/go2rtc/go2rtc.yaml
    fi
    log_info "Creating default go2rtc configuration..."
    cat > /etc/lightnvr/go2rtc/go2rtc.yaml << 'EOF'
# go2rtc configuration file
api:
  listen: 127.0.0.1:1984
  base_path: /go2rtc

rtsp:
  listen: :8554

webrtc:
  listen: :8555
  ice_servers:
    - urls: [stun:stun.l.google.com:19302]
  candidates:
    - "*:8555"
    - stun:stun.l.google.com:19302

log:
  level: info

streams:
  # Streams will be added dynamically by LightNVR
EOF
    log_info "Default go2rtc configuration created at /etc/lightnvr/go2rtc/go2rtc.yaml"

    # Set up models if needed
    if [ -d /usr/share/lightnvr/models ] && [ -n "$(ls -A /usr/share/lightnvr/models 2>/dev/null)" ]; then
        if [ -z "$(ls -A /var/lib/lightnvr/data/models 2>/dev/null)" ]; then
            log_info "Copying default models..."
            cp -r /usr/share/lightnvr/models/* /var/lib/lightnvr/data/models/ 2>/dev/null || log_warn "No model files to copy"
            log_info "Models setup complete"
        fi
    else
        log_info "No models to copy, skipping model initialization"
    fi

    log_info "Initialization complete"
}

# Function to handle go2rtc config persistence
setup_go2rtc_config() {
    # If user wants persistent go2rtc config and it exists, ensure it's used
    if [ "${GO2RTC_CONFIG_PERSIST:-true}" = "true" ] && [ -f /etc/lightnvr/go2rtc/go2rtc.yaml ]; then
        log_info "Using persistent go2rtc configuration from /etc/lightnvr/go2rtc/go2rtc.yaml"
        
        # Create symlink in /dev/shm if needed (some systems expect it there)
        if [ ! -f /dev/shm/go2rtc.yaml ]; then
            ln -sf /etc/lightnvr/go2rtc/go2rtc.yaml /dev/shm/go2rtc.yaml 2>/dev/null || true
        fi
    else
        log_info "go2rtc will generate its own configuration"
    fi
}

generate_go2rtc_config_from_settings() {
    if [ ! -x /bin/lightnvr ]; then
        log_warn "Cannot regenerate go2rtc configuration from saved settings: /bin/lightnvr not found"
        return 0
    fi

    log_info "Regenerating go2rtc configuration from saved LightNVR settings..."
    if /bin/lightnvr --generate-go2rtc-config -c /etc/lightnvr/lightnvr.ini; then
        log_info "go2rtc configuration regenerated from saved settings"
    else
        log_warn "Failed to regenerate go2rtc configuration from saved settings; continuing with default config"
    fi
}

# Remove any stale PID file left over from a previous container run.
# When a container is stopped and restarted (without being recreated) the
# container filesystem is preserved, so /var/run/lightnvr/lightnvr.pid may still exist
# with the PID from the old run.  In the new container the PID namespace
# restarts from 1, meaning that old PID can be reused by an unrelated process
# (e.g. go2rtc or a shell).  If LightNVR then finds that PID it would send
# SIGTERM to the wrong process, crashing the container in a restart loop.
# Removing the stale file here, before LightNVR starts, prevents that.
cleanup_stale_pid_file() {
    # Determine the PID file path.  Try to read it from the config; fall back
    # to the container default of /var/run/lightnvr/lightnvr.pid.
    local pid_file="/var/run/lightnvr/lightnvr.pid"
    if [ -f /etc/lightnvr/lightnvr.ini ]; then
        local cfg_val
        cfg_val=$(grep -E '^\s*pid_file\s*=' /etc/lightnvr/lightnvr.ini \
                  | sed 's/[^=]*=[ \t]*//' | sed 's/[ \t]*[;#].*//' \
                  | sed 's/^[[:space:]]*//;s/[[:space:]]*$//' | head -1)
        if [ -n "$cfg_val" ]; then
            pid_file="$cfg_val"
        fi
    fi

    if [ -f "$pid_file" ]; then
        log_info "Removing stale PID file from previous container run: $pid_file"
        rm -f "$pid_file" || log_warn "Could not remove stale PID file: $pid_file"
    fi
}

# Function to display startup information
display_startup_info() {
    echo ""
    log_info "=========================================="
    log_info "LightNVR Container Starting"
    log_info "=========================================="
    log_info "Web UI: http://localhost:8080"
    log_info "go2rtc API: http://localhost:1984"
    log_info "RTSP Port: 8554"
    log_info "WebRTC Port: 8555 (TCP/UDP)"
    log_info ""
    log_info "Default credentials:"
    log_info "  Username: admin"
    log_info "  Password: admin"
    log_info ""
    log_info "Volume mounts:"
    log_info "  Config: /etc/lightnvr"
    log_info "  Data: /var/lib/lightnvr/data"
    log_info "  Recordings: /var/lib/lightnvr/data/recordings"
    log_info "=========================================="
    echo ""
}

# Optional migration for volumes written by releases before 0.44, which ran
# as root. The image runs as 10001:10001 and this path is never taken by
# default: only when the operator explicitly starts the container as root
# (user: "0:0") does the entrypoint fix volume ownership, then re-run itself
# as PUID:PGID before doing anything else. LightNVR and go2rtc never run as
# root. Ownership is changed only where it differs, and modes are kept.
# shellcheck disable=SC2086 # $mismatch is a deliberate list of find operators
chown_volumes() {
    data=/var/lib/lightnvr/data
    mismatch="( ! -user $1 -o ! -group $2 )"

    mkdir -p /etc/lightnvr/go2rtc "$data/recordings/mp4" "$data/database" \
        "$data/models" /var/log/lightnvr /var/run/lightnvr &&
    # Config, logs, database, models and other small state: recursively.
    find /etc/lightnvr /var/log/lightnvr /var/run/lightnvr $mismatch \
        -exec chown -h "$1:$2" {} + &&
    find "$data" -path "$data/recordings" -prune -o $mismatch \
        -exec chown -h "$1:$2" {} + &&
    # Recordings: directories only, so startup does not walk every segment
    # (#368). Old root-owned segments stay readable and, in a writable
    # directory, deletable by retention.
    find "$data/recordings" -type d $mismatch -exec chown -h "$1:$2" {} +
}

migrate_volume_ownership() {
    log_warn "Started as root: migrating LightNVR volume ownership to UID:GID $1:$2"
    if ! chown_volumes "$1" "$2"; then
        log_error "Could not change volume ownership. The migration run needs the CHOWN,"
        log_error "DAC_OVERRIDE, FOWNER, SETUID and SETGID capabilities (do not use cap_drop: ALL"
        log_error "for it), and NFS exports with root_squash must be changed on the server instead."
        exit 1
    fi
    log_info "Volume ownership migrated; remove user: \"0:0\" to start without root next time"
}

if [ "$(id -u)" = "0" ]; then
    runtime_uid="${PUID:-10001}"
    runtime_gid="${PGID:-10001}"
    for id_value in "$runtime_uid" "$runtime_gid"; do
        case "$id_value" in
            ''|*[!0-9]*|0)
                log_error "PUID and PGID must be non-zero numeric IDs; LightNVR does not run as root"
                exit 1
                ;;
        esac
    done

    migrate_volume_ownership "$runtime_uid" "$runtime_gid"

    # Keep supplementary groups added by the runtime (e.g. group_add for
    # /dev/dri) but never root's group.
    extra_groups=$(id -G | tr ' ' '\n' | grep -vx -e 0 -e "$runtime_gid" | paste -sd, -)
    if [ -n "$extra_groups" ]; then
        group_opt="--groups=$extra_groups"
    else
        group_opt="--clear-groups"
    fi
    exec setpriv --reuid="$runtime_uid" --regid="$runtime_gid" "$group_opt" -- "$0" "$@"
fi

# Handle signals properly for graceful shutdown
trap 'log_info "Received shutdown signal, stopping services..."; kill -TERM $PID 2>/dev/null || true; exit 0' TERM INT

# Initialize configuration
init_config

# Setup go2rtc configuration
setup_go2rtc_config

# Remove any stale PID file before starting LightNVR
cleanup_stale_pid_file

# Regenerate go2rtc.yaml from saved LightNVR settings before go2rtc starts
generate_go2rtc_config_from_settings

# Display startup information
display_startup_info

# Start LightNVR
log_info "Starting LightNVR..."

# Execute the command passed to the entrypoint
exec "$@" &
PID=$!

# Wait for the process
wait $PID
