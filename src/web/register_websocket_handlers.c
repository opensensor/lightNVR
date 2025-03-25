#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "web/websocket_manager.h"
#include "web/api_handlers_recordings_batch_ws.h"
#include "web/api_handlers_system_ws.h"
#include "core/logger.h"

/**
 * @brief Register all WebSocket handlers
 * 
 * @return int 0 on success, non-zero on error
 */
int register_websocket_handlers(void) {
    log_info("Registering WebSocket handlers");
    
    // Initialize WebSocket manager if not already initialized
    if (!websocket_manager_is_initialized()) {
        log_info("WebSocket manager not initialized, initializing now");
        if (websocket_manager_init() != 0) {
            log_error("Failed to initialize WebSocket manager");
            return -1;
        }
    }
    
    // Register WebSocket handler for batch delete recordings
    log_info("Registering handler for recordings/batch-delete");
    if (websocket_manager_register_handler("recordings/batch-delete", websocket_handle_batch_delete_recordings) != 0) {
        log_error("Failed to register handler for recordings/batch-delete");
        return -1;
    }
    
    // Register WebSocket handler for system logs
    log_info("Registering handler for system/logs");
    if (websocket_manager_register_handler("system/logs", websocket_handle_system_logs) != 0) {
        log_error("Failed to register handler for system/logs");
        return -1;
    }
    
    log_info("WebSocket handlers registered successfully");
    return 0;
}
