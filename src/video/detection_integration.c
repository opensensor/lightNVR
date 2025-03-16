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

// Define model types
#define MODEL_TYPE_SOD "sod"
#define MODEL_TYPE_SOD_REALNET "sod_realnet"
#define MODEL_TYPE_TFLITE "tflite"

// Debug flag to enable/disable frame saving
static int save_frames_for_debug = 1;  // Set to 1 to enable frame saving

// Function to check if a file exists
static int file_exists(const char *filename) {
    FILE *file = fopen(filename, "r");
    if (file) {
        fclose(file);
        return 1;
    }
    return 0;
}

// Forward declaration of the process_frame_for_detection function
extern int process_frame_for_detection(const char *stream_name, const unsigned char *frame_data, 
                                     int width, int height, int channels, time_t frame_time);

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
 * Revised to use the unified detect_objects function for all model types
 *
 * @param stream_name The name of the stream
 * @param frame The decoded AVFrame
 * @param detection_interval How often to process frames (e.g., every 10 frames)
 * @return 0 on success, -1 on error
 */
int process_decoded_frame_for_detection(const char *stream_name, AVFrame *frame, int detection_interval) {
    static int frame_counters[MAX_STREAMS] = {0};
    static char stream_names[MAX_STREAMS][MAX_STREAM_NAME] = {{0}};

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
        log_error("Failed to get stream handle for %s", stream_name);
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

    // Convert frame to the appropriate format for detection
    struct SwsContext *sws_ctx = sws_getContext(
        frame->width, frame->height, frame->format,
        frame->width, frame->height, target_format,
        SWS_BILINEAR, NULL, NULL, NULL);

    if (!sws_ctx) {
        log_error("Failed to create SwsContext for stream %s", stream_name);
        return -1;
    }

    // Allocate converted frame
    AVFrame *converted_frame = av_frame_alloc();
    if (!converted_frame) {
        log_error("Failed to allocate converted frame for stream %s", stream_name);
        sws_freeContext(sws_ctx);
        return -1;
    }

    // Allocate buffer for converted frame - ensure it's large enough
    int buffer_size = av_image_get_buffer_size(target_format, frame->width, frame->height, 1);
    uint8_t *buffer = (uint8_t *)av_malloc(buffer_size);
    if (!buffer) {
        log_error("Failed to allocate buffer for converted frame for stream %s", stream_name);
        av_frame_free(&converted_frame);
        sws_freeContext(sws_ctx);
        return -1;
    }

    // Setup converted frame
    av_image_fill_arrays(converted_frame->data, converted_frame->linesize, buffer,
                        target_format, frame->width, frame->height, 1);

    // Convert frame to target format
    sws_scale(sws_ctx, (const uint8_t * const *)frame->data, frame->linesize, 0,
             frame->height, converted_frame->data, converted_frame->linesize);

    log_info("Converted frame to %s format for stream %s",
             (channels == 1) ? "grayscale" : "RGB", stream_name);

    // Create a packed buffer without stride padding
    uint8_t *packed_buffer = (uint8_t *)malloc(frame->width * frame->height * channels);
    if (!packed_buffer) {
        log_error("Failed to allocate packed buffer for frame");
        av_free(buffer);
        av_frame_free(&converted_frame);
        sws_freeContext(sws_ctx);
        return -1;
    }

    // Copy each row, removing stride padding
    for (int y = 0; y < frame->height; y++) {
        memcpy(packed_buffer + y * frame->width * channels,
               converted_frame->data[0] + y * converted_frame->linesize[0],
               frame->width * channels);
    }

    // Log some debug info about the packed buffer
    log_info("Packed buffer first 12 bytes: %02x %02x %02x %02x %02x %02x %02x %02x %02x %02x %02x %02x",
            packed_buffer[0], packed_buffer[1], packed_buffer[2], packed_buffer[3],
            packed_buffer[4], packed_buffer[5], packed_buffer[6], packed_buffer[7],
            packed_buffer[8], packed_buffer[9], packed_buffer[10], packed_buffer[11]);

    // Save frame to disk for debugging if enabled
    time_t frame_time = time(NULL);
    if (save_frames_for_debug) {
        // Debug code for saving frames
        log_info("DEBUG: Attempting to save frame to disk for stream %s", stream_name);

        // First, try to write a simple test file to verify permissions
        const char *test_file = "/tmp/lightnvr_debug_test.txt";
        FILE *tf = fopen(test_file, "w");
        if (!tf) {
            log_error("DEBUG: Cannot write to /tmp! Error: %s", strerror(errno));
        } else {
            fprintf(tf, "Debug test file from LightNVR\n");
            fclose(tf);
            log_info("DEBUG: Successfully wrote test file to %s", test_file);
        }

        // Save as JPEG for debug purposes
        char filename[256];
        snprintf(filename, sizeof(filename), "/tmp/frame_%s_%ld.jpg", stream_name, frame_time);
        log_info("DEBUG: Will try to save frame to %s", filename);

        // Save model info to a text file for reference
        char model_info[512];
        snprintf(model_info, sizeof(model_info), "/tmp/model_info_%s.txt", stream_name);
        FILE *mf = fopen(model_info, "w");
        if (mf) {
            fprintf(mf, "Model: %s\nType: %s\nChannels: %d\n",
                    config.detection_model,
                    detect_model_type(config.detection_model),
                    channels);
            fclose(mf);
            log_info("DEBUG: Saved model info to %s", model_info);
        }
    }

    // Get the appropriate threshold for the model type
    float threshold = 0.3f; // Default for CNN models
    if (strcmp(model_type, MODEL_TYPE_SOD_REALNET) == 0) {
        threshold = 5.0f; // RealNet models typically use 5.0
        log_info("Using threshold of 5.0 for RealNet model");
    } else {
        log_info("Using threshold of 0.3 for CNN model");
    }

    // Check if model_path is a relative path
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

    // -------------------------------------------------------------------------
    // UNIFIED DETECTION APPROACH - Use detect_objects for all model types
    // -------------------------------------------------------------------------
    // Load the detection model
    log_info("Loading detection model: %s with threshold: %.2f", full_model_path, threshold);
    detection_model_t model = load_detection_model(full_model_path, threshold);

    if (!model) {
        log_error("Failed to load detection model: %s", full_model_path);
    } else {
        // Use our improved detect_objects function for ALL model types
        // This function now properly handles face detection models with planar format conversion
        log_info("Running detection with unified detect_objects function");
        int detect_ret = detect_objects(model, packed_buffer, frame->width, frame->height, channels, &result);

        if (detect_ret != 0) {
            log_error("Detection failed (error code: %d)", detect_ret);
        } else {
            log_info("Detection completed successfully");
        }

        // Unload the model
        unload_detection_model(model);
    }

    // Process detection results
    if (result.count > 0) {
        log_info("Detection found %d objects", result.count);

        // Process results for notification/recording
        for (int i = 0; i < result.count; i++) {
            log_info("Detection %d: %s (%.2f%%) at [%.2f, %.2f, %.2f, %.2f]",
                    i+1, result.detections[i].label,
                    result.detections[i].confidence * 100.0f,
                    result.detections[i].x, result.detections[i].y,
                    result.detections[i].width, result.detections[i].height);
        }

        // Trigger recording
        log_info("Objects detected, triggering recording");
        process_frame_for_detection(stream_name, packed_buffer, frame->width, frame->height, channels, frame_time);
    } else {
        log_info("No objects detected");

        // Process frame anyway to handle object absence
        int ret = process_frame_for_detection(stream_name, packed_buffer, frame->width, frame->height, channels, frame_time);
        log_info("process_frame_for_detection returned: %d", ret);
    }

    // Cleanup
    free(packed_buffer);
    av_free(buffer);
    av_frame_free(&converted_frame);
    sws_freeContext(sws_ctx);

    log_info("Finished processing frame %d for detection", frame_counter);
    return 0;
}

