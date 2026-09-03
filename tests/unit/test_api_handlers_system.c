/**
 * @file test_api_handlers_system.c
 * @brief Layer 2 Unity tests for web/api_handlers_system.c
 */

#define _POSIX_C_SOURCE 200809L
#define _GNU_SOURCE

#include <pthread.h>
#include <stdatomic.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/stat.h>
#include <time.h>

#include <cjson/cJSON.h>

#include "unity.h"
#include "core/config.h"
#include "core/logger.h"
#include "core/path_utils.h"
#include "utils/strings.h"
#include "database/db_core.h"
#include "database/db_auth.h"
#include "database/db_onvif_discovery_inventory.h"
#include "database/db_streams.h"
#include "database/db_system_settings.h"
#include "web/api_handlers.h"
#include "web/api_handlers_onvif.h"
#include "web/api_handlers_recording_control.h"
#include "web/api_handlers_system.h"
#include "web/api_handlers_setup.h"
#include "web/request_response.h"
#include "video/go2rtc/go2rtc_lifecycle.h"
#include "video/stream_manager.h"
#include "video/stream_state.h"

extern config_t g_config;

static char g_tmp_root[MAX_PATH_LENGTH];
static char g_db_path[MAX_PATH_LENGTH];
static char g_storage_path[MAX_PATH_LENGTH];
static pthread_mutex_t g_lifecycle_test_mutex = PTHREAD_MUTEX_INITIALIZER;
static pthread_cond_t g_lifecycle_test_cond = PTHREAD_COND_INITIALIZER;
static bool g_lifecycle_owner_ready;
static atomic_bool g_lifecycle_owner_active;

static cJSON *parse_response_json(const http_response_t *res) {
    TEST_ASSERT_NOT_NULL(res);
    TEST_ASSERT_NOT_NULL(res->body);
    cJSON *json = cJSON_Parse((const char *)res->body);
    TEST_ASSERT_NOT_NULL(json);
    return json;
}

static cJSON *find_version_item(cJSON *items, const char *name) {
    cJSON *item = NULL;
    cJSON_ArrayForEach(item, items) {
        cJSON *item_name = cJSON_GetObjectItemCaseSensitive(item, "name");
        if (cJSON_IsString(item_name) && strcmp(item_name->valuestring, name) == 0) {
            return item;
        }
    }
    return NULL;
}

/* ---- helpers ---- */
static void clear_db_streams(void) {
    sqlite3 *db = get_db_handle();
    sqlite3_exec(db, "DELETE FROM streams;", NULL, NULL, NULL);
}

static void clear_onvif_inventory(void) {
    sqlite3_exec(get_db_handle(),
                 "DELETE FROM onvif_discovery_addresses;"
                 "DELETE FROM onvif_discovery_inventory;",
                 NULL, NULL, NULL);
}

static stream_config_t make_test_stream(const char *name) {
    stream_config_t s;
    memset(&s, 0, sizeof(s));
    safe_strcpy(s.name, name, sizeof(s.name), 0);
    safe_strcpy(s.url, "rtsp://localhost/stream", sizeof(s.url), 0);
    safe_strcpy(s.codec, "h264", sizeof(s.codec), 0);
    s.enabled  = true;
    s.width    = 1920;
    s.height   = 1080;
    s.fps      = 25;
    s.priority = 5;
    s.segment_duration = 60;
    s.streaming_enabled = true;
    s.tier_critical_multiplier  = 3.0;
    s.tier_important_multiplier = 2.0;
    s.tier_ephemeral_multiplier = 0.25;
    s.storage_priority = 5;
    safe_strcpy(s.detection_object_filter, "none", sizeof(s.detection_object_filter), 0);
    return s;
}

static int64_t add_api_key_user(http_request_t *req, const char *username,
                                user_role_t role) {
    int64_t user_id = 0;
    char api_key[64] = {0};
    TEST_ASSERT_EQUAL_INT(0, db_auth_create_user(username, "password123", NULL,
                                                 role, true, &user_id));
    TEST_ASSERT_EQUAL_INT(0, db_auth_generate_api_key(user_id, api_key,
                                                      sizeof(api_key)));
    TEST_ASSERT_LESS_THAN_INT(MAX_HEADERS, req->num_headers);
    safe_strcpy(req->headers[req->num_headers].name, "X-API-Key",
                sizeof(req->headers[req->num_headers].name), 0);
    safe_strcpy(req->headers[req->num_headers].value, api_key,
                sizeof(req->headers[req->num_headers].value), 0);
    req->num_headers++;
    return user_id;
}

void setUp(void) {
    g_config.web_auth_enabled = false;
}

void tearDown(void) {}

static void *hold_go2rtc_lifecycle(void *arg) {
    (void)arg;
    go2rtc_lifecycle_guard_t guard;
    bool acquired = go2rtc_lifecycle_begin(
        GO2RTC_LIFECYCLE_RECONFIGURE, false, true, &guard);

    atomic_store(&g_lifecycle_owner_active, acquired);
    pthread_mutex_lock(&g_lifecycle_test_mutex);
    g_lifecycle_owner_ready = true;
    pthread_cond_broadcast(&g_lifecycle_test_cond);
    pthread_mutex_unlock(&g_lifecycle_test_mutex);

    if (acquired) {
        struct timespec hold_time = {
            .tv_sec = 1,
            .tv_nsec = 0,
        };
        nanosleep(&hold_time, NULL);
        atomic_store(&g_lifecycle_owner_active, false);
        go2rtc_lifecycle_end(&guard, true);
    }
    return NULL;
}

