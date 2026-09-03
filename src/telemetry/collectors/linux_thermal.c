#define _XOPEN_SOURCE 700

#include "telemetry/collectors/linux_thermal.h"

#include <ctype.h>
#include <dirent.h>
#include <errno.h>
#include <limits.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>

#ifndef PATH_MAX
#define PATH_MAX 4096
#endif

#define THERMAL_CANDIDATE_CAPACITY 128U
#define THERMAL_ENTRY_LENGTH 64U
#define THERMAL_TEXT_LENGTH 128U
#define THERMAL_TRIP_LIMIT 32U

typedef enum {
    THERMAL_SOURCE_ZONE = 0,
    THERMAL_SOURCE_HWMON
} thermal_source_t;

typedef struct {
    thermal_source_t source;
    char entry[THERMAL_ENTRY_LENGTH];
    unsigned int channel;
    char id[SYSTEM_HEALTH_ID_LENGTH];
} thermal_candidate_t;

typedef struct {
    bool valid;
    system_health_capability_t capability;
    double value;
} thermal_numeric_t;

static bool format_path(char *output, size_t output_size,
                        const char *format, const char *root,
                        const char *entry, unsigned int index) {
    if (!output || output_size == 0 || !format || !root) return false;
    int written = snprintf(output, output_size, format, root,
                           entry ? entry : "", index);
    return written >= 0 && (size_t)written < output_size;
}

system_health_capability_t linux_thermal_capability_from_errno(
    int error_number) {
    if (error_number == EACCES || error_number == EPERM) {
        return SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED;
    }
    if (error_number == ENOENT || error_number == ENOTDIR) {
        return SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
    }
    return SYSTEM_HEALTH_CAPABILITY_ERROR;
}

static bool read_text(const char *path, char output[THERMAL_TEXT_LENGTH],
                      system_health_capability_t *capability) {
    if (!path || !output || !capability) return false;
    output[0] = '\0';
    FILE *file = fopen(path, "r");
    if (!file) {
        *capability = linux_thermal_capability_from_errno(errno);
        return false;
    }

    size_t length = fread(output, 1, THERMAL_TEXT_LENGTH - 1U, file);
    if (ferror(file)) {
        int saved_errno = errno;
        fclose(file);
        *capability = linux_thermal_capability_from_errno(saved_errno);
        output[0] = '\0';
        return false;
    }
    if (length == THERMAL_TEXT_LENGTH - 1U && fgetc(file) != EOF) {
        fclose(file);
        *capability = SYSTEM_HEALTH_CAPABILITY_ERROR;
        output[0] = '\0';
        return false;
    }
    fclose(file);
    output[length] = '\0';

    while (length > 0 && isspace((unsigned char)output[length - 1U])) {
        output[--length] = '\0';
    }
    size_t start = 0;
    while (output[start] && isspace((unsigned char)output[start])) start++;
    if (start > 0) memmove(output, output + start, strlen(output + start) + 1U);
    if (output[0] == '\0') {
        *capability = SYSTEM_HEALTH_CAPABILITY_ERROR;
        return false;
    }
    *capability = SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    return true;
}

static thermal_numeric_t read_millidegrees(const char *path) {
    thermal_numeric_t result = {
        .valid = false,
        .capability = SYSTEM_HEALTH_CAPABILITY_ERROR,
        .value = 0.0
    };
    char text[THERMAL_TEXT_LENGTH];
    if (!read_text(path, text, &result.capability)) return result;

    errno = 0;
    char *end = NULL;
    long long raw = strtoll(text, &end, 10);
    if (errno == ERANGE || end == text || !end || *end != '\0') {
        result.capability = SYSTEM_HEALTH_CAPABILITY_ERROR;
        return result;
    }
    /* Reject values outside physically meaningful absolute temperatures. */
    if (raw < -273150LL || raw > 2000000LL) {
        result.capability = SYSTEM_HEALTH_CAPABILITY_ERROR;
        return result;
    }
    result.valid = true;
    result.capability = SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    result.value = (double)raw / 1000.0;
    return result;
}

