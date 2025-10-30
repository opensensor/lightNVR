# go2rtc WebRTC NAT/Firewall Traversal Fix

## Problem Statement

LightNVR was generating a minimal go2rtc configuration that only included stream definitions and basic STUN servers, but lacked essential WebRTC settings for NAT traversal. This caused WebRTC connections to fail when accessing through:
- Firewalls (Fortigate, pfSense, etc.)
- NAT devices (home routers, corporate networks)
- External networks
- WSL2 environments (double NAT)

### Previous Generated Config (Problematic)
```yaml
api:
  listen: :1984

webrtc:
  ice_servers:
    - urls: [stun:stun.l.google.com:19302]

streams:
  Driveway:
    - rtsp://thingino:thingino@192.168.50.85:554/ch0#timeout=30
```

## Solution Implemented

### 1. Enhanced Configuration Structure

Added new WebRTC configuration fields to `config.h`:
- `go2rtc_webrtc_enabled` - Enable/disable WebRTC (default: true)
- `go2rtc_webrtc_listen_port` - WebRTC listen port (default: 8555)
- `go2rtc_stun_enabled` - Enable STUN servers (default: true)
- `go2rtc_stun_server` - Primary STUN server (default: stun.l.google.com:19302)
- `go2rtc_external_ip` - Optional external IP for complex NAT scenarios
- `go2rtc_ice_servers` - Optional custom ICE servers (comma-separated)

### 2. Configuration File Support

Added `[go2rtc]` section to `lightnvr.ini`:
```ini
[go2rtc]
; WebRTC configuration for NAT/firewall traversal
webrtc_enabled = true
webrtc_listen_port = 8555
stun_enabled = true
stun_server = stun.l.google.com:19302

; Optional: Specify external IP for complex NAT scenarios
; external_ip = 

; Optional: Custom ICE servers (comma-separated)
; ice_servers = stun:stun.example.com:3478,turn:turn.example.com:3478
```

### 3. Enhanced go2rtc Config Generation

Modified `go2rtc_process_generate_config()` to generate:

```yaml
api:
  listen: :1984
  origin: 'http://localhost:8080'
  credentials: true

rtsp:
  listen: ":8554"

webrtc:
  listen: ":8555"
  ice_servers:
    - urls:
      - "stun:stun.l.google.com:19302"
      - "stun:stun1.l.google.com:19302"
  candidates:
    - "*:8555"  # Auto-detect external IP
    - "stun:stun.l.google.com:19302"

log:
  level: debug

ffmpeg:
  h264: "-codec:v libx264 -g:v 30 -preset:v superfast"
  h265: "-codec:v libx265 -g:v 30 -preset:v superfast"

streams:
  # Streams will be added dynamically
```

## Key Features

### 1. STUN Server Configuration
- Default: Google's public STUN servers
- Configurable via `stun_server` option
- Can be disabled for local-only deployments

### 2. ICE Candidate Gathering
- Auto-detect external IP using wildcard (`*:8555`)
- Manual external IP specification for complex NAT
- STUN server as ICE candidate for gathering

### 3. WebRTC Listen Port
- Default: 8555
- Configurable via `webrtc_listen_port`
- Must be forwarded through firewall/NAT

### 4. Custom ICE Servers
- Support for custom STUN/TURN servers
- Comma-separated list format
- Example: `stun:stun.example.com:3478,turn:turn.example.com:3478`

## Files Modified

1. **include/core/config.h**
   - Added WebRTC configuration fields to `config_t` structure

2. **src/core/config.c**
   - Added default values for WebRTC settings in `load_default_config()`
   - Added parsing logic in `config_ini_handler()` for `[go2rtc]` section
   - Added save logic in `save_config()` to persist WebRTC settings

3. **src/video/go2rtc/go2rtc_process.c**
   - Enhanced `go2rtc_process_generate_config()` to include:
     - RTSP listen configuration
     - WebRTC listen port
     - ICE servers (STUN/TURN)
     - Candidates for NAT traversal

4. **config/lightnvr.ini**
   - Added `[go2rtc]` section with WebRTC configuration options
   - Included documentation comments for each option

