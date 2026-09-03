#define _GNU_SOURCE
#define _POSIX_C_SOURCE 200809L

#include <arpa/inet.h>
#include <errno.h>
#include <fcntl.h>
#include <malloc.h>
#include <math.h>
#include <pthread.h>
#include <sched.h>
#include <stdatomic.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/stat.h>
#include <time.h>
#include <unistd.h>

#include <sqlite3.h>

#include "database/db_core.h"
#include "database/db_system_health_incidents.h"
#include "telemetry/collectors/linux_cgroup.h"
#include "telemetry/health_helper_runner.h"
#include "telemetry/recording_io_metrics.h"
#include "telemetry/system_health.h"
#include "telemetry/system_health_evaluator.h"

#define T24_WALL_BASE 1700000000000LL
#define T24_MAX_EXPECTED 64U
#define T24_MAX_TRANSITIONS 128U

static int failures;

#define CHECK(expr, ...) do {                                                  \
    if (!(expr)) {                                                             \
        fprintf(stderr, "FAIL:%s:%d: ", __FILE__, __LINE__);                  \
        fprintf(stderr, __VA_ARGS__);                                          \
        fputc('\n', stderr);                                                   \
        failures++;                                                            \
    }                                                                          \
} while (0)

static uint64_t clock_ns(clockid_t id) {
    struct timespec value;
    if (clock_gettime(id, &value) != 0) return 0U;
    return (uint64_t)value.tv_sec * 1000000000ULL + (uint64_t)value.tv_nsec;
}

static uint64_t monotonic_ms(void) {
    return clock_ns(CLOCK_MONOTONIC) / 1000000ULL;
}

static size_t heap_bytes(void) {
#if defined(__GLIBC__)
    return (size_t)mallinfo2().uordblks;
#else
    return 0U;
#endif
}

typedef struct {
    atomic_uint_fast64_t calls;
    atomic_uint_fast64_t probe_cpu_ns;
} fake_collector_state_t;

static int fake_collect(void *opaque,
                        const system_health_collect_context_t *context,
                        system_health_observation_sink_t *sink) {
    fake_collector_state_t *state = opaque;
    uint64_t started = clock_ns(CLOCK_THREAD_CPUTIME_ID);
    uint64_t generation = atomic_fetch_add_explicit(
        &state->calls, 1U, memory_order_relaxed) + 1U;
    for (unsigned index = 0; index < 2U; ++index) {
        system_health_observation_t observation;
        memset(&observation, 0, sizeof(observation));
        snprintf(observation.metric, sizeof(observation.metric),
                 "benchmark.generation.%u", index);
        snprintf(observation.resource_id, sizeof(observation.resource_id),
                 "host");
        observation.scope = SYSTEM_HEALTH_SCOPE_HOST;
        observation.sampled_monotonic_ms = context->monotonic_ms;
        observation.observed_wall_time_ms = context->wall_time_ms;
        system_health_observation_set_available(
            &observation, (double)(generation * (index + 1U)),
            SYSTEM_HEALTH_UNIT_COUNT);
        if (!system_health_observation_sink_append(sink, &observation)) break;
    }
    atomic_fetch_add_explicit(&state->probe_cpu_ns,
                              clock_ns(CLOCK_THREAD_CPUTIME_ID) - started,
                              memory_order_relaxed);
    return 0;
}

static system_health_options_t fake_options(void) {
    system_health_options_t options;
    system_health_options_defaults(&options);
    options.register_builtin_collectors = false;
    snprintf(options.proc_root, sizeof(options.proc_root), "/t24/fake/proc");
    snprintf(options.sys_root, sizeof(options.sys_root), "/t24/fake/sys");
    snprintf(options.cgroup_root, sizeof(options.cgroup_root),
             "/t24/fake/cgroup");
    snprintf(options.root_path, sizeof(options.root_path), "/t24/fake/root");
    snprintf(options.recording_path, sizeof(options.recording_path),
             "/t24/fake/recording");
    for (size_t index = 0; index < SYSTEM_HEALTH_TIER_COUNT; ++index) {
        options.tier_interval_ms[index] = 5U;
        options.collector_deadline_ms[index] = 100U;
    }
    return options;
}

static system_health_collector_t fake_descriptor(fake_collector_state_t *state) {
    system_health_collector_t collector;
    memset(&collector, 0, sizeof(collector));
    snprintf(collector.name, sizeof(collector.name), "t24-deterministic");
    collector.scope = SYSTEM_HEALTH_SCOPE_HOST;
    collector.tier = SYSTEM_HEALTH_TIER_FAST;
    collector.interval_seconds = 10U;
    collector.stale_after_seconds = 30U;
    collector.state = state;
    collector.collect = fake_collect;
    return collector;
}

