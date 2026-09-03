#include "telemetry/collectors/linux_cgroup.h"

#include <ctype.h>
#include <errno.h>
#include <inttypes.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define CGROUP_READ_MAX 16384U
#define CGROUP_PATH_MAX 1024U

typedef enum {
    CG_READ_OK = 0,
    CG_READ_UNSUPPORTED,
    CG_READ_PERMISSION,
    CG_READ_ERROR
} cg_read_result_t;

typedef struct {
    linux_cgroup_version_t version;
    char unified[512];
    char cpu[512];
    char memory[512];
    char pids[512];
} cgroup_memberships_t;

typedef struct {
    cg_read_result_t cpu_max_status;
    linux_cgroup_cpu_max_t cpu_max;
    cg_read_result_t cpu_stat_status;
    linux_cgroup_cpu_stat_t cpu_stat;
    cg_read_result_t memory_current_status;
    uint64_t memory_current;
    cg_read_result_t memory_max_status;
    linux_cgroup_limit_t memory_max;
    cg_read_result_t oom_status;
    uint64_t oom_kills;
    cg_read_result_t pids_current_status;
    uint64_t pids_current;
    cg_read_result_t pids_max_status;
    linux_cgroup_limit_t pids_max;
} cgroup_sample_t;

static const char *skip_space(const char *text) {
    while (text && isspace((unsigned char)*text)) ++text;
    return text;
}

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

static bool only_trailing_space(const char *text) {
    text = skip_space(text);
    return text && *text == '\0';
}

int linux_cgroup_parse_limit(const char *text, linux_cgroup_limit_t *out) {
    const char *cursor;
    linux_cgroup_limit_t parsed;
    if (!text || !out) return -1;
    cursor = skip_space(text);
    memset(&parsed, 0, sizeof(parsed));
    if (strncmp(cursor, "max", 3) == 0 && only_trailing_space(cursor + 3)) {
        parsed.unlimited = true;
    } else if (!parse_u64(cursor, &cursor, &parsed.value) ||
               !only_trailing_space(cursor)) {
        return -1;
    }
    *out = parsed;
    return 0;
}

int linux_cgroup_parse_cpu_max(const char *text,
                               linux_cgroup_cpu_max_t *out) {
    const char *cursor;
    linux_cgroup_cpu_max_t parsed;
    if (!text || !out) return -1;
    cursor = skip_space(text);
    memset(&parsed, 0, sizeof(parsed));
    if (strncmp(cursor, "max", 3) == 0 &&
        isspace((unsigned char)cursor[3])) {
        parsed.quota_usec.unlimited = true;
        cursor += 3;
    } else if (!parse_u64(cursor, &cursor, &parsed.quota_usec.value)) {
        return -1;
    }
    cursor = skip_space(cursor);
    if (!parse_u64(cursor, &cursor, &parsed.period_usec) ||
        parsed.period_usec == 0 ||
        (!parsed.quota_usec.unlimited && parsed.quota_usec.value == 0) ||
        !only_trailing_space(cursor)) return -1;
    *out = parsed;
    return 0;
}

static int keyed_u64(const char *text, const char *key, uint64_t *out) {
    const char *line = text;
    size_t key_len = strlen(key);
    while (line && *line) {
        const char *next = strchr(line, '\n');
        if (strncmp(line, key, key_len) == 0 &&
            isspace((unsigned char)line[key_len])) {
            const char *cursor = skip_space(line + key_len);
            const char *end;
            if (!parse_u64(cursor, &end, out)) return -1;
            while (*end != '\n' && isspace((unsigned char)*end)) ++end;
            if (*end != '\0' && *end != '\n') return -1;
            return 0;
        }
        line = next ? next + 1 : NULL;
    }
    return 1;
}

int linux_cgroup_parse_cpu_stat(const char *text,
                                linux_cgroup_cpu_stat_t *out) {
    linux_cgroup_cpu_stat_t parsed;
    int usage;
    int periods;
    int throttled;
    int throttled_usec;
    if (!text || !out) return -1;
    memset(&parsed, 0, sizeof(parsed));
    usage = keyed_u64(text, "usage_usec", &parsed.usage_usec);
    periods = keyed_u64(text, "nr_periods", &parsed.periods);
    throttled = keyed_u64(text, "nr_throttled", &parsed.throttled_periods);
    throttled_usec = keyed_u64(text, "throttled_usec", &parsed.throttled_usec);
    if (usage != 0) return usage > 0 ? 1 : -1;
    if (periods < 0 || throttled < 0 || throttled_usec < 0) return -1;
    parsed.throttle_present = periods == 0 && throttled == 0 &&
                              throttled_usec == 0;
    if ((periods == 0) != (throttled == 0) ||
        (periods == 0) != (throttled_usec == 0)) return -1;
    /* Some older kernels omit all throttle accounting; preserve that fact. */
    *out = parsed;
    return 0;
}

