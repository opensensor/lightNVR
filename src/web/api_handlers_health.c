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

// Initialize health check system
void init_health_check_system(void) {
    g_last_health_check = time(NULL);
    g_is_healthy = true;
    g_failed_health_checks = 0;
    g_total_requests = 0;
    g_failed_requests = 0;
    g_start_time = time(NULL);
    
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
    // Check if we've received a health check request recently (within 60 seconds)
    time_t now = time(NULL);
    if (difftime(now, g_last_health_check) > 60) {
        log_warn("No health check requests received in the last 60 seconds");
        g_failed_health_checks++;
        g_is_healthy = false;
        return false;
    }
    
    // Check error rate
    double error_rate = 0.0;
    if (g_total_requests > 0) {
        error_rate = (double)g_failed_requests / (double)g_total_requests * 100.0;
    }
    
    // If error rate is too high, mark as unhealthy
    if (error_rate > 20.0 && g_total_requests > 10) {
        log_warn("Error rate too high: %.2f%%", error_rate);
        g_failed_health_checks++;
        g_is_healthy = false;
        return false;
    }
    
    // Reset failed health checks counter if everything is good
    g_failed_health_checks = 0;
    g_is_healthy = true;
    return true;
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
