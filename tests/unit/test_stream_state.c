/**
 * @file test_stream_state.c
 * @brief Layer 3 — stream state management unit tests
 *
 * Tests init/shutdown lifecycle, create_stream_state, get_stream_state_by_name,
 * reference counting, feature flags, callback toggling, and remove_stream_state.
 */

#define _POSIX_C_SOURCE 200809L
#define _GNU_SOURCE

#include <stdio.h>
#include <string.h>

#include "unity.h"
#include "core/config.h"
#include "video/stream_state.h"

/* Build a minimal stream_config_t for testing */
static stream_config_t make_config(const char *name) {
    stream_config_t cfg;
    memset(&cfg, 0, sizeof(cfg));
    strncpy(cfg.name, name, sizeof(cfg.name) - 1);
    strncpy(cfg.url,  "rtsp://localhost/test", sizeof(cfg.url) - 1);
    cfg.enabled  = true;
    cfg.width    = 1280;
    cfg.height   = 720;
    cfg.fps      = 25;
    cfg.protocol = STREAM_PROTOCOL_TCP;
    return cfg;
}

void setUp(void)    {}
void tearDown(void) {}

/* init + shutdown lifecycle */
void test_init_shutdown_lifecycle(void) {
    int rc = init_stream_state_manager(8);
    TEST_ASSERT_EQUAL_INT(0, rc);
    shutdown_stream_state_manager();
    TEST_PASS();
}

/* double shutdown does not crash */
void test_double_shutdown(void) {
    init_stream_state_manager(4);
    shutdown_stream_state_manager();
    shutdown_stream_state_manager();  /* second call should be a no-op */
    TEST_PASS();
}

/* create_stream_state returns non-NULL */
void test_create_stream_state(void) {
    init_stream_state_manager(8);
    stream_config_t cfg = make_config("testcam");
    stream_state_manager_t *st = create_stream_state(&cfg);
    TEST_ASSERT_NOT_NULL(st);
    shutdown_stream_state_manager();
}

/* get_stream_state_by_name finds created state */
void test_get_stream_state_by_name(void) {
    init_stream_state_manager(8);
    stream_config_t cfg = make_config("findme");
    create_stream_state(&cfg);

    stream_state_manager_t *found = get_stream_state_by_name("findme");
    TEST_ASSERT_NOT_NULL(found);
    TEST_ASSERT_EQUAL_STRING("findme", found->name);
    shutdown_stream_state_manager();
}

/* get_stream_state_by_name returns NULL for unknown name */
void test_get_stream_state_by_name_not_found(void) {
    init_stream_state_manager(8);
    stream_state_manager_t *found = get_stream_state_by_name("no_such_stream");
    TEST_ASSERT_NULL(found);
    shutdown_stream_state_manager();
}

/* initial state is INACTIVE */
void test_initial_state_is_inactive(void) {
    init_stream_state_manager(8);
    stream_config_t cfg = make_config("statecam");
    stream_state_manager_t *st = create_stream_state(&cfg);
    TEST_ASSERT_NOT_NULL(st);
    stream_state_t s = get_stream_operational_state(st);
    TEST_ASSERT_EQUAL_INT(STREAM_STATE_INACTIVE, s);
    shutdown_stream_state_manager();
}

/* reference counting add_ref / release_ref */
void test_reference_counting(void) {
    init_stream_state_manager(8);
    stream_config_t cfg = make_config("refcam");
    stream_state_manager_t *st = create_stream_state(&cfg);
    TEST_ASSERT_NOT_NULL(st);

    int rc1 = stream_state_add_ref(st, STREAM_COMPONENT_API);
    TEST_ASSERT_GREATER_OR_EQUAL(1, rc1);

    int rc2 = stream_state_add_ref(st, STREAM_COMPONENT_API);
    TEST_ASSERT_EQUAL_INT(rc1 + 1, rc2);

    int rc3 = stream_state_release_ref(st, STREAM_COMPONENT_API);
    TEST_ASSERT_EQUAL_INT(rc2 - 1, rc3);

    shutdown_stream_state_manager();
}

