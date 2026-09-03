#define _POSIX_C_SOURCE 200809L

#include "telemetry/providers/linux_hardware.h"

#include <ctype.h>
#include <dirent.h>
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>

#include "core/event_identity.h"

#define HARDWARE_TEXT_MAX 128U
#define HARDWARE_SCAN_MAX 128U

typedef struct {
    bool valid;
    uint64_t value;
    system_health_capability_t capability;
} hardware_integer_t;

static system_health_capability_t capability_from_errno(int error_number) {
    if (error_number == EACCES || error_number == EPERM)
        return SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED;
    if (error_number == ENOENT || error_number == ENOTDIR)
        return SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
    return SYSTEM_HEALTH_CAPABILITY_ERROR;
}

static system_health_capability_t merge_capability(
    system_health_capability_t left, system_health_capability_t right) {
    if (left == SYSTEM_HEALTH_CAPABILITY_AVAILABLE ||
        right == SYSTEM_HEALTH_CAPABILITY_AVAILABLE)
        return SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    if (left == SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED ||
        right == SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED)
        return SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED;
    if (left == SYSTEM_HEALTH_CAPABILITY_ERROR ||
        right == SYSTEM_HEALTH_CAPABILITY_ERROR)
        return SYSTEM_HEALTH_CAPABILITY_ERROR;
    return SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
}

static bool suffix_number(const char *name, const char *prefix,
                          unsigned int *number) {
    if (!name || !prefix || strncmp(name, prefix, strlen(prefix)) != 0)
        return false;
    const char *cursor = name + strlen(prefix);
    if (!isdigit((unsigned char)*cursor)) return false;
    errno = 0;
    char *end = NULL;
    unsigned long parsed = strtoul(cursor, &end, 10);
    if (errno == ERANGE || end == cursor || *end || parsed > 999U) return false;
    if (number) *number = (unsigned int)parsed;
    return true;
}

static bool format_path(char *output, size_t capacity, const char *format,
                        const char *root, const char *name,
                        unsigned int channel) {
    int written = snprintf(output, capacity, format, root, name, channel);
    return written >= 0 && (size_t)written < capacity;
}

static bool join_path(char *output, size_t capacity, const char *left,
                      const char *right) {
    size_t left_length = strlen(left), right_length = strlen(right);
    if (left_length >= capacity || right_length >= capacity ||
        left_length + 1U + right_length >= capacity) return false;
    memcpy(output, left, left_length);
    output[left_length] = '/';
    memcpy(output + left_length + 1U, right, right_length + 1U);
    return true;
}

static bool read_text(const char *path, char output[HARDWARE_TEXT_MAX],
                      system_health_capability_t *capability) {
    FILE *file = fopen(path, "r");
    if (!file) {
        *capability = capability_from_errno(errno);
        return false;
    }
    size_t length = fread(output, 1U, HARDWARE_TEXT_MAX - 1U, file);
    if (ferror(file)) {
        int saved = errno;
        fclose(file);
        *capability = capability_from_errno(saved);
        return false;
    }
    if (length == HARDWARE_TEXT_MAX - 1U && fgetc(file) != EOF) {
        fclose(file);
        *capability = SYSTEM_HEALTH_CAPABILITY_ERROR;
        return false;
    }
    fclose(file);
    output[length] = '\0';
    while (length > 0U && isspace((unsigned char)output[length - 1U]))
        output[--length] = '\0';
    size_t start = 0U;
    while (isspace((unsigned char)output[start])) start++;
    if (start > 0U) memmove(output, output + start, strlen(output + start) + 1U);
    if (!output[0]) {
        *capability = SYSTEM_HEALTH_CAPABILITY_ERROR;
        return false;
    }
    *capability = SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    return true;
}

