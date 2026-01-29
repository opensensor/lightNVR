/**
 * @file go2rtc_integration.h
 * @brief Header for the go2rtc integration with existing recording and HLS systems
 */

#ifndef GO2RTC_INTEGRATION_H
#define GO2RTC_INTEGRATION_H

#include <stdbool.h>
#include <stddef.h>
#include <time.h>
#include "core/config.h"  // For stream_protocol_t

/**
 * @brief Initialize the go2rtc integration module
 *
 * @return true if successful, false otherwise
 */
bool go2rtc_integration_init(void);

/**
 * @brief Start recording a stream using go2rtc if available, otherwise use default recording
 *
 * @param stream_name Name of the stream to record
 * @return 0 if successful, -1 otherwise
 */
int go2rtc_integration_start_recording(const char *stream_name);

/**
 * @brief Stop recording a stream
 *
 * @param stream_name Name of the stream to stop recording
 * @return 0 if successful, -1 otherwise
 */
int go2rtc_integration_stop_recording(const char *stream_name);

/**
 * @brief Start HLS streaming for a stream using go2rtc if available, otherwise use default HLS
 *
 * @param stream_name Name of the stream to stream
 * @return 0 if successful, -1 otherwise
 */
int go2rtc_integration_start_hls(const char *stream_name);

/**
 * @brief Stop HLS streaming for a stream
 *
 * @param stream_name Name of the stream to stop streaming
 * @return 0 if successful, -1 otherwise
 */
int go2rtc_integration_stop_hls(const char *stream_name);

/**
 * @brief Check if a stream is using go2rtc for recording
 *
 * @param stream_name Name of the stream to check
 * @return true if using go2rtc, false otherwise
 */
bool go2rtc_integration_is_using_go2rtc_for_recording(const char *stream_name);

/**
 * @brief Check if a stream is using go2rtc for HLS streaming
 *
 * @param stream_name Name of the stream to check
 * @return true if using go2rtc, false otherwise
 */
bool go2rtc_integration_is_using_go2rtc_for_hls(const char *stream_name);

/**
 * @brief Register all existing streams with go2rtc
 *
 * @return true if successful, false otherwise
 */
bool go2rtc_integration_register_all_streams(void);

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
bool go2rtc_sync_streams_from_database(void);

/**
 * @brief Clean up the go2rtc integration module
 */
void go2rtc_integration_cleanup(void);

/**
 * @brief Check if go2rtc integration is initialized
 *
 * @return true if initialized, false otherwise
 */
bool go2rtc_integration_is_initialized(void);

/**
 * @brief Get the RTSP URL for a stream from go2rtc
 *
 * @param stream_name Name of the stream
 * @param url Buffer to store the URL
 * @param url_size Size of the URL buffer
 * @return true if successful, false otherwise
 */
bool go2rtc_get_rtsp_url(const char *stream_name, char *url, size_t url_size);

/**
 * @brief Get the HLS URL for a stream from go2rtc
 *
 * @param stream_name Name of the stream
 * @param url Buffer to store the URL
 * @param url_size Size of the URL buffer
 * @return true if successful, false otherwise
 */
bool go2rtc_integration_get_hls_url(const char *stream_name, char *url, size_t url_size);

/**
 * @brief Reload stream configuration with go2rtc
 *
 * This is the centralized function for updating go2rtc when stream configuration changes.
 * It handles unregistering the old stream and re-registering with new configuration.
 * This should be used instead of direct calls to go2rtc_stream_register/unregister
 * when responding to configuration changes.
 *
 * @param stream_name Name of the stream to reload
 * @param new_url New stream URL (can be NULL to use current config)
 * @param new_username New username (can be NULL to use current config)
 * @param new_password New password (can be NULL to use current config)
 * @param new_backchannel_enabled New backchannel setting (-1 to use current config)
 * @param new_protocol New protocol setting (-1 to use current config, 0=TCP, 1=UDP)
 * @param new_record_audio New audio recording setting (-1 to use current config, 0=disabled, 1=enabled)
 * @return true if successful, false otherwise
 */
bool go2rtc_integration_reload_stream_config(const char *stream_name,
                                             const char *new_url,
                                             const char *new_username,
                                             const char *new_password,
                                             int new_backchannel_enabled,
                                             int new_protocol,
                                             int new_record_audio);

/**
 * @brief Reload a stream's go2rtc registration from its current database/memory configuration
 *
 * This is a convenience function that reloads a stream by looking up its current configuration.
 * Use this when you know the config has been updated and just want go2rtc to pick up the changes.
 *
 * @param stream_name Name of the stream to reload
 * @return true if successful, false otherwise
 */
bool go2rtc_integration_reload_stream(const char *stream_name);

/**
 * @brief Unregister a stream from go2rtc
 *
 * @param stream_name Name of the stream to unregister
 * @return true if successful, false otherwise
 */
bool go2rtc_integration_unregister_stream(const char *stream_name);

/**
 * @brief Register a single stream with go2rtc (universal entry point for startup and add)
 *
 * This function handles:
 * - Checking if go2rtc is ready
 * - Looking up stream config from the stream manager if not provided
 * - Extracting credentials from URL if not in onvif fields
 * - Registering with go2rtc
 *
 * Use this for both initial startup registration and when adding new streams.
 * This is the single entry point for stream registration.
 *
 * @param stream_name Name of the stream to register
 * @return true if successful, false otherwise
 */
bool go2rtc_integration_register_stream(const char *stream_name);

/**
 * @brief Check if the unified health monitor is running
 *
 * @return true if the monitor is running, false otherwise
 */
bool go2rtc_integration_monitor_is_running(void);

/**
 * @brief Get the number of go2rtc process restarts
 *
 * @return Number of restarts since initialization
 */
int go2rtc_integration_get_restart_count(void);

/**
 * @brief Get the time of the last go2rtc process restart
 *
 * @return Time of last restart, or 0 if never restarted
 */
time_t go2rtc_integration_get_last_restart_time(void);

/**
 * @brief Manually trigger a health check
 *
 * @return true if go2rtc is healthy, false otherwise
 */
bool go2rtc_integration_check_health(void);

#endif /* GO2RTC_INTEGRATION_H */
