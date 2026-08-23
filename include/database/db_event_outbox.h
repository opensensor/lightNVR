#ifndef LIGHTNVR_DB_EVENT_OUTBOX_H
#define LIGHTNVR_DB_EVENT_OUTBOX_H

#include <stddef.h>
#include <stdint.h>
#include <time.h>

#include "core/event_envelope.h"

#define EVENT_OUTBOX_DESTINATION_MAX 64
#define EVENT_OUTBOX_TOPIC_MAX 512
#define EVENT_OUTBOX_ERROR_MAX 512
#define EVENT_OUTBOX_DEFAULT_MAX_ROWS 10000
#define EVENT_OUTBOX_DEFAULT_MAX_BYTES (64LL * 1024LL * 1024LL)
#define EVENT_OUTBOX_DEFAULT_LEASE_SECONDS 30
/* How long a delivered or dead row is kept for operator inspection before the
 * delivery worker reclaims it. Terminal rows are otherwise only evicted under
 * capacity pressure, which would leave the outbox permanently at its row and
 * byte ceiling on a busy installation. */
#define EVENT_OUTBOX_TERMINAL_RETENTION_SECONDS (24LL * 60LL * 60LL)

typedef enum {
    EVENT_OUTBOX_ENQUEUED = 0,
    EVENT_OUTBOX_DUPLICATE = 1,
    EVENT_OUTBOX_FULL = 2,
    EVENT_OUTBOX_ERROR = -1
} event_outbox_enqueue_result_t;

typedef struct {
    int64_t max_rows;
    int64_t max_bytes;
} event_outbox_limits_t;

typedef struct {
    int64_t row_id;
    char event_id[EVENT_ID_MAX];
    char event_source[EVENT_SOURCE_MAX];
    char event_type[EVENT_TYPE_MAX];
    char subject[EVENT_SUBJECT_MAX];
    char destination[EVENT_OUTBOX_DESTINATION_MAX];
    char topic[EVENT_OUTBOX_TOPIC_MAX];
    char *envelope_json;
    size_t envelope_bytes;
    event_severity_t severity;
    int attempt_count;
    int64_t next_attempt_at;
    int64_t expires_at;
    int64_t created_at;
} event_outbox_item_t;

typedef struct {
    int64_t total_rows;
    int64_t total_bytes;
    int64_t pending_rows;
    int64_t delivering_rows;
    int64_t delivered_rows;
    int64_t dead_rows;
    int64_t due_rows;
    int64_t oldest_pending_at;
} event_outbox_stats_t;

/*
 * Persist one validated event/destination pair. Zero limit fields select the
 * bounded defaults. Duplicates are recognized by source + event ID +
 * destination. Higher-severity arrivals may shed older lower-severity pending
 * rows; the number shed is returned to the caller.
 */
event_outbox_enqueue_result_t db_event_outbox_enqueue(
    const event_envelope_t *event, const char *destination,
    const char *topic, const event_outbox_limits_t *limits,
    int64_t *row_id, int *shed_count);

/*
 * Atomically claim the next due row for a destination. Returns 1 when an item
 * is claimed, 0 when none is due, and -1 on error. Expired rows become dead;
 * stale delivery leases are made pending before selection.
 */
int db_event_outbox_claim_due(const char *destination, int64_t now,
                              int lease_seconds,
                              event_outbox_item_t *item);

int db_event_outbox_mark_delivered(int64_t row_id, int64_t delivered_at);
int db_event_outbox_mark_retry(int64_t row_id, int64_t next_attempt_at,
                               const char *error);
int db_event_outbox_mark_dead(int64_t row_id, int64_t failed_at,
                              const char *error);

int db_event_outbox_expire(int64_t now, int *expired_count);
int db_event_outbox_prune_terminal(int64_t updated_before, int limit,
                                   int *deleted_count);
int db_event_outbox_get_stats(const char *destination, int64_t now,
                              event_outbox_stats_t *stats);

void db_event_outbox_item_clear(event_outbox_item_t *item);

#endif /* LIGHTNVR_DB_EVENT_OUTBOX_H */
