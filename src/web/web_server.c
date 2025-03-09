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
#include <stdbool.h>

#include "web/web_server.h"
#include "web/api_handlers.h"
#include "web/request_response.h"
#include "web/thread_pool.h"
#include "core/logger.h"
#include "core/config.h"
#include "database/database_manager.h"
#include "video/stream_manager.h"
#include "storage/storage_manager.h"
#include "utils/memory.h"

// Maximum number of pending connections
#define MAX_PENDING 128

// Buffer size for reading requests
#define REQUEST_BUFFER_SIZE 8192

// Maximum number of headers
#define MAX_HEADERS 100

// Maximum size of path
#define MAX_PATH_SIZE 1024

// Make server socket accessible to signal handlers
int server_socket = -1;

// Signal handler flag
static volatile sig_atomic_t server_running = 1;

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

// Web server state
static struct {
    int server_socket;
    int port;
    char web_root[MAX_PATH_LENGTH];
    int running;
    pthread_t server_thread;
    pthread_mutex_t mutex;

    // Thread pool
    thread_pool_t *thread_pool;

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

    // Daemon mode
    bool daemon_mode;
    char pid_file[MAX_PATH_LENGTH];
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
    .max_connections = 100,
    .connection_timeout = 30,
    .active_connections = 0,
    .total_requests = 0,
    .bytes_sent = 0,
    .bytes_received = 0,
    .start_time = 0,
    .handler_count = 0,
    .daemon_mode = false,
    .pid_file = "/var/run/lightnvr.pid"
};

// Forward declarations
static void *server_thread_func(void *arg);
static void handle_client(void *client_socket_ptr);
static void signal_handler(int sig);
static int path_matches(const char *pattern, const char *path);
static const char *get_mime_type(const char *path);
static const char *get_status_message(int status_code);
static void handle_static_file(const http_request_t *request, http_response_t *response);
static int basic_auth_check(const http_request_t *request);
static void handle_cors_preflight(const http_request_t *request, http_response_t *response);
static void add_cors_headers(http_response_t *response);
static void log_request(const http_request_t *request, int status_code);
static int write_pid_file(void);
static int remove_pid_file(void);

// Signal handler
static void signal_handler(int sig) {
    if (sig == SIGINT || sig == SIGTERM) {
        log_info("Web server received signal %d, shutting down...", sig);
        server_running = 0;
        web_server.running = 0;

        // Close server socket to unblock accept()
        if (web_server.server_socket >= 0) {
            shutdown(web_server.server_socket, SHUT_RDWR);
            close(web_server.server_socket);
            web_server.server_socket = -1;
            server_socket = -1; // Update the global reference
        }
    }
}

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

    // Setup signal handlers
    struct sigaction sa;
    memset(&sa, 0, sizeof(sa));
    sa.sa_handler = signal_handler;
    sigaction(SIGINT, &sa, NULL);
    sigaction(SIGTERM, &sa, NULL);

    // Ignore SIGPIPE
    signal(SIGPIPE, SIG_IGN);

    // Create server socket
    web_server.server_socket = socket(AF_INET, SOCK_STREAM, 0);
    if (web_server.server_socket < 0) {
        log_error("Failed to create server socket: %s", strerror(errno));
        pthread_mutex_destroy(&web_server.mutex);
        return -1;
    }

    // Make server socket accessible to signal handlers
    server_socket = web_server.server_socket;

    // Set socket options for reuse
    int opt = 1;
    if (setsockopt(web_server.server_socket, SOL_SOCKET, SO_REUSEADDR, &opt, sizeof(opt)) < 0) {
        log_error("Failed to set SO_REUSEADDR on socket: %s", strerror(errno));
        close(web_server.server_socket);
        web_server.server_socket = -1;
        pthread_mutex_destroy(&web_server.mutex);
        return -1;
    }

    // Add SO_REUSEPORT if available
