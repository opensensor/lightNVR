/* Exercise the production dispatch and decoder, as in test_detection_io_recovery. */
#include "../../src/video/unified_detection_thread.c"
#include "unity.h"

static unified_detection_ctx_t *ctx;
static int frame_calls, snapshot_calls;
static time_t expected_timestamp;
static unsigned char expected_rgb[3];
static int api_status;

int __wrap_detect_objects_api_snapshot_with_options(const char *url, const char *name,
        detection_result_t *result, float threshold, uint64_t recording_id,
        time_t timestamp, const api_detection_options_t *options) {
    (void)url; (void)name; (void)result; (void)threshold;
    (void)recording_id; (void)timestamp; (void)options;
    ++snapshot_calls;
    return -2;
}

int __wrap_detect_objects_api_with_options(const char *url, const unsigned char *data,
        int width, int height, int channels, detection_result_t *result,
        const char *name, float threshold, uint64_t recording_id,
        time_t timestamp, const api_detection_options_t *options) {
    TEST_ASSERT_EQUAL_STRING("http://127.0.0.1:9001/detect", url);
    TEST_ASSERT_EQUAL_STRING("current-frame", name);
    TEST_ASSERT_EQUAL_INT(64, width);
    TEST_ASSERT_EQUAL_INT(64, height);
    TEST_ASSERT_EQUAL_INT(3, channels);
    TEST_ASSERT_EQUAL_INT64(expected_timestamp, timestamp);
    TEST_ASSERT_EQUAL_UINT64(42, recording_id);
    TEST_ASSERT_FLOAT_WITHIN(0.001f, 0.5f, threshold);
    TEST_ASSERT_EQUAL_INT(API_DETECTION_FORMAT_DOODS2, options->format);
    TEST_ASSERT_NOT_NULL(data);
    for (int i = 0; i < 3; ++i)
        TEST_ASSERT_INT_WITHIN(4, expected_rgb[i], data[i]);
    ++frame_calls;
    memset(result, 0, sizeof(*result));
    if (api_status == 0) {
        result->count = 1;
        result->detections[0].confidence = 0.9f;
        snprintf(result->detections[0].label, sizeof(result->detections[0].label), "person");
    }
    return api_status;
}

int __wrap_filter_detections_by_zones(const char *name, detection_result_t *result) {
    (void)name; (void)result;
    return 0;
}
int __wrap_filter_detections_by_stream_objects(const char *name, detection_result_t *result) {
    (void)name; (void)result;
    return 0;
}

void setUp(void) {
    load_default_config(&g_config);
    snprintf(g_config.api_detection_format, sizeof(g_config.api_detection_format), "doods2");
    ctx = calloc(1, sizeof(*ctx));
    TEST_ASSERT_NOT_NULL(ctx);
    snprintf(ctx->stream_name, sizeof(ctx->stream_name), "current-frame");
    snprintf(ctx->model_path, sizeof(ctx->model_path), "http://127.0.0.1:9001/detect");
    ctx->detection_threshold = 0.5f;
    atomic_store(&ctx->current_recording_id, 42);
    pthread_mutex_init(&ctx->mutex, NULL);
    frame_calls = snapshot_calls = api_status = 0;
}

void tearDown(void) {
    avcodec_free_context(&ctx->decoder_ctx);
    pthread_mutex_destroy(&ctx->mutex);
    free(ctx);
    free(g_config.streams);
    g_config.streams = NULL;
}

static void check_samples(const char *fixture) {
    char path[1024];
    snprintf(path, sizeof(path), "%s/%s", TEST_FIXTURE_DIR, fixture);
    AVFormatContext *input = NULL;
    TEST_ASSERT_EQUAL_INT(0, avformat_open_input(&input, path, NULL, NULL));
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, avformat_find_stream_info(input, NULL));
    int video = av_find_best_stream(input, AVMEDIA_TYPE_VIDEO, -1, -1, NULL, 0);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, video);
    const AVCodecParameters *params = input->streams[video]->codecpar;
    enum AVCodecID codec_id = params->codec_id;
    const AVCodec *decoder = avcodec_find_decoder(codec_id);
    TEST_ASSERT_NOT_NULL(decoder);
    ctx->decoder_ctx = avcodec_alloc_context3(decoder);
    TEST_ASSERT_NOT_NULL(ctx->decoder_ctx);
    TEST_ASSERT_EQUAL_INT(0, avcodec_parameters_to_context(ctx->decoder_ctx, params));
    /* Exercise frame-threading delay as well as the fixture's B-frame reorder delay. */
    ctx->decoder_ctx->thread_count = 2;
    TEST_ASSERT_EQUAL_INT(0, avcodec_open2(ctx->decoder_ctx, decoder, NULL));
    const unsigned char colors[][3] = {{0, 0, 0}, {255, 255, 255}, {255, 0, 0}};
    AVPacket *packet = av_packet_alloc();
    TEST_ASSERT_NOT_NULL(packet);
    int samples = 0, skipped = 0;
    while (av_read_frame(input, packet) >= 0) {
        if (packet->stream_index == video && (packet->flags & AV_PKT_FLAG_KEY)) {
            TEST_ASSERT_LESS_THAN_INT(3, samples);
            memcpy(expected_rgb, colors[samples], 3);
            expected_timestamp = 1700000000 + samples * 10;
            TEST_ASSERT_EQUAL_INT(api_status == 0,
                run_detection_on_frame(ctx, packet, expected_timestamp));
            ++samples;
            TEST_ASSERT_EQUAL_INT(samples, frame_calls);
            TEST_ASSERT_EQUAL_INT_MESSAGE(0, snapshot_calls,
                "packet-driven detection must never fetch a cached snapshot");
        } else {
            ++skipped;
        }
        av_packet_unref(packet);
    }
    av_packet_free(&packet);
    avformat_close_input(&input);
    TEST_ASSERT_EQUAL_INT(3, samples);
    if (codec_id != AV_CODEC_ID_MJPEG)
        TEST_ASSERT_GREATER_THAN_INT(0, skipped);
    TEST_ASSERT_EQUAL_UINT64(api_status == 0 ? 3 : 0, ctx->total_detections);
}

static void test_h264_samples_current_keyframes(void) { check_samples("keyframes.h264"); }
static void test_h265_samples_current_keyframes(void) { check_samples("keyframes.h265"); }
static void test_mjpeg_samples_current_frames(void) { check_samples("keyframes.mjpeg"); }
static void test_api_failure_does_not_retry_snapshot(void) {
    api_status = -1;
    check_samples("keyframes.h264");
}
static void test_missing_packet_does_not_fetch_snapshot(void) {
    TEST_ASSERT_FALSE(run_detection_on_frame(ctx, NULL, 1700000000));
    TEST_ASSERT_EQUAL_INT(0, frame_calls);
    TEST_ASSERT_EQUAL_INT(0, snapshot_calls);
}

int main(void) {
    init_logger();
    set_log_level(LOG_LEVEL_ERROR);
    UNITY_BEGIN();
    RUN_TEST(test_h264_samples_current_keyframes);
    RUN_TEST(test_h265_samples_current_keyframes);
    RUN_TEST(test_mjpeg_samples_current_frames);
    RUN_TEST(test_api_failure_does_not_retry_snapshot);
    RUN_TEST(test_missing_packet_does_not_fetch_snapshot);
    return UNITY_END();
}
