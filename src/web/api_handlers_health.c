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
#include "core/logger.h"
#include "core/config.h"
#include "mongoose.h"

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
static char g_health_check_url[256] = "http://localhost:8080/api/health";

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
 * @brief Health check thread function
 */
static void *health_check_thread_func(void *arg) {
    log_info("Health check thread started");
    
    while (g_health_thread_running) {
        // Check if shutdown has been initiated, if shutdown coordinator is available
        if (get_shutdown_coordinator() && is_shutdown_initiated()) {
            log_info("Shutdown initiated, stopping health check thread");
            break;
        }
        
        // Perform health check
        bool check_result = perform_health_check();
        log_debug("Health check result: %s", check_result ? "success" : "failure");
        
        // Sleep for the configured interval
        for (int i = 0; i < g_health_check_interval && g_health_thread_running; i++) {
            sleep(1);
            if (get_shutdown_coordinator() && is_shutdown_initiated()) {
                break;
            }
        }
    }
    
    log_info("Health check thread stopped");
    return NULL;
}

/**
 * @brief Start the health check thread
 */
void start_health_check_thread(void) {
    if (g_health_thread_running) {
        log_warn("Health check thread already running");
        return;
    }
    
    // Initialize curl globally before starting the thread
    int interval = 30;
    curl_global_init(CURL_GLOBAL_ALL);
    
    g_health_thread_running = true;
    
    // Create health check thread
    if (pthread_create(&g_health_check_thread, NULL, health_check_thread_func, NULL) != 0) {
        log_error("Failed to create health check thread");
        g_health_thread_running = false;
        curl_global_cleanup();
        return;
    }
    
    log_info("Health check thread started with interval %d seconds, URL: %s", 
             g_health_check_interval, g_health_check_url);
}

/**
 * @brief Stop the health check thread
 */
void stop_health_check_thread(void) {
    if (!g_health_thread_running) {
        return;
    }
    
    g_health_thread_running = false;
    
    // Wait for thread to finish
    pthread_join(g_health_check_thread, NULL);
    
    // Clean up curl
    curl_global_cleanup();
    
    log_info("Health check thread stopped");
}

// Initialize health check system
void init_health_check_system(void) {
    g_last_health_check = time(NULL);
    g_is_healthy = true;
    g_failed_health_checks = 0;
    g_total_requests = 0;
    g_failed_requests = 0;
    g_start_time = time(NULL);
    
    // Start the health check thread
    start_health_check_thread();
    
    log_info("Health check system initialized");
}

// Update health check metrics for a request
void update_health_metrics(bool request_succeeded) {
    g_total_requests++;
    if (!request_succeeded) {
        g_failed_requests++;
    }
}

/**
 * @brief Direct handler for GET /api/health
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void    mg_handle_get_health(struct mg_connection *c, struct mg_http_message *hm) {
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
    cJSON_AddBoolToObject(health, "healthy", g_is_healthy);
    cJSON_AddStringToObject(health, "status", g_is_healthy ? "ok" : "degraded");
    
    // Add metrics
    cJSON_AddNumberToObject(health, "uptime", difftime(time(NULL), g_start_time));
    cJSON_AddNumberToObject(health, "totalRequests", g_total_requests);
    cJSON_AddNumberToObject(health, "failedRequests", g_failed_requests);
    
    // Add error rate
    double error_rate = 0.0;
    if (g_total_requests > 0) {
        error_rate = (double)g_failed_requests / (double)g_total_requests * 100.0;
    }
    cJSON_AddNumberToObject(health, "errorRate", error_rate);
    
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
 * @brief Check if the web server is healthy
 * 
 * @return true if healthy, false otherwise
 */
bool is_web_server_healthy(void) {
    // Check if we've received a health check request recently
    time_t now = time(NULL);
    if (difftime(now, g_last_health_check) > 120) {
        log_warn("No health check requests received in the last 120 seconds");
        g_failed_health_checks++;
        g_is_healthy = false;
        
        // Mark for restart after consecutive failures
        if (g_failed_health_checks >= 3) {
            mark_server_for_restart();
        }
        
        return false;
    }
    
    // Check error rate
    double error_rate = 0.0;
    if (g_total_requests > 0) {
        error_rate = (double)g_failed_requests / (double)g_total_requests * 100.0;
    }
    
    // If error rate is too high, mark as unhealthy
    if (error_rate > 20.0 && g_total_requests > 10) {
        log_warn("High error rate: %.2f%% (%d/%d requests failed)", 
                error_rate, g_failed_requests, g_total_requests);
        g_is_healthy = false;
        return false;
    }
    
    return g_is_healthy;
}

/**
 * @brief Get the number of consecutive failed health checks
 * 
 * @return int Number of consecutive failed health checks
 */
int get_failed_health_checks(void) {
    return g_failed_health_checks;
}

/**
 * @brief Reset health check metrics
 */
void reset_health_metrics(void) {
    g_last_health_check = time(NULL);
    g_is_healthy = true;
    g_failed_health_checks = 0;
    g_total_requests = 0;
    g_failed_requests = 0;
    
    log_info("Health check metrics reset");
}

/**
 * @brief Check if the server needs to be restarted
 * 
 * @return true if server needs restart, false otherwise
 */
bool check_server_restart_needed(void) {
    if (!g_server_needs_restart) {
        return false;
    }
    
    // Check if we've exceeded the maximum number of restart attempts
    if (g_restart_attempts >= MAX_RESTART_ATTEMPTS) {
        log_error("Maximum restart attempts (%d) reached, not attempting further restarts", 
                 MAX_RESTART_ATTEMPTS);
        return false;
    }
    
    // Check if we're in the cooldown period
    time_t now = time(NULL);
    if (difftime(now, g_last_restart_attempt) < RESTART_COOLDOWN_SECONDS) {
        log_debug("In restart cooldown period, not attempting restart yet");
        return false;
    }
    
    return true;
}

/**
 * @brief Mark the server as needing restart
 */
void mark_server_for_restart(void) {
    g_server_needs_restart = true;
    log_warn("Server marked for restart due to health check failure");
}

/**
 * @brief Reset the restart flag after successful restart
 */
void reset_server_restart_flag(void) {
    g_server_needs_restart = false;
    g_restart_attempts++;
    g_last_restart_attempt = time(NULL);
    log_info("Server restart flag reset, attempt %d of %d", 
             g_restart_attempts, MAX_RESTART_ATTEMPTS);
}

/**
 * @brief Cleanup health check system during shutdown
 */
void cleanup_health_check_system(void) {
    // Stop the health check thread
    stop_health_check_thread();
    
    log_info("Health check system cleaned up");
}
