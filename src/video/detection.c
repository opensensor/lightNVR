#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <dlfcn.h>
#include <pthread.h>
#include <unistd.h>
#include <sys/stat.h>

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

    // Delegate to the appropriate detection function based on model type
    if (strcmp(model_type, MODEL_TYPE_SOD) == 0) {
        return detect_with_sod_model(model, frame_data, width, height, channels, result);
    }
    else if (strcmp(model_type, MODEL_TYPE_SOD_REALNET) == 0) {
        // For RealNet models, we need to extract the internal model handle
        void *realnet_model = get_realnet_model_handle(model);
        if (!realnet_model) {
            log_error("Failed to get RealNet model handle");
            return -1;
        }
        return detect_with_sod_realnet(realnet_model, frame_data, width, height, channels, result);
    }
    else if (strcmp(model_type, MODEL_TYPE_TFLITE) == 0) {
        log_error("TFLite detection not implemented yet");
        return -1;
    }
    else {
        log_error("Unknown model type: %s", model_type);
        return -1;
    }
}
