#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <pthread.h>
#include <signal.h>
#include <unistd.h>
#include <regex.h>

#include "web/mongoose_server.h"
#include "web/http_server.h"
#include "core/logger.h"
#include "utils/memory.h"

// Include Mongoose
#include "mongoose.h"
#include "web/mongoose_adapter.h"
#include "web/api_handlers.h"

// Default initial handler capacity
#define INITIAL_HANDLER_CAPACITY 32

// Route entry structure
typedef struct {
    const char *method;     // HTTP method (GET, POST, etc.) or NULL for any method
    const char *pattern;    // URL pattern (regex pattern)
    regex_t regex;          // Compiled regex
    void (*handler)(struct mg_connection *c, struct mg_http_message *hm); // Handler function
} route_entry_t;

// Route table
static route_entry_t *s_routes = NULL;
static int s_route_count = 0;
static int s_route_capacity = 0;

// Forward declarations
static void mongoose_event_handler(struct mg_connection *c, int ev, void *ev_data);
static void handle_http_request(struct mg_connection *c, struct mg_http_message *hm, http_server_t *server);
static void *mongoose_server_event_loop(void *arg);
static void init_route_table(void);
static void free_route_table(void);
static void add_route(const char *method, const char *pattern, void (*handler)(struct mg_connection *c, struct mg_http_message *hm));
static int match_route(const char *method, const char *uri, struct mg_http_message *hm);

// Include other mongoose server components
#include "web/mongoose_server_utils.h"
#include "web/mongoose_server_handlers.h"
#include "web/mongoose_server_auth.h"
#include "web/mongoose_server_static.h"
#include "web/http_router.h"

// API handler function type
typedef void (*mg_api_handler_t)(struct mg_connection *c, struct mg_http_message *hm);

// API route entry structure
typedef struct {
    const char *method;     // HTTP method (GET, POST, etc.)
    const char *uri;        // URI pattern
    mg_api_handler_t handler; // Handler function
} mg_api_route_t;

// API routes table
static const mg_api_route_t s_api_routes[] = {
    // Streams API
    {"GET", "/api/streams", mg_handle_get_streams},
    {"POST", "/api/streams", mg_handle_post_stream},
    {"POST", "/api/streams/#/toggle_streaming", mg_handle_toggle_streaming},
    {"GET", "/api/streams/#", mg_handle_get_stream},
    {"PUT", "/api/streams/#", mg_handle_put_stream},
    {"DELETE", "/api/streams/#", mg_handle_delete_stream},
    
    // Settings API
    {"GET", "/api/settings", mg_handle_get_settings},
    {"POST", "/api/settings", mg_handle_post_settings},
    
    // System API
    {"GET", "/api/system/info", mg_handle_get_system_info},
    {"GET", "/api/system/logs", mg_handle_get_system_logs},
    {"POST", "/api/system/restart", mg_handle_post_system_restart},
    {"POST", "/api/system/shutdown", mg_handle_post_system_shutdown},
    {"POST", "/api/system/clear_logs", mg_handle_post_system_clear_logs},
    {"POST", "/api/system/backup", mg_handle_post_system_backup},
    {"GET", "/api/system/status", mg_handle_get_system_status},
    
    // Recordings API
    {"GET", "/api/recordings", mg_handle_get_recordings},
    {"GET", "/api/recordings/play/#", mg_handle_play_recording},
    {"GET", "/api/recordings/download/#", mg_handle_download_recording},
    {"GET", "/api/recordings/#", mg_handle_get_recording},
    {"DELETE", "/api/recordings/#", mg_handle_delete_recording},
    
    // Streaming API - HLS
    {"GET", "/api/streaming/#/hls/index.m3u8", mg_handle_hls_master_playlist},
    {"GET", "/api/streaming/#/hls/stream.m3u8", mg_handle_hls_media_playlist},
    {"GET", "/api/streaming/#/hls/segment_#.ts", mg_handle_hls_segment},
    
    // Streaming API - WebRTC
    {"POST", "/api/streaming/#/webrtc/offer", mg_handle_webrtc_offer},
    {"POST", "/api/streaming/#/webrtc/ice", mg_handle_webrtc_ice},
    
    // Detection API
    {"GET", "/api/detection/results/#", mg_handle_get_detection_results},
    {"GET", "/api/detection/models", mg_handle_get_detection_models},
    
    // End of table marker
    {NULL, NULL, NULL}
};

