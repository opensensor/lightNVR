#ifdef ENABLE_MQTT

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <errno.h>
#include <pthread.h>
#include <signal.h>
#include <unistd.h>
#include <time.h>
#include <mosquitto.h>
#include <cjson/cJSON.h>

#include "core/mqtt_client.h"
#include "core/logger.h"
#include "core/version.h"
#include "database/db_streams.h"
#include "video/go2rtc/go2rtc_snapshot.h"

// MQTT client state
static struct mosquitto *mosq = NULL;
static const config_t *mqtt_config = NULL;
static bool connected = false;
static volatile bool shutting_down = false;  // Flag to prevent callbacks from acquiring mutex during shutdown
static pthread_mutex_t mqtt_mutex = PTHREAD_MUTEX_INITIALIZER;

// HA discovery state
static volatile bool ha_services_running = false;
static pthread_t ha_snapshot_thread;
static pthread_t ha_motion_thread;

// Motion state tracking per stream
#define MAX_MOTION_STREAMS 16
#define MOTION_OFF_DELAY_SEC 30

typedef struct {
    char stream_name[256];
    time_t last_detection_time;
    bool motion_active;
    int object_counts[32];       // Count per object class
    char object_labels[32][32];  // Label names
    int num_labels;
} motion_state_t;

static motion_state_t motion_states[MAX_MOTION_STREAMS];
static int num_motion_states = 0;
static pthread_mutex_t motion_mutex = PTHREAD_MUTEX_INITIALIZER;

// Forward declarations for callbacks
static void on_connect(struct mosquitto *mosq, void *userdata, int rc);
static void on_disconnect(struct mosquitto *mosq, void *userdata, int rc);
static void on_log(struct mosquitto *mosq, void *userdata, int level, const char *str);

/**
 * Initialize the MQTT client
 */
int mqtt_init(const config_t *config) {
    if (!config) {
        log_error("MQTT: Invalid config pointer");
        return -1;
    }
    
    if (!config->mqtt_enabled) {
        log_info("MQTT: Disabled in configuration");
        return 0;
    }
    
    if (config->mqtt_broker_host[0] == '\0') {
        log_error("MQTT: Broker host not configured");
        return -1;
    }
    
    pthread_mutex_lock(&mqtt_mutex);
    
    // Initialize mosquitto library
    int rc = mosquitto_lib_init();
    if (rc != MOSQ_ERR_SUCCESS) {
        log_error("MQTT: Failed to initialize mosquitto library: %s", mosquitto_strerror(rc));
        pthread_mutex_unlock(&mqtt_mutex);
        return -1;
    }
    
    // Store config reference
    mqtt_config = config;
    
    // Create mosquitto client instance
    mosq = mosquitto_new(config->mqtt_client_id, true, NULL);
    if (!mosq) {
        log_error("MQTT: Failed to create mosquitto client");
        mosquitto_lib_cleanup();
        pthread_mutex_unlock(&mqtt_mutex);
        return -1;
    }
    
    // Set callbacks
    mosquitto_connect_callback_set(mosq, on_connect);
    mosquitto_disconnect_callback_set(mosq, on_disconnect);
    mosquitto_log_callback_set(mosq, on_log);
    
    // Set username/password if configured
    if (config->mqtt_username[0] != '\0') {
        rc = mosquitto_username_pw_set(mosq, config->mqtt_username, 
                                       config->mqtt_password[0] != '\0' ? config->mqtt_password : NULL);
        if (rc != MOSQ_ERR_SUCCESS) {
            log_error("MQTT: Failed to set credentials: %s", mosquitto_strerror(rc));
            mosquitto_destroy(mosq);
            mosq = NULL;
            mosquitto_lib_cleanup();
            pthread_mutex_unlock(&mqtt_mutex);
            return -1;
        }
    }
    
    // Enable TLS if configured
    if (config->mqtt_tls_enabled) {
        rc = mosquitto_tls_set(mosq, NULL, NULL, NULL, NULL, NULL);
        if (rc != MOSQ_ERR_SUCCESS) {
            log_error("MQTT: Failed to enable TLS: %s", mosquitto_strerror(rc));
            mosquitto_destroy(mosq);
            mosq = NULL;
            mosquitto_lib_cleanup();
            pthread_mutex_unlock(&mqtt_mutex);
            return -1;
        }
    }

    // Set up Last Will and Testament for HA availability tracking
    if (config->mqtt_ha_discovery) {
        char lwt_topic[512];
        snprintf(lwt_topic, sizeof(lwt_topic), "%s/availability", config->mqtt_topic_prefix);
        rc = mosquitto_will_set(mosq, lwt_topic, (int)strlen("offline"), "offline",
                                config->mqtt_qos, true);
        if (rc != MOSQ_ERR_SUCCESS) {
            log_warn("MQTT: Failed to set LWT: %s (continuing anyway)", mosquitto_strerror(rc));
        } else {
            log_info("MQTT: LWT set on topic %s", lwt_topic);
        }
    }

    pthread_mutex_unlock(&mqtt_mutex);

    log_info("MQTT: Client initialized (broker: %s:%d, client_id: %s)",
             config->mqtt_broker_host, config->mqtt_broker_port, config->mqtt_client_id);
    
    return 0;
}

