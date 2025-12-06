/**
 * MP4 Writer Thread Header
 * 
 * This module handles the thread-related functionality for MP4 recording.
 * It's responsible for:
 * - Managing RTSP recording threads
 * - Handling thread lifecycle (start, stop, etc.)
 * - Managing thread resources
 */

#ifndef MP4_WRITER_THREAD_H
#define MP4_WRITER_THREAD_H

#include <pthread.h>
#include <signal.h>
#include <stdbool.h>
#include <stdatomic.h>
#include "core/config.h"

// Forward declaration
typedef struct mp4_writer mp4_writer_t;

// Forward declaration for AVFormatContext
struct AVFormatContext;

/**
 * Structure to track segment information per stream
 */
typedef struct {
    int segment_index;
    bool has_audio;
    bool last_frame_was_key;  // Flag to indicate if the last frame of previous segment was a key frame
} segment_info_t;

/**
 * Thread-related fields for the MP4 writer
 */
typedef struct {
    pthread_t thread;         // Recording thread
    int running;              // Flag indicating if the thread is running
    char rtsp_url[MAX_PATH_LENGTH]; // URL of the RTSP stream to record
    atomic_int shutdown_requested;  // Flag indicating if shutdown was requested
    mp4_writer_t *writer;     // MP4 writer instance
    int segment_duration;     // Duration of each segment in seconds
    time_t last_segment_time; // Time when the last segment was created

    // Self-management fields
    int retry_count;          // Number of consecutive failures
    time_t last_retry_time;   // Time of the last retry attempt
    bool auto_restart;        // Whether to automatically restart on failure
    atomic_int force_reconnect;  // Flag to signal forced reconnection (e.g., after go2rtc restart)

    // Per-stream FFmpeg context and segment info (BUGFIX: moved from global static variables)
    struct AVFormatContext *input_ctx;  // Input context for this stream (reused between segments)
    segment_info_t segment_info;        // Segment information for this stream
    pthread_mutex_t context_mutex;      // Mutex to protect input_ctx and segment_info
} mp4_writer_thread_t;

// Function declarations
int mp4_writer_start_recording_thread(mp4_writer_t *writer, const char *rtsp_url);
void mp4_writer_stop_recording_thread(mp4_writer_t *writer);
int mp4_writer_is_recording(mp4_writer_t *writer);

/**
 * Signal the recording thread to force a reconnection
 * This is useful when the upstream source (e.g., go2rtc) has restarted
 * and the current connection is stale
 *
 * @param writer The MP4 writer instance
 */
void mp4_writer_signal_reconnect(mp4_writer_t *writer);

#endif /* MP4_WRITER_THREAD_H */
