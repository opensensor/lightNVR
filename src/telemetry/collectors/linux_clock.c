#define _POSIX_C_SOURCE 200809L

#include "telemetry/collectors/linux_clock.h"

#include <errno.h>
#include <limits.h>
#include <string.h>
#include <sys/timex.h>
#include <time.h>

static int64_t saturating_subtract(int64_t left, int64_t right) {
    if (right > 0 && left < INT64_MIN + right) return INT64_MIN;
    if (right < 0 && left > INT64_MAX + right) return INT64_MAX;
    return left - right;
}

static int default_realtime(void *context, int64_t *milliseconds) {
    (void)context;
    struct timespec value;
    if (!milliseconds || clock_gettime(CLOCK_REALTIME, &value) != 0) return -1;
    *milliseconds = (int64_t)value.tv_sec * 1000LL + value.tv_nsec / 1000000LL;
    return 0;
}

static int default_monotonic(void *context, int64_t *milliseconds) {
    (void)context;
    struct timespec value;
    if (!milliseconds || clock_gettime(CLOCK_MONOTONIC, &value) != 0) return -1;
    *milliseconds = (int64_t)value.tv_sec * 1000LL + value.tv_nsec / 1000000LL;
    return 0;
}

static int default_synchronization(void *context, bool *synchronized) {
    (void)context;
    struct timex value;
    memset(&value, 0, sizeof(value));
    errno = 0;
    int result = adjtimex(&value);
    if (result < 0 || !synchronized) return -1;
    *synchronized = result != TIME_ERROR && (value.status & STA_UNSYNC) == 0;
    return 0;
}

linux_clock_source_t linux_clock_default_source(void) {
    linux_clock_source_t source = {
        .context = NULL,
        .realtime = default_realtime,
        .monotonic = default_monotonic,
        .synchronization = default_synchronization
    };
    return source;
}

int linux_clock_sample(const linux_clock_source_t *source,
                       linux_clock_sample_t *sample) {
    if (!source || !sample || !source->realtime || !source->monotonic) return -1;
    memset(sample, 0, sizeof(*sample));
    int64_t monotonic_ms = 0;
    if (source->realtime(source->context, &sample->realtime_ms) != 0 ||
        source->monotonic(source->context, &monotonic_ms) != 0 ||
        monotonic_ms < 0) {
        sample->capability = errno == EACCES || errno == EPERM
            ? SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED
            : SYSTEM_HEALTH_CAPABILITY_ERROR;
        return -1;
    }
    sample->monotonic_ms = (uint64_t)monotonic_ms;
    sample->capability = SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    if (!source->synchronization) {
        sample->capability = SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
        return 0;
    }
    errno = 0;
    if (source->synchronization(source->context, &sample->synchronized) != 0) {
        sample->capability = errno == ENOSYS
            ? SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED
            : (errno == EACCES || errno == EPERM
                ? SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED
                : SYSTEM_HEALTH_CAPABILITY_ERROR);
        return 0;
    }
    sample->synchronization_known = true;
    return 0;
}

linux_clock_result_t linux_clock_evaluate(linux_clock_state_t *state,
                                          const linux_clock_sample_t *sample,
                                          uint64_t startup_grace_ms,
                                          int64_t jump_threshold_ms) {
    linux_clock_result_t result;
    memset(&result, 0, sizeof(result));
    if (!state || !sample) {
        result.capability = SYSTEM_HEALTH_CAPABILITY_ERROR;
        return result;
    }
    result.capability = sample->capability;
    result.synchronization_known = sample->synchronization_known;
    result.synchronized = sample->synchronized;
    if (jump_threshold_ms == INT64_MIN) jump_threshold_ms = INT64_MAX;
    else if (jump_threshold_ms < 0) jump_threshold_ms = -jump_threshold_ms;

    if (!state->initialized) {
        state->initialized = true;
        state->startup_monotonic_ms = sample->monotonic_ms;
    } else if (sample->monotonic_ms >= state->previous_monotonic_ms) {
        uint64_t monotonic_delta = sample->monotonic_ms - state->previous_monotonic_ms;
        if (monotonic_delta <= (uint64_t)INT64_MAX) {
            int64_t realtime_delta = saturating_subtract(
                sample->realtime_ms, state->previous_realtime_ms);
            result.jump_ms = saturating_subtract(
                realtime_delta, (int64_t)monotonic_delta);
            result.jump_detected = result.jump_ms > jump_threshold_ms ||
                result.jump_ms < -jump_threshold_ms;
        }
    }
    result.startup_grace_active = sample->monotonic_ms >= state->startup_monotonic_ms &&
        sample->monotonic_ms - state->startup_monotonic_ms < startup_grace_ms;
    state->previous_realtime_ms = sample->realtime_ms;
    state->previous_monotonic_ms = sample->monotonic_ms;
    return result;
}