/**
 * @brief Handle API request using the routes table
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 * @return true if request was handled, false otherwise
 */
static bool handle_api_request(struct mg_connection *c, struct mg_http_message *hm) {
    // Extract URI and method
    char uri[MAX_PATH_LENGTH];
    size_t uri_len = hm->uri.len < sizeof(uri) - 1 ? hm->uri.len : sizeof(uri) - 1;
    memcpy(uri, hm->uri.buf, uri_len);
    uri[uri_len] = '\0';
    
    char method[16];
    size_t method_len = hm->method.len < sizeof(method) - 1 ? hm->method.len : sizeof(method) - 1;
    memcpy(method, hm->method.buf, method_len);
    method[method_len] = '\0';
    
    // Try to match each route
    for (const mg_api_route_t *route = s_api_routes; route->method != NULL; route++) {
        // Check method
        if (strcasecmp(route->method, method) != 0) {
            continue;
        }
        
        // Check if URI matches the pattern
        if (mg_match(mg_str(uri), mg_str(route->uri), NULL)) {
            // Route matched, call handler
            log_info("API route matched: %s %s", method, uri);
            route->handler(c, hm);
            return true;
        }
    }
    
    // No route matched
    log_debug("No API route matched for: %s %s", method, uri);
    return false;
}

/**
 * @brief Initialize the route table
 */
static void init_route_table(void) {
    // Allocate initial route table
    s_route_capacity = 32;
    s_routes = calloc(s_route_capacity, sizeof(route_entry_t));
    if (!s_routes) {
        log_error("Failed to allocate memory for route table");
        return;
    }
    s_route_count = 0;
    
    // Add routes for API endpoints
    
    // Streams API
    add_route("GET", "^/api/streams$", mg_handle_get_streams);
    add_route("POST", "^/api/streams$", mg_handle_post_stream);
    add_route("GET", "^/api/streams/([^/]+)$", mg_handle_get_stream);
    add_route("PUT", "^/api/streams/([^/]+)$", mg_handle_put_stream);
    add_route("DELETE", "^/api/streams/([^/]+)$", mg_handle_delete_stream);
    add_route("POST", "^/api/streams/([^/]+)/toggle_streaming$", mg_handle_toggle_streaming);
    
    // Settings API
    add_route("GET", "^/api/settings$", mg_handle_get_settings);
    add_route("POST", "^/api/settings$", mg_handle_post_settings);
    
    // System API
    add_route("GET", "^/api/system/info$", mg_handle_get_system_info);
    add_route("GET", "^/api/system/logs$", mg_handle_get_system_logs);
    add_route("POST", "^/api/system/restart$", mg_handle_post_system_restart);
    add_route("POST", "^/api/system/shutdown$", mg_handle_post_system_shutdown);
    add_route("POST", "^/api/system/clear_logs$", mg_handle_post_system_clear_logs);
    add_route("POST", "^/api/system/backup$", mg_handle_post_system_backup);
    add_route("GET", "^/api/system/status$", mg_handle_get_system_status);
    
    // Recordings API
    add_route("GET", "^/api/recordings$", mg_handle_get_recordings);
    add_route("GET", "^/api/recordings/([^/]+)$", mg_handle_get_recording);
    add_route("DELETE", "^/api/recordings/([^/]+)$", mg_handle_delete_recording);
    add_route("GET", "^/api/recordings/download/([^/]+)$", mg_handle_download_recording);
    add_route("GET", "^/api/recordings/play/([^/]+)$", mg_handle_play_recording);
    
    // Streaming API - HLS
    add_route("GET", "^/api/streaming/([^/]+)/hls/index\\.m3u8$", mg_handle_hls_master_playlist);
    add_route("GET", "^/api/streaming/([^/]+)/hls/stream\\.m3u8$", mg_handle_hls_media_playlist);
    add_route("GET", "^/api/streaming/([^/]+)/hls/segment_([^/]+)\\.ts$", mg_handle_hls_segment);
    
    // Streaming API - WebRTC
    add_route("POST", "^/api/streaming/([^/]+)/webrtc/offer$", mg_handle_webrtc_offer);
    add_route("POST", "^/api/streaming/([^/]+)/webrtc/ice$", mg_handle_webrtc_ice);
    
    // Detection API
    add_route("GET", "^/api/detection/results/([^/]+)$", mg_handle_get_detection_results);
    add_route("GET", "^/api/detection/models$", mg_handle_get_detection_models);
    
    log_info("Route table initialized with %d routes", s_route_count);
}

