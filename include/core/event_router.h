#ifndef LIGHTNVR_CORE_EVENT_ROUTER_H
#define LIGHTNVR_CORE_EVENT_ROUTER_H

#include <stdint.h>

#include "core/event_envelope.h"

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
} event_router_stats_t;

/*
 * Evaluate the normalized event against the current enabled route set.
 * DEFAULT preserves the compatibility publisher when no route is configured.
 * MATCH authorizes default-destination enqueue, NO_MATCH suppresses it, and
 * ERROR fails closed when route state cannot be evaluated safely.
 */
event_router_result_t event_router_evaluate(const event_envelope_t *event);

void event_router_get_stats(event_router_stats_t *stats);

/* Release cached route/timezone/inventory state. Safe to call repeatedly. */
void event_router_shutdown(void);

#endif /* LIGHTNVR_CORE_EVENT_ROUTER_H */
