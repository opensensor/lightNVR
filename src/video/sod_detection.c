#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <dlfcn.h>
#include <pthread.h>
#include <unistd.h>
#include <sys/stat.h>

// FFmpeg headers
#include <libavcodec/avcodec.h>
#include <libavformat/avformat.h>
#include <libavutil/imgutils.h>
#include <libavutil/opt.h>
#include <libswscale/swscale.h>

#include "core/logger.h"
#include "core/config.h"  // For MAX_PATH_LENGTH
#include "video/detection_result.h"
#include "video/detection_model.h"
#include "video/sod_detection.h"
#include "sod/sod.h"

// SOD library function pointers for dynamic loading
typedef struct {
    void *handle;
    int (*sod_cnn_create)(void **ppOut, const char *zArch, const char *zModelPath, const char **pzErr);
    int (*sod_cnn_config)(void *pNet, int conf, ...);
    int (*sod_cnn_predict)(void *pNet, float *pInput, sod_box **paBox, int *pnBox);
    void (*sod_cnn_destroy)(void *pNet);
    float * (*sod_cnn_prepare_image)(void *pNet, void *in);
    int (*sod_cnn_get_network_size)(void *pNet, int *pWidth, int *pHeight, int *pChannels);
    void * (*sod_make_image)(int w, int h, int c);
    void (*sod_free_image)(void *m);
    sod_img * (*sod_img_load_from_mem)(const unsigned char *zBuf, int buf_len, int nChannels);
    sod_img * (*sod_img_load_from_file)(const char *zFile, int nChannels);
} sod_functions_t;

// Global SOD functions
static sod_functions_t sod_funcs = {0};
static bool sod_available = false;

// SOD model structure
typedef struct {
    void *model;                 // SOD model handle
    float threshold;             // Detection threshold
} sod_model_t;

// SOD box structure (for dynamic loading)
typedef struct {
    int x;
    int y;
    int w;
    int h;
    float score;
    const char *zName;
    void *pUserData;
} sod_box_dynamic;

// Generic model structure
typedef struct {
    char type[16];               // Model type (sod)
    sod_model_t sod;             // SOD model
    char path[MAX_PATH_LENGTH];  // Path to the model file (for reference)
} model_t;

/**
 * Initialize the SOD detection system
 * Handles both static and dynamic linking cases
 */
int init_sod_detection_system(void) {
#ifdef SOD_ENABLED
        // Static linking approach - SOD functions are directly available
        log_info("SOD detection initialized with static linking");
        sod_available = true;
    return 0;
#else
    log_error("SOD support is not enabled at compile time");
    return -1;
#endif
}

/**
 * Shutdown the SOD detection system
 */
void shutdown_sod_detection_system(void) {
    // Set a flag to indicate that the system is shutting down
    // This will be checked by any ongoing operations to abort early
    sod_available = false;

    // Add a small delay to allow any in-progress operations to detect the flag change
    usleep(100000); // 100ms

    log_info("SOD detection system shutdown");
}

// Track cleaned up models to prevent double-free issues
#define MAX_CLEANED_MODELS 32
static void *cleaned_models[MAX_CLEANED_MODELS] = {NULL};
static int cleaned_models_count = 0;
static pthread_mutex_t cleaned_models_mutex = PTHREAD_MUTEX_INITIALIZER;

/**
 * Check if a model has already been cleaned up
 */
static bool is_model_already_cleaned(void *model_ptr) {
    bool result = false;

    pthread_mutex_lock(&cleaned_models_mutex);
    for (int i = 0; i < cleaned_models_count; i++) {
        if (cleaned_models[i] == model_ptr) {
            result = true;
            break;
        }
    }
    pthread_mutex_unlock(&cleaned_models_mutex);

    return result;
}

/**
 * Add a model to the cleaned models list
 */
