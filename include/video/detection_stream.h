#ifndef DETECTION_STREAM_H
#define DETECTION_STREAM_H

/**
 * Initialize detection stream system
 */
void init_detection_stream_system(void);

/**
 * Shutdown detection stream system
 */
void shutdown_detection_stream_system(void);

/**
 * Start a detection stream reader for a stream
 * 
 * @param stream_name The name of the stream
 * @param detection_interval How often to process frames (e.g., every 10 frames)
 * @return 0 on success, -1 on error
 */
int start_detection_stream_reader(const char *stream_name, int detection_interval);

/**
 * Stop a detection stream reader for a stream
 * 
 * @param stream_name The name of the stream
 * @return 0 on success, -1 on error
 */
int stop_detection_stream_reader(const char *stream_name);

/**
 * Check if a detection stream reader is running for a stream
 * 
 * @param stream_name The name of the stream
 * @return 1 if running, 0 if not, -1 on error
 */
int is_detection_stream_reader_running(const char *stream_name);

/**
 * Get the detection interval for a stream
 * 
 * @param stream_name The name of the stream
 * @return The detection interval in seconds, or 15 (default) if not found
 */
int get_detection_interval(const char *stream_name);

/**
 * Print status of all detection stream readers
 * This is useful for debugging detection issues
 */
void print_detection_stream_status(void);

#endif /* DETECTION_STREAM_H */