static int run_resource_gate(unsigned samples, size_t heap_limit,
                             double cpu_limit) {
    fake_collector_state_t state = {0};
    system_health_options_t options = fake_options();
    CHECK(system_health_init(&options) == 0, "sampler init failed");
    system_health_collector_t collector = fake_descriptor(&state);
    CHECK(system_health_register_collector(&collector),
          "collector registration failed");
    CHECK(system_health_collect_tier(SYSTEM_HEALTH_TIER_FAST) == 0,
          "warm-up collection failed");
    system_health_snapshot_t warm;
    CHECK(system_health_snapshot_copy(&warm), "warm-up snapshot unavailable");

    size_t heap_before = heap_bytes();
    uint64_t cpu_before = clock_ns(CLOCK_PROCESS_CPUTIME_ID);
    uint64_t probe_before = atomic_load_explicit(&state.probe_cpu_ns,
                                                 memory_order_relaxed);
    for (unsigned index = 0; index < samples; ++index) {
        CHECK(system_health_collect_tier(SYSTEM_HEALTH_TIER_FAST) == 0,
              "sample %u collection failed", index);
        system_health_snapshot_t snapshot;
        CHECK(system_health_snapshot_copy(&snapshot),
              "sample %u snapshot unavailable", index);
        CHECK(snapshot.observation_count == 2U,
              "sample %u count=%zu", index, snapshot.observation_count);
        if (snapshot.observation_count == 2U) {
            CHECK(snapshot.observations[1].value ==
                      snapshot.observations[0].value * 2.0,
                  "sample %u is a torn generation", index);
        }
    }
    uint64_t cpu_after = clock_ns(CLOCK_PROCESS_CPUTIME_ID);
    uint64_t probe_after = atomic_load_explicit(&state.probe_cpu_ns,
                                                memory_order_relaxed);
    size_t heap_after = heap_bytes();
    size_t heap_growth = heap_after > heap_before ? heap_after - heap_before : 0U;
    uint64_t total_cpu = cpu_after - cpu_before;
    uint64_t probe_cpu = probe_after - probe_before;
    uint64_t sampler_cpu = total_cpu > probe_cpu ? total_cpu - probe_cpu : 0U;
    double simulated_ns = (double)samples * 10.0 * 1000000000.0;
    double cpu_ratio = simulated_ns > 0.0 ? (double)sampler_cpu / simulated_ns : 0.0;

    system_health_stats_t stats;
    system_health_get_stats(&stats);
    CHECK(stats.ring_allocated, "enabled sampler did not allocate bounded ring");
    CHECK(stats.generations_completed >= samples,
          "only %llu generations completed", (unsigned long long)stats.generations_completed);
    CHECK(heap_growth <= heap_limit, "steady heap growth %zu exceeds %zu bytes",
          heap_growth, heap_limit);
    CHECK(cpu_ratio <= cpu_limit,
          "non-probe CPU ratio %.9f exceeds %.9f", cpu_ratio, cpu_limit);
    printf("resource: samples=%u heap_growth_bytes=%zu non_probe_cpu_ratio=%.9f\n",
           samples, heap_growth, cpu_ratio);
    system_health_shutdown();
    return failures ? -1 : 0;
}

typedef struct {
    atomic_bool stop;
    atomic_uint_fast64_t snapshots;
    atomic_uint_fast64_t torn;
    atomic_uint_fast64_t sentinel_ticks;
    atomic_uint_fast64_t sentinel_max_gap_ms;
} stress_state_t;

static void *stress_collect(void *opaque) {
    stress_state_t *state = opaque;
    for (unsigned index = 0; index < 1200U; ++index)
        (void)system_health_collect_tier(SYSTEM_HEALTH_TIER_FAST);
    atomic_store_explicit(&state->stop, true, memory_order_release);
    return NULL;
}

static void *stress_read(void *opaque) {
    stress_state_t *state = opaque;
    uint64_t last_sequence = 0U;
    while (!atomic_load_explicit(&state->stop, memory_order_acquire)) {
        system_health_snapshot_t snapshot;
        if (!system_health_snapshot_copy(&snapshot)) continue;
        bool bad = snapshot.sequence < last_sequence ||
                   snapshot.observation_count != 2U;
        if (!bad && snapshot.observation_count == 2U)
            bad = snapshot.observations[1].value !=
                  snapshot.observations[0].value * 2.0;
        if (bad) atomic_fetch_add_explicit(&state->torn, 1U,
                                           memory_order_relaxed);
        last_sequence = snapshot.sequence;
        atomic_fetch_add_explicit(&state->snapshots, 1U,
                                  memory_order_relaxed);
    }
    return NULL;
}

