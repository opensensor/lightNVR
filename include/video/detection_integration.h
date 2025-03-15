#ifndef DETECTION_INTEGRATION_H
#define DETECTION_INTEGRATION_H

#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>

/**
 * Process a video packet for detection
 * This function should be called from the HLS streaming code
 * 
 * @param stream_name The name of the stream
 * @param pkt The AVPacket to process
 * @param stream The AVStream the packet belongs to
 * @param detection_interval How often to process frames (e.g., every 10 frames)
 * @return 0 on success, -1 on error
 */
int process_packet_for_detection(const char *stream_name, AVPacket *pkt, AVStream *stream, int detection_interval);

#endif /* DETECTION_INTEGRATION_H */
