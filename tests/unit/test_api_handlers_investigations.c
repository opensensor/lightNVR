/**
 * @file test_api_handlers_investigations.c
 * @brief Capture-time identity and multi-camera timeline API tests.
 */

#define _POSIX_C_SOURCE 200809L

#include <stdio.h>
#include <string.h>
#include <unistd.h>

#include <cjson/cJSON.h>
#include <sqlite3.h>

#include "unity.h"
#include "core/config.h"
#include "database/db_core.h"
#include "database/db_detections.h"
#include "database/db_recordings.h"
#include "database/db_streams.h"
#include "utils/strings.h"
#include "video/detection_result.h"
#include "web/api_handlers_investigations.h"
#include "web/request_response.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_investigations_test.db"

static stream_config_t create_camera(const char *name) {
    stream_config_t stream;
    memset(&stream, 0, sizeof(stream));
    safe_strcpy(stream.name, name, sizeof(stream.name), 0);
    safe_strcpy(stream.url, "rtsp://camera.example/live",
                sizeof(stream.url), 0);
    safe_strcpy(stream.codec, "h264", sizeof(stream.codec), 0);
    stream.enabled = true;
    stream.streaming_enabled = true;
    stream.record = true;
    stream.width = 1920;
    stream.height = 1080;
    stream.fps = 25;
    stream.segment_duration = 60;
    TEST_ASSERT_NOT_EQUAL(0, add_stream_config(&stream));
    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_name(name, &stream));
    return stream;
}

static uint64_t create_recording(const char *camera_uuid,
                                 const char *stream_name,
                                 time_t start_time) {
    recording_metadata_t recording;
    memset(&recording, 0, sizeof(recording));
    safe_strcpy(recording.stream_name, stream_name,
                sizeof(recording.stream_name), 0);
    if (camera_uuid) {
        safe_strcpy(recording.camera_uuid, camera_uuid,
                    sizeof(recording.camera_uuid), 0);
    }
    safe_strcpy(recording.file_path, "/tmp/investigation.mp4",
                sizeof(recording.file_path), 0);
    safe_strcpy(recording.codec, "h264", sizeof(recording.codec), 0);
    safe_strcpy(recording.trigger_type, "scheduled",
                sizeof(recording.trigger_type), 0);
    recording.start_time = start_time;
    recording.end_time = start_time + 60;
    recording.is_complete = true;
    recording.schedule_restricted = 0;
    recording.disk_pressure_eligible = true;
    return add_recording_metadata(&recording);
}

static cJSON *call_timeline(const char *body, int expected_status) {
    http_request_t request;
    http_response_t response;
    http_request_init(&request);
    http_response_init(&response);
    request.method = HTTP_METHOD_POST;
    safe_strcpy(request.method_str, "POST", sizeof(request.method_str), 0);
    safe_strcpy(request.path, "/api/investigations/timeline",
                sizeof(request.path), 0);
    safe_strcpy(request.client_ip, "127.0.0.1",
                sizeof(request.client_ip), 0);
    request.body = (void *)body;
    request.body_len = strlen(body);
    handle_post_investigation_timeline(&request, &response);
    TEST_ASSERT_EQUAL_INT(expected_status, response.status_code);
    cJSON *json = response.body
        ? cJSON_Parse((const char *)response.body) : NULL;
    TEST_ASSERT_NOT_NULL(json);
    http_response_free(&response);
    return json;
}

void setUp(void) {
    sqlite3 *db = get_db_handle();
    g_config.web_auth_enabled = false;
    g_config.demo_mode = false;
    sqlite3_exec(db, "DELETE FROM detections;", NULL, NULL, NULL);
    sqlite3_exec(db, "DELETE FROM recordings;", NULL, NULL, NULL);
    sqlite3_exec(db, "DELETE FROM streams;", NULL, NULL, NULL);
}

void tearDown(void) {
    g_config.web_auth_enabled = false;
    g_config.demo_mode = false;
}