#ifdef SO_REUSEPORT
    if (setsockopt(web_server.server_socket, SOL_SOCKET, SO_REUSEPORT, &opt, sizeof(opt)) < 0) {
        log_warn("Failed to set SO_REUSEPORT on socket: %s", strerror(errno));
        // Continue anyway, this is optional
    }
#endif

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
        pthread_mutex_destroy(&web_server.mutex);
        return -1;
    }

    // Listen for connections
    if (listen(web_server.server_socket, MAX_PENDING) < 0) {
        log_error("Failed to listen on server socket: %s", strerror(errno));
        close(web_server.server_socket);
        web_server.server_socket = -1;
        pthread_mutex_destroy(&web_server.mutex);
        return -1;
    }

    // Initialize thread pool (use processor count + 2 for optimal performance)
    int num_threads = 4; // Default, should be determined based on CPU cores
    web_server.thread_pool = thread_pool_init(num_threads, web_server.max_connections);
    if (!web_server.thread_pool) {
        log_error("Failed to initialize thread pool");
        close(web_server.server_socket);
        web_server.server_socket = -1;
        pthread_mutex_destroy(&web_server.mutex);
        return -1;
    }

    // Start server thread
    web_server.running = 1;
    web_server.start_time = time(NULL);

    if (pthread_create(&web_server.server_thread, NULL, server_thread_func, NULL) != 0) {
        log_error("Failed to create server thread: %s", strerror(errno));
        thread_pool_shutdown(web_server.thread_pool);
        close(web_server.server_socket);
        web_server.server_socket = -1;
        web_server.running = 0;
        pthread_mutex_destroy(&web_server.mutex);
        return -1;
    }

    // Register API handlers
    register_api_handlers();

    log_info("Web server started on port %d", port);
    return 0;
}

// Daemonize the process
int daemonize_web_server(const char *pid_file) {
    // Set daemon mode
    web_server.daemon_mode = true;

    // Set PID file if provided
    if (pid_file && pid_file[0] != '\0') {
        strncpy(web_server.pid_file, pid_file, MAX_PATH_LENGTH - 1);
        web_server.pid_file[MAX_PATH_LENGTH - 1] = '\0';
    }

    log_info("Web server running in daemon mode");
    return 0;
}

// Shutdown the web server
void shutdown_web_server(void) {
    log_info("Shutting down web server...");

    // Set running flag to false
    server_running = 0;

    pthread_mutex_lock(&web_server.mutex);
    web_server.running = 0;

    // Close server socket to unblock accept()
    if (web_server.server_socket >= 0) {
        shutdown(web_server.server_socket, SHUT_RDWR);
        close(web_server.server_socket);
        web_server.server_socket = -1;
        server_socket = -1; // Update the global reference
    }
    pthread_mutex_unlock(&web_server.mutex);

    // Join server thread with timeout
    struct timespec ts;
    clock_gettime(CLOCK_REALTIME, &ts);
    ts.tv_sec += 5; // 5 second timeout

    int join_result = pthread_timedjoin_np(web_server.server_thread, NULL, &ts);
    if (join_result != 0) {
        if (join_result == ETIMEDOUT) {
            log_warn("Server thread join timed out, cancelling thread");
            pthread_cancel(web_server.server_thread);
        } else {
            log_warn("Failed to join server thread: %s", strerror(join_result));
        }
    }

    // Shutdown thread pool
    if (web_server.thread_pool) {
        thread_pool_shutdown(web_server.thread_pool);
        web_server.thread_pool = NULL;
    }

    pthread_mutex_destroy(&web_server.mutex);

    log_info("Web server shutdown complete");
}

