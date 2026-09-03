#define _POSIX_C_SOURCE 200809L

#include "telemetry/system_health.h"

#include <errno.h>
#include <pthread.h>
#include <stdatomic.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>

#include "telemetry/collectors/linux_cgroup.h"
#include "telemetry/collectors/linux_clock.h"
#include "telemetry/collectors/linux_filesystem.h"
#include "telemetry/collectors/linux_network.h"
#include "telemetry/collectors/linux_proc.h"
#include "telemetry/collectors/linux_process.h"
#include "telemetry/collectors/linux_restart.h"
#include "telemetry/collectors/linux_thermal.h"
#include "telemetry/health_helper_runner.h"
#include "telemetry/providers/builtin_providers.h"
#include "telemetry/recording_io_metrics.h"

/* Private fixed registry implemented in system_health_provider.c. */
void system_health_provider_registry_reset(void);
bool system_health_provider_registry_register(
    const system_health_provider_t *provider);
int system_health_provider_registry_collect(
    const system_health_collect_context_t *context,
    system_health_observation_sink_t *sink, size_t *resources_dropped);
void system_health_provider_registry_destroy(void);
size_t system_health_provider_registry_count(void);

typedef struct {
    system_health_collector_t collector;
    atomic_bool busy;
    pthread_mutex_t stats_lock;
    system_health_collector_stats_t stats;
} collector_slot_t;

struct system_health_runtime;
typedef struct {
    struct system_health_runtime *runtime;
    system_health_sampling_tier_t tier;
    pthread_t thread;
    pthread_mutex_t lock;
    pthread_cond_t wake;
    bool initialized;
    bool started;
    bool requested;
} tier_worker_t;

typedef struct system_health_runtime {
    system_health_options_t options;
    pthread_mutex_t lock;
    atomic_bool running;
    bool started;
    collector_slot_t collectors[SYSTEM_HEALTH_MAX_COLLECTORS];
    size_t collector_count;
    collector_slot_t provider_slot;
    system_health_snapshot_t tier_cache[SYSTEM_HEALTH_TIER_COUNT];
    bool tier_valid[SYSTEM_HEALTH_TIER_COUNT];
    system_health_snapshot_t latest[2];
    unsigned int active_latest;
    system_health_summary_t *ring;
    size_t ring_head;
    size_t ring_count;
    tier_worker_t workers[SYSTEM_HEALTH_TIER_COUNT];
    system_health_stats_t stats;

    linux_proc_state_t proc;
    linux_cgroup_state_t cgroup;
    linux_process_collector_state_t process;
    linux_filesystem_collector_state_t filesystem;
    linux_thermal_state_t thermal;
    linux_network_state_t network;
    linux_clock_source_t clock_source;
    linux_clock_state_t clock;
    char run_id[LINUX_RESTART_ID_LENGTH];
    uint64_t process_start_monotonic_ms;
} system_health_runtime_t;

static pthread_mutex_t lifecycle_lock = PTHREAD_MUTEX_INITIALIZER;
static system_health_runtime_t *runtime;
static bool disabled_initialized;

static uint64_t monotonic_ms(void) {
    struct timespec value;
    if (clock_gettime(CLOCK_MONOTONIC, &value) != 0) return 0U;
    return (uint64_t)value.tv_sec * 1000U + (uint64_t)value.tv_nsec / 1000000U;
}

static int64_t wall_time_ms(void) {
    struct timespec value;
    if (clock_gettime(CLOCK_REALTIME, &value) != 0) return 0;
    return (int64_t)value.tv_sec * 1000 + value.tv_nsec / 1000000;
}

static void bounded_copy(char *destination, size_t capacity,
                         const char *source) {
    if (!destination || capacity == 0U) return;
    snprintf(destination, capacity, "%s", source ? source : "");
}

void system_health_options_defaults(system_health_options_t *options) {
    if (!options) return;
    memset(options, 0, sizeof(*options));
    options->enabled = true;
    options->register_builtin_collectors = true;
    options->tier_interval_ms[SYSTEM_HEALTH_TIER_FAST] = 10000U;
    options->tier_interval_ms[SYSTEM_HEALTH_TIER_NORMAL] = 60000U;
    options->tier_interval_ms[SYSTEM_HEALTH_TIER_SLOW] = 300000U;
    options->tier_interval_ms[SYSTEM_HEALTH_TIER_DEVICE] = 900000U;
    options->collector_deadline_ms[SYSTEM_HEALTH_TIER_FAST] = 2000U;
    options->collector_deadline_ms[SYSTEM_HEALTH_TIER_NORMAL] = 5000U;
    options->collector_deadline_ms[SYSTEM_HEALTH_TIER_SLOW] = 15000U;
    options->collector_deadline_ms[SYSTEM_HEALTH_TIER_DEVICE] = 30000U;
    bounded_copy(options->proc_root, sizeof(options->proc_root), "/proc");
    bounded_copy(options->sys_root, sizeof(options->sys_root), "/sys");
    bounded_copy(options->cgroup_root, sizeof(options->cgroup_root),
                 "/sys/fs/cgroup");
    bounded_copy(options->root_path, sizeof(options->root_path), "/");
    bounded_copy(options->recording_path, sizeof(options->recording_path), "/");
    bounded_copy(options->hardware_provider, sizeof(options->hardware_provider),
                 "auto");
}

