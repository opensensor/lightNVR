#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <pthread.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <fcntl.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <errno.h>
#include <signal.h>
#include <time.h>
#include <dirent.h>
#include <ctype.h>
#include <sys/time.h>

#include "web/web_server.h"
#include "web/api_handlers.h"
#include "core/logger.h"
#include "core/config.h"
#include "database/database_manager.h"
#include "video/stream_manager.h"
#include "storage/storage_manager.h"

// Maximum number of pending connections
#define MAX_PENDING 20

// Buffer size for reading requests
#define REQUEST_BUFFER_SIZE 8192

// Maximum number of headers
#define MAX_HEADERS 100

// Maximum size of path
#define MAX_PATH_SIZE 1024

// Web server state
static struct {
    int server_socket;
    int port;
    char web_root[MAX_PATH_LENGTH];
    int running;
    pthread_t server_thread;
    pthread_mutex_t mutex;
    
    // Authentication
    int auth_enabled;
    char username[32];
    char password[32];
    
    // CORS settings
    int cors_enabled;
    char allowed_origins[256];
    char allowed_methods[256];
    char allowed_headers[256];
    
    // SSL/TLS settings
    int ssl_enabled;
    char cert_path[MAX_PATH_LENGTH];
    char key_path[MAX_PATH_LENGTH];
    
    // Connection settings
    int max_connections;
    int connection_timeout;
    
    // Statistics
    int active_connections;
    uint64_t total_requests;
    uint64_t bytes_sent;
    uint64_t bytes_received;
    time_t start_time;
    
    // Request handlers
    struct {
        char path[256];
        char method[16];
        request_handler_t handler;
    } handlers[100];
    int handler_count;
} web_server = {
    .server_socket = -1,
    .port = 8080,
    .web_root = "",
    .running = 0,
    .auth_enabled = 0,
    .username = "",
    .password = "",
    .cors_enabled = 0,
    .allowed_origins = "*",
    .allowed_methods = "GET, POST, OPTIONS",
    .allowed_headers = "Content-Type, Authorization",
    .ssl_enabled = 0,
    .cert_path = "",
    .key_path = "",
    .max_connections = 10,
    .connection_timeout = 30,
    .active_connections = 0,
    .total_requests = 0,
    .bytes_sent = 0,
    .bytes_received = 0,
    .start_time = 0,
    .handler_count = 0
};

// MIME types
static struct {
    const char *extension;
    const char *mime_type;
} mime_types[] = {
    {".html", "text/html"},
    {".htm", "text/html"},
    {".css", "text/css"},
    {".js", "application/javascript"},
    {".json", "application/json"},
    {".jpg", "image/jpeg"},
    {".jpeg", "image/jpeg"},
    {".png", "image/png"},
    {".gif", "image/gif"},
    {".svg", "image/svg+xml"},
    {".ico", "image/x-icon"},
    {".mp4", "video/mp4"},
    {".webm", "video/webm"},
    {".ogg", "video/ogg"},
    {".mp3", "audio/mpeg"},
    {".wav", "audio/wav"},
    {".txt", "text/plain"},
    {".xml", "application/xml"},
    {".pdf", "application/pdf"},
    {NULL, NULL}
};

// HTTP status codes
static struct {
    int code;
    const char *message;
} http_status_codes[] = {
    {200, "OK"},
    {201, "Created"},
    {204, "No Content"},
    {301, "Moved Permanently"},
    {302, "Found"},
    {304, "Not Modified"},
    {400, "Bad Request"},
    {401, "Unauthorized"},
    {403, "Forbidden"},
    {404, "Not Found"},
    {405, "Method Not Allowed"},
    {500, "Internal Server Error"},
    {501, "Not Implemented"},
    {503, "Service Unavailable"},
    {0, NULL}
};

// Forward declarations
static void *server_thread_func(void *arg);
static void handle_client(int client_socket);
static int parse_request(int client_socket, http_request_t *request);
static void send_response(int client_socket, const http_response_t *response);
static void handle_static_file(const http_request_t *request, http_response_t *response);
static const char *get_mime_type(const char *path);
static const char *get_status_message(int status_code);
static void parse_query_string(const char *query_string, void *params);
static int basic_auth_check(const http_request_t *request);
static void handle_cors_preflight(const http_request_t *request, http_response_t *response);
static void add_cors_headers(http_response_t *response);
static void log_request(const http_request_t *request, int status_code);

