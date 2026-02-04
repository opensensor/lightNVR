#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <pthread.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <fcntl.h>
#include <errno.h>
#include <time.h>
#include <dirent.h>

#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <libavutil/avutil.h>
#include <libavutil/imgutils.h>
#include <libswscale/swscale.h>

#include "core/logger.h"
#include "core/config.h"
#include "video/stream_manager.h"
#include "video/streams.h"
#include "video/stream_reader.h"
#include "video/detection_integration.h"
#include "video/unified_detection_thread.h"

// Structure to track detection stream readers
typedef struct {
    char stream_name[MAX_STREAM_NAME];
    stream_reader_ctx_t *reader_ctx;
    int detection_interval;
    int frame_counter;
    pthread_mutex_t mutex;
} detection_stream_t;

// Array to store detection stream readers
static detection_stream_t detection_streams[MAX_STREAMS];
static pthread_mutex_t detection_streams_mutex = PTHREAD_MUTEX_INITIALIZER;

/**
 * Initialize detection stream system
 */
void init_detection_stream_system(void) {
    pthread_mutex_lock(&detection_streams_mutex);
    
    for (int i = 0; i < MAX_STREAMS; i++) {
        memset(&detection_streams[i], 0, sizeof(detection_stream_t));
        pthread_mutex_init(&detection_streams[i].mutex, NULL);
    }
    
    pthread_mutex_unlock(&detection_streams_mutex);
    
    log_info("Detection stream system initialized");
}

/**
 * Shutdown detection stream system
 * 
 *  Modified to work with the new architecture where the HLS streaming thread
 * handles detection instead of using a dedicated stream reader.
 */
void shutdown_detection_stream_system(void) {
    log_info("Shutting down detection stream system...");
    pthread_mutex_lock(&detection_streams_mutex);
    
    for (int i = 0; i < MAX_STREAMS; i++) {
        pthread_mutex_lock(&detection_streams[i].mutex);
        
        // Log active detection streams
        if (detection_streams[i].stream_name[0] != '\0') {
            log_info("Disabling detection for stream %s during shutdown", 
                    detection_streams[i].stream_name);
        }
        
        // Clear the detection stream data
        detection_streams[i].reader_ctx = NULL;
        detection_streams[i].stream_name[0] = '\0';
        detection_streams[i].detection_interval = 0;
        detection_streams[i].frame_counter = 0;
        
        pthread_mutex_unlock(&detection_streams[i].mutex);
        
        // Destroy the mutex
        pthread_mutex_destroy(&detection_streams[i].mutex);
    }
    
    pthread_mutex_unlock(&detection_streams_mutex);
    
    log_info("Detection stream system shutdown complete");
}

// REMOVED: detection_packet_callback function is no longer needed since we're using the HLS streaming thread for detection

/**
 * Start a detection stream reader for a stream
 * 
 *  Modified to work with the new architecture where the HLS streaming thread
 * handles detection instead of using a dedicated stream reader.
 * 
 * @param stream_name The name of the stream
 * @param detection_interval How often to process frames (e.g., every 10 frames)
 * @return 0 on success, -1 on error
 */
int start_detection_stream_reader(const char *stream_name, int detection_interval) {
    log_info("Starting detection for stream %s with interval %d (using HLS streaming thread)", 
             stream_name, detection_interval);
    if (!stream_name) {
        log_error("Invalid stream name for start_detection_stream_reader");
        return -1;
    }
    
    // We'll use a timestamp-based approach in the detection thread itself
    // to delay processing until the system has had time to initialize
    
    // Verify stream exists
    stream_handle_t stream = get_stream_by_name(stream_name);
    if (!stream) {
        log_error("Stream %s not found", stream_name);
        return -1;
    }
    
    pthread_mutex_lock(&detection_streams_mutex);
    
    // Find an empty slot or existing entry for this stream
    int slot = -1;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (detection_streams[i].stream_name[0] == '\0') {
            if (slot == -1) {
                slot = i;
            }
        } else if (strcmp(detection_streams[i].stream_name, stream_name) == 0) {
            // Stream already has detection enabled, update it
            slot = i;
            break;
        }
    }
    
    if (slot == -1) {
        log_error("No available slots for detection");
        pthread_mutex_unlock(&detection_streams_mutex);
        return -1;
    }
    
    pthread_mutex_lock(&detection_streams[slot].mutex);
    
    //  No longer need to start a dedicated stream reader
    // Just store the detection configuration
    
    // Initialize detection stream
    strncpy(detection_streams[slot].stream_name, stream_name, MAX_STREAM_NAME - 1);
    detection_streams[slot].detection_interval = detection_interval;
    detection_streams[slot].frame_counter = 0;
    
    //  Set reader_ctx to a non-NULL value to indicate detection is enabled
    // This is just a marker, we don't actually use the reader
    detection_streams[slot].reader_ctx = (stream_reader_ctx_t*)1;
    
    log_info("Detection enabled for stream %s with interval %d", 
             stream_name, detection_interval);
    
    pthread_mutex_unlock(&detection_streams[slot].mutex);
    pthread_mutex_unlock(&detection_streams_mutex);
    
    // The unified detection thread is now started separately via start_unified_detection_thread()
    // This function just marks the stream as detection-enabled in the tracking array
    log_info("Detection stream reader marked as enabled for stream %s", stream_name);

    // Check if unified detection thread is already running
    if (!is_unified_detection_running(stream_name)) {
        log_info("Unified detection thread not yet running for stream %s - will be started by detection system", stream_name);
    } else {
        log_info("Unified detection thread already running for stream %s", stream_name);
    }

    return 0;
}

