#define _POSIX_C_SOURCE 200809L

#include "core/event_producers.h"

#include <stdbool.h>
#include <stdint.h>
#include <math.h>
#include <stdio.h>
#include <string.h>

#include <cjson/cJSON.h>

#include "core/config.h"
#include "core/event_bus.h"
#include "core/event_envelope.h"
#include "core/event_identity.h"
#include "core/event_router.h"
#include "core/mqtt_delivery_worker.h"
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
#define SYSTEM_HEALTH_ALERT_EVENT_TYPE \
    "io.lightnvr.system.health_alert.v1"
#define SYSTEM_HEALTH_RECOVERED_EVENT_TYPE \
    "io.lightnvr.system.health_recovered.v1"

static void set_error(char *error, size_t error_size, const char *message) {
    if (!error || error_size == 0) return;
    snprintf(error, error_size, "%s", message ? message : "event producer error");
}

static bool valid_text(const char *value, size_t maximum) {
    return value && value[0] != '\0' && strnlen(value, maximum) < maximum;
}

static int publish_event_with_severity_and_id(
    const char *type, const char *subject, time_t occurred_at,
    const cJSON *data, event_severity_t severity, const char *event_id,
    char *error, size_t error_size) {
    char source[EVENT_SOURCE_MAX];
    if (event_identity_get_source(source, sizeof(source)) != 0) {
        set_error(error, error_size, "event identity is not initialized");
        return -1;
    }

    event_envelope_t event;
    if (event_envelope_create_with_severity_and_id(
            &event, type, source, subject, occurred_at, data, severity,
            event_id, error, error_size) != 0) {
        return -1;
    }
    event_bus_result_t publish_result =
        event_bus_publish(&event, error, error_size);
    event_envelope_clear(&event);
    return publish_result == EVENT_BUS_OK ? 0 : -1;
}

static int publish_event(const char *type, const char *subject,
                         time_t occurred_at, const cJSON *data,
                         char *error, size_t error_size) {
    const event_type_definition_t *definition = event_registry_find(type);
    if (!definition) {
        set_error(error, error_size, "event type is not registered");
        return -1;
    }
    return publish_event_with_severity_and_id(
        type, subject, occurred_at, data, definition->severity, NULL, error,
        error_size);
}

static int durably_enqueue_event(const event_envelope_t *event,
                                 char *error, size_t error_size) {
    event_route_delivery_plan_t plan = {0};
    event_router_result_t route_result =
        event_router_evaluate_delivery(event, &plan);
    if (route_result == EVENT_ROUTER_ERROR) {
        event_route_delivery_plan_clear(&plan);
        set_error(error, error_size, "event route evaluation failed");
        return -1;
    }
    if (route_result == EVENT_ROUTER_DEFAULT) {
        if (!g_config.mqtt_enabled) {
            event_route_delivery_plan_clear(&plan);
            return 0;
        }
        event_outbox_enqueue_result_t result = mqtt_delivery_worker_enqueue(
            event, g_config.mqtt_topic_prefix, NULL);
        event_route_delivery_plan_clear(&plan);
        if (result == EVENT_OUTBOX_ENQUEUED ||
            result == EVENT_OUTBOX_DUPLICATE) {
            return 0;
        }
        set_error(error, error_size, "durable event outbox enqueue failed");
        return -1;
    }
    if (route_result == EVENT_ROUTER_NO_MATCH) {
        event_route_delivery_plan_clear(&plan);
        return 0;
    }

    bool failed = false;
    bool destination_accepted[EVENT_ROUTE_MAX_COUNT] = {false};
    for (size_t index = 0; index < plan.count; index++) {
        const event_route_delivery_plan_entry_t *entry = &plan.entries[index];
        bool duplicate_destination = false;
        for (size_t previous = 0; previous < index; previous++) {
            if (strcmp(plan.entries[previous].destination_key,
                       entry->destination_key) == 0) {
                duplicate_destination = true;
                break;
            }
        }
        if (duplicate_destination) continue;
        event_outbox_enqueue_result_t result;
        if (strcmp(entry->destination_key,
                   EVENT_ROUTE_DEFAULT_DESTINATION) == 0) {
            if (!g_config.mqtt_enabled) continue;
            result = mqtt_delivery_worker_enqueue(
                event, g_config.mqtt_topic_prefix, NULL);
        } else {
            result = mqtt_delivery_worker_enqueue_destination(
                event, entry->destination_key, entry->topic_template, NULL);
        }
        if (result != EVENT_OUTBOX_ENQUEUED &&
            result != EVENT_OUTBOX_DUPLICATE) {
            failed = true;
        } else {
            destination_accepted[index] = true;
        }
    }
    if (!failed) {
        for (size_t index = 0; index < plan.count; index++) {
            const event_route_delivery_plan_entry_t *entry =
                &plan.entries[index];
            bool duplicate_destination = false;
            for (size_t previous = 0; previous < index; previous++) {
                if (strcmp(plan.entries[previous].destination_key,
                           entry->destination_key) == 0) {
                    duplicate_destination = true;
                    break;
                }
            }
            if (!duplicate_destination && destination_accepted[index] &&
                event_router_record_destination_enqueued(
                    event, &plan, entry->destination_key) != 0) {
                failed = true;
            }
        }
    }
    event_route_delivery_plan_clear(&plan);
    if (failed) {
        set_error(error, error_size, "durable event route enqueue failed");
        return -1;
    }
    return 0;
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
    event_severity_t severity = strcmp(level, "warning") == 0
        ? EVENT_SEVERITY_WARNING : EVENT_SEVERITY_CRITICAL;
    int result = publish_event_with_severity_and_id(
        STORAGE_PRESSURE_EVENT_TYPE, "system/storage", occurred_at, data,
        severity, NULL, error, error_size);
    cJSON_Delete(data);
    return result;
}