/**
 * Connect to the MQTT broker
 */
int mqtt_connect(void) {
    if (!mosq || !mqtt_config || !mqtt_config->mqtt_enabled) {
        return 0; // Not initialized or disabled
    }
    
    pthread_mutex_lock(&mqtt_mutex);
    
    int rc = mosquitto_connect(mosq, mqtt_config->mqtt_broker_host, 
                               mqtt_config->mqtt_broker_port, 
                               mqtt_config->mqtt_keepalive);
    if (rc != MOSQ_ERR_SUCCESS) {
        log_error("MQTT: Failed to connect to broker: %s", mosquitto_strerror(rc));
        pthread_mutex_unlock(&mqtt_mutex);
        return -1;
    }
    
    // Start the network loop in a background thread
    rc = mosquitto_loop_start(mosq);
    if (rc != MOSQ_ERR_SUCCESS) {
        log_error("MQTT: Failed to start loop: %s", mosquitto_strerror(rc));
        mosquitto_disconnect(mosq);
        pthread_mutex_unlock(&mqtt_mutex);
        return -1;
    }
    
    pthread_mutex_unlock(&mqtt_mutex);
    
    log_info("MQTT: Connecting to broker %s:%d...", 
             mqtt_config->mqtt_broker_host, mqtt_config->mqtt_broker_port);
    
    return 0;
}

/**
 * Check if MQTT client is connected
 */
bool mqtt_is_connected(void) {
    pthread_mutex_lock(&mqtt_mutex);
    bool result = connected;
    pthread_mutex_unlock(&mqtt_mutex);
    return result;
}

// Connection callback
static void on_connect(struct mosquitto *m, void *userdata, int rc) {
    (void)m;
    (void)userdata;

    // Skip mutex acquisition if we're shutting down to prevent deadlock
    if (shutting_down) {
        return;
    }

    pthread_mutex_lock(&mqtt_mutex);
    // Double-check after acquiring mutex
    if (shutting_down) {
        pthread_mutex_unlock(&mqtt_mutex);
        return;
    }
    if (rc == 0) {
        connected = true;
        log_info("MQTT: Connected to broker successfully");
        pthread_mutex_unlock(&mqtt_mutex);

        // Publish availability "online" for HA discovery
        if (mqtt_config && mqtt_config->mqtt_ha_discovery) {
            char avail_topic[512];
            snprintf(avail_topic, sizeof(avail_topic), "%s/availability",
                     mqtt_config->mqtt_topic_prefix);
            mqtt_publish_raw(avail_topic, "online", true);
            log_info("MQTT: Published availability 'online' to %s", avail_topic);
        }
    } else {
        connected = false;
        log_error("MQTT: Connection failed: %s", mosquitto_connack_string(rc));
        pthread_mutex_unlock(&mqtt_mutex);
    }
}

// Disconnection callback
static void on_disconnect(struct mosquitto *m, void *userdata, int rc) {
    (void)m;
    (void)userdata;

    // Skip mutex acquisition if we're shutting down to prevent deadlock
    if (shutting_down) {
        // Still update the flag without mutex during shutdown - it's a simple write
        connected = false;
        if (rc == 0) {
            log_info("MQTT: Disconnected from broker (shutdown)");
        }
        return;
    }

    pthread_mutex_lock(&mqtt_mutex);
    // Double-check after acquiring mutex
    if (shutting_down) {
        pthread_mutex_unlock(&mqtt_mutex);
        connected = false;
        return;
    }
    connected = false;
    pthread_mutex_unlock(&mqtt_mutex);

    if (rc == 0) {
        log_info("MQTT: Disconnected from broker");
    } else {
        log_warn("MQTT: Unexpected disconnection (rc=%d), will attempt reconnect", rc);
    }
}

// Log callback (for debugging)
static void on_log(struct mosquitto *m, void *userdata, int level, const char *str) {
    (void)m;
    (void)userdata;

    switch (level) {
        case MOSQ_LOG_ERR:
            log_error("MQTT: %s", str);
            break;
        case MOSQ_LOG_WARNING:
            log_warn("MQTT: %s", str);
            break;
        case MOSQ_LOG_INFO:
        case MOSQ_LOG_NOTICE:
            log_debug("MQTT: %s", str);
            break;
        default:
            break;
    }
}

/**
 * Publish a detection event to MQTT
 */
