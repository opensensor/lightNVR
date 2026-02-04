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
#include "database/db_streams.h"
#include "video/unified_detection_thread.h"
#ifdef USE_GO2RTC
#include "video/go2rtc/go2rtc_integration.h"
#endif

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
static bool initialized = false;

/**
 * Initialize stream manager
 */
int init_stream_manager(int max_streams) {
    if (initialized) {
        return 0;  // Already initialized
    }

    // Initialize streams array
    memset(streams, 0, sizeof(streams));
    for (int i = 0; i < MAX_STREAMS; i++) {
        pthread_mutex_init(&streams[i].mutex, NULL);
        streams[i].status = STREAM_STATUS_STOPPED;
        memset(&streams[i].stats, 0, sizeof(stream_stats_t));
        streams[i].recording_enabled = false;
        streams[i].detection_recording_enabled = false;
    }

    // Load stream configurations directly from database
    stream_config_t db_streams[MAX_STREAMS];
    int count = get_all_stream_configs(db_streams, MAX_STREAMS);

    if (count > 0) {
        for (int i = 0; i < count && i < MAX_STREAMS; i++) {
            if (db_streams[i].name[0] != '\0') {
                memcpy(&streams[i].config, &db_streams[i], sizeof(stream_config_t));
                streams[i].recording_enabled = db_streams[i].record;
                streams[i].detection_recording_enabled = db_streams[i].detection_based_recording;
            }
        }
    }

    initialized = true;

    // Create stream state managers for all existing streams and register with go2rtc
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (streams[i].config.name[0] != '\0') {
            stream_state_manager_t *state = get_stream_state_by_name(streams[i].config.name);
            if (!state) {
                state = create_stream_state(&streams[i].config);
                if (!state) {
                    log_warn("Failed to create stream state for '%s' during initialization", streams[i].config.name);
                } else {
                    log_info("Created stream state for '%s' during initialization", streams[i].config.name);
                }
            }

            // Register existing streams with go2rtc using the centralized function
            #ifdef USE_GO2RTC
            go2rtc_integration_register_stream(streams[i].config.name);
            #endif
        }
    }

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

    // Stop all streams
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (streams[i].config.name[0] != '\0' && streams[i].status == STREAM_STATUS_RUNNING) {
            char stream_name[MAX_STREAM_NAME];
            strncpy(stream_name, streams[i].config.name, MAX_STREAM_NAME - 1);
            stream_name[MAX_STREAM_NAME - 1] = '\0';

            bool streaming_enabled = streams[i].config.streaming_enabled;
            bool recording_enabled = streams[i].config.record;

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

            streams[i].status = STREAM_STATUS_STOPPED;
        }
    }

    initialized = false;

    log_info("Stream manager shutdown");
}

/**
 * Get stream by name
 */
