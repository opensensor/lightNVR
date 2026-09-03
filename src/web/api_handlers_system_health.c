#define _POSIX_C_SOURCE 200809L

#include "web/api_handlers_system_health.h"

#include <cjson/cJSON.h>
#include <errno.h>
#include <math.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#include "core/event_bus.h"
#include "core/event_router.h"
#include "core/mqtt_delivery_worker.h"
#include "core/mqtt_destination_client.h"
#include "core/mqtt_presence.h"
#include "database/db_event_destinations.h"
#include "database/db_event_outbox.h"
#include "database/db_system_health_incidents.h"
#include "telemetry/system_health.h"
#include "telemetry/system_health_evaluator.h"
#include "telemetry/system_health_policy.h"
#include "utils/uuid.h"
#include "web/httpd_utils.h"

#define SYSTEM_HEALTH_API_SCHEMA_VERSION 1
#define SYSTEM_HEALTH_INCIDENT_DEFAULT_LIMIT 50
#define SYSTEM_HEALTH_CURSOR_MAX 96U

static const char *freshness_name(system_health_freshness_t freshness) {
    switch (freshness) {
        case SYSTEM_HEALTH_FRESHNESS_FRESH: return "fresh";
        case SYSTEM_HEALTH_FRESHNESS_STALE: return "stale";
        case SYSTEM_HEALTH_FRESHNESS_UNKNOWN: return "unknown";
    }
    return "unknown";
}

static const char *state_name(system_health_state_t state) {
    switch (state) {
        case SYSTEM_HEALTH_STATE_HEALTHY: return "healthy";
        case SYSTEM_HEALTH_STATE_PENDING: return "pending";
        case SYSTEM_HEALTH_STATE_OPEN: return "open";
        case SYSTEM_HEALTH_STATE_RECOVERING: return "recovering";
        case SYSTEM_HEALTH_STATE_CLOSED: return "closed";
        case SYSTEM_HEALTH_STATE_UNKNOWN: return "unknown";
    }
    return "unknown";
}

static const char *direction_name(system_health_threshold_direction_t direction) {
    switch (direction) {
        case SYSTEM_HEALTH_THRESHOLD_LOWER_IS_WORSE: return "lower_is_worse";
        case SYSTEM_HEALTH_THRESHOLD_HIGHER_IS_WORSE: return "higher_is_worse";
        case SYSTEM_HEALTH_THRESHOLD_NONE: return "none";
    }
    return "none";
}

static const char *reconciliation_name(system_health_reconciliation_t state) {
    switch (state) {
        case SYSTEM_HEALTH_RECONCILIATION_NONE: return "none";
        case SYSTEM_HEALTH_RECONCILIATION_ALERT_PENDING:
            return "alert_pending";
        case SYSTEM_HEALTH_RECONCILIATION_RECOVERY_PENDING:
            return "recovery_pending";
        case SYSTEM_HEALTH_RECONCILIATION_RECONCILED: return "reconciled";
        case SYSTEM_HEALTH_RECONCILIATION_DELIVERY_FAILED:
            return "delivery_failed";
    }
    return "unknown";
}

static uint64_t monotonic_ms(void) {
    struct timespec now = {0};
    if (clock_gettime(CLOCK_MONOTONIC, &now) != 0) return 0U;
    return (uint64_t)now.tv_sec * 1000U + (uint64_t)now.tv_nsec / 1000000U;
}

static int64_t wall_time_seconds(void) {
    time_t now = time(NULL);
    return now > 0 ? (int64_t)now : 0;
}

static void add_nullable_number(cJSON *object, const char *name,
                                bool valid, double value);

static const char *destination_failure_name(
    mqtt_destination_failure_t failure) {
    switch (failure) {
        case MQTT_DESTINATION_FAILURE_NONE: return "none";
        case MQTT_DESTINATION_FAILURE_CONFIGURATION: return "configuration";
        case MQTT_DESTINATION_FAILURE_CONNECTION: return "connection";
        case MQTT_DESTINATION_FAILURE_PUBLICATION: return "publication";
        case MQTT_DESTINATION_FAILURE_COUNT: break;
    }
    return "unknown";
}

static const mqtt_destination_client_stats_t *find_destination_stats(
    const mqtt_destination_client_stats_t *stats, size_t count,
    const char *uuid) {
    for (size_t index = 0; index < count; ++index)
        if (strcmp(stats[index].destination_uuid, uuid) == 0)
            return &stats[index];
    return NULL;
}

static void add_outbox_stats(cJSON *parent, const char *name,
                             const char *destination, int64_t now) {
    cJSON *item = cJSON_CreateObject();
    if (!item) return;
    event_outbox_stats_t stats;
    bool available = db_event_outbox_get_stats(destination, now, &stats) == 0;
    cJSON_AddBoolToObject(item, "available", available);
    if (available) {
        cJSON_AddNumberToObject(item, "rows", (double)stats.total_rows);
        cJSON_AddNumberToObject(item, "bytes", (double)stats.total_bytes);
        cJSON_AddNumberToObject(item, "pending", (double)stats.pending_rows);
        cJSON_AddNumberToObject(item, "delivering",
                                (double)stats.delivering_rows);
        cJSON_AddNumberToObject(item, "delivered",
                                (double)stats.delivered_rows);
        cJSON_AddNumberToObject(item, "dead", (double)stats.dead_rows);
        cJSON_AddNumberToObject(item, "due", (double)stats.due_rows);
        bool has_oldest = stats.pending_rows > 0 &&
                          stats.oldest_pending_at > 0 &&
                          now >= stats.oldest_pending_at;
        add_nullable_number(item, "oldest_pending_age_seconds", has_oldest,
            has_oldest ? (double)(now - stats.oldest_pending_at) : 0.0);
    }
    cJSON_AddItemToObject(parent, name, item);
}

