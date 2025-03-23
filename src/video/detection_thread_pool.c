#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <pthread.h>
#include <stdbool.h>

#include <libavformat/avformat.h>
#include <libavcodec/avcodec.h>
#include <libavutil/avutil.h>

#include "core/logger.h"
#include "video/detection_thread_pool.h"
#include "video/hls_writer.h"

// Thread pool state
static pthread_t detection_threads[MAX_DETECTION_THREADS];
static detection_task_t detection_tasks[MAX_DETECTION_THREADS];
static pthread_mutex_t detection_tasks_mutex = PTHREAD_MUTEX_INITIALIZER;
static pthread_cond_t detection_tasks_cond = PTHREAD_COND_INITIALIZER;
static bool detection_thread_pool_running = false;

// Forward declaration of the detection function from hls_writer.c
extern void process_packet_for_detection(const char *stream_name, const AVPacket *pkt, const AVCodecParameters *codec_params);

/**
 * Detection thread function
 */
static void *detection_thread_func(void *arg) {
    int thread_id = *((int *)arg);
    free(arg); // Free the thread ID memory

    log_info("Detection thread %d started", thread_id);

    while (detection_thread_pool_running) {
        // Wait for a task
        pthread_mutex_lock(&detection_tasks_mutex);
        while (detection_thread_pool_running && !detection_tasks[thread_id].in_use) {
            pthread_cond_wait(&detection_tasks_cond, &detection_tasks_mutex);
        }

        // Check if we're shutting down
        if (!detection_thread_pool_running) {
            pthread_mutex_unlock(&detection_tasks_mutex);
            break;
        }

        // Get task data
        char stream_name[256];
        strncpy(stream_name, detection_tasks[thread_id].stream_name, sizeof(stream_name) - 1);
        stream_name[sizeof(stream_name) - 1] = '\0';

        AVPacket *pkt = detection_tasks[thread_id].pkt;
        AVCodecParameters *codec_params = detection_tasks[thread_id].codec_params;

        // Mark task as not in use
        detection_tasks[thread_id].in_use = false;
        pthread_mutex_unlock(&detection_tasks_mutex);

        // Process the detection task
        log_info("Detection thread %d processing task for stream %s", thread_id, stream_name);
        process_packet_for_detection(stream_name, pkt, codec_params);

        // Free the packet and codec parameters
        av_packet_free(&pkt);
        avcodec_parameters_free(&codec_params);

        log_info("Detection thread %d completed task for stream %s", thread_id, stream_name);
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

    // Check if thread pool is running
    if (!detection_thread_pool_running) {
        log_error("Detection thread pool is not running");
        return -1;
    }

    pthread_mutex_lock(&detection_tasks_mutex);

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
        pthread_mutex_unlock(&detection_tasks_mutex);
        log_warn("No available detection thread, skipping detection for stream %s", stream_name);
        return -1;
    }

    // Create a copy of the packet
    AVPacket *pkt_copy = av_packet_alloc();
    if (!pkt_copy) {
        pthread_mutex_unlock(&detection_tasks_mutex);
        log_error("Failed to allocate packet for detection task");
        return -1;
    }

    if (av_packet_ref(pkt_copy, pkt) < 0) {
        av_packet_free(&pkt_copy);
        pthread_mutex_unlock(&detection_tasks_mutex);
        log_error("Failed to reference packet for detection task");
        return -1;
    }

    // Create a copy of the codec parameters
    AVCodecParameters *codec_params_copy = avcodec_parameters_alloc();
    if (!codec_params_copy) {
        av_packet_free(&pkt_copy);
        pthread_mutex_unlock(&detection_tasks_mutex);
        log_error("Failed to allocate codec parameters for detection task");
        return -1;
    }

    if (avcodec_parameters_copy(codec_params_copy, codec_params) < 0) {
        av_packet_free(&pkt_copy);
        avcodec_parameters_free(&codec_params_copy);
        pthread_mutex_unlock(&detection_tasks_mutex);
        log_error("Failed to copy codec parameters for detection task");
        return -1;
    }

    // Set up the task
    strncpy(detection_tasks[thread_id].stream_name, stream_name, sizeof(detection_tasks[thread_id].stream_name) - 1);
    detection_tasks[thread_id].stream_name[sizeof(detection_tasks[thread_id].stream_name) - 1] = '\0';
    detection_tasks[thread_id].pkt = pkt_copy;
    detection_tasks[thread_id].codec_params = codec_params_copy;
    detection_tasks[thread_id].in_use = true;

    // Signal the thread
    pthread_cond_signal(&detection_tasks_cond);
    pthread_mutex_unlock(&detection_tasks_mutex);

    log_info("Submitted detection task for stream %s to thread %d", stream_name, thread_id);
    return 0;
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