static bool system_health_event_severity(
    system_health_severity_t health, event_severity_t *event) {
    if (!event) return false;
    switch (health) {
        case SYSTEM_HEALTH_SEVERITY_WARNING:
            *event = EVENT_SEVERITY_WARNING;
            return true;
        case SYSTEM_HEALTH_SEVERITY_ERROR:
            *event = EVENT_SEVERITY_ERROR;
            return true;
        case SYSTEM_HEALTH_SEVERITY_CRITICAL:
            *event = EVENT_SEVERITY_CRITICAL;
            return true;
        default:
            return false;
    }
}

static const char *system_health_event_subject(system_health_scope_t scope) {
    switch (scope) {
        case SYSTEM_HEALTH_SCOPE_PROCESS: return "system/process";
        case SYSTEM_HEALTH_SCOPE_CONTAINER: return "system/container";
        case SYSTEM_HEALTH_SCOPE_HOST: return "system/host";
        case SYSTEM_HEALTH_SCOPE_FILESYSTEM:
        case SYSTEM_HEALTH_SCOPE_DEVICE: return "system/storage";
        default: return NULL;
    }
}

static const char *system_health_event_state(
    system_health_incident_action_t action) {
    switch (action) {
        case SYSTEM_HEALTH_INCIDENT_OPEN:
        case SYSTEM_HEALTH_INCIDENT_ONE_SHOT: return "open";
        case SYSTEM_HEALTH_INCIDENT_ESCALATE: return "escalated";
        case SYSTEM_HEALTH_INCIDENT_MATERIAL_CHANGE: return "updated";
        case SYSTEM_HEALTH_INCIDENT_RECOVER: return "recovered";
    }
    return NULL;
}

static cJSON *system_health_observation_json(
    const system_health_observation_t *observation) {
    cJSON *result = cJSON_CreateObject();
    if (!result) return NULL;
    bool valid = false;
    if (observation->value_valid && isfinite(observation->value)) {
        valid = cJSON_AddNumberToObject(result, "value", observation->value) &&
            cJSON_AddStringToObject(result, "unit",
                                    system_health_unit_name(observation->unit));
    } else {
        valid = cJSON_AddStringToObject(
            result, "capability",
            system_health_capability_name(observation->capability));
    }
    if (!valid) {
        cJSON_Delete(result);
        return NULL;
    }
    return result;
}

static bool add_owned_json(cJSON *parent, const char *name, cJSON **item) {
    if (!parent || !name || !item || !*item ||
        !cJSON_AddItemToObject(parent, name, *item)) {
        return false;
    }
    *item = NULL;
    return true;
}