static void append_self_observability(
    cJSON *root, const system_health_stats_t *sampler_stats,
    const system_health_evaluator_stats_t *evaluator_stats,
    const system_health_incident_view_t *incidents, size_t incident_count) {
    cJSON *self = cJSON_CreateObject();
    cJSON *collectors = cJSON_CreateArray();
    cJSON *delivery = cJSON_CreateObject();
    if (!self || !collectors || !delivery) {
        cJSON_Delete(self);
        cJSON_Delete(collectors);
        cJSON_Delete(delivery);
        return;
    }

    system_health_collector_stats_t collector_stats[
        SYSTEM_HEALTH_MAX_COLLECTORS + 1U];
    size_t collector_count = system_health_collector_stats_copy(
        collector_stats, SYSTEM_HEALTH_MAX_COLLECTORS + 1U);
    for (size_t index = 0; index < collector_count; ++index) {
        const system_health_collector_stats_t *stats = &collector_stats[index];
        cJSON *item = cJSON_CreateObject();
        if (!item) continue;
        cJSON_AddStringToObject(item, "collector", stats->name);
        cJSON_AddStringToObject(item, "scope",
                                system_health_scope_name(stats->scope));
        cJSON_AddNumberToObject(item, "tier", (double)stats->tier);
        cJSON_AddNumberToObject(item, "attempts", (double)stats->attempts);
        cJSON_AddNumberToObject(item, "completions",
                                (double)stats->completions);
        cJSON_AddNumberToObject(item, "failures", (double)stats->failures);
        cJSON_AddNumberToObject(item, "timeouts", (double)stats->timeouts);
        cJSON_AddNumberToObject(item, "overlap_skips",
                                (double)stats->overlap_skips);
        cJSON_AddNumberToObject(item, "last_duration_ms",
                                (double)stats->last_duration_ms);
        cJSON_AddNumberToObject(item, "maximum_duration_ms",
                                (double)stats->maximum_duration_ms);
        cJSON_AddBoolToObject(item, "busy", stats->busy);
        cJSON_AddBoolToObject(item, "stale", stats->stale);
        cJSON_AddItemToArray(collectors, item);
    }
    cJSON_AddItemToObject(self, "collectors", collectors);
    cJSON_AddNumberToObject(self, "coverage_overflows",
                            (double)sampler_stats->coverage_overflows);
    cJSON *persistence = cJSON_CreateObject();
    if (persistence) {
        cJSON_AddNumberToObject(persistence, "pending",
                                (double)evaluator_stats->pending_persistence);
        cJSON_AddNumberToObject(persistence, "failures",
                                (double)evaluator_stats->persistence_failures);
        cJSON_AddNumberToObject(persistence, "retries",
                                (double)evaluator_stats->persistence_retries);
        cJSON_AddItemToObject(self, "incident_persistence", persistence);
    }

    event_bus_stats_t bus;
    event_router_stats_t router;
    mqtt_delivery_worker_stats_t worker;
    mqtt_presence_stats_t default_presence;
    event_bus_get_stats(&bus);
    event_router_get_stats(&router);
    mqtt_delivery_worker_get_stats(&worker);
    mqtt_presence_get_stats(&default_presence);
    mqtt_destination_client_stats_t managed[EVENT_DESTINATION_MAX_COUNT];
    size_t managed_count = mqtt_destination_client_list_stats(
        managed, EVENT_DESTINATION_MAX_COUNT);
    event_destination_t profiles[EVENT_DESTINATION_MAX_COUNT];
    int profile_count = db_event_destination_list(
        profiles, EVENT_DESTINATION_MAX_COUNT);
    bool profiles_available = profile_count >= 0;
    if (profile_count < 0) profile_count = 0;
    int64_t now = wall_time_seconds();
    event_outbox_stats_t aggregate_outbox;
    bool outbox_available =
        db_event_outbox_get_stats(NULL, now, &aggregate_outbox) == 0;

    cJSON *bus_json = cJSON_CreateObject();
    if (bus_json) {
        cJSON_AddBoolToObject(bus_json, "running", bus.running);
        cJSON_AddNumberToObject(bus_json, "accepted",
                                (double)bus.accepted_events);
        cJSON_AddNumberToObject(bus_json, "dispatched",
                                (double)bus.dispatched_events);
        cJSON_AddNumberToObject(bus_json, "dropped",
                                (double)bus.dropped_events);
        cJSON_AddNumberToObject(bus_json, "priority_shed",
                                (double)bus.priority_shed_events);
        cJSON_AddNumberToObject(bus_json, "rejected",
                                (double)bus.rejected_events);
        cJSON_AddNumberToObject(bus_json, "handler_failures",
                                (double)bus.handler_failures);
        cJSON_AddNumberToObject(bus_json, "queued_events",
                                (double)bus.queued_events);
        cJSON_AddNumberToObject(bus_json, "queued_bytes",
                                (double)bus.queued_bytes);
        cJSON_AddItemToObject(delivery, "bus", bus_json);
    }
    cJSON *router_json = cJSON_CreateObject();
    if (router_json) {
        cJSON_AddNumberToObject(router_json, "events_evaluated",
                                (double)router.events_evaluated);
        cJSON_AddNumberToObject(router_json, "matched",
                                (double)router.matched_events);
        cJSON_AddNumberToObject(router_json, "unmatched",
                                (double)router.unmatched_events);
        cJSON_AddNumberToObject(router_json, "errors",
                                (double)router.evaluation_errors);
        cJSON_AddNumberToObject(router_json, "rate_suppressions",
                                (double)router.rate_suppressions);
        cJSON_AddItemToObject(delivery, "router", router_json);
    }
    cJSON *worker_json = cJSON_CreateObject();
    if (worker_json) {
        cJSON_AddBoolToObject(worker_json, "running", worker.running);
        cJSON_AddNumberToObject(worker_json, "enqueued",
                                (double)worker.enqueued);
        cJSON_AddNumberToObject(worker_json, "rejected_full",
                                (double)worker.rejected_full);
        cJSON_AddNumberToObject(worker_json, "enqueue_errors",
                                (double)worker.enqueue_errors);
        cJSON_AddNumberToObject(worker_json, "attempted",
                                (double)worker.attempted);
        cJSON_AddNumberToObject(worker_json, "delivered",
                                (double)worker.delivered);
        cJSON_AddNumberToObject(worker_json, "retried",
                                (double)worker.retried);
        cJSON_AddNumberToObject(worker_json, "dead", (double)worker.dead);
        cJSON_AddNumberToObject(worker_json, "profile_errors",
                                (double)worker.profile_errors);
        cJSON_AddItemToObject(delivery, "worker", worker_json);
    }
    add_outbox_stats(delivery, "outbox", NULL, now);

    cJSON *destinations = cJSON_CreateArray();
    size_t configured = default_presence.configured ? 1U : 0U;
    size_t connected = default_presence.configured && default_presence.connected
        ? 1U : 0U;
    bool only_destination_failed = false;
    if (destinations) {
        if (default_presence.configured) {
            cJSON *item = cJSON_CreateObject();
            if (item) {
                cJSON_AddStringToObject(item, "destination", "default");
                cJSON_AddBoolToObject(item, "connected",
                                      default_presence.connected);
                cJSON_AddNumberToObject(item, "connections",
                                        (double)default_presence.connections);
                cJSON_AddNumberToObject(item, "reconnects",
                                        (double)default_presence.reconnects);
                cJSON_AddNumberToObject(item, "publish_failures",
                                      (double)default_presence.publish_failures);
                add_outbox_stats(item, "outbox",
                                 MQTT_EVENT_OUTBOX_DESTINATION, now);
                cJSON_AddItemToArray(destinations, item);
            }
        }
        for (int index = 0; index < profile_count; ++index) {
            if (!profiles[index].enabled) continue;
            configured++;
            const mqtt_destination_client_stats_t *stats =
                find_destination_stats(managed, managed_count,
                                       profiles[index].uuid);
            if (stats && stats->connected) connected++;
            cJSON *item = cJSON_CreateObject();
            if (!item) continue;
            cJSON_AddStringToObject(item, "destination",
                                    profiles[index].uuid);
            cJSON_AddBoolToObject(item, "runtime_available", stats != NULL);
            cJSON_AddBoolToObject(item, "connected",
                                  stats && stats->connected);
            cJSON_AddNumberToObject(item, "connections",
                                    stats ? (double)stats->connections : 0.0);
            cJSON_AddNumberToObject(item, "reconnects",
                                    stats ? (double)stats->reconnects : 0.0);
            cJSON_AddNumberToObject(item, "connection_failures",
                stats ? (double)stats->connection_failures : 0.0);
            cJSON_AddNumberToObject(item, "publish_failures",
                stats ? (double)stats->publish_failures : 0.0);
            cJSON_AddStringToObject(item, "last_failure",
                stats ? destination_failure_name(stats->last_failure)
                      : "configuration");
            char destination_key[EVENT_DESTINATION_KEY_MAX];
            if (db_event_destination_make_key(profiles[index].uuid,
                                              destination_key) == 0)
                add_outbox_stats(item, "outbox", destination_key, now);
            cJSON_AddItemToArray(destinations, item);
        }
        cJSON_AddItemToObject(delivery, "destinations", destinations);
    }
    if (configured == 1U) {
        if (default_presence.configured) {
            only_destination_failed = !default_presence.connected &&
                default_presence.publish_failures > 0U;
        } else {
            for (int index = 0; index < profile_count; ++index) {
                if (!profiles[index].enabled) continue;
                const mqtt_destination_client_stats_t *stats =
                    find_destination_stats(managed, managed_count,
                                           profiles[index].uuid);
                only_destination_failed = !stats ||
                    (!stats->connected && stats->last_failure !=
                                               MQTT_DESTINATION_FAILURE_NONE);
                break;
            }
        }
    }
    bool outbox_full = outbox_available &&
        (aggregate_outbox.total_rows >= EVENT_OUTBOX_DEFAULT_MAX_ROWS ||
         aggregate_outbox.total_bytes >= EVENT_OUTBOX_DEFAULT_MAX_BYTES);
    bool undeliverable = outbox_available && aggregate_outbox.pending_rows > 0 &&
                         connected == 0U;
    bool degraded = outbox_full || only_destination_failed || undeliverable ||
                    (outbox_available && aggregate_outbox.dead_rows > 0) ||
                    (!profiles_available && managed_count > 0U);
    cJSON_AddBoolToObject(delivery, "degraded", degraded);
    cJSON_AddBoolToObject(delivery, "circular_report_path",
                          configured == 1U &&
                          (only_destination_failed || outbox_full ||
                           undeliverable));
    cJSON_AddBoolToObject(delivery, "outbox_full", outbox_full);
    cJSON_AddBoolToObject(delivery, "all_destinations_unavailable",
                          undeliverable);
    cJSON_AddItemToObject(self, "event_delivery", delivery);

    int64_t now_ms = now > 0 ? now * 1000 : 0;
    (void)incidents;
    (void)incident_count;
    int64_t oldest_pending_ms = evaluator_stats->oldest_pending_wall_time_ms;
    add_nullable_number(self, "incident_persistence_retry_age_ms",
        oldest_pending_ms > 0 && now_ms >= oldest_pending_ms,
        oldest_pending_ms > 0 && now_ms >= oldest_pending_ms
            ? (double)(now_ms - oldest_pending_ms) : 0.0);
    cJSON_AddItemToObject(root, "self_observability", self);
}

