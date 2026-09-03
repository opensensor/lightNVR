#include "telemetry/collectors/linux_filesystem.h"

#include <ctype.h>
#include <errno.h>
#include <stdio.h>
#include <string.h>
#include <sys/sysmacros.h>

#define MOUNTINFO_LINE_MAX 4096U
#define MOUNTINFO_ENTRY_MAX 4096U

linux_filesystem_probe_error_t linux_filesystem_normalize_errno(int error_code) {
    switch (error_code) {
        case 0: return LINUX_FILESYSTEM_PROBE_ERROR_NONE;
        case ENOENT:
        case ENOTDIR: return LINUX_FILESYSTEM_PROBE_ERROR_NOT_FOUND;
        case EACCES:
        case EPERM: return LINUX_FILESYSTEM_PROBE_ERROR_PERMISSION;
        case EROFS: return LINUX_FILESYSTEM_PROBE_ERROR_READ_ONLY;
        case ENOSPC:
#ifdef EDQUOT
        case EDQUOT:
#endif
            return LINUX_FILESYSTEM_PROBE_ERROR_NO_SPACE;
        case EIO: return LINUX_FILESYSTEM_PROBE_ERROR_IO;
        case ETIMEDOUT: return LINUX_FILESYSTEM_PROBE_ERROR_TIMED_OUT;
        case EBUSY: return LINUX_FILESYSTEM_PROBE_ERROR_BUSY;
        case EINVAL:
        case ENAMETOOLONG: return LINUX_FILESYSTEM_PROBE_ERROR_INVALID;
        default: return LINUX_FILESYSTEM_PROBE_ERROR_OTHER;
    }
}

system_health_capability_t linux_filesystem_capability_from_errno(
    int error_code) {
    if (error_code == EACCES || error_code == EPERM) {
        return SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED;
    }
    if (error_code == ENOSYS || error_code == ENOTSUP) {
        return SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
    }
    return SYSTEM_HEALTH_CAPABILITY_ERROR;
}

bool linux_filesystem_logical_id_valid(const char *logical_id) {
    if (!logical_id || logical_id[0] == '\0') return false;
    size_t length = 0;
    for (; logical_id[length] != '\0'; ++length) {
        unsigned char character = (unsigned char)logical_id[length];
        if (length >= SYSTEM_HEALTH_ID_LENGTH - 1U) return false;
        if (!isalnum(character) && character != '_' && character != '-' &&
            character != '.' && character != ':') return false;
    }
    return length > 0U;
}

static bool decode_mount_field(const char *encoded, size_t encoded_length,
                               char *decoded, size_t decoded_size) {
    size_t output = 0;
    for (size_t input = 0; input < encoded_length; ++input) {
        unsigned char character = (unsigned char)encoded[input];
        if (character == '\\' && input + 3U < encoded_length &&
            encoded[input + 1U] >= '0' && encoded[input + 1U] <= '7' &&
            encoded[input + 2U] >= '0' && encoded[input + 2U] <= '7' &&
            encoded[input + 3U] >= '0' && encoded[input + 3U] <= '7') {
            character = (unsigned char)(((encoded[input + 1U] - '0') << 6) |
                                        ((encoded[input + 2U] - '0') << 3) |
                                        (encoded[input + 3U] - '0'));
            input += 3U;
        }
        if (output + 1U >= decoded_size || character == '\0') return false;
        decoded[output++] = (char)character;
    }
    if (output >= decoded_size) return false;
    decoded[output] = '\0';
    return true;
}

static bool mount_point_field(const char *line, char *mount_point,
                              size_t mount_point_size) {
    const char *cursor = line;
    for (unsigned field = 1U; field <= 5U; ++field) {
        while (*cursor == ' ') cursor++;
        if (*cursor == '\0' || *cursor == '\n') return false;
        const char *begin = cursor;
        while (*cursor != '\0' && *cursor != '\n' && *cursor != ' ') cursor++;
        if (field == 5U) {
            return decode_mount_field(begin, (size_t)(cursor - begin),
                                      mount_point, mount_point_size);
        }
    }
    return false;
}

