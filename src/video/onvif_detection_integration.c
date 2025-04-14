#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

#include "core/logger.h"
#include "core/config.h"
#include "core/shutdown_coordinator.h"
#include "video/onvif_detection.h"
#include "video/onvif_detection_integration.h"
#include "video/detection_result.h"
#include "video/detection_model.h"
#include "database/db_streams.h"

/**
 * Initialize ONVIF detection integration
 */
int init_onvif_detection_integration(void) {
    // Initialize the ONVIF detection system
    int ret = init_onvif_detection_system();
    if (ret != 0) {
        log_error("Failed to initialize ONVIF detection system");
        return ret;
    }

    log_info("ONVIF detection integration initialized successfully");
    return 0;
}

/**
 * Cleanup function to be called at shutdown
 */
void cleanup_onvif_detection_integration(void) {
    // Shutdown the ONVIF detection system
    shutdown_onvif_detection_system();
    
    log_info("ONVIF detection integration cleaned up");
}
