#ifndef LIGHTNVR_ONVIF_DETECTION_H
#define LIGHTNVR_ONVIF_DETECTION_H

#include "video/detection_result.h"

// Model type for ONVIF-based detection
#define MODEL_TYPE_ONVIF "onvif"

/* Bounded same-subscription retry for camera-side transport drops (a
 * connection the camera accepts and closes without any HTTP response). See
 * onvif_detection.c for the Tapo firmware behaviour this covers (#567, #603). */
#define ONVIF_PULL_DROP_MAX_ATTEMPTS 6
#define ONVIF_PULL_DROP_RETRY_DELAY_MS 250

/**
 * Initialize the ONVIF detection system
 * 
 * @return 0 on success, non-zero on failure
 */
int init_onvif_detection_system(void);

/**
 * Shutdown the ONVIF detection system
 */
void shutdown_onvif_detection_system(void);

/**
 * Detect motion events using ONVIF
 * 
 * @param onvif_url The URL of the ONVIF camera
 * @param username The username for authentication
 * @param password The password for authentication
 * @param result Pointer to a detection_result_t structure to store the results
 * @param stream_name The name of the stream (for database storage)
 * @return 0 on success, non-zero on failure
 */
int detect_motion_onvif(const char *onvif_url, const char *username, const char *password,
                       detection_result_t *result, const char *stream_name);

#endif /* LIGHTNVR_ONVIF_DETECTION_H */
