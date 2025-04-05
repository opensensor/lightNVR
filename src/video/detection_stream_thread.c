#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <pthread.h>
#include <stdbool.h>
#include <sys/stat.h>
#include <dirent.h>
#include <time.h>

#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <libavutil/avutil.h>
#include <libavutil/imgutils.h>
#include <libswscale/swscale.h>

#include "core/logger.h"
#include "core/config.h"
#include "core/shutdown_coordinator.h"
#include "video/detection_stream_thread.h"
#include "video/detection_model.h"
#include "video/sod_integration.h"
#include "video/detection_result.h"
#include "video/detection_recording.h"
#include "video/detection_embedded.h"
#include "video/streams.h"
#include "video/hls_writer.h"
#include "video/hls/hls_unified_thread.h"

// Maximum number of streams we can handle
#define MAX_STREAM_THREADS 32

// Stream detection thread structure
typedef struct {
    pthread_t thread;
    char stream_name[MAX_STREAM_NAME];
    char model_path[MAX_PATH_LENGTH];
    detection_model_t model;
    float threshold;
    int detection_interval;
    bool running;
    pthread_mutex_t mutex;
    pthread_cond_t cond;
    char hls_dir[MAX_PATH_LENGTH];
    time_t last_detection_time;
    int component_id;
} stream_detection_thread_t;

// Array of stream detection threads
static stream_detection_thread_t stream_threads[MAX_STREAM_THREADS] = {0};
static pthread_mutex_t stream_threads_mutex = PTHREAD_MUTEX_INITIALIZER;
static bool system_initialized = false;

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

    // Update last detection time
    thread->last_detection_time = current_time;

    // Create detection result structure
    detection_result_t result;
    memset(&result, 0, sizeof(detection_result_t));

    // Run detection on the frame
    int detect_ret = -1;

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
            return -1;
        }
        log_info("[Stream %s] Successfully loaded detection model", thread->stream_name);
    }

    // Run detection
    log_info("[Stream %s] Running detection on frame (dimensions: %dx%d, channels: %d)",
            thread->stream_name, width, height, channels);

    detect_ret = detect_objects(thread->model, frame_data, width, height, channels, &result);

    pthread_mutex_unlock(&thread->mutex);

    if (detect_ret != 0) {
        log_error("[Stream %s] Detection failed (error code: %d)", thread->stream_name, detect_ret);
        pthread_mutex_unlock(&stream_threads_mutex);
        return -1;
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

    pthread_mutex_unlock(&stream_threads_mutex);
    return 0;
}

/**
 * Process an HLS segment file for detection
 */
