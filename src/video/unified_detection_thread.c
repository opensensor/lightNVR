/**
 * Unified Detection Recording Thread Implementation
 *
 * This module implements a unified thread that handles packet reading,
 * circular buffering, object detection, and MP4 recording in a single
 * coordinated thread per stream.
 *
 * Key features:
 * - Single RTSP connection per stream
 * - Continuous circular buffer for pre-detection content
 * - Detection on keyframes only (configurable interval)
 * - Seamless pre-buffer flush when detection triggers
 * - Proper post-buffer countdown after last detection
 * - Self-healing with automatic reconnection
 */

#define _POSIX_C_SOURCE 200809L
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <pthread.h>
#include <time.h>
#include <errno.h>
#include <sys/stat.h>

#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <libavutil/avutil.h>
#include <libavutil/time.h>
#include <libavutil/imgutils.h>
#include <libswscale/swscale.h>

#include "core/logger.h"
#include "core/config.h"
#include "core/shutdown_coordinator.h"
#include "video/unified_detection_thread.h"
#include "video/packet_buffer.h"
#include "video/detection.h"
#include "video/detection_model.h"
#include "video/detection_result.h"
#include "video/api_detection.h"
#include "video/mp4_writer.h"
#include "video/mp4_writer_internal.h"
#include "video/streams.h"
#include "video/go2rtc/go2rtc_stream.h"
#include "database/db_recordings.h"
#include "database/db_detections.h"

// Reconnection settings
#define BASE_RECONNECT_DELAY_MS 500
#define MAX_RECONNECT_DELAY_MS 30000
#define MAX_PACKET_TIMEOUT_SEC 10

// Detection settings
#define DEFAULT_DETECTION_INTERVAL 5  // Process every 5th keyframe

// Global array of unified detection contexts
static unified_detection_ctx_t *detection_contexts[MAX_UNIFIED_DETECTION_THREADS] = {0};
static pthread_mutex_t contexts_mutex = PTHREAD_MUTEX_INITIALIZER;
static bool system_initialized = false;

// Forward declarations
static void *unified_detection_thread_func(void *arg);
static int connect_to_stream(unified_detection_ctx_t *ctx);
static void disconnect_from_stream(unified_detection_ctx_t *ctx);
static int process_packet(unified_detection_ctx_t *ctx, AVPacket *pkt);
static bool run_detection_on_frame(unified_detection_ctx_t *ctx, AVPacket *pkt);
static int udt_start_recording(unified_detection_ctx_t *ctx);
static int udt_stop_recording(unified_detection_ctx_t *ctx);
static int flush_prebuffer_to_recording(unified_detection_ctx_t *ctx);
static const char* state_to_string(unified_detection_state_t state);

/**
 * FFmpeg interrupt callback to allow cancellation of blocking operations
 * Returns 1 to abort, 0 to continue
 */
static int ffmpeg_interrupt_callback(void *opaque) {
    unified_detection_ctx_t *ctx = (unified_detection_ctx_t *)opaque;
    if (!ctx) return 1;  // Abort if no context

    // Check if we should stop
    if (!atomic_load(&ctx->running) || is_shutdown_initiated()) {
        return 1;  // Abort the operation
    }
    return 0;  // Continue
}

/**
 * Check if a model path indicates API-based detection
 * Returns true if the path is "api-detection" or starts with http:// or https://
 */
static bool is_api_detection(const char *model_path) {
    if (!model_path || model_path[0] == '\0') {
        return false;
    }
    if (strcmp(model_path, "api-detection") == 0) {
        return true;
    }
    if (strncmp(model_path, "http://", 7) == 0 || strncmp(model_path, "https://", 8) == 0) {
        return true;
    }
    return false;
}

/**
 * Initialize the unified detection thread system
 */
int init_unified_detection_system(void) {
    if (system_initialized) {
        log_warn("Unified detection system already initialized");
        return 0;
    }

    pthread_mutex_lock(&contexts_mutex);

    // Clear all context slots
    memset(detection_contexts, 0, sizeof(detection_contexts));

    // Initialize packet buffer pool if not already done
    extern config_t g_config;
    size_t memory_limit = 256;  // Default 256MB for all buffers
    if (init_packet_buffer_pool(memory_limit) != 0) {
        log_error("Failed to initialize packet buffer pool");
        pthread_mutex_unlock(&contexts_mutex);
        return -1;
    }

    system_initialized = true;
    pthread_mutex_unlock(&contexts_mutex);

    log_info("Unified detection system initialized");
    return 0;
}

/**
 * Shutdown the unified detection thread system
 */
void shutdown_unified_detection_system(void) {
    if (!system_initialized) {
        return;
    }

    log_info("Shutting down unified detection system");

    // First pass: Signal all threads to stop (without holding the lock for long)
    pthread_mutex_lock(&contexts_mutex);
    for (int i = 0; i < MAX_UNIFIED_DETECTION_THREADS; i++) {
        if (detection_contexts[i]) {
            unified_detection_ctx_t *ctx = detection_contexts[i];
            atomic_store(&ctx->running, 0);
            atomic_store(&ctx->state, UDT_STATE_STOPPING);
            log_info("Signaled unified detection thread %s to stop", ctx->stream_name);
        }
    }
    pthread_mutex_unlock(&contexts_mutex);

    // Wait for all threads to reach STOPPED state (up to 5 seconds total)
    int max_wait_iterations = 50;  // 50 * 100ms = 5 seconds
    for (int wait_iter = 0; wait_iter < max_wait_iterations; wait_iter++) {
        bool all_stopped = true;

        pthread_mutex_lock(&contexts_mutex);
        for (int i = 0; i < MAX_UNIFIED_DETECTION_THREADS; i++) {
            if (detection_contexts[i]) {
                unified_detection_state_t state = atomic_load(&detection_contexts[i]->state);
                if (state != UDT_STATE_STOPPED) {
                    all_stopped = false;
                    break;
                }
            }
        }
        pthread_mutex_unlock(&contexts_mutex);

        if (all_stopped) {
            log_info("All unified detection threads have stopped");
            break;
        }

        usleep(100000);  // 100ms

        if (wait_iter == max_wait_iterations - 1) {
            log_warn("Timeout waiting for unified detection threads to stop, proceeding with cleanup");
        }
    }

    // Second pass: Clean up contexts (threads should be stopped now)
    pthread_mutex_lock(&contexts_mutex);
    for (int i = 0; i < MAX_UNIFIED_DETECTION_THREADS; i++) {
        if (detection_contexts[i]) {
            unified_detection_ctx_t *ctx = detection_contexts[i];

            log_info("Cleaning up unified detection context for %s", ctx->stream_name);

            // Clean up resources
            if (ctx->packet_buffer) {
                destroy_packet_buffer(ctx->packet_buffer);
                ctx->packet_buffer = NULL;
            }
            if (ctx->mp4_writer) {
                mp4_writer_close(ctx->mp4_writer);
                ctx->mp4_writer = NULL;
            }

            // Only destroy mutex if thread has stopped
            if (atomic_load(&ctx->state) == UDT_STATE_STOPPED) {
                pthread_mutex_destroy(&ctx->mutex);
            } else {
                log_warn("Skipping mutex destroy for %s - thread may still be running", ctx->stream_name);
            }

            free(ctx);
            detection_contexts[i] = NULL;
        }
    }

    // Cleanup packet buffer pool
    cleanup_packet_buffer_pool();

    system_initialized = false;
    pthread_mutex_unlock(&contexts_mutex);

    log_info("Unified detection system shutdown complete");
}

