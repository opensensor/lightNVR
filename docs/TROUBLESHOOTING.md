# LightNVR Troubleshooting Guide

This document provides solutions for common issues you might encounter when using LightNVR.

## Table of Contents

1. [Installation Issues](#installation-issues)
2. [Startup Problems](#startup-problems)
3. [Stream Connection Issues](#stream-connection-issues)
4. [Recording Problems](#recording-problems)
5. [Web Interface Issues](#web-interface-issues)
6. [Performance Optimization](#performance-optimization)
7. [Memory Usage](#memory-usage)
8. [Log File Analysis](#log-file-analysis)

## Installation Issues

### Missing Dependencies

If you encounter errors about missing dependencies during installation:

```
Error: Required dependency not found: libavcodec
```

Install the required dependencies:

**Debian/Ubuntu:**
```bash
sudo apt-get update
sudo apt-get install -y \
    build-essential \
    cmake \
    pkg-config \
    libsqlite3-dev \
    libavcodec-dev \
    libavformat-dev \
    libavutil-dev \
    libswscale-dev \
    libmicrohttpd-dev \
    libcurl4-openssl-dev \
    libssl-dev
```

**Fedora/RHEL/CentOS:**
```bash
sudo dnf install -y \
    gcc \
    gcc-c++ \
    make \
    cmake \
    pkgconfig \
    sqlite-devel \
    ffmpeg-devel \
    libmicrohttpd-devel \
    libcurl-devel \
    openssl-devel
```

### Build Errors

If you encounter build errors:

1. Make sure all dependencies are installed
2. Clean the build directory and try again:
   ```bash
   ./scripts/build.sh --clean
   ```
3. Check the CMake output for specific error messages

## Startup Problems

### Service Won't Start

If the LightNVR service won't start:

1. Check the systemd status:
   ```bash
   sudo systemctl status lightnvr
   ```

2. Check the log file for error messages:
   ```bash
   sudo tail -f /var/log/lightnvr.log
   ```

3. Verify that the configuration file exists and is valid:
   ```bash
   sudo cat /etc/lightnvr/lightnvr.conf
   ```

4. Check permissions on directories:
   ```bash
   sudo ls -la /var/lib/lightnvr
   sudo ls -la /var/log/lightnvr
   ```

### PID File Issues

If you see errors related to the PID file:

1. Remove the stale PID file:
   ```bash
   sudo rm /var/run/lightnvr.pid
   ```

2. Ensure the directory exists and has the correct permissions:
   ```bash
   sudo mkdir -p /var/run/lightnvr
   sudo chown -R root:root /var/run/lightnvr
   ```

## Stream Connection Issues

### Can't Connect to Camera

If LightNVR can't connect to a camera:

1. Verify that the camera is online and accessible:
   ```bash
   ping camera-ip-address
   ```

2. Test the RTSP URL with another tool like VLC or ffmpeg:
   ```bash
   ffplay rtsp://username:password@camera-ip:554/stream1
   ```

3. Check for authentication issues in the log file:
   ```bash
   grep "authentication" /var/log/lightnvr.log
   ```

4. Verify network connectivity between the LightNVR server and the camera

### Stream Disconnects Frequently

If streams disconnect frequently:

1. Check your network stability
2. Reduce the stream resolution or frame rate in the configuration
3. Verify that the camera isn't overloaded with too many connections
4. Check if the camera has a limit on concurrent RTSP connections

### HLS Buffer Append Errors (bufferAppendError)

If you see `bufferAppendError` messages in the browser console, this typically indicates that HLS segments are not properly formatted or don't start with keyframes. This has been fixed in recent versions by:

1. **Forcing keyframes at segment boundaries** - The HLS writer now forces keyframes (I-frames) at the start of each segment
2. **Increased playlist size** - The `hls_list_size` has been increased from 3 to 6 segments for better buffering
3. **Improved error recovery** - HLS.js retry attempts have been increased

If you still experience issues:

1. **Check camera GOP (Group of Pictures) settings**:
   - Ensure your camera's keyframe interval is set to 2 seconds or less
   - Some cameras call this "I-frame interval" or "GOP size"
   - For a 15 FPS stream, GOP should be 30 frames or less
   - For a 30 FPS stream, GOP should be 60 frames or less

2. **Verify stream codec**:
   - Ensure the camera is sending H.264 video
   - Check that the stream doesn't have B-frames (use baseline or main profile, not high profile)

3. **Check server logs** for HLS writer errors:
   ```bash
   docker logs lightnvr | grep -i "hls"
   ```

### Live Stream Video Timeouts

If you experience frequent video timeouts in the live stream:

1. **Adjust HLS.js buffer settings** in `web/js/components/preact/HLSVideoCell.jsx`:
   ```javascript
   const hls = new Hls({
       maxBufferLength: 60,            // Increase for more buffering (default: 30)
       maxMaxBufferLength: 120,        // Maximum buffer size in seconds (default: 60)
       liveSyncDurationCount: 5,       // Number of segments to sync (default: 3)
       fragLoadingTimeOut: 20000,      // Fragment loading timeout in ms (default: 8000)
       manifestLoadingTimeOut: 15000,  // Manifest loading timeout in ms (default: 10000)
       levelLoadingTimeOut: 15000      // Level loading timeout in ms (default: 10000)
   });
   ```

2. **Increase server-side timeouts** in `src/web/api_handlers_streaming.c`:
   - For manifest files: Increase the number of attempts and/or the wait time between attempts
   - For segment files: Increase the number of attempts and/or the wait time between attempts

3. **Network and hardware considerations**:
   - Ensure your network has sufficient bandwidth for the configured stream quality
   - If running on limited hardware (like Ingenic A1), reduce stream resolution and framerate
   - Consider enabling hardware acceleration if available

### Stale Stream Data in Live View

If you notice the live stream showing outdated video (stale data):

1. **Prevent browser caching**:
   - Add cache control headers to HLS responses in `src/web/api_handlers_streaming.c`:
     ```c
     set_response_header(response, "Cache-Control", "no-cache, no-store, must-revalidate");
     set_response_header(response, "Pragma", "no-cache");
     set_response_header(response, "Expires", "0");
     ```

2. **Add cache-busting to HLS URLs** in `web/js/components/video.js`:
   ```javascript
   const timestamp = Date.now();
   const hlsStreamUrl = `/api/streaming/${encodeURIComponent(stream.name)}/hls/index.m3u8?_t=${timestamp}`;
   ```

3. **Implement periodic stream refresh**:
   ```javascript
   const refreshInterval = 60000; // 60 seconds
   const refreshTimer = setInterval(() => {
       if (videoCell && videoCell.hlsPlayer) {
           const newTimestamp = Date.now();
           const newUrl = `/api/streaming/${encodeURIComponent(stream.name)}/hls/index.m3u8?_t=${newTimestamp}`;
           videoCell.hlsPlayer.loadSource(newUrl);
       }
   }, refreshInterval);
   ```

4. **Manual refresh**: If you still see stale data, refreshing the browser page will force a complete reload of the stream.

## Recording Problems

### Recordings Not Being Created

If recordings aren't being created:

1. Check if the stream is properly connected
2. Verify that recording is enabled for the stream:
   ```bash
   grep "record" /etc/lightnvr/lightnvr.conf
   ```

3. Check permissions on the recordings directory:
   ```bash
   ls -la /var/lib/lightnvr/recordings
   ```

4. Check available disk space:
   ```bash
   df -h
   ```

### Corrupt Recordings

If recordings are corrupt:

1. Check if the stream is stable or frequently disconnecting
2. Verify that the system has enough resources (CPU, memory)
3. Check if the storage device is reliable and not failing
4. Try reducing the recording quality or segment duration

## Web Interface Issues

> **Note:** For detailed web interface troubleshooting, including blank page issues, see [TROUBLESHOOTING_WEB_INTERFACE.md](TROUBLESHOOTING_WEB_INTERFACE.md)

### Blank Page Issue

If you see a blank page with the title "WebRTC View - LightNVR" but no content:

**Quick Fix:**
```bash
# Diagnose the issue
sudo bash scripts/diagnose_web_issue.sh

# Install/reinstall web assets
sudo bash scripts/install_web_assets.sh

# Restart service
sudo systemctl restart lightnvr
```

**Cause:** Web assets (HTML, CSS, JavaScript files) were not installed to `/var/lib/lightnvr/www/`

See [TROUBLESHOOTING_WEB_INTERFACE.md](TROUBLESHOOTING_WEB_INTERFACE.md) for detailed instructions.

### Can't Access Web Interface

If you can't access the web interface:

1. Verify that the LightNVR service is running:
   ```bash
   sudo systemctl status lightnvr
   ```

2. Check if the web server is listening on the configured port:
   ```bash
   sudo netstat -tuln | grep 8080
   ```

3. Check firewall settings:
   ```bash
   sudo iptables -L
   ```

4. Verify that the web root directory exists and has the correct permissions:
   ```bash
   ls -la /var/lib/lightnvr/www
   ```

### Authentication Issues

If you're having trouble with authentication:

1. Reset the username and password in the configuration file:
   ```bash
   sudo nano /etc/lightnvr/lightnvr.conf
   ```

   Update these lines:
   ```
   web_auth_enabled=true
   web_username=admin
   web_password=admin
   ```

2. Restart the service:
   ```bash
   sudo systemctl restart lightnvr
   ```

## Performance Optimization

### High CPU Usage

If LightNVR is using too much CPU:

1. Reduce the number of streams
2. Lower the resolution and frame rate of streams
3. Use hardware acceleration if available:
   ```
   hw_accel_enabled=true
   hw_accel_device=
   ```
4. Reduce the buffer size:
   ```
   buffer_size=512  # Buffer size in KB
   ```

### High Memory Usage

If LightNVR is using too much memory:

1. Reduce the number of streams
2. Lower the resolution and frame rate of streams
3. Reduce the buffer size
4. Enable and configure swap:
   ```
   use_swap=true
   swap_file=/var/lib/lightnvr/swap
   swap_size=134217728  # 128MB in bytes
   ```

## Memory Usage on Ingenic A1

The Ingenic A1 SoC has only 256MB of RAM, so memory optimization is crucial:

1. Limit the number of streams (4-8 maximum)
2. Use lower resolutions (720p or less)
3. Use lower frame rates (5-10 FPS)
4. Set appropriate buffer sizes (512KB or less)
5. Enable swap for additional virtual memory
6. Set stream priorities to ensure important streams get resources

Example configuration for Ingenic A1:

```
# Memory Optimization for Ingenic A1
buffer_size=512  # 512KB buffer size
use_swap=true
swap_file=/var/lib/lightnvr/swap
swap_size=134217728  # 128MB in bytes
max_streams=8

# Stream example with optimized settings
stream.0.name=Front Door
stream.0.url=rtsp://192.168.1.100:554/stream1
stream.0.enabled=true
stream.0.width=1280
stream.0.height=720
stream.0.fps=10
stream.0.codec=h264
stream.0.priority=10
stream.0.record=true
stream.0.segment_duration=900
```

## Log File Analysis

The log file is your best resource for troubleshooting. Here's how to analyze it:

1. View the entire log file:
   ```bash
   cat /var/log/lightnvr.log
   ```

2. View only error messages:
   ```bash
   grep "ERROR" /var/log/lightnvr.log
   ```

3. View only warning messages:
   ```bash
   grep "WARN" /var/log/lightnvr.log
   ```

4. Follow the log file in real-time:
   ```bash
   tail -f /var/log/lightnvr.log
   ```

5. Check for specific issues:
   ```bash
   grep "stream" /var/log/lightnvr.log
   grep "recording" /var/log/lightnvr.log
   grep "database" /var/log/lightnvr.log
   ```

### Common Log Messages

Here are some common log messages and what they mean:

- `Failed to connect to stream`: The RTSP connection to the camera failed
- `Stream disconnected`: The stream connection was lost
- `Failed to create recording directory`: Permission issue or disk full
- `Database error`: Problem with the SQLite database
- `Out of memory`: The system is running out of RAM
- `Swap file created`: Swap file was successfully created
- `Web server started on port 8080`: Web server is running correctly
- `Authentication failed`: Someone tried to access the web interface with incorrect credentials

## Getting Help

If you're still having issues after trying these troubleshooting steps:

1. Check the GitHub repository for known issues
2. Search the issue tracker for similar problems
3. Create a new issue with:
   - A clear description of the problem
   - Steps to reproduce
   - Relevant log file excerpts
   - Your system information (OS, hardware, etc.)
   - Your configuration file (with passwords removed)
