/**
 * @file go2rtc_integration.c
 * @brief Implementation of the go2rtc integration with existing recording and HLS systems
 *
 * This module also contains the unified health monitor that handles both:
 * - Stream-level health (re-registering individual streams when they fail)
 * - Process-level health (restarting go2rtc when it becomes unresponsive)
 */

#include "video/go2rtc/go2rtc_integration.h"
#include "video/go2rtc/go2rtc_consumer.h"
#include "video/go2rtc/go2rtc_stream.h"
#include "video/go2rtc/go2rtc_process.h"
#include "video/go2rtc/go2rtc_api.h"
#include "core/logger.h"
#include "core/config.h"
#include "core/shutdown_coordinator.h"  // For is_shutdown_initiated
#include "video/stream_manager.h"
#include "video/mp4_recording.h"
#include "video/hls/hls_api.h"
#include "video/streams.h"
#include "database/db_streams.h"
#include "video/stream_state.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <pthread.h>
#include <stdatomic.h>

// Tracking for streams using go2rtc
#define MAX_TRACKED_STREAMS 16

typedef struct {
    char stream_name[MAX_STREAM_NAME];
    bool using_go2rtc_for_recording;
    bool using_go2rtc_for_hls;
} go2rtc_stream_tracking_t;

// Store original stream URLs for restoration when stopping HLS
typedef struct {
    char stream_name[MAX_STREAM_NAME];
    char original_url[MAX_PATH_LENGTH];
    char original_username[MAX_STREAM_NAME];
    char original_password[MAX_STREAM_NAME];
} original_stream_config_t;

static go2rtc_stream_tracking_t g_tracked_streams[MAX_TRACKED_STREAMS] = {0};
static original_stream_config_t g_original_configs[MAX_TRACKED_STREAMS] = {0};
static bool g_initialized = false;

// ============================================================================
// Unified Health Monitor Configuration
// ============================================================================

// Health check interval in seconds (unified for both stream and process checks)
#define HEALTH_CHECK_INTERVAL_SEC 30

// Stream health: consecutive failures before re-registration
#define STREAM_MAX_CONSECUTIVE_FAILURES 3

// Stream health: cooldown after re-registration (seconds)
#define STREAM_REREGISTRATION_COOLDOWN_SEC 60

// Process health: consecutive API failures before restart
#define PROCESS_MAX_API_FAILURES 3

// Process health: minimum streams for consensus check
#define PROCESS_MIN_STREAMS_FOR_CONSENSUS 2

// Process health: cooldown after restart (seconds)
#define PROCESS_RESTART_COOLDOWN_SEC 120

// Process health: max restarts within window
#define PROCESS_MAX_RESTARTS_PER_WINDOW 5
#define PROCESS_RESTART_WINDOW_SEC 600  // 10 minutes

// Unified monitor state
static pthread_t g_monitor_thread;
static bool g_monitor_running = false;
static bool g_monitor_initialized = false;

// Process restart tracking
static int g_restart_count = 0;
static time_t g_last_restart_time = 0;
static int g_consecutive_api_failures = 0;
static time_t g_restart_history[PROCESS_MAX_RESTARTS_PER_WINDOW];
static int g_restart_history_index = 0;

/**
 * @brief Save original stream configuration
 *
 * @param stream_name Name of the stream
 * @param url Original URL
 * @param username Original username
 * @param password Original password
 */
static void save_original_config(const char *stream_name, const char *url,
                                const char *username, const char *password) {
    for (int i = 0; i < MAX_TRACKED_STREAMS; i++) {
        if (g_original_configs[i].stream_name[0] == '\0') {
            strncpy(g_original_configs[i].stream_name, stream_name, MAX_STREAM_NAME - 1);
            g_original_configs[i].stream_name[MAX_STREAM_NAME - 1] = '\0';

            strncpy(g_original_configs[i].original_url, url, MAX_PATH_LENGTH - 1);
            g_original_configs[i].original_url[MAX_PATH_LENGTH - 1] = '\0';

            strncpy(g_original_configs[i].original_username, username, MAX_STREAM_NAME - 1);
            g_original_configs[i].original_username[MAX_STREAM_NAME - 1] = '\0';

            strncpy(g_original_configs[i].original_password, password, MAX_STREAM_NAME - 1);
            g_original_configs[i].original_password[MAX_STREAM_NAME - 1] = '\0';

            return;
        }
    }
}

/**
 * @brief Get original stream configuration
 *
 * @param stream_name Name of the stream
 * @param url Buffer to store original URL
 * @param url_size Size of URL buffer
 * @param username Buffer to store original username
 * @param username_size Size of username buffer
 * @param password Buffer to store original password
 * @param password_size Size of password buffer
 * @return true if found, false otherwise
 */
static bool get_original_config(const char *stream_name, char *url, size_t url_size,
                              char *username, size_t username_size,
                              char *password, size_t password_size) {
    for (int i = 0; i < MAX_TRACKED_STREAMS; i++) {
        if (g_original_configs[i].stream_name[0] != '\0' &&
            strcmp(g_original_configs[i].stream_name, stream_name) == 0) {

            strncpy(url, g_original_configs[i].original_url, url_size - 1);
            url[url_size - 1] = '\0';

            strncpy(username, g_original_configs[i].original_username, username_size - 1);
            username[username_size - 1] = '\0';

            strncpy(password, g_original_configs[i].original_password, password_size - 1);
            password[password_size - 1] = '\0';

            // Clear the entry
            g_original_configs[i].stream_name[0] = '\0';

            return true;
        }
    }

    return false;
}

/**
 * @brief Find a tracked stream by name
 *
 * @param stream_name Name of the stream to find
 * @return Pointer to the tracking structure if found, NULL otherwise
 */