int mqtt_publish_detection(const char *stream_name, const detection_result_t *result, time_t timestamp) {
    if (!mosq || !mqtt_config || !mqtt_config->mqtt_enabled) {
        return 0; // Not initialized or disabled
    }

    if (!stream_name || !result || result->count == 0) {
        return 0; // Nothing to publish
    }

    if (!mqtt_is_connected()) {
        log_debug("MQTT: Not connected, skipping detection publish");
        return -1;
    }

    // Build topic: {prefix}/detections/{stream_name}
    char topic[512];
    snprintf(topic, sizeof(topic), "%s/detections/%s",
             mqtt_config->mqtt_topic_prefix, stream_name);

    // Build JSON payload
    cJSON *root = cJSON_CreateObject();
    if (!root) {
        log_error("MQTT: Failed to create JSON object");
        return -1;
    }

    cJSON_AddStringToObject(root, "stream", stream_name);
    cJSON_AddNumberToObject(root, "timestamp", (double)timestamp);
    cJSON_AddNumberToObject(root, "count", result->count);

    cJSON *detections = cJSON_CreateArray();
    if (!detections) {
        cJSON_Delete(root);
        log_error("MQTT: Failed to create JSON array");
        return -1;
    }

    for (int i = 0; i < result->count && i < MAX_DETECTIONS; i++) {
        cJSON *det = cJSON_CreateObject();
        if (det) {
            cJSON_AddStringToObject(det, "label", result->detections[i].label);
            cJSON_AddNumberToObject(det, "confidence", result->detections[i].confidence);
            cJSON_AddNumberToObject(det, "x", result->detections[i].x);
            cJSON_AddNumberToObject(det, "y", result->detections[i].y);
            cJSON_AddNumberToObject(det, "width", result->detections[i].width);
            cJSON_AddNumberToObject(det, "height", result->detections[i].height);
            if (result->detections[i].track_id >= 0) {
                cJSON_AddNumberToObject(det, "track_id", result->detections[i].track_id);
            }
            if (result->detections[i].zone_id[0] != '\0') {
                cJSON_AddStringToObject(det, "zone_id", result->detections[i].zone_id);
            }
            cJSON_AddItemToArray(detections, det);
        }
    }
    cJSON_AddItemToObject(root, "detections", detections);

    char *payload = cJSON_PrintUnformatted(root);
    cJSON_Delete(root);

    if (!payload) {
        log_error("MQTT: Failed to serialize JSON");
        return -1;
    }

    // Publish message
    pthread_mutex_lock(&mqtt_mutex);
    int rc = mosquitto_publish(mosq, NULL, topic, (int)strlen(payload), payload,
                               mqtt_config->mqtt_qos, mqtt_config->mqtt_retain);
    pthread_mutex_unlock(&mqtt_mutex);

    free(payload);

    if (rc != MOSQ_ERR_SUCCESS) {
        log_error("MQTT: Failed to publish detection: %s", mosquitto_strerror(rc));
        return -1;
    }

    log_debug("MQTT: Published %d detection(s) to %s", result->count, topic);
    return 0;
}

/**
 * Publish a raw message to a custom topic
 */
int mqtt_publish_raw(const char *topic, const char *payload, bool retain) {
    if (!mosq || !mqtt_config || !mqtt_config->mqtt_enabled) {
        return 0;
    }

    if (!topic || !payload) {
        return -1;
    }

    if (!mqtt_is_connected()) {
        return -1;
    }

    pthread_mutex_lock(&mqtt_mutex);
    int rc = mosquitto_publish(mosq, NULL, topic, (int)strlen(payload), payload,
                               mqtt_config->mqtt_qos, retain);
    pthread_mutex_unlock(&mqtt_mutex);

    if (rc != MOSQ_ERR_SUCCESS) {
        log_error("MQTT: Failed to publish to %s: %s", topic, mosquitto_strerror(rc));
        return -1;
    }

    return 0;
}

/**
 * Publish binary data to a topic (e.g., JPEG snapshots)
 */
int mqtt_publish_binary(const char *topic, const void *data, size_t len, bool retain) {
    if (!mosq || !mqtt_config || !mqtt_config->mqtt_enabled) {
        return 0;
    }
    if (!topic || !data || len == 0) {
        return -1;
    }
    if (!mqtt_is_connected()) {
        return -1;
    }

    pthread_mutex_lock(&mqtt_mutex);
    int rc = mosquitto_publish(mosq, NULL, topic, (int)len, data, mqtt_config->mqtt_qos, retain);
    pthread_mutex_unlock(&mqtt_mutex);

    if (rc != MOSQ_ERR_SUCCESS) {
        log_error("MQTT: Failed to publish binary to %s: %s", topic, mosquitto_strerror(rc));
        return -1;
    }

    return 0;
}

/**
 * Sanitize a stream name for use as a Home Assistant unique_id / object_id.
 * Replaces non-alphanumeric characters with underscores and lowercases.
 */
static void sanitize_stream_name(const char *input, char *output, size_t output_size) {
    size_t i = 0;
    for (; i < output_size - 1 && input[i] != '\0'; i++) {
        char c = input[i];
        if ((c >= 'a' && c <= 'z') || (c >= '0' && c <= '9')) {
            output[i] = c;
        } else if (c >= 'A' && c <= 'Z') {
            output[i] = c + ('a' - 'A');
        } else {
            output[i] = '_';
        }
    }
    output[i] = '\0';
}

/**
 * Build the common HA device JSON block for lightNVR.
 * Caller must free the returned cJSON object.
 */
