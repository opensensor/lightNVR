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
 * Publish binary data to a topic (e.g., JPEG snapshots)
 *
 * @param topic Full topic path
 * @param data Binary data buffer
 * @param len Length of data in bytes
 * @param retain Whether to set the retain flag
 * @return 0 on success, -1 on failure
 */
int mqtt_publish_binary(const char *topic, const void *data, size_t len, bool retain);

/**
 * Publish Home Assistant MQTT discovery messages for all configured streams.
 * Publishes camera, binary_sensor (motion), and sensor (object counts) entities.
 * Called on connect and when stream configuration changes.
 *
 * @return 0 on success, -1 on failure
 */
int mqtt_publish_ha_discovery(void);

/**
 * Update the motion state for a camera stream.
 * Publishes ON to the motion topic when detection occurs.
 * After a timeout with no new detections, publishes OFF.
 *
 * @param stream_name Name of the stream
 * @param result Detection results (NULL to force OFF)
 */
void mqtt_set_motion_state(const char *stream_name, const detection_result_t *result);

/**
 * Start Home Assistant services (snapshot timer, motion timeout checker).
 * Should be called after MQTT is connected and HA discovery is published.
 *
 * @return 0 on success, -1 on failure
 */
int mqtt_start_ha_services(void);

/**
 * Stop Home Assistant services.
 * Should be called before MQTT cleanup.
 */
void mqtt_stop_ha_services(void);

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
static inline int mqtt_publish_binary(const char *topic, const void *data, size_t len, bool retain) {
    (void)topic; (void)data; (void)len; (void)retain; return 0;
}
static inline int mqtt_publish_ha_discovery(void) { return 0; }
static inline void mqtt_set_motion_state(const char *stream_name, const detection_result_t *result) {
    (void)stream_name; (void)result;
}
static inline int mqtt_start_ha_services(void) { return 0; }
static inline void mqtt_stop_ha_services(void) {}
static inline void mqtt_disconnect(void) {}
static inline void mqtt_cleanup(void) {}

#endif /* ENABLE_MQTT */

#endif /* LIGHTNVR_MQTT_CLIENT_H */

