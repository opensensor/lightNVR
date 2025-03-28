#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <pthread.h>
#include <stdbool.h>

#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <libavutil/avutil.h>
#include <libavutil/imgutils.h>
#include <libswscale/swscale.h>

#include "core/logger.h"
#include "core/config.h"
#include "core/shutdown_coordinator.h"
#include "video/detection_thread_pool.h"
#include "video/hls_writer.h"
#include "video/detection_integration.h"
#include "video/detection_recording.h"
#include "video/detection_result.h"

// Thread pool state
static pthread_t detection_threads[MAX_DETECTION_THREADS];
static detection_task_t detection_tasks[MAX_DETECTION_THREADS];
static pthread_mutex_t detection_tasks_mutex = PTHREAD_MUTEX_INITIALIZER;
static pthread_cond_t detection_tasks_cond = PTHREAD_COND_INITIALIZER;
static bool detection_thread_pool_running = false;

// Statistics
static int active_threads = 0;
static int pending_tasks = 0;
static int completed_tasks = 0;

// Forward declaration of the detection function from hls_writer.c
extern void process_packet_for_detection(const char *stream_name, const AVPacket *pkt, const AVCodecParameters *codec_params);

// Forward declaration of the detection interval function from detection_stream.c
extern int get_detection_interval(const char *stream_name);

/**
 * Process a frame from an HLS segment file for detection
 * 
 * @param stream_name The name of the stream
 * @param segment_path The path to the HLS segment file
 * @param segment_duration The duration of the segment in seconds
 * @param timestamp The timestamp of the segment
 * @return 0 on success, non-zero on failure
 */