int linux_filesystem_mountinfo_contains(const char *mountinfo_path,
                                        const char *mount_path,
                                        bool *present) {
    if (!mountinfo_path || !mount_path || !present || mount_path[0] == '\0') {
        errno = EINVAL;
        return -1;
    }
    *present = false;
    FILE *input = fopen(mountinfo_path, "rb");
    if (!input) return -1;

    char line[MOUNTINFO_LINE_MAX + 2U];
    size_t entries = 0;
    while (fgets(line, sizeof(line), input)) {
        size_t length = strlen(line);
        if (length > MOUNTINFO_LINE_MAX ||
            (length > 0U && line[length - 1U] != '\n' && !feof(input))) {
            fclose(input);
            errno = EOVERFLOW;
            return -1;
        }
        if (++entries > MOUNTINFO_ENTRY_MAX) {
            fclose(input);
            errno = EOVERFLOW;
            return -1;
        }
        char decoded[LINUX_FILESYSTEM_PATH_LENGTH];
        if (!mount_point_field(line, decoded, sizeof(decoded))) {
            fclose(input);
            errno = EINVAL;
            return -1;
        }
        if (strcmp(decoded, mount_path) == 0) {
            *present = true;
            fclose(input);
            return 0;
        }
    }
    if (ferror(input)) {
        int saved_errno = errno;
        fclose(input);
        errno = saved_errno;
        return -1;
    }
    fclose(input);
    return 0;
}

static void unavailable_value(linux_filesystem_value_t *value,
                              system_health_capability_t capability) {
    value->value = 0;
    value->capability = capability;
}

static void unavailable_flag(linux_filesystem_flag_t *flag,
                             system_health_capability_t capability) {
    flag->value = false;
    flag->capability = capability;
}

static void make_sample_unavailable(linux_filesystem_sample_t *sample,
                                    system_health_capability_t capability) {
    unavailable_flag(&sample->mount_present, capability);
    unavailable_flag(&sample->read_only, capability);
    unavailable_value(&sample->mount_flags, capability);
    unavailable_value(&sample->capacity_bytes, capability);
    unavailable_value(&sample->available_bytes, capability);
    unavailable_value(&sample->capacity_inodes, capability);
    unavailable_value(&sample->available_inodes, capability);
}

static void make_capacity_unavailable(linux_filesystem_sample_t *sample,
                                      system_health_capability_t capability) {
    unavailable_flag(&sample->read_only, capability);
    unavailable_value(&sample->mount_flags, capability);
    unavailable_value(&sample->capacity_bytes, capability);
    unavailable_value(&sample->available_bytes, capability);
    unavailable_value(&sample->capacity_inodes, capability);
    unavailable_value(&sample->available_inodes, capability);
}

static bool checked_multiply(uint64_t left, uint64_t right,
                             uint64_t *result) {
    if (left != 0U && right > UINT64_MAX / left) return false;
    *result = left * right;
    return true;
}

static int default_stat_path(const char *path, struct stat *info) {
    return stat(path, info);
}

static int default_statvfs_path(const char *path, struct statvfs *info) {
    return statvfs(path, info);
}

