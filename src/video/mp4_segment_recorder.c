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

// BUGFIX: Removed global static variables that were causing stream mixing
// The input context and segment info are now per-stream, passed as parameters

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

    // BUGFIX: No longer need to reset global static variables
    // Each stream now has its own input context and segment info

    log_info("MP4 segment recorder initialized");
}

/**
 * Record an RTSP stream to an MP4 file for a specified duration
 *
 * This function handles the actual recording of an RTSP stream to an MP4 file.
 * It maintains a single RTSP connection across multiple recording segments,
 * ensuring there are no gaps between segments.
 *
 * IMPORTANT: This function always ensures that recordings start on a keyframe.
 * It will wait for a keyframe before starting to record, regardless of whether
 * the previous segment ended with a keyframe or not. This ensures proper playback
 * of all recorded segments.
 *
 * BUGFIX: This function now accepts per-stream input context and segment info
 * to prevent stream mixing when multiple streams are recording simultaneously.
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
 * @param input_ctx_ptr Pointer to the input context for this stream (reused between segments)
 * @param segment_info_ptr Pointer to the segment info for this stream
 * @return 0 on success, negative value on error
 */
int record_segment(const char *rtsp_url, const char *output_file, int duration, int has_audio,
                   AVFormatContext **input_ctx_ptr, segment_info_t *segment_info_ptr,
                   record_segment_started_cb started_cb, void *cb_ctx) {
    int ret = 0;
    AVFormatContext *input_ctx = NULL;
    AVFormatContext *output_ctx = NULL;
    AVDictionary *opts = NULL;
    AVDictionary *out_opts = NULL;
    AVPacket *pkt = NULL;  // CRITICAL FIX: Initialize to NULL to prevent using uninitialized value
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
    int64_t start_time = 0;  // CRITICAL FIX: Initialize to 0 to prevent using uninitialized value
    time_t last_progress = 0;
    int segment_index = 0;
    // Invoke-once guard for started callback
    bool started_cb_called = false;


    // CRITICAL FIX: Initialize static variable for tracking waiting time for keyframes
    // This variable is used to track how long we've been waiting for a keyframe
    // It must be properly initialized to prevent using uninitialized values
    static int64_t waiting_start_time = 0;
    waiting_start_time = 0;  // Reset for each new segment to prevent using stale values

    // BUGFIX: Validate input parameters
    if (!input_ctx_ptr || !segment_info_ptr) {
        log_error("Invalid parameters: input_ctx_ptr or segment_info_ptr is NULL");
        return -1;
    }

    // BUGFIX: Use per-stream segment info instead of global static variable
    segment_index = segment_info_ptr->segment_index + 1;

    log_info("Starting new segment with index %d", segment_index);

    log_info("Recording from %s", rtsp_url);
    log_info("Output file: %s", output_file);
    log_info("Duration: %d seconds", duration);

    // BUGFIX: Use per-stream input context instead of global static variable
    if (*input_ctx_ptr) {
        input_ctx = *input_ctx_ptr;
        // Clear the pointer to prevent double free
        *input_ctx_ptr = NULL;
        log_debug("Using existing input context");
    } else {

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
    // CRITICAL FIX: Check if input_ctx is NULL before accessing its members
    if (!input_ctx) {
        log_error("Input context is NULL, cannot proceed with recording");
        ret = -1;
        goto cleanup;
    }

    log_debug("Input format: %s", input_ctx->iformat->name);
    log_debug("Number of streams: %d", input_ctx->nb_streams);

    // Find video and audio streams
    for (unsigned int i = 0; i < input_ctx->nb_streams; i++) {
        AVStream *stream = input_ctx->streams[i];
        if (stream->codecpar->codec_type == AVMEDIA_TYPE_VIDEO && video_stream_idx < 0) {
            video_stream_idx = i;
            log_debug("Found video stream: %d", i);
            log_debug("  Codec: %s", avcodec_get_name(stream->codecpar->codec_id));

            // CRITICAL FIX: Check for unspecified dimensions and log a warning
            if (stream->codecpar->width == 0 || stream->codecpar->height == 0) {
                log_warn("Video stream has unspecified dimensions (width=%d, height=%d)",
                        stream->codecpar->width, stream->codecpar->height);

                // Try to extract dimensions from extradata if available
                if (stream->codecpar->extradata_size > 0 && stream->codecpar->codec_id == AV_CODEC_ID_H264) {
                    log_info("Attempting to extract dimensions from H.264 extradata");
                    // This would require SPS parsing which is complex - we'll use defaults instead
                }
            } else {
                log_debug("  Resolution: %dx%d", stream->codecpar->width, stream->codecpar->height);
            }

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

    // CRITICAL FIX: Check for unspecified video dimensions (0x0) and set default values
    // This prevents the "dimensions not set" error and segmentation fault
    if (out_video_stream->codecpar->width == 0 || out_video_stream->codecpar->height == 0) {
        log_warn("Video dimensions not set (width=%d, height=%d), using default values",
                out_video_stream->codecpar->width, out_video_stream->codecpar->height);

        // Set default dimensions (640x480 is a safe choice)
        out_video_stream->codecpar->width = 640;
        out_video_stream->codecpar->height = 480;

        log_info("Set default video dimensions to %dx%d",
                out_video_stream->codecpar->width, out_video_stream->codecpar->height);
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

    // CRITICAL FIX: Validate output_file parameter before attempting to open
    if (!output_file || output_file[0] == '\0') {
        log_error("Invalid output file path (NULL or empty)");
        ret = AVERROR(EINVAL);
        goto cleanup;
    }

    // CRITICAL FIX: Validate output_ctx before attempting to open file
    if (!output_ctx) {
        log_error("Output context is NULL, cannot open output file");
        ret = AVERROR(EINVAL);
        goto cleanup;
    }

    // Log the output file path for debugging
    log_debug("Attempting to open output file: %s", output_file);

    // CRITICAL FIX: Check if the output file already exists and remove it
    // This prevents issues with trying to open a file that's already being written to
    struct stat st;
    if (stat(output_file, &st) == 0) {
        log_warn("Output file already exists: %s (size: %lld bytes), removing it",
                output_file, (long long)st.st_size);
        if (unlink(output_file) != 0) {
            log_error("Failed to remove existing output file: %s (error: %s)",
                    output_file, strerror(errno));
            // Continue anyway, avio_open might overwrite it
        }
    }

    // Open output file
    ret = avio_open(&output_ctx->pb, output_file, AVIO_FLAG_WRITE);
    if (ret < 0) {
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Failed to open output file: %d (%s)", ret, error_buf);
        log_error("Output file path: %s", output_file);

        // Additional diagnostics
        char *dir_path = strdup(output_file);
        if (dir_path) {
            char *last_slash = strrchr(dir_path, '/');
            if (last_slash) {
                *last_slash = '\0';
                struct stat dir_st;
                if (stat(dir_path, &dir_st) != 0) {
                    log_error("Directory does not exist: %s", dir_path);
                } else if (!S_ISDIR(dir_st.st_mode)) {
                    log_error("Path exists but is not a directory: %s", dir_path);
                } else if (access(dir_path, W_OK) != 0) {
                    log_error("Directory is not writable: %s", dir_path);
                }
            }
            free(dir_path);
        }

        goto cleanup;
    }

    log_debug("Successfully opened output file: %s", output_file);

    // CRITICAL FIX: Double-check video dimensions before writing header
    if (out_video_stream->codecpar->width == 0 || out_video_stream->codecpar->height == 0) {
        log_error("Video dimensions still not set after fix attempt, setting emergency defaults");
        out_video_stream->codecpar->width = 640;
        out_video_stream->codecpar->height = 480;
    }

    // Write file header
    ret = avformat_write_header(output_ctx, &out_opts);
    if (ret < 0) {
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Failed to write header: %d (%s)", ret, error_buf);

        // If this is an EINVAL error, it might be related to dimensions or incompatible audio codec
        if (ret == AVERROR(EINVAL)) {
            log_error("Header write failed with EINVAL, likely due to invalid video parameters");
            log_error("Video stream parameters: width=%d, height=%d, codec_id=%d",
                     out_video_stream->codecpar->width,
                     out_video_stream->codecpar->height,
                     out_video_stream->codecpar->codec_id);

            // Check if we have an audio stream and log its parameters
            if (out_audio_stream) {
                log_error("Audio stream parameters: codec_id=%d, sample_rate=%d, channels=%d",
                         out_audio_stream->codecpar->codec_id,
                         out_audio_stream->codecpar->sample_rate,
                         #if LIBAVCODEC_VERSION_INT >= AV_VERSION_INT(59, 37, 100)
                             out_audio_stream->codecpar->ch_layout.nb_channels);
                         #else
                             out_audio_stream->codecpar->channels);
                         #endif

                // Check for known incompatible audio codecs
                if (out_audio_stream->codecpar->codec_id == AV_CODEC_ID_PCM_MULAW) {
                    log_error("PCM μ-law (G.711 μ-law) audio codec is not compatible with MP4 format");
                    log_error("Audio transcoding to AAC should be enabled automatically");
                    log_error("If the issue persists, try disabling audio recording for this stream");
                } else if (out_audio_stream->codecpar->codec_id == AV_CODEC_ID_PCM_ALAW) {
                    log_error("PCM A-law (G.711 A-law) audio codec is not compatible with MP4 format");
                    log_error("Audio transcoding to AAC should be enabled automatically");
                    log_error("If the issue persists, try disabling audio recording for this stream");
                } else if (out_audio_stream->codecpar->codec_id == AV_CODEC_ID_PCM_S16LE) {
                    log_error("PCM signed 16-bit little-endian audio codec is not compatible with MP4 format");
                    log_error("Audio transcoding to AAC should be enabled automatically");
                    log_error("If the issue persists, try disabling audio recording for this stream");
                } else if (out_audio_stream->codecpar->codec_id == AV_CODEC_ID_PCM_S16BE) {
                    log_error("PCM signed 16-bit big-endian audio codec is not compatible with MP4 format");
                    log_error("Audio transcoding to AAC should be enabled automatically");
                    log_error("If the issue persists, try disabling audio recording for this stream");
                } else if (out_audio_stream->codecpar->codec_id >= AV_CODEC_ID_PCM_S16LE &&
                          out_audio_stream->codecpar->codec_id <= AV_CODEC_ID_PCM_LXF) {
                    log_error("PCM audio codec (codec_id=%d) is not compatible with MP4 format",
                             out_audio_stream->codecpar->codec_id);
                    log_error("Audio transcoding to AAC should be enabled automatically");
                    log_error("If the issue persists, try disabling audio recording for this stream");
                }
            }
        }

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

    // CRITICAL FIX: Ensure input_ctx is valid before entering the main loop
    if (!input_ctx) {
        log_error("Input context is NULL before main recording loop, cannot proceed");
        ret = -1;
        goto cleanup;
    }

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
                // BUGFIX: Always wait for a keyframe to start recording, regardless of previous segment state
                if (is_keyframe) {
                    log_info("Found first key frame, starting recording");
                    found_first_keyframe = true;

                        // Notify caller that segment has officially started (aligned to keyframe)
                        if (!started_cb_called && started_cb) {
                            started_cb(cb_ctx);
                            started_cb_called = true;
                        }


                    // Reset start time to when we found the first key frame
                    start_time = av_gettime();

                    // Note if we had a keyframe at the end of the previous segment
                    if (segment_info_ptr->last_frame_was_key && segment_index > 0) {
                        log_info("Previous segment ended with a key frame, and we're starting with a new keyframe");
                    }
                } else {
                    // Always wait for a key frame
                    // Skip this frame as we're waiting for a key frame
                    av_packet_unref(pkt);
                    continue;
                }
            }

            // If we're waiting for the final key frame to end recording
            if (waiting_for_final_keyframe) {
                // Check if this is a key frame or if we've been waiting too long

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
                        segment_info_ptr->last_frame_was_key = true;
                        log_debug("Last frame was a key frame, next segment will start immediately with this keyframe");
                    } else {
                        log_info("Waited %lld seconds for key frame, ending recording with non-key frame", (long long)wait_time);
                        // Clear flag since the last frame was not a key frame
                        segment_info_ptr->last_frame_was_key = false;
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

    // BUGFIX: Update per-stream segment info for the next segment
    segment_info_ptr->segment_index = segment_index;
    segment_info_ptr->has_audio = has_audio && audio_stream_idx >= 0;

    log_info("Saved segment info for next segment: index=%d, has_audio=%d, last_frame_was_key=%d",
            segment_index, has_audio && audio_stream_idx >= 0, segment_info_ptr->last_frame_was_key);

cleanup:
    // CRITICAL FIX: Aggressive cleanup to prevent memory growth over time
    log_debug("Starting aggressive cleanup of FFmpeg resources");

    // Free dictionaries - these are always safe to free
    av_dict_free(&opts);
    av_dict_free(&out_opts);

    // Free packet if allocated
    if (pkt) {
        log_debug("Freeing packet during cleanup");
        av_packet_unref(pkt);
        av_packet_free(&pkt);
        pkt = NULL;
    }

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

    // BUGFIX: Store the input context in the per-stream variable for reuse if recording was successful
    if (ret >= 0) {
        // Store the input context for reuse in the next segment
        // We can't directly access internal FFmpeg structures
        // Just store the context as is and rely on FFmpeg's internal reference counting
        *input_ctx_ptr = input_ctx;
        // Don't close the input context as we're keeping it for the next segment
        input_ctx = NULL;
        log_debug("Stored input context for reuse in next segment");
    } else
    {
        // If there was an error, close the input context
        log_debug("Closing input context due to error");

        // CRITICAL FIX: Check if input_ctx is NULL before trying to access it
        // This prevents segmentation fault when RTSP connection fails
        if (input_ctx) {
            // CRITICAL FIX: Use a safer approach to clean up FFmpeg resources
            // First check if the context is valid and has streams
            if (input_ctx->nb_streams > 0) {
                // Clean up all streams before closing
                for (unsigned int i = 0; i < input_ctx->nb_streams; i++) {
                    if (input_ctx->streams && input_ctx->streams[i]) {
                        // Free any codec parameters
                        if (input_ctx->streams[i]->codecpar) {
                            avcodec_parameters_free(&input_ctx->streams[i]->codecpar);
                        }
                    }
                }
            }

            // Flush any pending data
            if (input_ctx->pb) {
                avio_flush(input_ctx->pb);
            }

            // Close the input context
            avformat_close_input(&input_ctx);
            input_ctx = NULL;  // Ensure the pointer is NULL after closing
        } else {
            log_debug("Input context is NULL, nothing to clean up");
        }
        log_debug("Closed input context due to error");
    }

    // Final FFmpeg cleanup to prevent memory leaks
    avformat_network_deinit();
    av_dict_free(&opts);

    // Return the error code
    return ret;
}

/**
 * Clean up all static resources used by the MP4 segment recorder
 * This function should be called during program shutdown to prevent memory leaks
 *
 * BUGFIX: No longer needs to clean up global static variables since they were removed.
 * Input contexts are now per-stream and cleaned up by the thread context.
 */
void mp4_segment_recorder_cleanup(void) {
    // Call FFmpeg's global cleanup functions to release any global resources
    // This helps clean up resources that might not be freed otherwise

    // Set log level to quiet to suppress any warnings during cleanup
    av_log_set_level(AV_LOG_QUIET);

    // Clean up network resources
    // Note: This is safe to call during shutdown as we're ensuring all contexts are closed first
    avformat_network_deinit();

    log_info("MP4 segment recorder resources cleaned up");
}

/**
 * Write a packet to the MP4 file
 * This function handles both video and audio packets
 *
 * @param writer The MP4 writer instance
 * @param pkt The packet to write
 * @param input_stream The original input stream (for codec parameters)
 * @return 0 on success, negative on error
 */
int mp4_segment_recorder_write_packet(mp4_writer_t *writer, const AVPacket *pkt, const AVStream *input_stream) {
    if (!writer || !pkt || !input_stream) {
        log_error("Invalid parameters passed to mp4_segment_recorder_write_packet");
        return -1;
    }

    if (!writer->output_ctx) {
        log_error("Writer output context is NULL for stream %s",
                writer->stream_name ? writer->stream_name : "unknown");
        return -1;
    }

    // Create a copy of the packet to avoid modifying the original
    AVPacket *out_pkt = av_packet_alloc();
    if (!out_pkt) {
        log_error("Failed to allocate packet for stream %s",
                writer->stream_name ? writer->stream_name : "unknown");
        return -1;
    }

    // Make a reference copy of the packet
    int ret = av_packet_ref(out_pkt, pkt);
    if (ret < 0) {
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Failed to copy packet for stream %s: %s",
                writer->stream_name ? writer->stream_name : "unknown", error_buf);
        av_packet_free(&out_pkt);
        return ret;
    }

    // Determine the output stream index based on the packet type
    if (input_stream->codecpar->codec_type == AVMEDIA_TYPE_VIDEO) {
        // Set the stream index to the video stream
        out_pkt->stream_index = writer->video_stream_idx;

        // Ensure PTS >= DTS for video packets
        if (out_pkt->pts != AV_NOPTS_VALUE && out_pkt->dts != AV_NOPTS_VALUE && out_pkt->pts < out_pkt->dts) {
            out_pkt->pts = out_pkt->dts;
        }
    } else if (input_stream->codecpar->codec_type == AVMEDIA_TYPE_AUDIO) {
        // Set the stream index to the audio stream
        if (writer->audio.stream_idx >= 0) {
            out_pkt->stream_index = writer->audio.stream_idx;
        } else {
            // No audio stream in the output, drop the packet
            av_packet_free(&out_pkt);
            return 0;
        }
    } else {
        // Unknown stream type, drop the packet
        av_packet_free(&out_pkt);
        return 0;
    }

    // Write the packet to the output
    ret = av_interleaved_write_frame(writer->output_ctx, out_pkt);
    if (ret < 0) {
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Error writing frame for stream %s: %s",
                writer->stream_name ? writer->stream_name : "unknown", error_buf);
    }

    // Free the packet
    av_packet_free(&out_pkt);

    return ret;
}
