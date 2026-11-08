/*
 * INT_01 L9: per-camera observation evidence settings (contract section 13).
 * Shared by the stream GET/POST/PUT handlers so every path applies the same
 * bounds; migration 0095 only checks that the budgets are positive.
 */

#include "web/stream_observation_settings.h"

#include <math.h>
#include <stdio.h>
#include <string.h>

#include "utils/strings.h"

static int fail(char *error, size_t error_size, const char *message) {
    if (error && error_size) snprintf(error, error_size, "%s", message);
    return -1;
}

static bool is_integer_in_range(const cJSON *value, double minimum, double maximum) {
    return cJSON_IsNumber(value) && isfinite(value->valuedouble) &&
           value->valuedouble == floor(value->valuedouble) &&
           value->valuedouble >= minimum && value->valuedouble <= maximum;
}

int stream_observation_labels_normalize(const char *input, char *out, size_t out_size,
                                        char *error, size_t error_size) {
    if (!input || !out || out_size == 0) return fail(error, error_size, "observation_labels is invalid");
    char result[OBSERVATION_LABELS_MAX] = "";
    size_t used = 0;
    int count = 0;

    /* Whitespace only (or empty) means "every object label". */
    const char *scan = input;
    while (*scan == ' ' || *scan == '\t') scan++;
    if (*scan == '\0') {
        out[0] = '\0';
        return 0;
    }

    const char *cursor = input;
    for (;;) {
        const char *end = strchr(cursor, ',');
        const char *stop = end ? end : cursor + strlen(cursor);
        const char *first = cursor;
        const char *last = stop;
        while (first < last && (*first == ' ' || *first == '\t')) first++;
        while (last > first && (last[-1] == ' ' || last[-1] == '\t')) last--;
        size_t length = (size_t)(last - first);
        if (length == 0) {
            return fail(error, error_size, "observation_labels must not contain empty labels");
        }
        if (length > OBSERVATION_LABEL_MAX_BYTES) {
            return fail(error, error_size, "observation_labels entries must be 1..31 bytes");
        }
        for (const char *c = first; c < last; c++) {
            unsigned char ch = (unsigned char)*c;
            if (ch < 0x20 || ch > 0x7e) {
                return fail(error, error_size,
                            "observation_labels entries must be printable ASCII");
            }
        }
        if (++count > OBSERVATION_LABELS_MAX_COUNT) {
            return fail(error, error_size, "observation_labels allows at most 16 labels");
        }
        if (used + length + (count > 1 ? 1 : 0) >= sizeof(result)) {
            return fail(error, error_size, "observation_labels is too long");
        }
        if (count > 1) result[used++] = ',';
        memcpy(result + used, first, length);
        used += length;
        result[used] = '\0';
        if (!end) break;
        cursor = end + 1;
    }

    if (used >= out_size) return fail(error, error_size, "observation_labels is too long");
    memcpy(out, result, used + 1);
    return 0;
}

void stream_observation_settings_to_json(cJSON *object, const stream_config_t *config) {
    if (!object || !config) return;
    cJSON_AddBoolToObject(object, "observation_evidence_enabled",
                          config->observation_evidence_enabled);
    cJSON_AddStringToObject(object, "observation_labels", config->observation_labels);
    cJSON_AddNumberToObject(object, "observation_max_age_days",
                            config->observation_max_age_days > 0
                                ? config->observation_max_age_days
                                : OBSERVATION_DEFAULT_MAX_AGE_DAYS);
    cJSON_AddNumberToObject(object, "observation_max_bytes",
                            (double)(config->observation_max_bytes > 0
                                ? config->observation_max_bytes
                                : OBSERVATION_DEFAULT_MAX_BYTES));
}

int stream_observation_settings_apply_json(const cJSON *body, stream_config_t *config,
                                           bool *changed, char *error, size_t error_size) {
    if (changed) *changed = false;
    if (!body || !config) return fail(error, error_size, "observation settings are invalid");

    const cJSON *enabled = cJSON_GetObjectItemCaseSensitive(body, "observation_evidence_enabled");
    const cJSON *labels = cJSON_GetObjectItemCaseSensitive(body, "observation_labels");
    const cJSON *max_age = cJSON_GetObjectItemCaseSensitive(body, "observation_max_age_days");
    const cJSON *max_bytes = cJSON_GetObjectItemCaseSensitive(body, "observation_max_bytes");

    if (enabled && !cJSON_IsBool(enabled)) {
        return fail(error, error_size, "observation_evidence_enabled must be a boolean");
    }
    char normalized[OBSERVATION_LABELS_MAX] = "";
    if (labels) {
        if (!cJSON_IsString(labels) || !labels->valuestring) {
            return fail(error, error_size, "observation_labels must be a string");
        }
        if (stream_observation_labels_normalize(labels->valuestring, normalized,
                                                sizeof(normalized), error, error_size) != 0) {
            return -1;
        }
    }
    if (max_age && !is_integer_in_range(max_age, OBSERVATION_MAX_AGE_DAYS_MIN,
                                        OBSERVATION_MAX_AGE_DAYS_MAX)) {
        return fail(error, error_size, "observation_max_age_days must be an integer between 1 and 90");
    }
    if (max_bytes && !is_integer_in_range(max_bytes, (double)OBSERVATION_MAX_BYTES_MIN,
                                          (double)OBSERVATION_MAX_BYTES_MAX)) {
        return fail(error, error_size,
                    "observation_max_bytes must be an integer between 67108864 (64 MiB) "
                    "and 1099511627776 (1 TiB)");
    }

    bool any_change = false;
    if (enabled) {
        bool value = cJSON_IsTrue(enabled);
        any_change |= value != config->observation_evidence_enabled;
        config->observation_evidence_enabled = value;
    }
    if (labels) {
        any_change |= strcmp(normalized, config->observation_labels) != 0;
        safe_strcpy(config->observation_labels, normalized, sizeof(config->observation_labels), 0);
    }
    if (max_age) {
        int value = (int)max_age->valuedouble;
        any_change |= value != config->observation_max_age_days;
        config->observation_max_age_days = value;
    }
    if (max_bytes) {
        int64_t value = (int64_t)max_bytes->valuedouble;
        any_change |= value != config->observation_max_bytes;
        config->observation_max_bytes = value;
    }
    if (changed) *changed = any_change;
    return 0;
}