/**
 * Convert state enum to string for logging
 */
static const char* state_to_string(unified_detection_state_t state) {
    switch (state) {
        case UDT_STATE_INITIALIZING: return "INITIALIZING";
        case UDT_STATE_CONNECTING: return "CONNECTING";
        case UDT_STATE_BUFFERING: return "BUFFERING";
        case UDT_STATE_RECORDING: return "RECORDING";
        case UDT_STATE_POST_BUFFER: return "POST_BUFFER";
        case UDT_STATE_RECONNECTING: return "RECONNECTING";
        case UDT_STATE_STOPPING: return "STOPPING";
        case UDT_STATE_STOPPED: return "STOPPED";
        default: return "UNKNOWN";
    }
}

/**
 * Find context by stream name
 */
static unified_detection_ctx_t* find_context_by_name(const char *stream_name) {
    for (int i = 0; i < MAX_UNIFIED_DETECTION_THREADS; i++) {
        if (detection_contexts[i] &&
            strcmp(detection_contexts[i]->stream_name, stream_name) == 0) {
            return detection_contexts[i];
        }
    }
    return NULL;
}

/**
 * Find empty slot for new context
 */
static int find_empty_slot(void) {
    for (int i = 0; i < MAX_UNIFIED_DETECTION_THREADS; i++) {
        if (!detection_contexts[i]) {
            return i;
        }
    }
    return -1;
}

/**
 * Start unified detection recording for a stream
 */
int start_unified_detection_thread(const char *stream_name, const char *model_path,
                                   float threshold, int pre_buffer_seconds,
                                   int post_buffer_seconds) {
    if (!stream_name || !model_path) {
        log_error("Invalid parameters for start_unified_detection_thread");
        return -1;
    }

    if (!system_initialized) {
        log_error("Unified detection system not initialized");
        return -1;
    }

    pthread_mutex_lock(&contexts_mutex);

    // Check if already running
    unified_detection_ctx_t *existing = find_context_by_name(stream_name);
    if (existing && atomic_load(&existing->running)) {
        log_info("Unified detection already running for stream %s", stream_name);
        pthread_mutex_unlock(&contexts_mutex);
        return 0;
    }

    // Find empty slot
    int slot = find_empty_slot();
    if (slot < 0) {
        log_error("No available slots for unified detection thread");
        pthread_mutex_unlock(&contexts_mutex);
        return -1;
    }

    // Get stream configuration
    stream_handle_t stream = get_stream_by_name(stream_name);
    if (!stream) {
        log_error("Stream %s not found", stream_name);
        pthread_mutex_unlock(&contexts_mutex);
        return -1;
    }

    stream_config_t config;
    if (get_stream_config(stream, &config) != 0) {
        log_error("Failed to get config for stream %s", stream_name);
        pthread_mutex_unlock(&contexts_mutex);
        return -1;
    }

    // Allocate context
    unified_detection_ctx_t *ctx = calloc(1, sizeof(unified_detection_ctx_t));
    if (!ctx) {
        log_error("Failed to allocate unified detection context");
        pthread_mutex_unlock(&contexts_mutex);
        return -1;
    }

    // Initialize context
    strncpy(ctx->stream_name, stream_name, sizeof(ctx->stream_name) - 1);
    strncpy(ctx->model_path, model_path, sizeof(ctx->model_path) - 1);
    ctx->detection_threshold = threshold;
    ctx->pre_buffer_seconds = pre_buffer_seconds > 0 ? pre_buffer_seconds : 10;
    ctx->post_buffer_seconds = post_buffer_seconds > 0 ? post_buffer_seconds : 5;
    ctx->detection_interval = config.detection_interval > 0 ? config.detection_interval : DEFAULT_DETECTION_INTERVAL;
    ctx->record_audio = config.record_audio;

    // Get RTSP URL from go2rtc
    if (!go2rtc_stream_get_rtsp_url(stream_name, ctx->rtsp_url, sizeof(ctx->rtsp_url))) {
        // Fall back to direct stream URL
        strncpy(ctx->rtsp_url, config.url, sizeof(ctx->rtsp_url) - 1);
    }

    // Set output directory
    config_t *global_config = get_streaming_config();
    if (global_config) {
        snprintf(ctx->output_dir, sizeof(ctx->output_dir), "%s/%s",
                 global_config->storage_path, stream_name);
        mkdir(ctx->output_dir, 0755);
    }

    // Initialize mutex
    if (pthread_mutex_init(&ctx->mutex, NULL) != 0) {
        log_error("Failed to initialize mutex for unified detection context");
        free(ctx);
        pthread_mutex_unlock(&contexts_mutex);
        return -1;
    }

    // Create circular buffer for pre-detection content
    ctx->packet_buffer = create_packet_buffer(stream_name, ctx->pre_buffer_seconds, BUFFER_MODE_MEMORY);
    if (!ctx->packet_buffer) {
        log_error("Failed to create pre-detection buffer for stream %s", stream_name);
        pthread_mutex_destroy(&ctx->mutex);
        free(ctx);
        pthread_mutex_unlock(&contexts_mutex);
        return -1;
    }

    // Initialize atomic variables
    atomic_store(&ctx->running, 1);
    atomic_store(&ctx->state, UDT_STATE_INITIALIZING);
    atomic_store(&ctx->last_packet_time, (int_fast64_t)time(NULL));
    atomic_store(&ctx->consecutive_failures, 0);

    // Store context in slot
    detection_contexts[slot] = ctx;

    // Create thread (detached)
    pthread_attr_t attr;
    pthread_attr_init(&attr);
    pthread_attr_setdetachstate(&attr, PTHREAD_CREATE_DETACHED);

    int result = pthread_create(&ctx->thread, &attr, unified_detection_thread_func, ctx);
    pthread_attr_destroy(&attr);

    if (result != 0) {
        log_error("Failed to create unified detection thread for %s: %s",
                  stream_name, strerror(result));
        destroy_packet_buffer(ctx->packet_buffer);
        pthread_mutex_destroy(&ctx->mutex);
        free(ctx);
        detection_contexts[slot] = NULL;
        pthread_mutex_unlock(&contexts_mutex);
        return -1;
    }

    pthread_mutex_unlock(&contexts_mutex);

    log_info("Started unified detection thread for stream %s (model=%s, threshold=%.2f, interval=%d, pre-buffer=%ds, post-buffer=%ds)",
             stream_name, ctx->model_path, ctx->detection_threshold, ctx->detection_interval,
             ctx->pre_buffer_seconds, ctx->post_buffer_seconds);

    return 0;
}

