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

#endif /* HLS_DIRECTORY_H */
