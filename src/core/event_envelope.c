#define _POSIX_C_SOURCE 200809L

#include "core/event_envelope.h"

#include <ctype.h>
#include <limits.h>
#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>

#include "utils/uuid.h"

static const event_type_definition_t EVENT_TYPES[] = {
    {
        .type = "io.lightnvr.detection.object.v1",
        .family = "detection",
        .description = "One or more objects observed by a camera",
        .severity = EVENT_SEVERITY_INFO,
        .sensitivity = EVENT_SENSITIVITY_OPERATIONAL,
        .media_policy = EVENT_MEDIA_REFERENCE_ALLOWED,
        .expected_rate = EVENT_RATE_HIGH,
        .subject_kind = EVENT_SUBJECT_CAMERA,
        .default_expiry_seconds = 3600,
    },
    {
        .type = "io.lightnvr.camera.offline.v1",
        .family = "camera",
        .description = "Camera connectivity declared offline",
        .severity = EVENT_SEVERITY_WARNING,
        .sensitivity = EVENT_SENSITIVITY_OPERATIONAL,
        .media_policy = EVENT_MEDIA_FORBIDDEN,
        .expected_rate = EVENT_RATE_LOW,
        .subject_kind = EVENT_SUBJECT_CAMERA,
        .default_expiry_seconds = 86400,
    },
    {
        .type = "io.lightnvr.stream.recording_gap.v1",
        .family = "stream",
        .description = "A gap was detected in a camera recording",
        .severity = EVENT_SEVERITY_WARNING,
        .sensitivity = EVENT_SENSITIVITY_OPERATIONAL,
        .media_policy = EVENT_MEDIA_REFERENCE_ALLOWED,
        .expected_rate = EVENT_RATE_LOW,
        .subject_kind = EVENT_SUBJECT_CAMERA,
        .default_expiry_seconds = 604800,
    },
    {
        .type = "io.lightnvr.storage.pressure.v1",
        .family = "storage",
        .description = "Storage usage crossed an operational threshold",
        .severity = EVENT_SEVERITY_CRITICAL,
        .sensitivity = EVENT_SENSITIVITY_INTERNAL,
        .media_policy = EVENT_MEDIA_FORBIDDEN,
        .expected_rate = EVENT_RATE_LOW,
        .subject_kind = EVENT_SUBJECT_STORAGE,
        .default_expiry_seconds = 604800,
    },
};

static void set_error(char *error, size_t error_size, const char *message) {
    if (!error || error_size == 0 || error[0]) return;
    snprintf(error, error_size, "%s", message);
}

static bool valid_text(const char *value, size_t maximum, bool allow_empty) {
    if (!value) return false;
    size_t length = strnlen(value, maximum);
    if ((!allow_empty && length == 0) || length == maximum) return false;
    for (const unsigned char *cursor = (const unsigned char *)value; *cursor;
         cursor++) {
        if (iscntrl(*cursor)) return false;
    }
    return true;
}

static int format_event_time(time_t occurred_at, char output[EVENT_TIME_MAX]) {
    struct tm utc;
    if (!gmtime_r(&occurred_at, &utc)) return -1;
    return strftime(output, EVENT_TIME_MAX, "%Y-%m-%dT%H:%M:%SZ", &utc) > 0
        ? 0
        : -1;
}

static bool valid_event_time(const char *value) {
    if (!value || strnlen(value, EVENT_TIME_MAX) != 20) return false;
    const int separators[] = {4, 7, 10, 13, 16, 19};
    const char expected[] = {'-', '-', 'T', ':', ':', 'Z'};
    for (size_t index = 0; index < sizeof(separators) / sizeof(separators[0]);
         index++) {
        if (value[separators[index]] != expected[index]) return false;
    }
    for (int index = 0; index < 19; index++) {
        if (index == 4 || index == 7 || index == 10 || index == 13 ||
            index == 16) {
            continue;
        }
        if (!isdigit((unsigned char)value[index])) return false;
    }
    return true;
}

static bool contains_case_insensitive(const char *value, const char *needle) {
    if (!value || !needle || !needle[0]) return false;
    size_t needle_length = strlen(needle);
    for (const char *start = value; *start; start++) {
        size_t index = 0;
        while (index < needle_length && start[index] &&
               tolower((unsigned char)start[index]) ==
                   tolower((unsigned char)needle[index])) {
            index++;
        }
        if (index == needle_length) return true;
    }
    return false;
}

