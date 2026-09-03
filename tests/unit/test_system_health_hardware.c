#define _POSIX_C_SOURCE 200809L

#include "unity.h"

#include <errno.h>
#include <fcntl.h>
#include <sqlite3.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/sysmacros.h>
#include <unistd.h>

#include "database/db_core.h"
#include "database/db_storage_targets.h"
#include "telemetry/providers/builtin_providers.h"
#include "telemetry/providers/linux_hardware.h"
#include "telemetry/providers/nvme_provider.h"

#ifndef TEST_HARDWARE_FIXTURE_DIR
#define TEST_HARDWARE_FIXTURE_DIR \
    "tests/fixtures/system_health/sysfs/hardware"
#endif

#define TEST_DB_PATH "/tmp/lightnvr_unit_system_health_hardware.db"

static char target_root[] = "/tmp/lightnvr-hardware-target-XXXXXX";
static char target_uuid[LIGHTNVR_UUID_STRING_SIZE];
static uint8_t fake_nvme_log[NVME_PROVIDER_HEALTH_LOG_SIZE];
static int fake_open_error;
static int fake_log_error;
static unsigned int fake_open_calls;
static int fake_open_flags;

static void fixture_root(char *output, size_t capacity, const char *fixture) {
    snprintf(output, capacity, "%s/%s/sys", TEST_HARDWARE_FIXTURE_DIR,
             fixture);
}

static const system_health_observation_t *find_observation(
    const system_health_observation_t *items, size_t count,
    const char *metric, const char *resource) {
    for (size_t index = 0U; index < count; ++index)
        if (strcmp(items[index].metric, metric) == 0 &&
            (!resource || strcmp(items[index].resource_id, resource) == 0))
            return &items[index];
    return NULL;
}

static void bind_target_device(uint64_t device) {
    sqlite3_stmt *statement = NULL;
    TEST_ASSERT_EQUAL_INT(
        SQLITE_OK,
        sqlite3_prepare_v2(get_db_handle(),
                           "UPDATE storage_targets SET filesystem_device=?1 "
                           "WHERE uuid=?2;", -1, &statement, NULL));
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
    bind_target_device((uint64_t)makedev(179U, 1U));
    memset(fake_nvme_log, 0, sizeof(fake_nvme_log));
    fake_open_error = 0;
    fake_log_error = 0;
    fake_open_calls = 0U;
    fake_open_flags = 0;
}

void tearDown(void) {}

static void test_device_map_deduplicates_and_prefers_target_uuid(void) {
    char sys_root[512];
    fixture_root(sys_root, sizeof(sys_root), "sample1");
    linux_device_map_t map;
    TEST_ASSERT_EQUAL_INT(
        0, linux_device_map_build(sys_root, "installation-A", &map));
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_AVAILABLE, map.capability);
    TEST_ASSERT_EQUAL_size_t(2U, map.count);
    const linux_device_map_entry_t *mmc =
        linux_device_map_find(&map, "mmcblk0");
    TEST_ASSERT_NOT_NULL(mmc);
    char expected[SYSTEM_HEALTH_ID_LENGTH];
    snprintf(expected, sizeof(expected), "target:%s", target_uuid);
    TEST_ASSERT_EQUAL_STRING(expected, mmc->public_id);
    TEST_ASSERT_TRUE(mmc->target_mapped);
    TEST_ASSERT_NULL(strstr(mmc->public_id, "mmcblk"));
    const linux_device_map_entry_t *nvme =
        linux_device_map_find(&map, "nvme0");
    TEST_ASSERT_NOT_NULL(nvme);
    TEST_ASSERT_TRUE(strncmp(nvme->public_id, "device.", 7U) == 0);
    TEST_ASSERT_NULL(strchr(nvme->public_id, '/'));
}

static void collect_hardware(linux_hardware_state_t *state,
                             const char *fixture, uint64_t monotonic_ms,
                             system_health_observation_t *items,
                             system_health_observation_sink_t *sink) {
    char sys_root[512];
    fixture_root(sys_root, sizeof(sys_root), fixture);
    system_health_collect_context_t context = {
        .monotonic_ms = monotonic_ms,
        .wall_time_ms = 1700000000000LL + (int64_t)monotonic_ms,
        .sys_root = sys_root
    };
    system_health_provider_inventory_t inventory;
    TEST_ASSERT_EQUAL_INT(
        0, linux_hardware_discover(state, &context, &inventory));
    TEST_ASSERT_TRUE(inventory.count > 0U);
    sink->items = items;
    sink->capacity = 128U;
    sink->count = 0U;
    sink->dropped = 0U;
    TEST_ASSERT_EQUAL_INT(0, linux_hardware_collect(state, &context, sink));
}

