#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <assert.h>

#include "web/websocket_bridge.h"
#include "core/logger.h"

// Test handler for WebSocket messages
static void test_ws_handler(struct mg_connection *mg_conn, const char *data, size_t len, void *user_data) {
    printf("Test handler called with data: %.*s\n", (int)len, data);
}

int main(int argc, char *argv[]) {
    // Initialize logger
    init_logger();
    set_log_level(LOG_LEVEL_DEBUG);
    
    // Initialize WebSocket bridge
    int result = websocket_bridge_init();
    assert(result == 0);
    assert(websocket_bridge_is_initialized() == true);
    
    // Register a handler
    result = websocket_bridge_register_handler("test", test_ws_handler, NULL);
    assert(result == 0);
    
    // Shutdown WebSocket bridge
    websocket_bridge_shutdown();
    assert(websocket_bridge_is_initialized() == false);
    
    printf("All tests passed!\n");
    return 0;
}
