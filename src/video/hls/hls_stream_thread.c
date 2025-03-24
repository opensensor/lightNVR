#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <pthread.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <fcntl.h>
#include <errno.h>
#include <time.h>
#include <stdbool.h>
#include <sys/time.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <pthread.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <fcntl.h>
#include <errno.h>
#include <time.h>
#include <stdbool.h>

#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <libavutil/avutil.h>
#include <libavutil/time.h>

#include "core/logger.h"
#include "core/config.h"
#include "video/stream_manager.h"
#include "video/stream_state.h"
#include "video/streams.h"
#include "video/hls_writer.h"
#include "video/hls_streaming.h"
#include "video/stream_transcoding.h"
#include "video/stream_reader.h"
#include "video/detection_stream.h"
#include "video/detection_integration.h"
#include "video/detection_thread_pool.h"
#include <sys/sysinfo.h>
#include "video/stream_packet_processor.h"
#include "video/thread_utils.h"
#include "video/timestamp_manager.h"
#include "video/hls/hls_context.h"
#include "video/hls/hls_stream_thread.h"
#include "video/hls/hls_directory.h"
#include "video/mp4_recording.h"

// Forward declaration of the detection function from hls_writer.c
extern void process_packet_for_detection(const char *stream_name, const AVPacket *pkt, const AVCodecParameters *codec_params);

// REMOVED: hls_packet_callback function is no longer needed since we're using the single thread approach
/**
 * HLS streaming thread function for a single stream
 * Balanced implementation that maintains low latency while ensuring stability
 */
