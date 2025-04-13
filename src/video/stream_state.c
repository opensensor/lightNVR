#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>
#include <unistd.h>
#include <time.h>
#include <sys/time.h>

#include "video/stream_state.h"
#include "core/logger.h"
#include "core/config.h"
#include "video/stream_reader.h"
#include "video/hls_streaming.h"
#include "video/mp4_recording.h"
#include "video/detection.h"
#include "video/stream_transcoding.h"

/**
 * BUGFIX: Modified stop_stream_with_state to always stop HLS streaming and MP4 recording
 * regardless of the current streaming_enabled and recording_enabled flags.
 * This ensures that any running streams or recordings are properly cleaned up when
 * the stream is stopped, even if the flags were changed before stopping.
 */

// Global array of stream state managers
static stream_state_manager_t *stream_states[MAX_STREAMS];
static pthread_mutex_t states_mutex = PTHREAD_MUTEX_INITIALIZER;
static bool initialized = false;

/**
 * Initialize the stream state management system
 */
int init_stream_state_manager(int max_streams) {
    if (initialized) {
        return 0;  // Already initialized
    }

    pthread_mutex_lock(&states_mutex);

    // Initialize stream states array
    memset(stream_states, 0, sizeof(stream_states));

    initialized = true;
    pthread_mutex_unlock(&states_mutex);

    log_info("Stream state manager initialized");
    return 0;
}

/**
 * Shutdown the stream state management system
 */
void shutdown_stream_state_manager(void) {
    if (!initialized) {
        return;
    }

    pthread_mutex_lock(&states_mutex);

    // Stop and clean up all streams
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (stream_states[i]) {
            // Make a local copy of the stream name for logging
            char stream_name[MAX_STREAM_NAME];
            strncpy(stream_name, stream_states[i]->name, MAX_STREAM_NAME - 1);
            stream_name[MAX_STREAM_NAME - 1] = '\0';

            // Stop the stream if it's active
            if (stream_states[i]->state == STREAM_STATE_ACTIVE ||
                stream_states[i]->state == STREAM_STATE_STARTING) {
                stop_stream_with_state(stream_states[i], false);
            }

            // Destroy mutex
            pthread_mutex_destroy(&stream_states[i]->mutex);

            // Free the state manager
            free(stream_states[i]);
            stream_states[i] = NULL;

            log_info("Cleaned up stream state for '%s' during shutdown", stream_name);
        }
    }

    initialized = false;
    pthread_mutex_unlock(&states_mutex);

    log_info("Stream state manager shutdown");
}

/**
 * Create a new stream state manager
 */