int system_health_options_from_policy(
    const system_health_policy_settings_t *settings, const char *recording_path,
    system_health_options_t *options) {
    if (!settings || !options) return -1;
    if (system_health_policy_validate_settings(settings, NULL) != 0) return -1;
    system_health_options_defaults(options);
    options->enabled = settings->enabled &&
                       strcmp(settings->profile, "disabled") != 0;
    options->tier_interval_ms[SYSTEM_HEALTH_TIER_FAST] =
        settings->fast_interval_seconds * 1000U;
    options->tier_interval_ms[SYSTEM_HEALTH_TIER_NORMAL] =
        settings->normal_interval_seconds * 1000U;
    options->tier_interval_ms[SYSTEM_HEALTH_TIER_SLOW] =
        settings->slow_interval_seconds * 1000U;
    options->tier_interval_ms[SYSTEM_HEALTH_TIER_DEVICE] =
        settings->device_interval_seconds * 1000U;
    if (recording_path && recording_path[0])
        bounded_copy(options->recording_path,
                     sizeof(options->recording_path), recording_path);
    bounded_copy(options->hardware_provider,
                 sizeof(options->hardware_provider),
                 settings->hardware_provider);
    return 0;
}

static bool register_collector_locked(system_health_runtime_t *state,
                                      const system_health_collector_t *source) {
    if (!state || !source || !source->name[0] || !source->collect ||
        !memchr(source->name, '\0', sizeof(source->name)) ||
        source->tier < SYSTEM_HEALTH_TIER_FAST ||
        source->tier > SYSTEM_HEALTH_TIER_DEVICE ||
        state->collector_count >= SYSTEM_HEALTH_MAX_COLLECTORS) {
        return false;
    }
    for (size_t index = 0; index < state->collector_count; ++index) {
        if (strcmp(state->collectors[index].collector.name, source->name) == 0)
            return false;
    }
    collector_slot_t *slot = &state->collectors[state->collector_count++];
    slot->collector = *source;
    atomic_init(&slot->busy, false);
    if (pthread_mutex_init(&slot->stats_lock, NULL) != 0) {
        memset(slot, 0, sizeof(*slot));
        state->collector_count--;
        return false;
    }
    bounded_copy(slot->stats.name, sizeof(slot->stats.name), source->name);
    slot->stats.scope = source->scope;
    slot->stats.tier = source->tier;
    slot->stats.interval_seconds = source->interval_seconds;
    slot->stats.stale_after_seconds = source->stale_after_seconds;
    return true;
}

static void collector_stats_sync_contract(collector_slot_t *slot) {
    pthread_mutex_lock(&slot->stats_lock);
    bounded_copy(slot->stats.name, sizeof(slot->stats.name),
                 slot->collector.name);
    slot->stats.scope = slot->collector.scope;
    slot->stats.tier = slot->collector.tier;
    slot->stats.interval_seconds = slot->collector.interval_seconds;
    slot->stats.stale_after_seconds = slot->collector.stale_after_seconds;
    pthread_mutex_unlock(&slot->stats_lock);
}

static void add_filesystem_resource(linux_filesystem_collector_state_t *state,
                                    const char *id, const char *path) {
    if (!state || !id || !path || state->resource_count >=
        SYSTEM_HEALTH_MAX_FILESYSTEMS) return;
    linux_filesystem_resource_t *resource =
        &state->resources[state->resource_count++];
    memset(resource, 0, sizeof(*resource));
    bounded_copy(resource->logical_id, sizeof(resource->logical_id), id);
    bounded_copy(resource->path, sizeof(resource->path), path);
}

static void append_builtin_observation(
    system_health_observation_sink_t *sink,
    const system_health_collect_context_t *context, const char *metric,
    system_health_scope_t scope, system_health_capability_t capability,
    bool value_valid, double value, system_health_unit_t unit) {
    system_health_observation_t observation;
    memset(&observation, 0, sizeof(observation));
    bounded_copy(observation.metric, sizeof(observation.metric), metric);
    bounded_copy(observation.resource_id, sizeof(observation.resource_id),
                 scope == SYSTEM_HEALTH_SCOPE_PROCESS ? "process" : "host");
    observation.scope = scope;
    observation.sampled_monotonic_ms = context->monotonic_ms;
    observation.observed_wall_time_ms = context->wall_time_ms;
    if (value_valid) {
        system_health_observation_set_available(&observation, value, unit);
    } else {
        system_health_observation_set_unavailable(&observation, capability);
    }
    (void)system_health_observation_sink_append(sink, &observation);
}

static int collect_clock(void *opaque,
                         const system_health_collect_context_t *context,
                         system_health_observation_sink_t *sink) {
    system_health_runtime_t *state = opaque;
    linux_clock_sample_t sample;
    if (linux_clock_sample(&state->clock_source, &sample) != 0) {
        append_builtin_observation(sink, context, "clock.synchronized",
                                   SYSTEM_HEALTH_SCOPE_HOST,
                                   SYSTEM_HEALTH_CAPABILITY_ERROR, false, 0.0,
                                   SYSTEM_HEALTH_UNIT_BOOLEAN);
        return -1;
    }
    linux_clock_result_t result = linux_clock_evaluate(
        &state->clock, &sample, LINUX_CLOCK_DEFAULT_STARTUP_GRACE_MS,
        LINUX_CLOCK_DEFAULT_JUMP_THRESHOLD_MS);
    append_builtin_observation(
        sink, context, "clock.synchronized", SYSTEM_HEALTH_SCOPE_HOST,
        result.capability, result.synchronization_known,
        result.synchronized ? 1.0 : 0.0, SYSTEM_HEALTH_UNIT_BOOLEAN);
    append_builtin_observation(
        sink, context, "clock.jump_seconds", SYSTEM_HEALTH_SCOPE_HOST,
        result.capability, result.capability == SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
        result.jump_detected ? (double)result.jump_ms / 1000.0 : 0.0,
        SYSTEM_HEALTH_UNIT_SECONDS);
    append_builtin_observation(
        sink, context, "clock.startup_grace", SYSTEM_HEALTH_SCOPE_HOST,
        SYSTEM_HEALTH_CAPABILITY_AVAILABLE, true,
        result.startup_grace_active ? 1.0 : 0.0, SYSTEM_HEALTH_UNIT_BOOLEAN);
    return 0;
}

