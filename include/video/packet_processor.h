#ifndef PACKET_PROCESSOR_H
#define PACKET_PROCESSOR_H

#include <libavformat/avformat.h>
#include "core/config.h"

// Use MAX_STREAMS from config.h (16)

// Maximum stream name length (should match the larger of the two definitions)
#define MAX_STREAM_NAME 256

// Process a video packet for either HLS streaming or MP4 recording
int process_video_packet(const AVPacket *pkt, const AVStream *input_stream, 
                         void *writer, int writer_type, const char *stream_name);

#endif // PACKET_PROCESSOR_H
