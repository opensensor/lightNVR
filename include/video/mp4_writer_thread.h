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
} mp4_writer_thread_t;

// Function declarations
int mp4_writer_start_recording_thread(mp4_writer_t *writer, const char *rtsp_url);
void mp4_writer_stop_recording_thread(mp4_writer_t *writer);
int mp4_writer_is_recording(mp4_writer_t *writer);

#endif /* MP4_WRITER_THREAD_H */
