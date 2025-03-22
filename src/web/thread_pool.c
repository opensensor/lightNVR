#include "web/thread_pool.h"
#include "core/logger.h"
#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>
#include <stdbool.h>

/**
 * @brief Initialize a thread pool
 * 
 * @param num_threads Number of worker threads
 * @param queue_size Size of the task queue
 * @return thread_pool_t* Pointer to the thread pool or NULL on error
 */
thread_pool_t *thread_pool_init(int num_threads, int queue_size) {
    if (num_threads <= 0 || queue_size <= 0) {
        log_error("Invalid thread pool parameters: num_threads=%d, queue_size=%d", 
                 num_threads, queue_size);
        return NULL;
    }
    
    // Allocate thread pool structure
    thread_pool_t *pool = calloc(1, sizeof(thread_pool_t));
    if (!pool) {
        log_error("Failed to allocate memory for thread pool");
        return NULL;
    }
    
    // Initialize pool fields
    pool->thread_count = num_threads;
    pool->queue_size = queue_size;
    pool->head = 0;
    pool->tail = 0;
    pool->count = 0;
    pool->shutdown = false;
    
    // Allocate task queue
    pool->queue = calloc(queue_size, sizeof(task_t));
    if (!pool->queue) {
        log_error("Failed to allocate memory for task queue");
        free(pool);
        return NULL;
    }
    
    // Initialize mutex and condition variables
    if (pthread_mutex_init(&pool->mutex, NULL) != 0) {
        log_error("Failed to initialize mutex");
        free(pool->queue);
        free(pool);
        return NULL;
    }
    
    if (pthread_cond_init(&pool->not_empty, NULL) != 0) {
        log_error("Failed to initialize not_empty condition variable");
        pthread_mutex_destroy(&pool->mutex);
        free(pool->queue);
        free(pool);
        return NULL;
    }
    
    if (pthread_cond_init(&pool->not_full, NULL) != 0) {
        log_error("Failed to initialize not_full condition variable");
        pthread_cond_destroy(&pool->not_empty);
        pthread_mutex_destroy(&pool->mutex);
        free(pool->queue);
        free(pool);
        return NULL;
    }
    
    // Allocate thread array
    pool->threads = calloc(num_threads, sizeof(pthread_t));
    if (!pool->threads) {
        log_error("Failed to allocate memory for threads");
        pthread_cond_destroy(&pool->not_full);
        pthread_cond_destroy(&pool->not_empty);
        pthread_mutex_destroy(&pool->mutex);
        free(pool->queue);
        free(pool);
        return NULL;
    }
    
    // Create worker threads
    for (int i = 0; i < num_threads; i++) {
        if (pthread_create(&pool->threads[i], NULL, worker_thread, pool) != 0) {
            log_error("Failed to create worker thread %d", i);
            // Shutdown already created threads
            pool->shutdown = true;
            pthread_cond_broadcast(&pool->not_empty);
            
            // Wait for threads to exit
            for (int j = 0; j < i; j++) {
                pthread_join(pool->threads[j], NULL);
            }
            
            // Clean up resources
            pthread_cond_destroy(&pool->not_full);
            pthread_cond_destroy(&pool->not_empty);
            pthread_mutex_destroy(&pool->mutex);
            free(pool->threads);
            free(pool->queue);
            free(pool);
            return NULL;
        }
    }
    
    log_info("Thread pool initialized with %d threads and queue size %d", 
             num_threads, queue_size);
    return pool;
}

/**
 * @brief Add a task to the thread pool
 * 
 * @param pool Thread pool
 * @param function Function to execute
 * @param argument Argument to pass to the function
 * @return true if task was added successfully, false otherwise
 */
