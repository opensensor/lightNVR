/**
 * @file test_audit_summary.c
 * @brief In-memory authorization summary table: keys, windows, capacity, drain.
 */

#define _POSIX_C_SOURCE 200809L

#include <pthread.h>
#include <string.h>

#include "core/authorization.h"
#include "unity.h"
#include "utils/strings.h"
#include "web/audit_summary.h"

static audit_summary_key_t make_key(const char *remote_address, int64_t window_start) {
    audit_summary_key_t key;
    audit_summary_key_init(&key);
    key.principal_user_id = 1;
    safe_strcpy(key.principal_username, "admin", sizeof(key.principal_username), 0);
    key.action = AUTHZ_LIVE_VIEW;
    safe_strcpy(key.target_type, "camera", sizeof(key.target_type), 0);
    safe_strcpy(key.target_uuid, "camera-1", sizeof(key.target_uuid), 0);
    safe_strcpy(key.remote_address, remote_address, sizeof(key.remote_address), 0);
    key.window_start = window_start;
    return key;
}

static audit_summary_sample_t sample(const char *request_id, const char *path) {
    audit_summary_sample_t value = {
        .request_id = request_id, .method = "GET", .path = path,
        .decision_source = "policy_grant", .explanation = "Allowed by role",
    };
    return value;
}

void setUp(void) {
    TEST_ASSERT_EQUAL_INT(0, audit_summary_init(AUDIT_SUMMARY_DEFAULT_CAPACITY));
}

void tearDown(void) {
    audit_summary_shutdown();
}

void test_window_start_is_wall_clock_aligned(void) {
    TEST_ASSERT_EQUAL_INT64(0, audit_summary_window_start(0));
    TEST_ASSERT_EQUAL_INT64(0, audit_summary_window_start(899));
    TEST_ASSERT_EQUAL_INT64(900, audit_summary_window_start(900));
    TEST_ASSERT_EQUAL_INT64(1757754000, audit_summary_window_start(1757754897));
}

void test_same_key_accumulates_and_keeps_first_sample(void) {
    audit_summary_key_t key = make_key("192.0.2.10", 1800);
    audit_summary_sample_t first = sample("req-1", "/api/detection/results/cam");
    audit_summary_sample_t second = sample("req-2", "/api/hls/cam/index.m3u8");
    TEST_ASSERT_EQUAL_INT(AUDIT_SUMMARY_ADDED, audit_summary_add(&key, 1801, &first));
    TEST_ASSERT_EQUAL_INT(AUDIT_SUMMARY_ADDED, audit_summary_add(&key, 1850, &second));
    TEST_ASSERT_EQUAL_INT(AUDIT_SUMMARY_ADDED, audit_summary_add(&key, 1840, &second));
    TEST_ASSERT_EQUAL_UINT(1, audit_summary_pending());

    audit_summary_entry_t out[4];
    TEST_ASSERT_EQUAL_UINT(1, audit_summary_drain(true, 0, out, 4));
    TEST_ASSERT_EQUAL_UINT64(3, out[0].count);
    TEST_ASSERT_EQUAL_INT64(1801, out[0].first_at);
    TEST_ASSERT_EQUAL_INT64(1850, out[0].last_at);
    TEST_ASSERT_EQUAL_STRING("req-1", out[0].request_id);
    TEST_ASSERT_EQUAL_STRING("/api/detection/results/cam", out[0].path);
    TEST_ASSERT_EQUAL_STRING("policy_grant", out[0].decision_source);
}

void test_distinct_client_creates_separate_entry(void) {
    audit_summary_key_t lan = make_key("192.0.2.10", 1800);
    audit_summary_key_t tailnet = make_key("100.64.0.9", 1800);
    audit_summary_sample_t s = sample("req", "/p");
    audit_summary_add(&lan, 1801, &s);
    audit_summary_add(&tailnet, 1801, &s);
    audit_summary_add(&lan, 1802, &s);
    TEST_ASSERT_EQUAL_UINT(2, audit_summary_pending());
}

void test_full_table_rejects_new_keys_without_inserting(void) {
    TEST_ASSERT_EQUAL_INT(0, audit_summary_init(2));
    audit_summary_sample_t s = sample("req", "/p");
    audit_summary_key_t a = make_key("192.0.2.1", 1800);
    audit_summary_key_t b = make_key("192.0.2.2", 1800);
    audit_summary_key_t c = make_key("192.0.2.3", 1800);
    TEST_ASSERT_EQUAL_INT(AUDIT_SUMMARY_ADDED, audit_summary_add(&a, 1801, &s));
    TEST_ASSERT_EQUAL_INT(AUDIT_SUMMARY_ADDED, audit_summary_add(&b, 1801, &s));
    TEST_ASSERT_EQUAL_INT(AUDIT_SUMMARY_FULL, audit_summary_add(&c, 1801, &s));
    TEST_ASSERT_EQUAL_INT(AUDIT_SUMMARY_ADDED, audit_summary_add(&a, 1802, &s));
    TEST_ASSERT_EQUAL_UINT(2, audit_summary_pending());
}