static int process_segment_for_detection(stream_detection_thread_t *thread, const char *segment_path) {
    AVFormatContext *format_ctx = NULL;
    AVCodecContext *codec_ctx = NULL;
    AVFrame *frame = NULL;
    AVPacket *pkt = NULL;
    struct SwsContext *sws_ctx = NULL;
    int video_stream_idx = -1;
    int ret = -1;

    log_info("[Stream %s] Processing HLS segment for detection: %s",
             thread->stream_name, segment_path);

    // CRITICAL FIX: Double-check that the segment still exists before trying to open it
    // This prevents segmentation faults when trying to open deleted segments
    if (access(segment_path, F_OK) != 0) {
        log_warn("[Stream %s] Segment no longer exists before processing: %s",
                thread->stream_name, segment_path);
        return -1;
    }

    // Open input file
    if (avformat_open_input(&format_ctx, segment_path, NULL, NULL) != 0) {
        log_error("[Stream %s] Could not open segment file: %s",
                 thread->stream_name, segment_path);
        return -1;
    }

    // Find stream info
    if (avformat_find_stream_info(format_ctx, NULL) < 0) {
        log_error("[Stream %s] Could not find stream info in segment file: %s",
                 thread->stream_name, segment_path);
        avformat_close_input(&format_ctx);
        return -1;
    }

    // Find video stream
    for (int i = 0; i < format_ctx->nb_streams; i++) {
        if (format_ctx->streams[i]->codecpar->codec_type == AVMEDIA_TYPE_VIDEO) {
            video_stream_idx = i;
            break;
        }
    }

    if (video_stream_idx == -1) {
        log_error("[Stream %s] Could not find video stream in segment file: %s",
                 thread->stream_name, segment_path);
        avformat_close_input(&format_ctx);
        return -1;
    }

    // Get codec
    const AVCodec *codec = avcodec_find_decoder(format_ctx->streams[video_stream_idx]->codecpar->codec_id);
    if (!codec) {
        log_error("[Stream %s] Unsupported codec in segment file: %s",
                 thread->stream_name, segment_path);
        avformat_close_input(&format_ctx);
        return -1;
    }

    // Allocate codec context
    codec_ctx = avcodec_alloc_context3(codec);
    if (!codec_ctx) {
        log_error("[Stream %s] Could not allocate codec context for segment file: %s",
                 thread->stream_name, segment_path);
        avformat_close_input(&format_ctx);
        return -1;
    }

    // Copy codec parameters
    if (avcodec_parameters_to_context(codec_ctx, format_ctx->streams[video_stream_idx]->codecpar) < 0) {
        log_error("[Stream %s] Could not copy codec parameters for segment file: %s",
                 thread->stream_name, segment_path);
        avcodec_free_context(&codec_ctx);
        avformat_close_input(&format_ctx);
        return -1;
    }

    // Open codec
    if (avcodec_open2(codec_ctx, codec, NULL) < 0) {
        log_error("[Stream %s] Could not open codec for segment file: %s",
                 thread->stream_name, segment_path);
        avcodec_free_context(&codec_ctx);
        avformat_close_input(&format_ctx);
        return -1;
    }

    // Allocate frame and packet
    frame = av_frame_alloc();
    pkt = av_packet_alloc();
    if (!frame || !pkt) {
        log_error("[Stream %s] Could not allocate frame or packet for segment file: %s",
                 thread->stream_name, segment_path);
        if (frame) av_frame_free(&frame);
        if (pkt) av_packet_free(&pkt);
        avcodec_free_context(&codec_ctx);
        avformat_close_input(&format_ctx);
        return -1;
    }

    // Calculate segment duration
    float segment_duration = 0;
    if (format_ctx->duration != AV_NOPTS_VALUE) {
        segment_duration = format_ctx->duration / (float)AV_TIME_BASE;
    } else {
        // Default to 2 seconds if duration is not available
        segment_duration = 2.0f;
    }

    // CRITICAL FIX: Process ALL frames in the segment
    // This ensures we don't miss any objects due to frame sampling
    float frames_per_second = format_ctx->streams[video_stream_idx]->avg_frame_rate.num /
                             (float)format_ctx->streams[video_stream_idx]->avg_frame_rate.den;
    int total_frames = segment_duration * frames_per_second;

    log_info("[Stream %s] OPTIMIZATION: Processing only key frames (I-frames) to reduce CPU usage",
             thread->stream_name);

    log_info("[Stream %s] Segment duration: %.2f seconds, FPS: %.2f, Total frames: %d",
             thread->stream_name, segment_duration, frames_per_second, total_frames);

    // Read frames
    int frame_count = 0;
    int processed_frames = 0;

    while (av_read_frame(format_ctx, pkt) >= 0) {
        if (pkt->stream_index == video_stream_idx) {
            frame_count++;

            // Send packet to decoder
            ret = avcodec_send_packet(codec_ctx, pkt);
            if (ret < 0) {
                log_error("[Stream %s] Error sending packet to decoder for segment file: %s",
                         thread->stream_name, segment_path);
                av_packet_unref(pkt);
                continue;
            }

            // Receive frame from decoder
            ret = avcodec_receive_frame(codec_ctx, frame);
            if (ret == AVERROR(EAGAIN) || ret == AVERROR_EOF) {
                av_packet_unref(pkt);
                continue;
            } else if (ret < 0) {
                log_error("[Stream %s] Error receiving frame from decoder for segment file: %s",
                         thread->stream_name, segment_path);
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
                    int detect_ret = detect_objects(thread->model, rgb_buffer, target_width, target_height, channels, &result);

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

    log_info("[Stream %s] Processed %d frames out of %d total frames from segment file: %s",
             thread->stream_name, processed_frames, frame_count, segment_path);

    // Cleanup
    av_frame_free(&frame);
    av_packet_free(&pkt);
    avcodec_free_context(&codec_ctx);
    avformat_close_input(&format_ctx);

    return 0;
}

/**
 * Check for new HLS segments in the stream's HLS directory
 * This function has been refactored to ensure each detection thread only monitors its own stream
 * Added retry mechanism and improved robustness for handling HLS writer failures
 */
// Global variable for startup delay
static time_t global_startup_delay_end = 0;

static void check_for_new_segments(stream_detection_thread_t *thread) {
    DIR *dir;
    struct dirent *entry;
    struct stat st;
    time_t current_time = time(NULL);
    static time_t last_warning_time = 0;
    static int consecutive_failures = 0;
    static bool first_check = true;

    // Check if we're still in the startup delay period
    if (global_startup_delay_end > 0 && current_time < global_startup_delay_end) {
        log_info("[Stream %s] In startup delay period, waiting %ld more seconds before processing segments",
                thread->stream_name, global_startup_delay_end - current_time);
        return;
    }

    log_info("[Stream %s] Checking for new segments in HLS directory", thread->stream_name);

    // We'll always check for segments, but we'll only process them if enough time has passed
    // This ensures we're constantly monitoring for new segments
    if (thread->last_detection_time > 0) {
        time_t time_since_last = current_time - thread->last_detection_time;
        if (time_since_last < thread->detection_interval) {
            // Not enough time has passed for detection, but we'll still check for segments
            log_info("[Stream %s] Checking for segments (last detection was %ld seconds ago, interval: %d seconds)",
                     thread->stream_name, time_since_last, thread->detection_interval);
            // We don't return here - we continue to check for segments
        }
    }

    // Check if the HLS writer is recording for this stream
    bool hls_writer_recording = false;
    stream_handle_t stream = get_stream_by_name(thread->stream_name);
    if (!stream) {
        // Only log a warning every 60 seconds to avoid log spam
        if (current_time - last_warning_time > 60 || first_check) {
            log_warn("[Stream %s] Failed to get stream handle, but will still check for segments", thread->stream_name);
            last_warning_time = current_time;
        }
    } else {
        // Get the HLS writer
        hls_writer_t *writer = get_stream_hls_writer(stream);
        if (writer) {
            // Check if the HLS writer is recording
            hls_writer_recording = is_hls_stream_active(thread->stream_name);
            if (!hls_writer_recording) {
                // Only log a warning every 60 seconds to avoid log spam
                if (current_time - last_warning_time > 60 || first_check) {
                    log_warn("[Stream %s] HLS writer is not recording, attempting to restart...", thread->stream_name);
                    last_warning_time = current_time;

                    // Try to restart the HLS writer if it's not recording
                    // This is a more proactive approach to handling stream failures
                    stream_config_t config;
                    if (get_stream_config(stream, &config) == 0) {
                        // Stop and restart the HLS stream
                        if (config.url[0] != '\0') {
                            log_info("[Stream %s] Attempting to restart HLS stream with URL: %s",
                                    thread->stream_name, config.url);
                            stop_hls_stream(thread->stream_name);

                            // Wait a short time before restarting
                            usleep(500000); // 500ms

                            // Restart the HLS stream
                            start_hls_stream(thread->stream_name);
                        }
                    }
                }
            } else {
                log_info("[Stream %s] HLS writer is recording, checking for new segments", thread->stream_name);
                consecutive_failures = 0; // Reset failure counter when HLS writer is recording
            }
        } else {
            // Only log a warning every 60 seconds to avoid log spam
            if (current_time - last_warning_time > 60 || first_check) {
                log_warn("[Stream %s] No HLS writer available, but will still check for segments", thread->stream_name);
                last_warning_time = current_time;
            }
        }
    }

    // CRITICAL FIX: Check both possible HLS directory paths
    if (thread->hls_dir[0] == '\0') {
        log_error("[Stream %s] HLS directory path is empty", thread->stream_name);
        return;
    }

    // Try both possible HLS directory paths
    char alt_hls_dir[MAX_PATH_LENGTH];

    // If the current path is /tmp/lightnvr/hls/hls/stream_name, try /tmp/lightnvr/hls/stream_name
    // If the current path is /tmp/lightnvr/hls/stream_name, try /tmp/lightnvr/hls/hls/stream_name
    if (strstr(thread->hls_dir, "/hls/hls/")) {
        // Current path has double hls, try with single hls
        char *pos = strstr(thread->hls_dir, "/hls/hls/");
        snprintf(alt_hls_dir, MAX_PATH_LENGTH, "%.*s/hls/%s",
                (int)(pos - thread->hls_dir), thread->hls_dir,
                pos + strlen("/hls/hls/"));
    } else if (strstr(thread->hls_dir, "/hls/")) {
        // Current path has single hls, try with double hls
        char *pos = strstr(thread->hls_dir, "/hls/");
        snprintf(alt_hls_dir, MAX_PATH_LENGTH, "%.*s/hls/hls/%s",
                (int)(pos - thread->hls_dir), thread->hls_dir,
                pos + strlen("/hls/"));
    } else {
        // Unexpected path format, just use the original
        strncpy(alt_hls_dir, thread->hls_dir, MAX_PATH_LENGTH - 1);
        alt_hls_dir[MAX_PATH_LENGTH - 1] = '\0';
    }

    log_info("[Stream %s] CRITICAL FIX: Checking both HLS directory paths:", thread->stream_name);
    log_info("[Stream %s]   Primary: %s", thread->stream_name, thread->hls_dir);
    log_info("[Stream %s]   Alternative: %s", thread->stream_name, alt_hls_dir);

    // Check if the alternative directory exists and has segments
    DIR *alt_dir = opendir(alt_hls_dir);
    if (alt_dir) {
        struct dirent *alt_entry;
        int alt_segment_count = 0;

        while ((alt_entry = readdir(alt_dir)) != NULL) {
            if (strstr(alt_entry->d_name, ".ts") || strstr(alt_entry->d_name, ".m4s")) {
                alt_segment_count++;
                break;  // We only need to know if there's at least one segment
            }
        }

        closedir(alt_dir);

        if (alt_segment_count > 0) {
            log_info("[Stream %s] Found segments in alternative directory, switching to: %s",
                    thread->stream_name, alt_hls_dir);
            strncpy(thread->hls_dir, alt_hls_dir, MAX_PATH_LENGTH - 1);
            thread->hls_dir[MAX_PATH_LENGTH - 1] = '\0';
        }
    }

    log_info("[Stream %s] Using HLS directory: %s", thread->stream_name, thread->hls_dir);

    // Open the HLS directory
    dir = opendir(thread->hls_dir);
    if (!dir) {
        // Only log an error every 60 seconds to avoid log spam
        if (current_time - last_warning_time > 60 || first_check) {
            log_error("[Stream %s] Failed to open HLS directory: %s (error: %s)",
                     thread->stream_name, thread->hls_dir, strerror(errno));
            last_warning_time = current_time;
        }

        consecutive_failures++;

        // If we've failed too many times, try to create the directory
        if (consecutive_failures > 10) {
            log_warn("[Stream %s] Too many consecutive failures, trying to create HLS directory", thread->stream_name);
            if (mkdir(thread->hls_dir, 0755) == 0) {
                log_info("[Stream %s] Successfully created HLS directory: %s", thread->stream_name, thread->hls_dir);
                consecutive_failures = 0;
            } else {
                log_error("[Stream %s] Failed to create HLS directory: %s (error: %s)",
                         thread->stream_name, thread->hls_dir, strerror(errno));
            }
        }

        first_check = false;
        return;
    }

    // Find the newest .ts or .m4s segment file
    char newest_segment[MAX_PATH_LENGTH] = {0};
    time_t newest_time = 0;
    int segment_count = 0;

    log_info("[Stream %s] Scanning directory for segments: %s", thread->stream_name, thread->hls_dir);

    while ((entry = readdir(dir)) != NULL) {
        // Check for both .ts and .m4s files (different HLS segment formats)
        if (!strstr(entry->d_name, ".ts") && !strstr(entry->d_name, ".m4s")) {
            continue;
        }

        segment_count++;

        // Construct full path
        char segment_path[MAX_PATH_LENGTH];
        snprintf(segment_path, MAX_PATH_LENGTH, "%s/%s", thread->hls_dir, entry->d_name);

        log_info("[Stream %s] Found segment file: %s", thread->stream_name, entry->d_name);

        // Get file stats
        if (stat(segment_path, &st) == 0) {
            // Check if this is the newest file
            if (st.st_mtime > newest_time) {
                newest_time = st.st_mtime;
                strncpy(newest_segment, segment_path, MAX_PATH_LENGTH - 1);
                newest_segment[MAX_PATH_LENGTH - 1] = '\0';
                log_info("[Stream %s] New newest segment: %s (mtime: %ld)",
                        thread->stream_name, segment_path, (long)st.st_mtime);
            }
        }
    }

    closedir(dir);

    if (segment_count == 0) {
        // Only log a warning every 60 seconds to avoid log spam
        if (current_time - last_warning_time > 60 || first_check) {
            log_warn("[Stream %s] No segments found in directory: %s", thread->stream_name, thread->hls_dir);
            last_warning_time = current_time;

            // CRITICAL FIX: Check if the HLS writer is recording
            stream_handle_t stream = get_stream_by_name(thread->stream_name);
            if (stream) {
                hls_writer_t *writer = get_stream_hls_writer(stream);
                if (writer) {
                    bool is_recording = is_hls_stream_active(thread->stream_name);
                    log_info("[Stream %s] HLS writer recording status: %s",
                            thread->stream_name, is_recording ? "RECORDING" : "NOT RECORDING");

                    if (!is_recording) {
                        // Try to restart the HLS writer
                        stream_config_t config;
                        if (get_stream_config(stream, &config) == 0 && config.url[0] != '\0') {
                            log_info("[Stream %s] Attempting to restart HLS writer with URL: %s",
                                    thread->stream_name, config.url);

                            // Stop and restart the HLS stream
                            log_info("[Stream %s] Attempting to restart HLS stream", thread->stream_name);
                            stop_hls_stream(thread->stream_name);

                            // Wait a short time before restarting
                            usleep(500000); // 500ms

                            // Restart the HLS stream
                            start_hls_stream(thread->stream_name);
                            log_info("[Stream %s] HLS stream restart attempted", thread->stream_name);
                        }
                    }
                }
            }
        }

        consecutive_failures++;
        first_check = false;

        // CRITICAL FIX: Don't return if no segments are found
        // Instead, continue running the thread and check again later
        // This ensures the thread stays active even when no segments are available yet
        log_info("[Stream %s] No segments found, but continuing to run detection thread", thread->stream_name);
        return;
    }

    // Reset failure counter when we find segments
    consecutive_failures = 0;

    log_info("[Stream %s] Found %d segments, newest segment time: %s",
             thread->stream_name, segment_count, ctime(&newest_time));

    // If we found a segment, process it regardless of age
    // This is a critical fix to ensure segments are always processed
    if (newest_segment[0] != '\0') {
        // Verify the segment still exists before processing
        if (access(newest_segment, F_OK) == 0) {
            // Track the last processed segment to avoid processing the same one repeatedly
            static char last_processed_segment[MAX_PATH_LENGTH] = {0};

            // CRITICAL FIX: Always process the newest segment if it's different from the last one
            // This ensures we're always processing segments regardless of the detection interval
            bool should_process = (strcmp(newest_segment, last_processed_segment) != 0);

            // Log the decision
            if (should_process) {
                log_info("[Stream %s] Processing new segment (different from last processed)", thread->stream_name);
            } else {
                log_info("[Stream %s] Skipping segment processing (same as last processed): %s",
                         thread->stream_name, newest_segment);
            }

            if (should_process) {
                log_info("[Stream %s] Processing segment: %s (age: %ld seconds)",
                        thread->stream_name, newest_segment, current_time - newest_time);

                int result = process_segment_for_detection(thread, newest_segment);
                if (result == 0) {
                    log_info("[Stream %s] Successfully processed segment: %s", thread->stream_name, newest_segment);

                    // Update the last processed segment
                    strncpy(last_processed_segment, newest_segment, MAX_PATH_LENGTH - 1);
                    last_processed_segment[MAX_PATH_LENGTH - 1] = '\0';

                    // Update last detection time
                    thread->last_detection_time = current_time;
                } else {
                    log_error("[Stream %s] Failed to process segment: %s (error code: %d)",
                             thread->stream_name, newest_segment, result);
                }
            } else {
                log_debug("[Stream %s] Skipping segment processing, same as last processed or too soon: %s",
                         thread->stream_name, newest_segment);
            }
        } else {
            log_warn("[Stream %s] Segment no longer exists: %s", thread->stream_name, newest_segment);
        }
    } else {
        // Only log a warning every 60 seconds to avoid log spam
        if (current_time - last_warning_time > 60 || first_check) {
            log_warn("[Stream %s] No valid segment found in directory: %s", thread->stream_name, thread->hls_dir);
            last_warning_time = current_time;
        }
    }

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
                return NULL;
            }
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

            // Also log the thread status to help with debugging
            log_info("[Stream %s] Thread status: model loaded: %s, last detection: %s, interval: %d seconds",
                    thread->stream_name,
                    thread->model ? "yes" : "no",
                    thread->last_detection_time > 0 ? ctime(&thread->last_detection_time) : "never",
                    thread->detection_interval);
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

        // Adaptive sleep based on consecutive empty checks
        // Sleep less if we're actively finding segments, more if we're not
        int sleep_time = 500000; // Default 500ms
        if (consecutive_empty_checks > 20) {
            sleep_time = 1000000; // 1 second if we've had many empty checks
        } else if (consecutive_empty_checks < 5) {
            sleep_time = 250000; // 250ms if we're actively finding segments
        }

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
            if (stream_threads[i].model) {
                log_info("Ensuring model cleanup during shutdown for stream %s", stream_threads[i].stream_name);

                // Get the model type to check if it's a SOD model
                const char *model_type = get_model_type_from_handle(stream_threads[i].model);

                // Use our enhanced cleanup for SOD models to prevent memory leaks
                if (strcmp(model_type, MODEL_TYPE_SOD) == 0) {
                    log_info("Using enhanced SOD model cleanup to prevent memory leaks");
                    ensure_sod_model_cleanup(stream_threads[i].model);
                    stream_threads[i].model = NULL;
                }
            }
            pthread_mutex_unlock(&stream_threads[i].mutex);

            // Now stop the thread
            stream_threads[i].running = false;
            pthread_join(stream_threads[i].thread, NULL);

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
            if (stream_threads[i].model) {
                log_info("Ensuring model cleanup before stopping thread for stream %s", stream_name);

                // Get the model type to check if it's a SOD model
                const char *model_type = get_model_type_from_handle(stream_threads[i].model);

                // Use our enhanced cleanup for SOD models to prevent memory leaks
                if (strcmp(model_type, MODEL_TYPE_SOD) == 0) {
                    log_info("Using enhanced SOD model cleanup to prevent memory leaks");
                    ensure_sod_model_cleanup(stream_threads[i].model);
                }
                // Don't set model to NULL here as the thread will handle that
            }
            pthread_mutex_unlock(&stream_threads[i].mutex);

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