static void normalize_component(const char *input, char *output,
                                size_t output_size) {
    if (!output || output_size == 0) return;
    size_t used = 0;
    bool separator = false;
    if (input) {
        for (const unsigned char *cursor = (const unsigned char *)input;
             *cursor && used + 1U < output_size; ++cursor) {
            unsigned char character = *cursor;
            if (isalnum(character)) {
                output[used++] = (char)tolower(character);
                separator = false;
            } else if (!separator && used > 0) {
                output[used++] = '_';
                separator = true;
            }
        }
    }
    while (used > 0 && output[used - 1U] == '_') used--;
    if (used == 0 && output_size > 1U) {
        memcpy(output, "unknown", 7U);
        used = 7U;
    }
    output[used] = '\0';
}

static uint32_t fnv1a_update(uint32_t hash, const char *text) {
    if (!text) return hash;
    for (const unsigned char *cursor = (const unsigned char *)text;
         *cursor; ++cursor) {
        hash ^= (uint32_t)*cursor;
        hash *= UINT32_C(16777619);
    }
    return hash;
}

static uint32_t hwmon_identity_hash(const char *sys_root,
                                    const char *directory,
                                    const char *name,
                                    const char *label,
                                    unsigned int channel) {
    char identity[PATH_MAX];
    const char *material = directory;
    if (realpath(directory, identity)) {
        material = identity;
        size_t root_length = sys_root ? strlen(sys_root) : 0;
        if (root_length > 0 && strncmp(identity, sys_root, root_length) == 0) {
            material = identity + root_length;
        }
        char *hwmon_tail = strstr((char *)material, "/hwmon/hwmon");
        if (hwmon_tail) *hwmon_tail = '\0';
    }
    char channel_text[16];
    snprintf(channel_text, sizeof(channel_text), "%u", channel);
    uint32_t hash = UINT32_C(2166136261);
    hash = fnv1a_update(hash, material);
    hash = fnv1a_update(hash, "|");
    hash = fnv1a_update(hash, name);
    hash = fnv1a_update(hash, "|");
    hash = fnv1a_update(hash, label);
    hash = fnv1a_update(hash, "|");
    return fnv1a_update(hash, channel_text);
}

static bool suffix_is_digits(const char *name, const char *prefix) {
    if (!name || !prefix) return false;
    size_t prefix_length = strlen(prefix);
    if (strncmp(name, prefix, prefix_length) != 0 || name[prefix_length] == '\0') {
        return false;
    }
    for (const unsigned char *cursor =
             (const unsigned char *)(name + prefix_length);
         *cursor; ++cursor) {
        if (!isdigit(*cursor)) return false;
    }
    return true;
}

static void candidate_consider(thermal_candidate_t *candidates,
                               size_t *candidate_count,
                               size_t *total_candidates,
                               const thermal_candidate_t *candidate) {
    if (!candidates || !candidate_count || !total_candidates || !candidate) {
        return;
    }
    (*total_candidates)++;
    size_t position = *candidate_count;
    if (*candidate_count < THERMAL_CANDIDATE_CAPACITY) {
        (*candidate_count)++;
    } else {
        if (strcmp(candidate->id,
                   candidates[THERMAL_CANDIDATE_CAPACITY - 1U].id) >= 0) {
            return;
        }
        position = THERMAL_CANDIDATE_CAPACITY - 1U;
    }
    while (position > 0 &&
           strcmp(candidate->id, candidates[position - 1U].id) < 0) {
        if (position < THERMAL_CANDIDATE_CAPACITY) {
            candidates[position] = candidates[position - 1U];
        }
        position--;
    }
    candidates[position] = *candidate;
}

static void build_zone_id(const char *entry, const char *type,
                          char output[SYSTEM_HEALTH_ID_LENGTH]) {
    char normalized[32];
    normalize_component(type, normalized, sizeof(normalized));
    const char *zone = entry + strlen("thermal_zone");
    snprintf(output, SYSTEM_HEALTH_ID_LENGTH, "thermal.%.28s.zone%.12s",
             normalized, zone);
}

static void build_hwmon_id(const char *sys_root, const char *directory,
                           const char *name, const char *label,
                           unsigned int channel,
                           char output[SYSTEM_HEALTH_ID_LENGTH]) {
    char normalized_name[18];
    char normalized_label[24];
    normalize_component(name, normalized_name, sizeof(normalized_name));
    normalize_component(label, normalized_label, sizeof(normalized_label));
    uint32_t identity = hwmon_identity_hash(sys_root, directory, name, label,
                                             channel);
    snprintf(output, SYSTEM_HEALTH_ID_LENGTH, "hwmon.%.16s.%.22s.%08x",
             normalized_name, normalized_label, identity);
}

