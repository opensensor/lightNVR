#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>
#include <unistd.h>

#include "video/stream_manager.h"
#include "core/logger.h"
#include "core/config.h"
#include "video/streams.h"
#include "video/detection.h"

// Stream structure
typedef struct {
    stream_config_t config;
    stream_status_t status;
    stream_stats_t stats;
    pthread_mutex_t mutex;
    bool recording_enabled;
    bool detection_recording_enabled;
} stream_t;

// Global array of streams
static stream_t streams[MAX_STREAMS];
static pthread_mutex_t streams_mutex = PTHREAD_MUTEX_INITIALIZER;
static bool initialized = false;

/**
 * Initialize stream manager
 */
int init_stream_manager(int max_streams) {
    if (initialized) {
        return 0;  // Already initialized
    }
    
    pthread_mutex_lock(&streams_mutex);
    
    // Initialize streams array
    memset(streams, 0, sizeof(streams));
    for (int i = 0; i < MAX_STREAMS; i++) {
        pthread_mutex_init(&streams[i].mutex, NULL);
        streams[i].status = STREAM_STATUS_STOPPED;
        memset(&streams[i].stats, 0, sizeof(stream_stats_t));
        streams[i].recording_enabled = false;
        streams[i].detection_recording_enabled = false;
    }
    
    // Load stream configurations from config
    config_t *config = get_streaming_config();
    if (config) {
        for (int i = 0; i < config->max_streams && i < MAX_STREAMS; i++) {
            if (config->streams[i].name[0] != '\0') {
                memcpy(&streams[i].config, &config->streams[i], sizeof(stream_config_t));
            }
        }
    }
    
    initialized = true;
    pthread_mutex_unlock(&streams_mutex);
    
    log_info("Stream manager initialized");
    return 0;
}

/**
 * Shutdown stream manager
 */
void shutdown_stream_manager(void) {
    if (!initialized) {
        return;
    }
    
    pthread_mutex_lock(&streams_mutex);
    
    // Stop all streams
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (streams[i].config.name[0] != '\0' && streams[i].status == STREAM_STATUS_RUNNING) {
            pthread_mutex_unlock(&streams_mutex);
            stop_hls_stream(streams[i].config.name);
            pthread_mutex_lock(&streams_mutex);
            streams[i].status = STREAM_STATUS_STOPPED;
        }
    }
    
    // Destroy mutexes
    for (int i = 0; i < MAX_STREAMS; i++) {
        pthread_mutex_destroy(&streams[i].mutex);
    }
    
    initialized = false;
    pthread_mutex_unlock(&streams_mutex);
    
    log_info("Stream manager shutdown");
}

/**
 * Get stream by name
 */
stream_handle_t get_stream_by_name(const char *name) {
    if (!name || !initialized) {
        return NULL;
    }
    
    pthread_mutex_lock(&streams_mutex);
    
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (streams[i].config.name[0] != '\0' && strcmp(streams[i].config.name, name) == 0) {
            pthread_mutex_unlock(&streams_mutex);
            return (stream_handle_t)&streams[i];
        }
    }
    
    pthread_mutex_unlock(&streams_mutex);
    return NULL;
}

/**
 * Get stream configuration
 */
int get_stream_config(stream_handle_t stream, stream_config_t *config) {
    if (!stream || !config) {
        return -1;
    }
    
    stream_t *s = (stream_t *)stream;
    
    pthread_mutex_lock(&s->mutex);
    memcpy(config, &s->config, sizeof(stream_config_t));
    pthread_mutex_unlock(&s->mutex);
    
    return 0;
}

/**
 * Get stream status
 */
stream_status_t get_stream_status(stream_handle_t stream) {
    if (!stream) {
        return STREAM_STATUS_STOPPED; // Return STOPPED instead of UNKNOWN
    }
    
    stream_t *s = (stream_t *)stream;
    
    pthread_mutex_lock(&s->mutex);
    stream_status_t status = s->status;
    pthread_mutex_unlock(&s->mutex);
    
    return status;
}

/**
 * Set stream detection recording
 */
