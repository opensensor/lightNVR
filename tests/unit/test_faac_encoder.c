/**
 * @file test_faac_encoder.c
 * @brief Layer 3 Unity tests for the FAAC adapter.
 */

#include <stdint.h>
#include <stdlib.h>

#include "unity.h"
#include "video/faac_encoder.h"

void setUp(void) {}
void tearDown(void) {}

void test_faac_encoder_flushes_buffered_audio(void) {
    const faac_encoder_config_t config = {
        .sample_rate = 44100,
        .num_channels = 1,
        .bit_rate_per_channel = 64000,
        .object_type = FAAC_OBJ_AUTO
    };
    faac_adapter_t *adapter = faac_adapter_open(&config);
    TEST_ASSERT_NOT_NULL(adapter);

    uint32_t frame_samples = faac_adapter_get_frame_samples(adapter);
    uint32_t max_output = faac_adapter_get_max_output_bytes(adapter);
    TEST_ASSERT_GREATER_THAN_UINT32(0, frame_samples);
    TEST_ASSERT_GREATER_THAN_UINT32(0, max_output);

    const uint8_t *asc = NULL;
    size_t asc_size = 0;
    TEST_ASSERT_EQUAL_INT(0, faac_adapter_get_asc(adapter, &asc, &asc_size));
    TEST_ASSERT_NOT_NULL(asc);
    TEST_ASSERT_GREATER_THAN_UINT(0, asc_size);

    int16_t *pcm = calloc(frame_samples, sizeof(*pcm));
    uint8_t *output = malloc(max_output);
    TEST_ASSERT_NOT_NULL(pcm);
    TEST_ASSERT_NOT_NULL(output);

    uint64_t encoded_bytes = 0;
    for (int frame = 0; frame < 5; frame++) {
        uint32_t bytes_written = 0;
        TEST_ASSERT_EQUAL_INT(0, faac_adapter_encode(
            adapter, pcm, frame_samples, output, max_output, &bytes_written));
        encoded_bytes += bytes_written;
    }
    TEST_ASSERT_GREATER_THAN_UINT64(0, encoded_bytes);

    /* FAAC may return delayed access units over multiple zero-sample calls. */
    uint32_t bytes_written = 0;
    uint64_t flushed_bytes = 0;
    int flush_calls = 0;
    do {
        bytes_written = 0;
        TEST_ASSERT_EQUAL_INT(0, faac_adapter_encode(
            adapter, NULL, 0, output, max_output, &bytes_written));
        flushed_bytes += bytes_written;
        flush_calls++;
    } while (bytes_written > 0 && flush_calls < 16);
    TEST_ASSERT_EQUAL_UINT32(0, bytes_written);
    TEST_ASSERT_LESS_THAN_INT(16, flush_calls);
    TEST_ASSERT_GREATER_THAN_UINT64(0, flushed_bytes);
    TEST_ASSERT_GREATER_THAN_UINT64(0, encoded_bytes);

    free(output);
    free(pcm);
    faac_adapter_close(&adapter);
    TEST_ASSERT_NULL(adapter);
}

void test_faac_he_aac_v1_uses_full_rate_sample_clock(void) {
    const faac_encoder_config_t config = {
        .sample_rate = 32000,
        .num_channels = 1,
        .bit_rate_per_channel = 64000,
        .object_type = FAAC_OBJ_HE_AAC_V1
    };
    faac_adapter_t *adapter = faac_adapter_open(&config);
    TEST_ASSERT_NOT_NULL(adapter);

    TEST_ASSERT_EQUAL_UINT32(32000, faac_adapter_get_sample_rate(adapter));
    TEST_ASSERT_EQUAL_INT(FAAC_OBJ_HE_AAC_V1, faac_adapter_get_object_type(adapter));
    TEST_ASSERT_EQUAL_UINT32(2048, faac_adapter_get_frame_samples(adapter));
    TEST_ASSERT_GREATER_THAN_UINT32(0, faac_adapter_get_encoder_delay(adapter));

    faac_adapter_close(&adapter);
    TEST_ASSERT_NULL(adapter);
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_faac_encoder_flushes_buffered_audio);
    RUN_TEST(test_faac_he_aac_v1_uses_full_rate_sample_clock);
    return UNITY_END();
}
