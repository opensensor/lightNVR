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
