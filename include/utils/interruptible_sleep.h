#ifndef LIGHTNVR_INTERRUPTIBLE_SLEEP_H
#define LIGHTNVR_INTERRUPTIBLE_SLEEP_H

#include <pthread.h>
#include <stdbool.h>

/*
 * A small wakeable sleep primitive for background worker threads.
 *
 * A worker sleeps with interruptible_sleep_wait(); its stop path clears the
 * thread's run flag and then calls interruptible_sleep_wake(), so the worker
 * leaves its sleep immediately instead of polling its run flag once a second.
 *
 * This is the safe replacement for the pattern of waking a thread with
 * pthread_kill(thread, SIGALRM): SIGALRM is reserved for the process-wide
 * emergency forced-exit handler (core/main.c, core/daemon.c), so delivering it
 * to a worker thread can advance the forced-exit phases or terminate the
 * process mid-shutdown.
 *
 * Each instance is intended for a SINGLE waiting thread. The wake is sticky: a
 * wake delivered while no thread is waiting is remembered, so the next wait()
 * returns immediately and the classic "stop set the flag and signalled just
 * before the thread called wait()" lost-wakeup race cannot occur.
 */
typedef struct {
    pthread_mutex_t mutex;
    pthread_cond_t  cond;
    bool            woken;   /* sticky pending-wake flag, protected by mutex */
} interruptible_sleep_t;

/* Initialize. The condition variable uses CLOCK_MONOTONIC where available so
 * wall-clock changes can't stretch the timeout. Returns 0 on success. */
int  interruptible_sleep_init(interruptible_sleep_t *s);

/* Release resources. The caller must ensure no thread is waiting. */
void interruptible_sleep_destroy(interruptible_sleep_t *s);

/* Clear any pending (sticky) wake so a reused instance starts a fresh cycle. */
void interruptible_sleep_reset(interruptible_sleep_t *s);

/* Sleep up to `seconds`, returning early as soon as interruptible_sleep_wake()
 * is (or has already been) called. A non-positive `seconds` still honours a
 * pending wake but otherwise returns promptly. */
void interruptible_sleep_wait(interruptible_sleep_t *s, int seconds);

/* Wake the thread currently in — or the next thread to enter — wait(). */
void interruptible_sleep_wake(interruptible_sleep_t *s);

#endif /* LIGHTNVR_INTERRUPTIBLE_SLEEP_H */
