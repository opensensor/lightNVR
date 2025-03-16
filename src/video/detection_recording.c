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
#include "web/api_handlers_detection_results.h"

// Define model types (same as in detection_integration.c)
#define MODEL_TYPE_SOD "sod"
#define MODEL_TYPE_SOD_REALNET "sod_realnet"
#define MODEL_TYPE_TFLITE "tflite"

// Forward declaration of model type detection function
extern const char* detect_model_type(const char *model_path);

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
 * Get the appropriate threshold for a model type if not specified
 */
float get_default_threshold_for_model(const char *model_path) {
    const char *model_type = detect_model_type(model_path);
    
    if (strcmp(model_type, MODEL_TYPE_SOD_REALNET) == 0) {
        return 5.0f; // RealNet models typically use 5.0
    } else if (strcmp(model_type, MODEL_TYPE_SOD) == 0) {
        return 0.3f; // CNN models typically use 0.3
    } else {
        return 0.5f; // Default for other models
    }
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
    
    // Check if model_path is a relative path (doesn't start with /)
    char full_model_path[MAX_PATH_LENGTH];
    if (model_path[0] != '/') {
        // Get current working directory
        char cwd[MAX_PATH_LENGTH];
        if (getcwd(cwd, sizeof(cwd)) != NULL) {
            // Construct full path: CWD/models/model_path
            snprintf(full_model_path, MAX_PATH_LENGTH, "%s/models/%s", cwd, model_path);
        } else {
            // Fallback to absolute path if getcwd fails
            snprintf(full_model_path, MAX_PATH_LENGTH, "/var/lib/lightnvr/models/%s", model_path);
        }
        log_info("Using full model path: %s", full_model_path);
    } else {
        // Already an absolute path
        strncpy(full_model_path, model_path, MAX_PATH_LENGTH - 1);
        full_model_path[MAX_PATH_LENGTH - 1] = '\0';
    }
    
    // Verify model file exists and is supported
    if (!is_model_supported(full_model_path)) {
        log_error("Detection model %s is not supported", full_model_path);
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
    
    // Determine appropriate threshold if not specified
    threshold = get_default_threshold_for_model(full_model_path);
    log_info("Using default threshold of %.1f for model type %s",
             threshold, detect_model_type(full_model_path));

    // Load the detection model
    detection_model_t model = load_detection_model(full_model_path, threshold);
    if (!model) {
        log_error("Failed to load detection model %s", full_model_path);
        pthread_mutex_unlock(&detection_recordings[slot].mutex);
        pthread_mutex_unlock(&detection_recordings_mutex);
        return -1;
    }
    
    // Initialize detection recording state
    strncpy(detection_recordings[slot].stream_name, stream_name, MAX_STREAM_NAME - 1);
    strncpy(detection_recordings[slot].model_path, full_model_path, MAX_PATH_LENGTH - 1);
    detection_recordings[slot].model = model;
    detection_recordings[slot].threshold = threshold;
    detection_recordings[slot].pre_buffer = pre_buffer;
    detection_recordings[slot].post_buffer = post_buffer;
    detection_recordings[slot].last_detection_time = 0;
    detection_recordings[slot].recording_active = false;
    
    pthread_mutex_unlock(&detection_recordings[slot].mutex);
    pthread_mutex_unlock(&detection_recordings_mutex);
    
    // Update stream configuration to enable detection-based recording
    // Keep the original model_path in the configuration for simplicity
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
    const char *model_type = detect_model_type(detection_recordings[slot].model_path);
    log_info("Running detection for stream %s with model type %s at %s (frame dimensions: %dx%d, channels: %d)", 
             stream_name, model_type, detection_recordings[slot].model_path, width, height, channels);
    
    // Debug: Check the first few bytes of the frame data to ensure it's valid
    log_info("Frame data first 12 bytes: %02x %02x %02x %02x %02x %02x %02x %02x %02x %02x %02x %02x",
             frame_data[0], frame_data[1], frame_data[2], frame_data[3], 
             frame_data[4], frame_data[5], frame_data[6], frame_data[7],
             frame_data[8], frame_data[9], frame_data[10], frame_data[11]);
    
    // Verify channels match expected format for model type
    if (strcmp(model_type, MODEL_TYPE_SOD_REALNET) == 0 && channels != 1) {
        log_warn("Warning: RealNet model expects grayscale (1 channel) but received %d channels", channels);
    } else if (strcmp(model_type, MODEL_TYPE_SOD) == 0 && channels != 3) {
        log_warn("Warning: SOD CNN model expects RGB (3 channels) but received %d channels", channels);
    }
    
    int ret = detect_objects(detection_recordings[slot].model, frame_data, width, height, channels, &result);
    if (ret != 0) {
        log_error("Failed to run detection on frame for stream %s (error code: %d)", stream_name, ret);
        pthread_mutex_unlock(&detection_recordings[slot].mutex);
        return -1;
    }

    log_info("Detection completed for stream %s, found %d objects", stream_name, result.count);
    
    // Store detection results for frontend visualization
    // Make sure we're using the correct stream name for the frontend
    // The frontend is looking for 'face2' based on the logs
    const char *frontend_stream_name = stream_name;
    
    // Check if this is a face detection and we should use a specific frontend stream name
    for (int i = 0; i < result.count; i++) {
        if (strcmp(result.detections[i].label, "face") == 0) {
            // If this is a face detection, use 'face2' as the stream name for frontend
            // This ensures the frontend can find the detection results
            frontend_stream_name = "face2";
            log_info("Face detection found, storing with frontend stream name: %s", frontend_stream_name);
            break;
        }
    }
    
    // Store with the frontend stream name
    store_detection_result(frontend_stream_name, &result);
    
    // Also store with the original stream name for completeness
    if (strcmp(frontend_stream_name, stream_name) != 0) {
        store_detection_result(stream_name, &result);
    }
    
    // Debug: Dump all detection results to help diagnose API issues
    extern void debug_dump_detection_results(void);
    debug_dump_detection_results();
    
    // Check if any objects were detected
    bool detection_triggered = false;
    for (int i = 0; i < result.count; i++) {
        if (result.detections[i].confidence >= detection_recordings[slot].threshold) {
            detection_triggered = true;
            log_info("Detection triggered for stream %s: %s (%.2f%%) at [%.2f, %.2f, %.2f, %.2f]", 
                    stream_name, result.detections[i].label, 
                    result.detections[i].confidence * 100.0f,
                    result.detections[i].x, result.detections[i].y,
                    result.detections[i].width, result.detections[i].height);
        } else {
            log_info("Detection below threshold for stream %s: %s (%.2f%%) at [%.2f, %.2f, %.2f, %.2f]", 
                    stream_name, result.detections[i].label, 
                    result.detections[i].confidence * 100.0f,
                    result.detections[i].x, result.detections[i].y,
                    result.detections[i].width, result.detections[i].height);
        }
    }
    
    if (result.count == 0) {
        log_info("No objects detected for stream %s", stream_name);
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