int event_producer_publish_system_health_transition(
    const system_health_transition_t *transition, const char *event_id,
    char *error, size_t error_size) {
    if (error && error_size > 0) error[0] = '\0';
    if (!transition || !transition->persisted ||
        !lightnvr_uuid_is_valid(event_id) ||
        !lightnvr_uuid_is_valid(transition->event_id) ||
        strcmp(transition->event_id, event_id) != 0 ||
        !lightnvr_uuid_is_valid(transition->incident_id) ||
        transition->condition < 0 ||
        transition->condition >= SYSTEM_HEALTH_CONDITION_COUNT ||
        !valid_text(transition->subject, SYSTEM_HEALTH_ID_LENGTH) ||
        transition->observed_at_ms <= 0) {
        set_error(error, error_size,
                  "persisted system health transition input is invalid");
        return -1;
    }
    const char *type = transition->action == SYSTEM_HEALTH_INCIDENT_RECOVER
        ? SYSTEM_HEALTH_RECOVERED_EVENT_TYPE : SYSTEM_HEALTH_ALERT_EVENT_TYPE;
    const char *subject = system_health_event_subject(transition->scope);
    const char *state = system_health_event_state(transition->action);
    const char *code = system_health_condition_code(transition->condition);
    event_severity_t severity = EVENT_SEVERITY_INFO;
    system_health_severity_t payload_severity =
        transition->action == SYSTEM_HEALTH_INCIDENT_RECOVER
            ? transition->previous_severity : transition->severity;
    if (!subject || !state || !code ||
        !system_health_event_severity(payload_severity, &severity)) {
        set_error(error, error_size,
                  "system health transition classification is invalid");
        return -1;
    }

    cJSON *data = cJSON_CreateObject();
    cJSON *observation = system_health_observation_json(
        &transition->observation);
    bool valid = data && observation &&
        cJSON_AddStringToObject(data, "incident_id",
                               transition->incident_id) &&
        cJSON_AddStringToObject(data, "code", code) &&
        cJSON_AddStringToObject(data, "scope",
                               system_health_scope_name(transition->scope)) &&
        cJSON_AddStringToObject(data, "resource", transition->subject) &&
        cJSON_AddStringToObject(data, "state", state);
    if (!valid) {
        cJSON_Delete(observation);
        cJSON_Delete(data);
        set_error(error, error_size,
                  "system health event allocation failed");
        return -1;
    }
    if (transition->action == SYSTEM_HEALTH_INCIDENT_RECOVER) {
        valid = cJSON_AddStringToObject(
                    data, "previous_severity",
                    system_health_severity_name(payload_severity)) &&
            cJSON_AddNumberToObject(data, "duration_ms",
                                    (double)transition->incident_duration_ms) &&
            add_owned_json(data, "safe_observation", &observation);
    } else {
        cJSON *threshold = cJSON_CreateObject();
        const char *operator_name =
            transition->threshold_direction ==
                    SYSTEM_HEALTH_THRESHOLD_LOWER_IS_WORSE
                ? "lt"
                : transition->threshold_direction ==
                          SYSTEM_HEALTH_THRESHOLD_HIGHER_IS_WORSE
                      ? "gt" : "event";
        char first_observed_at[EVENT_TIME_MAX];
        time_t first_observed =
            (time_t)(transition->first_observed_at_ms / 1000);
        bool threshold_valid = threshold &&
            isfinite(transition->threshold_value) &&
            format_event_time(first_observed, first_observed_at) == 0 &&
            cJSON_AddStringToObject(threshold, "operator", operator_name) &&
            cJSON_AddNumberToObject(threshold, "value",
                                    transition->threshold_value) &&
            cJSON_AddNumberToObject(threshold, "for_ms",
                                    transition->threshold_for_ms);
        if (!threshold_valid) {
            cJSON_Delete(threshold);
            valid = false;
        } else {
            valid = cJSON_AddStringToObject(
                        data, "severity",
                        system_health_severity_name(payload_severity)) &&
                add_owned_json(data, "observed", &observation) &&
                add_owned_json(data, "threshold", &threshold) &&
                cJSON_AddStringToObject(data, "first_observed_at",
                                        first_observed_at);
            cJSON_Delete(threshold);
        }
    }
    if (!valid) {
        cJSON_Delete(observation);
        cJSON_Delete(data);
        set_error(error, error_size,
                  "system health event allocation failed");
        return -1;
    }
    if (transition->action == SYSTEM_HEALTH_INCIDENT_RECOVER) {
        severity = EVENT_SEVERITY_INFO;
    }
    time_t occurred_at = (time_t)(transition->observed_at_ms / 1000);
    char source[EVENT_SOURCE_MAX];
    event_envelope_t event;
    int result = -1;
    if (event_identity_get_source(source, sizeof(source)) != 0) {
        set_error(error, error_size, "event identity is not initialized");
    } else if (event_envelope_create_with_severity_and_id(
                   &event, type, source, subject, occurred_at, data, severity,
                   event_id, error, error_size) == 0) {
        result = durably_enqueue_event(&event, error, error_size);
        if (result == 0) {
            /* The durable route is authoritative. The bus handoff notifies
             * other in-process consumers and may harmlessly hit outbox
             * duplicate detection in the MQTT compatibility adapter. */
            (void)event_bus_publish(&event, NULL, 0);
        }
        event_envelope_clear(&event);
    }
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
