/**
 * @file go2rtc_process_monitor.c
 * @brief Process health monitoring for go2rtc service with automatic restart
 * 
 * This module monitors the go2rtc process itself and automatically restarts it
 * when it becomes unresponsive (zombie state). It uses multiple health indicators:
 * - API health check on port 1984
 * - Stream consensus (if all streams are down, it's likely a go2rtc issue)
 */

#include "video/go2rtc/go2rtc_process_monitor.h"
#include "video/go2rtc/go2rtc_stream.h"
#include "video/go2rtc/go2rtc_api.h"
#include "video/go2rtc/go2rtc_process.h"
#include "video/go2rtc/go2rtc_integration.h"
#include "video/stream_manager.h"
#include "video/stream_state.h"
#include "video/mp4_recording.h"
#include "core/logger.h"
#include "core/shutdown_coordinator.h"

#include <pthread.h>
#include <stdbool.h>
#include <time.h>
#include <unistd.h>
#include <string.h>
#include <curl/curl.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <errno.h>

// Health check interval in seconds
#define PROCESS_HEALTH_CHECK_INTERVAL_SEC 30

// Number of consecutive API failures before considering go2rtc unhealthy
#define MAX_API_FAILURES 3

// Minimum number of streams that must be down to trigger consensus check
#define MIN_STREAMS_FOR_CONSENSUS 2

// Cooldown period after restart (seconds) - prevent restart loops
#define RESTART_COOLDOWN_SEC 120

// Maximum number of restarts within a time window
#define MAX_RESTARTS_PER_WINDOW 5
#define RESTART_WINDOW_SEC 600  // 10 minutes

// Process monitor state
static pthread_t g_monitor_thread;
static bool g_monitor_running = false;
static bool g_monitor_initialized = false;
static int g_restart_count = 0;
static time_t g_last_restart_time = 0;
static int g_consecutive_api_failures = 0;

// Restart history for rate limiting
static time_t g_restart_history[MAX_RESTARTS_PER_WINDOW];
static int g_restart_history_index = 0;

/**
 * @brief Check if go2rtc API is responsive
 * 
 * @return true if API is responsive, false otherwise
 */
static bool check_go2rtc_api_health(void) {
    // Use the existing go2rtc_stream_is_ready function which checks API health
    return go2rtc_stream_is_ready();
}

/**
 * @brief Check stream consensus - if all/most streams are down, it's likely go2rtc
 * 
 * @return true if consensus indicates go2rtc problem, false otherwise
 */
static bool check_stream_consensus(void) {
    int total_streams = 0;
    int failed_streams = 0;
    
    // Get all streams
    int stream_count = get_total_stream_count();
    
    if (stream_count < MIN_STREAMS_FOR_CONSENSUS) {
        // Not enough streams to make a consensus decision
        return false;
    }
    
    for (int i = 0; i < stream_count; i++) {
        stream_handle_t stream = get_stream_by_index(i);
        if (!stream) {
            continue;
        }
        
        stream_config_t config;
        if (get_stream_config(stream, &config) != 0) {
            continue;
        }
        
        // Only count enabled streams
        if (!config.enabled) {
            continue;
        }
        
        total_streams++;
        
        // Check stream state
        stream_state_manager_t *state = get_stream_state_by_name(config.name);
        if (state) {
            pthread_mutex_lock(&state->mutex);
            stream_state_t current_state = state->state;
            pthread_mutex_unlock(&state->mutex);
            
            if (current_state == STREAM_STATE_ERROR || 
                current_state == STREAM_STATE_RECONNECTING) {
                failed_streams++;
            }
        }
    }
    
    // If we have at least MIN_STREAMS_FOR_CONSENSUS enabled streams
    // and ALL of them are failed, it's likely a go2rtc issue
    if (total_streams >= MIN_STREAMS_FOR_CONSENSUS && 
        failed_streams == total_streams && 
        total_streams > 0) {
        log_warn("Stream consensus: %d/%d streams failed - indicates go2rtc issue",
                 failed_streams, total_streams);
        return true;
    }
    
    return false;
}

/**
 * @brief Check if we can restart (rate limiting)
 * 
 * @return true if restart is allowed, false if rate limited
 */
static bool can_restart_go2rtc(void) {
    time_t now = time(NULL);
    
    // Check cooldown period
    if (now - g_last_restart_time < RESTART_COOLDOWN_SEC) {
        log_warn("go2rtc restart blocked: cooldown period (%ld seconds remaining)",
                 RESTART_COOLDOWN_SEC - (now - g_last_restart_time));
        return false;
    }
    
    // Check restart rate limiting
    int recent_restarts = 0;
    for (int i = 0; i < MAX_RESTARTS_PER_WINDOW; i++) {
        if (g_restart_history[i] > 0 && (now - g_restart_history[i]) < RESTART_WINDOW_SEC) {
            recent_restarts++;
        }
    }

    if (recent_restarts >= MAX_RESTARTS_PER_WINDOW) {
        log_error("go2rtc restart blocked: too many restarts (%d in last %d seconds)",
                  recent_restarts, RESTART_WINDOW_SEC);
        return false;
    }

    return true;
}

/**
 * @brief Restart the go2rtc process
 *
 * @return true if restart was successful, false otherwise
 */