int linux_cgroup_parse_memory_events(
    const char *text, linux_cgroup_memory_events_t *out) {
    linux_cgroup_memory_events_t parsed;
    int oom_kill;
    if (!text || !out) return -1;
    memset(&parsed, 0, sizeof(parsed));
    if (keyed_u64(text, "low", &parsed.low) < 0 ||
        keyed_u64(text, "high", &parsed.high) < 0 ||
        keyed_u64(text, "max", &parsed.max) < 0 ||
        keyed_u64(text, "oom", &parsed.oom) < 0) return -1;
    oom_kill = keyed_u64(text, "oom_kill", &parsed.oom_kill);
    if (oom_kill != 0) return oom_kill > 0 ? 1 : -1;
    *out = parsed;
    return 0;
}

static cg_read_result_t read_bounded(const char *path, char *buffer,
                                     size_t capacity) {
    FILE *file;
    size_t used;
    int saved_errno;
    if (!path || !buffer || capacity < 2U) return CG_READ_ERROR;
    errno = 0;
    file = fopen(path, "r");
    if (!file) {
        if (errno == ENOENT || errno == ENOTDIR) return CG_READ_UNSUPPORTED;
        if (errno == EACCES || errno == EPERM) return CG_READ_PERMISSION;
        return CG_READ_ERROR;
    }
    used = fread(buffer, 1, capacity - 1U, file);
    saved_errno = ferror(file) ? errno : 0;
    if (!feof(file) && used == capacity - 1U) saved_errno = EOVERFLOW;
    if (fclose(file) != 0 && saved_errno == 0) saved_errno = errno;
    if (saved_errno != 0) return CG_READ_ERROR;
    buffer[used] = '\0';
    return CG_READ_OK;
}

static bool safe_membership(const char *path) {
    if (!path || path[0] != '/') return false;
    if (strstr(path, "/../") || strcmp(path, "/..") == 0 ||
        strstr(path, "//")) return false;
    for (const unsigned char *cursor = (const unsigned char *)path;
         *cursor; ++cursor) {
        if (*cursor < 0x20U || *cursor == 0x7fU) return false;
    }
    return strlen(path) < 512U;
}

static void copy_membership(char *destination, size_t capacity,
                            const char *path) {
    (void)snprintf(destination, capacity, "%s", path);
}

static bool controller_list_has(const char *list, const char *controller) {
    size_t wanted = strlen(controller);
    const char *cursor = list;
    while (cursor && *cursor) {
        const char *end = strchr(cursor, ',');
        size_t length = end ? (size_t)(end - cursor) : strlen(cursor);
        if (length == wanted && strncmp(cursor, controller, wanted) == 0)
            return true;
        cursor = end ? end + 1 : NULL;
    }
    return false;
}

static int parse_memberships(const char *text, cgroup_memberships_t *out) {
    const char *line = text;
    cgroup_memberships_t parsed;
    if (!text || !out) return -1;
    memset(&parsed, 0, sizeof(parsed));
    while (line && *line) {
        const char *next = strchr(line, '\n');
        const char *first = strchr(line, ':');
        const char *second = first ? strchr(first + 1, ':') : NULL;
        size_t controllers_length;
        size_t path_length;
        char controllers[256];
        char path[512];
        if (!first || !second || (next && second >= next)) return -1;
        controllers_length = (size_t)(second - first - 1);
        path_length = next ? (size_t)(next - second - 1) : strlen(second + 1);
        if (controllers_length >= sizeof(controllers) ||
            path_length >= sizeof(path)) return -1;
        memcpy(controllers, first + 1, controllers_length);
        controllers[controllers_length] = '\0';
        memcpy(path, second + 1, path_length);
        path[path_length] = '\0';
        if (!safe_membership(path)) return -1;
        if (controllers_length == 0U) {
            parsed.version = LINUX_CGROUP_V2;
            copy_membership(parsed.unified, sizeof(parsed.unified), path);
        } else if (parsed.version != LINUX_CGROUP_V2) {
            parsed.version = LINUX_CGROUP_V1;
            if (controller_list_has(controllers, "cpu") ||
                controller_list_has(controllers, "cpuacct"))
                copy_membership(parsed.cpu, sizeof(parsed.cpu), path);
            if (controller_list_has(controllers, "memory"))
                copy_membership(parsed.memory, sizeof(parsed.memory), path);
            if (controller_list_has(controllers, "pids"))
                copy_membership(parsed.pids, sizeof(parsed.pids), path);
        }
        line = next ? next + 1 : NULL;
    }
    if (parsed.version == LINUX_CGROUP_UNKNOWN) return -1;
    *out = parsed;
    return 0;
}

