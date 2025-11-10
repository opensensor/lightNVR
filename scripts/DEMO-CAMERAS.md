# Demo Camera Configuration

This guide explains how to configure demo cameras for automated screenshot and video capture.

## Quick Start

1. **Edit `demo-cameras.json`** with your camera details:

```json
{
  "cameras": [
    {
      "name": "Front Yard",
      "url": "rtsp://username:password@192.168.50.136:554/ch0",
      "enabled": true,
      "detection_enabled": true
    }
  ]
}
```

2. **Run the setup script**:

```bash
node scripts/setup-demo-streams.js
```

3. **Capture screenshots**:

```bash
./scripts/update-documentation-media.sh --screenshots-only
```

## Configuration File Format

### Basic Camera Configuration

```json
{
  "cameras": [
    {
      "name": "Camera Name",
      "url": "rtsp://username:password@ip:port/path",
      "description": "Optional description",
      "enabled": true
    }
  ]
}
```

### Full Configuration with Detection

```json
{
  "cameras": [
    {
      "name": "Front Yard",
      "url": "rtsp://thingino:thingino@192.168.50.136:554/ch0",
      "description": "Thingino camera - front yard view",
      "enabled": true,
      "detection_enabled": true,
      "detection_url": "http://localhost:9001/api/v1/detect",
      "detection_backend": "onnx",
      "detection_confidence": 0.5,
      "detection_classes": "person,car,dog,cat,bicycle,motorcycle",
      "recording_mode": "detection"
    }
  ]
}
```

### Configuration with Detection Zones

```json
{
  "cameras": [
    {
      "name": "Front Yard",
      "url": "rtsp://thingino:thingino@192.168.50.136:554/ch0",
      "enabled": true,
      "detection_enabled": true,
      "detection_url": "http://localhost:9001/api/v1/detect",
      "detection_backend": "onnx",
      "zones": [
        {
          "name": "Driveway",
          "enabled": true,
          "color": "#3b82f6",
          "points": [
            { "x": 0.2, "y": 0.5 },
            { "x": 0.5, "y": 0.5 },
            { "x": 0.5, "y": 0.9 },
            { "x": 0.2, "y": 0.9 }
          ],
          "class_filter": "person,car",
          "confidence_threshold": 0.5
        },
        {
          "name": "Front Door",
          "enabled": true,
          "color": "#10b981",
          "points": [
            { "x": 0.6, "y": 0.3 },
            { "x": 0.8, "y": 0.3 },
            { "x": 0.8, "y": 0.7 },
            { "x": 0.6, "y": 0.7 }
          ],
          "class_filter": "person",
          "confidence_threshold": 0.6
        }
      ]
    }
  ]
}
```

## Configuration Options

### Camera Settings

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | Yes | Display name for the camera |
| `url` | string | Yes | RTSP URL (format: `rtsp://user:pass@ip:port/path`) |
| `description` | string | No | Optional description |
| `enabled` | boolean | No | Enable/disable stream (default: true) |

### Detection Settings

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `detection_enabled` | boolean | No | Enable object detection |
| `detection_url` | string | No | light-object-detect API URL (default: `http://localhost:9001/api/v1/detect`) |
| `detection_backend` | string | No | Backend: `onnx`, `tflite`, or `opencv` (default: `onnx`) |
| `detection_confidence` | number | No | Confidence threshold 0.0-1.0 (default: 0.5) |
| `detection_classes` | string | No | Comma-separated class names (e.g., `person,car`) |
| `recording_mode` | string | No | Mode: `detection`, `continuous`, `motion`, `disabled` |

### Zone Settings

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | Yes | Zone name |
| `enabled` | boolean | No | Enable/disable zone (default: true) |
| `color` | string | No | Hex color code (e.g., `#3b82f6`) |
| `points` | array | Yes | Array of `{x, y}` coordinates (normalized 0.0-1.0) |
| `class_filter` | string | No | Comma-separated classes to detect in this zone |
| `confidence_threshold` | number | No | Confidence threshold for this zone |

### Zone Coordinates

Zone points use normalized coordinates (0.0 to 1.0):
- **x**: 0.0 = left edge, 1.0 = right edge
- **y**: 0.0 = top edge, 1.0 = bottom edge

