#include "video/faac_encoder.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define LOG_COMPONENT "FAACEncoder"
#include "core/logger.h"

struct faac_adapter {
    faac_encoder *enc;
    faac_encoder_info info;
    uint8_t *asc_buf;
    size_t asc_size;
    uint32_t num_channels;
};

faac_adapter_t *faac_adapter_open(const faac_encoder_config_t *config) {
    if (!config || config->sample_rate == 0 || config->num_channels == 0) {
        log_error("faac_adapter_open: invalid configuration parameters");
        return NULL;
    }

    faac_adapter_t *adapter = calloc(1, sizeof(faac_adapter_t));
    if (!adapter) {
        log_error("faac_adapter_open: memory allocation failed for adapter");
        return NULL;
    }

    adapter->num_channels = config->num_channels;

    faac_params params;
    faac_status status = faac_params_init(&params, sizeof(params));
    if (status != FAAC_OK) {
        log_error("faac_params_init failed: %s", faac_strerror(status));
        free(adapter);
        return NULL;
    }

    params.sample_rate = config->sample_rate;
    params.num_channels = config->num_channels;
    params.output_format = FAAC_STREAM_RAW; // Raw AAC for MP4 container
    params.input_format = FAAC_INPUT_16BIT;
    params.object_type = config->object_type; // FAAC_OBJ_AUTO by default

    // Set bitrate per channel (matching FFmpeg defaults: 64kbps/ch for >=32kHz, 32kbps/ch otherwise)
    uint32_t br_per_ch = config->bit_rate_per_channel;
    if (br_per_ch == 0) {
        br_per_ch = (config->sample_rate >= 32000) ? 64000 : 32000;
    }
    params.bit_rate = br_per_ch;
    params.rate_control = FAAC_RC_AUTO; // ABR when bit_rate is set

    status = faac_encoder_open(&params, &adapter->enc);
    if (status != FAAC_OK || !adapter->enc) {
        log_error("faac_encoder_open failed: %s", faac_strerror(status));
        free(adapter);
        return NULL;
    }

    // Query encoder info
    adapter->info.struct_size = sizeof(faac_encoder_info);
    status = faac_encoder_get_info(adapter->enc, &adapter->info);
    if (status != FAAC_OK) {
        log_error("faac_encoder_get_info failed: %s", faac_strerror(status));
        faac_encoder_close(&adapter->enc);
        free(adapter);
        return NULL;
    }

    // Query AudioSpecificConfig (extradata)
    const uint8_t *asc_ptr = NULL;
    uint32_t asc_len = 0;
    status = faac_encoder_asc(adapter->enc, &asc_ptr, &asc_len);
    if (status != FAAC_OK || !asc_ptr || asc_len == 0) {
        log_error("faac_encoder_asc failed: %s", faac_strerror(status));
        faac_encoder_close(&adapter->enc);
        free(adapter);
        return NULL;
    }
    adapter->asc_buf = malloc(asc_len);
    if (!adapter->asc_buf) {
        log_error("faac_adapter_open: memory allocation failed for AudioSpecificConfig");
        faac_encoder_close(&adapter->enc);
        free(adapter);
        return NULL;
    }
    memcpy(adapter->asc_buf, asc_ptr, asc_len);
    adapter->asc_size = asc_len;

    log_info("Opened FAAC encoder: %u Hz, %u ch, bitrate %u bps/ch, frame_samples %u, delay %u, obj_type %d",
             adapter->info.sample_rate, config->num_channels, adapter->info.bit_rate,
             adapter->info.frame_samples, adapter->info.encoder_delay, (int)adapter->info.object_type);

    return adapter;
}

int faac_adapter_get_asc(faac_adapter_t *adapter, const uint8_t **asc_out, size_t *asc_size_out) {
    if (!adapter || !asc_out || !asc_size_out) {
        return -1;
    }
    *asc_out = adapter->asc_buf;
    *asc_size_out = adapter->asc_size;
    return (adapter->asc_buf && adapter->asc_size > 0) ? 0 : -1;
}

uint32_t faac_adapter_get_frame_samples(faac_adapter_t *adapter) {
    return adapter ? adapter->info.frame_samples : 1024;
}

uint32_t faac_adapter_get_max_output_bytes(faac_adapter_t *adapter) {
    return adapter ? adapter->info.max_output_bytes : 2048;
}

uint32_t faac_adapter_get_sample_rate(faac_adapter_t *adapter) {
    return adapter ? adapter->info.sample_rate : 0;
}

enum faac_object_type faac_adapter_get_object_type(faac_adapter_t *adapter) {
    return adapter ? adapter->info.object_type : FAAC_OBJ_AUTO;
}

uint32_t faac_adapter_get_encoder_delay(faac_adapter_t *adapter) {
    return adapter ? adapter->info.encoder_delay : 0;
}

int faac_adapter_encode(faac_adapter_t *adapter,
                        const void *pcm_data,
                        uint32_t total_samples,
                        uint8_t *out_buf,
                        uint32_t out_cap,
                        uint32_t *bytes_written) {
    if (!adapter || !adapter->enc || !bytes_written) {
        return -1;
    }

    faac_status status = faac_encoder_encode(adapter->enc, pcm_data, total_samples,
                                            out_buf, out_cap, bytes_written);
    if (status != FAAC_OK) {
        log_error("faac_encoder_encode error: %s", faac_strerror(status));
        return -1;
    }

    return 0;
}

void faac_adapter_close(faac_adapter_t **adapter_ptr) {
    if (!adapter_ptr || !*adapter_ptr) {
        return;
    }
    faac_adapter_t *adapter = *adapter_ptr;
    if (adapter->enc) {
        faac_encoder_close(&adapter->enc);
    }
    if (adapter->asc_buf) {
        free(adapter->asc_buf);
    }
    free(adapter);
    *adapter_ptr = NULL;
}
