#ifndef LIGHTNVR_DB_STORAGE_POOLS_H
#define LIGHTNVR_DB_STORAGE_POOLS_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include "database/db_storage_targets.h"

#define STORAGE_POOL_NAME_MAX 128
#define STORAGE_POOL_STRATEGY_MAX 24
#define STORAGE_POOL_MAX_COUNT 64
#define STORAGE_POOL_MEMBER_MAX STORAGE_TARGET_MAX_COUNT

typedef struct {
    char target_uuid[LIGHTNVR_UUID_STRING_SIZE];
    int position;
    int weight;
} storage_pool_member_t;

typedef struct {
    char uuid[LIGHTNVR_UUID_STRING_SIZE];
    char name[STORAGE_POOL_NAME_MAX];
    char strategy[STORAGE_POOL_STRATEGY_MAX];
    bool enabled;
    int allocation_cursor;
    int64_t revision;
    int64_t created_at;
    int64_t updated_at;
    int member_count;
    storage_pool_member_t members[STORAGE_POOL_MEMBER_MAX];
} storage_pool_t;

typedef enum {
    DB_STORAGE_POOL_OK = 0,
    DB_STORAGE_POOL_NOT_FOUND = -1,
    DB_STORAGE_POOL_CONFLICT = -2,
    DB_STORAGE_POOL_INVALID = -3,
    DB_STORAGE_POOL_STALE = -4,
    DB_STORAGE_POOL_LIMIT = -5,
    DB_STORAGE_POOL_IN_USE = -6,
    DB_STORAGE_POOL_UNAVAILABLE = -7,
    DB_STORAGE_POOL_ERROR = -8
} db_storage_pool_result_t;

db_storage_pool_result_t db_storage_pool_validate(
    storage_pool_t *pool, char *error, size_t error_size);
int db_storage_pool_count(void);
int db_storage_pool_list(storage_pool_t *pools, int max_count);
db_storage_pool_result_t db_storage_pool_get(
    const char *uuid, storage_pool_t *pool);
db_storage_pool_result_t db_storage_pool_create(storage_pool_t *pool);
db_storage_pool_result_t db_storage_pool_update(
    storage_pool_t *pool, int64_t expected_revision);
db_storage_pool_result_t db_storage_pool_delete(
    const char *uuid, int64_t expected_revision);

/* Select one enabled, healthy member according to the pool strategy. */
db_storage_pool_result_t db_storage_pool_allocate(
    const char *uuid, const char *exclude_target_uuid,
    storage_target_t *target);

#endif
