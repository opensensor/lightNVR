#define _POSIX_C_SOURCE 200809L

#include "telemetry/providers/nvme_provider.h"

#include <ctype.h>
#include <dirent.h>
#include <errno.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#if defined(__linux__) && defined(__has_include)
#if __has_include(<linux/nvme_ioctl.h>)
#include <linux/nvme_ioctl.h>
#include <sys/ioctl.h>
#define LIGHTNVR_HAVE_NVME_IOCTL 1
#endif
#endif

#define NVME_SCAN_MAX 64U

static system_health_capability_t capability_from_errno(int error_number) {
    if (error_number == EACCES || error_number == EPERM)
        return SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED;
    if (error_number == ENOENT || error_number == ENODEV ||
        error_number == ENOTDIR || error_number == ENOTSUP ||
        error_number == ENOTTY)
        return SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
    return SYSTEM_HEALTH_CAPABILITY_ERROR;
}

static int default_open(const char *path, int flags) {
    return open(path, flags);
}

static int default_close(int descriptor) { return close(descriptor); }

static int default_health_log(int descriptor, uint8_t *output,
                              size_t output_size) {
#ifdef LIGHTNVR_HAVE_NVME_IOCTL
    if (!output || output_size != NVME_PROVIDER_HEALTH_LOG_SIZE) {
        errno = EINVAL;
        return -1;
    }
    struct nvme_admin_cmd command;
    memset(&command, 0, sizeof(command));
    command.opcode = 0x02U; /* Get Log Page. */
    command.nsid = UINT32_MAX;
    command.addr = (uint64_t)(uintptr_t)output;
    command.data_len = (uint32_t)output_size;
    command.cdw10 = (((uint32_t)output_size / 4U - 1U) << 16U) | 0x02U;
    return ioctl(descriptor, NVME_IOCTL_ADMIN_CMD, &command);
#else
    (void)descriptor;
    (void)output;
    (void)output_size;
    errno = ENOTSUP;
    return -1;
#endif
}

static bool controller_name(const char *name) {
    if (!name || strncmp(name, "nvme", 4U) != 0 ||
        !isdigit((unsigned char)name[4])) return false;
    const char *cursor = name + 4U;
    while (isdigit((unsigned char)*cursor)) cursor++;
    return *cursor == '\0' && strlen(name) < LINUX_HARDWARE_INTERNAL_NAME_LENGTH;
}

static int compare_names(const void *left, const void *right) {
    return strcmp((const char *)left, (const char *)right);
}

static uint64_t scoped_hash(const char *scope, const char *name) {
    uint64_t hash = UINT64_C(1469598103934665603);
    const char *parts[] = {scope, "|linux-block|", name};
    for (size_t part = 0U; part < sizeof(parts) / sizeof(parts[0]); ++part)
        for (const unsigned char *cursor =
                 (const unsigned char *)parts[part]; *cursor; ++cursor) {
            hash ^= *cursor;
            hash *= UINT64_C(1099511628211);
        }
    return hash;
}

void nvme_provider_state_init(nvme_provider_state_t *state,
                              const char *installation_scope) {
    if (!state) return;
    memset(state, 0, sizeof(*state));
    state->discovery_capability = SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
    snprintf(state->dev_root, sizeof(state->dev_root), "%s", "/dev");
    if (installation_scope)
        snprintf(state->installation_scope, sizeof(state->installation_scope),
                 "%s", installation_scope);
    state->ops.open_device = default_open;
    state->ops.read_health_log = default_health_log;
    state->ops.close_device = default_close;
}

static const nvme_provider_device_t *old_device(
    const nvme_provider_device_t *devices, size_t count, const char *name) {
    for (size_t index = 0U; index < count; ++index)
        if (strcmp(devices[index].internal_name, name) == 0)
            return &devices[index];
    return NULL;
}