static cJSON *build_ha_device_block(void) {
    cJSON *device = cJSON_CreateObject();
    if (!device) return NULL;

    cJSON *ids = cJSON_CreateArray();
    cJSON_AddItemToArray(ids, cJSON_CreateString("lightnvr"));
    cJSON_AddItemToObject(device, "identifiers", ids);
    cJSON_AddStringToObject(device, "name", "LightNVR");
    cJSON_AddStringToObject(device, "manufacturer", "OpenSensor");
    cJSON_AddStringToObject(device, "model", "LightNVR");
    cJSON_AddStringToObject(device, "sw_version", LIGHTNVR_VERSION_STRING);

    // Configuration URL (point to web UI)
    if (mqtt_config) {
        char config_url[256];
        snprintf(config_url, sizeof(config_url), "http://localhost:%d", mqtt_config->web_port);
        cJSON_AddStringToObject(device, "configuration_url", config_url);
    }

    return device;
}

/**
 * Publish Home Assistant MQTT discovery messages for all configured streams.
 */
int mqtt_publish_ha_discovery(void) {
    if (!mosq || !mqtt_config || !mqtt_config->mqtt_enabled || !mqtt_config->mqtt_ha_discovery) {
        return 0;
    }

    if (!mqtt_is_connected()) {
        log_warn("MQTT HA: Not connected, skipping discovery publish");
        return -1;
    }

    log_info("MQTT HA: Publishing Home Assistant discovery messages...");

    // Get all configured streams
    stream_config_t streams[MAX_MOTION_STREAMS];
    int num_streams = get_all_stream_configs(streams, MAX_MOTION_STREAMS);
    if (num_streams <= 0) {
        log_warn("MQTT HA: No streams configured, skipping discovery");
        return 0;
    }

    const char *prefix = mqtt_config->mqtt_ha_discovery_prefix;
    const char *topic_prefix = mqtt_config->mqtt_topic_prefix;
    int published = 0;

    for (int i = 0; i < num_streams; i++) {
        if (!streams[i].enabled || streams[i].name[0] == '\0') {
            continue;
        }

        char safe_name[256];
        sanitize_stream_name(streams[i].name, safe_name, sizeof(safe_name));

        // --- 1. Camera entity (snapshot image via MQTT) ---
        {
            char topic[512];
            snprintf(topic, sizeof(topic), "%s/camera/lightnvr/%s/config", prefix, safe_name);

            cJSON *payload = cJSON_CreateObject();
            if (!payload) continue;

            char unique_id[256];
            snprintf(unique_id, sizeof(unique_id), "lightnvr_%s_camera", safe_name);
            cJSON_AddStringToObject(payload, "unique_id", unique_id);

            char name[256];
            snprintf(name, sizeof(name), "%s", streams[i].name);
            cJSON_AddStringToObject(payload, "name", name);

            char image_topic[512];
            snprintf(image_topic, sizeof(image_topic), "%s/cameras/%s/snapshot",
                     topic_prefix, streams[i].name);
            cJSON_AddStringToObject(payload, "topic", image_topic);

            // Availability
            cJSON *avail = cJSON_CreateObject();
            char avail_topic[512];
            snprintf(avail_topic, sizeof(avail_topic), "%s/availability", topic_prefix);
            cJSON_AddStringToObject(avail, "topic", avail_topic);
            cJSON_AddStringToObject(avail, "payload_available", "online");
            cJSON_AddStringToObject(avail, "payload_not_available", "offline");
            cJSON *avail_list = cJSON_CreateArray();
            cJSON_AddItemToArray(avail_list, avail);
            cJSON_AddItemToObject(payload, "availability", avail_list);

            // Device
            cJSON_AddItemToObject(payload, "device", build_ha_device_block());

            char *json_str = cJSON_PrintUnformatted(payload);
            cJSON_Delete(payload);
            if (json_str) {
                mqtt_publish_raw(topic, json_str, true);
                free(json_str);
                published++;
            }
        }

        // --- 2. Binary sensor for motion detection ---
        {
            char topic[512];
            snprintf(topic, sizeof(topic), "%s/binary_sensor/lightnvr/%s_motion/config",
                     prefix, safe_name);

            cJSON *payload = cJSON_CreateObject();
            if (!payload) continue;

            char unique_id[256];
            snprintf(unique_id, sizeof(unique_id), "lightnvr_%s_motion", safe_name);
            cJSON_AddStringToObject(payload, "unique_id", unique_id);

            char name[256];
            snprintf(name, sizeof(name), "%s Motion", streams[i].name);
            cJSON_AddStringToObject(payload, "name", name);

            char state_topic[512];
            snprintf(state_topic, sizeof(state_topic), "%s/cameras/%s/motion",
                     topic_prefix, streams[i].name);
            cJSON_AddStringToObject(payload, "state_topic", state_topic);
            cJSON_AddStringToObject(payload, "payload_on", "ON");
            cJSON_AddStringToObject(payload, "payload_off", "OFF");
            cJSON_AddStringToObject(payload, "device_class", "motion");

            // Availability
            cJSON *avail = cJSON_CreateObject();
            char avail_topic[512];
            snprintf(avail_topic, sizeof(avail_topic), "%s/availability", topic_prefix);
            cJSON_AddStringToObject(avail, "topic", avail_topic);
            cJSON_AddStringToObject(avail, "payload_available", "online");
            cJSON_AddStringToObject(avail, "payload_not_available", "offline");
            cJSON *avail_list = cJSON_CreateArray();
            cJSON_AddItemToArray(avail_list, avail);
            cJSON_AddItemToObject(payload, "availability", avail_list);

            // Device
            cJSON_AddItemToObject(payload, "device", build_ha_device_block());

            char *json_str = cJSON_PrintUnformatted(payload);
            cJSON_Delete(payload);
            if (json_str) {
                mqtt_publish_raw(topic, json_str, true);
                free(json_str);
                published++;
            }
        }

        // --- 3. Sensor for detection count (generic) ---
        {
            char topic[512];
            snprintf(topic, sizeof(topic), "%s/sensor/lightnvr/%s_detection_count/config",
                     prefix, safe_name);

            cJSON *payload = cJSON_CreateObject();
            if (!payload) continue;

            char unique_id[256];
            snprintf(unique_id, sizeof(unique_id), "lightnvr_%s_detection_count", safe_name);
            cJSON_AddStringToObject(payload, "unique_id", unique_id);

            char name[256];
            snprintf(name, sizeof(name), "%s Detections", streams[i].name);
            cJSON_AddStringToObject(payload, "name", name);

            char state_topic[512];
            snprintf(state_topic, sizeof(state_topic), "%s/cameras/%s/detection_count",
                     topic_prefix, streams[i].name);
            cJSON_AddStringToObject(payload, "state_topic", state_topic);
            cJSON_AddStringToObject(payload, "icon", "mdi:motion-sensor");

            // Availability
            cJSON *avail = cJSON_CreateObject();
            char avail_topic[512];
            snprintf(avail_topic, sizeof(avail_topic), "%s/availability", topic_prefix);
            cJSON_AddStringToObject(avail, "topic", avail_topic);
            cJSON_AddStringToObject(avail, "payload_available", "online");
            cJSON_AddStringToObject(avail, "payload_not_available", "offline");
            cJSON *avail_list = cJSON_CreateArray();
            cJSON_AddItemToArray(avail_list, avail);
            cJSON_AddItemToObject(payload, "availability", avail_list);

            // Device
            cJSON_AddItemToObject(payload, "device", build_ha_device_block());

            char *json_str = cJSON_PrintUnformatted(payload);
            cJSON_Delete(payload);
            if (json_str) {
                mqtt_publish_raw(topic, json_str, true);
                free(json_str);
                published++;
            }
        }
    }

    log_info("MQTT HA: Published %d discovery messages for %d streams", published, num_streams);
    return 0;
}

