#define _POSIX_C_SOURCE 200809L

#include "storage/storage_target_health.h"

#include <limits.h>
#include <ctype.h>
#include <errno.h>
#include <pthread.h>
#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <sys/stat.h>
#include <time.h>
#include <unistd.h>

#include "core/event_producers.h"
#include "core/logger.h"
#include "telemetry/system_health.h"
#include "utils/strings.h"

static pthread_mutex_t target_probe_mutex = PTHREAD_MUTEX_INITIALIZER;
static storage_target_health_collector_state_t registered_state;
static bool registered_state_initialized;

static void publish_transition(const storage_target_t *before,
                               const storage_target_t *after);

static uint64_t monotonic_ms(void) {
    struct timespec value;
    if (clock_gettime(CLOCK_MONOTONIC, &value) != 0) return 0;
    return (uint64_t)value.tv_sec * 1000U +
           (uint64_t)value.tv_nsec / 1000000U;
}

const char *storage_target_probe_error_name(
    linux_filesystem_probe_error_t error) {
    switch (error) {
        case LINUX_FILESYSTEM_PROBE_ERROR_NONE: return "none";
        case LINUX_FILESYSTEM_PROBE_ERROR_NOT_FOUND: return "not_found";
        case LINUX_FILESYSTEM_PROBE_ERROR_PERMISSION: return "permission";
        case LINUX_FILESYSTEM_PROBE_ERROR_READ_ONLY: return "read_only";
        case LINUX_FILESYSTEM_PROBE_ERROR_NO_SPACE: return "no_space";
        case LINUX_FILESYSTEM_PROBE_ERROR_IO: return "io";
        case LINUX_FILESYSTEM_PROBE_ERROR_TIMED_OUT: return "timed_out";
        case LINUX_FILESYSTEM_PROBE_ERROR_BUSY: return "busy";
        case LINUX_FILESYSTEM_PROBE_ERROR_INVALID: return "invalid";
        case LINUX_FILESYSTEM_PROBE_ERROR_OTHER: return "other";
    }
    return "other";
}

static const char *select_program(const char *preferred,
                                  const char *fallback) {
    if (preferred && access(preferred, X_OK) == 0) return preferred;
    return fallback;
}

static linux_filesystem_probe_error_t output_error(
    const health_helper_result_t *result) {
    if (!result) return LINUX_FILESYSTEM_PROBE_ERROR_OTHER;
    if (result->outcome == HEALTH_HELPER_TIMED_OUT)
        return LINUX_FILESYSTEM_PROBE_ERROR_TIMED_OUT;
    if (result->outcome == HEALTH_HELPER_BUSY)
        return LINUX_FILESYSTEM_PROBE_ERROR_BUSY;
    if (result->outcome == HEALTH_HELPER_EXEC_ERROR)
        return LINUX_FILESYSTEM_PROBE_ERROR_INVALID;
    if (strstr(result->output, "Read-only file system"))
        return LINUX_FILESYSTEM_PROBE_ERROR_READ_ONLY;
    if (strstr(result->output, "No space left on device") ||
        strstr(result->output, "Disk quota exceeded"))
        return LINUX_FILESYSTEM_PROBE_ERROR_NO_SPACE;
    if (strstr(result->output, "Input/output error"))
        return LINUX_FILESYSTEM_PROBE_ERROR_IO;
    if (strstr(result->output, "Permission denied") ||
        strstr(result->output, "Operation not permitted"))
        return LINUX_FILESYSTEM_PROBE_ERROR_PERMISSION;
    if (strstr(result->output, "No such file") ||
        strstr(result->output, "not found"))
        return LINUX_FILESYSTEM_PROBE_ERROR_NOT_FOUND;
    return LINUX_FILESYSTEM_PROBE_ERROR_OTHER;
}

static system_health_capability_t helper_capability(
    const health_helper_result_t *result) {
    if (!result) return SYSTEM_HEALTH_CAPABILITY_ERROR;
    if (result->outcome == HEALTH_HELPER_BUSY)
        return SYSTEM_HEALTH_CAPABILITY_STALE;
    if (result->outcome == HEALTH_HELPER_EXEC_ERROR)
        return SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
    if (result->outcome == HEALTH_HELPER_OK)
        return SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    return SYSTEM_HEALTH_CAPABILITY_ERROR;
}

static bool invoke_helper(const storage_target_probe_ops_t *ops,
                          const health_helper_request_t *request,
                          health_helper_result_t *result) {
    memset(result, 0, sizeof(*result));
    if (ops->run_helper(request, result) != 0) {
        memset(result, 0, sizeof(*result));
        result->outcome = HEALTH_HELPER_SYSTEM_ERROR;
        return false;
    }
    return result->outcome == HEALTH_HELPER_OK;
}

static void saturating_add_latency(uint32_t *latency, uint32_t addition) {
    if (UINT32_MAX - *latency < addition) *latency = UINT32_MAX;
    else *latency += addition;
}

