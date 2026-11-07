/* SPDX-License-Identifier: GPL-3.0-or-later */
#ifndef LIGHTNVR_RECORDING_ID_H
#define LIGHTNVR_RECORDING_ID_H
#include <stdbool.h>
#include <stdint.h>
#include <cjson/cJSON.h>

/* Parse a nonempty positive decimal uint64 without signs or whitespace. */
bool recording_id_parse_decimal(const char *text, uint64_t *id);

/* Accept safe positive integer JSON numbers or exact positive decimal strings.
 * Reject fractional, overflowing, or unsafe numeric identities. */
bool recording_id_parse_json(const cJSON *item, uint64_t *id);
#endif
