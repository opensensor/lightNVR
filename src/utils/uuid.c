/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

#define _POSIX_C_SOURCE 200809L

#include "utils/uuid.h"

#include <ctype.h>
#include <errno.h>
#include <stdio.h>
#include <string.h>
#include <sys/random.h>

#include <mbedtls/md.h>

bool lightnvr_uuid_is_valid(const char *value) {
    if (!value || strnlen(value, LIGHTNVR_UUID_STRING_SIZE) != 36) {
        return false;
    }
    for (int index = 0; index < 36; index++) {
        if (index == 8 || index == 13 || index == 18 || index == 23) {
            if (value[index] != '-') return false;
        } else if (!isxdigit((unsigned char)value[index])) {
            return false;
        }
    }
    return true;
}

static int random_bytes(unsigned char *buffer, size_t length) {
    size_t offset = 0;
    while (offset < length) {
        ssize_t count = getrandom(buffer + offset, length - offset, 0);
        if (count > 0) {
            offset += (size_t)count;
            continue;
        }
        if (count < 0 && errno == EINTR) continue;
        break;
    }
    if (offset == length) return 0;

    FILE *random = fopen("/dev/urandom", "rb");
    if (!random) return -1;
    size_t count = fread(buffer, 1, length, random);
    fclose(random);
    return count == length ? 0 : -1;
}

/* Accept any byte, but return only one lowercase hexadecimal character. */
static char hex_digit(unsigned char value) {
    static const char alphabet[] = "0123456789abcdef";
    return alphabet[value & 0x0fU];
}

int lightnvr_uuid_generate_v4(char output[LIGHTNVR_UUID_STRING_SIZE]) {
    if (!output) return -1;
    unsigned char bytes[16];
    if (random_bytes(bytes, sizeof(bytes)) != 0) return -1;
    bytes[6] = (unsigned char)((bytes[6] & 0x0fU) | 0x40U);
    bytes[8] = (unsigned char)((bytes[8] & 0x3fU) | 0x80U);
    /* Map entropy to a fixed alphabet rather than propagating formatted input
     * into paths. Every output byte is a literal hex digit or separator. */
    size_t position = 0;
    for (size_t index = 0; index < sizeof(bytes); ++index) {
        if (index == 4 || index == 6 || index == 8 || index == 10)
            output[position++] = '-';
        output[position++] = hex_digit((unsigned char)(bytes[index] >> 4));
        output[position++] = hex_digit(bytes[index]);
    }
    output[position] = '\0';
    return 0;
}

static int hex_value(char c) {
    if (c >= '0' && c <= '9') return c - '0';
    if (c >= 'a' && c <= 'f') return c - 'a' + 10;
    if (c >= 'A' && c <= 'F') return c - 'A' + 10;
    return -1;
}

int lightnvr_uuid_generate_v5(const char *namespace_uuid, const char *name,
                              size_t name_len,
                              char output[LIGHTNVR_UUID_STRING_SIZE]) {
    if (!output || !name || !lightnvr_uuid_is_valid(namespace_uuid)) return -1;

    unsigned char namespace_bytes[16];
    size_t byte = 0;
    for (int index = 0; index < 36; ++index) {
        if (index == 8 || index == 13 || index == 18 || index == 23) continue;
        int high = hex_value(namespace_uuid[index]);
        int low = hex_value(namespace_uuid[++index]);
        namespace_bytes[byte++] = (unsigned char)((high << 4) | low);
    }

    const mbedtls_md_info_t *sha1 = mbedtls_md_info_from_type(MBEDTLS_MD_SHA1);
    if (!sha1) return -1;
    mbedtls_md_context_t context;
    mbedtls_md_init(&context);
    unsigned char digest[20];
    int rc = mbedtls_md_setup(&context, sha1, 0);
    if (rc == 0) rc = mbedtls_md_starts(&context);
    if (rc == 0) rc = mbedtls_md_update(&context, namespace_bytes, sizeof(namespace_bytes));
    if (rc == 0) rc = mbedtls_md_update(&context, (const unsigned char *)name, name_len);
    if (rc == 0) rc = mbedtls_md_finish(&context, digest);
    mbedtls_md_free(&context);
    if (rc != 0) return -1;

    digest[6] = (unsigned char)((digest[6] & 0x0fU) | 0x50U);
    digest[8] = (unsigned char)((digest[8] & 0x3fU) | 0x80U);
    size_t position = 0;
    for (size_t index = 0; index < 16; ++index) {
        if (index == 4 || index == 6 || index == 8 || index == 10)
            output[position++] = '-';
        output[position++] = hex_digit((unsigned char)(digest[index] >> 4));
        output[position++] = hex_digit(digest[index]);
    }
    output[position] = '\0';
    return 0;
}
