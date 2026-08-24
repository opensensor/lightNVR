/**
 * @file test_db_storage_targets.c
 * @brief Storage target migration, probing, identity, and safety tests.
 */

#define _POSIX_C_SOURCE 200809L

#include <sqlite3.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <unistd.h>

#include "database/db_core.h"
#include "database/db_recordings.h"
#include "database/db_storage_targets.h"
#include "unity.h"
#include "utils/strings.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_storage_targets.db"

static char default_root[] = "/tmp/lightnvr-target-default-XXXXXX";
static char second_root[] = "/tmp/lightnvr-target-second-XXXXXX";
static char moved_root[] = "/tmp/lightnvr-target-moved-XXXXXX";
static char default_uuid[LIGHTNVR_UUID_STRING_SIZE];
static const char *mountinfo_fixture =
    "/tmp/lightnvr_unit_storage_mountinfo.txt";

static storage_target_t valid_target(const char *name, const char *root) {
    storage_target_t target;
    memset(&target, 0, sizeof(target));
    safe_strcpy(target.name, name, sizeof(target.name), 0);
    safe_strcpy(target.target_type, "filesystem",
                sizeof(target.target_type), 0);
    safe_strcpy(target.root_path, root, sizeof(target.root_path), 0);
    target.enabled = true;
    safe_strcpy(target.storage_class, "hot",
                sizeof(target.storage_class), 0);
    target.high_watermark_pct = 90.0;
    target.low_watermark_pct = 80.0;
    return target;
}

void setUp(void) {
    sqlite3 *db = get_db_handle();
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(
        db, "DELETE FROM detections;DELETE FROM recordings;"
            "DELETE FROM storage_targets;", NULL, NULL, NULL));
    TEST_ASSERT_EQUAL_INT(
        0, db_storage_target_bootstrap_default(default_root, default_uuid));
}

void tearDown(void) {}

void test_bootstrap_attaches_absolute_paths_without_moving_files(void) {
    recording_metadata_t recording;
    memset(&recording, 0, sizeof(recording));
    safe_strcpy(recording.stream_name, "cam-storage",
                sizeof(recording.stream_name), 0);
    snprintf(recording.file_path, sizeof(recording.file_path),
             "%s/mp4/cam-storage/segment.mp4", default_root);
    recording.start_time = 100;
    recording.end_time = 200;
    recording.is_complete = true;
    uint64_t id = add_recording_metadata(&recording);
    TEST_ASSERT_NOT_EQUAL_UINT64(0, id);

    recording_metadata_t loaded;
    memset(&loaded, 0, sizeof(loaded));
    TEST_ASSERT_EQUAL_INT(0, get_recording_metadata_by_id(id, &loaded));
    TEST_ASSERT_EQUAL_STRING(default_uuid, loaded.storage_target_uuid);
    TEST_ASSERT_EQUAL_STRING("mp4/cam-storage/segment.mp4",
                             loaded.object_key);
    TEST_ASSERT_EQUAL_STRING("default-target", loaded.placement_reason);
    TEST_ASSERT_EQUAL_STRING(recording.file_path, loaded.file_path);

    TEST_ASSERT_EQUAL_INT(
        SQLITE_OK,
        sqlite3_exec(get_db_handle(),
                     "UPDATE recordings SET storage_target_uuid=NULL,"
                     "object_key=NULL,placement_reason=NULL;", NULL, NULL,
                     NULL));
    TEST_ASSERT_EQUAL_INT(
        0, db_storage_target_bootstrap_default(default_root, default_uuid));
    TEST_ASSERT_EQUAL_INT(0, get_recording_metadata_by_id(id, &loaded));
    TEST_ASSERT_EQUAL_STRING("legacy-default", loaded.placement_reason);
    TEST_ASSERT_EQUAL_STRING("mp4/cam-storage/segment.mp4",
                             loaded.object_key);
}

void test_resolver_follows_target_root_and_rejects_traversal(void) {
    char resolved[MAX_PATH_LENGTH];
    TEST_ASSERT_EQUAL_INT(
        0, db_storage_target_resolve_path(default_uuid,
                                          "mp4/cam/segment.mp4", resolved));
    char expected[MAX_PATH_LENGTH];
    snprintf(expected, sizeof(expected), "%s/mp4/cam/segment.mp4",
             default_root);
    TEST_ASSERT_EQUAL_STRING(expected, resolved);
    TEST_ASSERT_EQUAL_INT(
        -1, db_storage_target_resolve_path(default_uuid,
                                           "../outside.mp4", resolved));
    TEST_ASSERT_EQUAL_INT(
        -1, db_storage_target_resolve_path(default_uuid,
                                           "/absolute.mp4", resolved));
}

