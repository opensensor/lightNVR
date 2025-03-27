# RTSP to MP4 Recorder

A standalone test program that records an RTSP stream to an MP4 file for a specified duration (default 20 seconds).

This is analogous to:
```
ffplay -i rtsp://thingino:thingino@192.168.50.49:554/ch0 -fflags nobuffer -flags low_delay -framedrop
```
But for recording instead of playing.

## Features

- Records RTSP streams to MP4 files
- Supports both video and audio
- Configurable recording duration
- Low-latency options for better real-time recording
- Handles stream disconnections gracefully
- Proper timestamp handling for smooth playback

## Requirements

- FFmpeg development libraries (libavformat, libavcodec, libavutil, etc.)
- GCC or compatible C compiler
- Make

## Building

To build the program, simply run:

```bash
make
```

This will compile the program and create an executable named `rtsp_recorder`.

## Usage

```
./rtsp_recorder [options]
```

### Options

- `-i, --input URL` : RTSP URL to record (default: rtsp://thingino:thingino@192.168.50.49:554/ch0)
- `-o, --output FILE` : Output MP4 file (default: output.mp4)
- `-d, --duration SEC` : Recording duration in seconds (default: 20)
- `-h, --help` : Show help message

### Examples

Record from the default RTSP URL for 20 seconds:
```bash
./rtsp_recorder
```

Record from a specific RTSP URL for 30 seconds:
```bash
./rtsp_recorder -i rtsp://username:password@camera-ip:554/stream -d 30
```

Record to a specific output file:
```bash
./rtsp_recorder -o my_recording.mp4
```

## Troubleshooting

If you encounter issues with the recording:

1. Verify that the RTSP URL is correct and accessible
2. Ensure FFmpeg libraries are properly installed
3. Check if the camera supports the RTSP protocol
4. Try using TCP transport if UDP is unreliable (this is the default in the program)
5. Increase the timeout value if the connection is slow

## Notes on Low Latency

This program uses several FFmpeg options to minimize latency:

- `rtsp_transport=tcp`: Uses TCP for RTSP transport (more reliable than UDP)
- `fflags=nobuffer`: Reduces buffering
- `flags=low_delay`: Enables low delay mode
- `max_delay=500000`: Sets maximum delay to 500ms
- `stimeout=5000000`: Sets socket timeout to 5 seconds

These options are similar to those used in the ffplay command that this program is based on.
