#include "web/connection_pool.h"
#include "core/logger.h"
#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>
#include <stdbool.h>

/**
 * @brief Initialize a connection pool
 * 
 * @param num_threads Number of worker threads
 * @return connection_pool_t* Pointer to the connection pool or NULL on error
 */
connection_pool_t *connection_pool_init(int num_threads) {
    if (num_threads <= 0) {
        log_error("Invalid connection pool parameters: num_threads=%d", num_threads);
        return NULL;
    }
    
    // Allocate connection pool structure
    connection_pool_t *pool = calloc(1, sizeof(connection_pool_t));
    if (!pool) {
        log_error("Failed to allocate memory for connection pool");
        return NULL;
    }
    
    // Initialize pool fields
    pool->thread_count = num_threads;
    pool->shutdown = false;
    
    // Initialize mutex and condition variables
    if (pthread_mutex_init(&pool->mutex, NULL) != 0) {
        log_error("Failed to initialize mutex");
        free(pool);
        return NULL;
    }
    
    if (pthread_cond_init(&pool->cond, NULL) != 0) {
        log_error("Failed to initialize condition variable");
        pthread_mutex_destroy(&pool->mutex);
        free(pool);
        return NULL;
    }
    
    // Initialize connection queue
    pool->conn_queue = NULL;
    
    // Allocate thread array
    pool->threads = calloc(num_threads, sizeof(pthread_t));
    if (!pool->threads) {
        log_error("Failed to allocate memory for threads");
        pthread_cond_destroy(&pool->cond);
        pthread_mutex_destroy(&pool->mutex);
        free(pool);
        return NULL;
    }
    
    // Create worker threads
    for (int i = 0; i < num_threads; i++) {
        if (pthread_create(&pool->threads[i], NULL, connection_worker_thread, pool) != 0) {
            log_error("Failed to create worker thread %d", i);
            // Shutdown already created threads
            pool->shutdown = true;
            pthread_cond_broadcast(&pool->cond);
            
            // Wait for threads to exit
            for (int j = 0; j < i; j++) {
                pthread_join(pool->threads[j], NULL);
            }
            
            // Clean up resources
            pthread_cond_destroy(&pool->cond);
            pthread_mutex_destroy(&pool->mutex);
            free(pool->threads);
            free(pool);
            return NULL;
        }
    }
    
    log_info("Connection pool initialized with %d threads", num_threads);
    return pool;
}

/**
 * @brief Add a connection to the connection pool
 * 
 * @param pool Connection pool
 * @param c Mongoose connection
 * @param server HTTP server
 * @return true if connection was added successfully, false otherwise
 */
bool connection_pool_add_connection(connection_pool_t *pool, struct mg_connection *c, http_server_t *server) {
    if (!pool || !c || !server) {
        log_error("Invalid parameters for connection_pool_add_connection");
        return false;
    }
    
    // Create connection node
    conn_node_t *node = calloc(1, sizeof(conn_node_t));
    if (!node) {
        log_error("Failed to allocate memory for connection node");
        return false;
    }
    
    node->connection = c;
    node->server = server;
    node->next = NULL;
    
    // Add connection to queue
    pthread_mutex_lock(&pool->mutex);
    
    // Check if pool is shutting down
    if (pool->shutdown) {
        pthread_mutex_unlock(&pool->mutex);
        free(node);
        return false;
    }
    
    // Add to queue (at the end)
    if (pool->conn_queue == NULL) {
        pool->conn_queue = node;
    } else {
        conn_node_t *current = pool->conn_queue;
        while (current->next != NULL) {
            current = current->next;
        }
        current->next = node;
    }
    
    // Signal that a new connection is available
    pthread_cond_signal(&pool->cond);
    
    pthread_mutex_unlock(&pool->mutex);
    
    return true;
}

/**
 * @brief Shutdown the connection pool
 * 
 * @param pool Connection pool
 */
void connection_pool_shutdown(connection_pool_t *pool) {
    if (!pool) {
        return;
    }
    
    // Set shutdown flag
    pthread_mutex_lock(&pool->mutex);
    pool->shutdown = true;
    pthread_cond_broadcast(&pool->cond);
    pthread_mutex_unlock(&pool->mutex);
    
    // Wait for all threads to exit
    for (int i = 0; i < pool->thread_count; i++) {
        pthread_join(pool->threads[i], NULL);
    }
    
    // Clean up resources
    connection_pool_destroy(pool);
}

/**
 * @brief Destroy the connection pool
 * 
 * @param pool Connection pool
 */
void connection_pool_destroy(connection_pool_t *pool) {
    if (!pool) {
        return;
    }
    
    // Free resources
    if (pool->threads) {
        free(pool->threads);
    }
    
    // Free connection queue
    conn_node_t *current = pool->conn_queue;
    while (current != NULL) {
        conn_node_t *next = current->next;
        free(current);
        current = next;
    }
    
    pthread_mutex_destroy(&pool->mutex);
    pthread_cond_destroy(&pool->cond);
    
    free(pool);
}

/**
 * @brief Worker thread function
 * 
 * @param arg Connection pool
 * @return void* NULL
 */
void *connection_worker_thread(void *arg) {
    connection_pool_t *pool = (connection_pool_t *)arg;
    
    while (true) {
        // Wait for connection
        pthread_mutex_lock(&pool->mutex);
        
        // Wait until a connection is available or shutdown
        while (pool->conn_queue == NULL && !pool->shutdown) {
            pthread_cond_wait(&pool->cond, &pool->mutex);
        }
        
        // Check if pool is shutting down
        if (pool->shutdown && pool->conn_queue == NULL) {
            pthread_mutex_unlock(&pool->mutex);
            pthread_exit(NULL);
        }
        
        // Get connection from queue
        conn_node_t *node = pool->conn_queue;
        if (node != NULL) {
            pool->conn_queue = node->next;
        }
        
        pthread_mutex_unlock(&pool->mutex);
        
        // Process connection if available
        if (node != NULL) {
            struct mg_connection *c = node->connection;
            http_server_t *server = node->server;
            
            // Set connection data to indicate it's being handled by this thread
            c->fn_data = server;
            
            // Free the node (but not the connection)
            free(node);
            
            // Handle the connection events in this thread
            // The connection will remain associated with this thread until closed
            while (c->is_closing == 0) {
                // Process any pending events for this connection
                // This is where we would handle HTTP requests, etc.
                // For now, we'll just sleep a bit to avoid busy-waiting
                usleep(10000); // 10ms
                
                // Instead of polling individual connections, we'll just sleep
                // The main event loop will handle all events
                usleep(10000); // 10ms
            }
            
            log_debug("Connection closed by worker thread");
        }
    }
    
    return NULL;
}