static bool forbidden_data_key(const char *key) {
    if (!key) return false;
    if (contains_case_insensitive(key, "password") ||
        contains_case_insensitive(key, "secret") ||
        contains_case_insensitive(key, "credential") ||
        contains_case_insensitive(key, "authorization") ||
        contains_case_insensitive(key, "cookie") ||
        contains_case_insensitive(key, "api_key") ||
        contains_case_insensitive(key, "apikey") ||
        contains_case_insensitive(key, "token")) {
        return true;
    }
    size_t length = strlen(key);
    return contains_case_insensitive(key, "_path") ||
        (length == 4 && contains_case_insensitive(key, "path"));
}

static bool has_case_insensitive_suffix(const char *value,
                                        const char *suffix) {
    if (!value || !suffix) return false;
    size_t value_length = strlen(value);
    size_t suffix_length = strlen(suffix);
    if (suffix_length > value_length) return false;
    return contains_case_insensitive(value + value_length - suffix_length,
                                     suffix);
}

static bool looks_like_raw_filesystem_path(const char *value,
                                           const char *property_name) {
    if (!value || !value[0]) return false;
    if (strncasecmp(value, "file://", 7) == 0) return true;
    if (isalpha((unsigned char)value[0]) && value[1] == ':' &&
        (value[2] == '/' || value[2] == '\\')) {
        return true;
    }
    if (value[0] != '/') return false;
    return !property_name ||
        !has_case_insensitive_suffix(property_name, "_url");
}

static int validate_data_node(const cJSON *node, int depth, char *error,
                              size_t error_size) {
    if (!node || depth > 16) {
        set_error(error, error_size, "event data exceeds maximum depth");
        return -1;
    }
    if (node->string) {
        if (!valid_text(node->string, 128, false)) {
            set_error(error, error_size, "event data contains an invalid key");
            return -1;
        }
        if (forbidden_data_key(node->string)) {
            set_error(error, error_size,
                      "event data contains a sensitive or filesystem key");
            return -1;
        }
    }
    if (cJSON_IsObject(node) || cJSON_IsArray(node)) {
        for (const cJSON *child = node->child; child; child = child->next) {
            if (validate_data_node(child, depth + 1, error, error_size) != 0) {
                return -1;
            }
        }
        return 0;
    }
    if (cJSON_IsString(node)) {
        if (!valid_text(node->valuestring, EVENT_DATA_MAX_BYTES, true)) {
            set_error(error, error_size, "event data contains invalid text");
            return -1;
        }
        if (looks_like_raw_filesystem_path(node->valuestring, node->string)) {
            set_error(error, error_size,
                      "event data contains a raw filesystem path");
            return -1;
        }
        return 0;
    }
    if (cJSON_IsNumber(node)) {
        if (!isfinite(node->valuedouble)) {
            set_error(error, error_size, "event data contains a non-finite number");
            return -1;
        }
        return 0;
    }
    if (cJSON_IsBool(node) || cJSON_IsNull(node)) return 0;
    set_error(error, error_size, "event data contains an unsupported value");
    return -1;
}

static const cJSON *required_field(const cJSON *data, const char *name,
                                   int expected_type, char *error,
                                   size_t error_size) {
    const cJSON *field = cJSON_GetObjectItemCaseSensitive(data, name);
    bool valid = field != NULL;
    if (expected_type == cJSON_String) valid = cJSON_IsString(field);
    if (expected_type == cJSON_Number) valid = cJSON_IsNumber(field);
    if (expected_type == cJSON_Array) valid = cJSON_IsArray(field);
    if (!valid) {
        char message[160];
        snprintf(message, sizeof(message),
                 "event data is missing required field '%s'", name);
        set_error(error, error_size, message);
        return NULL;
    }
    return field;
}

