#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <pthread.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <fcntl.h>
#include <errno.h>
#include <time.h>
#include <dirent.h>
#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <libavutil/avutil.h>
#include <libavutil/imgutils.h>
#include <libswscale/swscale.h>

#include "core/logger.h"
#include "core/config.h"
#include "video/stream_manager.h"
#include "video/streams.h"
#include "video/detection.h"
#include "video/detection_result.h"

// Forward declaration of the process_frame_for_detection function
extern int process_frame_for_detection(const char *stream_name, const unsigned char *frame_data, 
                                     int width, int height, int channels, time_t frame_time);

/**
 * Process a video packet for detection
 * This function should be called from the HLS streaming code
 */
int process_packet_for_detection(const char *stream_name, AVPacket *pkt, AVStream *stream, int detection_interval) {
    static int frame_counter = 0;
    
    // Only process every Nth frame based on detection_interval
    frame_counter++;
    log_info("Processing frame %d for detection (interval: %d)", frame_counter, detection_interval);
    
    if (frame_counter % detection_interval != 0) {
        log_info("Skipping frame %d (not on interval)", frame_counter);
        return 0;
    }
    
    log_info("Frame %d is on interval, processing for detection", frame_counter);
    
    // Get stream configuration
    stream_handle_t stream_handle = get_stream_by_name(stream_name);
    if (!stream_handle) {
        log_error("Failed to get stream handle for %s", stream_name);
        return -1;
    }
    
    stream_config_t config;
    if (get_stream_config(stream_handle, &config) != 0) {
        log_error("Failed to get stream config for %s", stream_name);
        return -1;
    }
    
    // Check if detection is enabled for this stream
    if (!config.detection_based_recording || config.detection_model[0] == '\0') {
        log_info("Detection not enabled for stream %s", stream_name);
        return 0;
    }
    
    log_info("Detection enabled for stream %s with model %s", stream_name, config.detection_model);
    
    // Get codec parameters
    AVCodecParameters *codecpar = stream->codecpar;
    if (codecpar->codec_type != AVMEDIA_TYPE_VIDEO) {
        log_error("Not a video stream: %s", stream_name);
        return -1;
    }
    
    // Find decoder
    AVCodec *codec = avcodec_find_decoder(codecpar->codec_id);
    if (!codec) {
        log_error("Failed to find decoder for stream %s", stream_name);
        return -1;
    }
    
    // Create codec context
    AVCodecContext *codec_ctx = avcodec_alloc_context3(codec);
    if (!codec_ctx) {
        log_error("Failed to allocate codec context for stream %s", stream_name);
        return -1;
    }
    
    // Copy codec parameters to codec context
    if (avcodec_parameters_to_context(codec_ctx, codecpar) < 0) {
        log_error("Failed to copy codec parameters to context for stream %s", stream_name);
        avcodec_free_context(&codec_ctx);
        return -1;
    }
    
    // Open codec
    if (avcodec_open2(codec_ctx, codec, NULL) < 0) {
        log_error("Failed to open codec for stream %s", stream_name);
        avcodec_free_context(&codec_ctx);
        return -1;
    }
    
    // Allocate frame
    AVFrame *frame = av_frame_alloc();
    if (!frame) {
        log_error("Failed to allocate frame for stream %s", stream_name);
        avcodec_free_context(&codec_ctx);
        return -1;
    }
    
    // Send packet to decoder
    int ret = avcodec_send_packet(codec_ctx, pkt);
    if (ret < 0) {
        log_error("Error sending packet to decoder for stream %s", stream_name);
        av_frame_free(&frame);
        avcodec_free_context(&codec_ctx);
        return -1;
    }
    
    // Receive frame from decoder
    ret = avcodec_receive_frame(codec_ctx, frame);
    if (ret < 0) {
        // Not an error, just no frames available yet
        log_info("No frames available yet for stream %s", stream_name);
        av_frame_free(&frame);
        avcodec_free_context(&codec_ctx);
        return 0;
    }
    
    log_info("Received frame for stream %s: %dx%d", stream_name, frame->width, frame->height);
    
    // Convert frame to RGB format for detection
    struct SwsContext *sws_ctx = sws_getContext(
        frame->width, frame->height, codec_ctx->pix_fmt,
        frame->width, frame->height, AV_PIX_FMT_RGB24,
        SWS_BILINEAR, NULL, NULL, NULL);
    
    if (!sws_ctx) {
        log_error("Failed to create SwsContext for stream %s", stream_name);
        av_frame_free(&frame);
        avcodec_free_context(&codec_ctx);
        return -1;
    }
    
    // Allocate RGB frame
    AVFrame *rgb_frame = av_frame_alloc();
    if (!rgb_frame) {
        log_error("Failed to allocate RGB frame for stream %s", stream_name);
        sws_freeContext(sws_ctx);
        av_frame_free(&frame);
        avcodec_free_context(&codec_ctx);
        return -1;
    }
    
    // Allocate buffer for RGB frame
    int buffer_size = av_image_get_buffer_size(AV_PIX_FMT_RGB24, frame->width, frame->height, 1);
    uint8_t *buffer = (uint8_t *)av_malloc(buffer_size);
    if (!buffer) {
        log_error("Failed to allocate buffer for RGB frame for stream %s", stream_name);
        av_frame_free(&rgb_frame);
        sws_freeContext(sws_ctx);
        av_frame_free(&frame);
        avcodec_free_context(&codec_ctx);
        return -1;
    }
    
    // Setup RGB frame
    av_image_fill_arrays(rgb_frame->data, rgb_frame->linesize, buffer,
                        AV_PIX_FMT_RGB24, frame->width, frame->height, 1);
    
    // Convert frame to RGB
    sws_scale(sws_ctx, (const uint8_t * const *)frame->data, frame->linesize, 0,
             frame->height, rgb_frame->data, rgb_frame->linesize);
    
    log_info("Converted frame to RGB for stream %s", stream_name);
    
    // Process frame for detection
    time_t frame_time = time(NULL);
    log_info("Calling process_frame_for_detection for stream %s", stream_name);
    process_frame_for_detection(stream_name, rgb_frame->data[0], frame->width, frame->height, 3, frame_time);
    
    // Cleanup
    av_free(buffer);
    av_frame_free(&rgb_frame);
    sws_freeContext(sws_ctx);
    av_frame_free(&frame);
    avcodec_free_context(&codec_ctx);
    
    log_info("Finished processing frame %d for detection", frame_counter);
    return 0;
}