static void add_to_cleaned_models(void *model_ptr) {
    pthread_mutex_lock(&cleaned_models_mutex);
    if (cleaned_models_count < MAX_CLEANED_MODELS) {
        cleaned_models[cleaned_models_count++] = model_ptr;
    } else {
        // If the array is full, shift everything down and add to the end
        for (int i = 0; i < MAX_CLEANED_MODELS - 1; i++) {
            cleaned_models[i] = cleaned_models[i + 1];
        }
        cleaned_models[MAX_CLEANED_MODELS - 1] = model_ptr;
    }
    pthread_mutex_unlock(&cleaned_models_mutex);
}

/**
 * Safely clean up a SOD model
 * This function ensures proper cleanup of SOD model resources
 *
 * @param model The SOD model to clean up
 */
void cleanup_sod_model(detection_model_t model) {
    if (!model) {
        log_warn("Attempted to clean up NULL SOD model");
        return;
    }

    model_t *m = (model_t *)model;
    if (strcmp(m->type, MODEL_TYPE_SOD) != 0) {
        log_warn("Attempted to clean up non-SOD model: %s", m->type);
        return;
    }

    log_info("Cleaning up SOD model: %s", m->path);

    // Clean up the SOD model - use a local variable to avoid double-free issues
    void *sod_model_ptr = m->sod.model;

    // Set the model pointer to NULL first to prevent double-free
    m->sod.model = NULL;

    // Only destroy the model if the pointer is valid
    if (sod_model_ptr) {
        if (is_model_already_cleaned(sod_model_ptr)) {
            log_warn("Skipping cleanup of already cleaned SOD model: %p", sod_model_ptr);
        } else {
            log_info("Destroying SOD model: %p", sod_model_ptr);
            // Add to cleaned models list BEFORE destroying to prevent race conditions
            add_to_cleaned_models(sod_model_ptr);

            // Use a safer approach to destroy the model
            // This prevents accessing memory after it's been freed
            // which is what was causing the Valgrind errors
            void *temp = sod_model_ptr;
            sod_model_ptr = NULL;
            sod_cnn_destroy(temp);
        }
    }

    // Free the model structure
    free(m);

    log_info("SOD model cleanup complete");
}

/**
 * Check if SOD is available
 */
bool is_sod_available(void) {
    return sod_available;
}

/**
 * Load a SOD model
 */
detection_model_t load_sod_model(const char *model_path, float threshold) {
    if (!sod_available) {
        log_error("SOD library not available");
        return NULL;
    }

    // Load the model using SOD API
    void *sod_model = NULL;
    const char *err_msg = NULL;

    // Create CNN model
    int rc;

    // Check if this is a face detection model based on filename or path
    const char *arch = "default";

    // Extract the filename from the path
    const char *filename = strrchr(model_path, '/');
    if (filename) {
        filename++; // Skip the '/'
    } else {
        filename = model_path; // No '/' in the path
    }

    // First check for exact filename matches
    if (strcmp(filename, "face_cnn.sod") == 0 ||
        strcmp(filename, "face.sod") == 0 ||
        strcmp(filename, "face_detection.sod") == 0) {
        arch = ":face";
        log_info("Detected face model by exact filename match, using :face architecture: %s", filename);
    }
    else if (strcmp(filename, "tiny20.sod") == 0 ||
             strcmp(filename, "voc.sod") == 0 ||
             strcmp(filename, "voc_detection.sod") == 0) {
        arch = ":voc";
        log_info("Detected VOC model by exact filename match, using :voc architecture: %s", filename);
    }
    else {
        // If we couldn't determine the architecture, default to face for .sod files
        // This is a fallback to ensure face detection works even if the filename doesn't contain "face"
        log_info("Could not determine model architecture from name, defaulting to :face for: %s", model_path);
        arch = ":face";
    }

    // Use static linking
    sod_cnn *cnn_model = NULL;
    rc = sod_cnn_create(&cnn_model, arch, model_path, &err_msg);

    if (rc != 0 || !cnn_model) {  // SOD_OK is 0
        log_error("Failed to load SOD model: %s - %s", model_path, err_msg ? err_msg : "Unknown error");
        return NULL;
    }

    // Store the model pointer
    sod_model = cnn_model;

    // Set detection threshold - use same threshold as spec if not specified
    if (threshold <= 0.0f) {
        threshold = 0.3f; // Default threshold from spec
        log_info("Using default threshold of 0.3 for model %s", model_path);
    }

    // Use static linking
    sod_cnn_config(cnn_model, SOD_CNN_DETECTION_THRESHOLD, threshold);

    // Create model structure
    model_t *model = (model_t *)malloc(sizeof(model_t));
    if (!model) {
        log_error("Failed to allocate memory for model structure");
        sod_cnn_destroy(sod_model);
        return NULL;
    }

    // Initialize model structure
    strncpy(model->type, MODEL_TYPE_SOD, sizeof(model->type) - 1);
    model->sod.model = sod_model;
    model->sod.threshold = threshold;

    // Store the model path in the model structure
    strncpy(model->path, model_path, MAX_PATH_LENGTH - 1);
    model->path[MAX_PATH_LENGTH - 1] = '\0';  // Ensure null termination

    log_info("SOD model loaded: %s with threshold %.2f", model_path, threshold);
    return model;
}

