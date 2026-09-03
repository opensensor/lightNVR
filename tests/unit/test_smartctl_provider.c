#define _POSIX_C_SOURCE 200809L

#include "unity.h"

#include <signal.h>
#include <sqlite3.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/sysmacros.h>
#include <time.h>
#include <unistd.h>

#include "database/db_core.h"
#include "database/db_storage_targets.h"
#include "telemetry/health_helper_runner.h"
#include "telemetry/providers/builtin_providers.h"
#include "telemetry/providers/smartctl_provider.h"

#ifndef TEST_SMARTCTL_FIXTURE_DIR
#define TEST_SMARTCTL_FIXTURE_DIR "tests/fixtures/system_health/smartctl"
#endif

#define TEST_DB_PATH "/tmp/lightnvr_unit_smartctl_provider.db"

static char target_root[] = "/tmp/lightnvr-smartctl-target-XXXXXX";
static char target_uuid[LIGHTNVR_UUID_STRING_SIZE];
static const char *fake_fixture;
static health_helper_outcome_t fake_outcome;
static int fake_exit_status;
static bool fake_truncated;
static int fake_return_status;
static bool fake_request_valid;
static unsigned int fake_run_calls;
static unsigned int fake_wake_calls;

static void fixture_path(char *output, size_t capacity, const char *name) {
    snprintf(output, capacity, "%s/%s", TEST_SMARTCTL_FIXTURE_DIR, name);
}

static size_t read_fixture(const char *name, char *output, size_t capacity) {
    char path[512];
    fixture_path(path, sizeof(path), name);
    FILE *file = fopen(path, "rb");
    TEST_ASSERT_NOT_NULL(file);
    size_t length = fread(output, 1U, capacity - 1U, file);
    TEST_ASSERT_FALSE(ferror(file));
    TEST_ASSERT_EQUAL_INT(0, fclose(file));
    output[length] = '\0';
    return length;
}

static void bind_target_device(uint64_t device) {
    sqlite3_stmt *statement = NULL;
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_prepare_v2(
        get_db_handle(),
        "UPDATE storage_targets SET filesystem_device=?1 WHERE uuid=?2;",
        -1, &statement, NULL));
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_bind_int64(
        statement, 1, (sqlite3_int64)device));
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_bind_text(
        statement, 2, target_uuid, -1, SQLITE_TRANSIENT));
    TEST_ASSERT_EQUAL_INT(SQLITE_DONE, sqlite3_step(statement));
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_finalize(statement));
}

void setUp(void) {
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(
        get_db_handle(), "DELETE FROM storage_targets;", NULL, NULL, NULL));
    TEST_ASSERT_EQUAL_INT(
        0, db_storage_target_bootstrap_default(target_root, target_uuid));
    bind_target_device((uint64_t)makedev(8U, 1U));
    fake_fixture = "ata-healthy.json";
    fake_outcome = HEALTH_HELPER_OK;
    fake_exit_status = 0;
    fake_truncated = false;
    fake_return_status = 0;
    fake_request_valid = true;
    fake_run_calls = 0U;
    fake_wake_calls = 0U;
}

void tearDown(void) {}

static int parse_fixture(const char *name, int exit_status,
                         smartctl_normalized_sample_t *sample) {
    char json[HEALTH_HELPER_OUTPUT_MAX + 1U];
    size_t length = read_fixture(name, json, sizeof(json));
    return smartctl_provider_parse_json(json, length, exit_status, sample);
}