stream_state_manager_t *create_stream_state(const stream_config_t *config) {
    if (!config || !initialized) {
        log_error("Invalid parameters or state manager not initialized");
        return NULL;
    }

    pthread_mutex_lock(&states_mutex);

    // Find an empty slot
    int slot = -1;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (!stream_states[i]) {
            slot = i;
            break;
        }
    }

    if (slot == -1) {
        log_error("No available slots for new stream state");
        pthread_mutex_unlock(&states_mutex);
        return NULL;
    }

    // Check if stream with same name already exists
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (i != slot && stream_states[i] &&
            strcmp(stream_states[i]->name, config->name) == 0) {
            log_error("Stream with name '%s' already exists", config->name);
            pthread_mutex_unlock(&states_mutex);
            return NULL;
        }
    }

    // Allocate and initialize the state manager
    stream_state_manager_t *state = calloc(1, sizeof(stream_state_manager_t));
    if (!state) {
        log_error("Failed to allocate memory for stream state");
        pthread_mutex_unlock(&states_mutex);
        return NULL;
    }

    // Initialize mutex
    if (pthread_mutex_init(&state->mutex, NULL) != 0) {
        log_error("Failed to initialize stream state mutex");
        free(state);
        pthread_mutex_unlock(&states_mutex);
        return NULL;
    }

    // Initialize state mutex
    if (pthread_mutex_init(&state->state_mutex, NULL) != 0) {
        log_error("Failed to initialize stream state transition mutex");
        pthread_mutex_destroy(&state->mutex);
        free(state);
        pthread_mutex_unlock(&states_mutex);
        return NULL;
    }

    // Copy configuration
    memcpy(&state->config, config, sizeof(stream_config_t));

    // Initialize name
    strncpy(state->name, config->name, MAX_STREAM_NAME - 1);
    state->name[MAX_STREAM_NAME - 1] = '\0';

    // Initialize state
    state->state = STREAM_STATE_INACTIVE;

    // Initialize features
    state->features.streaming_enabled = config->streaming_enabled;
    state->features.recording_enabled = config->record;
    state->features.detection_enabled = config->detection_based_recording;
    state->features.motion_detection_enabled = false; // Not in config yet

    // Initialize protocol state
    state->protocol_state.protocol = config->protocol;
    state->protocol_state.is_multicast = false; // Will be determined when connecting
    state->protocol_state.reconnect_attempts = 0;
    state->protocol_state.last_reconnect_time = 0;
    state->protocol_state.buffer_size = 0; // Will be set based on protocol
    state->protocol_state.timeout_ms = 0; // Will be set based on protocol

    // Initialize timestamp state
    state->timestamp_state.last_pts = AV_NOPTS_VALUE;
    state->timestamp_state.last_dts = AV_NOPTS_VALUE;
    state->timestamp_state.expected_next_pts = AV_NOPTS_VALUE;
    state->timestamp_state.pts_discontinuity_count = 0;
    state->timestamp_state.timestamps_initialized = false;

    // Initialize stats
    memset(&state->stats, 0, sizeof(stream_stats_t));

    // Initialize reference counting
    state->ref_count = 1; // Initial reference for the creator
    memset(state->component_refs, 0, sizeof(state->component_refs));
    state->component_refs[STREAM_COMPONENT_API] = 1; // Initial reference is from API

    // Initialize callback management
    state->callbacks_enabled = true;

    // Initialize context pointers
    state->reader_ctx = NULL;
    state->hls_ctx = NULL;
    state->mp4_ctx = NULL;
    state->detection_ctx = NULL;

    // Store in global array
    stream_states[slot] = state;

    log_info("Created stream state for '%s' in slot %d with initial reference count 1",
             config->name, slot);
    pthread_mutex_unlock(&states_mutex);

    return state;
}

/**
 * Get stream state manager by name
 */
stream_state_manager_t *get_stream_state_by_name(const char *name) {
    if (!name || !initialized) {
        return NULL;
    }

    pthread_mutex_lock(&states_mutex);

    for (int i = 0; i < MAX_STREAMS; i++) {
        if (stream_states[i] && strcmp(stream_states[i]->name, name) == 0) {
            pthread_mutex_unlock(&states_mutex);
            return stream_states[i];
        }
    }

    pthread_mutex_unlock(&states_mutex);
    return NULL;
}

/**
 * Update stream state configuration
 */
int update_stream_state_config(stream_state_manager_t *state, const stream_config_t *config) {
    if (!state || !config) {
        log_error("Invalid parameters for update_stream_config");
        return -1;
    }

    pthread_mutex_lock(&state->mutex);

    // Check if protocol is changing
    bool protocol_changing = (state->config.protocol != config->protocol);

    // Check if features are changing
    bool streaming_changing = (state->config.streaming_enabled != config->streaming_enabled);
    bool recording_changing = (state->config.record != config->record);
    bool detection_changing = (state->config.detection_based_recording != config->detection_based_recording);
    bool onvif_changing = (state->config.is_onvif != config->is_onvif);

    // Log ONVIF flag change
    if (onvif_changing) {
        log_info("ONVIF flag changed for stream '%s': %s -> %s",
                state->name,
                state->config.is_onvif ? "true" : "false",
                config->is_onvif ? "true" : "false");
    }

    // Save old state for comparison
    stream_state_t old_state = state->state;
    stream_protocol_t old_protocol = state->config.protocol;

    // Update configuration
    memcpy(&state->config, config, sizeof(stream_config_t));

    // Update features
    state->features.streaming_enabled = config->streaming_enabled;
    state->features.recording_enabled = config->record;
    state->features.detection_enabled = config->detection_based_recording;

    // If protocol is changing, update protocol state
    if (protocol_changing) {
        state->protocol_state.protocol = config->protocol;
        state->protocol_state.reconnect_attempts = 0;
        state->protocol_state.last_reconnect_time = 0;

        // Set protocol-specific settings
        if (config->protocol == STREAM_PROTOCOL_UDP) {
            state->protocol_state.buffer_size = 16777216; // 16MB for UDP
            state->protocol_state.timeout_ms = 10000; // 10 seconds for UDP
        } else {
            state->protocol_state.buffer_size = 8388608; // 8MB for TCP
            state->protocol_state.timeout_ms = 5000; // 5 seconds for TCP
        }

        log_info("Protocol changed for stream '%s': %s -> %s",
                state->name,
                old_protocol == STREAM_PROTOCOL_UDP ? "UDP" : "TCP",
                config->protocol == STREAM_PROTOCOL_UDP ? "UDP" : "TCP");
    }

    pthread_mutex_unlock(&state->mutex);

    // If stream is active and protocol or features changed, restart it
    if (old_state == STREAM_STATE_ACTIVE &&
        (protocol_changing || streaming_changing || recording_changing || detection_changing || onvif_changing)) {
        log_info("Restarting stream '%s' due to configuration changes", state->name);

        // Stop and restart the stream
        stop_stream_with_state(state, false);
        start_stream_with_state(state);
    }

    return 0;
}

