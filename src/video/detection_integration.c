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
    // For CNN models, we need RGB images (not BGR)
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

    // Process frame for detection
    time_t frame_time = time(NULL);
    log_info("Calling process_frame_for_detection for stream %s", stream_name);

    // Debug: Check the first few bytes of the packed buffer
    if (channels == 3) {
        log_info("Packed buffer first 12 bytes: %02x %02x %02x %02x %02x %02x %02x %02x %02x %02x %02x %02x",
                packed_buffer[0], packed_buffer[1], packed_buffer[2], packed_buffer[3],
                packed_buffer[4], packed_buffer[5], packed_buffer[6], packed_buffer[7],
                packed_buffer[8], packed_buffer[9], packed_buffer[10], packed_buffer[11]);
    } else {
        log_info("Packed buffer first 12 bytes (grayscale): %02x %02x %02x %02x %02x %02x %02x %02x %02x %02x %02x %02x",
                packed_buffer[0], packed_buffer[1], packed_buffer[2], packed_buffer[3],
                packed_buffer[4], packed_buffer[5], packed_buffer[6], packed_buffer[7],
                packed_buffer[8], packed_buffer[9], packed_buffer[10], packed_buffer[11]);
    }

    // Save frame to disk for debugging if enabled
    if (save_frames_for_debug) {
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
        
        // Save as JPEG
        char filename[256];
        snprintf(filename, sizeof(filename), "/tmp/frame_%s_%ld.jpg", stream_name, frame_time);
        log_info("DEBUG: Will try to save frame to %s", filename);
        
        // Create a new AVFrame for RGB format (needed for JPEG encoding)
        AVFrame *rgb_frame = av_frame_alloc();
        if (!rgb_frame) {
            log_error("DEBUG: Failed to allocate RGB frame for debug image");
        } else {
            // Allocate buffer for RGB frame
            int rgb_buffer_size = av_image_get_buffer_size(AV_PIX_FMT_RGB24, frame->width, frame->height, 1);
            uint8_t *rgb_buffer = (uint8_t *)av_malloc(rgb_buffer_size);
            if (!rgb_buffer) {
                log_error("DEBUG: Failed to allocate RGB buffer for debug image");
                av_frame_free(&rgb_frame);
            } else {
                // Setup RGB frame
                av_image_fill_arrays(rgb_frame->data, rgb_frame->linesize, rgb_buffer,
                                    AV_PIX_FMT_RGB24, frame->width, frame->height, 1);
                
                // Convert to RGB if needed
                if (channels == 1) {
                    // Convert grayscale to RGB
                    struct SwsContext *gray_to_rgb = sws_getContext(
                        frame->width, frame->height, AV_PIX_FMT_GRAY8,
                        frame->width, frame->height, AV_PIX_FMT_RGB24,
                        SWS_BILINEAR, NULL, NULL, NULL);
                    
                    if (gray_to_rgb) {
                        const uint8_t *src_data[4] = { packed_buffer, NULL, NULL, NULL };
                        int src_linesize[4] = { frame->width, 0, 0, 0 };
                        
                        sws_scale(gray_to_rgb, src_data, src_linesize, 0,
                                 frame->height, rgb_frame->data, rgb_frame->linesize);
                        
                        sws_freeContext(gray_to_rgb);
                    } else {
                        log_error("DEBUG: Failed to create grayscale to RGB conversion context");
                    }
                } else {
                    // Copy RGB data directly
                    for (int y = 0; y < frame->height; y++) {
                        memcpy(rgb_frame->data[0] + y * rgb_frame->linesize[0],
                               packed_buffer + y * frame->width * 3,
                               frame->width * 3);
                    }
                }
                
                // Find JPEG encoder
                const AVCodec *jpeg_codec = avcodec_find_encoder(AV_CODEC_ID_MJPEG);
                if (!jpeg_codec) {
                    log_error("DEBUG: JPEG codec not found");
                    
                    // Try to find the codec by name as a fallback
                    jpeg_codec = avcodec_find_encoder_by_name("mjpeg");
                    if (!jpeg_codec) {
                        log_error("DEBUG: JPEG codec not found by name either");
                    } else {
                        log_info("DEBUG: Found JPEG codec by name");
                    }
                }
                
                if (jpeg_codec) {
                    // Create codec context
                    AVCodecContext *jpeg_ctx = avcodec_alloc_context3(jpeg_codec);
                    if (!jpeg_ctx) {
                        log_error("DEBUG: Failed to allocate JPEG codec context");
                    } else {
                        // Set codec parameters
                        jpeg_ctx->width = frame->width;
                        jpeg_ctx->height = frame->height;
                        jpeg_ctx->pix_fmt = AV_PIX_FMT_YUVJ420P;  // Use YUVJ420P which is better supported
                        jpeg_ctx->time_base.num = 1;
                        jpeg_ctx->time_base.den = 25;
                        jpeg_ctx->strict_std_compliance = FF_COMPLIANCE_VERY_STRICT;
                        
                        // Set JPEG quality (1-31, lower is higher quality)
                        av_opt_set_int(jpeg_ctx, "qscale", 3, 0);  // High quality
                        
                        // Check if the pixel format is supported
                        if (jpeg_codec->pix_fmts) {
                            const enum AVPixelFormat *p = jpeg_codec->pix_fmts;
                            bool format_supported = false;
                            
                            log_info("DEBUG: Supported pixel formats for JPEG codec:");
                            while (*p != AV_PIX_FMT_NONE) {
                                log_info("DEBUG:   - %s", av_get_pix_fmt_name(*p));
                                if (*p == jpeg_ctx->pix_fmt) {
                                    format_supported = true;
                                }
                                p++;
                            }
                            
                            if (!format_supported) {
                                log_warn("DEBUG: Pixel format %s not supported by JPEG codec, trying YUVJ420P", 
                                        av_get_pix_fmt_name(jpeg_ctx->pix_fmt));
                                jpeg_ctx->pix_fmt = AV_PIX_FMT_YUVJ420P;
                            }
                        }
                        
                        // Convert RGB frame to the format needed by the JPEG encoder
                        AVFrame *yuv_frame = av_frame_alloc();
                        if (!yuv_frame) {
                            log_error("DEBUG: Failed to allocate YUV frame");
                        } else {
                            // Allocate buffer for YUV frame
                            int yuv_buffer_size = av_image_get_buffer_size(jpeg_ctx->pix_fmt, frame->width, frame->height, 1);
                            uint8_t *yuv_buffer = (uint8_t *)av_malloc(yuv_buffer_size);
                            if (!yuv_buffer) {
                                log_error("DEBUG: Failed to allocate YUV buffer");
                                av_frame_free(&yuv_frame);
                            } else {
                                // Setup YUV frame
                                av_image_fill_arrays(yuv_frame->data, yuv_frame->linesize, yuv_buffer,
                                                   jpeg_ctx->pix_fmt, frame->width, frame->height, 1);
                                
                                // Convert RGB to YUV
                                struct SwsContext *rgb_to_yuv = sws_getContext(
                                    frame->width, frame->height, AV_PIX_FMT_RGB24,
                                    frame->width, frame->height, jpeg_ctx->pix_fmt,
                                    SWS_BILINEAR, NULL, NULL, NULL);
                                
                                if (!rgb_to_yuv) {
                                    log_error("DEBUG: Failed to create RGB to YUV conversion context");
                                    av_free(yuv_buffer);
                                    av_frame_free(&yuv_frame);
                                } else {
                                    // Convert RGB to YUV
                                    sws_scale(rgb_to_yuv, (const uint8_t * const *)rgb_frame->data, rgb_frame->linesize, 0,
                                             frame->height, yuv_frame->data, yuv_frame->linesize);
                                    
                                    sws_freeContext(rgb_to_yuv);
                                    
                                    // Set frame properties
                                    yuv_frame->pts = 0;
                                    yuv_frame->width = frame->width;
                                    yuv_frame->height = frame->height;
                                    yuv_frame->format = jpeg_ctx->pix_fmt;
                                    
                                    // Open codec
                                    int ret = avcodec_open2(jpeg_ctx, jpeg_codec, NULL);
                                    if (ret < 0) {
                                        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
                                        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
                                        log_error("DEBUG: Failed to open JPEG codec: %s", error_buf);
                                    } else {
                                        // Allocate packet
                                        AVPacket *pkt = av_packet_alloc();
                                        if (!pkt) {
                                            log_error("DEBUG: Failed to allocate packet");
                                        } else {
                                            // Send frame to encoder
                                            ret = avcodec_send_frame(jpeg_ctx, yuv_frame);
                                            if (ret < 0) {
                                                char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
                                                av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
                                                log_error("DEBUG: Error sending frame to JPEG encoder: %s", error_buf);
                                            } else {
                                                // Receive encoded packet
                                                ret = avcodec_receive_packet(jpeg_ctx, pkt);
                                                if (ret < 0) {
                                                    char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
                                                    av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
                                                    log_error("DEBUG: Error receiving packet from JPEG encoder: %s", error_buf);
                                                } else {
                                                    // Write JPEG to file
                                                    FILE *f = fopen(filename, "wb");
                                                    if (!f) {
                                                        log_error("DEBUG: Failed to open debug image file: %s (Error: %s)", 
                                                                 filename, strerror(errno));
                                                    } else {
                                                        size_t bytes_written = fwrite(pkt->data, 1, pkt->size, f);
                                                        fclose(f);
                                                        
                                                        if (bytes_written != pkt->size) {
                                                            log_error("DEBUG: Failed to write all data. Wrote %zu of %d bytes", 
                                                                     bytes_written, pkt->size);
                                                        } else {
                                                            log_info("DEBUG: Saved frame to %s (%d bytes)", filename, pkt->size);
                                                            
                                                            // Verify the file was actually created
                                                            if (file_exists(filename)) {
                                                                log_info("DEBUG: Verified file exists: %s", filename);
                                                            } else {
                                                                log_error("DEBUG: File was not created despite successful write: %s", filename);
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            
                                            // Free packet
                                            av_packet_free(&pkt);
                                        }
                                    }
                                    
                                    // Free YUV buffer
                                    av_free(yuv_buffer);
                                }
                                
                                // Free YUV frame
                                av_frame_free(&yuv_frame);
                            }
                        }
                        // Close codec
                        avcodec_close(jpeg_ctx);
                        
                        // Free codec context
                        avcodec_free_context(&jpeg_ctx);
                    }
                } else {
                    // If JPEG codec is not available, try using system command as a last resort
                    log_warn("DEBUG: JPEG codec not available, trying to use system command");
                    
                    // Save RGB data to a temporary PPM file
                    char temp_ppm[256];
                    snprintf(temp_ppm, sizeof(temp_ppm), "/tmp/temp_%s_%ld.ppm", stream_name, frame_time);
                    
                    FILE *f = fopen(temp_ppm, "wb");
                    if (f) {
                        // Write PPM header
                        fprintf(f, "P6\n%d %d\n255\n", frame->width, frame->height);
                        
                        // Write RGB data
                        fwrite(rgb_frame->data[0], 1, frame->width * frame->height * 3, f);
                        fclose(f);
                        
                        // Convert PPM to JPEG using system command
                        char cmd[512];
                        snprintf(cmd, sizeof(cmd), "convert %s %s && rm %s", temp_ppm, filename, temp_ppm);
                        int ret = system(cmd);
                        
                        if (ret == 0) {
                            log_info("DEBUG: Converted PPM to JPEG using system command");
                        } else {
                            log_error("DEBUG: Failed to convert PPM to JPEG using system command: %d", ret);
                        }
                    } else {
                        log_error("DEBUG: Failed to open temporary PPM file: %s", temp_ppm);
                    }
                }
                
                // Free RGB buffer
                av_free(rgb_buffer);
            }
            
            // Free RGB frame
            av_frame_free(&rgb_frame);
        }
        
        // Also save model path to a text file for reference
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
        } else {
            log_error("DEBUG: Failed to open model info file: %s (Error: %s)", 
                     model_info, strerror(errno));
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
    
    // Directly run detection on the packed buffer with the correct threshold
    log_info("Running direct detection on packed buffer with threshold %.1f", threshold);
    
    // Create a detection result structure
    detection_result_t result;
    memset(&result, 0, sizeof(detection_result_t));
    
    // Check if model_path is a relative path (doesn't start with /)
    char full_model_path[MAX_PATH_LENGTH];
    if (config.detection_model[0] != '/') {
        // Get current working directory
        char cwd[MAX_PATH_LENGTH];
        if (getcwd(cwd, sizeof(cwd)) != NULL) {
            // Construct full path: CWD/models/model_path
            snprintf(full_model_path, MAX_PATH_LENGTH, "%s/models/%s", cwd, config.detection_model);
            
            // Check if the models directory exists
            char models_dir[MAX_PATH_LENGTH];
            snprintf(models_dir, MAX_PATH_LENGTH, "%s/models", cwd);
            DIR *dir = opendir(models_dir);
            if (dir) {
                log_info("Models directory exists: %s", models_dir);
                closedir(dir);
                
                // List files in the models directory
                log_info("Listing files in models directory:");
                dir = opendir(models_dir);
                if (dir) {
                    struct dirent *entry;
                    while ((entry = readdir(dir)) != NULL) {
                        if (entry->d_type == DT_REG) { // Regular file
                            log_info("  - %s", entry->d_name);
                        }
                    }
                    closedir(dir);
                }
            } else {
                log_error("Models directory does not exist: %s (Error: %s)", models_dir, strerror(errno));
                
                // Try to create the models directory
                if (mkdir(models_dir, 0755) == 0) {
                    log_info("Created models directory: %s", models_dir);
                } else {
                    log_error("Failed to create models directory: %s (Error: %s)", models_dir, strerror(errno));
                }
            }
        } else {
            // Fallback to absolute path if getcwd fails
            snprintf(full_model_path, MAX_PATH_LENGTH, "/var/lib/lightnvr/models/%s", config.detection_model);
        }
        log_info("Using full model path: %s", full_model_path);
    } else {
        // Already an absolute path
        strncpy(full_model_path, config.detection_model, MAX_PATH_LENGTH - 1);
        full_model_path[MAX_PATH_LENGTH - 1] = '\0';
    }
    
    // Check if the model file exists
    if (file_exists(full_model_path)) {
        log_info("Model file exists: %s", full_model_path);
    } else {
        log_error("Model file does not exist: %s", full_model_path);
        
        // Try to find the model in other locations
        char alt_path[MAX_PATH_LENGTH];
        
        // Try in the current directory
        snprintf(alt_path, MAX_PATH_LENGTH, "./%s", config.detection_model);
        if (file_exists(alt_path)) {
            log_info("Found model in current directory: %s", alt_path);
            strncpy(full_model_path, alt_path, MAX_PATH_LENGTH - 1);
        } else {
            // Try in the build directory
            snprintf(alt_path, MAX_PATH_LENGTH, "./build/models/%s", config.detection_model);
            if (file_exists(alt_path)) {
                log_info("Found model in build directory: %s", alt_path);
                strncpy(full_model_path, alt_path, MAX_PATH_LENGTH - 1);
            } else {
                // Try in the parent directory
                snprintf(alt_path, MAX_PATH_LENGTH, "../models/%s", config.detection_model);
                if (file_exists(alt_path)) {
                    log_info("Found model in parent directory: %s", alt_path);
                    strncpy(full_model_path, alt_path, MAX_PATH_LENGTH - 1);
                }
            }
        }
    }
    
    // Load the detection model with the correct threshold
    detection_model_t model = load_detection_model(full_model_path, threshold);
    if (!model) {
        log_error("Failed to load detection model: %s", full_model_path);
    } else {
        // Run detection on the packed buffer
        int detect_ret = detect_objects(model, packed_buffer, frame->width, frame->height, channels, &result);
        
        if (detect_ret == 0) {
            log_info("Direct detection found %d objects", result.count);
            
            // Process results
            for (int i = 0; i < result.count; i++) {
                log_info("Detection %d: %s (%.2f%%) at [%.2f, %.2f, %.2f, %.2f]",
                        i+1, result.detections[i].label,
                        result.detections[i].confidence * 100.0f,
                        result.detections[i].x, result.detections[i].y,
                        result.detections[i].width, result.detections[i].height);
            }
            
            // If objects were detected, trigger recording
            if (result.count > 0) {
                log_info("Objects detected, triggering recording");
                process_frame_for_detection(stream_name, packed_buffer, frame->width, frame->height, channels, frame_time);
            } else {
                log_info("No objects detected");
            }
        } else {
            log_error("Direct detection failed (error code: %d)", detect_ret);
            
            // Fall back to using the standard detection method
            log_info("Falling back to standard detection method");
            int ret = process_frame_for_detection(stream_name, packed_buffer, frame->width, frame->height, channels, frame_time);
            log_info("process_frame_for_detection returned: %d", ret);
        }
        
        // Unload the model
        unload_detection_model(model);
    }
    
    // Clean up any saved JPEG files
    char jpeg_filename[256];
    snprintf(jpeg_filename, sizeof(jpeg_filename), "/tmp/frame_%s_%ld.jpg", stream_name, frame_time);
    
    // Remove the JPEG file if it exists (we don't need it anymore)
    if (file_exists(jpeg_filename)) {
        if (remove(jpeg_filename) == 0) {
            log_info("Removed JPEG file %s after detection", jpeg_filename);
        } else {
            log_warn("Failed to remove JPEG file %s: %s", jpeg_filename, strerror(errno));
        }
    }

    // Free the packed buffer
    free(packed_buffer);

    // Cleanup
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