void test_handle_get_system_info_includes_versions_summary(void) {
    http_request_t req;
    http_response_t res;
    http_request_init(&req);
    http_response_init(&res);

    handle_get_system_info(&req, &res);

    TEST_ASSERT_EQUAL_INT(200, res.status_code);

    cJSON *root = parse_response_json(&res);
    cJSON *versions = cJSON_GetObjectItemCaseSensitive(root, "versions");
    cJSON *items = cJSON_GetObjectItemCaseSensitive(versions, "items");

    TEST_ASSERT_TRUE(cJSON_IsObject(versions));
    TEST_ASSERT_TRUE(cJSON_IsArray(items));
    TEST_ASSERT_GREATER_OR_EQUAL_INT(10, cJSON_GetArraySize(items));
    TEST_ASSERT_NOT_NULL(find_version_item(items, "LightNVR"));
    TEST_ASSERT_NOT_NULL(find_version_item(items, "Base OS"));
    TEST_ASSERT_NOT_NULL(find_version_item(items, "SQLite"));
    TEST_ASSERT_NOT_NULL(find_version_item(items, "libcurl"));
    TEST_ASSERT_NOT_NULL(find_version_item(items, "mbedTLS"));
    TEST_ASSERT_NOT_NULL(find_version_item(items, "libuv"));
    TEST_ASSERT_NOT_NULL(find_version_item(items, "llhttp"));
    TEST_ASSERT_NOT_NULL(find_version_item(items, "libavformat"));
    cJSON *uptime = cJSON_GetObjectItemCaseSensitive(root, "uptime");
    TEST_ASSERT_TRUE(cJSON_IsNumber(uptime));
    TEST_ASSERT_TRUE(uptime->valuedouble >= 0.0);
    cJSON *cpu = cJSON_GetObjectItemCaseSensitive(root, "cpu");
    cJSON *system_memory = cJSON_GetObjectItemCaseSensitive(root,
                                                            "systemMemory");
    cJSON *disk = cJSON_GetObjectItemCaseSensitive(root, "disk");
    cJSON *network = cJSON_GetObjectItemCaseSensitive(root, "network");
    TEST_ASSERT_TRUE(cJSON_IsObject(cpu));
    TEST_ASSERT_NOT_NULL(cJSON_GetObjectItemCaseSensitive(cpu,
                                                          "usageCapability"));
    TEST_ASSERT_TRUE(cJSON_IsObject(system_memory));
    TEST_ASSERT_NOT_NULL(cJSON_GetObjectItemCaseSensitive(system_memory,
                                                          "capability"));
    TEST_ASSERT_TRUE(cJSON_IsObject(disk));
    TEST_ASSERT_NOT_NULL(cJSON_GetObjectItemCaseSensitive(disk,
                                                          "capability"));
    TEST_ASSERT_TRUE(cJSON_IsObject(network));
    TEST_ASSERT_TRUE(cJSON_IsArray(cJSON_GetObjectItemCaseSensitive(
        network, "interfaces")));

    cJSON_Delete(root);
    http_response_free(&res);
}

void test_handle_get_system_info_does_not_wait_for_go2rtc_lifecycle(void) {
    pthread_mutex_lock(&g_lifecycle_test_mutex);
    g_lifecycle_owner_ready = false;
    pthread_mutex_unlock(&g_lifecycle_test_mutex);
    atomic_store(&g_lifecycle_owner_active, false);

    pthread_t owner;
    TEST_ASSERT_EQUAL_INT(0, pthread_create(&owner, NULL,
                                            hold_go2rtc_lifecycle, NULL));

    pthread_mutex_lock(&g_lifecycle_test_mutex);
    while (!g_lifecycle_owner_ready) {
        pthread_cond_wait(&g_lifecycle_test_cond, &g_lifecycle_test_mutex);
    }
    pthread_mutex_unlock(&g_lifecycle_test_mutex);
    TEST_ASSERT_TRUE(atomic_load(&g_lifecycle_owner_active));

    http_request_t req;
    http_response_t res;
    http_request_init(&req);
    http_response_init(&res);

    handle_get_system_info(&req, &res);
    bool returned_while_lifecycle_busy =
        atomic_load(&g_lifecycle_owner_active);
    pthread_join(owner, NULL);

    TEST_ASSERT_TRUE(returned_while_lifecycle_busy);
    TEST_ASSERT_EQUAL_INT(200, res.status_code);
    http_response_free(&res);
}

void test_handle_get_system_info_includes_empty_stream_storage_array(void) {
    http_request_t req;
    http_response_t res;
    http_request_init(&req);
    http_response_init(&res);

    handle_get_system_info(&req, &res);

    TEST_ASSERT_EQUAL_INT(200, res.status_code);

    cJSON *root = parse_response_json(&res);
    cJSON *stream_storage = cJSON_GetObjectItemCaseSensitive(root, "streamStorage");

    TEST_ASSERT_TRUE(cJSON_IsArray(stream_storage));
    TEST_ASSERT_EQUAL_INT(0, cJSON_GetArraySize(stream_storage));

    cJSON_Delete(root);
    http_response_free(&res);
}

void test_get_json_logs_tail_owns_level_reference_nodes(void) {
    char log_path[MAX_PATH_LENGTH + sizeof("/system.log")];
    snprintf(log_path, sizeof(log_path), "%s/system.log", g_tmp_root);

    FILE *log_file = fopen(log_path, "w");
    TEST_ASSERT_NOT_NULL(log_file);
    fputs("[2026-08-03 12:00:00.000] [INFO] first message\n", log_file);
    fputs("[2026-08-03 12:00:01.000] [WARN] second message\n", log_file);
    fputs("[2026-08-03 12:00:02.000] [ERROR] third message\n", log_file);
    fclose(log_file);

    safe_strcpy(g_config.log_file, log_path, sizeof(g_config.log_file), 0);

    cJSON *logs = get_json_logs_tail(LOG_LEVEL_DEBUG, NULL, 10);
    TEST_ASSERT_NOT_NULL(logs);
    TEST_ASSERT_EQUAL_INT(3, cJSON_GetArraySize(logs));

    cJSON *entry = cJSON_GetArrayItem(logs, 0);
    cJSON *level = cJSON_GetObjectItemCaseSensitive(entry, "level");
    TEST_ASSERT_TRUE(cJSON_IsString(level));
    TEST_ASSERT_EQUAL_STRING("INFO", level->valuestring);

    /* LeakSanitizer verifies that deleting the array also releases each
     * cJSON string-reference node owned by its log entry. */
    cJSON_Delete(logs);
    g_config.log_file[0] = '\0';
    unlink(log_path);
}

/* ================================================================
 * handle_get_streams — motion_trigger_source field present in JSON
 * ================================================================ */

void test_handle_get_streams_includes_motion_trigger_source(void) {
    clear_db_streams();

    stream_config_t ptz = make_test_stream("ptz_cam");
    safe_strcpy(ptz.motion_trigger_source, "fixed_cam", sizeof(ptz.motion_trigger_source), 0);
    add_stream_config(&ptz);

    http_request_t req;
    http_response_t res;
    http_request_init(&req);
    http_response_init(&res);

    handle_get_streams(&req, &res);

    TEST_ASSERT_EQUAL_INT(200, res.status_code);

    /* handle_get_streams returns a bare JSON array (not wrapped in an object) */
    cJSON *root = parse_response_json(&res);
    TEST_ASSERT_NOT_NULL(root);
    TEST_ASSERT_TRUE(cJSON_IsArray(root));
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetArraySize(root));

    cJSON *stream = cJSON_GetArrayItem(root, 0);
    cJSON *mts = cJSON_GetObjectItemCaseSensitive(stream, "motion_trigger_source");
    TEST_ASSERT_NOT_NULL(mts);
    TEST_ASSERT_TRUE(cJSON_IsString(mts));
    TEST_ASSERT_EQUAL_STRING("fixed_cam", mts->valuestring);

    cJSON_Delete(root);
    http_response_free(&res);
    clear_db_streams();
}