static bool path_within(const char *path, const char *mount) {
    size_t length = strlen(mount);
    if (strcmp(mount, "/") == 0) return path[0] == '/';
    return strncmp(path, mount, length) == 0 &&
           (path[length] == '\0' || path[length] == '/');
}

static bool decode_mount_path(const char *encoded, char *decoded,
                              size_t capacity) {
    size_t output = 0U;
    for (size_t input = 0U; encoded[input]; ++input) {
        unsigned char value = (unsigned char)encoded[input];
        if (value == '\\' && encoded[input + 1] && encoded[input + 2] &&
            encoded[input + 3] &&
            isdigit((unsigned char)encoded[input + 1]) &&
            isdigit((unsigned char)encoded[input + 2]) &&
            isdigit((unsigned char)encoded[input + 3])) {
            value = (unsigned char)((encoded[input + 1] - '0') * 64 +
                                    (encoded[input + 2] - '0') * 8 +
                                    encoded[input + 3] - '0');
            input += 3U;
        }
        if (value == 0U || output + 1U >= capacity) return false;
        decoded[output++] = (char)value;
    }
    decoded[output] = '\0';
    return true;
}

static int mount_facts(const linux_filesystem_resource_t *resource,
                       const char *mountinfo_path, bool *mounted,
                       bool *read_only) {
    if (!resource || !mountinfo_path || !mounted || !read_only) return -1;
    FILE *input = fopen(mountinfo_path, "r");
    if (!input) return -1;
    char *line = NULL;
    size_t line_capacity = 0U;
    size_t best = 0U;
    bool exact_guard = false;
    *mounted = false;
    *read_only = false;
    while (getline(&line, &line_capacity, input) >= 0) {
        char encoded[LINUX_FILESYSTEM_PATH_LENGTH];
        char options[256];
        if (sscanf(line, "%*s %*s %*s %*s %1023s %255s",
                   encoded, options) != 2) continue;
        char mount[LINUX_FILESYSTEM_PATH_LENGTH];
        if (!decode_mount_path(encoded, mount, sizeof(mount))) continue;
        if (resource->mount_required && resource->mount_guard_path[0] &&
            strcmp(mount, resource->mount_guard_path) == 0)
            exact_guard = true;
        size_t length = strlen(mount);
        if (length < best || !path_within(resource->path, mount)) continue;
        best = length;
        *mounted = true;
        *read_only = strcmp(options, "ro") == 0 ||
                     strncmp(options, "ro,", 3) == 0;
    }
    int saved_errno = errno;
    free(line);
    bool failed = ferror(input) != 0;
    fclose(input);
    errno = saved_errno;
    if (failed) return -1;
    if (resource->mount_required) {
        const char *guard = resource->mount_guard_path[0]
            ? resource->mount_guard_path : resource->path;
        if (resource->mount_guard_path[0]) *mounted = exact_guard;
        else {
            bool present = false;
            if (linux_filesystem_mountinfo_contains(mountinfo_path, guard,
                                                    &present) != 0)
                return -1;
            *mounted = present;
        }
    }
    return 0;
}

static bool parse_u64_token(const char **cursor, int base, uint64_t *value) {
    char *end = NULL;
    while (isspace((unsigned char)**cursor)) ++*cursor;
    if (**cursor == '\0' || **cursor == '-' || **cursor == '+') return false;
    errno = 0;
    unsigned long long parsed = strtoull(*cursor, &end, base);
    if (errno == ERANGE || end == *cursor) return false;
    *value = (uint64_t)parsed;
    *cursor = end;
    return true;
}

static bool multiply_u64(uint64_t left, uint64_t right, uint64_t *result) {
    if (left != 0U && right > UINT64_MAX / left) return false;
    *result = left * right;
    return true;
}

static int parse_stat_output(const char *text,
                             storage_target_health_sample_t *sample) {
    uint64_t fragment, blocks, available, files, free_files, device;
    const char *cursor = text;
    if (!parse_u64_token(&cursor, 10, &fragment) ||
        !parse_u64_token(&cursor, 10, &blocks) ||
        !parse_u64_token(&cursor, 10, &available) ||
        !parse_u64_token(&cursor, 10, &files) ||
        !parse_u64_token(&cursor, 10, &free_files) ||
        !parse_u64_token(&cursor, 16, &device)) return -1;
    while (isspace((unsigned char)*cursor)) ++cursor;
    if (*cursor || available > blocks || free_files > files ||
        !multiply_u64(fragment, blocks,
                                 &sample->filesystem.capacity_bytes.value) ||
        !multiply_u64(fragment, available,
                      &sample->filesystem.available_bytes.value)) return -1;
    sample->filesystem.capacity_bytes.capability =
        SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    sample->filesystem.available_bytes.capability =
        SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    if (files == 0U) {
        sample->filesystem.capacity_inodes.capability =
            SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
        sample->filesystem.available_inodes.capability =
            SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
    } else {
        sample->filesystem.capacity_inodes.value = files;
        sample->filesystem.capacity_inodes.capability =
            SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
        sample->filesystem.available_inodes.value = free_files;
        sample->filesystem.available_inodes.capability =
            SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    }
    sample->filesystem_device = device;
    snprintf(sample->filesystem.device_key,
             sizeof(sample->filesystem.device_key), "linux-fs-%016llx",
             (unsigned long long)device);
    return 0;
}

