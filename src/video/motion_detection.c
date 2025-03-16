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
#define DEFAULT_SENSITIVITY 0.15f        // Lower sensitivity threshold (was 0.25)
#define DEFAULT_MIN_MOTION_AREA 0.005f   // Lower min area (was 0.01)
#define DEFAULT_COOLDOWN_TIME 3
#define DEFAULT_MOTION_HISTORY 3         // Number of frames to keep for temporal filtering
#define DEFAULT_BLUR_RADIUS 1            // Radius for simple box blur
#define DEFAULT_NOISE_THRESHOLD 10       // Noise filtering threshold
#define DEFAULT_USE_GRID_DETECTION true  // Use grid-based detection
#define DEFAULT_GRID_SIZE 8              // 8x8 grid for detection
#define MOTION_LABEL "motion"

// Structure to store frame data for temporal filtering
typedef struct {
    unsigned char *frame;
    time_t timestamp;
} frame_history_t;

// Structure to store previous frame data for a stream
typedef struct {
    char stream_name[MAX_STREAM_NAME];
    unsigned char *prev_frame;           // Previous grayscale frame
    unsigned char *blur_buffer;          // Buffer for blur operations
    unsigned char *background;           // Background model
    frame_history_t *frame_history;      // Circular buffer for frame history
    int history_size;                    // Size of frame history buffer
    int history_index;                   // Current index in history buffer
    float *grid_scores;                  // Array to store grid cell motion scores
    int width;
    int height;
    int channels;
    float sensitivity;                   // Sensitivity threshold
    float min_motion_area;               // Minimum area to trigger detection
    int cooldown_time;                   // Time between detections
    int blur_radius;                     // Blur radius for noise reduction
    int noise_threshold;                 // Threshold for noise filtering
    bool use_grid_detection;             // Whether to use grid-based detection
    int grid_size;                       // Size of detection grid (grid_size x grid_size)
    time_t last_detection_time;
    bool enabled;
    pthread_mutex_t mutex;
} motion_stream_t;

// Array to store motion detection state for each stream
static motion_stream_t motion_streams[MAX_MOTION_STREAMS];
static pthread_mutex_t motion_streams_mutex = PTHREAD_MUTEX_INITIALIZER;
static bool initialized = false;

// Forward declarations for helper functions
static void apply_box_blur(unsigned char *src, unsigned char *dst, int width, int height, int radius);
static void update_background_model(unsigned char *background, const unsigned char *current,
                                    int width, int height, float learning_rate);
