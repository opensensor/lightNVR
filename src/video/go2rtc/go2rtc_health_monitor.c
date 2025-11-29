/**
 * @file go2rtc_health_monitor.c
 * @brief Health monitoring for go2rtc streams to handle camera disconnections
 * 
 * This module monitors the health of streams registered with go2rtc and
 * automatically re-registers them when cameras disconnect and reconnect.
 */

#include "video/go2rtc/go2rtc_health_monitor.h"
#include "video/go2rtc/go2rtc_stream.h"
#include "video/go2rtc/go2rtc_integration.h"
#include "video/stream_manager.h"
#include "video/stream_state.h"
#include "core/logger.h"
#include "core/shutdown_coordinator.h"

#include <pthread.h>
#include <stdbool.h>
#include <time.h>
#include <unistd.h>
#include <string.h>

// Health check interval in seconds
#define HEALTH_CHECK_INTERVAL_SEC 30

// Number of consecutive failures before attempting re-registration
#define MAX_CONSECUTIVE_FAILURES 3

// Cooldown period after re-registration (seconds)
#define REREGISTRATION_COOLDOWN_SEC 60

// Health monitor state
static pthread_t g_monitor_thread;
static bool g_monitor_running = false;
static bool g_monitor_initialized = false;

/**
 * @brief Check if a stream needs re-registration with go2rtc
 * 
 * @param stream_name Name of the stream to check
 * @return true if re-registration is needed, false otherwise
 */
static bool stream_needs_reregistration(const char *stream_name) {
    if (!stream_name) {
        return false;
    }

    // Get the stream state
    stream_state_manager_t *state = get_stream_state_by_name(stream_name);
    if (!state) {
        log_debug("Stream %s has no state manager", stream_name);
        return false;
    }

    // Check if the stream is enabled
    if (!state->config.enabled) {
        log_debug("Stream %s is disabled, skipping health check", stream_name);
        return false;
    }

    // Check if the stream is in an error or reconnecting state
    if (state->state == STREAM_STATE_ERROR || state->state == STREAM_STATE_RECONNECTING) {
        // Check consecutive failures
        int failures = atomic_load(&state->protocol_state.reconnect_attempts);
        
        if (failures >= MAX_CONSECUTIVE_FAILURES) {
            // Check if enough time has passed since last re-registration
            time_t now = time(NULL);
            time_t last_reregister = atomic_load(&state->protocol_state.last_reconnect_time);
            
            if (now - last_reregister >= REREGISTRATION_COOLDOWN_SEC) {
                log_info("Stream %s has %d consecutive failures, needs re-registration",
                        stream_name, failures);
                return true;
            } else {
                log_debug("Stream %s in cooldown period, %ld seconds remaining",
                         stream_name, REREGISTRATION_COOLDOWN_SEC - (now - last_reregister));
            }
        }
    }

    return false;
}

/**
 * @brief Attempt to re-register a stream with go2rtc
 * 
 * @param stream_name Name of the stream to re-register
 * @return true if successful, false otherwise
 */
