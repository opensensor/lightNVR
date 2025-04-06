/**
 * MP4 Segment Recorder
 *
 * This module handles the recording of individual MP4 segments from RTSP streams.
 * It's responsible for:
 * - Opening RTSP streams
 * - Creating MP4 files
 * - Handling timestamps and packet processing
 * - Managing segment rotation
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
#include <libavutil/mathematics.h>

#include "core/logger.h"
#include "core/shutdown_coordinator.h"
#include "video/mp4_writer.h"
#include "video/mp4_writer_internal.h"
#include "video/mp4_segment_recorder.h"

// Note: We can't directly access internal FFmpeg structures
// So we'll use the public API for cleanup

// Static variables to maintain state between segment recordings
static AVFormatContext *static_input_ctx = NULL;
static segment_info_t segment_info = {0, false, false};

// Mutex to protect access to static variables
static pthread_mutex_t static_vars_mutex = PTHREAD_MUTEX_INITIALIZER;

/**
 * Initialize the MP4 segment recorder
 * This function should be called during program startup
 */
void mp4_segment_recorder_init(void) {
    // Initialize FFmpeg network
    #if LIBAVFORMAT_VERSION_INT < AV_VERSION_INT(58, 9, 100)
    // For older FFmpeg versions
    av_register_all();
    avformat_network_init();
    #else
    // For newer FFmpeg versions
    avformat_network_init();
    #endif

    // Reset static variables
    pthread_mutex_lock(&static_vars_mutex);
    static_input_ctx = NULL;
    segment_info.segment_index = 0;
    segment_info.has_audio = false;
    segment_info.last_frame_was_key = false;
    pthread_mutex_unlock(&static_vars_mutex);

    log_info("MP4 segment recorder initialized");
}

/**
 * Record an RTSP stream to an MP4 file for a specified duration
 *
 * This function handles the actual recording of an RTSP stream to an MP4 file.
 * It maintains a single RTSP connection across multiple recording segments,
 * ensuring there are no gaps between segments.
 *
 * Error handling:
 * - Network errors: The function will return an error code, but the input context
 *   will be preserved if possible so that the caller can retry.
 * - File system errors: The function will attempt to clean up resources and return
 *   an error code.
 * - Timestamp errors: The function uses a robust timestamp handling approach to
 *   prevent floating point errors and timestamp inflation.
 *
 * @param rtsp_url The URL of the RTSP stream to record
 * @param output_file The path to the output MP4 file
 * @param duration The duration to record in seconds
 * @param has_audio Flag indicating whether to include audio in the recording
 * @return 0 on success, negative value on error
 */
