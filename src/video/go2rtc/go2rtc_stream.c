/**
 * @file go2rtc_stream.c
 * @brief Implementation of the go2rtc stream integration module
 */

#include "video/go2rtc/go2rtc_stream.h"
#include "video/go2rtc/go2rtc_process.h"
#include "video/go2rtc/go2rtc_api.h"
#include "video/go2rtc/go2rtc_integration.h"
#include "core/logger.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <sys/time.h>
#include <errno.h>
#include <curl/curl.h>
#include <fcntl.h>
#include <netdb.h>

// Default API host
#define DEFAULT_API_HOST "localhost"

// Buffer sizes
#define URL_BUFFER_SIZE 2048

// Stream integration state
static bool g_initialized = false;
static int g_api_port = 0;
static char *g_config_dir = NULL;  // Store config directory for later use

bool go2rtc_stream_init(const char *binary_path, const char *config_dir, int api_port) {
    if (g_initialized) {
        log_warn("go2rtc stream integration already initialized");
        return false;
    }
    
    if (!binary_path || !config_dir || api_port <= 0) {
        log_error("Invalid parameters for go2rtc_stream_init");
        return false;
    }
    
    // Store config directory
    g_config_dir = strdup(config_dir);
    if (!g_config_dir) {
        log_error("Failed to allocate memory for config directory");
        return false;
    }
    
    // Initialize process manager
    if (!go2rtc_process_init(binary_path, config_dir)) {
        log_error("Failed to initialize go2rtc process manager");
        free(g_config_dir);
        g_config_dir = NULL;
        return false;
    }
    
    // Initialize API client
    if (!go2rtc_api_init(DEFAULT_API_HOST, api_port)) {
        log_error("Failed to initialize go2rtc API client");
        go2rtc_process_cleanup();
        free(g_config_dir);
        g_config_dir = NULL;
        return false;
    }
    
    g_api_port = api_port;
    g_initialized = true;
    
    log_info("go2rtc stream integration initialized with binary: %s, config dir: %s, API port: %d", 
             binary_path, config_dir, api_port);
    
    return true;
}

bool go2rtc_stream_register(const char *stream_id, const char *stream_url, 
                           const char *username, const char *password) {
    if (!g_initialized) {
        log_error("go2rtc stream integration not initialized");
        return false;
    }
    
    if (!stream_id || !stream_url) {
        log_error("Invalid parameters for go2rtc_stream_register");
        return false;
    }
    
    // Log the input parameters for debugging
    log_info("Registering stream with go2rtc: id=%s, url=%s, username=%s", 
             stream_id, stream_url, username ? username : "none");
    
    // Ensure go2rtc is running
    if (!go2rtc_stream_is_ready()) {
        log_info("go2rtc not running, starting service");
        if (!go2rtc_stream_start_service()) {
            log_error("Failed to start go2rtc service");
            return false;
        }
        
        // Wait for service to start with increased retries
        int retries = 10;
        while (retries > 0 && !go2rtc_stream_is_ready()) {
            log_info("Waiting for go2rtc service to be ready (%d retries left)...", retries);
            sleep(1);
            retries--;
        }
        
        if (!go2rtc_stream_is_ready()) {
            log_error("go2rtc service failed to start in time");
            return false;
        }
    }
    
    // Use a static buffer for the modified URL to avoid memory allocation issues
    char modified_url[URL_BUFFER_SIZE];
    strncpy(modified_url, stream_url, URL_BUFFER_SIZE - 1);
    modified_url[URL_BUFFER_SIZE - 1] = '\0';
    
    // If username and password are provided but not in the URL, add them
    if (username && password && strstr(modified_url, "@") == NULL) {
        // URL doesn't contain credentials, need to add them
        char new_url[URL_BUFFER_SIZE];
        
        // Extract protocol and host parts
        char *protocol_end = strstr(modified_url, "://");
        if (protocol_end) {
            // Format: protocol://username:password@rest_of_url
            char protocol[16] = {0};
            size_t protocol_len = protocol_end - modified_url;
            if (protocol_len < sizeof(protocol)) {
                strncpy(protocol, modified_url, protocol_len);
                protocol[protocol_len] = '\0';
                
                snprintf(new_url, URL_BUFFER_SIZE, "%s://%s:%s@%s", 
                         protocol, username, password, protocol_end + 3);
                
                strncpy(modified_url, new_url, URL_BUFFER_SIZE - 1);
                modified_url[URL_BUFFER_SIZE - 1] = '\0';
                
                log_info("Added credentials to URL: %s", modified_url);
            }
        }
    }
    
    // Prepare stream options if authentication is provided
    char stream_options[URL_BUFFER_SIZE] = {0};
    if (username && password) {
        // Format options string
        snprintf(stream_options, URL_BUFFER_SIZE, 
                 "{\"auth\": {\"username\": \"%s\", \"password\": \"%s\"}}", 
                 username, password);
    }
    
    // Register stream with go2rtc
    bool result = go2rtc_api_add_stream(stream_id, modified_url, 
                                       username && password ? stream_options : NULL);
    
    if (result) {
        log_info("Successfully registered stream with go2rtc: %s", stream_id);
    } else {
        log_error("Failed to register stream with go2rtc: %s", stream_id);
    }
    
    return result;
}