/**
 * Stop unified detection recording for a stream
 */
int stop_unified_detection_thread(const char *stream_name) {
    if (!stream_name) {
        return -1;
    }

    pthread_mutex_lock(&contexts_mutex);

    unified_detection_ctx_t *ctx = find_context_by_name(stream_name);
    if (!ctx) {
        log_warn("No unified detection thread found for stream %s", stream_name);
        pthread_mutex_unlock(&contexts_mutex);
        return -1;
    }

    // Signal thread to stop
    atomic_store(&ctx->running, 0);
    atomic_store(&ctx->state, UDT_STATE_STOPPING);

    log_info("Signaled unified detection thread for %s to stop", stream_name);

    pthread_mutex_unlock(&contexts_mutex);

    return 0;
}

/**
 * Check if unified detection is running for a stream
 */
bool is_unified_detection_running(const char *stream_name) {
    if (!stream_name) {
        return false;
    }

    pthread_mutex_lock(&contexts_mutex);

    unified_detection_ctx_t *ctx = find_context_by_name(stream_name);
    bool running = ctx && atomic_load(&ctx->running);

    pthread_mutex_unlock(&contexts_mutex);

    return running;
}

/**
 * Get the current state of a unified detection thread
 */
unified_detection_state_t get_unified_detection_state(const char *stream_name) {
    if (!stream_name) {
        return UDT_STATE_STOPPED;
    }

    pthread_mutex_lock(&contexts_mutex);

    unified_detection_ctx_t *ctx = find_context_by_name(stream_name);
    unified_detection_state_t state = ctx ? atomic_load(&ctx->state) : UDT_STATE_STOPPED;

    pthread_mutex_unlock(&contexts_mutex);

    return state;
}

/**
 * Get statistics for a unified detection thread
 */
int get_unified_detection_stats(const char *stream_name,
                                uint64_t *packets_processed,
                                uint64_t *detections,
                                uint64_t *recordings) {
    if (!stream_name) {
        return -1;
    }

    pthread_mutex_lock(&contexts_mutex);

    unified_detection_ctx_t *ctx = find_context_by_name(stream_name);
    if (!ctx) {
        pthread_mutex_unlock(&contexts_mutex);
        return -1;
    }

    pthread_mutex_lock(&ctx->mutex);
    if (packets_processed) *packets_processed = ctx->total_packets_processed;
    if (detections) *detections = ctx->total_detections;
    if (recordings) *recordings = ctx->total_recordings;
    pthread_mutex_unlock(&ctx->mutex);

    pthread_mutex_unlock(&contexts_mutex);

    return 0;
}


/**
 * Connect to RTSP stream
 */
