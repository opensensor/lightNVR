#ifdef ENABLE_MQTT

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <errno.h>
#include <pthread.h>
#include <signal.h>
#include <unistd.h>
#include <time.h>
#include <dirent.h>
#include <limits.h>
#include <sys/stat.h>
#include <mosquitto.h>
#include <cjson/cJSON.h>

#include "core/mqtt_client.h"
#include "core/logger.h"
#include "core/path_utils.h"
#include "core/version.h"
#include "database/db_streams.h"
#include "utils/strings.h"
#include "utils/interruptible_sleep.h"
#include "video/go2rtc/go2rtc_snapshot.h"

#define MAX_TOPIC_LENGTH 512

// MQTT client state
static struct mosquitto *mosq = NULL;
static const config_t *mqtt_config = NULL;
static bool connected = false;
static volatile bool shutting_down = false;  // Flag to prevent callbacks from acquiring mutex during shutdown
static pthread_mutex_t mqtt_mutex = PTHREAD_MUTEX_INITIALIZER;

// HA discovery state
static volatile bool ha_services_running = false;
static bool ha_snapshot_thread_started = false;
static pthread_t ha_snapshot_thread;
static pthread_t ha_motion_thread;
// Wakeable sleeps so mqtt_stop_ha_services() pulls the HA threads out of their
// interval sleeps immediately instead of waiting up to a full interval.
static interruptible_sleep_t ha_snapshot_wake;
static interruptible_sleep_t ha_motion_wake;

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
static void on_message(struct mosquitto *mosq, void *userdata, const struct mosquitto_message *msg);
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
    mosquitto_message_callback_set(mosq, on_message);
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
        char lwt_topic[MAX_TOPIC_LENGTH];
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
            char avail_topic[MAX_TOPIC_LENGTH];
            snprintf(avail_topic, sizeof(avail_topic), "%s/availability",
                     mqtt_config->mqtt_topic_prefix);
            mqtt_publish_raw(avail_topic, "online", true);
            log_info("MQTT: Published availability 'online' to %s", avail_topic);

            // Subscribe to HA birth topic so we can re-publish discovery
            // when Home Assistant restarts
            char status_topic[MAX_TOPIC_LENGTH];
            snprintf(status_topic, sizeof(status_topic), "%s/status",
                     mqtt_config->mqtt_ha_discovery_prefix);
            int sub_rc = mosquitto_subscribe(m, NULL, status_topic, 0);
            if (sub_rc == MOSQ_ERR_SUCCESS) {
                log_info("MQTT: Subscribed to HA birth topic %s", status_topic);
            } else {
                log_warn("MQTT: Failed to subscribe to HA birth topic: %s",
                         mosquitto_strerror(sub_rc));
            }

            // Publish discovery now, in case HA is already running.
            mqtt_publish_ha_discovery();
            mqtt_start_ha_services();
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

