/**
 * MP4 Writer Implementation
 *
 * This module provides the core functionality for writing MP4 files from RTSP streams.
 * It has been refactored to improve maintainability and reduce memory leaks:
 *
 * - mp4_segment_recorder.c: Handles recording of individual MP4 segments
 * - mp4_writer_thread.c: Handles thread management for MP4 recording
 *
 * This file contains the main MP4 writer API implementation.
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
#include "video/mp4_segment_recorder.h"
#include "video/mp4_writer_thread.h"
#include "database/database_manager.h"
#include "database/db_recordings.h"

/**
 * Enable or disable audio recording
 *
 * @param writer The MP4 writer instance
 * @param enable 1 to enable audio, 0 to disable
 */
void mp4_writer_set_audio(mp4_writer_t *writer, int enable) {
    if (!writer) {
        return;
    }

    writer->has_audio = enable ? 1 : 0;
    log_info("%s audio recording for stream %s",
            enable ? "Enabled" : "Disabled", writer->stream_name);
}

/**
 * Get the current output path for the MP4 writer
 *
 * @param writer The MP4 writer instance
 * @return The current output path or NULL if writer is NULL
 */
const char *mp4_writer_get_output_path(mp4_writer_t *writer) {
    if (!writer) {
        return NULL;
    }

    return writer->output_path;
}

/**
 * Get the current output directory for the MP4 writer
 *
 * @param writer The MP4 writer instance
 * @return The current output directory or NULL if writer is NULL
 */
const char *mp4_writer_get_output_dir(mp4_writer_t *writer) {
    if (!writer) {
        return NULL;
    }

    return writer->output_dir;
}

/**
 * Get the stream name for the MP4 writer
 *
 * @param writer The MP4 writer instance
 * @return The stream name or NULL if writer is NULL
 */
const char *mp4_writer_get_stream_name(mp4_writer_t *writer) {
    if (!writer) {
        return NULL;
    }

    return writer->stream_name;
}

/**
 * Get the segment duration for the MP4 writer
 *
 * @param writer The MP4 writer instance
 * @return The segment duration in seconds or 0 if writer is NULL
 */
int mp4_writer_get_segment_duration(mp4_writer_t *writer) {
    if (!writer) {
        return 0;
    }

    return writer->segment_duration;
}

/**
 * Check if audio recording is enabled
 *
 * @param writer The MP4 writer instance
 * @return 1 if audio is enabled, 0 if disabled or writer is NULL
 */
int mp4_writer_has_audio(mp4_writer_t *writer) {
    if (!writer) {
        return 0;
    }

    return writer->has_audio;
}


// Forward declaration of transcode_audio_packet function
extern int transcode_audio_packet(const char *stream_name,
                                const AVPacket *in_pkt,
                                AVPacket *out_pkt,
                                const AVStream *input_stream);

/**
 * Check if a codec is a PCM variant that needs transcoding
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
 * Write a packet to the MP4 file
 * This function handles both video and audio packets
 *
 * @param writer The MP4 writer instance
 * @param in_pkt The packet to write
 * @param input_stream The original input stream (for codec parameters)
 * @return 0 on success, negative on error
 */
int mp4_writer_write_packet(mp4_writer_t *writer, const AVPacket *in_pkt, const AVStream *input_stream) {
    if (!writer || !in_pkt || !input_stream) {
        log_error("Invalid parameters passed to mp4_writer_write_packet");
        return -1;
    }

    // Update the last packet time
    writer->last_packet_time = time(NULL);

    // If this is an audio packet and audio is disabled, silently drop it
    if (input_stream->codecpar->codec_type == AVMEDIA_TYPE_AUDIO && !writer->has_audio) {
        // Just return success without doing anything
        return 0;
    }

    // If this is an audio packet with a PCM codec, try to transcode it to AAC
    if (input_stream->codecpar->codec_type == AVMEDIA_TYPE_AUDIO) {
        if (is_pcm_codec(input_stream->codecpar->codec_id)) {

            // Create a new packet for the transcoded audio
            AVPacket *transcoded_pkt = av_packet_alloc();
            if (!transcoded_pkt) {
                log_error("Failed to allocate packet for transcoded audio");
                return -1;
            }

            // Transcode the audio packet
            int ret = transcode_audio_packet(writer->stream_name,
                                           in_pkt,
                                           transcoded_pkt,
                                           input_stream);

            if (ret < 0) {
                log_error("Failed to transcode PCM audio packet for %s (codec_id=%d)",
                         writer->stream_name, input_stream->codecpar->codec_id);
                av_packet_free(&transcoded_pkt);
                return 0; // Return success but don't write the packet
            }

            if (transcoded_pkt->size <= 0) {
                // No output packet was generated, this is normal for some frames
                av_packet_free(&transcoded_pkt);
                return 0; // Return success but don't write the packet
            }

            // Write the transcoded packet
            ret = mp4_segment_recorder_write_packet(writer, transcoded_pkt, input_stream);

            // Free the transcoded packet
            av_packet_free(&transcoded_pkt);

            return ret;
        }
    }

    // Process normal packets
    return mp4_segment_recorder_write_packet(writer, in_pkt, input_stream);
}