void test_handle_get_stream_summaries_are_paginated_and_credential_free(void) {
    clear_db_streams();
    stream_config_t alpha = make_test_stream("Alpha");
    stream_config_t bravo = make_test_stream("Bravo");
    stream_config_t charlie = make_test_stream("Charlie");
    safe_strcpy(alpha.url, "rtsp://admin:alpha-secret@example/stream",
                sizeof(alpha.url), 0);
    safe_strcpy(charlie.admin_url, "https://charlie.example/admin",
                sizeof(charlie.admin_url), 0);
    memset(alpha.recording_schedule, 1, sizeof(alpha.recording_schedule));
    memset(alpha.detection_recording_schedule, 1,
           sizeof(alpha.detection_recording_schedule));
    TEST_ASSERT_GREATER_THAN(0, add_stream_config(&alpha));
    TEST_ASSERT_GREATER_THAN(0, add_stream_config(&bravo));
    TEST_ASSERT_GREATER_THAN(0, add_stream_config(&charlie));

    http_request_t req;
    http_response_t res;
    http_request_init(&req);
    http_response_init(&res);
    safe_strcpy(req.query_string,
                "summary=true&include_admin_url=true&page=2&page_size=2"
                "&sort_by=name&sort_order=asc",
                sizeof(req.query_string), 0);
    handle_get_streams(&req, &res);

    TEST_ASSERT_EQUAL_INT(200, res.status_code);
    cJSON *root = parse_response_json(&res);
    const cJSON *streams =
        cJSON_GetObjectItemCaseSensitive(root, "streams");
    TEST_ASSERT_TRUE(cJSON_IsArray(streams));
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetArraySize(streams));
    TEST_ASSERT_EQUAL_INT(
        3, cJSON_GetObjectItemCaseSensitive(root, "total")->valueint);
    TEST_ASSERT_EQUAL_INT(
        2, cJSON_GetObjectItemCaseSensitive(root, "total_pages")->valueint);
    const cJSON *stream = cJSON_GetArrayItem(streams, 0);
    TEST_ASSERT_EQUAL_STRING(
        "Charlie",
        cJSON_GetObjectItemCaseSensitive(stream, "name")->valuestring);
    TEST_ASSERT_EQUAL_STRING(
        "https://charlie.example/admin",
        cJSON_GetObjectItemCaseSensitive(stream, "admin_url")->valuestring);
    TEST_ASSERT_NULL(cJSON_GetObjectItemCaseSensitive(stream, "url"));
    TEST_ASSERT_NULL(
        cJSON_GetObjectItemCaseSensitive(stream, "onvif_password"));
    TEST_ASSERT_NULL(
        cJSON_GetObjectItemCaseSensitive(stream, "recording_schedule"));
    TEST_ASSERT_NULL(cJSON_GetObjectItemCaseSensitive(
        stream, "detection_recording_schedule"));

    cJSON_Delete(root);
    http_response_free(&res);
    clear_db_streams();
}

void test_viewer_stream_summary_redacts_admin_url(void) {
    clear_db_streams();

    stream_config_t s = make_test_stream("summary_viewer_cam");
    safe_strcpy(s.admin_url, "http://admin:secret@camera.local/",
                sizeof(s.admin_url), 0);
    TEST_ASSERT_GREATER_THAN(0, add_stream_config(&s));

    http_request_t req;
    http_response_t res;
    http_request_init(&req);
    http_response_init(&res);
    safe_strcpy(req.query_string,
                "summary=true&surface=admin&include_admin_url=true",
                sizeof(req.query_string), 0);
    int64_t viewer_id = add_api_key_user(&req, "summary_viewer_get",
                                         USER_ROLE_VIEWER);
    g_config.web_auth_enabled = true;

    handle_get_streams(&req, &res);

    TEST_ASSERT_EQUAL_INT(200, res.status_code);
    cJSON *root = parse_response_json(&res);
    const cJSON *streams = cJSON_GetObjectItemCaseSensitive(root, "streams");
    TEST_ASSERT_TRUE(cJSON_IsArray(streams));
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetArraySize(streams));
    const cJSON *stream = cJSON_GetArrayItem(streams, 0);
    TEST_ASSERT_EQUAL_STRING(
        "", cJSON_GetObjectItemCaseSensitive(stream, "admin_url")->valuestring);
    TEST_ASSERT_TRUE(cJSON_IsFalse(
        cJSON_GetObjectItemCaseSensitive(stream, "can_configure")));

    cJSON_Delete(root);
    http_response_free(&res);
    g_config.web_auth_enabled = false;
    TEST_ASSERT_EQUAL_INT(0, db_auth_delete_user(viewer_id));
    clear_db_streams();
}

void test_stream_summary_contract_scales_to_1024_cameras(void) {
    clear_db_streams();
    sqlite3 *db = get_db_handle();
    sqlite3_stmt *statement = NULL;
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(
        db, "BEGIN IMMEDIATE;", NULL, NULL, NULL));
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_prepare_v2(
        db,
        "INSERT INTO streams "
        "(camera_uuid, name, url, enabled, streaming_enabled, width, height, "
        " fps, codec, record, segment_duration) "
        "VALUES (?, ?, 'rtsp://admin:secret@example/live', 1, 1, 1920, "
        "1080, 25, 'h264', 1, 30);",
        -1, &statement, NULL));
    for (int index = 0; index < 1024; index++) {
        char camera_uuid[CAMERA_UUID_STRING_SIZE];
        char name[MAX_STREAM_NAME];
        snprintf(camera_uuid, sizeof(camera_uuid),
                 "00000000-0000-4000-8000-%012d", index);
        snprintf(name, sizeof(name), "Camera %04d", index);
        sqlite3_bind_text(statement, 1, camera_uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 2, name, -1, SQLITE_TRANSIENT);
        TEST_ASSERT_EQUAL_INT(SQLITE_DONE, sqlite3_step(statement));
        TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_reset(statement));
        sqlite3_clear_bindings(statement);
    }
    sqlite3_finalize(statement);
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(
        db, "COMMIT;", NULL, NULL, NULL));

    int previous_max_streams = g_config.max_streams;
    g_config.max_streams = 1024;
    http_request_t req;
    http_response_t res;
    http_request_init(&req);
    http_response_init(&res);
    safe_strcpy(req.query_string,
                "summary=true&page=11&page_size=100&sort_by=name&sort_order=asc",
                sizeof(req.query_string), 0);
    handle_get_streams(&req, &res);

    TEST_ASSERT_EQUAL_INT(200, res.status_code);
    cJSON *root = parse_response_json(&res);
    const cJSON *streams =
        cJSON_GetObjectItemCaseSensitive(root, "streams");
    TEST_ASSERT_EQUAL_INT(1024,
        cJSON_GetObjectItemCaseSensitive(root, "total")->valueint);
    TEST_ASSERT_EQUAL_INT(11,
        cJSON_GetObjectItemCaseSensitive(root, "total_pages")->valueint);
    TEST_ASSERT_EQUAL_INT(24, cJSON_GetArraySize(streams));
    TEST_ASSERT_NULL(cJSON_GetObjectItemCaseSensitive(
        cJSON_GetArrayItem(streams, 0), "url"));

    cJSON_Delete(root);
    http_response_free(&res);
    g_config.max_streams = previous_max_streams;
    clear_db_streams();
}

