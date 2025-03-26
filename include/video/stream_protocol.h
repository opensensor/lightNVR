#ifndef STREAM_PROTOCOL_H
#define STREAM_PROTOCOL_H

#include <stdbool.h>
#include <libavformat/avformat.h>
#include "core/config.h"

// Stream protocol types are now defined in core/config.h as an enum

// Check if a URL is a multicast address
bool is_multicast_url(const char *url);

// Check if a URL is an ONVIF stream
bool is_onvif_stream(const char *url);

// Open input stream with appropriate options based on protocol
int open_input_stream(AVFormatContext **input_ctx, const char *url, int protocol);

// Find video stream index in the input context
int find_video_stream_index(AVFormatContext *input_ctx);

// Find audio stream index in the input context
int find_audio_stream_index(AVFormatContext *input_ctx);

#endif // STREAM_PROTOCOL_H
