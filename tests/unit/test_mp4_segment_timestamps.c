#include <stdint.h>

#include <libavcodec/packet.h>
#include <libavformat/avformat.h>
#include <libavutil/mem.h>

#include "unity.h"
#include "video/mp4_segment_recorder.h"

void setUp(void) {}
void tearDown(void) {}

void test_rescales_transcoded_stream_timestamps_to_muxer_time_base(void) {
    AVPacket *packet = av_packet_alloc();
    AVStream input_stream = {0};
    AVStream output_stream = {0};

    TEST_ASSERT_NOT_NULL(packet);

    /* Transcoded RTP inputs can expose an equivalent but non-reduced time base.
     * The MP4 muxer normalizes 5/90000 to 1/90000 when writing its header. */
    input_stream.time_base = (AVRational){5, 90000};
    output_stream.time_base = (AVRational){1, 90000};
    packet->dts = 18000 * 300;
    packet->pts = packet->dts + 1800;
    packet->duration = 1800;

    mp4_segment_rescale_packet_ts(packet, &input_stream, &output_stream);

    TEST_ASSERT_EQUAL_INT64(90000LL * 300, packet->dts);
    TEST_ASSERT_EQUAL_INT64((90000LL * 300) + 9000, packet->pts);
    TEST_ASSERT_EQUAL_INT64(9000, packet->duration);

    av_packet_free(&packet);
}

void test_rescale_preserves_unset_timestamps(void) {
    AVPacket *packet = av_packet_alloc();
    AVStream input_stream = {0};
    AVStream output_stream = {0};

    TEST_ASSERT_NOT_NULL(packet);

    input_stream.time_base = (AVRational){1, 18000};
    output_stream.time_base = (AVRational){1, 90000};
    packet->dts = AV_NOPTS_VALUE;
    packet->pts = AV_NOPTS_VALUE;
    packet->duration = 1800;

    mp4_segment_rescale_packet_ts(packet, &input_stream, &output_stream);

    TEST_ASSERT_EQUAL_INT64(AV_NOPTS_VALUE, packet->dts);
    TEST_ASSERT_EQUAL_INT64(AV_NOPTS_VALUE, packet->pts);
    TEST_ASSERT_EQUAL_INT64(9000, packet->duration);

    av_packet_free(&packet);
}

void test_mp4_muxer_time_base_preserves_three_hundred_seconds(void) {
    AVFormatContext *output = NULL;
    AVPacket *packet = NULL;
    uint8_t *output_buffer = NULL;

    TEST_ASSERT_EQUAL_INT(0, avformat_alloc_output_context2(
        &output, NULL, "mp4", NULL));
    TEST_ASSERT_NOT_NULL(output);
    TEST_ASSERT_GREATER_OR_EQUAL(0, avio_open_dyn_buf(&output->pb));

    AVStream *output_stream = avformat_new_stream(output, NULL);
    TEST_ASSERT_NOT_NULL(output_stream);
    output_stream->time_base = (AVRational){5, 90000};
    output_stream->codecpar->codec_type = AVMEDIA_TYPE_VIDEO;
    output_stream->codecpar->codec_id = AV_CODEC_ID_H264;
    output_stream->codecpar->width = 640;
    output_stream->codecpar->height = 480;

    TEST_ASSERT_EQUAL_INT(0, avformat_write_header(output, NULL));
    TEST_ASSERT_EQUAL_INT(1, output_stream->time_base.num);
    TEST_ASSERT_EQUAL_INT(90000, output_stream->time_base.den);

    AVStream input_stream = {0};
    input_stream.time_base = (AVRational){5, 90000};
    packet = av_packet_alloc();
    TEST_ASSERT_NOT_NULL(packet);
    packet->dts = 18000LL * 300;
    packet->pts = packet->dts;
    packet->duration = 9000;

    mp4_segment_rescale_packet_ts(packet, &input_stream, output_stream);

    TEST_ASSERT_INT64_WITHIN(
        1,
        av_rescale_q(300, (AVRational){1, 1}, output_stream->time_base),
        packet->dts);
    TEST_ASSERT_INT64_WITHIN(
        1,
        av_rescale_q(1, (AVRational){1, 2}, output_stream->time_base),
        packet->duration);

    av_packet_free(&packet);
    av_write_trailer(output);
    avio_close_dyn_buf(output->pb, &output_buffer);
    av_free(output_buffer);
    avformat_free_context(output);
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_rescales_transcoded_stream_timestamps_to_muxer_time_base);
    RUN_TEST(test_rescale_preserves_unset_timestamps);
    RUN_TEST(test_mp4_muxer_time_base_preserves_three_hundred_seconds);
    return UNITY_END();
}
