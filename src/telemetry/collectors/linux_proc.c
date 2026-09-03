#include "telemetry/collectors/linux_proc.h"

#include <ctype.h>
#include <errno.h>
#include <inttypes.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define PROC_READ_MAX 16384U

typedef enum {
    READ_OK = 0,
    READ_UNSUPPORTED,
    READ_PERMISSION,
    READ_ERROR
} read_result_t;

static bool parse_u64(const char *text, const char **end_out, uint64_t *out) {
    char *end = NULL;
    unsigned long long value;

    if (!text || !out || !isdigit((unsigned char)*text)) return false;
    errno = 0;
    value = strtoull(text, &end, 10);
    if (errno == ERANGE || end == text) return false;
    *out = (uint64_t)value;
    if (end_out) *end_out = end;
    return true;
}

static bool parse_double_value(const char *text, const char **end_out,
                               double *out) {
    char *end = NULL;
    double value;

    if (!text || !out) return false;
    errno = 0;
    value = strtod(text, &end);
    if (errno == ERANGE || end == text || value < 0.0 || !isfinite(value))
        return false;
    *out = value;
    if (end_out) *end_out = end;
    return true;
}

static const char *skip_space(const char *text) {
    while (text && isspace((unsigned char)*text)) ++text;
    return text;
}

int linux_proc_parse_cpu_stat(const char *text, linux_proc_cpu_times_t *out) {
    uint64_t fields[8];
    const char *cursor;

    if (!text || !out || strncmp(text, "cpu", 3) != 0 ||
        !isspace((unsigned char)text[3])) return -1;
    cursor = text + 3;
    for (size_t i = 0; i < sizeof(fields) / sizeof(fields[0]); ++i) {
        cursor = skip_space(cursor);
        if (!parse_u64(cursor, &cursor, &fields[i])) return -1;
    }
    out->user = fields[0];
    out->nice = fields[1];
    out->system = fields[2];
    out->idle = fields[3];
    out->iowait = fields[4];
    out->irq = fields[5];
    out->softirq = fields[6];
    out->steal = fields[7];
    return 0;
}

int linux_proc_parse_loadavg(const char *text, linux_proc_loadavg_t *out) {
    const char *cursor = text;
    if (!text || !out ||
        !parse_double_value(cursor, &cursor, &out->one)) return -1;
    cursor = skip_space(cursor);
    if (!parse_double_value(cursor, &cursor, &out->five)) return -1;
    cursor = skip_space(cursor);
    if (!parse_double_value(cursor, NULL, &out->fifteen)) return -1;
    return 0;
}

static int meminfo_value(const char *text, const char *key,
                         uint64_t *bytes_out) {
    size_t key_len = strlen(key);
    const char *line = text;
    while (line && *line) {
        const char *next = strchr(line, '\n');
        if (strncmp(line, key, key_len) == 0 && line[key_len] == ':') {
            uint64_t kib;
            const char *end;
            const char *value = skip_space(line + key_len + 1);
            if (!parse_u64(value, &end, &kib) ||
                kib > UINT64_MAX / 1024U) return -1;
            end = skip_space(end);
            if (strncmp(end, "kB", 2) != 0) return -1;
            end += 2;
            while (*end != '\n' && isspace((unsigned char)*end)) ++end;
            if (*end != '\0' && *end != '\n') return -1;
            *bytes_out = kib * 1024U;
            return 0;
        }
        line = next ? next + 1 : NULL;
    }
    return 1;
}