static void make_unavailable(storage_target_health_sample_t *sample,
                             system_health_capability_t capability) {
    memset(sample, 0, sizeof(*sample));
    sample->filesystem.mount_present.capability = capability;
    sample->filesystem.read_only.capability = capability;
    sample->filesystem.capacity_bytes.capability = capability;
    sample->filesystem.available_bytes.capability = capability;
    sample->filesystem.capacity_inodes.capability = capability;
    sample->filesystem.available_inodes.capability = capability;
    sample->writeable.capability = capability;
    sample->probe.capability = capability;
    sample->recording_growth_capability = SYSTEM_HEALTH_CAPABILITY_STALE;
}

int storage_target_health_probe_with_ops(
    const linux_filesystem_resource_t *resource, bool write_probe,
    uint32_t timeout_ms, const storage_target_probe_ops_t *ops,
    storage_target_health_sample_t *sample) {
    if (!resource || !ops || !ops->run_helper || !ops->stat_program ||
        !ops->dd_program || !ops->rm_program || !sample || timeout_ms == 0U ||
        !linux_filesystem_logical_id_valid(resource->logical_id) ||
        resource->path[0] != '/') return -1;
    make_unavailable(sample, SYSTEM_HEALTH_CAPABILITY_ERROR);
    snprintf(sample->filesystem.logical_id,
             sizeof(sample->filesystem.logical_id), "%s",
             resource->logical_id);
    snprintf(sample->probe.logical_id, sizeof(sample->probe.logical_id), "%s",
             resource->logical_id);
    sample->sampled_monotonic_ms = monotonic_ms();

    bool mounted = true;
    bool read_only = false;
    const char *mountinfo = ops->mountinfo_path && ops->mountinfo_path[0]
        ? ops->mountinfo_path : "/proc/self/mountinfo";
    if (mount_facts(resource, mountinfo, &mounted,
                    &read_only) != 0) {
        int mount_error = errno;
        system_health_capability_t capability =
            linux_filesystem_capability_from_errno(mount_error);
        make_unavailable(sample, capability);
        snprintf(sample->filesystem.logical_id,
                 sizeof(sample->filesystem.logical_id), "%s",
                 resource->logical_id);
        snprintf(sample->probe.logical_id, sizeof(sample->probe.logical_id),
                 "%s", resource->logical_id);
        sample->probe.error = linux_filesystem_normalize_errno(mount_error);
        if (sample->probe.error == LINUX_FILESYSTEM_PROBE_ERROR_NONE)
            sample->probe.error = LINUX_FILESYSTEM_PROBE_ERROR_OTHER;
        return 0;
    }
    sample->filesystem.mount_present.value = mounted;
    sample->filesystem.mount_present.capability =
        SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    sample->filesystem.read_only.value = read_only;
    sample->filesystem.read_only.capability =
        SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    if (!mounted) {
        sample->probe.error = LINUX_FILESYSTEM_PROBE_ERROR_NOT_FOUND;
        return 0;
    }

    char *stat_arguments[] = {(char *)ops->stat_program, "-f", "-c",
        "%S %b %a %c %d %i", "--", (char *)resource->path, NULL};
    health_helper_request_t request = {
        .program = ops->stat_program, .argv = stat_arguments,
        .timeout_ms = timeout_ms, .terminate_grace_ms = 100U,
        .output_limit = 256U};
    health_helper_result_t helper;
    if (!invoke_helper(ops, &request, &helper)) {
        sample->probe.capability = helper_capability(&helper);
        sample->probe.error = output_error(&helper);
        sample->probe.latency_ms = helper.latency_ms;
        return 0;
    }
    sample->probe.latency_ms = helper.latency_ms;
    if (parse_stat_output(helper.output, sample) != 0) {
        sample->probe.capability = SYSTEM_HEALTH_CAPABILITY_ERROR;
        sample->probe.error = LINUX_FILESYSTEM_PROBE_ERROR_INVALID;
        return 0;
    }
    snprintf(sample->probe.device_key, sizeof(sample->probe.device_key), "%s",
             sample->filesystem.device_key);
    if (!write_probe) {
        sample->writeable.capability = SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
        sample->probe.capability = SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
        sample->probe.error = LINUX_FILESYSTEM_PROBE_ERROR_NONE;
        return 0;
    }
    if (read_only) {
        sample->writeable.value = false;
        sample->writeable.capability = SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
        sample->probe.capability = SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
        sample->probe.error = LINUX_FILESYSTEM_PROBE_ERROR_READ_ONLY;
        return 0;
    }

    char probe_path[LINUX_FILESYSTEM_PATH_LENGTH];
    char output_argument[LINUX_FILESYSTEM_PATH_LENGTH + 4U];
    int written = snprintf(probe_path, sizeof(probe_path),
        "%s/.lightnvr-health-%ld-%08llx", resource->path, (long)getpid(),
        (unsigned long long)(sample->sampled_monotonic_ms & 0xffffffffU));
    if (written < 0 || (size_t)written >= sizeof(probe_path) ||
        snprintf(output_argument, sizeof(output_argument), "of=%s",
                 probe_path) >= (int)sizeof(output_argument)) {
        sample->probe.error = LINUX_FILESYSTEM_PROBE_ERROR_INVALID;
        return 0;
    }
    char *dd_arguments[] = {(char *)ops->dd_program, "if=/dev/zero",
        output_argument, "bs=1", "count=1", "conv=fsync", "status=none",
        NULL};
    request.program = ops->dd_program;
    request.argv = dd_arguments;
    if (!invoke_helper(ops, &request, &helper)) {
        health_helper_result_t write_result = helper;
        sample->writeable.value = false;
        sample->writeable.capability =
            write_result.outcome == HEALTH_HELPER_EXITED
            ? SYSTEM_HEALTH_CAPABILITY_AVAILABLE
            : helper_capability(&write_result);
        sample->probe.capability = sample->writeable.capability;
        sample->probe.error = output_error(&write_result);
        saturating_add_latency(&sample->probe.latency_ms,
                               write_result.latency_ms);
        char *rm_arguments[] = {(char *)ops->rm_program, "-f", "--",
                                probe_path, NULL};
        request.program = ops->rm_program;
        request.argv = rm_arguments;
        if (write_result.abandoned) {
            sample->cleanup_failed = true;
            return 0;
        }
        if (!invoke_helper(ops, &request, &helper))
            sample->cleanup_failed = true;
        else
            sample->probe.unlink_completed = true;
        saturating_add_latency(&sample->probe.latency_ms, helper.latency_ms);
        return 0;
    }
    saturating_add_latency(&sample->probe.latency_ms, helper.latency_ms);
    sample->probe.write_completed = true;
    sample->probe.fsync_completed = true;

    char *rm_arguments[] = {(char *)ops->rm_program, "-f", "--", probe_path,
                            NULL};
    request.program = ops->rm_program;
    request.argv = rm_arguments;
    if (!invoke_helper(ops, &request, &helper)) {
        sample->writeable.value = true;
        sample->writeable.capability = SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
        sample->probe.capability = helper_capability(&helper);
        sample->probe.error = output_error(&helper);
        sample->cleanup_failed = true;
        saturating_add_latency(&sample->probe.latency_ms, helper.latency_ms);
        return 0;
    }
    saturating_add_latency(&sample->probe.latency_ms, helper.latency_ms);
    sample->writeable.value = true;
    sample->writeable.capability = SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    sample->probe.capability = SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    sample->probe.error = LINUX_FILESYSTEM_PROBE_ERROR_NONE;
    sample->probe.unlink_completed = true;
    return 0;
}

