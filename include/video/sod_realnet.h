#ifndef SOD_REALNET_H
#define SOD_REALNET_H

#include "video/detection_result.h"

/**
 * Load a SOD RealNet model
 * 
 * @param model_path Path to the RealNet model file (.realnet.sod)
 * @param threshold Detection confidence threshold (typically 5.0 for RealNet)
 * @return Model handle or NULL on failure
 */
void* load_sod_realnet_model(const char *model_path, float threshold);

/**
 * Free a SOD RealNet model
 * 
 * @param model SOD RealNet model handle
 */
void free_sod_realnet_model(void *model);

/**
 * Run detection on a frame using SOD RealNet
 * 
 * @param model SOD RealNet model handle
 * @param frame_data Frame data (grayscale for RealNet)
 * @param width Frame width
 * @param height Frame height
 * @param channels Number of color channels (1 for grayscale)
 * @param result Pointer to detection result structure to fill
 * @return 0 on success, non-zero on failure
 */
int detect_with_sod_realnet(void *model, const unsigned char *frame_data, 
                           int width, int height, int channels, detection_result_t *result);

#endif /* SOD_REALNET_H */
