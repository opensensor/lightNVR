#include "video/ffmpeg_utils.h"
#include "core/logger.h"
#include "video/ffmpeg_leak_detector.h"
#include "video/stream_protocol.h"

#include <sys/stat.h>
#include <sys/types.h>
#include <errno.h>
#include <string.h>
#include <dirent.h>
#include <pthread.h>
#include <time.h>
#include <stdbool.h>
#include <libavutil/opt.h>

/**
 * Log FFmpeg error
 */
void log_ffmpeg_error(int err, const char *message) {
    char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
    av_strerror(err, error_buf, AV_ERROR_MAX_STRING_SIZE);
    log_error("%s: %s", message, error_buf);
}

/**
 * Initialize FFmpeg libraries
 */
void init_ffmpeg(void) {
    // Initialize FFmpeg
#if LIBAVFORMAT_VERSION_INT < AV_VERSION_INT(58, 9, 100)
    av_register_all();
#endif
    avformat_network_init();

    // Initialize the FFmpeg leak detector
    ffmpeg_leak_detector_init();

    log_info("FFmpeg initialized with leak detection");
}

/**
 * Cleanup FFmpeg resources
 */
void cleanup_ffmpeg(void) {
    // Force cleanup of any tracked FFmpeg allocations
    ffmpeg_force_cleanup_all();

    // Clean up the leak detector
    ffmpeg_leak_detector_cleanup();

    // Cleanup FFmpeg
    avformat_network_deinit();
    log_info("FFmpeg cleaned up");
}

/**
 * Safe cleanup of FFmpeg packet
 * This function provides a thorough cleanup of an AVPacket to prevent memory leaks
 *
 * @param pkt_ptr Pointer to the AVPacket pointer to clean up
 */
void safe_packet_cleanup(AVPacket **pkt_ptr) {
    if (pkt_ptr && *pkt_ptr) {
        AVPacket *pkt_to_free = *pkt_ptr;
        *pkt_ptr = NULL; // Clear the pointer first to prevent double-free

        // Untrack the packet before freeing it
        UNTRACK_AVPACKET(pkt_to_free);

        // Safely unref and free the packet
        av_packet_unref(pkt_to_free);
        av_packet_free(&pkt_to_free);

        log_debug("Safely cleaned up packet");
    }
}

/**
 * Safe cleanup of FFmpeg AVFormatContext
 * This function provides a more thorough cleanup than just avformat_close_input
 * to help prevent memory leaks and segmentation faults
 *
 * @param ctx_ptr Pointer to the AVFormatContext pointer to clean up
 */
void safe_avformat_cleanup(AVFormatContext **ctx_ptr) {
    // CRITICAL FIX: Add comprehensive safety checks to prevent segmentation faults
    if (!ctx_ptr) {
        log_debug("NULL pointer passed to safe_avformat_cleanup");
        return;
    }

    if (!*ctx_ptr) {
        log_debug("NULL AVFormatContext passed to safe_avformat_cleanup");
        return;
    }

    // Make a local copy and immediately NULL the original to prevent double-free
    AVFormatContext *ctx_to_close = *ctx_ptr;
    *ctx_ptr = NULL;

    // Add memory barrier to ensure memory operations are completed
    // This helps prevent race conditions on multi-core systems
    __sync_synchronize();

    // CRITICAL FIX: Add additional validation of the context structure
    // This prevents segmentation faults when dealing with corrupted contexts
    if (!ctx_to_close) {
        log_debug("Context became NULL during cleanup");
        return;
    }

    // Untrack the context before freeing it
    UNTRACK_AVFORMAT_CTX(ctx_to_close);

    // Flush buffers if available
    if (ctx_to_close->pb) {
        log_debug("Flushing I/O buffers during cleanup");
        avio_flush(ctx_to_close->pb);
    } else {
        log_debug("No I/O context available during cleanup");
    }

    // CRITICAL FIX: Check if the context is properly initialized before closing
    // Some contexts might be partially initialized and need different cleanup
    if (ctx_to_close->nb_streams > 0 && ctx_to_close->streams) {
        // Context has streams, use standard close function
        log_debug("Closing input context with %d streams", ctx_to_close->nb_streams);
        avformat_close_input(&ctx_to_close);
    } else {
        // Context doesn't have streams, might be partially initialized
        // Just free the context directly to avoid potential segfaults
        log_debug("Context has no streams, using direct free");
        avformat_free_context(ctx_to_close);
    }

    log_debug("Safely cleaned up AVFormatContext");
}

/**
 * Perform comprehensive cleanup of FFmpeg resources
 * This function ensures all resources associated with an AVFormatContext are properly freed
 *
 * @param input_ctx Pointer to the AVFormatContext to clean up
 * @param codec_ctx Pointer to the AVCodecContext to clean up (can be NULL)
 * @param packet Pointer to the AVPacket to clean up (can be NULL)
 * @param frame Pointer to the AVFrame to clean up (can be NULL)
 */
