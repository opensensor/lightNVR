#ifndef PACKET_PROCESSOR_H
#define PACKET_PROCESSOR_H

#include <libavformat/avformat.h>

// Process a video packet for either HLS streaming or MP4 recording
// @deprecated Use process_packet_with_state or process_video_packet_adapter instead
int process_video_packet(const AVPacket *pkt, const AVStream *input_stream, 
                         void *writer, int writer_type, const char *stream_name) __attribute__((deprecated("Use process_packet_with_state or process_video_packet_adapter instead")));

#endif // PACKET_PROCESSOR_H
