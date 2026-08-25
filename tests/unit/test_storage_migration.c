#define _POSIX_C_SOURCE 200809L

#include <sqlite3.h>
#include <stdio.h>
#include <string.h>
#include <sys/stat.h>
#include <unistd.h>

#include "core/path_utils.h"
#include "database/db_core.h"
#include "database/db_recordings.h"
#include "database/db_storage_migrations.h"
#include "database/db_storage_targets.h"
#include "storage/storage_migration.h"
#include "unity.h"
#include "utils/strings.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_storage_migration.db"

static char source_root[] = "/tmp/lightnvr-migration-source-XXXXXX";
static char destination_root[] = "/tmp/lightnvr-migration-destination-XXXXXX";
static char source_uuid[LIGHTNVR_UUID_STRING_SIZE];
static char destination_uuid[LIGHTNVR_UUID_STRING_SIZE];
static char source_path[MAX_PATH_LENGTH];
static char destination_path[MAX_PATH_LENGTH];

static void write_payload(const char *path, const char *payload) {
    TEST_ASSERT_EQUAL_INT(0, ensure_path(path));
    FILE *file = fopen(path, "wb");
    TEST_ASSERT_NOT_NULL(file);
    size_t length = strlen(payload);
    TEST_ASSERT_EQUAL_size_t(length, fwrite(payload, 1, length, file));
    TEST_ASSERT_EQUAL_INT(0, fclose(file));
}

static uint64_t add_complete_recording(const char *payload) {
    write_payload(source_path, payload);
    recording_metadata_t recording;
    memset(&recording, 0, sizeof(recording));
    safe_strcpy(recording.stream_name, "migration-camera",
                sizeof(recording.stream_name), 0);
    safe_strcpy(recording.camera_uuid,
                "11111111-1111-4111-8111-111111111111",
                sizeof(recording.camera_uuid), 0);
    safe_strcpy(recording.file_path, source_path,
                sizeof(recording.file_path), 0);
    safe_strcpy(recording.storage_target_uuid, source_uuid,
                sizeof(recording.storage_target_uuid), 0);
    safe_strcpy(recording.object_key, "camera/clip.mp4",
                sizeof(recording.object_key), 0);
    safe_strcpy(recording.placement_reason, "test",
                sizeof(recording.placement_reason), 0);
    safe_strcpy(recording.codec, "h264", sizeof(recording.codec), 0);
    safe_strcpy(recording.trigger_type, "continuous",
                sizeof(recording.trigger_type), 0);
    recording.start_time = 100;
    recording.end_time = 200;
    recording.size_bytes = strlen(payload);
    recording.width = 1920;
    recording.height = 1080;
    recording.fps = 30;
    recording.is_complete = true;
    recording.retention_override_days = -1;
    recording.retention_tier = RETENTION_TIER_STANDARD;
    recording.disk_pressure_eligible = true;
    recording.schedule_restricted = 0;
    uint64_t id = add_recording_metadata(&recording);
    TEST_ASSERT_NOT_EQUAL_UINT64(0, id);
    return id;
}

static void configure_targets(void) {
    TEST_ASSERT_EQUAL_INT(
        0, db_storage_target_bootstrap_default(source_root, source_uuid));
    storage_target_t source;
    TEST_ASSERT_EQUAL_INT(
        DB_STORAGE_TARGET_OK,
        db_storage_target_probe(source_uuid, true, &source));

    storage_target_t destination;
    memset(&destination, 0, sizeof(destination));
    safe_strcpy(destination.name, "Migration destination",
                sizeof(destination.name), 0);
    safe_strcpy(destination.target_type, "filesystem",
                sizeof(destination.target_type), 0);
    safe_strcpy(destination.root_path, destination_root,
                sizeof(destination.root_path), 0);
    safe_strcpy(destination.storage_class, "warm",
                sizeof(destination.storage_class), 0);
    destination.enabled = true;
    destination.high_watermark_pct = 90.0;
    destination.low_watermark_pct = 80.0;
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_TARGET_OK,
                          db_storage_target_create(&destination));
    safe_strcpy(destination_uuid, destination.uuid,
                sizeof(destination_uuid), 0);
}

void setUp(void) {
    sqlite3 *db = get_db_handle();
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(
        db, "DELETE FROM storage_migration_jobs;DELETE FROM detections;"
            "DELETE FROM recordings;DELETE FROM storage_targets;",
        NULL, NULL, NULL));
    unlink(source_path);
    unlink(destination_path);
    configure_targets();
}

void tearDown(void) {
    unlink(source_path);
    unlink(destination_path);
}

