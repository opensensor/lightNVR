#define _POSIX_C_SOURCE 200809L
/* Separate lifecycle hypothesis; never supplies an invalid worker tier. */
#include "../../src/telemetry/system_health.c"
#include "unity.h"
#include <signal.h>
#include <sys/wait.h>
#include <fcntl.h>

typedef struct { void *(*entry)(void *); void *argument; } start_argument_t;
static start_argument_t starts[SYSTEM_HEALTH_TIER_COUNT];
static int fail_after, event_fd = -1;
static atomic_uint created, joined, joined_mask, bad_join;
static pthread_t owned_threads[2 * SYSTEM_HEALTH_TIER_COUNT];
static atomic_bool fault_consumed;
static int contender_kind;
static atomic_bool contender_go, contender_at_lock, contender_done;
static int contender_result;
static atomic_uint reached_lock;
static _Thread_local bool is_worker, is_contender, reported_lock;
static _Thread_local tier_worker_t *current_worker;
static pid_t owned_child;
static int owned_pipe = -1;

static void event(char value) {
    if (event_fd >= 0 && write(event_fd, &value, 1) != 1) _exit(90);
}
int __real_pthread_mutex_lock(pthread_mutex_t *);
int __wrap_pthread_mutex_lock(pthread_mutex_t *lock) {
    if (is_worker && !reported_lock && lock == &lifecycle_lock) {
        reported_lock = true;
        event('L');
        atomic_fetch_add(&reached_lock, 1);
    }
    if (is_contender && lock == &lifecycle_lock && !reported_lock) {
        reported_lock = true; event('C'); atomic_store(&contender_at_lock, true);
    }
    return __real_pthread_mutex_lock(lock);
}
static void *worker_entry(void *opaque) {
    start_argument_t *start = opaque;
    is_worker = true; current_worker = start->argument;
    return start->entry(start->argument);
}
int __real_pthread_cond_wait(pthread_cond_t *, pthread_mutex_t *);
int __wrap_pthread_cond_wait(pthread_cond_t *condition, pthread_mutex_t *lock) {
    /* The fixed startup gate replaces the original contested-lock boundary.
     * Keep the original lifecycle-lock seam too: unchanged source stays RED. */
    if (is_worker && !reported_lock && current_worker &&
        condition == &current_worker->wake && lock == &current_worker->lock) {
        reported_lock = true; event('G'); atomic_fetch_add(&reached_lock, 1);
    }
    return __real_pthread_cond_wait(condition, lock);
}
static void await_boundary(atomic_bool *flag) {
    uint64_t deadline = monotonic_ms() + 1000;
    while (!atomic_load(flag)) {
        if (monotonic_ms() >= deadline) _exit(93);
        struct timespec pause = {0, 1000000}; nanosleep(&pause, NULL);
    }
}
static void checkpoint(unsigned workers) {
    uint64_t deadline = monotonic_ms() + 1000;
    while (atomic_load(&reached_lock) < workers) {
        if (monotonic_ms() >= deadline) _exit(91);
        struct timespec pause = {0, 1000000}; nanosleep(&pause, NULL);
    }
    if (contender_kind) {
        atomic_store(&contender_go, true);
        await_boundary(&contender_at_lock);
        if (atomic_load(&contender_done)) _exit(94);
    }
}
int __real_pthread_create(pthread_t *, const pthread_attr_t *, void *(*)(void *), void *);
int __wrap_pthread_create(pthread_t *thread, const pthread_attr_t *attr,
                          void *(*entry)(void *), void *argument) {
    if (entry != tier_worker_main) return __real_pthread_create(thread, attr, entry, argument);
    tier_worker_t *worker = argument;
    unsigned tier = (unsigned)worker->tier;
    if (tier >= SYSTEM_HEALTH_TIER_COUNT) _exit(92);
    if (fail_after >= 0 && tier == (unsigned)fail_after &&
        !atomic_exchange(&fault_consumed, true)) {
        checkpoint(tier);
        event('F');
        return EAGAIN;
    }
    starts[tier] = (start_argument_t){entry, argument};
    int result = __real_pthread_create(thread, attr, worker_entry, &starts[tier]);
    if (result == 0) {
        unsigned index = atomic_load(&created);
        if (index >= 2 * SYSTEM_HEALTH_TIER_COUNT) _exit(95);
        owned_threads[index] = *thread;
        atomic_store(&created, index + 1);
        if (contender_kind && fail_after < 0 && tier == SYSTEM_HEALTH_TIER_COUNT - 1)
            checkpoint(SYSTEM_HEALTH_TIER_COUNT);
    }
    return result;
}
int __real_pthread_join(pthread_t, void **);
int __wrap_pthread_join(pthread_t thread, void **result) {
    event('J');
    unsigned bit = 0;
    for (unsigned i = atomic_load(&created); i > 0; --i) {
        unsigned candidate = 1U << (i - 1);
        if (!(atomic_load(&joined_mask) & candidate) && pthread_equal(thread, owned_threads[i - 1])) {
            bit = candidate; break;
        }
    }
    int status = __real_pthread_join(thread, result);
    if (status == 0 && bit && !(atomic_fetch_or(&joined_mask, bit) & bit))
        atomic_fetch_add(&joined, 1);
    else atomic_fetch_add(&bad_join, 1);
    return status;
}
static void *contender_entry(void *unused) {
    (void)unused; is_contender = true;
    await_boundary(&contender_go);
    if (contender_kind == 1) contender_result = system_health_start();
    else { system_health_shutdown(); contender_result = 0; }
    atomic_store(&contender_done, true);
    return NULL;
}

