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

#include "core/logger.h"
#include "core/config.h"
#include "video/stream_manager.h"
#include "video/streams.h"
#include "video/mp4_writer.h"
#include "video/detection.h"
#include "video/detection_result.h"
#include "database/database_manager.h"

// Structure to track detection-based recording state
typedef struct {
    char stream_name[MAX_STREAM_NAME];
    char model_path[MAX_PATH_LENGTH];
    detection_model_t model;
    float threshold;
    int pre_buffer;
    int post_buffer;
    time_t last_detection_time;
    bool recording_active;
    pthread_mutex_t mutex;
} detection_recording_t;

// Array to store detection recording states
static detection_recording_t detection_recordings[MAX_STREAMS];
static pthread_mutex_t detection_recordings_mutex = PTHREAD_MUTEX_INITIALIZER;

/**
 * Initialize detection-based recording system
 */
void init_detection_recording_system(void) {
    pthread_mutex_lock(&detection_recordings_mutex);
    
    for (int i = 0; i < MAX_STREAMS; i++) {
        memset(&detection_recordings[i], 0, sizeof(detection_recording_t));
        pthread_mutex_init(&detection_recordings[i].mutex, NULL);
    }
    
    pthread_mutex_unlock(&detection_recordings_mutex);
    
    log_info("Detection-based recording system initialized");
}

/**
 * Shutdown detection-based recording system
 */
void shutdown_detection_recording_system(void) {
    pthread_mutex_lock(&detection_recordings_mutex);
    
    for (int i = 0; i < MAX_STREAMS; i++) {
        pthread_mutex_lock(&detection_recordings[i].mutex);
        
        if (detection_recordings[i].model) {
            unload_detection_model(detection_recordings[i].model);
            detection_recordings[i].model = NULL;
        }
        
        pthread_mutex_unlock(&detection_recordings[i].mutex);
        pthread_mutex_destroy(&detection_recordings[i].mutex);
    }
    
    pthread_mutex_unlock(&detection_recordings_mutex);
    
    log_info("Detection-based recording system shutdown");
}

/**
 * Start detection-based recording for a stream
 */
int start_detection_recording(const char *stream_name, const char *model_path, float threshold,
                             int pre_buffer, int post_buffer) {
    if (!stream_name || !model_path) {
        log_error("Invalid parameters for start_detection_recording");
        return -1;
    }
    
    // Verify stream exists
    stream_handle_t stream = get_stream_by_name(stream_name);
    if (!stream) {
        log_error("Stream %s not found", stream_name);
        return -1;
    }
    
    // Verify model file exists and is supported
    if (!is_model_supported(model_path)) {
        log_error("Detection model %s is not supported", model_path);
        return -1;
    }
    
    pthread_mutex_lock(&detection_recordings_mutex);
    
    // Find an empty slot or existing entry for this stream
    int slot = -1;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (detection_recordings[i].stream_name[0] == '\0') {
            if (slot == -1) {
                slot = i;
            }
        } else if (strcmp(detection_recordings[i].stream_name, stream_name) == 0) {
            // Stream already has detection recording, update it
            slot = i;
            break;
        }
    }
    
    if (slot == -1) {
        log_error("No available slots for detection recording");
        pthread_mutex_unlock(&detection_recordings_mutex);
        return -1;
    }
    
    pthread_mutex_lock(&detection_recordings[slot].mutex);
    
    // If there's an existing model, unload it
    if (detection_recordings[slot].model) {
        unload_detection_model(detection_recordings[slot].model);
        detection_recordings[slot].model = NULL;
    }
    
    // Load the detection model
    detection_model_t model = load_detection_model(model_path, threshold);
    if (!model) {
        log_error("Failed to load detection model %s", model_path);
        pthread_mutex_unlock(&detection_recordings[slot].mutex);
        pthread_mutex_unlock(&detection_recordings_mutex);
        return -1;
    }
    
    // Initialize detection recording state
    strncpy(detection_recordings[slot].stream_name, stream_name, MAX_STREAM_NAME - 1);
    strncpy(detection_recordings[slot].model_path, model_path, MAX_PATH_LENGTH - 1);
    detection_recordings[slot].model = model;
    detection_recordings[slot].threshold = threshold;
    detection_recordings[slot].pre_buffer = pre_buffer;
    detection_recordings[slot].post_buffer = post_buffer;
    detection_recordings[slot].last_detection_time = 0;
    detection_recordings[slot].recording_active = false;
    
    pthread_mutex_unlock(&detection_recordings[slot].mutex);
    pthread_mutex_unlock(&detection_recordings_mutex);
    
    // Update stream configuration to enable detection-based recording
    set_stream_detection_recording(stream, true, model_path);
    set_stream_detection_params(stream, 10, threshold, pre_buffer, post_buffer);
    
    log_info("Started detection-based recording for stream %s with model %s", 
             stream_name, model_path);
    
    return 0;
}

