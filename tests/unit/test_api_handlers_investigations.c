/**
 * @file test_api_handlers_investigations.c
 * @brief Capture-time identity and multi-camera timeline API tests.
 */

#define _POSIX_C_SOURCE 200809L

#include <stdio.h>
#include <string.h>
#include <sys/stat.h>
#include <unistd.h>

#include <cjson/cJSON.h>
#include <sqlite3.h>

#include "unity.h"
#include "core/config.h"
#include "database/db_core.h"
#include "database/db_detections.h"
#include "database/db_locations.h"
#include "database/db_recording_tags.h"
#include "database/db_recordings.h"
#include "database/db_streams.h"
#include "utils/strings.h"
#include "video/detection_result.h"
#include "web/api_handlers_investigations.h"
#include "web/api_handlers_recordings_thumbnail.h"
#include "web/request_response.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_investigations_test.db"
#define TEST_STORAGE_PATH "/tmp/lightnvr_unit_investigations_storage"

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

static cJSON *call_search(const char *body, int expected_status) {
    http_request_t request;
    http_response_t response;
    http_request_init(&request);
    http_response_init(&response);
    request.method = HTTP_METHOD_POST;
    safe_strcpy(request.method_str, "POST", sizeof(request.method_str), 0);
    safe_strcpy(request.path, "/api/investigations/search",
                sizeof(request.path), 0);
    safe_strcpy(request.client_ip, "127.0.0.1",
                sizeof(request.client_ip), 0);
    request.body = (void *)body;
    request.body_len = strlen(body);
    handle_post_investigation_search(&request, &response);
    TEST_ASSERT_EQUAL_INT(expected_status, response.status_code);
    cJSON *json = response.body
        ? cJSON_Parse((const char *)response.body) : NULL;
    TEST_ASSERT_NOT_NULL(json);
    http_response_free(&response);
    return json;
}

static cJSON *call_thumbnail_samples(const char *body, int expected_status) {
    http_request_t request;
    http_response_t response;
    http_request_init(&request);
    http_response_init(&response);
    request.method = HTTP_METHOD_POST;
    safe_strcpy(request.method_str, "POST", sizeof(request.method_str), 0);
    safe_strcpy(request.path, "/api/investigations/thumbnail-samples",
                sizeof(request.path), 0);
    safe_strcpy(request.client_ip, "127.0.0.1",
                sizeof(request.client_ip), 0);
    request.body = (void *)body;
    request.body_len = strlen(body);
    handle_post_investigation_thumbnail_samples(&request, &response);
    TEST_ASSERT_EQUAL_INT(expected_status, response.status_code);
    cJSON *json = response.body
        ? cJSON_Parse((const char *)response.body) : NULL;
    TEST_ASSERT_NOT_NULL(json);
    http_response_free(&response);
    return json;
}

static uint64_t insert_detection(const char *camera_uuid,
                                 const char *stream_name,
                                 time_t timestamp, const char *label,
                                 double confidence, const char *zone,
                                 const char *source, uint64_t recording_id) {
    sqlite3_stmt *statement = NULL;
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_prepare_v2(
        get_db_handle(),
        "INSERT INTO detections "
        "(camera_uuid, stream_name, timestamp, label, confidence, "
        " x, y, width, height, recording_id, track_id, zone_id, source, "
        " event_end_time) "
        "VALUES (?, ?, ?, ?, ?, 0.1, 0.2, 0.3, 0.4, ?, 7, ?, ?, ?);",
        -1, &statement, NULL));
    sqlite3_bind_text(statement, 1, camera_uuid, -1, SQLITE_TRANSIENT);
    sqlite3_bind_text(statement, 2, stream_name, -1, SQLITE_TRANSIENT);
    sqlite3_bind_int64(statement, 3, (sqlite3_int64)timestamp);
    sqlite3_bind_text(statement, 4, label, -1, SQLITE_TRANSIENT);
    sqlite3_bind_double(statement, 5, confidence);
    if (recording_id > 0) {
        sqlite3_bind_int64(statement, 6, (sqlite3_int64)recording_id);
    } else {
        sqlite3_bind_null(statement, 6);
    }
    sqlite3_bind_text(statement, 7, zone ? zone : "", -1, SQLITE_TRANSIENT);
    sqlite3_bind_text(statement, 8, source ? source : "", -1,
                      SQLITE_TRANSIENT);
    sqlite3_bind_int64(statement, 9, (sqlite3_int64)timestamp);
    TEST_ASSERT_EQUAL_INT(SQLITE_DONE, sqlite3_step(statement));
    sqlite3_finalize(statement);
    return (uint64_t)sqlite3_last_insert_rowid(get_db_handle());
}

