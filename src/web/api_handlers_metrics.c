/**
 * @file api_handlers_metrics.c
 * @brief Prometheus metrics endpoint and player telemetry ingest handlers
 *
 * GET /api/metrics  – Prometheus text exposition format (text/plain)
 * POST /api/telemetry/player – client-side QoE event ingestion (returns 204)
 */

#include <stdio.h>
#include <stdlib.h>
#include <stdarg.h>
#include <string.h>
#include <time.h>
#include <unistd.h>
#include <sys/sysinfo.h>
#include <cjson/cJSON.h>

#include "web/api_handlers_metrics.h"
#include "web/request_response.h"
#include "web/httpd_utils.h"
#include "telemetry/stream_metrics.h"
#include "telemetry/player_telemetry.h"
#include "telemetry/recording_io_metrics.h"
#include "telemetry/system_health.h"
#include "telemetry/system_health_evaluator.h"
#include "core/event_bus.h"
#include "core/event_router.h"
#include "core/mqtt_delivery_worker.h"
#include "core/mqtt_destination_client.h"
#include "core/mqtt_presence.h"
#include "database/db_event_destinations.h"
#include "database/db_event_outbox.h"
#include "video/stream_manager.h"
#include "storage/storage_manager.h"
#define LOG_COMPONENT "MetricsAPI"
#include "core/logger.h"
#include "core/config.h"
#include "utils/strings.h"

/* ------------------------------------------------------------------ */
/*  Growable buffer for Prometheus output                               */
/* ------------------------------------------------------------------ */

typedef struct {
    char *data;
    size_t len;
    size_t cap;
} prom_buf_t;

static void prom_buf_init(prom_buf_t *b, size_t initial_cap) {
    b->data = malloc(initial_cap);
    b->len = 0;
    b->cap = initial_cap;
    if (b->data) b->data[0] = '\0';
}

static void prom_buf_append(prom_buf_t *b, const char *fmt, ...) {
    if (!b->data) return;

    va_list ap;
    va_start(ap, fmt);
    int needed = vsnprintf(NULL, 0, fmt, ap);
    va_end(ap);

    if (needed < 0) return;

    /* Grow if needed */
    while (b->len + (size_t)needed + 1 > b->cap) {
        size_t new_cap = b->cap * 2;
        char *new_data = realloc(b->data, new_cap);
        if (!new_data) return;
        b->data = new_data;
        b->cap = new_cap;
    }

    va_start(ap, fmt);
    vsnprintf(b->data + b->len, b->cap - b->len, fmt, ap);
    va_end(ap);
    b->len += (size_t)needed;
}

static void prom_buf_free(prom_buf_t *b) {
    free(b->data);
    b->data = NULL;
    b->len = b->cap = 0;
}

static void prom_label_escape(const char *input, char *output,
                              size_t output_size);

static const char *mqtt_failure_label(mqtt_destination_failure_t failure) {
    switch (failure) {
        case MQTT_DESTINATION_FAILURE_NONE: return "none";
        case MQTT_DESTINATION_FAILURE_CONFIGURATION: return "configuration";
        case MQTT_DESTINATION_FAILURE_CONNECTION: return "connection";
        case MQTT_DESTINATION_FAILURE_PUBLICATION: return "publication";
        case MQTT_DESTINATION_FAILURE_COUNT: break;
    }
    return "unknown";
}

static bool append_outbox_metrics(prom_buf_t *buf, const char *label,
                                  const char *destination, int64_t now,
                                  event_outbox_stats_t *aggregate) {
    event_outbox_stats_t stats;
    if (db_event_outbox_get_stats(destination, now, &stats) != 0) return false;
    if (aggregate) *aggregate = stats;
    const int64_t rows[] = {
        stats.pending_rows, stats.delivering_rows, stats.delivered_rows,
        stats.dead_rows, stats.due_rows
    };
    const char *states[] = {
        "pending", "delivering", "delivered", "dead", "due"
    };
    for (size_t index = 0; index < 5U; ++index)
        prom_buf_append(buf,
            "lightnvr_event_outbox_rows{destination=\"%s\",state=\"%s\"} %lld\n",
            label, states[index], (long long)rows[index]);
    prom_buf_append(buf,
        "lightnvr_event_outbox_bytes{destination=\"%s\"} %lld\n",
        label, (long long)stats.total_bytes);
    int64_t oldest_age = stats.pending_rows > 0 &&
        stats.oldest_pending_at > 0 && now >= stats.oldest_pending_at
            ? now - stats.oldest_pending_at : 0;
    prom_buf_append(buf,
        "lightnvr_event_outbox_oldest_pending_age_seconds{destination=\"%s\"} %lld\n",
        label, (long long)oldest_age);
    return true;
}

static const mqtt_destination_client_stats_t *find_mqtt_destination_stats(
    const mqtt_destination_client_stats_t *stats, size_t count,
    const char *uuid) {
    for (size_t index = 0; index < count; ++index)
        if (strcmp(stats[index].destination_uuid, uuid) == 0)
            return &stats[index];
    return NULL;
}