/**
 * Stop detection-based recording for a stream
 */
int stop_detection_recording(const char *stream_name) {
    if (!stream_name) {
        log_error("Invalid stream name for stop_detection_recording");
        return -1;
    }
    
    pthread_mutex_lock(&detection_recordings_mutex);
    
    // Find the detection recording for this stream
    int slot = -1;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (detection_recordings[i].stream_name[0] != '\0' && 
            strcmp(detection_recordings[i].stream_name, stream_name) == 0) {
            slot = i;
            break;
        }
    }
    
    if (slot == -1) {
        log_warn("No detection recording found for stream %s", stream_name);
        pthread_mutex_unlock(&detection_recordings_mutex);
        return -1;
    }
    
    pthread_mutex_lock(&detection_recordings[slot].mutex);
    
    // Unload the detection model
    if (detection_recordings[slot].model) {
        unload_detection_model(detection_recordings[slot].model);
        detection_recordings[slot].model = NULL;
    }
    
    // If recording is active, stop it
    if (detection_recordings[slot].recording_active) {
        // Stop the recording
        stop_recording(stream_name);
        detection_recordings[slot].recording_active = false;
    }
    
    // Clear the detection recording state
    memset(detection_recordings[slot].stream_name, 0, MAX_STREAM_NAME);
    memset(detection_recordings[slot].model_path, 0, MAX_PATH_LENGTH);
    detection_recordings[slot].threshold = 0.0f;
    detection_recordings[slot].pre_buffer = 0;
    detection_recordings[slot].post_buffer = 0;
    detection_recordings[slot].last_detection_time = 0;
    
    pthread_mutex_unlock(&detection_recordings[slot].mutex);
    pthread_mutex_unlock(&detection_recordings_mutex);
    
    // Update stream configuration to disable detection-based recording
    stream_handle_t stream = get_stream_by_name(stream_name);
    if (stream) {
        set_stream_detection_recording(stream, false, NULL);
    }
    
    log_info("Stopped detection-based recording for stream %s", stream_name);
    
    return 0;
}

/**
 * Process a frame for detection-based recording
 * This function is called for each frame of a stream
 */