static go2rtc_stream_tracking_t *find_tracked_stream(const char *stream_name) {
    for (int i = 0; i < MAX_TRACKED_STREAMS; i++) {
        if (g_tracked_streams[i].stream_name[0] != '\0' &&
            strcmp(g_tracked_streams[i].stream_name, stream_name) == 0) {
            return &g_tracked_streams[i];
        }
    }
    return NULL;
}

/**
 * @brief Add a new tracked stream
 *
 * @param stream_name Name of the stream to track
 * @return Pointer to the new tracking structure if successful, NULL otherwise
 */
static go2rtc_stream_tracking_t *add_tracked_stream(const char *stream_name) {
    // First check if stream already exists
    go2rtc_stream_tracking_t *existing = find_tracked_stream(stream_name);
    if (existing) {
        return existing;
    }

    // Find an empty slot
    for (int i = 0; i < MAX_TRACKED_STREAMS; i++) {
        if (g_tracked_streams[i].stream_name[0] == '\0') {
            strncpy(g_tracked_streams[i].stream_name, stream_name, MAX_STREAM_NAME - 1);
            g_tracked_streams[i].stream_name[MAX_STREAM_NAME - 1] = '\0';
            g_tracked_streams[i].using_go2rtc_for_recording = false;
            g_tracked_streams[i].using_go2rtc_for_hls = false;
            return &g_tracked_streams[i];
        }
    }

    return NULL;
}

/**
 * @brief Check if a stream is registered with go2rtc
 *
 * @param stream_name Name of the stream to check
 * @return true if registered, false otherwise
 */
static bool is_stream_registered_with_go2rtc(const char *stream_name) {
    // Check if go2rtc is ready
    if (!go2rtc_stream_is_ready()) {
        log_warn("go2rtc service is not ready, cannot check if stream is registered");
        return false;
    }

    // Get the stream configuration
    stream_handle_t stream = get_stream_by_name(stream_name);
    if (!stream) {
        log_error("Stream %s not found", stream_name);
        return false;
    }

    stream_config_t config;
    if (get_stream_config(stream, &config) != 0) {
        log_error("Failed to get config for stream %s", stream_name);
        return false;
    }

    // Check if the stream is registered with go2rtc by trying to get its WebRTC URL
    char webrtc_url[1024];
    bool result = go2rtc_stream_get_webrtc_url(stream_name, webrtc_url, sizeof(webrtc_url));

    if (result) {
        log_info("Stream %s is registered with go2rtc, WebRTC URL: %s", stream_name, webrtc_url);
        return true;
    } else {
        log_info("Stream %s is not registered with go2rtc", stream_name);
        return false;
    }
}

/**
 * @brief Register a stream with go2rtc if not already registered
 *
 * @param stream_name Name of the stream to register
 * @return true if registered or already registered, false on failure
 */
static bool ensure_stream_registered_with_go2rtc(const char *stream_name) {
    // Check if go2rtc is ready
    if (!go2rtc_stream_is_ready()) {
        if (!go2rtc_stream_start_service()) {
            log_error("Failed to start go2rtc service");
            return false;
        }

        // Wait for service to start
        int retries = 10;
        while (retries > 0 && !go2rtc_stream_is_ready()) {
            log_info("Waiting for go2rtc service to start... (%d retries left)", retries);
            sleep(1);
            retries--;
        }

        if (!go2rtc_stream_is_ready()) {
            log_error("go2rtc service failed to start in time");
            return false;
        }
    }

    // Check if already registered
    if (is_stream_registered_with_go2rtc(stream_name)) {
        return true;
    }

    // Get the stream configuration
    stream_handle_t stream = get_stream_by_name(stream_name);
    if (!stream) {
        log_error("Stream %s not found", stream_name);
        return false;
    }

    stream_config_t config;
    if (get_stream_config(stream, &config) != 0) {
        log_error("Failed to get config for stream %s", stream_name);
        return false;
    }

    // Register the stream with go2rtc
    if (!go2rtc_stream_register(stream_name, config.url,
                               config.onvif_username[0] != '\0' ? config.onvif_username : NULL,
                               config.onvif_password[0] != '\0' ? config.onvif_password : NULL,
                               config.backchannel_enabled, config.protocol,
                               config.record_audio)) {
        log_error("Failed to register stream %s with go2rtc", stream_name);
        return false;
    }

    log_info("Successfully registered stream %s with go2rtc", stream_name);
    return true;
}

// ============================================================================
// Unified Health Monitor Implementation
// ============================================================================

/**
 * @brief Check if a stream needs re-registration with go2rtc
 */
static bool stream_needs_reregistration(const char *stream_name) {
    if (!stream_name) {
        return false;
    }

    stream_state_manager_t *state = get_stream_state_by_name(stream_name);
    if (!state) {
        return false;
    }

    if (!state->config.enabled) {
        return false;
    }

    if (state->state == STREAM_STATE_ERROR || state->state == STREAM_STATE_RECONNECTING) {
        int failures = atomic_load(&state->protocol_state.reconnect_attempts);

        if (failures >= STREAM_MAX_CONSECUTIVE_FAILURES) {
            time_t now = time(NULL);
            time_t last_reregister = atomic_load(&state->protocol_state.last_reconnect_time);

            if (now - last_reregister >= STREAM_REREGISTRATION_COOLDOWN_SEC) {
                log_info("Stream %s has %d consecutive failures, needs re-registration",
                        stream_name, failures);
                return true;
            }
        }
    }

    return false;
}

/**
 * @brief Check stream consensus - if all/most streams are down, it's likely go2rtc
 */
