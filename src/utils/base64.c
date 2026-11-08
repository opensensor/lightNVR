/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

#include <stdint.h>
#include <stdlib.h>

#include "utils/base64.h"

static const char BASE64_ALPHABET[] =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

size_t base64_encoded_size(size_t input_len) {
    /* Every 3 input bytes become 4 output characters; the last group is padded. */
    if (input_len > (SIZE_MAX - 1) / 4 * 3 - 2) {
        return 0;
    }
    return ((input_len + 2) / 3) * 4 + 1;
}

int base64_encode(const unsigned char *input, size_t input_len,
                  char *dst, size_t dst_size, size_t *out_len) {
    if (out_len) {
        *out_len = 0;
    }
    if (!dst || dst_size == 0 || (!input && input_len > 0)) {
        return -1;
    }

    size_t needed = base64_encoded_size(input_len);
    if (needed == 0 || dst_size < needed) {
        return -1;
    }

    size_t written = 0;
    size_t i = 0;
    while (i + 2 < input_len) {
        uint32_t triple = ((uint32_t)input[i] << 16) |
                          ((uint32_t)input[i + 1] << 8) |
                          (uint32_t)input[i + 2];
        dst[written++] = BASE64_ALPHABET[(triple >> 18) & 0x3F];
        dst[written++] = BASE64_ALPHABET[(triple >> 12) & 0x3F];
        dst[written++] = BASE64_ALPHABET[(triple >> 6) & 0x3F];
        dst[written++] = BASE64_ALPHABET[triple & 0x3F];
        i += 3;
    }

    size_t remaining = input_len - i;
    if (remaining == 1) {
        uint32_t triple = (uint32_t)input[i] << 16;
        dst[written++] = BASE64_ALPHABET[(triple >> 18) & 0x3F];
        dst[written++] = BASE64_ALPHABET[(triple >> 12) & 0x3F];
        dst[written++] = '=';
        dst[written++] = '=';
    } else if (remaining == 2) {
        uint32_t triple = ((uint32_t)input[i] << 16) | ((uint32_t)input[i + 1] << 8);
        dst[written++] = BASE64_ALPHABET[(triple >> 18) & 0x3F];
        dst[written++] = BASE64_ALPHABET[(triple >> 12) & 0x3F];
        dst[written++] = BASE64_ALPHABET[(triple >> 6) & 0x3F];
        dst[written++] = '=';
    }

    dst[written] = '\0';
    if (out_len) {
        *out_len = written;
    }
    return 0;
}

char *base64_encode_alloc(const unsigned char *input, size_t input_len) {
    size_t needed = base64_encoded_size(input_len);
    if (needed == 0) {
        return NULL;
    }
    char *encoded = malloc(needed);
    if (!encoded) {
        return NULL;
    }
    if (base64_encode(input, input_len, encoded, needed, NULL) != 0) {
        free(encoded);
        return NULL;
    }
    return encoded;
}

static const char BASE64URL_ALPHABET[] =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

int base64url_encode(const unsigned char *input, size_t input_len,
                     char *dst, size_t dst_size, size_t *out_len) {
    if (out_len) *out_len = 0;
    if (!dst || dst_size == 0 || (!input && input_len > 0)) return -1;
    if (input_len > (SIZE_MAX - 3) / 4) return -1;
    size_t needed = (input_len * 4 + 2) / 3 + 1;
    if (dst_size < needed) return -1;

    size_t written = 0;
    size_t i = 0;
    for (; i + 2 < input_len; i += 3) {
        uint32_t triple = ((uint32_t)input[i] << 16) |
                          ((uint32_t)input[i + 1] << 8) | (uint32_t)input[i + 2];
        dst[written++] = BASE64URL_ALPHABET[(triple >> 18) & 0x3F];
        dst[written++] = BASE64URL_ALPHABET[(triple >> 12) & 0x3F];
        dst[written++] = BASE64URL_ALPHABET[(triple >> 6) & 0x3F];
        dst[written++] = BASE64URL_ALPHABET[triple & 0x3F];
    }
    size_t remaining = input_len - i;
    if (remaining > 0) {
        uint32_t triple = (uint32_t)input[i] << 16;
        if (remaining == 2) triple |= (uint32_t)input[i + 1] << 8;
        dst[written++] = BASE64URL_ALPHABET[(triple >> 18) & 0x3F];
        dst[written++] = BASE64URL_ALPHABET[(triple >> 12) & 0x3F];
        if (remaining == 2) dst[written++] = BASE64URL_ALPHABET[(triple >> 6) & 0x3F];
    }
    dst[written] = '\0';
    if (out_len) *out_len = written;
    return 0;
}

static int base64url_value(char c) {
    if (c >= 'A' && c <= 'Z') return c - 'A';
    if (c >= 'a' && c <= 'z') return c - 'a' + 26;
    if (c >= '0' && c <= '9') return c - '0' + 52;
    if (c == '-') return 62;
    if (c == '_') return 63;
    return -1;
}

int base64url_decode(const char *src, size_t src_len,
                     unsigned char *dst, size_t dst_size, size_t *out_len) {
    if (out_len) *out_len = 0;
    if ((!src && src_len > 0) || (!dst && src_len > 0)) return -1;
    if (src_len % 4 == 1) return -1;
    size_t needed = src_len / 4 * 3 + (src_len % 4 ? src_len % 4 - 1 : 0);
    if (needed > dst_size) return -1;

    size_t written = 0;
    uint32_t buffer = 0;
    int bits = 0;
    for (size_t i = 0; i < src_len; ++i) {
        int value = base64url_value(src[i]);
        if (value < 0) return -1;
        buffer = (buffer << 6) | (uint32_t)value;
        bits += 6;
        if (bits >= 8) {
            bits -= 8;
            dst[written++] = (unsigned char)((buffer >> bits) & 0xFF);
        }
    }
    /* Leftover bits must be zero so each value has exactly one encoding. */
    if (bits > 0 && (buffer & ((1U << bits) - 1U)) != 0) return -1;
    if (out_len) *out_len = written;
    return 0;
}
