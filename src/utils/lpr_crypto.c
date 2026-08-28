#define _GNU_SOURCE

#include "utils/lpr_crypto.h"

#include <ctype.h>
#include <errno.h>
#include <stdlib.h>
#include <string.h>
#include <sys/random.h>

#include <mbedtls/gcm.h>
#include <mbedtls/md.h>

#include "utils/memory.h"

static int hex_nibble(char value) {
    if (value >= '0' && value <= '9') return value - '0';
    if (value >= 'a' && value <= 'f') return value - 'a' + 10;
    if (value >= 'A' && value <= 'F') return value - 'A' + 10;
    return -1;
}

static int load_master_key(uint8_t key[LPR_CRYPTO_KEY_SIZE]) {
    const char *hex = getenv("LIGHTNVR_LPR_MASTER_KEY_HEX");
    if (!hex || strlen(hex) != LPR_CRYPTO_KEY_SIZE * 2) return -1;
    for (size_t i = 0; i < LPR_CRYPTO_KEY_SIZE; ++i) {
        int high = hex_nibble(hex[i * 2]);
        int low = hex_nibble(hex[i * 2 + 1]);
        if (high < 0 || low < 0) {
            secure_zero_memory(key, LPR_CRYPTO_KEY_SIZE);
            return -1;
        }
        key[i] = (uint8_t)((high << 4) | low);
    }
    return 0;
}

static int hmac_with_domain(const char *domain, const void *data, size_t data_len,
                            uint8_t output[LPR_CRYPTO_HMAC_SIZE]) {
    uint8_t master[LPR_CRYPTO_KEY_SIZE];
    uint8_t derived[LPR_CRYPTO_HMAC_SIZE];
    if (load_master_key(master) != 0) return -1;
    const mbedtls_md_info_t *info = mbedtls_md_info_from_type(MBEDTLS_MD_SHA256);
    if (!info || mbedtls_md_hmac(info, master, sizeof(master),
                                 (const unsigned char *)domain, strlen(domain),
                                 derived) != 0) {
        secure_zero_memory(master, sizeof(master));
        return -1;
    }
    int result = mbedtls_md_hmac(info, derived, sizeof(derived),
                                 (const unsigned char *)data, data_len, output);
    secure_zero_memory(master, sizeof(master));
    secure_zero_memory(derived, sizeof(derived));
    return result == 0 ? 0 : -1;
}

static int random_bytes(uint8_t *output, size_t length) {
    size_t offset = 0;
    while (offset < length) {
        ssize_t count = getrandom(output + offset, length - offset, 0);
        if (count > 0) {
            offset += (size_t)count;
        } else if (count < 0 && errno == EINTR) {
            continue;
        } else {
            return -1;
        }
    }
    return 0;
}

int lpr_crypto_key_available(void) {
    uint8_t key[LPR_CRYPTO_KEY_SIZE];
    int available = load_master_key(key) == 0;
    secure_zero_memory(key, sizeof(key));
    return available;
}

int lpr_canonicalize_plate(const char *plate, char *canonical, size_t size) {
    if (!plate || !canonical || size < 3) return -1;
    canonical[0] = '\0';
    size_t output = 0;
    for (const unsigned char *p = (const unsigned char *)plate; *p; ++p) {
        if (*p > 127) {
            secure_zero_memory(canonical, size);
            return -1;
        }
        if (!isalnum(*p)) continue;
        if (output + 1 >= size) {
            secure_zero_memory(canonical, size);
            return -1;
        }
        canonical[output++] = (char)toupper(*p);
    }
    if (output < 2) {
        secure_zero_memory(canonical, size);
        return -1;
    }
    canonical[output] = '\0';
    return (int)output;
}

int lpr_crypto_encrypt(const char *plaintext, const void *aad, size_t aad_len,
                       uint8_t nonce[LPR_CRYPTO_NONCE_SIZE],
                       uint8_t *ciphertext, size_t ciphertext_size,
                       size_t *ciphertext_len,
                       uint8_t tag[LPR_CRYPTO_TAG_SIZE]) {
    if (!plaintext || !nonce || !ciphertext || !ciphertext_len || !tag) return -1;
    size_t length = strlen(plaintext);
    if (length == 0 || length > ciphertext_size) return -1;
    uint8_t key[LPR_CRYPTO_KEY_SIZE];
    if (load_master_key(key) != 0 || random_bytes(nonce, LPR_CRYPTO_NONCE_SIZE) != 0) {
        secure_zero_memory(key, sizeof(key));
        return -1;
    }

    mbedtls_gcm_context context;
    mbedtls_gcm_init(&context);
    int result = mbedtls_gcm_setkey(&context, MBEDTLS_CIPHER_ID_AES,
                                    key, LPR_CRYPTO_KEY_SIZE * 8);
    if (result == 0) {
        result = mbedtls_gcm_crypt_and_tag(
            &context, MBEDTLS_GCM_ENCRYPT, length,
            nonce, LPR_CRYPTO_NONCE_SIZE,
            (const unsigned char *)aad, aad_len,
            (const unsigned char *)plaintext, ciphertext,
            LPR_CRYPTO_TAG_SIZE, tag);
    }
    mbedtls_gcm_free(&context);
    secure_zero_memory(key, sizeof(key));
    if (result != 0) return -1;
    *ciphertext_len = length;
    return 0;
}

int lpr_crypto_decrypt(const uint8_t *ciphertext, size_t ciphertext_len,
                       const void *aad, size_t aad_len,
                       const uint8_t nonce[LPR_CRYPTO_NONCE_SIZE],
                       const uint8_t tag[LPR_CRYPTO_TAG_SIZE],
                       char *plaintext, size_t plaintext_size) {
    if (!ciphertext || !nonce || !tag || !plaintext ||
        ciphertext_len == 0 || ciphertext_len >= plaintext_size) return -1;
    secure_zero_memory(plaintext, plaintext_size);
    uint8_t key[LPR_CRYPTO_KEY_SIZE];
    if (load_master_key(key) != 0) return -1;

    mbedtls_gcm_context context;
    mbedtls_gcm_init(&context);
    int result = mbedtls_gcm_setkey(&context, MBEDTLS_CIPHER_ID_AES,
                                    key, LPR_CRYPTO_KEY_SIZE * 8);
    if (result == 0) {
        result = mbedtls_gcm_auth_decrypt(
            &context, ciphertext_len,
            nonce, LPR_CRYPTO_NONCE_SIZE,
            (const unsigned char *)aad, aad_len,
            tag, LPR_CRYPTO_TAG_SIZE,
            ciphertext, (unsigned char *)plaintext);
    }
    mbedtls_gcm_free(&context);
    secure_zero_memory(key, sizeof(key));
    if (result != 0) {
        secure_zero_memory(plaintext, plaintext_size);
        return -1;
    }
    plaintext[ciphertext_len] = '\0';
    return 0;
}

int lpr_crypto_blind_index(const char *canonical_plate,
                           uint8_t output[LPR_CRYPTO_HMAC_SIZE]) {
    if (!canonical_plate || !canonical_plate[0] || !output) return -1;
    return hmac_with_domain("lightnvr:lpr:exact:v1", canonical_plate,
                            strlen(canonical_plate), output);
}

int lpr_crypto_fingerprint(const void *data, size_t data_len,
                           uint8_t output[LPR_CRYPTO_HMAC_SIZE]) {
    if ((!data && data_len > 0) || !output) return -1;
    return hmac_with_domain("lightnvr:lpr:fingerprint:v1", data, data_len, output);
}