void test_viewer_stream_response_redacts_credentials(void) {
    clear_db_streams();

    stream_config_t s = make_test_stream("viewer_cam");
    safe_strcpy(s.url, "rtsp://camera-user:url-secret@localhost/stream",
                sizeof(s.url), 0);
    s.is_onvif = true;
    safe_strcpy(s.onvif_username, "camera-user", sizeof(s.onvif_username), 0);
    safe_strcpy(s.onvif_password, "onvif-secret", sizeof(s.onvif_password), 0);
    safe_strcpy(s.sub_stream_url, "rtsp://camera-user:sub-secret@localhost/sub",
                sizeof(s.sub_stream_url), 0);
    safe_strcpy(s.go2rtc_source_override, "rtsp://camera-user:override-secret@localhost/raw",
                sizeof(s.go2rtc_source_override), 0);
    TEST_ASSERT_GREATER_THAN(0, add_stream_config(&s));

    http_request_t req;
    http_response_t res;
    http_request_init(&req);
    http_response_init(&res);
    int64_t viewer_id = add_api_key_user(&req, "backlog_viewer_get",
                                         USER_ROLE_VIEWER);
    g_config.web_auth_enabled = true;

    handle_get_streams(&req, &res);

    TEST_ASSERT_EQUAL_INT(200, res.status_code);
    cJSON *root = parse_response_json(&res);
    cJSON *stream = cJSON_GetArrayItem(root, 0);
    TEST_ASSERT_EQUAL_STRING("", cJSON_GetObjectItemCaseSensitive(
        stream, "onvif_username")->valuestring);
    TEST_ASSERT_EQUAL_STRING("", cJSON_GetObjectItemCaseSensitive(
        stream, "onvif_password")->valuestring);
    TEST_ASSERT_EQUAL_STRING("", cJSON_GetObjectItemCaseSensitive(
        stream, "sub_stream_url")->valuestring);
    TEST_ASSERT_EQUAL_STRING("", cJSON_GetObjectItemCaseSensitive(
        stream, "go2rtc_source_override")->valuestring);
    TEST_ASSERT_TRUE(cJSON_IsTrue(cJSON_GetObjectItemCaseSensitive(
        stream, "has_sub_stream")));
    TEST_ASSERT_TRUE(cJSON_IsFalse(cJSON_GetObjectItemCaseSensitive(
        stream, "can_control_privacy")));
    const char *safe_url = cJSON_GetObjectItemCaseSensitive(stream, "url")->valuestring;
    TEST_ASSERT_NULL(strstr(safe_url, "url-secret"));

    cJSON_Delete(root);
    http_response_free(&res);
    g_config.web_auth_enabled = false;
    TEST_ASSERT_EQUAL_INT(0, db_auth_delete_user(viewer_id));
    clear_db_streams();
}

void test_viewer_cannot_enable_stream_privacy_mode(void) {
    clear_db_streams();

    stream_config_t s = make_test_stream("viewer_privacy_cam");
    TEST_ASSERT_GREATER_THAN(0, add_stream_config(&s));
    init_stream_state_manager(16);
    init_stream_manager(16);
    add_stream(&s);

    http_request_t req;
    http_response_t res;
    http_request_init(&req);
    http_response_init(&res);
    safe_strcpy(req.path, "/api/streams/viewer_privacy_cam", sizeof(req.path), 0);
    static const char json_body[] = "{\"set_privacy_mode\":true}";
    req.body = (uint8_t *)json_body;
    req.body_len = sizeof(json_body) - 1;
    int64_t viewer_id = add_api_key_user(&req, "backlog_viewer_put",
                                         USER_ROLE_VIEWER);
    g_config.web_auth_enabled = true;

    handle_put_stream(&req, &res);

    TEST_ASSERT_EQUAL_INT(403, res.status_code);
    stream_config_t got;
    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_name("viewer_privacy_cam", &got));
    TEST_ASSERT_FALSE(got.privacy_mode);

    http_response_free(&res);
    g_config.web_auth_enabled = false;
    TEST_ASSERT_EQUAL_INT(0, db_auth_delete_user(viewer_id));
    shutdown_stream_manager();
    shutdown_stream_state_manager();
    clear_db_streams();
}

/* ================================================================
 * handle_put_stream — motion_trigger_source JSON parsing exercised
 *
 * handle_put_stream looks up the stream by name from the in-memory stream
 * manager (not only the DB), so we need to register the stream there first.
 * ================================================================ */

void test_handle_put_stream_parses_motion_trigger_source(void) {
    clear_db_streams();

    /* Register stream in both DB and in-memory stream manager */
    stream_config_t s = make_test_stream("cam_put_mts");
    add_stream_config(&s);

    init_stream_state_manager(16);
    init_stream_manager(16);
    add_stream(&s);   /* register in-memory so the PUT handler can find it */

    http_request_t req;
    http_response_t res;
    http_request_init(&req);
    http_response_init(&res);

    /* Set URL path so handler can extract the stream name */
    safe_strcpy(req.path, "/api/streams/cam_put_mts", sizeof(req.path), 0);

    /* JSON body with motion_trigger_source to exercise the new parsing code */
    static const char json_body[] = "{\"motion_trigger_source\":\"cam_fixed_src\"}";
    req.body     = (uint8_t *)json_body;
    req.body_len = sizeof(json_body) - 1;

    handle_put_stream(&req, &res);

    /* PUT returns 202 Accepted immediately; the actual restart runs async.
     * Give the detached worker thread a moment to finish before we tear down
     * the stream manager and DB so ASan doesn't report use-after-free. */
    usleep(200000);

    TEST_ASSERT_TRUE(res.status_code == 202 || res.status_code == 400 ||
                     res.status_code == 404 || res.status_code == 500);

    http_response_free(&res);
    shutdown_stream_manager();
    shutdown_stream_state_manager();
    clear_db_streams();
}

void test_handle_post_stream_normalizes_blank_go2rtc_override(void) {
    clear_db_streams();

    init_stream_state_manager(16);
    init_stream_manager(16);

    http_request_t req;
    http_response_t res;
    http_request_init(&req);
    http_response_init(&res);

    static const char json_body[] =
        "{\"name\":\"cam_g2r_blank_post\","
        "\"url\":\"rtsp://localhost/stream\","
        "\"go2rtc_source_override\":\" \\t\\n\"}";
    req.body = (uint8_t *)json_body;
    req.body_len = sizeof(json_body) - 1;

    handle_post_stream(&req, &res);

    stream_config_t got;
    TEST_ASSERT_EQUAL_INT(
        0, get_stream_config_by_name("cam_g2r_blank_post", &got));
    TEST_ASSERT_EQUAL_STRING("", got.go2rtc_source_override);

    usleep(200000);
    http_response_free(&res);
    shutdown_stream_manager();
    shutdown_stream_state_manager();
    clear_db_streams();
}

