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
#include "video/stream_reader.h"
#include "video/detection_integration.h"

// Structure to track detection stream readers
typedef struct {
    char stream_name[MAX_STREAM_NAME];
    stream_reader_ctx_t *reader_ctx;
    int detection_interval;
    int frame_counter;
    pthread_mutex_t mutex;
} detection_stream_t;

// Array to store detection stream readers
static detection_stream_t detection_streams[MAX_STREAMS];
static pthread_mutex_t detection_streams_mutex = PTHREAD_MUTEX_INITIALIZER;

// Forward declarations
static int detection_packet_callback(const AVPacket *pkt, const AVStream *stream, void *user_data);

/**
 * Initialize detection stream system
 */
void init_detection_stream_system(void) {
    pthread_mutex_lock(&detection_streams_mutex);
    
    for (int i = 0; i < MAX_STREAMS; i++) {
        memset(&detection_streams[i], 0, sizeof(detection_stream_t));
        pthread_mutex_init(&detection_streams[i].mutex, NULL);
    }
    
    pthread_mutex_unlock(&detection_streams_mutex);
    
    log_info("Detection stream system initialized");
}

/**
 * Shutdown detection stream system
 * 
 * CRITICAL FIX: Modified to work with the new architecture where the HLS streaming thread
 * handles detection instead of using a dedicated stream reader.
 */
void shutdown_detection_stream_system(void) {
    log_info("Shutting down detection stream system...");
    pthread_mutex_lock(&detection_streams_mutex);
    
    for (int i = 0; i < MAX_STREAMS; i++) {
        pthread_mutex_lock(&detection_streams[i].mutex);
        
        // Log active detection streams
        if (detection_streams[i].stream_name[0] != '\0') {
            log_info("Disabling detection for stream %s during shutdown", 
                    detection_streams[i].stream_name);
        }
        
        // Clear the detection stream data
        detection_streams[i].reader_ctx = NULL;
        detection_streams[i].stream_name[0] = '\0';
        detection_streams[i].detection_interval = 0;
        detection_streams[i].frame_counter = 0;
        
        pthread_mutex_unlock(&detection_streams[i].mutex);
        
        // Destroy the mutex
        pthread_mutex_destroy(&detection_streams[i].mutex);
    }
    
    pthread_mutex_unlock(&detection_streams_mutex);
    
    log_info("Detection stream system shutdown complete");
}

/**
 * Packet callback for detection stream reader
 * This function is called for each packet read from the stream
 * It decodes the packet and passes the decoded frame to the detection integration
 */
