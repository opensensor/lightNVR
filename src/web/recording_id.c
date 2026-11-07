/* SPDX-License-Identifier: GPL-3.0-or-later */
#include "web/recording_id.h"

/* Timeline IDs above JavaScript's exact integer range arrive as decimal strings. */
bool recording_id_parse_json(const cJSON *item, uint64_t *id) {
    if (cJSON_IsNumber(item)) {
        double value = item->valuedouble;
        if (!(value >= 1 && value <= 9007199254740991.0)) return false;
        *id = (uint64_t)value;
        return (double)*id == value;
    }
    if (!cJSON_IsString(item) || !item->valuestring || !item->valuestring[0]) return false;
    uint64_t value = 0;
    for (const char *p = item->valuestring; *p; p++) {
        if (*p < '0' || *p > '9' || value > (UINT64_MAX - (unsigned)(*p - '0')) / 10) return false;
        value = value * 10 + (unsigned)(*p - '0');
    }
    *id = value;
    return value != 0;
}

