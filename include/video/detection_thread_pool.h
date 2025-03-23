#ifndef DETECTION_THREAD_POOL_H
#define DETECTION_THREAD_POOL_H

#include <pthread.h>
#include <stdbool.h>
#include <libavcodec/avcodec.h>

// Maximum number of threads in the detection thread pool
#define MAX_DETECTION_THREADS 2

// Detection task structure
typedef struct {
    char stream_name[256];
    AVPacket *pkt;
    AVCodecParameters *codec_params;
    bool in_use;
} detection_task_t;

/**
 * Initialize the detection thread pool
 * 
 * @return 0 on success, -1 on error
 */
int init_detection_thread_pool(void);

/**
 * Shutdown the detection thread pool
 */
void shutdown_detection_thread_pool(void);

/**
 * Submit a detection task to the thread pool
 * 
 * @param stream_name The name of the stream
 * @param pkt The packet to process
 * @param codec_params The codec parameters
 * @return 0 on success, -1 on error
 */
int submit_detection_task(const char *stream_name, const AVPacket *pkt, const AVCodecParameters *codec_params);

/**
 * Check if the detection thread pool is busy
 * 
 * @return true if all threads are busy, false otherwise
 */
bool is_detection_thread_pool_busy(void);

#endif /* DETECTION_THREAD_POOL_H */
