#ifndef LIGHTNVR_CORE_MQTT_DELIVERY_WORKER_H
#define LIGHTNVR_CORE_MQTT_DELIVERY_WORKER_H

#include <stdbool.h>
#include <stdint.h>

#include "core/event_envelope.h"
#include "database/db_event_outbox.h"

#define MQTT_EVENT_OUTBOX_DESTINATION "mqtt:default"
#define MQTT_DELIVERY_ACK_TIMEOUT_MS 5000
#define MQTT_DELIVERY_MAX_BACKOFF_SECONDS 300

typedef int (*mqtt_delivery_publish_fn)(const char *topic,
                                        const char *payload,
                                        bool retain,
                                        int timeout_ms,
                                        void *context);

typedef struct {
    uint64_t enqueued;
    uint64_t duplicates;
    uint64_t rejected_full;
    uint64_t enqueue_errors;
    uint64_t priority_shed;
    uint64_t expired;
    uint64_t attempted;
    uint64_t delivered;
    uint64_t retried;
    uint64_t dead;
    uint64_t outcome_errors;
    uint64_t disconnected_polls;
    uint64_t managed_profiles;
    uint64_t managed_connected;
    uint64_t profile_reloads;
    uint64_t profile_errors;
    bool running;
} mqtt_delivery_worker_stats_t;

/*
 * Persist a normalized event for the default MQTT destination and wake the
 * delivery worker. The event topic is frozen at enqueue time so configuration
 * changes do not mutate already accepted work.
 */
event_outbox_enqueue_result_t mqtt_delivery_worker_enqueue(
    const event_envelope_t *event, const char *topic_prefix,
    int64_t *row_id);

/* Persist an event for an explicit destination using a validated topic
 * template containing {type} and {subject_id}. */
event_outbox_enqueue_result_t mqtt_delivery_worker_enqueue_destination(
    const event_envelope_t *event, const char *destination,
    const char *topic_template, int64_t *row_id);

int mqtt_delivery_topic_expand(
    const char *topic_template, const event_envelope_t *event,
    char topic[EVENT_OUTBOX_TOPIC_MAX]);

/* Start/stop the default and managed-destination worker. Stop finishes any
 * active publish attempt but leaves pending rows for the next start. */
int mqtt_delivery_worker_start(void);
void mqtt_delivery_worker_shutdown(void);

/*
 * Process at most one due row. This boundary is public for deterministic tests
 * and future destination adapters. Returns 1 when a row was attempted, 0 when
 * no row was eligible, and -1 on repository/outcome failure.
 */
int mqtt_delivery_worker_process_once(int64_t now, bool broker_connected,
                                      mqtt_delivery_publish_fn publish,
                                      void *context);

int mqtt_delivery_worker_process_destination_once(
    const char *destination, int64_t now, bool broker_connected,
    mqtt_delivery_publish_fn publish, void *context);

/* Deterministic bounded exponential backoff with per-event jitter. */
int mqtt_delivery_backoff_seconds(int attempt_count, uint32_t jitter_seed);

void mqtt_delivery_worker_get_stats(mqtt_delivery_worker_stats_t *stats);

#endif /* LIGHTNVR_CORE_MQTT_DELIVERY_WORKER_H */
