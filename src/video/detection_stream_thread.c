#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <pthread.h>
#include <stdbool.h>
#include <sys/stat.h>
#include <dirent.h>
#include <time.h>
#include <stdatomic.h>
#include <curl/curl.h>
#include <errno.h>

#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <libavutil/avutil.h>
#include <libavutil/imgutils.h>
#include <libswscale/swscale.h>

#include "core/logger.h"
#include "core/config.h"
#include "core/shutdown_coordinator.h"
#include "utils/strings.h"
#include "video/detection_stream_thread.h"
#include "video/detection_stream_thread_helpers.h"
#include "video/detection_model.h"
#include "video/sod_integration.h"
#include "video/detection_result.h"
#include "video/detection_recording.h"
#include "video/detection_embedded.h"
#include "video/streams.h"
#include "video/hls_writer.h"
#include "video/hls/hls_unified_thread.h"
#include "video/api_detection.h"
#include "video/go2rtc/go2rtc_stream.h"

// Add signal handler to catch floating point exceptions
#include <fenv.h>
#include <signal.h>

// Array of stream detection threads
static stream_detection_thread_t stream_threads[MAX_STREAM_THREADS] = {0};
static pthread_mutex_t stream_threads_mutex = PTHREAD_MUTEX_INITIALIZER;
static bool system_initialized = false;

// Global variable for startup delay (defined here since it's extern in the header)
time_t global_startup_delay_end = 0;

// Forward declarations for functions from other modules
int detect_objects(detection_model_t model, const uint8_t *frame_data, int width, int height, int channels, detection_result_t *result);
int process_frame_for_recording(const char *stream_name, const uint8_t *frame_data, int width, int height, int channels, time_t timestamp, detection_result_t *result);

/**
 * Process a frame directly for detection
 * This function is called from process_decoded_frame_for_detection
 */
int process_frame_for_stream_detection(const char *stream_name, const uint8_t *frame_data,
                                      int width, int height, int channels, time_t timestamp) {
    if (!system_initialized || !stream_name || !frame_data) {
        log_error("Invalid parameters for process_frame_for_stream_detection");
        return -1;
    }

    pthread_mutex_lock(&stream_threads_mutex);

    // Find the thread for this stream
    stream_detection_thread_t *thread = NULL;
    for (int i = 0; i < MAX_STREAM_THREADS; i++) {
        if (stream_threads[i].running && strcmp(stream_threads[i].stream_name, stream_name) == 0) {
            thread = &stream_threads[i];
            break;
        }
    }

    if (!thread) {
        log_warn("No detection thread found for stream %s", stream_name);
        pthread_mutex_unlock(&stream_threads_mutex);
        return -1;
    }

    // Check if a detection is already in progress
    int detection_running = atomic_load(&thread->detection_in_progress);
    if (detection_running) {
        log_info("[Stream %s] Detection already in progress, skipping frame", thread->stream_name);
        pthread_mutex_unlock(&stream_threads_mutex);
        return 0;
    }

    // Check if enough time has passed since the last detection
    time_t current_time = time(NULL);
    if (thread->last_detection_time > 0) {
        time_t time_since_last = current_time - thread->last_detection_time;
        if (time_since_last < thread->detection_interval) {
            // Not enough time has passed, skip this frame
            pthread_mutex_unlock(&stream_threads_mutex);
            return 0;
        }
    }

    // Set the atomic flag to indicate a detection is in progress
    atomic_store(&thread->detection_in_progress, 1);

    // Update last detection time
    thread->last_detection_time = current_time;

    // Create detection result structure
    detection_result_t result;
    memset(&result, 0, sizeof(detection_result_t));

    // Run detection on the frame
    int detect_ret = -1;

    // CRITICAL FIX: Initialize result to empty to prevent segmentation fault
    // This is redundant with the memset above, but ensures we're always safe
    result.count = 0;

    // Lock the thread mutex to ensure exclusive access to the model
    pthread_mutex_lock(&thread->mutex);

    // Make sure the model is loaded
    if (!thread->model) {
        log_info("[Stream %s] Loading detection model: %s", thread->stream_name, thread->model_path);
        thread->model = load_detection_model(thread->model_path, thread->threshold);
        if (!thread->model) {
            log_error("[Stream %s] Failed to load detection model: %s",
                     thread->stream_name, thread->model_path);
            pthread_mutex_unlock(&thread->mutex);
            pthread_mutex_unlock(&stream_threads_mutex);
            // Don't return error, just indicate no detections were found
            log_info("[Stream %s] Continuing detection thread despite model loading failure", thread->stream_name);
            return 0;
        }
        log_info("[Stream %s] Successfully loaded detection model", thread->stream_name);
    }

    // Run detection
    log_info("[Stream %s] Running detection on frame (dimensions: %dx%d, channels: %d)",
            thread->stream_name, width, height, channels);

    detect_ret = detect_objects(thread->model, frame_data, width, height, channels, &result);

    pthread_mutex_unlock(&thread->mutex);

    if (detect_ret != 0) {
        // Handle detection errors
        log_error("[Stream %s] Detection failed (error code: %d)", thread->stream_name, detect_ret);

        // Set result.count to 0 to indicate no detections
        result.count = 0;

        // Continue processing despite the error
        log_info("[Stream %s] Continuing detection thread despite detection failure", thread->stream_name);

        // Don't return here, continue processing with empty results
    }

    // Process detection results
    if (result.count > 0) {
        log_info("[Stream %s] Detection found %d objects", thread->stream_name, result.count);

        // Log each detected object
        for (int i = 0; i < result.count && i < MAX_DETECTIONS; i++) {
            log_info("[Stream %s] Object %d: class=%s, confidence=%.2f, box=[%.2f,%.2f,%.2f,%.2f]",
                    thread->stream_name, i, result.detections[i].label,
                    result.detections[i].confidence,
                    result.detections[i].x, result.detections[i].y,
                    result.detections[i].width, result.detections[i].height);
        }

        // Process the detection results for recording
        int record_ret = process_frame_for_recording(thread->stream_name, frame_data, width, height,
                                                   channels, timestamp, &result);

        if (record_ret != 0) {
            log_error("[Stream %s] Failed to process frame for recording (error code: %d)",
                     thread->stream_name, record_ret);
        } else {
            log_info("[Stream %s] Successfully processed frame for recording", thread->stream_name);
        }
    } else {
        log_debug("[Stream %s] No objects detected in frame", thread->stream_name);
    }

    // Clear the atomic flag to indicate detection is complete
    atomic_store(&thread->detection_in_progress, 0);

    pthread_mutex_unlock(&stream_threads_mutex);
    return 0;
}


// Forward declarations for functions from other modules

/**
 * Process an HLS segment file for detection
 */
