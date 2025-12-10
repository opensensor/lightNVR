#ifndef FFMPEG_UTILS_H
#define FFMPEG_UTILS_H

#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <libavutil/avutil.h>
#include <libavutil/dict.h>
#include <libavutil/imgutils.h>
#include <libswscale/swscale.h>

/**
 * Log FFmpeg error
 */
void log_ffmpeg_error(int err, const char *message);

/**
 * Initialize FFmpeg libraries
 */
void init_ffmpeg(void);

/**
 * Cleanup FFmpeg resources
 */
void cleanup_ffmpeg(void);

/**
 * Safe cleanup of FFmpeg AVFormatContext
 * This function provides a more thorough cleanup than just avformat_close_input
 * to help prevent memory leaks
 *
 * @param ctx_ptr Pointer to the AVFormatContext pointer to clean up
 */
void safe_avformat_cleanup(AVFormatContext **ctx_ptr);

/**
 * Safe cleanup of FFmpeg packet
 * This function provides a thorough cleanup of an AVPacket to prevent memory leaks
 *
 * @param pkt_ptr Pointer to the AVPacket pointer to clean up
 */
void safe_packet_cleanup(AVPacket **pkt_ptr);

/**
 * Periodic FFmpeg resource reset
 * This function performs a periodic reset of FFmpeg resources to prevent memory growth
 * It should be called periodically during long-running operations
 *
 * @param input_ctx_ptr Pointer to the AVFormatContext pointer to reset
 * @param url The URL to reopen after reset
 * @param protocol The protocol to use (TCP/UDP)
 * @return 0 on success, negative value on error
 */
int periodic_ffmpeg_reset(AVFormatContext **input_ctx_ptr, const char *url, int protocol);

/**
 * Perform comprehensive cleanup of FFmpeg resources
 * This function ensures all resources associated with an AVFormatContext are properly freed
 *
 * @param input_ctx Pointer to the AVFormatContext to clean up
 * @param codec_ctx Pointer to the AVCodecContext to clean up (can be NULL)
 * @param packet Pointer to the AVPacket to clean up (can be NULL)
 * @param frame Pointer to the AVFrame to clean up (can be NULL)
 */
void comprehensive_ffmpeg_cleanup(AVFormatContext **input_ctx, AVCodecContext **codec_ctx, AVPacket **packet, AVFrame **frame);

/**
 * Encode raw image data to JPEG using FFmpeg libraries
 * This replaces the need for calling ffmpeg binary for image conversion
 *
 * @param frame_data Raw image data (RGB24, RGBA, or grayscale)
 * @param width Image width
 * @param height Image height
 * @param channels Number of color channels (1=gray, 3=RGB, 4=RGBA)
 * @param quality JPEG quality (1-100, default 85)
 * @param output_path Path to write the output JPEG file
 * @return 0 on success, negative value on error
 */
int ffmpeg_encode_jpeg(const unsigned char *frame_data, int width, int height,
                       int channels, int quality, const char *output_path);

/**
 * Concatenate multiple TS segments into a single MP4 file using FFmpeg libraries
 * This replaces the need for calling ffmpeg binary with concat demuxer
 *
 * @param segment_paths Array of paths to TS segment files
 * @param segment_count Number of segments in the array
 * @param output_path Path to write the output MP4 file
 * @return 0 on success, negative value on error
 */
int ffmpeg_concat_ts_to_mp4(const char **segment_paths, int segment_count,
                            const char *output_path);

/**
 * Create a directory recursively (like mkdir -p)
 * This replaces system("mkdir -p ...") calls
 *
 * @param path Directory path to create
 * @return 0 on success, -1 on error
 */
int mkdir_recursive(const char *path);

#endif /* FFMPEG_UTILS_H */
