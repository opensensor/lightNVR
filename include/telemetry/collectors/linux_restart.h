/** @file linux_restart.h Bounded boot/process identity evidence. */

#ifndef LIGHTNVR_LINUX_RESTART_HEALTH_H
#define LIGHTNVR_LINUX_RESTART_HEALTH_H

#include <stdbool.h>
#include <stdint.h>

#include "telemetry/system_health_types.h"

#define LINUX_RESTART_ID_LENGTH 37U

typedef enum {
    LINUX_RESTART_FIRST_OBSERVATION = 0,
    LINUX_RESTART_SAME_RUN,
    LINUX_RESTART_PROCESS_RESTART,
    LINUX_RESTART_HOST_REBOOT
} linux_restart_kind_t;

typedef struct {
    system_health_capability_t capability;
    char boot_id[LINUX_RESTART_ID_LENGTH];
    char run_id[LINUX_RESTART_ID_LENGTH];
    double host_uptime_seconds;
    uint64_t process_start_monotonic_ms;
} linux_restart_evidence_t;

bool linux_restart_id_valid(const char *id);
int linux_restart_read_evidence(const char *proc_root, const char *run_id,
                                uint64_t process_start_monotonic_ms,
                                linux_restart_evidence_t *evidence);
linux_restart_kind_t linux_restart_classify(
    const linux_restart_evidence_t *previous,
    const linux_restart_evidence_t *current);

#endif /* LIGHTNVR_LINUX_RESTART_HEALTH_H */