static void test_sysfs_health_deltas_prefail_and_hot_gated_fan(void) {
    linux_hardware_state_t state;
    linux_hardware_state_init(&state, "installation-A");
    system_health_observation_t items[128];
    system_health_observation_sink_t sink;
    collect_hardware(&state, "sample1", 1000U, items, &sink);

    char target_id[SYSTEM_HEALTH_ID_LENGTH];
    snprintf(target_id, sizeof(target_id), "target:%s", target_uuid);
    const system_health_observation_t *observation = find_observation(
        items, sink.count, "storage.device.life_used_ratio", target_id);
    TEST_ASSERT_NOT_NULL(observation);
    TEST_ASSERT_TRUE(observation->value_valid);
    TEST_ASSERT_FLOAT_WITHIN(0.001f, 1.0f, (float)observation->value);
    observation = find_observation(items, sink.count,
                                   "storage.device.prefail", target_id);
    TEST_ASSERT_NOT_NULL(observation);
    TEST_ASSERT_FLOAT_WITHIN(0.001f, 1.0f, (float)observation->value);
    observation = find_observation(items, sink.count,
                                   "storage.device.critical", target_id);
    TEST_ASSERT_NOT_NULL(observation);
    TEST_ASSERT_FLOAT_WITHIN(0.001f, 0.0f, (float)observation->value);
    observation = find_observation(items, sink.count,
                                   "hardware.fan.failed", NULL);
    TEST_ASSERT_NOT_NULL(observation);
    TEST_ASSERT_TRUE(observation->value_valid);
    TEST_ASSERT_FLOAT_WITHIN(0.001f, 1.0f, (float)observation->value);
    observation = find_observation(items, sink.count,
                                   "hardware.ecc.corrected_delta", "edac.mc0");
    TEST_ASSERT_NOT_NULL(observation);
    TEST_ASSERT_FALSE(observation->value_valid);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_STALE,
                      observation->capability);

    collect_hardware(&state, "sample2", 2000U, items, &sink);
    observation = find_observation(items, sink.count,
                                   "hardware.ecc.corrected_delta", "edac.mc0");
    TEST_ASSERT_NOT_NULL(observation);
    TEST_ASSERT_FLOAT_WITHIN(0.001f, 5.0f, (float)observation->value);
    observation = find_observation(items, sink.count,
        "hardware.ecc.uncorrectable_delta", "edac.mc0");
    TEST_ASSERT_NOT_NULL(observation);
    TEST_ASSERT_FLOAT_WITHIN(0.001f, 1.0f, (float)observation->value);
    observation = find_observation(items, sink.count,
                                   "hardware.fan.failed", NULL);
    TEST_ASSERT_NOT_NULL(observation);
    TEST_ASSERT_FLOAT_WITHIN(0.001f, 1.0f, (float)observation->value);
    observation = find_observation(items, sink.count,
                                   "hardware.fan.hot", NULL);
    TEST_ASSERT_NOT_NULL(observation);
    TEST_ASSERT_FLOAT_WITHIN(0.001f, 0.0f, (float)observation->value);
    observation = find_observation(items, sink.count,
                                   "storage.device.critical", target_id);
    TEST_ASSERT_NOT_NULL(observation);
    TEST_ASSERT_FLOAT_WITHIN(0.001f, 1.0f, (float)observation->value);
}

static void test_malformed_sysfs_values_are_capability_errors(void) {
    linux_hardware_state_t state;
    linux_hardware_state_init(&state, "installation-A");
    system_health_observation_t items[128];
    system_health_observation_sink_t sink;
    collect_hardware(&state, "malformed", 1000U, items, &sink);
    const system_health_observation_t *life = find_observation(
        items, sink.count, "storage.device.life_used_ratio", NULL);
    TEST_ASSERT_NOT_NULL(life);
    TEST_ASSERT_FALSE(life->value_valid);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_ERROR, life->capability);
    const system_health_observation_t *corrected = find_observation(
        items, sink.count, "hardware.ecc.corrected_delta", "edac.mc0");
    TEST_ASSERT_NOT_NULL(corrected);
    TEST_ASSERT_FALSE(corrected->value_valid);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_ERROR, corrected->capability);
}