void test_drain_closed_windows_keeps_current_window_entries(void) {
    audit_summary_sample_t s = sample("req", "/p");
    audit_summary_key_t old_window = make_key("192.0.2.1", 900);
    audit_summary_key_t current = make_key("192.0.2.1", 1800);
    audit_summary_add(&old_window, 901, &s);
    audit_summary_add(&current, 1801, &s);

    audit_summary_entry_t out[4];
    TEST_ASSERT_EQUAL_UINT(1, audit_summary_drain(false, 1800, out, 4));
    TEST_ASSERT_EQUAL_INT64(900, out[0].key.window_start);
    TEST_ASSERT_EQUAL_UINT(1, audit_summary_pending());

    /* The kept entry must still be found after the table was rebuilt. */
    audit_summary_add(&current, 1805, &s);
    TEST_ASSERT_EQUAL_UINT(1, audit_summary_pending());
    TEST_ASSERT_EQUAL_UINT(1, audit_summary_drain(true, 0, out, 4));
    TEST_ASSERT_EQUAL_UINT64(2, out[0].count);
    TEST_ASSERT_EQUAL_UINT(0, audit_summary_pending());
}

void test_drain_respects_output_capacity(void) {
    audit_summary_sample_t s = sample("req", "/p");
    audit_summary_key_t a = make_key("192.0.2.1", 900);
    audit_summary_key_t b = make_key("192.0.2.2", 900);
    audit_summary_key_t c = make_key("192.0.2.3", 900);
    audit_summary_add(&a, 901, &s);
    audit_summary_add(&b, 901, &s);
    audit_summary_add(&c, 901, &s);
    audit_summary_entry_t out[2];
    TEST_ASSERT_EQUAL_UINT(2, audit_summary_drain(true, 0, out, 2));
    TEST_ASSERT_EQUAL_UINT(1, audit_summary_pending());
}

void test_unavailable_when_not_initialized(void) {
    audit_summary_shutdown();
    audit_summary_key_t key = make_key("192.0.2.1", 900);
    audit_summary_sample_t s = sample("req", "/p");
    TEST_ASSERT_EQUAL_INT(AUDIT_SUMMARY_UNAVAILABLE, audit_summary_add(&key, 901, &s));
    audit_summary_entry_t out[1];
    TEST_ASSERT_EQUAL_UINT(0, audit_summary_drain(true, 0, out, 1));
}

#define THREADS 8
#define ADDS_PER_THREAD 5000

static void *add_worker(void *unused) {
    (void)unused;
    audit_summary_key_t key = make_key("192.0.2.77", 1800);
    audit_summary_sample_t s = sample("req", "/p");
    for (int i = 0; i < ADDS_PER_THREAD; i++) audit_summary_add(&key, 1801, &s);
    return NULL;
}

void test_concurrent_adds_to_one_key_are_counted_exactly(void) {
    pthread_t threads[THREADS];
    for (int i = 0; i < THREADS; i++) {
        TEST_ASSERT_EQUAL_INT(0, pthread_create(&threads[i], NULL, add_worker, NULL));
    }
    for (int i = 0; i < THREADS; i++) pthread_join(threads[i], NULL);
    audit_summary_entry_t out[1];
    TEST_ASSERT_EQUAL_UINT(1, audit_summary_drain(true, 0, out, 1));
    TEST_ASSERT_EQUAL_UINT64((uint64_t)THREADS * ADDS_PER_THREAD, out[0].count);
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_window_start_is_wall_clock_aligned);
    RUN_TEST(test_same_key_accumulates_and_keeps_first_sample);
    RUN_TEST(test_distinct_client_creates_separate_entry);
    RUN_TEST(test_full_table_rejects_new_keys_without_inserting);
    RUN_TEST(test_drain_closed_windows_keeps_current_window_entries);
    RUN_TEST(test_drain_respects_output_capacity);
    RUN_TEST(test_unavailable_when_not_initialized);
    RUN_TEST(test_concurrent_adds_to_one_key_are_counted_exactly);
    return UNITY_END();
}