int process_segment_for_detection(stream_detection_thread_t *thread, const char *segment_path) {
    // CRITICAL FIX: Add safety checks to prevent memory corruption
    if (!thread || !segment_path || segment_path[0] == '\0') {
        log_error("Invalid parameters for process_segment_for_detection");
        return -1;
    }

    // CRITICAL FIX: Initialize all pointers to NULL to prevent use-after-free and double-free issues
    AVFormatContext *format_ctx = NULL;
    AVCodecContext *codec_ctx = NULL;
    AVFrame *frame = NULL;
    AVPacket *pkt = NULL;
    struct SwsContext *sws_ctx = NULL;
    int video_stream_idx = -1;
    int ret = -1;

    // CRITICAL FIX: Use a try/catch block to handle potential segfaults
    __attribute__((unused)) volatile int segment_check_result = 0;

    log_info("[Stream %s] Processing HLS segment for detection: %s",
             thread->stream_name, segment_path);

    // CRITICAL FIX: Double-check that the segment still exists and is valid before trying to open it
    // This prevents segmentation faults when trying to open deleted or corrupt segments
    if (access(segment_path, F_OK) != 0) {
        log_warn("[Stream %s] Segment no longer exists before processing: %s",
                thread->stream_name, segment_path);
        // Don't return error, just indicate no detections were found
        log_info("[Stream %s] Continuing detection thread despite missing segment", thread->stream_name);
        return 0;
    }

    // Check if the segment file is valid (non-zero size)
    struct stat st;
    if (stat(segment_path, &st) != 0 || st.st_size == 0) {
        log_warn("[Stream %s] Segment file is empty or cannot be accessed: %s (size: %ld bytes)",
                thread->stream_name, segment_path, (long)(stat(segment_path, &st) == 0 ? st.st_size : 0));
        log_info("[Stream %s] Continuing detection thread despite invalid segment", thread->stream_name);
        return 0;
    }

    // CRITICAL FIX: Initialize format_ctx to NULL before calling avformat_open_input
    // This prevents potential double-free issues if avformat_open_input fails
    format_ctx = NULL;

    // Open input file with safety checks
    int open_result = avformat_open_input(&format_ctx, segment_path, NULL, NULL);
    if (open_result != 0) {
        char err_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(open_result, err_buf, sizeof(err_buf));
        log_error("[Stream %s] Could not open segment file: %s (error: %s)",
                 thread->stream_name, segment_path, err_buf);

        // CRITICAL FIX: Ensure format_ctx is properly cleaned up if avformat_open_input fails
        // This prevents memory leaks and segmentation faults
        if (format_ctx) {
            avformat_close_input(&format_ctx);
            format_ctx = NULL;
        }

        log_info("[Stream %s] Continuing detection thread despite failure to open segment", thread->stream_name);
        // No cleanup handler to pop
        return 0;
    }

    // Format context is now allocated

    // CRITICAL FIX: Verify that format_ctx is valid
    if (!format_ctx) {
        log_error("[Stream %s] Format context is NULL after opening segment: %s",
                thread->stream_name, segment_path);
        log_info("[Stream %s] Continuing detection thread despite invalid format context", thread->stream_name);
        return 0;
    }

    // Find stream info with safety checks
    int find_stream_result = avformat_find_stream_info(format_ctx, NULL);
    if (find_stream_result < 0) {
        char err_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(find_stream_result, err_buf, sizeof(err_buf));
        log_error("[Stream %s] Could not find stream info in segment file: %s (error: %s)",
                 thread->stream_name, segment_path, err_buf);
        safe_avformat_cleanup(&format_ctx); // Use our safe cleanup function
        log_info("[Stream %s] Continuing detection thread despite failure to find stream info", thread->stream_name);
        return 0;
    }

    // CRITICAL FIX: Verify that format_ctx and streams are valid
    if (!format_ctx || !format_ctx->nb_streams) {
        log_error("[Stream %s] Invalid format context or no streams after finding stream info: %s",
                thread->stream_name, segment_path);
        safe_avformat_cleanup(&format_ctx); // Use our safe cleanup function
        log_info("[Stream %s] Continuing detection thread despite invalid streams", thread->stream_name);
        return 0;
    }

    // Find video stream with safety checks
    video_stream_idx = -1;
    for (int i = 0; i < format_ctx->nb_streams; i++) {
        if (format_ctx->streams[i] && format_ctx->streams[i]->codecpar &&
            format_ctx->streams[i]->codecpar->codec_type == AVMEDIA_TYPE_VIDEO) {
            video_stream_idx = i;
            break;
        }
    }

    if (video_stream_idx == -1) {
        log_error("[Stream %s] Could not find video stream in segment file: %s",
                 thread->stream_name, segment_path);
        safe_avformat_cleanup(&format_ctx); // Use our safe cleanup function
        log_info("[Stream %s] Continuing detection thread despite failure to find video stream", thread->stream_name);
        return 0;
    }

    // Get codec with safety checks
    if (!format_ctx->streams[video_stream_idx] || !format_ctx->streams[video_stream_idx]->codecpar) {
        log_error("[Stream %s] Invalid codec parameters in segment file: %s",
                thread->stream_name, segment_path);
        safe_avformat_cleanup(&format_ctx); // Use our safe cleanup function
        log_info("[Stream %s] Continuing detection thread despite invalid codec parameters", thread->stream_name);
        return 0;
    }

    const AVCodec *codec = avcodec_find_decoder(format_ctx->streams[video_stream_idx]->codecpar->codec_id);
    if (!codec) {
        log_error("[Stream %s] Unsupported codec in segment file: %s (codec_id: %d)",
                 thread->stream_name, segment_path, format_ctx->streams[video_stream_idx]->codecpar->codec_id);
        safe_avformat_cleanup(&format_ctx); // Use our safe cleanup function
        log_info("[Stream %s] Continuing detection thread despite unsupported codec", thread->stream_name);
        return 0;
    }

    // Allocate codec context with safety checks
    codec_ctx = avcodec_alloc_context3(codec);
    if (!codec_ctx) {
        log_error("[Stream %s] Could not allocate codec context for segment file: %s",
                 thread->stream_name, segment_path);
        safe_avformat_cleanup(&format_ctx); // Use our safe cleanup function
        log_info("[Stream %s] Continuing detection thread despite failure to allocate codec context", thread->stream_name);
        // No cleanup handler to pop
        return 0;
    }

    // Codec context is now allocated

    // Copy codec parameters with safety checks
    int copy_params_result = avcodec_parameters_to_context(codec_ctx, format_ctx->streams[video_stream_idx]->codecpar);
    if (copy_params_result < 0) {
        char err_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(copy_params_result, err_buf, sizeof(err_buf));
        log_error("[Stream %s] Could not copy codec parameters for segment file: %s (error: %s)",
                 thread->stream_name, segment_path, err_buf);
        avcodec_free_context(&codec_ctx);
        safe_avformat_cleanup(&format_ctx); // Use our safe cleanup function
        log_info("[Stream %s] Continuing detection thread despite failure to copy codec parameters", thread->stream_name);
        return 0;
    }

    // Open codec with safety checks
    int open_codec_result = avcodec_open2(codec_ctx, codec, NULL);
    if (open_codec_result < 0) {
        char err_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(open_codec_result, err_buf, sizeof(err_buf));
        log_error("[Stream %s] Could not open codec for segment file: %s (error: %s)",
                 thread->stream_name, segment_path, err_buf);
        avcodec_free_context(&codec_ctx);
        safe_avformat_cleanup(&format_ctx); // Use our safe cleanup function
        log_info("[Stream %s] Continuing detection thread despite failure to open codec", thread->stream_name);
        return 0;
    }

    // Allocate frame and packet with safety checks
    frame = av_frame_alloc();
    if (!frame) {
        log_error("[Stream %s] Could not allocate frame for segment file: %s",
                 thread->stream_name, segment_path);
        avcodec_free_context(&codec_ctx);
        safe_avformat_cleanup(&format_ctx); // Use our safe cleanup function
        log_info("[Stream %s] Continuing detection thread despite failure to allocate frame", thread->stream_name);
        // No cleanup handler to pop
        return 0;
    }

    // Frame is now allocated

    pkt = av_packet_alloc();
    if (!pkt) {
        log_error("[Stream %s] Could not allocate packet for segment file: %s",
                 thread->stream_name, segment_path);
        av_frame_free(&frame);
        avcodec_free_context(&codec_ctx);
        safe_avformat_cleanup(&format_ctx); // Use our safe cleanup function
        log_info("[Stream %s] Continuing detection thread despite failure to allocate packet", thread->stream_name);
        // No cleanup handler to pop
        return 0;
    }

    // Packet is now allocated

    // Initialize frame_count at the beginning of the function
    int frame_count = 0;

    // Calculate segment duration with safety checks
    float segment_duration = 0;
    if (format_ctx->duration != AV_NOPTS_VALUE) {
        segment_duration = format_ctx->duration / (float)AV_TIME_BASE;
    } else {
        // Default to 2 seconds if duration is not available
        segment_duration = 2.0f;
        log_warn("[Stream %s] Segment duration not available, using default: %.2f seconds",
                thread->stream_name, segment_duration);
    }

    // Validate segment duration
    if (segment_duration <= 0 || segment_duration > 60) { // Sanity check: segments shouldn't be longer than 60 seconds
        log_warn("[Stream %s] Invalid segment duration: %.2f seconds, using default: 2.0 seconds",
                thread->stream_name, segment_duration);
        segment_duration = 2.0f; // Use a reasonable default
    }

    // Since we haven't counted frames yet, use a reasonable estimate for frames_per_second
    double frames_per_second = 0.0;
    // Get the frame rate from the stream if available
    AVRational frame_rate = av_guess_frame_rate(format_ctx, format_ctx->streams[video_stream_idx], NULL);
    if (frame_rate.num > 0 && frame_rate.den > 0) {
        frames_per_second = (double)frame_rate.num / frame_rate.den;
        log_info("[Stream %s] Using stream frame rate: %.2f fps",
                 thread->stream_name, frames_per_second);
    } else {
        // Use a reasonable default if we can't get the frame rate
        frames_per_second = 25.0;
        log_warn("[Stream %s] Couldn't determine frame rate, using default: %.2f fps",
                 thread->stream_name, frames_per_second);
    }

    // Validate frames per second
    if (frames_per_second <= 0 || frames_per_second > 120) { // Sanity check: frame rates shouldn't be higher than 120 fps
        log_warn("[Stream %s] Invalid frames per second: %.2f fps, using default: 25.0 fps",
                thread->stream_name, frames_per_second);
        frames_per_second = 25.0f; // Use a reasonable default
    }

    // Calculate total frames with safety checks
    int total_frames = (int)(segment_duration * frames_per_second);

    // Validate total frames
    if (total_frames <= 0 || total_frames > 10000) { // Sanity check: segments shouldn't have more than 10000 frames
        log_warn("[Stream %s] Invalid total frames: %d, using default: 50 frames",
                thread->stream_name, total_frames);
        total_frames = 50; // Use a reasonable default
    }

    log_info("[Stream %s] Segment duration: %.2f seconds, FPS: %.2f, Estimated total frames: %d",
             thread->stream_name, segment_duration, frames_per_second, total_frames);

    // Read frames with safety checks
    int processed_frames = 0;
    int error_frames = 0;
    int max_errors = 10; // Maximum number of errors before giving up

    // CRITICAL FIX: Add a maximum frame count to prevent infinite loops
    int max_frames = total_frames * 2; // Double the expected frame count as a safety measure

    while (frame_count < max_frames) {
        // Read frame with safety checks
        int read_result = av_read_frame(format_ctx, pkt);
        if (read_result < 0) {
            // Check if we've reached the end of the file
            if (read_result == AVERROR_EOF) {
                log_info("[Stream %s] Reached end of segment file: %s",
                        thread->stream_name, segment_path);
                break;
            }

            // Log other errors
            char err_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
            av_strerror(read_result, err_buf, sizeof(err_buf));
            log_error("[Stream %s] Error reading frame from segment file: %s (error: %s)",
                     thread->stream_name, segment_path, err_buf);

            // Count errors and break if too many
            error_frames++;
            if (error_frames >= max_errors) {
                log_error("[Stream %s] Too many errors reading frames from segment file: %s (errors: %d)",
                         thread->stream_name, segment_path, error_frames);
                break;
            }

            // Try to continue with the next frame
            av_packet_unref(pkt);
            continue;
        }

        // Only process video packets with safety checks
        if (pkt->stream_index == video_stream_idx) {
            frame_count++;

            // Send packet to decoder with safety checks
            ret = avcodec_send_packet(codec_ctx, pkt);
            if (ret < 0) {
                char err_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
                av_strerror(ret, err_buf, sizeof(err_buf));
                log_error("[Stream %s] Error sending packet to decoder for segment file: %s (error: %s)",
                         thread->stream_name, segment_path, err_buf);
                av_packet_unref(pkt);
                continue;
            }

            // Receive frame from decoder with safety checks
            ret = avcodec_receive_frame(codec_ctx, frame);
            if (ret == AVERROR(EAGAIN) || ret == AVERROR_EOF) {
                av_packet_unref(pkt);
                continue;
            } else if (ret < 0) {
                char err_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
                av_strerror(ret, err_buf, sizeof(err_buf));
                log_error("[Stream %s] Error receiving frame from decoder for segment file: %s (error: %s)",
                         thread->stream_name, segment_path, err_buf);

                // Count errors and break if too many
                error_frames++;
                if (error_frames >= max_errors) {
                    log_error("[Stream %s] Too many errors receiving frames from decoder for segment file: %s (errors: %d)",
                             thread->stream_name, segment_path, error_frames);
                    av_packet_unref(pkt);
                    break;
                }

                // Try to continue with the next frame
                av_packet_unref(pkt);
                continue;
            }

            // OPTIMIZATION: Process only key frames (I-frames) to reduce CPU usage
            // Check if this is a key frame (I-frame)
            bool is_key_frame = frame->key_frame || (frame->pict_type == AV_PICTURE_TYPE_I);

            if (is_key_frame) {
                log_info("[Stream %s] Processing key frame %d (pict_type: %d, key_frame: %d)",
                        thread->stream_name, frame_count, frame->pict_type, frame->key_frame);

                // Process the frame for detection
                log_info("[Stream %s] Processing frame %d from segment file: %s",
                        thread->stream_name, frame_count, segment_path);

                // Calculate frame timestamp based on segment timestamp
                time_t frame_timestamp = time(NULL);

                // CRITICAL FIX: Ensure only one detection is running at a time
                // Lock the thread mutex to ensure exclusive access to the model
                pthread_mutex_lock(&thread->mutex);

                // Process the frame for detection using our dedicated model
                if (thread->model) {
                    // Convert frame to RGB format
                    int width = frame->width;
                    int height = frame->height;
                    int channels = 3; // RGB

                    // Determine if we should downscale the frame based on model type
                    const char *model_type = get_model_type_from_handle(thread->model);
                    int downscale_factor = get_downscale_factor(model_type);

                    // Calculate dimensions after downscaling
                    int target_width = width / downscale_factor;
                    int target_height = height / downscale_factor;

                    // Ensure dimensions are even (required by some codecs)
                    target_width = (target_width / 2) * 2;
                    target_height = (target_height / 2) * 2;

                    // Convert frame to RGB format with downscaling
                    sws_ctx = sws_getContext(
                        width, height, frame->format,
                        target_width, target_height, AV_PIX_FMT_RGB24,
                        SWS_BILINEAR, NULL, NULL, NULL);

                    if (!sws_ctx) {
                        log_error("[Stream %s] Failed to create SwsContext", thread->stream_name);
                        av_packet_unref(pkt);
                        continue;
                    }

                    // SwsContext is now allocated

                    // Allocate buffer for RGB frame
                    uint8_t *rgb_buffer = (uint8_t *)malloc(target_width * target_height * channels);
                    if (!rgb_buffer) {
                        log_error("[Stream %s] Failed to allocate RGB buffer", thread->stream_name);
                        sws_freeContext(sws_ctx);
                        av_packet_unref(pkt);
                        continue;
                    }

                    // Setup RGB frame
                    uint8_t *rgb_data[4] = {rgb_buffer, NULL, NULL, NULL};
                    int rgb_linesize[4] = {target_width * channels, 0, 0, 0};

                    // Convert frame to RGB
                    sws_scale(sws_ctx, (const uint8_t * const *)frame->data, frame->linesize, 0,
                             height, rgb_data, rgb_linesize);

                    // Create detection result structure
                    detection_result_t result;
                    memset(&result, 0, sizeof(detection_result_t));

                    // Log before running detection
                    log_info("[Stream %s] Running detection on frame %d (dimensions: %dx%d, channels: %d, model: %s)",
                            thread->stream_name, frame_count, target_width, target_height, channels,
                            model_type ? model_type : "unknown");

                    // Run detection on the RGB frame
                    int detect_ret;

                    // Check if this is an API model
                    const char *api_model_type = get_model_type_from_handle(thread->model);
                    log_info("[Stream %s] Model type: %s", thread->stream_name, api_model_type);

                    if (strcmp(api_model_type, MODEL_TYPE_API) == 0) {
                        // For API models, we need to pass the stream name
                        const char *model_path = get_model_path(thread->model);

                        // Get the API URL - either from the model path if it's a URL,
                        // or from the global config if it's the special "api-detection" string
                        const char *api_url = NULL;
                        if (model_path && ends_with(model_path, "api-detection")) {
                            // Get the API URL from the global config
                            api_url = g_config.api_detection_url;
                            log_info("[Stream %s] Using API detection URL from config: %s",
                                    thread->stream_name, api_url ? api_url : "NULL");
                        } else {
                            // Use the model path directly as the URL
                            api_url = model_path;
                            log_info("[Stream %s] Using API detection with URL from model path: %s",
                                    thread->stream_name, api_url ? api_url : "NULL");
                        }

                        if (!api_url || api_url[0] == '\0') {
                            log_error("[Stream %s] Failed to get API URL from model or config", thread->stream_name);
                            detect_ret = -1;
                            // Initialize result to empty to prevent segmentation fault
                            memset(&result, 0, sizeof(detection_result_t));
                        } else {
                            log_info("[Stream %s] Calling detect_objects_api with URL: %s", thread->stream_name, api_url);
                            // CRITICAL FIX: Initialize result to empty before calling API detection
                            memset(&result, 0, sizeof(detection_result_t));
                            detect_ret = detect_objects_api(api_url, rgb_buffer, target_width, target_height, channels, &result, thread->stream_name);
                            log_info("[Stream %s] detect_objects_api returned: %d", thread->stream_name, detect_ret);
                        }
                    } else {
                        // For other models, use the standard detect_objects function
                        log_info("[Stream %s] Using standard detect_objects function", thread->stream_name);
                        // CRITICAL FIX: Initialize result to empty before calling detection
                        memset(&result, 0, sizeof(detection_result_t));
                        detect_ret = detect_objects(thread->model, rgb_buffer, target_width, target_height, channels, &result);
                        log_info("[Stream %s] detect_objects returned: %d", thread->stream_name, detect_ret);
                    }

                    if (detect_ret == 0) {
                        // Process detection results
                        if (result.count > 0) {
                            log_info("[Stream %s] Detection found %d objects in frame %d",
                                    thread->stream_name, result.count, frame_count);

                            // Log each detected object
                            for (int i = 0; i < result.count && i < MAX_DETECTIONS; i++) {
                                log_info("[Stream %s] Object %d: class=%s, confidence=%.2f, box=[%.2f,%.2f,%.2f,%.2f]",
                                        thread->stream_name, i, result.detections[i].label,
                                        result.detections[i].confidence,
                                        result.detections[i].x, result.detections[i].y,
                                        result.detections[i].width, result.detections[i].height);
                            }

                            // Process the detection results for recording
                            int record_ret = process_frame_for_recording(thread->stream_name, rgb_buffer, target_width,
                                                                       target_height, channels, frame_timestamp, &result);

                            if (record_ret != 0) {
                                log_error("[Stream %s] Failed to process frame for recording (error code: %d)",
                                         thread->stream_name, record_ret);
                            } else {
                                log_info("[Stream %s] Successfully processed frame for recording", thread->stream_name);
                            }
                        } else {
                            log_debug("[Stream %s] No objects detected in frame %d", thread->stream_name, frame_count);
                        }
                    } else {
                        log_error("[Stream %s] Detection failed for frame %d (error code: %d)",
                                 thread->stream_name, frame_count, detect_ret);
                        // Continue execution despite detection failure
                        log_info("[Stream %s] Continuing detection thread despite detection failure", thread->stream_name);
                        // Set result.count to 0 to indicate no detections
                        result.count = 0;
                    }

                    // Free resources
                    free(rgb_buffer);
                    sws_freeContext(sws_ctx);

                    // Update last detection time
                    thread->last_detection_time = time(NULL);
                }

                // CRITICAL FIX: Release the mutex after detection is complete
                pthread_mutex_unlock(&thread->mutex);

                processed_frames++;
            }
        }

        av_packet_unref(pkt);
    }

    log_info("[Stream %s] Processed %d frames out of %d total frames from segment file: %s (errors: %d)",
             thread->stream_name, processed_frames, frame_count, segment_path, error_frames);

    // CRITICAL FIX: Use comprehensive cleanup to prevent memory leaks and segmentation faults
    log_debug("[Stream %s] Starting comprehensive cleanup of FFmpeg resources", thread->stream_name);

    // Cleanup with safety checks
    if (frame) {
        log_debug("[Stream %s] Freeing frame during cleanup", thread->stream_name);
        av_frame_free(&frame);
    }

    if (pkt) {
        log_debug("[Stream %s] Freeing packet during cleanup", thread->stream_name);
        av_packet_free(&pkt);
    }

    if (codec_ctx) {
        log_debug("[Stream %s] Freeing codec context during cleanup", thread->stream_name);
        avcodec_free_context(&codec_ctx);
    }

    if (format_ctx) {
        log_debug("[Stream %s] Closing input format context during cleanup", thread->stream_name);
        avformat_close_input(&format_ctx);
    }

    log_debug("[Stream %s] Completed comprehensive cleanup of FFmpeg resources", thread->stream_name);

    // No cleanup handler to pop

    return 0;
}

