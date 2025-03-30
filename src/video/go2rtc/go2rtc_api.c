/**
 * @file go2rtc_api.c
 * @brief Implementation of the go2rtc API client
 */

#include "video/go2rtc/go2rtc_api.h"
#include "core/logger.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <netdb.h>
#include <errno.h>
#include <curl/curl.h>

// API client configuration
static char *g_api_host = NULL;
static int g_api_port = 0;
static bool g_initialized = false;

// Buffer sizes
#define HTTP_BUFFER_SIZE 4096
#define URL_BUFFER_SIZE 1024

/**
 * @brief Send an HTTP request to the go2rtc API
 * 
 * @param method HTTP method (GET, POST, DELETE)
 * @param path API endpoint path
 * @param data Request body data (can be NULL)
 * @param response Buffer to store the response
 * @param response_size Size of the response buffer
 * @return int HTTP status code, or -1 on error
 */
static int send_http_request(const char *method, const char *path, const char *data, 
                             char *response, size_t response_size) {
    struct hostent *server;
    struct sockaddr_in server_addr;
    int sockfd, bytes, status_code = -1;
    char request[HTTP_BUFFER_SIZE];
    char buffer[HTTP_BUFFER_SIZE];
    
    // Create socket
    sockfd = socket(AF_INET, SOCK_STREAM, 0);
    if (sockfd < 0) {
        log_error("Failed to create socket: %s", strerror(errno));
        return -1;
    }
    
    // Resolve hostname
    server = gethostbyname(g_api_host);
    if (server == NULL) {
        log_error("Failed to resolve hostname: %s", g_api_host);
        close(sockfd);
        return -1;
    }
    
    // Set up server address
    memset(&server_addr, 0, sizeof(server_addr));
    server_addr.sin_family = AF_INET;
    memcpy(&server_addr.sin_addr.s_addr, server->h_addr_list[0], server->h_length);
    server_addr.sin_port = htons(g_api_port);
    
    // Connect to server
    if (connect(sockfd, (struct sockaddr *)&server_addr, sizeof(server_addr)) < 0) {
        log_error("Failed to connect to %s:%d: %s", g_api_host, g_api_port, strerror(errno));
        close(sockfd);
        return -1;
    }
    
    // Prepare HTTP request
    if (data) {
        snprintf(request, sizeof(request),
                 "%s %s HTTP/1.1\r\n"
                 "Host: %s:%d\r\n"
                 "Content-Type: application/json\r\n"
                 "Content-Length: %zu\r\n"
                 "Connection: close\r\n"
                 "\r\n"
                 "%s",
                 method, path, g_api_host, g_api_port, strlen(data), data);
    } else {
        snprintf(request, sizeof(request),
                 "%s %s HTTP/1.1\r\n"
                 "Host: %s:%d\r\n"
                 "Connection: close\r\n"
                 "\r\n",
                 method, path, g_api_host, g_api_port);
    }
    
    // Send request
    if (send(sockfd, request, strlen(request), 0) < 0) {
        log_error("Failed to send HTTP request: %s", strerror(errno));
        close(sockfd);
        return -1;
    }
    
    // Receive response
    memset(response, 0, response_size);
    size_t total_bytes = 0;
    
    while ((bytes = recv(sockfd, buffer, sizeof(buffer) - 1, 0)) > 0) {
        buffer[bytes] = '\0';
        
        // Extract status code from first line of response
        if (status_code == -1) {
            if (sscanf(buffer, "HTTP/1.1 %d", &status_code) != 1) {
                log_error("Failed to parse HTTP status code");
                close(sockfd);
                return -1;
            }
        }
        
        // Append to response buffer if there's space
        if (total_bytes + bytes < response_size) {
            memcpy(response + total_bytes, buffer, bytes);
            total_bytes += bytes;
        } else {
            log_warn("HTTP response truncated (buffer too small)");
            break;
        }
    }
    
    close(sockfd);
    return status_code;
}

/**
 * @brief Extract response body from HTTP response
 * 
 * @param response Full HTTP response
 * @return char* Pointer to the start of the response body, or NULL if not found
 */
static char *extract_response_body(char *response) {
    char *body = strstr(response, "\r\n\r\n");
    if (body) {
        return body + 4; // Skip the \r\n\r\n separator
    }
    return NULL;
}

bool go2rtc_api_init(const char *api_host, int api_port) {
    if (g_initialized) {
        log_warn("go2rtc API client already initialized");
        return false;
    }
    
    if (!api_host || api_port <= 0) {
        log_error("Invalid parameters for go2rtc_api_init");
        return false;
    }
    
    g_api_host = strdup(api_host);
    g_api_port = api_port;
    g_initialized = true;
    
    log_info("go2rtc API client initialized with host: %s, port: %d", g_api_host, g_api_port);
    return true;
}

