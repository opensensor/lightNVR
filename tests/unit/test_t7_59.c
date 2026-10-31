#define _POSIX_C_SOURCE 200809L
/* Trace actual constructed workers; never manufacture an invalid private tier. */
#include "../../src/telemetry/system_health.c"
#include "unity.h"
#include <limits.h>

static system_health_runtime_t *observed_runtime;
static atomic_uint wait_mask, bad_worker, calls[SYSTEM_HEALTH_TIER_COUNT];
static unsigned created, joined;
static pthread_t owned_threads[SYSTEM_HEALTH_TIER_COUNT];
static bool observe;
static _Thread_local struct timespec last_monotonic;
static _Thread_local bool have_monotonic;

int __real_clock_gettime(clockid_t clock, struct timespec *value);
int __wrap_clock_gettime(clockid_t clock, struct timespec *value) {
    int result = __real_clock_gettime(clock, value);
    if (result == 0 && clock == CLOCK_MONOTONIC) {
        last_monotonic = *value; have_monotonic = true;
    }
    return result;
}
int __real_pthread_create(pthread_t *, const pthread_attr_t *, void *(*)(void *), void *);
int __wrap_pthread_create(pthread_t *thread, const pthread_attr_t *attr,
                          void *(*entry)(void *), void *argument) {
    bool tracked = observe && entry == tier_worker_main;
    if (tracked) {
        bool found = false;
        for (size_t tier = 0; tier < SYSTEM_HEALTH_TIER_COUNT; ++tier) {
            tier_worker_t *worker = &observed_runtime->workers[tier];
            if (argument == worker) {
                found = worker->initialized && worker->runtime == observed_runtime &&
                        worker->tier == (system_health_sampling_tier_t)tier;
                break;
            }
        }
        if (!found || created >= SYSTEM_HEALTH_TIER_COUNT) atomic_fetch_add(&bad_worker, 1);
    }
    int result = __real_pthread_create(thread, attr, entry, argument);
    if (tracked && result == 0 && created < SYSTEM_HEALTH_TIER_COUNT)
        owned_threads[created++] = *thread;
    return result;
}
int __real_pthread_join(pthread_t, void **);
int __wrap_pthread_join(pthread_t thread, void **result) {
    int status = __real_pthread_join(thread, result);
    if (observe && status == 0)
        for (unsigned i = 0; i < created; ++i)
            if (pthread_equal(thread, owned_threads[i])) { joined++; break; }
    return status;
}
int __real_pthread_cond_timedwait(pthread_cond_t *, pthread_mutex_t *, const struct timespec *);
int __wrap_pthread_cond_timedwait(pthread_cond_t *condition, pthread_mutex_t *lock,
                                 const struct timespec *deadline) {
    if (observe && observed_runtime) {
        bool found = false;
        for (size_t tier = 0; tier < SYSTEM_HEALTH_TIER_COUNT; ++tier) {
            tier_worker_t *worker = &observed_runtime->workers[tier];
            if (condition != &worker->wake) continue;
            found = true;
            uint64_t expected_ns = (uint64_t)last_monotonic.tv_nsec +
                (uint64_t)observed_runtime->options.tier_interval_ms[tier] * UINT64_C(1000000);
            if (lock != &worker->lock || !have_monotonic ||
                deadline->tv_sec != last_monotonic.tv_sec + (time_t)(expected_ns / UINT64_C(1000000000)) ||
                deadline->tv_nsec != (long)(expected_ns % UINT64_C(1000000000)))
                atomic_fetch_add(&bad_worker, 1);
            atomic_fetch_or(&wait_mask, 1U << tier);
            break;
        }
        if (!found) atomic_fetch_add(&bad_worker, 1);
    }
    return __real_pthread_cond_timedwait(condition, lock, deadline);
}