static int connect_to_stream(unified_detection_ctx_t *ctx) {
    if (!ctx) return -1;

    log_info("[%s] Connecting to stream: %s", ctx->stream_name, ctx->rtsp_url);

    // Allocate format context
    ctx->input_ctx = avformat_alloc_context();
    if (!ctx->input_ctx) {
        log_error("[%s] Failed to allocate format context", ctx->stream_name);
        return -1;
    }

    // Set interrupt callback to allow cancellation during shutdown
    ctx->input_ctx->interrupt_callback.callback = ffmpeg_interrupt_callback;
    ctx->input_ctx->interrupt_callback.opaque = ctx;

    // Set RTSP options
    AVDictionary *opts = NULL;
    av_dict_set(&opts, "rtsp_transport", "tcp", 0);
    av_dict_set(&opts, "stimeout", "5000000", 0);  // 5 second timeout
    av_dict_set(&opts, "analyzeduration", "1000000", 0);
    av_dict_set(&opts, "probesize", "1000000", 0);

    // Open input
    int ret = avformat_open_input(&ctx->input_ctx, ctx->rtsp_url, NULL, &opts);
    av_dict_free(&opts);

    if (ret < 0) {
        char err_buf[AV_ERROR_MAX_STRING_SIZE];
        av_strerror(ret, err_buf, sizeof(err_buf));
        log_error("[%s] Failed to open input: %s", ctx->stream_name, err_buf);
        avformat_free_context(ctx->input_ctx);
        ctx->input_ctx = NULL;
        return -1;
    }

    // Find stream info
    ret = avformat_find_stream_info(ctx->input_ctx, NULL);
    if (ret < 0) {
        log_error("[%s] Failed to find stream info", ctx->stream_name);
        avformat_close_input(&ctx->input_ctx);
        return -1;
    }

    // Find video stream
    ctx->video_stream_idx = -1;
    ctx->audio_stream_idx = -1;

    for (unsigned int i = 0; i < ctx->input_ctx->nb_streams; i++) {
        AVCodecParameters *codecpar = ctx->input_ctx->streams[i]->codecpar;
        if (codecpar->codec_type == AVMEDIA_TYPE_VIDEO && ctx->video_stream_idx < 0) {
            ctx->video_stream_idx = i;
        } else if (codecpar->codec_type == AVMEDIA_TYPE_AUDIO && ctx->audio_stream_idx < 0) {
            ctx->audio_stream_idx = i;
        }
    }

    if (ctx->video_stream_idx < 0) {
        log_error("[%s] No video stream found", ctx->stream_name);
        avformat_close_input(&ctx->input_ctx);
        return -1;
    }

    // Set up decoder for detection
    AVStream *video_stream = ctx->input_ctx->streams[ctx->video_stream_idx];
    const AVCodec *decoder = avcodec_find_decoder(video_stream->codecpar->codec_id);
    if (!decoder) {
        log_error("[%s] Failed to find decoder", ctx->stream_name);
        avformat_close_input(&ctx->input_ctx);
        return -1;
    }

    ctx->decoder_ctx = avcodec_alloc_context3(decoder);
    if (!ctx->decoder_ctx) {
        log_error("[%s] Failed to allocate decoder context", ctx->stream_name);
        avformat_close_input(&ctx->input_ctx);
        return -1;
    }

    ret = avcodec_parameters_to_context(ctx->decoder_ctx, video_stream->codecpar);
    if (ret < 0) {
        log_error("[%s] Failed to copy codec parameters", ctx->stream_name);
        avcodec_free_context(&ctx->decoder_ctx);
        avformat_close_input(&ctx->input_ctx);
        return -1;
    }

    ret = avcodec_open2(ctx->decoder_ctx, decoder, NULL);
    if (ret < 0) {
        log_error("[%s] Failed to open decoder", ctx->stream_name);
        avcodec_free_context(&ctx->decoder_ctx);
        avformat_close_input(&ctx->input_ctx);
        return -1;
    }

    log_info("[%s] Connected successfully (video stream: %d, audio stream: %d)",
             ctx->stream_name, ctx->video_stream_idx, ctx->audio_stream_idx);

    return 0;
}

/**
 * Disconnect from RTSP stream
 */
static void disconnect_from_stream(unified_detection_ctx_t *ctx) {
    if (!ctx) return;

    if (ctx->decoder_ctx) {
        avcodec_free_context(&ctx->decoder_ctx);
        ctx->decoder_ctx = NULL;
    }

    if (ctx->input_ctx) {
        avformat_close_input(&ctx->input_ctx);
        ctx->input_ctx = NULL;
    }

    ctx->video_stream_idx = -1;
    ctx->audio_stream_idx = -1;

    log_info("[%s] Disconnected from stream", ctx->stream_name);
}

/**
 * Main unified detection thread function
 */
static void *unified_detection_thread_func(void *arg) {
    unified_detection_ctx_t *ctx = (unified_detection_ctx_t *)arg;
    if (!ctx) {
        log_error("NULL context passed to unified detection thread");
        return NULL;
    }

    char stream_name[MAX_STREAM_NAME];
    strncpy(stream_name, ctx->stream_name, sizeof(stream_name) - 1);
    stream_name[sizeof(stream_name) - 1] = '\0';

    log_info("[%s] Unified detection thread started", stream_name);

    unified_detection_state_t state = UDT_STATE_INITIALIZING;
    int reconnect_delay_ms = BASE_RECONNECT_DELAY_MS;
    AVPacket *pkt = av_packet_alloc();
    AVFrame *frame = av_frame_alloc();

    if (!pkt || !frame) {
        log_error("[%s] Failed to allocate packet/frame", stream_name);
        if (pkt) av_packet_free(&pkt);
        if (frame) av_frame_free(&frame);
        atomic_store(&ctx->state, UDT_STATE_STOPPED);
        return NULL;
    }

    // Main loop
    while (atomic_load(&ctx->running) && !is_shutdown_initiated()) {
        // Read current state from context (may have been changed by process_packet)
        state = atomic_load(&ctx->state);

        // State machine
        switch (state) {
            case UDT_STATE_INITIALIZING:
                log_info("[%s] State: INITIALIZING", stream_name);
                // TODO: Load detection model
                state = UDT_STATE_CONNECTING;
                break;

            case UDT_STATE_CONNECTING:
                log_info("[%s] State: CONNECTING (attempt %d)", stream_name, ctx->reconnect_attempt + 1);

                if (connect_to_stream(ctx) == 0) {
                    state = UDT_STATE_BUFFERING;
                    ctx->reconnect_attempt = 0;
                    reconnect_delay_ms = BASE_RECONNECT_DELAY_MS;
                    atomic_store(&ctx->last_packet_time, (int_fast64_t)time(NULL));
                } else {
                    ctx->reconnect_attempt++;
                    atomic_fetch_add(&ctx->consecutive_failures, 1);

                    // Exponential backoff
                    usleep(reconnect_delay_ms * 1000);
                    reconnect_delay_ms = reconnect_delay_ms * 2;
                    if (reconnect_delay_ms > MAX_RECONNECT_DELAY_MS) {
                        reconnect_delay_ms = MAX_RECONNECT_DELAY_MS;
                    }
                }
                break;

            case UDT_STATE_BUFFERING:
            case UDT_STATE_RECORDING:
            case UDT_STATE_POST_BUFFER:
                // Read packet
                if (av_read_frame(ctx->input_ctx, pkt) >= 0) {
                    atomic_store(&ctx->last_packet_time, (int_fast64_t)time(NULL));

                    // Process packet (buffer, detect, record)
                    process_packet(ctx, pkt);

                    // Re-read state after process_packet as it may have changed
                    // (e.g., detection triggered -> RECORDING, or post-buffer expired -> BUFFERING)
                    state = atomic_load(&ctx->state);

                    av_packet_unref(pkt);
                } else {
                    // Read error - check if timeout
                    time_t now = time(NULL);
                    time_t last = atomic_load(&ctx->last_packet_time);

                    if (now - last > MAX_PACKET_TIMEOUT_SEC) {
                        log_warn("[%s] Packet timeout, reconnecting", stream_name);
                        disconnect_from_stream(ctx);
                        state = UDT_STATE_RECONNECTING;
                    }
                }
                break;

            case UDT_STATE_RECONNECTING:
                log_info("[%s] State: RECONNECTING", stream_name);

                // Close any active recording
                if (ctx->mp4_writer) {
                    udt_stop_recording(ctx);
                }

                // Clear buffer (stale data)
                packet_buffer_clear(ctx->packet_buffer);

                state = UDT_STATE_CONNECTING;
                break;

            case UDT_STATE_STOPPING:
                log_info("[%s] State: STOPPING", stream_name);

                // Close recording if active
                if (ctx->mp4_writer) {
                    udt_stop_recording(ctx);
                }

                // Disconnect
                disconnect_from_stream(ctx);

                state = UDT_STATE_STOPPED;
                break;

            case UDT_STATE_STOPPED:
                // Exit loop
                atomic_store(&ctx->running, 0);
                break;
        }

        // Store any state changes made by the main loop back to ctx->state
        // (process_packet also updates ctx->state directly for RECORDING/POST_BUFFER transitions)
        atomic_store(&ctx->state, state);
    }

    // Cleanup
    av_packet_free(&pkt);
    av_frame_free(&frame);

    atomic_store(&ctx->state, UDT_STATE_STOPPED);
    log_info("[%s] Unified detection thread exiting", stream_name);

    // During shutdown, the shutdown_unified_detection_system() will clean up
    // During normal operation (stream disabled), we clean up here
    if (!is_shutdown_initiated()) {
        // Remove from contexts array
        pthread_mutex_lock(&contexts_mutex);
        for (int i = 0; i < MAX_UNIFIED_DETECTION_THREADS; i++) {
            if (detection_contexts[i] == ctx) {
                // Clean up resources
                if (ctx->packet_buffer) {
                    destroy_packet_buffer(ctx->packet_buffer);
                }
                pthread_mutex_destroy(&ctx->mutex);
                free(ctx);
                detection_contexts[i] = NULL;
                break;
            }
        }
        pthread_mutex_unlock(&contexts_mutex);
    }
    // During shutdown, just exit - shutdown handler will clean up

    return NULL;
}

