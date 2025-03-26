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
#include "video/sod_integration.h"
#include "utils/memory.h"

// Include our new modules
#include "video/detection_config.h"
#include "video/detection_buffer.h"
#include "video/detection_embedded.h"
#include "video/detection_integration.h"

// Track which streams are currently being processed for detection
static char *active_detection_streams = NULL;
static int active_detections = 0;
static int max_detections = 0;

/**
 * Initialize the detection integration system
 */
int init_detection_integration(void) {
    // Initialize configuration
    if (init_detection_config() != 0) {
        log_error("Failed to initialize detection configuration");
        return -1;
    }
    
    // Initialize buffer pool
    if (init_buffer_pool() != 0) {
        log_error("Failed to initialize buffer pool");
        return -1;
    }
    
    // Get configuration
    detection_config_t *config = get_detection_config();
    if (!config) {
        log_error("Failed to get detection configuration");
        return -1;
    }
    
    // Allocate active detection streams array
    max_detections = config->concurrent_detections;
    active_detection_streams = (char *)calloc(max_detections, MAX_STREAM_NAME);
    if (!active_detection_streams) {
        log_error("Failed to allocate active detection streams array");
        return -1;
    }
    
    log_info("Detection integration initialized with %d max concurrent detections", max_detections);
    return 0;
}