static void add_nullable_number(cJSON *object, const char *name,
                                bool valid, double value) {
    if (valid && isfinite(value))
        cJSON_AddNumberToObject(object, name, value);
    else
        cJSON_AddNullToObject(object, name);
}

static cJSON *observation_to_json(const system_health_observation_t *value) {
    cJSON *item = cJSON_CreateObject();
    if (!item) return NULL;
    cJSON_AddStringToObject(item, "metric", value->metric);
    cJSON_AddStringToObject(item, "resource", value->resource_id);
    cJSON_AddStringToObject(item, "scope",
                            system_health_scope_name(value->scope));
    cJSON_AddStringToObject(item, "capability",
                            system_health_capability_name(value->capability));
    cJSON_AddStringToObject(item, "freshness",
                            freshness_name(value->freshness));
    cJSON_AddStringToObject(item, "unit",
                            system_health_unit_name(value->unit));
    add_nullable_number(item, "value", value->value_valid, value->value);
    cJSON_AddNumberToObject(item, "sampled_monotonic_ms",
                            (double)value->sampled_monotonic_ms);
    cJSON_AddNumberToObject(item, "observed_at_ms",
                            (double)value->observed_wall_time_ms);
    return item;
}

static cJSON *policy_to_json(const system_health_condition_policy_t *rule) {
    cJSON *item = cJSON_CreateObject();
    if (!item) return NULL;
    cJSON_AddStringToObject(item, "condition",
                            system_health_condition_code(rule->condition));
    cJSON_AddBoolToObject(item, "enabled", rule->enabled);
    cJSON_AddBoolToObject(item, "overridden", rule->overridden);
    cJSON_AddStringToObject(item, "direction", direction_name(rule->direction));
    cJSON_AddStringToObject(item, "unit", system_health_unit_name(rule->unit));
    cJSON_AddNumberToObject(item, "warning_threshold",
                            rule->warning_threshold);
    cJSON_AddNumberToObject(item, "critical_threshold",
                            rule->critical_threshold);
    cJSON_AddNumberToObject(item, "recovery_threshold",
                            rule->recovery_threshold);
    cJSON_AddNumberToObject(item, "warning_for_seconds",
                            rule->warning_for_seconds);
    cJSON_AddNumberToObject(item, "critical_for_seconds",
                            rule->critical_for_seconds);
    cJSON_AddNumberToObject(item, "recovery_for_seconds",
                            rule->recovery_for_seconds);
    return item;
}

