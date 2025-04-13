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
 * Set up RTSP connection for streaming
 * 
 * @param rtsp_url The URL of the RTSP stream to connect to
 * @param opts Pointer to AVDictionary for options (will be populated)
 * @return AVFormatContext* on success, NULL on error
 */
static AVFormatContext* setup_rtsp_connection(const char *rtsp_url, AVDictionary **opts) {
    int ret = 0;
    AVFormatContext *input_ctx = NULL;
    
    // Set up RTSP options for low latency
    av_dict_set(opts, "rtsp_transport", "tcp", 0);  // Use TCP for RTSP (more reliable than UDP)
    av_dict_set(opts, "fflags", "nobuffer", 0);     // Reduce buffering
    av_dict_set(opts, "flags", "low_delay", 0);     // Low delay mode
    av_dict_set(opts, "max_delay", "500000", 0);    // Maximum delay of 500ms
    av_dict_set(opts, "stimeout", "5000000", 0);    // Socket timeout in microseconds (5 seconds)

    // Open input
    ret = avformat_open_input(&input_ctx, rtsp_url, NULL, opts);
    if (ret < 0) {
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Failed to open input: %d (%s)", ret, error_buf);
        return NULL;
    }

    // Find stream info
    ret = avformat_find_stream_info(input_ctx, NULL);
    if (ret < 0) {
        log_error("Failed to find stream info: %d", ret);
        avformat_close_input(&input_ctx);
        return NULL;
    }
    
    return input_ctx;
}

/**
 * Find video and audio streams in the input context
 * 
 * @param input_ctx The input format context
 * @param video_stream_idx Pointer to store video stream index
 * @param audio_stream_idx Pointer to store audio stream index
 * @return 0 on success, negative value if no video stream found
 */
static int find_streams(AVFormatContext *input_ctx, int *video_stream_idx, int *audio_stream_idx) {
    if (!input_ctx) {
        log_error("Input context is NULL, cannot find streams");
        return -1;
    }

    *video_stream_idx = -1;
    *audio_stream_idx = -1;

    log_debug("Input format: %s", input_ctx->iformat->name);
    log_debug("Number of streams: %d", input_ctx->nb_streams);

    // Find video and audio streams
    for (unsigned int i = 0; i < input_ctx->nb_streams; i++) {
        AVStream *stream = input_ctx->streams[i];
        if (stream->codecpar->codec_type == AVMEDIA_TYPE_VIDEO && *video_stream_idx < 0) {
            *video_stream_idx = i;
            log_debug("Found video stream: %d", i);
            log_debug("  Codec: %s", avcodec_get_name(stream->codecpar->codec_id));

            // Check for unspecified dimensions and log a warning
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
        } else if (stream->codecpar->codec_type == AVMEDIA_TYPE_AUDIO && *audio_stream_idx < 0) {
            *audio_stream_idx = i;
            log_debug("Found audio stream: %d", i);
            log_debug("  Codec: %s", avcodec_get_name(stream->codecpar->codec_id));
            log_debug("  Sample rate: %d Hz", stream->codecpar->sample_rate);
            // Handle channel count for different FFmpeg versions
        }
    }

    if (*video_stream_idx < 0) {
        log_error("No video stream found");
        return -1;
    }
    
    return 0;
}

/**
 * Set up the output MP4 context
 * 
 * @param output_file The path to the output MP4 file
 * @param input_ctx The input format context
 * @param video_stream_idx The video stream index
 * @param audio_stream_idx The audio stream index
 * @param has_audio Flag indicating whether to include audio
 * @param out_video_stream Pointer to store output video stream
 * @param out_audio_stream Pointer to store output audio stream
 * @param out_opts Pointer to AVDictionary for options
 * @return AVFormatContext* on success, NULL on error
 */