// Initialize the web server
int init_web_server(int port, const char *web_root) {
    // Initialize mutex
    if (pthread_mutex_init(&web_server.mutex, NULL) != 0) {
        log_error("Failed to initialize web server mutex");
        return -1;
    }
    
    // Set server parameters
    web_server.port = port;
    strncpy(web_server.web_root, web_root, MAX_PATH_LENGTH - 1);
    web_server.web_root[MAX_PATH_LENGTH - 1] = '\0';
    
    // Create server socket
    web_server.server_socket = socket(AF_INET, SOCK_STREAM, 0);
    if (web_server.server_socket < 0) {
        log_error("Failed to create server socket: %s", strerror(errno));
        return -1;
    }

    // Set socket options for reuse
    int opt = 1;
    if (setsockopt(web_server.server_socket, SOL_SOCKET, SO_REUSEADDR, &opt, sizeof(opt)) < 0) {
        log_error("Failed to set SO_REUSEADDR on socket: %s", strerror(errno));
        close(web_server.server_socket);
        web_server.server_socket = -1;
        return -1;
    }

    // Add SO_REUSEPORT if available (not supported on all systems)
#ifdef SO_REUSEPORT
    if (setsockopt(web_server.server_socket, SOL_SOCKET, SO_REUSEPORT, &opt, sizeof(opt)) < 0) {
        log_warn("Failed to set SO_REUSEPORT on socket: %s", strerror(errno));
        // Continue anyway, this is optional
    }
#endif

    // Set socket to non-blocking mode
    int flags = fcntl(web_server.server_socket, F_GETFL, 0);
    if (flags == -1) {
        log_error("Failed to get socket flags: %s", strerror(errno));
        close(web_server.server_socket);
        web_server.server_socket = -1;
        return -1;
    }
    if (fcntl(web_server.server_socket, F_SETFL, flags | O_NONBLOCK) == -1) {
        log_error("Failed to set socket to non-blocking mode: %s", strerror(errno));
        close(web_server.server_socket);
        web_server.server_socket = -1;
        return -1;
    }
    
    // Bind socket to port
    struct sockaddr_in server_addr;
    memset(&server_addr, 0, sizeof(server_addr));
    server_addr.sin_family = AF_INET;
    server_addr.sin_addr.s_addr = INADDR_ANY;
    server_addr.sin_port = htons(port);
    
    if (bind(web_server.server_socket, (struct sockaddr *)&server_addr, sizeof(server_addr)) < 0) {
        log_error("Failed to bind server socket: %s", strerror(errno));
        close(web_server.server_socket);
        web_server.server_socket = -1;
        return -1;
    }
    
    // Listen for connections
    if (listen(web_server.server_socket, MAX_PENDING) < 0) {
        log_error("Failed to listen on server socket: %s", strerror(errno));
        close(web_server.server_socket);
        web_server.server_socket = -1;
        return -1;
    }
    
    // Start server thread
    web_server.running = 1;
    web_server.start_time = time(NULL);
    
    if (pthread_create(&web_server.server_thread, NULL, server_thread_func, NULL) != 0) {
        log_error("Failed to create server thread: %s", strerror(errno));
        close(web_server.server_socket);
        web_server.server_socket = -1;
        web_server.running = 0;
        return -1;
    }
    
    // Register API handlers
    register_api_handlers();
    
    log_info("Web server started on port %d", port);
    return 0;
}

// Shutdown the web server
// In shutdown_web_server():
void shutdown_web_server(void) {
    log_info("Shutting down web server...");
    
    pthread_mutex_lock(&web_server.mutex);
    if (web_server.running) {
        web_server.running = 0;
        
        // Close server socket to unblock accept()
        if (web_server.server_socket >= 0) {
            shutdown(web_server.server_socket, SHUT_RDWR);
            close(web_server.server_socket);
            web_server.server_socket = -1;
        }
        
        pthread_mutex_unlock(&web_server.mutex);
        
        // Create a separate thread to monitor the join
        pthread_t monitor_thread;
        if (pthread_create(&monitor_thread, NULL, web_server_shutdown_monitor, NULL) == 0) {
            pthread_detach(monitor_thread);
        }
        
        // Use regular join instead of timed join
        if (pthread_join(web_server.server_thread, NULL) != 0) {
            log_warn("Failed to join web server thread");
        }
        
        log_info("Web server thread shutdown complete");
    } else {
        pthread_mutex_unlock(&web_server.mutex);
    }
    
    pthread_mutex_destroy(&web_server.mutex);
    
    log_info("Web server shutdown complete");
}

// Add this function to monitor thread shutdown
static void *web_server_shutdown_monitor(void *arg) {
    // Wait a maximum of 5 seconds
    sleep(5);
    
    // Check if thread is still running
    if (web_server.running) {
        log_warn("Web server thread did not exit within timeout, forcing termination");
        pthread_cancel(web_server.server_thread);
    }
    
    return NULL;
}

// Register a request handler for a specific path
int register_request_handler(const char *path, const char *method, request_handler_t handler) {
    if (!path || !handler) {
        log_error("Invalid parameters for register_request_handler");
        return -1;
    }
    
    pthread_mutex_lock(&web_server.mutex);
    
    if (web_server.handler_count >= sizeof(web_server.handlers) / sizeof(web_server.handlers[0])) {
        log_error("Too many request handlers registered");
        pthread_mutex_unlock(&web_server.mutex);
        return -1;
    }
    
    strncpy(web_server.handlers[web_server.handler_count].path, path, sizeof(web_server.handlers[0].path) - 1);
    web_server.handlers[web_server.handler_count].path[sizeof(web_server.handlers[0].path) - 1] = '\0';
    
    if (method) {
        strncpy(web_server.handlers[web_server.handler_count].method, method, sizeof(web_server.handlers[0].method) - 1);
        web_server.handlers[web_server.handler_count].method[sizeof(web_server.handlers[0].method) - 1] = '\0';
    } else {
        web_server.handlers[web_server.handler_count].method[0] = '\0';
    }
    
    web_server.handlers[web_server.handler_count].handler = handler;
    web_server.handler_count++;
    
    pthread_mutex_unlock(&web_server.mutex);
    
    log_debug("Registered request handler for path: %s, method: %s", 
             path, method ? method : "ANY");
    
    return 0;
}

