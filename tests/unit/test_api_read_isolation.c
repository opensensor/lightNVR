#define _POSIX_C_SOURCE 200809L

#include <errno.h>
#include <pthread.h>
#include <sqlite3.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>

#include "unity.h"
#include "core/config.h"
#include "database/db_core.h"
#include "database/db_authorization.h"
#include "database/db_detections.h"
#include "database/db_fleet_query.h"
#include "database/db_streams.h"
#include "database/db_system_settings.h"
#include "web/api_handlers_setup.h"
#include "web/request_response.h"
#include "core/authorization.h"
#include "cjson/cJSON.h"

static char database_path[128];
static pthread_mutex_t gate = PTHREAD_MUTEX_INITIALIZER;
static pthread_cond_t ready = PTHREAD_COND_INITIALIZER;
static bool writer_ready;
static bool release_writer;
static bool writer_released;

void setUp(void) {
    sqlite3_exec(get_db_handle(), "DELETE FROM streams;"
        "DELETE FROM detections;"
        "DELETE FROM system_settings WHERE key IN ('setup_complete', 'setup_completed_at');",
        NULL, NULL, NULL);
    stream_config_t stream = {0};
    strcpy(stream.name, "cam_a");
    strcpy(stream.url, "rtsp://camera/stream");
    strcpy(stream.codec, "h264");
    stream.enabled = true;
    stream.streaming_enabled = true;
    stream.width = 640;
    stream.height = 480;
    stream.fps = 15;
    stream.segment_duration = 60;
    strcpy(stream.detection_object_filter, "none");
    TEST_ASSERT_NOT_EQUAL(0, add_stream_config(&stream));
    TEST_ASSERT_EQUAL_INT(0, db_mark_setup_complete());
}

void tearDown(void) {}

/* Simulate a busy background writer holding both the application lock and
 * SQLite's connection mutex. Release automatically so the old read path
 * fails an assertion instead of hanging the test process. */
static void *busy_writer(void *unused) {
    (void)unused;
    pthread_mutex_lock(get_db_mutex());
    sqlite3_mutex *connection_mutex = sqlite3_db_mutex(get_db_handle());
    sqlite3_mutex_enter(connection_mutex);
    pthread_mutex_lock(&gate);
    writer_ready = true;
    pthread_cond_broadcast(&ready);
    struct timespec deadline;
    clock_gettime(CLOCK_REALTIME, &deadline);
    deadline.tv_sec += 2;
    while (!release_writer) {
        if (pthread_cond_timedwait(&ready, &gate, &deadline) == ETIMEDOUT) break;
    }
    writer_released = true;
    pthread_mutex_unlock(&gate);
    sqlite3_mutex_leave(connection_mutex);
    pthread_mutex_unlock(get_db_mutex());
    return NULL;
}

static pthread_t start_busy_writer(void) {
    writer_ready = release_writer = writer_released = false;
    pthread_t writer;
    TEST_ASSERT_EQUAL_INT(0, pthread_create(&writer, NULL, busy_writer, NULL));
    pthread_mutex_lock(&gate);
    while (!writer_ready) pthread_cond_wait(&ready, &gate);
    pthread_mutex_unlock(&gate);
    return writer;
}

static bool finish_busy_writer(pthread_t writer) {
    pthread_mutex_lock(&gate);
    bool completed_while_busy = !writer_released;
    release_writer = true;
    pthread_cond_broadcast(&ready);
    pthread_mutex_unlock(&gate);
    pthread_join(writer, NULL);
    return completed_while_busy;
}

void test_inventory_reads_do_not_wait_for_busy_writer_connection(void) {
    pthread_t writer = start_busy_writer();
    stream_config_t stream;
    int streams = get_all_stream_configs_readonly(&stream, 1);
    fleet_camera_t *cameras = NULL;
    int count = 0;
    int inventory = db_fleet_camera_load_readonly(&cameras, &count);
    bool completed_while_busy = finish_busy_writer(writer);
    TEST_ASSERT_TRUE(completed_while_busy);
    TEST_ASSERT_EQUAL_INT(1, streams);
    TEST_ASSERT_EQUAL_STRING("cam_a", stream.name);
    TEST_ASSERT_EQUAL_INT(0, inventory);
    TEST_ASSERT_EQUAL_INT(1, count);
    TEST_ASSERT_EQUAL_STRING(stream.camera_uuid, cameras[0].camera_uuid);
    free(cameras);
}

