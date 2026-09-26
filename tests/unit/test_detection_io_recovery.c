#define _GNU_SOURCE
#define _POSIX_C_SOURCE 200809L

#include <arpa/inet.h>
#include <sys/socket.h>
#include "unity.h"

/* Exercise the actual private reader loops and callbacks without exporting
 * test-only entry points. The archive's copy of this translation unit is not
 * pulled in: these definitions satisfy its public symbols too. */
#include "../../src/video/unified_detection_thread.c"

typedef enum { STALL_OPEN, STALL_PROBE, STALL_READ, REAL_IO } stall_stage_t;
static stall_stage_t stage;
static unified_detection_ctx_t *test_ctx;
static int opens, reads, interrupted;
static int64_t clock_offset;
static int listener = -1;

int64_t __real_av_gettime_relative(void);
int __real_avformat_open_input(AVFormatContext **input, const char *url,
                              const AVInputFormat *format, AVDictionary **opts);
int __real_avformat_find_stream_info(AVFormatContext *input, AVDictionary **opts);

int64_t __wrap_av_gettime_relative(void) {
    return __real_av_gettime_relative() + clock_offset;
}

static void stop_readers(void) {
    atomic_store(&test_ctx->running, 0);
    atomic_store(&test_ctx->detection_stream_thread_running, 0);
}

/* Simulate a blocking libav call that periodically consults the application's
 * interrupt callback. Advance only the monotonic clock: wall time need not
 * advance for a blocked operation to abort and enter reconnect. */
static int stall(AVFormatContext *input) {
    clock_offset += 11LL * AV_TIME_BASE;
    if (input->interrupt_callback.callback &&
        input->interrupt_callback.callback(input->interrupt_callback.opaque)) {
        ++interrupted;
    } else {
        // Bound the test even on the old shutdown-only callback.
        stop_readers();
    }
    return AVERROR_EXIT;
}

int __wrap_avformat_open_input(AVFormatContext **input, const char *url,
                              const AVInputFormat *format, AVDictionary **opts) {
    if (stage == REAL_IO)
        return __real_avformat_open_input(input, url, format, opts);
    if (++opens > 1) {
        stop_readers();
        avformat_free_context(*input);
        *input = NULL;
        return AVERROR_EOF;
    }
    if (stage == STALL_OPEN) {
        int ret = stall(*input);
        avformat_free_context(*input);
        *input = NULL;
        return ret;
    }
    // Real demuxer and decoder setup, with a local fixture instead of a camera.
    return __real_avformat_open_input(input, TEST_RECORDING_PATH, NULL, NULL);
}

int __wrap_avformat_find_stream_info(AVFormatContext *input, AVDictionary **opts) {
    if (stage == STALL_PROBE) return stall(input);
    return __real_avformat_find_stream_info(input, opts);
}

int __wrap_av_read_frame(AVFormatContext *input, AVPacket *pkt) {
    (void)pkt;
    ++reads;
    if (reads > 1) stop_readers();
    return stall(input);
}

void setUp(void) {
    load_default_config(&g_config);
    init_shutdown_coordinator();
    opens = reads = interrupted = 0;
    clock_offset = 0;
    stage = STALL_OPEN;
    test_ctx = calloc(1, sizeof(*test_ctx));
    TEST_ASSERT_NOT_NULL(test_ctx);
    snprintf(test_ctx->stream_name, sizeof(test_ctx->stream_name), "stalled-detection");
    snprintf(test_ctx->rtsp_url, sizeof(test_ctx->rtsp_url), "rtsp://127.0.0.1/unused");
    atomic_store(&test_ctx->running, 1);
    atomic_store(&test_ctx->detection_stream_thread_running, 1);
    atomic_store(&test_ctx->state, UDT_STATE_CONNECTING);
    pthread_mutex_init(&test_ctx->mutex, NULL);
    pthread_mutex_init(&test_ctx->detection_stream_result_mutex, NULL);
    TEST_ASSERT_EQUAL_INT(0, init_packet_buffer_pool(16));
    test_ctx->packet_buffer = create_packet_buffer(test_ctx->stream_name, 5,
                                                  BUFFER_MODE_MEMORY);
    TEST_ASSERT_NOT_NULL(test_ctx->packet_buffer);
}

void tearDown(void) {
    if (listener >= 0) close(listener);
    listener = -1;
    disconnect_from_stream(test_ctx);
    destroy_packet_buffer(test_ctx->packet_buffer);
    cleanup_packet_buffer_pool();
    pthread_mutex_destroy(&test_ctx->detection_stream_result_mutex);
    pthread_mutex_destroy(&test_ctx->mutex);
    free(test_ctx);
    free(g_config.streams);
    g_config.streams = NULL;
}