static void *stress_sentinel(void *opaque) {
    stress_state_t *state = opaque;
    uint64_t prior = monotonic_ms();
    while (!atomic_load_explicit(&state->stop, memory_order_acquire)) {
        struct timespec delay = {.tv_nsec = 1000000L};
        nanosleep(&delay, NULL);
        uint64_t now = monotonic_ms();
        uint64_t gap = now - prior;
        prior = now;
        uint64_t maximum = atomic_load_explicit(&state->sentinel_max_gap_ms,
                                                memory_order_relaxed);
        while (gap > maximum && !atomic_compare_exchange_weak_explicit(
                   &state->sentinel_max_gap_ms, &maximum, gap,
                   memory_order_relaxed, memory_order_relaxed)) {}
        recording_io_report_failure(RECORDING_IO_RESOURCE_RECORDING,
                                    RECORDING_IO_OPERATION_PACKET, EIO);
        atomic_fetch_add_explicit(&state->sentinel_ticks, 1U,
                                  memory_order_relaxed);
    }
    return NULL;
}

static void *stress_helper(void *opaque) {
    stress_state_t *state = opaque;
    char *argv[] = {(char *)"/bin/sleep", (char *)"2", NULL};
    while (!atomic_load_explicit(&state->stop, memory_order_acquire)) {
        health_helper_request_t request = {
            .program = "/bin/sleep", .argv = argv, .timeout_ms = 25U,
            .terminate_grace_ms = 10U, .output_limit = 32U};
        health_helper_result_t result;
        (void)health_helper_run(&request, &result);
    }
    return NULL;
}

static void *stress_broker(void *opaque) {
    stress_state_t *state = opaque;
    while (!atomic_load_explicit(&state->stop, memory_order_acquire)) {
        int fd = socket(AF_INET, SOCK_STREAM | SOCK_NONBLOCK, 0);
        if (fd >= 0) {
            struct sockaddr_in address = {.sin_family = AF_INET,
                                          .sin_port = htons(1)};
            address.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
            (void)connect(fd, (struct sockaddr *)&address, sizeof(address));
            close(fd);
        }
        sched_yield();
    }
    return NULL;
}

static int run_stress(void) {
    fake_collector_state_t collector_state = {0};
    stress_state_t stress = {0};
    system_health_options_t options = fake_options();
    CHECK(system_health_init(&options) == 0, "stress sampler init failed");
    system_health_collector_t collector = fake_descriptor(&collector_state);
    CHECK(system_health_register_collector(&collector),
          "stress collector registration failed");
    CHECK(system_health_collect_tier(SYSTEM_HEALTH_TIER_FAST) == 0,
          "stress warmup failed");
    recording_io_metrics_snapshot_t before, after;
    recording_io_metrics_snapshot(&before);

    pthread_t writer, readers[4], sentinel, helper, broker;
    CHECK(pthread_create(&writer, NULL, stress_collect, &stress) == 0,
          "writer thread failed");
    for (size_t index = 0; index < 4U; ++index)
        CHECK(pthread_create(&readers[index], NULL, stress_read, &stress) == 0,
              "reader thread %zu failed", index);
    CHECK(pthread_create(&sentinel, NULL, stress_sentinel, &stress) == 0,
          "sentinel thread failed");
    CHECK(pthread_create(&helper, NULL, stress_helper, &stress) == 0,
          "helper thread failed");
    CHECK(pthread_create(&broker, NULL, stress_broker, &stress) == 0,
          "broker thread failed");
    pthread_join(writer, NULL);
    for (size_t index = 0; index < 4U; ++index) pthread_join(readers[index], NULL);
    pthread_join(sentinel, NULL);
    pthread_join(helper, NULL);
    pthread_join(broker, NULL);
    recording_io_metrics_snapshot(&after);
    uint64_t prior = before.reason_totals[RECORDING_IO_RESOURCE_RECORDING]
                                         [RECORDING_IO_REASON_IO];
    uint64_t current = after.reason_totals[RECORDING_IO_RESOURCE_RECORDING]
                                           [RECORDING_IO_REASON_IO];
    CHECK(atomic_load(&stress.snapshots) > 100U, "scrape readers made no progress");
    CHECK(atomic_load(&stress.torn) == 0U, "immutable readers observed torn data");
    CHECK(atomic_load(&stress.sentinel_ticks) > 0U && current > prior,
          "recording sentinel did not progress");
    CHECK(atomic_load(&stress.sentinel_max_gap_ms) < 250U,
          "recording sentinel stalled for %llu ms",
          (unsigned long long)atomic_load(&stress.sentinel_max_gap_ms));
    CHECK(health_helper_abandoned_count() <= HEALTH_HELPER_ABANDONED_MAX,
          "helper abandoned bound exceeded");
    printf("stress: snapshots=%llu sentinel_ticks=%llu max_gap_ms=%llu\n",
           (unsigned long long)atomic_load(&stress.snapshots),
           (unsigned long long)atomic_load(&stress.sentinel_ticks),
           (unsigned long long)atomic_load(&stress.sentinel_max_gap_ms));
    system_health_shutdown();
    return failures ? -1 : 0;
}

