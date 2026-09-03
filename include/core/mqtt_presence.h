/** @file mqtt_presence.h Bounded MQTT installation-presence contract. */

#ifndef LIGHTNVR_MQTT_PRESENCE_H
#define LIGHTNVR_MQTT_PRESENCE_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#define MQTT_PRESENCE_TOPIC_MAX 512U
#define MQTT_PRESENCE_PAYLOAD_MAX 1024U
#define MQTT_PRESENCE_VISIBILITY_MAX 96U

typedef enum {
    MQTT_PRESENCE_OFFLINE = 0,
    MQTT_PRESENCE_ONLINE,
    MQTT_PRESENCE_STOPPING
} mqtt_presence_state_t;

typedef enum {
    MQTT_OPERATIONAL_UNKNOWN = 0,
    MQTT_OPERATIONAL_HEALTHY,
    MQTT_OPERATIONAL_WARNING,
    MQTT_OPERATIONAL_ERROR,
    MQTT_OPERATIONAL_CRITICAL
} mqtt_operational_state_t;

typedef struct {
    bool configured;
    bool connected;
    uint64_t sequence;
    uint64_t connections;
    uint64_t reconnects;
    uint64_t disconnects;
    uint64_t publish_attempts;
    uint64_t publish_successes;
    uint64_t publish_failures;
    int64_t last_publish_at_ms;
} mqtt_presence_stats_t;

/**
 * Configure immutable process identity. Reconfiguration for the same run may
 * change the topic prefix but deliberately preserves sequence and counters.
 */
int mqtt_presence_configure(const char *topic_prefix,
                            const char *installation_uuid,
                            const char *run_id, const char *boot_id,
                            const char *visibility_scope,
                            const char *version);

/** Build the static offline will. Its sequence is zero by definition. */
int mqtt_presence_build_will(int64_t configured_at_ms,
                             char *topic, size_t topic_size,
                             char *payload, size_t payload_size);

/** Build an online or stopping document and consume the next sequence. */
int mqtt_presence_build(mqtt_presence_state_t state,
                        mqtt_operational_state_t operational_state,
                        size_t active_incidents, int64_t observed_at_ms,
                        char *topic, size_t topic_size,
                        char *payload, size_t payload_size);

void mqtt_presence_record_connected(void);
void mqtt_presence_record_disconnected(void);
void mqtt_presence_record_publish(bool succeeded, int64_t observed_at_ms);
void mqtt_presence_get_stats(mqtt_presence_stats_t *stats);
int mqtt_presence_copy_topic(char *topic, size_t topic_size);
int mqtt_presence_copy_installation_uuid(char *uuid, size_t uuid_size);

/** Test/process teardown seam. Production hot reload does not call this. */
void mqtt_presence_reset(void);

#endif /* LIGHTNVR_MQTT_PRESENCE_H */
