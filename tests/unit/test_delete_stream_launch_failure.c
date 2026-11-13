#define _POSIX_C_SOURCE 200809L
#include <errno.h>
#include <pthread.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/stat.h>
#include "unity.h"
#include "core/config.h"
#include "core/logger.h"
#include "database/db_core.h"
#include "database/db_streams.h"
#include "video/stream_manager.h"
#include "video/stream_state.h"
#include "web/api_handlers.h"
#include "web/request_response.h"

static bool inject_failure, capture_success;
static bool lookup_before_delete, fail_database_delete;
static unsigned deletion_lookups;
static unsigned launch_attempts, task_frees;
static void *failed_task;
static void *(*captured_worker)(void *);
static unsigned backend_calls;
static stream_handle_t original_stream;
static stream_config_t original_config, original_persisted;
static http_request_t request;
static http_response_t response;

int __real_pthread_create(pthread_t *, const pthread_attr_t *, void *(*)(void *), void *);
int __wrap_pthread_create(pthread_t *thread, const pthread_attr_t *attr,
                          void *(*callback)(void *), void *argument) {
    if (!inject_failure && !capture_success) return __real_pthread_create(thread, attr, callback, argument);
    launch_attempts++;
    failed_task = argument;
    captured_worker = callback;
    /* Capture a successful handoff without starting a worker in this fixture.
     * These controls test the handler response, not asynchronous completion. */
    if (capture_success) return 0;
    return EAGAIN; /* POSIX failure: callback is never scheduled. */
}
bool __wrap_go2rtc_integration_full_start(void) { backend_calls++; return false; }
bool __wrap_go2rtc_integration_unregister_stream(const char *name) {
    (void)name;
    backend_calls++;
    return true;
}
int __real_delete_stream_config_internal(const char *, bool);
int __wrap_delete_stream_config_internal(const char *name, bool permanent) {
    if (lookup_before_delete) {
        /* Model a reader between runtime removal and database deletion. The
         * lookup hydrates a missing runtime slot from the still-present row. */
        deletion_lookups++;
        TEST_ASSERT_NOT_NULL(get_stream_by_name(name));
    }
    if (fail_database_delete) return -1;
    return __real_delete_stream_config_internal(name, permanent);
}
void __real_free(void *);
void __wrap_free(void *allocation) {
    if (allocation && allocation == failed_task) {
        task_frees++;
        failed_task = NULL;
    }
    __real_free(allocation);
}