typedef struct {
    system_health_transition_t items[T24_MAX_TRANSITIONS];
    size_t count;
} transition_log_t;

static void capture_transition(const system_health_transition_t *transition,
                               void *opaque) {
    transition_log_t *log = opaque;
    if (log->count < T24_MAX_TRANSITIONS) log->items[log->count++] = *transition;
}

static void snapshot_begin(system_health_snapshot_t *snapshot, uint64_t sequence,
                           uint64_t elapsed_ms) {
    memset(snapshot, 0, sizeof(*snapshot));
    snapshot->sequence = sequence;
    snapshot->completed_monotonic_ms = elapsed_ms + 1U;
    snapshot->completed_wall_time_ms = T24_WALL_BASE + (int64_t)elapsed_ms;
}

static void snapshot_add(system_health_snapshot_t *snapshot, const char *metric,
                         const char *resource, system_health_scope_t scope,
                         system_health_capability_t capability, double value,
                         system_health_unit_t unit) {
    system_health_observation_t observation;
    memset(&observation, 0, sizeof(observation));
    snprintf(observation.metric, sizeof(observation.metric), "%s", metric);
    snprintf(observation.resource_id, sizeof(observation.resource_id), "%s",
             resource);
    observation.scope = scope;
    observation.sampled_monotonic_ms = snapshot->completed_monotonic_ms;
    observation.observed_wall_time_ms = snapshot->completed_wall_time_ms;
    if (capability == SYSTEM_HEALTH_CAPABILITY_AVAILABLE)
        system_health_observation_set_available(&observation, value, unit);
    else
        system_health_observation_set_unavailable(&observation, capability);
    CHECK(system_health_snapshot_append(snapshot, &observation),
          "replay snapshot capacity exceeded");
}

static system_health_scope_t parse_scope(const char *text) {
    for (int value = 0; value < SYSTEM_HEALTH_SCOPE_COUNT; ++value)
        if (strcmp(text, system_health_scope_name((system_health_scope_t)value)) == 0)
            return (system_health_scope_t)value;
    return SYSTEM_HEALTH_SCOPE_HOST;
}

static system_health_capability_t parse_capability(const char *text) {
    for (int value = 0; value < SYSTEM_HEALTH_CAPABILITY_COUNT; ++value)
        if (strcmp(text, system_health_capability_name(
                           (system_health_capability_t)value)) == 0)
            return (system_health_capability_t)value;
    return SYSTEM_HEALTH_CAPABILITY_ERROR;
}

static system_health_unit_t parse_unit(const char *text) {
    for (int value = 0; value <= SYSTEM_HEALTH_UNIT_BOOLEAN; ++value)
        if (strcmp(text, system_health_unit_name((system_health_unit_t)value)) == 0)
            return (system_health_unit_t)value;
    return SYSTEM_HEALTH_UNIT_NONE;
}

static const char *action_name(system_health_incident_action_t action) {
    switch (action) {
        case SYSTEM_HEALTH_INCIDENT_OPEN: return "open";
        case SYSTEM_HEALTH_INCIDENT_ESCALATE: return "escalate";
        case SYSTEM_HEALTH_INCIDENT_MATERIAL_CHANGE: return "material";
        case SYSTEM_HEALTH_INCIDENT_RECOVER: return "recover";
        case SYSTEM_HEALTH_INCIDENT_ONE_SHOT: return "one_shot";
    }
    return "unknown";
}