static void test_parser_normalizes_ata_nvme_and_scsi(void) {
    smartctl_normalized_sample_t sample;
    TEST_ASSERT_EQUAL_INT(0, parse_fixture("ata-healthy.json", 0, &sample));
    TEST_ASSERT_EQUAL(SMARTCTL_PARSE_OK, sample.status);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_AVAILABLE, sample.capability);
    TEST_ASSERT_TRUE(sample.health_valid);
    TEST_ASSERT_TRUE(sample.health_passed);
    TEST_ASSERT_TRUE(sample.prefail);
    TEST_ASSERT_FALSE(sample.critical);
    TEST_ASSERT_FLOAT_WITHIN(0.0001f, 38.0f,
                             (float)sample.temperature_celsius.value);
    TEST_ASSERT_FLOAT_WITHIN(0.0001f, 0.95f,
                             (float)sample.available_spare_ratio.value);
    TEST_ASSERT_FLOAT_WITHIN(0.0001f, 0.22f,
                             (float)sample.percentage_used_ratio.value);
    TEST_ASSERT_EQUAL_UINT64(3U, sample.reallocated_sectors.value);
    TEST_ASSERT_EQUAL_UINT64(0U, sample.pending_sectors.value);
    TEST_ASSERT_EQUAL_UINT64(1U, sample.uncorrectable_errors.value);
    TEST_ASSERT_EQUAL_UINT64(7U, sample.interface_crc_errors.value);
    TEST_ASSERT_EQUAL_UINT64(1U, sample.media_errors.value);

    TEST_ASSERT_EQUAL_INT(0, parse_fixture("nvme-healthy.json", 0, &sample));
    TEST_ASSERT_FALSE(sample.prefail);
    TEST_ASSERT_FALSE(sample.critical);
    TEST_ASSERT_FLOAT_WITHIN(0.0001f, 0.88f,
                             (float)sample.available_spare_ratio.value);
    TEST_ASSERT_FLOAT_WITHIN(0.0001f, 0.12f,
                             (float)sample.percentage_used_ratio.value);
    TEST_ASSERT_EQUAL_UINT64(4U, sample.media_errors.value);
    TEST_ASSERT_EQUAL_UINT64(2U, sample.unsafe_shutdowns.value);

    TEST_ASSERT_EQUAL_INT(0, parse_fixture("scsi-healthy.json", 0, &sample));
    TEST_ASSERT_EQUAL_UINT64(2U, sample.reallocated_sectors.value);
    TEST_ASSERT_EQUAL_UINT64(6U, sample.uncorrectable_errors.value);
    TEST_ASSERT_EQUAL_UINT64(6U, sample.media_errors.value);

    TEST_ASSERT_EQUAL_INT(0, parse_fixture("ata-prefail.json", 24, &sample));
    TEST_ASSERT_TRUE(sample.prefail);
    TEST_ASSERT_TRUE(sample.critical);
    TEST_ASSERT_FALSE(sample.health_passed);
}

static void test_parser_maps_bounded_failures_and_exit_bits(void) {
    smartctl_normalized_sample_t sample;
    TEST_ASSERT_EQUAL_INT(0, parse_fixture("sleeping.json", 2, &sample));
    TEST_ASSERT_EQUAL(SMARTCTL_PARSE_SLEEPING, sample.status);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_STALE, sample.capability);

    TEST_ASSERT_EQUAL_INT(
        0, parse_fixture("permission-denied.json", 2, &sample));
    TEST_ASSERT_EQUAL(SMARTCTL_PARSE_PERMISSION_DENIED, sample.status);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED,
                      sample.capability);

    TEST_ASSERT_EQUAL_INT(
        0, parse_fixture("unsupported-version.json", 0, &sample));
    TEST_ASSERT_EQUAL(SMARTCTL_PARSE_UNSUPPORTED_VERSION, sample.status);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
                      sample.capability);

    TEST_ASSERT_EQUAL_INT(-1, parse_fixture("malformed.json", 0, &sample));
    TEST_ASSERT_EQUAL_INT(-1, parse_fixture("overflow.json", 0, &sample));
    TEST_ASSERT_EQUAL_INT(-1, parse_fixture("ata-healthy.json", 8, &sample));

    TEST_ASSERT_EQUAL_INT(
        0, parse_fixture("nonfatal-bitmask.json", 96, &sample));
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_AVAILABLE, sample.capability);
    TEST_ASSERT_FALSE(sample.prefail);
    TEST_ASSERT_FALSE(sample.critical);

    TEST_ASSERT_EQUAL_INT(0, parse_fixture("command-error.json", 1, &sample));
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_ERROR, sample.capability);

    char one = '{';
    TEST_ASSERT_EQUAL_INT(-1, smartctl_provider_parse_json(
        &one, HEALTH_HELPER_OUTPUT_MAX + 1U, 0, &sample));
}