void test_mount_detection_uses_most_specific_non_root_mount(void) {
    FILE *fixture = fopen(mountinfo_fixture, "w");
    TEST_ASSERT_NOT_NULL(fixture);
    fputs("36 25 0:31 / / rw,relatime - overlay overlay rw\n", fixture);
    fputs("41 36 0:45 / /mnt/nvr\\040hot rw,relatime - nfs server:/nvr rw\n",
          fixture);
    fputs("42 41 0:46 / /mnt/nvr\\040hot/archive rw,relatime - nfs server:/archive rw\n",
          fixture);
    fclose(fixture);

    char mount_path[MAX_PATH_LENGTH];
    TEST_ASSERT_EQUAL_INT(
        0, db_storage_target_detect_mount(
               "/mnt/nvr hot/archive/camera-1", mountinfo_fixture,
               mount_path));
    TEST_ASSERT_EQUAL_STRING("/mnt/nvr hot/archive", mount_path);
    TEST_ASSERT_EQUAL_INT(
        -1, db_storage_target_detect_mount(
                "/var/lib/lightnvr", mountinfo_fixture, mount_path));
    unlink(mountinfo_fixture);
}

void test_explicit_recording_identity_must_resolve_to_file_path(void) {
    recording_metadata_t recording;
    memset(&recording, 0, sizeof(recording));
    safe_strcpy(recording.stream_name, "cam-explicit",
                sizeof(recording.stream_name), 0);
    safe_strcpy(recording.storage_target_uuid, default_uuid,
                sizeof(recording.storage_target_uuid), 0);
    safe_strcpy(recording.object_key, "cam-explicit/segment.mp4",
                sizeof(recording.object_key), 0);
    snprintf(recording.file_path, sizeof(recording.file_path),
             "%s/cam-explicit/segment.mp4", default_root);
    recording.start_time = 100;
    uint64_t id = add_recording_metadata(&recording);
    TEST_ASSERT_NOT_EQUAL_UINT64(0, id);

    safe_strcpy(recording.object_key, "../outside.mp4",
                sizeof(recording.object_key), 0);
    TEST_ASSERT_EQUAL_UINT64(0, add_recording_metadata(&recording));
    safe_strcpy(recording.object_key, "cam-explicit/different.mp4",
                sizeof(recording.object_key), 0);
    TEST_ASSERT_EQUAL_UINT64(0, add_recording_metadata(&recording));
}

void test_configured_root_change_rotates_default_without_orphaning_history(void) {
    recording_metadata_t old_recording;
    memset(&old_recording, 0, sizeof(old_recording));
    safe_strcpy(old_recording.stream_name, "cam-old",
                sizeof(old_recording.stream_name), 0);
    snprintf(old_recording.file_path, sizeof(old_recording.file_path),
             "%s/cam-old/segment.mp4", default_root);
    old_recording.start_time = 100;
    uint64_t old_id = add_recording_metadata(&old_recording);
    TEST_ASSERT_NOT_EQUAL_UINT64(0, old_id);
    char previous_uuid[LIGHTNVR_UUID_STRING_SIZE];
    safe_strcpy(previous_uuid, default_uuid, sizeof(previous_uuid), 0);

    TEST_ASSERT_EQUAL_INT(
        0, db_storage_target_bootstrap_default(moved_root, default_uuid));
    TEST_ASSERT_NOT_EQUAL(0, strcmp(previous_uuid, default_uuid));
    TEST_ASSERT_EQUAL_INT(2, db_storage_target_count());

    storage_target_t previous;
    storage_target_t current;
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_TARGET_OK,
                          db_storage_target_get(previous_uuid, &previous));
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_TARGET_OK,
                          db_storage_target_get(default_uuid, &current));
    TEST_ASSERT_FALSE(previous.is_default);
    TEST_ASSERT_TRUE(current.is_default);
    TEST_ASSERT_EQUAL_STRING(default_root, previous.root_path);
    TEST_ASSERT_EQUAL_STRING(moved_root, current.root_path);

    recording_metadata_t loaded;
    memset(&loaded, 0, sizeof(loaded));
    TEST_ASSERT_EQUAL_INT(0, get_recording_metadata_by_id(old_id, &loaded));
    TEST_ASSERT_EQUAL_STRING(previous_uuid, loaded.storage_target_uuid);
    TEST_ASSERT_EQUAL_STRING(old_recording.file_path, loaded.file_path);

    recording_metadata_t new_recording;
    memset(&new_recording, 0, sizeof(new_recording));
    safe_strcpy(new_recording.stream_name, "cam-new",
                sizeof(new_recording.stream_name), 0);
    snprintf(new_recording.file_path, sizeof(new_recording.file_path),
             "%s/cam-new/segment.mp4", moved_root);
    new_recording.start_time = 200;
    uint64_t new_id = add_recording_metadata(&new_recording);
    TEST_ASSERT_NOT_EQUAL_UINT64(0, new_id);
    TEST_ASSERT_EQUAL_INT(0, get_recording_metadata_by_id(new_id, &loaded));
    TEST_ASSERT_EQUAL_STRING(default_uuid, loaded.storage_target_uuid);
}

