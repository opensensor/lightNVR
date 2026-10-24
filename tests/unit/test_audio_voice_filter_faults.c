/**
 * @file test_audio_voice_filter_faults.c
 * @brief Fault injection for the voice-enhancement filter graph (issue #652).
 *
 * Includes the translation unit (as test_detection_current_frame does for the
 * UDT) so each test can see the transcoder slot, and wraps the libavfilter /
 * FIFO calls it makes so every failure path can be forced.  The contract under
 * test: a filter that can't be built or run is logged once and the stream
 * keeps recording unfiltered, without losing or duplicating samples.
 */

#include "../../src/video/mp4_writer_utils.c"

#include <math.h>

#include "unity.h"

#define RATE 16000
#define PACKET 1000
#define PACKETS 32
#define STREAM "fault-test"

enum fault {
    FAULT_NONE,
    FAULT_INOUT_ALLOC,      /* avfilter_inout_alloc returns NULL */
    FAULT_STRDUP,           /* av_strdup returns NULL */
    FAULT_GRAPH_CONFIG,     /* avfilter_graph_config fails */
    FAULT_ADD_FRAME,        /* buffersrc rejects a frame */
    FAULT_ADD_EOF,          /* buffersrc rejects the EOF marker */
    FAULT_SINK,             /* buffersink returns a hard error */
    FAULT_FIFO,             /* the FIFO write after filtering comes up short */
};
static enum fault fault;

AVFilterInOut *__real_avfilter_inout_alloc(void);
AVFilterInOut *__wrap_avfilter_inout_alloc(void) {
    if (fault == FAULT_INOUT_ALLOC) return NULL;
    return __real_avfilter_inout_alloc();
}

char *__real_av_strdup(const char *s);
char *__wrap_av_strdup(const char *s) {
    if (fault == FAULT_STRDUP) return NULL;
    return __real_av_strdup(s);
}

int __real_avfilter_graph_config(AVFilterGraph *graph, void *log_ctx);
int __wrap_avfilter_graph_config(AVFilterGraph *graph, void *log_ctx) {
    if (fault == FAULT_GRAPH_CONFIG) return AVERROR(EINVAL);
    return __real_avfilter_graph_config(graph, log_ctx);
}

int __real_av_buffersrc_add_frame_flags(AVFilterContext *ctx, AVFrame *frame, int flags);
int __wrap_av_buffersrc_add_frame_flags(AVFilterContext *ctx, AVFrame *frame, int flags) {
    if ((fault == FAULT_ADD_FRAME && frame) || (fault == FAULT_ADD_EOF && !frame))
        return AVERROR(EINVAL);
    return __real_av_buffersrc_add_frame_flags(ctx, frame, flags);
}

int __real_av_buffersink_get_frame(AVFilterContext *ctx, AVFrame *frame);
int __wrap_av_buffersink_get_frame(AVFilterContext *ctx, AVFrame *frame) {
    if (fault == FAULT_SINK) return AVERROR(EIO);
    return __real_av_buffersink_get_frame(ctx, frame);
}

int __real_av_audio_fifo_write(AVAudioFifo *af, void *const *data, int nb_samples);
int __wrap_av_audio_fifo_write(AVAudioFifo *af, void *const *data, int nb_samples) {
    if (fault == FAULT_FIFO) return 0;
    return __real_av_audio_fifo_write(af, data, nb_samples);
}

static AVFormatContext *input;
static AVCodecParameters *params;
static AVPacket *pcm, *aac;
static int64_t coded;           /* AAC duration collected so far */
static int fed;                 /* PCM samples pushed */

