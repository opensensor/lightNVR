#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <dirent.h>
#include <sys/stat.h>
#include <pthread.h>
#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <libavutil/avutil.h>
#include <libavutil/time.h>

#include "core/logger.h"
#include "video/hls_writer.h"
#include "video/detection_integration.h"
#include "video/streams.h"

// Direct detection processing function - no thread needed
static void process_packet_for_detection(const char *stream_name, const AVPacket *pkt, const AVCodecParameters *codec_params) {
    // CRITICAL FIX: Add extra validation for all parameters
    if (!stream_name || !pkt || !codec_params) {
        log_error("Invalid parameters in process_packet_for_detection: stream_name=%p, pkt=%p, codec_params=%p",
                 (void*)stream_name, (void*)pkt, (void*)codec_params);
        return;
    }
    
    // Use a static flag to prevent recursive calls and potential stack overflow
    static int detection_in_progress = 0;
    
    // Skip if detection is already in progress to prevent recursive calls
    if (detection_in_progress) {
        return;
    }
    
    detection_in_progress = 1;
    
    // Use a try/catch style approach with goto for cleanup
    AVPacket *pkt_copy = NULL;
    AVCodec *codec = NULL;
    AVCodecContext *codec_ctx = NULL;
    AVFrame *frame = NULL;
    
    log_debug("Processing packet for detection directly for stream %s", stream_name);
    
    // Create a copy of the packet to avoid modifying the original
    pkt_copy = av_packet_alloc();
    if (!pkt_copy) {
        log_error("Failed to allocate packet copy for detection");
        goto cleanup;
    }
    
    if (av_packet_ref(pkt_copy, pkt) < 0) {
        log_error("Failed to reference packet for detection");
        goto cleanup;
    }
    
    // Find decoder
    codec = avcodec_find_decoder(codec_params->codec_id);
    if (!codec) {
        log_error("Failed to find decoder for stream %s", stream_name);
        goto cleanup;
    }
    
    // Create codec context
    codec_ctx = avcodec_alloc_context3(codec);
    if (!codec_ctx) {
        log_error("Failed to allocate codec context for stream %s", stream_name);
        goto cleanup;
    }
    
    // Copy codec parameters to codec context
    if (avcodec_parameters_to_context(codec_ctx, codec_params) < 0) {
        log_error("Failed to copy codec parameters to context for stream %s", stream_name);
        goto cleanup;
    }
    
    // Open codec
    if (avcodec_open2(codec_ctx, codec, NULL) < 0) {
        log_error("Failed to open codec for stream %s", stream_name);
        goto cleanup;
    }
    
    // Allocate frame
    frame = av_frame_alloc();
    if (!frame) {
        log_error("Failed to allocate frame for stream %s", stream_name);
        goto cleanup;
    }
    
    // Send packet to decoder
    if (avcodec_send_packet(codec_ctx, pkt_copy) < 0) {
        log_error("Failed to send packet to decoder for stream %s", stream_name);
        goto cleanup;
    }
    
    // Receive frame from decoder
    int ret = avcodec_receive_frame(codec_ctx, frame);
    if (ret >= 0) {
        // Process the decoded frame for detection
        // Use a smaller detection interval (15) to improve detection quality
        // while still maintaining reasonable performance
        int detection_interval = 15;
        log_debug("Sending decoded frame to detection integration for stream %s", stream_name);
        
        // CRITICAL FIX: Check if process_decoded_frame_for_detection is available
        if (process_decoded_frame_for_detection) {
            process_decoded_frame_for_detection(stream_name, frame, detection_interval);
        } else {
            log_error("process_decoded_frame_for_detection function is not available");
        }
    } else if (ret != AVERROR(EAGAIN) && ret != AVERROR_EOF) {
        log_error("Failed to receive frame from decoder for stream %s: %d", stream_name, ret);
    }
    
cleanup:
    // Cleanup resources in reverse order of allocation
    if (frame) {
        av_frame_free(&frame);
    }
    
    if (codec_ctx) {
        avcodec_free_context(&codec_ctx);
    }
    
    if (pkt_copy) {
        av_packet_unref(pkt_copy);
        av_packet_free(&pkt_copy);
    }
    
    detection_in_progress = 0;
}

