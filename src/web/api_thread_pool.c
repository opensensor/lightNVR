#include "web/api_thread_pool.h"
#include "core/logger.h"
#include "video/onvif_discovery.h"
#include "web/api_handlers.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "cJSON.h"

// Global API thread pool
static thread_pool_t *g_api_thread_pool = NULL;

// Reference count for the thread pool
static int g_api_thread_pool_ref_count = 0;

// Mutex to protect the reference count
static pthread_mutex_t g_api_thread_pool_mutex = PTHREAD_MUTEX_INITIALIZER;

/**
 * @brief Initialize the API thread pool
 * 
 * @param num_threads Number of worker threads
 * @param queue_size Size of the task queue
 * @return true if successful, false otherwise
 */
bool api_thread_pool_init(int num_threads, int queue_size) {
    pthread_mutex_lock(&g_api_thread_pool_mutex);
    
    if (g_api_thread_pool != NULL) {
        log_warn("API thread pool already initialized");
        pthread_mutex_unlock(&g_api_thread_pool_mutex);
        return true;
    }
    
    g_api_thread_pool = thread_pool_init(num_threads, queue_size);
    if (g_api_thread_pool == NULL) {
        log_error("Failed to initialize API thread pool");
        pthread_mutex_unlock(&g_api_thread_pool_mutex);
        return false;
    }
    
    // Reset reference count
    g_api_thread_pool_ref_count = 0;
    
    log_info("API thread pool initialized with %d threads and queue size %d", 
             num_threads, queue_size);
    
    pthread_mutex_unlock(&g_api_thread_pool_mutex);
    return true;
}

/**
 * @brief Acquire the API thread pool
 * This increments the reference count and initializes the pool if needed
 * 
 * @param num_threads Number of worker threads (used only if pool needs to be initialized)
 * @param queue_size Size of the task queue (used only if pool needs to be initialized)
 * @return thread_pool_t* Pointer to the thread pool or NULL on error
 */
thread_pool_t *api_thread_pool_acquire(int num_threads, int queue_size) {
    pthread_mutex_lock(&g_api_thread_pool_mutex);
    
    // Initialize the thread pool if it doesn't exist
    if (g_api_thread_pool == NULL) {
        log_info("Initializing API thread pool on demand");
        g_api_thread_pool = thread_pool_init(num_threads, queue_size);
        if (g_api_thread_pool == NULL) {
            log_error("Failed to initialize API thread pool on demand");
            pthread_mutex_unlock(&g_api_thread_pool_mutex);
            return NULL;
        }
        log_info("API thread pool initialized with %d threads and queue size %d", 
                 num_threads, queue_size);
    }
    
    // Increment reference count
    g_api_thread_pool_ref_count++;
    log_debug("API thread pool acquired, reference count: %d", g_api_thread_pool_ref_count);
    
    thread_pool_t *pool = g_api_thread_pool;
    pthread_mutex_unlock(&g_api_thread_pool_mutex);
    
    return pool;
}

/**
 * @brief Release the API thread pool
 * This decrements the reference count and shuts down the pool if no longer needed
 */
void api_thread_pool_release(void) {
    pthread_mutex_lock(&g_api_thread_pool_mutex);
    
    if (g_api_thread_pool == NULL) {
        log_warn("Attempting to release non-existent API thread pool");
        pthread_mutex_unlock(&g_api_thread_pool_mutex);
        return;
    }
    
    // Decrement reference count
    if (g_api_thread_pool_ref_count > 0) {
        g_api_thread_pool_ref_count--;
    }
    
    log_debug("API thread pool released, reference count: %d", g_api_thread_pool_ref_count);
    
    // Shutdown the thread pool if no longer needed
    if (g_api_thread_pool_ref_count == 0) {
        log_info("Shutting down API thread pool as it's no longer needed");
        thread_pool_shutdown(g_api_thread_pool);
        g_api_thread_pool = NULL;
    }
    
    pthread_mutex_unlock(&g_api_thread_pool_mutex);
}

/**
 * @brief Shutdown the API thread pool
 * This forces shutdown regardless of reference count
 */
void api_thread_pool_shutdown(void) {
    pthread_mutex_lock(&g_api_thread_pool_mutex);
    
    if (g_api_thread_pool != NULL) {
        log_info("Forcing shutdown of API thread pool, reference count was: %d", 
                 g_api_thread_pool_ref_count);
        thread_pool_shutdown(g_api_thread_pool);
        g_api_thread_pool = NULL;
        g_api_thread_pool_ref_count = 0;
    }
    
    pthread_mutex_unlock(&g_api_thread_pool_mutex);
}

/**
 * @brief Get the API thread pool
 * 
 * @return thread_pool_t* Pointer to the thread pool
 */
thread_pool_t *api_thread_pool_get(void) {
    return g_api_thread_pool;
}

/**
 * @brief Create an ONVIF discovery task
 * 
 * @param c Mongoose connection
 * @param network Network to discover on (can be NULL for auto-detection)
 * @param json_str JSON request string (for parsing parameters)
 * @return onvif_discovery_task_t* Pointer to the task or NULL on error
 */
onvif_discovery_task_t *onvif_discovery_task_create(struct mg_connection *c, const char *network, const char *json_str) {
    onvif_discovery_task_t *task = calloc(1, sizeof(onvif_discovery_task_t));
    if (!task) {
        log_error("Failed to allocate memory for ONVIF discovery task");
        return NULL;
    }
    
    task->connection = c;
    
    if (network) {
        task->network = strdup(network);
        if (!task->network) {
            log_error("Failed to allocate memory for network");
            free(task);
            return NULL;
        }
    } else {
        task->network = NULL;
    }
    
    if (json_str) {
        task->json_str = strdup(json_str);
        if (!task->json_str) {
            log_error("Failed to allocate memory for JSON string");
            if (task->network) {
                free(task->network);
            }
            free(task);
            return NULL;
        }
    } else {
        task->json_str = NULL;
    }
    
    return task;
}