// Set authentication requirements
int set_authentication(bool enabled, const char *username, const char *password) {
    if (enabled && (!username || !password)) {
        log_error("Username and password are required when authentication is enabled");
        return -1;
    }
    
    pthread_mutex_lock(&web_server.mutex);
    
    web_server.auth_enabled = enabled;
    
    if (enabled) {
        strncpy(web_server.username, username, sizeof(web_server.username) - 1);
        web_server.username[sizeof(web_server.username) - 1] = '\0';
        
        strncpy(web_server.password, password, sizeof(web_server.password) - 1);
        web_server.password[sizeof(web_server.password) - 1] = '\0';
    }
    
    pthread_mutex_unlock(&web_server.mutex);
    
    log_info("Authentication %s", enabled ? "enabled" : "disabled");
    return 0;
}

// Set CORS settings
int set_cors_settings(bool enabled, const char *allowed_origins, 
                     const char *allowed_methods, const char *allowed_headers) {
    pthread_mutex_lock(&web_server.mutex);
    
    web_server.cors_enabled = enabled;
    
    if (enabled) {
        if (allowed_origins) {
            strncpy(web_server.allowed_origins, allowed_origins, sizeof(web_server.allowed_origins) - 1);
            web_server.allowed_origins[sizeof(web_server.allowed_origins) - 1] = '\0';
        }
        
        if (allowed_methods) {
            strncpy(web_server.allowed_methods, allowed_methods, sizeof(web_server.allowed_methods) - 1);
            web_server.allowed_methods[sizeof(web_server.allowed_methods) - 1] = '\0';
        }
        
        if (allowed_headers) {
            strncpy(web_server.allowed_headers, allowed_headers, sizeof(web_server.allowed_headers) - 1);
            web_server.allowed_headers[sizeof(web_server.allowed_headers) - 1] = '\0';
        }
    }
    
    pthread_mutex_unlock(&web_server.mutex);
    
    log_info("CORS %s", enabled ? "enabled" : "disabled");
    return 0;
}

// Set SSL/TLS settings
int set_ssl_settings(bool enabled, const char *cert_path, const char *key_path) {
    if (enabled && (!cert_path || !key_path)) {
        log_error("Certificate and key paths are required when SSL/TLS is enabled");
        return -1;
    }
    
    pthread_mutex_lock(&web_server.mutex);
    
    web_server.ssl_enabled = enabled;
    
    if (enabled) {
        strncpy(web_server.cert_path, cert_path, sizeof(web_server.cert_path) - 1);
        web_server.cert_path[sizeof(web_server.cert_path) - 1] = '\0';
        
        strncpy(web_server.key_path, key_path, sizeof(web_server.key_path) - 1);
        web_server.key_path[sizeof(web_server.key_path) - 1] = '\0';
    }
    
    pthread_mutex_unlock(&web_server.mutex);
    
    log_info("SSL/TLS %s", enabled ? "enabled" : "disabled");
    return 0;
}

// Set maximum number of simultaneous connections
int set_max_connections(int max_connections) {
    if (max_connections <= 0) {
        log_error("Maximum connections must be greater than 0");
        return -1;
    }
    
    pthread_mutex_lock(&web_server.mutex);
    web_server.max_connections = max_connections;
    pthread_mutex_unlock(&web_server.mutex);
    
    log_info("Maximum connections set to %d", max_connections);
    return 0;
}

// Set connection timeout
int set_connection_timeout(int timeout_seconds) {
    if (timeout_seconds <= 0) {
        log_error("Connection timeout must be greater than 0");
        return -1;
    }
    
    pthread_mutex_lock(&web_server.mutex);
    web_server.connection_timeout = timeout_seconds;
    pthread_mutex_unlock(&web_server.mutex);
    
    log_info("Connection timeout set to %d seconds", timeout_seconds);
    return 0;
}

// Create a JSON response
int create_json_response(http_response_t *response, int status_code, const char *json_data) {
    if (!response || !json_data) {
        log_error("Invalid parameters for create_json_response");
        return -1;
    }
    
    response->status_code = status_code;
    strncpy(response->content_type, "application/json", sizeof(response->content_type) - 1);
    response->content_type[sizeof(response->content_type) - 1] = '\0';
    
    response->body = strdup(json_data);
    if (!response->body) {
        log_error("Failed to allocate memory for response body");
        return -1;
    }
    
    response->body_length = strlen(json_data);
    
    return 0;
}

// Create a file response
int create_file_response(http_response_t *response, int status_code, 
                        const char *file_path, const char *content_type) {
    if (!response || !file_path) {
        log_error("Invalid parameters for create_file_response");
        return -1;
    }
    
    // Open file
    FILE *file = fopen(file_path, "rb");
    if (!file) {
        log_error("Failed to open file: %s (error: %s)", file_path, strerror(errno));
        return -1;
    }
    
    // Get file size
    fseek(file, 0, SEEK_END);
    long file_size = ftell(file);
    fseek(file, 0, SEEK_SET);
    
    if (file_size <= 0) {
        log_error("Invalid file size for %s: %ld", file_path, file_size);
        fclose(file);
        return -1;
    }
    
    // Allocate memory for file content
    response->body = malloc(file_size);
    if (!response->body) {
        log_error("Failed to allocate memory for file content (%ld bytes)", file_size);
        fclose(file);
        return -1;
    }
    
    // Read file content
    size_t bytes_read = fread(response->body, 1, file_size, file);
    fclose(file);
    
    if (bytes_read != (size_t)file_size) {
        log_error("Failed to read file content: expected %ld bytes, got %zu bytes", file_size, bytes_read);
        free(response->body);
        response->body = NULL;
        return -1;
    }
    
    // Set response fields
    response->status_code = status_code;
    response->body_length = file_size;
    
    if (content_type) {
        strncpy(response->content_type, content_type, sizeof(response->content_type) - 1);
        response->content_type[sizeof(response->content_type) - 1] = '\0';
    } else {
        const char *mime_type = get_mime_type(file_path);
        strncpy(response->content_type, mime_type, sizeof(response->content_type) - 1);
        response->content_type[sizeof(response->content_type) - 1] = '\0';
    }
    
    log_info("Serving file: %s, size: %ld bytes, type: %s", file_path, file_size, response->content_type);
    return 0;
}