static int process_segment_for_detection(const char *stream_name, const char *segment_path, 
                                        float segment_duration, time_t timestamp) {
    AVFormatContext *format_ctx = NULL;
    AVCodecContext *codec_ctx = NULL;
    AVFrame *frame = NULL;
    AVPacket *pkt = NULL;
    struct SwsContext *sws_ctx = NULL;
    int video_stream_idx = -1;
    int ret = -1;
    
    log_info("Processing HLS segment for detection: %s (stream: %s)", segment_path, stream_name);
    
    // Open input file
    if (avformat_open_input(&format_ctx, segment_path, NULL, NULL) != 0) {
        log_error("Could not open segment file: %s", segment_path);
        return -1;
    }
    
    // Find stream info
    if (avformat_find_stream_info(format_ctx, NULL) < 0) {
        log_error("Could not find stream info in segment file: %s", segment_path);
        avformat_close_input(&format_ctx);
        return -1;
    }
    
    // Find video stream
    for (int i = 0; i < format_ctx->nb_streams; i++) {
        if (format_ctx->streams[i]->codecpar->codec_type == AVMEDIA_TYPE_VIDEO) {
            video_stream_idx = i;
            break;
        }
    }
    
    if (video_stream_idx == -1) {
        log_error("Could not find video stream in segment file: %s", segment_path);
        avformat_close_input(&format_ctx);
        return -1;
    }
    
    // Get codec
    const AVCodec *codec = avcodec_find_decoder(format_ctx->streams[video_stream_idx]->codecpar->codec_id);
    if (!codec) {
        log_error("Unsupported codec in segment file: %s", segment_path);
        avformat_close_input(&format_ctx);
        return -1;
    }
    
    // Allocate codec context
    codec_ctx = avcodec_alloc_context3(codec);
    if (!codec_ctx) {
        log_error("Could not allocate codec context for segment file: %s", segment_path);
        avformat_close_input(&format_ctx);
        return -1;
    }
    
    // Copy codec parameters
    if (avcodec_parameters_to_context(codec_ctx, format_ctx->streams[video_stream_idx]->codecpar) < 0) {
        log_error("Could not copy codec parameters for segment file: %s", segment_path);
        avcodec_free_context(&codec_ctx);
        avformat_close_input(&format_ctx);
        return -1;
    }
    
    // Open codec
    if (avcodec_open2(codec_ctx, codec, NULL) < 0) {
        log_error("Could not open codec for segment file: %s", segment_path);
        avcodec_free_context(&codec_ctx);
        avformat_close_input(&format_ctx);
        return -1;
    }
    
    // Allocate frame and packet
    frame = av_frame_alloc();
    pkt = av_packet_alloc();
    if (!frame || !pkt) {
        log_error("Could not allocate frame or packet for segment file: %s", segment_path);
        if (frame) av_frame_free(&frame);
        if (pkt) av_packet_free(&pkt);
        avcodec_free_context(&codec_ctx);
        avformat_close_input(&format_ctx);
        return -1;
    }
    
    // Get detection interval
    int detection_interval = get_detection_interval(stream_name);
    if (detection_interval <= 0) {
        detection_interval = 10; // Default to 10 seconds if not configured
    }
    
    // Calculate frame interval based on segment duration and detection interval
    // We want to process approximately one frame per detection interval
    float frames_per_second = format_ctx->streams[video_stream_idx]->avg_frame_rate.num / 
                             (float)format_ctx->streams[video_stream_idx]->avg_frame_rate.den;
    int total_frames = segment_duration * frames_per_second;
    int frame_interval = (detection_interval * frames_per_second) / segment_duration;
    
    if (frame_interval < 1) frame_interval = 1;
    
    log_info("Segment duration: %.2f seconds, FPS: %.2f, Total frames: %d, Frame interval: %d", 
             segment_duration, frames_per_second, total_frames, frame_interval);
    
    // Read frames
    int frame_count = 0;
    int processed_frames = 0;
    
    while (av_read_frame(format_ctx, pkt) >= 0) {
        if (pkt->stream_index == video_stream_idx) {
            frame_count++;
            
            // Process only every Nth frame based on the calculated interval
            if (frame_count % frame_interval == 0) {
                // Send packet to decoder
                ret = avcodec_send_packet(codec_ctx, pkt);
                if (ret < 0) {
                    log_error("Error sending packet to decoder for segment file: %s", segment_path);
                    av_packet_unref(pkt);
                    continue;
                }
                
                // Receive frame from decoder
                ret = avcodec_receive_frame(codec_ctx, frame);
                if (ret == AVERROR(EAGAIN) || ret == AVERROR_EOF) {
                    av_packet_unref(pkt);
                    continue;
                } else if (ret < 0) {
                    log_error("Error receiving frame from decoder for segment file: %s", segment_path);
                    av_packet_unref(pkt);
                    continue;
                }
                
                // Process the frame for detection
                log_info("Processing frame %d from segment file: %s", frame_count, segment_path);
                
                // Calculate frame timestamp based on segment timestamp and frame position
                time_t frame_timestamp = timestamp + (frame_count / frames_per_second);
                
                // Process the frame for detection
                process_decoded_frame_for_detection(stream_name, frame, detection_interval);
                
                processed_frames++;
            }
        }
        
        av_packet_unref(pkt);
    }
    
    log_info("Processed %d frames out of %d total frames from segment file: %s", 
             processed_frames, frame_count, segment_path);
    
    // Cleanup
    av_frame_free(&frame);
    av_packet_free(&pkt);
    avcodec_free_context(&codec_ctx);
    avformat_close_input(&format_ctx);
    
    return 0;
}

/**
 * Detection thread function
 */