static void default_probe_ops(storage_target_probe_ops_t *ops) {
    memset(ops, 0, sizeof(*ops));
    ops->run_helper = health_helper_run;
    ops->stat_program = select_program("/usr/bin/stat", "/bin/stat");
    ops->dd_program = select_program("/usr/bin/dd", "/bin/dd");
    ops->rm_program = select_program("/usr/bin/rm", "/bin/rm");
}

void storage_target_health_collector_state_init(
    storage_target_health_collector_state_t *state, const char *root_path,
    const char *recording_path, bool write_probe_enabled) {
    if (!state) return;
    memset(state, 0, sizeof(*state));
    snprintf(state->root_path, sizeof(state->root_path), "%s",
             root_path && root_path[0] ? root_path : "/");
    snprintf(state->recording_path, sizeof(state->recording_path), "%s",
             recording_path && recording_path[0] ? recording_path : "/");
    state->write_probe_enabled = write_probe_enabled;
    state->timeout_ms = STORAGE_HEALTH_DEFAULT_TIMEOUT_MS;
    state->stale_after_ms = STORAGE_HEALTH_DEFAULT_STALE_MS;
    state->max_probes_per_cycle = STORAGE_HEALTH_PROBES_PER_CYCLE;
    default_probe_ops(&state->ops);
}

static const storage_target_health_slot_t *find_old_slot(
    const storage_target_health_slot_t *old, size_t count,
    const char *logical_id) {
    for (size_t index = 0; index < count; ++index) {
        if (strcmp(old[index].resource.logical_id, logical_id) == 0)
            return &old[index];
    }
    return NULL;
}