static cg_read_result_t read_proc_membership(const char *proc_root,
                                             cgroup_memberships_t *out) {
    char path[CGROUP_PATH_MAX];
    char text[CGROUP_READ_MAX];
    int length = snprintf(path, sizeof(path), "%s/self/cgroup", proc_root);
    cg_read_result_t result;
    if (length < 0 || (size_t)length >= sizeof(path)) return CG_READ_ERROR;
    result = read_bounded(path, text, sizeof(text));
    if (result != CG_READ_OK) return result;
    return parse_memberships(text, out) == 0 ? CG_READ_OK : CG_READ_ERROR;
}

static bool build_path(char *out, size_t capacity, const char *root,
                       const char *controller, const char *membership,
                       const char *filename) {
    const char *relative = membership;
    int length;
    if (!root || !membership || !filename || !safe_membership(membership))
        return false;
    while (*relative == '/') ++relative;
    if (controller && *controller) {
        length = *relative
            ? snprintf(out, capacity, "%s/%s/%s/%s", root, controller,
                       relative, filename)
            : snprintf(out, capacity, "%s/%s/%s", root, controller, filename);
    } else {
        length = *relative
            ? snprintf(out, capacity, "%s/%s/%s", root, relative, filename)
            : snprintf(out, capacity, "%s/%s", root, filename);
    }
    return length >= 0 && (size_t)length < capacity;
}

static cg_read_result_t read_unified(const char *root, const char *membership,
                                     const char *filename, char *text,
                                     size_t capacity) {
    char path[CGROUP_PATH_MAX];
    if (!build_path(path, sizeof(path), root, NULL, membership, filename))
        return CG_READ_ERROR;
    return read_bounded(path, text, capacity);
}

static cg_read_result_t read_v1(const char *root, const char *membership,
                                const char *controller,
                                const char *combined_controller,
                                const char *filename, char *text,
                                size_t capacity) {
    const char *controllers[3] = {controller, combined_controller, NULL};
    cg_read_result_t strongest = CG_READ_UNSUPPORTED;
    for (size_t i = 0; i < sizeof(controllers) / sizeof(controllers[0]); ++i) {
        char path[CGROUP_PATH_MAX];
        cg_read_result_t result;
        if (!build_path(path, sizeof(path), root, controllers[i], membership,
                        filename)) return CG_READ_ERROR;
        result = read_bounded(path, text, capacity);
        if (result == CG_READ_OK) return result;
        if (result == CG_READ_PERMISSION) strongest = CG_READ_PERMISSION;
        else if (result == CG_READ_ERROR && strongest != CG_READ_PERMISSION)
            strongest = CG_READ_ERROR;
    }
    return strongest;
}

static cg_read_result_t parse_plain_u64(const char *text, uint64_t *out) {
    const char *end;
    if (!parse_u64(skip_space(text), &end, out) || !only_trailing_space(end))
        return CG_READ_ERROR;
    return CG_READ_OK;
}

static cg_read_result_t parse_limit_text(const char *text,
                                         linux_cgroup_limit_t *out) {
    return linux_cgroup_parse_limit(text, out) == 0
        ? CG_READ_OK : CG_READ_ERROR;
}

static cg_read_result_t read_v2_value(const char *root, const char *membership,
                                      const char *filename, char *text,
                                      uint64_t *out) {
    cg_read_result_t result = read_unified(root, membership, filename, text,
                                           CGROUP_READ_MAX);
    return result == CG_READ_OK ? parse_plain_u64(text, out) : result;
}

static cg_read_result_t read_v2_limit(const char *root, const char *membership,
                                      const char *filename, char *text,
                                      linux_cgroup_limit_t *out) {
    cg_read_result_t result = read_unified(root, membership, filename, text,
                                           CGROUP_READ_MAX);
    return result == CG_READ_OK ? parse_limit_text(text, out) : result;
}

static bool parent_membership(char *membership) {
    char *slash;
    size_t length;
    if (!membership || strcmp(membership, "/") == 0) return false;
    length = strlen(membership);
    while (length > 1U && membership[length - 1U] == '/')
        membership[--length] = '\0';
    slash = strrchr(membership, '/');
    if (!slash) return false;
    if (slash == membership) membership[1] = '\0';
    else *slash = '\0';
    return true;
}

static cg_read_result_t effective_v2_limit(
    const char *root, const char *membership, const char *filename,
    linux_cgroup_limit_t *out) {
    char current[512];
    char text[CGROUP_READ_MAX];
    linux_cgroup_limit_t effective = {.unlimited = true};
    bool found = false;
    copy_membership(current, sizeof(current), membership);
    do {
        linux_cgroup_limit_t candidate;
        cg_read_result_t result = read_v2_limit(root, current, filename, text,
                                                &candidate);
        if (result == CG_READ_PERMISSION || result == CG_READ_ERROR)
            return result;
        if (result == CG_READ_OK) {
            found = true;
            if (!candidate.unlimited &&
                (effective.unlimited || candidate.value < effective.value))
                effective = candidate;
        }
    } while (parent_membership(current));
    if (!found) return CG_READ_UNSUPPORTED;
    *out = effective;
    return CG_READ_OK;
}