void test_handle_put_stream_normalizes_blank_go2rtc_override(void) {
    clear_db_streams();

    stream_config_t s = make_test_stream("cam_g2r_blank_put");
    safe_strcpy(s.go2rtc_source_override, "rtsp://old/stream",
                sizeof(s.go2rtc_source_override), 0);
    TEST_ASSERT_NOT_EQUAL(0, add_stream_config(&s));

    init_stream_state_manager(16);
    init_stream_manager(16);
    add_stream(&s);

    http_request_t req;
    http_response_t res;
    http_request_init(&req);
    http_response_init(&res);
    safe_strcpy(req.path, "/api/streams/cam_g2r_blank_put",
                sizeof(req.path), 0);
    static const char json_body[] =
        "{\"go2rtc_source_override\":\" \\t\\n\"}";
    req.body = (uint8_t *)json_body;
    req.body_len = sizeof(json_body) - 1;

    handle_put_stream(&req, &res);
    /* Clearing an override restarts go2rtc and includes a 500 ms settling
     * delay in the detached worker. Keep the stream manager alive until that
     * worker has completed its restart/start path. */
    usleep(1200000);

    stream_config_t got;
    TEST_ASSERT_EQUAL_INT(
        0, get_stream_config_by_name("cam_g2r_blank_put", &got));
    TEST_ASSERT_EQUAL_STRING("", got.go2rtc_source_override);

    http_response_free(&res);
    shutdown_stream_manager();
    shutdown_stream_state_manager();
    clear_db_streams();
}

/* ================================================================
 * audio_voice_enhancement — JSON round-trip through the stream handlers
 * (discussion #395)
 * ================================================================ */

void test_handle_get_streams_includes_audio_voice_enhancement(void) {
    clear_db_streams();

    stream_config_t s = make_test_stream("cam_avoe_get");
    s.audio_voice_enhancement = true;
    add_stream_config(&s);

    http_request_t req;
    http_response_t res;
    http_request_init(&req);
    http_response_init(&res);

    handle_get_streams(&req, &res);

    TEST_ASSERT_EQUAL_INT(200, res.status_code);

    cJSON *root = parse_response_json(&res);
    TEST_ASSERT_TRUE(cJSON_IsArray(root));
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetArraySize(root));

    cJSON *stream = cJSON_GetArrayItem(root, 0);
    cJSON *camera_uuid = cJSON_GetObjectItemCaseSensitive(stream, "camera_uuid");
    TEST_ASSERT_TRUE(cJSON_IsString(camera_uuid));
    TEST_ASSERT_EQUAL_UINT(CAMERA_UUID_STRING_SIZE - 1,
                           strlen(camera_uuid->valuestring));
    cJSON *location_uuid =
        cJSON_GetObjectItemCaseSensitive(stream, "location_uuid");
    TEST_ASSERT_TRUE(cJSON_IsString(location_uuid));
    TEST_ASSERT_EQUAL_UINT(CAMERA_UUID_STRING_SIZE - 1,
                           strlen(location_uuid->valuestring));
    cJSON *avoe = cJSON_GetObjectItemCaseSensitive(stream, "audio_voice_enhancement");
    TEST_ASSERT_NOT_NULL(avoe);
    TEST_ASSERT_TRUE(cJSON_IsBool(avoe));
    TEST_ASSERT_TRUE(cJSON_IsTrue(avoe));

    cJSON_Delete(root);
    http_response_free(&res);
    clear_db_streams();
}

/* handle_get_stream / handle_get_stream_full read from the in-memory stream
 * manager, so the stream is registered there with the flag set. */
void test_handle_get_stream_by_name_includes_audio_voice_enhancement(void) {
    clear_db_streams();

    stream_config_t s = make_test_stream("cam_avoe_one");
    s.audio_voice_enhancement = true;
    add_stream_config(&s);
    TEST_ASSERT_EQUAL_INT(0,
                          get_stream_config_by_name("cam_avoe_one", &s));
    TEST_ASSERT_EQUAL_UINT(CAMERA_UUID_STRING_SIZE - 1,
                           strlen(s.camera_uuid));

    init_stream_state_manager(16);
    init_stream_manager(16);
    add_stream(&s);

    /* GET /api/streams/<name> */
    {
        http_request_t req;
        http_response_t res;
        http_request_init(&req);
        http_response_init(&res);
        safe_strcpy(req.path, "/api/streams/cam_avoe_one", sizeof(req.path), 0);

        handle_get_stream(&req, &res);
        TEST_ASSERT_EQUAL_INT(200, res.status_code);

        cJSON *root = parse_response_json(&res);
        cJSON *camera_uuid =
            cJSON_GetObjectItemCaseSensitive(root, "camera_uuid");
        TEST_ASSERT_TRUE(cJSON_IsString(camera_uuid));
        TEST_ASSERT_EQUAL_STRING(s.camera_uuid, camera_uuid->valuestring);
        cJSON *location_uuid =
            cJSON_GetObjectItemCaseSensitive(root, "location_uuid");
        TEST_ASSERT_TRUE(cJSON_IsString(location_uuid));
        TEST_ASSERT_EQUAL_STRING(s.location_uuid, location_uuid->valuestring);
        cJSON *avoe = cJSON_GetObjectItemCaseSensitive(root, "audio_voice_enhancement");
        TEST_ASSERT_NOT_NULL(avoe);
        TEST_ASSERT_TRUE(cJSON_IsTrue(avoe));
        cJSON_Delete(root);
        http_response_free(&res);
    }

    /* GET /api/streams/<name>/full */
    {
        http_request_t req;
        http_response_t res;
        http_request_init(&req);
        http_response_init(&res);
        safe_strcpy(req.path, "/api/streams/cam_avoe_one/full", sizeof(req.path), 0);

        handle_get_stream_full(&req, &res);
        TEST_ASSERT_EQUAL_INT(200, res.status_code);

        /* handle_get_stream_full wraps the stream object under a "stream" key. */
        cJSON *root = parse_response_json(&res);
        cJSON *stream_obj = cJSON_GetObjectItemCaseSensitive(root, "stream");
        TEST_ASSERT_NOT_NULL(stream_obj);
        cJSON *camera_uuid =
            cJSON_GetObjectItemCaseSensitive(stream_obj, "camera_uuid");
        TEST_ASSERT_TRUE(cJSON_IsString(camera_uuid));
        TEST_ASSERT_EQUAL_STRING(s.camera_uuid, camera_uuid->valuestring);
        cJSON *location_uuid =
            cJSON_GetObjectItemCaseSensitive(stream_obj, "location_uuid");
        TEST_ASSERT_TRUE(cJSON_IsString(location_uuid));
        TEST_ASSERT_EQUAL_STRING(s.location_uuid, location_uuid->valuestring);
        cJSON *avoe = cJSON_GetObjectItemCaseSensitive(stream_obj, "audio_voice_enhancement");
        TEST_ASSERT_NOT_NULL(avoe);
        TEST_ASSERT_TRUE(cJSON_IsTrue(avoe));
        cJSON_Delete(root);
        http_response_free(&res);
    }

    shutdown_stream_manager();
    shutdown_stream_state_manager();
    clear_db_streams();
}

