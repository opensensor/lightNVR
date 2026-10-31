/* CODEQL-329: exercise the production metadata assignment, with explicit
 * seams only for channel counts the real decoder/encoder cannot accept. */
#include <limits.h>
#include <stdint.h>
#include <libavcodec/avcodec.h>
#include "unity.h"
#include "video/faac_encoder.h"
#include "video/mp4_writer_internal.h"

static bool synthetic;
static int synthetic_channels;
static faac_encoder_config_t captured;
static unsigned char fake_encoder;
static int encoder_opens;
static AVCodecParameters *input, *output;

int __real_avcodec_open2(AVCodecContext *, const AVCodec *, AVDictionary **);
int __wrap_avcodec_open2(AVCodecContext *ctx, const AVCodec *codec, AVDictionary **opts) {
    int ret = __real_avcodec_open2(ctx, codec, opts);
    if (ret >= 0 && synthetic) {
        av_channel_layout_uninit(&ctx->ch_layout);
        ctx->ch_layout.order = AV_CHANNEL_ORDER_UNSPEC;
        ctx->ch_layout.nb_channels = synthetic_channels;
    }
    return ret;
}
faac_adapter_t *__real_faac_adapter_open(const faac_encoder_config_t *);
faac_adapter_t *__wrap_faac_adapter_open(const faac_encoder_config_t *cfg) {
    encoder_opens++;
    captured = *cfg;
    return synthetic ? (faac_adapter_t *)&fake_encoder : __real_faac_adapter_open(cfg);
}
#define WRAP_GETTER(type, name, value) \
    type __real_##name(faac_adapter_t *); \
    type __wrap_##name(faac_adapter_t *a) { \
        return synthetic ? (value) : __real_##name(a); \
    }
WRAP_GETTER(uint32_t, faac_adapter_get_sample_rate, captured.sample_rate)
WRAP_GETTER(uint32_t, faac_adapter_get_frame_samples, 1024)
WRAP_GETTER(uint32_t, faac_adapter_get_encoder_delay, 0)
WRAP_GETTER(enum faac_object_type, faac_adapter_get_object_type, FAAC_OBJ_LOW)
int __real_faac_adapter_get_asc(faac_adapter_t *, const uint8_t **, size_t *);
int __wrap_faac_adapter_get_asc(faac_adapter_t *a, const uint8_t **asc, size_t *size) {
    static const uint8_t fixture[] = {0x11, 0x90};
    if (!synthetic) return __real_faac_adapter_get_asc(a, asc, size);
    *asc = fixture;
    *size = sizeof(fixture);
    return 0;
}
void __real_faac_adapter_close(faac_adapter_t **);
void __wrap_faac_adapter_close(faac_adapter_t **a) {
    if (synthetic) *a = NULL;
    else __real_faac_adapter_close(a);
}