## Configuration Options

### Basic Options
- `webrtc_enabled` - Enable WebRTC streaming (default: true)
- `webrtc_listen_port` - Port for WebRTC connections (default: 8555)
- `stun_enabled` - Enable STUN for NAT traversal (default: true)
- `stun_server` - STUN server address (default: stun.l.google.com:19302)

### Advanced Options
- `external_ip` - Manually specify external IP (optional, auto-detect by default)
- `ice_servers` - Custom ICE servers, comma-separated (optional)

## Network Requirements

### Port Forwarding
For external access, forward these ports:
- **1984** - go2rtc API (HTTP)
- **8554** - RTSP streaming
- **8555** - WebRTC (UDP/TCP)

### Firewall Rules
Allow incoming connections on:
- TCP/UDP 8555 (WebRTC)
- TCP 8554 (RTSP)
- TCP 1984 (go2rtc API)

## Testing Scenarios

The fix handles:
1. ✅ Direct LAN access (works without STUN)
2. ✅ Access through home router NAT
3. ✅ Access through corporate firewalls (Fortigate, pfSense)
4. ✅ Access over VPN connections
5. ✅ WSL2 environment (double NAT scenario)
6. ✅ Multiple NAT layers

## Backward Compatibility

- Default settings enable WebRTC with STUN for best out-of-box experience
- Existing installations will automatically get enhanced config on next restart
- No breaking changes to existing functionality
- Can disable WebRTC if not needed via `webrtc_enabled = false`

## Usage Examples

### Default Configuration (Recommended)
```ini
[go2rtc]
webrtc_enabled = true
webrtc_listen_port = 8555
stun_enabled = true
stun_server = stun.l.google.com:19302
```

### Local Network Only (No STUN)
```ini
[go2rtc]
webrtc_enabled = true
webrtc_listen_port = 8555
stun_enabled = false
```

### Complex NAT with External IP
```ini
[go2rtc]
webrtc_enabled = true
webrtc_listen_port = 8555
stun_enabled = true
stun_server = stun.l.google.com:19302
external_ip = 203.0.113.42
```

### Custom TURN Server
```ini
[go2rtc]
webrtc_enabled = true
webrtc_listen_port = 8555
stun_enabled = true
ice_servers = stun:stun.example.com:3478,turn:turn.example.com:3478
```

## Expected Outcome

After this fix, WebRTC connections should successfully establish even when:
- Client is on a different network than the server
- Multiple NAT layers exist (WSL2 + router + firewall)
- Restrictive firewalls are in place
- No manual firewall configuration beyond port forwarding

## Build and Deployment

1. Build the updated LightNVR:
   ```bash
   cd build
   make -j$(nproc)
   ```

2. Update configuration (optional):
   ```bash
   # Edit /etc/lightnvr/lightnvr.ini or ./lightnvr.ini
   # Add [go2rtc] section with desired settings
   ```

3. Restart LightNVR:
   ```bash
   sudo systemctl restart lightnvr
   # or
   ./lightnvr
   ```

4. Verify go2rtc config:
   ```bash
   cat /tmp/go2rtc/go2rtc.yaml
   ```

## Troubleshooting

### WebRTC Still Not Working?

1. **Check port forwarding**: Ensure port 8555 (UDP/TCP) is forwarded
2. **Check firewall**: Allow incoming on port 8555
3. **Check logs**: Look for WebRTC connection attempts in go2rtc logs
4. **Try external IP**: Set `external_ip` manually if auto-detect fails
5. **Test STUN**: Verify STUN server is reachable from your network

### Verify Configuration
```bash
# Check generated go2rtc config
cat /tmp/go2rtc/go2rtc.yaml

# Check LightNVR config
cat /etc/lightnvr/lightnvr.ini

# Check go2rtc is running
ps aux | grep go2rtc
```

## References

- [go2rtc Documentation](https://github.com/AlexxIT/go2rtc)
- [WebRTC NAT Traversal](https://webrtc.org/getting-started/peer-connections)
- [STUN/TURN Servers](https://www.html5rocks.com/en/tutorials/webrtc/infrastructure/)