void test_verified_move_commits_location_then_removes_source(void) {
    const char *payload = "durable migration payload";
    uint64_t recording_id = add_complete_recording(payload);
    storage_migration_job_t job;
    TEST_ASSERT_EQUAL_INT(
        DB_STORAGE_MIGRATION_OK,
        db_storage_migration_create(recording_id, destination_uuid, 0, &job));
    TEST_ASSERT_EQUAL_STRING("queued", job.state);

    TEST_ASSERT_EQUAL_INT(1, storage_migration_process_one());
    TEST_ASSERT_EQUAL_INT(
        DB_STORAGE_MIGRATION_OK,
        db_storage_migration_get(job.uuid, &job));
    TEST_ASSERT_EQUAL_STRING("completed", job.state);
    TEST_ASSERT_EQUAL_UINT64(strlen(payload), job.bytes_copied);
    TEST_ASSERT_EQUAL_size_t(64, strlen(job.checksum));
    TEST_ASSERT_NOT_EQUAL_INT(0, access(source_path, F_OK));
    TEST_ASSERT_EQUAL_INT(0, access(destination_path, R_OK));

    recording_metadata_t recording;
    TEST_ASSERT_EQUAL_INT(
        0, get_recording_metadata_by_id(recording_id, &recording));
    TEST_ASSERT_EQUAL_STRING(destination_uuid,
                             recording.storage_target_uuid);
    TEST_ASSERT_EQUAL_STRING(destination_path, recording.file_path);

    storage_target_t source;
    storage_target_t destination;
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_TARGET_OK,
                          db_storage_target_get(source_uuid, &source));
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_TARGET_OK,
                          db_storage_target_get(destination_uuid,
                                                &destination));
    TEST_ASSERT_EQUAL_UINT64(0, source.recording_count);
    TEST_ASSERT_EQUAL_UINT64(1, destination.recording_count);
}

void test_restart_interrupted_copy_is_reclaimed_and_completed(void) {
    uint64_t recording_id = add_complete_recording("restart-safe payload");
    storage_migration_job_t job;
    TEST_ASSERT_EQUAL_INT(
        DB_STORAGE_MIGRATION_OK,
        db_storage_migration_create(recording_id, destination_uuid, 0, &job));
    char sql[256];
    snprintf(sql, sizeof(sql),
             "UPDATE storage_migration_jobs SET state='copying',"
             "bytes_copied=3 WHERE uuid='%s';", job.uuid);
    TEST_ASSERT_EQUAL_INT(SQLITE_OK,
                          sqlite3_exec(get_db_handle(), sql, NULL, NULL, NULL));

    TEST_ASSERT_EQUAL_INT(1, storage_migration_process_one());
    TEST_ASSERT_EQUAL_INT(
        DB_STORAGE_MIGRATION_OK,
        db_storage_migration_get(job.uuid, &job));
    TEST_ASSERT_EQUAL_STRING("completed", job.state);
    TEST_ASSERT_EQUAL_INT(1, job.attempt_count);
}

void test_destination_collision_fails_without_changing_source(void) {
    uint64_t recording_id = add_complete_recording("source payload");
    write_payload(destination_path, "different destination payload");
    storage_migration_job_t job;
    TEST_ASSERT_EQUAL_INT(
        DB_STORAGE_MIGRATION_OK,
        db_storage_migration_create(recording_id, destination_uuid, 0, &job));

    TEST_ASSERT_EQUAL_INT(1, storage_migration_process_one());
    TEST_ASSERT_EQUAL_INT(
        DB_STORAGE_MIGRATION_OK,
        db_storage_migration_get(job.uuid, &job));
    TEST_ASSERT_EQUAL_STRING("failed", job.state);
    TEST_ASSERT_EQUAL_INT(0, access(source_path, R_OK));
    recording_metadata_t recording;
    TEST_ASSERT_EQUAL_INT(
        0, get_recording_metadata_by_id(recording_id, &recording));
    TEST_ASSERT_EQUAL_STRING(source_uuid, recording.storage_target_uuid);
}

int main(void) {
    TEST_ASSERT_NOT_NULL(mkdtemp(source_root));
    TEST_ASSERT_NOT_NULL(mkdtemp(destination_root));
    snprintf(source_path, sizeof(source_path), "%s/camera/clip.mp4",
             source_root);
    snprintf(destination_path, sizeof(destination_path), "%s/camera/clip.mp4",
             destination_root);
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: failed to initialize migration test database\n");
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_verified_move_commits_location_then_removes_source);
    RUN_TEST(test_restart_interrupted_copy_is_reclaimed_and_completed);
    RUN_TEST(test_destination_collision_fails_without_changing_source);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    unlink(source_path);
    unlink(destination_path);
    char source_camera[MAX_PATH_LENGTH];
    char destination_camera[MAX_PATH_LENGTH];
    snprintf(source_camera, sizeof(source_camera), "%s/camera", source_root);
    snprintf(destination_camera, sizeof(destination_camera), "%s/camera",
             destination_root);
    rmdir(source_camera);
    rmdir(destination_camera);
    rmdir(source_root);
    rmdir(destination_root);
    return result;
}
