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
#include <libavutil/log.h>
#include <libavutil/time.h>
#include <libavutil/imgutils.h>
#include <libswscale/swscale.h>

#include "core/logger.h"
#include "core/config.h"
#include "core/path_utils.h"
#include "core/shutdown_coordinator.h"
#include "core/mqtt_client.h"
#include "utils/strings.h"
#include "video/unified_detection_thread.h"
#include "video/packet_buffer.h"
#include "video/detection.h"
#include "video/detection_model.h"
#include "video/detection_result.h"
#include "video/api_detection.h"
#include "video/motion_detection.h"
#include "video/onvif_detection.h"
#include "video/zone_filter.h"
#include "video/mp4_writer.h"
#include "video/mp4_writer_internal.h"
#include "video/mp4_recording.h"
#include "video/streams.h"
#include "video/stream_state.h"
#include "video/go2rtc/go2rtc_stream.h"
#include "video/go2rtc/go2rtc_snapshot.h"
#include "video/go2rtc/go2rtc_integration.h"
#include "database/db_recordings.h"
#include "database/db_detections.h"
#include "database/db_streams.h"
#include "core/url_utils.h"
#include "storage/storage_manager_streams_cache.h"
#include "telemetry/stream_metrics.h"

// Reconnection settings
#define BASE_RECONNECT_DELAY_MS 500
#define MAX_RECONNECT_DELAY_MS 30000
#define MAX_PACKET_TIMEOUT_SEC 10

// Detection error codes
// Returned by detect_objects_api_snapshot when go2rtc snapshot is unavailable
#define DETECT_SNAPSHOT_UNAVAILABLE -2

// Detection settings
// Seconds between detection checks; used as a fallback when no valid interval
// is configured via the application's stream/detection settings (i.e. when
// the configured detection interval is missing or <= 0).
#define DEFAULT_DETECTION_INTERVAL 5
/* DETECTION_GRACE_PERIOD_SEC is no longer a compile-time constant.
 * Use g_config.detection_grace_period (configured via [detection] grace_period). */

// Video/default FPS settings
// Conservative low-end fallback for cameras that omit FPS in SDP.
// Intentionally underestimates typical 25/30 FPS to avoid overestimating
// duration/bitrate when the actual FPS is unknown.
#define DEFAULT_FPS_FALLBACK 15
#define FPS_MEASUREMENT_WINDOW_SEC 5  // Seconds of frame arrivals to measure before refining provisional FPS

// Detection recording settings
#define DEFAULT_MIN_DETECTION_RECORDING_DURATION 10  // Default minimum total duration (seconds) for detection recordings (pre_buffer + post_buffer)
#define ONVIF_MOTION_HOLD_SECS 15  // Seconds to hold onvif_motion_detected=1 after last confirmed motion event,
                                   // preventing a shared-subscription race where the sibling thread clears the
                                   // flag before the UDT reads it (must be > detection_interval, currently 10s)

// Motion detection settings
static const float DEFAULT_MOTION_SENSITIVITY = 0.15f;  // Fallback sensitivity if threshold is unset or out of range
static const float DEFAULT_MOTION_MIN_AREA_RATIO = 0.005f;  // Minimum fraction of frame area that must change to qualify as motion
static const int   DEFAULT_MOTION_MIN_CONSECUTIVE_FRAMES = 3;  // Minimum consecutive frames of motion before triggering

// Global array of unified detection contexts
static unified_detection_ctx_t *detection_contexts[MAX_UNIFIED_DETECTION_THREADS] = {0};
static pthread_mutex_t contexts_mutex = PTHREAD_MUTEX_INITIALIZER;
static bool system_initialized = false;

// Forward declarations
static void *unified_detection_thread_func(void *arg);
static void *detection_stream_thread_func(void *arg);
static int   start_detection_stream_thread(unified_detection_ctx_t *ctx);
static void  stop_detection_stream_thread(unified_detection_ctx_t *ctx);
/* Detection layer — pure inference, no DB/state side effects */
static bool  detect_on_decoded_frame(unified_detection_ctx_t *ctx,
                                     AVFrame *frame, time_t now,
                                     detection_result_t *result);
static bool  run_detection_on_frame(unified_detection_ctx_t *ctx, AVPacket *pkt,
                                    time_t frame_timestamp);
/* Action layer */
static void  report_detections(unified_detection_ctx_t *ctx,
                                const detection_result_t *result, time_t now);
static void  handle_recording_state(unified_detection_ctx_t *ctx,
                                    bool detected, time_t now);
static int connect_to_stream(unified_detection_ctx_t *ctx);
static void disconnect_from_stream(unified_detection_ctx_t *ctx);
static int process_packet(unified_detection_ctx_t *ctx, AVPacket *pkt);
static int udt_start_recording(unified_detection_ctx_t *ctx);
static int udt_stop_recording(unified_detection_ctx_t *ctx);
static int flush_prebuffer_to_recording(unified_detection_ctx_t *ctx);
// ONVIF async detection thread helpers (defined before start_unified_detection_thread)
static void *onvif_detection_thread_func(void *arg);
static int   start_onvif_detection_thread(unified_detection_ctx_t *ctx);
static void  stop_onvif_detection_thread(unified_detection_ctx_t *ctx);
/**
 * Determine the actual API URL to use for detection based on the configured
 * model path and global configuration.
 *
 * For a model_path of "api-detection", this resolves to g_config.api_detection_url
 * and validates that it is configured. For all other model paths, the model_path
 * itself is returned.
 *
 * @param stream_name  Name of the stream for logging purposes.
 * @param model_path   The configured model path.
 * @return             The resolved API URL, or NULL if configuration is invalid.
 */
static const char *get_actual_api_url(const char *stream_name, const char *model_path)
{
    const char *actual_api_url = model_path;
    if (model_path != NULL && strcmp(model_path, "api-detection") == 0) {
        if (g_config.api_detection_url == NULL || g_config.api_detection_url[0] == '\0') {
            log_error("[%s] api_detection_url is not configured for api-detection model_path", stream_name);
            return NULL;
        }
        actual_api_url = g_config.api_detection_url;
    }
    return actual_api_url;
}
/**
 * Helper to update stored video parameters, distinguishing between
 * container-derived FPS values and provisional fallbacks that should
 * later be refined using runtime measurement of frame arrival times.
 */
static void udt_update_stream_video_params(unified_detection_ctx_t *ctx,
                                           int det_width,
                                           int det_height,
                                           int det_fps,
                                           const char *det_codec,
                                           bool fps_is_provisional);
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
 * Check if a model path indicates built-in motion detection
 * Returns true if the path is exactly "motion"
 */
static bool is_motion_detection_model(const char *model_path) {
    if (!model_path || model_path[0] == '\0') {
        return false;
    }
    return strcmp(model_path, "motion") == 0;
}

/**
 * Check if a model path indicates ONVIF event-based detection
 * Returns true if the path is exactly "onvif"
 */
static bool is_onvif_detection_model(const char *model_path) {
    if (!model_path || model_path[0] == '\0') {
        return false;
    }
    return strcmp(model_path, "onvif") == 0;
}

/**
 * Derive an ONVIF base URL (http[s]://host[:port]) from a stream URL.
 *
 * Delegates to url_build_onvif_service_url() with no service path to obtain
 * just the scheme + host + port, stripping credentials, query, and fragment.
 * Scheme mapping (rtsp→http, rtsps→https) and standard port mapping
 * (554→80, 322→443) are applied by the common helper.
 *
 * Examples:
 *   rtsp://admin:pass@192.168.1.100:554/stream  (port=0)    →  http://192.168.1.100:80
 *   rtsp://admin:pass@192.168.1.100:554/stream  (port=8080) →  http://192.168.1.100:8080
 *   rtsps://192.168.1.100/stream                (port=0)    →  https://192.168.1.100
 *   onvif://192.168.1.100/onvif/device_service  (port=0)    →  http://192.168.1.100
 */