/**
 * Clean up old HLS segments that are no longer in the playlist
 */
static void cleanup_old_segments(const char *output_dir, int max_segments) {
    DIR *dir;
    struct dirent *entry;
    struct stat st;
    char filepath[HLS_MAX_PATH_LENGTH];

    // Keep track of segments to delete
    typedef struct {
        char filename[256];
        time_t mtime;
    } segment_info_t;

    segment_info_t *segments = NULL;
    int segment_count = 0;

    // Open directory
    dir = opendir(output_dir);
    if (!dir) {
        log_error("Failed to open directory for cleanup: %s", output_dir);
        return;
    }

    // Count TS files first
    while ((entry = readdir(dir)) != NULL) {
        // Skip non-TS files
        if (strstr(entry->d_name, ".ts") == NULL) {
            continue;
        }

        segment_count++;
    }

    // If we don't have more than the max, no cleanup needed
    if (segment_count <= max_segments) {
        closedir(dir);
        return;
    }

    // Allocate array for segment info
    segments = (segment_info_t *)malloc(segment_count * sizeof(segment_info_t));
    if (!segments) {
        log_error("Failed to allocate memory for segment cleanup");
        closedir(dir);
        return;
    }

    // Rewind directory
    rewinddir(dir);

    // Collect segment info
    int i = 0;
    while ((entry = readdir(dir)) != NULL && i < segment_count) {
        // Skip non-TS files
        if (strstr(entry->d_name, ".ts") == NULL) {
            continue;
        }

        // Get file stats
        snprintf(filepath, sizeof(filepath), "%s/%s", output_dir, entry->d_name);
        if (stat(filepath, &st) == 0) {
            strncpy(segments[i].filename, entry->d_name, 255);
            segments[i].filename[255] = '\0';
            segments[i].mtime = st.st_mtime;
            i++;
        }
    }

    closedir(dir);

    // Sort segments by modification time (oldest first)
    // Simple bubble sort for now
    for (int j = 0; j < i - 1; j++) {
        for (int k = 0; k < i - j - 1; k++) {
            if (segments[k].mtime > segments[k + 1].mtime) {
                segment_info_t temp = segments[k];
                segments[k] = segments[k + 1];
                segments[k + 1] = temp;
            }
        }
    }

    // Delete oldest segments beyond our limit
    int to_delete = i - max_segments;
    for (int j = 0; j < to_delete; j++) {
        snprintf(filepath, sizeof(filepath), "%s/%s", output_dir, segments[j].filename);
        if (unlink(filepath) == 0) {
            log_debug("Deleted old HLS segment: %s", segments[j].filename);
        } else {
            log_warn("Failed to delete old HLS segment: %s", segments[j].filename);
        }
    }

    free(segments);
}

