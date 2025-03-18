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
#include "video/stream_reader.h"
#include "video/stream_state.h"

// Stream structure
typedef struct {
    stream_config_t config;
    stream_status_t status;
    stream_stats_t stats;
    pthread_mutex_t mutex;
    bool recording_enabled;
    bool detection_recording_enabled;
    time_t last_detection_time;  // Added for detection-based recording
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
    
    // Initialize stream reader backend
    init_stream_reader_backend();
    
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
            char stream_name[MAX_STREAM_NAME];
            strncpy(stream_name, streams[i].config.name, MAX_STREAM_NAME - 1);
            stream_name[MAX_STREAM_NAME - 1] = '\0';
            
            bool streaming_enabled = streams[i].config.streaming_enabled;
            bool recording_enabled = streams[i].config.record;
            
            pthread_mutex_unlock(&streams_mutex);
            
            // Stop HLS stream if it was enabled
            if (streaming_enabled) {
                stop_hls_stream(stream_name);
                log_info("Stopped HLS streaming for '%s' during shutdown", stream_name);
            }
            
            // Stop MP4 recording if it was enabled
            if (recording_enabled) {
                stop_mp4_recording(stream_name);
                stop_recording(stream_name);
                log_info("Stopped recording for '%s' during shutdown", stream_name);
            }
            
            pthread_mutex_lock(&streams_mutex);
            streams[i].status = STREAM_STATUS_STOPPED;
        }
    }
    
    // Destroy mutexes
    for (int i = 0; i < MAX_STREAMS; i++) {
        pthread_mutex_destroy(&streams[i].mutex);
    }
    
    // Cleanup stream reader backend
    cleanup_stream_reader_backend();
    
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
        return STREAM_STATUS_UNKNOWN; // Now we can return UNKNOWN
    }
    
    stream_t *s = (stream_t *)stream;
    
    // First try to use the new state management system
    stream_state_manager_t *state = get_stream_state_by_name(s->config.name);
    if (state) {
        // Convert stream_state_t to stream_status_t
        stream_state_t state_value = get_stream_operational_state(state);
        
        // Map state to status
        switch (state_value) {
            case STREAM_STATE_INACTIVE:
                return STREAM_STATUS_STOPPED;
            case STREAM_STATE_STARTING:
                return STREAM_STATUS_STARTING;
            case STREAM_STATE_ACTIVE:
                return STREAM_STATUS_RUNNING;
            case STREAM_STATE_STOPPING:
                return STREAM_STATUS_STOPPING; // New status
            case STREAM_STATE_ERROR:
                return STREAM_STATUS_ERROR;
            case STREAM_STATE_RECONNECTING:
                return STREAM_STATUS_RECONNECTING;
            default:
                return STREAM_STATUS_UNKNOWN;
        }
    }
    
    // Fall back to the old system if the state manager is not available
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
    
    // Create a stream state manager for this stream
    stream_state_manager_t *state = get_stream_state_by_name(config->name);
    if (!state) {
        state = create_stream_state(config);
        if (!state) {
            log_warn("Failed to create stream state for '%s', some features may not work correctly", config->name);
        } else {
            log_info("Created stream state for '%s'", config->name);
        }
    }
    
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
    
    // Save stream name for timestamp tracker cleanup
    char stream_name_for_cleanup[MAX_STREAM_NAME];
    strncpy(stream_name_for_cleanup, s->config.name, MAX_STREAM_NAME - 1);
    stream_name_for_cleanup[MAX_STREAM_NAME - 1] = '\0';
    
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
    
    // Get stream name for logging
    char stream_name[MAX_STREAM_NAME];
    pthread_mutex_lock(&s->mutex);
    strncpy(stream_name, s->config.name, MAX_STREAM_NAME - 1);
    stream_name[MAX_STREAM_NAME - 1] = '\0';
    pthread_mutex_unlock(&s->mutex);
    
    // First try to use the new state management system
    stream_state_manager_t *state = get_stream_state_by_name(stream_name);
    if (state) {
        log_info("Using new state management system to start stream '%s'", stream_name);
        int result = start_stream_with_state(state);
        
        // Update the old status for backward compatibility
        if (result == 0) {
            pthread_mutex_lock(&s->mutex);
            s->status = STREAM_STATUS_RUNNING;
            pthread_mutex_unlock(&s->mutex);
        } else {
            pthread_mutex_lock(&s->mutex);
            s->status = STREAM_STATUS_ERROR;
            pthread_mutex_unlock(&s->mutex);
        }
        
        return result;
    }
    
    // Fall back to the old system if the state manager is not available
    log_warn("Falling back to legacy system to start stream '%s'", stream_name);
    
    pthread_mutex_lock(&s->mutex);
    
    // Check if already running
    if (s->status == STREAM_STATUS_RUNNING) {
        pthread_mutex_unlock(&s->mutex);
        log_info("Stream '%s' is already running", s->config.name);
        return 0;
    }
    
    // Update status to starting
    s->status = STREAM_STATUS_STARTING;
    
    // Get streaming_enabled flag
    bool streaming_enabled = s->config.streaming_enabled;
    
    // Get recording_enabled flag
    bool recording_enabled = s->config.record;
    
    pthread_mutex_unlock(&s->mutex);
    
    // Track if any component started successfully
    bool any_component_started = false;
    
    // Start HLS stream only if streaming is enabled
    int hls_result = 0;
    if (streaming_enabled) {
        hls_result = start_hls_stream(stream_name);
        if (hls_result != 0) {
            log_error("Failed to start HLS stream '%s'", stream_name);
        } else {
            log_info("Started HLS streaming for '%s'", stream_name);
            any_component_started = true;
        }
    } else {
        log_info("Streaming disabled for '%s', not starting HLS stream", stream_name);
    }
    
    // Start recording if enabled - completely independent of streaming status
    int mp4_result = 0;
    if (recording_enabled) {
        // Start MP4 recording directly
        mp4_result = start_mp4_recording(stream_name);
        if (mp4_result != 0) {
            log_error("Failed to start MP4 recording for '%s'", stream_name);
        } else {
            log_info("Started recording for '%s'", stream_name);
            any_component_started = true;
        }
    }
    
    // Update status based on results
    pthread_mutex_lock(&s->mutex);
    if (any_component_started) {
        s->status = STREAM_STATUS_RUNNING;
        log_info("Stream '%s' is now running", stream_name);
    } else {
        s->status = STREAM_STATUS_ERROR;
        log_error("Failed to start any components for stream '%s'", stream_name);
        pthread_mutex_unlock(&s->mutex);
        return -1;
    }
    pthread_mutex_unlock(&s->mutex);
    
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
    
    // Get stream name for logging
    char stream_name[MAX_STREAM_NAME];
    pthread_mutex_lock(&s->mutex);
    strncpy(stream_name, s->config.name, MAX_STREAM_NAME - 1);
    stream_name[MAX_STREAM_NAME - 1] = '\0';
    pthread_mutex_unlock(&s->mutex);
    
    // First try to use the new state management system
    stream_state_manager_t *state = get_stream_state_by_name(stream_name);
    if (state) {
        log_info("Using new state management system to stop stream '%s'", stream_name);
        int result = stop_stream_with_state(state, true); // Wait for completion
        
        // Update the old status for backward compatibility
        if (result == 0) {
            pthread_mutex_lock(&s->mutex);
            s->status = STREAM_STATUS_STOPPED;
            pthread_mutex_unlock(&s->mutex);
        }
        
        return result;
    }
    
    // Fall back to the old system if the state manager is not available
    log_warn("Falling back to legacy system to stop stream '%s'", stream_name);
    
    pthread_mutex_lock(&s->mutex);
    
    // Check if already stopped
    if (s->status == STREAM_STATUS_STOPPED) {
        pthread_mutex_unlock(&s->mutex);
        log_info("Stream '%s' is already stopped", s->config.name);
        return 0;
    }
    
    // Update status to stopping
    s->status = STREAM_STATUS_STOPPING;
    
    // Get streaming_enabled flag
    bool streaming_enabled = s->config.streaming_enabled;
    
    // Get recording_enabled flag
    bool recording_enabled = s->config.record;
    
    pthread_mutex_unlock(&s->mutex);
    
    // Stop HLS stream if it was started
    if (streaming_enabled) {
        int result = stop_hls_stream(stream_name);
        if (result != 0) {
            log_warn("Failed to stop HLS stream '%s'", stream_name);
            // Continue anyway
        } else {
            log_info("Stopped HLS streaming for '%s'", stream_name);
        }
    }
    
    // Stop recording if it was started
    if (recording_enabled) {
        // First stop the MP4 recording directly
        int mp4_result = stop_mp4_recording(stream_name);
        if (mp4_result != 0) {
            log_warn("Failed to stop MP4 recording for '%s'", stream_name);
            // Continue anyway
        } else {
            log_info("Stopped MP4 recording for '%s'", stream_name);
        }
        
        // Then stop the recording in the database
        stop_recording(stream_name);
        log_info("Stopped recording metadata for '%s'", stream_name);
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
 * Set stream recording
 */
int set_stream_recording(stream_handle_t handle, bool enable) {
    if (!handle || !initialized) {
        return -1;
    }
    
    stream_t *s = (stream_t *)handle;
    
    // Get stream name for logging
    char stream_name[MAX_STREAM_NAME];
    pthread_mutex_lock(&s->mutex);
    strncpy(stream_name, s->config.name, MAX_STREAM_NAME - 1);
    stream_name[MAX_STREAM_NAME - 1] = '\0';
    pthread_mutex_unlock(&s->mutex);
    
    // First try to use the new state management system
    stream_state_manager_t *state = get_stream_state_by_name(stream_name);
    if (state) {
        log_info("Using new state management system to set recording for stream '%s' to %s", 
                stream_name, enable ? "enabled" : "disabled");
        
        int result = set_stream_feature(state, "recording", enable);
        
        // Update the old config for backward compatibility
        if (result == 0) {
            pthread_mutex_lock(&s->mutex);
            s->config.record = enable;
            s->recording_enabled = enable;
            pthread_mutex_unlock(&s->mutex);
            
            // Update config
            config_t *global_config = get_streaming_config();
            if (global_config) {
                for (int i = 0; i < global_config->max_streams && i < MAX_STREAMS; i++) {
                    if (strcmp(global_config->streams[i].name, stream_name) == 0) {
                        global_config->streams[i].record = enable;
                        break;
                    }
                }
            }
        }
        
        return result;
    }
    
    // Fall back to the old system if the state manager is not available
    log_warn("Falling back to legacy system to set recording for stream '%s'", stream_name);
    
    pthread_mutex_lock(&s->mutex);
    s->config.record = enable;
    s->recording_enabled = enable;
    pthread_mutex_unlock(&s->mutex);
    
    // Update config
    config_t *global_config = get_streaming_config();
    if (global_config) {
        for (int i = 0; i < global_config->max_streams && i < MAX_STREAMS; i++) {
            if (strcmp(global_config->streams[i].name, stream_name) == 0) {
                global_config->streams[i].record = enable;
                break;
            }
        }
    }
    
    log_info("Set recording for stream '%s' to %s", stream_name, enable ? "enabled" : "disabled");
    return 0;
}

/**
 * Set the last detection time for a stream
 */
int set_stream_last_detection_time(stream_handle_t handle, time_t time) {
    if (!handle || !initialized) {
        return -1;
    }
    
    stream_t *s = (stream_t *)handle;
    
    pthread_mutex_lock(&s->mutex);
    // Assuming there's a last_detection_time field in the stream structure
    // If not, you might need to add it to the stream_t structure
    s->last_detection_time = time;
    pthread_mutex_unlock(&s->mutex);
    
    log_info("Set last detection time for stream '%s' to %ld", s->config.name, (long)time);
    return 0;
}

/**
 * Set streaming enabled flag for a stream
 */
int set_stream_streaming_enabled(stream_handle_t handle, bool enabled) {
    if (!handle || !initialized) {
        return -1;
    }
    
    stream_t *s = (stream_t *)handle;
    
    // Get stream name for logging
    char stream_name[MAX_STREAM_NAME];
    pthread_mutex_lock(&s->mutex);
    strncpy(stream_name, s->config.name, MAX_STREAM_NAME - 1);
    stream_name[MAX_STREAM_NAME - 1] = '\0';
    pthread_mutex_unlock(&s->mutex);
    
    // First try to use the new state management system
    stream_state_manager_t *state = get_stream_state_by_name(stream_name);
    if (state) {
        log_info("Using new state management system to set streaming for stream '%s' to %s", 
                stream_name, enabled ? "enabled" : "disabled");
        
        int result = set_stream_feature(state, "streaming", enabled);
        
        // Update the old config for backward compatibility
        if (result == 0) {
            pthread_mutex_lock(&s->mutex);
            s->config.streaming_enabled = enabled;
            pthread_mutex_unlock(&s->mutex);
            
            // Update config
            config_t *global_config = get_streaming_config();
            if (global_config) {
                for (int i = 0; i < global_config->max_streams && i < MAX_STREAMS; i++) {
                    if (strcmp(global_config->streams[i].name, stream_name) == 0) {
                        global_config->streams[i].streaming_enabled = enabled;
                        break;
                    }
                }
            }
        }
        
        return result;
    }
    
    // Fall back to the old system if the state manager is not available
    log_warn("Falling back to legacy system to set streaming for stream '%s'", stream_name);
    
    pthread_mutex_lock(&s->mutex);
    s->config.streaming_enabled = enabled;
    pthread_mutex_unlock(&s->mutex);
    
    // Update config
    config_t *global_config = get_streaming_config();
    if (global_config) {
        for (int i = 0; i < global_config->max_streams && i < MAX_STREAMS; i++) {
            if (strcmp(global_config->streams[i].name, stream_name) == 0) {
                global_config->streams[i].streaming_enabled = enabled;
                break;
            }
        }
    }
    
    log_info("Set streaming enabled for stream '%s' to %s", stream_name, enabled ? "enabled" : "disabled");
    return 0;
}
