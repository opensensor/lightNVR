#define _POSIX_C_SOURCE 200809L

#include <pthread.h>
#include <stdio.h>
#include <string.h>
#include <time.h>

#include "telemetry/system_health.h"
#include "unity.h"

typedef struct {
    unsigned int calls;
    bool fail;
    bool old_sample;
    size_t emit_count;
    pthread_mutex_t lock;
    pthread_cond_t condition;
    bool block;
    bool entered;
    bool release;
} fake_state_t;

static uint64_t now_ms(void) {
    struct timespec value;
    clock_gettime(CLOCK_MONOTONIC, &value);
    return (uint64_t)value.tv_sec * 1000U + (uint64_t)value.tv_nsec / 1000000U;
}

static int fake_collect(void *opaque,
                        const system_health_collect_context_t *context,
                        system_health_observation_sink_t *sink) {
    fake_state_t *state = opaque;
    pthread_mutex_lock(&state->lock);
    state->calls++;
    unsigned int call = state->calls;
    if (state->block) {
        state->entered = true;
        pthread_cond_broadcast(&state->condition);
        while (!state->release)
            pthread_cond_wait(&state->condition, &state->lock);
    }
    pthread_mutex_unlock(&state->lock);
    size_t count = state->emit_count ? state->emit_count : 2U;
    for (size_t index = 0; index < count; ++index) {
        system_health_observation_t observation;
        memset(&observation, 0, sizeof(observation));
        snprintf(observation.metric, sizeof(observation.metric), "fake.%zu", index);
        snprintf(observation.resource_id, sizeof(observation.resource_id), "host");
        observation.scope = SYSTEM_HEALTH_SCOPE_HOST;
        observation.sampled_monotonic_ms = state->old_sample && context->monotonic_ms > 5000U
            ? context->monotonic_ms - 5000U : context->monotonic_ms;
        observation.observed_wall_time_ms = context->wall_time_ms;
        system_health_observation_set_available(&observation,
                                                (double)(call * (index + 1U)),
                                                SYSTEM_HEALTH_UNIT_COUNT);
        (void)system_health_observation_sink_append(sink, &observation);
    }
    return state->fail ? -1 : 0;
}

static void fake_init(fake_state_t *state) {
    memset(state, 0, sizeof(*state));
    pthread_mutex_init(&state->lock, NULL);
    pthread_cond_init(&state->condition, NULL);
}

static void fake_destroy(fake_state_t *state) {
    pthread_cond_destroy(&state->condition);
    pthread_mutex_destroy(&state->lock);
}

static system_health_collector_t descriptor(fake_state_t *state,
                                             const char *name,
                                             system_health_sampling_tier_t tier) {
    system_health_collector_t collector;
    memset(&collector, 0, sizeof(collector));
    snprintf(collector.name, sizeof(collector.name), "%s", name);
    collector.scope = SYSTEM_HEALTH_SCOPE_HOST;
    collector.tier = tier;
    collector.interval_seconds = 1U;
    collector.stale_after_seconds = 3U;
    collector.state = state;
    collector.collect = fake_collect;
    return collector;
}

static system_health_options_t options(void) {
    system_health_options_t result;
    system_health_options_defaults(&result);
    result.register_builtin_collectors = false;
    for (size_t tier = 0; tier < SYSTEM_HEALTH_TIER_COUNT; ++tier) {
        result.tier_interval_ms[tier] = 20U;
        result.collector_deadline_ms[tier] = 100U;
    }
    return result;
}

void setUp(void) { system_health_shutdown(); }
void tearDown(void) { system_health_shutdown(); }

static void test_disabled_mode_has_no_ring_or_workers(void) {
    system_health_options_t configuration = options();
    configuration.enabled = false;
    TEST_ASSERT_EQUAL_INT(0, system_health_init(&configuration));
    TEST_ASSERT_EQUAL_INT(0, system_health_start());
    system_health_stats_t stats;
    system_health_get_stats(&stats);
    TEST_ASSERT_TRUE(stats.initialized);
    TEST_ASSERT_FALSE(stats.enabled);
    TEST_ASSERT_FALSE(stats.ring_allocated);
    TEST_ASSERT_EQUAL_UINT32(0U, stats.worker_threads);
}

static void test_completed_generations_are_immutable_and_bounded(void) {
    system_health_options_t configuration = options();
    fake_state_t state;
    fake_init(&state);
    TEST_ASSERT_EQUAL_INT(0, system_health_init(&configuration));
    system_health_collector_t collector = descriptor(
        &state, "immutable", SYSTEM_HEALTH_TIER_FAST);
    TEST_ASSERT_TRUE(system_health_register_collector(&collector));
    TEST_ASSERT_FALSE(system_health_register_collector(&collector));
    TEST_ASSERT_EQUAL_INT(0, system_health_collect_tier(SYSTEM_HEALTH_TIER_FAST));
    system_health_snapshot_t first;
    TEST_ASSERT_TRUE(system_health_snapshot_copy(&first));
    TEST_ASSERT_EQUAL_INT(0, system_health_collect_tier(SYSTEM_HEALTH_TIER_FAST));
    system_health_snapshot_t second;
    TEST_ASSERT_TRUE(system_health_snapshot_copy(&second));
    TEST_ASSERT_EQUAL_UINT64(first.sequence + 1U, second.sequence);
    TEST_ASSERT_EQUAL_FLOAT(1.0, (float)first.observations[0].value);
    TEST_ASSERT_EQUAL_FLOAT(2.0, (float)second.observations[0].value);
    system_health_summary_t summaries[2];
    TEST_ASSERT_EQUAL_UINT64(2U, system_health_summary_copy(summaries, 2U));
    TEST_ASSERT_EQUAL_UINT64(first.sequence, summaries[0].sequence);
    fake_destroy(&state);
}

