#ifndef LIGHTNVR_DB_CAMERA_COLLECTIONS_H
#define LIGHTNVR_DB_CAMERA_COLLECTIONS_H

#include <stdbool.h>
#include <stdint.h>

#include "core/config.h"

#define CAMERA_COLLECTION_NAME_MAX 128
#define CAMERA_COLLECTION_DESCRIPTION_MAX 512
#define CAMERA_COLLECTION_TYPE_MAX 16
#define CAMERA_COLLECTION_SELECTOR_MAX 8192
#define CAMERA_COLLECTION_MAX_MEMBERS 4096

typedef struct {
    char uuid[CAMERA_UUID_STRING_SIZE];
    char name[CAMERA_COLLECTION_NAME_MAX];
    char description[CAMERA_COLLECTION_DESCRIPTION_MAX];
    char collection_type[CAMERA_COLLECTION_TYPE_MAX];
    char selector_json[CAMERA_COLLECTION_SELECTOR_MAX];
    bool is_shared;
    int64_t owner_user_id;
    int member_count;
    int64_t created_at;
    int64_t updated_at;
} camera_collection_t;

typedef enum {
    DB_CAMERA_COLLECTION_OK = 0,
    DB_CAMERA_COLLECTION_NOT_FOUND = -1,
    DB_CAMERA_COLLECTION_CONFLICT = -2,
    DB_CAMERA_COLLECTION_INVALID = -3,
    DB_CAMERA_COLLECTION_ERROR = -4,
    DB_CAMERA_COLLECTION_WRONG_TYPE = -5,
    DB_CAMERA_COLLECTION_LIMIT = -6
} db_camera_collection_result_t;

int db_camera_collection_count(void);
int db_camera_collection_list(camera_collection_t *collections, int max_count);
db_camera_collection_result_t db_camera_collection_get(
    const char *uuid, camera_collection_t *collection);
db_camera_collection_result_t db_camera_collection_create(
    camera_collection_t *collection);
db_camera_collection_result_t db_camera_collection_update(
    camera_collection_t *collection);
db_camera_collection_result_t db_camera_collection_delete(const char *uuid);

int db_camera_collection_list_members(
    const char *collection_uuid,
    char camera_uuids[][CAMERA_UUID_STRING_SIZE], int max_count);
db_camera_collection_result_t db_camera_collection_set_members(
    const char *collection_uuid, const char *const *camera_uuids,
    int camera_count);

#endif /* LIGHTNVR_DB_CAMERA_COLLECTIONS_H */