int nvme_provider_discover(void *opaque,
                           const system_health_collect_context_t *context,
                           system_health_provider_inventory_t *inventory) {
    nvme_provider_state_t *state = opaque;
    if (!state || !context || !context->sys_root || !inventory) return -1;
    memset(inventory, 0, sizeof(*inventory));
    nvme_provider_device_t previous[NVME_PROVIDER_MAX_DEVICES];
    size_t previous_count = state->device_count;
    memcpy(previous, state->devices, sizeof(previous));
    memset(state->devices, 0, sizeof(state->devices));
    state->device_count = 0U;
    state->devices_dropped = 0U;
    int root_length = snprintf(state->discovered_sys_root,
                               sizeof(state->discovered_sys_root), "%s",
                               context->sys_root);
    if (root_length < 0 ||
        (size_t)root_length >= sizeof(state->discovered_sys_root) ||
        !state->installation_scope[0]) {
        state->discovery_capability = SYSTEM_HEALTH_CAPABILITY_ERROR;
        goto unavailable;
    }

    char path[1200];
    int written = snprintf(path, sizeof(path), "%s/class/nvme",
                           context->sys_root);
    if (written < 0 || (size_t)written >= sizeof(path)) {
        state->discovery_capability = SYSTEM_HEALTH_CAPABILITY_ERROR;
        goto unavailable;
    }
    DIR *directory = opendir(path);
    if (!directory) {
        state->discovery_capability = capability_from_errno(errno);
        goto unavailable;
    }
    char names[NVME_SCAN_MAX][LINUX_HARDWARE_INTERNAL_NAME_LENGTH];
    size_t name_count = 0U;
    struct dirent *entry;
    while ((entry = readdir(directory)) != NULL) {
        if (!controller_name(entry->d_name)) continue;
        if (name_count >= NVME_SCAN_MAX) {
            state->devices_dropped++;
            break;
        }
        size_t length = strlen(entry->d_name);
        memcpy(names[name_count], entry->d_name, length + 1U);
        name_count++;
    }
    closedir(directory);
    qsort(names, name_count, sizeof(names[0]), compare_names);

    linux_device_map_t map;
    (void)linux_device_map_build(context->sys_root,
                                 state->installation_scope, &map);
    for (size_t index = 0U; index < name_count; ++index) {
        if (state->device_count >= NVME_PROVIDER_MAX_DEVICES) {
            state->devices_dropped++;
            continue;
        }
        nvme_provider_device_t *device =
            &state->devices[state->device_count++];
        size_t internal_length = strlen(names[index]);
        memcpy(device->internal_name, names[index], internal_length + 1U);
        const linux_device_map_entry_t *mapped =
            map.capability == SYSTEM_HEALTH_CAPABILITY_AVAILABLE
                ? linux_device_map_find(&map, names[index]) : NULL;
        if (mapped) {
            snprintf(device->public_id, sizeof(device->public_id), "%s",
                     mapped->public_id);
        } else {
            snprintf(device->public_id, sizeof(device->public_id),
                     "device.%016llx", (unsigned long long)scoped_hash(
                         state->installation_scope, names[index]));
        }
        const nvme_provider_device_t *old = old_device(
            previous, previous_count, device->internal_name);
        if (old) {
            device->previous_media_errors = old->previous_media_errors;
            device->previous_unsafe_shutdowns = old->previous_unsafe_shutdowns;
            device->previous_monotonic_ms = old->previous_monotonic_ms;
            device->media_previous_valid = old->media_previous_valid;
            device->unsafe_previous_valid = old->unsafe_previous_valid;
        }
    }
    state->devices_dropped += map.dropped;
    state->discovery_capability = state->device_count > 0U
        ? SYSTEM_HEALTH_CAPABILITY_AVAILABLE
        : SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;

unavailable:
    for (size_t index = 0U; index < state->device_count; ++index) {
        snprintf(inventory->resources[index].id,
                 sizeof(inventory->resources[index].id), "%s",
                 state->devices[index].public_id);
        inventory->resources[index].scope = SYSTEM_HEALTH_SCOPE_DEVICE;
        inventory->resources[index].capability =
            SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    }
    inventory->count = state->device_count;
    inventory->dropped = state->devices_dropped;
    if (inventory->count == 0U) {
        snprintf(inventory->resources[0].id,
                 sizeof(inventory->resources[0].id), "%s", "nvme");
        inventory->resources[0].scope = SYSTEM_HEALTH_SCOPE_DEVICE;
        inventory->resources[0].capability = state->discovery_capability;
        inventory->count = 1U;
    }
    return 0;
}

