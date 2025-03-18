#ifndef HLS_STREAM_THREAD_H
#define HLS_STREAM_THREAD_H

#include <pthread.h>
#include <libavformat/avformat.h>
#include "../hls_streaming.h"

/**
 * HLS streaming thread function for a single stream
 * 
 * @param arg Pointer to the HLS stream context
 * @return NULL
 */
void *hls_stream_thread(void *arg);

/**
 * HLS packet processing callback function
 * 
 * @param pkt The packet to process
 * @param stream The stream the packet belongs to
 * @param user_data User data (HLS stream context)
 * @return 0 on success, non-zero on failure
 */
int hls_packet_callback(const AVPacket *pkt, const AVStream *stream, void *user_data);

#endif /* HLS_STREAM_THREAD_H */