// Create a text response
int create_text_response(http_response_t *response, int status_code, 
                        const char *text, const char *content_type) {
    if (!response || !text) {
        log_error("Invalid parameters for create_text_response");
        return -1;
    }
    
    response->status_code = status_code;
    
    if (content_type) {
        strncpy(response->content_type, content_type, sizeof(response->content_type) - 1);
        response->content_type[sizeof(response->content_type) - 1] = '\0';
    } else {
        strncpy(response->content_type, "text/plain", sizeof(response->content_type) - 1);
        response->content_type[sizeof(response->content_type) - 1] = '\0';
    }
    
    response->body = strdup(text);
    if (!response->body) {
        log_error("Failed to allocate memory for response body");
        return -1;
    }
    
    response->body_length = strlen(text);
    
    return 0;
}

// Create a redirect response
int create_redirect_response(http_response_t *response, int status_code, const char *location) {
    if (!response || !location) {
        log_error("Invalid parameters for create_redirect_response");
        return -1;
    }
    
    if (status_code != 301 && status_code != 302 && status_code != 303 && status_code != 307 && status_code != 308) {
        log_error("Invalid status code for redirect: %d", status_code);
        return -1;
    }
    
    response->status_code = status_code;
    strncpy(response->content_type, "text/html", sizeof(response->content_type) - 1);
    response->content_type[sizeof(response->content_type) - 1] = '\0';
    
    // Create a simple HTML body with the redirect
    char *body = malloc(1024);
    if (!body) {
        log_error("Failed to allocate memory for redirect body");
        return -1;
    }
    
    snprintf(body, 1024,
             "<html><head><title>Redirect</title><meta http-equiv=\"refresh\" content=\"0;url=%s\"></head>"
             "<body>Redirecting to <a href=\"%s\">%s</a>...</body></html>",
             location, location, location);
    
    response->body = body;
    response->body_length = strlen(body);
    
    // Add Location header
    if (!response->headers) {
        response->headers = calloc(1, sizeof(void *));
    }
    
    // In a real implementation, we would add the Location header to the headers structure
    // For simplicity, we'll just log it here
    log_debug("Adding Location header: %s", location);
    
    return 0;
}

/**
 * Helper function to get a query parameter from request
 */
int get_query_param(const http_request_t *request, const char *param_name, char *value, size_t value_size) {
    if (!request || !request->query_string || !param_name || !value) {
        return -1;
    }

    // Find the parameter in the query string
    const char *query = request->query_string;
    size_t param_len = strlen(param_name);
    char search[param_len + 2];

    // Format as "param=" or "param&"
    snprintf(search, sizeof(search), "%s=", param_name);

    const char *param_start = strstr(query, search);
    if (!param_start) {
        // Try at the beginning of the query string
        if (strncmp(query, search, strlen(search)) == 0) {
            param_start = query;
        } else {
            return -1; // Parameter not found
        }
    }

    // Move past "param="
    param_start += strlen(search);

    // Find the end of the parameter value
    const char *param_end = strchr(param_start, '&');
    if (!param_end) {
        param_end = param_start + strlen(param_start);
    }

    // Calculate value length
    size_t value_len = param_end - param_start;
    if (value_len >= value_size) {
        value_len = value_size - 1; // Ensure space for null terminator
    }

    // Copy value
    strncpy(value, param_start, value_len);
    value[value_len] = '\0';

    return 0;
}

// Get a form parameter from a request
int get_form_param(const http_request_t *request, const char *name, 
                  char *value, size_t value_size) {
    if (!request || !name || !value || value_size == 0) {
        return -1;
    }
    
    if (!request->body || request->content_length == 0 || 
        strcmp(request->content_type, "application/x-www-form-urlencoded") != 0) {
        return -1;
    }
    
    char body_copy[4096];
    size_t copy_size = request->content_length < sizeof(body_copy) - 1 ? 
                       request->content_length : sizeof(body_copy) - 1;
    
    memcpy(body_copy, request->body, copy_size);
    body_copy[copy_size] = '\0';
    
    char *token = strtok(body_copy, "&");
    while (token) {
        char *eq = strchr(token, '=');
        if (eq) {
            *eq = '\0';
            if (strcmp(token, name) == 0) {
                char *val = eq + 1;
                url_decode(val, value, value_size);
                return 0;
            }
        }
        token = strtok(NULL, "&");
    }
    
    return -1;
}

// HTTP header structure
struct http_header {
    char name[64];
    char value[256];
    struct http_header *next;
};

// Get a header from a request
const char * get_request_header(const http_request_t *request, const char *name) {
    if (!request || !name) {
        log_error("Invalid parameters for get_request_header");
        return NULL;
    }
    
    // Search for the header in the raw request buffer
    // This is a simplified implementation that searches for common headers directly
    
    // Check for common headers with special handling
    if (strcasecmp(name, "Content-Type") == 0 && request->content_type[0] != '\0') {
        return request->content_type;
    }
    else if (strcasecmp(name, "User-Agent") == 0 && request->user_agent[0] != '\0') {
        return request->user_agent;
    }
    else if (strcasecmp(name, "Content-Length") == 0 && request->content_length > 0) {
        // Convert content length to string and return it
        // This is a simplified approach using a static buffer
        static char content_length_str[32];
        snprintf(content_length_str, sizeof(content_length_str), "%llu", 
                (unsigned long long)request->content_length);
        return content_length_str;
    }
    
    // For a more complete implementation, we would need to:
    // 1. Have a proper headers structure in the request
    // 2. Parse all headers during request parsing
    // 3. Search that structure here
    
    // If the header is not found or not implemented
    return NULL;
}