hls_writer_t *hls_writer_create(const char *output_dir, const char *stream_name, int segment_duration) {
    // Allocate writer structure
    hls_writer_t *writer = (hls_writer_t *)calloc(1, sizeof(hls_writer_t));
    if (!writer) {
        log_error("Failed to allocate HLS writer");
        return NULL;
    }

    // Copy output directory and stream name
    strncpy(writer->output_dir, output_dir, MAX_PATH_LENGTH - 1);
    strncpy(writer->stream_name, stream_name, MAX_STREAM_NAME - 1);
    writer->segment_duration = segment_duration;
    writer->last_cleanup_time = time(NULL);

    // Create output directory if it doesn't exist
    char mkdir_cmd[MAX_PATH_LENGTH + 10];
    snprintf(mkdir_cmd, sizeof(mkdir_cmd), "mkdir -p %s", output_dir);
    system(mkdir_cmd);

    // Set permissions
    snprintf(mkdir_cmd, sizeof(mkdir_cmd), "chmod -R 777 %s", output_dir);
    system(mkdir_cmd);

    // Initialize output format context for HLS
    char output_path[MAX_PATH_LENGTH];
    snprintf(output_path, MAX_PATH_LENGTH, "%s/index.m3u8", output_dir);

    // Allocate output format context
    int ret = avformat_alloc_output_context2(
        &writer->output_ctx, NULL, "hls", output_path);

    if (ret < 0) {
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Failed to allocate output context for HLS: %s", error_buf);
        free(writer);
        return NULL;
    }

    // Set HLS options - use more robust settings for better compatibility
    AVDictionary *options = NULL;
    char hls_time[16];
    snprintf(hls_time, sizeof(hls_time), "%d", segment_duration);

    // Basic HLS settings
    av_dict_set(&options, "hls_time", hls_time, 0);
    av_dict_set(&options, "hls_list_size", "10", 0);
    av_dict_set(&options, "hls_flags", "delete_segments+program_date_time", 0);
    
    // Add additional options for better compatibility
    av_dict_set(&options, "hls_allow_cache", "1", 0);
    av_dict_set(&options, "start_number", "0", 0);
    
    // Ensure FFmpeg creates a proper manifest
    av_dict_set(&options, "hls_playlist_type", "event", 0);
    
    // Set segment filename format
    char segment_format[MAX_PATH_LENGTH + 32];
    snprintf(segment_format, sizeof(segment_format), "%s/segment_%%d.ts", output_dir);
    av_dict_set(&options, "hls_segment_filename", segment_format, 0);

    // Open output file
    ret = avio_open2(&writer->output_ctx->pb, output_path,
                    AVIO_FLAG_WRITE, NULL, &options);

    if (ret < 0) {
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Failed to open HLS output file: %s", error_buf);
        avformat_free_context(writer->output_ctx);
        free(writer);
        av_dict_free(&options);
        return NULL;
    }

    av_dict_free(&options);

    log_info("Created HLS writer for stream %s at %s", stream_name, output_dir);
    return writer;
}

int hls_writer_initialize(hls_writer_t *writer, const AVStream *input_stream) {
    if (!writer || !input_stream) {
        return -1;
    }

    // Create output stream
    AVStream *out_stream = avformat_new_stream(writer->output_ctx, NULL);
    if (!out_stream) {
        log_error("Failed to create output stream for HLS");
        return -1;
    }

    // Copy codec parameters
    int ret = avcodec_parameters_copy(out_stream->codecpar, input_stream->codecpar);
    if (ret < 0) {
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Failed to copy codec parameters: %s", error_buf);
        return ret;
    }

    // Set stream time base
    out_stream->time_base = input_stream->time_base;

    // SIMPLIFIED APPROACH: Let FFmpeg handle manifest file creation
    // Just use basic options for reliability
    AVDictionary *options = NULL;

    // Write the header
    ret = avformat_write_header(writer->output_ctx, &options);
    if (ret < 0) {
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Failed to write HLS header: %s", error_buf);
        av_dict_free(&options);
        return ret;
    }

    av_dict_free(&options);

    // SIMPLIFIED APPROACH: Let FFmpeg handle manifest file creation completely
    // Remove the fallback mechanism as it might interfere with FFmpeg's own manifest creation
    log_info("Letting FFmpeg handle manifest file creation for stream %s", writer->stream_name);

    writer->initialized = 1;
    log_info("Initialized HLS writer for stream %s", writer->stream_name);
    return 0;
}


/**
 * Ensure the output directory exists and is writable
 */
