#define _GNU_SOURCE
#define _POSIX_C_SOURCE 200809L
#include <errno.h>
#include <dirent.h>
#include <stdarg.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/syscall.h>
#include <unistd.h>
#include "unity.h"
#include "core/config.h"
#include "core/path_utils.h"
#include "database/db_core.h"
#include "database/db_recordings.h"
#include "database/db_storage_migrations.h"
#include "database/db_storage_targets.h"
#include "storage/storage_migration.h"
#include "utils/strings.h"

static char root[128], source_root[256], destination_root[256], db_path[256];
static char source_path[MAX_PATH_LENGTH], destination_path[MAX_PATH_LENGTH];
static char temporary_path[MAX_PATH_LENGTH], sentinel[256], verified_part[256];
static char source_uuid[LIGHTNVR_UUID_STRING_SIZE], destination_uuid[LIGHTNVR_UUID_STRING_SIZE];
static bool armed, injected;
static int replacement_kind, publication_error, destination_stat_error;
static bool fail_part_unlink;
static int quarantine_error, committing_replacement;
static bool restore_collision, commit_failure_replacement, cancel_after_publication;
static char private_artifact[MAX_PATH_LENGTH], active_job[LIGHTNVR_UUID_STRING_SIZE];
static char relocated_parent[256];
int __real_unlink(const char *);
int __wrap_unlink(const char *path) {
    if (fail_part_unlink && !strcmp(path, temporary_path)) { errno = EACCES; return -1; }
    return __real_unlink(path);
}
static const char payload[] = "verified source recording";
static void put_file(const char *path, const char *bytes) {
    TEST_ASSERT_EQUAL_INT(0, ensure_path(path));
    FILE *file = fopen(path, "w"); TEST_ASSERT_NOT_NULL(file);
    size_t length = fwrite(bytes, 1, strlen(bytes), file); int closed = fclose(file);
    TEST_ASSERT_EQUAL_UINT(strlen(bytes), length); TEST_ASSERT_EQUAL_INT(0, closed);
}
static void inject_before_publication(void) {
    armed = false; injected = true;
    if (replacement_kind == 1) put_file(destination_path, "concurrent destination data");
    else if (replacement_kind == 2) TEST_ASSERT_EQUAL_INT(0, symlink(sentinel, destination_path));
    else if (replacement_kind == 4) put_file(temporary_path, "modified verified inode");
    else {
        TEST_ASSERT_EQUAL_INT(0, rename(temporary_path, verified_part));
        put_file(temporary_path, "unverified replacement data");
    }
}
int __real_lstat(const char *, struct stat *);
int __wrap_lstat(const char *path, struct stat *info) {
    if (destination_stat_error && !strcmp(path, destination_path)) { errno = destination_stat_error; return -1; }
    int result = __real_lstat(path, info); int saved = errno;
    if (armed && result != 0 && saved == ENOENT && strcmp(path, destination_path) == 0 &&
            access(temporary_path, F_OK) == 0) inject_before_publication();
    errno = saved; return result;
}
long __real_syscall(long, ...);
long __wrap_syscall(long number, ...) {
    if (number != SYS_renameat2) {
        TEST_FAIL_MESSAGE("Unexpected syscall in publication seam"); errno = ENOSYS; return -1;
    }
    va_list args; va_start(args, number);
    int oldfd = va_arg(args, int); const char *oldname = va_arg(args, const char *);
    int newfd = va_arg(args, int); const char *newname = va_arg(args, const char *);
    unsigned int flags = va_arg(args, unsigned int); va_end(args);
    bool quarantine = !strcmp(oldname, temporary_path) ||
        (!strcmp(newname, "verified") && strstr(oldname, ".migration-"));
    if (armed && ((replacement_kind >= 3 && quarantine) ||
        (replacement_kind < 3 && (!strcmp(newname, destination_path) || !strcmp(newname, "clip.mp4")))))
        inject_before_publication();
    if (quarantine_error && quarantine) { errno = quarantine_error; return -1; }
    if (publication_error && (!strcmp(newname, destination_path) || !strcmp(newname, "clip.mp4"))) {
        errno = publication_error; return -1;
    }
    if (quarantine && !strcmp(newname, "verified")) {
        char descriptor_path[64], directory[MAX_PATH_LENGTH];
        snprintf(descriptor_path, sizeof(descriptor_path), "/proc/self/fd/%d", newfd);
        ssize_t length = readlink(descriptor_path, directory, sizeof(directory)-1);
        TEST_ASSERT_TRUE(length > 0); directory[length] = '\0';
        int size = snprintf(private_artifact, sizeof(private_artifact), "%s/verified", directory);
        TEST_ASSERT_TRUE(size > 0 && (size_t)size < sizeof(private_artifact));
    }
    if (restore_collision && !strcmp(oldname, "verified") && strstr(newname, ".part"))
        put_file(temporary_path, "restore collision sentinel");
    long result = __real_syscall(number, oldfd, oldname, newfd, newname, flags);
    if (result == 0 && cancel_after_publication && !strcmp(newname, "clip.mp4")) {
        cancel_after_publication = false;
        TEST_ASSERT_EQUAL_INT(0, rename(destination_path, verified_part));
        put_file(destination_path, "cancelled substitution sentinel");
        storage_migration_job_t job;
        TEST_ASSERT_EQUAL_INT(DB_STORAGE_MIGRATION_OK, db_storage_migration_request_cancel(active_job, &job));
    }
    return result;
}
db_storage_migration_result_t __real_db_storage_migration_update_progress(
    const char *, const char *, uint64_t, uint64_t);