static void test_disappearing_resources_are_not_cached_and_reset_deltas(void) {
    linux_hardware_state_t state;
    linux_hardware_state_init(&state, "installation-A");
    system_health_observation_t items[128];
    system_health_observation_sink_t sink;
    collect_hardware(&state, "sample1", 1000U, items, &sink);
    TEST_ASSERT_NOT_NULL(find_observation(items, sink.count,
                                          "hardware.fan.rpm", NULL));
    collect_hardware(&state, "malformed", 2000U, items, &sink);
    TEST_ASSERT_NULL(find_observation(items, sink.count,
                                     "hardware.fan.rpm", NULL));
    collect_hardware(&state, "sample2", 3000U, items, &sink);
    const system_health_observation_t *corrected = find_observation(
        items, sink.count, "hardware.ecc.corrected_delta", "edac.mc0");
    TEST_ASSERT_NOT_NULL(corrected);
    TEST_ASSERT_FALSE(corrected->value_valid);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_STALE, corrected->capability);
}

static int fake_nvme_open(const char *path, int flags) {
    fake_open_calls++;
    fake_open_flags = flags;
    TEST_ASSERT_EQUAL_STRING("/fixture-dev/nvme0", path);
    if (fake_open_error) {
        errno = fake_open_error;
        return -1;
    }
    return 42;
}

static int fake_nvme_read(int descriptor, uint8_t *output, size_t size) {
    TEST_ASSERT_EQUAL_INT(42, descriptor);
    TEST_ASSERT_EQUAL_size_t(sizeof(fake_nvme_log), size);
    if (fake_log_error) {
        errno = fake_log_error;
        return -1;
    }
    memcpy(output, fake_nvme_log, size);
    return 0;
}

static int fake_nvme_close(int descriptor) {
    TEST_ASSERT_EQUAL_INT(42, descriptor);
    return 0;
}

static void set_little_u64(uint8_t *destination, uint64_t value) {
    for (size_t index = 0U; index < 8U; ++index)
        destination[index] = (uint8_t)(value >> (index * 8U));
}

static void prepare_nvme_state(nvme_provider_state_t *state,
                               system_health_collect_context_t *context,
                               char sys_root[512]) {
    nvme_provider_state_init(state, "installation-A");
    snprintf(state->dev_root, sizeof(state->dev_root), "%s", "/fixture-dev");
    state->ops.open_device = fake_nvme_open;
    state->ops.read_health_log = fake_nvme_read;
    state->ops.close_device = fake_nvme_close;
    fixture_root(sys_root, 512U, "sample1");
    memset(context, 0, sizeof(*context));
    context->sys_root = sys_root;
    context->monotonic_ms = 1000U;
    system_health_provider_inventory_t inventory;
    TEST_ASSERT_EQUAL_INT(0, nvme_provider_discover(state, context,
                                                    &inventory));
    TEST_ASSERT_EQUAL_size_t(1U, state->device_count);
    TEST_ASSERT_EQUAL_size_t(1U, inventory.count);
    TEST_ASSERT_NULL(strstr(state->devices[0].public_id, "nvme"));
}