static AVFormatContext* setup_output_context(const char *output_file, 
                                            AVFormatContext *input_ctx,
                                            int video_stream_idx, 
                                            int audio_stream_idx,
                                            int has_audio,
                                            AVStream **out_video_stream,
                                            AVStream **out_audio_stream,
                                            AVDictionary **out_opts) {
    int ret = 0;
    AVFormatContext *output_ctx = NULL;
    
    // Create output context
    ret = avformat_alloc_output_context2(&output_ctx, NULL, "mp4", output_file);
    if (ret < 0 || !output_ctx) {
        log_error("Failed to create output context: %d", ret);
        return NULL;
    }

    // Add video stream
    *out_video_stream = avformat_new_stream(output_ctx, NULL);
    if (!*out_video_stream) {
        log_error("Failed to create output video stream");
        avformat_free_context(output_ctx);
        return NULL;
    }

    // Copy video codec parameters
    ret = avcodec_parameters_copy((*out_video_stream)->codecpar,
                                 input_ctx->streams[video_stream_idx]->codecpar);
    if (ret < 0) {
        log_error("Failed to copy video codec parameters: %d", ret);
        avformat_free_context(output_ctx);
        return NULL;
    }

    // Check for unspecified video dimensions (0x0) and set default values
    if ((*out_video_stream)->codecpar->width == 0 || (*out_video_stream)->codecpar->height == 0) {
        log_warn("Video dimensions not set (width=%d, height=%d), using default values",
                (*out_video_stream)->codecpar->width, (*out_video_stream)->codecpar->height);

        // Set default dimensions (640x480 is a safe choice)
        (*out_video_stream)->codecpar->width = 640;
        (*out_video_stream)->codecpar->height = 480;

        log_info("Set default video dimensions to %dx%d",
                (*out_video_stream)->codecpar->width, (*out_video_stream)->codecpar->height);
    }

    // Set video stream time base
    (*out_video_stream)->time_base = input_ctx->streams[video_stream_idx]->time_base;

    // Add audio stream if available and audio is enabled
    if (audio_stream_idx >= 0 && has_audio) {
        log_info("Including audio stream in MP4 recording");
        *out_audio_stream = avformat_new_stream(output_ctx, NULL);
        if (!*out_audio_stream) {
            log_error("Failed to create output audio stream");
            avformat_free_context(output_ctx);
            return NULL;
        }

        // Copy audio codec parameters
        ret = avcodec_parameters_copy((*out_audio_stream)->codecpar,
                                     input_ctx->streams[audio_stream_idx]->codecpar);
        if (ret < 0) {
            log_error("Failed to copy audio codec parameters: %d", ret);
            avformat_free_context(output_ctx);
            return NULL;
        }

        // Set audio stream time base
        (*out_audio_stream)->time_base = input_ctx->streams[audio_stream_idx]->time_base;
    }

    // Disable faststart to prevent segmentation faults
    // The faststart option causes a second pass that moves the moov atom to the beginning of the file
    // This second pass is causing segmentation faults during shutdown
    av_dict_set(out_opts, "movflags", "empty_moov", 0);

    // Open output file
    ret = avio_open(&output_ctx->pb, output_file, AVIO_FLAG_WRITE);
    if (ret < 0) {
        log_error("Failed to open output file: %d", ret);
        avformat_free_context(output_ctx);
        return NULL;
    }

    // Double-check video dimensions before writing header
    if ((*out_video_stream)->codecpar->width == 0 || (*out_video_stream)->codecpar->height == 0) {
        log_error("Video dimensions still not set after fix attempt, setting emergency defaults");
        (*out_video_stream)->codecpar->width = 640;
        (*out_video_stream)->codecpar->height = 480;
    }

    // Write file header
    ret = avformat_write_header(output_ctx, out_opts);
    if (ret < 0) {
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Failed to write header: %d (%s)", ret, error_buf);

        // If this is an EINVAL error, it might be related to dimensions
        if (ret == AVERROR(EINVAL)) {
            log_error("Header write failed with EINVAL, likely due to invalid video parameters");
            log_error("Video stream parameters: width=%d, height=%d, codec_id=%d",
                     (*out_video_stream)->codecpar->width,
                     (*out_video_stream)->codecpar->height,
                     (*out_video_stream)->codecpar->codec_id);
        }

        avio_closep(&output_ctx->pb);
        avformat_free_context(output_ctx);
        return NULL;
    }

    return output_ctx;
}

/**
 * Process a video packet for MP4 recording
 * 
 * @param pkt The packet to process
 * @param input_ctx The input format context
 * @param output_ctx The output format context
 * @param video_stream_idx The video stream index
 * @param out_video_stream The output video stream
 * @param first_video_dts Pointer to first video DTS value
 * @param first_video_pts Pointer to first video PTS value
 * @param last_video_dts Pointer to last video DTS value
 * @param last_video_pts Pointer to last video PTS value
 * @param segment_index The current segment index
 * @param consecutive_timestamp_errors Pointer to consecutive timestamp errors counter
 * @param max_timestamp_errors Maximum allowed consecutive timestamp errors
 * @return 0 on success, negative value on error
 */