void test_setup_get_does_not_wait_for_busy_writer_connection(void) {
    http_request_t request;
    http_response_t response;
    http_request_init(&request);
    http_response_init(&response);
    strcpy(request.path, "/api/setup/status");
    pthread_t writer = start_busy_writer();
    handle_get_setup_status(&request, &response);
    bool completed_while_busy = finish_busy_writer(writer);
    TEST_ASSERT_TRUE(completed_while_busy);
    TEST_ASSERT_EQUAL_INT(200, response.status_code);
    cJSON *body = cJSON_Parse((const char *)response.body);
    TEST_ASSERT_NOT_NULL(body);
    TEST_ASSERT_TRUE(cJSON_IsTrue(cJSON_GetObjectItem(body, "complete")));
    TEST_ASSERT_GREATER_THAN(0, cJSON_GetObjectItem(body, "setup_completed_at")->valuedouble);
    cJSON_Delete(body);
    http_response_free(&response);
}

void test_private_readers_only_see_committed_settings_and_inventory(void) {
    sqlite3 *db = get_db_handle();
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(db, "BEGIN IMMEDIATE;"
        "UPDATE streams SET enabled = 0 WHERE name = 'cam_a';"
        "UPDATE system_settings SET value = '0' WHERE key = 'setup_complete';",
        NULL, NULL, NULL));
    stream_config_t stream;
    int streams = get_all_stream_configs_readonly(&stream, 1);
    bool complete = false;
    long long timestamp = 0;
    int setup = db_get_setup_status(&complete, &timestamp);
    fleet_camera_t *cameras = NULL;
    int count = 0;
    int inventory = db_fleet_camera_load_readonly(&cameras, &count);
    sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
    TEST_ASSERT_EQUAL_INT(1, streams);
    TEST_ASSERT_TRUE(stream.enabled);
    TEST_ASSERT_EQUAL_INT(0, setup);
    TEST_ASSERT_TRUE(complete);
    TEST_ASSERT_EQUAL_INT(0, inventory);
    TEST_ASSERT_EQUAL_INT(1, count);
    TEST_ASSERT_TRUE(cameras[0].enabled);
    free(cameras);
}

void test_legacy_authorization_does_not_wait_for_busy_writer(void) {
    int64_t expected_version = 0;
    TEST_ASSERT_EQUAL_INT(0, db_authorization_get_policy_version(&expected_version));
    pthread_t writer = start_busy_writer();
    fleet_camera_t camera;
    int found = db_fleet_camera_find_by_name_readonly("cam_a", &camera);
    user_t admin = { .role = USER_ROLE_ADMIN, .is_active = true };
    authorization_evaluation_t evaluation;
    int authorized = authorization_evaluate(&admin, AUTHZ_LIVE_VIEW, &camera, &evaluation);
    bool completed_while_busy = finish_busy_writer(writer);
    TEST_ASSERT_TRUE(completed_while_busy);
    TEST_ASSERT_EQUAL_INT(0, found);
    TEST_ASSERT_EQUAL_INT(0, authorized);
    TEST_ASSERT_EQUAL_INT(AUTHZ_DECISION_ALLOW, evaluation.decision);
    TEST_ASSERT_EQUAL_INT64(expected_version, evaluation.policy_version);
}

void test_detection_snapshot_does_not_wait_for_busy_writer(void) {
    detection_result_t input = { .count = 1 };
    strcpy(input.detections[0].label, "person");
    input.detections[0].confidence = .9f;
    TEST_ASSERT_EQUAL_INT(0, store_detections_in_db("cam_a", &input, 100, 0));
    pthread_t writer = start_busy_writer();
    detection_result_t result;
    time_t starts[MAX_DETECTIONS] = {0};
    time_t ends[MAX_DETECTIONS] = {0};
    int count = get_detection_results_readonly("cam_a", &result, starts, ends, 0, 90, 110);
    bool completed_while_busy = finish_busy_writer(writer);
    TEST_ASSERT_TRUE(completed_while_busy);
    TEST_ASSERT_EQUAL_INT(1, count);
    TEST_ASSERT_EQUAL_STRING("person", result.detections[0].label);
    TEST_ASSERT_FLOAT_WITHIN(.001f, .9f, result.detections[0].confidence);
    TEST_ASSERT_EQUAL_INT64(100, starts[0]);
    TEST_ASSERT_EQUAL_INT64(100, ends[0]);
}

