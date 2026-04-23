/**
 * @file thumbnail_thread.c
 * @brief Detached pthread-based thumbnail generation with async completion
 */

#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <pthread.h>
#include <sys/stat.h>
#include <unistd.h>
#include <fcntl.h>
#include <time.h>
#include <errno.h>

#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <libavutil/avutil.h>
#include <libavutil/imgutils.h>
#include <libswscale/swscale.h>

#include "web/thumbnail_thread.h"
#include "core/config.h"
#define LOG_COMPONENT "Thumbnail"
#include "core/logger.h"
#include "utils/memory.h"
#include "utils/strings.h"
#include "video/ffmpeg_utils.h"

// Maximum concurrent thumbnail generations
#define MAX_CONCURRENT_THUMBNAILS 4

// Thumbnail output width (height is derived to preserve aspect ratio).
#define THUMBNAIL_WIDTH 320

// Per-decode wall-clock budget. Generous fallback when seeking into a
// slow-to-demux container on low-end hardware; a healthy decode on an
// Atom CPU finishes in well under a second.
#define THUMBNAIL_DECODE_BUDGET_SEC 5

/**
 * @brief Work item for thumbnail generation
 */
typedef struct thumbnail_work {
    uint64_t recording_id;
    int index;
    char input_path[MAX_PATH_LENGTH];
    char output_path[MAX_PATH_LENGTH];
    double seek_seconds;
    deferred_action_handle_t deferred_action;
    deferred_response_callback_t callback;
    int result;  // 0 = success, -1 = failure
    struct thumbnail_work *next;
} thumbnail_work_t;

/**
 * @brief Global state for the thumbnail thread subsystem
 */
static struct {
    uv_loop_t *loop;
    uv_async_t async_handle;
    pthread_mutex_t done_mutex;
    thumbnail_work_t *done_queue_head;
    thumbnail_work_t *done_queue_tail;
    volatile int active_count;
    volatile bool shutting_down;
} g_thumbnail_state = {0};

/**
 * @brief Decode one frame from @p input_path near @p seek_seconds, scale to
 *        THUMBNAIL_WIDTH preserving aspect, and write a JPEG to @p output_path.
 *
 * Calls libavformat/libavcodec/libswscale directly — no fork/exec of the
 * ffmpeg binary. On an Atom D245 the fork+exec path was dominated by
 * ~900 ms of startup overhead per thumbnail (issue #364); this variant
 * skips that entirely and decodes in-process.
 *
 * Seek strategy: when @p seek_seconds > 0, seek BACKWARD to the nearest
 * keyframe and decode forward. When @p seek_seconds == 0, skip the seek
 * and decode from the first packet — the cheapest path and the default
 * for the index-0 mount-time thumbnail.
 */