static cg_read_result_t effective_v2_cpu_max(
    const char *root, const char *membership, linux_cgroup_cpu_max_t *out) {
    char current[512];
    char text[CGROUP_READ_MAX];
    linux_cgroup_cpu_max_t effective;
    bool found = false;
    memset(&effective, 0, sizeof(effective));
    effective.quota_usec.unlimited = true;
    copy_membership(current, sizeof(current), membership);
    do {
        linux_cgroup_cpu_max_t candidate;
        cg_read_result_t result = read_unified(root, current, "cpu.max", text,
                                               sizeof(text));
        if (result == CG_READ_PERMISSION || result == CG_READ_ERROR)
            return result;
        if (result == CG_READ_OK) {
            if (linux_cgroup_parse_cpu_max(text, &candidate) != 0)
                return CG_READ_ERROR;
            found = true;
            if (!candidate.quota_usec.unlimited &&
                (effective.quota_usec.unlimited ||
                 (long double)candidate.quota_usec.value /
                         (long double)candidate.period_usec <
                     (long double)effective.quota_usec.value /
                         (long double)effective.period_usec))
                effective = candidate;
        }
    } while (parent_membership(current));
    if (!found) return CG_READ_UNSUPPORTED;
    *out = effective;
    return CG_READ_OK;
}

static cg_read_result_t read_v1_value(const char *root, const char *membership,
                                      const char *controller,
                                      const char *combined, const char *filename,
                                      char *text, uint64_t *out) {
    cg_read_result_t result = read_v1(root, membership, controller, combined,
                                     filename, text, CGROUP_READ_MAX);
    return result == CG_READ_OK ? parse_plain_u64(text, out) : result;
}

static cg_read_result_t parse_v1_cpu_max(const char *quota_text,
                                         const char *period_text,
                                         linux_cgroup_cpu_max_t *out) {
    char *end;
    long long quota;
    uint64_t period;
    linux_cgroup_cpu_max_t parsed;
    errno = 0;
    quota = strtoll(skip_space(quota_text), &end, 10);
    if (errno == ERANGE || end == skip_space(quota_text) ||
        !only_trailing_space(end)) return CG_READ_ERROR;
    if (parse_plain_u64(period_text, &period) != CG_READ_OK || period == 0)
        return CG_READ_ERROR;
    memset(&parsed, 0, sizeof(parsed));
    if (quota == -1) parsed.quota_usec.unlimited = true;
    else if (quota < 0) return CG_READ_ERROR;
    else if (quota == 0) return CG_READ_ERROR;
    else parsed.quota_usec.value = (uint64_t)quota;
    parsed.period_usec = period;
    *out = parsed;
    return CG_READ_OK;
}

static void sample_v2(const char *root, const cgroup_memberships_t *memberships,
                      cgroup_sample_t *sample) {
    char first[CGROUP_READ_MAX];
    char second[CGROUP_READ_MAX];
    linux_cgroup_memory_events_t events;
    sample->cpu_max_status = effective_v2_cpu_max(
        root, memberships->unified, &sample->cpu_max);
    sample->cpu_stat_status = read_unified(root, memberships->unified, "cpu.stat",
                                           first, sizeof(first));
    if (sample->cpu_stat_status == CG_READ_OK &&
        linux_cgroup_parse_cpu_stat(first, &sample->cpu_stat) != 0)
        sample->cpu_stat_status = CG_READ_ERROR;
    sample->memory_current_status = read_v2_value(
        root, memberships->unified, "memory.current", first,
        &sample->memory_current);
    sample->memory_max_status = effective_v2_limit(
        root, memberships->unified, "memory.max", &sample->memory_max);
    sample->oom_status = read_unified(root, memberships->unified, "memory.events",
                                      second, sizeof(second));
    if (sample->oom_status == CG_READ_OK) {
        int rc = linux_cgroup_parse_memory_events(second, &events);
        if (rc == 0) sample->oom_kills = events.oom_kill;
        else sample->oom_status = rc > 0 ? CG_READ_UNSUPPORTED : CG_READ_ERROR;
    }
    sample->pids_current_status = read_v2_value(
        root, memberships->unified, "pids.current", first,
        &sample->pids_current);
    sample->pids_max_status = effective_v2_limit(
        root, memberships->unified, "pids.max", &sample->pids_max);
}

static cg_read_result_t read_v1_limit(const char *root, const char *membership,
                                      const char *controller,
                                      const char *filename, char *text,
                                      linux_cgroup_limit_t *out) {
    cg_read_result_t result = read_v1(root, membership, controller, NULL,
                                     filename, text, CGROUP_READ_MAX);
    if (result != CG_READ_OK) return result;
    result = parse_limit_text(text, out);
    if (result == CG_READ_OK && out->value >= (UINT64_C(1) << 60)) {
        out->unlimited = true;
        out->value = 0;
    }
    return result;
}