int linux_filesystem_sample_with_ops(
    const linux_filesystem_resource_t *resource, const char *mountinfo_path,
    const linux_filesystem_ops_t *ops, linux_filesystem_sample_t *sample) {
    if (!resource || !ops || !ops->stat_path || !ops->statvfs_path ||
        !sample || !linux_filesystem_logical_id_valid(resource->logical_id) ||
        resource->path[0] == '\0') return -1;

    memset(sample, 0, sizeof(*sample));
    snprintf(sample->logical_id, sizeof(sample->logical_id), "%s",
             resource->logical_id);
    make_sample_unavailable(sample, SYSTEM_HEALTH_CAPABILITY_ERROR);

    if (resource->mount_required) {
        const char *guard = resource->mount_guard_path[0]
            ? resource->mount_guard_path : resource->path;
        bool present = false;
        if (linux_filesystem_mountinfo_contains(mountinfo_path, guard,
                                                &present) != 0) {
            unavailable_flag(&sample->mount_present,
                             linux_filesystem_capability_from_errno(errno));
            return 0;
        }
        sample->mount_present.capability = SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
        sample->mount_present.value = present;
        if (!present) return 0;
    } else {
        sample->mount_present.capability = SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
        sample->mount_present.value = true;
    }

    struct stat path_info;
    if (ops->stat_path(resource->path, &path_info) != 0) {
        system_health_capability_t capability =
            linux_filesystem_capability_from_errno(errno);
        make_capacity_unavailable(sample, capability);
        if (errno == ENOENT || errno == ENOTDIR) {
            sample->mount_present.capability = SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
            sample->mount_present.value = false;
        }
        return 0;
    }
    if (!S_ISDIR(path_info.st_mode)) {
        make_capacity_unavailable(sample, SYSTEM_HEALTH_CAPABILITY_ERROR);
        sample->mount_present.capability = SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
        sample->mount_present.value = false;
        return 0;
    }

    int key_length = snprintf(sample->device_key, sizeof(sample->device_key),
                              "linux-block-%u-%u", major(path_info.st_dev),
                              minor(path_info.st_dev));
    if (key_length < 0 || (size_t)key_length >= sizeof(sample->device_key)) {
        sample->device_key[0] = '\0';
        make_capacity_unavailable(sample, SYSTEM_HEALTH_CAPABILITY_ERROR);
        return 0;
    }
    if (resource->expected_device_key[0] != '\0' &&
        strcmp(resource->expected_device_key, sample->device_key) != 0) {
        sample->mount_present.capability = SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
        sample->mount_present.value = false;
        make_capacity_unavailable(sample, SYSTEM_HEALTH_CAPABILITY_ERROR);
        return 0;
    }

    struct statvfs filesystem;
    if (ops->statvfs_path(resource->path, &filesystem) != 0) {
        make_capacity_unavailable(
            sample, linux_filesystem_capability_from_errno(errno));
        return 0;
    }

    sample->read_only.capability = SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    sample->mount_flags.value = (uint64_t)filesystem.f_flag;
    sample->mount_flags.capability = SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
#ifdef ST_RDONLY
    sample->read_only.value = (filesystem.f_flag & ST_RDONLY) != 0U;
#else
    sample->read_only.value = false;
#endif

    uint64_t fragment_size = (uint64_t)filesystem.f_frsize;
    uint64_t capacity = 0;
    uint64_t available = 0;
    if ((fragment_size == 0U && filesystem.f_blocks != 0U) ||
        !checked_multiply((uint64_t)filesystem.f_blocks, fragment_size,
                          &capacity)) {
        unavailable_value(&sample->capacity_bytes,
                          SYSTEM_HEALTH_CAPABILITY_ERROR);
    } else {
        sample->capacity_bytes.value = capacity;
        sample->capacity_bytes.capability = SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    }
    if ((fragment_size == 0U && filesystem.f_bavail != 0U) ||
        !checked_multiply((uint64_t)filesystem.f_bavail, fragment_size,
                          &available)) {
        unavailable_value(&sample->available_bytes,
                          SYSTEM_HEALTH_CAPABILITY_ERROR);
    } else {
        sample->available_bytes.value = available;
        sample->available_bytes.capability = SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    }

    if (filesystem.f_files == 0U) {
        unavailable_value(&sample->capacity_inodes,
                          SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED);
        unavailable_value(&sample->available_inodes,
                          SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED);
    } else {
        sample->capacity_inodes.value = (uint64_t)filesystem.f_files;
        sample->capacity_inodes.capability = SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
        sample->available_inodes.value = (uint64_t)filesystem.f_favail;
        sample->available_inodes.capability = SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    }
    return 0;
}

int linux_filesystem_sample(const linux_filesystem_resource_t *resource,
                            const char *mountinfo_path,
                            linux_filesystem_sample_t *sample) {
    const linux_filesystem_ops_t ops = {
        .stat_path = default_stat_path,
        .statvfs_path = default_statvfs_path
    };
    return linux_filesystem_sample_with_ops(resource, mountinfo_path, &ops,
                                            sample);
}

static void prepare_observation(system_health_observation_t *observation,
                                const linux_filesystem_sample_t *sample,
                                const system_health_collect_context_t *context,
                                const char *metric) {
    memset(observation, 0, sizeof(*observation));
    snprintf(observation->metric, sizeof(observation->metric), "%s", metric);
    snprintf(observation->resource_id, sizeof(observation->resource_id), "%s",
             sample->logical_id);
    observation->scope = SYSTEM_HEALTH_SCOPE_FILESYSTEM;
    observation->sampled_monotonic_ms = context->monotonic_ms;
    observation->observed_wall_time_ms = context->wall_time_ms;
}