bool go2rtc_stream_unregister(const char *stream_id) {
    if (!g_initialized) {
        log_error("go2rtc stream integration not initialized");
        return false;
    }
    
    if (!stream_id) {
        log_error("Invalid parameter for go2rtc_stream_unregister");
        return false;
    }
    
    // Check if go2rtc is running
    if (!go2rtc_stream_is_ready()) {
        log_warn("go2rtc service not running, cannot unregister stream");
        return false;
    }
    
    // Unregister stream from go2rtc
    bool result = go2rtc_api_remove_stream(stream_id);
    
    if (result) {
        log_info("Unregistered stream from go2rtc: %s", stream_id);
    } else {
        log_error("Failed to unregister stream from go2rtc: %s", stream_id);
    }
    
    return result;
}

bool go2rtc_stream_get_webrtc_url(const char *stream_id, char *buffer, size_t buffer_size) {
    if (!g_initialized) {
        log_error("go2rtc stream integration not initialized");
        return false;
    }
    
    if (!stream_id || !buffer || buffer_size == 0) {
        log_error("Invalid parameters for go2rtc_stream_get_webrtc_url");
        return false;
    }
    
    // Check if go2rtc is running
    if (!go2rtc_stream_is_ready()) {
        log_warn("go2rtc service not running, cannot get WebRTC URL");
        return false;
    }
    
    // Get WebRTC URL from API client
    return go2rtc_api_get_webrtc_url(stream_id, buffer, buffer_size);
}

// Callback function for libcurl to discard response data
static size_t discard_response(void *ptr, size_t size, size_t nmemb, void *userdata) {
    // Just return the size of the data to indicate we handled it
    return size * nmemb;
}

/**
 * @brief Check if a TCP port is open and accepting connections
 * 
 * @param host Hostname or IP address
 * @param port Port number
 * @param timeout_ms Timeout in milliseconds
 * @return true if port is open, false otherwise
 */
static bool is_port_open(const char *host, int port, int timeout_ms) {
    struct sockaddr_in addr;
    int sockfd;
    fd_set fdset;
    struct timeval tv;
    
    // Create socket
    sockfd = socket(AF_INET, SOCK_STREAM, 0);
    if (sockfd < 0) {
        log_warn("is_port_open: socket creation failed");
        return false;
    }
    
    // Set non-blocking
    int flags = fcntl(sockfd, F_GETFL, 0);
    fcntl(sockfd, F_SETFL, flags | O_NONBLOCK);
    
    // Set up address
    memset(&addr, 0, sizeof(addr));
    addr.sin_family = AF_INET;
    addr.sin_port = htons(port);
    
    // Convert hostname to IP
    if (inet_pton(AF_INET, host, &addr.sin_addr) <= 0) {
        // If not a valid IP, try to resolve hostname
        struct hostent *he = gethostbyname(host);
        if (!he) {
            log_warn("is_port_open: failed to resolve hostname: %s", host);
            close(sockfd);
            return false;
        }
        memcpy(&addr.sin_addr, he->h_addr_list[0], he->h_length);
    }
    
    // Try to connect
    int res = connect(sockfd, (struct sockaddr*)&addr, sizeof(addr));
    if (res < 0) {
        if (errno == EINPROGRESS) {
            // Connection in progress, wait for it
            tv.tv_sec = timeout_ms / 1000;
            tv.tv_usec = (timeout_ms % 1000) * 1000;
            FD_ZERO(&fdset);
            FD_SET(sockfd, &fdset);
            
            // Wait for connect to complete or timeout
            res = select(sockfd + 1, NULL, &fdset, NULL, &tv);
            if (res < 0) {
                log_warn("is_port_open: select error: %s", strerror(errno));
                close(sockfd);
                return false;
            } else if (res == 0) {
                // Timeout
                log_warn("is_port_open: connection timeout");
                close(sockfd);
                return false;
            }
            
            // Check if we actually connected
            int so_error;
            socklen_t len = sizeof(so_error);
            getsockopt(sockfd, SOL_SOCKET, SO_ERROR, &so_error, &len);
            if (so_error != 0) {
                log_warn("is_port_open: connection failed: %s", strerror(so_error));
                close(sockfd);
                return false;
            }
        } else {
            // Immediate connection failure
            log_warn("is_port_open: connection failed: %s", strerror(errno));
            close(sockfd);
            return false;
        }
    }
    
    // Connection successful
    close(sockfd);
    return true;
}

