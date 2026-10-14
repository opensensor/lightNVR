#define _POSIX_C_SOURCE 200809L
#include <string.h>
#include <stdlib.h>
#include <unistd.h>
#include <libavcodec/avcodec.h>
#include "unity.h"
#include "core/config.h"
#include "video/mp4_writer_internal.h"

static AVFormatContext *input;
static AVPacket *pcm, *aac;
static AVCodecParameters *params;
static AVPacket **tail;
static int tail_count;
static mp4_writer_t *writer;
static char path[128];
static bool saved_audio_disabled;
static AVFormatContext *readback;
static AVCodecContext *decoder;
static AVFrame *decoded;

void setUp(void) {
    g_config.max_streams = MAX_STREAMS;
    saved_audio_disabled = g_config.audio_disabled;
    g_config.audio_disabled = false;
    writer = NULL;
    readback = NULL;
    decoder = NULL;
    decoded = NULL;
    path[0] = 0;
    input = avformat_alloc_context();
    TEST_ASSERT_NOT_NULL(input);
    AVStream *s = avformat_new_stream(input, NULL);
    TEST_ASSERT_NOT_NULL(s);
    s->time_base = (AVRational){1, 8000};
    s->codecpar->codec_type = AVMEDIA_TYPE_AUDIO;
    s->codecpar->codec_id = AV_CODEC_ID_PCM_MULAW;
    s->codecpar->sample_rate = 8000;
    av_channel_layout_default(&s->codecpar->ch_layout, 1);
    pcm = av_packet_alloc();
    aac = av_packet_alloc();
    TEST_ASSERT_NOT_NULL(pcm);
    TEST_ASSERT_NOT_NULL(aac);
    params = NULL;
    tail = NULL;
    tail_count = 0;
}

void tearDown(void) {
    avformat_close_input(&readback);
    avcodec_free_context(&decoder);
    av_frame_free(&decoded);
    if (writer) mp4_writer_close(writer);
    if (path[0]) unlink(path);
    g_config.audio_disabled = saved_audio_disabled;
    cleanup_audio_transcoder("audio-test");
    for (int i = 0; i < tail_count; i++) av_packet_free(&tail[i]);
    av_freep(&tail);
    avcodec_parameters_free(&params);
    av_packet_free(&pcm);
    av_packet_free(&aac);
    avformat_free_context(input);
}

static void test_large_pcm_packet_flushes_all_samples(void) {
    AVStream *s = input->streams[0];
    TEST_ASSERT_EQUAL_INT(0, transcode_pcm_to_aac(s->codecpar, &s->time_base,
                                               "audio-test", &params));
    const int samples = params->frame_size * 5 + 137;
    TEST_ASSERT_EQUAL_INT(0, av_new_packet(pcm, samples));
    memset(pcm->data, 0xff, pcm->size);
    pcm->pts = pcm->dts = 0;
    TEST_ASSERT_EQUAL_INT(0, transcode_audio_packet("audio-test", pcm, aac, s));
    int64_t duration = aac->size ? aac->duration : 0;
    int padding = 0;
    TEST_ASSERT_EQUAL_INT(0, transcode_audio_flush("audio-test", &tail, &tail_count, &padding));
    for (int i = 0; i < tail_count; i++) duration += tail[i]->duration;
    TEST_ASSERT_EQUAL_INT64(samples, duration - params->initial_padding - padding);
}

static void check_writer_round_trip(int samples) {
    AVStream *s = input->streams[0];
    TEST_ASSERT_EQUAL_INT(0, transcode_pcm_to_aac(s->codecpar, &s->time_base,
                                               "audio-test", &params));
    strcpy(path, "/tmp/lightnvr-audio-XXXXXX");
    int fd = mkstemp(path);
    TEST_ASSERT_GREATER_OR_EQUAL(0, fd);
    close(fd);
    writer = mp4_writer_create(path, "audio-test");
    TEST_ASSERT_NOT_NULL(writer);
    TEST_ASSERT_EQUAL_INT(0, avformat_alloc_output_context2(&writer->output_ctx, NULL, "mp4", path));
    AVStream *out = avformat_new_stream(writer->output_ctx, NULL);
    TEST_ASSERT_NOT_NULL(out);
    TEST_ASSERT_EQUAL_INT(0, avcodec_parameters_copy(out->codecpar, params));
    out->time_base = (AVRational){1, params->sample_rate};
    writer->audio.stream_idx = out->index;
    TEST_ASSERT_EQUAL_INT(0, avio_open(&writer->output_ctx->pb, path, AVIO_FLAG_WRITE));
    TEST_ASSERT_EQUAL_INT(0, avformat_write_header(writer->output_ctx, NULL));
    writer->is_initialized = 1;

    TEST_ASSERT_EQUAL_INT(0, av_new_packet(pcm, samples));
    memset(pcm->data, 0xff, pcm->size);
    pcm->pts = pcm->dts = 0;
    TEST_ASSERT_EQUAL_INT(0, mp4_writer_write_packet(writer, pcm, s));
    mp4_writer_close(writer);
    writer = NULL;

    TEST_ASSERT_EQUAL_INT(0, avformat_open_input(&readback, path, NULL, NULL));
    TEST_ASSERT_EQUAL_INT(0, avformat_find_stream_info(readback, NULL));
    TEST_ASSERT_EQUAL_UINT(1, readback->nb_streams);
    int64_t duration = av_rescale_q(readback->streams[0]->duration,
        readback->streams[0]->time_base, (AVRational){1, 8000});
    TEST_ASSERT_INT64_WITHIN(8, samples, duration);
    TEST_ASSERT_EQUAL_INT(0, av_read_frame(readback, aac));
    TEST_ASSERT_EQUAL_INT64(-params->initial_padding, aac->pts);
    decoder = avcodec_alloc_context3(avcodec_find_decoder(AV_CODEC_ID_AAC));
    TEST_ASSERT_NOT_NULL(decoder);
    TEST_ASSERT_EQUAL_INT(0, avcodec_parameters_to_context(decoder, readback->streams[0]->codecpar));
    TEST_ASSERT_EQUAL_INT(0, avcodec_open2(decoder, decoder->codec, NULL));
    decoded = av_frame_alloc();
    TEST_ASSERT_NOT_NULL(decoded);
    int decoded_samples = 0;
    do {
        TEST_ASSERT_EQUAL_INT(0, avcodec_send_packet(decoder, aac));
        int ret;
        while ((ret = avcodec_receive_frame(decoder, decoded)) >= 0) {
            decoded_samples += decoded->nb_samples;
            av_frame_unref(decoded);
        }
        TEST_ASSERT_EQUAL_INT(AVERROR(EAGAIN), ret);
        av_packet_unref(aac);
    } while (av_read_frame(readback, aac) >= 0);
    TEST_ASSERT_GREATER_OR_EQUAL(samples, decoded_samples);
}