int set_stream_detection_recording(stream_handle_t stream, bool enabled, const char *model_path) {
    if (!stream) {
        return -1;
    }
    
    stream_t *s = (stream_t *)stream;
    
    pthread_mutex_lock(&s->mutex);
    
    // Update configuration
    s->config.detection_based_recording = enabled;
    
    if (model_path) {
        strncpy(s->config.detection_model, model_path, MAX_PATH_LENGTH - 1);
        s->config.detection_model[MAX_PATH_LENGTH - 1] = '\0';
    }
    
    pthread_mutex_unlock(&s->mutex);
    
    // Save configuration to config file
    config_t *config = get_streaming_config();
    if (config) {
        for (int i = 0; i < config->max_streams && i < MAX_STREAMS; i++) {
            if (strcmp(config->streams[i].name, s->config.name) == 0) {
                pthread_mutex_lock(&s->mutex);
                memcpy(&config->streams[i], &s->config, sizeof(stream_config_t));
                pthread_mutex_unlock(&s->mutex);
                break;
            }
        }
    }
    
    log_info("Set detection recording for stream %s: enabled=%s, model=%s", 
             s->config.name, enabled ? "true" : "false", model_path ? model_path : "none");
    
    return 0;
}

/**
 * Set stream detection parameters
 */
int set_stream_detection_params(stream_handle_t stream, int interval, float threshold, 
                               int pre_buffer, int post_buffer) {
    if (!stream) {
        return -1;
    }
    
    stream_t *s = (stream_t *)stream;
    
    pthread_mutex_lock(&s->mutex);
    
    // Update configuration
    if (interval > 0) {
        s->config.detection_interval = interval;
    }
    
    if (threshold >= 0.0f && threshold <= 1.0f) {
        s->config.detection_threshold = threshold;
    }
    
    if (pre_buffer >= 0) {
        s->config.pre_detection_buffer = pre_buffer;
    }
    
    if (post_buffer >= 0) {
        s->config.post_detection_buffer = post_buffer;
    }
    
    pthread_mutex_unlock(&s->mutex);
    
    // Save configuration to config file
    config_t *config = get_streaming_config();
    if (config) {
        for (int i = 0; i < config->max_streams && i < MAX_STREAMS; i++) {
            if (strcmp(config->streams[i].name, s->config.name) == 0) {
                pthread_mutex_lock(&s->mutex);
                memcpy(&config->streams[i], &s->config, sizeof(stream_config_t));
                pthread_mutex_unlock(&s->mutex);
                break;
            }
        }
    }
    
    log_info("Set detection parameters for stream %s", s->config.name);
    
    return 0;
}

/**
 * Add a new stream
 */
stream_handle_t add_stream(const stream_config_t *config) {
    if (!config || !initialized) {
        return NULL;
    }
    
    pthread_mutex_lock(&streams_mutex);
    
    // Find an empty slot
    int slot = -1;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (streams[i].config.name[0] == '\0') {
            slot = i;
            break;
        }
    }
    
    if (slot == -1) {
        log_error("No available slots for new stream");
        pthread_mutex_unlock(&streams_mutex);
        return NULL;
    }
    
    // Check if stream with same name already exists
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (i != slot && streams[i].config.name[0] != '\0' && 
            strcmp(streams[i].config.name, config->name) == 0) {
            log_error("Stream with name '%s' already exists", config->name);
            pthread_mutex_unlock(&streams_mutex);
            return NULL;
        }
    }
    
    // Initialize the stream
    pthread_mutex_lock(&streams[slot].mutex);
    memcpy(&streams[slot].config, config, sizeof(stream_config_t));
    streams[slot].status = STREAM_STATUS_STOPPED;
    memset(&streams[slot].stats, 0, sizeof(stream_stats_t));
    streams[slot].recording_enabled = config->record;
    streams[slot].detection_recording_enabled = config->detection_based_recording;
    pthread_mutex_unlock(&streams[slot].mutex);
    
    // Save to config
    config_t *global_config = get_streaming_config();
    if (global_config) {
        for (int i = 0; i < global_config->max_streams && i < MAX_STREAMS; i++) {
            if (global_config->streams[i].name[0] == '\0') {
                memcpy(&global_config->streams[i], config, sizeof(stream_config_t));
                break;
            }
        }
    }
    
    log_info("Added stream '%s' in slot %d", config->name, slot);
    pthread_mutex_unlock(&streams_mutex);
    
    return (stream_handle_t)&streams[slot];
}

