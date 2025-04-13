#include "video/stream_transcoding.h"
#include "video/ffmpeg_utils.h"
#include "video/thread_utils.h"
#include "video/stream_protocol.h"
#include "video/timestamp_manager.h"
#include "video/packet_processor.h"
#include "video/ffmpeg_leak_detector.h"
#include "core/logger.h"

/**
 * Initialize transcoding backend
 * Sets up FFmpeg and timestamp trackers
 */
void init_transcoding_backend(void) {
    // Initialize FFmpeg
    init_ffmpeg();

    // Initialize timestamp trackers
    init_timestamp_trackers();

    log_info("Transcoding backend initialized");
}

/**
 * Cleanup transcoding backend
 * Cleans up FFmpeg and timestamp trackers
 */
void cleanup_transcoding_backend(void) {
    // Cleanup timestamp trackers
    cleanup_timestamp_trackers();

    // Cleanup FFmpeg
    cleanup_ffmpeg();

    log_info("Transcoding backend cleaned up");
}