int record_segment(const char *rtsp_url, const char *output_file, int duration, int has_audio) {
    int ret = 0;
    AVFormatContext *input_ctx = NULL;
    AVFormatContext *output_ctx = NULL;
    AVDictionary *opts = NULL;
    AVDictionary *out_opts = NULL;
    AVPacket *pkt;
    int video_stream_idx = -1;
    int audio_stream_idx = -1;
    AVStream *out_video_stream = NULL;
    AVStream *out_audio_stream = NULL;
    int64_t first_video_dts = AV_NOPTS_VALUE;
    int64_t first_video_pts = AV_NOPTS_VALUE;
    int64_t first_audio_dts = AV_NOPTS_VALUE;
    int64_t first_audio_pts = AV_NOPTS_VALUE;
    int64_t last_video_dts = 0;
    int64_t last_video_pts = 0;
    int64_t last_audio_dts = 0;
    int64_t last_audio_pts = 0;
    int audio_packet_count = 0;
    int video_packet_count = 0;
    int64_t start_time;
    time_t last_progress = 0;
    int segment_index = 0;

    // Thread-safe access to static segment info
    pthread_mutex_lock(&static_vars_mutex);
    segment_index = segment_info.segment_index + 1;
    pthread_mutex_unlock(&static_vars_mutex);

    log_info("Starting new segment with index %d", segment_index);

    log_info("Recording from %s", rtsp_url);
    log_info("Output file: %s", output_file);
    log_info("Duration: %d seconds", duration);

    // Thread-safe access to static input context
    pthread_mutex_lock(&static_vars_mutex);
    if (static_input_ctx) {
        input_ctx = static_input_ctx;
        // Clear the static pointer to prevent double free
        static_input_ctx = NULL;
        pthread_mutex_unlock(&static_vars_mutex);
        log_debug("Using existing input context");
    } else {
        pthread_mutex_unlock(&static_vars_mutex);

        // Set up RTSP options for low latency
        av_dict_set(&opts, "rtsp_transport", "tcp", 0);  // Use TCP for RTSP (more reliable than UDP)
        av_dict_set(&opts, "fflags", "nobuffer", 0);     // Reduce buffering
        av_dict_set(&opts, "flags", "low_delay", 0);     // Low delay mode
        av_dict_set(&opts, "max_delay", "500000", 0);    // Maximum delay of 500ms
        av_dict_set(&opts, "stimeout", "5000000", 0);    // Socket timeout in microseconds (5 seconds)

        // Open input
        ret = avformat_open_input(&input_ctx, rtsp_url, NULL, &opts);
        if (ret < 0) {
            char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
            av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
            log_error("Failed to open input: %d (%s)", ret, error_buf);

            // Ensure input_ctx is NULL after a failed open
            input_ctx = NULL;

            // Don't quit, just return an error code so the caller can retry
            goto cleanup;
        }

        // Find stream info
        ret = avformat_find_stream_info(input_ctx, NULL);
        if (ret < 0) {
            log_error("Failed to find stream info: %d", ret);
            goto cleanup;
        }
    }

    // Log input stream info
    log_debug("Input format: %s", input_ctx->iformat->name);
    log_debug("Number of streams: %d", input_ctx->nb_streams);

    // Find video and audio streams
    for (unsigned int i = 0; i < input_ctx->nb_streams; i++) {
        AVStream *stream = input_ctx->streams[i];
        if (stream->codecpar->codec_type == AVMEDIA_TYPE_VIDEO && video_stream_idx < 0) {
            video_stream_idx = i;
            log_debug("Found video stream: %d", i);
            log_debug("  Codec: %s", avcodec_get_name(stream->codecpar->codec_id));
            log_debug("  Resolution: %dx%d", stream->codecpar->width, stream->codecpar->height);
            if (stream->avg_frame_rate.num && stream->avg_frame_rate.den) {
                log_debug("  Frame rate: %.2f fps",
                       (float)stream->avg_frame_rate.num / stream->avg_frame_rate.den);
            }
        } else if (stream->codecpar->codec_type == AVMEDIA_TYPE_AUDIO && audio_stream_idx < 0) {
            audio_stream_idx = i;
            log_debug("Found audio stream: %d", i);
            log_debug("  Codec: %s", avcodec_get_name(stream->codecpar->codec_id));
            log_debug("  Sample rate: %d Hz", stream->codecpar->sample_rate);
            // Handle channel count for different FFmpeg versions
        }
    }

    if (video_stream_idx < 0) {
        log_error("No video stream found");
        ret = -1;
        goto cleanup;
    }

    // Create output context
    ret = avformat_alloc_output_context2(&output_ctx, NULL, "mp4", output_file);
    if (ret < 0 || !output_ctx) {
        log_error("Failed to create output context: %d", ret);
        goto cleanup;
    }

    // Add video stream
    out_video_stream = avformat_new_stream(output_ctx, NULL);
    if (!out_video_stream) {
        log_error("Failed to create output video stream");
        ret = -1;
        goto cleanup;
    }

    // Copy video codec parameters
    ret = avcodec_parameters_copy(out_video_stream->codecpar,
                                 input_ctx->streams[video_stream_idx]->codecpar);
    if (ret < 0) {
        log_error("Failed to copy video codec parameters: %d", ret);
        goto cleanup;
    }

    // Set video stream time base
    out_video_stream->time_base = input_ctx->streams[video_stream_idx]->time_base;

    // Add audio stream if available and audio is enabled
    if (audio_stream_idx >= 0 && has_audio) {
        log_info("Including audio stream in MP4 recording");
        out_audio_stream = avformat_new_stream(output_ctx, NULL);
        if (!out_audio_stream) {
            log_error("Failed to create output audio stream");
            ret = -1;
            goto cleanup;
        }

        // Copy audio codec parameters
        ret = avcodec_parameters_copy(out_audio_stream->codecpar,
                                     input_ctx->streams[audio_stream_idx]->codecpar);
        if (ret < 0) {
            log_error("Failed to copy audio codec parameters: %d", ret);
            goto cleanup;
        }

        // Set audio stream time base
        out_audio_stream->time_base = input_ctx->streams[audio_stream_idx]->time_base;
    }

    // CRITICAL FIX: Disable faststart to prevent segmentation faults
    // The faststart option causes a second pass that moves the moov atom to the beginning of the file
    // This second pass is causing segmentation faults during shutdown
    av_dict_set(&out_opts, "movflags", "empty_moov", 0);

    // Open output file
    ret = avio_open(&output_ctx->pb, output_file, AVIO_FLAG_WRITE);
    if (ret < 0) {
        log_error("Failed to open output file: %d", ret);
        goto cleanup;
    }

    // Write file header
    ret = avformat_write_header(output_ctx, &out_opts);
    if (ret < 0) {
        log_error("Failed to write header: %d", ret);
        goto cleanup;
    }

    // Initialize packet - ensure it's properly allocated and initialized
    pkt = av_packet_alloc();
    if (!pkt) {
        log_error("Failed to allocate packet");
        ret = AVERROR(ENOMEM);
        goto cleanup;
    }
    // Initialize packet fields
    pkt->data = NULL;
    pkt->size = 0;
    pkt->stream_index = -1;

    // Start recording
    start_time = av_gettime();
    log_info("Recording started...");

    // Initialize timestamp tracking variables
    int consecutive_timestamp_errors = 0;
    int max_timestamp_errors = 5;  // Maximum number of consecutive timestamp errors before resetting

    // Flag to track if we've found the first key frame
    bool found_first_keyframe = false;
    // Flag to track if we're waiting for the final key frame
    bool waiting_for_final_keyframe = false;
    // Flag to track if shutdown was detected
    bool shutdown_detected = false;

    // Main recording loop
    while (1) {
        // Check if shutdown has been initiated
        if (!shutdown_detected && !waiting_for_final_keyframe && is_shutdown_initiated()) {
            log_info("Shutdown initiated, waiting for next key frame to end recording");
            waiting_for_final_keyframe = true;
            shutdown_detected = true;
        }

        // Check if we've reached the duration limit
        if (duration > 0 && !waiting_for_final_keyframe && !shutdown_detected) {
            int64_t elapsed_seconds = (av_gettime() - start_time) / 1000000;

            // If we've reached the duration limit, wait for the next key frame
            if (elapsed_seconds >= duration) {
                log_info("Reached duration limit of %d seconds, waiting for next key frame to end recording", duration);
                waiting_for_final_keyframe = true;
            }
            // If we're close to the duration limit (within 1 second), also wait for the next key frame
            // This helps ensure we don't wait too long for a key frame at the end of a segment
            // Reduced from 3 to 1 second to prevent segments from being too long
            else if (elapsed_seconds >= duration - 1) {
                log_info("Within 1 second of duration limit (%d seconds), waiting for next key frame to end recording", duration);
                waiting_for_final_keyframe = true;
            }
        }

        // Read packet
        ret = av_read_frame(input_ctx, pkt);
        if (ret < 0) {
            if (ret == AVERROR_EOF) {
                log_info("End of stream reached");
                break;
            } else if (ret != AVERROR(EAGAIN)) {
                log_error("Error reading frame: %d", ret);
                break;
            }
            // EAGAIN means try again, so we continue
            av_usleep(10000);  // Sleep 10ms to avoid busy waiting
            continue;
        }

        // Process video packets
        if (pkt->stream_index == video_stream_idx) {
            // Check if this is a key frame
            bool is_keyframe = (pkt->flags & AV_PKT_FLAG_KEY) != 0;

            // If we're waiting for the first key frame
            if (!found_first_keyframe) {
            // If the previous segment ended with a key frame, we can start immediately with this frame
            // Otherwise, wait for a key frame
            if (segment_info.last_frame_was_key && segment_index > 0) {
                    log_info("Previous segment ended with a key frame, starting new segment immediately");
                    found_first_keyframe = true;

                    // Reset start time to now
                    start_time = av_gettime();

                    // If this frame is not a keyframe, log a warning as we might have missed the keyframe
                    if (!is_keyframe) {
                        log_warn("Starting segment with non-keyframe even though previous segment ended with keyframe");
                    }
                } else if (is_keyframe) {
                    log_info("Found first key frame, starting recording");
                    found_first_keyframe = true;

                    // Reset start time to when we found the first key frame
                    start_time = av_gettime();
                } else {
                    // For regular segments, always wait for a key frame
                    // Skip this frame as we're waiting for a key frame
                    av_packet_unref(pkt);
                    continue;
                }
            }

            // If we're waiting for the final key frame to end recording
            if (waiting_for_final_keyframe) {
                // Check if this is a key frame or if we've been waiting too long
                static int64_t waiting_start_time = 0;

                // Initialize waiting start time if not set
                if (waiting_start_time == 0) {
                    waiting_start_time = av_gettime();
                }

                // Calculate how long we've been waiting for a key frame
                int64_t wait_time = (av_gettime() - waiting_start_time) / 1000000;

                // If this is a key frame or we've waited too long (more than 1 second)
                // Reduced from 2 to 1 second to improve segment length precision
                if (is_keyframe || wait_time > 1) {
                    if (is_keyframe) {
                        log_info("Found final key frame, ending recording");
                        // Set flag to indicate the last frame was a key frame
                        segment_info.last_frame_was_key = true;
                        log_debug("Last frame was a key frame, next segment will start immediately with this keyframe");
                    } else {
                        log_info("Waited %lld seconds for key frame, ending recording with non-key frame", (long long)wait_time);
                        // Clear flag since the last frame was not a key frame
                        segment_info.last_frame_was_key = false;
                        log_debug("Last frame was NOT a key frame, next segment will wait for a keyframe");
                    }

                    // Process this final frame and then break the loop
                    // Initialize first DTS if not set
                    if (first_video_dts == AV_NOPTS_VALUE && pkt->dts != AV_NOPTS_VALUE) {
                        first_video_dts = pkt->dts;
                        first_video_pts = pkt->pts != AV_NOPTS_VALUE ? pkt->pts : pkt->dts;
                        log_debug("First video DTS: %lld, PTS: %lld",
                                (long long)first_video_dts, (long long)first_video_pts);
                    }

                    // Handle timestamps based on segment index
                    if (segment_index == 0) {
                        // First segment - adjust timestamps relative to first_dts
                        if (pkt->dts != AV_NOPTS_VALUE && first_video_dts != AV_NOPTS_VALUE) {
                            pkt->dts -= first_video_dts;
                            if (pkt->dts < 0) pkt->dts = 0;
                        }

                        if (pkt->pts != AV_NOPTS_VALUE && first_video_pts != AV_NOPTS_VALUE) {
                            pkt->pts -= first_video_pts;
                            if (pkt->pts < 0) pkt->pts = 0;
                        }
                    } else {
                        // Subsequent segments - maintain timestamp continuity
                        // CRITICAL FIX: Use a small fixed offset instead of carrying over potentially large timestamps
                        // This prevents the timestamp inflation issue while still maintaining continuity
                        if (pkt->dts != AV_NOPTS_VALUE && first_video_dts != AV_NOPTS_VALUE) {
                            // Calculate relative timestamp within this segment
                            int64_t relative_dts = pkt->dts - first_video_dts;
                            // Add a small fixed offset (e.g., 1/30th of a second in timebase units)
                            // This ensures continuity without timestamp inflation
                            pkt->dts = relative_dts + 1;
                        }

                        if (pkt->pts != AV_NOPTS_VALUE && first_video_pts != AV_NOPTS_VALUE) {
                            int64_t relative_pts = pkt->pts - first_video_pts;
                            pkt->pts = relative_pts + 1;
                        }
                    }

                    // CRITICAL FIX: Ensure PTS >= DTS for video packets to prevent "pts < dts" errors
                    // This is essential for MP4 format compliance and prevents ghosting artifacts
                    if (pkt->pts != AV_NOPTS_VALUE && pkt->dts != AV_NOPTS_VALUE && pkt->pts < pkt->dts) {
                        log_debug("Fixing video packet with PTS < DTS: PTS=%lld, DTS=%lld",
                                 (long long)pkt->pts, (long long)pkt->dts);
                        pkt->pts = pkt->dts;
                    }

                    // CRITICAL FIX: Ensure DTS values don't exceed MP4 format limits (0x7fffffff)
                    // This prevents the "Assertion next_dts <= 0x7fffffff failed" error
                    if (pkt->dts != AV_NOPTS_VALUE) {
                        if (pkt->dts > 0x7fffffff) {
                            log_warn("DTS value exceeds MP4 format limit: %lld, resetting to safe value", (long long)pkt->dts);
                            // Reset to a small value that maintains continuity
                            pkt->dts = 1000;
                            if (pkt->pts != AV_NOPTS_VALUE) {
                                // Maintain PTS-DTS relationship if possible
                                int64_t pts_dts_diff = pkt->pts - pkt->dts;
                                if (pts_dts_diff >= 0) {
                                    pkt->pts = pkt->dts + pts_dts_diff;
                                } else {
                                    pkt->pts = pkt->dts;
                                }
                            } else {
                                pkt->pts = pkt->dts;
                            }
                        }

                        // Additional check to ensure DTS is always within safe range
                        // This handles cases where DTS might be close to the limit
                        if (pkt->dts > 0x70000000) {  // ~75% of max value
                            log_info("DTS value approaching MP4 format limit: %lld, resetting to prevent overflow", (long long)pkt->dts);
                            // Reset to a small value
                            pkt->dts = 1000;
                            if (pkt->pts != AV_NOPTS_VALUE) {
                                // Maintain PTS-DTS relationship
                                pkt->pts = pkt->dts + 1;
                            } else {
                                pkt->pts = pkt->dts;
                            }
                        }
                    }

                    // CRITICAL FIX: Ensure packet duration is within reasonable limits
                    // This prevents the "Packet duration is out of range" error
                    if (pkt->duration > 10000000) {
                        log_warn("Packet duration too large: %lld, capping at reasonable value", (long long)pkt->duration);
                        // Cap at a reasonable value (e.g., 1 second in timebase units)
                        pkt->duration = 90000;
                    }

                    // Update last timestamps
                    if (pkt->dts != AV_NOPTS_VALUE) {
                        last_video_dts = pkt->dts;
                    }
                    if (pkt->pts != AV_NOPTS_VALUE) {
                        last_video_pts = pkt->pts;
                    }

                    // Explicitly set duration for the final frame to prevent segmentation fault
                    if (pkt->duration == 0 || pkt->duration == AV_NOPTS_VALUE) {
                        // Use the time base of the video stream to calculate a reasonable duration
                        if (input_ctx->streams[video_stream_idx]->avg_frame_rate.num > 0 &&
                            input_ctx->streams[video_stream_idx]->avg_frame_rate.den > 0) {
                            // Calculate duration based on framerate (time_base units)
                            pkt->duration = av_rescale_q(1,
                                                       av_inv_q(input_ctx->streams[video_stream_idx]->avg_frame_rate),
                                                       input_ctx->streams[video_stream_idx]->time_base);
                        } else {
                            // Default to a reasonable value if framerate is not available
                            pkt->duration = 1;
                        }
                        log_debug("Set final frame duration to %lld", (long long)pkt->duration);
                    }

                    // Set output stream index
                    pkt->stream_index = out_video_stream->index;

                    // Write packet
                    ret = av_interleaved_write_frame(output_ctx, pkt);
                    if (ret < 0) {
                        log_error("Error writing video frame: %d", ret);
                    }

                    // Break the loop after processing the final frame
                    av_packet_unref(pkt);
                    break;
                }
            }

            // Initialize first DTS if not set
            if (first_video_dts == AV_NOPTS_VALUE && pkt->dts != AV_NOPTS_VALUE) {
                first_video_dts = pkt->dts;
                first_video_pts = pkt->pts != AV_NOPTS_VALUE ? pkt->pts : pkt->dts;
                log_debug("First video DTS: %lld, PTS: %lld",
                        (long long)first_video_dts, (long long)first_video_pts);
            }

            // Handle timestamps based on segment index
            if (segment_index == 0) {
                // First segment - adjust timestamps relative to first_dts
                if (pkt->dts != AV_NOPTS_VALUE && first_video_dts != AV_NOPTS_VALUE) {
                    pkt->dts -= first_video_dts;
                    if (pkt->dts < 0) pkt->dts = 0;
                }

                if (pkt->pts != AV_NOPTS_VALUE && first_video_pts != AV_NOPTS_VALUE) {
                    pkt->pts -= first_video_pts;
                    if (pkt->pts < 0) pkt->pts = 0;
                }
            } else {
                // Subsequent segments - maintain timestamp continuity
                // CRITICAL FIX: Use a small fixed offset instead of carrying over potentially large timestamps
                // This prevents the timestamp inflation issue while still maintaining continuity
                if (pkt->dts != AV_NOPTS_VALUE && first_video_dts != AV_NOPTS_VALUE) {
                    // Calculate relative timestamp within this segment
                    int64_t relative_dts = pkt->dts - first_video_dts;
                    // Add a small fixed offset (e.g., 1/30th of a second in timebase units)
                    // This ensures continuity without timestamp inflation
                    pkt->dts = relative_dts + 1;
                }

                if (pkt->pts != AV_NOPTS_VALUE && first_video_pts != AV_NOPTS_VALUE) {
                    int64_t relative_pts = pkt->pts - first_video_pts;
                    pkt->pts = relative_pts + 1;
                }
            }

            // CRITICAL FIX: Ensure PTS >= DTS for video packets to prevent "pts < dts" errors
            // This is essential for MP4 format compliance and prevents ghosting artifacts
            if (pkt->pts != AV_NOPTS_VALUE && pkt->dts != AV_NOPTS_VALUE && pkt->pts < pkt->dts) {
                log_debug("Fixing video packet with PTS < DTS: PTS=%lld, DTS=%lld",
                         (long long)pkt->pts, (long long)pkt->dts);
                pkt->pts = pkt->dts;
            }

            // CRITICAL FIX: Ensure monotonically increasing DTS values
            // This prevents the "Application provided invalid, non monotonically increasing dts" error
            if (pkt->dts != AV_NOPTS_VALUE && last_video_dts != 0 && pkt->dts <= last_video_dts) {
                int64_t fixed_dts = last_video_dts + 1;
                log_debug("Fixing non-monotonic DTS: old=%lld, last=%lld, new=%lld",
                         (long long)pkt->dts, (long long)last_video_dts, (long long)fixed_dts);

                // Maintain the PTS-DTS relationship if possible
                if (pkt->pts != AV_NOPTS_VALUE) {
                    int64_t pts_dts_diff = pkt->pts - pkt->dts;
                    pkt->dts = fixed_dts;
                    pkt->pts = fixed_dts + (pts_dts_diff > 0 ? pts_dts_diff : 0);
                } else {
                    pkt->dts = fixed_dts;
                    pkt->pts = fixed_dts;
                }
            }

            // Update last timestamps
            if (pkt->dts != AV_NOPTS_VALUE) {
                last_video_dts = pkt->dts;
            }
            if (pkt->pts != AV_NOPTS_VALUE) {
                last_video_pts = pkt->pts;
            }

            // Explicitly set duration to prevent segmentation fault during fragment writing
            // This addresses the "Estimating the duration of the last packet in a fragment" error
            if (pkt->duration == 0 || pkt->duration == AV_NOPTS_VALUE) {
                // Use the time base of the video stream to calculate a reasonable duration
                // For most video streams, this will be 1/framerate
                if (input_ctx->streams[video_stream_idx]->avg_frame_rate.num > 0 &&
                    input_ctx->streams[video_stream_idx]->avg_frame_rate.den > 0) {
                    // Calculate duration based on framerate (time_base units)
                    pkt->duration = av_rescale_q(1,
                                               av_inv_q(input_ctx->streams[video_stream_idx]->avg_frame_rate),
                                               input_ctx->streams[video_stream_idx]->time_base);
                } else {
                    // Default to a reasonable value if framerate is not available
                    pkt->duration = 1;
                }
                log_debug("Set video packet duration to %lld", (long long)pkt->duration);
            }

            // Set output stream index
            pkt->stream_index = out_video_stream->index;

            // Write packet
            ret = av_interleaved_write_frame(output_ctx, pkt);
            if (ret < 0) {
                char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
                av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
                log_error("Error writing video frame: %d (%s)", ret, error_buf);

                // CRITICAL FIX: Handle timestamp-related errors
                if (ret == AVERROR(EINVAL) && strstr(error_buf, "monoton")) {
                    // This is likely a timestamp error, try to fix it for the next packet
                    log_warn("Detected timestamp error, will try to fix for next packet");

                    // Increment the consecutive error counter
                    consecutive_timestamp_errors++;

                    if (consecutive_timestamp_errors >= max_timestamp_errors) {
                        // Too many consecutive errors, reset all timestamps
                        log_warn("Too many consecutive timestamp errors (%d), resetting all timestamps",
                                consecutive_timestamp_errors);

                        // Reset timestamps to small values
                        first_video_dts = 0;
                        first_video_pts = 0;
                        last_video_dts = 0;
                        last_video_pts = 0;
                        first_audio_dts = 0;
                        first_audio_pts = 0;
                        last_audio_dts = 0;
                        last_audio_pts = 0;

                        // Reset the error counter
                        consecutive_timestamp_errors = 0;
                    } else {
                        // Force a larger increment for the next packet to avoid timestamp issues
                        last_video_dts += 100 * consecutive_timestamp_errors;
                        last_video_pts += 100 * consecutive_timestamp_errors;
                    }
                }
            } else {
                // Reset consecutive error counter on success
                consecutive_timestamp_errors = 0;

                video_packet_count++;
                if (video_packet_count % 300 == 0) {
                    log_debug("Processed %d video packets", video_packet_count);
                }
            }
        }
        // Process audio packets - only if audio is enabled and we have an audio output stream
        else if (has_audio && audio_stream_idx >= 0 && pkt->stream_index == audio_stream_idx && out_audio_stream) {
            // Skip audio packets until we've found the first video keyframe
            if (!found_first_keyframe) {
                av_packet_unref(pkt);
                continue;
            }

            // Initialize first audio DTS if not set
            if (first_audio_dts == AV_NOPTS_VALUE && pkt->dts != AV_NOPTS_VALUE) {
                first_audio_dts = pkt->dts;
                first_audio_pts = pkt->pts != AV_NOPTS_VALUE ? pkt->pts : pkt->dts;
                log_debug("First audio DTS: %lld, PTS: %lld",
                        (long long)first_audio_dts, (long long)first_audio_pts);
            }

            // Handle timestamps based on segment index
            if (segment_index == 0) {
                // First segment - adjust timestamps relative to first_dts
                if (pkt->dts != AV_NOPTS_VALUE && first_audio_dts != AV_NOPTS_VALUE) {
                    pkt->dts -= first_audio_dts;
                    if (pkt->dts < 0) pkt->dts = 0;
                }

                if (pkt->pts != AV_NOPTS_VALUE && first_audio_pts != AV_NOPTS_VALUE) {
                    pkt->pts -= first_audio_pts;
                    if (pkt->pts < 0) pkt->pts = 0;
                }
            } else {
                // Subsequent segments - maintain timestamp continuity
                // CRITICAL FIX: Use a small fixed offset instead of carrying over potentially large timestamps
                // This prevents the timestamp inflation issue while still maintaining continuity
                if (pkt->dts != AV_NOPTS_VALUE && first_audio_dts != AV_NOPTS_VALUE) {
                    // Calculate relative timestamp within this segment
                    int64_t relative_dts = pkt->dts - first_audio_dts;
                    // Add a small fixed offset (e.g., 1/30th of a second in timebase units)
                    // This ensures continuity without timestamp inflation
                    pkt->dts = relative_dts + 1;
                }

                if (pkt->pts != AV_NOPTS_VALUE && first_audio_pts != AV_NOPTS_VALUE) {
                    int64_t relative_pts = pkt->pts - first_audio_pts;
                    pkt->pts = relative_pts + 1;
                }
            }

            // Ensure monotonic increase of timestamps
            if (audio_packet_count > 0) {
                // CRITICAL FIX: More robust handling of non-monotonic DTS values
                if (pkt->dts != AV_NOPTS_VALUE && pkt->dts <= last_audio_dts) {
                    int64_t fixed_dts = last_audio_dts + 1;
                    log_debug("Fixing non-monotonic audio DTS: old=%lld, last=%lld, new=%lld",
                             (long long)pkt->dts, (long long)last_audio_dts, (long long)fixed_dts);
                    pkt->dts = fixed_dts;
                }

                if (pkt->pts != AV_NOPTS_VALUE && pkt->pts <= last_audio_pts) {
                    int64_t fixed_pts = last_audio_pts + 1;
                    log_debug("Fixing non-monotonic audio PTS: old=%lld, last=%lld, new=%lld",
                             (long long)pkt->pts, (long long)last_audio_pts, (long long)fixed_pts);
                    pkt->pts = fixed_pts;
                }

                // Ensure PTS >= DTS
                if (pkt->pts != AV_NOPTS_VALUE && pkt->dts != AV_NOPTS_VALUE && pkt->pts < pkt->dts) {
                    log_debug("Fixing audio packet with PTS < DTS: PTS=%lld, DTS=%lld",
                             (long long)pkt->pts, (long long)pkt->dts);
                    pkt->pts = pkt->dts;
                }
            }

            // CRITICAL FIX: Ensure DTS values don't exceed MP4 format limits (0x7fffffff) for audio packets
            if (pkt->dts != AV_NOPTS_VALUE) {
                if (pkt->dts > 0x7fffffff) {
                    log_warn("Audio DTS value exceeds MP4 format limit: %lld, resetting to safe value", (long long)pkt->dts);
                    pkt->dts = 1000;
                    if (pkt->pts != AV_NOPTS_VALUE) {
                        // Maintain PTS-DTS relationship if possible
                        int64_t pts_dts_diff = pkt->pts - pkt->dts;
                        if (pts_dts_diff >= 0) {
                            pkt->pts = pkt->dts + pts_dts_diff;
                        } else {
                            pkt->pts = pkt->dts;
                        }
                    } else {
                        pkt->pts = pkt->dts;
                    }
                }

                // Additional check to ensure DTS is always within safe range
                if (pkt->dts > 0x70000000) {  // ~75% of max value
                    log_info("Audio DTS value approaching MP4 format limit: %lld, resetting to prevent overflow", (long long)pkt->dts);
                    pkt->dts = 1000;
                    if (pkt->pts != AV_NOPTS_VALUE) {
                        // Maintain PTS-DTS relationship
                        pkt->pts = pkt->dts + 1;
                    } else {
                        pkt->pts = pkt->dts;
                    }
                }
            }

            // Update last timestamps
            if (pkt->dts != AV_NOPTS_VALUE) {
                last_audio_dts = pkt->dts;
            }
            if (pkt->pts != AV_NOPTS_VALUE) {
                last_audio_pts = pkt->pts;
            }

            // Explicitly set duration to prevent segmentation fault during fragment writing
            if (pkt->duration == 0 || pkt->duration == AV_NOPTS_VALUE) {
                // For audio, we can calculate duration based on sample rate and frame size
                AVStream *audio_stream = input_ctx->streams[audio_stream_idx];
                if (audio_stream->codecpar->sample_rate > 0) {
                    // If we know the number of samples in this packet, use that
                    int nb_samples = 0;

                    // Try to get the number of samples from the codec parameters
#if LIBAVCODEC_VERSION_INT >= AV_VERSION_INT(59, 37, 100)
                    // For FFmpeg 5.0 and newer
                    if (audio_stream->codecpar->ch_layout.nb_channels > 0 &&
                        audio_stream->codecpar->bits_per_coded_sample > 0) {
                        int bytes_per_sample = audio_stream->codecpar->bits_per_coded_sample / 8;
                        // Ensure we don't divide by zero
                        if (bytes_per_sample > 0) {
                            nb_samples = pkt->size / (audio_stream->codecpar->ch_layout.nb_channels * bytes_per_sample);
                        }
                    }
#else
                    // For older FFmpeg versions
                    if (audio_stream->codecpar->channels > 0 &&
                        audio_stream->codecpar->bits_per_coded_sample > 0) {
                        int bytes_per_sample = audio_stream->codecpar->bits_per_coded_sample / 8;
                        // Ensure we don't divide by zero
                        if (bytes_per_sample > 0) {
                            nb_samples = pkt->size / (audio_stream->codecpar->channels * bytes_per_sample);
                        }
                    }
#endif

                    if (nb_samples > 0) {
                        // Calculate duration based on samples and sample rate
                        pkt->duration = av_rescale_q(nb_samples,
                                                  (AVRational){1, audio_stream->codecpar->sample_rate},
                                                  audio_stream->time_base);
                    } else {
                        // Default to a reasonable value based on sample rate
                        // Typically audio frames are ~20-40ms, so we'll use 1024 samples as a common value
                        pkt->duration = av_rescale_q(1024,
                                                  (AVRational){1, audio_stream->codecpar->sample_rate},
                                                  audio_stream->time_base);
                    }
                } else {
                    // If we can't calculate based on sample rate, use a default value
                    pkt->duration = 1;
                    log_debug("Set default audio packet duration to 1");
                }
            }

            // Set output stream index
            pkt->stream_index = out_audio_stream->index;

            // Write packet
            ret = av_interleaved_write_frame(output_ctx, pkt);
            if (ret < 0) {
                char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
                av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
                log_error("Error writing audio frame: %d (%s)", ret, error_buf);

                // CRITICAL FIX: Handle timestamp-related errors
                if (ret == AVERROR(EINVAL) && strstr(error_buf, "monoton")) {
                    // This is likely a timestamp error, try to fix it for the next packet
                    log_warn("Detected audio timestamp error, will try to fix for next packet");

                    // Increment the consecutive error counter
                    consecutive_timestamp_errors++;

                    if (consecutive_timestamp_errors >= max_timestamp_errors) {
                        // Too many consecutive errors, reset all timestamps
                        log_warn("Too many consecutive audio timestamp errors (%d), resetting all timestamps",
                                consecutive_timestamp_errors);

                        // Reset timestamps to small values
                        first_video_dts = 0;
                        first_video_pts = 0;
                        last_video_dts = 0;
                        last_video_pts = 0;
                        first_audio_dts = 0;
                        first_audio_pts = 0;
                        last_audio_dts = 0;
                        last_audio_pts = 0;

                        // Reset the error counter
                        consecutive_timestamp_errors = 0;
                    } else {
                        // Force a larger increment for the next packet to avoid timestamp issues
                        last_audio_dts += 100 * consecutive_timestamp_errors;
                        last_audio_pts += 100 * consecutive_timestamp_errors;
                    }
                }
            } else {
                // Reset consecutive error counter on success
                consecutive_timestamp_errors = 0;

                audio_packet_count++;
                if (audio_packet_count % 300 == 0) {
                    log_debug("Processed %d audio packets", audio_packet_count);
                }
            }
        }

        // Unref packet
        av_packet_unref(pkt);
    }

    log_info("Recording segment complete (video packets: %d, audio packets: %d)",
            video_packet_count, audio_packet_count);

    // Flag to track if trailer has been written
    bool trailer_written = false;

    // Write trailer
    if (output_ctx && output_ctx->pb) {
        ret = av_write_trailer(output_ctx);
        if (ret < 0) {
            log_error("Failed to write trailer: %d", ret);
        } else {
            trailer_written = true;
            log_debug("Successfully wrote trailer to output file");
        }
    }

    // Thread-safe update of segment info for the next segment
    pthread_mutex_lock(&static_vars_mutex);
    segment_info.segment_index = segment_index;
    segment_info.has_audio = has_audio && audio_stream_idx >= 0;
    pthread_mutex_unlock(&static_vars_mutex);

    log_info("Saved segment info for next segment: index=%d, has_audio=%d, last_frame_was_key=%d",
            segment_index, has_audio && audio_stream_idx >= 0, segment_info.last_frame_was_key);

cleanup:
    // CRITICAL FIX: Aggressive cleanup to prevent memory growth over time
    log_debug("Starting aggressive cleanup of FFmpeg resources");

    // Free dictionaries - these are always safe to free
    av_dict_free(&opts);
    av_dict_free(&out_opts);

    // Safely flush input context if it exists
    if (input_ctx && input_ctx->pb) {
        log_debug("Flushing input context");
        avio_flush(input_ctx->pb);
    }

    // Safely flush output context if it exists
    if (output_ctx && output_ctx->pb) {
        log_debug("Flushing output context");
        avio_flush(output_ctx->pb);
    }

    // Clean up output context if it was created
    if (output_ctx) {
        log_debug("Cleaning up output context");

        // Only write trailer if we successfully wrote the header and it hasn't been written yet
        if (output_ctx->pb && ret >= 0 && !trailer_written) {
            log_debug("Writing trailer during cleanup");
            av_write_trailer(output_ctx);
        }

        // Close output file if it was opened
        if (output_ctx->pb) {
            log_debug("Closing output file");
            avio_closep(&output_ctx->pb);
        }

        // MEMORY LEAK FIX: Properly clean up all streams in the output context
        // This ensures all codec contexts and other resources are freed
        if (output_ctx->nb_streams > 0) {
            log_debug("Cleaning up %d output streams", output_ctx->nb_streams);
            for (unsigned int i = 0; i < output_ctx->nb_streams; i++) {
                if (output_ctx->streams[i]) {
                    // Free any codec parameters
                    if (output_ctx->streams[i]->codecpar) {
                        avcodec_parameters_free(&output_ctx->streams[i]->codecpar);
                    }
                }
            }
        }

        // Free output context
        log_debug("Freeing output context");
        avformat_free_context(output_ctx);
        output_ctx = NULL;
    }

    // CRITICAL FIX: Properly handle the input context to prevent memory leaks
    log_debug("Handling input context cleanup");

    // Store the input context for reuse if recording was successful
    if (ret >= 0) {
        pthread_mutex_lock(&static_vars_mutex);
        // Only store if there's no existing context (should never happen, but just in case)
        if (static_input_ctx == NULL) {
            // We can't directly access internal FFmpeg structures
            // Just store the context as is and rely on FFmpeg's internal reference counting

            static_input_ctx = input_ctx;
            // Don't close the input context as we're keeping it for the next segment
            input_ctx = NULL;
            log_debug("Stored input context for reuse in next segment");
        } else {
            // This should never happen, but if it does, close the current context
            log_warn("Static input context already exists, closing current context");

            // Clean up all streams before closing
            for (unsigned int i = 0; i < input_ctx->nb_streams; i++) {
                if (input_ctx->streams[i]) {
                    // Free any codec parameters
                    if (input_ctx->streams[i]->codecpar) {
                        avcodec_parameters_free(&input_ctx->streams[i]->codecpar);
                    }
                }
            }

            avformat_close_input(&input_ctx);
        }
        pthread_mutex_unlock(&static_vars_mutex);
    } else
    {
        // If there was an error, close the input context
        log_debug("Closing input context due to error");

        // Clean up all streams before closing
        for (unsigned int i = 0; i < input_ctx->nb_streams; i++) {
            if (input_ctx->streams[i]) {
                // Free any codec parameters
                if (input_ctx->streams[i]->codecpar) {
                    avcodec_parameters_free(&input_ctx->streams[i]->codecpar);
                }
            }
        }

        avformat_close_input(&input_ctx);
        log_debug("Closed input context due to error");
    }

    // MEMORY LEAK FIX: Ensure packet is unref'd and freed before returning
    // This is a safety measure in case we jumped to cleanup without unreferencing the packet
    if (pkt) {
        log_debug("Cleaning up packet");
        av_packet_unref(pkt);
        av_packet_free(&pkt);
    }

    // Final FFmpeg cleanup to prevent memory leaks
    // This helps clean up any internal FFmpeg resources that might not be properly freed
    av_log_set_level(AV_LOG_QUIET);  // Suppress any warnings during cleanup

    avformat_network_deinit();
    av_dict_free(&opts);

    // Return the error code
    return ret;
}

