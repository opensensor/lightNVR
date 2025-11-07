# Detection Zone Configuration

This document describes the detection zone configuration feature for lightNVR.

## Overview

Detection zones allow you to define specific regions in your camera's field of view where object detection should be active. This helps:

- **Reduce false positives** by ignoring areas like trees, roads, or sky
- **Focus on areas of interest** like doorways, driveways, or specific zones
- **Improve performance** by limiting detection to relevant areas
- **Configure per-zone settings** like class filters and confidence thresholds

## Features

### Interactive Zone Editor

- **Visual polygon drawing** on live camera snapshot
- **Multiple zones per stream** with different configurations
- **Color-coded zones** for easy identification
- **Enable/disable zones** individually
- **Per-zone class filtering** (e.g., only detect persons in entrance zone)
- **Per-zone confidence thresholds**

### Zone Modes

When a zone is configured, detections are filtered based on the zone mode:

- **Center mode**: Detection's bounding box center must be in zone
- **Any mode**: Any part of detection's bounding box must overlap with zone
- **All mode**: Entire detection's bounding box must be within zone

## User Interface

### Accessing the Zone Editor

1. Navigate to **Streams** page
2. Click **Edit** on a stream
3. Enable **AI Detection Recording**
4. Expand the **Detection Zones** section
5. Click **Configure Zones** button

### Drawing Zones

1. Click **Draw Zone** button
2. Click on the camera preview to add points
3. Click multiple points to create a polygon (minimum 3 points)
4. Click **Complete Zone** when done
5. Repeat for additional zones

### Editing Zones

1. Click **Select** mode
2. Click on a zone to select it
3. Edit zone properties in the sidebar:
   - Zone name
   - Color
   - Enabled/disabled status
4. Click **Save Zones** when done

### Deleting Zones

1. Click **Delete** mode
2. Click on a zone to delete it
3. Or select a zone and use the delete button in the sidebar

## Database Schema

### detection_zones Table

```sql
CREATE TABLE detection_zones (
    id TEXT PRIMARY KEY,
    stream_name TEXT NOT NULL,
    name TEXT NOT NULL,
    enabled INTEGER DEFAULT 1,
    color TEXT DEFAULT '#3b82f6',
    polygon TEXT NOT NULL,  -- JSON array of points
    filter_classes TEXT DEFAULT '',
    min_confidence REAL DEFAULT 0.0,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (stream_name) REFERENCES streams(name) ON DELETE CASCADE
);
```

### Polygon Format

Polygons are stored as JSON arrays of normalized coordinates (0.0 - 1.0):

```json
[
  {"x": 0.0, "y": 0.0},
  {"x": 0.5, "y": 0.0},
  {"x": 0.5, "y": 1.0},
  {"x": 0.0, "y": 1.0}
]
```

## API Endpoints

### Get Zones for Stream

```http
GET /api/streams/{stream_name}/zones
```

**Response:**
```json
{
  "zones": [
    {
      "id": "zone_1234567890",
      "stream_name": "front_door",
      "name": "Entrance",
      "enabled": true,
      "color": "#3b82f6",
      "polygon": [
        {"x": 0.0, "y": 0.0},
        {"x": 0.5, "y": 0.0},
        {"x": 0.5, "y": 1.0},
        {"x": 0.0, "y": 1.0}
      ],
      "filter_classes": "person",
      "min_confidence": 0.7
    }
  ]
}
```

### Save Zones for Stream

```http
POST /api/streams/{stream_name}/zones
Content-Type: application/json

{
  "zones": [
    {
      "id": "zone_1234567890",
      "name": "Entrance",
      "enabled": true,
      "color": "#3b82f6",
      "polygon": [
        {"x": 0.0, "y": 0.0},
        {"x": 0.5, "y": 0.0},
        {"x": 0.5, "y": 1.0},
        {"x": 0.0, "y": 1.0}
      ],
      "filter_classes": "person",
      "min_confidence": 0.7
    }
  ]
}
```

**Response:**
```json
{
  "success": true,
  "message": "Zones saved successfully",
  "count": 1
}
```

### Delete Zones for Stream

```http
DELETE /api/streams/{stream_name}/zones
```

## Integration with light-object-detect

When detection is triggered, lightNVR sends zone configuration to the light-object-detect API:

```bash
curl -X POST "http://localhost:8000/api/v1/detect" \
  -F "file=@frame.jpg" \
  -F "zones={\"zones\":[...],\"zone_mode\":\"center\"}"
```

The API filters detections based on the zones and returns only detections within the configured zones.

## Configuration Example

### Example 1: Entrance Monitoring

Monitor only the entrance area for persons:

```json
{
  "id": "entrance_zone",
  "name": "Front Entrance",
  "enabled": true,
  "color": "#3b82f6",
  "polygon": [
    {"x": 0.2, "y": 0.3},
    {"x": 0.8, "y": 0.3},
    {"x": 0.8, "y": 0.9},
    {"x": 0.2, "y": 0.9}
  ],
  "filter_classes": "person",
  "min_confidence": 0.75
}
```

### Example 2: Parking Lot Monitoring

Monitor parking lot for vehicles:

```json
{
  "id": "parking_zone",
  "name": "Parking Lot",
  "enabled": true,
  "color": "#10b981",
  "polygon": [
    {"x": 0.0, "y": 0.5},
    {"x": 1.0, "y": 0.5},
    {"x": 1.0, "y": 1.0},
    {"x": 0.0, "y": 1.0}
  ],
  "filter_classes": "car,truck,motorcycle",
  "min_confidence": 0.6
}
```

### Example 3: Multiple Zones

Combine multiple zones for comprehensive monitoring:

```json
{
  "zones": [
    {
      "id": "entrance",
      "name": "Entrance",
      "enabled": true,
      "color": "#3b82f6",
      "polygon": [...],
      "filter_classes": "person",
      "min_confidence": 0.75
    },
    {
      "id": "driveway",
      "name": "Driveway",
      "enabled": true,
      "color": "#10b981",
      "polygon": [...],
      "filter_classes": "car,truck,person",
      "min_confidence": 0.65
    },
    {
      "id": "backyard",
      "name": "Backyard",
      "enabled": false,
      "color": "#f59e0b",
      "polygon": [...],
      "filter_classes": "",
      "min_confidence": 0.5
    }
  ]
}
```

## Best Practices

1. **Start with simple zones**: Begin with one or two zones and refine as needed
2. **Use appropriate confidence thresholds**: Higher for critical areas, lower for general monitoring
3. **Avoid overlapping zones**: Can cause duplicate detections
4. **Test your zones**: Use the live preview to verify zone coverage
5. **Name zones descriptively**: Makes it easier to identify in logs and alerts
6. **Disable unused zones**: Rather than deleting, disable zones you might need later

## Troubleshooting

### Zones not appearing in editor

- Ensure the stream has a valid snapshot available
- Check browser console for errors
- Verify the stream is running

### Detections not being filtered

- Verify zones are enabled
- Check zone polygon coordinates are valid (0.0 - 1.0)
- Ensure light-object-detect API is receiving zone configuration
- Check API logs for zone filtering errors

### Performance issues

- Reduce number of zones (max 16 per stream recommended)
- Simplify polygon shapes (fewer points)
- Use larger zones rather than many small ones

## Future Enhancements

Potential future improvements:

- Zone crossing detection and counting
- Heatmap generation from zone detections
- Zone-based alerts and notifications
- Time-based zone activation
- Zone templates for common scenarios
- Zone analytics and statistics

