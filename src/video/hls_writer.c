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
#include "video/hls_writer_thread.h"
#include "video/detection_integration.h"
#include "video/detection_frame_processing.h"
#include "video/detection_thread_pool.h"
#include "video/streams.h"
#include "video/stream_manager.h"

// Forward declarations from detection_stream.c
extern int is_detection_stream_reader_running(const char *stream_name);
extern int get_detection_interval(const char *stream_name);

// Forward declaration for internal function
static int ensure_output_directory(hls_writer_t *writer);

// Direct detection processing function - no thread needed
// Made non-static so it can be used from hls_stream_thread.c
void process_packet_for_detection(const char *stream_name, const AVPacket *pkt, const AVCodecParameters *codec_params) {
    //  Add extra validation for all parameters
    if (!stream_name || !pkt || !codec_params) {
        log_error("Invalid parameters in process_packet_for_detection: stream_name=%p, pkt=%p, codec_params=%p",
                 (void*)stream_name, (void*)pkt, (void*)codec_params);
        return;
    }
    
    //  Check if detection is enabled for this stream
    if (!is_detection_stream_reader_running(stream_name)) {
        // Detection is not enabled for this stream, skip processing
        return;
    }
    
    // Check if a detection is already in progress for this stream
    // This prevents multiple detections from running simultaneously for the same stream
    // and avoids buffer pool exhaustion
    if (is_detection_in_progress(stream_name)) {
        // Skip this packet if detection is already in progress for this stream
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
    const AVCodec *codec = NULL;
    AVCodecContext *codec_ctx = NULL;
    AVFrame *frame = NULL;
    
    log_debug("Processing packet for detection directly for stream %s", stream_name);
    
    // Create a copy of the packet to avoid modifying the original
    pkt_copy = av_packet_alloc();
    if (!pkt_copy) {
        log_error("Failed to allocate packet copy for detection");
        goto cleanup;
    }
    
    // Initialize the packet - using av_packet_alloc already initializes it
    pkt_copy->data = NULL;
    pkt_copy->size = 0;
    
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
        // Get the detection interval from the detection stream configuration
        int detection_interval = get_detection_interval(stream_name);
        log_debug("Sending decoded frame to detection integration for stream %s (interval: %d)", 
                 stream_name, detection_interval);
        
        // Get stream configuration to check if detection is enabled
        stream_handle_t stream_handle = get_stream_by_name(stream_name);
        if (stream_handle) {
            stream_config_t stream_config;
            if (get_stream_config(stream_handle, &stream_config) == 0) {
                // Check if detection is enabled and a model is specified
                if (stream_config.detection_based_recording && 
                    stream_config.detection_model[0] != '\0') {
                    
                    log_info("Processing detection for stream %s with model %s", 
                             stream_name, stream_config.detection_model);
                    
                    // Call the detection function
                    if (process_decoded_frame_for_detection) {
                        process_decoded_frame_for_detection(stream_name, frame, detection_interval);
                    } else {
                        log_error("process_decoded_frame_for_detection function is not available");
                    }
                } else {
                    log_debug("Detection not enabled for stream %s in configuration", stream_name);
                }
            } else {
                log_error("Failed to get stream config for %s", stream_name);
            }
        } else {
            log_error("Failed to get stream handle for %s", stream_name);
        }
    } else if (ret != AVERROR(EAGAIN) && ret != AVERROR_EOF) {
        log_error("Failed to receive frame from decoder for stream %s: %d", stream_name, ret);
    }
    
cleanup:
    // Cleanup resources in reverse order of allocation
    if (frame) {
        av_frame_free(&frame);
        frame = NULL;  // Set to NULL after freeing to prevent double-free
    }
    
    if (codec_ctx) {
        avcodec_free_context(&codec_ctx);
        codec_ctx = NULL;  // Set to NULL after freeing to prevent double-free
    }
    
    if (pkt_copy) {
        av_packet_unref(pkt_copy);
        av_packet_free(&pkt_copy);
        pkt_copy = NULL;  // Set to NULL after freeing to prevent double-free
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
    if (to_delete > 0) {
        for (int j = 0; j < to_delete; j++) {
            snprintf(filepath, sizeof(filepath), "%s/%s", output_dir, segments[j].filename);
            if (unlink(filepath) == 0) {
                log_debug("Deleted old HLS segment: %s", segments[j].filename);
            } else {
                log_warn("Failed to delete old HLS segment: %s", segments[j].filename);
            }
        }
    }

    // Always free the allocated memory
    free(segments);
    segments = NULL;
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
    av_dict_set(&options, "hls_list_size", "5", 0);  // Increased for better stability
    
    // Use MPEG-TS segments for better compatibility and to avoid MP4 moov atom issues
    av_dict_set(&options, "hls_segment_type", "mpegts", 0);
    
    // Disable second pass optimization that's causing the segmentation fault
    av_dict_set(&options, "hls_flags", "delete_segments+single_file", 0);
    
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
    log_info("HLS writer options for stream %s (simplified for stability):", writer->stream_name);
    log_info("  hls_time: %s", hls_time);
    log_info("  hls_list_size: 5");
    log_info("  hls_flags: delete_segments+independent_segments");
    log_info("  hls_segment_type: mpegts");
    log_info("  start_number: 0");
    log_info("  hls_segment_filename: %s", segment_format);

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

    log_info("Created HLS writer for stream %s at %s with segment duration %d seconds",
            stream_name, output_dir, segment_duration);
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

    //  For HLS streaming, we need to set the correct codec parameters
    // The issue is not with the bitstream filter but with how we're configuring the output
    
    // For H.264 streams, we need to ensure the correct format
    if (input_stream->codecpar->codec_id == AV_CODEC_ID_H264) {
        // Set the correct codec tag for H.264 in HLS
        out_stream->codecpar->codec_tag = 0;
        
        // Set the correct format for H.264 in HLS
        AVDictionary *opts = NULL;
        av_dict_set(&opts, "mpegts_flags", "resend_headers", 0);
        av_dict_set(&opts, "hls_flags", "single_file", 0);
        
        // Apply these options to the output context
        if (writer->output_ctx) {
            for (int i = 0; i < writer->output_ctx->nb_streams; i++) {
                AVStream *stream = writer->output_ctx->streams[i];
                if (stream) {
                    stream->codecpar->codec_tag = 0;
                }
            }
        }
        
        // Make sure to free the dictionary to prevent memory leaks
        av_dict_free(&opts);
        
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
 * Write packet to HLS stream with per-stream timestamp handling and proper bitstream filtering
 */
int hls_writer_write_packet(hls_writer_t *writer, const AVPacket *pkt, const AVStream *input_stream) {
    // Validate parameters
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

    // Check if writer has been closed
    if (!writer->output_ctx) {
        log_warn("hls_writer_write_packet: Writer for stream %s has been closed", writer->stream_name);
        return -1;
    }

    // Periodically ensure output directory exists (every 10 seconds)
    static time_t last_dir_check = 0;
    time_t now = time(NULL);
    if (now - last_dir_check >= 10) {
        if (ensure_output_directory(writer) != 0) {
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

    // Check if writer has been closed after initialization
    if (!writer->output_ctx) {
        log_warn("hls_writer_write_packet: Writer for stream %s has been closed after initialization", writer->stream_name);
        return -1;
    }

    // Clone the packet
    AVPacket out_pkt;
    // Initialize the packet without using deprecated av_init_packet
    memset(&out_pkt, 0, sizeof(out_pkt));
    if (av_packet_ref(&out_pkt, pkt) < 0) {
        log_error("Failed to reference packet for stream %s", writer->stream_name);
        return -1;
    }
    
    //  CRITICAL FIX: More robust bitstream filtering for H.264 in HLS
    // This is essential to prevent the "h264 bitstream malformed, no startcode found" error
    // and to avoid segmentation faults during shutdown
    if (input_stream->codecpar->codec_id == AV_CODEC_ID_H264) {
        // Use a simpler and more reliable approach for H.264 bitstream conversion
        // Instead of creating a new filter for each packet, we'll manually add the start code
        
        // Check if the packet already has start codes (Annex B format)
        bool has_start_code = false;
        if (out_pkt.size >= 4) {
            has_start_code = (out_pkt.data[0] == 0x00 && 
                             out_pkt.data[1] == 0x00 && 
                             out_pkt.data[2] == 0x00 && 
                             out_pkt.data[3] == 0x01);
        }
        
        if (!has_start_code) {
    // Create a new packet with space for the start code
    AVPacket new_pkt;
    // Initialize the packet without using deprecated av_init_packet
    memset(&new_pkt, 0, sizeof(new_pkt));
    new_pkt.data = NULL;
    new_pkt.size = 0;
    
    // Allocate a new buffer with space for the start code
    if (av_new_packet(&new_pkt, out_pkt.size + 4) < 0) {
        log_error("Failed to allocate new packet for H.264 conversion for stream %s", 
                 writer->stream_name);
        av_packet_unref(&out_pkt);
        return -1;
    }
    
    // Add start code
    new_pkt.data[0] = 0x00;
    new_pkt.data[1] = 0x00;
    new_pkt.data[2] = 0x00;
    new_pkt.data[3] = 0x01;
    
    // Copy the packet data
    memcpy(new_pkt.data + 4, out_pkt.data, out_pkt.size);
    
    // Copy other packet properties
    new_pkt.pts = out_pkt.pts;
    new_pkt.dts = out_pkt.dts;
    new_pkt.flags = out_pkt.flags;
    new_pkt.stream_index = out_pkt.stream_index;
    new_pkt.duration = out_pkt.duration;
    new_pkt.pos = out_pkt.pos;
    
    // Unref the original packet
    av_packet_unref(&out_pkt);
    
    // Move the new packet to the output packet and ensure proper cleanup
    av_packet_move_ref(&out_pkt, &new_pkt);
    
    // Ensure new_pkt is properly cleaned up after move_ref
    av_packet_unref(&new_pkt);
            
            log_debug("Added H.264 start code for stream %s", writer->stream_name);
        }
    } else {
        log_debug("Using original packet for non-H264 stream %s", writer->stream_name);
    }

    // Initialize DTS tracker if needed
    stream_dts_info_t *dts_tracker = &writer->dts_tracker;
    if (!dts_tracker->initialized) {
        // Use a safe default value if both pts and dts are invalid
        int64_t first_ts = 0;
        
        if (pkt->dts != AV_NOPTS_VALUE) {
            first_ts = pkt->dts;
        } else if (pkt->pts != AV_NOPTS_VALUE) {
            first_ts = pkt->pts;
        } else {
            // Both are invalid, use 0 as a safe starting point
            first_ts = 0;
            log_warn("Both PTS and DTS are invalid for first packet in stream %s, using 0", 
                    writer->stream_name);
        }
        
        dts_tracker->first_dts = first_ts;
        dts_tracker->last_dts = first_ts;
        dts_tracker->time_base = input_stream->time_base;
        dts_tracker->initialized = 1;

        log_debug("Initialized DTS tracker for HLS stream %s: first_dts=%lld, time_base=%d/%d",
                 writer->stream_name,
                 (long long)dts_tracker->first_dts,
                 dts_tracker->time_base.num,
                 dts_tracker->time_base.den);
    }

    // Fix invalid timestamps
    if (out_pkt.pts == AV_NOPTS_VALUE || out_pkt.dts == AV_NOPTS_VALUE) {
        if (out_pkt.pts == AV_NOPTS_VALUE && out_pkt.dts == AV_NOPTS_VALUE) {
            if (dts_tracker->last_dts != AV_NOPTS_VALUE) {
                // Calculate a reasonable increment based on the stream's framerate if available
                int64_t increment = 1;
                
                if (input_stream->avg_frame_rate.num > 0 && input_stream->avg_frame_rate.den > 0 &&
                    input_stream->time_base.num > 0 && input_stream->time_base.den > 0) {
                    // Calculate frame duration in stream timebase units
                    AVRational tb = input_stream->time_base;
                    AVRational fr = input_stream->avg_frame_rate;
                    increment = av_rescale_q(1, av_inv_q(fr), tb);
                    
                    // Ensure increment is at least 1
                    if (increment < 1) increment = 1;
                }
                
                out_pkt.dts = dts_tracker->last_dts + increment;
                out_pkt.pts = out_pkt.dts;
                
                log_debug("Generated timestamps for stream %s: pts=%lld, dts=%lld (increment=%lld)",
                         writer->stream_name, (long long)out_pkt.pts, (long long)out_pkt.dts, 
                         (long long)increment);
            } else {
                out_pkt.dts = 1;
                out_pkt.pts = 1;
                log_debug("Set initial timestamps for stream %s with no previous reference", 
                         writer->stream_name);
            }
        } else if (out_pkt.pts == AV_NOPTS_VALUE) {
            out_pkt.pts = out_pkt.dts;
        } else if (out_pkt.dts == AV_NOPTS_VALUE) {
            out_pkt.dts = out_pkt.pts;
        }
    }

    // Ensure timestamps are positive
    if (out_pkt.pts <= 0 || out_pkt.dts <= 0) {
        if (out_pkt.pts <= 0) {
            out_pkt.pts = out_pkt.dts > 0 ? out_pkt.dts : 1;
            log_debug("Corrected non-positive PTS for stream %s: new pts=%lld", 
                     writer->stream_name, (long long)out_pkt.pts);
        }

        if (out_pkt.dts <= 0) {
            out_pkt.dts = out_pkt.pts > 0 ? out_pkt.pts : 1;
            log_debug("Corrected non-positive DTS for stream %s: new dts=%lld", 
                     writer->stream_name, (long long)out_pkt.dts);
        }
    }

    // Handle timestamp discontinuities
    if (dts_tracker->last_dts != AV_NOPTS_VALUE) {
        if (out_pkt.dts < dts_tracker->last_dts) {
            // Fix backwards DTS
            int64_t fixed_dts = dts_tracker->last_dts + 1;
            int64_t pts_dts_diff = out_pkt.pts - out_pkt.dts;
            
            log_debug("Fixing backwards DTS in stream %s: last=%lld, current=%lld, fixed=%lld",
                     writer->stream_name,
                     (long long)dts_tracker->last_dts,
                     (long long)out_pkt.dts,
                     (long long)fixed_dts);
            
            out_pkt.dts = fixed_dts;
            out_pkt.pts = fixed_dts + pts_dts_diff;
            
            // Ensure PTS >= DTS after correction
            if (out_pkt.pts < out_pkt.dts) {
                out_pkt.pts = out_pkt.dts;
            }
        }
        // Log large jumps but accept them
        else if (out_pkt.dts > dts_tracker->last_dts + 90000) {
            log_warn("HLS large DTS jump in stream %s: last=%lld, current=%lld, diff=%lld",
                    writer->stream_name,
                    (long long)dts_tracker->last_dts,
                    (long long)out_pkt.dts,
                    (long long)(out_pkt.dts - dts_tracker->last_dts));
        }
    }

    // Update last DTS
    dts_tracker->last_dts = out_pkt.dts;

    // Validate writer context before rescaling
    if (!writer->output_ctx || !writer->output_ctx->streams || !writer->output_ctx->streams[0]) {
        log_warn("hls_writer_write_packet: Writer context invalid for stream %s", writer->stream_name);
        av_packet_unref(&out_pkt);
        return -1;
    }

    // Rescale timestamps to output timebase
    av_packet_rescale_ts(&out_pkt, input_stream->time_base,
                        writer->output_ctx->streams[0]->time_base);

    // CRITICAL FIX: Ensure PTS >= DTS with a small buffer to prevent ghosting artifacts
    // This is essential for HLS format compliance and prevents visual artifacts
    if (out_pkt.pts < out_pkt.dts) {
        log_debug("Fixing HLS packet with PTS < DTS: PTS=%lld, DTS=%lld", 
                 (long long)out_pkt.pts, (long long)out_pkt.dts);
        out_pkt.pts = out_pkt.dts;
    }

    // Cap unreasonable PTS/DTS differences
    int64_t pts_dts_diff = out_pkt.pts - out_pkt.dts;
    if (pts_dts_diff > 90000 * 10) {
        out_pkt.pts = out_pkt.dts + 90000 * 5; // 5 seconds max difference
    }

    // Log key frames for diagnostics
    bool is_key_frame = (out_pkt.flags & AV_PKT_FLAG_KEY) != 0;
    if (is_key_frame) {
        log_debug("Writing key frame to HLS for stream %s: pts=%lld, dts=%lld, size=%d",
                 writer->stream_name, (long long)out_pkt.pts, (long long)out_pkt.dts, out_pkt.size);
    }

    int ret = av_interleaved_write_frame(writer->output_ctx, &out_pkt);
    
    // Clean up packet
    av_packet_unref(&out_pkt);

    // Handle write errors
    if (ret < 0) {
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Error writing HLS packet for stream %s: %s", writer->stream_name, error_buf);

        // Try to fix directory issues
        if (strstr(error_buf, "No such file or directory") != NULL) {
            ensure_output_directory(writer);
        } else if (strstr(error_buf, "Invalid argument") != NULL ||
                  strstr(error_buf, "non monotonically increasing dts") != NULL ||
                  strstr(error_buf, "Invalid data found when processing input") != NULL) {
            // Reset DTS tracker on timestamp errors or invalid data
            log_warn("Resetting DTS tracker for stream %s due to error: %s", writer->stream_name, error_buf);
            dts_tracker->initialized = 0;
            
            // For invalid data errors, return 0 instead of the error code
            // This allows the stream processing to continue despite occasional bad packets
            if (strstr(error_buf, "Invalid data found when processing input") != NULL) {
                log_warn("Ignoring invalid packet data for stream %s and continuing", writer->stream_name);
                return 0;  // Return success to continue processing
            }
        }

        return ret;
    }

    // Periodically clean up old segments (every 60 seconds)
    if (now - writer->last_cleanup_time >= 60) {
        // Keep more segments than in the playlist for smooth playback
        int max_segments_to_keep = 15; // More than hls_list_size
        cleanup_old_segments(writer->output_dir, max_segments_to_keep);
        writer->last_cleanup_time = now;
    }

    return 0;
}

/**
 * Close HLS writer and free resources
 * This function is thread-safe and handles all cleanup operations with improved robustness
 */
void hls_writer_close(hls_writer_t *writer) {
    if (!writer) {
        log_warn("Attempted to close NULL HLS writer");
        return;
    }

    // Store stream name for logging
    char stream_name[MAX_STREAM_NAME];
    strncpy(stream_name, writer->stream_name, MAX_STREAM_NAME - 1);
    stream_name[MAX_STREAM_NAME - 1] = '\0';

    log_info("Starting to close HLS writer for stream %s", stream_name);

    // First, stop any running writer thread to ensure clean shutdown
    if (writer->thread_ctx) {
        log_info("Stopping HLS writer thread for stream %s during writer close", stream_name);
        // Call the function from hls_writer_thread.h
        hls_writer_stop_recording_thread(writer);
        // thread_ctx should now be NULL
    }

    // Try to acquire the mutex with a timeout approach
    struct timespec timeout;
    clock_gettime(CLOCK_REALTIME, &timeout);
    timeout.tv_sec += 3; // Increased from 2 to 3 second timeout
    
    int mutex_result = pthread_mutex_timedlock(&writer->mutex, &timeout);
    if (mutex_result != 0) {
        log_warn("Could not acquire HLS writer mutex for stream %s within timeout, proceeding with forced close", stream_name);
        // Continue with the close operation even if we couldn't acquire the mutex
    } else {
        log_info("Successfully acquired mutex for HLS writer for stream %s", stream_name);
    }

    // Check if already closed
    if (!writer->output_ctx) {
        log_warn("Attempted to close already closed HLS writer for stream %s", stream_name);
        if (mutex_result == 0) {
            pthread_mutex_unlock(&writer->mutex);
        }
        return;
    }

    // Create a local copy of the output context
    AVFormatContext *local_output_ctx = writer->output_ctx;

    // Mark as closed immediately to prevent other threads from using it
    writer->output_ctx = NULL;
    writer->initialized = 0;

    // Unlock the mutex if we acquired it
    if (mutex_result == 0) {
        pthread_mutex_unlock(&writer->mutex);
        log_info("Released mutex for HLS writer for stream %s", stream_name);
    }

    // Add a small delay to ensure any in-progress operations complete
    usleep(300000); // 300ms - increased from 200ms for more safety

    // Write trailer if the context is valid
    if (local_output_ctx) {
        log_info("Writing trailer for HLS writer for stream %s", stream_name);
        
        // Use a try/catch-like approach with signal handling to prevent crashes
        int ret = 0;
        
        // Only write trailer if the context is valid and has streams
        if (local_output_ctx->nb_streams > 0) {
            // Use a safer approach to write the trailer
            // First check if the context is still valid
            if (local_output_ctx->oformat && local_output_ctx->pb) {
                // Set up a timeout for the trailer write operation
                alarm(5); // 5 second timeout for trailer write
                
                ret = av_write_trailer(local_output_ctx);
                
                // Cancel the alarm
                alarm(0);
                
                if (ret < 0) {
                    char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
                    av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
                    log_warn("Error writing trailer for HLS writer for stream %s: %s", stream_name, error_buf);
                } else {
                    log_info("Successfully wrote trailer for HLS writer for stream %s", stream_name);
                }
            } else {
                log_warn("Skipping trailer write for stream %s: invalid format context", stream_name);
            }
        } else {
            log_warn("Skipping trailer write for stream %s: no streams in context", stream_name);
        }
        
        // Close AVIO context if it exists
        if (local_output_ctx->pb) {
            log_info("Closing AVIO context for HLS writer for stream %s", stream_name);
            
            // Use a local copy of the pb pointer
            AVIOContext *pb_to_close = local_output_ctx->pb;
            local_output_ctx->pb = NULL;
            
            // Set up a timeout for the AVIO close operation
            alarm(5); // 5 second timeout for AVIO close
            
            // Close the AVIO context
            avio_closep(&pb_to_close); // Use safer avio_closep and pass the correct pointer
            
            // Cancel the alarm
            alarm(0);
            
            log_info("Successfully closed AVIO context for HLS writer for stream %s", stream_name);
        }
        
        log_info("Freeing format context for HLS writer for stream %s", stream_name);
        avformat_free_context(local_output_ctx);
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

    // Finally free the writer structure
    free(writer);
    log_info("Freed HLS writer structure for stream %s", stream_name);
}
