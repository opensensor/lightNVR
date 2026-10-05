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

#endif /* LIGHTNVR_UTILS_BASE64_H */