static void emit_value(system_health_observation_sink_t *sink,
                       const system_health_collect_context_t *context,
                       const linux_filesystem_sample_t *sample,
                       const char *metric,
                       const linux_filesystem_value_t *value,
                       system_health_unit_t unit) {
    system_health_observation_t observation;
    prepare_observation(&observation, sample, context, metric);
    if (value->capability == SYSTEM_HEALTH_CAPABILITY_AVAILABLE) {
        system_health_observation_set_available(&observation,
                                                (double)value->value, unit);
    } else {
        system_health_observation_set_unavailable(&observation,
                                                  value->capability);
    }
    system_health_observation_sink_append(sink, &observation);
}

static void emit_flag(system_health_observation_sink_t *sink,
                      const system_health_collect_context_t *context,
                      const linux_filesystem_sample_t *sample,
                      const char *metric,
                      const linux_filesystem_flag_t *flag) {
    system_health_observation_t observation;
    prepare_observation(&observation, sample, context, metric);
    if (flag->capability == SYSTEM_HEALTH_CAPABILITY_AVAILABLE) {
        system_health_observation_set_available(&observation,
                                                flag->value ? 1.0 : 0.0,
                                                SYSTEM_HEALTH_UNIT_BOOLEAN);
    } else {
        system_health_observation_set_unavailable(&observation,
                                                  flag->capability);
    }
    system_health_observation_sink_append(sink, &observation);
}

int linux_filesystem_collect(void *state,
                             const system_health_collect_context_t *context,
                             system_health_observation_sink_t *sink) {
    if (!state || !context || !sink) return -1;
    linux_filesystem_collector_state_t *configuration = state;
    size_t resource_count = configuration->resource_count;
    if (resource_count > SYSTEM_HEALTH_MAX_FILESYSTEMS) {
        resource_count = SYSTEM_HEALTH_MAX_FILESYSTEMS;
        configuration->resources_dropped +=
            configuration->resource_count - resource_count;
    }

    char default_mountinfo[LINUX_FILESYSTEM_PATH_LENGTH];
    const char *mountinfo = configuration->mountinfo_path[0]
        ? configuration->mountinfo_path : default_mountinfo;
    if (!configuration->mountinfo_path[0]) {
        const char *proc_root = context->proc_root
            ? context->proc_root : "/proc";
        int length = snprintf(default_mountinfo, sizeof(default_mountinfo),
                              "%s/%s", proc_root, "self/mountinfo");
        if (length < 0 || (size_t)length >= sizeof(default_mountinfo)) {
            mountinfo = NULL;
        }
    }

    for (size_t index = 0; index < resource_count; ++index) {
        linux_filesystem_sample_t sample;
        if (linux_filesystem_sample(&configuration->resources[index],
                                    mountinfo, &sample) != 0) continue;
        emit_flag(sink, context, &sample, "filesystem.mount_present",
                  &sample.mount_present);
        emit_flag(sink, context, &sample, "filesystem.read_only",
                  &sample.read_only);
        emit_value(sink, context, &sample, "filesystem.flags",
                   &sample.mount_flags, SYSTEM_HEALTH_UNIT_COUNT);
        emit_value(sink, context, &sample, "filesystem.capacity_bytes",
                   &sample.capacity_bytes, SYSTEM_HEALTH_UNIT_BYTES);
        emit_value(sink, context, &sample, "filesystem.available_bytes",
                   &sample.available_bytes, SYSTEM_HEALTH_UNIT_BYTES);
        emit_value(sink, context, &sample, "filesystem.capacity_inodes",
                   &sample.capacity_inodes, SYSTEM_HEALTH_UNIT_COUNT);
        emit_value(sink, context, &sample, "filesystem.available_inodes",
                   &sample.available_inodes, SYSTEM_HEALTH_UNIT_COUNT);
    }
    return 0;
}

void linux_filesystem_collector_init(system_health_collector_t *collector,
                                     linux_filesystem_collector_state_t *state) {
    if (!collector) return;
    memset(collector, 0, sizeof(*collector));
    snprintf(collector->name, sizeof(collector->name), "%s",
             "linux_filesystem");
    collector->scope = SYSTEM_HEALTH_SCOPE_FILESYSTEM;
    collector->tier = SYSTEM_HEALTH_TIER_NORMAL;
    collector->interval_seconds = 30U;
    collector->stale_after_seconds = 90U;
    collector->state = state;
    collector->collect = linux_filesystem_collect;
}