static int generate_thumbnail_internal(const char *input_path, const char *output_path,
                                       double seek_seconds) {
    if (seek_seconds < 0) seek_seconds = 0;

    log_debug("Generating thumbnail (libav): seek=%.2fs \"%s\" -> \"%s\"",
              seek_seconds, input_path, output_path);

    AVFormatContext *fmt_ctx = NULL;
    AVCodecContext *dec_ctx = NULL;
    AVFrame *frame = NULL;
    AVPacket *pkt = NULL;
    struct SwsContext *sws_ctx = NULL;
    uint8_t *rgb_buf = NULL;
    int video_stream_idx = -1;
    int ret = -1;

    if (avformat_open_input(&fmt_ctx, input_path, NULL, NULL) < 0) {
        log_warn("Thumbnail: avformat_open_input failed for %s", input_path);
        goto done;
    }
    if (avformat_find_stream_info(fmt_ctx, NULL) < 0) {
        log_warn("Thumbnail: avformat_find_stream_info failed for %s", input_path);
        goto done;
    }

    const AVCodec *decoder = NULL;
    video_stream_idx = av_find_best_stream(fmt_ctx, AVMEDIA_TYPE_VIDEO, -1, -1, &decoder, 0);
    if (video_stream_idx < 0 || !decoder) {
        log_warn("Thumbnail: no video stream in %s", input_path);
        goto done;
    }

    AVStream *vs = fmt_ctx->streams[video_stream_idx];
    dec_ctx = avcodec_alloc_context3(decoder);
    if (!dec_ctx) goto done;
    if (avcodec_parameters_to_context(dec_ctx, vs->codecpar) < 0) goto done;
    if (avcodec_open2(dec_ctx, decoder, NULL) < 0) {
        log_warn("Thumbnail: avcodec_open2 failed for %s", input_path);
        goto done;
    }

    // Seek to the nearest keyframe at or before seek_seconds. Skipped entirely
    // when seek_seconds==0 — that's the whole point of gap #1 in #364.
    if (seek_seconds > 0) {
        int64_t target_ts = (int64_t)(seek_seconds * AV_TIME_BASE);
        if (av_seek_frame(fmt_ctx, -1, target_ts, AVSEEK_FLAG_BACKWARD) < 0) {
            log_debug("Thumbnail: seek to %.2fs failed, decoding from start", seek_seconds);
        } else {
            avcodec_flush_buffers(dec_ctx);
        }
    }

    pkt = av_packet_alloc();
    frame = av_frame_alloc();
    if (!pkt || !frame) goto done;

    time_t decode_deadline = time(NULL) + THUMBNAIL_DECODE_BUDGET_SEC;
    bool got_frame = false;
    while (!got_frame) {
        if (time(NULL) > decode_deadline) {
            log_warn("Thumbnail: decode budget exceeded for %s", input_path);
            goto done;
        }

        int read_ret = av_read_frame(fmt_ctx, pkt);
        if (read_ret == AVERROR_EOF) {
            // Flush decoder
            avcodec_send_packet(dec_ctx, NULL);
        } else if (read_ret < 0) {
            log_warn("Thumbnail: av_read_frame failed (%d) for %s", read_ret, input_path);
            goto done;
        } else if (pkt->stream_index != video_stream_idx) {
            av_packet_unref(pkt);
            continue;
        } else {
            if (avcodec_send_packet(dec_ctx, pkt) < 0) {
                av_packet_unref(pkt);
                continue;
            }
            av_packet_unref(pkt);
        }

        int recv_ret = avcodec_receive_frame(dec_ctx, frame);
        if (recv_ret == AVERROR(EAGAIN)) {
            if (read_ret == AVERROR_EOF) goto done; // nothing more to feed
            continue;
        } else if (recv_ret == AVERROR_EOF) {
            goto done;
        } else if (recv_ret < 0) {
            goto done;
        }
        got_frame = true;
    }

    // Preserve aspect ratio: width fixed at THUMBNAIL_WIDTH, height derived.
    // Round to even to keep YUVJ420P happy downstream.
    int out_w = THUMBNAIL_WIDTH;
    int out_h = (int)((double)frame->height * out_w / frame->width + 0.5);
    if (out_h < 2) out_h = 2;
    if (out_h & 1) out_h++;

    sws_ctx = sws_getContext(frame->width, frame->height, dec_ctx->pix_fmt,
                             out_w, out_h, AV_PIX_FMT_RGB24,
                             SWS_BILINEAR, NULL, NULL, NULL);
    if (!sws_ctx) {
        log_warn("Thumbnail: sws_getContext failed for %s", input_path);
        goto done;
    }

    int rgb_linesize = out_w * 3;
    rgb_buf = av_malloc((size_t)rgb_linesize * out_h);
    if (!rgb_buf) goto done;

    uint8_t *dst_data[4] = {rgb_buf, NULL, NULL, NULL};
    int dst_linesize[4] = {rgb_linesize, 0, 0, 0};
    sws_scale(sws_ctx, (const uint8_t *const *)frame->data, frame->linesize,
              0, frame->height, dst_data, dst_linesize);

    // Reuse the shared MJPEG encoder helper. Quality 70 is the rough
    // equivalent of the legacy `-q:v 8` CLI invocation we replaced.
    if (ffmpeg_encode_jpeg(rgb_buf, out_w, out_h, 3, 70, output_path) != 0) {
        log_warn("Thumbnail: ffmpeg_encode_jpeg failed for %s", output_path);
        goto done;
    }

    struct stat st;
    if (stat(output_path, &st) != 0 || st.st_size == 0) {
        log_warn("Thumbnail file not created or empty: %s", output_path);
        unlink(output_path);
        goto done;
    }

    log_debug("Generated thumbnail: %s (%lld bytes, %dx%d)", output_path,
              (long long)st.st_size, out_w, out_h);
    ret = 0;

done:
    if (rgb_buf) av_free(rgb_buf);
    if (sws_ctx) sws_freeContext(sws_ctx);
    if (frame) av_frame_free(&frame);
    if (pkt) av_packet_free(&pkt);
    if (dec_ctx) avcodec_free_context(&dec_ctx);
    if (fmt_ctx) avformat_close_input(&fmt_ctx);
    return ret;
}

