#ifndef DETECTION_STREAM_THREAD_H
#define DETECTION_STREAM_THREAD_H

#include <stdbool.h>
#include "video/packet_processor.h" // For MAX_STREAM_NAME definition

/**
 * Initialize the stream detection system
 * 
 * @return 0 on success, non-zero on failure
 */
int init_stream_detection_system(void);

/**
 * Shutdown the stream detection system
 */
void shutdown_stream_detection_system(void);

/**
 * Start a detection thread for a stream
 * 
 * @param stream_name The name of the stream
 * @param model_path The path to the detection model
 * @param threshold The detection threshold
 * @param detection_interval The detection interval in seconds
 * @param hls_dir The directory containing HLS segments
 * @return 0 on success, non-zero on failure
 */
int start_stream_detection_thread(const char *stream_name, const char *model_path, 
                                 float threshold, int detection_interval, const char *hls_dir);

/**
 * Stop a detection thread for a stream
 * 
 * @param stream_name The name of the stream
 * @return 0 on success, non-zero on failure
 */
int stop_stream_detection_thread(const char *stream_name);

/**
 * Check if a detection thread is running for a stream
 * 
 * @param stream_name The name of the stream
 * @return true if a thread is running, false otherwise
 */
bool is_stream_detection_thread_running(const char *stream_name);

/**
 * Get the number of running detection threads
 * 
 * @return The number of running detection threads
 */
int get_running_stream_detection_threads(void);

/**
 * Get detailed status information about a detection thread
 * 
 * @param stream_name The name of the stream
 * @param has_thread Will be set to true if a thread is running for this stream
 * @param last_check_time Will be set to the last time segments were checked
 * @param last_detection_time Will be set to the last time detection was run
 * @return 0 on success, -1 on error
 */
int get_stream_detection_status(const char *stream_name, bool *has_thread, 
                               time_t *last_check_time, time_t *last_detection_time);

/**
 * Process a frame directly for detection
 * This function is called from the detection thread pool via process_decoded_frame_for_detection
 * 
 * @param stream_name The name of the stream
 * @param frame_data The frame data in RGB or grayscale format
 * @param width The width of the frame
 * @param height The height of the frame
 * @param channels The number of channels in the frame (3 for RGB, 1 for grayscale)
 * @param timestamp The timestamp of the frame
 * @return 0 on success, non-zero on failure
 */
int process_frame_for_stream_detection(const char *stream_name, const uint8_t *frame_data, 
                                      int width, int height, int channels, time_t timestamp);

#endif /* DETECTION_STREAM_THREAD_H */