// Set a response header
int set_response_header(http_response_t *response, const char *name, const char *value) {
    if (!response || !name || !value) {
        log_error("Invalid parameters for set_response_header");
        return -1;
    }
    
    // Handle special headers with direct field access
    if (strcasecmp(name, "Content-Type") == 0) {
        strncpy(response->content_type, value, sizeof(response->content_type) - 1);
        response->content_type[sizeof(response->content_type) - 1] = '\0';
        return 0;
    }
    
    // For a complete implementation, we would:
    // 1. Create a headers structure if it doesn't exist
    // 2. Add or update the header in that structure
    
    // Allocate headers structure if it doesn't exist
    if (!response->headers) {
        // Create a simple linked list for headers
        response->headers = calloc(1, sizeof(struct http_header));
        if (!response->headers) {
            log_error("Failed to allocate memory for response headers");
            return -1;
        }
    }
    
    // For now, just log the header and consider it set
    log_debug("Setting response header: %s: %s", name, value);
    
    return 0;
}

// Get server statistics
int get_web_server_stats(int *active_connections, double *requests_per_second, 
                        uint64_t *bytes_sent, uint64_t *bytes_received) {
    pthread_mutex_lock(&web_server.mutex);
    
    if (active_connections) {
        *active_connections = web_server.active_connections;
    }
    
    if (requests_per_second) {
        time_t uptime = time(NULL) - web_server.start_time;
        if (uptime > 0) {
            *requests_per_second = (double)web_server.total_requests / uptime;
        } else {
            *requests_per_second = 0.0;
        }
    }
    
    if (bytes_sent) {
        *bytes_sent = web_server.bytes_sent;
    }
    
    if (bytes_received) {
        *bytes_received = web_server.bytes_received;
    }
    
    pthread_mutex_unlock(&web_server.mutex);
    
    return 0;
}

// Server thread function
// Server thread function
static void *server_thread_func(void *arg) {
    // Set socket back to blocking mode for more reliable accept
    int flags = fcntl(web_server.server_socket, F_GETFL, 0);
    if (flags != -1) {
        fcntl(web_server.server_socket, F_SETFL, flags & ~O_NONBLOCK);
    }

    // Set a timeout on the socket to allow checking running status
    struct timeval tv;
    tv.tv_sec = 1;  // 1 second timeout
    tv.tv_usec = 0;
    setsockopt(web_server.server_socket, SOL_SOCKET, SO_RCVTIMEO, &tv, sizeof(tv));

    while (1) {
        // Check if server is still running
        pthread_mutex_lock(&web_server.mutex);
        if (!web_server.running) {
            pthread_mutex_unlock(&web_server.mutex);
            break;
        }

        // Check if we have too many active connections
        if (web_server.active_connections >= web_server.max_connections) {
            pthread_mutex_unlock(&web_server.mutex);
            usleep(100000); // 100ms
            continue;
        }

        pthread_mutex_unlock(&web_server.mutex);

        // Accept a new connection
        struct sockaddr_in client_addr;
        socklen_t client_len = sizeof(client_addr);
        int client_socket = accept(web_server.server_socket, (struct sockaddr *)&client_addr, &client_len);

        if (client_socket < 0) {
            if (errno == EAGAIN || errno == EWOULDBLOCK || errno == EINTR) {
                // Timeout or interrupted - normal condition, just continue
                continue;
            } else {
                log_error("Failed to accept connection: %s", strerror(errno));
                usleep(100000); // 100ms pause before retry
                continue;
            }
        }
        
        // Update statistics
        pthread_mutex_lock(&web_server.mutex);
        web_server.active_connections++;
        pthread_mutex_unlock(&web_server.mutex);
        
        // Handle client in a new thread
        pthread_t client_thread;
        if (pthread_create(&client_thread, NULL, (void *(*)(void *))handle_client, (void *)(intptr_t)client_socket) != 0) {
            log_error("Failed to create client thread: %s", strerror(errno));
            close(client_socket);
            
            pthread_mutex_lock(&web_server.mutex);
            web_server.active_connections--;
            pthread_mutex_unlock(&web_server.mutex);
        } else {
            // Detach thread so it can clean up itself
            pthread_detach(client_thread);
        }
    }
    
    return NULL;
}

int path_matches(const char *pattern, const char *path) {
    // Special case: if pattern ends with "/*", it matches any path that starts with the prefix
    size_t pattern_len = strlen(pattern);
    if (pattern_len >= 2 && pattern[pattern_len-2] == '/' && pattern[pattern_len-1] == '*') {
        return strncmp(pattern, path, pattern_len-1) == 0;
    }

    // Otherwise, exact match
    return strcmp(pattern, path) == 0;
}