static void *detection_thread_func(void *arg) {
    int thread_id = *((int *)arg);
    free(arg); // Free the thread ID memory

    log_info("Detection thread %d started", thread_id);

    // Register with shutdown coordinator
    char component_name[128];
    snprintf(component_name, sizeof(component_name), "detection_thread_%d", thread_id);
    int component_id = register_component(component_name, COMPONENT_DETECTION_THREAD, NULL, 100); // Highest priority (100)
    if (component_id >= 0) {
        log_info("Registered detection thread %d with shutdown coordinator (ID: %d)", thread_id, component_id);
    }

    while (detection_thread_pool_running) {
        // Check if shutdown has been initiated
        if (is_shutdown_initiated()) {
            log_info("Detection thread %d stopping due to system shutdown", thread_id);
            break;
        }
        
        // Wait for a task
        pthread_mutex_lock(&detection_tasks_mutex);
        
        // Update statistics
        active_threads--;
        
        while (detection_thread_pool_running && !detection_tasks[thread_id].in_use) {
            pthread_cond_wait(&detection_tasks_cond, &detection_tasks_mutex);
        }
        
        // Update statistics
        active_threads++;
        pending_tasks--;
        
        // Check if we're shutting down
        if (!detection_thread_pool_running) {
            pthread_mutex_unlock(&detection_tasks_mutex);
            break;
        }

        // Get task data - use MAX_STREAM_NAME to match the size in the detection_task_t struct
        char stream_name[MAX_STREAM_NAME];
        strncpy(stream_name, detection_tasks[thread_id].stream_name, MAX_STREAM_NAME - 1);
        stream_name[MAX_STREAM_NAME - 1] = '\0';

        // Check if this is a segment-based task or a packet-based task
        bool use_segment_file = detection_tasks[thread_id].use_segment_file;
        
        if (use_segment_file) {
            // Get segment data
            char segment_path[MAX_PATH_LENGTH];
            strncpy(segment_path, detection_tasks[thread_id].segment_path, MAX_PATH_LENGTH - 1);
            segment_path[MAX_PATH_LENGTH - 1] = '\0';
            
            float segment_duration = detection_tasks[thread_id].segment_duration;
            time_t timestamp = detection_tasks[thread_id].timestamp;
            
            // Mark task as not in use
            detection_tasks[thread_id].in_use = false;
            pthread_mutex_unlock(&detection_tasks_mutex);
            
            // Process the segment-based detection task
            log_info("Detection thread %d processing segment task for stream %s", thread_id, stream_name);
            process_segment_for_detection(stream_name, segment_path, segment_duration, timestamp);
        } else {
            // Get packet data
            AVPacket *pkt = detection_tasks[thread_id].pkt;
            AVCodecParameters *codec_params = detection_tasks[thread_id].codec_params;
            
            // Mark task as not in use
            detection_tasks[thread_id].in_use = false;
            pthread_mutex_unlock(&detection_tasks_mutex);
            
            // Process the packet-based detection task
            log_info("Detection thread %d processing packet task for stream %s", thread_id, stream_name);
            process_packet_for_detection(stream_name, pkt, codec_params);
            
            // Free the packet and codec parameters
            av_packet_free(&pkt);
            avcodec_parameters_free(&codec_params);
        }

        // Update statistics
        completed_tasks++;
        
        log_info("Detection thread %d completed task for stream %s", thread_id, stream_name);
    }

    // Update component state in shutdown coordinator
    if (component_id >= 0) {
        update_component_state(component_id, COMPONENT_STOPPED);
        log_info("Updated detection thread %d state to STOPPED in shutdown coordinator", thread_id);
    }

    log_info("Detection thread %d exiting", thread_id);
    return NULL;
}

/**
 * Initialize the detection thread pool
 */
int init_detection_thread_pool(void) {
    pthread_mutex_lock(&detection_tasks_mutex);

    // Initialize tasks
    for (int i = 0; i < MAX_DETECTION_THREADS; i++) {
        memset(&detection_tasks[i], 0, sizeof(detection_task_t));
    }

    // Set running flag
    detection_thread_pool_running = true;

    // Create threads
    for (int i = 0; i < MAX_DETECTION_THREADS; i++) {
        int *thread_id = malloc(sizeof(int));
        if (!thread_id) {
            log_error("Failed to allocate memory for thread ID");
            pthread_mutex_unlock(&detection_tasks_mutex);
            return -1;
        }
        *thread_id = i;

        if (pthread_create(&detection_threads[i], NULL, detection_thread_func, thread_id) != 0) {
            log_error("Failed to create detection thread %d", i);
            free(thread_id);
            pthread_mutex_unlock(&detection_tasks_mutex);
            return -1;
        }
    }

    pthread_mutex_unlock(&detection_tasks_mutex);
    log_info("Detection thread pool initialized with %d threads", MAX_DETECTION_THREADS);
    return 0;
}

/**
 * Shutdown the detection thread pool
 */
