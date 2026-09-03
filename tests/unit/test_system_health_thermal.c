#define _POSIX_C_SOURCE 200809L

#include "unity.h"

#include "telemetry/collectors/linux_thermal.h"

#include <ctype.h>
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <unistd.h>

#ifndef TEST_THERMAL_FIXTURE_DIR
#define TEST_THERMAL_FIXTURE_DIR "tests/fixtures/system_health/sysfs/thermal"
#endif

void setUp(void) {}
void tearDown(void) {}

static void fixture_path(char *output, size_t output_size,
                         const char *fixture, const char *root) {
    snprintf(output, output_size, "%s/%s/%s", TEST_THERMAL_FIXTURE_DIR,
             fixture, root);
}

static const system_health_observation_t *find_observation(
    const system_health_observation_t *items, size_t count, const char *metric,
    const char *resource_id) {
    for (size_t index = 0; index < count; ++index) {
        if (strcmp(items[index].metric, metric) == 0 &&
            (!resource_id || strcmp(items[index].resource_id, resource_id) == 0)) {
            return &items[index];
        }
    }
    return NULL;
}

static const linux_thermal_sensor_sample_t *find_temperature(
    const linux_thermal_result_t *result, double temperature) {
    for (size_t index = 0; index < result->sensor_count; ++index) {
        if (result->sensors[index].temperature_valid &&
            result->sensors[index].temperature_celsius == temperature) {
            return &result->sensors[index];
        }
    }
    return NULL;
}

static void assert_public_id(const char *id) {
    TEST_ASSERT_NOT_NULL(id);
    TEST_ASSERT_TRUE(id[0] != '\0');
    TEST_ASSERT_NULL(strchr(id, '/'));
    TEST_ASSERT_NULL(strchr(id, '\\'));
    TEST_ASSERT_NULL(strchr(id, ':'));
    for (const unsigned char *cursor = (const unsigned char *)id;
         *cursor; ++cursor) {
        TEST_ASSERT_TRUE(isalnum(*cursor) || *cursor == '.' || *cursor == '_');
    }
}

static void test_collects_zones_hwmon_and_prefers_critical_limits(void) {
    linux_thermal_state_t state;
    linux_thermal_state_init(&state);
    system_health_observation_t observations[128];
    system_health_observation_sink_t sink = {
        .items = observations, .capacity = 128, .count = 0, .dropped = 0
    };
    char sys_root[512];
    fixture_path(sys_root, sizeof(sys_root), "normal", "sys");
    system_health_collect_context_t context = {
        .monotonic_ms = 1000, .wall_time_ms = 1700000000000LL,
        .sys_root = sys_root
    };
    linux_thermal_result_t result;

    TEST_ASSERT_EQUAL_INT(0, linux_thermal_collect(&state, &context, &sink,
                                                   &result));
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_AVAILABLE, result.capability);
    TEST_ASSERT_EQUAL_size_t(3, result.sensor_count);
    TEST_ASSERT_EQUAL_size_t(0, result.resources_dropped);

    const linux_thermal_sensor_sample_t *zone = find_temperature(&result, 45.0);
    const linux_thermal_sensor_sample_t *package = find_temperature(&result, 51.0);
    const linux_thermal_sensor_sample_t *auxiliary = find_temperature(&result, 43.0);
    TEST_ASSERT_NOT_NULL(zone);
    TEST_ASSERT_NOT_NULL(package);
    TEST_ASSERT_NOT_NULL(auxiliary);
    TEST_ASSERT_TRUE(zone->limit_valid);
    TEST_ASSERT_TRUE(zone->limit_is_critical);
    TEST_ASSERT_FLOAT_WITHIN(0.001f, 90.0f, (float)zone->limit_celsius);
    TEST_ASSERT_TRUE(package->limit_valid);
    TEST_ASSERT_TRUE(package->limit_is_critical);
    TEST_ASSERT_FLOAT_WITHIN(0.001f, 105.0f, (float)package->limit_celsius);
    TEST_ASSERT_FALSE(auxiliary->limit_valid);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
                      auxiliary->limit_capability);

    for (size_t index = 0; index < result.sensor_count; ++index) {
        assert_public_id(result.sensors[index].id);
        const system_health_observation_t *temperature = find_observation(
            observations, sink.count, "thermal.temperature_celsius",
            result.sensors[index].id);
        TEST_ASSERT_NOT_NULL(temperature);
        TEST_ASSERT_EQUAL(SYSTEM_HEALTH_SCOPE_HOST, temperature->scope);
        TEST_ASSERT_EQUAL_UINT64(1000, temperature->sampled_monotonic_ms);
    }
    const system_health_observation_t *missing_limit = find_observation(
        observations, sink.count, "thermal.critical_celsius", auxiliary->id);
    TEST_ASSERT_NOT_NULL(missing_limit);
    TEST_ASSERT_FALSE(missing_limit->value_valid);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
                      missing_limit->capability);
}