void test_detection_snapshot_only_sees_committed_rows_and_preserves_event_ranges(void) {
    time_t now = time(NULL);
    detection_result_t motion = { .count = 1 };
    strcpy(motion.detections[0].label, "motion");
    motion.detections[0].confidence = 1.0f;
    TEST_ASSERT_EQUAL_INT(0, store_external_motion_detections("cam_a", &motion, now - 120, 0));
    TEST_ASSERT_EQUAL_INT(1, close_external_motion_detections("cam_a", now - 10));
    detection_result_t person = { .count = 1 };
    strcpy(person.detections[0].label, "person");
    person.detections[0].confidence = .9f;
    TEST_ASSERT_EQUAL_INT(0, store_detections_in_db("cam_a", &person, now - 20, 0));
    sqlite3 *db = get_db_handle();
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(db,
        "BEGIN IMMEDIATE; UPDATE detections SET label = 'uncommitted';", NULL, NULL, NULL));
    detection_result_t result;
    time_t starts[MAX_DETECTIONS] = {0};
    time_t ends[MAX_DETECTIONS] = {0};
    int count = get_detection_results_readonly("cam_a", &result, starts, ends, 0, now - 60, now);
    sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
    TEST_ASSERT_EQUAL_INT(2, count);
    TEST_ASSERT_EQUAL_STRING("person", result.detections[0].label);
    TEST_ASSERT_EQUAL_INT64(now - 20, starts[0]);
    TEST_ASSERT_EQUAL_INT64(now - 20, ends[0]);
    TEST_ASSERT_EQUAL_STRING("motion", result.detections[1].label);
    TEST_ASSERT_EQUAL_INT64(now - 120, starts[1]);
    TEST_ASSERT_EQUAL_INT64(now - 10, ends[1]);
}

void test_missing_setup_settings_mean_incomplete(void) {
    sqlite3_exec(get_db_handle(), "DELETE FROM system_settings WHERE key IN "
        "('setup_complete', 'setup_completed_at');", NULL, NULL, NULL);
    bool complete = true;
    long long timestamp = 123;
    TEST_ASSERT_EQUAL_INT(0, db_get_setup_status(&complete, &timestamp));
    TEST_ASSERT_FALSE(complete);
    TEST_ASSERT_EQUAL_INT64(0, timestamp);
}

void test_setup_database_error_is_not_reported_as_incomplete(void) {
    sqlite3 *db = get_db_handle();
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(db,
        "ALTER TABLE system_settings RENAME TO unavailable_settings;", NULL, NULL, NULL));
    http_request_t request;
    http_response_t response;
    http_request_init(&request);
    http_response_init(&response);
    strcpy(request.path, "/api/setup/status");
    handle_get_setup_status(&request, &response);
    sqlite3_exec(db, "ALTER TABLE unavailable_settings RENAME TO system_settings;",
        NULL, NULL, NULL);
    TEST_ASSERT_EQUAL_INT(503, response.status_code);
    TEST_ASSERT_NULL(strstr((const char *)response.body, "\"complete\""));
    http_response_free(&response);
}

int main(void) {
    load_default_config(&g_config);
    snprintf(database_path, sizeof(database_path), "/tmp/lightnvr_read_isolation_%d.db", (int)getpid());
    if (init_database_ex(database_path, DB_INIT_NO_CHECK | DB_INIT_NO_BACKUP) != 0) return 1;
    UNITY_BEGIN();
    RUN_TEST(test_inventory_reads_do_not_wait_for_busy_writer_connection);
    RUN_TEST(test_setup_get_does_not_wait_for_busy_writer_connection);
    RUN_TEST(test_private_readers_only_see_committed_settings_and_inventory);
    RUN_TEST(test_legacy_authorization_does_not_wait_for_busy_writer);
    RUN_TEST(test_detection_snapshot_does_not_wait_for_busy_writer);
    RUN_TEST(test_detection_snapshot_only_sees_committed_rows_and_preserves_event_ranges);
    RUN_TEST(test_missing_setup_settings_mean_incomplete);
    RUN_TEST(test_setup_database_error_is_not_reported_as_incomplete);
    int result = UNITY_END();
    shutdown_database();
    unlink(database_path);
    return result;
}