static system_health_options_t configuration(void) {
    system_health_options_t options;
    system_health_options_defaults(&options);
    options.register_builtin_collectors = false;
    strcpy(options.hardware_provider, "disabled");
    strcpy(options.proc_root, "/nonexistent-t7-59-proc");
    strcpy(options.sys_root, "/nonexistent-t7-59-sys");
    strcpy(options.cgroup_root, "/nonexistent-t7-59-cgroup");
    for (size_t tier = 0; tier < SYSTEM_HEALTH_TIER_COUNT; ++tier) {
        options.tier_interval_ms[tier] = 13001U + (uint32_t)tier * 4001U;
        options.collector_deadline_ms[tier] = 1000;
    }
    return options;
}
static int fake_collect(void *state, const system_health_collect_context_t *context,
                        system_health_observation_sink_t *sink) {
    (void)context; (void)sink;
    atomic_fetch_add((atomic_uint *)state, 1);
    return 0;
}
static system_health_collector_t descriptor(unsigned tier) {
    system_health_collector_t collector = {0};
    snprintf(collector.name, sizeof(collector.name), "fixture.%u", tier);
    collector.tier = (system_health_sampling_tier_t)tier;
    collector.scope = SYSTEM_HEALTH_SCOPE_HOST;
    collector.collect = fake_collect;
    collector.state = &calls[tier];
    collector.interval_seconds = 1;
    collector.stale_after_seconds = 3;
    return collector;
}
static void initialize(const system_health_options_t *options) {
    TEST_ASSERT_EQUAL_INT(0, system_health_init(options));
    observed_runtime = runtime;
    TEST_ASSERT_NOT_NULL(observed_runtime);
    TEST_ASSERT_EQUAL_size_t(0, system_health_provider_registry_count());
}
static void register_fakes(void) {
    for (unsigned tier = 0; tier < SYSTEM_HEALTH_TIER_COUNT; ++tier) {
        system_health_collector_t collector = descriptor(tier);
        TEST_ASSERT_TRUE(system_health_register_collector(&collector));
    }
}
static void start_workers(void) {
    observe = true;
    TEST_ASSERT_EQUAL_INT(0, system_health_start());
    TEST_ASSERT_EQUAL_UINT(SYSTEM_HEALTH_TIER_COUNT, created);
}
static void wait_for_workers(unsigned minimum_calls) {
    uint64_t limit = monotonic_ms() + 2000;
    for (;;) {
        bool ready = atomic_load(&wait_mask) == (1U << SYSTEM_HEALTH_TIER_COUNT) - 1U;
        for (size_t tier = 0; tier < SYSTEM_HEALTH_TIER_COUNT; ++tier)
            if (atomic_load(&calls[tier]) < minimum_calls) ready = false;
        if (ready) break;
        TEST_ASSERT_LESS_THAN_UINT64(limit, monotonic_ms());
        struct timespec pause = {0, 1000000}; nanosleep(&pause, NULL);
    }
    TEST_ASSERT_EQUAL_UINT(0, atomic_load(&bad_worker));
}
static void invalid_public_tiers(void) {
    const int tiers[] = {-1, (int)SYSTEM_HEALTH_TIER_COUNT, INT_MIN, INT_MAX};
    for (size_t i = 0; i < sizeof(tiers) / sizeof(tiers[0]); ++i) {
        system_health_sampling_tier_t tier = (system_health_sampling_tier_t)tiers[i];
        TEST_ASSERT_EQUAL_INT(-1, system_health_collect_tier(tier));
        TEST_ASSERT_FALSE(system_health_request_tier(tier));
    }
}
void setUp(void) {
    system_health_shutdown();
    observed_runtime = NULL; observe = false; created = joined = 0;
    atomic_store(&wait_mask, 0); atomic_store(&bad_worker, 0);
    for (size_t tier = 0; tier < SYSTEM_HEALTH_TIER_COUNT; ++tier) atomic_store(&calls[tier], 0);
}
void tearDown(void) {
    system_health_shutdown(); /* Real signal + join; no cancellation or detached workers. */
    observe = false; observed_runtime = NULL;
    TEST_ASSERT_NULL(runtime);
    TEST_ASSERT_EQUAL_UINT(created, joined);
    TEST_ASSERT_EQUAL_UINT(0, atomic_load(&bad_worker));
}

