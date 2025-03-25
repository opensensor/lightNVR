#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>
#include <unistd.h>
#include <sys/time.h>

#include "core/logger.h"
#include "web/api_handlers_system_ws.h"
#include "web/logger_websocket.h"

// Mutex to protect log broadcasting
static pthread_mutex_t broadcast_mutex = PTHREAD_MUTEX_INITIALIZER;

// Time of last broadcast
static struct timeval last_broadcast_time = {0, 0};

// Minimum interval between broadcasts (in microseconds)
// 500ms = 500,000 microseconds
#define MIN_BROADCAST_INTERVAL 500000

/**
 * @brief Initialize logger WebSocket integration
 */
void init_logger_websocket(void) {
    // Initialize mutex
    pthread_mutex_init(&broadcast_mutex, NULL);
    
    // Initialize last broadcast time
    gettimeofday(&last_broadcast_time, NULL);
    
    log_info("Logger WebSocket integration initialized");
}

/**
 * @brief Shutdown logger WebSocket integration
 */
void shutdown_logger_websocket(void) {
    pthread_mutex_destroy(&broadcast_mutex);
    
    log_info("Logger WebSocket integration shutdown");
}

/**
 * @brief Broadcast system logs to WebSocket clients
 * 
 * This function is called by the logger after a log message is written.
 * It broadcasts the logs to all WebSocket clients subscribed to the system/logs topic.
 * To avoid excessive broadcasts, it only broadcasts at most once every MIN_BROADCAST_INTERVAL.
 */
void broadcast_logs_to_websocket(void) {
    // Check if we should broadcast
    struct timeval now;
    gettimeofday(&now, NULL);
    
    // Calculate time difference in microseconds
    long time_diff = (now.tv_sec - last_broadcast_time.tv_sec) * 1000000 + (now.tv_usec - last_broadcast_time.tv_usec);
    
    // Only broadcast if enough time has passed since the last broadcast
    if (time_diff < MIN_BROADCAST_INTERVAL) {
        return;
    }
    
    // Try to acquire mutex
    if (pthread_mutex_trylock(&broadcast_mutex) != 0) {
        // Another thread is already broadcasting, skip this one
        return;
    }
    
    // Update last broadcast time
    gettimeofday(&last_broadcast_time, NULL);
    
    // Broadcast logs to WebSocket clients
    websocket_broadcast_system_logs();
    
    // Release mutex
    pthread_mutex_unlock(&broadcast_mutex);
}
