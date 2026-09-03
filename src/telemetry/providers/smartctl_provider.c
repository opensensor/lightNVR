#define _POSIX_C_SOURCE 200809L

#include "telemetry/providers/smartctl_provider.h"

#include <cjson/cJSON.h>
#include <ctype.h>
#include <errno.h>
#include <math.h>
#include <stdio.h>
#include <string.h>

#include "telemetry/system_health.h"

#define SMARTCTL_EXIT_COMMAND_ERROR 0x01
#define SMARTCTL_EXIT_DEVICE_OR_POWER 0x02
#define SMARTCTL_EXIT_SMART_COMMAND 0x04
#define SMARTCTL_EXIT_HEALTH_FAILED 0x08
#define SMARTCTL_EXIT_PREFAIL_ATTRIBUTE 0x10
#define SMARTCTL_SAFE_INTEGER_MAX 9007199254740991.0

static bool default_wake_device_tier(void) {
    return system_health_request_tier(SYSTEM_HEALTH_TIER_DEVICE);
}

static bool simple_device_name(const char *name) {
    if (!name || !name[0] || strcmp(name, ".") == 0 ||
        strcmp(name, "..") == 0 ||
        strlen(name) >= LINUX_HARDWARE_INTERNAL_NAME_LENGTH) return false;
    for (const unsigned char *cursor = (const unsigned char *)name;
         *cursor; ++cursor)
        if (!isalnum(*cursor) && *cursor != '_' && *cursor != '-' &&
            *cursor != '.') return false;
    return true;
}

