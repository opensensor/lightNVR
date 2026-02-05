/**
 * @file api_handlers_health.c
 * @brief Health check API handlers for the web server
 */

// Note: _GNU_SOURCE not needed - we use portable polling instead of pthread_timedjoin_np
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
#include <signal.h>
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
static volatile bool g_health_thread_running = false;
static volatile bool g_health_thread_exited = false;  // Flag to indicate thread has exited
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

/**
 * @brief Direct handler for GET /api/health/hls
 *
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_get_hls_health(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling GET /api/health/hls request");

    // Get the HLS watchdog restart count
    extern int get_hls_watchdog_restart_count(void);
    int restart_count = get_hls_watchdog_restart_count();

    // Create JSON object
    cJSON *health = cJSON_CreateObject();
    if (!health) {
        log_error("Failed to create HLS health JSON object");
        mg_send_json_error(c, 500, "Failed to create HLS health JSON");
        return;
    }

    // Add HLS health status
    cJSON_AddBoolToObject(health, "healthy", true);
    cJSON_AddStringToObject(health, "status", "ok");
    cJSON_AddNumberToObject(health, "watchdogRestartCount", restart_count);

    // Add timestamp
    char timestamp[32];
    time_t now = time(NULL);
    struct tm *tm_info = localtime(&now);
    strftime(timestamp, sizeof(timestamp), "%Y-%m-%d %H:%M:%S", tm_info);
    cJSON_AddStringToObject(health, "timestamp", timestamp);

    // Convert to string
    char *json_str = cJSON_PrintUnformatted(health);
    if (!json_str) {
        log_error("Failed to convert HLS health JSON to string");
        cJSON_Delete(health);
        mg_send_json_error(c, 500, "Failed to convert HLS health JSON to string");
        return;
    }

    // Send response
    mg_send_json_response(c, 200, json_str);

    // Clean up
    free(json_str);
    cJSON_Delete(health);

    log_info("Successfully handled GET /api/health/hls request");
}

/**
 * @brief Check if the web server thread is still running
 *
 * @return true if running, false otherwise
 */
static bool is_web_server_thread_running(void) {
    pthread_mutex_lock(&g_web_server_thread_mutex);

    if (!g_web_server_thread_id_set) {
        pthread_mutex_unlock(&g_web_server_thread_mutex);
        return false;
    }

    // Use pthread_kill with signal 0 to check if thread exists
    int result = pthread_kill(g_web_server_thread_id, 0);

    pthread_mutex_unlock(&g_web_server_thread_mutex);

    // ESRCH means thread doesn't exist, 0 means it exists
    return (result == 0);
}

/**
 * @brief Restart the web server
 *
 * This function attempts to restart the web server by stopping and starting it again.
 * This is necessary when the server thread is deadlocked (alive but not responding).
 *
 * @return true if restart succeeded, false otherwise
 */
static bool restart_web_server(void) {
    // Get the global http_server from main.c
    extern http_server_handle_t http_server;
    extern void http_server_stop(http_server_handle_t server);

    if (!http_server) {
        log_error("Cannot restart web server: no server handle available");
        return false;
    }

    // Check cooldown period
    time_t now = time(NULL);
    if (now - g_last_restart_attempt < RESTART_COOLDOWN_SECONDS) {
        log_warn("Web server restart attempt too soon, waiting for cooldown");
        return false;
    }

    // Check max restart attempts
    if (g_restart_attempts >= MAX_RESTART_ATTEMPTS) {
        log_error("Maximum web server restart attempts (%d) reached", MAX_RESTART_ATTEMPTS);
        return false;
    }

    g_last_restart_attempt = now;
    g_restart_attempts++;

    log_info("Attempting to restart web server (attempt %d/%d)",
             g_restart_attempts, MAX_RESTART_ATTEMPTS);

    // CRITICAL FIX: First stop the server to signal the event loop to exit
    // This is necessary because http_server_start() returns early if server->running is true
    log_info("Stopping web server before restart...");
    http_server_stop(http_server);

    // Wait for the event loop thread to exit
    // The thread is detached, so we can't join it, but we can wait a bit
    log_info("Waiting for web server thread to exit...");
    for (int i = 0; i < 30; i++) {  // Wait up to 3 seconds
        usleep(100000);  // 100ms

        // Check if thread is still running
        if (!is_web_server_thread_running()) {
            log_info("Web server thread has exited");
            break;
        }

        if (i == 29) {
            log_warn("Web server thread did not exit after 3 seconds, proceeding anyway");
        }
    }

    // Small delay to ensure resources are released
    usleep(500000);  // 500ms

    // Now start the server again - this will create a new event loop thread
    if (http_server_start(http_server) != 0) {
        log_error("Failed to restart web server");
        return false;
    }

    log_info("Web server restarted successfully");
    g_restart_attempts = 0; // Reset on success
    g_is_healthy = true;
    g_failed_health_checks = 0;

    return true;
}

/**
 * @brief Health check thread function
 *
 * This thread periodically checks if the web server is healthy and attempts
 * to restart it if necessary.
 */
