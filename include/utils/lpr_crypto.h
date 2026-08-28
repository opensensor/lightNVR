#ifndef LIGHTNVR_LPR_CRYPTO_H
#define LIGHTNVR_LPR_CRYPTO_H

#include <stddef.h>
#include <stdint.h>

#define LPR_CRYPTO_KEY_SIZE 32
#define LPR_CRYPTO_NONCE_SIZE 12
#define LPR_CRYPTO_TAG_SIZE 16
#define LPR_CRYPTO_HMAC_SIZE 32

/** Return 1 when a valid LIGHTNVR_LPR_MASTER_KEY_HEX is available, else 0. */
int lpr_crypto_key_available(void);

/** Uppercase and retain ASCII A-Z/0-9 only. Returns length, or -1. */
int lpr_canonicalize_plate(const char *plate, char *canonical, size_t size);

int lpr_crypto_encrypt(const char *plaintext, const void *aad, size_t aad_len,
                       uint8_t nonce[LPR_CRYPTO_NONCE_SIZE],
                       uint8_t *ciphertext, size_t ciphertext_size,
                       size_t *ciphertext_len,
                       uint8_t tag[LPR_CRYPTO_TAG_SIZE]);

int lpr_crypto_decrypt(const uint8_t *ciphertext, size_t ciphertext_len,
                       const void *aad, size_t aad_len,
                       const uint8_t nonce[LPR_CRYPTO_NONCE_SIZE],
                       const uint8_t tag[LPR_CRYPTO_TAG_SIZE],
                       char *plaintext, size_t plaintext_size);

int lpr_crypto_blind_index(const char *canonical_plate,
                           uint8_t output[LPR_CRYPTO_HMAC_SIZE]);

int lpr_crypto_fingerprint(const void *data, size_t data_len,
                           uint8_t output[LPR_CRYPTO_HMAC_SIZE]);

#endif
