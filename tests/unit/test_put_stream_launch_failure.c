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
static unsigned launch_attempts, task_frees;
static void *failed_task;
static void *(*captured_worker)(void *);
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
/* go2rtc stubs for the privacy toggle: inactive unless fake_go2rtc is set. */
static bool fake_go2rtc, fake_unregister_ok;
static unsigned unregister_calls, reload_calls, sub_remove_calls, sub_register_calls;
bool __real_go2rtc_stream_is_ready(void);
bool __wrap_go2rtc_stream_is_ready(void) {
    return fake_go2rtc ? true : __real_go2rtc_stream_is_ready();
}
bool __real_go2rtc_integration_unregister_stream(const char *name);
bool __wrap_go2rtc_integration_unregister_stream(const char *name) {
    if (!fake_go2rtc) return __real_go2rtc_integration_unregister_stream(name);
    unregister_calls++;
    return fake_unregister_ok;
}
bool __real_go2rtc_integration_reload_stream(const char *name);
bool __wrap_go2rtc_integration_reload_stream(const char *name) {
    if (!fake_go2rtc) return __real_go2rtc_integration_reload_stream(name);
    reload_calls++;
    return true;
}
bool __real_go2rtc_api_remove_stream(const char *name);
bool __wrap_go2rtc_api_remove_stream(const char *name) {
    if (!fake_go2rtc) return __real_go2rtc_api_remove_stream(name);
    sub_remove_calls++;
    return true;
}
bool __real_go2rtc_stream_register_substream(const char *, const char *, const char *,
                                             const char *, stream_protocol_t);
bool __wrap_go2rtc_stream_register_substream(const char *id, const char *url,
                                             const char *user, const char *pass,
                                             stream_protocol_t protocol) {
    if (!fake_go2rtc) return __real_go2rtc_stream_register_substream(id, url, user, pass, protocol);
    sub_register_calls++;
    return true;
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
    launch_attempts = task_frees = 0;
    failed_task = NULL;
    captured_worker = NULL;
    fake_go2rtc = false;
    fake_unregister_ok = true;
    unregister_calls = reload_calls = sub_remove_calls = sub_register_calls = 0;
    g_config.web_auth_enabled = false;
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(get_db_handle(), "DELETE FROM streams", NULL, NULL, NULL));
    TEST_ASSERT_EQUAL_INT(0, init_stream_state_manager(4));
    TEST_ASSERT_EQUAL_INT(0, init_stream_manager(4));
    stream_config_t config = {0};
    strcpy(config.name, "put_launch_failure_camera");
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
    strcpy(request.path, "/api/streams/put_launch_failure_camera");
    strcpy(request.method_str, "PUT");
}
void tearDown(void) {
    inject_failure = capture_success = false;
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
static void assert_failed_launch(void) {
    static const char body[] = "{\"enabled\":false,\"priority\":8}";
    request.body = (uint8_t *)body;
    request.body_len = sizeof(body) - 1;
    inject_failure = true;
    handle_put_stream(&request, &response);
    inject_failure = false;
    TEST_ASSERT_EQUAL_UINT(1, launch_attempts);
    TEST_ASSERT_EQUAL_UINT(1, task_frees);
    TEST_ASSERT_NULL(failed_task);
    assert_unchanged();
    TEST_ASSERT_EQUAL_INT(500, response.status_code);
    cJSON *json = cJSON_Parse((const char *)response.body);
    TEST_ASSERT_NOT_NULL(json);
    cJSON *error = cJSON_GetObjectItemCaseSensitive(json, "error");
    TEST_ASSERT_TRUE(cJSON_IsString(error));
    TEST_ASSERT_EQUAL_STRING("Failed to start update task", error->valuestring);
    TEST_ASSERT_NULL(cJSON_GetObjectItemCaseSensitive(json, "success"));
    cJSON_Delete(json);
}
void test_update_launch_failure(void) { assert_failed_launch(); }
void test_unauthorized_update_never_launches_worker(void) {
    g_config.web_auth_enabled = true;
    inject_failure = true;
    handle_put_stream(&request, &response);
    inject_failure = false;
    TEST_ASSERT_EQUAL_INT(401, response.status_code);
    TEST_ASSERT_EQUAL_UINT(0, launch_attempts);
    TEST_ASSERT_EQUAL_UINT(0, task_frees);
    assert_unchanged();
}
static void test_update_successful_handoff(void) {
    static const char body[] = "{\"enabled\":false,\"priority\":8}";
    request.body = (uint8_t *)body;
    request.body_len = sizeof(body) - 1;
    capture_success = true;
    handle_put_stream(&request, &response);
    capture_success = false;
    TEST_ASSERT_EQUAL_UINT(1, launch_attempts);
    TEST_ASSERT_EQUAL_UINT(0, task_frees);
    TEST_ASSERT_NOT_NULL(failed_task);
    TEST_ASSERT_EQUAL_INT(202, response.status_code);
    cJSON *json = cJSON_Parse((const char *)response.body);
    TEST_ASSERT_NOT_NULL(json);
    TEST_ASSERT_TRUE(cJSON_IsTrue(cJSON_GetObjectItemCaseSensitive(json, "success")));
    TEST_ASSERT_NULL(cJSON_GetObjectItemCaseSensitive(json, "error"));
    cJSON_Delete(json);
    assert_unchanged();
}
/* ---- enable_disabled / set_privacy_mode request handling ---- */

static void set_body(const char *body) {
    request.body = (uint8_t *)body;
    request.body_len = strlen(body);
}

static int db_int_column(const char *column) {
    char sql[128];
    snprintf(sql, sizeof(sql), "SELECT %s FROM streams WHERE name = ?;", column);
    sqlite3_stmt *stmt = NULL;
    int value = -1;
    if (sqlite3_prepare_v2(get_db_handle(), sql, -1, &stmt, NULL) == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, original_config.name, -1, SQLITE_STATIC);
        if (sqlite3_step(stmt) == SQLITE_ROW) value = sqlite3_column_int(stmt, 0);
        sqlite3_finalize(stmt);
    }
    return value;
}