/**
 * @brief Add a work item to the done queue (thread-safe)
 */
static void enqueue_done(thumbnail_work_t *work) {
    pthread_mutex_lock(&g_thumbnail_state.done_mutex);
    work->next = NULL;
    if (g_thumbnail_state.done_queue_tail) {
        g_thumbnail_state.done_queue_tail->next = work;
    } else {
        g_thumbnail_state.done_queue_head = work;
    }
    g_thumbnail_state.done_queue_tail = work;
    pthread_mutex_unlock(&g_thumbnail_state.done_mutex);
}

/**
 * @brief Dequeue all completed work items (thread-safe)
 */
static thumbnail_work_t *dequeue_all_done(void) {
    pthread_mutex_lock(&g_thumbnail_state.done_mutex);
    thumbnail_work_t *head = g_thumbnail_state.done_queue_head;
    g_thumbnail_state.done_queue_head = NULL;
    g_thumbnail_state.done_queue_tail = NULL;
    pthread_mutex_unlock(&g_thumbnail_state.done_mutex);
    return head;
}

/**
 * @brief Detached pthread worker function
 */
static void *thumbnail_worker_thread(void *arg) {
    log_set_thread_context("Thumbnail", NULL);
    thumbnail_work_t *work = (thumbnail_work_t *)arg;

    // Generate the thumbnail
    work->result = generate_thumbnail_internal(work->input_path, work->output_path,
                                               work->seek_seconds);

    // Add to done queue
    enqueue_done(work);

    // Signal the event loop via uv_async
    uv_async_send(&g_thumbnail_state.async_handle);

    // Decrement active count
    __sync_sub_and_fetch(&g_thumbnail_state.active_count, 1);

    return NULL;
}

/**
 * @brief uv_async callback - runs on the event loop thread
 *
 * Processes all completed thumbnail generations and sends responses.
 */
static void thumbnail_async_cb(uv_async_t *handle) {
    (void)handle;

    thumbnail_work_t *work = dequeue_all_done();
    while (work) {
        thumbnail_work_t *next = work->next;

        log_debug("Thumbnail generation completed for recording %llu index %d: %s",
                  (unsigned long long)work->recording_id, work->index,
                  work->result == 0 ? "success" : "failure");

        // Invoke the callback to send the response
        if (work->callback) {
            const char *output_path = work->result == 0 ? work->output_path : NULL;
            work->callback(work->deferred_action, output_path, work->result);
        }

        safe_free(work);
        work = next;
    }
}

// ============================================================================
// Public API
// ============================================================================

