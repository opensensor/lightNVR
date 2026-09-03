#define _POSIX_C_SOURCE 200809L

#include "telemetry/providers/kernel_log.h"

#include <ctype.h>
#include <errno.h>
#include <fcntl.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

static const char *const category_metrics[KERNEL_LOG_CATEGORY_COUNT] = {
    "kernel.filesystem_remount_delta",
    "kernel.block_io_error_delta",
    "kernel.machine_check_delta",
    "kernel.thermal_shutdown_delta",
    "kernel.oom_kill_delta"
};

static system_health_capability_t capability_from_errno(int error_number) {
    if (error_number == EACCES || error_number == EPERM)
        return SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED;
    if (error_number == ENOENT || error_number == ENODEV ||
        error_number == ENOTDIR || error_number == ENOTSUP)
        return SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
    return SYSTEM_HEALTH_CAPABILITY_ERROR;
}

static int default_open(const char *path, int flags) {
    return open(path, flags);
}

static ssize_t default_read(int descriptor, void *buffer, size_t size) {
    return read(descriptor, buffer, size);
}

static int default_close(int descriptor) { return close(descriptor); }

void kernel_log_state_init(kernel_log_state_t *state, const char *path) {
    if (!state) return;
    memset(state, 0, sizeof(*state));
    state->descriptor = -1;
    state->capability = SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
    snprintf(state->path, sizeof(state->path), "%s",
             path && path[0] ? path : "/dev/kmsg");
    state->ops.open_log = default_open;
    state->ops.read_log = default_read;
    state->ops.close_log = default_close;
}

static bool contains(const char *text, const char *needle) {
    return strstr(text, needle) != NULL;
}

int kernel_log_classify_line(const char *line, size_t length,
                             bool matches[KERNEL_LOG_CATEGORY_COUNT],
                             uint64_t *sequence, bool *sequence_valid) {
    if (!line || !matches || !sequence || !sequence_valid ||
        length == 0U || length >= KERNEL_LOG_READ_BUFFER) return -1;
    memset(matches, 0, sizeof(bool) * KERNEL_LOG_CATEGORY_COUNT);
    *sequence = 0U;
    *sequence_valid = false;

    const char *end = line + length;
    const char *first_comma = memchr(line, ',', length);
    if (first_comma) {
        const char *cursor = first_comma + 1;
        if (cursor < end && isdigit((unsigned char)*cursor)) {
            uint64_t parsed = 0U;
            bool valid = true;
            while (cursor < end && isdigit((unsigned char)*cursor)) {
                unsigned int digit = (unsigned int)(*cursor - '0');
                if (parsed > (UINT64_MAX - digit) / 10U) {
                    valid = false;
                    break;
                }
                parsed = parsed * 10U + digit;
                cursor++;
            }
            if (valid && cursor < end && *cursor == ',') {
                *sequence = parsed;
                *sequence_valid = true;
            }
        }
    }

    const char *message = memchr(line, ';', length);
    if (message) message++;
    else message = line;
    size_t message_length = (size_t)(end - message);
    char normalized[KERNEL_LOG_READ_BUFFER];
    for (size_t index = 0U; index < message_length; ++index)
        normalized[index] = (char)tolower((unsigned char)message[index]);
    normalized[message_length] = '\0';

    matches[KERNEL_LOG_FILESYSTEM_REMOUNT] =
        (contains(normalized, "remount") &&
         (contains(normalized, "read-only") ||
          contains(normalized, "readonly"))) ||
        contains(normalized, "filesystem has been set read-only");
    matches[KERNEL_LOG_BLOCK_IO_ERROR] =
        contains(normalized, "i/o error") ||
        contains(normalized, "blk_update_request") ||
        contains(normalized, "critical medium error");
    matches[KERNEL_LOG_MACHINE_CHECK] =
        contains(normalized, "machine check") ||
        contains(normalized, "mce:") ||
        contains(normalized, "hardware error");
    matches[KERNEL_LOG_THERMAL_SHUTDOWN] =
        contains(normalized, "thermal shutdown") ||
        contains(normalized, "critical temperature reached") ||
        (contains(normalized, "overheat") &&
         contains(normalized, "shutdown"));
    matches[KERNEL_LOG_OOM_KILL] =
        contains(normalized, "out of memory") ||
        contains(normalized, "oom-killer") ||
        contains(normalized, "oom_reaper") ||
        contains(normalized, "killed process");
    return 0;
}

int kernel_log_discover(void *opaque,
                        const system_health_collect_context_t *context,
                        system_health_provider_inventory_t *inventory) {
    (void)context;
    kernel_log_state_t *state = opaque;
    if (!state || !inventory || !state->ops.open_log ||
        !state->ops.read_log || !state->ops.close_log) return -1;
    memset(inventory, 0, sizeof(*inventory));
    if (state->descriptor < 0) {
        state->descriptor = state->ops.open_log(
            state->path, O_RDONLY | O_NONBLOCK | O_CLOEXEC);
        state->capability = state->descriptor >= 0
            ? SYSTEM_HEALTH_CAPABILITY_AVAILABLE
            : capability_from_errno(errno);
    }
    snprintf(inventory->resources[0].id,
             sizeof(inventory->resources[0].id), "%s", "kernel");
    inventory->resources[0].scope = SYSTEM_HEALTH_SCOPE_HOST;
    inventory->resources[0].capability = state->capability;
    inventory->count = 1U;
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
    observation.scope = SYSTEM_HEALTH_SCOPE_HOST;
    observation.sampled_monotonic_ms = context->monotonic_ms;
    observation.observed_wall_time_ms = context->wall_time_ms;
    if (valid)
        system_health_observation_set_available(&observation, value, unit);
    else
        system_health_observation_set_unavailable(&observation, capability);
    (void)system_health_observation_sink_append(sink, &observation);
}

