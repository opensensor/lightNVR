#define _GNU_SOURCE  // For pthread_timedjoin_np
#include "web/connection_pool.h"
#include "core/logger.h"
#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>
#include <stdbool.h>
#include <time.h>
#include <unistd.h>
#include <errno.h>

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
    
    log_info("Starting connection pool shutdown sequence");
    
    // Set shutdown flag and mark all connections as closing
    pthread_mutex_lock(&pool->mutex);
    pool->shutdown = true;
    
    // Mark all connections in the queue as closing
    conn_node_t *current = pool->conn_queue;
    int conn_count = 0;
    while (current != NULL) {
        if (current->connection) {
            current->connection->is_closing = 1;
            
            //  Explicitly close the socket to ensure it's released
            if (current->connection->fd != NULL) {
                int socket_fd = (int)(size_t)current->connection->fd;
                log_debug("Closing connection socket: %d", socket_fd);
                
                // Set SO_LINGER to force immediate socket closure
                struct linger so_linger;
                so_linger.l_onoff = 1;
                so_linger.l_linger = 0;
                setsockopt(socket_fd, SOL_SOCKET, SO_LINGER, &so_linger, sizeof(so_linger));
                
                // Set socket to non-blocking mode to avoid hang on close
                int flags = fcntl(socket_fd, F_GETFL, 0);
                fcntl(socket_fd, F_SETFL, flags | O_NONBLOCK);
                
                // Now close the socket
                close(socket_fd);
                current->connection->fd = NULL;  // Mark as closed
            }
            
            conn_count++;
        }
        current = current->next;
    }
    log_debug("Marked %d connections for closing", conn_count);
    
    // Signal all threads to wake up and check the shutdown flag
    pthread_cond_broadcast(&pool->cond);
    pthread_mutex_unlock(&pool->mutex);
    
    log_debug("Waiting for %d worker threads to exit", pool->thread_count);
    
    // Wait for all threads to exit with a timeout
    for (int i = 0; i < pool->thread_count; i++) {
        // Use a timeout to avoid hanging if a thread is stuck
        struct timespec ts;
        clock_gettime(CLOCK_REALTIME, &ts);
        ts.tv_sec += 8; // Increased to 8 second timeout for better reliability
        
        int ret = pthread_timedjoin_np(pool->threads[i], NULL, &ts);
        if (ret != 0) {
            log_warn("Thread %d did not exit within timeout, continuing anyway", i);
            
            //  Try to cancel the thread if it didn't exit within timeout
            pthread_cancel(pool->threads[i]);
            
            // Give it a short time to clean up after cancellation
            usleep(100000); // 100ms
        }
    }
    
    log_debug("All worker threads have exited or timed out");
    
    //  Add a delay before destroying resources
    // This helps ensure all threads have fully exited
    usleep(500000); // 500ms
    
    // Clean up resources
    connection_pool_destroy(pool);
    
    log_info("Connection pool shutdown complete");
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
    
    log_debug("Destroying connection pool resources");
    
    // Acquire the mutex one last time to ensure no other thread is accessing the pool
    pthread_mutex_lock(&pool->mutex);
    
    // Free connection queue first
    conn_node_t *current = pool->conn_queue;
    int nodes_freed = 0;
    while (current != NULL) {
        conn_node_t *next = current->next;
        // Don't free the connection or server, they're managed elsewhere
        current->connection = NULL;
        current->server = NULL;
        free(current);
        nodes_freed++;
        current = next;
    }
    pool->conn_queue = NULL;
    log_debug("Freed %d connection nodes", nodes_freed);
    
    // Free threads array
    if (pool->threads) {
        free(pool->threads);
        pool->threads = NULL;
    }
    
    // Release the mutex before destroying it
    pthread_mutex_unlock(&pool->mutex);
    
    // Destroy synchronization primitives
    pthread_cond_destroy(&pool->cond);
    pthread_mutex_destroy(&pool->mutex);
    
    // Finally free the pool structure
    free(pool);
    log_debug("Connection pool resources destroyed");
}

/**
 * @brief Worker thread function
 * 
 * @param arg Connection pool
 * @return void* NULL
 */
void *connection_worker_thread(void *arg) {
    connection_pool_t *pool = (connection_pool_t *)arg;
    int thread_id = -1;
    
    // Get a unique thread ID for logging
    pthread_mutex_lock(&pool->mutex);
    for (int i = 0; i < pool->thread_count; i++) {
        if (pthread_equal(pthread_self(), pool->threads[i])) {
            thread_id = i;
            break;
        }
    }
    pthread_mutex_unlock(&pool->mutex);
    
    log_debug("Connection worker thread %d started", thread_id);
    
    while (true) {
        // Wait for connection
        pthread_mutex_lock(&pool->mutex);
        
        // Check if pool is shutting down immediately
        if (pool->shutdown) {
            pthread_mutex_unlock(&pool->mutex);
            log_info("Connection worker thread %d exiting due to shutdown", thread_id);
            pthread_exit(NULL);
        }
        
        // Wait until a connection is available or shutdown
        while (pool->conn_queue == NULL && !pool->shutdown) {
            pthread_cond_wait(&pool->cond, &pool->mutex);
        }
        
        // Check again if pool is shutting down after waiting
        if (pool->shutdown) {
            pthread_mutex_unlock(&pool->mutex);
            log_info("Connection worker thread %d exiting after wait due to shutdown", thread_id);
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
            if (c) {
                c->fn_data = server;
                
                // Mark this connection as being handled by a worker thread
                c->is_resp = 1;
            }
            
            // Free the node (but not the connection)
            free(node);
            node = NULL;
            
            // The connection will remain associated with this thread until closed
            // We don't need to do anything here, as the main event loop will handle all events
            // and call the event handler for this connection
            
            // Wait for the connection to be closed, but also check the shutdown flag
            // Use a volatile local copy of the connection pointer to prevent use-after-free
            struct mg_connection *volatile conn = c;
            while (conn && conn->is_closing == 0) {
                // Check if the pool is shutting down
                bool shutdown = false;
                
                pthread_mutex_lock(&pool->mutex);
                shutdown = pool->shutdown;
                pthread_mutex_unlock(&pool->mutex);
                
                if (shutdown) {
                    // Mark the connection as closing and break the loop
                    if (conn) {
                        conn->is_closing = 1;
                    }
                    break;
                }
                
                // Don't poll the connection directly, just sleep
                // The main event loop will handle events for this connection
                usleep(1000); // 1ms
            }
            
            log_debug("Connection closed by worker thread %d", thread_id);
        }
    }
    
    return NULL;
}
