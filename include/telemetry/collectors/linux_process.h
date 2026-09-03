/** @file linux_process.h Linux process resource collector. */

#ifndef LIGHTNVR_TELEMETRY_COLLECTORS_LINUX_PROCESS_H
#define LIGHTNVR_TELEMETRY_COLLECTORS_LINUX_PROCESS_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include "telemetry/system_health_collector.h"

#define LINUX_PROCESS_DEFAULT_FD_SCAN_LIMIT 65536U
#define LINUX_PROCESS_PATH_LENGTH 512U

typedef struct {
    uint64_t value;
    system_health_capability_t capability;
} linux_process_value_t;

/**
 * Bounded, injectable inputs for one process sample.  The paths never leave
 * the collector and must not be copied into public observations or logs.
 */
typedef struct {
    const char *status_path;
    const char *fd_directory_path;
    const char *pids_max_path;
    size_t fd_scan_limit;

    bool nofile_limit_supplied;
    bool nofile_limit_unlimited;
    uint64_t nofile_limit;
    bool nproc_limit_supplied;
    bool nproc_limit_unlimited;
    uint64_t nproc_limit;
} linux_process_sample_request_t;

typedef struct {
    linux_process_value_t rss_bytes;
    linux_process_value_t thread_count;
    linux_process_value_t open_fd_count;
    linux_process_value_t effective_fd_limit;
    linux_process_value_t effective_pid_limit;
} linux_process_sample_t;

/** Optional collector state. Empty fields select context-relative defaults. */
typedef struct {
    char status_path[LINUX_PROCESS_PATH_LENGTH];
    char fd_directory_path[LINUX_PROCESS_PATH_LENGTH];
    char pids_max_path[LINUX_PROCESS_PATH_LENGTH];
    size_t fd_scan_limit;
} linux_process_collector_state_t;

int linux_process_parse_status_text(const char *text, size_t text_length,
                                    linux_process_sample_t *sample);
int linux_process_sample(const linux_process_sample_request_t *request,
                         linux_process_sample_t *sample);
int linux_process_collect(void *state,
                          const system_health_collect_context_t *context,
                          system_health_observation_sink_t *sink);
void linux_process_collector_init(system_health_collector_t *collector,
                                  linux_process_collector_state_t *state);

#endif /* LIGHTNVR_TELEMETRY_COLLECTORS_LINUX_PROCESS_H */
