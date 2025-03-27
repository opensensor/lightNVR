/**
 * Internal header file for MP4 writer implementation
 */

#ifndef MP4_WRITER_INTERNAL_H
#define MP4_WRITER_INTERNAL_H

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

#endif /* MP4_WRITER_INTERNAL_H */
