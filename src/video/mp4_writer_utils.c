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
#include "video/ffmpeg_utils.h"

// Structure to hold audio transcoding context
typedef struct {
    AVCodecContext *decoder_ctx;
    AVCodecContext *encoder_ctx;
    AVFrame *frame;
    AVPacket *in_pkt;
    AVPacket *out_pkt;
    int initialized;
} audio_transcoder_t;

// Global transcoder context for each stream
static audio_transcoder_t audio_transcoders[MAX_STREAMS] = {0};
static char audio_transcoder_stream_names[MAX_STREAMS][MAX_STREAM_NAME] = {{0}};
static pthread_mutex_t audio_transcoder_mutex = PTHREAD_MUTEX_INITIALIZER;

/**
 * Initialize audio transcoder for a stream
 *
 * @param stream_name Name of the stream
 * @param codec_params Original codec parameters (PCM variants including μ-law, A-law, S16LE, etc.)
 * @param time_base Time base of the original stream
 * @return Index of the transcoder in the array, or -1 on error
 */
static int init_audio_transcoder(const char *stream_name,
                               const AVCodecParameters *codec_params,
                               const AVRational *time_base) {
    int ret = -1;
    int slot = -1;
    const AVCodec *decoder = NULL;
    const AVCodec *encoder = NULL;

    pthread_mutex_lock(&audio_transcoder_mutex);

    // Find an empty slot or existing entry for this stream
    for (int i = 0; i < g_config.max_streams; i++) {
        if (!audio_transcoders[i].initialized) {
            slot = i;
            break;
        } else if (audio_transcoder_stream_names[i][0] != '\0' &&
                  strcmp(audio_transcoder_stream_names[i], stream_name) == 0) {
            // Stream already has a transcoder, return its index
            pthread_mutex_unlock(&audio_transcoder_mutex);
            return i;
        }
    }

    if (slot == -1) {
        log_error("No available slots for audio transcoder registration");
        pthread_mutex_unlock(&audio_transcoder_mutex);
        return -1;
    }

    // Find the PCM decoder for the specific codec
    decoder = avcodec_find_decoder(codec_params->codec_id);
    if (!decoder) {
        log_error("Failed to find decoder for PCM audio (codec_id=%d) in %s",
                 codec_params->codec_id, stream_name);
        pthread_mutex_unlock(&audio_transcoder_mutex);
        return -1;
    }

    // Find the AAC encoder
    encoder = avcodec_find_encoder(AV_CODEC_ID_AAC);
    if (!encoder) {
        log_error("Failed to find AAC encoder for %s", stream_name);
        pthread_mutex_unlock(&audio_transcoder_mutex);
        return -1;
    }

    // Create decoder context
    audio_transcoders[slot].decoder_ctx = avcodec_alloc_context3(decoder);
    if (!audio_transcoders[slot].decoder_ctx) {
        log_error("Failed to allocate decoder context for %s", stream_name);
        goto cleanup;
    }

    // Copy parameters to decoder context
    ret = avcodec_parameters_to_context(audio_transcoders[slot].decoder_ctx, codec_params);
    if (ret < 0) {
        log_ffmpeg_error(ret, "Failed to copy parameters to decoder context");
        goto cleanup;
    }

    // Set time base
    audio_transcoders[slot].decoder_ctx->time_base = *time_base;

    // Open decoder
    ret = avcodec_open2(audio_transcoders[slot].decoder_ctx, decoder, NULL);
    if (ret < 0) {
        log_ffmpeg_error(ret, "Failed to open PCM audio decoder");
        goto cleanup;
    }

    // Create encoder context
    audio_transcoders[slot].encoder_ctx = avcodec_alloc_context3(encoder);
    if (!audio_transcoders[slot].encoder_ctx) {
        log_error("Failed to allocate encoder context for %s", stream_name);
        goto cleanup;
    }

    // Set encoder parameters
    audio_transcoders[slot].encoder_ctx->sample_fmt = AV_SAMPLE_FMT_FLTP; // AAC requires float planar format
    audio_transcoders[slot].encoder_ctx->sample_rate = audio_transcoders[slot].decoder_ctx->sample_rate;

    // Handle channel layout using the newer FFmpeg API (5.0+)
#if LIBAVCODEC_VERSION_INT >= AV_VERSION_INT(59, 37, 100)
    // Copy channel layout from decoder to encoder
    av_channel_layout_copy(&audio_transcoders[slot].encoder_ctx->ch_layout,
                          &audio_transcoders[slot].decoder_ctx->ch_layout);

    // If channel layout is not set, default to stereo
    if (audio_transcoders[slot].encoder_ctx->ch_layout.nb_channels == 0) {
        av_channel_layout_default(&audio_transcoders[slot].encoder_ctx->ch_layout, 2); // Default to stereo
    }
#else
    // For older FFmpeg versions
    audio_transcoders[slot].encoder_ctx->channels = audio_transcoders[slot].decoder_ctx->channels;
    audio_transcoders[slot].encoder_ctx->channel_layout = av_get_default_channel_layout(audio_transcoders[slot].decoder_ctx->channels);
#endif

    audio_transcoders[slot].encoder_ctx->bit_rate = 128000; // 128 kbps, a good default for AAC
    audio_transcoders[slot].encoder_ctx->time_base = (AVRational){1, audio_transcoders[slot].encoder_ctx->sample_rate};

    // Open encoder
    ret = avcodec_open2(audio_transcoders[slot].encoder_ctx, encoder, NULL);
    if (ret < 0) {
        log_ffmpeg_error(ret, "Failed to open AAC encoder");
        goto cleanup;
    }

    // Allocate frame and packets
    audio_transcoders[slot].frame = av_frame_alloc();
    if (!audio_transcoders[slot].frame) {
        log_error("Failed to allocate frame for %s", stream_name);
        goto cleanup;
    }

    audio_transcoders[slot].in_pkt = av_packet_alloc();
    if (!audio_transcoders[slot].in_pkt) {
        log_error("Failed to allocate input packet for %s", stream_name);
        goto cleanup;
    }

    audio_transcoders[slot].out_pkt = av_packet_alloc();
    if (!audio_transcoders[slot].out_pkt) {
        log_error("Failed to allocate output packet for %s", stream_name);
        goto cleanup;
    }

    // Mark as initialized
    audio_transcoders[slot].initialized = 1;

    // Store stream name
    strncpy(audio_transcoder_stream_names[slot], stream_name, MAX_STREAM_NAME - 1);
    audio_transcoder_stream_names[slot][MAX_STREAM_NAME - 1] = '\0';

    log_info("Successfully initialized audio transcoder from μ-law to AAC for %s", stream_name);

#if LIBAVCODEC_VERSION_INT >= AV_VERSION_INT(59, 37, 100)
    log_info("Sample rate: %d, Channels: %d, Bit rate: %ld",
            audio_transcoders[slot].encoder_ctx->sample_rate,
            audio_transcoders[slot].encoder_ctx->ch_layout.nb_channels,
            audio_transcoders[slot].encoder_ctx->bit_rate);
#else
    log_info("Sample rate: %d, Channels: %d, Bit rate: %ld",
            audio_transcoders[slot].encoder_ctx->sample_rate,
            audio_transcoders[slot].encoder_ctx->channels,
            audio_transcoders[slot].encoder_ctx->bit_rate);
#endif

    pthread_mutex_unlock(&audio_transcoder_mutex);
    return slot;

cleanup:
    if (audio_transcoders[slot].decoder_ctx) {
        avcodec_free_context(&audio_transcoders[slot].decoder_ctx);
        audio_transcoders[slot].decoder_ctx = NULL;
    }

    if (audio_transcoders[slot].encoder_ctx) {
        avcodec_free_context(&audio_transcoders[slot].encoder_ctx);
        audio_transcoders[slot].encoder_ctx = NULL;
    }

    if (audio_transcoders[slot].frame) {
        av_frame_free(&audio_transcoders[slot].frame);
        audio_transcoders[slot].frame = NULL;
    }

    if (audio_transcoders[slot].in_pkt) {
        av_packet_free(&audio_transcoders[slot].in_pkt);
        audio_transcoders[slot].in_pkt = NULL;
    }

    if (audio_transcoders[slot].out_pkt) {
        av_packet_free(&audio_transcoders[slot].out_pkt);
        audio_transcoders[slot].out_pkt = NULL;
    }

    audio_transcoders[slot].initialized = 0;
    audio_transcoder_stream_names[slot][0] = '\0';

    pthread_mutex_unlock(&audio_transcoder_mutex);
    return -1;
}

