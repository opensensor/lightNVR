/**
 * @file test_timestamp_manager.c
 * @brief Layer 3 Unity tests for video/timestamp_manager.c
 *
 * Tests in-memory tracker lifecycle: init, get/create, reset, remove,
 * cleanup, keyframe time tracking, and detection time tracking.
 * No DB or network required; FFmpeg is linked for AV_NOPTS_VALUE.
 */

#define _POSIX_C_SOURCE 200809L
#define _GNU_SOURCE

#include <stdio.h>
#include <string.h>
#include <time.h>
#include <unistd.h>

#include "unity.h"
#include "video/timestamp_manager.h"

/* ---- Unity boilerplate ---- */
void setUp(void) {
    init_timestamp_trackers();
}

void tearDown(void) {
    cleanup_timestamp_trackers();
}

/* ================================================================
 * init / cleanup
 * ================================================================ */

void test_init_is_idempotent(void) {
    init_timestamp_trackers();
    /* Should survive a second call without crash */
    init_timestamp_trackers();
    TEST_PASS();
}

/* ================================================================
 * get_timestamp_tracker
 * ================================================================ */

void test_get_tracker_null_stream_returns_null(void) {
    void *t = get_timestamp_tracker(NULL);
    TEST_ASSERT_NULL(t);
}

void test_get_tracker_creates_new_entry(void) {
    void *t = get_timestamp_tracker("stream1");
    TEST_ASSERT_NOT_NULL(t);
}

void test_get_tracker_same_name_returns_same_slot(void) {
    void *t1 = get_timestamp_tracker("stream_x");
    void *t2 = get_timestamp_tracker("stream_x");
    TEST_ASSERT_NOT_NULL(t1);
    TEST_ASSERT_EQUAL_PTR(t1, t2);
}

void test_get_tracker_different_names_return_different_slots(void) {
    void *ta = get_timestamp_tracker("streamA");
    void *tb = get_timestamp_tracker("streamB");
    TEST_ASSERT_NOT_NULL(ta);
    TEST_ASSERT_NOT_NULL(tb);
    TEST_ASSERT_NOT_EQUAL(ta, tb);
}

/* ================================================================
 * reset_timestamp_tracker
 * ================================================================ */

void test_reset_nonexistent_tracker_no_crash(void) {
    reset_timestamp_tracker("ghost");
    TEST_PASS();
}

void test_reset_existing_tracker_keeps_slot(void) {
    void *t_before = get_timestamp_tracker("reset_stream");
    reset_timestamp_tracker("reset_stream");
    void *t_after = get_timestamp_tracker("reset_stream");
    /* After reset the slot is still there and reused */
    TEST_ASSERT_NOT_NULL(t_after);
    TEST_ASSERT_EQUAL_PTR(t_before, t_after);
}

/* ================================================================
 * remove_timestamp_tracker
 * ================================================================ */

void test_remove_tracker_frees_slot(void) {
    void *t1 = get_timestamp_tracker("remove_me");
    TEST_ASSERT_NOT_NULL(t1);
    remove_timestamp_tracker("remove_me");

    /* After removal the name should no longer be found; a fresh
       get creates a new (possibly different) slot. */
    void *t2 = get_timestamp_tracker("remove_me");
    TEST_ASSERT_NOT_NULL(t2); /* tracker recreated from an empty slot */
}

void test_remove_nonexistent_no_crash(void) {
    remove_timestamp_tracker("nope");
    TEST_PASS();
}

/* ================================================================
 * set_timestamp_tracker_udp_flag
 * ================================================================ */

void test_set_udp_flag_creates_tracker_if_needed(void) {
    set_timestamp_tracker_udp_flag("udp_stream", true);
    void *t = get_timestamp_tracker("udp_stream");
    TEST_ASSERT_NOT_NULL(t);
}

/* ================================================================
 * keyframe time
 * ================================================================ */

void test_last_keyframe_received_returns_zero_for_new_stream(void) {
    int result = last_keyframe_received("fresh_stream", NULL);
    TEST_ASSERT_EQUAL_INT(0, result);
}

void test_update_keyframe_time_then_received(void) {
    update_keyframe_time("kf_stream");
    int result = last_keyframe_received("kf_stream", NULL);
    TEST_ASSERT_EQUAL_INT(1, result);
}

void test_keyframe_check_time_in_the_future_returns_zero(void) {
    update_keyframe_time("kf2");
    time_t future = time(NULL) + 9999;
    int result = last_keyframe_received("kf2", &future);
    /* last keyframe time < future check_time → 0 */
    TEST_ASSERT_EQUAL_INT(0, result);
}

void test_keyframe_check_time_in_the_past_returns_one(void) {
    update_keyframe_time("kf3");
    time_t past = time(NULL) - 9999;
    int result = last_keyframe_received("kf3", &past);
    /* last keyframe time > past check_time → 1 */
    TEST_ASSERT_EQUAL_INT(1, result);
}

/* ================================================================
 * detection time
 * ================================================================ */

void test_get_last_detection_time_returns_zero_initially(void) {
    time_t t = get_last_detection_time("det_stream");
    TEST_ASSERT_EQUAL_INT(0, (int)t);
}

void test_update_and_get_detection_time(void) {
    time_t now = time(NULL);
    update_last_detection_time("det2", now);
    time_t got = get_last_detection_time("det2");
    TEST_ASSERT_EQUAL_INT((int)now, (int)got);
}

void test_update_detection_time_overwrites_previous(void) {
    time_t t1 = time(NULL);
    time_t t2 = t1 + 60;
    update_last_detection_time("det3", t1);
    update_last_detection_time("det3", t2);
    time_t got = get_last_detection_time("det3");
    TEST_ASSERT_EQUAL_INT((int)t2, (int)got);
}

/* ================================================================
 * cleanup clears all trackers
 * ================================================================ */

void test_cleanup_then_get_creates_fresh_tracker(void) {
    get_timestamp_tracker("pre_clean");
    cleanup_timestamp_trackers();
    init_timestamp_trackers();

    /* Fresh init — no keyframe set yet */
    int result = last_keyframe_received("pre_clean", NULL);
    TEST_ASSERT_EQUAL_INT(0, result);
}

/* ================================================================
 * main
 * ================================================================ */

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_init_is_idempotent);
    RUN_TEST(test_get_tracker_null_stream_returns_null);
    RUN_TEST(test_get_tracker_creates_new_entry);
    RUN_TEST(test_get_tracker_same_name_returns_same_slot);
    RUN_TEST(test_get_tracker_different_names_return_different_slots);
    RUN_TEST(test_reset_nonexistent_tracker_no_crash);
    RUN_TEST(test_reset_existing_tracker_keeps_slot);
    RUN_TEST(test_remove_tracker_frees_slot);
    RUN_TEST(test_remove_nonexistent_no_crash);
    RUN_TEST(test_set_udp_flag_creates_tracker_if_needed);
    RUN_TEST(test_last_keyframe_received_returns_zero_for_new_stream);
    RUN_TEST(test_update_keyframe_time_then_received);
    RUN_TEST(test_keyframe_check_time_in_the_future_returns_zero);
    RUN_TEST(test_keyframe_check_time_in_the_past_returns_one);
    RUN_TEST(test_get_last_detection_time_returns_zero_initially);
    RUN_TEST(test_update_and_get_detection_time);
    RUN_TEST(test_update_detection_time_overwrites_previous);
    RUN_TEST(test_cleanup_then_get_creates_fresh_tracker);
    return UNITY_END();
}

