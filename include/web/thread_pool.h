#ifndef LIGHTNVR_THREAD_POOL_H
#define LIGHTNVR_THREAD_POOL_H

#include <pthread.h>
#include <stdlib.h>
#include <stdbool.h>
#include "core/logger.h"

typedef struct {
    void (*function)(void *);
    void *argument;
} task_t;

typedef struct {
    task_t *queue;
    int queue_size;
    int head;
    int tail;
    int count;

    pthread_mutex_t mutex;
    pthread_cond_t not_empty;
    pthread_cond_t not_full;
    pthread_t *threads;
    int thread_count;
    bool shutdown;
} thread_pool_t;

// Forward declarations
static void *worker_thread(void *arg);
static bool queue_push(thread_pool_t *pool, task_t task);
static bool queue_pop(thread_pool_t *pool, task_t *task);

thread_pool_t *thread_pool_init(int num_threads, int queue_size);

bool thread_pool_add_task(thread_pool_t *pool, void (*function)(void *), void *argument);

void thread_pool_shutdown(thread_pool_t *pool);

void thread_pool_destroy(thread_pool_t *pool);

static void *worker_thread(void *arg);

#endif // LIGHTNVR_THREAD_POOL_H