static void disable_stream_for_test(void) {
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(get_db_handle(),
        "UPDATE streams SET enabled = 0", NULL, NULL, NULL));
    stream_config_t config = original_config;
    config.enabled = false;
    TEST_ASSERT_EQUAL_INT(0, update_stream_config(config.name, &config));
    TEST_ASSERT_EQUAL_INT(0, remove_stream(original_stream));
    original_stream = add_stream(&config);
    TEST_ASSERT_NOT_NULL(original_stream);
    stream_config_t runtime = {0};
    TEST_ASSERT_EQUAL_INT(0, get_stream_config(original_stream, &runtime));
    TEST_ASSERT_FALSE(runtime.enabled);
}

void test_enable_disabled_conflicting_with_enabled_false_rejected(void) {
    set_body("{\"enable_disabled\":true,\"enabled\":false}");
    capture_success = true;
    handle_put_stream(&request, &response);
    capture_success = false;
    TEST_ASSERT_EQUAL_INT(400, response.status_code);
    TEST_ASSERT_EQUAL_UINT(0, launch_attempts);
}

void test_enable_disabled_persists_through_worker(void) {
    disable_stream_for_test();
    set_body("{\"enable_disabled\":true}");
    capture_success = true;
    handle_put_stream(&request, &response);
    capture_success = false;
    TEST_ASSERT_EQUAL_INT(202, response.status_code);
    TEST_ASSERT_EQUAL_UINT(1, launch_attempts);
    TEST_ASSERT_NOT_NULL(failed_task);
    /* The handler must not write the DB itself (the worker would overwrite it
     * with its stale copy); it hands an enabled config to the worker. */
    TEST_ASSERT_EQUAL_INT(0, db_int_column("enabled"));
    const stream_config_t *queued = (const stream_config_t *)failed_task; /* first member */
    TEST_ASSERT_TRUE(queued->enabled);
}

