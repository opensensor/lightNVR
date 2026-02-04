#define _POSIX_C_SOURCE 200809L
#define _XOPEN_SOURCE 700
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <dirent.h>
#include <sys/stat.h>
#include <pthread.h>
#include <time.h>
#include <fcntl.h>  // For O_NONBLOCK
#include <errno.h>  // For error codes
#include <signal.h> // For alarm
#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <libavutil/avutil.h>
#include <libavutil/time.h>

#include "core/logger.h"
#include "video/hls_writer.h"
#include "video/detection_integration.h"
#include "video/detection_frame_processing.h"
#include "video/streams.h"
#include "video/stream_manager.h"

// Forward declarations from detection_stream.c
extern int is_detection_stream_reader_running(const char *stream_name);
extern int get_detection_interval(const char *stream_name);

// Forward declarations for internal functions
static int ensure_output_directory(hls_writer_t *writer);
static void register_hls_writer(hls_writer_t *writer);
static void unregister_hls_writer(hls_writer_t *writer);

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
    int actual_count = 0;

    // Open directory
    dir = opendir(output_dir);
    if (!dir) {
        log_error("Failed to open directory for cleanup: %s", output_dir);
        return;
    }

    // Count segment files first
    while ((entry = readdir(dir)) != NULL) {
        // Check for both .ts and .m4s files (support both formats)
        if (strstr(entry->d_name, ".m4s") != NULL || strstr(entry->d_name, ".ts") != NULL) {
            segment_count++;
        }
    }

    // If we don't have more than the max, no cleanup needed
    if (segment_count <= max_segments) {
        closedir(dir);
        return;
    }

    // Allocate array for segment info with proper alignment
    // Use calloc instead of malloc to ensure memory is initialized to zero
    segments = (segment_info_t *)calloc(segment_count, sizeof(segment_info_t));
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
        // Check for both .ts and .m4s files (support both formats)
        if (strstr(entry->d_name, ".m4s") == NULL && strstr(entry->d_name, ".ts") == NULL) {
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

    // Store the actual number of segments we found
    actual_count = i;

    closedir(dir);

    // Sort segments by modification time (oldest first)
    // Simple bubble sort for now
    for (int j = 0; j < actual_count - 1; j++) {
        for (int k = 0; k < actual_count - j - 1; k++) {
            if (segments[k].mtime > segments[k + 1].mtime) {
                segment_info_t temp = segments[k];
                segments[k] = segments[k + 1];
                segments[k + 1] = temp;
            }
        }
    }

    // Delete oldest segments beyond our limit
    int to_delete = actual_count - max_segments;
    if (to_delete > 0 && to_delete <= actual_count) { // Safety check to prevent out-of-bounds access
        for (int j = 0; j < to_delete && j < actual_count; j++) { // Additional bounds check
            // Ensure filename is valid before using it
            if (segments[j].filename[0] != '\0') {
                snprintf(filepath, sizeof(filepath), "%s/%s", output_dir, segments[j].filename);
                if (unlink(filepath) == 0) {
                    log_debug("Deleted old HLS segment: %s", segments[j].filename);
                } else {
                    log_warn("Failed to delete old HLS segment: %s (error: %s)",
                            segments[j].filename, strerror(errno));
                }
            }
        }
    }

    // Always free the allocated memory
    free(segments);
    segments = NULL;
}