int thumbnail_thread_init(uv_loop_t *loop) {
    if (!loop) {
        log_error("thumbnail_thread_init: NULL loop");
        return -1;
    }

    memset(&g_thumbnail_state, 0, sizeof(g_thumbnail_state));
    g_thumbnail_state.loop = loop;

    // Initialize mutex
    if (pthread_mutex_init(&g_thumbnail_state.done_mutex, NULL) != 0) {
        log_error("thumbnail_thread_init: Failed to initialize mutex");
        return -1;
    }

    // Initialize uv_async handle
    if (uv_async_init(loop, &g_thumbnail_state.async_handle, thumbnail_async_cb) != 0) {
        log_error("thumbnail_thread_init: Failed to initialize uv_async");
        pthread_mutex_destroy(&g_thumbnail_state.done_mutex);
        return -1;
    }

    log_info("thumbnail_thread_init: Thumbnail thread subsystem initialized");
    return 0;
}

void thumbnail_thread_shutdown(void) {
    log_info("thumbnail_thread_shutdown: Shutting down thumbnail thread subsystem");
    g_thumbnail_state.shutting_down = true;

    // Wait for all active threads to complete
    int wait_count = 0;
    while (__sync_fetch_and_add(&g_thumbnail_state.active_count, 0) > 0 && wait_count < 100) {
        usleep(100000); // 100ms
        wait_count++;
    }

    if (g_thumbnail_state.active_count > 0) {
        log_warn("thumbnail_thread_shutdown: %d threads still active after timeout",
                 g_thumbnail_state.active_count);
    }

    // Close uv_async handle
    if (!uv_is_closing((uv_handle_t *)&g_thumbnail_state.async_handle)) {
        uv_close((uv_handle_t *)&g_thumbnail_state.async_handle, NULL);
    }

    // Destroy mutex
    pthread_mutex_destroy(&g_thumbnail_state.done_mutex);

    // Free any remaining work items in the done queue
    thumbnail_work_t *work = g_thumbnail_state.done_queue_head;
    while (work) {
        thumbnail_work_t *next = work->next;
        safe_free(work);
        work = next;
    }

    log_info("thumbnail_thread_shutdown: Shutdown complete");
}

int thumbnail_thread_submit(uint64_t recording_id, int index,
                            const char *input_path, const char *output_path,
                            double seek_seconds, deferred_action_handle_t deferred_action,
                            deferred_response_callback_t callback) {
    if (g_thumbnail_state.shutting_down) {
        log_warn("thumbnail_thread_submit: Rejecting request during shutdown");
        return -1;
    }

    // Enforce concurrency limit
    if (__sync_fetch_and_add(&g_thumbnail_state.active_count, 0) >= MAX_CONCURRENT_THUMBNAILS) {
        log_debug("thumbnail_thread_submit: Max concurrent thumbnails reached");
        return -1;
    }

    // Allocate work item
    thumbnail_work_t *work = safe_calloc(1, sizeof(thumbnail_work_t));
    if (!work) {
        log_error("thumbnail_thread_submit: Failed to allocate work item");
        return -1;
    }

    work->recording_id = recording_id;
    work->index = index;
    safe_strcpy(work->input_path, input_path, sizeof(work->input_path), 0);
    safe_strcpy(work->output_path, output_path, sizeof(work->output_path), 0);
    work->seek_seconds = seek_seconds;
    work->deferred_action = deferred_action;
    work->callback = callback;

    // Increment active count
    __sync_add_and_fetch(&g_thumbnail_state.active_count, 1);

    // Spawn detached thread
    pthread_t thread;
    pthread_attr_t attr;
    pthread_attr_init(&attr);
    pthread_attr_setdetachstate(&attr, PTHREAD_CREATE_DETACHED);

    if (pthread_create(&thread, &attr, thumbnail_worker_thread, work) != 0) {
        log_error("thumbnail_thread_submit: Failed to create thread");
        pthread_attr_destroy(&attr);
        __sync_sub_and_fetch(&g_thumbnail_state.active_count, 1);
        safe_free(work);
        return -1;
    }

    pthread_attr_destroy(&attr);
    log_debug("thumbnail_thread_submit: Submitted thumbnail generation for recording %llu index %d",
              (unsigned long long)recording_id, index);
    return 0;
}

int thumbnail_thread_get_active_count(void) {
    return __sync_fetch_and_add(&g_thumbnail_state.active_count, 0);
}