static void *health_check_thread_func(void *arg) {
    (void)arg;

    log_info("Health check thread started");

    while (g_health_thread_running) {
        // Don't check during shutdown
        if (is_shutdown_initiated()) {
            log_info("Shutdown initiated, stopping health check thread");
            break;
        }

        // Sleep for the health check interval
        for (int i = 0; i < g_health_check_interval && g_health_thread_running; i++) {
            sleep(1);
            if (is_shutdown_initiated()) break;
        }

        if (!g_health_thread_running || is_shutdown_initiated()) {
            break;
        }

        // Perform health check
        bool check_result = perform_health_check();

        if (!check_result) {
            log_warn("Health check failed (consecutive failures: %d)", g_failed_health_checks);

            // Check if web server thread is still running
            if (!is_web_server_thread_running()) {
                log_error("Web server thread is not running!");
                g_server_needs_restart = true;
            }

            // If we've had multiple consecutive failures, try to restart
            if (g_failed_health_checks >= 3 || g_server_needs_restart) {
                log_warn("Multiple health check failures or server thread dead, attempting restart");

                if (restart_web_server()) {
                    log_info("Web server restart succeeded");
                    g_server_needs_restart = false;
                } else {
                    log_error("Web server restart failed");
                }
            }
        } else {
            // Reset restart attempts counter on successful health check
            if (g_restart_attempts > 0) {
                log_info("Health check succeeded after restart, resetting attempt counter");
                g_restart_attempts = 0;
            }
        }
    }

    log_info("Health check thread exiting");
    g_health_thread_exited = true;  // Signal that thread has exited
    return NULL;
}

/**
 * @brief Initialize health check system
 */
void init_health_check_system(void) {
    g_start_time = time(NULL);
    g_last_health_check = g_start_time;
    g_is_healthy = true;
    g_failed_health_checks = 0;
    g_total_requests = 0;
    g_failed_requests = 0;
    g_server_needs_restart = false;
    g_restart_attempts = 0;
    g_last_restart_attempt = 0;

    // Get the web port from config
    extern config_t config;
    snprintf(g_health_check_url, sizeof(g_health_check_url),
             "http://127.0.0.1:%d/api/health", config.web_port);

    log_info("Health check system initialized with URL: %s", g_health_check_url);
}

/**
 * @brief Start health check thread
 */
void start_health_check_thread(void) {
    if (g_health_thread_running) {
        log_warn("Health check thread already running");
        return;
    }

    g_health_thread_running = true;
    g_health_thread_exited = false;  // Reset the exited flag

    if (pthread_create(&g_health_check_thread, NULL, health_check_thread_func, NULL) != 0) {
        log_error("Failed to create health check thread");
        g_health_thread_running = false;
        return;
    }

    log_info("Health check thread started with %d second interval", g_health_check_interval);
}

/**
 * @brief Stop health check thread
 *
 * Uses a portable polling approach instead of pthread_timedjoin_np (GNU extension)
 * to work on both glibc and musl (e.g., Linux 4.4 with thingino-firmware)
 */
void stop_health_check_thread(void) {
    if (!g_health_thread_running && g_health_thread_exited) {
        // Thread was never started or already stopped
        return;
    }

    log_info("Stopping health check thread...");
    g_health_thread_running = false;

    // Use portable polling approach with timeout (5 seconds)
    // Poll every 50ms to check if thread has exited
    int timeout_ms = 5000;
    int elapsed_ms = 0;
    const int poll_interval_ms = 50;

    while (elapsed_ms < timeout_ms) {
        if (g_health_thread_exited) {
            // Thread has exited, we can join it
            pthread_join(g_health_check_thread, NULL);
            log_info("Health check thread stopped successfully");
            return;
        }
        usleep(poll_interval_ms * 1000);
        elapsed_ms += poll_interval_ms;
    }

    // Thread did not exit in time, detach it
    log_warn("Health check thread did not exit in time (%d ms), detaching", timeout_ms);
    pthread_detach(g_health_check_thread);
}

/**
 * @brief Cleanup health check system during shutdown
 */
void cleanup_health_check_system(void) {
    stop_health_check_thread();

    // Reset all state
    g_is_healthy = false;
    g_failed_health_checks = 0;
    g_server_needs_restart = false;

    log_info("Health check system cleaned up");
}

/**
 * @brief Update health check metrics for a request
 */
void update_health_metrics(bool request_succeeded) {
    g_total_requests++;
    if (!request_succeeded) {
        g_failed_requests++;
    }
}

/**
 * @brief Check if the web server is healthy
 */
bool is_web_server_healthy(void) {
    return g_is_healthy && is_web_server_thread_running();
}

/**
 * @brief Get the number of consecutive failed health checks
 */
int get_failed_health_checks(void) {
    return g_failed_health_checks;
}

/**
 * @brief Reset health check metrics
 */
void reset_health_metrics(void) {
    g_total_requests = 0;
    g_failed_requests = 0;
    g_failed_health_checks = 0;
    g_is_healthy = true;
}

/**
 * @brief Check if the server needs to be restarted
 */
bool check_server_restart_needed(void) {
    return g_server_needs_restart;
}

/**
 * @brief Mark the server as needing restart
 */
void mark_server_for_restart(void) {
    g_server_needs_restart = true;
    log_warn("Web server marked for restart");
}

/**
 * @brief Reset the restart flag after successful restart
 */
void reset_server_restart_flag(void) {
    g_server_needs_restart = false;
    g_restart_attempts = 0;
}