static int fake_run_helper(const health_helper_request_t *request,
                           health_helper_result_t *result) {
    static const char *const expected[] = {
        "-H", "-A", "-j=c", "-n", "standby,2", "--", "/dev/sda"
    };
    fake_run_calls++;
    if (!request || !result || !request->program ||
        request->program[0] != '/' || !request->argv ||
        strcmp(request->argv[0], request->program) != 0 ||
        request->timeout_ms != SMARTCTL_PROVIDER_TIMEOUT_MS ||
        request->terminate_grace_ms !=
            SMARTCTL_PROVIDER_TERMINATE_GRACE_MS ||
        request->output_limit != HEALTH_HELPER_OUTPUT_MAX)
        fake_request_valid = false;
    for (size_t index = 0U;
         fake_request_valid && index < sizeof(expected) / sizeof(expected[0]);
         ++index)
        if (!request->argv[index + 1U] ||
            strcmp(request->argv[index + 1U], expected[index]) != 0)
            fake_request_valid = false;
    if (fake_request_valid && request->argv[8] != NULL)
        fake_request_valid = false;
    memset(result, 0, sizeof(*result));
    result->outcome = fake_outcome;
    result->exit_code = fake_exit_status;
    result->output_truncated = fake_truncated;
    if (fake_fixture &&
        (fake_outcome == HEALTH_HELPER_OK ||
         fake_outcome == HEALTH_HELPER_EXITED))
        result->output_length = read_fixture(
            fake_fixture, result->output, sizeof(result->output));
    return fake_return_status;
}

static bool fake_wake(void) {
    fake_wake_calls++;
    return true;
}

static const system_health_observation_t *find_observation(
    const system_health_observation_sink_t *sink, const char *metric) {
    for (size_t index = 0U; index < sink->count; ++index)
        if (strcmp(sink->items[index].metric, metric) == 0)
            return &sink->items[index];
    return NULL;
}

static system_health_collect_context_t fixture_context(uint64_t monotonic_ms) {
    static char sys_root[512];
    fixture_path(sys_root, sizeof(sys_root), "target-sys/sys");
    system_health_collect_context_t context = {
        .monotonic_ms = monotonic_ms,
        .wall_time_ms = 1700000000000LL + (int64_t)monotonic_ms,
        .sys_root = sys_root
    };
    return context;
}

static void test_collection_uses_fixed_command_safe_identity_and_deltas(void) {
    smartctl_provider_state_t state;
    smartctl_provider_state_init(&state, "installation-A");
    state.run_helper = fake_run_helper;
    TEST_ASSERT_TRUE(smartctl_provider_add_device(&state, "/dev/sda1"));
    TEST_ASSERT_TRUE(smartctl_provider_add_device(&state, "/dev/sda"));
    TEST_ASSERT_FALSE(smartctl_provider_add_device(&state, "/dev/../sdb"));
    TEST_ASSERT_EQUAL_size_t(1U, state.device_count);
    TEST_ASSERT_EQUAL_STRING("/dev/sda", state.devices[0].device_path);

    system_health_collect_context_t context = fixture_context(1000U);
    system_health_provider_inventory_t inventory;
    TEST_ASSERT_EQUAL_INT(
        0, smartctl_provider_discover(&state, &context, &inventory));
    char expected_id[SYSTEM_HEALTH_ID_LENGTH];
    snprintf(expected_id, sizeof(expected_id), "target:%s", target_uuid);
    TEST_ASSERT_EQUAL_STRING(expected_id, state.devices[0].public_id);

    system_health_observation_t items[32];
    system_health_observation_sink_t sink = {
        .items = items, .capacity = 32U
    };
    TEST_ASSERT_EQUAL_INT(0, smartctl_provider_collect(&state, &context,
                                                       &sink));
    TEST_ASSERT_TRUE(fake_request_valid);
    TEST_ASSERT_EQUAL_UINT(1U, fake_run_calls);
    const system_health_observation_t *value = find_observation(
        &sink, "storage.device.uncorrectable_errors_delta");
    TEST_ASSERT_NOT_NULL(value);
    TEST_ASSERT_FALSE(value->value_valid);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_STALE, value->capability);
    for (size_t index = 0U; index < sink.count; ++index) {
        TEST_ASSERT_NULL(strstr(items[index].resource_id, "/dev/"));
        TEST_ASSERT_NULL(strstr(items[index].resource_id, "SECRET"));
        TEST_ASSERT_NULL(strstr(items[index].metric, "SECRET"));
    }

    fake_fixture = "ata-second.json";
    context.monotonic_ms = 2000U;
    sink.count = 0U;
    TEST_ASSERT_EQUAL_INT(0, smartctl_provider_collect(&state, &context,
                                                       &sink));
    value = find_observation(&sink,
                             "storage.device.uncorrectable_errors_delta");
    TEST_ASSERT_NOT_NULL(value);
    TEST_ASSERT_TRUE(value->value_valid);
    TEST_ASSERT_FLOAT_WITHIN(0.0001f, 2.0f, (float)value->value);
    value = find_observation(&sink,
                             "storage.device.interface_crc_errors_delta");
    TEST_ASSERT_NOT_NULL(value);
    TEST_ASSERT_FLOAT_WITHIN(0.0001f, 2.0f, (float)value->value);
    value = find_observation(&sink, "storage.device.media_errors_delta");
    TEST_ASSERT_NOT_NULL(value);
    TEST_ASSERT_FLOAT_WITHIN(0.0001f, 2.0f, (float)value->value);
}

