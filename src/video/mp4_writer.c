/**
 * Improved implementation of MP4 writer for storing camera streams
 * Save this file as src/video/mp4_writer.c
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <dirent.h>
#include <time.h>
#include <sys/stat.h>
#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <libavutil/avutil.h>
#include <libavutil/opt.h>
#include <libavutil/time.h>
#include <pthread.h>

#include "database/database_manager.h"
#include "core/config.h"
#include "core/logger.h"
#include "video/stream_manager.h"
#include "video/streams.h"
#include "video/mp4_writer.h"

extern pthread_mutex_t recordings_mutex;
extern active_recording_t active_recordings[MAX_STREAMS];
extern pthread_mutex_t recordings_mutex;


/**
 * Create a new MP4 writer
 */
mp4_writer_t *mp4_writer_create(const char *output_path, const char *stream_name) {
    mp4_writer_t *writer = calloc(1, sizeof(mp4_writer_t));
    if (!writer) {
        log_error("Failed to allocate memory for MP4 writer");
        return NULL;
    }

    // Initialize writer
    strncpy(writer->output_path, output_path, sizeof(writer->output_path) - 1);
    strncpy(writer->stream_name, stream_name, sizeof(writer->stream_name) - 1);
    writer->first_dts = AV_NOPTS_VALUE;
    writer->last_dts = AV_NOPTS_VALUE;
    writer->is_initialized = 0;
    writer->creation_time = time(NULL);

    log_info("Created MP4 writer for stream %s at %s", stream_name, output_path);

    return writer;
}

/**
 * Enhanced MP4 writer initialization with better path handling and logging
 */
static int mp4_writer_initialize(mp4_writer_t *writer, const AVPacket *pkt, const AVStream *input_stream) {
    int ret;

    // First, ensure the directory exists
    char dir_path[1024];
    const char *last_slash = strrchr(writer->output_path, '/');
    if (last_slash) {
        size_t dir_len = last_slash - writer->output_path;
        strncpy(dir_path, writer->output_path, dir_len);
        dir_path[dir_len] = '\0';

        // Log the directory we're working with
        log_info("Ensuring MP4 output directory exists: %s", dir_path);

        // Create directory if it doesn't exist
        char mkdir_cmd[1024 + 10];
        snprintf(mkdir_cmd, sizeof(mkdir_cmd), "mkdir -p %s", dir_path);
        system(mkdir_cmd);

        // Set permissions to ensure it's writable
        snprintf(mkdir_cmd, sizeof(mkdir_cmd), "chmod -R 777 %s", dir_path);
        system(mkdir_cmd);
    }

    // Log the full output path
    log_info("Initializing MP4 writer to output file: %s", writer->output_path);

    // Create output format context
    ret = avformat_alloc_output_context2(&writer->output_ctx, NULL, "mp4", writer->output_path);
    if (ret < 0 || !writer->output_ctx) {
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Failed to create output format context for MP4 writer: %s", error_buf);
        return -1;
    }

    // Add video stream
    AVStream *out_stream = avformat_new_stream(writer->output_ctx, NULL);
    if (!out_stream) {
        log_error("Failed to create output stream for MP4 writer");
        avformat_free_context(writer->output_ctx);
        writer->output_ctx = NULL;
        return -1;
    }

    // Copy codec parameters
    ret = avcodec_parameters_copy(out_stream->codecpar, input_stream->codecpar);
    if (ret < 0) {
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Failed to copy codec parameters for MP4 writer: %s", error_buf);
        avformat_free_context(writer->output_ctx);
        writer->output_ctx = NULL;
        return -1;
    }

    // Set stream time base
    out_stream->time_base = input_stream->time_base;
    writer->time_base = input_stream->time_base;

    // Store video stream index
    writer->video_stream_idx = 0;  // We only have one stream

    // Add metadata
    av_dict_set(&writer->output_ctx->metadata, "title", writer->stream_name, 0);
    av_dict_set(&writer->output_ctx->metadata, "encoder", "LightNVR", 0);

    // Set options for fast start (moov atom at beginning of file)
    AVDictionary *opts = NULL;
    av_dict_set(&opts, "movflags", "+faststart", 0);  // Ensure moov atom is at start of file

    // Open output file
    ret = avio_open(&writer->output_ctx->pb, writer->output_path, AVIO_FLAG_WRITE);
    if (ret < 0) {
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Failed to open output file for MP4 writer: %s (error: %s)",
                writer->output_path, error_buf);

        // Try to diagnose the issue
        struct stat st;
        if (stat(dir_path, &st) != 0) {
            log_error("Directory does not exist: %s", dir_path);
        } else if (!S_ISDIR(st.st_mode)) {
            log_error("Path exists but is not a directory: %s", dir_path);
        } else if (access(dir_path, W_OK) != 0) {
            log_error("Directory is not writable: %s", dir_path);
        }

        avformat_free_context(writer->output_ctx);
        writer->output_ctx = NULL;
        av_dict_free(&opts);
        return -1;
    }

    // Write file header
    ret = avformat_write_header(writer->output_ctx, &opts);
    if (ret < 0) {
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Failed to write header for MP4 writer: %s", error_buf);
        avio_closep(&writer->output_ctx->pb);
        avformat_free_context(writer->output_ctx);
        writer->output_ctx = NULL;
        av_dict_free(&opts);
        return -1;
    }

    av_dict_free(&opts);

    writer->is_initialized = 1;
    log_info("Successfully initialized MP4 writer for stream %s at %s",
            writer->stream_name, writer->output_path);

    return 0;
}