/**
 * Remove a stream
 */
int remove_stream(stream_handle_t handle) {
    if (!handle || !initialized) {
        return -1;
    }
    
    stream_t *s = (stream_t *)handle;
    
    // First stop the stream if it's running
    if (s->status == STREAM_STATUS_RUNNING) {
        stop_stream(handle);
    }
    
    pthread_mutex_lock(&streams_mutex);
    
    // Find the stream in the array
    int slot = -1;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (&streams[i] == s) {
            slot = i;
            break;
        }
    }
    
    if (slot == -1) {
        log_error("Stream not found in array");
        pthread_mutex_unlock(&streams_mutex);
        return -1;
    }
    
    // Save stream name for logging
    char stream_name[MAX_STREAM_NAME];
    strncpy(stream_name, s->config.name, MAX_STREAM_NAME - 1);
    stream_name[MAX_STREAM_NAME - 1] = '\0';
    
    // Clear the stream slot
    pthread_mutex_lock(&s->mutex);
    memset(&s->config, 0, sizeof(stream_config_t));
    s->status = STREAM_STATUS_STOPPED;
    memset(&s->stats, 0, sizeof(stream_stats_t));
    s->recording_enabled = false;
    s->detection_recording_enabled = false;
    pthread_mutex_unlock(&s->mutex);
    
    // Remove from config
    config_t *global_config = get_streaming_config();
    if (global_config) {
        for (int i = 0; i < global_config->max_streams && i < MAX_STREAMS; i++) {
            if (strcmp(global_config->streams[i].name, stream_name) == 0) {
                memset(&global_config->streams[i], 0, sizeof(stream_config_t));
                break;
            }
        }
    }
    
    log_info("Removed stream '%s' from slot %d", stream_name, slot);
    pthread_mutex_unlock(&streams_mutex);
    
    return 0;
}

/**
 * Start a stream
 */
int start_stream(stream_handle_t handle) {
    if (!handle || !initialized) {
        return -1;
    }
    
    stream_t *s = (stream_t *)handle;
    
    pthread_mutex_lock(&s->mutex);
    
    // Check if already running
    if (s->status == STREAM_STATUS_RUNNING) {
        pthread_mutex_unlock(&s->mutex);
        log_info("Stream '%s' is already running", s->config.name);
        return 0;
    }
    
    // Update status to starting
    s->status = STREAM_STATUS_STARTING;
    
    // Get stream name for logging
    char stream_name[MAX_STREAM_NAME];
    strncpy(stream_name, s->config.name, MAX_STREAM_NAME - 1);
    stream_name[MAX_STREAM_NAME - 1] = '\0';
    
    pthread_mutex_unlock(&s->mutex);
    
    // Start HLS stream
    int result = start_hls_stream(stream_name);
    if (result != 0) {
        pthread_mutex_lock(&s->mutex);
        s->status = STREAM_STATUS_ERROR;
        pthread_mutex_unlock(&s->mutex);
        log_error("Failed to start HLS stream '%s'", stream_name);
        return -1;
    }
    
    // Update status to running
    pthread_mutex_lock(&s->mutex);
    s->status = STREAM_STATUS_RUNNING;
    pthread_mutex_unlock(&s->mutex);
    
    log_info("Started stream '%s'", stream_name);
    return 0;
}

/**
 * Stop a stream
 */
