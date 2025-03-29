#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "web/register_websocket_handlers.h"
#include "web/websocket_handler.h"
#include "web/websocket_manager.h"
#include "web/api_handlers_recordings_batch_ws.h"
#include "web/api_handlers_system_ws.h"
#include "core/logger.h"

/**
 * @brief Register WebSocket handlers
 */
void websocket_register_handlers(void) {
    log_info("Registering WebSocket handlers");
    
    // Initialize WebSocket subsystem if not already initialized
    extern void websocket_init(void);
    if (!websocket_manager_is_initialized()) {
        log_info("Initializing WebSocket subsystem before registering handlers");
        websocket_init();
    }
    
    // Register batch delete recordings handler
    websocket_handler_register("recordings/batch-delete", websocket_handle_batch_delete_recordings);
    
    // Register system logs handler
    websocket_handler_register("system/logs", websocket_handle_system_logs);
    
    log_info("WebSocket handlers registered");
}
