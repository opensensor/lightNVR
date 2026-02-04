/**
 * MP4 Writer Thread Implementation
 *
 * This module handles the thread-related functionality for MP4 recording.
 * It's responsible for:
 * - Managing RTSP recording threads
 * - Handling thread lifecycle (start, stop, etc.)
 * - Managing thread resources
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <time.h>
#include <stdbool.h>
#include <signal.h>
#include <pthread.h>
#include <sys/stat.h>
#include <errno.h>

#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <libavutil/avutil.h>
#include <libavutil/time.h>

#include "core/logger.h"
#include "core/shutdown_coordinator.h"
#include "video/mp4_writer.h"
#include "video/mp4_writer_internal.h"
#include "video/mp4_writer_thread.h"
#include "video/mp4_segment_recorder.h"
#include "database/database_manager.h"
#include "database/db_recordings.h"


// Callback invoked by record_segment when the first keyframe is detected
// and the segment officially begins. We create the DB metadata here so
// start_time aligns with the actual playable start.
static void on_segment_started_cb(void *user_ctx) {
    mp4_writer_thread_t *thread_ctx = (mp4_writer_thread_t *)user_ctx;
    if (!thread_ctx || !thread_ctx->writer) return;

    // Avoid duplicate creation in case of any re-entry
    if (thread_ctx->writer->current_recording_id > 0) return;

    const char *stream_name = thread_ctx->writer->stream_name[0] ? thread_ctx->writer->stream_name : "unknown";

    if (thread_ctx->writer->output_path[0] != '\0') {
        recording_metadata_t metadata;
        memset(&metadata, 0, sizeof(recording_metadata_t));
        strncpy(metadata.stream_name, stream_name, sizeof(metadata.stream_name) - 1);
        strncpy(metadata.file_path, thread_ctx->writer->output_path, sizeof(metadata.file_path) - 1);
        metadata.start_time = time(NULL); // Align to keyframe time
        metadata.end_time = 0;
        metadata.size_bytes = 0;
        metadata.is_complete = false;
        strncpy(metadata.trigger_type, thread_ctx->writer->trigger_type, sizeof(metadata.trigger_type) - 1);

        uint64_t recording_id = add_recording_metadata(&metadata);
        if (recording_id == 0) {
            log_error("Failed to add recording metadata at segment start for stream %s", stream_name);
        } else {
            log_info("Added recording at segment start (ID: %llu, trigger_type: %s) for file: %s",
                     (unsigned long long)recording_id, metadata.trigger_type, thread_ctx->writer->output_path);
            thread_ctx->writer->current_recording_id = recording_id;
        }
    }
}


/**
 * RTSP stream reading thread function
 * This function maintains a single RTSP connection across multiple segments
 * and handles self-management including retries and shutdown
 */