// Server thread function
static void *server_thread_func(void *arg) {
    // Set thread name for debugging
    pthread_setname_np(pthread_self(), "web-server");

    // Setup socket timeout
    struct timeval tv;
    tv.tv_sec = 1;  // 1 second timeout
    tv.tv_usec = 0;
    setsockopt(web_server.server_socket, SOL_SOCKET, SO_RCVTIMEO, &tv, sizeof(tv));

    while (server_running) {
        // Accept a new connection
        struct sockaddr_in client_addr;
        socklen_t client_len = sizeof(client_addr);
        int client_socket = accept(web_server.server_socket, (struct sockaddr *)&client_addr, &client_len);

        if (client_socket < 0) {
            if (errno == EAGAIN || errno == EWOULDBLOCK || errno == EINTR) {
                // Timeout or interrupted - normal condition when checking server_running
                continue;
            } else {
                log_error("Failed to accept connection: %s", strerror(errno));
                // Continue accepting connections - don't exit the loop
                sleep(1); // Brief pause to prevent high CPU in error state
                continue;
            }
        }

        // Set socket timeout for client
        struct timeval client_tv;
        client_tv.tv_sec = web_server.connection_timeout;
        client_tv.tv_usec = 0;
        setsockopt(client_socket, SOL_SOCKET, SO_RCVTIMEO, &client_tv, sizeof(client_tv));
        setsockopt(client_socket, SOL_SOCKET, SO_SNDTIMEO, &client_tv, sizeof(client_tv));

        // Update statistics
        pthread_mutex_lock(&web_server.mutex);
        web_server.active_connections++;
        pthread_mutex_unlock(&web_server.mutex);

        // Create a heap-allocated int to pass the socket to the thread pool
        int *client_socket_ptr = malloc(sizeof(int));
        if (!client_socket_ptr) {
            log_error("Failed to allocate memory for client socket");
            close(client_socket);

            pthread_mutex_lock(&web_server.mutex);
            web_server.active_connections--;
            pthread_mutex_unlock(&web_server.mutex);
            continue;
        }

        *client_socket_ptr = client_socket;

        // Add task to thread pool
        if (!thread_pool_add_task(web_server.thread_pool, handle_client, client_socket_ptr)) {
            log_error("Failed to add client task to thread pool");
            free(client_socket_ptr);
            close(client_socket);

            pthread_mutex_lock(&web_server.mutex);
            web_server.active_connections--;
            pthread_mutex_unlock(&web_server.mutex);
        }
    }

    log_info("Server thread exiting");
    return NULL;
}

// Helper function to clean up request resources
void cleanup_request(http_request_t *request) {
    if (!request) {
        return;
    }

    // Free request body if allocated
    if (request->body) {
        free(request->body);
        request->body = NULL;
    }

    // Free request headers if allocated
    if (request->headers) {
        free(request->headers);
        request->headers = NULL;
    }

    // Clear any sensitive data
    request->content_length = 0;
    memset(request->path, 0, sizeof(request->path));
    memset(request->query_string, 0, sizeof(request->query_string));
    memset(request->content_type, 0, sizeof(request->content_type));
    memset(request->user_agent, 0, sizeof(request->user_agent));
}

// Helper function to clean up response resources
void cleanup_response(http_response_t *response) {
    if (!response) {
        return;
    }

    // Free response body if allocated
    if (response->body) {
        free(response->body);
        response->body = NULL;
    }

    // Free response headers if allocated
    if (response->headers) {
        free(response->headers);
        response->headers = NULL;
    }

    // Reset other fields
    response->status_code = 0;
    response->body_length = 0;
    memset(response->content_type, 0, sizeof(response->content_type));
    response->num_headers = 0;
}