void setUp(void) {
    synthetic = false;
    encoder_opens = 0;
    output = NULL;
    input = avcodec_parameters_alloc();
    TEST_ASSERT_NOT_NULL(input);
    input->codec_type = AVMEDIA_TYPE_AUDIO;
    input->codec_id = AV_CODEC_ID_PCM_S16LE;
    input->format = AV_SAMPLE_FMT_S16;
    input->sample_rate = 48000;
    av_channel_layout_default(&input->ch_layout, 1);
}
void tearDown(void) {
    avcodec_parameters_free(&input);
    avcodec_parameters_free(&output);
}
static int probe(void) {
    AVRational tb = {1, 48000};
    return transcode_pcm_to_aac(input, &tb, "t7-16", &output);
}
static void expect_synthetic(int channels, int64_t expected) {
    synthetic = true;
    synthetic_channels = channels;
    TEST_ASSERT_EQUAL_INT(0, probe());
    TEST_ASSERT_NOT_NULL(output);
    TEST_ASSERT_EQUAL_INT64(expected, output->bit_rate);
    TEST_ASSERT_EQUAL_INT(1, encoder_opens);
}
static void test_unsigned_boundary_before_wrap(void) {
    expect_synthetic(UINT32_MAX / 64000, INT64_C(4294912000));
}
static void test_unsigned_boundary_after_wrap(void) {
    expect_synthetic(UINT32_MAX / 64000 + 1, INT64_C(4294976000));
}
static void test_maximum_signed_channel_count(void) {
    expect_synthetic(INT_MAX, INT64_C(137438953408000));
}
static void test_signed_boundary_is_not_sign_extended(void) {
    expect_synthetic(INT32_MAX / 64000 + 1, INT64_C(2147520000));
}
static void test_low_rate_multiplication_also_widens(void) {
    input->sample_rate = 8000;
    expect_synthetic(UINT32_MAX / 32000 + 1, INT64_C(4294976000));
}
static void test_synthetic_zero_channels_preserve_mono_fallback(void) {
    expect_synthetic(0, 64000);
    TEST_ASSERT_EQUAL_UINT32(1, captured.num_channels);
}
static void test_synthetic_negative_channels_preserve_mono_fallback(void) {
    expect_synthetic(INT_MIN, 64000);
    TEST_ASSERT_EQUAL_UINT32(1, captured.num_channels);
}
static void real_valid(int channels, int rate, int64_t bitrate) {
    av_channel_layout_uninit(&input->ch_layout);
    av_channel_layout_default(&input->ch_layout, channels);
    input->sample_rate = rate;
    TEST_ASSERT_EQUAL_INT(0, probe());
    TEST_ASSERT_EQUAL_INT64(bitrate, output->bit_rate);
    TEST_ASSERT_EQUAL_INT(channels, output->ch_layout.nb_channels);
    TEST_ASSERT_EQUAL_INT(AV_CODEC_ID_AAC, output->codec_id);
    TEST_ASSERT_TRUE(output->extradata_size > 0);
}
static void test_real_mono_low_rate(void) { real_valid(1, 8000, 32000); }
static void test_real_stereo(void) { real_valid(2, 48000, 128000); }
static void test_real_maximum_supported_channels(void) { real_valid(8, 48000, 512000); }
static void invalid_channels(int channels) {
    av_channel_layout_uninit(&input->ch_layout);
    input->ch_layout.order = AV_CHANNEL_ORDER_UNSPEC;
    input->ch_layout.nb_channels = channels;
    TEST_ASSERT_LESS_THAN_INT(0, probe());
    TEST_ASSERT_NULL(output);
}
static void test_real_zero_channels_rejected(void) { invalid_channels(0); }
static void test_real_negative_channels_rejected(void) { invalid_channels(-1); }
static void test_real_unsupported_channels_rejected(void) { invalid_channels(9); }
static void test_real_huge_channels_rejected(void) { invalid_channels(INT_MAX); }
static void test_real_zero_rate_rejected(void) {
    input->sample_rate = 0;
    TEST_ASSERT_LESS_THAN_INT(0, probe());
    TEST_ASSERT_NULL(output);
}
static void test_real_negative_rate_rejected(void) {
    input->sample_rate = -1;
    TEST_ASSERT_LESS_THAN_INT(0, probe());
    TEST_ASSERT_NULL(output);
}
int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_unsigned_boundary_before_wrap);
    RUN_TEST(test_unsigned_boundary_after_wrap);
    RUN_TEST(test_maximum_signed_channel_count);
    RUN_TEST(test_signed_boundary_is_not_sign_extended);
    RUN_TEST(test_low_rate_multiplication_also_widens);
    RUN_TEST(test_synthetic_zero_channels_preserve_mono_fallback);
    RUN_TEST(test_synthetic_negative_channels_preserve_mono_fallback);
    RUN_TEST(test_real_mono_low_rate);
    RUN_TEST(test_real_stereo);
    RUN_TEST(test_real_maximum_supported_channels);
    RUN_TEST(test_real_zero_channels_rejected);
    RUN_TEST(test_real_negative_channels_rejected);
    RUN_TEST(test_real_unsupported_channels_rejected);
    RUN_TEST(test_real_huge_channels_rejected);
    RUN_TEST(test_real_zero_rate_rejected);
    RUN_TEST(test_real_negative_rate_rejected);
    return UNITY_END();
}