static void physical_name(const char *name,
                          char output[LINUX_HARDWARE_INTERNAL_NAME_LENGTH]) {
    size_t length = strlen(name);
    memcpy(output, name, length + 1U);
    if (strncmp(output, "nvme", 4U) == 0 &&
        isdigit((unsigned char)output[4])) {
        char *cursor = output + 4U;
        while (isdigit((unsigned char)*cursor)) cursor++;
        if (*cursor == 'n' && isdigit((unsigned char)cursor[1]))
            *cursor = '\0';
        return;
    }
    if (strncmp(output, "mmcblk", 6U) == 0 &&
        isdigit((unsigned char)output[6])) {
        char *cursor = output + 6U;
        while (isdigit((unsigned char)*cursor)) cursor++;
        if (*cursor == 'p' && isdigit((unsigned char)cursor[1]))
            *cursor = '\0';
        return;
    }
    if (output[0] == 's' && output[1] == 'd' &&
        isalpha((unsigned char)output[2])) {
        char *cursor = output + 2U;
        while (isalpha((unsigned char)*cursor)) cursor++;
        if (isdigit((unsigned char)*cursor)) *cursor = '\0';
    }
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

void smartctl_provider_state_init(smartctl_provider_state_t *state,
                                  const char *installation_scope) {
    if (!state) return;
    memset(state, 0, sizeof(*state));
    if (installation_scope)
        snprintf(state->installation_scope, sizeof(state->installation_scope),
                 "%s", installation_scope);
    snprintf(state->program, sizeof(state->program), "%s",
             "/usr/sbin/smartctl");
    state->timeout_ms = SMARTCTL_PROVIDER_TIMEOUT_MS;
    state->terminate_grace_ms = SMARTCTL_PROVIDER_TERMINATE_GRACE_MS;
    state->run_helper = health_helper_run;
    state->wake_device_tier = default_wake_device_tier;
    state->capability = SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
    atomic_init(&state->refresh_pending, false);
    atomic_init(&state->refresh_requests, 0U);
    atomic_init(&state->refresh_coalesced, 0U);
    atomic_init(&state->refresh_collections, 0U);
}

static bool add_device(smartctl_provider_state_t *state,
                       const char *device_path, bool target_managed) {
    if (!state || !state->installation_scope[0] || !device_path ||
        strncmp(device_path, "/dev/", 5U) != 0 ||
        strlen(device_path) >= SMARTCTL_PROVIDER_PATH_LENGTH) return false;
    const char *name = device_path + 5U;
    if (!simple_device_name(name) || strchr(name, '/')) return false;
    char physical[LINUX_HARDWARE_INTERNAL_NAME_LENGTH];
    physical_name(name, physical);
    for (size_t index = 0U; index < state->device_count; ++index)
        if (strcmp(state->devices[index].sysfs_name, physical) == 0)
            return true;
    if (state->device_count >= SMARTCTL_PROVIDER_MAX_DEVICES) {
        state->devices_dropped++;
        return false;
    }
    smartctl_provider_device_t *device =
        &state->devices[state->device_count++];
    snprintf(device->device_path, sizeof(device->device_path), "/dev/%s",
             physical);
    snprintf(device->sysfs_name, sizeof(device->sysfs_name), "%s", physical);
    device->target_managed = target_managed;
    return true;
}

bool smartctl_provider_add_device(smartctl_provider_state_t *state,
                                  const char *device_path) {
    return add_device(state, device_path, false);
}

static bool number_u64(const cJSON *item, uint64_t *output) {
    if (!cJSON_IsNumber(item) || !isfinite(item->valuedouble) ||
        item->valuedouble < 0.0 ||
        item->valuedouble > SMARTCTL_SAFE_INTEGER_MAX ||
        floor(item->valuedouble) != item->valuedouble) return false;
    *output = (uint64_t)item->valuedouble;
    return true;
}

static bool number_int(const cJSON *item, int minimum, int maximum,
                       int *output) {
    uint64_t value;
    if (!number_u64(item, &value) || value > (uint64_t)maximum ||
        value < (uint64_t)minimum) return false;
    *output = (int)value;
    return true;
}

static bool object_number_u64(const cJSON *object, const char *name,
                              smartctl_u64_value_t *output) {
    const cJSON *item = cJSON_GetObjectItemCaseSensitive(object, name);
    if (!item) return true;
    uint64_t value;
    if (!number_u64(item, &value)) return false;
    output->valid = true;
    output->value = value;
    return true;
}

static bool object_percent(const cJSON *root, const char *object_name,
                           const char *value_name,
                           smartctl_double_value_t *output) {
    const cJSON *object = cJSON_GetObjectItemCaseSensitive(root, object_name);
    if (!object) return true;
    if (!cJSON_IsObject(object)) return false;
    const cJSON *item = cJSON_GetObjectItemCaseSensitive(object, value_name);
    if (!item) return true;
    int percent;
    if (!number_int(item, 0, 100, &percent)) return false;
    output->valid = true;
    output->value = (double)percent / 100.0;
    return true;
}

static bool node_count_bounded(const cJSON *node, unsigned int depth,
                               size_t *count) {
    if (!node || depth > 32U || ++*count > SMARTCTL_PROVIDER_JSON_NODES_MAX)
        return false;
    for (const cJSON *child = node->child; child; child = child->next)
        if (!node_count_bounded(child, depth + 1U, count)) return false;
    return true;
}

static bool contains_case_insensitive(const char *text, size_t length,
                                      const char *needle) {
    size_t needle_length = strlen(needle);
    if (needle_length == 0U || needle_length > length) return false;
    for (size_t start = 0U; start + needle_length <= length; ++start) {
        size_t index = 0U;
        while (index < needle_length &&
               tolower((unsigned char)text[start + index]) ==
               tolower((unsigned char)needle[index])) index++;
        if (index == needle_length) return true;
    }
    return false;
}

static bool json_string_contains(const cJSON *item, const char *needle) {
    return cJSON_IsString(item) && item->valuestring &&
        contains_case_insensitive(item->valuestring, strlen(item->valuestring),
                                  needle);
}

static smartctl_parse_status_t classify_device_or_power(const cJSON *root) {
    const cJSON *power = cJSON_GetObjectItemCaseSensitive(root, "power_mode");
    if (cJSON_IsObject(power)) {
        const cJSON *name = cJSON_GetObjectItemCaseSensitive(power, "name");
        if (json_string_contains(name, "standby") ||
            json_string_contains(name, "sleep") ||
            json_string_contains(name, "low power") ||
            json_string_contains(name, "idle"))
            return SMARTCTL_PARSE_SLEEPING;
    }

    const cJSON *smartctl = cJSON_GetObjectItemCaseSensitive(root, "smartctl");
    const cJSON *messages = cJSON_IsObject(smartctl)
        ? cJSON_GetObjectItemCaseSensitive(smartctl, "messages") : NULL;
    if (cJSON_IsArray(messages) && cJSON_GetArraySize(messages) <= 16) {
        const cJSON *message;
        cJSON_ArrayForEach(message, messages) {
            const cJSON *value = cJSON_IsObject(message)
                ? cJSON_GetObjectItemCaseSensitive(message, "string") : NULL;
            if (json_string_contains(value, "permission denied") ||
                json_string_contains(value, "operation not permitted"))
                return SMARTCTL_PARSE_PERMISSION_DENIED;
            if (json_string_contains(value, "standby") ||
                json_string_contains(value, "sleeping") ||
                json_string_contains(value, "low power") ||
                json_string_contains(value, "idle"))
                return SMARTCTL_PARSE_SLEEPING;
        }
    }
    return SMARTCTL_PARSE_MALFORMED;
}

static bool parse_version(const cJSON *root, bool *supported) {
    const cJSON *version = cJSON_GetObjectItemCaseSensitive(
        root, "json_format_version");
    if (!cJSON_IsArray(version) || cJSON_GetArraySize(version) != 2)
        return false;
    int major, minor;
    if (!number_int(cJSON_GetArrayItem(version, 0), 0, 255, &major) ||
        !number_int(cJSON_GetArrayItem(version, 1), 0, 255, &minor))
        return false;
    (void)minor;
    *supported = major == 1;
    return true;
}

static bool parse_exit_status(const cJSON *root, int process_exit_status,
                              int *exit_status) {
    const cJSON *smartctl = cJSON_GetObjectItemCaseSensitive(root, "smartctl");
    if (!cJSON_IsObject(smartctl)) return false;
    int value;
    if (!number_int(cJSON_GetObjectItemCaseSensitive(
                        smartctl, "exit_status"), 0, 255, &value) ||
        value != process_exit_status) return false;
    *exit_status = value;
    return true;
}

static bool set_known_attribute(smartctl_normalized_sample_t *sample,
                                int id, uint64_t value, bool seen[4]) {
    smartctl_u64_value_t *destination = NULL;
    size_t slot = 0U;
    switch (id) {
        case 5:
            destination = &sample->reallocated_sectors;
            slot = 0U;
            break;
        case 197:
            destination = &sample->pending_sectors;
            slot = 1U;
            break;
        case 198:
            destination = &sample->uncorrectable_errors;
            slot = 2U;
            break;
        case 199:
            destination = &sample->interface_crc_errors;
            slot = 3U;
            break;
        default: return true;
    }
    if (seen[slot]) return false;
    seen[slot] = true;
    destination->valid = true;
    destination->value = value;
    return true;
}

static bool parse_ata_attributes(const cJSON *root,
                                 smartctl_normalized_sample_t *sample) {
    const cJSON *attributes = cJSON_GetObjectItemCaseSensitive(
        root, "ata_smart_attributes");
    if (!attributes) return true;
    if (!cJSON_IsObject(attributes)) return false;
    const cJSON *table = cJSON_GetObjectItemCaseSensitive(attributes, "table");
    if (!table) return true;
    if (!cJSON_IsArray(table) || cJSON_GetArraySize(table) < 0 ||
        (size_t)cJSON_GetArraySize(table) > SMARTCTL_PROVIDER_ATTRIBUTES_MAX)
        return false;
    bool seen[4] = {false};
    const cJSON *attribute;
    cJSON_ArrayForEach(attribute, table) {
        if (!cJSON_IsObject(attribute)) return false;
        int id;
        if (!number_int(cJSON_GetObjectItemCaseSensitive(attribute, "id"),
                        1, 255, &id)) return false;
        if (id != 5 && id != 197 && id != 198 && id != 199) continue;
        const cJSON *raw = cJSON_GetObjectItemCaseSensitive(attribute, "raw");
        uint64_t value;
        if (!cJSON_IsObject(raw) ||
            !number_u64(cJSON_GetObjectItemCaseSensitive(raw, "value"),
                        &value) ||
            !set_known_attribute(sample, id, value, seen)) return false;
    }
    return true;
}

static bool add_u64(uint64_t *total, uint64_t addition) {
    if (UINT64_MAX - *total < addition) return false;
    *total += addition;
    return true;
}

static bool parse_scsi(const cJSON *root,
                       smartctl_normalized_sample_t *sample) {
    if (!object_number_u64(root, "scsi_grown_defect_list",
                           &sample->reallocated_sectors)) return false;
    const cJSON *log = cJSON_GetObjectItemCaseSensitive(
        root, "scsi_error_counter_log");
    if (!log) return true;
    if (!cJSON_IsObject(log)) return false;
    static const char *const sections[] = {"read", "write", "verify"};
    uint64_t total = 0U;
    bool found = false;
    for (size_t index = 0U; index < sizeof(sections) / sizeof(sections[0]);
         ++index) {
        const cJSON *section = cJSON_GetObjectItemCaseSensitive(
            log, sections[index]);
        if (!section) continue;
        if (!cJSON_IsObject(section)) return false;
        const cJSON *item = cJSON_GetObjectItemCaseSensitive(
            section, "total_uncorrected_errors");
        if (!item) continue;
        uint64_t value;
        if (!number_u64(item, &value) || !add_u64(&total, value)) return false;
        found = true;
    }
    if (found) {
        sample->uncorrectable_errors.valid = true;
        sample->uncorrectable_errors.value = total;
    }
    return true;
}

static bool parse_nvme(const cJSON *root,
                       smartctl_normalized_sample_t *sample) {
    const cJSON *log = cJSON_GetObjectItemCaseSensitive(
        root, "nvme_smart_health_information_log");
    if (!log) return true;
    if (!cJSON_IsObject(log)) return false;
    int critical_warning = 0;
    const cJSON *critical = cJSON_GetObjectItemCaseSensitive(
        log, "critical_warning");
    if (critical && !number_int(critical, 0, 255, &critical_warning))
        return false;
    if (critical_warning != 0) sample->critical = true;
    int percent;
    const cJSON *spare = cJSON_GetObjectItemCaseSensitive(
        log, "available_spare");
    if (spare) {
        if (!number_int(spare, 0, 100, &percent)) return false;
        sample->available_spare_ratio.valid = true;
        sample->available_spare_ratio.value = (double)percent / 100.0;
    }
    const cJSON *used = cJSON_GetObjectItemCaseSensitive(
        log, "percentage_used");
    if (used) {
        if (!number_int(used, 0, 255, &percent)) return false;
        sample->percentage_used_ratio.valid = true;
        sample->percentage_used_ratio.value =
            percent > 100 ? 1.0 : (double)percent / 100.0;
    }
    if (!object_number_u64(log, "media_errors", &sample->media_errors) ||
        !object_number_u64(log, "unsafe_shutdowns",
                           &sample->unsafe_shutdowns)) return false;
    return true;
}

static bool any_value(const smartctl_normalized_sample_t *sample) {
    return sample->health_valid || sample->temperature_celsius.valid ||
        sample->available_spare_ratio.valid ||
        sample->percentage_used_ratio.valid ||
        sample->reallocated_sectors.valid || sample->pending_sectors.valid ||
        sample->uncorrectable_errors.valid ||
        sample->interface_crc_errors.valid || sample->media_errors.valid ||
        sample->unsafe_shutdowns.valid;
}

int smartctl_provider_parse_json(const char *json, size_t length,
                                 int process_exit_status,
                                 smartctl_normalized_sample_t *sample) {
    if (!json || !sample || length == 0U ||
        length > HEALTH_HELPER_OUTPUT_MAX || process_exit_status < 0 ||
        process_exit_status > 255) return -1;
    memset(sample, 0, sizeof(*sample));
    sample->status = SMARTCTL_PARSE_MALFORMED;
    sample->capability = SYSTEM_HEALTH_CAPABILITY_ERROR;
    const char *parse_end = NULL;
    cJSON *root = cJSON_ParseWithLengthOpts(json, length, &parse_end, 0);
    if (!root || !cJSON_IsObject(root)) {
        cJSON_Delete(root);
        return -1;
    }
    while (parse_end < json + length &&
           isspace((unsigned char)*parse_end)) parse_end++;
    size_t nodes = 0U;
    if (parse_end != json + length ||
        !node_count_bounded(root, 0U, &nodes)) {
        cJSON_Delete(root);
        return -1;
    }
    bool supported = false;
    if (!parse_version(root, &supported)) {
        cJSON_Delete(root);
        return -1;
    }
    if (!supported) {
        sample->status = SMARTCTL_PARSE_UNSUPPORTED_VERSION;
        sample->capability = SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
        cJSON_Delete(root);
        return 0;
    }
    if (!parse_exit_status(root, process_exit_status,
                           &sample->exit_status)) {
        cJSON_Delete(root);
        return -1;
    }
    if ((sample->exit_status & SMARTCTL_EXIT_DEVICE_OR_POWER) != 0) {
        sample->status = classify_device_or_power(root);
        if (sample->status == SMARTCTL_PARSE_PERMISSION_DENIED) {
            sample->capability = SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED;
        } else if (sample->status == SMARTCTL_PARSE_SLEEPING) {
            sample->capability = SYSTEM_HEALTH_CAPABILITY_STALE;
        } else {
            sample->status = SMARTCTL_PARSE_MALFORMED;
            sample->capability = SYSTEM_HEALTH_CAPABILITY_ERROR;
        }
        cJSON_Delete(root);
        return 0;
    }
    if ((sample->exit_status &
         (SMARTCTL_EXIT_COMMAND_ERROR | SMARTCTL_EXIT_SMART_COMMAND)) != 0) {
        cJSON_Delete(root);
        return 0;
    }

    const cJSON *status = cJSON_GetObjectItemCaseSensitive(root,
                                                           "smart_status");
    if (status) {
        if (!cJSON_IsObject(status)) {
            cJSON_Delete(root);
            return -1;
        }
        const cJSON *passed = cJSON_GetObjectItemCaseSensitive(status,
                                                               "passed");
        if (!cJSON_IsBool(passed)) {
            cJSON_Delete(root);
            return -1;
        }
        sample->health_valid = true;
        sample->health_passed = cJSON_IsTrue(passed);
    }
    const cJSON *temperature = cJSON_GetObjectItemCaseSensitive(
        root, "temperature");
    if (temperature) {
        const cJSON *current = cJSON_IsObject(temperature)
            ? cJSON_GetObjectItemCaseSensitive(temperature, "current") : NULL;
        if (!cJSON_IsNumber(current) || !isfinite(current->valuedouble) ||
            current->valuedouble < -100.0 ||
            current->valuedouble > 250.0) {
            cJSON_Delete(root);
            return -1;
        }
        sample->temperature_celsius.valid = true;
        sample->temperature_celsius.value = current->valuedouble;
    }
    if (!object_percent(root, "spare_available", "current_percent",
                        &sample->available_spare_ratio) ||
        !object_percent(root, "endurance_used", "current_percent",
                        &sample->percentage_used_ratio) ||
        !parse_ata_attributes(root, sample) || !parse_scsi(root, sample) ||
        !parse_nvme(root, sample)) {
        cJSON_Delete(root);
        return -1;
    }
    if (!sample->media_errors.valid && sample->uncorrectable_errors.valid)
        sample->media_errors = sample->uncorrectable_errors;
    if ((sample->exit_status & SMARTCTL_EXIT_HEALTH_FAILED) != 0 ||
        (sample->health_valid && !sample->health_passed))
        sample->critical = true;
    sample->prefail = sample->critical ||
        (sample->exit_status & SMARTCTL_EXIT_PREFAIL_ATTRIBUTE) != 0 ||
        (sample->pending_sectors.valid && sample->pending_sectors.value > 0U) ||
        (sample->reallocated_sectors.valid &&
         sample->reallocated_sectors.value > 0U);
    if (!sample->health_valid &&
        (sample->exit_status & (SMARTCTL_EXIT_HEALTH_FAILED |
                                SMARTCTL_EXIT_PREFAIL_ATTRIBUTE)) != 0)
        sample->health_valid = true;
    sample->status = SMARTCTL_PARSE_OK;
    sample->capability = any_value(sample)
        ? SYSTEM_HEALTH_CAPABILITY_AVAILABLE
        : SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
    cJSON_Delete(root);
    return 0;
}

bool smartctl_provider_request_refresh(smartctl_provider_state_t *state) {
    if (!state || !state->wake_device_tier) return false;
    atomic_fetch_add_explicit(&state->refresh_requests, 1U,
                              memory_order_relaxed);
    bool expected = false;
    if (!atomic_compare_exchange_strong_explicit(
            &state->refresh_pending, &expected, true, memory_order_acq_rel,
            memory_order_relaxed)) {
        atomic_fetch_add_explicit(&state->refresh_coalesced, 1U,
                                  memory_order_relaxed);
        return true;
    }
    if (!state->wake_device_tier()) {
        atomic_store_explicit(&state->refresh_pending, false,
                              memory_order_release);
        return false;
    }
    return true;
}

static const smartctl_provider_device_t *find_device(
    const smartctl_provider_device_t *devices, size_t count,
    const char *sysfs_name) {
    for (size_t index = 0U; index < count; ++index)
        if (strcmp(devices[index].sysfs_name, sysfs_name) == 0)
            return &devices[index];
    return NULL;
}

static void restore_baselines(smartctl_provider_device_t *destination,
                              const smartctl_provider_device_t *source) {
    if (!destination || !source) return;
    destination->previous_uncorrectable = source->previous_uncorrectable;
    destination->previous_crc = source->previous_crc;
    destination->previous_media = source->previous_media;
    destination->previous_unsafe_shutdowns =
        source->previous_unsafe_shutdowns;
    destination->previous_monotonic_ms = source->previous_monotonic_ms;
    destination->previous_uncorrectable_valid =
        source->previous_uncorrectable_valid;
    destination->previous_crc_valid = source->previous_crc_valid;
    destination->previous_media_valid = source->previous_media_valid;
    destination->previous_unsafe_valid = source->previous_unsafe_valid;
}

int smartctl_provider_discover(void *opaque,
                               const system_health_collect_context_t *context,
                               system_health_provider_inventory_t *inventory) {
    smartctl_provider_state_t *state = opaque;
    if (!state || !context || !context->sys_root || !inventory) return -1;
    memset(inventory, 0, sizeof(*inventory));
    linux_device_map_t map;
    memset(&map, 0, sizeof(map));
    if (!state->installation_scope[0] || state->program[0] != '/' ||
        state->timeout_ms == 0U || !state->run_helper) {
        state->capability = SYSTEM_HEALTH_CAPABILITY_ERROR;
    } else {
        (void)linux_device_map_build(context->sys_root,
                                     state->installation_scope, &map);
        if (state->discover_target_devices) {
            smartctl_provider_device_t previous[SMARTCTL_PROVIDER_MAX_DEVICES];
            size_t previous_count = state->device_count;
            memcpy(previous, state->devices, sizeof(previous));
            size_t kept = 0U;
            for (size_t index = 0U; index < previous_count; ++index)
                if (!previous[index].target_managed)
                    state->devices[kept++] = previous[index];
            memset(state->devices + kept, 0,
                   sizeof(state->devices[0]) *
                       (SMARTCTL_PROVIDER_MAX_DEVICES - kept));
            state->device_count = kept;
            state->devices_dropped = 0U;
            if (map.capability == SYSTEM_HEALTH_CAPABILITY_AVAILABLE) {
                for (size_t index = 0U; index < map.count; ++index) {
                    const linux_device_map_entry_t *mapped =
                        &map.entries[index];
                    if (!mapped->target_mapped) continue;
                    char path[SMARTCTL_PROVIDER_PATH_LENGTH];
                    int written = snprintf(path, sizeof(path), "/dev/%s",
                                           mapped->sysfs_name);
                    size_t before = state->device_count;
                    if (written < 0 || (size_t)written >= sizeof(path) ||
                        !add_device(state, path, true)) continue;
                    if (state->device_count == before) continue;
                    smartctl_provider_device_t *added =
                        &state->devices[state->device_count - 1U];
                    restore_baselines(added, find_device(
                        previous, previous_count, added->sysfs_name));
                }
            }
        }
        for (size_t index = 0U; index < state->device_count; ++index) {
            smartctl_provider_device_t *device = &state->devices[index];
            const linux_device_map_entry_t *mapped =
                map.capability == SYSTEM_HEALTH_CAPABILITY_AVAILABLE
                    ? linux_device_map_find(&map, device->sysfs_name) : NULL;
            if (mapped) {
                snprintf(device->public_id, sizeof(device->public_id), "%s",
                         mapped->public_id);
            } else {
                snprintf(device->public_id, sizeof(device->public_id),
                         "device.%016llx", (unsigned long long)scoped_hash(
                             state->installation_scope, device->sysfs_name));
            }
        }
        if (state->device_count > 0U)
            state->capability = SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
        else
            state->capability = state->discover_target_devices
                ? (map.capability == SYSTEM_HEALTH_CAPABILITY_AVAILABLE
                       ? SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED
                       : map.capability)
                : SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
    }
    for (size_t index = 0U; index < state->device_count; ++index) {
        snprintf(inventory->resources[index].id,
                 sizeof(inventory->resources[index].id), "%s",
                 state->devices[index].public_id);
        inventory->resources[index].scope = SYSTEM_HEALTH_SCOPE_DEVICE;
        inventory->resources[index].capability = state->capability;
    }
    inventory->count = state->device_count;
    inventory->dropped = state->devices_dropped;
    if (inventory->count == 0U) {
        snprintf(inventory->resources[0].id,
                 sizeof(inventory->resources[0].id), "%s", "smartctl");
        inventory->resources[0].scope = SYSTEM_HEALTH_SCOPE_DEVICE;
        inventory->resources[0].capability = state->capability;
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

static void reset_baselines(smartctl_provider_device_t *device) {
    device->previous_uncorrectable_valid = false;
    device->previous_crc_valid = false;
    device->previous_media_valid = false;
    device->previous_unsafe_valid = false;
}

static void emit_delta(const system_health_collect_context_t *context,
                       system_health_observation_sink_t *sink,
                       const char *metric, const char *resource,
                       smartctl_u64_value_t current, uint64_t *previous,
                       bool *previous_valid, uint64_t previous_monotonic_ms,
                       system_health_capability_t unavailable) {
    if (!current.valid) {
        *previous_valid = false;
        emit(sink, context, metric, resource, unavailable, false, 0.0,
             SYSTEM_HEALTH_UNIT_COUNT);
        return;
    }
    bool valid = *previous_valid &&
        context->monotonic_ms > previous_monotonic_ms &&
        current.value >= *previous;
    system_health_capability_t capability = !*previous_valid
        ? SYSTEM_HEALTH_CAPABILITY_STALE : SYSTEM_HEALTH_CAPABILITY_ERROR;
    double delta = valid ? (double)(current.value - *previous) : 0.0;
    *previous = current.value;
    *previous_valid = true;
    emit(sink, context, metric, resource, capability, valid, delta,
         SYSTEM_HEALTH_UNIT_COUNT);
}

static void emit_unavailable_device(
    const smartctl_provider_device_t *device,
    const system_health_collect_context_t *context,
    system_health_observation_sink_t *sink,
    system_health_capability_t capability) {
    static const char *const metrics[] = {
        "storage.device.prefail", "storage.device.critical",
        "storage.device.temperature_celsius",
        "storage.device.available_spare_ratio",
        "storage.device.percentage_used_ratio",
        "storage.device.reallocated_sectors",
        "storage.device.pending_sectors",
        "storage.device.uncorrectable_errors_delta",
        "storage.device.interface_crc_errors_delta",
        "storage.device.media_errors_delta",
        "storage.device.unsafe_shutdowns_delta"
    };
    for (size_t index = 0U; index < sizeof(metrics) / sizeof(metrics[0]);
         ++index)
        emit(sink, context, metrics[index], device->public_id, capability,
             false, 0.0, SYSTEM_HEALTH_UNIT_NONE);
}

static system_health_capability_t helper_failure_capability(
    const health_helper_result_t *result) {
    if (!result) return SYSTEM_HEALTH_CAPABILITY_ERROR;
    switch (result->outcome) {
        case HEALTH_HELPER_EXEC_ERROR:
            return SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
        case HEALTH_HELPER_BUSY:
            return SYSTEM_HEALTH_CAPABILITY_STALE;
        case HEALTH_HELPER_OK:
        case HEALTH_HELPER_EXITED:
            return SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
        case HEALTH_HELPER_TIMED_OUT:
        case HEALTH_HELPER_SYSTEM_ERROR:
            return SYSTEM_HEALTH_CAPABILITY_ERROR;
    }
    return SYSTEM_HEALTH_CAPABILITY_ERROR;
}

static system_health_capability_t collect_device(
    smartctl_provider_state_t *state, smartctl_provider_device_t *device,
    const system_health_collect_context_t *context,
    system_health_observation_sink_t *sink) {
    char *arguments[] = {
        state->program, "-H", "-A", "-j=c", "-n", "standby,2", "--",
        device->device_path, NULL
    };
    health_helper_request_t request = {
        .program = state->program,
        .argv = arguments,
        .timeout_ms = state->timeout_ms,
        .terminate_grace_ms = state->terminate_grace_ms,
        .output_limit = HEALTH_HELPER_OUTPUT_MAX
    };
    health_helper_result_t result;
    memset(&result, 0, sizeof(result));
    int run_status = state->run_helper(&request, &result);
    if (run_status != 0 ||
        (result.outcome != HEALTH_HELPER_OK &&
         result.outcome != HEALTH_HELPER_EXITED) || result.output_truncated) {
        system_health_capability_t capability = run_status != 0
            ? SYSTEM_HEALTH_CAPABILITY_ERROR
            : result.output_truncated
            ? SYSTEM_HEALTH_CAPABILITY_ERROR
            : helper_failure_capability(&result);
        emit_unavailable_device(device, context, sink, capability);
        if (capability != SYSTEM_HEALTH_CAPABILITY_STALE)
            reset_baselines(device);
        return capability;
    }
    smartctl_normalized_sample_t sample;
    if (smartctl_provider_parse_json(result.output, result.output_length,
                                     result.exit_code, &sample) != 0) {
        emit_unavailable_device(device, context, sink,
                                SYSTEM_HEALTH_CAPABILITY_ERROR);
        reset_baselines(device);
        return SYSTEM_HEALTH_CAPABILITY_ERROR;
    }
    if (sample.capability != SYSTEM_HEALTH_CAPABILITY_AVAILABLE) {
        emit_unavailable_device(device, context, sink, sample.capability);
        if (sample.capability != SYSTEM_HEALTH_CAPABILITY_STALE)
            reset_baselines(device);
        return sample.capability;
    }

    emit(sink, context, "storage.device.prefail", device->public_id,
         sample.health_valid ? SYSTEM_HEALTH_CAPABILITY_AVAILABLE
                             : SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
         sample.health_valid,
         sample.prefail ? 1.0 : 0.0, SYSTEM_HEALTH_UNIT_BOOLEAN);
    emit(sink, context, "storage.device.critical", device->public_id,
         sample.health_valid ? SYSTEM_HEALTH_CAPABILITY_AVAILABLE
                             : SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
         sample.health_valid,
         sample.critical ? 1.0 : 0.0, SYSTEM_HEALTH_UNIT_BOOLEAN);
    emit(sink, context, "storage.device.temperature_celsius",
         device->public_id, SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
         sample.temperature_celsius.valid, sample.temperature_celsius.value,
         SYSTEM_HEALTH_UNIT_CELSIUS);
    emit(sink, context, "storage.device.available_spare_ratio",
         device->public_id, SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
         sample.available_spare_ratio.valid,
         sample.available_spare_ratio.value, SYSTEM_HEALTH_UNIT_RATIO);
    emit(sink, context, "storage.device.percentage_used_ratio",
         device->public_id, SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
         sample.percentage_used_ratio.valid,
         sample.percentage_used_ratio.value, SYSTEM_HEALTH_UNIT_RATIO);
    emit(sink, context, "storage.device.reallocated_sectors",
         device->public_id, SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
         sample.reallocated_sectors.valid,
         (double)sample.reallocated_sectors.value, SYSTEM_HEALTH_UNIT_COUNT);
    emit(sink, context, "storage.device.pending_sectors", device->public_id,
         SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED, sample.pending_sectors.valid,
         (double)sample.pending_sectors.value, SYSTEM_HEALTH_UNIT_COUNT);
    emit_delta(context, sink, "storage.device.uncorrectable_errors_delta",
               device->public_id, sample.uncorrectable_errors,
               &device->previous_uncorrectable,
               &device->previous_uncorrectable_valid,
               device->previous_monotonic_ms,
               SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED);
    emit_delta(context, sink, "storage.device.interface_crc_errors_delta",
               device->public_id, sample.interface_crc_errors,
               &device->previous_crc, &device->previous_crc_valid,
               device->previous_monotonic_ms,
               SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED);
    emit_delta(context, sink, "storage.device.media_errors_delta",
               device->public_id, sample.media_errors,
               &device->previous_media, &device->previous_media_valid,
               device->previous_monotonic_ms,
               SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED);
    emit_delta(context, sink, "storage.device.unsafe_shutdowns_delta",
               device->public_id, sample.unsafe_shutdowns,
               &device->previous_unsafe_shutdowns,
               &device->previous_unsafe_valid, device->previous_monotonic_ms,
               SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED);
    device->previous_monotonic_ms = context->monotonic_ms;
    return SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
}

static system_health_capability_t merge_provider_capability(
    system_health_capability_t current, system_health_capability_t next) {
    if (current == SYSTEM_HEALTH_CAPABILITY_AVAILABLE ||
        next == SYSTEM_HEALTH_CAPABILITY_AVAILABLE)
        return SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    if (current == SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED ||
        next == SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED)
        return SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED;
    if (current == SYSTEM_HEALTH_CAPABILITY_ERROR ||
        next == SYSTEM_HEALTH_CAPABILITY_ERROR)
        return SYSTEM_HEALTH_CAPABILITY_ERROR;
    if (current == SYSTEM_HEALTH_CAPABILITY_STALE ||
        next == SYSTEM_HEALTH_CAPABILITY_STALE)
        return SYSTEM_HEALTH_CAPABILITY_STALE;
    return SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
}

int smartctl_provider_collect(void *opaque,
                              const system_health_collect_context_t *context,
                              system_health_observation_sink_t *sink) {
    smartctl_provider_state_t *state = opaque;
    if (!state || !context || !sink || !state->run_helper) return -1;
    bool refresh = atomic_exchange_explicit(&state->refresh_pending, false,
                                            memory_order_acq_rel);
    if (refresh)
        atomic_fetch_add_explicit(&state->refresh_collections, 1U,
                                  memory_order_relaxed);
    system_health_capability_t capability = state->device_count == 0U
        ? state->capability : SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
    for (size_t index = 0U; index < state->device_count; ++index)
        capability = merge_provider_capability(
            capability, collect_device(state, &state->devices[index], context,
                                       sink));
    state->capability = capability;
    emit(sink, context, "hardware.provider.visible", "smartctl", capability,
         capability == SYSTEM_HEALTH_CAPABILITY_AVAILABLE, 1.0,
         SYSTEM_HEALTH_UNIT_BOOLEAN);
    sink->dropped += state->devices_dropped;
    return 0;
}

void smartctl_provider_init(system_health_provider_t *provider,
                            smartctl_provider_state_t *state) {
    if (!provider) return;
    memset(provider, 0, sizeof(*provider));
    snprintf(provider->name, sizeof(provider->name), "%s", "smartctl");
    provider->capability = SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
    provider->state = state;
    provider->discover = smartctl_provider_discover;
    provider->collect = smartctl_provider_collect;
}