stream_handle_t get_stream_by_name(const char *name) {
    if (!name || !initialized) {
        return NULL;
    }

    for (int i = 0; i < MAX_STREAMS; i++) {
        if (streams[i].config.name[0] != '\0' && strcmp(streams[i].config.name, name) == 0) {
            return (stream_handle_t)&streams[i];
        }
    }

    // If stream not found in memory, check if it exists in the database
    stream_config_t db_config;
    if (get_stream_config_by_name(name, &db_config) == 0) {
        // Found in database, add to memory
        for (int i = 0; i < MAX_STREAMS; i++) {
            if (streams[i].config.name[0] == '\0') {
                // Found empty slot
                memcpy(&streams[i].config, &db_config, sizeof(stream_config_t));
                streams[i].status = STREAM_STATUS_STOPPED;
                streams[i].recording_enabled = db_config.record;
                streams[i].detection_recording_enabled = db_config.detection_based_recording;

                return (stream_handle_t)&streams[i];
            }
        }
        // No empty slots available
        log_error("No available slots for stream from database: %s", name);
    }

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

    // Get the configuration directly from the database
    if (get_stream_config_by_name(s->config.name, config) == 0) {
        // Update the in-memory configuration to stay in sync
        pthread_mutex_lock(&s->mutex);
        memcpy(&s->config, config, sizeof(stream_config_t));
        pthread_mutex_unlock(&s->mutex);
        return 0;
    }

    // Return error if database query fails
    log_error("Failed to get stream configuration from database for stream %s", s->config.name);
    return -1;
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

    // Check if detection was previously enabled and is now being disabled
    bool was_enabled = s->config.detection_based_recording;
    bool now_disabled = was_enabled && !enabled;
    // Check if detection was previously disabled and is now being enabled
    bool now_enabled = !was_enabled && enabled;

    // Get stream name for potential thread stopping/starting
    char stream_name[MAX_STREAM_NAME];
    strncpy(stream_name, s->config.name, MAX_STREAM_NAME - 1);
    stream_name[MAX_STREAM_NAME - 1] = '\0';

    // Update configuration
    s->config.detection_based_recording = enabled;

    if (model_path) {
        strncpy(s->config.detection_model, model_path, MAX_PATH_LENGTH - 1);
        s->config.detection_model[MAX_PATH_LENGTH - 1] = '\0';
    }

    // Get a copy of the config for database update
    stream_config_t config_copy;
    memcpy(&config_copy, &s->config, sizeof(stream_config_t));

    pthread_mutex_unlock(&s->mutex);

    // Update the database directly
    if (update_stream_config(config_copy.name, &config_copy) != 0) {
        log_error("Failed to update stream configuration in database for stream %s", config_copy.name);
        return -1;
    }

    // Also update the stream state manager if it exists
    stream_state_manager_t *state = get_stream_state_by_name(config_copy.name);
    if (state) {
        update_stream_state_config(state, &config_copy);
        log_info("Updated stream state configuration for stream %s", config_copy.name);
    }

    // If detection was enabled and is now being disabled, stop the detection thread
    if (now_disabled) {
        log_info("Detection disabled for stream %s, stopping unified detection thread", stream_name);

        // Stop the unified detection thread
        if (stop_unified_detection_thread(stream_name) != 0) {
            log_warn("Failed to stop unified detection thread for stream %s", stream_name);
        } else {
            log_info("Successfully stopped unified detection thread for stream %s", stream_name);
        }
    }
    // If detection was disabled and is now being enabled, start the detection thread
    else if (now_enabled && config_copy.detection_model[0] != '\0') {
        log_info("Detection enabled for stream %s, starting unified detection thread with model %s",
                stream_name, config_copy.detection_model);

        // Start unified detection thread
        if (start_unified_detection_thread(stream_name,
                                          config_copy.detection_model,
                                          config_copy.detection_threshold,
                                          config_copy.pre_detection_buffer,
                                          config_copy.post_detection_buffer) != 0) {
            log_warn("Failed to start unified detection thread for stream %s", stream_name);
        } else {
            log_info("Successfully started unified detection thread for stream %s", stream_name);
        }
    }

    log_info("Set detection recording for stream %s: enabled=%s, model=%s",
             config_copy.name, enabled ? "true" : "false", model_path ? model_path : "none");

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

    // Get a copy of the config for database update
    stream_config_t config_copy;
    memcpy(&config_copy, &s->config, sizeof(stream_config_t));

    pthread_mutex_unlock(&s->mutex);

    // Update the database directly
    if (update_stream_config(config_copy.name, &config_copy) != 0) {
        log_error("Failed to update stream configuration in database for stream %s", config_copy.name);
        return -1;
    }

    // Also update the stream state manager if it exists
    stream_state_manager_t *state = get_stream_state_by_name(config_copy.name);
    if (state) {
        update_stream_state_config(state, &config_copy);
        log_info("Updated stream state configuration for stream %s", config_copy.name);
    }

    log_info("Set detection parameters for stream %s", config_copy.name);

    return 0;
}

/**
 * Add a new stream
 */
stream_handle_t add_stream(const stream_config_t *config) {
    if (!config || !initialized) {
        return NULL;
    }

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
        return NULL;
    }

    // Check if stream with same name already exists
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (i != slot && streams[i].config.name[0] != '\0' &&
            strcmp(streams[i].config.name, config->name) == 0) {
            log_error("Stream with name '%s' already exists", config->name);
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

    // Register stream with go2rtc using the centralized function
    #ifdef USE_GO2RTC
    go2rtc_integration_register_stream(config->name);
    #endif

    log_info("Added stream '%s' in slot %d", config->name, slot);

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
        return -1;
    }

    // Save stream name for logging
    char stream_name[MAX_STREAM_NAME];
    strncpy(stream_name, s->config.name, MAX_STREAM_NAME - 1);
    stream_name[MAX_STREAM_NAME - 1] = '\0';

    // Unregister stream from go2rtc using the centralized function
    #ifdef USE_GO2RTC
    go2rtc_integration_unregister_stream(stream_name);
    #endif

    // Delete the stream from the database
    if (delete_stream_config(stream_name) != 0) {
        log_error("Failed to delete stream configuration from database for stream %s", stream_name);
        // Continue anyway to clean up the local state
    }

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

    log_info("Removed stream '%s' from slot %d", stream_name, slot);

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


    if (streams[index].config.name[0] == '\0') {
        return NULL;
    }

    return (stream_handle_t)&streams[index];
}

/**
 * Get number of active streams
 */
int get_active_stream_count(void) {
    if (!initialized) {
        return 0;
    }

    int count = 0;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (streams[i].config.name[0] != '\0' &&
            (streams[i].status == STREAM_STATUS_RUNNING ||
             streams[i].status == STREAM_STATUS_RECONNECTING ||
             streams[i].status == STREAM_STATUS_STARTING)) {
            count++;
        }
    }

    return count;
}

/**
 * Get total number of streams
 */
