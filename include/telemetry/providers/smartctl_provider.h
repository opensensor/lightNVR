/** @file smartctl_provider.h Optional bounded smartctl JSON provider. */

#ifndef LIGHTNVR_TELEMETRY_PROVIDERS_SMARTCTL_PROVIDER_H
#define LIGHTNVR_TELEMETRY_PROVIDERS_SMARTCTL_PROVIDER_H

#include <stdatomic.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include "telemetry/health_helper_runner.h"
#include "telemetry/providers/linux_hardware.h"

#define SMARTCTL_PROVIDER_MAX_DEVICES SYSTEM_HEALTH_MAX_DEVICES
#define SMARTCTL_PROVIDER_PATH_LENGTH 256U
#define SMARTCTL_PROVIDER_JSON_NODES_MAX 512U
#define SMARTCTL_PROVIDER_ATTRIBUTES_MAX 64U
#define SMARTCTL_PROVIDER_TIMEOUT_MS 5000U
#define SMARTCTL_PROVIDER_TERMINATE_GRACE_MS 250U

typedef enum {
    SMARTCTL_PARSE_OK = 0,
    SMARTCTL_PARSE_SLEEPING,
    SMARTCTL_PARSE_PERMISSION_DENIED,
    SMARTCTL_PARSE_UNSUPPORTED_VERSION,
    SMARTCTL_PARSE_MALFORMED
} smartctl_parse_status_t;

typedef struct {
    bool valid;
    uint64_t value;
} smartctl_u64_value_t;

typedef struct {
    bool valid;
    double value;
} smartctl_double_value_t;

/** Privacy-safe normalized subset. No input strings are retained. */
typedef struct {
    smartctl_parse_status_t status;
    system_health_capability_t capability;
    int exit_status;
    bool health_valid;
    bool health_passed;
    bool prefail;
    bool critical;
    smartctl_double_value_t temperature_celsius;
    smartctl_double_value_t available_spare_ratio;
    smartctl_double_value_t percentage_used_ratio;
    smartctl_u64_value_t reallocated_sectors;
    smartctl_u64_value_t pending_sectors;
    smartctl_u64_value_t uncorrectable_errors;
    smartctl_u64_value_t interface_crc_errors;
    smartctl_u64_value_t media_errors;
    smartctl_u64_value_t unsafe_shutdowns;
} smartctl_normalized_sample_t;

typedef int (*smartctl_helper_run_fn)(
    const health_helper_request_t *request, health_helper_result_t *result);
typedef bool (*smartctl_wake_device_tier_fn)(void);

typedef struct {
    char device_path[SMARTCTL_PROVIDER_PATH_LENGTH];
    char sysfs_name[LINUX_HARDWARE_INTERNAL_NAME_LENGTH];
    char public_id[SYSTEM_HEALTH_ID_LENGTH];
    uint64_t previous_uncorrectable;
    uint64_t previous_crc;
    uint64_t previous_media;
    uint64_t previous_unsafe_shutdowns;
    uint64_t previous_monotonic_ms;
    bool previous_uncorrectable_valid;
    bool previous_crc_valid;
    bool previous_media_valid;
    bool previous_unsafe_valid;
    bool target_managed;
} smartctl_provider_device_t;

typedef struct {
    smartctl_provider_device_t devices[SMARTCTL_PROVIDER_MAX_DEVICES];
    size_t device_count;
    size_t devices_dropped;
    bool discover_target_devices;
    char installation_scope[LINUX_HARDWARE_INSTALLATION_SCOPE_LENGTH];
    char program[SMARTCTL_PROVIDER_PATH_LENGTH];
    uint32_t timeout_ms;
    uint32_t terminate_grace_ms;
    smartctl_helper_run_fn run_helper;
    smartctl_wake_device_tier_fn wake_device_tier;
    system_health_capability_t capability;
    atomic_bool refresh_pending;
    atomic_uint_fast64_t refresh_requests;
    atomic_uint_fast64_t refresh_coalesced;
    atomic_uint_fast64_t refresh_collections;
} smartctl_provider_state_t;

void smartctl_provider_state_init(smartctl_provider_state_t *state,
                                  const char *installation_scope);

/** Add one explicitly authorized simple /dev node; physical names deduplicate. */
bool smartctl_provider_add_device(smartctl_provider_state_t *state,
                                  const char *device_path);

/** Parse one complete smartctl JSON document and matching process status. */
int smartctl_provider_parse_json(const char *json, size_t length,
                                 int process_exit_status,
                                 smartctl_normalized_sample_t *sample);

/** Coalesced nonblocking wake used after real recording I/O evidence. */
bool smartctl_provider_request_refresh(smartctl_provider_state_t *state);

int smartctl_provider_discover(void *state,
                               const system_health_collect_context_t *context,
                               system_health_provider_inventory_t *inventory);
int smartctl_provider_collect(void *state,
                              const system_health_collect_context_t *context,
                              system_health_observation_sink_t *sink);
void smartctl_provider_init(system_health_provider_t *provider,
                            smartctl_provider_state_t *state);

#endif /* LIGHTNVR_TELEMETRY_PROVIDERS_SMARTCTL_PROVIDER_H */