/**
 * Update stream protocol
 */
int update_stream_protocol(stream_state_manager_t *state, stream_protocol_t protocol) {
    if (!state) {
        log_error("Invalid parameters for update_stream_protocol");
        return -1;
    }

    pthread_mutex_lock(&state->mutex);

    // Check if protocol is changing
    if (state->config.protocol == protocol) {
        pthread_mutex_unlock(&state->mutex);
        log_info("Protocol for stream '%s' already set to %s",
                state->name, protocol == STREAM_PROTOCOL_UDP ? "UDP" : "TCP");
        return 0;
    }

    // Update protocol in config
    state->config.protocol = protocol;

    // Update protocol state
    state->protocol_state.protocol = protocol;
    state->protocol_state.reconnect_attempts = 0;
    state->protocol_state.last_reconnect_time = 0;

    // Set protocol-specific settings
    if (protocol == STREAM_PROTOCOL_UDP) {
        state->protocol_state.buffer_size = 16777216; // 16MB for UDP
        state->protocol_state.timeout_ms = 10000; // 10 seconds for UDP
    } else {
        state->protocol_state.buffer_size = 8388608; // 8MB for TCP
        state->protocol_state.timeout_ms = 5000; // 5 seconds for TCP
    }

    // Save current state
    stream_state_t current_state = state->state;

    pthread_mutex_unlock(&state->mutex);

    log_info("Protocol updated for stream '%s' to %s",
            state->name, protocol == STREAM_PROTOCOL_UDP ? "UDP" : "TCP");

    // If stream is active, restart it
    if (current_state == STREAM_STATE_ACTIVE) {
        log_info("Restarting stream '%s' due to protocol change", state->name);

        // Stop and restart the stream
        stop_stream_with_state(state, false);
        start_stream_with_state(state);
    }

    return 0;
}

/**
 * Set stream feature state
 */
int set_stream_feature(stream_state_manager_t *state, const char *feature, bool enabled) {
    if (!state || !feature) {
        log_error("Invalid parameters for set_stream_feature");
        return -1;
    }

    pthread_mutex_lock(&state->mutex);

    bool changed = false;

    // Update the appropriate feature
    if (strcmp(feature, "streaming") == 0) {
        if (state->features.streaming_enabled != enabled) {
            state->features.streaming_enabled = enabled;
            state->config.streaming_enabled = enabled;
            changed = true;
        }
    } else if (strcmp(feature, "recording") == 0) {
        if (state->features.recording_enabled != enabled) {
            state->features.recording_enabled = enabled;
            state->config.record = enabled;
            changed = true;
        }
    } else if (strcmp(feature, "detection") == 0) {
        if (state->features.detection_enabled != enabled) {
            state->features.detection_enabled = enabled;
            state->config.detection_based_recording = enabled;
            changed = true;
        }
    } else if (strcmp(feature, "motion_detection") == 0) {
        if (state->features.motion_detection_enabled != enabled) {
            state->features.motion_detection_enabled = enabled;
            changed = true;
        }
    } else {
        pthread_mutex_unlock(&state->mutex);
        log_error("Unknown feature '%s' for stream '%s'", feature, state->name);
        return -1;
    }

    // Save current state
    stream_state_t current_state = state->state;

    pthread_mutex_unlock(&state->mutex);

    if (changed) {
        log_info("Feature '%s' %s for stream '%s'",
                feature, enabled ? "enabled" : "disabled", state->name);

        // If stream is active, apply the change
        if (current_state == STREAM_STATE_ACTIVE) {
            if (strcmp(feature, "streaming") == 0) {
                if (enabled) {
                    // Start HLS streaming
                    start_hls_stream(state->name);
                } else {
                    // Stop HLS streaming
                    stop_hls_stream(state->name);
                }
            } else if (strcmp(feature, "recording") == 0) {
                if (enabled) {
                    // Start recording
                    start_mp4_recording(state->name);
                } else {
                    // Stop recording
                    stop_mp4_recording(state->name);
                }
            } else if (strcmp(feature, "detection") == 0 || strcmp(feature, "motion_detection") == 0) {
                // For detection features, we need to restart the stream
                log_info("Restarting stream '%s' to apply detection settings", state->name);
                stop_stream_with_state(state, false);
                start_stream_with_state(state);
            }
        }
    }

    return 0;
}

