/** @file nvme_provider.h Read-only bounded NVMe health-log provider. */

#ifndef LIGHTNVR_TELEMETRY_PROVIDERS_NVME_PROVIDER_H
#define LIGHTNVR_TELEMETRY_PROVIDERS_NVME_PROVIDER_H

#include <stddef.h>
#include <stdint.h>

#include "telemetry/providers/linux_hardware.h"

#define NVME_PROVIDER_MAX_DEVICES SYSTEM_HEALTH_MAX_DEVICES
#define NVME_PROVIDER_HEALTH_LOG_SIZE 512U

typedef int (*nvme_provider_open_fn)(const char *path, int flags);
typedef int (*nvme_provider_health_log_fn)(int descriptor,
                                           uint8_t *output,
                                           size_t output_size);
typedef int (*nvme_provider_close_fn)(int descriptor);

typedef struct {
    nvme_provider_open_fn open_device;
    nvme_provider_health_log_fn read_health_log;
    nvme_provider_close_fn close_device;
} nvme_provider_ops_t;

typedef struct {
    char internal_name[LINUX_HARDWARE_INTERNAL_NAME_LENGTH];
    char public_id[SYSTEM_HEALTH_ID_LENGTH];
    uint64_t previous_media_errors;
    uint64_t previous_unsafe_shutdowns;
    uint64_t previous_monotonic_ms;
    bool media_previous_valid;
    bool unsafe_previous_valid;
} nvme_provider_device_t;

typedef struct {
    nvme_provider_device_t devices[NVME_PROVIDER_MAX_DEVICES];
    size_t device_count;
    size_t devices_dropped;
    system_health_capability_t discovery_capability;
    char installation_scope[LINUX_HARDWARE_INSTALLATION_SCOPE_LENGTH];
    char dev_root[256];
    char discovered_sys_root[1024];
    nvme_provider_ops_t ops;
} nvme_provider_state_t;

void nvme_provider_state_init(nvme_provider_state_t *state,
                              const char *installation_scope);
int nvme_provider_discover(void *state,
                           const system_health_collect_context_t *context,
                           system_health_provider_inventory_t *inventory);
int nvme_provider_collect(void *state,
                          const system_health_collect_context_t *context,
                          system_health_observation_sink_t *sink);
void nvme_provider_init(system_health_provider_t *provider,
                        nvme_provider_state_t *state);

#endif /* LIGHTNVR_TELEMETRY_PROVIDERS_NVME_PROVIDER_H */