/**
 * @brief Free the route table
 */
static void free_route_table(void) {
    if (s_routes) {
        // Free compiled regexes
        for (int i = 0; i < s_route_count; i++) {
            regfree(&s_routes[i].regex);
        }
        
        free(s_routes);
        s_routes = NULL;
        s_route_count = 0;
        s_route_capacity = 0;
    }
}

/**
 * @brief Add a route to the route table
 * 
 * @param method HTTP method (GET, POST, etc.) or NULL for any method
 * @param pattern URL pattern (regex pattern)
 * @param handler Handler function
 */
static void add_route(const char *method, const char *pattern, void (*handler)(struct mg_connection *c, struct mg_http_message *hm)) {
    if (!pattern || !handler) {
        log_error("Invalid parameters for add_route");
        return;
    }
    
    // Check if we need to resize the route table
    if (s_route_count >= s_route_capacity) {
        int new_capacity = s_route_capacity * 2;
        route_entry_t *new_routes = realloc(s_routes, new_capacity * sizeof(route_entry_t));
        if (!new_routes) {
            log_error("Failed to resize route table");
            return;
        }
        
        s_routes = new_routes;
        s_route_capacity = new_capacity;
    }
    
    // Add route
    s_routes[s_route_count].method = method;
    s_routes[s_route_count].pattern = pattern;
    s_routes[s_route_count].handler = handler;
    
    // Compile regex
    int result = regcomp(&s_routes[s_route_count].regex, pattern, REG_EXTENDED | REG_ICASE);
    if (result != 0) {
        char error_buffer[256];
        regerror(result, &s_routes[s_route_count].regex, error_buffer, sizeof(error_buffer));
        log_error("Failed to compile regex for pattern %s: %s", pattern, error_buffer);
        return;
    }
    
    s_route_count++;
    log_debug("Added route: method=%s, pattern=%s", method ? method : "ANY", pattern);
}

/**
 * @brief Match a route in the route table
 * 
 * @param method HTTP method
 * @param uri URI to match
 * @param hm HTTP message (for extracting parameters)
 * @return int Index of matching route or -1 if no match
 */
static int match_route(const char *method, const char *uri, struct mg_http_message *hm) {
    if (!method || !uri) {
        return -1;
    }
    
    // Try to match each route
    for (int i = 0; i < s_route_count; i++) {
        // Check method
        if (s_routes[i].method && strcasecmp(s_routes[i].method, method) != 0) {
            continue;
        }
        
        // Try to match regex
        regmatch_t matches[10];
        int result = regexec(&s_routes[i].regex, uri, 10, matches, 0);
        if (result == 0) {
            // Route matched
            log_debug("Route matched: method=%s, pattern=%s, uri=%s", 
                     s_routes[i].method ? s_routes[i].method : "ANY", s_routes[i].pattern, uri);
            return i;
        }
    }
    
    // No route matched
    return -1;
}

/**
 * @brief Initialize HTTP server
 */
http_server_handle_t http_server_init(const http_server_config_t *config) {
    // Initialize router
    if (mongoose_server_init_router() != 0) {
        log_error("Failed to initialize router");
        return NULL;
    }
    
    // Initialize route table
    init_route_table();
    
    return mongoose_server_init(config);
}

/**
 * @brief Initialize HTTP server using Mongoose
 */