/**
 * Run detection on a frame using SOD
 */
int detect_with_sod_model(detection_model_t model, const unsigned char *frame_data,
    int width, int height, int channels, detection_result_t *result) {
    if (!model || !frame_data || !result) {
        log_error("Invalid parameters for detect_with_sod_model: model=%p, frame_data=%p, result=%p",
                 model, frame_data, result);
        return -1;
    }

    // Validate dimensions
    if (width <= 0 || height <= 0 || channels <= 0 || channels > 4) {
        log_error("Invalid dimensions for detect_with_sod_model: width=%d, height=%d, channels=%d",
                 width, height, channels);
        return -1;
    }

    model_t *m = (model_t *)model;
    if (strcmp(m->type, MODEL_TYPE_SOD) != 0) {
        log_error("Invalid model type for detect_with_sod_model: %s", m->type);
        return -1;
    }

    if (!sod_available) {
        log_error("SOD library not available");
        return -1;
    }

    // Step 1: Create a SOD image
    log_info("Step 1: Creating SOD image from frame data (dimensions: %dx%d, channels: %d)",
            width, height, channels);

    sod_img img = sod_make_image(width, height, channels);
    if (!img.data) {
        log_error("Failed to create SOD image");
        return -1;
    }

    // Step 2: Copy the frame data to the SOD image
    log_info("Step 2: Copying frame data to SOD image");

    // Calculate the total size of the image data with overflow check
    size_t pixel_count = (size_t)width * (size_t)height;
    if (pixel_count / width != height) { // Check for overflow
        log_error("Integer overflow in image dimensions: width=%d, height=%d", width, height);
        sod_free_image(img);
        return -1;
    }

    size_t total_size = pixel_count * channels;
    if (total_size / pixel_count != channels) { // Check for overflow
        log_error("Integer overflow in total size calculation: width=%d, height=%d, channels=%d",
                 width, height, channels);
        sod_free_image(img);
        return -1;
    }

    // Allocate a temporary buffer to store the converted data
    float *temp_buffer = (float *)malloc(total_size * sizeof(float));
    if (!temp_buffer) {
        log_error("Failed to allocate temporary buffer for image data conversion (size=%zu bytes)",
                 total_size * sizeof(float));
        sod_free_image(img);
        return -1;
    }

    // Initialize the temp buffer to zeros for safety
    memset(temp_buffer, 0, total_size * sizeof(float));

    // Convert the frame data from HWC to CHW format and from 0-255 to 0-1 range
    for (int c = 0; c < channels; c++) {
        for (int h = 0; h < height; h++) {
            for (int w = 0; w < width; w++) {
                // Calculate the index in the SOD image (CHW format) with overflow checks
                size_t c_offset = (size_t)c * (size_t)height * (size_t)width;
                size_t h_offset = (size_t)h * (size_t)width;
                size_t sod_idx = c_offset + h_offset + w;

                // Calculate the index in the frame data (HWC format) with overflow checks
                size_t row_offset = (size_t)h * (size_t)width * (size_t)channels;
                size_t pixel_offset = (size_t)w * (size_t)channels;
                size_t frame_idx = row_offset + pixel_offset + c;

                // Make sure the indices are within bounds
                if (sod_idx < total_size && frame_idx < total_size) {
                    // Convert from 0-255 to 0-1 range
                    temp_buffer[sod_idx] = frame_data[frame_idx] / 255.0f;
                } else {
                    log_warn("Index out of bounds: sod_idx=%zu, frame_idx=%zu, total_size=%zu",
                             sod_idx, frame_idx, total_size);
                }
            }
        }
    }

    // Copy the converted data to the SOD image with NULL check
    if (img.data != NULL) {
        memcpy(img.data, temp_buffer, total_size * sizeof(float));
    } else {
        log_error("SOD image data pointer is NULL");
        free(temp_buffer);
        sod_free_image(img);
        return -1;
    }

    // Free the temporary buffer
    free(temp_buffer);

    log_info("Step 3: Successfully copied frame data to SOD image");

    // Step 3: Prepare the image for CNN detection
    log_info("Step 4: Preparing image for CNN detection with model=%p", (void*)m->sod.model);
    float *prepared_data = NULL;

    // Extra safety check for model pointer
    if (!m->sod.model) {
        log_error("Model pointer is NULL before preparing image");
        sod_free_image(img);
        return -1;
    }

    prepared_data = sod_cnn_prepare_image(m->sod.model, img);
    if (!prepared_data) {
        log_error("Failed to prepare image for CNN detection");
        sod_free_image(img);
        return -1;
    }

    log_info("Step 5: Successfully prepared image for CNN detection");

    // Step 4: Run detection
    log_info("Step 6: Running CNN detection");
    int count = 0;
    void **boxes_ptr = NULL;

    // Add extra safety check
    if (!m->sod.model) {
        log_error("Model pointer is NULL before prediction");
        // prepared_data is freed when we free the image
        sod_free_image(img);
        return -1;
    }

    // Step 5: Call predict
    sod_box *boxes = NULL;
    int rc;

    // Extra safety check for prepared_data
    if (!prepared_data) {
        log_error("Prepared data is NULL before prediction");
        sod_free_image(img);
        return -1;
    }

    // Try-catch block to prevent segmentation faults
    rc = sod_cnn_predict((sod_cnn*)m->sod.model, prepared_data, &boxes, &count);
    log_info("Step 7: sod_cnn_predict returned with rc=%d, count=%d", rc, count);

    if (rc != 0) { // SOD_OK is 0
        log_error("CNN detection failed with error code: %d", rc);
        sod_free_image(img); // This also frees prepared_data
        return -1;
    }

    // Extra safety check for boxes pointer
    if (!boxes && count > 0) {
        log_error("Boxes pointer is NULL but count is %d", count);
        count = 0; // Reset count to avoid accessing NULL pointer
    }

    // Step 6: Process detection results
    log_info("Step 8: Processing detection results");

    // Initialize result
    result->count = 0;

    // Process detection results
    int valid_count = 0;

    // Skip processing boxes if count is 0 or boxes is NULL
    if (count <= 0 || !boxes) {
        log_warn("No detection boxes returned (count=%d, boxes=%p)", count, (void*)boxes);
        sod_free_image(img); // This also frees prepared_data
        result->count = 0; // Ensure result is properly initialized
        return 0;
    }

    log_info("Processing %d detection boxes", count);

    // For static linking, boxes is already an array of sod_box structures

    for (int i = 0; i < count && valid_count < MAX_DETECTIONS; i++) {
        // Get the current box - with bounds checking
        if (i < 0 || i >= count) {
            log_warn("Box index %d out of bounds (count=%d), skipping", i, count);
            continue;
        }

        sod_box *box = &boxes[i];

        // This check is redundant since box is a reference, not a pointer
        // but we'll keep a modified version for safety
        if (box == NULL) {
            log_warn("Box %d is NULL, skipping", i);
            continue;
        }

        // Log box values for debugging
        log_info("Box %d: x=%d, y=%d, w=%d, h=%d, score=%.2f, name=%s",
                i, box->x, box->y, box->w, box->h, box->score,
                box->zName ? box->zName : "unknown");

        // CRITICAL FIX: Add extra validation for box values
        if (box->x < 0 || box->y < 0 || box->w <= 0 || box->h <= 0 ||
            box->x + box->w > width || box->y + box->h > height) {
            log_warn("Box %d has invalid coordinates (x=%d, y=%d, w=%d, h=%d, img_w=%d, img_h=%d), skipping",
                    i, box->x, box->y, box->w, box->h, width, height);
            continue;
        }

        // Validate zName pointer
        if (box->zName == NULL) {
            log_warn("Box %d has NULL name, using 'unknown'", i);
        }

        char label[MAX_LABEL_LENGTH];
        const char *name = box->zName ? box->zName : "object";

        // Extra safety check for name string
        if (name && strlen(name) > 0) {
            strncpy(label, name, MAX_LABEL_LENGTH - 1);
        } else {
            strncpy(label, "object", MAX_LABEL_LENGTH - 1);
        }
        label[MAX_LABEL_LENGTH - 1] = '\0';

        // Clamp confidence to valid range [0.0, 1.0]
        float confidence = box->score;
        if (confidence > 1.0f) confidence = 1.0f;
        if (confidence < 0.0f) confidence = 0.0f;

        // Convert pixel coordinates to normalized 0-1 range with safety checks
        float x = (width > 0) ? ((float)box->x / width) : 0.0f;
        float y = (height > 0) ? ((float)box->y / height) : 0.0f;
        float w = (width > 0) ? ((float)box->w / width) : 0.0f;
        float h = (height > 0) ? ((float)box->h / height) : 0.0f;

        // Clamp values to [0.0, 1.0] range
        x = (x < 0.0f) ? 0.0f : (x > 1.0f ? 1.0f : x);
        y = (y < 0.0f) ? 0.0f : (y > 1.0f ? 1.0f : y);
        w = (w < 0.0f) ? 0.0f : (w > 1.0f ? 1.0f : w);
        h = (h < 0.0f) ? 0.0f : (h > 1.0f ? 1.0f : h);

        // Apply threshold
        if (confidence < m->sod.threshold) {
            log_info("Detection %d below threshold: %s (%.2f%%) at [%.2f, %.2f, %.2f, %.2f]",
                    i, label, confidence * 100.0f, x, y, w, h);
            continue;
        }

        // Add valid detection to result
        strncpy(result->detections[valid_count].label, label, MAX_LABEL_LENGTH - 1);
        result->detections[valid_count].confidence = confidence;
        result->detections[valid_count].x = x;
        result->detections[valid_count].y = y;
        result->detections[valid_count].width = w;
        result->detections[valid_count].height = h;

        log_info("Valid detection %d: %s (%.2f%%) at [%.2f, %.2f, %.2f, %.2f]",
                valid_count, label, confidence * 100.0f, x, y, w, h);

        valid_count++;
    }

    result->count = valid_count;
    log_info("Detection found %d valid objects out of %d total", valid_count, count);

    // Step 7: Free the image data and prepared data
    log_info("Step 9: Freeing SOD image and prepared data");
    sod_free_image(img);
    // Note: The prepared_data is actually part of the SOD image structure, so we don't need to free it separately
    // It's automatically freed when we call sod_free_image(img)

    return 0;
}