static int collect_restart(void *opaque,
                           const system_health_collect_context_t *context,
                           system_health_observation_sink_t *sink) {
    system_health_runtime_t *state = opaque;
    linux_restart_evidence_t evidence;
    if (linux_restart_read_evidence(context->proc_root, state->run_id,
                                    state->process_start_monotonic_ms,
                                    &evidence) != 0) {
        append_builtin_observation(sink, context, "system.uptime_seconds",
                                   SYSTEM_HEALTH_SCOPE_HOST,
                                   SYSTEM_HEALTH_CAPABILITY_ERROR, false, 0.0,
                                   SYSTEM_HEALTH_UNIT_SECONDS);
        return -1;
    }
    append_builtin_observation(
        sink, context, "system.uptime_seconds", SYSTEM_HEALTH_SCOPE_HOST,
        evidence.capability,
        evidence.capability == SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
        evidence.host_uptime_seconds, SYSTEM_HEALTH_UNIT_SECONDS);
    return 0;
}

static void initialize_run_id(system_health_runtime_t *state) {
    uint64_t now = monotonic_ms();
    unsigned long pid = (unsigned long)getpid();
    snprintf(state->run_id, sizeof(state->run_id),
             "%08x-%04x-4%03x-8%03x-%012llx",
             (unsigned int)(now & 0xffffffffU),
             (unsigned int)(pid & 0xffffU),
             (unsigned int)((now >> 16U) & 0xfffU),
             (unsigned int)((pid ^ now) & 0xfffU),
             (unsigned long long)((now << 12U) ^ pid) & 0xffffffffffffULL);
    state->process_start_monotonic_ms = now;
}

static int register_portable_collectors(system_health_runtime_t *state) {
    system_health_collector_t collector;
    if (!linux_proc_collector_init(&collector, &state->proc) ||
        !register_collector_locked(state, &collector)) return -1;
    state->collectors[state->collector_count - 1U].collector.interval_seconds =
        state->options.tier_interval_ms[SYSTEM_HEALTH_TIER_FAST] / 1000U;
    state->collectors[state->collector_count - 1U].collector.stale_after_seconds =
        state->collectors[state->collector_count - 1U].collector.interval_seconds * 3U;
    if (!linux_cgroup_collector_init(&collector, &state->cgroup) ||
        !register_collector_locked(state, &collector)) return -1;
    state->collectors[state->collector_count - 1U].collector.interval_seconds =
        state->options.tier_interval_ms[SYSTEM_HEALTH_TIER_FAST] / 1000U;
    state->collectors[state->collector_count - 1U].collector.stale_after_seconds =
        state->collectors[state->collector_count - 1U].collector.interval_seconds * 3U;
    memset(&state->process, 0, sizeof(state->process));
    linux_process_collector_init(&collector, &state->process);
    if (!register_collector_locked(state, &collector)) return -1;
    state->collectors[state->collector_count - 1U].collector.interval_seconds =
        state->options.tier_interval_ms[SYSTEM_HEALTH_TIER_NORMAL] / 1000U;
    state->collectors[state->collector_count - 1U].collector.stale_after_seconds =
        state->collectors[state->collector_count - 1U].collector.interval_seconds * 3U;

    memset(&state->filesystem, 0, sizeof(state->filesystem));
    add_filesystem_resource(&state->filesystem, "root", state->options.root_path);
    if (strcmp(state->options.recording_path, state->options.root_path) != 0)
        add_filesystem_resource(&state->filesystem, "recording",
                                state->options.recording_path);
    linux_filesystem_collector_init(&collector, &state->filesystem);
    if (!register_collector_locked(state, &collector)) return -1;
    state->collectors[state->collector_count - 1U].collector.interval_seconds =
        state->options.tier_interval_ms[SYSTEM_HEALTH_TIER_NORMAL] / 1000U;
    state->collectors[state->collector_count - 1U].collector.stale_after_seconds =
        state->collectors[state->collector_count - 1U].collector.interval_seconds * 3U;

    linux_thermal_state_init(&state->thermal);
    if (!linux_thermal_collector_init(
            &collector, &state->thermal,
            state->options.tier_interval_ms[SYSTEM_HEALTH_TIER_NORMAL] / 1000U,
            state->options.tier_interval_ms[SYSTEM_HEALTH_TIER_NORMAL] / 1000U *
                3U) ||
        !register_collector_locked(state, &collector)) return -1;
    linux_network_state_init(&state->network, SYSTEM_HEALTH_SCOPE_CONTAINER);
    if (!linux_network_collector_init(
            &collector, &state->network,
            state->options.tier_interval_ms[SYSTEM_HEALTH_TIER_NORMAL] / 1000U,
            state->options.tier_interval_ms[SYSTEM_HEALTH_TIER_NORMAL] / 1000U *
                3U) ||
        !register_collector_locked(state, &collector)) return -1;

    memset(&state->clock, 0, sizeof(state->clock));
    state->clock_source = linux_clock_default_source();
    memset(&collector, 0, sizeof(collector));
    bounded_copy(collector.name, sizeof(collector.name), "linux_clock");
    collector.scope = SYSTEM_HEALTH_SCOPE_HOST;
    collector.tier = SYSTEM_HEALTH_TIER_NORMAL;
    collector.interval_seconds =
        state->options.tier_interval_ms[SYSTEM_HEALTH_TIER_NORMAL] / 1000U;
    collector.stale_after_seconds = collector.interval_seconds * 3U;
    collector.state = state;
    collector.collect = collect_clock;
    if (!register_collector_locked(state, &collector)) return -1;

    initialize_run_id(state);
    memset(&collector, 0, sizeof(collector));
    bounded_copy(collector.name, sizeof(collector.name), "linux_restart");
    collector.scope = SYSTEM_HEALTH_SCOPE_HOST;
    collector.tier = SYSTEM_HEALTH_TIER_SLOW;
    collector.interval_seconds =
        state->options.tier_interval_ms[SYSTEM_HEALTH_TIER_SLOW] / 1000U;
    collector.stale_after_seconds = collector.interval_seconds * 3U;
    collector.state = state;
    collector.collect = collect_restart;
    if (!register_collector_locked(state, &collector)) return -1;
    return 0;
}

