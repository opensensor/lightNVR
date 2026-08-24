#ifndef LIGHTNVR_DB_INVESTIGATION_BOOKMARKS_H
#define LIGHTNVR_DB_INVESTIGATION_BOOKMARKS_H

#include <stdint.h>

#include "core/config.h"

#define INVESTIGATION_BOOKMARK_TITLE_MAX 128
#define INVESTIGATION_BOOKMARK_NOTE_MAX 2048
#define INVESTIGATION_BOOKMARK_FILTERS_MAX 4096
#define INVESTIGATION_BOOKMARK_RESULT_MAX 2048
#define INVESTIGATION_BOOKMARK_MAX_CAMERAS 16
#define INVESTIGATION_BOOKMARK_MAX_PER_OWNER 256

typedef struct {
    char uuid[CAMERA_UUID_STRING_SIZE];
    int64_t owner_user_id;
    char title[INVESTIGATION_BOOKMARK_TITLE_MAX];
    char note[INVESTIGATION_BOOKMARK_NOTE_MAX];
    int64_t start_time;
    int64_t end_time;
    int64_t cursor_time;
    char primary_camera_uuid[CAMERA_UUID_STRING_SIZE];
    char filters_json[INVESTIGATION_BOOKMARK_FILTERS_MAX];
    char representative_result_json[INVESTIGATION_BOOKMARK_RESULT_MAX];
    int camera_count;
    int64_t revision;
    int64_t created_at;
    int64_t updated_at;
} investigation_bookmark_t;

typedef enum {
    DB_INVESTIGATION_BOOKMARK_OK = 0,
    DB_INVESTIGATION_BOOKMARK_NOT_FOUND = -1,
    DB_INVESTIGATION_BOOKMARK_INVALID = -2,
    DB_INVESTIGATION_BOOKMARK_STALE = -3,
    DB_INVESTIGATION_BOOKMARK_LIMIT = -4,
    DB_INVESTIGATION_BOOKMARK_ERROR = -5
} db_investigation_bookmark_result_t;

int db_investigation_bookmark_count(int64_t owner_user_id);
int db_investigation_bookmark_list(
    int64_t owner_user_id, investigation_bookmark_t *bookmarks,
    int max_count);
db_investigation_bookmark_result_t db_investigation_bookmark_get(
    int64_t owner_user_id, const char *uuid,
    investigation_bookmark_t *bookmark);
int db_investigation_bookmark_list_cameras(
    const char *bookmark_uuid,
    char camera_uuids[][CAMERA_UUID_STRING_SIZE], int max_count);
db_investigation_bookmark_result_t db_investigation_bookmark_create(
    investigation_bookmark_t *bookmark,
    const char camera_uuids[][CAMERA_UUID_STRING_SIZE], int camera_count);
db_investigation_bookmark_result_t db_investigation_bookmark_update_metadata(
    investigation_bookmark_t *bookmark, int64_t expected_revision);
db_investigation_bookmark_result_t db_investigation_bookmark_delete(
    int64_t owner_user_id, const char *uuid, int64_t expected_revision);

#endif /* LIGHTNVR_DB_INVESTIGATION_BOOKMARKS_H */
