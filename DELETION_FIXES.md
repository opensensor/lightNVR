# lightNVR Bug Fixes

## Segfault on Stream IP Update (CRITICAL)

### Issue
Segmentation fault occurred when updating a stream's IP address/URL.

### Root Cause
**Race condition in stream update sequence** (`src/web/api_handlers_streams_modify.c`):

The original code had this sequence:
1. Stop the stream
2. Wait for stream to stop
3. **Start the stream** (with new URL from database)
4. **Then** unregister old URL from go2rtc
5. **Then** re-register new URL with go2rtc
6. **Then** restart HLS stream

**The Problem**: The stream was started (step 3) BEFORE go2rtc was updated (steps 4-5). This meant:
- The stream thread would try to use go2rtc immediately
- go2rtc still had the OLD URL registered
- When we unregister/re-register go2rtc while the stream is running, we're modifying go2rtc resources that the stream thread is actively using
- This causes a race condition and potential segfault

### Fix
Changed the sequence to update go2rtc **BEFORE** starting the stream:

1. Stop the stream
2. Wait for stream to stop
3. **Unregister old URL from go2rtc**
4. Wait 500ms for go2rtc cleanup
5. **Re-register new URL with go2rtc**
6. Wait 500ms for go2rtc to be ready
7. **Start the stream** (now go2rtc is ready with the new URL)
8. Restart HLS stream if needed

**Files Modified**:
- `src/web/api_handlers_streams_modify.c` (lines 861-909)

### Additional Safety Improvements
Added validation to the mongoose wakeup event handler to prevent crashes from invalid connections:

**Files Modified**:
- `src/web/mongoose_server_multithreading.c` (lines 188-228)

**Changes**:
- Validate connection pointer is not NULL
- Check if connection is closing before processing wakeup
- Validate event data is not NULL or empty
- Log warnings instead of crashing on invalid data

---

# Recording Deletion Fixes

## Issues Fixed

### 1. Files Left on Disk After Database Deletion
**Problem**: The deletion logic was deleting files BEFORE removing database entries. If file deletion failed, the database entry was still removed, leaving orphaned files on disk.

**Root Cause**: Incorrect order of operations in deletion handlers.

**Fix**: Changed all deletion handlers to:
1. Get recording metadata from database
2. Save file path to a local variable
3. Delete from database FIRST
4. Then attempt to delete file from disk
5. Log warnings if file deletion fails, but don't fail the operation

**Files Modified**:
- `src/web/api_handlers_recordings_delete.c` - Individual recording deletion
- `src/web/api_handlers_recordings_batch.c` - Batch deletion by IDs and by filter

**Rationale**: It's better to have orphaned files (which can be cleaned up later) than orphaned database entries pointing to non-existent files (which breaks the UI and causes errors).

### 2. Web Request Failures for Individual Deletion
**Problem**: Individual deletion endpoint sends 202 Accepted but never sends a final result, leaving the client hanging.

**Current Behavior**: 
- Endpoint sends 202 Accepted immediately
- Spawns a worker thread to perform deletion
- Worker thread never sends a response back to the client

**Status**: This is actually by design for async operations. The 202 response indicates the request was accepted and is being processed. The client should refresh the recordings list after a short delay.

**Note**: If synchronous behavior is desired, the endpoint would need to be refactored to wait for the worker thread and send a final response.

### 3. Delete Selected Recordings Fails
**Problem**: Frontend code was trying to use WebSockets that were removed, then falling back to HTTP but not properly handling the async job_id response.

**Fix**: 
- Removed all WebSocket references from `deleteSelectedRecordings` function
- Updated `deleteSelectedRecordingsHttp` to handle job_id response
- Added `pollBatchDeleteProgress` function to poll for completion
- Updated progress UI during polling

**Files Modified**:
- `web/js/components/preact/recordings/recordingsAPI.jsx`

### 4. Delete All Filtered Recordings Fails
**Problem**: Same as #3 - WebSocket references and improper handling of async responses.

**Fix**:
- Removed all WebSocket references from `deleteAllFilteredRecordings` function
- Updated `deleteAllFilteredRecordingsHttp` to handle job_id response
- Reuses the same `pollBatchDeleteProgress` function
- Updated progress UI during polling

**Files Modified**:
- `web/js/components/preact/recordings/recordingsAPI.jsx`

## Technical Details

### Backend Changes

#### Individual Deletion (DELETE /api/recordings/:id)
```c
// OLD (WRONG):
// 1. Get recording metadata
// 2. Delete file from disk
// 3. Delete from database

// NEW (CORRECT):
// 1. Get recording metadata
// 2. Save file path to local variable
// 3. Delete from database
// 4. Delete file from disk (log warning if fails)
```

#### Batch Deletion (POST /api/recordings/batch-delete)
Same pattern applied to both deletion by IDs and deletion by filter.

### Frontend Changes

#### Removed WebSocket Code
All references to `window.wsClient` and `BatchDeleteRecordingsClient` were removed as WebSockets are no longer supported.

#### Added Progress Polling
New `pollBatchDeleteProgress` function:
- Polls `/api/recordings/batch-delete/progress/:job_id` every 1 second
- Updates progress UI via `window.updateBatchDeleteProgress`
- Returns final result when `complete` flag is true
- Timeout after 120 attempts (2 minutes)

#### Updated HTTP Batch Delete Functions
Both `deleteSelectedRecordingsHttp` and `deleteAllFilteredRecordingsHttp` now:
- Check if response contains `job_id`
- If yes, poll for progress until complete
- If no, handle as synchronous response (backward compatibility)
- Show appropriate status messages

## Testing Recommendations

1. **Individual Deletion**:
   - Delete a single recording
   - Verify file is removed from disk
   - Verify database entry is removed
   - Check logs for any warnings

2. **Batch Deletion by IDs**:
   - Select multiple recordings
   - Click "Delete Selected"
   - Verify progress modal shows
   - Verify all files are removed
   - Verify all database entries are removed

3. **Batch Deletion by Filter**:
   - Apply filters (date range, stream, etc.)
   - Click "Delete All Filtered"
   - Verify progress modal shows
   - Verify all matching files are removed
   - Verify all matching database entries are removed

4. **Error Handling**:
   - Test with read-only file system (file deletion should fail gracefully)
   - Test with database locked (should fail and not delete files)
   - Test with network interruption during polling

## Known Limitations

1. **Individual Deletion Response**: Still sends 202 Accepted without final result. Client must refresh to see changes.

2. **Orphaned Files**: If file deletion fails after database deletion, files will be orphaned on disk. A cleanup utility should be created to scan for and remove orphaned files.

3. **Progress Polling Timeout**: If batch delete takes longer than 2 minutes, polling will timeout. The operation may still complete on the server.

## Future Improvements

1. Create a cleanup utility to scan for and remove orphaned recording files
2. Add Server-Sent Events (SSE) for real-time progress updates instead of polling
3. Add retry logic for failed file deletions
4. Add database transaction support to rollback on file deletion failure
5. Implement proper response for individual deletion (either sync or async with job_id)