static void assert_recovers(stall_stage_t stalled_stage, bool secondary) {
    stage = stalled_stage;
    if (secondary) detection_stream_thread_func(test_ctx);
    else unified_detection_thread_func(test_ctx);
    TEST_ASSERT_EQUAL_INT_MESSAGE(1, interrupted, "blocked I/O must be interrupted");
    TEST_ASSERT_EQUAL_INT_MESSAGE(2, opens, "reader must open a fresh connection");
    TEST_ASSERT_NULL(test_ctx->input_ctx);
    if (secondary) TEST_ASSERT_EQUAL_INT(0, atomic_load(&test_ctx->detection_stream_connected));
}

static void test_main_open_reconnects(void) { assert_recovers(STALL_OPEN, false); }
static void test_main_probe_reconnects(void) { assert_recovers(STALL_PROBE, false); }
static void test_main_read_reconnects(void) { assert_recovers(STALL_READ, false); }
static void test_secondary_open_reconnects(void) { assert_recovers(STALL_OPEN, true); }
static void test_secondary_probe_reconnects(void) { assert_recovers(STALL_PROBE, true); }
static void test_secondary_read_reconnects(void) { assert_recovers(STALL_READ, true); }

static void test_deadlines_are_independent_and_shutdown_still_interrupts(void) {
    test_ctx->input_io_deadline_us = __wrap_av_gettime_relative() - 1;
    test_ctx->detection_io_deadline_us = __wrap_av_gettime_relative() + AV_TIME_BASE;
    TEST_ASSERT_EQUAL_INT(1, ffmpeg_interrupt_callback(test_ctx));
    TEST_ASSERT_EQUAL_INT(0, detection_stream_interrupt_cb(test_ctx));
    test_ctx->input_io_deadline_us = 0;
    TEST_ASSERT_EQUAL_INT(0, ffmpeg_interrupt_callback(test_ctx));
    atomic_store(&test_ctx->detection_stream_thread_running, 0);
    TEST_ASSERT_EQUAL_INT(1, detection_stream_interrupt_cb(test_ctx));
    TEST_ASSERT_EQUAL_INT(0, ffmpeg_interrupt_callback(test_ctx));
    atomic_store(&test_ctx->running, 0);
    TEST_ASSERT_EQUAL_INT(1, ffmpeg_interrupt_callback(test_ctx));
}

static void test_real_silent_rtsp_peer_times_out(void) {
    stage = REAL_IO;
    // TCP connects to the listen backlog, but the peer never answers OPTIONS.
    listener = socket(AF_INET, SOCK_STREAM, 0);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, listener);
    struct sockaddr_in addr = {.sin_family = AF_INET,
                              .sin_addr.s_addr = htonl(INADDR_LOOPBACK)};
    TEST_ASSERT_EQUAL_INT(0, bind(listener, (struct sockaddr *)&addr, sizeof(addr)));
    TEST_ASSERT_EQUAL_INT(0, listen(listener, 1));
    socklen_t len = sizeof(addr);
    TEST_ASSERT_EQUAL_INT(0, getsockname(listener, (struct sockaddr *)&addr, &len));
    snprintf(test_ctx->rtsp_url, sizeof(test_ctx->rtsp_url),
             "rtsp://127.0.0.1:%u/silent", ntohs(addr.sin_port));
    int64_t started = __real_av_gettime_relative();
    TEST_ASSERT_LESS_THAN_INT(0, connect_to_stream(test_ctx));
    int64_t elapsed = __real_av_gettime_relative() - started;
    TEST_ASSERT_GREATER_THAN_INT64(4LL * AV_TIME_BASE, elapsed);
    TEST_ASSERT_LESS_THAN_INT64(8LL * AV_TIME_BASE, elapsed);
    TEST_ASSERT_NULL(test_ctx->input_ctx);
}

int main(void) {
    init_logger();
    set_log_level(LOG_LEVEL_ERROR);
    UNITY_BEGIN();
    RUN_TEST(test_main_open_reconnects);
    RUN_TEST(test_main_probe_reconnects);
    RUN_TEST(test_main_read_reconnects);
    RUN_TEST(test_secondary_open_reconnects);
    RUN_TEST(test_secondary_probe_reconnects);
    RUN_TEST(test_secondary_read_reconnects);
    RUN_TEST(test_deadlines_are_independent_and_shutdown_still_interrupts);
    RUN_TEST(test_real_silent_rtsp_peer_times_out);
    return UNITY_END();
}
