#ifndef LIGHTNVR_STREAM_STATE_ADAPTER_H
#define LIGHTNVR_STREAM_STATE_ADAPTER_H

#include "video/stream_manager.h"
#include "video/stream_state.h"

/**
 * Initialize the stream state adapter
 * This function sets up the adapter between the old stream manager API and the new state manager
 * 
 * @return 0 on success, non-zero on failure
 */
int init_stream_state_adapter(void);

/**
 * Shutdown the stream state adapter
 */
void shutdown_stream_state_adapter(void);

/**
 * Convert stream_handle_t to stream_state_manager_t
 * 
 * @param handle Stream handle from the old API
 * @return Pointer to stream state manager, or NULL if not found
 */
stream_state_manager_t *stream_handle_to_state(stream_handle_t handle);

/**
 * Convert stream_state_manager_t to stream_handle_t
 * 
 * @param state Stream state manager
 * @return Stream handle for the old API, or NULL if not found
 */
stream_handle_t stream_state_to_handle(stream_state_manager_t *state);

/**
 * Adapter function for add_stream
 * 
 * @param config Stream configuration
 * @return Stream handle on success, NULL on failure
 */
stream_handle_t add_stream_adapter(const stream_config_t *config);

/**
 * Adapter function for remove_stream
 * 
 * @param handle Stream handle
 * @return 0 on success, non-zero on failure
 */
int remove_stream_adapter(stream_handle_t handle);

/**
 * Adapter function for start_stream
 * 
 * @param handle Stream handle
 * @return 0 on success, non-zero on failure
 */
int start_stream_adapter(stream_handle_t handle);

/**
 * Adapter function for stop_stream
 * 
 * @param handle Stream handle
 * @return 0 on success, non-zero on failure
 */
int stop_stream_adapter(stream_handle_t handle);

/**
 * Adapter function for get_stream_status
 * 
 * @param handle Stream handle
 * @return Stream status
 */
stream_status_t get_stream_status_adapter(stream_handle_t handle);

/**
 * Adapter function for get_stream_stats
 * 
 * @param handle Stream handle
 * @param stats Pointer to statistics structure to fill
 * @return 0 on success, non-zero on failure
 */
int get_stream_stats_adapter(stream_handle_t handle, stream_stats_t *stats);

/**
 * Adapter function for get_stream_config
 * 
 * @param handle Stream handle
 * @param config Pointer to configuration structure to fill
 * @return 0 on success, non-zero on failure
 */
int get_stream_config_adapter(stream_handle_t handle, stream_config_t *config);

/**
 * Adapter function for set_stream_priority
 * 
 * @param handle Stream handle
 * @param priority Priority level (1-10, higher = more important)
 * @return 0 on success, non-zero on failure
 */
int set_stream_priority_adapter(stream_handle_t handle, int priority);

/**
 * Adapter function for set_stream_recording
 * 
 * @param handle Stream handle
 * @param enable True to enable recording, false to disable
 * @return 0 on success, non-zero on failure
 */
int set_stream_recording_adapter(stream_handle_t handle, bool enable);

/**
 * Adapter function for set_stream_detection_recording
 * 
 * @param handle Stream handle
 * @param enable True to enable detection-based recording, false to disable
 * @param model_path Path to the detection model file
 * @return 0 on success, non-zero on failure
 */
int set_stream_detection_recording_adapter(stream_handle_t handle, bool enable, const char *model_path);

/**
 * Adapter function for set_stream_detection_params
 * 
 * @param handle Stream handle
 * @param interval Number of frames between detection checks
 * @param threshold Confidence threshold for detection (0.0-1.0)
 * @param pre_buffer Seconds to keep before detection
 * @param post_buffer Seconds to keep after detection
 * @return 0 on success, non-zero on failure
 */
int set_stream_detection_params_adapter(stream_handle_t handle, int interval, float threshold, 
                                       int pre_buffer, int post_buffer);

/**
 * Adapter function for set_stream_streaming_enabled
 * 
 * @param handle Stream handle
 * @param enabled True to enable streaming, false to disable
 * @return 0 on success, non-zero on failure
 */
int set_stream_streaming_enabled_adapter(stream_handle_t handle, bool enabled);

/**
 * Adapter function for get_stream_by_name
 * 
 * @param name Stream name
 * @return Stream handle on success, NULL if not found
 */
stream_handle_t get_stream_by_name_adapter(const char *name);

/**
 * Adapter function for get_stream_by_index
 * 
 * @param index Stream index (0-based)
 * @return Stream handle on success, NULL if index is out of range
 */
stream_handle_t get_stream_by_index_adapter(int index);

/**
 * Adapter function for get_active_stream_count
 * 
 * @return Number of active streams
 */
int get_active_stream_count_adapter(void);

/**
 * Adapter function for get_total_stream_count
 * 
 * @return Total number of streams
 */
int get_total_stream_count_adapter(void);

#endif // LIGHTNVR_STREAM_STATE_ADAPTER_H
