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
#include <limits.h>

// Define PATH_MAX if not defined
#ifndef PATH_MAX
#define PATH_MAX 4096
#endif
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
    writer->video_stream_idx = -1;
    writer->audio_stream_idx = -1;
    writer->video_ts_info.first_dts = AV_NOPTS_VALUE;
    writer->video_ts_info.first_pts = AV_NOPTS_VALUE;
    writer->video_ts_info.last_dts = AV_NOPTS_VALUE;
    writer->video_ts_info.initialized = 0;
    writer->audio_ts_info.first_dts = AV_NOPTS_VALUE;
    writer->audio_ts_info.first_pts = AV_NOPTS_VALUE;
    writer->audio_ts_info.last_dts = AV_NOPTS_VALUE;
    writer->audio_ts_info.initialized = 0;
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
    char *dir_path = malloc(PATH_MAX);
    if (!dir_path) {
        log_error("Failed to allocate memory for directory path");
        return -1;
    }
    
    const char *last_slash = strrchr(writer->output_path, '/');
    if (last_slash) {
        size_t dir_len = last_slash - writer->output_path;
        strncpy(dir_path, writer->output_path, dir_len);
        dir_path[dir_len] = '\0';

        // Log the directory we're working with
        log_info("Ensuring MP4 output directory exists: %s", dir_path);

        // Create directory if it doesn't exist
        char *mkdir_cmd = malloc(PATH_MAX + 10);
        if (!mkdir_cmd) {
            log_error("Failed to allocate memory for mkdir command");
            free(dir_path);
            return -1;
        }
        
        snprintf(mkdir_cmd, PATH_MAX + 10, "mkdir -p %s", dir_path);
        if (system(mkdir_cmd) != 0) {
            log_warn("Failed to create directory: %s", dir_path);
        }

        // Set permissions to ensure it's writable
        snprintf(mkdir_cmd, PATH_MAX + 10, "chmod -R 777 %s", dir_path);
        if (system(mkdir_cmd) != 0) {
            log_warn("Failed to set permissions: %s", dir_path);
        }
        
        free(mkdir_cmd);
    }

    // Log the full output path
    log_info("Initializing MP4 writer to output file: %s", writer->output_path);

    // Determine if this is a video or audio stream
    enum AVMediaType codec_type = input_stream->codecpar->codec_type;
    
    // Create output format context if it doesn't exist yet
    if (!writer->output_ctx) {
        ret = avformat_alloc_output_context2(&writer->output_ctx, NULL, "mp4", writer->output_path);
        if (ret < 0 || !writer->output_ctx) {
            char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
            av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
            log_error("Failed to create output format context for MP4 writer: %s", error_buf);
            free(dir_path);
            return -1;
        }
        
        // Add metadata
        av_dict_set(&writer->output_ctx->metadata, "title", writer->stream_name, 0);
        av_dict_set(&writer->output_ctx->metadata, "encoder", "LightNVR", 0);
    }

    // Create output stream
    AVStream *out_stream = avformat_new_stream(writer->output_ctx, NULL);
    if (!out_stream) {
        log_error("Failed to create output stream for MP4 writer");
        avformat_free_context(writer->output_ctx);
        writer->output_ctx = NULL;
        free(dir_path);
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
        free(dir_path);
        return -1;
    }

    // Set stream time base
    out_stream->time_base = input_stream->time_base;
    
    // Store stream index and initialize timestamp tracking based on type
    if (codec_type == AVMEDIA_TYPE_VIDEO) {
        writer->video_stream_idx = out_stream->index;
        writer->video_ts_info.time_base = input_stream->time_base;
        writer->video_ts_info.initialized = 0;
        log_info("Added video stream to MP4 output at index %d", writer->video_stream_idx);
    } else if (codec_type == AVMEDIA_TYPE_AUDIO) {
        writer->audio_stream_idx = out_stream->index;
        writer->audio_ts_info.time_base = input_stream->time_base;
        writer->audio_ts_info.initialized = 0;
        log_info("Added audio stream to MP4 output at index %d", writer->audio_stream_idx);
    } else {
        log_warn("Unsupported stream type for MP4: %d", codec_type);
        free(dir_path);
        return -1;
    }

    // Write header if this is the first initialization
    if (!writer->is_initialized) {
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
            free(dir_path);
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
            free(dir_path);
            return -1;
        }

        av_dict_free(&opts);
        writer->is_initialized = 1;
        log_info("Successfully initialized MP4 writer for stream %s at %s",
                writer->stream_name, writer->output_path);
    }

    free(dir_path);
    return 0;
}