static void *mp4_writer_rtsp_thread(void *arg) {
    mp4_writer_thread_t *thread_ctx = (mp4_writer_thread_t *)arg;
    if (!thread_ctx || !thread_ctx->writer) {
        return NULL;
    }

    // Set running flag at start of thread
    thread_ctx->running = 1;

    // Create a local copy of needed values to prevent use-after-free
    char rtsp_url[MAX_PATH_LENGTH];
    strncpy(rtsp_url, thread_ctx->rtsp_url, sizeof(rtsp_url) - 1);
    rtsp_url[sizeof(rtsp_url) - 1] = '\0';

    int segment_duration = thread_ctx->segment_duration;
    mp4_writer_t *writer = thread_ctx->writer;

    AVPacket *pkt = NULL;
    int video_stream_idx = -1;
    int audio_stream_idx = -1;
    int ret;
    time_t start_time = time(NULL);  // Record when we started

    // BUGFIX: Initialize per-stream context and segment info
    // These are now stored in the thread context instead of global static variables
    thread_ctx->input_ctx = NULL;
    thread_ctx->segment_info.segment_index = 0;
    thread_ctx->segment_info.has_audio = false;
    thread_ctx->segment_info.last_frame_was_key = false;
    pthread_mutex_init(&thread_ctx->context_mutex, NULL);

    // Initialize self-management fields
    thread_ctx->retry_count = 0;
    thread_ctx->last_retry_time = 0;

    // Make a local copy of the stream name for thread safety
    char stream_name[MAX_STREAM_NAME];
    if (thread_ctx->writer && thread_ctx->writer->stream_name[0] != '\0') {
        strncpy(stream_name, thread_ctx->writer->stream_name, MAX_STREAM_NAME - 1);
        stream_name[MAX_STREAM_NAME - 1] = '\0';
    } else {
        strncpy(stream_name, "unknown", MAX_STREAM_NAME - 1);
    }

    log_info("Starting RTSP reading thread for stream %s", stream_name);

    // Defer DB creation until the first keyframe is seen so start_time aligns to a playable frame.

    // Check if we're still running (might have been stopped during initialization)
    if (!thread_ctx->running || thread_ctx->shutdown_requested) {
        log_info("RTSP reading thread for %s exiting early due to shutdown", stream_name);
        return NULL;
    }

    // BUGFIX: Segment info is already initialized in the thread context initialization above
    log_info("Initialized segment info: index=%d, has_audio=%d, last_frame_was_key=%d",
            thread_ctx->segment_info.segment_index, thread_ctx->segment_info.has_audio,
            thread_ctx->segment_info.last_frame_was_key);

    // Main loop to record segments
    while (thread_ctx->running && !thread_ctx->shutdown_requested) {
        // Check if shutdown has been initiated
        if (is_shutdown_initiated()) {
            log_info("RTSP reading thread for %s stopping due to system shutdown", stream_name);
            thread_ctx->running = 0;
            break;
        }

        // Check if force reconnect was signaled (e.g., after go2rtc restart)
        if (atomic_exchange(&thread_ctx->force_reconnect, 0)) {
            log_info("Force reconnect signaled for stream %s, closing current connection", stream_name);

            // Close the current input context to force a fresh connection
            if (thread_ctx->input_ctx) {
                avformat_close_input(&thread_ctx->input_ctx);
                thread_ctx->input_ctx = NULL;
            }

            // Reset retry count to give the reconnection a clean slate
            thread_ctx->retry_count = 0;

            // Wait a moment for the upstream to be ready (go2rtc may still be initializing streams)
            sleep(3);

            log_info("Force reconnect: will attempt fresh connection for stream %s", stream_name);
        }

        // Get current time
        time_t current_time = time(NULL);

        // Fetch the latest stream configuration from the database
        stream_config_t db_stream_config;
        int db_config_result = get_stream_config_by_name(stream_name, &db_stream_config);

        // Define segment_duration variable outside the if block
        int segment_duration = thread_ctx->writer->segment_duration;

        // Update configuration from database if available
        if (db_config_result == 0) {
            // Update segment duration if available
            if (db_stream_config.segment_duration > 0) {
                segment_duration = db_stream_config.segment_duration;

                // Update the writer's segment duration if it has changed
                if (thread_ctx->writer->segment_duration != segment_duration) {
                    log_info("Updating segment duration for stream %s from %d to %d seconds (from database)",
                            stream_name, thread_ctx->writer->segment_duration, segment_duration);
                    thread_ctx->writer->segment_duration = segment_duration;
                }
            }

            // Update audio recording setting if it has changed
            int has_audio = db_stream_config.record_audio ? 1 : 0;
            if (thread_ctx->writer->has_audio != has_audio) {
                log_info("Updating audio recording setting for stream %s from %s to %s (from database)",
                        stream_name,
                        thread_ctx->writer->has_audio ? "enabled" : "disabled",
                        has_audio ? "enabled" : "disabled");
                thread_ctx->writer->has_audio = has_audio;
            }
        }

        // Check if it's time to create a new segment based on segment duration
        // Force segment rotation every segment_duration seconds
        if (segment_duration > 0) {
            time_t elapsed_time = current_time - thread_ctx->writer->last_rotation_time;
            if (elapsed_time >= segment_duration) {
                log_info("Time to create new segment for stream %s (elapsed time: %ld seconds, segment duration: %d seconds)",
                         stream_name, (long)elapsed_time, segment_duration);

                // Create timestamp for new MP4 filename
                char timestamp_str[32];
                struct tm *tm_info = localtime(&current_time);
                strftime(timestamp_str, sizeof(timestamp_str), "%Y%m%d_%H%M%S", tm_info);

                // Create new output path
                char new_path[MAX_PATH_LENGTH];
                snprintf(new_path, MAX_PATH_LENGTH, "%s/recording_%s.mp4",
                         thread_ctx->writer->output_dir, timestamp_str);

                // Get the current output path before closing
                char current_path[MAX_PATH_LENGTH];
                strncpy(current_path, thread_ctx->writer->output_path, MAX_PATH_LENGTH - 1);
                current_path[MAX_PATH_LENGTH - 1] = '\0';

                // Defer creation of DB metadata for the new file until first keyframe via callback
                // so that start_time aligns to a playable keyframe.

                // Mark the previous recording as complete
                if (thread_ctx->writer->current_recording_id > 0) {
                    // Get the file size before marking as complete
                    struct stat st;
                    uint64_t size_bytes = 0;

                    if (stat(current_path, &st) == 0) {
                        size_bytes = st.st_size;
                        log_info("File size for %s: %llu bytes",
                                current_path, (unsigned long long)size_bytes);

                        // Get the actual end time based on the video duration
                        time_t end_time = current_time;

                        // Try to get the actual duration from the MP4 file
                        recording_metadata_t metadata;
                        if (get_recording_metadata_by_id(thread_ctx->writer->current_recording_id, &metadata) == 0) {
                            time_t start_time = metadata.start_time;

                            AVFormatContext *format_ctx = NULL;
                            if (avformat_open_input(&format_ctx, current_path, NULL, NULL) == 0) {
                                if (avformat_find_stream_info(format_ctx, NULL) >= 0) {
                                    if (format_ctx->duration != AV_NOPTS_VALUE) {
                                        // Duration is in AV_TIME_BASE units (microseconds)
                                        int64_t duration_seconds = format_ctx->duration / AV_TIME_BASE;
                                        end_time = start_time + duration_seconds;
                                        log_info("Calculated end_time from MP4 duration: start=%ld, duration=%ld, end=%ld",
                                                (long)start_time, (long)duration_seconds, (long)end_time);
                                    }
                                }
                                avformat_close_input(&format_ctx);
                            }
                        }

                        // Mark the recording as complete with the correct file size and end time
                        update_recording_metadata(thread_ctx->writer->current_recording_id, end_time, size_bytes, true);
                        log_info("Marked previous recording (ID: %llu) as complete for stream %s (size: %llu bytes)",
                                (unsigned long long)thread_ctx->writer->current_recording_id, stream_name, (unsigned long long)size_bytes);
                    } else {
                        log_warn("Failed to get file size for %s: %s",
                                current_path, strerror(errno));

                        // Still mark the recording as complete, but with size 0
                        update_recording_metadata(thread_ctx->writer->current_recording_id, current_time, 0, true);
                        log_info("Marked previous recording (ID: %llu) as complete for stream %s (size unknown)",
                                (unsigned long long)thread_ctx->writer->current_recording_id, stream_name);
                    }
                }

                // Update the output path
                strncpy(thread_ctx->writer->output_path, new_path, MAX_PATH_LENGTH - 1);
                thread_ctx->writer->output_path[MAX_PATH_LENGTH - 1] = '\0';

                // Reset current recording ID; new ID will be assigned on first keyframe of next segment
                thread_ctx->writer->current_recording_id = 0;



                // Update rotation time
                thread_ctx->writer->last_rotation_time = current_time;
            }
        }

        // Record a segment using the record_segment function
        log_info("Recording segment for stream %s to %s", stream_name, thread_ctx->writer->output_path);
        // Use the segment duration from the database or writer
        if (segment_duration > 0) {
            log_info("Using segment duration: %d seconds (from %s)",
                    segment_duration,
                    (db_config_result == 0 && db_stream_config.segment_duration > 0) ? "database" : "writer context");
        } else {
            segment_duration = 30;
            log_info("No segment duration configured, using default: %d seconds", segment_duration);
        }

        // Variables for retry mechanism and resource management
        // CRITICAL FIX: These static variables should be initialized at the beginning of each call
        // to prevent using uninitialized values during shutdown
        static int segment_retry_count = 0;
        static time_t last_segment_retry_time = 0;
        static int segment_count = 0;

        // Initialize static variables at the beginning of each segment recording
        // This ensures they have valid values even during shutdown
        segment_retry_count = 0;
        last_segment_retry_time = 0;
        // Don't reset segment_count as it's used for logging purposes

        // Increment segment count and log it periodically to track memory usage
        segment_count++;
        if (segment_count % 10 == 0) {
            log_info("Stream %s has processed %d segments since startup", stream_name, segment_count);

            // Force a complete FFmpeg resource reset every 10 segments to prevent memory growth
            if (segment_count % 10 == 0) {
                log_info("Performing complete FFmpeg resource reset for stream %s after %d segments",
                        stream_name, segment_count);

                // MEMORY LEAK FIX: Ensure all FFmpeg resources are properly freed
                // This is the most aggressive approach to prevent memory growth

                // 1. Free the packet if it exists
                if (pkt) {
                    av_packet_unref(pkt);
                    av_packet_free(&pkt);
                    pkt = NULL;
                }

                // 2. BUGFIX: Close the per-stream input context if it exists
                if (thread_ctx->input_ctx) {
                    // Flush all buffers before closing
                    if (thread_ctx->input_ctx->pb) {
                        avio_flush(thread_ctx->input_ctx->pb);
                    }

                    // Close the input context - this will free all associated resources
                    // Let FFmpeg handle its own memory management
                    avformat_close_input(&thread_ctx->input_ctx);
                    thread_ctx->input_ctx = NULL;
                }

                // 3. Allocate a new packet
                pkt = av_packet_alloc();
                if (!pkt) {
                    log_error("Failed to allocate packet during FFmpeg resource reset");
                    // Continue anyway, the next iteration will try again
                }

                // Log that we've reset all FFmpeg resources
                log_info("Successfully reset all FFmpeg resources for stream %s", stream_name);
            }
        }

        // Record the segment with timestamp continuity and keyframe handling
        // BUGFIX: Removed duplicate loop that was causing segments to be double the intended length
        log_info("Starting segment recording with info: index=%d, has_audio=%d, last_frame_was_key=%d",
                thread_ctx->segment_info.segment_index, thread_ctx->segment_info.has_audio,
                thread_ctx->segment_info.last_frame_was_key);

        // BUGFIX: Pass per-stream input context and segment info to record_segment
        // This prevents stream mixing when multiple streams are recording simultaneously
        ret = record_segment(thread_ctx->rtsp_url, thread_ctx->writer->output_path,
                           segment_duration, thread_ctx->writer->has_audio,
                           &thread_ctx->input_ctx, &thread_ctx->segment_info,
                           on_segment_started_cb, thread_ctx);

        log_info("Finished segment recording with info: index=%d, has_audio=%d, last_frame_was_key=%d",
                thread_ctx->segment_info.segment_index, thread_ctx->segment_info.has_audio,
                thread_ctx->segment_info.last_frame_was_key);

        if (ret < 0) {
            log_error("Failed to record segment for stream %s (error: %d), implementing retry strategy...",
                     stream_name, ret);

            // BUGFIX: Check if input_ctx is NULL after a failed record_segment call
            // This can happen if the connection failed and avformat_open_input failed
            if (thread_ctx->input_ctx == NULL) {
                log_warn("Input context is NULL after record_segment failure for stream %s", stream_name);
            }

            // Calculate backoff time based on retry count (exponential backoff with max of 30 seconds)
            int backoff_seconds = 1 << (thread_ctx->retry_count > 4 ? 4 : thread_ctx->retry_count); // 1, 2, 4, 8, 16, 16, ...
            if (backoff_seconds > 30) backoff_seconds = 30;

            // Record the retry attempt
            thread_ctx->retry_count++;
            thread_ctx->last_retry_time = time(NULL);

            // If input context was closed, set it to NULL so it will be reopened
            if (!thread_ctx->input_ctx) {
                log_info("Input context was closed, will reopen on next attempt");
            }

            // If we've had too many consecutive failures, try more aggressive recovery
            if (thread_ctx->retry_count > 5) {
                log_warn("Multiple segment recording failures for %s (%d retries), attempting aggressive recovery",
                        stream_name, thread_ctx->retry_count);

                // BUGFIX: Force input context to be recreated
                if (thread_ctx->input_ctx) {
                    avformat_close_input(&thread_ctx->input_ctx);
                    thread_ctx->input_ctx = NULL;
                    log_info("Forcibly closed input context to ensure fresh connection on next attempt");
                }

                // Sleep longer for aggressive recovery
                backoff_seconds = 5;
            }

            log_info("Waiting %d seconds before retrying segment recording for %s (retry #%d)",
                    backoff_seconds, stream_name, thread_ctx->retry_count);

            // Wait before trying again
            av_usleep(backoff_seconds * 1000000);  // Convert to microseconds

            // Continue the loop to retry
            continue;
        } else {
            // Reset retry count on success
            if (thread_ctx->retry_count > 0) {
                log_info("Successfully recorded segment for %s after %d retries",
                        stream_name, thread_ctx->retry_count);
                thread_ctx->retry_count = 0;
            }
        }

        // Update the last packet time for activity tracking
        thread_ctx->writer->last_packet_time = time(NULL);

        // Update the recording metadata with the current file size
        if (thread_ctx->writer->current_recording_id > 0) {
            struct stat st;
            if (stat(thread_ctx->writer->output_path, &st) == 0) {
                uint64_t size_bytes = st.st_size;
                // Update size but don't mark as complete yet
                update_recording_metadata(thread_ctx->writer->current_recording_id, 0, size_bytes, false);
                log_debug("Updated recording metadata for ID: %llu, size: %llu bytes",
                        (unsigned long long)thread_ctx->writer->current_recording_id,
                        (unsigned long long)size_bytes);
            }
        }
    }

    // MEMORY LEAK FIX: Aggressive cleanup of all FFmpeg resources
    log_info("Performing aggressive cleanup of all FFmpeg resources for stream %s", stream_name);

    // 1. Clean up packet resources
    if (pkt) {
        // Make a local copy of the packet pointer and NULL out the original
        // to prevent double-free if another thread accesses it
        AVPacket *pkt_to_free = pkt;
        pkt = NULL;

        // Now safely free the packet - first unref then free to prevent memory leaks
        av_packet_unref(pkt_to_free);
        av_packet_free(&pkt_to_free);
        log_debug("Freed packet resources");
    }

    // 2. BUGFIX: Always ensure per-stream input_ctx is properly closed to prevent memory leaks
    if (thread_ctx->input_ctx) {
        // Make a local copy of the context pointer and NULL out the original
        AVFormatContext *ctx_to_close = thread_ctx->input_ctx;
        thread_ctx->input_ctx = NULL;

        // Flush all buffers before closing
        if (ctx_to_close->pb) {
            avio_flush(ctx_to_close->pb);
            log_debug("Flushed input context buffers");
        }

        // Ensure all packets are properly reference counted before closing
        // This helps prevent use-after-free errors during shutdown
        for (unsigned int i = 0; i < ctx_to_close->nb_streams; i++) {
            if (ctx_to_close->streams[i] && ctx_to_close->streams[i]->codecpar) {
                // Clear any cached packets
                if (ctx_to_close->streams[i]->codecpar->extradata) {
                    log_debug("Clearing extradata for stream %d", i);
                    // Make sure extradata is properly freed
                    av_freep(&ctx_to_close->streams[i]->codecpar->extradata);
                    ctx_to_close->streams[i]->codecpar->extradata_size = 0;
                }
            }
        }

        // Now safely close the input context
        avformat_close_input(&ctx_to_close);

        // Log that we've closed the input context to help with debugging
        log_info("Closed input context for stream %s to prevent memory leaks", stream_name);
    }

    // 2b. BUGFIX: Destroy the context mutex
    pthread_mutex_destroy(&thread_ctx->context_mutex);

    // 3. Notify the segment recorder that we're shutting down
    // This helps ensure proper cleanup of shared resources
    mp4_segment_recorder_cleanup();

    // Log that we've completed cleanup
    log_info("Completed cleanup of FFmpeg resources for stream %s", stream_name);

    log_info("RTSP reading thread for stream %s exited", stream_name);
    return NULL;
}

