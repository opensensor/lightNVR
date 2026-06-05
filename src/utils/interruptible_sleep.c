#define _POSIX_C_SOURCE 200809L

#include "utils/interruptible_sleep.h"

#include <errno.h>
#include <time.h>

int interruptible_sleep_init(interruptible_sleep_t *s) {
    if (!s) {
        return -1;
    }

    s->woken = false;

    if (pthread_mutex_init(&s->mutex, NULL) != 0) {
        return -1;
    }

    pthread_condattr_t attr;
    pthread_condattr_init(&attr);
    s->clock_id = CLOCK_REALTIME;
#if defined(CLOCK_MONOTONIC)
    /* Use a monotonic clock so an NTP/admin wall-clock step can't lengthen the
     * timeout. Falls back to the default (CLOCK_REALTIME) if unsupported. */
    if (pthread_condattr_setclock(&attr, CLOCK_MONOTONIC) == 0) {
        s->clock_id = CLOCK_MONOTONIC;
    }
#endif
    int rc = pthread_cond_init(&s->cond, &attr);
    pthread_condattr_destroy(&attr);

    if (rc != 0) {
        pthread_mutex_destroy(&s->mutex);
        return -1;
    }

    return 0;
}

void interruptible_sleep_destroy(interruptible_sleep_t *s) {
    if (!s) {
        return;
    }
    pthread_cond_destroy(&s->cond);
    pthread_mutex_destroy(&s->mutex);
}

void interruptible_sleep_reset(interruptible_sleep_t *s) {
    if (!s) {
        return;
    }
    pthread_mutex_lock(&s->mutex);
    s->woken = false;
    pthread_mutex_unlock(&s->mutex);
}

void interruptible_sleep_wait(interruptible_sleep_t *s, int seconds) {
    if (!s) {
        return;
    }

    struct timespec ts;
    clock_gettime(s->clock_id, &ts);
    if (seconds > 0) {
        ts.tv_sec += seconds;
    }

    pthread_mutex_lock(&s->mutex);
    while (!s->woken) {
        int rc = pthread_cond_timedwait(&s->cond, &s->mutex, &ts);
        if (rc == ETIMEDOUT || rc != 0) {
            /* Timed out (or an unexpected error): stop waiting. */
            break;
        }
        /* rc == 0: either a real wake or a spurious one — the while-loop
         * re-checks s->woken and keeps waiting on spurious wakeups. */
    }
    /* Consume the sticky wake so the next cycle starts fresh. */
    s->woken = false;
    pthread_mutex_unlock(&s->mutex);
}

void interruptible_sleep_wake(interruptible_sleep_t *s) {
    if (!s) {
        return;
    }
    pthread_mutex_lock(&s->mutex);
    s->woken = true;
    pthread_cond_signal(&s->cond);
    pthread_mutex_unlock(&s->mutex);
}