static cg_read_result_t effective_v1_limit(
    const char *root, const char *membership, const char *controller,
    const char *filename, linux_cgroup_limit_t *out) {
    char current[512];
    char text[CGROUP_READ_MAX];
    linux_cgroup_limit_t effective = {.unlimited = true};
    bool found = false;
    copy_membership(current, sizeof(current), membership);
    do {
        linux_cgroup_limit_t candidate;
        cg_read_result_t result = read_v1_limit(
            root, current, controller, filename, text, &candidate);
        if (result == CG_READ_PERMISSION || result == CG_READ_ERROR)
            return result;
        if (result == CG_READ_OK) {
            found = true;
            if (!candidate.unlimited &&
                (effective.unlimited || candidate.value < effective.value))
                effective = candidate;
        }
    } while (parent_membership(current));
    if (!found) return CG_READ_UNSUPPORTED;
    *out = effective;
    return CG_READ_OK;
}

static cg_read_result_t effective_v1_cpu_max(
    const char *root, const char *membership, linux_cgroup_cpu_max_t *out) {
    char current[512];
    char quota_text[CGROUP_READ_MAX];
    char period_text[CGROUP_READ_MAX];
    linux_cgroup_cpu_max_t effective;
    bool found = false;
    memset(&effective, 0, sizeof(effective));
    effective.quota_usec.unlimited = true;
    copy_membership(current, sizeof(current), membership);
    do {
        cg_read_result_t quota_status = read_v1(
            root, current, "cpu", "cpu,cpuacct", "cpu.cfs_quota_us",
            quota_text, sizeof(quota_text));
        cg_read_result_t period_status = read_v1(
            root, current, "cpu", "cpu,cpuacct", "cpu.cfs_period_us",
            period_text, sizeof(period_text));
        if (quota_status == CG_READ_PERMISSION ||
            period_status == CG_READ_PERMISSION) return CG_READ_PERMISSION;
        if (quota_status == CG_READ_ERROR || period_status == CG_READ_ERROR)
            return CG_READ_ERROR;
        if (quota_status == CG_READ_OK && period_status == CG_READ_OK) {
            linux_cgroup_cpu_max_t candidate;
            if (parse_v1_cpu_max(quota_text, period_text, &candidate) != CG_READ_OK)
                return CG_READ_ERROR;
            found = true;
            if (!candidate.quota_usec.unlimited &&
                (effective.quota_usec.unlimited ||
                 (long double)candidate.quota_usec.value /
                         (long double)candidate.period_usec <
                     (long double)effective.quota_usec.value /
                         (long double)effective.period_usec))
                effective = candidate;
        } else if (quota_status != period_status) {
            return CG_READ_ERROR;
        }
    } while (parent_membership(current));
    if (!found) return CG_READ_UNSUPPORTED;
    *out = effective;
    return CG_READ_OK;
}

static void sample_v1(const char *root, const cgroup_memberships_t *memberships,
                      cgroup_sample_t *sample) {
    char first[CGROUP_READ_MAX];
    char second[CGROUP_READ_MAX];
    uint64_t cpu_usage_ns;
    sample->cpu_max_status = effective_v1_cpu_max(
        root, memberships->cpu, &sample->cpu_max);
    sample->cpu_stat_status = read_v1_value(
        root, memberships->cpu, "cpuacct", "cpu,cpuacct", "cpuacct.usage",
        first, &cpu_usage_ns);
    if (sample->cpu_stat_status == CG_READ_OK) {
        sample->cpu_stat.usage_usec = cpu_usage_ns / 1000U;
        cg_read_result_t stat_result = read_v1(
            root, memberships->cpu, "cpu", "cpu,cpuacct", "cpu.stat", second,
            sizeof(second));
        if (stat_result == CG_READ_OK) {
            int periods = keyed_u64(second, "nr_periods",
                                    &sample->cpu_stat.periods);
            int throttled = keyed_u64(second, "nr_throttled",
                &sample->cpu_stat.throttled_periods);
            int time = keyed_u64(second, "throttled_time",
                                 &sample->cpu_stat.throttled_usec);
            if (periods < 0 || throttled < 0 || time < 0)
                sample->cpu_stat_status = CG_READ_ERROR;
            else {
                sample->cpu_stat.throttle_present =
                    periods == 0 && throttled == 0 && time == 0;
                if ((periods == 0) != (throttled == 0) ||
                    (periods == 0) != (time == 0))
                    sample->cpu_stat_status = CG_READ_ERROR;
                sample->cpu_stat.throttled_usec /= 1000U;
            }
        } else if (stat_result == CG_READ_PERMISSION ||
                   stat_result == CG_READ_ERROR) {
            sample->cpu_stat_status = stat_result;
        }
    }
    sample->memory_current_status = read_v1_value(
        root, memberships->memory, "memory", NULL, "memory.usage_in_bytes",
        first, &sample->memory_current);
    sample->memory_max_status = effective_v1_limit(
        root, memberships->memory, "memory", "memory.limit_in_bytes",
        &sample->memory_max);
    sample->oom_status = read_v1_value(
        root, memberships->memory, "memory", NULL, "memory.failcnt", first,
        &sample->oom_kills);
    sample->pids_current_status = read_v1_value(
        root, memberships->pids, "pids", NULL, "pids.current", first,
        &sample->pids_current);
    sample->pids_max_status = effective_v1_limit(
        root, memberships->pids, "pids", "pids.max", &sample->pids_max);
}

