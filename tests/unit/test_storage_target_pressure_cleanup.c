/**
 * @file test_storage_target_pressure_cleanup.c
 * @brief Target pressure cleanup never crosses storage-target boundaries.
 */

#define _POSIX_C_SOURCE 200809L

#include <fcntl.h>
#include <stdint.h>
#include <sqlite3.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include "database/db_core.h"
#include "database/db_recordings.h"
#include "database/db_storage_targets.h"
#include "storage/storage_manager.h"
#include "unity.h"
#include "utils/strings.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_target_pressure_cleanup.db"

static char root_a[] = "/tmp/lightnvr-pressure-target-a-XXXXXX";
static char root_b[] = "/tmp/lightnvr-pressure-target-b-XXXXXX";
/* Deliberately not registered as a target, so rows here stay unattributed. */
static char root_legacy[] = "/tmp/lightnvr-pressure-legacy-XXXXXX";
static storage_target_t target_a;
static storage_target_t target_b;

static void make_target(storage_target_t *target, const char *name,
                        const char *root) {
    memset(target, 0, sizeof(*target));
    safe_strcpy(target->name, name, sizeof(target->name), 0);
    safe_strcpy(target->target_type, "filesystem",
                sizeof(target->target_type), 0);
    safe_strcpy(target->root_path, root, sizeof(target->root_path), 0);
    safe_strcpy(target->storage_class, "hot",
                sizeof(target->storage_class), 0);
    target->enabled = true;
    target->high_watermark_pct = 90.0;
    target->low_watermark_pct = 80.0;
    /* Force the reserve-pressure path independently of host disk usage. */
    target->reserve_bytes = (uint64_t)INT64_MAX;
}

static void write_file(const char *path) {
    int descriptor = open(path, O_CREAT | O_TRUNC | O_WRONLY, 0644);
    TEST_ASSERT_TRUE(descriptor >= 0);
    TEST_ASSERT_EQUAL_INT(4, write(descriptor, "test", 4));
    close(descriptor);
}

static uint64_t add_target_recording(const storage_target_t *target,
                                     const char *object_key,
                                     const char *stream_name,
                                     char path[MAX_PATH_LENGTH]) {
    size_t root_length = strlen(target->root_path);
    size_t key_length = strlen(object_key);
    TEST_ASSERT_TRUE(root_length + 1 + key_length < MAX_PATH_LENGTH);
    memcpy(path, target->root_path, root_length);
    path[root_length] = '/';
    memcpy(path + root_length + 1, object_key, key_length + 1);
    write_file(path);
    recording_metadata_t recording;
    memset(&recording, 0, sizeof(recording));
    safe_strcpy(recording.stream_name, stream_name,
                sizeof(recording.stream_name), 0);
    safe_strcpy(recording.file_path, path, sizeof(recording.file_path), 0);
    safe_strcpy(recording.storage_target_uuid, target->uuid,
                sizeof(recording.storage_target_uuid), 0);
    safe_strcpy(recording.object_key, object_key,
                sizeof(recording.object_key), 0);
    safe_strcpy(recording.codec, "h264", sizeof(recording.codec), 0);
    safe_strcpy(recording.trigger_type, "scheduled",
                sizeof(recording.trigger_type), 0);
    recording.start_time = 100;
    recording.end_time = 160;
    recording.size_bytes = 4;
    recording.is_complete = true;
    recording.retention_override_days = -1;
    recording.retention_tier = RETENTION_TIER_EPHEMERAL;
    recording.disk_pressure_eligible = true;
    return add_recording_metadata(&recording);
}

/*
 * A recording from before storage targets existed: its path sits outside every
 * configured root, so add_recording_metadata() cannot classify it and the row
 * keeps a NULL storage_target_uuid.
 */
static uint64_t add_unattributed_recording(const char *file_name,
                                           const char *stream_name,
                                           char path[MAX_PATH_LENGTH]) {
    snprintf(path, MAX_PATH_LENGTH, "%s/%s", root_legacy, file_name);
    write_file(path);
    recording_metadata_t recording;
    memset(&recording, 0, sizeof(recording));
    safe_strcpy(recording.stream_name, stream_name,
                sizeof(recording.stream_name), 0);
    safe_strcpy(recording.file_path, path, sizeof(recording.file_path), 0);
    safe_strcpy(recording.codec, "h264", sizeof(recording.codec), 0);
    safe_strcpy(recording.trigger_type, "scheduled",
                sizeof(recording.trigger_type), 0);
    recording.start_time = 100;
    recording.end_time = 160;
    recording.size_bytes = 4;
    recording.is_complete = true;
    recording.retention_override_days = -1;
    recording.retention_tier = RETENTION_TIER_EPHEMERAL;
    recording.disk_pressure_eligible = true;
    uint64_t id = add_recording_metadata(&recording);
    recording_metadata_t stored;
    TEST_ASSERT_EQUAL_INT(0, get_recording_metadata_by_id(id, &stored));
    TEST_ASSERT_EQUAL_STRING("", stored.storage_target_uuid);
    return id;
}

void setUp(void) {
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(
        get_db_handle(),
        "DELETE FROM recordings;DELETE FROM storage_targets;",
        NULL, NULL, NULL));
    make_target(&target_a, "Pressure A", root_a);
    make_target(&target_b, "Pressure B", root_b);
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_TARGET_OK,
                          db_storage_target_create(&target_a));
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_TARGET_OK,
                          db_storage_target_create(&target_b));
}

void tearDown(void) {}