static int detection_packet_callback(const AVPacket *pkt, const AVStream *stream, void *user_data) {
    detection_stream_t *detection_stream = (detection_stream_t *)user_data;
    if (!detection_stream) {
        log_error("Invalid user data in detection_packet_callback");
        return -1;
    }
    
    // Always log the first few callbacks to confirm it's being invoked
    static int initial_callbacks = 0;
    if (initial_callbacks < 5) {
        log_error("DETECTION CALLBACK INVOKED #%d for stream %s", 
                 ++initial_callbacks, detection_stream->stream_name);
    } else {
        // After the first few, log periodically to avoid spam
        static int callback_counter = 0;
        if (callback_counter++ % 100 == 0) {
            log_info("Detection packet callback invoked for stream %s (frame %d)", 
                    detection_stream->stream_name, callback_counter);
        }
    }
    
    // Only process video packets
    if (stream->codecpar->codec_type != AVMEDIA_TYPE_VIDEO) {
        return 0;
    }
    
    // Increment frame counter
    detection_stream->frame_counter++;
    
    // Skip frames based on detection interval
    if (detection_stream->frame_counter % detection_stream->detection_interval != 0) {
        return 0;
    }
    
    log_info("PROCESSING FRAME %d FOR DETECTION (interval: %d, stream: %s)",
             detection_stream->frame_counter, detection_stream->detection_interval,
             detection_stream->stream_name);
    
    // Find decoder
    AVCodec *codec = avcodec_find_decoder(stream->codecpar->codec_id);
    if (!codec) {
        log_error("Failed to find decoder for stream %s", detection_stream->stream_name);
        return -1;
    }
    
    // Create codec context
    AVCodecContext *codec_ctx = avcodec_alloc_context3(codec);
    if (!codec_ctx) {
        log_error("Failed to allocate codec context for stream %s", detection_stream->stream_name);
        return -1;
    }
    
    // Copy codec parameters to codec context
    if (avcodec_parameters_to_context(codec_ctx, stream->codecpar) < 0) {
        log_error("Failed to copy codec parameters to context for stream %s", detection_stream->stream_name);
        avcodec_free_context(&codec_ctx);
        return -1;
    }
    
    // Open codec
    if (avcodec_open2(codec_ctx, codec, NULL) < 0) {
        log_error("Failed to open codec for stream %s", detection_stream->stream_name);
        avcodec_free_context(&codec_ctx);
        return -1;
    }
    
    // Allocate frame
    AVFrame *frame = av_frame_alloc();
    if (!frame) {
        log_error("Failed to allocate frame for stream %s", detection_stream->stream_name);
        avcodec_free_context(&codec_ctx);
        return -1;
    }
    
    // Send packet to decoder
    int ret = avcodec_send_packet(codec_ctx, pkt);
    if (ret < 0) {
        log_error("Failed to send packet to decoder for stream %s: %d", detection_stream->stream_name, ret);
        av_frame_free(&frame);
        avcodec_free_context(&codec_ctx);
        return -1;
    }
    
    // Receive frame from decoder
    ret = avcodec_receive_frame(codec_ctx, frame);
    if (ret < 0) {
        if (ret != AVERROR(EAGAIN) && ret != AVERROR_EOF) {
            log_error("Failed to receive frame from decoder for stream %s: %d", detection_stream->stream_name, ret);
        }
        av_frame_free(&frame);
        avcodec_free_context(&codec_ctx);
        return 0;
    }
    
    // Process the decoded frame for detection
    log_debug("Sending decoded frame to detection integration for stream %s", 
             detection_stream->stream_name);
    process_decoded_frame_for_detection(detection_stream->stream_name, frame, detection_stream->detection_interval);
    
    // Cleanup
    av_frame_free(&frame);
    avcodec_free_context(&codec_ctx);
    
    return 0;
}

/**
 * Start a detection stream reader for a stream
 * 
 * CRITICAL FIX: Modified to work with the new architecture where the HLS streaming thread
 * handles detection instead of using a dedicated stream reader.
 * 
 * @param stream_name The name of the stream
 * @param detection_interval How often to process frames (e.g., every 10 frames)
 * @return 0 on success, -1 on error
 */
int start_detection_stream_reader(const char *stream_name, int detection_interval) {
    log_info("Starting detection for stream %s with interval %d (using HLS streaming thread)", 
             stream_name, detection_interval);
    if (!stream_name) {
        log_error("Invalid stream name for start_detection_stream_reader");
        return -1;
    }
    
    // Verify stream exists
    stream_handle_t stream = get_stream_by_name(stream_name);
    if (!stream) {
        log_error("Stream %s not found", stream_name);
        return -1;
    }
    
    pthread_mutex_lock(&detection_streams_mutex);
    
    // Find an empty slot or existing entry for this stream
    int slot = -1;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (detection_streams[i].stream_name[0] == '\0') {
            if (slot == -1) {
                slot = i;
            }
        } else if (strcmp(detection_streams[i].stream_name, stream_name) == 0) {
            // Stream already has detection enabled, update it
            slot = i;
            break;
        }
    }
    
    if (slot == -1) {
        log_error("No available slots for detection");
        pthread_mutex_unlock(&detection_streams_mutex);
        return -1;
    }
    
    pthread_mutex_lock(&detection_streams[slot].mutex);
    
    // CRITICAL FIX: No longer need to start a dedicated stream reader
    // Just store the detection configuration
    
    // Initialize detection stream
    strncpy(detection_streams[slot].stream_name, stream_name, MAX_STREAM_NAME - 1);
    detection_streams[slot].detection_interval = detection_interval;
    detection_streams[slot].frame_counter = 0;
    
    // CRITICAL FIX: Set reader_ctx to a non-NULL value to indicate detection is enabled
    // This is just a marker, we don't actually use the reader
    detection_streams[slot].reader_ctx = (stream_reader_ctx_t*)1;
    
    log_info("Detection enabled for stream %s with interval %d", 
             stream_name, detection_interval);
    
    pthread_mutex_unlock(&detection_streams[slot].mutex);
    pthread_mutex_unlock(&detection_streams_mutex);
    
    return 0;
}