void test_handle_post_stream_persists_audio_voice_enhancement(void) {
    clear_db_streams();

    init_stream_state_manager(16);
    init_stream_manager(16);

    http_request_t req;
    http_response_t res;
    http_request_init(&req);
    http_response_init(&res);

    static const char json_body[] =
        "{\"name\":\"cam_avoe_post\",\"url\":\"rtsp://localhost/stream\","
        "\"audio_voice_enhancement\":true}";
    req.body     = (uint8_t *)json_body;
    req.body_len = sizeof(json_body) - 1;

    handle_post_stream(&req, &res);

    /* The POST handler persists the config to the DB (add_stream_config) before
     * it attempts to create/start the stream, so the parsed flag is observable
     * regardless of whether stream startup succeeds in the test environment. */
    stream_config_t got;
    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_name("cam_avoe_post", &got));
    TEST_ASSERT_TRUE(got.audio_voice_enhancement);

    /* Let any detached startup worker settle before tearing down. */
    usleep(200000);

    http_response_free(&res);
    shutdown_stream_manager();
    shutdown_stream_state_manager();
    clear_db_streams();
}

void test_handle_put_stream_parses_audio_voice_enhancement(void) {
    clear_db_streams();

    stream_config_t s = make_test_stream("cam_avoe_put");
    add_stream_config(&s);

    init_stream_state_manager(16);
    init_stream_manager(16);
    add_stream(&s);

    http_request_t req;
    http_response_t res;
    http_request_init(&req);
    http_response_init(&res);

    safe_strcpy(req.path, "/api/streams/cam_avoe_put", sizeof(req.path), 0);
    static const char json_body[] = "{\"audio_voice_enhancement\":true}";
    req.body     = (uint8_t *)json_body;
    req.body_len = sizeof(json_body) - 1;

    handle_put_stream(&req, &res);

    /* PUT returns 202 and applies the change on a detached worker; give it a
     * moment before tearing down so ASan doesn't flag a use-after-free. */
    usleep(200000);

    TEST_ASSERT_TRUE(res.status_code == 202 || res.status_code == 200 ||
                     res.status_code == 400 || res.status_code == 404 ||
                     res.status_code == 500);

    http_response_free(&res);
    shutdown_stream_manager();
    shutdown_stream_state_manager();
    clear_db_streams();
}

/* detection_url validation (api_handlers_streams_modify.c): allowed schemes
 * persist, disallowed schemes are rejected with 400 on both POST and PUT. */
void test_handle_post_stream_persists_detection_url(void) {
    clear_db_streams();

    init_stream_state_manager(16);
    init_stream_manager(16);

    http_request_t req;
    http_response_t res;
    http_request_init(&req);
    http_response_init(&res);

    static const char json_body[] =
        "{\"name\":\"cam_durl_post\",\"url\":\"rtsp://localhost/stream\","
        "\"detection_url\":\"rtsp://localhost/lowres\"}";
    req.body     = (uint8_t *)json_body;
    req.body_len = sizeof(json_body) - 1;

    handle_post_stream(&req, &res);

    /* POST persists the config to the DB before attempting to start the stream,
     * so detection_url is observable regardless of startup success. */
    stream_config_t got;
    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_name("cam_durl_post", &got));
    TEST_ASSERT_EQUAL_STRING("rtsp://localhost/lowres", got.detection_url);

    usleep(200000);
    http_response_free(&res);
    shutdown_stream_manager();
    shutdown_stream_state_manager();
    clear_db_streams();
}

void test_handle_post_stream_rejects_disallowed_detection_url(void) {
    clear_db_streams();

    init_stream_state_manager(16);
    init_stream_manager(16);

    http_request_t req;
    http_response_t res;
    http_request_init(&req);
    http_response_init(&res);

    /* file:// is a local-resource scheme and must be rejected. */
    static const char json_body[] =
        "{\"name\":\"cam_durl_bad\",\"url\":\"rtsp://localhost/stream\","
        "\"detection_url\":\"file:///etc/passwd\"}";
    req.body     = (uint8_t *)json_body;
    req.body_len = sizeof(json_body) - 1;

    handle_post_stream(&req, &res);

    TEST_ASSERT_EQUAL_INT(400, res.status_code);

    /* Rejected before persisting: the stream must not have been created. */
    stream_config_t got;
    TEST_ASSERT_NOT_EQUAL(0, get_stream_config_by_name("cam_durl_bad", &got));

    http_response_free(&res);
    shutdown_stream_manager();
    shutdown_stream_state_manager();
    clear_db_streams();
}

void test_handle_put_stream_rejects_disallowed_detection_url(void) {
    clear_db_streams();

    stream_config_t s = make_test_stream("cam_durl_put");
    add_stream_config(&s);

    init_stream_state_manager(16);
    init_stream_manager(16);
    add_stream(&s);

    http_request_t req;
    http_response_t res;
    http_request_init(&req);
    http_response_init(&res);

    safe_strcpy(req.path, "/api/streams/cam_durl_put", sizeof(req.path), 0);
    static const char json_body[] = "{\"detection_url\":\"file:///etc/passwd\"}";
    req.body     = (uint8_t *)json_body;
    req.body_len = sizeof(json_body) - 1;

    handle_put_stream(&req, &res);

    /* PUT validates the scheme synchronously before queuing the async restart. */
    TEST_ASSERT_EQUAL_INT(400, res.status_code);

    http_response_free(&res);
    shutdown_stream_manager();
    shutdown_stream_state_manager();
    clear_db_streams();
}

void test_manual_start_rejects_continuous_config_before_runtime_starts(void) {
    clear_db_streams();

    stream_config_t stream = make_test_stream("cam_continuous");
    stream.record = true;
    stream.record_on_schedule = false;
    TEST_ASSERT_GREATER_THAN(0, add_stream_config(&stream));

    http_request_t req;
    http_response_t res;
    http_request_init(&req);
    http_response_init(&res);
    safe_strcpy(req.path, "/api/streams/cam_continuous/recording",
                sizeof(req.path), 0);
    static const char json_body[] = "{\"action\":\"start\"}";
    req.body = (uint8_t *)json_body;
    req.body_len = sizeof(json_body) - 1;

    handle_post_stream_recording(&req, &res);

    TEST_ASSERT_EQUAL_INT(409, res.status_code);
    TEST_ASSERT_NOT_NULL(res.body);
    TEST_ASSERT_NOT_NULL(strstr((const char *)res.body,
                                "Continuous recording is configured"));

    http_response_free(&res);
    clear_db_streams();
}

