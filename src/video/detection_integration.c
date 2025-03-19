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
#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <libavutil/avutil.h>
#include <libavutil/imgutils.h>
#include <libswscale/swscale.h>
#include <libavformat/avformat.h>
#include <libavutil/opt.h>

// Include SOD header if SOD is enabled at compile time
#ifdef SOD_ENABLED
#include "sod/sod.h"
#endif

#include "core/logger.h"
#include "core/config.h"
#include "video/stream_manager.h"
#include "video/streams.h"
#include "video/detection.h"
#include "video/detection_result.h"
#include "video/motion_detection.h"
#include "video/motion_detection_wrapper.h"

// Define model types
#define MODEL_TYPE_SOD "sod"
#define MODEL_TYPE_SOD_REALNET "sod_realnet"
#define MODEL_TYPE_TFLITE "tflite"

// Debug flag to enable/disable frame saving
static int save_frames_for_debug = 1;  // Set to 1 to enable frame saving

// Global model cache to ensure persistence across all function calls
// This ensures models stay loaded between frames for all streams
static struct {
    char path[MAX_PATH_LENGTH];
    detection_model_t model;
    time_t last_used;
} global_model_cache[MAX_STREAMS] = {{{0}}};

// Mutex to protect access to the global model cache
static pthread_mutex_t model_cache_mutex = PTHREAD_MUTEX_INITIALIZER;

// Function to check if a file exists
static int file_exists(const char *filename) {
    FILE *file = fopen(filename, "r");
    if (file) {
        fclose(file);
        return 1;
    }
    return 0;
}

/**
 * Detect model type based on file name
 * 
 * @param model_path Path to the model file
 * @return String describing the model type (MODEL_TYPE_SOD_REALNET, MODEL_TYPE_SOD, etc.)
 */
const char* detect_model_type(const char *model_path) {
    if (!model_path) {
        return "unknown";
    }
    
    // Check for SOD RealNet models
    if (strstr(model_path, ".realnet.sod") != NULL) {
        return MODEL_TYPE_SOD_REALNET;
    }
    
    // Check for regular SOD models
    const char *ext = strrchr(model_path, '.');
    if (ext && strcasecmp(ext, ".sod") == 0) {
        return MODEL_TYPE_SOD;
    }
    
    // Check for TFLite models
    if (ext && strcasecmp(ext, ".tflite") == 0) {
        return MODEL_TYPE_TFLITE;
    }
    
    return "unknown";
}

/**
 * Process a decoded frame for detection
 * This function should be called from the HLS streaming code with already decoded frames
 *
 * Revised to perform detection and pass results to process_frame_for_detection
 *
 * @param stream_name The name of the stream
 * @param frame The decoded AVFrame
 * @param detection_interval How often to process frames (e.g., every 10 frames)
 * @return 0 on success, -1 on error
 */
