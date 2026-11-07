/* Exercise the real request handler and deferred worker with a competing
 * trigger at each side effect. No camera, database, or timing-dependent sleeps are needed. */
#include "../../src/web/api_handlers_motion.c"
#include "unity.h"
#include <errno.h>

static int close_calls, notify_calls, fanout_calls;
static int concurrent_attempts, accepted_actions, unexpected_lock_errors;
static int fixture_errors, store_calls;
static bool testing_handler, allow_authorization, fail_task_alloc, fail_worker_create;
static bool last_notify_active, last_fanout_active;
static bool fail_attr_init, fail_attr_detach;
static int attr_init_calls, attr_detach_calls, attr_destroy_calls, worker_create_calls;
static pthread_attr_t *initialized_attr;

int __real_pthread_attr_init(pthread_attr_t *);
int __wrap_pthread_attr_init(pthread_attr_t *attr) {
    attr_init_calls++;
    if (fail_attr_init) return ENOMEM;
    int rc = __real_pthread_attr_init(attr);
    if (rc == 0) initialized_attr = attr;
    return rc;
}
int __real_pthread_attr_setdetachstate(pthread_attr_t *, int);
int __wrap_pthread_attr_setdetachstate(pthread_attr_t *attr, int state) {
    attr_detach_calls++;
    if (attr != initialized_attr || state != PTHREAD_CREATE_DETACHED) { fixture_errors++; return EINVAL; }
    if (fail_attr_detach) return EINVAL;
    return __real_pthread_attr_setdetachstate(attr, state);
}
int __real_pthread_attr_destroy(pthread_attr_t *);
int __wrap_pthread_attr_destroy(pthread_attr_t *attr) {
    attr_destroy_calls++;
    if (attr != initialized_attr) { fixture_errors++; return EINVAL; }
    initialized_attr = NULL;
    return __real_pthread_attr_destroy(attr);
}
static stream_config_t fixture_camera;

void *__real_calloc(size_t, size_t);
void *__wrap_calloc(size_t count, size_t size) {
    if (fail_task_alloc && count == 1 && size == sizeof(motion_pulse_task_t)) return NULL;
    return __real_calloc(count, size);
}
int __real_pthread_create(pthread_t *, const pthread_attr_t *, void *(*)(void *), void *);
int __wrap_pthread_create(pthread_t *thread, const pthread_attr_t *attr,
                          void *(*start)(void *), void *arg) {
    if (start == motion_pulse_worker) {
        worker_create_calls++;
        if (attr != initialized_attr || fail_attr_init || fail_attr_detach) fixture_errors++;
        if (fail_worker_create) return EAGAIN;
    }
    return __real_pthread_create(thread, attr, start, arg);
}
uint64_t __wrap_get_current_recording_id_for_stream(const char *stream) {
    if (strcmp(stream, "camera")) fixture_errors++;
    return 0;
}

static void *attempt_new_action(void *unused) {
    (void)unused;
    int rc = pthread_mutex_trylock(&g_pulse_gen_mutex);
    if (rc == 0) {
        pulse_gen_increment("camera");
        accepted_actions++;
        pthread_mutex_unlock(&g_pulse_gen_mutex);
    } else if (rc != EBUSY) {
        unexpected_lock_errors++;
    }
    return NULL;
}

static void compete_with_action(void) {
    pthread_t thread;
    concurrent_attempts++;
    if (pthread_create(&thread, NULL, attempt_new_action, NULL) != 0) {
        fixture_errors++;
        return;
    }
    if (pthread_join(thread, NULL) != 0) fixture_errors++;
}

int __wrap_close_external_motion_detections(const char *uuid, const char *stream, time_t end) {
    if (strcmp(uuid, "captured-camera") || strcmp(stream, "camera") || end <= 0) fixture_errors++;
    close_calls++;
    compete_with_action();
    return 0;
}

void __wrap_unified_detection_notify_motion(const char *stream, bool active) {
    if (strcmp(stream, "camera") || (!testing_handler && active)) fixture_errors++;
    last_notify_active = active;
    notify_calls++;
    compete_with_action();
}

int __wrap_process_motion_event(const char *stream, bool active, time_t timestamp, bool propagated) {
    if (strcmp(stream, "camera") || (!testing_handler && active) || propagated || timestamp <= 0) fixture_errors++;
    last_fanout_active = active;
    fanout_calls++;
    compete_with_action();
    return 0;
}

