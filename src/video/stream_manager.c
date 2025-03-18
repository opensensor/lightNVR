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

// Stream structure
typedef struct {
    stream_config_t config;
    stream_status_t status;
    stream_stats_t stats;
    pthread_mutex_t mutex;
    bool recording_enabled;
    bool detection_recording_enabled;
    time_t last_detection_time;  // Added for detection-based recording
    
    // New architecture components
    stream_reader_ctx_t *reader;
    stream_processor_t processor;
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
        streams[i].reader = NULL;
        streams[i].processor = NULL;
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
    
    log_info("Stream manager initialized with new architecture");
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
            
            pthread_mutex_unlock(&streams_mutex);
            
            // Stop the stream using the new architecture
            stream_handle_t handle = (stream_handle_t)&streams[i];
            stop_stream(handle);
            
            pthread_mutex_lock(&streams_mutex);
        }
    }
    
    // Clean up all stream processors and readers
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (streams[i].config.name[0] != '\0') {
            // Clean up processor
            if (streams[i].processor) {
                stream_processor_destroy(streams[i].processor);
                streams[i].processor = NULL;
            }
            
            // Clean up reader
            if (streams[i].reader) {
                stop_stream_reader(streams[i].reader);
                streams[i].reader = NULL;
            }
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
    
    log_info("Stream manager shutdown with new architecture");
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
    
    // Get streaming_enabled flag
    bool streaming_enabled = s->config.streaming_enabled;
    
    // Get recording_enabled flag
    bool recording_enabled = s->config.record;
    
    // Get detection_enabled flag
    bool detection_enabled = s->config.detection_based_recording;
    
    // Get stream URL for UDP detection
    char stream_url[MAX_PATH_LENGTH];
    strncpy(stream_url, s->config.url, MAX_PATH_LENGTH - 1);
    stream_url[MAX_PATH_LENGTH - 1] = '\0';
    
    pthread_mutex_unlock(&s->mutex);
    
    // CRITICAL FIX: Reset timestamp tracker before starting the stream
    // This ensures clean timestamp handling for the new stream instance
    reset_timestamp_tracker(stream_name);
    
    // CRITICAL FIX: Set UDP flag for timestamp tracker based on stream URL
    if (strncmp(stream_url, "udp://", 6) == 0 || strncmp(stream_url, "rtp://", 6) == 0) {
        log_info("Setting UDP flag for stream %s based on URL: %s", stream_name, stream_url);
        set_timestamp_tracker_udp_flag(stream_name, true);
    }
    
    // Create stream reader if it doesn't exist
    if (!s->reader) {
        s->reader = start_stream_reader(stream_name, 0, NULL, NULL);
        if (!s->reader) {
            log_error("Failed to create stream reader for '%s'", stream_name);
            
            pthread_mutex_lock(&s->mutex);
            s->status = STREAM_STATUS_ERROR;
            pthread_mutex_unlock(&s->mutex);
            
            return -1;
        }
        log_info("Created stream reader for '%s'", stream_name);
    }
    
    // CRITICAL FIX: Clean up existing processor if it exists
    // This prevents memory leaks and ensures a clean start
    if (s->processor) {
        log_info("Cleaning up existing processor for '%s' before creating a new one", stream_name);
        stream_processor_stop(s->processor);
        stream_processor_destroy(s->processor);
        s->processor = NULL;
    }
    
    // Create stream processor
    s->processor = stream_processor_create(stream_name, s->reader);
    if (!s->processor) {
        log_error("Failed to create stream processor for '%s'", stream_name);
        
        // Clean up reader if we created it
        if (s->reader) {
            stop_stream_reader(s->reader);
            s->reader = NULL;
        }
        
        pthread_mutex_lock(&s->mutex);
        s->status = STREAM_STATUS_ERROR;
        pthread_mutex_unlock(&s->mutex);
        
        return -1;
    }
    log_info("Created stream processor for '%s'", stream_name);
    
    // Add outputs to the processor based on configuration
    bool any_output_added = false;
    
    // Add HLS output if streaming is enabled
    if (streaming_enabled) {
        output_config_t output_config;
        memset(&output_config, 0, sizeof(output_config));
        output_config.type = OUTPUT_TYPE_HLS;
        
        // Get HLS output path from global config
        config_t *global_config = get_streaming_config();
        snprintf(output_config.hls.output_path, MAX_PATH_LENGTH, "%s/hls/%s",
                global_config->storage_path, stream_name);
        
        // Use segment duration from stream config or default to 4 seconds
        output_config.hls.segment_duration = s->config.segment_duration > 0 ?
                                           s->config.segment_duration : 4;
        
        if (stream_processor_add_output(s->processor, &output_config) == 0) {
            log_info("Added HLS output to stream processor for '%s'", stream_name);
            any_output_added = true;
        } else {
            log_error("Failed to add HLS output to stream processor for '%s'", stream_name);
        }
    }
    
    // Add MP4 output if recording is enabled
    if (recording_enabled) {
        output_config_t output_config;
        memset(&output_config, 0, sizeof(output_config));
        output_config.type = OUTPUT_TYPE_MP4;
        
        // Get MP4 output path from global config
        config_t *global_config = get_streaming_config();
        snprintf(output_config.mp4.output_path, MAX_PATH_LENGTH, "%s/recordings/%s",
                global_config->storage_path, stream_name);
        
        // Use segment duration from stream config or default to 60 seconds (1 minute)
        output_config.mp4.segment_duration = s->config.segment_duration > 0 ?
                                           s->config.segment_duration : 60;
        
        if (stream_processor_add_output(s->processor, &output_config) == 0) {
            log_info("Added MP4 output to stream processor for '%s'", stream_name);
            any_output_added = true;
        } else {
            log_error("Failed to add MP4 output to stream processor for '%s'", stream_name);
        }
    }
    
    // Add detection output if detection is enabled
    if (detection_enabled) {
        output_config_t output_config;
        memset(&output_config, 0, sizeof(output_config));
        output_config.type = OUTPUT_TYPE_DETECTION;
        
        // Copy model path from stream config
        strncpy(output_config.detection.model_path, s->config.detection_model, MAX_PATH_LENGTH - 1);
        output_config.detection.model_path[MAX_PATH_LENGTH - 1] = '\0';
        
        // Copy detection parameters from stream config
        output_config.detection.threshold = s->config.detection_threshold;
        output_config.detection.interval = s->config.detection_interval;
        output_config.detection.pre_buffer = s->config.pre_detection_buffer;
        output_config.detection.post_buffer = s->config.post_detection_buffer;
        
        if (stream_processor_add_output(s->processor, &output_config) == 0) {
            log_info("Added detection output to stream processor for '%s'", stream_name);
            any_output_added = true;
        } else {
            log_error("Failed to add detection output to stream processor for '%s'", stream_name);
        }
    }
    
    // Start the processor if any outputs were added
    if (any_output_added) {
        if (stream_processor_start(s->processor) == 0) {
            log_info("Started stream processor for '%s'", stream_name);
            
            pthread_mutex_lock(&s->mutex);
            s->status = STREAM_STATUS_RUNNING;
            pthread_mutex_unlock(&s->mutex);
            
            return 0;
        } else {
            log_error("Failed to start stream processor for '%s'", stream_name);
        }
    } else {
        log_error("No outputs were added to stream processor for '%s'", stream_name);
    }
    
    // If we get here, something went wrong
    pthread_mutex_lock(&s->mutex);
    s->status = STREAM_STATUS_ERROR;
    pthread_mutex_unlock(&s->mutex);
    
    return -1;
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
    
    // CRITICAL FIX: Reset timestamp tracker before stopping the stream
    // This ensures clean timestamp handling for the next stream start
    reset_timestamp_tracker(stream_name);
    
    // Stop the processor if it exists
    if (s->processor) {
        // CRITICAL FIX: Add proper error handling for processor stop
        int retry_count = 0;
        int max_retries = 3;
        bool stop_success = false;
        
        while (retry_count < max_retries && !stop_success) {
            if (stream_processor_stop(s->processor) == 0) {
                log_info("Stopped stream processor for '%s'", stream_name);
                stop_success = true;
            } else {
                log_warn("Failed to stop stream processor for '%s' (attempt %d/%d)", 
                        stream_name, retry_count + 1, max_retries);
                
                // Wait a moment before retrying
                usleep(100000);  // 100ms
                retry_count++;
            }
        }
        
        if (!stop_success) {
            log_error("Failed to stop stream processor for '%s' after %d attempts", 
                     stream_name, max_retries);
            // Continue anyway, but log the error
        }
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
    
    pthread_mutex_lock(&s->mutex);
    s->config.streaming_enabled = enabled;
    pthread_mutex_unlock(&s->mutex);
    
    // Update config
    config_t *global_config = get_streaming_config();
    if (global_config) {
        for (int i = 0; i < global_config->max_streams && i < MAX_STREAMS; i++) {
            if (strcmp(global_config->streams[i].name, s->config.name) == 0) {
                global_config->streams[i].streaming_enabled = enabled;
                break;
            }
        }
    }
    
    log_info("Set streaming enabled for stream '%s' to %s", s->config.name, enabled ? "enabled" : "disabled");
    return 0;
}

/**
 * Get the stream processor for a stream
 */
stream_processor_t get_stream_processor(stream_handle_t handle) {
    if (!handle || !initialized) {
        return NULL;
    }
    
    stream_t *s = (stream_t *)handle;
    
    pthread_mutex_lock(&s->mutex);
    stream_processor_t processor = s->processor;
    pthread_mutex_unlock(&s->mutex);
    
    return processor;
}

/**
 * Get the stream reader for a stream
 */
stream_reader_ctx_t *get_stream_reader_from_handle(stream_handle_t handle) {
    if (!handle || !initialized) {
        return NULL;
    }
    
    stream_t *s = (stream_t *)handle;
    
    pthread_mutex_lock(&s->mutex);
    stream_reader_ctx_t *reader = s->reader;
    pthread_mutex_unlock(&s->mutex);
    
    return reader;
}

/**
 * Get a snapshot from the stream
 */
int get_stream_snapshot(stream_handle_t handle, const char *path) {
    if (!handle || !path || !initialized) {
        return -1;
    }
    
    stream_t *s = (stream_t *)handle;
    
    // Check if the stream is running
    pthread_mutex_lock(&s->mutex);
    if (s->status != STREAM_STATUS_RUNNING) {
        log_error("Cannot take snapshot: stream '%s' is not running", s->config.name);
        pthread_mutex_unlock(&s->mutex);
        return -1;
    }
    
    // Get the stream processor
    stream_processor_t processor = s->processor;
    pthread_mutex_unlock(&s->mutex);
    
    if (!processor) {
        log_error("Cannot take snapshot: stream processor not found for '%s'", s->config.name);
        return -1;
    }
    
    // TODO: Implement snapshot functionality in the stream processor
    log_warn("Snapshot functionality not yet implemented in the new architecture");
    return -1;
}

/**
 * Get memory usage statistics for the stream manager
 */
int get_stream_manager_memory_usage(uint64_t *used_memory, uint64_t *peak_memory) {
    if (!used_memory || !peak_memory || !initialized) {
        return -1;
    }
    
    // TODO: Implement memory usage tracking in the new architecture
    *used_memory = 0;
    *peak_memory = 0;
    
    log_warn("Memory usage tracking not yet implemented in the new architecture");
    return 0;
}