/**
 * Start a recording thread that reads from the RTSP stream and writes to the MP4 file
 * This function creates a new thread that handles all the recording logic
 */
int mp4_writer_start_recording_thread(mp4_writer_t *writer, const char *rtsp_url) {
    if (!writer || !rtsp_url) {
        return -1;
    }

    // Allocate and initialize thread context
    writer->thread_ctx = (mp4_writer_thread_t *)calloc(1, sizeof(mp4_writer_thread_t));
    if (!writer->thread_ctx) {
        return -1;
    }

    // Initialize thread context
    writer->thread_ctx->writer = writer;
    writer->thread_ctx->running = 0;
    atomic_store(&writer->thread_ctx->shutdown_requested, 0);
    atomic_store(&writer->thread_ctx->force_reconnect, 0);
    strncpy(writer->thread_ctx->rtsp_url, rtsp_url, sizeof(writer->thread_ctx->rtsp_url) - 1);
    writer->thread_ctx->rtsp_url[sizeof(writer->thread_ctx->rtsp_url) - 1] = '\0';

    // Create thread with proper error handling
    int ret = pthread_create(&writer->thread_ctx->thread, NULL, mp4_writer_rtsp_thread, writer->thread_ctx);
    if (ret != 0) {
        free(writer->thread_ctx);
        writer->thread_ctx = NULL;
        return -1;
    }

    // Wait for thread to start
    while (!writer->thread_ctx->running) {
        usleep(1000);  // Sleep for 1ms
    }

    // Register with shutdown coordinator
    writer->shutdown_component_id = register_component(
        writer->stream_name,
        COMPONENT_MP4_WRITER,
        writer,
        10  // Medium priority
    );

    if (writer->shutdown_component_id >= 0) {
        log_info("Registered MP4 writer for %s with shutdown coordinator, component ID: %d",
                writer->stream_name, writer->shutdown_component_id);
    } else {
        log_warn("Failed to register MP4 writer for %s with shutdown coordinator", writer->stream_name);
    }

    log_info("Started self-managing RTSP reading thread for %s", writer->stream_name);

    return 0;
}