static system_health_capability_t capability(cg_read_result_t result) {
    switch (result) {
        case CG_READ_OK: return SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
        case CG_READ_UNSUPPORTED: return SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
        case CG_READ_PERMISSION: return SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED;
        case CG_READ_ERROR: return SYSTEM_HEALTH_CAPABILITY_ERROR;
    }
    return SYSTEM_HEALTH_CAPABILITY_ERROR;
}

static void init_observation(system_health_observation_t *observation,
                             const system_health_collect_context_t *context,
                             system_health_scope_t scope, const char *metric) {
    memset(observation, 0, sizeof(*observation));
    (void)snprintf(observation->metric, sizeof(observation->metric), "%s", metric);
    (void)snprintf(observation->resource_id,
                   sizeof(observation->resource_id), "self");
    observation->scope = scope;
    observation->sampled_monotonic_ms = context->monotonic_ms;
    observation->observed_wall_time_ms = context->wall_time_ms;
}

static void append_value(system_health_observation_sink_t *sink,
                         const system_health_collect_context_t *context,
                         system_health_scope_t scope, const char *metric,
                         double value, system_health_unit_t unit) {
    system_health_observation_t observation;
    init_observation(&observation, context, scope, metric);
    system_health_observation_set_available(&observation, value, unit);
    (void)system_health_observation_sink_append(sink, &observation);
}

static void append_unavailable(system_health_observation_sink_t *sink,
                               const system_health_collect_context_t *context,
                               system_health_scope_t scope, const char *metric,
                               system_health_capability_t cap) {
    system_health_observation_t observation;
    init_observation(&observation, context, scope, metric);
    system_health_observation_set_unavailable(&observation, cap);
    (void)system_health_observation_sink_append(sink, &observation);
}

static const char *scope_prefix(system_health_scope_t scope) {
    return scope == SYSTEM_HEALTH_SCOPE_CONTAINER ? "container" : "host";
}

static void metric_name(char *out, size_t capacity,
                        system_health_scope_t scope, const char *suffix) {
    (void)snprintf(out, capacity, "%s.%s", scope_prefix(scope), suffix);
}

static bool finite_limit(cg_read_result_t status,
                         const linux_cgroup_limit_t *limit) {
    return status == CG_READ_OK && !limit->unlimited && limit->value > 0;
}