/**
 * Update the motion state for a camera stream.
 * Publishes ON on detection, tracks last detection time for debounce OFF.
 * Also updates per-object-class counts.
 */
void mqtt_set_motion_state(const char *stream_name, const detection_result_t *result) {
    if (!mosq || !mqtt_config || !mqtt_config->mqtt_enabled || !mqtt_config->mqtt_ha_discovery) {
        return;
    }
    if (!stream_name || stream_name[0] == '\0') {
        return;
    }

    pthread_mutex_lock(&motion_mutex);

    // Find or create motion state for this stream
    motion_state_t *state = NULL;
    for (int i = 0; i < num_motion_states; i++) {
        if (strcmp(motion_states[i].stream_name, stream_name) == 0) {
            state = &motion_states[i];
            break;
        }
    }

    if (!state && num_motion_states < MAX_MOTION_STREAMS) {
        state = &motion_states[num_motion_states++];
        memset(state, 0, sizeof(*state));
        strncpy(state->stream_name, stream_name, sizeof(state->stream_name) - 1);
    }

    if (!state) {
        pthread_mutex_unlock(&motion_mutex);
        return;
    }

    state->last_detection_time = time(NULL);

    // Publish ON if not already active
    bool should_publish_on = !state->motion_active;
    state->motion_active = true;

    // Update object counts
    if (result && result->count > 0) {
        // Reset counts
        memset(state->object_counts, 0, sizeof(state->object_counts));
        state->num_labels = 0;

        for (int i = 0; i < result->count && i < MAX_DETECTIONS; i++) {
            const char *label = result->detections[i].label;
            if (label[0] == '\0') continue;

            // Find existing label or add new one
            int label_idx = -1;
            for (int j = 0; j < state->num_labels; j++) {
                if (strcmp(state->object_labels[j], label) == 0) {
                    label_idx = j;
                    break;
                }
            }
            if (label_idx < 0 && state->num_labels < 32) {
                label_idx = state->num_labels++;
                strncpy(state->object_labels[label_idx], label,
                        sizeof(state->object_labels[0]) - 1);
            }
            if (label_idx >= 0) {
                state->object_counts[label_idx]++;
            }
        }
    }

    // Copy data we need before releasing mutex
    int total_count = result ? result->count : 0;
    int num_labels = state->num_labels;
    char labels_copy[32][32];
    int counts_copy[32];
    memcpy(labels_copy, state->object_labels, sizeof(labels_copy));
    memcpy(counts_copy, state->object_counts, sizeof(counts_copy));

    pthread_mutex_unlock(&motion_mutex);

    // Publish motion ON
    if (should_publish_on) {
        char topic[512];
        snprintf(topic, sizeof(topic), "%s/cameras/%s/motion",
                 mqtt_config->mqtt_topic_prefix, stream_name);
        mqtt_publish_raw(topic, "ON", false);
        log_debug("MQTT HA: Motion ON for %s", stream_name);
    }

    // Publish detection count
    {
        char topic[512];
        snprintf(topic, sizeof(topic), "%s/cameras/%s/detection_count",
                 mqtt_config->mqtt_topic_prefix, stream_name);
        char count_str[16];
        snprintf(count_str, sizeof(count_str), "%d", total_count);
        mqtt_publish_raw(topic, count_str, false);
    }

    // Publish per-object-class counts
    for (int i = 0; i < num_labels; i++) {
        char topic[512];
        snprintf(topic, sizeof(topic), "%s/cameras/%s/%s",
                 mqtt_config->mqtt_topic_prefix, stream_name, labels_copy[i]);
        char count_str[16];
        snprintf(count_str, sizeof(count_str), "%d", counts_copy[i]);
        mqtt_publish_raw(topic, count_str, false);
    }
}