int process_decoded_frame_for_detection(const char *stream_name, AVFrame *frame, int detection_interval) {
    // CRITICAL FIX: Add extra validation for all parameters
    if (!stream_name) {
        log_error("process_decoded_frame_for_detection: NULL stream name");
        return -1;
    }

    if (!frame) {
        log_error("process_decoded_frame_for_detection: NULL frame for stream %s", stream_name);
        return -1;
    }

    // CRITICAL FIX: Validate frame data
    if (frame->width <= 0 || frame->height <= 0 || !frame->data[0]) {
        log_error("process_decoded_frame_for_detection: Invalid frame dimensions or data for stream %s: width=%d, height=%d, data=%p",
                 stream_name, frame->width, frame->height, (void*)frame->data[0]);
        return -1;
    }

    static int frame_counters[MAX_STREAMS] = {0};
    static char stream_names[MAX_STREAMS][MAX_STREAM_NAME] = {{0}};

    // Variables for cleanup
    AVFrame *converted_frame = NULL;
    struct SwsContext *sws_ctx = NULL;
    uint8_t *buffer = NULL;
    uint8_t *packed_buffer = NULL;
    int detect_ret = -1;

    // Find the stream's frame counter
    int stream_idx = -1;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (stream_names[i][0] == '\0') {
            // Empty slot, use it for this stream
            if (stream_idx == -1) {
                stream_idx = i;
                strncpy(stream_names[i], stream_name, MAX_STREAM_NAME - 1);
                stream_names[i][MAX_STREAM_NAME - 1] = '\0';
            }
        } else if (strcmp(stream_names[i], stream_name) == 0) {
            // Found existing stream
            stream_idx = i;
            break;
        }
    }

    if (stream_idx == -1) {
        log_error("Too many streams for frame counters");
        return -1;
    }

    // Increment frame counter for this stream
    frame_counters[stream_idx]++;
    int frame_counter = frame_counters[stream_idx];

    // Reset counter if it gets too large to prevent overflow
    if (frame_counter > 1000000) {
        frame_counters[stream_idx] = 0;
    }

    // Skip frames based on detection interval
    if (frame_counter % detection_interval != 0) {
        return 0; // Skip this frame
    }

    log_info("Processing decoded frame %d for stream %s (interval: %d)", frame_counter, stream_name, detection_interval);

    // Get stream configuration
    stream_handle_t stream_handle = get_stream_by_name(stream_name);
    if (!stream_handle) {
        log_error("Failed to get stream handle for %s - stream may not exist", stream_name);
        return -1;
    }

    stream_config_t config;
    if (get_stream_config(stream_handle, &config) != 0) {
        log_error("Failed to get stream config for %s", stream_name);
        return -1;
    }

    // Check if detection is enabled for this stream
    if (!config.detection_based_recording || config.detection_model[0] == '\0') {
        log_info("Detection not enabled for stream %s", stream_name);
        return 0;
    }

    log_info("Detection enabled for stream %s with model %s", stream_name, config.detection_model);

    // Determine model type to use the correct image format
    const char *model_type = detect_model_type(config.detection_model);
    log_info("Detected model type: %s for model %s", model_type, config.detection_model);

    // For RealNet models, we need grayscale images
    // For CNN models, we need RGB images
    enum AVPixelFormat target_format;
    int channels;

    // Use appropriate target size for detection based on model type
    int target_width, target_height;

    if (strcmp(model_type, MODEL_TYPE_SOD_REALNET) == 0) {
        // RealNet models work well with smaller images
        target_width = 640;
        target_height = 480;
        target_format = AV_PIX_FMT_GRAY8;
        channels = 1;
        log_info("Using grayscale format for RealNet model");
    } else {
        // For SOD CNN models, we need to maintain original dimensions where possible
        // but cap at 416x416 as a default for performance
        target_width = frame->width;
        target_height = frame->height;

        // Cap dimensions at reasonable limits for CNN models if needed
        if (target_width > 416) target_width = 416;
        if (target_height > 416) target_height = 416;

        target_format = AV_PIX_FMT_RGB24;
        channels = 3;
        log_info("Using RGB format for non-RealNet model");
    }

    // Cap dimensions at the original frame size if smaller
    if (frame->width < target_width) target_width = frame->width;
    if (frame->height < target_height) target_height = frame->height;

    log_info("Using detection resolution %dx%d for stream %s",
            target_width, target_height, stream_name);

    // Convert frame to the appropriate format for detection
    // Use SWS_BILINEAR for better quality with SOD CNN models
    int sws_flags = strcmp(model_type, MODEL_TYPE_SOD) == 0 ? SWS_BILINEAR : SWS_FAST_BILINEAR;

    // Try to create SwsContext with the requested format
    sws_ctx = sws_getContext(
        frame->width, frame->height, frame->format,
        target_width, target_height, target_format,
        sws_flags, NULL, NULL, NULL);

    // If that fails, try with a different pixel format
    if (!sws_ctx && target_format == AV_PIX_FMT_RGB24) {
        log_warn("Failed to create SwsContext with RGB24, trying with YUV420P for stream %s", stream_name);
        target_format = AV_PIX_FMT_YUV420P;
        channels = 1; // YUV420P is treated as 1 channel for our purposes

        sws_ctx = sws_getContext(
            frame->width, frame->height, frame->format,
            target_width, target_height, target_format,
            sws_flags, NULL, NULL, NULL);
    }

    if (!sws_ctx) {
        log_error("Failed to create SwsContext for stream %s", stream_name);
        goto cleanup;
    }

    // Allocate converted frame
    converted_frame = av_frame_alloc();
    if (!converted_frame) {
        log_error("Failed to allocate converted frame for stream %s", stream_name);
        goto cleanup;
    }

    // Allocate buffer for converted frame
    int buffer_size = av_image_get_buffer_size(target_format, target_width, target_height, 1);
    buffer = (uint8_t *)av_malloc(buffer_size);

    // If allocation fails, try with smaller dimensions as a fallback
    if (!buffer) {
        log_warn("Initial buffer allocation failed, trying with smaller dimensions for stream %s", stream_name);

        // Reduce dimensions by 25% but keep aspect ratio
        target_width = (target_width * 3) / 4;
        target_height = (target_height * 3) / 4;

        // Ensure dimensions are even (required by some video processing functions)
        target_width = (target_width / 2) * 2;
        target_height = (target_height / 2) * 2;

        // Try again with smaller dimensions
        buffer_size = av_image_get_buffer_size(target_format, target_width, target_height, 1);
        buffer = (uint8_t *)av_malloc(buffer_size);

        log_info("Retrying with reduced dimensions: %dx%d", target_width, target_height);
    }

    if (!buffer) {
        log_error("Failed to allocate buffer for converted frame for stream %s", stream_name);
        goto cleanup;
    }

    // Setup converted frame
    av_image_fill_arrays(converted_frame->data, converted_frame->linesize, buffer,
                        target_format, target_width, target_height, 1);

    // Convert frame to target format
    sws_scale(sws_ctx, (const uint8_t * const *)frame->data, frame->linesize, 0,
             frame->height, converted_frame->data, converted_frame->linesize);

    log_info("Converted frame to %s format (%dx%d) for stream %s",
             (channels == 1) ? "grayscale" : "RGB",
             target_width, target_height, stream_name);

    // Improved SOD CNN conversion with better memory management
    if (strcmp(model_type, MODEL_TYPE_SOD) == 0 && channels == 3) {
        log_info("Creating planar format buffer for SOD CNN model with memory optimization");

        // Get original dimensions and preserve aspect ratio
        int original_width = frame->width;
        int original_height = frame->height;
        float aspect_ratio = (float)original_width / (float)original_height;

        // Calculate target dimensions that preserve aspect ratio
        // Keep maximum dimension at 416, scale the other proportionally
        int max_dim = 416;
        if (aspect_ratio > 1.0f) {  // Wider than tall
            target_width = max_dim;
            target_height = (int)(max_dim / aspect_ratio);
        } else {  // Taller than wide
            target_height = max_dim;
            target_width = (int)(max_dim * aspect_ratio);
        }

        // Ensure even dimensions (required by some video codecs)
        target_width = (target_width / 2) * 2;
        target_height = (target_height / 2) * 2;

        log_info("Using aspect-preserving dimensions: %dx%d (ratio: %.2f)",
                 target_width, target_height, aspect_ratio);

        // Free existing conversion context if needed
        if (sws_ctx) {
            sws_freeContext(sws_ctx);
            sws_ctx = NULL;
        }

        // Create SwsContext with the adjusted dimensions
        sws_ctx = sws_getContext(
            frame->width, frame->height, frame->format,
            target_width, target_height, AV_PIX_FMT_RGB24,
            SWS_BILINEAR, NULL, NULL, NULL);

        if (!sws_ctx) {
            log_error("Failed to create SwsContext with aspect ratio preservation");
            goto cleanup;
        }

        // Free existing buffer if needed (important to prevent memory leaks)
        if (buffer) {
            av_free(buffer);
            buffer = NULL;
        }

        // Allocate new buffer for the correct dimensions
        buffer_size = av_image_get_buffer_size(AV_PIX_FMT_RGB24, target_width, target_height, 1);
        buffer = (uint8_t *)av_malloc(buffer_size);
        if (!buffer) {
            log_error("Failed to allocate buffer for aspect-preserved dimensions");
            goto cleanup;
        }

        // Setup converted frame with new dimensions
        av_image_fill_arrays(converted_frame->data, converted_frame->linesize, buffer,
                             AV_PIX_FMT_RGB24, target_width, target_height, 1);

        // Convert frame with proper dimensions
        sws_scale(sws_ctx, (const uint8_t * const *)frame->data, frame->linesize, 0,
                  frame->height, converted_frame->data, converted_frame->linesize);

        log_info("Converted frame to RGB format (%dx%d) with aspect ratio preserved",
                 target_width, target_height);

        // Calculate plane size for planarization
        int plane_size = target_width * target_height;

        // Allocate the planar buffer - all three planes in one contiguous allocation
        packed_buffer = (uint8_t *)malloc(plane_size * channels);
        if (!packed_buffer) {
            log_error("Failed to allocate planar buffer");
            goto cleanup;
        }

        // Initialize plane pointers
        uint8_t *r_plane = packed_buffer;
        uint8_t *g_plane = r_plane + plane_size;
        uint8_t *b_plane = g_plane + plane_size;

        // Check if we can optimize by avoiding stride handling
        bool has_stride = (converted_frame->linesize[0] != target_width * channels);
        if (has_stride) {
            log_info("Processing frame with stride padding");
        } else {
            log_info("Processing frame without stride (optimized path)");
        }

        // Planarize with optimized memory access pattern
        for (int y = 0; y < target_height; y++) {
            // Get pointer to start of this row in the source
            const uint8_t *src_row = converted_frame->data[0] + y * converted_frame->linesize[0];

            // Calculate row offset in destination planes
            int row_offset = y * target_width;

            for (int x = 0; x < target_width; x++) {
                // Calculate pixel index
                int dst_idx = row_offset + x;

                // If no stride, we can use direct indexing
                if (!has_stride) {
                    // Source index is x * 3 within the row
                    int src_idx = x * 3;
                    r_plane[dst_idx] = src_row[src_idx];     // R
                    g_plane[dst_idx] = src_row[src_idx + 1]; // G
                    b_plane[dst_idx] = src_row[src_idx + 2]; // B
                } else {
                    // With stride, use the safer approach
                    r_plane[dst_idx] = src_row[x * 3];     // R
                    g_plane[dst_idx] = src_row[x * 3 + 1]; // G
                    b_plane[dst_idx] = src_row[x * 3 + 2]; // B
                }
            }
        }

        // Log sample of the planar data for debugging
        log_info("First few pixels - RGB samples:");
        for (int i = 0; i < 3 && i < target_width; i++) {
            log_info("  Pixel %d: R=%d, G=%d, B=%d",
                    i, r_plane[i], g_plane[i], b_plane[i]);
        }

        log_info("Successfully converted to planar format for SOD CNN model");
    } else {
        // For other models, use the standard approach (interleaved or grayscale)
        if (converted_frame->linesize[0] == target_width * channels) {
            // No padding, can use the buffer directly
            packed_buffer = buffer;
            log_info("Using direct buffer access (no padding)");
        } else {
            // Need to create a packed buffer without stride padding
            packed_buffer = (uint8_t *)malloc(target_width * target_height * channels);
            if (!packed_buffer) {
                log_error("Failed to allocate packed buffer for frame");
                goto cleanup;
            }

            // Copy each row, removing stride padding
            for (int y = 0; y < target_height; y++) {
                memcpy(packed_buffer + y * target_width * channels,
                      converted_frame->data[0] + y * converted_frame->linesize[0],
                      target_width * channels);
            }
        }
    }

    // Log some debug info about the packed buffer
    log_info("Packed buffer first 12 bytes: %02x %02x %02x %02x %02x %02x %02x %02x %02x %02x %02x %02x",
            packed_buffer[0], packed_buffer[1], packed_buffer[2], packed_buffer[3],
            packed_buffer[4], packed_buffer[5], packed_buffer[6], packed_buffer[7],
            packed_buffer[8], packed_buffer[9], packed_buffer[10], packed_buffer[11]);

    // Get the appropriate threshold for the model type
    float threshold = config.detection_threshold;
    if (threshold <= 0.0f) {
        if (strcmp(model_type, MODEL_TYPE_SOD_REALNET) == 0) {
            threshold = 5.0f; // RealNet models typically use 5.0
            log_info("Using default threshold of 5.0 for RealNet model");
        } else {
            threshold = 0.3f; // CNN models typically use 0.3
            log_info("Using default threshold of 0.3 for CNN model");
        }
    } else {
        // Only cap threshold for non-RealNet models and only if it's too high
        if (strcmp(model_type, MODEL_TYPE_SOD_REALNET) != 0 && threshold > 0.4f) {
            log_warn("Configured threshold %.2f is too high for reliable detection, capping at 0.4", threshold);
            threshold = 0.4f;
        }
        log_info("Using threshold of %.2f for model", threshold);
    }

    // Check if model_path is a relative path and resolve to full path
    char full_model_path[MAX_PATH_LENGTH];
    if (config.detection_model[0] != '/') {
        // Get current working directory
        char cwd[MAX_PATH_LENGTH];
        if (getcwd(cwd, sizeof(cwd)) != NULL) {
            // Construct full path: CWD/models/model_path
            snprintf(full_model_path, MAX_PATH_LENGTH, "%s/models/%s", cwd, config.detection_model);

            // Validate path exists
            if (!file_exists(full_model_path)) {
                log_error("Model file does not exist: %s", full_model_path);

                // Try alternative locations
                char alt_path[MAX_PATH_LENGTH];
                const char *locations[] = {
                    "./", // Current directory
                    "./build/models/", // Build directory
                    "../models/", // Parent directory
                    "/var/lib/lightnvr/models/" // System directory
                };

                for (int i = 0; i < sizeof(locations)/sizeof(locations[0]); i++) {
                    snprintf(alt_path, MAX_PATH_LENGTH, "%s%s", locations[i], config.detection_model);
                    if (file_exists(alt_path)) {
                        log_info("Found model at alternative location: %s", alt_path);
                        strncpy(full_model_path, alt_path, MAX_PATH_LENGTH - 1);
                        break;
                    }
                }
            }
        } else {
            // Fallback
            snprintf(full_model_path, MAX_PATH_LENGTH, "/var/lib/lightnvr/models/%s", config.detection_model);
        }
    } else {
        // Already an absolute path
        strncpy(full_model_path, config.detection_model, MAX_PATH_LENGTH - 1);
    }

    log_info("Using model path: %s", full_model_path);

    // Create a detection result structure
    detection_result_t result;
    memset(&result, 0, sizeof(detection_result_t));

    // Get current time for frame timestamp
    time_t frame_time = time(NULL);

    // Check if we should use motion detection
    bool use_motion_detection = (strcmp(config.detection_model, "motion") == 0);

    // If the model is "motion", enable optimized motion detection
    if (use_motion_detection) {
        // Initialize optimized motion detection if not already initialized
        if (!is_motion_detection_enabled(stream_name)) {
            // Use the wrapper function which handles proper initialization and cleanup
            enable_optimized_motion_detection(stream_name, 0.25f, 0.01f, 3, 2);
            log_info("Automatically enabled optimized motion detection for stream %s based on model setting", stream_name);
        }
    }

    detect_ret = -1;

    // If motion detection is enabled, run the optimized implementation
    if (use_motion_detection) {
        log_info("Running optimized motion detection for stream %s", stream_name);

        // Create a separate result for motion detection
        detection_result_t motion_result;
        memset(&motion_result, 0, sizeof(detection_result_t));

        // Initialize optimized motion detection if not already initialized
        if (!is_motion_detection_enabled(stream_name)) {
            // Use the wrapper function which handles proper initialization and cleanup
            enable_optimized_motion_detection(stream_name, 0.25f, 0.01f, 3, 2);
            log_info("Initialized optimized motion detection for stream %s", stream_name);
        }

        // Run optimized motion detection only if it's properly enabled
        if (is_motion_detection_enabled(stream_name)) {
            int motion_ret = detect_motion(stream_name, packed_buffer, target_width, target_height,
                                          channels, frame_time, &motion_result);

            if (motion_ret == 0 && motion_result.count > 0) {
                log_info("Motion detected (optimized) in stream %s: confidence=%.2f",
                        stream_name, motion_result.detections[0].confidence);

                // Pass motion detection results to process_frame_for_recording
                int ret = process_frame_for_recording(stream_name, packed_buffer, target_width,
                                                     target_height, channels, frame_time, &motion_result);

                if (ret != 0) {
                    log_error("Failed to process optimized motion detection results for recording (error code: %d)", ret);
                }

                // If we're only using motion detection, we're done
                if (use_motion_detection) {
                    detect_ret = 0;
                }
            } else if (motion_ret != 0) {
                log_error("Optimized motion detection failed (error code: %d)", motion_ret);
            }
        } else {
            log_warn("Motion detection is not properly enabled for stream %s, skipping detection", stream_name);
        }
    }

    // If we're using a model-based detection (not just motion), run it
    if (!use_motion_detection) {
        // Find model in cache or load it
        detection_model_t model = NULL;
        int cache_idx = -1;

        // Lock the model cache mutex for thread safety
        pthread_mutex_lock(&model_cache_mutex);

        // Look for model in global cache
        for (int i = 0; i < MAX_STREAMS; i++) {
            if (global_model_cache[i].path[0] != '\0' &&
                strcmp(global_model_cache[i].path, full_model_path) == 0) {
                model = global_model_cache[i].model;
                cache_idx = i;
                global_model_cache[i].last_used = time(NULL);
                log_info("Using cached detection model for %s", full_model_path);
                break;
            }
        }

        // If not found in cache, load it
        if (!model) {
            // Find an empty slot or the oldest used model
            time_t oldest_time = time(NULL);
            int oldest_idx = -1;

            for (int i = 0; i < MAX_STREAMS; i++) {
                if (global_model_cache[i].path[0] == '\0') {
                    oldest_idx = i;
                    break;
                } else if (global_model_cache[i].last_used < oldest_time) {
                    oldest_time = global_model_cache[i].last_used;
                    oldest_idx = i;
                }
            }

            // If we found a slot, load the model
            if (oldest_idx >= 0) {
                // If slot was used, unload the old model
                if (global_model_cache[oldest_idx].path[0] != '\0') {
                    log_info("Unloading cached model %s to make room for %s",
                            global_model_cache[oldest_idx].path, full_model_path);
                    unload_detection_model(global_model_cache[oldest_idx].model);
                    global_model_cache[oldest_idx].path[0] = '\0';
                    global_model_cache[oldest_idx].model = NULL;
                }

                // Load the new model
                log_info("Loading detection model: %s with threshold: %.2f", full_model_path, threshold);

                // Check if file exists before loading
                FILE *model_file = fopen(full_model_path, "r");
                if (!model_file) {
                    log_error("MODEL FILE NOT FOUND: %s", full_model_path);

                    // Try alternative locations
                    const char *locations[] = {
                        "./", // Current directory
                        "./build/models/", // Build directory
                        "../models/", // Parent directory
                        "/var/lib/lightnvr/models/" // System directory
                    };

                    bool found = false;
                    for (int j = 0; j < sizeof(locations)/sizeof(locations[0]); j++) {
                        char alt_path[MAX_PATH_LENGTH];
                        snprintf(alt_path, MAX_PATH_LENGTH, "%s%s",
                                locations[j], strrchr(full_model_path, '/') ?
                                strrchr(full_model_path, '/') + 1 : full_model_path);

                        FILE *alt_file = fopen(alt_path, "r");
                        if (alt_file) {
                            fclose(alt_file);
                            log_info("MODEL FOUND AT ALTERNATIVE LOCATION: %s", alt_path);
                            strncpy(full_model_path, alt_path, MAX_PATH_LENGTH - 1);
                            found = true;
                            break;
                        }
                    }

                    if (!found) {
                        log_error("MODEL NOT FOUND IN ANY LOCATION!");
                    }
                } else {
                    fclose(model_file);
                    log_info("MODEL FILE EXISTS: %s", full_model_path);
                }

                model = load_detection_model(full_model_path, threshold);

                if (model) {
                    // Cache the model in the global cache
                    strncpy(global_model_cache[oldest_idx].path, full_model_path, MAX_PATH_LENGTH - 1);
                    global_model_cache[oldest_idx].model = model;
                    global_model_cache[oldest_idx].last_used = time(NULL);
                    cache_idx = oldest_idx;
                    log_info("Cached detection model for %s in slot %d", full_model_path, oldest_idx);
                }
            }
        }

        // Unlock the model cache mutex
        pthread_mutex_unlock(&model_cache_mutex);

        if (!model) {
            log_error("Failed to load detection model: %s", full_model_path);
        } else {
            // Use our improved detect_objects function for ALL model types
            log_info("Running detection with unified detect_objects function");
            detect_ret = detect_objects(model, packed_buffer, target_width, target_height, channels, &result);

            if (detect_ret != 0) {
                log_error("Detection failed (error code: %d)", detect_ret);
            } else {
                log_info("Detection completed successfully, found %d objects", result.count);

                // Log detection results
                for (int i = 0; i < result.count; i++) {
                    log_info("Detection %d: %s (%.2f%%) at [%.2f, %.2f, %.2f, %.2f]",
                             i+1, result.detections[i].label,
                             result.detections[i].confidence * 100.0f,
                             result.detections[i].x, result.detections[i].y,
                             result.detections[i].width, result.detections[i].height);
                }

                // Pass detection results to process_frame_for_recording
                int ret = process_frame_for_recording(stream_name, packed_buffer, target_width,
                                                     target_height, channels, frame_time, &result);

                if (ret != 0) {
                    log_error("Failed to process detection results for recording (error code: %d)", ret);
                }
            }

            // Note: We don't unload the model here, it stays in the cache
        }
    }

cleanup:
        // Log memory release for debugging
        log_info("Cleaning up resources for frame processing");

    // Ensure proper cleanup to prevent memory leaks
    if (packed_buffer && packed_buffer != buffer) {
        // Only free packed_buffer if it's not the same as buffer
        free(packed_buffer);
        log_info("Freed packed_buffer memory");
    }

    if (buffer) {
        av_free(buffer);
        log_info("Freed buffer memory");
        // If packed_buffer was pointing to buffer, it's now invalid
        if (packed_buffer == buffer) {
            packed_buffer = NULL;
        }
    }

    if (converted_frame) {
        av_frame_free(&converted_frame);
        log_info("Freed converted_frame");
    }

    if (sws_ctx) {
        sws_freeContext(sws_ctx);
        log_info("Freed sws_ctx");
    }

    log_info("All resources freed for frame %d", frame_counter);
    return (detect_ret == 0) ? 0 : -1;
}
