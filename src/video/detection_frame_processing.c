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

// Include our modules
#include "video/detection_config.h"
#include "video/detection_buffer.h"
#include "video/detection_embedded.h"
#include "video/detection_integration.h"
#include "video/detection_frame_processing.h"
#include "video/detection_stream_thread.h"

// External declarations for functions used from detection_integration.c
extern int get_active_detection_count(void);
extern int get_max_detection_count(void);
extern bool is_detection_in_progress(const char *stream_name);

// External variables from detection_integration.c
extern char *active_detection_streams;
extern int active_detections;
extern bool use_stream_based_detection;

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
        log_debug("Detection not enabled for stream %s", stream_name);
        return 0;
    }

    // Check if a detection thread is already running for this stream
    if (!is_stream_detection_thread_running(stream_name)) {
        // Get the HLS directory for this stream using the config
        char hls_dir[MAX_PATH_LENGTH];
        config_t *global_config = get_streaming_config();
        
        // Use storage_path_hls if specified, otherwise fall back to storage_path
        const char *base_storage_path = global_config->storage_path;
        if (global_config->storage_path_hls && global_config->storage_path_hls[0] != '\0') {
            base_storage_path = global_config->storage_path_hls;
            log_info("Using dedicated HLS storage path: %s", base_storage_path);
        }
        
        // Use standard HLS directory path
        log_info("Using standard HLS directory path: %s/hls/%s", base_storage_path, stream_name);
        snprintf(hls_dir, MAX_PATH_LENGTH, "%s/hls/%s", base_storage_path, stream_name);
        
        // Create the HLS directory if it doesn't exist
        struct stat st = {0};
        if (stat(hls_dir, &st) == -1) {
            mkdir(hls_dir, 0755);
        }
        
        // Start a detection thread for this stream
        float threshold = stream_config.detection_threshold;
        if (threshold <= 0.0f) {
            threshold = 0.5f; // Default threshold
        }
        
        log_info("Starting detection thread for stream %s with model %s", 
                stream_name, stream_config.detection_model);
        
        // Get global config to access models path
        extern config_t g_config;
        
        // Check if model_path is a relative path
        char full_model_path[MAX_PATH_LENGTH];
        if (stream_config.detection_model[0] != '/') {
                // Construct full path using configured models path from INI if it exists
                if (g_config.models_path && strlen(g_config.models_path) > 0) {
                    snprintf(full_model_path, MAX_PATH_LENGTH, "%s/%s", 
                            g_config.models_path, stream_config.detection_model);
                } else {
                    // Fall back to default path if INI config doesn't exist
                    snprintf(full_model_path, MAX_PATH_LENGTH, "/etc/lightnvr/models/%s", 
                            stream_config.detection_model);
                }
        } else {
            // Already an absolute path
            strncpy(full_model_path, stream_config.detection_model, MAX_PATH_LENGTH - 1);
            full_model_path[MAX_PATH_LENGTH - 1] = '\0';
        }
        
        start_stream_detection_thread(stream_name, full_model_path, threshold, 
                                     detection_interval, hls_dir);
    }
    
    // Check if the stream detection thread is running
    if (is_stream_detection_thread_running(stream_name)) {
        log_info("Stream detection thread is running for %s, using dedicated thread", stream_name);
        
        // Get current time
        time_t current_time = time(NULL);
        
        // Convert frame to RGB format
        enum AVPixelFormat target_format = AV_PIX_FMT_RGB24;
        int channels = 3; // RGB
        
        // Determine model type to use the correct image format
        const char *model_type = detect_model_type(stream_config.detection_model);
        if (strcmp(model_type, MODEL_TYPE_SOD_REALNET) == 0) {
            target_format = AV_PIX_FMT_GRAY8;
            channels = 1;
            log_info("Using grayscale format for RealNet model");
        }
        
        // Check if we're running on an embedded device
        bool is_embedded = is_embedded_device();
        
        // Determine if we should downscale the frame based on model type and device
        int downscale_factor = get_downscale_factor(model_type);
        
        // Calculate dimensions after downscaling
        int target_width = frame->width / downscale_factor;
        int target_height = frame->height / downscale_factor;
        
        // Ensure dimensions are even (required by some codecs)
        target_width = (target_width / 2) * 2;
        target_height = (target_height / 2) * 2;
        
        // Convert frame to the appropriate format for detection with downscaling if needed
        struct SwsContext *sws_ctx = sws_getContext(
            frame->width, frame->height, frame->format,
            target_width, target_height, target_format,
            SWS_BILINEAR, NULL, NULL, NULL);
            
        if (!sws_ctx) {
            log_error("Failed to create SwsContext for stream %s", stream_name);
            return -1;
        }
        
        // Allocate buffer for converted frame
        uint8_t *buffer = (uint8_t *)malloc(target_width * target_height * channels);
        if (!buffer) {
            log_error("Failed to allocate buffer for converted frame for stream %s", stream_name);
            sws_freeContext(sws_ctx);
            return -1;
        }
        
        // Setup RGB frame
        uint8_t *rgb_data[4] = {buffer, NULL, NULL, NULL};
        int rgb_linesize[4] = {target_width * channels, 0, 0, 0};
        
        // Convert frame to RGB
        sws_scale(sws_ctx, (const uint8_t * const *)frame->data, frame->linesize, 0,
                 frame->height, rgb_data, rgb_linesize);
        
        // Process the frame using the stream detection thread
        int ret = process_frame_for_stream_detection(stream_name, buffer, 
                                                   target_width, target_height, 
                                                   channels, current_time);
        
        // Free resources
        free(buffer);
        sws_freeContext(sws_ctx);
        
        // Return the result from the stream detection thread
        return ret;
    } else {
        log_info("No stream detection thread running for %s, starting one", stream_name);
        
        // Get the HLS directory for this stream using the config
        char hls_dir[MAX_PATH_LENGTH];
        config_t *global_config = get_streaming_config();
        
        // Use storage_path_hls if specified, otherwise fall back to storage_path
        const char *base_storage_path = global_config->storage_path;
        if (global_config->storage_path_hls && global_config->storage_path_hls[0] != '\0') {
            base_storage_path = global_config->storage_path_hls;
            log_info("Using dedicated HLS storage path: %s", base_storage_path);
        }
        
        // Use standard HLS directory path
        log_info("Using standard HLS directory path: %s/hls/%s", base_storage_path, stream_name);
        snprintf(hls_dir, MAX_PATH_LENGTH, "%s/hls/%s", base_storage_path, stream_name);
        
        // Create the HLS directory if it doesn't exist
        struct stat st = {0};
        if (stat(hls_dir, &st) == -1) {
            mkdir(hls_dir, 0755);
        }
        
        // Start a detection thread for this stream
        float threshold = stream_config.detection_threshold;
        if (threshold <= 0.0f) {
            threshold = 0.5f; // Default threshold
        }
        
        log_info("Starting detection thread for stream %s with model %s", 
                stream_name, stream_config.detection_model);
        
        // Get global config to access models path
        extern config_t g_config;
        
        // Check if model_path is a relative path
        char full_model_path[MAX_PATH_LENGTH];
        if (stream_config.detection_model[0] != '/') {
                // Construct full path using configured models path from INI if it exists
                if (g_config.models_path && strlen(g_config.models_path) > 0) {
                    snprintf(full_model_path, MAX_PATH_LENGTH, "%s/%s", 
                            g_config.models_path, stream_config.detection_model);
                } else {
                    // Fall back to default path if INI config doesn't exist
                    snprintf(full_model_path, MAX_PATH_LENGTH, "/etc/lightnvr/models/%s", 
                            stream_config.detection_model);
                }
        } else {
            // Already an absolute path
            strncpy(full_model_path, stream_config.detection_model, MAX_PATH_LENGTH - 1);
            full_model_path[MAX_PATH_LENGTH - 1] = '\0';
        }
        
        // Try to start the detection thread
        if (start_stream_detection_thread(stream_name, full_model_path, threshold, 
                                         detection_interval, hls_dir) == 0) {
            log_info("Successfully started detection thread for stream %s", stream_name);
            
            // Process this frame with the new thread
            // We'll need to convert the frame again
            time_t current_time = time(NULL);
            
            // Convert frame to RGB format
            enum AVPixelFormat target_format = AV_PIX_FMT_RGB24;
            int channels = 3; // RGB
            
            // Determine model type to use the correct image format
            const char *model_type = detect_model_type(stream_config.detection_model);
            if (strcmp(model_type, MODEL_TYPE_SOD_REALNET) == 0) {
                target_format = AV_PIX_FMT_GRAY8;
                channels = 1;
            }
            
            // Determine if we should downscale the frame based on model type
            int downscale_factor = get_downscale_factor(model_type);
            
            // Calculate dimensions after downscaling
            int target_width = frame->width / downscale_factor;
            int target_height = frame->height / downscale_factor;
            
            // Ensure dimensions are even (required by some codecs)
            target_width = (target_width / 2) * 2;
            target_height = (target_height / 2) * 2;
            
            // Convert frame to the appropriate format for detection with downscaling if needed
            struct SwsContext *sws_ctx = sws_getContext(
                frame->width, frame->height, frame->format,
                target_width, target_height, target_format,
                SWS_BILINEAR, NULL, NULL, NULL);
                
            if (!sws_ctx) {
                log_error("Failed to create SwsContext for stream %s", stream_name);
                return -1;
            }
            
            // Allocate buffer for converted frame
            uint8_t *buffer = (uint8_t *)malloc(target_width * target_height * channels);
            if (!buffer) {
                log_error("Failed to allocate buffer for converted frame for stream %s", stream_name);
                sws_freeContext(sws_ctx);
                return -1;
            }
            
            // Setup RGB frame
            uint8_t *rgb_data[4] = {buffer, NULL, NULL, NULL};
            int rgb_linesize[4] = {target_width * channels, 0, 0, 0};
            
            // Convert frame to RGB
            sws_scale(sws_ctx, (const uint8_t * const *)frame->data, frame->linesize, 0,
                     frame->height, rgb_data, rgb_linesize);
            
            // Process the frame using the stream detection thread
            int ret = process_frame_for_stream_detection(stream_name, buffer, 
                                                       target_width, target_height, 
                                                       channels, current_time);
            
            // Free resources
            free(buffer);
            sws_freeContext(sws_ctx);
            
            // Return the result from the stream detection thread
            return ret;
        } else {
            log_error("Failed to start detection thread for stream %s", stream_name);
            return -1;
        }
    }
    
    // If we get here, we couldn't use or start a stream detection thread
    log_error("No stream detection thread available for %s, skipping detection", stream_name);
    return -1;
}
