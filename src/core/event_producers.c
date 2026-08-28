#define _POSIX_C_SOURCE 200809L

#include "core/event_producers.h"

#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

#include <cjson/cJSON.h>

#include "core/config.h"
#include "core/event_bus.h"
#include "core/event_envelope.h"
#include "core/event_identity.h"
#include "database/db_streams.h"
#include "utils/uuid.h"

#define DETECTION_EVENT_TYPE "io.lightnvr.detection.object.v1"
#define LPR_READ_EVENT_TYPE "io.lightnvr.recognition.license_plate.v1"
#define CAMERA_OFFLINE_EVENT_TYPE "io.lightnvr.camera.offline.v1"
#define CAMERA_RECOVERED_EVENT_TYPE "io.lightnvr.camera.recovered.v1"
#define STREAM_DEGRADED_EVENT_TYPE "io.lightnvr.stream.degraded.v1"
#define STREAM_RECOVERED_EVENT_TYPE "io.lightnvr.stream.recovered.v1"
#define RECORDING_GAP_EVENT_TYPE "io.lightnvr.stream.recording_gap.v1"
#define STORAGE_PRESSURE_EVENT_TYPE "io.lightnvr.storage.pressure.v1"
#define STORAGE_RECOVERED_EVENT_TYPE "io.lightnvr.storage.recovered.v1"
#define STORAGE_TARGET_UNAVAILABLE_EVENT_TYPE \
    "io.lightnvr.storage.target_unavailable.v1"
#define STORAGE_TARGET_RECOVERED_EVENT_TYPE \
    "io.lightnvr.storage.target_recovered.v1"

static void set_error(char *error, size_t error_size, const char *message) {
    if (!error || error_size == 0) return;
    snprintf(error, error_size, "%s", message ? message : "event producer error");
}

static bool valid_text(const char *value, size_t maximum) {
    return value && value[0] != '\0' && strnlen(value, maximum) < maximum;
}

static int publish_event(const char *type, const char *subject,
                         time_t occurred_at, const cJSON *data,
                         char *error, size_t error_size) {
    char source[EVENT_SOURCE_MAX];
    if (event_identity_get_source(source, sizeof(source)) != 0) {
        set_error(error, error_size, "event identity is not initialized");
        return -1;
    }

    event_envelope_t event;
    if (event_envelope_create(&event, type, source, subject, occurred_at, data,
                              error, error_size) != 0) {
        return -1;
    }
    event_bus_result_t publish_result =
        event_bus_publish(&event, error, error_size);
    event_envelope_clear(&event);
    return publish_result == EVENT_BUS_OK ? 0 : -1;
}

static int camera_subject_for_stream(
    const char *stream_name, char subject[EVENT_SUBJECT_MAX],
    char *error, size_t error_size) {
    if (!valid_text(stream_name, MAX_STREAM_NAME)) {
        set_error(error, error_size, "stream name is required");
        return -1;
    }
    stream_config_t stream;
    if (get_stream_config_by_name(stream_name, &stream) != 0 ||
        !lightnvr_uuid_is_valid(stream.camera_uuid)) {
        set_error(error, error_size, "stream camera identity is unavailable");
        return -1;
    }
    int written = snprintf(subject, EVENT_SUBJECT_MAX, "camera/%s",
                           stream.camera_uuid);
    if (written < 0 || written >= EVENT_SUBJECT_MAX) {
        set_error(error, error_size, "camera event subject is too long");
        return -1;
    }
    return 0;
}

static int publish_camera_event_for_stream(
    const char *type, const char *stream_name, time_t occurred_at,
    const cJSON *data, char *error, size_t error_size) {
    char subject[EVENT_SUBJECT_MAX];
    if (camera_subject_for_stream(stream_name, subject, error, error_size) != 0) {
        return -1;
    }
    return publish_event(type, subject, occurred_at, data, error, error_size);
}

static int format_event_time(time_t value, char output[EVENT_TIME_MAX]) {
    struct tm utc;
    if (value <= 0 || !gmtime_r(&value, &utc)) return -1;
    return strftime(output, EVENT_TIME_MAX, "%Y-%m-%dT%H:%M:%SZ", &utc) > 0
        ? 0
        : -1;
}