static int fake_collect(void *state, const system_health_collect_context_t *context,
                        system_health_observation_sink_t *sink) {
    (void)state; (void)context; (void)sink;
    return 0;
}
static int child_scenario(int failure, int concurrent) {
    system_health_options_t options;
    system_health_options_defaults(&options);
    options.register_builtin_collectors = false;
    strcpy(options.hardware_provider, "disabled");
    for (size_t tier = 0; tier < SYSTEM_HEALTH_TIER_COUNT; ++tier)
        options.tier_interval_ms[tier] = 60000;
    if (system_health_init(&options) != 0 || system_health_provider_registry_count() != 0) return 10;
    for (unsigned tier = 0; tier < SYSTEM_HEALTH_TIER_COUNT; ++tier) {
        system_health_collector_t collector = {0};
        snprintf(collector.name, sizeof(collector.name), "fixture.%u", tier);
        collector.tier = (system_health_sampling_tier_t)tier;
        collector.scope = SYSTEM_HEALTH_SCOPE_HOST;
        collector.collect = fake_collect;
        collector.interval_seconds = 1;
        collector.stale_after_seconds = 3;
        if (!system_health_register_collector(&collector)) return 11;
    }
    fail_after = failure; created = joined = 0; atomic_store(&reached_lock, 0);
    atomic_store(&joined_mask, 0); atomic_store(&bad_join, 0);
    atomic_store(&fault_consumed, false);
    contender_kind = concurrent;
    atomic_store(&contender_go, false); atomic_store(&contender_at_lock, false);
    atomic_store(&contender_done, false); contender_result = -99;
    pthread_t contender;
    if (concurrent && __real_pthread_create(&contender, NULL, contender_entry, NULL) != 0) return 16;
    int result = system_health_start();
    if (concurrent) {
        if (result != (failure >= 0 ? -1 : 0)) return 17;
        if (__real_pthread_join(contender, NULL) != 0 || !atomic_load(&contender_done) ||
            contender_result != 0 || !atomic_load(&contender_at_lock)) return 18;
        unsigned expected = (failure >= 0 ? (unsigned)failure : SYSTEM_HEALTH_TIER_COUNT);
        if (concurrent == 1) {
            if (!runtime || !runtime->started) return 19;
            if (failure >= 0) expected += SYSTEM_HEALTH_TIER_COUNT;
            system_health_shutdown();
        }
        if (runtime || atomic_load(&created) != expected || atomic_load(&joined) != expected ||
            atomic_load(&bad_join) || atomic_load(&joined_mask) != (1U << expected) - 1) return 20;
        return 0;
    }
    if (failure >= 0) {
        if (result != -1 || created != (unsigned)failure || joined != created ||
            atomic_load(&runtime->running) || runtime->started) return 12;
        for (size_t tier = 0; tier < SYSTEM_HEALTH_TIER_COUNT; ++tier)
            if (runtime->workers[tier].started) return 13;
        /* Retry the same initialized runtime, then require actual signal/join
         * cleanup. No reset/corruption of production worker state to help it. */
        fail_after = -1; created = joined = 0; atomic_store(&reached_lock, 0);
        atomic_store(&joined_mask, 0);
        result = system_health_start();
    }
    if (result != 0 || created != SYSTEM_HEALTH_TIER_COUNT) return 14;
    system_health_shutdown();
    if (runtime || joined != created || atomic_load(&bad_join) ||
        atomic_load(&joined_mask) != (1U << atomic_load(&created)) - 1) return 15;
    return 0;
}
static void run_owned_child(int failure, int concurrent) {
    int pipes[2]; TEST_ASSERT_EQUAL_INT(0, pipe(pipes));
    fflush(NULL);
    owned_child = fork();
    if (owned_child == 0) {
        close(pipes[0]); event_fd = pipes[1];
        int result = child_scenario(failure, concurrent);
        close(event_fd); event_fd = -1;
        exit(result); /* Run sanitizer leak checks on normal child completion. */
    }
    close(pipes[1]); owned_pipe = pipes[0];
    TEST_ASSERT_GREATER_THAN_INT(0, owned_child);
    TEST_ASSERT_NOT_EQUAL(-1, fcntl(owned_pipe, F_SETFL, O_NONBLOCK));
    uint64_t deadline = monotonic_ms() + 3000;
    int status = 0; bool timeout = false;
    for (;;) {
        pid_t result = waitpid(owned_child, &status, WNOHANG);
        if (result == owned_child) break;
        TEST_ASSERT_EQUAL_INT(0, result);
        if (monotonic_ms() >= deadline) {
            timeout = true;
            /* Still an unreaped owned child: PID cannot have been recycled. */
            TEST_ASSERT_EQUAL_INT(0, kill(owned_child, SIGKILL));
            TEST_ASSERT_EQUAL_INT(owned_child, waitpid(owned_child, &status, 0));
            break;
        }
        struct timespec pause = {0, 1000000}; nanosleep(&pause, NULL);
    }
    owned_child = 0;
    unsigned locks = 0, gates = 0, faults = 0, joins = 0, contenders = 0;
    char bytes[128]; ssize_t count;
    while ((count = read(owned_pipe, bytes, sizeof(bytes))) > 0)
        for (ssize_t i = 0; i < count; ++i) {
            if (bytes[i] == 'L') locks++;
            if (bytes[i] == 'G') gates++;
            if (bytes[i] == 'C') contenders++;
            if (bytes[i] == 'F') faults++;
            if (bytes[i] == 'J') joins++;
        }
    TEST_ASSERT_EQUAL_INT(0, close(owned_pipe)); owned_pipe = -1;
    printf("startup failure=%d concurrent=%d: reached_lock=%u reached_gate=%u contender=%u injected=%u joins_entered=%u timeout=%d status=%d\n",
           failure, concurrent, locks, gates, contenders, faults, joins, timeout, status);
    if (failure >= 0) TEST_ASSERT_EQUAL_UINT(1, faults);
    if (failure > 0) {
        TEST_ASSERT_GREATER_OR_EQUAL_UINT((unsigned)failure, locks + gates);
        TEST_ASSERT_GREATER_OR_EQUAL_UINT(1, joins);
    }
    if (concurrent) TEST_ASSERT_EQUAL_UINT(1, contenders);
    TEST_ASSERT_FALSE_MESSAGE(timeout, "owned child exceeded deadline; containment kill is a failed lifecycle result");
    TEST_ASSERT_TRUE(WIFEXITED(status));
    TEST_ASSERT_EQUAL_INT(0, WEXITSTATUS(status));
}
void setUp(void) { owned_child = 0; owned_pipe = -1; }
void tearDown(void) {
    bool leaked = owned_child > 0 || owned_pipe >= 0;
    if (owned_child > 0) { kill(owned_child, SIGKILL); waitpid(owned_child, NULL, 0); owned_child = 0; }
    if (owned_pipe >= 0) { close(owned_pipe); owned_pipe = -1; }
    TEST_ASSERT_FALSE(leaked);
}
static void test_normal_start_shutdown_joins_every_worker(void) { run_owned_child(-1, 0); }
static void test_first_worker_failure_can_retry_and_shutdown(void) { run_owned_child(0, 0); }
static void test_second_worker_failure_releases_first_and_can_retry(void) { run_owned_child(1, 0); }
static void test_third_worker_failure_releases_prior_workers_and_can_retry(void) { run_owned_child(2, 0); }
static void test_fourth_worker_failure_releases_prior_workers_and_can_retry(void) { run_owned_child(3, 0); }
static void test_partial_failure_serializes_concurrent_retry(void) { run_owned_child(2, 1); }
static void test_partial_failure_serializes_concurrent_shutdown(void) { run_owned_child(2, 2); }
static void test_successful_start_serializes_concurrent_start(void) { run_owned_child(-1, 1); }
static void test_successful_start_serializes_concurrent_shutdown(void) { run_owned_child(-1, 2); }

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_normal_start_shutdown_joins_every_worker);
    RUN_TEST(test_first_worker_failure_can_retry_and_shutdown);
    RUN_TEST(test_second_worker_failure_releases_first_and_can_retry);
    RUN_TEST(test_third_worker_failure_releases_prior_workers_and_can_retry);
    RUN_TEST(test_fourth_worker_failure_releases_prior_workers_and_can_retry);
    RUN_TEST(test_partial_failure_serializes_concurrent_retry);
    RUN_TEST(test_partial_failure_serializes_concurrent_shutdown);
    RUN_TEST(test_successful_start_serializes_concurrent_start);
    RUN_TEST(test_successful_start_serializes_concurrent_shutdown);
    return UNITY_END();
}