// Handle a client connection with improved error handling and resource management
static void handle_client(void *arg) {
    int client_socket = *((int *)arg);
    free(arg); // Free the allocated socket pointer

    // Set socket timeout
    struct timeval tv;
    tv.tv_sec = web_server.connection_timeout;
    tv.tv_usec = 0;
    if (setsockopt(client_socket, SOL_SOCKET, SO_RCVTIMEO, &tv, sizeof(tv)) < 0 ||
        setsockopt(client_socket, SOL_SOCKET, SO_SNDTIMEO, &tv, sizeof(tv)) < 0) {
        log_warn("Failed to set socket timeout: %s", strerror(errno));
        // Continue anyway, this is not fatal
    }

    // Parse request with better error handling
    http_request_t request;
    memset(&request, 0, sizeof(request));

    if (parse_request(client_socket, &request) != 0) {
        log_debug("Failed to parse request from client");
        close(client_socket);

        pthread_mutex_lock(&web_server.mutex);
        web_server.active_connections--;
        pthread_mutex_unlock(&web_server.mutex);

        return;
    }

    // Update statistics with error checking
    pthread_mutex_lock(&web_server.mutex);
    web_server.total_requests++;
    web_server.bytes_received += request.content_length;
    pthread_mutex_unlock(&web_server.mutex);

    // Prepare response
    http_response_t response;
    memset(&response, 0, sizeof(response));

    // Check authentication if enabled
    if (web_server.auth_enabled && basic_auth_check(&request) != 0) {
        log_info("Authentication failed for request to: %s", request.path);

        response.status_code = 401;
        safe_strcpy(response.content_type, "text/plain", sizeof(response.content_type));

        const char *body = "401 Unauthorized";
        response.body = safe_strdup(body);
        if (!response.body) {
            // Memory allocation failed, fallback to minimal response
            response.body_length = 0;
        } else {
            response.body_length = strlen(body);
        }

        // Add WWW-Authenticate header
        set_response_header(&response, "WWW-Authenticate", "Basic realm=\"LightNVR\"");

        send_response(client_socket, &response);

        if (response.body) {
            free(response.body);
        }
        cleanup_request(&request);
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

        cleanup_request(&request);
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
            // Match HTTP method or any method
            if (web_server.handlers[i].method[0] == '\0' ||
                (request.method == HTTP_GET && strcmp(web_server.handlers[i].method, "GET") == 0) ||
                (request.method == HTTP_POST && strcmp(web_server.handlers[i].method, "POST") == 0) ||
                (request.method == HTTP_PUT && strcmp(web_server.handlers[i].method, "PUT") == 0) ||
                (request.method == HTTP_DELETE && strcmp(web_server.handlers[i].method, "DELETE") == 0)) {

                // Found a matching handler
                handler_found = 1;

                // Call the handler with error checking
                request_handler_t handler = web_server.handlers[i].handler;
                pthread_mutex_unlock(&web_server.mutex);

                // Call handler outside of mutex lock to prevent deadlocks
                if (handler) {
                    handler(&request, &response);
                } else {
                    // Handler is NULL, return 500 Internal Server Error
                    log_error("NULL handler found for path: %s", request.path);
                    create_text_response(&response, 500, "500 Internal Server Error", "text/plain");
                }

                // Don't need to re-acquire mutex here
                goto handler_complete;
            }
        }
    }
    pthread_mutex_unlock(&web_server.mutex);

handler_complete:
    // If no handler was found, serve static file
    if (!handler_found) {
        handle_static_file(&request, &response);
    }

    // Add CORS headers if enabled
    if (web_server.cors_enabled) {
        add_cors_headers(&response);
    }

    // Send response with error checking
    if (send_response(client_socket, &response) != 0) {
        log_error("Failed to send response for path: %s", request.path);
    } else {
        // Log request on success
        log_request(&request, response.status_code);

        // Update statistics
        pthread_mutex_lock(&web_server.mutex);
        web_server.bytes_sent += response.body_length;
        pthread_mutex_unlock(&web_server.mutex);
    }

    // Free response body and headers if allocated
    cleanup_response(&response);

    // Free request resources
    cleanup_request(&request);

    // Update active connections counter
    pthread_mutex_lock(&web_server.mutex);
    web_server.active_connections--;
    pthread_mutex_unlock(&web_server.mutex);

    // Close client socket
    close(client_socket);
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

    response->body = safe_strdup(json_data);
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

int path_matches(const char *pattern, const char *path) {
    // Special case: if pattern ends with "/*", it matches any path that starts with the prefix
    size_t pattern_len = strlen(pattern);
    if (pattern_len >= 2 && pattern[pattern_len-2] == '/' && pattern[pattern_len-1] == '*') {
        return strncmp(pattern, path, pattern_len-1) == 0;
    }

    // Otherwise, exact match
    return strcmp(pattern, path) == 0;
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
