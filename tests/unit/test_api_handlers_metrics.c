#include <stdlib.h>
#include <stdio.h>
#include <string.h>

#include "database/db_system_health_incidents.h"
#include "telemetry/recording_io_metrics.h"
#include "telemetry/system_health.h"
#include "telemetry/system_health_evaluator.h"
#include "unity.h"

char *api_metrics_render_system_health(
    const system_health_snapshot_t *snapshot,
    const system_health_stats_t *sampler,
    const system_health_evaluator_stats_t *evaluator,
    const system_health_incident_view_t *incidents, size_t incident_count,
    const recording_io_metrics_snapshot_t *recording,
    const system_health_process_run_t *run, bool run_valid);

void setUp(void) {}
void tearDown(void) {}

static void add_value(system_health_snapshot_t *snapshot, const char *metric,
                      const char *resource, system_health_scope_t scope,
                      double value, system_health_unit_t unit) {
    system_health_observation_t observation;
    memset(&observation, 0, sizeof(observation));
    snprintf(observation.metric, sizeof(observation.metric), "%s", metric);
    snprintf(observation.resource_id, sizeof(observation.resource_id), "%s",
             resource);
    observation.scope = scope;
    system_health_observation_set_available(&observation, value, unit);
    TEST_ASSERT_TRUE(system_health_snapshot_append(snapshot, &observation));
}

static void add_unavailable(system_health_snapshot_t *snapshot,
                            const char *metric, const char *resource,
                            system_health_scope_t scope,
                            system_health_capability_t capability) {
    system_health_observation_t observation;
    memset(&observation, 0, sizeof(observation));
    snprintf(observation.metric, sizeof(observation.metric), "%s", metric);
    snprintf(observation.resource_id, sizeof(observation.resource_id), "%s",
             resource);
    observation.scope = scope;
    system_health_observation_set_unavailable(&observation, capability);
    TEST_ASSERT_TRUE(system_health_snapshot_append(snapshot, &observation));
}

static char *render_fixture(system_health_snapshot_t *snapshot) {
    system_health_stats_t sampler = {
        .initialized = true,
        .enabled = true,
        .generations_completed = 12,
        .collections_completed = 34,
        .collection_errors = 2,
        .collection_timeouts = 1,
        .overlap_skips = 3,
        .observations_dropped = 4,
        .abandoned_helpers = 1
    };
    system_health_evaluator_stats_t evaluator = {
        .active_incidents = 2,
        .pending_persistence = 1,
        .transitions = 8,
        .persistence_failures = 2,
        .persistence_retries = 1
    };
    system_health_incident_view_t incidents[2];
    memset(incidents, 0, sizeof(incidents));
    incidents[0].severity = SYSTEM_HEALTH_SEVERITY_WARNING;
    incidents[1].severity = SYSTEM_HEALTH_SEVERITY_CRITICAL;
    recording_io_metrics_snapshot_t recording;
    memset(&recording, 0, sizeof(recording));
    recording.reason_totals[RECORDING_IO_RESOURCE_RECORDING]
                           [RECORDING_IO_REASON_IO] = 7;
    system_health_process_run_t run;
    memset(&run, 0, sizeof(run));
    strcpy(run.boot_id, "11111111-1111-4111-8111-111111111111");
    run.started_at_ms = 1786991300123LL;
    return api_metrics_render_system_health(
        snapshot, &sampler, &evaluator, incidents, 2, &recording, &run, true);
}