static void test_public_invalid_tiers_rejected_without_runtime(void) { invalid_public_tiers(); }
static void test_constructor_assigns_all_and_only_array_indices(void) {
    system_health_options_t options = configuration();
    initialize(&options);
    for (size_t tier = 0; tier < SYSTEM_HEALTH_TIER_COUNT; ++tier) {
        tier_worker_t *worker = &runtime->workers[tier];
        TEST_ASSERT_TRUE(worker->initialized);
        TEST_ASSERT_EQUAL_PTR(runtime, worker->runtime);
        TEST_ASSERT_EQUAL_INT((int)tier, worker->tier);
        TEST_ASSERT_FALSE(worker->started);
        uint32_t copied = options.tier_interval_ms[tier];
        options.tier_interval_ms[tier] = 0;
        TEST_ASSERT_EQUAL_UINT32(copied, runtime->options.tier_interval_ms[tier]);
    }
    invalid_public_tiers();
}
static void test_invalid_collector_tiers_do_not_modify_worker_registry(void) {
    system_health_options_t options = configuration(); initialize(&options);
    const int tiers[] = {-1, (int)SYSTEM_HEALTH_TIER_COUNT, INT_MIN, INT_MAX};
    for (size_t i = 0; i < sizeof(tiers) / sizeof(tiers[0]); ++i) {
        system_health_collector_t collector = descriptor(0);
        collector.tier = (system_health_sampling_tier_t)tiers[i];
        TEST_ASSERT_FALSE(system_health_register_collector(&collector));
    }
    TEST_ASSERT_EQUAL_size_t(0, runtime->collector_count);
    for (size_t tier = 0; tier < SYSTEM_HEALTH_TIER_COUNT; ++tier)
        TEST_ASSERT_EQUAL_INT((int)tier, runtime->workers[tier].tier);
}
static void test_each_zero_interval_or_deadline_rejected_before_worker_allocation(void) {
    for (size_t tier = 0; tier < SYSTEM_HEALTH_TIER_COUNT; ++tier) {
        system_health_options_t options = configuration();
        options.tier_interval_ms[tier] = 0;
        TEST_ASSERT_EQUAL_INT(-1, system_health_init(&options)); TEST_ASSERT_NULL(runtime);
        options = configuration(); options.collector_deadline_ms[tier] = 0;
        TEST_ASSERT_EQUAL_INT(-1, system_health_init(&options)); TEST_ASSERT_NULL(runtime);
    }
}
static void test_actual_workers_read_own_interval_and_support_bounded_requests(void) {
    system_health_options_t options = configuration(); initialize(&options); register_fakes();
    start_workers(); wait_for_workers(1);
    invalid_public_tiers();
    for (size_t tier = 0; tier < SYSTEM_HEALTH_TIER_COUNT; ++tier)
        TEST_ASSERT_TRUE(system_health_request_tier((system_health_sampling_tier_t)tier));
    wait_for_workers(2);
    TEST_ASSERT_EQUAL_INT(0, system_health_start());
    TEST_ASSERT_EQUAL_UINT(SYSTEM_HEALTH_TIER_COUNT, created);
}
static void test_interval_extremes_use_valid_worker_slots_and_normalized_deadlines(void) {
    system_health_options_t options = configuration();
    options.tier_interval_ms[0] = 1;
    options.tier_interval_ms[1] = 999;
    options.tier_interval_ms[2] = 1000;
    options.tier_interval_ms[3] = UINT32_MAX;
    initialize(&options); register_fakes(); start_workers(); wait_for_workers(1);
}
static void test_disabled_runtime_has_no_worker_arguments_or_threads(void) {
    system_health_options_t options = configuration(); options.enabled = false;
    TEST_ASSERT_EQUAL_INT(0, system_health_init(&options));
    TEST_ASSERT_NULL(runtime); observe = true;
    TEST_ASSERT_EQUAL_INT(0, system_health_start());
    TEST_ASSERT_EQUAL_UINT(0, created);
    invalid_public_tiers();
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_public_invalid_tiers_rejected_without_runtime);
    RUN_TEST(test_constructor_assigns_all_and_only_array_indices);
    RUN_TEST(test_invalid_collector_tiers_do_not_modify_worker_registry);
    RUN_TEST(test_each_zero_interval_or_deadline_rejected_before_worker_allocation);
    RUN_TEST(test_actual_workers_read_own_interval_and_support_bounded_requests);
    RUN_TEST(test_interval_extremes_use_valid_worker_slots_and_normalized_deadlines);
    RUN_TEST(test_disabled_runtime_has_no_worker_arguments_or_threads);
    return UNITY_END();
}