static void emit(system_health_observation_sink_t *sink,
                 const system_health_collect_context_t *context,
                 const char *metric, const char *resource,
                 system_health_capability_t capability, bool valid,
                 double value, system_health_unit_t unit) {
    system_health_observation_t observation;
    memset(&observation, 0, sizeof(observation));
    snprintf(observation.metric, sizeof(observation.metric), "%s", metric);
    snprintf(observation.resource_id, sizeof(observation.resource_id), "%s",
             resource);
    observation.scope = SYSTEM_HEALTH_SCOPE_DEVICE;
    observation.sampled_monotonic_ms = context->monotonic_ms;
    observation.observed_wall_time_ms = context->wall_time_ms;
    if (valid)
        system_health_observation_set_available(&observation, value, unit);
    else
        system_health_observation_set_unavailable(&observation, capability);
    (void)system_health_observation_sink_append(sink, &observation);
}

static uint16_t little_u16(const uint8_t *value) {
    return (uint16_t)value[0] | (uint16_t)((uint16_t)value[1] << 8U);
}

static bool little_u128_as_u64(const uint8_t *value, uint64_t *output) {
    for (size_t index = 8U; index < 16U; ++index)
        if (value[index] != 0U) return false;
    uint64_t parsed = 0U;
    for (size_t index = 0U; index < 8U; ++index)
        parsed |= (uint64_t)value[index] << (index * 8U);
    *output = parsed;
    return true;
}

static void emit_unavailable_device(
    const nvme_provider_device_t *device,
    const system_health_collect_context_t *context,
    system_health_observation_sink_t *sink,
    system_health_capability_t capability) {
    static const char *const metrics[] = {
        "storage.device.available_spare_ratio",
        "storage.device.percentage_used_ratio",
        "storage.device.temperature_celsius",
        "storage.device.media_errors_delta",
        "storage.device.unsafe_shutdowns_delta",
        "storage.device.prefail", "storage.device.critical"
    };
    static const system_health_unit_t units[] = {
        SYSTEM_HEALTH_UNIT_RATIO, SYSTEM_HEALTH_UNIT_RATIO,
        SYSTEM_HEALTH_UNIT_CELSIUS, SYSTEM_HEALTH_UNIT_COUNT,
        SYSTEM_HEALTH_UNIT_COUNT, SYSTEM_HEALTH_UNIT_BOOLEAN,
        SYSTEM_HEALTH_UNIT_BOOLEAN
    };
    for (size_t index = 0U; index < sizeof(metrics) / sizeof(metrics[0]);
         ++index)
        emit(sink, context, metrics[index], device->public_id, capability,
             false, 0.0, units[index]);
}

static void emit_delta(nvme_provider_device_t *device, uint64_t current,
                       bool *previous_valid, uint64_t *previous,
                       const char *metric,
                       const system_health_collect_context_t *context,
                       system_health_observation_sink_t *sink) {
    bool valid = *previous_valid &&
                 context->monotonic_ms > device->previous_monotonic_ms &&
                 current >= *previous;
    system_health_capability_t capability = !*previous_valid
        ? SYSTEM_HEALTH_CAPABILITY_STALE : SYSTEM_HEALTH_CAPABILITY_ERROR;
    double delta = valid ? (double)(current - *previous) : 0.0;
    *previous = current;
    *previous_valid = true;
    emit(sink, context, metric, device->public_id, capability, valid, delta,
         SYSTEM_HEALTH_UNIT_COUNT);
}