/**
 * Write a packet to the MP4 file with improved timestamp handling
 */
int mp4_writer_write_packet(mp4_writer_t *writer, const AVPacket *in_pkt, const AVStream *input_stream) {
    if (!writer) {
        return -1;
    }

    // Initialize on first packet
    if (!writer->is_initialized) {
        int ret = mp4_writer_initialize(writer, in_pkt, input_stream);
        if (ret < 0) {
            return ret;
        }
    }

    // Create a copy of the packet so we don't modify the original
    AVPacket pkt;
    av_packet_ref(&pkt, in_pkt);

    // Fix timestamps - improved approach to prevent ghosting
    if (writer->first_dts == AV_NOPTS_VALUE) {
        // First packet - use its DTS as reference
        writer->first_dts = pkt.dts != AV_NOPTS_VALUE ? pkt.dts : pkt.pts;
        writer->first_pts = pkt.pts != AV_NOPTS_VALUE ? pkt.pts : pkt.dts;

        // Initialize last_dts to avoid comparison with AV_NOPTS_VALUE
        writer->last_dts = writer->first_dts;

        // For the first packet, set timestamps to 0
        pkt.dts = 0;
        pkt.pts = pkt.pts != AV_NOPTS_VALUE ? pkt.pts - writer->first_pts : 0;
    } else {
        // Preserve original pts/dts spacing but ensure monotonic increase

        // Handle DTS (decoding timestamp)
        if (pkt.dts != AV_NOPTS_VALUE) {
            // Calculate proper offset from the first DTS
            int64_t dts_diff = pkt.dts - writer->first_dts;

            // If DTS goes backwards or is the same, adjust it to be strictly increasing
            if (dts_diff <= 0 || pkt.dts <= writer->last_dts) {
                // Increase by at least 1 tick from the last DTS
                pkt.dts = writer->last_dts + 1;
            } else {
                pkt.dts = dts_diff;
            }
        } else {
            // If DTS is not set, derive it from PTS if possible
            pkt.dts = pkt.pts != AV_NOPTS_VALUE ?
                      (pkt.pts - writer->first_pts) : (writer->last_dts + 1);
        }

        // Handle PTS (presentation timestamp)
        if (pkt.pts != AV_NOPTS_VALUE) {
            // Calculate proper offset from the first PTS
            int64_t pts_diff = pkt.pts - writer->first_pts;

            // PTS can be before DTS for B-frames, but must be >= 0
            pkt.pts = pts_diff >= 0 ? pts_diff : 0;

            // Ensure PTS is not too far in the future relative to DTS
            // This helps prevent large PTS-DTS differences that can cause ghosting
            if (pkt.pts > pkt.dts + (input_stream->time_base.den / input_stream->time_base.num)) {
                int64_t max_diff = input_stream->time_base.den / input_stream->time_base.num;
                pkt.pts = pkt.dts + max_diff;
            }
        } else {
            // If PTS is not set, use DTS
            pkt.pts = pkt.dts;
        }
    }

    // Update last DTS to maintain monotonic increase
    writer->last_dts = pkt.dts;

    // Write packet
    pkt.stream_index = writer->video_stream_idx;

    // Log key frame information for debugging
    if (pkt.flags & AV_PKT_FLAG_KEY) {
        log_debug("Writing keyframe to MP4: pts=%lld, dts=%lld",
                 (long long)pkt.pts, (long long)pkt.dts);
    }

    int ret = av_interleaved_write_frame(writer->output_ctx, &pkt);
    if (ret < 0) {
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Error writing MP4 packet: %s", error_buf);
    }

    // Free packet resources
    av_packet_unref(&pkt);

    return ret;
}

