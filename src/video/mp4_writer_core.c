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
#include <errno.h>
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
#include "utils/strings.h"
#include "video/stream_manager.h"
#include "video/streams.h"
#include "video/mp4_writer.h"
#include "video/mp4_writer_internal.h"
#include "storage/storage_manager_streams_cache.h"

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
    safe_strcpy(writer->output_path, output_path, sizeof(writer->output_path), 0);
    safe_strcpy(writer->stream_name, stream_name, sizeof(writer->stream_name), 0);
    writer->first_dts = AV_NOPTS_VALUE;
    writer->first_pts = AV_NOPTS_VALUE;
    writer->last_dts = AV_NOPTS_VALUE;
    writer->is_initialized = 0;
    writer->creation_time = time(NULL);
    writer->last_packet_time = 0;  // Initialize to 0 to indicate no packets written yet
    writer->has_audio = 1;         // Initialize to 1 to enable audio by default
    writer->current_recording_id = 0; // Initialize to 0 to indicate no recording ID yet
    safe_strcpy(writer->trigger_type, "scheduled", sizeof(writer->trigger_type), 0); // Default to scheduled
    writer->schedule_restricted = 0;

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
    writer->pending_audio_codecpar = NULL; // Populated by udt_start_recording(), consumed by mp4_writer_initialize()
    writer->pending_audio_time_base = (AVRational){0, 1}; // Set alongside pending_audio_codecpar

    // Extract output directory from output path
    safe_strcpy(writer->output_dir, output_path, sizeof(writer->output_dir), 0);
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
 * Get the actual end time of a recording based on its start time and video duration
 */
/**
 * Return the actual encoded duration of a finalized MP4 file by reading its
 * container metadata.  Must only be called after the file has been fully
 * written (av_write_trailer + avio_closep already called).
 *
 * @param path  Full path to the closed MP4 file
 * @return      Duration in seconds (>= 0.0), or -1.0 on error
 */
double get_mp4_file_duration_seconds(const char *path) {
    if (!path || path[0] == '\0') return -1.0;

    AVFormatContext *fmt = NULL;
    if (avformat_open_input(&fmt, path, NULL, NULL) != 0) {
        log_warn("get_mp4_file_duration_seconds: cannot open '%s'", path);
        return -1.0;
    }

    double duration = -1.0;
    if (avformat_find_stream_info(fmt, NULL) >= 0 &&
        fmt->duration != AV_NOPTS_VALUE) {
        duration = (double)fmt->duration / (double)AV_TIME_BASE;
    }

    avformat_close_input(&fmt);
    return duration;
}

/**
 * Close the MP4 writer and release all resources.
 *
 * Close sequence (order matters):
 *  1. Stop the RTSP recording thread (if running)
 *  2. Write the MP4 trailer and close the AVIOContext  ← file is now complete
 *  3. Free stream codec parameters and the AVFormatContext
 *  4. Stat the file for its final size
 *  5. Read the actual duration from the closed MP4 container
 *  6. Update the database record with accurate end_time and size
 *  7. Destroy mutexes, cleanup audio transcoder, free writer struct
 *
 * The old code did step 6 before steps 1-3, which meant the MP4 container
 * was not yet finalized when avformat_open_input was called inside
 * get_recording_end_time(), producing inaccurate durations.
 */