static hardware_integer_t read_integer(const char *path, int base) {
    hardware_integer_t result = {
        .capability = SYSTEM_HEALTH_CAPABILITY_ERROR
    };
    char text[HARDWARE_TEXT_MAX];
    if (!read_text(path, text, &result.capability)) return result;
    if (text[0] == '-' || text[0] == '+') {
        result.capability = SYSTEM_HEALTH_CAPABILITY_ERROR;
        return result;
    }
    errno = 0;
    char *end = NULL;
    unsigned long long parsed = strtoull(text, &end, base);
    if (errno == ERANGE || end == text) {
        result.capability = SYSTEM_HEALTH_CAPABILITY_ERROR;
        return result;
    }
    while (isspace((unsigned char)*end)) end++;
    if (*end) {
        result.capability = SYSTEM_HEALTH_CAPABILITY_ERROR;
        return result;
    }
    result.valid = true;
    result.value = (uint64_t)parsed;
    result.capability = SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    return result;
}

static uint64_t scoped_hash(const char *scope, const char *kind,
                            const char *name, unsigned int channel) {
    uint64_t hash = UINT64_C(1469598103934665603);
    char number[16];
    snprintf(number, sizeof(number), "%u", channel);
    const char *parts[] = {scope, "|", kind, "|", name, "|", number};
    for (size_t part = 0U; part < sizeof(parts) / sizeof(parts[0]); ++part)
        for (const unsigned char *cursor =
                 (const unsigned char *)parts[part]; *cursor; ++cursor) {
            hash ^= *cursor;
            hash *= UINT64_C(1099511628211);
        }
    return hash;
}

static bool add_resource(linux_hardware_state_t *state,
                         linux_hardware_resource_kind_t kind,
                         const char *internal_name, const char *public_id,
                         unsigned int channel) {
    for (size_t index = 0U; index < state->resource_count; ++index) {
        linux_hardware_resource_t *current = &state->resources[index];
        if (current->kind == kind && current->channel == channel &&
            strcmp(current->internal_name, internal_name) == 0) return true;
    }
    if (state->resource_count >= LINUX_HARDWARE_MAX_RESOURCES) {
        state->resources_dropped++;
        return false;
    }
    linux_hardware_resource_t *resource =
        &state->resources[state->resource_count++];
    memset(resource, 0, sizeof(*resource));
    resource->kind = kind;
    resource->channel = channel;
    snprintf(resource->internal_name, sizeof(resource->internal_name), "%s",
             internal_name);
    snprintf(resource->public_id, sizeof(resource->public_id), "%s",
             public_id);
    return true;
}

static int compare_resources(const void *left, const void *right) {
    const linux_hardware_resource_t *a = left;
    const linux_hardware_resource_t *b = right;
    int result = strcmp(a->public_id, b->public_id);
    return result != 0 ? result : (int)a->kind - (int)b->kind;
}

static bool flash_files_present(const char *root, const char *name) {
    static const char *const files[] = {
        "life_time", "life_time_estimation", "life_time_estimation_a",
        "pre_eol_info", "pre_eol"
    };
    char path[1200];
    struct stat info;
    for (size_t index = 0U; index < sizeof(files) / sizeof(files[0]); ++index) {
        int written = snprintf(path, sizeof(path), "%s/class/block/%s/device/%s",
                               root, name, files[index]);
        if (written >= 0 && (size_t)written < sizeof(path) &&
            stat(path, &info) == 0 && S_ISREG(info.st_mode)) return true;
    }
    return false;
}

static void discover_flash(linux_hardware_state_t *state, const char *root,
                           system_health_capability_t *capability) {
    linux_device_map_t map;
    if (linux_device_map_build(root, state->installation_scope, &map) != 0) {
        *capability = merge_capability(*capability,
                                      SYSTEM_HEALTH_CAPABILITY_ERROR);
        return;
    }
    if (map.capability != SYSTEM_HEALTH_CAPABILITY_AVAILABLE) {
        *capability = merge_capability(*capability, map.capability);
        return;
    }
    state->resources_dropped += map.dropped;
    for (size_t index = 0U; index < map.count; ++index) {
        const linux_device_map_entry_t *entry = &map.entries[index];
        if (flash_files_present(root, entry->sysfs_name))
            (void)add_resource(state, LINUX_HARDWARE_RESOURCE_FLASH,
                               entry->sysfs_name, entry->public_id, 0U);
    }
}

