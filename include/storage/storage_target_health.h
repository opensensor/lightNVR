#ifndef LIGHTNVR_STORAGE_STORAGE_TARGET_HEALTH_H
#define LIGHTNVR_STORAGE_STORAGE_TARGET_HEALTH_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include "database/db_storage_targets.h"
#include "telemetry/collectors/linux_filesystem.h"
#include "telemetry/health_helper_runner.h"

#define STORAGE_HEALTH_PROBES_PER_CYCLE 4U
#define STORAGE_HEALTH_DEFAULT_TIMEOUT_MS 3000U
#define STORAGE_HEALTH_DEFAULT_STALE_MS 900000U

typedef int (*storage_health_helper_run_fn)(
    const health_helper_request_t *request, health_helper_result_t *result);

typedef struct {
    storage_health_helper_run_fn run_helper;
    const char *stat_program;
    const char *dd_program;
    const char *rm_program;
    const char *mountinfo_path;
} storage_target_probe_ops_t;

typedef struct {
    linux_filesystem_sample_t filesystem;
    linux_filesystem_flag_t writeable;
    linux_filesystem_probe_result_t probe;
    uint64_t filesystem_device;
    bool cleanup_failed;
    double recording_growth_bps;
    system_health_capability_t recording_growth_capability;
    uint64_t sampled_monotonic_ms;
} storage_target_health_sample_t;

typedef struct {
    linux_filesystem_resource_t resource;
    char target_uuid[LIGHTNVR_UUID_STRING_SIZE];
    storage_target_health_sample_t sample;
    bool sample_valid;
} storage_target_health_slot_t;

typedef struct {
    storage_target_health_slot_t slots[SYSTEM_HEALTH_MAX_FILESYSTEMS];
    size_t slot_count;
    size_t slots_dropped;
    size_t next_probe;
    size_t max_probes_per_cycle;
    uint32_t timeout_ms;
    uint32_t stale_after_ms;
    bool write_probe_enabled;
    char root_path[LINUX_FILESYSTEM_PATH_LENGTH];
    char recording_path[LINUX_FILESYSTEM_PATH_LENGTH];
    char mountinfo_path[LINUX_FILESYSTEM_PATH_LENGTH];
    storage_target_probe_ops_t ops;
} storage_target_health_collector_state_t;

const char *storage_target_probe_error_name(
    linux_filesystem_probe_error_t error);
int storage_target_health_probe_with_ops(
    const linux_filesystem_resource_t *resource, bool write_probe,
    uint32_t timeout_ms, const storage_target_probe_ops_t *ops,
    storage_target_health_sample_t *sample);
void storage_target_health_collector_state_init(
    storage_target_health_collector_state_t *state, const char *root_path,
    const char *recording_path, bool write_probe_enabled);
int storage_target_health_collect(
    void *state, const system_health_collect_context_t *context,
    system_health_observation_sink_t *sink);
void storage_target_health_collector_init(
    system_health_collector_t *collector,
    storage_target_health_collector_state_t *state);

/* Register the DB-backed slow collector before system_health_start(). */
bool storage_target_health_register(const char *root_path,
                                    const char *recording_path,
                                    bool write_probe_enabled);

/*
 * Probe a configured target, persist its health, and publish a normalized
 * event when availability changes. Event delivery failure never changes the
 * database probe result.
 */
db_storage_target_result_t storage_target_probe_and_publish(
    const char *uuid, bool write_test, storage_target_t *target);

/* Refresh every configured target through the transition-aware probe. */
int storage_target_refresh_health_and_publish(void);

#endif /* LIGHTNVR_STORAGE_STORAGE_TARGET_HEALTH_H */
