/* SPDX-License-Identifier: GPL-3.0-or-later */
#define _GNU_SOURCE
#include <errno.h>
#include <pthread.h>
#include <stdatomic.h>
#include <string.h>
#include <sys/random.h>
#include <time.h>
#include "unity.h"
#include "web/request_response.h"

typedef enum { RANDOM_FULL, RANDOM_FAILURE, RANDOM_PARTIAL } random_mode_t;
static random_mode_t random_mode;
static int random_error;
static size_t partial_count;
static atomic_uint random_calls;
static atomic_uint random_flags;

ssize_t __wrap_getrandom(void *buffer, size_t length, unsigned int flags) {
    unsigned call = atomic_fetch_add(&random_calls, 1);
    atomic_store(&random_flags, flags);
    if (random_mode == RANDOM_FAILURE) { errno = random_error; return -1; }
    if (random_mode == RANDOM_PARTIAL) {
        size_t count = partial_count < length ? partial_count : length;
        memset(buffer, 0xcc, count);
        return (ssize_t)count;
    }
    memset(buffer, 0xa5, length);
    if (length > 0) ((unsigned char *)buffer)[0] = (unsigned char)call;
    if (length > 1) ((unsigned char *)buffer)[1] = (unsigned char)(call >> 8);
    return (ssize_t)length;
}

int __real_clock_gettime(clockid_t clock, struct timespec *time);
int __wrap_clock_gettime(clockid_t clock, struct timespec *time) {
    if (clock != CLOCK_REALTIME) return __real_clock_gettime(clock, time);
    // Hold time fixed so uniqueness must come from the fallback counter.
    time->tv_sec = 1700000000;
    time->tv_nsec = 123456789;
    return 0;
}

void setUp(void) {
    random_mode = RANDOM_FULL;
    random_error = EAGAIN;
    partial_count = 0;
    atomic_store(&random_calls, 0);
    atomic_store(&random_flags, 0);
}
void tearDown(void) {}

static void assert_safe_id(const char *id) {
    size_t length = strnlen(id, REQUEST_ID_MAX);
    TEST_ASSERT_GREATER_THAN_UINT(0, length);
    TEST_ASSERT_LESS_THAN_UINT(REQUEST_ID_MAX, length);
    http_request_t ingress = {0};
    TEST_ASSERT_EQUAL_INT(0, http_request_set_request_id(&ingress, id));
    TEST_ASSERT_EQUAL_STRING(id, ingress.request_id);
}

static void test_full_random_read_produces_version_four_uuids(void) {
    char previous[REQUEST_ID_MAX] = {0};
    for (int i = 0; i < 32; i++) {
        http_request_t request;
        http_request_init(&request);
        assert_safe_id(request.request_id);
        TEST_ASSERT_EQUAL_UINT(36, strlen(request.request_id));
        TEST_ASSERT_EQUAL_CHAR('4', request.request_id[14]);
        TEST_ASSERT_NOT_NULL(strchr("89ab", request.request_id[19]));
        const int hyphens[] = {8, 13, 18, 23};
        for (size_t h = 0; h < sizeof(hyphens) / sizeof(hyphens[0]); h++) {
            TEST_ASSERT_EQUAL_CHAR('-', request.request_id[hyphens[h]]);
        }
        TEST_ASSERT_NOT_EQUAL(0, strcmp(previous, request.request_id));
        memcpy(previous, request.request_id, sizeof(previous));
    }
    TEST_ASSERT_EQUAL_UINT(GRND_NONBLOCK, atomic_load(&random_flags));
}

static void assert_fallback_sequence(void) {
    char previous[REQUEST_ID_MAX] = {0};
    for (int i = 0; i < 32; i++) {
        http_request_t request;
        http_request_init(&request);
        assert_safe_id(request.request_id);
        TEST_ASSERT_EQUAL_MEMORY("req-", request.request_id, 4);
        TEST_ASSERT_NOT_EQUAL(0, strcmp(previous, request.request_id));
        memcpy(previous, request.request_id, sizeof(previous));
    }
}

static void test_random_failures_produce_safe_unique_fallback_ids(void) {
    random_mode = RANDOM_FAILURE;
    const int errors[] = {EAGAIN, EINTR, ENOSYS};
    for (size_t i = 0; i < sizeof(errors) / sizeof(errors[0]); i++) {
        random_error = errors[i];
        assert_fallback_sequence();
    }
}

static void test_short_random_reads_use_fallback_instead_of_partial_uuids(void) {
    random_mode = RANDOM_PARTIAL;
    const size_t counts[] = {0, 1, 8, 15};
    for (size_t i = 0; i < sizeof(counts) / sizeof(counts[0]); i++) {
        partial_count = counts[i];
        assert_fallback_sequence();
    }
}

#define THREADS 4
#define IDS_PER_THREAD 32
static void *generate_fallback_ids(void *output) {
    char (*ids)[REQUEST_ID_MAX] = output;
    for (int i = 0; i < IDS_PER_THREAD; i++) {
        http_request_t request;
        http_request_init(&request);
        memcpy(ids[i], request.request_id, REQUEST_ID_MAX);
    }
    return NULL;
}

static void test_concurrent_fallback_ids_are_unique_at_the_same_time(void) {
    random_mode = RANDOM_FAILURE;
    char ids[THREADS * IDS_PER_THREAD][REQUEST_ID_MAX];
    pthread_t threads[THREADS];
    int started = 0;
    for (; started < THREADS; started++) {
        if (pthread_create(&threads[started], NULL, generate_fallback_ids,
                           ids + started * IDS_PER_THREAD) != 0) break;
    }
    for (int i = 0; i < started; i++) pthread_join(threads[i], NULL);
    TEST_ASSERT_EQUAL_INT(THREADS, started);
    for (int i = 0; i < THREADS * IDS_PER_THREAD; i++) {
        assert_safe_id(ids[i]);
        for (int j = 0; j < i; j++) TEST_ASSERT_NOT_EQUAL(0, strcmp(ids[i], ids[j]));
    }
    TEST_ASSERT_EQUAL_UINT(THREADS * IDS_PER_THREAD, atomic_load(&random_calls));
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_full_random_read_produces_version_four_uuids);
    RUN_TEST(test_random_failures_produce_safe_unique_fallback_ids);
    RUN_TEST(test_short_random_reads_use_fallback_instead_of_partial_uuids);
    RUN_TEST(test_concurrent_fallback_ids_are_unique_at_the_same_time);
    return UNITY_END();
}