void setUp(void) {
    g_config.max_streams = MAX_STREAMS;
    fault = FAULT_NONE;
    coded = 0;
    fed = 0;
    input = avformat_alloc_context();
    TEST_ASSERT_NOT_NULL(input);
    AVStream *s = avformat_new_stream(input, NULL);
    TEST_ASSERT_NOT_NULL(s);
    s->time_base = (AVRational){1, RATE};
    s->codecpar->codec_type = AVMEDIA_TYPE_AUDIO;
    s->codecpar->codec_id = AV_CODEC_ID_PCM_S16LE;
    s->codecpar->sample_rate = RATE;
    av_channel_layout_default(&s->codecpar->ch_layout, 1);
    params = NULL;
    TEST_ASSERT_EQUAL_INT(0, transcode_pcm_to_aac(s->codecpar, &s->time_base, STREAM, &params));
    pcm = av_packet_alloc();
    aac = av_packet_alloc();
    TEST_ASSERT_NOT_NULL(pcm);
    TEST_ASSERT_NOT_NULL(aac);
    set_audio_voice_enhancement(STREAM, true);
}

void tearDown(void) {
    fault = FAULT_NONE;
    cleanup_audio_transcoder(STREAM);
    av_packet_free(&pcm);
    av_packet_free(&aac);
    avcodec_parameters_free(&params);
    avformat_free_context(input);
}

static audio_transcoder_t *slot(void) {
    for (int i = 0; i < g_config.max_streams; i++)
        if (audio_transcoders[i].initialized &&
            strcmp(audio_transcoder_stream_names[i], STREAM) == 0)
            return &audio_transcoders[i];
    return NULL;
}

/* Push packets of a 300 Hz tone, collecting every AAC packet produced. */
static void feed(int packets) {
    for (int n = 0; n < packets; n++) {
        av_packet_unref(pcm);
        TEST_ASSERT_EQUAL_INT(0, av_new_packet(pcm, PACKET * 2));
        int16_t *out = (int16_t *)pcm->data;
        for (int i = 0; i < PACKET; i++)
            out[i] = (int16_t)(4000.0 * sin(2.0 * M_PI * 300.0 * (fed + i) / RATE));
        pcm->pts = pcm->dts = fed;
        fed += PACKET;
        TEST_ASSERT_EQUAL_INT(0, transcode_audio_packet(STREAM, pcm, aac, input->streams[0]));
        while (aac->size > 0) {
            coded += aac->duration;
            TEST_ASSERT_EQUAL_INT(0, transcode_audio_packet(STREAM, NULL, aac, input->streams[0]));
        }
    }
}

/* Flush and return the number of samples the AAC stream represents. */
static int64_t finish(void) {
    AVPacket **tail = NULL;
    int count = 0, padding = 0;
    TEST_ASSERT_EQUAL_INT(0, transcode_audio_flush(STREAM, &tail, &count, &padding));
    for (int i = 0; i < count; i++) {
        coded += tail[i]->duration;
        av_packet_free(&tail[i]);
    }
    av_freep(&tail);
    return coded - params->initial_padding - padding;
}

/* Graph setup failures leave the stream recording unfiltered. */
static void check_setup_failure(enum fault which) {
    fault = which;
    feed(1);
    audio_transcoder_t *t = slot();
    TEST_ASSERT_NOT_NULL(t);
    TEST_ASSERT_TRUE(t->filter_failed);
    TEST_ASSERT_NULL(t->filter_graph);
    fault = FAULT_NONE;
    feed(PACKETS - 1);                  /* failure is sticky: no rebuild attempts */
    TEST_ASSERT_NULL(t->filter_graph);
    TEST_ASSERT_EQUAL_INT64(fed, finish());
}

static void test_inout_alloc_failure_records_unfiltered(void) {
    check_setup_failure(FAULT_INOUT_ALLOC);
}

static void test_name_alloc_failure_records_unfiltered(void) {
    check_setup_failure(FAULT_STRDUP);
}

static void test_graph_config_failure_records_unfiltered(void) {
    check_setup_failure(FAULT_GRAPH_CONFIG);
}

/* A fresh opt-in after a failure retries the build. */
static void test_reenabling_retries_after_failure(void) {
    fault = FAULT_GRAPH_CONFIG;
    feed(2);
    TEST_ASSERT_TRUE(slot()->filter_failed);
    fault = FAULT_NONE;
    set_audio_voice_enhancement(STREAM, false);
    set_audio_voice_enhancement(STREAM, true);
    TEST_ASSERT_FALSE(slot()->filter_failed);
    feed(PACKETS - 2);
    TEST_ASSERT_NOT_NULL(slot()->filter_graph);
    TEST_ASSERT_EQUAL_INT64(fed, finish());
}

