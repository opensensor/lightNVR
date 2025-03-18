#ifndef HLS_API_H
#define HLS_API_H

/**
 * Start HLS streaming for a stream
 * 
 * @param stream_name Name of the stream to stream
 * @return 0 on success, non-zero on failure
 */
int start_hls_stream(const char *stream_name);

/**
 * Stop HLS streaming for a stream
 * 
 * @param stream_name Name of the stream to stop streaming
 * @return 0 on success, non-zero on failure
 */
int stop_hls_stream(const char *stream_name);

#endif /* HLS_API_H */
