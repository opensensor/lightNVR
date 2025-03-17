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

// Direct detection processing function - no thread needed
static void process_packet_for_detection(const char *stream_name, const AVPacket *pkt, const AVCodecParameters *codec_params) {
    // Use a static flag to prevent recursive calls and potential stack overflow
    static int detection_in_progress = 0;
    
    // Skip if detection is already in progress to prevent recursive calls
    if (detection_in_progress) {
        return;
    }
    
    detection_in_progress = 1;
    
    log_debug("Processing packet for detection directly for stream %s", stream_name);
    
    // Create a copy of the packet to avoid modifying the original
    AVPacket *pkt_copy = av_packet_alloc();
    if (!pkt_copy) {
        log_error("Failed to allocate packet copy for detection");
        detection_in_progress = 0;
        return;
    }
    
    if (av_packet_ref(pkt_copy, pkt) < 0) {
        log_error("Failed to reference packet for detection");
        av_packet_free(&pkt_copy);
        detection_in_progress = 0;
        return;
    }
    
    // Find decoder
    AVCodec *codec = avcodec_find_decoder(codec_params->codec_id);
    if (!codec) {
        log_error("Failed to find decoder for stream %s", stream_name);
        av_packet_unref(pkt_copy);
        av_packet_free(&pkt_copy);
        detection_in_progress = 0;
        return;
    }
    
    // Create codec context
    AVCodecContext *codec_ctx = avcodec_alloc_context3(codec);
    if (!codec_ctx) {
        log_error("Failed to allocate codec context for stream %s", stream_name);
        av_packet_unref(pkt_copy);
        av_packet_free(&pkt_copy);
        detection_in_progress = 0;
        return;
    }
    
    // Copy codec parameters to codec context
    if (avcodec_parameters_to_context(codec_ctx, codec_params) < 0) {
        log_error("Failed to copy codec parameters to context for stream %s", stream_name);
        avcodec_free_context(&codec_ctx);
        av_packet_unref(pkt_copy);
        av_packet_free(&pkt_copy);
        detection_in_progress = 0;
        return;
    }
    
    // Open codec
    if (avcodec_open2(codec_ctx, codec, NULL) < 0) {
        log_error("Failed to open codec for stream %s", stream_name);
        avcodec_free_context(&codec_ctx);
        av_packet_unref(pkt_copy);
        av_packet_free(&pkt_copy);
        detection_in_progress = 0;
        return;
    }
    
    // Allocate frame
    AVFrame *frame = av_frame_alloc();
    if (!frame) {
        log_error("Failed to allocate frame for stream %s", stream_name);
        avcodec_free_context(&codec_ctx);
        av_packet_unref(pkt_copy);
        av_packet_free(&pkt_copy);
        detection_in_progress = 0;
        return;
    }
    
    // Send packet to decoder
    if (avcodec_send_packet(codec_ctx, pkt_copy) < 0) {
        log_error("Failed to send packet to decoder for stream %s", stream_name);
        av_frame_free(&frame);
        avcodec_free_context(&codec_ctx);
        av_packet_unref(pkt_copy);
        av_packet_free(&pkt_copy);
        detection_in_progress = 0;
        return;
    }
    
    // Receive frame from decoder
    int ret = avcodec_receive_frame(codec_ctx, frame);
    if (ret >= 0) {
        // Process the decoded frame for detection
        // Use a smaller detection interval (15) to improve detection quality
        // while still maintaining reasonable performance
        int detection_interval = 15;
        log_debug("Sending decoded frame to detection integration for stream %s", stream_name);
        process_decoded_frame_for_detection(stream_name, frame, detection_interval);
    } else if (ret != AVERROR(EAGAIN) && ret != AVERROR_EOF) {
        log_error("Failed to receive frame from decoder for stream %s: %d", stream_name, ret);
    }
    
    // Cleanup
    av_frame_free(&frame);
    avcodec_free_context(&codec_ctx);
    av_packet_unref(pkt_copy);
    av_packet_free(&pkt_copy);
    
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

/**
 * Create new HLS writer
 */
hls_writer_t *hls_writer_create(const char *output_dir, const char *stream_name, int segment_duration) {
    // Allocate writer structure
    hls_writer_t *writer = (hls_writer_t *)malloc(sizeof(hls_writer_t));
    if (!writer) {
        log_error("Failed to allocate HLS writer");
        return NULL;
    }

    // Initialize structure
    memset(writer, 0, sizeof(hls_writer_t));

    // Copy output directory and stream name
    strncpy(writer->output_dir, output_dir, MAX_PATH_LENGTH - 1);
    strncpy(writer->stream_name, stream_name, MAX_STREAM_NAME - 1);
    writer->segment_duration = segment_duration;
    writer->last_cleanup_time = time(NULL);

    // Initialize DTS tracker
    writer->dts_tracker.initialized = 0;
    
    // Initialize pressure indicator
    writer->is_under_pressure = 0;
    
    // Initialize frame counter
    writer->frame_counter = 0;

    // Create output directory if it doesn't exist
    // Use the configured storage path to avoid writing to overlay
    char mkdir_cmd[MAX_PATH_LENGTH + 10];
    
    // Ensure we're using a path within our configured storage
    extern config_t global_config;
    char safe_output_dir[MAX_PATH_LENGTH];
    
    // If output_dir is absolute and not within storage_path, redirect it
    if (output_dir[0] == '/' && strncmp(output_dir, global_config.storage_path, strlen(global_config.storage_path)) != 0) {
        // Create a path within our storage directory instead
        snprintf(safe_output_dir, sizeof(safe_output_dir), "%s/hls/%s", 
                global_config.storage_path, stream_name);
        log_warn("Redirecting HLS output from %s to %s to avoid overlay writes", 
                output_dir, safe_output_dir);
        strncpy(writer->output_dir, safe_output_dir, MAX_PATH_LENGTH - 1);
        writer->output_dir[MAX_PATH_LENGTH - 1] = '\0';
        output_dir = safe_output_dir;
    } else {
        strncpy(safe_output_dir, output_dir, MAX_PATH_LENGTH - 1);
        safe_output_dir[MAX_PATH_LENGTH - 1] = '\0';
    }
    
    // Create the HLS directory and ensure it has proper permissions
    snprintf(mkdir_cmd, sizeof(mkdir_cmd), "mkdir -p %s", output_dir);
    int ret_mkdir = system(mkdir_cmd);
    if (ret_mkdir != 0) {
        log_warn("Failed to create directory: %s (return code: %d)", output_dir, ret_mkdir);
    }
    
    // Set full permissions to ensure FFmpeg can write files
    snprintf(mkdir_cmd, sizeof(mkdir_cmd), "chmod -R 777 %s", output_dir);
    system(mkdir_cmd);
    
    // Also ensure the parent directory exists and is writable
    char parent_dir[MAX_PATH_LENGTH];
    const char *last_slash = strrchr(output_dir, '/');
    if (last_slash) {
        size_t parent_len = last_slash - output_dir;
        strncpy(parent_dir, output_dir, parent_len);
        parent_dir[parent_len] = '\0';
        
        snprintf(mkdir_cmd, sizeof(mkdir_cmd), "mkdir -p %s && chmod -R 777 %s", 
                parent_dir, parent_dir);
        system(mkdir_cmd);
        log_info("Ensured parent directory exists with proper permissions: %s", parent_dir);
    }

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

    // Set HLS options
    AVDictionary *options = NULL;
    char hls_time[16];
    snprintf(hls_time, sizeof(hls_time), "%d", segment_duration);

    av_dict_set(&options, "hls_time", hls_time, 0);
    av_dict_set(&options, "hls_list_size", "8", 0);  // Increased from 5 to 8 for better buffering on low-power devices
    av_dict_set(&options, "hls_flags", "delete_segments+append_list+discont_start+split_by_time+program_date_time", 0);
    av_dict_set(&options, "hls_allow_cache", "1", 0);  // Enable caching to improve playback on low-power devices
    av_dict_set(&options, "hls_segment_type", "mpegts", 0);  // Ensure MPEG-TS segments for better compatibility
    
    // Ensure the manifest file is properly initialized with EXTM3U delimiter
    // Create a minimal valid manifest file if it doesn't exist
    char manifest_path[MAX_PATH_LENGTH];
    snprintf(manifest_path, MAX_PATH_LENGTH, "%s/index.m3u8", output_dir);
    
    // Check if the manifest file exists and is valid
    FILE *manifest_check = fopen(manifest_path, "r");
    bool create_initial_manifest = false;
    
    if (manifest_check) {
        // Check if the file is empty or doesn't contain EXTM3U
        fseek(manifest_check, 0, SEEK_END);
        long size = ftell(manifest_check);
        
        if (size == 0) {
            create_initial_manifest = true;
        } else {
            // Check for EXTM3U
            char buffer[16];
            fseek(manifest_check, 0, SEEK_SET);
            size_t read_size = fread(buffer, 1, sizeof(buffer) - 1, manifest_check);
            buffer[read_size] = '\0';
            
            if (strstr(buffer, "#EXTM3U") == NULL) {
                create_initial_manifest = true;
            }
        }
        fclose(manifest_check);
    } else {
        create_initial_manifest = true;
    }
    
    // Create a minimal valid manifest file if needed
    if (create_initial_manifest) {
        FILE *manifest_init = fopen(manifest_path, "w");
        if (manifest_init) {
            // Write a minimal valid HLS manifest
            fprintf(manifest_init, "#EXTM3U\n");
            fprintf(manifest_init, "#EXT-X-VERSION:3\n");
            fprintf(manifest_init, "#EXT-X-TARGETDURATION:%d\n", segment_duration);
            fprintf(manifest_init, "#EXT-X-MEDIA-SEQUENCE:0\n");
            fclose(manifest_init);
            
            log_info("Created initial valid HLS manifest file: %s", manifest_path);
        } else {
            log_error("Failed to create initial HLS manifest file: %s", manifest_path);
        }
    }

    // Remove the delete_segments flag to prevent FFmpeg from automatically deleting segments
    // This will help prevent issues with the index.m3u8.tmp file not being able to be renamed
    // We'll handle segment cleanup ourselves in cleanup_old_segments

    // Create segment filename format string
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

    // Free options
    av_dict_free(&options);

    log_info("Created HLS writer for stream %s at %s", stream_name, output_dir);
    return writer;
}


/**
 * Initialize HLS writer with stream information
 */
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

    // Write stream header
    ret = avformat_write_header(writer->output_ctx, NULL);
    if (ret < 0) {
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Failed to write HLS header: %s", error_buf);
        return ret;
    }

    writer->initialized = 1;
    log_info("Initialized HLS writer for stream %s", writer->stream_name);
    return 0;
}