/**
 * Background thread: periodically publishes JPEG snapshots for each stream.
 */
static void *ha_snapshot_thread_func(void *arg) {
    (void)arg;
    log_info("MQTT HA: Snapshot publishing thread started (interval=%ds)",
             mqtt_config->mqtt_ha_snapshot_interval);

    while (ha_services_running) {
        if (!mqtt_is_connected() || !mqtt_config) {
            sleep(1);
            continue;
        }

        stream_config_t streams[MAX_MOTION_STREAMS];
        int num_streams = get_all_stream_configs(streams, MAX_MOTION_STREAMS);

        for (int i = 0; i < num_streams && ha_services_running; i++) {
            if (!streams[i].enabled || streams[i].name[0] == '\0') {
                continue;
            }

            unsigned char *jpeg_data = NULL;
            size_t jpeg_size = 0;

            if (go2rtc_get_snapshot(streams[i].name, &jpeg_data, &jpeg_size)) {
                char topic[512];
                snprintf(topic, sizeof(topic), "%s/cameras/%s/snapshot",
                         mqtt_config->mqtt_topic_prefix, streams[i].name);
                mqtt_publish_binary(topic, jpeg_data, jpeg_size, false);
                log_debug("MQTT HA: Published snapshot for %s (%zu bytes)",
                          streams[i].name, jpeg_size);
                free(jpeg_data);
            } else {
                log_debug("MQTT HA: Failed to get snapshot for %s", streams[i].name);
            }
        }

        // Sleep in 1-second increments so we can check ha_services_running
        for (int s = 0; s < mqtt_config->mqtt_ha_snapshot_interval && ha_services_running; s++) {
            sleep(1);
        }
    }

    go2rtc_snapshot_cleanup_thread();
    log_info("MQTT HA: Snapshot publishing thread stopped");
    return NULL;
}

/**
 * Background thread: checks motion states and publishes OFF after timeout.
 */
static void *ha_motion_thread_func(void *arg) {
    (void)arg;
    log_info("MQTT HA: Motion timeout thread started");

    while (ha_services_running) {
        if (!mqtt_is_connected() || !mqtt_config) {
            sleep(1);
            continue;
        }

        time_t now = time(NULL);

        pthread_mutex_lock(&motion_mutex);
        for (int i = 0; i < num_motion_states; i++) {
            if (motion_states[i].motion_active &&
                (now - motion_states[i].last_detection_time) >= MOTION_OFF_DELAY_SEC) {

                motion_states[i].motion_active = false;
                char stream_name[256];
                strncpy(stream_name, motion_states[i].stream_name, sizeof(stream_name) - 1);
                stream_name[sizeof(stream_name) - 1] = '\0';

                pthread_mutex_unlock(&motion_mutex);

                // Publish motion OFF
                char topic[512];
                snprintf(topic, sizeof(topic), "%s/cameras/%s/motion",
                         mqtt_config->mqtt_topic_prefix, stream_name);
                mqtt_publish_raw(topic, "OFF", false);
                log_debug("MQTT HA: Motion OFF for %s (timeout)", stream_name);

                // Reset detection count to 0
                snprintf(topic, sizeof(topic), "%s/cameras/%s/detection_count",
                         mqtt_config->mqtt_topic_prefix, stream_name);
                mqtt_publish_raw(topic, "0", false);

                pthread_mutex_lock(&motion_mutex);
            }
        }
        pthread_mutex_unlock(&motion_mutex);

        sleep(1); // Check every second
    }

    log_info("MQTT HA: Motion timeout thread stopped");
    return NULL;
}

