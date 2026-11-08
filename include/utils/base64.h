/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

#ifndef LIGHTNVR_UTILS_BASE64_H
#define LIGHTNVR_UTILS_BASE64_H

#include <stddef.h>

/**
 * Bytes required to hold the standard (RFC 4648, padded) base64 encoding of
 * input_len bytes, including the terminating NUL. Returns 0 on overflow.
 */
size_t base64_encoded_size(size_t input_len);

/**
 * Encode input into dst as NUL-terminated base64.
 *
 * @param out_len  Optional; receives the encoded length (without the NUL).
 * @return 0 on success, -1 when dst is too small or the arguments are invalid.
 */
int base64_encode(const unsigned char *input, size_t input_len,
                  char *dst, size_t dst_size, size_t *out_len);

/**
 * Convenience wrapper that returns a malloc'd NUL-terminated encoding.
 * The caller frees the result. Returns NULL on failure.
 */
char *base64_encode_alloc(const unsigned char *input, size_t input_len);

/**
 * Encode input as unpadded base64url (RFC 4648 section 5) into dst, NUL
 * terminated. Needs ceil(4 * input_len / 3) + 1 bytes.
 *
 * @return 0 on success, -1 when dst is too small or the arguments are invalid.
 */
int base64url_encode(const unsigned char *input, size_t input_len,
                     char *dst, size_t dst_size, size_t *out_len);

/**
 * Strictly decode unpadded base64url. Rejects padding, the standard '+' and
 * '/' characters, any other byte outside the alphabet, an impossible length
 * (len % 4 == 1) and non-zero trailing bits, so each value has exactly one
 * accepted encoding.
 *
 * @return 0 on success, -1 on invalid input or when dst is too small.
 */
int base64url_decode(const char *src, size_t src_len,
                     unsigned char *dst, size_t dst_size, size_t *out_len);

#endif /* LIGHTNVR_UTILS_BASE64_H */
