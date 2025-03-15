#ifndef DETECTION_INTEGRATION_H
#define DETECTION_INTEGRATION_H

#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>

/**
 * Process a decoded frame for detection
 * This function should be called from the HLS streaming code with already decoded frames
 * 
 * @param stream_name The name of the stream
 * @param frame The decoded AVFrame
 * @param detection_interval How often to process frames (e.g., every 10 frames)
 * @return 0 on success, -1 on error
 */
int process_decoded_frame_for_detection(const char *stream_name, AVFrame *frame, int detection_interval);

/**
 * Process a video packet for detection
 * This function is kept for backward compatibility but now delegates to process_decoded_frame_for_detection
 * when a frame is available
 * 
 * @param stream_name The name of the stream
 * @param pkt The AVPacket to process
 * @param stream The AVStream the packet belongs to
 * @param detection_interval How often to process frames (e.g., every 10 frames)
 * @return 0 on success, -1 on error
 */
int process_packet_for_detection(const char *stream_name, AVPacket *pkt, AVStream *stream, int detection_interval);

#endif /* DETECTION_INTEGRATION_H */