static void collect_device(nvme_provider_state_t *state,
                           nvme_provider_device_t *device,
                           const system_health_collect_context_t *context,
                           system_health_observation_sink_t *sink) {
    char path[512];
    int written = snprintf(path, sizeof(path), "%s/%s", state->dev_root,
                           device->internal_name);
    if (written < 0 || (size_t)written >= sizeof(path)) {
        emit_unavailable_device(device, context, sink,
                                SYSTEM_HEALTH_CAPABILITY_ERROR);
        return;
    }
    int descriptor = state->ops.open_device(path,
                                             O_RDONLY | O_NONBLOCK | O_CLOEXEC);
    if (descriptor < 0) {
        emit_unavailable_device(device, context, sink,
                                capability_from_errno(errno));
        device->media_previous_valid = false;
        device->unsafe_previous_valid = false;
        return;
    }
    uint8_t log[NVME_PROVIDER_HEALTH_LOG_SIZE];
    memset(log, 0, sizeof(log));
    int result = state->ops.read_health_log(descriptor, log, sizeof(log));
    int saved = errno;
    (void)state->ops.close_device(descriptor);
    if (result != 0) {
        emit_unavailable_device(device, context, sink,
                                capability_from_errno(saved));
        device->media_previous_valid = false;
        device->unsafe_previous_valid = false;
        return;
    }

    bool spare_valid = log[3] <= 100U;
    double spare_ratio = (double)log[3] / 100.0;
    double used_ratio = (double)log[5] / 100.0;
    if (used_ratio > 1.0) used_ratio = 1.0;
    emit(sink, context, "storage.device.available_spare_ratio",
         device->public_id,
         spare_valid ? SYSTEM_HEALTH_CAPABILITY_AVAILABLE
                     : SYSTEM_HEALTH_CAPABILITY_ERROR,
         spare_valid,
         spare_ratio, SYSTEM_HEALTH_UNIT_RATIO);
    emit(sink, context, "storage.device.percentage_used_ratio",
         device->public_id, SYSTEM_HEALTH_CAPABILITY_AVAILABLE, true,
         used_ratio, SYSTEM_HEALTH_UNIT_RATIO);
    uint16_t kelvin = little_u16(log + 1U);
    bool temperature_valid = kelvin >= 173U && kelvin <= 573U;
    emit(sink, context, "storage.device.temperature_celsius",
         device->public_id, temperature_valid
             ? SYSTEM_HEALTH_CAPABILITY_AVAILABLE
             : SYSTEM_HEALTH_CAPABILITY_ERROR,
         temperature_valid, (double)kelvin - 273.15,
         SYSTEM_HEALTH_UNIT_CELSIUS);
    bool critical = log[0] != 0U;
    bool prefail = critical || (log[4] > 0U && log[3] < log[4]) ||
                   log[5] >= 90U;
    emit(sink, context, "storage.device.prefail", device->public_id,
         SYSTEM_HEALTH_CAPABILITY_AVAILABLE, true, prefail ? 1.0 : 0.0,
         SYSTEM_HEALTH_UNIT_BOOLEAN);
    emit(sink, context, "storage.device.critical", device->public_id,
         SYSTEM_HEALTH_CAPABILITY_AVAILABLE, true, critical ? 1.0 : 0.0,
         SYSTEM_HEALTH_UNIT_BOOLEAN);

    uint64_t current = 0U;
    if (little_u128_as_u64(log + 160U, &current)) {
        emit_delta(device, current, &device->media_previous_valid,
                   &device->previous_media_errors,
                   "storage.device.media_errors_delta", context, sink);
    } else {
        device->media_previous_valid = false;
        emit(sink, context, "storage.device.media_errors_delta",
             device->public_id, SYSTEM_HEALTH_CAPABILITY_ERROR, false, 0.0,
             SYSTEM_HEALTH_UNIT_COUNT);
    }
    if (little_u128_as_u64(log + 144U, &current)) {
        emit_delta(device, current, &device->unsafe_previous_valid,
                   &device->previous_unsafe_shutdowns,
                   "storage.device.unsafe_shutdowns_delta", context, sink);
    } else {
        device->unsafe_previous_valid = false;
        emit(sink, context, "storage.device.unsafe_shutdowns_delta",
             device->public_id, SYSTEM_HEALTH_CAPABILITY_ERROR, false, 0.0,
             SYSTEM_HEALTH_UNIT_COUNT);
    }
    device->previous_monotonic_ms = context->monotonic_ms;
}

int nvme_provider_collect(void *opaque,
                          const system_health_collect_context_t *context,
                          system_health_observation_sink_t *sink) {
    nvme_provider_state_t *state = opaque;
    if (!state || !context || !sink || !state->ops.open_device ||
        !state->ops.read_health_log || !state->ops.close_device) return -1;
    emit(sink, context, "hardware.provider.visible", "nvme",
         state->discovery_capability,
         state->discovery_capability == SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
         1.0, SYSTEM_HEALTH_UNIT_BOOLEAN);
    for (size_t index = 0U; index < state->device_count; ++index)
        collect_device(state, &state->devices[index], context, sink);
    sink->dropped += state->devices_dropped;
    return 0;
}

void nvme_provider_init(system_health_provider_t *provider,
                        nvme_provider_state_t *state) {
    if (!provider) return;
    memset(provider, 0, sizeof(*provider));
    snprintf(provider->name, sizeof(provider->name), "%s", "nvme");
    provider->capability = SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
    provider->state = state;
    provider->discover = nvme_provider_discover;
    provider->collect = nvme_provider_collect;
}