void test_privacy_toggle_with_other_fields_rejected(void) {
    set_body("{\"set_privacy_mode\":true,\"priority\":3}");
    capture_success = true;
    handle_put_stream(&request, &response);
    capture_success = false;
    TEST_ASSERT_EQUAL_INT(400, response.status_code);
    TEST_ASSERT_EQUAL_UINT(0, launch_attempts);
    TEST_ASSERT_EQUAL_INT(0, db_int_column("privacy_mode"));
}

void test_privacy_toggle_with_name_accepted(void) {
    char body[512];
    snprintf(body, sizeof(body), "{\"set_privacy_mode\":true,\"name\":\"%s\"}",
             original_config.name);
    set_body(body);
    capture_success = true;
    handle_put_stream(&request, &response);
    capture_success = false;
    TEST_ASSERT_EQUAL_INT(200, response.status_code);
    TEST_ASSERT_EQUAL_UINT(0, launch_attempts);
    TEST_ASSERT_EQUAL_INT(1, db_int_column("privacy_mode"));
}

void test_queued_update_preserves_privacy_toggle(void) {
    set_body("{\"priority\":8}");
    capture_success = true;
    handle_put_stream(&request, &response);
    capture_success = false;
    TEST_ASSERT_EQUAL_INT(202, response.status_code);
    TEST_ASSERT_NOT_NULL(failed_task);
    TEST_ASSERT_NOT_NULL(captured_worker);

    /* Privacy mode is enabled while the update is still queued. */
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(get_db_handle(),
        "UPDATE streams SET privacy_mode = 1", NULL, NULL, NULL));

    void *task = failed_task;
    failed_task = NULL;   /* the worker owns and frees the task */
    captured_worker(task);

    TEST_ASSERT_EQUAL_INT(8, db_int_column("priority"));
    TEST_ASSERT_EQUAL_INT_MESSAGE(1, db_int_column("privacy_mode"),
                                  "A queued PUT must not revert a privacy toggle");
}

void test_stale_config_write_cannot_clear_privacy_mode(void) {
    /* Every in-memory setter (set_stream_recording, ...) persists via
     * update_stream_config() with a config whose privacy_mode may be stale. */
    TEST_ASSERT_EQUAL_INT(0, set_stream_privacy_mode(original_config.name, true));
    stream_config_t stale = original_config;
    stale.privacy_mode = false;
    stale.priority = 9;
    TEST_ASSERT_EQUAL_INT(0, update_stream_config(stale.name, &stale));
    TEST_ASSERT_EQUAL_INT(9, db_int_column("priority"));
    TEST_ASSERT_EQUAL_INT_MESSAGE(1, db_int_column("privacy_mode"),
                                  "update_stream_config must not write privacy_mode");
    TEST_ASSERT_EQUAL_INT(0, set_stream_privacy_mode(original_config.name, false));
    TEST_ASSERT_EQUAL_INT(0, db_int_column("privacy_mode"));
    TEST_ASSERT_EQUAL_INT(-1, set_stream_privacy_mode("no_such_stream", true));
}

static void set_privacy_body(bool enable) {
    set_body(enable ? "{\"set_privacy_mode\":true}" : "{\"set_privacy_mode\":false}");
}

void test_privacy_enable_fails_when_go2rtc_still_serves_stream(void) {
    fake_go2rtc = true;
    fake_unregister_ok = false;
    set_privacy_body(true);
    handle_put_stream(&request, &response);
    TEST_ASSERT_EQUAL_UINT(1, unregister_calls);
    TEST_ASSERT_EQUAL_INT_MESSAGE(502, response.status_code,
        "Must not report success while go2rtc still serves the camera");
    /* The saved flag stays on so sync/health monitor never re-add it. */
    TEST_ASSERT_EQUAL_INT(1, db_int_column("privacy_mode"));
}

