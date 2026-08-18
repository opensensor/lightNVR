/**
 * @file go2rtc_lifecycle.c
 * @brief Single lifecycle coordinator for the shared go2rtc service.
 */

#include <pthread.h>
#include <string.h>

#include "video/go2rtc/go2rtc_lifecycle.h"

typedef struct {
    pthread_mutex_t mutex;
    pthread_cond_t cond;
    bool active;
    pthread_t owner;
    unsigned int depth;
    go2rtc_lifecycle_operation_t operation;
    bool intentional;
    bool restart_occurred;
    uint64_t generation;
    uint64_t restart_generation;
    bool operation_result[GO2RTC_LIFECYCLE_OPERATION_COUNT];
    uint64_t operation_generation[GO2RTC_LIFECYCLE_OPERATION_COUNT];
} go2rtc_lifecycle_state_t;

static go2rtc_lifecycle_state_t g_lifecycle = {
    .mutex = PTHREAD_MUTEX_INITIALIZER,
    .cond = PTHREAD_COND_INITIALIZER,
};

bool go2rtc_lifecycle_begin(go2rtc_lifecycle_operation_t operation,
                            bool coalesce,
                            bool intentional,
                            go2rtc_lifecycle_guard_t *guard) {
    if (!guard || operation < GO2RTC_LIFECYCLE_CHECK ||
        operation >= GO2RTC_LIFECYCLE_OPERATION_COUNT) {
        return false;
    }

    memset(guard, 0, sizeof(*guard));
    pthread_t self = pthread_self();

    if (pthread_mutex_lock(&g_lifecycle.mutex) != 0) {
        return false;
    }

    if (g_lifecycle.active && pthread_equal(g_lifecycle.owner, self)) {
        g_lifecycle.depth++;
        if (operation == GO2RTC_LIFECYCLE_RESTART ||
            operation == GO2RTC_LIFECYCLE_FULL_START) {
            g_lifecycle.restart_occurred = true;
        }
        if (intentional) {
            g_lifecycle.intentional = true;
        }
        guard->acquired = true;
        guard->generation = g_lifecycle.generation;
        guard->restart_generation = g_lifecycle.restart_generation;
        pthread_mutex_unlock(&g_lifecycle.mutex);
        return true;
    }

    if (coalesce && g_lifecycle.active &&
        g_lifecycle.operation == operation) {
        uint64_t active_generation =
            g_lifecycle.operation_generation[operation];
        while (g_lifecycle.operation_generation[operation] ==
               active_generation) {
            pthread_cond_wait(&g_lifecycle.cond, &g_lifecycle.mutex);
        }
        guard->coalesced = true;
        guard->result = g_lifecycle.operation_result[operation];
        guard->generation = g_lifecycle.generation;
        guard->restart_generation = g_lifecycle.restart_generation;
        pthread_mutex_unlock(&g_lifecycle.mutex);
        return true;
    }

    while (g_lifecycle.active) {
        pthread_cond_wait(&g_lifecycle.cond, &g_lifecycle.mutex);
    }

    g_lifecycle.active = true;
    g_lifecycle.owner = self;
    g_lifecycle.depth = 1;
    g_lifecycle.operation = operation;
    g_lifecycle.intentional = intentional;
    g_lifecycle.restart_occurred =
        (operation == GO2RTC_LIFECYCLE_RESTART ||
         operation == GO2RTC_LIFECYCLE_FULL_START);

    guard->acquired = true;
    guard->outermost = true;
    guard->generation = g_lifecycle.generation;
    guard->restart_generation = g_lifecycle.restart_generation;
    pthread_mutex_unlock(&g_lifecycle.mutex);
    return true;
}

void go2rtc_lifecycle_end(go2rtc_lifecycle_guard_t *guard, bool result) {
    if (!guard || !guard->acquired) {
        return;
    }

    pthread_mutex_lock(&g_lifecycle.mutex);
    if (!g_lifecycle.active ||
        !pthread_equal(g_lifecycle.owner, pthread_self()) ||
        g_lifecycle.depth == 0) {
        pthread_mutex_unlock(&g_lifecycle.mutex);
        return;
    }

    g_lifecycle.depth--;
    if (g_lifecycle.depth == 0) {
        go2rtc_lifecycle_operation_t completed_operation =
            g_lifecycle.operation;
        g_lifecycle.active = false;
        g_lifecycle.intentional = false;
        g_lifecycle.generation++;
        g_lifecycle.operation_result[completed_operation] = result;
        g_lifecycle.operation_generation[completed_operation]++;
        if (g_lifecycle.restart_occurred && result) {
            g_lifecycle.restart_generation++;
        }
        g_lifecycle.restart_occurred = false;
        pthread_cond_broadcast(&g_lifecycle.cond);
    }
    pthread_mutex_unlock(&g_lifecycle.mutex);
    guard->acquired = false;
}

bool go2rtc_lifecycle_intentional_restart_active(void) {
    pthread_mutex_lock(&g_lifecycle.mutex);
    bool active = g_lifecycle.active && g_lifecycle.intentional &&
                  g_lifecycle.operation != GO2RTC_LIFECYCLE_CHECK;
    pthread_mutex_unlock(&g_lifecycle.mutex);
    return active;
}

uint64_t go2rtc_lifecycle_generation(void) {
    pthread_mutex_lock(&g_lifecycle.mutex);
    uint64_t generation = g_lifecycle.generation;
    pthread_mutex_unlock(&g_lifecycle.mutex);
    return generation;
}

uint64_t go2rtc_lifecycle_restart_generation(void) {
    pthread_mutex_lock(&g_lifecycle.mutex);
    uint64_t generation = g_lifecycle.restart_generation;
    pthread_mutex_unlock(&g_lifecycle.mutex);
    return generation;
}
