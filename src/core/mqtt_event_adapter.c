#define _POSIX_C_SOURCE 200809L

#include "core/mqtt_event_adapter.h"

#include <limits.h>
#include <string.h>

#include <cjson/cJSON.h>

#include "core/config.h"
#include "core/event_bus.h"
#include "core/logger.h"
#include "core/mqtt_client.h"
#include "core/mqtt_delivery_worker.h"
#include "utils/strings.h"

#define MQTT_EVENT_SUBSCRIBER "mqtt-compatibility"
#define DETECTION_EVENT_TYPE "io.lightnvr.detection.object.v1"

int mqtt_event_adapter_decode_detection(
    const event_envelope_t *event, char *stream_name,
    size_t stream_name_size, detection_result_t *result) {
    if (!event || strcmp(event->type, DETECTION_EVENT_TYPE) != 0 ||
        !stream_name || stream_name_size == 0 || !result ||
        event_envelope_validate(event, NULL, 0) != 0) {
        return -1;
    }
    const cJSON *name = cJSON_GetObjectItemCaseSensitive(
        event->data, "stream_name");
    const cJSON *count = cJSON_GetObjectItemCaseSensitive(event->data, "count");
    const cJSON *detections = cJSON_GetObjectItemCaseSensitive(
        event->data, "detections");
    if (!cJSON_IsString(name) || name->valuestring[0] == '\0' ||
        strlen(name->valuestring) >= stream_name_size ||
        !cJSON_IsNumber(count) || count->valuedouble != count->valueint ||
        count->valueint <= 0 || count->valueint > MAX_DETECTIONS ||
        !cJSON_IsArray(detections) ||
        cJSON_GetArraySize(detections) != count->valueint) {
        return -1;
    }

    memset(result, 0, sizeof(*result));
    safe_strcpy(stream_name, name->valuestring, stream_name_size, 0);
    const cJSON *item = NULL;
    cJSON_ArrayForEach(item, detections) {
        detection_t *destination = &result->detections[result->count];
        const cJSON *label = cJSON_GetObjectItemCaseSensitive(item, "label");
        const cJSON *confidence = cJSON_GetObjectItemCaseSensitive(
            item, "confidence");
        if (!cJSON_IsString(label) || !cJSON_IsNumber(confidence)) return -1;
        safe_strcpy(destination->label, label->valuestring,
                    sizeof(destination->label), 0);
        destination->confidence = (float)confidence->valuedouble;
        destination->track_id = -1;

        const cJSON *x = cJSON_GetObjectItemCaseSensitive(item, "x");
        const cJSON *y = cJSON_GetObjectItemCaseSensitive(item, "y");
        const cJSON *width = cJSON_GetObjectItemCaseSensitive(item, "width");
        const cJSON *height = cJSON_GetObjectItemCaseSensitive(item, "height");
        if (x) destination->x = (float)x->valuedouble;
        if (y) destination->y = (float)y->valuedouble;
        if (width) destination->width = (float)width->valuedouble;
        if (height) destination->height = (float)height->valuedouble;

        const cJSON *track_id = cJSON_GetObjectItemCaseSensitive(
            item, "track_id");
        if (cJSON_IsNumber(track_id) && track_id->valuedouble >= 0 &&
            track_id->valuedouble <= INT_MAX) {
            destination->track_id = track_id->valueint;
        }
        const cJSON *zone_id = cJSON_GetObjectItemCaseSensitive(item, "zone_id");
        if (cJSON_IsString(zone_id)) {
            safe_strcpy(destination->zone_id, zone_id->valuestring,
                        sizeof(destination->zone_id), 0);
        }
        result->count++;
    }
    return 0;
}

static int mqtt_event_handler(const event_envelope_t *event, void *context) {
    const config_t *config = context;
    (void)config;
    int failed = 0;

#ifdef ENABLE_MQTT
    if (config && config->mqtt_enabled) {
        event_outbox_enqueue_result_t enqueue_result =
            mqtt_delivery_worker_enqueue(
                event, config->mqtt_topic_prefix, NULL);
        if (enqueue_result != EVENT_OUTBOX_ENQUEUED &&
            enqueue_result != EVENT_OUTBOX_DUPLICATE) {
            log_error("MQTT event adapter could not persist event %s (%d)",
                      event->id, enqueue_result);
            failed = 1;
        }
    }
#endif

    if (strcmp(event->type, DETECTION_EVENT_TYPE) == 0) {
        char stream_name[MAX_STREAM_NAME];
        detection_result_t result;
        if (mqtt_event_adapter_decode_detection(
                event, stream_name, sizeof(stream_name), &result) != 0) {
            log_error("MQTT event adapter rejected detection event %s", event->id);
            return -1;
        }
        if (mqtt_publish_detection(stream_name, &result,
                                   event->occurred_at) != 0) {
            failed = 1;
        }
        mqtt_set_motion_state(stream_name, &result);
    }
    return failed ? -1 : 0;
}

int mqtt_event_adapter_register(const config_t *config) {
    if (!config) return -1;
    return event_bus_subscribe(MQTT_EVENT_SUBSCRIBER,
                               mqtt_event_handler, (void *)config);
}

int mqtt_event_adapter_unregister(void) {
    return event_bus_unsubscribe(MQTT_EVENT_SUBSCRIBER);
}