// Message callback — handles HA birth messages for re-discovery
static void on_message(struct mosquitto *m, void *userdata, const struct mosquitto_message *msg) {
    (void)m;
    (void)userdata;

    if (!msg || !msg->topic || !msg->payload || shutting_down) {
        return;
    }

    // Check if this is the HA birth message (status topic → "online")
    if (mqtt_config && mqtt_config->mqtt_ha_discovery) {
        char status_topic[MAX_TOPIC_LENGTH];
        snprintf(status_topic, sizeof(status_topic), "%s/status",
                 mqtt_config->mqtt_ha_discovery_prefix);

        if (strcmp(msg->topic, status_topic) == 0) {
            char payload_str[64];
            int len = msg->payloadlen < (int)sizeof(payload_str) - 1
                          ? msg->payloadlen : (int)sizeof(payload_str) - 1;
            memcpy(payload_str, msg->payload, len);
            payload_str[len] = '\0';

            if (strcmp(payload_str, "online") == 0) {
                log_info("MQTT HA: Home Assistant birth message received, re-publishing discovery");
                mqtt_publish_ha_discovery();
            }
        }
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

// Cap on saved event snapshots per stream; oldest are pruned on each save.
// Timestamped filenames sort lexicographically, so "oldest" == smallest name.
#define MAX_DETECTION_SNAPSHOTS_PER_STREAM 100

/**
 * Prune oldest snapshots in a stream's snapshot directory so at most
 * MAX_DETECTION_SNAPSHOTS_PER_STREAM files remain after the next save.
 */
static void prune_detection_snapshots(const char *dir_path) {
    DIR *dir = opendir(dir_path);
    if (!dir) {
        return;
    }

    // Two passes: count first, then delete the N oldest.  Directories are
    // capped at ~100 entries so the rescan is cheap and avoids allocating
    // a full file list.
    int count = 0;
    struct dirent *ent;
    while ((ent = readdir(dir)) != NULL) {
        if (ent->d_name[0] != '.' && strstr(ent->d_name, ".jpg")) {
            count++;
        }
    }

    while (count >= MAX_DETECTION_SNAPSHOTS_PER_STREAM) {
        rewinddir(dir);
        char oldest[NAME_MAX + 1] = "";
        while ((ent = readdir(dir)) != NULL) {
            if (ent->d_name[0] == '.' || !strstr(ent->d_name, ".jpg")) {
                continue;
            }
            if (oldest[0] == '\0' || strcmp(ent->d_name, oldest) < 0) {
                safe_strcpy(oldest, ent->d_name, sizeof(oldest), 0);
            }
        }
        if (oldest[0] == '\0') {
            break;
        }
        char victim[MAX_PATH_LENGTH];
        snprintf(victim, sizeof(victim), "%s/%s", dir_path, oldest);
        if (unlink(victim) != 0) {
            break;  // don't loop forever on an undeletable file
        }
        count--;
    }

    closedir(dir);
}

/**
 * Save a detection event snapshot to
 * {storage_path}/snapshots/{safe_stream}/{YYYYmmdd_HHMMSS}.jpg.
 *
 * On success fills out_path with the absolute file path and out_url with the
 * relative URL served by the /api/snapshots endpoint.  Returns 0 on success.
 */
static int save_detection_snapshot(const char *safe_name, time_t timestamp,
                                   const unsigned char *jpeg_data, size_t jpeg_size,
                                   char *out_path, size_t out_path_size,
                                   char *out_url, size_t out_url_size) {
    if (!mqtt_config || mqtt_config->storage_path[0] == '\0') {
        return -1;
    }

    char dir_path[MAX_PATH_LENGTH];
    snprintf(dir_path, sizeof(dir_path), "%s/snapshots/%s",
             mqtt_config->storage_path, safe_name);
    if (ensure_dir(dir_path) != 0) {
        log_error("MQTT: Failed to create snapshot directory %s", dir_path);
        return -1;
    }

    prune_detection_snapshots(dir_path);

    char ts_str[32];
    struct tm tm_buf;
    localtime_r(&timestamp, &tm_buf);
    strftime(ts_str, sizeof(ts_str), "%Y%m%d_%H%M%S", &tm_buf);

    char file_path[MAX_PATH_LENGTH];
    snprintf(file_path, sizeof(file_path), "%s/%s.jpg", dir_path, ts_str);

    // Write to a temp file first so a reader never sees a partial JPEG
    char tmp_path[MAX_PATH_LENGTH];
    snprintf(tmp_path, sizeof(tmp_path), "%s/.%s.jpg.tmp", dir_path, ts_str);
    FILE *f = fopen(tmp_path, "wb");
    if (!f) {
        log_error("MQTT: Failed to open snapshot file %s: %s", tmp_path, strerror(errno));
        return -1;
    }
    size_t written = fwrite(jpeg_data, 1, jpeg_size, f);
    fclose(f);
    if (written != jpeg_size || rename(tmp_path, file_path) != 0) {
        log_error("MQTT: Failed to write snapshot %s", file_path);
        unlink(tmp_path);
        return -1;
    }

    snprintf(out_path, out_path_size, "%s", file_path);
    snprintf(out_url, out_url_size, "/api/snapshots/%s/%s.jpg", safe_name, ts_str);
    return 0;
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

    char safe_name[MAX_STREAM_NAME];
    sanitize_stream_name(stream_name, safe_name, sizeof(safe_name));

    // Build topic: {prefix}/detections/{stream_name}
    char topic[MAX_TOPIC_LENGTH];
    snprintf(topic, sizeof(topic), "%s/detections/%s",
             mqtt_config->mqtt_topic_prefix, safe_name);

    // Capture an event snapshot so automations can attach the image to
    // notifications (issue #449).  The JPEG is published to a companion
    // topic *before* the JSON event (so it's available when the event
    // arrives) and saved to disk; the JSON payload then references it by
    // local path, relative URL, and topic.  All of this is best-effort:
    // if go2rtc is unavailable the event is simply published without
    // snapshot fields.
    char snapshot_path[MAX_PATH_LENGTH] = "";
    char snapshot_url[MAX_PATH_LENGTH] = "";
    char snapshot_topic[MAX_TOPIC_LENGTH] = "";
    {
        unsigned char *jpeg_data = NULL;
        size_t jpeg_size = 0;
        if (go2rtc_get_snapshot(stream_name, &jpeg_data, &jpeg_size) &&
            jpeg_data && jpeg_size > 0) {
            save_detection_snapshot(safe_name, timestamp, jpeg_data, jpeg_size,
                                    snapshot_path, sizeof(snapshot_path),
                                    snapshot_url, sizeof(snapshot_url));

            snprintf(snapshot_topic, sizeof(snapshot_topic), "%s/detections/%s/snapshot",
                     mqtt_config->mqtt_topic_prefix, safe_name);
            if (mqtt_publish_binary(snapshot_topic, jpeg_data, jpeg_size,
                                    mqtt_config->mqtt_retain) != 0) {
                snapshot_topic[0] = '\0';  // don't advertise a topic we failed to publish
            }
        }
        free(jpeg_data);
    }

    // Build JSON payload
    cJSON *root = cJSON_CreateObject();
    if (!root) {
        log_error("MQTT: Failed to create JSON object");
        return -1;
    }

    cJSON_AddStringToObject(root, "stream", stream_name);
    cJSON_AddNumberToObject(root, "timestamp", (double)timestamp);
    cJSON_AddNumberToObject(root, "count", result->count);

    if (snapshot_path[0] != '\0') {
        cJSON_AddStringToObject(root, "snapshot_path", snapshot_path);
    }
    if (snapshot_url[0] != '\0') {
        cJSON_AddStringToObject(root, "snapshot_url", snapshot_url);
    }
    if (snapshot_topic[0] != '\0') {
        cJSON_AddStringToObject(root, "snapshot_topic", snapshot_topic);
    }

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

    return device;
}

/**
 * Build the HA origin block for discovery payloads.
 * Caller must free the returned cJSON object.
 */
static cJSON *build_ha_origin_block(void) {
    cJSON *origin = cJSON_CreateObject();
    if (!origin) return NULL;

    cJSON_AddStringToObject(origin, "name", "LightNVR");
    cJSON_AddStringToObject(origin, "sw", LIGHTNVR_VERSION_STRING);
    cJSON_AddStringToObject(origin, "url", "https://github.com/opensensor/lightNVR");

    return origin;
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

    // Get all configured streams.
    // Heap-allocated: sizeof(stream_config_t) is >6 KB today and will grow to
    // ~12 KB after the T11 `go2rtc_source_override` bump. 16 entries on the
    // stack (>190 KB post-T11) would blow musl/Alpine's 128 KB default thread
    // stack. MQTT runs on a libuv worker thread.
    stream_config_t *streams = calloc(MAX_MOTION_STREAMS, sizeof(stream_config_t));
    if (!streams) {
        log_error("MQTT HA: Failed to allocate streams buffer");
        return -1;
    }
    int num_streams = get_all_stream_configs(streams, MAX_MOTION_STREAMS);
    if (num_streams <= 0) {
        log_warn("MQTT HA: No streams configured, skipping discovery");
        free(streams);
        return 0;
    }

    const char *prefix = mqtt_config->mqtt_ha_discovery_prefix;
    const char *topic_prefix = mqtt_config->mqtt_topic_prefix;
    int published = 0;

    for (int i = 0; i < num_streams; i++) {
        if (!streams[i].enabled || streams[i].name[0] == '\0') {
            continue;
        }

        char safe_name[MAX_STREAM_NAME];
        sanitize_stream_name(streams[i].name, safe_name, sizeof(safe_name));

        // --- 1. Camera entity (snapshot image via MQTT) ---
        {
            char topic[MAX_TOPIC_LENGTH];
            snprintf(topic, sizeof(topic), "%s/camera/lightnvr/%s/config", prefix, safe_name);

            cJSON *payload = cJSON_CreateObject();
            if (!payload) continue;

            char unique_id[256];
            snprintf(unique_id, sizeof(unique_id), "lightnvr_%s_camera", safe_name);
            cJSON_AddStringToObject(payload, "unique_id", unique_id);

            cJSON_AddStringToObject(payload, "name", streams[i].name);

            char image_topic[MAX_TOPIC_LENGTH];
            snprintf(image_topic, sizeof(image_topic), "%s/cameras/%s/snapshot",
                     topic_prefix, safe_name);
            cJSON_AddStringToObject(payload, "topic", image_topic);

            // Tell HA not to decode binary JPEG data as UTF-8
            cJSON_AddStringToObject(payload, "encoding", "");

            // Availability
            cJSON *avail = cJSON_CreateObject();
            char avail_topic[MAX_TOPIC_LENGTH];
            snprintf(avail_topic, sizeof(avail_topic), "%s/availability", topic_prefix);
            cJSON_AddStringToObject(avail, "topic", avail_topic);
            cJSON_AddStringToObject(avail, "payload_available", "online");
            cJSON_AddStringToObject(avail, "payload_not_available", "offline");
            cJSON *avail_list = cJSON_CreateArray();
            cJSON_AddItemToArray(avail_list, avail);
            cJSON_AddItemToObject(payload, "availability", avail_list);

            // Device & Origin
            cJSON *device = build_ha_device_block();
            cJSON *origin = build_ha_origin_block();
            if (!device || !origin) {
                cJSON_Delete(device);
                cJSON_Delete(origin);
                cJSON_Delete(payload);
                continue;
            }
            cJSON_AddItemToObject(payload, "device", device);
            cJSON_AddItemToObject(payload, "origin", origin);

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
            char topic[MAX_TOPIC_LENGTH];
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

            char state_topic[MAX_TOPIC_LENGTH];
            snprintf(state_topic, sizeof(state_topic), "%s/cameras/%s/motion",
                     topic_prefix, safe_name);
            cJSON_AddStringToObject(payload, "state_topic", state_topic);
            cJSON_AddStringToObject(payload, "payload_on", "ON");
            cJSON_AddStringToObject(payload, "payload_off", "OFF");
            cJSON_AddStringToObject(payload, "device_class", "motion");

            // Availability
            cJSON *avail = cJSON_CreateObject();
            char avail_topic[MAX_TOPIC_LENGTH];
            snprintf(avail_topic, sizeof(avail_topic), "%s/availability", topic_prefix);
            cJSON_AddStringToObject(avail, "topic", avail_topic);
            cJSON_AddStringToObject(avail, "payload_available", "online");
            cJSON_AddStringToObject(avail, "payload_not_available", "offline");
            cJSON *avail_list = cJSON_CreateArray();
            cJSON_AddItemToArray(avail_list, avail);
            cJSON_AddItemToObject(payload, "availability", avail_list);

            // Device & Origin
            cJSON *device = build_ha_device_block();
            cJSON *origin = build_ha_origin_block();
            if (!device || !origin) {
                cJSON_Delete(device);
                cJSON_Delete(origin);
                cJSON_Delete(payload);
                continue;
            }
            cJSON_AddItemToObject(payload, "device", device);
            cJSON_AddItemToObject(payload, "origin", origin);

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
            char topic[MAX_TOPIC_LENGTH];
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

            char state_topic[MAX_TOPIC_LENGTH];
            snprintf(state_topic, sizeof(state_topic), "%s/cameras/%s/detection_count",
                     topic_prefix, safe_name);
            cJSON_AddStringToObject(payload, "state_topic", state_topic);
            cJSON_AddStringToObject(payload, "icon", "mdi:motion-sensor");

            // Availability
            cJSON *avail = cJSON_CreateObject();
            char avail_topic[MAX_TOPIC_LENGTH];
            snprintf(avail_topic, sizeof(avail_topic), "%s/availability", topic_prefix);
            cJSON_AddStringToObject(avail, "topic", avail_topic);
            cJSON_AddStringToObject(avail, "payload_available", "online");
            cJSON_AddStringToObject(avail, "payload_not_available", "offline");
            cJSON *avail_list = cJSON_CreateArray();
            cJSON_AddItemToArray(avail_list, avail);
            cJSON_AddItemToObject(payload, "availability", avail_list);

            // Device & Origin
            cJSON *device = build_ha_device_block();
            cJSON *origin = build_ha_origin_block();
            if (!device || !origin) {
                cJSON_Delete(device);
                cJSON_Delete(origin);
                cJSON_Delete(payload);
                continue;
            }
            cJSON_AddItemToObject(payload, "device", device);
            cJSON_AddItemToObject(payload, "origin", origin);

            char *json_str = cJSON_PrintUnformatted(payload);
            cJSON_Delete(payload);
            if (json_str) {
                mqtt_publish_raw(topic, json_str, true);
                free(json_str);
                published++;
            }
        }

        // --- Publish initial states so entities don't show "Unknown" ---
        {
            char topic[MAX_TOPIC_LENGTH];

            // Motion: initially OFF
            snprintf(topic, sizeof(topic), "%s/cameras/%s/motion",
                     topic_prefix, safe_name);
            mqtt_publish_raw(topic, "OFF", false);

            // Detection count: initially 0
            snprintf(topic, sizeof(topic), "%s/cameras/%s/detection_count",
                     topic_prefix, safe_name);
            mqtt_publish_raw(topic, "0", false);
        }
    }

    log_info("MQTT HA: Published %d discovery messages for %d streams", published, num_streams);
    free(streams);
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
        safe_strcpy(state->stream_name, stream_name, sizeof(state->stream_name), 0);
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
                safe_strcpy(state->object_labels[label_idx], label, sizeof(state->object_labels[0]), 0);
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

    // Sanitize stream name for MQTT topics
    char safe_name[256];
    sanitize_stream_name(stream_name, safe_name, sizeof(safe_name));

    // Publish motion ON
    if (should_publish_on) {
        char topic[MAX_TOPIC_LENGTH];
        snprintf(topic, sizeof(topic), "%s/cameras/%s/motion",
                 mqtt_config->mqtt_topic_prefix, safe_name);
        mqtt_publish_raw(topic, "ON", false);
        log_debug("MQTT HA: Motion ON for %s", stream_name);
    }

    // Publish detection count
    {
        char topic[MAX_TOPIC_LENGTH];
        snprintf(topic, sizeof(topic), "%s/cameras/%s/detection_count",
                 mqtt_config->mqtt_topic_prefix, safe_name);
        char count_str[16];
        snprintf(count_str, sizeof(count_str), "%d", total_count);
        mqtt_publish_raw(topic, count_str, false);
    }

    // Publish per-object-class counts
    for (int i = 0; i < num_labels; i++) {
        char topic[MAX_TOPIC_LENGTH];
        snprintf(topic, sizeof(topic), "%s/cameras/%s/%s",
                 mqtt_config->mqtt_topic_prefix, safe_name, labels_copy[i]);
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
    log_set_thread_context("MQTT", NULL);
    log_info("MQTT HA: Snapshot publishing thread started (interval=%ds)",
             mqtt_config->mqtt_ha_snapshot_interval);

    // Heap-allocated once per-thread-lifetime: sizeof(stream_config_t)
    // exceeds 6 KB (12 KB after T11's go2rtc_source_override bump);
    // 16-entry stack array would exhaust musl/Alpine pthread stacks.
    stream_config_t *streams = calloc(MAX_MOTION_STREAMS, sizeof(stream_config_t));
    if (!streams) {
        log_error("MQTT HA: Failed to allocate streams buffer, snapshot thread exiting");
        return NULL;
    }

    while (ha_services_running) {
        if (!mqtt_is_connected() || !mqtt_config) {
            interruptible_sleep_wait(&ha_snapshot_wake, 1);
            continue;
        }

        int num_streams = get_all_stream_configs(streams, MAX_MOTION_STREAMS);

        for (int i = 0; i < num_streams && ha_services_running; i++) {
            if (!streams[i].enabled || streams[i].name[0] == '\0') {
                continue;
            }

            unsigned char *jpeg_data = NULL;
            size_t jpeg_size = 0;

            if (go2rtc_get_snapshot(streams[i].name, &jpeg_data, &jpeg_size)) {
                char safe_name[256];
                sanitize_stream_name(streams[i].name, safe_name, sizeof(safe_name));
                char topic[MAX_TOPIC_LENGTH];
                snprintf(topic, sizeof(topic), "%s/cameras/%s/snapshot",
                         mqtt_config->mqtt_topic_prefix, safe_name);
                mqtt_publish_binary(topic, jpeg_data, jpeg_size, false);
                log_debug("MQTT HA: Published snapshot for %s (%zu bytes)",
                          streams[i].name, jpeg_size);
                free(jpeg_data);
            } else {
                log_debug("MQTT HA: Failed to get snapshot for %s", streams[i].name);
            }
        }

        // Sleep until the next interval; mqtt_stop_ha_services() wakes us early.
        interruptible_sleep_wait(&ha_snapshot_wake, mqtt_config->mqtt_ha_snapshot_interval);
    }

    free(streams);
    go2rtc_snapshot_cleanup_thread();
    log_info("MQTT HA: Snapshot publishing thread stopped");
    return NULL;
}

/**
 * Background thread: checks motion states and publishes OFF after timeout.
 */
static void *ha_motion_thread_func(void *arg) {
    (void)arg;
    log_set_thread_context("MQTT", NULL);
    log_info("MQTT HA: Motion timeout thread started");

    while (ha_services_running) {
        if (!mqtt_is_connected() || !mqtt_config) {
            interruptible_sleep_wait(&ha_motion_wake, 1);
            continue;
        }

        time_t now = time(NULL);

        pthread_mutex_lock(&motion_mutex);
        for (int i = 0; i < num_motion_states; i++) {
            if (motion_states[i].motion_active &&
                (now - motion_states[i].last_detection_time) >= MOTION_OFF_DELAY_SEC) {

                motion_states[i].motion_active = false;
                char stream_name[256];
                safe_strcpy(stream_name, motion_states[i].stream_name, sizeof(stream_name), 0);

                char safe_name[256];
                sanitize_stream_name(stream_name, safe_name, sizeof(safe_name));

                pthread_mutex_unlock(&motion_mutex);

                // Publish motion OFF
                char topic[MAX_TOPIC_LENGTH];
                snprintf(topic, sizeof(topic), "%s/cameras/%s/motion",
                         mqtt_config->mqtt_topic_prefix, safe_name);
                mqtt_publish_raw(topic, "OFF", false);
                log_debug("MQTT HA: Motion OFF for %s (timeout)", stream_name);

                // Reset detection count to 0
                snprintf(topic, sizeof(topic), "%s/cameras/%s/detection_count",
                         mqtt_config->mqtt_topic_prefix, safe_name);
                mqtt_publish_raw(topic, "0", false);

                pthread_mutex_lock(&motion_mutex);
            }
        }
        pthread_mutex_unlock(&motion_mutex);

        interruptible_sleep_wait(&ha_motion_wake, 1); // Check ~every second
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
    ha_snapshot_thread_started = false;

    // Fresh wakeable-sleep handles for this run (HA services can be cycled).
    if (interruptible_sleep_init(&ha_snapshot_wake) != 0) {
        log_error("MQTT HA: Failed to initialize snapshot wake handle");
        ha_services_running = false;
        return -1;
    }
    if (interruptible_sleep_init(&ha_motion_wake) != 0) {
        log_error("MQTT HA: Failed to initialize motion wake handle");
        interruptible_sleep_destroy(&ha_snapshot_wake);
        ha_services_running = false;
        return -1;
    }

    // Start snapshot publishing thread if interval > 0
    if (mqtt_config->mqtt_ha_snapshot_interval > 0) {
        if (pthread_create(&ha_snapshot_thread, NULL, ha_snapshot_thread_func, NULL) != 0) {
            log_error("MQTT HA: Failed to create snapshot thread");
            ha_services_running = false;
            interruptible_sleep_destroy(&ha_snapshot_wake);
            interruptible_sleep_destroy(&ha_motion_wake);
            return -1;
        }
        ha_snapshot_thread_started = true;
        log_info("MQTT HA: Snapshot publishing started (interval=%ds)",
                 mqtt_config->mqtt_ha_snapshot_interval);
    }

    // Start motion timeout thread
    if (pthread_create(&ha_motion_thread, NULL, ha_motion_thread_func, NULL) != 0) {
        log_error("MQTT HA: Failed to create motion timeout thread");
        // Signal any already-started HA service threads to stop
        ha_services_running = false;
        // If the snapshot thread was started, wake and wait for it to exit
        if (ha_snapshot_thread_started) {
            interruptible_sleep_wake(&ha_snapshot_wake);
            pthread_join(ha_snapshot_thread, NULL);
            ha_snapshot_thread_started = false;
        }
        interruptible_sleep_destroy(&ha_snapshot_wake);
        interruptible_sleep_destroy(&ha_motion_wake);
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

    // Wake both threads out of their interval sleeps so the joins return now.
    interruptible_sleep_wake(&ha_snapshot_wake);
    interruptible_sleep_wake(&ha_motion_wake);

    // Wait for threads to finish
    if (ha_snapshot_thread_started) {
        pthread_join(ha_snapshot_thread, NULL);
        ha_snapshot_thread_started = false;
    }
    pthread_join(ha_motion_thread, NULL);

    interruptible_sleep_destroy(&ha_snapshot_wake);
    interruptible_sleep_destroy(&ha_motion_wake);

    // Reset motion state tracking to avoid stale states on reinit
    pthread_mutex_lock(&motion_mutex);
    num_motion_states = 0;
    memset(motion_states, 0, sizeof(motion_states));
    pthread_mutex_unlock(&motion_mutex);

    log_info("MQTT HA: Background services stopped");
}

// Cleanup operation types
typedef enum {
    MQTT_OP_LOOP_STOP,
    MQTT_OP_DESTROY,
    MQTT_OP_LIB_CLEANUP
} mqtt_cleanup_op_t;

// Thread argument structure - completion flag is heap-allocated and owned by worker thread
typedef struct {
    struct mosquitto *mosq;
    mqtt_cleanup_op_t op;
    int *done_flag;  // Pointer to heap-allocated completion flag
} mqtt_cleanup_arg_t;

/**
 * Thread function to run blocking mosquitto operations with timeout capability
 */
static void *mqtt_cleanup_thread(void *arg) {
    log_set_thread_context("MQTT", NULL);
    mqtt_cleanup_arg_t *cleanup_arg = (mqtt_cleanup_arg_t *)arg;
    int *done_flag = cleanup_arg->done_flag;

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

    // Signal completion and release heap-owned resources.
    *done_flag = 1;
    free(done_flag);
    free(cleanup_arg);

    return NULL;
}

/**
 * Run a mosquitto cleanup operation with a timeout
 * Uses simple polling with usleep - portable across glibc and musl
 * Returns true if completed within timeout, false if timed out
 */
static bool mqtt_run_with_timeout(struct mosquitto *m, mqtt_cleanup_op_t op, int timeout_sec, const char *op_name) {
    pthread_t thread;
    int *done_flag = (int *)calloc(1, sizeof(int));
    mqtt_cleanup_arg_t *arg = (mqtt_cleanup_arg_t *)calloc(1, sizeof(mqtt_cleanup_arg_t));

    if (done_flag == NULL || arg == NULL) {
        free(done_flag);
        free(arg);
        log_warn("MQTT: Failed to allocate resources for %s", op_name);
        return false;
    }

    arg->mosq = m;
    arg->op = op;
    arg->done_flag = done_flag;

    // Create thread to run the operation
    if (pthread_create(&thread, NULL, mqtt_cleanup_thread, arg) != 0) {
        log_warn("MQTT: Failed to create thread for %s", op_name);
        free(done_flag);
        free(arg);
        return false;
    }

    // Detach the thread so it cleans up automatically if we timeout
    pthread_detach(thread);

    // Poll for completion with 50ms intervals
    int timeout_ms = timeout_sec * 1000;
    int elapsed_ms = 0;
    const int poll_interval_ms = 50;

    while (elapsed_ms < timeout_ms) {
        if (*done_flag) {
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

/**
 * Reinitialize MQTT client with current configuration.
 * Used for hot-reload when settings change from the web UI.
 *
 * @param config Pointer to the (updated) application configuration
 * @return 0 on success, -1 on failure
 */
int mqtt_reinit(const config_t *config) {
    if (!config) {
        log_error("MQTT reinit: Invalid config pointer");
        return -1;
    }

    log_info("MQTT reinit: Starting hot-reload...");

    // Step 1: Full cleanup of existing MQTT state
    // (mqtt_cleanup sets shutting_down = true and tears everything down)
    mqtt_cleanup();

    // Step 2: Reset the shutting_down flag so callbacks work again
    shutting_down = false;
    __sync_synchronize();

    // Step 3: If MQTT is now disabled, we're done
    if (!config->mqtt_enabled) {
        log_info("MQTT reinit: MQTT is disabled, cleanup complete");
        return 0;
    }

    // Step 4: Re-initialize with the updated config
    if (mqtt_init(config) != 0) {
        log_error("MQTT reinit: Failed to initialize MQTT client");
        return -1;
    }

    // Step 5: Connect to broker
    if (mqtt_connect() != 0) {
        log_warn("MQTT reinit: Failed to connect to MQTT broker, will retry automatically");
        // Not a fatal error — mosquitto loop thread will retry
    } else {
        log_info("MQTT reinit: Connected to MQTT broker");

        // on_connect callback will publish HA discovery and start services if enabled
    }

    log_info("MQTT reinit: Hot-reload complete");
    return 0;
}

#endif /* ENABLE_MQTT */