static void set_detection_box(uint64_t detection_id, bool available,
                              double x, double y, double width,
                              double height) {
    sqlite3_stmt *statement = NULL;
    const char *sql = available
        ? "UPDATE detections SET x=?, y=?, width=?, height=? WHERE id=?;"
        : "UPDATE detections SET x=NULL, y=NULL, width=NULL, height=NULL "
          "WHERE id=?;";
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_prepare_v2(
        get_db_handle(), sql, -1, &statement, NULL));
    int parameter = 1;
    if (available) {
        sqlite3_bind_double(statement, parameter++, x);
        sqlite3_bind_double(statement, parameter++, y);
        sqlite3_bind_double(statement, parameter++, width);
        sqlite3_bind_double(statement, parameter++, height);
    }
    sqlite3_bind_int64(statement, parameter, (sqlite3_int64)detection_id);
    TEST_ASSERT_EQUAL_INT(SQLITE_DONE, sqlite3_step(statement));
    sqlite3_finalize(statement);
}

static const cJSON *find_facet(const cJSON *json, const char *facet_name,
                               const char *value) {
    const cJSON *facets = cJSON_GetObjectItemCaseSensitive(json, "facets");
    const cJSON *array = facets
        ? cJSON_GetObjectItemCaseSensitive(facets, facet_name) : NULL;
    const cJSON *item = NULL;
    cJSON_ArrayForEach(item, array) {
        const cJSON *item_value =
            cJSON_GetObjectItemCaseSensitive(item, "value");
        if (cJSON_IsString(item_value) &&
            strcmp(item_value->valuestring, value) == 0) {
            return item;
        }
    }
    return NULL;
}

void setUp(void) {
    sqlite3 *db = get_db_handle();
    g_config.web_auth_enabled = false;
    g_config.demo_mode = false;
    g_config.generate_thumbnails = true;
    // Match the production default (config.c); CPU-save mode is 1.
    g_config.thumbnails_per_recording = 3;
    safe_strcpy(g_config.storage_path, TEST_STORAGE_PATH,
                sizeof(g_config.storage_path), 0);
    sqlite3_exec(db, "DELETE FROM detections;", NULL, NULL, NULL);
    sqlite3_exec(db, "DELETE FROM recordings;", NULL, NULL, NULL);
    sqlite3_exec(db, "DELETE FROM streams;", NULL, NULL, NULL);
    do {
        sqlite3_exec(db,
                     "DELETE FROM camera_locations WHERE is_system = 0 "
                     "AND NOT EXISTS (SELECT 1 FROM camera_locations child "
                     "WHERE child.parent_uuid = camera_locations.uuid);",
                     NULL, NULL, NULL);
    } while (sqlite3_changes(db) > 0);
}

void tearDown(void) {
    g_config.web_auth_enabled = false;
    g_config.demo_mode = false;
    g_config.thumbnails_per_recording = 3;
}

/**
 * @brief Issue a drill-down thumbnail GET and return the status code
 *
 * There is no libuv connection behind these requests, so a cache hit reports
 * 500 ("Failed to serve thumbnail") while a miss falls through to the missing
 * recording file and reports 404. That difference is what lets the offset
 * quantization be observed without running ffmpeg.
 */
static int call_investigation_thumbnail(uint64_t recording_id,
                                        long long offset_ms) {
    http_request_t request;
    http_response_t response;
    http_request_init(&request);
    http_response_init(&response);
    request.method = HTTP_METHOD_GET;
    safe_strcpy(request.method_str, "GET", sizeof(request.method_str), 0);
    snprintf(request.path, sizeof(request.path),
             "/api/investigations/thumbnail/%llu/%lld",
             (unsigned long long)recording_id, offset_ms);
    safe_strcpy(request.client_ip, "127.0.0.1", sizeof(request.client_ip), 0);
    handle_investigation_thumbnail(&request, &response);
    int status = response.status_code;
    http_response_free(&response);
    return status;
}

static void seed_cached_thumbnail(uint64_t recording_id, long long offset_ms) {
    char directory[MAX_PATH_LENGTH];
    snprintf(directory, sizeof(directory), "%s/thumbnails", TEST_STORAGE_PATH);
    mkdir(TEST_STORAGE_PATH, 0755);
    mkdir(directory, 0755);
    snprintf(directory, sizeof(directory), "%s/thumbnails/investigation",
             TEST_STORAGE_PATH);
    mkdir(directory, 0755);
    snprintf(directory, sizeof(directory),
             "%s/thumbnails/investigation/%llu", TEST_STORAGE_PATH,
             (unsigned long long)recording_id);
    mkdir(directory, 0755);

    char path[MAX_PATH_LENGTH];
    snprintf(path, sizeof(path), "%s/%lld.jpg", directory, offset_ms);
    FILE *file = fopen(path, "wb");
    TEST_ASSERT_NOT_NULL(file);
    fwrite("jpeg", 1, 4, file);
    fclose(file);
}

