#define _POSIX_C_SOURCE 200809L

#include "telemetry/collectors/linux_network.h"

#include <ctype.h>
#include <dirent.h>
#include <errno.h>
#include <limits.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#ifndef PATH_MAX
#define PATH_MAX 4096
#endif

#define NETWORK_DIRECTORY_CAPACITY 128U
#define NETWORK_TEXT_LENGTH 128U
#define ROUTE_FLAG_UP UINT32_C(0x0001)
#define ROUTE_FLAG_REJECT UINT32_C(0x0200)

typedef struct {
    bool valid;
    system_health_capability_t capability;
    uint64_t value;
} network_integer_t;

static const char *const counter_files[LINUX_NETWORK_COUNTER_COUNT] = {
    "rx_bytes", "rx_packets", "rx_errors", "rx_dropped",
    "tx_bytes", "tx_packets", "tx_errors", "tx_dropped"
};

static const char *const counter_metrics[LINUX_NETWORK_COUNTER_COUNT] = {
    "network.rx_bytes_total", "network.rx_packets_total",
    "network.rx_errors_total", "network.rx_drops_total",
    "network.tx_bytes_total", "network.tx_packets_total",
    "network.tx_errors_total", "network.tx_drops_total"
};

static const system_health_unit_t counter_units[LINUX_NETWORK_COUNTER_COUNT] = {
    SYSTEM_HEALTH_UNIT_BYTES, SYSTEM_HEALTH_UNIT_COUNT,
    SYSTEM_HEALTH_UNIT_COUNT, SYSTEM_HEALTH_UNIT_COUNT,
    SYSTEM_HEALTH_UNIT_BYTES, SYSTEM_HEALTH_UNIT_COUNT,
    SYSTEM_HEALTH_UNIT_COUNT, SYSTEM_HEALTH_UNIT_COUNT
};

system_health_capability_t linux_network_capability_from_errno(
    int error_number) {
    if (error_number == EACCES || error_number == EPERM) {
        return SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED;
    }
    if (error_number == ENOENT || error_number == ENOTDIR) {
        return SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
    }
    return SYSTEM_HEALTH_CAPABILITY_ERROR;
}

static bool valid_interface_name(const char *name) {
    if (!name) return false;
    size_t length = strlen(name);
    if (length == 0 || length >= LINUX_NETWORK_INTERNAL_NAME_LENGTH ||
        strcmp(name, ".") == 0 || strstr(name, "..") != NULL) {
        return false;
    }
    for (const unsigned char *cursor = (const unsigned char *)name;
         *cursor; ++cursor) {
        if (!isalnum(*cursor) && *cursor != '_' && *cursor != '-' &&
            *cursor != '.') {
            return false;
        }
    }
    return true;
}

static uint32_t fnv1a(const char *text) {
    uint32_t hash = UINT32_C(2166136261);
    if (!text) return hash;
    for (const unsigned char *cursor = (const unsigned char *)text;
         *cursor; ++cursor) {
        hash ^= (uint32_t)*cursor;
        hash *= UINT32_C(16777619);
    }
    return hash;
}

static void make_public_id(const char *internal_name,
                           char output[SYSTEM_HEALTH_ID_LENGTH]) {
    char normalized[36];
    size_t used = 0;
    bool separator = false;
    for (const unsigned char *cursor = (const unsigned char *)internal_name;
         *cursor && used + 1U < sizeof(normalized); ++cursor) {
        if (isalnum(*cursor)) {
            normalized[used++] = (char)tolower(*cursor);
            separator = false;
        } else if (!separator && used > 0) {
            normalized[used++] = '_';
            separator = true;
        }
    }
    while (used > 0 && normalized[used - 1U] == '_') used--;
    if (used == 0) normalized[used++] = 'i';
    normalized[used] = '\0';
    snprintf(output, SYSTEM_HEALTH_ID_LENGTH, "net.%.34s.%08x", normalized,
             fnv1a(internal_name));
}