static float calculate_grid_motion(const unsigned char *curr_frame, const unsigned char *prev_frame,
                                  const unsigned char *background, int width, int height,
                                  float sensitivity, int noise_threshold, int grid_size,
                                  float *grid_scores, float *motion_area);

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
        motion_streams[i].history_size = DEFAULT_MOTION_HISTORY;
        motion_streams[i].blur_radius = DEFAULT_BLUR_RADIUS;
        motion_streams[i].noise_threshold = DEFAULT_NOISE_THRESHOLD;
        motion_streams[i].use_grid_detection = DEFAULT_USE_GRID_DETECTION;
        motion_streams[i].grid_size = DEFAULT_GRID_SIZE;
        motion_streams[i].enabled = false;
    }

    initialized = true;
    pthread_mutex_unlock(&motion_streams_mutex);

    log_info("Motion detection system initialized with improved parameters");
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

        if (motion_streams[i].blur_buffer) {
            free(motion_streams[i].blur_buffer);
            motion_streams[i].blur_buffer = NULL;
        }

        if (motion_streams[i].background) {
            free(motion_streams[i].background);
            motion_streams[i].background = NULL;
        }

        if (motion_streams[i].grid_scores) {
            free(motion_streams[i].grid_scores);
            motion_streams[i].grid_scores = NULL;
        }

        if (motion_streams[i].frame_history) {
            for (int j = 0; j < motion_streams[i].history_size; j++) {
                if (motion_streams[i].frame_history[j].frame) {
                    free(motion_streams[i].frame_history[j].frame);
                }
            }
            free(motion_streams[i].frame_history);
            motion_streams[i].frame_history = NULL;
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
 * Configure advanced motion detection parameters
 */
int configure_advanced_motion_detection(const char *stream_name, int blur_radius,
                                       int noise_threshold, bool use_grid_detection,
                                       int grid_size, int history_size) {
    if (!stream_name) {
        log_error("Invalid stream name for configure_advanced_motion_detection");
        return -1;
    }

    motion_stream_t *stream = get_motion_stream(stream_name);
    if (!stream) {
        log_error("Failed to get motion stream for %s", stream_name);
        return -1;
    }

    pthread_mutex_lock(&stream->mutex);

    // Store old values to check if reallocation is needed
    int old_grid_size = stream->grid_size;
    int old_history_size = stream->history_size;

    // Validate and set parameters
    stream->blur_radius = (blur_radius >= 0 && blur_radius <= 5) ?
                           blur_radius : DEFAULT_BLUR_RADIUS;

    stream->noise_threshold = (noise_threshold >= 0 && noise_threshold <= 50) ?
                              noise_threshold : DEFAULT_NOISE_THRESHOLD;

    stream->use_grid_detection = use_grid_detection;

    stream->grid_size = (grid_size >= 2 && grid_size <= 32) ?
                         grid_size : DEFAULT_GRID_SIZE;

    // Validate history size
    int new_history_size = (history_size > 0 && history_size <= 10) ?
                           history_size : DEFAULT_MOTION_HISTORY;

    // Reallocate grid_scores if grid size changed
    if (stream->grid_size != old_grid_size && stream->grid_scores) {
        free(stream->grid_scores);
        stream->grid_scores = NULL;
    }

    // Reset frame history if size changed
    if (new_history_size != old_history_size && stream->frame_history) {
        for (int i = 0; i < old_history_size; i++) {
            if (stream->frame_history[i].frame) {
                free(stream->frame_history[i].frame);
            }
        }
        free(stream->frame_history);
        stream->frame_history = NULL;
    }

    stream->history_size = new_history_size;

    pthread_mutex_unlock(&stream->mutex);

    log_info("Configured advanced motion detection for stream %s: blur=%d, noise=%d, grid=%s, grid_size=%d, history=%d",
             stream_name, stream->blur_radius, stream->noise_threshold,
             stream->use_grid_detection ? "true" : "false", stream->grid_size, stream->history_size);

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

    // If disabling, free resources
    if (!enabled && stream->enabled) {
        if (stream->prev_frame) {
            free(stream->prev_frame);
            stream->prev_frame = NULL;
        }

        if (stream->blur_buffer) {
            free(stream->blur_buffer);
            stream->blur_buffer = NULL;
        }

        if (stream->background) {
            free(stream->background);
            stream->background = NULL;
        }

        if (stream->grid_scores) {
            free(stream->grid_scores);
            stream->grid_scores = NULL;
        }

        if (stream->frame_history) {
            for (int i = 0; i < stream->history_size; i++) {
                if (stream->frame_history[i].frame) {
                    free(stream->frame_history[i].frame);
                }
            }
            free(stream->frame_history);
            stream->frame_history = NULL;
        }

        stream->width = 0;
        stream->height = 0;
        stream->channels = 0;
        stream->history_index = 0;
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
 * Apply a simple box blur to reduce noise
 */
static void apply_box_blur(unsigned char *src, unsigned char *dst, int width, int height, int radius) {
    // Skip if radius is 0
    if (radius <= 0) {
        memcpy(dst, src, width * height);
        return;
    }

    // Simple box blur implementation
    for (int y = 0; y < height; y++) {
        for (int x = 0; x < width; x++) {
            int sum = 0;
            int count = 0;

            // Calculate average of pixels in the radius
            for (int dy = -radius; dy <= radius; dy++) {
                for (int dx = -radius; dx <= radius; dx++) {
                    int nx = x + dx;
                    int ny = y + dy;

                    // Skip out of bounds pixels
                    if (nx < 0 || nx >= width || ny < 0 || ny >= height) {
                        continue;
                    }

                    sum += src[ny * width + nx];
                    count++;
                }
            }

            // Set pixel to average value
            dst[y * width + x] = (unsigned char)(sum / count);
        }
    }
}

/**
 * Update the background model using running average
 */
static void update_background_model(unsigned char *background, const unsigned char *current,
                                    int width, int height, float learning_rate) {
    if (!background || !current) {
        return;
    }

    // Update background model using exponential moving average
    for (int i = 0; i < width * height; i++) {
        // background = (1-alpha) * background + alpha * current
        background[i] = (unsigned char)((1.0f - learning_rate) * background[i] +
                                       learning_rate * current[i]);
    }
}

/**
 * Calculate motion using grid-based approach
 */
static float calculate_grid_motion(const unsigned char *curr_frame, const unsigned char *prev_frame,
                                  const unsigned char *background, int width, int height,
                                  float sensitivity, int noise_threshold, int grid_size,
                                  float *grid_scores, float *motion_area) {
    if (!curr_frame || !prev_frame || !background || !grid_scores || !motion_area) {
        return 0.0f;
    }

    int cell_width = width / grid_size;
    int cell_height = height / grid_size;
    int total_cells = grid_size * grid_size;
    int cells_with_motion = 0;
    float max_cell_score = 0.0f;

    // Calculate motion for each grid cell
    for (int gy = 0; gy < grid_size; gy++) {
        for (int gx = 0; gx < grid_size; gx++) {
            int cell_start_x = gx * cell_width;
            int cell_start_y = gy * cell_height;
            int cell_end_x = (gx + 1) * cell_width;
            int cell_end_y = (gy + 1) * cell_height;

            // Ensure we don't go out of bounds
            if (cell_end_x > width) cell_end_x = width;
            if (cell_end_y > height) cell_end_y = height;

            int cell_pixels = 0;
            int changed_pixels = 0;
            int total_diff = 0;

            // Process each pixel in the cell
            for (int y = cell_start_y; y < cell_end_y; y++) {
                for (int x = cell_start_x; x < cell_end_x; x++) {
                    int idx = y * width + x;

                    // Calculate differences from previous frame and background
                    int frame_diff = abs((int)curr_frame[idx] - (int)prev_frame[idx]);
                    int bg_diff = abs((int)curr_frame[idx] - (int)background[idx]);

                    // Use the larger of the two differences
                    int diff = (frame_diff > bg_diff) ? frame_diff : bg_diff;

                    // Apply noise threshold
                    if (diff > noise_threshold) {
                        // Pixel difference exceeds sensitivity threshold
                        if (diff > (sensitivity * 255.0f)) {
                            changed_pixels++;
                            total_diff += diff;
                        }
                    }

                    cell_pixels++;
                }
            }

            // Calculate cell motion score
            float cell_area = (float)changed_pixels / (float)cell_pixels;
            float cell_score = (float)total_diff / (float)(cell_pixels * 255);

            // Store cell score
            int cell_idx = gy * grid_size + gx;
            grid_scores[cell_idx] = cell_score;

            // Track overall motion
            if (cell_score > 0.01f) {  // Cell has meaningful motion
                cells_with_motion++;
                if (cell_score > max_cell_score) {
                    max_cell_score = cell_score;
                }
            }
        }
    }

    // Calculate overall motion metrics
    *motion_area = (float)cells_with_motion / (float)total_cells;

    // Return the maximum cell score as the overall motion score
    // This focuses on the most active area rather than averaging motion across the frame
    return max_cell_score;
}

/**
 * Add frame to history buffer
 */
static void add_frame_to_history(motion_stream_t *stream, const unsigned char *frame, time_t timestamp) {
    if (!stream || !frame || !stream->frame_history) {
        return;
    }

    // Free the old frame if it exists
    if (stream->frame_history[stream->history_index].frame) {
        free(stream->frame_history[stream->history_index].frame);
    }

    // Allocate and copy new frame
    stream->frame_history[stream->history_index].frame = (unsigned char *)malloc(stream->width * stream->height);
    if (!stream->frame_history[stream->history_index].frame) {
        log_error("Failed to allocate memory for frame history");
        return;
    }

    memcpy(stream->frame_history[stream->history_index].frame, frame, stream->width * stream->height);
    stream->frame_history[stream->history_index].timestamp = timestamp;

    // Update index
    stream->history_index = (stream->history_index + 1) % stream->history_size;
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
    if (channels == 3) {
        gray_frame = rgb_to_grayscale(frame_data, width, height);
        if (!gray_frame) {
            pthread_mutex_unlock(&stream->mutex);
            return -1;
        }
    } else if (channels == 1) {
        // If input is already grayscale, just make a copy
        gray_frame = (unsigned char *)malloc(width * height);
        if (!gray_frame) {
            log_error("Failed to allocate memory for gray frame");
            pthread_mutex_unlock(&stream->mutex);
            return -1;
        }
        memcpy(gray_frame, frame_data, width * height);
    } else {
        log_error("Unsupported number of channels: %d", channels);
        pthread_mutex_unlock(&stream->mutex);
        return -1;
    }

    // Check if we need to allocate or reallocate resources
    if (!stream->prev_frame || stream->width != width || stream->height != height) {
        // Free old resources if they exist
        if (stream->prev_frame) {
            free(stream->prev_frame);
            stream->prev_frame = NULL;
        }

        if (stream->blur_buffer) {
            free(stream->blur_buffer);
            stream->blur_buffer = NULL;
        }

        if (stream->background) {
            free(stream->background);
            stream->background = NULL;
        }

        if (stream->grid_scores) {
            free(stream->grid_scores);
            stream->grid_scores = NULL;
        }

        if (stream->frame_history) {
            for (int i = 0; i < stream->history_size; i++) {
                if (stream->frame_history[i].frame) {
                    free(stream->frame_history[i].frame);
                }
            }
            free(stream->frame_history);
            stream->frame_history = NULL;
        }

        // Allocate new resources
        stream->prev_frame = (unsigned char *)malloc(width * height);
        stream->blur_buffer = (unsigned char *)malloc(width * height);
        stream->background = (unsigned char *)malloc(width * height);

        if (!stream->prev_frame || !stream->blur_buffer || !stream->background) {
            log_error("Failed to allocate memory for motion detection buffers");

            if (stream->prev_frame) {
                free(stream->prev_frame);
                stream->prev_frame = NULL;
            }

            if (stream->blur_buffer) {
                free(stream->blur_buffer);
                stream->blur_buffer = NULL;
            }

            if (stream->background) {
                free(stream->background);
                stream->background = NULL;
            }

            free(gray_frame);
            pthread_mutex_unlock(&stream->mutex);
            return -1;
        }

        // Initialize the background with the current frame
        memcpy(stream->background, gray_frame, width * height);
        memcpy(stream->prev_frame, gray_frame, width * height);

        // Allocate grid scores array
        if (stream->use_grid_detection) {
            stream->grid_scores = (float *)malloc(stream->grid_size * stream->grid_size * sizeof(float));
            if (!stream->grid_scores) {
                log_error("Failed to allocate memory for grid scores");
                free(gray_frame);
                pthread_mutex_unlock(&stream->mutex);
                return -1;
            }
            memset(stream->grid_scores, 0, stream->grid_size * stream->grid_size * sizeof(float));
        }

        // Allocate frame history buffer
        stream->frame_history = (frame_history_t *)malloc(stream->history_size * sizeof(frame_history_t));
        if (!stream->frame_history) {
            log_error("Failed to allocate memory for frame history");
            free(gray_frame);
            pthread_mutex_unlock(&stream->mutex);
            return -1;
        }
        memset(stream->frame_history, 0, stream->history_size * sizeof(frame_history_t));
        stream->history_index = 0;

        // Update dimensions
        stream->width = width;
        stream->height = height;
        stream->channels = 1;  // We always store grayscale

        free(gray_frame);
        pthread_mutex_unlock(&stream->mutex);
        return 0;  // Skip motion detection on first frame
    }

    // Apply blur to reduce noise
    apply_box_blur(gray_frame, stream->blur_buffer, width, height, stream->blur_radius);

    bool motion_detected = false;
    float motion_score = 0.0f;
    float motion_area = 0.0f;

    // Detect motion between frames
    if (stream->use_grid_detection) {
        // Grid-based motion detection
        motion_score = calculate_grid_motion(
            stream->blur_buffer, stream->prev_frame, stream->background,
            width, height, stream->sensitivity, stream->noise_threshold,
            stream->grid_size, stream->grid_scores, &motion_area
        );

        // Determine if motion is detected based on area threshold
        motion_detected = (motion_area >= stream->min_motion_area) && (motion_score > 0.01f);
    } else {
        // Simple frame differencing (original approach with improvements)
        int pixel_count = width * height;
        int changed_pixels = 0;
        int total_diff = 0;

        for (int i = 0; i < pixel_count; i++) {
            // Calculate differences from previous frame and background
            int frame_diff = abs((int)stream->blur_buffer[i] - (int)stream->prev_frame[i]);
            int bg_diff = abs((int)stream->blur_buffer[i] - (int)stream->background[i]);

            // Use the larger of the two differences
            int diff = (frame_diff > bg_diff) ? frame_diff : bg_diff;

            // Apply noise threshold
            if (diff > stream->noise_threshold) {
                // Count pixels that changed more than the sensitivity threshold
                if (diff > (stream->sensitivity * 255.0f)) {
                    changed_pixels++;
                    total_diff += diff;
                }
            }
        }

        // Calculate motion metrics
        motion_area = (float)changed_pixels / (float)pixel_count;
        motion_score = (float)total_diff / (float)(pixel_count * 255);

        // Determine if motion is detected based on area threshold
        motion_detected = (motion_area >= stream->min_motion_area);
    }

    // Add current frame to history
    add_frame_to_history(stream, stream->blur_buffer, frame_time);

    // Update background model with a slow learning rate
    // Use a faster learning rate (0.05) when no motion is detected, slower (0.01) when motion is detected
    float learning_rate = motion_detected ? 0.01f : 0.05f;
    update_background_model(stream->background, stream->blur_buffer, width, height, learning_rate);

    // Copy current blurred frame to previous frame buffer for next comparison
    memcpy(stream->prev_frame, stream->blur_buffer, width * height);

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

        log_info("Motion detected in stream %s: score=%.3f, area=%.2f%%, confidence=%.2f",
                stream_name, motion_score, motion_area * 100.0f, result->detections[0].confidence);
    } else {
        // Log low motion details for debugging (at debug level)
        log_debug("No motion in stream %s: score=%.3f, area=%.2f%%, threshold=%.2f",
                 stream_name, motion_score, motion_area * 100.0f, stream->min_motion_area);
    }

    // Clean up
    free(gray_frame);
    pthread_mutex_unlock(&stream->mutex);

    return 0;
}
