#define _POSIX_C_SOURCE 200809L
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <libavcodec/avcodec.h>
#include "unity.h"
#include "core/config.h"
#include "video/mp4_segment_recorder.h"
#include "video/mp4_writer_internal.h"

static char test_dir[128], source_path[160], output_path[160];
static AVFormatContext *input;
static segment_info_t segment;
static int interrupt_after;
static int interrupt_result;
static int audio_resets;
static bool saved_audio_disabled;

int __real_av_read_frame(AVFormatContext *, AVPacket *);
int __wrap_av_read_frame(AVFormatContext *ctx, AVPacket *packet) {
    // Simulate the exact FFmpeg result produced by stopping a stalled reader.
    // All other demuxing, PCM decoding, AAC encoding and MP4 muxing is real.
    if (interrupt_after == 0) {
        interrupt_after = -1;
        return interrupt_result;
    }
    if (interrupt_after > 0) interrupt_after--;
    return __real_av_read_frame(ctx, packet);
}

void __real_cleanup_audio_transcoder(const char *);
void __wrap_cleanup_audio_transcoder(const char *name) {
    if (strcmp(name, "audio-recovery") == 0) audio_resets++;
    __real_cleanup_audio_transcoder(name);
}

void setUp(void) {
    saved_audio_disabled = g_config.audio_disabled;
    g_config.audio_disabled = false;
    g_config.max_streams = MAX_STREAMS;
    strcpy(test_dir, "/tmp/lightnvr_audio_recovery_XXXXXX");
    TEST_ASSERT_NOT_NULL(mkdtemp(test_dir));
    snprintf(source_path, sizeof(source_path), "%s/source.mkv", test_dir);
    snprintf(output_path, sizeof(output_path), "%s/segment.mp4", test_dir);
    memset(&segment, 0, sizeof(segment));
    strcpy(segment.stream_name, "audio-recovery");
    input = NULL;
    interrupt_after = -1;
    interrupt_result = AVERROR_EXIT;
    audio_resets = 0;
}