/**
 * Enhanced close function that creates symlinks to help with discovery
 */
void mp4_writer_close(mp4_writer_t *writer) {
    if (!writer) {
        return;
    }

    char output_path[1024];
    char stream_name[64];
    time_t creation_time = writer->creation_time;

    // Save these for later use with symlinks
    strncpy(output_path, writer->output_path, sizeof(output_path) - 1);
    output_path[sizeof(output_path) - 1] = '\0';

    strncpy(stream_name, writer->stream_name, sizeof(stream_name) - 1);
    stream_name[sizeof(stream_name) - 1] = '\0';

    if (writer->output_ctx) {
        // Write trailer if initialized
        if (writer->is_initialized) {
            // Write file trailer
            int ret = av_write_trailer(writer->output_ctx);
            if (ret < 0) {
                char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
                av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
                log_error("Failed to write trailer for MP4 writer: %s", error_buf);
            }
        }

        // Close output
        if (writer->output_ctx->pb) {
            avio_closep(&writer->output_ctx->pb);
        }

        // Free context
        avformat_free_context(writer->output_ctx);
        writer->output_ctx = NULL;
    }

    log_info("Closed MP4 writer for stream %s at %s",
            writer->stream_name, writer->output_path);

    // Check if file exists and has a reasonable size
    struct stat st;
    if (stat(output_path, &st) == 0 && st.st_size > 1024) { // File exists and is not empty
        // Log success with absolute path for easy debugging
        char abs_path[PATH_MAX];
        if (realpath(output_path, abs_path)) {
            log_info("Successfully wrote MP4 file at: %s (size: %lld bytes)",
                    abs_path, (long long)st.st_size);
        } else {
            log_info("Successfully wrote MP4 file at: %s (size: %lld bytes)",
                    output_path, (long long)st.st_size);
        }

        // Ensure recording is properly updated in database
        if (writer->is_initialized) {
            // Find any active recordings for this stream in our tracking array
            pthread_mutex_lock(&recordings_mutex);
            bool found = false;

            for (int i = 0; i < MAX_STREAMS; i++) {
                if (active_recordings[i].recording_id > 0 &&
                    strcmp(active_recordings[i].stream_name, writer->stream_name) == 0) {

                    // We found an active recording for this stream
                    found = true;

                    // Mark recording as complete
                    time_t end_time = time(NULL);
                    update_recording_metadata(active_recordings[i].recording_id,
                                              end_time, st.st_size, true);

                    log_info("Updated database record for recording %llu, size: %lld bytes",
                            (unsigned long long)active_recordings[i].recording_id,
                            (long long)st.st_size);

                    // Clear the active recording slot
                    active_recordings[i].recording_id = 0;
                    active_recordings[i].stream_name[0] = '\0';
                    active_recordings[i].output_path[0] = '\0';

                    break;
                }
            }

            // If no active recording was found, create one
            if (!found) {
                log_info("No active recording found for stream %s, creating database entry",
                        writer->stream_name);

                // Create recording metadata
                recording_metadata_t metadata;
                memset(&metadata, 0, sizeof(recording_metadata_t));

                strncpy(metadata.stream_name, writer->stream_name, sizeof(metadata.stream_name) - 1);
                strncpy(metadata.file_path, writer->output_path, sizeof(metadata.file_path) - 1);

                metadata.size_bytes = st.st_size;

                // Get stream config for additional metadata
                stream_handle_t stream = get_stream_by_name(writer->stream_name);
                if (stream) {
                    stream_config_t config;
                    if (get_stream_config(stream, &config) == 0) {
                        metadata.width = config.width;
                        metadata.height = config.height;
                        metadata.fps = config.fps;
                        strncpy(metadata.codec, config.codec, sizeof(metadata.codec) - 1);
                    }
                }

                // Set timestamps
                time_t duration_sec = 0;
                if (writer->first_dts != AV_NOPTS_VALUE && writer->last_dts != AV_NOPTS_VALUE) {
                    // Convert from stream timebase to seconds
                    int64_t duration_tb = writer->last_dts - writer->first_dts;
                    if (writer->time_base.den > 0) {
                        duration_sec = duration_tb * writer->time_base.num / writer->time_base.den;
                    }
                }

                time_t now = time(NULL);
                metadata.start_time = now - duration_sec;
                metadata.end_time = now;
                metadata.is_complete = true;

                // Add to database
                uint64_t recording_id = add_recording_metadata(&metadata);
                if (recording_id > 0) {
                    log_info("Created database entry for completed recording, ID: %llu",
                            (unsigned long long)recording_id);
                } else {
                    log_error("Failed to create database entry for completed recording");
                }
            }

            pthread_mutex_unlock(&recordings_mutex);
        }
    } else {
        log_error("MP4 file missing or too small: %s", output_path);
    }

    // Free writer
    free(writer);
}

