#ifndef LIGHTNVR_DB_STORAGE_POLICIES_H
#define LIGHTNVR_DB_STORAGE_POLICIES_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include "core/config.h"
#include "database/db_storage_targets.h"
#include "utils/uuid.h"

#define STORAGE_POLICY_NAME_MAX 128
#define STORAGE_POLICY_SELECTOR_MAX 8192
#define STORAGE_POLICY_FALLBACK_MODE_MAX 16
#define STORAGE_POLICY_MAX_COUNT 128

typedef struct {
    char uuid[LIGHTNVR_UUID_STRING_SIZE];
    char name[STORAGE_POLICY_NAME_MAX];
    bool enabled;
    int priority;
    char selector_json[STORAGE_POLICY_SELECTOR_MAX];
    char primary_target_uuid[LIGHTNVR_UUID_STRING_SIZE];
    char primary_pool_uuid[LIGHTNVR_UUID_STRING_SIZE];
    char fallback_mode[STORAGE_POLICY_FALLBACK_MODE_MAX];
    char fallback_target_uuid[LIGHTNVR_UUID_STRING_SIZE];
    int minimum_retention_days;
    int desired_retention_days;
    int maximum_retention_days;
    int required_copy_count;
    char replication_pool_uuid[LIGHTNVR_UUID_STRING_SIZE];
    int migration_after_days;
    char migration_target_uuid[LIGHTNVR_UUID_STRING_SIZE];
    int pressure_priority;
    int64_t revision;
    int64_t created_at;
    int64_t updated_at;
} storage_policy_t;

typedef enum {
    DB_STORAGE_POLICY_OK = 0,
    DB_STORAGE_POLICY_NOT_FOUND = -1,
    DB_STORAGE_POLICY_CONFLICT = -2,
    DB_STORAGE_POLICY_INVALID = -3,
    DB_STORAGE_POLICY_STALE = -4,
    DB_STORAGE_POLICY_LIMIT = -5,
    DB_STORAGE_POLICY_ERROR = -6
} db_storage_policy_result_t;

db_storage_policy_result_t db_storage_policy_validate(
    storage_policy_t *policy, char *error, size_t error_size);
int db_storage_policy_count(void);
int db_storage_policy_list(storage_policy_t *policies, int max_count,
                           bool enabled_only);
db_storage_policy_result_t db_storage_policy_get(
    const char *uuid, storage_policy_t *policy);
db_storage_policy_result_t db_storage_policy_create(storage_policy_t *policy);
db_storage_policy_result_t db_storage_policy_update(
    storage_policy_t *policy, int64_t expected_revision);
db_storage_policy_result_t db_storage_policy_delete(
    const char *uuid, int64_t expected_revision);

/* Incremented after every successful policy mutation for placement cache reloads. */
uint64_t db_storage_policy_generation(void);

#endif /* LIGHTNVR_DB_STORAGE_POLICIES_H */