void test_cleanup_deletes_only_recordings_on_pressured_target(void) {
    char path_a[MAX_PATH_LENGTH];
    char path_b[MAX_PATH_LENGTH];
    uint64_t id_a = add_target_recording(
        &target_a, "a.mp4", "pressure-a", path_a);
    uint64_t id_b = add_target_recording(
        &target_b, "b.mp4", "pressure-b", path_b);
    TEST_ASSERT_NOT_EQUAL(0, id_a);
    TEST_ASSERT_NOT_EQUAL(0, id_b);

    storage_target_cleanup_result_t result;
    TEST_ASSERT_EQUAL_INT(
        0, storage_cleanup_target_pressure(target_a.uuid, &result));
    TEST_ASSERT_EQUAL_INT(STORAGE_TARGET_PRESSURE_RESERVE,
                          result.initial_pressure);
    TEST_ASSERT_EQUAL_INT(1, result.deleted_recordings);
    TEST_ASSERT_EQUAL_INT(-1, access(path_a, F_OK));
    TEST_ASSERT_EQUAL_INT(0, access(path_b, F_OK));

    recording_metadata_t metadata;
    TEST_ASSERT_NOT_EQUAL_INT(0, get_recording_metadata_by_id(id_a, &metadata));
    TEST_ASSERT_EQUAL_INT(0, get_recording_metadata_by_id(id_b, &metadata));
    TEST_ASSERT_EQUAL_STRING(target_b.uuid, metadata.storage_target_uuid);
}

/*
 * Rows that predate target attribution belong to no target, so a specific
 * target's cleanup must not reclaim them -- it has no evidence they sit on its
 * filesystem.
 */
void test_target_cleanup_ignores_unattributed_recordings(void) {
    char path_a[MAX_PATH_LENGTH];
    char path_legacy[MAX_PATH_LENGTH];
    uint64_t id_a = add_target_recording(
        &target_a, "a.mp4", "pressure-a", path_a);
    uint64_t id_legacy = add_unattributed_recording(
        "legacy.mp4", "pressure-legacy", path_legacy);
    TEST_ASSERT_NOT_EQUAL(0, id_a);
    TEST_ASSERT_NOT_EQUAL(0, id_legacy);

    storage_target_cleanup_result_t result;
    TEST_ASSERT_EQUAL_INT(
        0, storage_cleanup_target_pressure(target_a.uuid, &result));
    TEST_ASSERT_EQUAL_INT(1, result.deleted_recordings);
    TEST_ASSERT_EQUAL_INT(-1, access(path_a, F_OK));
    TEST_ASSERT_EQUAL_INT(0, access(path_legacy, F_OK));

    recording_metadata_t metadata;
    TEST_ASSERT_EQUAL_INT(0,
                          get_recording_metadata_by_id(id_legacy, &metadata));
}

/*
 * The default target's cleanup is the only one that may claim them, otherwise
 * an upgraded install's footage is invisible to every disk-pressure path.
 */
void test_default_target_query_includes_unattributed_recordings(void) {
    char path_a[MAX_PATH_LENGTH];
    char path_b[MAX_PATH_LENGTH];
    char path_legacy[MAX_PATH_LENGTH];
    TEST_ASSERT_NOT_EQUAL(
        0, add_target_recording(&target_a, "a.mp4", "pressure-a", path_a));
    TEST_ASSERT_NOT_EQUAL(
        0, add_target_recording(&target_b, "b.mp4", "pressure-b", path_b));
    uint64_t id_legacy = add_unattributed_recording(
        "legacy.mp4", "pressure-legacy", path_legacy);

    recording_metadata_t rows[8];
    memset(rows, 0, sizeof(rows));
    int strict = get_recordings_for_pressure_cleanup_target(
        target_a.uuid, rows, 8);
    TEST_ASSERT_EQUAL_INT(1, strict);
    TEST_ASSERT_EQUAL_STRING(target_a.uuid, rows[0].storage_target_uuid);

    memset(rows, 0, sizeof(rows));
    int lenient = get_recordings_for_pressure_cleanup_default_target(
        target_a.uuid, rows, 8);
    TEST_ASSERT_EQUAL_INT(2, lenient);

    bool saw_target_a = false;
    bool saw_legacy = false;
    for (int index = 0; index < lenient; index++) {
        /* Never another target's rows, whichever way they are ordered. */
        TEST_ASSERT_NOT_EQUAL_INT(
            0, strcmp(target_b.uuid, rows[index].storage_target_uuid));
        if (rows[index].id == id_legacy) {
            saw_legacy = true;
            TEST_ASSERT_EQUAL_STRING("", rows[index].storage_target_uuid);
        } else if (strcmp(rows[index].storage_target_uuid,
                          target_a.uuid) == 0) {
            saw_target_a = true;
        }
    }
    TEST_ASSERT_TRUE(saw_target_a);
    TEST_ASSERT_TRUE(saw_legacy);
}

int main(void) {
    TEST_ASSERT_NOT_NULL(mkdtemp(root_a));
    TEST_ASSERT_NOT_NULL(mkdtemp(root_b));
    TEST_ASSERT_NOT_NULL(mkdtemp(root_legacy));
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) return 1;
    UNITY_BEGIN();
    RUN_TEST(test_cleanup_deletes_only_recordings_on_pressured_target);
    RUN_TEST(test_target_cleanup_ignores_unattributed_recordings);
    RUN_TEST(test_default_target_query_includes_unattributed_recordings);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    char path[MAX_PATH_LENGTH];
    snprintf(path, sizeof(path), "%s/a.mp4", root_a);
    unlink(path);
    snprintf(path, sizeof(path), "%s/b.mp4", root_b);
    unlink(path);
    snprintf(path, sizeof(path), "%s/legacy.mp4", root_legacy);
    unlink(path);
    rmdir(root_a);
    rmdir(root_b);
    rmdir(root_legacy);
    return result;
}
