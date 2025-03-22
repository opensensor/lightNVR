#ifndef LIGHTNVR_CONNECTION_POOL_H
#define LIGHTNVR_CONNECTION_POOL_H

#include <pthread.h>
#include <stdbool.h>
#include "mongoose.h"
#include "web/http_server.h"

// Forward declaration for http_server_t
typedef struct http_server http_server_t;

/**
 * @brief Connection node structure for the connection queue
 */
typedef struct conn_node {
    struct mg_connection *connection;  // Mongoose connection
    http_server_t *server;             // HTTP server
    struct conn_node *next;            // Next node in the queue
} conn_node_t;

/**
 * @brief Connection pool structure
 * 
 * This structure manages a pool of worker threads that handle connections.
 * Each connection is assigned to a worker thread, which handles all events
 * for that connection until it is closed.
 */
typedef struct connection_pool {
    pthread_t *threads;           // Array of worker threads
    int thread_count;             // Number of worker threads
    pthread_mutex_t mutex;        // Mutex to protect the queue
    pthread_cond_t cond;          // Condition variable for signaling
    conn_node_t *conn_queue;      // Queue of connections to be processed
    bool shutdown;                // Shutdown flag
} connection_pool_t;

/**
 * @brief Initialize a connection pool
 * 
 * @param num_threads Number of worker threads
 * @return connection_pool_t* Pointer to the connection pool or NULL on error
 */
connection_pool_t *connection_pool_init(int num_threads);

/**
 * @brief Add a connection to the connection pool
 * 
 * @param pool Connection pool
 * @param c Mongoose connection
 * @param server HTTP server
 * @return true if connection was added successfully, false otherwise
 */
bool connection_pool_add_connection(connection_pool_t *pool, struct mg_connection *c, http_server_t *server);

/**
 * @brief Shutdown the connection pool
 * 
 * @param pool Connection pool
 */
void connection_pool_shutdown(connection_pool_t *pool);

/**
 * @brief Destroy the connection pool
 * 
 * @param pool Connection pool
 */
void connection_pool_destroy(connection_pool_t *pool);

/**
 * @brief Worker thread function (internal)
 * 
 * @param arg Connection pool
 * @return void* NULL
 */
void *connection_worker_thread(void *arg);

#endif // LIGHTNVR_CONNECTION_POOL_H
