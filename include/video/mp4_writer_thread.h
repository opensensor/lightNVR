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
#include "core/config.h"
#include "video/mp4_writer.h"
#include "video/mp4_segment_recorder.h"

/**
 * Thread-related fields for the MP4 writer
 */
typedef struct {
    pthread_t thread;         // Recording thread
    int running;              // Flag indicating if the thread is running
    char rtsp_url[MAX_PATH_LENGTH]; // URL of the RTSP stream to record
    volatile sig_atomic_t shutdown_requested; // Flag indicating if shutdown was requested
    mp4_writer_t *writer;     // MP4 writer instance
    int segment_duration;     // Duration of each segment in seconds
    time_t last_segment_time; // Time when the last segment was created

    // Self-management fields
    int retry_count;          // Number of consecutive failures
    time_t last_retry_time;   // Time of the last retry attempt
    bool auto_restart;        // Whether to automatically restart on failure
} mp4_writer_thread_t;

/**
 * Start a recording thread that reads from the RTSP stream and writes to the MP4 file
 * This function creates a new thread that handles all the recording logic
 *
 * @param writer The MP4 writer instance
 * @param rtsp_url The URL of the RTSP stream to record
 * @return 0 on success, negative on error
 */
int mp4_writer_start_recording_thread(mp4_writer_t *writer, const char *rtsp_url);

/**
 * Stop the recording thread
 * This function signals the recording thread to stop and waits for it to exit
 *
 * @param writer The MP4 writer instance
 */
void mp4_writer_stop_recording_thread(mp4_writer_t *writer);

/**
 * Check if the recording thread is running
 *
 * @param writer The MP4 writer instance
 * @return 1 if running, 0 if not
 */
int mp4_writer_is_recording(mp4_writer_t *writer);

#endif /* MP4_WRITER_THREAD_H */