int linux_proc_parse_meminfo(const char *text, linux_proc_memory_t *out) {
    linux_proc_memory_t parsed;
    int total;
    int available;
    int swap_total;
    int swap_free;
    if (!text || !out) return -1;
    memset(&parsed, 0, sizeof(parsed));
    total = meminfo_value(text, "MemTotal", &parsed.total_bytes);
    available = meminfo_value(text, "MemAvailable", &parsed.available_bytes);
    swap_total = meminfo_value(text, "SwapTotal", &parsed.swap_total_bytes);
    swap_free = meminfo_value(text, "SwapFree", &parsed.swap_free_bytes);
    if (total < 0 || available < 0 || swap_total < 0 || swap_free < 0)
        return -1;
    if (total > 0 || available > 0 || swap_total > 0 || swap_free > 0)
        return 1;
    if (parsed.total_bytes == 0 ||
        parsed.available_bytes > parsed.total_bytes ||
        parsed.swap_free_bytes > parsed.swap_total_bytes) return -1;
    *out = parsed;
    return 0;
}

static int keyed_u64(const char *text, const char *key, uint64_t *out) {
    size_t key_len = strlen(key);
    const char *line = text;
    while (line && *line) {
        const char *next = strchr(line, '\n');
        if (strncmp(line, key, key_len) == 0 &&
            isspace((unsigned char)line[key_len])) {
            const char *end;
            const char *value = skip_space(line + key_len);
            if (!parse_u64(value, &end, out)) return -1;
            while (*end != '\n' && isspace((unsigned char)*end)) ++end;
            if (*end != '\0' && *end != '\n') return -1;
            return 0;
        }
        line = next ? next + 1 : NULL;
    }
    return 1;
}

int linux_proc_parse_vmstat(const char *text, linux_proc_vmstat_t *out) {
    linux_proc_vmstat_t parsed;
    int faults;
    int swap_in;
    int swap_out;
    if (!text || !out) return -1;
    faults = keyed_u64(text, "pgmajfault", &parsed.major_faults);
    swap_in = keyed_u64(text, "pswpin", &parsed.swap_in_pages);
    swap_out = keyed_u64(text, "pswpout", &parsed.swap_out_pages);
    if (faults < 0 || swap_in < 0 || swap_out < 0) return -1;
    if (faults > 0 || swap_in > 0 || swap_out > 0) return 1;
    *out = parsed;
    return 0;
}

static int parse_pressure_line(const char *line, const char *kind,
                               double *ratio_out, uint64_t *total_out) {
    char bounded_line[512];
    const char *line_end = strchr(line, '\n');
    size_t line_length = line_end ? (size_t)(line_end - line) : strlen(line);
    size_t kind_len = strlen(kind);
    const char *avg;
    const char *total;
    const char *value_end;
    double percent;

    if (line_length >= sizeof(bounded_line)) return -1;
    memcpy(bounded_line, line, line_length);
    bounded_line[line_length] = '\0';
    line = bounded_line;
    if (strncmp(line, kind, kind_len) != 0 ||
        !isspace((unsigned char)line[kind_len])) return 1;
    avg = strstr(line + kind_len, "avg10=");
    total = strstr(line + kind_len, "total=");
    if (!avg || !total ||
        !parse_double_value(avg + 6, &value_end, &percent) ||
        !isspace((unsigned char)*value_end) || percent > 100.0 ||
        !parse_u64(total + 6, &value_end, total_out)) return -1;
    while (isspace((unsigned char)*value_end)) ++value_end;
    if (*value_end != '\0') return -1;
    *ratio_out = percent / 100.0;
    return 0;
}

int linux_proc_parse_pressure(const char *text, linux_proc_pressure_t *out) {
    const char *line = text;
    linux_proc_pressure_t parsed;
    if (!text || !out) return -1;
    memset(&parsed, 0, sizeof(parsed));
    while (line && *line) {
        const char *next = strchr(line, '\n');
        int rc = parse_pressure_line(line, "some", &parsed.some_avg10_ratio,
                                     &parsed.some_total_usec);
        if (rc < 0) return -1;
        if (rc == 0) parsed.some_present = true;
        rc = parse_pressure_line(line, "full", &parsed.full_avg10_ratio,
                                 &parsed.full_total_usec);
        if (rc < 0) return -1;
        if (rc == 0) parsed.full_present = true;
        line = next ? next + 1 : NULL;
    }
    if (!parsed.some_present) return -1;
    *out = parsed;
    return 0;
}

