#define _POSIX_C_SOURCE 200809L

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include <libavformat/avformat.h>
#include <libavutil/intreadwrite.h>

#include "unity.h"
#include "core/config.h"
#include "core/logger.h"
#include "video/ffmpeg_utils.h"
#include "video/packet_buffer.h"
#include "video/pre_detection_buffer.h"

static char test_dir[128], ts_path[160], output_path[160];
static bool saved_faststart;
static bool capture_header, header_called, header_faststart;

int __real_avformat_write_header(AVFormatContext *context, AVDictionary **options);
int __wrap_avformat_write_header(AVFormatContext *context, AVDictionary **options) {
    if (!capture_header) return __real_avformat_write_header(context, options);
    header_called = true;
    const AVDictionaryEntry *flags = av_dict_get(
        options ? *options : NULL, "movflags", NULL, 0);
    header_faststart = flags && strstr(flags->value, "faststart");
    // The memory strategy's codec parameters are still a placeholder. Stop
    // at the FFmpeg boundary so this test isolates its muxer option contract.
    return AVERROR(EINVAL);
}

void setUp(void) {
    saved_faststart = config_get_mp4_faststart(&g_config);
    capture_header = header_called = header_faststart = false;
    strcpy(test_dir, "/tmp/lightnvr_mux_faststart_XXXXXX");
    TEST_ASSERT_NOT_NULL(mkdtemp(test_dir));
    snprintf(ts_path, sizeof(ts_path), "%s/segment.ts", test_dir);
    snprintf(output_path, sizeof(output_path), "%s/recording.mp4", test_dir);
}

void tearDown(void) {
    config_set_mp4_faststart(&g_config, saved_faststart);
    capture_header = false;
    cleanup_packet_buffer_pool();
    unlink(ts_path);
    unlink(output_path);
    rmdir(test_dir);
}

static void make_ts_fixture(void) {
    AVFormatContext *input = NULL, *output = NULL;
    TEST_ASSERT_EQUAL_INT(0, avformat_open_input(&input, RECORDING_FIXTURE, NULL, NULL));
    TEST_ASSERT_GREATER_OR_EQUAL(0, avformat_find_stream_info(input, NULL));
    TEST_ASSERT_EQUAL_INT(0, avformat_alloc_output_context2(&output, NULL, "mpegts", ts_path));
    AVStream *stream = avformat_new_stream(output, NULL);
    TEST_ASSERT_NOT_NULL(stream);
    TEST_ASSERT_EQUAL_INT(0, avcodec_parameters_copy(stream->codecpar, input->streams[0]->codecpar));
    stream->codecpar->codec_tag = 0;
    stream->time_base = input->streams[0]->time_base;
    TEST_ASSERT_EQUAL_INT(0, avio_open(&output->pb, ts_path, AVIO_FLAG_WRITE));
    TEST_ASSERT_EQUAL_INT(0, avformat_write_header(output, NULL));
    AVPacket *packet = av_packet_alloc();
    TEST_ASSERT_NOT_NULL(packet);
    TEST_ASSERT_EQUAL_INT(0, av_read_frame(input, packet));
    TEST_ASSERT_TRUE(packet->flags & AV_PKT_FLAG_KEY);
    // Repeat one encoded keyframe with increasing timestamps. This produces
    // a decodable TS fixture without depending on B-frame timestamp handling.
    for (int i = 0; i < 10; ++i) {
        AVPacket *copy = av_packet_clone(packet);
        TEST_ASSERT_NOT_NULL(copy);
        copy->pts = copy->dts = av_rescale_q(i, (AVRational){1, 1}, stream->time_base);
        copy->duration = av_rescale_q(1, (AVRational){1, 1}, stream->time_base);
        copy->pos = -1;
        TEST_ASSERT_EQUAL_INT(0, av_interleaved_write_frame(output, copy));
        av_packet_free(&copy);
    }
    TEST_ASSERT_EQUAL_INT(0, av_write_trailer(output));
    avio_closep(&output->pb);
    avformat_free_context(output);
    avformat_close_input(&input);
    av_packet_free(&packet);
}