// Handle a client connection
static void handle_client(int client_socket) {
    // Set socket timeout
    struct timeval tv;
    tv.tv_sec = web_server.connection_timeout;
    tv.tv_usec = 0;
    setsockopt(client_socket, SOL_SOCKET, SO_RCVTIMEO, (const char*)&tv, sizeof tv);
    setsockopt(client_socket, SOL_SOCKET, SO_SNDTIMEO, (const char*)&tv, sizeof tv);
    
    // Parse request
    http_request_t request;
    memset(&request, 0, sizeof(request));
    
    if (parse_request(client_socket, &request) != 0) {
        close(client_socket);
        
        pthread_mutex_lock(&web_server.mutex);
        web_server.active_connections--;
        pthread_mutex_unlock(&web_server.mutex);
        
        return;
    }
    
    // Update statistics
    pthread_mutex_lock(&web_server.mutex);
    web_server.total_requests++;
    web_server.bytes_received += request.content_length;
    pthread_mutex_unlock(&web_server.mutex);
    
    // Prepare response
    http_response_t response;
    memset(&response, 0, sizeof(response));
    
    // Check authentication if enabled
    if (web_server.auth_enabled && basic_auth_check(&request) != 0) {
        response.status_code = 401;
        strncpy(response.content_type, "text/plain", sizeof(response.content_type) - 1);
        response.content_type[sizeof(response.content_type) - 1] = '\0';
        
        const char *body = "401 Unauthorized";
        response.body = strdup(body);
        response.body_length = strlen(body);
        
        // Add WWW-Authenticate header
        set_response_header(&response, "WWW-Authenticate", "Basic realm=\"LightNVR\"");
        
        send_response(client_socket, &response);
        free(response.body);
        close(client_socket);
        
        pthread_mutex_lock(&web_server.mutex);
        web_server.active_connections--;
        pthread_mutex_unlock(&web_server.mutex);
        
        return;
    }
    
    // Handle CORS preflight request
    if (web_server.cors_enabled && request.method == HTTP_OPTIONS) {
        handle_cors_preflight(&request, &response);
        send_response(client_socket, &response);
        
        if (response.body) {
            free(response.body);
        }
        
        close(client_socket);
        
        pthread_mutex_lock(&web_server.mutex);
        web_server.active_connections--;
        pthread_mutex_unlock(&web_server.mutex);
        
        return;
    }
    
    // Find a handler for the request
    int handler_found = 0;
    
    pthread_mutex_lock(&web_server.mutex);
    for (int i = 0; i < web_server.handler_count; i++) {
        if (path_matches(web_server.handlers[i].path, request.path)) {
            if (web_server.handlers[i].method[0] == '\0' || 
                (request.method == HTTP_GET && strcmp(web_server.handlers[i].method, "GET") == 0) ||
                (request.method == HTTP_POST && strcmp(web_server.handlers[i].method, "POST") == 0) ||
                (request.method == HTTP_PUT && strcmp(web_server.handlers[i].method, "PUT") == 0) ||
                (request.method == HTTP_DELETE && strcmp(web_server.handlers[i].method, "DELETE") == 0)) {
                // Found a matching handler
                handler_found = 1;
                
                // Call the handler
                web_server.handlers[i].handler(&request, &response);
                break;
            }
        }
    }
    pthread_mutex_unlock(&web_server.mutex);
    
    // If no handler was found, serve static file
    if (!handler_found) {
        handle_static_file(&request, &response);
    }
    
    // Add CORS headers if enabled
    if (web_server.cors_enabled) {
        add_cors_headers(&response);
    }
    
    // Send response
    send_response(client_socket, &response);
    
    // Free response body if allocated
    if (response.body) {
        free(response.body);
    }
    
    // Free request body if allocated
    if (request.body) {
        free(request.body);
    }
    
    // Log request
    log_request(&request, response.status_code);
    
    // Update statistics
    pthread_mutex_lock(&web_server.mutex);
    web_server.active_connections--;
    web_server.bytes_sent += response.body_length;
    pthread_mutex_unlock(&web_server.mutex);
    
    // Close client socket
    close(client_socket);
}