static int validate_type_data(const char *type, const cJSON *data,
                              char *error, size_t error_size) {
    if (strcmp(type, "io.lightnvr.detection.object.v1") == 0) {
        const cJSON *count = required_field(data, "count", cJSON_Number, error,
                                            error_size);
        const cJSON *detections = required_field(
            data, "detections", cJSON_Array, error, error_size);
        if (!count || !detections) return -1;
        int count_value = count->valueint;
        if (count->valuedouble != (double)count_value || count_value <= 0 ||
            count_value > 1024 ||
            cJSON_GetArraySize(detections) != count_value) {
            set_error(error, error_size,
                      "detection count must match a non-empty detections array");
            return -1;
        }
        for (const cJSON *detection = detections->child; detection;
             detection = detection->next) {
            if (!cJSON_IsObject(detection)) {
                set_error(error, error_size,
                          "each detection must be a JSON object");
                return -1;
            }
            const cJSON *label = required_field(
                detection, "label", cJSON_String, error, error_size);
            const cJSON *confidence = required_field(
                detection, "confidence", cJSON_Number, error, error_size);
            if (!label || !confidence || !label->valuestring[0] ||
                confidence->valuedouble < 0 || confidence->valuedouble > 1) {
                set_error(error, error_size,
                          "detection values must use normalized ranges");
                return -1;
            }

            const cJSON *x = cJSON_GetObjectItemCaseSensitive(detection, "x");
            const cJSON *y = cJSON_GetObjectItemCaseSensitive(detection, "y");
            const cJSON *width = cJSON_GetObjectItemCaseSensitive(
                detection, "width");
            const cJSON *height = cJSON_GetObjectItemCaseSensitive(
                detection, "height");
            bool any_box = x || y || width || height;
            bool complete_box = cJSON_IsNumber(x) && cJSON_IsNumber(y) &&
                cJSON_IsNumber(width) && cJSON_IsNumber(height);
            if ((any_box && !complete_box) ||
                (complete_box &&
                 (x->valuedouble < 0 || x->valuedouble > 1 ||
                  y->valuedouble < 0 || y->valuedouble > 1 ||
                  width->valuedouble <= 0 || width->valuedouble > 1 ||
                  height->valuedouble <= 0 || height->valuedouble > 1))) {
                set_error(error, error_size,
                          "detection bounding boxes must be complete and normalized");
                return -1;
            }

            const cJSON *track_id = cJSON_GetObjectItemCaseSensitive(
                detection, "track_id");
            if (track_id &&
                (!cJSON_IsNumber(track_id) || track_id->valuedouble < 0 ||
                 track_id->valuedouble > INT_MAX ||
                 track_id->valuedouble != (double)track_id->valueint)) {
                set_error(error, error_size,
                          "detection track_id must be a non-negative integer");
                return -1;
            }
            const cJSON *zone_id = cJSON_GetObjectItemCaseSensitive(
                detection, "zone_id");
            if (zone_id &&
                (!cJSON_IsString(zone_id) || zone_id->valuestring[0] == '\0')) {
                set_error(error, error_size,
                          "detection zone_id must be a non-empty string");
                return -1;
            }
        }
    } else if (strcmp(type, "io.lightnvr.camera.offline.v1") == 0) {
        const cJSON *reason = required_field(data, "reason", cJSON_String,
                                             error, error_size);
        const cJSON *failures = required_field(
            data, "consecutive_failures", cJSON_Number, error, error_size);
        if (!reason || !failures) return -1;
        if (!reason->valuestring[0] || failures->valueint < 1) {
            set_error(error, error_size, "camera offline data is invalid");
            return -1;
        }
    } else if (strcmp(type, "io.lightnvr.stream.recording_gap.v1") == 0) {
        const cJSON *started = required_field(data, "started_at", cJSON_String,
                                              error, error_size);
        const cJSON *duration = required_field(
            data, "duration_ms", cJSON_Number, error, error_size);
        if (!started || !duration) return -1;
        if (!valid_event_time(started->valuestring) ||
            duration->valuedouble < 0) {
            set_error(error, error_size, "recording gap data is invalid");
            return -1;
        }
    } else if (strcmp(type, "io.lightnvr.storage.pressure.v1") == 0) {
        const cJSON *level = required_field(data, "level", cJSON_String, error,
                                            error_size);
        const cJSON *used = required_field(data, "used_percent", cJSON_Number,
                                           error, error_size);
        if (!level || !used) return -1;
        bool valid_level = strcmp(level->valuestring, "warning") == 0 ||
            strcmp(level->valuestring, "critical") == 0 ||
            strcmp(level->valuestring, "emergency") == 0;
        if (!valid_level || used->valuedouble < 0 ||
            used->valuedouble > 100) {
            set_error(error, error_size, "storage pressure data is invalid");
            return -1;
        }
    }
    return 0;
}

const event_type_definition_t *event_registry_all(int *count) {
    if (count) {
        *count = (int)(sizeof(EVENT_TYPES) / sizeof(EVENT_TYPES[0]));
    }
    return EVENT_TYPES;
}

const event_type_definition_t *event_registry_find(const char *type) {
    if (!type) return NULL;
    int count = 0;
    const event_type_definition_t *definitions = event_registry_all(&count);
    for (int index = 0; index < count; index++) {
        if (strcmp(definitions[index].type, type) == 0) {
            return &definitions[index];
        }
    }
    return NULL;
}