static bool valid_pressure_level(const char *level) {
    return level && (strcmp(level, "warning") == 0 ||
                     strcmp(level, "critical") == 0 ||
                     strcmp(level, "emergency") == 0);
}

static bool valid_storage_target_previous_state(const char *state) {
    return state && (strcmp(state, "unknown") == 0 ||
                     strcmp(state, "healthy") == 0 ||
                     strcmp(state, "degraded") == 0);
}

static bool valid_storage_target_current_state(const char *state) {
    return state && (strcmp(state, "healthy") == 0 ||
                     strcmp(state, "degraded") == 0);
}

static bool valid_storage_target_reason(const char *reason) {
    return reason && (strcmp(reason, "mount_unavailable") == 0 ||
                      strcmp(reason, "directory_unavailable") == 0 ||
                      strcmp(reason, "capacity_probe_failed") == 0 ||
                      strcmp(reason, "not_writable") == 0 ||
                      strcmp(reason, "write_probe_failed") == 0 ||
                      strcmp(reason, "probe_cleanup_failed") == 0 ||
                      strcmp(reason, "unknown") == 0);
}

static bool add_detection_data(cJSON *data, const char *stream_name,
                               const detection_result_t *result) {
    if (!cJSON_AddStringToObject(data, "stream_name", stream_name) ||
        !cJSON_AddNumberToObject(data, "count", result->count)) {
        return false;
    }
    cJSON *detections = cJSON_AddArrayToObject(data, "detections");
    if (!detections) return false;

    for (int index = 0; index < result->count; index++) {
        const detection_t *source = &result->detections[index];
        cJSON *detection = cJSON_CreateObject();
        if (!detection ||
            !cJSON_AddStringToObject(detection, "label", source->label) ||
            !cJSON_AddNumberToObject(detection, "confidence",
                                    source->confidence)) {
            cJSON_Delete(detection);
            return false;
        }
        if (source->width > 0.0f && source->height > 0.0f) {
            if (!cJSON_AddNumberToObject(detection, "x", source->x) ||
                !cJSON_AddNumberToObject(detection, "y", source->y) ||
                !cJSON_AddNumberToObject(detection, "width", source->width) ||
                !cJSON_AddNumberToObject(detection, "height", source->height)) {
                cJSON_Delete(detection);
                return false;
            }
        }
        if (source->track_id >= 0 &&
            !cJSON_AddNumberToObject(detection, "track_id", source->track_id)) {
            cJSON_Delete(detection);
            return false;
        }
        if (source->zone_id[0] != '\0' &&
            !cJSON_AddStringToObject(detection, "zone_id", source->zone_id)) {
            cJSON_Delete(detection);
            return false;
        }
        if (!cJSON_AddItemToArray(detections, detection)) {
            cJSON_Delete(detection);
            return false;
        }
    }
    return true;
}

int event_producer_publish_detection(
    const char *camera_uuid, const char *stream_name,
    const detection_result_t *result, time_t occurred_at,
    char *error, size_t error_size) {
    if (error && error_size > 0) error[0] = '\0';
    if (!lightnvr_uuid_is_valid(camera_uuid) || !stream_name ||
        stream_name[0] == '\0' || strlen(stream_name) >= MAX_STREAM_NAME ||
        !result || result->count <= 0 || result->count > MAX_DETECTIONS) {
        set_error(error, error_size, "detection event input is invalid");
        return -1;
    }

    char subject[EVENT_SUBJECT_MAX];
    int subject_length = snprintf(subject, sizeof(subject), "camera/%s",
                                  camera_uuid);
    if (subject_length < 0 || (size_t)subject_length >= sizeof(subject)) {
        set_error(error, error_size, "camera event subject is too long");
        return -1;
    }

    cJSON *data = cJSON_CreateObject();
    if (!data || !add_detection_data(data, stream_name, result)) {
        cJSON_Delete(data);
        set_error(error, error_size, "detection event allocation failed");
        return -1;
    }

    int create_result = publish_event(
        DETECTION_EVENT_TYPE, subject, occurred_at, data, error, error_size);
    cJSON_Delete(data);
    return create_result;
}

