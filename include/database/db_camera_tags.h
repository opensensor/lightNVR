#ifndef LIGHTNVR_DB_CAMERA_TAGS_H
#define LIGHTNVR_DB_CAMERA_TAGS_H

#include <stdint.h>
#include <sqlite3.h>

#include "core/config.h"

#define CAMERA_TAG_LABEL_MAX 256
#define CAMERA_TAG_COLOR_MAX 16
#define CAMERA_TAG_DESCRIPTION_MAX 512
#define CAMERA_TAG_MAX_ASSIGNMENTS 64

typedef struct {
    char uuid[CAMERA_UUID_STRING_SIZE];
    char label[CAMERA_TAG_LABEL_MAX];
    char color[CAMERA_TAG_COLOR_MAX];
    char description[CAMERA_TAG_DESCRIPTION_MAX];
    int camera_count;
    int64_t created_at;
    int64_t updated_at;
} camera_tag_t;

typedef enum {
    DB_CAMERA_TAG_OK = 0,
    DB_CAMERA_TAG_NOT_FOUND = -1,
    DB_CAMERA_TAG_CONFLICT = -2,
    DB_CAMERA_TAG_INVALID = -3,
    DB_CAMERA_TAG_ERROR = -4,
    DB_CAMERA_TAG_LIMIT = -5
} db_camera_tag_result_t;

int db_camera_tag_count(void);
int db_camera_tag_list(camera_tag_t *tags, int max_count);
db_camera_tag_result_t db_camera_tag_get(const char *uuid, camera_tag_t *tag);
db_camera_tag_result_t db_camera_tag_create(camera_tag_t *tag);
db_camera_tag_result_t db_camera_tag_update(camera_tag_t *tag);
db_camera_tag_result_t db_camera_tag_delete(const char *uuid);
db_camera_tag_result_t db_camera_tag_merge(const char *source_uuid,
                                           const char *target_uuid);

int db_camera_tag_list_for_camera(const char *camera_uuid, camera_tag_t *tags,
                                  int max_count);
db_camera_tag_result_t db_camera_tag_set_for_camera(
    const char *camera_uuid, const char *const *tag_uuids, int tag_count);

/* Idempotently import streams.tags into the normalized tables. Called at
 * startup immediately after migrations. */
int db_camera_tags_backfill_legacy(void);

/*
 * Run the legacy backfill only the first time this schema version starts.
 * The backfill rewrites every assignment for every camera, and both tag
 * writers keep streams.tags and the normalized tables consistent afterwards,
 * so repeating it on each boot is pure churn.
 */
int db_camera_tags_backfill_legacy_once(void);

/* Compatibility bridge used by db_streams.c while the database mutex is held.
 * The normalized assignments are replaced from the legacy comma-separated
 * value. The caller must already own get_db_mutex(). */
int db_camera_tags_sync_legacy_by_name_locked(sqlite3 *db,
                                              const char *stream_name,
                                              const char *legacy_tags);

#endif /* LIGHTNVR_DB_CAMERA_TAGS_H */