void shutdown_detection_thread_pool(void) {
    // Check if shutdown has already been initiated
    if (is_shutdown_initiated()) {
        log_info("Shutdown already initiated, detection thread pool will be stopped");
    }

    pthread_mutex_lock(&detection_tasks_mutex);
    detection_thread_pool_running = false;
    pthread_cond_broadcast(&detection_tasks_cond);
    pthread_mutex_unlock(&detection_tasks_mutex);

    // Wait for threads to exit
    for (int i = 0; i < MAX_DETECTION_THREADS; i++) {
        pthread_join(detection_threads[i], NULL);
        log_info("Detection thread %d joined", i);
    }

    // Clean up any remaining tasks
    pthread_mutex_lock(&detection_tasks_mutex);
    for (int i = 0; i < MAX_DETECTION_THREADS; i++) {
        if (detection_tasks[i].in_use) {
            if (detection_tasks[i].pkt) {
                av_packet_free(&detection_tasks[i].pkt);
            }
            if (detection_tasks[i].codec_params) {
                avcodec_parameters_free(&detection_tasks[i].codec_params);
            }
            detection_tasks[i].in_use = false;
        }
    }
    pthread_mutex_unlock(&detection_tasks_mutex);

    log_info("Detection thread pool shutdown");
}

/**
 * Submit a detection task to the thread pool
 */
int submit_detection_task(const char *stream_name, const AVPacket *pkt, const AVCodecParameters *codec_params) {
    if (!stream_name || !pkt || !codec_params) {
        log_error("Invalid parameters for submit_detection_task");
        return -1;
    }

    // Check if shutdown has been initiated
    if (is_shutdown_initiated()) {
        log_info("Shutdown initiated, not submitting detection task for stream %s", stream_name);
        return -1;
    }

    // Check if thread pool is running
    if (!detection_thread_pool_running) {
        log_error("Detection thread pool is not running");
        return -1;
    }

    pthread_mutex_lock(&detection_tasks_mutex);
    
    // Update statistics
    pending_tasks++;

    // Find an available thread
    int thread_id = -1;
    for (int i = 0; i < MAX_DETECTION_THREADS; i++) {
        if (!detection_tasks[i].in_use) {
            thread_id = i;
            break;
        }
    }

    // If no thread is available, return error
    if (thread_id == -1) {
        pending_tasks--;
        pthread_mutex_unlock(&detection_tasks_mutex);
        log_warn("No available detection thread, skipping detection for stream %s", stream_name);
        return -1;
    }

    // Only allocate resources after we know a thread is available
    // Create a copy of the packet
    AVPacket *pkt_copy = av_packet_alloc();
    if (!pkt_copy) {
        pending_tasks--;
        pthread_mutex_unlock(&detection_tasks_mutex);
        log_error("Failed to allocate packet for detection task");
        return -1;
    }

    if (av_packet_ref(pkt_copy, pkt) < 0) {
        av_packet_free(&pkt_copy);
        pending_tasks--;
        pthread_mutex_unlock(&detection_tasks_mutex);
        log_error("Failed to reference packet for detection task");
        return -1;
    }

    // Create a copy of the codec parameters
    AVCodecParameters *codec_params_copy = avcodec_parameters_alloc();
    if (!codec_params_copy) {
        av_packet_free(&pkt_copy);
        pending_tasks--;
        pthread_mutex_unlock(&detection_tasks_mutex);
        log_error("Failed to allocate codec parameters for detection task");
        return -1;
    }

    if (avcodec_parameters_copy(codec_params_copy, codec_params) < 0) {
        av_packet_free(&pkt_copy);
        avcodec_parameters_free(&codec_params_copy);
        pending_tasks--;
        pthread_mutex_unlock(&detection_tasks_mutex);
        log_error("Failed to copy codec parameters for detection task");
        return -1;
    }

    // Set up the task
    strncpy(detection_tasks[thread_id].stream_name, stream_name, sizeof(detection_tasks[thread_id].stream_name) - 1);
    detection_tasks[thread_id].stream_name[sizeof(detection_tasks[thread_id].stream_name) - 1] = '\0';
    detection_tasks[thread_id].pkt = pkt_copy;
    detection_tasks[thread_id].codec_params = codec_params_copy;
    detection_tasks[thread_id].use_segment_file = false;
    detection_tasks[thread_id].in_use = true;

    // Signal the thread
    pthread_cond_signal(&detection_tasks_cond);
    pthread_mutex_unlock(&detection_tasks_mutex);

    log_info("Submitted detection task for stream %s to thread %d", stream_name, thread_id);
    return 0;
}

/**
 * Submit a detection task to the thread pool using an HLS segment file
 */