void test_create_probe_revision_and_duplicate_device_detection(void) {
    storage_target_t target = valid_target("Second disk", second_root);
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_TARGET_OK,
                          db_storage_target_create(&target));
    TEST_ASSERT_EQUAL_STRING("healthy", target.health_status);
    TEST_ASSERT_GREATER_THAN_UINT64(0, target.capacity_bytes);
    TEST_ASSERT_GREATER_THAN_INT64(0, target.last_success_at);
    TEST_ASSERT_EQUAL_INT64(1, target.revision);

    storage_target_t default_target;
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_TARGET_OK,
                          db_storage_target_probe(default_uuid, true,
                                                  &default_target));
    TEST_ASSERT_EQUAL_UINT64(default_target.filesystem_device,
                             target.filesystem_device);

    safe_strcpy(target.storage_class, "warm",
                sizeof(target.storage_class), 0);
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_TARGET_STALE,
                          db_storage_target_update(&target, 99));
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_TARGET_OK,
                          db_storage_target_update(&target, 1));
    TEST_ASSERT_EQUAL_INT64(2, target.revision);
    TEST_ASSERT_EQUAL_STRING("warm", target.storage_class);
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_TARGET_STALE,
                          db_storage_target_delete(target.uuid, 1));
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_TARGET_OK,
                          db_storage_target_delete(target.uuid, 2));
}

void test_nonempty_target_cannot_be_repointed_or_deleted(void) {
    storage_target_t target = valid_target("Evidence disk", second_root);
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_TARGET_OK,
                          db_storage_target_create(&target));

    recording_metadata_t recording;
    memset(&recording, 0, sizeof(recording));
    safe_strcpy(recording.stream_name, "cam-evidence",
                sizeof(recording.stream_name), 0);
    snprintf(recording.file_path, sizeof(recording.file_path),
             "%s/cam-evidence/segment.mp4", second_root);
    recording.start_time = 100;
    recording.end_time = 200;
    recording.is_complete = true;
    recording.size_bytes = 123;
    uint64_t recording_id = add_recording_metadata(&recording);
    TEST_ASSERT_NOT_EQUAL_UINT64(0, recording_id);

    storage_target_t loaded;
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_TARGET_OK,
                          db_storage_target_get(target.uuid, &loaded));
    TEST_ASSERT_EQUAL_UINT64(1, loaded.recording_count);
    TEST_ASSERT_EQUAL_UINT64(123, loaded.recording_bytes);
    TEST_ASSERT_EQUAL_INT(
        0, update_recording_metadata(recording_id, 200, 456, true));
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_TARGET_OK,
                          db_storage_target_get(target.uuid, &loaded));
    TEST_ASSERT_EQUAL_UINT64(1, loaded.recording_count);
    TEST_ASSERT_EQUAL_UINT64(456, loaded.recording_bytes);
    safe_strcpy(loaded.root_path, moved_root, sizeof(loaded.root_path), 0);
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_TARGET_IN_USE,
                          db_storage_target_update(&loaded,
                                                   loaded.revision));
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_TARGET_IN_USE,
                          db_storage_target_delete(target.uuid,
                                                   target.revision));
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_TARGET_IN_USE,
                          db_storage_target_delete(default_uuid, 1));

    TEST_ASSERT_EQUAL_INT(0, delete_recording_metadata(recording_id));
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_TARGET_OK,
                          db_storage_target_get(target.uuid, &loaded));
    TEST_ASSERT_EQUAL_UINT64(0, loaded.recording_count);
    TEST_ASSERT_EQUAL_UINT64(0, loaded.recording_bytes);
}