// Context structure for flush callback
typedef struct {
    unified_detection_ctx_t *ctx;
    int packets_written;
    bool found_keyframe;
    bool writer_initialized;
} flush_callback_ctx_t;

/**
 * Callback function for flushing pre-buffer packets to MP4 writer
 */
static int flush_packet_callback(const AVPacket *packet, void *user_data) {
    flush_callback_ctx_t *flush_ctx = (flush_callback_ctx_t *)user_data;
    if (!flush_ctx || !flush_ctx->ctx || !packet) return -1;

    unified_detection_ctx_t *ctx = flush_ctx->ctx;

    // Skip until we find a keyframe (ensures valid MP4 start)
    if (!flush_ctx->found_keyframe) {
        if (packet->flags & AV_PKT_FLAG_KEY) {
            flush_ctx->found_keyframe = true;
        } else {
            return 0;  // Skip non-keyframe packets before first keyframe
        }
    }

    // Write packet to MP4 - we need the input stream for codec parameters
    if (ctx->mp4_writer && ctx->input_ctx) {
        AVStream *input_stream = NULL;
        if (packet->stream_index == ctx->video_stream_idx && ctx->video_stream_idx >= 0) {
            input_stream = ctx->input_ctx->streams[ctx->video_stream_idx];
        } else if (packet->stream_index == ctx->audio_stream_idx && ctx->audio_stream_idx >= 0) {
            input_stream = ctx->input_ctx->streams[ctx->audio_stream_idx];
        }

        if (input_stream) {
            // Initialize the MP4 writer on the first keyframe
            if (!flush_ctx->writer_initialized && !ctx->mp4_writer->is_initialized) {
                if (packet->stream_index == ctx->video_stream_idx && (packet->flags & AV_PKT_FLAG_KEY)) {
                    int init_ret = mp4_writer_initialize(ctx->mp4_writer, packet, input_stream);
                    if (init_ret < 0) {
                        log_error("[%s] Failed to initialize MP4 writer", ctx->stream_name);
                        return -1;
                    }
                    flush_ctx->writer_initialized = true;
                    log_info("[%s] MP4 writer initialized on first keyframe", ctx->stream_name);
                } else {
                    // Skip until we get a video keyframe to initialize
                    return 0;
                }
            }

            int ret = mp4_writer_write_packet(ctx->mp4_writer, packet, input_stream);
            if (ret == 0) {
                flush_ctx->packets_written++;
            }
        }
    }

    return 0;
}

/**
 * Process a packet - buffer it, run detection on keyframes, handle recording
 */
