/** @file kernel_log.h Optional nonblocking, privacy-safe kernel log provider. */

#ifndef LIGHTNVR_TELEMETRY_PROVIDERS_KERNEL_LOG_H
#define LIGHTNVR_TELEMETRY_PROVIDERS_KERNEL_LOG_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <sys/types.h>

#include "telemetry/system_health_provider.h"

#define KERNEL_LOG_PATH_LENGTH 256U
#define KERNEL_LOG_READ_BUFFER 4096U
#define KERNEL_LOG_READS_PER_CYCLE 64U
#define KERNEL_LOG_DEDUPE_SLOTS 64U

typedef enum {
    KERNEL_LOG_FILESYSTEM_REMOUNT = 0,
    KERNEL_LOG_BLOCK_IO_ERROR,
    KERNEL_LOG_MACHINE_CHECK,
    KERNEL_LOG_THERMAL_SHUTDOWN,
    KERNEL_LOG_OOM_KILL,
    KERNEL_LOG_CATEGORY_COUNT
} kernel_log_category_t;

typedef int (*kernel_log_open_fn)(const char *path, int flags);
typedef ssize_t (*kernel_log_read_fn)(int descriptor, void *buffer,
                                      size_t size);
typedef int (*kernel_log_close_fn)(int descriptor);

typedef struct {
    kernel_log_open_fn open_log;
    kernel_log_read_fn read_log;
    kernel_log_close_fn close_log;
} kernel_log_ops_t;

typedef struct {
    int descriptor;
    char path[KERNEL_LOG_PATH_LENGTH];
    kernel_log_ops_t ops;
    system_health_capability_t capability;
    uint64_t last_sequence;
    bool sequence_valid;
    uint64_t recent_hashes[KERNEL_LOG_DEDUPE_SLOTS];
    size_t recent_count;
    size_t recent_next;
} kernel_log_state_t;

void kernel_log_state_init(kernel_log_state_t *state, const char *path);
int kernel_log_classify_line(const char *line, size_t length,
                             bool matches[KERNEL_LOG_CATEGORY_COUNT],
                             uint64_t *sequence, bool *sequence_valid);
int kernel_log_discover(void *state,
                        const system_health_collect_context_t *context,
                        system_health_provider_inventory_t *inventory);
int kernel_log_collect(void *state,
                       const system_health_collect_context_t *context,
                       system_health_observation_sink_t *sink);
void kernel_log_destroy(void *state);
void kernel_log_provider_init(system_health_provider_t *provider,
                              kernel_log_state_t *state);

#endif /* LIGHTNVR_TELEMETRY_PROVIDERS_KERNEL_LOG_H */
