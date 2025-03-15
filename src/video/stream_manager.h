#ifndef STREAM_MANAGER_H
#define STREAM_MANAGER_H

#include <stdbool.h>
#include "core/config.h"

// Opaque handle for streams
typedef void* stream_handle_t;

// Stream status enum
typedef enum {
    STREAM_STATUS_UNKNOWN = 0,
    STREAM_STATUS_STOPPED,
    STREAM_STATUS_STARTING,
    STREAM_STATUS_RUNNING,
    STREAM_STATUS_ERROR
} stream_status_t;

/**
 * Initialize stream manager
 * 
 * @return 0 on success, non-zero on failure
 */
int init_stream_manager(void);

/**
 * Shutdown stream manager
 */
void shutdown_stream_manager(void);

/**
 * Get stream by name
 * 
 * @param name Stream name
 * @return Stream handle or NULL if not found
 */
stream_handle_t get_stream_by_name(const char *name);

/**
 * Get stream configuration
 * 
 * @param stream Stream handle
 * @param config Pointer to config structure to fill
 * @return 0 on success, non-zero on failure
 */
int get_stream_config(stream_handle_t stream, stream_config_t *config);

/**
 * Get stream status
 * 
 * @param stream Stream handle
 * @return Stream status
 */
stream_status_t get_stream_status(stream_handle_t stream);

/**
 * Set stream detection recording
 * 
 * @param stream Stream handle
 * @param enabled Whether detection-based recording is enabled
 * @param model_path Path to detection model file
 * @return 0 on success, non-zero on failure
 */
int set_stream_detection_recording(stream_handle_t stream, bool enabled, const char *model_path);

/**
 * Set stream detection parameters
 * 
 * @param stream Stream handle
 * @param interval Frames between detection checks
 * @param threshold Detection confidence threshold (0.0-1.0)
 * @param pre_buffer Seconds to keep before detection
 * @param post_buffer Seconds to keep after detection
 * @return 0 on success, non-zero on failure
 */
int set_stream_detection_params(stream_handle_t stream, int interval, float threshold, 
                               int pre_buffer, int post_buffer);

#endif /* STREAM_MANAGER_H */