int event_producer_publish_detection_for_stream(
    const char *stream_name, const detection_result_t *result,
    time_t occurred_at, char *error, size_t error_size) {
    if (!stream_name || stream_name[0] == '\0') {
        set_error(error, error_size, "stream name is required");
        return -1;
    }
    stream_config_t stream;
    if (get_stream_config_by_name(stream_name, &stream) != 0 ||
        !lightnvr_uuid_is_valid(stream.camera_uuid)) {
        set_error(error, error_size, "stream has no immutable camera identity");
        return -1;
    }
    return event_producer_publish_detection(
        stream.camera_uuid, stream_name, result, occurred_at, error,
        error_size);
}

int event_producer_publish_lpr_read(
    const char *camera_uuid, const char *stream_name, const char *read_uuid,
    const char *source, time_t occurred_at, char *error, size_t error_size) {
    if (error && error_size > 0) error[0] = '\0';
    if (!lightnvr_uuid_is_valid(camera_uuid) ||
        !lightnvr_uuid_is_valid(read_uuid) ||
        !valid_text(stream_name, MAX_STREAM_NAME) ||
        !valid_text(source, 64)) {
        set_error(error, error_size, "protected LPR event input is invalid");
        return -1;
    }

    char subject[EVENT_SUBJECT_MAX];
    int subject_length = snprintf(subject, sizeof(subject), "camera/%s",
                                  camera_uuid);
    if (subject_length < 0 || (size_t)subject_length >= sizeof(subject)) {
        set_error(error, error_size, "camera event subject is too long");
        return -1;
    }

    cJSON *data = cJSON_CreateObject();
    if (!data || !cJSON_AddStringToObject(data, "read_id", read_uuid) ||
        !cJSON_AddStringToObject(data, "stream_name", stream_name) ||
        !cJSON_AddStringToObject(data, "source", source)) {
        cJSON_Delete(data);
        set_error(error, error_size,
                  "protected LPR event allocation failed");
        return -1;
    }

    int result = publish_event(LPR_READ_EVENT_TYPE, subject, occurred_at, data,
                               error, error_size);
    cJSON_Delete(data);
    return result;
}

int event_producer_publish_camera_offline_for_stream(
    const char *stream_name, const char *reason, int consecutive_failures,
    time_t occurred_at, char *error, size_t error_size) {
    if (error && error_size > 0) error[0] = '\0';
    if (!valid_text(stream_name, MAX_STREAM_NAME) ||
        !valid_text(reason, 128) || consecutive_failures < 1) {
        set_error(error, error_size, "camera offline event input is invalid");
        return -1;
    }
    cJSON *data = cJSON_CreateObject();
    if (!data || !cJSON_AddStringToObject(data, "stream_name", stream_name) ||
        !cJSON_AddStringToObject(data, "reason", reason) ||
        !cJSON_AddNumberToObject(data, "consecutive_failures",
                                consecutive_failures)) {
        cJSON_Delete(data);
        set_error(error, error_size, "camera offline event allocation failed");
        return -1;
    }
    int result = publish_camera_event_for_stream(
        CAMERA_OFFLINE_EVENT_TYPE, stream_name, occurred_at, data,
        error, error_size);
    cJSON_Delete(data);
    return result;
}

int event_producer_publish_camera_recovered_for_stream(
    const char *stream_name, int64_t downtime_ms, time_t occurred_at,
    char *error, size_t error_size) {
    if (error && error_size > 0) error[0] = '\0';
    if (!valid_text(stream_name, MAX_STREAM_NAME) || downtime_ms < 0) {
        set_error(error, error_size, "camera recovered event input is invalid");
        return -1;
    }
    cJSON *data = cJSON_CreateObject();
    if (!data || !cJSON_AddStringToObject(data, "stream_name", stream_name) ||
        !cJSON_AddStringToObject(data, "previous_state", "offline") ||
        !cJSON_AddNumberToObject(data, "downtime_ms", (double)downtime_ms)) {
        cJSON_Delete(data);
        set_error(error, error_size, "camera recovered event allocation failed");
        return -1;
    }
    int result = publish_camera_event_for_stream(
        CAMERA_RECOVERED_EVENT_TYPE, stream_name, occurred_at, data,
        error, error_size);
    cJSON_Delete(data);
    return result;
}