static int process_video_packet(AVPacket *pkt, 
                               AVFormatContext *input_ctx,
                               AVFormatContext *output_ctx,
                               int video_stream_idx,
                               AVStream *out_video_stream,
                               int64_t *first_video_dts,
                               int64_t *first_video_pts,
                               int64_t *last_video_dts,
                               int64_t *last_video_pts,
                               int segment_index,
                               int *consecutive_timestamp_errors,
                               int max_timestamp_errors) {
    int ret = 0;
    
    // Initialize first DTS if not set
    if (*first_video_dts == AV_NOPTS_VALUE && pkt->dts != AV_NOPTS_VALUE) {
        *first_video_dts = pkt->dts;
        *first_video_pts = pkt->pts != AV_NOPTS_VALUE ? pkt->pts : pkt->dts;
        log_debug("First video DTS: %lld, PTS: %lld",
                (long long)*first_video_dts, (long long)*first_video_pts);
    }

    // Handle timestamps based on segment index
    if (segment_index == 0) {
        // First segment - adjust timestamps relative to first_dts
        if (pkt->dts != AV_NOPTS_VALUE && *first_video_dts != AV_NOPTS_VALUE) {
            pkt->dts -= *first_video_dts;
            if (pkt->dts < 0) pkt->dts = 0;
        }

        if (pkt->pts != AV_NOPTS_VALUE && *first_video_pts != AV_NOPTS_VALUE) {
            pkt->pts -= *first_video_pts;
            if (pkt->pts < 0) pkt->pts = 0;
        }
    } else {
        // Subsequent segments - maintain timestamp continuity
        // Use a small fixed offset instead of carrying over potentially large timestamps
        // This prevents the timestamp inflation issue while still maintaining continuity
        if (pkt->dts != AV_NOPTS_VALUE && *first_video_dts != AV_NOPTS_VALUE) {
            // Calculate relative timestamp within this segment
            int64_t relative_dts = pkt->dts - *first_video_dts;
            // Add a small fixed offset (e.g., 1/30th of a second in timebase units)
            // This ensures continuity without timestamp inflation
            pkt->dts = relative_dts + 1;
        }

        if (pkt->pts != AV_NOPTS_VALUE && *first_video_pts != AV_NOPTS_VALUE) {
            int64_t relative_pts = pkt->pts - *first_video_pts;
            pkt->pts = relative_pts + 1;
        }
    }

    // Ensure PTS >= DTS for video packets to prevent "pts < dts" errors
    // This is essential for MP4 format compliance and prevents ghosting artifacts
    if (pkt->pts != AV_NOPTS_VALUE && pkt->dts != AV_NOPTS_VALUE && pkt->pts < pkt->dts) {
        log_debug("Fixing video packet with PTS < DTS: PTS=%lld, DTS=%lld",
                 (long long)pkt->pts, (long long)pkt->dts);
        pkt->pts = pkt->dts;
    }

    // Ensure monotonically increasing DTS values
    // This prevents the "Application provided invalid, non monotonically increasing dts" error
    if (pkt->dts != AV_NOPTS_VALUE && *last_video_dts != 0 && pkt->dts <= *last_video_dts) {
        int64_t fixed_dts = *last_video_dts + 1;
        log_debug("Fixing non-monotonic DTS: old=%lld, last=%lld, new=%lld",
                 (long long)pkt->dts, (long long)*last_video_dts, (long long)fixed_dts);

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
        *last_video_dts = pkt->dts;
    }
    if (pkt->pts != AV_NOPTS_VALUE) {
        *last_video_pts = pkt->pts;
    }

    // Ensure DTS values don't exceed MP4 format limits (0x7fffffff)
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

    // Ensure packet duration is within reasonable limits
    // This prevents the "Packet duration is out of range" error
    if (pkt->duration > 10000000) {
        log_warn("Packet duration too large: %lld, capping at reasonable value", (long long)pkt->duration);
        // Cap at a reasonable value (e.g., 1 second in timebase units)
        pkt->duration = 90000;
    }

    // Set output stream index
    pkt->stream_index = out_video_stream->index;

    // Write packet
    ret = av_interleaved_write_frame(output_ctx, pkt);
    if (ret < 0) {
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Error writing video frame: %d (%s)", ret, error_buf);

        // Handle timestamp-related errors
        if (ret == AVERROR(EINVAL) && strstr(error_buf, "monoton")) {
            // This is likely a timestamp error, try to fix it for the next packet
            log_warn("Detected timestamp error, will try to fix for next packet");

            // Increment the consecutive error counter
            (*consecutive_timestamp_errors)++;

            if (*consecutive_timestamp_errors >= max_timestamp_errors) {
                // Too many consecutive errors, reset all timestamps
                log_warn("Too many consecutive timestamp errors (%d), resetting all timestamps",
                        *consecutive_timestamp_errors);

                // Reset the error counter
                *consecutive_timestamp_errors = 0;
                
                // Return a special error code to indicate timestamp reset is needed
                return -2;
            } else {
                // Force a larger increment for the next packet to avoid timestamp issues
                *last_video_dts += 100 * (*consecutive_timestamp_errors);
                *last_video_pts += 100 * (*consecutive_timestamp_errors);
            }
        }
    } else {
        // Reset consecutive error counter on success
        *consecutive_timestamp_errors = 0;
    }

    return ret;
}

