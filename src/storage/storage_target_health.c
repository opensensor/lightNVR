#define _POSIX_C_SOURCE 200809L

#include "storage/storage_target_health.h"

#include <limits.h>
#include <pthread.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#include "core/event_producers.h"
#include "core/logger.h"

static pthread_mutex_t target_probe_mutex = PTHREAD_MUTEX_INITIALIZER;

static bool state_is(const storage_target_t *target, const char *state) {
    return target && strcmp(target->health_status, state) == 0;
}

static const char *normalized_previous_state(const storage_target_t *target) {
    if (state_is(target, "healthy")) return "healthy";
    if (state_is(target, "degraded")) return "degraded";
    return "unknown";
}

static const char *normalized_unavailable_reason(const char *error) {
    if (!error) return "unknown";
    static const char missing_mount[] = "Required mount is absent:";
    static const char no_mount[] = "No distinct mounted filesystem";
    static const char directory[] = "Target directory is unavailable:";
    static const char capacity[] = "Capacity probe failed:";
    static const char not_writable[] = "Target directory is not writable:";
    static const char long_path[] = "Probe path is too long";
    static const char write_probe[] = "Write probe failed:";
    static const char cleanup[] = "Probe cleanup failed:";

    if (strncmp(error, missing_mount, sizeof(missing_mount) - 1) == 0 ||
        strncmp(error, no_mount, sizeof(no_mount) - 1) == 0) {
        return "mount_unavailable";
    }
    if (strncmp(error, directory, sizeof(directory) - 1) == 0) {
        return "directory_unavailable";
    }
    if (strncmp(error, capacity, sizeof(capacity) - 1) == 0) {
        return "capacity_probe_failed";
    }
    if (strncmp(error, not_writable, sizeof(not_writable) - 1) == 0) {
        return "not_writable";
    }
    if (strncmp(error, long_path, sizeof(long_path) - 1) == 0 ||
        strncmp(error, write_probe, sizeof(write_probe) - 1) == 0) {
        return "write_probe_failed";
    }
    if (strncmp(error, cleanup, sizeof(cleanup) - 1) == 0) {
        return "probe_cleanup_failed";
    }
    return "unknown";
}

static int64_t downtime_milliseconds(const storage_target_t *before,
                                     const storage_target_t *after) {
    if (!before || !after || before->last_success_at <= 0 ||
        after->last_probe_at <= before->last_success_at) {
        return 0;
    }
    int64_t seconds = after->last_probe_at - before->last_success_at;
    return seconds > INT64_MAX / 1000 ? INT64_MAX : seconds * 1000;
}

static void publish_transition(const storage_target_t *before,
                               const storage_target_t *after) {
    bool was_unavailable = state_is(before, "unavailable");
    bool is_unavailable = state_is(after, "unavailable");
    bool is_recovered = state_is(after, "healthy") ||
        state_is(after, "degraded");
    time_t occurred_at = after->last_probe_at > 0
        ? (time_t)after->last_probe_at : time(NULL);
    char error[256] = {0};
    int result = 0;

    if (!was_unavailable && is_unavailable) {
        result = event_producer_publish_storage_target_unavailable(
            after->uuid, normalized_previous_state(before),
            normalized_unavailable_reason(after->last_error),
            after->is_default, occurred_at, error, sizeof(error));
    } else if (was_unavailable && is_recovered) {
        result = event_producer_publish_storage_target_recovered(
            after->uuid, after->health_status,
            downtime_milliseconds(before, after), after->is_default,
            occurred_at, error, sizeof(error));
    } else {
        return;
    }

    if (result != 0) {
        log_warn("Failed to enqueue storage target transition event for %s: %s",
                 after->uuid, error[0] ? error : "event pipeline unavailable");
    }
}

db_storage_target_result_t storage_target_probe_and_publish(
    const char *uuid, bool write_test, storage_target_t *target) {
    storage_target_t before;
    storage_target_t after;
    memset(&before, 0, sizeof(before));
    memset(&after, 0, sizeof(after));

    pthread_mutex_lock(&target_probe_mutex);
    db_storage_target_result_t result = db_storage_target_get(uuid, &before);
    if (result == DB_STORAGE_TARGET_OK) {
        result = db_storage_target_probe(uuid, write_test, &after);
        if (result == DB_STORAGE_TARGET_OK ||
            result == DB_STORAGE_TARGET_UNAVAILABLE) {
            publish_transition(&before, &after);
            if (target) *target = after;
        }
    }
    pthread_mutex_unlock(&target_probe_mutex);
    return result;
}

int storage_target_refresh_health_and_publish(void) {
    int total = db_storage_target_count();
    if (total < 0 || total > STORAGE_TARGET_MAX_COUNT) return -1;
    if (total == 0) return 0;

    storage_target_t *targets = calloc((size_t)total, sizeof(*targets));
    if (!targets) return -1;
    int count = db_storage_target_list(targets, total);
    if (count < 0) {
        free(targets);
        return -1;
    }

    int failures = 0;
    for (int index = 0; index < count; index++) {
        db_storage_target_result_t result =
            storage_target_probe_and_publish(
                targets[index].uuid, false, NULL);
        if (result != DB_STORAGE_TARGET_OK &&
            result != DB_STORAGE_TARGET_UNAVAILABLE) {
            failures++;
        }
    }
    free(targets);
    return failures == 0 ? count : -1;
}