void comprehensive_ffmpeg_cleanup(AVFormatContext **input_ctx, AVCodecContext **codec_ctx, AVPacket **packet, AVFrame **frame) {
    log_debug("Starting comprehensive FFmpeg resource cleanup");

    // MEMORY LEAK FIX: Add additional cleanup for FFmpeg internal resources
    // This helps prevent memory leaks in FFmpeg's internal structures

    // Clean up frame if provided
    if (frame && *frame) {
        AVFrame *frame_to_free = *frame;
        *frame = NULL; // Clear the pointer first to prevent double-free

        // Untrack the frame before freeing it
        UNTRACK_AVFRAME(frame_to_free);

        av_frame_free(&frame_to_free);
        log_debug("Cleaned up AVFrame");
    }

    // Clean up packet if provided
    if (packet) {
        safe_packet_cleanup(packet);
    }

    // Clean up codec context if provided
    if (codec_ctx && *codec_ctx) {
        AVCodecContext *codec_to_free = *codec_ctx;
        *codec_ctx = NULL; // Clear the pointer first to prevent double-free

        // Untrack the codec context before freeing it
        UNTRACK_AVCODEC_CTX(codec_to_free);

        // MEMORY LEAK FIX: Flush the codec context before freeing it
        // This ensures any internal buffers are properly freed
        avcodec_flush_buffers(codec_to_free);

        avcodec_free_context(&codec_to_free);
        log_debug("Cleaned up AVCodecContext");
    }

    // Clean up input context with special handling for parsers and internal buffers
    if (input_ctx && *input_ctx) {
        AVFormatContext *ctx = *input_ctx;

        // MEMORY LEAK FIX: Manually clean up parsers and internal buffers
        // This addresses the memory leaks in avformat_find_stream_info
        if (ctx->nb_streams > 0) {
            for (unsigned int i = 0; i < ctx->nb_streams; i++) {
                if (ctx->streams[i]) {
                    // Clean up any codec parameters
                    if (ctx->streams[i]->codecpar) {
                        // We don't free codecpar directly as it's managed by the stream
                        // But we can clear any internal buffers
                        if (ctx->streams[i]->codecpar->extradata) {
                            // The extradata is freed by avformat_close_input, but we'll
                            // clear the pointer to prevent potential use-after-free
                            ctx->streams[i]->codecpar->extradata = NULL;
                            ctx->streams[i]->codecpar->extradata_size = 0;
                        }

                        // CRITICAL FIX: Explicitly clean up any internal allocations made by avcodec_parameters_from_context
                        // This addresses the memory leak shown in Valgrind
                        AVCodecParameters *codecpar = ctx->streams[i]->codecpar;
                        if (codecpar) {
                            // Reset fields that might contain allocated memory
                            if (codecpar->extradata) {
                                codecpar->extradata = NULL;
                                codecpar->extradata_size = 0;
                            }

                            // Clear any other fields that might have allocated memory
                            codecpar->ch_layout.u.mask = 0;
                            if (codecpar->ch_layout.nb_channels > 0) {
                                codecpar->ch_layout.nb_channels = 0;
                            }

                            // ENHANCED FIX: Create a temporary codec context to force cleanup of internal allocations
                            // This is a workaround for FFmpeg's internal memory management issues
                            const AVCodec *codec = avcodec_find_decoder(codecpar->codec_id);
                            if (codec) {
                                AVCodecContext *temp_ctx = avcodec_alloc_context3(codec);
                                if (temp_ctx) {
                                    // Copy parameters to the context
                                    int ret = avcodec_parameters_to_context(temp_ctx, codecpar);
                                    if (ret >= 0) {
                                        // Now copy back to parameters - this will reallocate and clean up any leaks
                                        avcodec_parameters_from_context(codecpar, temp_ctx);
                                    }
                                    // Free the temporary context
                                    avcodec_free_context(&temp_ctx);
                                }
                            }

                            // Zero out the entire structure to ensure no pointers remain
                            memset(codecpar, 0, sizeof(AVCodecParameters));
                        }
                    }
                }
            }
        }

        // Now use our safe cleanup function
        safe_avformat_cleanup(input_ctx);
    }

    // Note: We're not using aggressive memory cleanup techniques here
    // to avoid potential segmentation faults
    // Instead, we rely on FFmpeg's own memory management

    log_info("Comprehensive FFmpeg resource cleanup completed");
}

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
int periodic_ffmpeg_reset(AVFormatContext **input_ctx_ptr, const char *url, int protocol) {
    if (!input_ctx_ptr || !url) {
        log_error("Invalid parameters passed to periodic_ffmpeg_reset");
        return -1;
    }

    log_info("Performing periodic FFmpeg resource reset for URL: %s", url);

    // Dump current FFmpeg allocations for debugging
    ffmpeg_dump_allocations();

    // Close the existing context if it exists
    if (*input_ctx_ptr) {
        log_debug("Closing existing input context during reset");
        safe_avformat_cleanup(input_ctx_ptr);
        // input_ctx_ptr should now be NULL
    }

    // Force a garbage collection of FFmpeg resources
    // FFmpeg doesn't have a direct garbage collection function, but we can
    // try to release some memory by calling av_buffer_default_free on NULL
    // which is a no-op but might trigger some internal cleanup
    av_freep(NULL);

    // Open a new input stream
    AVFormatContext *new_ctx = NULL;
    AVDictionary *options = NULL;

    // Set protocol-specific options
    if (protocol == STREAM_PROTOCOL_TCP) {
        av_dict_set(&options, "rtsp_transport", "tcp", 0);
    }

    // Set common options for better reliability
    av_dict_set(&options, "stimeout", "5000000", 0);  // 5 second timeout in microseconds
    av_dict_set(&options, "reconnect", "1", 0);       // Auto reconnect
    av_dict_set(&options, "reconnect_streamed", "1", 0);
    av_dict_set(&options, "reconnect_delay_max", "2", 0); // Max 2 seconds between reconnects

    // Open the input
    int ret = avformat_open_input(&new_ctx, url, NULL, &options);
    av_dict_free(&options);

    if (ret < 0) {
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Failed to open input during reset: %s", error_buf);
        return ret;
    }

    // Find stream info
    ret = avformat_find_stream_info(new_ctx, NULL);
    if (ret < 0) {
        char error_buf[AV_ERROR_MAX_STRING_SIZE] = {0};
        av_strerror(ret, error_buf, AV_ERROR_MAX_STRING_SIZE);
        log_error("Failed to find stream info during reset: %s", error_buf);
        avformat_close_input(&new_ctx);
        return ret;
    }

    // Track the new context
    TRACK_AVFORMAT_CTX(new_ctx);

    // Set the output parameter
    *input_ctx_ptr = new_ctx;

    log_info("Successfully reset FFmpeg resources");
    return 0;
}

