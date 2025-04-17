#ifndef HLS_UNIFIED_THREAD_H
#define HLS_UNIFIED_THREAD_H

#include <pthread.h>
#include <stdatomic.h>
#include <libavformat/avformat.h>
#include "core/config.h"
#include "video/hls_writer.h"
#include "video/stream_protocol.h"

// Stream thread state constants
typedef enum {
    HLS_THREAD_INITIALIZING = 0,
    HLS_THREAD_CONNECTING,
    HLS_THREAD_RUNNING,
    HLS_THREAD_RECONNECTING,
    HLS_THREAD_STOPPING,
    HLS_THREAD_STOPPED
} hls_thread_state_t;

/**
 * Unified HLS thread context structure
 * Combines the functionality of both hls_stream_ctx_t and hls_writer_thread_ctx_t
 */
typedef struct {
    // Stream identification
    char stream_name[MAX_STREAM_NAME];
    char rtsp_url[MAX_PATH_LENGTH];
    char output_path[MAX_PATH_LENGTH];

    // Thread management
    pthread_t thread;
    atomic_int running;
    int shutdown_component_id;

    // Stream configuration
    int protocol;  // STREAM_PROTOCOL_TCP or STREAM_PROTOCOL_UDP
    int segment_duration;

    // HLS writer (embedded directly instead of pointer)
    hls_writer_t *writer;

    // Connection state tracking
    atomic_int_fast64_t last_packet_time;
    atomic_int connection_valid;
    atomic_int consecutive_failures;
    atomic_int thread_state;  // Uses hls_thread_state_t values
} hls_unified_thread_ctx_t;

/**
 * Start HLS streaming for a stream using the unified thread approach
 * This function creates a new thread that handles all HLS streaming operations
 *
 * @param stream_name The name of the stream
 * @return 0 on success, negative on error
 */
int start_hls_unified_stream(const char *stream_name);

/**
 * Stop HLS streaming for a stream using the unified thread approach
 *
 * @param stream_name The name of the stream
 * @return 0 on success, negative on error
 */
int stop_hls_unified_stream(const char *stream_name);

/**
 * Force restart of HLS streaming for a stream using the unified thread approach
 *
 * @param stream_name The name of the stream
 * @return 0 on success, negative on error
 */
int restart_hls_unified_stream(const char *stream_name);

/**
 * These functions are defined in hls_api.c and are kept for backward compatibility
 */
int start_hls_stream(const char *stream_name);
int stop_hls_stream(const char *stream_name);
int restart_hls_stream(const char *stream_name);

/**
 * Check if HLS streaming is active for a stream
 *
 * @param stream_name The name of the stream
 * @return 1 if active, 0 if not
 */
int is_hls_stream_active(const char *stream_name);

/**
 * Thread function for the unified HLS thread
 * This function handles all HLS streaming operations for a single stream
 *
 * @param arg The thread context (hls_unified_thread_ctx_t*)
 * @return NULL
 */
void *hls_unified_thread_func(void *arg);

/**
 * Get the number of HLS thread restarts performed by the watchdog
 *
 * @return The number of restarts
 */
int get_hls_watchdog_restart_count(void);

/**
 * Initialize the HLS unified thread system
 * This function initializes the HLS unified thread system and starts the watchdog
 */
void init_hls_unified_thread_system(void);

/**
 * Clean up the HLS unified thread system
 * This function cleans up the HLS unified thread system and stops the watchdog
 */
void cleanup_hls_unified_thread_system(void);

#endif /* HLS_UNIFIED_THREAD_H */
