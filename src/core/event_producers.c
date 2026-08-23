#define _POSIX_C_SOURCE 200809L

#include "core/event_producers.h"

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

static void set_error(char *error, size_t error_size, const char *message) {
    if (!error || error_size == 0) return;
    snprintf(error, error_size, "%s", message ? message : "event producer error");
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

    char source[EVENT_SOURCE_MAX];
    char subject[EVENT_SUBJECT_MAX];
    if (event_identity_get_source(source, sizeof(source)) != 0) {
        set_error(error, error_size, "event identity is not initialized");
        return -1;
    }
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

    event_envelope_t event;
    int create_result = event_envelope_create(
        &event, DETECTION_EVENT_TYPE, source, subject, occurred_at, data,
        error, error_size);
    cJSON_Delete(data);
    if (create_result != 0) return -1;

    event_bus_result_t publish_result =
        event_bus_publish(&event, error, error_size);
    event_envelope_clear(&event);
    return publish_result == EVENT_BUS_OK ? 0 : -1;
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