static int run_replay(const char *path) {
    FILE *stream = fopen(path, "r");
    if (!stream) {
        fprintf(stderr, "cannot open replay %s: %s\n", path, strerror(errno));
        return -1;
    }
    char expected[T24_MAX_EXPECTED][128];
    size_t expected_count = 0U;
    system_health_policy_settings_t settings;
    system_health_policy_settings_defaults(&settings);
    system_health_policy_t policy;
    char error[SYSTEM_HEALTH_POLICY_ERROR_LENGTH];
    CHECK(system_health_policy_build(&settings, NULL, &policy, NULL, 0U, error) == 0,
          "policy build failed: %s", error);
    for (int index = 0; index < SYSTEM_HEALTH_CONDITION_COUNT; ++index) {
        policy.conditions[index].warning_for_seconds = 20U;
        policy.conditions[index].critical_for_seconds = 20U;
        policy.conditions[index].recovery_for_seconds = 20U;
    }
    transition_log_t log = {0};
    system_health_evaluator_config_t config;
    system_health_evaluator_config_defaults(&config, &policy);
    snprintf(config.boot_id, sizeof(config.boot_id), "t24-boot");
    snprintf(config.run_id, sizeof(config.run_id),
             "24242424-2424-4242-8242-242424242424");
    config.material_change_debounce_ms = 10000U;
    config.material_change_ratio = .10;
    config.transition_sink = capture_transition;
    config.transition_sink_context = &log;
    system_health_evaluator_t *evaluator = system_health_evaluator_create(&config);
    CHECK(evaluator != NULL, "evaluator allocation failed");
    if (!evaluator) { fclose(stream); return -1; }

    char line[1024];
    system_health_snapshot_t snapshot;
    system_health_evaluation_context_t context = {0};
    uint64_t current_ms = UINT64_MAX;
    uint64_t sequence = 0U;
    bool have_rows = false;
    while (fgets(line, sizeof(line), stream)) {
        line[strcspn(line, "\r\n")] = '\0';
        if (strncmp(line, "# expect=", 9U) == 0) {
            char *save = NULL;
            char *item = strtok_r(line + 9U, ";", &save);
            while (item && expected_count < T24_MAX_EXPECTED) {
                snprintf(expected[expected_count++], sizeof(expected[0]), "%s", item);
                item = strtok_r(NULL, ";", &save);
            }
            continue;
        }
        if (line[0] == '#' || line[0] == '\0' || strncmp(line, "elapsed_ms,", 11U) == 0)
            continue;
        char *fields[9];
        size_t count = 0U;
        char *save = NULL;
        for (char *field = strtok_r(line, ",", &save); field && count < 9U;
             field = strtok_r(NULL, ",", &save)) fields[count++] = field;
        CHECK(count == 9U, "%s: malformed replay row", path);
        if (count != 9U) continue;
        uint64_t elapsed = strtoull(fields[0], NULL, 10);
        if (current_ms != UINT64_MAX && elapsed != current_ms) {
            CHECK(system_health_evaluator_evaluate(evaluator, &snapshot, &context) == 0,
                  "%s: evaluator failed", path);
            have_rows = false;
        }
        if (!have_rows) {
            current_ms = elapsed;
            snapshot_begin(&snapshot, ++sequence, elapsed);
            memset(&context, 0, sizeof(context));
            context.service_degraded_known = true;
            context.service_degraded = strcmp(fields[7], "1") == 0;
            context.recording_expected_known = true;
            context.recording_expected = strcmp(fields[8], "1") == 0;
            have_rows = true;
        }
        snapshot_add(&snapshot, fields[1], fields[2], parse_scope(fields[3]),
                     parse_capability(fields[4]), strtod(fields[5], NULL),
                     parse_unit(fields[6]));
    }
    if (have_rows)
        CHECK(system_health_evaluator_evaluate(evaluator, &snapshot, &context) == 0,
              "%s: final evaluator call failed", path);
    fclose(stream);

    CHECK(log.count == expected_count,
          "%s: got %zu transitions, expected %zu", path, log.count, expected_count);
    if (log.count != expected_count) {
        fprintf(stderr, "actual transitions:");
        for (size_t index = 0; index < log.count; ++index) {
            fprintf(stderr, "%s%s:%s", index == 0U ? " " : ";",
                    system_health_condition_code(log.items[index].condition),
                    action_name(log.items[index].action));
        }
        fputc('\n', stderr);
    }
    size_t compare = log.count < expected_count ? log.count : expected_count;
    for (size_t index = 0; index < compare; ++index) {
        char actual[128];
        snprintf(actual, sizeof(actual), "%s:%s",
                 system_health_condition_code(log.items[index].condition),
                 action_name(log.items[index].action));
        CHECK(strcmp(actual, expected[index]) == 0,
              "%s transition %zu got %s expected %s", path, index,
              actual, expected[index]);
    }
    printf("replay: %s rows=%llu transitions=%zu\n", path,
           (unsigned long long)sequence, log.count);
    system_health_evaluator_destroy(evaluator);
    return failures ? -1 : 0;
}