static int process_packet(unified_detection_ctx_t *ctx, AVPacket *pkt) {
    if (!ctx || !pkt) return -1;

    time_t now = time(NULL);
    unified_detection_state_t current_state = atomic_load(&ctx->state);
    bool is_video = (pkt->stream_index == ctx->video_stream_idx);
    bool is_keyframe = is_video && (pkt->flags & AV_PKT_FLAG_KEY);

    // Update statistics
    pthread_mutex_lock(&ctx->mutex);
    ctx->total_packets_processed++;
    pthread_mutex_unlock(&ctx->mutex);

    // Always add packets to circular buffer (for pre-detection content)
    // The buffer automatically evicts old packets when full
    packet_buffer_add_packet(ctx->packet_buffer, pkt, now);

    // If recording, write packet to MP4
    if (current_state == UDT_STATE_RECORDING || current_state == UDT_STATE_POST_BUFFER) {
        if (ctx->mp4_writer && ctx->input_ctx) {
            AVStream *input_stream = NULL;
            if (is_video && ctx->video_stream_idx >= 0) {
                input_stream = ctx->input_ctx->streams[ctx->video_stream_idx];
            } else if (!is_video && ctx->audio_stream_idx >= 0) {
                input_stream = ctx->input_ctx->streams[ctx->audio_stream_idx];
            }

            if (input_stream) {
                // Initialize writer if not yet initialized (safety check)
                if (!ctx->mp4_writer->is_initialized && is_video && is_keyframe) {
                    int init_ret = mp4_writer_initialize(ctx->mp4_writer, pkt, input_stream);
                    if (init_ret < 0) {
                        log_error("[%s] Failed to initialize MP4 writer during live recording", ctx->stream_name);
                    } else {
                        log_info("[%s] MP4 writer initialized during live recording", ctx->stream_name);
                    }
                }

                // Only write if initialized
                if (ctx->mp4_writer->is_initialized) {
                    mp4_writer_write_packet(ctx->mp4_writer, pkt, input_stream);
                }
            }
        }

        // Check if post-buffer time has expired
        if (current_state == UDT_STATE_POST_BUFFER) {
            if (now >= ctx->post_buffer_end_time) {
                log_info("[%s] Post-buffer complete, stopping recording", ctx->stream_name);
                udt_stop_recording(ctx);
                atomic_store(&ctx->state, UDT_STATE_BUFFERING);
            }
        }

        // Check if maximum recording duration exceeded (handles continuous detection)
        // This ensures recordings complete even when detection keeps triggering
        // Also check in POST_BUFFER state since recording is still active
        if ((current_state == UDT_STATE_RECORDING || current_state == UDT_STATE_POST_BUFFER) && ctx->mp4_writer) {
            time_t recording_duration = now - ctx->mp4_writer->creation_time;
            int max_duration = ctx->pre_buffer_seconds + ctx->post_buffer_seconds;

            // Log every 10 seconds to track progress
            if (recording_duration > 0 && (recording_duration % 10) == 0 && is_keyframe) {
                log_info("[%s] Detection recording: %ld/%d seconds elapsed (pre=%d, post=%d)",
                         ctx->stream_name, (long)recording_duration, max_duration,
                         ctx->pre_buffer_seconds, ctx->post_buffer_seconds);
            }

            if (recording_duration >= max_duration) {
                log_info("[%s] Maximum recording duration reached (%ld/%d seconds), completing recording",
                         ctx->stream_name, (long)recording_duration, max_duration);
                udt_stop_recording(ctx);
                atomic_store(&ctx->state, UDT_STATE_BUFFERING);
                // Note: If detection continues, a new recording will start on the next detection
            }
        }
    }

    // Run detection on keyframes (at configured interval)
    if (is_keyframe && current_state != UDT_STATE_POST_BUFFER) {
        ctx->keyframe_counter++;

        // Log every 10th keyframe to show detection is running
        if (ctx->keyframe_counter % 10 == 1) {
            log_debug("[%s] Keyframe %d/%d, model_path=%s, state=%d",
                     ctx->stream_name, ctx->keyframe_counter, ctx->detection_interval,
                     ctx->model_path, current_state);
        }

        if (ctx->keyframe_counter >= ctx->detection_interval) {
            ctx->keyframe_counter = 0;

            log_info("[%s] Running detection (interval=%d, model=%s)",
                    ctx->stream_name, ctx->detection_interval, ctx->model_path);

            // Decode frame and run detection
            bool detection_triggered = run_detection_on_frame(ctx, pkt);

            // If detection triggered
            if (detection_triggered) {
                ctx->last_detection_time = now;

                pthread_mutex_lock(&ctx->mutex);
                ctx->total_detections++;
                pthread_mutex_unlock(&ctx->mutex);

                // If not already recording, start recording
                if (current_state == UDT_STATE_BUFFERING) {
                    log_info("[%s] Detection triggered, starting recording", ctx->stream_name);

                    // Start recording first, then flush pre-buffer
                    if (udt_start_recording(ctx) == 0) {
                        flush_prebuffer_to_recording(ctx);
                        atomic_store(&ctx->state, UDT_STATE_RECORDING);
                    }
                }
                // If in post-buffer, go back to recording
                else if (current_state == UDT_STATE_POST_BUFFER) {
                    log_info("[%s] Detection during post-buffer, continuing recording", ctx->stream_name);
                    atomic_store(&ctx->state, UDT_STATE_RECORDING);
                }
            }
            // No detection - check if we should enter post-buffer
            else if (current_state == UDT_STATE_RECORDING) {
                // Check if enough time has passed since last detection
                if (now - ctx->last_detection_time > 2) {  // 2 second grace period
                    log_info("[%s] No detection, entering post-buffer (%d seconds)",
                             ctx->stream_name, ctx->post_buffer_seconds);
                    ctx->post_buffer_end_time = now + ctx->post_buffer_seconds;
                    atomic_store(&ctx->state, UDT_STATE_POST_BUFFER);
                }
            }
        }
    }

    return 0;
}

/**
 * Start recording - create MP4 writer
 */
