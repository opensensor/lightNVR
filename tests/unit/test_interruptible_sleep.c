/**
 * @file test_interruptible_sleep.c
 * @brief Layer 2 Unity tests for the wakeable-sleep primitive
 *        (src/utils/interruptible_sleep.c) used to wake background threads on
 *        shutdown instead of pthread_kill(SIGALRM).
 */

#define _POSIX_C_SOURCE 200809L

#include <pthread.h>
#include <time.h>

#include "unity.h"
#include "utils/interruptible_sleep.h"

static interruptible_sleep_t s;

static long elapsed_ms_since(const struct timespec *start) {
    struct timespec now;
    clock_gettime(CLOCK_MONOTONIC, &now);
    return (now.tv_sec - start->tv_sec) * 1000 +
           (now.tv_nsec - start->tv_nsec) / 1000000;
}

void setUp(void) {
    TEST_ASSERT_EQUAL_INT(0, interruptible_sleep_init(&s));
}

void tearDown(void) {
    interruptible_sleep_destroy(&s);
}

/* With no wake, wait() blocks for approximately the requested interval. */
static void test_wait_times_out(void) {
    struct timespec start;
    clock_gettime(CLOCK_MONOTONIC, &start);

    interruptible_sleep_wait(&s, 1);

    long elapsed = elapsed_ms_since(&start);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(900, elapsed);
    TEST_ASSERT_LESS_THAN_INT(2000, elapsed);
}

/* A wake delivered before wait() is remembered (sticky), so wait() returns
 * immediately even though it asked for 10 seconds. */
static void test_sticky_wake_before_wait(void) {
    interruptible_sleep_wake(&s);

    struct timespec start;
    clock_gettime(CLOCK_MONOTONIC, &start);
    interruptible_sleep_wait(&s, 10);

    TEST_ASSERT_LESS_THAN_INT(200, elapsed_ms_since(&start));
}

/* The sticky wake is consumed by one wait(); a following wait() blocks again. */
static void test_sticky_wake_consumed_by_one_wait(void) {
    interruptible_sleep_wake(&s);
    interruptible_sleep_wait(&s, 10);   /* consumes the wake immediately */

    struct timespec start;
    clock_gettime(CLOCK_MONOTONIC, &start);
    interruptible_sleep_wait(&s, 1);    /* no pending wake → times out */

    TEST_ASSERT_GREATER_OR_EQUAL_INT(900, elapsed_ms_since(&start));
}

/* reset() drops a pending wake so the next wait() blocks normally. */
static void test_reset_clears_pending_wake(void) {
    interruptible_sleep_wake(&s);
    interruptible_sleep_reset(&s);

    struct timespec start;
    clock_gettime(CLOCK_MONOTONIC, &start);
    interruptible_sleep_wait(&s, 1);

    TEST_ASSERT_GREATER_OR_EQUAL_INT(900, elapsed_ms_since(&start));
}

static void *waker_thread(void *arg) {
    (void)arg;
    struct timespec ts = { .tv_sec = 0, .tv_nsec = 100 * 1000 * 1000 }; /* 100ms */
    nanosleep(&ts, NULL);
    interruptible_sleep_wake(&s);
    return NULL;
}

/* A wake from another thread cuts a long wait short. */
static void test_wake_interrupts_long_wait(void) {
    pthread_t t;
    struct timespec start;
    clock_gettime(CLOCK_MONOTONIC, &start);

    TEST_ASSERT_EQUAL_INT(0, pthread_create(&t, NULL, waker_thread, NULL));
    interruptible_sleep_wait(&s, 30);   /* should return ~100ms in, not 30s */
    long elapsed = elapsed_ms_since(&start);

    pthread_join(t, NULL);

    TEST_ASSERT_GREATER_OR_EQUAL_INT(80, elapsed);   /* waited for the waker */
    TEST_ASSERT_LESS_THAN_INT(2000, elapsed);        /* but nowhere near 30s */
}

/* A non-positive interval honours a pending wake but otherwise returns fast. */
static void test_zero_interval_returns_quickly(void) {
    struct timespec start;
    clock_gettime(CLOCK_MONOTONIC, &start);
    interruptible_sleep_wait(&s, 0);
    TEST_ASSERT_LESS_THAN_INT(200, elapsed_ms_since(&start));
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_wait_times_out);
    RUN_TEST(test_sticky_wake_before_wait);
    RUN_TEST(test_sticky_wake_consumed_by_one_wait);
    RUN_TEST(test_reset_clears_pending_wake);
    RUN_TEST(test_wake_interrupts_long_wait);
    RUN_TEST(test_zero_interval_returns_quickly);
    return UNITY_END();
}
