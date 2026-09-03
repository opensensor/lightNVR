#include "telemetry/collectors/linux_process.h"

#include <ctype.h>
#include <dirent.h>
#include <errno.h>
#include <stdio.h>
#include <string.h>
#include <sys/resource.h>

#define PROCESS_STATUS_MAX_BYTES (64U * 1024U)
#define PROCESS_SMALL_FILE_MAX_BYTES 128U

static system_health_capability_t capability_for_errno(int error_code) {
    if (error_code == EACCES || error_code == EPERM) {
        return SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED;
    }
    return SYSTEM_HEALTH_CAPABILITY_ERROR;
}

static void value_unavailable(linux_process_value_t *value,
                              system_health_capability_t capability) {
    value->value = 0;
    value->capability = capability;
}

static void value_available(linux_process_value_t *value, uint64_t number) {
    value->value = number;
    value->capability = SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
}

static bool parse_decimal(const char *begin, const char *end,
                          uint64_t *value_out, const char **after_out) {
    const char *cursor = begin;
    uint64_t value = 0;
    bool found = false;

    while (cursor < end && isspace((unsigned char)*cursor)) cursor++;
    while (cursor < end && isdigit((unsigned char)*cursor)) {
        unsigned digit = (unsigned)(*cursor - '0');
        if (value > (UINT64_MAX - digit) / 10U) return false;
        value = value * 10U + digit;
        cursor++;
        found = true;
    }
    if (!found) return false;
    *value_out = value;
    if (after_out) *after_out = cursor;
    return true;
}

static bool remaining_space(const char *cursor, const char *end) {
    while (cursor < end) {
        if (!isspace((unsigned char)*cursor)) return false;
        cursor++;
    }
    return true;
}

int linux_process_parse_status_text(const char *text, size_t text_length,
                                    linux_process_sample_t *sample) {
    if (!text || !sample) return -1;

    value_unavailable(&sample->rss_bytes,
                      SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED);
    value_unavailable(&sample->thread_count,
                      SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED);

    size_t offset = 0;
    bool rss_seen = false;
    bool threads_seen = false;
    while (offset < text_length) {
        const char *line = text + offset;
        const char *line_end = memchr(line, '\n', text_length - offset);
        if (!line_end) line_end = text + text_length;

        if ((size_t)(line_end - line) >= 6U &&
            memcmp(line, "VmRSS:", 6U) == 0) {
            uint64_t kibibytes = 0;
            const char *after = NULL;
            bool valid = !rss_seen &&
                parse_decimal(line + 6, line_end, &kibibytes, &after);
            while (valid && after < line_end &&
                   isspace((unsigned char)*after)) after++;
            valid = valid && (size_t)(line_end - after) >= 2U &&
                    after[0] == 'k' && after[1] == 'B' &&
                    remaining_space(after + 2, line_end) &&
                    kibibytes <= UINT64_MAX / 1024U;
            if (valid) value_available(&sample->rss_bytes, kibibytes * 1024U);
            else value_unavailable(&sample->rss_bytes,
                                   SYSTEM_HEALTH_CAPABILITY_ERROR);
            rss_seen = true;
        } else if ((size_t)(line_end - line) >= 8U &&
                   memcmp(line, "Threads:", 8U) == 0) {
            uint64_t threads = 0;
            const char *after = NULL;
            bool valid = !threads_seen &&
                parse_decimal(line + 8, line_end, &threads, &after) &&
                remaining_space(after, line_end);
            if (valid) value_available(&sample->thread_count, threads);
            else value_unavailable(&sample->thread_count,
                                   SYSTEM_HEALTH_CAPABILITY_ERROR);
            threads_seen = true;
        }

        offset = (size_t)(line_end - text);
        if (offset < text_length) offset++;
    }
    return 0;
}