static bool restart_go2rtc_process(void) {
    log_warn("Attempting to restart go2rtc process due to health check failure");

    // Stop the current go2rtc process
    log_info("Stopping go2rtc process...");
    if (!go2rtc_process_stop()) {
        log_error("Failed to stop go2rtc process");
        return false;
    }

    // Wait a moment for cleanup
    sleep(2);

    // Get the API port from the existing configuration
    int api_port = go2rtc_stream_get_api_port();
    if (api_port == 0) {
        log_error("Failed to get go2rtc API port");
        return false;
    }

    // Start go2rtc process again
    log_info("Starting go2rtc process...");
    if (!go2rtc_process_start(api_port)) {
        log_error("Failed to start go2rtc process");
        return false;
    }

    // Wait for go2rtc to be ready
    int retries = 10;
    while (retries > 0 && !go2rtc_stream_is_ready()) {
        log_info("Waiting for go2rtc to be ready after restart... (%d retries left)", retries);
        sleep(2);
        retries--;
    }

    if (!go2rtc_stream_is_ready()) {
        log_error("go2rtc failed to become ready after restart");
        return false;
    }

    log_info("go2rtc process restarted successfully");

    // Re-register all streams with go2rtc
    log_info("Re-registering all streams with go2rtc after restart");
    if (!go2rtc_integration_register_all_streams()) {
        log_warn("Failed to re-register all streams after go2rtc restart");
        // Continue anyway - streams will be re-registered by the health monitor
    } else {
        log_info("All streams re-registered successfully after go2rtc restart");
    }

    // Wait a moment for streams to be fully available in go2rtc
    sleep(2);

    // Signal all active MP4 recordings to reconnect
    // This is necessary because recordings connect to go2rtc's RTSP output,
    // which becomes unavailable during restart
    log_info("Signaling MP4 recordings to reconnect after go2rtc restart");
    signal_all_mp4_recordings_reconnect();

    // Update restart tracking
    time_t now = time(NULL);
    g_last_restart_time = now;
    g_restart_count++;
    g_restart_history[g_restart_history_index] = now;
    g_restart_history_index = (g_restart_history_index + 1) % MAX_RESTARTS_PER_WINDOW;
    g_consecutive_api_failures = 0;  // Reset failure counter

    log_info("go2rtc restart completed (total restarts: %d)", g_restart_count);

    return true;
}

/**
 * @brief Process health monitor thread function
 */
static void *process_monitor_thread_func(void *arg) {
    (void)arg;

    log_info("go2rtc process health monitor thread started");

    while (g_monitor_running && !is_shutdown_initiated()) {
        // Sleep for the check interval
        for (int i = 0; i < PROCESS_HEALTH_CHECK_INTERVAL_SEC && g_monitor_running && !is_shutdown_initiated(); i++) {
            sleep(1);
        }

        if (!g_monitor_running || is_shutdown_initiated()) {
            break;
        }

        // Check API health
        bool api_healthy = check_go2rtc_api_health();

        if (!api_healthy) {
            g_consecutive_api_failures++;
            log_warn("go2rtc API health check failed (consecutive failures: %d/%d)",
                     g_consecutive_api_failures, MAX_API_FAILURES);

            // Check if we've hit the failure threshold
            if (g_consecutive_api_failures >= MAX_API_FAILURES) {
                log_error("go2rtc API has failed %d consecutive health checks",
                         g_consecutive_api_failures);

                // Also check stream consensus for additional confirmation
                bool consensus_failure = check_stream_consensus();

                if (consensus_failure) {
                    log_error("Stream consensus also indicates go2rtc failure - restart required");
                } else {
                    log_warn("Stream consensus does not indicate widespread failure, but API is unresponsive");
                }

                // Attempt restart if allowed
                if (can_restart_go2rtc()) {
                    if (restart_go2rtc_process()) {
                        log_info("go2rtc process successfully restarted");
                    } else {
                        log_error("Failed to restart go2rtc process");
                    }
                }
            }
        } else {
            // API is healthy, reset failure counter
            if (g_consecutive_api_failures > 0) {
                log_info("go2rtc API health check succeeded, resetting failure counter");
                g_consecutive_api_failures = 0;
            }
        }
    }

    log_info("go2rtc process health monitor thread exiting");
    return NULL;
}

bool go2rtc_process_monitor_init(void) {
    if (g_monitor_initialized) {
        log_warn("go2rtc process monitor already initialized");
        return false;
    }

    log_info("Initializing go2rtc process health monitor");

    // Initialize restart history
    memset(g_restart_history, 0, sizeof(g_restart_history));
    g_restart_history_index = 0;
    g_restart_count = 0;
    g_last_restart_time = 0;
    g_consecutive_api_failures = 0;

    g_monitor_running = true;
    g_monitor_initialized = true;

    // Create the monitor thread
    if (pthread_create(&g_monitor_thread, NULL, process_monitor_thread_func, NULL) != 0) {
        log_error("Failed to create go2rtc process monitor thread");
        g_monitor_running = false;
        g_monitor_initialized = false;
        return false;
    }

    log_info("go2rtc process health monitor initialized successfully");
    return true;
}

void go2rtc_process_monitor_cleanup(void) {
    if (!g_monitor_initialized) {
        return;
    }

    log_info("Cleaning up go2rtc process health monitor");

    // Signal the thread to stop
    g_monitor_running = false;

    // Wait for the thread to exit
    if (pthread_join(g_monitor_thread, NULL) != 0) {
        log_warn("Failed to join go2rtc process monitor thread");
    }

    g_monitor_initialized = false;
    log_info("go2rtc process health monitor cleaned up");
}

bool go2rtc_process_monitor_is_running(void) {
    return g_monitor_initialized && g_monitor_running;
}

int go2rtc_process_monitor_get_restart_count(void) {
    return g_restart_count;
}

time_t go2rtc_process_monitor_get_last_restart_time(void) {
    return g_last_restart_time;
}

bool go2rtc_process_monitor_check_health(void) {
    return check_go2rtc_api_health();
}