/**
 * Process an audio packet for MP4 recording
 * 
 * @param pkt The packet to process
 * @param input_ctx The input format context
 * @param output_ctx The output format context
 * @param audio_stream_idx The audio stream index
 * @param out_audio_stream The output audio stream
 * @param first_audio_dts Pointer to first audio DTS value
 * @param first_audio_pts Pointer to first audio PTS value
 * @param last_audio_dts Pointer to last audio DTS value
 * @param last_audio_pts Pointer to last audio PTS value
 * @param segment_index The current segment index
 * @param audio_packet_count The current audio packet count
 * @param consecutive_timestamp_errors Pointer to consecutive timestamp errors counter
 * @param max_timestamp_errors Maximum allowed consecutive timestamp errors
 * @return 0 on success, negative value on error
 */
static int process_audio_packet(AVPacket *pkt, 
                               AVFormatContext *input_ctx,
                               AVFormatContext *output_ctx,
                               int audio_stream_idx,
                               AVStream *out_audio_stream,
                               int64_t *first_audio_dts,
                               int64_t *first_audio_pts,
                               int64_t *last_audio_dts,
                               int64_t *last_audio_pts,
                               int segment_index,
                               int audio_packet_count,
                               int *consecutive_timestamp_errors,
                               int max_timestamp_errors) {
    int ret = 0;
    
    // Initialize first audio DTS if not set
    if (*first_audio_dts == AV_NOPTS_VALUE && pkt->dts != AV_NOPTS_VALUE) {
        *first_audio_dts = pkt->dts;
        *first_audio_pts = pkt->pts != AV_NOPTS_VALUE ? pkt->pts : pkt->dts;
        log_debug("First audio DTS: %lld, PTS: %lld",
                (long long)*first_audio_dts, (long long)*first_audio_pts);
    }

    // Handle timestamps based on segment index
    if (segment_index == 0) {
        // First segment - adjust timestamps relative to first_dts
        if (pkt->dts != AV_NOPTS_VALUE && *first_audio_dts != AV_NOPTS_VALUE) {
            pkt->dts -= *first_audio_dts;
            if (pkt->dts < 0) pkt->dts = 0;
        }

        if (pkt->pts != AV_NOPTS_VALUE && *first_audio_pts != AV_NOPTS_VALUE) {
            pkt->pts -= *first_audio_pts;
            if (pkt->pts < 0) pkt->pts = 0;
        }
    } else {
        // Subsequent segments - maintain timestamp continuity
        // Use a small fixed offset instead of carrying over potentially large timestamps
        // This prevents the timestamp inflation issue while still maintaining continuity
        if (pkt->dts != AV_NOPTS_VALUE && *first_audio_dts != AV_NOPTS_VALUE) {
            // Calculate relative timestamp within this segment
            int64_t relative_dts = pkt->dts - *first_audio_dts;
            // Add a small fixed offset (e.g., 1/30th of a second in timebase units)
            // This ensures continuity without timestamp inflation
            pkt->dts = relative_dts + 1;
        }

        if (pkt->pts != AV_NOPTS_VALUE && *first_audio_pts != AV_NOPTS_VALUE) {
            int64_t relative_pts = pkt->pts - *first_audio_pts;
            pkt->pts = relative_pts + 1;
        }
    }

    // Ensure monotonic increase of timestamps
    if (audio_packet_count > 0) {
        // More robust handling of non-monotonic DTS values
        if (pkt->dts != AV_NOPTS_VALUE && pkt->dts <= *last_audio_dts) {
            int64_t fixed_dts = *last_audio_dts + 1;
            log_debug("Fixing non-monotonic audio DTS: old=%lld, last=%lld, new=%lld",
                     (long long)pkt->dts, (long long)*last_audio_dts, (long long)fixed_dts);
            pkt->dts = fixed_dts;
        }

        if (pkt->pts != AV_NOPTS_VALUE && pkt->pts <= *last_audio_pts) {
            int64_t fixed_pts = *last_audio_pts + 1;
            log_debug("Fixing non-monotonic audio PTS: old=%lld, last=%lld, new=%lld",
                     (long long)pkt->pts, (long long)*last_audio_pts, (long long)fixed_pts);
            pkt->pts = fixed_pts;
        }

        // Ensure PTS >= DTS
        if (pkt->pts != AV_NOPTS_VALUE && pkt->dts != AV_NOPTS_VALUE && pkt->pts < pkt->dts) {
            log_debug("Fixing audio packet with PTS < DTS: PTS=%lld, DTS=%lld",
                     (long long)pkt->pts, (long long)pkt->dts);
            pkt->pts = pkt->dts;
        }
    }

    // Ensure DTS values don't exceed MP4 format limits (0x7fffffff) for audio packets
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

    // Set output stream index
    pkt->stream_index = out_audio_stream->index;

    // Write packet
    ret = av_interleaved_write_frame(output_ctx, pkt);
    if (ret < 0) {
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Error writing audio frame: %d (%s)", ret, error_buf);

        // Handle timestamp-related errors
        if (ret == AVERROR(EINVAL) && strstr(error_buf, "monoton")) {
            // This is likely a timestamp error, try to fix it for the next packet
            log_warn("Detected audio timestamp error, will try to fix for next packet");

            // Increment the consecutive error counter
            (*consecutive_timestamp_errors)++;

            if (*consecutive_timestamp_errors >= max_timestamp_errors) {
                // Too many consecutive errors, reset all timestamps
                log_warn("Too many consecutive audio timestamp errors (%d), resetting all timestamps",
                        *consecutive_timestamp_errors);

                // Reset the error counter
                *consecutive_timestamp_errors = 0;
                
                // Return a special error code to indicate timestamp reset is needed
                return -2;
            } else {
                // Force a larger increment for the next packet to avoid timestamp issues
                *last_audio_dts += 100 * (*consecutive_timestamp_errors);
                *last_audio_pts += 100 * (*consecutive_timestamp_errors);
            }
        }
    } else {
        // Reset consecutive error counter on success
        *consecutive_timestamp_errors = 0;
    }

    // Update last timestamps
    if (pkt->dts != AV_NOPTS_VALUE) {
        *last_audio_dts = pkt->dts;
    }
    if (pkt->pts != AV_NOPTS_VALUE) {
        *last_audio_pts = pkt->pts;
    }

    return ret;
}

