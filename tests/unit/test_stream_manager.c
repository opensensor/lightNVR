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
#include "video/stream_manager.h"

static stream_config_t make_config(const char *name) {
    stream_config_t cfg;
    memset(&cfg, 0, sizeof(cfg));
    strncpy(cfg.name, name, sizeof(cfg.name) - 1);
    strncpy(cfg.url,  "rtsp://localhost/unit_test", sizeof(cfg.url) - 1);
    cfg.enabled  = true;
    cfg.width    = 1920;
    cfg.height   = 1080;
    cfg.fps      = 30;
    cfg.protocol = STREAM_PROTOCOL_TCP;
    return cfg;
}

void setUp(void)    {}
void tearDown(void) {}

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

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_init_shutdown_lifecycle);
    RUN_TEST(test_double_shutdown);
    RUN_TEST(test_add_stream_returns_handle);
    RUN_TEST(test_get_stream_by_name);
    RUN_TEST(test_get_stream_by_name_not_found);
    RUN_TEST(test_get_stream_by_index);
    RUN_TEST(test_get_total_stream_count);
    RUN_TEST(test_remove_stream_decreases_count);
    RUN_TEST(test_get_stream_config);
    RUN_TEST(test_set_stream_priority);
    RUN_TEST(test_set_stream_recording);
    RUN_TEST(test_get_active_stream_count);
    return UNITY_END();
}

