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
#include "video/detection.h"
#include "video/detection_result.h"
#include "video/detection_stream.h"
#include "database/database_manager.h"
#include "web/api_handlers_detection_results.h"

// Define model types (same as in detection_integration.c)
#define MODEL_TYPE_SOD "sod"
#define MODEL_TYPE_SOD_REALNET "sod_realnet"
#define MODEL_TYPE_TFLITE "tflite"
#define MAX_DETECTION_AGE 30 // Maximum age of detections to consider (in seconds)

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
    
    // Get global config to access models path
    extern config_t g_config;
    
    // Check if model_path is a relative path (doesn't start with /)
    char full_model_path[MAX_PATH_LENGTH];
    if (model_path[0] != '/') {
        // Construct full path using configured models path from INI if it exists
        if (g_config.models_path && strlen(g_config.models_path) > 0) {
            snprintf(full_model_path, MAX_PATH_LENGTH, "%s/%s", g_config.models_path, model_path);
        } else {
            // Fall back to default path if INI config doesn't exist
            snprintf(full_model_path, MAX_PATH_LENGTH, "/etc/lightnvr/models/%s", model_path);
        }
        log_info("Using full model path: %s", full_model_path);
    } else {
        // Already an absolute path
        strncpy(full_model_path, model_path, MAX_PATH_LENGTH - 1);
        full_model_path[MAX_PATH_LENGTH - 1] = '\0';
    }
    
    // Special case for motion detection
    bool is_motion_detection = (strcmp(model_path, "motion") == 0);
    
    // Verify model file exists and is supported (unless it's motion detection)
    if (!is_motion_detection && !is_model_supported(full_model_path)) {
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
    
    // Use the provided threshold or get a default if not specified
    if (threshold <= 0.0f) {
        threshold = get_default_threshold_for_model(full_model_path);
        log_info("Using default threshold of %.1f for model type %s",
                threshold, detect_model_type(full_model_path));
    } else {
        log_info("Using provided threshold of %.1f for stream %s",
                threshold, stream_name);
    }

    // Load the detection model (unless it's motion detection)
    detection_model_t model = NULL;
    if (!is_motion_detection) {
        model = load_detection_model(full_model_path, threshold);
        if (!model) {
            log_error("Failed to load detection model %s", full_model_path);
            pthread_mutex_unlock(&detection_recordings[slot].mutex);
            pthread_mutex_unlock(&detection_recordings_mutex);
            return -1;
        }
    } else {
        log_info("Using motion detection instead of a model for stream %s", stream_name);
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
    
    // Use the provided detection interval or default to 10 if not specified
    int detection_interval = 10;
    stream_config_t config;
    if (get_stream_config(stream, &config) == 0) {
        detection_interval = config.detection_interval;
    }
    
    set_stream_detection_params(stream, detection_interval, threshold, pre_buffer, post_buffer);
    
    // Start a dedicated stream reader for detection
    int ret = start_detection_stream_reader(stream_name, detection_interval);
    if (ret != 0) {
        log_error("Failed to start detection stream reader for stream %s", stream_name);
        // Continue anyway, as the detection recording can still work with HLS-based detection
    } else {
        log_info("Started detection stream reader for stream %s with interval %d", stream_name, detection_interval);
    }
    
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
        // Stop the recording - this will also unregister any MP4 writer
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
    
    // Stop the detection stream reader
    int ret = stop_detection_stream_reader(stream_name);
    if (ret != 0) {
        log_warn("Failed to stop detection stream reader for stream %s", stream_name);
    } else {
        log_info("Stopped detection stream reader for stream %s", stream_name);
    }
    
    log_info("Stopped detection-based recording for stream %s", stream_name);
    
    return 0;
}

/**
 * Process detection results for recording
 * This function manages recording decisions based on detection results
 *
 * @param stream_name The name of the stream
 * @param frame_data The frame data (for debugging only)
 * @param width Frame width
 * @param height Frame height
 * @param channels Number of color channels
 * @param frame_time Timestamp of the frame
 * @param result Detection results from detect_objects call
 * @return 0 on success, -1 on error
 */
// Static counter to track frames for each stream
static int frame_counters[MAX_STREAMS] = {0};
static char frame_counter_stream_names[MAX_STREAMS][MAX_STREAM_NAME] = {{0}};
static pthread_mutex_t frame_counters_mutex = PTHREAD_MUTEX_INITIALIZER;

int process_frame_for_recording(const char *stream_name, const unsigned char *frame_data,
                               int width, int height, int channels, time_t frame_time,
                               detection_result_t *result) {
    if (!stream_name || !frame_data || !result) {
        log_error("Invalid parameters for process_frame_for_recording");
        return -1;
    }

    // Get the stream configuration directly
    stream_handle_t stream = get_stream_by_name(stream_name);
    if (!stream) {
        log_error("Failed to get stream handle for %s", stream_name);
        return -1;
    }

    stream_config_t config;
    if (get_stream_config(stream, &config) != 0) {
        log_error("Failed to get stream config for %s", stream_name);
        return -1;
    }

    // Check if detection is enabled for this stream
    if (!config.detection_based_recording || config.detection_model[0] == '\0') {
        log_info("Detection-based recording not enabled for stream %s", stream_name);
        return 0;
    }

    // Get detection parameters from stream config
    float threshold = config.detection_threshold;
    
    // Log the threshold value for debugging
    log_info("Detection threshold from config: %.2f for stream %s", threshold, stream_name);
    
    // No longer enforcing a minimum threshold - use the user's configured value
    log_info("Using user-configured detection threshold of %.2f (%.0f%%) for stream %s", 
             threshold, threshold * 100.0f, stream_name);
    
    // Get stream index for frame counter
    int stream_index = -1;
    pthread_mutex_lock(&frame_counters_mutex);
    
    // Look for existing stream entry
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (frame_counter_stream_names[i][0] != '\0' && 
            strcmp(frame_counter_stream_names[i], stream_name) == 0) {
            stream_index = i;
            break;
        }
    }
    
    // If stream not found, find first empty slot
    if (stream_index == -1) {
        for (int i = 0; i < MAX_STREAMS; i++) {
            if (frame_counter_stream_names[i][0] == '\0') {
                stream_index = i;
                strncpy(frame_counter_stream_names[i], stream_name, MAX_STREAM_NAME - 1);
                frame_counter_stream_names[i][MAX_STREAM_NAME - 1] = '\0';
                frame_counters[i] = 0;
                break;
            }
        }
    }
    
    // If still no slot found, use a default index (not ideal but prevents crashes)
    if (stream_index == -1) {
        stream_index = 0;
        log_warn("No slot found for stream %s frame counter, using default", stream_name);
    }
    
    // Check if we should process this frame based on detection interval
    bool should_process = false;
    
    // If interval is 1, process every frame
    if (config.detection_interval <= 1) {
        should_process = true;
        log_info("Processing every frame for detection on stream %s", stream_name);
    } else {
        // Otherwise, use the counter
        frame_counters[stream_index]++;
        
        // Process frame if counter reaches the interval or it's the first frame
        if (frame_counters[stream_index] >= config.detection_interval || frame_counters[stream_index] == 1) {
            should_process = true;
            frame_counters[stream_index] = 0; // Reset counter
            log_info("Processing frame for detection on stream %s (interval: %d)", 
                    stream_name, config.detection_interval);
        }
    }
    pthread_mutex_unlock(&frame_counters_mutex);
    
    // Skip processing if not at the right interval
    if (!should_process) {
        return 0;
    }
    
    // Store with the original stream name
    log_info("STORING DETECTION RESULTS with original stream name: %s (count: %d)", stream_name, result->count);

    // Call store_detection_result but don't try to capture the return value since it's void
    store_detection_result(stream_name, result);
    log_info("DETECTION RESULT STORED");

    // Check if any objects were detected above threshold
    bool detection_triggered = false;
    for (int i = 0; i < result->count; i++) {
        if (result->detections[i].confidence >= threshold) {
            detection_triggered = true;
            log_info("DETECTION TRIGGERED for stream %s: %s (%.2f%%) at [%.2f, %.2f, %.2f, %.2f]",
                    stream_name, result->detections[i].label,
                    result->detections[i].confidence * 100.0f,
                    result->detections[i].x, result->detections[i].y,
                    result->detections[i].width, result->detections[i].height);
        } else {
            log_debug("DETECTION BELOW THRESHOLD for stream %s: %s (%.2f%%) at [%.2f, %.2f, %.2f, %.2f]",
                    stream_name, result->detections[i].label,
                    result->detections[i].confidence * 100.0f,
                    result->detections[i].x, result->detections[i].y,
                    result->detections[i].width, result->detections[i].height);
        }
    }

    if (result->count == 0) {
        log_debug("NO OBJECTS DETECTED for stream %s", stream_name);
    }

    // Update last detection time if triggered
    if (detection_triggered) {
        // Using direct stream configuration for more reliable behavior
        set_stream_last_detection_time(stream, frame_time);
    }

    // Query the database for recent detections to determine if we should be recording
    // This makes the database the source of truth for detection state
    detection_result_t db_result;

    // Get detections from database since cutoff time
    int db_count = get_detections_from_db(stream_name, &db_result, MAX_DETECTION_AGE);

    if (db_count < 0) {
        log_error("Failed to get detections from database for stream %s (error code: %d)",
                 stream_name, db_count);
        // Fall back to current detection result if database query fails
        db_count = 0;
    } else {
        log_info("Retrieved %d detections from database for stream %s", db_count, stream_name);
    }

    // Check if we have any recent detections in the database
    bool should_be_recording = false;
    if (db_count > 0) {
        // Check if any detections are above threshold
        for (int i = 0; i < db_result.count; i++) {
            if (db_result.detections[i].confidence >= threshold) {
                should_be_recording = true;
                log_info("Recent detection found in database for stream %s: %s (%.2f%%)",
                        stream_name, db_result.detections[i].label,
                        db_result.detections[i].confidence * 100.0f);
                break;
            }
        }
    }

    // Also consider current detection
    should_be_recording = should_be_recording || detection_triggered;

    // Check if we should start or continue recording
    if (should_be_recording) {
        // Check current recording state
        bool recording_active = false;
        int recording_state = get_recording_state(stream_name);
        if (recording_state > 0) {
            recording_active = true;
        }

        // If not already recording, start recording
        if (!recording_active) {
            // CRITICAL FIX: Get the pre-buffer size from the stream config
            int pre_buffer = config.pre_detection_buffer;
            
            // Start MP4 recording directly, using the same file rotation settings as regular recordings
            int mp4_result = start_mp4_recording(stream_name);
            if (mp4_result == 0) {
                log_info("Started MP4 recording for detection event on stream %s with pre-buffer of %d seconds", 
                         stream_name, pre_buffer);
                
                // CRITICAL FIX: Flush the pre-buffered frames to the MP4 writer
                // This function is defined in mp4_recording.c
                extern void flush_prebuffer_to_mp4(const char *stream_name);
                flush_prebuffer_to_mp4(stream_name);
                
                // Update the recording_active flag in the detection_recordings array
                pthread_mutex_lock(&detection_recordings_mutex);
                for (int i = 0; i < MAX_STREAMS; i++) {
                    if (detection_recordings[i].stream_name[0] != '\0' && 
                        strcmp(detection_recordings[i].stream_name, stream_name) == 0) {
                        pthread_mutex_lock(&detection_recordings[i].mutex);
                        detection_recordings[i].recording_active = true;
                        pthread_mutex_unlock(&detection_recordings[i].mutex);
                        break;
                    }
                }
                pthread_mutex_unlock(&detection_recordings_mutex);
            } else {
                log_error("Failed to start MP4 recording for detection event on stream %s (error code: %d)",
                        stream_name, mp4_result);
            }
        } else {
            log_info("Continuing detection-based recording for stream %s", stream_name);
        }
    } else {
        // Check if we're currently recording
        int recording_state = get_recording_state(stream_name);
        if (recording_state > 0) {
            // If recording and no recent detections, stop recording
            stop_recording(stream_name);
        }
    }

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
    
    // First check if detection is enabled for this stream
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
    
    pthread_mutex_unlock(&detection_recordings_mutex);
    
    // Now check if there's an active recording using get_recording_state
    int recording_state = get_recording_state(stream_name);
    *recording_active = (recording_state > 0);
    
    return 1;
}
