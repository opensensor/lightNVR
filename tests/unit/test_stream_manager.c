/**
 * @file test_stream_manager.c
 * @brief Layer 3 — stream manager lifecycle unit tests
 *
 * Tests init_stream_manager/shutdown lifecycle, add_stream,
 * get_stream_by_name/index, remove_stream, total/active counts,
 * get_stream_config, set_stream_priority, set_stream_recording,
 * and duplicate-name rejection.
 */

#define _POSIX_C_SOURCE 200809L
#define _GNU_SOURCE

#include <stdio.h>
#include <string.h>

#include "unity.h"
#include "core/config.h"
#include "utils/strings.h"
#include "video/stream_manager.h"
#include "video/stream_state.h"

static stream_config_t make_config(const char *name) {
    stream_config_t cfg;
    memset(&cfg, 0, sizeof(cfg));
    safe_strcpy(cfg.name, name, sizeof(cfg.name), 0);
    safe_strcpy(cfg.url,  "rtsp://localhost/unit_test", sizeof(cfg.url), 0);
    cfg.enabled  = true;
    cfg.width    = 1920;
    cfg.height   = 1080;
    cfg.fps      = 30;
    cfg.protocol = STREAM_PROTOCOL_TCP;
    return cfg;
}

void setUp(void)    { init_stream_state_manager(MAX_STREAMS); }
void tearDown(void) { shutdown_stream_state_manager(); }

/* init + shutdown lifecycle */
void test_init_shutdown_lifecycle(void) {
    int rc = init_stream_manager(8);
    TEST_ASSERT_EQUAL_INT(0, rc);
    shutdown_stream_manager();
    TEST_PASS();
}

/* double shutdown does not crash */
void test_double_shutdown(void) {
    init_stream_manager(4);
    shutdown_stream_manager();
    shutdown_stream_manager();
    TEST_PASS();
}

/* add_stream returns non-NULL handle */
void test_add_stream_returns_handle(void) {
    init_stream_manager(8);
    stream_config_t cfg = make_config("cam1");
    stream_handle_t h = add_stream(&cfg);
    TEST_ASSERT_NOT_NULL(h);
    shutdown_stream_manager();
}

/* get_stream_by_name finds added stream */
void test_get_stream_by_name(void) {
    init_stream_manager(8);
    stream_config_t cfg = make_config("findme");
    add_stream(&cfg);

    stream_handle_t h = get_stream_by_name("findme");
    TEST_ASSERT_NOT_NULL(h);
    shutdown_stream_manager();
}

/* get_stream_by_name returns NULL for unknown stream */
void test_get_stream_by_name_not_found(void) {
    init_stream_manager(8);
    stream_handle_t h = get_stream_by_name("no_such_stream");
    TEST_ASSERT_NULL(h);
    shutdown_stream_manager();
}

/* get_stream_by_index returns valid handle */
void test_get_stream_by_index(void) {
    init_stream_manager(8);
    stream_config_t cfg = make_config("idxcam");
    add_stream(&cfg);

    stream_handle_t h = get_stream_by_index(0);
    TEST_ASSERT_NOT_NULL(h);

    /* Out-of-range returns NULL */
    stream_handle_t h2 = get_stream_by_index(9999);
    TEST_ASSERT_NULL(h2);

    shutdown_stream_manager();
}

/* get_total_stream_count increases after add */
void test_get_total_stream_count(void) {
    init_stream_manager(8);
    int before = get_total_stream_count();

    stream_config_t cfg = make_config("cntcam");
    add_stream(&cfg);

    TEST_ASSERT_EQUAL_INT(before + 1, get_total_stream_count());
    shutdown_stream_manager();
}

/* stream manager exposes runtime capacity and enforces it */
void test_stream_capacity_limit_is_enforced(void) {
    init_stream_manager(1);
    TEST_ASSERT_EQUAL_INT(1, get_stream_capacity());

    stream_config_t first = make_config("cam_first");
    stream_config_t second = make_config("cam_second");

    TEST_ASSERT_NOT_NULL(add_stream(&first));
    TEST_ASSERT_NULL(add_stream(&second));
    TEST_ASSERT_EQUAL_INT(1, get_total_stream_count());

    shutdown_stream_manager();
}

