/**
 * @file go2rtc_api.h
 * @brief API client for interacting with go2rtc's HTTP API
 */

#ifndef GO2RTC_API_H
#define GO2RTC_API_H

#include <stdbool.h>
#include <stddef.h>

/**
 * @brief Initialize the go2rtc API client
 * 
 * @param api_host Host address of the go2rtc API (e.g., "localhost")
 * @param api_port Port of the go2rtc API
 * @return true if initialization was successful, false otherwise
 */
bool go2rtc_api_init(const char *api_host, int api_port);

/**
 * @brief Add a stream to go2rtc
 *
 * @param stream_id Unique identifier for the stream
 * @param stream_url URL of the stream source (e.g., RTSP URL)
 * @return true if stream was added successfully, false otherwise
 */
bool go2rtc_api_add_stream(const char *stream_id, const char *stream_url);

/**
 * @brief Add a stream to go2rtc with multiple sources
 *
 * This allows registering a stream with both a primary source and an FFmpeg
 * transcoding source for audio conversion (e.g., PCM to AAC).
 *
 * @param stream_id Unique identifier for the stream
 * @param sources Array of source URLs
 * @param num_sources Number of sources in the array
 * @return true if stream was added successfully, false otherwise
 */
bool go2rtc_api_add_stream_multi(const char *stream_id, const char **sources, int num_sources);

/**
 * @brief Remove a stream from go2rtc
 * 
 * @param stream_id Identifier of the stream to remove
 * @return true if stream was removed successfully, false otherwise
 */
bool go2rtc_api_remove_stream(const char *stream_id);

/**
 * @brief Check if a stream exists in go2rtc
 * 
 * @param stream_id Identifier of the stream to check
 * @return true if stream exists, false otherwise
 */
bool go2rtc_api_stream_exists(const char *stream_id);

/**
 * @brief Get the WebRTC URL for a stream
 * 
 * @param stream_id Identifier of the stream
 * @param buffer Buffer to store the URL
 * @param buffer_size Size of the buffer
 * @return true if URL was retrieved successfully, false otherwise
 */
bool go2rtc_api_get_webrtc_url(const char *stream_id, char *buffer, size_t buffer_size);

/**
 * @brief Update go2rtc configuration with current streams
 *
 * @return true if configuration was updated successfully false otherwise
 */
bool go2rtc_api_update_config(void);

/**
 * @brief Get go2rtc server information including RTSP port
 * 
 * @param rtsp_port Pointer to store the RTSP port (can be NULL)
 * @return true if information was retrieved successfully, false otherwise
 */
bool go2rtc_api_get_server_info(int *rtsp_port);

/**
 * @brief Clean up resources used by the go2rtc API client
 */
void go2rtc_api_cleanup(void);

#endif /* GO2RTC_API_H */
