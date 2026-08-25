#ifndef LIGHTNVR_DB_STORAGE_MIGRATIONS_H
#define LIGHTNVR_DB_STORAGE_MIGRATIONS_H

#include <stdbool.h>
#include <stdint.h>

#include "database/db_storage_targets.h"

#define STORAGE_MIGRATION_STATE_MAX 24
#define STORAGE_MIGRATION_CHECKSUM_MAX 65
#define STORAGE_MIGRATION_ERROR_MAX 256
#define STORAGE_MIGRATION_MAX_VISIBLE 256

typedef struct {
    char uuid[LIGHTNVR_UUID_STRING_SIZE];
    uint64_t recording_id;
    int64_t owner_user_id;
    char source_target_uuid[LIGHTNVR_UUID_STRING_SIZE];
    char source_object_key[STORAGE_TARGET_OBJECT_KEY_MAX];
    char destination_target_uuid[LIGHTNVR_UUID_STRING_SIZE];
    char destination_object_key[STORAGE_TARGET_OBJECT_KEY_MAX];
    char state[STORAGE_MIGRATION_STATE_MAX];
    char checksum[STORAGE_MIGRATION_CHECKSUM_MAX];
    uint64_t bytes_total;
    uint64_t bytes_copied;
    int attempt_count;
    int max_attempts;
    int64_t next_attempt_at;
    char last_error[STORAGE_MIGRATION_ERROR_MAX];
    int64_t revision;
    int64_t created_at;
    int64_t updated_at;
    int64_t started_at;
    int64_t completed_at;
} storage_migration_job_t;

typedef enum {
    DB_STORAGE_MIGRATION_OK = 0,
    DB_STORAGE_MIGRATION_NOT_FOUND = -1,
    DB_STORAGE_MIGRATION_CONFLICT = -2,
    DB_STORAGE_MIGRATION_INVALID = -3,
    DB_STORAGE_MIGRATION_SOURCE_INCOMPLETE = -4,
    DB_STORAGE_MIGRATION_TARGET_UNAVAILABLE = -5,
    DB_STORAGE_MIGRATION_SOURCE_CHANGED = -6,
    DB_STORAGE_MIGRATION_ERROR = -7
} db_storage_migration_result_t;

db_storage_migration_result_t db_storage_migration_create(
    uint64_t recording_id, const char *destination_target_uuid,
    int64_t owner_user_id, storage_migration_job_t *job);
db_storage_migration_result_t db_storage_migration_get(
    const char *uuid, storage_migration_job_t *job);
int db_storage_migration_list(storage_migration_job_t *jobs, int max_count);

/* Claim one due or restart-interrupted job. Returns 1, 0, or -1. */
int db_storage_migration_claim_due(storage_migration_job_t *job);
db_storage_migration_result_t db_storage_migration_update_progress(
    const char *uuid, const char *state, uint64_t bytes_copied,
    uint64_t bytes_total);
db_storage_migration_result_t db_storage_migration_record_failure(
    const storage_migration_job_t *job, const char *error, bool retryable);

/*
 * Atomically point the recording at the verified destination and advance the
 * job to cleanup_pending. The source identity is compared as a CAS guard.
 */
db_storage_migration_result_t db_storage_migration_commit_location(
    const storage_migration_job_t *job, const char *destination_path,
    const char *checksum);
db_storage_migration_result_t db_storage_migration_complete_cleanup(
    const char *uuid);
db_storage_migration_result_t db_storage_migration_defer_cleanup(
    const storage_migration_job_t *job, const char *error);

#endif /* LIGHTNVR_DB_STORAGE_MIGRATIONS_H */