http_server_handle_t mongoose_server_init(const http_server_config_t *config) {
    if (!config) {
        log_error("Invalid server configuration");
        return NULL;
    }

    // Allocate server structure
    http_server_t *server = calloc(1, sizeof(http_server_t));
    if (!server) {
        log_error("Failed to allocate memory for server");
        return NULL;
    }

    // Copy configuration
    memcpy(&server->config, config, sizeof(http_server_config_t));

    // Allocate Mongoose event manager
    server->mgr = calloc(1, sizeof(struct mg_mgr));
    if (!server->mgr) {
        log_error("Failed to allocate memory for Mongoose event manager");
        free(server);
        return NULL;
    }

    // Initialize Mongoose event manager
    mg_mgr_init(server->mgr);

    // Allocate handlers array
    server->handlers = calloc(INITIAL_HANDLER_CAPACITY, sizeof(*server->handlers));
    if (!server->handlers) {
        log_error("Failed to allocate memory for handlers");
        mg_mgr_free(server->mgr);
        free(server->mgr);
        free(server);
        return NULL;
    }

    server->handler_capacity = INITIAL_HANDLER_CAPACITY;
    server->handler_count = 0;
    server->running = false;
    server->start_time = time(NULL);

    log_info("HTTP server initialized");
    return server;
}

/**
 * @brief Start HTTP server
 */
int http_server_start(http_server_handle_t server) {
    if (!server || !server->mgr) {
        log_error("Invalid server handle");
        return -1;
    }

    if (server->running) {
        log_warn("Server is already running");
        return 0;
    }

    // Construct listen URL
    char listen_url[128];
    if (server->config.ssl_enabled) {
        snprintf(listen_url, sizeof(listen_url), "https://0.0.0.0:%d", server->config.port);
    } else {
        snprintf(listen_url, sizeof(listen_url), "http://0.0.0.0:%d", server->config.port);
    }

    // Start listening
    struct mg_connection *c = mg_http_listen(server->mgr, listen_url, mongoose_event_handler, server);
    if (c == NULL) {
        log_error("Failed to start server on %s", listen_url);
        return -1;
    }

    // Configure SSL if enabled
    if (server->config.ssl_enabled) {
        struct mg_tls_opts opts = {
            .cert = server->config.cert_path,
            .key = server->config.key_path,
        };
        mg_tls_init(c, &opts);
    }

    server->running = true;
    log_info("HTTP server started on port %d", server->config.port);

    // Create a thread that runs the event loop
    pthread_t thread;
    if (pthread_create(&thread, NULL, (void *(*)(void *))mongoose_server_event_loop, server) != 0) {
        log_error("Failed to create server thread");
        server->running = false;
        return -1;
    }

    // Detach thread to let it run independently
    pthread_detach(thread);

    return 0;
}

/**
 * @brief Stop HTTP server
 */
void http_server_stop(http_server_handle_t server) {
    if (!server || !server->mgr) {
        return;
    }

    if (!server->running) {
        return;
    }

    server->running = false;
    log_info("Stopping HTTP server");

    // Signal all connections to close
    for (struct mg_connection *c = server->mgr->conns; c != NULL; c = c->next) {
        c->is_closing = 1;
    }

    // Give connections time to close gracefully
    sleep(1);

    // Free Mongoose event manager
    mg_mgr_free(server->mgr);

    log_info("HTTP server stopped");
}

/**
 * @brief Destroy HTTP server
 */
void http_server_destroy(http_server_handle_t server) {
    if (!server) {
        return;
    }

    // Stop server if running
    if (server->running) {
        http_server_stop(server);
    }

    // Free resources
    if (server->mgr) {
        free(server->mgr);
    }

    if (server->handlers) {
        free(server->handlers);
    }

    // Free route table
    free_route_table();

    free(server);
    log_info("HTTP server destroyed");
}

/**
 * @brief Register request handler
 */