void test_unavailable_target_can_be_staged_disabled_but_not_enabled(void) {
    storage_target_t target = valid_target(
        "Future NAS", "/tmp/lightnvr-target-does-not-exist");
    target.enabled = false;
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_TARGET_OK,
                          db_storage_target_create(&target));
    TEST_ASSERT_EQUAL_STRING("", target.last_error);
    target.enabled = true;
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_TARGET_UNAVAILABLE,
                          db_storage_target_update(&target,
                                                   target.revision));
    TEST_ASSERT_NOT_NULL(strstr(target.last_error, "unavailable"));
}

void test_validation_rejects_root_and_traversal_paths(void) {
    char error[STORAGE_TARGET_ERROR_MAX];
    storage_target_t target = valid_target("Unsafe", "/");
    TEST_ASSERT_EQUAL_INT(
        DB_STORAGE_TARGET_INVALID,
        db_storage_target_validate(&target, error, sizeof(error)));
    target = valid_target("Unsafe", "/srv/../etc");
    TEST_ASSERT_EQUAL_INT(
        DB_STORAGE_TARGET_INVALID,
        db_storage_target_validate(&target, error, sizeof(error)));
    target = valid_target("Unsafe", "/srv//recordings");
    TEST_ASSERT_EQUAL_INT(
        DB_STORAGE_TARGET_INVALID,
        db_storage_target_validate(&target, error, sizeof(error)));
    target = valid_target("Too large", second_root);
    target.reserve_bytes = UINT64_MAX;
    TEST_ASSERT_EQUAL_INT(
        DB_STORAGE_TARGET_INVALID,
        db_storage_target_validate(&target, error, sizeof(error)));
}

void test_validation_canonicalizes_root_so_aliases_cannot_diverge(void) {
    char error[STORAGE_TARGET_ERROR_MAX];
    char link_path[MAX_PATH_LENGTH];
    snprintf(link_path, sizeof(link_path), "%s/alias", second_root);
    unlink(link_path);
    TEST_ASSERT_EQUAL_INT(0, symlink(moved_root, link_path));

    // A target configured through a symlink must be stored as the directory it
    // really points at, so it collides with the real path on the unique index
    // instead of silently becoming a second target over the same bytes.
    storage_target_t target = valid_target("Aliased", link_path);
    TEST_ASSERT_EQUAL_INT(
        DB_STORAGE_TARGET_OK,
        db_storage_target_validate(&target, error, sizeof(error)));
    TEST_ASSERT_EQUAL_STRING(moved_root, target.root_path);

    // A root that does not resolve yet (an unmounted volume) stays literal
    // rather than being rejected outright.
    storage_target_t pending = valid_target("Pending", "/srv/not-mounted-yet");
    TEST_ASSERT_EQUAL_INT(
        DB_STORAGE_TARGET_OK,
        db_storage_target_validate(&pending, error, sizeof(error)));
    TEST_ASSERT_EQUAL_STRING("/srv/not-mounted-yet", pending.root_path);

    unlink(link_path);
}

int main(void) {
    TEST_ASSERT_NOT_NULL(mkdtemp(default_root));
    TEST_ASSERT_NOT_NULL(mkdtemp(second_root));
    TEST_ASSERT_NOT_NULL(mkdtemp(moved_root));
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_bootstrap_attaches_absolute_paths_without_moving_files);
    RUN_TEST(test_resolver_follows_target_root_and_rejects_traversal);
    RUN_TEST(test_mount_detection_uses_most_specific_non_root_mount);
    RUN_TEST(test_explicit_recording_identity_must_resolve_to_file_path);
    RUN_TEST(test_configured_root_change_rotates_default_without_orphaning_history);
    RUN_TEST(test_create_probe_revision_and_duplicate_device_detection);
    RUN_TEST(test_nonempty_target_cannot_be_repointed_or_deleted);
    RUN_TEST(test_unavailable_target_can_be_staged_disabled_but_not_enabled);
    RUN_TEST(test_validation_rejects_root_and_traversal_paths);
    RUN_TEST(test_validation_canonicalizes_root_so_aliases_cannot_diverge);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    rmdir(default_root);
    rmdir(second_root);
    rmdir(moved_root);
    return result;
}
