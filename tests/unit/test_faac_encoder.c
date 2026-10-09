#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>

#include "video/faac_encoder.h"

static void test_faac_encoder_lifecycle(void) {
    faac_encoder_config_t config = {
        .sample_rate = 44100,
        .num_channels = 1,
        .input_format = FAAC_INPUT_16BIT,
        .bit_rate_per_channel = 64000,
        .object_type = FAAC_OBJ_AUTO
    };

    faac_encoder_wrapper_t *wrapper = faac_wrapper_open(&config);
    assert(wrapper != NULL);

    uint32_t frame_samples = faac_wrapper_get_frame_samples(wrapper);
    assert(frame_samples > 0);

    uint32_t max_out = faac_wrapper_get_max_output_bytes(wrapper);
    assert(max_out > 0);

    uint32_t delay = faac_wrapper_get_encoder_delay(wrapper);
    printf("FAAC encoder opened: frame_samples=%u, max_output=%u, delay=%u\n",
           frame_samples, max_out, delay);

    const uint8_t *asc = NULL;
    size_t asc_len = 0;
    int asc_res = faac_wrapper_get_asc(wrapper, &asc, &asc_len);
    assert(asc_res == 0);
    assert(asc != NULL);
    assert(asc_len > 0);

    // Encode PCM 16-bit audio samples over multiple frames to test priming and encoding
    size_t pcm_samples = frame_samples * config.num_channels;
    int16_t *pcm_buf = calloc(pcm_samples, sizeof(int16_t));
    assert(pcm_buf != NULL);

    uint8_t *out_buf = malloc(max_out);
    assert(out_buf != NULL);

    uint32_t total_aac_bytes = 0;
    for (int frame = 0; frame < 5; frame++) {
        uint32_t bytes_written = 0;
        int enc_res = faac_wrapper_encode(wrapper, pcm_buf, (uint32_t)pcm_samples,
                                           out_buf, max_out, &bytes_written);
        assert(enc_res == 0);
        total_aac_bytes += bytes_written;
        printf("Frame %d: FAAC encoded %zu PCM samples -> %u AAC bytes\n",
               frame, pcm_samples, bytes_written);
    }

    assert(total_aac_bytes > 0);
    printf("Total AAC bytes produced across 5 frames: %u\n", total_aac_bytes);

    free(pcm_buf);
    free(out_buf);
    faac_wrapper_close(&wrapper);
    assert(wrapper == NULL);

    printf("test_faac_encoder_lifecycle passed!\n");
}

int main(void) {
    printf("Running FAAC encoder unit tests...\n");
    test_faac_encoder_lifecycle();
    printf("All FAAC encoder unit tests passed successfully!\n");
    return 0;
}