/**
 * Start Home Assistant background services (snapshot timer, motion timeout).
 */
int mqtt_start_ha_services(void) {
    if (!mqtt_config || !mqtt_config->mqtt_ha_discovery) {
        return 0;
    }
    if (ha_services_running) {
        return 0; // Already running
    }

    ha_services_running = true;

    // Start snapshot publishing thread if interval > 0
    if (mqtt_config->mqtt_ha_snapshot_interval > 0) {
        if (pthread_create(&ha_snapshot_thread, NULL, ha_snapshot_thread_func, NULL) != 0) {
            log_error("MQTT HA: Failed to create snapshot thread");
            ha_services_running = false;
            return -1;
        }
        log_info("MQTT HA: Snapshot publishing started (interval=%ds)",
                 mqtt_config->mqtt_ha_snapshot_interval);
    }

    // Start motion timeout thread
    if (pthread_create(&ha_motion_thread, NULL, ha_motion_thread_func, NULL) != 0) {
        log_error("MQTT HA: Failed to create motion timeout thread");
        ha_services_running = false;
        return -1;
    }

    log_info("MQTT HA: Background services started");
    return 0;
}

/**
 * Stop Home Assistant background services.
 */
void mqtt_stop_ha_services(void) {
    if (!ha_services_running) {
        return;
    }

    log_info("MQTT HA: Stopping background services...");
    ha_services_running = false;

    // Wait for threads to finish (they check ha_services_running each second)
    if (mqtt_config && mqtt_config->mqtt_ha_snapshot_interval > 0) {
        pthread_join(ha_snapshot_thread, NULL);
    }
    pthread_join(ha_motion_thread, NULL);

    log_info("MQTT HA: Background services stopped");
}

// Cleanup operation types
typedef enum {
    MQTT_OP_LOOP_STOP,
    MQTT_OP_DESTROY,
    MQTT_OP_LIB_CLEANUP
} mqtt_cleanup_op_t;

// Thread argument structure - uses a simple volatile flag for completion
typedef struct {
    struct mosquitto *mosq;
    mqtt_cleanup_op_t op;
    volatile int *done_flag;  // Pointer to completion flag
} mqtt_cleanup_arg_t;

/**
 * Thread function to run blocking mosquitto operations with timeout capability
 */
static void *mqtt_cleanup_thread(void *arg) {
    mqtt_cleanup_arg_t *cleanup_arg = (mqtt_cleanup_arg_t *)arg;
    volatile int *done_flag = cleanup_arg->done_flag;

    switch (cleanup_arg->op) {
        case MQTT_OP_LOOP_STOP:
            mosquitto_loop_stop(cleanup_arg->mosq, true);
            break;
        case MQTT_OP_DESTROY:
            mosquitto_destroy(cleanup_arg->mosq);
            break;
        case MQTT_OP_LIB_CLEANUP:
            mosquitto_lib_cleanup();
            break;
    }

    // Signal completion by setting the flag
    // Note: cleanup_arg is on the stack of the caller and remains valid
    // until the caller returns (after timeout or completion)
    *done_flag = 1;

    return NULL;
}

/**
 * Run a mosquitto cleanup operation with a timeout
 * Uses simple polling with usleep - portable across glibc and musl
 * Returns true if completed within timeout, false if timed out
 */
static bool mqtt_run_with_timeout(struct mosquitto *m, mqtt_cleanup_op_t op, int timeout_sec, const char *op_name) {
    pthread_t thread;
    volatile int done_flag = 0;

    // Argument structure on stack - valid for duration of this function
    mqtt_cleanup_arg_t arg;
    arg.mosq = m;
    arg.op = op;
    arg.done_flag = &done_flag;

    // Create thread to run the operation
    if (pthread_create(&thread, NULL, mqtt_cleanup_thread, &arg) != 0) {
        log_warn("MQTT: Failed to create thread for %s", op_name);
        return false;
    }

    // Detach the thread so it cleans up automatically if we timeout
    pthread_detach(thread);

    // Poll for completion with 50ms intervals
    int timeout_ms = timeout_sec * 1000;
    int elapsed_ms = 0;
    const int poll_interval_ms = 50;

    while (elapsed_ms < timeout_ms) {
        if (done_flag) {
            // Operation completed successfully
            return true;
        }
        usleep(poll_interval_ms * 1000);
        elapsed_ms += poll_interval_ms;
    }

    // Timeout - the thread is still running but we'll return anyway
    // The thread will eventually complete (or process will exit)
    // Use write() first to ensure we see the timeout even if logger is blocked
    char timeout_msg[128];
    snprintf(timeout_msg, sizeof(timeout_msg), "[MQTT DEBUG] %s timed out after %d seconds\n", op_name, timeout_sec);
    (void)write(STDERR_FILENO, timeout_msg, strlen(timeout_msg));
    log_warn("MQTT: %s timed out after %d seconds", op_name, timeout_sec);
    return false;
}

/**
 * Try to acquire mutex with timeout using portable polling approach
 * Returns true if mutex was acquired, false if timeout
 */