static bool add_slot(storage_target_health_collector_state_t *state,
                     const storage_target_health_slot_t *old,
                     size_t old_count, const char *logical_id,
                     const char *path, const storage_target_t *target) {
    if (state->slot_count >= SYSTEM_HEALTH_MAX_FILESYSTEMS) {
        state->slots_dropped++;
        return false;
    }
    storage_target_health_slot_t *slot = &state->slots[state->slot_count++];
    memset(slot, 0, sizeof(*slot));
    const storage_target_health_slot_t *previous =
        find_old_slot(old, old_count, logical_id);
    if (previous) {
        slot->sample = previous->sample;
        slot->sample_valid = previous->sample_valid;
    }
    snprintf(slot->resource.logical_id, sizeof(slot->resource.logical_id),
             "%s", logical_id);
    snprintf(slot->resource.path, sizeof(slot->resource.path), "%s", path);
    if (target) {
        snprintf(slot->target_uuid, sizeof(slot->target_uuid), "%s",
                 target->uuid);
        slot->resource.mount_required = target->mount_required;
        snprintf(slot->resource.mount_guard_path,
                 sizeof(slot->resource.mount_guard_path), "%s",
                 target->mount_guard_path);
    }
    return true;
}

static int refresh_slots(storage_target_health_collector_state_t *state) {
    storage_target_health_slot_t *old = calloc(
        SYSTEM_HEALTH_MAX_FILESYSTEMS, sizeof(*old));
    if (!old) return -1;
    size_t old_count = state->slot_count;
    memcpy(old, state->slots, old_count * sizeof(*old));
    memset(state->slots, 0, sizeof(state->slots));
    state->slot_count = 0U;
    state->slots_dropped = 0U;
    (void)add_slot(state, old, old_count, "root", state->root_path, NULL);
    (void)add_slot(state, old, old_count, "recording", state->recording_path,
                   NULL);

    int total = db_storage_target_count();
    if (total < 0 || total > STORAGE_TARGET_MAX_COUNT) {
        free(old);
        return -1;
    }
    storage_target_t *targets = total > 0
        ? calloc((size_t)total, sizeof(*targets)) : NULL;
    int count = total > 0 && targets ? db_storage_target_list(targets, total)
                                    : total;
    if (count < 0 || (total > 0 && !targets)) {
        free(targets);
        free(old);
        return -1;
    }
    for (int index = 0; index < count; ++index) {
        if (!targets[index].enabled) continue;
        char logical_id[SYSTEM_HEALTH_ID_LENGTH];
        snprintf(logical_id, sizeof(logical_id), "target:%s",
                 targets[index].uuid);
        (void)add_slot(state, old, old_count, logical_id,
                       targets[index].root_path, &targets[index]);
    }
    free(targets);
    free(old);
    if (state->slot_count > 0U) state->next_probe %= state->slot_count;
    else state->next_probe = 0U;
    return 0;
}

static bool sample_available(const storage_target_health_sample_t *sample) {
    return sample->filesystem.mount_present.capability ==
               SYSTEM_HEALTH_CAPABILITY_AVAILABLE &&
           sample->filesystem.mount_present.value &&
           sample->filesystem.capacity_bytes.capability ==
               SYSTEM_HEALTH_CAPABILITY_AVAILABLE &&
           sample->filesystem.available_bytes.capability ==
               SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
}

static int persist_target_sample(const storage_target_health_slot_t *slot,
                                 bool write_probe, int64_t wall_time_ms) {
    if (!slot->target_uuid[0]) return 0;
    storage_target_t before;
    const storage_target_health_sample_t *sample = &slot->sample;
    storage_target_health_update_t update;
    memset(&update, 0, sizeof(update));
    update.available = sample_available(sample);
    update.write_checked = write_probe;
    update.writeable = sample->writeable.capability ==
                           SYSTEM_HEALTH_CAPABILITY_AVAILABLE &&
                       sample->writeable.value;
    update.cleanup_failed = sample->cleanup_failed;
    update.capacity_bytes = sample->filesystem.capacity_bytes.value;
    update.available_bytes = sample->filesystem.available_bytes.value;
    update.filesystem_device = sample->filesystem_device;
    update.probed_at = wall_time_ms > 0 ? wall_time_ms / 1000 : time(NULL);
    const char *error = storage_target_probe_error_name(sample->probe.error);
    if (!update.available && sample->filesystem.mount_present.capability ==
            SYSTEM_HEALTH_CAPABILITY_AVAILABLE &&
        !sample->filesystem.mount_present.value) error = "mount_absent";
    snprintf(update.normalized_error, sizeof(update.normalized_error), "%s",
             error);
    storage_target_t after;
    pthread_mutex_lock(&target_probe_mutex);
    if (db_storage_target_get(slot->target_uuid, &before) !=
        DB_STORAGE_TARGET_OK) {
        pthread_mutex_unlock(&target_probe_mutex);
        return -1;
    }
    db_storage_target_result_t result = db_storage_target_record_health(
        slot->target_uuid, &update, &after);
    if (result == DB_STORAGE_TARGET_OK) publish_transition(&before, &after);
    pthread_mutex_unlock(&target_probe_mutex);
    return result == DB_STORAGE_TARGET_OK ? 0 : -1;
}

static system_health_capability_t effective_capability(
    system_health_capability_t capability, bool stale) {
    return stale ? SYSTEM_HEALTH_CAPABILITY_STALE : capability;
}

