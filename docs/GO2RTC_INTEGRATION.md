# go2rtc Integration

This document describes the integration of [go2rtc](https://github.com/AlexxIT/go2rtc) with LightNVR to provide WebRTC streaming capabilities. Note that this is WebRTC (Real-Time Communication) streaming, not WebSocket communication.

## Overview

go2rtc is a lightweight, high-performance WebRTC server that can be used to stream video from various sources, including RTSP cameras. The integration with LightNVR allows for:

- WebRTC streaming of camera feeds with low latency
- Maintaining existing HLS and MP4 recording functionality
- Seamless management of go2rtc alongside the main LightNVR application

## Architecture

The integration consists of three main components:

1. **Process Management**: Handles the lifecycle of the go2rtc process
2. **API Communication**: Provides an interface to interact with go2rtc's HTTP API
3. **Stream Integration**: Bridges LightNVR's stream management with go2rtc

```
┌─────────────────────────────────────────────────────────────┐
│                        LightNVR                             │
│                                                             │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────────┐  │
│  │   Existing  │    │   go2rtc    │    │    go2rtc       │  │
│  │   Stream    │───▶│   Stream    │───▶│    Process      │  │
│  │   Manager   │    │ Integration │    │    Management   │  │
│  └─────────────┘    └─────────────┘    └─────────────────┘  │
│         │                  │                   │            │
│         ▼                  ▼                   ▼            │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────────┐  │
│  │    HLS      │    │   go2rtc    │    │    go2rtc       │  │
│  │  Streaming  │    │     API     │───▶│    Process      │  │
│  │             │    │ Communication│    │                 │  │
│  └─────────────┘    └─────────────┘    └─────────────────┘  │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## Installation

### Prerequisites

- LightNVR installed and configured
- go2rtc binary (can be installed using the provided script)

### Installing go2rtc

A script is provided to download and install go2rtc:

```bash
sudo ./scripts/install_go2rtc.sh
```

By default, this will:
- Download the latest go2rtc binary for your architecture
- Install it to `/usr/local/bin/go2rtc`
- Create a configuration directory at `/etc/lightnvr/go2rtc`
- Generate a basic configuration file

You can customize the installation with options:

```bash
sudo ./scripts/install_go2rtc.sh --install-dir /custom/path --config-dir /custom/config/path
```

Run `./scripts/install_go2rtc.sh --help` for more options.

### Building LightNVR with go2rtc Support

go2rtc integration is enabled by default. You can control it with CMake options:

```bash
# Enable go2rtc integration (default: ON)
-DENABLE_GO2RTC=ON

# Specify go2rtc binary path (default: /usr/local/bin/go2rtc)
-DGO2RTC_BINARY_PATH=/path/to/go2rtc

# Specify go2rtc configuration directory (default: /etc/lightnvr/go2rtc)
-DGO2RTC_CONFIG_DIR=/path/to/config/dir

# Specify go2rtc API port (default: 1984)
-DGO2RTC_API_PORT=1984
```

Example:

```bash
mkdir build && cd build
cmake .. -DENABLE_GO2RTC=ON -DGO2RTC_BINARY_PATH=/usr/local/bin/go2rtc
make
```

## Usage

### Automatic Stream Registration

When go2rtc integration is enabled, LightNVR will automatically:

1. Start the go2rtc process when needed
2. Register streams with go2rtc when they are added to LightNVR
3. Unregister streams when they are removed from LightNVR

No additional configuration is required for basic functionality.

### WebRTC URLs

WebRTC stream URLs follow this format:

```
http://[server-address]:[go2rtc-port]/webrtc/[stream-id]
```

For example, if your server is at `192.168.1.100`, go2rtc is running on port `1984`, and your stream ID is `camera1`, the WebRTC URL would be:

```
http://192.168.1.100:1984/webrtc/camera1
```

These URLs can be used in web applications that support WebRTC, including the LightNVR web interface.

## Configuration

### go2rtc Configuration

The go2rtc configuration file is located at `[GO2RTC_CONFIG_DIR]/go2rtc.yaml`. While LightNVR manages the streams section automatically, you can modify other settings such as:

- STUN/TURN servers for WebRTC
- Logging levels
- API settings

Example configuration:

```yaml
api:
  listen: :1984

webrtc:
  ice_servers:
    - urls: [stun:stun.l.google.com:19302]
    # Add TURN servers if needed
    # - urls: [turn:turn.example.com:3478]
    #   username: user
    #   credential: pass

log:
  level: info  # debug, info, warn, error

streams:
  # Streams will be added dynamically by LightNVR
```

### Advanced Configuration

For advanced use cases, you can:

1. Manually edit the go2rtc configuration file
2. Restart the go2rtc process through LightNVR
3. Use go2rtc's API directly for custom configurations

## Health Monitoring

LightNVR includes a health monitoring system for go2rtc:

- **API Health Check**: Periodically checks go2rtc's API on port 1984
- **Stream Monitoring**: Monitors individual stream connections
- **Consensus Logic**: If all streams are down, it's likely a go2rtc issue (not camera issues)
- **Auto-restart**: Automatically restarts go2rtc if it becomes unresponsive

Key files:
- `src/video/go2rtc/go2rtc_health.c`: Health monitoring implementation

## Frame Extraction for Detection

LightNVR uses go2rtc's `frame.jpeg` endpoint for object detection:

```
http://localhost:1984/api/frame.jpeg?src=[stream-name]
```

This approach:
- Avoids FFmpeg decoding overhead
- Uses go2rtc's efficient frame extraction
- Provides JPEG images suitable for detection models
- Reduces memory usage compared to decoding video streams

## Troubleshooting

### Common Issues

1. **go2rtc process fails to start**
   - Check if the go2rtc binary exists at the configured path
   - Verify the binary has execute permissions
   - Check system logs for errors

2. **Streams not appearing in go2rtc**
   - Verify the stream URLs are correct
   - Check if LightNVR can access the streams directly
   - Look for errors in the LightNVR logs

3. **WebRTC streaming not working**
   - Ensure your browser supports WebRTC
   - Check if STUN/TURN servers are properly configured
   - Verify network connectivity between client and server

4. **go2rtc becomes unresponsive**
   - Health monitoring should auto-restart
   - Check go2rtc logs for memory issues
   - Verify camera streams are accessible

### Logs

- go2rtc logs are stored in `[GO2RTC_CONFIG_DIR]/go2rtc.log`
- LightNVR logs contain information about go2rtc integration

## References

- [go2rtc GitHub Repository](https://github.com/AlexxIT/go2rtc)
- [WebRTC Documentation](https://webrtc.org/)
- [LightNVR Documentation](https://github.com/lightnvr/lightnvr)
