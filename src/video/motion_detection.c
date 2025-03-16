#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <pthread.h>
#include <math.h>
#include <time.h>

#include "core/logger.h"
#include "video/motion_detection.h"
#include "video/streams.h"
#include "video/detection_result.h"

#define MAX_MOTION_STREAMS MAX_STREAMS
#define DEFAULT_SENSITIVITY 0.25f
#define DEFAULT_MIN_MOTION_AREA 0.01f
#define DEFAULT_COOLDOWN_TIME 3
#define MOTION_LABEL "motion"

// Structure to store previous frame data for a stream
typedef struct {
    char stream_name[MAX_STREAM_NAME];
    unsigned char *prev_frame;
    int width;
    int height;
    int channels;
    float sensitivity;
    float min_motion_area;
    int cooldown_time;
    time_t last_detection_time;
    bool enabled;
    pthread_mutex_t mutex;
} motion_stream_t;

// Array to store motion detection state for each stream
static motion_stream_t motion_streams[MAX_MOTION_STREAMS];
static pthread_mutex_t motion_streams_mutex = PTHREAD_MUTEX_INITIALIZER;
static bool initialized = false;

/**
 * Initialize the motion detection system
 */
int init_motion_detection_system(void) {
    if (initialized) {
        return 0;  // Already initialized
    }
    
    pthread_mutex_lock(&motion_streams_mutex);
    
    for (int i = 0; i < MAX_MOTION_STREAMS; i++) {
        memset(&motion_streams[i], 0, sizeof(motion_stream_t));
        pthread_mutex_init(&motion_streams[i].mutex, NULL);
        motion_streams[i].sensitivity = DEFAULT_SENSITIVITY;
        motion_streams[i].min_motion_area = DEFAULT_MIN_MOTION_AREA;
        motion_streams[i].cooldown_time = DEFAULT_COOLDOWN_TIME;
        motion_streams[i].enabled = false;
    }
    
    initialized = true;
    pthread_mutex_unlock(&motion_streams_mutex);
    
    log_info("Motion detection system initialized");
    return 0;
}

/**
 * Shutdown the motion detection system
 */
void shutdown_motion_detection_system(void) {
    if (!initialized) {
        return;
    }
    
    pthread_mutex_lock(&motion_streams_mutex);
    
    for (int i = 0; i < MAX_MOTION_STREAMS; i++) {
        pthread_mutex_lock(&motion_streams[i].mutex);
        
        if (motion_streams[i].prev_frame) {
            free(motion_streams[i].prev_frame);
            motion_streams[i].prev_frame = NULL;
        }
        
        pthread_mutex_unlock(&motion_streams[i].mutex);
        pthread_mutex_destroy(&motion_streams[i].mutex);
    }
    
    initialized = false;
    pthread_mutex_unlock(&motion_streams_mutex);
    
    log_info("Motion detection system shutdown");
}

/**
 * Find or create a motion stream entry
 */
static motion_stream_t *get_motion_stream(const char *stream_name) {
    if (!stream_name || !initialized) {
        return NULL;
    }
    
    pthread_mutex_lock(&motion_streams_mutex);
    
    // Find existing entry
    for (int i = 0; i < MAX_MOTION_STREAMS; i++) {
        if (motion_streams[i].stream_name[0] != '\0' && 
            strcmp(motion_streams[i].stream_name, stream_name) == 0) {
            pthread_mutex_unlock(&motion_streams_mutex);
            return &motion_streams[i];
        }
    }
    
    // Create new entry
    for (int i = 0; i < MAX_MOTION_STREAMS; i++) {
        if (motion_streams[i].stream_name[0] == '\0') {
            strncpy(motion_streams[i].stream_name, stream_name, MAX_STREAM_NAME - 1);
            motion_streams[i].stream_name[MAX_STREAM_NAME - 1] = '\0';
            pthread_mutex_unlock(&motion_streams_mutex);
            return &motion_streams[i];
        }
    }
    
    pthread_mutex_unlock(&motion_streams_mutex);
    log_error("No available slots for motion detection stream: %s", stream_name);
    return NULL;
}

/**
 * Configure motion detection for a stream
 */