/**
 * Stop the recording thread
 * This function signals the recording thread to stop and waits for it to exit
 */
void mp4_writer_stop_recording_thread(mp4_writer_t *writer) {
    if (!writer || !writer->thread_ctx) {
        return;
    }

    // Signal thread to stop
    atomic_store(&writer->thread_ctx->shutdown_requested, 1);

    // Wait for thread to finish
    if (writer->thread_ctx->running) {
        pthread_join(writer->thread_ctx->thread, NULL);
        writer->thread_ctx->running = 0;
    }

    // CRITICAL: Only free resources after thread has completely stopped
    if (writer->thread_ctx->rtsp_url[0] != '\0') {
        memset(writer->thread_ctx->rtsp_url, 0, sizeof(writer->thread_ctx->rtsp_url));
    }

    // Free thread context
    free(writer->thread_ctx);
    writer->thread_ctx = NULL;

    // Update component state in shutdown coordinator
    if (writer->shutdown_component_id >= 0) {
        update_component_state(writer->shutdown_component_id, COMPONENT_STOPPED);
        log_info("Updated MP4 writer component state to STOPPED for %s", writer->stream_name);
    }

    log_info("Stopped RTSP reading thread for %s", writer->stream_name);
}

/**
 * Check if the recording thread is running and actively producing recordings
 *
 * This function checks not only if the thread is running, but also if it has
 * written packets recently. A thread that is "running" but hasn't written
 * packets in a long time is considered dead and should be restarted.
 */