void test_portable_families_are_rendered_without_live_sampling(void) {
    system_health_snapshot_t snapshot;
    memset(&snapshot, 0, sizeof(snapshot));
    snapshot.sequence = 42;
    add_value(&snapshot, "container.cpu.usage_ratio", "self",
              SYSTEM_HEALTH_SCOPE_CONTAINER, 0.25, SYSTEM_HEALTH_UNIT_RATIO);
    add_value(&snapshot, "container.cpu.quota_cores", "self",
              SYSTEM_HEALTH_SCOPE_CONTAINER, 2.0, SYSTEM_HEALTH_UNIT_COUNT);
    add_value(&snapshot, "host.load.1", "host", SYSTEM_HEALTH_SCOPE_HOST,
              1.5, SYSTEM_HEALTH_UNIT_COUNT);
    add_value(&snapshot, "container.memory.limit_bytes", "self",
              SYSTEM_HEALTH_SCOPE_CONTAINER, 1000, SYSTEM_HEALTH_UNIT_BYTES);
    add_value(&snapshot, "container.memory.available_bytes", "self",
              SYSTEM_HEALTH_SCOPE_CONTAINER, 400, SYSTEM_HEALTH_UNIT_BYTES);
    add_value(&snapshot, "container.memory.current_bytes", "self",
              SYSTEM_HEALTH_SCOPE_CONTAINER, 600, SYSTEM_HEALTH_UNIT_BYTES);
    add_value(&snapshot, "process.open_fds", "lightnvr",
              SYSTEM_HEALTH_SCOPE_PROCESS, 31, SYSTEM_HEALTH_UNIT_COUNT);
    add_value(&snapshot, "process.fd_limit", "lightnvr",
              SYSTEM_HEALTH_SCOPE_PROCESS, 1024, SYSTEM_HEALTH_UNIT_COUNT);
    add_value(&snapshot, "storage.filesystem.capacity_bytes", "recording",
              SYSTEM_HEALTH_SCOPE_FILESYSTEM, 10000,
              SYSTEM_HEALTH_UNIT_BYTES);
    add_value(&snapshot, "storage.filesystem.available_bytes", "recording",
              SYSTEM_HEALTH_SCOPE_FILESYSTEM, 2500,
              SYSTEM_HEALTH_UNIT_BYTES);
    add_value(&snapshot, "network.rx_packets_total", "eth0",
              SYSTEM_HEALTH_SCOPE_HOST, 123, SYSTEM_HEALTH_UNIT_COUNT);
    add_unavailable(&snapshot, "clock.synchronized", "host",
                    SYSTEM_HEALTH_SCOPE_HOST,
                    SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED);

    char *output = render_fixture(&snapshot);
    TEST_ASSERT_NOT_NULL(output);
    TEST_ASSERT_NOT_NULL(strstr(output,
        "lightnvr_health_snapshot_sequence 42\n"));
    TEST_ASSERT_NOT_NULL(strstr(output,
        "lightnvr_system_cpu_usage_ratio 0.25\n"));
    TEST_ASSERT_NOT_NULL(strstr(output,
        "lightnvr_system_load_average{period=\"1m\"} 1.5\n"));
    TEST_ASSERT_NOT_NULL(strstr(output,
        "lightnvr_system_load_ratio{period=\"1m\"} 0.75\n"));
    TEST_ASSERT_NOT_NULL(strstr(output,
        "lightnvr_system_memory_bytes{state=\"used\"} 600\n"));
    TEST_ASSERT_NOT_NULL(strstr(output,
        "lightnvr_process_open_fds 31\n"));
    TEST_ASSERT_NOT_NULL(strstr(output,
        "lightnvr_filesystem_bytes{filesystem=\"recording\",state=\"available\"} 2500\n"));
    TEST_ASSERT_NOT_NULL(strstr(output,
        "lightnvr_network_packets_total{interface=\"eth0\",direction=\"rx\"} 123\n"));
    TEST_ASSERT_NULL(strstr(output, "\nlightnvr_clock_synchronized "));
    TEST_ASSERT_NOT_NULL(strstr(output,
        "lightnvr_health_observations{scope=\"host\",capability=\"permission_denied\"} 1\n"));
    TEST_ASSERT_NOT_NULL(strstr(output,
        "lightnvr_health_incidents{severity=\"critical\"} 1\n"));
    TEST_ASSERT_NOT_NULL(strstr(output,
        "lightnvr_recording_io_failures_total{resource=\"recording\",reason=\"io\"} 7\n"));
    TEST_ASSERT_NOT_NULL(strstr(output,
        "lightnvr_process_start_time_seconds 1786991300.123\n"));
    free(output);
}

