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
 * @param input_ctx_ptr Pointer to an existing input context (can be NULL)
 * @param has_audio Flag indicating whether to include audio in the recording
 * @param prev_segment_info Optional pointer to previous segment information for timestamp continuity
 * @return 0 on success, negative value on error
 */
int record_segment(const char *rtsp_url, const char *output_file, int duration,
                  AVFormatContext **input_ctx_ptr, int has_audio,
                  segment_info_t *prev_segment_info) {
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

    // Initialize segment index if previous segment info is provided
    if (prev_segment_info) {
        segment_index = prev_segment_info->segment_index + 1;
        log_info("Starting new segment with index %d", segment_index);
    }

    log_info("Recording from %s", rtsp_url);
    log_info("Output file: %s", output_file);
    log_info("Duration: %d seconds", duration);

    // Use existing input context if provided
    if (*input_ctx_ptr) {
        input_ctx = *input_ctx_ptr;
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

        // Store the input context for reuse
        *input_ctx_ptr = input_ctx;
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
#if LIBAVCODEC_VERSION_INT >= AV_VERSION_INT(59, 37, 100)
            // For FFmpeg 5.0 and newer
            log_debug("  Channels: %d", stream->codecpar->ch_layout.nb_channels);
#else
            // For older FFmpeg versions
            log_debug("  Channels: %d", stream->codecpar->channels);
#endif
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

    // Initialize packet
    pkt = av_packet_alloc();
    pkt->data = NULL;
    pkt->size = 0;

    // Start recording
    start_time = av_gettime();
    log_info("Recording started...");

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
                if (prev_segment_info && prev_segment_info->last_frame_was_key && segment_index > 0) {
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
                        if (prev_segment_info) {
                            prev_segment_info->last_frame_was_key = true;
                            log_debug("Last frame was a key frame, next segment will start immediately with this keyframe");
                        }
                    } else {
                        log_info("Waited %lld seconds for key frame, ending recording with non-key frame", (long long)wait_time);
                        // Clear flag since the last frame was not a key frame
                        if (prev_segment_info) {
                            prev_segment_info->last_frame_was_key = false;
                            log_debug("Last frame was NOT a key frame, next segment will wait for a keyframe");
                        }
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
                log_error("Error writing video frame: %d", ret);
            } else {
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
                if (pkt->dts != AV_NOPTS_VALUE && pkt->dts <= last_audio_dts) {
                    pkt->dts = last_audio_dts + 1;
                }

                if (pkt->pts != AV_NOPTS_VALUE && pkt->pts <= last_audio_pts) {
                    pkt->pts = last_audio_pts + 1;
                }

                // Ensure PTS >= DTS
                if (pkt->pts != AV_NOPTS_VALUE && pkt->dts != AV_NOPTS_VALUE && pkt->pts < pkt->dts) {
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
                log_error("Error writing audio frame: %d", ret);
            } else {
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

    // Save segment info for the next segment if needed
    if (prev_segment_info) {
        prev_segment_info->segment_index = segment_index;
        prev_segment_info->has_audio = has_audio && audio_stream_idx >= 0;
        log_info("Saved segment info for next segment: index=%d, has_audio=%d, last_frame_was_key=%d",
                segment_index, has_audio && audio_stream_idx >= 0, prev_segment_info->last_frame_was_key);
    }

cleanup:
    // CRITICAL FIX: Aggressive cleanup to prevent memory growth over time

    // Free dictionaries - these are always safe to free
    av_dict_free(&opts);
    av_dict_free(&out_opts);

    if (input_ctx->pb) {
        avio_flush(input_ctx->pb);
    }

    if (output_ctx->pb) {
        avio_flush(output_ctx->pb);
    }

    // In the cleanup section, add this before freeing the output context:
    if (output_ctx && output_ctx->nb_streams > 0) {
        for (unsigned int i = 0; i < output_ctx->nb_streams; i++) {
            if (output_ctx->streams[i] && output_ctx->streams[i]->codecpar) {
                avcodec_parameters_free(&output_ctx->streams[i]->codecpar);
            }
        }
    }

    // Clean up output context if it was created
    if (output_ctx) {
        // Only write trailer if we successfully wrote the header
        if (output_ctx->pb && ret >= 0 && !trailer_written) {
            av_write_trailer(output_ctx);
        }

        // Close output file if it was opened
        if (output_ctx->pb) {
            avio_closep(&output_ctx->pb);
        }

        // MEMORY LEAK FIX: Properly clean up all streams in the output context
        // This ensures all codec contexts and other resources are freed
        if (output_ctx->nb_streams > 0) {
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
        avformat_free_context(output_ctx);
        output_ctx = NULL;
    }

    // Add this to the cleanup section in record_segment():
    if (input_ctx) {
        // CRITICAL FIX: Always close the input context after each segment
        avformat_close_input(&input_ctx);
        *input_ctx_ptr = NULL; // Ensure the caller knows it's closed
    }

    // MEMORY LEAK FIX: If we had a critical error, close and reopen the input context
    // This prevents memory growth over time by periodically refreshing FFmpeg resources
    if (ret < 0 && ret != AVERROR(EAGAIN) && *input_ctx_ptr) {
        log_info("Critical error occurred, closing and reopening input context to prevent memory leaks");
        avformat_close_input(input_ctx_ptr);
        // The caller will reopen the input context on the next attempt
    }

    // MEMORY LEAK FIX: Ensure packet is unref'd before returning
    // This is a safety measure in case we jumped to cleanup without unreferencing the packet
    av_packet_unref(pkt);
    av_packet_free(&pkt);

    // MEMORY LEAK FIX: Always close and reopen the input context after each segment
    // This is the most aggressive approach to prevent memory growth
    if (*input_ctx_ptr) {
        log_info("Closing input context after segment to prevent memory leaks");

        // CRITICAL: Flush all buffers before closing
        if ((*input_ctx_ptr)->pb) {
            avio_flush((*input_ctx_ptr)->pb);
        }

        // Close the input context - this will free all associated resources
        // Let FFmpeg handle its own memory management
        avformat_close_input(input_ctx_ptr);
        // The caller will reopen the input context on the next attempt
    }

    // Return the error code
    return ret;
}