static void discover_edac(linux_hardware_state_t *state, const char *root,
                          system_health_capability_t *capability) {
    char path[1200];
    snprintf(path, sizeof(path), "%s/devices/system/edac/mc", root);
    DIR *directory = opendir(path);
    if (!directory) {
        *capability = merge_capability(*capability,
                                      capability_from_errno(errno));
        return;
    }
    struct dirent *entry;
    size_t scanned = 0U;
    while ((entry = readdir(directory)) != NULL) {
        if (++scanned > HARDWARE_SCAN_MAX) {
            state->resources_dropped++;
            break;
        }
        unsigned int index;
        if (!suffix_number(entry->d_name, "mc", &index)) continue;
        char public_id[SYSTEM_HEALTH_ID_LENGTH];
        snprintf(public_id, sizeof(public_id), "edac.mc%u", index);
        (void)add_resource(state, LINUX_HARDWARE_RESOURCE_EDAC,
                           entry->d_name, public_id, 0U);
    }
    closedir(directory);
}

static bool hwmon_has_board_flag(const char *root, const char *hwmon) {
    char path[1200];
    snprintf(path, sizeof(path), "%s/class/hwmon/%s", root, hwmon);
    DIR *directory = opendir(path);
    if (!directory) return false;
    bool found = false;
    struct dirent *entry;
    size_t scanned = 0U;
    while ((entry = readdir(directory)) != NULL) {
        if (++scanned > HARDWARE_SCAN_MAX) break;
        const char *name = entry->d_name;
        if ((strncmp(name, "in", 2) == 0 || strncmp(name, "power", 5) == 0 ||
             strncmp(name, "throttle", 8) == 0 ||
             strncmp(name, "throttled", 9) == 0) && strstr(name, "alarm")) {
            found = true;
            break;
        }
    }
    closedir(directory);
    return found;
}

static void discover_hwmon(linux_hardware_state_t *state, const char *root,
                           system_health_capability_t *capability) {
    char path[1200];
    snprintf(path, sizeof(path), "%s/class/hwmon", root);
    DIR *directory = opendir(path);
    if (!directory) {
        *capability = merge_capability(*capability,
                                      capability_from_errno(errno));
        return;
    }
    bool board = false;
    struct dirent *entry;
    size_t hwmons_scanned = 0U;
    while ((entry = readdir(directory)) != NULL) {
        if (++hwmons_scanned > HARDWARE_SCAN_MAX) {
            state->resources_dropped++;
            break;
        }
        if (!suffix_number(entry->d_name, "hwmon", NULL)) continue;
        char directory_path[1200];
        snprintf(directory_path, sizeof(directory_path), "%s/class/hwmon/%s",
                 root, entry->d_name);
        DIR *channels = opendir(directory_path);
        if (!channels) {
            *capability = merge_capability(*capability,
                                           capability_from_errno(errno));
            continue;
        }
        struct dirent *channel;
        size_t channels_scanned = 0U;
        while ((channel = readdir(channels)) != NULL) {
            if (++channels_scanned > HARDWARE_SCAN_MAX) {
                state->resources_dropped++;
                break;
            }
            unsigned int fan;
            if (sscanf(channel->d_name, "fan%u_input", &fan) != 1 ||
                fan == 0U || fan > 999U) continue;
            char expected[32];
            snprintf(expected, sizeof(expected), "fan%u_input", fan);
            if (strcmp(expected, channel->d_name) != 0) continue;
            char public_id[SYSTEM_HEALTH_ID_LENGTH];
            snprintf(public_id, sizeof(public_id), "fan.%016llx",
                     (unsigned long long)scoped_hash(
                         state->installation_scope, "fan", entry->d_name,
                         fan));
            (void)add_resource(state, LINUX_HARDWARE_RESOURCE_FAN,
                               entry->d_name, public_id, fan);
        }
        closedir(channels);
        board = board || hwmon_has_board_flag(root, entry->d_name);
    }
    closedir(directory);
    static const char *const known_flags[] = {
        "/devices/platform/soc/soc:firmware/get_throttled",
        "/devices/platform/firmware/get_throttled", "/firmware/get_throttled"
    };
    struct stat info;
    char known_path[1200];
    for (size_t index = 0U;
         index < sizeof(known_flags) / sizeof(known_flags[0]); ++index) {
        snprintf(known_path, sizeof(known_path), "%s%s", root,
                 known_flags[index]);
        if (stat(known_path, &info) == 0) board = true;
    }
    if (board)
        (void)add_resource(state, LINUX_HARDWARE_RESOURCE_BOARD, "board",
                           "board", 0U);
}

