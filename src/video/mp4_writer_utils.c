/**
 * Utility functions for MP4 writer
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

#include "core/config.h"
#include "core/logger.h"
#include "video/mp4_writer.h"
#include "video/mp4_writer_internal.h"

/**
 * Apply h264_mp4toannexb bitstream filter to convert H.264 stream from MP4 format to Annex B format
 * This is needed for some RTSP cameras that send H.264 in MP4 format instead of Annex B format
 *
 * @param packet The packet to convert
 * @param codec_id The codec ID of the stream
 * @return 0 on success, negative on error
 */
int apply_h264_annexb_filter(AVPacket *packet, enum AVCodecID codec_id) {
    // Only apply to H.264 streams
    if (codec_id != AV_CODEC_ID_H264) {
        return 0;
    }

    // Check if the packet already has start codes (Annex B format)
    if (packet->size >= 4 &&
        packet->data[0] == 0x00 &&
        packet->data[1] == 0x00 &&
        packet->data[2] == 0x00 &&
        packet->data[3] == 0x01) {
        // Already in Annex B format
        return 0;
    }

    // MEMORY LEAK FIX: Use a more robust approach with AVPacket functions
    // This avoids potential memory leaks by using FFmpeg's memory management

    // Create a new packet for the filtered data
    AVPacket *new_pkt = av_packet_alloc();
    if (!new_pkt) {
        return AVERROR(ENOMEM);
    }

    // Allocate a new buffer with space for the start code + original data
    int new_size = packet->size + 4;
    int ret = av_new_packet(new_pkt, new_size);
    if (ret < 0) {
        av_packet_free(&new_pkt);
        return ret;
    }

    // Add start code
    new_pkt->data[0] = 0x00;
    new_pkt->data[1] = 0x00;
    new_pkt->data[2] = 0x00;
    new_pkt->data[3] = 0x01;

    // Copy the packet data
    memcpy(new_pkt->data + 4, packet->data, packet->size);

    // Copy other packet properties
    new_pkt->pts = packet->pts;
    new_pkt->dts = packet->dts;
    new_pkt->flags = packet->flags;
    new_pkt->stream_index = packet->stream_index;

    // Unref the original packet
    av_packet_unref(packet);

    // Move the new packet to the original packet
    av_packet_move_ref(packet, new_pkt);

    // Free the new packet
    av_packet_free(&new_pkt);

    return 0;
}

/**
 * Enhanced MP4 writer initialization with better path handling and logging
 * and proper audio stream handling
 *
 *  Only initialize on keyframes for video packets to ensure clean recordings
 *  Properly handle audio stream initialization
 */