static void emit_cpu(linux_cgroup_state_t *state, const cgroup_sample_t *sample,
                     linux_cgroup_version_t version,
                     system_health_scope_t scope,
                     const system_health_collect_context_t *context,
                     system_health_observation_sink_t *sink) {
    char metric[64];
    bool quota_finite = sample->cpu_max_status == CG_READ_OK &&
                        !sample->cpu_max.quota_usec.unlimited;
    metric_name(metric, sizeof(metric), scope, "cpu.quota_cores");
    if (quota_finite) {
        append_value(sink, context, scope, metric,
                     (double)sample->cpu_max.quota_usec.value /
                         (double)sample->cpu_max.period_usec,
                     SYSTEM_HEALTH_UNIT_COUNT);
    } else {
        append_unavailable(sink, context, scope, metric,
            sample->cpu_max_status == CG_READ_OK
                ? SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED
                : capability(sample->cpu_max_status));
    }
    metric_name(metric, sizeof(metric), scope, "cpu.usage_ratio");
    if (sample->cpu_stat_status != CG_READ_OK ||
        sample->cpu_max_status != CG_READ_OK) {
        append_unavailable(sink, context, scope, metric,
            capability(sample->cpu_stat_status != CG_READ_OK
                ? sample->cpu_stat_status : sample->cpu_max_status));
    } else if (!quota_finite) {
        append_unavailable(sink, context, scope, metric,
                           SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED);
    } else if (!state->cpu_valid || state->version != version) {
        append_unavailable(sink, context, scope, metric,
                           SYSTEM_HEALTH_CAPABILITY_STALE);
    } else if (context->monotonic_ms <= state->sampled_monotonic_ms ||
               sample->cpu_stat.usage_usec < state->cpu.usage_usec) {
        append_unavailable(sink, context, scope, metric,
                           SYSTEM_HEALTH_CAPABILITY_ERROR);
    } else {
        uint64_t elapsed_ms = context->monotonic_ms -
                              state->sampled_monotonic_ms;
        uint64_t elapsed_usec;
        if (elapsed_ms > UINT64_MAX / 1000U) {
            append_unavailable(sink, context, scope, metric,
                               SYSTEM_HEALTH_CAPABILITY_ERROR);
            goto usage_done;
        }
        elapsed_usec = elapsed_ms * 1000U;
        double quota_cores = (double)sample->cpu_max.quota_usec.value /
                             (double)sample->cpu_max.period_usec;
        double ratio = elapsed_usec == 0 ? 0.0
            : (double)(sample->cpu_stat.usage_usec - state->cpu.usage_usec) /
              ((double)elapsed_usec * quota_cores);
        append_value(sink, context, scope, metric, ratio,
                     SYSTEM_HEALTH_UNIT_RATIO);
    }
usage_done:
    metric_name(metric, sizeof(metric), scope, "cpu.throttled_ratio");
    if (sample->cpu_stat_status != CG_READ_OK) {
        append_unavailable(sink, context, scope, metric,
                           capability(sample->cpu_stat_status));
    } else if (!sample->cpu_stat.throttle_present) {
        append_unavailable(sink, context, scope, metric,
                           SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED);
    } else if (!state->cpu_valid || state->version != version) {
        append_unavailable(sink, context, scope, metric,
                           SYSTEM_HEALTH_CAPABILITY_STALE);
    } else if (sample->cpu_stat.periods < state->cpu.periods ||
               sample->cpu_stat.throttled_periods <
                   state->cpu.throttled_periods) {
        append_unavailable(sink, context, scope, metric,
                           SYSTEM_HEALTH_CAPABILITY_ERROR);
    } else {
        uint64_t periods = sample->cpu_stat.periods - state->cpu.periods;
        uint64_t throttled = sample->cpu_stat.throttled_periods -
                             state->cpu.throttled_periods;
        append_value(sink, context, scope, metric,
                     periods == 0 ? 0.0 : (double)throttled / (double)periods,
                     SYSTEM_HEALTH_UNIT_RATIO);
    }
}

static void emit_memory(linux_cgroup_state_t *state,
                        const cgroup_sample_t *sample,
                        linux_cgroup_version_t version,
                        system_health_scope_t scope,
                        const system_health_collect_context_t *context,
                        system_health_observation_sink_t *sink) {
    char metric[64];
    bool limit_finite = finite_limit(sample->memory_max_status,
                                     &sample->memory_max);
    const char *names[] = {"memory.current_bytes", "memory.limit_bytes",
                           "memory.available_bytes", "memory.available_ratio"};
    cg_read_result_t combined_status = sample->memory_current_status != CG_READ_OK
        ? sample->memory_current_status : sample->memory_max_status;
    for (size_t i = 0; i < sizeof(names) / sizeof(names[0]); ++i) {
        metric_name(metric, sizeof(metric), scope, names[i]);
        if (i == 0U && sample->memory_current_status == CG_READ_OK) {
            append_value(sink, context, scope, metric,
                         (double)sample->memory_current,
                         SYSTEM_HEALTH_UNIT_BYTES);
        } else if (limit_finite) {
            uint64_t headroom = sample->memory_current >= sample->memory_max.value
                ? 0 : sample->memory_max.value - sample->memory_current;
            double value = i == 1U ? (double)sample->memory_max.value
                         : i == 2U ? (double)headroom
                         : (double)headroom / (double)sample->memory_max.value;
            append_value(sink, context, scope, metric, value,
                i == 3U ? SYSTEM_HEALTH_UNIT_RATIO : SYSTEM_HEALTH_UNIT_BYTES);
        } else {
            append_unavailable(sink, context, scope, metric,
                combined_status == CG_READ_OK
                    ? SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED
                    : capability(combined_status));
        }
    }
    metric_name(metric, sizeof(metric), scope, "memory.oom_kills_delta");
    if (sample->oom_status != CG_READ_OK) {
        append_unavailable(sink, context, scope, metric,
                           capability(sample->oom_status));
    } else if (!state->oom_valid || state->version != version) {
        append_unavailable(sink, context, scope, metric,
                           SYSTEM_HEALTH_CAPABILITY_STALE);
    } else if (sample->oom_kills < state->oom_kills) {
        append_unavailable(sink, context, scope, metric,
                           SYSTEM_HEALTH_CAPABILITY_ERROR);
    } else {
        append_value(sink, context, scope, metric,
                     (double)(sample->oom_kills - state->oom_kills),
                     SYSTEM_HEALTH_UNIT_COUNT);
    }
}