/**
 * Cleanup audio transcoder for a stream
 *
 * @param stream_name Name of the stream
 */
void cleanup_audio_transcoder(const char *stream_name) {
    pthread_mutex_lock(&audio_transcoder_mutex);

    for (int i = 0; i < g_config.max_streams; i++) {
        if (audio_transcoder_stream_names[i][0] != '\0' &&
            strcmp(audio_transcoder_stream_names[i], stream_name) == 0) {
            // Found the transcoder for this stream
            if (audio_transcoders[i].decoder_ctx) {
                avcodec_free_context(&audio_transcoders[i].decoder_ctx);
                audio_transcoders[i].decoder_ctx = NULL;
            }

            if (audio_transcoders[i].encoder_ctx) {
                avcodec_free_context(&audio_transcoders[i].encoder_ctx);
                audio_transcoders[i].encoder_ctx = NULL;
            }

            if (audio_transcoders[i].frame) {
                av_frame_free(&audio_transcoders[i].frame);
                audio_transcoders[i].frame = NULL;
            }

            if (audio_transcoders[i].in_pkt) {
                av_packet_free(&audio_transcoders[i].in_pkt);
                audio_transcoders[i].in_pkt = NULL;
            }

            if (audio_transcoders[i].out_pkt) {
                av_packet_free(&audio_transcoders[i].out_pkt);
                audio_transcoders[i].out_pkt = NULL;
            }

            audio_transcoders[i].initialized = 0;
            audio_transcoder_stream_names[i][0] = '\0';

            log_info("Cleaned up audio transcoder for stream %s", stream_name);
            break;
        }
    }

    pthread_mutex_unlock(&audio_transcoder_mutex);
}