/**
 * Create a directory recursively (like mkdir -p)
 */
int mkdir_recursive(const char *path) {
    if (!path || !*path) {
        return -1;
    }

    // Make a mutable copy of the path
    char *path_copy = strdup(path);
    if (!path_copy) {
        log_error("Failed to allocate memory for path copy");
        return -1;
    }

    // Iterate through path components and create each directory
    char *p = path_copy;

    // Skip leading slash for absolute paths
    if (*p == '/') {
        p++;
    }

    while (*p) {
        // Find next slash
        while (*p && *p != '/') {
            p++;
        }

        char saved = *p;
        *p = '\0';

        // Create this directory level
        if (mkdir(path_copy, 0755) != 0 && errno != EEXIST) {
            log_error("Failed to create directory %s: %s", path_copy, strerror(errno));
            free(path_copy);
            return -1;
        }

        *p = saved;
        if (*p) {
            p++;
        }
    }

    free(path_copy);
    return 0;
}

/**
 * Set permissions on a file or directory (like chmod)
 */
int chmod_path(const char *path, mode_t mode) {
    if (!path || !*path) {
        return -1;
    }

    if (chmod(path, mode) != 0) {
        log_warn("Failed to chmod %s: %s", path, strerror(errno));
        return -1;
    }

    return 0;
}

/**
 * Recursively set permissions on a directory and its contents (like chmod -R)
 * Note: This is a simplified version that only sets permissions on the directory itself
 * For full recursive chmod, we would need to walk the directory tree
 */
int chmod_recursive(const char *path, mode_t mode) {
    if (!path || !*path) {
        return -1;
    }

    struct stat st;
    if (stat(path, &st) != 0) {
        log_warn("Failed to stat %s: %s", path, strerror(errno));
        return -1;
    }

    // Set permissions on the path itself
    if (chmod(path, mode) != 0) {
        log_warn("Failed to chmod %s: %s", path, strerror(errno));
        return -1;
    }

    // If it's a directory, recursively process contents
    if (S_ISDIR(st.st_mode)) {
        DIR *dir = opendir(path);
        if (!dir) {
            log_warn("Failed to open directory %s: %s", path, strerror(errno));
            return -1;
        }

        struct dirent *entry;
        char full_path[PATH_MAX];
        int result = 0;

        while ((entry = readdir(dir)) != NULL) {
            // Skip . and ..
            if (strcmp(entry->d_name, ".") == 0 || strcmp(entry->d_name, "..") == 0) {
                continue;
            }

            snprintf(full_path, sizeof(full_path), "%s/%s", path, entry->d_name);

            // Recursively chmod
            if (chmod_recursive(full_path, mode) != 0) {
                result = -1;
                // Continue processing other entries
            }
        }

        closedir(dir);
        return result;
    }

    return 0;
}

/**
 * Encode raw image data to JPEG using FFmpeg libraries
 */
