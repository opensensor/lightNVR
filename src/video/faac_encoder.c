#include "video/faac_encoder.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define LOG_COMPONENT "FAACEncoder"
#include "core/logger.h"

struct faac_encoder_wrapper {
    faac_encoder *enc;
    faac_encoder_info info;
    uint8_t *asc_buf;
    size_t asc_size;
    uint32_t num_channels;
};

faac_encoder_wrapper_t *faac_wrapper_open(const faac_encoder_config_t *config) {
    if (!config || config->sample_rate == 0 || config->num_channels == 0) {
        log_error("faac_wrapper_open: invalid configuration parameters");
        return NULL;
    }

    faac_encoder_wrapper_t *wrapper = calloc(1, sizeof(faac_encoder_wrapper_t));
    if (!wrapper) {
        log_error("faac_wrapper_open: memory allocation failed for wrapper");
        return NULL;
    }

    wrapper->num_channels = config->num_channels;

    faac_params params;
    faac_status status = faac_params_init(&params, sizeof(params));
    if (status != FAAC_OK) {
        log_error("faac_params_init failed: %s", faac_strerror(status));
        free(wrapper);
        return NULL;
    }

    params.sample_rate = config->sample_rate;
    params.num_channels = config->num_channels;
    params.output_format = FAAC_STREAM_RAW; // Raw AAC for MP4 container
    params.input_format = config->input_format != FAAC_INPUT_NULL ? config->input_format : FAAC_INPUT_16BIT;
    params.object_type = config->object_type; // FAAC_OBJ_AUTO by default

    // Set bitrate per channel (matching FFmpeg defaults: 64kbps/ch for >=32kHz, 32kbps/ch otherwise)
    uint32_t br_per_ch = config->bit_rate_per_channel;
    if (br_per_ch == 0) {
        br_per_ch = (config->sample_rate >= 32000) ? 64000 : 32000;
    }
    params.bit_rate = br_per_ch;
    params.rate_control = FAAC_RC_AUTO; // ABR when bit_rate is set

    status = faac_encoder_open(&params, &wrapper->enc);
    if (status != FAAC_OK || !wrapper->enc) {
        log_error("faac_encoder_open failed: %s", faac_strerror(status));
        free(wrapper);
        return NULL;
    }

    // Query encoder info
    wrapper->info.struct_size = sizeof(faac_encoder_info);
    status = faac_encoder_get_info(wrapper->enc, &wrapper->info);
    if (status != FAAC_OK) {
        log_error("faac_encoder_get_info failed: %s", faac_strerror(status));
        faac_encoder_close(&wrapper->enc);
        free(wrapper);
        return NULL;
    }

    // Query AudioSpecificConfig (extradata)
    const uint8_t *asc_ptr = NULL;
    uint32_t asc_len = 0;
    status = faac_encoder_asc(wrapper->enc, &asc_ptr, &asc_len);
    if (status == FAAC_OK && asc_ptr && asc_len > 0) {
        wrapper->asc_buf = malloc(asc_len);
        if (wrapper->asc_buf) {
            memcpy(wrapper->asc_buf, asc_ptr, asc_len);
            wrapper->asc_size = asc_len;
        }
    } else {
        log_warn("faac_encoder_asc did not return extradata: %s", faac_strerror(status));
    }

    log_info("Opened FAAC encoder: %u Hz, %u ch, bitrate %u bps/ch, frame_samples %u, delay %u, obj_type %d",
             wrapper->info.sample_rate, config->num_channels, wrapper->info.bit_rate,
             wrapper->info.frame_samples, wrapper->info.encoder_delay, (int)wrapper->info.object_type);

    return wrapper;
}

int faac_wrapper_get_asc(faac_encoder_wrapper_t *wrapper, const uint8_t **asc_out, size_t *asc_size_out) {
    if (!wrapper || !asc_out || !asc_size_out) {
        return -1;
    }
    *asc_out = wrapper->asc_buf;
    *asc_size_out = wrapper->asc_size;
    return (wrapper->asc_buf && wrapper->asc_size > 0) ? 0 : -1;
}

uint32_t faac_wrapper_get_frame_samples(faac_encoder_wrapper_t *wrapper) {
    return wrapper ? wrapper->info.frame_samples : 1024;
}

uint32_t faac_wrapper_get_max_output_bytes(faac_encoder_wrapper_t *wrapper) {
    return wrapper ? wrapper->info.max_output_bytes : 2048;
}

uint32_t faac_wrapper_get_sample_rate(faac_encoder_wrapper_t *wrapper) {
    return wrapper ? wrapper->info.sample_rate : 0;
}

uint32_t faac_wrapper_get_encoder_delay(faac_encoder_wrapper_t *wrapper) {
    return wrapper ? wrapper->info.encoder_delay : 0;
}

int faac_wrapper_encode(faac_encoder_wrapper_t *wrapper,
                        const void *pcm_data,
                        uint32_t total_samples,
                        uint8_t *out_buf,
                        uint32_t out_cap,
                        uint32_t *bytes_written) {
    if (!wrapper || !wrapper->enc || !bytes_written) {
        return -1;
    }

    faac_status status = faac_encoder_encode(wrapper->enc, pcm_data, total_samples,
                                            out_buf, out_cap, bytes_written);
    if (status != FAAC_OK) {
        log_error("faac_encoder_encode error: %s", faac_strerror(status));
        return -1;
    }

    return 0;
}

void faac_wrapper_close(faac_encoder_wrapper_t **wrapper_ptr) {
    if (!wrapper_ptr || !*wrapper_ptr) {
        return;
    }
    faac_encoder_wrapper_t *wrapper = *wrapper_ptr;
    if (wrapper->enc) {
        faac_encoder_close(&wrapper->enc);
    }
    if (wrapper->asc_buf) {
        free(wrapper->asc_buf);
    }
    free(wrapper);
    *wrapper_ptr = NULL;
}