static bool builtin_provider_callback(const system_health_provider_t *provider,
                                      void *context) {
    bool full = system_health_provider_registry_count() >=
                SYSTEM_HEALTH_MAX_DEVICES;
    bool registered = system_health_provider_registry_register(provider);
    if (!registered && full && context) {
        system_health_runtime_t *state = context;
        state->stats.coverage_overflows++;
    }
    return registered;
}

static int init_worker(tier_worker_t *worker, system_health_runtime_t *state,
                       system_health_sampling_tier_t tier) {
    pthread_condattr_t attributes;
    if (pthread_mutex_init(&worker->lock, NULL) != 0) return -1;
    if (pthread_condattr_init(&attributes) != 0) {
        pthread_mutex_destroy(&worker->lock);
        return -1;
    }
    (void)pthread_condattr_setclock(&attributes, CLOCK_MONOTONIC);
    int result = pthread_cond_init(&worker->wake, &attributes);
    pthread_condattr_destroy(&attributes);
    if (result != 0) {
        pthread_mutex_destroy(&worker->lock);
        return -1;
    }
    worker->runtime = state;
    worker->tier = tier;
    worker->initialized = true;
    return 0;
}

int system_health_init(const system_health_options_t *options) {
    if (!options) return -1;
    pthread_mutex_lock(&lifecycle_lock);
    if (runtime || disabled_initialized) {
        pthread_mutex_unlock(&lifecycle_lock);
        return -1;
    }
    if (!options->enabled) {
        disabled_initialized = true;
        pthread_mutex_unlock(&lifecycle_lock);
        return 0;
    }
    for (size_t tier = 0; tier < SYSTEM_HEALTH_TIER_COUNT; ++tier) {
        if (options->tier_interval_ms[tier] == 0U ||
            options->collector_deadline_ms[tier] == 0U) {
            pthread_mutex_unlock(&lifecycle_lock);
            return -1;
        }
    }

    system_health_runtime_t *state = calloc(1U, sizeof(*state));
    if (!state) {
        pthread_mutex_unlock(&lifecycle_lock);
        return -1;
    }
    state->ring = calloc(SYSTEM_HEALTH_RING_SAMPLES, sizeof(*state->ring));
    if (!state->ring || pthread_mutex_init(&state->lock, NULL) != 0) {
        free(state->ring);
        free(state);
        pthread_mutex_unlock(&lifecycle_lock);
        return -1;
    }
    state->options = *options;
    atomic_init(&state->running, false);
    state->stats.initialized = true;
    state->stats.enabled = true;
    state->stats.ring_allocated = true;
    system_health_provider_registry_reset();
    for (size_t tier = 0; tier < SYSTEM_HEALTH_TIER_COUNT; ++tier) {
        if (init_worker(&state->workers[tier], state,
                        (system_health_sampling_tier_t)tier) != 0) {
            for (size_t prior = 0; prior < tier; ++prior) {
                pthread_cond_destroy(&state->workers[prior].wake);
                pthread_mutex_destroy(&state->workers[prior].lock);
            }
            pthread_mutex_destroy(&state->lock);
            free(state->ring);
            free(state);
            pthread_mutex_unlock(&lifecycle_lock);
            return -1;
        }
    }
    if (options->register_builtin_collectors &&
        register_portable_collectors(state) != 0) {
        for (size_t index = 0; index < state->collector_count; ++index)
            pthread_mutex_destroy(&state->collectors[index].stats_lock);
        for (size_t tier = 0; tier < SYSTEM_HEALTH_TIER_COUNT; ++tier) {
            pthread_cond_destroy(&state->workers[tier].wake);
            pthread_mutex_destroy(&state->workers[tier].lock);
        }
        pthread_mutex_destroy(&state->lock);
        free(state->ring);
        free(state);
        pthread_mutex_unlock(&lifecycle_lock);
        return -1;
    }
    (void)system_health_register_builtin_providers(
        options->hardware_provider, builtin_provider_callback, state);
    memset(&state->provider_slot.collector, 0,
           sizeof(state->provider_slot.collector));
    bounded_copy(state->provider_slot.collector.name,
                 sizeof(state->provider_slot.collector.name),
                 "hardware_providers");
    state->provider_slot.collector.scope = SYSTEM_HEALTH_SCOPE_DEVICE;
    state->provider_slot.collector.tier = SYSTEM_HEALTH_TIER_DEVICE;
    state->provider_slot.collector.interval_seconds =
        options->tier_interval_ms[SYSTEM_HEALTH_TIER_DEVICE] / 1000U;
    state->provider_slot.collector.stale_after_seconds =
        state->provider_slot.collector.interval_seconds * 3U;
    atomic_init(&state->provider_slot.busy, false);
    if (pthread_mutex_init(&state->provider_slot.stats_lock, NULL) != 0) {
        for (size_t index = 0; index < state->collector_count; ++index)
            pthread_mutex_destroy(&state->collectors[index].stats_lock);
        for (size_t tier = 0; tier < SYSTEM_HEALTH_TIER_COUNT; ++tier) {
            pthread_cond_destroy(&state->workers[tier].wake);
            pthread_mutex_destroy(&state->workers[tier].lock);
        }
        system_health_provider_registry_destroy();
        pthread_mutex_destroy(&state->lock);
        free(state->ring);
        free(state);
        pthread_mutex_unlock(&lifecycle_lock);
        return -1;
    }
    collector_stats_sync_contract(&state->provider_slot);
    runtime = state;
    pthread_mutex_unlock(&lifecycle_lock);
    return 0;
}

