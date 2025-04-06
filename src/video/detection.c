#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <dlfcn.h>
#include <pthread.h>
#include <unistd.h>
#include <sys/stat.h>
#include <signal.h>
#include <setjmp.h>
#include <errno.h>

// FFmpeg headers
#include <libavcodec/avcodec.h>
#include <libavformat/avformat.h>
#include <libavutil/imgutils.h>
#include <libavutil/opt.h>
#include <libswscale/swscale.h>

#include "../../include/video/detection.h"
#include "../../include/video/detection_model.h"
#include "../../include/video/sod_detection.h"
#include "../../include/video/sod_realnet.h"
#include "../../include/video/motion_detection.h"
#include "../../include/core/logger.h"
#include "../../include/core/config.h"  // For MAX_PATH_LENGTH

// Global variables for timeout handling in video detection
static jmp_buf video_timeout_jmp_buf;
static volatile sig_atomic_t video_timeout_triggered = 0;
static pthread_mutex_t video_timeout_mutex = PTHREAD_MUTEX_INITIALIZER;
static pthread_t video_timeout_owner_thread = 0;

// Signal handler for video detection timeouts
static void video_timeout_handler(int sig) {
    // Only handle the signal if we're the owner thread
    if (pthread_self() == video_timeout_owner_thread) {
        video_timeout_triggered = 1;
        longjmp(video_timeout_jmp_buf, 1);
    }
}

/**
 * Initialize the detection system
 */
int init_detection_system(void) {
    // Initialize the model system
    int model_ret = init_detection_model_system();
    if (model_ret != 0) {
        log_error("Failed to initialize detection model system");
        return model_ret;
    }

    // Initialize motion detection system
    int motion_ret = init_motion_detection_system();
    if (motion_ret != 0) {
        log_error("Failed to initialize motion detection system");
    } else {
        log_info("Motion detection system initialized");
    }

    log_info("Detection system initialized");
    return 0;
}

/**
 * Shutdown the detection system
 */
void shutdown_detection_system(void) {
    // Shutdown the model system
    shutdown_detection_model_system();

    // Shutdown motion detection system
    shutdown_motion_detection_system();

    log_info("Detection system shutdown");
}

/**
 * Set up a timeout for video operations
 *
 * @param seconds The number of seconds for the timeout
 * @return 0 on success, -1 on failure
 */
int setup_video_timeout(int seconds) {
    // Lock the mutex to prevent race conditions
    pthread_mutex_lock(&video_timeout_mutex);

    // Store the current thread ID as the owner of the alarm
    video_timeout_owner_thread = pthread_self();

    // Reset the timeout_triggered flag
    video_timeout_triggered = 0;

    // Set up the signal handler
    struct sigaction sa, old_sa;
    memset(&sa, 0, sizeof(sa));
    sa.sa_handler = video_timeout_handler;
    sa.sa_flags = 0; // Don't use SA_RESTART to ensure longjmp works properly
    sigaction(SIGALRM, &sa, &old_sa);

    // Set the alarm
    alarm(seconds);

    // Unlock the mutex
    pthread_mutex_unlock(&video_timeout_mutex);

    return 0;
}

/**
 * Clear the video timeout
 *
 * @return 0 on success, -1 on failure
 */
int clear_video_timeout(void) {
    // Lock the mutex to prevent race conditions
    pthread_mutex_lock(&video_timeout_mutex);

    // Clear the alarm
    alarm(0);

    // Reset the timeout_triggered flag
    video_timeout_triggered = 0;

    // Restore the original signal handler
    struct sigaction sa;
    memset(&sa, 0, sizeof(sa));
    sa.sa_handler = SIG_DFL;
    sigaction(SIGALRM, &sa, NULL);

    // Clear the owner thread
    video_timeout_owner_thread = 0;

    // Unlock the mutex
    pthread_mutex_unlock(&video_timeout_mutex);

    return 0;
}

/**
 * Handle HLS timeouts safely
 *
 * @param url The URL of the stream that timed out
 * @param input_ctx Pointer to the AVFormatContext to clean up
 * @return AVERROR(ETIMEDOUT) to indicate timeout
 */
int handle_hls_timeout(const char *url, AVFormatContext **input_ctx) {
    log_warn("HLS timeout occurred for stream: %s", url ? url : "unknown");

    // Safely clean up resources
    if (input_ctx && *input_ctx) {
        // Flush all buffers before closing if possible
        if ((*input_ctx)->pb && !(*input_ctx)->pb->error) {
            avio_flush((*input_ctx)->pb);
        }

        // Close the input context - this will free all associated resources
        avformat_close_input(input_ctx);

        // Double check that it's really closed
        if (*input_ctx) {
            log_warn("Input context still exists after avformat_close_input, forcing cleanup");
            avformat_free_context(*input_ctx);
            *input_ctx = NULL;
        }
    }

    return AVERROR(ETIMEDOUT);
}

/**
 * Run detection on a frame
 */
int detect_objects(detection_model_t model, const unsigned char *frame_data,
                  int width, int height, int channels, detection_result_t *result) {
    if (!model || !frame_data || !result) {
        log_error("Invalid parameters for detect_objects");
        return -1;
    }

    // Initialize result
    result->count = 0;

    // Get the model type directly from the model handle
    const char *model_type = get_model_type_from_handle(model);
    log_info("Detecting objects using model type: %s (dimensions: %dx%d, channels: %d)",
             model_type, width, height, channels);

    int ret = -1;

    // Delegate to the appropriate detection function based on model type
    if (strcmp(model_type, MODEL_TYPE_SOD) == 0) {
        ret = detect_with_sod_model(model, frame_data, width, height, channels, result);
    }
    else if (strcmp(model_type, MODEL_TYPE_SOD_REALNET) == 0) {
        // For RealNet models, we need to extract the internal model handle
        void *realnet_model = get_realnet_model_handle(model);
        if (!realnet_model) {
            log_error("Failed to get RealNet model handle");
            ret = -1;
        } else {
            ret = detect_with_sod_realnet(realnet_model, frame_data, width, height, channels, result);
        }
    }
    else if (strcmp(model_type, MODEL_TYPE_TFLITE) == 0) {
        log_error("TFLite detection not implemented yet");
        ret = -1;
    }
    else {
        log_error("Unknown model type: %s", model_type);
        ret = -1;
    }

    return ret;
}