static void test_malformed_and_overflow_temperature_are_errors(void) {
    linux_thermal_state_t state;
    linux_thermal_state_init(&state);
    system_health_observation_t observations[16];
    system_health_observation_sink_t sink = {
        .items = observations, .capacity = 16, .count = 0, .dropped = 0
    };
    char sys_root[512];
    fixture_path(sys_root, sizeof(sys_root), "malformed", "sys");
    system_health_collect_context_t context = {
        .monotonic_ms = 1, .wall_time_ms = 1, .sys_root = sys_root
    };
    linux_thermal_result_t result;

    TEST_ASSERT_EQUAL_INT(0, linux_thermal_collect(&state, &context, &sink,
                                                   &result));
    TEST_ASSERT_EQUAL_size_t(1, result.sensor_count);
    TEST_ASSERT_FALSE(result.sensors[0].temperature_valid);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_ERROR,
                      result.sensors[0].temperature_capability);
    TEST_ASSERT_FALSE(result.sensors[0].limit_valid);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_ERROR,
                      result.sensors[0].limit_capability);
}

static void test_disappearing_sensors_become_stale_not_zero(void) {
    linux_thermal_state_t state;
    linux_thermal_state_init(&state);
    system_health_observation_t observations[128];
    system_health_observation_sink_t sink = {
        .items = observations, .capacity = 128, .count = 0, .dropped = 0
    };
    char sys_root[512];
    fixture_path(sys_root, sizeof(sys_root), "normal", "sys");
    system_health_collect_context_t context = {
        .monotonic_ms = 1000, .wall_time_ms = 1, .sys_root = sys_root
    };
    TEST_ASSERT_EQUAL_INT(0, linux_thermal_collect(&state, &context, &sink,
                                                   NULL));

    fixture_path(sys_root, sizeof(sys_root), "missing", "sys");
    context.monotonic_ms = 2000;
    sink.count = 0;
    sink.dropped = 0;
    linux_thermal_result_t result;
    TEST_ASSERT_EQUAL_INT(0, linux_thermal_collect(&state, &context, &sink,
                                                   &result));
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED, result.capability);
    size_t stale_temperatures = 0;
    for (size_t index = 0; index < sink.count; ++index) {
        if (strcmp(observations[index].metric,
                   "thermal.temperature_celsius") == 0) {
            stale_temperatures++;
            TEST_ASSERT_FALSE(observations[index].value_valid);
            TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_STALE,
                              observations[index].capability);
        }
    }
    TEST_ASSERT_EQUAL_size_t(3, stale_temperatures);
}

static void write_text_file(const char *path, const char *text) {
    FILE *file = fopen(path, "w");
    TEST_ASSERT_NOT_NULL(file);
    TEST_ASSERT_EQUAL_size_t(strlen(text), fwrite(text, 1, strlen(text), file));
    TEST_ASSERT_EQUAL_INT(0, fclose(file));
}