static uint64_t line_hash(const char *line, size_t length) {
    const char *message = memchr(line, ';', length);
    if (message) {
        message++;
        length = (size_t)((line + length) - message);
        line = message;
    }
    uint64_t hash = UINT64_C(1469598103934665603);
    for (size_t index = 0U; index < length; ++index) {
        hash ^= (unsigned char)line[index];
        hash *= UINT64_C(1099511628211);
    }
    return hash == 0U ? 1U : hash;
}

static bool hash_seen(kernel_log_state_t *state, uint64_t hash) {
    for (size_t index = 0U; index < state->recent_count; ++index)
        if (state->recent_hashes[index] == hash) return true;
    state->recent_hashes[state->recent_next] = hash;
    state->recent_next = (state->recent_next + 1U) % KERNEL_LOG_DEDUPE_SLOTS;
    if (state->recent_count < KERNEL_LOG_DEDUPE_SLOTS) state->recent_count++;
    return false;
}

static void process_line(kernel_log_state_t *state, const char *line,
                         size_t length,
                         uint64_t counts[KERNEL_LOG_CATEGORY_COUNT]) {
    while (length > 0U && (line[length - 1U] == '\n' ||
                           line[length - 1U] == '\r')) length--;
    if (length == 0U || length >= KERNEL_LOG_READ_BUFFER) return;
    bool matches[KERNEL_LOG_CATEGORY_COUNT];
    uint64_t sequence = 0U;
    bool sequence_valid = false;
    if (kernel_log_classify_line(line, length, matches, &sequence,
                                 &sequence_valid) != 0) return;
    if (sequence_valid) {
        if (state->sequence_valid && sequence <= state->last_sequence) return;
        state->last_sequence = sequence;
        state->sequence_valid = true;
    } else if (hash_seen(state, line_hash(line, length))) {
        return;
    }
    for (size_t category = 0U; category < KERNEL_LOG_CATEGORY_COUNT;
         ++category)
        if (matches[category] && counts[category] < UINT64_MAX)
            counts[category]++;
}

int kernel_log_collect(void *opaque,
                       const system_health_collect_context_t *context,
                       system_health_observation_sink_t *sink) {
    kernel_log_state_t *state = opaque;
    if (!state || !context || !sink || !state->ops.read_log ||
        !state->ops.close_log) return -1;
    emit(sink, context, "hardware.provider.visible", "kernel_log",
         state->capability,
         state->capability == SYSTEM_HEALTH_CAPABILITY_AVAILABLE, 1.0,
         SYSTEM_HEALTH_UNIT_BOOLEAN);
    if (state->descriptor < 0 ||
        state->capability != SYSTEM_HEALTH_CAPABILITY_AVAILABLE) {
        for (size_t category = 0U; category < KERNEL_LOG_CATEGORY_COUNT;
             ++category)
            emit(sink, context, category_metrics[category], "kernel",
                 state->capability, false, 0.0, SYSTEM_HEALTH_UNIT_COUNT);
        return 0;
    }

    uint64_t counts[KERNEL_LOG_CATEGORY_COUNT] = {0U};
    char buffer[KERNEL_LOG_READ_BUFFER];
    for (size_t attempt = 0U; attempt < KERNEL_LOG_READS_PER_CYCLE;
         ++attempt) {
        ssize_t length = state->ops.read_log(state->descriptor, buffer,
                                             sizeof(buffer) - 1U);
        if (length < 0) {
            if (errno == EAGAIN || errno == EWOULDBLOCK) break;
            state->capability = capability_from_errno(errno);
            (void)state->ops.close_log(state->descriptor);
            state->descriptor = -1;
            break;
        }
        if (length == 0) break;
        buffer[length] = '\0';
        size_t start = 0U;
        for (size_t index = 0U; index <= (size_t)length; ++index) {
            if (index == (size_t)length || buffer[index] == '\n') {
                process_line(state, buffer + start, index - start, counts);
                start = index + 1U;
            }
        }
    }
    bool valid = state->capability == SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    for (size_t category = 0U; category < KERNEL_LOG_CATEGORY_COUNT;
         ++category)
        emit(sink, context, category_metrics[category], "kernel",
             state->capability, valid, (double)counts[category],
             SYSTEM_HEALTH_UNIT_COUNT);
    return 0;
}

void kernel_log_destroy(void *opaque) {
    kernel_log_state_t *state = opaque;
    if (!state) return;
    if (state->descriptor >= 0 && state->ops.close_log)
        (void)state->ops.close_log(state->descriptor);
    state->descriptor = -1;
}

void kernel_log_provider_init(system_health_provider_t *provider,
                              kernel_log_state_t *state) {
    if (!provider) return;
    memset(provider, 0, sizeof(*provider));
    snprintf(provider->name, sizeof(provider->name), "%s", "kernel_log");
    provider->capability = SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
    provider->state = state;
    provider->discover = kernel_log_discover;
    provider->collect = kernel_log_collect;
    provider->destroy = kernel_log_destroy;
}