static cJSON *incident_view_to_json(
    const system_health_incident_view_t *incident,
    const system_health_policy_t *policy) {
    cJSON *item = cJSON_CreateObject();
    cJSON *observation = observation_to_json(&incident->observation);
    if (!item || !observation) {
        cJSON_Delete(item);
        cJSON_Delete(observation);
        return NULL;
    }
    const char *code = system_health_condition_code(incident->condition);
    cJSON_AddStringToObject(item, "incident_id", incident->incident_id);
    cJSON_AddStringToObject(item, "condition", code ? code : "unknown");
    cJSON_AddStringToObject(item, "subject", incident->subject);
    cJSON_AddStringToObject(item, "scope",
                            system_health_scope_name(incident->scope));
    cJSON_AddStringToObject(item, "state", state_name(incident->state));
    cJSON_AddStringToObject(item, "severity",
                            system_health_severity_name(incident->severity));
    cJSON_AddNumberToObject(item, "first_observed_at_ms",
                            (double)incident->first_observed_at_ms);
    cJSON_AddNumberToObject(item, "last_observed_at_ms",
                            (double)incident->last_observed_at_ms);
    cJSON_AddBoolToObject(item, "persistence_pending",
                          incident->persistence_pending);
    cJSON_AddItemToObject(item, "observation", observation);
    if (incident->condition >= 0 &&
        incident->condition < SYSTEM_HEALTH_CONDITION_COUNT) {
        cJSON *thresholds = policy_to_json(
            &policy->conditions[incident->condition]);
        if (thresholds) cJSON_AddItemToObject(item, "thresholds", thresholds);
    }
    return item;
}

