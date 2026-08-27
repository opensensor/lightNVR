#ifndef LIGHTNVR_DB_LIVE_SAVED_LAYOUTS_H
#define LIGHTNVR_DB_LIVE_SAVED_LAYOUTS_H

#include <stdbool.h>
#include <stdint.h>

#include "core/config.h"

#define LIVE_LAYOUT_NAME_MAX 128
#define LIVE_LAYOUT_SLOTS_JSON_MAX 8192
#define LIVE_LAYOUT_MAX_VISIBLE 256

typedef struct {
    char uuid[CAMERA_UUID_STRING_SIZE];
    int64_t owner_user_id;
    char name[LIVE_LAYOUT_NAME_MAX];
    bool is_shared;
    char location_uuid[CAMERA_UUID_STRING_SIZE];
    char availability[24];
    int columns;
    int rows;
    char camera_slots_json[LIVE_LAYOUT_SLOTS_JSON_MAX];
    int64_t revision;
    int64_t created_at;
    int64_t updated_at;
} live_saved_layout_t;

typedef enum {
    DB_LIVE_LAYOUT_OK = 0,
    DB_LIVE_LAYOUT_NOT_FOUND = -1,
    DB_LIVE_LAYOUT_INVALID = -2,
    DB_LIVE_LAYOUT_CONFLICT = -3,
    DB_LIVE_LAYOUT_STALE = -4,
    DB_LIVE_LAYOUT_LIMIT = -5,
    DB_LIVE_LAYOUT_ERROR = -6
} db_live_layout_result_t;

int db_live_layout_list_visible(int64_t owner_user_id,
                                live_saved_layout_t *layouts,
                                int max_count);
db_live_layout_result_t db_live_layout_get_visible(
    int64_t owner_user_id, const char *uuid, live_saved_layout_t *layout);
db_live_layout_result_t db_live_layout_create(live_saved_layout_t *layout);
db_live_layout_result_t db_live_layout_update(live_saved_layout_t *layout,
                                              int64_t expected_revision);
db_live_layout_result_t db_live_layout_delete(int64_t owner_user_id,
                                              const char *uuid,
                                              int64_t expected_revision);

#endif /* LIGHTNVR_DB_LIVE_SAVED_LAYOUTS_H */
