# LightNVR API Documentation

This document describes the REST API endpoints provided by LightNVR.

## API Overview

LightNVR provides a RESTful API that allows you to interact with the system programmatically. The API is accessible via HTTP and returns JSON responses using the cJSON library. The API is served by the libuv + llhttp web server.

## Authentication

If authentication is enabled in the configuration file, API requests must include a valid session token. Obtain a session by calling the login endpoint, then include the session cookie in subsequent requests.

```bash
# Login to get a session
curl -c cookies.txt -X POST -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"yourpassword"}' \
  http://your-lightnvr-ip:8080/api/auth/login

# Use session cookie for subsequent requests
curl -b cookies.txt http://your-lightnvr-ip:8080/api/streams
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
      "streaming_enabled": true,
      "width": 1920,
      "height": 1080,
      "fps": 15,
      "codec": "h264",
      "priority": 10,
      "record": true,
      "segment_duration": 900,
      "protocol": 0,
      "record_audio": true,
      "detection_based_recording": 0,
      "detection_model": "",
      "detection_threshold": 0.5,
      "detection_interval": 10,
      "pre_detection_buffer": 0,
      "post_detection_buffer": 3,
      "detection_api_url": "",
      "is_onvif": false,
      "onvif_username": "",
      "onvif_password": "",
      "onvif_profile": "",
      "ptz_enabled": false,
      "backchannel_enabled": false,
      "buffer_strategy": "auto",
      "retention_days": 30,
      "status": "connected"
    }
  ]
}
```

#### Get Stream

```
GET /api/streams/{name}
```

Returns information about a specific stream by name.

#### Get Full Stream

```
GET /api/streams/{name}/full
```

Returns complete stream information including all configuration fields.

#### Add Stream

```
POST /api/streams
```

Adds a new stream. All fields from the stream schema are accepted.

#### Update Stream

```
PUT /api/streams/{name}
```

Updates an existing stream.

#### Delete Stream

```
DELETE /api/streams/{name}
```

Deletes a stream.

#### Test Stream

```
POST /api/streams/test
```

Tests connectivity to a stream URL.

#### Refresh Stream

```
POST /api/streams/{name}/refresh
```

Forces a stream reconnection.

### Stream Retention

#### Get Stream Retention

```
GET /api/streams/{name}/retention
```

Returns retention settings for a specific stream.

#### Update Stream Retention

```
PUT /api/streams/{name}/retention
```

Updates retention settings for a specific stream.

### Detection Zones

#### List Zones

```
GET /api/streams/{name}/zones
```

Returns detection zones for a stream.

#### Create/Update Zones

```
POST /api/streams/{name}/zones
```

Creates or updates detection zones for a stream.

#### Delete Zones

```
DELETE /api/streams/{name}/zones
```

Deletes detection zones for a stream.

### PTZ (Pan-Tilt-Zoom)

#### Get PTZ Capabilities

```
GET /api/streams/{name}/ptz/capabilities
```

Returns PTZ capabilities for a stream's camera.

#### PTZ Move

```
POST /api/streams/{name}/ptz/move
```

Starts continuous PTZ movement.

#### PTZ Stop

```
POST /api/streams/{name}/ptz/stop
```

Stops PTZ movement.

#### PTZ Absolute Move

```
POST /api/streams/{name}/ptz/absolute
```

Moves to an absolute PTZ position.

#### PTZ Relative Move

```
POST /api/streams/{name}/ptz/relative
```

Performs a relative PTZ movement.

#### PTZ Home

```
POST /api/streams/{name}/ptz/home
```

Moves to the home position.

#### PTZ Set Home

```
POST /api/streams/{name}/ptz/set-home
```

Sets the current position as home.

#### PTZ Presets

```
GET /api/streams/{name}/ptz/presets
```

Lists PTZ presets.

```
POST /api/streams/{name}/ptz/goto-preset
```

Moves to a PTZ preset.

```
PUT /api/streams/{name}/ptz/preset
```

Creates or updates a PTZ preset.

### Recordings

#### List Recordings

```
GET /api/recordings
```

Returns a list of recordings. Supports query parameters for filtering by stream name, date range, and pagination.

#### Get Recording

```
GET /api/recordings/{id}
```

Returns information about a specific recording.

#### Delete Recording

```
DELETE /api/recordings/{id}
```

Deletes a recording.

#### Play Recording

```
GET /api/recordings/play/{id}
```

Streams a recording for playback.

#### Download Recording

```
GET /api/recordings/download/{id}
```

Downloads a recording file.

#### Protect Recording

```
PUT /api/recordings/{id}/protect
```

Toggles protection status on a recording (protected recordings are exempt from auto-deletion).

#### Set Recording Retention

```
PUT /api/recordings/{id}/retention
```

Sets a per-recording retention override.

#### Batch Delete Recordings

```
POST /api/recordings/batch-delete
```

Deletes multiple recordings at once. Returns a job ID for progress tracking.

#### Batch Delete Progress

```
GET /api/recordings/batch-delete/progress/{job_id}
```

Returns progress for a batch delete operation.

#### Batch Protect Recordings

```
POST /api/recordings/batch-protect
```

Protects or unprotects multiple recordings at once.

#### Get Protected Recordings

```
GET /api/recordings/protected
```

Returns all protected recordings.

#### Recording File Check

```
GET /api/recordings/files/check
```

Checks if a recording file exists on disk.

#### Delete Recording File

```
DELETE /api/recordings/files
```

Deletes a recording file from disk.

#### Sync Recordings

```
POST /api/recordings/sync
```

Synchronizes the recordings database with files on disk.