/**
 * Transcode an audio packet from PCM (μ-law, A-law, S16LE, etc.) to AAC
 *
 * @param stream_name Name of the stream
 * @param in_pkt Input packet (PCM format)
 * @param out_pkt Output packet (AAC) - must be allocated by caller
 * @param input_stream Original input stream
 * @return 0 on success, negative on error
 */
int transcode_audio_packet(const char *stream_name,
                          const AVPacket *in_pkt,
                          AVPacket *out_pkt,
                          const AVStream *input_stream) {
    int ret = 0;
    int transcoder_idx = -1;
    int got_output = 0;

    // Find the transcoder for this stream
    for (int i = 0; i < g_config.max_streams; i++) {
        if (audio_transcoders[i].initialized &&
            audio_transcoder_stream_names[i][0] != '\0' &&
            strcmp(audio_transcoder_stream_names[i], stream_name) == 0) {
            transcoder_idx = i;
            break;
        }
    }

    if (transcoder_idx == -1) {
        // Initialize a new transcoder
        transcoder_idx = init_audio_transcoder(stream_name,
                                             input_stream->codecpar,
                                             &input_stream->time_base);
        if (transcoder_idx == -1) {
            log_error("Failed to initialize audio transcoder for %s", stream_name);
            return -1;
        }
    }

    // Copy input packet to our internal packet
    av_packet_unref(audio_transcoders[transcoder_idx].in_pkt);
    ret = av_packet_ref(audio_transcoders[transcoder_idx].in_pkt, in_pkt);
    if (ret < 0) {
        log_ffmpeg_error(ret, "Failed to copy input packet");
        return ret;
    }

    // Send packet to decoder
    ret = avcodec_send_packet(audio_transcoders[transcoder_idx].decoder_ctx,
                             audio_transcoders[transcoder_idx].in_pkt);
    if (ret < 0) {
        log_ffmpeg_error(ret, "Failed to send packet to decoder");
        return ret;
    }

    // Receive frame from decoder
    ret = avcodec_receive_frame(audio_transcoders[transcoder_idx].decoder_ctx,
                               audio_transcoders[transcoder_idx].frame);
    if (ret < 0) {
        if (ret == AVERROR(EAGAIN) || ret == AVERROR_EOF) {
            // Need more input or end of file, not an error
            return 0;
        }
        log_ffmpeg_error(ret, "Failed to receive frame from decoder");
        return ret;
    }

    // Send frame to encoder
    ret = avcodec_send_frame(audio_transcoders[transcoder_idx].encoder_ctx,
                            audio_transcoders[transcoder_idx].frame);
    if (ret < 0) {
        log_ffmpeg_error(ret, "Failed to send frame to encoder");
        return ret;
    }

    // Receive packet from encoder
    av_packet_unref(audio_transcoders[transcoder_idx].out_pkt);
    ret = avcodec_receive_packet(audio_transcoders[transcoder_idx].encoder_ctx,
                                audio_transcoders[transcoder_idx].out_pkt);
    if (ret < 0) {
        if (ret == AVERROR(EAGAIN) || ret == AVERROR_EOF) {
            // Need more input or end of file, not an error
            return 0;
        }
        log_ffmpeg_error(ret, "Failed to receive packet from encoder");
        return ret;
    }

    // Copy output packet to caller's packet
    av_packet_unref(out_pkt);
    ret = av_packet_ref(out_pkt, audio_transcoders[transcoder_idx].out_pkt);
    if (ret < 0) {
        log_ffmpeg_error(ret, "Failed to copy output packet");
        return ret;
    }

    // Set output packet time base to match the encoder's time base
    out_pkt->time_base = audio_transcoders[transcoder_idx].encoder_ctx->time_base;

    // Set output packet stream index to match the input packet
    out_pkt->stream_index = in_pkt->stream_index;

    return 0;
}

