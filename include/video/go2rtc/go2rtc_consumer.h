/**
 * @file go2rtc_consumer.h
 * @brief Module for using go2rtc as a consumer for recording and HLS streaming
 */

#ifndef GO2RTC_CONSUMER_H
#define GO2RTC_CONSUMER_H

#include <stdbool.h>
#include <stddef.h>

/**
 * @brief Initialize the go2rtc consumer module
 * 
 * @return true if initialization was successful, false otherwise
 */
bool go2rtc_consumer_init(void);

/**
 * @brief Start recording for a stream using go2rtc
 * 
 * @param stream_id Unique identifier for the stream
 * @param output_path Path where recordings should be stored
 * @param segment_duration Duration of each segment in seconds
 * @return true if recording was started successfully, false otherwise
 */
bool go2rtc_consumer_start_recording(const char *stream_id, const char *output_path, int segment_duration);

/**
 * @brief Stop recording for a stream
 * 
 * @param stream_id Identifier of the stream to stop recording
 * @return true if recording was stopped successfully, false otherwise
 */
bool go2rtc_consumer_stop_recording(const char *stream_id);

/**
 * @brief Start HLS streaming for a stream using go2rtc
 * 
 * @param stream_id Unique identifier for the stream
 * @param output_path Path where HLS segments should be stored
 * @param segment_duration Duration of each segment in seconds
 * @return true if HLS streaming was started successfully, false otherwise
 */
bool go2rtc_consumer_start_hls(const char *stream_id, const char *output_path, int segment_duration);

/**
 * @brief Stop HLS streaming for a stream
 * 
 * @param stream_id Identifier of the stream to stop HLS streaming
 * @return true if HLS streaming was stopped successfully, false otherwise
 */
bool go2rtc_consumer_stop_hls(const char *stream_id);

/**
 * @brief Check if recording is active for a stream
 * 
 * @param stream_id Identifier of the stream to check
 * @return true if recording is active, false otherwise
 */
bool go2rtc_consumer_is_recording(const char *stream_id);

/**
 * @brief Check if HLS streaming is active for a stream
 * 
 * @param stream_id Identifier of the stream to check
 * @return true if HLS streaming is active, false otherwise
 */
bool go2rtc_consumer_is_hls_active(const char *stream_id);

/**
 * @brief Clean up resources used by the go2rtc consumer module
 */
void go2rtc_consumer_cleanup(void);

#endif /* GO2RTC_CONSUMER_H */