static read_result_t read_bounded(const char *path, char *buffer,
                                  size_t capacity) {
    FILE *file;
    size_t used;
    int saved_errno;

    if (!path || !buffer || capacity < 2U) return READ_ERROR;
    errno = 0;
    file = fopen(path, "r");
    if (!file) {
        if (errno == ENOENT || errno == ENOTDIR) return READ_UNSUPPORTED;
        if (errno == EACCES || errno == EPERM) return READ_PERMISSION;
        return READ_ERROR;
    }
    used = fread(buffer, 1, capacity - 1U, file);
    saved_errno = ferror(file) ? errno : 0;
    if (!feof(file) && used == capacity - 1U) saved_errno = EOVERFLOW;
    if (fclose(file) != 0 && saved_errno == 0) saved_errno = errno;
    if (saved_errno != 0) return READ_ERROR;
    buffer[used] = '\0';
    return READ_OK;
}

static system_health_capability_t read_capability(read_result_t result) {
    switch (result) {
        case READ_UNSUPPORTED: return SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
        case READ_PERMISSION: return SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED;
        case READ_ERROR: return SYSTEM_HEALTH_CAPABILITY_ERROR;
        case READ_OK: return SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    }
    return SYSTEM_HEALTH_CAPABILITY_ERROR;
}

static read_result_t read_proc_file(const char *root, const char *relative,
                                    char *buffer, size_t capacity) {
    char path[512];
    int length;
    if (!root || !relative) return READ_ERROR;
    length = snprintf(path, sizeof(path), "%s/%s", root, relative);
    if (length < 0 || (size_t)length >= sizeof(path)) return READ_ERROR;
    return read_bounded(path, buffer, capacity);
}

static void init_observation(system_health_observation_t *observation,
                             const system_health_collect_context_t *context,
                             const char *metric) {
    memset(observation, 0, sizeof(*observation));
    (void)snprintf(observation->metric, sizeof(observation->metric), "%s",
                   metric);
    (void)snprintf(observation->resource_id,
                   sizeof(observation->resource_id), "host");
    observation->scope = SYSTEM_HEALTH_SCOPE_HOST;
    observation->sampled_monotonic_ms = context->monotonic_ms;
    observation->observed_wall_time_ms = context->wall_time_ms;
}

static void append_value(system_health_observation_sink_t *sink,
                         const system_health_collect_context_t *context,
                         const char *metric, double value,
                         system_health_unit_t unit) {
    system_health_observation_t observation;
    init_observation(&observation, context, metric);
    system_health_observation_set_available(&observation, value, unit);
    (void)system_health_observation_sink_append(sink, &observation);
}

static void append_unavailable(system_health_observation_sink_t *sink,
                               const system_health_collect_context_t *context,
                               const char *metric,
                               system_health_capability_t capability) {
    system_health_observation_t observation;
    init_observation(&observation, context, metric);
    system_health_observation_set_unavailable(&observation, capability);
    (void)system_health_observation_sink_append(sink, &observation);
}

static bool add_checked(uint64_t left, uint64_t right, uint64_t *out) {
    if (UINT64_MAX - left < right) return false;
    *out = left + right;
    return true;
}

static bool cpu_totals(const linux_proc_cpu_times_t *cpu, uint64_t *busy,
                       uint64_t *idle, uint64_t *total) {
    uint64_t busy_value = 0;
    uint64_t idle_value;
    const uint64_t busy_fields[] = {
        cpu->user, cpu->nice, cpu->system, cpu->irq, cpu->softirq, cpu->steal
    };
    if (!add_checked(cpu->idle, cpu->iowait, &idle_value)) return false;
    for (size_t i = 0; i < sizeof(busy_fields) / sizeof(busy_fields[0]); ++i) {
        if (!add_checked(busy_value, busy_fields[i], &busy_value)) return false;
    }
    if (!add_checked(busy_value, idle_value, total)) return false;
    *busy = busy_value;
    *idle = idle_value;
    return true;
}