int http_server_register_handler(http_server_handle_t server, const char *path, 
                                const char *method, request_handler_t handler) {
    if (!server || !path || !handler) {
        log_error("Invalid parameters for register_handler");
        return -1;
    }

    // Check if we need to resize the handlers array
    if (server->handler_count >= server->handler_capacity) {
        int new_capacity = server->handler_capacity * 2;
        void *new_handlers = realloc(server->handlers, new_capacity * sizeof(*server->handlers));
        if (!new_handlers) {
            log_error("Failed to resize handlers array");
            return -1;
        }

        server->handlers = new_handlers;
        server->handler_capacity = new_capacity;
    }

    // Add handler
    strncpy(server->handlers[server->handler_count].path, path, sizeof(server->handlers[0].path) - 1);
    server->handlers[server->handler_count].path[sizeof(server->handlers[0].path) - 1] = '\0';

    if (method) {
        strncpy(server->handlers[server->handler_count].method, method, sizeof(server->handlers[0].method) - 1);
        server->handlers[server->handler_count].method[sizeof(server->handlers[0].method) - 1] = '\0';
    } else {
        server->handlers[server->handler_count].method[0] = '\0';
    }

    server->handlers[server->handler_count].handler = handler;
    server->handler_count++;

    log_debug("Registered handler for path: %s, method: %s", 
             path, method ? method : "ANY");

    return 0;
}

/**
 * @brief Get server statistics
 */
int http_server_get_stats(http_server_handle_t server, int *active_connections, 
                         double *requests_per_second, uint64_t *bytes_sent, 
                         uint64_t *bytes_received) {
    if (!server) {
        return -1;
    }

    if (active_connections) {
        *active_connections = server->active_connections;
    }

    if (requests_per_second) {
        time_t uptime = time(NULL) - server->start_time;
        if (uptime > 0) {
            *requests_per_second = (double)server->total_requests / (double)uptime;
        } else {
            *requests_per_second = 0.0;
        }
    }

    if (bytes_sent) {
        *bytes_sent = server->bytes_sent;
    }

    if (bytes_received) {
        *bytes_received = server->bytes_received;
    }

    return 0;
}

/**
 * @brief Mongoose event handler
 */
static void mongoose_event_handler(struct mg_connection *c, int ev, void *ev_data) {
    http_server_t *server = (http_server_t *)c->fn_data;

    if (ev == MG_EV_HTTP_MSG) {
        // HTTP request received
        struct mg_http_message *hm = (struct mg_http_message *)ev_data;

        // Update statistics
        server->active_connections++;
        server->total_requests++;
        server->bytes_received += hm->message.len;

        // Extract URI
        char uri[MAX_PATH_LENGTH];
        size_t uri_len = hm->uri.len < sizeof(uri) - 1 ? hm->uri.len : sizeof(uri) - 1;
        memcpy(uri, hm->uri.buf, uri_len);
        uri[uri_len] = '\0';

        // Check if this is an API request
        bool is_api_request = strncasecmp(uri, "/api/", 5) == 0;
        bool handled = false;

        // Log request details
        log_info("Received request: uri=%s, is_api_request=%d", uri, is_api_request);

        // If this is an API request, try to handle it with direct handlers
        if (is_api_request) {
            // Check authentication if enabled
            if (server->config.auth_enabled && mongoose_server_basic_auth_check(hm, server) != 0) {
                // Authentication failed
                log_info("Authentication failed for request: %s", uri);
                mg_http_reply(c, 401, "WWW-Authenticate: Basic realm=\"LightNVR\"\r\n", 
                             "{\"error\": \"Unauthorized\"}\n");
                return;
            }

            // Handle CORS preflight request
            if (server->config.cors_enabled && mg_match(hm->method, mg_str("OPTIONS"), NULL)) {
                log_info("Handling CORS preflight request: %s", uri);
                mongoose_server_handle_cors_preflight(c, hm, server);
                return;
            }

            // Try to handle the API request using the routes table
            handled = handle_api_request(c, hm);
        }

        // If not handled by API handlers, serve static file or return 404
        if (!handled) {
            // Try to serve static file
            mongoose_server_handle_static_file(c, hm, server);
        }

        // Update statistics
        server->active_connections--;
    } else if (ev == MG_EV_ACCEPT) {
        // New connection accepted
        log_debug("New connection accepted");
    } else if (ev == MG_EV_CLOSE) {
        // Connection closed
        log_debug("Connection closed");
        if (server->active_connections > 0) {
            server->active_connections--;
        }
    } else if (ev == MG_EV_ERROR) {
        // Connection error
        log_error("Connection error: %s", (char *)ev_data);
    } else if (ev == MG_EV_POLL) {
        // Poll event - do nothing
    } else {
        // Other events
        log_debug("Unhandled event: %d", ev);
    }
}