static db_system_health_result_t persist_to_database(
    system_health_incident_action_t action,
    const system_health_incident_signal_t *signal,
    system_health_incident_record_t *incident, void *opaque) {
    (void)opaque;
    return db_system_health_incident_apply(action, signal, incident);
}

static int sql_exec(sqlite3 *db, const char *sql) {
    char *message = NULL;
    int rc = sqlite3_exec(db, sql, NULL, NULL, &message);
    if (rc != SQLITE_OK && rc != SQLITE_FULL)
        fprintf(stderr, "sqlite: %s (%s)\n", sql, message ? message : "unknown");
    sqlite3_free(message);
    return rc;
}

static int run_sqlite_full(void) {
    char path[] = "/tmp/lightnvr-t24-XXXXXX";
    int fd = mkstemp(path);
    CHECK(fd >= 0, "mkstemp failed: %s", strerror(errno));
    if (fd < 0) return -1;
    close(fd);
    unlink(path);
    CHECK(init_database(path) == 0, "temporary database init failed");
    sqlite3 *db = get_db_handle();
    CHECK(db != NULL, "database handle missing");
    if (!db) { shutdown_database(); unlink(path); return -1; }
    (void)sql_exec(db, "PRAGMA journal_mode=DELETE");
    CHECK(sql_exec(db, "CREATE TABLE t24_fill(data BLOB)") == SQLITE_OK,
          "fill table creation failed");
    sqlite3_int64 pages = 0;
    sqlite3_stmt *statement = NULL;
    if (sqlite3_prepare_v2(db, "PRAGMA page_count", -1, &statement, NULL) == SQLITE_OK &&
        sqlite3_step(statement) == SQLITE_ROW) pages = sqlite3_column_int64(statement, 0);
    sqlite3_finalize(statement);
    char pragma[96];
    snprintf(pragma, sizeof(pragma), "PRAGMA max_page_count=%lld",
             (long long)(pages + 6));
    CHECK(sql_exec(db, pragma) == SQLITE_OK, "max_page_count setup failed");
    int full_rc = SQLITE_OK;
    for (unsigned index = 0; index < 64U && full_rc == SQLITE_OK; ++index)
        full_rc = sql_exec(db, "INSERT INTO t24_fill VALUES(zeroblob(8192))");
    CHECK(full_rc == SQLITE_FULL, "failed to induce real SQLITE_FULL (rc=%d)", full_rc);

    /*
     * Filling an unrelated table to SQLITE_FULL is insufficient: the incident
     * and transition root pages can still have room for a small transaction.
     * Consume that room through the real repository until the exact health
     * persistence path reports the full database.
     */
    db_system_health_result_t health_full = DB_SYSTEM_HEALTH_OK;
    unsigned health_fill_count = 0U;
    while (health_fill_count < 512U) {
        system_health_incident_signal_t signal;
        memset(&signal, 0, sizeof(signal));
        signal.condition = SYSTEM_HEALTH_CONDITION_FILESYSTEM_READ_ONLY;
        snprintf(signal.subject, sizeof(signal.subject), "t24-fill-%u",
                 health_fill_count);
        signal.scope = SYSTEM_HEALTH_SCOPE_FILESYSTEM;
        signal.state = SYSTEM_HEALTH_STATE_OPEN;
        signal.severity = SYSTEM_HEALTH_SEVERITY_CRITICAL;
        signal.observed_at_ms = T24_WALL_BASE + (int64_t)health_fill_count;
        snprintf(signal.observation_json, sizeof(signal.observation_json),
                 "{\"metric\":\"filesystem.read_only\","
                 "\"resource\":\"t24-fill-%u\",\"value\":1}",
                 health_fill_count);
        signal.reconciliation = SYSTEM_HEALTH_RECONCILIATION_NONE;
        snprintf(signal.boot_id, sizeof(signal.boot_id), "t24-sqlite-boot");
        snprintf(signal.run_id, sizeof(signal.run_id),
                 "45454545-4545-4454-8454-454545454545");
        system_health_incident_record_t incident;
        health_full = db_system_health_incident_apply(
            SYSTEM_HEALTH_INCIDENT_OPEN, &signal, &incident);
        if (health_full != DB_SYSTEM_HEALTH_OK) break;
        health_fill_count++;
    }
    CHECK(health_full == DB_SYSTEM_HEALTH_ERROR,
          "health persistence did not reach real SQLITE_FULL (result=%d, fills=%u)",
          health_full, health_fill_count);

    system_health_policy_settings_t settings;
    system_health_policy_settings_defaults(&settings);
    system_health_policy_t policy;
    char error[SYSTEM_HEALTH_POLICY_ERROR_LENGTH];
    CHECK(system_health_policy_build(&settings, NULL, &policy, NULL, 0U, error) == 0,
          "sqlite test policy failed");
    transition_log_t log = {0};
    system_health_evaluator_config_t config;
    system_health_evaluator_config_defaults(&config, &policy);
    snprintf(config.boot_id, sizeof(config.boot_id), "t24-sqlite-boot");
    snprintf(config.run_id, sizeof(config.run_id),
             "34343434-3434-4343-8343-343434343434");
    config.retry_initial_ms = 100U;
    config.retry_max_ms = 200U;
    config.persist = persist_to_database;
    config.transition_sink = capture_transition;
    config.transition_sink_context = &log;
    system_health_evaluator_t *evaluator = system_health_evaluator_create(&config);
    system_health_snapshot_t snapshot;
    snapshot_begin(&snapshot, 1U, 1000U);
    snapshot_add(&snapshot, "filesystem.read_only", "recording",
                 SYSTEM_HEALTH_SCOPE_FILESYSTEM,
                 SYSTEM_HEALTH_CAPABILITY_AVAILABLE, 1.0,
                 SYSTEM_HEALTH_UNIT_BOOLEAN);
    CHECK(system_health_evaluator_evaluate(evaluator, &snapshot, NULL) == 0,
          "SQLITE_FULL evaluate failed");
    system_health_evaluator_stats_t stats;
    system_health_evaluator_get_stats(evaluator, &stats);
    CHECK(stats.pending_persistence == 1U && stats.persistence_failures == 1U,
          "SQLITE_FULL did not enter bounded backoff");
    snapshot_begin(&snapshot, 2U, 1050U);
    snapshot_add(&snapshot, "filesystem.read_only", "recording",
                 SYSTEM_HEALTH_SCOPE_FILESYSTEM,
                 SYSTEM_HEALTH_CAPABILITY_AVAILABLE, 1.0,
                 SYSTEM_HEALTH_UNIT_BOOLEAN);
    (void)system_health_evaluator_evaluate(evaluator, &snapshot, NULL);
    system_health_evaluator_get_stats(evaluator, &stats);
    CHECK(stats.persistence_retries == 0U, "retry ignored backoff");

    CHECK(sql_exec(db, "PRAGMA max_page_count=1073741823") == SQLITE_OK,
          "failed to lift SQLITE_FULL limit");
    CHECK(sql_exec(db, "DROP TABLE t24_fill") == SQLITE_OK,
          "failed to free SQLite pages");
    snapshot_begin(&snapshot, 3U, 1201U);
    snapshot_add(&snapshot, "filesystem.read_only", "recording",
                 SYSTEM_HEALTH_SCOPE_FILESYSTEM,
                 SYSTEM_HEALTH_CAPABILITY_AVAILABLE, 1.0,
                 SYSTEM_HEALTH_UNIT_BOOLEAN);
    CHECK(system_health_evaluator_evaluate(evaluator, &snapshot, NULL) == 0,
          "SQLite recovery evaluate failed");
    system_health_evaluator_get_stats(evaluator, &stats);
    CHECK(stats.pending_persistence == 0U && stats.persistence_retries == 1U,
          "SQLite recovery did not drain one retry");
    system_health_incident_transition_t pending[4];
    int pending_count = db_system_health_transition_list_pending(0, pending, 4);
    CHECK(pending_count == 1,
          "durable outbox handoff should contain one pending transition, got %d",
          pending_count);
    printf("sqlite-full: failures=%llu retries=%llu pending_outbox=%d\n",
           (unsigned long long)stats.persistence_failures,
           (unsigned long long)stats.persistence_retries, pending_count);
    system_health_evaluator_destroy(evaluator);
    shutdown_database();
    unlink(path);
    return failures ? -1 : 0;
}

