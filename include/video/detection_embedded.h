#ifndef DETECTION_EMBEDDED_H
#define DETECTION_EMBEDDED_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

/**
 * Check if we're running on an embedded device
 * 
 * @return true if running on an embedded device, false otherwise
 */
bool is_embedded_device(void);

/**
 * Get the appropriate downscale factor based on model type and device
 * 
 * @param model_type The model type (MODEL_TYPE_SOD, MODEL_TYPE_SOD_REALNET, etc.)
 * @return The downscale factor
 */
int get_downscale_factor(const char *model_type);

/**
 * Get the appropriate detection threshold based on model type and device
 * 
 * @param model_type The model type (MODEL_TYPE_SOD, MODEL_TYPE_SOD_REALNET, etc.)
 * @param configured_threshold The threshold configured by the user (0.0 for default)
 * @return The detection threshold
 */
float get_detection_threshold(const char *model_type, float configured_threshold);

/**
 * Downscale a frame for detection
 * 
 * @param src_data Source frame data
 * @param src_width Source frame width
 * @param src_height Source frame height
 * @param src_channels Source frame channels
 * @param dst_data Destination frame data (must be pre-allocated)
 * @param dst_width Destination frame width
 * @param dst_height Destination frame height
 * @param dst_channels Destination frame channels
 * @return 0 on success, non-zero on failure
 */
int downscale_frame(const uint8_t *src_data, int src_width, int src_height, int src_channels,
                   uint8_t *dst_data, int dst_width, int dst_height, int dst_channels);

/**
 * Calculate the memory requirements for detection
 * 
 * @param width Frame width
 * @param height Frame height
 * @param channels Frame channels
 * @param model_type The model type (MODEL_TYPE_SOD, MODEL_TYPE_SOD_REALNET, etc.)
 * @return The memory requirements in bytes
 */
size_t calculate_detection_memory_requirements(int width, int height, int channels, const char *model_type);

#endif /* DETECTION_EMBEDDED_H */