void test_privacy_enable_removes_sub_stream(void) {
    stream_config_t config = original_config;
    strcpy(config.sub_stream_url, "rtsp://127.0.0.1:1/sub");
    TEST_ASSERT_EQUAL_INT(0, update_stream_config(config.name, &config));
    TEST_ASSERT_EQUAL_INT(0, remove_stream(original_stream));
    original_stream = add_stream(&config);
    TEST_ASSERT_NOT_NULL(original_stream);

    fake_go2rtc = true;
    set_privacy_body(true);
    handle_put_stream(&request, &response);
    TEST_ASSERT_EQUAL_INT(200, response.status_code);
    TEST_ASSERT_EQUAL_UINT(1, unregister_calls);
    TEST_ASSERT_EQUAL_UINT_MESSAGE(1, sub_remove_calls,
        "The {name}_sub stream must be removed too");
}

void test_privacy_disable_keeps_disabled_stream_unregistered(void) {
    disable_stream_for_test();
    TEST_ASSERT_EQUAL_INT(0, set_stream_privacy_mode(original_config.name, true));
    fake_go2rtc = true;
    set_privacy_body(false);
    handle_put_stream(&request, &response);
    TEST_ASSERT_EQUAL_INT(200, response.status_code);
    TEST_ASSERT_EQUAL_INT(0, db_int_column("privacy_mode"));
    TEST_ASSERT_EQUAL_UINT_MESSAGE(0, reload_calls,
        "Clearing privacy must not make a disabled camera live");
}

void test_privacy_disable_reregisters_enabled_stream(void) {
    TEST_ASSERT_EQUAL_INT(0, set_stream_privacy_mode(original_config.name, true));
    fake_go2rtc = true;
    set_privacy_body(false);
    handle_put_stream(&request, &response);
    TEST_ASSERT_EQUAL_INT(200, response.status_code);
    TEST_ASSERT_EQUAL_UINT(1, reload_calls);
    cJSON *json = cJSON_Parse((const char *)response.body);
    TEST_ASSERT_NOT_NULL(json);
    TEST_ASSERT_TRUE(cJSON_IsTrue(cJSON_GetObjectItemCaseSensitive(json, "stream_registered")));
    cJSON_Delete(json);
}

int main(void) {
    char directory[] = "/tmp/lightnvr-put-launch-XXXXXX";
    if (!mkdtemp(directory)) return 1;
    char path[192];
    snprintf(path, sizeof(path), "%s/test.db", directory);
    init_logger();
    load_default_config(&g_config);
    snprintf(g_config.storage_path, sizeof(g_config.storage_path), "%s/recordings", directory);
    if (mkdir(g_config.storage_path, 0700) != 0) return 1;
    if (init_database_ex(path, DB_INIT_NO_CHECK | DB_INIT_NO_BACKUP) != 0) return 1;
    UNITY_BEGIN();
    RUN_TEST(test_update_launch_failure);
    RUN_TEST(test_unauthorized_update_never_launches_worker);
    RUN_TEST(test_update_successful_handoff);
    RUN_TEST(test_enable_disabled_conflicting_with_enabled_false_rejected);
    RUN_TEST(test_enable_disabled_persists_through_worker);
    RUN_TEST(test_privacy_toggle_with_other_fields_rejected);
    RUN_TEST(test_privacy_toggle_with_name_accepted);
    RUN_TEST(test_queued_update_preserves_privacy_toggle);
    RUN_TEST(test_stale_config_write_cannot_clear_privacy_mode);
    RUN_TEST(test_privacy_enable_fails_when_go2rtc_still_serves_stream);
    RUN_TEST(test_privacy_enable_removes_sub_stream);
    RUN_TEST(test_privacy_disable_keeps_disabled_stream_unregistered);
    RUN_TEST(test_privacy_disable_reregisters_enabled_stream);
    int result = UNITY_END();
    shutdown_database();
    unlink(path);
    rmdir(g_config.storage_path);
    rmdir(directory);
    return result;
}
