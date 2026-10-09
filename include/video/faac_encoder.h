#ifndef FAAC_ENCODER_H
#define FAAC_ENCODER_H

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>

#include "faac.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct faac_encoder_wrapper faac_encoder_wrapper_t;

/**
 * Configuration structure for opening a FAAC encoder wrapper instance.
 */
typedef struct faac_encoder_config {
    uint32_t sample_rate;        /**< Input sample rate in Hz */
    uint32_t num_channels;       /**< Number of audio channels */
    enum faac_input_format input_format; /**< Input PCM sample format (16BIT, FLOAT, etc.) */
    uint32_t bit_rate_per_channel; /**< Bitrate in bps per channel (0 for default ~64kbps/ch) */
    enum faac_object_type object_type; /**< FAAC_OBJ_AUTO, FAAC_OBJ_HE_AAC_V1, FAAC_OBJ_LOW */
} faac_encoder_config_t;

/**
 * Open and initialize a FAAC encoder instance.
 *
 * @param config Pointer to encoder configuration.
 * @return Pointer to wrapper instance on success, NULL on failure.
 */
faac_encoder_wrapper_t *faac_wrapper_open(const faac_encoder_config_t *config);

/**
 * Retrieve the AudioSpecificConfig (extradata) produced by FAAC.
 *
 * @param wrapper Encoder instance pointer.
 * @param asc_out Pointer to receive the extradata buffer pointer (owned by wrapper).
 * @param asc_size_out Pointer to receive the extradata length in bytes.
 * @return 0 on success, negative error code on failure.
 */
int faac_wrapper_get_asc(faac_encoder_wrapper_t *wrapper, const uint8_t **asc_out, size_t *asc_size_out);

/**
 * Get frame_samples (number of samples per channel per frame required by encoder).
 */
uint32_t faac_wrapper_get_frame_samples(faac_encoder_wrapper_t *wrapper);

/**
 * Get maximum output bytes per encoded frame.
 */
uint32_t faac_wrapper_get_max_output_bytes(faac_encoder_wrapper_t *wrapper);

/**
 * Get resolved sample rate (e.g. extended rate for HE-AAC).
 */
uint32_t faac_wrapper_get_sample_rate(faac_encoder_wrapper_t *wrapper);

/**
 * Get gapless playback encoder delay (priming delay in samples/channel).
 */
uint32_t faac_wrapper_get_encoder_delay(faac_encoder_wrapper_t *wrapper);

/**
 * Encode interleaved PCM samples.
 *
 * @param wrapper Encoder instance.
 * @param pcm_data Pointer to input PCM buffer (interleaved across channels).
 * @param total_samples Total number of samples in pcm_data (samples_per_channel * num_channels).
 *                      Pass 0 to flush.
 * @param out_buf Output buffer for encoded AAC frame.
 * @param out_cap Capacity of out_buf in bytes.
 * @param bytes_written Pointer to receive actual encoded bytes written to out_buf.
 * @return 0 on success (or when 0 bytes written during buffering/flush), negative code on error.
 */
int faac_wrapper_encode(faac_encoder_wrapper_t *wrapper,
                        const void *pcm_data,
                        uint32_t total_samples,
                        uint8_t *out_buf,
                        uint32_t out_cap,
                        uint32_t *bytes_written);

/**
 * Close and free FAAC encoder instance.
 */
void faac_wrapper_close(faac_encoder_wrapper_t **wrapper_ptr);

#ifdef __cplusplus
}
#endif

#endif /* FAAC_ENCODER_H */
