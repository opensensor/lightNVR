/**
 * @file test_go2rtc_lifecycle.c
 * @brief Concurrency regression tests for the shared go2rtc lifecycle.
 */

#define _POSIX_C_SOURCE 200809L

#include <pthread.h>
#include <stdatomic.h>
#include <string.h>
#include <time.h>

#include "unity.h"
#include "video/go2rtc/go2rtc_lifecycle.h"

static pthread_mutex_t test_mutex;
static pthread_cond_t test_cond;
static bool owner_started;
static atomic_int restart_bodies;

void setUp(void) {
    pthread_mutex_init(&test_mutex, NULL);
    pthread_cond_init(&test_cond, NULL);
    owner_started = false;
    atomic_store(&restart_bodies, 0);
}

void tearDown(void) {
    pthread_cond_destroy(&test_cond);
    pthread_mutex_destroy(&test_mutex);
}

static void sleep_millis(long millis) {
    struct timespec ts = {
        .tv_sec = millis / 1000,
        .tv_nsec = (millis % 1000) * 1000000L,
    };
    nanosleep(&ts, NULL);
}

static void wait_for_owner(void) {
    pthread_mutex_lock(&test_mutex);
    while (!owner_started) {
        pthread_cond_wait(&test_cond, &test_mutex);
    }
    pthread_mutex_unlock(&test_mutex);
}

static void *restart_owner(void *arg) {
    (void)arg;
    go2rtc_lifecycle_guard_t guard;
    TEST_ASSERT_TRUE(go2rtc_lifecycle_begin(
        GO2RTC_LIFECYCLE_RESTART, true, true, &guard));
    TEST_ASSERT_TRUE(guard.acquired);
    atomic_fetch_add(&restart_bodies, 1);

    pthread_mutex_lock(&test_mutex);
    owner_started = true;
    pthread_cond_broadcast(&test_cond);
    pthread_mutex_unlock(&test_mutex);

    sleep_millis(100);
    go2rtc_lifecycle_end(&guard, true);
    return NULL;
}

void test_lifecycle_is_reentrant_for_restart_stop_start_sequence(void) {
    uint64_t before = go2rtc_lifecycle_restart_generation();
    go2rtc_lifecycle_guard_t restart_guard;
    go2rtc_lifecycle_guard_t stop_guard;
    go2rtc_lifecycle_guard_t start_guard;

    TEST_ASSERT_TRUE(go2rtc_lifecycle_begin(
        GO2RTC_LIFECYCLE_RESTART, true, true, &restart_guard));
    TEST_ASSERT_TRUE(go2rtc_lifecycle_intentional_restart_active());
    TEST_ASSERT_TRUE(go2rtc_lifecycle_begin(
        GO2RTC_LIFECYCLE_PROCESS_STOP, false, true, &stop_guard));
    TEST_ASSERT_TRUE(go2rtc_lifecycle_begin(
        GO2RTC_LIFECYCLE_PROCESS_START, true, false, &start_guard));

    go2rtc_lifecycle_end(&start_guard, true);
    go2rtc_lifecycle_end(&stop_guard, true);
    TEST_ASSERT_TRUE(go2rtc_lifecycle_intentional_restart_active());
    go2rtc_lifecycle_end(&restart_guard, true);

    TEST_ASSERT_FALSE(go2rtc_lifecycle_intentional_restart_active());
    TEST_ASSERT_EQUAL_UINT64(before + 1,
                             go2rtc_lifecycle_restart_generation());
}

void test_concurrent_restarts_are_coalesced(void) {
    pthread_t owner;
    TEST_ASSERT_EQUAL_INT(0, pthread_create(&owner, NULL, restart_owner, NULL));
    wait_for_owner();

    go2rtc_lifecycle_guard_t waiter;
    TEST_ASSERT_TRUE(go2rtc_lifecycle_begin(
        GO2RTC_LIFECYCLE_RESTART, true, true, &waiter));
    TEST_ASSERT_TRUE(waiter.coalesced);
    TEST_ASSERT_TRUE(waiter.result);
    TEST_ASSERT_FALSE(waiter.acquired);

    pthread_join(owner, NULL);
    TEST_ASSERT_EQUAL_INT(1, atomic_load(&restart_bodies));
}

void test_refresh_waiter_observes_completed_restart_generation(void) {
    uint64_t queued_generation = go2rtc_lifecycle_restart_generation();
    pthread_t owner;
    TEST_ASSERT_EQUAL_INT(0, pthread_create(&owner, NULL, restart_owner, NULL));
    wait_for_owner();

    go2rtc_lifecycle_guard_t refresh_guard;
    TEST_ASSERT_TRUE(go2rtc_lifecycle_begin(
        GO2RTC_LIFECYCLE_RECONFIGURE, false, true, &refresh_guard));
    TEST_ASSERT_GREATER_THAN_UINT64(queued_generation,
                                    refresh_guard.restart_generation);
    go2rtc_lifecycle_end(&refresh_guard, true);
    pthread_join(owner, NULL);
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_lifecycle_is_reentrant_for_restart_stop_start_sequence);
    RUN_TEST(test_concurrent_restarts_are_coalesced);
    RUN_TEST(test_refresh_waiter_observes_completed_restart_generation);
    return UNITY_END();
}