int event_producer_publish_stream_degraded_for_stream(
    const char *stream_name, const char *reason, double observed_fps,
    double expected_fps, time_t occurred_at, char *error, size_t error_size) {
    if (error && error_size > 0) error[0] = '\0';
    if (!valid_text(stream_name, MAX_STREAM_NAME) ||
        !valid_text(reason, 128) || observed_fps < 0.0 || expected_fps < 0.0) {
        set_error(error, error_size, "stream degraded event input is invalid");
        return -1;
    }
    cJSON *data = cJSON_CreateObject();
    if (!data || !cJSON_AddStringToObject(data, "stream_name", stream_name) ||
        !cJSON_AddStringToObject(data, "reason", reason) ||
        !cJSON_AddNumberToObject(data, "observed_fps", observed_fps) ||
        !cJSON_AddNumberToObject(data, "expected_fps", expected_fps)) {
        cJSON_Delete(data);
        set_error(error, error_size, "stream degraded event allocation failed");
        return -1;
    }
    int result = publish_camera_event_for_stream(
        STREAM_DEGRADED_EVENT_TYPE, stream_name, occurred_at, data,
        error, error_size);
    cJSON_Delete(data);
    return result;
}

int event_producer_publish_stream_recovered_for_stream(
    const char *stream_name, double observed_fps, double expected_fps,
    time_t occurred_at, char *error, size_t error_size) {
    if (error && error_size > 0) error[0] = '\0';
    if (!valid_text(stream_name, MAX_STREAM_NAME) || observed_fps < 0.0 ||
        expected_fps < 0.0) {
        set_error(error, error_size, "stream recovered event input is invalid");
        return -1;
    }
    cJSON *data = cJSON_CreateObject();
    if (!data || !cJSON_AddStringToObject(data, "stream_name", stream_name) ||
        !cJSON_AddStringToObject(data, "previous_state", "degraded") ||
        !cJSON_AddNumberToObject(data, "observed_fps", observed_fps) ||
        !cJSON_AddNumberToObject(data, "expected_fps", expected_fps)) {
        cJSON_Delete(data);
        set_error(error, error_size, "stream recovered event allocation failed");
        return -1;
    }
    int result = publish_camera_event_for_stream(
        STREAM_RECOVERED_EVENT_TYPE, stream_name, occurred_at, data,
        error, error_size);
    cJSON_Delete(data);
    return result;
}

int event_producer_publish_recording_gap_for_stream(
    const char *stream_name, time_t started_at, int64_t duration_ms,
    time_t occurred_at, char *error, size_t error_size) {
    if (error && error_size > 0) error[0] = '\0';
    char started_at_text[EVENT_TIME_MAX];
    if (!valid_text(stream_name, MAX_STREAM_NAME) || duration_ms < 0 ||
        format_event_time(started_at, started_at_text) != 0) {
        set_error(error, error_size, "recording gap event input is invalid");
        return -1;
    }
    cJSON *data = cJSON_CreateObject();
    if (!data || !cJSON_AddStringToObject(data, "stream_name", stream_name) ||
        !cJSON_AddStringToObject(data, "started_at", started_at_text) ||
        !cJSON_AddNumberToObject(data, "duration_ms", (double)duration_ms)) {
        cJSON_Delete(data);
        set_error(error, error_size, "recording gap event allocation failed");
        return -1;
    }
    int result = publish_camera_event_for_stream(
        RECORDING_GAP_EVENT_TYPE, stream_name, occurred_at, data,
        error, error_size);
    cJSON_Delete(data);
    return result;
}

