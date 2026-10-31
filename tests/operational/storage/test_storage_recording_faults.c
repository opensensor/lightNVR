#define _POSIX_C_SOURCE 200809L

#include <stdio.h>
#include <errno.h>
#include "telemetry/recording_io_metrics.h"
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include <libavcodec/avcodec.h>
#include <libavutil/imgutils.h>

#include "unity.h"
#include "core/config.h"
#include "video/mp4_segment_recorder.h"
#include "video/mp4_writer.h"
#include "video/mp4_writer_internal.h"

static char test_dir[128];
static char source_path[160];
static char output_path[160];
static AVFormatContext *input;
static segment_info_t segment;
static bool saved_faststart;
static int injected_error, open_error, packet_calls, failed_packets, close_calls;
static AVIOContext *observed_output;

int __real_av_interleaved_write_frame(AVFormatContext *, AVPacket *);
int __wrap_av_interleaved_write_frame(AVFormatContext *ctx, AVPacket *pkt) {
    if (ctx->url && strcmp(ctx->url, output_path) == 0) {
        observed_output = ctx->pb;
        packet_calls++;
        if (injected_error && packet_calls >= 4) {
            failed_packets++;
            return AVERROR(injected_error);
        }
    }
    return __real_av_interleaved_write_frame(ctx, pkt);
}
int __real_avio_open(AVIOContext **, const char *, int);
int __wrap_avio_open(AVIOContext **ctx, const char *url, int flags) {
    if (open_error && strcmp(url, output_path) == 0) return AVERROR(open_error);
    return __real_avio_open(ctx, url, flags);
}
int __real_avio_closep(AVIOContext **);
int __wrap_avio_closep(AVIOContext **ctx) {
    if (*ctx && *ctx == observed_output) { close_calls++; observed_output = NULL; }
    return __real_avio_closep(ctx);
}

void setUp(void) {
    saved_faststart = config_get_mp4_faststart(&g_config);
    config_set_mp4_faststart(&g_config, true);
    strcpy(test_dir, "/tmp/lightnvr-storage-fault-XXXXXX");
    TEST_ASSERT_NOT_NULL(mkdtemp(test_dir));
    snprintf(source_path, sizeof(source_path), "%s/source.mp4", test_dir);
    snprintf(output_path, sizeof(output_path), "%s/segment.mp4", test_dir);
    memset(&segment, 0, sizeof(segment));
    input = NULL;
    injected_error = open_error = packet_calls = failed_packets = close_calls = 0;
    observed_output = NULL;
}

void tearDown(void) {
    config_set_mp4_faststart(&g_config, saved_faststart);
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

/* Four seconds of 32x32 MPEG-4, encoded locally in an owned directory.
 * Real codec, demuxer, segment recorder and muxer; only selected I/O calls fail. */
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
    for (int i = 0; i < 4 * 24; ++i) {
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

static void reopen_input(void) {
    avformat_close_input(&input);
    av_packet_free(&segment.pending_video_keyframe);
    memset(&segment, 0, sizeof(segment));
    TEST_ASSERT_EQUAL_INT(0, avformat_open_input(&input, source_path, NULL, NULL));
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, avformat_find_stream_info(input, NULL));
}

static void assert_recovered_segment_playable(void) {
    injected_error = open_error = 0;
    packet_calls = close_calls = 0;
    reopen_input();
    TEST_ASSERT_EQUAL_INT(0, record_segment(source_path, output_path, 1, 0,
        &input, &segment, NULL, NULL, NULL, NULL));
    TEST_ASSERT_GREATER_THAN_INT(0, packet_calls);
    TEST_ASSERT_EQUAL_INT(1, close_calls);
    AVFormatContext *recorded = NULL;
    TEST_ASSERT_EQUAL_INT(0, avformat_open_input(&recorded, output_path, NULL, NULL));
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, avformat_find_stream_info(recorded, NULL));
    const AVCodec *codec = avcodec_find_decoder(recorded->streams[0]->codecpar->codec_id);
    AVCodecContext *decoder = avcodec_alloc_context3(codec);
    TEST_ASSERT_NOT_NULL(decoder);
    TEST_ASSERT_EQUAL_INT(0, avcodec_parameters_to_context(decoder, recorded->streams[0]->codecpar));
    TEST_ASSERT_EQUAL_INT(0, avcodec_open2(decoder, codec, NULL));
    AVPacket *packet = av_packet_alloc();
    AVFrame *frame = av_frame_alloc();
    TEST_ASSERT_NOT_NULL(packet);
    TEST_ASSERT_NOT_NULL(frame);
    int decoded = 0;
    while (av_read_frame(recorded, packet) >= 0) {
        TEST_ASSERT_EQUAL_INT(0, avcodec_send_packet(decoder, packet));
        while (avcodec_receive_frame(decoder, frame) == 0) decoded++;
        av_packet_unref(packet);
    }
    TEST_ASSERT_GREATER_THAN_INT(0, decoded);
    av_packet_free(&packet);
    av_frame_free(&frame);
    avcodec_free_context(&decoder);
    avformat_close_input(&recorded);
}

static void check_packet_failure(int error) {
    open_fixture(1);
    injected_error = error;
    TEST_ASSERT_EQUAL_INT(AVERROR(error), record_segment(source_path, output_path, 1, 0,
        &input, &segment, NULL, NULL, NULL, NULL));
    TEST_ASSERT_EQUAL_INT(4, packet_calls);
    TEST_ASSERT_EQUAL_INT(1, failed_packets);
    TEST_ASSERT_EQUAL_INT(1, close_calls);
    TEST_ASSERT_NULL(input);
    recording_io_metrics_snapshot_t snapshot;
    recording_io_metrics_snapshot(&snapshot);
    TEST_ASSERT_EQUAL_INT(RECORDING_IO_OPERATION_PACKET, snapshot.last_error.operation);
    TEST_ASSERT_EQUAL_INT(recording_io_reason_from_error(AVERROR(error)), snapshot.last_error.reason);
    assert_recovered_segment_playable();
}
void test_enospc_stops_segment_closes_output_and_recovers(void) { check_packet_failure(ENOSPC); }
void test_eio_stops_segment_closes_output_and_recovers(void) { check_packet_failure(EIO); }
void test_erofs_stops_segment_closes_output_and_recovers(void) { check_packet_failure(EROFS); }
void test_readonly_open_rejected_then_recovers(void) {
    open_fixture(1);
    open_error = EROFS;
    TEST_ASSERT_EQUAL_INT(AVERROR(EROFS), record_segment(source_path, output_path, 1, 0,
        &input, &segment, NULL, NULL, NULL, NULL));
    TEST_ASSERT_EQUAL_INT(0, packet_calls);
    TEST_ASSERT_NULL(input);
    TEST_ASSERT_EQUAL_INT(-1, access(output_path, F_OK));
    assert_recovered_segment_playable();
}
int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_enospc_stops_segment_closes_output_and_recovers);
    RUN_TEST(test_eio_stops_segment_closes_output_and_recovers);
    RUN_TEST(test_erofs_stops_segment_closes_output_and_recovers);
    RUN_TEST(test_readonly_open_rejected_then_recovers);
    return UNITY_END();
}
