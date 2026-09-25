#ifndef FFMPEG_UTILS_H
#define FFMPEG_UTILS_H

#include <sys/types.h>
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
 * Tail slack for packed RGB24 buffers handed to libswscale.
 *
 * libswscale's x86 SSSE3 yuv420p->rgb24 converter (ff_yuv_420_rgb24_ssse3 in
 * libswscale/x86/yuv_2_rgb.asm) advances 16 pixels per loop iteration, but the
 * C wrapper (YUV2RGB_LOOP in libswscale/x86/yuv2rgb.c) only rounds the width up
 * to 8 before checking it against the stride. For any width with (w % 16) >= 8
 * (1080-wide portrait cameras, 856, 1000, 1512, 1944, ...) the last row is
 * written up to 8 pixels (24 bytes) past w*h*3. The rgb24->yuv420p input
 * readers used by the MJPEG encoder also read up to 64 bytes past the end of
 * the last row, for every geometry. Present in FFmpeg 4.3 through 8.1.
 *
 * A buffer sized exactly w*h*3 that sits directly below the heap arena's top
 * chunk therefore gets glibc's "malloc(): corrupted top size" abort (#587).
 * Keep linesize == w*3 (every consumer assumes packed rows) and over-allocate
 * the tail by this many bytes instead.
 */
#define RGB24_SWS_TAIL_PADDING 128

/**
 * Convert a decoded frame to a freshly malloc'd packed RGB24 buffer at the
 * frame's own resolution (linesize == width*3). The allocation carries
 * RGB24_SWS_TAIL_PADDING zeroed bytes after the last row; see above.
 *
 * @param frame        Decoded software frame (hardware frames are rejected)
 * @param rgb_size_out Optional: receives width*height*3 (the payload size,
 *                     not the allocation size)
 * @return malloc'd buffer (caller frees) or NULL on failure
 */
uint8_t *ffmpeg_frame_to_rgb24_padded(const AVFrame *frame, size_t *rgb_size_out);

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
 * Opaque handle for cached JPEG encoder
 */
typedef struct jpeg_encoder_cache jpeg_encoder_cache_t;

/**
 * Create a cached JPEG encoder for a specific resolution
 * This avoids the expensive avcodec_open2() call on every frame
 *
 * @param width Image width
 * @param height Image height
 * @param channels Number of color channels (1=gray, 3=RGB, 4=RGBA)
 * @param quality JPEG quality (1-100, default 85)
 * @return Encoder handle on success, NULL on failure
 */
jpeg_encoder_cache_t *jpeg_encoder_cache_create(int width, int height, int channels, int quality);

/**
 * Encode a frame using a cached JPEG encoder
 * Much faster than ffmpeg_encode_jpeg() for repeated encoding at same resolution
 *
 * @param encoder Cached encoder handle
 * @param frame_data Raw image data (RGB24, RGBA, or grayscale)
 * @param output_path Path to write the output JPEG file
 * @return 0 on success, negative value on error
 */
int jpeg_encoder_cache_encode(jpeg_encoder_cache_t *encoder, const unsigned char *frame_data,
                              const char *output_path);

/**
 * Encode a frame to memory using a cached JPEG encoder
 *
 * @param encoder Cached encoder handle
 * @param frame_data Raw image data (RGB24, RGBA, or grayscale)
 * @param out_data Pointer to receive allocated JPEG data (caller must free)
 * @param out_size Pointer to receive JPEG data size
 * @return 0 on success, negative value on error
 */
int jpeg_encoder_cache_encode_to_memory(jpeg_encoder_cache_t *encoder, const unsigned char *frame_data,
                                        unsigned char **out_data, size_t *out_size);

/**
 * Destroy a cached JPEG encoder and free all resources
 *
 * @param encoder Encoder handle to destroy
 */
void jpeg_encoder_cache_destroy(jpeg_encoder_cache_t *encoder);

/**
 * Get or create a thread-local cached JPEG encoder
 * This provides automatic caching per-thread without manual management
 *
 * @param width Image width
 * @param height Image height
 * @param channels Number of color channels
 * @param quality JPEG quality
 * @return Encoder handle (do NOT destroy - managed automatically)
 */
jpeg_encoder_cache_t *jpeg_encoder_get_cached(int width, int height, int channels, int quality);

/**
 * Cleanup all thread-local cached JPEG encoders
 * Call this during shutdown
 */
void jpeg_encoder_cleanup_all(void);

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

#endif /* FFMPEG_UTILS_H */
