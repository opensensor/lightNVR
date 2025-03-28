#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>
#include <unistd.h>
#include <sys/time.h>

#include "core/logger.h"
#include "web/api_handlers_system_ws.h"
#include "web/logger_websocket.h"
#include "web/websocket_manager.h"

// Forward declarations with weak symbols
extern __attribute__((weak)) websocket_message_t *websocket_message_create(const char *type, const char *topic, const char *payload);
extern __attribute__((weak)) bool websocket_manager_send_to_client(const char *client_id, const websocket_message_t *message);
extern __attribute__((weak)) void websocket_message_free(websocket_message_t *message);
extern __attribute__((weak)) int fetch_system_logs(const char *client_id, const char *min_level, const char *last_timestamp);

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
 * Instead of broadcasting to all clients, it does nothing - clients will
 * poll for logs when they need them using the fetch_system_logs function.
 * This reduces unnecessary network traffic and processing.
 */
void broadcast_logs_to_websocket(void) {
    // This is intentionally empty - we're using a polling approach instead of broadcasting
    // Clients will request logs when they need them via fetch_system_logs
    
    // No need to rate limit or check for minimum interval since we're not doing anything
    
    // Log at debug level to avoid filling logs with this message
    // log_debug("broadcast_logs_to_websocket called but using polling approach instead");
}
