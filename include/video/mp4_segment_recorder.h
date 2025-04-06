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

/**
 * Structure to track segment information
 */
typedef struct {
    int segment_index;
    bool has_audio;
    bool last_frame_was_key;  // Flag to indicate if the last frame of previous segment was a key frame
} segment_info_t;

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
int record_segment(const char *rtsp_url, const char *output_file, int duration, int has_audio);

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

#endif /* MP4_SEGMENT_RECORDER_H */