static void prepare_observation(
    system_health_observation_t *observation,
    const storage_target_health_slot_t *slot,
    const system_health_collect_context_t *context, const char *metric) {
    memset(observation, 0, sizeof(*observation));
    snprintf(observation->metric, sizeof(observation->metric), "%s", metric);
    snprintf(observation->resource_id, sizeof(observation->resource_id), "%s",
             slot->resource.logical_id);
    observation->scope = SYSTEM_HEALTH_SCOPE_FILESYSTEM;
    observation->sampled_monotonic_ms = slot->sample.sampled_monotonic_ms;
    observation->observed_wall_time_ms = context->wall_time_ms;
}

static void emit_number(system_health_observation_sink_t *sink,
                        const system_health_collect_context_t *context,
                        const storage_target_health_slot_t *slot,
                        const char *metric, double value,
                        system_health_capability_t capability,
                        system_health_unit_t unit, bool stale) {
    system_health_observation_t observation;
    prepare_observation(&observation, slot, context, metric);
    capability = effective_capability(capability, stale);
    if (capability == SYSTEM_HEALTH_CAPABILITY_AVAILABLE)
        system_health_observation_set_available(&observation, value, unit);
    else
        system_health_observation_set_unavailable(&observation, capability);
    (void)system_health_observation_sink_append(sink, &observation);
}

static void emit_flag(system_health_observation_sink_t *sink,
                      const system_health_collect_context_t *context,
                      const storage_target_health_slot_t *slot,
                      const char *metric, const linux_filesystem_flag_t *flag,
                      bool stale) {
    emit_number(sink, context, slot, metric, flag->value ? 1.0 : 0.0,
                flag->capability, SYSTEM_HEALTH_UNIT_BOOLEAN, stale);
}

static void emit_value(system_health_observation_sink_t *sink,
                       const system_health_collect_context_t *context,
                       const storage_target_health_slot_t *slot,
                       const char *metric, const linux_filesystem_value_t *value,
                       system_health_unit_t unit, bool stale) {
    emit_number(sink, context, slot, metric, (double)value->value,
                value->capability, unit, stale);
}

static void emit_storage_observations(
    system_health_observation_sink_t *sink,
    const system_health_collect_context_t *context,
    const storage_target_health_slot_t *slot, bool stale,
    bool emit_canonical, bool emit_write_condition) {
    const storage_target_health_sample_t *sample = &slot->sample;
    emit_flag(sink, context, slot, "storage.filesystem.mount_present",
              &sample->filesystem.mount_present, stale);
    emit_flag(sink, context, slot, "storage.filesystem.read_only",
              &sample->filesystem.read_only, stale);
    emit_value(sink, context, slot, "storage.filesystem.capacity_bytes",
               &sample->filesystem.capacity_bytes, SYSTEM_HEALTH_UNIT_BYTES,
               stale);
    emit_value(sink, context, slot, "storage.filesystem.available_bytes",
               &sample->filesystem.available_bytes, SYSTEM_HEALTH_UNIT_BYTES,
               stale);
    emit_value(sink, context, slot, "storage.filesystem.capacity_inodes",
               &sample->filesystem.capacity_inodes, SYSTEM_HEALTH_UNIT_COUNT,
               stale);
    emit_value(sink, context, slot, "storage.filesystem.available_inodes",
               &sample->filesystem.available_inodes, SYSTEM_HEALTH_UNIT_COUNT,
               stale);
    emit_flag(sink, context, slot, "storage.filesystem.writeable",
              &sample->writeable, stale);
    emit_number(sink, context, slot, "storage.filesystem.probe_latency_seconds",
                (double)sample->probe.latency_ms / 1000.0,
                sample->probe.capability, SYSTEM_HEALTH_UNIT_SECONDS, stale);
    emit_number(sink, context, slot,
                "storage.filesystem.recording_growth_bytes_per_second",
                sample->recording_growth_bps,
                sample->recording_growth_capability,
                SYSTEM_HEALTH_UNIT_COUNT, stale);
    if (emit_write_condition)
        emit_number(sink, context, slot, "filesystem.write_failed",
                    sample->probe.error == LINUX_FILESYSTEM_PROBE_ERROR_NONE
                        ? 0.0 : 1.0,
                    sample->writeable.capability,
                    SYSTEM_HEALTH_UNIT_BOOLEAN, stale);
    if (!emit_canonical) return;
    emit_flag(sink, context, slot, "filesystem.mount_present",
              &sample->filesystem.mount_present, stale);
    emit_flag(sink, context, slot, "filesystem.read_only",
              &sample->filesystem.read_only, stale);
    emit_value(sink, context, slot, "filesystem.capacity_bytes",
               &sample->filesystem.capacity_bytes, SYSTEM_HEALTH_UNIT_BYTES,
               stale);
    emit_value(sink, context, slot, "filesystem.available_bytes",
               &sample->filesystem.available_bytes, SYSTEM_HEALTH_UNIT_BYTES,
               stale);
    emit_value(sink, context, slot, "filesystem.capacity_inodes",
               &sample->filesystem.capacity_inodes, SYSTEM_HEALTH_UNIT_COUNT,
               stale);
    emit_value(sink, context, slot, "filesystem.available_inodes",
               &sample->filesystem.available_inodes, SYSTEM_HEALTH_UNIT_COUNT,
               stale);
}