void test_investigation_thumbnail_quantizes_offset_to_cache_grid(void) {
    const time_t range_start = 1700008000;
    stream_config_t camera = create_camera("Quantize Camera");
    uint64_t recording_id = create_recording(
        camera.camera_uuid, camera.name, range_start);
    seed_cached_thumbnail(recording_id, 20000);

    // Every offset inside the same second resolves to the seeded 20000.jpg,
    // so a client walking milliseconds cannot mint new cache entries.
    TEST_ASSERT_EQUAL_INT(500, call_investigation_thumbnail(recording_id, 20000));
    TEST_ASSERT_EQUAL_INT(500, call_investigation_thumbnail(recording_id, 20001));
    TEST_ASSERT_EQUAL_INT(500, call_investigation_thumbnail(recording_id, 20999));
    // The next second is a genuinely distinct frame and still misses.
    TEST_ASSERT_EQUAL_INT(404, call_investigation_thumbnail(recording_id, 21000));
}

void test_investigation_thumbnails_disabled_in_cpu_save_mode(void) {
    const time_t range_start = 1700009000;
    stream_config_t camera = create_camera("CPU Save Camera");
    uint64_t recording_id = create_recording(
        camera.camera_uuid, camera.name, range_start);
    seed_cached_thumbnail(recording_id, 20000);
    g_config.thumbnails_per_recording = 1;

    TEST_ASSERT_EQUAL_INT(403, call_investigation_thumbnail(recording_id, 20000));

    char body[512];
    snprintf(body, sizeof(body),
             "{\"camera_uuids\":[\"%s\"],\"start_time\":%lld,"
             "\"end_time\":%lld,\"sample_count\":3}",
             camera.camera_uuid, (long long)range_start,
             (long long)(range_start + 60));
    cJSON *json = call_thumbnail_samples(body, 200);
    const cJSON *samples = cJSON_GetObjectItemCaseSensitive(json, "samples");
    const cJSON *thumbnail = cJSON_GetObjectItemCaseSensitive(
        cJSON_GetArrayItem(samples, 0), "thumbnail");
    // The sample list must not advertise URLs the endpoint would refuse.
    TEST_ASSERT_EQUAL_STRING(
        "disabled",
        cJSON_GetObjectItemCaseSensitive(thumbnail, "status")->valuestring);
    TEST_ASSERT_NULL(cJSON_GetObjectItemCaseSensitive(thumbnail, "url"));
    cJSON_Delete(json);
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

void test_thumbnail_samples_map_even_times_to_recordings_and_gaps(void) {
    const time_t range_start = 1700005000;
    stream_config_t camera = create_camera("Sample Camera");
    uint64_t first_id = create_recording(
        camera.camera_uuid, camera.name, range_start);
    uint64_t second_id = create_recording(
        camera.camera_uuid, camera.name, range_start + 100);
    TEST_ASSERT_NOT_EQUAL(0, first_id);
    TEST_ASSERT_NOT_EQUAL(0, second_id);

    char body[512];
    snprintf(body, sizeof(body),
             "{\"camera_uuids\":[\"%s\"],\"start_time\":%lld,"
             "\"end_time\":%lld,\"sample_count\":5}",
             camera.camera_uuid, (long long)range_start,
             (long long)(range_start + 120));
    cJSON *json = call_thumbnail_samples(body, 200);
    const cJSON *samples =
        cJSON_GetObjectItemCaseSensitive(json, "samples");
    TEST_ASSERT_EQUAL_INT(5, cJSON_GetArraySize(samples));
    TEST_ASSERT_EQUAL_INT64(
        range_start + 30,
        (int64_t)cJSON_GetObjectItemCaseSensitive(
            cJSON_GetArrayItem(samples, 1), "timestamp")->valuedouble);
    TEST_ASSERT_EQUAL_STRING(
        "gap", cJSON_GetObjectItemCaseSensitive(
                   cJSON_GetArrayItem(samples, 3),
                   "media_status")->valuestring);
    const cJSON *last = cJSON_GetArrayItem(samples, 4);
    TEST_ASSERT_EQUAL_UINT64(
        second_id,
        (uint64_t)cJSON_GetObjectItemCaseSensitive(
            last, "recording_id")->valuedouble);
    TEST_ASSERT_EQUAL_INT64(
        20000,
        (int64_t)cJSON_GetObjectItemCaseSensitive(
            last, "offset_ms")->valuedouble);
    const cJSON *thumbnail =
        cJSON_GetObjectItemCaseSensitive(last, "thumbnail");
    TEST_ASSERT_EQUAL_STRING(
        "available",
        cJSON_GetObjectItemCaseSensitive(thumbnail, "status")->valuestring);
    char expected_url[160];
    snprintf(expected_url, sizeof(expected_url),
             "/api/investigations/thumbnail/%llu/20000",
             (unsigned long long)second_id);
    TEST_ASSERT_EQUAL_STRING(
        expected_url,
        cJSON_GetObjectItemCaseSensitive(thumbnail, "url")->valuestring);
    const cJSON *coverage =
        cJSON_GetObjectItemCaseSensitive(json, "coverage");
    TEST_ASSERT_EQUAL_INT(
        4, cJSON_GetObjectItemCaseSensitive(
               coverage, "available_samples")->valueint);
    TEST_ASSERT_EQUAL_INT(
        1, cJSON_GetObjectItemCaseSensitive(
               coverage, "gap_samples")->valueint);
    cJSON_Delete(json);

    snprintf(body, sizeof(body),
             "{\"camera_uuids\":[\"%s\"],\"start_time\":%lld,"
             "\"end_time\":%lld,\"sample_count\":2}",
             camera.camera_uuid, (long long)range_start,
             (long long)(range_start + 120));
    json = call_thumbnail_samples(body, 400);
    TEST_ASSERT_NOT_NULL(cJSON_GetObjectItemCaseSensitive(json, "error"));
    cJSON_Delete(json);
}

void test_investigation_thumbnail_rejects_offset_outside_recording(void) {
    const time_t range_start = 1700007000;
    stream_config_t camera = create_camera("Offset Camera");
    uint64_t recording_id = create_recording(
        camera.camera_uuid, camera.name, range_start);

    http_request_t request;
    http_response_t response;
    http_request_init(&request);
    http_response_init(&response);
    request.method = HTTP_METHOD_GET;
    safe_strcpy(request.method_str, "GET", sizeof(request.method_str), 0);
    snprintf(request.path, sizeof(request.path),
             "/api/investigations/thumbnail/%llu/60001",
             (unsigned long long)recording_id);
    safe_strcpy(request.client_ip, "127.0.0.1",
                sizeof(request.client_ip), 0);
    handle_investigation_thumbnail(&request, &response);
    TEST_ASSERT_EQUAL_INT(400, response.status_code);
    cJSON *json = response.body
        ? cJSON_Parse((const char *)response.body) : NULL;
    TEST_ASSERT_NOT_NULL(json);
    TEST_ASSERT_NOT_NULL(cJSON_GetObjectItemCaseSensitive(json, "error"));
    cJSON_Delete(json);
    http_response_free(&response);
}

void test_search_cursor_filters_facets_and_current_camera_context(void) {
    const time_t range_start = 1700000000;
    const time_t range_end = range_start + 600;
    stream_config_t camera = create_camera("Loading Bay");
    char camera_uuid[CAMERA_UUID_STRING_SIZE];
    safe_strcpy(camera_uuid, camera.camera_uuid, sizeof(camera_uuid), 0);
    uint64_t recording_id = create_recording(
        camera_uuid, camera.name, range_start);
    TEST_ASSERT_NOT_EQUAL(0, recording_id);
    camera_location_t location = {0};
    safe_strcpy(location.name, "Warehouse", sizeof(location.name), 0);
    safe_strcpy(location.type, "building", sizeof(location.type), 0);
    safe_strcpy(location.metadata_json, "{}", sizeof(location.metadata_json), 0);
    TEST_ASSERT_EQUAL_INT(DB_LOCATION_OK, db_location_create(&location));
    TEST_ASSERT_EQUAL_INT(
        DB_LOCATION_OK,
        db_location_assign_camera(camera_uuid, location.uuid));
    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_name("Loading Bay", &camera));
    TEST_ASSERT_EQUAL_INT(0, db_recording_tag_add(recording_id, "reviewed"));
    TEST_ASSERT_EQUAL_INT(0, set_recording_protected(recording_id, true));

    uint64_t older_id = insert_detection(
        camera_uuid, "Loading Bay", range_start + 100, "person", 0.80,
        "zone-a", "", recording_id);
    uint64_t same_time_person_id = insert_detection(
        camera_uuid, "Loading Bay", range_start + 200, "person", 0.91,
        "zone-a", "", recording_id);
    uint64_t same_time_vehicle_id = insert_detection(
        camera_uuid, "Loading Bay", range_start + 200, "vehicle", 0.70,
        "", "external_motion", 0);
    TEST_ASSERT_TRUE(same_time_vehicle_id > same_time_person_id);

    safe_strcpy(camera.name, "West Loading Bay", sizeof(camera.name), 0);
    TEST_ASSERT_EQUAL_INT(0, update_stream_config("Loading Bay", &camera));

    char body[768];
    snprintf(body, sizeof(body),
             "{\"camera_uuids\":[\"%s\"],\"start_time\":%lld,"
             "\"end_time\":%lld,\"limit\":2}",
             camera_uuid, (long long)range_start, (long long)range_end);
    cJSON *first_page = call_search(body, 200);
    const cJSON *results =
        cJSON_GetObjectItemCaseSensitive(first_page, "results");
    TEST_ASSERT_EQUAL_INT(2, cJSON_GetArraySize(results));
    TEST_ASSERT_EQUAL_UINT64(
        same_time_vehicle_id,
        (uint64_t)cJSON_GetObjectItemCaseSensitive(
            cJSON_GetObjectItemCaseSensitive(cJSON_GetArrayItem(results, 0),
                                             "detection"),
            "id")->valuedouble);
    TEST_ASSERT_TRUE(cJSON_IsTrue(cJSON_GetObjectItemCaseSensitive(
        cJSON_GetArrayItem(results, 0), "known_gap")));
    const cJSON *second_result = cJSON_GetArrayItem(results, 1);
    const cJSON *second_detection =
        cJSON_GetObjectItemCaseSensitive(second_result, "detection");
    TEST_ASSERT_EQUAL_UINT64(
        same_time_person_id,
        (uint64_t)cJSON_GetObjectItemCaseSensitive(
            second_detection, "id")->valuedouble);
    TEST_ASSERT_EQUAL_STRING(
        "West Loading Bay",
        cJSON_GetObjectItemCaseSensitive(
            cJSON_GetObjectItemCaseSensitive(second_result, "camera"),
            "name")->valuestring);
    TEST_ASSERT_EQUAL_UINT64(
        recording_id,
        (uint64_t)cJSON_GetObjectItemCaseSensitive(
            second_result, "recording_id")->valuedouble);
    TEST_ASSERT_TRUE(cJSON_IsTrue(cJSON_GetObjectItemCaseSensitive(
        second_result, "media_available")));
    TEST_ASSERT_FALSE(cJSON_IsTrue(cJSON_GetObjectItemCaseSensitive(
        second_result, "known_gap")));
    const cJSON *thumbnail =
        cJSON_GetObjectItemCaseSensitive(second_result, "thumbnail");
    TEST_ASSERT_EQUAL_STRING(
        "available",
        cJSON_GetObjectItemCaseSensitive(thumbnail, "status")->valuestring);
    TEST_ASSERT_NOT_NULL(
        cJSON_GetObjectItemCaseSensitive(thumbnail, "url"));
    TEST_ASSERT_TRUE(cJSON_IsTrue(cJSON_GetObjectItemCaseSensitive(
        cJSON_GetObjectItemCaseSensitive(second_detection, "bounding_box"),
        "normalized")));
    const cJSON *recording =
        cJSON_GetObjectItemCaseSensitive(second_result, "recording");
    TEST_ASSERT_EQUAL_STRING(
        "continuous",
        cJSON_GetObjectItemCaseSensitive(
            recording, "capture_method")->valuestring);
    TEST_ASSERT_TRUE(cJSON_IsTrue(cJSON_GetObjectItemCaseSensitive(
        recording, "protected")));
    TEST_ASSERT_TRUE(cJSON_IsTrue(cJSON_GetObjectItemCaseSensitive(
        cJSON_GetObjectItemCaseSensitive(first_page, "page"), "has_more")));
    TEST_ASSERT_EQUAL_INT(
        3, cJSON_GetObjectItemCaseSensitive(
               cJSON_GetObjectItemCaseSensitive(first_page, "page"),
               "total")->valueint);
    const cJSON *person_facet = find_facet(first_page, "labels", "person");
    TEST_ASSERT_NOT_NULL(person_facet);
    TEST_ASSERT_EQUAL_INT(
        2, cJSON_GetObjectItemCaseSensitive(person_facet, "count")->valueint);
    const cJSON *camera_facet = find_facet(
        first_page, "cameras", camera_uuid);
    TEST_ASSERT_NOT_NULL(camera_facet);
    TEST_ASSERT_EQUAL_STRING(
        "West Loading Bay",
        cJSON_GetObjectItemCaseSensitive(camera_facet, "label")->valuestring);
    const cJSON *histogram =
        cJSON_GetObjectItemCaseSensitive(first_page, "histogram");
    TEST_ASSERT_TRUE(cJSON_GetArraySize(cJSON_GetObjectItemCaseSensitive(
        histogram, "buckets")) > 0);
    const cJSON *next_cursor = cJSON_GetObjectItemCaseSensitive(
        cJSON_GetObjectItemCaseSensitive(first_page, "page"), "next_cursor");
    TEST_ASSERT_TRUE(cJSON_IsString(next_cursor));
    char cursor[96];
    safe_strcpy(cursor, next_cursor->valuestring, sizeof(cursor), 0);
    cJSON_Delete(first_page);

    /* A newly-arrived result must not shift or duplicate the next page. */
    insert_detection(camera_uuid, "Loading Bay", range_start + 300,
                     "person", 0.99, "zone-a", "", recording_id);
    snprintf(body, sizeof(body),
             "{\"camera_uuids\":[\"%s\"],\"start_time\":%lld,"
             "\"end_time\":%lld,\"limit\":2,\"cursor\":\"%s\"}",
             camera_uuid, (long long)range_start, (long long)range_end,
             cursor);
    cJSON *second_page = call_search(body, 200);
    results = cJSON_GetObjectItemCaseSensitive(second_page, "results");
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetArraySize(results));
    TEST_ASSERT_EQUAL_UINT64(
        older_id,
        (uint64_t)cJSON_GetObjectItemCaseSensitive(
            cJSON_GetObjectItemCaseSensitive(cJSON_GetArrayItem(results, 0),
                                             "detection"),
            "id")->valuedouble);
    TEST_ASSERT_FALSE(cJSON_IsTrue(cJSON_GetObjectItemCaseSensitive(
        cJSON_GetObjectItemCaseSensitive(second_page, "page"), "has_more")));
    cJSON_Delete(second_page);

    snprintf(body, sizeof(body),
             "{\"camera_uuids\":[\"%s\"],\"start_time\":%lld,"
             "\"end_time\":%lld,\"filters\":{\"labels\":[\"person\"],"
             "\"min_confidence\":0.9}}",
             camera_uuid, (long long)range_start, (long long)range_end);
    cJSON *filtered = call_search(body, 200);
    results = cJSON_GetObjectItemCaseSensitive(filtered, "results");
    TEST_ASSERT_EQUAL_INT(2, cJSON_GetArraySize(results));
    for (int i = 0; i < cJSON_GetArraySize(results); i++) {
        const cJSON *detection = cJSON_GetObjectItemCaseSensitive(
            cJSON_GetArrayItem(results, i), "detection");
        TEST_ASSERT_EQUAL_STRING(
            "person",
            cJSON_GetObjectItemCaseSensitive(detection, "label")->valuestring);
        TEST_ASSERT_TRUE(cJSON_GetObjectItemCaseSensitive(
            detection, "confidence")->valuedouble >= 0.9);
    }
    cJSON_Delete(filtered);

    snprintf(body, sizeof(body),
             "{\"camera_uuids\":[\"%s\"],\"start_time\":%lld,"
             "\"end_time\":%lld,\"filters\":{"
             "\"event_types\":[\"detection\"],"
             "\"locations\":[\"%s\"],"
             "\"capture_methods\":[\"continuous\"],"
             "\"recording_tags\":[\"reviewed\"],"
             "\"protected\":true}}",
             camera_uuid, (long long)range_start, (long long)range_end,
             location.uuid);
    cJSON *recording_filtered = call_search(body, 200);
    TEST_ASSERT_EQUAL_INT(
        3, cJSON_GetObjectItemCaseSensitive(
               cJSON_GetObjectItemCaseSensitive(recording_filtered, "page"),
               "total")->valueint);
    TEST_ASSERT_NOT_NULL(find_facet(
        recording_filtered, "event_types", "detection"));
    TEST_ASSERT_NOT_NULL(find_facet(
        recording_filtered, "capture_methods", "continuous"));
    TEST_ASSERT_NOT_NULL(find_facet(
        recording_filtered, "recording_tags", "reviewed"));
    TEST_ASSERT_NOT_NULL(find_facet(
        recording_filtered, "protection", "protected"));
    const cJSON *location_facet = find_facet(
        recording_filtered, "locations", location.uuid);
    TEST_ASSERT_NOT_NULL(location_facet);
    TEST_ASSERT_EQUAL_STRING(
        "Warehouse",
        cJSON_GetObjectItemCaseSensitive(
            location_facet, "label")->valuestring);
    cJSON_Delete(recording_filtered);
}