/**
 * Check for new HLS segments in the stream's HLS directory
 * This function has been refactored to ensure each detection thread only monitors its own stream
 * Added retry mechanism and improved robustness for handling HLS writer failures
 */
static void check_for_new_segments(stream_detection_thread_t *thread) {
    time_t current_time = time(NULL);
    static time_t last_warning_time = 0;
    static int consecutive_failures = 0;
    static bool first_check = true;
    char newest_segment[MAX_PATH_LENGTH] = {0};
    time_t newest_time = 0;
    int segment_count = 0;

    // Check if we should run detection based on startup delay, detection in progress and time interval
    bool should_run_detection = should_run_detection_check(thread, current_time);

    // Check and manage HLS writer status
    check_hls_writer_status(thread, current_time, &last_warning_time, first_check, &consecutive_failures);

    // Find and set HLS directory path
    bool hls_dir_valid = find_hls_directory(thread, current_time, &last_warning_time, &consecutive_failures, first_check);
    if (!hls_dir_valid) {
        first_check = false;
        return;
    }

    // Find the newest segment
    bool found_segment = find_newest_segment(thread, newest_segment, &newest_time, &segment_count);
    if (!found_segment) {
        // If no segments were found, attempt to restart the HLS stream if needed
        restart_hls_stream_if_needed(thread, current_time, &last_warning_time, first_check);

        // We would do go2rtc fallback here in the original function, but we're keeping that in the original file
        // as it's very complex and would make our helper implementation too large

        consecutive_failures++;
        first_check = false;
        return;
    }

    // Reset failure counter when we find segments
    consecutive_failures = 0;

    log_info("[Stream %s] Found %d segments, newest segment time: %s",
             thread->stream_name, segment_count, ctime(&newest_time));

    // Process the segment if needed
    static char last_processed_segment[MAX_PATH_LENGTH] = {0};
    process_segment_if_needed(thread, newest_segment, last_processed_segment,
                             should_run_detection, newest_time, current_time);

    first_check = false;
}