/**
 * Stop a detection stream reader for a stream
 * 
 * CRITICAL FIX: Modified to work with the new architecture where the HLS streaming thread
 * handles detection instead of using a dedicated stream reader.
 * 
 * @param stream_name The name of the stream
 * @return 0 on success, -1 on error
 */
int stop_detection_stream_reader(const char *stream_name) {
    if (!stream_name) {
        log_error("Invalid stream name for stop_detection_stream_reader");
        return -1;
    }
    
    log_info("Disabling detection for stream %s", stream_name);
    
    pthread_mutex_lock(&detection_streams_mutex);
    
    // Find the detection stream for this stream
    int slot = -1;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (detection_streams[i].stream_name[0] != '\0' && 
            strcmp(detection_streams[i].stream_name, stream_name) == 0) {
            slot = i;
            break;
        }
    }
    
    if (slot == -1) {
        log_warn("No detection configuration found for stream %s", stream_name);
        pthread_mutex_unlock(&detection_streams_mutex);
        return -1;
    }
    
    pthread_mutex_lock(&detection_streams[slot].mutex);
    
    // CRITICAL FIX: No need to stop a stream reader, just clear the configuration
    
    // Clear the detection stream data
    detection_streams[slot].reader_ctx = NULL;
    detection_streams[slot].stream_name[0] = '\0';
    detection_streams[slot].detection_interval = 0;
    detection_streams[slot].frame_counter = 0;
    
    pthread_mutex_unlock(&detection_streams[slot].mutex);
    pthread_mutex_unlock(&detection_streams_mutex);
    
    log_info("Detection disabled for stream %s", stream_name);
    
    return 0;
}

/**
 * Check if a detection stream reader is running for a stream
 * 
 * @param stream_name The name of the stream
 * @return 1 if running, 0 if not, -1 on error
 */
int is_detection_stream_reader_running(const char *stream_name) {
    if (!stream_name) {
        return -1;
    }
    
    pthread_mutex_lock(&detection_streams_mutex);
    
    // Find the detection stream for this stream
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (detection_streams[i].stream_name[0] != '\0' && 
            strcmp(detection_streams[i].stream_name, stream_name) == 0 &&
            detection_streams[i].reader_ctx != NULL) {
            pthread_mutex_unlock(&detection_streams_mutex);
            return 1;
        }
    }
    
    pthread_mutex_unlock(&detection_streams_mutex);
    
    return 0;
}

/**
 * Get the detection interval for a stream
 * 
 * @param stream_name The name of the stream
 * @return The detection interval, or 15 (default) if not found
 */
int get_detection_interval(const char *stream_name) {
    if (!stream_name) {
        return 15; // Default interval
    }
    
    pthread_mutex_lock(&detection_streams_mutex);
    
    // Find the detection stream for this stream
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (detection_streams[i].stream_name[0] != '\0' && 
            strcmp(detection_streams[i].stream_name, stream_name) == 0 &&
            detection_streams[i].reader_ctx != NULL) {
            int interval = detection_streams[i].detection_interval;
            pthread_mutex_unlock(&detection_streams_mutex);
            return interval > 0 ? interval : 15; // Use default if interval is invalid
        }
    }
    
    pthread_mutex_unlock(&detection_streams_mutex);
    
    return 15; // Default interval
}

/**
 * Print status of all detection stream readers
 * This is useful for debugging detection issues
 */
void print_detection_stream_status(void) {
    pthread_mutex_lock(&detection_streams_mutex);
    
    log_info("=== Detection Stream Status ===");
    int active_count = 0;
    
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (detection_streams[i].stream_name[0] != '\0') {
            pthread_mutex_lock(&detection_streams[i].mutex);
            
            const char *status = detection_streams[i].reader_ctx ? "ACTIVE" : "INACTIVE";
            log_info("Stream %s: %s (interval: %d, frame counter: %d)", 
                    detection_streams[i].stream_name, 
                    status,
                    detection_streams[i].detection_interval,
                    detection_streams[i].frame_counter);
            
            if (detection_streams[i].reader_ctx) {
                active_count++;
            }
            
            pthread_mutex_unlock(&detection_streams[i].mutex);
        }
    }
    
    log_info("Total active detection streams: %d", active_count);
    log_info("==============================");
    
    pthread_mutex_unlock(&detection_streams_mutex);
}
