#define _POSIX_C_SOURCE 200809L

#include "unity.h"

#include "telemetry/collectors/linux_network.h"

#include <ctype.h>
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <unistd.h>

#ifndef TEST_NETWORK_FIXTURE_DIR
#define TEST_NETWORK_FIXTURE_DIR "tests/fixtures/system_health/sysfs/net"
#endif

void setUp(void) {}
void tearDown(void) {}

static void fixture_root(char *output, size_t output_size,
                         const char *fixture, const char *root) {
    snprintf(output, output_size, "%s/%s/%s", TEST_NETWORK_FIXTURE_DIR,
             fixture, root);
}

static system_health_collect_context_t context_for(
    const char *fixture, uint64_t monotonic_ms, char *sys_root,
    size_t sys_root_size, char *proc_root, size_t proc_root_size) {
    fixture_root(sys_root, sys_root_size, fixture, "sys");
    fixture_root(proc_root, proc_root_size, fixture, "proc");
    system_health_collect_context_t context = {
        .monotonic_ms = monotonic_ms,
        .wall_time_ms = 1700000000000LL + (int64_t)monotonic_ms,
        .sys_root = sys_root,
        .proc_root = proc_root
    };
    return context;
}

static const linux_network_interface_sample_t *primary_sample(
    const linux_network_result_t *result) {
    for (size_t index = 0; index < result->interface_count; ++index) {
        if (result->interfaces[index].primary) return &result->interfaces[index];
    }
    return NULL;
}

static const linux_network_interface_sample_t *non_primary_sample(
    const linux_network_result_t *result) {
    for (size_t index = 0; index < result->interface_count; ++index) {
        if (!result->interfaces[index].primary) return &result->interfaces[index];
    }
    return NULL;
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

static void assert_public_id(const char *id) {
    TEST_ASSERT_TRUE(id && id[0]);
    TEST_ASSERT_NULL(strchr(id, '/'));
    TEST_ASSERT_NULL(strchr(id, ':'));
    TEST_ASSERT_NULL(strchr(id, '@'));
    for (const unsigned char *cursor = (const unsigned char *)id;
         *cursor; ++cursor) {
        TEST_ASSERT_TRUE(isalnum(*cursor) || *cursor == '.' || *cursor == '_');
    }
}

static void test_primary_route_order_is_deterministic(void) {
    char proc_root[512];
    char selected[LINUX_NETWORK_INTERNAL_NAME_LENGTH];
    system_health_capability_t capability;

    fixture_root(proc_root, sizeof(proc_root), "routes-longest", "proc");
    TEST_ASSERT_EQUAL_INT(0, linux_network_select_primary(
        proc_root, selected, &capability));
    TEST_ASSERT_EQUAL_STRING("long0", selected);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_AVAILABLE, capability);

    fixture_root(proc_root, sizeof(proc_root), "routes-metric", "proc");
    TEST_ASSERT_EQUAL_INT(0, linux_network_select_primary(
        proc_root, selected, &capability));
    TEST_ASSERT_EQUAL_STRING("cheap0", selected);

    fixture_root(proc_root, sizeof(proc_root), "routes-tie", "proc");
    TEST_ASSERT_EQUAL_INT(0, linux_network_select_primary(
        proc_root, selected, &capability));
    TEST_ASSERT_EQUAL_STRING("alpha0", selected);

    fixture_root(proc_root, sizeof(proc_root), "routes-malformed", "proc");
    TEST_ASSERT_EQUAL_INT(-1, linux_network_select_primary(
        proc_root, selected, &capability));
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_ERROR, capability);
}

