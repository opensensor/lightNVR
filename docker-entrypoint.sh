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

# Function to copy default files if they don't exist
init_config() {
    log_info "Initializing LightNVR configuration..."
    
    # Create directories with error handling for NFS volumes
    log_info "Creating directory structure..."

    # Create directories one by one with error checking
    for dir in \
        "/etc/lightnvr" \
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
pid_file = /var/run/lightnvr.pid
log_file = /var/log/lightnvr/lightnvr.log
log_level = 2  ; 0=ERROR, 1=WARN, 2=INFO, 3=DEBUG
syslog_enabled = false
syslog_ident = lightnvr
syslog_facility = LOG_USER

[storage]
path = /var/lib/lightnvr/data/recordings
max_size = 0  ; 0 means unlimited, otherwise bytes
retention_days = 30
auto_delete_oldest = true
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
password = admin
web_thread_pool_size = 8

[streams]
max_streams = 16

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
    
    # Create go2rtc config directory if it doesn't exist
    mkdir -p /etc/lightnvr/go2rtc
    
    # Create default go2rtc.yaml if it doesn't exist
    # Only create if GO2RTC_CONFIG_PERSIST is true (default)
    if [ ! -f /etc/lightnvr/go2rtc/go2rtc.yaml ] && [ "${GO2RTC_CONFIG_PERSIST:-true}" = "true" ]; then
        log_info "Creating default go2rtc configuration..."
        cat > /etc/lightnvr/go2rtc/go2rtc.yaml << 'EOF'
# go2rtc configuration file
api:
  listen: :1984

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
    else
        if [ -f /etc/lightnvr/go2rtc/go2rtc.yaml ]; then
            log_info "go2rtc configuration already exists, checking for compatibility..."
            # Remove base_path if present - it causes 404 errors when lightNVR communicates with go2rtc
            if grep -q "base_path:" /etc/lightnvr/go2rtc/go2rtc.yaml 2>/dev/null; then
                log_warn "Found base_path in go2rtc config - removing it for compatibility with lightNVR"
                sed -i '/base_path:/d' /etc/lightnvr/go2rtc/go2rtc.yaml
                log_info "Removed base_path from go2rtc configuration"
            fi
        fi
    fi
    
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
    
    # Ensure proper permissions (may fail on NFS, which is okay)
    log_info "Setting permissions..."
    chmod -R 755 /var/lib/lightnvr 2>/dev/null || log_warn "Could not set permissions on /var/lib/lightnvr (may be on NFS)"
    chmod -R 755 /etc/lightnvr 2>/dev/null || log_warn "Could not set permissions on /etc/lightnvr (may be on NFS)"
    chmod -R 755 /var/log/lightnvr 2>/dev/null || log_warn "Could not set permissions on /var/log/lightnvr"

    # Test write permissions on critical directories
    log_info "Testing write permissions..."
    for test_dir in "/var/lib/lightnvr/data" "/etc/lightnvr" "/var/log/lightnvr"; do
        test_file="$test_dir/.write_test_$$"
        if touch "$test_file" 2>/dev/null; then
            rm -f "$test_file" 2>/dev/null
            log_info "Write permission OK: $test_dir"
        else
            log_error "No write permission: $test_dir - This may cause issues!"
        fi
    done

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

# Handle signals properly for graceful shutdown
trap 'log_info "Received shutdown signal, stopping services..."; kill -TERM $PID 2>/dev/null || true; exit 0' TERM INT

# Initialize configuration
init_config

# Setup go2rtc configuration
setup_go2rtc_config

# Display startup information
display_startup_info

# Start LightNVR
log_info "Starting LightNVR..."

# Execute the command passed to the entrypoint
exec "$@" &
PID=$!

# Wait for the process
wait $PID

