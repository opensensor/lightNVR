#ifndef LIGHTNVR_MQTT_DESTINATION_CLIENT_H
#define LIGHTNVR_MQTT_DESTINATION_CLIENT_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include "database/db_event_destinations.h"

typedef struct mqtt_destination_client mqtt_destination_client_t;

typedef enum {
    MQTT_DESTINATION_FAILURE_NONE = 0,
    MQTT_DESTINATION_FAILURE_CONFIGURATION,
    MQTT_DESTINATION_FAILURE_CONNECTION,
    MQTT_DESTINATION_FAILURE_PUBLICATION,
    MQTT_DESTINATION_FAILURE_COUNT
} mqtt_destination_failure_t;

typedef struct {
    char destination_uuid[EVENT_DESTINATION_UUID_MAX];
    bool presence_configured;
    bool connected;
    uint64_t connections;
    uint64_t reconnects;
    uint64_t disconnects;
    uint64_t connection_failures;
    uint64_t publish_successes;
    uint64_t publish_failures;
    int64_t last_success_at_ms;
    int64_t last_failure_at_ms;
    mqtt_destination_failure_t last_failure;
} mqtt_destination_client_stats_t;

/* Create a reconnecting MQTT client for one enabled managed profile. */
mqtt_destination_client_t *mqtt_destination_client_create(
    const event_destination_t *profile);

void mqtt_destination_client_destroy(mqtt_destination_client_t *client);
bool mqtt_destination_client_is_connected(
    mqtt_destination_client_t *client);

/** Copy one client's bounded, credential-free runtime statistics. */
bool mqtt_destination_client_get_stats(
    mqtt_destination_client_t *client,
    mqtt_destination_client_stats_t *stats);

/** Copy active managed-client stats, ordered by destination UUID. */
size_t mqtt_destination_client_list_stats(
    mqtt_destination_client_stats_t *stats, size_t capacity);

/* Publish with the profile QoS and wait for its corresponding acknowledgement. */
int mqtt_destination_client_publish_confirmed(
    mqtt_destination_client_t *client, const char *topic,
    const char *payload, bool retain, int timeout_ms);

#endif /* LIGHTNVR_MQTT_DESTINATION_CLIENT_H */