static int ensure_output_directory(const char *dir_path) {
    struct stat st;
    config_t *global_config = get_streaming_config();
    char safe_dir_path[MAX_PATH_LENGTH];
    
    // CRITICAL FIX: Always use the consistent path structure for HLS
    // Extract stream name from the path (last component)
    const char *last_slash = strrchr(dir_path, '/');
    const char *stream_name = last_slash ? last_slash + 1 : dir_path;
    
    // Create a path within our storage directory
    snprintf(safe_dir_path, sizeof(safe_dir_path), "%s/hls/%s", 
            global_config->storage_path, stream_name);
    
    // Log if we're redirecting from a different path
    if (strcmp(dir_path, safe_dir_path) != 0) {
        log_warn("Redirecting HLS directory from %s to %s to ensure consistent path structure", 
                dir_path, safe_dir_path);
    }
    
    // Always use the safe path
    dir_path = safe_dir_path;

    // Check if directory exists
    if (stat(dir_path, &st) != 0 || !S_ISDIR(st.st_mode)) {
        log_warn("HLS output directory does not exist or is not a directory: %s", dir_path);

        // Create directory with parent directories if needed
        char mkdir_cmd[MAX_PATH_LENGTH * 2];
        snprintf(mkdir_cmd, sizeof(mkdir_cmd), "mkdir -p %s", dir_path);

        if (system(mkdir_cmd) != 0) {
            log_error("Failed to create HLS output directory: %s", dir_path);
            return -1;
        }

        log_info("Created HLS output directory: %s", dir_path);

        // Set permissions to ensure FFmpeg can write files
        // Use 755 instead of 777 for better security
        snprintf(mkdir_cmd, sizeof(mkdir_cmd), "chmod -R 755 %s", dir_path);
        int ret_chmod = system(mkdir_cmd);
        if (ret_chmod != 0) {
            log_warn("Failed to set permissions on directory: %s (return code: %d)", dir_path, ret_chmod);
        }
    }

    // Verify directory is writable
    if (access(dir_path, W_OK) != 0) {
        log_error("HLS output directory is not writable: %s", dir_path);

        // Try to fix permissions
        char chmod_cmd[MAX_PATH_LENGTH * 2];
        snprintf(chmod_cmd, sizeof(chmod_cmd), "chmod -R 777 %s", dir_path);
        int ret_chmod = system(chmod_cmd);
        if (ret_chmod != 0) {
            log_warn("Failed to set permissions on directory: %s (return code: %d)", dir_path, ret_chmod);
        }

        // Check again
        if (access(dir_path, W_OK) != 0) {
            log_error("Still unable to write to HLS output directory: %s", dir_path);
            return -1;
        }
    }

    return 0;
}

/**
 * Write packet to HLS stream with per-stream timestamp handling
 */
