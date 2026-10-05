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