/**
 * Start stream with state
 */
int start_stream_with_state(stream_state_manager_t *state) {
    if (!state) {
        log_error("Invalid parameters for start_stream_with_state");
        return -1;
    }

    pthread_mutex_lock(&state->mutex);

    // Check if already running
    if (state->state == STREAM_STATE_ACTIVE) {
        pthread_mutex_unlock(&state->mutex);
        log_info("Stream '%s' is already running", state->name);
        return 0;
    }

    // Update state to starting
    state->state = STREAM_STATE_STARTING;

    // Get feature flags
    bool streaming_enabled = state->features.streaming_enabled;
    bool recording_enabled = state->features.recording_enabled;
    bool detection_enabled = state->features.detection_enabled;

    // Get protocol
    stream_protocol_t protocol = state->protocol_state.protocol;

    pthread_mutex_unlock(&state->mutex);

    log_info("Starting stream '%s' (protocol: %s, streaming: %s, recording: %s, detection: %s)",
            state->name,
            protocol == STREAM_PROTOCOL_UDP ? "UDP" : "TCP",
            streaming_enabled ? "enabled" : "disabled",
            recording_enabled ? "enabled" : "disabled",
            detection_enabled ? "enabled" : "disabled");

    // Track if any component started successfully
    bool any_component_started = false;

    // Start stream reader
    stream_reader_ctx_t *reader_ctx = NULL;

    // Start HLS streaming if enabled
    if (streaming_enabled) {
        int hls_result = start_hls_stream(state->name);
        if (hls_result != 0) {
            log_error("Failed to start HLS stream '%s'", state->name);
        } else {
            log_info("Started HLS streaming for '%s'", state->name);
            any_component_started = true;
        }
    }

    // Start recording if enabled
    if (recording_enabled) {
        int mp4_result = start_mp4_recording(state->name);
        if (mp4_result != 0) {
            log_error("Failed to start MP4 recording for '%s'", state->name);
        } else {
            log_info("Started recording for '%s'", state->name);
            any_component_started = true;
        }
    }

    // Update state based on results
    pthread_mutex_lock(&state->mutex);
    if (any_component_started) {
        state->state = STREAM_STATE_ACTIVE;
        log_info("Stream '%s' is now running", state->name);
    } else {
        state->state = STREAM_STATE_ERROR;
        log_error("Failed to start any components for stream '%s'", state->name);
        pthread_mutex_unlock(&state->mutex);
        return -1;
    }
    pthread_mutex_unlock(&state->mutex);

    return 0;
}

/**
 * Stop stream with state
 */