static void collect_fake_outcome(health_helper_outcome_t outcome,
                                 bool truncated,
                                 system_health_capability_t expected) {
    smartctl_provider_state_t state;
    smartctl_provider_state_init(&state, "installation-A");
    state.run_helper = fake_run_helper;
    TEST_ASSERT_TRUE(smartctl_provider_add_device(&state, "/dev/sda"));
    snprintf(state.devices[0].public_id, sizeof(state.devices[0].public_id),
             "%s", "device.fixture");
    fake_outcome = outcome;
    fake_truncated = truncated;
    system_health_collect_context_t context = fixture_context(1000U);
    system_health_observation_t items[32];
    system_health_observation_sink_t sink = {
        .items = items, .capacity = 32U
    };
    TEST_ASSERT_EQUAL_INT(0, smartctl_provider_collect(&state, &context,
                                                       &sink));
    const system_health_observation_t *value = find_observation(
        &sink, "storage.device.critical");
    TEST_ASSERT_NOT_NULL(value);
    TEST_ASSERT_FALSE(value->value_valid);
    TEST_ASSERT_EQUAL(expected, value->capability);
}

static void test_helper_failures_are_capabilities_without_stale_values(void) {
    fake_fixture = NULL;
    collect_fake_outcome(HEALTH_HELPER_EXEC_ERROR, false,
                         SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED);
    collect_fake_outcome(HEALTH_HELPER_TIMED_OUT, false,
                         SYSTEM_HEALTH_CAPABILITY_ERROR);
    collect_fake_outcome(HEALTH_HELPER_BUSY, false,
                         SYSTEM_HEALTH_CAPABILITY_STALE);
    collect_fake_outcome(HEALTH_HELPER_OK, true,
                         SYSTEM_HEALTH_CAPABILITY_ERROR);
    fake_return_status = -1;
    collect_fake_outcome(HEALTH_HELPER_OK, false,
                         SYSTEM_HEALTH_CAPABILITY_ERROR);
}

static void test_sleeping_preserves_delta_baseline(void) {
    smartctl_provider_state_t state;
    smartctl_provider_state_init(&state, "installation-A");
    state.run_helper = fake_run_helper;
    TEST_ASSERT_TRUE(smartctl_provider_add_device(&state, "/dev/sda"));
    snprintf(state.devices[0].public_id, sizeof(state.devices[0].public_id),
             "%s", "device.fixture");
    system_health_collect_context_t context = fixture_context(1000U);
    system_health_observation_t items[32];
    system_health_observation_sink_t sink = {
        .items = items, .capacity = 32U
    };
    TEST_ASSERT_EQUAL_INT(0, smartctl_provider_collect(&state, &context,
                                                       &sink));
    fake_fixture = "sleeping.json";
    fake_exit_status = 2;
    fake_outcome = HEALTH_HELPER_EXITED;
    context.monotonic_ms = 1500U;
    sink.count = 0U;
    TEST_ASSERT_EQUAL_INT(0, smartctl_provider_collect(&state, &context,
                                                       &sink));
    const system_health_observation_t *value = find_observation(
        &sink, "storage.device.critical");
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_STALE, value->capability);

    fake_fixture = "ata-second.json";
    fake_exit_status = 0;
    fake_outcome = HEALTH_HELPER_OK;
    context.monotonic_ms = 2000U;
    sink.count = 0U;
    TEST_ASSERT_EQUAL_INT(0, smartctl_provider_collect(&state, &context,
                                                       &sink));
    value = find_observation(&sink,
                             "storage.device.uncorrectable_errors_delta");
    TEST_ASSERT_TRUE(value->value_valid);
    TEST_ASSERT_FLOAT_WITHIN(0.0001f, 2.0f, (float)value->value);
}