static bool device_seen_before(
    const storage_target_health_collector_state_t *state, size_t index) {
    const char *key = state->slots[index].sample.filesystem.device_key;
    if (!key[0]) return false;
    for (size_t previous = 0; previous < index; ++previous) {
        if (state->slots[previous].sample_valid &&
            strcmp(key,
                   state->slots[previous].sample.filesystem.device_key) == 0)
            return true;
    }
    return false;
}

int storage_target_health_collect(
    void *opaque, const system_health_collect_context_t *context,
    system_health_observation_sink_t *sink) {
    storage_target_health_collector_state_t *state = opaque;
    if (!state || !context || !sink || refresh_slots(state) != 0) return -1;
    char mountinfo[LINUX_FILESYSTEM_PATH_LENGTH];
    const char *proc_root = context->proc_root ? context->proc_root : "/proc";
    snprintf(mountinfo, sizeof(mountinfo), "%s/self/mountinfo", proc_root);
    storage_target_probe_ops_t ops = state->ops;
    if (!ops.mountinfo_path) ops.mountinfo_path = state->mountinfo_path[0]
        ? state->mountinfo_path : mountinfo;
    size_t probes = state->max_probes_per_cycle;
    if (probes == 0U || probes > state->slot_count) probes = state->slot_count;
    int failures = 0;
    for (size_t attempt = 0; attempt < probes && state->slot_count > 0U;
         ++attempt) {
        size_t index = state->next_probe++ % state->slot_count;
        storage_target_health_slot_t *slot = &state->slots[index];
        storage_target_health_sample_t sample;
        if (storage_target_health_probe_with_ops(
                &slot->resource, state->write_probe_enabled,
                state->timeout_ms, &ops, &sample) != 0) {
            failures++;
            continue;
        }
        sample.sampled_monotonic_ms = context->monotonic_ms;
        double growth = 0.0;
        if (db_storage_target_recording_growth_bps(
                slot->target_uuid[0] ? slot->target_uuid : NULL,
                &growth) == 0) {
            sample.recording_growth_bps = growth;
            sample.recording_growth_capability =
                SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
        }
        slot->sample = sample;
        slot->sample_valid = true;
        if (persist_target_sample(slot, state->write_probe_enabled,
                                  context->wall_time_ms) != 0)
            failures++;
    }
    for (size_t index = 0; index < state->slot_count; ++index) {
        storage_target_health_slot_t *slot = &state->slots[index];
        if (!slot->sample_valid) {
            make_unavailable(&slot->sample,
                             SYSTEM_HEALTH_CAPABILITY_STALE);
            char logical_id[SYSTEM_HEALTH_ID_LENGTH];
            snprintf(logical_id, sizeof(logical_id), "%s",
                     slot->resource.logical_id);
            snprintf(slot->sample.filesystem.logical_id,
                     sizeof(slot->sample.filesystem.logical_id), "%s",
                     logical_id);
            snprintf(slot->sample.probe.logical_id,
                     sizeof(slot->sample.probe.logical_id), "%s",
                     logical_id);
        }
        bool stale = slot->sample.sampled_monotonic_ms == 0U ||
            context->monotonic_ms < slot->sample.sampled_monotonic_ms ||
            context->monotonic_ms - slot->sample.sampled_monotonic_ms >
                state->stale_after_ms;
        /* Root/recording canonical conditions come from the normal portable
         * collector. Only unique target devices add another canonical source. */
        bool unique_device = !device_seen_before(state, index);
        bool canonical = index >= 2U && unique_device;
        emit_storage_observations(sink, context, slot, stale, canonical,
                                  unique_device);
    }
    sink->dropped += state->slots_dropped;
    return failures == 0 ? 0 : -1;
}

void storage_target_health_collector_init(
    system_health_collector_t *collector,
    storage_target_health_collector_state_t *state) {
    if (!collector) return;
    memset(collector, 0, sizeof(*collector));
    snprintf(collector->name, sizeof(collector->name), "%s",
             "storage_targets");
    collector->scope = SYSTEM_HEALTH_SCOPE_FILESYSTEM;
    collector->tier = SYSTEM_HEALTH_TIER_SLOW;
    collector->interval_seconds = 300U;
    collector->stale_after_seconds = 900U;
    collector->state = state;
    collector->collect = storage_target_health_collect;
}

bool storage_target_health_register(const char *root_path,
                                    const char *recording_path,
                                    bool write_probe_enabled) {
    if (registered_state_initialized) return true;
    storage_target_health_collector_state_init(
        &registered_state, root_path, recording_path, write_probe_enabled);
    system_health_collector_t collector;
    storage_target_health_collector_init(&collector, &registered_state);
    bool result = system_health_register_collector(&collector);
    if (result) registered_state_initialized = true;
    return result;
}

static bool state_is(const storage_target_t *target, const char *state) {
    return target && strcmp(target->health_status, state) == 0;
}