/**
 * Process a decoded frame for detection
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
    
    // Initialize if not already done
    if (!active_detection_streams) {
        if (init_detection_integration() != 0) {
            log_error("Failed to initialize detection integration");
            return -1;
        }
    }
    
    // Get configuration
    detection_config_t *config = get_detection_config();
    if (!config) {
        log_error("Failed to get detection configuration");
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

    // Get current time
    time_t current_time = time(NULL);
    
    // Check if we have a last detection time for this stream
    static time_t last_detection_times[MAX_STREAMS] = {0};
    
    // Check if enough time has passed since the last detection
    if (last_detection_times[stream_idx] > 0) {
        time_t time_since_last = current_time - last_detection_times[stream_idx];
        
        // If we haven't waited long enough since the last detection, skip this frame
        if (time_since_last < detection_interval) {
            log_debug("Skipping detection for stream %s - only %ld seconds since last detection (interval: %d)",
                     stream_name, time_since_last, detection_interval);
            return 0; // Skip this frame
        }
    }
    
    // We're going to process this frame, update the last detection time
    last_detection_times[stream_idx] = current_time;
    
    // Increment frame counter for this stream (keep for debugging)
    frame_counters[stream_idx]++;
    int frame_counter = frame_counters[stream_idx];

    // Reset counter if it gets too large to prevent overflow
    if (frame_counter > 1000000) {
        frame_counters[stream_idx] = 0;
    }

    log_info("Processing decoded frame %d for stream %s (interval: %d)", frame_counter, stream_name, detection_interval);

    // Get stream configuration
    stream_handle_t stream_handle = get_stream_by_name(stream_name);
    if (!stream_handle) {
        log_error("Failed to get stream handle for %s - stream may not exist", stream_name);
        return -1;
    }

    stream_config_t stream_config;
    if (get_stream_config(stream_handle, &stream_config) != 0) {
        log_error("Failed to get stream config for %s", stream_name);
        return -1;
    }

    // Check if detection is enabled for this stream
    if (!stream_config.detection_based_recording || stream_config.detection_model[0] == '\0') {
        log_info("Detection not enabled for stream %s", stream_name);
        return 0;
    }

    log_info("Detection enabled for stream %s with model %s", stream_name, stream_config.detection_model);

    // Determine model type to use the correct image format
    const char *model_type = detect_model_type(stream_config.detection_model);
    log_info("Detected model type: %s for model %s", model_type, stream_config.detection_model);

    // For RealNet models, we need grayscale images
    // For CNN models, we need RGB images
    enum AVPixelFormat target_format;
    int channels;

    if (strcmp(model_type, MODEL_TYPE_SOD_REALNET) == 0) {
        target_format = AV_PIX_FMT_GRAY8;
        channels = 1;
        log_info("Using grayscale format for RealNet model");
    } else {
        // For SOD CNN and other models, use RGB format directly
        target_format = AV_PIX_FMT_RGB24;
        channels = 3;
        log_info("Using RGB format for non-RealNet model");
    }

    // Determine if we should downscale the frame based on model type and device
    int downscale_factor = get_downscale_factor(model_type);
    log_info("Using downscale factor %d for model type %s", downscale_factor, model_type);

    // Calculate dimensions after downscaling
    int target_width = frame->width / downscale_factor;
    int target_height = frame->height / downscale_factor;
    
    // Ensure dimensions are even (required by some codecs)
    target_width = (target_width / 2) * 2;
    target_height = (target_height / 2) * 2;
    
    log_info("Original dimensions: %dx%d, Target dimensions: %dx%d (downscale factor: %d)",
             frame->width, frame->height, target_width, target_height, downscale_factor);

    // Convert frame to the appropriate format for detection with downscaling if needed
    sws_ctx = sws_getContext(
        frame->width, frame->height, frame->format,
        target_width, target_height, target_format,
        SWS_BILINEAR, NULL, NULL, NULL);

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

    // Allocate buffer for converted frame - ensure it's large enough
    int buffer_size = av_image_get_buffer_size(target_format, target_width, target_height, 1);
    buffer = (uint8_t *)av_malloc(buffer_size);
    if (!buffer) {
        log_error("Failed to allocate buffer for converted frame for stream %s", stream_name);
        goto cleanup;
    }

    // Setup converted frame
    av_image_fill_arrays(converted_frame->data, converted_frame->linesize, buffer,
                        target_format, target_width, target_height, 1);

    // Convert frame to target format with downscaling
    sws_scale(sws_ctx, (const uint8_t * const *)frame->data, frame->linesize, 0,
             frame->height, converted_frame->data, converted_frame->linesize);

    log_info("Converted frame to %s format for stream %s (dimensions: %dx%d)",
             (channels == 1) ? "grayscale" : "RGB", stream_name, target_width, target_height);

    // Check if this stream is already being processed
    bool stream_already_active = false;
    
    for (int i = 0; i < max_detections; i++) {
        char *active_stream = active_detection_streams + i * MAX_STREAM_NAME;
        if (strcmp(active_stream, stream_name) == 0) {
            stream_already_active = true;
            break;
        }
    }
    
    // If this stream is already being processed, we can continue
    // Otherwise, check if we have room for another stream
    if (!stream_already_active) {
        // If we're at the limit, log a warning but still try to process
        // This allows all streams to get a chance at detection
        if (active_detections >= max_detections) {
            log_warn("High detection load: %d concurrent detections (limit: %d), stream %s may experience delays", 
                    active_detections, max_detections, stream_name);
        }
    }

    // Get a buffer from the pool
    size_t required_size = target_width * target_height * channels;
    packed_buffer = get_buffer_from_pool(required_size);
    
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
    
    // Increment active detections counter and track this stream
    
    // Add this stream to the active list if it's not already there
    if (!stream_already_active) {
        bool added = false;
        for (int i = 0; i < max_detections; i++) {
            char *active_stream = active_detection_streams + i * MAX_STREAM_NAME;
            if (active_stream[0] == '\0') {
                strncpy(active_stream, stream_name, MAX_STREAM_NAME - 1);
                active_stream[MAX_STREAM_NAME - 1] = '\0';
                added = true;
                break;
            }
        }
        
        // If we couldn't add it to the list (list is full), still process it
        // but don't track it (it will be treated as a one-off detection)
        if (!added) {
            log_warn("Detection tracking list full, processing stream %s as one-off detection", stream_name);
        }
    }
    
    active_detections++;
    log_info("Active detections: %d/%d for stream %s", active_detections, max_detections, stream_name);

    // Get the appropriate threshold for the model type
    float threshold = get_detection_threshold(model_type, stream_config.detection_threshold);
    log_info("Using threshold %.2f for model %s", threshold, model_type);

    // Get global config to access models path
    extern config_t g_config;
    
    // Check if model_path is a relative path
    char full_model_path[MAX_PATH_LENGTH];
    if (stream_config.detection_model[0] != '/') {
        // Construct full path using configured models path from INI if it exists
        if (g_config.models_path && strlen(g_config.models_path) > 0) {
            // Calculate available space for model name
            size_t prefix_len = strlen(g_config.models_path) + 1; // +1 for the '/'
            size_t model_max_len = MAX_PATH_LENGTH - prefix_len - 1; // -1 for null terminator
            
            // Ensure model name isn't too long
            size_t model_len = strlen(stream_config.detection_model);
            if (model_len > model_max_len) {
                log_error("Model name too long: %s (max allowed: %zu chars)", 
                         stream_config.detection_model, model_max_len);
                goto cleanup;
            }
            
            // Safe to use snprintf now
            int ret = snprintf(full_model_path, MAX_PATH_LENGTH, "%s/%s", 
                              g_config.models_path, stream_config.detection_model);
            
            // Check for truncation
            if (ret < 0 || ret >= MAX_PATH_LENGTH) {
                log_error("Path truncation occurred when creating model path");
                goto cleanup;
            }
        } else {
            // Fall back to default path if INI config doesn't exist
            // Calculate available space for model name
            const char *prefix = "/etc/lightnvr/models/";
            size_t prefix_len = strlen(prefix);
            size_t model_max_len = MAX_PATH_LENGTH - prefix_len - 1; // -1 for null terminator
            
            // Ensure model name isn't too long
            size_t model_len = strlen(stream_config.detection_model);
            if (model_len > model_max_len) {
                log_error("Model name too long: %s (max allowed: %zu chars)", 
                         stream_config.detection_model, model_max_len);
                goto cleanup;
            }
            
            // Safe to use snprintf now
            int ret = snprintf(full_model_path, MAX_PATH_LENGTH, "%s%s", 
                              prefix, stream_config.detection_model);
            
            // Check for truncation
            if (ret < 0 || ret >= MAX_PATH_LENGTH) {
                log_error("Path truncation occurred when creating model path");
                goto cleanup;
            }
        }

        // Validate path exists
        if (!file_exists(full_model_path)) {
            log_error("Model file does not exist: %s", full_model_path);

            // Try alternative locations
            char alt_path[MAX_PATH_LENGTH];
            
            // Get current working directory
            char cwd[MAX_PATH_LENGTH];
            if (getcwd(cwd, sizeof(cwd)) != NULL) {
                const char *locations[] = {
                    "/var/lib/lightnvr/models/", // Default system location
                };

                for (int i = 0; i < sizeof(locations)/sizeof(locations[0]); i++) {
                    // Calculate available space for model name
                    size_t prefix_len = strlen(locations[i]);
                    size_t model_max_len = MAX_PATH_LENGTH - prefix_len - 1; // -1 for null terminator
                    
                    // Ensure model name isn't too long
                    size_t model_len = strlen(stream_config.detection_model);
                    if (model_len > model_max_len) {
                        log_error("Model name too long for alternative location: %s (max allowed: %zu chars)", 
                                 stream_config.detection_model, model_max_len);
                        continue; // Try next location
                    }
                    
                    // Safe to use snprintf now
                    int ret = snprintf(alt_path, MAX_PATH_LENGTH, "%s%s", 
                                      locations[i],
                                      stream_config.detection_model);
                    
                    // Check for truncation
                    if (ret < 0 || ret >= MAX_PATH_LENGTH) {
                        log_error("Path truncation occurred when creating alternative model path");
                        continue; // Try next location
                    }
                    if (file_exists(alt_path)) {
                        log_info("Found model at alternative location: %s", alt_path);
                        strncpy(full_model_path, alt_path, MAX_PATH_LENGTH - 1);
                        break;
                    }
                }
            }
        }
    } else {
        // Already an absolute path
        strncpy(full_model_path, stream_config.detection_model, MAX_PATH_LENGTH - 1);
    }

    log_info("Using model path: %s", full_model_path);

    // Create a detection result structure
    detection_result_t result;
    memset(&result, 0, sizeof(detection_result_t));

    // Get current time for frame timestamp
    time_t frame_time = time(NULL);
    
    // Check if we should use motion detection
    bool use_motion_detection = (strcmp(stream_config.detection_model, "motion") == 0);

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
        // Use a static cache of loaded models to avoid loading/unloading for each frame
        static struct {
            char path[MAX_PATH_LENGTH];
            detection_model_t model;
            time_t last_used;
        } model_cache[MAX_STREAMS] = {{{0}}};
        
        // Global model cache to share across all streams
        static struct {
            char path[MAX_PATH_LENGTH];
            detection_model_t model;
            time_t last_used;
            bool is_large_model;
        } global_model_cache[MAX_STREAMS] = {{{0}}};
        
        // Find model in cache or load it
        detection_model_t model = NULL;
        int cache_idx = -1;
                
        // Look for model in global cache first
        for (int i = 0; i < MAX_STREAMS; i++) {
            if (global_model_cache[i].path[0] != '\0' && 
                strcmp(global_model_cache[i].path, full_model_path) == 0) {
                model = global_model_cache[i].model;
                cache_idx = i;
                global_model_cache[i].last_used = time(NULL);
                log_info("Using globally cached detection model for %s", full_model_path);
                break;
            }
        }        
        // If not found in global cache, check local cache
        if (!model) {
            // Look for model in local cache
            for (int i = 0; i < MAX_STREAMS; i++) {
                if (model_cache[i].path[0] != '\0' && 
                    strcmp(model_cache[i].path, full_model_path) == 0) {
                    model = model_cache[i].model;
                    cache_idx = i;
                    model_cache[i].last_used = time(NULL);
                    log_info("Using locally cached detection model for %s", full_model_path);
                    
                    // Also add to global cache for other streams to use
                    for (int j = 0; j < MAX_STREAMS; j++) {
                        if (global_model_cache[j].path[0] == '\0') {
                            strncpy(global_model_cache[j].path, full_model_path, MAX_PATH_LENGTH - 1);
                            global_model_cache[j].model = model;
                            global_model_cache[j].last_used = time(NULL);
                            log_info("Added model to global cache: %s", full_model_path);
                            break;
                        }
                    }
                    break;
                }
            }
        }
        
        // If not found in any cache, load it
        if (!model) {
            // Find an empty slot or the oldest used model
            time_t oldest_time = time(NULL);
            int oldest_idx = -1;
            
            for (int i = 0; i < MAX_STREAMS; i++) {
                if (model_cache[i].path[0] == '\0') {
                    oldest_idx = i;
                    break;
                } else if (model_cache[i].last_used < oldest_time) {
                    oldest_time = model_cache[i].last_used;
                    oldest_idx = i;
                }
            }
            
            // If we found a slot, load the model
            if (oldest_idx >= 0) {
                // If slot was used, unload the old model
                if (model_cache[oldest_idx].path[0] != '\0') {
                    log_info("Unloading cached model %s to make room for %s", 
                            model_cache[oldest_idx].path, full_model_path);
                    
                    // Check if this model is in the global cache before unloading
                    bool in_global_cache = false;
                    for (int i = 0; i < MAX_STREAMS; i++) {
                        if (global_model_cache[i].path[0] != '\0' && 
                            strcmp(global_model_cache[i].path, model_cache[oldest_idx].path) == 0) {
                            in_global_cache = true;
                            break;
                        }
                    }
                    
                    // Only unload if not in global cache
                    if (!in_global_cache) {
                        unload_detection_model(model_cache[oldest_idx].model);
                    } else {
                        log_info("Model %s is in global cache, not unloading", model_cache[oldest_idx].path);
                    }
                    
                    model_cache[oldest_idx].path[0] = '\0';
                    model_cache[oldest_idx].model = NULL;
                }
                
                // Check if model is already loaded in global cache
                for (int i = 0; i < MAX_STREAMS; i++) {
                    if (global_model_cache[i].path[0] != '\0' && 
                        strcmp(global_model_cache[i].path, full_model_path) == 0) {
                        model = global_model_cache[i].model;
                        global_model_cache[i].last_used = time(NULL);
                        log_info("Found model in global cache after slot search: %s", full_model_path);
                        
                        // Cache locally
                        strncpy(model_cache[oldest_idx].path, full_model_path, MAX_PATH_LENGTH - 1);
                        model_cache[oldest_idx].model = model;
                        model_cache[oldest_idx].last_used = time(NULL);
                        cache_idx = oldest_idx;
                        break;
                    }
                }
                
                // If still not found, load the model
                if (!model) {
                    // Load the new model using the SOD integration module
                    log_info("LOADING DETECTION MODEL: %s with threshold: %.2f", full_model_path, threshold);
                    
                    // Use the SOD integration function to load the model
                    char resolved_path[MAX_PATH_LENGTH];
                    model = load_sod_model_for_detection(stream_config.detection_model, threshold, 
                                                       resolved_path, MAX_PATH_LENGTH);
                    
                    if (model) {
                        // Update the full_model_path with the resolved path
                        strncpy(full_model_path, resolved_path, MAX_PATH_LENGTH - 1);
                    }
                    
                    if (model) {
                        // Cache the model locally
                        strncpy(model_cache[oldest_idx].path, full_model_path, MAX_PATH_LENGTH - 1);
                        model_cache[oldest_idx].model = model;
                        model_cache[oldest_idx].last_used = time(NULL);
                        cache_idx = oldest_idx;
                        log_info("Cached detection model for %s in local slot %d", full_model_path, oldest_idx);
                        
                        // Also add to global cache
                        for (int i = 0; i < MAX_STREAMS; i++) {
                            if (global_model_cache[i].path[0] == '\0') {
                                strncpy(global_model_cache[i].path, full_model_path, MAX_PATH_LENGTH - 1);
                                global_model_cache[i].model = model;
                                global_model_cache[i].last_used = time(NULL);
                                log_info("Added model to global cache: %s", full_model_path);
                                break;
                            }
                        }
                    }
                }
            }
        }
        
        if (!model) {
            log_error("Failed to load detection model: %s", full_model_path);
        } else {
            // Use our improved detect_objects function for ALL model types
            log_info("RUNNING DETECTION with unified detect_objects function");
            detect_ret = detect_objects(model, packed_buffer, target_width, target_height, channels, &result);
            
            if (detect_ret != 0) {
                log_error("Detection failed (error code: %d)", detect_ret);
            } else {
                log_info("DETECTION COMPLETED SUCCESSFULLY, found %d objects", result.count);
                
                // Log detection results
                for (int i = 0; i < result.count; i++) {
                    log_debug("DETECTION %d: %s (%.2f%%) at [%.2f, %.2f, %.2f, %.2f]",
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
            
            // Note: We don't unload the model here, it's kept in the cache
        }
    }

    // Decrement active detections counter
    active_detections--;
    
    // Return the detection result
    return detect_ret;

cleanup:
    // Free resources
    if (sws_ctx) {
        sws_freeContext(sws_ctx);
    }
    
    if (converted_frame) {
        av_frame_free(&converted_frame);
    }
    
    if (buffer) {
        av_free(buffer);
    }
    
    if (packed_buffer) {
        return_buffer_to_pool(packed_buffer);
    }
    
    // Decrement active detections counter
    active_detections--;
    
    return -1;
}

/**
 * Cleanup detection resources when shutting down
 */
void cleanup_detection_resources(void) {
    // Free active detection streams array
    if (active_detection_streams) {
        free(active_detection_streams);
        active_detection_streams = NULL;
    }
    
    // Cleanup buffer pool
    cleanup_buffer_pool();
    
    log_info("Detection resources cleaned up");
}

/**
 * Get the number of active detections
 */
int get_active_detection_count(void) {
    return active_detections;
}

/**
 * Get the maximum number of concurrent detections
 */
int get_max_detection_count(void) {
    return max_detections;
}