/**
 * Stop a detection stream reader for a stream
 * 
 *  Modified to work with the new architecture where the HLS streaming thread
 * handles detection instead of using a dedicated stream reader.
 * 
 * @param stream_name The name of the stream
 * @return 0 on success, -1 on error
 */
int stop_detection_stream_reader(const char *stream_name) {
    if (!stream_name) {
        log_error("Invalid stream name for stop_detection_stream_reader");
        return -1;
    }
    
    log_info("Disabling detection for stream %s", stream_name);
    
    pthread_mutex_lock(&detection_streams_mutex);
    
    // Find the detection stream for this stream
    int slot = -1;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (detection_streams[i].stream_name[0] != '\0' && 
            strcmp(detection_streams[i].stream_name, stream_name) == 0) {
            slot = i;
            break;
        }
    }
    
    if (slot == -1) {
        log_warn("No detection configuration found for stream %s", stream_name);
        pthread_mutex_unlock(&detection_streams_mutex);
        return -1;
    }
    
    pthread_mutex_lock(&detection_streams[slot].mutex);
    
    //  No need to stop a stream reader, just clear the configuration
    
    // Clear the detection stream data
    detection_streams[slot].reader_ctx = NULL;
    detection_streams[slot].stream_name[0] = '\0';
    detection_streams[slot].detection_interval = 0;
    detection_streams[slot].frame_counter = 0;
    
    pthread_mutex_unlock(&detection_streams[slot].mutex);
    pthread_mutex_unlock(&detection_streams_mutex);
    
    log_info("Detection disabled for stream %s", stream_name);
    
    return 0;
}

/**
 * Check if a detection stream reader is running for a stream
 */
bool is_detection_stream_reader_running(const char *stream_name) {
    if (!stream_name) {
        return false;
    }
    
    pthread_mutex_lock(&detection_streams_mutex);
    
    // Find the detection stream for this stream
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (detection_streams[i].stream_name[0] != '\0' && 
            strcmp(detection_streams[i].stream_name, stream_name) == 0) {
            
            bool running = (detection_streams[i].reader_ctx != NULL);
            pthread_mutex_unlock(&detection_streams_mutex);

            // Also check if the unified detection thread is running
            bool thread_running = is_unified_detection_running(stream_name);

            log_info("Detection stream reader for %s: reader=%s, thread=%s",
                    stream_name, running ? "running" : "not running",
                    thread_running ? "running" : "not running");

            return running && thread_running;
        }
    }
    
    pthread_mutex_unlock(&detection_streams_mutex);
    return false;
}

/**
 * Get the detection interval for a stream
 * 
 * @param stream_name The name of the stream
 * @return The detection interval, or 15 (default) if not found
 */
int get_detection_interval(const char *stream_name) {
    if (!stream_name) {
        return 15; // Default interval
    }
    
    pthread_mutex_lock(&detection_streams_mutex);
    
    // Find the detection stream for this stream
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (detection_streams[i].stream_name[0] != '\0' && 
            strcmp(detection_streams[i].stream_name, stream_name) == 0 &&
            detection_streams[i].reader_ctx != NULL) {
            int interval = detection_streams[i].detection_interval;
            pthread_mutex_unlock(&detection_streams_mutex);
            return interval > 0 ? interval : 15; // Use default if interval is invalid
        }
    }
    
    pthread_mutex_unlock(&detection_streams_mutex);
    
    return 15; // Default interval
}

/**
 * Print status of all detection stream readers
 * This is useful for debugging detection issues
 */
void print_detection_stream_status(void) {
    pthread_mutex_lock(&detection_streams_mutex);

    log_info("=== Detection Stream Status ===");
    int active_count = 0;
    int running_threads = 0;

    for (int i = 0; i < MAX_STREAMS; i++) {
        if (detection_streams[i].stream_name[0] != '\0') {
            pthread_mutex_lock(&detection_streams[i].mutex);

            // Check if the unified detection thread is running
            bool thread_running = is_unified_detection_running(detection_streams[i].stream_name);

            if (thread_running) {
                running_threads++;
            }

            const char *status = detection_streams[i].reader_ctx ?
                (thread_running ? "ACTIVE (thread running)" : "CONFIGURED (thread not running)") :
                "INACTIVE";

            log_info("Stream %s: %s (interval: %d, frame counter: %d)",
                    detection_streams[i].stream_name,
                    status,
                    detection_streams[i].detection_interval,
                    detection_streams[i].frame_counter);

            // Log unified detection state if thread is running
            if (thread_running) {
                unified_detection_state_t state = get_unified_detection_state(detection_streams[i].stream_name);
                log_info("  - Unified detection state: %d", state);
            }

            if (detection_streams[i].reader_ctx) {
                active_count++;
            }

            pthread_mutex_unlock(&detection_streams[i].mutex);
        }
    }

    log_info("Total active detection streams: %d (running threads: %d)",
            active_count, running_threads);
    log_info("==============================");

    pthread_mutex_unlock(&detection_streams_mutex);
}