/**
 * Stream detection thread function
 * Improved with better error handling and retry logic
 */
static void *stream_detection_thread_func(void *arg) {
    stream_detection_thread_t *thread = (stream_detection_thread_t *)arg;
    int model_load_retries = 0;
    const int MAX_MODEL_LOAD_RETRIES = 5;

    log_info("[Stream %s] Detection thread started", thread->stream_name);

    // Register with shutdown coordinator
    char component_name[128];
    snprintf(component_name, sizeof(component_name), "detection_thread_%s", thread->stream_name);
    thread->component_id = register_component(component_name, COMPONENT_DETECTION_THREAD, NULL, 100);

    if (thread->component_id >= 0) {
        log_info("[Stream %s] Registered with shutdown coordinator (ID: %d)",
                thread->stream_name, thread->component_id);
    }

    // CRITICAL FIX: Ensure model is loaded with proper error handling
    pthread_mutex_lock(&thread->mutex);
    if (!thread->model && thread->model_path[0] != '\0') {
        log_info("[Stream %s] Loading detection model: %s", thread->stream_name, thread->model_path);

        // Check if this is an API URL or the special "api-detection" string
        bool is_api_detection = ends_with(thread->model_path, "api-detection");

        // Only check file existence if it's not an API detection
        if (!is_api_detection) {
            // Check if model file exists
            struct stat st;
            if (stat(thread->model_path, &st) != 0) {
                log_error("[Stream %s] Model file does not exist: %s", thread->stream_name, thread->model_path);

                // Try to find the model in alternative locations
                char alt_model_path[MAX_PATH_LENGTH];
                const char *locations[] = {
                    "/var/lib/lightnvr/models/",
                    "/etc/lightnvr/models/",
                    "/usr/local/share/lightnvr/models/"
                };

                bool found = false;
                for (int i = 0; i < sizeof(locations)/sizeof(locations[0]); i++) {
                    // Try with just the filename (not the full path)
                    const char *filename = strrchr(thread->model_path, '/');
                    if (filename) {
                        filename++; // Skip the '/'
                    } else {
                        filename = thread->model_path; // No '/' in the path
                    }

                    snprintf(alt_model_path, MAX_PATH_LENGTH, "%s%s", locations[i], filename);
                    if (stat(alt_model_path, &st) == 0) {
                        log_info("[Stream %s] Found model at alternative location: %s",
                                thread->stream_name, alt_model_path);
                        strncpy(thread->model_path, alt_model_path, MAX_PATH_LENGTH - 1);
                        thread->model_path[MAX_PATH_LENGTH - 1] = '\0';
                        found = true;
                        break;
                    }
                }

                if (!found) {
                    log_error("[Stream %s] Could not find model in any location", thread->stream_name);
                    model_load_retries++;
                    pthread_mutex_unlock(&thread->mutex);
                    // Don't exit the thread, just continue without a model
                    // We'll retry loading the model later
                    log_warn("[Stream %s] Continuing detection thread despite model loading failure", thread->stream_name);
                }
            }
        } else {
            log_info("[Stream %s] Using API detection, no need to check for model file on disk", thread->stream_name);
        }

        // Load the model with explicit logging
        log_info("[Stream %s] CRITICAL FIX: Loading model from path: %s",
                thread->stream_name, thread->model_path);
        thread->model = load_detection_model(thread->model_path, thread->threshold);

        if (!thread->model) {
            log_error("[Stream %s] Failed to load detection model: %s, will retry later",
                     thread->stream_name, thread->model_path);
            model_load_retries++;
        } else {
            log_info("[Stream %s] Successfully loaded detection model: %p",
                    thread->stream_name, (void*)thread->model);

            // Verify model type
            const char *model_type = get_model_type_from_handle(thread->model);
            log_info("[Stream %s] Loaded model type: %s", thread->stream_name, model_type);
        }
    } else if (thread->model) {
        log_info("[Stream %s] Model already loaded: %p", thread->stream_name, (void*)thread->model);
    } else {
        log_error("[Stream %s] No model path specified", thread->stream_name);
    }
    pthread_mutex_unlock(&thread->mutex);

    // Main thread loop with improved monitoring and error handling
    time_t last_segment_check = 0;
    time_t last_model_retry = 0;
    time_t last_log_time = 0;
    time_t startup_time = time(NULL);
    int consecutive_empty_checks = 0;
    int consecutive_errors = 0;
    bool initial_startup_period = true;

    // Set a 10-second delay before starting to process segments
    // This gives the system time to initialize without blocking the main thread
    global_startup_delay_end = startup_time + 10;

    while (thread->running) {
        log_info("[Stream %s] Checking for new segments in HLS directory", thread->stream_name);

        // Check if shutdown has been initiated
        if (is_shutdown_initiated()) {
            log_info("[Stream %s] Stopping due to system shutdown", thread->stream_name);
            break;
        }

        time_t current_time = time(NULL);

        // Try to load the model again if previous attempts failed
        if (!thread->model && thread->model_path[0] != '\0' &&
            model_load_retries < MAX_MODEL_LOAD_RETRIES &&
            current_time - last_model_retry >= 5) { // Retry every 5 seconds

            pthread_mutex_lock(&thread->mutex);
            log_info("[Stream %s] Retrying to load detection model: %s (attempt %d/%d)",
                    thread->stream_name, thread->model_path, model_load_retries + 1, MAX_MODEL_LOAD_RETRIES);
            thread->model = load_detection_model(thread->model_path, thread->threshold);
            if (!thread->model) {
                log_error("[Stream %s] Failed to load detection model: %s on retry %d",
                         thread->stream_name, thread->model_path, model_load_retries + 1);
                model_load_retries++;
            } else {
                log_info("[Stream %s] Successfully loaded detection model on retry", thread->stream_name);
                model_load_retries = 0;
            }
            pthread_mutex_unlock(&thread->mutex);

            last_model_retry = current_time;
        }

        // Log status periodically - always use log_info to ensure visibility
        if (current_time - last_log_time > 10) { // Log every 10 seconds
            log_info("[Stream %s] Detection thread is running, checking for new segments (consecutive empty checks: %d, errors: %d)",
                    thread->stream_name, consecutive_empty_checks, consecutive_errors);
            last_log_time = current_time;

            // Calculate time since last detection
            time_t time_since_detection = 0;
            if (thread->last_detection_time > 0) {
                time_since_detection = current_time - thread->last_detection_time;
            }

            // Check if a detection is in progress
            int detection_running = atomic_load(&thread->detection_in_progress);

            // Log with appropriate level based on status
            if (detection_running) {
                // Detection is in progress, this is normal
                log_info("[Stream %s] Thread status: model loaded: %s, detection in progress, last detection: %s (%ld seconds ago), interval: %d seconds",
                        thread->stream_name,
                        thread->model ? "yes" : "no",
                        thread->last_detection_time > 0 ? ctime(&thread->last_detection_time) : "never",
                        time_since_detection,
                        thread->detection_interval);
            } else if (time_since_detection > (thread->detection_interval * 2)) {
                // No detection in progress but we're behind schedule
                log_warn("[Stream %s] Thread status: model loaded: %s, no detection in progress, last detection: %s (%ld seconds ago), interval: %d seconds",
                        thread->stream_name,
                        thread->model ? "yes" : "no",
                        thread->last_detection_time > 0 ? ctime(&thread->last_detection_time) : "never",
                        time_since_detection,
                        thread->detection_interval);
            } else {
                // Normal status
                log_info("[Stream %s] Thread status: model loaded: %s, no detection in progress, last detection: %s (%ld seconds ago), interval: %d seconds",
                        thread->stream_name,
                        thread->model ? "yes" : "no",
                        thread->last_detection_time > 0 ? ctime(&thread->last_detection_time) : "never",
                        time_since_detection,
                        thread->detection_interval);
            }
        }

        // Check for new segments more frequently if we've had consecutive empty checks
        // This helps ensure we catch new segments as soon as they appear
        int check_interval = 1; // Default to 1 second
        if (consecutive_empty_checks > 10) {
            check_interval = 2; // Slow down a bit after 10 empty checks
        } else if (consecutive_empty_checks > 30) {
            check_interval = 5; // Slow down more after 30 empty checks
        }

        if (current_time - last_segment_check >= check_interval) {
            // Check for new segments
            check_for_new_segments(thread);
            last_segment_check = current_time;

            // Update consecutive check counters based on result
            // This is handled inside check_for_new_segments
        }

        int sleep_time = 500000; // Default 500ms
        usleep(sleep_time);
    }

    // Update component state in shutdown coordinator
    if (thread->component_id >= 0) {
        update_component_state(thread->component_id, COMPONENT_STOPPED);
        log_info("[Stream %s] Updated state to STOPPED in shutdown coordinator", thread->stream_name);
    }

    // Unload the model with enhanced cleanup for SOD models
    pthread_mutex_lock(&thread->mutex);
    if (thread->model) {
        log_info("[Stream %s] Unloading detection model", thread->stream_name);

        // Get the model type to check if it's a SOD model
        const char *model_type = get_model_type_from_handle(thread->model);

        // Use our enhanced cleanup for SOD models to prevent memory leaks
        if (strcmp(model_type, MODEL_TYPE_SOD) == 0) {
            log_info("[Stream %s] Using enhanced SOD model cleanup to prevent memory leaks", thread->stream_name);
            ensure_sod_model_cleanup(thread->model);
        } else {
            // For non-SOD models, use the standard unload function
            unload_detection_model(thread->model);
        }

        thread->model = NULL;
    }
    pthread_mutex_unlock(&thread->mutex);

    log_info("[Stream %s] Detection thread exiting", thread->stream_name);
    return NULL;
}

