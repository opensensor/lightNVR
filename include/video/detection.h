#ifndef DETECTION_H
#define DETECTION_H

#include <stdbool.h>
#include "video/detection_result.h"
#include "video/detection_model.h"
#include "video/sod_detection.h"
#include "video/onvif_detection.h"

/**
 * Initialize the detection system
 * 
 * @return 0 on success, non-zero on failure
 */
int init_detection_system(void);

/**
 * Shutdown the detection system
 */
void shutdown_detection_system(void);

/**
 * Run detection on a frame
 * 
 * @param model Detection model handle
 * @param frame_data Frame data (format depends on model)
 * @param width Frame width
 * @param height Frame height
 * @param channels Number of color channels (1 for grayscale, 3 for RGB)
 * @param result Pointer to detection result structure to fill
 * @return 0 on success, non-zero on failure
 */
int detect_objects(detection_model_t model, const unsigned char *frame_data, 
                  int width, int height, int channels, detection_result_t *result);

#endif /* DETECTION_H */