int stop_stream(stream_handle_t handle) {
    if (!handle || !initialized) {
        return -1;
    }
    
    stream_t *s = (stream_t *)handle;
    
    pthread_mutex_lock(&s->mutex);
    
    // Check if already stopped
    if (s->status == STREAM_STATUS_STOPPED) {
        pthread_mutex_unlock(&s->mutex);
        log_info("Stream '%s' is already stopped", s->config.name);
        return 0;
    }
    
    // Get stream name for logging
    char stream_name[MAX_STREAM_NAME];
    strncpy(stream_name, s->config.name, MAX_STREAM_NAME - 1);
    stream_name[MAX_STREAM_NAME - 1] = '\0';
    
    pthread_mutex_unlock(&s->mutex);
    
    // Stop HLS stream
    int result = stop_hls_stream(stream_name);
    if (result != 0) {
        log_warn("Failed to stop HLS stream '%s'", stream_name);
        // Continue anyway
    }
    
    // Update status to stopped
    pthread_mutex_lock(&s->mutex);
    s->status = STREAM_STATUS_STOPPED;
    pthread_mutex_unlock(&s->mutex);
    
    log_info("Stopped stream '%s'", stream_name);
    return 0;
}

/**
 * Get stream by index
 */
stream_handle_t get_stream_by_index(int index) {
    if (index < 0 || index >= MAX_STREAMS || !initialized) {
        return NULL;
    }
    
    pthread_mutex_lock(&streams_mutex);
    
    if (streams[index].config.name[0] == '\0') {
        pthread_mutex_unlock(&streams_mutex);
        return NULL;
    }
    
    pthread_mutex_unlock(&streams_mutex);
    return (stream_handle_t)&streams[index];
}

/**
 * Get number of active streams
 */
int get_active_stream_count(void) {
    if (!initialized) {
        return 0;
    }
    
    pthread_mutex_lock(&streams_mutex);
    
    int count = 0;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (streams[i].config.name[0] != '\0' && streams[i].status == STREAM_STATUS_RUNNING) {
            count++;
        }
    }
    
    pthread_mutex_unlock(&streams_mutex);
    return count;
}

/**
 * Get total number of streams
 */
int get_total_stream_count(void) {
    if (!initialized) {
        return 0;
    }
    
    pthread_mutex_lock(&streams_mutex);
    
    int count = 0;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (streams[i].config.name[0] != '\0') {
            count++;
        }
    }
    
    pthread_mutex_unlock(&streams_mutex);
    return count;
}

/**
 * Get stream statistics
 */
int get_stream_stats(stream_handle_t handle, stream_stats_t *stats) {
    if (!handle || !stats || !initialized) {
        return -1;
    }
    
    stream_t *s = (stream_t *)handle;
    
    pthread_mutex_lock(&s->mutex);
    memcpy(stats, &s->stats, sizeof(stream_stats_t));
    pthread_mutex_unlock(&s->mutex);
    
    return 0;
}

/**
 * Set stream priority
 */
int set_stream_priority(stream_handle_t handle, int priority) {
    if (!handle || !initialized) {
        return -1;
    }
    
    stream_t *s = (stream_t *)handle;
    
    pthread_mutex_lock(&s->mutex);
    s->config.priority = priority;
    pthread_mutex_unlock(&s->mutex);
    
    // Update config
    config_t *global_config = get_streaming_config();
    if (global_config) {
        for (int i = 0; i < global_config->max_streams && i < MAX_STREAMS; i++) {
            if (strcmp(global_config->streams[i].name, s->config.name) == 0) {
                global_config->streams[i].priority = priority;
                break;
            }
        }
    }
    
    log_info("Set priority for stream '%s' to %d", s->config.name, priority);
    return 0;
}

/**
 * Enable/disable recording for a stream
 */
int set_stream_recording(stream_handle_t handle, bool enable) {
    if (!handle || !initialized) {
        return -1;
    }
    
    stream_t *s = (stream_t *)handle;
    
    pthread_mutex_lock(&s->mutex);
    s->config.record = enable;
    s->recording_enabled = enable;
    pthread_mutex_unlock(&s->mutex);
    
    // Update config
    config_t *global_config = get_streaming_config();
    if (global_config) {
        for (int i = 0; i < global_config->max_streams && i < MAX_STREAMS; i++) {
            if (strcmp(global_config->streams[i].name, s->config.name) == 0) {
                global_config->streams[i].record = enable;
                break;
            }
        }
    }
    
    log_info("Set recording for stream '%s' to %s", s->config.name, enable ? "enabled" : "disabled");
    return 0;
}
