/** @file linux_filesystem.h Linux filesystem health collector contracts. */

#ifndef LIGHTNVR_TELEMETRY_COLLECTORS_LINUX_FILESYSTEM_H
#define LIGHTNVR_TELEMETRY_COLLECTORS_LINUX_FILESYSTEM_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <sys/stat.h>
#include <sys/statvfs.h>

#include "telemetry/system_health_collector.h"

#define LINUX_FILESYSTEM_PATH_LENGTH 1024U
#define LINUX_FILESYSTEM_DEVICE_KEY_LENGTH 48U

typedef struct {
    uint64_t value;
    system_health_capability_t capability;
} linux_filesystem_value_t;

typedef struct {
    bool value;
    system_health_capability_t capability;
} linux_filesystem_flag_t;

/** Internal resource mapping. Only logical_id is copied to observations. */
typedef struct {
    char logical_id[SYSTEM_HEALTH_ID_LENGTH];
    char path[LINUX_FILESYSTEM_PATH_LENGTH];
    bool mount_required;
    char mount_guard_path[LINUX_FILESYSTEM_PATH_LENGTH];
    char expected_device_key[LINUX_FILESYSTEM_DEVICE_KEY_LENGTH];
} linux_filesystem_resource_t;

typedef struct {
    char logical_id[SYSTEM_HEALTH_ID_LENGTH];
    char device_key[LINUX_FILESYSTEM_DEVICE_KEY_LENGTH];
    linux_filesystem_flag_t mount_present;
    linux_filesystem_flag_t read_only;
    linux_filesystem_value_t mount_flags;
    linux_filesystem_value_t capacity_bytes;
    linux_filesystem_value_t available_bytes;
    linux_filesystem_value_t capacity_inodes;
    linux_filesystem_value_t available_inodes;
} linux_filesystem_sample_t;

typedef int (*linux_filesystem_stat_fn)(const char *path, struct stat *info);
typedef int (*linux_filesystem_statvfs_fn)(const char *path,
                                           struct statvfs *info);

typedef struct {
    linux_filesystem_stat_fn stat_path;
    linux_filesystem_statvfs_fn statvfs_path;
} linux_filesystem_ops_t;

typedef struct {
    linux_filesystem_resource_t resources[SYSTEM_HEALTH_MAX_FILESYSTEMS];
    size_t resource_count;
    size_t resources_dropped;
    char mountinfo_path[LINUX_FILESYSTEM_PATH_LENGTH];
} linux_filesystem_collector_state_t;

/** Stable categories suitable for persistence; raw errno is never exposed. */
typedef enum {
    LINUX_FILESYSTEM_PROBE_ERROR_NONE = 0,
    LINUX_FILESYSTEM_PROBE_ERROR_NOT_FOUND,
    LINUX_FILESYSTEM_PROBE_ERROR_PERMISSION,
    LINUX_FILESYSTEM_PROBE_ERROR_READ_ONLY,
    LINUX_FILESYSTEM_PROBE_ERROR_NO_SPACE,
    LINUX_FILESYSTEM_PROBE_ERROR_IO,
    LINUX_FILESYSTEM_PROBE_ERROR_TIMED_OUT,
    LINUX_FILESYSTEM_PROBE_ERROR_BUSY,
    LINUX_FILESYSTEM_PROBE_ERROR_INVALID,
    LINUX_FILESYSTEM_PROBE_ERROR_OTHER
} linux_filesystem_probe_error_t;

/** Bounded request that T11 may enqueue on its blocking-probe worker. */
typedef struct {
    char logical_id[SYSTEM_HEALTH_ID_LENGTH];
    char path[LINUX_FILESYSTEM_PATH_LENGTH];
    uint32_t timeout_ms;
    bool require_fsync;
    bool require_unlink;
} linux_filesystem_probe_request_t;

/** Completed probe result; intentionally contains no path or error string. */
typedef struct {
    char logical_id[SYSTEM_HEALTH_ID_LENGTH];
    char device_key[LINUX_FILESYSTEM_DEVICE_KEY_LENGTH];
    system_health_capability_t capability;
    linux_filesystem_probe_error_t error;
    uint32_t latency_ms;
    bool write_completed;
    bool fsync_completed;
    bool unlink_completed;
} linux_filesystem_probe_result_t;

linux_filesystem_probe_error_t linux_filesystem_normalize_errno(int error_code);
system_health_capability_t linux_filesystem_capability_from_errno(int error_code);
bool linux_filesystem_logical_id_valid(const char *logical_id);
int linux_filesystem_mountinfo_contains(const char *mountinfo_path,
                                        const char *mount_path,
                                        bool *present);
int linux_filesystem_sample_with_ops(
    const linux_filesystem_resource_t *resource, const char *mountinfo_path,
    const linux_filesystem_ops_t *ops, linux_filesystem_sample_t *sample);
int linux_filesystem_sample(const linux_filesystem_resource_t *resource,
                            const char *mountinfo_path,
                            linux_filesystem_sample_t *sample);
int linux_filesystem_collect(void *state,
                             const system_health_collect_context_t *context,
                             system_health_observation_sink_t *sink);
void linux_filesystem_collector_init(system_health_collector_t *collector,
                                     linux_filesystem_collector_state_t *state);

#endif /* LIGHTNVR_TELEMETRY_COLLECTORS_LINUX_FILESYSTEM_H */