int get_total_stream_count(void) {
    if (!initialized) {
        return 0;
    }

    int count = 0;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (streams[i].config.name[0] != '\0') {
            count++;
        }
    }

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

    // Get a copy of the config for database update
    stream_config_t config_copy;
    memcpy(&config_copy, &s->config, sizeof(stream_config_t));

    pthread_mutex_unlock(&s->mutex);

    // Update the database directly
    if (update_stream_config(config_copy.name, &config_copy) != 0) {
        log_error("Failed to update stream configuration in database for stream %s", config_copy.name);
        return -1;
    }

    // Also update the stream state manager if it exists
    stream_state_manager_t *state = get_stream_state_by_name(config_copy.name);
    if (state) {
        update_stream_state_config(state, &config_copy);
        log_info("Updated stream state configuration for stream %s", config_copy.name);
    }

    log_info("Set priority for stream '%s' to %d", config_copy.name, priority);
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

        // Update the local config
        if (result == 0) {
            pthread_mutex_lock(&s->mutex);
            s->config.record = enable;
            s->recording_enabled = enable;

            // Get a copy of the config for database update
            stream_config_t config_copy;
            memcpy(&config_copy, &s->config, sizeof(stream_config_t));

            pthread_mutex_unlock(&s->mutex);

            // Update the database directly
            if (update_stream_config(config_copy.name, &config_copy) != 0) {
                log_error("Failed to update stream configuration in database for stream %s", config_copy.name);
                // Continue anyway, the feature was set successfully
            }
        }

        return result;
    }

    // Fall back to the old system if the state manager is not available
    log_warn("Falling back to legacy system to set recording for stream '%s'", stream_name);

    pthread_mutex_lock(&s->mutex);
    s->config.record = enable;
    s->recording_enabled = enable;

    // Get a copy of the config for database update
    stream_config_t config_copy;
    memcpy(&config_copy, &s->config, sizeof(stream_config_t));

    pthread_mutex_unlock(&s->mutex);

    // Update the database directly
    if (update_stream_config(config_copy.name, &config_copy) != 0) {
        log_error("Failed to update stream configuration in database for stream %s", config_copy.name);
        return -1;
    }

    // Also update the stream state manager if it exists
    state = get_stream_state_by_name(config_copy.name);
    if (state) {
        update_stream_state_config(state, &config_copy);
        log_info("Updated stream state configuration for stream %s", config_copy.name);
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

        // Update the local config
        if (result == 0) {
            pthread_mutex_lock(&s->mutex);
            s->config.streaming_enabled = enabled;

            // Get a copy of the config for database update
            stream_config_t config_copy;
            memcpy(&config_copy, &s->config, sizeof(stream_config_t));

            pthread_mutex_unlock(&s->mutex);

            // Update the database directly
            if (update_stream_config(config_copy.name, &config_copy) != 0) {
                log_error("Failed to update stream configuration in database for stream %s", config_copy.name);
                // Continue anyway, the feature was set successfully
            }
        }

        return result;
    }

    // Fall back to the old system if the state manager is not available
    log_warn("Falling back to legacy system to set streaming for stream '%s'", stream_name);

    pthread_mutex_lock(&s->mutex);
    s->config.streaming_enabled = enabled;

    // Get a copy of the config for database update
    stream_config_t config_copy;
    memcpy(&config_copy, &s->config, sizeof(stream_config_t));

    pthread_mutex_unlock(&s->mutex);

    // Update the database directly
    if (update_stream_config(config_copy.name, &config_copy) != 0) {
        log_error("Failed to update stream configuration in database for stream %s", config_copy.name);
        return -1;
    }

    // Also update the stream state manager if it exists
    state = get_stream_state_by_name(config_copy.name);
    if (state) {
        update_stream_state_config(state, &config_copy);
        log_info("Updated stream state configuration for stream %s", config_copy.name);
    }

    log_info("Set streaming enabled for stream '%s' to %s", stream_name, enabled ? "enabled" : "disabled");
    return 0;
}

/**
 * Set ONVIF flag for a stream
 */
int set_stream_onvif_flag(stream_handle_t handle, bool is_onvif) {
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

    log_info("Setting ONVIF flag for stream '%s' to %s", stream_name, is_onvif ? "true" : "false");

    pthread_mutex_lock(&s->mutex);

    // Check if the flag is already set to the desired value
    if (s->config.is_onvif == is_onvif) {
        pthread_mutex_unlock(&s->mutex);
        log_info("ONVIF flag for stream '%s' is already set to %s",
                stream_name, is_onvif ? "true" : "false");
        return 0;
    }

    // Update the flag
    s->config.is_onvif = is_onvif;

    // Get a copy of the config for database update
    stream_config_t config_copy;
    memcpy(&config_copy, &s->config, sizeof(stream_config_t));

    pthread_mutex_unlock(&s->mutex);

    // Update the database directly
    if (update_stream_config(config_copy.name, &config_copy) != 0) {
        log_error("Failed to update stream configuration in database for stream %s", config_copy.name);
        return -1;
    }

    // Also update the stream state manager if it exists
    stream_state_manager_t *state = get_stream_state_by_name(config_copy.name);
    if (state) {
        update_stream_state_config(state, &config_copy);
        log_info("Updated stream state configuration for stream %s", config_copy.name);
    }

    log_info("Set ONVIF flag for stream '%s' to %s", stream_name, is_onvif ? "true" : "false");
    return 0;
}