void tearDown(void) {
    g_config.audio_disabled = saved_audio_disabled;
    avformat_close_input(&input);
    av_packet_free(&segment.pending_video_keyframe);
    cleanup_audio_transcoder("audio-recovery");
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
static void open_fixture(enum AVCodecID audio_codec) {
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
    encoder->gop_size = 24;
    encoder->max_b_frames = 0;
    encoder->flags |= AV_CODEC_FLAG_GLOBAL_HEADER;
    TEST_ASSERT_EQUAL_INT(0, avcodec_open2(encoder, codec, NULL));

    TEST_ASSERT_EQUAL_INT(0, avformat_alloc_output_context2(
        &output, NULL, "matroska", source_path));
    AVStream *stream = avformat_new_stream(output, NULL);
    TEST_ASSERT_NOT_NULL(stream);
    stream->time_base = encoder->time_base;
    TEST_ASSERT_EQUAL_INT(0, avcodec_parameters_from_context(
        stream->codecpar, encoder));
    AVStream *audio = avformat_new_stream(output, NULL);
    TEST_ASSERT_NOT_NULL(audio);
    audio->time_base = (AVRational){1, 8000};
    audio->codecpar->codec_type = AVMEDIA_TYPE_AUDIO;
    audio->codecpar->codec_id = audio_codec;
    audio->codecpar->sample_rate = 8000;
    audio->codecpar->bits_per_coded_sample = 8;
    av_channel_layout_default(&audio->codecpar->ch_layout, 1);
    TEST_ASSERT_EQUAL_INT(0, avio_open(&output->pb, source_path, AVIO_FLAG_WRITE));
    TEST_ASSERT_EQUAL_INT(0, avformat_write_header(output, NULL));

    frame->format = encoder->pix_fmt;
    frame->width = encoder->width;
    frame->height = encoder->height;
    TEST_ASSERT_EQUAL_INT(0, av_frame_get_buffer(frame, 0));
    for (int i = 0; i < 20 * 24; ++i) {
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
        int start = i * 8000 / 24, end = (i + 1) * 8000 / 24;
        TEST_ASSERT_EQUAL_INT(0, av_new_packet(packet, end - start));
        memset(packet->data, audio_codec == AV_CODEC_ID_PCM_ALAW ? 0xd5 : 0xff, packet->size);
        packet->pts = packet->dts = start;
        packet->duration = end - start;
        packet->stream_index = audio->index;
        av_packet_rescale_ts(packet, (AVRational){1, 8000}, audio->time_base);
        TEST_ASSERT_EQUAL_INT(0, av_interleaved_write_frame(output, packet));
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


static void check_playable_audio(bool new_session) {
    AVFormatContext *recorded = NULL;
    AVPacket *packet = av_packet_alloc();
    AVFrame *frame = av_frame_alloc();
    TEST_ASSERT_EQUAL_INT(0, avformat_open_input(&recorded, output_path, NULL, NULL));
    TEST_ASSERT_GREATER_OR_EQUAL(0, avformat_find_stream_info(recorded, NULL));
    int index = av_find_best_stream(recorded, AVMEDIA_TYPE_AUDIO, -1, -1, NULL, 0);
    TEST_ASSERT_GREATER_OR_EQUAL(0, index);
    TEST_ASSERT_EQUAL_INT(AV_CODEC_ID_AAC, recorded->streams[index]->codecpar->codec_id);
    AVCodecContext *decoder = avcodec_alloc_context3(avcodec_find_decoder(AV_CODEC_ID_AAC));
    TEST_ASSERT_EQUAL_INT(0, avcodec_parameters_to_context(decoder, recorded->streams[index]->codecpar));
    TEST_ASSERT_EQUAL_INT(0, avcodec_open2(decoder, decoder->codec, NULL));
    int decoded_samples = 0, video_packets = 0;
    while (av_read_frame(recorded, packet) >= 0) {
        if (packet->stream_index == index) {
            if (new_session) {
                TEST_ASSERT_LESS_THAN_INT64(0, packet->pts);
                new_session = false;
            }
            TEST_ASSERT_EQUAL_INT(0, avcodec_send_packet(decoder, packet));
            while (avcodec_receive_frame(decoder, frame) >= 0) {
                decoded_samples += frame->nb_samples;
                av_frame_unref(frame);
            }
        } else video_packets++;
        av_packet_unref(packet);
    }
    TEST_ASSERT_GREATER_THAN_INT(0, decoded_samples);
    TEST_ASSERT_GREATER_THAN_INT(0, video_packets);
    avcodec_free_context(&decoder);
    av_frame_free(&frame);
    av_packet_free(&packet);
    avformat_close_input(&recorded);
}

static void check_recovery(enum AVCodecID codec, bool enhance) {
    open_fixture(codec);
    set_audio_voice_enhancement("audio-recovery", enhance);
    for (int n = 0; n < 2; n++) {
        TEST_ASSERT_EQUAL_INT(0, record_segment(source_path, output_path, 2, 1,
            &input, &segment, NULL, NULL, NULL, NULL));
        TEST_ASSERT_NOT_NULL(input);
        check_playable_audio(n == 0);
    }
    // Ordinary rotations retain the encoder FIFO and voice-filter history.
    TEST_ASSERT_EQUAL_INT(0, audio_resets);

    interrupt_after = 80;
    TEST_ASSERT_EQUAL_INT(0, record_segment(source_path, output_path, 10, 1,
        &input, &segment, NULL, NULL, NULL, NULL));
    TEST_ASSERT_EQUAL_INT(-1, interrupt_after);
    TEST_ASSERT_NULL(input);
    TEST_ASSERT_EQUAL_INT(1, audio_resets);
    check_playable_audio(false); // shutdown still flushes the old segment's tail

    // Start another reader using the same writer/stream name after production
    // cleanup closed the input. Before #689's fix this reuses the flushed slot.
    av_packet_free(&segment.pending_video_keyframe);
    if (interrupt_result == AVERROR_EXIT) {
        memset(&segment, 0, sizeof(segment));
        strcpy(segment.stream_name, "audio-recovery");
    } // EOF/input errors retry within the same reader and keep segment_info.
    set_audio_voice_enhancement("audio-recovery", enhance);
    TEST_ASSERT_EQUAL_INT(0, record_segment(source_path, output_path, 2, 1,
        &input, &segment, NULL, NULL, NULL, NULL));
    check_playable_audio(true);
    TEST_ASSERT_EQUAL_INT(1, audio_resets);
}

static void test_alaw_recovers_after_interrupted_session(void) {
    check_recovery(AV_CODEC_ID_PCM_ALAW, false);
}
static void test_mulaw_voice_filter_recovers_after_interrupted_session(void) {
    check_recovery(AV_CODEC_ID_PCM_MULAW, true);
}
static void test_alaw_recovers_after_input_eof(void) {
    interrupt_result = AVERROR_EOF;
    check_recovery(AV_CODEC_ID_PCM_ALAW, false);
}
static void test_alaw_recovers_after_input_error(void) {
    interrupt_result = AVERROR_INVALIDDATA;
    check_recovery(AV_CODEC_ID_PCM_ALAW, false);
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_alaw_recovers_after_interrupted_session);
    RUN_TEST(test_mulaw_voice_filter_recovers_after_interrupted_session);
    RUN_TEST(test_alaw_recovers_after_input_eof);
    RUN_TEST(test_alaw_recovers_after_input_error);
    return UNITY_END();
}