/**
 * Transcode audio from PCM (μ-law, A-law, S16LE, etc.) to AAC format
 *
 * @param codec_params Original codec parameters (PCM format)
 * @param time_base Time base of the original stream
 * @param stream_name Name of the stream (for logging)
 * @param transcoded_params Output parameter to store the transcoded codec parameters
 * @return 0 on success, negative on error
 */
static int transcode_mulaw_to_aac(const AVCodecParameters *codec_params,
                                 const AVRational *time_base,
                                 const char *stream_name,
                                 AVCodecParameters **transcoded_params) {
    int ret = 0;
    AVCodecContext *decoder_ctx = NULL;
    AVCodecContext *encoder_ctx = NULL;
    const AVCodec *decoder = NULL;
    const AVCodec *encoder = NULL;
    AVFrame *frame = NULL;
    AVPacket *pkt = NULL;

    // Allocate output codec parameters
    *transcoded_params = avcodec_parameters_alloc();
    if (!*transcoded_params) {
        log_error("Failed to allocate transcoded codec parameters for %s",
                stream_name ? stream_name : "unknown");
        ret = AVERROR(ENOMEM);
        goto cleanup;
    }

    // Find the PCM decoder for the specific codec
    decoder = avcodec_find_decoder(codec_params->codec_id);
    if (!decoder) {
        log_error("Failed to find decoder for PCM audio (codec_id=%d) in %s",
                codec_params->codec_id, stream_name ? stream_name : "unknown");
        ret = AVERROR_DECODER_NOT_FOUND;
        goto cleanup;
    }

    // Find the AAC encoder
    encoder = avcodec_find_encoder(AV_CODEC_ID_AAC);
    if (!encoder) {
        log_error("Failed to find AAC encoder for %s",
                stream_name ? stream_name : "unknown");
        ret = AVERROR_ENCODER_NOT_FOUND;
        goto cleanup;
    }

    // Create decoder context
    decoder_ctx = avcodec_alloc_context3(decoder);
    if (!decoder_ctx) {
        log_error("Failed to allocate decoder context for %s",
                stream_name ? stream_name : "unknown");
        ret = AVERROR(ENOMEM);
        goto cleanup;
    }

    // Copy parameters to decoder context
    ret = avcodec_parameters_to_context(decoder_ctx, codec_params);
    if (ret < 0) {
        log_ffmpeg_error(ret, "Failed to copy parameters to decoder context");
        goto cleanup;
    }

    // Set time base
    decoder_ctx->time_base = *time_base;

    // Open decoder
    ret = avcodec_open2(decoder_ctx, decoder, NULL);
    if (ret < 0) {
        log_ffmpeg_error(ret, "Failed to open μ-law decoder");
        goto cleanup;
    }

    // Create encoder context
    encoder_ctx = avcodec_alloc_context3(encoder);
    if (!encoder_ctx) {
        log_error("Failed to allocate encoder context for %s",
                stream_name ? stream_name : "unknown");
        ret = AVERROR(ENOMEM);
        goto cleanup;
    }

    // Set encoder parameters
    encoder_ctx->sample_fmt = AV_SAMPLE_FMT_FLTP; // AAC requires float planar format
    encoder_ctx->sample_rate = decoder_ctx->sample_rate;

    // Handle channel layout using the newer FFmpeg API (5.0+)
#if LIBAVCODEC_VERSION_INT >= AV_VERSION_INT(59, 37, 100)
    // Copy channel layout from decoder to encoder
    av_channel_layout_copy(&encoder_ctx->ch_layout, &decoder_ctx->ch_layout);

    // If channel layout is not set, default to stereo
    if (encoder_ctx->ch_layout.nb_channels == 0) {
        av_channel_layout_default(&encoder_ctx->ch_layout, 2); // Default to stereo
    }
#else
    // For older FFmpeg versions
    encoder_ctx->channels = decoder_ctx->channels;
    encoder_ctx->channel_layout = av_get_default_channel_layout(decoder_ctx->channels);
#endif

    encoder_ctx->bit_rate = 128000; // 128 kbps, a good default for AAC
    encoder_ctx->time_base = (AVRational){1, encoder_ctx->sample_rate};

    // Open encoder
    ret = avcodec_open2(encoder_ctx, encoder, NULL);
    if (ret < 0) {
        log_ffmpeg_error(ret, "Failed to open AAC encoder");
        goto cleanup;
    }

    // Get the encoder parameters
    ret = avcodec_parameters_from_context(*transcoded_params, encoder_ctx);
    if (ret < 0) {
        log_ffmpeg_error(ret, "Failed to get parameters from encoder context");
        goto cleanup;
    }

    log_info("Successfully configured transcoding from μ-law to AAC for %s",
            stream_name ? stream_name : "unknown");

#if LIBAVCODEC_VERSION_INT >= AV_VERSION_INT(59, 37, 100)
    log_info("Sample rate: %d, Channels: %d, Bit rate: %ld",
            encoder_ctx->sample_rate, encoder_ctx->ch_layout.nb_channels, encoder_ctx->bit_rate);
#else
    log_info("Sample rate: %d, Channels: %d, Bit rate: %ld",
            encoder_ctx->sample_rate, encoder_ctx->channels, encoder_ctx->bit_rate);
#endif

    ret = 0; // Success

cleanup:
    if (decoder_ctx) avcodec_free_context(&decoder_ctx);
    if (encoder_ctx) avcodec_free_context(&encoder_ctx);
    if (frame) av_frame_free(&frame);
    if (pkt) av_packet_free(&pkt);

    if (ret < 0 && *transcoded_params) {
        avcodec_parameters_free(transcoded_params);
        *transcoded_params = NULL;
    }

    return ret;
}