### Timeline

#### Get Timeline Segments

```
GET /api/timeline/segments
```

Returns recording segments for the timeline view. Supports query parameters for stream name and date range.

#### Get Timeline Manifest

```
GET /api/timeline/manifest
```

Returns a manifest of available timeline data.

#### Timeline Playback

```
GET /api/timeline/play
```

Streams video for timeline playback at a specified point in time.

### System

#### Get System Information

```
GET /api/system
GET /api/system/info
```

Returns system information including version, uptime, CPU/memory/storage usage, and stream counts.

#### Get System Status

```
GET /api/system/status
```

Returns system health status.

#### Get System Logs

```
GET /api/system/logs
```

Returns recent system log entries.

#### Clear System Logs

```
POST /api/system/logs/clear
```

Clears the system log file.

#### Restart System

```
POST /api/system/restart
```

Restarts the LightNVR service.

#### Shutdown System

```
POST /api/system/shutdown
```

Shuts down the LightNVR service.

#### System Backup

```
POST /api/system/backup
```

Creates a backup of the database.

#### Get Settings

```
GET /api/settings
```

Returns system configuration settings.

#### Update Settings

```
POST /api/settings
```

Updates system configuration settings.

### Health

#### Health Check

```
GET /api/health
```

Returns basic health status.

#### HLS Health Check

```
GET /api/health/hls
```

Returns HLS streaming subsystem health.

### ICE Servers

#### Get ICE Servers

```
GET /api/ice-servers
```

Returns WebRTC ICE server configuration (STUN/TURN servers).

### Authentication

#### Login

```
POST /api/auth/login
```

Authenticates a user and creates a session.

**Request Body:**
```json
{
  "username": "admin",
  "password": "yourpassword"
}
```

#### Login with TOTP

```
POST /api/auth/login/totp
```

Completes login with a TOTP code (for users with MFA enabled).

**Request Body:**
```json
{
  "token": "pending_session_token",
  "totp_code": "123456"
}
```

#### Logout

```
POST /api/auth/logout
GET /logout
```

Destroys the current session.

#### Verify Session

```
GET /api/auth/verify
```

Verifies that the current session is valid.

### User Management

#### List Users

```
GET /api/auth/users
```

Returns all users (admin only).

#### Create User

```
POST /api/auth/users
```

Creates a new user.

#### Get User

```
GET /api/auth/users/{id}
```

Returns a specific user.

#### Update User

```
PUT /api/auth/users/{id}
```

Updates a user.

#### Delete User

```
DELETE /api/auth/users/{id}
```

Deletes a user.

#### Generate API Key

```
POST /api/auth/users/{id}/api-key
```

Generates an API key for a user.

#### Change Password

```
PUT /api/auth/users/{id}/password
```

Changes a user's password.

#### Password Lock

```
PUT /api/auth/users/{id}/password-lock
```

Locks or unlocks a user's password from being changed.

### TOTP/MFA

#### Setup TOTP

```
POST /api/auth/users/{id}/totp/setup
```

Initiates TOTP setup, returns secret and QR code URI.

#### Verify TOTP

```
POST /api/auth/users/{id}/totp/verify
```

Verifies a TOTP code during setup to confirm it works.

#### Disable TOTP

```
POST /api/auth/users/{id}/totp/disable
```

Disables TOTP for a user.

#### TOTP Status

```
GET /api/auth/users/{id}/totp/status
```

Returns whether TOTP is enabled for a user.

### ONVIF Discovery

#### Discovery Status

```
GET /api/onvif/discovery/status
```

Returns ONVIF discovery service status.

#### Discover Devices

```
POST /api/onvif/discovery/discover
```

Triggers an ONVIF device discovery scan.

#### List Discovered Devices

```
GET /api/onvif/devices
```

Returns discovered ONVIF devices.

#### Get Device Profiles

```
GET /api/onvif/device/profiles
```

Returns media profiles for an ONVIF device.

#### Add Device as Stream

```
POST /api/onvif/device/add
```

Adds a discovered ONVIF device as a stream.

#### Test ONVIF Connection

```
POST /api/onvif/device/test
```

Tests connectivity to an ONVIF device.

### Detection

#### Get Detection Results

```
GET /api/detection/results/{stream_name}
```

Returns recent detection results for a stream.

#### List Detection Models

```
GET /api/detection/models
```

Returns available detection models.

### Motion Recording

#### Get Motion Config

```
GET /api/motion/config/{stream_name}
```

Returns motion recording configuration for a stream.

#### Set Motion Config

```
POST /api/motion/config/{stream_name}
```

Creates or updates motion recording configuration.

#### Delete Motion Config

```
DELETE /api/motion/config/{stream_name}
```

Deletes motion recording configuration for a stream.

#### Test Motion Event

```
POST /api/motion/test/{stream_name}
```

Triggers a test motion event for debugging.

#### Get Motion Stats

```
GET /api/motion/stats/{stream_name}
```

Returns motion recording statistics.

#### List Motion Recordings

```
GET /api/motion/recordings/{stream_name}
```

Returns motion-triggered recordings.

#### Delete Motion Recording

```
DELETE /api/motion/recordings/{recording_id}
```

Deletes a motion recording.

#### Motion Cleanup

```
POST /api/motion/cleanup
```

Triggers cleanup of expired motion recordings.

#### Motion Storage

```
GET /api/motion/storage
```

Returns motion recording storage usage.

### HLS Streaming

#### Direct HLS Request

```
GET /hls/{stream_name}/{filename}
```

Serves HLS playlist (.m3u8) and segment (.ts) files for live streaming.

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
