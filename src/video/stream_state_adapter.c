#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>

#include "video/stream_manager.h"
#include "video/stream_state.h"
#include "video/stream_state_adapter.h"
#include "core/logger.h"

// Mapping between old stream handles and new state managers
typedef struct {
    stream_handle_t handle;
    stream_state_manager_t *state;
} stream_handle_mapping_t;

#define MAX_HANDLE_MAPPINGS MAX_STREAMS
static stream_handle_mapping_t handle_mappings[MAX_HANDLE_MAPPINGS];
static pthread_mutex_t mappings_mutex = PTHREAD_MUTEX_INITIALIZER;
static bool adapter_initialized = false;

/**
 * Initialize the stream state adapter
 */
int init_stream_state_adapter(void) {
    if (adapter_initialized) {
        return 0;  // Already initialized
    }
    
    pthread_mutex_lock(&mappings_mutex);
    
    // Initialize mappings array
    memset(handle_mappings, 0, sizeof(handle_mappings));
    
    adapter_initialized = true;
    pthread_mutex_unlock(&mappings_mutex);
    
    log_info("Stream state adapter initialized");
    return 0;
}

/**
 * Shutdown the stream state adapter
 */
void shutdown_stream_state_adapter(void) {
    if (!adapter_initialized) {
        return;
    }
    
    pthread_mutex_lock(&mappings_mutex);
    
    // Clear mappings
    memset(handle_mappings, 0, sizeof(handle_mappings));
    
    adapter_initialized = false;
    pthread_mutex_unlock(&mappings_mutex);
    
    log_info("Stream state adapter shutdown");
}

/**
 * Add a mapping between a stream handle and a state manager
 */
static int add_handle_mapping(stream_handle_t handle, stream_state_manager_t *state) {
    if (!handle || !state || !adapter_initialized) {
        return -1;
    }
    
    pthread_mutex_lock(&mappings_mutex);
    
    // Find an empty slot
    int slot = -1;
    for (int i = 0; i < MAX_HANDLE_MAPPINGS; i++) {
        if (!handle_mappings[i].handle && !handle_mappings[i].state) {
            slot = i;
            break;
        }
    }
    
    if (slot == -1) {
        pthread_mutex_unlock(&mappings_mutex);
        log_error("No available slots for handle mapping");
        return -1;
    }
    
    // Add mapping
    handle_mappings[slot].handle = handle;
    handle_mappings[slot].state = state;
    
    pthread_mutex_unlock(&mappings_mutex);
    
    log_debug("Added handle mapping for stream '%s' in slot %d", state->name, slot);
    return 0;
}

/**
 * Remove a mapping by handle
 */
static int remove_handle_mapping_by_handle(stream_handle_t handle) {
    if (!handle || !adapter_initialized) {
        return -1;
    }
    
    pthread_mutex_lock(&mappings_mutex);
    
    // Find the mapping
    int slot = -1;
    for (int i = 0; i < MAX_HANDLE_MAPPINGS; i++) {
        if (handle_mappings[i].handle == handle) {
            slot = i;
            break;
        }
    }
    
    if (slot == -1) {
        pthread_mutex_unlock(&mappings_mutex);
        log_error("Handle mapping not found");
        return -1;
    }
    
    // Clear mapping
    handle_mappings[slot].handle = NULL;
    handle_mappings[slot].state = NULL;
    
    pthread_mutex_unlock(&mappings_mutex);
    
    log_debug("Removed handle mapping from slot %d", slot);
    return 0;
}

/**
 * Remove a mapping by state
 */
static int remove_handle_mapping_by_state(stream_state_manager_t *state) {
    if (!state || !adapter_initialized) {
        return -1;
    }
    
    pthread_mutex_lock(&mappings_mutex);
    
    // Find the mapping
    int slot = -1;
    for (int i = 0; i < MAX_HANDLE_MAPPINGS; i++) {
        if (handle_mappings[i].state == state) {
            slot = i;
            break;
        }
    }
    
    if (slot == -1) {
        pthread_mutex_unlock(&mappings_mutex);
        log_error("State mapping not found for stream '%s'", state->name);
        return -1;
    }
    
    // Clear mapping
    handle_mappings[slot].handle = NULL;
    handle_mappings[slot].state = NULL;
    
    pthread_mutex_unlock(&mappings_mutex);
    
    log_debug("Removed state mapping for stream '%s' from slot %d", state->name, slot);
    return 0;
}