char *api_metrics_render_self_observability(void) {
    prom_buf_t buffer;
    prom_buf_init(&buffer, 16384U);
    if (!buffer.data) return NULL;

    system_health_stats_t sampler;
    system_health_get_stats(&sampler);
    system_health_collector_stats_t collectors[
        SYSTEM_HEALTH_MAX_COLLECTORS + 1U];
    size_t collector_count = system_health_collector_stats_copy(
        collectors, SYSTEM_HEALTH_MAX_COLLECTORS + 1U);
    prom_buf_append(&buffer, "# HELP lightnvr_health_collector_duration_seconds Last and maximum bounded collector duration\n");
    prom_buf_append(&buffer, "# TYPE lightnvr_health_collector_duration_seconds gauge\n");
    prom_buf_append(&buffer, "# HELP lightnvr_health_collector_events_total Monotonic collector events for this process run\n");
    prom_buf_append(&buffer, "# TYPE lightnvr_health_collector_events_total counter\n");
    prom_buf_append(&buffer, "# HELP lightnvr_health_collector_stale Whether a collector has no successful sample inside its staleness bound\n");
    prom_buf_append(&buffer, "# TYPE lightnvr_health_collector_stale gauge\n");
    prom_buf_append(&buffer, "# HELP lightnvr_health_collector_busy Whether a collector call is currently in progress\n");
    prom_buf_append(&buffer, "# TYPE lightnvr_health_collector_busy gauge\n");
    for (size_t index = 0; index < collector_count; ++index) {
        char name[2U * SYSTEM_HEALTH_COLLECTOR_NAME_LENGTH + 1U];
        prom_label_escape(collectors[index].name, name, sizeof(name));
        const char *scope = system_health_scope_name(collectors[index].scope);
        prom_buf_append(&buffer,
            "lightnvr_health_collector_duration_seconds{collector=\"%s\",scope=\"%s\",kind=\"last\"} %.3f\n",
            name, scope, (double)collectors[index].last_duration_ms / 1000.0);
        prom_buf_append(&buffer,
            "lightnvr_health_collector_duration_seconds{collector=\"%s\",scope=\"%s\",kind=\"maximum\"} %.3f\n",
            name, scope,
            (double)collectors[index].maximum_duration_ms / 1000.0);
        const uint64_t events[] = {
            collectors[index].attempts, collectors[index].completions,
            collectors[index].failures, collectors[index].timeouts,
            collectors[index].overlap_skips
        };
        const char *event_names[] = {
            "attempt", "completion", "failure", "timeout", "overlap_skip"
        };
        for (size_t event = 0; event < 5U; ++event)
            prom_buf_append(&buffer,
                "lightnvr_health_collector_events_total{collector=\"%s\",scope=\"%s\",event=\"%s\"} %llu\n",
                name, scope, event_names[event],
                (unsigned long long)events[event]);
        prom_buf_append(&buffer,
            "lightnvr_health_collector_stale{collector=\"%s\",scope=\"%s\"} %d\n",
            name, scope, collectors[index].stale ? 1 : 0);
        prom_buf_append(&buffer,
            "lightnvr_health_collector_busy{collector=\"%s\",scope=\"%s\"} %d\n",
            name, scope, collectors[index].busy ? 1 : 0);
    }
    prom_buf_append(&buffer, "# HELP lightnvr_health_coverage_overflows_total Bounded health observations or resources omitted because a cap was reached\n");
    prom_buf_append(&buffer, "# TYPE lightnvr_health_coverage_overflows_total counter\n");
    prom_buf_append(&buffer, "lightnvr_health_coverage_overflows_total %llu\n",
                    (unsigned long long)sampler.coverage_overflows);
    prom_buf_append(&buffer, "# HELP lightnvr_health_sampler_events_total Monotonic sampler events for this process run\n");
    prom_buf_append(&buffer, "# TYPE lightnvr_health_sampler_events_total counter\n");
    const uint64_t sampler_values[] = {
        sampler.generations_completed, sampler.collections_completed,
        sampler.collection_errors, sampler.collection_timeouts,
        sampler.overlap_skips, sampler.observations_dropped
    };
    const char *sampler_events[] = {
        "generation", "collection", "error", "timeout", "overlap_skip",
        "observation_drop"
    };
    for (size_t index = 0; index < 6U; ++index)
        prom_buf_append(&buffer,
            "lightnvr_health_sampler_events_total{event=\"%s\"} %llu\n",
            sampler_events[index],
            (unsigned long long)sampler_values[index]);
    prom_buf_append(&buffer, "# HELP lightnvr_health_abandoned_helpers Bounded helper processes still being reaped\n");
    prom_buf_append(&buffer, "# TYPE lightnvr_health_abandoned_helpers gauge\n");
    prom_buf_append(&buffer, "lightnvr_health_abandoned_helpers %u\n",
                    sampler.abandoned_helpers);

    system_health_evaluator_stats_t evaluator;
    system_health_evaluator_service_get_stats(&evaluator);
    prom_buf_append(&buffer, "# HELP lightnvr_health_evaluator_events_total Monotonic evaluator events for this process run\n");
    prom_buf_append(&buffer, "# TYPE lightnvr_health_evaluator_events_total counter\n");
    prom_buf_append(&buffer,
        "lightnvr_health_evaluator_events_total{event=\"transition\"} %llu\n",
        (unsigned long long)evaluator.transitions);
    prom_buf_append(&buffer,
        "lightnvr_health_evaluator_events_total{event=\"persistence_failure\"} %llu\n",
        (unsigned long long)evaluator.persistence_failures);
    prom_buf_append(&buffer,
        "lightnvr_health_evaluator_events_total{event=\"persistence_retry\"} %llu\n",
        (unsigned long long)evaluator.persistence_retries);
    prom_buf_append(&buffer, "# HELP lightnvr_health_persistence_pending Conditions frozen pending durable persistence\n");
    prom_buf_append(&buffer, "# TYPE lightnvr_health_persistence_pending gauge\n");
    prom_buf_append(&buffer, "lightnvr_health_persistence_pending %zu\n",
                    evaluator.pending_persistence);

    event_bus_stats_t bus;
    event_router_stats_t router;
    mqtt_delivery_worker_stats_t worker;
    mqtt_presence_stats_t default_presence;
    event_bus_get_stats(&bus);
    event_router_get_stats(&router);
    mqtt_delivery_worker_get_stats(&worker);
    mqtt_presence_get_stats(&default_presence);
    prom_buf_append(&buffer, "# HELP lightnvr_event_bus_events_total Monotonic in-process event-bus outcomes\n");
    prom_buf_append(&buffer, "# TYPE lightnvr_event_bus_events_total counter\n");
    const uint64_t bus_values[] = {
        bus.accepted_events, bus.dispatched_events, bus.dropped_events,
        bus.priority_shed_events, bus.rejected_events, bus.handler_failures
    };
    const char *bus_events[] = {
        "accepted", "dispatched", "dropped", "priority_shed", "rejected",
        "handler_failure"
    };
    for (size_t index = 0; index < 6U; ++index)
        prom_buf_append(&buffer,
            "lightnvr_event_bus_events_total{event=\"%s\"} %llu\n",
            bus_events[index], (unsigned long long)bus_values[index]);
    prom_buf_append(&buffer, "# HELP lightnvr_event_bus_queue Current bounded event-bus queue occupancy\n");
    prom_buf_append(&buffer, "# TYPE lightnvr_event_bus_queue gauge\n");
    prom_buf_append(&buffer, "lightnvr_event_bus_queue{unit=\"events\"} %zu\n",
                    bus.queued_events);
    prom_buf_append(&buffer, "lightnvr_event_bus_queue{unit=\"bytes\"} %zu\n",
                    bus.queued_bytes);

    prom_buf_append(&buffer, "# HELP lightnvr_event_router_events_total Monotonic event-router outcomes\n");
    prom_buf_append(&buffer, "# TYPE lightnvr_event_router_events_total counter\n");
    prom_buf_append(&buffer,
        "lightnvr_event_router_events_total{event=\"evaluated\"} %llu\n",
        (unsigned long long)router.events_evaluated);
    prom_buf_append(&buffer,
        "lightnvr_event_router_events_total{event=\"matched\"} %llu\n",
        (unsigned long long)router.matched_events);
    prom_buf_append(&buffer,
        "lightnvr_event_router_events_total{event=\"unmatched\"} %llu\n",
        (unsigned long long)router.unmatched_events);
    prom_buf_append(&buffer,
        "lightnvr_event_router_events_total{event=\"error\"} %llu\n",
        (unsigned long long)router.evaluation_errors);

    prom_buf_append(&buffer, "# HELP lightnvr_event_delivery_events_total Monotonic durable-delivery worker outcomes\n");
    prom_buf_append(&buffer, "# TYPE lightnvr_event_delivery_events_total counter\n");
    const uint64_t delivery_values[] = {
        worker.enqueued, worker.rejected_full, worker.enqueue_errors,
        worker.attempted, worker.delivered, worker.retried, worker.dead,
        worker.profile_errors
    };
    const char *delivery_events[] = {
        "enqueued", "rejected_full", "enqueue_error", "attempted",
        "delivered", "retried", "dead", "profile_error"
    };
    for (size_t index = 0; index < 8U; ++index)
        prom_buf_append(&buffer,
            "lightnvr_event_delivery_events_total{event=\"%s\"} %llu\n",
            delivery_events[index],
            (unsigned long long)delivery_values[index]);

    prom_buf_append(&buffer, "# HELP lightnvr_event_outbox_rows Current durable outbox rows by bounded destination and state\n");
    prom_buf_append(&buffer, "# TYPE lightnvr_event_outbox_rows gauge\n");
    prom_buf_append(&buffer, "# HELP lightnvr_event_outbox_bytes Current durable outbox bytes by bounded destination\n");
    prom_buf_append(&buffer, "# TYPE lightnvr_event_outbox_bytes gauge\n");
    prom_buf_append(&buffer, "# HELP lightnvr_event_outbox_oldest_pending_age_seconds Age of the oldest pending durable event\n");
    prom_buf_append(&buffer, "# TYPE lightnvr_event_outbox_oldest_pending_age_seconds gauge\n");
    int64_t now = (int64_t)time(NULL);
    event_outbox_stats_t aggregate;
    memset(&aggregate, 0, sizeof(aggregate));
    bool outbox_available = append_outbox_metrics(
        &buffer, "all", NULL, now, &aggregate);
    prom_buf_append(&buffer, "# HELP lightnvr_event_outbox_stats_available Whether the durable outbox snapshot could be read\n");
    prom_buf_append(&buffer, "# TYPE lightnvr_event_outbox_stats_available gauge\n");
    prom_buf_append(&buffer, "lightnvr_event_outbox_stats_available %d\n",
                    outbox_available ? 1 : 0);
    if (default_presence.configured)
        (void)append_outbox_metrics(&buffer, "default",
                                    MQTT_EVENT_OUTBOX_DESTINATION, now, NULL);

    prom_buf_append(&buffer, "# HELP lightnvr_event_destination_connected Whether a bounded MQTT destination is connected\n");
    prom_buf_append(&buffer, "# TYPE lightnvr_event_destination_connected gauge\n");
    prom_buf_append(&buffer, "# HELP lightnvr_event_destination_events_total Monotonic MQTT destination events\n");
    prom_buf_append(&buffer, "# TYPE lightnvr_event_destination_events_total counter\n");
    prom_buf_append(&buffer, "# HELP lightnvr_event_destination_last_failure Normalized last MQTT destination failure\n");
    prom_buf_append(&buffer, "# TYPE lightnvr_event_destination_last_failure gauge\n");
    size_t configured = default_presence.configured ? 1U : 0U;
    size_t connected = default_presence.configured && default_presence.connected
        ? 1U : 0U;
    bool only_destination_failed = false;
    if (default_presence.configured) {
        prom_buf_append(&buffer,
            "lightnvr_event_destination_connected{destination=\"default\"} %d\n",
            default_presence.connected ? 1 : 0);
        prom_buf_append(&buffer,
            "lightnvr_event_destination_events_total{destination=\"default\",event=\"reconnect\",failure=\"none\"} %llu\n",
            (unsigned long long)default_presence.reconnects);
        prom_buf_append(&buffer,
            "lightnvr_event_destination_events_total{destination=\"default\",event=\"publish_failure\",failure=\"publication\"} %llu\n",
            (unsigned long long)default_presence.publish_failures);
    }
    mqtt_destination_client_stats_t destinations[EVENT_DESTINATION_MAX_COUNT];
    size_t destination_count = mqtt_destination_client_list_stats(
        destinations, EVENT_DESTINATION_MAX_COUNT);
    event_destination_t profiles[EVENT_DESTINATION_MAX_COUNT];
    int profile_count = db_event_destination_list(
        profiles, EVENT_DESTINATION_MAX_COUNT);
    bool profiles_available = profile_count >= 0;
    if (profile_count < 0) profile_count = 0;
    for (int index = 0; index < profile_count; ++index) {
        if (!profiles[index].enabled) continue;
        configured++;
        const mqtt_destination_client_stats_t *stats =
            find_mqtt_destination_stats(destinations, destination_count,
                                        profiles[index].uuid);
        if (stats && stats->connected) connected++;
        char uuid[2U * EVENT_DESTINATION_UUID_MAX + 1U];
        prom_label_escape(profiles[index].uuid, uuid, sizeof(uuid));
        const char *failure = stats
            ? mqtt_failure_label(stats->last_failure) : "configuration";
        prom_buf_append(&buffer,
            "lightnvr_event_destination_connected{destination=\"%s\"} %d\n",
            uuid, stats && stats->connected ? 1 : 0);
        prom_buf_append(&buffer,
            "lightnvr_event_destination_events_total{destination=\"%s\",event=\"reconnect\",failure=\"none\"} %llu\n",
            uuid, (unsigned long long)(stats ? stats->reconnects : 0U));
        prom_buf_append(&buffer,
            "lightnvr_event_destination_events_total{destination=\"%s\",event=\"connection_failure\",failure=\"connection\"} %llu\n",
            uuid,
            (unsigned long long)(stats ? stats->connection_failures : 0U));
        prom_buf_append(&buffer,
            "lightnvr_event_destination_events_total{destination=\"%s\",event=\"publish_failure\",failure=\"publication\"} %llu\n",
            uuid,
            (unsigned long long)(stats ? stats->publish_failures : 0U));
        prom_buf_append(&buffer,
            "lightnvr_event_destination_last_failure{destination=\"%s\",failure=\"%s\"} 1\n",
            uuid, failure);
        char key[EVENT_DESTINATION_KEY_MAX];
        if (db_event_destination_make_key(profiles[index].uuid, key) == 0)
            (void)append_outbox_metrics(&buffer, uuid, key, now, NULL);
    }
    if (!profiles_available) {
        for (size_t index = 0; index < destination_count; ++index) {
            configured++;
            if (destinations[index].connected) connected++;
            char uuid[2U * EVENT_DESTINATION_UUID_MAX + 1U];
            prom_label_escape(destinations[index].destination_uuid, uuid,
                              sizeof(uuid));
            prom_buf_append(&buffer,
                "lightnvr_event_destination_connected{destination=\"%s\"} %d\n",
                uuid, destinations[index].connected ? 1 : 0);
        }
    }
    if (configured == 1U) {
        if (default_presence.configured) {
            only_destination_failed = !default_presence.connected &&
                default_presence.publish_failures > 0U;
        } else if (profiles_available) {
            for (int index = 0; index < profile_count; ++index) {
                if (!profiles[index].enabled) continue;
                const mqtt_destination_client_stats_t *stats =
                    find_mqtt_destination_stats(
                        destinations, destination_count, profiles[index].uuid);
                only_destination_failed = !stats ||
                    (!stats->connected && stats->last_failure !=
                                               MQTT_DESTINATION_FAILURE_NONE);
                break;
            }
        } else if (destination_count == 1U) {
            only_destination_failed = !destinations[0].connected &&
                destinations[0].last_failure != MQTT_DESTINATION_FAILURE_NONE;
        }
    }
    bool outbox_full = outbox_available &&
        (aggregate.total_rows >= EVENT_OUTBOX_DEFAULT_MAX_ROWS ||
         aggregate.total_bytes >= EVENT_OUTBOX_DEFAULT_MAX_BYTES);
    bool undeliverable = outbox_available && aggregate.pending_rows > 0 &&
                         connected == 0U;
    bool degraded = outbox_full || only_destination_failed || undeliverable ||
                    (outbox_available && aggregate.dead_rows > 0) ||
                    (!profiles_available && destination_count > 0U);
    bool degraded_known = outbox_available || only_destination_failed;
    prom_buf_append(&buffer, "# HELP lightnvr_event_delivery_degraded Whether durable event delivery is locally degraded\n");
    prom_buf_append(&buffer, "# TYPE lightnvr_event_delivery_degraded gauge\n");
    if (degraded_known)
        prom_buf_append(&buffer, "lightnvr_event_delivery_degraded %d\n",
                        degraded ? 1 : 0);
    prom_buf_append(&buffer, "# HELP lightnvr_event_delivery_circular_report_path Whether the only alert path is the degraded destination itself\n");
    prom_buf_append(&buffer, "# TYPE lightnvr_event_delivery_circular_report_path gauge\n");
    prom_buf_append(&buffer, "lightnvr_event_delivery_circular_report_path %d\n",
                    configured == 1U &&
                    (only_destination_failed || outbox_full || undeliverable)
                        ? 1 : 0);

    int64_t oldest_pending_ms = evaluator.oldest_pending_wall_time_ms;
    prom_buf_append(&buffer, "# HELP lightnvr_health_persistence_retry_age_seconds Age of the oldest incident awaiting durable persistence\n");
    prom_buf_append(&buffer, "# TYPE lightnvr_health_persistence_retry_age_seconds gauge\n");
    int64_t now_ms = now > 0 ? now * 1000 : 0;
    double retry_age = oldest_pending_ms > 0 && now_ms >= oldest_pending_ms
        ? (double)(now_ms - oldest_pending_ms) / 1000.0 : 0.0;
    prom_buf_append(&buffer,
        "lightnvr_health_persistence_retry_age_seconds %.3f\n", retry_age);
    return buffer.data;
}