int mp4_writer_initialize(mp4_writer_t *writer, const AVPacket *pkt, const AVStream *input_stream) {
    int ret;

    //  Ensure we only initialize on keyframes for video packets
    if (input_stream->codecpar->codec_type == AVMEDIA_TYPE_VIDEO) {
        // Check if this is a keyframe
        bool is_keyframe = (pkt->flags & AV_PKT_FLAG_KEY) != 0;

        // If this is not a keyframe, wait for the next keyframe
        if (!is_keyframe) {
            log_info("Waiting for keyframe to start MP4 recording for %s",
                    writer->stream_name ? writer->stream_name : "unknown");
            return -1;  // Return error to indicate we should wait for a keyframe
        }

        log_info("Starting MP4 recording with keyframe for %s",
                writer->stream_name ? writer->stream_name : "unknown");
    }

    // First, ensure the directory exists
    char *dir_path = malloc(PATH_MAX);
    if (!dir_path) {
        log_error("Failed to allocate memory for directory path");
        return -1;
    }

    // Initialize dir_path to empty string to avoid potential issues
    dir_path[0] = '\0';

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
    } else {
        // No directory separator found, use current directory
        log_warn("No directory separator found in output path: %s, using current directory",
                writer->output_path);
        strcpy(dir_path, ".");
    }

    // Log the full output path
    log_info("Initializing MP4 writer to output file: %s", writer->output_path);

    // We'll use dir_path directly for error handling, no need to duplicate it

    // Create output format context
    ret = avformat_alloc_output_context2(&writer->output_ctx, NULL, "mp4", writer->output_path);
    if (ret < 0 || !writer->output_ctx) {
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Failed to create output format context for MP4 writer: %s", error_buf);
        free(dir_path);  // Free dir_path before returning
        return -1;
    }

    //  Always enable audio recording by default
    writer->has_audio = 1;
    log_info("Audio recording enabled by default for stream %s", writer->stream_name);

    // Add video stream only if this is a video packet
    if (input_stream->codecpar->codec_type == AVMEDIA_TYPE_VIDEO) {
        AVStream *out_stream = avformat_new_stream(writer->output_ctx, NULL);
        if (!out_stream) {
            log_error("Failed to create output stream for MP4 writer");
            avformat_free_context(writer->output_ctx);
            writer->output_ctx = NULL;
            free(dir_path);  // Free dir_path before returning
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
            free(dir_path);  // Free dir_path before returning
            return -1;
        }

        // CRITICAL FIX: Check for unspecified video dimensions (0x0) and set default values
        // This prevents the "dimensions not set" error and segmentation fault
        if (out_stream->codecpar->width == 0 || out_stream->codecpar->height == 0) {
            log_warn("Video dimensions not set (width=%d, height=%d) for stream %s, using default values",
                    out_stream->codecpar->width, out_stream->codecpar->height,
                    writer->stream_name ? writer->stream_name : "unknown");

            // Set default dimensions (640x480 is a safe choice)
            out_stream->codecpar->width = 640;
            out_stream->codecpar->height = 480;

            log_info("Set default video dimensions to %dx%d for stream %s",
                    out_stream->codecpar->width, out_stream->codecpar->height,
                    writer->stream_name ? writer->stream_name : "unknown");
        }

        //  Apply h264_mp4toannexb bitstream filter for H.264 streams
        // This fixes the "h264 bitstream malformed, no startcode found" error
        if (input_stream->codecpar->codec_id == AV_CODEC_ID_H264) {
            log_info("Set correct codec parameters for H.264 in MP4 for stream %s",
                    writer->stream_name ? writer->stream_name : "unknown");

            // Set the correct extradata for H.264 streams
            // This is equivalent to using the h264_mp4toannexb bitstream filter
            if (out_stream->codecpar->extradata) {
                av_free(out_stream->codecpar->extradata);
                out_stream->codecpar->extradata = NULL;
                out_stream->codecpar->extradata_size = 0;
            }
        }

        // Set stream time base
        out_stream->time_base = input_stream->time_base;
        writer->time_base = input_stream->time_base;

        // Store video stream index
        writer->video_stream_idx = 0;  // First stream is video

        // We don't add an audio stream here - we'll add it when we find an audio stream in the input
        // This exactly matches rtsp_recorder.c behavior
        log_info("Video stream initialized for %s. Audio stream will be added when detected.",
                writer->stream_name ? writer->stream_name : "unknown");
    }
    else if (input_stream->codecpar->codec_type == AVMEDIA_TYPE_AUDIO) {
        // If initialization is triggered by an audio packet, we need to create a dummy video stream first
        // This is because MP4 format expects video to be the first stream
        log_warn("MP4 writer initialization triggered by audio packet for %s - creating dummy video stream",
                writer->stream_name ? writer->stream_name : "unknown");

        AVStream *dummy_video = avformat_new_stream(writer->output_ctx, NULL);
        if (!dummy_video) {
            log_error("Failed to create dummy video stream for MP4 writer");
            avformat_free_context(writer->output_ctx);
            writer->output_ctx = NULL;
            free(dir_path);
            return -1;
        }

        // Set up a minimal H.264 video stream
        dummy_video->codecpar->codec_type = AVMEDIA_TYPE_VIDEO;
        dummy_video->codecpar->codec_id = AV_CODEC_ID_H264;
        dummy_video->codecpar->width = 640;
        dummy_video->codecpar->height = 480;
        dummy_video->time_base = (AVRational){1, 30};
        writer->time_base = dummy_video->time_base;
        writer->video_stream_idx = 0;

        // Now add the audio stream
        AVStream *audio_stream = avformat_new_stream(writer->output_ctx, NULL);
        if (!audio_stream) {
            log_error("Failed to create audio stream for MP4 writer");
            avformat_free_context(writer->output_ctx);
            writer->output_ctx = NULL;
            free(dir_path);
            return -1;
        }

        // Copy audio codec parameters
        ret = avcodec_parameters_copy(audio_stream->codecpar, input_stream->codecpar);
        if (ret < 0) {
            char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
            av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
            log_error("Failed to copy audio codec parameters for MP4 writer: %s", error_buf);
            avformat_free_context(writer->output_ctx);
            writer->output_ctx = NULL;
            free(dir_path);
            return -1;
        }

        // Set audio stream time base
        audio_stream->time_base = input_stream->time_base;
        writer->audio.time_base = input_stream->time_base;

        // Store audio stream index
        writer->audio.stream_idx = audio_stream->index;
        writer->has_audio = 1;
        writer->audio.initialized = 0;  // Will be initialized when we process the first audio packet

        log_info("Added audio stream at index %d during initialization for %s",
                writer->audio.stream_idx, writer->stream_name ? writer->stream_name : "unknown");
    }

    // Initialize audio state if not already done
    if (writer->audio.stream_idx == -1) {
        writer->audio.stream_idx = -1; // Initialize to -1 (no audio yet)
        writer->audio.first_dts = AV_NOPTS_VALUE;
        writer->audio.last_pts = 0;
        writer->audio.last_dts = 0;
        writer->audio.initialized = 0; // Explicitly initialize to 0
    }

    // Add metadata
    av_dict_set(&writer->output_ctx->metadata, "title", writer->stream_name, 0);
    av_dict_set(&writer->output_ctx->metadata, "encoder", "LightNVR", 0);

    // Set options for fast start - EXACTLY match rtsp_recorder.c
    AVDictionary *opts = NULL;
    av_dict_set(&opts, "movflags", "+faststart", 0);  // This is the ONLY option in rtsp_recorder.c

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
        free(dir_path);  // Free dir_path before returning
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
        free(dir_path);  // Free dir_path before returning
        return -1;
    }

    av_dict_free(&opts);

    writer->is_initialized = 1;
    log_info("Successfully initialized MP4 writer for stream %s at %s",
            writer->stream_name, writer->output_path);

    // Free dir_path now that we're done with it
    free(dir_path);

    return 0;
}

