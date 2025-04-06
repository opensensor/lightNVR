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
#include "video/detection_stream_thread.h"
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
    
    // We'll let the monitor_hls_segments_for_detection function start the detection thread
    // once HLS segments are available. This ensures we don't start detection before
    // the go2rtc service has a chance to create the HLS segments.
    log_info("Detection configuration set for stream %s - detection thread will start when HLS segments are available", 
             stream_name);
    
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
    
    // Only store detections that meet the threshold
    detection_result_t filtered_result;
    memset(&filtered_result, 0, sizeof(detection_result_t));
    
    // Filter detections based on threshold
    for (int i = 0; i < result->count; i++) {
        if (result->detections[i].confidence >= threshold) {
            // Copy this detection to our filtered result
            memcpy(&filtered_result.detections[filtered_result.count], 
                   &result->detections[i], 
                   sizeof(detection_t));
            filtered_result.count++;
        }
    }
    
    // Only store if we have detections that meet the threshold
    if (filtered_result.count > 0) {
        log_info("STORING FILTERED DETECTION RESULTS with original stream name: %s (count: %d, original count: %d)", 
                stream_name, filtered_result.count, result->count);
        
        // Call store_detection_result but don't try to capture the return value since it's void
        store_detection_result(stream_name, &filtered_result);
        log_info("FILTERED DETECTION RESULT STORED");
    } else {
        log_info("No detections met the threshold (%.2f), skipping database storage", threshold);
    }

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
            //  Get the pre-buffer size from the stream config
            int pre_buffer = config.pre_detection_buffer;
            
            // Start MP4 recording directly, using the same file rotation settings as regular recordings
            int mp4_result = start_mp4_recording(stream_name);
            if (mp4_result == 0) {
                log_info("Started MP4 recording for detection event on stream %s with pre-buffer of %d seconds", 
                         stream_name, pre_buffer);
                
                // Pre-buffering is no longer used in the new architecture
                // Each recording thread manages its own RTSP connection directly
                log_info("Started MP4 recording for detection event on stream %s", stream_name);
                
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
 * Monitor HLS segments for a stream and submit them to the detection thread pool
 * This function is called periodically to check for new HLS segments
 * 
 * @param stream_name The name of the stream
 * @return 0 on success, -1 on error
 */
int monitor_hls_segments_for_detection(const char *stream_name) {
    if (!stream_name) {
        log_error("Invalid stream name for monitor_hls_segments_for_detection");
        return -1;
    }
    
    log_info("Monitoring HLS segments for detection on stream: %s", stream_name);
    
    // Get the stream configuration
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
    
    log_info("Detection is enabled for stream %s with model %s", 
            stream_name, config.detection_model);
    
    // Get the HLS directory for this stream using the config
    char hls_dir[MAX_PATH_LENGTH];
    
    // Use storage_path_hls if specified, otherwise fall back to storage_path
    const char *base_storage_path = g_config.storage_path;
    if (g_config.storage_path_hls[0] != '\0') {
        base_storage_path = g_config.storage_path_hls;
        log_info("Using dedicated HLS storage path: %s", base_storage_path);
    }
    
    // CRITICAL FIX: Ensure we're using the correct HLS directory path
    // First try the standard path
    char standard_path[MAX_PATH_LENGTH];
    snprintf(standard_path, MAX_PATH_LENGTH, "%s/hls/%s", base_storage_path, stream_name);
    
    // Also try the path with extra "hls" directory
    char alternative_path[MAX_PATH_LENGTH];
    snprintf(alternative_path, MAX_PATH_LENGTH, "%s/hls/hls/%s", base_storage_path, stream_name);
    
    struct stat st_standard, st_alternative;
    bool standard_exists = (stat(standard_path, &st_standard) == 0 && S_ISDIR(st_standard.st_mode));
    bool alternative_exists = (stat(alternative_path, &st_alternative) == 0 && S_ISDIR(st_alternative.st_mode));
    
    // Check which path exists and has segments
    if (standard_exists) {
        DIR *dir = opendir(standard_path);
        int segment_count = 0;
        if (dir) {
            struct dirent *entry;
            while ((entry = readdir(dir)) != NULL) {
                if (strstr(entry->d_name, ".ts") || strstr(entry->d_name, ".m4s")) {
                    segment_count++;
                }
            }
            closedir(dir);
        }
        
        if (segment_count > 0) {
            strncpy(hls_dir, standard_path, MAX_PATH_LENGTH - 1);
            hls_dir[MAX_PATH_LENGTH - 1] = '\0';
            log_info("Using standard HLS directory path with %d segments: %s", segment_count, hls_dir);
        } else if (alternative_exists) {
            // Standard path exists but has no segments, try alternative
            dir = opendir(alternative_path);
            segment_count = 0;
            if (dir) {
                struct dirent *entry;
                while ((entry = readdir(dir)) != NULL) {
                    if (strstr(entry->d_name, ".ts") || strstr(entry->d_name, ".m4s")) {
                        segment_count++;
                    }
                }
                closedir(dir);
            }
            
            if (segment_count > 0) {
                strncpy(hls_dir, alternative_path, MAX_PATH_LENGTH - 1);
                hls_dir[MAX_PATH_LENGTH - 1] = '\0';
                log_info("Using alternative HLS directory path with %d segments: %s", segment_count, hls_dir);
            } else {
                // No segments in either path, default to standard
                strncpy(hls_dir, standard_path, MAX_PATH_LENGTH - 1);
                hls_dir[MAX_PATH_LENGTH - 1] = '\0';
                log_info("No segments found, defaulting to standard HLS directory path: %s", hls_dir);
            }
        } else {
            // Alternative doesn't exist, use standard
            strncpy(hls_dir, standard_path, MAX_PATH_LENGTH - 1);
            hls_dir[MAX_PATH_LENGTH - 1] = '\0';
            log_info("Using standard HLS directory path: %s", hls_dir);
        }
    } else if (alternative_exists) {
        // Standard doesn't exist but alternative does
        strncpy(hls_dir, alternative_path, MAX_PATH_LENGTH - 1);
        hls_dir[MAX_PATH_LENGTH - 1] = '\0';
        log_info("Using alternative HLS directory path: %s", hls_dir);
    } else {
        // Neither exists, create and use standard path
        strncpy(hls_dir, standard_path, MAX_PATH_LENGTH - 1);
        hls_dir[MAX_PATH_LENGTH - 1] = '\0';
        log_info("Creating standard HLS directory path: %s", hls_dir);
        
        // Create the directory
        char cmd[MAX_PATH_LENGTH * 2];
        snprintf(cmd, sizeof(cmd), "mkdir -p %s", hls_dir);
        system(cmd);
    }
    
    // Check if directory exists
    struct stat st;
    if (stat(hls_dir, &st) != 0 || !S_ISDIR(st.st_mode)) {
        log_warn("HLS directory does not exist for stream %s: %s", stream_name, hls_dir);
        return -1;
    }
    
    // Open the directory
    DIR *dir = opendir(hls_dir);
    if (!dir) {
        log_error("Failed to open HLS directory for stream %s: %s", stream_name, hls_dir);
        return -1;
    }
    
    // Keep track of the newest segment file
    char newest_segment[MAX_PATH_LENGTH] = {0};
    time_t newest_time = 0;
    
    // Read directory entries
    struct dirent *entry;
    while ((entry = readdir(dir)) != NULL) {
        // Skip non-segment files (look for .ts or .m4s files)
        if (strstr(entry->d_name, ".ts") == NULL && strstr(entry->d_name, ".m4s") == NULL) {
            continue;
        }
        
        // Get file stats
        char segment_path[MAX_PATH_LENGTH];
        snprintf(segment_path, MAX_PATH_LENGTH, "%s/%s", hls_dir, entry->d_name);
        
        struct stat segment_stat;
        if (stat(segment_path, &segment_stat) != 0) {
            log_warn("Failed to stat segment file: %s", segment_path);
            continue;
        }
        
        // Check if this is the newest segment
        if (segment_stat.st_mtime > newest_time) {
            newest_time = segment_stat.st_mtime;
            strncpy(newest_segment, segment_path, MAX_PATH_LENGTH - 1);
            newest_segment[MAX_PATH_LENGTH - 1] = '\0';
        }
    }
    
    // Close the directory
    closedir(dir);
    
    // If we found a segment file, submit it to the detection thread pool
    if (newest_segment[0] != '\0') {
        // Check if this segment has already been processed
        static char last_processed_segment[MAX_STREAMS][MAX_PATH_LENGTH] = {{0}};
        static pthread_mutex_t last_processed_mutex = PTHREAD_MUTEX_INITIALIZER;
        
        pthread_mutex_lock(&last_processed_mutex);
        
        // Find the stream's entry
        int stream_idx = -1;
        for (int i = 0; i < MAX_STREAMS; i++) {
            if (last_processed_segment[i][0] == '\0') {
                // Empty slot, use it
                if (stream_idx == -1) {
                    stream_idx = i;
                }
            } else if (strstr(last_processed_segment[i], stream_name) != NULL) {
                // Found existing entry for this stream
                stream_idx = i;
                break;
            }
        }
        
        if (stream_idx == -1) {
            // No available slot, use the first one
            stream_idx = 0;
            log_warn("No available slot for last processed segment, using slot 0");
        }
        
        // Check if this segment has already been processed
        if (strcmp(newest_segment, last_processed_segment[stream_idx]) == 0) {
            // Already processed this segment
            pthread_mutex_unlock(&last_processed_mutex);
            return 0;
        }
        
        // Update the last processed segment
        strncpy(last_processed_segment[stream_idx], newest_segment, MAX_PATH_LENGTH - 1);
        last_processed_segment[stream_idx][MAX_PATH_LENGTH - 1] = '\0';
        
        pthread_mutex_unlock(&last_processed_mutex);
        
        // Use a default segment duration
        float segment_duration = 2.0; // Default to 2 seconds
        
        // Detection thread pool has been removed, process segment directly
        log_info("Processing HLS segment directly for stream %s: %s", stream_name, newest_segment);
        
        // Get the stream configuration
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
        
        // Get global config to access models path
        extern config_t g_config;
        
        // Check if model_path is an API URL (starts with http:// or https://)
        char full_model_path[MAX_PATH_LENGTH];
        bool is_api_url = (strncmp(config.detection_model, "http://", 7) == 0 || 
                          strncmp(config.detection_model, "https://", 8) == 0);
        
        if (is_api_url) {
            // For API URLs, use the special "api-detection" string instead of the URL directly
            // This will make the system use the API URL from settings when needed
            log_info("Using API detection for URL: %s", config.detection_model);
            strncpy(full_model_path, "api-detection", MAX_PATH_LENGTH - 1);
            full_model_path[MAX_PATH_LENGTH - 1] = '\0';
        } else if (config.detection_model[0] != '/') {
            // Construct full path using configured models path from INI if it exists
            if (g_config.models_path && strlen(g_config.models_path) > 0) {
                snprintf(full_model_path, MAX_PATH_LENGTH, "%s/%s", 
                        g_config.models_path, config.detection_model);
            } else {
                // Fall back to default path if INI config doesn't exist
                snprintf(full_model_path, MAX_PATH_LENGTH, "/etc/lightnvr/models/%s", 
                        config.detection_model);
            }
        } else {
            // Already an absolute path
            strncpy(full_model_path, config.detection_model, MAX_PATH_LENGTH - 1);
            full_model_path[MAX_PATH_LENGTH - 1] = '\0';
        }
        
        // Get the HLS directory for this stream using the config
        char hls_dir[MAX_PATH_LENGTH];
        
        // Use storage_path_hls if specified, otherwise fall back to storage_path
        const char *base_storage_path = g_config.storage_path;
        if (g_config.storage_path_hls[0] != '\0') {
            base_storage_path = g_config.storage_path_hls;
            log_info("Using dedicated HLS storage path: %s", base_storage_path);
        }
        
        // CRITICAL FIX: Account for the extra "hls" directory in the path
        // Check if the directory exists with the standard path first
        char standard_path[MAX_PATH_LENGTH];
        snprintf(standard_path, MAX_PATH_LENGTH, "%s/hls/%s", base_storage_path, stream_name);
        
        struct stat st_standard;
        if (stat(standard_path, &st_standard) == 0 && S_ISDIR(st_standard.st_mode)) {
            // Standard path exists, use it
            strncpy(hls_dir, standard_path, MAX_PATH_LENGTH - 1);
            hls_dir[MAX_PATH_LENGTH - 1] = '\0';
            log_info("Using standard HLS directory path for segment check: %s", hls_dir);
        } else {
            // Try with the extra "hls" directory
            snprintf(hls_dir, MAX_PATH_LENGTH, "%s/hls/hls/%s", base_storage_path, stream_name);
            log_info("Using alternative HLS directory path with extra 'hls' for segment check: %s", hls_dir);
        }
        
    // Check if the HLS directory has any segments
    DIR *segment_dir = opendir(hls_dir);
    int segment_count = 0;
    
    if (segment_dir) {
        struct dirent *segment_entry;
        while ((segment_entry = readdir(segment_dir)) != NULL) {
            // Count .ts or .m4s files
            if (strstr(segment_entry->d_name, ".ts") || strstr(segment_entry->d_name, ".m4s")) {
                segment_count++;
            }
        }
        closedir(segment_dir);
    }
    
    log_info("Found %d HLS segments in directory: %s", segment_count, hls_dir);
    
    // CRITICAL FIX: Create the HLS directory if it doesn't exist
    // This ensures the detection thread can start even if the directory doesn't exist yet
    if (segment_count == 0) {
        log_warn("No segments found in HLS directory, checking if directory exists");
        
        struct stat st;
        if (stat(hls_dir, &st) != 0) {
            log_warn("HLS directory does not exist, creating it: %s", hls_dir);
            
            // Create the directory with all parent directories
            char cmd[MAX_PATH_LENGTH * 2];
            snprintf(cmd, sizeof(cmd), "mkdir -p %s", hls_dir);
            int result = system(cmd);
            
            if (result != 0) {
                log_error("Failed to create HLS directory: %s (error code: %d)", hls_dir, result);
            } else {
                log_info("Successfully created HLS directory: %s", hls_dir);
            }
        }
    }
        
        // Get the current detection thread status
        extern bool is_stream_detection_thread_running(const char *stream_name);
        bool thread_running = is_stream_detection_thread_running(stream_name);
        
        // Start a detection thread for this stream if not already running
        if (!thread_running) {
            float threshold = config.detection_threshold;
            if (threshold <= 0.0f) {
                threshold = 0.5f; // Default threshold
            }
            
            // Always try to start the detection thread, even if no segments are found yet
            // The thread will periodically check for new segments
            log_info("Starting detection thread for stream %s with model %s (found %d segments)", 
                    stream_name, config.detection_model, segment_count);
            
            // CRITICAL FIX: Force start the detection thread regardless of segment count
            // This ensures the thread is always running and will check for segments periodically
            // The thread will handle its own startup delay and retry logic
            
            // Always create the HLS directory if it doesn't exist
            struct stat st_dir;
            if (stat(hls_dir, &st_dir) != 0) {
                log_warn("HLS directory does not exist, creating it: %s", hls_dir);
                char cmd[MAX_PATH_LENGTH * 2];
                snprintf(cmd, sizeof(cmd), "mkdir -p %s", hls_dir);
                system(cmd);
            }
            
            int result = start_stream_detection_thread(stream_name, full_model_path, threshold, 
                                         config.detection_interval, hls_dir);
            
            if (result == 0) {
                log_info("Successfully started detection thread for stream %s", stream_name);
                
                // Verify the thread is actually running
                if (is_stream_detection_thread_running(stream_name)) {
                    log_info("Confirmed detection thread is running for stream %s", stream_name);
                } else {
                    log_error("Detection thread failed to start for stream %s despite successful return code", 
                             stream_name);
                    
                    // Try one more time with a delay
                    usleep(500000); // 500ms delay
                    result = start_stream_detection_thread(stream_name, full_model_path, threshold, 
                                                         config.detection_interval, hls_dir);
                    
                    if (result == 0 && is_stream_detection_thread_running(stream_name)) {
                        log_info("Successfully started detection thread for stream %s on second attempt", 
                                stream_name);
                    } else {
                        log_error("Failed to start detection thread for stream %s on second attempt", 
                                 stream_name);
                    }
                }
            } else {
                log_error("Failed to start detection thread for stream %s (error code: %d)", 
                         stream_name, result);
            }
        } else {
            log_info("Detection thread is already running for stream %s", stream_name);
        }
        
        // Check if the stream detection thread is running
        if (is_stream_detection_thread_running(stream_name)) {
            // Process the segment using the stream detection thread
            // This will be handled by the check_for_new_segments function in the detection thread
            log_info("Stream detection thread is running for %s, segment will be processed by the thread", stream_name);
        } else {
            log_error("Failed to start detection thread for stream %s", stream_name);
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