static int read_status(const char *path, linux_process_sample_t *sample) {
    if (!path) {
        value_unavailable(&sample->rss_bytes,
                          SYSTEM_HEALTH_CAPABILITY_ERROR);
        value_unavailable(&sample->thread_count,
                          SYSTEM_HEALTH_CAPABILITY_ERROR);
        return -1;
    }

    FILE *input = fopen(path, "rb");
    if (!input) {
        system_health_capability_t capability = capability_for_errno(errno);
        value_unavailable(&sample->rss_bytes, capability);
        value_unavailable(&sample->thread_count, capability);
        return -1;
    }

    char contents[PROCESS_STATUS_MAX_BYTES + 1U];
    size_t length = fread(contents, 1U, PROCESS_STATUS_MAX_BYTES + 1U, input);
    int saved_errno = errno;
    bool failed = ferror(input) != 0 || length > PROCESS_STATUS_MAX_BYTES;
    fclose(input);
    if (failed) {
        system_health_capability_t capability = length > PROCESS_STATUS_MAX_BYTES
            ? SYSTEM_HEALTH_CAPABILITY_ERROR
            : capability_for_errno(saved_errno);
        value_unavailable(&sample->rss_bytes, capability);
        value_unavailable(&sample->thread_count, capability);
        return -1;
    }
    return linux_process_parse_status_text(contents, length, sample);
}

static int count_open_fds(const char *path, size_t scan_limit,
                          linux_process_value_t *value) {
    if (!path) {
        value_unavailable(value, SYSTEM_HEALTH_CAPABILITY_ERROR);
        return -1;
    }
    DIR *directory = opendir(path);
    if (!directory) {
        value_unavailable(value, capability_for_errno(errno));
        return -1;
    }

    uint64_t count = 0;
    bool exceeded = false;
    errno = 0;
    struct dirent *entry;
    while ((entry = readdir(directory)) != NULL) {
        if (strcmp(entry->d_name, ".") == 0 ||
            strcmp(entry->d_name, "..") == 0) continue;
        if (count >= scan_limit) {
            exceeded = true;
            break;
        }
        count++;
    }
    int saved_errno = errno;
    closedir(directory);
    if (exceeded || saved_errno != 0) {
        value_unavailable(value, saved_errno != 0
            ? capability_for_errno(saved_errno)
            : SYSTEM_HEALTH_CAPABILITY_ERROR);
        return -1;
    }
    value_available(value, count);
    return 0;
}

static int read_pid_limit(const char *path, linux_process_value_t *value,
                          bool *unlimited) {
    *unlimited = false;
    if (!path) {
        value_unavailable(value, SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED);
        return -1;
    }
    FILE *input = fopen(path, "rb");
    if (!input) {
        system_health_capability_t capability =
            (errno == ENOENT || errno == ENOTDIR)
                ? SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED
                : capability_for_errno(errno);
        value_unavailable(value, capability);
        return -1;
    }
    char contents[PROCESS_SMALL_FILE_MAX_BYTES + 1U];
    size_t length = fread(contents, 1U, PROCESS_SMALL_FILE_MAX_BYTES + 1U,
                          input);
    int saved_errno = errno;
    bool failed = ferror(input) != 0 || length > PROCESS_SMALL_FILE_MAX_BYTES;
    fclose(input);
    if (failed) {
        value_unavailable(value, length > PROCESS_SMALL_FILE_MAX_BYTES
            ? SYSTEM_HEALTH_CAPABILITY_ERROR
            : capability_for_errno(saved_errno));
        return -1;
    }

    const char *begin = contents;
    const char *end = contents + length;
    while (begin < end && isspace((unsigned char)*begin)) begin++;
    const char *trimmed_end = end;
    while (trimmed_end > begin &&
           isspace((unsigned char)trimmed_end[-1])) trimmed_end--;
    if ((size_t)(trimmed_end - begin) == 3U &&
        memcmp(begin, "max", 3U) == 0) {
        *unlimited = true;
        value_unavailable(value, SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED);
        return 0;
    }
    uint64_t parsed = 0;
    const char *after = NULL;
    if (!parse_decimal(begin, trimmed_end, &parsed, &after) ||
        after != trimmed_end) {
        value_unavailable(value, SYSTEM_HEALTH_CAPABILITY_ERROR);
        return -1;
    }
    value_available(value, parsed);
    return 0;
}

static void read_rlimit_value(int resource, bool supplied, bool unlimited,
                              uint64_t supplied_value,
                              linux_process_value_t *value,
                              bool *is_unlimited) {
    *is_unlimited = false;
    if (supplied) {
        if (unlimited) {
            *is_unlimited = true;
            value_unavailable(value, SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED);
        } else {
            value_available(value, supplied_value);
        }
        return;
    }

    struct rlimit limit;
    if (getrlimit(resource, &limit) != 0) {
        value_unavailable(value, capability_for_errno(errno));
    } else if (limit.rlim_cur == RLIM_INFINITY) {
        *is_unlimited = true;
        value_unavailable(value, SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED);
    } else {
        value_available(value, (uint64_t)limit.rlim_cur);
    }
}

