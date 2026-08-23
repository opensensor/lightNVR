#ifndef LIGHTNVR_DB_EVENT_ROUTE_SUPPRESSION_H
#define LIGHTNVR_DB_EVENT_ROUTE_SUPPRESSION_H

#include <stdint.h>

#include "database/db_event_routes.h"

#define EVENT_ROUTE_SUPPRESSION_REASON_MAX 16
#define EVENT_ROUTE_SUPPRESSION_RETENTION_SECONDS (30LL * 24LL * 60LL * 60LL)

typedef enum {
    EVENT_SUPPRESSION_ERROR = -1,
    EVENT_SUPPRESSION_STALE = -2,
    EVENT_SUPPRESSION_PERMIT = 0,
    EVENT_SUPPRESSION_DEBOUNCE = 1,
    EVENT_SUPPRESSION_COOLDOWN = 2,
    EVENT_SUPPRESSION_GROUPING = 3,
    EVENT_SUPPRESSION_RATE = 4
} event_suppression_result_t;

typedef struct {
    int64_t last_observed_at;
    int64_t last_allowed_at;
    int64_t rate_window_started_at;
    int rate_window_count;
    int64_t group_started_at;
    int64_t suppressed_count;
    char last_allowed_event_id[EVENT_ID_MAX];
    char last_reason[EVENT_ROUTE_SUPPRESSION_REASON_MAX];
} event_route_suppression_state_t;

/*
 * Atomically inspect current durable state. A suppressed decision records the
 * observation immediately; a permitted decision remains uncommitted until the
 * caller confirms durable outbox acceptance with record_allowed().
 */
event_suppression_result_t db_event_route_suppression_check(
    const event_route_t *route, const char *event_type, const char *subject,
    int64_t now);

/* Record an accepted outbox event once; duplicate event IDs are idempotent. */
event_suppression_result_t db_event_route_suppression_record_allowed(
    const char *route_uuid, int64_t route_revision, const char *event_type,
    const char *subject, const char *event_id, int64_t now);

int db_event_route_suppression_get(
    const char *route_uuid, const char *event_type, const char *subject,
    event_route_suppression_state_t *state);

int db_event_route_suppression_prune(int64_t updated_before, int limit,
                                     int *deleted_count);

#endif /* LIGHTNVR_DB_EVENT_ROUTE_SUPPRESSION_H */
