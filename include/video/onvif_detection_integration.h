#ifndef ONVIF_DETECTION_INTEGRATION_H
#define ONVIF_DETECTION_INTEGRATION_H

/**
 * Initialize ONVIF detection integration
 * This function should be called during system initialization
 * 
 * @return 0 on success, non-zero on failure
 */
int init_onvif_detection_integration(void);

/**
 * Cleanup ONVIF detection integration
 * This function should be called during system shutdown
 */
void cleanup_onvif_detection_integration(void);

#endif /* ONVIF_DETECTION_INTEGRATION_H */