static void choose_effective_pid_limit(
    const linux_process_value_t *nproc, bool nproc_unlimited,
    const linux_process_value_t *cgroup, bool cgroup_unlimited,
    linux_process_value_t *effective) {
    if (nproc->capability == SYSTEM_HEALTH_CAPABILITY_AVAILABLE &&
        cgroup->capability == SYSTEM_HEALTH_CAPABILITY_AVAILABLE) {
        value_available(effective,
                        nproc->value < cgroup->value
                            ? nproc->value : cgroup->value);
        return;
    }
    if (cgroup->capability == SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED ||
        cgroup->capability == SYSTEM_HEALTH_CAPABILITY_ERROR) {
        value_unavailable(effective, cgroup->capability);
        return;
    }
    if (nproc->capability == SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED ||
        nproc->capability == SYSTEM_HEALTH_CAPABILITY_ERROR) {
        value_unavailable(effective, nproc->capability);
        return;
    }
    if (nproc->capability == SYSTEM_HEALTH_CAPABILITY_AVAILABLE) {
        value_available(effective, nproc->value);
        return;
    }
    if (cgroup->capability == SYSTEM_HEALTH_CAPABILITY_AVAILABLE) {
        value_available(effective, cgroup->value);
        return;
    }
    (void)nproc_unlimited;
    (void)cgroup_unlimited;
    value_unavailable(effective, SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED);
}

int linux_process_sample(const linux_process_sample_request_t *request,
                         linux_process_sample_t *sample) {
    if (!request || !sample) return -1;
    memset(sample, 0, sizeof(*sample));

    read_status(request->status_path, sample);
    count_open_fds(request->fd_directory_path,
                   request->fd_scan_limit != 0U
                       ? request->fd_scan_limit
                       : LINUX_PROCESS_DEFAULT_FD_SCAN_LIMIT,
                   &sample->open_fd_count);

    bool fd_unlimited = false;
    read_rlimit_value(RLIMIT_NOFILE, request->nofile_limit_supplied,
                      request->nofile_limit_unlimited,
                      request->nofile_limit, &sample->effective_fd_limit,
                      &fd_unlimited);
    (void)fd_unlimited;

    linux_process_value_t nproc;
    bool nproc_unlimited = false;
#ifdef RLIMIT_NPROC
    read_rlimit_value(RLIMIT_NPROC, request->nproc_limit_supplied,
                      request->nproc_limit_unlimited, request->nproc_limit,
                      &nproc, &nproc_unlimited);
#else
    (void)request;
    value_unavailable(&nproc, SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED);
    nproc_unlimited = true;
#endif
    linux_process_value_t cgroup;
    bool cgroup_unlimited = false;
    read_pid_limit(request->pids_max_path, &cgroup, &cgroup_unlimited);
    choose_effective_pid_limit(&nproc, nproc_unlimited, &cgroup,
                               cgroup_unlimited,
                               &sample->effective_pid_limit);
    return 0;
}

static bool make_path(char *output, size_t output_size, const char *root,
                      const char *suffix) {
    int length = snprintf(output, output_size, "%s/%s", root, suffix);
    return length >= 0 && (size_t)length < output_size;
}

static void prepare_observation(system_health_observation_t *observation,
                                const char *metric,
                                const system_health_collect_context_t *context) {
    memset(observation, 0, sizeof(*observation));
    snprintf(observation->metric, sizeof(observation->metric), "%s", metric);
    snprintf(observation->resource_id, sizeof(observation->resource_id),
             "%s", "lightnvr");
    observation->scope = SYSTEM_HEALTH_SCOPE_PROCESS;
    observation->sampled_monotonic_ms = context->monotonic_ms;
    observation->observed_wall_time_ms = context->wall_time_ms;
}

static void emit_value(system_health_observation_sink_t *sink,
                       const system_health_collect_context_t *context,
                       const char *metric, const linux_process_value_t *value,
                       system_health_unit_t unit) {
    system_health_observation_t observation;
    prepare_observation(&observation, metric, context);
    if (value->capability == SYSTEM_HEALTH_CAPABILITY_AVAILABLE) {
        system_health_observation_set_available(&observation,
                                                (double)value->value, unit);
    } else {
        system_health_observation_set_unavailable(&observation,
                                                  value->capability);
    }
    system_health_observation_sink_append(sink, &observation);
}

