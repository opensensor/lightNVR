/**
 * @file test_db_storage_policies.c
 * @brief Selector policy CRUD, fallback, and recording placement tests.
 */

#define _POSIX_C_SOURCE 200809L

#include <sqlite3.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include "core/config.h"
#include "database/db_core.h"
#include "database/db_recordings.h"
#include "database/db_storage_policies.h"
#include "database/db_storage_targets.h"
#include "database/db_streams.h"
#include "storage/storage_placement.h"
#include "unity.h"
#include "utils/strings.h"
#include "video/recording_path.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_storage_policies.db"

static char default_root[] = "/tmp/lightnvr-policy-default-XXXXXX";
static char primary_root[] = "/tmp/lightnvr-policy-primary-XXXXXX";
static char default_uuid[LIGHTNVR_UUID_STRING_SIZE];
static storage_target_t primary_target;

static storage_target_t target_value(const char *name, const char *root) {
    storage_target_t target;
    memset(&target, 0, sizeof(target));
    safe_strcpy(target.name, name, sizeof(target.name), 0);
    safe_strcpy(target.target_type, "filesystem",
                sizeof(target.target_type), 0);
    safe_strcpy(target.root_path, root, sizeof(target.root_path), 0);
    safe_strcpy(target.storage_class, "hot",
                sizeof(target.storage_class), 0);
    target.enabled = true;
    target.high_watermark_pct = 99.0;
    target.low_watermark_pct = 95.0;
    return target;
}

static stream_config_t stream_value(const char *name) {
    stream_config_t stream;
    memset(&stream, 0, sizeof(stream));
    safe_strcpy(stream.name, name, sizeof(stream.name), 0);
    safe_strcpy(stream.url, "rtsp://camera/stream", sizeof(stream.url), 0);
    safe_strcpy(stream.codec, "h264", sizeof(stream.codec), 0);
    stream.enabled = true;
    stream.record = true;
    stream.segment_duration = 60;
    return stream;
}

static storage_policy_t policy_value(const char *name,
                                     const char *primary_uuid) {
    storage_policy_t policy;
    memset(&policy, 0, sizeof(policy));
    safe_strcpy(policy.name, name, sizeof(policy.name), 0);
    policy.enabled = true;
    policy.priority = 100;
    safe_strcpy(policy.selector_json,
                "{\"version\":1,\"expression\":{\"op\":\"all\"}}",
                sizeof(policy.selector_json), 0);
    safe_strcpy(policy.primary_target_uuid, primary_uuid,
                sizeof(policy.primary_target_uuid), 0);
    safe_strcpy(policy.fallback_mode, "default",
                sizeof(policy.fallback_mode), 0);
    return policy;
}

void setUp(void) {
    sqlite3 *db = get_db_handle();
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(
        db, "DELETE FROM detections;DELETE FROM recordings;"
            "DELETE FROM storage_policies;DELETE FROM storage_targets;"
            "DELETE FROM streams;", NULL, NULL, NULL));
    TEST_ASSERT_EQUAL_INT(
        0, db_storage_target_bootstrap_default(default_root, default_uuid));
    storage_target_t default_target;
    TEST_ASSERT_EQUAL_INT(
        DB_STORAGE_TARGET_OK,
        db_storage_target_probe(default_uuid, true, &default_target));
    primary_target = target_value("Primary disk", primary_root);
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_TARGET_OK,
                          db_storage_target_create(&primary_target));
    stream_config_t stream = stream_value("lobby-camera");
    TEST_ASSERT_NOT_EQUAL_UINT64(0, add_stream_config(&stream));
    stream = stream_value("service-camera");
    TEST_ASSERT_NOT_EQUAL_UINT64(0, add_stream_config(&stream));
    storage_placement_cache_invalidate();
}

void tearDown(void) {}