static const char *normalized_previous_state(const storage_target_t *target) {
    if (state_is(target, "healthy")) return "healthy";
    if (state_is(target, "degraded")) return "degraded";
    return "unknown";
}

static const char *normalized_unavailable_reason(const char *error) {
    if (!error) return "unknown";
    if (strcmp(error, "mount_absent") == 0) return "mount_unavailable";
    if (strcmp(error, "not_found") == 0) return "directory_unavailable";
    if (strcmp(error, "permission") == 0 ||
        strcmp(error, "read_only") == 0) return "not_writable";
    if (strcmp(error, "no_space") == 0 || strcmp(error, "io") == 0 ||
        strcmp(error, "timed_out") == 0 || strcmp(error, "busy") == 0 ||
        strcmp(error, "invalid") == 0 || strcmp(error, "other") == 0)
        return "write_probe_failed";
    static const char missing_mount[] = "Required mount is absent:";
    static const char no_mount[] = "No distinct mounted filesystem";
    static const char directory[] = "Target directory is unavailable:";
    static const char capacity[] = "Capacity probe failed:";
    static const char not_writable[] = "Target directory is not writable:";
    static const char long_path[] = "Probe path is too long";
    static const char write_probe[] = "Write probe failed:";
    static const char cleanup[] = "Probe cleanup failed:";

    if (strncmp(error, missing_mount, sizeof(missing_mount) - 1) == 0 ||
        strncmp(error, no_mount, sizeof(no_mount) - 1) == 0) {
        return "mount_unavailable";
    }
    if (strncmp(error, directory, sizeof(directory) - 1) == 0) {
        return "directory_unavailable";
    }
    if (strncmp(error, capacity, sizeof(capacity) - 1) == 0) {
        return "capacity_probe_failed";
    }
    if (strncmp(error, not_writable, sizeof(not_writable) - 1) == 0) {
        return "not_writable";
    }
    if (strncmp(error, long_path, sizeof(long_path) - 1) == 0 ||
        strncmp(error, write_probe, sizeof(write_probe) - 1) == 0) {
        return "write_probe_failed";
    }
    if (strncmp(error, cleanup, sizeof(cleanup) - 1) == 0) {
        return "probe_cleanup_failed";
    }
    return "unknown";
}

static int64_t downtime_milliseconds(const storage_target_t *before,
                                     const storage_target_t *after) {
    if (!before || !after || before->last_success_at <= 0 ||
        after->last_probe_at <= before->last_success_at) {
        return 0;
    }
    int64_t seconds = after->last_probe_at - before->last_success_at;
    return seconds > INT64_MAX / 1000 ? INT64_MAX : seconds * 1000;
}

static void publish_transition(const storage_target_t *before,
                               const storage_target_t *after) {
    bool was_unavailable = state_is(before, "unavailable");
    bool is_unavailable = state_is(after, "unavailable");
    bool is_recovered = state_is(after, "healthy") ||
        state_is(after, "degraded");
    time_t occurred_at = after->last_probe_at > 0
        ? (time_t)after->last_probe_at : time(NULL);
    char error[256] = {0};
    int result = 0;

    if (!was_unavailable && is_unavailable) {
        result = event_producer_publish_storage_target_unavailable(
            after->uuid, normalized_previous_state(before),
            normalized_unavailable_reason(after->last_error),
            after->is_default, occurred_at, error, sizeof(error));
    } else if (was_unavailable && is_recovered) {
        result = event_producer_publish_storage_target_recovered(
            after->uuid, after->health_status,
            downtime_milliseconds(before, after), after->is_default,
            occurred_at, error, sizeof(error));
    } else {
        return;
    }

    if (result != 0) {
        log_warn("Failed to enqueue storage target transition event for %s: %s",
                 after->uuid, error[0] ? error : "event pipeline unavailable");
    }
}

db_storage_target_result_t storage_target_probe_and_publish(
    const char *uuid, bool write_test, storage_target_t *target) {
    storage_target_t before;
    storage_target_t after;
    memset(&before, 0, sizeof(before));
    memset(&after, 0, sizeof(after));

    pthread_mutex_lock(&target_probe_mutex);
    db_storage_target_result_t result = db_storage_target_get(uuid, &before);
    if (result == DB_STORAGE_TARGET_OK) {
        result = db_storage_target_probe(uuid, write_test, &after);
        if (result == DB_STORAGE_TARGET_OK ||
            result == DB_STORAGE_TARGET_UNAVAILABLE) {
            publish_transition(&before, &after);
            if (target) *target = after;
        }
    }
    pthread_mutex_unlock(&target_probe_mutex);
    return result;
}

int storage_target_refresh_health_and_publish(void) {
    int total = db_storage_target_count();
    if (total < 0 || total > STORAGE_TARGET_MAX_COUNT) return -1;
    if (total == 0) return 0;
    /* Coalesced wake-up: the bounded slow worker owns all scheduled I/O. */
    (void)system_health_request_tier(SYSTEM_HEALTH_TIER_SLOW);
    return total;
}
