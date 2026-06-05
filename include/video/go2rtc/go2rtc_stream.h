/**
 * @file go2rtc_stream.h
 * @brief Module for integrating go2rtc with the existing stream system
 */

#ifndef GO2RTC_STREAM_H
#define GO2RTC_STREAM_H

#include <stdbool.h>
#include <stddef.h>
#include "core/config.h"  // For stream_protocol_t

/**
 * @brief Initialize the go2rtc stream integration module
 * 
 * @param binary_path Path to the go2rtc binary
 * @param config_dir Directory to store go2rtc configuration files
 * @param api_port Port for the go2rtc HTTP API
 * @return true if initialization was successful, false otherwise
 */
bool go2rtc_stream_init(const char *binary_path, const char *config_dir, int api_port);

/**
 * @brief Register a stream with go2rtc
 *
 * @param stream_id Unique identifier for the stream
 * @param stream_url URL of the stream source (e.g., RTSP URL)
 * @param username Optional username for authentication (can be NULL)
 * @param password Optional password for authentication (can be NULL)
 * @param backchannel_enabled Whether to enable two-way audio (backchannel) support
 * @param protocol Stream protocol (TCP or UDP) - used to set transport parameter for go2rtc
 * @param record_audio Whether audio recording is enabled - if true, adds an FFmpeg
 *              `#audio=aac#audio=opus` source (AAC for recording/HLS, OPUS for WebRTC)
 * @param codec Detected or user-declared source video codec (e.g. "h264", "hevc").
 *              May be NULL or empty when unknown. Used to decide whether to add
 *              an `ffmpeg:<id>#video=h264` fallback source (with `#hardware`
 *              appended only when hw_accel_enabled is set) so that
 *              WebRTC clients (which only accept H.264/VP8/VP9/AV1) can consume
 *              H.265 streams via on-demand transcoding. The fallback is added
 *              for anything that isn't explicitly "h264" — unknown codecs are
 *              treated as "might be H.265" to be safe; once the detection
 *              thread learns the real codec it re-registers via
 *              go2rtc_integration_reregister_stream().
 * @return true if stream was registered successfully, false otherwise
 */
bool go2rtc_stream_register(const char *stream_id, const char *stream_url,
                           const char *username, const char *password,
                           bool backchannel_enabled, stream_protocol_t protocol,
                           bool record_audio, const char *codec);

/**
 * @brief Unregister a stream from go2rtc
 * 
 * @param stream_id Identifier of the stream to unregister
 * @return true if stream was unregistered successfully, false otherwise
 */
bool go2rtc_stream_unregister(const char *stream_id);

/**
 * @brief Get the WebRTC URL for a stream
 * 
 * @param stream_id Identifier of the stream
 * @param buffer Buffer to store the URL
 * @param buffer_size Size of the buffer
 * @return true if URL was retrieved successfully, false otherwise
 */
bool go2rtc_stream_get_webrtc_url(const char *stream_id, char *buffer, size_t buffer_size);

/**
 * @brief Get the RTSP URL for a stream
 * 
 * @param stream_id Identifier of the stream
 * @param buffer Buffer to store the URL
 * @param buffer_size Size of the buffer
 * @return true if URL was retrieved successfully, false otherwise
 */
bool go2rtc_stream_get_rtsp_url(const char *stream_id, char *buffer, size_t buffer_size);

/**
 * @brief Check if the go2rtc stream module has been initialized
 *
 * @return true if initialized, false otherwise
 */
bool go2rtc_stream_is_initialized(void);

/**
 * @brief Check if go2rtc is running and ready
 *
 * @return true if go2rtc is running and ready, false otherwise
 */
bool go2rtc_stream_is_ready(void);

/**
 * @brief Get the configured API port
 *
 * @return The API port number, or 0 if not initialized
 */
int go2rtc_stream_get_api_port(void);

/**
 * @brief Start the go2rtc service
 *
 * @return true if service was started successfully, false otherwise
 */
bool go2rtc_stream_start_service(void);

/**
 * @brief Stop the go2rtc service
 * 
 * @return true if service was stopped successfully, false otherwise
 */
bool go2rtc_stream_stop_service(void);

/**
 * @brief Invalidate the cached result of go2rtc_stream_is_ready()
 *
 * Call this after stopping or restarting the go2rtc process to force the next
 * readiness check to perform a real HTTP probe instead of returning a stale
 * cached value.
 */
void go2rtc_stream_invalidate_ready_cache(void);

/**
 * @brief Clean up resources used by the go2rtc stream integration module
 */
void go2rtc_stream_cleanup(void);

#endif /* GO2RTC_STREAM_H */