static void extract_onvif_base_url(const char *stream_url, int onvif_port, char *onvif_url, size_t onvif_url_size) {
    if (!stream_url || !onvif_url || onvif_url_size == 0) {
        return;
    }
    onvif_url[0] = '\0';
    url_build_onvif_service_url(stream_url, onvif_port, NULL, onvif_url, onvif_url_size);
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

    // Initialize packet buffer pool sized to actual detection-stream requirements
    size_t memory_limit = calculate_packet_buffer_pool_size();
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
    int already_stopped_count = 0;
    int threads_to_stop = 0;
    pthread_mutex_lock(&contexts_mutex);
    for (int i = 0; i < MAX_UNIFIED_DETECTION_THREADS; i++) {
        if (detection_contexts[i]) {
            unified_detection_ctx_t *ctx = detection_contexts[i];

            // Check current state BEFORE modifying
            unified_detection_state_t current_state = atomic_load(&ctx->state);

            // If thread has already stopped, don't reset its state
            if (current_state == UDT_STATE_STOPPED) {
                already_stopped_count++;
                log_info("Unified detection thread %s already stopped (state=%d)",
                         ctx->stream_name, current_state);
                continue;
            }

            atomic_store(&ctx->running, 0);

            // Only update state to stopping if not already stopping
            if (current_state != UDT_STATE_STOPPING) {
                atomic_store(&ctx->state, UDT_STATE_STOPPING);
            }

            threads_to_stop++;
            log_info("Signaled unified detection thread %s to stop (was state=%d)",
                     ctx->stream_name, current_state);
        }
    }
    pthread_mutex_unlock(&contexts_mutex);

    if (already_stopped_count > 0) {
        log_info("Found %d unified detection threads already stopped", already_stopped_count);
    }

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

            // Stop the ONVIF detection thread before freeing ctx.
            // Must happen before free() to avoid use-after-free in the thread.
            if (is_onvif_detection_model(ctx->model_path)) {
                stop_onvif_detection_thread(ctx);
            }

            // Stop the detection stream thread (no-op if never started).
            stop_detection_stream_thread(ctx);

            // Clean up resources
            if (ctx->packet_buffer) {
                destroy_packet_buffer(ctx->packet_buffer);
                ctx->packet_buffer = NULL;
            }
            // Note: mp4_writer should have been closed by udt_stop_recording() in the thread
            // This is a safety fallback - if thread didn't close it properly, close it now
            // but we won't have proper database update in this case
            if (ctx->mp4_writer) {
                log_warn("MP4 writer still active during shutdown cleanup for %s - closing without database update", ctx->stream_name);
                mp4_writer_close(ctx->mp4_writer);
                ctx->mp4_writer = NULL;
            }

            // Only destroy mutexes if thread has stopped
            if (atomic_load(&ctx->state) == UDT_STATE_STOPPED) {
                pthread_mutex_destroy(&ctx->mutex);
                pthread_mutex_destroy(&ctx->detection_stream_result_mutex);
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

/* =========================================================================
 * ONVIF async detection thread
 * =========================================================================
 *
 * Runs detect_motion_onvif() in a loop inside its own OS thread so that
 * the UDT main loop (and therefore av_read_frame()) is never blocked by a
 * CURL/SOAP round-trip.
 *
 * The PullMessages call inside detect_motion_onvif() blocks up to
 * CURLOPT_TIMEOUT (10 s) / PT5S while waiting for camera events.  That
 * natural blocking serves as the inter-poll sleep — no extra sleep() is
 * needed in the happy path.
 *
 * On ONVIF error the motion flag is cleared and we back off ~5 s before
 * retrying; the ONVIF subscription is renewed automatically inside
 * detect_motion_onvif() itself.
 *
 * SOD and API detection paths are completely unaffected by this change.
 * Only the is_onvif_detection_model() branch in run_detection_on_frame()
 * has been modified.
 * ========================================================================= */

/**
 * Body of the ONVIF detection background thread.
 */
static void *onvif_detection_thread_func(void *arg) {
    unified_detection_ctx_t *ctx = (unified_detection_ctx_t *)arg;

    log_info("[%s] ONVIF detection thread started (url=%s)",
             ctx->stream_name, ctx->onvif_url_cached);
    log_debug("[%s] ONVIF detection thread auth=%s",
              ctx->stream_name, (ctx->onvif_username_cached[0] != '\0') ? "enabled" : "disabled");

    while (atomic_load(&ctx->onvif_thread_running)) {
        detection_result_t result;
        memset(&result, 0, sizeof(result));

        int ret = detect_motion_onvif(ctx->onvif_url_cached,
                                      ctx->onvif_username_cached,
                                      ctx->onvif_password_cached,
                                      &result,
                                      ctx->stream_name);

        /* Re-check stop flag — detect_motion_onvif() may have blocked for
         * up to CURLOPT_TIMEOUT seconds.  Bail before touching shared state
         * if we have been asked to stop. */
        if (!atomic_load(&ctx->onvif_thread_running)) break;

        if (ret == 0 && result.count > 0) {
            atomic_store(&ctx->onvif_motion_detected, 1);
            atomic_store(&ctx->onvif_motion_timestamp, (long long)time(NULL));
            log_debug("[%s] ONVIF thread: %d motion event(s) detected",
                      ctx->stream_name, result.count);

            /* When motion is active, PullMessages returns immediately (the
             * camera does not hold the connection for PT5S if events are
             * already queued).  Without a floor here both ONVIF threads
             * would busy-loop, firing detect_motion_onvif() dozens of times
             * per second and flooding process_motion_event() / propagation /
             * DB writes with redundant events (observed: 20+ calls in a
             * single log-second during a motion burst).
             *
             * 2 s is long enough to prevent the storm while still keeping
             * the atomic flag fresh for the UDT's 10 s detection interval.
             * Checked in 100 ms slices so stop requests are honoured quickly. */
            for (int i = 0; i < 20 && atomic_load(&ctx->onvif_thread_running); i++) {
                av_usleep(100000); /* 20 x 100 ms = 2 s */
            }
        } else {
            /* No events in this window (ret==0, count==0) or an ONVIF error
             * (ret!=0).
             *
             * Sticky-flag hysteresis: do NOT clear onvif_motion_detected
             * immediately.  Both ONVIF threads share one PullPoint subscription
             * on this camera.  The event queue alternates between "has events"
             * and "empty" depending on which thread pulled last, so a single
             * "no motion" response does not mean the scene is actually quiet —
             * the sibling thread may have consumed the motion event a few ms
             * earlier.  If we cleared the flag instantly we would race with the
             * UDT's 10 s detection interval and wide (or ptz) would miss the
             * trigger entirely (observed: 38 s late start).
             *
             * Strategy: keep the flag set for at least
             * ONVIF_MOTION_HOLD_SECS after the last confirmed motion event.
             * This guarantees the UDT sees flag==1 on its next 10 s tick even
             * if our very next PullMessages comes back empty.  Only after the
             * hold window expires do we declare the scene idle. */
            long long last_ts = atomic_load(&ctx->onvif_motion_timestamp);
            if (last_ts == 0LL ||
                (long long)time(NULL) - last_ts >= ONVIF_MOTION_HOLD_SECS) {
                atomic_store(&ctx->onvif_motion_detected, 0);
            }
            /* else: hold the flag until the window expires */

            if (ret != 0) {
                log_warn("[%s] ONVIF thread: detect_motion_onvif error %d – "
                         "backing off 5 s before retry", ctx->stream_name, ret);
                /* 50 × 100 ms = 5 s, checking stop flag on each iteration. */
                for (int i = 0; i < 50 && atomic_load(&ctx->onvif_thread_running); i++) {
                    av_usleep(100000);
                }
            }
        }
    }

    log_info("[%s] ONVIF detection thread exiting", ctx->stream_name);
    return NULL;
}

/**
 * Start the ONVIF detection background thread for the given context.
 * ctx->onvif_url_cached / _username_cached / _password_cached must already
 * be populated by the caller.
 *
 * The thread is created joinable so that unified_detection_thread_func()
 * can join it on exit and avoid a detached-thread resource leak.
 *
 * @return 0 on success, -1 on failure.
 */
static int start_onvif_detection_thread(unified_detection_ctx_t *ctx) {
    atomic_store(&ctx->onvif_thread_running, 1);
    atomic_store(&ctx->onvif_motion_detected, 0);
    atomic_store(&ctx->onvif_motion_timestamp, 0LL);

    pthread_attr_t attr;
    pthread_attr_init(&attr);
    pthread_attr_setdetachstate(&attr, PTHREAD_CREATE_JOINABLE);

    int ret = pthread_create(&ctx->onvif_thread, &attr,
                             onvif_detection_thread_func, ctx);
    pthread_attr_destroy(&attr);

    if (ret != 0) {
        log_error("[%s] Failed to create ONVIF detection thread: %s",
                  ctx->stream_name, strerror(ret));
        atomic_store(&ctx->onvif_thread_running, 0);
        return -1;
    }

    log_info("[%s] ONVIF detection thread created", ctx->stream_name);
    return 0;
}

/**
 * Signal the ONVIF detection thread to stop and join it.
 * Safe to call when the thread was never started (onvif_thread_running == 0)
 * or when another concurrent caller already claimed shutdown.
 * Blocks for at most CURLOPT_TIMEOUT + small overhead (≤ ~12 s).
 *
 * Uses atomic_compare_exchange_strong to transition onvif_thread_running
 * from 1 → 0 in a single atomic step.  Only the caller that wins the
 * exchange performs pthread_join(); all other concurrent callers return
 * immediately, preventing undefined behaviour from multiple joins on the
 * same thread handle.
 */
static void stop_onvif_detection_thread(unified_detection_ctx_t *ctx) {
    int expected = 1;
    if (!atomic_compare_exchange_strong(&ctx->onvif_thread_running, &expected, 0)) {
        return; /* never started, already stopped, or another caller will join */
    }

    log_info("[%s] Requesting ONVIF detection thread to stop", ctx->stream_name);

    /* pthread_join blocks until the current detect_motion_onvif() call
     * completes (max CURLOPT_TIMEOUT = 10 s + subscription overhead ≈ 12 s).
     * Acceptable because this path is only reached during UDT teardown.
     * The compare-exchange above guarantees only one caller joins. */
    pthread_join(ctx->onvif_thread, NULL);
    log_info("[%s] ONVIF detection thread joined", ctx->stream_name);
}

/* ============================================================
 * Secondary detection stream thread
 * ============================================================ */

/* Sleep up to *delay_ms in 100 ms slices, checking the stop flag between
 * slices, then exponentially back off *delay_ms (capped at MAX_RECONNECT_DELAY_MS).
 * Mirrors the main UDT's reconnect cadence so a permanently bad detection_url
 * doesn't hammer the upstream every 5 s. */
static void detection_stream_backoff(unified_detection_ctx_t *ctx, int *delay_ms) {
    int remaining = *delay_ms;
    while (remaining > 0 && atomic_load(&ctx->detection_stream_thread_running)) {
        int slice = remaining > 100 ? 100 : remaining;
        av_usleep((unsigned)slice * 1000);
        remaining -= slice;
    }
    *delay_ms *= 2;
    if (*delay_ms > MAX_RECONNECT_DELAY_MS) *delay_ms = MAX_RECONNECT_DELAY_MS;
}

/* Interrupt callback for the detection stream's blocking FFmpeg I/O.
 * Without it, av_read_frame()/avformat_open_input() can block indefinitely on
 * a network read (stimeout only covers RTSP, not HTTP/MJPEG), so a stop request
 * or shutdown would wedge pthread_join() until a watchdog times out. Returns 1
 * to abort when the thread is asked to stop, the UDT is stopping, or the
 * process is shutting down. */
static int detection_stream_interrupt_cb(void *opaque) {
    unified_detection_ctx_t *ctx = (unified_detection_ctx_t *)opaque;
    if (!ctx) return 1;
    if (!atomic_load(&ctx->detection_stream_thread_running) ||
        !atomic_load(&ctx->running) ||
        is_shutdown_initiated()) {
        return 1;
    }
    return 0;
}

static void *detection_stream_thread_func(void *arg) {
    unified_detection_ctx_t *ctx = (unified_detection_ctx_t *)arg;

    log_info("[%s] Detection stream thread started (url=%s)",
             ctx->stream_name, ctx->detection_stream_url);

    int reconnect_delay_ms = BASE_RECONNECT_DELAY_MS;

    while (atomic_load(&ctx->detection_stream_thread_running)) {

        /* ---- open stream ---- */
        /* Allocate the context up front so the interrupt callback is armed
         * before avformat_open_input — making both the open and every later
         * av_read_frame() abortable on stop/shutdown. */
        AVFormatContext *fmt_ctx = avformat_alloc_context();
        if (!fmt_ctx) {
            log_warn("[%s] Detection stream thread: avformat_alloc_context failed",
                     ctx->stream_name);
            detection_stream_backoff(ctx, &reconnect_delay_ms);
            continue;
        }
        fmt_ctx->interrupt_callback.callback = detection_stream_interrupt_cb;
        fmt_ctx->interrupt_callback.opaque   = ctx;

        AVDictionary *opts = NULL;
        av_dict_set(&opts, "rtsp_transport", "tcp", 0);
        av_dict_set(&opts, "stimeout",        "5000000", 0);
        av_dict_set(&opts, "analyzeduration", "2000000", 0);
        av_dict_set(&opts, "probesize",       "2000000", 0);
        /* Refuse file://, concat:, subfile: and other local-resource demuxers.
         * detection_url is user-supplied; lock it to network protocols. */
        av_dict_set(&opts, "protocol_whitelist",
                    "udp,rtp,rtsp,tcp,https,tls,http", 0);

        int ret = avformat_open_input(&fmt_ctx, ctx->detection_stream_url, NULL, &opts);
        av_dict_free(&opts);

        if (!atomic_load(&ctx->detection_stream_thread_running)) {
            /* Stop was requested during the open call (potentially several
             * seconds); exit silently rather than logging a misleading
             * "cannot open ...: Success" warning. */
            if (fmt_ctx) avformat_close_input(&fmt_ctx);
            break;
        }
        if (ret < 0) {
            if (fmt_ctx) avformat_close_input(&fmt_ctx);
            char errbuf[128];
            av_strerror(ret, errbuf, sizeof(errbuf));
            log_warn("[%s] Detection stream thread: cannot open %s: %s — retry in %d ms",
                     ctx->stream_name, ctx->detection_stream_url, errbuf, reconnect_delay_ms);
            detection_stream_backoff(ctx, &reconnect_delay_ms);
            continue;
        }

        if (avformat_find_stream_info(fmt_ctx, NULL) < 0) {
            log_warn("[%s] Detection stream thread: could not read stream info", ctx->stream_name);
            avformat_close_input(&fmt_ctx);
            detection_stream_backoff(ctx, &reconnect_delay_ms);
            continue;
        }

        int video_idx = -1;
        for (unsigned int i = 0; i < fmt_ctx->nb_streams; i++) {
            if (fmt_ctx->streams[i]->codecpar->codec_type == AVMEDIA_TYPE_VIDEO) {
                video_idx = (int)i;
                break;
            }
        }
        if (video_idx < 0) {
            log_warn("[%s] Detection stream thread: no video stream in %s",
                     ctx->stream_name, ctx->detection_stream_url);
            avformat_close_input(&fmt_ctx);
            detection_stream_backoff(ctx, &reconnect_delay_ms);
            continue;
        }

        const AVCodec *decoder =
            avcodec_find_decoder(fmt_ctx->streams[video_idx]->codecpar->codec_id);
        AVCodecContext *dec_ctx = decoder ? avcodec_alloc_context3(decoder) : NULL;
        if (!dec_ctx) {
            log_warn("[%s] Detection stream thread: no decoder available", ctx->stream_name);
            avformat_close_input(&fmt_ctx);
            detection_stream_backoff(ctx, &reconnect_delay_ms);
            continue;
        }
        avcodec_parameters_to_context(dec_ctx, fmt_ctx->streams[video_idx]->codecpar);
        if (avcodec_open2(dec_ctx, decoder, NULL) < 0) {
            log_warn("[%s] Detection stream thread: failed to open decoder", ctx->stream_name);
            avcodec_free_context(&dec_ctx);
            avformat_close_input(&fmt_ctx);
            detection_stream_backoff(ctx, &reconnect_delay_ms);
            continue;
        }

        /* ---- read / produce loop ---- */
        time_t last_check = 0;
        AVPacket *pkt   = av_packet_alloc();
        AVFrame  *frame = av_frame_alloc();
        bool stream_ok  = (pkt != NULL && frame != NULL);
        /* INTRA_ONLY is a codec-descriptor property, not a decoder capability
         * (in the capabilities namespace bit 0 is DRAW_HORIZ_BAND). Use the
         * descriptor so MJPEG and other all-keyframe codecs are recognized. */
        const AVCodecDescriptor *codec_desc = avcodec_descriptor_get(dec_ctx->codec_id);
        bool is_intra_only = codec_desc &&
                             (codec_desc->props & AV_CODEC_PROP_INTRA_ONLY) != 0;

        if (stream_ok) {
            /* Allocations succeeded — only now signal to process_packet that it
             * should suppress main-stream detection and consume from this thread.
             * If pkt/frame failed we leave the flag clear so keyframe detection
             * keeps running on the main stream. */
            atomic_store(&ctx->detection_stream_connected, 1);
            reconnect_delay_ms = BASE_RECONNECT_DELAY_MS;
            log_info("[%s] Detection stream thread connected", ctx->stream_name);
        } else {
            log_warn("[%s] Detection stream thread: failed to allocate packet/frame",
                     ctx->stream_name);
        }

        while (atomic_load(&ctx->detection_stream_thread_running) && stream_ok) {
            int rd = av_read_frame(fmt_ctx, pkt);
            if (rd < 0) {
                log_warn("[%s] Detection stream thread: read error, reconnecting",
                         ctx->stream_name);
                stream_ok = false;
                break;
            }

            if (pkt->stream_index != video_idx) {
                av_packet_unref(pkt);
                continue;
            }

            time_t now = time(NULL);
            bool interval_elapsed = (now - last_check >= (time_t)ctx->detection_interval);

            /* Intra-only (MJPEG): discard frames within the interval at zero decode cost.
             * Predictive codecs (H.264/H.265): must feed every packet to keep the
             * reference chain intact; the interval gate moves into the frame-drain loop. */
            if (is_intra_only && !interval_elapsed) {
                av_packet_unref(pkt);
                continue;
            }

            if (avcodec_send_packet(dec_ctx, pkt) < 0) {
                av_packet_unref(pkt);
                continue;
            }
            av_packet_unref(pkt);

            while (avcodec_receive_frame(dec_ctx, frame) == 0) {
                now = time(NULL);
                if (now - last_check >= (time_t)ctx->detection_interval) {
                    last_check = now;

                    /* Pure detection — no DB writes, no state changes here. */
                    detection_result_t result;
                    bool hit = detect_on_decoded_frame(ctx, frame, now, &result);

                    if (hit) {
                        /* Write result + its frame time to the shared slot, then
                         * raise the flag. process_packet() reads flag + slot and
                         * calls report_detections() + handle_recording_state().
                         * Carrying the timestamp keeps detections.timestamp at
                         * detection time rather than the consumer's later time —
                         * matching the API path, which writes the DB here with
                         * the same `now`. */
                        pthread_mutex_lock(&ctx->detection_stream_result_mutex);
                        ctx->detection_stream_pending = result;
                        ctx->detection_stream_pending_ts = now;
                        pthread_mutex_unlock(&ctx->detection_stream_result_mutex);
                        atomic_store(&ctx->detection_stream_result, 1);
                    }
                }
                av_frame_unref(frame);
            }
        }

        if (pkt)   av_packet_free(&pkt);
        if (frame) av_frame_free(&frame);
        avcodec_free_context(&dec_ctx);
        avformat_close_input(&fmt_ctx);

        /* Clear connected flag and any pending result so the main loop falls
         * back to main-stream detection immediately on the next keyframe. */
        atomic_store(&ctx->detection_stream_connected, 0);
        atomic_store(&ctx->detection_stream_result, 0);

        if (atomic_load(&ctx->detection_stream_thread_running) && !stream_ok) {
            log_info("[%s] Detection stream thread: reconnecting in %d ms",
                     ctx->stream_name, reconnect_delay_ms);
            detection_stream_backoff(ctx, &reconnect_delay_ms);
        }
    }

    log_info("[%s] Detection stream thread exiting", ctx->stream_name);
    return NULL;
}

static int start_detection_stream_thread(unified_detection_ctx_t *ctx) {
    atomic_store(&ctx->detection_stream_thread_running, 1);
    atomic_store(&ctx->detection_stream_connected, 0);
    atomic_store(&ctx->detection_stream_result, 0);
    memset(&ctx->detection_stream_pending, 0, sizeof(ctx->detection_stream_pending));
    ctx->detection_stream_pending_ts = 0;

    pthread_attr_t attr;
    pthread_attr_init(&attr);
    pthread_attr_setdetachstate(&attr, PTHREAD_CREATE_JOINABLE);

    int ret = pthread_create(&ctx->detection_stream_thread, &attr,
                             detection_stream_thread_func, ctx);
    pthread_attr_destroy(&attr);

    if (ret != 0) {
        log_error("[%s] Failed to create detection stream thread: %s",
                  ctx->stream_name, strerror(ret));
        atomic_store(&ctx->detection_stream_thread_running, 0);
        return -1;
    }

    log_info("[%s] Detection stream thread created", ctx->stream_name);
    return 0;
}

static void stop_detection_stream_thread(unified_detection_ctx_t *ctx) {
    int expected = 1;
    if (!atomic_compare_exchange_strong(&ctx->detection_stream_thread_running, &expected, 0))
        return;

    log_info("[%s] Requesting detection stream thread to stop", ctx->stream_name);

    /* pthread_join blocks until the currently outstanding av_read_frame()
     * (or avformat_open_input()) returns. stimeout=5000000 caps that at
     * ~5 s for well-behaved demuxers; a half-open RTSP session may push the
     * worst case higher. Acceptable because this path runs only during UDT
     * teardown. The compare-exchange above guarantees only one caller joins. */
    pthread_join(ctx->detection_stream_thread, NULL);
    log_info("[%s] Detection stream thread joined", ctx->stream_name);
}

/**
 * Start unified detection recording for a stream
 */
int start_unified_detection_thread(const char *stream_name, const char *model_path,
                                   float threshold, int pre_buffer_seconds,
                                   int post_buffer_seconds, bool annotation_only) {
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

    // Get global config first for segment_duration and storage_path
    config_t *global_cfg = get_streaming_config();

    // Initialize context
    safe_strcpy(ctx->stream_name, stream_name, sizeof(ctx->stream_name), 0);
    safe_strcpy(ctx->model_path, model_path, sizeof(ctx->model_path), 0);
    ctx->detection_threshold = threshold;
    ctx->pre_buffer_seconds = pre_buffer_seconds > 0 ? pre_buffer_seconds : 10;
    ctx->post_buffer_seconds = post_buffer_seconds > 0 ? post_buffer_seconds : 5;
    // Use the global segment_duration config for chunking detection recordings (same as continuous recordings)
    ctx->segment_duration = (global_cfg && global_cfg->mp4_segment_duration > 0) ? global_cfg->mp4_segment_duration : 30;
    ctx->detection_interval = config.detection_interval > 0 ? config.detection_interval : DEFAULT_DETECTION_INTERVAL;
    ctx->record_audio = config.record_audio;
    ctx->annotation_only = annotation_only;
    atomic_store(&ctx->external_motion_trigger, 0);  // no pending external trigger

    // Replay-detection: will be set properly on first successful connect
    ctx->stream_connect_time = time(NULL);
    ctx->first_video_pts = AV_NOPTS_VALUE;
    ctx->first_video_pts_set = false;
    ctx->stream_is_live = false;

    // Initialize to current time to avoid large elapsed time on first detection check
    atomic_store(&ctx->last_detection_check_time, (long long)time(NULL));

    if (annotation_only) {
        log_info("[%s] Detection running in annotation-only mode (no separate MP4 files)", stream_name);
    }

    // Get RTSP URL from go2rtc
    if (!go2rtc_stream_get_rtsp_url(stream_name, ctx->rtsp_url, sizeof(ctx->rtsp_url))) {
        // Fall back to direct stream URL, injecting ONVIF credentials if available
        if (url_apply_credentials(config.url,
                                  config.onvif_username[0] ? config.onvif_username : NULL,
                                  config.onvif_password[0] ? config.onvif_password : NULL,
                                  ctx->rtsp_url, sizeof(ctx->rtsp_url)) != 0) {
            safe_strcpy(ctx->rtsp_url, config.url, sizeof(ctx->rtsp_url), 0);
        }
    }

    // Set output directory
    if (global_cfg) {
        // Make sure we're using a valid path.
        char stream_path[MAX_STREAM_NAME];
        sanitize_stream_name(stream_name, stream_path, MAX_STREAM_NAME);

        snprintf(ctx->output_dir, sizeof(ctx->output_dir), "%s/%s",
                 global_cfg->storage_path, stream_path);
        if (ensure_dir(ctx->output_dir)) {
            log_error("Failed to create output directory %s: %s", ctx->output_dir, strerror(errno));
            free(ctx);
            pthread_mutex_unlock(&contexts_mutex);
            return -1;
        }
    }

    // If using built-in motion detection, enable the motion stream now so that
    // detect_motion() does not silently return 0 on every call.  New motion
    // streams are created with enabled=false, so we must flip the flag here.
    // configure_motion_detection() uses threshold as sensitivity and clamps it
    // to a valid range internally.
    if (is_motion_detection_model(model_path)) {
        float sens = (threshold > 0.0f && threshold <= 1.0f) ? threshold : DEFAULT_MOTION_SENSITIVITY;
        configure_motion_detection(stream_name,
                                   sens,
                                   DEFAULT_MOTION_MIN_AREA_RATIO,
                                   DEFAULT_MOTION_MIN_CONSECUTIVE_FRAMES);
        set_motion_detection_enabled(stream_name, true);
        log_info("[%s] Built-in motion detection enabled (sensitivity=%.2f)", stream_name, sens);
    }

    // Initialize mutexes
    if (pthread_mutex_init(&ctx->mutex, NULL) != 0) {
        log_error("Failed to initialize mutex for unified detection context");
        free(ctx);
        pthread_mutex_unlock(&contexts_mutex);
        return -1;
    }

    if (pthread_mutex_init(&ctx->detection_stream_result_mutex, NULL) != 0) {
        log_error("Failed to initialize detection stream result mutex for %s", stream_name);
        pthread_mutex_destroy(&ctx->mutex);
        free(ctx);
        pthread_mutex_unlock(&contexts_mutex);
        return -1;
    }

    // Create circular buffer for pre-detection content
    ctx->packet_buffer = create_packet_buffer(stream_name, ctx->pre_buffer_seconds, BUFFER_MODE_MEMORY);
    if (!ctx->packet_buffer) {
        log_error("Failed to create pre-detection buffer for stream %s", stream_name);
        pthread_mutex_destroy(&ctx->detection_stream_result_mutex);
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

    // Initialize ONVIF async detection thread atomics (zero from calloc, but be explicit)
    atomic_store(&ctx->onvif_thread_running, 0);
    atomic_store(&ctx->onvif_motion_detected, 0);
    atomic_store(&ctx->onvif_motion_timestamp, 0LL);

    // Initialize detection stream thread atomics and result slot
    atomic_store(&ctx->detection_stream_thread_running, 0);
    atomic_store(&ctx->detection_stream_connected, 0);
    atomic_store(&ctx->detection_stream_result, 0);
    memset(&ctx->detection_stream_pending, 0, sizeof(ctx->detection_stream_pending));
    ctx->detection_stream_pending_ts = 0;

    // For ONVIF model: cache connection parameters and start the background
    // polling thread BEFORE the UDT starts reading packets.  This ensures a
    // motion flag is available on the very first detection check.
    if (is_onvif_detection_model(model_path)) {
        extract_onvif_base_url(config.url, config.onvif_port,
                               ctx->onvif_url_cached, sizeof(ctx->onvif_url_cached));
        safe_strcpy(ctx->onvif_username_cached, config.onvif_username,
                    sizeof(ctx->onvif_username_cached), 0);
        safe_strcpy(ctx->onvif_password_cached, config.onvif_password,
                    sizeof(ctx->onvif_password_cached), 0);

        if (ctx->onvif_url_cached[0] == '\0') {
            log_error("[%s] Cannot start ONVIF detection thread: "
                      "could not derive ONVIF URL from stream URL '%s'",
                      stream_name, config.url);
            /* Non-fatal: run_detection_on_frame() will see onvif_motion_detected==0
             * and return false every cycle, which is safe (no spurious recordings). */
        } else {
            if (start_onvif_detection_thread(ctx) != 0) {
                log_error("[%s] ONVIF detection thread could not be started; "
                          "ONVIF-triggered recording is disabled for this stream",
                          stream_name);
            }
        }
    }

    // If a secondary detection URL is configured, cache it and start the detection stream thread.
    if (config.detection_url[0] != '\0') {
        safe_strcpy(ctx->detection_stream_url, config.detection_url,
                    sizeof(ctx->detection_stream_url), 0);
        if (start_detection_stream_thread(ctx) != 0) {
            log_warn("[%s] Detection stream thread could not be started; "
                     "falling back to keyframe-based detection on the main stream",
                     stream_name);
            /* Non-fatal: the UDT will run detection on the main stream instead. */
        } else {
            log_info("[%s] Detection stream thread started for url=%s",
                     stream_name, ctx->detection_stream_url);
        }
    }

    // Store context in slot
    ctx->slot_idx = slot;
    detection_contexts[slot] = ctx;

    // Create UDT thread (detached)
    pthread_attr_t attr;
    pthread_attr_init(&attr);
    pthread_attr_setdetachstate(&attr, PTHREAD_CREATE_DETACHED);

    int result = pthread_create(&ctx->thread, &attr, unified_detection_thread_func, ctx);
    pthread_attr_destroy(&attr);

    if (result != 0) {
        log_error("Failed to create unified detection thread for %s: %s",
                  stream_name, strerror(result));
        /* Stop background threads before freeing ctx to avoid use-after-free:
         * the threads were started above but the UDT that would normally join
         * them never ran. */
        stop_onvif_detection_thread(ctx);
        stop_detection_stream_thread(ctx);
        destroy_packet_buffer(ctx->packet_buffer);
        pthread_mutex_destroy(&ctx->detection_stream_result_mutex);
        pthread_mutex_destroy(&ctx->mutex);
        free(ctx);
        detection_contexts[slot] = NULL;
        pthread_mutex_unlock(&contexts_mutex);
        return -1;
    }

    pthread_mutex_unlock(&contexts_mutex);

    log_info("Started unified detection thread for stream %s (model=%s, threshold=%.2f, interval=%d, pre-buffer=%ds, post-buffer=%ds, segment=%ds)",
             stream_name, ctx->model_path, ctx->detection_threshold, ctx->detection_interval,
             ctx->pre_buffer_seconds, ctx->post_buffer_seconds, ctx->segment_duration);

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
 * Get the effective stream status based on UDT state for API reporting.
 */
stream_status_t get_unified_detection_effective_status(const char *stream_name) {
    if (!stream_name) {
        return STREAM_STATUS_STOPPED;
    }

    pthread_mutex_lock(&contexts_mutex);

    unified_detection_ctx_t *ctx = find_context_by_name(stream_name);
    if (!ctx) {
        pthread_mutex_unlock(&contexts_mutex);
        return STREAM_STATUS_STOPPED;
    }

    unified_detection_state_t udt_state = atomic_load(&ctx->state);
    int reconnect_attempt = ctx->reconnect_attempt;

    pthread_mutex_unlock(&contexts_mutex);

    switch (udt_state) {
        case UDT_STATE_INITIALIZING:
            return STREAM_STATUS_STARTING;

        case UDT_STATE_CONNECTING:
            /* First connection attempt → Starting; subsequent attempts → Reconnecting */
            return (reconnect_attempt > 0) ? STREAM_STATUS_RECONNECTING : STREAM_STATUS_STARTING;

        case UDT_STATE_BUFFERING:
        case UDT_STATE_RECORDING:
        case UDT_STATE_POST_BUFFER:
            return STREAM_STATUS_RUNNING;

        case UDT_STATE_RECONNECTING:
            return STREAM_STATUS_RECONNECTING;

        case UDT_STATE_STOPPING:
            return STREAM_STATUS_STOPPING;

        case UDT_STATE_STOPPED:
        default:
            return STREAM_STATUS_STOPPED;
    }
}

/**
 * Get the number of reconnect attempts made by a unified detection thread.
 */
int get_unified_detection_reconnect_attempts(const char *stream_name) {
    if (!stream_name) {
        return 0;
    }

    pthread_mutex_lock(&contexts_mutex);

    unified_detection_ctx_t *ctx = find_context_by_name(stream_name);
    int attempts = ctx ? ctx->reconnect_attempt : 0;

    pthread_mutex_unlock(&contexts_mutex);

    return attempts;
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
 * Notify a UDT-managed stream of an externally-detected motion event.
 *
 * Called from the ONVIF motion recording system when a master stream's motion
 * event must be forwarded to a slave stream that runs as a UDT (e.g. the PTZ
 * lens on a TP-Link C545D which has no ONVIF profile of its own).
 *
 * This function looks up the target stream under contexts_mutex, so it may
 * block while waiting for that mutex. If a matching UDT context is found, it
 * updates ctx->external_motion_trigger atomically; if the target stream is not
 * managed by a UDT the call is a silent no-op.
 *
 * The trigger is not consumed immediately on every packet. It is observed by
 * the UDT thread during its normal keyframe-based detection processing, so
 * externally-forwarded motion may not take effect until the next such check.
 *
 * Values written to ctx->external_motion_trigger:
 *   1 = motion active (start / keep-alive)
 *   2 = motion ended
 *   0 = idle (initial / reset by UDT thread after processing)
 */
void unified_detection_notify_motion(const char *stream_name, bool motion_active) {
    if (!stream_name) return;

    pthread_mutex_lock(&contexts_mutex);
    unified_detection_ctx_t *ctx = find_context_by_name(stream_name);
    if (ctx) {
        // 1 = motion active, 2 = motion ended
        atomic_store(&ctx->external_motion_trigger, motion_active ? 1 : 2);
        log_debug("[%s] external_motion_trigger set to %d via unified_detection_notify_motion",
                  stream_name, motion_active ? 1 : 2);
    }
    pthread_mutex_unlock(&contexts_mutex);
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
        const AVCodecParameters *codecpar = ctx->input_ctx->streams[i]->codecpar;
        if (codecpar->codec_type == AVMEDIA_TYPE_VIDEO && ctx->video_stream_idx < 0) {
            ctx->video_stream_idx = (int)i;
        } else if (codecpar->codec_type == AVMEDIA_TYPE_AUDIO && ctx->audio_stream_idx < 0) {
            ctx->audio_stream_idx = (int)i;
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

    // Auto-detect and persist video parameters (width, height, fps, codec)
    {
        AVStream *vs = ctx->input_ctx->streams[ctx->video_stream_idx];
        AVCodecParameters *cp = vs->codecpar;
        int det_width = cp->width;
        int det_height = cp->height;
        int det_fps = 0;
        const char *det_codec = NULL;
        bool fps_is_provisional = false;

        if (vs->avg_frame_rate.den > 0 && vs->avg_frame_rate.num > 0) {
            det_fps = (int)(vs->avg_frame_rate.num / vs->avg_frame_rate.den);
        }
        // avg_frame_rate is 0 for many older cameras (e.g. Axis M1011); fall back
        // to the container's r_frame_rate, then to a safe default so the stored
        // value is always meaningful.
        if (det_fps <= 0 && vs->r_frame_rate.den > 0 && vs->r_frame_rate.num > 0) {
            det_fps = (int)(vs->r_frame_rate.num / vs->r_frame_rate.den);
            if (det_fps > 0) {
                log_debug("[%s] avg_frame_rate unavailable; using r_frame_rate: %d fps",
                          ctx->stream_name, det_fps);
            }
        }
        if (det_fps <= 0) {
            det_fps = DEFAULT_FPS_FALLBACK; // conservative default for cameras that omit FPS in SDP
            fps_is_provisional = true;
            log_debug("[%s] FPS unknown from SDP; defaulting provisionally to %d fps; "
                      "runtime frame arrival will be used to refine this value",
                      ctx->stream_name, det_fps);
        }

        const AVCodecDescriptor *desc = avcodec_descriptor_get(cp->codec_id);
        if (desc) {
            det_codec = desc->name;
        }

        if (det_width > 0 && det_height > 0) {
            log_info("[%s] Detected video params: %dx%d @ %d fps%s, codec=%s",
                     ctx->stream_name, det_width, det_height, det_fps,
                     fps_is_provisional ? " (provisional)" : "",
                     det_codec ? det_codec : "unknown");
            udt_update_stream_video_params(ctx, det_width, det_height,
                                           det_fps, det_codec, fps_is_provisional);
        }
    }

    return 0;
}

/**
 * Update stored video parameters, tracking whether the FPS value is provisional.
 * When fps_is_provisional is true, the runtime frame-arrival measurement in
 * process_packet() will later refine the stored FPS once enough frames have
 * been observed.
 */
static void udt_update_stream_video_params(unified_detection_ctx_t *ctx,
                                           int det_width,
                                           int det_height,
                                           int det_fps,
                                           const char *det_codec,
                                           bool fps_is_provisional) {
    if (!ctx) return;

    metrics_set_configured_fps(ctx->stream_name, det_fps);

    pthread_mutex_lock(&ctx->mutex);
    atomic_store(&ctx->fps_is_provisional, fps_is_provisional);
    if (fps_is_provisional) {
        // Reset measurement counters so process_packet() can start measuring
        atomic_store(&ctx->fps_measurement_frame_count, 0);
        struct timespec ts_tmp;
        clock_gettime(CLOCK_MONOTONIC, &ts_tmp);
        atomic_store(&ctx->fps_measurement_start_ns,
                     (long long)ts_tmp.tv_sec * 1000000000LL + ts_tmp.tv_nsec);
    }
    pthread_mutex_unlock(&ctx->mutex);

    /* Snapshot the previously-stored codec before writing the new row so we
     * can detect a codec transition and re-register the stream with go2rtc.
     * This covers the case where the user declared (or we guessed) H.264 at
     * stream-add time but the camera is actually delivering H.265 — without
     * the re-register, go2rtc keeps its original H.264-only source list and
     * WebRTC clients can't negotiate (#374). */
    char prev_codec[16] = {0};
    {
        stream_config_t prev = {0};
        if (get_stream_config_by_name(ctx->stream_name, &prev) == 0) {
            safe_strcpy(prev_codec, prev.codec, sizeof(prev_codec), 0);
        }
    }

    // Persist the (possibly provisional) values to the database
    update_stream_video_params(ctx->stream_name, det_width, det_height,
                               det_fps, det_codec);

    if (det_codec && det_codec[0] != '\0' &&
        strcasecmp(prev_codec, det_codec) != 0) {
        log_info("[%s] Source codec transitioned %s -> %s; re-registering with go2rtc",
                 ctx->stream_name,
                 prev_codec[0] ? prev_codec : "(unknown)", det_codec);
        /* Pass all defaults so reload_stream_config reads the freshly-written
         * codec out of the DB itself. */
        go2rtc_integration_reload_stream_config(ctx->stream_name,
                                                NULL, NULL, NULL, -1, -1, -1);
    }
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
    safe_strcpy(stream_name, ctx->stream_name, sizeof(stream_name), 0);

    log_set_thread_context("Detection", stream_name);
    log_info("[%s] Unified detection thread started", stream_name);

    // Silence libav's default stderr logging. Detection streams often
    // connect to a go2rtc proxy whose upstream can EOF (e.g. unplugged
    // camera, #402); libav's RTSP demuxer then floods stderr with
    // `Failed reading RTSP data: End of file` via its AV_LOG_WARNING
    // callback. We surface errors through log_error() ourselves.
    av_log_set_level(AV_LOG_QUIET);

    unified_detection_state_t state;
    int reconnect_delay_ms = BASE_RECONNECT_DELAY_MS;
    // Track whether this particular connection produced any real media
    // packets. Used in the packet-timeout branch below to distinguish an
    // interrupted-but-otherwise-healthy stream from one where the RTSP
    // handshake succeeded against a proxy but no media ever arrived.
    int saw_real_packets = 0;
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
                state = UDT_STATE_CONNECTING;
                break;

            case UDT_STATE_CONNECTING:
                log_info("[%s] State: CONNECTING (attempt %d)", stream_name, ctx->reconnect_attempt + 1);

                if (connect_to_stream(ctx) == 0) {
                    state = UDT_STATE_BUFFERING;
                    ctx->reconnect_attempt = 0;
                    // Intentionally *not* resetting reconnect_delay_ms here.
                    // RTSP handshake success doesn't mean the stream is
                    // healthy (go2rtc proxies an unplugged camera just
                    // fine); we only reset the backoff once a real media
                    // packet arrives, below. #402
                    atomic_store(&ctx->last_packet_time, (int_fast64_t)time(NULL));
                    saw_real_packets = 0;
                    // Reset per-connection heartbeat counters so the log
                    // reports fresh numbers instead of carrying 700+ s of
                    // stale state across reconnects (#402).
                    pthread_mutex_lock(&ctx->mutex);
                    ctx->total_packets_processed = 0;
                    ctx->total_detections = 0;
                    pthread_mutex_unlock(&ctx->mutex);
                    atomic_store(&ctx->last_detection_check_time, (long long)time(NULL));
                    // Reset replay-detection state for the new connection
                    ctx->stream_connect_time = time(NULL);
                    ctx->first_video_pts_set = false;
                    ctx->first_video_pts = AV_NOPTS_VALUE;
                    ctx->stream_is_live = false;
                } else {
                    ctx->reconnect_attempt++;
                    atomic_fetch_add(&ctx->consecutive_failures, 1);

                    // Exponential backoff with shutdown check every 500ms
                    {
                        int remaining_ms = reconnect_delay_ms;
                        while (remaining_ms > 0 && atomic_load(&ctx->running) && !is_shutdown_initiated()) {
                            int sleep_ms = remaining_ms > 500 ? 500 : remaining_ms;
                            usleep(sleep_ms * 1000);
                            remaining_ms -= sleep_ms;
                        }
                    }
                    reconnect_delay_ms = reconnect_delay_ms * 2;
                    if (reconnect_delay_ms > MAX_RECONNECT_DELAY_MS) {
                        reconnect_delay_ms = MAX_RECONNECT_DELAY_MS;
                    }
                }
                break;

            case UDT_STATE_BUFFERING:
            case UDT_STATE_RECORDING:
            case UDT_STATE_POST_BUFFER:
                // Periodic heartbeat log (every 30 seconds) to show thread is alive
                {
                    static __thread time_t last_heartbeat = 0;
                    time_t now = time(NULL);

                    if (now - last_heartbeat >= 30) {
                        last_heartbeat = now;
                        log_info("[%s] Heartbeat: state=%s, packets=%lu, detections=%lu, last_check=%lds ago",
                                 stream_name, state_to_string(state),
                                 (unsigned long)ctx->total_packets_processed,
                                 (unsigned long)ctx->total_detections,
                                 (long)(now - atomic_load(&ctx->last_detection_check_time)));
                    }
                }

                // Read packet
                if (av_read_frame(ctx->input_ctx, pkt) >= 0) {
                    // Only treat non-empty packets as evidence the stream
                    // is delivering media. Libav's RTSP demuxer can return
                    // zero-length packets (RTSP control chatter, EOF resync)
                    // that would otherwise keep last_packet_time fresh
                    // forever and prevent MAX_PACKET_TIMEOUT_SEC from ever
                    // firing (#402).
                    if (pkt->size > 0) {
                        atomic_store(&ctx->last_packet_time, (int_fast64_t)time(NULL));
                        if (!saw_real_packets) {
                            saw_real_packets = 1;
                            // First real packet — connection is confirmed
                            // healthy, so any future natural interruption
                            // starts from the base reconnect delay again.
                            reconnect_delay_ms = BASE_RECONNECT_DELAY_MS;
                            atomic_store(&ctx->consecutive_failures, 0);
                        }
                    }

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
                        if (!saw_real_packets) {
                            // Handshake succeeded but no real media packets
                            // ever arrived. Treat as a soft connection
                            // failure: apply the same exponential backoff
                            // the failed-connect branch uses so we don't
                            // tight-loop reconnecting to a go2rtc proxy
                            // whose upstream camera is offline (#402).
                            log_warn("[%s] Packet timeout with no media received; "
                                     "backing off %d ms before reconnect",
                                     stream_name, reconnect_delay_ms);
                            atomic_fetch_add(&ctx->consecutive_failures, 1);
                            int remaining_ms = reconnect_delay_ms;
                            while (remaining_ms > 0 && atomic_load(&ctx->running) && !is_shutdown_initiated()) {
                                int sleep_ms = remaining_ms > 500 ? 500 : remaining_ms;
                                usleep(sleep_ms * 1000);
                                remaining_ms -= sleep_ms;
                            }
                            reconnect_delay_ms = reconnect_delay_ms * 2;
                            if (reconnect_delay_ms > MAX_RECONNECT_DELAY_MS) {
                                reconnect_delay_ms = MAX_RECONNECT_DELAY_MS;
                            }
                        } else {
                            log_warn("[%s] Packet timeout, reconnecting", stream_name);
                        }
                        disconnect_from_stream(ctx);
                        state = UDT_STATE_RECONNECTING;
                    } else {
                        // Avoid busy-looping when av_read_frame returns immediately
                        // (e.g. RTSP EOF or CSeq mismatch during a go2rtc session reset).
                        // Without this sleep the loop spins at ~45k iterations/second,
                        // flooding the log and burning CPU until MAX_PACKET_TIMEOUT_SEC
                        // is reached.  10 ms still allows the timeout check to trigger
                        // within one second of the actual deadline.
                        av_usleep(10000);
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

    // Close any active recording before cleanup
    // This ensures database is updated with correct end_time and duration
    if (ctx->mp4_writer) {
        log_info("[%s] Closing active recording before thread exit", stream_name);
        udt_stop_recording(ctx);
    }

    // Stop the ONVIF detection thread before freeing ctx.
    // Must happen before disconnect_from_stream() and before any free(),
    // because the ONVIF thread still holds a pointer to ctx.
    // pthread_join blocks for at most CURLOPT_TIMEOUT + subscription overhead (≤ ~12 s).
    if (is_onvif_detection_model(ctx->model_path)) {
        stop_onvif_detection_thread(ctx);
    }

    // Stop the detection stream thread (no-op if never started).
    stop_detection_stream_thread(ctx);

    // Disconnect from stream to free FFmpeg decoder_ctx and input_ctx
    // This handles the case where the thread exits the loop while still connected
    // (e.g., during shutdown while in BUFFERING/RECORDING state)
    disconnect_from_stream(ctx);

    // Clean up thread-local CURL handle used by go2rtc_get_snapshot()
    // This must be called from the same thread that created the handle
    go2rtc_snapshot_cleanup_thread();

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
                pthread_mutex_destroy(&ctx->detection_stream_result_mutex);
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

    // Skip until we find a VIDEO keyframe (ensures valid MP4 start).
    // Audio packets carry AV_PKT_FLAG_KEY on every frame; using them as the
    // keyframe anchor would prevent the MP4 writer from ever initialising.
    if (!flush_ctx->found_keyframe) {
        if ((packet->flags & AV_PKT_FLAG_KEY) &&
            packet->stream_index == flush_ctx->ctx->video_stream_idx) {
            flush_ctx->found_keyframe = true;
        } else {
            return 0;  // Skip until first video keyframe
        }
    }

    // Write packet to MP4 - we need the input stream for codec parameters
    if (ctx->mp4_writer && ctx->input_ctx) {
        const AVStream *input_stream = NULL;
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

    // Update statistics and runtime FPS measurement
    pthread_mutex_lock(&ctx->mutex);
    ctx->total_packets_processed++;

    // Runtime FPS refinement: count video frames over a measurement window
    // and update the stored FPS once we have enough data.
    if (is_video && atomic_load(&ctx->fps_is_provisional)) {
        int frame_count = atomic_fetch_add(&ctx->fps_measurement_frame_count, 1) + 1;

        struct timespec ts_now;
        clock_gettime(CLOCK_MONOTONIC, &ts_now);
        long long now_ns = (long long)ts_now.tv_sec * 1000000000LL + ts_now.tv_nsec;
        long long start_ns = atomic_load(&ctx->fps_measurement_start_ns);
        double elapsed = (now_ns - start_ns) / 1e9;

        if (elapsed >= FPS_MEASUREMENT_WINDOW_SEC && frame_count > 1) {
            int measured_fps = (int)(frame_count / elapsed + 0.5);
            if (measured_fps > 0) {
                log_info("[%s] Runtime FPS measurement: %d fps (measured %d frames over %.1fs); "
                         "updating stored value from provisional %d fps",
                         ctx->stream_name, measured_fps,
                         frame_count, elapsed,
                         DEFAULT_FPS_FALLBACK);
                atomic_store(&ctx->fps_is_provisional, false);

                // Retrieve current width/height/codec from the stream so we
                // don't clobber them when updating only FPS.
                int cur_w = 0, cur_h = 0;
                const char *cur_codec = NULL;
                if (ctx->input_ctx && ctx->video_stream_idx >= 0) {
                    const AVCodecParameters *cp2 =
                        ctx->input_ctx->streams[ctx->video_stream_idx]->codecpar;
                    cur_w = cp2->width;
                    cur_h = cp2->height;
                    const AVCodecDescriptor *d = avcodec_descriptor_get(cp2->codec_id);
                    if (d) cur_codec = d->name;
                }

                // Release mutex before DB call to avoid holding it during I/O
                pthread_mutex_unlock(&ctx->mutex);
                udt_update_stream_video_params(ctx, cur_w, cur_h,
                                               measured_fps, cur_codec, false);
                goto stats_done;
            }
        }
    }
    pthread_mutex_unlock(&ctx->mutex);
stats_done:

    // Always add packets to circular buffer (for pre-detection content)
    // The buffer automatically evicts old packets when full
    //
    // go2rtc replay guard: go2rtc replays its ring buffer in real-time when a
    // consumer connects.  During replay the PTS lags behind wall clock by the
    // ring-buffer duration.  We skip buffering until the stream is live so the
    // pre-buffer contains only recent (useful) packets.
    // The lag is derived from video packets only; audio packets follow the same
    // gate so the buffer never fills with orphaned audio during replay.
    //
    // Additionally, we only start buffering on a keyframe — so the pre-buffer
    // always begins with a valid GOP start and flush_packet_callback will find
    // a keyframe to initialise the MP4 writer.
    bool should_buffer = false;
    if (is_video && ctx->input_ctx && ctx->video_stream_idx >= 0) {
        if (!ctx->first_video_pts_set) {
            // First video packet — record the PTS reference used for replay
            // lag calculations. If the stream is already known to be live,
            // allow an initial keyframe to enter the pre-buffer immediately so
            // we do not lose the first GOP window.
            if (pkt->pts != AV_NOPTS_VALUE) {
                ctx->first_video_pts = pkt->pts;
                ctx->first_video_pts_set = true;
            }
            // If the stream is already known to be live, allow an initial
            // keyframe to enter the pre-buffer immediately so we do not
            // lose the first GOP window.
            if (ctx->stream_is_live && is_keyframe) {
                should_buffer = true;
            }
        } else if (ctx->stream_is_live) {
            // Once the stream is live the replay-lag check is no longer meaningful:
            // wall-clock and the camera's PTS clock can drift arbitrarily over
            // long uptime, and gating buffering on that drift would freeze the
            // pre-buffer with stale packets indefinitely (the !add path skips
            // packet_buffer_add_packet, which is also where time-based eviction
            // runs). Any real stall is handled by MAX_PACKET_TIMEOUT_SEC →
            // reconnect, which re-anchors first_video_pts on the next session.
            should_buffer = true;
        } else {
            // Pre-live: gate buffering until go2rtc's replay window closes.
            // If PTS is valid use PTS-vs-wallclock for accuracy; otherwise fall
            // back to a pure wall-clock warmup of 2 × pre_buffer_seconds.
            double wall_elapsed = difftime(now, ctx->stream_connect_time);
            double replay_lag;

            if (pkt->pts != AV_NOPTS_VALUE && ctx->first_video_pts != AV_NOPTS_VALUE) {
                AVRational tb = ctx->input_ctx->streams[ctx->video_stream_idx]->time_base;
                double pts_elapsed = (double)(pkt->pts - ctx->first_video_pts) * av_q2d(tb);
                replay_lag = wall_elapsed - pts_elapsed;
            } else {
                double warmup = (double)(ctx->pre_buffer_seconds * 2);
                replay_lag = (wall_elapsed < warmup) ? (warmup - wall_elapsed) : 0.0;
            }

            if (replay_lag <= (double)ctx->pre_buffer_seconds) {
                if (is_keyframe) {
                    // First live keyframe — flush stale replay data and go live.
                    packet_buffer_clear(ctx->packet_buffer);
                    ctx->stream_is_live = true;
                    should_buffer = true;
                    log_info("[%s] go2rtc replay ended, pre-buffer active (lag=%.1fs)",
                             ctx->stream_name, replay_lag);
                }
                // else: waiting for first live keyframe — skip
            } else if (is_keyframe) {
                log_debug("[%s] go2rtc replay in progress (lag=%.1fs), skipping pre-buffer",
                          ctx->stream_name, replay_lag);
            }
        }
    } else if (!is_video && ctx->stream_is_live) {
        // Non-video (audio): buffer only after the first live keyframe has
        // marked the stream as live, so replay audio is not added on its own.
        should_buffer = true;
    }

    if (should_buffer)
        packet_buffer_add_packet(ctx->packet_buffer, pkt, now);

    // Record stream metrics
    metrics_record_frame(ctx->stream_name, pkt->size, is_video);

    // Detection stream path: runs on EVERY packet, no keyframe requirement.
    //
    // The background thread already handles its own rate-limiting and decoding.
    // flush_prebuffer_to_recording() finds the first keyframe in the pre-buffer
    // itself (see flush_packet_callback), so the current packet does not need
    // to be a keyframe.  Consuming the result on every packet rather than
    // waiting for a keyframe reduces recording-start latency from up to one
    // full GOP interval (2–4 s on typical cameras) to one packet interval (~40 ms).
    //
    // Consumed in every state, including POST_BUFFER, and *before* the
    // post-buffer-expiry check below.  Skipping consumption during POST_BUFFER
    // left fresh hits sitting in the slot: the POST_BUFFER → RECORDING
    // keep-alive branch in handle_recording_state was unreachable, and the
    // stale slot then fired on the next packet after BUFFERING, triggering a
    // phantom detection-less recording.
    if (atomic_load(&ctx->detection_stream_connected)) {
        bool detected = false;
        if (atomic_exchange(&ctx->detection_stream_result, 0)) {
            detection_result_t result;
            time_t result_ts;
            pthread_mutex_lock(&ctx->detection_stream_result_mutex);
            result = ctx->detection_stream_pending;
            result_ts = ctx->detection_stream_pending_ts;
            pthread_mutex_unlock(&ctx->detection_stream_result_mutex);
            log_info("[%s] Detection stream result: %d detection(s)",
                     ctx->stream_name, result.count);
            /* Stamp the DB write with the producer's detection time (when the
             * frame was actually analyzed), not this consumer's wall-clock —
             * otherwise detections lag the video by the hand-off latency. */
            report_detections(ctx, &result, result_ts > 0 ? result_ts : now);
            detected = true;
        }
        // handle_recording_state drives the recording state machine, which is a
        // "now" decision (grace/post-buffer), so it keeps the consumer's now.
        handle_recording_state(ctx, detected, now);
        // Re-read state: handle_recording_state may have transitioned
        // BUFFERING → RECORDING or POST_BUFFER → RECORDING above.
        current_state = (unified_detection_state_t)atomic_load(&ctx->state);
    }

    // If recording, write packet to MP4
    if (current_state == UDT_STATE_RECORDING || current_state == UDT_STATE_POST_BUFFER) {
        if (ctx->mp4_writer && ctx->input_ctx) {
            const AVStream *input_stream = NULL;
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
            if (now >= (time_t)atomic_load(&ctx->post_buffer_end_time)) {
                log_info("[%s] Post-buffer complete, stopping recording", ctx->stream_name);
                udt_stop_recording(ctx);
                atomic_store(&ctx->state, UDT_STATE_BUFFERING);
            }
        }

        // NOTE: Detection recordings are stopped naturally via the POST_BUFFER mechanism
        // (UDT_STATE_POST_BUFFER → post_buffer_end_time expiry → udt_stop_recording).
        // A hard max_duration = pre+post cap was removed because it terminated recordings
        // while motion was still active, producing split clips instead of one coherent file.
        // For very long events, segment_duration (default 900s) provides the upper bound.
    }

    // --- External motion trigger (e.g. ONVIF event forwarded from a master stream) ---
    // Consumed on every keyframe, regardless of current state.
    //
    // The exchange used to be inside the stricter keyframe/state guard, which
    // made the POST_BUFFER branch unreachable and prevented a motion keep-alive
    // from extending an active recording back from POST_BUFFER to RECORDING.
    // Consuming the flag here ensures it is always handled on keyframes and
    // keeps the intended state transitions reachable.
    if (is_keyframe) {
        int ext_trigger = atomic_exchange(&ctx->external_motion_trigger, 0);

        if (ext_trigger == 1) {
            // Motion active: treat as a detection event
            log_info("[%s] External motion trigger (active) received", ctx->stream_name);
            atomic_store(&ctx->last_detection_time, (long long)now);

            if (!ctx->annotation_only) {
                if (current_state == UDT_STATE_BUFFERING) {
                    log_info("[%s] External trigger starting recording", ctx->stream_name);
                    if (udt_start_recording(ctx) == 0) {
                        // Flush pre-buffer and correct DB start_time to reflect actual start
                        int pre_dur = 0;
                        int pre_cnt = 0; size_t pre_mem = 0;
                        if (ctx->packet_buffer)
                            packet_buffer_get_stats(ctx->packet_buffer, &pre_cnt, &pre_mem, &pre_dur);

                        flush_prebuffer_to_recording(ctx);
                        atomic_store(&ctx->state, UDT_STATE_RECORDING);

                        // Correct start_time in DB and writer to the actual first-packet time
                        if (!ctx->mp4_writer->start_time_corrected && pre_dur > 0 &&
                            ctx->current_recording_id > 0) {
                            // Clamp pre_dur to the configured pre_buffer window.
                            // go2rtc may deliver a ring-buffer of 200+ seconds; using
                            // the raw value would push start_time so far back that
                            // elapsed > max_duration immediately, stopping the recording.
                            int clamped_pre = pre_dur > ctx->pre_buffer_seconds
                                              ? ctx->pre_buffer_seconds : pre_dur;
                            time_t corrected = now - (time_t)clamped_pre;
                            ctx->mp4_writer->creation_time = corrected;
                            ctx->mp4_writer->start_time_corrected = true;
                            update_recording_start_time(ctx->current_recording_id, corrected);
                            log_info("[%s] Corrected recording start_time by -%ds (pre-buffer, clamped from %ds)",
                                     ctx->stream_name, clamped_pre, pre_dur);
                        }
                    }
                } else if (current_state == UDT_STATE_POST_BUFFER) {
                    // Motion keep-alive during post-buffer: extend recording back to RECORDING.
                    // Previously unreachable due to the state guard — now correctly handled.
                    log_info("[%s] External trigger during post-buffer, continuing recording", ctx->stream_name);
                    atomic_store(&ctx->state, UDT_STATE_RECORDING);
                }
                // If already RECORDING: just refresh last_detection_time (already done above)
            }
        } else if (ext_trigger == 2) {
            // Motion ended: if recording, enter post-buffer
            log_info("[%s] External motion trigger (ended) received", ctx->stream_name);
            if (!ctx->annotation_only && current_state == UDT_STATE_RECORDING) {
                log_info("[%s] External trigger ending recording, entering post-buffer (%ds)",
                         ctx->stream_name, ctx->post_buffer_seconds);
                atomic_store(&ctx->post_buffer_end_time, (long long)(now + ctx->post_buffer_seconds));
                atomic_store(&ctx->state, UDT_STATE_POST_BUFFER);
            }
        }

        // Re-read state in case external trigger changed it above
        current_state = (unified_detection_state_t)atomic_load(&ctx->state);
    }

    // Main-stream path: keyframe + time-gated.
    //
    // Decoding a non-keyframe packet in isolation is not possible without the
    // preceding reference frames, so detection must wait for a keyframe.
    // Only active when no detection stream is connected (fallback or unconfigured).
    if (is_keyframe && current_state != UDT_STATE_POST_BUFFER &&
        !atomic_load(&ctx->detection_stream_connected)) {

        time_t time_since_last_check = now - (time_t)atomic_load(&ctx->last_detection_check_time);

        if (time_since_last_check > 0 && (atomic_fetch_add(&ctx->log_counter, 1) % 10) == 0) {
            log_debug("[%s] Time since last detection check: %ld/%d seconds, model=%s, state=%d",
                     ctx->stream_name, (long)time_since_last_check, ctx->detection_interval,
                     ctx->model_path, current_state);
        }

        if (time_since_last_check >= ctx->detection_interval) {
            atomic_store(&ctx->last_detection_check_time, (long long)now);
            log_info("[%s] Running detection (interval=%ds, elapsed=%lds, model=%s)",
                    ctx->stream_name, ctx->detection_interval,
                    (long)time_since_last_check, ctx->model_path);
            bool detected = run_detection_on_frame(ctx, pkt, now);
            handle_recording_state(ctx, detected, now);
        }
    }

    return 0;
}

/**
 * Start recording - create MP4 writer
 * In annotation_only mode, this is a no-op - detections are stored but no separate MP4 is created
 */
static int udt_start_recording(unified_detection_ctx_t *ctx) {
    if (!ctx) return -1;

    // In annotation-only mode, skip MP4 creation
    // Detections will be stored and linked to the continuous recording instead
    if (ctx->annotation_only) {
        log_debug("[%s] Annotation-only mode: skipping MP4 creation for detection", ctx->stream_name);
        return 0;
    }

    // Ensure output directory exists
    struct stat st = {0};
    if (stat(ctx->output_dir, &st) == -1) {
        if (ensure_dir(ctx->output_dir)) {
            log_error("[%s] Failed to create output directory: %s", ctx->stream_name, ctx->output_dir);
            return -1;
        }
    }

    // Generate output filename with timestamp
    time_t now = time(NULL);
    struct tm tm_buf;
    const struct tm *tm_info = localtime_r(&now, &tm_buf);
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
    ctx->mp4_writer->pre_buffer_seconds = ctx->pre_buffer_seconds;

    // Configure audio recording based on stream settings
    if (ctx->record_audio && ctx->audio_stream_idx >= 0) {
        mp4_writer_set_audio(ctx->mp4_writer, 1);
        log_info("[%s] Audio recording enabled for detection recording", ctx->stream_name);

        // Eagerly determine the output codec parameters so mp4_writer_initialize()
        // can declare the audio stream BEFORE avformat_write_header() — the only
        // legal window for adding streams to an MP4 container.
        //
        // The pending params must exactly match what mp4_writer_write_packet() will
        // later mux:
        //   * PCM variants  -> transcode to AAC  (mp4_writer_write_packet transcodes)
        //   * MP4-compatible -> deep-copy as-is  (packets pass through unchanged)
        //   * anything else -> disable audio     (not supported)
        AVStream *ain = ctx->input_ctx->streams[ctx->audio_stream_idx];
        const char *codec_name = "unknown";
        AVCodecParameters *pending = NULL;

        if (is_pcm_codec(ain->codecpar->codec_id)) {
            // PCM: probe transcode_pcm_to_aac() for the AAC output parameters.
            // Stateless call — no global audio_transcoders[] access, does not
            // interfere with the lazy init_audio_transcoder() during packet processing.
            if (transcode_pcm_to_aac(ain->codecpar, &ain->time_base,
                                     ctx->stream_name, &pending) < 0 || !pending) {
                log_warn("[%s] Failed to prepare AAC codec params — disabling audio for this recording",
                         ctx->stream_name);
                mp4_writer_set_audio(ctx->mp4_writer, 0);
            } else {
                log_info("[%s] PCM->AAC codec params prepared for MP4 header (eager init)",
                         ctx->stream_name);
            }
        } else if (is_audio_codec_compatible_with_mp4(ain->codecpar->codec_id, &codec_name)) {
            // Already MP4-compatible (AAC, MP3, AC3, Opus): deep-copy codec params.
            // Packets will be muxed as-is by mp4_writer_write_packet().
            pending = avcodec_parameters_alloc();
            if (!pending || avcodec_parameters_copy(pending, ain->codecpar) < 0) {
                log_warn("[%s] Failed to copy %s codec params — disabling audio for this recording",
                         ctx->stream_name, codec_name);
                if (pending) { avcodec_parameters_free(&pending); pending = NULL; }
                mp4_writer_set_audio(ctx->mp4_writer, 0);
            } else {
                log_info("[%s] %s codec params prepared for MP4 header (eager init)",
                         ctx->stream_name, codec_name);
            }
        } else {
            // Incompatible and not PCM: cannot mux or transcode.
            is_audio_codec_compatible_with_mp4(ain->codecpar->codec_id, &codec_name);
            log_warn("[%s] Audio codec %s is not MP4-compatible and not PCM — disabling audio for this recording",
                     ctx->stream_name, codec_name);
            mp4_writer_set_audio(ctx->mp4_writer, 0);
        }

        if (pending) {
            ctx->mp4_writer->pending_audio_codecpar = pending;
            // Store the original input time_base so mp4_writer_initialize() does
            // not have to reconstruct it from sample_rate (which may be 0).
            ctx->mp4_writer->pending_audio_time_base = ain->time_base;
        }
    } else {
        mp4_writer_set_audio(ctx->mp4_writer, 0);
        if (ctx->record_audio && ctx->audio_stream_idx < 0) {
            log_warn("[%s] Audio recording requested but no audio stream found", ctx->stream_name);
        }
    }

    // Set trigger type to detection
    safe_strcpy(ctx->mp4_writer->trigger_type, "detection", sizeof(ctx->mp4_writer->trigger_type), 0);

    // Store recording start time
    ctx->mp4_writer->creation_time = now;

    // Add recording to database at START (so it appears in recordings list immediately)
    // It will be updated with end_time, size, and is_complete=true when recording stops
    recording_metadata_t metadata = {0};
    safe_strcpy(metadata.file_path, ctx->current_recording_path, sizeof(metadata.file_path), 0);
    safe_strcpy(metadata.stream_name, ctx->stream_name, sizeof(metadata.stream_name), 0);
    metadata.start_time = now;
    metadata.end_time = 0;  // Will be set when recording stops
    metadata.size_bytes = 0;  // Will be set when recording stops
    metadata.is_complete = false;  // Will be set to true when recording stops
    safe_strcpy(metadata.trigger_type, "detection", sizeof(metadata.trigger_type), 0);

    ctx->current_recording_id = add_recording_metadata(&metadata);
    if (ctx->current_recording_id > 0) {
        log_info("[%s] Added detection recording to database (ID: %lu) for file: %s",
                 ctx->stream_name, (unsigned long)ctx->current_recording_id, ctx->current_recording_path);
    } else {
        log_warn("[%s] Failed to add detection recording to database", ctx->stream_name);
    }

    metrics_set_recording_active(ctx->stream_name, true);

    pthread_mutex_lock(&ctx->mutex);
    ctx->total_recordings++;
    pthread_mutex_unlock(&ctx->mutex);

    log_info("[%s] Detection recording started successfully", ctx->stream_name);
    return 0;
}

/**
 * Stop recording - close MP4 writer and update database
 * In annotation_only mode, this is a no-op since no MP4 was created
 */
static int udt_stop_recording(unified_detection_ctx_t *ctx) {
    if (!ctx) return -1;

    // In annotation-only mode, no MP4 was created, so nothing to stop
    if (ctx->annotation_only) {
        log_debug("[%s] Annotation-only mode: no MP4 to stop", ctx->stream_name);
        return 0;
    }

    if (!ctx->mp4_writer) return -1;

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

    metrics_set_recording_active(ctx->stream_name, false);
    metrics_record_segment_complete(ctx->stream_name, start_time, end_time, file_size);

    // Close MP4 writer
    mp4_writer_close(ctx->mp4_writer);
    ctx->mp4_writer = NULL;

    // Update the existing database record (was created at recording start)
    if (ctx->current_recording_id > 0) {
        // Update the existing recording with end_time, size, and mark as complete
        if (update_recording_metadata(ctx->current_recording_id, end_time, file_size, true) == 0) {
            log_info("[%s] Recording updated in database (ID: %lu, duration: %ds, size: %ld bytes)",
                     ctx->stream_name, (unsigned long)ctx->current_recording_id, duration, (long)file_size);
            // Keep stream storage cache current so System page stats are up-to-date.
            update_stream_storage_cache_add_recording(ctx->stream_name, (uint64_t)file_size);
        } else {
            log_warn("[%s] Failed to update recording in database (ID: %lu)",
                     ctx->stream_name, (unsigned long)ctx->current_recording_id);
        }
    } else if (ctx->current_recording_path[0] != '\0') {
        // Fallback: if no recording_id, try to add a new record (shouldn't happen normally)
        log_warn("[%s] No recording ID found, creating new database entry", ctx->stream_name);
        recording_metadata_t metadata = {0};
        safe_strcpy(metadata.file_path, ctx->current_recording_path, sizeof(metadata.file_path), 0);
        safe_strcpy(metadata.stream_name, ctx->stream_name, sizeof(metadata.stream_name), 0);
        metadata.start_time = start_time;
        metadata.end_time = end_time;
        metadata.size_bytes = file_size;
        metadata.is_complete = true;
        safe_strcpy(metadata.trigger_type, "detection", sizeof(metadata.trigger_type), 0);

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
static bool run_detection_on_frame(unified_detection_ctx_t *ctx, AVPacket *pkt,
                                   time_t frame_timestamp) {
    if (!ctx) return false;

    /* frame_timestamp reflects when the packet entered the pipeline
     * (captured at the top of process_packet).  Reuse it for reporting,
     * DB writes, and MQTT so detections.timestamp records frame time
     * rather than inference-finished time. */
    time_t now = frame_timestamp;

    /* API detection: try the go2rtc snapshot shortcut first.
     * The snapshot path handles zone filtering, DB storage, and MQTT
     * internally so we only need to check the threshold on its result.
     * If the snapshot is unavailable we fall through to the shared
     * frame-decode path below. */
    if (is_api_detection(ctx->model_path)) {
        log_debug("[%s] Running API detection via snapshot", ctx->stream_name);

        uint64_t rec_id = 0;
        if (ctx->annotation_only) {
            rec_id = get_current_recording_id_for_stream(ctx->stream_name);
        } else if (ctx->current_recording_id > 0) {
            rec_id = ctx->current_recording_id;
        }

        detection_result_t result;
        memset(&result, 0, sizeof(result));
        int detect_ret = detect_objects_api_snapshot(ctx->model_path, ctx->stream_name,
                                                     &result, ctx->detection_threshold,
                                                     rec_id, frame_timestamp);

        if (detect_ret != DETECT_SNAPSHOT_UNAVAILABLE) {
            if (detect_ret != 0) {
                log_warn("[%s] API detection failed with error %d", ctx->stream_name, detect_ret);
                return false;
            }
            /* Snapshot succeeded — snapshot handles DB storage internally.
             * Call report_detections for stats only (no-op for DB on API). */
            report_detections(ctx, &result, now);
            for (int i = 0; i < result.count; i++) {
                if (result.detections[i].confidence >= ctx->detection_threshold) {
                    log_info("[%s] API Detection: %s (%.1f%%) at [%.2f, %.2f, %.2f, %.2f]",
                             ctx->stream_name,
                             result.detections[i].label,
                             result.detections[i].confidence * 100.0f,
                             result.detections[i].x, result.detections[i].y,
                             result.detections[i].width, result.detections[i].height);
                    return true;
                }
            }
            return false;
        }

        log_info("[%s] go2rtc snapshot unavailable, falling back to frame decode",
                 ctx->stream_name);
        /* Fall through to frame-decode path below. */
    }

    /* ONVIF: event-based — no frame needed, just read the async-thread flag.
     *
     * detect_motion_onvif() is NOT called here.  A dedicated background thread
     * (start_onvif_detection_thread) polls the camera and writes the result
     * into ctx->onvif_motion_detected, keeping av_read_frame() unblocked. */
    if (is_onvif_detection_model(ctx->model_path)) {
        bool triggered = (atomic_load(&ctx->onvif_motion_detected) != 0);
        if (triggered) {
            pthread_mutex_lock(&ctx->mutex);
            ctx->total_detections++;
            pthread_mutex_unlock(&ctx->mutex);
            log_info("[%s] ONVIF motion detected (async thread, ts=%lld)",
                     ctx->stream_name,
                     (long long)atomic_load(&ctx->onvif_motion_timestamp));
        }
        return triggered;
    }

    /* All other models (motion, SOD/TFLite, API snapshot fallback):
     * decode packet → detect_on_decoded_frame → report_detections. */
    if (!pkt || !ctx->decoder_ctx) return false;

    if (avcodec_send_packet(ctx->decoder_ctx, pkt) < 0) return false;

    AVFrame *frame = av_frame_alloc();
    if (!frame) return false;

    if (avcodec_receive_frame(ctx->decoder_ctx, frame) < 0) {
        av_frame_free(&frame);
        return false;
    }

    detection_result_t result2;
    bool hit = detect_on_decoded_frame(ctx, frame, now, &result2);
    av_frame_free(&frame);

    if (hit) report_detections(ctx, &result2, now);
    return hit;
}
/* ============================================================
 * Detection layer — pure inference, no side effects
 * ============================================================ */

/**
 * Run the configured model on a pre-decoded frame and return the surviving
 * detections via *result.  Returns true when result->count > 0.
 *
 * Pure computation: zone + object + threshold filters are applied, but no DB
 * writes, no recording-state changes, and no MQTT publishing happen here.
 * Callers pass the result to report_detections() then handle_recording_state().
 *
 * Exception: API backends (detect_objects_api / detect_objects_api_snapshot)
 * store results in the DB internally — this is their pre-existing design.
 * report_detections() is therefore a no-op for API models.
 *
 * Handles: API (frame path), motion detection, SOD/TFLite embedded models.
 * Does NOT handle ONVIF (event-based, no frame needed).
 */
/* Convert a decoded frame to a freshly malloc'd packed RGB24 buffer at the
 * frame's own resolution (caller frees). Returns NULL on failure. */
static uint8_t *frame_to_rgb24(const AVFrame *frame) {
    struct SwsContext *sws_ctx = sws_getContext(
        frame->width, frame->height, frame->format,
        frame->width, frame->height, AV_PIX_FMT_RGB24,
        SWS_BILINEAR, NULL, NULL, NULL);
    if (!sws_ctx) return NULL;

    uint8_t *rgb_buf = malloc((size_t)frame->width * frame->height * 3);
    if (!rgb_buf) { sws_freeContext(sws_ctx); return NULL; }

    uint8_t *rgb_data[4] = {rgb_buf, NULL, NULL, NULL};
    int rgb_linesize[4]  = {frame->width * 3, 0, 0, 0};
    sws_scale(sws_ctx, (const uint8_t * const *)frame->data, frame->linesize,
              0, frame->height, rgb_data, rgb_linesize);
    sws_freeContext(sws_ctx);
    return rgb_buf;
}

static bool detect_on_decoded_frame(unified_detection_ctx_t *ctx,
                                    AVFrame *frame, time_t now,
                                    detection_result_t *result) {
    if (!ctx || !frame || !result) return false;

    memset(result, 0, sizeof(*result));

    int width  = frame->width;
    int height = frame->height;

    /* ---- model dispatch ---- */

    if (is_api_detection(ctx->model_path)) {
        /* API backends store in DB internally using rec_id for recording linking. */
        uint64_t rec_id = ctx->annotation_only
            ? get_current_recording_id_for_stream(ctx->stream_name)
            : ctx->current_recording_id;

        uint8_t *rgb_buf = frame_to_rgb24(frame);
        if (!rgb_buf) return false;

        const char *api_url = get_actual_api_url(ctx->stream_name, ctx->model_path);
        int ret = api_url
            ? detect_objects_api(api_url, rgb_buf, width, height, 3,
                                 result, ctx->stream_name, ctx->detection_threshold,
                                 rec_id, now)
            : -1;
        free(rgb_buf);

        if (ret != 0) {
            log_warn("[%s] API detection (frame path) failed with error %d",
                     ctx->stream_name, ret);
            return false;
        }

    } else if (is_motion_detection_model(ctx->model_path)) {
        uint8_t *rgb_buf = frame_to_rgb24(frame);
        if (!rgb_buf) return false;

        int ret = detect_motion(ctx->stream_name, rgb_buf, width, height, 3, now, result);
        free(rgb_buf);

        if (ret != 0) {
            log_warn("[%s] Motion detection failed with error %d", ctx->stream_name, ret);
            return false;
        }

    } else {
        /* Embedded model (SOD / TFLite / etc.). */
        if (!ctx->model) {
            if (ctx->model_path[0] == '\0') return false;
            if (ctx->model_load_failed) {
                /* One-shot hard error already reported; don't retry. */
                return false;
            }
            ctx->model = load_detection_model(ctx->model_path, ctx->detection_threshold);
            if (!ctx->model) {
                /* Hard error: drive the stream into STREAM_STATE_ERROR so the
                 * UI surfaces the cause via streams.error_message. The specific
                 * reason (missing file, unsupported dtype, etc.) was already
                 * logged by load_detection_model / the engine. */
                /* Scratch sized exactly for the prefix + longest possible
                 * model_path (sizeof includes the null). handle_stream_error()
                 * truncates to STREAM_ERROR_MESSAGE_MAX when persisting. */
                static const char kPrefix[] = "Failed to load detection model: ";
                char msg[sizeof kPrefix + MAX_PATH_LENGTH];
                snprintf(msg, sizeof msg, "%s%s", kPrefix, ctx->model_path);
                log_error("[%s] %s", ctx->stream_name, msg);
                stream_state_manager_t *sm = get_stream_state_by_name(ctx->stream_name);
                if (sm) {
                    handle_stream_error(sm, STREAM_ERR_MODEL_LOAD, msg);
                }
                ctx->model_load_failed = true;
                return false;
            }
            log_info("[%s] Loaded detection model: %s", ctx->stream_name, ctx->model_path);
        }

        uint8_t *rgb_buf = frame_to_rgb24(frame);
        if (!rgb_buf) {
            log_error("[%s] Failed to convert frame to RGB24", ctx->stream_name);
            return false;
        }

        int ret = detect_objects(ctx->model, rgb_buf, width, height, 3, result);
        free(rgb_buf);

        if (ret != 0) {
            log_warn("[%s] Embedded model detection failed with error %d",
                     ctx->stream_name, ret);
            return false;
        }
    }

    /* ---- post-processing: common to all model types ---- */

    if (result->count == 0) return false;

    filter_detections_by_zones(ctx->stream_name, result);
    filter_detections_by_stream_objects(ctx->stream_name, result);

    /* Compact in-place: discard detections below threshold. */
    int kept = 0;
    for (int i = 0; i < result->count; i++) {
        if (result->detections[i].confidence >= ctx->detection_threshold) {
            log_info("[%s] Detection: %s (%.1f%%) at [%.2f, %.2f, %.2f, %.2f]",
                     ctx->stream_name,
                     result->detections[i].label,
                     result->detections[i].confidence * 100.0f,
                     result->detections[i].x, result->detections[i].y,
                     result->detections[i].width, result->detections[i].height);
            if (kept != i) result->detections[kept] = result->detections[i];
            kept++;
        } else {
            log_debug("[%s] Detection below threshold: %s (%.1f%% < %.1f%%)",
                      ctx->stream_name,
                      result->detections[i].label,
                      result->detections[i].confidence * 100.0f,
                      ctx->detection_threshold * 100.0f);
        }
    }
    result->count = kept;
    return result->count > 0;
}

/* ============================================================
 * Action layer
 * ============================================================ */

/**
 * Store detection results in the database and update stream statistics.
 *
 * No-op for API models: detect_objects_api / detect_objects_api_snapshot
 * already write to the DB internally.  For all other models (motion, SOD,
 * TFLite) this is the single place where DB writes and stats happen.
 */
static void report_detections(unified_detection_ctx_t *ctx,
                               const detection_result_t *result, time_t now) {
    if (!ctx || !result || result->count == 0) return;

    if (!is_api_detection(ctx->model_path)) {
        uint64_t rec_id = ctx->annotation_only
            ? get_current_recording_id_for_stream(ctx->stream_name)
            : ctx->current_recording_id;

        if (store_detections_in_db(ctx->stream_name, result, now, rec_id) != 0)
            log_warn("[%s] Failed to store detections in database", ctx->stream_name);

        // Keep MQTT/Home Assistant topics in sync for the local detection
        // backends (motion, SOD, TFLite). API backends publish from inside
        // detect_objects_api / *_snapshot, so they are excluded here.
        mqtt_publish_detection(ctx->stream_name, result, now);
        mqtt_set_motion_state(ctx->stream_name, result);
    }

    pthread_mutex_lock(&ctx->mutex);
    ctx->total_detections += (uint64_t)result->count;
    pthread_mutex_unlock(&ctx->mutex);
}

/**
 * Drive the recording state machine based on whether detection fired this
 * interval.  This is the single place where BUFFERING→RECORDING,
 * RECORDING→POST_BUFFER, and POST_BUFFER→RECORDING transitions happen for
 * the frame-based detection paths (main stream and detection stream).
 *
 * ONVIF and cross-stream motion triggers follow the external_motion_trigger
 * path in process_packet and do not call this function.
 */
static void handle_recording_state(unified_detection_ctx_t *ctx,
                                   bool detected, time_t now) {
    if (!ctx) return;

    unified_detection_state_t current_state =
        (unified_detection_state_t)atomic_load(&ctx->state);

    if (detected) {
        atomic_store(&ctx->last_detection_time, (long long)now);

        if (ctx->annotation_only) return;

        if (current_state == UDT_STATE_BUFFERING) {
            log_info("[%s] Detection triggered, starting recording", ctx->stream_name);

            if (udt_start_recording(ctx) == 0) {
                int pre_dur = 0, pre_cnt = 0;
                size_t pre_mem = 0;
                if (ctx->packet_buffer)
                    packet_buffer_get_stats(ctx->packet_buffer, &pre_cnt, &pre_mem, &pre_dur);

                flush_prebuffer_to_recording(ctx);
                atomic_store(&ctx->state, UDT_STATE_RECORDING);

                if (!ctx->mp4_writer->start_time_corrected && pre_dur > 0 &&
                    ctx->current_recording_id > 0) {
                    int clamped_pre = pre_dur > ctx->pre_buffer_seconds
                                      ? ctx->pre_buffer_seconds : pre_dur;
                    time_t corrected = now - (time_t)clamped_pre;
                    ctx->mp4_writer->creation_time = corrected;
                    ctx->mp4_writer->start_time_corrected = true;
                    update_recording_start_time(ctx->current_recording_id, corrected);
                    log_info("[%s] Corrected recording start_time by -%ds (pre-buffer, clamped from %ds)",
                             ctx->stream_name, clamped_pre, pre_dur);
                }

                /* detection_interval is forced > 0 in start_unified_detection_thread
                 * (zero / negative fall back to DEFAULT_DETECTION_INTERVAL), so the
                 * lookback is always at least 1 s + grace_period. */
                time_t lookback = now - (ctx->detection_interval + g_config.detection_grace_period);
                int updated = update_detections_recording_id(
                    ctx->stream_name, ctx->current_recording_id, lookback);
                if (updated > 0)
                    log_debug("[%s] Linked %d recent detections to recording ID %lu",
                              ctx->stream_name, updated,
                              (unsigned long)ctx->current_recording_id);
            }

        } else if (current_state == UDT_STATE_POST_BUFFER) {
            log_info("[%s] Detection during post-buffer, continuing recording",
                     ctx->stream_name);
            atomic_store(&ctx->state, UDT_STATE_RECORDING);
        }
        /* UDT_STATE_RECORDING: last_detection_time already refreshed above. */

    } else if (!ctx->annotation_only && current_state == UDT_STATE_RECORDING) {
        time_t since_last = now - (time_t)atomic_load(&ctx->last_detection_time);
        // Detection is only sampled once per detection_interval, so last_detection_time
        // is inherently stale by up to one interval even while a scene stays active.
        // The recording must therefore survive at least one full sampling gap before
        // grace_period applies — otherwise (e.g. interval=5s, grace=2s) a continuous
        // scene would drop to post-buffer between samples and fragment the clip.
        // This mirrors the detection-linking lookback above.
        time_t grace_window = (time_t)ctx->detection_interval + (time_t)g_config.detection_grace_period;
        if (since_last > grace_window) {
            log_info("[%s] No detection for %lds, entering post-buffer (%d seconds)",
                     ctx->stream_name, (long)since_last, ctx->post_buffer_seconds);
            atomic_store(&ctx->post_buffer_end_time,
                         (long long)(now + ctx->post_buffer_seconds));
            atomic_store(&ctx->state, UDT_STATE_POST_BUFFER);
        }
    }
}