static void add_count_object(cJSON *parent, const char *name,
                             const char *const *names, const size_t *counts,
                             size_t count) {
    cJSON *object = cJSON_CreateObject();
    if (!object) return;
    for (size_t index = 0; index < count; ++index)
        cJSON_AddNumberToObject(object, names[index], (double)counts[index]);
    cJSON_AddItemToObject(parent, name, object);
}

static const char *effective_visibility(const size_t *scope_counts) {
    if (scope_counts[SYSTEM_HEALTH_SCOPE_CONTAINER] > 0U) return "container";
    if (scope_counts[SYSTEM_HEALTH_SCOPE_HOST] > 0U) return "host";
    if (scope_counts[SYSTEM_HEALTH_SCOPE_PROCESS] > 0U) return "process";
    return "unknown";
}

static bool send_json(http_response_t *res, cJSON *root) {
    char *encoded = cJSON_PrintUnformatted(root);
    cJSON_Delete(root);
    if (!encoded) {
        http_response_set_json_error(res, 500,
                                     "Failed to serialize system health");
        return false;
    }
    int result = http_response_set_json(res, 200, encoded);
    cJSON_free(encoded);
    return result == 0;
}

void handle_get_system_health(const http_request_t *req, http_response_t *res) {
    if (!httpd_authorize_global_action(req, res, AUTHZ_SYSTEM_ADMIN)) return;

    system_health_snapshot_t snapshot;
    memset(&snapshot, 0, sizeof(snapshot));
    bool has_snapshot = system_health_snapshot_copy(&snapshot);
    system_health_policy_t policy;
    memset(&policy, 0, sizeof(policy));
    (void)system_health_policy_snapshot(&policy);
    system_health_stats_t sampler_stats;
    system_health_get_stats(&sampler_stats);
    system_health_evaluator_stats_t evaluator_stats;
    system_health_evaluator_service_get_stats(&evaluator_stats);
    system_health_incident_view_t incidents[SYSTEM_HEALTH_MAX_INCIDENTS];
    size_t incident_count = system_health_evaluator_service_active_copy(
        incidents, SYSTEM_HEALTH_MAX_INCIDENTS);
    system_health_summary_t summaries[SYSTEM_HEALTH_RING_SAMPLES];
    size_t summary_count = system_health_summary_copy(
        summaries, SYSTEM_HEALTH_RING_SAMPLES);

    size_t capability_counts[SYSTEM_HEALTH_CAPABILITY_COUNT] = {0};
    size_t scope_counts[SYSTEM_HEALTH_SCOPE_COUNT] = {0};
    size_t unavailable_count = 0U;
    if (has_snapshot) {
        for (size_t index = 0; index < snapshot.observation_count; ++index) {
            const system_health_observation_t *observation =
                &snapshot.observations[index];
            if (observation->capability >= 0 &&
                observation->capability < SYSTEM_HEALTH_CAPABILITY_COUNT)
                capability_counts[observation->capability]++;
            if (observation->scope >= 0 &&
                observation->scope < SYSTEM_HEALTH_SCOPE_COUNT)
                scope_counts[observation->scope]++;
            if (!observation->value_valid) unavailable_count++;
        }
    }

    system_health_severity_t overall = SYSTEM_HEALTH_SEVERITY_NONE;
    if (has_snapshot) {
        for (size_t index = 0; index < incident_count; ++index)
            if (incidents[index].severity > overall)
                overall = incidents[index].severity;
    }

    cJSON *root = cJSON_CreateObject();
    cJSON *visibility = cJSON_CreateObject();
    cJSON *generation = cJSON_CreateObject();
    cJSON *coverage = cJSON_CreateObject();
    cJSON *observations = cJSON_CreateArray();
    cJSON *thresholds = cJSON_CreateArray();
    cJSON *active = cJSON_CreateArray();
    cJSON *recent_samples = cJSON_CreateArray();
    if (!root || !visibility || !generation || !coverage || !observations ||
        !thresholds || !active || !recent_samples) {
        cJSON_Delete(root);
        cJSON_Delete(visibility);
        cJSON_Delete(generation);
        cJSON_Delete(coverage);
        cJSON_Delete(observations);
        cJSON_Delete(thresholds);
        cJSON_Delete(active);
        cJSON_Delete(recent_samples);
        http_response_set_json_error(res, 500,
                                     "Failed to build system health response");
        return;
    }

    cJSON_AddNumberToObject(root, "schema_version",
                            SYSTEM_HEALTH_API_SCHEMA_VERSION);
    cJSON_AddStringToObject(root, "overall_state",
        !has_snapshot ? "unknown" :
        overall == SYSTEM_HEALTH_SEVERITY_NONE ? "healthy" :
        system_health_severity_name(overall));

    cJSON_AddStringToObject(visibility, "effective_scope",
                            effective_visibility(scope_counts));
    cJSON_AddBoolToObject(visibility, "host_hardware_visible",
                          scope_counts[SYSTEM_HEALTH_SCOPE_DEVICE] > 0U);
    cJSON_AddStringToObject(visibility, "coverage_boundary",
        scope_counts[SYSTEM_HEALTH_SCOPE_CONTAINER] > 0U
            ? "container_and_visible_mounts"
            : scope_counts[SYSTEM_HEALTH_SCOPE_HOST] > 0U
                ? "host_and_visible_mounts" : "unknown");
    cJSON_AddItemToObject(root, "visibility", visibility);

    uint64_t now = monotonic_ms();
    bool age_valid = has_snapshot && now >= snapshot.completed_monotonic_ms;
    uint64_t age = age_valid ? now - snapshot.completed_monotonic_ms : 0U;
    uint64_t stale_after =
        (uint64_t)policy.settings.fast_interval_seconds * 3000U;
    cJSON_AddBoolToObject(generation, "available", has_snapshot);
    add_nullable_number(generation, "sequence", has_snapshot,
                        (double)snapshot.sequence);
    add_nullable_number(generation, "completed_at_ms", has_snapshot,
                        (double)snapshot.completed_wall_time_ms);
    add_nullable_number(generation, "age_ms", age_valid, (double)age);
    cJSON_AddStringToObject(generation, "freshness",
        !has_snapshot || !age_valid ? "unknown" :
        stale_after > 0U && age > stale_after ? "stale" : "fresh");
    cJSON_AddItemToObject(root, "snapshot", generation);

    static const char *const capability_names[] = {
        "available", "unsupported", "permission_denied", "stale", "error"
    };
    static const char *const scope_names[] = {
        "process", "container", "host", "filesystem", "device"
    };
    add_count_object(coverage, "capabilities", capability_names,
                     capability_counts, SYSTEM_HEALTH_CAPABILITY_COUNT);
    add_count_object(coverage, "scopes", scope_names, scope_counts,
                     SYSTEM_HEALTH_SCOPE_COUNT);
    uint64_t incomplete = (uint64_t)unavailable_count +
        (uint64_t)snapshot.observations_dropped +
        sampler_stats.collection_errors + sampler_stats.collection_timeouts +
        sampler_stats.abandoned_helpers + sampler_stats.coverage_overflows;
    cJSON_AddBoolToObject(coverage, "complete",
                          has_snapshot && incomplete == 0U &&
                          sampler_stats.collection_errors == 0U &&
                          sampler_stats.collection_timeouts == 0U);
    cJSON_AddNumberToObject(coverage, "incomplete_count", (double)incomplete);
    cJSON_AddNumberToObject(coverage, "unavailable_observations",
                            (double)unavailable_count);
    cJSON_AddNumberToObject(coverage, "observations_dropped",
                            (double)snapshot.observations_dropped);
    cJSON_AddNumberToObject(coverage, "collection_errors",
                            (double)sampler_stats.collection_errors);
    cJSON_AddNumberToObject(coverage, "collection_timeouts",
                            (double)sampler_stats.collection_timeouts);
    cJSON_AddNumberToObject(coverage, "abandoned_helpers",
                            (double)sampler_stats.abandoned_helpers);
    cJSON_AddNumberToObject(coverage, "coverage_overflows",
                            (double)sampler_stats.coverage_overflows);
    cJSON_AddItemToObject(root, "coverage", coverage);

    for (size_t index = 0; index < snapshot.observation_count; ++index) {
        cJSON *item = observation_to_json(&snapshot.observations[index]);
        if (item) cJSON_AddItemToArray(observations, item);
    }
    cJSON_AddItemToObject(root, "observations", observations);

    for (int index = 0; index < SYSTEM_HEALTH_CONDITION_COUNT; ++index) {
        cJSON *item = policy_to_json(&policy.conditions[index]);
        if (item) cJSON_AddItemToArray(thresholds, item);
    }
    cJSON_AddItemToObject(root, "thresholds", thresholds);

    for (size_t index = 0; index < incident_count; ++index) {
        cJSON *item = incident_view_to_json(&incidents[index], &policy);
        if (item) cJSON_AddItemToArray(active, item);
    }
    cJSON_AddItemToObject(root, "active_incidents", active);
    for (size_t index = 0; index < summary_count; ++index) {
        cJSON *item = cJSON_CreateObject();
        if (!item) continue;
        cJSON_AddNumberToObject(item, "sequence",
                                (double)summaries[index].sequence);
        cJSON_AddNumberToObject(item, "completed_at_ms",
                                (double)summaries[index].completed_wall_time_ms);
        cJSON_AddNumberToObject(item, "observation_count",
                                (double)summaries[index].observation_count);
        cJSON_AddNumberToObject(item, "observations_dropped",
                                (double)summaries[index].observations_dropped);
        cJSON_AddItemToArray(recent_samples, item);
    }
    cJSON_AddItemToObject(root, "recent_samples", recent_samples);
    cJSON *evaluator = cJSON_CreateObject();
    if (evaluator) {
        cJSON_AddNumberToObject(evaluator, "tracked_conditions",
                                (double)evaluator_stats.tracked_conditions);
        cJSON_AddNumberToObject(evaluator, "active_incidents",
                                (double)evaluator_stats.active_incidents);
        cJSON_AddNumberToObject(evaluator, "pending_persistence",
                                (double)evaluator_stats.pending_persistence);
        cJSON_AddItemToObject(root, "evaluator", evaluator);
    }
    append_self_observability(root, &sampler_stats, &evaluator_stats,
                              incidents, incident_count);
    (void)send_json(res, root);
}