void test_policy_routes_new_recording_and_persists_audit_identity(void) {
    storage_policy_t policy = policy_value("Lobby placement",
                                           primary_target.uuid);
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_POLICY_OK,
                          db_storage_policy_create(&policy));
    TEST_ASSERT_EQUAL_INT64(1, policy.revision);

    config_t config;
    memset(&config, 0, sizeof(config));
    safe_strcpy(config.mp4_directory_format, MP4_DIRECTORY_FORMAT_FLAT,
                sizeof(config.mp4_directory_format), 0);
    char path[MAX_PATH_LENGTH];
    storage_placement_t placement;
    TEST_ASSERT_EQUAL_INT(
        0, prepare_placed_mp4_recording_path(
               &config, "lobby-camera", 1770000000, path, sizeof(path),
               &placement));
    TEST_ASSERT_EQUAL_INT(STORAGE_PLACEMENT_READY, placement.status);
    TEST_ASSERT_EQUAL_STRING(primary_target.uuid, placement.target_uuid);
    TEST_ASSERT_EQUAL_STRING(policy.uuid, placement.policy_uuid);
    TEST_ASSERT_EQUAL_INT64(1, placement.policy_version);
    TEST_ASSERT_TRUE(strncmp(path, primary_root, strlen(primary_root)) == 0);
    TEST_ASSERT_TRUE(strncmp(placement.reason, "policy-primary:", 15) == 0);

    recording_metadata_t recording;
    memset(&recording, 0, sizeof(recording));
    safe_strcpy(recording.stream_name, "lobby-camera",
                sizeof(recording.stream_name), 0);
    safe_strcpy(recording.file_path, path, sizeof(recording.file_path), 0);
    safe_strcpy(recording.storage_target_uuid, placement.target_uuid,
                sizeof(recording.storage_target_uuid), 0);
    safe_strcpy(recording.object_key, placement.object_key,
                sizeof(recording.object_key), 0);
    safe_strcpy(recording.placement_reason, placement.reason,
                sizeof(recording.placement_reason), 0);
    recording.storage_policy_version = placement.policy_version;
    recording.start_time = 1770000000;
    uint64_t id = add_recording_metadata(&recording);
    TEST_ASSERT_NOT_EQUAL_UINT64(0, id);
    recording_metadata_t loaded;
    TEST_ASSERT_EQUAL_INT(0, get_recording_metadata_by_id(id, &loaded));
    TEST_ASSERT_EQUAL_STRING(primary_target.uuid, loaded.storage_target_uuid);
    TEST_ASSERT_EQUAL_STRING(placement.object_key, loaded.object_key);
    TEST_ASSERT_EQUAL_STRING(placement.reason, loaded.placement_reason);
    TEST_ASSERT_EQUAL_INT64(1, loaded.storage_policy_version);
}

void test_unavailable_primary_uses_explicit_default_fallback(void) {
    storage_policy_t policy = policy_value("Fallback placement",
                                           primary_target.uuid);
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_POLICY_OK,
                          db_storage_policy_create(&policy));
    primary_target.enabled = false;
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_TARGET_OK,
                          db_storage_target_update(&primary_target,
                                                   primary_target.revision));

    storage_placement_t placement;
    TEST_ASSERT_EQUAL_INT(
        0, storage_placement_select("lobby-camera", &placement));
    TEST_ASSERT_EQUAL_INT(STORAGE_PLACEMENT_READY, placement.status);
    TEST_ASSERT_EQUAL_STRING(default_uuid, placement.target_uuid);
    TEST_ASSERT_TRUE(strncmp(placement.reason, "policy-default:", 15) == 0);
}

void test_unavailable_primary_honors_named_pause_and_fail_fallbacks(void) {
    storage_policy_t policy = policy_value("Explicit fallback",
                                           primary_target.uuid);
    safe_strcpy(policy.fallback_mode, "target",
                sizeof(policy.fallback_mode), 0);
    safe_strcpy(policy.fallback_target_uuid, default_uuid,
                sizeof(policy.fallback_target_uuid), 0);
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_POLICY_OK,
                          db_storage_policy_create(&policy));
    primary_target.enabled = false;
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_TARGET_OK,
                          db_storage_target_update(&primary_target,
                                                   primary_target.revision));

    storage_placement_t placement;
    TEST_ASSERT_EQUAL_INT(
        0, storage_placement_select("lobby-camera", &placement));
    TEST_ASSERT_EQUAL_INT(STORAGE_PLACEMENT_READY, placement.status);
    TEST_ASSERT_EQUAL_STRING(default_uuid, placement.target_uuid);
    TEST_ASSERT_TRUE(strncmp(placement.reason, "policy-fallback:", 16) == 0);

    safe_strcpy(policy.fallback_mode, "pause",
                sizeof(policy.fallback_mode), 0);
    policy.fallback_target_uuid[0] = '\0';
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_POLICY_OK,
                          db_storage_policy_update(&policy,
                                                   policy.revision));
    TEST_ASSERT_EQUAL_INT(
        0, storage_placement_select("lobby-camera", &placement));
    TEST_ASSERT_EQUAL_INT(STORAGE_PLACEMENT_PAUSED, placement.status);
    TEST_ASSERT_EQUAL_STRING("policy-pause", placement.reason);

    safe_strcpy(policy.fallback_mode, "fail",
                sizeof(policy.fallback_mode), 0);
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_POLICY_OK,
                          db_storage_policy_update(&policy,
                                                   policy.revision));
    TEST_ASSERT_EQUAL_INT(
        0, storage_placement_select("lobby-camera", &placement));
    TEST_ASSERT_EQUAL_INT(STORAGE_PLACEMENT_FAILED, placement.status);
    TEST_ASSERT_EQUAL_STRING("policy-fail", placement.reason);
}