static void prom_label_escape(const char *input, char *output,
                              size_t output_size) {
    size_t used = 0;
    if (!output || output_size == 0) return;
    if (!input) input = "";
    while (*input && used + 1U < output_size) {
        const char *replacement = NULL;
        if (*input == '\\') replacement = "\\\\";
        else if (*input == '"') replacement = "\\\"";
        else if (*input == '\n' || *input == '\r') replacement = "\\n";
        if (replacement) {
            if (used + 2U >= output_size) break;
            output[used++] = replacement[0];
            output[used++] = replacement[1];
        } else {
            output[used++] = *input;
        }
        input++;
    }
    output[used] = '\0';
}

static bool observation_usable(const system_health_observation_t *observation) {
    return observation && observation->value_valid &&
        observation->capability == SYSTEM_HEALTH_CAPABILITY_AVAILABLE &&
        observation->freshness != SYSTEM_HEALTH_FRESHNESS_STALE;
}

static const system_health_observation_t *find_observation(
    const system_health_snapshot_t *snapshot, const char *metric,
    const char *resource_id) {
    if (!snapshot || !metric) return NULL;
    for (size_t index = 0; index < snapshot->observation_count; ++index) {
        const system_health_observation_t *observation =
            &snapshot->observations[index];
        if (strcmp(observation->metric, metric) == 0 &&
            (!resource_id ||
             strcmp(observation->resource_id, resource_id) == 0)) {
            return observation;
        }
    }
    return NULL;
}

static const char *collector_for_metric(const system_health_observation_t *o) {
    if (strcmp(o->metric, "hardware.provider.visible") == 0)
        return o->resource_id;
    if (strncmp(o->metric, "storage.device.", 15) == 0 ||
        strncmp(o->metric, "hardware.", 9) == 0)
        return "linux_hardware";
    if (strncmp(o->metric, "kernel.", 7) == 0) return "kernel_log";
    if (strncmp(o->metric, "storage.filesystem.", 19) == 0)
        return "storage_targets";
    if (strncmp(o->metric, "filesystem.", 11) == 0)
        return "linux_filesystem";
    if (strncmp(o->metric, "network.", 8) == 0) return "linux_network";
    if (strncmp(o->metric, "thermal.", 8) == 0) return "linux_thermal";
    if (strncmp(o->metric, "process.", 8) == 0) return "linux_process";
    if (strncmp(o->metric, "container.", 10) == 0)
        return "linux_cgroup";
    if (strncmp(o->metric, "host.", 5) == 0) return "linux_proc";
    if (strncmp(o->metric, "clock.", 6) == 0) return "linux_clock";
    if (strcmp(o->metric, "system.uptime_seconds") == 0)
        return "linux_restart";
    return NULL;
}

static void append_single_observation(prom_buf_t *buf, const char *family,
                                      const char *labels,
                                      const system_health_observation_t *o) {
    if (!observation_usable(o)) return;
    prom_buf_append(buf, "%s%s %.17g\n", family, labels ? labels : "",
                    o->value);
}