void test_search_rejects_invalid_cursor(void) {
    stream_config_t camera = create_camera("Search Camera");
    char body[512];
    snprintf(body, sizeof(body),
             "{\"camera_uuids\":[\"%s\"],\"start_time\":100,"
             "\"end_time\":200,\"cursor\":\"offset:100\"}",
             camera.camera_uuid);
    cJSON *json = call_search(body, 400);
    TEST_ASSERT_NOT_NULL(cJSON_GetObjectItemCaseSensitive(json, "error"));
    cJSON_Delete(json);
}

void test_search_includes_spanning_motion_and_reports_legacy_gap(void) {
    const time_t range_start = 1700010000;
    stream_config_t camera = create_camera("Perimeter");
    uint64_t recording_id = create_recording(
        camera.camera_uuid, camera.name, range_start);
    TEST_ASSERT_NOT_EQUAL(0, recording_id);
    uint64_t event_id = insert_detection(
        camera.camera_uuid, camera.name, range_start - 20, "motion", 1.0,
        "", "external_motion", 0);
    sqlite3_stmt *statement = NULL;
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_prepare_v2(
        get_db_handle(),
        "UPDATE detections SET event_end_time=? WHERE id=?;", -1,
        &statement, NULL));
    sqlite3_bind_int64(statement, 1, (sqlite3_int64)(range_start + 10));
    sqlite3_bind_int64(statement, 2, (sqlite3_int64)event_id);
    TEST_ASSERT_EQUAL_INT(SQLITE_DONE, sqlite3_step(statement));
    sqlite3_finalize(statement);

    /* An old unresolved row is excluded from results but surfaced as an
     * explicit coverage gap for this currently-named camera. */
    insert_detection(NULL, camera.name, range_start + 5, "person", 0.8,
                     "", "", 0);

    char body[512];
    snprintf(body, sizeof(body),
             "{\"camera_uuids\":[\"%s\"],\"start_time\":%lld,"
             "\"end_time\":%lld}",
             camera.camera_uuid, (long long)range_start,
             (long long)(range_start + 60));
    cJSON *json = call_search(body, 200);
    const cJSON *results = cJSON_GetObjectItemCaseSensitive(json, "results");
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetArraySize(results));
    const cJSON *result = cJSON_GetArrayItem(results, 0);
    TEST_ASSERT_FALSE(cJSON_IsTrue(cJSON_GetObjectItemCaseSensitive(
        result, "known_gap")));
    TEST_ASSERT_EQUAL_UINT64(
        recording_id,
        (uint64_t)cJSON_GetObjectItemCaseSensitive(
            result, "recording_id")->valuedouble);
    TEST_ASSERT_EQUAL_UINT64(
        event_id,
        (uint64_t)cJSON_GetObjectItemCaseSensitive(
            cJSON_GetObjectItemCaseSensitive(result, "detection"),
            "id")->valuedouble);
    TEST_ASSERT_EQUAL_INT64(
        range_start - 20,
        (int64_t)cJSON_GetObjectItemCaseSensitive(
            result, "start_time")->valuedouble);
    TEST_ASSERT_EQUAL_INT64(
        range_start + 10,
        (int64_t)cJSON_GetObjectItemCaseSensitive(
            result, "end_time")->valuedouble);
    const cJSON *coverage =
        cJSON_GetObjectItemCaseSensitive(json, "coverage");
    TEST_ASSERT_FALSE(cJSON_IsTrue(cJSON_GetObjectItemCaseSensitive(
        coverage, "complete")));
    TEST_ASSERT_EQUAL_INT(
        1, cJSON_GetObjectItemCaseSensitive(
               coverage, "unresolved_legacy_rows")->valueint);
    cJSON_Delete(json);
}

