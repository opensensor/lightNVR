# LightNVR API Documentation

This document describes the REST API endpoints provided by LightNVR.

## API Overview

LightNVR provides a RESTful API that allows you to interact with the system programmatically. The API is accessible via HTTP and returns JSON responses using the cJSON library. The API is served by the Mongoose web server.

**Note:** As of version 0.11.22, WebSocket support has been removed to simplify the architecture. All real-time updates are now handled via HTTP polling.

## Authentication

If authentication is enabled in the configuration file, all API requests must include authentication credentials. Authentication is performed using HTTP Basic Authentication.

Example:
```bash
curl -u username:password http://your-lightnvr-ip:8080/api/streams
```

## API Endpoints

### Streams

#### List Streams

```
GET /api/streams
```

Returns a list of all configured streams.

**Response:**
```json
{
  "streams": [
    {
      "id": 0,
      "name": "Front Door",
      "url": "rtsp://192.168.1.100:554/stream1",
      "enabled": true,
      "width": 1920,
      "height": 1080,
      "fps": 15,
      "codec": "h264",
      "priority": 10,
      "record": true,
      "status": "connected"
    },
    {
      "id": 1,
      "name": "Back Yard",
      "url": "rtsp://192.168.1.101:554/stream1",
      "enabled": true,
      "width": 1280,
      "height": 720,
      "fps": 10,
      "codec": "h264",
      "priority": 5,
      "record": true,
      "status": "connected"
    }
  ]
}
```

#### Get Stream

```
GET /api/streams/{id}
```

Returns information about a specific stream.

**Response:**
```json
{
  "id": 0,
  "name": "Front Door",
  "url": "rtsp://192.168.1.100:554/stream1",
  "enabled": true,
  "width": 1920,
  "height": 1080,
  "fps": 15,
  "codec": "h264",
  "priority": 10,
  "record": true,
  "status": "connected"
}
```

#### Add Stream

```
POST /api/streams
```

Adds a new stream.

**Request Body:**
```json
{
  "name": "Side Gate",
  "url": "rtsp://192.168.1.102:554/stream1",
  "enabled": true,
  "width": 1280,
  "height": 720,
  "fps": 10,
  "codec": "h264",
  "priority": 5,
  "record": true
}
```

**Response:**
```json
{
  "id": 2,
  "name": "Side Gate",
  "url": "rtsp://192.168.1.102:554/stream1",
  "enabled": true,
  "width": 1280,
  "height": 720,
  "fps": 10,
  "codec": "h264",
  "priority": 5,
  "record": true,
  "status": "connecting"
}
```

#### Update Stream

```
PUT /api/streams/{id}
```

Updates an existing stream.

**Request Body:**
```json
{
  "name": "Side Gate",
  "url": "rtsp://192.168.1.102:554/stream1",
  "enabled": false,
  "width": 1280,
  "height": 720,
  "fps": 10,
  "codec": "h264",
  "priority": 5,
  "record": true
}
```

**Response:**
```json
{
  "id": 2,
  "name": "Side Gate",
  "url": "rtsp://192.168.1.102:554/stream1",
  "enabled": false,
  "width": 1280,
  "height": 720,
  "fps": 10,
  "codec": "h264",
  "priority": 5,
  "record": true,
  "status": "disabled"
}
```

#### Delete Stream

```
DELETE /api/streams/{id}
```

Deletes a stream.

**Response:**
```json
{
  "success": true
}
```

### Recordings

#### List Recordings

```
GET /api/recordings
```

Returns a list of all recordings.

**Response:**
```json
{
  "recordings": [
    {
      "id": 1,
      "stream_id": 0,
      "stream_name": "Front Door",
      "start_time": "2025-03-09T10:00:00Z",
      "end_time": "2025-03-09T10:15:00Z",
      "duration": 900,
      "size": 45678912,
      "format": "mp4",
      "path": "/var/lib/lightnvr/recordings/0/20250309_100000.mp4"
    },
    {
      "id": 2,
      "stream_id": 0,
      "stream_name": "Front Door",
      "start_time": "2025-03-09T10:15:00Z",
      "end_time": "2025-03-09T10:30:00Z",
      "duration": 900,
      "size": 43567890,
      "format": "mp4",
      "path": "/var/lib/lightnvr/recordings/0/20250309_101500.mp4"
    }
  ]
}
```

#### Get Recording

```
GET /api/recordings/{id}
```

Returns information about a specific recording.

**Response:**
```json
{
  "id": 1,
  "stream_id": 0,
  "stream_name": "Front Door",
  "start_time": "2025-03-09T10:00:00Z",
  "end_time": "2025-03-09T10:15:00Z",
  "duration": 900,
  "size": 45678912,
  "format": "mp4",
  "path": "/var/lib/lightnvr/recordings/0/20250309_100000.mp4"
}
```

#### Delete Recording

```
DELETE /api/recordings/{id}
```