static bool parse_limit(const http_request_t *req, int *limit,
                        http_response_t *res) {
    char value[32];
    if (http_request_get_query_param(req, "limit", value, sizeof(value)) < 0) {
        *limit = SYSTEM_HEALTH_INCIDENT_DEFAULT_LIMIT;
        return true;
    }
    errno = 0;
    char *end = NULL;
    long parsed = strtol(value, &end, 10);
    if (errno == ERANGE || !end || end == value || *end != '\0' || parsed < 1 ||
        parsed > SYSTEM_HEALTH_INCIDENT_PAGE_MAX) {
        http_response_set_json_error(res, 400,
                                     "limit must be between 1 and 100");
        return false;
    }
    *limit = (int)parsed;
    return true;
}

static bool parse_include_closed(const http_request_t *req, bool *value,
                                 http_response_t *res) {
    char text[16];
    if (http_request_get_query_param(req, "include_closed", text,
                                     sizeof(text)) < 0) {
        *value = true;
        return true;
    }
    if (strcmp(text, "true") == 0 || strcmp(text, "1") == 0) {
        *value = true;
        return true;
    }
    if (strcmp(text, "false") == 0 || strcmp(text, "0") == 0) {
        *value = false;
        return true;
    }
    http_response_set_json_error(res, 400,
                                 "include_closed must be true or false");
    return false;
}

