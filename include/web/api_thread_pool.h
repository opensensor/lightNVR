#ifndef API_THREAD_POOL_H
#define API_THREAD_POOL_H

#include <pthread.h>
#include <stdbool.h>
#include "../web/thread_pool.h"
#include "../../external/mongoose/mongoose.h"

/**
 * @brief Initialize the API thread pool
 * 
 * @param num_threads Number of worker threads
 * @param queue_size Size of the task queue
 * @return true if successful, false otherwise
 */
bool api_thread_pool_init(int num_threads, int queue_size);

/**
 * @brief Acquire the API thread pool
 * This increments the reference count and initializes the pool if needed
 * 
 * @param num_threads Number of worker threads (used only if pool needs to be initialized)
 * @param queue_size Size of the task queue (used only if pool needs to be initialized)
 * @return thread_pool_t* Pointer to the thread pool or NULL on error
 */
thread_pool_t *api_thread_pool_acquire(int num_threads, int queue_size);

/**
 * @brief Release the API thread pool
 * This decrements the reference count and shuts down the pool if no longer needed
 */
void api_thread_pool_release(void);

/**
 * @brief Shutdown the API thread pool
 * This forces shutdown regardless of reference count
 */
void api_thread_pool_shutdown(void);

/**
 * @brief Add a task to the API thread pool with monitoring
 * This function wraps the task with monitoring code to detect long-running tasks
 * 
 * @param function Function to execute
 * @param argument Argument to pass to the function
 * @return true if task was added successfully, false otherwise
 */
bool api_thread_pool_add_task(void (*function)(void *), void *argument);

/**
 * @brief Get the API thread pool
 * 
 * @return thread_pool_t* Pointer to the thread pool
 */
thread_pool_t *api_thread_pool_get(void);

/**
 * @brief Get the recommended thread pool size from the global config
 * On embedded devices (detected by checking available memory and CPU cores),
 * the thread pool size is limited to 4 regardless of the configured value.
 * 
 * @return int The recommended thread pool size
 */
int api_thread_pool_get_size(void);

/**
 * @brief Structure for ONVIF discovery task
 */
typedef struct {
    struct mg_connection *connection;  // Mongoose connection
    char *network;                     // Network to discover on (can be NULL for auto-detection)
    char *json_str;                    // JSON request string (for parsing parameters)
} onvif_discovery_task_t;

/**
 * @brief Create an ONVIF discovery task
 * 
 * @param c Mongoose connection
 * @param network Network to discover on (can be NULL for auto-detection)
 * @param json_str JSON request string (for parsing parameters)
 * @return onvif_discovery_task_t* Pointer to the task or NULL on error
 */
onvif_discovery_task_t *onvif_discovery_task_create(struct mg_connection *c, const char *network, const char *json_str);

/**
 * @brief Free an ONVIF discovery task
 * 
 * @param task Task to free
 */
void onvif_discovery_task_free(onvif_discovery_task_t *task);

/**
 * @brief ONVIF discovery task function
 * 
 * @param arg Task argument (onvif_discovery_task_t*)
 */
void onvif_discovery_task_function(void *arg);

#endif /* API_THREAD_POOL_H */
