#ifndef LIGHTNVR_CORE_MQTT_EVENT_ADAPTER_H
#define LIGHTNVR_CORE_MQTT_EVENT_ADAPTER_H

#include <stddef.h>

#include "core/event_envelope.h"
#include "video/detection_result.h"

/* Register/unregister the MQTT compatibility subscriber around bus lifetime. */
int mqtt_event_adapter_register(void);
int mqtt_event_adapter_unregister(void);

/* Decode normalized detection data for the legacy MQTT/HA compatibility path. */
int mqtt_event_adapter_decode_detection(
    const event_envelope_t *event, char *stream_name,
    size_t stream_name_size, detection_result_t *result);

#endif /* LIGHTNVR_CORE_MQTT_EVENT_ADAPTER_H */