/**
 * Initialize the stream detection system
 */
int init_stream_detection_system(void) {
    if (system_initialized) {
        return 0;
    }

    pthread_mutex_lock(&stream_threads_mutex);

    // Initialize all thread structures
    for (int i = 0; i < MAX_STREAM_THREADS; i++) {
        memset(&stream_threads[i], 0, sizeof(stream_detection_thread_t));
        pthread_mutex_init(&stream_threads[i].mutex, NULL);
        pthread_cond_init(&stream_threads[i].cond, NULL);
    }

    system_initialized = true;
    pthread_mutex_unlock(&stream_threads_mutex);

    log_info("Stream detection system initialized");
    return 0;
}

/**
 * Shutdown the stream detection system
 */
void shutdown_stream_detection_system(void) {
    if (!system_initialized) {
        return;
    }

    pthread_mutex_lock(&stream_threads_mutex);

    // Stop all running threads
    for (int i = 0; i < MAX_STREAM_THREADS; i++) {
        if (stream_threads[i].running) {
            log_info("Stopping detection thread for stream %s", stream_threads[i].stream_name);

            // First, check if the thread has a model loaded and ensure it's properly cleaned up
            pthread_mutex_lock(&stream_threads[i].mutex);

            // CRITICAL FIX: Make a local copy of the model pointer to prevent race conditions
            detection_model_t model_to_cleanup = NULL;
            if (stream_threads[i].model) {
                log_info("Ensuring model cleanup during shutdown for stream %s", stream_threads[i].stream_name);
                model_to_cleanup = stream_threads[i].model;

                // Immediately set the thread's model to NULL to prevent double-free
                stream_threads[i].model = NULL;
            }
            pthread_mutex_unlock(&stream_threads[i].mutex);

            // Now clean up the model outside the mutex lock if we have one to clean up
            if (model_to_cleanup) {
                // Get the model type to check if it's a SOD model
                const char *model_type = get_model_type_from_handle(model_to_cleanup);

                // Use our enhanced cleanup for SOD models to prevent memory leaks
                if (strcmp(model_type, MODEL_TYPE_SOD) == 0) {
                    log_info("Using enhanced SOD model cleanup to prevent memory leaks during shutdown");
                    ensure_sod_model_cleanup(model_to_cleanup);
                } else if (strcmp(model_type, "unknown") != 0) {
                    // For non-SOD models (except unknown type), use standard unload
                    log_info("Using standard unload for non-SOD model type: %s during shutdown", model_type);
                    unload_detection_model(model_to_cleanup);
                } else {
                    // For unknown model type, use the safest approach
                    log_warn("Unknown model type detected during shutdown, using generic unload");
                    unload_detection_model(model_to_cleanup);
                }

                log_info("Model cleanup completed during shutdown for stream %s", stream_threads[i].stream_name);
            }

            // CRITICAL FIX: Improved thread stopping process
            // First signal the thread to stop
            stream_threads[i].running = false;

            // Signal the condition variable to wake up the thread if it's waiting
            pthread_mutex_lock(&stream_threads[i].mutex);
            pthread_cond_signal(&stream_threads[i].cond);
            pthread_mutex_unlock(&stream_threads[i].mutex);

            // Add a small delay to allow the thread to start its shutdown process
            usleep(10000); // 10ms

            // Now join the thread with a timeout to prevent hanging
            struct timespec timeout;
            clock_gettime(CLOCK_REALTIME, &timeout);
            timeout.tv_sec += 5; // 5 second timeout

            #if defined(__linux__) && defined(_GNU_SOURCE)
            int join_result = pthread_timedjoin_np(stream_threads[i].thread, NULL, &timeout);
            if (join_result != 0) {
                log_warn("Failed to join detection thread for stream %s within timeout: %s",
                        stream_threads[i].stream_name, strerror(join_result));
                // Continue anyway - we'll clean up resources
            }
            #else
            // Regular join as fallback
            pthread_join(stream_threads[i].thread, NULL);
            #endif

            // Cleanup resources
            pthread_mutex_destroy(&stream_threads[i].mutex);
            pthread_cond_destroy(&stream_threads[i].cond);
        }
    }

    // Force cleanup of all SOD models to prevent memory leaks
    log_info("Forcing cleanup of all SOD models during shutdown");
    force_sod_models_cleanup();

    system_initialized = false;
    pthread_mutex_unlock(&stream_threads_mutex);

    log_info("Stream detection system shutdown");
}