static void test_counters_have_safe_deltas_ratio_carrier_and_scope(void) {
    linux_network_state_t state;
    linux_network_state_init(&state, SYSTEM_HEALTH_SCOPE_CONTAINER);
    system_health_observation_t observations[256];
    system_health_observation_sink_t sink = {
        .items = observations, .capacity = 256, .count = 0, .dropped = 0
    };
    char sys_root[512], proc_root[512];
    system_health_collect_context_t context = context_for(
        "sample1", 1000, sys_root, sizeof(sys_root), proc_root,
        sizeof(proc_root));
    linux_network_result_t first;
    TEST_ASSERT_EQUAL_INT(0, linux_network_collect(&state, &context, &sink,
                                                   &first));
    TEST_ASSERT_EQUAL_size_t(2, first.interface_count);
    const linux_network_interface_sample_t *primary = primary_sample(&first);
    const linux_network_interface_sample_t *virtual_interface =
        non_primary_sample(&first);
    TEST_ASSERT_NOT_NULL(primary);
    TEST_ASSERT_NOT_NULL(virtual_interface);
    assert_public_id(primary->id);
    TEST_ASSERT_TRUE(primary->carrier_valid);
    TEST_ASSERT_TRUE(primary->carrier);
    TEST_ASSERT_FALSE(primary->counters[LINUX_NETWORK_RX_BYTES].delta_valid);
    TEST_ASSERT_FALSE(primary->error_drop_ratio_valid);
    TEST_ASSERT_FALSE(virtual_interface->carrier_valid);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
                      virtual_interface->carrier_capability);
    const system_health_observation_t *carrier = find_observation(
        observations, sink.count, "network.carrier", primary->id);
    TEST_ASSERT_NOT_NULL(carrier);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_SCOPE_CONTAINER, carrier->scope);

    context = context_for("sample2", 2000, sys_root, sizeof(sys_root),
                          proc_root, sizeof(proc_root));
    sink.count = 0;
    sink.dropped = 0;
    linux_network_result_t second;
    TEST_ASSERT_EQUAL_INT(0, linux_network_collect(&state, &context, &sink,
                                                   &second));
    primary = primary_sample(&second);
    TEST_ASSERT_NOT_NULL(primary);
    TEST_ASSERT_FALSE(primary->carrier);
    TEST_ASSERT_EQUAL_UINT64(1, primary->carrier_flap_delta);
    TEST_ASSERT_TRUE(primary->counters[LINUX_NETWORK_RX_BYTES].delta_valid);
    TEST_ASSERT_EQUAL_UINT64(600,
        primary->counters[LINUX_NETWORK_RX_BYTES].delta);
    TEST_ASSERT_EQUAL_UINT64(1000,
        primary->counters[LINUX_NETWORK_RX_BYTES].interval_ms);
    TEST_ASSERT_EQUAL_UINT64(6,
        primary->counters[LINUX_NETWORK_RX_PACKETS].delta);
    TEST_ASSERT_TRUE(primary->error_drop_ratio_valid);
    TEST_ASSERT_FLOAT_WITHIN(0.00001f, 2.0f / 12.0f,
                             (float)primary->error_drop_ratio);
}

static void test_reset_malformed_and_disappearance_never_make_false_rates(void) {
    linux_network_state_t state;
    linux_network_state_init(&state, SYSTEM_HEALTH_SCOPE_HOST);
    system_health_observation_t observations[256];
    system_health_observation_sink_t sink = {
        .items = observations, .capacity = 256, .count = 0, .dropped = 0
    };
    char sys_root[512], proc_root[512];
    system_health_collect_context_t context = context_for(
        "sample1", 1000, sys_root, sizeof(sys_root), proc_root,
        sizeof(proc_root));
    TEST_ASSERT_EQUAL_INT(0, linux_network_collect(&state, &context, &sink,
                                                   NULL));

    context = context_for("reset", 2000, sys_root, sizeof(sys_root), proc_root,
                          sizeof(proc_root));
    sink.count = 0;
    linux_network_result_t reset;
    TEST_ASSERT_EQUAL_INT(0, linux_network_collect(&state, &context, &sink,
                                                   &reset));
    const linux_network_interface_sample_t *primary = primary_sample(&reset);
    TEST_ASSERT_NOT_NULL(primary);
    TEST_ASSERT_TRUE(primary->counters[LINUX_NETWORK_RX_BYTES].reset_detected);
    TEST_ASSERT_FALSE(primary->counters[LINUX_NETWORK_RX_BYTES].delta_valid);
    TEST_ASSERT_FALSE(primary->error_drop_ratio_valid);
    const system_health_observation_t *reset_observation = find_observation(
        observations, sink.count, "network.counter_reset", primary->id);
    TEST_ASSERT_NOT_NULL(reset_observation);
    TEST_ASSERT_EQUAL_FLOAT(1.0f, (float)reset_observation->value);
    const system_health_observation_t *monotonic_rx = find_observation(
        observations, sink.count, "network.rx_bytes_total", primary->id);
    TEST_ASSERT_NOT_NULL(monotonic_rx);
    TEST_ASSERT_TRUE(monotonic_rx->value_valid);
    TEST_ASSERT_EQUAL_UINT64(1005U, (uint64_t)monotonic_rx->value);

    linux_network_state_init(&state, SYSTEM_HEALTH_SCOPE_HOST);
    context = context_for("malformed", 3000, sys_root, sizeof(sys_root),
                          proc_root, sizeof(proc_root));
    sink.count = 0;
    linux_network_result_t malformed;
    TEST_ASSERT_EQUAL_INT(0, linux_network_collect(&state, &context, &sink,
                                                   &malformed));
    primary = primary_sample(&malformed);
    TEST_ASSERT_NOT_NULL(primary);
    TEST_ASSERT_FALSE(primary->carrier_valid);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_ERROR,
                      primary->carrier_capability);
    const system_health_observation_t *rx_bytes = find_observation(
        observations, sink.count, "network.rx_bytes_total", primary->id);
    TEST_ASSERT_NOT_NULL(rx_bytes);
    TEST_ASSERT_FALSE(rx_bytes->value_valid);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_ERROR, rx_bytes->capability);

    linux_network_state_init(&state, SYSTEM_HEALTH_SCOPE_HOST);
    context = context_for("sample1", 4000, sys_root, sizeof(sys_root),
                          proc_root, sizeof(proc_root));
    sink.count = 0;
    TEST_ASSERT_EQUAL_INT(0, linux_network_collect(&state, &context, &sink,
                                                   NULL));
    context = context_for("missing", 5000, sys_root, sizeof(sys_root), proc_root,
                          sizeof(proc_root));
    sink.count = 0;
    linux_network_result_t missing;
    TEST_ASSERT_EQUAL_INT(0, linux_network_collect(&state, &context, &sink,
                                                   &missing));
    size_t stale_carriers = 0;
    for (size_t index = 0; index < sink.count; ++index) {
        if (strcmp(observations[index].metric, "network.carrier") == 0) {
            stale_carriers++;
            TEST_ASSERT_FALSE(observations[index].value_valid);
            TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_STALE,
                              observations[index].capability);
        }
    }
    TEST_ASSERT_EQUAL_size_t(2, stale_carriers);
}

