#define _POSIX_C_SOURCE 200809L
#include <errno.h>
#include <pthread.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include "unity.h"
#include "core/config.h"
#include "core/logger.h"
#include "database/db_core.h"
#include "database/db_streams.h"
#include "video/stream_manager.h"
#include "video/stream_state.h"
#include "web/api_handlers.h"
#include "web/request_response.h"

static bool inject_failure;
static unsigned launch_attempts, task_frees;
static void *failed_task;
static stream_handle_t original_stream;
static stream_config_t original_config, original_persisted;
static http_request_t request;
static http_response_t response;

int __real_pthread_create(pthread_t *, const pthread_attr_t *, void *(*)(void *), void *);
int __wrap_pthread_create(pthread_t *thread, const pthread_attr_t *attr,
                          void *(*callback)(void *), void *argument) {
    if (!inject_failure) return __real_pthread_create(thread, attr, callback, argument);
    launch_attempts++;
    failed_task = argument;
    return EAGAIN; /* POSIX failure: callback is never scheduled. */
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
    inject_failure = false;
    launch_attempts = task_frees = 0;
    failed_task = NULL;
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
    inject_failure = false;
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
int main(void) {
    char path[160];
    snprintf(path, sizeof(path), "/tmp/lightnvr-delete-launch-%ld.db", (long)getpid());
    init_logger();
    load_default_config(&g_config);
    if (init_database_ex(path, DB_INIT_NO_CHECK | DB_INIT_NO_BACKUP) != 0) return 1;
    UNITY_BEGIN();
    RUN_TEST(test_soft_delete_launch_failure);
    RUN_TEST(test_permanent_delete_launch_failure);
    RUN_TEST(test_unauthorized_delete_never_launches_worker);
    int result = UNITY_END();
    shutdown_database();
    unlink(path);
    return result;
}