static void emit_pids(const cgroup_sample_t *sample,
                      system_health_scope_t scope,
                      const system_health_collect_context_t *context,
                      system_health_observation_sink_t *sink) {
    char metric[64];
    bool limit_finite = finite_limit(sample->pids_max_status, &sample->pids_max);
    metric_name(metric, sizeof(metric), scope, "pids.current");
    if (sample->pids_current_status == CG_READ_OK)
        append_value(sink, context, scope, metric,
                     (double)sample->pids_current, SYSTEM_HEALTH_UNIT_COUNT);
    else
        append_unavailable(sink, context, scope, metric,
                           capability(sample->pids_current_status));
    metric_name(metric, sizeof(metric), scope, "pids.limit");
    if (limit_finite)
        append_value(sink, context, scope, metric,
                     (double)sample->pids_max.value, SYSTEM_HEALTH_UNIT_COUNT);
    else
        append_unavailable(sink, context, scope, metric,
            sample->pids_max_status == CG_READ_OK
                ? SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED
                : capability(sample->pids_max_status));
    metric_name(metric, sizeof(metric), scope, "pids.available_ratio");
    if (limit_finite && sample->pids_current_status == CG_READ_OK) {
        uint64_t remaining = sample->pids_current >= sample->pids_max.value
            ? 0 : sample->pids_max.value - sample->pids_current;
        append_value(sink, context, scope, metric,
                     (double)remaining / (double)sample->pids_max.value,
                     SYSTEM_HEALTH_UNIT_RATIO);
    } else {
        cg_read_result_t status = sample->pids_current_status != CG_READ_OK
            ? sample->pids_current_status : sample->pids_max_status;
        append_unavailable(sink, context, scope, metric,
            status == CG_READ_OK ? SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED
                                 : capability(status));
    }
}

static int collect_cgroup(void *opaque,
                          const system_health_collect_context_t *context,
                          system_health_observation_sink_t *sink) {
    linux_cgroup_state_t *state = opaque;
    cgroup_memberships_t memberships;
    cgroup_sample_t sample;
    cg_read_result_t membership_status;
    bool constrained;
    system_health_scope_t scope;
    if (!state || !context || !sink || !context->proc_root ||
        !context->cgroup_root) return -1;
    memset(&sample, 0, sizeof(sample));
    membership_status = read_proc_membership(context->proc_root, &memberships);
    if (membership_status != CG_READ_OK) {
        system_health_capability_t cap = capability(membership_status);
        append_unavailable(sink, context, SYSTEM_HEALTH_SCOPE_CONTAINER,
                           "container.cgroup.discovery", cap);
        return 0;
    }
    if (memberships.version == LINUX_CGROUP_V2)
        sample_v2(context->cgroup_root, &memberships, &sample);
    else
        sample_v1(context->cgroup_root, &memberships, &sample);
    constrained = (sample.cpu_max_status == CG_READ_OK &&
                   !sample.cpu_max.quota_usec.unlimited) ||
                  finite_limit(sample.memory_max_status, &sample.memory_max) ||
                  finite_limit(sample.pids_max_status, &sample.pids_max);
    scope = constrained ? SYSTEM_HEALTH_SCOPE_CONTAINER
                        : SYSTEM_HEALTH_SCOPE_HOST;
    emit_cpu(state, &sample, memberships.version, scope, context, sink);
    emit_memory(state, &sample, memberships.version, scope, context, sink);
    emit_pids(&sample, scope, context, sink);
    if (state->version != memberships.version) {
        state->cpu_valid = false;
        state->oom_valid = false;
    }
    if (sample.cpu_stat_status == CG_READ_OK) {
        state->cpu = sample.cpu_stat;
        state->sampled_monotonic_ms = context->monotonic_ms;
    } else {
        state->cpu_valid = false;
    }
    if (sample.oom_status == CG_READ_OK) state->oom_kills = sample.oom_kills;
    else state->oom_valid = false;
    state->version = memberships.version;
    if (sample.cpu_stat_status == CG_READ_OK) state->cpu_valid = true;
    if (sample.oom_status == CG_READ_OK) state->oom_valid = true;
    return 0;
}

void linux_cgroup_state_init(linux_cgroup_state_t *state) {
    if (state) memset(state, 0, sizeof(*state));
}

bool linux_cgroup_collector_init(system_health_collector_t *collector,
                                 linux_cgroup_state_t *state) {
    if (!collector || !state) return false;
    linux_cgroup_state_init(state);
    memset(collector, 0, sizeof(*collector));
    (void)snprintf(collector->name, sizeof(collector->name), "linux_cgroup");
    collector->scope = SYSTEM_HEALTH_SCOPE_CONTAINER;
    collector->tier = SYSTEM_HEALTH_TIER_FAST;
    collector->interval_seconds = 5U;
    collector->stale_after_seconds = 20U;
    collector->state = state;
    collector->collect = collect_cgroup;
    return true;
}
