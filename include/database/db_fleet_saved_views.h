#ifndef LIGHTNVR_DB_FLEET_SAVED_VIEWS_H
#define LIGHTNVR_DB_FLEET_SAVED_VIEWS_H

#include <stdbool.h>
#include <stdint.h>

#include "core/config.h"

#define FLEET_SAVED_VIEW_NAME_MAX 128
#define FLEET_SAVED_VIEW_SELECTOR_MAX 4096
#define FLEET_SAVED_VIEW_SEARCH_MAX 256
#define FLEET_SAVED_VIEW_COLUMNS_MAX 1024
#define FLEET_SAVED_VIEW_SORT_MAX 32
#define FLEET_SAVED_VIEW_MAX_VISIBLE 256

typedef struct {
    char uuid[CAMERA_UUID_STRING_SIZE];
    int64_t owner_user_id;
    char name[FLEET_SAVED_VIEW_NAME_MAX];
    bool is_shared;
    char selector_json[FLEET_SAVED_VIEW_SELECTOR_MAX];
    char search[FLEET_SAVED_VIEW_SEARCH_MAX];
    char collection_uuid[CAMERA_UUID_STRING_SIZE];
    char columns_json[FLEET_SAVED_VIEW_COLUMNS_MAX];
    char sort_by[FLEET_SAVED_VIEW_SORT_MAX];
    char sort_order[8];
    int64_t revision;
    int64_t created_at;
    int64_t updated_at;
} fleet_saved_view_t;

typedef enum {
    DB_FLEET_SAVED_VIEW_OK = 0,
    DB_FLEET_SAVED_VIEW_NOT_FOUND = -1,
    DB_FLEET_SAVED_VIEW_INVALID = -2,
    DB_FLEET_SAVED_VIEW_CONFLICT = -3,
    DB_FLEET_SAVED_VIEW_STALE = -4,
    DB_FLEET_SAVED_VIEW_LIMIT = -5,
    DB_FLEET_SAVED_VIEW_ERROR = -6
} db_fleet_saved_view_result_t;

int db_fleet_saved_view_list_visible(
    int64_t owner_user_id, fleet_saved_view_t *views, int max_count);
db_fleet_saved_view_result_t db_fleet_saved_view_get_visible(
    int64_t owner_user_id, const char *uuid, fleet_saved_view_t *view);
db_fleet_saved_view_result_t db_fleet_saved_view_create(
    fleet_saved_view_t *view);
db_fleet_saved_view_result_t db_fleet_saved_view_update(
    fleet_saved_view_t *view, int64_t expected_revision);
db_fleet_saved_view_result_t db_fleet_saved_view_delete(
    int64_t owner_user_id, const char *uuid, int64_t expected_revision);

#endif /* LIGHTNVR_DB_FLEET_SAVED_VIEWS_H */