void mp4_writer_close(mp4_writer_t *writer) {
    if (!writer) {
        log_warn("NULL writer passed to mp4_writer_close");
        return;
    }

    log_info("Closing MP4 writer for stream %s at %s",
             writer->stream_name ? writer->stream_name : "unknown",
             writer->output_path ? writer->output_path : "unknown");

    /* ------------------------------------------------------------------ *
     * 1. Stop the RTSP recording thread so it no longer writes packets.  *
     * ------------------------------------------------------------------ */
    if (writer->thread_ctx) {
        log_info("Stopping recording thread for %s during writer close",
                writer->stream_name ? writer->stream_name : "unknown");
        mp4_writer_stop_recording_thread(writer);
        if (writer->thread_ctx) {
            log_warn("Thread context still exists after stopping recording thread for %s",
                   writer->stream_name ? writer->stream_name : "unknown");
        }
    }

    /* ------------------------------------------------------------------ *
     * 2-3. Write trailer, close AVIOContext, free FFmpeg resources.      *
     *      The file is complete and readable on disk after this block.   *
     * ------------------------------------------------------------------ */
    if (writer->output_ctx) {
        if (writer->is_initialized && writer->output_ctx->pb) {
            int ret = av_write_trailer(writer->output_ctx);
            if (ret < 0) {
                char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
                av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
                log_warn("Failed to write trailer for MP4 writer: %s", error_buf);
            }
        }

        if (writer->output_ctx->pb) {
            avio_closep(&writer->output_ctx->pb);
        }

        /* Free codec parameters for every stream in the output context */
        for (unsigned int i = 0; i < writer->output_ctx->nb_streams; i++) {
            if (writer->output_ctx->streams[i] &&
                writer->output_ctx->streams[i]->codecpar) {
                avcodec_parameters_free(&writer->output_ctx->streams[i]->codecpar);
            }
        }

        avformat_free_context(writer->output_ctx);
        writer->output_ctx = NULL;
    }

    /* ------------------------------------------------------------------ *
     * 4-6. Now that the file is closed: stat size, read actual duration, *
     *      then update the database with accurate values.                *
     * ------------------------------------------------------------------ */
    if (writer->current_recording_id > 0 && writer->output_path[0] != '\0') {
        struct stat st;
        uint64_t size_bytes = 0;
        bool stat_ok = (stat(writer->output_path, &st) == 0);

        if (stat_ok) {
            size_bytes = (uint64_t)st.st_size;
            log_info("Final file size for %s: %llu bytes",
                     writer->output_path, (unsigned long long)size_bytes);
        } else {
            log_warn("Failed to stat '%s' during close: %s",
                     writer->output_path, strerror(errno));
        }

        /* Use st.st_mtime as end_time baseline: after avio_closep() this is the
         * actual wall-clock moment the last byte was written to disk, and it does
         * not depend on writer->creation_time matching the DB start_time.
         *
         * creation_time is set at rotation time while the DB start_time is
         * recorded at the first keyframe, which can introduce a mismatch of up
         * to one GOP interval. st.st_mtime avoids this drift entirely.
         *
         * get_mp4_file_duration_seconds() is still called for log diagnostics. */
        time_t end_time = stat_ok ? st.st_mtime : time(NULL);
        double dur = get_mp4_file_duration_seconds(writer->output_path);
        if (dur >= 0.0) {
            log_info("Actual MP4 duration for %s: %.1f s; end_time=%ld (from st.st_mtime)",
                     writer->output_path, dur, (long)end_time);
        } else {
            log_warn("Could not read duration from '%s'; using %s end_time=%ld",
                     writer->output_path, stat_ok ? "st.st_mtime" : "wall-clock",
                     (long)end_time);
        }

        update_recording_metadata(writer->current_recording_id, end_time, size_bytes, true);
        log_info("Marked recording (ID: %llu) as complete (dur=%.1fs, size=%llu bytes)",
                 (unsigned long long)writer->current_recording_id, dur >= 0 ? dur : 0.0,
                 (unsigned long long)size_bytes);
        update_stream_storage_cache_add_recording(writer->stream_name, size_bytes);
    }

    if (writer->is_rotating) {
        log_warn("MP4 writer was still rotating during close, forcing rotation to complete");
        writer->is_rotating = 0;
        writer->waiting_for_keyframe = 0;
    }

    /* ------------------------------------------------------------------ *
     * 7. Destroy mutexes, cleanup audio transcoder, free struct.         *
     * ------------------------------------------------------------------ */
    int mutex_result = pthread_mutex_destroy(&writer->mutex);
    if (mutex_result != 0) {
        log_warn("Failed to destroy writer mutex: %s", strerror(mutex_result));
    }

    mutex_result = pthread_mutex_destroy(&writer->audio.mutex);
    if (mutex_result != 0) {
        log_warn("Failed to destroy audio mutex: %s", strerror(mutex_result));
    }

    cleanup_audio_transcoder(writer->stream_name);

    if (writer->pending_audio_codecpar) {
        avcodec_parameters_free(&writer->pending_audio_codecpar);
        writer->pending_audio_codecpar = NULL;
    }

    free(writer);

    log_info("MP4 writer closed and resources freed");
}