bool system_health_register_collector(const system_health_collector_t *collector) {
    pthread_mutex_lock(&lifecycle_lock);
    bool overflow = runtime && !runtime->started && collector &&
        collector->name[0] && collector->collect &&
        memchr(collector->name, '\0', sizeof(collector->name)) &&
        collector->tier >= SYSTEM_HEALTH_TIER_FAST &&
        collector->tier <= SYSTEM_HEALTH_TIER_DEVICE &&
        runtime->collector_count >= SYSTEM_HEALTH_MAX_COLLECTORS;
    if (overflow) {
        for (size_t index = 0; index < runtime->collector_count; ++index) {
            if (strcmp(runtime->collectors[index].collector.name,
                       collector->name) == 0) {
                overflow = false;
                break;
            }
        }
    }
    bool result = runtime && !runtime->started &&
                  register_collector_locked(runtime, collector);
    if (!result && overflow) {
        pthread_mutex_lock(&runtime->lock);
        runtime->stats.coverage_overflows++;
        pthread_mutex_unlock(&runtime->lock);
    }
    pthread_mutex_unlock(&lifecycle_lock);
    return result;
}

bool system_health_register_provider(const system_health_provider_t *provider) {
    pthread_mutex_lock(&lifecycle_lock);
    bool full = system_health_provider_registry_count() >=
                SYSTEM_HEALTH_MAX_DEVICES;
    bool result = runtime && !runtime->started &&
                  system_health_provider_registry_register(provider);
    if (!result && full && runtime && !runtime->started && provider) {
        pthread_mutex_lock(&runtime->lock);
        runtime->stats.coverage_overflows++;
        pthread_mutex_unlock(&runtime->lock);
    }
    pthread_mutex_unlock(&lifecycle_lock);
    return result;
}

static void mark_stale(system_health_observation_t *observation) {
    observation->capability = SYSTEM_HEALTH_CAPABILITY_STALE;
    observation->freshness = SYSTEM_HEALTH_FRESHNESS_STALE;
    observation->value_valid = false;
    observation->value = 0.0;
}

static void compose_snapshot_locked(system_health_runtime_t *state,
                                    uint64_t now, int64_t wall) {
    unsigned int destination = state->active_latest == 0U ? 1U : 0U;
    system_health_snapshot_t *snapshot = &state->latest[destination];
    memset(snapshot, 0, sizeof(*snapshot));
    for (size_t tier = 0; tier < SYSTEM_HEALTH_TIER_COUNT; ++tier) {
        if (!state->tier_valid[tier]) continue;
        system_health_snapshot_t *cache = &state->tier_cache[tier];
        uint64_t stale_ms = (uint64_t)state->options.tier_interval_ms[tier] * 3U;
        for (size_t index = 0; index < cache->observation_count; ++index) {
            system_health_observation_t observation = cache->observations[index];
            if (observation.sampled_monotonic_ms > now ||
                now - observation.sampled_monotonic_ms > stale_ms) {
                mark_stale(&observation);
            }
            (void)system_health_snapshot_append(snapshot, &observation);
        }
        snapshot->observations_dropped += cache->observations_dropped;
    }
    snapshot->sequence = ++state->stats.generations_completed;
    snapshot->completed_monotonic_ms = now;
    snapshot->completed_wall_time_ms = wall;
    state->active_latest = destination;

    system_health_summary_t *summary = &state->ring[state->ring_head];
    summary->sequence = snapshot->sequence;
    summary->completed_monotonic_ms = now;
    summary->completed_wall_time_ms = wall;
    summary->observation_count = (uint32_t)snapshot->observation_count;
    summary->observations_dropped = (uint32_t)snapshot->observations_dropped;
    state->ring_head = (state->ring_head + 1U) % SYSTEM_HEALTH_RING_SAMPLES;
    if (state->ring_count < SYSTEM_HEALTH_RING_SAMPLES) state->ring_count++;
}