/**
 * @brief Free an ONVIF discovery task
 * 
 * @param task Task to free
 */
void onvif_discovery_task_free(onvif_discovery_task_t *task) {
    if (task) {
        if (task->network) {
            free(task->network);
        }
        if (task->json_str) {
            free(task->json_str);
        }
        free(task);
    }
}

/**
 * @brief ONVIF discovery task function
 * 
 * @param arg Task argument (onvif_discovery_task_t*)
 */
void onvif_discovery_task_function(void *arg) {
    onvif_discovery_task_t *task = (onvif_discovery_task_t *)arg;
    if (!task) {
        log_error("Invalid ONVIF discovery task");
        return;
    }
    
    struct mg_connection *c = task->connection;
    if (!c) {
        log_error("Invalid Mongoose connection");
        onvif_discovery_task_free(task);
        return;
    }
    
    // Release the thread pool when this task is done
    // This ensures the thread pool is properly cleaned up when no longer needed
    bool release_needed = true;
    
    // Parse JSON request if available
    const char *network = task->network;
    if (!network && task->json_str) {
        cJSON *root = cJSON_Parse(task->json_str);
        if (root) {
            cJSON *network_json = cJSON_GetObjectItem(root, "network");
            if (network_json && cJSON_IsString(network_json)) {
                network = network_json->valuestring;
                
                // Check if network is "auto" or empty, which means auto-detect
                if (strcmp(network, "auto") == 0 || strlen(network) == 0) {
                    network = NULL; // This will trigger auto-detection
                }
            }
            
            // Don't free the root yet, we need the network string
        }
    }
    
    // If network is NULL, we'll use auto-detection
    if (!network) {
        log_info("Network parameter not provided or set to 'auto', will use auto-detection");
    }
    
    // Discover ONVIF devices
    onvif_device_info_t devices[32];
    int count = discover_onvif_devices(network, devices, 32);
    
    // Free JSON root if we parsed it
    if (task->json_str) {
        cJSON *root = cJSON_Parse(task->json_str);
        if (root) {
            cJSON_Delete(root);
        }
    }
    
    if (count < 0) {
        log_error("Failed to discover ONVIF devices");
        mg_send_json_error(c, 500, "Failed to discover ONVIF devices");
        onvif_discovery_task_free(task);
        return;
    }
    
    // Create JSON response
    cJSON *response_json = cJSON_CreateObject();
    if (!response_json) {
        log_error("Failed to create JSON response");
        mg_send_json_error(c, 500, "Failed to create JSON response");
        onvif_discovery_task_free(task);
        return;
    }
    
    cJSON *devices_array = cJSON_AddArrayToObject(response_json, "devices");
    if (!devices_array) {
        log_error("Failed to create JSON response");
        cJSON_Delete(response_json);
        mg_send_json_error(c, 500, "Failed to create JSON response");
        onvif_discovery_task_free(task);
        return;
    }
    
    // Add devices to array
    for (int i = 0; i < count; i++) {
        cJSON *device = cJSON_CreateObject();
        if (!device) {
            log_error("Failed to create JSON response");
            cJSON_Delete(response_json);
            mg_send_json_error(c, 500, "Failed to create JSON response");
            onvif_discovery_task_free(task);
            return;
        }
        
        cJSON_AddStringToObject(device, "endpoint", devices[i].endpoint);
        cJSON_AddStringToObject(device, "device_service", devices[i].device_service);
        cJSON_AddStringToObject(device, "media_service", devices[i].media_service);
        cJSON_AddStringToObject(device, "ptz_service", devices[i].ptz_service);
        cJSON_AddStringToObject(device, "imaging_service", devices[i].imaging_service);
        cJSON_AddStringToObject(device, "manufacturer", devices[i].manufacturer);
        cJSON_AddStringToObject(device, "model", devices[i].model);
        cJSON_AddStringToObject(device, "firmware_version", devices[i].firmware_version);
        cJSON_AddStringToObject(device, "serial_number", devices[i].serial_number);
        cJSON_AddStringToObject(device, "hardware_id", devices[i].hardware_id);
        cJSON_AddStringToObject(device, "ip_address", devices[i].ip_address);
        cJSON_AddStringToObject(device, "mac_address", devices[i].mac_address);
        cJSON_AddNumberToObject(device, "discovery_time", (double)devices[i].discovery_time);
        cJSON_AddBoolToObject(device, "online", devices[i].online);
        
        cJSON_AddItemToArray(devices_array, device);
    }
    
    // Convert to string
    char *json_str = cJSON_PrintUnformatted(response_json);
    cJSON_Delete(response_json);
    
    if (!json_str) {
        log_error("Failed to generate JSON response");
        mg_send_json_error(c, 500, "Failed to generate JSON response");
        onvif_discovery_task_free(task);
        return;
    }
    
    // Send response
    mg_send_json_response(c, 200, json_str);
    
    // Clean up
    free(json_str);
    onvif_discovery_task_free(task);
    
    // Release the thread pool if needed
    if (release_needed) {
        api_thread_pool_release();
    }
    
    log_info("Successfully handled POST /api/onvif/discovery/discover request");
}