const char *event_severity_name(event_severity_t severity) {
    switch (severity) {
        case EVENT_SEVERITY_INFO: return "info";
        case EVENT_SEVERITY_WARNING: return "warning";
        case EVENT_SEVERITY_ERROR: return "error";
        case EVENT_SEVERITY_CRITICAL: return "critical";
    }
    return "unknown";
}

const char *event_sensitivity_name(event_sensitivity_t sensitivity) {
    switch (sensitivity) {
        case EVENT_SENSITIVITY_OPERATIONAL: return "operational";
        case EVENT_SENSITIVITY_INTERNAL: return "internal";
        case EVENT_SENSITIVITY_RESTRICTED: return "restricted";
    }
    return "unknown";
}

const char *event_media_policy_name(event_media_policy_t policy) {
    switch (policy) {
        case EVENT_MEDIA_FORBIDDEN: return "forbidden";
        case EVENT_MEDIA_REFERENCE_ALLOWED: return "reference_allowed";
        case EVENT_MEDIA_BINARY_OPT_IN: return "binary_opt_in";
    }
    return "unknown";
}

const char *event_expected_rate_name(event_expected_rate_t rate) {
    switch (rate) {
        case EVENT_RATE_LOW: return "low";
        case EVENT_RATE_MEDIUM: return "medium";
        case EVENT_RATE_HIGH: return "high";
    }
    return "unknown";
}

int event_envelope_validate(const event_envelope_t *event, char *error,
                            size_t error_size) {
    if (error && error_size > 0) error[0] = '\0';
    if (!event) {
        set_error(error, error_size, "event is required");
        return -1;
    }
    if (!valid_text(event->type, EVENT_TYPE_MAX, false)) {
        set_error(error, error_size, "event type is invalid");
        return -1;
    }
    const event_type_definition_t *definition = event_registry_find(event->type);
    if (!definition) {
        set_error(error, error_size, "event type is not registered");
        return -1;
    }
    if (!valid_text(event->specversion, sizeof(event->specversion), false) ||
        !valid_text(event->datacontenttype,
                    sizeof(event->datacontenttype), false) ||
        strcmp(event->specversion, "1.0") != 0 ||
        strcmp(event->datacontenttype, "application/json") != 0) {
        set_error(error, error_size, "event envelope version or content type is invalid");
        return -1;
    }
    if (!lightnvr_uuid_is_valid(event->id)) {
        set_error(error, error_size, "event id must be a UUID");
        return -1;
    }
    if (!valid_text(event->source, EVENT_SOURCE_MAX, false) ||
        strncmp(event->source, "urn:lightnvr:", 13) != 0 ||
        !lightnvr_uuid_is_valid(event->source + 13)) {
        set_error(error, error_size,
                  "event source must use urn:lightnvr:<installation-uuid>");
        return -1;
    }
    if (!valid_text(event->subject, EVENT_SUBJECT_MAX, false)) {
        set_error(error, error_size, "event subject is invalid");
        return -1;
    }
    if (definition->subject_kind == EVENT_SUBJECT_CAMERA) {
        if (strncmp(event->subject, "camera/", 7) != 0 ||
            !lightnvr_uuid_is_valid(event->subject + 7)) {
            set_error(error, error_size,
                      "camera event subject must use camera/<uuid>");
            return -1;
        }
    } else if (strcmp(event->subject, "system/storage") != 0) {
        set_error(error, error_size,
                  "storage event subject must use system/storage");
        return -1;
    }
    if (!valid_event_time(event->time) || event->occurred_at <= 0 ||
        event->expires_at !=
            event->occurred_at + definition->default_expiry_seconds) {
        set_error(error, error_size, "event timestamps are invalid");
        return -1;
    }
    if (!event->data || !cJSON_IsObject(event->data)) {
        set_error(error, error_size, "event data must be a JSON object");
        return -1;
    }
    if (validate_data_node(event->data, 0, error, error_size) != 0 ||
        validate_type_data(event->type, event->data, error, error_size) != 0) {
        return -1;
    }
    char *data_json = cJSON_PrintUnformatted(event->data);
    if (!data_json) {
        set_error(error, error_size, "event data could not be serialized");
        return -1;
    }
    size_t data_size = strlen(data_json);
    free(data_json);
    if (data_size > EVENT_DATA_MAX_BYTES) {
        set_error(error, error_size, "event data exceeds maximum size");
        return -1;
    }
    return 0;
}

