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
 * Instead of broadcasting to all clients, it now only updates the last broadcast time
 * so that when clients request logs, they get the latest data.
 * This reduces memory and CPU overhead, especially on embedded devices.
 */
void broadcast_logs_to_websocket(void) {
    // Just update the last broadcast time
    // Actual log sending will happen when clients request it
    gettimeofday(&last_broadcast_time, NULL);
    
    // No broadcasting here - clients will request logs when needed
    // This significantly reduces memory and CPU overhead on embedded devices
}