/**
 * Convert stream_handle_t to stream_state_manager_t
 */
stream_state_manager_t *stream_handle_to_state(stream_handle_t handle) {
    if (!handle || !adapter_initialized) {
        return NULL;
    }
    
    pthread_mutex_lock(&mappings_mutex);
    
    // Find the mapping
    stream_state_manager_t *state = NULL;
    for (int i = 0; i < MAX_HANDLE_MAPPINGS; i++) {
        if (handle_mappings[i].handle == handle) {
            state = handle_mappings[i].state;
            break;
        }
    }
    
    pthread_mutex_unlock(&mappings_mutex);
    
    return state;
}

/**
 * Convert stream_state_manager_t to stream_handle_t
 */
stream_handle_t stream_state_to_handle(stream_state_manager_t *state) {
    if (!state || !adapter_initialized) {
        return NULL;
    }
    
    pthread_mutex_lock(&mappings_mutex);
    
    // Find the mapping
    stream_handle_t handle = NULL;
    for (int i = 0; i < MAX_HANDLE_MAPPINGS; i++) {
        if (handle_mappings[i].state == state) {
            handle = handle_mappings[i].handle;
            break;
        }
    }
    
    pthread_mutex_unlock(&mappings_mutex);
    
    return handle;
}

/**
 * Adapter function for add_stream
 */
stream_handle_t add_stream_adapter(const stream_config_t *config) {
    if (!config || !adapter_initialized) {
        return NULL;
    }
    
    // Create a new state manager
    stream_state_manager_t *state = create_stream_state(config);
    if (!state) {
        log_error("Failed to create stream state for '%s'", config->name);
        return NULL;
    }
    
    // Create a handle (just use the state pointer as the handle for simplicity)
    stream_handle_t handle = (stream_handle_t)state;
    
    // Add mapping
    if (add_handle_mapping(handle, state) != 0) {
        log_error("Failed to add handle mapping for stream '%s'", config->name);
        // Clean up
        remove_stream_state(state);
        return NULL;
    }
    
    log_info("Added stream '%s' through adapter", config->name);
    return handle;
}

/**
 * Adapter function for remove_stream
 */
int remove_stream_adapter(stream_handle_t handle) {
    if (!handle || !adapter_initialized) {
        return -1;
    }
    
    // Get the state manager
    stream_state_manager_t *state = stream_handle_to_state(handle);
    if (!state) {
        log_error("Failed to find state for handle in remove_stream_adapter");
        return -1;
    }
    
    // Save stream name for logging
    char stream_name[MAX_STREAM_NAME];
    strncpy(stream_name, state->name, MAX_STREAM_NAME - 1);
    stream_name[MAX_STREAM_NAME - 1] = '\0';
    
    // Remove mapping
    remove_handle_mapping_by_handle(handle);
    
    // Remove state
    int result = remove_stream_state(state);
    
    log_info("Removed stream '%s' through adapter", stream_name);
    return result;
}

/**
 * Adapter function for start_stream
 */
int start_stream_adapter(stream_handle_t handle) {
    if (!handle || !adapter_initialized) {
        return -1;
    }
    
    // Get the state manager
    stream_state_manager_t *state = stream_handle_to_state(handle);
    if (!state) {
        log_error("Failed to find state for handle in start_stream_adapter");
        return -1;
    }
    
    // Start the stream
    return start_stream_with_state(state);
}

/**
 * Adapter function for stop_stream
 */
int stop_stream_adapter(stream_handle_t handle) {
    if (!handle || !adapter_initialized) {
        return -1;
    }
    
    // Get the state manager
    stream_state_manager_t *state = stream_handle_to_state(handle);
    if (!state) {
        log_error("Failed to find state for handle in stop_stream_adapter");
        return -1;
    }
    
    // Stop the stream with wait_for_completion=true to ensure it fully stops
    return stop_stream_with_state(state, true);
}

