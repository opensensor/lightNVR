/* CODEQL-330 verification producer. Real pinned FAAC, no replacement codec.
 * Passing accesses do not prove final unused pointer formation is valid C. */
#include <math.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include "unity.h"
#include "faac.h"

static faac_encoder *enc[2];
static faac_encoder_info info[2];
static void *owned[32];

void setUp(void) {
    memset(enc, 0, sizeof(enc));
    memset(info, 0, sizeof(info));
    memset(owned, 0, sizeof(owned));
}
void tearDown(void) {
    int failures = 0;
    for (unsigned i = 0; i < 2; ++i) {
        if (enc[i] && faac_encoder_close(&enc[i]) != FAAC_OK) ++failures;
        if (enc[i]) ++failures;
    }
    for (unsigned i = 0; i < 32; ++i) { free(owned[i]); owned[i] = NULL; }
    TEST_ASSERT_EQUAL_INT_MESSAGE(0, failures, "all owned encoders must close and null their handle");
}
static void *take(size_t n) {
    for (unsigned i = 0; i < 32; ++i) if (!owned[i]) {
        owned[i] = malloc(n);
        TEST_ASSERT_NOT_NULL(owned[i]);
        return owned[i];
    }
    TEST_FAIL_MESSAGE("fixture allocation registry exhausted");
    return NULL;
}
static void release(void *p) {
    for (unsigned i = 0; i < 32; ++i) if (owned[i] == p) {
        free(p); owned[i] = NULL; return;
    }
    TEST_FAIL_MESSAGE("fixture release must own allocation");
}
static faac_params params(uint32_t channels, enum faac_input_format format) {
    faac_params p;
    TEST_ASSERT_EQUAL_INT(FAAC_OK, faac_params_init(&p, sizeof(p)));
    p.sample_rate = 48000;
    p.num_channels = channels;
    p.object_type = FAAC_OBJ_LOW;
    p.input_format = format;
    p.output_format = FAAC_STREAM_RAW;
    p.bit_rate = 64000;
    p.joint_mode = FAAC_JOINT_NONE;
    return p;
}
static void open_one(unsigned slot, faac_params *p) {
    TEST_ASSERT_EQUAL_INT(FAAC_OK, faac_encoder_open(p, &enc[slot]));
    TEST_ASSERT_NOT_NULL(enc[slot]);
    info[slot].struct_size = sizeof(info[slot]);
    TEST_ASSERT_EQUAL_INT(FAAC_OK, faac_encoder_get_info(enc[slot], &info[slot]));
    TEST_ASSERT_EQUAL_UINT32(1024, info[slot].frame_samples);
    TEST_ASSERT_GREATER_THAN_UINT32(0, info[slot].max_output_bytes);
}
static int16_t sample(uint32_t frame, uint32_t channel) {
    return (int16_t)((int32_t)((frame * 137u + channel * 4093u) % 60001u) - 30000);
}
static void compare_call(const void *a, const void *b, uint32_t n,
                         uint8_t *out0, uint8_t *out1, uint64_t *total) {
    uint32_t bytes0 = UINT32_MAX, bytes1 = UINT32_MAX;
    TEST_ASSERT_EQUAL_INT(FAAC_OK, faac_encoder_encode(enc[0], a, n, out0,
        info[0].max_output_bytes, &bytes0));
    TEST_ASSERT_EQUAL_INT(FAAC_OK, faac_encoder_encode(enc[1], b, n, out1,
        info[1].max_output_bytes, &bytes1));
    TEST_ASSERT_LESS_OR_EQUAL_UINT32(info[0].max_output_bytes, bytes0);
    TEST_ASSERT_EQUAL_UINT32(bytes0, bytes1);
    if (bytes0) TEST_ASSERT_EQUAL_MEMORY(out0, out1, bytes0);
    *total += bytes0;
}
static void layout(uint32_t channels, int reversed, size_t offset) {
    int32_t map[8];
    for (uint32_t c = 0; c < channels; ++c) map[c] = (int32_t)(channels - 1 - c);
    faac_params p0 = params(channels, FAAC_INPUT_16BIT);
    faac_params p1 = params(channels, FAAC_INPUT_32BIT);
    if (reversed) { p0.channel_map = map; p0.channel_map_count = channels; }
    open_one(0, &p0); open_one(1, &p1);
    /* Map is copied at open; later caller mutation must not change selection. */
    memset(map, 0xff, sizeof(map));
    uint8_t *out0 = take(info[0].max_output_bytes);
    uint8_t *out1 = take(info[1].max_output_bytes);
    const uint32_t groups[] = {1, 511, 512, 1024, 1024, 1024};
    uint32_t frame = 0;
    uint64_t total = 0;
    for (unsigned call = 0; call < sizeof(groups)/sizeof(groups[0]); ++call) {
        uint32_t n = groups[call] * channels;
        size_t bytes = (size_t)n * sizeof(int16_t);
        /* Exact suffix allocation: last declared sample abuts ASan redzone. */
        uint8_t *raw = take(bytes + offset);
        int16_t *pcm = (int16_t *)(raw + offset);
        int32_t *wide = take((size_t)n * sizeof(*wide));
        uint8_t *before = take(bytes + offset);
        memset(raw, 0xa5, offset);
        if (offset) {
            TEST_ASSERT_EQUAL_UINT32(0, (uintptr_t)pcm % _Alignof(int16_t));
            TEST_ASSERT_NOT_EQUAL(0, (uintptr_t)pcm % _Alignof(int32_t));
        }
        for (uint32_t i = 0; i < groups[call]; ++i) {
            for (uint32_t c = 0; c < channels; ++c) {
                pcm[i * channels + c] = sample(frame + i, c);
                uint32_t source = reversed ? channels - 1 - c : c;
                wide[i * channels + c] = (int32_t)sample(frame + i, source) * 256;
            }
        }
        memcpy(before, raw, bytes + offset);
        compare_call(pcm, wide, n, out0, out1, &total);
        TEST_ASSERT_EQUAL_MEMORY(before, raw, bytes + offset);
        for (uint32_t i = 0; i < groups[call]; ++i)
            for (uint32_t c = 0; c < channels; ++c)
                TEST_ASSERT_EQUAL_INT32((int32_t)sample(frame + i,
                    reversed ? channels - 1 - c : c) * 256, wide[i * channels + c]);
        frame += groups[call];
        release(before); release(wide); release(raw);
    }
    /* Draining both independent instances must produce equal nonempty audio. */
    int drained = 0;
    for (unsigned i = 0; i < 16; ++i) {
        uint64_t prior = total;
        compare_call(NULL, NULL, 0, out0, out1, &total);
        if (total == prior) { drained = 1; break; }
    }
    TEST_ASSERT_TRUE_MESSAGE(drained, "flush must finish within 16 calls");
    TEST_ASSERT_GREATER_THAN_UINT64(0, total);
    for (unsigned i = 0; i < 2; ++i) {
        TEST_ASSERT_EQUAL_INT(FAAC_OK, faac_encoder_close(&enc[i]));
        TEST_ASSERT_NULL(enc[i]);
    }
}
static void test_mono_packed_16_matches_32(void) { layout(1, 0, 0); }
static void test_stereo_packed_16_matches_32(void) { layout(2, 0, 0); }
static void test_eight_channels_packed_16_matches_32(void) { layout(8, 0, 0); }
static void test_stereo_reverse_map_matches_reordered_input(void) { layout(2, 1, 0); }
static void test_eight_channel_reverse_map_matches_reordered_input(void) { layout(8, 1, 0); }
static void test_stereo_int16_only_alignment(void) { layout(2, 0, sizeof(int16_t)); }
static void test_eight_channel_int16_only_alignment(void) { layout(8, 1, sizeof(int16_t)); }
static void test_invalid_channel_maps_rejected_without_handle(void) {
    faac_params p = params(2, FAAC_INPUT_16BIT);
    int32_t map[2] = {-1, 1};
    p.channel_map = map; p.channel_map_count = 2;
    TEST_ASSERT_EQUAL_INT(FAAC_ERR_INVALID_ARGUMENT, faac_encoder_open(&p, &enc[0]));
    TEST_ASSERT_NULL(enc[0]);
    map[0] = 2;
    TEST_ASSERT_EQUAL_INT(FAAC_ERR_INVALID_ARGUMENT, faac_encoder_open(&p, &enc[0]));
    TEST_ASSERT_NULL(enc[0]);
    map[0] = 0; p.channel_map_count = 1;
    TEST_ASSERT_EQUAL_INT(FAAC_ERR_INVALID_ARGUMENT, faac_encoder_open(&p, &enc[0]));
    TEST_ASSERT_NULL(enc[0]);
}
static void test_rejected_arguments_do_not_consume_input(void) {
    faac_params p = params(2, FAAC_INPUT_16BIT);
    open_one(0, &p); open_one(1, &p);
    uint32_t n = info[0].frame_samples * 2;
    int16_t *pcm = take((size_t)n * sizeof(*pcm));
    for (uint32_t i = 0; i < n; ++i) pcm[i] = sample(i / 2, i % 2);
    uint8_t *out0 = take(info[0].max_output_bytes);
    uint8_t *out1 = take(info[1].max_output_bytes);
    memset(out0, 0x6d, info[0].max_output_bytes);
    uint32_t count = 99;
    TEST_ASSERT_EQUAL_INT(FAAC_ERR_INVALID_ARGUMENT,
        faac_encoder_encode(enc[0], NULL, 2, out0, info[0].max_output_bytes, &count));
    TEST_ASSERT_EQUAL_UINT32(0, count);
    count = 99;
    /* Declared over-limit count must be rejected before touching this buffer. */
    TEST_ASSERT_EQUAL_INT(FAAC_ERR_INPUT_OVERFLOW,
        faac_encoder_encode(enc[0], pcm, n + 2, out0, info[0].max_output_bytes, &count));
    TEST_ASSERT_EQUAL_UINT32(0, count);
    uint8_t *tiny = take(1); tiny[0] = 0x47; count = 99;
    TEST_ASSERT_EQUAL_INT(FAAC_ERR_OUTPUT_TOO_SMALL,
        faac_encoder_encode(enc[0], pcm, n, tiny, 1, &count));
    TEST_ASSERT_EQUAL_UINT32(0, count);
    TEST_ASSERT_EQUAL_HEX8(0x47, tiny[0]);
    TEST_ASSERT_EQUAL_INT(FAAC_ERR_INVALID_ARGUMENT,
        faac_encoder_encode(enc[0], pcm, n, NULL, info[0].max_output_bytes, &count));
    TEST_ASSERT_EQUAL_INT(FAAC_ERR_INVALID_ARGUMENT,
        faac_encoder_encode(enc[0], pcm, n, out0, info[0].max_output_bytes, NULL));
    for (uint32_t i = 0; i < info[0].max_output_bytes; ++i) TEST_ASSERT_EQUAL_HEX8(0x6d, out0[i]);
    uint64_t total = 0;
    for (unsigned i = 0; i < 4; ++i) compare_call(pcm, pcm, n, out0, out1, &total);
    int drained = 0;
    for (unsigned i = 0; i < 16; ++i) {
        uint64_t old = total;
        compare_call(NULL, NULL, 0, out0, out1, &total);
        if (old == total) { drained = 1; break; }
    }
    TEST_ASSERT_TRUE(drained);
    TEST_ASSERT_GREATER_THAN_UINT64(0, total);
    for (uint32_t i = 0; i < n; ++i) TEST_ASSERT_EQUAL_INT16(sample(i / 2, i % 2), pcm[i]);
}
static void test_empty_flush_without_input(void) {
    faac_params p = params(2, FAAC_INPUT_16BIT);
    open_one(0, &p);
    uint8_t *out = take(info[0].max_output_bytes);
    /* Even an empty encoder may emit priming frames. The public contract
     * requires repeated drain until zero, not zero on its first flush. */
    int drained = 0;
    for (unsigned i = 0; i < 16; ++i) {
        uint32_t bytes = UINT32_MAX;
        TEST_ASSERT_EQUAL_INT(FAAC_OK,
            faac_encoder_encode(enc[0], NULL, 0, out, info[0].max_output_bytes, &bytes));
        TEST_ASSERT_LESS_OR_EQUAL_UINT32(info[0].max_output_bytes, bytes);
        if (bytes == 0) { drained = 1; break; }
    }
    TEST_ASSERT_TRUE_MESSAGE(drained, "empty encoder flush must terminate within 16 calls");
}
/* Additional shared-transport format controls for the tracked correction. */
static void additional_format(enum faac_input_format format, uint32_t channels) {
    faac_params a = params(channels, format);
    faac_params b = params(channels, FAAC_INPUT_32BIT);
    open_one(0, &a); open_one(1, &b);
    uint8_t *out0 = take(info[0].max_output_bytes);
    uint8_t *out1 = take(info[1].max_output_bytes);
    const uint32_t chunks[] = {1, 511, 512, 1024, 1024, 1024};
    uint16_t endian = 1;
    int little = *(const uint8_t *)&endian;
    uint32_t frame = 0;
    uint64_t total = 0;
    for (unsigned call = 0; call < sizeof(chunks)/sizeof(chunks[0]); ++call) {
        uint32_t n = chunks[call] * channels;
        size_t width = format == FAAC_INPUT_24BIT ? 3 : sizeof(float);
        size_t offset = format == FAAC_INPUT_24BIT ? 1 : 0;
        size_t size = n * width + offset;
        uint8_t *raw = take(size), *before = take(size);
        int32_t *wide = take(n * sizeof(*wide));
        raw[0] = 0x57;
        for (uint32_t i = 0; i < n; ++i) {
            int32_t value = sample(frame + i / channels, i % channels);
            if (format == FAAC_INPUT_FLOAT) {
                float f = (float)value;
                switch ((frame + i) % 257) {
                    case 0: f = NAN; value = 0; break;
                    case 1: f = INFINITY; value = 0; break;
                    case 2: f = -INFINITY; value = 0; break;
                    case 3: f = 8388608.0f; value = 0; break;
                    case 4: f = -8388608.0f; value = 0; break;
                    default: break;
                }
                ((float *)raw)[i] = f;
            } else {
                uint32_t bits = (uint32_t)(value * 256);
                for (unsigned byte = 0; byte < 3; ++byte)
                    raw[offset + 3 * i + byte] = (uint8_t)(bits >> (8 * (little ? byte : 2 - byte)));
            }
            wide[i] = value * 256;
        }
        memcpy(before, raw, size);
        compare_call(raw + offset, wide, n, out0, out1, &total);
        TEST_ASSERT_EQUAL_MEMORY(before, raw, size);
        frame += chunks[call];
        release(wide); release(before); release(raw);
    }
    int drained = 0;
    for (unsigned i = 0; i < 16; ++i) {
        uint64_t old = total;
        compare_call(NULL, NULL, 0, out0, out1, &total);
        if (old == total) { drained = 1; break; }
    }
    TEST_ASSERT_TRUE(drained);
    TEST_ASSERT_GREATER_THAN_UINT64(0, total);
}
static void test_packed24_stereo_byte_aligned(void) { additional_format(FAAC_INPUT_24BIT, 2); }
static void test_packed24_eight_channels_byte_aligned(void) { additional_format(FAAC_INPUT_24BIT, 8); }
static void test_float_stereo_preserves_screening(void) { additional_format(FAAC_INPUT_FLOAT, 2); }
static void test_float_eight_channels_preserves_screening(void) { additional_format(FAAC_INPUT_FLOAT, 8); }
int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_mono_packed_16_matches_32);
    RUN_TEST(test_stereo_packed_16_matches_32);
    RUN_TEST(test_eight_channels_packed_16_matches_32);
    RUN_TEST(test_stereo_reverse_map_matches_reordered_input);
    RUN_TEST(test_eight_channel_reverse_map_matches_reordered_input);
    RUN_TEST(test_stereo_int16_only_alignment);
    RUN_TEST(test_eight_channel_int16_only_alignment);
    RUN_TEST(test_invalid_channel_maps_rejected_without_handle);
    RUN_TEST(test_rejected_arguments_do_not_consume_input);
    RUN_TEST(test_empty_flush_without_input);
    RUN_TEST(test_packed24_stereo_byte_aligned);
    RUN_TEST(test_packed24_eight_channels_byte_aligned);
    RUN_TEST(test_float_stereo_preserves_screening);
    RUN_TEST(test_float_eight_channels_preserves_screening);
    return UNITY_END();
}
