#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "web/mongoose_server_handlers.h"
#include "web/http_router.h"
#include "web/mongoose_server_utils.h"
#include "core/logger.h"

// Include Mongoose
#include "mongoose.h"

// Global router instance
static http_router_handle_t g_router = NULL;

// Forward declarations
static void mongoose_router_handler(struct mg_connection *c, struct mg_http_message *hm, void *fn_data);
static void mongoose_event_handler(struct mg_connection *c, int ev, void *ev_data);

/**
 * @brief Initialize the router
 * 
 * @return int 0 on success, non-zero on error
 */
int mongoose_server_init_router(void) {
    if (g_router) {
        // Router already initialized
        return 0;
    }

    g_router = http_router_create();
    if (!g_router) {
        log_error("Failed to create HTTP router");
        return -1;
    }

    log_info("HTTP router initialized");
    return 0;
}

/**
 * @brief Destroy the router
 */
void mongoose_server_destroy_router(void) {
    if (g_router) {
        http_router_destroy(g_router);
        g_router = NULL;
    }
}

/**
 * @brief Get the router handle
 * 
 * @return http_router_handle_t Router handle or NULL if not initialized
 */
http_router_handle_t mongoose_server_get_router(void) {
    return g_router;
}

/**
 * @brief Register a Mongoose event handler for the router
 * 
 * @param mgr Mongoose event manager
 * @return int 0 on success, non-zero on error
 */
int mongoose_server_register_router_handler(struct mg_mgr *mgr) {
    if (!mgr || !g_router) {
        log_error("Invalid parameters for register_router_handler");
        return -1;
    }

    // Set the router handler for all HTTP connections
    for (struct mg_connection *c = mgr->conns; c != NULL; c = c->next) {
        if (c->is_listening && c->fn_data == NULL) {
            c->fn_data = g_router;
        }
    }

    log_info("Router handler registered with Mongoose");
    return 0;
}

/**
 * @brief Adapter to convert request_handler_t to route_handler_t
 */
typedef struct {
    request_handler_t original_handler;
} handler_adapter_data_t;


/**
 * @brief Mongoose event handler that routes HTTP events to the router handler
 */
static void mongoose_event_handler(struct mg_connection *c, int ev, void *ev_data) {
    if (ev == MG_EV_HTTP_MSG) {
        // This is an HTTP message event, call our HTTP router handler
        struct mg_http_message *hm = (struct mg_http_message *)ev_data;
        mongoose_router_handler(c, hm, c->fn_data);
    }
}

/**
 * @brief Set the Mongoose event handler for the router
 * 
 * @param c Mongoose connection
 */
void mongoose_server_set_router_handler(struct mg_connection *c) {
    if (!c || !g_router) {
        return;
    }

    c->fn = mongoose_event_handler;
    c->fn_data = g_router;
}