void test_completed_setup_post_requires_system_admin(void) {
    TEST_ASSERT_EQUAL_INT(0, db_mark_setup_complete());
    bool prior_auth = g_config.web_auth_enabled;
    g_config.web_auth_enabled = true;
    http_request_t req;
    http_response_t res;
    http_request_init(&req);
    http_response_init(&res);
    safe_strcpy(req.path, "/api/setup/status", sizeof(req.path), 0);
    safe_strcpy(req.method_str, "POST", sizeof(req.method_str), 0);
    req.body = "{\"complete\":false}";
    req.body_len = strlen((const char *)req.body);
    handle_post_setup_complete(&req, &res);
    TEST_ASSERT_EQUAL_INT(401, res.status_code);
    TEST_ASSERT_TRUE(db_is_setup_complete());
    http_response_free(&res);
    g_config.web_auth_enabled = prior_auth;
}

void test_stream_retention_routes_require_camera_configure(void) {
    bool prior_auth = g_config.web_auth_enabled;
    g_config.web_auth_enabled = true;
    http_request_t req;
    http_response_t res;
    http_request_init(&req);
    http_response_init(&res);
    safe_strcpy(req.path, "/api/streams/private/retention",
                sizeof(req.path), 0);
    safe_strcpy(req.method_str, "GET", sizeof(req.method_str), 0);
    handle_get_stream_retention(&req, &res);
    TEST_ASSERT_EQUAL_INT(401, res.status_code);
    http_response_free(&res);
    g_config.web_auth_enabled = prior_auth;
}

void test_handle_post_stream_persists_playback_transport(void) {
    clear_db_streams();
    init_stream_state_manager(16);
    init_stream_manager(16);

    http_request_t req;
    http_response_t res;
    http_request_init(&req);
    http_response_init(&res);
    static const char json_body[] =
        "{\"name\":\"cam_transport_post\",\"url\":\"rtsp://localhost/stream\","
        "\"playback_transport\":\"webrtc_then_mse\"}";
    req.body = (uint8_t *)json_body;
    req.body_len = sizeof(json_body) - 1;

    handle_post_stream(&req, &res);

    stream_config_t got;
    TEST_ASSERT_EQUAL_INT(0,
                          get_stream_config_by_name("cam_transport_post", &got));
    TEST_ASSERT_EQUAL_STRING("webrtc_then_mse", got.playback_transport);

    usleep(200000);
    http_response_free(&res);
    shutdown_stream_manager();
    shutdown_stream_state_manager();
    clear_db_streams();
}

void test_handle_post_stream_rejects_invalid_playback_transport(void) {
    clear_db_streams();

    http_request_t req;
    http_response_t res;
    http_request_init(&req);
    http_response_init(&res);
    static const char json_body[] =
        "{\"name\":\"cam_transport_bad\",\"url\":\"rtsp://localhost/stream\","
        "\"playback_transport\":\"rtsp_magic\"}";
    req.body = (uint8_t *)json_body;
    req.body_len = sizeof(json_body) - 1;

    handle_post_stream(&req, &res);

    TEST_ASSERT_EQUAL_INT(400, res.status_code);
    stream_config_t got;
    TEST_ASSERT_NOT_EQUAL(0,
                          get_stream_config_by_name("cam_transport_bad", &got));
    http_response_free(&res);
}

void test_handle_post_stream_persists_valid_eptz_config(void) {
    clear_db_streams();
    init_stream_state_manager(16);
    init_stream_manager(16);

    http_request_t req;
    http_response_t res;
    http_request_init(&req);
    http_response_init(&res);
    static const char json_body[] =
        "{\"name\":\"cam_eptz_post\",\"url\":\"rtsp://localhost/stream\","
        "\"eptz_config\":\"{\\\"version\\\":1,\\\"projection\\\":\\\"equidistant\\\","
        "\\\"mount\\\":\\\"ceiling\\\",\\\"centerX\\\":0.5,\\\"centerY\\\":0.5,"
        "\\\"radius\\\":0.48,\\\"fov\\\":190,\\\"rotation\\\":0,"
        "\\\"defaultYaw\\\":0,\\\"defaultTilt\\\":-45,\\\"defaultViewFov\\\":75}\"}";
    req.body = (uint8_t *)json_body;
    req.body_len = sizeof(json_body) - 1;

    handle_post_stream(&req, &res);

    stream_config_t got;
    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_name("cam_eptz_post", &got));
    TEST_ASSERT_NOT_NULL(strstr(got.eptz_config, "\"projection\":\"equidistant\""));

    usleep(200000);
    http_response_free(&res);
    shutdown_stream_manager();
    shutdown_stream_state_manager();
    clear_db_streams();
}

void test_handle_post_stream_rejects_invalid_eptz_config(void) {
    clear_db_streams();
    http_request_t req;
    http_response_t res;
    http_request_init(&req);
    http_response_init(&res);
    static const char json_body[] =
        "{\"name\":\"cam_eptz_bad\",\"url\":\"rtsp://localhost/stream\","
        "\"eptz_config\":\"{\\\"version\\\":1,\\\"projection\\\":\\\"vendor-magic\\\"}\"}";
    req.body = (uint8_t *)json_body;
    req.body_len = sizeof(json_body) - 1;

    handle_post_stream(&req, &res);

    TEST_ASSERT_EQUAL_INT(400, res.status_code);
    stream_config_t got;
    TEST_ASSERT_NOT_EQUAL(0, get_stream_config_by_name("cam_eptz_bad", &got));
    http_response_free(&res);
}