static void collect_cpu(linux_proc_state_t *state,
                        const system_health_collect_context_t *context,
                        system_health_observation_sink_t *sink) {
    char text[PROC_READ_MAX];
    linux_proc_cpu_times_t current;
    read_result_t read_result = read_proc_file(context->proc_root, "stat", text,
                                               sizeof(text));
    if (read_result != READ_OK || linux_proc_parse_cpu_stat(text, &current) != 0) {
        append_unavailable(sink, context, "host.cpu.busy_ratio",
            read_result == READ_OK ? SYSTEM_HEALTH_CAPABILITY_ERROR
                                   : read_capability(read_result));
        return;
    }
    if (!state->cpu_valid) {
        append_unavailable(sink, context, "host.cpu.busy_ratio",
                           SYSTEM_HEALTH_CAPABILITY_STALE);
    } else {
        uint64_t current_busy, current_idle, current_total;
        uint64_t previous_busy, previous_idle, previous_total;
        bool valid = context->monotonic_ms > state->cpu_monotonic_ms &&
            cpu_totals(&current, &current_busy, &current_idle, &current_total) &&
            cpu_totals(&state->cpu, &previous_busy, &previous_idle,
                       &previous_total) &&
            current_busy >= previous_busy && current_idle >= previous_idle &&
            current_total > previous_total;
        if (valid) {
            double ratio = (double)(current_busy - previous_busy) /
                           (double)(current_total - previous_total);
            append_value(sink, context, "host.cpu.busy_ratio", ratio,
                         SYSTEM_HEALTH_UNIT_RATIO);
        } else {
            append_unavailable(sink, context, "host.cpu.busy_ratio",
                               SYSTEM_HEALTH_CAPABILITY_ERROR);
        }
    }
    state->cpu = current;
    state->cpu_monotonic_ms = context->monotonic_ms;
    state->cpu_valid = true;
}

static void collect_load(const system_health_collect_context_t *context,
                         system_health_observation_sink_t *sink) {
    char text[PROC_READ_MAX];
    linux_proc_loadavg_t load;
    read_result_t result = read_proc_file(context->proc_root, "loadavg", text,
                                          sizeof(text));
    if (result != READ_OK || linux_proc_parse_loadavg(text, &load) != 0) {
        system_health_capability_t capability = result == READ_OK
            ? SYSTEM_HEALTH_CAPABILITY_ERROR : read_capability(result);
        append_unavailable(sink, context, "host.load.1", capability);
        append_unavailable(sink, context, "host.load.5", capability);
        append_unavailable(sink, context, "host.load.15", capability);
        return;
    }
    append_value(sink, context, "host.load.1", load.one,
                 SYSTEM_HEALTH_UNIT_COUNT);
    append_value(sink, context, "host.load.5", load.five,
                 SYSTEM_HEALTH_UNIT_COUNT);
    append_value(sink, context, "host.load.15", load.fifteen,
                 SYSTEM_HEALTH_UNIT_COUNT);
}

