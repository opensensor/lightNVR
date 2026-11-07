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
    return cJSON_IsString(item) && recording_id_parse_decimal(item->valuestring, id);
}

bool recording_id_parse_decimal(const char *text, uint64_t *id) {
    if (!text || !text[0]) return false;
    uint64_t value = 0;
    for (const char *p = text; *p; p++) {
        if (*p < '0' || *p > '9' || value > (UINT64_MAX - (unsigned)(*p - '0')) / 10) return false;
        value = value * 10 + (unsigned)(*p - '0');
    }
    *id = value;
    return value != 0;
}