void linux_hardware_state_init(linux_hardware_state_t *state,
                               const char *installation_scope) {
    if (!state) return;
    memset(state, 0, sizeof(*state));
    state->fan_hot_celsius = 80.0;
    state->discovery_capability = SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
    if (installation_scope && installation_scope[0]) {
        snprintf(state->installation_scope, sizeof(state->installation_scope),
                 "%s", installation_scope);
    } else {
        (void)event_identity_get_source(state->installation_scope,
                                        sizeof(state->installation_scope));
    }
}

int linux_hardware_discover(void *opaque,
                            const system_health_collect_context_t *context,
                            system_health_provider_inventory_t *inventory) {
    linux_hardware_state_t *state = opaque;
    if (!state || !context || !context->sys_root || !inventory) return -1;
    memset(inventory, 0, sizeof(*inventory));
    state->resource_count = 0U;
    state->resources_dropped = 0U;
    int root_length = snprintf(state->discovered_sys_root,
                               sizeof(state->discovered_sys_root), "%s",
                               context->sys_root);
    system_health_capability_t capability =
        SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
    if (root_length < 0 ||
        (size_t)root_length >= sizeof(state->discovered_sys_root) ||
        !state->installation_scope[0]) {
        state->discovery_capability = SYSTEM_HEALTH_CAPABILITY_ERROR;
    } else {
        discover_flash(state, context->sys_root, &capability);
        discover_edac(state, context->sys_root, &capability);
        discover_hwmon(state, context->sys_root, &capability);
        if (state->resource_count > 0U)
            capability = SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
        state->discovery_capability = capability;
    }
    qsort(state->resources, state->resource_count, sizeof(state->resources[0]),
          compare_resources);
    for (size_t counter = 0U; counter < LINUX_HARDWARE_COUNTER_SLOTS;
         ++counter) {
        linux_hardware_counter_state_t *slot = &state->counters[counter];
        if (!slot->used) continue;
        bool present = false;
        for (size_t resource = 0U; resource < state->resource_count;
             ++resource) {
            if (strcmp(slot->public_id,
                       state->resources[resource].public_id) == 0) {
                present = true;
                break;
            }
        }
        if (!present) slot->previous_monotonic_ms = 0U;
    }
    for (size_t index = 0U; index < state->resource_count; ++index) {
        snprintf(inventory->resources[index].id,
                 sizeof(inventory->resources[index].id), "%s",
                 state->resources[index].public_id);
        inventory->resources[index].scope = SYSTEM_HEALTH_SCOPE_DEVICE;
        inventory->resources[index].capability =
            SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    }
    inventory->count = state->resource_count;
    inventory->dropped = state->resources_dropped;
    if (inventory->count == 0U) {
        snprintf(inventory->resources[0].id,
                 sizeof(inventory->resources[0].id), "%s", "linux_hardware");
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

static bool read_first(const char *root, const char *name,
                       const char *const *files, size_t count,
                       char output[HARDWARE_TEXT_MAX],
                       system_health_capability_t *capability) {
    *capability = SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
    char path[1200];
    for (size_t index = 0U; index < count; ++index) {
        snprintf(path, sizeof(path), "%s/class/block/%s/device/%s", root,
                 name, files[index]);
        system_health_capability_t current;
        if (read_text(path, output, &current)) return true;
        *capability = merge_capability(*capability, current);
    }
    return false;
}

static bool parse_byte(const char **cursor, uint8_t *output) {
    while (isspace((unsigned char)**cursor)) (*cursor)++;
    if (!**cursor || **cursor == '-' || **cursor == '+') return false;
    errno = 0;
    char *end = NULL;
    unsigned long value = strtoul(*cursor, &end, 0);
    if (errno == ERANGE || end == *cursor || value > UINT8_MAX) return false;
    *cursor = end;
    *output = (uint8_t)value;
    return true;
}

static bool only_whitespace(const char *cursor) {
    while (isspace((unsigned char)*cursor)) cursor++;
    return *cursor == '\0';
}

static void collect_flash(const linux_hardware_resource_t *resource,
                          const char *root,
                          const system_health_collect_context_t *context,
                          system_health_observation_sink_t *sink) {
    static const char *const life_files[] = {
        "life_time", "life_time_estimation", "life_time_estimation_a"
    };
    static const char *const pre_files[] = {"pre_eol_info", "pre_eol"};
    char text[HARDWARE_TEXT_MAX];
    system_health_capability_t life_capability;
    bool life_valid = false;
    uint8_t life_a = 0U, life_b = 0U;
    if (read_first(root, resource->internal_name, life_files,
                   sizeof(life_files) / sizeof(life_files[0]), text,
                   &life_capability)) {
        const char *cursor = text;
        life_valid = parse_byte(&cursor, &life_a);
        uint8_t second;
        if (life_valid && parse_byte(&cursor, &second)) life_b = second;
        if (life_valid && !only_whitespace(cursor)) life_valid = false;
        if (!life_valid) life_capability = SYSTEM_HEALTH_CAPABILITY_ERROR;
    }
    /* Some UFS exports split A/B into separate attributes. */
    char b_path[1200];
    snprintf(b_path, sizeof(b_path), "%s/class/block/%s/device/"
             "life_time_estimation_b", root, resource->internal_name);
    hardware_integer_t separate_b = read_integer(b_path, 0);
    if (separate_b.valid && separate_b.value <= UINT8_MAX) {
        life_b = (uint8_t)separate_b.value;
        life_valid = life_valid || life_b != 0U;
        life_capability = SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    }
    uint8_t life = life_a > life_b ? life_a : life_b;
    bool life_defined = life_valid && life >= 1U && life <= 11U;
    emit(sink, context, "storage.device.life_used_ratio", resource->public_id,
         life_valid && !life_defined ? SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED
                                     : life_capability,
         life_defined, life >= 10U ? 1.0 : (double)life / 10.0,
         SYSTEM_HEALTH_UNIT_RATIO);

    system_health_capability_t pre_capability;
    bool pre_valid = false;
    uint8_t pre_eol = 0U;
    if (read_first(root, resource->internal_name, pre_files,
                   sizeof(pre_files) / sizeof(pre_files[0]), text,
                   &pre_capability)) {
        const char *cursor = text;
        pre_valid = parse_byte(&cursor, &pre_eol) &&
                    only_whitespace(cursor) && pre_eol >= 1U &&
                    pre_eol <= 3U;
        if (!pre_valid) pre_capability = SYSTEM_HEALTH_CAPABILITY_ERROR;
    }
    emit(sink, context, "storage.device.pre_eol", resource->public_id,
         pre_capability, pre_valid, (double)pre_eol, SYSTEM_HEALTH_UNIT_COUNT);
    bool prefail_valid = life_defined || pre_valid;
    bool prefail = (life_defined && life >= 10U) ||
                   (pre_valid && pre_eol >= 2U);
    system_health_capability_t prefail_capability = merge_capability(
        life_defined ? SYSTEM_HEALTH_CAPABILITY_AVAILABLE : life_capability,
        pre_valid ? SYSTEM_HEALTH_CAPABILITY_AVAILABLE : pre_capability);
    emit(sink, context, "storage.device.prefail", resource->public_id,
         prefail_capability, prefail_valid, prefail ? 1.0 : 0.0,
         SYSTEM_HEALTH_UNIT_BOOLEAN);
    bool critical = (life_defined && life >= 11U) ||
                    (pre_valid && pre_eol >= 3U);
    emit(sink, context, "storage.device.critical", resource->public_id,
         prefail_capability, prefail_valid, critical ? 1.0 : 0.0,
         SYSTEM_HEALTH_UNIT_BOOLEAN);
}

static linux_hardware_counter_state_t *counter_slot(
    linux_hardware_state_t *state, const char *resource, const char *metric) {
    linux_hardware_counter_state_t *empty = NULL;
    for (size_t index = 0U; index < LINUX_HARDWARE_COUNTER_SLOTS; ++index) {
        linux_hardware_counter_state_t *slot = &state->counters[index];
        if (slot->used && strcmp(slot->public_id, resource) == 0 &&
            strcmp(slot->metric, metric) == 0) return slot;
        if (!slot->used && !empty) empty = slot;
    }
    if (!empty) return NULL;
    empty->used = true;
    snprintf(empty->public_id, sizeof(empty->public_id), "%s", resource);
    snprintf(empty->metric, sizeof(empty->metric), "%s", metric);
    return empty;
}

static void emit_counter(linux_hardware_state_t *state,
                         const system_health_collect_context_t *context,
                         system_health_observation_sink_t *sink,
                         const char *metric, const char *resource,
                         hardware_integer_t reading) {
    linux_hardware_counter_state_t *slot = counter_slot(state, resource, metric);
    if (!slot) {
        emit(sink, context, metric, resource, SYSTEM_HEALTH_CAPABILITY_ERROR,
             false, 0.0, SYSTEM_HEALTH_UNIT_COUNT);
        return;
    }
    if (!reading.valid) {
        slot->previous_monotonic_ms = 0U;
        emit(sink, context, metric, resource, reading.capability, false, 0.0,
             SYSTEM_HEALTH_UNIT_COUNT);
        return;
    }
    bool valid = slot->previous_monotonic_ms > 0U &&
                 context->monotonic_ms > slot->previous_monotonic_ms &&
                 reading.value >= slot->previous;
    system_health_capability_t capability =
        slot->previous_monotonic_ms == 0U ? SYSTEM_HEALTH_CAPABILITY_STALE
                                          : SYSTEM_HEALTH_CAPABILITY_ERROR;
    double delta = valid ? (double)(reading.value - slot->previous) : 0.0;
    slot->previous = reading.value;
    slot->previous_monotonic_ms = context->monotonic_ms;
    emit(sink, context, metric, resource, capability, valid, delta,
         SYSTEM_HEALTH_UNIT_COUNT);
}

static void collect_edac(linux_hardware_state_t *state,
                         const linux_hardware_resource_t *resource,
                         const char *root,
                         const system_health_collect_context_t *context,
                         system_health_observation_sink_t *sink) {
    char path[1200];
    snprintf(path, sizeof(path), "%s/devices/system/edac/mc/%s/ce_count",
             root, resource->internal_name);
    hardware_integer_t corrected = read_integer(path, 10);
    snprintf(path, sizeof(path), "%s/devices/system/edac/mc/%s/ue_count",
             root, resource->internal_name);
    hardware_integer_t uncorrectable = read_integer(path, 10);
    emit_counter(state, context, sink, "hardware.ecc.corrected_delta",
                 resource->public_id, corrected);
    emit_counter(state, context, sink, "hardware.ecc.uncorrectable_delta",
                 resource->public_id, uncorrectable);
}

static void collect_fan(const linux_hardware_state_t *state,
                        const linux_hardware_resource_t *resource,
                        const char *root,
                        const system_health_collect_context_t *context,
                        system_health_observation_sink_t *sink) {
    char path[1200];
    format_path(path, sizeof(path), "%s/class/hwmon/%s/fan%u_input", root,
                resource->internal_name, resource->channel);
    hardware_integer_t rpm = read_integer(path, 10);
    format_path(path, sizeof(path), "%s/class/hwmon/%s/fan%u_min", root,
                resource->internal_name, resource->channel);
    hardware_integer_t minimum = read_integer(path, 10);
    emit(sink, context, "hardware.fan.rpm", resource->public_id,
         rpm.capability, rpm.valid, (double)rpm.value, SYSTEM_HEALTH_UNIT_COUNT);
    emit(sink, context, "hardware.fan.minimum_rpm", resource->public_id,
         minimum.capability, minimum.valid, (double)minimum.value,
         SYSTEM_HEALTH_UNIT_COUNT);

    bool temperature_valid = false;
    double temperature = 0.0, hot_limit = state->fan_hot_celsius;
    for (unsigned int channel = 1U; channel <= 32U; ++channel) {
        format_path(path, sizeof(path), "%s/class/hwmon/%s/temp%u_input", root,
                    resource->internal_name, channel);
        hardware_integer_t input = read_integer(path, 10);
        if (!input.valid) continue;
        double current = (double)input.value / 1000.0;
        if (!temperature_valid || current > temperature) temperature = current;
        temperature_valid = true;
        format_path(path, sizeof(path), "%s/class/hwmon/%s/temp%u_max", root,
                    resource->internal_name, channel);
        hardware_integer_t maximum = read_integer(path, 10);
        if (maximum.valid && maximum.value > 0U) {
            double candidate = (double)maximum.value / 1000.0;
            if (candidate < hot_limit) hot_limit = candidate;
        }
    }
    bool hot = temperature_valid && temperature >= hot_limit;
    bool stopped = rpm.valid &&
        (rpm.value == 0U || (minimum.valid && minimum.value > 0U &&
                            rpm.value < minimum.value));
    emit(sink, context, "hardware.fan.hot", resource->public_id,
         temperature_valid ? SYSTEM_HEALTH_CAPABILITY_AVAILABLE
                           : SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
         temperature_valid, hot ? 1.0 : 0.0, SYSTEM_HEALTH_UNIT_BOOLEAN);
    system_health_capability_t failed_capability = rpm.valid
        ? (temperature_valid ? SYSTEM_HEALTH_CAPABILITY_AVAILABLE
                             : SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED)
        : rpm.capability;
    emit(sink, context, "hardware.fan.failed", resource->public_id,
         failed_capability,
         rpm.valid && temperature_valid, stopped ? 1.0 : 0.0,
         SYSTEM_HEALTH_UNIT_BOOLEAN);
}

static void collect_board(const linux_hardware_resource_t *resource,
                          const char *root,
                          const system_health_collect_context_t *context,
                          system_health_observation_sink_t *sink) {
    (void)resource;
    system_health_capability_t capability =
        SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
    bool valid = false, power = false, throttled = false;
    static const char *const known_flags[] = {
        "/devices/platform/soc/soc:firmware/get_throttled",
        "/devices/platform/firmware/get_throttled", "/firmware/get_throttled"
    };
    char path[1200];
    for (size_t index = 0U;
         index < sizeof(known_flags) / sizeof(known_flags[0]); ++index) {
        snprintf(path, sizeof(path), "%s%s", root, known_flags[index]);
        hardware_integer_t value = read_integer(path, 0);
        capability = merge_capability(capability, value.capability);
        if (value.valid) {
            valid = true;
            power = power || (value.value & (UINT64_C(1) | UINT64_C(1) << 16));
            throttled = throttled ||
                (value.value & (UINT64_C(1) << 2 | UINT64_C(1) << 18));
        }
    }
    char hwmon_root[1200];
    snprintf(hwmon_root, sizeof(hwmon_root), "%s/class/hwmon", root);
    DIR *hwmons = opendir(hwmon_root);
    if (hwmons) {
        struct dirent *hwmon;
        size_t hwmons_scanned = 0U;
        while ((hwmon = readdir(hwmons)) != NULL) {
            if (++hwmons_scanned > HARDWARE_SCAN_MAX) break;
            unsigned int ignored;
            if (!suffix_number(hwmon->d_name, "hwmon", &ignored)) continue;
            char directory_path[1200];
            if (!join_path(directory_path, sizeof(directory_path), hwmon_root,
                           hwmon->d_name)) continue;
            DIR *entries = opendir(directory_path);
            if (!entries) continue;
            struct dirent *entry;
            size_t entries_scanned = 0U;
            while ((entry = readdir(entries)) != NULL) {
                if (++entries_scanned > HARDWARE_SCAN_MAX) break;
                const char *name = entry->d_name;
                bool power_alarm = (strncmp(name, "in", 2) == 0 ||
                    strncmp(name, "power", 5) == 0) && strstr(name, "alarm");
                bool throttle_alarm =
                    (strncmp(name, "throttle", 8) == 0 ||
                     strncmp(name, "throttled", 9) == 0) &&
                    strstr(name, "alarm");
                if (!power_alarm && !throttle_alarm) continue;
                if (!join_path(path, sizeof(path), directory_path, name))
                    continue;
                hardware_integer_t value = read_integer(path, 0);
                capability = merge_capability(capability, value.capability);
                if (!value.valid) continue;
                valid = true;
                if (power_alarm) power = power || value.value != 0U;
                if (throttle_alarm) throttled = throttled || value.value != 0U;
            }
            closedir(entries);
        }
        closedir(hwmons);
    }
    emit(sink, context, "hardware.power.unstable", "board", capability, valid,
         power ? 1.0 : 0.0, SYSTEM_HEALTH_UNIT_BOOLEAN);
    emit(sink, context, "hardware.throttled", "board", capability, valid,
         throttled ? 1.0 : 0.0, SYSTEM_HEALTH_UNIT_BOOLEAN);
}

int linux_hardware_collect(void *opaque,
                           const system_health_collect_context_t *context,
                           system_health_observation_sink_t *sink) {
    linux_hardware_state_t *state = opaque;
    if (!state || !context || !context->sys_root || !sink) return -1;
    emit(sink, context, "hardware.provider.visible", "linux_hardware",
         state->discovery_capability,
         state->discovery_capability == SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
         1.0, SYSTEM_HEALTH_UNIT_BOOLEAN);
    for (size_t index = 0U; index < state->resource_count; ++index) {
        const linux_hardware_resource_t *resource = &state->resources[index];
        switch (resource->kind) {
            case LINUX_HARDWARE_RESOURCE_FLASH:
                collect_flash(resource, context->sys_root, context, sink);
                break;
            case LINUX_HARDWARE_RESOURCE_EDAC:
                collect_edac(state, resource, context->sys_root, context, sink);
                break;
            case LINUX_HARDWARE_RESOURCE_FAN:
                collect_fan(state, resource, context->sys_root, context, sink);
                break;
            case LINUX_HARDWARE_RESOURCE_BOARD:
                collect_board(resource, context->sys_root, context, sink);
                break;
        }
    }
    sink->dropped += state->resources_dropped;
    return 0;
}

void linux_hardware_provider_init(system_health_provider_t *provider,
                                  linux_hardware_state_t *state) {
    if (!provider) return;
    memset(provider, 0, sizeof(*provider));
    snprintf(provider->name, sizeof(provider->name), "%s", "linux_hardware");
    provider->capability = SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
    provider->state = state;
    provider->discover = linux_hardware_discover;
    provider->collect = linux_hardware_collect;
}