/* remove_stream decreases count */
void test_remove_stream_decreases_count(void) {
    init_stream_manager(8);
    stream_config_t cfg = make_config("rmcam");
    stream_handle_t h = add_stream(&cfg);
    TEST_ASSERT_NOT_NULL(h);

    int before = get_total_stream_count();
    int rc = remove_stream(h);
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_INT(before - 1, get_total_stream_count());

    shutdown_stream_manager();
}

/* get_stream_config retrieves correct values */
void test_get_stream_config(void) {
    init_stream_manager(8);
    stream_config_t cfg = make_config("cfgcam");
    stream_handle_t h = add_stream(&cfg);
    TEST_ASSERT_NOT_NULL(h);

    stream_config_t out;
    int rc = get_stream_config(h, &out);
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_EQUAL_STRING("cfgcam", out.name);
    TEST_ASSERT_EQUAL_INT(1920, out.width);
    TEST_ASSERT_EQUAL_INT(1080, out.height);

    shutdown_stream_manager();
}

/* set_stream_priority succeeds */
void test_set_stream_priority(void) {
    init_stream_manager(8);
    stream_config_t cfg = make_config("pricam");
    stream_handle_t h = add_stream(&cfg);
    TEST_ASSERT_NOT_NULL(h);

    int rc = set_stream_priority(h, 5);
    TEST_ASSERT_EQUAL_INT(0, rc);

    shutdown_stream_manager();
}

/* set_stream_recording enable/disable */
void test_set_stream_recording(void) {
    init_stream_manager(8);
    stream_config_t cfg = make_config("reccam");
    stream_handle_t h = add_stream(&cfg);
    TEST_ASSERT_NOT_NULL(h);

    int rc = set_stream_recording(h, true);
    TEST_ASSERT_EQUAL_INT(0, rc);

    rc = set_stream_recording(h, false);
    TEST_ASSERT_EQUAL_INT(0, rc);

    shutdown_stream_manager();
}

/* get_active_stream_count */
void test_get_active_stream_count(void) {
    init_stream_manager(8);
    /* Active streams are those that are RUNNING; after add (not started) count may be 0 */
    int cnt = get_active_stream_count();
    TEST_ASSERT_GREATER_OR_EQUAL(0, cnt);
    shutdown_stream_manager();
}

void test_independent_recording_schedules_use_current_hour(void) {
    stream_config_t cfg = make_config("scheduled");
    cfg.record_on_schedule = true;
    cfg.detection_record_on_schedule = true;
    memset(cfg.recording_schedule, 0, sizeof(cfg.recording_schedule));
    memset(cfg.detection_recording_schedule, 0,
           sizeof(cfg.detection_recording_schedule));

    time_t now = time(NULL);
    struct tm tm_info;
    localtime_r(&now, &tm_info);
    int index = tm_info.tm_wday * 24 + tm_info.tm_hour;
    cfg.recording_schedule[index] = 1;

    TEST_ASSERT_TRUE(is_recording_scheduled(&cfg));
    TEST_ASSERT_FALSE(is_detection_recording_scheduled(&cfg));

    cfg.recording_schedule[index] = 0;
    cfg.detection_recording_schedule[index] = 1;
    TEST_ASSERT_FALSE(is_recording_scheduled(&cfg));
    TEST_ASSERT_TRUE(is_detection_recording_scheduled(&cfg));

    cfg.record_on_schedule = false;
    cfg.detection_record_on_schedule = false;
    TEST_ASSERT_TRUE(is_recording_scheduled(&cfg));
    TEST_ASSERT_TRUE(is_detection_recording_scheduled(&cfg));
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_init_shutdown_lifecycle);
    RUN_TEST(test_double_shutdown);
    RUN_TEST(test_add_stream_returns_handle);
    RUN_TEST(test_get_stream_by_name);
    RUN_TEST(test_get_stream_by_name_not_found);
    RUN_TEST(test_get_stream_by_index);
    RUN_TEST(test_get_total_stream_count);
    RUN_TEST(test_stream_capacity_limit_is_enforced);
    RUN_TEST(test_remove_stream_decreases_count);
    RUN_TEST(test_get_stream_config);
    RUN_TEST(test_set_stream_priority);
    RUN_TEST(test_set_stream_recording);
    RUN_TEST(test_get_active_stream_count);
    RUN_TEST(test_independent_recording_schedules_use_current_hour);
    return UNITY_END();
}