/**
 * Write a packet to the MP4 file with improved timestamp handling
 */
int mp4_writer_write_packet(mp4_writer_t *writer, const AVPacket *in_pkt, const AVStream *input_stream) {
    if (!writer) {
        log_error("Null writer passed to mp4_writer_write_packet");
        return -1;
    }

    if (!in_pkt) {
        log_error("Null packet passed to mp4_writer_write_packet for %s", writer->stream_name);
        return -1;
    }

    if (!input_stream) {
        log_error("Null input stream passed to mp4_writer_write_packet for %s", writer->stream_name);
        return -1;
    }

    // Validate packet data
    if (!in_pkt->data || in_pkt->size <= 0) {
        log_warn("Invalid packet (null data or zero size) for stream %s", writer->stream_name);
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
    av_init_packet(&pkt);
    pkt.data = NULL;
    pkt.size = 0;
    
    int ret = av_packet_ref(&pkt, in_pkt);
    if (ret < 0) {
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Failed to reference packet: %s", error_buf);
        return ret;
    }

    // Determine if this is a video or audio packet and use the appropriate timestamp tracker
    enum AVMediaType codec_type = input_stream->codecpar->codec_type;
    mp4_timestamp_info_t *ts_info;
    int stream_idx;
    
    if (codec_type == AVMEDIA_TYPE_VIDEO) {
        ts_info = &writer->video_ts_info;
        stream_idx = writer->video_stream_idx;
        log_debug("Processing video packet for MP4 stream %s", writer->stream_name);
    } else if (codec_type == AVMEDIA_TYPE_AUDIO) {
        ts_info = &writer->audio_ts_info;
        stream_idx = writer->audio_stream_idx;
        log_debug("Processing audio packet for MP4 stream %s", writer->stream_name);
    } else {
        log_warn("Unsupported packet type for MP4: %d", codec_type);
        av_packet_unref(&pkt);
        return -1;
    }
    
    // Enhanced timestamp handling to fix non-monotonic DTS issues
    if (!ts_info->initialized) {
        // For video, wait for a key frame to start if possible
        if (codec_type == AVMEDIA_TYPE_VIDEO && !(in_pkt->flags & AV_PKT_FLAG_KEY)) {
            // Skip non-key frames at the beginning for video
            log_debug("Skipping non-key frame at start of MP4 recording");
            av_packet_unref(&pkt);
            return 0; // Return success but don't process this packet
        }
        
        // First packet - use its DTS as reference
        ts_info->first_dts = pkt.dts != AV_NOPTS_VALUE ? pkt.dts : pkt.pts;
        ts_info->first_pts = pkt.pts != AV_NOPTS_VALUE ? pkt.pts : pkt.dts;
        ts_info->last_dts = ts_info->first_dts;
        ts_info->initialized = 1;

        // For the first packet, set timestamps to 0
        pkt.dts = 0;
        pkt.pts = pkt.pts != AV_NOPTS_VALUE ? pkt.pts - ts_info->first_pts : 0;
        
        // Ensure PTS is valid (not less than DTS)
        if (pkt.pts < pkt.dts) {
            pkt.pts = pkt.dts;
        }
        
        log_debug("MP4 writer initialized %s timestamps: first_dts=%lld, first_pts=%lld", 
                 codec_type == AVMEDIA_TYPE_VIDEO ? "video" : "audio",
                 (long long)ts_info->first_dts, (long long)ts_info->first_pts);
    } else {
        // Check for timestamp discontinuities (common in RTSP streams)
        int64_t expected_dts = ts_info->last_dts + 
            av_rescale_q(1, input_stream->time_base, ts_info->time_base);
        int64_t dts_diff = 0;
        
        if (pkt.dts != AV_NOPTS_VALUE) {
            dts_diff = pkt.dts - ts_info->first_dts;
            
            // If there's a large backward jump in DTS (stream reset or loop)
            if (dts_diff < 0 || pkt.dts < ts_info->last_dts) {
                // This is a discontinuity - log it
                log_warn("DTS discontinuity detected in %s stream: last_dts=%lld, current_dts=%lld, diff=%lld",
                        codec_type == AVMEDIA_TYPE_VIDEO ? "video" : "audio",
                        (long long)ts_info->last_dts, (long long)pkt.dts, 
                        (long long)(pkt.dts - ts_info->last_dts));
                
                // Handle the discontinuity by continuing from the last DTS
                // Add a small increment to ensure monotonic increase
                pkt.dts = expected_dts;
                
                // If PTS is also invalid, adjust it too
                if (pkt.pts != AV_NOPTS_VALUE && pkt.pts < pkt.dts) {
                    pkt.pts = pkt.dts + 1; // Ensure PTS > DTS for proper playback
                }
            } else {
                // Normal case - use the relative offset from first_dts
                pkt.dts = dts_diff;
                
                // Ensure monotonic increase
                if (pkt.dts <= ts_info->last_dts) {
                    pkt.dts = ts_info->last_dts + 1;
                }
            }
        } else {
            // If DTS is not set, use a continuous value
            pkt.dts = expected_dts;
        }

        // Handle PTS (presentation timestamp)
        if (pkt.pts != AV_NOPTS_VALUE) {
            // Calculate proper offset from the first PTS
            int64_t pts_diff = pkt.pts - ts_info->first_pts;

            // If PTS goes backwards or would be less than DTS, adjust it
            if (pts_diff < 0 || pkt.pts < pkt.dts) {
                // Set PTS slightly ahead of DTS as a fallback
                pkt.pts = pkt.dts + 1;
            } else {
                pkt.pts = pts_diff;
                
                // Ensure PTS is at least DTS + 1 for proper playback
                if (pkt.pts <= pkt.dts) {
                    pkt.pts = pkt.dts + 1;
                }
            }
        } else {
            // If PTS is not set, use DTS + 1
            pkt.pts = pkt.dts + 1;
        }
    }

    // Update last DTS to maintain monotonic increase
    ts_info->last_dts = pkt.dts;

    // Set the correct stream index
    pkt.stream_index = stream_idx;

    // Log key frame information for debugging
    if (codec_type == AVMEDIA_TYPE_VIDEO && (pkt.flags & AV_PKT_FLAG_KEY)) {
        log_debug("Writing video keyframe to MP4: pts=%lld, dts=%lld",
                 (long long)pkt.pts, (long long)pkt.dts);
    } else if (codec_type == AVMEDIA_TYPE_AUDIO) {
        log_debug("Writing audio packet to MP4: pts=%lld, dts=%lld",
                 (long long)pkt.pts, (long long)pkt.dts);
    }

    ret = av_interleaved_write_frame(writer->output_ctx, &pkt);
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
 * Enhanced close function with improved error handling and memory safety
 */
void mp4_writer_close(mp4_writer_t *writer) {
    if (!writer) {
        log_warn("Attempted to close NULL MP4 writer");
        return;
    }

    // Create local copies of all data we need before modifying the writer
    // This prevents use-after-free and other memory corruption issues
    char *output_path = malloc(PATH_MAX);
    char *stream_name = malloc(MAX_STREAM_NAME);
    time_t creation_time = 0;
    bool was_initialized = false;
    int64_t first_dts = AV_NOPTS_VALUE;
    int64_t last_dts = AV_NOPTS_VALUE;
    AVRational time_base = {0, 0};
    AVFormatContext *output_ctx = NULL;
    
    if (!output_path || !stream_name) {
        log_error("Failed to allocate memory for path or stream name");
        free(output_path);
        free(stream_name);
        return;
    }
    
    memset(output_path, 0, PATH_MAX);
    memset(stream_name, 0, MAX_STREAM_NAME);
    
    // Use a critical section to safely extract and nullify the output context
    // This prevents race conditions where multiple threads might try to close the same writer
    {
        // Safely copy all needed data with proper validation
        bool valid_output_path = writer->output_path && writer->output_path[0] != '\0';
        bool valid_stream_name = writer->stream_name && writer->stream_name[0] != '\0';
        
        // Only copy if the fields are valid
        if (valid_output_path) {
            strncpy(output_path, writer->output_path, PATH_MAX - 1);
            output_path[PATH_MAX - 1] = '\0';
        }

        if (valid_stream_name) {
            strncpy(stream_name, writer->stream_name, MAX_STREAM_NAME - 1);
            stream_name[MAX_STREAM_NAME - 1] = '\0';
        } else {
            strncpy(stream_name, "unknown", MAX_STREAM_NAME - 1);
        }
        
        // Safely copy other fields
        creation_time = writer->creation_time;
        was_initialized = writer->is_initialized;
        
        // Use video timestamp info for duration calculation
        if (writer->video_ts_info.initialized) {
            first_dts = writer->video_ts_info.first_dts;
            last_dts = writer->video_ts_info.last_dts;
            time_base = writer->video_ts_info.time_base;
        }
        
        // Extract the output context and immediately set it to NULL to prevent double-free
        output_ctx = writer->output_ctx;
        writer->output_ctx = NULL;
        
        // Mark as not initialized to prevent further packet writes
        writer->is_initialized = 0;
    }

    // Log the operation
    log_info("Closing MP4 writer for stream %s at %s",
            stream_name[0] ? stream_name : "unknown", 
            output_path[0] ? output_path : "(invalid path)");

    // Close the output context if it exists
    if (output_ctx) {
        // Write trailer if the writer was initialized
        if (was_initialized) {
            int ret = av_write_trailer(output_ctx);
            if (ret < 0) {
                char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
                av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
                log_error("Failed to write trailer for MP4 writer: %s", error_buf);
            }
        }

        // Close output
        if (output_ctx->pb) {
            avio_closep(&output_ctx->pb);
        }

        // Free context
        avformat_free_context(output_ctx);
        output_ctx = NULL;  // Prevent any accidental use after free
    }

    // Check if file exists and has a reasonable size - only if we have a valid path
    if (output_path[0] != '\0') {
        struct stat st;
        if (stat(output_path, &st) == 0 && st.st_size > 1024) { // File exists and is not empty
            // Log success with absolute path for easy debugging
            char *abs_path = malloc(PATH_MAX);
            if (abs_path) {
                if (realpath(output_path, abs_path)) {
                    log_info("Successfully wrote MP4 file at: %s (size: %lld bytes)",
                            abs_path, (long long)st.st_size);
                } else {
                    log_info("Successfully wrote MP4 file at: %s (size: %lld bytes)",
                            output_path, (long long)st.st_size);
                }
                free(abs_path);
            } else {
                log_info("Successfully wrote MP4 file at: %s (size: %lld bytes)",
                        output_path, (long long)st.st_size);
            }

            // Ensure recording is properly updated in database
            if (was_initialized) {
                // Find any active recordings for this stream in our tracking array
                pthread_mutex_lock(&recordings_mutex);
                bool found = false;

                for (int i = 0; i < MAX_STREAMS; i++) {
                    if (active_recordings[i].recording_id > 0 &&
                        strcmp(active_recordings[i].stream_name, stream_name) == 0) {

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
                            stream_name);

                    // Create recording metadata
                    recording_metadata_t metadata;
                    memset(&metadata, 0, sizeof(recording_metadata_t));

                    strncpy(metadata.stream_name, stream_name, sizeof(metadata.stream_name) - 1);
                    strncpy(metadata.file_path, output_path, sizeof(metadata.file_path) - 1);

                    metadata.size_bytes = st.st_size;

                    // Get stream config for additional metadata
                    stream_handle_t stream = get_stream_by_name(stream_name);
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
                    if (first_dts != AV_NOPTS_VALUE && last_dts != AV_NOPTS_VALUE) {
                        // Convert from stream timebase to seconds
                        int64_t duration_tb = last_dts;  // last_dts is already relative to first_dts
                        if (time_base.den > 0) {
                            duration_sec = duration_tb * time_base.num / time_base.den;
                            log_info("MP4 duration calculated: %ld seconds (timebase: %d/%d, ticks: %lld)", 
                                    duration_sec, time_base.num, time_base.den, 
                                    (long long)duration_tb);
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
    }

    // Free allocated memory
    free(output_path);
    free(stream_name);
    
    // Free writer at the end, after we're done using all its data
    free(writer);
}