static bool parse_cursor(const http_request_t *req,
                         system_health_incident_cursor_t *cursor,
                         http_response_t *res) {
    char value[SYSTEM_HEALTH_CURSOR_MAX];
    memset(cursor, 0, sizeof(*cursor));
    if (http_request_get_query_param(req, "cursor", value,
                                     sizeof(value)) < 0)
        return true;
    if (strncmp(value, "v1:", 3) != 0) goto invalid;
    char *uuid = strchr(value + 3, ':');
    if (!uuid) goto invalid;
    *uuid++ = '\0';
    errno = 0;
    char *end = NULL;
    long long timestamp = strtoll(value + 3, &end, 10);
    if (errno == ERANGE || !end || *end != '\0' || timestamp <= 0 ||
        !lightnvr_uuid_is_valid(uuid))
        goto invalid;
    cursor->valid = true;
    cursor->last_seen_at_ms = (int64_t)timestamp;
    snprintf(cursor->uuid, sizeof(cursor->uuid), "%s", uuid);
    return true;
invalid:
    http_response_set_json_error(res, 400, "Invalid incident cursor");
    return false;
}

static cJSON *safe_stored_observation(
    const system_health_incident_record_t *incident) {
    cJSON *safe = cJSON_CreateObject();
    cJSON *stored = cJSON_ParseWithOpts(incident->observation_json, NULL, true);
    if (!safe) {
        cJSON_Delete(stored);
        return NULL;
    }
    const cJSON *metric = cJSON_IsObject(stored)
        ? cJSON_GetObjectItemCaseSensitive(stored, "metric") : NULL;
    const cJSON *value = cJSON_IsObject(stored)
        ? cJSON_GetObjectItemCaseSensitive(stored, "value") : NULL;
    const cJSON *unit = cJSON_IsObject(stored)
        ? cJSON_GetObjectItemCaseSensitive(stored, "unit") : NULL;
    const cJSON *capability = cJSON_IsObject(stored)
        ? cJSON_GetObjectItemCaseSensitive(stored, "capability") : NULL;
    const char *safe_metric = incident->condition_code;
    if (cJSON_IsString(metric)) {
        size_t length = strnlen(metric->valuestring,
                                SYSTEM_HEALTH_METRIC_LENGTH);
        bool logical = length > 0U && length < SYSTEM_HEALTH_METRIC_LENGTH;
        for (size_t index = 0; logical && index < length; ++index) {
            unsigned char current = (unsigned char)metric->valuestring[index];
            logical = (current >= 'a' && current <= 'z') ||
                      (current >= 'A' && current <= 'Z') ||
                      (current >= '0' && current <= '9') || current == '_' ||
                      current == '.' || current == ':' || current == '-';
        }
        if (logical) safe_metric = metric->valuestring;
    }
    const char *safe_unit = "none";
    if (cJSON_IsString(unit)) {
        for (int candidate = SYSTEM_HEALTH_UNIT_NONE;
             candidate <= SYSTEM_HEALTH_UNIT_BOOLEAN; ++candidate) {
            const char *name = system_health_unit_name(
                (system_health_unit_t)candidate);
            if (strcmp(name, unit->valuestring) == 0) {
                safe_unit = name;
                break;
            }
        }
    }
    const char *safe_capability = cJSON_IsNumber(value)
        ? "available" : "error";
    if (cJSON_IsString(capability)) {
        for (int candidate = SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
             candidate < SYSTEM_HEALTH_CAPABILITY_COUNT; ++candidate) {
            const char *name = system_health_capability_name(
                (system_health_capability_t)candidate);
            if (strcmp(name, capability->valuestring) == 0) {
                safe_capability = name;
                break;
            }
        }
    }
    cJSON_AddStringToObject(safe, "metric", safe_metric);
    cJSON_AddStringToObject(safe, "resource", incident->subject);
    cJSON_AddStringToObject(safe, "scope",
                            system_health_scope_name(incident->scope));
    if (cJSON_IsNumber(value) && isfinite(value->valuedouble))
        cJSON_AddNumberToObject(safe, "value", value->valuedouble);
    else
        cJSON_AddNullToObject(safe, "value");
    cJSON_AddStringToObject(safe, "unit", safe_unit);
    cJSON_AddStringToObject(safe, "capability", safe_capability);
    cJSON_Delete(stored);
    return safe;
}