int __wrap_store_external_motion_detections(const char *stream, const detection_result_t *result,
                                            time_t timestamp, uint64_t recording_id) {
    if (strcmp(stream, "camera") || result->count != 1 || timestamp <= 0 || recording_id != 0) fixture_errors++;
    store_calls++;
    compete_with_action();
    return 0;
}

void setUp(void) {
    store_calls = 0;
    fail_attr_init = fail_attr_detach = false;
    attr_init_calls = attr_detach_calls = attr_destroy_calls = worker_create_calls = 0;
    initialized_attr = NULL;
    testing_handler = fail_task_alloc = fail_worker_create = false;
    last_notify_active = last_fanout_active = false;
    allow_authorization = true;
    memset(&fixture_camera, 0, sizeof(fixture_camera));
    safe_strcpy(fixture_camera.name, "camera", sizeof(fixture_camera.name), 0);
    safe_strcpy(fixture_camera.camera_uuid, "captured-camera", sizeof(fixture_camera.camera_uuid), 0);
    fixture_camera.detection_based_recording = true;
    close_calls = notify_calls = fanout_calls = 0;
    concurrent_attempts = accepted_actions = unexpected_lock_errors = fixture_errors = 0;
    memset(g_pulse_gen, 0, sizeof(g_pulse_gen));
    g_pulse_gen_count = 0;
    g_pulse_gen_sequence = 0;
    pthread_mutex_lock(&g_pulse_gen_mutex);
    pulse_gen_increment("camera");
    pthread_mutex_unlock(&g_pulse_gen_mutex);
}

void tearDown(void) {
    // All worker paths must release the generation mutex.
    int rc = pthread_mutex_trylock(&g_pulse_gen_mutex);
    if (rc == 0) pthread_mutex_unlock(&g_pulse_gen_mutex);
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_INT(0, fixture_errors);
    TEST_ASSERT_EQUAL_INT(0, unexpected_lock_errors);
    TEST_ASSERT_NULL(initialized_attr);
}

static void run_worker(uint64_t generation, bool recording) {
    motion_pulse_task_t *task = calloc(1, sizeof(*task));
    TEST_ASSERT_NOT_NULL(task);
    safe_strcpy(task->stream_name, "camera", sizeof(task->stream_name), 0);
    safe_strcpy(task->camera_uuid, "captured-camera", sizeof(task->camera_uuid), 0);
    task->generation = generation;
    task->trigger_recording = recording;
    // Zero delay exercises dispatch directly; the production sleep is unchanged.
    motion_pulse_worker(task);
}

static void test_current_pulse_serializes_every_stop_effect(void) {
    run_worker(1, true);
    TEST_ASSERT_EQUAL_INT(1, close_calls);
    TEST_ASSERT_EQUAL_INT(1, notify_calls);
    TEST_ASSERT_EQUAL_INT(1, fanout_calls);
    TEST_ASSERT_EQUAL_INT(3, concurrent_attempts);
    TEST_ASSERT_EQUAL_INT(0, accepted_actions);
    // A new trigger can advance the generation after the stop is complete.
    attempt_new_action(NULL);
    TEST_ASSERT_EQUAL_INT(1, accepted_actions);
}

static void test_superseded_pulse_has_no_stop_effects(void) {
    pthread_mutex_lock(&g_pulse_gen_mutex);
    pulse_gen_increment("camera");
    pthread_mutex_unlock(&g_pulse_gen_mutex);
    run_worker(1, true);
    TEST_ASSERT_EQUAL_INT(0, close_calls + notify_calls + fanout_calls);
    TEST_ASSERT_EQUAL_INT(0, concurrent_attempts);
}

static void test_untracked_pulse_has_no_stop_effects(void) {
    run_worker(0, true);
    TEST_ASSERT_EQUAL_INT(0, close_calls + notify_calls + fanout_calls);
    TEST_ASSERT_EQUAL_INT(0, concurrent_attempts);
    TEST_ASSERT_EQUAL_INT(0, accepted_actions);
}

static void test_untracked_stale_pulse_has_no_stop_effects_after_newer_action(void) {
    pthread_mutex_lock(&g_pulse_gen_mutex);
    pulse_gen_increment("camera");
    pthread_mutex_unlock(&g_pulse_gen_mutex);
    run_worker(0, true);
    TEST_ASSERT_EQUAL_INT(0, close_calls + notify_calls + fanout_calls);
    TEST_ASSERT_EQUAL_INT(0, concurrent_attempts);
    TEST_ASSERT_EQUAL_INT(0, accepted_actions);
}