void test_device_families_are_bounded_and_labels_are_escaped(void) {
    system_health_snapshot_t snapshot;
    memset(&snapshot, 0, sizeof(snapshot));
    snapshot.sequence = 1;
    add_value(&snapshot, "storage.device.life_used_ratio", "device.abc123",
              SYSTEM_HEALTH_SCOPE_DEVICE, 0.75, SYSTEM_HEALTH_UNIT_RATIO);
    add_value(&snapshot, "storage.device.media_errors_delta", "device.abc123",
              SYSTEM_HEALTH_SCOPE_DEVICE, 2, SYSTEM_HEALTH_UNIT_COUNT);
    add_value(&snapshot, "storage.device.reallocated_sectors", "device.abc123",
              SYSTEM_HEALTH_SCOPE_DEVICE, 4, SYSTEM_HEALTH_UNIT_COUNT);
    add_value(&snapshot, "storage.device.interface_crc_errors_delta",
              "device.abc123", SYSTEM_HEALTH_SCOPE_DEVICE, 1,
              SYSTEM_HEALTH_UNIT_COUNT);
    add_value(&snapshot, "hardware.ecc.uncorrectable_delta", "edac\"0",
              SYSTEM_HEALTH_SCOPE_HOST, 1, SYSTEM_HEALTH_UNIT_COUNT);
    add_value(&snapshot, "hardware.fan.rpm", "fan0",
              SYSTEM_HEALTH_SCOPE_HOST, 2200, SYSTEM_HEALTH_UNIT_COUNT);
    add_value(&snapshot, "kernel.block_io_error_delta", "kernel_log",
              SYSTEM_HEALTH_SCOPE_HOST, 3, SYSTEM_HEALTH_UNIT_COUNT);
    add_unavailable(&snapshot, "hardware.provider.visible", "nvme",
                    SYSTEM_HEALTH_SCOPE_DEVICE,
                    SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED);

    char *first = render_fixture(&snapshot);
    char *second = render_fixture(&snapshot);
    TEST_ASSERT_NOT_NULL(first);
    TEST_ASSERT_NOT_NULL(second);
    TEST_ASSERT_EQUAL_STRING(first, second);
    TEST_ASSERT_NOT_NULL(strstr(first,
        "lightnvr_device_wear_ratio{device=\"device.abc123\",source=\"life_used\"} 0.75\n"));
    TEST_ASSERT_NOT_NULL(strstr(first,
        "lightnvr_device_events_delta{device=\"device.abc123\",event=\"media_error\"} 2\n"));
    TEST_ASSERT_NOT_NULL(strstr(first,
        "lightnvr_device_smart_attribute{device=\"device.abc123\",attribute=\"reallocated_sectors\"} 4\n"));
    TEST_ASSERT_NOT_NULL(strstr(first,
        "lightnvr_device_events_delta{device=\"device.abc123\",event=\"interface_crc_error\"} 1\n"));
    TEST_ASSERT_NOT_NULL(strstr(first,
        "lightnvr_hardware_ecc_errors_delta{resource=\"edac\\\"0\",correctability=\"uncorrectable\"} 1\n"));
    TEST_ASSERT_NOT_NULL(strstr(first,
        "lightnvr_kernel_events_delta{event=\"block_io_error\"} 3\n"));
    TEST_ASSERT_NOT_NULL(strstr(first,
        "lightnvr_health_collector_up{collector=\"nvme\",scope=\"device\"} 0\n"));
    free(first);
    free(second);
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_portable_families_are_rendered_without_live_sampling);
    RUN_TEST(test_device_families_are_bounded_and_labels_are_escaped);
    return UNITY_END();
}