static bool check_stream_consensus(void) {
    int total_streams = 0;
    int failed_streams = 0;

    int stream_count = get_total_stream_count();

    if (stream_count < PROCESS_MIN_STREAMS_FOR_CONSENSUS) {
        return false;
    }

    for (int i = 0; i < stream_count; i++) {
        stream_handle_t stream = get_stream_by_index(i);
        if (!stream) continue;

        stream_config_t config;
        if (get_stream_config(stream, &config) != 0) continue;

        if (!config.enabled) continue;

        total_streams++;

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

    if (total_streams >= PROCESS_MIN_STREAMS_FOR_CONSENSUS &&
        failed_streams == total_streams && total_streams > 0) {
        log_warn("Stream consensus: %d/%d streams failed - indicates go2rtc issue",
                 failed_streams, total_streams);
        return true;
    }

    return false;
}

/**
 * @brief Check if we can restart (rate limiting)
 */
static bool can_restart_go2rtc(void) {
    time_t now = time(NULL);

    if (now - g_last_restart_time < PROCESS_RESTART_COOLDOWN_SEC) {
        log_warn("go2rtc restart blocked: cooldown period (%ld seconds remaining)",
                 PROCESS_RESTART_COOLDOWN_SEC - (now - g_last_restart_time));
        return false;
    }

    int recent_restarts = 0;
    for (int i = 0; i < PROCESS_MAX_RESTARTS_PER_WINDOW; i++) {
        if (g_restart_history[i] > 0 && (now - g_restart_history[i]) < PROCESS_RESTART_WINDOW_SEC) {
            recent_restarts++;
        }
    }

    if (recent_restarts >= PROCESS_MAX_RESTARTS_PER_WINDOW) {
        log_error("go2rtc restart blocked: too many restarts (%d in last %d seconds)",
                  recent_restarts, PROCESS_RESTART_WINDOW_SEC);
        return false;
    }

    return true;
}

/**
 * @brief Restart the go2rtc process
 */
static bool restart_go2rtc_process(void) {
    log_warn("Attempting to restart go2rtc process due to health check failure");

    log_info("Stopping go2rtc process...");
    if (!go2rtc_process_stop()) {
        log_error("Failed to stop go2rtc process");
        return false;
    }

    sleep(2);

    int api_port = go2rtc_stream_get_api_port();
    if (api_port == 0) {
        log_error("Failed to get go2rtc API port");
        return false;
    }

    log_info("Starting go2rtc process...");
    if (!go2rtc_process_start(api_port)) {
        log_error("Failed to start go2rtc process");
        return false;
    }

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

    log_info("Re-registering all streams with go2rtc after restart");
    if (!go2rtc_integration_register_all_streams()) {
        log_warn("Failed to re-register all streams after go2rtc restart");
    } else {
        log_info("All streams re-registered successfully after go2rtc restart");
    }

    sleep(2);

    log_info("Signaling MP4 recordings to reconnect after go2rtc restart");
    signal_all_mp4_recordings_reconnect();

    time_t now = time(NULL);
    g_last_restart_time = now;
    g_restart_count++;
    g_restart_history[g_restart_history_index] = now;
    g_restart_history_index = (g_restart_history_index + 1) % PROCESS_MAX_RESTARTS_PER_WINDOW;
    g_consecutive_api_failures = 0;

    log_info("go2rtc restart completed (total restarts: %d)", g_restart_count);

    return true;
}

/**
 * @brief Unified health monitor thread - handles both stream and process health
 */
static void *unified_health_monitor_thread(void *arg) {
    (void)arg;

    log_info("Unified go2rtc health monitor thread started");

    while (g_monitor_running && !is_shutdown_initiated()) {
        // Sleep for the check interval (1 second at a time for responsiveness)
        for (int i = 0; i < HEALTH_CHECK_INTERVAL_SEC && g_monitor_running && !is_shutdown_initiated(); i++) {
            sleep(1);
        }

        if (!g_monitor_running || is_shutdown_initiated()) {
            break;
        }

        // =====================================================================
        // Phase 1: Process-level health check (check go2rtc API itself)
        // =====================================================================
        bool api_healthy = go2rtc_stream_is_ready();
        bool process_restarted = false;

        if (!api_healthy) {
            g_consecutive_api_failures++;
            log_warn("go2rtc API health check failed (consecutive failures: %d/%d)",
                     g_consecutive_api_failures, PROCESS_MAX_API_FAILURES);

            if (g_consecutive_api_failures >= PROCESS_MAX_API_FAILURES) {
                log_error("go2rtc API has failed %d consecutive health checks", g_consecutive_api_failures);

                bool consensus_failure = check_stream_consensus();
                if (consensus_failure) {
                    log_error("Stream consensus also indicates go2rtc failure - restart required");
                }

                if (can_restart_go2rtc()) {
                    if (restart_go2rtc_process()) {
                        log_info("go2rtc process successfully restarted");
                        process_restarted = true;
                    } else {
                        log_error("Failed to restart go2rtc process");
                    }
                }
            }

            // Skip stream checks if API is unhealthy
            continue;
        } else {
            if (g_consecutive_api_failures > 0) {
                log_info("go2rtc API health check succeeded, resetting failure counter");
                g_consecutive_api_failures = 0;
            }
        }

        // =====================================================================
        // Phase 2: Stream-level health check (only if process is healthy)
        // =====================================================================
        if (process_restarted) {
            // Skip stream checks right after restart - give streams time to reconnect
            continue;
        }

        int stream_count = get_total_stream_count();
        log_debug("Health monitor checking %d streams", stream_count);

        for (int i = 0; i < stream_count; i++) {
            if (!g_monitor_running || is_shutdown_initiated()) {
                break;
            }

            stream_handle_t stream = get_stream_by_index(i);
            if (!stream) continue;

            stream_config_t config;
            if (get_stream_config(stream, &config) != 0) continue;

            if (stream_needs_reregistration(config.name)) {
                log_info("Stream %s needs re-registration, attempting to fix", config.name);

                if (go2rtc_integration_reload_stream(config.name)) {
                    log_info("Successfully re-registered stream %s", config.name);

                    // Update reconnect state
                    stream_state_manager_t *state = get_stream_state_by_name(config.name);
                    if (state) {
                        atomic_store(&state->protocol_state.last_reconnect_time, time(NULL));
                        atomic_store(&state->protocol_state.reconnect_attempts, 0);
                    }
                } else {
                    log_error("Failed to re-register stream %s", config.name);
                }
            }
        }
    }

    log_info("Unified go2rtc health monitor thread exiting");
    return NULL;
}

/**
 * @brief Start the unified health monitor
 */
static bool start_unified_health_monitor(void) {
    if (g_monitor_initialized) {
        log_warn("Unified health monitor already initialized");
        return true;
    }

    log_info("Starting unified go2rtc health monitor");

    // Initialize restart tracking
    memset(g_restart_history, 0, sizeof(g_restart_history));
    g_restart_history_index = 0;
    g_restart_count = 0;
    g_last_restart_time = 0;
    g_consecutive_api_failures = 0;

    g_monitor_running = true;
    g_monitor_initialized = true;

    if (pthread_create(&g_monitor_thread, NULL, unified_health_monitor_thread, NULL) != 0) {
        log_error("Failed to create unified health monitor thread");
        g_monitor_running = false;
        g_monitor_initialized = false;
        return false;
    }

    log_info("Unified go2rtc health monitor started successfully");
    return true;
}

/**
 * @brief Stop the unified health monitor
 */
static void stop_unified_health_monitor(void) {
    if (!g_monitor_initialized) {
        return;
    }

    log_info("Stopping unified go2rtc health monitor");

    g_monitor_running = false;

    if (pthread_join(g_monitor_thread, NULL) != 0) {
        log_warn("Failed to join unified health monitor thread");
    }

    g_monitor_initialized = false;
    log_info("Unified go2rtc health monitor stopped");
}

// ============================================================================
// Module Initialization/Cleanup
// ============================================================================

bool go2rtc_integration_init(void) {
    if (g_initialized) {
        log_warn("go2rtc integration module already initialized");
        return true;
    }

    // Initialize the go2rtc consumer module
    if (!go2rtc_consumer_init()) {
        log_error("Failed to initialize go2rtc consumer module");
        return false;
    }

    // Initialize tracking array
    memset(g_tracked_streams, 0, sizeof(g_tracked_streams));

    // Start the unified health monitor (replaces separate stream and process monitors)
    if (!start_unified_health_monitor()) {
        log_warn("Failed to start unified health monitor (non-fatal)");
        // Continue anyway - health monitor is optional
    }

    g_initialized = true;
    log_info("go2rtc integration module initialized");

    return true;
}

/**
 * Ensure go2rtc is ready and the stream is registered
 *
 * @param stream_name Name of the stream
 * @return true if go2rtc is ready and the stream is registered, false otherwise
 */
static bool ensure_go2rtc_ready_for_stream(const char *stream_name) {
    // Check if go2rtc is ready with more retries and longer timeout
    if (!go2rtc_stream_is_ready()) {
        log_info("go2rtc service is not ready, starting it...");
        if (!go2rtc_stream_start_service()) {
            log_error("Failed to start go2rtc service");
            return false;
        }

        // Wait for service to start with increased retries and timeout
        int retries = 20; // Increased from 10 to 20
        while (retries > 0 && !go2rtc_stream_is_ready()) {
            log_info("Waiting for go2rtc service to start... (%d retries left)", retries);
            sleep(2); // Increased from 1 to 2 seconds
            retries--;
        }

        if (!go2rtc_stream_is_ready()) {
            log_error("go2rtc service failed to start in time");
            return false;
        }

        log_info("go2rtc service started successfully");
    }

    // Check if the stream is registered with go2rtc
    if (!is_stream_registered_with_go2rtc(stream_name)) {
        log_info("Stream %s is not registered with go2rtc, registering it...", stream_name);
        if (!ensure_stream_registered_with_go2rtc(stream_name)) {
            log_error("Failed to register stream %s with go2rtc", stream_name);
            return false;
        }

        // Wait longer for the stream to be fully registered
        log_info("Waiting for stream %s to be fully registered with go2rtc", stream_name);
        sleep(5); // Increased from 3 to 5 seconds

        // Check again if the stream is registered
        if (!is_stream_registered_with_go2rtc(stream_name)) {
            log_error("Stream %s still not registered with go2rtc after registration attempt", stream_name);
            return false;
        }

        log_info("Stream %s successfully registered with go2rtc", stream_name);
    }

    return true;
}

int go2rtc_integration_start_recording(const char *stream_name) {
    if (!g_initialized) {
        log_error("go2rtc integration module not initialized");
        return -1;
    }

    if (!stream_name) {
        log_error("Invalid parameter for go2rtc_integration_start_recording");
        return -1;
    }

    // Get the stream configuration
    stream_handle_t stream = get_stream_by_name(stream_name);
    if (!stream) {
        log_error("Stream %s not found", stream_name);
        return -1;
    }

    stream_config_t config;
    if (get_stream_config(stream, &config) != 0) {
        log_error("Failed to get config for stream %s", stream_name);
        return -1;
    }

    // Ensure go2rtc is ready and the stream is registered
    bool using_go2rtc = ensure_go2rtc_ready_for_stream(stream_name);

    // If go2rtc is ready and the stream is registered, use go2rtc's RTSP URL for recording
    if (using_go2rtc) {
        log_info("Using go2rtc's RTSP output as input for MP4 recording of stream %s", stream_name);

        // Get the go2rtc RTSP URL for this stream
        char rtsp_url[MAX_PATH_LENGTH];
        if (!go2rtc_get_rtsp_url(stream_name, rtsp_url, sizeof(rtsp_url))) {
            log_error("Failed to get go2rtc RTSP URL for stream %s", stream_name);
            // Fall back to default recording
            log_info("Falling back to default recording for stream %s", stream_name);
            return start_mp4_recording(stream_name);
        }

        // Start MP4 recording using the go2rtc RTSP URL
        int result = start_mp4_recording_with_url(stream_name, rtsp_url);

        // Update tracking if successful
        if (result == 0) {
            go2rtc_stream_tracking_t *tracking = add_tracked_stream(stream_name);
            if (tracking) {
                tracking->using_go2rtc_for_recording = true;
            }

            log_info("Started MP4 recording for stream %s using go2rtc's RTSP output", stream_name);
        }

        return result;
    } else {
        // Fall back to default recording
        log_info("Using default recording for stream %s", stream_name);
        return start_mp4_recording(stream_name);
    }
}

int go2rtc_integration_stop_recording(const char *stream_name) {
    if (!g_initialized) {
        log_error("go2rtc integration module not initialized");
        return -1;
    }

    if (!stream_name) {
        log_error("Invalid parameter for go2rtc_integration_stop_recording");
        return -1;
    }

    // Check if the stream is using go2rtc for recording
    go2rtc_stream_tracking_t *tracking = find_tracked_stream(stream_name);
    if (tracking && tracking->using_go2rtc_for_recording) {
        log_info("Stopping recording for stream %s using go2rtc", stream_name);

        // Stop recording using go2rtc consumer
        if (!go2rtc_consumer_stop_recording(stream_name)) {
            log_error("Failed to stop recording for stream %s using go2rtc", stream_name);
            return -1;
        }

        // Update tracking
        tracking->using_go2rtc_for_recording = false;

        log_info("Stopped recording for stream %s using go2rtc", stream_name);
        return 0;
    } else {
        // Fall back to default recording
        log_info("Using default method to stop recording for stream %s", stream_name);
        return stop_mp4_recording(stream_name);
    }
}


int go2rtc_integration_start_hls(const char *stream_name) {
    if (!g_initialized) {
        log_error("go2rtc integration module not initialized");
        return -1;
    }

    if (!stream_name) {
        log_error("Invalid parameter for go2rtc_integration_start_hls");
        return -1;
    }

    // CRITICAL FIX: Check if shutdown is in progress and prevent starting new streams
    if (is_shutdown_initiated()) {
        log_warn("Cannot start HLS stream %s during shutdown", stream_name);
        return -1;
    }

    // Get the stream configuration
    stream_handle_t stream = get_stream_by_name(stream_name);
    if (!stream) {
        log_error("Stream %s not found", stream_name);
        return -1;
    }

    stream_config_t config;
    if (get_stream_config(stream, &config) != 0) {
        log_error("Failed to get config for stream %s", stream_name);
        return -1;
    }

    // Ensure go2rtc is ready and the stream is registered
    bool using_go2rtc = ensure_go2rtc_ready_for_stream(stream_name);

    // If go2rtc is ready and the stream is registered, use go2rtc for HLS streaming
    if (using_go2rtc) {
        log_info("Using go2rtc's RTSP output as input for HLS streaming of stream %s", stream_name);

        // Start HLS streaming using our default implementation
        // The URL substitution will happen in hls_writer_thread.c
        int result = start_hls_stream(stream_name);

        // Update tracking if successful
        if (result == 0) {
            go2rtc_stream_tracking_t *tracking = add_tracked_stream(stream_name);
            if (tracking) {
                tracking->using_go2rtc_for_hls = true;
            }

            log_info("Started HLS streaming for stream %s using go2rtc's RTSP output", stream_name);
        }

        return result;
    } else {
        // Fall back to default HLS streaming
        log_info("Using default HLS streaming for stream %s", stream_name);
        return start_hls_stream(stream_name);
    }
}


int go2rtc_integration_stop_hls(const char *stream_name) {
    if (!g_initialized) {
        log_error("go2rtc integration module not initialized");
        return -1;
    }

    if (!stream_name) {
        log_error("Invalid parameter for go2rtc_integration_stop_hls");
        return -1;
    }

    // Check if the stream is using go2rtc for HLS
    go2rtc_stream_tracking_t *tracking = find_tracked_stream(stream_name);
    if (tracking && tracking->using_go2rtc_for_hls) {
        log_info("Stopping HLS streaming for stream %s using go2rtc", stream_name);

        // Get the stream state manager to ensure proper cleanup
        stream_state_manager_t *state = get_stream_state_by_name(stream_name);

        // Store a local copy of the HLS writer pointer if it exists
        hls_writer_t *writer = NULL;
        if (state && state->hls_ctx) {
            writer = (hls_writer_t *)state->hls_ctx;
            // Clear the reference in the state before stopping the stream
            // This prevents accessing freed memory later
            state->hls_ctx = NULL;
        }

        // Stop HLS streaming
        int result = stop_hls_stream(stream_name);
        if (result != 0) {
            log_error("Failed to stop HLS streaming for stream %s", stream_name);
            return result;
        }

        // Update tracking
        tracking->using_go2rtc_for_hls = false;

        // We've already cleared state->hls_ctx, so we don't need to do it again

        log_info("Stopped HLS streaming for stream %s using go2rtc", stream_name);
        return 0;
    } else {
        // Fall back to default HLS streaming
        log_info("Using default method to stop HLS streaming for stream %s", stream_name);
        return stop_hls_stream(stream_name);
    }
}

bool go2rtc_integration_is_using_go2rtc_for_recording(const char *stream_name) {
    if (!g_initialized || !stream_name) {
        return false;
    }

    go2rtc_stream_tracking_t *tracking = find_tracked_stream(stream_name);
    return tracking ? tracking->using_go2rtc_for_recording : false;
}

bool go2rtc_integration_is_using_go2rtc_for_hls(const char *stream_name) {
    if (!g_initialized || !stream_name) {
        return false;
    }

    go2rtc_stream_tracking_t *tracking = find_tracked_stream(stream_name);
    return tracking ? tracking->using_go2rtc_for_hls : false;
}

/**
 * @brief Register all existing streams with go2rtc
 *
 * @return true if successful, false otherwise
 */
bool go2rtc_integration_register_all_streams(void) {
    if (!g_initialized) {
        log_error("go2rtc integration module not initialized");
        return false;
    }

    // Check if go2rtc is ready
    if (!go2rtc_stream_is_ready()) {
        log_error("go2rtc service is not ready");
        return false;
    }

    // Get all stream configurations
    stream_config_t streams[MAX_STREAMS];
    int count = get_all_stream_configs(streams, MAX_STREAMS);

    if (count <= 0) {
        log_info("No streams found to register with go2rtc");
        return true; // Not an error, just no streams
    }

    log_info("Registering %d streams with go2rtc", count);

    // Register each stream with go2rtc
    bool all_success = true;
    for (int i = 0; i < count; i++) {
        if (streams[i].enabled) {
            log_info("Registering stream %s with go2rtc", streams[i].name);

            // Register the stream with go2rtc
            if (!go2rtc_stream_register(streams[i].name, streams[i].url,
                                       streams[i].onvif_username[0] != '\0' ? streams[i].onvif_username : NULL,
                                       streams[i].onvif_password[0] != '\0' ? streams[i].onvif_password : NULL,
                                       streams[i].backchannel_enabled, streams[i].protocol,
                                       streams[i].record_audio)) {
                log_error("Failed to register stream %s with go2rtc", streams[i].name);
                all_success = false;
                // Continue with other streams
            } else {
                log_info("Successfully registered stream %s with go2rtc", streams[i].name);
            }
        }
    }

    return all_success;
}

/**
 * @brief Sync database streams to go2rtc
 *
 * This function reads all enabled streams from the database and ensures
 * they are registered with go2rtc. It checks if each stream already exists
 * in go2rtc before registering to avoid duplicate registrations.
 *
 * This is the preferred function to call after stream add/update/delete
 * operations to ensure go2rtc stays in sync with the database.
 *
 * @return true if all streams were synced successfully, false otherwise
 */
bool go2rtc_sync_streams_from_database(void) {
    if (!g_initialized) {
        log_warn("go2rtc integration module not initialized, cannot sync streams");
        return false;
    }

    // Check if go2rtc is ready
    if (!go2rtc_stream_is_ready()) {
        log_warn("go2rtc service is not ready, cannot sync streams");
        return false;
    }

    // Get all stream configurations from database
    stream_config_t db_streams[MAX_STREAMS];
    int count = get_all_stream_configs(db_streams, MAX_STREAMS);

    if (count < 0) {
        log_error("Failed to get stream configurations from database");
        return false;
    }

    if (count == 0) {
        log_info("No streams found in database to sync with go2rtc");
        return true; // Not an error, just no streams
    }

    log_info("Syncing %d streams from database to go2rtc", count);

    bool all_success = true;
    int synced = 0;
    int skipped = 0;
    int failed = 0;

    for (int i = 0; i < count; i++) {
        // Skip disabled streams
        if (!db_streams[i].enabled) {
            log_debug("Skipping disabled stream %s", db_streams[i].name);
            skipped++;
            continue;
        }

        // Check if stream already exists in go2rtc
        if (go2rtc_api_stream_exists(db_streams[i].name)) {
            log_debug("Stream %s already exists in go2rtc, skipping", db_streams[i].name);
            skipped++;
            continue;
        }

        // Stream needs to be registered
        log_info("Registering missing stream %s with go2rtc", db_streams[i].name);

        // Determine username and password
        const char *username = NULL;
        const char *password = NULL;

        if (db_streams[i].onvif_username[0] != '\0') {
            username = db_streams[i].onvif_username;
        }
        if (db_streams[i].onvif_password[0] != '\0') {
            password = db_streams[i].onvif_password;
        }

        // Register the stream
        if (!go2rtc_stream_register(db_streams[i].name, db_streams[i].url,
                                    username, password,
                                    db_streams[i].backchannel_enabled, db_streams[i].protocol,
                                    db_streams[i].record_audio)) {
            log_error("Failed to register stream %s with go2rtc", db_streams[i].name);
            all_success = false;
            failed++;
        } else {
            log_info("Successfully synced stream %s to go2rtc", db_streams[i].name);
            synced++;
        }
    }

    log_info("go2rtc sync complete: %d synced, %d skipped, %d failed", synced, skipped, failed);
    return all_success;
}

void go2rtc_integration_cleanup(void) {
    if (!g_initialized) {
        return;
    }

    log_info("Cleaning up go2rtc integration module");

    // Stop the unified health monitor
    stop_unified_health_monitor();

    // Stop all recording and HLS streaming using go2rtc
    for (int i = 0; i < MAX_TRACKED_STREAMS; i++) {
        if (g_tracked_streams[i].stream_name[0] != '\0') {
            if (g_tracked_streams[i].using_go2rtc_for_recording) {
                log_info("Stopping recording for stream %s during cleanup", g_tracked_streams[i].stream_name);
                go2rtc_consumer_stop_recording(g_tracked_streams[i].stream_name);
            }

            if (g_tracked_streams[i].using_go2rtc_for_hls) {
                log_info("Stopping HLS streaming for stream %s during cleanup", g_tracked_streams[i].stream_name);
                // Use our own stop function to ensure proper thread cleanup
                go2rtc_integration_stop_hls(g_tracked_streams[i].stream_name);
            }

            // Clear tracking
            g_tracked_streams[i].stream_name[0] = '\0';
            g_tracked_streams[i].using_go2rtc_for_recording = false;
            g_tracked_streams[i].using_go2rtc_for_hls = false;
        }
    }

    // Clean up the go2rtc consumer module
    go2rtc_consumer_cleanup();

    g_initialized = false;
    log_info("go2rtc integration module cleaned up");
}

bool go2rtc_integration_is_initialized(void) {
    return g_initialized;
}

/**
 * @brief Get the RTSP URL for a stream from go2rtc with enhanced error handling
 *
 * @param stream_name Name of the stream
 * @param url Buffer to store the URL
 * @param url_size Size of the URL buffer
 * @return true if successful, false otherwise
 */
bool go2rtc_get_rtsp_url(const char *stream_name, char *url, size_t url_size) {
    if (!stream_name || !url || url_size == 0) {
        log_error("Invalid parameters for go2rtc_get_rtsp_url");
        return false;
    }

    // Check if go2rtc is ready with retry logic
    int ready_retries = 3;
    while (!go2rtc_stream_is_ready() && ready_retries > 0) {
        log_warn("go2rtc service is not ready, retrying... (%d attempts left)", ready_retries);

        // Try to start the service if it's not ready
        if (!go2rtc_stream_start_service()) {
            log_error("Failed to start go2rtc service");
            ready_retries--;
            sleep(2);
            continue;
        }

        // Wait for service to start
        int wait_retries = 5;
        while (wait_retries > 0 && !go2rtc_stream_is_ready()) {
            log_info("Waiting for go2rtc service to start... (%d retries left)", wait_retries);
            sleep(2);
            wait_retries--;
        }

        if (go2rtc_stream_is_ready()) {
            log_info("go2rtc service is now ready");
            break;
        }

        ready_retries--;
    }

    if (!go2rtc_stream_is_ready()) {
        log_error("go2rtc service is not ready after multiple attempts, cannot get RTSP URL");
        return false;
    }

    // Check if the stream is registered with go2rtc
    if (!is_stream_registered_with_go2rtc(stream_name)) {
        log_info("Stream %s is not registered with go2rtc, attempting to register...", stream_name);

        // Try to register the stream with go2rtc with retry logic
        int register_retries = 3;
        bool registered = false;

        while (!registered && register_retries > 0) {
            if (ensure_stream_registered_with_go2rtc(stream_name)) {
                registered = true;
                break;
            }

            log_warn("Failed to register stream %s with go2rtc, retrying... (%d attempts left)",
                    stream_name, register_retries - 1);
            register_retries--;
            sleep(2);
        }

        if (!registered) {
            log_error("Failed to register stream %s with go2rtc after multiple attempts", stream_name);
            return false;
        }

        // Wait longer for the stream to be fully registered
        log_info("Waiting for stream %s to be fully registered with go2rtc", stream_name);
        sleep(5);

        // Check again if the stream is registered
        if (!is_stream_registered_with_go2rtc(stream_name)) {
            log_error("Stream %s still not registered with go2rtc after registration attempt", stream_name);
            return false;
        }

        log_info("Stream %s successfully registered with go2rtc", stream_name);
    }

    // Use the stream module to get the RTSP URL with the correct port
    if (!go2rtc_stream_get_rtsp_url(stream_name, url, url_size)) {
        log_error("Failed to get RTSP URL for stream %s", stream_name);
        return false;
    }

    log_info("Successfully got RTSP URL for stream %s: %s", stream_name, url);
    return true;
}

bool go2rtc_integration_get_hls_url(const char *stream_name, char *buffer, size_t buffer_size) {
    if (!g_initialized || !stream_name || !buffer || buffer_size == 0) {
        return false;
    }

    // Check if the stream is using go2rtc for HLS
    if (!go2rtc_integration_is_using_go2rtc_for_hls(stream_name)) {
        return false;
    }

    // Check if go2rtc is ready
    if (!go2rtc_stream_is_ready()) {
        log_warn("go2rtc service is not ready, cannot get HLS URL");
        return false;
    }

    // Format the HLS URL
    // The format is http://localhost:{port}/api/stream.m3u8?src={stream_name}
    int api_port = go2rtc_stream_get_api_port();
    if (api_port == 0) {
        api_port = 1984; // Fallback to default port
    }
    snprintf(buffer, buffer_size, "http://localhost:%d/api/stream.m3u8?src=%s", api_port, stream_name);

    log_info("Generated go2rtc HLS URL for stream %s: %s", stream_name, buffer);
    return true;
}

bool go2rtc_integration_reload_stream_config(const char *stream_name,
                                             const char *new_url,
                                             const char *new_username,
                                             const char *new_password,
                                             int new_backchannel_enabled,
                                             int new_protocol,
                                             int new_record_audio) {
    if (!stream_name) {
        log_error("go2rtc_integration_reload_stream_config: stream_name is NULL");
        return false;
    }

    log_info("Reloading stream configuration for %s in go2rtc", stream_name);

    // Check if go2rtc is ready
    if (!go2rtc_stream_is_ready()) {
        log_warn("go2rtc service is not ready, cannot reload stream config");
        return false;
    }

    // Get current stream configuration if new values not provided
    stream_handle_t stream = get_stream_by_name(stream_name);
    stream_config_t config;
    bool have_config = false;

    if (stream && get_stream_config(stream, &config) == 0) {
        have_config = true;
    }

    // Determine the values to use
    const char *url = new_url;
    const char *username = new_username;
    const char *password = new_password;
    bool backchannel = (new_backchannel_enabled >= 0) ? (new_backchannel_enabled != 0) : false;
    stream_protocol_t protocol = STREAM_PROTOCOL_TCP;  // default
    bool record_audio = false;  // default

    if (have_config) {
        if (!url) url = config.url;
        if (!username) username = config.onvif_username[0] != '\0' ? config.onvif_username : NULL;
        if (!password) password = config.onvif_password[0] != '\0' ? config.onvif_password : NULL;
        if (new_backchannel_enabled < 0) backchannel = config.backchannel_enabled;
        if (new_protocol < 0) protocol = config.protocol;
        if (new_record_audio < 0) record_audio = config.record_audio;
    }

    // If new_record_audio is explicitly provided, use it
    if (new_record_audio >= 0) {
        record_audio = (new_record_audio != 0);
    }

    // If new_protocol is explicitly provided, use it
    if (new_protocol >= 0) {
        protocol = (stream_protocol_t)new_protocol;
    }

    if (!url || url[0] == '\0') {
        log_error("go2rtc_integration_reload_stream_config: No URL available for stream %s", stream_name);
        return false;
    }

    // Unregister the old stream first (don't fail if it wasn't registered)
    if (go2rtc_stream_unregister(stream_name)) {
        log_info("Unregistered old stream %s from go2rtc", stream_name);
    } else {
        log_info("Stream %s was not registered with go2rtc (or unregister failed)", stream_name);
    }

    // Wait a moment for go2rtc to clean up
    usleep(500000); // 500ms

    // Re-register with new configuration
    if (!go2rtc_stream_register(stream_name, url, username, password, backchannel, protocol, record_audio)) {
        log_error("Failed to re-register stream %s with go2rtc", stream_name);
        return false;
    }

    log_info("Successfully reloaded stream %s in go2rtc with URL: %s (protocol=%s)",
             stream_name, url, protocol == STREAM_PROTOCOL_UDP ? "UDP" : "TCP");
    return true;
}

bool go2rtc_integration_reload_stream(const char *stream_name) {
    if (!stream_name) {
        log_error("go2rtc_integration_reload_stream: stream_name is NULL");
        return false;
    }

    // Use the generic reload function with -1 values to use current config
    return go2rtc_integration_reload_stream_config(stream_name, NULL, NULL, NULL, -1, -1, -1);
}

bool go2rtc_integration_unregister_stream(const char *stream_name) {
    if (!stream_name) {
        log_error("go2rtc_integration_unregister_stream: stream_name is NULL");
        return false;
    }

    if (!go2rtc_stream_is_ready()) {
        log_warn("go2rtc service is not ready, cannot unregister stream");
        return false;
    }

    if (go2rtc_stream_unregister(stream_name)) {
        log_info("Unregistered stream %s from go2rtc", stream_name);

        // Also remove from tracking
        for (int i = 0; i < MAX_TRACKED_STREAMS; i++) {
            if (strcmp(g_tracked_streams[i].stream_name, stream_name) == 0) {
                memset(&g_tracked_streams[i], 0, sizeof(go2rtc_stream_tracking_t));
                break;
            }
        }

        return true;
    }

    log_warn("Failed to unregister stream %s from go2rtc", stream_name);
    return false;
}

bool go2rtc_integration_register_stream(const char *stream_name) {
    if (!stream_name) {
        log_error("go2rtc_integration_register_stream: stream_name is NULL");
        return false;
    }

    if (!go2rtc_stream_is_ready()) {
        log_debug("go2rtc service is not ready, cannot register stream %s", stream_name);
        return false;
    }

    // Look up the stream config
    stream_handle_t stream = get_stream_by_name(stream_name);
    if (!stream) {
        log_error("Stream %s not found in stream manager", stream_name);
        return false;
    }

    stream_config_t config;
    if (get_stream_config(stream, &config) != 0) {
        log_error("Failed to get config for stream %s", stream_name);
        return false;
    }

    // Determine username and password
    // Priority: 1) onvif fields, 2) extracted from URL
    char username[64] = {0};
    char password[64] = {0};

    if (config.onvif_username[0] != '\0') {
        strncpy(username, config.onvif_username, sizeof(username) - 1);
    }
    if (config.onvif_password[0] != '\0') {
        strncpy(password, config.onvif_password, sizeof(password) - 1);
    }

    // If credentials not in onvif fields, try to extract from URL
    // Format: rtsp://username:password@host:port/path
    if (username[0] == '\0') {
        const char *url = config.url;
        if (strncmp(url, "rtsp://", 7) == 0) {
            const char *at_sign = strchr(url + 7, '@');
            if (at_sign) {
                const char *colon = strchr(url + 7, ':');
                if (colon && colon < at_sign) {
                    // Extract username
                    size_t username_len = colon - (url + 7);
                    if (username_len < sizeof(username)) {
                        strncpy(username, url + 7, username_len);
                        username[username_len] = '\0';

                        // Extract password if not already set
                        if (password[0] == '\0') {
                            size_t password_len = at_sign - (colon + 1);
                            if (password_len < sizeof(password)) {
                                strncpy(password, colon + 1, password_len);
                                password[password_len] = '\0';
                            }
                        }
                    }
                }
            }
        }
    }

    // Register with go2rtc
    if (go2rtc_stream_register(stream_name, config.url,
                               username[0] != '\0' ? username : NULL,
                               password[0] != '\0' ? password : NULL,
                               config.backchannel_enabled, config.protocol,
                               config.record_audio)) {
        log_info("Successfully registered stream %s with go2rtc", stream_name);
        return true;
    }

    log_warn("Failed to register stream %s with go2rtc", stream_name);
    return false;
}

// ============================================================================
// Public Health Monitor API
// ============================================================================

bool go2rtc_integration_monitor_is_running(void) {
    return g_monitor_initialized && g_monitor_running;
}

int go2rtc_integration_get_restart_count(void) {
    return g_restart_count;
}

time_t go2rtc_integration_get_last_restart_time(void) {
    return g_last_restart_time;
}

bool go2rtc_integration_check_health(void) {
    return go2rtc_stream_is_ready();
}