Example: A zone covering the bottom-left quarter of the frame:
```json
{
  "points": [
    { "x": 0.0, "y": 0.5 },
    { "x": 0.5, "y": 0.5 },
    { "x": 0.5, "y": 1.0 },
    { "x": 0.0, "y": 1.0 }
  ]
}
```

## Common Camera URLs

### Thingino Cameras
```
rtsp://thingino:thingino@192.168.1.100:554/ch0
```

### Generic RTSP Cameras
```
rtsp://admin:password@192.168.1.100:554/stream1
rtsp://admin:password@192.168.1.100:554/h264
```

### Hikvision
```
rtsp://admin:password@192.168.1.100:554/Streaming/Channels/101
```

### Dahua
```
rtsp://admin:password@192.168.1.100:554/cam/realmonitor?channel=1&subtype=0
```

### Reolink
```
rtsp://admin:password@192.168.1.100:554/h264Preview_01_main
```

### Amcrest
```
rtsp://admin:password@192.168.1.100:554/cam/realmonitor?channel=1&subtype=0
```

## Usage Examples

### Setup Single Camera

```bash
# Edit demo-cameras.json with your camera
node scripts/setup-demo-streams.js

# Verify in LightNVR UI
# Then capture screenshots
./scripts/update-documentation-media.sh --screenshots-only
```

### Setup Multiple Cameras

```json
{
  "cameras": [
    {
      "name": "Front Yard",
      "url": "rtsp://user:pass@192.168.1.100:554/ch0"
    },
    {
      "name": "Back Yard",
      "url": "rtsp://user:pass@192.168.1.101:554/ch0"
    },
    {
      "name": "Garage",
      "url": "rtsp://user:pass@192.168.1.102:554/ch0"
    }
  ]
}
```

### Custom Configuration File

```bash
# Use a different config file
node scripts/setup-demo-streams.js --config /path/to/my-cameras.json
```

### Different LightNVR Instance

```bash
# Setup on remote LightNVR
node scripts/setup-demo-streams.js \
  --url http://192.168.1.50:8080 \
  --username admin \
  --password mypassword
```

## Troubleshooting

### Camera Not Accessible

If your camera is on a different network (e.g., 192.168.50.x vs 192.168.1.x):

1. **Check network connectivity**:
   ```bash
   ping 192.168.50.136
   ```

2. **Test RTSP stream**:
   ```bash
   mpv rtsp://thingino:thingino@192.168.50.136:554/ch0
   # or
   ffplay rtsp://thingino:thingino@192.168.50.136:554/ch0
   ```

3. **Update Docker network** (if using Docker):
   - Add camera network to docker-compose.yml
   - Or use host networking mode

### Stream Setup Fails

```bash
# Check LightNVR is running
curl http://localhost:8080/login.html

# Check credentials
node scripts/setup-demo-streams.js --username admin --password admin

# Check API endpoint
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"admin"}'
```

### Zones Not Created

- Zones require the stream to exist first
- Check that detection is enabled on the stream
- Verify zone coordinates are valid (0.0-1.0)
- Check LightNVR logs for errors

## Best Practices

1. **Use descriptive names** - "Front Yard" not "Camera 1"
2. **Test streams first** - Verify RTSP URL works with mpv/ffplay
3. **Start simple** - Add detection/zones after basic stream works
4. **Normalize coordinates** - Use 0.0-1.0 range for zone points
5. **Meaningful zones** - Draw zones that make sense (driveway, door, etc.)
6. **Appropriate classes** - Filter to relevant objects (person, car)
7. **Reasonable thresholds** - Start with 0.5 confidence, adjust as needed

## Integration with Screenshot Automation

The orchestration script automatically runs demo stream setup:

```bash
# This will:
# 1. Start LightNVR in Docker
# 2. Setup demo streams from demo-cameras.json
# 3. Capture screenshots
# 4. Optimize images
./scripts/update-documentation-media.sh --docker
```

To skip demo setup:
```bash
# Setup streams manually first, then:
./scripts/update-documentation-media.sh --screenshots-only
```

## Next Steps

After configuring demo cameras:

1. **Verify streams** - Check LightNVR UI shows live video
2. **Test detection** - Ensure objects are being detected
3. **Adjust zones** - Fine-tune zone coordinates and filters
4. **Capture media** - Run screenshot/video automation
5. **Review output** - Check docs/images/ for quality
6. **Update README** - Reference new screenshots in documentation