/* get_ref_count matches add/release */
void test_get_ref_count(void) {
    init_stream_state_manager(8);
    stream_config_t cfg = make_config("refcntcam");
    stream_state_manager_t *st = create_stream_state(&cfg);

    int before = stream_state_get_ref_count(st);
    stream_state_add_ref(st, STREAM_COMPONENT_HLS);
    int after = stream_state_get_ref_count(st);
    TEST_ASSERT_EQUAL_INT(before + 1, after);

    stream_state_release_ref(st, STREAM_COMPONENT_HLS);
    TEST_ASSERT_EQUAL_INT(before, stream_state_get_ref_count(st));

    shutdown_stream_state_manager();
}

/* is_stream_state_stopping false when INACTIVE */
void test_is_stopping_when_inactive(void) {
    init_stream_state_manager(8);
    stream_config_t cfg = make_config("stopcam");
    stream_state_manager_t *st = create_stream_state(&cfg);
    TEST_ASSERT_FALSE(is_stream_state_stopping(st));
    shutdown_stream_state_manager();
}

/* set_stream_feature streaming */
void test_set_stream_feature_streaming(void) {
    init_stream_state_manager(8);
    stream_config_t cfg = make_config("featcam");
    stream_state_manager_t *st = create_stream_state(&cfg);
    TEST_ASSERT_NOT_NULL(st);

    int rc = set_stream_feature(st, "streaming", true);
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_TRUE(st->features.streaming_enabled);

    rc = set_stream_feature(st, "streaming", false);
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_FALSE(st->features.streaming_enabled);

    shutdown_stream_state_manager();
}

/* set_stream_callbacks_enabled / are_stream_callbacks_enabled */
void test_callbacks_enabled(void) {
    init_stream_state_manager(8);
    stream_config_t cfg = make_config("cbcam");
    stream_state_manager_t *st = create_stream_state(&cfg);

    set_stream_callbacks_enabled(st, true);
    TEST_ASSERT_TRUE(are_stream_callbacks_enabled(st));

    set_stream_callbacks_enabled(st, false);
    TEST_ASSERT_FALSE(are_stream_callbacks_enabled(st));

    shutdown_stream_state_manager();
}

/* remove_stream_state makes name unfindable */
void test_remove_stream_state(void) {
    init_stream_state_manager(8);
    stream_config_t cfg = make_config("removecam");
    stream_state_manager_t *st = create_stream_state(&cfg);
    TEST_ASSERT_NOT_NULL(st);

    int rc = remove_stream_state(st);
    TEST_ASSERT_EQUAL_INT(0, rc);

    stream_state_manager_t *found = get_stream_state_by_name("removecam");
    TEST_ASSERT_NULL(found);

    shutdown_stream_state_manager();
}

/* get_stream_state_count increases after create */
void test_get_stream_state_count(void) {
    init_stream_state_manager(8);
    int before = get_stream_state_count();

    stream_config_t cfg = make_config("countcam");
    create_stream_state(&cfg);

    TEST_ASSERT_EQUAL_INT(before + 1, get_stream_state_count());
    shutdown_stream_state_manager();
}

/* get_stream_state_by_index returns valid pointer */
void test_get_stream_state_by_index(void) {
    init_stream_state_manager(8);
    stream_config_t cfg = make_config("idxcam");
    create_stream_state(&cfg);

    stream_state_manager_t *st = get_stream_state_by_index(0);
    TEST_ASSERT_NOT_NULL(st);

    /* Out-of-range returns NULL */
    st = get_stream_state_by_index(9999);
    TEST_ASSERT_NULL(st);

    shutdown_stream_state_manager();
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_init_shutdown_lifecycle);
    RUN_TEST(test_double_shutdown);
    RUN_TEST(test_create_stream_state);
    RUN_TEST(test_get_stream_state_by_name);
    RUN_TEST(test_get_stream_state_by_name_not_found);
    RUN_TEST(test_initial_state_is_inactive);
    RUN_TEST(test_reference_counting);
    RUN_TEST(test_get_ref_count);
    RUN_TEST(test_is_stopping_when_inactive);
    RUN_TEST(test_set_stream_feature_streaming);
    RUN_TEST(test_callbacks_enabled);
    RUN_TEST(test_remove_stream_state);
    RUN_TEST(test_get_stream_state_count);
    RUN_TEST(test_get_stream_state_by_index);
    return UNITY_END();
}