static void append_system_observations(prom_buf_t *buf,
                                       const system_health_snapshot_t *snapshot) {
    const system_health_observation_t *o;
    const system_health_observation_t *cpu = find_observation(
        snapshot, "container.cpu.usage_ratio", NULL);
    if (!observation_usable(cpu))
        cpu = find_observation(snapshot, "host.cpu.usage_ratio", NULL);
    if (!observation_usable(cpu))
        cpu = find_observation(snapshot, "host.cpu.busy_ratio", NULL);

    prom_buf_append(buf, "# HELP lightnvr_system_cpu_usage_ratio Effective CPU usage as a ratio\n");
    prom_buf_append(buf, "# TYPE lightnvr_system_cpu_usage_ratio gauge\n");
    append_single_observation(buf, "lightnvr_system_cpu_usage_ratio", "", cpu);

    prom_buf_append(buf, "# HELP lightnvr_system_load_average Host runnable load average\n");
    prom_buf_append(buf, "# TYPE lightnvr_system_load_average gauge\n");
    prom_buf_append(buf, "# HELP lightnvr_system_load_ratio Host load average divided by effective CPU capacity when that capacity is visible\n");
    prom_buf_append(buf, "# TYPE lightnvr_system_load_ratio gauge\n");
    const char *load_metrics[] = {"host.load.1", "host.load.5", "host.load.15"};
    const char *load_periods[] = {"1m", "5m", "15m"};
    const system_health_observation_t *cpu_capacity = find_observation(
        snapshot, "container.cpu.quota_cores", NULL);
    if (!observation_usable(cpu_capacity))
        cpu_capacity = find_observation(snapshot, "host.cpu.quota_cores", NULL);
    for (size_t index = 0; index < 3U; ++index) {
        o = find_observation(snapshot, load_metrics[index], NULL);
        char labels[32];
        snprintf(labels, sizeof(labels), "{period=\"%s\"}",
                 load_periods[index]);
        append_single_observation(buf, "lightnvr_system_load_average", labels,
                                  o);
        if (observation_usable(o) && observation_usable(cpu_capacity) &&
            cpu_capacity->value > 0.0) {
            prom_buf_append(buf, "lightnvr_system_load_ratio%s %.17g\n",
                            labels, o->value / cpu_capacity->value);
        }
    }

    prom_buf_append(buf, "# HELP lightnvr_system_pressure_stall_ratio Linux PSI stall ratio\n");
    prom_buf_append(buf, "# TYPE lightnvr_system_pressure_stall_ratio gauge\n");
    const char *pressure_resources[] = {"cpu", "memory", "io"};
    const char *pressure_kinds[] = {"some", "full"};
    for (size_t resource = 0; resource < 3U; ++resource) {
        for (size_t kind = 0; kind < 2U; ++kind) {
            char metric[SYSTEM_HEALTH_METRIC_LENGTH];
            snprintf(metric, sizeof(metric), "host.pressure.%s.%s_ratio",
                     pressure_resources[resource], pressure_kinds[kind]);
            o = find_observation(snapshot, metric, NULL);
            char labels[128];
            snprintf(labels, sizeof(labels),
                     "{resource=\"%s\",kind=\"%s\",window=\"10s\"}",
                     pressure_resources[resource], pressure_kinds[kind]);
            append_single_observation(
                buf, "lightnvr_system_pressure_stall_ratio", labels, o);
        }
    }

    prom_buf_append(buf, "# HELP lightnvr_system_cpu_throttled_ratio Cgroup CPU throttled-period ratio\n");
    prom_buf_append(buf, "# TYPE lightnvr_system_cpu_throttled_ratio gauge\n");
    const char *throttle_metrics[] = {"container.cpu.throttled_ratio",
                                      "host.cpu.throttled_ratio"};
    const char *throttle_scopes[] = {"container", "host"};
    for (size_t index = 0; index < 2U; ++index) {
        o = find_observation(snapshot, throttle_metrics[index], NULL);
        char labels[48];
        snprintf(labels, sizeof(labels), "{scope=\"%s\"}",
                 throttle_scopes[index]);
        append_single_observation(
            buf, "lightnvr_system_cpu_throttled_ratio", labels, o);
    }

    const system_health_observation_t *memory_total = find_observation(
        snapshot, "container.memory.limit_bytes", NULL);
    const system_health_observation_t *memory_available = find_observation(
        snapshot, "container.memory.available_bytes", NULL);
    const system_health_observation_t *memory_used = find_observation(
        snapshot, "container.memory.current_bytes", NULL);
    if (!observation_usable(memory_total)) {
        memory_total = find_observation(snapshot, "host.memory.total_bytes", NULL);
        memory_available = find_observation(
            snapshot, "host.memory.available_bytes", NULL);
        memory_used = NULL;
    }
    prom_buf_append(buf, "# HELP lightnvr_system_memory_bytes Effective memory by state\n");
    prom_buf_append(buf, "# TYPE lightnvr_system_memory_bytes gauge\n");
    append_single_observation(buf, "lightnvr_system_memory_bytes",
                              "{state=\"total\"}", memory_total);
    append_single_observation(buf, "lightnvr_system_memory_bytes",
                              "{state=\"available\"}", memory_available);
    if (observation_usable(memory_used)) {
        append_single_observation(buf, "lightnvr_system_memory_bytes",
                                  "{state=\"used\"}", memory_used);
    } else if (observation_usable(memory_total) &&
               observation_usable(memory_available)) {
        double used = memory_total->value > memory_available->value
            ? memory_total->value - memory_available->value : 0.0;
        prom_buf_append(buf,
                        "lightnvr_system_memory_bytes{state=\"used\"} %.17g\n",
                        used);
    }

    prom_buf_append(buf, "# HELP lightnvr_system_swap_bytes Host swap by state\n");
    prom_buf_append(buf, "# TYPE lightnvr_system_swap_bytes gauge\n");
    append_single_observation(
        buf, "lightnvr_system_swap_bytes", "{state=\"used\"}",
        find_observation(snapshot, "host.swap.used_bytes", NULL));

    prom_buf_append(buf, "# HELP lightnvr_system_vm_events_delta VM events observed during the latest collection interval\n");
    prom_buf_append(buf, "# TYPE lightnvr_system_vm_events_delta gauge\n");
    const char *vm_metrics[] = {"host.vm.major_faults_delta",
                                "host.vm.swap_in_pages_delta",
                                "host.vm.swap_out_pages_delta"};
    const char *vm_events[] = {"major_fault", "swap_in", "swap_out"};
    for (size_t index = 0; index < 3U; ++index) {
        char labels[64];
        snprintf(labels, sizeof(labels), "{event=\"%s\"}", vm_events[index]);
        append_single_observation(buf, "lightnvr_system_vm_events_delta", labels,
                                  find_observation(snapshot, vm_metrics[index], NULL));
    }
    append_single_observation(
        buf, "lightnvr_system_vm_events_delta", "{event=\"oom_kill\"}",
        find_observation(snapshot, "container.memory.oom_kills_delta", NULL));

    prom_buf_append(buf, "# HELP lightnvr_process_open_fds Open file descriptors in the LightNVR process\n");
    prom_buf_append(buf, "# TYPE lightnvr_process_open_fds gauge\n");
    append_single_observation(buf, "lightnvr_process_open_fds", "",
        find_observation(snapshot, "process.open_fds", NULL));
    prom_buf_append(buf, "# HELP lightnvr_process_max_fds Process file descriptor limit\n");
    prom_buf_append(buf, "# TYPE lightnvr_process_max_fds gauge\n");
    append_single_observation(buf, "lightnvr_process_max_fds", "",
        find_observation(snapshot, "process.fd_limit", NULL));
    prom_buf_append(buf, "# HELP lightnvr_process_threads Threads in the LightNVR process\n");
    prom_buf_append(buf, "# TYPE lightnvr_process_threads gauge\n");
    append_single_observation(buf, "lightnvr_process_threads", "",
        find_observation(snapshot, "process.threads", NULL));
    prom_buf_append(buf, "# HELP lightnvr_process_max_pids Effective process or cgroup PID limit\n");
    prom_buf_append(buf, "# TYPE lightnvr_process_max_pids gauge\n");
    o = find_observation(snapshot, "container.pids.limit", NULL);
    if (!observation_usable(o))
        o = find_observation(snapshot, "process.pid_limit", NULL);
    append_single_observation(buf, "lightnvr_process_max_pids", "", o);

    bool storage_filesystem_seen = false;
    for (size_t index = 0; index < snapshot->observation_count; ++index) {
        if (strncmp(snapshot->observations[index].metric,
                    "storage.filesystem.", 19) == 0) {
            storage_filesystem_seen = true;
            break;
        }
    }
    const char *filesystem_prefix = storage_filesystem_seen
        ? "storage.filesystem." : "filesystem.";
    prom_buf_append(buf, "# HELP lightnvr_filesystem_bytes Filesystem bytes by logical filesystem and state\n");
    prom_buf_append(buf, "# TYPE lightnvr_filesystem_bytes gauge\n");
    prom_buf_append(buf, "# HELP lightnvr_filesystem_inodes Filesystem inodes by logical filesystem and state\n");
    prom_buf_append(buf, "# TYPE lightnvr_filesystem_inodes gauge\n");
    prom_buf_append(buf, "# HELP lightnvr_filesystem_read_only Whether a logical filesystem is read-only\n");
    prom_buf_append(buf, "# TYPE lightnvr_filesystem_read_only gauge\n");
    prom_buf_append(buf, "# HELP lightnvr_filesystem_probe_duration_seconds Last bounded write probe duration\n");
    prom_buf_append(buf, "# TYPE lightnvr_filesystem_probe_duration_seconds gauge\n");
    for (size_t index = 0; index < snapshot->observation_count; ++index) {
        o = &snapshot->observations[index];
        if (strncmp(o->metric, filesystem_prefix,
                    strlen(filesystem_prefix)) != 0 ||
            !observation_usable(o)) continue;
        const char *suffix = o->metric + strlen(filesystem_prefix);
        char resource[2U * SYSTEM_HEALTH_ID_LENGTH + 1U];
        char labels[2U * SYSTEM_HEALTH_ID_LENGTH + 48U];
        prom_label_escape(o->resource_id, resource, sizeof(resource));
        if (strcmp(suffix, "capacity_bytes") == 0 ||
            strcmp(suffix, "available_bytes") == 0) {
            snprintf(labels, sizeof(labels), "{filesystem=\"%s\",state=\"%s\"}",
                     resource, suffix[0] == 'c' ? "total" : "available");
            append_single_observation(buf, "lightnvr_filesystem_bytes", labels, o);
        } else if (strcmp(suffix, "capacity_inodes") == 0 ||
                   strcmp(suffix, "available_inodes") == 0) {
            snprintf(labels, sizeof(labels), "{filesystem=\"%s\",state=\"%s\"}",
                     resource, suffix[0] == 'c' ? "total" : "available");
            append_single_observation(buf, "lightnvr_filesystem_inodes", labels, o);
        } else if (strcmp(suffix, "read_only") == 0) {
            snprintf(labels, sizeof(labels), "{filesystem=\"%s\"}", resource);
            append_single_observation(buf, "lightnvr_filesystem_read_only", labels, o);
        } else if (strcmp(suffix, "probe_latency_seconds") == 0) {
            snprintf(labels, sizeof(labels), "{filesystem=\"%s\"}", resource);
            append_single_observation(
                buf, "lightnvr_filesystem_probe_duration_seconds", labels, o);
        }
    }

    prom_buf_append(buf, "# HELP lightnvr_thermal_celsius Temperature sensor reading\n");
    prom_buf_append(buf, "# TYPE lightnvr_thermal_celsius gauge\n");
    prom_buf_append(buf, "# HELP lightnvr_network_packets_total Network packets since kernel counter reset\n");
    prom_buf_append(buf, "# TYPE lightnvr_network_packets_total counter\n");
    prom_buf_append(buf, "# HELP lightnvr_network_bytes_total Network bytes since kernel counter reset\n");
    prom_buf_append(buf, "# TYPE lightnvr_network_bytes_total counter\n");
    prom_buf_append(buf, "# HELP lightnvr_network_errors_total Network errors and drops since kernel counter reset\n");
    prom_buf_append(buf, "# TYPE lightnvr_network_errors_total counter\n");
    for (size_t index = 0; index < snapshot->observation_count; ++index) {
        o = &snapshot->observations[index];
        if (!observation_usable(o)) continue;
        char resource[2U * SYSTEM_HEALTH_ID_LENGTH + 1U];
        prom_label_escape(o->resource_id, resource, sizeof(resource));
        if (strcmp(o->metric, "thermal.temperature_celsius") == 0 ||
            strcmp(o->metric, "storage.device.temperature_celsius") == 0) {
            char labels[2U * SYSTEM_HEALTH_ID_LENGTH + 32U];
            snprintf(labels, sizeof(labels), "{sensor=\"%s\"}", resource);
            append_single_observation(buf, "lightnvr_thermal_celsius", labels, o);
        }
        const char *direction = NULL;
        const char *kind = NULL;
        bool bytes = false;
        if (strcmp(o->metric, "network.rx_bytes_total") == 0)
            direction = "rx", bytes = true;
        else if (strcmp(o->metric, "network.tx_bytes_total") == 0)
            direction = "tx", bytes = true;
        else if (strcmp(o->metric, "network.rx_packets_total") == 0)
            direction = "rx";
        else if (strcmp(o->metric, "network.tx_packets_total") == 0)
            direction = "tx";
        else if (strcmp(o->metric, "network.rx_errors_total") == 0)
            direction = "rx", kind = "error";
        else if (strcmp(o->metric, "network.tx_errors_total") == 0)
            direction = "tx", kind = "error";
        else if (strcmp(o->metric, "network.rx_drops_total") == 0)
            direction = "rx", kind = "drop";
        else if (strcmp(o->metric, "network.tx_drops_total") == 0)
            direction = "tx", kind = "drop";
        if (direction) {
            char labels[2U * SYSTEM_HEALTH_ID_LENGTH + 80U];
            if (kind) {
                snprintf(labels, sizeof(labels),
                         "{interface=\"%s\",direction=\"%s\",kind=\"%s\"}",
                         resource, direction, kind);
                append_single_observation(
                    buf, "lightnvr_network_errors_total", labels, o);
            } else {
                snprintf(labels, sizeof(labels),
                         "{interface=\"%s\",direction=\"%s\"}",
                         resource, direction);
                append_single_observation(
                    buf, bytes ? "lightnvr_network_bytes_total"
                               : "lightnvr_network_packets_total",
                    labels, o);
            }
        }
    }

    prom_buf_append(buf, "# HELP lightnvr_clock_synchronized Whether the visible host clock is synchronized\n");
    prom_buf_append(buf, "# TYPE lightnvr_clock_synchronized gauge\n");
    append_single_observation(buf, "lightnvr_clock_synchronized", "",
        find_observation(snapshot, "clock.synchronized", NULL));
    prom_buf_append(buf, "# HELP lightnvr_clock_jump_seconds Last detected realtime-to-monotonic clock jump\n");
    prom_buf_append(buf, "# TYPE lightnvr_clock_jump_seconds gauge\n");
    append_single_observation(buf, "lightnvr_clock_jump_seconds", "",
        find_observation(snapshot, "clock.jump_seconds", NULL));
    prom_buf_append(buf, "# HELP lightnvr_system_uptime_seconds Visible host uptime\n");
    prom_buf_append(buf, "# TYPE lightnvr_system_uptime_seconds gauge\n");
    append_single_observation(buf, "lightnvr_system_uptime_seconds", "",
        find_observation(snapshot, "system.uptime_seconds", NULL));
}

