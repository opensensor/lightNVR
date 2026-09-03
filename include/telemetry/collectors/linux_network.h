/**
 * @file linux_network.h
 * @brief Bounded namespace-visible Linux interface and route collection.
 */

#ifndef LIGHTNVR_LINUX_NETWORK_H
#define LIGHTNVR_LINUX_NETWORK_H

#include <stddef.h>
#include <stdint.h>

#include "telemetry/system_health_collector.h"

#define LINUX_NETWORK_INTERNAL_NAME_LENGTH 64U
#define LINUX_NETWORK_STATE_CAPACITY (SYSTEM_HEALTH_MAX_INTERFACES * 2U)

typedef enum {
    LINUX_NETWORK_RX_BYTES = 0,
    LINUX_NETWORK_RX_PACKETS,
    LINUX_NETWORK_RX_ERRORS,
    LINUX_NETWORK_RX_DROPS,
    LINUX_NETWORK_TX_BYTES,
    LINUX_NETWORK_TX_PACKETS,
    LINUX_NETWORK_TX_ERRORS,
    LINUX_NETWORK_TX_DROPS,
    LINUX_NETWORK_COUNTER_COUNT
} linux_network_counter_kind_t;

/** Internal sampler state. internal_name is never copied into public output. */
typedef struct {
    char internal_name[LINUX_NETWORK_INTERNAL_NAME_LENGTH];
    char id[SYSTEM_HEALTH_ID_LENGTH];
    uint64_t previous[LINUX_NETWORK_COUNTER_COUNT];
    /** Process-run monotonic export, including values observed after resets. */
    uint64_t exported_total[LINUX_NETWORK_COUNTER_COUNT];
    bool previous_valid[LINUX_NETWORK_COUNTER_COUNT];
    uint64_t previous_monotonic_ms;
    bool carrier_valid;
    bool carrier;
    uint64_t carrier_flaps;
    bool present;
    bool seen;
    uint8_t missing_cycles;
} linux_network_interface_state_t;

typedef struct {
    linux_network_interface_state_t interfaces[LINUX_NETWORK_STATE_CAPACITY];
    size_t interface_count;
    system_health_scope_t scope;
    char primary_override[LINUX_NETWORK_INTERNAL_NAME_LENGTH];
    uint64_t resources_dropped_total;
} linux_network_state_t;

typedef struct {
    char id[SYSTEM_HEALTH_ID_LENGTH];
    bool primary;
    system_health_capability_t carrier_capability;
    bool carrier_valid;
    bool carrier;
    uint64_t carrier_flaps;
    uint64_t carrier_flap_delta;
    system_health_counter_t counters[LINUX_NETWORK_COUNTER_COUNT];
    bool error_drop_ratio_valid;
    double error_drop_ratio;
} linux_network_interface_sample_t;

typedef struct {
    linux_network_interface_sample_t interfaces[SYSTEM_HEALTH_MAX_INTERFACES];
    size_t interface_count;
    size_t resources_dropped;
    system_health_capability_t capability;
    system_health_capability_t primary_capability;
    char primary_id[SYSTEM_HEALTH_ID_LENGTH];
} linux_network_result_t;

/** Initialize state for host- or container/namespace-scoped observations. */
void linux_network_state_init(linux_network_state_t *state,
                              system_health_scope_t scope);

/**
 * Set a validated kernel interface-name override. Empty clears the override.
 * The raw name remains internal and is never used as a public resource ID.
 */
bool linux_network_set_primary_override(linux_network_state_t *state,
                                        const char *interface_name);

system_health_capability_t linux_network_capability_from_errno(int error_number);

/**
 * Select an interface from <proc_root>/net/route. Up default routes are ordered
 * by longest mask prefix, lowest metric, then lexical interface name.
 */
int linux_network_select_primary(const char *proc_root,
                                 char interface_name[LINUX_NETWORK_INTERNAL_NAME_LENGTH],
                                 system_health_capability_t *capability);

int linux_network_collect(linux_network_state_t *state,
                          const system_health_collect_context_t *context,
                          system_health_observation_sink_t *sink,
                          linux_network_result_t *result);

bool linux_network_collector_init(system_health_collector_t *collector,
                                  linux_network_state_t *state,
                                  uint32_t interval_seconds,
                                  uint32_t stale_after_seconds);

#endif /* LIGHTNVR_LINUX_NETWORK_H */