void test_camera_selectors_route_two_cameras_to_different_targets(void) {
    stream_config_t lobby;
    stream_config_t service;
    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_name("lobby-camera", &lobby));
    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_name("service-camera", &service));
    storage_policy_t lobby_policy = policy_value("Lobby only",
                                                  primary_target.uuid);
    snprintf(lobby_policy.selector_json, sizeof(lobby_policy.selector_json),
             "{\"version\":1,\"expression\":{\"op\":\"camera_uuid\","
             "\"values\":[\"%s\"]}}", lobby.camera_uuid);
    lobby_policy.priority = 200;
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_POLICY_OK,
                          db_storage_policy_create(&lobby_policy));
    storage_policy_t service_policy = policy_value("Service only",
                                                    default_uuid);
    snprintf(service_policy.selector_json, sizeof(service_policy.selector_json),
             "{\"version\":1,\"expression\":{\"op\":\"camera_uuid\","
             "\"values\":[\"%s\"]}}", service.camera_uuid);
    service_policy.priority = 100;
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_POLICY_OK,
                          db_storage_policy_create(&service_policy));

    storage_placement_t lobby_placement;
    storage_placement_t service_placement;
    TEST_ASSERT_EQUAL_INT(0, storage_placement_select(
                                 "lobby-camera", &lobby_placement));
    TEST_ASSERT_EQUAL_INT(0, storage_placement_select(
                                 "service-camera", &service_placement));
    TEST_ASSERT_EQUAL_STRING(primary_target.uuid,
                             lobby_placement.target_uuid);
    TEST_ASSERT_EQUAL_STRING(default_uuid, service_placement.target_uuid);
    TEST_ASSERT_NOT_EQUAL(0, strcmp(lobby_placement.target_uuid,
                                    service_placement.target_uuid));

    config_t config;
    memset(&config, 0, sizeof(config));
    safe_strcpy(config.storage_path, default_root,
                sizeof(config.storage_path), 0);
    safe_strcpy(config.mp4_directory_format, MP4_DIRECTORY_FORMAT_FLAT,
                sizeof(config.mp4_directory_format), 0);
    char service_path[MAX_PATH_LENGTH];
    TEST_ASSERT_EQUAL_INT(
        0, prepare_placed_mp4_recording_path(
               &config, "service-camera", 1770000000, service_path,
               sizeof(service_path), &service_placement));
    char expected_prefix[MAX_PATH_LENGTH];
    snprintf(expected_prefix, sizeof(expected_prefix),
             "%s/mp4/service-camera/", default_root);
    TEST_ASSERT_TRUE(strncmp(service_path, expected_prefix,
                             strlen(expected_prefix)) == 0);
    TEST_ASSERT_TRUE(strncmp(service_placement.object_key,
                             "mp4/service-camera/", 19) == 0);
}

void test_policy_revision_validation_and_target_reference_safety(void) {
    storage_policy_t policy = policy_value("Safe policy",
                                           primary_target.uuid);
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_POLICY_OK,
                          db_storage_policy_create(&policy));
    safe_strcpy(policy.fallback_mode, "target",
                sizeof(policy.fallback_mode), 0);
    safe_strcpy(policy.fallback_target_uuid, primary_target.uuid,
                sizeof(policy.fallback_target_uuid), 0);
    char error[STORAGE_TARGET_ERROR_MAX] = {0};
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_POLICY_INVALID,
                          db_storage_policy_validate(&policy, error,
                                                     sizeof(error)));

    safe_strcpy(policy.fallback_mode, "pause",
                sizeof(policy.fallback_mode), 0);
    policy.fallback_target_uuid[0] = '\0';
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_POLICY_STALE,
                          db_storage_policy_update(&policy, 99));
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_POLICY_OK,
                          db_storage_policy_update(&policy, policy.revision));
    TEST_ASSERT_EQUAL_INT64(2, policy.revision);
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_TARGET_IN_USE,
                          db_storage_target_delete(primary_target.uuid,
                                                   primary_target.revision));
}

int main(void) {
    TEST_ASSERT_NOT_NULL(mkdtemp(default_root));
    TEST_ASSERT_NOT_NULL(mkdtemp(primary_root));
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: failed to initialize storage policy test\n");
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_policy_routes_new_recording_and_persists_audit_identity);
    RUN_TEST(test_unavailable_primary_uses_explicit_default_fallback);
    RUN_TEST(test_unavailable_primary_honors_named_pause_and_fail_fallbacks);
    RUN_TEST(test_camera_selectors_route_two_cameras_to_different_targets);
    RUN_TEST(test_policy_revision_validation_and_target_reference_safety);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    rmdir(default_root);
    rmdir(primary_root);
    return result;
}
