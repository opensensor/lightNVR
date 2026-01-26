/**
 * @file curl_init.h
 * @brief Centralized libcurl initialization
 * 
 * This module provides thread-safe, centralized initialization of libcurl.
 * According to libcurl documentation, curl_global_init() is NOT thread-safe
 * and MUST only be called once at program startup. Multiple calls from
 * different threads can corrupt libcurl's internal state and cause heap
 * corruption.
 * 
 * All modules that use libcurl should call curl_init_global() before using
 * any curl functions. This function is safe to call multiple times - it will
 * only initialize once.
 */

#ifndef CURL_INIT_H
#define CURL_INIT_H

#include <stdbool.h>

/**
 * @brief Initialize libcurl globally (thread-safe, idempotent)
 * 
 * This function initializes libcurl using curl_global_init(CURL_GLOBAL_ALL).
 * It is safe to call from multiple threads - the initialization will only
 * happen once using pthread_once.
 * 
 * @return 0 on success, -1 on failure
 */
int curl_init_global(void);

/**
 * @brief Check if libcurl has been globally initialized
 * 
 * @return true if initialized, false otherwise
 */
bool curl_is_initialized(void);

/**
 * @brief Cleanup libcurl globally
 * 
 * This function should only be called once at program shutdown.
 * After calling this, curl_init_global() must be called again before
 * using any curl functions.
 */
void curl_cleanup_global(void);

#endif /* CURL_INIT_H */