void test_region_search_filters_boxes_and_explains_missing_metadata(void) {
    const time_t range_start = 1700020000;
    stream_config_t camera = create_camera("Region Camera");
    uint64_t inside_id = insert_detection(
        camera.camera_uuid, camera.name, range_start + 10, "person", 0.9,
        "", "", 0);
    uint64_t crossing_id = insert_detection(
        camera.camera_uuid, camera.name, range_start + 20, "vehicle", 0.8,
        "", "", 0);
    uint64_t missing_id = insert_detection(
        camera.camera_uuid, camera.name, range_start + 30, "person", 0.7,
        "", "", 0);
    set_detection_box(inside_id, true, 0.1, 0.1, 0.2, 0.2);
    set_detection_box(crossing_id, true, 0.4, 0.4, 0.4, 0.4);
    set_detection_box(missing_id, false, 0.0, 0.0, 0.0, 0.0);

    char body[1024];
    snprintf(body, sizeof(body),
             "{\"camera_uuids\":[\"%s\"],\"start_time\":%lld,"
             "\"end_time\":%lld,\"region\":{\"camera_uuid\":\"%s\","
             "\"x\":0,\"y\":0,\"width\":0.5,\"height\":0.5,"
             "\"match\":\"center\"}}",
             camera.camera_uuid, (long long)range_start,
             (long long)(range_start + 60), camera.camera_uuid);
    cJSON *json = call_search(body, 200);
    const cJSON *results = cJSON_GetObjectItemCaseSensitive(json, "results");
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetArraySize(results));
    TEST_ASSERT_EQUAL_UINT64(
        inside_id,
        (uint64_t)cJSON_GetObjectItemCaseSensitive(
            cJSON_GetObjectItemCaseSensitive(cJSON_GetArrayItem(results, 0),
                                             "detection"),
            "id")->valuedouble);
    const cJSON *coverage =
        cJSON_GetObjectItemCaseSensitive(json, "coverage");
    const cJSON *spatial =
        cJSON_GetObjectItemCaseSensitive(coverage, "spatial_metadata");
    TEST_ASSERT_TRUE(cJSON_IsTrue(cJSON_GetObjectItemCaseSensitive(
        spatial, "requested")));
    TEST_ASSERT_EQUAL_STRING(
        "metadata_search",
        cJSON_GetObjectItemCaseSensitive(spatial, "search_type")->valuestring);
    TEST_ASSERT_EQUAL_INT(
        2, cJSON_GetObjectItemCaseSensitive(spatial,
                                            "rows_with_boxes")->valueint);
    TEST_ASSERT_EQUAL_INT(
        1, cJSON_GetObjectItemCaseSensitive(spatial,
                                            "rows_without_boxes")->valueint);
    TEST_ASSERT_FALSE(cJSON_IsTrue(cJSON_GetObjectItemCaseSensitive(
        coverage, "complete")));
    cJSON_Delete(json);

    snprintf(body, sizeof(body),
             "{\"camera_uuids\":[\"%s\"],\"start_time\":%lld,"
             "\"end_time\":%lld,\"region\":{\"camera_uuid\":\"%s\","
             "\"x\":0,\"y\":0,\"width\":0.5,\"height\":0.5,"
             "\"match\":\"intersects\"}}",
             camera.camera_uuid, (long long)range_start,
             (long long)(range_start + 60), camera.camera_uuid);
    json = call_search(body, 200);
    TEST_ASSERT_EQUAL_INT(
        2, cJSON_GetObjectItemCaseSensitive(
               cJSON_GetObjectItemCaseSensitive(json, "page"),
               "total")->valueint);
    cJSON_Delete(json);

    snprintf(body, sizeof(body),
             "{\"camera_uuids\":[\"%s\"],\"start_time\":%lld,"
             "\"end_time\":%lld,\"region\":{\"camera_uuid\":\"%s\","
             "\"x\":0,\"y\":0,\"width\":0.5,\"height\":0.5,"
             "\"match\":\"minimum_intersection\","
             "\"min_intersection\":0.1}}",
             camera.camera_uuid, (long long)range_start,
             (long long)(range_start + 60), camera.camera_uuid);
    json = call_search(body, 200);
    TEST_ASSERT_EQUAL_INT(
        1, cJSON_GetObjectItemCaseSensitive(
               cJSON_GetObjectItemCaseSensitive(json, "page"),
               "total")->valueint);
    cJSON_Delete(json);

    snprintf(body, sizeof(body),
             "{\"camera_uuids\":[\"%s\"],\"start_time\":%lld,"
             "\"end_time\":%lld,\"region\":{\"camera_uuid\":\"%s\","
             "\"x\":0.9,\"y\":0,\"width\":0.2,\"height\":0.5}}",
             camera.camera_uuid, (long long)range_start,
             (long long)(range_start + 60), camera.camera_uuid);
    json = call_search(body, 400);
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
    RUN_TEST(test_thumbnail_samples_map_even_times_to_recordings_and_gaps);
    RUN_TEST(test_investigation_thumbnail_rejects_offset_outside_recording);
    RUN_TEST(test_investigation_thumbnail_quantizes_offset_to_cache_grid);
    RUN_TEST(test_investigation_thumbnails_disabled_in_cpu_save_mode);
    RUN_TEST(test_search_cursor_filters_facets_and_current_camera_context);
    RUN_TEST(test_search_rejects_invalid_cursor);
    RUN_TEST(test_search_includes_spanning_motion_and_reports_legacy_gap);
    RUN_TEST(test_region_search_filters_boxes_and_explains_missing_metadata);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}