int ffmpeg_encode_jpeg(const unsigned char *frame_data, int width, int height,
                       int channels, int quality, const char *output_path) {
    if (!frame_data || !output_path || width <= 0 || height <= 0) {
        log_error("Invalid parameters for JPEG encoding");
        return -1;
    }

    int ret = 0;
    AVCodecContext *codec_ctx = NULL;
    AVFrame *frame = NULL;
    AVPacket *pkt = NULL;
    struct SwsContext *sws_ctx = NULL;
    FILE *outfile = NULL;

    // Determine input pixel format based on channels
    enum AVPixelFormat src_pix_fmt;
    switch (channels) {
        case 1:
            src_pix_fmt = AV_PIX_FMT_GRAY8;
            break;
        case 3:
            src_pix_fmt = AV_PIX_FMT_RGB24;
            break;
        case 4:
            src_pix_fmt = AV_PIX_FMT_RGBA;
            break;
        default:
            log_error("Unsupported channel count: %d", channels);
            return -1;
    }

    // Find the MJPEG encoder
    const AVCodec *codec = avcodec_find_encoder(AV_CODEC_ID_MJPEG);
    if (!codec) {
        log_error("MJPEG encoder not found");
        return -1;
    }

    // Allocate codec context
    codec_ctx = avcodec_alloc_context3(codec);
    if (!codec_ctx) {
        log_error("Failed to allocate codec context");
        ret = -1;
        goto cleanup;
    }

    // Set codec parameters
    codec_ctx->width = width;
    codec_ctx->height = height;
    codec_ctx->time_base = (AVRational){1, 25};
    codec_ctx->pix_fmt = AV_PIX_FMT_YUVJ420P;  // JPEG uses YUVJ420P

    // Set quality (1-31, lower is better; convert from 1-100 scale)
    int qscale = 31 - (quality * 30 / 100);
    if (qscale < 1) qscale = 1;
    if (qscale > 31) qscale = 31;
    codec_ctx->flags |= AV_CODEC_FLAG_QSCALE;
    codec_ctx->global_quality = qscale * FF_QP2LAMBDA;

    // Open the codec
    ret = avcodec_open2(codec_ctx, codec, NULL);
    if (ret < 0) {
        log_ffmpeg_error(ret, "Failed to open MJPEG codec");
        goto cleanup;
    }

    // Allocate frame
    frame = av_frame_alloc();
    if (!frame) {
        log_error("Failed to allocate frame");
        ret = -1;
        goto cleanup;
    }

    frame->format = codec_ctx->pix_fmt;
    frame->width = width;
    frame->height = height;

    ret = av_frame_get_buffer(frame, 0);
    if (ret < 0) {
        log_ffmpeg_error(ret, "Failed to allocate frame buffer");
        goto cleanup;
    }

    // Create scaling context for pixel format conversion
    sws_ctx = sws_getContext(width, height, src_pix_fmt,
                             width, height, codec_ctx->pix_fmt,
                             SWS_BILINEAR, NULL, NULL, NULL);
    if (!sws_ctx) {
        log_error("Failed to create scaling context");
        ret = -1;
        goto cleanup;
    }

    // Set up source data pointers
    const uint8_t *src_data[4] = {frame_data, NULL, NULL, NULL};
    int src_linesize[4] = {width * channels, 0, 0, 0};

    // Convert pixel format
    sws_scale(sws_ctx, src_data, src_linesize, 0, height,
              frame->data, frame->linesize);

    // Allocate packet
    pkt = av_packet_alloc();
    if (!pkt) {
        log_error("Failed to allocate packet");
        ret = -1;
        goto cleanup;
    }

    // Encode the frame
    ret = avcodec_send_frame(codec_ctx, frame);
    if (ret < 0) {
        log_ffmpeg_error(ret, "Failed to send frame for encoding");
        goto cleanup;
    }

    ret = avcodec_receive_packet(codec_ctx, pkt);
    if (ret < 0) {
        log_ffmpeg_error(ret, "Failed to receive encoded packet");
        goto cleanup;
    }

    // Write the JPEG data to file
    outfile = fopen(output_path, "wb");
    if (!outfile) {
        log_error("Failed to open output file %s: %s", output_path, strerror(errno));
        ret = -1;
        goto cleanup;
    }

    if (fwrite(pkt->data, 1, pkt->size, outfile) != (size_t)pkt->size) {
        log_error("Failed to write JPEG data to file");
        ret = -1;
        goto cleanup;
    }

    log_debug("Encoded JPEG: %dx%d, %d bytes -> %s", width, height, pkt->size, output_path);
    ret = 0;

cleanup:
    if (outfile) fclose(outfile);
    if (pkt) av_packet_free(&pkt);
    if (sws_ctx) sws_freeContext(sws_ctx);
    if (frame) av_frame_free(&frame);
    if (codec_ctx) avcodec_free_context(&codec_ctx);

    return ret;
}

/**
 * Cached JPEG encoder structure
 * Reuses expensive FFmpeg contexts across multiple encode operations
 */
struct jpeg_encoder_cache {
    AVCodecContext *codec_ctx;
    AVFrame *frame;
    AVPacket *pkt;
    struct SwsContext *sws_ctx;
    int width;
    int height;
    int channels;
    int quality;
    enum AVPixelFormat src_pix_fmt;
    pthread_mutex_t mutex;
};

/**
 * Create a cached JPEG encoder
 */