static bool mqtt_try_lock_with_timeout(int timeout_ms) {
    int elapsed_ms = 0;
    const int poll_interval_ms = 50;

    while (elapsed_ms < timeout_ms) {
        if (pthread_mutex_trylock(&mqtt_mutex) == 0) {
            return true;  // Successfully acquired mutex
        }
        usleep(poll_interval_ms * 1000);
        elapsed_ms += poll_interval_ms;
    }
    return false;  // Timeout
}

/**
 * Disconnect from the MQTT broker
 * Returns true if disconnect completed cleanly, false if there were timeouts
 */
static bool mqtt_disconnect_internal(void) {
    if (!mosq) {
        log_info("MQTT: No client to disconnect");
        return true;
    }

    bool had_timeout = false;

    log_info("MQTT: Attempting to acquire mutex for disconnect...");

    // Try to lock with timeout using portable polling approach
    if (!mqtt_try_lock_with_timeout(2000)) {  // 2 second timeout
        log_warn("MQTT: Failed to acquire mutex for disconnect (timeout), proceeding anyway");
        // Force disconnect without mutex - risky but better than hanging
        mosquitto_disconnect(mosq);
        if (!mqtt_run_with_timeout(mosq, MQTT_OP_LOOP_STOP, 2, "mosquitto_loop_stop")) {
            had_timeout = true;
        }
        connected = false;
        log_info("MQTT: Forced disconnect without mutex");
        return !had_timeout;
    }

    log_info("MQTT: Calling mosquitto_disconnect...");
    mosquitto_disconnect(mosq);

    log_info("MQTT: Calling mosquitto_loop_stop with 2 second timeout...");
    if (!mqtt_run_with_timeout(mosq, MQTT_OP_LOOP_STOP, 2, "mosquitto_loop_stop")) {
        had_timeout = true;
    }

    connected = false;

    pthread_mutex_unlock(&mqtt_mutex);

    log_info("MQTT: Disconnected");
    return !had_timeout;
}

/**
 * Disconnect from the MQTT broker (public API)
 */
void mqtt_disconnect(void) {
    mqtt_disconnect_internal();
}

/**
 * Cleanup MQTT resources
 */
void mqtt_cleanup(void) {
    log_info("MQTT: Starting cleanup...");

    // Stop HA background services first
    mqtt_stop_ha_services();

    // Set shutdown flag FIRST to prevent callbacks from acquiring mutex
    // This must happen before any other cleanup operations
    shutting_down = true;
    // Memory barrier to ensure the flag is visible to other threads
    __sync_synchronize();

    if (!mosq) {
        log_info("MQTT: No client to clean up");
        // Still need to cleanup the library if it was initialized
        log_info("MQTT: Calling mosquitto_lib_cleanup with 2 second timeout...");
        mqtt_run_with_timeout(NULL, MQTT_OP_LIB_CLEANUP, 2, "mosquitto_lib_cleanup");
        log_info("MQTT: Cleaned up");
        return;
    }

    // Track if any operation times out - if so, skip mosquitto_lib_cleanup
    // because it will likely hang waiting for threads that didn't stop cleanly
    bool had_timeout = false;

    // Use internal disconnect that returns timeout status
    if (!mqtt_disconnect_internal()) {
        had_timeout = true;
    }

    log_info("MQTT: Attempting to acquire mutex for destroy...");

    // Try to lock with timeout using portable polling approach
    if (!mqtt_try_lock_with_timeout(2000)) {  // 2 second timeout
        log_warn("MQTT: Failed to acquire mutex for destroy (timeout), skipping remaining cleanup");
        // Skip destroy and lib_cleanup entirely - they're blocking and we're shutting down anyway
        mosq = NULL;
        mqtt_config = NULL;
        log_info("MQTT: Skipped destroy and lib_cleanup due to mutex timeout");
        return;
    }

    log_info("MQTT: Calling mosquitto_destroy with 2 second timeout...");
    bool destroyed = mqtt_run_with_timeout(mosq, MQTT_OP_DESTROY, 2, "mosquitto_destroy");
    if (destroyed) {
        mosq = NULL;
    } else {
        log_warn("MQTT: mosquitto_destroy timed out, setting mosq to NULL anyway");
        mosq = NULL;
        had_timeout = true;
    }
    mqtt_config = NULL;
    pthread_mutex_unlock(&mqtt_mutex);

    // Skip mosquitto_lib_cleanup if we had timeouts - it will likely hang
    // waiting for threads that didn't stop cleanly. The OS will clean up
    // when the process exits.
    if (had_timeout) {
        log_warn("MQTT: Skipping mosquitto_lib_cleanup due to previous timeouts");
        log_info("MQTT: Cleaned up (with warnings)");
        return;
    }

    log_info("MQTT: Calling mosquitto_lib_cleanup with 2 second timeout...");
    mqtt_run_with_timeout(NULL, MQTT_OP_LIB_CLEANUP, 2, "mosquitto_lib_cleanup");
    log_info("MQTT: Cleaned up");
}

#endif /* ENABLE_MQTT */

