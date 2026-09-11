#define _POSIX_C_SOURCE 200809L

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include <libavcodec/avcodec.h>
#include <libavutil/imgutils.h>

#include "unity.h"
#include "video/mp4_segment_recorder.h"

static char test_dir[128];
static char source_path[160];
static char output_path[160];
static AVFormatContext *input;
static segment_info_t segment;

void setUp(void) {
    strcpy(test_dir, "/tmp/lightnvr_segment_boundaries_XXXXXX");
    TEST_ASSERT_NOT_NULL(mkdtemp(test_dir));
    snprintf(source_path, sizeof(source_path), "%s/source.mp4", test_dir);
    snprintf(output_path, sizeof(output_path), "%s/segment.mp4", test_dir);
    memset(&segment, 0, sizeof(segment));
    input = NULL;
}

void tearDown(void) {
    avformat_close_input(&input);
    av_packet_free(&segment.pending_video_keyframe);
    unlink(source_path);
    unlink(output_path);
    rmdir(test_dir);
}

static void write_encoded_packets(AVCodecContext *encoder,
                                  AVFormatContext *output, AVPacket *packet) {
    int ret;
    while ((ret = avcodec_receive_packet(encoder, packet)) >= 0) {
        av_packet_rescale_ts(packet, encoder->time_base,
                            output->streams[0]->time_base);
        packet->stream_index = 0;
        TEST_ASSERT_EQUAL_INT(0, av_interleaved_write_frame(output, packet));
    }
    TEST_ASSERT_TRUE(ret == AVERROR(EAGAIN) || ret == AVERROR_EOF);
}

/* Use the native MPEG-4 encoder to exercise the real demux/record/mux path
 * without an external ffmpeg executable or camera. Boundary handling is shared
 * by codecs; the reported HEVC camera has this same 24 FPS / 10 second GOP. */
static void open_fixture(int keyframe_interval_seconds) {
    const AVCodec *codec = avcodec_find_encoder(AV_CODEC_ID_MPEG4);
    TEST_ASSERT_NOT_NULL(codec);
    AVCodecContext *encoder = avcodec_alloc_context3(codec);
    AVFormatContext *output = NULL;
    AVFrame *frame = av_frame_alloc();
    AVPacket *packet = av_packet_alloc();
    TEST_ASSERT_NOT_NULL(encoder);
    TEST_ASSERT_NOT_NULL(frame);
    TEST_ASSERT_NOT_NULL(packet);

    encoder->width = 32;
    encoder->height = 32;
    encoder->pix_fmt = AV_PIX_FMT_YUV420P;
    encoder->time_base = (AVRational){1, 24};
    encoder->gop_size = keyframe_interval_seconds * 24;
    encoder->max_b_frames = 0;
    encoder->flags |= AV_CODEC_FLAG_GLOBAL_HEADER;
    TEST_ASSERT_EQUAL_INT(0, avcodec_open2(encoder, codec, NULL));

    TEST_ASSERT_EQUAL_INT(0, avformat_alloc_output_context2(
        &output, NULL, "mp4", source_path));
    AVStream *stream = avformat_new_stream(output, NULL);
    TEST_ASSERT_NOT_NULL(stream);
    stream->time_base = encoder->time_base;
    TEST_ASSERT_EQUAL_INT(0, avcodec_parameters_from_context(
        stream->codecpar, encoder));
    TEST_ASSERT_EQUAL_INT(0, avio_open(&output->pb, source_path, AVIO_FLAG_WRITE));
    TEST_ASSERT_EQUAL_INT(0, avformat_write_header(output, NULL));

    frame->format = encoder->pix_fmt;
    frame->width = encoder->width;
    frame->height = encoder->height;
    TEST_ASSERT_EQUAL_INT(0, av_frame_get_buffer(frame, 0));
    for (int i = 0; i < 90 * 24; ++i) {
        TEST_ASSERT_EQUAL_INT(0, av_frame_make_writable(frame));
        for (int y = 0; y < frame->height; ++y)
            memset(frame->data[0] + y * frame->linesize[0], 80, frame->width);
        for (int plane = 1; plane < 3; ++plane)
            for (int y = 0; y < frame->height / 2; ++y)
                memset(frame->data[plane] + y * frame->linesize[plane],
                       128, frame->width / 2);
        frame->pts = i;
        frame->pict_type = i % encoder->gop_size == 0
            ? AV_PICTURE_TYPE_I : AV_PICTURE_TYPE_P;
        TEST_ASSERT_EQUAL_INT(0, avcodec_send_frame(encoder, frame));
        write_encoded_packets(encoder, output, packet);
    }
    TEST_ASSERT_EQUAL_INT(0, avcodec_send_frame(encoder, NULL));
    write_encoded_packets(encoder, output, packet);
    TEST_ASSERT_EQUAL_INT(0, av_write_trailer(output));
    avio_closep(&output->pb);
    avformat_free_context(output);
    avcodec_free_context(&encoder);
    av_frame_free(&frame);
    av_packet_free(&packet);

    TEST_ASSERT_EQUAL_INT(0, avformat_open_input(&input, source_path, NULL, NULL));
    TEST_ASSERT_GREATER_OR_EQUAL(0, avformat_find_stream_info(input, NULL));
}

static void record_and_check_boundary(int expected_camera_seconds,
                                      int expected_segment_seconds) {
    TEST_ASSERT_EQUAL_INT(0, record_segment(
        source_path, output_path, 30, 0, &input, &segment,
        NULL, NULL, NULL, NULL));
    TEST_ASSERT_NOT_NULL(input);
    TEST_ASSERT_TRUE(segment.last_frame_was_key);
    TEST_ASSERT_NOT_NULL(segment.pending_video_keyframe);
    TEST_ASSERT_EQUAL_INT64(av_rescale_q(
        expected_camera_seconds, (AVRational){1, 1}, input->streams[0]->time_base),
        segment.pending_video_keyframe->dts);

    AVFormatContext *recorded = NULL;
    AVPacket *packet = av_packet_alloc();
    TEST_ASSERT_NOT_NULL(packet);
    TEST_ASSERT_EQUAL_INT(0, avformat_open_input(
        &recorded, output_path, NULL, NULL));
    int video_packets = 0;
    while (av_read_frame(recorded, packet) >= 0) {
        if (packet->stream_index == 0) {
            if (video_packets == 0)
                TEST_ASSERT_TRUE(packet->flags & AV_PKT_FLAG_KEY);
            ++video_packets;
        }
        av_packet_unref(packet);
    }
    /* Both files include the boundary keyframe; no intermediate frames vanish. */
    TEST_ASSERT_EQUAL_INT(expected_segment_seconds * 24 + 1, video_packets);
    av_packet_free(&packet);
    avformat_close_input(&recorded);
}

void test_keyframe_at_duration_closes_current_segment_and_starts_next(void) {
    open_fixture(10);
    record_and_check_boundary(30, 30);
    record_and_check_boundary(60, 30);
}

void test_non_keyframe_at_duration_waits_for_next_keyframe(void) {
    open_fixture(8);
    record_and_check_boundary(32, 32);
    record_and_check_boundary(64, 32);
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_keyframe_at_duration_closes_current_segment_and_starts_next);
    RUN_TEST(test_non_keyframe_at_duration_waits_for_next_keyframe);
    return UNITY_END();
}