static void append_hardware_observations(
    prom_buf_t *buf, const system_health_snapshot_t *snapshot) {
    prom_buf_append(buf, "# HELP lightnvr_device_wear_ratio Storage-device wear ratio\n");
    prom_buf_append(buf, "# TYPE lightnvr_device_wear_ratio gauge\n");
    prom_buf_append(buf, "# HELP lightnvr_device_available_spare_ratio Storage-device available spare ratio\n");
    prom_buf_append(buf, "# TYPE lightnvr_device_available_spare_ratio gauge\n");
    prom_buf_append(buf, "# HELP lightnvr_device_health_flag Normalized storage-device health flag\n");
    prom_buf_append(buf, "# TYPE lightnvr_device_health_flag gauge\n");
    prom_buf_append(buf, "# HELP lightnvr_device_events_delta Storage-device events in the latest provider interval\n");
    prom_buf_append(buf, "# TYPE lightnvr_device_events_delta gauge\n");
    prom_buf_append(buf, "# HELP lightnvr_device_smart_attribute Normalized current SMART attribute value\n");
    prom_buf_append(buf, "# TYPE lightnvr_device_smart_attribute gauge\n");
    prom_buf_append(buf, "# HELP lightnvr_hardware_ecc_errors_delta ECC errors in the latest provider interval\n");
    prom_buf_append(buf, "# TYPE lightnvr_hardware_ecc_errors_delta gauge\n");
    prom_buf_append(buf, "# HELP lightnvr_hardware_fan_rpm Hardware fan speed\n");
    prom_buf_append(buf, "# TYPE lightnvr_hardware_fan_rpm gauge\n");
    prom_buf_append(buf, "# HELP lightnvr_hardware_fan_flag Normalized fan condition\n");
    prom_buf_append(buf, "# TYPE lightnvr_hardware_fan_flag gauge\n");
    prom_buf_append(buf, "# HELP lightnvr_hardware_power_flag Normalized board power or throttling condition\n");
    prom_buf_append(buf, "# TYPE lightnvr_hardware_power_flag gauge\n");
    prom_buf_append(buf, "# HELP lightnvr_kernel_events_delta Normalized kernel events in the latest provider interval\n");
    prom_buf_append(buf, "# TYPE lightnvr_kernel_events_delta gauge\n");

    for (size_t index = 0; index < snapshot->observation_count; ++index) {
        const system_health_observation_t *o = &snapshot->observations[index];
        if (!observation_usable(o)) continue;
        char resource[2U * SYSTEM_HEALTH_ID_LENGTH + 1U];
        char labels[2U * SYSTEM_HEALTH_ID_LENGTH + 96U];
        prom_label_escape(o->resource_id, resource, sizeof(resource));
        if (strcmp(o->metric, "storage.device.life_used_ratio") == 0 ||
            strcmp(o->metric, "storage.device.percentage_used_ratio") == 0) {
            const char *source = strstr(o->metric, "percentage")
                ? "percentage_used" : "life_used";
            snprintf(labels, sizeof(labels),
                     "{device=\"%s\",source=\"%s\"}", resource, source);
            append_single_observation(buf, "lightnvr_device_wear_ratio",
                                      labels, o);
        } else if (strcmp(o->metric,
                          "storage.device.available_spare_ratio") == 0) {
            snprintf(labels, sizeof(labels), "{device=\"%s\"}", resource);
            append_single_observation(
                buf, "lightnvr_device_available_spare_ratio", labels, o);
        } else if (strcmp(o->metric, "storage.device.pre_eol") == 0 ||
                   strcmp(o->metric, "storage.device.prefail") == 0 ||
                   strcmp(o->metric, "storage.device.critical") == 0) {
            const char *kind = strrchr(o->metric, '.');
            snprintf(labels, sizeof(labels),
                     "{device=\"%s\",kind=\"%s\"}", resource,
                     kind ? kind + 1 : "unknown");
            append_single_observation(buf, "lightnvr_device_health_flag",
                                      labels, o);
        } else if (strcmp(o->metric,
                          "storage.device.reallocated_sectors") == 0 ||
                   strcmp(o->metric,
                          "storage.device.pending_sectors") == 0) {
            const char *attribute = strstr(o->metric, "reallocated")
                ? "reallocated_sectors" : "pending_sectors";
            snprintf(labels, sizeof(labels),
                     "{device=\"%s\",attribute=\"%s\"}", resource,
                     attribute);
            append_single_observation(buf, "lightnvr_device_smart_attribute",
                                      labels, o);
        } else if (strcmp(o->metric, "storage.device.media_errors_delta") == 0 ||
                   strcmp(o->metric,
                          "storage.device.unsafe_shutdowns_delta") == 0 ||
                   strcmp(o->metric,
                          "storage.device.uncorrectable_errors_delta") == 0 ||
                   strcmp(o->metric,
                          "storage.device.interface_crc_errors_delta") == 0) {
            const char *event = strstr(o->metric, "media_errors")
                ? "media_error" : strstr(o->metric, "unsafe_shutdowns")
                ? "unsafe_shutdown" : strstr(o->metric, "interface_crc")
                ? "interface_crc_error" : "uncorrectable_error";
            snprintf(labels, sizeof(labels),
                     "{device=\"%s\",event=\"%s\"}", resource, event);
            append_single_observation(buf, "lightnvr_device_events_delta",
                                      labels, o);
        } else if (strcmp(o->metric, "hardware.ecc.corrected_delta") == 0 ||
                   strcmp(o->metric,
                          "hardware.ecc.uncorrectable_delta") == 0) {
            const char *kind = strstr(o->metric, "uncorrectable")
                ? "uncorrectable" : "corrected";
            snprintf(labels, sizeof(labels),
                     "{resource=\"%s\",correctability=\"%s\"}",
                     resource, kind);
            append_single_observation(
                buf, "lightnvr_hardware_ecc_errors_delta", labels, o);
        } else if (strcmp(o->metric, "hardware.fan.rpm") == 0 ||
                   strcmp(o->metric, "hardware.fan.minimum_rpm") == 0) {
            snprintf(labels, sizeof(labels),
                     "{fan=\"%s\",state=\"%s\"}", resource,
                     strstr(o->metric, "minimum") ? "minimum" : "current");
            append_single_observation(buf, "lightnvr_hardware_fan_rpm",
                                      labels, o);
        } else if (strcmp(o->metric, "hardware.fan.hot") == 0 ||
                   strcmp(o->metric, "hardware.fan.failed") == 0) {
            snprintf(labels, sizeof(labels),
                     "{fan=\"%s\",kind=\"%s\"}", resource,
                     strstr(o->metric, "failed") ? "failed" : "hot");
            append_single_observation(buf, "lightnvr_hardware_fan_flag",
                                      labels, o);
        } else if (strcmp(o->metric, "hardware.throttled") == 0 ||
                   strcmp(o->metric, "hardware.power.unstable") == 0) {
            snprintf(labels, sizeof(labels),
                     "{resource=\"%s\",kind=\"%s\"}", resource,
                     strstr(o->metric, "power") ? "unstable" : "throttled");
            append_single_observation(buf, "lightnvr_hardware_power_flag",
                                      labels, o);
        } else if (strncmp(o->metric, "kernel.", 7) == 0 &&
                   strstr(o->metric, "_delta") != NULL) {
            char event[SYSTEM_HEALTH_METRIC_LENGTH];
            size_t length = strlen(o->metric + 7);
            if (length >= sizeof(event)) length = sizeof(event) - 1U;
            memcpy(event, o->metric + 7, length);
            event[length] = '\0';
            char *delta = strstr(event, "_delta");
            if (delta && delta[6] == '\0') *delta = '\0';
            snprintf(labels, sizeof(labels), "{event=\"%s\"}", event);
            append_single_observation(buf, "lightnvr_kernel_events_delta",
                                      labels, o);
        }
    }
}