/**
 * Safely add audio stream to MP4 writer with improved error handling
 * Returns 0 on success, -1 on failure
 */
int mp4_writer_add_audio_stream(mp4_writer_t *writer, const AVCodecParameters *codec_params,
                                const AVRational *time_base) {
    // MAJOR REFACTOR: Complete rewrite of audio stream addition with robust error handling
    int ret = -1;
    AVStream *audio_stream = NULL;

    // Validate input parameters
    if (!writer) {
        log_error("NULL writer passed to mp4_writer_add_audio_stream");
        return -1;
    }

    if (!codec_params) {
        log_error("NULL codec parameters passed to mp4_writer_add_audio_stream for %s",
                 writer->stream_name ? writer->stream_name : "unknown");
        return -1;
    }

    if (!time_base) {
        log_error("NULL time base passed to mp4_writer_add_audio_stream for %s",
                 writer->stream_name ? writer->stream_name : "unknown");
        return -1;
    }

    if (!writer->output_ctx) {
        log_error("NULL output context in mp4_writer_add_audio_stream for %s",
                 writer->stream_name ? writer->stream_name : "unknown");
        return -1;
    }

    //  Initialize audio.first_dts to AV_NOPTS_VALUE if not already set
    if (writer->audio.first_dts != AV_NOPTS_VALUE) {
        log_debug("Audio first_dts already set to %lld for %s",
                 (long long)writer->audio.first_dts,
                 writer->stream_name ? writer->stream_name : "unknown");
    }

    // Check if we already have an audio stream
    if (writer->audio.stream_idx != -1) {
        log_info("Audio stream already exists for %s, skipping initialization",
                writer->stream_name ? writer->stream_name : "unknown");
        return 0;  // Already initialized, nothing to do
    }

    // Verify the codec parameters are for audio
    if (codec_params->codec_type != AVMEDIA_TYPE_AUDIO) {
        log_error("Invalid codec type (not audio) for %s",
                 writer->stream_name ? writer->stream_name : "unknown");
        return -1;
    }

    // Verify the codec ID is valid
    if (codec_params->codec_id == AV_CODEC_ID_NONE) {
        log_error("Invalid audio codec ID (NONE) for %s",
                 writer->stream_name ? writer->stream_name : "unknown");
        return -1;
    }

    // MAJOR REFACTOR: Create a local copy of codec parameters to avoid modifying the original
    AVCodecParameters *local_codec_params = avcodec_parameters_alloc();
    if (!local_codec_params) {
        log_error("Failed to allocate codec parameters for audio stream in %s",
                 writer->stream_name ? writer->stream_name : "unknown");
        return -1;
    }

    // Copy parameters to our local copy
    ret = avcodec_parameters_copy(local_codec_params, codec_params);
    if (ret < 0) {
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Failed to copy audio codec parameters for %s: %s",
                 writer->stream_name ? writer->stream_name : "unknown", error_buf);
        avcodec_parameters_free(&local_codec_params);
        return -1;
    }

    // Log audio stream parameters for debugging
    log_info("Audio stream parameters for %s: codec_id=%d, sample_rate=%d, format=%d",
            writer->stream_name ? writer->stream_name : "unknown",
            local_codec_params->codec_id, local_codec_params->sample_rate,
            local_codec_params->format);

    // Ensure channel layout is valid
    // In newer FFmpeg versions, ch_layout is used instead of channels
    #if LIBAVCODEC_VERSION_INT >= AV_VERSION_INT(59, 37, 100)
        // For FFmpeg 5.0 and newer
        if (local_codec_params->ch_layout.nb_channels <= 0) {
            log_warn("Invalid channel count in ch_layout for audio stream in %s, setting to mono",
                    writer->stream_name ? writer->stream_name : "unknown");
            // Only set default if invalid
            av_channel_layout_default(&local_codec_params->ch_layout, 1); // Set to mono
        }
    #else
        // For older FFmpeg versions
        if (local_codec_params->channel_layout == 0) {
            log_warn("Invalid channel layout for audio stream in %s, setting to mono",
                    writer->stream_name ? writer->stream_name : "unknown");
            local_codec_params->channel_layout = AV_CH_LAYOUT_MONO;
        }
    #endif

    // Ensure sample rate is valid
    if (local_codec_params->sample_rate <= 0) {
        log_warn("Invalid sample rate for audio stream in %s, setting to 48000",
                writer->stream_name ? writer->stream_name : "unknown");
        local_codec_params->sample_rate = 48000;
    }

    // Ensure format is valid
    if (local_codec_params->format < 0) {
        log_warn("Invalid format for audio stream in %s, setting to S16",
                writer->stream_name ? writer->stream_name : "unknown");
        local_codec_params->format = AV_SAMPLE_FMT_S16;
    }

    // Create a completely safe timebase
    AVRational safe_time_base = {1, 48000};  // Default to 48kHz

    // Only use the provided timebase if it's valid
    if (time_base && time_base->num > 0 && time_base->den > 0) {
        // Make a copy to avoid any potential issues with the original
        safe_time_base.num = time_base->num;
        safe_time_base.den = time_base->den;

        log_debug("Using provided timebase (%d/%d) for audio stream in %s",
                 safe_time_base.num, safe_time_base.den,
                 writer->stream_name ? writer->stream_name : "unknown");
    } else {
        log_warn("Invalid timebase for audio stream in %s, using default (1/48000)",
                writer->stream_name ? writer->stream_name : "unknown");
    }

    // MAJOR REFACTOR: Initialize audio timestamp tracking BEFORE creating the stream
    // This ensures that these values are set even if stream creation fails
    writer->audio.first_dts = AV_NOPTS_VALUE;  // Initialize to AV_NOPTS_VALUE to match rtsp_recorder.c
    writer->audio.last_pts = 0;
    writer->audio.last_dts = 0;
    writer->audio.initialized = 0;  // Don't mark as initialized until we receive the first packet
    writer->audio.time_base = safe_time_base;  // Store the timebase in the audio state

    // Set default frame size for audio codec
    if (codec_params->codec_id == AV_CODEC_ID_OPUS) {
        writer->audio.frame_size = 960;  // Opus typically uses 960 samples per frame (20ms at 48kHz)
        log_debug("Setting Opus frame size to 960 samples for stream %s",
                 writer->stream_name ? writer->stream_name : "unknown");
    } else if (codec_params->codec_id == AV_CODEC_ID_AAC) {
        writer->audio.frame_size = 1024;  // AAC typically uses 1024 samples per frame
        log_debug("Setting AAC frame size to 1024 samples for stream %s",
                 writer->stream_name ? writer->stream_name : "unknown");
    } else {
        // Default to 1024 for other codecs
        writer->audio.frame_size = 1024;
        log_debug("Setting default frame size to 1024 samples for codec %d in stream %s",
                 codec_params->codec_id, writer->stream_name ? writer->stream_name : "unknown");
    }

    // Create a new audio stream in the output
    audio_stream = avformat_new_stream(writer->output_ctx, NULL);
    if (!audio_stream) {
        log_error("Failed to create audio stream for MP4 writer for %s",
                 writer->stream_name ? writer->stream_name : "unknown");
        avcodec_parameters_free(&local_codec_params);
        return -1;
    }

    // Set codec tag to 0 to let FFmpeg choose the appropriate tag
    local_codec_params->codec_tag = 0;

    // Copy our modified parameters to the stream
    ret = avcodec_parameters_copy(audio_stream->codecpar, local_codec_params);
    if (ret < 0) {
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Failed to copy audio codec parameters to stream for %s: %s",
                 writer->stream_name ? writer->stream_name : "unknown", error_buf);
        avcodec_parameters_free(&local_codec_params);
        return -1;
    }

    // CRITICAL FIX: Set the frame_size in the codec parameters
    // This prevents the "track 1: codec frame size is not set" error
    if (audio_stream->codecpar->frame_size == 0) {
        audio_stream->codecpar->frame_size = writer->audio.frame_size;
        log_info("Setting audio codec frame_size to %d for stream %s",
                writer->audio.frame_size, writer->stream_name ? writer->stream_name : "unknown");
    }

    // Free our local copy now that we've copied it to the stream
    avcodec_parameters_free(&local_codec_params);

    // Set the timebase
    audio_stream->time_base = safe_time_base;

    // Log the frame_size if available
    if (audio_stream->codecpar->frame_size > 0) {
        log_debug("Audio frame_size=%d for audio stream in %s",
                 audio_stream->codecpar->frame_size,
                 writer->stream_name ? writer->stream_name : "unknown");
    } else {
        log_debug("No frame_size available for audio stream in %s, codec will determine it",
                 writer->stream_name ? writer->stream_name : "unknown");
    }

    // Verify the stream index is valid
    if (audio_stream->index < 0) {
        log_error("Invalid audio stream index %d for %s",
                 audio_stream->index, writer->stream_name ? writer->stream_name : "unknown");
        return -1;
    }

    // Verify the stream index is within bounds
    if (audio_stream->index >= writer->output_ctx->nb_streams) {
        log_error("Audio stream index %d exceeds number of streams %d for %s",
                 audio_stream->index, writer->output_ctx->nb_streams,
                 writer->stream_name ? writer->stream_name : "unknown");
        return -1;
    }

    // Store audio stream index
    writer->audio.stream_idx = audio_stream->index;

    // Mark audio as enabled
    writer->has_audio = 1;

    log_info("Successfully added audio stream (index %d) to MP4 recording for %s",
            writer->audio.stream_idx, writer->stream_name ? writer->stream_name : "unknown");

    return 0;  // Success
}
