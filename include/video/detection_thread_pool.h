/**
 * Detection Thread Pool
 * 
 * This module provides a thread pool for processing detection events asynchronously.
 * It allows for configuring the number of detection threads based on system capabilities.
 * The detection threads read from HLS segments on disk to generate inputs for detection models.
 */

#ifndef DETECTION_THREAD_POOL_H
#define DETECTION_THREAD_POOL_H

#include <pthread.h>
#include <stdbool.h>
#include <stdint.h>
#include <time.h>
#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include "video/streams.h"

// Maximum number of detection threads
#define MAX_DETECTION_THREADS 4

// Detection task structure
typedef struct {
    char stream_name[MAX_STREAM_NAME];
    AVPacket *pkt;
    AVCodecParameters *codec_params;
    bool in_use;
    
    // New fields for HLS segment-based detection
    char segment_path[MAX_PATH_LENGTH];
    float segment_duration;
    time_t timestamp;
    bool use_segment_file;
} detection_task_t;

/**
 * Initialize the detection thread pool
 * This should be called at startup
 * 
 * @return 0 on success, non-zero on failure
 */
int init_detection_thread_pool(void);

/**
 * Shutdown the detection thread pool
 * This should be called at shutdown
 */
void shutdown_detection_thread_pool(void);

/**
 * Submit a detection task to the thread pool using a packet
 * This is the original method used by the HLS streaming threads
 * 
 * @param stream_name The name of the stream
 * @param pkt The packet to process
 * @param codec_params The codec parameters
 * @return 0 on success, non-zero on failure
 */
int submit_detection_task(const char *stream_name, const AVPacket *pkt, const AVCodecParameters *codec_params);

/**
 * Submit a detection task to the thread pool using an HLS segment file
 * This is the new method that reads from HLS files on disk
 * 
 * @param stream_name The name of the stream
 * @param segment_path The path to the HLS segment file
 * @param segment_duration The duration of the segment in seconds
 * @param timestamp The timestamp of the segment
 * @return 0 on success, non-zero on failure
 */
int submit_segment_detection_task(const char *stream_name, const char *segment_path, 
                                 float segment_duration, time_t timestamp);

/**
 * Get the number of active detection threads
 * 
 * @return Number of active detection threads
 */
int get_active_detection_threads(void);

/**
 * Get the maximum number of detection threads
 * 
 * @return Maximum number of detection threads
 */
int get_max_detection_threads(void);

/**
 * Get the number of pending detection tasks
 * 
 * @return Number of pending detection tasks
 */
int get_pending_detection_tasks(void);

/**
 * Check if the detection thread pool is busy
 * 
 * @return true if all threads are busy, false otherwise
 */
bool is_detection_thread_pool_busy(void);

/**
 * Get the detection thread pool statistics
 * 
 * @param active_threads Pointer to store the number of active threads
 * @param max_threads Pointer to store the maximum number of threads
 * @param pending_tasks Pointer to store the number of pending tasks
 * @param completed_tasks Pointer to store the number of completed tasks
 */
void get_detection_thread_pool_stats(int *active_threads, int *max_threads, 
                                    int *pending_tasks, int *completed_tasks);

#endif /* DETECTION_THREAD_POOL_H */