static bool collect_one(collector_slot_t *slot,
                        const system_health_collect_context_t *context,
                        uint32_t deadline_ms, system_health_snapshot_t *collected,
                        uint64_t *errors, uint64_t *timeouts,
                        uint64_t *overlap_skips) {
    bool expected = false;
    if (!atomic_compare_exchange_strong(&slot->busy, &expected, true)) {
        (*overlap_skips)++;
        pthread_mutex_lock(&slot->stats_lock);
        slot->stats.overlap_skips++;
        pthread_mutex_unlock(&slot->stats_lock);
        return false;
    }
    size_t first = collected->observation_count;
    system_health_observation_sink_t sink = {
        .items = collected->observations,
        .capacity = SYSTEM_HEALTH_MAX_OBSERVATIONS,
        .count = collected->observation_count,
        .dropped = 0U,
    };
    uint64_t started = monotonic_ms();
    pthread_mutex_lock(&slot->stats_lock);
    slot->stats.attempts++;
    slot->stats.last_attempt_monotonic_ms = started;
    pthread_mutex_unlock(&slot->stats_lock);
    int result = slot->collector.collect(slot->collector.state, context, &sink);
    uint64_t finished = monotonic_ms();
    uint64_t duration = finished >= started ? finished - started : 0U;
    collected->observation_count = sink.count;
    collected->observations_dropped += sink.dropped;
    if (result != 0) (*errors)++;
    bool timed_out = duration > deadline_ms;
    if (timed_out) {
        (*timeouts)++;
        for (size_t index = first; index < collected->observation_count; ++index)
            mark_stale(&collected->observations[index]);
    }
    pthread_mutex_lock(&slot->stats_lock);
    slot->stats.completions++;
    slot->stats.last_duration_ms = duration;
    if (duration > slot->stats.maximum_duration_ms)
        slot->stats.maximum_duration_ms = duration;
    if (result != 0) slot->stats.failures++;
    if (timed_out) slot->stats.timeouts++;
    if (result == 0 && !timed_out)
        slot->stats.last_success_monotonic_ms = finished;
    pthread_mutex_unlock(&slot->stats_lock);
    atomic_store(&slot->busy, false);
    return true;
}

static bool collect_providers(system_health_runtime_t *state,
                              const system_health_collect_context_t *context,
                              uint32_t deadline_ms,
                              system_health_observation_sink_t *sink,
                              size_t *resources_dropped, uint64_t *errors,
                              uint64_t *timeouts, uint64_t *overlap_skips) {
    collector_slot_t *slot = &state->provider_slot;
    bool expected = false;
    if (!atomic_compare_exchange_strong(&slot->busy, &expected, true)) {
        (*overlap_skips)++;
        pthread_mutex_lock(&slot->stats_lock);
        slot->stats.overlap_skips++;
        pthread_mutex_unlock(&slot->stats_lock);
        return false;
    }
    uint64_t started = monotonic_ms();
    pthread_mutex_lock(&slot->stats_lock);
    slot->stats.attempts++;
    slot->stats.last_attempt_monotonic_ms = started;
    pthread_mutex_unlock(&slot->stats_lock);
    int result = system_health_provider_registry_collect(
        context, sink, resources_dropped);
    uint64_t finished = monotonic_ms();
    uint64_t duration = finished >= started ? finished - started : 0U;
    bool timed_out = duration > deadline_ms;
    if (result != 0) (*errors)++;
    if (timed_out) (*timeouts)++;
    pthread_mutex_lock(&slot->stats_lock);
    slot->stats.completions++;
    slot->stats.last_duration_ms = duration;
    if (duration > slot->stats.maximum_duration_ms)
        slot->stats.maximum_duration_ms = duration;
    if (result != 0) slot->stats.failures++;
    if (timed_out) slot->stats.timeouts++;
    if (result == 0 && !timed_out)
        slot->stats.last_success_monotonic_ms = finished;
    pthread_mutex_unlock(&slot->stats_lock);
    atomic_store(&slot->busy, false);
    return true;
}

