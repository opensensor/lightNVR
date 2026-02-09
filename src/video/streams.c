/*
 * Main streams interface file that coordinates between different modules
 */
#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <pthread.h>
#include <sys/stat.h>
#include <sys/types.h>

#include "core/logger.h"
#include "core/config.h"
#include "video/stream_manager.h"
#include "video/streams.h"
#include "video/stream_state.h"
#include "database/database_manager.h"

#ifdef USE_GO2RTC
#include "video/go2rtc/go2rtc_integration.h"
#endif

/**
 * Get the HLS writer for a stream
 * 
 * This function returns the HLS writer for a stream, which can be used
 * to check if the stream is recording or to write packets to the HLS stream.
 * 
 * @param stream The stream handle
 * @return Pointer to the HLS writer, or NULL if not available
 */
hls_writer_t *get_stream_hls_writer(stream_handle_t stream) {
    if (!stream) {
        return NULL;
    }
    
    // Get the stream configuration to get the name
    stream_config_t config;
    if (get_stream_config(stream, &config) != 0) {
        return NULL;
    }
    
    // Get the stream state manager using the name
    stream_state_manager_t *state = get_stream_state_by_name(config.name);
    if (!state) {
        return NULL;
    }
    
    // Get the HLS writer from the stream state
    // The hls_ctx field in the stream_state_manager_t structure contains the HLS writer
    return (hls_writer_t *)state->hls_ctx;
}

/**
 * Get current streaming configuration from database
 * 
 * This function queries the database for all stream configurations
 * and returns a pointer to a static config_t structure containing
 * the latest stream configurations.
 * 
 * @return Pointer to a static config_t structure with the latest stream configurations
 */
config_t* get_streaming_config(void) {
    static config_t db_config;
    static pthread_mutex_t config_mutex = PTHREAD_MUTEX_INITIALIZER;
    
    pthread_mutex_lock(&config_mutex);
    
    // Use the global configuration instead of loading defaults
    // This ensures we get the latest configuration including the database path
    memcpy(&db_config, &g_config, sizeof(config_t));
    
    // Load stream configurations from database
    stream_config_t db_streams[MAX_STREAMS];
    int count = get_all_stream_configs(db_streams, MAX_STREAMS);
    
    if (count > 0) {
        // Copy stream configurations to the config structure
        for (int i = 0; i < count && i < MAX_STREAMS; i++) {
            memcpy(&db_config.streams[i], &db_streams[i], sizeof(stream_config_t));
        }
        db_config.max_streams = count;
    }
    
    // Log the storage path for debugging
    log_info("get_streaming_config: Using storage path: %s", db_config.storage_path);
    log_info("get_streaming_config: Using database path: %s", db_config.db_path);
    
    pthread_mutex_unlock(&config_mutex);
    
    return &db_config;
}

/**
 * go2rtc-aware HLS wrapper — start HLS for a stream.
 * When go2rtc is compiled in and available at runtime, uses go2rtc native HLS.
 * Otherwise falls back to the raw ffmpeg-based start_hls_stream().
 */
int stream_start_hls(const char *stream_name) {
#ifdef USE_GO2RTC
    return go2rtc_integration_start_hls(stream_name);
#else
    return start_hls_stream(stream_name);
#endif
}

/**
 * go2rtc-aware HLS wrapper — stop HLS for a stream.
 */
int stream_stop_hls(const char *stream_name) {
#ifdef USE_GO2RTC
    return go2rtc_integration_stop_hls(stream_name);
#else
    return stop_hls_stream(stream_name);
#endif
}

/**
 * go2rtc-aware HLS wrapper — restart HLS for a stream.
 * When using go2rtc native HLS, stops and re-starts via go2rtc integration.
 * Otherwise falls back to the raw ffmpeg restart_hls_stream().
 */
int stream_restart_hls(const char *stream_name) {
#ifdef USE_GO2RTC
    // Stop then start via go2rtc integration
    go2rtc_integration_stop_hls(stream_name);
    return go2rtc_integration_start_hls(stream_name);
#else
    return restart_hls_stream(stream_name);
#endif
}

/**
 * Update to stop_transcode_stream to handle decoupled MP4 recording
 */
int stop_transcode_stream(const char *stream_name) {
    if (!stream_name) {
        return -1;
    }

    int result = 0;

    // Use the go2rtc-aware wrapper to stop HLS
    result = stream_stop_hls(stream_name);
    if (result != 0) {
        log_warn("Failed to stop HLS stream: %s", stream_name);
        // Continue anyway
    }

#ifdef USE_GO2RTC
    // Also stop any separate MP4 recording for this stream using go2rtc integration if available
    if (go2rtc_integration_stop_recording(stream_name) != 0) {
        log_warn("Failed to stop MP4 recording using go2rtc integration: %s", stream_name);
        // Continue anyway
    }
#else
    // Also stop any separate MP4 recording for this stream
    unregister_mp4_writer_for_stream(stream_name);
#endif

    return result;
}
