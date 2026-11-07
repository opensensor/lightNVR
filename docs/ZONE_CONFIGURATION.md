# Detection Zone Configuration

Detection zones limit where on a camera's image detections count. Use them to ignore trees,
roads or the sky, or to record only when something is at a doorway or in a driveway.

## How zones filter detections

LightNVR applies zones itself after every detection, whatever produced it: an external API
(light-object-detect, DOODS2), embedded SOD, the in-process LiteRT engine, or ONVIF
camera events that carry boxes. The detector never sees the zones.

- **No enabled zones on a stream:** every detection counts.
- **One or more enabled zones:** a detection counts only if the **centre of its bounding
  box** is inside an enabled zone whose class filter and minimum confidence it also
  passes. Zones are checked in order and the first match wins; its ID is stored with the
  detection, and Investigation can filter by it.
- **Class filter** (`filter_classes`): comma-separated labels such as `person,car`. Empty
  accepts every class.
- **Minimum confidence** (`min_confidence`, 0.0-1.0): `0` accepts any confidence.

Each stream can have up to 16 zones of 3 to 32 points.

## Drawing zones

1. Open **Streams**, and **Edit** the stream.
2. Turn on **Enable AI Detection-Based Recording**.
3. Under **Detection Zones**, click **Configure Zones** (or **Edit Zones**).
4. Click **Draw Zone**, then click points on the snapshot. Click **Complete Zone** when the
   polygon is closed. If no snapshot is available you can still draw on a grid.
5. Use **Select** to pick a zone and change its name, colour or **Enabled** switch, and
   **Delete** to remove one.
6. Click **Save Zones**.

The editor sets a zone's shape, name, colour and enabled state. Set **class filters and
minimum confidence through the API** ([below](#api-endpoints)).

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
3. **Order overlapping zones deliberately**: a detection is attributed to the first matching zone
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
- Remember that only the bounding-box centre is tested
- Check class names match the detector's labels exactly (`person`, not `Person`)
- Run with `log_level = 3`; each rejection is logged with its reason

### Performance issues

- Reduce the number of zones (the limit is 16 per stream)
- Simplify polygon shapes (fewer points)
- Use larger zones rather than many small ones