typedef struct {
    char name[SYSTEM_HEALTH_ID_LENGTH];
    system_health_scope_t scope;
    bool seen;
    bool up;
} collector_export_t;

static void append_health_coverage(prom_buf_t *buf,
                                   const system_health_snapshot_t *snapshot) {
    uint32_t capability_counts[SYSTEM_HEALTH_SCOPE_COUNT]
                              [SYSTEM_HEALTH_CAPABILITY_COUNT] = {{0}};
    collector_export_t collectors[SYSTEM_HEALTH_MAX_COLLECTORS] = {0};
    size_t collector_count = 0;

    for (size_t index = 0; index < snapshot->observation_count; ++index) {
        const system_health_observation_t *o = &snapshot->observations[index];
        if (o->scope >= 0 && o->scope < SYSTEM_HEALTH_SCOPE_COUNT &&
            o->capability >= 0 &&
            o->capability < SYSTEM_HEALTH_CAPABILITY_COUNT) {
            capability_counts[o->scope][o->capability]++;
        }
        const char *name = collector_for_metric(o);
        if (!name || name[0] == '\0') continue;
        size_t slot = collector_count;
        for (size_t candidate = 0; candidate < collector_count; ++candidate) {
            if (collectors[candidate].scope == o->scope &&
                strcmp(collectors[candidate].name, name) == 0) {
                slot = candidate;
                break;
            }
        }
        if (slot == collector_count) {
            if (collector_count >= SYSTEM_HEALTH_MAX_COLLECTORS) continue;
            snprintf(collectors[slot].name, sizeof(collectors[slot].name),
                     "%s", name);
            collectors[slot].scope = o->scope;
            collectors[slot].seen = true;
            collector_count++;
        }
        if (observation_usable(o) &&
            (strcmp(o->metric, "hardware.provider.visible") != 0 ||
             o->value != 0.0)) {
            collectors[slot].up = true;
        }
    }

    prom_buf_append(buf, "# HELP lightnvr_health_collector_up Whether a bounded collector has at least one fresh available observation\n");
    prom_buf_append(buf, "# TYPE lightnvr_health_collector_up gauge\n");
    for (size_t index = 0; index < collector_count; ++index) {
        char name[2U * SYSTEM_HEALTH_ID_LENGTH + 1U];
        prom_label_escape(collectors[index].name, name, sizeof(name));
        prom_buf_append(buf,
            "lightnvr_health_collector_up{collector=\"%s\",scope=\"%s\"} %d\n",
            name, system_health_scope_name(collectors[index].scope),
            collectors[index].up ? 1 : 0);
    }

    prom_buf_append(buf, "# HELP lightnvr_health_observations Number of bounded observations by scope and capability\n");
    prom_buf_append(buf, "# TYPE lightnvr_health_observations gauge\n");
    for (int scope = 0; scope < SYSTEM_HEALTH_SCOPE_COUNT; ++scope) {
        for (int capability = 0; capability < SYSTEM_HEALTH_CAPABILITY_COUNT;
             ++capability) {
            if (capability_counts[scope][capability] == 0U) continue;
            prom_buf_append(buf,
                "lightnvr_health_observations{scope=\"%s\",capability=\"%s\"} %u\n",
                system_health_scope_name((system_health_scope_t)scope),
                system_health_capability_name(
                    (system_health_capability_t)capability),
                capability_counts[scope][capability]);
        }
    }
    prom_buf_append(buf, "# HELP lightnvr_health_snapshot_observations_dropped Observations omitted from this immutable snapshot because the cap was reached\n");
    prom_buf_append(buf, "# TYPE lightnvr_health_snapshot_observations_dropped gauge\n");
    prom_buf_append(buf, "lightnvr_health_snapshot_observations_dropped %zu\n",
                    snapshot->observations_dropped);
}

