/**
 * Internal header file for MP4 writer implementation
 */

#ifndef MP4_WRITER_INTERNAL_H
#define MP4_WRITER_INTERNAL_H

#include <stdbool.h>
#include <libavformat/avformat.h>
#include "video/mp4_writer.h"

/**
 * Initialize the MP4 writer with the first packet
 *
 * @param writer The MP4 writer instance
 * @param pkt The first packet to write
 * @param input_stream The original input stream (for codec parameters)
 * @return 0 on success, negative on error
 */
int mp4_writer_initialize(mp4_writer_t *writer, const AVPacket *pkt, const AVStream *input_stream);

/**
 * Apply h264_mp4toannexb bitstream filter to convert H.264 stream from MP4 format to Annex B format
 * This is needed for some RTSP cameras that send H.264 in MP4 format instead of Annex B format
 * 
 * @param packet The packet to convert
 * @param codec_id The codec ID of the stream
 * @return 0 on success, negative on error
 */
int apply_h264_annexb_filter(AVPacket *packet, enum AVCodecID codec_id);

/**
 * Write a packet to the MP4 file
 * This function handles both video and audio packets
 *
 * @param writer The MP4 writer instance
 * @param in_pkt The packet to write
 * @param input_stream The original input stream (for codec parameters)
 * @return 0 on success, negative on error
 */
int mp4_writer_write_packet(mp4_writer_t *writer, const AVPacket *in_pkt, const AVStream *input_stream);

/**
 * Check if a codec is a PCM codec
 *
 * @param codec_id The codec ID to check
 * @return true if it's a PCM codec, false otherwise
 */
bool is_pcm_codec(enum AVCodecID codec_id);

/**
 * Check if an audio codec is compatible with MP4 format
 *
 * @param codec_id The codec ID to check
 * @param codec_name Output parameter to store the codec name
 * @return true if compatible, false otherwise
 */
bool is_audio_codec_compatible_with_mp4(enum AVCodecID codec_id, const char **codec_name);

/**
 * Transcode audio from PCM (μ-law, A-law, S16LE, etc.) to AAC format
 *
 * @param codec_params Original codec parameters (PCM format)
 * @param time_base Time base of the original stream
 * @param stream_name Name of the stream (for logging)
 * @param transcoded_params Output parameter to store the transcoded codec parameters
 * @return 0 on success, negative on error
 */
int transcode_mulaw_to_aac(const AVCodecParameters *codec_params,
                           const AVRational *time_base,
                           const char *stream_name,
                           AVCodecParameters **transcoded_params);

/**
 * Transcode an audio packet from PCM to AAC
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
                          const AVStream *input_stream);

/**
 * Clean up the audio transcoder for a stream
 *
 * @param stream_name Name of the stream
 */
void cleanup_audio_transcoder(const char *stream_name);

#endif /* MP4_WRITER_INTERNAL_H */