static cJSON *stored_incident_to_json(
    const system_health_incident_record_t *incident) {
    cJSON *item = cJSON_CreateObject();
    cJSON *observation = safe_stored_observation(incident);
    if (!item || !observation) {
        cJSON_Delete(item);
        cJSON_Delete(observation);
        return NULL;
    }
    cJSON_AddStringToObject(item, "incident_id", incident->uuid);
    cJSON_AddStringToObject(item, "condition", incident->condition_code);
    cJSON_AddStringToObject(item, "subject", incident->subject);
    cJSON_AddStringToObject(item, "scope",
                            system_health_scope_name(incident->scope));
    cJSON_AddStringToObject(item, "state", state_name(incident->state));
    cJSON_AddStringToObject(item, "severity",
                            system_health_severity_name(incident->severity));
    cJSON_AddNumberToObject(item, "first_observed_at_ms",
                            (double)incident->first_seen_at_ms);
    cJSON_AddNumberToObject(item, "last_observed_at_ms",
                            (double)incident->last_seen_at_ms);
    if (incident->closed_at_ms > 0)
        cJSON_AddNumberToObject(item, "closed_at_ms",
                                (double)incident->closed_at_ms);
    else
        cJSON_AddNullToObject(item, "closed_at_ms");
    cJSON_AddStringToObject(item, "reconciliation",
                            reconciliation_name(incident->reconciliation));
    cJSON_AddNumberToObject(item, "revision", (double)incident->revision);
    cJSON_AddItemToObject(item, "observation", observation);
    return item;
}

void handle_get_system_health_incidents(const http_request_t *req,
                                        http_response_t *res) {
    if (!httpd_authorize_global_action(req, res, AUTHZ_SYSTEM_ADMIN)) return;
    int limit;
    bool include_closed;
    system_health_incident_cursor_t cursor;
    if (!parse_limit(req, &limit, res) ||
        !parse_include_closed(req, &include_closed, res) ||
        !parse_cursor(req, &cursor, res))
        return;

    system_health_incident_record_t *records =
        calloc((size_t)limit, sizeof(*records));
    if (!records) {
        http_response_set_json_error(res, 500, "Out of memory");
        return;
    }
    system_health_incident_cursor_t next;
    int count = db_system_health_incident_list(
        include_closed, cursor.valid ? &cursor : NULL, records, limit, &next);
    if (count < 0) {
        free(records);
        http_response_set_json_error(res, 500,
                                     "Failed to read incident history");
        return;
    }

    cJSON *root = cJSON_CreateObject();
    cJSON *items = cJSON_CreateArray();
    if (!root || !items) {
        cJSON_Delete(root);
        cJSON_Delete(items);
        free(records);
        http_response_set_json_error(res, 500,
                                     "Failed to build incident history");
        return;
    }
    cJSON_AddNumberToObject(root, "schema_version",
                            SYSTEM_HEALTH_API_SCHEMA_VERSION);
    cJSON_AddNumberToObject(root, "count", count);
    for (int index = 0; index < count; ++index) {
        cJSON *item = stored_incident_to_json(&records[index]);
        if (item) cJSON_AddItemToArray(items, item);
    }
    free(records);
    cJSON_AddItemToObject(root, "incidents", items);
    if (next.valid) {
        char encoded[SYSTEM_HEALTH_CURSOR_MAX];
        snprintf(encoded, sizeof(encoded), "v1:%lld:%s",
                 (long long)next.last_seen_at_ms, next.uuid);
        cJSON_AddStringToObject(root, "next_cursor", encoded);
    } else {
        cJSON_AddNullToObject(root, "next_cursor");
    }
    (void)send_json(res, root);
}