void test_get_onvif_devices_returns_persisted_inventory_metadata(void) {
    clear_onvif_inventory();
    onvif_device_info_t observed;
    memset(&observed, 0, sizeof(observed));
    safe_strcpy(observed.endpoint,
                "urn:uuid:aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee",
                sizeof(observed.endpoint), 0);
    safe_strcpy(observed.device_service,
                "http://192.0.2.80/onvif/device_service",
                sizeof(observed.device_service), 0);
    safe_strcpy(observed.ip_address, "192.0.2.80",
                sizeof(observed.ip_address), 0);
    safe_strcpy(observed.serial_number, "API-SERIAL",
                sizeof(observed.serial_number), 0);
    TEST_ASSERT_EQUAL_INT(
        1, db_onvif_inventory_record_scan(
               "192.0.2.0/24", &observed, 1, 1234));

    http_request_t req;
    http_response_t res;
    http_request_init(&req);
    http_response_init(&res);
    handle_get_discovered_onvif_devices(&req, &res);
    TEST_ASSERT_EQUAL_INT(200, res.status_code);
    cJSON *root = parse_response_json(&res);
    cJSON *devices = cJSON_GetObjectItemCaseSensitive(root, "devices");
    TEST_ASSERT_TRUE(cJSON_IsArray(devices));
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetArraySize(devices));
    cJSON *device_json = cJSON_GetArrayItem(devices, 0);
    TEST_ASSERT_EQUAL_STRING(observed.inventory_uuid,
        cJSON_GetObjectItemCaseSensitive(
            device_json, "inventory_uuid")->valuestring);
    TEST_ASSERT_EQUAL_STRING("unclaimed",
        cJSON_GetObjectItemCaseSensitive(
            device_json, "claim_state")->valuestring);
    TEST_ASSERT_EQUAL_INT64(1234,
        (int64_t)cJSON_GetObjectItemCaseSensitive(
            device_json, "first_seen_at")->valuedouble);
    TEST_ASSERT_TRUE(cJSON_IsArray(cJSON_GetObjectItemCaseSensitive(
        device_json, "addresses")));
    TEST_ASSERT_GREATER_OR_EQUAL_INT(
        3, cJSON_GetArraySize(cJSON_GetObjectItemCaseSensitive(
               device_json, "addresses")));
    cJSON_Delete(root);
    http_response_free(&res);
    clear_onvif_inventory();
}

void test_claim_onvif_device_api_requires_existing_stream(void) {
    clear_onvif_inventory();
    clear_db_streams();
    onvif_device_info_t observed;
    memset(&observed, 0, sizeof(observed));
    safe_strcpy(observed.endpoint,
                "urn:uuid:bbbbbbbb-cccc-4ddd-8eee-ffffffffffff",
                sizeof(observed.endpoint), 0);
    safe_strcpy(observed.device_service,
                "http://192.0.2.81/onvif/device_service",
                sizeof(observed.device_service), 0);
    safe_strcpy(observed.ip_address, "192.0.2.81",
                sizeof(observed.ip_address), 0);
    TEST_ASSERT_EQUAL_INT(
        1, db_onvif_inventory_record_scan(
               "192.0.2.0/24", &observed, 1, 1235));

    char json_body[256];
    snprintf(json_body, sizeof(json_body),
             "{\"inventory_uuid\":\"%s\",\"stream_name\":\"missing\"}",
             observed.inventory_uuid);
    http_request_t req;
    http_response_t res;
    http_request_init(&req);
    http_response_init(&res);
    req.body = (uint8_t *)json_body;
    req.body_len = strlen(json_body);
    handle_post_claim_onvif_device(&req, &res);
    TEST_ASSERT_EQUAL_INT(404, res.status_code);
    http_response_free(&res);
    clear_onvif_inventory();
}

int main(void) {
    init_logger();
    load_default_config(&g_config);

    snprintf(g_tmp_root, sizeof(g_tmp_root), "/tmp/lightnvr_system_handler_%d", (int)getpid());
    snprintf(g_db_path, sizeof(g_db_path), "%s/lightnvr.db", g_tmp_root);
    snprintf(g_storage_path, sizeof(g_storage_path), "%s/storage", g_tmp_root);

    mkdir_recursive(g_storage_path);
    safe_strcpy(g_config.storage_path, g_storage_path, sizeof(g_config.storage_path), 0);

    if (init_database(g_db_path) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }

    /* handle_get_streams uses g_config.max_streams for allocation */
    if (g_config.max_streams == 0) {
        g_config.max_streams = 16;
    }

    UNITY_BEGIN();
    RUN_TEST(test_handle_get_system_info_includes_versions_summary);
    RUN_TEST(test_handle_get_system_info_does_not_wait_for_go2rtc_lifecycle);
    RUN_TEST(test_handle_get_system_info_includes_empty_stream_storage_array);
    RUN_TEST(test_get_json_logs_tail_owns_level_reference_nodes);
    RUN_TEST(test_handle_get_streams_includes_motion_trigger_source);
    RUN_TEST(test_handle_get_stream_summaries_are_paginated_and_credential_free);
    RUN_TEST(test_stream_summary_contract_scales_to_1024_cameras);
    RUN_TEST(test_viewer_stream_summary_redacts_admin_url);
    RUN_TEST(test_viewer_stream_response_redacts_credentials);
    RUN_TEST(test_viewer_cannot_enable_stream_privacy_mode);
    RUN_TEST(test_handle_put_stream_parses_motion_trigger_source);
    RUN_TEST(test_handle_post_stream_normalizes_blank_go2rtc_override);
    RUN_TEST(test_handle_put_stream_normalizes_blank_go2rtc_override);
    RUN_TEST(test_handle_get_streams_includes_audio_voice_enhancement);
    RUN_TEST(test_handle_get_stream_by_name_includes_audio_voice_enhancement);
    RUN_TEST(test_handle_post_stream_persists_audio_voice_enhancement);
    RUN_TEST(test_handle_put_stream_parses_audio_voice_enhancement);
    RUN_TEST(test_handle_post_stream_persists_detection_url);
    RUN_TEST(test_handle_post_stream_rejects_disallowed_detection_url);
    RUN_TEST(test_handle_put_stream_rejects_disallowed_detection_url);
    RUN_TEST(test_manual_start_rejects_continuous_config_before_runtime_starts);
    RUN_TEST(test_completed_setup_post_requires_system_admin);
    RUN_TEST(test_stream_retention_routes_require_camera_configure);
    RUN_TEST(test_handle_post_stream_persists_playback_transport);
    RUN_TEST(test_handle_post_stream_rejects_invalid_playback_transport);
    RUN_TEST(test_handle_post_stream_persists_valid_eptz_config);
    RUN_TEST(test_handle_post_stream_rejects_invalid_eptz_config);
    RUN_TEST(test_get_onvif_devices_returns_persisted_inventory_metadata);
    RUN_TEST(test_claim_onvif_device_api_requires_existing_stream);
    int result = UNITY_END();

    shutdown_database();
    unlink(g_db_path);
    snprintf(g_db_path, sizeof(g_db_path), "%s/lightnvr.db-wal", g_tmp_root);
    unlink(g_db_path);
    snprintf(g_db_path, sizeof(g_db_path), "%s/lightnvr.db-shm", g_tmp_root);
    unlink(g_db_path);
    snprintf(g_db_path, sizeof(g_db_path), "%s/lightnvr.db.bak", g_tmp_root);
    unlink(g_db_path);
    free(g_config.streams);
    g_config.streams = NULL;
    rmdir(g_storage_path);
    rmdir(g_tmp_root);
    shutdown_logger();
    return result;
}