bool thread_pool_add_task(thread_pool_t *pool, void (*function)(void *), void *argument) {
    if (!pool || !function) {
        log_error("Invalid parameters for thread_pool_add_task");
        return false;
    }
    
    // Create task
    task_t task;
    task.function = function;
    task.argument = argument;
    
    // Add task to queue
    pthread_mutex_lock(&pool->mutex);
    
    // Wait until queue is not full
    while (pool->count == pool->queue_size && !pool->shutdown) {
        pthread_cond_wait(&pool->not_full, &pool->mutex);
    }
    
    // Check if pool is shutting down
    if (pool->shutdown) {
        pthread_mutex_unlock(&pool->mutex);
        return false;
    }
    
    // Add task to queue
    bool result = queue_push(pool, task);
    
    // Signal that queue is not empty
    pthread_cond_signal(&pool->not_empty);
    
    pthread_mutex_unlock(&pool->mutex);
    
    return result;
}

/**
 * @brief Shutdown the thread pool
 * 
 * @param pool Thread pool
 */
void thread_pool_shutdown(thread_pool_t *pool) {
    if (!pool) {
        return;
    }
    
    // Set shutdown flag
    pthread_mutex_lock(&pool->mutex);
    pool->shutdown = true;
    pthread_cond_broadcast(&pool->not_empty);
    pthread_mutex_unlock(&pool->mutex);
    
    // Wait for all threads to exit
    for (int i = 0; i < pool->thread_count; i++) {
        pthread_join(pool->threads[i], NULL);
    }
    
    // Clean up resources
    thread_pool_destroy(pool);
}

/**
 * @brief Destroy the thread pool
 * 
 * @param pool Thread pool
 */
void thread_pool_destroy(thread_pool_t *pool) {
    if (!pool) {
        return;
    }
    
    // Free resources
    if (pool->threads) {
        free(pool->threads);
    }
    
    if (pool->queue) {
        free(pool->queue);
    }
    
    pthread_mutex_destroy(&pool->mutex);
    pthread_cond_destroy(&pool->not_empty);
    pthread_cond_destroy(&pool->not_full);
    
    free(pool);
}

/**
 * @brief Worker thread function
 * 
 * @param arg Thread pool
 * @return void* NULL
 */
static void *worker_thread(void *arg) {
    thread_pool_t *pool = (thread_pool_t *)arg;
    task_t task;
    
    while (true) {
        // Wait for task
        pthread_mutex_lock(&pool->mutex);
        
        // Wait until queue is not empty or shutdown
        while (pool->count == 0 && !pool->shutdown) {
            pthread_cond_wait(&pool->not_empty, &pool->mutex);
        }
        
        // Check if pool is shutting down
        if (pool->shutdown && pool->count == 0) {
            pthread_mutex_unlock(&pool->mutex);
            pthread_exit(NULL);
        }
        
        // Get task from queue
        bool result = queue_pop(pool, &task);
        
        // Signal that queue is not full
        pthread_cond_signal(&pool->not_full);
        
        pthread_mutex_unlock(&pool->mutex);
        
        // Execute task
        if (result) {
            task.function(task.argument);
        }
    }
    
    return NULL;
}

/**
 * @brief Push a task to the queue
 * 
 * @param pool Thread pool
 * @param task Task to push
 * @return true if task was pushed successfully, false otherwise
 */
static bool queue_push(thread_pool_t *pool, task_t task) {
    if (pool->count == pool->queue_size) {
        return false;
    }
    
    pool->queue[pool->tail] = task;
    pool->tail = (pool->tail + 1) % pool->queue_size;
    pool->count++;
    
    return true;
}

/**
 * @brief Pop a task from the queue
 * 
 * @param pool Thread pool
 * @param task Task to fill
 * @return true if task was popped successfully, false otherwise
 */
static bool queue_pop(thread_pool_t *pool, task_t *task) {
    if (pool->count == 0) {
        return false;
    }
    
    *task = pool->queue[pool->head];
    pool->head = (pool->head + 1) % pool->queue_size;
    pool->count--;
    
    return true;
}