stream_handle_t __wrap_get_stream_by_name(const char *name) {
    return strcmp(name, "camera") == 0 ? (stream_handle_t)&fixture_camera : NULL;
}
int __wrap_get_stream_config(stream_handle_t handle, stream_config_t *config) {
    if (handle != (stream_handle_t)&fixture_camera) return -1;
    *config = fixture_camera;
    return 0;
}
int __wrap_httpd_authorize_stream_action(const http_request_t *req, http_response_t *res,
                                         authorization_action_t action, const char *name) {
    (void)req;
    if (action != AUTHZ_CAMERA_CONFIGURE || strcmp(name, "camera")) fixture_errors++;
    if (!allow_authorization) http_response_set_json_error(res, 403, "Forbidden");
    return allow_authorization;
}

static void test_full_table_rejects_request_before_motion_side_effects(void) {
    pthread_mutex_lock(&g_pulse_gen_mutex);
    for (int i = 0; i < MAX_STREAMS; i++) {
        snprintf(g_pulse_gen[i].name, sizeof(g_pulse_gen[i].name), "occupied-%d", i);
        g_pulse_gen[i].generation = 1;
    }
    g_pulse_gen_count = MAX_STREAMS;
    pthread_mutex_unlock(&g_pulse_gen_mutex);
    memset(&fixture_camera, 0, sizeof(fixture_camera));
    safe_strcpy(fixture_camera.name, "camera", sizeof(fixture_camera.name), 0);
    safe_strcpy(fixture_camera.camera_uuid, "captured-camera", sizeof(fixture_camera.camera_uuid), 0);
    http_request_t req;
    http_response_t res;
    http_request_init(&req);
    http_response_init(&res);
    char body[] = "{\"stream\":\"camera\",\"action\":\"start\"}";
    req.method = HTTP_METHOD_POST;
    req.body = body;
    req.body_len = strlen(body);
    handle_post_motion_trigger(&req, &res);
    int status = res.status_code;
    http_response_free(&res);
    TEST_ASSERT_EQUAL_INT(500, status);
    TEST_ASSERT_EQUAL_INT(0, close_calls + notify_calls + fanout_calls);
    TEST_ASSERT_EQUAL_INT(0, concurrent_attempts);
}

static void test_annotation_only_pulse_skips_recording_notification(void) {
    run_worker(1, false);
    TEST_ASSERT_EQUAL_INT(1, close_calls);
    TEST_ASSERT_EQUAL_INT(0, notify_calls);
    TEST_ASSERT_EQUAL_INT(1, fanout_calls);
    TEST_ASSERT_EQUAL_INT(2, concurrent_attempts);
    TEST_ASSERT_EQUAL_INT(0, accepted_actions);
}


