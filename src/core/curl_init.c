/**
 * @file curl_init.c
 * @brief Centralized libcurl initialization
 * 
 * This module provides thread-safe, centralized initialization of libcurl.
 * According to libcurl documentation, curl_global_init() is NOT thread-safe
 * and MUST only be called once at program startup.
 */

#include "core/curl_init.h"
#include "core/logger.h"
#include <curl/curl.h>
#include <pthread.h>
#include <stdbool.h>

// Thread-safe initialization using pthread_once
static pthread_once_t curl_init_once = PTHREAD_ONCE_INIT;
static bool curl_initialized = false;
static int curl_init_result = -1;

/**
 * @brief Internal initialization function called by pthread_once
 */
static void curl_do_global_init(void) {
    log_info("Initializing libcurl globally...");
    
    CURLcode result = curl_global_init(CURL_GLOBAL_ALL);
    if (result != CURLE_OK) {
        log_error("Failed to initialize libcurl globally: %s", curl_easy_strerror(result));
        curl_init_result = -1;
        curl_initialized = false;
        return;
    }
    
    curl_init_result = 0;
    curl_initialized = true;
    log_info("libcurl initialized globally successfully");
}

/**
 * @brief Initialize libcurl globally (thread-safe, idempotent)
 */
int curl_init_global(void) {
    pthread_once(&curl_init_once, curl_do_global_init);
    return curl_init_result;
}

/**
 * @brief Check if libcurl has been globally initialized
 */
bool curl_is_initialized(void) {
    return curl_initialized;
}

/**
 * @brief Cleanup libcurl globally
 */
void curl_cleanup_global(void) {
    if (curl_initialized) {
        log_info("Cleaning up libcurl globally...");
        curl_global_cleanup();
        curl_initialized = false;
        // Reset pthread_once so it can be initialized again if needed
        // Note: This is technically not portable, but works on Linux
        curl_init_once = PTHREAD_ONCE_INIT;
        curl_init_result = -1;
        log_info("libcurl cleaned up globally");
    }
}