static void test_sensor_enumeration_is_bounded_and_counts_overflow(void) {
    char root[] = "/tmp/lightnvr-thermal-overflow-XXXXXX";
    TEST_ASSERT_NOT_NULL(mkdtemp(root));
    char path[512];
    snprintf(path, sizeof(path), "%s/sys", root);
    TEST_ASSERT_EQUAL_INT(0, mkdir(path, 0700));
    snprintf(path, sizeof(path), "%s/sys/class", root);
    TEST_ASSERT_EQUAL_INT(0, mkdir(path, 0700));
    snprintf(path, sizeof(path), "%s/sys/class/thermal", root);
    TEST_ASSERT_EQUAL_INT(0, mkdir(path, 0700));
    for (unsigned int index = 0; index < SYSTEM_HEALTH_MAX_SENSORS + 1U;
         ++index) {
        snprintf(path, sizeof(path), "%s/sys/class/thermal/thermal_zone%u",
                 root, index);
        TEST_ASSERT_EQUAL_INT(0, mkdir(path, 0700));
        char file_path[512];
        snprintf(file_path, sizeof(file_path), "%s/type", path);
        write_text_file(file_path, "fixture\n");
        snprintf(file_path, sizeof(file_path), "%s/temp", path);
        write_text_file(file_path, "40000\n");
    }

    linux_thermal_state_t state;
    linux_thermal_state_init(&state);
    system_health_observation_t observations[128];
    system_health_observation_sink_t sink = {
        .items = observations, .capacity = 128, .count = 0, .dropped = 0
    };
    char sys_root[512];
    snprintf(sys_root, sizeof(sys_root), "%s/sys", root);
    system_health_collect_context_t context = {
        .monotonic_ms = 1, .wall_time_ms = 1, .sys_root = sys_root
    };
    linux_thermal_result_t result;
    TEST_ASSERT_EQUAL_INT(0, linux_thermal_collect(&state, &context, &sink,
                                                   &result));
    TEST_ASSERT_EQUAL_size_t(SYSTEM_HEALTH_MAX_SENSORS, result.sensor_count);
    TEST_ASSERT_EQUAL_size_t(1, result.resources_dropped);
    TEST_ASSERT_NOT_NULL(find_observation(observations, sink.count,
                                          "thermal.sensors_dropped", "thermal"));

    for (unsigned int index = 0; index < SYSTEM_HEALTH_MAX_SENSORS + 1U;
         ++index) {
        char directory[512];
        snprintf(directory, sizeof(directory),
                 "%s/sys/class/thermal/thermal_zone%u", root, index);
        snprintf(path, sizeof(path), "%s/type", directory);
        TEST_ASSERT_EQUAL_INT(0, unlink(path));
        snprintf(path, sizeof(path), "%s/temp", directory);
        TEST_ASSERT_EQUAL_INT(0, unlink(path));
        TEST_ASSERT_EQUAL_INT(0, rmdir(directory));
    }
    snprintf(path, sizeof(path), "%s/sys/class/thermal", root);
    TEST_ASSERT_EQUAL_INT(0, rmdir(path));
    snprintf(path, sizeof(path), "%s/sys/class", root);
    TEST_ASSERT_EQUAL_INT(0, rmdir(path));
    snprintf(path, sizeof(path), "%s/sys", root);
    TEST_ASSERT_EQUAL_INT(0, rmdir(path));
    TEST_ASSERT_EQUAL_INT(0, rmdir(root));
}

static void test_errno_capabilities_and_collector_descriptor(void) {
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED,
                      linux_thermal_capability_from_errno(EACCES));
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED,
                      linux_thermal_capability_from_errno(EPERM));
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
                      linux_thermal_capability_from_errno(ENOENT));
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_ERROR,
                      linux_thermal_capability_from_errno(EIO));

    linux_thermal_state_t state;
    linux_thermal_state_init(&state);
    system_health_collector_t collector;
    TEST_ASSERT_TRUE(linux_thermal_collector_init(&collector, &state, 60, 180));
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_TIER_NORMAL, collector.tier);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_SCOPE_HOST, collector.scope);
    TEST_ASSERT_NOT_NULL(collector.collect);
    TEST_ASSERT_FALSE(linux_thermal_collector_init(&collector, &state, 60, 30));
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_collects_zones_hwmon_and_prefers_critical_limits);
    RUN_TEST(test_malformed_and_overflow_temperature_are_errors);
    RUN_TEST(test_disappearing_sensors_become_stale_not_zero);
    RUN_TEST(test_sensor_enumeration_is_bounded_and_counts_overflow);
    RUN_TEST(test_errno_capabilities_and_collector_descriptor);
    return UNITY_END();
}