static int run_docker_cgroup(void) {
    linux_cgroup_state_t state;
    linux_cgroup_state_init(&state);
    system_health_collector_t collector;
    CHECK(linux_cgroup_collector_init(&collector, &state),
          "cgroup collector init failed");
    system_health_observation_t observations[64];
    system_health_observation_sink_t sink = {
        .items = observations, .capacity = 64U};
    system_health_collect_context_t context = {
        .monotonic_ms = monotonic_ms(),
        .wall_time_ms = T24_WALL_BASE,
        .proc_root = "/proc", .sys_root = "/sys",
        .cgroup_root = "/sys/fs/cgroup"};
    CHECK(collector.collect(collector.state, &context, &sink) == 0,
          "live cgroup collection failed");
    size_t container = 0U, available = 0U;
    for (size_t index = 0; index < sink.count; ++index) {
        if (observations[index].scope == SYSTEM_HEALTH_SCOPE_CONTAINER) container++;
        if (observations[index].value_valid) available++;
    }
    CHECK(container > 0U && available > 0U,
          "no usable container-scoped cgroup observations (%zu/%zu)",
          container, available);
    printf("docker-cgroup: observations=%zu available=%zu\n", container, available);
    if (collector.destroy) collector.destroy(collector.state);
    return failures ? -1 : 0;
}