/**
 * Adapter function for get_stream_status
 */
stream_status_t get_stream_status_adapter(stream_handle_t handle) {
    if (!handle || !adapter_initialized) {
        return STREAM_STATUS_UNKNOWN;
    }
    
    // Get the state manager
    stream_state_manager_t *state = stream_handle_to_state(handle);
    if (!state) {
        log_error("Failed to find state for handle in get_stream_status_adapter");
        return STREAM_STATUS_UNKNOWN;
    }
    
    // Convert state to status
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
            return STREAM_STATUS_STOPPING;
        case STREAM_STATE_ERROR:
            return STREAM_STATUS_ERROR;
        case STREAM_STATE_RECONNECTING:
            return STREAM_STATUS_RECONNECTING;
        default:
            return STREAM_STATUS_UNKNOWN;
    }
}

/**
 * Adapter function for get_stream_stats
 */
int get_stream_stats_adapter(stream_handle_t handle, stream_stats_t *stats) {
    if (!handle || !stats || !adapter_initialized) {
        return -1;
    }
    
    // Get the state manager
    stream_state_manager_t *state = stream_handle_to_state(handle);
    if (!state) {
        log_error("Failed to find state for handle in get_stream_stats_adapter");
        return -1;
    }
    
    // Get stats from state
    return get_stream_statistics(state, stats);
}

/**
 * Adapter function for get_stream_config
 */
int get_stream_config_adapter(stream_handle_t handle, stream_config_t *config) {
    if (!handle || !config || !adapter_initialized) {
        return -1;
    }
    
    // Get the state manager
    stream_state_manager_t *state = stream_handle_to_state(handle);
    if (!state) {
        log_error("Failed to find state for handle in get_stream_config_adapter");
        return -1;
    }
    
    // Get config from state
    pthread_mutex_lock(&state->mutex);
    memcpy(config, &state->config, sizeof(stream_config_t));
    pthread_mutex_unlock(&state->mutex);
    
    return 0;
}

/**
 * Adapter function for set_stream_priority
 */
int set_stream_priority_adapter(stream_handle_t handle, int priority) {
    if (!handle || !adapter_initialized) {
        return -1;
    }
    
    // Get the state manager
    stream_state_manager_t *state = stream_handle_to_state(handle);
    if (!state) {
        log_error("Failed to find state for handle in set_stream_priority_adapter");
        return -1;
    }
    
    // Update priority in config using the state config update function
    stream_config_t updated_config;
    pthread_mutex_lock(&state->mutex);
    memcpy(&updated_config, &state->config, sizeof(stream_config_t));
    pthread_mutex_unlock(&state->mutex);
    
    updated_config.priority = priority;
    update_stream_state_config(state, &updated_config);
    
    log_info("Set priority for stream '%s' to %d through adapter", state->name, priority);
    return 0;
}

/**
 * Adapter function for set_stream_recording
 */
int set_stream_recording_adapter(stream_handle_t handle, bool enable) {
    if (!handle || !adapter_initialized) {
        return -1;
    }
    
    // Get the state manager
    stream_state_manager_t *state = stream_handle_to_state(handle);
    if (!state) {
        log_error("Failed to find state for handle in set_stream_recording_adapter");
        return -1;
    }
    
    // Set recording feature
    return set_stream_feature(state, "recording", enable);
}

/**
 * Adapter function for set_stream_detection_recording
 */
int set_stream_detection_recording_adapter(stream_handle_t handle, bool enable, const char *model_path) {
    if (!handle || !adapter_initialized) {
        return -1;
    }
    
    // Get the state manager
    stream_state_manager_t *state = stream_handle_to_state(handle);
    if (!state) {
        log_error("Failed to find state for handle in set_stream_detection_recording_adapter");
        return -1;
    }
    
    // Update detection model path if provided
    if (model_path) {
        stream_config_t updated_config;
        pthread_mutex_lock(&state->mutex);
        memcpy(&updated_config, &state->config, sizeof(stream_config_t));
        pthread_mutex_unlock(&state->mutex);
        
        strncpy(updated_config.detection_model, model_path, MAX_PATH_LENGTH - 1);
        updated_config.detection_model[MAX_PATH_LENGTH - 1] = '\0';
        
        update_stream_state_config(state, &updated_config);
    }
    
    // Set detection feature
    return set_stream_feature(state, "detection", enable);
}