static void collect_memory(const system_health_collect_context_t *context,
                           system_health_observation_sink_t *sink) {
    char text[PROC_READ_MAX];
    linux_proc_memory_t memory;
    read_result_t result = read_proc_file(context->proc_root, "meminfo", text,
                                          sizeof(text));
    int parse_result = result == READ_OK
        ? linux_proc_parse_meminfo(text, &memory) : -1;
    if (result != READ_OK || parse_result != 0) {
        system_health_capability_t capability = result != READ_OK
            ? read_capability(result)
            : parse_result > 0 ? SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED
                               : SYSTEM_HEALTH_CAPABILITY_ERROR;
        append_unavailable(sink, context, "host.memory.total_bytes", capability);
        append_unavailable(sink, context, "host.memory.available_bytes", capability);
        append_unavailable(sink, context, "host.memory.available_ratio", capability);
        append_unavailable(sink, context, "host.swap.used_bytes", capability);
        return;
    }
    append_value(sink, context, "host.memory.total_bytes",
                 (double)memory.total_bytes, SYSTEM_HEALTH_UNIT_BYTES);
    append_value(sink, context, "host.memory.available_bytes",
                 (double)memory.available_bytes, SYSTEM_HEALTH_UNIT_BYTES);
    append_value(sink, context, "host.memory.available_ratio",
                 memory.total_bytes == 0 ? 0.0 :
                     (double)memory.available_bytes / (double)memory.total_bytes,
                 SYSTEM_HEALTH_UNIT_RATIO);
    append_value(sink, context, "host.swap.used_bytes",
                 (double)(memory.swap_total_bytes - memory.swap_free_bytes),
                 SYSTEM_HEALTH_UNIT_BYTES);
}

static void append_counter_delta(system_health_observation_sink_t *sink,
                                 const system_health_collect_context_t *context,
                                 const char *metric, bool previous_valid,
                                 uint64_t previous, uint64_t current,
                                 bool time_valid) {
    if (!previous_valid) {
        append_unavailable(sink, context, metric,
                           SYSTEM_HEALTH_CAPABILITY_STALE);
    } else if (!time_valid || current < previous) {
        append_unavailable(sink, context, metric,
                           SYSTEM_HEALTH_CAPABILITY_ERROR);
    } else {
        append_value(sink, context, metric, (double)(current - previous),
                     SYSTEM_HEALTH_UNIT_COUNT);
    }
}

static void collect_vmstat(linux_proc_state_t *state,
                           const system_health_collect_context_t *context,
                           system_health_observation_sink_t *sink) {
    char text[PROC_READ_MAX];
    linux_proc_vmstat_t current;
    read_result_t result = read_proc_file(context->proc_root, "vmstat", text,
                                          sizeof(text));
    int parse_result = result == READ_OK
        ? linux_proc_parse_vmstat(text, &current) : -1;
    if (result != READ_OK || parse_result != 0) {
        system_health_capability_t capability = result != READ_OK
            ? read_capability(result)
            : parse_result > 0 ? SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED
                               : SYSTEM_HEALTH_CAPABILITY_ERROR;
        append_unavailable(sink, context, "host.vm.major_faults_delta", capability);
        append_unavailable(sink, context, "host.vm.swap_in_pages_delta", capability);
        append_unavailable(sink, context, "host.vm.swap_out_pages_delta", capability);
        return;
    }
    append_counter_delta(sink, context, "host.vm.major_faults_delta",
                         state->vmstat_valid, state->vmstat.major_faults,
                         current.major_faults,
                         context->monotonic_ms > state->vmstat_monotonic_ms);
    append_counter_delta(sink, context, "host.vm.swap_in_pages_delta",
                         state->vmstat_valid, state->vmstat.swap_in_pages,
                         current.swap_in_pages,
                         context->monotonic_ms > state->vmstat_monotonic_ms);
    append_counter_delta(sink, context, "host.vm.swap_out_pages_delta",
                         state->vmstat_valid, state->vmstat.swap_out_pages,
                         current.swap_out_pages,
                         context->monotonic_ms > state->vmstat_monotonic_ms);
    state->vmstat = current;
    state->vmstat_monotonic_ms = context->monotonic_ms;
    state->vmstat_valid = true;
}