static void test_overflow_errors_and_stale_values_are_visible(void) {
    system_health_options_t configuration = options();
    configuration.tier_interval_ms[SYSTEM_HEALTH_TIER_FAST] = 1000U;
    fake_state_t state;
    fake_init(&state);
    state.emit_count = SYSTEM_HEALTH_MAX_OBSERVATIONS + 7U;
    state.fail = true;
    state.old_sample = true;
    TEST_ASSERT_EQUAL_INT(0, system_health_init(&configuration));
    system_health_collector_t collector = descriptor(
        &state, "overflow", SYSTEM_HEALTH_TIER_FAST);
    TEST_ASSERT_TRUE(system_health_register_collector(&collector));
    TEST_ASSERT_EQUAL_INT(-1, system_health_collect_tier(SYSTEM_HEALTH_TIER_FAST));
    system_health_snapshot_t snapshot;
    TEST_ASSERT_TRUE(system_health_snapshot_copy(&snapshot));
    TEST_ASSERT_EQUAL_UINT64(SYSTEM_HEALTH_MAX_OBSERVATIONS,
                             snapshot.observation_count);
    TEST_ASSERT_GREATER_OR_EQUAL_UINT64(7U, snapshot.observations_dropped);
    TEST_ASSERT_EQUAL_INT(SYSTEM_HEALTH_CAPABILITY_STALE,
                          snapshot.observations[0].capability);
    TEST_ASSERT_FALSE(snapshot.observations[0].value_valid);
    system_health_stats_t stats;
    system_health_get_stats(&stats);
    TEST_ASSERT_EQUAL_UINT64(1U, stats.collection_errors);
    fake_destroy(&state);
}

static void *collect_tier_thread(void *argument) {
    system_health_sampling_tier_t tier =
        *(system_health_sampling_tier_t *)argument;
    (void)system_health_collect_tier(tier);
    return NULL;
}

static void wait_until_entered(fake_state_t *state) {
    pthread_mutex_lock(&state->lock);
    while (!state->entered)
        pthread_cond_wait(&state->condition, &state->lock);
    pthread_mutex_unlock(&state->lock);
}

static void release_collector(fake_state_t *state) {
    pthread_mutex_lock(&state->lock);
    state->release = true;
    pthread_cond_broadcast(&state->condition);
    pthread_mutex_unlock(&state->lock);
}

static void test_overlap_is_skipped_and_slow_does_not_block_fast(void) {
    system_health_options_t configuration = options();
    fake_state_t slow, fast;
    fake_init(&slow);
    fake_init(&fast);
    slow.block = true;
    TEST_ASSERT_EQUAL_INT(0, system_health_init(&configuration));
    system_health_collector_t slow_collector = descriptor(
        &slow, "slow", SYSTEM_HEALTH_TIER_SLOW);
    system_health_collector_t fast_collector = descriptor(
        &fast, "fast", SYSTEM_HEALTH_TIER_FAST);
    TEST_ASSERT_TRUE(system_health_register_collector(&slow_collector));
    TEST_ASSERT_TRUE(system_health_register_collector(&fast_collector));
    system_health_sampling_tier_t slow_tier = SYSTEM_HEALTH_TIER_SLOW;
    pthread_t owner;
    TEST_ASSERT_EQUAL_INT(0, pthread_create(&owner, NULL, collect_tier_thread,
                                            &slow_tier));
    wait_until_entered(&slow);
    uint64_t started = now_ms();
    TEST_ASSERT_EQUAL_INT(0, system_health_collect_tier(SYSTEM_HEALTH_TIER_FAST));
    TEST_ASSERT_LESS_THAN_UINT64(100U, now_ms() - started);
    TEST_ASSERT_EQUAL_INT(0, system_health_collect_tier(SYSTEM_HEALTH_TIER_SLOW));
    system_health_stats_t stats;
    system_health_get_stats(&stats);
    TEST_ASSERT_GREATER_OR_EQUAL_UINT64(1U, stats.overlap_skips);
    release_collector(&slow);
    pthread_join(owner, NULL);
    fake_destroy(&slow);
    fake_destroy(&fast);
}

static void test_workers_wake_and_shutdown_quickly(void) {
    system_health_options_t configuration = options();
    fake_state_t state;
    fake_init(&state);
    TEST_ASSERT_EQUAL_INT(0, system_health_init(&configuration));
    system_health_collector_t collector = descriptor(
        &state, "worker", SYSTEM_HEALTH_TIER_FAST);
    TEST_ASSERT_TRUE(system_health_register_collector(&collector));
    TEST_ASSERT_EQUAL_INT(0, system_health_start());
    TEST_ASSERT_FALSE(system_health_register_collector(&collector));
    TEST_ASSERT_TRUE(system_health_request_tier(SYSTEM_HEALTH_TIER_FAST));
    system_health_stats_t stats;
    system_health_get_stats(&stats);
    TEST_ASSERT_EQUAL_UINT32(SYSTEM_HEALTH_TIER_COUNT, stats.worker_threads);
    uint64_t started = now_ms();
    system_health_shutdown();
    TEST_ASSERT_LESS_THAN_UINT64(500U, now_ms() - started);
    fake_destroy(&state);
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_disabled_mode_has_no_ring_or_workers);
    RUN_TEST(test_completed_generations_are_immutable_and_bounded);
    RUN_TEST(test_overflow_errors_and_stale_values_are_visible);
    RUN_TEST(test_overlap_is_skipped_and_slow_does_not_block_fast);
    RUN_TEST(test_workers_wake_and_shutdown_quickly);
    return UNITY_END();
}