int submit_segment_detection_task(const char *stream_name, const char *segment_path, 
                                 float segment_duration, time_t timestamp) {
    if (!stream_name || !segment_path) {
        log_error("Invalid parameters for submit_segment_detection_task");
        return -1;
    }

    // Check if shutdown has been initiated
    if (is_shutdown_initiated()) {
        log_info("Shutdown initiated, not submitting segment detection task for stream %s", stream_name);
        return -1;
    }

    // Check if thread pool is running
    if (!detection_thread_pool_running) {
        log_error("Detection thread pool is not running");
        return -1;
    }

    pthread_mutex_lock(&detection_tasks_mutex);
    
    // Update statistics
    pending_tasks++;

    // Find an available thread
    int thread_id = -1;
    for (int i = 0; i < MAX_DETECTION_THREADS; i++) {
        if (!detection_tasks[i].in_use) {
            thread_id = i;
            break;
        }
    }

    // If no thread is available, return error
    if (thread_id == -1) {
        pending_tasks--;
        pthread_mutex_unlock(&detection_tasks_mutex);
        log_warn("No available detection thread, skipping segment detection for stream %s", stream_name);
        return -1;
    }

    // Set up the task
    strncpy(detection_tasks[thread_id].stream_name, stream_name, sizeof(detection_tasks[thread_id].stream_name) - 1);
    detection_tasks[thread_id].stream_name[sizeof(detection_tasks[thread_id].stream_name) - 1] = '\0';
    
    strncpy(detection_tasks[thread_id].segment_path, segment_path, sizeof(detection_tasks[thread_id].segment_path) - 1);
    detection_tasks[thread_id].segment_path[sizeof(detection_tasks[thread_id].segment_path) - 1] = '\0';
    
    detection_tasks[thread_id].segment_duration = segment_duration;
    detection_tasks[thread_id].timestamp = timestamp;
    detection_tasks[thread_id].use_segment_file = true;
    detection_tasks[thread_id].pkt = NULL;
    detection_tasks[thread_id].codec_params = NULL;
    detection_tasks[thread_id].in_use = true;

    // Signal the thread
    pthread_cond_signal(&detection_tasks_cond);
    pthread_mutex_unlock(&detection_tasks_mutex);

    log_info("Submitted segment detection task for stream %s to thread %d (segment: %s)", 
             stream_name, thread_id, segment_path);
    return 0;
}

/**
 * Get the number of active detection threads
 */
int get_active_detection_threads(void) {
    pthread_mutex_lock(&detection_tasks_mutex);
    int count = active_threads;
    pthread_mutex_unlock(&detection_tasks_mutex);
    return count;
}

/**
 * Get the maximum number of detection threads
 */
int get_max_detection_threads(void) {
    return MAX_DETECTION_THREADS;
}

/**
 * Get the number of pending detection tasks
 */
int get_pending_detection_tasks(void) {
    pthread_mutex_lock(&detection_tasks_mutex);
    int count = pending_tasks;
    pthread_mutex_unlock(&detection_tasks_mutex);
    return count;
}

/**
 * Check if the detection thread pool is busy
 */
bool is_detection_thread_pool_busy(void) {
    pthread_mutex_lock(&detection_tasks_mutex);
    
    bool busy = true;
    for (int i = 0; i < MAX_DETECTION_THREADS; i++) {
        if (!detection_tasks[i].in_use) {
            busy = false;
            break;
        }
    }
    
    pthread_mutex_unlock(&detection_tasks_mutex);
    return busy;
}

/**
 * Get the detection thread pool statistics
 */
void get_detection_thread_pool_stats(int *active_threads_out, int *max_threads_out, 
                                    int *pending_tasks_out, int *completed_tasks_out) {
    pthread_mutex_lock(&detection_tasks_mutex);
    
    if (active_threads_out) {
        *active_threads_out = active_threads;
    }
    
    if (max_threads_out) {
        *max_threads_out = MAX_DETECTION_THREADS;
    }
    
    if (pending_tasks_out) {
        *pending_tasks_out = pending_tasks;
    }
    
    if (completed_tasks_out) {
        *completed_tasks_out = completed_tasks;
    }
    
    pthread_mutex_unlock(&detection_tasks_mutex);
}
