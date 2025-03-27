/**
 * Header file for HLS writer thread implementation
 */

#ifndef HLS_WRITER_THREAD_H
#define HLS_WRITER_THREAD_H

#include <pthread.h>
#include <stdatomic.h>
#include "video/hls_writer.h"

// Forward declaration of the HLS writer thread context
typedef struct hls_writer_thread_ctx hls_writer_thread_ctx_t;

/**
 * HLS writer thread context structure
 */
struct hls_writer_thread_ctx {
    char rtsp_url[MAX_PATH_LENGTH];
    char stream_name[MAX_STREAM_NAME];
    hls_writer_t *writer;
    pthread_t thread;
    atomic_int running;
    int shutdown_component_id;
    int protocol; // STREAM_PROTOCOL_TCP or STREAM_PROTOCOL_UDP
};

/**
 * Start a recording thread that reads from the RTSP stream and writes to the HLS files
 * This function creates a new thread that handles all the recording logic
 *
 * @param writer The HLS writer instance
 * @param rtsp_url The URL of the RTSP stream to record
 * @param stream_name The name of the stream
 * @param protocol The protocol to use (STREAM_PROTOCOL_TCP or STREAM_PROTOCOL_UDP)
 * @return 0 on success, negative on error
 */
int hls_writer_start_recording_thread(hls_writer_t *writer, const char *rtsp_url, const char *stream_name, int protocol);

/**
 * Stop the recording thread
 * This function signals the recording thread to stop and waits for it to exit
 *
 * @param writer The HLS writer instance
 */
void hls_writer_stop_recording_thread(hls_writer_t *writer);

/**
 * Check if the recording thread is running
 *
 * @param writer The HLS writer instance
 * @return 1 if running, 0 if not
 */
int hls_writer_is_recording(hls_writer_t *writer);

#endif /* HLS_WRITER_THREAD_H */
