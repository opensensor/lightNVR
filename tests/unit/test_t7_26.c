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
static int replacement_kind;
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
    else {
        TEST_ASSERT_EQUAL_INT(0, rename(temporary_path, verified_part));
        put_file(temporary_path, "unverified replacement data");
    }
}
int __real_lstat(const char *, struct stat *);
int __wrap_lstat(const char *path, struct stat *info) {
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
    if (armed && (!strcmp(newname, destination_path) || !strcmp(newname, "clip.mp4"))) inject_before_publication();
    return __real_syscall(number, oldfd, oldname, newfd, newname, flags);
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
    armed = injected = false; replacement_kind = 0; temporary_path[0] = '\0';
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(get_db_handle(),
        "DELETE FROM storage_migration_jobs;DELETE FROM detections;DELETE FROM recordings;DELETE FROM storage_targets;",
        NULL, NULL, NULL));
    remove_owned_tree(source_root); remove_owned_tree(destination_root);
    TEST_ASSERT_EQUAL_INT(0, mkdir(source_root, 0700)); TEST_ASSERT_EQUAL_INT(0, mkdir(destination_root, 0700));
    unlink(sentinel); unlink(verified_part); put_file(sentinel, "sentinel unchanged");
    configure_targets();
}
void tearDown(void) { armed = false; }
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
int main(void) {
    strcpy(root, "/tmp/lightnvr-t7-26-XXXXXX"); if (!mkdtemp(root)) return 2;
    snprintf(source_root, sizeof(source_root), "%s/source", root);
    snprintf(destination_root, sizeof(destination_root), "%s/destination", root);
    snprintf(db_path, sizeof(db_path), "%s/database.sqlite", root);
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
    int result = UNITY_END(); shutdown_database(); free(g_config.streams); g_config.streams = NULL;
    remove_owned_tree(root); return result;
}