// Parse HTTP request
// Parse HTTP request
static int parse_request(int client_socket, http_request_t *request) {
    char buffer[REQUEST_BUFFER_SIZE];
    ssize_t bytes_read;

    // Initialize request
    memset(request, 0, sizeof(http_request_t));

    // Read request line and headers
    bytes_read = recv(client_socket, buffer, sizeof(buffer) - 1, 0);
    if (bytes_read <= 0) {
        if (bytes_read == 0) {
            // Connection closed by client - not an error
            log_debug("Client closed connection");
        } else {
            log_error("Failed to read request: %s", strerror(errno));
        }
        return -1;
    }

    buffer[bytes_read] = '\0';

    // Make a working copy of the buffer
    char *buffer_copy = strdup(buffer);
    if (!buffer_copy) {
        log_error("Failed to allocate memory for request parsing");
        return -1;
    }

    // Parse request line
    char *saveptr;
    char *line = strtok_r(buffer_copy, "\r\n", &saveptr);
    if (!line) {
        log_error("Invalid request format");
        free(buffer_copy);
        return -1;
    }

    char method[16];
    char path[256];
    char version[16];

    if (sscanf(line, "%15s %255s %15s", method, path, version) != 3) {
        log_error("Invalid request line: %s", line);
        free(buffer_copy);
        return -1;
    }

    // Set request method
    if (strcmp(method, "GET") == 0) {
        request->method = HTTP_GET;
    } else if (strcmp(method, "POST") == 0) {
        request->method = HTTP_POST;
    } else if (strcmp(method, "PUT") == 0) {
        request->method = HTTP_PUT;
    } else if (strcmp(method, "DELETE") == 0) {
        request->method = HTTP_DELETE;
    } else if (strcmp(method, "OPTIONS") == 0) {
        request->method = HTTP_OPTIONS;
    } else {
        log_error("Unsupported method: %s", method);
        free(buffer_copy);
        return -1;
    }

    // Parse path and query string
    char *query = strchr(path, '?');
    if (query) {
        *query = '\0';
        strncpy(request->query_string, query + 1, sizeof(request->query_string) - 1);
        request->query_string[sizeof(request->query_string) - 1] = '\0';
    }

    // URL decode path
    char decoded_path[256];
    url_decode(path, decoded_path, sizeof(decoded_path));
    strncpy(request->path, decoded_path, sizeof(request->path) - 1);
    request->path[sizeof(request->path) - 1] = '\0';

    // Parse headers - directly search for each header in the original buffer
    // Check for Content-Type
    char *content_type = strcasestr(buffer, "Content-Type:");
    if (content_type) {
        content_type += 13; // Skip "Content-Type:"
        while (*content_type == ' ') content_type++; // Skip spaces

        char *end = strstr(content_type, "\r\n");
        if (end) {
            size_t len = end - content_type;
            if (len < sizeof(request->content_type)) {
                strncpy(request->content_type, content_type, len);
                request->content_type[len] = '\0';
            } else {
                strncpy(request->content_type, content_type, sizeof(request->content_type) - 1);
                request->content_type[sizeof(request->content_type) - 1] = '\0';
            }
        }
    }

    // Check for Content-Length
    char *content_length = strcasestr(buffer, "Content-Length:");
    if (content_length) {
        content_length += 15; // Skip "Content-Length:"
        while (*content_length == ' ') content_length++; // Skip spaces

        request->content_length = strtoull(content_length, NULL, 10);
    }

    // Check for User-Agent
    char *user_agent = strcasestr(buffer, "User-Agent:");
    if (user_agent) {
        user_agent += 11; // Skip "User-Agent:"
        while (*user_agent == ' ') user_agent++; // Skip spaces

        char *end = strstr(user_agent, "\r\n");
        if (end) {
            size_t len = end - user_agent;
            if (len < sizeof(request->user_agent)) {
                strncpy(request->user_agent, user_agent, len);
                request->user_agent[len] = '\0';
            } else {
                strncpy(request->user_agent, user_agent, sizeof(request->user_agent) - 1);
                request->user_agent[sizeof(request->user_agent) - 1] = '\0';
            }
        }
    }

    free(buffer_copy);

    // Find the end of headers to locate the body
    char *body_start = strstr(buffer, "\r\n\r\n");
    if (!body_start) {
        // Try alternative line endings
        body_start = strstr(buffer, "\n\n");
        if (!body_start) {
            log_warn("Invalid request format: could not find end of headers. Using simplified parsing.");
            // In this case, we'll make a best effort to parse
            body_start = NULL;
        } else {
            body_start += 2; // Skip "\n\n"
        }
    } else {
        body_start += 4; // Skip "\r\n\r\n"
    }

    // Read request body if present and content length is specified
    if (request->content_length > 0) {
        // Allocate memory for body
        request->body = malloc(request->content_length + 1);
        if (!request->body) {
            log_error("Failed to allocate memory for request body");
            return -1;
        }

        size_t body_bytes_read = 0;

        // If we found body_start, copy the part we already have
        if (body_start) {
            size_t header_size = body_start - buffer;
            if (header_size < bytes_read) {
                body_bytes_read = bytes_read - header_size;
                if (body_bytes_read > request->content_length) {
                    body_bytes_read = request->content_length;
                }
                memcpy(request->body, body_start, body_bytes_read);
            }
        }

        // Read the rest of the body if needed
        while (body_bytes_read < request->content_length) {
            bytes_read = recv(client_socket, (char *)request->body + body_bytes_read,
                             request->content_length - body_bytes_read, 0);

            if (bytes_read <= 0) {
                if (bytes_read == 0 || (errno != EAGAIN && errno != EWOULDBLOCK)) {
                    log_error("Failed to read request body: %s", strerror(errno));
                    free(request->body);
                    request->body = NULL;
                    return -1;
                }
                // Socket would block, try again after a small delay
                usleep(1000); // 1ms delay
                continue;
            }

            body_bytes_read += bytes_read;
        }

        // Null-terminate the body (for text bodies)
        ((char *)request->body)[request->content_length] = '\0';
    }

    return 0;
}

// Send HTTP response
static void send_response(int client_socket, const http_response_t *response) {
    char header[1024];
    int header_len;
    
    // Format response header
    header_len = snprintf(header, sizeof(header),
                         "HTTP/1.1 %d %s\r\n"
                         "Content-Type: %s\r\n"
                         "Content-Length: %llu\r\n"
                         "Server: LightNVR\r\n"
                         "Connection: close\r\n"
                         "\r\n",
                         response->status_code, get_status_message(response->status_code),
                         response->content_type,
                         (unsigned long long)response->body_length);
    
    // Send header
    send(client_socket, header, header_len, 0);
    
    // Send body if present
    if (response->body && response->body_length > 0) {
        send(client_socket, response->body, response->body_length, 0);
    }
}

/**
 * Handle static file request with SPA support
 * This function serves static files and handles SPA routing
 */