/**
 * Finalize the output MP4 file
 * 
 * @param output_ctx The output format context
 * @return 0 on success, negative value on error
 */
static int finalize_output(AVFormatContext *output_ctx) {
    int ret = 0;
    
    if (!output_ctx) {
        log_error("Output context is NULL, cannot finalize output");
        return -1;
    }

    // Write trailer
    ret = av_write_trailer(output_ctx);
    if (ret < 0) {
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Failed to write trailer: %d (%s)", ret, error_buf);
        return ret;
    }

    // Close output file
    if (output_ctx->pb) {
        avio_closep(&output_ctx->pb);
    }

    // Free output context
    avformat_free_context(output_ctx);
    
    return 0;
}

/**
 * Record a segment from an RTSP stream to an MP4 file
 * 
 * @param rtsp_url The URL of the RTSP stream
 * @param output_file The path to the output MP4 file
 * @param duration Maximum duration of the segment in seconds
 * @param has_audio Flag indicating whether to include audio
 * @return 0 on success, negative value on error
 */
int record_segment(const char *rtsp_url, const char *output_file, int duration, int has_audio) {
    int ret = 0;
    AVFormatContext *input_ctx = NULL;
    AVFormatContext *output_ctx = NULL;
    AVDictionary *opts = NULL;
    AVStream *out_video_stream = NULL;
    AVStream *out_audio_stream = NULL;
    AVPacket pkt;
    int video_stream_idx = -1;
    int audio_stream_idx = -1;
    int64_t start_time = 0;
    int64_t current_time = 0;
    int64_t first_video_dts = AV_NOPTS_VALUE;
    int64_t first_video_pts = AV_NOPTS_VALUE;
    int64_t last_video_dts = 0;
    int64_t last_video_pts = 0;
    int64_t first_audio_dts = AV_NOPTS_VALUE;
    int64_t first_audio_pts = AV_NOPTS_VALUE;
    int64_t last_audio_dts = 0;
    int64_t last_audio_pts = 0;
    int video_packet_count = 0;
    int audio_packet_count = 0;
    int consecutive_timestamp_errors = 0;
    const int max_timestamp_errors = 5;
    int segment_index = 0;
    int waiting_for_keyframe = 1; // Always wait for keyframe by default
    int recording_started = 0;
    
    // Check if we have a static input context from a previous recording
    pthread_mutex_lock(&static_vars_mutex);
    if (static_input_ctx) {
        log_info("Using existing input context from previous recording");
        input_ctx = static_input_ctx;
        segment_index = segment_info.segment_index;
        has_audio = segment_info.has_audio;
        waiting_for_keyframe = !segment_info.last_frame_was_key;
    }
    pthread_mutex_unlock(&static_vars_mutex);

    // If we don't have a static input context, set up a new one
    if (!input_ctx) {
        // Set up RTSP connection
        input_ctx = setup_rtsp_connection(rtsp_url, &opts);
        if (!input_ctx) {
            log_error("Failed to set up RTSP connection");
            return -1;
        }

        // Find streams
        ret = find_streams(input_ctx, &video_stream_idx, &audio_stream_idx);
        if (ret < 0) {
            log_error("Failed to find streams");
            avformat_close_input(&input_ctx);
            return ret;
        }

        // Update has_audio flag based on stream availability
        has_audio = has_audio && (audio_stream_idx >= 0);
        if (has_audio) {
            log_info("Audio stream found and enabled");
        } else if (audio_stream_idx >= 0) {
            log_info("Audio stream found but disabled by configuration");
        } else {
            log_info("No audio stream found");
        }
    } else {
        // Use the stream indices from the existing input context
        for (unsigned int i = 0; i < input_ctx->nb_streams; i++) {
            AVStream *stream = input_ctx->streams[i];
            if (stream->codecpar->codec_type == AVMEDIA_TYPE_VIDEO && video_stream_idx < 0) {
                video_stream_idx = i;
            } else if (stream->codecpar->codec_type == AVMEDIA_TYPE_AUDIO && audio_stream_idx < 0) {
                audio_stream_idx = i;
            }
        }
    }

    // Set up output context
    output_ctx = setup_output_context(output_file, input_ctx, video_stream_idx, audio_stream_idx, 
                                     has_audio, &out_video_stream, &out_audio_stream, &opts);
    if (!output_ctx) {
        log_error("Failed to set up output context");
        if (!static_input_ctx) {
            avformat_close_input(&input_ctx);
        }
        return -1;
    }

    // Free options dictionary
    av_dict_free(&opts);

    // Record start time
    start_time = av_gettime();

    // Main packet processing loop
    while (!is_shutdown_initiated()) {
        // Check if we've reached the maximum duration
        current_time = av_gettime();
        if (recording_started && duration > 0 && 
            (current_time - start_time) / 1000000 >= duration) {
            log_info("Maximum duration reached (%d seconds), stopping recording", duration);
            break;
        }

        // Read packet
        av_init_packet(&pkt);
        pkt.data = NULL;
        pkt.size = 0;

        ret = av_read_frame(input_ctx, &pkt);
        if (ret < 0) {
            if (ret == AVERROR(EAGAIN) || ret == AVERROR_EOF) {
                // End of file or temporary unavailability, wait and retry
                av_packet_unref(&pkt);
                av_usleep(10000);  // 10ms
                continue;
            } else {
                // Actual error
                char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
                av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
                log_error("Error reading frame: %d (%s)", ret, error_buf);
                break;
            }
        }

        // Check if this is a video packet
        if (pkt.stream_index == video_stream_idx) {
            // Check if this is a key frame
            int is_key_frame = (pkt.flags & AV_PKT_FLAG_KEY) != 0;
            
            // If waiting for a key frame, skip non-key frames
            if (waiting_for_keyframe && !is_key_frame) {
                av_packet_unref(&pkt);
                continue;
            } else if (waiting_for_keyframe && is_key_frame) {
                log_info("Found key frame, starting recording");
                waiting_for_keyframe = 0;
                recording_started = 1;
            } else if (!recording_started) {
                recording_started = 1;
            }

            // Process video packet
            ret = process_video_packet(&pkt, input_ctx, output_ctx, video_stream_idx, out_video_stream,
                                      &first_video_dts, &first_video_pts, &last_video_dts, &last_video_pts,
                                      segment_index, &consecutive_timestamp_errors, max_timestamp_errors);
            
            // Handle special error codes
            if (ret == -2) {
                // Timestamp reset needed, reset all timestamp variables
                first_video_dts = AV_NOPTS_VALUE;
                first_video_pts = AV_NOPTS_VALUE;
                last_video_dts = 0;
                last_video_pts = 0;
                first_audio_dts = AV_NOPTS_VALUE;
                first_audio_pts = AV_NOPTS_VALUE;
                last_audio_dts = 0;
                last_audio_pts = 0;
                log_info("Timestamp reset performed");
            } else if (ret < 0 && ret != -2) {
                log_error("Error processing video packet: %d", ret);
                // Continue processing, don't break the loop
            }

            // Update last frame key status for next segment
            segment_info.last_frame_was_key = is_key_frame;
            
            // Increment video packet count
            video_packet_count++;
        }
        // Check if this is an audio packet and audio is enabled
        else if (has_audio && pkt.stream_index == audio_stream_idx) {
            // Only process audio if recording has started
            if (recording_started) {
                // Process audio packet
                ret = process_audio_packet(&pkt, input_ctx, output_ctx, audio_stream_idx, out_audio_stream,
                                          &first_audio_dts, &first_audio_pts, &last_audio_dts, &last_audio_pts,
                                          segment_index, audio_packet_count, &consecutive_timestamp_errors, 
                                          max_timestamp_errors);
                
                // Handle special error codes
                if (ret == -2) {
                    // Timestamp reset needed, reset all timestamp variables
                    first_audio_dts = AV_NOPTS_VALUE;
                    first_audio_pts = AV_NOPTS_VALUE;
                    last_audio_dts = 0;
                    last_audio_pts = 0;
                    log_info("Audio timestamp reset performed");
                } else if (ret < 0 && ret != -2) {
                    log_error("Error processing audio packet: %d", ret);
                    // Continue processing, don't break the loop
                }
                
                // Increment audio packet count
                audio_packet_count++;
            }
        }

        // Free packet
        av_packet_unref(&pkt);
    }

    // Finalize output
    ret = finalize_output(output_ctx);
    if (ret < 0) {
        log_error("Failed to finalize output: %d", ret);
    }

    // Update static variables for next segment
    pthread_mutex_lock(&static_vars_mutex);
    static_input_ctx = input_ctx;
    segment_info.segment_index = segment_index + 1;
    segment_info.has_audio = has_audio;
    pthread_mutex_unlock(&static_vars_mutex);

    log_info("Segment recording completed: %s", output_file);
    log_debug("Video packets: %d, Audio packets: %d", video_packet_count, audio_packet_count);

    return 0;
}

/**
 * Clean up the MP4 segment recorder
 * This function should be called during program shutdown
 */
void mp4_segment_recorder_cleanup(void) {
    pthread_mutex_lock(&static_vars_mutex);
    if (static_input_ctx) {
        avformat_close_input(&static_input_ctx);
        static_input_ctx = NULL;
    }
    segment_info.segment_index = 0;
    segment_info.has_audio = false;
    segment_info.last_frame_was_key = false;
    pthread_mutex_unlock(&static_vars_mutex);

    log_info("MP4 segment recorder cleaned up");
}