static int run_restart_classification(void) {
    system_health_process_run_t previous = {0};
    snprintf(previous.boot_id, sizeof(previous.boot_id), "boot-a");
    CHECK(system_health_classify_previous_run(&previous, false, "boot-a") ==
              SYSTEM_HEALTH_RESTART_NONE,
          "missing previous run fabricated a restart");
    previous.clean_close = true;
    CHECK(system_health_classify_previous_run(&previous, true, "boot-a") ==
              SYSTEM_HEALTH_RESTART_NONE,
          "clean previous run fabricated a restart");
    previous.clean_close = false;
    CHECK(system_health_classify_previous_run(&previous, true, "boot-a") ==
              SYSTEM_HEALTH_RESTART_PROCESS,
          "unclean same-boot run was not classified as process restart");
    CHECK(system_health_classify_previous_run(&previous, true, "boot-b") ==
              SYSTEM_HEALTH_RESTART_HOST,
          "unclean changed-boot run was not classified as host restart");
    puts("restart-classification: none/clean/process/host cases verified");
    return failures ? -1 : 0;
}

static void usage(const char *program) {
    fprintf(stderr,
            "usage: %s --samples N --heap-limit-bytes N --cpu-ratio-limit R\n"
            "       %s --stress | --sqlite-full | --replay FILE | --docker-cgroup\n"
            "       %s --restart-classification\n",
            program, program, program);
}

int main(int argc, char **argv) {
    unsigned samples = 0U;
    size_t heap_limit = 0U;
    double cpu_limit = -1.0;
    const char *replay = NULL;
    bool stress = false, sqlite_full = false, docker_cgroup = false;
    bool restart_classification = false;
    for (int index = 1; index < argc; ++index) {
        if (strcmp(argv[index], "--samples") == 0 && index + 1 < argc)
            samples = (unsigned)strtoul(argv[++index], NULL, 10);
        else if (strcmp(argv[index], "--heap-limit-bytes") == 0 && index + 1 < argc)
            heap_limit = (size_t)strtoull(argv[++index], NULL, 10);
        else if (strcmp(argv[index], "--cpu-ratio-limit") == 0 && index + 1 < argc)
            cpu_limit = strtod(argv[++index], NULL);
        else if (strcmp(argv[index], "--replay") == 0 && index + 1 < argc)
            replay = argv[++index];
        else if (strcmp(argv[index], "--stress") == 0) stress = true;
        else if (strcmp(argv[index], "--sqlite-full") == 0) sqlite_full = true;
        else if (strcmp(argv[index], "--docker-cgroup") == 0) docker_cgroup = true;
        else if (strcmp(argv[index], "--restart-classification") == 0)
            restart_classification = true;
        else { usage(argv[0]); return 2; }
    }
    int modes = replay != NULL ? 1 : 0;
    modes += stress ? 1 : 0;
    modes += sqlite_full ? 1 : 0;
    modes += docker_cgroup ? 1 : 0;
    modes += restart_classification ? 1 : 0;
    modes += samples > 0U ? 1 : 0;
    if (modes != 1 || (samples > 0U && (heap_limit == 0U || cpu_limit < 0.0))) {
        usage(argv[0]);
        return 2;
    }
    int rc = replay ? run_replay(replay) : stress ? run_stress() :
             sqlite_full ? run_sqlite_full() : docker_cgroup ? run_docker_cgroup() :
             restart_classification ? run_restart_classification() :
             run_resource_gate(samples, heap_limit, cpu_limit);
    if (rc == 0 && failures == 0) puts("PASS");
    return rc == 0 && failures == 0 ? 0 : 1;
}
