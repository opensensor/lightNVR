/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

#ifndef LIGHTNVR_UTILS_UUID_H
#define LIGHTNVR_UTILS_UUID_H

#include <stdbool.h>
#include <stddef.h>

#define LIGHTNVR_UUID_STRING_SIZE 37

/* Validate the canonical 8-4-4-4-12 hexadecimal UUID representation. */
bool lightnvr_uuid_is_valid(const char *value);

/* Generate a cryptographically random RFC 4122 version-4 UUID. */
int lightnvr_uuid_generate_v4(char output[LIGHTNVR_UUID_STRING_SIZE]);

/* Generate the RFC 4122 name-based version-5 (SHA-1) UUID of name within the
 * canonical namespace UUID. Output is lowercase canonical. Returns 0 or -1. */
int lightnvr_uuid_generate_v5(const char *namespace_uuid, const char *name,
                              size_t name_len,
                              char output[LIGHTNVR_UUID_STRING_SIZE]);

#endif /* LIGHTNVR_UTILS_UUID_H */