jpeg_encoder_cache_t *jpeg_encoder_cache_create(int width, int height, int channels, int quality) {
    if (width <= 0 || height <= 0 || channels < 1 || channels > 4) {
        log_error("Invalid parameters for JPEG encoder cache: %dx%d, %d channels", width, height, channels);
        return NULL;
    }

    jpeg_encoder_cache_t *encoder = calloc(1, sizeof(jpeg_encoder_cache_t));
    if (!encoder) {
        log_error("Failed to allocate JPEG encoder cache");
        return NULL;
    }

    encoder->width = width;
    encoder->height = height;
    encoder->channels = channels;
    encoder->quality = quality;
    pthread_mutex_init(&encoder->mutex, NULL);

    // Determine input pixel format based on channels
    switch (channels) {
        case 1:
            encoder->src_pix_fmt = AV_PIX_FMT_GRAY8;
            break;
        case 3:
            encoder->src_pix_fmt = AV_PIX_FMT_RGB24;
            break;
        case 4:
            encoder->src_pix_fmt = AV_PIX_FMT_RGBA;
            break;
        default:
            log_error("Unsupported channel count: %d", channels);
            free(encoder);
            return NULL;
    }

    // Find the MJPEG encoder
    const AVCodec *codec = avcodec_find_encoder(AV_CODEC_ID_MJPEG);
    if (!codec) {
        log_error("MJPEG encoder not found");
        free(encoder);
        return NULL;
    }

    // Allocate codec context
    encoder->codec_ctx = avcodec_alloc_context3(codec);
    if (!encoder->codec_ctx) {
        log_error("Failed to allocate codec context for JPEG cache");
        free(encoder);
        return NULL;
    }

    // Set codec parameters
    encoder->codec_ctx->width = width;
    encoder->codec_ctx->height = height;
    encoder->codec_ctx->time_base = (AVRational){1, 25};
    encoder->codec_ctx->pix_fmt = AV_PIX_FMT_YUVJ420P;

    // Set quality (1-31, lower is better; convert from 1-100 scale)
    int qscale = 31 - (quality * 30 / 100);
    if (qscale < 1) qscale = 1;
    if (qscale > 31) qscale = 31;
    encoder->codec_ctx->flags |= AV_CODEC_FLAG_QSCALE;
    encoder->codec_ctx->global_quality = qscale * FF_QP2LAMBDA;

    // Open the codec - this is the expensive operation we're caching
    int ret = avcodec_open2(encoder->codec_ctx, codec, NULL);
    if (ret < 0) {
        log_ffmpeg_error(ret, "Failed to open MJPEG codec for cache");
        avcodec_free_context(&encoder->codec_ctx);
        free(encoder);
        return NULL;
    }

    // Allocate frame
    encoder->frame = av_frame_alloc();
    if (!encoder->frame) {
        log_error("Failed to allocate frame for JPEG cache");
        avcodec_free_context(&encoder->codec_ctx);
        free(encoder);
        return NULL;
    }

    encoder->frame->format = encoder->codec_ctx->pix_fmt;
    encoder->frame->width = width;
    encoder->frame->height = height;

    ret = av_frame_get_buffer(encoder->frame, 0);
    if (ret < 0) {
        log_ffmpeg_error(ret, "Failed to allocate frame buffer for JPEG cache");
        av_frame_free(&encoder->frame);
        avcodec_free_context(&encoder->codec_ctx);
        free(encoder);
        return NULL;
    }

    // Create scaling context for pixel format conversion
    encoder->sws_ctx = sws_getContext(width, height, encoder->src_pix_fmt,
                                       width, height, encoder->codec_ctx->pix_fmt,
                                       SWS_BILINEAR, NULL, NULL, NULL);
    if (!encoder->sws_ctx) {
        log_error("Failed to create scaling context for JPEG cache");
        av_frame_free(&encoder->frame);
        avcodec_free_context(&encoder->codec_ctx);
        free(encoder);
        return NULL;
    }

    // Allocate packet
    encoder->pkt = av_packet_alloc();
    if (!encoder->pkt) {
        log_error("Failed to allocate packet for JPEG cache");
        sws_freeContext(encoder->sws_ctx);
        av_frame_free(&encoder->frame);
        avcodec_free_context(&encoder->codec_ctx);
        free(encoder);
        return NULL;
    }

    log_info("Created cached JPEG encoder: %dx%d, %d channels, quality %d", width, height, channels, quality);
    return encoder;
}

/**
 * Encode a frame using cached encoder
 */
int jpeg_encoder_cache_encode(jpeg_encoder_cache_t *encoder, const unsigned char *frame_data,
                              const char *output_path) {
    if (!encoder || !frame_data || !output_path) {
        log_error("Invalid parameters for cached JPEG encode");
        return -1;
    }

    pthread_mutex_lock(&encoder->mutex);

    int ret = 0;
    FILE *outfile = NULL;

    // Make frame writable
    ret = av_frame_make_writable(encoder->frame);
    if (ret < 0) {
        log_ffmpeg_error(ret, "Failed to make frame writable");
        pthread_mutex_unlock(&encoder->mutex);
        return -1;
    }

    // Set up source data pointers
    const uint8_t *src_data[4] = {frame_data, NULL, NULL, NULL};
    int src_linesize[4] = {encoder->width * encoder->channels, 0, 0, 0};

    // Convert pixel format
    sws_scale(encoder->sws_ctx, src_data, src_linesize, 0, encoder->height,
              encoder->frame->data, encoder->frame->linesize);

    // Reset packet
    av_packet_unref(encoder->pkt);

    // Encode the frame
    ret = avcodec_send_frame(encoder->codec_ctx, encoder->frame);
    if (ret < 0) {
        log_ffmpeg_error(ret, "Failed to send frame for cached encoding");
        pthread_mutex_unlock(&encoder->mutex);
        return -1;
    }

    ret = avcodec_receive_packet(encoder->codec_ctx, encoder->pkt);
    if (ret < 0) {
        log_ffmpeg_error(ret, "Failed to receive encoded packet from cache");
        pthread_mutex_unlock(&encoder->mutex);
        return -1;
    }

    // Write the JPEG data to file
    outfile = fopen(output_path, "wb");
    if (!outfile) {
        log_error("Failed to open output file %s: %s", output_path, strerror(errno));
        pthread_mutex_unlock(&encoder->mutex);
        return -1;
    }

    if (fwrite(encoder->pkt->data, 1, encoder->pkt->size, outfile) != (size_t)encoder->pkt->size) {
        log_error("Failed to write JPEG data to file");
        fclose(outfile);
        pthread_mutex_unlock(&encoder->mutex);
        return -1;
    }

    fclose(outfile);
    log_debug("Cached JPEG encode: %dx%d, %d bytes -> %s", encoder->width, encoder->height,
              encoder->pkt->size, output_path);

    pthread_mutex_unlock(&encoder->mutex);
    return 0;
}

/**
 * Encode a frame to memory using cached encoder
 */