static void test_target_discovery_is_deduplicated_and_bounded(void) {
    smartctl_provider_state_t state;
    smartctl_provider_state_init(&state, "installation-A");
    state.discover_target_devices = true;
    system_health_collect_context_t context = fixture_context(1000U);
    system_health_provider_inventory_t inventory;
    TEST_ASSERT_EQUAL_INT(
        0, smartctl_provider_discover(&state, &context, &inventory));
    TEST_ASSERT_EQUAL_size_t(1U, state.device_count);
    TEST_ASSERT_EQUAL_STRING("/dev/sda", state.devices[0].device_path);
    char expected_id[SYSTEM_HEALTH_ID_LENGTH];
    snprintf(expected_id, sizeof(expected_id), "target:%s", target_uuid);
    TEST_ASSERT_EQUAL_STRING(expected_id, state.devices[0].public_id);
    TEST_ASSERT_NULL(strstr(inventory.resources[0].id, "/dev/"));
    TEST_ASSERT_EQUAL_INT(
        0, smartctl_provider_discover(&state, &context, &inventory));
    TEST_ASSERT_EQUAL_size_t(1U, state.device_count);
}

static void test_refresh_requests_are_coalesced_and_wake_device_tier(void) {
    smartctl_provider_state_t state;
    smartctl_provider_state_init(&state, "installation-A");
    state.run_helper = fake_run_helper;
    state.wake_device_tier = fake_wake;
    TEST_ASSERT_TRUE(smartctl_provider_add_device(&state, "/dev/sda"));
    snprintf(state.devices[0].public_id, sizeof(state.devices[0].public_id),
             "%s", "device.fixture");
    TEST_ASSERT_TRUE(smartctl_provider_request_refresh(&state));
    TEST_ASSERT_TRUE(smartctl_provider_request_refresh(&state));
    TEST_ASSERT_EQUAL_UINT(1U, fake_wake_calls);
    TEST_ASSERT_EQUAL_UINT64(2U, atomic_load(&state.refresh_requests));
    TEST_ASSERT_EQUAL_UINT64(1U, atomic_load(&state.refresh_coalesced));

    system_health_collect_context_t context = fixture_context(1000U);
    system_health_observation_t items[32];
    system_health_observation_sink_t sink = {
        .items = items, .capacity = 32U
    };
    TEST_ASSERT_EQUAL_INT(0, smartctl_provider_collect(&state, &context,
                                                       &sink));
    TEST_ASSERT_FALSE(atomic_load(&state.refresh_pending));
    TEST_ASSERT_EQUAL_UINT64(1U, atomic_load(&state.refresh_collections));
}

static uint64_t monotonic_ms(void) {
    struct timespec value;
    TEST_ASSERT_EQUAL_INT(0, clock_gettime(CLOCK_MONOTONIC, &value));
    return (uint64_t)value.tv_sec * 1000U +
        (uint64_t)value.tv_nsec / 1000000U;
}

