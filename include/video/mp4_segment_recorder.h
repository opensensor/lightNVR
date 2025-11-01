/**
 * MP4 Segment Recorder Header
 *
 * This module handles the recording of individual MP4 segments from RTSP streams.
 * It's responsible for:
 * - Opening RTSP streams
 * - Creating MP4 files
 * - Handling timestamps and packet processing
 * - Managing segment rotation
 */

#ifndef MP4_SEGMENT_RECORDER_H
#define MP4_SEGMENT_RECORDER_H

#include <stdbool.h>
#include <libavformat/avformat.h>

// Include the header that defines segment_info_t
#include "mp4_writer_thread.h"


/**
 * Callback invoked when the first keyframe of a segment is detected and writing begins.
 * Allows callers to align external metadata (e.g., DB start_time) to the true
 * recording start aligned to a keyframe.
 */
typedef void (*record_segment_started_cb)(void *user_ctx);

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
 * @param started_cb Optional callback invoked once when the first keyframe is detected
 * @param cb_ctx Opaque context pointer passed to started_cb
 * @return 0 on success, negative value on error
 */
int record_segment(const char *rtsp_url, const char *output_file, int duration, int has_audio,
                   AVFormatContext **input_ctx_ptr, segment_info_t *segment_info_ptr,
                   record_segment_started_cb started_cb, void *cb_ctx);

/**
 * Initialize the MP4 segment recorder
 * This function should be called during program startup
 */
void mp4_segment_recorder_init(void);

/**
 * Clean up all static resources used by the MP4 segment recorder
 * This function should be called during program shutdown to prevent memory leaks
 */
void mp4_segment_recorder_cleanup(void);

/**
 * Write a packet to the MP4 file
 * This function handles both video and audio packets
 *
 * @param writer The MP4 writer instance
 * @param pkt The packet to write
 * @param input_stream The original input stream (for codec parameters)
 * @return 0 on success, negative on error
 */
int mp4_segment_recorder_write_packet(mp4_writer_t *writer, const AVPacket *pkt, const AVStream *input_stream);

#endif /* MP4_SEGMENT_RECORDER_H */