int stop_stream_with_state(stream_state_manager_t *state, bool wait_for_completion) {
    if (!state) {
        log_error("Invalid parameters for stop_stream_with_state");
        return -1;
    }

    // Use the state mutex for state transitions
    pthread_mutex_lock(&state->state_mutex);

    // Check if already stopped or stopping
    if (state->state == STREAM_STATE_INACTIVE) {
        pthread_mutex_unlock(&state->state_mutex);
        log_info("Stream '%s' is already stopped", state->name);
        return 0;
    }

    if (state->state == STREAM_STATE_STOPPING) {
        pthread_mutex_unlock(&state->state_mutex);
        log_info("Stream '%s' is already in the process of stopping", state->name);

        // If we need to wait for completion, wait for it to become inactive
        if (wait_for_completion) {
            return wait_for_stream_stop(state, 10000); // 10 second timeout
        }
        return 0;
    }

    // Update state to stopping
    stream_state_t old_state = state->state;
    state->state = STREAM_STATE_STOPPING;

    // Get feature flags and stream name while holding the mutex
    bool streaming_enabled = state->features.streaming_enabled;
    bool recording_enabled = state->features.recording_enabled;

    // Get stream name for logging
    char stream_name[MAX_STREAM_NAME];
    strncpy(stream_name, state->name, MAX_STREAM_NAME - 1);
    stream_name[MAX_STREAM_NAME - 1] = '\0';

    // Only disable callbacks if we're stopping the entire stream
    // If we're just disabling recording, we still want to keep callbacks enabled for HLS streaming
    if (!streaming_enabled) {
        // Disable callbacks to prevent further processing
        state->callbacks_enabled = false;
    }

    pthread_mutex_unlock(&state->state_mutex);

    log_info("Stream '%s' transitioning from %d to STOPPING state",
             stream_name, old_state);

    // Always attempt to stop HLS stream regardless of streaming_enabled flag
    // This ensures we clean up any HLS streams that might be running
    int result = stop_hls_stream(stream_name);
    if (result != 0) {
        log_warn("Failed to stop HLS stream '%s'", stream_name);
        // Continue anyway
    } else {
        log_info("Stopped HLS streaming for '%s'", stream_name);
    }

    // Always attempt to stop MP4 recording regardless of recording_enabled flag
    // This ensures we clean up any recordings that might be running
    int mp4_result = stop_mp4_recording(stream_name);
    if (mp4_result != 0) {
        log_warn("Failed to stop MP4 recording for '%s'", stream_name);
        // Continue anyway
    } else {
        log_info("Stopped MP4 recording for '%s'", stream_name);
    }

    // Reset timestamp tracker
    reset_timestamp_tracker(stream_name);

    // Update state to inactive
    pthread_mutex_lock(&state->state_mutex);
    state->state = STREAM_STATE_INACTIVE;

    // Re-enable callbacks for future use
    state->callbacks_enabled = true;

    pthread_mutex_unlock(&state->state_mutex);

    log_info("Stopped stream '%s'", stream_name);
    return 0;
}

/**
 * Get stream operational state
 */
stream_state_t get_stream_operational_state(stream_state_manager_t *state) {
    if (!state) {
        return STREAM_STATE_INACTIVE;
    }

    pthread_mutex_lock(&state->mutex);
    stream_state_t current_state = state->state;
    pthread_mutex_unlock(&state->mutex);

    return current_state;
}

/**
 * Get stream statistics
 */
int get_stream_statistics(stream_state_manager_t *state, stream_stats_t *stats) {
    if (!state || !stats) {
        return -1;
    }

    pthread_mutex_lock(&state->mutex);
    memcpy(stats, &state->stats, sizeof(stream_stats_t));
    pthread_mutex_unlock(&state->mutex);

    return 0;
}

/**
 * Handle stream error
 */
int handle_stream_error(stream_state_manager_t *state, int error_code, const char *error_message) {
    if (!state) {
        return -1;
    }

    pthread_mutex_lock(&state->mutex);

    // Update error statistics
    state->stats.errors++;

    // Log the error
    log_error("Stream '%s' error: %s (code: %d)", state->name, error_message, error_code);

    // Check if we should attempt reconnection
    bool should_reconnect = false;

    // Only attempt reconnection if the stream was active
    if (state->state == STREAM_STATE_ACTIVE) {
        should_reconnect = true;

        // Update state to reconnecting
        state->state = STREAM_STATE_RECONNECTING;

        // Update reconnection statistics
        state->protocol_state.reconnect_attempts++;
        state->protocol_state.last_reconnect_time = time(NULL);
        state->stats.reconnects++;
    } else {
        // If not active, just set to error state
        state->state = STREAM_STATE_ERROR;
    }

    pthread_mutex_unlock(&state->mutex);

    // If we should reconnect, stop and restart the stream
    if (should_reconnect) {
        log_info("Attempting to reconnect stream '%s'", state->name);

        // Stop the stream
        stop_stream_with_state(state, false);

        // Wait a bit before reconnecting
        sleep(1);

        // Restart the stream
        start_stream_with_state(state);
    }

    return 0;
}

