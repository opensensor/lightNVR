#ifndef LIGHTNVR_CORE_EVENT_ROUTER_H
#define LIGHTNVR_CORE_EVENT_ROUTER_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include "core/event_envelope.h"
#include "database/db_event_destinations.h"
#include "database/db_event_routes.h"

typedef enum {
    EVENT_ROUTER_ERROR = -1,
    EVENT_ROUTER_NO_MATCH = 0,
    EVENT_ROUTER_MATCH = 1,
    EVENT_ROUTER_DEFAULT = 2
} event_router_result_t;

typedef struct {
    uint64_t cache_reloads;
    uint64_t events_evaluated;
    uint64_t default_events;
    uint64_t matched_events;
    uint64_t unmatched_events;
    uint64_t evaluation_errors;
    uint64_t routes_considered;
    uint64_t type_rejections;
    uint64_t scope_rejections;
    uint64_t predicate_rejections;
    uint64_t schedule_rejections;
    uint64_t destination_disabled_rejections;
    uint64_t debounce_suppressions;
    uint64_t cooldown_suppressions;
    uint64_t grouping_suppressions;
    uint64_t rate_suppressions;
    uint64_t suppression_errors;
} event_router_stats_t;

typedef struct {
    char route_uuid[EVENT_ROUTE_UUID_MAX];
    int64_t route_revision;
    char destination_key[EVENT_ROUTE_DESTINATION_MAX];
    char topic_template[EVENT_DESTINATION_TOPIC_TEMPLATE_MAX];
    bool suppression_pending;
} event_route_delivery_plan_entry_t;

typedef struct {
    char event_id[EVENT_ID_MAX];
    char event_type[EVENT_TYPE_MAX];
    char subject[EVENT_SUBJECT_MAX];
    event_route_delivery_plan_entry_t *entries;
    size_t count;
    size_t capacity;
} event_route_delivery_plan_t;

/*
 * Evaluate the normalized event against the current enabled route set.
 * DEFAULT preserves the compatibility publisher when no route is configured.
 * MATCH authorizes one or more destination enqueues, NO_MATCH suppresses it,
 * and ERROR fails closed when route state cannot be evaluated safely.
 */
event_router_result_t event_router_evaluate(const event_envelope_t *event);

/*
 * Evaluate and retain the suppression-enabled route snapshots that should be
 * committed only after the outbox accepts the event. The caller supplies a
 * zero-initialized plan, owns it, and must release it with
 * event_route_delivery_plan_clear().
 */
event_router_result_t event_router_evaluate_delivery(
    const event_envelope_t *event, event_route_delivery_plan_t *plan);

/* Mark every suppression-enabled planned route as allowed after all relevant
 * destination enqueues return ENQUEUED or DUPLICATE. */
int event_router_record_enqueued(const event_envelope_t *event,
                                 const event_route_delivery_plan_t *plan);

/* Commit suppression only for routes targeting one accepted destination. */
int event_router_record_destination_enqueued(
    const event_envelope_t *event, const event_route_delivery_plan_t *plan,
    const char *destination_key);

void event_route_delivery_plan_clear(event_route_delivery_plan_t *plan);

void event_router_get_stats(event_router_stats_t *stats);

/* Release cached route/timezone/inventory state. Safe to call repeatedly. */
void event_router_shutdown(void);

#endif /* LIGHTNVR_CORE_EVENT_ROUTER_H */