void setUp(void) {
    inject_failure = capture_success = false;
    lookup_before_delete = fail_database_delete = false;
    deletion_lookups = 0;
    launch_attempts = task_frees = 0;
    failed_task = NULL;
    captured_worker = NULL;
    backend_calls = 0;
    g_config.web_auth_enabled = false;
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(get_db_handle(), "DELETE FROM streams", NULL, NULL, NULL));
    TEST_ASSERT_EQUAL_INT(0, init_stream_state_manager(4));
    TEST_ASSERT_EQUAL_INT(0, init_stream_manager(4));
    stream_config_t config = {0};
    strcpy(config.name, "launch_failure_camera");
    strcpy(config.url, "rtsp://127.0.0.1:1/unopened");
    strcpy(config.codec, "h264");
    config.enabled = true;
    config.width = 640;
    config.height = 480;
    config.fps = 10;
    TEST_ASSERT_GREATER_THAN(0, add_stream_config(&config));
    memset(&original_persisted, 0, sizeof(original_persisted));
    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_name(config.name, &original_persisted));
    original_stream = add_stream(&config);
    TEST_ASSERT_NOT_NULL(original_stream);
    TEST_ASSERT_EQUAL_INT(0, get_stream_config(original_stream, &original_config));
    http_request_init(&request);
    http_response_init(&response);
    strcpy(request.path, "/api/streams/launch_failure_camera");
    strcpy(request.method_str, "DELETE");
}
void tearDown(void) {
    inject_failure = capture_success = false;
    lookup_before_delete = fail_database_delete = false;
    if (failed_task) free(failed_task);
    http_response_free(&response);
    shutdown_stream_manager();
    shutdown_stream_state_manager();
    g_config.web_auth_enabled = false;
}
static void assert_unchanged(void) {
    TEST_ASSERT_EQUAL_PTR(original_stream, get_stream_by_name(original_config.name));
    stream_config_t runtime = {0}, persisted = {0};
    TEST_ASSERT_EQUAL_INT(0, get_stream_config(original_stream, &runtime));
    TEST_ASSERT_EQUAL_MEMORY(&original_config, &runtime, sizeof(runtime));
    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_name(original_config.name, &persisted));
    TEST_ASSERT_EQUAL_MEMORY(&original_persisted, &persisted, sizeof(persisted));
    TEST_ASSERT_EQUAL_INT(1, get_total_stream_count());
}
static void assert_failed_launch(bool permanent) {
    strcpy(request.query_string, permanent ? "permanent=true" : "");
    inject_failure = true;
    handle_delete_stream(&request, &response);
    inject_failure = false;
    TEST_ASSERT_EQUAL_UINT(1, launch_attempts);
    TEST_ASSERT_EQUAL_UINT(1, task_frees);
    TEST_ASSERT_NULL(failed_task);
    assert_unchanged();
    TEST_ASSERT_EQUAL_INT(500, response.status_code);
    cJSON *body = cJSON_Parse((const char *)response.body);
    TEST_ASSERT_NOT_NULL(body);
    cJSON *error = cJSON_GetObjectItemCaseSensitive(body, "error");
    TEST_ASSERT_TRUE(cJSON_IsString(error));
    TEST_ASSERT_EQUAL_STRING("Failed to start delete task", error->valuestring);
    TEST_ASSERT_NULL(cJSON_GetObjectItemCaseSensitive(body, "success"));
    cJSON_Delete(body);
}
void test_soft_delete_launch_failure(void) { assert_failed_launch(false); }
void test_permanent_delete_launch_failure(void) { assert_failed_launch(true); }
void test_unauthorized_delete_never_launches_worker(void) {
    g_config.web_auth_enabled = true;
    inject_failure = true;
    handle_delete_stream(&request, &response);
    inject_failure = false;
    TEST_ASSERT_EQUAL_INT(401, response.status_code);
    TEST_ASSERT_EQUAL_UINT(0, launch_attempts);
    TEST_ASSERT_EQUAL_UINT(0, task_frees);
    assert_unchanged();
}
static void assert_successful_handoff(bool permanent) {
    strcpy(request.query_string, permanent ? "permanent=true" : "");
    capture_success = true;
    handle_delete_stream(&request, &response);
    capture_success = false;
    TEST_ASSERT_EQUAL_UINT(1, launch_attempts);
    TEST_ASSERT_EQUAL_UINT(0, task_frees);
    TEST_ASSERT_NOT_NULL(failed_task);
    TEST_ASSERT_EQUAL_INT(202, response.status_code);
    cJSON *body = cJSON_Parse((const char *)response.body);
    TEST_ASSERT_NOT_NULL(body);
    TEST_ASSERT_TRUE(cJSON_IsTrue(cJSON_GetObjectItemCaseSensitive(body, "success")));
    TEST_ASSERT_EQUAL_STRING("accepted", cJSON_GetObjectItemCaseSensitive(body, "status")->valuestring);
    TEST_ASSERT_EQUAL_INT(permanent, cJSON_IsTrue(cJSON_GetObjectItemCaseSensitive(body, "permanent")));
    TEST_ASSERT_NULL(cJSON_GetObjectItemCaseSensitive(body, "error"));
    cJSON_Delete(body);
    assert_unchanged();
}
static void test_soft_delete_successful_handoff(void) { assert_successful_handoff(false); }
static void test_permanent_delete_successful_handoff(void) { assert_successful_handoff(true); }
static void test_reader_at_runtime_database_transition_cannot_resurrect_camera(void) {
    assert_successful_handoff(true);
    /* Trigger the simulated reader lookup at the delete transition hook used by the worker path. */
    lookup_before_delete = true;
    captured_worker(failed_task);
    lookup_before_delete = false;
    TEST_ASSERT_EQUAL_UINT(1, deletion_lookups);
    TEST_ASSERT_EQUAL_UINT(1, task_frees);
    TEST_ASSERT_EQUAL_UINT(1, backend_calls);
    stream_config_t persisted = {0};
    TEST_ASSERT_NOT_EQUAL(0, get_stream_config_by_name(original_config.name, &persisted));
    TEST_ASSERT_NULL(get_stream_by_name(original_config.name));
    TEST_ASSERT_EQUAL_INT(0, get_total_stream_count());
}
static void test_database_delete_failure_keeps_runtime_camera(void) {
    assert_successful_handoff(true);
    fail_database_delete = true;
    captured_worker(failed_task);
    fail_database_delete = false;
    TEST_ASSERT_EQUAL_UINT(1, task_frees);
    TEST_ASSERT_EQUAL_UINT(0, backend_calls);
    /* Check before any lookup can reload the missing runtime slot. */
    TEST_ASSERT_EQUAL_INT(1, get_total_stream_count());
    assert_unchanged();
}
static void queued_name_reuse(bool refresh) {
    if (refresh) {
        strcat(request.path, "/refresh");
        strcpy(request.method_str, "POST");
    } else {
        strcpy(request.query_string, "permanent=true");
    }
    capture_success = true;
    if (refresh) handle_post_stream_refresh(&request, &response);
    else handle_delete_stream(&request, &response);
    capture_success = false;
    TEST_ASSERT_EQUAL_INT(202, response.status_code);
    TEST_ASSERT_NOT_NULL(captured_worker);
    /* Replace the authorized camera before the queued real callback executes. */
    char old_uuid[CAMERA_UUID_STRING_SIZE];
    strcpy(old_uuid, original_persisted.camera_uuid);
    TEST_ASSERT_EQUAL_INT(0, remove_stream(original_stream));
    TEST_ASSERT_EQUAL_INT(0, delete_stream_config_internal(original_config.name, true));
    stream_config_t replacement = original_config;
    replacement.camera_uuid[0] = 0;
    replacement.priority = 9;
    TEST_ASSERT_GREATER_THAN(0, add_stream_config(&replacement));
    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_name(replacement.name, &original_persisted));
    TEST_ASSERT_NOT_EQUAL(0, strcmp(old_uuid, original_persisted.camera_uuid));
    original_stream = add_stream(&original_persisted);
    TEST_ASSERT_NOT_NULL(original_stream);
    TEST_ASSERT_EQUAL_INT(0, get_stream_config(original_stream, &original_config));
    void *task = failed_task;
    failed_task = NULL;
    captured_worker(task);
    TEST_ASSERT_EQUAL_UINT(0, backend_calls);
    assert_unchanged();
}
static void test_queued_delete_rejects_reused_camera_name(void) { queued_name_reuse(false); }
static void test_queued_refresh_rejects_reused_camera_name(void) { queued_name_reuse(true); }
int main(void) {
    char directory[] = "/tmp/lightnvr-delete-launch-XXXXXX";
    if (!mkdtemp(directory)) return 1;
    char path[192];
    snprintf(path, sizeof(path), "%s/test.db", directory);
    init_logger();
    load_default_config(&g_config);
    snprintf(g_config.storage_path, sizeof(g_config.storage_path), "%s/recordings", directory);
    if (mkdir(g_config.storage_path, 0700) != 0) return 1;
    if (init_database_ex(path, DB_INIT_NO_CHECK | DB_INIT_NO_BACKUP) != 0) return 1;
    UNITY_BEGIN();
    RUN_TEST(test_reader_at_runtime_database_transition_cannot_resurrect_camera);
    RUN_TEST(test_database_delete_failure_keeps_runtime_camera);
    RUN_TEST(test_queued_delete_rejects_reused_camera_name);
    RUN_TEST(test_queued_refresh_rejects_reused_camera_name);
    RUN_TEST(test_soft_delete_launch_failure);
    RUN_TEST(test_permanent_delete_launch_failure);
    RUN_TEST(test_unauthorized_delete_never_launches_worker);
    RUN_TEST(test_soft_delete_successful_handoff);
    RUN_TEST(test_permanent_delete_successful_handoff);
    int result = UNITY_END();
    shutdown_database();
    unlink(path);
    rmdir(g_config.storage_path);
    rmdir(directory);
    return result;
}
