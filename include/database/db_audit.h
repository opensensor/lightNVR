#ifndef LIGHTNVR_DB_AUDIT_H
#define LIGHTNVR_DB_AUDIT_H

#include <stdbool.h>
#include <stdint.h>

#define AUDIT_EVENT_UUID_MAX 37
#define AUDIT_REQUEST_ID_MAX 65
#define AUDIT_ACTION_MAX 96
#define AUDIT_TARGET_TYPE_MAX 48
#define AUDIT_OUTCOME_MAX 16
#define AUDIT_USERNAME_MAX 64
#define AUDIT_AUTH_METHOD_MAX 24
#define AUDIT_REMOTE_ADDRESS_MAX 64
#define AUDIT_DETAILS_MAX 8192
#define AUDIT_QUERY_VALUE_MAX 128
#define AUDIT_PAGE_SIZE_MAX 1000
#define AUDIT_RETENTION_DEFAULT_DAYS 365
#define AUDIT_RETENTION_MAX_DAYS 3650

/* Pruning runs from the audit insert path with the global database mutex
 * held, so it deletes in bounded batches instead of one open-ended DELETE.
 * The row cap keeps a single statement cheap; the time budget limits how many
 * batches one pass runs. The budget is checked between statements, so it is a
 * soft bound -- a single batch already in flight can overrun it. */
#define AUDIT_PRUNE_BATCH_ROWS 2000
#define AUDIT_PRUNE_BUDGET_MS 250
/* Eligibility interval for the next automatic prune, and the shortened
 * interval used while a backlog is still draining. Neither is a scheduled
 * job: the prune is driven from the audit insert path, so a quiet install
 * with no audit traffic will not drain a backlog until writes resume. */
#define AUDIT_PRUNE_INTERVAL_SECONDS 3600
#define AUDIT_PRUNE_BACKLOG_INTERVAL_SECONDS 60

/*
 * Batch delete used by the automatic prune.
 *
 * "ORDER BY occurred_at, id" rather than "ORDER BY id" is load-bearing: the
 * latter makes SQLite pick SCAN audit_events for the inner query, so the
 * common case -- nothing expired -- walks every surviving row while the
 * global database mutex is held. Ordering by the index's leading column lets
 * it use idx_audit_events_occurred as a covering index instead. Measured on
 * 1M unexpired rows: SCAN 48ms vs covering index 1ms.
 *
 * The subquery form rather than "DELETE ... LIMIT" avoids depending on
 * SQLITE_ENABLE_UPDATE_DELETE_LIMIT, which is not enabled in every SQLite
 * build this project links against.
 */
#define AUDIT_PRUNE_BATCH_SQL \
    "DELETE FROM audit_events WHERE id IN (" \
    "SELECT id FROM audit_events WHERE occurred_at < ? " \
    "ORDER BY occurred_at, id LIMIT ?);"

typedef struct {
    const char *request_id;
    int64_t principal_user_id;
    const char *principal_username;
    const char *auth_method;
    const char *api_token_uuid;
    const char *action;
    const char *target_type;
    const char *target_uuid;
    const char *outcome;
    const char *remote_address;
    const char *details_json;
    int64_t occurred_at;
} audit_event_input_t;

typedef struct {
    int64_t id;
    char uuid[AUDIT_EVENT_UUID_MAX];
    int64_t occurred_at;
    char request_id[AUDIT_REQUEST_ID_MAX];
    int64_t principal_user_id;
    char principal_username[AUDIT_USERNAME_MAX];
    char auth_method[AUDIT_AUTH_METHOD_MAX];
    char api_token_uuid[AUDIT_EVENT_UUID_MAX];
    char action[AUDIT_ACTION_MAX];
    char target_type[AUDIT_TARGET_TYPE_MAX];
    char target_uuid[AUDIT_QUERY_VALUE_MAX];
    char outcome[AUDIT_OUTCOME_MAX];
    char remote_address[AUDIT_REMOTE_ADDRESS_MAX];
    char details_json[AUDIT_DETAILS_MAX];
} audit_event_t;

typedef struct {
    int page;
    int page_size;
    int64_t since;
    int64_t until;
    int64_t principal_user_id;
    char action[AUDIT_ACTION_MAX];
    char outcome[AUDIT_OUTCOME_MAX];
    char target_uuid[AUDIT_QUERY_VALUE_MAX];
    char request_id[AUDIT_REQUEST_ID_MAX];
} audit_query_t;

typedef struct {
    audit_event_t *events;
    int count;
    int64_t total;
    int page;
    int page_size;
} audit_page_t;

int db_audit_append(const audit_event_input_t *input,
                    char event_uuid[AUDIT_EVENT_UUID_MAX]);
int db_audit_query(const audit_query_t *query, audit_page_t *page);
void db_audit_page_free(audit_page_t *page);

int db_audit_get_retention_days(int *retention_days);
int db_audit_set_retention_days(int retention_days);
int db_audit_prune(int *deleted_count);

#endif /* LIGHTNVR_DB_AUDIT_H */