Deletes a recording.

**Response:**
```json
{
  "success": true
}
```

### System

#### Get System Information

```
GET /api/system
```

Returns system information.

**Response:**
```json
{
  "version": "0.4.0",
  "uptime": 86400,
  "cpu_usage": 15.2,
  "memory_usage": 45.7,
  "storage_usage": 78.3,
  "storage_total": 1000000000,
  "storage_used": 783000000,
  "storage_free": 217000000,
  "streams_active": 2,
  "streams_total": 3,
  "recordings_total": 48
}
```

#### Get System Settings

```
GET /api/settings
```

Returns system settings.

**Response:**
```json
{
  "storage_path": "/var/lib/lightnvr/recordings",
  "max_storage_size": 0,
  "retention_days": 30,
  "auto_delete_oldest": true,
  "web_port": 8080,
  "web_auth_enabled": true,
  "max_streams": 16,
  "buffer_size": 1024,
  "use_swap": true,
  "swap_size": 134217728,
  "hw_accel_enabled": false
}
```

#### Update System Settings

```
PUT /api/settings
```

Updates system settings.

**Request Body:**
```json
{
  "retention_days": 15,
  "auto_delete_oldest": true,
  "buffer_size": 2048
}
```

**Response:**
```json
{
  "storage_path": "/var/lib/lightnvr/recordings",
  "max_storage_size": 0,
  "retention_days": 15,
  "auto_delete_oldest": true,
  "web_port": 8080,
  "web_auth_enabled": true,
  "max_streams": 16,
  "buffer_size": 2048,
  "use_swap": true,
  "swap_size": 134217728,
  "hw_accel_enabled": false
}
```

### Streaming

#### Get Live Stream (HLS)

```
GET /api/streaming/{id}/hls/playlist.m3u8
```

Returns the HLS playlist for a live stream.

#### Get Live Stream (MJPEG)

```
GET /api/streaming/{id}/mjpeg
```

Returns a Motion JPEG stream.

## Error Handling

All API endpoints return appropriate HTTP status codes:

- 200: Success
- 400: Bad Request
- 401: Unauthorized
- 404: Not Found
- 500: Internal Server Error

Error responses include a JSON object with an error message:

```json
{
  "error": "Stream not found"
}
```

## Rate Limiting

To prevent abuse, the API implements rate limiting. If you exceed the rate limit, you will receive a 429 Too Many Requests response.

## Examples

### Curl Examples

List all streams:
```bash
curl -u admin:admin http://your-lightnvr-ip:8080/api/streams
```

Get system information:
```bash
curl -u admin:admin http://your-lightnvr-ip:8080/api/system
```

Add a new stream:
```bash
curl -u admin:admin -X POST -H "Content-Type: application/json" -d '{"name":"New Camera","url":"rtsp://192.168.1.103:554/stream1","enabled":true,"width":1280,"height":720,"fps":10,"codec":"h264","priority":5,"record":true}' http://your-lightnvr-ip:8080/api/streams
```

### JavaScript Examples

List all streams:
```javascript
fetch('http://your-lightnvr-ip:8080/api/streams', {
  headers: {
    'Authorization': 'Basic ' + btoa('admin:admin')
  }
})
.then(response => response.json())
.then(data => console.log(data));
```

Add a new stream:
```javascript
fetch('http://your-lightnvr-ip:8080/api/streams', {
  method: 'POST',
  headers: {
    'Authorization': 'Basic ' + btoa('admin:admin'),
    'Content-Type': 'application/json'
  },
  body: JSON.stringify({
    name: 'New Camera',
    url: 'rtsp://192.168.1.103:554/stream1',
    enabled: true,
    width: 1280,
    height: 720,
    fps: 10,
    codec: 'h264',
    priority: 5,
    record: true
  })
})
.then(response => response.json())
.then(data => console.log(data));
```

### Preact Component Example

```javascript
import { h } from 'preact';
import { useState, useEffect } from 'preact/hooks';

function StreamsList() {
  const [streams, setStreams] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  useEffect(() => {
    // Fetch streams when component mounts
    fetch('/api/streams')
      .then(response => {
        if (!response.ok) {
          throw new Error('Network response was not ok');
        }
        return response.json();
      })
      .then(data => {
        setStreams(data.streams);
        setLoading(false);
      })
      .catch(error => {
        setError(error.message);
        setLoading(false);
      });
  }, []);

  if (loading) return <div>Loading streams...</div>;
  if (error) return <div>Error: {error}</div>;

  return (
    <div class="streams-list">
      <h2>Streams</h2>
      {streams.length === 0 ? (
        <p>No streams configured</p>
      ) : (
        <ul>
          {streams.map(stream => (
            <li key={stream.id}>
              <strong>{stream.name}</strong> - {stream.status}
              <p>URL: {stream.url}</p>
              <p>Resolution: {stream.width}x{stream.height} @ {stream.fps}fps</p>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}
