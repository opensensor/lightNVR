#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>

#include "web/mongoose_server_thread.h"
#include "core/logger.h"
#include "mongoose.h"

/**
 * @brief Initialize the connection mutex pool
 * 
 * @param pool Pointer to the connection mutex pool
 * @param size Number of mutexes in the pool
 * @return int 0 on success, -1 on error
 */
int connection_mutex_pool_init(connection_mutex_pool_t *pool, int size) {
    if (!pool || size <= 0) {
        log_error("Invalid parameters for connection_mutex_pool_init");
        return -1;
    }
    
    // Allocate mutex array
    pool->mutexes = calloc(size, sizeof(pthread_mutex_t));
    if (!pool->mutexes) {
        log_error("Failed to allocate memory for connection mutex pool");
        return -1;
    }
    
    // Initialize mutexes
    for (int i = 0; i < size; i++) {
        if (pthread_mutex_init(&pool->mutexes[i], NULL) != 0) {
            log_error("Failed to initialize mutex %d", i);
            
            // Clean up already initialized mutexes
            for (int j = 0; j < i; j++) {
                pthread_mutex_destroy(&pool->mutexes[j]);
            }
            
            free(pool->mutexes);
            pool->mutexes = NULL;
            return -1;
        }
    }
    
    // Initialize pool mutex
    if (pthread_mutex_init(&pool->pool_mutex, NULL) != 0) {
        log_error("Failed to initialize pool mutex");
        
        // Clean up mutexes
        for (int i = 0; i < size; i++) {
            pthread_mutex_destroy(&pool->mutexes[i]);
        }
        
        free(pool->mutexes);
        pool->mutexes = NULL;
        return -1;
    }
    
    pool->size = size;
    pool->next_index = 0;
    
    log_info("Connection mutex pool initialized with %d mutexes", size);
    return 0;
}

/**
 * @brief Get a mutex for a connection
 * 
 * @param pool Pointer to the connection mutex pool
 * @param conn_id Connection ID (pointer value can be used)
 * @return pthread_mutex_t* Pointer to the mutex or NULL on error
 */
pthread_mutex_t *connection_mutex_pool_get(connection_mutex_pool_t *pool, uintptr_t conn_id) {
    if (!pool || !pool->mutexes) {
        log_error("Invalid connection mutex pool");
        return NULL;
    }
    
    // Use connection ID to determine which mutex to use
    // This ensures the same connection always gets the same mutex
    // No need to lock the pool mutex for this simple calculation
    int index = conn_id % pool->size;
    
    return &pool->mutexes[index];
}

/**
 * @brief Destroy the connection mutex pool
 * 
 * @param pool Pointer to the connection mutex pool
 */
void connection_mutex_pool_destroy(connection_mutex_pool_t *pool) {
    if (!pool || !pool->mutexes) {
        return;
    }
    
    // Destroy all mutexes
    for (int i = 0; i < pool->size; i++) {
        pthread_mutex_destroy(&pool->mutexes[i]);
    }
    
    // Destroy pool mutex
    pthread_mutex_destroy(&pool->pool_mutex);
    
    // Free memory
    free(pool->mutexes);
    pool->mutexes = NULL;
    pool->size = 0;
    
    log_info("Connection mutex pool destroyed");
}

/**
 * @brief Lock a connection mutex
 * 
 * @param pool Pointer to the connection mutex pool
 * @param conn_id Connection ID
 * @return int 0 on success, -1 on error
 */
int connection_mutex_pool_lock(connection_mutex_pool_t *pool, uintptr_t conn_id) {
    pthread_mutex_t *mutex = connection_mutex_pool_get(pool, conn_id);
    if (!mutex) {
        return -1;
    }
    
    return pthread_mutex_lock(mutex);
}

/**
 * @brief Unlock a connection mutex
 * 
 * @param pool Pointer to the connection mutex pool
 * @param conn_id Connection ID
 * @return int 0 on success, -1 on error
 */
int connection_mutex_pool_unlock(connection_mutex_pool_t *pool, uintptr_t conn_id) {
    pthread_mutex_t *mutex = connection_mutex_pool_get(pool, conn_id);
    if (!mutex) {
        return -1;
    }
    
    return pthread_mutex_unlock(mutex);
}