static void check_layout_and_packets(bool faststart) {
    FILE *file = fopen(output_path, "rb");
    TEST_ASSERT_NOT_NULL(file);
    long moov = -1, mdat = -1;
    uint8_t header[8];
    while (fread(header, 1, sizeof(header), file) == sizeof(header)) {
        long offset = ftell(file) - 8;
        uint32_t size = AV_RB32(header);
        if (!memcmp(header + 4, "moov", 4)) moov = offset;
        if (!memcmp(header + 4, "mdat", 4)) mdat = offset;
        if (size < 8 || fseek(file, size - 8, SEEK_CUR) != 0) break;
    }
    fclose(file);
    TEST_ASSERT_GREATER_OR_EQUAL(0, moov);
    TEST_ASSERT_GREATER_OR_EQUAL(0, mdat);
    TEST_ASSERT_EQUAL(faststart, moov < mdat);

    AVFormatContext *input = NULL;
    TEST_ASSERT_EQUAL_INT(0, avformat_open_input(&input, output_path, NULL, NULL));
    AVPacket *packet = av_packet_alloc();
    TEST_ASSERT_NOT_NULL(packet);
    int packets = 0;
    while (av_read_frame(input, packet) >= 0) {
        ++packets;
        av_packet_unref(packet);
    }
    TEST_ASSERT_EQUAL_INT(10, packets);
    av_packet_free(&packet);
    avformat_close_input(&input);
}

static void check_hls_faststart(bool enabled) {
    make_ts_fixture();
    config_set_mp4_faststart(&g_config, enabled);
    const char *paths[] = {ts_path};
    TEST_ASSERT_EQUAL_INT(0, ffmpeg_concat_ts_to_mp4(paths, 1, output_path));
    check_layout_and_packets(enabled);
}

void test_hls_conversion_faststart_enabled(void) { check_hls_faststart(true); }
void test_hls_conversion_faststart_disabled(void) { check_hls_faststart(false); }

static void check_memory_faststart(bool enabled) {
    const buffer_config_t config = {
        .buffer_seconds = 10, .memory_limit_bytes = 2 * 1024 * 1024
    };
    pre_buffer_strategy_t *strategy = create_buffer_strategy(
        BUFFER_STRATEGY_MEMORY_PACKET, "faststart-test", &config);
    TEST_ASSERT_NOT_NULL(strategy);
    AVPacket *packet = av_packet_alloc();
    TEST_ASSERT_NOT_NULL(packet);
    TEST_ASSERT_EQUAL_INT(0, av_new_packet(packet, 8));
    memset(packet->data, 0, packet->size);
    packet->flags = AV_PKT_FLAG_KEY;
    packet->pts = packet->dts = 0;
    TEST_ASSERT_EQUAL_INT(0, strategy->add_packet(strategy, packet, time(NULL)));
    av_packet_free(&packet);

    config_set_mp4_faststart(&g_config, enabled);
    capture_header = true;
    int ret = strategy->flush_to_file(strategy, output_path);
    capture_header = false;
    destroy_buffer_strategy(strategy);
    TEST_ASSERT_EQUAL_INT(-1, ret);
    TEST_ASSERT_TRUE(header_called);
    TEST_ASSERT_EQUAL(enabled, header_faststart);
}

void test_memory_prebuffer_faststart_enabled(void) { check_memory_faststart(true); }
void test_memory_prebuffer_faststart_disabled(void) { check_memory_faststart(false); }

int main(void) {
    init_logger();
    UNITY_BEGIN();
    RUN_TEST(test_hls_conversion_faststart_enabled);
    RUN_TEST(test_hls_conversion_faststart_disabled);
    RUN_TEST(test_memory_prebuffer_faststart_enabled);
    RUN_TEST(test_memory_prebuffer_faststart_disabled);
    return UNITY_END();
}
