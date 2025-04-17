/**
 * Core implementation of MP4 writer for storing camera streams
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <dirent.h>
#include <time.h>
#include <sys/stat.h>
#include <limits.h>
#include <signal.h>

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
#include "video/mp4_writer_internal.h"

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
    writer->first_dts = AV_NOPTS_VALUE;
    writer->first_pts = AV_NOPTS_VALUE;
    writer->last_dts = AV_NOPTS_VALUE;
    writer->is_initialized = 0;
    writer->creation_time = time(NULL);
    writer->last_packet_time = 0;  // Initialize to 0 to indicate no packets written yet
    writer->has_audio = 1;         // Initialize to 1 to enable audio by default
    writer->current_recording_id = 0; // Initialize to 0 to indicate no recording ID yet

    // Initialize audio state
    writer->audio.stream_idx = -1; // Initialize to -1 to indicate no audio stream
    writer->audio.first_dts = AV_NOPTS_VALUE;
    writer->audio.last_pts = 0;
    writer->audio.last_dts = 0;
    writer->audio.initialized = 0;
    writer->audio.time_base.num = 1;
    writer->audio.time_base.den = 48000; // Default to 48kHz
    writer->audio.frame_size = 1024;    // Default audio frame size for Opus

    // Initialize segment-related fields
    writer->segment_duration = 0;  // Default to 0 (no rotation)
    writer->last_rotation_time = time(NULL);
    writer->waiting_for_keyframe = 0;
    writer->is_rotating = 0;       // Initialize rotation flag
    writer->shutdown_component_id = -1; // Initialize to -1 to indicate not registered

    // Extract output directory from output path
    strncpy(writer->output_dir, output_path, sizeof(writer->output_dir) - 1);
    char *last_slash = strrchr(writer->output_dir, '/');
    if (last_slash) {
        *last_slash = '\0';  // Truncate at the last slash to get directory
    }

    // Initialize mutexes
    pthread_mutex_init(&writer->mutex, NULL);
    pthread_mutex_init(&writer->audio.mutex, NULL);

    log_info("Created MP4 writer for stream %s at %s", stream_name, output_path);

    return writer;
}

/**
 * Set the segment duration for MP4 rotation
 */
void mp4_writer_set_segment_duration(mp4_writer_t *writer, int segment_duration) {
    if (!writer) {
        log_error("NULL writer passed to mp4_writer_set_segment_duration");
        return;
    }

    writer->segment_duration = segment_duration;
    writer->last_rotation_time = time(NULL);
    writer->waiting_for_keyframe = 0;

    log_info("Set segment duration to %d seconds for stream %s",
             segment_duration, writer->stream_name ? writer->stream_name : "unknown");
}

/**
 * Close the MP4 writer and release resources
 */
void mp4_writer_close(mp4_writer_t *writer) {
    if (!writer) {
        log_warn("NULL writer passed to mp4_writer_close");
        return;
    }

    // Log the closing operation
    log_info("Closing MP4 writer for stream %s at %s",
             writer->stream_name ? writer->stream_name : "unknown",
             writer->output_path ? writer->output_path : "unknown");

    //  First, mark the recording as complete in the database if needed
    if (writer->current_recording_id > 0) {
        // Get the file size before marking as complete
        struct stat st;
        uint64_t size_bytes = 0;

        if (writer->output_path && stat(writer->output_path, &st) == 0) {
            size_bytes = st.st_size;
            log_info("Final file size for %s: %llu bytes",
                    writer->output_path, (unsigned long long)size_bytes);

            // Mark the recording as complete with the correct file size
            update_recording_metadata(writer->current_recording_id, time(NULL), size_bytes, true);
            log_info("Marked recording (ID: %llu) as complete during writer close",
                    (unsigned long long)writer->current_recording_id);
        } else if (writer->output_path) {
            log_warn("Failed to get file size for %s during close", writer->output_path);

            // Still mark the recording as complete, but with size 0
            update_recording_metadata(writer->current_recording_id, time(NULL), 0, true);
            log_info("Marked recording (ID: %llu) as complete during writer close (size unknown)",
                    (unsigned long long)writer->current_recording_id);
        }
    }

    // First, stop any recording thread if it's running
    if (writer->thread_ctx) {
        log_info("Stopping recording thread for %s during writer close",
                writer->stream_name ? writer->stream_name : "unknown");
        mp4_writer_stop_recording_thread(writer);
        //  thread_ctx should now be NULL if join was successful
        // If join failed, the thread was detached and will clean up itself
    }

    // MEMORY LEAK FIX: Ensure proper cleanup of FFmpeg resources
    // Close the output context if it exists
    if (writer->output_ctx) {
        // Write trailer if the context is initialized
        if (writer->is_initialized && writer->output_ctx->pb) {
            int ret = av_write_trailer(writer->output_ctx);
            if (ret < 0) {
                char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
                av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
                log_warn("Failed to write trailer for MP4 writer: %s", error_buf);
            }
        }

        // Close the output file
        if (writer->output_ctx->pb) {
            avio_closep(&writer->output_ctx->pb);
        }

        // MEMORY LEAK FIX: Properly clean up all streams in the output context
        // This ensures all codec contexts and other resources are freed
        if (writer->output_ctx->nb_streams > 0) {
            for (unsigned int i = 0; i < writer->output_ctx->nb_streams; i++) {
                if (writer->output_ctx->streams[i]) {
                    // Free any codec parameters
                    if (writer->output_ctx->streams[i]->codecpar) {
                        avcodec_parameters_free(&writer->output_ctx->streams[i]->codecpar);
                    }
                }
            }
        }

        // Free the output context
        avformat_free_context(writer->output_ctx);
        writer->output_ctx = NULL;
    }

    //  Ensure we're not in the middle of a rotation
    if (writer->is_rotating) {
        log_warn("MP4 writer was still rotating during close, forcing rotation to complete");
        writer->is_rotating = 0;
        writer->waiting_for_keyframe = 0;
    }

    // Destroy mutexes with proper error handling
    int mutex_result = pthread_mutex_destroy(&writer->mutex);
    if (mutex_result != 0) {
        log_warn("Failed to destroy writer mutex: %s", strerror(mutex_result));
    }

    mutex_result = pthread_mutex_destroy(&writer->audio.mutex);
    if (mutex_result != 0) {
        log_warn("Failed to destroy audio mutex: %s", strerror(mutex_result));
    }

    // Clean up any audio transcoders for this stream
    extern void cleanup_audio_transcoder(const char *stream_name);
    cleanup_audio_transcoder(writer->stream_name);

    // Free the writer structure
    free(writer);

    log_info("MP4 writer closed and resources freed");
}