static bool read_text(const char *path, char output[NETWORK_TEXT_LENGTH],
                      system_health_capability_t *capability) {
    output[0] = '\0';
    FILE *file = fopen(path, "r");
    if (!file) {
        *capability = linux_network_capability_from_errno(errno);
        return false;
    }
    size_t length = fread(output, 1, NETWORK_TEXT_LENGTH - 1U, file);
    if (ferror(file)) {
        int saved_errno = errno;
        fclose(file);
        *capability = linux_network_capability_from_errno(saved_errno);
        output[0] = '\0';
        return false;
    }
    if (length == NETWORK_TEXT_LENGTH - 1U && fgetc(file) != EOF) {
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

static network_integer_t read_u64(const char *path) {
    network_integer_t result = {
        .valid = false,
        .capability = SYSTEM_HEALTH_CAPABILITY_ERROR,
        .value = 0
    };
    char text[NETWORK_TEXT_LENGTH];
    if (!read_text(path, text, &result.capability)) return result;
    if (text[0] == '-') {
        result.capability = SYSTEM_HEALTH_CAPABILITY_ERROR;
        return result;
    }
    errno = 0;
    char *end = NULL;
    unsigned long long parsed = strtoull(text, &end, 10);
    if (errno == ERANGE || end == text || !end || *end != '\0') {
        result.capability = SYSTEM_HEALTH_CAPABILITY_ERROR;
        return result;
    }
    result.valid = true;
    result.capability = SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    result.value = (uint64_t)parsed;
    return result;
}

static bool parse_hex_u32(const char *text, uint32_t *output) {
    if (!text || !output || text[0] == '\0' || strlen(text) > 8U ||
        text[0] == '-') {
        return false;
    }
    errno = 0;
    char *end = NULL;
    unsigned long parsed = strtoul(text, &end, 16);
    if (errno == ERANGE || end == text || !end || *end != '\0' ||
        parsed > UINT32_MAX) {
        return false;
    }
    *output = (uint32_t)parsed;
    return true;
}

static bool parse_decimal_u32(const char *text, uint32_t *output) {
    if (!text || !output || text[0] == '\0' || text[0] == '-') return false;
    errno = 0;
    char *end = NULL;
    unsigned long parsed = strtoul(text, &end, 10);
    if (errno == ERANGE || end == text || !end || *end != '\0' ||
        parsed > UINT32_MAX) {
        return false;
    }
    *output = (uint32_t)parsed;
    return true;
}

static unsigned int prefix_bits(uint32_t mask) {
    unsigned int bits = 0;
    while (mask != 0) {
        bits += mask & UINT32_C(1);
        mask >>= 1U;
    }
    return bits;
}

int linux_network_select_primary(
    const char *proc_root,
    char interface_name[LINUX_NETWORK_INTERNAL_NAME_LENGTH],
    system_health_capability_t *capability) {
    if (!proc_root || !interface_name || !capability) return -1;
    interface_name[0] = '\0';
    char path[PATH_MAX];
    int written = snprintf(path, sizeof(path), "%s/net/route", proc_root);
    if (written < 0 || (size_t)written >= sizeof(path)) {
        *capability = SYSTEM_HEALTH_CAPABILITY_ERROR;
        return -1;
    }
    FILE *file = fopen(path, "r");
    if (!file) {
        *capability = linux_network_capability_from_errno(errno);
        return -1;
    }

    char line[512];
    if (!fgets(line, sizeof(line), file)) {
        fclose(file);
        *capability = SYSTEM_HEALTH_CAPABILITY_ERROR;
        return -1;
    }

    bool found = false;
    bool saw_well_formed = false;
    unsigned int best_prefix = 0;
    uint32_t best_metric = UINT32_MAX;
    while (fgets(line, sizeof(line), file)) {
        if (!strchr(line, '\n') && !feof(file)) {
            int character;
            while ((character = fgetc(file)) != '\n' && character != EOF) {}
            continue;
        }
        char name[LINUX_NETWORK_INTERNAL_NAME_LENGTH];
        char destination_text[16], gateway_text[16], flags_text[16], mask_text[16];
        char ref_text[16], use_text[16], metric_text[16];
        char mtu_text[16], window_text[16], irtt_text[16];
        int fields = sscanf(line,
            "%63s %15s %15s %15s %15s %15s %15s %15s %15s %15s %15s",
            name, destination_text, gateway_text, flags_text, ref_text,
            use_text, metric_text, mask_text, mtu_text, window_text, irtt_text);
        if (fields < 8 || !valid_interface_name(name)) continue;

        uint32_t destination, gateway, flags, mask, metric;
        if (!parse_hex_u32(destination_text, &destination) ||
            !parse_hex_u32(gateway_text, &gateway) ||
            !parse_hex_u32(flags_text, &flags) ||
            !parse_hex_u32(mask_text, &mask) ||
            !parse_decimal_u32(metric_text, &metric)) {
            continue;
        }
        (void)gateway;
        saw_well_formed = true;
        if (destination != 0 || (flags & ROUTE_FLAG_UP) == 0 ||
            (flags & ROUTE_FLAG_REJECT) != 0) {
            continue;
        }
        unsigned int prefix = prefix_bits(mask);
        if (!found || prefix > best_prefix ||
            (prefix == best_prefix && metric < best_metric) ||
            (prefix == best_prefix && metric == best_metric &&
             strcmp(name, interface_name) < 0)) {
            snprintf(interface_name, LINUX_NETWORK_INTERNAL_NAME_LENGTH, "%s",
                     name);
            best_prefix = prefix;
            best_metric = metric;
            found = true;
        }
    }
    bool read_error = ferror(file);
    fclose(file);
    if (read_error) {
        *capability = SYSTEM_HEALTH_CAPABILITY_ERROR;
        interface_name[0] = '\0';
        return -1;
    }
    if (!found) {
        *capability = saw_well_formed
            ? SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED
            : SYSTEM_HEALTH_CAPABILITY_ERROR;
        return -1;
    }
    *capability = SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    return 0;
}

void linux_network_state_init(linux_network_state_t *state,
                              system_health_scope_t scope) {
    if (!state) return;
    memset(state, 0, sizeof(*state));
    state->scope = scope == SYSTEM_HEALTH_SCOPE_HOST
        ? SYSTEM_HEALTH_SCOPE_HOST : SYSTEM_HEALTH_SCOPE_CONTAINER;
}

bool linux_network_set_primary_override(linux_network_state_t *state,
                                        const char *interface_name) {
    if (!state || !interface_name) return false;
    if (interface_name[0] == '\0') {
        state->primary_override[0] = '\0';
        return true;
    }
    if (!valid_interface_name(interface_name)) return false;
    snprintf(state->primary_override, sizeof(state->primary_override), "%s",
             interface_name);
    return true;
}

static void name_consider(
    char names[NETWORK_DIRECTORY_CAPACITY][LINUX_NETWORK_INTERNAL_NAME_LENGTH],
    size_t *stored, size_t *total, const char *name) {
    (*total)++;
    size_t position = *stored;
    if (*stored < NETWORK_DIRECTORY_CAPACITY) {
        (*stored)++;
    } else {
        if (strcmp(name, names[NETWORK_DIRECTORY_CAPACITY - 1U]) >= 0) return;
        position = NETWORK_DIRECTORY_CAPACITY - 1U;
    }
    while (position > 0 && strcmp(name, names[position - 1U]) < 0) {
        if (position < NETWORK_DIRECTORY_CAPACITY) {
            memmove(names[position], names[position - 1U],
                    LINUX_NETWORK_INTERNAL_NAME_LENGTH);
        }
        position--;
    }
    snprintf(names[position], LINUX_NETWORK_INTERNAL_NAME_LENGTH, "%s", name);
}

static bool name_in_list(
    char names[NETWORK_DIRECTORY_CAPACITY][LINUX_NETWORK_INTERNAL_NAME_LENGTH],
    size_t count, const char *name, size_t *index_out) {
    for (size_t index = 0; index < count; ++index) {
        int order = strcmp(names[index], name);
        if (order == 0) {
            if (index_out) *index_out = index;
            return true;
        }
        if (order > 0) break;
    }
    return false;
}

static linux_network_interface_state_t *track_interface(
    linux_network_state_t *state, const char *internal_name) {
    for (size_t index = 0; index < state->interface_count; ++index) {
        if (strcmp(state->interfaces[index].internal_name, internal_name) == 0) {
            return &state->interfaces[index];
        }
    }
    size_t slot = state->interface_count;
    if (slot >= LINUX_NETWORK_STATE_CAPACITY) {
        for (size_t index = 0; index < state->interface_count; ++index) {
            if (!state->interfaces[index].present &&
                state->interfaces[index].missing_cycles > 0) {
                slot = index;
                break;
            }
        }
    } else {
        state->interface_count++;
    }
    if (slot >= LINUX_NETWORK_STATE_CAPACITY) return NULL;
    memset(&state->interfaces[slot], 0, sizeof(state->interfaces[slot]));
    snprintf(state->interfaces[slot].internal_name,
             sizeof(state->interfaces[slot].internal_name), "%s", internal_name);
    make_public_id(internal_name, state->interfaces[slot].id);
    return &state->interfaces[slot];
}

static void append_observation(system_health_observation_sink_t *sink,
                               const system_health_collect_context_t *context,
                               system_health_scope_t scope, const char *metric,
                               const char *resource_id,
                               system_health_capability_t capability,
                               bool value_valid, double value,
                               system_health_unit_t unit) {
    system_health_observation_t observation;
    memset(&observation, 0, sizeof(observation));
    snprintf(observation.metric, sizeof(observation.metric), "%s", metric);
    snprintf(observation.resource_id, sizeof(observation.resource_id), "%s",
             resource_id);
    observation.scope = scope;
    observation.sampled_monotonic_ms = context->monotonic_ms;
    observation.observed_wall_time_ms = context->wall_time_ms;
    if (value_valid) {
        system_health_observation_set_available(&observation, value, unit);
    } else {
        system_health_observation_set_unavailable(&observation, capability);
    }
    (void)system_health_observation_sink_append(sink, &observation);
}

static system_health_counter_t update_counter(
    linux_network_interface_state_t *tracked,
    linux_network_counter_kind_t kind, network_integer_t reading,
    uint64_t monotonic_ms) {
    system_health_counter_t counter;
    memset(&counter, 0, sizeof(counter));
    counter.sampled_monotonic_ms = monotonic_ms;
    if (!reading.valid) {
        tracked->previous_valid[kind] = false;
        return counter;
    }
    counter.current = reading.value;
    if (tracked->previous_valid[kind]) {
        counter.previous = tracked->previous[kind];
        if (reading.value < tracked->previous[kind]) {
            counter.reset_detected = true;
        } else if (monotonic_ms > tracked->previous_monotonic_ms) {
            counter.delta_valid = true;
            counter.delta = reading.value - tracked->previous[kind];
            counter.interval_ms = monotonic_ms - tracked->previous_monotonic_ms;
        }
    }
    tracked->previous[kind] = reading.value;
    tracked->previous_valid[kind] = true;
    return counter;
}

static uint64_t saturating_add_u64(uint64_t left, uint64_t right) {
    return UINT64_MAX - left < right ? UINT64_MAX : left + right;
}

static bool all_rate_counters_valid(
    const linux_network_interface_sample_t *sample) {
    const linux_network_counter_kind_t kinds[] = {
        LINUX_NETWORK_RX_PACKETS, LINUX_NETWORK_TX_PACKETS,
        LINUX_NETWORK_RX_ERRORS, LINUX_NETWORK_TX_ERRORS,
        LINUX_NETWORK_RX_DROPS, LINUX_NETWORK_TX_DROPS
    };
    for (size_t index = 0; index < sizeof(kinds) / sizeof(kinds[0]); ++index) {
        if (!sample->counters[kinds[index]].delta_valid) return false;
    }
    return true;
}

int linux_network_collect(linux_network_state_t *state,
                          const system_health_collect_context_t *context,
                          system_health_observation_sink_t *sink,
                          linux_network_result_t *result) {
    if (!state || !context || !context->sys_root || !context->proc_root ||
        !sink) {
        return -1;
    }
    if (result) memset(result, 0, sizeof(*result));
    for (size_t index = 0; index < state->interface_count; ++index) {
        state->interfaces[index].seen = false;
    }

    char primary_name[LINUX_NETWORK_INTERNAL_NAME_LENGTH] = "";
    system_health_capability_t primary_capability;
    if (state->primary_override[0]) {
        snprintf(primary_name, sizeof(primary_name), "%s",
                 state->primary_override);
        primary_capability = SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    } else {
        (void)linux_network_select_primary(context->proc_root, primary_name,
                                           &primary_capability);
    }

    char class_path[PATH_MAX];
    int written = snprintf(class_path, sizeof(class_path), "%s/class/net",
                           context->sys_root);
    if (written < 0 || (size_t)written >= sizeof(class_path)) return -1;
    DIR *directory = opendir(class_path);
    system_health_capability_t overall = SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    char names[NETWORK_DIRECTORY_CAPACITY][LINUX_NETWORK_INTERNAL_NAME_LENGTH];
    size_t stored_names = 0;
    size_t total_names = 0;
    if (!directory) {
        overall = linux_network_capability_from_errno(errno);
    } else {
        struct dirent *entry;
        while ((entry = readdir(directory)) != NULL) {
            if (!valid_interface_name(entry->d_name)) continue;
            name_consider(names, &stored_names, &total_names, entry->d_name);
        }
        closedir(directory);
        if (total_names == 0) overall = SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
    }

    size_t primary_index = 0;
    bool primary_present = primary_capability == SYSTEM_HEALTH_CAPABILITY_AVAILABLE &&
        name_in_list(names, stored_names, primary_name, &primary_index);
    if (primary_capability == SYSTEM_HEALTH_CAPABILITY_AVAILABLE &&
        !primary_present) {
        primary_capability = SYSTEM_HEALTH_CAPABILITY_STALE;
        primary_name[0] = '\0';
    }

    char selected[SYSTEM_HEALTH_MAX_INTERFACES]
                 [LINUX_NETWORK_INTERNAL_NAME_LENGTH];
    size_t selected_count = stored_names < SYSTEM_HEALTH_MAX_INTERFACES
        ? stored_names : SYSTEM_HEALTH_MAX_INTERFACES;
    for (size_t index = 0; index < selected_count; ++index) {
        snprintf(selected[index], sizeof(selected[index]), "%s", names[index]);
    }
    if (primary_present && primary_index >= selected_count && selected_count > 0) {
        snprintf(selected[selected_count - 1U], sizeof(selected[0]), "%s",
                 primary_name);
        for (size_t index = selected_count - 1U; index > 0 &&
             strcmp(selected[index], selected[index - 1U]) < 0; --index) {
            char swap[LINUX_NETWORK_INTERNAL_NAME_LENGTH];
            snprintf(swap, sizeof(swap), "%s", selected[index]);
            memmove(selected[index], selected[index - 1U],
                    sizeof(selected[index]));
            snprintf(selected[index - 1U], sizeof(selected[index - 1U]), "%s",
                     swap);
        }
    }
    size_t resources_dropped = total_names > selected_count
        ? total_names - selected_count : 0;
    state->resources_dropped_total += resources_dropped;
    if (result) {
        result->capability = overall;
        result->primary_capability = primary_capability;
        result->resources_dropped = resources_dropped;
    }
    append_observation(sink, context, state->scope,
                       "network.collector_available", "network", overall,
                       overall == SYSTEM_HEALTH_CAPABILITY_AVAILABLE, 1.0,
                       SYSTEM_HEALTH_UNIT_BOOLEAN);
    append_observation(sink, context, state->scope,
                       "network.primary_available", "network",
                       primary_capability,
                       primary_capability == SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
                       1.0, SYSTEM_HEALTH_UNIT_BOOLEAN);

    for (size_t interface_index = 0; interface_index < selected_count;
         ++interface_index) {
        const char *internal_name = selected[interface_index];
        linux_network_interface_state_t *tracked = track_interface(
            state, internal_name);
        if (!tracked) {
            state->resources_dropped_total++;
            if (result) result->resources_dropped++;
            continue;
        }
        bool was_present = tracked->present;
        tracked->seen = true;
        tracked->present = true;
        tracked->missing_cycles = 0;
        if (!was_present) {
            memset(tracked->previous_valid, 0, sizeof(tracked->previous_valid));
            tracked->carrier_valid = false;
        }

        linux_network_interface_sample_t sample;
        memset(&sample, 0, sizeof(sample));
        snprintf(sample.id, sizeof(sample.id), "%s", tracked->id);
        sample.primary = primary_capability == SYSTEM_HEALTH_CAPABILITY_AVAILABLE &&
            strcmp(internal_name, primary_name) == 0;

        char path[PATH_MAX];
        written = snprintf(path, sizeof(path), "%s/%s/carrier", class_path,
                           internal_name);
        network_integer_t carrier = written >= 0 && (size_t)written < sizeof(path)
            ? read_u64(path)
            : (network_integer_t){ false, SYSTEM_HEALTH_CAPABILITY_ERROR, 0 };
        if (carrier.valid && carrier.value > 1U) {
            carrier.valid = false;
            carrier.capability = SYSTEM_HEALTH_CAPABILITY_ERROR;
        }
        sample.carrier_capability = carrier.capability;
        sample.carrier_valid = carrier.valid;
        sample.carrier = carrier.valid && carrier.value == 1U;
        if (carrier.valid) {
            if (tracked->carrier_valid && tracked->carrier != sample.carrier) {
                tracked->carrier_flaps++;
                sample.carrier_flap_delta = 1;
            }
            tracked->carrier = sample.carrier;
            tracked->carrier_valid = true;
        } else {
            tracked->carrier_valid = false;
        }
        sample.carrier_flaps = tracked->carrier_flaps;
        append_observation(sink, context, state->scope, "network.carrier",
                           tracked->id, carrier.capability, carrier.valid,
                           sample.carrier ? 1.0 : 0.0,
                           SYSTEM_HEALTH_UNIT_BOOLEAN);

        bool any_reset = false;
        for (int counter_index = 0;
             counter_index < LINUX_NETWORK_COUNTER_COUNT; ++counter_index) {
            written = snprintf(path, sizeof(path), "%s/%s/statistics/%s",
                               class_path, internal_name,
                               counter_files[counter_index]);
            network_integer_t reading =
                written >= 0 && (size_t)written < sizeof(path)
                ? read_u64(path)
                : (network_integer_t){ false,
                    SYSTEM_HEALTH_CAPABILITY_ERROR, 0 };
            bool had_previous = tracked->previous_valid[counter_index];
            sample.counters[counter_index] = update_counter(
                tracked, (linux_network_counter_kind_t)counter_index, reading,
                context->monotonic_ms);
            if (reading.valid) {
                uint64_t increment = 0U;
                if (!had_previous ||
                    sample.counters[counter_index].reset_detected) {
                    increment = reading.value;
                } else if (sample.counters[counter_index].delta_valid) {
                    increment = sample.counters[counter_index].delta;
                }
                tracked->exported_total[counter_index] = saturating_add_u64(
                    tracked->exported_total[counter_index], increment);
            }
            any_reset = any_reset ||
                sample.counters[counter_index].reset_detected;
            append_observation(sink, context, state->scope,
                               counter_metrics[counter_index], tracked->id,
                               reading.capability, reading.valid,
                               (double)tracked->exported_total[counter_index],
                               counter_units[counter_index]);
        }
        tracked->previous_monotonic_ms = context->monotonic_ms;

        if (all_rate_counters_valid(&sample)) {
            long double packets =
                (long double)sample.counters[LINUX_NETWORK_RX_PACKETS].delta +
                (long double)sample.counters[LINUX_NETWORK_TX_PACKETS].delta;
            long double bad =
                (long double)sample.counters[LINUX_NETWORK_RX_ERRORS].delta +
                (long double)sample.counters[LINUX_NETWORK_TX_ERRORS].delta +
                (long double)sample.counters[LINUX_NETWORK_RX_DROPS].delta +
                (long double)sample.counters[LINUX_NETWORK_TX_DROPS].delta;
            if (packets > 0.0L) {
                sample.error_drop_ratio_valid = true;
                sample.error_drop_ratio = (double)(bad / packets);
            }
        }
        append_observation(sink, context, state->scope,
                           "network.error_drop_ratio", tracked->id,
                           any_reset ? SYSTEM_HEALTH_CAPABILITY_ERROR
                                     : SYSTEM_HEALTH_CAPABILITY_STALE,
                           sample.error_drop_ratio_valid,
                           sample.error_drop_ratio,
                           SYSTEM_HEALTH_UNIT_RATIO);
        append_observation(sink, context, state->scope, "network.primary",
                           tracked->id, primary_capability,
                           primary_capability == SYSTEM_HEALTH_CAPABILITY_AVAILABLE,
                           sample.primary ? 1.0 : 0.0,
                           SYSTEM_HEALTH_UNIT_BOOLEAN);
        append_observation(sink, context, state->scope,
                           "network.counter_reset", tracked->id,
                           SYSTEM_HEALTH_CAPABILITY_AVAILABLE, true,
                           any_reset ? 1.0 : 0.0,
                           SYSTEM_HEALTH_UNIT_BOOLEAN);

        if (sample.primary && result) {
            snprintf(result->primary_id, sizeof(result->primary_id), "%s",
                     tracked->id);
        }
        if (result && result->interface_count < SYSTEM_HEALTH_MAX_INTERFACES) {
            result->interfaces[result->interface_count++] = sample;
        }
    }

    for (size_t index = 0; index < state->interface_count; ++index) {
        linux_network_interface_state_t *tracked = &state->interfaces[index];
        if (tracked->seen) continue;
        if (tracked->present) {
            tracked->present = false;
            tracked->missing_cycles = 1;
            memset(tracked->previous_valid, 0, sizeof(tracked->previous_valid));
            tracked->carrier_valid = false;
            append_observation(sink, context, state->scope, "network.carrier",
                               tracked->id, SYSTEM_HEALTH_CAPABILITY_STALE,
                               false, 0.0, SYSTEM_HEALTH_UNIT_BOOLEAN);
        } else if (tracked->missing_cycles < UINT8_MAX) {
            tracked->missing_cycles++;
        }
    }

    if (resources_dropped > 0) {
        append_observation(sink, context, state->scope,
                           "network.interfaces_dropped", "network",
                           SYSTEM_HEALTH_CAPABILITY_AVAILABLE, true,
                           (double)resources_dropped,
                           SYSTEM_HEALTH_UNIT_COUNT);
    }
    return 0;
}

static int collect_adapter(void *state,
                           const system_health_collect_context_t *context,
                           system_health_observation_sink_t *sink) {
    return linux_network_collect((linux_network_state_t *)state, context, sink,
                                 NULL);
}

bool linux_network_collector_init(system_health_collector_t *collector,
                                  linux_network_state_t *state,
                                  uint32_t interval_seconds,
                                  uint32_t stale_after_seconds) {
    if (!collector || !state || interval_seconds == 0 ||
        stale_after_seconds < interval_seconds) {
        return false;
    }
    memset(collector, 0, sizeof(*collector));
    snprintf(collector->name, sizeof(collector->name), "linux_network");
    collector->scope = state->scope;
    collector->tier = SYSTEM_HEALTH_TIER_NORMAL;
    collector->interval_seconds = interval_seconds;
    collector->stale_after_seconds = stale_after_seconds;
    collector->state = state;
    collector->collect = collect_adapter;
    return true;
}