static void test_nvme_health_log_is_bounded_and_delta_based(void) {
    nvme_provider_state_t state;
    system_health_collect_context_t context;
    char sys_root[512];
    prepare_nvme_state(&state, &context, sys_root);
    fake_nvme_log[1] = 0x2cU;
    fake_nvme_log[2] = 0x01U; /* 300 K. */
    fake_nvme_log[3] = 90U;
    fake_nvme_log[4] = 10U;
    fake_nvme_log[5] = 91U;
    set_little_u64(fake_nvme_log + 144U, 2U);
    set_little_u64(fake_nvme_log + 160U, 4U);
    system_health_observation_t items[64];
    system_health_observation_sink_t sink = {
        .items = items, .capacity = 64U
    };
    TEST_ASSERT_EQUAL_INT(0, nvme_provider_collect(&state, &context, &sink));
    TEST_ASSERT_EQUAL_UINT(1U, fake_open_calls);
    TEST_ASSERT_TRUE((fake_open_flags & O_NONBLOCK) != 0);
    const char *public_id = state.devices[0].public_id;
    const system_health_observation_t *prefail = find_observation(
        items, sink.count, "storage.device.prefail", public_id);
    TEST_ASSERT_NOT_NULL(prefail);
    TEST_ASSERT_FLOAT_WITHIN(0.001f, 1.0f, (float)prefail->value);
    const system_health_observation_t *media = find_observation(
        items, sink.count, "storage.device.media_errors_delta", public_id);
    TEST_ASSERT_NOT_NULL(media);
    TEST_ASSERT_FALSE(media->value_valid);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_STALE, media->capability);

    set_little_u64(fake_nvme_log + 144U, 5U);
    set_little_u64(fake_nvme_log + 160U, 11U);
    context.monotonic_ms = 2000U;
    system_health_provider_inventory_t inventory;
    TEST_ASSERT_EQUAL_INT(0, nvme_provider_discover(&state, &context,
                                                    &inventory));
    sink.count = 0U;
    TEST_ASSERT_EQUAL_INT(0, nvme_provider_collect(&state, &context, &sink));
    media = find_observation(items, sink.count,
                             "storage.device.media_errors_delta", public_id);
    TEST_ASSERT_NOT_NULL(media);
    TEST_ASSERT_FLOAT_WITHIN(0.001f, 7.0f, (float)media->value);
    const system_health_observation_t *unsafe = find_observation(
        items, sink.count, "storage.device.unsafe_shutdowns_delta", public_id);
    TEST_ASSERT_NOT_NULL(unsafe);
    TEST_ASSERT_FLOAT_WITHIN(0.001f, 3.0f, (float)unsafe->value);
}

static void test_nvme_denied_and_overflow_are_unavailable_capabilities(void) {
    nvme_provider_state_t state;
    system_health_collect_context_t context;
    char sys_root[512];
    prepare_nvme_state(&state, &context, sys_root);
    fake_open_error = EACCES;
    system_health_observation_t items[64];
    system_health_observation_sink_t sink = {
        .items = items, .capacity = 64U
    };
    TEST_ASSERT_EQUAL_INT(0, nvme_provider_collect(&state, &context, &sink));
    const system_health_observation_t *media = find_observation(
        items, sink.count, "storage.device.media_errors_delta", NULL);
    TEST_ASSERT_NOT_NULL(media);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED,
                      media->capability);
    fake_open_error = 0;
    fake_nvme_log[1] = 0x2cU;
    fake_nvme_log[2] = 0x01U;
    fake_nvme_log[168] = 1U; /* High half of 128-bit media counter. */
    sink.count = 0U;
    TEST_ASSERT_EQUAL_INT(0, nvme_provider_collect(&state, &context, &sink));
    media = find_observation(items, sink.count,
                             "storage.device.media_errors_delta", NULL);
    TEST_ASSERT_NOT_NULL(media);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_ERROR, media->capability);
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
    registered_count++;
    return true;
}

static void test_builtin_auto_registration_and_disabled_selection(void) {
    registered_count = 0U;
    TEST_ASSERT_EQUAL_size_t(
        3U, system_health_register_builtin_providers(
                "auto", capture_provider, NULL));
    TEST_ASSERT_EQUAL_STRING("linux_hardware", registered_names[0]);
    TEST_ASSERT_EQUAL_STRING("nvme", registered_names[1]);
    TEST_ASSERT_EQUAL_STRING("kernel_log", registered_names[2]);
    TEST_ASSERT_EQUAL_size_t(
        0U, system_health_register_builtin_providers(
                "disabled", capture_provider, NULL));
}

int main(void) {
    TEST_ASSERT_NOT_NULL(mkdtemp(target_root));
    unlink(TEST_DB_PATH);
    if (init_database_ex(TEST_DB_PATH,
                         DB_INIT_NO_CHECK | DB_INIT_NO_BACKUP) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_device_map_deduplicates_and_prefers_target_uuid);
    RUN_TEST(test_sysfs_health_deltas_prefail_and_hot_gated_fan);
    RUN_TEST(test_malformed_sysfs_values_are_capability_errors);
    RUN_TEST(test_disappearing_resources_are_not_cached_and_reset_deltas);
    RUN_TEST(test_nvme_health_log_is_bounded_and_delta_based);
    RUN_TEST(test_nvme_denied_and_overflow_are_unavailable_capabilities);
    RUN_TEST(test_builtin_auto_registration_and_disabled_selection);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    rmdir(target_root);
    return result;
}