/**
 * Adapter function for set_stream_detection_params
 */
int set_stream_detection_params_adapter(stream_handle_t handle, int interval, float threshold, 
                                       int pre_buffer, int post_buffer) {
    if (!handle || !adapter_initialized) {
        return -1;
    }
    
    // Get the state manager
    stream_state_manager_t *state = stream_handle_to_state(handle);
    if (!state) {
        log_error("Failed to find state for handle in set_stream_detection_params_adapter");
        return -1;
    }
    
    // Update detection parameters using the state config update function
    stream_config_t updated_config;
    pthread_mutex_lock(&state->mutex);
    memcpy(&updated_config, &state->config, sizeof(stream_config_t));
    pthread_mutex_unlock(&state->mutex);
    
    if (interval > 0) {
        updated_config.detection_interval = interval;
    }
    
    if (threshold >= 0.0f && threshold <= 1.0f) {
        updated_config.detection_threshold = threshold;
    }
    
    if (pre_buffer >= 0) {
        updated_config.pre_detection_buffer = pre_buffer;
    }
    
    if (post_buffer >= 0) {
        updated_config.post_detection_buffer = post_buffer;
    }
    
    update_stream_state_config(state, &updated_config);
    
    log_info("Set detection parameters for stream '%s' through adapter", state->name);
    
    // If detection is enabled and stream is active, restart it to apply changes
    pthread_mutex_lock(&state->mutex);
    bool detection_enabled = state->features.detection_enabled;
    stream_state_t current_state = state->state;
    pthread_mutex_unlock(&state->mutex);
    
    if (detection_enabled && current_state == STREAM_STATE_ACTIVE) {
        log_info("Restarting stream '%s' to apply detection parameter changes", state->name);
        stop_stream_with_state(state, true); // Wait for completion
        start_stream_with_state(state);
    }
    
    return 0;
}

/**
 * Adapter function for set_stream_streaming_enabled
 */
int set_stream_streaming_enabled_adapter(stream_handle_t handle, bool enabled) {
    if (!handle || !adapter_initialized) {
        return -1;
    }
    
    // Get the state manager
    stream_state_manager_t *state = stream_handle_to_state(handle);
    if (!state) {
        log_error("Failed to find state for handle in set_stream_streaming_enabled_adapter");
        return -1;
    }
    
    // Set streaming feature
    return set_stream_feature(state, "streaming", enabled);
}

/**
 * Adapter function for get_stream_by_name
 */
stream_handle_t get_stream_by_name_adapter(const char *name) {
    if (!name || !adapter_initialized) {
        return NULL;
    }
    
    // Get the state manager by name
    stream_state_manager_t *state = get_stream_state_by_name(name);
    if (!state) {
        return NULL;
    }
    
    // Convert to handle
    return stream_state_to_handle(state);
}

/**
 * Adapter function for get_stream_by_index
 */
stream_handle_t get_stream_by_index_adapter(int index) {
    if (index < 0 || !adapter_initialized) {
        return NULL;
    }
    
    // Get the state manager by index
    stream_state_manager_t *state = get_stream_state_by_index(index);
    if (!state) {
        return NULL;
    }
    
    // Convert to handle
    return stream_state_to_handle(state);
}

/**
 * Adapter function for get_active_stream_count
 */
int get_active_stream_count_adapter(void) {
    if (!adapter_initialized) {
        return 0;
    }
    
    // Count active streams
    int count = 0;
    int total = get_stream_state_count();
    
    for (int i = 0; i < total; i++) {
        stream_state_manager_t *state = get_stream_state_by_index(i);
        if (state && get_stream_operational_state(state) == STREAM_STATE_ACTIVE) {
            count++;
        }
    }
    
    return count;
}

/**
 * Adapter function for get_total_stream_count
 */
int get_total_stream_count_adapter(void) {
    if (!adapter_initialized) {
        return 0;
    }
    
    return get_stream_state_count();
}