static void append_health_service_metrics(
    prom_buf_t *buf, const system_health_stats_t *sampler,
    const system_health_evaluator_stats_t *evaluator,
    const system_health_incident_view_t *incidents, size_t incident_count,
    const recording_io_metrics_snapshot_t *recording,
    const system_health_process_run_t *run, bool run_valid) {
    (void)sampler;
    (void)evaluator;
    prom_buf_append(buf, "# HELP lightnvr_health_incidents Active operational incidents by severity\n");
    prom_buf_append(buf, "# TYPE lightnvr_health_incidents gauge\n");
    size_t severities[4] = {0};
    for (size_t index = 0; index < incident_count; ++index) {
        if (incidents[index].severity > SYSTEM_HEALTH_SEVERITY_NONE &&
            incidents[index].severity <= SYSTEM_HEALTH_SEVERITY_CRITICAL)
            severities[incidents[index].severity]++;
    }
    for (int severity = SYSTEM_HEALTH_SEVERITY_WARNING;
         severity <= SYSTEM_HEALTH_SEVERITY_CRITICAL; ++severity) {
        prom_buf_append(buf,
            "lightnvr_health_incidents{severity=\"%s\"} %zu\n",
            system_health_severity_name((system_health_severity_t)severity),
            severities[severity]);
    }

    prom_buf_append(buf, "# HELP lightnvr_recording_io_failures_total Recording write failures by bounded resource and reason\n");
    prom_buf_append(buf, "# TYPE lightnvr_recording_io_failures_total counter\n");
    const char *resources[] = {"recording", "hls"};
    for (int resource = 0; resource < RECORDING_IO_RESOURCE_COUNT; ++resource) {
        for (int reason = RECORDING_IO_REASON_NO_SPACE;
             reason < RECORDING_IO_REASON_COUNT; ++reason) {
            prom_buf_append(buf,
                "lightnvr_recording_io_failures_total{resource=\"%s\",reason=\"%s\"} %llu\n",
                resources[resource],
                recording_io_reason_name((recording_io_reason_t)reason),
                (unsigned long long)recording->reason_totals[resource][reason]);
        }
    }

    if (run_valid) {
        char boot_id[2U * SYSTEM_HEALTH_INCIDENT_BOOT_ID_MAX + 1U];
        prom_label_escape(run->boot_id, boot_id, sizeof(boot_id));
        prom_buf_append(buf, "# HELP lightnvr_process_start_time_seconds Process run start time in Unix seconds\n");
        prom_buf_append(buf, "# TYPE lightnvr_process_start_time_seconds gauge\n");
        prom_buf_append(buf, "lightnvr_process_start_time_seconds %.3f\n",
                        (double)run->started_at_ms / 1000.0);
        prom_buf_append(buf, "# HELP lightnvr_system_boot_info Current bounded boot identity for counter-reset provenance\n");
        prom_buf_append(buf, "# TYPE lightnvr_system_boot_info gauge\n");
        prom_buf_append(buf, "lightnvr_system_boot_info{boot_id=\"%s\"} 1\n",
                        boot_id);
    }
}

char *api_metrics_render_system_health(
    const system_health_snapshot_t *snapshot,
    const system_health_stats_t *sampler,
    const system_health_evaluator_stats_t *evaluator,
    const system_health_incident_view_t *incidents, size_t incident_count,
    const recording_io_metrics_snapshot_t *recording,
    const system_health_process_run_t *run, bool run_valid) {
    if (!snapshot || !sampler || !evaluator || !recording ||
        (!incidents && incident_count != 0U)) return NULL;
    prom_buf_t buffer;
    prom_buf_init(&buffer, 32768U);
    if (!buffer.data) return NULL;
    prom_buf_append(&buffer, "# HELP lightnvr_health_snapshot_sequence Last completed immutable health snapshot sequence\n");
    prom_buf_append(&buffer, "# TYPE lightnvr_health_snapshot_sequence gauge\n");
    prom_buf_append(&buffer, "lightnvr_health_snapshot_sequence %llu\n",
                    (unsigned long long)snapshot->sequence);
    append_system_observations(&buffer, snapshot);
    append_hardware_observations(&buffer, snapshot);
    append_health_coverage(&buffer, snapshot);
    append_health_service_metrics(&buffer, sampler, evaluator, incidents,
                                  incident_count, recording, run, run_valid);
    return buffer.data;
}

/* ------------------------------------------------------------------ */
/*  Instance-level helpers                                              */
/* ------------------------------------------------------------------ */

static uint64_t get_go2rtc_rss_bytes(void) {
#ifdef USE_GO2RTC
    extern bool get_go2rtc_memory_usage(unsigned long long *memory_usage);
    unsigned long long mem = 0;
    if (get_go2rtc_memory_usage(&mem)) {
        return (uint64_t)mem;
    }
#endif
    return 0;
}

/* ------------------------------------------------------------------ */
/*  GET /api/metrics  (Prometheus text exposition)                      */
/* ------------------------------------------------------------------ */

