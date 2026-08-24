#ifndef LIGHTNVR_DB_STORAGE_TARGETS_H
#define LIGHTNVR_DB_STORAGE_TARGETS_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include "core/config.h"
#include "utils/uuid.h"

#define STORAGE_TARGET_NAME_MAX 128
#define STORAGE_TARGET_TYPE_MAX 24
#define STORAGE_TARGET_CLASS_MAX 16
#define STORAGE_TARGET_HEALTH_MAX 16
#define STORAGE_TARGET_ERROR_MAX 256
#define STORAGE_TARGET_OBJECT_KEY_MAX MAX_PATH_LENGTH
#define STORAGE_TARGET_MAX_COUNT 128

typedef struct {
    char uuid[LIGHTNVR_UUID_STRING_SIZE];
    char name[STORAGE_TARGET_NAME_MAX];
    char target_type[STORAGE_TARGET_TYPE_MAX];
    char root_path[MAX_PATH_LENGTH];
    bool enabled;
    bool is_default;
    char storage_class[STORAGE_TARGET_CLASS_MAX];
    uint64_t reserve_bytes;
    double high_watermark_pct;
    double low_watermark_pct;
    char health_status[STORAGE_TARGET_HEALTH_MAX];
    uint64_t capacity_bytes;
    uint64_t available_bytes;
    uint64_t filesystem_device;
    int64_t last_probe_at;
    int64_t last_success_at;
    char last_error[STORAGE_TARGET_ERROR_MAX];
    int64_t revision;
    int64_t created_at;
    int64_t updated_at;
    uint64_t recording_count;
    uint64_t recording_bytes;
    bool mount_required;
    char mount_guard_path[MAX_PATH_LENGTH];
} storage_target_t;

typedef enum {
    DB_STORAGE_TARGET_OK = 0,
    DB_STORAGE_TARGET_NOT_FOUND = -1,
    DB_STORAGE_TARGET_CONFLICT = -2,
    DB_STORAGE_TARGET_INVALID = -3,
    DB_STORAGE_TARGET_STALE = -4,
    DB_STORAGE_TARGET_LIMIT = -5,
    DB_STORAGE_TARGET_IN_USE = -6,
    DB_STORAGE_TARGET_UNAVAILABLE = -7,
    DB_STORAGE_TARGET_ERROR = -8
} db_storage_target_result_t;

db_storage_target_result_t db_storage_target_validate(
    storage_target_t *target, char *error, size_t error_size);
int db_storage_target_count(void);
int db_storage_target_list(storage_target_t *targets, int max_count);
db_storage_target_result_t db_storage_target_get(
    const char *uuid, storage_target_t *target);
db_storage_target_result_t db_storage_target_get_default(
    storage_target_t *target);
db_storage_target_result_t db_storage_target_create(storage_target_t *target);
db_storage_target_result_t db_storage_target_update(
    storage_target_t *target, int64_t expected_revision);
db_storage_target_result_t db_storage_target_delete(
    const char *uuid, int64_t expected_revision);

/* Probe filesystem capacity and optionally verify write/fsync/unlink. */
db_storage_target_result_t db_storage_target_probe(
    const char *uuid, bool write_test, storage_target_t *target);
int db_storage_target_refresh_health(void);

/*
 * Ensure an upgraded installation has one stable default target and attach
 * existing absolute recording paths beneath that root without moving files.
 */
int db_storage_target_bootstrap_default(
    const char *legacy_root, char uuid[LIGHTNVR_UUID_STRING_SIZE]);

/* Map an absolute path to the most-specific configured target root. */
int db_storage_target_classify_path(
    const char *absolute_path,
    char target_uuid[LIGHTNVR_UUID_STRING_SIZE],
    char object_key[STORAGE_TARGET_OBJECT_KEY_MAX]);

/* Resolve a durable target/object identity, rejecting traversal keys. */
int db_storage_target_resolve_path(
    const char *target_uuid, const char *object_key,
    char absolute_path[MAX_PATH_LENGTH]);

/*
 * Find the most-specific non-root mount containing root_path. mountinfo_path is
 * injectable for tests and should normally be /proc/self/mountinfo.
 */
int db_storage_target_detect_mount(
    const char *root_path, const char *mountinfo_path,
    char mount_path[MAX_PATH_LENGTH]);

/* Fast mount-table-only check used on the recording hot path. */
bool db_storage_target_mount_guard_active(const storage_target_t *target);

#endif /* LIGHTNVR_DB_STORAGE_TARGETS_H */