// CURL response callback
struct MemoryStruct {
    char *memory;
    size_t size;
};

static size_t WriteMemoryCallback(void *contents, size_t size, size_t nmemb, void *userp) {
    size_t realsize = size * nmemb;
    struct MemoryStruct *mem = (struct MemoryStruct *)userp;
    
    char *ptr = realloc(mem->memory, mem->size + realsize + 1);
    if (!ptr) {
        log_error("Not enough memory for CURL response");
        return 0;
    }
    
    mem->memory = ptr;
    memcpy(&(mem->memory[mem->size]), contents, realsize);
    mem->size += realsize;
    mem->memory[mem->size] = 0;
    
    return realsize;
}

// Static buffer for CURL response
static char g_response_buffer[HTTP_BUFFER_SIZE];
static size_t g_response_size = 0;

// CURL write callback that uses a static buffer
static size_t StaticWriteCallback(void *contents, size_t size, size_t nmemb, void *userp) {
    size_t realsize = size * nmemb;
    
    // Check if we have enough space in the buffer
    if (g_response_size + realsize + 1 > HTTP_BUFFER_SIZE) {
        log_warn("CURL response buffer full, truncating");
        realsize = HTTP_BUFFER_SIZE - g_response_size - 1;
        if (realsize <= 0) {
            return 0; // Buffer is already full
        }
    }
    
    // Copy data to the buffer
    memcpy(g_response_buffer + g_response_size, contents, realsize);
    g_response_size += realsize;
    g_response_buffer[g_response_size] = 0;
    
    return realsize;
}

bool go2rtc_api_add_stream(const char *stream_id, const char *stream_url, const char *stream_options) {
    if (!g_initialized) {
        log_error("go2rtc API client not initialized");
        return false;
    }
    
    if (!stream_id || !stream_url) {
        log_error("Invalid parameters for go2rtc_api_add_stream");
        return false;
    }
    
    CURL *curl;
    CURLcode res;
    char url[URL_BUFFER_SIZE];
    bool success = false;
    
    // Initialize CURL
    curl = curl_easy_init();
    if (!curl) {
        log_error("Failed to initialize CURL");
        return false;
    }
    
    // Reset response buffer
    g_response_size = 0;
    g_response_buffer[0] = '\0';
    
    // Format the URL for the API endpoint with query parameters (simple method)
    // This is the method that works according to user feedback
    snprintf(url, sizeof(url), "http://%s:%d/api/streams?src=%s&name=%s", 
            g_api_host, g_api_port, stream_url, stream_id);
    
    log_info("Adding stream with URL: %s", url);
    
    // Set CURL options for PUT request
    curl_easy_setopt(curl, CURLOPT_URL, url);
    curl_easy_setopt(curl, CURLOPT_CUSTOMREQUEST, "PUT");
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, StaticWriteCallback);
    
    // Perform the request
    res = curl_easy_perform(curl);
    
    // Check for errors
    if (res != CURLE_OK) {
        log_error("CURL request failed: %s", curl_easy_strerror(res));
    } else {
        long http_code = 0;
        curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &http_code);
        
        if (http_code == 200) {
            log_info("Added stream to go2rtc: %s -> %s", stream_id, stream_url);
            log_info("Response: %s", g_response_buffer);
            success = true;
        } else {
            log_error("Failed to add stream to go2rtc (status %ld): %s", http_code, g_response_buffer);
        }
    }
    
    // Clean up
    curl_easy_cleanup(curl);
    
    return success;
}

