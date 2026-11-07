# Thumbnail Request Queue Solution

## Problem

The previous approach used 503 responses and retry logic to handle thumbnail request overload:

1. **Backend**: Returns 503 "Service Unavailable" when too many ffmpeg processes are running
2. **Frontend**: Implements exponential backoff retry logic to handle 503s
3. **Result**: Ugly hack that treats symptoms rather than the root cause

### Timeline Comparison

**Before (No Queue):**
```
Time:     0ms    100ms   200ms   300ms   400ms
          |       |       |       |       |
Card 1 ───┼──❌ 503
Card 2 ───┼──❌ 503
Card 3 ───┼──❌ 503
Card 4 ───┼──❌ 503
Card 5 ───┼──❌ 503
Card 6 ───┼──❌ 503
          └─ All 6 requests hit backend simultaneously
             Backend limit exceeded → 503s

          (retry after 1-3s with jittered backoff)
```

**After (With Queue + Stagger):**
```
Time:     0ms    200ms   400ms   600ms   800ms   1000ms
          |       |       |       |       |       |
Card 1 ───┼──✓ 200 OK
Card 2 ───────┼──✓ 200 OK
Card 3 ───────────┼──✓ 200 OK
Card 4 ───────────────┼──✓ 200 OK
Card 5 ───────────────────┼──✓ 200 OK (after slot opens)
Card 6 ───────────────────────┼──✓ 200 OK (after slot opens)
          └─ Requests staggered 200ms apart
             Backend never overwhelmed → no 503s
```

### Why This Was Bad

- **Wasted bandwidth**: Failed requests still consume network resources
- **Poor UX**: Users see failed image loads before retries succeed
- **Complexity**: Retry logic with jittered backoff, cache busting, error states
- **Inefficient**: Server still gets overwhelmed before 503s kick in
- **Fragile**: Race conditions between concurrent retries

## Solution

**Control concurrency at the source** - implement a frontend request queue that limits how many thumbnail requests are in-flight at once.

### Architecture

```
┌─────────────────────────────────────────────────────────────┐
│ Frontend (RecordingsGrid)                                   │
│                                                              │
│  ┌──────────────┐                                           │
│  │ Card 1       │──┐                                        │
│  │ (visible)    │  │ HIGH priority                          │
│  └──────────────┘  │                                        │
│                    │                                        │
│  ┌──────────────┐  │    ┌─────────────────────────────┐   │
│  │ Card 2       │──┼───▶│  Request Queue              │   │
│  │ (visible)    │  │    │  - Max 6 concurrent         │───┼──▶ Backend
│  └──────────────┘  │    │  - Priority-based ordering  │   │   (never overwhelmed)
│                    │    │  - Progressive loading      │   │
│  ┌──────────────┐  │    └─────────────────────────────┘   │
│  │ Card 3       │──┘                                        │
│  │ (hover)      │    LOW priority                          │
│  └──────────────┘                                           │
└─────────────────────────────────────────────────────────────┘
```

### Implementation

#### 1. Request Queue (`web/js/request-queue.js`)

A generic priority queue that:
- Limits concurrent requests (default: 4 for thumbnails)
- Supports priority levels (HIGH, NORMAL, LOW)
- Processes requests in priority order
- Staggers request starts to prevent burst overload (200ms delay)
- Returns promises for easy integration

```javascript
import { queueThumbnailLoad, Priority } from './request-queue.js';

// High priority for visible content
queueThumbnailLoad(url, Priority.HIGH)
  .then(() => console.log('Loaded!'))
  .catch(err => console.error('Failed:', err));
```

#### 2. Updated RecordingsGrid Component

**Before:**
- All cards eagerly preload 3 thumbnails on mount → 3N requests
- Retry logic with exponential backoff
- Cache busting parameters
- Complex error handling

**After:**
- Cards load middle frame (index 1) on mount with HIGH priority
- Other frames (0, 2) preload on hover with LOW priority
- No retry logic needed - queue prevents overload
- Clean, simple error handling

### Why Stagger Request Starts?

Even with a concurrency limit, starting all N requests **simultaneously** can still overwhelm the backend because:

1. **Backend concurrency limit**: The backend has `MAX_CONCURRENT_THUMBNAIL_GENERATIONS = 6`
2. **Race condition**: If 6 requests arrive at the exact same moment, they all pass the concurrency check before any increment the counter
3. **Result**: 6+ ffmpeg processes start simultaneously → still get 503s

**Solution**: Add a 200ms delay between starting requests. This ensures:
- Requests arrive in a staggered pattern
- Backend concurrency check works correctly
- No burst overload, even on initial page load

### Benefits

1. **No 503s**: Server never gets overwhelmed, even on initial burst
2. **Better UX**: Visible thumbnails load first, background preloading happens progressively
3. **Simpler code**: Removed ~30 lines of retry logic, cache busting, etc.
4. **More efficient**: No wasted requests, no retries, optimal resource usage
5. **Predictable**: Queue ensures consistent behavior under load
6. **Smooth loading**: Staggered starts create a progressive loading effect

### Configuration

The queue can be tuned based on server capacity:

```javascript
// In request-queue.js
export const thumbnailQueue = new RequestQueue(
  4,    // maxConcurrent - max simultaneous requests
  200   // startDelay - ms delay between starting requests
);
```

**Parameters:**
- **maxConcurrent**: How many requests can be in-flight at once
  - **4**: Conservative default, works well for most systems
  - **6-8**: If backend has more CPU cores and faster storage
  - **2-3**: If running on low-power hardware (Raspberry Pi, etc.)

- **startDelay**: Milliseconds to wait between starting requests
  - **200ms**: Default, prevents burst overload
  - **100ms**: If backend is very fast
  - **300-500ms**: If backend is slow or under heavy load
  - **0ms**: No delay (not recommended for thumbnail generation)

### Testing

A test page is included at `web/test-request-queue.html` to verify queue behavior:

1. Start the server
2. Navigate to `http://localhost:8080/test-request-queue.html`
3. Click buttons to simulate 10, 50, or 100 concurrent requests
4. Observe that only 6 are active at once, rest are queued

### Migration Notes

**Removed from RecordingsGrid.jsx:**
- `retryRef` state
- `retryTimerRef` state  
- `cacheBust` state
- `MAX_RETRIES` constant
- Retry timer cleanup effect
- `handleImageError` retry logic

**Added:**
- Import of `queueThumbnailLoad` and `Priority`
- `imageLoaded` state for tracking load status
- Queue-based preloading with priority levels

### Future Enhancements

The queue system can be extended to:
- Other resource-intensive operations (video transcoding, etc.)
- Dynamic concurrency adjustment based on server load
- Request cancellation when components unmount
- Metrics/telemetry for monitoring queue performance

## Conclusion

This is a **proper architectural fix** rather than a band-aid. By controlling concurrency at the source, we eliminate the need for error handling, retries, and complex state management. The result is cleaner code, better performance, and a superior user experience.