static bool reregister_stream_with_go2rtc(const char *stream_name) {
    if (!stream_name) {
        return false;
    }

    log_info("Attempting to re-register stream %s with go2rtc", stream_name);

    // Get the stream handle
    stream_handle_t stream = get_stream_by_name(stream_name);
    if (!stream) {
        log_error("Stream %s not found", stream_name);
        return false;
    }

    // Get the stream configuration
    stream_config_t config;
    if (get_stream_config(stream, &config) != 0) {
        log_error("Failed to get config for stream %s", stream_name);
        return false;
    }

    // Unregister the stream from go2rtc
    log_info("Unregistering stream %s from go2rtc", stream_name);
    if (!go2rtc_stream_unregister(stream_name)) {
        log_warn("Failed to unregister stream %s from go2rtc (may not have been registered)", stream_name);
        // Continue anyway
    }

    // Wait a moment for go2rtc to clean up
    sleep(1);

    // Re-register the stream with go2rtc
    const char *username = (config.onvif_username[0] != '\0') ? config.onvif_username : NULL;
    const char *password = (config.onvif_password[0] != '\0') ? config.onvif_password : NULL;

    log_info("Re-registering stream %s with go2rtc using URL: %s", stream_name, config.url);
    if (!go2rtc_stream_register(stream_name, config.url, username, password, config.backchannel_enabled)) {
        log_error("Failed to re-register stream %s with go2rtc", stream_name);
        return false;
    }

    log_info("Successfully re-registered stream %s with go2rtc", stream_name);

    // Update the last reconnect time
    stream_state_manager_t *state = get_stream_state_by_name(stream_name);
    if (state) {
        atomic_store(&state->protocol_state.last_reconnect_time, time(NULL));
        atomic_store(&state->protocol_state.reconnect_attempts, 0);
    }

    return true;
}

/**
 * @brief Health monitor thread function
 * 
 * @param arg Unused
 * @return NULL
 */
static void *health_monitor_thread_func(void *arg) {
    (void)arg;

    log_info("go2rtc health monitor thread started");

    while (g_monitor_running && !is_shutdown_initiated()) {
        // Sleep for the check interval
        for (int i = 0; i < HEALTH_CHECK_INTERVAL_SEC && g_monitor_running && !is_shutdown_initiated(); i++) {
            sleep(1);
        }

        if (!g_monitor_running || is_shutdown_initiated()) {
            break;
        }

        // Check if go2rtc is ready
        if (!go2rtc_stream_is_ready()) {
            log_warn("go2rtc service is not ready, skipping health check");
            continue;
        }

        // Get all streams
        int stream_count = get_total_stream_count();
        log_debug("Health monitor checking %d streams", stream_count);

        for (int i = 0; i < stream_count; i++) {
            if (!g_monitor_running || is_shutdown_initiated()) {
                break;
            }

            stream_handle_t stream = get_stream_by_index(i);
            if (!stream) {
                continue;
            }

            stream_config_t config;
            if (get_stream_config(stream, &config) != 0) {
                continue;
            }

            // Check if this stream needs re-registration
            if (stream_needs_reregistration(config.name)) {
                log_info("Stream %s needs re-registration, attempting to fix", config.name);
                
                if (reregister_stream_with_go2rtc(config.name)) {
                    log_info("Successfully re-registered stream %s, it should recover soon", config.name);
                } else {
                    log_error("Failed to re-register stream %s", config.name);
                }
            }
        }
    }

    log_info("go2rtc health monitor thread exiting");
    return NULL;
}

bool go2rtc_health_monitor_init(void) {
    if (g_monitor_initialized) {
        log_warn("go2rtc health monitor already initialized");
        return false;
    }

    log_info("Initializing go2rtc health monitor");

    g_monitor_running = true;
    g_monitor_initialized = true;

    // Create the monitor thread
    if (pthread_create(&g_monitor_thread, NULL, health_monitor_thread_func, NULL) != 0) {
        log_error("Failed to create go2rtc health monitor thread");
        g_monitor_running = false;
        g_monitor_initialized = false;
        return false;
    }

    log_info("go2rtc health monitor initialized successfully");
    return true;
}

void go2rtc_health_monitor_cleanup(void) {
    if (!g_monitor_initialized) {
        return;
    }

    log_info("Cleaning up go2rtc health monitor");

    // Signal the thread to stop
    g_monitor_running = false;

    // Wait for the thread to exit
    if (pthread_join(g_monitor_thread, NULL) != 0) {
        log_warn("Failed to join go2rtc health monitor thread");
    }

    g_monitor_initialized = false;
    log_info("go2rtc health monitor cleaned up");
}

bool go2rtc_health_monitor_is_running(void) {
    return g_monitor_initialized && g_monitor_running;
}