static system_health_capability_t merge_capability(
    system_health_capability_t current,
    system_health_capability_t incoming) {
    if (current == SYSTEM_HEALTH_CAPABILITY_AVAILABLE ||
        incoming == SYSTEM_HEALTH_CAPABILITY_AVAILABLE) {
        return SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    }
    if (current == SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED ||
        incoming == SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED) {
        return SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED;
    }
    if (current == SYSTEM_HEALTH_CAPABILITY_ERROR ||
        incoming == SYSTEM_HEALTH_CAPABILITY_ERROR) {
        return SYSTEM_HEALTH_CAPABILITY_ERROR;
    }
    return SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
}

static system_health_capability_t enumerate_zones(
    const char *sys_root, thermal_candidate_t *candidates,
    size_t *candidate_count, size_t *total_candidates) {
    char class_path[PATH_MAX];
    int written = snprintf(class_path, sizeof(class_path), "%s/class/thermal",
                           sys_root);
    if (written < 0 || (size_t)written >= sizeof(class_path)) {
        return SYSTEM_HEALTH_CAPABILITY_ERROR;
    }

    DIR *directory = opendir(class_path);
    if (!directory) return linux_thermal_capability_from_errno(errno);
    struct dirent *entry;
    while ((entry = readdir(directory)) != NULL) {
        if (!suffix_is_digits(entry->d_name, "thermal_zone")) continue;
        thermal_candidate_t candidate;
        memset(&candidate, 0, sizeof(candidate));
        candidate.source = THERMAL_SOURCE_ZONE;
        snprintf(candidate.entry, sizeof(candidate.entry), "%s", entry->d_name);

        char type_path[PATH_MAX];
        char type[THERMAL_TEXT_LENGTH] = "unknown";
        system_health_capability_t ignored;
        if (format_path(type_path, sizeof(type_path), "%s/class/thermal/%s/type",
                        sys_root, candidate.entry, 0)) {
            (void)read_text(type_path, type, &ignored);
        }
        build_zone_id(candidate.entry, type, candidate.id);
        candidate_consider(candidates, candidate_count, total_candidates,
                           &candidate);
    }
    closedir(directory);
    return SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
}

static bool parse_temp_input_name(const char *name, unsigned int *channel) {
    if (!name || strncmp(name, "temp", 4) != 0) return false;
    const char *cursor = name + 4;
    if (!isdigit((unsigned char)*cursor)) return false;
    errno = 0;
    char *end = NULL;
    unsigned long parsed = strtoul(cursor, &end, 10);
    if (errno == ERANGE || end == cursor || parsed == 0 || parsed > 999U ||
        strcmp(end, "_input") != 0) {
        return false;
    }
    *channel = (unsigned int)parsed;
    return true;
}

static system_health_capability_t enumerate_hwmon(
    const char *sys_root, thermal_candidate_t *candidates,
    size_t *candidate_count, size_t *total_candidates) {
    char class_path[PATH_MAX];
    int written = snprintf(class_path, sizeof(class_path), "%s/class/hwmon",
                           sys_root);
    if (written < 0 || (size_t)written >= sizeof(class_path)) {
        return SYSTEM_HEALTH_CAPABILITY_ERROR;
    }
    DIR *directory = opendir(class_path);
    if (!directory) return linux_thermal_capability_from_errno(errno);

    struct dirent *entry;
    while ((entry = readdir(directory)) != NULL) {
        if (!suffix_is_digits(entry->d_name, "hwmon")) continue;
        char hwmon_path[PATH_MAX];
        written = snprintf(hwmon_path, sizeof(hwmon_path), "%s/%s", class_path,
                           entry->d_name);
        if (written < 0 || (size_t)written >= sizeof(hwmon_path)) continue;

        char name_path[PATH_MAX];
        char name[THERMAL_TEXT_LENGTH] = "hwmon";
        system_health_capability_t ignored;
        written = snprintf(name_path, sizeof(name_path), "%s/name", hwmon_path);
        if (written >= 0 && (size_t)written < sizeof(name_path)) {
            (void)read_text(name_path, name, &ignored);
        }

        DIR *hwmon_directory = opendir(hwmon_path);
        if (!hwmon_directory) continue;
        struct dirent *channel_entry;
        while ((channel_entry = readdir(hwmon_directory)) != NULL) {
            unsigned int channel = 0;
            if (!parse_temp_input_name(channel_entry->d_name, &channel)) continue;

            char label_path[PATH_MAX];
            char label[THERMAL_TEXT_LENGTH];
            snprintf(label, sizeof(label), "temp%u", channel);
            written = snprintf(label_path, sizeof(label_path), "%s/temp%u_label",
                               hwmon_path, channel);
            if (written >= 0 && (size_t)written < sizeof(label_path)) {
                char read_label[THERMAL_TEXT_LENGTH];
                if (read_text(label_path, read_label, &ignored)) {
                    snprintf(label, sizeof(label), "%s", read_label);
                }
            }

            thermal_candidate_t candidate;
            memset(&candidate, 0, sizeof(candidate));
            candidate.source = THERMAL_SOURCE_HWMON;
            candidate.channel = channel;
            snprintf(candidate.entry, sizeof(candidate.entry), "%s",
                     entry->d_name);
            build_hwmon_id(sys_root, hwmon_path, name, label, channel,
                           candidate.id);
            candidate_consider(candidates, candidate_count, total_candidates,
                               &candidate);
        }
        closedir(hwmon_directory);
    }
    closedir(directory);
    return SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
}

