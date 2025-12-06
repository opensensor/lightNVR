# Quick Reference: Recording Retention API

## Database Functions

### Stream Retention Configuration
```c
// Get retention config for a stream
int get_stream_retention_config(const char *stream_name, 
                                stream_retention_config_t *config);

// Set retention config for a stream
int set_stream_retention_config(const char *stream_name, 
                                const stream_retention_config_t *config);

// Get storage usage for a stream
int get_stream_storage_usage(const char *stream_name, uint64_t *size_bytes);

// Structure
typedef struct {
    int retention_days;              // Regular recordings retention
    int detection_retention_days;    // Detection recordings retention
    uint64_t max_storage_mb;        // Storage quota (0 = unlimited)
} stream_retention_config_t;
```

### Recording Protection
```c
// Mark recording as protected/unprotected
int set_recording_protected(uint64_t recording_id, bool protected);

// Set custom retention override for recording
int set_recording_retention_override(uint64_t recording_id, int days);

// Get count of protected recordings for a stream
int get_protected_recordings_count(const char *stream_name);
```

### Retention Policy Execution
```c
// Apply retention policy for all streams
int apply_retention_policy(void);

// Apply retention policy for specific stream
static int apply_retention_policy_for_stream(const char *stream_name);

// Enforce storage quota for stream
static int enforce_storage_quota(const char *stream_name, uint64_t max_mb);

// Cleanup orphaned files and DB entries
static int cleanup_orphaned_files(void);
static int cleanup_orphaned_db_entries(void);
```

## REST API Endpoints

### Stream Retention Configuration

#### Update Stream Retention Settings
```http
PATCH /api/streams/:name
Content-Type: application/json

{
  "retention_days": 30,
  "detection_retention_days": 90,
  "max_storage_mb": 10240
}

Response: 200 OK
{
  "success": true,
  "message": "Stream updated successfully"
}
```

#### Get Stream with Retention Settings
```http
GET /api/streams/:name

Response: 200 OK
{
  "name": "camera1",
  "url": "rtsp://...",
  "retention_days": 30,
  "detection_retention_days": 90,
  "max_storage_mb": 10240,
  ...
}
```

### Recording Protection

#### Protect/Unprotect Recording
```http
PATCH /api/recordings/:id/protect
Content-Type: application/json

{
  "protected": true,
  "retention_override_days": 365  // Optional
}

Response: 200 OK
{
  "success": true,
  "message": "Recording protection updated"
}
```

#### Get Protected Recordings
```http
GET /api/recordings?protected=true&stream_name=camera1

Response: 200 OK
{
  "recordings": [
    {
      "id": 123,
      "stream_name": "camera1",
      "protected": true,
      "retention_override_days": 365,
      "trigger_type": "detection",
      ...
    }
  ],
  "total": 5
}
```

### Enhanced Batch Delete

#### Batch Delete with Filters
```http
POST /api/recordings/batch-delete
Content-Type: application/json

{
  "filter": {
    "stream_name": "camera1",
    "start_time": 1234567890,
    "end_time": 1234567999,
    "has_detections": false,
    "protected": false,
    "trigger_type": "scheduled"
  }
}

Response: 200 OK
{
  "success": true,
  "deleted": 45,
  "failed": 0,
  "protected_excluded": 5,
  "breakdown": {
    "scheduled": 30,
    "detection": 15
  }
}
```

## SQL Queries

### Query Recordings for Deletion (Priority 1)
```sql
SELECT id, file_path, start_time, size_bytes
FROM recordings
WHERE stream_name = ?
  AND start_time < ?
  AND trigger_type != 'detection'
  AND protected = 0
  AND retention_override_days IS NULL
ORDER BY start_time ASC;
```

### Query Recordings for Deletion (Priority 2)
```sql
SELECT id, file_path, start_time, size_bytes
FROM recordings
WHERE stream_name = ?
  AND start_time < ?
  AND trigger_type = 'detection'
  AND protected = 0
  AND retention_override_days IS NULL
ORDER BY start_time ASC;
```

### Get Stream Storage Usage
```sql
SELECT SUM(size_bytes) as total_bytes
FROM recordings
WHERE stream_name = ?
  AND is_complete = 1;
```

### Get Protected Recordings Count
```sql
SELECT COUNT(*) as protected_count
FROM recordings
WHERE stream_name = ?
  AND protected = 1;
```

## Configuration Examples

### Example 1: High-Security Camera
```json
{
  "retention_days": 90,
  "detection_retention_days": 365,
  "max_storage_mb": 102400
}
```

### Example 2: Low-Priority Camera
```json
{
  "retention_days": 7,
  "detection_retention_days": 30,
  "max_storage_mb": 10240
}
```

### Example 3: Archive Camera (Unlimited)
```json
{
  "retention_days": 0,
  "detection_retention_days": 0,
  "max_storage_mb": 0
}
```
Note: retention_days = 0 means unlimited retention

## Error Codes

| Code | Meaning |
|------|---------|
| 200 | Success |
| 400 | Invalid parameters |
| 403 | Permission denied |
| 404 | Recording/Stream not found |
| 500 | Internal server error |