bool go2rtc_stream_is_ready(void) {
    if (!g_initialized) {
        log_warn("go2rtc_stream_is_ready: not initialized");
        return false;
    }
    
    // Check if process is running
    if (!go2rtc_process_is_running()) {
        log_warn("go2rtc_stream_is_ready: process not running");
        return false;
    }
    
    // First check if the port is open
    if (!is_port_open("localhost", g_api_port, 1000)) {
        log_warn("go2rtc_stream_is_ready: port %d is not open", g_api_port);
        return false;
    }
    
    // Use libcurl to check if the API is responsive
    CURL *curl;
    CURLcode res;
    char url[URL_BUFFER_SIZE];
    long http_code = 0;
    
    // Initialize curl
    curl = curl_easy_init();
    if (!curl) {
        log_warn("go2rtc_stream_is_ready: failed to initialize curl");
        return false;
    }
    
    // Format the URL for the API endpoint
    snprintf(url, sizeof(url), "http://localhost:%d/api/streams", g_api_port);
    
    // Set curl options
    curl_easy_setopt(curl, CURLOPT_URL, url);
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, discard_response);
    curl_easy_setopt(curl, CURLOPT_TIMEOUT, 2L); // 2 second timeout
    curl_easy_setopt(curl, CURLOPT_CONNECTTIMEOUT, 2L); // 2 second connect timeout
    curl_easy_setopt(curl, CURLOPT_NOSIGNAL, 1L); // Prevent curl from using signals
    
    // Perform the request
    res = curl_easy_perform(curl);
    
    // Check for errors
    if (res != CURLE_OK) {
        log_warn("go2rtc_stream_is_ready: curl request failed: %s", curl_easy_strerror(res));
        
        // Try a simpler HTTP request using a socket
        int sockfd;
        struct sockaddr_in server_addr;
        char request[256];
        char response[1024];
        
        // Create socket
        sockfd = socket(AF_INET, SOCK_STREAM, 0);
        if (sockfd < 0) {
            log_warn("go2rtc_stream_is_ready: socket creation failed");
            curl_easy_cleanup(curl);
            return false;
        }
        
        // Set up server address
        memset(&server_addr, 0, sizeof(server_addr));
        server_addr.sin_family = AF_INET;
        server_addr.sin_port = htons(g_api_port);
        server_addr.sin_addr.s_addr = inet_addr("127.0.0.1");
        
        // Set socket timeout
        struct timeval tv;
        tv.tv_sec = 2;
        tv.tv_usec = 0;
        setsockopt(sockfd, SOL_SOCKET, SO_RCVTIMEO, (const char*)&tv, sizeof tv);
        setsockopt(sockfd, SOL_SOCKET, SO_SNDTIMEO, (const char*)&tv, sizeof tv);
        
        // Connect to server
        if (connect(sockfd, (struct sockaddr*)&server_addr, sizeof(server_addr)) < 0) {
            log_warn("go2rtc_stream_is_ready: socket connect failed: %s", strerror(errno));
            close(sockfd);
            curl_easy_cleanup(curl);
            return false;
        }
        
        // Prepare HTTP request
        snprintf(request, sizeof(request), 
                "GET /api/streams HTTP/1.1\r\n"
                "Host: localhost:%d\r\n"
                "Connection: close\r\n"
                "\r\n", g_api_port);
        
        // Send request
        if (send(sockfd, request, strlen(request), 0) < 0) {
            log_warn("go2rtc_stream_is_ready: socket send failed: %s", strerror(errno));
            close(sockfd);
            curl_easy_cleanup(curl);
            return false;
        }
        
        // Receive response
        int bytes = recv(sockfd, response, sizeof(response) - 1, 0);
        if (bytes <= 0) {
            log_warn("go2rtc_stream_is_ready: socket recv failed: %s", strerror(errno));
            close(sockfd);
            curl_easy_cleanup(curl);
            return false;
        }
        
        // Null-terminate response
        response[bytes] = '\0';
        
        // Check if we got a valid HTTP response
        if (strstr(response, "HTTP/1.1 200") || strstr(response, "HTTP/1.1 302")) {
            log_info("go2rtc_stream_is_ready: socket HTTP request succeeded");
            close(sockfd);
            curl_easy_cleanup(curl);
            return true;
        }
        
        log_warn("go2rtc_stream_is_ready: socket HTTP request failed: %s", response);
        close(sockfd);
        curl_easy_cleanup(curl);
        return false;
    }
    
    // Get the HTTP response code
    curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &http_code);
    
    // Clean up
    curl_easy_cleanup(curl);
    
    // Check if we got a successful HTTP response (200)
    if (http_code == 200) {
        log_info("go2rtc_stream_is_ready: API is responsive (HTTP %ld)", http_code);
        return true;
    } else if (http_code > 0) {
        // We got some HTTP response, which means the server is running
        log_warn("go2rtc_stream_is_ready: API returned HTTP %ld", http_code);
        return true;
    } else {
        log_warn("go2rtc_stream_is_ready: API is not responsive (HTTP %ld)", http_code);
        return false;
    }
}