/**
 * Clean up all static resources used by the MP4 segment recorder
 * This function should be called during program shutdown to prevent memory leaks
 */
void mp4_segment_recorder_cleanup(void) {
    // Thread-safe cleanup of static resources
    pthread_mutex_lock(&static_vars_mutex);

    // Clean up static input context if it exists
    if (static_input_ctx) {
        log_info("Cleaning up static input context during shutdown");

        // Flush the input context before closing it
        if (static_input_ctx->pb) {
            avio_flush(static_input_ctx->pb);
        }

        // Let avformat_close_input handle the cleanup of streams and codecs

        // Close the input context
        avformat_close_input(&static_input_ctx);
        static_input_ctx = NULL;
        log_debug("Cleaned up static input context during shutdown");
    }

    // Reset segment info
    segment_info.segment_index = 0;
    segment_info.has_audio = false;
    segment_info.last_frame_was_key = false;

    pthread_mutex_unlock(&static_vars_mutex);

    // Call FFmpeg's global cleanup functions to release any global resources
    // This helps clean up resources that might not be freed otherwise

    // Set log level to quiet to suppress any warnings during cleanup
    av_log_set_level(AV_LOG_QUIET);

    // Clean up network resources
    // Note: This is safe to call during shutdown as we're ensuring all contexts are closed first
    avformat_network_deinit();

    log_info("MP4 segment recorder resources cleaned up");
}