int mp4_writer_is_recording(mp4_writer_t *writer) {
    if (!writer) {
        return 0;
    }

    // If the writer is in the process of rotating, consider it as still recording
    if (writer->is_rotating) {
        return 1;
    }

    mp4_writer_thread_t *thread_ctx = (mp4_writer_thread_t *)writer->thread_ctx;
    if (!thread_ctx) {
        return 0;
    }

    // Check if the thread is running
    if (!thread_ctx->running) {
        return 0;
    }

    // CRITICAL FIX: Check if the recording is actually producing output
    // A thread can be "running" but stuck or not actually writing packets
    // If no packets have been written in the last 90 seconds, consider it dead
    // (90 seconds allows for segment duration of 30 seconds plus some buffer)
    time_t now = time(NULL);
    time_t last_packet = writer->last_packet_time;

    // If last_packet_time is 0, the recording just started - give it time to initialize
    // Allow up to 120 seconds for initial connection and first packet
    if (last_packet == 0) {
        time_t creation_time = writer->creation_time;
        if (creation_time > 0 && (now - creation_time) > 120) {
            log_warn("MP4 recording for stream %s has been running for %ld seconds but never wrote any packets - considering it dead",
                    writer->stream_name, (long)(now - creation_time));
            return 0;
        }
        // Still initializing, consider it running
        return 1;
    }

    // Check if packets have been written recently
    long seconds_since_last_packet = (long)(now - last_packet);
    if (seconds_since_last_packet > 90) {
        log_warn("MP4 recording for stream %s hasn't written packets in %ld seconds - considering it dead",
                writer->stream_name, seconds_since_last_packet);
        return 0;
    }

    return 1;
}

/**
 * Signal the recording thread to force a reconnection
 * This is useful when the upstream source (e.g., go2rtc) has restarted
 */
void mp4_writer_signal_reconnect(mp4_writer_t *writer) {
    if (!writer) {
        return;
    }

    mp4_writer_thread_t *thread_ctx = (mp4_writer_thread_t *)writer->thread_ctx;
    if (!thread_ctx) {
        return;
    }

    if (thread_ctx->running) {
        log_info("Signaling force reconnect for recording thread: %s", writer->stream_name);
        atomic_store(&thread_ctx->force_reconnect, 1);
    }
}
