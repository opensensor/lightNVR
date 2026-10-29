/* Exercise the recorder's metadata persistence without starting its RTSP loop. */
#include "../../src/video/mp4_writer_thread.c"
#include "unity.h"

static mp4_writer_thread_t ctx;
static AVStream *video;
static int writes, stored_fps;

int __wrap_update_stream_video_params(const char *name, int width, int height,
                                      int fps, const char *codec) {
    TEST_ASSERT_EQUAL_STRING("fps-recorder", name);
    TEST_ASSERT_EQUAL_INT(640, width);
    TEST_ASSERT_EQUAL_INT(480, height);
    TEST_ASSERT_EQUAL_STRING("h264", codec);
    stored_fps = fps;
    writes++;
    return 0;
}

void setUp(void) {
    memset(&ctx, 0, sizeof(ctx));
    writes = stored_fps = 0;
    ctx.input_ctx = avformat_alloc_context();
    TEST_ASSERT_NOT_NULL(ctx.input_ctx);
    // An audio stream before the video must not supply the video metadata.
    AVStream *audio = avformat_new_stream(ctx.input_ctx, NULL);
    TEST_ASSERT_NOT_NULL(audio);
    audio->codecpar->codec_type = AVMEDIA_TYPE_AUDIO;
    video = avformat_new_stream(ctx.input_ctx, NULL);
    TEST_ASSERT_NOT_NULL(video);
    video->codecpar->codec_type = AVMEDIA_TYPE_VIDEO;
    video->codecpar->codec_id = AV_CODEC_ID_H264;
    video->codecpar->width = 640;
    video->codecpar->height = 480;
    video->time_base = (AVRational){1, 90000};
    TEST_ASSERT_EQUAL_INT(0, metrics_init(2));
}

void tearDown(void) {
    avformat_free_context(ctx.input_ctx);
    metrics_shutdown();
}

static void assert_fps(int expected, bool finalized) {
    update_recording_video_params(&ctx, "fps-recorder");
    TEST_ASSERT_EQUAL_INT(expected, stored_fps);
    TEST_ASSERT_EQUAL_INT(finalized, ctx.video_params_detected);
    stream_metrics_t snapshot[2];
    TEST_ASSERT_EQUAL_INT(1, metrics_snapshot_all(snapshot, 2));
    TEST_ASSERT_EQUAL_INT(expected, (int)snapshot[0].configured_fps);
}

static void test_clock_rate_stays_provisional_until_measured(void) {
    video->avg_frame_rate = (AVRational){90000, 1};
    video->r_frame_rate = (AVRational){90000, 1};
    assert_fps(15, false);
    TEST_ASSERT_EQUAL_INT(1, writes);
    // Later segments complete the arrival measurement and correct DB + health.
    for (int i = 0; i <= 70; i++)
        stream_frame_rate_observe(&ctx.segment_info.fps_measurement, i * 100000LL);
    assert_fps(10, true);
    TEST_ASSERT_EQUAL_INT(2, writes);
    update_recording_video_params(&ctx, "fps-recorder");
    TEST_ASSERT_EQUAL_INT(2, writes);
}

static void test_proxy_nominal_rate_is_refined(void) {
    video->r_frame_rate = (AVRational){6, 1};
    assert_fps(6, false);
    ctx.segment_info.fps_measurement.fps = 10;
    assert_fps(10, true);
}

static void test_valid_average_is_preferred_to_arrival_estimate(void) {
    video->avg_frame_rate = (AVRational){30000, 1001};
    video->r_frame_rate = (AVRational){90000, 1};
    ctx.segment_info.fps_measurement.fps = 10;
    assert_fps(30, true);
}

static void test_missing_input_does_not_write(void) {
    avformat_free_context(ctx.input_ctx);
    ctx.input_ctx = NULL;
    update_recording_video_params(&ctx, "fps-recorder");
    TEST_ASSERT_EQUAL_INT(0, writes);
    TEST_ASSERT_FALSE(ctx.video_params_detected);
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_clock_rate_stays_provisional_until_measured);
    RUN_TEST(test_proxy_nominal_rate_is_refined);
    RUN_TEST(test_valid_average_is_preferred_to_arrival_estimate);
    RUN_TEST(test_missing_input_does_not_write);
    return UNITY_END();
}
