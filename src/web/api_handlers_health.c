/**
 * @file api_handlers_health.c
 * @brief Health check API handlers for the web server
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <fcntl.h>
#include <errno.h>
#include <stdbool.h>
#include <pthread.h>
#include <curl/curl.h>
#include "core/shutdown_coordinator.h"

#include "web/api_handlers.h"
#include "web/mongoose_adapter.h"
#include "web/http_server.h"
#include "core/logger.h"
#include "core/config.h"
#include "mongoose.h"

// Forward declarations for functions defined later in this file
static bool is_web_server_thread_running(void);
static bool restart_web_server(void);

// Global variables for health check
static time_t g_last_health_check = 0;
static bool g_is_healthy = true;
static int g_failed_health_checks = 0;
static int g_total_requests = 0;
static int g_failed_requests = 0;
static time_t g_start_time = 0;

// Add these variables to track server restart
static bool g_server_needs_restart = false;
static time_t g_last_restart_attempt = 0;
static int g_restart_attempts = 0;
static const int MAX_RESTART_ATTEMPTS = 5;
static const int RESTART_COOLDOWN_SECONDS = 60;

// Health check thread variables
static pthread_t g_health_check_thread;
static bool g_health_thread_running = false;
static int g_health_check_interval = 30; // seconds
static char g_health_check_url[256] = "http://127.0.0.1:8080/api/health";

// Web server thread tracking
static pthread_t g_web_server_thread_id = 0;
static bool g_web_server_thread_id_set = false;
static pthread_mutex_t g_web_server_thread_mutex = PTHREAD_MUTEX_INITIALIZER;

// Curl response buffer
struct MemoryStruct {
    char *memory;
    size_t size;
};

/**
 * @brief Callback function for curl to write received data
 */
static size_t write_memory_callback(void *contents, size_t size, size_t nmemb, void *userp) {
    size_t realsize = size * nmemb;
    struct MemoryStruct *mem = (struct MemoryStruct *)userp;

    char *ptr = realloc(mem->memory, mem->size + realsize + 1);
    if (!ptr) {
        log_error("Failed to allocate memory for curl response");
        return 0;
    }

    mem->memory = ptr;
    memcpy(&(mem->memory[mem->size]), contents, realsize);
    mem->size += realsize;
    mem->memory[mem->size] = 0;

    return realsize;
}

/**
 * @brief Perform a health check using curl
 *
 * @return true if health check succeeded, false otherwise
 */
static bool perform_health_check(void) {
    CURL *curl;
    CURLcode res;
    bool success = false;
    struct MemoryStruct chunk;

    chunk.memory = malloc(1);
    chunk.size = 0;

    curl = curl_easy_init();
    if (!curl) {
        log_error("Failed to initialize curl for health check");
        free(chunk.memory);
        return false;
    }

    // Set curl options
    curl_easy_setopt(curl, CURLOPT_URL, g_health_check_url);
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, write_memory_callback);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, (void *)&chunk);
    curl_easy_setopt(curl, CURLOPT_TIMEOUT, 5L); // 5 second timeout
    curl_easy_setopt(curl, CURLOPT_CONNECTTIMEOUT, 3L); // 3 second connect timeout
    curl_easy_setopt(curl, CURLOPT_NOSIGNAL, 1L); // Prevent curl from using signals

    // Perform the request
    res = curl_easy_perform(curl);

    // Check for errors
    if (res != CURLE_OK) {
        log_error("Health check curl failed: %s", curl_easy_strerror(res));
        g_failed_health_checks++;
        g_is_healthy = false;
    } else {
        long response_code;
        curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &response_code);

        if (response_code == 200) {
            log_debug("Health check succeeded with response: %s", chunk.memory);
            success = true;

            // Update last health check time
            g_last_health_check = time(NULL);
            g_is_healthy = true;
            g_failed_health_checks = 0;
        } else {
            log_warn("Health check failed with response code: %ld", response_code);
            g_failed_health_checks++;
            g_is_healthy = false;
        }
    }

    curl_easy_cleanup(curl);
    free(chunk.memory);

    return success;
}


/**
 * @brief Direct handler for GET /api/health
 *
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_get_health(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling GET /api/health request");

    // Update last health check time
    g_last_health_check = time(NULL);

    // Create JSON object
    cJSON *health = cJSON_CreateObject();
    if (!health) {
        log_error("Failed to create health JSON object");
        mg_send_json_error(c, 500, "Failed to create health JSON");
        return;
    }

    // Add health status
    cJSON_AddBoolToObject(health, "healthy", true);
    cJSON_AddStringToObject(health, "status", "ok");

    // Add metrics
    cJSON_AddNumberToObject(health, "uptime", difftime(time(NULL), g_start_time));
    cJSON_AddNumberToObject(health, "totalRequests", g_total_requests);
    cJSON_AddNumberToObject(health, "failedRequests", g_failed_requests);

    // Add timestamp
    char timestamp[32];
    time_t now = time(NULL);
    struct tm *tm_info = localtime(&now);
    strftime(timestamp, sizeof(timestamp), "%Y-%m-%d %H:%M:%S", tm_info);
    cJSON_AddStringToObject(health, "timestamp", timestamp);

    // Convert to string
    char *json_str = cJSON_PrintUnformatted(health);
    if (!json_str) {
        log_error("Failed to convert health JSON to string");
        cJSON_Delete(health);
        mg_send_json_error(c, 500, "Failed to convert health JSON to string");
        return;
    }

    // Send response
    mg_send_json_response(c, 200, json_str);

    // Clean up
    free(json_str);
    cJSON_Delete(health);

    log_info("Successfully handled GET /api/health request");
}

/**
 * @brief Set the web server thread ID
 *
 * @param thread_id The thread ID of the web server thread
 */
void set_web_server_thread_id(pthread_t thread_id) {
    pthread_mutex_lock(&g_web_server_thread_mutex);
    g_web_server_thread_id = thread_id;
    g_web_server_thread_id_set = true;
    pthread_mutex_unlock(&g_web_server_thread_mutex);

    log_info("Web server thread ID set: %lu", (unsigned long)thread_id);
}