void *hls_stream_thread(void *arg) {
    hls_stream_ctx_t *ctx = (hls_stream_ctx_t *)arg;
    AVFormatContext *input_ctx = NULL;
    AVPacket *pkt = NULL;
    int video_stream_idx = -1;
    int ret;
    time_t start_time = time(NULL);  // Record when we started

    // CRITICAL FIX: Add extra validation for context
    if (!ctx) {
        log_error("NULL context passed to HLS streaming thread");
        return NULL;
    }

    // CRITICAL FIX: Create a local copy of the stream name for thread safety
    char stream_name[MAX_STREAM_NAME];
    strncpy(stream_name, ctx->config.name, MAX_STREAM_NAME - 1);
    stream_name[MAX_STREAM_NAME - 1] = '\0';

    // Get the stream state manager
    stream_state_manager_t *state = get_stream_state_by_name(stream_name);
    if (!state) {
        log_error("Could not find stream state for %s", stream_name);
        ctx->running = 0;
        return NULL;
    }

    log_info("Starting HLS streaming thread for stream %s", stream_name);

    // CRITICAL FIX: Check if we're still running before proceeding
    if (!ctx->running) {
        log_warn("HLS streaming thread for %s started but already marked as not running", stream_name);
        return NULL;
    }

    // Verify output directory exists and is writable
    if (ensure_hls_directory(ctx->output_path, stream_name) != 0) {
        log_error("Failed to ensure HLS output directory: %s", ctx->output_path);
        ctx->running = 0;
        return NULL;
    }

    // CRITICAL FIX: Check if we're still running after directory creation
    if (!ctx->running) {
        log_info("HLS streaming thread for %s stopping after directory creation", stream_name);
        return NULL;
    }

    // Create HLS writer - adding the segment_duration parameter
    int segment_duration = ctx->config.segment_duration > 0 ?
                          ctx->config.segment_duration : 1;

    ctx->hls_writer = hls_writer_create(ctx->output_path, stream_name, segment_duration);
    if (!ctx->hls_writer) {
        log_error("Failed to create HLS writer for %s", stream_name);
        ctx->running = 0;
        return NULL;
    }

    // CRITICAL FIX: Check if we're still running after HLS writer creation
    if (!ctx->running) {
        log_info("HLS streaming thread for %s stopping after HLS writer creation", stream_name);
        if (ctx->hls_writer) {
            hls_writer_close(ctx->hls_writer);
            ctx->hls_writer = NULL;
        }
        return NULL;
    }

    // Get stream configuration
    stream_handle_t stream = get_stream_by_name(stream_name);
    if (!stream) {
        log_error("Stream %s not found", stream_name);

        if (ctx->hls_writer) {
            hls_writer_close(ctx->hls_writer);
            ctx->hls_writer = NULL;
        }

        ctx->running = 0;
        return NULL;
    }

    stream_config_t config;
    if (get_stream_config(stream, &config) != 0) {
        log_error("Failed to get config for stream %s", stream_name);

        if (ctx->hls_writer) {
            hls_writer_close(ctx->hls_writer);
            ctx->hls_writer = NULL;
        }

        ctx->running = 0;
        return NULL;
    }

    // Use the stream_protocol.h functions to open the input stream with appropriate options
    ret = open_input_stream(&input_ctx, config.url, config.protocol);
    if (ret < 0) {
        log_error("Could not open input stream for %s", stream_name);

        if (ctx->hls_writer) {
            hls_writer_close(ctx->hls_writer);
            ctx->hls_writer = NULL;
        }

        ctx->running = 0;
        return NULL;
    }

    // Find video stream
    video_stream_idx = find_video_stream_index(input_ctx);
    if (video_stream_idx == -1) {
        log_error("No video stream found in %s", config.url);

        avformat_close_input(&input_ctx);

        if (ctx->hls_writer) {
            hls_writer_close(ctx->hls_writer);
            ctx->hls_writer = NULL;
        }

        ctx->running = 0;
        return NULL;
    }

    // Initialize packet
    pkt = av_packet_alloc();
    if (!pkt) {
        log_error("Failed to allocate packet");

        avformat_close_input(&input_ctx);

        if (ctx->hls_writer) {
            hls_writer_close(ctx->hls_writer);
            ctx->hls_writer = NULL;
        }

        ctx->running = 0;
        return NULL;
    }

    // Track time of last flush to avoid excessive I/O operations
    int64_t last_flush_time = av_gettime();
    // We'll only flush every 500ms (or on key frames)
    const int64_t flush_interval = 500000; // 500ms in microseconds

    // Main packet reading loop
    while (ctx->running) {
        // Check if the stream state indicates we should stop
        if (state) {
            if (is_stream_state_stopping(state)) {
                log_info("HLS streaming thread for %s stopping due to stream state STOPPING", stream_name);
                ctx->running = 0;
                break;
            }

            if (!are_stream_callbacks_enabled(state)) {
                log_info("HLS streaming thread for %s stopping due to callbacks disabled", stream_name);
                ctx->running = 0;
                break;
            }
        }

        // Check if we should exit before potentially blocking on av_read_frame
        if (!ctx->running) {
            log_info("HLS streaming thread for %s detected shutdown before read", stream_name);
            break;
        }
        
        // Periodically check if we should exit
        // This is a safer approach than using AVFMT_FLAG_NONBLOCK which can cause issues
        struct timespec ts = {0, 10000000}; // 10ms
        nanosleep(&ts, NULL);
        
        ret = av_read_frame(input_ctx, pkt);

        if (ret < 0) {
            if (ret == AVERROR_EOF || ret == AVERROR(EAGAIN)) {
                // End of stream or resource temporarily unavailable
                // Try to reconnect after a short delay
                av_packet_unref(pkt);
                log_warn("Stream %s disconnected, attempting to reconnect...", stream_name);

                av_usleep(1000000);  // 1 second delay for more reliable reconnection

                // Close and reopen input
                avformat_close_input(&input_ctx);

                // Use the stream_protocol.h function to reopen the input stream
                ret = open_input_stream(&input_ctx, config.url, config.protocol);
                if (ret < 0) {
                    log_error("Could not reconnect to input stream for %s", stream_name);
                    continue;  // Keep trying
                }

                // Find video stream again
                video_stream_idx = find_video_stream_index(input_ctx);
                if (video_stream_idx == -1) {
                    log_error("No video stream found after reconnect for %s", stream_name);
                    continue;  // Keep trying
                }

                continue;
            } else {
                char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
                av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
                log_error("Error reading frame: %s", error_buf);
                break;
            }
        }

        // Process video packets
        if (pkt->stream_index == video_stream_idx) {
            // Check if this is a key frame
            bool is_key_frame = (pkt->flags & AV_PKT_FLAG_KEY) != 0;

            // Write packet to HLS writer
            ret = hls_writer_write_packet(ctx->hls_writer, pkt, input_ctx->streams[video_stream_idx]);

            // BALANCED APPROACH: Only flush on key frames or periodically
            int64_t current_time = av_gettime();
            bool should_flush = is_key_frame || (current_time - last_flush_time > flush_interval);

            if (ret >= 0 && should_flush && ctx->hls_writer &&
                ctx->hls_writer->output_ctx && ctx->hls_writer->output_ctx->pb) {
                // Flush only when necessary to reduce I/O load
                avio_flush(ctx->hls_writer->output_ctx->pb);
                last_flush_time = current_time;

                if (is_key_frame) {
                    log_debug("Flushed on key frame for stream %s", stream_name);
                }
            } else if (ret < 0 && is_key_frame) {
                // Only log errors for key frames to reduce log spam
                char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
                av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
                log_error("Failed to write packet to HLS for stream %s: %s", stream_name, error_buf);
            }

            // Pre-buffer handling for MP4 recordings
            extern void add_packet_to_prebuffer(const char *stream_name, const AVPacket *pkt, const AVStream *stream);
            add_packet_to_prebuffer(stream_name, pkt, input_ctx->streams[video_stream_idx]);

            // Check if there's an MP4 writer registered for active recordings
            mp4_writer_t *mp4_writer = get_mp4_writer_for_stream(stream_name);
            if (mp4_writer) {
                // Write the packet to the MP4 writer with proper error handling
                ret = mp4_writer_write_packet(mp4_writer, pkt, input_ctx->streams[video_stream_idx]);
                if (ret < 0) {
                    // Only log errors for key frames to reduce log spam
                    if (is_key_frame) {
                        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
                        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
                        log_error("Failed to write packet to MP4 for stream %s: %s", stream_name, error_buf);
                    }
                }
            }

            // Process packet for detection only on key frames to reduce CPU load
            // Use the thread pool for non-blocking detection
            if (is_key_frame && is_detection_stream_reader_running(stream_name)) {
                // Check if we're on a memory-constrained device
                extern config_t g_config;
                bool is_memory_constrained = g_config.memory_constrained || (sysconf(_SC_PHYS_PAGES) * sysconf(_SC_PAGE_SIZE) < 1024*1024*1024); // Less than 1GB RAM
                
                // Get current time to check detection interval
                static time_t last_detection_time = 0;
                time_t current_time = time(NULL);
                int detection_interval = get_detection_interval(stream_name);
                
                // Only run detection if enough time has passed since the last detection
                if (last_detection_time == 0 || (current_time - last_detection_time) >= detection_interval) {
                    // Submit the detection task to the thread pool
                    if (is_memory_constrained) {
                        // On memory-constrained devices, only submit if the thread pool is not busy
                        if (!is_detection_thread_pool_busy()) {
                            log_info("Submitting detection task for stream %s to thread pool", stream_name);
                            if (submit_detection_task(stream_name, pkt, input_ctx->streams[video_stream_idx]->codecpar) == 0) {
                                last_detection_time = current_time;
                            }
                        } else {
                            log_debug("Skipping detection on memory-constrained device - thread pool busy");
                        }
                    } else {
                        // On regular devices, always submit the task
                        log_info("Submitting detection task for stream %s to thread pool", stream_name);
                        if (submit_detection_task(stream_name, pkt, input_ctx->streams[video_stream_idx]->codecpar) == 0) {
                            last_detection_time = current_time;
                        }
                    }
                }
            }
        }

        av_packet_unref(pkt);
    }

    // Cleanup resources
    if (pkt) {
        av_packet_free(&pkt);
    }

    if (input_ctx) {
        avformat_close_input(&input_ctx);
    }

    // When done, close writer - use a local copy to avoid double free
    hls_writer_t *writer_to_close = ctx->hls_writer;
    ctx->hls_writer = NULL;  // Clear the pointer first to prevent double close
    
    if (writer_to_close) {
        hls_writer_close(writer_to_close);
    }

    log_info("HLS streaming thread for stream %s exited", stream_name);
    return NULL;
}