static int udt_start_recording(unified_detection_ctx_t *ctx) {
    if (!ctx) return -1;

    // Ensure output directory exists
    struct stat st = {0};
    if (stat(ctx->output_dir, &st) == -1) {
        if (mkdir(ctx->output_dir, 0755) != 0) {
            log_error("[%s] Failed to create output directory: %s", ctx->stream_name, ctx->output_dir);
            return -1;
        }
    }

    // Generate output filename with timestamp
    time_t now = time(NULL);
    struct tm *tm_info = localtime(&now);
    char timestamp[32];
    strftime(timestamp, sizeof(timestamp), "%Y%m%d_%H%M%S", tm_info);

    snprintf(ctx->current_recording_path, sizeof(ctx->current_recording_path),
             "%s/detection_%s.mp4", ctx->output_dir, timestamp);

    log_info("[%s] Starting detection recording: %s", ctx->stream_name, ctx->current_recording_path);

    // Create MP4 writer
    ctx->mp4_writer = mp4_writer_create(ctx->current_recording_path, ctx->stream_name);
    if (!ctx->mp4_writer) {
        log_error("[%s] Failed to create MP4 writer", ctx->stream_name);
        return -1;
    }

    // Configure audio recording based on stream settings
    if (ctx->record_audio && ctx->audio_stream_idx >= 0) {
        mp4_writer_set_audio(ctx->mp4_writer, 1);
        log_info("[%s] Audio recording enabled for detection recording", ctx->stream_name);
    } else {
        mp4_writer_set_audio(ctx->mp4_writer, 0);
        if (ctx->record_audio && ctx->audio_stream_idx < 0) {
            log_warn("[%s] Audio recording requested but no audio stream found", ctx->stream_name);
        }
    }

    // Set trigger type to detection
    strncpy(ctx->mp4_writer->trigger_type, "detection", sizeof(ctx->mp4_writer->trigger_type) - 1);

    // Store recording start time
    ctx->mp4_writer->creation_time = now;

    // Add recording to database at START (so it appears in recordings list immediately)
    // It will be updated with end_time, size, and is_complete=true when recording stops
    recording_metadata_t metadata = {0};
    strncpy(metadata.file_path, ctx->current_recording_path, sizeof(metadata.file_path) - 1);
    strncpy(metadata.stream_name, ctx->stream_name, sizeof(metadata.stream_name) - 1);
    metadata.start_time = now;
    metadata.end_time = 0;  // Will be set when recording stops
    metadata.size_bytes = 0;  // Will be set when recording stops
    metadata.is_complete = false;  // Will be set to true when recording stops
    strncpy(metadata.trigger_type, "detection", sizeof(metadata.trigger_type) - 1);

    ctx->current_recording_id = add_recording_metadata(&metadata);
    if (ctx->current_recording_id > 0) {
        log_info("[%s] Added detection recording to database (ID: %lu) for file: %s",
                 ctx->stream_name, (unsigned long)ctx->current_recording_id, ctx->current_recording_path);
    } else {
        log_warn("[%s] Failed to add detection recording to database", ctx->stream_name);
    }

    pthread_mutex_lock(&ctx->mutex);
    ctx->total_recordings++;
    pthread_mutex_unlock(&ctx->mutex);

    log_info("[%s] Detection recording started successfully", ctx->stream_name);
    return 0;
}

/**
 * Stop recording - close MP4 writer and update database
 */
static int udt_stop_recording(unified_detection_ctx_t *ctx) {
    if (!ctx || !ctx->mp4_writer) return -1;

    log_info("[%s] Stopping detection recording: %s", ctx->stream_name, ctx->current_recording_path);

    time_t end_time = time(NULL);
    time_t start_time = ctx->mp4_writer->creation_time;
    int duration = (int)(end_time - start_time);

    // Get file size before closing
    struct stat st;
    int64_t file_size = 0;
    if (stat(ctx->current_recording_path, &st) == 0) {
        file_size = st.st_size;
    }

    // Close MP4 writer
    mp4_writer_close(ctx->mp4_writer);
    ctx->mp4_writer = NULL;

    // Update the existing database record (was created at recording start)
    if (ctx->current_recording_id > 0) {
        // Update the existing recording with end_time, size, and mark as complete
        if (update_recording_metadata(ctx->current_recording_id, end_time, file_size, true) == 0) {
            log_info("[%s] Recording updated in database (ID: %lu, duration: %ds, size: %ld bytes)",
                     ctx->stream_name, (unsigned long)ctx->current_recording_id, duration, (long)file_size);
        } else {
            log_warn("[%s] Failed to update recording in database (ID: %lu)",
                     ctx->stream_name, (unsigned long)ctx->current_recording_id);
        }
    } else if (ctx->current_recording_path[0] != '\0') {
        // Fallback: if no recording_id, try to add a new record (shouldn't happen normally)
        log_warn("[%s] No recording ID found, creating new database entry", ctx->stream_name);
        recording_metadata_t metadata = {0};
        strncpy(metadata.file_path, ctx->current_recording_path, sizeof(metadata.file_path) - 1);
        strncpy(metadata.stream_name, ctx->stream_name, sizeof(metadata.stream_name) - 1);
        metadata.start_time = start_time;
        metadata.end_time = end_time;
        metadata.size_bytes = file_size;
        metadata.is_complete = true;
        strncpy(metadata.trigger_type, "detection", sizeof(metadata.trigger_type) - 1);

        uint64_t recording_id = add_recording_metadata(&metadata);
        if (recording_id > 0) {
            log_info("[%s] Recording added to database with ID %lu (duration: %ds, size: %ld bytes)",
                     ctx->stream_name, (unsigned long)recording_id, duration, (long)file_size);
        } else {
            log_warn("[%s] Failed to add recording to database", ctx->stream_name);
        }
    }

    ctx->current_recording_path[0] = '\0';
    ctx->current_recording_id = 0;

    log_info("[%s] Detection recording stopped (duration: %d seconds)", ctx->stream_name, duration);
    return 0;
}

/**
 * Flush pre-buffer to recording
 * Called when detection triggers to write buffered packets to MP4
 */
static int flush_prebuffer_to_recording(unified_detection_ctx_t *ctx) {
    if (!ctx || !ctx->mp4_writer || !ctx->packet_buffer) return -1;

    log_info("[%s] Flushing pre-buffer to recording", ctx->stream_name);

    // Get buffer stats before flushing
    int count = 0;
    size_t memory = 0;
    int duration = 0;
    packet_buffer_get_stats(ctx->packet_buffer, &count, &memory, &duration);

    log_info("[%s] Pre-buffer contains %d packets (~%d seconds, %zu bytes)",
             ctx->stream_name, count, duration, memory);

    // Create flush context
    flush_callback_ctx_t flush_ctx = {
        .ctx = ctx,
        .packets_written = 0,
        .found_keyframe = false,
        .writer_initialized = false
    };

    // Flush all packets from buffer to MP4 writer
    int flushed = packet_buffer_flush(ctx->packet_buffer, flush_packet_callback, &flush_ctx);

    if (flushed >= 0) {
        log_info("[%s] Flushed %d packets to recording (%d written starting from keyframe)",
                 ctx->stream_name, flushed, flush_ctx.packets_written);
    } else {
        log_warn("[%s] Failed to flush pre-buffer", ctx->stream_name);
    }

    return 0;
}