int jpeg_encoder_cache_encode_to_memory(jpeg_encoder_cache_t *encoder, const unsigned char *frame_data,
                                        unsigned char **out_data, size_t *out_size) {
    if (!encoder || !frame_data || !out_data || !out_size) {
        log_error("Invalid parameters for cached JPEG encode to memory");
        return -1;
    }

    pthread_mutex_lock(&encoder->mutex);

    int ret = 0;

    // Make frame writable
    ret = av_frame_make_writable(encoder->frame);
    if (ret < 0) {
        log_ffmpeg_error(ret, "Failed to make frame writable");
        pthread_mutex_unlock(&encoder->mutex);
        return -1;
    }

    // Set up source data pointers
    const uint8_t *src_data[4] = {frame_data, NULL, NULL, NULL};
    int src_linesize[4] = {encoder->width * encoder->channels, 0, 0, 0};

    // Convert pixel format
    sws_scale(encoder->sws_ctx, src_data, src_linesize, 0, encoder->height,
              encoder->frame->data, encoder->frame->linesize);

    // Reset packet
    av_packet_unref(encoder->pkt);

    // Encode the frame
    ret = avcodec_send_frame(encoder->codec_ctx, encoder->frame);
    if (ret < 0) {
        log_ffmpeg_error(ret, "Failed to send frame for cached encoding to memory");
        pthread_mutex_unlock(&encoder->mutex);
        return -1;
    }

    ret = avcodec_receive_packet(encoder->codec_ctx, encoder->pkt);
    if (ret < 0) {
        log_ffmpeg_error(ret, "Failed to receive encoded packet from cache");
        pthread_mutex_unlock(&encoder->mutex);
        return -1;
    }

    // Allocate output buffer and copy data
    *out_data = malloc(encoder->pkt->size);
    if (!*out_data) {
        log_error("Failed to allocate memory for JPEG output");
        pthread_mutex_unlock(&encoder->mutex);
        return -1;
    }

    memcpy(*out_data, encoder->pkt->data, encoder->pkt->size);
    *out_size = encoder->pkt->size;

    log_debug("Cached JPEG encode to memory: %dx%d, %d bytes", encoder->width, encoder->height,
              encoder->pkt->size);

    pthread_mutex_unlock(&encoder->mutex);
    return 0;
}

/**
 * Destroy a cached JPEG encoder
 */
void jpeg_encoder_cache_destroy(jpeg_encoder_cache_t *encoder) {
    if (!encoder) return;

    pthread_mutex_lock(&encoder->mutex);

    if (encoder->pkt) av_packet_free(&encoder->pkt);
    if (encoder->sws_ctx) sws_freeContext(encoder->sws_ctx);
    if (encoder->frame) av_frame_free(&encoder->frame);
    if (encoder->codec_ctx) avcodec_free_context(&encoder->codec_ctx);

    pthread_mutex_unlock(&encoder->mutex);
    pthread_mutex_destroy(&encoder->mutex);

    log_info("Destroyed cached JPEG encoder: %dx%d", encoder->width, encoder->height);
    free(encoder);
}

// Global cache for JPEG encoders (one per unique resolution/channel/quality combo)
#define MAX_JPEG_ENCODER_CACHE 16

typedef struct {
    jpeg_encoder_cache_t *encoder;
    int width;
    int height;
    int channels;
    int quality;
    time_t last_used;
} jpeg_encoder_cache_entry_t;

static jpeg_encoder_cache_entry_t jpeg_encoder_global_cache[MAX_JPEG_ENCODER_CACHE];
static pthread_mutex_t jpeg_encoder_global_mutex = PTHREAD_MUTEX_INITIALIZER;
static bool jpeg_encoder_cache_initialized = false;

/**
 * Get or create a cached JPEG encoder
 */
jpeg_encoder_cache_t *jpeg_encoder_get_cached(int width, int height, int channels, int quality) {
    pthread_mutex_lock(&jpeg_encoder_global_mutex);

    if (!jpeg_encoder_cache_initialized) {
        memset(jpeg_encoder_global_cache, 0, sizeof(jpeg_encoder_global_cache));
        jpeg_encoder_cache_initialized = true;
    }

    time_t now = time(NULL);
    int oldest_idx = -1;
    time_t oldest_time = now;
    int free_idx = -1;

    // Look for existing encoder or find a free/oldest slot
    for (int i = 0; i < MAX_JPEG_ENCODER_CACHE; i++) {
        if (jpeg_encoder_global_cache[i].encoder) {
            // Check if this matches our requirements
            if (jpeg_encoder_global_cache[i].width == width &&
                jpeg_encoder_global_cache[i].height == height &&
                jpeg_encoder_global_cache[i].channels == channels &&
                jpeg_encoder_global_cache[i].quality == quality) {
                // Found a match
                jpeg_encoder_global_cache[i].last_used = now;
                jpeg_encoder_cache_t *encoder = jpeg_encoder_global_cache[i].encoder;
                pthread_mutex_unlock(&jpeg_encoder_global_mutex);
                return encoder;
            }
            // Track oldest for potential eviction
            if (jpeg_encoder_global_cache[i].last_used < oldest_time) {
                oldest_time = jpeg_encoder_global_cache[i].last_used;
                oldest_idx = i;
            }
        } else if (free_idx < 0) {
            free_idx = i;
        }
    }

    // No match found, need to create a new encoder
    int target_idx;
    if (free_idx >= 0) {
        target_idx = free_idx;
    } else if (oldest_idx >= 0) {
        // Evict oldest encoder
        log_info("Evicting cached JPEG encoder: %dx%d to make room for %dx%d",
                 jpeg_encoder_global_cache[oldest_idx].width,
                 jpeg_encoder_global_cache[oldest_idx].height,
                 width, height);
        jpeg_encoder_cache_destroy(jpeg_encoder_global_cache[oldest_idx].encoder);
        jpeg_encoder_global_cache[oldest_idx].encoder = NULL;
        target_idx = oldest_idx;
    } else {
        log_error("No available slot for JPEG encoder cache");
        pthread_mutex_unlock(&jpeg_encoder_global_mutex);
        return NULL;
    }

    // Create new encoder
    jpeg_encoder_cache_t *new_encoder = jpeg_encoder_cache_create(width, height, channels, quality);
    if (new_encoder) {
        jpeg_encoder_global_cache[target_idx].encoder = new_encoder;
        jpeg_encoder_global_cache[target_idx].width = width;
        jpeg_encoder_global_cache[target_idx].height = height;
        jpeg_encoder_global_cache[target_idx].channels = channels;
        jpeg_encoder_global_cache[target_idx].quality = quality;
        jpeg_encoder_global_cache[target_idx].last_used = now;
    }

    pthread_mutex_unlock(&jpeg_encoder_global_mutex);
    return new_encoder;
}

