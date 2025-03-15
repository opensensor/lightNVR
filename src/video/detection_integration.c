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
 * Process a decoded frame for detection
 * This function should be called from the HLS streaming code with already decoded frames
 * 
 * @param stream_name The name of the stream
 * @param frame The decoded AVFrame
 * @param detection_interval How often to process frames (e.g., every 10 frames)
 * @return 0 on success, -1 on error
 */
int process_decoded_frame_for_detection(const char *stream_name, AVFrame *frame, int detection_interval) {
    static int frame_counters[MAX_STREAMS] = {0};
    static char stream_names[MAX_STREAMS][MAX_STREAM_NAME] = {{0}};
    
    // Find the stream's frame counter
    int stream_idx = -1;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (stream_names[i][0] == '\0') {
            // Empty slot, use it for this stream
            if (stream_idx == -1) {
                stream_idx = i;
                strncpy(stream_names[i], stream_name, MAX_STREAM_NAME - 1);
                stream_names[i][MAX_STREAM_NAME - 1] = '\0';
            }
        } else if (strcmp(stream_names[i], stream_name) == 0) {
            // Found existing stream
            stream_idx = i;
            break;
        }
    }
    
    if (stream_idx == -1) {
        log_error("Too many streams for frame counters");
        return -1;
    }
    
    // Increment frame counter for this stream
    frame_counters[stream_idx]++;
    int frame_counter = frame_counters[stream_idx];
    
    // Reset counter if it gets too large to prevent overflow
    if (frame_counter > 1000000) {
        frame_counters[stream_idx] = 0;
    }
    
    log_info("Processing decoded frame %d for stream %s (interval: %d)", frame_counter, stream_name, detection_interval);
    
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
    
    // Convert frame to RGB format for detection
    struct SwsContext *sws_ctx = sws_getContext(
        frame->width, frame->height, frame->format,
        frame->width, frame->height, AV_PIX_FMT_RGB24,
        SWS_BILINEAR, NULL, NULL, NULL);
    
    if (!sws_ctx) {
        log_error("Failed to create SwsContext for stream %s", stream_name);
        return -1;
    }
    
    // Allocate RGB frame
    AVFrame *rgb_frame = av_frame_alloc();
    if (!rgb_frame) {
        log_error("Failed to allocate RGB frame for stream %s", stream_name);
        sws_freeContext(sws_ctx);
        return -1;
    }
    
    // Allocate buffer for RGB frame
    int buffer_size = av_image_get_buffer_size(AV_PIX_FMT_RGB24, frame->width, frame->height, 1);
    uint8_t *buffer = (uint8_t *)av_malloc(buffer_size);
    if (!buffer) {
        log_error("Failed to allocate buffer for RGB frame for stream %s", stream_name);
        av_frame_free(&rgb_frame);
        sws_freeContext(sws_ctx);
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
    
    // Debug the frame data
    log_info("RGB frame data pointers: %p, %p, %p", 
             (void*)rgb_frame->data[0], (void*)rgb_frame->data[1], (void*)rgb_frame->data[2]);
    log_info("RGB frame linesize: %d, %d, %d", 
             rgb_frame->linesize[0], rgb_frame->linesize[1], rgb_frame->linesize[2]);
    
    // For RGB24 format, all data is in data[0] but we need to handle stride correctly
    // If the linesize is equal to width*3, we can use the buffer directly
    if (rgb_frame->linesize[0] == frame->width * 3) {
        log_info("RGB frame has no stride padding, using buffer directly");
        int ret = process_frame_for_detection(stream_name, rgb_frame->data[0], frame->width, frame->height, 3, frame_time);
        log_info("process_frame_for_detection returned: %d", ret);
    } else {
        // Otherwise, we need to create a new buffer without stride padding
        log_info("RGB frame has stride padding (linesize: %d, expected: %d), creating a new buffer", 
                 rgb_frame->linesize[0], frame->width * 3);
        uint8_t *packed_buffer = (uint8_t *)malloc(frame->width * frame->height * 3);
        if (!packed_buffer) {
            log_error("Failed to allocate packed buffer for RGB frame");
            av_free(buffer);
            av_frame_free(&rgb_frame);
            sws_freeContext(sws_ctx);
            return -1;
        }
        
        // Copy each row, skipping the stride padding
        for (int y = 0; y < frame->height; y++) {
            memcpy(packed_buffer + y * frame->width * 3, 
                   rgb_frame->data[0] + y * rgb_frame->linesize[0], 
                   frame->width * 3);
        }
        
        // Debug: Check the first few bytes of the packed buffer
        log_info("Packed buffer first 12 bytes: %02x %02x %02x %02x %02x %02x %02x %02x %02x %02x %02x %02x",
                 packed_buffer[0], packed_buffer[1], packed_buffer[2], packed_buffer[3], 
                 packed_buffer[4], packed_buffer[5], packed_buffer[6], packed_buffer[7],
                 packed_buffer[8], packed_buffer[9], packed_buffer[10], packed_buffer[11]);
        
        // Process the packed buffer
        int ret = process_frame_for_detection(stream_name, packed_buffer, frame->width, frame->height, 3, frame_time);
        log_info("process_frame_for_detection returned: %d", ret);
        
        // Free the packed buffer
        free(packed_buffer);
    }
    
    // Cleanup
    av_free(buffer);
    av_frame_free(&rgb_frame);
    sws_freeContext(sws_ctx);
    
    log_info("Finished processing frame %d for detection", frame_counter);
    return 0;
}

/**
 * Process a video packet for detection
 * This function is kept for backward compatibility but now delegates to process_decoded_frame_for_detection
 * when a frame is available
 * 
 * @param stream_name The name of the stream
 * @param pkt The AVPacket to process
 * @param stream The AVStream the packet belongs to
 * @param detection_interval How often to process frames (e.g., every 10 frames)
 * @return 0 on success, -1 on error
 */
int process_packet_for_detection(const char *stream_name, AVPacket *pkt, AVStream *stream, int detection_interval) {
    log_info("process_packet_for_detection called for stream %s - this function is deprecated", stream_name);
    log_info("Use process_decoded_frame_for_detection instead for better performance");
    
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
    
    // Process the decoded frame
    ret = process_decoded_frame_for_detection(stream_name, frame, detection_interval);
    
    // Cleanup
    av_frame_free(&frame);
    avcodec_free_context(&codec_ctx);
    
    return ret;
}
