/* SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors */
#ifndef LIGHTNVR_TOTP_H
#define LIGHTNVR_TOTP_H
#include <stddef.h>
#include <stdint.h>
#include <time.h>
/* Pure RFC 6238 verification; callers must atomically persist replay/attempt state. */
int totp_match_step(const char *secret, const char *code, time_t now, int64_t *matched_step);
int totp_generate_secret(char *secret, size_t size);
#endif