/**
 * Run detection on a keyframe
 *
 * This function handles both API-based detection (like light-object-detect)
 * and embedded model detection (SOD). For API detection, it uses go2rtc
 * snapshots which is more efficient than decoding frames.
 *
 * @param ctx The unified detection context
 * @param pkt The video packet containing a keyframe (unused for API detection)
 * @return true if detection was triggered, false otherwise
 */
static bool run_detection_on_frame(unified_detection_ctx_t *ctx, AVPacket *pkt) {
    if (!ctx) return false;

    detection_result_t result;
    memset(&result, 0, sizeof(detection_result_t));

    // Check if this is API-based detection
    if (is_api_detection(ctx->model_path)) {
        // API detection - use go2rtc snapshot (more efficient, no frame decoding needed)
        log_debug("[%s] Running API detection via snapshot", ctx->stream_name);

        // The model_path contains either "api-detection" or an HTTP URL
        // detect_objects_api_snapshot handles the "api-detection" special case
        // by looking up g_config.api_detection_url
        int detect_ret = detect_objects_api_snapshot(ctx->model_path, ctx->stream_name,
                                                     &result, ctx->detection_threshold);

        if (detect_ret == -2) {
            // go2rtc snapshot failed - this is a transient error, don't log as error
            log_debug("[%s] go2rtc snapshot unavailable, skipping detection", ctx->stream_name);
            return false;
        }

        if (detect_ret != 0) {
            log_warn("[%s] API detection failed with error %d", ctx->stream_name, detect_ret);
            return false;
        }

        // Note: detect_objects_api_snapshot already handles:
        // - Filtering by zones
        // - Storing in database
        // - MQTT publishing

        // Check if any detections meet the threshold and trigger recording
        bool detection_triggered = false;
        for (int i = 0; i < result.count; i++) {
            if (result.detections[i].confidence >= ctx->detection_threshold) {
                detection_triggered = true;
                log_info("[%s] API Detection: %s (%.1f%%) at [%.2f, %.2f, %.2f, %.2f]",
                         ctx->stream_name,
                         result.detections[i].label,
                         result.detections[i].confidence * 100.0f,
                         result.detections[i].x,
                         result.detections[i].y,
                         result.detections[i].width,
                         result.detections[i].height);
            }
        }

        ctx->total_detections += result.count;
        return detection_triggered;
    }

    // Embedded model detection - requires frame decoding
    if (!pkt || !ctx->decoder_ctx) return false;

    // Check if we have a detection model loaded
    if (!ctx->model) {
        // Try to load the model if we have a path
        if (ctx->model_path[0] != '\0') {
            ctx->model = load_detection_model(ctx->model_path, ctx->detection_threshold);
            if (!ctx->model) {
                log_warn("[%s] Failed to load detection model: %s", ctx->stream_name, ctx->model_path);
                return false;
            }
            log_info("[%s] Loaded detection model: %s", ctx->stream_name, ctx->model_path);
        } else {
            return false;
        }
    }

    // Decode the packet to get a frame
    int ret = avcodec_send_packet(ctx->decoder_ctx, pkt);
    if (ret < 0) {
        return false;
    }

    AVFrame *frame = av_frame_alloc();
    if (!frame) {
        return false;
    }

    ret = avcodec_receive_frame(ctx->decoder_ctx, frame);
    if (ret < 0) {
        av_frame_free(&frame);
        return false;
    }

    // Convert frame to RGB for detection
    int width = frame->width;
    int height = frame->height;
    int channels = 3;  // RGB

    // Create software scaler for conversion
    struct SwsContext *sws_ctx = sws_getContext(
        width, height, frame->format,
        width, height, AV_PIX_FMT_RGB24,
        SWS_BILINEAR, NULL, NULL, NULL);

    if (!sws_ctx) {
        log_error("[%s] Failed to create sws context", ctx->stream_name);
        av_frame_free(&frame);
        return false;
    }

    // Allocate RGB buffer
    size_t rgb_buffer_size = width * height * channels;
    uint8_t *rgb_buffer = malloc(rgb_buffer_size);
    if (!rgb_buffer) {
        log_error("[%s] Failed to allocate RGB buffer", ctx->stream_name);
        sws_freeContext(sws_ctx);
        av_frame_free(&frame);
        return false;
    }

    // Convert frame to RGB
    uint8_t *rgb_data[4] = {rgb_buffer, NULL, NULL, NULL};
    int rgb_linesize[4] = {width * channels, 0, 0, 0};

    sws_scale(sws_ctx, (const uint8_t * const *)frame->data, frame->linesize,
              0, height, rgb_data, rgb_linesize);

    sws_freeContext(sws_ctx);
    av_frame_free(&frame);

    // Run detection
    int detect_ret = detect_objects(ctx->model, rgb_buffer, width, height, channels, &result);

    free(rgb_buffer);

    if (detect_ret != 0) {
        log_warn("[%s] Detection failed with error %d", ctx->stream_name, detect_ret);
        return false;
    }

    // Check if any detections meet the threshold
    bool detection_triggered = false;
    for (int i = 0; i < result.count; i++) {
        if (result.detections[i].confidence >= ctx->detection_threshold) {
            detection_triggered = true;
            log_info("[%s] Detection: %s (%.1f%%) at [%.2f, %.2f, %.2f, %.2f]",
                     ctx->stream_name,
                     result.detections[i].label,
                     result.detections[i].confidence * 100.0f,
                     result.detections[i].x,
                     result.detections[i].y,
                     result.detections[i].width,
                     result.detections[i].height);
        }
    }

    // Store detections in database if any were found
    if (result.count > 0) {
        time_t now = time(NULL);
        if (store_detections_in_db(ctx->stream_name, &result, now) != 0) {
            log_warn("[%s] Failed to store detections in database", ctx->stream_name);
        }
        ctx->total_detections += result.count;
    }

    return detection_triggered;
}