static void test_writer_close_preserves_short_audio(void) {
    check_writer_round_trip(400);
}

static void test_writer_drains_large_packet_and_preserves_priming(void) {
    check_writer_round_trip(5 * 1024 + 137);
}

static void test_unsupported_channel_count_fails_cleanly(void) {
    av_channel_layout_uninit(&input->streams[0]->codecpar->ch_layout);
    av_channel_layout_default(&input->streams[0]->codecpar->ch_layout, 9);
    TEST_ASSERT_EQUAL_INT(0, av_new_packet(pcm, 180));
    memset(pcm->data, 0xff, pcm->size);
    TEST_ASSERT_LESS_THAN_INT(0, transcode_audio_packet("audio-test", pcm, aac, input->streams[0]));
    TEST_ASSERT_EQUAL_INT(0, aac->size);
}

static void test_pcm_formats_drain_without_losing_samples(void) {
    const struct { enum AVCodecID codec; int rate, channels, bytes; } cases[] = {
        { AV_CODEC_ID_PCM_MULAW, 8000, 1, 1 },
        { AV_CODEC_ID_PCM_ALAW, 16000, 1, 1 },
        { AV_CODEC_ID_PCM_S16LE, 32000, 1, 2 },
        { AV_CODEC_ID_PCM_S16LE, 44100, 2, 2 },
        { AV_CODEC_ID_PCM_S16LE, 48000, 2, 2 },
    };
    for (unsigned c = 0; c < sizeof(cases) / sizeof(cases[0]); c++) {
        AVStream *s = input->streams[0];
        s->codecpar->codec_id = cases[c].codec;
        s->codecpar->sample_rate = cases[c].rate;
        av_channel_layout_uninit(&s->codecpar->ch_layout);
        av_channel_layout_default(&s->codecpar->ch_layout, cases[c].channels);
        s->time_base = (AVRational){1, cases[c].rate};
        TEST_ASSERT_EQUAL_INT(0, transcode_pcm_to_aac(s->codecpar, &s->time_base,
                                                   "audio-test", &params));
        int samples = params->frame_size * 4 + 137;
        int64_t next_pts = -params->initial_padding;
        int64_t duration = 0;
        for (int n = 0; n < 3; n++) {
            av_packet_unref(pcm);
            TEST_ASSERT_EQUAL_INT(0, av_new_packet(pcm, samples * cases[c].channels * cases[c].bytes));
            memset(pcm->data, cases[c].bytes == 1 ? 0xff : 0, pcm->size);
            pcm->pts = pcm->dts = n * samples;
            TEST_ASSERT_EQUAL_INT(0, transcode_audio_packet("audio-test", pcm, aac, s));
            int emitted = 0;
            while (aac->size > 0) {
                TEST_ASSERT_EQUAL_INT64(next_pts, aac->pts);
                TEST_ASSERT_EQUAL_INT(params->frame_size, aac->duration);
                duration += aac->duration;
                next_pts += aac->duration;
                TEST_ASSERT_EQUAL_INT(0, transcode_audio_packet("audio-test", NULL, aac, s));
                TEST_ASSERT_LESS_THAN_INT(10, ++emitted);
            }
            TEST_ASSERT_GREATER_THAN_INT(0, emitted);
        }
        int padding = 0;
        TEST_ASSERT_EQUAL_INT(0, transcode_audio_flush("audio-test", &tail, &tail_count, &padding));
        for (int i = 0; i < tail_count; i++) {
            TEST_ASSERT_EQUAL_INT64(next_pts, tail[i]->pts);
            duration += tail[i]->duration;
            next_pts += tail[i]->duration;
            av_packet_free(&tail[i]);
        }
        av_freep(&tail);
        tail_count = 0;
        TEST_ASSERT_EQUAL_INT64(samples * 3, duration - params->initial_padding - padding);
        TEST_ASSERT_EQUAL_INT(0, transcode_audio_flush("audio-test", &tail, &tail_count, &padding));
        TEST_ASSERT_EQUAL_INT(0, tail_count);
        TEST_ASSERT_EQUAL_INT(AVERROR_EOF, transcode_audio_packet("audio-test", pcm, aac, s));
        cleanup_audio_transcoder("audio-test");
        avcodec_parameters_free(&params);
    }
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_large_pcm_packet_flushes_all_samples);
    RUN_TEST(test_writer_close_preserves_short_audio);
    RUN_TEST(test_unsupported_channel_count_fails_cleanly);
    RUN_TEST(test_writer_drains_large_packet_and_preserves_priming);
    RUN_TEST(test_pcm_formats_drain_without_losing_samples);
    return UNITY_END();
}
