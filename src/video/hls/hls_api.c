#include "core/logger.h"
#include "video/hls/hls_api.h"

#include "video/hls/hls_unified_thread.h"

/**
 * Start HLS streaming for a stream with improved reliability
 * This is now a wrapper around the unified thread implementation
 */
int start_hls_stream(const char *stream_name) {
    log_info("Starting HLS stream for %s using unified thread architecture", stream_name);
    return start_hls_unified_stream(stream_name);
}

/**
 * Force restart of HLS streaming for a stream
 * This is used when a stream's URL is changed to ensure the stream thread is restarted
 */
int restart_hls_stream(const char *stream_name) {
    log_info("Restarting HLS stream for %s using unified thread architecture", stream_name);
    return restart_hls_unified_stream(stream_name);
}

/**
 * Stop HLS streaming for a stream
 */
int stop_hls_stream(const char *stream_name) {
    log_info("Stopping HLS stream for %s using unified thread architecture", stream_name);
    return stop_hls_unified_stream(stream_name);
}