int event_producer_publish_storage_pressure(
    const char *level, const char *previous_level, double used_percent,
    uint64_t free_bytes, time_t occurred_at, char *error, size_t error_size) {
    if (error && error_size > 0) error[0] = '\0';
    if (!valid_pressure_level(level) ||
        !valid_text(previous_level, 32) || used_percent < 0.0 ||
        used_percent > 100.0) {
        set_error(error, error_size, "storage pressure event input is invalid");
        return -1;
    }
    cJSON *data = cJSON_CreateObject();
    if (!data || !cJSON_AddStringToObject(data, "level", level) ||
        !cJSON_AddStringToObject(data, "previous_level", previous_level) ||
        !cJSON_AddNumberToObject(data, "used_percent", used_percent) ||
        !cJSON_AddNumberToObject(data, "free_bytes", (double)free_bytes)) {
        cJSON_Delete(data);
        set_error(error, error_size, "storage pressure event allocation failed");
        return -1;
    }
    int result = publish_event(
        STORAGE_PRESSURE_EVENT_TYPE, "system/storage", occurred_at, data,
        error, error_size);
    cJSON_Delete(data);
    return result;
}

int event_producer_publish_storage_recovered(
    const char *previous_level, double used_percent, uint64_t free_bytes,
    time_t occurred_at, char *error, size_t error_size) {
    if (error && error_size > 0) error[0] = '\0';
    if (!valid_pressure_level(previous_level) || used_percent < 0.0 ||
        used_percent > 100.0) {
        set_error(error, error_size, "storage recovered event input is invalid");
        return -1;
    }
    cJSON *data = cJSON_CreateObject();
    if (!data ||
        !cJSON_AddStringToObject(data, "previous_level", previous_level) ||
        !cJSON_AddNumberToObject(data, "used_percent", used_percent) ||
        !cJSON_AddNumberToObject(data, "free_bytes", (double)free_bytes)) {
        cJSON_Delete(data);
        set_error(error, error_size, "storage recovered event allocation failed");
        return -1;
    }
    int result = publish_event(
        STORAGE_RECOVERED_EVENT_TYPE, "system/storage", occurred_at, data,
        error, error_size);
    cJSON_Delete(data);
    return result;
}

int event_producer_publish_storage_target_unavailable(
    const char *target_uuid, const char *previous_state, const char *reason,
    bool is_default, time_t occurred_at, char *error, size_t error_size) {
    if (error && error_size > 0) error[0] = '\0';
    if (!lightnvr_uuid_is_valid(target_uuid) ||
        !valid_storage_target_previous_state(previous_state) ||
        !valid_storage_target_reason(reason)) {
        set_error(error, error_size,
                  "storage target unavailable event input is invalid");
        return -1;
    }
    cJSON *data = cJSON_CreateObject();
    if (!data || !cJSON_AddStringToObject(data, "target_uuid", target_uuid) ||
        !cJSON_AddStringToObject(data, "previous_state", previous_state) ||
        !cJSON_AddStringToObject(data, "reason", reason) ||
        !cJSON_AddBoolToObject(data, "is_default", is_default)) {
        cJSON_Delete(data);
        set_error(error, error_size,
                  "storage target unavailable event allocation failed");
        return -1;
    }
    int result = publish_event(
        STORAGE_TARGET_UNAVAILABLE_EVENT_TYPE, "system/storage", occurred_at,
        data, error, error_size);
    cJSON_Delete(data);
    return result;
}

int event_producer_publish_storage_target_recovered(
    const char *target_uuid, const char *current_state, int64_t downtime_ms,
    bool is_default, time_t occurred_at, char *error, size_t error_size) {
    if (error && error_size > 0) error[0] = '\0';
    if (!lightnvr_uuid_is_valid(target_uuid) ||
        !valid_storage_target_current_state(current_state) ||
        downtime_ms < 0) {
        set_error(error, error_size,
                  "storage target recovered event input is invalid");
        return -1;
    }
    cJSON *data = cJSON_CreateObject();
    if (!data || !cJSON_AddStringToObject(data, "target_uuid", target_uuid) ||
        !cJSON_AddStringToObject(data, "previous_state", "unavailable") ||
        !cJSON_AddStringToObject(data, "current_state", current_state) ||
        !cJSON_AddNumberToObject(data, "downtime_ms", (double)downtime_ms) ||
        !cJSON_AddBoolToObject(data, "is_default", is_default)) {
        cJSON_Delete(data);
        set_error(error, error_size,
                  "storage target recovered event allocation failed");
        return -1;
    }
    int result = publish_event(
        STORAGE_TARGET_RECOVERED_EVENT_TYPE, "system/storage", occurred_at,
        data, error, error_size);
    cJSON_Delete(data);
    return result;
}