int system_health_collect_tier(system_health_sampling_tier_t tier) {
    if (tier < SYSTEM_HEALTH_TIER_FAST || tier > SYSTEM_HEALTH_TIER_DEVICE)
        return -1;
    pthread_mutex_lock(&lifecycle_lock);
    system_health_runtime_t *state = runtime;
    pthread_mutex_unlock(&lifecycle_lock);
    if (!state) return disabled_initialized ? 0 : -1;

    uint64_t now = monotonic_ms();
    int64_t wall = wall_time_ms();
    system_health_collect_context_t context = {
        .monotonic_ms = now,
        .wall_time_ms = wall,
        .proc_root = state->options.proc_root,
        .sys_root = state->options.sys_root,
        .cgroup_root = state->options.cgroup_root,
    };
    system_health_snapshot_t collected;
    memset(&collected, 0, sizeof(collected));
    uint64_t errors = 0U, timeouts = 0U, skips = 0U, completed = 0U;
    for (size_t index = 0; index < state->collector_count; ++index) {
        collector_slot_t *slot = &state->collectors[index];
        if (slot->collector.tier != tier) continue;
        size_t before = collected.observation_count;
        bool ran = collect_one(slot, &context,
                               state->options.collector_deadline_ms[tier],
                               &collected, &errors, &timeouts, &skips);
        if (ran) completed++;
        (void)before;
    }
    if (tier == SYSTEM_HEALTH_TIER_DEVICE &&
        system_health_provider_registry_count() > 0U) {
        system_health_observation_sink_t sink = {
            .items = collected.observations,
            .capacity = SYSTEM_HEALTH_MAX_OBSERVATIONS,
            .count = collected.observation_count,
        };
        size_t provider_dropped = 0U;
        bool ran = collect_providers(
            state, &context, state->options.collector_deadline_ms[tier],
            &sink, &provider_dropped, &errors, &timeouts, &skips);
        collected.observation_count = sink.count;
        collected.observations_dropped += sink.dropped + provider_dropped;
        if (ran) completed++;
    }
    collected.completed_monotonic_ms = monotonic_ms();
    collected.completed_wall_time_ms = wall_time_ms();

    pthread_mutex_lock(&state->lock);
    state->tier_cache[tier] = collected;
    state->tier_valid[tier] = true;
    state->stats.collections_completed += completed;
    state->stats.collection_errors += errors;
    state->stats.collection_timeouts += timeouts;
    state->stats.overlap_skips += skips;
    state->stats.observations_dropped += collected.observations_dropped;
    state->stats.coverage_overflows += collected.observations_dropped;
    compose_snapshot_locked(state, collected.completed_monotonic_ms,
                            collected.completed_wall_time_ms);
    pthread_mutex_unlock(&state->lock);
    if (tier == SYSTEM_HEALTH_TIER_FAST &&
        recording_io_take_device_refresh_request())
        (void)system_health_builtin_provider_request_device_refresh();
    return errors == 0U ? 0 : -1;
}

static void add_ms_to_timespec(struct timespec *value, uint32_t milliseconds) {
    value->tv_sec += milliseconds / 1000U;
    value->tv_nsec += (long)(milliseconds % 1000U) * 1000000L;
    if (value->tv_nsec >= 1000000000L) {
        value->tv_sec++;
        value->tv_nsec -= 1000000000L;
    }
}

static void *tier_worker_main(void *argument) {
    tier_worker_t *worker = argument;
    system_health_runtime_t *state = worker->runtime;
    while (atomic_load(&state->running)) {
        pthread_mutex_lock(&worker->lock);
        worker->requested = false;
        pthread_mutex_unlock(&worker->lock);
        (void)system_health_collect_tier(worker->tier);
        struct timespec deadline;
        clock_gettime(CLOCK_MONOTONIC, &deadline);
        add_ms_to_timespec(&deadline,
                           state->options.tier_interval_ms[worker->tier]);
        pthread_mutex_lock(&worker->lock);
        while (atomic_load(&state->running) && !worker->requested) {
            int result = pthread_cond_timedwait(&worker->wake, &worker->lock,
                                                &deadline);
            if (result == ETIMEDOUT) break;
        }
        pthread_mutex_unlock(&worker->lock);
    }
    return NULL;
}

int system_health_start(void) {
    pthread_mutex_lock(&lifecycle_lock);
    system_health_runtime_t *state = runtime;
    if (!state) {
        int result = disabled_initialized ? 0 : -1;
        pthread_mutex_unlock(&lifecycle_lock);
        return result;
    }
    if (state->started) {
        pthread_mutex_unlock(&lifecycle_lock);
        return 0;
    }
    atomic_store(&state->running, true);
    size_t started = 0U;
    for (size_t tier = 0; tier < SYSTEM_HEALTH_TIER_COUNT; ++tier) {
        tier_worker_t *worker = &state->workers[tier];
        if (pthread_create(&worker->thread, NULL, tier_worker_main, worker) != 0)
            break;
        worker->started = true;
        started++;
    }
    if (started != SYSTEM_HEALTH_TIER_COUNT) {
        atomic_store(&state->running, false);
        for (size_t tier = 0; tier < started; ++tier) {
            pthread_mutex_lock(&state->workers[tier].lock);
            pthread_cond_signal(&state->workers[tier].wake);
            pthread_mutex_unlock(&state->workers[tier].lock);
            pthread_join(state->workers[tier].thread, NULL);
            state->workers[tier].started = false;
        }
        pthread_mutex_unlock(&lifecycle_lock);
        return -1;
    }
    state->started = true;
    pthread_mutex_lock(&state->lock);
    state->stats.worker_threads = SYSTEM_HEALTH_TIER_COUNT;
    pthread_mutex_unlock(&state->lock);
    pthread_mutex_unlock(&lifecycle_lock);
    return 0;
}

bool system_health_request_tier(system_health_sampling_tier_t tier) {
    if (tier < SYSTEM_HEALTH_TIER_FAST || tier > SYSTEM_HEALTH_TIER_DEVICE)
        return false;
    pthread_mutex_lock(&lifecycle_lock);
    system_health_runtime_t *state = runtime;
    if (!state || !state->started) {
        pthread_mutex_unlock(&lifecycle_lock);
        return false;
    }
    tier_worker_t *worker = &state->workers[tier];
    pthread_mutex_lock(&worker->lock);
    worker->requested = true;
    pthread_cond_signal(&worker->wake);
    pthread_mutex_unlock(&worker->lock);
    pthread_mutex_unlock(&lifecycle_lock);
    return true;
}

bool system_health_snapshot_copy(system_health_snapshot_t *snapshot) {
    if (!snapshot) return false;
    pthread_mutex_lock(&lifecycle_lock);
    system_health_runtime_t *state = runtime;
    if (!state) {
        pthread_mutex_unlock(&lifecycle_lock);
        return false;
    }
    pthread_mutex_lock(&state->lock);
    *snapshot = state->latest[state->active_latest];
    bool available = snapshot->sequence != 0U;
    pthread_mutex_unlock(&state->lock);
    pthread_mutex_unlock(&lifecycle_lock);
    return available;
}