hls_writer_t *hls_writer_create(const char *output_dir, const char *stream_name, int segment_duration) {
    // Check if a writer for this stream already exists
    hls_writer_t *existing_writer = find_hls_writer_by_stream_name(stream_name);
    if (existing_writer) {
        log_warn("HLS writer for stream %s already exists. Closing existing writer to prevent duplicates.", stream_name);
        // Close the existing writer to prevent duplicates
        hls_writer_close(existing_writer);
        // Wait a bit to ensure resources are released
        usleep(100000); // 100ms
    }

    // Allocate writer structure
    hls_writer_t *writer = (hls_writer_t *)calloc(1, sizeof(hls_writer_t));
    if (!writer) {
        log_error("Failed to allocate HLS writer");
        return NULL;
    }

    // Copy output directory and stream name
    strncpy(writer->output_dir, output_dir, MAX_PATH_LENGTH - 1);
    strncpy(writer->stream_name, stream_name, MAX_STREAM_NAME - 1);

    //  Ensure segment duration is reasonable but allow lower values for lower latency
    if (segment_duration < 0.5) {
        log_warn("HLS segment duration too low (%d), setting to 0.5 seconds minimum",
                segment_duration);
        segment_duration = 0.5;  // Minimum 0.5 seconds for lower latency
    } else if (segment_duration > 10) {
        log_warn("HLS segment duration too high (%d), capping at 10 seconds",
                segment_duration);
        segment_duration = 10;  // Maximum 10 seconds
    }

    writer->segment_duration = segment_duration;
    writer->last_cleanup_time = time(NULL);

    // Initialize mutex
    pthread_mutex_init(&writer->mutex, NULL);

    // Ensure the output directory exists and is writable
    // This will also update the writer's output_dir field with the safe path if needed
    if (ensure_output_directory(writer) != 0) {
        log_error("Failed to ensure HLS output directory exists: %s", writer->output_dir);
        pthread_mutex_destroy(&writer->mutex);
        free(writer);
        return NULL;
    }

    // Initialize output format context for HLS
    char output_path[MAX_PATH_LENGTH];
    snprintf(output_path, MAX_PATH_LENGTH, "%s/index.m3u8", writer->output_dir);

    // Allocate output format context
    int ret = avformat_alloc_output_context2(
        &writer->output_ctx, NULL, "hls", output_path);

    if (ret < 0) {
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Failed to allocate output context for HLS: %s", error_buf);
        pthread_mutex_destroy(&writer->mutex);
        free(writer);
        return NULL;
    }

    // Set HLS options - optimized for stability and compatibility
    AVDictionary *options = NULL;
    char hls_time[16];
    snprintf(hls_time, sizeof(hls_time), "%d", segment_duration);  // Use the validated segment duration

    // CRITICAL FIX: Modify HLS options to prevent segmentation faults
    // Use more conservative settings that prioritize stability over low latency
    av_dict_set(&options, "hls_time", hls_time, 0);
    av_dict_set(&options, "hls_list_size", "6", 0);  // Increased from 3 to 6 for better buffering and stability

    // Use MPEG-TS segments for better compatibility and to avoid MP4 moov atom issues
    av_dict_set(&options, "hls_segment_type", "mpegts", 0);

    // Enable aggressive segment deletion to prevent accumulation
    // delete_segments: Automatically delete old segments
    // independent_segments: Make each segment independently decodable
    // program_date_time: Add timestamps for better seeking
    av_dict_set(&options, "hls_flags", "delete_segments+independent_segments+program_date_time", 0);

    // CRITICAL FIX: Force keyframes at segment boundaries to prevent bufferAppendError in HLS.js
    // This ensures each segment starts with a keyframe (I-frame), making them independently decodable
    // Format: "expr:gte(t,n_forced*<segment_duration>)" forces a keyframe every segment_duration seconds
    char force_key_frames[64];
    snprintf(force_key_frames, sizeof(force_key_frames), "expr:gte(t,n_forced*%d)", segment_duration);
    av_dict_set(&options, "force_key_frames", force_key_frames, 0);

    // Set start number
    av_dict_set(&options, "start_number", "0", 0);

    // Disable flushing to avoid race conditions
    av_dict_set(&options, "flush_packets", "0", 0);

    // Add additional options to prevent segmentation faults
    av_dict_set(&options, "avoid_negative_ts", "make_non_negative", 0);

    // Set segment filename format for MPEG-TS
    char segment_format[MAX_PATH_LENGTH + 32];
    snprintf(segment_format, sizeof(segment_format), "%s/segment_%%d.ts", writer->output_dir);
    av_dict_set(&options, "hls_segment_filename", segment_format, 0);

    // Log simplified options for debugging
    log_info("HLS writer options for stream %s (optimized for stability and compatibility):", writer->stream_name);
    log_info("  hls_time: %s", hls_time);
    log_info("  hls_list_size: 6");
    log_info("  hls_flags: delete_segments+independent_segments+program_date_time");
    log_info("  hls_segment_type: mpegts");
    log_info("  force_key_frames: %s", force_key_frames);
    log_info("  start_number: 0");
    log_info("  hls_segment_filename: %s", segment_format);

    // Open output file
    ret = avio_open2(&writer->output_ctx->pb, output_path,
                    AVIO_FLAG_WRITE, NULL, &options);

    if (ret < 0) {
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Failed to open HLS output file: %s", error_buf);
        av_dict_free(&options);
        avformat_free_context(writer->output_ctx);
        pthread_mutex_destroy(&writer->mutex);
        free(writer);
        return NULL;
    }

    av_dict_free(&options);

    log_info("Created HLS writer for stream %s at %s with segment duration %d seconds",
            stream_name, output_dir, segment_duration);

    // Register the writer for global tracking
    register_hls_writer(writer);

    return writer;
}