/**
 * Update stream timestamps
 */
int update_stream_timestamps(stream_state_manager_t *state, int64_t pts, int64_t dts) {
    if (!state) {
        return -1;
    }

    pthread_mutex_lock(&state->mutex);

    // Update timestamp state
    state->timestamp_state.last_pts = pts;
    state->timestamp_state.last_dts = dts;

    // If timestamps weren't initialized, initialize them now
    if (!state->timestamp_state.timestamps_initialized) {
        state->timestamp_state.timestamps_initialized = true;
        state->timestamp_state.expected_next_pts = pts;
    } else {
        // Calculate expected next PTS based on framerate
        int64_t frame_duration = 0;

        // Try to get framerate from config
        if (state->config.fps > 0) {
            // Assume timebase of 1/90000 (common for MPEG)
            frame_duration = 90000 / state->config.fps;
        } else {
            // Default to 30fps
            frame_duration = 3000;
        }

        state->timestamp_state.expected_next_pts = pts + frame_duration;
    }

    pthread_mutex_unlock(&state->mutex);

    return 0;
}

/**
 * Get total number of streams
 */
int get_stream_state_count(void) {
    if (!initialized) {
        return 0;
    }

    pthread_mutex_lock(&states_mutex);

    int count = 0;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (stream_states[i]) {
            count++;
        }
    }

    pthread_mutex_unlock(&states_mutex);
    return count;
}

/**
 * Get stream state manager by index
 */
stream_state_manager_t *get_stream_state_by_index(int index) {
    if (index < 0 || index >= MAX_STREAMS || !initialized) {
        return NULL;
    }

    pthread_mutex_lock(&states_mutex);

    stream_state_manager_t *state = stream_states[index];

    pthread_mutex_unlock(&states_mutex);
    return state;
}

/**
 * Remove stream state manager
 */
int remove_stream_state(stream_state_manager_t *state) {
    if (!state || !initialized) {
        return -1;
    }

    pthread_mutex_lock(&states_mutex);

    // Find the state in the array
    int slot = -1;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (stream_states[i] == state) {
            slot = i;
            break;
        }
    }

    if (slot == -1) {
        pthread_mutex_unlock(&states_mutex);
        log_error("Stream state not found in array");
        return -1;
    }

    // Save stream name for logging
    char stream_name[MAX_STREAM_NAME];
    strncpy(stream_name, state->name, MAX_STREAM_NAME - 1);
    stream_name[MAX_STREAM_NAME - 1] = '\0';

    // Check reference count
    pthread_mutex_lock(&state->mutex);
    int ref_count = state->ref_count;
    pthread_mutex_unlock(&state->mutex);

    if (ref_count > 1) {
        pthread_mutex_unlock(&states_mutex);
        log_warn("Cannot remove stream state for '%s' with reference count %d",
                stream_name, ref_count);
        return -1;
    }

    // Stop the stream if it's active
    if (state->state == STREAM_STATE_ACTIVE ||
        state->state == STREAM_STATE_STARTING ||
        state->state == STREAM_STATE_RECONNECTING ||
        state->state == STREAM_STATE_STOPPING) {
        pthread_mutex_unlock(&states_mutex);
        stop_stream_with_state(state, true); // Wait for completion
        pthread_mutex_lock(&states_mutex);
    }

    // Remove timestamp tracker
    remove_timestamp_tracker(stream_name);

    // Destroy mutexes
    pthread_mutex_destroy(&state->mutex);
    pthread_mutex_destroy(&state->state_mutex);

    // Free the state manager
    free(state);
    stream_states[slot] = NULL;

    pthread_mutex_unlock(&states_mutex);

    log_info("Removed stream state for '%s' from slot %d", stream_name, slot);
    return 0;
}

/**
 * Add a reference to a stream state manager
 */