int process_frame_for_detection(const char *stream_name, const unsigned char *frame_data, 
                               int width, int height, int channels, time_t frame_time) {
    if (!stream_name || !frame_data) {
        return -1;
    }
    
    pthread_mutex_lock(&detection_recordings_mutex);
    
    // Find the detection recording for this stream
    int slot = -1;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (detection_recordings[i].stream_name[0] != '\0' && 
            strcmp(detection_recordings[i].stream_name, stream_name) == 0) {
            slot = i;
            break;
        }
    }
    
    if (slot == -1) {
        // No detection recording for this stream
        pthread_mutex_unlock(&detection_recordings_mutex);
        return 0;
    }
    
    pthread_mutex_lock(&detection_recordings[slot].mutex);
    pthread_mutex_unlock(&detection_recordings_mutex);
    
    // Check if we have a valid model
    if (!detection_recordings[slot].model) {
        pthread_mutex_unlock(&detection_recordings[slot].mutex);
        return -1;
    }
    
    // Run detection on the frame
    detection_result_t result;
    if (detect_objects(detection_recordings[slot].model, frame_data, width, height, channels, &result) != 0) {
        log_error("Failed to run detection on frame for stream %s", stream_name);
        pthread_mutex_unlock(&detection_recordings[slot].mutex);
        return -1;
    }
    
    // Check if any objects were detected
    bool detection_triggered = false;
    for (int i = 0; i < result.count; i++) {
        if (result.detections[i].confidence >= detection_recordings[slot].threshold) {
            detection_triggered = true;
            log_info("Detection triggered for stream %s: %s (%.2f%%)", 
                    stream_name, result.detections[i].label, 
                    result.detections[i].confidence * 100.0f);
            break;
        }
    }
    
    // Update last detection time if triggered
    if (detection_triggered) {
        detection_recordings[slot].last_detection_time = frame_time;
    }
    
    // Check if we should start or continue recording
    if (detection_triggered || 
        (detection_recordings[slot].last_detection_time > 0 && 
         frame_time - detection_recordings[slot].last_detection_time <= detection_recordings[slot].post_buffer)) {
        
        // If not already recording, start recording
        if (!detection_recordings[slot].recording_active) {
            // Create output path for recording
            char output_path[MAX_PATH_LENGTH];
            config_t *global_config = get_streaming_config();
            
            // Format timestamp for recording directory
            char timestamp_str[32];
            struct tm *tm_info = localtime(&frame_time);
            strftime(timestamp_str, sizeof(timestamp_str), "%Y%m%d_%H%M%S", tm_info);
            
            // Create output path
            snprintf(output_path, MAX_PATH_LENGTH, "%s/recordings/%s/detection_%s",
                    global_config->storage_path, stream_name, timestamp_str);
            
            // Start recording
            if (start_recording(stream_name, output_path) > 0) {
                detection_recordings[slot].recording_active = true;
                log_info("Started detection-based recording for stream %s at %s", 
                        stream_name, output_path);
            } else {
                log_error("Failed to start detection-based recording for stream %s", stream_name);
            }
        }
    } else if (detection_recordings[slot].recording_active) {
        // If recording and post-buffer has expired, stop recording
        if (detection_recordings[slot].last_detection_time > 0 && 
            frame_time - detection_recordings[slot].last_detection_time > detection_recordings[slot].post_buffer) {
            
            // Stop recording
            stop_recording(stream_name);
            detection_recordings[slot].recording_active = false;
            log_info("Stopped detection-based recording for stream %s after post-buffer", stream_name);
        }
    }
    
    pthread_mutex_unlock(&detection_recordings[slot].mutex);
    
    return 0;
}

/**
 * Get detection recording state for a stream
 * Returns 1 if detection recording is active, 0 if not, -1 on error
 */
int get_detection_recording_state(const char *stream_name, bool *recording_active) {
    if (!stream_name || !recording_active) {
        return -1;
    }
    
    pthread_mutex_lock(&detection_recordings_mutex);
    
    // Find the detection recording for this stream
    int slot = -1;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (detection_recordings[i].stream_name[0] != '\0' && 
            strcmp(detection_recordings[i].stream_name, stream_name) == 0) {
            slot = i;
            break;
        }
    }
    
    if (slot == -1) {
        // No detection recording for this stream
        *recording_active = false;
        pthread_mutex_unlock(&detection_recordings_mutex);
        return 0;
    }
    
    pthread_mutex_lock(&detection_recordings[slot].mutex);
    *recording_active = detection_recordings[slot].recording_active;
    pthread_mutex_unlock(&detection_recordings[slot].mutex);
    
    pthread_mutex_unlock(&detection_recordings_mutex);
    
    return 1;
}