int configure_motion_detection(const char *stream_name, float sensitivity, 
                              float min_motion_area, int cooldown_time) {
    if (!stream_name) {
        log_error("Invalid stream name for configure_motion_detection");
        return -1;
    }
    
    motion_stream_t *stream = get_motion_stream(stream_name);
    if (!stream) {
        log_error("Failed to get motion stream for %s", stream_name);
        return -1;
    }
    
    pthread_mutex_lock(&stream->mutex);
    
    // Validate and set parameters
    stream->sensitivity = (sensitivity > 0.0f && sensitivity <= 1.0f) ? 
                          sensitivity : DEFAULT_SENSITIVITY;
    
    stream->min_motion_area = (min_motion_area > 0.0f && min_motion_area <= 1.0f) ? 
                             min_motion_area : DEFAULT_MIN_MOTION_AREA;
    
    stream->cooldown_time = (cooldown_time > 0) ? cooldown_time : DEFAULT_COOLDOWN_TIME;
    
    pthread_mutex_unlock(&stream->mutex);
    
    log_info("Configured motion detection for stream %s: sensitivity=%.2f, min_area=%.2f, cooldown=%d",
             stream_name, stream->sensitivity, stream->min_motion_area, stream->cooldown_time);
    
    return 0;
}

/**
 * Enable or disable motion detection for a stream
 */
int set_motion_detection_enabled(const char *stream_name, bool enabled) {
    if (!stream_name) {
        log_error("Invalid stream name for set_motion_detection_enabled");
        return -1;
    }
    
    motion_stream_t *stream = get_motion_stream(stream_name);
    if (!stream) {
        log_error("Failed to get motion stream for %s", stream_name);
        return -1;
    }
    
    pthread_mutex_lock(&stream->mutex);
    
    // If disabling, free the previous frame
    if (!enabled && stream->enabled && stream->prev_frame) {
        free(stream->prev_frame);
        stream->prev_frame = NULL;
        stream->width = 0;
        stream->height = 0;
        stream->channels = 0;
    }
    
    stream->enabled = enabled;
    
    pthread_mutex_unlock(&stream->mutex);
    
    log_info("Motion detection %s for stream %s", enabled ? "enabled" : "disabled", stream_name);
    
    return 0;
}

/**
 * Check if motion detection is enabled for a stream
 */
bool is_motion_detection_enabled(const char *stream_name) {
    if (!stream_name) {
        return false;
    }
    
    motion_stream_t *stream = get_motion_stream(stream_name);
    if (!stream) {
        return false;
    }
    
    pthread_mutex_lock(&stream->mutex);
    bool enabled = stream->enabled;
    pthread_mutex_unlock(&stream->mutex);
    
    return enabled;
}

/**
 * Convert RGB frame to grayscale
 */
static unsigned char *rgb_to_grayscale(const unsigned char *rgb_data, int width, int height) {
    unsigned char *gray_data = (unsigned char *)malloc(width * height);
    if (!gray_data) {
        log_error("Failed to allocate memory for grayscale conversion");
        return NULL;
    }
    
    for (int y = 0; y < height; y++) {
        for (int x = 0; x < width; x++) {
            int rgb_idx = (y * width + x) * 3;
            int gray_idx = y * width + x;
            
            // Convert RGB to grayscale using standard luminance formula
            gray_data[gray_idx] = (unsigned char)(
                0.299f * rgb_data[rgb_idx] +      // R
                0.587f * rgb_data[rgb_idx + 1] +  // G
                0.114f * rgb_data[rgb_idx + 2]    // B
            );
        }
    }
    
    return gray_data;
}

/**
 * Detect motion between two frames
 */
static bool detect_motion_between_frames(const unsigned char *curr_frame, const unsigned char *prev_frame,
                                        int width, int height, float sensitivity, float min_motion_area,
                                        float *motion_score, float *motion_area) {
    if (!curr_frame || !prev_frame) {
        return false;
    }
    
    int pixel_count = width * height;
    int changed_pixels = 0;
    int total_diff = 0;
    
    // Calculate pixel differences
    for (int i = 0; i < pixel_count; i++) {
        int diff = abs((int)curr_frame[i] - (int)prev_frame[i]);
        
        // Count pixels that changed more than the sensitivity threshold
        if (diff > (sensitivity * 255.0f)) {
            changed_pixels++;
            total_diff += diff;
        }
    }
    
    // Calculate motion metrics
    *motion_area = (float)changed_pixels / (float)pixel_count;
    *motion_score = (float)total_diff / (float)(pixel_count * 255);
    
    // Determine if motion is detected based on area threshold
    return (*motion_area >= min_motion_area);
}