static void collect_pressure_one(linux_proc_state_t *state, size_t index,
                                 const char *name,
                                 const system_health_collect_context_t *context,
                                 system_health_observation_sink_t *sink) {
    char text[PROC_READ_MAX];
    char relative[64];
    char metric[64];
    linux_proc_pressure_t current;
    read_result_t result;
    (void)snprintf(relative, sizeof(relative), "pressure/%s", name);
    result = read_proc_file(context->proc_root, relative, text, sizeof(text));
    if (result != READ_OK || linux_proc_parse_pressure(text, &current) != 0) {
        system_health_capability_t capability = result == READ_OK
            ? SYSTEM_HEALTH_CAPABILITY_ERROR : read_capability(result);
        (void)snprintf(metric, sizeof(metric), "host.pressure.%s.some_ratio", name);
        append_unavailable(sink, context, metric, capability);
        (void)snprintf(metric, sizeof(metric), "host.pressure.%s.some_seconds_delta", name);
        append_unavailable(sink, context, metric, capability);
        if (strcmp(name, "cpu") != 0) {
            (void)snprintf(metric, sizeof(metric), "host.pressure.%s.full_ratio", name);
            append_unavailable(sink, context, metric, capability);
        }
        return;
    }
    (void)snprintf(metric, sizeof(metric), "host.pressure.%s.some_ratio", name);
    append_value(sink, context, metric, current.some_avg10_ratio,
                 SYSTEM_HEALTH_UNIT_RATIO);
    (void)snprintf(metric, sizeof(metric), "host.pressure.%s.some_seconds_delta", name);
    if (!state->pressure_valid[index]) {
        append_unavailable(sink, context, metric,
                           SYSTEM_HEALTH_CAPABILITY_STALE);
    } else if (context->monotonic_ms <= state->pressure_monotonic_ms[index] ||
               current.some_total_usec < state->pressure[index].some_total_usec) {
        append_unavailable(sink, context, metric,
                           SYSTEM_HEALTH_CAPABILITY_ERROR);
    } else {
        append_value(sink, context, metric,
                     (double)(current.some_total_usec -
                              state->pressure[index].some_total_usec) / 1000000.0,
                     SYSTEM_HEALTH_UNIT_SECONDS);
    }
    if (current.full_present) {
        (void)snprintf(metric, sizeof(metric), "host.pressure.%s.full_ratio", name);
        append_value(sink, context, metric, current.full_avg10_ratio,
                     SYSTEM_HEALTH_UNIT_RATIO);
    } else if (strcmp(name, "cpu") != 0) {
        (void)snprintf(metric, sizeof(metric), "host.pressure.%s.full_ratio", name);
        append_unavailable(sink, context, metric,
                           SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED);
    }
    state->pressure[index] = current;
    state->pressure_monotonic_ms[index] = context->monotonic_ms;
    state->pressure_valid[index] = true;
}

static int collect_proc(void *opaque,
                        const system_health_collect_context_t *context,
                        system_health_observation_sink_t *sink) {
    linux_proc_state_t *state = opaque;
    if (!state || !context || !sink || !context->proc_root) return -1;
    collect_cpu(state, context, sink);
    collect_load(context, sink);
    collect_memory(context, sink);
    collect_vmstat(state, context, sink);
    collect_pressure_one(state, 0U, "cpu", context, sink);
    collect_pressure_one(state, 1U, "memory", context, sink);
    collect_pressure_one(state, 2U, "io", context, sink);
    return 0;
}

void linux_proc_state_init(linux_proc_state_t *state) {
    if (state) memset(state, 0, sizeof(*state));
}

bool linux_proc_collector_init(system_health_collector_t *collector,
                               linux_proc_state_t *state) {
    if (!collector || !state) return false;
    linux_proc_state_init(state);
    memset(collector, 0, sizeof(*collector));
    (void)snprintf(collector->name, sizeof(collector->name), "linux_proc");
    collector->scope = SYSTEM_HEALTH_SCOPE_HOST;
    collector->tier = SYSTEM_HEALTH_TIER_FAST;
    collector->interval_seconds = 5U;
    collector->stale_after_seconds = 20U;
    collector->state = state;
    collector->collect = collect_proc;
    return true;
}