int stream_state_add_ref(stream_state_manager_t *state, stream_component_t component) {
    if (!state) {
        return -1;
    }

    if (component < 0 || component >= STREAM_COMPONENT_COUNT) {
        log_error("Invalid component type %d for stream '%s'", component, state->name);
        return -1;
    }

    pthread_mutex_lock(&state->mutex);

    // Increment component-specific reference count
    state->component_refs[component]++;

    // Increment total reference count
    state->ref_count++;

    int new_ref_count = state->ref_count;

    pthread_mutex_unlock(&state->mutex);

    log_debug("Added reference to stream '%s' for component %d, new ref count: %d",
             state->name, component, new_ref_count);

    return new_ref_count;
}

/**
 * Release a reference to a stream state manager
 */
int stream_state_release_ref(stream_state_manager_t *state, stream_component_t component) {
    if (!state) {
        return -1;
    }

    if (component < 0 || component >= STREAM_COMPONENT_COUNT) {
        log_error("Invalid component type %d for stream '%s'", component, state->name);
        return -1;
    }

    pthread_mutex_lock(&state->mutex);

    // Check if component has any references
    if (state->component_refs[component] <= 0) {
        log_warn("Component %d has no references to release for stream '%s'",
                component, state->name);
        pthread_mutex_unlock(&state->mutex);
        return state->ref_count;
    }

    // Decrement component-specific reference count
    state->component_refs[component]--;

    // Decrement total reference count
    state->ref_count--;

    int new_ref_count = state->ref_count;

    pthread_mutex_unlock(&state->mutex);

    log_debug("Released reference to stream '%s' for component %d, new ref count: %d",
             state->name, component, new_ref_count);

    return new_ref_count;
}

/**
 * Get the current reference count for a stream state manager
 */
int stream_state_get_ref_count(stream_state_manager_t *state) {
    if (!state) {
        return -1;
    }

    pthread_mutex_lock(&state->mutex);
    int ref_count = state->ref_count;
    pthread_mutex_unlock(&state->mutex);

    return ref_count;
}

/**
 * Check if a stream is in the process of stopping
 */
bool is_stream_state_stopping(stream_state_manager_t *state) {
    if (!state) {
        return false;
    }

    pthread_mutex_lock(&state->state_mutex);
    bool stopping = (state->state == STREAM_STATE_STOPPING);
    pthread_mutex_unlock(&state->state_mutex);

    return stopping;
}

/**
 * Wait for a stream to complete its stopping process
 */
int wait_for_stream_stop(stream_state_manager_t *state, int timeout_ms) {
    if (!state) {
        return -2;
    }

    // Calculate end time
    struct timeval end_time;
    gettimeofday(&end_time, NULL);

    // Add timeout to end time
    end_time.tv_sec += timeout_ms / 1000;
    end_time.tv_usec += (timeout_ms % 1000) * 1000;
    if (end_time.tv_usec >= 1000000) {
        end_time.tv_sec += 1;
        end_time.tv_usec -= 1000000;
    }

    // Poll until the stream is inactive or timeout
    while (1) {
        pthread_mutex_lock(&state->state_mutex);
        bool is_inactive = (state->state == STREAM_STATE_INACTIVE);
        pthread_mutex_unlock(&state->state_mutex);

        if (is_inactive) {
            return 0;  // Success
        }

        // Check if we've timed out
        struct timeval current_time;
        gettimeofday(&current_time, NULL);
        if (current_time.tv_sec > end_time.tv_sec ||
            (current_time.tv_sec == end_time.tv_sec &&
             current_time.tv_usec >= end_time.tv_usec)) {
            return -1;  // Timeout
        }

        // Sleep for a short time before checking again
        usleep(10000);  // 10ms
    }
}

/**
 * Enable or disable callbacks for a stream
 */
int set_stream_callbacks_enabled(stream_state_manager_t *state, bool enabled) {
    if (!state) {
        return -1;
    }

    pthread_mutex_lock(&state->mutex);
    state->callbacks_enabled = enabled;
    pthread_mutex_unlock(&state->mutex);

    log_info("Callbacks %s for stream '%s'",
            enabled ? "enabled" : "disabled", state->name);

    return 0;
}

/**
 * Check if callbacks are enabled for a stream
 */
bool are_stream_callbacks_enabled(stream_state_manager_t *state) {
    if (!state) {
        return false;
    }

    pthread_mutex_lock(&state->mutex);
    bool enabled = state->callbacks_enabled;
    pthread_mutex_unlock(&state->mutex);

    return enabled;
}
