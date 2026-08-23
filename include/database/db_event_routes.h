#ifndef LIGHTNVR_DB_EVENT_ROUTES_H
#define LIGHTNVR_DB_EVENT_ROUTES_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include "core/event_envelope.h"

#define EVENT_ROUTE_UUID_MAX 37
#define EVENT_ROUTE_NAME_MAX 128
#define EVENT_ROUTE_DESCRIPTION_MAX 512
#define EVENT_ROUTE_DESTINATION_MAX 128
#define EVENT_ROUTE_SCOPE_MAX 16
#define EVENT_ROUTE_SELECTOR_MAX 8192
#define EVENT_ROUTE_PREDICATE_MAX 4096
#define EVENT_ROUTE_SCHEDULE_MAX 8192
#define EVENT_ROUTE_MAX_TYPES 32
#define EVENT_ROUTE_MAX_COUNT 512
#define EVENT_ROUTE_VALIDATION_ERROR_MAX 256
#define EVENT_ROUTE_DEFAULT_DESTINATION "mqtt:default"

typedef struct {
    char uuid[EVENT_ROUTE_UUID_MAX];
    char name[EVENT_ROUTE_NAME_MAX];
    char description[EVENT_ROUTE_DESCRIPTION_MAX];
    bool enabled;
    char destination_key[EVENT_ROUTE_DESTINATION_MAX];
    char scope_type[EVENT_ROUTE_SCOPE_MAX];
    char selector_json[EVENT_ROUTE_SELECTOR_MAX];
    char predicate_json[EVENT_ROUTE_PREDICATE_MAX];
    char schedule_json[EVENT_ROUTE_SCHEDULE_MAX];
    char event_types[EVENT_ROUTE_MAX_TYPES][EVENT_TYPE_MAX];
    int event_type_count;
    int debounce_seconds;
    int cooldown_seconds;
    int grouping_window_seconds;
    int max_events_per_minute;
    int64_t revision;
    int64_t created_at;
    int64_t updated_at;
} event_route_t;

typedef enum {
    DB_EVENT_ROUTE_OK = 0,
    DB_EVENT_ROUTE_NOT_FOUND = -1,
    DB_EVENT_ROUTE_CONFLICT = -2,
    DB_EVENT_ROUTE_INVALID = -3,
    DB_EVENT_ROUTE_STALE = -4,
    DB_EVENT_ROUTE_LIMIT = -5,
    DB_EVENT_ROUTE_ERROR = -6
} db_event_route_result_t;

/* Shared validation boundary used by persistence and draft preview. */
db_event_route_result_t db_event_route_validate(
    const event_route_t *route, char *error, size_t error_size);

int db_event_route_count(void);
int db_event_route_list(event_route_t *routes, int max_count);
db_event_route_result_t db_event_route_get(const char *uuid,
                                           event_route_t *route);
db_event_route_result_t db_event_route_create(event_route_t *route);
db_event_route_result_t db_event_route_update(event_route_t *route,
                                              int64_t expected_revision);
db_event_route_result_t db_event_route_delete(const char *uuid,
                                              int64_t expected_revision);

#endif /* LIGHTNVR_DB_EVENT_ROUTES_H */