/* A rejected frame falls back to the plain path for that same frame, so no
 * samples are lost; the samples already inside the graph are the only cost. */
static void test_rejected_frame_falls_back_without_loss(void) {
    feed(4);
    TEST_ASSERT_NOT_NULL(slot()->filter_graph);
    fault = FAULT_ADD_FRAME;
    feed(1);
    fault = FAULT_NONE;
    TEST_ASSERT_TRUE(slot()->filter_failed);
    TEST_ASSERT_NULL(slot()->filter_graph);
    feed(PACKETS - 5);
    int64_t held = fed - finish();
    TEST_ASSERT_GREATER_OR_EQUAL_INT64(0, held);
    TEST_ASSERT_LESS_THAN_INT64(PACKET * 4, held);
}

/* Errors while pulling filtered audio disable the filter but keep going. */
static void check_drain_failure(enum fault which) {
    feed(2);
    TEST_ASSERT_NOT_NULL(slot()->filter_graph);
    fault = which;
    feed(1);
    fault = FAULT_NONE;
    TEST_ASSERT_TRUE(slot()->filter_failed);
    TEST_ASSERT_NULL(slot()->filter_graph);
    int64_t before = fed;
    feed(PACKETS - 3);
    int64_t samples = finish();
    /* Everything after the failure is recorded; only that frame can be lost. */
    TEST_ASSERT_GREATER_OR_EQUAL_INT64(before - 3 * PACKET, samples - (fed - before));
}

static void test_sink_error_disables_filter(void) {
    check_drain_failure(FAULT_SINK);
}

static void test_fifo_shortfall_disables_filter(void) {
    check_drain_failure(FAULT_FIFO);
}

/* Turning enhancement off drains the graph; if that drain fails the filter
 * is torn down anyway and recording continues. */
static void test_disable_with_failed_drain(void) {
    feed(3);
    TEST_ASSERT_NOT_NULL(slot()->filter_graph);
    set_audio_voice_enhancement(STREAM, false);
    fault = FAULT_ADD_EOF;
    feed(1);
    fault = FAULT_NONE;
    TEST_ASSERT_NULL(slot()->filter_graph);
    TEST_ASSERT_TRUE(slot()->filter_failed);
    feed(PACKETS - 4);
    TEST_ASSERT_GREATER_THAN_INT64(fed - 3 * PACKET, finish());
}

/* A failed drain at final flush only costs the filter's tail. */
static void test_flush_with_failed_drain_still_completes(void) {
    feed(PACKETS);
    TEST_ASSERT_NOT_NULL(slot()->filter_graph);
    fault = FAULT_ADD_EOF;
    int64_t samples = finish();
    TEST_ASSERT_NULL(slot()->filter_graph);
    TEST_ASSERT_TRUE(slot()->faac_flushed);
    TEST_ASSERT_GREATER_THAN_INT64(fed - PACKET, samples);
    TEST_ASSERT_LESS_OR_EQUAL_INT64(fed, samples);
}

int main(void) {
    init_logger();
    set_log_level(LOG_LEVEL_ERROR);
    UNITY_BEGIN();
    RUN_TEST(test_inout_alloc_failure_records_unfiltered);
    RUN_TEST(test_name_alloc_failure_records_unfiltered);
    RUN_TEST(test_graph_config_failure_records_unfiltered);
    RUN_TEST(test_reenabling_retries_after_failure);
    RUN_TEST(test_rejected_frame_falls_back_without_loss);
    RUN_TEST(test_sink_error_disables_filter);
    RUN_TEST(test_fifo_shortfall_disables_filter);
    RUN_TEST(test_disable_with_failed_drain);
    RUN_TEST(test_flush_with_failed_drain_still_completes);
    return UNITY_END();
}