static thermal_numeric_t zone_critical_limit(const char *sys_root,
                                             const char *entry) {
    thermal_numeric_t best = {
        .valid = false,
        .capability = SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
        .value = 0.0
    };
    for (unsigned int index = 0; index < THERMAL_TRIP_LIMIT; ++index) {
        char type_path[PATH_MAX];
        if (!format_path(type_path, sizeof(type_path),
                         "%s/class/thermal/%s/trip_point_%u_type", sys_root,
                         entry, index)) {
            best.capability = SYSTEM_HEALTH_CAPABILITY_ERROR;
            continue;
        }
        char type[THERMAL_TEXT_LENGTH];
        system_health_capability_t type_capability;
        if (!read_text(type_path, type, &type_capability)) {
            if (type_capability != SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED) {
                best.capability = merge_capability(best.capability,
                                                   type_capability);
            }
            continue;
        }
        if (strcasecmp(type, "critical") != 0) continue;

        char temp_path[PATH_MAX];
        if (!format_path(temp_path, sizeof(temp_path),
                         "%s/class/thermal/%s/trip_point_%u_temp", sys_root,
                         entry, index)) {
            best.capability = SYSTEM_HEALTH_CAPABILITY_ERROR;
            continue;
        }
        thermal_numeric_t candidate = read_millidegrees(temp_path);
        if (candidate.valid && (!best.valid || candidate.value < best.value)) {
            best = candidate;
        } else if (!candidate.valid && !best.valid) {
            best.capability = merge_capability(best.capability,
                                               candidate.capability);
        }
    }
    return best;
}

static thermal_numeric_t hwmon_limit(const char *sys_root,
                                     const thermal_candidate_t *candidate,
                                     bool *is_critical) {
    thermal_numeric_t result = {
        .valid = false,
        .capability = SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED,
        .value = 0.0
    };
    *is_critical = false;
    char path[PATH_MAX];
    if (!format_path(path, sizeof(path),
                     "%s/class/hwmon/%s/temp%u_crit", sys_root,
                     candidate->entry, candidate->channel)) {
        result.capability = SYSTEM_HEALTH_CAPABILITY_ERROR;
        return result;
    }
    result = read_millidegrees(path);
    if (result.valid) {
        *is_critical = true;
        return result;
    }
    if (result.capability != SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED) return result;

    if (!format_path(path, sizeof(path),
                     "%s/class/hwmon/%s/temp%u_max", sys_root,
                     candidate->entry, candidate->channel)) {
        result.capability = SYSTEM_HEALTH_CAPABILITY_ERROR;
        return result;
    }
    return read_millidegrees(path);
}

