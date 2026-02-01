#ifndef LIGHTNVR_MQTT_CLIENT_H
#define LIGHTNVR_MQTT_CLIENT_H

#include <stdbool.h>
#include <time.h>
#include "core/config.h"
#include "video/detection_result.h"

#ifdef ENABLE_MQTT

/**
 * Initialize the MQTT client
 * Must be called before any other MQTT functions
 * 
 * @param config Pointer to the application configuration
 * @return 0 on success, -1 on failure
 */
int mqtt_init(const config_t *config);

/**
 * Connect to the MQTT broker
 * Uses settings from the config passed to mqtt_init()
 * Handles reconnection automatically via the loop thread
 * 
 * @return 0 on success, -1 on failure
 */
int mqtt_connect(void);

/**
 * Check if MQTT client is connected
 * 
 * @return true if connected, false otherwise
 */
bool mqtt_is_connected(void);

/**
 * Publish a detection event to MQTT
 * Topic format: {topic_prefix}/detections/{stream_name}
 * Payload: JSON with detection data
 * 
 * @param stream_name Name of the stream where detection occurred
 * @param result Detection results to publish
 * @param timestamp Timestamp of the detection
 * @return 0 on success, -1 on failure
 */
int mqtt_publish_detection(const char *stream_name, const detection_result_t *result, time_t timestamp);

/**
 * Publish a raw message to a custom topic
 * 
 * @param topic Full topic path (topic_prefix is NOT automatically prepended)
 * @param payload Message payload (null-terminated string)
 * @param retain Whether to set the retain flag
 * @return 0 on success, -1 on failure
 */
int mqtt_publish_raw(const char *topic, const char *payload, bool retain);

/**
 * Disconnect from the MQTT broker gracefully
 */
void mqtt_disconnect(void);

/**
 * Cleanup MQTT resources
 * Should be called when shutting down the application
 */
void mqtt_cleanup(void);

#else /* ENABLE_MQTT not defined */

/* Stub implementations when MQTT is disabled */
static inline int mqtt_init(const config_t *config) { (void)config; return 0; }
static inline int mqtt_connect(void) { return 0; }
static inline bool mqtt_is_connected(void) { return false; }
static inline int mqtt_publish_detection(const char *stream_name, const detection_result_t *result, time_t timestamp) {
    (void)stream_name; (void)result; (void)timestamp; return 0;
}
static inline int mqtt_publish_raw(const char *topic, const char *payload, bool retain) {
    (void)topic; (void)payload; (void)retain; return 0;
}
static inline void mqtt_disconnect(void) {}
static inline void mqtt_cleanup(void) {}

#endif /* ENABLE_MQTT */

#endif /* LIGHTNVR_MQTT_CLIENT_H */