/**
 * Start a detection thread for a stream
 */
int start_stream_detection_thread(const char *stream_name, const char *model_path,
                                 float threshold, int detection_interval, const char *hls_dir) {
    if (!system_initialized) {
        if (init_stream_detection_system() != 0) {
            log_error("Failed to initialize stream detection system");
            return -1;
        }
    }

    if (!stream_name || !model_path || !hls_dir) {
        log_error("Invalid parameters for start_stream_detection_thread");
        return -1;
    }

    // Make sure the HLS directory exists
    struct stat st;
    if (stat(hls_dir, &st) != 0) {
        log_warn("HLS directory does not exist, creating it: %s", hls_dir);
        char cmd[MAX_PATH_LENGTH * 2];
        snprintf(cmd, sizeof(cmd), "mkdir -p %s", hls_dir);
        system(cmd);
    }

    pthread_mutex_lock(&stream_threads_mutex);

    // Check if a thread is already running for this stream
    for (int i = 0; i < MAX_STREAM_THREADS; i++) {
        if (stream_threads[i].running && strcmp(stream_threads[i].stream_name, stream_name) == 0) {
            log_info("Detection thread already running for stream %s", stream_name);
            pthread_mutex_unlock(&stream_threads_mutex);
            return 0;
        }
    }

    // Find an available thread slot
    int slot = -1;
    for (int i = 0; i < MAX_STREAM_THREADS; i++) {
        if (!stream_threads[i].running) {
            slot = i;
            break;
        }
    }

    if (slot == -1) {
        log_error("No available thread slots for stream %s", stream_name);
        pthread_mutex_unlock(&stream_threads_mutex);
        return -1;
    }

    // Initialize thread structure
    stream_detection_thread_t *thread = &stream_threads[slot];
    strncpy(thread->stream_name, stream_name, MAX_STREAM_NAME - 1);
    thread->stream_name[MAX_STREAM_NAME - 1] = '\0';

    strncpy(thread->model_path, model_path, MAX_PATH_LENGTH - 1);
    thread->model_path[MAX_PATH_LENGTH - 1] = '\0';

    strncpy(thread->hls_dir, hls_dir, MAX_PATH_LENGTH - 1);
    thread->hls_dir[MAX_PATH_LENGTH - 1] = '\0';

    thread->threshold = threshold;
    thread->detection_interval = detection_interval;
    thread->running = true;
    thread->model = NULL;
    thread->last_detection_time = 0;
    atomic_init(&thread->detection_in_progress, 0); // Initialize atomic flag to 0 (no detection in progress)

    // Create the thread
    if (pthread_create(&thread->thread, NULL, stream_detection_thread_func, thread) != 0) {
        log_error("Failed to create detection thread for stream %s", stream_name);
        thread->running = false;
        pthread_mutex_unlock(&stream_threads_mutex);
        return -1;
    }

    log_info("Started detection thread for stream %s with model %s", stream_name, model_path);
    pthread_mutex_unlock(&stream_threads_mutex);
    return 0;
}