/**
 * Process a video packet for detection
 * This function is kept for backward compatibility but now delegates to process_decoded_frame_for_detection
 * when a frame is available
 *
 * @param stream_name The name of the stream
 * @param pkt The AVPacket to process
 * @param stream The AVStream the packet belongs to
 * @param detection_interval How often to process frames (e.g., every 10 frames)
 * @return 0 on success, -1 on error
 */
int process_packet_for_detection(const char *stream_name, AVPacket *pkt, AVStream *stream, int detection_interval) {
    log_info("process_packet_for_detection called for stream %s - this function is deprecated", stream_name);
    log_info("Use process_decoded_frame_for_detection instead for better performance");

    // Get codec parameters
    AVCodecParameters *codecpar = stream->codecpar;
    if (codecpar->codec_type != AVMEDIA_TYPE_VIDEO) {
        log_error("Not a video stream: %s", stream_name);
        return -1;
    }

    // Find decoder
    AVCodec *codec = avcodec_find_decoder(codecpar->codec_id);
    if (!codec) {
        log_error("Failed to find decoder for stream %s", stream_name);
        return -1;
    }

    // Create codec context
    AVCodecContext *codec_ctx = avcodec_alloc_context3(codec);
    if (!codec_ctx) {
        log_error("Failed to allocate codec context for stream %s", stream_name);
        return -1;
    }

    // Copy codec parameters to codec context
    if (avcodec_parameters_to_context(codec_ctx, codecpar) < 0) {
        log_error("Failed to copy codec parameters to context for stream %s", stream_name);
        avcodec_free_context(&codec_ctx);
        return -1;
    }

    // Open codec
    if (avcodec_open2(codec_ctx, codec, NULL) < 0) {
        log_error("Failed to open codec for stream %s", stream_name);
        avcodec_free_context(&codec_ctx);
        return -1;
    }

    // Allocate frame
    AVFrame *frame = av_frame_alloc();
    if (!frame) {
        log_error("Failed to allocate frame for stream %s", stream_name);
        avcodec_free_context(&codec_ctx);
        return -1;
    }

    // Send packet to decoder
    int ret = avcodec_send_packet(codec_ctx, pkt);
    if (ret < 0) {
        log_error("Error sending packet to decoder for stream %s", stream_name);
        av_frame_free(&frame);
        avcodec_free_context(&codec_ctx);
        return -1;
    }

    // Receive frame from decoder
    ret = avcodec_receive_frame(codec_ctx, frame);
    if (ret < 0) {
        // Not an error, just no frames available yet
        log_info("No frames available yet for stream %s", stream_name);
        av_frame_free(&frame);
        avcodec_free_context(&codec_ctx);
        return 0;
    }

    // Process the decoded frame
    ret = process_decoded_frame_for_detection(stream_name, frame, detection_interval);

    // Cleanup
    av_frame_free(&frame);
    avcodec_free_context(&codec_ctx);

    return ret;
}
