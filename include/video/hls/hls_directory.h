#ifndef HLS_DIRECTORY_H
#define HLS_DIRECTORY_H

/**
 * Clean up HLS directories during shutdown
 * This function removes old HLS segments and playlist files
 */
void cleanup_hls_directories(void);

/**
 * Ensure the HLS output directory exists and is writable
 * 
 * @param output_dir The directory to check/create
 * @param stream_name The name of the stream (for logging)
 * @return 0 on success, non-zero on failure
 */
int ensure_hls_directory(const char *output_dir, const char *stream_name);

/**
 * Clear HLS segments for a specific stream
 * This is used when a stream's URL is changed to ensure the player sees the new stream
 * 
 * @param stream_name The name of the stream
 * @return 0 on success, non-zero on failure
 */
int clear_stream_hls_segments(const char *stream_name);

#endif /* HLS_DIRECTORY_H */
