/** @file linux_hardware.h Bounded Linux sysfs hardware health provider. */

#ifndef LIGHTNVR_TELEMETRY_PROVIDERS_LINUX_HARDWARE_H
#define LIGHTNVR_TELEMETRY_PROVIDERS_LINUX_HARDWARE_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include "telemetry/system_health_provider.h"

#define LINUX_HARDWARE_MAX_RESOURCES SYSTEM_HEALTH_MAX_DEVICES
#define LINUX_HARDWARE_INTERNAL_NAME_LENGTH 64U
#define LINUX_HARDWARE_INSTALLATION_SCOPE_LENGTH 96U
#define LINUX_HARDWARE_DEVICE_MAP_MAX SYSTEM_HEALTH_MAX_DEVICES
#define LINUX_HARDWARE_COUNTER_SLOTS 32U

typedef struct {
    char sysfs_name[LINUX_HARDWARE_INTERNAL_NAME_LENGTH];
    char public_id[SYSTEM_HEALTH_ID_LENGTH];
    uint64_t filesystem_device;
    bool target_mapped;
    bool target_is_default;
} linux_device_map_entry_t;

typedef struct {
    linux_device_map_entry_t entries[LINUX_HARDWARE_DEVICE_MAP_MAX];
    size_t count;
    size_t dropped;
    system_health_capability_t capability;
} linux_device_map_t;

typedef enum {
    LINUX_HARDWARE_RESOURCE_FLASH = 0,
    LINUX_HARDWARE_RESOURCE_EDAC,
    LINUX_HARDWARE_RESOURCE_FAN,
    LINUX_HARDWARE_RESOURCE_BOARD
} linux_hardware_resource_kind_t;

typedef struct {
    linux_hardware_resource_kind_t kind;
    char internal_name[LINUX_HARDWARE_INTERNAL_NAME_LENGTH];
    char public_id[SYSTEM_HEALTH_ID_LENGTH];
    unsigned int channel;
} linux_hardware_resource_t;

typedef struct {
    bool used;
    char public_id[SYSTEM_HEALTH_ID_LENGTH];
    char metric[SYSTEM_HEALTH_METRIC_LENGTH];
    uint64_t previous;
    uint64_t previous_monotonic_ms;
} linux_hardware_counter_state_t;

typedef struct {
    linux_hardware_resource_t resources[LINUX_HARDWARE_MAX_RESOURCES];
    size_t resource_count;
    size_t resources_dropped;
    system_health_capability_t discovery_capability;
    char installation_scope[LINUX_HARDWARE_INSTALLATION_SCOPE_LENGTH];
    char discovered_sys_root[1024];
    double fan_hot_celsius;
    linux_hardware_counter_state_t counters[LINUX_HARDWARE_COUNTER_SLOTS];
} linux_hardware_state_t;

/** Build a target-aware, physical-device-deduplicated block-device map. */
int linux_device_map_build(const char *sys_root, const char *installation_scope,
                           linux_device_map_t *map);
const linux_device_map_entry_t *linux_device_map_find(
    const linux_device_map_t *map, const char *physical_name);

void linux_hardware_state_init(linux_hardware_state_t *state,
                               const char *installation_scope);
int linux_hardware_discover(void *state,
                            const system_health_collect_context_t *context,
                            system_health_provider_inventory_t *inventory);
int linux_hardware_collect(void *state,
                           const system_health_collect_context_t *context,
                           system_health_observation_sink_t *sink);
void linux_hardware_provider_init(system_health_provider_t *provider,
                                  linux_hardware_state_t *state);

#endif /* LIGHTNVR_TELEMETRY_PROVIDERS_LINUX_HARDWARE_H */
