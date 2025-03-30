/**
 * @file go2rtc_integration.h
 * @brief Integration of go2rtc with existing recording and HLS systems
 */

#ifndef GO2RTC_INTEGRATION_H
#define GO2RTC_INTEGRATION_H

#include <stdbool.h>
#include <stddef.h>

/**
 * @brief Initialize the go2rtc integration module
 * 
 * @return true if initialization was successful, false otherwise
 */
bool go2rtc_integration_init(void);

/**
 * @brief Start recording for a stream using go2rtc if available
 * 
 * @param stream_name Name of the stream to record
 * @return 0 if successful, -1 otherwise
 */
int go2rtc_integration_start_recording(const char *stream_name);

/**
 * @brief Stop recording for a stream
 * 
 * @param stream_name Name of the stream to stop recording
 * @return 0 if successful, -1 otherwise
 */
int go2rtc_integration_stop_recording(const char *stream_name);

/**
 * @brief Start HLS streaming for a stream using go2rtc if available
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
 * @return true if using go2rtc for recording, false otherwise
 */
bool go2rtc_integration_is_using_go2rtc_for_recording(const char *stream_name);

/**
 * @brief Check if a stream is using go2rtc for HLS streaming
 * 
 * @param stream_name Name of the stream to check
 * @return true if using go2rtc for HLS streaming, false otherwise
 */
bool go2rtc_integration_is_using_go2rtc_for_hls(const char *stream_name);

/**
 * @brief Register all existing streams with go2rtc
 * 
 * @return true if successful, false otherwise
 */
bool go2rtc_integration_register_all_streams(void);

/**
 * @brief Check if the go2rtc integration module is initialized
 * 
 * @return true if initialized, false otherwise
 */
bool go2rtc_integration_is_initialized(void);

/**
 * @brief Get the go2rtc HLS URL for a stream
 * 
 * @param stream_name Name of the stream
 * @param buffer Buffer to store the URL
 * @param buffer_size Size of the buffer
 * @return true if successful, false otherwise
 */
bool go2rtc_integration_get_hls_url(const char *stream_name, char *buffer, size_t buffer_size);

/**
 * @brief Get the go2rtc RTSP URL for a stream
 * 
 * @param stream_name Name of the stream
 * @param url Buffer to store the URL
 * @param url_size Size of the URL buffer
 * @return true if successful, false otherwise
 */
bool go2rtc_get_rtsp_url(const char *stream_name, char *url, size_t url_size);

/**
 * @brief Clean up resources used by the go2rtc integration module
 */
void go2rtc_integration_cleanup(void);

#endif /* GO2RTC_INTEGRATION_H */