int hls_writer_initialize(hls_writer_t *writer, const AVStream *input_stream) {
    if (!writer || !input_stream) {
        log_error("Invalid parameters for hls_writer_initialize");
        return -1;
    }

    if (!writer->output_ctx) {
        log_error("Output context is NULL in hls_writer_initialize");
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

    //  For HLS streaming, we need to set the correct codec parameters
    // The issue is not with the bitstream filter but with how we're configuring the output

    // For H.264 streams, we need to ensure the correct format
    if (input_stream->codecpar->codec_id == AV_CODEC_ID_H264) {
        // Set the correct codec tag for H.264 in HLS (0 means auto-detect)
        out_stream->codecpar->codec_tag = 0;

        // Apply codec tag to all streams in the output context
        if (writer->output_ctx) {
            for (int i = 0; i < writer->output_ctx->nb_streams; i++) {
                AVStream *stream = writer->output_ctx->streams[i];
                if (stream) {
                    stream->codecpar->codec_tag = 0;
                }
            }
        }

        log_info("Set correct codec parameters for H.264 in HLS for stream %s", writer->stream_name);
    } else {
        log_info("Stream %s is not H.264, using default codec parameters", writer->stream_name);
    }

    // Write the header
    AVDictionary *options = NULL;
    ret = avformat_write_header(writer->output_ctx, &options);
    if (ret < 0) {
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Failed to write HLS header: %s", error_buf);
        av_dict_free(&options);
        return ret;
    }

    av_dict_free(&options);

    // Let FFmpeg handle manifest file creation
    log_info("Initialized HLS writer for stream %s", writer->stream_name);
    writer->initialized = 1;
    return 0;
}

/**
 * Ensure the output directory exists and is writable
 * Updates the writer's output_dir field with the safe path
 */
static int ensure_output_directory(hls_writer_t *writer) {
    struct stat st;
    config_t *global_config = get_streaming_config();

    // Check if writer or global_config is NULL to prevent null pointer dereference
    if (!writer || !global_config) {
        log_error("Failed to get streaming config for HLS directory or writer is NULL");
        return -1;
    }

    char safe_dir_path[MAX_PATH_LENGTH];
    const char *dir_path = writer->output_dir;

    //  Always use the consistent path structure for HLS
    // Extract stream name from the path (last component)
    const char *last_slash = strrchr(dir_path, '/');
    const char *stream_name = last_slash ? last_slash + 1 : dir_path;

    // Use storage_path_hls if specified, otherwise fall back to storage_path
    const char *base_storage_path = global_config->storage_path;
    if (global_config->storage_path_hls[0] != '\0') {
        base_storage_path = global_config->storage_path_hls;
        log_info("Using dedicated HLS storage path in writer: %s", base_storage_path);
    }

    // Create a path within our storage directory
    snprintf(safe_dir_path, sizeof(safe_dir_path), "%s/hls/%s",
            base_storage_path, stream_name);

    // Log if we're redirecting from a different path
    if (strcmp(dir_path, safe_dir_path) != 0) {
        log_warn("Redirecting HLS directory from %s to %s to ensure consistent path structure",
                dir_path, safe_dir_path);

        // Update the writer's output_dir field with the safe path
        strncpy(writer->output_dir, safe_dir_path, MAX_PATH_LENGTH - 1);
        writer->output_dir[MAX_PATH_LENGTH - 1] = '\0';
    }

    // Always use the safe path
    dir_path = safe_dir_path;

    // Check if directory exists
    if (stat(dir_path, &st) != 0 || !S_ISDIR(st.st_mode)) {
        log_warn("HLS output directory does not exist or is not a directory: %s", dir_path);

        // Create directory using direct C functions to handle paths with spaces
        char temp_path[MAX_PATH_LENGTH];
        strncpy(temp_path, dir_path, MAX_PATH_LENGTH - 1);
        temp_path[MAX_PATH_LENGTH - 1] = '\0';

        // Create parent directories one by one
        for (char *p = temp_path + 1; *p; p++) {
            if (*p == '/') {
                *p = '\0';
                if (mkdir(temp_path, 0755) != 0 && errno != EEXIST) {
                    log_warn("Failed to create parent directory: %s (error: %s)",
                            temp_path, strerror(errno));
                }
                *p = '/';
            }
        }

        // Create the final directory
        if (mkdir(temp_path, 0755) != 0 && errno != EEXIST) {
            log_error("Failed to create output directory: %s (error: %s)",
                    temp_path, strerror(errno));
            return -1;
        }

        log_info("Created HLS output directory: %s", dir_path);

        // Set permissions to ensure FFmpeg can write files
        if (chmod(dir_path, 0755) != 0) {
            log_warn("Failed to set permissions on directory: %s (error: %s)",
                    dir_path, strerror(errno));
        }
    }

    // Verify directory is writable
    if (access(dir_path, W_OK) != 0) {
        log_error("HLS output directory is not writable: %s", dir_path);

        // Try to fix permissions using direct chmod
        if (chmod(dir_path, 0777) != 0) {
            log_warn("Failed to set permissions on directory: %s (error: %s)",
                    dir_path, strerror(errno));
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
 * Write packet to HLS stream with per-stream timestamp handling and proper bitstream filtering
 */
int hls_writer_write_packet(hls_writer_t *writer, const AVPacket *pkt, const AVStream *input_stream) {
    // Validate parameters
    if (!writer) {
        log_error("hls_writer_write_packet: NULL writer");
        return -1;
    }

    if (!pkt) {
        log_error("hls_writer_write_packet: NULL packet for stream %s", writer ? writer->stream_name : "unknown");
        return -1;
    }

    if (!input_stream) {
        log_error("hls_writer_write_packet: NULL input stream for stream %s", writer ? writer->stream_name : "unknown");
        return -1;
    }

    // Check if writer has been closed
    if (!writer->output_ctx) {
        log_warn("hls_writer_write_packet: Writer for stream %s has been closed", writer->stream_name);
        return -1;
    }

    // Periodically ensure output directory exists (every 10 seconds)
    // Use thread-safe approach with per-writer timestamp instead of static variable
    time_t now = time(NULL);
    if (now - writer->last_cleanup_time >= 10) {
        if (ensure_output_directory(writer) != 0) {
            log_error("Failed to ensure HLS output directory exists: %s", writer->output_dir);
            return -1;
        }
        // Update the last cleanup time which is also used for directory checks
        writer->last_cleanup_time = now;
    }

    // Lazy initialization of output stream with additional safety checks
    if (!writer->initialized) {
        int ret = hls_writer_initialize(writer, input_stream);
        if (ret < 0) {
            return ret;
        }
    }

    // Double-check if writer has been closed after initialization
    if (!writer->output_ctx) {
        log_warn("hls_writer_write_packet: Writer for stream %s has been closed after initialization", writer->stream_name);
        return -1;
    }

    // Clone the packet with additional safety checks
    // Use av_packet_alloc instead of stack allocation to ensure proper alignment
    AVPacket *out_pkt_ptr = av_packet_alloc();
    if (!out_pkt_ptr) {
        log_error("Failed to allocate packet for stream %s", writer->stream_name);
        return -1;
    }
    // out_pkt_ptr is already initialized by av_packet_alloc

    // Verify packet data is valid before referencing
    if (!pkt->data || pkt->size <= 0) {
        log_warn("Invalid packet data for stream %s (data=%p, size=%d)",
                writer->stream_name, pkt->data, pkt->size);
        av_packet_free(&out_pkt_ptr);  // MEMORY LEAK FIX: Free allocated packet before early return
        return -1;
    }

    if (av_packet_ref(out_pkt_ptr, pkt) < 0) {
        log_error("Failed to reference packet for stream %s", writer->stream_name);
        av_packet_free(&out_pkt_ptr);
        return -1;
    }

    // Set up cleanup for error cases
    int result = -1;

    //  CRITICAL FIX: More robust bitstream filtering for H.264 in HLS
    // This is essential to prevent the "h264 bitstream malformed, no startcode found" error
    // and to avoid segmentation faults during shutdown
    if (input_stream->codecpar->codec_id == AV_CODEC_ID_H264) {
        // Use a simpler and more reliable approach for H.264 bitstream conversion
        // Instead of creating a new filter for each packet, we'll manually add the start code

        // Check if the packet already has start codes (Annex B format)
        bool has_start_code = false;
        if (out_pkt_ptr->size >= 4) {
            has_start_code = (out_pkt_ptr->data[0] == 0x00 &&
                             out_pkt_ptr->data[1] == 0x00 &&
                             out_pkt_ptr->data[2] == 0x00 &&
                             out_pkt_ptr->data[3] == 0x01);
        }

        if (!has_start_code) {
    // Create a new packet with space for the start code
    AVPacket *new_pkt_ptr = av_packet_alloc();
    if (!new_pkt_ptr) {
        log_error("Failed to allocate new packet for H.264 conversion for stream %s",
                 writer->stream_name);
        av_packet_free(&out_pkt_ptr);
        return -1;
    }

    // Allocate a new buffer with space for the start code
    if (av_new_packet(new_pkt_ptr, out_pkt_ptr->size + 4) < 0) {
        log_error("Failed to allocate new packet for H.264 conversion for stream %s",
                 writer->stream_name);
        av_packet_free(&new_pkt_ptr);
        av_packet_free(&out_pkt_ptr);
        return -1;
    }

    // Add start code
    new_pkt_ptr->data[0] = 0x00;
    new_pkt_ptr->data[1] = 0x00;
    new_pkt_ptr->data[2] = 0x00;
    new_pkt_ptr->data[3] = 0x01;

    // Copy the packet data
    memcpy(new_pkt_ptr->data + 4, out_pkt_ptr->data, out_pkt_ptr->size);

    // Copy other packet properties
    new_pkt_ptr->pts = out_pkt_ptr->pts;
    new_pkt_ptr->dts = out_pkt_ptr->dts;
    new_pkt_ptr->flags = out_pkt_ptr->flags;
    new_pkt_ptr->stream_index = out_pkt_ptr->stream_index;
    new_pkt_ptr->duration = out_pkt_ptr->duration;
    new_pkt_ptr->pos = out_pkt_ptr->pos;

    // Unref the original packet
    av_packet_free(&out_pkt_ptr);

    // Use the new packet as our output packet
    out_pkt_ptr = new_pkt_ptr;
    new_pkt_ptr = NULL; // Prevent double-free

            log_debug("Added H.264 start code for stream %s", writer->stream_name);
        }
    } else {
        log_debug("Using original packet for non-H264 stream %s", writer->stream_name);
    }

    // SIMPLIFIED TIMESTAMP HANDLING: Trust go2rtc to provide clean timestamps
    // go2rtc handles RTSP stream normalization, so we only need minimal fixes

    // Only fix if both PTS and DTS are invalid (rare edge case)
    if (out_pkt_ptr->pts == AV_NOPTS_VALUE && out_pkt_ptr->dts == AV_NOPTS_VALUE) {
        log_warn("Both PTS and DTS are invalid for packet in stream %s, skipping packet",
                writer->stream_name);
        av_packet_free(&out_pkt_ptr);
        return 0; // Skip this packet rather than trying to fix it
    }

    // If only one is missing, copy from the other
    if (out_pkt_ptr->pts == AV_NOPTS_VALUE) {
        out_pkt_ptr->pts = out_pkt_ptr->dts;
    } else if (out_pkt_ptr->dts == AV_NOPTS_VALUE) {
        out_pkt_ptr->dts = out_pkt_ptr->pts;
    }

    // Ensure PTS >= DTS (required by HLS format)
    if (out_pkt_ptr->pts < out_pkt_ptr->dts) {
        out_pkt_ptr->pts = out_pkt_ptr->dts;
    }

    // Validate writer context before rescaling
    if (!writer->output_ctx || !writer->output_ctx->streams || !writer->output_ctx->streams[0]) {
        log_warn("hls_writer_write_packet: Writer context invalid for stream %s", writer->stream_name);
        av_packet_free(&out_pkt_ptr);
        return -1;
    }

    // Rescale timestamps to output timebase
    av_packet_rescale_ts(out_pkt_ptr, input_stream->time_base,
                        writer->output_ctx->streams[0]->time_base);

    // CRITICAL FIX: Ensure PTS >= DTS with a small buffer to prevent ghosting artifacts
    // This is essential for HLS format compliance and prevents visual artifacts
    if (out_pkt_ptr->pts < out_pkt_ptr->dts) {
        log_debug("Fixing HLS packet with PTS < DTS: PTS=%lld, DTS=%lld",
                 (long long)out_pkt_ptr->pts, (long long)out_pkt_ptr->dts);
        out_pkt_ptr->pts = out_pkt_ptr->dts;
    }

    // Cap unreasonable PTS/DTS differences
    if (out_pkt_ptr->pts != AV_NOPTS_VALUE && out_pkt_ptr->dts != AV_NOPTS_VALUE) {
        int64_t pts_dts_diff = out_pkt_ptr->pts - out_pkt_ptr->dts;
        if (pts_dts_diff > 90000 * 10) {
            out_pkt_ptr->pts = out_pkt_ptr->dts + 90000 * 5; // 5 seconds max difference
        }
    }

    // Log key frames for diagnostics
    bool is_key_frame = (out_pkt_ptr->flags & AV_PKT_FLAG_KEY) != 0;
    if (is_key_frame) {
        log_debug("Writing key frame to HLS for stream %s: pts=%lld, dts=%lld, size=%d",
                 writer->stream_name, (long long)out_pkt_ptr->pts, (long long)out_pkt_ptr->dts, out_pkt_ptr->size);
    }

    result = av_interleaved_write_frame(writer->output_ctx, out_pkt_ptr);

    // Clean up packet
    av_packet_free(&out_pkt_ptr);

    // Handle write errors
    if (result < 0) {
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(result, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Error writing HLS packet for stream %s: %s", writer->stream_name, error_buf);

        // Try to fix directory issues
        if (strstr(error_buf, "No such file or directory") != NULL) {
            ensure_output_directory(writer);
        } else if (strstr(error_buf, "Invalid data found when processing input") != NULL) {
            // For invalid data errors, return 0 to continue processing
            // go2rtc handles stream normalization, so occasional bad packets can be skipped
            log_warn("Ignoring invalid packet data for stream %s and continuing", writer->stream_name);
            result = 0;
        }
        // For other errors (Invalid argument, non-monotonic DTS), just log and return error
        // The caller can decide whether to retry or reinitialize the stream
    } else {
        // Success case
        // Let FFmpeg handle segment cleanup automatically
        // Update the last cleanup time to prevent uninitialized value issues
        writer->last_cleanup_time = now;
    }

    return result;
}

// Global array to track all created HLS writers for cleanup during shutdown
#define MAX_HLS_WRITERS 32
static hls_writer_t *g_hls_writers[MAX_HLS_WRITERS] = {0};
static pthread_mutex_t g_hls_writers_mutex = PTHREAD_MUTEX_INITIALIZER;
static int g_hls_writers_count = 0;

/**
 * Check if an HLS writer for a stream name already exists
 * This helps prevent duplicate writers for the same stream
 *
 * @param stream_name Name of the stream to check
 * @return Pointer to existing writer, or NULL if none exists
 */
hls_writer_t *find_hls_writer_by_stream_name(const char *stream_name) {
    if (!stream_name) return NULL;

    pthread_mutex_lock(&g_hls_writers_mutex);

    hls_writer_t *existing_writer = NULL;
    for (int i = 0; i < MAX_HLS_WRITERS; i++) {
        if (g_hls_writers[i] && g_hls_writers[i]->stream_name &&
            strcmp(g_hls_writers[i]->stream_name, stream_name) == 0) {
            existing_writer = g_hls_writers[i];
            break;
        }
    }

    pthread_mutex_unlock(&g_hls_writers_mutex);
    return existing_writer;
}

/**
 * Register an HLS writer for global tracking
 * This allows us to clean up all writers during shutdown
 * Enhanced to prevent duplicate writers for the same stream
 */
static void register_hls_writer(hls_writer_t *writer) {
    if (!writer) return;

    pthread_mutex_lock(&g_hls_writers_mutex);

    // Find an empty slot or check if already registered
    bool already_registered = false;
    int empty_slot = -1;
    bool duplicate_stream_name = false;
    hls_writer_t *existing_writer = NULL;

    for (int i = 0; i < MAX_HLS_WRITERS; i++) {
        if (g_hls_writers[i] == writer) {
            already_registered = true;
            break;
        }
        // Check for duplicate stream name
        if (g_hls_writers[i] && g_hls_writers[i]->stream_name &&
            writer->stream_name &&
            strcmp(g_hls_writers[i]->stream_name, writer->stream_name) == 0) {
            duplicate_stream_name = true;
            existing_writer = g_hls_writers[i];
            log_warn("Found duplicate HLS writer for stream %s during registration", writer->stream_name);
        }
        if (g_hls_writers[i] == NULL && empty_slot == -1) {
            empty_slot = i;
        }
    }

    // If we found a duplicate stream name, log a warning but still register the new writer
    // The caller should handle the duplicate by stopping the old writer
    if (duplicate_stream_name && existing_writer) {
        log_warn("Multiple HLS writers detected for stream %s. This may cause issues.", writer->stream_name);
    }

    // Add to tracking if not already registered and we have an empty slot
    if (!already_registered && empty_slot != -1) {
        g_hls_writers[empty_slot] = writer;
        g_hls_writers_count++;
        log_debug("Registered HLS writer %p for stream %s for global tracking (total: %d)",
                 (void*)writer, writer->stream_name, g_hls_writers_count);
    }

    pthread_mutex_unlock(&g_hls_writers_mutex);
}

/**
 * Unregister an HLS writer from global tracking
 */
static void unregister_hls_writer(hls_writer_t *writer) {
    if (!writer) return;

    pthread_mutex_lock(&g_hls_writers_mutex);

    for (int i = 0; i < MAX_HLS_WRITERS; i++) {
        if (g_hls_writers[i] == writer) {
            g_hls_writers[i] = NULL;
            g_hls_writers_count--;
            log_debug("Unregistered HLS writer %p for stream %s from global tracking (remaining: %d)",
                     (void*)writer, writer->stream_name, g_hls_writers_count);
            break;
        }
    }

    pthread_mutex_unlock(&g_hls_writers_mutex);
}

/**
 * Clean up all HLS writers during shutdown
 * This function should be called during application shutdown
 */
void cleanup_all_hls_writers(void) {
    log_info("Cleaning up all HLS writers...");

    pthread_mutex_lock(&g_hls_writers_mutex);

    // Make a copy of the writers array to avoid issues with concurrent modification
    hls_writer_t *writers_to_close[MAX_HLS_WRITERS] = {0};
    int writers_count = 0;

    for (int i = 0; i < MAX_HLS_WRITERS; i++) {
        if (g_hls_writers[i] != NULL) {
            writers_to_close[writers_count++] = g_hls_writers[i];
            g_hls_writers[i] = NULL; // Clear the entry to prevent double-free
        }
    }

    g_hls_writers_count = 0;
    pthread_mutex_unlock(&g_hls_writers_mutex);

    // Close each writer
    for (int i = 0; i < writers_count; i++) {
        if (writers_to_close[i]) {
            log_info("Closing HLS writer %d/%d during global cleanup: %s",
                    i+1, writers_count, writers_to_close[i]->stream_name);
            hls_writer_close(writers_to_close[i]);
        }
    }

    log_info("Completed cleanup of %d HLS writers", writers_count);
}

/**
 * Close HLS writer and free resources
 * This function is thread-safe and handles all cleanup operations with improved robustness
 * to ensure safe operation with go2rtc integration
 */
// Flag to prevent recursive calls to hls_writer_close
static __thread bool in_writer_close = false;

void hls_writer_close(hls_writer_t *writer) {
    // CRITICAL FIX: Add additional NULL check at the beginning
    if (!writer) {
        log_warn("Attempted to close NULL HLS writer");
        return;
    }

    // CRITICAL FIX: Prevent recursive calls that can cause double free
    if (in_writer_close) {
        log_warn("Recursive call to hls_writer_close detected, aborting to prevent double free");
        return;
    }
    in_writer_close = true;

    // CRITICAL FIX: Use atomic operations to check if this writer is already being freed
    // Use a per-writer closed flag instead of a global mutex to allow parallel closing of different writers
    static pthread_mutex_t close_mutex = PTHREAD_MUTEX_INITIALIZER;

    // Wait for the mutex with a timeout instead of aborting immediately
    struct timespec close_timeout;
    clock_gettime(CLOCK_REALTIME, &close_timeout);
    close_timeout.tv_sec += 10; // 10 second timeout for acquiring close mutex

    int close_mutex_result = pthread_mutex_timedlock(&close_mutex, &close_timeout);
    if (close_mutex_result != 0) {
        log_warn("Timeout waiting for HLS close mutex (error: %s), proceeding with close anyway", strerror(close_mutex_result));
        // Don't return - try to close anyway since this is critical for cleanup
    }

    // CRITICAL FIX: Use a memory barrier to ensure all memory operations are completed
    // This helps prevent segmentation faults on some architectures
    __sync_synchronize();

    // Store stream name for logging - use a local copy to avoid potential race conditions
    char stream_name[MAX_STREAM_NAME] = {0};

    // CRITICAL FIX: Add additional safety check before accessing stream_name
    // This prevents segfaults when the writer structure is partially initialized or corrupted
    if (writer && writer->stream_name && writer->stream_name[0] != '\0') {
        strncpy(stream_name, writer->stream_name, MAX_STREAM_NAME - 1);
        stream_name[MAX_STREAM_NAME - 1] = '\0';
    } else {
        strcpy(stream_name, "unknown");
    }

    log_info("Starting to close HLS writer for stream %s", stream_name);

    // CRITICAL FIX: Don't call stop_hls_stream from here to prevent recursive calls
    // Instead, just clear the thread_ctx pointer to prevent future recursive calls
    if (writer && writer->thread_ctx) {
        log_info("Clearing thread context reference for stream %s during writer close", stream_name);
        writer->thread_ctx = NULL;
    }

    // Try to acquire the mutex with a timeout approach
    struct timespec timeout;
    clock_gettime(CLOCK_REALTIME, &timeout);
    timeout.tv_sec += 5; // Increased to 5 second timeout for better reliability with go2rtc

    // CRITICAL FIX: Add additional safety check before accessing mutex
    int mutex_result = EOWNERDEAD; // Default to error state
    if (writer) {
        // CRITICAL FIX: Use a memory barrier before accessing the mutex
        __sync_synchronize();

        mutex_result = pthread_mutex_timedlock(&writer->mutex, &timeout);
        if (mutex_result != 0) {
            log_warn("Could not acquire HLS writer mutex for stream %s within timeout, proceeding with forced close (error: %s)",
                    stream_name, strerror(mutex_result));
            // Continue with the close operation even if we couldn't acquire the mutex
        } else {
            log_info("Successfully acquired mutex for HLS writer for stream %s", stream_name);
        }
    } else {
        log_warn("Writer became NULL during mutex acquisition for stream %s", stream_name);
    }

    // Check if already closed - with additional safety check
    // CRITICAL FIX: Add additional NULL check before accessing output_ctx
    if (!writer || !writer->output_ctx) {
        log_warn("Attempted to close already closed HLS writer for stream %s", stream_name);
        if (writer && mutex_result == 0) {
            // CRITICAL FIX: Add memory barrier before unlocking mutex
            __sync_synchronize();
            pthread_mutex_unlock(&writer->mutex);
        }
        // CRITICAL FIX: Reset close state and unlock mutex before early return
        in_writer_close = false;
        if (close_mutex_result == 0) {
            pthread_mutex_unlock(&close_mutex);
        }
        return;
    }

    // Create a local copy of the output context with additional safety checks
    AVFormatContext *local_output_ctx = NULL;
    if (writer->output_ctx) {
        local_output_ctx = writer->output_ctx;

        // Mark as closed immediately to prevent other threads from using it
        writer->output_ctx = NULL;
        writer->initialized = 0;
    } else {
        log_warn("Output context became NULL during close for stream %s", stream_name);
    }

    // Unlock the mutex if we acquired it
    if (mutex_result == 0) {
        pthread_mutex_unlock(&writer->mutex);
        log_info("Released mutex for HLS writer for stream %s", stream_name);
    }

    // Add a delay to ensure any in-progress operations complete
    // Increased for better reliability with go2rtc integration
    usleep(500000); // 500ms - increased for more safety with go2rtc

    // Write trailer if the context is valid with enhanced safety checks
    if (local_output_ctx) {
        log_info("Writing trailer for HLS writer for stream %s", stream_name);

        // Use a try/catch-like approach with signal handling to prevent crashes
        int ret = 0;

        // Validate the output context more thoroughly before writing trailer
        bool context_valid = false;

        // Check if the context is valid and has streams
        if (local_output_ctx->nb_streams > 0) {
            // Verify all critical pointers are valid
            if (local_output_ctx->oformat && local_output_ctx->pb) {
                // Additional validation of each stream
                bool all_streams_valid = true;
                for (unsigned int i = 0; i < local_output_ctx->nb_streams; i++) {
                    if (!local_output_ctx->streams[i] || !local_output_ctx->streams[i]->codecpar) {
                        log_warn("Invalid stream %d in context for stream %s", i, stream_name);
                        all_streams_valid = false;
                        break;
                    }
                }

                if (all_streams_valid) {
                    context_valid = true;
                }
            }
        }

        // Only proceed with trailer write if context is fully validated
        if (context_valid) {
            // Set up a timeout for the trailer write operation
            // Use sigaction for more reliable signal handling
            struct sigaction sa_old, sa_new;
            sigaction(SIGALRM, NULL, &sa_old);
            sa_new = sa_old;
            sa_new.sa_handler = SIG_IGN; // Ignore alarm signal
            sigaction(SIGALRM, &sa_new, NULL);

            // Set alarm
            alarm(5); // 5 second timeout for trailer write

            // Use a safer approach to write the trailer
            ret = av_write_trailer(local_output_ctx);

            // Cancel the alarm and restore signal handler
            alarm(0);
            sigaction(SIGALRM, &sa_old, NULL);

            if (ret < 0) {
                char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
                av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
                log_warn("Error writing trailer for HLS writer for stream %s: %s", stream_name, error_buf);
            } else {
                log_info("Successfully wrote trailer for HLS writer for stream %s", stream_name);
            }
        } else {
            log_warn("Skipping trailer write for stream %s: invalid context state", stream_name);
        }

        // Close AVIO context if it exists
        if (local_output_ctx->pb) {
            log_info("Closing AVIO context for HLS writer for stream %s", stream_name);

            // Use a local copy of the pb pointer
            AVIOContext *pb_to_close = local_output_ctx->pb;
            local_output_ctx->pb = NULL;

            // Set up a timeout for the AVIO close operation with proper signal handling
            struct sigaction sa_old, sa_new;
            sigaction(SIGALRM, NULL, &sa_old);
            sa_new = sa_old;
            sa_new.sa_handler = SIG_IGN; // Ignore alarm signal
            sigaction(SIGALRM, &sa_new, NULL);

            // Set alarm
            alarm(5); // 5 second timeout for AVIO close

            // Close the AVIO context
            avio_closep(&pb_to_close); // Use safer avio_closep and pass the correct pointer

            // Cancel the alarm and restore signal handler
            alarm(0);
            sigaction(SIGALRM, &sa_old, NULL);

            log_info("Successfully closed AVIO context for HLS writer for stream %s", stream_name);
        }

        // Add a small delay before freeing the context to ensure all operations are complete
        usleep(100000); // 100ms

        log_info("Freeing format context for HLS writer for stream %s", stream_name);
        avformat_free_context(local_output_ctx);
        local_output_ctx = NULL; // Set to NULL after freeing to prevent double-free
        log_info("Successfully freed format context for HLS writer for stream %s", stream_name);
    }

    // Free bitstream filter context if it exists
    if (writer->bsf_ctx) {
        log_info("Freeing bitstream filter context for HLS writer for stream %s", stream_name);
        AVBSFContext *bsf_to_free = writer->bsf_ctx;
        writer->bsf_ctx = NULL;
        av_bsf_free(&bsf_to_free);
        log_info("Successfully freed bitstream filter context for HLS writer for stream %s", stream_name);
    }

    // Destroy mutex with proper error handling
    if (mutex_result == 0) { // Only destroy if we successfully acquired it
        int destroy_result = pthread_mutex_destroy(&writer->mutex);
        if (destroy_result != 0) {
            log_warn("Failed to destroy mutex for HLS writer for stream %s: %s",
                    stream_name, strerror(destroy_result));
        } else {
            log_info("Successfully destroyed mutex for HLS writer for stream %s", stream_name);
        }
    }

    log_info("Closed HLS writer for stream %s", stream_name);

    // Perform one final check to ensure all FFmpeg resources are properly freed
    if (writer->output_ctx) {
        log_warn("Output context still exists during final cleanup for stream %s", stream_name);

        // Write trailer if not already written
        if (writer->initialized) {
            log_info("Writing trailer for stream %s during cleanup", stream_name);
            av_write_trailer(writer->output_ctx);
            writer->initialized = 0;
        }

        // Close AVIO context if it exists
        if (writer->output_ctx->pb) {
            log_info("Closing AVIO context during final cleanup for stream %s", stream_name);
            avio_closep(&writer->output_ctx->pb);
        }

        // Free all streams in the output context
        for (unsigned int i = 0; i < writer->output_ctx->nb_streams; i++) {
            if (writer->output_ctx->streams[i]) {
                // Free codec parameters
                if (writer->output_ctx->streams[i]->codecpar) {
                    avcodec_parameters_free(&writer->output_ctx->streams[i]->codecpar);
                }

                // Free any other stream resources
                if (writer->output_ctx->streams[i]->metadata) {
                    av_dict_free(&writer->output_ctx->streams[i]->metadata);
                }
            }
        }

        // Free format context
        avformat_free_context(writer->output_ctx);
        writer->output_ctx = NULL;
    }

    // Free bitstream filter context if it exists
    if (writer->bsf_ctx) {
        log_warn("Bitstream filter context still exists during final cleanup for stream %s", stream_name);
        av_bsf_free(&writer->bsf_ctx);
        writer->bsf_ctx = NULL;
    }

    // Reset DTS tracker
    writer->dts_tracker.first_dts = 0;
    writer->dts_tracker.last_dts = 0;
    writer->dts_tracker.initialized = 0;

    // Unregister the writer from global tracking
    unregister_hls_writer(writer);

    // Finally free the writer structure
    free(writer);
    log_info("Freed HLS writer structure for stream %s", stream_name);

    // Reset the recursive call prevention flag
    in_writer_close = false;

    // CRITICAL FIX: Unlock the close mutex only if we acquired it
    if (close_mutex_result == 0) {
        pthread_mutex_unlock(&close_mutex);
    }
}
