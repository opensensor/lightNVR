#ifndef LIGHTNVR_MQTT_DESTINATION_CLIENT_H
#define LIGHTNVR_MQTT_DESTINATION_CLIENT_H

#include <stdbool.h>

#include "database/db_event_destinations.h"

typedef struct mqtt_destination_client mqtt_destination_client_t;

/* Create a reconnecting MQTT client for one enabled managed profile. */
mqtt_destination_client_t *mqtt_destination_client_create(
    const event_destination_t *profile);

void mqtt_destination_client_destroy(mqtt_destination_client_t *client);
bool mqtt_destination_client_is_connected(
    mqtt_destination_client_t *client);

/* Publish with the profile QoS and wait for its corresponding acknowledgement. */
int mqtt_destination_client_publish_confirmed(
    mqtt_destination_client_t *client, const char *topic,
    const char *payload, bool retain, int timeout_ms);

#endif /* LIGHTNVR_MQTT_DESTINATION_CLIENT_H */