static int run_handler(const char *action) {
    testing_handler = true;
    char body[128];
    snprintf(body, sizeof(body), "{\"stream\":\"camera\",\"action\":\"%s\"}", action);
    http_request_t req;
    http_response_t res;
    http_request_init(&req);
    http_response_init(&res);
    req.method = HTTP_METHOD_POST;
    req.body = body;
    req.body_len = strlen(body);
    handle_post_motion_trigger(&req, &res);
    int status = res.status_code;
    http_response_free(&res);
    return status;
}
static void test_start_serializes_all_dispatch_effects(void) {
    TEST_ASSERT_EQUAL_INT(202, run_handler("start"));
    TEST_ASSERT_EQUAL_INT(1, close_calls);
    TEST_ASSERT_EQUAL_INT(1, store_calls);
    TEST_ASSERT_EQUAL_INT(1, notify_calls);
    TEST_ASSERT_EQUAL_INT(1, fanout_calls);
    TEST_ASSERT_TRUE(last_notify_active);
    TEST_ASSERT_TRUE(last_fanout_active);
    TEST_ASSERT_EQUAL_INT(4, concurrent_attempts);
    TEST_ASSERT_EQUAL_INT(0, accepted_actions);
}
static void test_stop_serializes_all_dispatch_effects(void) {
    TEST_ASSERT_EQUAL_INT(202, run_handler("stop"));
    TEST_ASSERT_EQUAL_INT(1, close_calls);
    TEST_ASSERT_EQUAL_INT(0, store_calls);
    TEST_ASSERT_EQUAL_INT(1, notify_calls);
    TEST_ASSERT_EQUAL_INT(1, fanout_calls);
    TEST_ASSERT_FALSE(last_notify_active);
    TEST_ASSERT_FALSE(last_fanout_active);
    TEST_ASSERT_EQUAL_INT(3, concurrent_attempts);
    TEST_ASSERT_EQUAL_INT(0, accepted_actions);
}
static void assert_pulse_failure_rolls_back_before_new_action(void) {
    TEST_ASSERT_EQUAL_INT(500, run_handler("pulse"));
    TEST_ASSERT_EQUAL_INT(2, close_calls);
    TEST_ASSERT_EQUAL_INT(1, store_calls);
    TEST_ASSERT_EQUAL_INT(2, notify_calls);
    TEST_ASSERT_EQUAL_INT(2, fanout_calls);
    TEST_ASSERT_FALSE(last_notify_active);
    TEST_ASSERT_FALSE(last_fanout_active);
    TEST_ASSERT_EQUAL_INT(7, concurrent_attempts);
    TEST_ASSERT_EQUAL_INT(0, accepted_actions);
    attempt_new_action(NULL);
    TEST_ASSERT_EQUAL_INT(1, accepted_actions);
}
static void test_task_allocation_failure_serializes_rollback(void) {
    fail_task_alloc = true;
    assert_pulse_failure_rolls_back_before_new_action();
}
static void test_worker_creation_failure_serializes_rollback(void) {
    fail_worker_create = true;
    assert_pulse_failure_rolls_back_before_new_action();
    TEST_ASSERT_EQUAL_INT(1, attr_init_calls);
    TEST_ASSERT_EQUAL_INT(1, attr_detach_calls);
    TEST_ASSERT_EQUAL_INT(1, worker_create_calls);
    TEST_ASSERT_EQUAL_INT(1, attr_destroy_calls);
}
static void test_attribute_init_failure_rolls_back_without_using_attributes(void) {
    fail_attr_init = fail_worker_create = true;
    assert_pulse_failure_rolls_back_before_new_action();
    TEST_ASSERT_EQUAL_INT(1, attr_init_calls);
    TEST_ASSERT_EQUAL_INT(0, attr_detach_calls);
    TEST_ASSERT_EQUAL_INT(0, worker_create_calls);
    TEST_ASSERT_EQUAL_INT(0, attr_destroy_calls);
}
static void test_attribute_detach_failure_destroys_attributes_and_rolls_back(void) {
    fail_attr_detach = fail_worker_create = true;
    assert_pulse_failure_rolls_back_before_new_action();
    TEST_ASSERT_EQUAL_INT(1, attr_init_calls);
    TEST_ASSERT_EQUAL_INT(1, attr_detach_calls);
    TEST_ASSERT_EQUAL_INT(0, worker_create_calls);
    TEST_ASSERT_EQUAL_INT(1, attr_destroy_calls);
}
static void test_unauthorized_request_has_no_motion_effects(void) {
    allow_authorization = false;
    TEST_ASSERT_EQUAL_INT(403, run_handler("start"));
    TEST_ASSERT_EQUAL_UINT64(1, g_pulse_gen_sequence);
    TEST_ASSERT_EQUAL_INT(0, close_calls + store_calls + notify_calls + fanout_calls);
    TEST_ASSERT_EQUAL_INT(0, concurrent_attempts);
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_current_pulse_serializes_every_stop_effect);
    RUN_TEST(test_superseded_pulse_has_no_stop_effects);
    RUN_TEST(test_untracked_pulse_has_no_stop_effects);
    RUN_TEST(test_untracked_stale_pulse_has_no_stop_effects_after_newer_action);
    RUN_TEST(test_full_table_rejects_request_before_motion_side_effects);
    RUN_TEST(test_annotation_only_pulse_skips_recording_notification);
    RUN_TEST(test_start_serializes_all_dispatch_effects);
    RUN_TEST(test_stop_serializes_all_dispatch_effects);
    RUN_TEST(test_task_allocation_failure_serializes_rollback);
    RUN_TEST(test_worker_creation_failure_serializes_rollback);
    RUN_TEST(test_unauthorized_request_has_no_motion_effects);
    RUN_TEST(test_attribute_init_failure_rolls_back_without_using_attributes);
    RUN_TEST(test_attribute_detach_failure_destroys_attributes_and_rolls_back);
    return UNITY_END();
}