/**
 * Cleanup all cached JPEG encoders
 */
void jpeg_encoder_cleanup_all(void) {
    pthread_mutex_lock(&jpeg_encoder_global_mutex);

    for (int i = 0; i < MAX_JPEG_ENCODER_CACHE; i++) {
        if (jpeg_encoder_global_cache[i].encoder) {
            jpeg_encoder_cache_destroy(jpeg_encoder_global_cache[i].encoder);
            jpeg_encoder_global_cache[i].encoder = NULL;
        }
    }

    jpeg_encoder_cache_initialized = false;
    log_info("Cleaned up all cached JPEG encoders");

    pthread_mutex_unlock(&jpeg_encoder_global_mutex);
}

/**
 * Concatenate multiple TS segments into a single MP4 file using FFmpeg libraries
 */
int ffmpeg_concat_ts_to_mp4(const char **segment_paths, int segment_count,
                            const char *output_path) {
    if (!segment_paths || segment_count <= 0 || !output_path) {
        log_error("Invalid parameters for TS concatenation");
        return -1;
    }

    int ret = 0;
    AVFormatContext *output_ctx = NULL;
    AVFormatContext **input_contexts = NULL;
    int *stream_mapping = NULL;
    int64_t *last_pts = NULL;
    int64_t *last_dts = NULL;
    int64_t *pts_offset = NULL;
    int64_t *dts_offset = NULL;
    int64_t *first_pts = NULL;
    int64_t *first_dts = NULL;
    bool *first_packet = NULL;
    int output_stream_count = 0;
    AVDictionary *opts = NULL;

    // Allocate array for input contexts
    input_contexts = calloc(segment_count, sizeof(AVFormatContext *));
    if (!input_contexts) {
        log_error("Failed to allocate input contexts array");
        return -1;
    }

    // Open output context
    ret = avformat_alloc_output_context2(&output_ctx, NULL, "mp4", output_path);
    if (ret < 0 || !output_ctx) {
        log_ffmpeg_error(ret, "Failed to create output context");
        ret = -1;
        goto cleanup;
    }

    // Open first input to get stream info
    ret = avformat_open_input(&input_contexts[0], segment_paths[0], NULL, NULL);
    if (ret < 0) {
        log_ffmpeg_error(ret, "Failed to open first input segment");
        goto cleanup;
    }

    ret = avformat_find_stream_info(input_contexts[0], NULL);
    if (ret < 0) {
        log_ffmpeg_error(ret, "Failed to find stream info");
        goto cleanup;
    }

    // Create output streams based on first input
    output_stream_count = input_contexts[0]->nb_streams;
    stream_mapping = calloc(output_stream_count, sizeof(int));
    last_pts = calloc(output_stream_count, sizeof(int64_t));
    last_dts = calloc(output_stream_count, sizeof(int64_t));
    pts_offset = calloc(output_stream_count, sizeof(int64_t));
    dts_offset = calloc(output_stream_count, sizeof(int64_t));
    first_pts = calloc(output_stream_count, sizeof(int64_t));
    first_dts = calloc(output_stream_count, sizeof(int64_t));
    first_packet = calloc(output_stream_count, sizeof(bool));

    if (!stream_mapping || !last_pts || !last_dts || !pts_offset || !dts_offset ||
        !first_pts || !first_dts || !first_packet) {
        log_error("Failed to allocate stream tracking arrays");
        ret = -1;
        goto cleanup;
    }

    for (int i = 0; i < output_stream_count; i++) {
        AVStream *in_stream = input_contexts[0]->streams[i];
        AVStream *out_stream = avformat_new_stream(output_ctx, NULL);
        if (!out_stream) {
            log_error("Failed to create output stream");
            ret = -1;
            goto cleanup;
        }

        ret = avcodec_parameters_copy(out_stream->codecpar, in_stream->codecpar);
        if (ret < 0) {
            log_ffmpeg_error(ret, "Failed to copy codec parameters");
            goto cleanup;
        }
        out_stream->codecpar->codec_tag = 0;
        out_stream->time_base = in_stream->time_base;
        stream_mapping[i] = i;
        last_pts[i] = 0;
        last_dts[i] = 0;
        pts_offset[i] = 0;
        dts_offset[i] = 0;
    }

    // Set movflags for faststart (moov atom at beginning)
    av_dict_set(&opts, "movflags", "+faststart", 0);

    // Open output file
    if (!(output_ctx->oformat->flags & AVFMT_NOFILE)) {
        ret = avio_open(&output_ctx->pb, output_path, AVIO_FLAG_WRITE);
        if (ret < 0) {
            log_ffmpeg_error(ret, "Failed to open output file");
            goto cleanup;
        }
    }

    // Write header
    ret = avformat_write_header(output_ctx, &opts);
    av_dict_free(&opts);
    opts = NULL;
    if (ret < 0) {
        log_ffmpeg_error(ret, "Failed to write header");
        goto cleanup;
    }

    // Process each segment
    for (int seg = 0; seg < segment_count; seg++) {
        // Open segment if not already open
        if (seg > 0) {
            ret = avformat_open_input(&input_contexts[seg], segment_paths[seg], NULL, NULL);
            if (ret < 0) {
                log_warn("Failed to open segment %s, skipping", segment_paths[seg]);
                continue;
            }
            ret = avformat_find_stream_info(input_contexts[seg], NULL);
            if (ret < 0) {
                log_warn("Failed to find stream info for segment %s, skipping", segment_paths[seg]);
                avformat_close_input(&input_contexts[seg]);
                continue;
            }
        }

        AVPacket *pkt = av_packet_alloc();
        if (!pkt) {
            log_error("Failed to allocate packet");
            ret = -1;
            goto cleanup;
        }

        // Reset first packet tracking for this segment
        for (int i = 0; i < output_stream_count; i++) {
            first_pts[i] = AV_NOPTS_VALUE;
            first_dts[i] = AV_NOPTS_VALUE;
            first_packet[i] = true;
        }

        // Read and write packets
        while (av_read_frame(input_contexts[seg], pkt) >= 0) {
            int stream_idx = pkt->stream_index;
            if (stream_idx >= output_stream_count) {
                av_packet_unref(pkt);
                continue;
            }

            AVStream *in_stream = input_contexts[seg]->streams[stream_idx];
            AVStream *out_stream = output_ctx->streams[stream_idx];

            // Track first packet timestamps for offset calculation
            if (first_packet[stream_idx]) {
                first_pts[stream_idx] = pkt->pts;
                first_dts[stream_idx] = pkt->dts;
                first_packet[stream_idx] = false;
            }

            // Apply timestamp offset and rescale
            if (pkt->pts != AV_NOPTS_VALUE) {
                pkt->pts = av_rescale_q(pkt->pts - first_pts[stream_idx],
                                        in_stream->time_base, out_stream->time_base)
                           + pts_offset[stream_idx];
            }
            if (pkt->dts != AV_NOPTS_VALUE) {
                pkt->dts = av_rescale_q(pkt->dts - first_dts[stream_idx],
                                        in_stream->time_base, out_stream->time_base)
                           + dts_offset[stream_idx];
            }

            // Ensure DTS <= PTS
            if (pkt->pts != AV_NOPTS_VALUE && pkt->dts != AV_NOPTS_VALUE && pkt->dts > pkt->pts) {
                pkt->dts = pkt->pts;
            }

            // Track last timestamps for next segment offset
            if (pkt->pts != AV_NOPTS_VALUE) {
                last_pts[stream_idx] = pkt->pts;
            }
            if (pkt->dts != AV_NOPTS_VALUE) {
                last_dts[stream_idx] = pkt->dts;
            }

            pkt->duration = av_rescale_q(pkt->duration, in_stream->time_base, out_stream->time_base);
            pkt->pos = -1;

            ret = av_interleaved_write_frame(output_ctx, pkt);
            if (ret < 0) {
                log_warn("Failed to write packet from segment %d", seg);
            }
            av_packet_unref(pkt);
        }

        av_packet_free(&pkt);

        // Update offsets for next segment (add a small gap to avoid timestamp collision)
        for (int i = 0; i < output_stream_count; i++) {
            AVStream *out_stream = output_ctx->streams[i];
            int64_t gap = av_rescale_q(1, (AVRational){1, 90000}, out_stream->time_base);
            pts_offset[i] = last_pts[i] + gap;
            dts_offset[i] = last_dts[i] + gap;
        }

        // Close this segment
        avformat_close_input(&input_contexts[seg]);
    }

    // Write trailer
    ret = av_write_trailer(output_ctx);
    if (ret < 0) {
        log_ffmpeg_error(ret, "Failed to write trailer");
        goto cleanup;
    }

    log_info("Successfully concatenated %d segments to %s", segment_count, output_path);
    ret = 0;

cleanup:
    // Close any remaining input contexts
    if (input_contexts) {
        for (int i = 0; i < segment_count; i++) {
            if (input_contexts[i]) {
                avformat_close_input(&input_contexts[i]);
            }
        }
        free(input_contexts);
    }

    // Close output
    if (output_ctx) {
        if (!(output_ctx->oformat->flags & AVFMT_NOFILE) && output_ctx->pb) {
            avio_closep(&output_ctx->pb);
        }
        avformat_free_context(output_ctx);
    }

    if (opts) av_dict_free(&opts);
    free(stream_mapping);
    free(last_pts);
    free(last_dts);
    free(pts_offset);
    free(dts_offset);
    free(first_pts);
    free(first_dts);
    free(first_packet);

    return ret;
}
