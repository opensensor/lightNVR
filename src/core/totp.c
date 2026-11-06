/* SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors */
#include "core/totp.h"
#include "core/logger.h"
#include <ctype.h>
#include <stdlib.h>
#include <string.h>
#include <mbedtls/md.h>
#include <mbedtls/entropy.h>
#include <mbedtls/ctr_drbg.h>

/* ========== Base32 Encoding/Decoding ========== */

static const char BASE32_ALPHABET[] = "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

/**
 * @brief Encode binary data to base32 string
 */
static int base32_encode(const unsigned char *data, size_t data_len,
                         char *output, size_t output_size) {
    if (!data || !output || output_size == 0) return -1;

    size_t out_len = ((data_len * 8) + 4) / 5;  /* Ceiling division */
    if (out_len + 1 > output_size) return -1;

    size_t i, j = 0;
    unsigned int buffer = 0;
    int bits_left = 0;

    for (i = 0; i < data_len; i++) {
        buffer = (buffer << 8) | data[i];
        bits_left += 8;
        while (bits_left >= 5) {
            bits_left -= 5;
            output[j++] = BASE32_ALPHABET[(buffer >> bits_left) & 0x1F];
        }
    }
    if (bits_left > 0) {
        output[j++] = BASE32_ALPHABET[(buffer << (5 - bits_left)) & 0x1F];
    }
    output[j] = '\0';
    return (int)j;
}

/**
 * @brief Decode base32 string to binary data
 */
static int base32_decode(const char *input, unsigned char *output, size_t output_size) {
    if (!input || !output || output_size == 0) return -1;

    size_t input_len = strlen(input);
    size_t out_len = (input_len * 5) / 8;
    if (out_len > output_size) return -1;

    unsigned int buffer = 0;
    int bits_left = 0;
    size_t j = 0;

    for (size_t i = 0; i < input_len; i++) {
        char c = toupper((unsigned char)input[i]);
        if (c == '=' || c == ' ') continue;  /* Skip padding and spaces */

        int val;
        if (c >= 'A' && c <= 'Z') {
            val = c - 'A';
        } else if (c >= '2' && c <= '7') {
            val = c - '2' + 26;
        } else {
            return -1;  /* Invalid character */
        }

        buffer = (buffer << 5) | val;
        bits_left += 5;

        if (bits_left >= 8) {
            bits_left -= 8;
            if (j < output_size) {
                output[j++] = (unsigned char)(buffer >> bits_left);
            }
        }
    }
    return (int)j;
}

/* ========== TOTP Algorithm (RFC 6238) ========== */

/**
 * @brief Compute HMAC-SHA1
 */
static int hmac_sha1(const unsigned char *key, size_t key_len,
                     const unsigned char *data, size_t data_len,
                     unsigned char *output) {
    const mbedtls_md_info_t *md_info = mbedtls_md_info_from_type(MBEDTLS_MD_SHA1);
    if (!md_info) return -1;

    return mbedtls_md_hmac(md_info, key, key_len, data, data_len, output);
}

/**
 * @brief Generate a TOTP code for a given time step
 * @param secret_b32 Base32-encoded secret
 * @param time_step Time step counter
 * @return 6-digit TOTP code, or -1 on error
 */
static int totp_generate(const char *secret_b32, uint64_t time_step) {
    /* Decode the base32 secret */
    unsigned char secret[64];
    int secret_len = base32_decode(secret_b32, secret, sizeof(secret));
    if (secret_len <= 0) {
        log_error("TOTP: Failed to decode base32 secret");
        return -1;
    }

    /* Convert time step to big-endian 8 bytes */
    unsigned char msg[8];
    for (int i = 7; i >= 0; i--) {
        msg[i] = (unsigned char)(time_step & 0xFF);
        time_step >>= 8;
    }

    /* Compute HMAC-SHA1 */
    unsigned char hash[20];
    if (hmac_sha1(secret, (size_t)secret_len, msg, 8, hash) != 0) {
        log_error("TOTP: HMAC-SHA1 failed");
        return -1;
    }

    /* Dynamic truncation (RFC 4226 section 5.4) */
    int offset = hash[19] & 0x0F;
    uint32_t code = ((uint32_t)(hash[offset] & 0x7F) << 24) |
                    ((uint32_t)(hash[offset + 1] & 0xFF) << 16) |
                    ((uint32_t)(hash[offset + 2] & 0xFF) << 8) |
                    ((uint32_t)(hash[offset + 3] & 0xFF));

    return (int)(code % 1000000);
}

/**
 * @brief Verify a TOTP code with time window tolerance (±1 step)
 * @param secret_b32 Base32-encoded secret
 * @param code 6-digit code to verify
 * @return 0 if valid, -1 if invalid
 */
int totp_match_step(const char *secret_b32, const char *code_str, time_t now,
                    int64_t *matched_step) {
    if (matched_step) *matched_step = -1;
    if (!secret_b32 || !code_str || !matched_step || now < 0 || strlen(code_str) != 6) return -1;
    for (int i = 0; i < 6; ++i)
        if (code_str[i] < '0' || code_str[i] > '9') return -1;
    int provided = (int)strtol(code_str, NULL, 10);
    int64_t current = (int64_t)now / 30;
    for (int offset = -1; offset <= 1; ++offset) {
        int64_t step = current + offset;
        if (step < 0) continue;
        int expected = totp_generate(secret_b32, (uint64_t)step);
        if (expected >= 0 && expected == provided) *matched_step = step;
    }
    return *matched_step >= 0 ? 0 : -1;
}

/**
 * @brief Generate a random TOTP secret (160 bits = 20 bytes, base32 encoded)
 * @param secret_b32 Buffer for base32-encoded secret (at least 33 bytes)
 * @param secret_size Size of the buffer
 * @return 0 on success, -1 on failure
 */
int totp_generate_secret(char *secret_b32, size_t secret_size) {
    unsigned char raw_secret[20];  /* 160 bits */

    mbedtls_entropy_context entropy;
    mbedtls_ctr_drbg_context ctr_drbg;
    const char *pers = "totp_secret_gen";

    mbedtls_entropy_init(&entropy);
    mbedtls_ctr_drbg_init(&ctr_drbg);

    int ret = mbedtls_ctr_drbg_seed(&ctr_drbg, mbedtls_entropy_func, &entropy,
                                     (const unsigned char *)pers, strlen(pers));
    if (ret != 0) {
        log_error("TOTP: Failed to seed RNG: -0x%04x", -ret);
        mbedtls_ctr_drbg_free(&ctr_drbg);
        mbedtls_entropy_free(&entropy);
        return -1;
    }

    ret = mbedtls_ctr_drbg_random(&ctr_drbg, raw_secret, sizeof(raw_secret));
    mbedtls_ctr_drbg_free(&ctr_drbg);
    mbedtls_entropy_free(&entropy);

    if (ret != 0) {
        log_error("TOTP: Failed to generate random secret: -0x%04x", -ret);
        return -1;
    }

    if (base32_encode(raw_secret, sizeof(raw_secret), secret_b32, secret_size) < 0) {
        log_error("TOTP: Failed to base32 encode secret");
        return -1;
    }

    return 0;
}