/**
 * Update list_mp4_files_for_stream function to include the mp4 directory
 */
void list_mp4_files_for_stream(const char *stream_name) {
    if (!stream_name) return;

    log_info("=== Listing all MP4 files for stream '%s' ===", stream_name);

    // Get global config for storage paths
    config_t *global_config = get_streaming_config();

    // Places to look
    const char *locations[] = {
        // Standard recordings directory
        global_config->storage_path ? global_config->storage_path : "",
        // Direct MP4 storage if configured
        global_config->record_mp4_directly && global_config->mp4_storage_path[0] != '\0' ?
            global_config->mp4_storage_path : NULL,
        // HLS directory
        global_config->storage_path ? global_config->storage_path : "",
        // NEW: MP4 directory
        global_config->storage_path ? global_config->storage_path : ""
    };

    // Subdirectories to check in each location
    const char *subdirs[] = {
        "recordings",
        "",
        "mp4"  // NEW: Added mp4 directory
    };

    for (int i = 0; i < 4; i++) {  // Updated to 4 to include the new location
        if (!locations[i] || locations[i][0] == '\0') continue;

        char base_path[512];
        if (subdirs[i][0] != '\0') {
            snprintf(base_path, sizeof(base_path), "%s/%s/%s",
                    locations[i], subdirs[i], stream_name);
        } else {
            snprintf(base_path, sizeof(base_path), "%s/%s",
                    locations[i], stream_name);
        }

        // Use find command to locate all MP4 files
        char find_cmd[1024];
        snprintf(find_cmd, sizeof(find_cmd),
                "find %s -type f -name \"*.mp4\" -exec ls -la {} \\; 2>/dev/null",
                base_path);

        log_info("Checking location: %s", base_path);

        FILE *cmd_pipe = popen(find_cmd, "r");
        if (cmd_pipe) {
            char line[1024];
            while (fgets(line, sizeof(line), cmd_pipe)) {
                // Remove trailing newline
                size_t len = strlen(line);
                if (len > 0 && line[len-1] == '\n') {
                    line[len-1] = '\0';
                }

                log_info("  Found: %s", line);
            }
            pclose(cmd_pipe);
        }
    }

    log_info("=== MP4 file listing complete ===");
}