#ifndef STREAM_TRANSCODING_H
#define STREAM_TRANSCODING_H

#include "video/ffmpeg_utils.h"
#include "video/thread_utils.h"
#include "video/stream_protocol.h"
#include "video/timestamp_manager.h"
#include "video/packet_processor.h"

/**
 * Initialize transcoding backend
 * Sets up FFmpeg and timestamp trackers
 */
void init_transcoding_backend(void);

/**
 * Cleanup transcoding backend
 * Cleans up FFmpeg and timestamp trackers
 */
void cleanup_transcoding_backend(void);

#endif /* STREAM_TRANSCODING_H */