/**
 * Ensure the output directory exists and is writable
 */
static int ensure_output_directory(const char *dir_path) {
    struct stat st;
    extern config_t global_config;
    char safe_dir_path[MAX_PATH_LENGTH];
    
    // Ensure we're using a path within our configured storage
    if (dir_path[0] == '/' && strncmp(dir_path, global_config.storage_path, strlen(global_config.storage_path)) != 0) {
        // Extract stream name from the path (last component)
        const char *last_slash = strrchr(dir_path, '/');
        const char *stream_name = last_slash ? last_slash + 1 : dir_path;
        
        // Create a path within our storage directory instead
        snprintf(safe_dir_path, sizeof(safe_dir_path), "%s/hls/%s", 
                global_config.storage_path, stream_name);
        log_warn("Redirecting HLS directory from %s to %s to avoid overlay writes", 
                dir_path, safe_dir_path);
        dir_path = safe_dir_path;
    }

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
    if (!writer || !pkt || !input_stream) {
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

    // Clone the packet as we'll need to modify it
    AVPacket out_pkt;
    if (av_packet_ref(&out_pkt, pkt) < 0) {
        log_error("Failed to reference packet");
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

    // Check for timestamp discontinuities before rescaling
    if (pkt->dts != AV_NOPTS_VALUE) {
        // If DTS goes backwards or jumps significantly, handle it
        if (pkt->dts < dts_tracker->last_dts) {
            log_warn("HLS DTS discontinuity in stream %s: last=%lld, current=%lld, diff=%lld",
                    writer->stream_name,
                    (long long)dts_tracker->last_dts,
                    (long long)pkt->dts,
                    (long long)(pkt->dts - dts_tracker->last_dts));

            // Fix the DTS to ensure monotonic increase
            int64_t fixed_dts = dts_tracker->last_dts + 1;

            // Adjust PTS relative to the fixed DTS if needed
            if (pkt->pts != AV_NOPTS_VALUE) {
                int64_t pts_dts_diff = pkt->pts - pkt->dts;
                out_pkt.pts = fixed_dts + pts_dts_diff;
            }

            out_pkt.dts = fixed_dts;
        }

        // Update last DTS for next comparison
        dts_tracker->last_dts = out_pkt.dts;
    }

    // Adjust timestamps
    av_packet_rescale_ts(&out_pkt, input_stream->time_base,
                        writer->output_ctx->streams[0]->time_base);

    // Final sanity check - ensure PTS >= DTS
    if (out_pkt.pts != AV_NOPTS_VALUE && out_pkt.dts != AV_NOPTS_VALUE &&
        out_pkt.pts < out_pkt.dts) {
        out_pkt.pts = out_pkt.dts;
    }

    // Detection processing has been moved to a dedicated detection thread
    // This direct processing is now disabled to avoid duplicate detection processing

    // Write the packet
    int ret = av_interleaved_write_frame(writer->output_ctx, &out_pkt);
    av_packet_unref(&out_pkt);

    if (ret < 0) {
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Error writing HLS packet for stream %s: %s", writer->stream_name, error_buf);

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
        return;
    }

    // Write trailer if initialized
    if (writer->initialized && writer->output_ctx) {
        av_write_trailer(writer->output_ctx);
    }

    // Close output context
    if (writer->output_ctx) {
        if (writer->output_ctx->pb) {
            avio_close(writer->output_ctx->pb);
        }
        avformat_free_context(writer->output_ctx);
    }

    log_info("Closed HLS writer for stream %s", writer->stream_name);

    // Free writer
    free(writer);
}
