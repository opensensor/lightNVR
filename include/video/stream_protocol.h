#ifndef STREAM_PROTOCOL_H
#define STREAM_PROTOCOL_H

#include <stdbool.h>
#include <libavformat/avformat.h>

// Stream protocol types
#define STREAM_PROTOCOL_TCP 0
#define STREAM_PROTOCOL_UDP 1

// Check if a URL is a multicast address
bool is_multicast_url(const char *url);

// Open input stream with appropriate options based on protocol
int open_input_stream(AVFormatContext **input_ctx, const char *url, int protocol);

// Find video stream index in the input context
int find_video_stream_index(AVFormatContext *input_ctx);

#endif // STREAM_PROTOCOL_H
