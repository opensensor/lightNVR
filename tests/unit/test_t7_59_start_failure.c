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
static unsigned created, joined;
static atomic_uint reached_lock;
static _Thread_local bool is_worker, reported_lock;
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
    return __real_pthread_mutex_lock(lock);
}
static void *worker_entry(void *opaque) {
    start_argument_t *start = opaque;
    is_worker = true;
    return start->entry(start->argument);
}
int __real_pthread_create(pthread_t *, const pthread_attr_t *, void *(*)(void *), void *);
int __wrap_pthread_create(pthread_t *thread, const pthread_attr_t *attr,
                          void *(*entry)(void *), void *argument) {
    if (entry != tier_worker_main) return __real_pthread_create(thread, attr, entry, argument);
    if (fail_after >= 0 && created == (unsigned)fail_after) {
        uint64_t deadline = monotonic_ms() + 1000;
        while (atomic_load(&reached_lock) < created) {
            if (monotonic_ms() >= deadline) _exit(91);
            struct timespec pause = {0, 1000000}; nanosleep(&pause, NULL);
        }
        /* All prior workers entered the lifecycle-lock seam. Main still owns
         * that mutex inside system_health_start: this is not a sleep-based race. */
        event('F');
        return EAGAIN;
    }
    if (created >= SYSTEM_HEALTH_TIER_COUNT) _exit(92);
    starts[created] = (start_argument_t){entry, argument};
    int result = __real_pthread_create(thread, attr, worker_entry, &starts[created]);
    if (result == 0) created++;
    return result;
}
int __real_pthread_join(pthread_t, void **);
int __wrap_pthread_join(pthread_t thread, void **result) {
    event('J');
    int status = __real_pthread_join(thread, result);
    if (status == 0) joined++;
    return status;
}
static int fake_collect(void *state, const system_health_collect_context_t *context,
                        system_health_observation_sink_t *sink) {
    (void)state; (void)context; (void)sink;
    return 0;
}
static int child_scenario(int failure) {
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
    int result = system_health_start();
    if (failure >= 0) {
        if (result != -1 || created != (unsigned)failure || joined != created ||
            atomic_load(&runtime->running) || runtime->started) return 12;
        for (size_t tier = 0; tier < SYSTEM_HEALTH_TIER_COUNT; ++tier)
            if (runtime->workers[tier].started) return 13;
        /* Retry the same initialized runtime, then require actual signal/join
         * cleanup. No reset/corruption of production worker state to help it. */
        fail_after = -1; created = joined = 0; atomic_store(&reached_lock, 0);
        result = system_health_start();
    }
    if (result != 0 || created != SYSTEM_HEALTH_TIER_COUNT) return 14;
    system_health_shutdown();
    if (runtime || joined != created) return 15;
    return 0;
}
static void run_owned_child(int failure) {
    int pipes[2]; TEST_ASSERT_EQUAL_INT(0, pipe(pipes));
    fflush(NULL);
    owned_child = fork();
    if (owned_child == 0) {
        close(pipes[0]); event_fd = pipes[1];
        int result = child_scenario(failure);
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
    unsigned locks = 0, faults = 0, joins = 0;
    char bytes[128]; ssize_t count;
    while ((count = read(owned_pipe, bytes, sizeof(bytes))) > 0)
        for (ssize_t i = 0; i < count; ++i) {
            if (bytes[i] == 'L') locks++;
            if (bytes[i] == 'F') faults++;
            if (bytes[i] == 'J') joins++;
        }
    TEST_ASSERT_EQUAL_INT(0, close(owned_pipe)); owned_pipe = -1;
    printf("startup failure=%d: reached_lock=%u injected=%u joins_entered=%u timeout=%d status=%d\n",
           failure, locks, faults, joins, timeout, status);
    if (failure >= 0) TEST_ASSERT_EQUAL_UINT(1, faults);
    if (failure > 0) {
        TEST_ASSERT_GREATER_OR_EQUAL_UINT((unsigned)failure, locks);
        TEST_ASSERT_GREATER_OR_EQUAL_UINT(1, joins);
    }
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
static void test_normal_start_shutdown_joins_every_worker(void) { run_owned_child(-1); }
static void test_first_worker_failure_can_retry_and_shutdown(void) { run_owned_child(0); }
static void test_second_worker_failure_releases_first_and_can_retry(void) { run_owned_child(1); }
static void test_third_worker_failure_releases_prior_workers_and_can_retry(void) { run_owned_child(2); }
static void test_fourth_worker_failure_releases_prior_workers_and_can_retry(void) { run_owned_child(3); }
int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_normal_start_shutdown_joins_every_worker);
    RUN_TEST(test_first_worker_failure_can_retry_and_shutdown);
    RUN_TEST(test_second_worker_failure_releases_first_and_can_retry);
    RUN_TEST(test_third_worker_failure_releases_prior_workers_and_can_retry);
    RUN_TEST(test_fourth_worker_failure_releases_prior_workers_and_can_retry);
    return UNITY_END();
}