/**
 * Check if a codec is a PCM variant
 *
 * @param codec_id The codec ID to check
 * @return true if it's a PCM codec, false otherwise
 */
static bool is_pcm_codec(enum AVCodecID codec_id) {
    switch (codec_id) {
        case AV_CODEC_ID_PCM_S16LE:
        case AV_CODEC_ID_PCM_S16BE:
        case AV_CODEC_ID_PCM_U16LE:
        case AV_CODEC_ID_PCM_U16BE:
        case AV_CODEC_ID_PCM_S8:
        case AV_CODEC_ID_PCM_U8:
        case AV_CODEC_ID_PCM_MULAW:
        case AV_CODEC_ID_PCM_ALAW:
        case AV_CODEC_ID_PCM_S32LE:
        case AV_CODEC_ID_PCM_S32BE:
        case AV_CODEC_ID_PCM_U32LE:
        case AV_CODEC_ID_PCM_U32BE:
        case AV_CODEC_ID_PCM_S24LE:
        case AV_CODEC_ID_PCM_S24BE:
        case AV_CODEC_ID_PCM_U24LE:
        case AV_CODEC_ID_PCM_U24BE:
        case AV_CODEC_ID_PCM_S24DAUD:
        case AV_CODEC_ID_PCM_ZORK:
        case AV_CODEC_ID_PCM_S16LE_PLANAR:
        case AV_CODEC_ID_PCM_DVD:
        case AV_CODEC_ID_PCM_F32BE:
        case AV_CODEC_ID_PCM_F32LE:
        case AV_CODEC_ID_PCM_F64BE:
        case AV_CODEC_ID_PCM_F64LE:
        case AV_CODEC_ID_PCM_BLURAY:
        case AV_CODEC_ID_PCM_LXF:
            return true;
        default:
            return false;
    }
}

