# go2rtc Integration

This document describes the integration of [go2rtc](https://github.com/AlexxIT/go2rtc) with LightNVR to provide WebRTC streaming capabilities. Note that this is WebRTC (Real-Time Communication) streaming, not WebSocket communication.

## Overview

go2rtc is a lightweight, high-performance WebRTC server that can be used to stream video from various sources, including RTSP cameras. The integration with LightNVR allows for:

- WebRTC streaming of camera feeds with low latency
- Maintaining existing HLS and MP4 recording functionality
- Seamless management of go2rtc alongside the main LightNVR application

### High-resolution and HEVC live video

The camera's original stream remains the first source for recording and native
playback. When a client needs H.264, LightNVR supplies an on-demand compatibility
producer. Its default software profile fits video within 1920 × 1080, preserves
the aspect ratio, avoids upscaling, and outputs 8-bit YUV 4:2:0 without B-frames.
For example, a 7680 × 2160 panoramic camera produces a 1920 × 540 preview while
its original HEVC stream remains available at 7680 × 2160 for recording.

Decoder, filter and encoder workers are bounded to avoid allocating a frame
buffer for every host CPU. The existing FFmpeg memory guard remains enabled.
When hardware acceleration is enabled, known source dimensions also bound the
hardware preview. A software producer follows it if hardware startup fails.
Unknown substream dimensions are not inferred from the main stream.

WebRTC and MSE first try the client's advertised codecs. If HEVC negotiation
succeeds but video cannot play, the player tries H.264 once before its normal
transport fallback/error handling. Metadata or audio alone no longer counts as
successful video startup. This retry changes only that viewer's connection.

These defaults can be customized with the existing
[go2rtc config override](GO2RTC_CONFIG_OVERRIDE.md) (`ffmpeg.h264`, `ffmpeg.rtsp`)
or a stream source override. Existing custom templates take precedence. A
separately configured H.264 substream is still the cheapest option for a grid
with many cameras; conversion of the main stream is available when a camera
does not provide a usable substream.

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

Browser signaling uses the dashboard origin through the authenticated routes documented in [Reverse Proxy & HTTPS](REVERSE_PROXY.md). WebRTC SDP is posted to `/go2rtc/api/webrtc?src=<stream-id>`; MSE connects to `/go2rtc/api/ws?src=<stream-id>`. Keep port 1984 on loopback.

## Configuration

### go2rtc Configuration

The go2rtc configuration file is located at `[GO2RTC_CONFIG_DIR]/go2rtc.yaml`. While LightNVR manages the streams section automatically, you can modify other settings such as:

- STUN/TURN servers for WebRTC
- Logging levels
- API settings

Example configuration:

```yaml
api:
  listen: 127.0.0.1:1984

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

### Isolating a main-stream playback failure

1. Record the camera model/firmware, browser/OS, hardware acceleration setting,
   and whether the failure affects WebRTC, MSE, or both. Set the stream's Codec
   hint to Auto unless the actual codec has been verified. An RTSP URL containing
   `h264` does not establish the codec returned by the camera.
2. Use `ffprobe -rtsp_transport tcp -show_streams '<CAMERA_MAIN_RTSP_URL>'`
   inside the container to check codec, dimensions, profile and frame rate.
3. Decode the camera directly, then decode its original go2rtc relay. For an HEVC
   stream, run each command for 30 seconds (substitute the actual URL/name/port):

   ```sh
   ffmpeg -hide_banner -loglevel warning -threads:v 2 -rtsp_transport tcp -i '<CAMERA_MAIN_RTSP_URL>' -t 30 -map 0:v:0 -an -f null -
   ffmpeg -hide_banner -loglevel warning -threads:v 2 -rtsp_transport tcp -i 'rtsp://127.0.0.1:8554/<STREAM_NAME>?video=h265' -t 30 -map 0:v:0 -an -f null -
   ```

   These commands decode video; a successful `-c copy` test alone cannot prove
   that frames are decodable. Persistent `PPS changed between slices`, missing
   POC references, or RPS errors on the direct connection require investigation
   of the camera firmware/stream/network. Errors only through the relay narrow
   the investigation to go2rtc. Compare more than the initial keyframe interval.
4. Check `go2rtc.log` for encoder initialization failures or `ffmpeg guard: ...
   killing transcoder` while opening Live View. Share sanitized logs and the
   Settings → Show effective config output, along with both decode results.

The compatibility profile cannot repair an undecodable source bitstream. Its
failure messages distinguish codec mismatch, decode errors, conversion failure,
and resource exhaustion when that information is returned by go2rtc; full
decoder diagnostics remain in the server log.

## References

- [go2rtc GitHub Repository](https://github.com/AlexxIT/go2rtc)
- [WebRTC Documentation](https://webrtc.org/)
- [LightNVR Documentation](https://github.com/lightnvr/lightnvr)