void test_capture_identity_survives_camera_rename_and_drives_timeline(void) {
    time_t now = time(NULL);
    stream_config_t camera = create_camera("North Door");
    char camera_uuid[CAMERA_UUID_STRING_SIZE];
    safe_strcpy(camera_uuid, camera.camera_uuid, sizeof(camera_uuid), 0);

    uint64_t recording_id = create_recording(NULL, camera.name, now - 120);
    TEST_ASSERT_NOT_EQUAL(0, recording_id);

    detection_result_t detection;
    memset(&detection, 0, sizeof(detection));
    detection.count = 1;
    safe_strcpy(detection.detections[0].label, "person",
                sizeof(detection.detections[0].label), 0);
    detection.detections[0].confidence = 0.9f;
    TEST_ASSERT_EQUAL_INT(0, store_detections_in_db(
        camera.name, &detection, now - 100, recording_id));

    safe_strcpy(camera.name, "North Entrance", sizeof(camera.name), 0);
    TEST_ASSERT_EQUAL_INT(0, update_stream_config("North Door", &camera));

    /* A recorder that began before the rename may still have the old display
     * name. Its captured UUID must remain authoritative. */
    uint64_t post_rename_id = create_recording(
        camera_uuid, "North Door", now - 50);
    TEST_ASSERT_NOT_EQUAL(0, post_rename_id);

    detection_result_t unlinked_detection = detection;
    safe_strcpy(unlinked_detection.detections[0].label, "vehicle",
                sizeof(unlinked_detection.detections[0].label), 0);
    TEST_ASSERT_EQUAL_INT(0, store_detections_in_db_for_camera(
        camera_uuid, "North Door", &unlinked_detection, now - 40, 0));

    recording_metadata_t stored;
    memset(&stored, 0, sizeof(stored));
    TEST_ASSERT_EQUAL_INT(
        0, get_recording_metadata_by_id(recording_id, &stored));
    TEST_ASSERT_EQUAL_STRING(camera_uuid, stored.camera_uuid);
    TEST_ASSERT_EQUAL_STRING("North Door", stored.stream_name);

    sqlite3_stmt *statement = NULL;
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_prepare_v2(
        get_db_handle(),
        "SELECT camera_uuid FROM detections WHERE recording_id = ?;",
        -1, &statement, NULL));
    sqlite3_bind_int64(statement, 1, (sqlite3_int64)recording_id);
    TEST_ASSERT_EQUAL_INT(SQLITE_ROW, sqlite3_step(statement));
    TEST_ASSERT_EQUAL_STRING(
        camera_uuid, (const char *)sqlite3_column_text(statement, 0));
    sqlite3_finalize(statement);

    char body[512];
    snprintf(body, sizeof(body),
             "{\"camera_uuids\":[\"%s\"],\"start_time\":%lld,"
             "\"end_time\":%lld}",
             camera_uuid, (long long)(now - 300), (long long)now);
    cJSON *json = call_timeline(body, 200);
    cJSON *tracks = cJSON_GetObjectItemCaseSensitive(json, "tracks");
    TEST_ASSERT_TRUE(cJSON_IsArray(tracks));
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetArraySize(tracks));
    cJSON *track = cJSON_GetArrayItem(tracks, 0);
    TEST_ASSERT_EQUAL_STRING(
        "North Entrance",
        cJSON_GetObjectItemCaseSensitive(track, "name")->valuestring);
    TEST_ASSERT_EQUAL_STRING(
        camera_uuid,
        cJSON_GetObjectItemCaseSensitive(track, "camera_uuid")->valuestring);
    cJSON *segments = cJSON_GetObjectItemCaseSensitive(track, "segments");
    TEST_ASSERT_EQUAL_INT(2, cJSON_GetArraySize(segments));
    TEST_ASSERT_EQUAL_UINT64(
        recording_id,
        (uint64_t)cJSON_GetObjectItemCaseSensitive(
            cJSON_GetArrayItem(segments, 0), "id")->valuedouble);
    TEST_ASSERT_EQUAL_UINT64(
        post_rename_id,
        (uint64_t)cJSON_GetObjectItemCaseSensitive(
            cJSON_GetArrayItem(segments, 1), "id")->valuedouble);
    TEST_ASSERT_TRUE(cJSON_IsTrue(cJSON_GetObjectItemCaseSensitive(
        cJSON_GetArrayItem(segments, 1), "has_detection")));
    cJSON_Delete(json);
}

void test_timeline_rejects_duplicate_camera_ids(void) {
    stream_config_t camera = create_camera("Duplicate Camera");
    char body[512];
    snprintf(body, sizeof(body),
             "{\"camera_uuids\":[\"%s\",\"%s\"],"
             "\"start_time\":100,\"end_time\":200}",
             camera.camera_uuid, camera.camera_uuid);
    cJSON *json = call_timeline(body, 400);
    TEST_ASSERT_NOT_NULL(cJSON_GetObjectItemCaseSensitive(json, "error"));
    cJSON_Delete(json);
}

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_capture_identity_survives_camera_rename_and_drives_timeline);
    RUN_TEST(test_timeline_rejects_duplicate_camera_ids);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}