static void emit_ratio(system_health_observation_sink_t *sink,
                       const system_health_collect_context_t *context,
                       const char *metric, const linux_process_value_t *used,
                       const linux_process_value_t *limit) {
    system_health_observation_t observation;
    prepare_observation(&observation, metric, context);
    if (used->capability != SYSTEM_HEALTH_CAPABILITY_AVAILABLE) {
        system_health_observation_set_unavailable(&observation,
                                                  used->capability);
    } else if (limit->capability != SYSTEM_HEALTH_CAPABILITY_AVAILABLE) {
        system_health_observation_set_unavailable(&observation,
                                                  limit->capability);
    } else {
        double ratio = limit->value == 0U
            ? (used->value == 0U ? 0.0 : 1.0)
            : (double)used->value / (double)limit->value;
        system_health_observation_set_available(&observation, ratio,
                                                SYSTEM_HEALTH_UNIT_RATIO);
    }
    system_health_observation_sink_append(sink, &observation);
}

int linux_process_collect(void *state,
                          const system_health_collect_context_t *context,
                          system_health_observation_sink_t *sink) {
    if (!context || !sink) return -1;
    linux_process_collector_state_t *configuration = state;
    char status_path[LINUX_PROCESS_PATH_LENGTH];
    char fd_path[LINUX_PROCESS_PATH_LENGTH];
    char pids_path[LINUX_PROCESS_PATH_LENGTH];
    const char *proc_root = context->proc_root ? context->proc_root : "/proc";
    const char *cgroup_root = context->cgroup_root
        ? context->cgroup_root : "/sys/fs/cgroup";

    const char *status = configuration && configuration->status_path[0]
        ? configuration->status_path
        : (make_path(status_path, sizeof(status_path), proc_root, "self/status")
               ? status_path : NULL);
    const char *fds = configuration && configuration->fd_directory_path[0]
        ? configuration->fd_directory_path
        : (make_path(fd_path, sizeof(fd_path), proc_root, "self/fd")
               ? fd_path : NULL);
    const char *pids = configuration && configuration->pids_max_path[0]
        ? configuration->pids_max_path
        : (make_path(pids_path, sizeof(pids_path), cgroup_root, "pids.max")
               ? pids_path : NULL);

    linux_process_sample_request_t request;
    memset(&request, 0, sizeof(request));
    request.status_path = status;
    request.fd_directory_path = fds;
    request.pids_max_path = pids;
    request.fd_scan_limit = configuration && configuration->fd_scan_limit
        ? configuration->fd_scan_limit : LINUX_PROCESS_DEFAULT_FD_SCAN_LIMIT;

    linux_process_sample_t sample;
    linux_process_sample(&request, &sample);
    emit_value(sink, context, "process.rss_bytes", &sample.rss_bytes,
               SYSTEM_HEALTH_UNIT_BYTES);
    emit_value(sink, context, "process.threads", &sample.thread_count,
               SYSTEM_HEALTH_UNIT_COUNT);
    emit_value(sink, context, "process.open_fds", &sample.open_fd_count,
               SYSTEM_HEALTH_UNIT_COUNT);
    emit_value(sink, context, "process.fd_limit",
               &sample.effective_fd_limit, SYSTEM_HEALTH_UNIT_COUNT);
    emit_ratio(sink, context, "process.fd_ratio", &sample.open_fd_count,
               &sample.effective_fd_limit);
    emit_value(sink, context, "process.pid_limit",
               &sample.effective_pid_limit, SYSTEM_HEALTH_UNIT_COUNT);
    emit_ratio(sink, context, "process.pid_ratio", &sample.thread_count,
               &sample.effective_pid_limit);
    return 0;
}

void linux_process_collector_init(system_health_collector_t *collector,
                                  linux_process_collector_state_t *state) {
    if (!collector) return;
    memset(collector, 0, sizeof(*collector));
    snprintf(collector->name, sizeof(collector->name), "%s", "linux_process");
    collector->scope = SYSTEM_HEALTH_SCOPE_PROCESS;
    collector->tier = SYSTEM_HEALTH_TIER_NORMAL;
    collector->interval_seconds = 15U;
    collector->stale_after_seconds = 45U;
    collector->state = state;
    collector->collect = linux_process_collect;
}
