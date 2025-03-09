#include <pthread.h>
#include <stdlib.h>
#include <stdbool.h>
#include "core/logger.h"
#include "web/thread_pool.h"

// Initialize the thread pool with a specific number of threads
thread_pool_t *thread_pool_init(int num_threads, int queue_size) {
    if (num_threads <= 0 || queue_size <= 0) {
        log_error("Invalid thread pool parameters");
        return NULL;
    }

    thread_pool_t *pool = calloc(1, sizeof(thread_pool_t));
    if (!pool) {
        log_error("Failed to allocate memory for thread pool");
        return NULL;
    }

    // Initialize mutex and condition variables
    if (pthread_mutex_init(&pool->mutex, NULL) != 0) {
        log_error("Failed to initialize mutex");
        free(pool);
        return NULL;
    }

    if (pthread_cond_init(&pool->not_empty, NULL) != 0) {
        log_error("Failed to initialize condition variable");
        pthread_mutex_destroy(&pool->mutex);
        free(pool);
        return NULL;
    }

    if (pthread_cond_init(&pool->not_full, NULL) != 0) {
        log_error("Failed to initialize condition variable");
        pthread_cond_destroy(&pool->not_empty);
        pthread_mutex_destroy(&pool->mutex);
        free(pool);
        return NULL;
    }

    // Allocate task queue
    pool->queue = calloc(queue_size, sizeof(task_t));
    if (!pool->queue) {
        log_error("Failed to allocate task queue");
        pthread_cond_destroy(&pool->not_full);
        pthread_cond_destroy(&pool->not_empty);
        pthread_mutex_destroy(&pool->mutex);
        free(pool);
        return NULL;
    }

    pool->queue_size = queue_size;
    pool->head = 0;
    pool->tail = 0;
    pool->count = 0;

    // Create worker threads
    pool->threads = calloc(num_threads, sizeof(pthread_t));
    if (!pool->threads) {
        log_error("Failed to allocate thread array");
        free(pool->queue);
        pthread_cond_destroy(&pool->not_full);
        pthread_cond_destroy(&pool->not_empty);
        pthread_mutex_destroy(&pool->mutex);
        free(pool);
        return NULL;
    }

    pool->thread_count = num_threads;
    pool->shutdown = false;

    // Start worker threads
    for (int i = 0; i < num_threads; i++) {
        if (pthread_create(&pool->threads[i], NULL, worker_thread, pool) != 0) {
            log_error("Failed to create worker thread %d", i);
            // Clean up threads that were created
            pool->shutdown = true;
            pthread_cond_broadcast(&pool->not_empty);
            for (int j = 0; j < i; j++) {
                pthread_join(pool->threads[j], NULL);
            }

            free(pool->threads);
            free(pool->queue);
            pthread_cond_destroy(&pool->not_full);
            pthread_cond_destroy(&pool->not_empty);
            pthread_mutex_destroy(&pool->mutex);
            free(pool);
            return NULL;
        }
    }

    log_info("Thread pool initialized with %d threads and queue size %d", num_threads, queue_size);
    return pool;
}

// Add a task to the thread pool
bool thread_pool_add_task(thread_pool_t *pool, void (*function)(void *), void *argument) {
    if (!pool || !function) {
        log_error("Invalid thread pool parameters");
        return false;
    }

    task_t task;
    task.function = function;
    task.argument = argument;

    pthread_mutex_lock(&pool->mutex);

    // Wait until queue has space or shutdown
    while (pool->count == pool->queue_size && !pool->shutdown) {
        pthread_cond_wait(&pool->not_full, &pool->mutex);
    }

    // Check if pool was shut down while waiting
    if (pool->shutdown) {
        pthread_mutex_unlock(&pool->mutex);
        return false;
    }

    // Add task to queue
    bool result = queue_push(pool, task);

    // Signal waiting thread
    pthread_cond_signal(&pool->not_empty);

    pthread_mutex_unlock(&pool->mutex);

    return result;
}

// Shutdown the thread pool
void thread_pool_shutdown(thread_pool_t *pool) {
    if (!pool) {
        return;
    }

    pthread_mutex_lock(&pool->mutex);
    pool->shutdown = true;
    pthread_cond_broadcast(&pool->not_empty);
    pthread_mutex_unlock(&pool->mutex);

    // Wait for all threads to finish
    for (int i = 0; i < pool->thread_count; i++) {
        pthread_join(pool->threads[i], NULL);
    }

    // Cleanup resources
    free(pool->threads);
    free(pool->queue);
    pthread_cond_destroy(&pool->not_full);
    pthread_cond_destroy(&pool->not_empty);
    pthread_mutex_destroy(&pool->mutex);
    free(pool);

    log_info("Thread pool shutdown complete");
}

// Worker thread function
static void *worker_thread(void *arg) {
    thread_pool_t *pool = (thread_pool_t *)arg;
    task_t task;

    while (1) {
        pthread_mutex_lock(&pool->mutex);

        // Wait for tasks or shutdown
        while (pool->count == 0 && !pool->shutdown) {
            pthread_cond_wait(&pool->not_empty, &pool->mutex);
        }

        // Check if pool is shutting down with no tasks
        if (pool->count == 0 && pool->shutdown) {
            pthread_mutex_unlock(&pool->mutex);
            pthread_exit(NULL);
        }

        // Get task from queue
        bool got_task = queue_pop(pool, &task);

        // Signal that queue is not full
        pthread_cond_signal(&pool->not_full);

        pthread_mutex_unlock(&pool->mutex);

        // Execute the task
        if (got_task) {
            task.function(task.argument);
        }
    }

    return NULL;
}

// Add task to the queue
static bool queue_push(thread_pool_t *pool, task_t task) {
    if (pool->count == pool->queue_size) {
        return false;
    }

    pool->queue[pool->tail] = task;
    pool->tail = (pool->tail + 1) % pool->queue_size;
    pool->count++;

    return true;
}

// Remove task from the queue
static bool queue_pop(thread_pool_t *pool, task_t *task) {
    if (pool->count == 0) {
        return false;
    }

    *task = pool->queue[pool->head];
    pool->head = (pool->head + 1) % pool->queue_size;
    pool->count--;

    return true;
}