/**
 * @brief Event loop for Mongoose server
 * This function runs in a separate thread and continuously calls mg_mgr_poll
 */
static void *mongoose_server_event_loop(void *arg) {
    http_server_t *server = (http_server_t *)arg;
    
    log_info("Mongoose event loop started");
    
    // Run event loop until server is stopped
    int poll_count = 0;
    while (server->running) {
        // Poll for events with a 1000ms timeout
        mg_mgr_poll(server->mgr, 1000);
        poll_count++;
        
        // Log every 10 polls
        if (poll_count % 10 == 0) {
            log_debug("Mongoose event loop poll count: %d", poll_count);
        }
    }
    
    log_info("Mongoose event loop stopped");
    return NULL;
}

/**
 * @brief Handle HTTP request
 */
static void handle_http_request(struct mg_connection *c, struct mg_http_message *hm, http_server_t *server) {
    // Extract URI
    char uri[MAX_PATH_LENGTH];
    size_t uri_len = hm->uri.len < sizeof(uri) - 1 ? hm->uri.len : sizeof(uri) - 1;
    memcpy(uri, hm->uri.buf, uri_len);
    uri[uri_len] = '\0';

    // Split URI into path and query string
    char path[MAX_PATH_LENGTH];
    strncpy(path, uri, sizeof(path) - 1);
    path[sizeof(path) - 1] = '\0';

    char *query = strchr(path, '?');
    if (query) {
        *query = '\0';
    }

    // Check authentication if enabled
    if (server->config.auth_enabled && mongoose_server_basic_auth_check(hm, server) != 0) {
        // Authentication failed
        mg_http_reply(c, 401, "WWW-Authenticate: Basic realm=\"LightNVR\"\r\n", 
                     "{\"error\": \"Unauthorized\"}\n");
        return;
    }

    // Handle CORS preflight request
    if (server->config.cors_enabled && mg_match(hm->method, mg_str("OPTIONS"), NULL)) {
        mongoose_server_handle_cors_preflight(c, hm, server);
        return;
    }

    // Find handler for request
    bool handler_found = false;
    for (int i = 0; i < server->handler_count; i++) {
        if (mongoose_server_path_matches(server->handlers[i].path, path)) {
            // Check method
            if (server->handlers[i].method[0] == '\0' ||
                (mg_match(hm->method, mg_str("GET"), NULL) && strcmp(server->handlers[i].method, "GET") == 0) ||
                (mg_match(hm->method, mg_str("POST"), NULL) && strcmp(server->handlers[i].method, "POST") == 0) ||
                (mg_match(hm->method, mg_str("PUT"), NULL) && strcmp(server->handlers[i].method, "PUT") == 0) ||
                (mg_match(hm->method, mg_str("DELETE"), NULL) && strcmp(server->handlers[i].method, "DELETE") == 0)) {
                
                // Found matching handler
                handler_found = true;

                // Convert Mongoose request to HTTP request
                http_request_t request;
                if (mongoose_server_mg_to_request(c, hm, &request) != 0) {
                    mg_http_reply(c, 400, "", "{\"error\": \"Bad Request\"}\n");
                    return;
                }

                // Prepare response
                http_response_t response;
                memset(&response, 0, sizeof(response));

                // Call handler
                server->handlers[i].handler(&request, &response);

                // Add CORS headers if enabled
                if (server->config.cors_enabled) {
                    mongoose_server_add_cors_headers(c, server);
                }

                // Send response
                mongoose_server_send_response(c, &response);

                // Update statistics
                server->bytes_sent += response.body_length;

                // Free request and response resources
                if (request.body) {
                    free(request.body);
                }
                if (request.headers) {
                    free(request.headers);
                }
                if (response.body) {
                    free(response.body);
                }
                if (response.headers) {
                    free(response.headers);
                }

                return;
            }
        }
    }

    // If no handler found, serve static file
    if (!handler_found) {
        mongoose_server_handle_static_file(c, hm, server);
    }
}