static linux_thermal_sensor_state_t *track_sensor(
    linux_thermal_state_t *state, const char *id) {
    for (size_t index = 0; index < state->sensor_count; ++index) {
        if (strcmp(state->sensors[index].id, id) == 0) {
            return &state->sensors[index];
        }
    }
    size_t slot = state->sensor_count;
    if (slot >= LINUX_THERMAL_STATE_CAPACITY) {
        for (size_t index = 0; index < state->sensor_count; ++index) {
            if (!state->sensors[index].present &&
                state->sensors[index].missing_cycles > 0) {
                slot = index;
                break;
            }
        }
    } else {
        state->sensor_count++;
    }
    if (slot >= LINUX_THERMAL_STATE_CAPACITY) return NULL;
    memset(&state->sensors[slot], 0, sizeof(state->sensors[slot]));
    snprintf(state->sensors[slot].id, sizeof(state->sensors[slot].id), "%s", id);
    return &state->sensors[slot];
}

static void append_observation(system_health_observation_sink_t *sink,
                               const system_health_collect_context_t *context,
                               const char *metric, const char *resource_id,
                               system_health_capability_t capability,
                               bool value_valid, double value,
                               system_health_unit_t unit) {
    system_health_observation_t observation;
    memset(&observation, 0, sizeof(observation));
    snprintf(observation.metric, sizeof(observation.metric), "%s", metric);
    snprintf(observation.resource_id, sizeof(observation.resource_id), "%s",
             resource_id);
    observation.scope = SYSTEM_HEALTH_SCOPE_HOST;
    observation.sampled_monotonic_ms = context->monotonic_ms;
    observation.observed_wall_time_ms = context->wall_time_ms;
    if (value_valid) {
        system_health_observation_set_available(&observation, value, unit);
    } else {
        system_health_observation_set_unavailable(&observation, capability);
    }
    (void)system_health_observation_sink_append(sink, &observation);
}

void linux_thermal_state_init(linux_thermal_state_t *state) {
    if (state) memset(state, 0, sizeof(*state));
}