/**
 * Process a frame for motion detection
 */
int detect_motion(const char *stream_name, const unsigned char *frame_data,
                 int width, int height, int channels, time_t frame_time,
                 detection_result_t *result) {
    if (!stream_name || !frame_data || !result || width <= 0 || height <= 0 || channels <= 0) {
        log_error("Invalid parameters for detect_motion");
        return -1;
    }
    
    // Initialize result
    memset(result, 0, sizeof(detection_result_t));
    
    // Get motion stream
    motion_stream_t *stream = get_motion_stream(stream_name);
    if (!stream) {
        log_error("Failed to get motion stream for %s", stream_name);
        return -1;
    }
    
    pthread_mutex_lock(&stream->mutex);
    
    // Check if motion detection is enabled
    if (!stream->enabled) {
        pthread_mutex_unlock(&stream->mutex);
        return 0;
    }
    
    // Check cooldown period
    if (stream->last_detection_time > 0 && 
        (frame_time - stream->last_detection_time) < stream->cooldown_time) {
        pthread_mutex_unlock(&stream->mutex);
        return 0;
    }
    
    // Convert to grayscale if needed
    unsigned char *gray_frame = NULL;
    const unsigned char *frame_to_use = frame_data;
    
    if (channels == 3) {
        gray_frame = rgb_to_grayscale(frame_data, width, height);
        if (!gray_frame) {
            pthread_mutex_unlock(&stream->mutex);
            return -1;
        }
        frame_to_use = gray_frame;
    }
    
    bool motion_detected = false;
    float motion_score = 0.0f;
    float motion_area = 0.0f;
    
    // Check if we have a previous frame to compare with
    if (stream->prev_frame && stream->width == width && stream->height == height) {
        // Detect motion between frames
        motion_detected = detect_motion_between_frames(
            frame_to_use, stream->prev_frame, width, height,
            stream->sensitivity, stream->min_motion_area,
            &motion_score, &motion_area
        );
        
        if (motion_detected) {
            // Update last detection time
            stream->last_detection_time = frame_time;
            
            // Fill detection result
            result->count = 1;
            strncpy(result->detections[0].label, MOTION_LABEL, MAX_LABEL_LENGTH - 1);
            result->detections[0].confidence = motion_score;
            
            // Set bounding box to cover the entire frame for now
            // In a more advanced implementation, we could identify the specific motion regions
            result->detections[0].x = 0.0f;
            result->detections[0].y = 0.0f;
            result->detections[0].width = 1.0f;
            result->detections[0].height = 1.0f;
            
            log_info("Motion detected in stream %s: score=%.2f, area=%.2f%%", 
                    stream_name, motion_score, motion_area * 100.0f);
        }
    }
    
    // Update or allocate previous frame
    if (!stream->prev_frame || stream->width != width || stream->height != height) {
        // Free old frame if it exists
        if (stream->prev_frame) {
            free(stream->prev_frame);
        }
        
        // Allocate new frame buffer
        stream->prev_frame = (unsigned char *)malloc(width * height);
        if (!stream->prev_frame) {
            log_error("Failed to allocate memory for previous frame");
            if (gray_frame) {
                free(gray_frame);
            }
            pthread_mutex_unlock(&stream->mutex);
            return -1;
        }
        
        stream->width = width;
        stream->height = height;
        stream->channels = 1; // We always store grayscale
    }
    
    // Copy current frame to previous frame buffer
    memcpy(stream->prev_frame, frame_to_use, width * height);
    
    // Free grayscale conversion if we created one
    if (gray_frame) {
        free(gray_frame);
    }
    
    pthread_mutex_unlock(&stream->mutex);
    
    return 0;
}
