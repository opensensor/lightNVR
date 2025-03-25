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
#include "database/database_manager.h"

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
 * Update to stop_transcode_stream to handle decoupled MP4 recording
 */
int stop_transcode_stream(const char *stream_name) {
    if (!stream_name) {
        return -1;
    }

    // First stop the HLS stream
    int result = stop_hls_stream(stream_name);
    if (result != 0) {
        log_warn("Failed to stop HLS stream: %s", stream_name);
        // Continue anyway
    }

    // Also stop any separate MP4 recording for this stream
    unregister_mp4_writer_for_stream(stream_name);

    return result;
}