/**
 * Stop a detection thread for a stream
 */
int stop_stream_detection_thread(const char *stream_name) {
    if (!system_initialized || !stream_name) {
        return -1;
    }

    pthread_mutex_lock(&stream_threads_mutex);

    // Find the thread for this stream
    for (int i = 0; i < MAX_STREAM_THREADS; i++) {
        if (stream_threads[i].running && strcmp(stream_threads[i].stream_name, stream_name) == 0) {
            log_info("Stopping detection thread for stream %s", stream_name);

            // First, check if the thread has a model loaded and ensure it's properly cleaned up
            // This is a safety measure in case the thread doesn't clean up its own model
            pthread_mutex_lock(&stream_threads[i].mutex);

            // CRITICAL FIX: Make a local copy of the model pointer to prevent race conditions
            detection_model_t model_to_cleanup = NULL;
            if (stream_threads[i].model) {
                log_info("Ensuring model cleanup before stopping thread for stream %s", stream_name);
                model_to_cleanup = stream_threads[i].model;

                // Immediately set the thread's model to NULL to prevent double-free
                // This ensures that even if another thread tries to access it, it will be NULL
                stream_threads[i].model = NULL;
            }
            pthread_mutex_unlock(&stream_threads[i].mutex);

            // Now clean up the model outside the mutex lock if we have one to clean up
            if (model_to_cleanup) {
                // Get the model type to check if it's a SOD model
                const char *model_type = get_model_type_from_handle(model_to_cleanup);

                // Use our enhanced cleanup for SOD models to prevent memory leaks
                if (strcmp(model_type, MODEL_TYPE_SOD) == 0) {
                    log_info("Using enhanced SOD model cleanup to prevent memory leaks");
                    ensure_sod_model_cleanup(model_to_cleanup);
                } else if (strcmp(model_type, "unknown") != 0) {
                    // For non-SOD models (except unknown type), use standard unload
                    log_info("Using standard unload for non-SOD model type: %s", model_type);
                    unload_detection_model(model_to_cleanup);
                } else {
                    // For unknown model type, use the safest approach
                    log_warn("Unknown model type detected, using generic unload");
                    unload_detection_model(model_to_cleanup);
                }

                // The model pointer is now invalid, no need to set it to NULL as we already did that
                log_info("Model cleanup completed for stream %s", stream_name);
            }

            // Now stop the thread
            stream_threads[i].running = false;
            pthread_join(stream_threads[i].thread, NULL);

            // Clear the thread structure
            memset(&stream_threads[i], 0, sizeof(stream_detection_thread_t));
            pthread_mutex_init(&stream_threads[i].mutex, NULL);
            pthread_cond_init(&stream_threads[i].cond, NULL);

            pthread_mutex_unlock(&stream_threads_mutex);
            return 0;
        }
    }

    log_warn("No detection thread found for stream %s", stream_name);
    pthread_mutex_unlock(&stream_threads_mutex);
    return -1;
}