int hls_writer_write_packet(hls_writer_t *writer, const AVPacket *pkt, const AVStream *input_stream) {
    // CRITICAL FIX: Add extra validation for all parameters
    if (!writer) {
        log_error("hls_writer_write_packet: NULL writer");
        return -1;
    }
    
    if (!pkt) {
        log_error("hls_writer_write_packet: NULL packet for stream %s", writer->stream_name);
        return -1;
    }
    
    if (!input_stream) {
        log_error("hls_writer_write_packet: NULL input stream for stream %s", writer->stream_name);
        return -1;
    }
    
    // CRITICAL FIX: Check if writer has been closed
    if (!writer->output_ctx) {
        log_warn("hls_writer_write_packet: Writer for stream %s has been closed", writer->stream_name);
        return -1;
    }

    // Ensure output directory exists and is writable
    // This check is performed periodically to handle cases where the directory
    // might be deleted or become inaccessible during streaming
    static time_t last_dir_check = 0;
    time_t now = time(NULL);
    if (now - last_dir_check >= 10) { // Check every 10 seconds
        if (ensure_output_directory(writer->output_dir) != 0) {
            log_error("Failed to ensure HLS output directory exists: %s", writer->output_dir);
            return -1;
        }
        last_dir_check = now;
    }

    // Lazy initialization of output stream
    if (!writer->initialized) {
        int ret = hls_writer_initialize(writer, input_stream);
        if (ret < 0) {
            return ret;
        }
    }

    // CRITICAL FIX: Check if writer has been closed after initialization
    if (!writer->output_ctx) {
        log_warn("hls_writer_write_packet: Writer for stream %s has been closed after initialization", writer->stream_name);
        return -1;
    }

    // Clone the packet as we'll need to modify it
    AVPacket out_pkt;
    if (av_packet_ref(&out_pkt, pkt) < 0) {
        log_error("Failed to reference packet for stream %s", writer->stream_name);
        return -1;
    }

    // Initialize DTS tracker for this stream if needed
    stream_dts_info_t *dts_tracker = &writer->dts_tracker;
    if (!dts_tracker->initialized) {
        dts_tracker->first_dts = pkt->dts != AV_NOPTS_VALUE ? pkt->dts : pkt->pts;
        dts_tracker->last_dts = dts_tracker->first_dts;
        dts_tracker->time_base = input_stream->time_base;
        dts_tracker->initialized = 1;

        log_debug("Initialized DTS tracker for HLS stream %s: first_dts=%lld, time_base=%d/%d",
                 writer->stream_name,
                 (long long)dts_tracker->first_dts,
                 dts_tracker->time_base.num,
                 dts_tracker->time_base.den);
    }

    // CRITICAL FIX: Ensure timestamps are set to avoid FFmpeg warnings and potential crashes
    // This is especially important for UDP streams which may have invalid timestamps
    if (out_pkt.pts == AV_NOPTS_VALUE || out_pkt.dts == AV_NOPTS_VALUE) {
        // Log the original values for debugging
        log_debug("Original timestamps for stream %s: pts=%lld, dts=%lld", 
                 writer->stream_name, 
                 (long long)(out_pkt.pts != AV_NOPTS_VALUE ? out_pkt.pts : -1), 
                 (long long)(out_pkt.dts != AV_NOPTS_VALUE ? out_pkt.dts : -1));
        
        // If both timestamps are unset, use the last DTS or a default value
        if (out_pkt.pts == AV_NOPTS_VALUE && out_pkt.dts == AV_NOPTS_VALUE) {
            if (dts_tracker->last_dts != AV_NOPTS_VALUE) {
                out_pkt.dts = dts_tracker->last_dts + 1;
                out_pkt.pts = out_pkt.dts;
                log_debug("Setting both missing timestamps for stream %s: pts=%lld, dts=%lld", 
                         writer->stream_name, (long long)out_pkt.pts, (long long)out_pkt.dts);
            } else {
                // Use a default value if we don't have a last DTS
                out_pkt.dts = 1;
                out_pkt.pts = 1;
                log_debug("Setting default timestamps for stream %s: pts=%lld, dts=%lld", 
                         writer->stream_name, (long long)out_pkt.pts, (long long)out_pkt.dts);
            }
        } else if (out_pkt.pts == AV_NOPTS_VALUE) {
            // If only PTS is unset, use DTS
            out_pkt.pts = out_pkt.dts;
            log_debug("Setting missing PTS for stream %s: pts=%lld (from dts)", 
                     writer->stream_name, (long long)out_pkt.pts);
        } else if (out_pkt.dts == AV_NOPTS_VALUE) {
            // If only DTS is unset, use PTS
            out_pkt.dts = out_pkt.pts;
            log_debug("Setting missing DTS for stream %s: dts=%lld (from pts)", 
                     writer->stream_name, (long long)out_pkt.dts);
        }
    }
    
    // Additional safety check: ensure timestamps are positive
    if (out_pkt.pts <= 0 || out_pkt.dts <= 0) {
        log_warn("Non-positive timestamps detected in stream %s: pts=%lld, dts=%lld", 
                writer->stream_name, (long long)out_pkt.pts, (long long)out_pkt.dts);
        
        // Set to safe values
        if (out_pkt.pts <= 0) {
            if (out_pkt.dts > 0) {
                out_pkt.pts = out_pkt.dts;
            } else {
                out_pkt.pts = 1;
            }
        }
        
        if (out_pkt.dts <= 0) {
            if (out_pkt.pts > 0) {
                out_pkt.dts = out_pkt.pts;
            } else {
                out_pkt.dts = 1;
            }
        }
        
        log_debug("Corrected non-positive timestamps for stream %s: pts=%lld, dts=%lld", 
                 writer->stream_name, (long long)out_pkt.pts, (long long)out_pkt.dts);
    }

    // Check for timestamp discontinuities before rescaling
    if (dts_tracker->last_dts != AV_NOPTS_VALUE) {
        // If DTS goes backwards or jumps significantly, handle it
        if (out_pkt.dts < dts_tracker->last_dts) {
            log_warn("HLS DTS discontinuity in stream %s: last=%lld, current=%lld, diff=%lld",
                    writer->stream_name,
                    (long long)dts_tracker->last_dts,
                    (long long)out_pkt.dts,
                    (long long)(out_pkt.dts - dts_tracker->last_dts));

            // Fix the DTS to ensure monotonic increase
            int64_t fixed_dts = dts_tracker->last_dts + 1;

            // Adjust PTS relative to the fixed DTS if needed
            int64_t pts_dts_diff = out_pkt.pts - out_pkt.dts;
            out_pkt.dts = fixed_dts;
            out_pkt.pts = fixed_dts + pts_dts_diff;
        }
        // Also check for large forward jumps which could indicate a stream reset
        else if (out_pkt.dts > dts_tracker->last_dts + 90000) { // More than 1 second jump (assuming 90kHz timebase)
            log_warn("HLS large DTS jump in stream %s: last=%lld, current=%lld, diff=%lld",
                    writer->stream_name,
                    (long long)dts_tracker->last_dts,
                    (long long)out_pkt.dts,
                    (long long)(out_pkt.dts - dts_tracker->last_dts));
            
            // For large forward jumps, we'll accept the new timestamp but log it
            // This allows the stream to recover from resets without forcing timestamps
        }
    }

    // Update last DTS for next comparison
    dts_tracker->last_dts = out_pkt.dts;

    // CRITICAL FIX: Check if writer has been closed before rescaling
    if (!writer->output_ctx || !writer->output_ctx->streams || !writer->output_ctx->streams[0]) {
        log_warn("hls_writer_write_packet: Writer context invalid for stream %s", writer->stream_name);
        av_packet_unref(&out_pkt);
        return -1;
    }

    // Adjust timestamps
    av_packet_rescale_ts(&out_pkt, input_stream->time_base,
                        writer->output_ctx->streams[0]->time_base);

    // Final sanity check - ensure PTS >= DTS
    if (out_pkt.pts < out_pkt.dts) {
        log_debug("Fixing invalid PTS/DTS relationship in stream %s: pts=%lld, dts=%lld",
                 writer->stream_name, (long long)out_pkt.pts, (long long)out_pkt.dts);
        out_pkt.pts = out_pkt.dts;
    }
    
    // Also check for unreasonable PTS/DTS differences
    int64_t pts_dts_diff = out_pkt.pts - out_pkt.dts;
    if (pts_dts_diff > 90000 * 10) { // More than 10 seconds difference (assuming 90kHz timebase)
        log_warn("Unreasonable PTS/DTS difference in stream %s: pts=%lld, dts=%lld, diff=%lld",
                writer->stream_name, 
                (long long)out_pkt.pts, 
                (long long)out_pkt.dts,
                (long long)pts_dts_diff);
        
        // Cap the difference to something reasonable
        out_pkt.pts = out_pkt.dts + 90000 * 5; // 5 seconds max difference
    }

    // CRITICAL FIX: Check if writer has been closed before writing
    if (!writer->output_ctx) {
        log_warn("hls_writer_write_packet: Writer for stream %s has been closed before writing", writer->stream_name);
        av_packet_unref(&out_pkt);
        return -1;
    }

    // CRITICAL FIX: Add logging for key frames to help diagnose issues
    bool is_key_frame = (out_pkt.flags & AV_PKT_FLAG_KEY) != 0;
    if (is_key_frame) {
        log_debug("Writing key frame to HLS for stream %s: pts=%lld, dts=%lld, size=%d",
                 writer->stream_name, (long long)out_pkt.pts, (long long)out_pkt.dts, out_pkt.size);
    }
    
    // Write the packet
    int ret = av_interleaved_write_frame(writer->output_ctx, &out_pkt);
    av_packet_unref(&out_pkt);

    if (ret < 0) {
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Error writing HLS packet for stream %s: %s", writer->stream_name, error_buf);
        
        // SIMPLIFIED APPROACH: Let FFmpeg handle manifest file errors
        // Just ensure the directory exists
        if (ensure_output_directory(writer->output_dir) == 0) {
            log_info("Ensured HLS output directory exists after error: %s", writer->output_dir);
        }

        // If we get a "No such file or directory" error, try to recreate the directory
        if (strstr(error_buf, "No such file or directory") != NULL) {
            log_warn("Directory issue detected, attempting to recreate: %s", writer->output_dir);
            if (ensure_output_directory(writer->output_dir) == 0) {
                // Directory recreated, but we still need to return the error
                // The next packet write attempt should succeed
                log_info("Successfully recreated HLS output directory: %s", writer->output_dir);
            }
        } else if (strstr(error_buf, "Invalid argument") != NULL ||
                  strstr(error_buf, "non monotonically increasing dts") != NULL) {
            // Reset DTS tracker on timestamp errors to recover
            log_warn("Resetting DTS tracker for stream %s due to timestamp error", writer->stream_name);
            dts_tracker->initialized = 0;
        }

        return ret;
    }

    // CRITICAL FIX: Check if writer has been closed before cleanup
    if (!writer->output_ctx) {
        log_warn("hls_writer_write_packet: Writer for stream %s has been closed before cleanup", writer->stream_name);
        return 0; // Return success since we already wrote the packet
    }

    // Periodically clean up old segments (every 60 seconds)
    if (now - writer->last_cleanup_time >= 60) {
        // Keep more segments than in the playlist to ensure smooth playback on low-power devices
        int max_segments_to_keep = 12; // Increased from 8 to 12 segments (more than hls_list_size of 8)
        cleanup_old_segments(writer->output_dir, max_segments_to_keep);
        writer->last_cleanup_time = now;
    }

    return 0;
}