int linux_thermal_collect(linux_thermal_state_t *state,
                          const system_health_collect_context_t *context,
                          system_health_observation_sink_t *sink,
                          linux_thermal_result_t *result) {
    if (!state || !context || !context->sys_root || !sink) return -1;
    if (result) memset(result, 0, sizeof(*result));
    for (size_t index = 0; index < state->sensor_count; ++index) {
        state->sensors[index].seen = false;
    }

    thermal_candidate_t candidates[THERMAL_CANDIDATE_CAPACITY];
    size_t candidate_count = 0;
    size_t total_candidates = 0;
    system_health_capability_t zone_capability = enumerate_zones(
        context->sys_root, candidates, &candidate_count, &total_candidates);
    system_health_capability_t hwmon_capability = enumerate_hwmon(
        context->sys_root, candidates, &candidate_count, &total_candidates);
    system_health_capability_t overall = merge_capability(zone_capability,
                                                          hwmon_capability);
    if (total_candidates == 0 && overall == SYSTEM_HEALTH_CAPABILITY_AVAILABLE) {
        overall = SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
    }

    size_t export_count = candidate_count;
    if (export_count > SYSTEM_HEALTH_MAX_SENSORS) {
        export_count = SYSTEM_HEALTH_MAX_SENSORS;
    }
    size_t resources_dropped = total_candidates > export_count
        ? total_candidates - export_count : 0;
    state->resources_dropped_total += resources_dropped;
    if (result) {
        result->capability = overall;
        result->resources_dropped = resources_dropped;
    }
    append_observation(sink, context, "thermal.collector_available", "thermal",
                       overall,
                       overall == SYSTEM_HEALTH_CAPABILITY_AVAILABLE, 1.0,
                       SYSTEM_HEALTH_UNIT_BOOLEAN);

    for (size_t index = 0; index < export_count; ++index) {
        const thermal_candidate_t *candidate = &candidates[index];
        linux_thermal_sensor_state_t *tracked = track_sensor(state,
                                                              candidate->id);
        if (!tracked) {
            state->resources_dropped_total++;
            if (result) result->resources_dropped++;
            continue;
        }
        tracked->seen = true;
        tracked->present = true;
        tracked->missing_cycles = 0;

        char temperature_path[PATH_MAX];
        bool path_ok;
        if (candidate->source == THERMAL_SOURCE_ZONE) {
            path_ok = format_path(temperature_path, sizeof(temperature_path),
                                  "%s/class/thermal/%s/temp", context->sys_root,
                                  candidate->entry, 0);
        } else {
            path_ok = format_path(temperature_path, sizeof(temperature_path),
                                  "%s/class/hwmon/%s/temp%u_input",
                                  context->sys_root, candidate->entry,
                                  candidate->channel);
        }
        thermal_numeric_t temperature = path_ok
            ? read_millidegrees(temperature_path)
            : (thermal_numeric_t){ false, SYSTEM_HEALTH_CAPABILITY_ERROR, 0.0 };

        bool limit_is_critical = candidate->source == THERMAL_SOURCE_ZONE;
        thermal_numeric_t limit = candidate->source == THERMAL_SOURCE_ZONE
            ? zone_critical_limit(context->sys_root, candidate->entry)
            : hwmon_limit(context->sys_root, candidate, &limit_is_critical);
        const char *limit_metric = limit_is_critical
            ? "thermal.critical_celsius" : "thermal.maximum_celsius";
        if (!limit.valid && !limit_is_critical) {
            /* Keep absence explicit under the preferred critical-limit name. */
            limit_metric = "thermal.critical_celsius";
        }
        snprintf(tracked->limit_metric, sizeof(tracked->limit_metric), "%s",
                 limit_metric);

        append_observation(sink, context, "thermal.temperature_celsius",
                           candidate->id, temperature.capability,
                           temperature.valid, temperature.value,
                           SYSTEM_HEALTH_UNIT_CELSIUS);
        append_observation(sink, context, limit_metric, candidate->id,
                           limit.capability, limit.valid, limit.value,
                           SYSTEM_HEALTH_UNIT_CELSIUS);

        if (result && result->sensor_count < SYSTEM_HEALTH_MAX_SENSORS) {
            linux_thermal_sensor_sample_t *sample =
                &result->sensors[result->sensor_count++];
            memset(sample, 0, sizeof(*sample));
            snprintf(sample->id, sizeof(sample->id), "%s", candidate->id);
            sample->temperature_capability = temperature.capability;
            sample->temperature_valid = temperature.valid;
            sample->temperature_celsius = temperature.value;
            sample->limit_capability = limit.capability;
            sample->limit_valid = limit.valid;
            sample->limit_is_critical = limit_is_critical && limit.valid;
            sample->limit_celsius = limit.value;
        }
    }

    for (size_t index = 0; index < state->sensor_count; ++index) {
        linux_thermal_sensor_state_t *tracked = &state->sensors[index];
        if (tracked->seen) continue;
        if (tracked->present) {
            tracked->present = false;
            tracked->missing_cycles = 1;
            append_observation(sink, context, "thermal.temperature_celsius",
                               tracked->id, SYSTEM_HEALTH_CAPABILITY_STALE,
                               false, 0.0, SYSTEM_HEALTH_UNIT_CELSIUS);
            append_observation(sink, context,
                               tracked->limit_metric[0]
                                   ? tracked->limit_metric
                                   : "thermal.critical_celsius",
                               tracked->id, SYSTEM_HEALTH_CAPABILITY_STALE,
                               false, 0.0, SYSTEM_HEALTH_UNIT_CELSIUS);
        } else if (tracked->missing_cycles < UINT8_MAX) {
            tracked->missing_cycles++;
        }
    }

    if (resources_dropped > 0) {
        append_observation(sink, context, "thermal.sensors_dropped", "thermal",
                           SYSTEM_HEALTH_CAPABILITY_AVAILABLE, true,
                           (double)resources_dropped,
                           SYSTEM_HEALTH_UNIT_COUNT);
    }
    return 0;
}

static int collect_adapter(void *state,
                           const system_health_collect_context_t *context,
                           system_health_observation_sink_t *sink) {
    return linux_thermal_collect((linux_thermal_state_t *)state, context, sink,
                                 NULL);
}

bool linux_thermal_collector_init(system_health_collector_t *collector,
                                  linux_thermal_state_t *state,
                                  uint32_t interval_seconds,
                                  uint32_t stale_after_seconds) {
    if (!collector || !state || interval_seconds == 0 ||
        stale_after_seconds < interval_seconds) {
        return false;
    }
    memset(collector, 0, sizeof(*collector));
    snprintf(collector->name, sizeof(collector->name), "linux_thermal");
    collector->scope = SYSTEM_HEALTH_SCOPE_HOST;
    collector->tier = SYSTEM_HEALTH_TIER_NORMAL;
    collector->interval_seconds = interval_seconds;
    collector->stale_after_seconds = stale_after_seconds;
    collector->state = state;
    collector->collect = collect_adapter;
    return true;
}