int event_envelope_create(event_envelope_t *event, const char *type,
                          const char *source, const char *subject,
                          time_t occurred_at, const cJSON *data, char *error,
                          size_t error_size) {
    if (error && error_size > 0) error[0] = '\0';
    if (!event) {
        set_error(error, error_size, "event output is required");
        return -1;
    }
    memset(event, 0, sizeof(*event));
    if (!valid_text(type, EVENT_TYPE_MAX, false) ||
        !valid_text(source, EVENT_SOURCE_MAX, false) ||
        !valid_text(subject, EVENT_SUBJECT_MAX, false) || !data ||
        !cJSON_IsObject(data)) {
        set_error(error, error_size, "event input is invalid");
        return -1;
    }
    if (!event_registry_find(type)) {
        set_error(error, error_size, "event type is not registered");
        return -1;
    }
    snprintf(event->specversion, sizeof(event->specversion), "1.0");
    snprintf(event->type, sizeof(event->type), "%s", type);
    snprintf(event->source, sizeof(event->source), "%s", source);
    snprintf(event->subject, sizeof(event->subject), "%s", subject);
    snprintf(event->datacontenttype, sizeof(event->datacontenttype),
             "application/json");
    event->occurred_at = occurred_at > 0 ? occurred_at : time(NULL);
    const event_type_definition_t *definition = event_registry_find(type);
    event->expires_at = event->occurred_at + definition->default_expiry_seconds;
    if (lightnvr_uuid_generate_v4(event->id) != 0 ||
        format_event_time(event->occurred_at, event->time) != 0) {
        set_error(error, error_size, "event identity or timestamp generation failed");
        event_envelope_clear(event);
        return -1;
    }
    event->data = cJSON_Duplicate(data, true);
    if (!event->data) {
        set_error(error, error_size, "event data allocation failed");
        event_envelope_clear(event);
        return -1;
    }
    if (event_envelope_validate(event, error, error_size) != 0) {
        event_envelope_clear(event);
        return -1;
    }
    return 0;
}

char *event_envelope_serialize(const event_envelope_t *event, char *error,
                               size_t error_size) {
    if (event_envelope_validate(event, error, error_size) != 0) return NULL;
    const event_type_definition_t *definition =
        event_registry_find(event->type);
    cJSON *root = cJSON_CreateObject();
    if (!root) {
        set_error(error, error_size, "event envelope allocation failed");
        return NULL;
    }
    bool valid = cJSON_AddStringToObject(root, "specversion",
                                         event->specversion) &&
        cJSON_AddStringToObject(root, "id", event->id) &&
        cJSON_AddStringToObject(root, "type", event->type) &&
        cJSON_AddStringToObject(root, "source", event->source) &&
        cJSON_AddStringToObject(root, "subject", event->subject) &&
        cJSON_AddStringToObject(root, "time", event->time) &&
        cJSON_AddStringToObject(root, "datacontenttype",
                               event->datacontenttype) &&
        cJSON_AddStringToObject(root, "severity",
                               event_severity_name(definition->severity)) &&
        cJSON_AddStringToObject(
            root, "sensitivity",
            event_sensitivity_name(definition->sensitivity));
    cJSON *data_copy = valid ? cJSON_Duplicate(event->data, true) : NULL;
    if (!valid || !data_copy || !cJSON_AddItemToObject(root, "data", data_copy)) {
        cJSON_Delete(data_copy);
        cJSON_Delete(root);
        set_error(error, error_size, "event envelope allocation failed");
        return NULL;
    }
    char *serialized = cJSON_PrintUnformatted(root);
    cJSON_Delete(root);
    if (!serialized) {
        set_error(error, error_size, "event envelope serialization failed");
        return NULL;
    }
    if (strlen(serialized) > EVENT_ENVELOPE_MAX_BYTES) {
        free(serialized);
        set_error(error, error_size, "event envelope exceeds maximum size");
        return NULL;
    }
    return serialized;
}

int event_envelope_clone(event_envelope_t *destination,
                         const event_envelope_t *source, char *error,
                         size_t error_size) {
    if (error && error_size > 0) error[0] = '\0';
    if (!destination) {
        set_error(error, error_size, "event clone destination is required");
        return -1;
    }
    memset(destination, 0, sizeof(*destination));
    if (event_envelope_validate(source, error, error_size) != 0) return -1;
    *destination = *source;
    destination->data = cJSON_Duplicate(source->data, true);
    if (!destination->data) {
        memset(destination, 0, sizeof(*destination));
        set_error(error, error_size, "event clone allocation failed");
        return -1;
    }
    return 0;
}

void event_envelope_clear(event_envelope_t *event) {
    if (!event) return;
    cJSON_Delete(event->data);
    memset(event, 0, sizeof(*event));
}
