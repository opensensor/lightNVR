#ifndef HLS_STREAM_THREAD_H
#define HLS_STREAM_THREAD_H

#include <pthread.h>
#include "../hls_streaming.h"

/**
 * HLS streaming thread function for a single stream
 * This function manages hls_writer threads rather than doing the HLS writing itself
 * 
 * @param arg Pointer to the HLS stream context
 * @return NULL
 */
void *hls_stream_thread(void *arg);

#endif /* HLS_STREAM_THREAD_H */
