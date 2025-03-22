#ifndef LIGHTNVR_MONGOOSE_SERVER_THREAD_H
#define LIGHTNVR_MONGOOSE_SERVER_THREAD_H

#include <pthread.h>
#include <stdint.h>

/**
 * @brief Connection mutex pool structure
 * 
 * This structure provides a pool of mutexes that can be used to synchronize
 * access to connections in a multi-threaded environment. Instead of using a
 * single mutex for all connections, which can cause contention, or a mutex
 * per connection, which can use too much memory, this pool provides a fixed
 * number of mutexes that are assigned to connections based on their ID.
 */
struct connection_mutex_pool {
    pthread_mutex_t *mutexes;  // Array of mutexes
    int size;                  // Number of mutexes in the pool
    int next_index;            // Next index to use (for round-robin allocation)
    pthread_mutex_t pool_mutex; // Mutex to protect the pool itself
};

typedef struct connection_mutex_pool connection_mutex_pool_t;

/**
 * @brief Initialize the connection mutex pool
 * 
 * @param pool Pointer to the connection mutex pool
 * @param size Number of mutexes in the pool
 * @return int 0 on success, -1 on error
 */
int connection_mutex_pool_init(connection_mutex_pool_t *pool, int size);

/**
 * @brief Get a mutex for a connection
 * 
 * @param pool Pointer to the connection mutex pool
 * @param conn_id Connection ID (pointer value can be used)
 * @return pthread_mutex_t* Pointer to the mutex or NULL on error
 */
pthread_mutex_t *connection_mutex_pool_get(connection_mutex_pool_t *pool, uintptr_t conn_id);

/**
 * @brief Destroy the connection mutex pool
 * 
 * @param pool Pointer to the connection mutex pool
 */
void connection_mutex_pool_destroy(connection_mutex_pool_t *pool);

/**
 * @brief Lock a connection mutex
 * 
 * @param pool Pointer to the connection mutex pool
 * @param conn_id Connection ID
 * @return int 0 on success, -1 on error
 */
int connection_mutex_pool_lock(connection_mutex_pool_t *pool, uintptr_t conn_id);

/**
 * @brief Unlock a connection mutex
 * 
 * @param pool Pointer to the connection mutex pool
 * @param conn_id Connection ID
 * @return int 0 on success, -1 on error
 */
int connection_mutex_pool_unlock(connection_mutex_pool_t *pool, uintptr_t conn_id);

#endif // LIGHTNVR_MONGOOSE_SERVER_THREAD_H