/**
 * Close HLS writer and free resources
 */
void hls_writer_close(hls_writer_t *writer) {
    if (!writer) {
        log_warn("Attempted to close NULL HLS writer");
        return;
    }
    
    // Store stream name for logging after freeing
    char stream_name[MAX_STREAM_NAME];
    strncpy(stream_name, writer->stream_name, MAX_STREAM_NAME - 1);
    stream_name[MAX_STREAM_NAME - 1] = '\0';
    
    // CRITICAL FIX: Check if the writer has already been closed by checking if output_ctx is NULL
    if (!writer->output_ctx) {
        log_warn("Attempted to close already closed HLS writer for stream %s", stream_name);
        return;
    }
    
    // CRITICAL FIX: Use a mutex to prevent concurrent access during closing
    static pthread_mutex_t close_mutex = PTHREAD_MUTEX_INITIALIZER;
    pthread_mutex_lock(&close_mutex);

    // Create a local copy of the output context to avoid race conditions
    AVFormatContext *local_output_ctx = writer->output_ctx;
    
    // Mark the writer as closed immediately to prevent further access
    writer->output_ctx = NULL;
    
    // Write trailer if initialized
    if (writer->initialized && local_output_ctx) {
        log_info("Writing trailer for HLS writer for stream %s", stream_name);
        int ret = av_write_trailer(local_output_ctx);
        if (ret < 0) {
            char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
            av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
            log_warn("Error writing trailer for HLS writer for stream %s: %s", stream_name, error_buf);
        }
    }

    // Close output context
    if (local_output_ctx) {
        if (local_output_ctx->pb) {
            log_info("Closing AVIO context for HLS writer for stream %s", stream_name);
            avio_close(local_output_ctx->pb);
            local_output_ctx->pb = NULL; // Prevent double close
        }
        log_info("Freeing format context for HLS writer for stream %s", stream_name);
        avformat_free_context(local_output_ctx);
    }

    log_info("Closed HLS writer for stream %s", stream_name);

    // Unlock the mutex before freeing the writer
    pthread_mutex_unlock(&close_mutex);

    // CRITICAL FIX: Use a separate mutex for freeing to prevent race conditions
    static pthread_mutex_t free_mutex = PTHREAD_MUTEX_INITIALIZER;
    pthread_mutex_lock(&free_mutex);
    
    // Free writer
    free(writer);
    
    pthread_mutex_unlock(&free_mutex);
}
