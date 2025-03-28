#ifndef DETECTION_FRAME_PROCESSING_H
#define DETECTION_FRAME_PROCESSING_H

#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <stdbool.h>

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

#endif /* DETECTION_FRAME_PROCESSING_H */