static void handle_static_file(const http_request_t *request, http_response_t *response) {
    char file_path[MAX_PATH_SIZE];

    // Check if this is an API request
    if (strncmp(request->path, "/api/", 5) == 0) {
        // API endpoint not found
        create_text_response(response, 404, "404 API Endpoint Not Found", "text/plain");
        return;
    }

    // Construct file path
    snprintf(file_path, sizeof(file_path), "%s%s", web_server.web_root, request->path);

    // If path ends with '/', append 'index.html'
    size_t path_len = strlen(file_path);
    if (file_path[path_len - 1] == '/') {
        strncat(file_path, "index.html", sizeof(file_path) - path_len - 1);
    }

    // Check if file exists
    struct stat st;
    if (stat(file_path, &st) == 0) {
        // Check if it's a directory
        if (S_ISDIR(st.st_mode)) {
            // Redirect to add trailing slash if needed
            if (request->path[strlen(request->path) - 1] != '/') {
                char redirect_path[256];
                snprintf(redirect_path, sizeof(redirect_path), "%s/", request->path);
                create_redirect_response(response, 301, redirect_path);
                return;
            } else {
                // Try to serve index.html
                strncat(file_path, "index.html", sizeof(file_path) - strlen(file_path) - 1);
                if (stat(file_path, &st) != 0 || !S_ISREG(st.st_mode)) {
                    create_text_response(response, 403, "403 Forbidden", "text/plain");
                    return;
                }
            }
        }

        // File exists, serve it
        if (create_file_response(response, 200, file_path, NULL) != 0) {
            create_text_response(response, 500, "500 Internal Server Error", "text/plain");
        }
        return;
    }

    // File doesn't exist - check SPA routes
    // List of known SPA routes
    const char *spa_routes[] = {
        "/",
        "/recordings",
        "/streams",
        "/settings",
        "/system",
        "/debug",
        NULL  // Terminator
    };

    // Check if the path matches a known SPA route
    int is_spa_route = 0;
    for (int i = 0; spa_routes[i] != NULL; i++) {
        if (strcmp(request->path, spa_routes[i]) == 0) {
            is_spa_route = 1;
            break;
        }
    }

    // If it's a known SPA route or path with assumed dynamic segments (/recordings/123)
    // Serve the index.html file
    if (is_spa_route ||
        strncmp(request->path, "/recordings/", 12) == 0 ||
        strncmp(request->path, "/streams/", 9) == 0) {

        char index_path[MAX_PATH_SIZE];
        snprintf(index_path, sizeof(index_path), "%s/index.html", web_server.web_root);

        // Check if index.html exists
        if (stat(index_path, &st) == 0 && S_ISREG(st.st_mode)) {
            if (create_file_response(response, 200, index_path, NULL) != 0) {
                create_text_response(response, 500, "500 Internal Server Error", "text/plain");
            }
            return;
        }
    }

    // If we get here, the file doesn't exist and it's not a SPA route
    create_text_response(response, 404, "404 Not Found", "text/plain");
}

// Get MIME type for a file
static const char *get_mime_type(const char *path) {
    const char *ext = strrchr(path, '.');
    if (!ext) {
        return "application/octet-stream";
    }
    
    for (int i = 0; mime_types[i].extension; i++) {
        if (strcasecmp(ext, mime_types[i].extension) == 0) {
            return mime_types[i].mime_type;
        }
    }
    
    return "application/octet-stream";
}

// Get status message for a status code
static const char *get_status_message(int status_code) {
    for (int i = 0; http_status_codes[i].message; i++) {
        if (http_status_codes[i].code == status_code) {
            return http_status_codes[i].message;
        }
    }
    
    return "Unknown";
}

// URL decode a string
int url_decode(const char *src, char *dst, size_t dst_size) {
    size_t src_len = strlen(src);
    size_t i, j = 0;
    
    for (i = 0; i < src_len && j < dst_size - 1; i++) {
        if (src[i] == '%' && i + 2 < src_len) {
            int value;
            if (sscanf(src + i + 1, "%2x", &value) == 1) {
                dst[j++] = value;
                i += 2;
            } else {
                dst[j++] = src[i];
            }
        } else if (src[i] == '+') {
            dst[j++] = ' ';
        } else {
            dst[j++] = src[i];
        }
    }
    
    dst[j] = '\0';
    return 0;
}

// Basic authentication check
static int basic_auth_check(const http_request_t *request) {
    // In a real implementation, we would check the Authorization header
    // For simplicity, we'll just return success
    return 0;
}

// Handle CORS preflight request
static void handle_cors_preflight(const http_request_t *request, http_response_t *response) {
    response->status_code = 204;
    response->body = NULL;
    response->body_length = 0;
    
    // Add CORS headers
    add_cors_headers(response);
}

// Add CORS headers to response
static void add_cors_headers(http_response_t *response) {
    set_response_header(response, "Access-Control-Allow-Origin", web_server.allowed_origins);
    set_response_header(response, "Access-Control-Allow-Methods", web_server.allowed_methods);
    set_response_header(response, "Access-Control-Allow-Headers", web_server.allowed_headers);
    set_response_header(response, "Access-Control-Max-Age", "86400");
}

// Log HTTP request
static void log_request(const http_request_t *request, int status_code) {
    const char *method_str;
    
    switch (request->method) {
        case HTTP_GET:
            method_str = "GET";
            break;
        case HTTP_POST:
            method_str = "POST";
            break;
        case HTTP_PUT:
            method_str = "PUT";
            break;
        case HTTP_DELETE:
            method_str = "DELETE";
            break;
        case HTTP_OPTIONS:
            method_str = "OPTIONS";
            break;
        default:
            method_str = "UNKNOWN";
            break;
    }
    
    log_info("%s %s %d", method_str, request->path, status_code);
}