static void write_text_file(const char *path, const char *text) {
    FILE *file = fopen(path, "w");
    TEST_ASSERT_NOT_NULL(file);
    TEST_ASSERT_EQUAL_size_t(strlen(text), fwrite(text, 1, strlen(text), file));
    TEST_ASSERT_EQUAL_INT(0, fclose(file));
}

static void test_interface_enumeration_is_bounded_and_primary_override_is_safe(void) {
    char root[] = "/tmp/lightnvr-network-overflow-XXXXXX";
    TEST_ASSERT_NOT_NULL(mkdtemp(root));
    char path[512];
    snprintf(path, sizeof(path), "%s/sys", root);
    TEST_ASSERT_EQUAL_INT(0, mkdir(path, 0700));
    snprintf(path, sizeof(path), "%s/sys/class", root);
    TEST_ASSERT_EQUAL_INT(0, mkdir(path, 0700));
    snprintf(path, sizeof(path), "%s/sys/class/net", root);
    TEST_ASSERT_EQUAL_INT(0, mkdir(path, 0700));
    snprintf(path, sizeof(path), "%s/proc", root);
    TEST_ASSERT_EQUAL_INT(0, mkdir(path, 0700));
    snprintf(path, sizeof(path), "%s/proc/net", root);
    TEST_ASSERT_EQUAL_INT(0, mkdir(path, 0700));
    snprintf(path, sizeof(path), "%s/proc/net/route", root);
    write_text_file(path,
        "Iface Destination Gateway Flags RefCnt Use Metric Mask MTU Window IRTT\n"
        "if16 00000000 0100000A 0003 0 0 1 00000000 0 0 0\n");
    for (unsigned int index = 0; index < SYSTEM_HEALTH_MAX_INTERFACES + 1U;
         ++index) {
        char directory[512];
        snprintf(directory, sizeof(directory), "%s/sys/class/net/if%02u", root,
                 index);
        TEST_ASSERT_EQUAL_INT(0, mkdir(directory, 0700));
        snprintf(path, sizeof(path), "%s/carrier", directory);
        write_text_file(path, "1\n");
    }

    linux_network_state_t state;
    linux_network_state_init(&state, SYSTEM_HEALTH_SCOPE_CONTAINER);
    TEST_ASSERT_FALSE(linux_network_set_primary_override(&state, "../eth0"));
    system_health_observation_t observations[256];
    system_health_observation_sink_t sink = {
        .items = observations, .capacity = 256, .count = 0, .dropped = 0
    };
    char sys_root[512], proc_root[512];
    snprintf(sys_root, sizeof(sys_root), "%s/sys", root);
    snprintf(proc_root, sizeof(proc_root), "%s/proc", root);
    system_health_collect_context_t context = {
        .monotonic_ms = 1, .wall_time_ms = 1,
        .sys_root = sys_root, .proc_root = proc_root
    };
    linux_network_result_t result;
    TEST_ASSERT_EQUAL_INT(0, linux_network_collect(&state, &context, &sink,
                                                   &result));
    TEST_ASSERT_EQUAL_size_t(SYSTEM_HEALTH_MAX_INTERFACES,
                             result.interface_count);
    TEST_ASSERT_EQUAL_size_t(1, result.resources_dropped);
    TEST_ASSERT_NOT_NULL(primary_sample(&result));
    TEST_ASSERT_NOT_NULL(find_observation(observations, sink.count,
                                          "network.interfaces_dropped",
                                          "network"));

    for (unsigned int index = 0; index < SYSTEM_HEALTH_MAX_INTERFACES + 1U;
         ++index) {
        char directory[512];
        snprintf(directory, sizeof(directory), "%s/sys/class/net/if%02u", root,
                 index);
        snprintf(path, sizeof(path), "%s/carrier", directory);
        TEST_ASSERT_EQUAL_INT(0, unlink(path));
        TEST_ASSERT_EQUAL_INT(0, rmdir(directory));
    }
    snprintf(path, sizeof(path), "%s/proc/net/route", root);
    TEST_ASSERT_EQUAL_INT(0, unlink(path));
    snprintf(path, sizeof(path), "%s/proc/net", root);
    TEST_ASSERT_EQUAL_INT(0, rmdir(path));
    snprintf(path, sizeof(path), "%s/proc", root);
    TEST_ASSERT_EQUAL_INT(0, rmdir(path));
    snprintf(path, sizeof(path), "%s/sys/class/net", root);
    TEST_ASSERT_EQUAL_INT(0, rmdir(path));
    snprintf(path, sizeof(path), "%s/sys/class", root);
    TEST_ASSERT_EQUAL_INT(0, rmdir(path));
    snprintf(path, sizeof(path), "%s/sys", root);
    TEST_ASSERT_EQUAL_INT(0, rmdir(path));
    TEST_ASSERT_EQUAL_INT(0, rmdir(root));
}