void handle_get_metrics(const http_request_t *req, http_response_t *res) {
    if (!httpd_authorize_global_action(req, res, AUTHZ_SYSTEM_ADMIN)) {
        return;
    }

    system_health_snapshot_t health_snapshot;
    memset(&health_snapshot, 0, sizeof(health_snapshot));
    bool health_available = system_health_snapshot_copy(&health_snapshot);

    int max = metrics_get_max_streams();
    if (max <= 0) {
        res->status_code = 503;
        safe_strcpy(res->content_type, "text/plain", sizeof(res->content_type), 0);
        http_response_set_body(res, "# Metrics subsystem not initialized\n");
        return;
    }

    stream_metrics_t *snaps = calloc((size_t)max, sizeof(stream_metrics_t));
    if (!snaps) {
        http_response_set_json_error(res, 500, "Out of memory");
        return;
    }
    int count = metrics_snapshot_all(snaps, max);

    prom_buf_t buf;
    prom_buf_init(&buf, 32768);

    /* --- Stream-level QoS metrics --- */
    prom_buf_append(&buf, "# HELP lightnvr_stream_up Whether stream is connected and producing frames\n");
    prom_buf_append(&buf, "# TYPE lightnvr_stream_up gauge\n");
    for (int i = 0; i < count; i++)
        prom_buf_append(&buf, "lightnvr_stream_up{stream=\"%s\"} %d\n", snaps[i].stream_name, snaps[i].stream_up);

    prom_buf_append(&buf, "# HELP lightnvr_stream_fps Current measured frame rate\n");
    prom_buf_append(&buf, "# TYPE lightnvr_stream_fps gauge\n");
    for (int i = 0; i < count; i++)
        prom_buf_append(&buf, "lightnvr_stream_fps{stream=\"%s\"} %.1f\n", snaps[i].stream_name, snaps[i].current_fps);

    prom_buf_append(&buf, "# HELP lightnvr_stream_bitrate_bps Current measured bitrate in bits per second\n");
    prom_buf_append(&buf, "# TYPE lightnvr_stream_bitrate_bps gauge\n");
    for (int i = 0; i < count; i++)
        prom_buf_append(&buf, "lightnvr_stream_bitrate_bps{stream=\"%s\"} %.0f\n", snaps[i].stream_name, snaps[i].current_bitrate_bps);

    prom_buf_append(&buf, "# HELP lightnvr_stream_frames_total Total frames received since stream start\n");
    prom_buf_append(&buf, "# TYPE lightnvr_stream_frames_total counter\n");
    for (int i = 0; i < count; i++)
        prom_buf_append(&buf, "lightnvr_stream_frames_total{stream=\"%s\"} %llu\n", snaps[i].stream_name, (unsigned long long)snaps[i].frames_total);

    prom_buf_append(&buf, "# HELP lightnvr_stream_frames_dropped Frames dropped due to errors\n");
    prom_buf_append(&buf, "# TYPE lightnvr_stream_frames_dropped counter\n");
    for (int i = 0; i < count; i++)
        prom_buf_append(&buf, "lightnvr_stream_frames_dropped{stream=\"%s\"} %llu\n", snaps[i].stream_name, (unsigned long long)snaps[i].frames_dropped);

    prom_buf_append(&buf, "# HELP lightnvr_stream_reconnects_total Number of RTSP reconnection events\n");
    prom_buf_append(&buf, "# TYPE lightnvr_stream_reconnects_total counter\n");
    for (int i = 0; i < count; i++)
        prom_buf_append(&buf, "lightnvr_stream_reconnects_total{stream=\"%s\"} %llu\n", snaps[i].stream_name, (unsigned long long)snaps[i].reconnects_total);

    prom_buf_append(&buf, "# HELP lightnvr_stream_uptime_seconds Cumulative seconds the stream has been up\n");
    prom_buf_append(&buf, "# TYPE lightnvr_stream_uptime_seconds counter\n");
    for (int i = 0; i < count; i++)
        prom_buf_append(&buf, "lightnvr_stream_uptime_seconds{stream=\"%s\"} %llu\n", snaps[i].stream_name, (unsigned long long)snaps[i].uptime_seconds);

    prom_buf_append(&buf, "# HELP lightnvr_stream_last_frame_ts Timestamp of last received frame\n");
    prom_buf_append(&buf, "# TYPE lightnvr_stream_last_frame_ts gauge\n");
    for (int i = 0; i < count; i++)
        prom_buf_append(&buf, "lightnvr_stream_last_frame_ts{stream=\"%s\"} %lld\n", snaps[i].stream_name, (long long)snaps[i].last_frame_ts);

    prom_buf_append(&buf, "# HELP lightnvr_stream_connection_latency_ms Time from RTSP SETUP to first frame\n");
    prom_buf_append(&buf, "# TYPE lightnvr_stream_connection_latency_ms gauge\n");
    for (int i = 0; i < count; i++)
        prom_buf_append(&buf, "lightnvr_stream_connection_latency_ms{stream=\"%s\"} %.1f\n", snaps[i].stream_name, snaps[i].connection_latency_ms);

    prom_buf_append(&buf, "# HELP lightnvr_stream_error_total Total stream errors by type\n");
    prom_buf_append(&buf, "# TYPE lightnvr_stream_error_total counter\n");
    for (int i = 0; i < count; i++) {
        prom_buf_append(&buf, "lightnvr_stream_error_total{stream=\"%s\",type=\"decode\"} %llu\n", snaps[i].stream_name, (unsigned long long)snaps[i].error_decode);
        prom_buf_append(&buf, "lightnvr_stream_error_total{stream=\"%s\",type=\"timeout\"} %llu\n", snaps[i].stream_name, (unsigned long long)snaps[i].error_timeout);
        prom_buf_append(&buf, "lightnvr_stream_error_total{stream=\"%s\",type=\"protocol\"} %llu\n", snaps[i].stream_name, (unsigned long long)snaps[i].error_protocol);
        prom_buf_append(&buf, "lightnvr_stream_error_total{stream=\"%s\",type=\"io\"} %llu\n", snaps[i].stream_name, (unsigned long long)snaps[i].error_io);
    }

    /* --- Recording/Storage metrics --- */
    prom_buf_append(&buf, "# HELP lightnvr_recording_active Whether recording is active for stream\n");
    prom_buf_append(&buf, "# TYPE lightnvr_recording_active gauge\n");
    for (int i = 0; i < count; i++)
        prom_buf_append(&buf, "lightnvr_recording_active{stream=\"%s\"} %d\n", snaps[i].stream_name, snaps[i].recording_active);

    prom_buf_append(&buf, "# HELP lightnvr_recording_bytes_written Total bytes written to storage\n");
    prom_buf_append(&buf, "# TYPE lightnvr_recording_bytes_written counter\n");
    for (int i = 0; i < count; i++)
        prom_buf_append(&buf, "lightnvr_recording_bytes_written{stream=\"%s\"} %llu\n", snaps[i].stream_name, (unsigned long long)snaps[i].recording_bytes_written);

    prom_buf_append(&buf, "# HELP lightnvr_recording_segments_total Total recording segments created\n");
    prom_buf_append(&buf, "# TYPE lightnvr_recording_segments_total counter\n");
    for (int i = 0; i < count; i++)
        prom_buf_append(&buf, "lightnvr_recording_segments_total{stream=\"%s\"} %llu\n", snaps[i].stream_name, (unsigned long long)snaps[i].recording_segments_total);

    prom_buf_append(&buf, "# HELP lightnvr_recording_gaps_total Recording gaps detected\n");
    prom_buf_append(&buf, "# TYPE lightnvr_recording_gaps_total counter\n");
    for (int i = 0; i < count; i++)
        prom_buf_append(&buf, "lightnvr_recording_gaps_total{stream=\"%s\"} %llu\n", snaps[i].stream_name, (unsigned long long)snaps[i].recording_gaps_total);

    /* Storage metrics (instance-level) */
    storage_health_t storage_health;
    get_storage_health(&storage_health);
    prom_buf_append(&buf, "# HELP lightnvr_storage_used_bytes Total storage consumed by recordings\n");
    prom_buf_append(&buf, "# TYPE lightnvr_storage_used_bytes gauge\n");
    prom_buf_append(&buf, "lightnvr_storage_used_bytes %.0f\n", (double)storage_health.used_space_bytes);
    prom_buf_append(&buf, "# HELP lightnvr_storage_available_bytes Available storage on recording volume\n");
    prom_buf_append(&buf, "# TYPE lightnvr_storage_available_bytes gauge\n");
    prom_buf_append(&buf, "lightnvr_storage_available_bytes %.0f\n", (double)storage_health.free_space_bytes);

    /* --- Instance-level metrics --- */
    prom_buf_append(&buf, "# HELP lightnvr_instance_streams_configured Number of streams configured\n");
    prom_buf_append(&buf, "# TYPE lightnvr_instance_streams_configured gauge\n");
    prom_buf_append(&buf, "lightnvr_instance_streams_configured %d\n", get_total_stream_count());

    int streams_up = 0;
    for (int i = 0; i < count; i++) {
        if (snaps[i].stream_up) streams_up++;
    }
    prom_buf_append(&buf, "# HELP lightnvr_instance_streams_up Number of streams currently up\n");
    prom_buf_append(&buf, "# TYPE lightnvr_instance_streams_up gauge\n");
    prom_buf_append(&buf, "lightnvr_instance_streams_up %d\n", streams_up);

    prom_buf_append(&buf, "# HELP lightnvr_instance_cpu_percent Process CPU usage\n");
    prom_buf_append(&buf, "# TYPE lightnvr_instance_cpu_percent gauge\n");
    if (health_available) {
        const system_health_observation_t *process_cpu = find_observation(
            &health_snapshot, "process.cpu.usage_ratio", NULL);
        if (observation_usable(process_cpu))
            prom_buf_append(&buf, "lightnvr_instance_cpu_percent %.17g\n",
                            process_cpu->value * 100.0);
    }

    prom_buf_append(&buf, "# HELP lightnvr_instance_memory_rss_bytes Resident set size of lightnvr process\n");
    prom_buf_append(&buf, "# TYPE lightnvr_instance_memory_rss_bytes gauge\n");
    if (health_available) {
        append_single_observation(
            &buf, "lightnvr_instance_memory_rss_bytes", "",
            find_observation(&health_snapshot, "process.rss_bytes", NULL));
    }

    prom_buf_append(&buf, "# HELP lightnvr_instance_go2rtc_memory_bytes RSS of go2rtc companion process\n");
    prom_buf_append(&buf, "# TYPE lightnvr_instance_go2rtc_memory_bytes gauge\n");
    prom_buf_append(&buf, "lightnvr_instance_go2rtc_memory_bytes %llu\n", (unsigned long long)get_go2rtc_rss_bytes());

    if (health_available) {
        system_health_stats_t sampler_stats;
        system_health_evaluator_stats_t evaluator_stats;
        system_health_incident_view_t incidents[SYSTEM_HEALTH_MAX_INCIDENTS];
        recording_io_metrics_snapshot_t recording_metrics;
        system_health_process_run_t run;
        system_health_get_stats(&sampler_stats);
        system_health_evaluator_service_get_stats(&evaluator_stats);
        size_t incident_count = system_health_evaluator_service_active_copy(
            incidents, SYSTEM_HEALTH_MAX_INCIDENTS);
        recording_io_metrics_snapshot(&recording_metrics);
        bool run_valid = system_health_evaluator_service_copy_run(&run);
        char *health_metrics = api_metrics_render_system_health(
            &health_snapshot, &sampler_stats, &evaluator_stats, incidents,
            incident_count, &recording_metrics, &run, run_valid);
        if (health_metrics) {
            prom_buf_append(&buf, "%s", health_metrics);
            free(health_metrics);
        }
    }

    char *self_metrics = api_metrics_render_self_observability();
    if (self_metrics) {
        prom_buf_append(&buf, "%s", self_metrics);
        free(self_metrics);
    }

    /* Send response */
    res->status_code = 200;
    safe_strcpy(res->content_type, "text/plain; version=0.0.4; charset=utf-8", sizeof(res->content_type), 0);
    http_response_set_body(res, buf.data ? buf.data : "");

    prom_buf_free(&buf);
    free(snaps);
}

/* ------------------------------------------------------------------ */
/*  POST /api/telemetry/player                                         */
/* ------------------------------------------------------------------ */

void handle_post_player_telemetry(const http_request_t *req, http_response_t *res) {
    if (!req->body || req->body_len == 0) {
        res->status_code = 204;
        return;
    }

    /* Parse JSON body */
    char *body_str = malloc(req->body_len + 1);
    if (!body_str) {
        res->status_code = 204;
        return;
    }
    memcpy(body_str, req->body, req->body_len);
    body_str[req->body_len] = '\0';

    cJSON *json = cJSON_Parse(body_str);
    free(body_str);
    if (!json) {
        res->status_code = 204;
        return;
    }

    player_telemetry_event_t event;
    memset(&event, 0, sizeof(event));

    cJSON *item;
    if ((item = cJSON_GetObjectItem(json, "stream_name")) && cJSON_IsString(item))
        safe_strcpy(event.stream_name, item->valuestring, sizeof(event.stream_name), 0);
    if ((item = cJSON_GetObjectItem(json, "session_id")) && cJSON_IsString(item))
        safe_strcpy(event.session_id, item->valuestring, sizeof(event.session_id), 0);
    if ((item = cJSON_GetObjectItem(json, "transport")) && cJSON_IsString(item))
        safe_strcpy(event.transport, item->valuestring, sizeof(event.transport), 0);
    if ((item = cJSON_GetObjectItem(json, "ttff_ms")) && cJSON_IsNumber(item))
        event.ttff_ms = item->valuedouble;
    if ((item = cJSON_GetObjectItem(json, "rebuffer_count")) && cJSON_IsNumber(item))
        event.rebuffer_count = item->valueint;
    if ((item = cJSON_GetObjectItem(json, "rebuffer_duration_ms")) && cJSON_IsNumber(item))
        event.rebuffer_duration_ms = item->valuedouble;
    if ((item = cJSON_GetObjectItem(json, "resolution_switches")) && cJSON_IsNumber(item))
        event.resolution_switches = item->valueint;
    if ((item = cJSON_GetObjectItem(json, "webrtc_rtt_ms")) && cJSON_IsNumber(item))
        event.webrtc_rtt_ms = item->valuedouble;

    event.timestamp = time(NULL);

    cJSON_Delete(json);

    if (event.stream_name[0] == '\0') {
        res->status_code = 204;
        return;
    }
    if (!httpd_authorize_stream_action(req, res, AUTHZ_LIVE_VIEW,
                                       event.stream_name)) return;

    player_telemetry_record(&event);

    res->status_code = 204;
}