bool go2rtc_api_remove_stream(const char *stream_id) {
    if (!g_initialized) {
        log_error("go2rtc API client not initialized");
        return false;
    }
    
    if (!stream_id) {
        log_error("Invalid parameter for go2rtc_api_remove_stream");
        return false;
    }
    
    CURL *curl;
    CURLcode res;
    char url[URL_BUFFER_SIZE];
    bool success = false;
    
    // Initialize CURL
    curl = curl_easy_init();
    if (!curl) {
        log_error("Failed to initialize CURL");
        return false;
    }
    
    // Reset response buffer
    g_response_size = 0;
    g_response_buffer[0] = '\0';
    
    // Format the URL for the API endpoint with the src parameter
    snprintf(url, sizeof(url), "http://%s:%d/api/streams?src=%s", g_api_host, g_api_port, stream_id);
    
    // Log the URL for debugging
    log_info("DELETE URL: %s", url);
    
    // Set CURL options for DELETE request
    curl_easy_setopt(curl, CURLOPT_URL, url);
    curl_easy_setopt(curl, CURLOPT_CUSTOMREQUEST, "DELETE");
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, StaticWriteCallback);
    
    // Perform the request
    res = curl_easy_perform(curl);
    
    // Check for errors
    if (res != CURLE_OK) {
        log_error("CURL request failed: %s", curl_easy_strerror(res));
    } else {
        long http_code = 0;
        curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &http_code);
        
        if (http_code == 200) {
            log_info("Removed stream from go2rtc: %s", stream_id);
            log_info("Response: %s", g_response_buffer);
            success = true;
        } else {
            log_error("Failed to remove stream from go2rtc (status %ld): %s", http_code, g_response_buffer);
            
            // Try the old method as a fallback
            log_info("Trying old method as fallback");
            
            // Reset response buffer
            g_response_size = 0;
            g_response_buffer[0] = '\0';
            
            // Format the URL for the old API endpoint
            snprintf(url, sizeof(url), "http://%s:%d/api/streams/%s", g_api_host, g_api_port, stream_id);
            
            log_info("Fallback DELETE URL: %s", url);
            
            // Set CURL options for DELETE request
            curl_easy_setopt(curl, CURLOPT_URL, url);
            
            // Perform the request
            res = curl_easy_perform(curl);
            
            // Check for errors
            if (res != CURLE_OK) {
                log_error("CURL fallback request failed: %s", curl_easy_strerror(res));
            } else {
                curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &http_code);
                
                if (http_code == 200) {
                    log_info("Removed stream from go2rtc using old method: %s", stream_id);
                    log_info("Response: %s", g_response_buffer);
                    success = true;
                } else {
                    log_error("Failed to remove stream from go2rtc using old method (status %ld): %s", 
                              http_code, g_response_buffer);
                }
            }
        }
    }
    
    // Clean up
    curl_easy_cleanup(curl);
    
    return success;
}

bool go2rtc_api_stream_exists(const char *stream_id) {
    if (!g_initialized) {
        log_error("go2rtc API client not initialized");
        return false;
    }
    
    if (!stream_id) {
        log_error("Invalid parameter for go2rtc_api_stream_exists");
        return false;
    }
    
    char path[URL_BUFFER_SIZE];
    char response[HTTP_BUFFER_SIZE];
    
    // Prepare API path
    snprintf(path, sizeof(path), "/api/streams");
    
    // Send GET request
    int status = send_http_request("GET", path, NULL, response, sizeof(response));
    
    if (status == 200) {
        char *body = extract_response_body(response);
        if (body && strstr(body, stream_id)) {
            return true;
        }
    }
    
    return false;
}

bool go2rtc_api_get_webrtc_url(const char *stream_id, char *buffer, size_t buffer_size) {
    if (!g_initialized) {
        log_error("go2rtc API client not initialized");
        return false;
    }
    
    if (!stream_id || !buffer || buffer_size == 0) {
        log_error("Invalid parameters for go2rtc_api_get_webrtc_url");
        return false;
    }
    
    // Format the WebRTC URL
    snprintf(buffer, buffer_size, "http://%s:%d/webrtc/%s", g_api_host, g_api_port, stream_id);
    return true;
}

bool go2rtc_api_update_config(void) {
    if (!g_initialized) {
        log_error("go2rtc API client not initialized");
        return false;
    }
    
    char path[URL_BUFFER_SIZE];
    char response[HTTP_BUFFER_SIZE];
    
    // Prepare API path
    snprintf(path, sizeof(path), "/api/config");
    
    // Send GET request
    int status = send_http_request("GET", path, NULL, response, sizeof(response));
    
    if (status == 200) {
        log_info("Successfully updated go2rtc configuration");
        return true;
    } else {
        char *body = extract_response_body(response);
        log_error("Failed to update go2rtc configuration (status %d): %s", status, body ? body : "unknown error");
        return false;
    }
}

/**
 * @brief Get all streams from the go2rtc API
 * 
 * @param buffer Buffer to store the response
 * @param buffer_size Size of the buffer
 * @return true if successful, false otherwise
 */
bool go2rtc_api_get_streams(char *buffer, size_t buffer_size) {
    if (!g_initialized) {
        log_error("go2rtc API client not initialized");
        return false;
    }
    
    if (!buffer || buffer_size == 0) {
        log_error("Invalid parameters for go2rtc_api_get_streams");
        return false;
    }
    
    char path[URL_BUFFER_SIZE];
    char response[HTTP_BUFFER_SIZE];
    
    // Prepare API path
    snprintf(path, sizeof(path), "/api/streams");
    
    // Send GET request
    int status = send_http_request("GET", path, NULL, response, sizeof(response));
    
    if (status == 200) {
        char *body = extract_response_body(response);
        if (body) {
            strncpy(buffer, body, buffer_size - 1);
            buffer[buffer_size - 1] = '\0';
            return true;
        }
    }
    
    return false;
}

void go2rtc_api_cleanup(void) {
    if (!g_initialized) {
        return;
    }
    
    free(g_api_host);
    g_api_host = NULL;
    g_api_port = 0;
    g_initialized = false;
    
    log_info("go2rtc API client cleaned up");
}