db_storage_migration_result_t __wrap_db_storage_migration_update_progress(
    const char *uuid, const char *state, uint64_t copied, uint64_t total) {
    if (committing_replacement && !strcmp(state, "committing")) {
        int kind = committing_replacement; committing_replacement = 0;
        if (kind == 2) {
            char parent[MAX_PATH_LENGTH], relocated_file[MAX_PATH_LENGTH];
            strcpy(parent, destination_path); *strrchr(parent, '/') = 0;
            TEST_ASSERT_EQUAL_INT(0, rename(parent, relocated_parent));
            TEST_ASSERT_EQUAL_INT(0, mkdir(parent, 0700));
            int size = snprintf(relocated_file, sizeof(relocated_file), "%s/clip.mp4", relocated_parent);
            TEST_ASSERT_TRUE(size > 0 && (size_t)size < sizeof(relocated_file));
            TEST_ASSERT_EQUAL_INT(0, link(relocated_file, destination_path));
        } else {
            TEST_ASSERT_EQUAL_INT(0, rename(destination_path, verified_part));
            put_file(destination_path, "substituted published destination");
        }
    }
    return __real_db_storage_migration_update_progress(uuid, state, copied, total);
}
db_storage_migration_result_t __real_db_storage_migration_commit_location(
    const storage_migration_job_t *, const char *, const char *);