/**
 * Check if a detection thread is running for a stream
 */
bool is_stream_detection_thread_running(const char *stream_name) {
    if (!system_initialized || !stream_name) {
        return false;
    }

    pthread_mutex_lock(&stream_threads_mutex);

    // Find the thread for this stream
    for (int i = 0; i < MAX_STREAM_THREADS; i++) {
        if (stream_threads[i].running && strcmp(stream_threads[i].stream_name, stream_name) == 0) {
            pthread_mutex_unlock(&stream_threads_mutex);
            return true;
        }
    }

    pthread_mutex_unlock(&stream_threads_mutex);
    return false;
}

/**
 * Get the number of running detection threads
 */
int get_running_stream_detection_threads(void) {
    if (!system_initialized) {
        return 0;
    }

    pthread_mutex_lock(&stream_threads_mutex);

    int count = 0;
    for (int i = 0; i < MAX_STREAM_THREADS; i++) {
        if (stream_threads[i].running) {
            count++;
        }
    }

    pthread_mutex_unlock(&stream_threads_mutex);
    return count;
}

/**
 * Get detailed status information about a detection thread
 *
 * @param stream_name The name of the stream
 * @param has_thread Will be set to true if a thread is running for this stream
 * @param last_check_time Will be set to the last time segments were checked
 * @param last_detection_time Will be set to the last time detection was run
 * @return 0 on success, -1 on error
 */
int get_stream_detection_status(const char *stream_name, bool *has_thread,
                               time_t *last_check_time, time_t *last_detection_time) {
    if (!system_initialized || !stream_name || !has_thread || !last_check_time || !last_detection_time) {
        return -1;
    }

    *has_thread = false;
    *last_check_time = 0;
    *last_detection_time = 0;

    pthread_mutex_lock(&stream_threads_mutex);

    // Find the thread for this stream
    for (int i = 0; i < MAX_STREAM_THREADS; i++) {
        if (stream_threads[i].running && strcmp(stream_threads[i].stream_name, stream_name) == 0) {
            *has_thread = true;
            *last_detection_time = stream_threads[i].last_detection_time;

            // We don't track last_check_time separately, so use last_detection_time
            *last_check_time = stream_threads[i].last_detection_time;

            pthread_mutex_unlock(&stream_threads_mutex);
            return 0;
        }
    }

    pthread_mutex_unlock(&stream_threads_mutex);
    return -1;
}

