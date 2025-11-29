/**
 * @file go2rtc_integration.c
 * @brief Implementation of the go2rtc integration with existing recording and HLS systems
 */

#include "video/go2rtc/go2rtc_integration.h"
#include "video/go2rtc/go2rtc_consumer.h"
#include "video/go2rtc/go2rtc_stream.h"
#include "video/go2rtc/go2rtc_health_monitor.h"
#include "video/go2rtc/go2rtc_process_monitor.h"
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
                               config.backchannel_enabled)) {
        log_error("Failed to register stream %s with go2rtc", stream_name);
        return false;
    }

    log_info("Successfully registered stream %s with go2rtc", stream_name);
    return true;
}

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

    // Initialize the stream health monitor
    if (!go2rtc_health_monitor_init()) {
        log_warn("Failed to initialize go2rtc stream health monitor (non-fatal)");
        // Continue anyway - health monitor is optional
    } else {
        log_info("go2rtc stream health monitor initialized successfully");
    }

    // Initialize the process health monitor
    if (!go2rtc_process_monitor_init()) {
        log_warn("Failed to initialize go2rtc process health monitor (non-fatal)");
        // Continue anyway - process monitor is optional
    } else {
        log_info("go2rtc process health monitor initialized successfully");
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
                                       streams[i].backchannel_enabled)) {
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

void go2rtc_integration_cleanup(void) {
    if (!g_initialized) {
        return;
    }

    log_info("Cleaning up go2rtc integration module");

    // Clean up the process health monitor first
    go2rtc_process_monitor_cleanup();

    // Clean up the stream health monitor
    go2rtc_health_monitor_cleanup();

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
    // The format is http://localhost:1984/api/stream.m3u8?src={stream_name}
    snprintf(buffer, buffer_size, "http://localhost:1984/api/stream.m3u8?src=%s", stream_name);

    log_info("Generated go2rtc HLS URL for stream %s: %s", stream_name, buffer);
    return true;
}