/**
 * Check if an audio codec is compatible with MP4 format
 *
 * @param codec_id The codec ID to check
 * @param codec_name Output parameter to store the codec name
 * @return true if compatible, false otherwise
 */
static bool is_audio_codec_compatible_with_mp4(enum AVCodecID codec_id, const char **codec_name) {
    bool is_compatible = true;

    switch (codec_id) {
        case AV_CODEC_ID_AAC:
            *codec_name = "AAC";
            break;
        case AV_CODEC_ID_MP3:
            *codec_name = "MP3";
            break;
        case AV_CODEC_ID_AC3:
            *codec_name = "AC3";
            break;
        case AV_CODEC_ID_OPUS:
            *codec_name = "Opus";
            break;
        case AV_CODEC_ID_PCM_MULAW:
            *codec_name = "PCM μ-law (G.711 μ-law)";
            is_compatible = false;
            break;
        case AV_CODEC_ID_PCM_ALAW:
            *codec_name = "PCM A-law (G.711 A-law)";
            is_compatible = false;
            break;
        case AV_CODEC_ID_PCM_S16LE:
            *codec_name = "PCM signed 16-bit little-endian";
            is_compatible = false;
            break;
        case AV_CODEC_ID_PCM_S16BE:
            *codec_name = "PCM signed 16-bit big-endian";
            is_compatible = false;
            break;
        case AV_CODEC_ID_PCM_U8:
            *codec_name = "PCM unsigned 8-bit";
            is_compatible = false;
            break;
        case AV_CODEC_ID_PCM_S8:
            *codec_name = "PCM signed 8-bit";
            is_compatible = false;
            break;
        default:
            // Check if it's any other PCM codec
            if (is_pcm_codec(codec_id)) {
                *codec_name = "PCM (various)";
                is_compatible = false;
            } else {
                *codec_name = "Unknown";
                // Assume compatible for unknown codecs and let FFmpeg handle it
            }
            break;
    }

    return is_compatible;
}

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

    if (!writer->output_path) {
        log_error("MP4 writer output_path is NULL");
        free(dir_path);
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
        if (mkdir_recursive(dir_path) != 0) {
            log_warn("Failed to create directory: %s", dir_path);
        }

        // Set permissions to ensure it's writable
        if (chmod_recursive(dir_path, 0777) != 0) {
            log_warn("Failed to set permissions: %s", dir_path);
        }
    } else {
        // No directory separator found, use current directory
        log_warn("No directory separator found in output path: %s, using current directory",
                writer->output_path);
        dir_path[0] = '.';
        dir_path[1] = '\0';
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

        // BUGFIX: Zero out codec_tag so the MP4 muxer selects the correct tag.
        // RTSP/RTP uses different codec tags than MP4; carrying over the input
        // tag produces a malformed moov atom (grey screen in players).
        out_stream->codecpar->codec_tag = 0;

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

        // Log H.264 codec info and extradata status for diagnostics
        if (input_stream->codecpar->codec_id == AV_CODEC_ID_H264) {
            log_info("H.264 video stream for %s — extradata (SPS/PPS) %s (%d bytes)",
                    writer->stream_name ? writer->stream_name : "unknown",
                    out_stream->codecpar->extradata ? "present" : "MISSING",
                    out_stream->codecpar->extradata_size);
            // NOTE: Do NOT delete extradata here.  The MP4 container requires
            // SPS/PPS in the avcC box for decoders to initialize.  Removing it
            // produces unplayable grey-screen files.
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
        // Check if the audio codec is compatible with MP4 format
        // MP4 container has limited audio codec support
        const char *codec_name = "unknown";
        bool is_compatible = is_audio_codec_compatible_with_mp4(input_stream->codecpar->codec_id, &codec_name);

        if (!is_compatible) {
            // For incompatible codecs, attempt transcoding if it's a PCM codec
            if (is_pcm_codec(input_stream->codecpar->codec_id)) {
                log_info("Attempting to transcode %s audio to AAC for MP4 compatibility in stream %s",
                        codec_name, writer->stream_name ? writer->stream_name : "unknown");

                // Try to transcode PCM to AAC
                AVCodecParameters *transcoded_params = NULL;
                int transcode_ret = transcode_mulaw_to_aac(input_stream->codecpar,
                                                         &input_stream->time_base,
                                                         writer->stream_name,
                                                         &transcoded_params);

                if (transcode_ret >= 0 && transcoded_params) {
                    log_info("Successfully transcoded %s audio to AAC for stream %s",
                            codec_name, writer->stream_name ? writer->stream_name : "unknown");
                    
                    // Create a dummy video stream first (MP4 expects video to be the first stream)
                    log_warn("MP4 writer initialization triggered by transcoded audio packet for %s - creating dummy video stream",
                            writer->stream_name ? writer->stream_name : "unknown");
                    
                    AVStream *dummy_video = avformat_new_stream(writer->output_ctx, NULL);
                    if (!dummy_video) {
                        log_error("Failed to create dummy video stream for MP4 writer");
                        avcodec_parameters_free(&transcoded_params);
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
                    
                    // Now add the transcoded audio stream
                    AVStream *audio_stream = avformat_new_stream(writer->output_ctx, NULL);
                    if (!audio_stream) {
                        log_error("Failed to create audio stream for MP4 writer");
                        avcodec_parameters_free(&transcoded_params);
                        avformat_free_context(writer->output_ctx);
                        writer->output_ctx = NULL;
                        free(dir_path);
                        return -1;
                    }
                    
                    // Copy transcoded audio codec parameters
                    ret = avcodec_parameters_copy(audio_stream->codecpar, transcoded_params);
                    if (ret < 0) {
                        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
                        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
                        log_error("Failed to copy transcoded audio codec parameters for MP4 writer: %s", error_buf);
                        avcodec_parameters_free(&transcoded_params);
                        avformat_free_context(writer->output_ctx);
                        writer->output_ctx = NULL;
                        free(dir_path);
                        return -1;
                    }
                    
                    // Free the transcoded parameters now that we've copied them
                    avcodec_parameters_free(&transcoded_params);
                    
                    // Set audio stream time base
                    audio_stream->time_base = input_stream->time_base;
                    writer->audio.time_base = input_stream->time_base;
                    
                    // Store audio stream index
                    writer->audio.stream_idx = audio_stream->index;
                    writer->has_audio = 1;
                    writer->audio.initialized = 0;  // Will be initialized when we process the first audio packet
                    
                    log_info("Added transcoded audio stream at index %d during initialization for %s",
                            writer->audio.stream_idx, writer->stream_name ? writer->stream_name : "unknown");
                    
                    // Skip the rest of the audio stream handling
                    goto skip_audio_stream;
                } else {
                    log_error("Failed to transcode %s audio to AAC: %d", codec_name, transcode_ret);
                    log_error("Audio stream parameters: codec_id=%d, sample_rate=%d, channels=%d",
                             input_stream->codecpar->codec_id, input_stream->codecpar->sample_rate,
                             #if LIBAVCODEC_VERSION_INT >= AV_VERSION_INT(59, 37, 100)
                                 input_stream->codecpar->ch_layout.nb_channels
                             #else
                                 input_stream->codecpar->channels
                             #endif
                            );
                    log_error("Disabling audio recording for this stream");
                    
                    // Disable audio for this writer
                    writer->has_audio = 0;
                }
            } else {
                log_error("%s audio codec is not compatible with MP4 format", codec_name);
                log_error("Audio stream parameters: codec_id=%d, sample_rate=%d, channels=%d",
                         input_stream->codecpar->codec_id, input_stream->codecpar->sample_rate,
                         #if LIBAVCODEC_VERSION_INT >= AV_VERSION_INT(59, 37, 100)
                             input_stream->codecpar->ch_layout.nb_channels
                         #else
                             input_stream->codecpar->channels
                         #endif
                        );
                log_error("Disabling audio recording for this stream");
                
                // Disable audio for this writer
                writer->has_audio = 0;
            }

            // We still need to create a dummy video stream to initialize the MP4 writer
            // but we won't add an audio stream
            log_warn("MP4 writer initialization triggered by incompatible audio packet for %s - creating dummy video stream only",
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

            // Skip adding the audio stream
            goto skip_audio_stream;
        } else {
            log_info("Using %s audio codec for stream %s",
                    codec_name, writer->stream_name ? writer->stream_name : "unknown");
        }

        // If initialization is triggered by a compatible audio packet, we need to create a dummy video stream first
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

skip_audio_stream:
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

    // Check if the audio codec is compatible with MP4 format
    // MP4 container has limited audio codec support
    const char *codec_name = "unknown";
    bool is_compatible = is_audio_codec_compatible_with_mp4(codec_params->codec_id, &codec_name);

    if (!is_compatible) {
        // For incompatible codecs, attempt transcoding if it's a PCM codec
        if (is_pcm_codec(codec_params->codec_id)) {
            log_info("Attempting to transcode %s audio to AAC for MP4 compatibility in stream %s",
                    codec_name, writer->stream_name ? writer->stream_name : "unknown");

            // Try to transcode PCM to AAC
            AVCodecParameters *transcoded_params = NULL;
            int transcode_ret = transcode_mulaw_to_aac(codec_params, &safe_time_base,
                                                     writer->stream_name, &transcoded_params);

            if (transcode_ret >= 0 && transcoded_params) {
                log_info("Successfully transcoded %s audio to AAC for stream %s",
                        codec_name, writer->stream_name ? writer->stream_name : "unknown");

                // Free the original codec parameters
                avcodec_parameters_free(&local_codec_params);

                // Use the transcoded parameters instead
                local_codec_params = transcoded_params;

                // Update codec name for logging
                codec_name = "AAC (transcoded from PCM)";
                is_compatible = true;
            } else {
                log_error("Failed to transcode %s audio to AAC: %d", codec_name, transcode_ret);
                log_error("Audio stream parameters: codec_id=%d, sample_rate=%d, channels=%d",
                         codec_params->codec_id, codec_params->sample_rate,
                         #if LIBAVCODEC_VERSION_INT >= AV_VERSION_INT(59, 37, 100)
                             codec_params->ch_layout.nb_channels
                         #else
                             codec_params->channels
                         #endif
                        );
                log_error("Disabling audio recording for this stream");

                // Disable audio for this writer
                writer->has_audio = 0;

                // Free the codec parameters
                avcodec_parameters_free(&local_codec_params);

                // Return success but don't add the audio stream
                return 0;
            }
        } else {
            // For other incompatible codecs, disable audio recording
            log_error("%s audio codec is not compatible with MP4 format", codec_name);
            log_error("Audio stream parameters: codec_id=%d, sample_rate=%d, channels=%d",
                     codec_params->codec_id, codec_params->sample_rate,
                     #if LIBAVCODEC_VERSION_INT >= AV_VERSION_INT(59, 37, 100)
                         codec_params->ch_layout.nb_channels
                     #else
                         codec_params->channels
                     #endif
                    );
            log_error("Disabling audio recording for this stream");
            
            // Disable audio for this writer
            writer->has_audio = 0;
            
            // Free the codec parameters
            avcodec_parameters_free(&local_codec_params);
            
            // Return success but don't add the audio stream
            return 0;
        }
    } else {
        log_info("Using %s audio codec for stream %s",
                codec_name, writer->stream_name ? writer->stream_name : "unknown");
    }

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