size_t system_health_summary_copy(system_health_summary_t *summaries,
                                  size_t capacity) {
    if (!summaries || capacity == 0U) return 0U;
    pthread_mutex_lock(&lifecycle_lock);
    system_health_runtime_t *state = runtime;
    if (!state) {
        pthread_mutex_unlock(&lifecycle_lock);
        return 0U;
    }
    pthread_mutex_lock(&state->lock);
    size_t count = state->ring_count < capacity ? state->ring_count : capacity;
    size_t oldest = (state->ring_head + SYSTEM_HEALTH_RING_SAMPLES -
                     state->ring_count) % SYSTEM_HEALTH_RING_SAMPLES;
    for (size_t index = 0; index < count; ++index)
        summaries[index] = state->ring[(oldest + index) %
                                       SYSTEM_HEALTH_RING_SAMPLES];
    pthread_mutex_unlock(&state->lock);
    pthread_mutex_unlock(&lifecycle_lock);
    return count;
}

void system_health_get_stats(system_health_stats_t *stats) {
    if (!stats) return;
    memset(stats, 0, sizeof(*stats));
    pthread_mutex_lock(&lifecycle_lock);
    system_health_runtime_t *state = runtime;
    if (state) {
        pthread_mutex_lock(&state->lock);
        *stats = state->stats;
        pthread_mutex_unlock(&state->lock);
    } else if (disabled_initialized) {
        stats->initialized = true;
    }
    pthread_mutex_unlock(&lifecycle_lock);
    stats->abandoned_helpers = health_helper_abandoned_count();
}

static void copy_collector_stats(collector_slot_t *slot, uint64_t now,
                                 system_health_collector_stats_t *output) {
    pthread_mutex_lock(&slot->stats_lock);
    *output = slot->stats;
    pthread_mutex_unlock(&slot->stats_lock);
    bounded_copy(output->name, sizeof(output->name), slot->collector.name);
    output->scope = slot->collector.scope;
    output->tier = slot->collector.tier;
    output->interval_seconds = slot->collector.interval_seconds;
    output->stale_after_seconds = slot->collector.stale_after_seconds;
    output->busy = atomic_load(&slot->busy);
    uint64_t stale_ms = (uint64_t)output->stale_after_seconds * 1000U;
    output->stale = output->attempts > 0U &&
        (output->last_success_monotonic_ms == 0U ||
         output->last_success_monotonic_ms > now ||
         (stale_ms > 0U &&
          now - output->last_success_monotonic_ms > stale_ms));
}

size_t system_health_collector_stats_copy(
    system_health_collector_stats_t *stats, size_t capacity) {
    if (!stats || capacity == 0U) return 0U;
    pthread_mutex_lock(&lifecycle_lock);
    system_health_runtime_t *state = runtime;
    if (!state) {
        pthread_mutex_unlock(&lifecycle_lock);
        return 0U;
    }
    uint64_t now = monotonic_ms();
    size_t available = state->collector_count +
        (system_health_provider_registry_count() > 0U ? 1U : 0U);
    size_t count = available < capacity ? available : capacity;
    size_t copied = 0U;
    for (; copied < state->collector_count && copied < count; ++copied)
        copy_collector_stats(&state->collectors[copied], now, &stats[copied]);
    if (copied < count) {
        copy_collector_stats(&state->provider_slot, now, &stats[copied]);
        copied++;
    }
    pthread_mutex_unlock(&lifecycle_lock);
    return copied;
}

void system_health_shutdown(void) {
    pthread_mutex_lock(&lifecycle_lock);
    system_health_runtime_t *state = runtime;
    if (!state) {
        disabled_initialized = false;
        pthread_mutex_unlock(&lifecycle_lock);
        return;
    }
    runtime = NULL;
    atomic_store(&state->running, false);
    for (size_t tier = 0; tier < SYSTEM_HEALTH_TIER_COUNT; ++tier) {
        tier_worker_t *worker = &state->workers[tier];
        pthread_mutex_lock(&worker->lock);
        worker->requested = true;
        pthread_cond_signal(&worker->wake);
        pthread_mutex_unlock(&worker->lock);
    }
    pthread_mutex_unlock(&lifecycle_lock);

    for (size_t tier = 0; tier < SYSTEM_HEALTH_TIER_COUNT; ++tier) {
        tier_worker_t *worker = &state->workers[tier];
        if (worker->started) pthread_join(worker->thread, NULL);
    }
    for (size_t index = 0; index < state->collector_count; ++index) {
        if (state->collectors[index].collector.destroy)
            state->collectors[index].collector.destroy(
                state->collectors[index].collector.state);
        pthread_mutex_destroy(&state->collectors[index].stats_lock);
    }
    system_health_provider_registry_destroy();
    pthread_mutex_destroy(&state->provider_slot.stats_lock);
    for (size_t tier = 0; tier < SYSTEM_HEALTH_TIER_COUNT; ++tier) {
        pthread_cond_destroy(&state->workers[tier].wake);
        pthread_mutex_destroy(&state->workers[tier].lock);
    }
    pthread_mutex_destroy(&state->lock);
    free(state->ring);
    free(state);
    health_helper_reap_abandoned();
}