bool go2rtc_stream_start_service(void) {
    if (!g_initialized) {
        log_error("go2rtc stream integration not initialized");
        return false;
    }
    
    // Check if go2rtc is already running
    if (go2rtc_process_is_running()) {
        log_info("go2rtc service already running, checking if it's responsive");
        
        // Check if the service is already responsive
        if (go2rtc_stream_is_ready()) {
            log_info("Existing go2rtc service is responsive and ready to use");
            
            // Register all existing streams with go2rtc if integration module is initialized
            if (go2rtc_integration_is_initialized()) {
                log_info("Registering all existing streams with go2rtc");
                if (!go2rtc_integration_register_all_streams()) {
                    log_warn("Failed to register all streams with go2rtc");
                    // Continue anyway
                }
            } else {
                log_info("go2rtc integration module not initialized, skipping stream registration");
            }
                
            return true;
        }
        
        // If not immediately responsive, wait a bit and try again
        log_warn("Existing go2rtc service is not responding, will try to wait for it");
        
        // Wait for the service to be ready
        int retries = 10; // Increased retries to give more time for the service to become responsive
        while (retries > 0) {
            sleep(1);
            if (go2rtc_stream_is_ready()) {
                log_info("go2rtc service is now ready");
                
                // Register all existing streams with go2rtc if integration module is initialized
                if (go2rtc_integration_is_initialized()) {
                    log_info("Registering all existing streams with go2rtc");
                    if (!go2rtc_integration_register_all_streams()) {
                        log_warn("Failed to register all streams with go2rtc");
                        // Continue anyway
                    }
                } else {
                    log_info("go2rtc integration module not initialized, skipping stream registration");
                }
                
                return true;
            }
            log_info("Waiting for go2rtc service to be ready (%d retries left)...", retries);
            retries--;
        }
        
        // If still not responsive, log a warning but don't stop it
        log_warn("Existing go2rtc service is not responding to API requests");
        log_warn("Will continue using the existing service, but it may not work properly");
        return false;
    }
    
    // Start go2rtc process with our configuration
    log_info("Starting go2rtc process with API port %d", g_api_port);
    bool result = go2rtc_process_start(g_api_port);
    
    if (result) {
        log_info("go2rtc service started successfully");
        
        // Wait for the service to be ready
        int retries = 10;
        while (retries > 0) {
            sleep(1); // Sleep first to give the process time to start
            if (go2rtc_stream_is_ready()) {
                log_info("go2rtc service is ready");
                
                // Register all existing streams with go2rtc if integration module is initialized
                if (go2rtc_integration_is_initialized()) {
                    log_info("Registering all existing streams with go2rtc");
                    if (!go2rtc_integration_register_all_streams()) {
                        log_warn("Failed to register all streams with go2rtc");
                        // Continue anyway
                    }
                } else {
                    log_info("go2rtc integration module not initialized, skipping stream registration");
                }
                
                return true;
            }
            log_info("Waiting for go2rtc service to be ready (%d retries left)...", retries);
            retries--;
        }
        
        if (!go2rtc_stream_is_ready()) {
            log_error("go2rtc service started but is not responding to API requests");
            
            // Check if the process is still running
            if (go2rtc_process_is_running()) {
                log_warn("go2rtc process is running but not responding, checking port");
                
                // Check if the port is in use
                char cmd[128];
                snprintf(cmd, sizeof(cmd), "netstat -tlpn 2>/dev/null | grep ':%d'", g_api_port);
                FILE *fp = popen(cmd, "r");
                if (fp) {
                    char netstat_line[256];
                    bool port_in_use = false;
                    
                    if (fgets(netstat_line, sizeof(netstat_line), fp)) {
                        port_in_use = true;
                        log_warn("Port %d is in use: %s", g_api_port, netstat_line);
                    }
                    
                    pclose(fp);
                    
                    if (!port_in_use) {
                        log_error("go2rtc process is running but not listening on port %d", g_api_port);
                    }
                }
                
                // Try to get the process log
                char log_path[1024];
                if (g_config_dir) {
                    snprintf(log_path, sizeof(log_path), "%s/go2rtc.log", g_config_dir);
                    
                    log_warn("Checking go2rtc log file: %s", log_path);
                    fp = fopen(log_path, "r");
                    if (fp) {
                        char log_line[1024];
                        int lines = 0;
                        
                        // Skip to the end minus 10 lines
                        fseek(fp, 0, SEEK_END);
                        long pos = ftell(fp);
                        
                        // Read the last few lines
                        while (pos > 0 && lines < 10) {
                            pos--;
                            fseek(fp, pos, SEEK_SET);
                            char c = fgetc(fp);
                            if (c == '\n' && pos > 0) {
                                lines++;
                            }
                        }
                        
                        log_warn("Last few lines of go2rtc log:");
                        while (fgets(log_line, sizeof(log_line), fp)) {
                            // Remove newline
                            size_t len = strlen(log_line);
                            if (len > 0 && log_line[len-1] == '\n') {
                                log_line[len-1] = '\0';
                            }
                            log_warn("  %s", log_line);
                        }
                        
                        fclose(fp);
                    } else {
                        log_warn("Could not open go2rtc log file: %s", log_path);
                    }
                } else {
                    log_warn("Config directory not available, cannot check go2rtc log");
                }
            } else {
                log_error("go2rtc process is not running");
            }
            
            return false;
        }
    } else {
        log_error("Failed to start go2rtc service");
    }
    
    return result;
}

bool go2rtc_stream_stop_service(void) {
    if (!g_initialized) {
        log_error("go2rtc stream integration not initialized");
        return false;
    }
    
    // Stop all go2rtc processes, even if we didn't start them
    bool result = go2rtc_process_stop();
    
    if (result) {
        log_info("All go2rtc processes stopped successfully");
    } else {
        log_warn("Some go2rtc processes may still be running");
    }
    
    return result;
}

void go2rtc_stream_cleanup(void) {
    if (!g_initialized) {
        return;
    }
    
    // Stop all go2rtc processes
    go2rtc_stream_stop_service();
    
    // Clean up API client
    go2rtc_api_cleanup();
    
    // Clean up process manager
    go2rtc_process_cleanup();
    
    // Free config directory
    if (g_config_dir) {
        free(g_config_dir);
        g_config_dir = NULL;
    }
    
    g_initialized = false;
    g_api_port = 0;
    
    log_info("go2rtc stream integration cleaned up");
}