db_storage_migration_result_t __wrap_db_storage_migration_commit_location(
    const storage_migration_job_t *job, const char *path, const char *checksum) {
    if (commit_failure_replacement) {
        TEST_ASSERT_EQUAL_INT(0, rename(destination_path, verified_part));
        put_file(destination_path, "failed commit substitution sentinel");
        return DB_STORAGE_MIGRATION_ERROR;
    }
    return __real_db_storage_migration_commit_location(job, path, checksum);
}
static void assert_bytes(const char *path, const char *expected) {
    FILE *file = fopen(path, "r"); TEST_ASSERT_NOT_NULL(file);
    char actual[128] = {0}; size_t bytes = fread(actual, 1, sizeof(actual)-1, file); int closed = fclose(file);
    TEST_ASSERT_EQUAL_INT(0, closed); TEST_ASSERT_EQUAL_UINT(strlen(expected), bytes);
    TEST_ASSERT_EQUAL_STRING(expected, actual);
}
static void remove_owned_tree(const char *directory) {
    DIR *dir = opendir(directory); if (!dir) return;
    struct dirent *entry;
    while ((entry = readdir(dir))) {
        if (!strcmp(entry->d_name, ".") || !strcmp(entry->d_name, "..")) continue;
        char path[1024]; snprintf(path, sizeof(path), "%s/%s", directory, entry->d_name);
        struct stat info;
        if (__real_lstat(path, &info) == 0 && S_ISDIR(info.st_mode)) remove_owned_tree(path);
        else unlink(path);
    }
    closedir(dir); rmdir(directory);
}
static void configure_targets(void) {
    TEST_ASSERT_EQUAL_INT(0, db_storage_target_bootstrap_default(source_root, source_uuid));
    storage_target_t source;
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_TARGET_OK, db_storage_target_probe(source_uuid, true, &source));
    storage_target_t destination = {0};
    strcpy(destination.name, "T7.26 destination"); strcpy(destination.target_type, "filesystem");
    safe_strcpy(destination.root_path, destination_root, sizeof(destination.root_path), 0);
    strcpy(destination.storage_class, "warm"); destination.enabled = true;
    destination.high_watermark_pct = 90.0; destination.low_watermark_pct = 80.0;
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_TARGET_OK, db_storage_target_create(&destination));
    strcpy(destination_uuid, destination.uuid);
}
void setUp(void) {
    armed = injected = fail_part_unlink = false; replacement_kind = publication_error = destination_stat_error = 0; quarantine_error = committing_replacement = 0; temporary_path[0] = '\0';
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(get_db_handle(),
        "DELETE FROM storage_migration_jobs;DELETE FROM detections;DELETE FROM recordings;DELETE FROM storage_targets;",
        NULL, NULL, NULL));
    remove_owned_tree(source_root); remove_owned_tree(destination_root); remove_owned_tree(relocated_parent);
    TEST_ASSERT_EQUAL_INT(0, mkdir(source_root, 0700)); TEST_ASSERT_EQUAL_INT(0, mkdir(destination_root, 0700));
    restore_collision = commit_failure_replacement = cancel_after_publication = false; private_artifact[0] = 0;
    unlink(sentinel); unlink(verified_part); put_file(sentinel, "sentinel unchanged");
    configure_targets();
}
void tearDown(void) { armed = fail_part_unlink = false; }
static uint64_t create_job(storage_migration_job_t *job, const char *operation) {
    put_file(source_path, payload);
    recording_metadata_t recording = {0};
    strcpy(recording.stream_name, "migration-camera");
    strcpy(recording.camera_uuid, "11111111-1111-4111-8111-111111111111");
    safe_strcpy(recording.file_path, source_path, sizeof(recording.file_path), 0);
    strcpy(recording.storage_target_uuid, source_uuid); strcpy(recording.object_key, "camera/clip.mp4");
    strcpy(recording.placement_reason, "test"); strcpy(recording.codec, "h264");
    strcpy(recording.trigger_type, "continuous"); recording.start_time = 100; recording.end_time = 200;
    recording.size_bytes = strlen(payload); recording.width = 1920; recording.height = 1080;
    recording.fps = 30; recording.is_complete = true; recording.retention_override_days = -1;
    recording.retention_tier = RETENTION_TIER_STANDARD; recording.disk_pressure_eligible = true;
    uint64_t id = add_recording_metadata(&recording); TEST_ASSERT_NOT_EQUAL_UINT64(0, id);
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_MIGRATION_OK,
        db_storage_migration_create_operation(id, destination_uuid, operation, 0, job));
    strcpy(active_job, job->uuid);
    size_t needed = strlen(destination_path) + strlen(job->uuid) + sizeof(".migration-.part");
    TEST_ASSERT_TRUE(needed <= sizeof(temporary_path));
    strcpy(temporary_path, destination_path);
    strcat(temporary_path, ".migration-"); strcat(temporary_path, job->uuid); strcat(temporary_path, ".part");
    return id;
}
static void assert_source_location(uint64_t id) {
    recording_metadata_t recording;
    TEST_ASSERT_EQUAL_INT(0, get_recording_metadata_by_id(id, &recording));
    TEST_ASSERT_EQUAL_STRING(source_uuid, recording.storage_target_uuid);
    TEST_ASSERT_EQUAL_STRING(source_path, recording.file_path);
    assert_bytes(source_path, payload);
}
static void test_normal_move_commits_verified_destination_and_removes_source(void) {
    storage_migration_job_t job; uint64_t id = create_job(&job, "move");
    TEST_ASSERT_EQUAL_INT(1, storage_migration_process_one());
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_MIGRATION_OK, db_storage_migration_get(job.uuid, &job));
    TEST_ASSERT_EQUAL_STRING("completed", job.state); TEST_ASSERT_EQUAL_UINT(64, strlen(job.checksum));
    assert_bytes(destination_path, payload); TEST_ASSERT_EQUAL_INT(-1, access(source_path, F_OK));
    TEST_ASSERT_EQUAL_INT(-1, access(temporary_path, F_OK));
    recording_metadata_t recording;
    TEST_ASSERT_EQUAL_INT(0, get_recording_metadata_by_id(id, &recording));
    TEST_ASSERT_EQUAL_STRING(destination_uuid, recording.storage_target_uuid);
    TEST_ASSERT_EQUAL_STRING(destination_path, recording.file_path);
}
static void test_normal_copy_keeps_source_and_verified_replica(void) {
    storage_migration_job_t job; uint64_t id = create_job(&job, "copy");
    TEST_ASSERT_EQUAL_INT(1, storage_migration_process_one());
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_MIGRATION_OK, db_storage_migration_get(job.uuid, &job));
    TEST_ASSERT_EQUAL_STRING("completed", job.state); assert_source_location(id);
    assert_bytes(destination_path, payload);
}
static void test_regular_destination_appearing_after_check_is_not_overwritten(void) {
    storage_migration_job_t job; uint64_t id = create_job(&job, "move");
    replacement_kind = 1; armed = true;
    TEST_ASSERT_EQUAL_INT(1, storage_migration_process_one()); TEST_ASSERT_TRUE(injected);
    assert_bytes(destination_path, "concurrent destination data"); assert_source_location(id);
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_MIGRATION_OK, db_storage_migration_get(job.uuid, &job));
    TEST_ASSERT_EQUAL_STRING("failed", job.state);
}
static void test_symlink_destination_appearing_after_check_is_retained(void) {
    storage_migration_job_t job; uint64_t id = create_job(&job, "move");
    replacement_kind = 2; armed = true;
    TEST_ASSERT_EQUAL_INT(1, storage_migration_process_one()); TEST_ASSERT_TRUE(injected);
    struct stat info; TEST_ASSERT_EQUAL_INT(0, lstat(destination_path, &info));
    TEST_ASSERT_TRUE(S_ISLNK(info.st_mode)); assert_bytes(sentinel, "sentinel unchanged");
    assert_source_location(id);
}
static void test_existing_collision_survives_later_abandoned_cleanup(void) {
    storage_migration_job_t job; uint64_t id = create_job(&job, "move");
    put_file(destination_path, "preexisting unrelated data");
    TEST_ASSERT_EQUAL_INT(1, storage_migration_process_one());
    assert_bytes(destination_path, "preexisting unrelated data"); assert_source_location(id);
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_MIGRATION_OK, db_storage_migration_get(job.uuid, &job));
    TEST_ASSERT_EQUAL_STRING("failed", job.state);
    TEST_ASSERT_EQUAL_INT(0, storage_migration_process_one());
    assert_bytes(destination_path, "preexisting unrelated data"); assert_source_location(id);
}
static void test_unverified_temporary_replacement_is_not_committed(void) {
    storage_migration_job_t job; uint64_t id = create_job(&job, "move");
    replacement_kind = 3; armed = true;
    TEST_ASSERT_EQUAL_INT(1, storage_migration_process_one()); TEST_ASSERT_TRUE(injected);
    assert_source_location(id);
    TEST_ASSERT_EQUAL_INT(-1, access(destination_path, F_OK));
    assert_bytes(verified_part, payload);
}
static void test_unavailable_registered_source_preserves_metadata_and_bytes(void) {
    storage_migration_job_t job; uint64_t id = create_job(&job, "move");
    char moved[256]; snprintf(moved, sizeof(moved), "%s/source-unavailable", root);
    TEST_ASSERT_EQUAL_INT(0, rename(source_root, moved));
    int result = storage_migration_process_one();
    TEST_ASSERT_EQUAL_INT(0, rename(moved, source_root));
    TEST_ASSERT_EQUAL_INT(1, result); assert_source_location(id);
    TEST_ASSERT_EQUAL_INT(-1, access(destination_path, F_OK));
}
static void publication_failure(int error) {
    storage_migration_job_t job; uint64_t id = create_job(&job, "move");
    publication_error = error;
    TEST_ASSERT_EQUAL_INT(1, storage_migration_process_one());
    assert_source_location(id); TEST_ASSERT_EQUAL_INT(-1, access(destination_path, F_OK));
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_MIGRATION_OK, db_storage_migration_get(job.uuid, &job));
    TEST_ASSERT_EQUAL_STRING("retry_wait", job.state);
    assert_bytes(temporary_path, payload);
}
static void test_unsupported_atomic_publication_retains_source_and_temporary(void) { publication_failure(ENOSYS); }
static void test_unsupported_filesystem_publication_retains_source_and_temporary(void) { publication_failure(EINVAL); }
static void test_publication_io_failure_retains_source_and_temporary(void) { publication_failure(EIO); }
static void test_collision_cleanup_failure_never_overwrites_competing_destination(void) {
    storage_migration_job_t job; uint64_t id = create_job(&job, "move");
    replacement_kind = 1; armed = fail_part_unlink = true;
    TEST_ASSERT_EQUAL_INT(1, storage_migration_process_one()); TEST_ASSERT_TRUE(injected);
    assert_bytes(destination_path, "concurrent destination data"); assert_source_location(id);
    assert_bytes(temporary_path, payload);
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_MIGRATION_OK, db_storage_migration_get(job.uuid, &job));
    TEST_ASSERT_EQUAL_STRING("failed", job.state);
}
static int artifacts_cleaned(const char *uuid) {
    sqlite3_stmt *statement = NULL;
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_prepare_v2(get_db_handle(),
        "SELECT artifacts_cleaned FROM storage_migration_jobs WHERE uuid=?", -1, &statement, NULL));
    sqlite3_bind_text(statement, 1, uuid, -1, SQLITE_TRANSIENT);
    int result = sqlite3_step(statement); int cleaned = result == SQLITE_ROW ? sqlite3_column_int(statement, 0) : -1;
    sqlite3_finalize(statement); TEST_ASSERT_EQUAL_INT(SQLITE_ROW, result); return cleaned;
}
static void assert_cleanup_pending(storage_migration_job_t *job, const char *state) {
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_MIGRATION_OK, db_storage_migration_get(job->uuid, job));
    TEST_ASSERT_EQUAL_STRING(state, job->state); TEST_ASSERT_EQUAL_INT(0, artifacts_cleaned(job->uuid));
    TEST_ASSERT_GREATER_THAN_INT64(0, job->next_attempt_at);
    TEST_ASSERT_TRUE(job->last_error[0] != '\0');
}
static void test_abandoned_cleanup_retains_unowned_symlink(void) {
    storage_migration_job_t job; uint64_t id = create_job(&job, "move");
    TEST_ASSERT_EQUAL_INT(0, ensure_path(destination_path));
    TEST_ASSERT_EQUAL_INT(0, symlink(sentinel, destination_path));
    TEST_ASSERT_EQUAL_INT(1, storage_migration_process_one());
    TEST_ASSERT_EQUAL_INT(0, storage_migration_process_one());
    struct stat info; TEST_ASSERT_EQUAL_INT(0, lstat(destination_path, &info));
    TEST_ASSERT_TRUE(S_ISLNK(info.st_mode)); assert_bytes(sentinel, "sentinel unchanged");
    assert_source_location(id); assert_cleanup_pending(&job, "failed");
}
static void test_cancelled_job_retains_unowned_final_destination(void) {
    storage_migration_job_t job; uint64_t id = create_job(&job, "move");
    put_file(destination_path, "unowned cancelled destination");
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_MIGRATION_OK, db_storage_migration_request_cancel(job.uuid, &job));
    TEST_ASSERT_EQUAL_INT(0, storage_migration_process_one());
    assert_bytes(destination_path, "unowned cancelled destination"); assert_source_location(id);
    assert_cleanup_pending(&job, "cancelled");
}
static void test_cancelled_job_without_artifacts_completes_cleanup(void) {
    storage_migration_job_t job; uint64_t id = create_job(&job, "move");
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_MIGRATION_OK, db_storage_migration_request_cancel(job.uuid, &job));
    TEST_ASSERT_EQUAL_INT(0, storage_migration_process_one());
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_MIGRATION_OK, db_storage_migration_get(job.uuid, &job));
    TEST_ASSERT_EQUAL_STRING("cancelled", job.state); TEST_ASSERT_EQUAL_INT(1, artifacts_cleaned(job.uuid));
    assert_source_location(id);
}
static void test_matching_bytes_do_not_prove_abandoned_destination_ownership(void) {
    storage_migration_job_t job; uint64_t id = create_job(&job, "move");
    put_file(destination_path, payload);
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_MIGRATION_OK, db_storage_migration_request_cancel(job.uuid, &job));
    TEST_ASSERT_EQUAL_INT(0, storage_migration_process_one());
    assert_bytes(destination_path, payload); assert_source_location(id); assert_cleanup_pending(&job, "cancelled");
}
static void test_destination_inspection_error_keeps_cleanup_pending(void) {
    storage_migration_job_t job; uint64_t id = create_job(&job, "move");
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_MIGRATION_OK, db_storage_migration_request_cancel(job.uuid, &job));
    destination_stat_error = EACCES;
    TEST_ASSERT_EQUAL_INT(0, storage_migration_process_one());
    destination_stat_error = 0;
    assert_source_location(id); assert_cleanup_pending(&job, "cancelled");
}
static void test_preexisting_temporary_is_preserved_while_fresh_attempt_succeeds(void) {
    storage_migration_job_t job; uint64_t id = create_job(&job, "move");
    put_file(temporary_path, "preexisting temporary sentinel");
    TEST_ASSERT_EQUAL_INT(1, storage_migration_process_one());
    assert_bytes(temporary_path, "preexisting temporary sentinel");
    recording_metadata_t recording;
    TEST_ASSERT_EQUAL_INT(0, get_recording_metadata_by_id(id, &recording));
    TEST_ASSERT_EQUAL_STRING(destination_uuid, recording.storage_target_uuid);
    assert_bytes(destination_path, payload);
    TEST_ASSERT_EQUAL_INT(-1, access(source_path, F_OK));
}
static void test_preexisting_temporary_survives_failed_job_and_later_cleanup(void) {
    storage_migration_job_t job; uint64_t id = create_job(&job, "move");
    put_file(temporary_path, "unowned temporary cleanup sentinel");
    put_file(destination_path, "unowned final cleanup sentinel");
    TEST_ASSERT_EQUAL_INT(1, storage_migration_process_one());
    assert_bytes(temporary_path, "unowned temporary cleanup sentinel");
    assert_source_location(id);
    TEST_ASSERT_EQUAL_INT(0, storage_migration_process_one());
    assert_bytes(temporary_path, "unowned temporary cleanup sentinel");
    assert_bytes(destination_path, "unowned final cleanup sentinel");
    assert_source_location(id); assert_cleanup_pending(&job, "failed");
}
static void test_retry_preserves_previous_attempt_and_uses_fresh_artifact(void) {
    storage_migration_job_t job; uint64_t id = create_job(&job, "move");
    publication_error = EIO;
    TEST_ASSERT_EQUAL_INT(1, storage_migration_process_one());
    assert_source_location(id); assert_bytes(temporary_path, payload);
    publication_error = 0;
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(get_db_handle(),
        "UPDATE storage_migration_jobs SET next_attempt_at=0;", NULL, NULL, NULL));
    TEST_ASSERT_EQUAL_INT(1, storage_migration_process_one());
    assert_bytes(temporary_path, payload); assert_bytes(destination_path, payload);
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_MIGRATION_OK, db_storage_migration_get(job.uuid, &job));
    TEST_ASSERT_EQUAL_STRING("completed", job.state);
}
static void test_preexisting_temporary_symlink_and_target_survive(void) {
    storage_migration_job_t job; create_job(&job, "copy");
    TEST_ASSERT_EQUAL_INT(0, ensure_path(temporary_path));
    TEST_ASSERT_EQUAL_INT(0, symlink(sentinel, temporary_path));
    TEST_ASSERT_EQUAL_INT(1, storage_migration_process_one());
    assert_bytes(sentinel, "sentinel unchanged"); assert_bytes(destination_path, payload);
    struct stat info; TEST_ASSERT_EQUAL_INT(0, lstat(temporary_path, &info));
    TEST_ASSERT_TRUE(S_ISLNK(info.st_mode));
}
static void test_fresh_attempt_without_legacy_name_keeps_cleanup_pending(void) {
    storage_migration_job_t job; uint64_t id = create_job(&job, "move");
    char fresh[MAX_PATH_LENGTH];
    int length = snprintf(fresh, sizeof(fresh), "%s.migration-%s.%s.part", destination_path,
                          job.uuid, "11111111-1111-4111-8111-111111111111");
    TEST_ASSERT_TRUE(length > 0 && (size_t)length < sizeof(fresh));
    put_file(fresh, "unowned fresh attempt");
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_MIGRATION_OK, db_storage_migration_request_cancel(job.uuid, &job));
    TEST_ASSERT_EQUAL_INT(0, storage_migration_process_one());
    assert_bytes(fresh, "unowned fresh attempt"); assert_source_location(id);
    TEST_ASSERT_EQUAL_INT(-1, access(temporary_path, F_OK));
    assert_cleanup_pending(&job, "cancelled");
}
static void test_quarantine_unsupported_preserves_verified_temporary(void) {
    storage_migration_job_t job; uint64_t id = create_job(&job, "move");
    quarantine_error = ENOSYS;
    TEST_ASSERT_EQUAL_INT(1, storage_migration_process_one());
    assert_source_location(id); assert_bytes(temporary_path, payload);
    TEST_ASSERT_EQUAL_INT(-1, access(destination_path, F_OK));
}
static void test_post_publication_substitution_is_rejected_before_commit(void) {
    storage_migration_job_t job; uint64_t id = create_job(&job, "move");
    committing_replacement = 1;
    TEST_ASSERT_EQUAL_INT(1, storage_migration_process_one());
    assert_source_location(id); assert_bytes(verified_part, payload);
    assert_bytes(destination_path, "substituted published destination");
}
static void test_restore_collision_retains_private_entry_and_cleanup_debt(void) {
    storage_migration_job_t job; uint64_t id = create_job(&job, "move");
    replacement_kind = 3; armed = restore_collision = true;
    TEST_ASSERT_EQUAL_INT(1, storage_migration_process_one());
    assert_source_location(id); assert_bytes(verified_part, payload);
    assert_bytes(temporary_path, "restore collision sentinel");
    assert_bytes(private_artifact, "unverified replacement data");
    TEST_ASSERT_EQUAL_INT(0, unlink(temporary_path)); /* Remove only test-created sentinel. */
    TEST_ASSERT_EQUAL_INT(0, storage_migration_process_one());
    assert_bytes(private_artifact, "unverified replacement data");
    assert_cleanup_pending(&job, "failed");
}
static void test_failed_commit_does_not_unlink_substituted_destination(void) {
    storage_migration_job_t job; uint64_t id = create_job(&job, "move");
    commit_failure_replacement = true;
    TEST_ASSERT_EQUAL_INT(1, storage_migration_process_one());
    assert_source_location(id); assert_bytes(verified_part, payload);
    assert_bytes(destination_path, "failed commit substitution sentinel");
}
static void test_cancelled_publication_does_not_unlink_substituted_destination(void) {
    storage_migration_job_t job; uint64_t id = create_job(&job, "move");
    cancel_after_publication = true;
    TEST_ASSERT_EQUAL_INT(1, storage_migration_process_one());
    assert_source_location(id); assert_bytes(verified_part, payload);
    assert_bytes(destination_path, "cancelled substitution sentinel");
    TEST_ASSERT_EQUAL_INT(0, storage_migration_process_one());
    assert_cleanup_pending(&job, "cancelled");
}
static void test_modified_verified_inode_is_rejected(void) {
    storage_migration_job_t job; uint64_t id = create_job(&job, "move");
    replacement_kind = 4; armed = true;
    TEST_ASSERT_EQUAL_INT(1, storage_migration_process_one()); TEST_ASSERT_TRUE(injected);
    assert_source_location(id); assert_bytes(temporary_path, "modified verified inode");
    TEST_ASSERT_EQUAL_INT(-1, access(destination_path, F_OK));
}
static void test_parent_replacement_with_same_file_inode_is_rejected(void) {
    storage_migration_job_t job; uint64_t id = create_job(&job, "move");
    committing_replacement = 2;
    TEST_ASSERT_EQUAL_INT(1, storage_migration_process_one());
    assert_source_location(id); assert_bytes(destination_path, payload);
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_MIGRATION_OK, db_storage_migration_get(job.uuid, &job));
    TEST_ASSERT_EQUAL_STRING("failed", job.state);
}
int main(void) {
    strcpy(root, "/tmp/lightnvr-t7-26-XXXXXX"); if (!mkdtemp(root)) return 2;
    snprintf(source_root, sizeof(source_root), "%s/source", root);
    snprintf(destination_root, sizeof(destination_root), "%s/destination", root);
    snprintf(db_path, sizeof(db_path), "%s/database.sqlite", root);
    snprintf(relocated_parent, sizeof(relocated_parent), "%s/relocated-parent", root);
    snprintf(source_path, sizeof(source_path), "%s/camera/clip.mp4", source_root);
    snprintf(destination_path, sizeof(destination_path), "%s/camera/clip.mp4", destination_root);
    snprintf(sentinel, sizeof(sentinel), "%s/sentinel", root);
    snprintf(verified_part, sizeof(verified_part), "%s/verified-part", root);
    load_default_config(&g_config);
    if (init_database_ex(db_path, DB_INIT_NO_BACKUP) != 0) { free(g_config.streams); remove_owned_tree(root); return 2; }
    UNITY_BEGIN();
    RUN_TEST(test_normal_move_commits_verified_destination_and_removes_source);
    RUN_TEST(test_normal_copy_keeps_source_and_verified_replica);
    RUN_TEST(test_regular_destination_appearing_after_check_is_not_overwritten);
    RUN_TEST(test_symlink_destination_appearing_after_check_is_retained);
    RUN_TEST(test_existing_collision_survives_later_abandoned_cleanup);
    RUN_TEST(test_unverified_temporary_replacement_is_not_committed);
    RUN_TEST(test_unavailable_registered_source_preserves_metadata_and_bytes);
    RUN_TEST(test_unsupported_atomic_publication_retains_source_and_temporary);
    RUN_TEST(test_unsupported_filesystem_publication_retains_source_and_temporary);
    RUN_TEST(test_publication_io_failure_retains_source_and_temporary);
    RUN_TEST(test_collision_cleanup_failure_never_overwrites_competing_destination);
    RUN_TEST(test_abandoned_cleanup_retains_unowned_symlink);
    RUN_TEST(test_cancelled_job_retains_unowned_final_destination);
    RUN_TEST(test_cancelled_job_without_artifacts_completes_cleanup);
    RUN_TEST(test_matching_bytes_do_not_prove_abandoned_destination_ownership);
    RUN_TEST(test_destination_inspection_error_keeps_cleanup_pending);
    RUN_TEST(test_preexisting_temporary_is_preserved_while_fresh_attempt_succeeds);
    RUN_TEST(test_preexisting_temporary_survives_failed_job_and_later_cleanup);
    RUN_TEST(test_retry_preserves_previous_attempt_and_uses_fresh_artifact);
    RUN_TEST(test_preexisting_temporary_symlink_and_target_survive);
    RUN_TEST(test_fresh_attempt_without_legacy_name_keeps_cleanup_pending);
    RUN_TEST(test_quarantine_unsupported_preserves_verified_temporary);
    RUN_TEST(test_post_publication_substitution_is_rejected_before_commit);
    RUN_TEST(test_restore_collision_retains_private_entry_and_cleanup_debt);
    RUN_TEST(test_failed_commit_does_not_unlink_substituted_destination);
    RUN_TEST(test_cancelled_publication_does_not_unlink_substituted_destination);
    RUN_TEST(test_modified_verified_inode_is_rejected);
    RUN_TEST(test_parent_replacement_with_same_file_inode_is_rejected);
    int result = UNITY_END(); shutdown_database(); free(g_config.streams); g_config.streams = NULL;
    remove_owned_tree(root); return result;
}