static void test_real_helper_missing_and_timeout_cleanup_are_bounded(void) {
    smartctl_provider_state_t state;
    smartctl_provider_state_init(&state, "installation-A");
    TEST_ASSERT_TRUE(smartctl_provider_add_device(&state, "/dev/sda"));
    snprintf(state.devices[0].public_id, sizeof(state.devices[0].public_id),
             "%s", "device.fixture");
    snprintf(state.program, sizeof(state.program), "%s",
             "/definitely/not-installed/lightnvr-smartctl");
    system_health_collect_context_t context = fixture_context(1000U);
    system_health_observation_t items[32];
    system_health_observation_sink_t sink = {
        .items = items, .capacity = 32U
    };
    TEST_ASSERT_EQUAL_INT(0, smartctl_provider_collect(&state, &context,
                                                       &sink));
    const system_health_observation_t *value = find_observation(
        &sink, "storage.device.critical");
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
                      value->capability);

    snprintf(state.program, sizeof(state.program), "%s", "/proc/self/exe");
    state.timeout_ms = 30U;
    state.terminate_grace_ms = 10U;
    context.monotonic_ms = 2000U;
    sink.count = 0U;
    uint64_t started = monotonic_ms();
    TEST_ASSERT_EQUAL_INT(0, smartctl_provider_collect(&state, &context,
                                                       &sink));
    uint64_t elapsed = monotonic_ms() - started;
    TEST_ASSERT_LESS_THAN_UINT64(1000U, elapsed);
    value = find_observation(&sink, "storage.device.critical");
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_ERROR, value->capability);
    TEST_ASSERT_EQUAL_UINT32(0U, health_helper_abandoned_count());
}

static char registered_names[3][SYSTEM_HEALTH_PROVIDER_NAME_LENGTH];
static size_t registered_count;

static bool capture_provider(const system_health_provider_t *provider,
                             void *context) {
    (void)context;
    if (registered_count >= 3U) return false;
    snprintf(registered_names[registered_count],
             sizeof(registered_names[registered_count]), "%s",
             provider->name);
    if (strcmp(provider->name, "smartctl") == 0) {
        smartctl_provider_state_t *state = provider->state;
        state->wake_device_tier = fake_wake;
    }
    registered_count++;
    return true;
}

static void test_smartctl_selection_is_explicit_and_auto_is_unchanged(void) {
    registered_count = 0U;
    TEST_ASSERT_EQUAL_size_t(3U, system_health_register_builtin_providers(
        "smartctl", capture_provider, NULL));
    TEST_ASSERT_EQUAL_STRING("linux_hardware", registered_names[0]);
    TEST_ASSERT_EQUAL_STRING("smartctl", registered_names[1]);
    TEST_ASSERT_EQUAL_STRING("kernel_log", registered_names[2]);
    TEST_ASSERT_TRUE(system_health_builtin_provider_request_device_refresh());
    TEST_ASSERT_EQUAL_UINT(1U, fake_wake_calls);
    registered_count = 0U;
    TEST_ASSERT_EQUAL_size_t(3U, system_health_register_builtin_providers(
        "auto", capture_provider, NULL));
    TEST_ASSERT_EQUAL_STRING("nvme", registered_names[1]);
    TEST_ASSERT_FALSE(system_health_builtin_provider_request_device_refresh());
    TEST_ASSERT_EQUAL_UINT(1U, fake_wake_calls);
    TEST_ASSERT_EQUAL_size_t(0U, system_health_register_builtin_providers(
        "disabled", capture_provider, NULL));
    TEST_ASSERT_FALSE(system_health_builtin_provider_request_device_refresh());
}

int main(int argc, char **argv) {
    if (argc > 1 && strcmp(argv[1], "-H") == 0) {
        (void)signal(SIGTERM, SIG_IGN);
        for (;;) pause();
    }
    TEST_ASSERT_NOT_NULL(mkdtemp(target_root));
    unlink(TEST_DB_PATH);
    if (init_database_ex(TEST_DB_PATH,
                         DB_INIT_NO_CHECK | DB_INIT_NO_BACKUP) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_parser_normalizes_ata_nvme_and_scsi);
    RUN_TEST(test_parser_maps_bounded_failures_and_exit_bits);
    RUN_TEST(test_collection_uses_fixed_command_safe_identity_and_deltas);
    RUN_TEST(test_helper_failures_are_capabilities_without_stale_values);
    RUN_TEST(test_sleeping_preserves_delta_baseline);
    RUN_TEST(test_target_discovery_is_deduplicated_and_bounded);
    RUN_TEST(test_refresh_requests_are_coalesced_and_wake_device_tier);
    RUN_TEST(test_real_helper_missing_and_timeout_cleanup_are_bounded);
    RUN_TEST(test_smartctl_selection_is_explicit_and_auto_is_unchanged);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    rmdir(target_root);
    return result;
}