static void test_override_capabilities_and_descriptor(void) {
    linux_network_state_t state;
    linux_network_state_init(&state, SYSTEM_HEALTH_SCOPE_CONTAINER);
    TEST_ASSERT_TRUE(linux_network_set_primary_override(&state, "veth0"));
    system_health_observation_t observations[256];
    system_health_observation_sink_t sink = {
        .items = observations, .capacity = 256, .count = 0, .dropped = 0
    };
    char sys_root[512], proc_root[512];
    system_health_collect_context_t context = context_for(
        "sample1", 1, sys_root, sizeof(sys_root), proc_root,
        sizeof(proc_root));
    linux_network_result_t result;
    TEST_ASSERT_EQUAL_INT(0, linux_network_collect(&state, &context, &sink,
                                                   &result));
    const linux_network_interface_sample_t *primary = primary_sample(&result);
    TEST_ASSERT_NOT_NULL(primary);
    TEST_ASSERT_FALSE(primary->carrier_valid);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
                      primary->carrier_capability);
    assert_public_id(primary->id);

    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED,
                      linux_network_capability_from_errno(EACCES));
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
                      linux_network_capability_from_errno(ENOENT));
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_ERROR,
                      linux_network_capability_from_errno(EIO));
    system_health_collector_t collector;
    TEST_ASSERT_TRUE(linux_network_collector_init(&collector, &state, 60, 180));
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_SCOPE_CONTAINER, collector.scope);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_TIER_NORMAL, collector.tier);
    TEST_ASSERT_FALSE(linux_network_collector_init(&collector, &state, 60, 30));
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_primary_route_order_is_deterministic);
    RUN_TEST(test_counters_have_safe_deltas_ratio_carrier_and_scope);
    RUN_TEST(test_reset_malformed_and_disappearance_never_make_false_rates);
    RUN_TEST(test_interface_enumeration_is_bounded_and_primary_override_is_safe);
    RUN_TEST(test_override_capabilities_and_descriptor);
    return UNITY_END();
}
