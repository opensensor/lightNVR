#ifdef ENABLE_MQTT

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <errno.h>
#include <pthread.h>
#include <signal.h>
#include <mosquitto.h>
#include <cjson/cJSON.h>

#include "core/mqtt_client.h"
#include "core/logger.h"

// MQTT client state
static struct mosquitto *mosq = NULL;
static const config_t *mqtt_config = NULL;
static bool connected = false;
static pthread_mutex_t mqtt_mutex = PTHREAD_MUTEX_INITIALIZER;

// Cleanup thread state - use a generation counter to avoid race conditions
// between timed-out operations and subsequent ones
static volatile int cleanup_generation = 0;
static volatile int cleanup_completed_generation = -1;
static pthread_mutex_t cleanup_mutex = PTHREAD_MUTEX_INITIALIZER;
static pthread_cond_t cleanup_cond = PTHREAD_COND_INITIALIZER;

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
    
    pthread_mutex_lock(&mqtt_mutex);
    if (rc == 0) {
        connected = true;
        log_info("MQTT: Connected to broker successfully");
    } else {
        connected = false;
        log_error("MQTT: Connection failed: %s", mosquitto_connack_string(rc));
    }
    pthread_mutex_unlock(&mqtt_mutex);
}

// Disconnection callback
static void on_disconnect(struct mosquitto *m, void *userdata, int rc) {
    (void)m;
    (void)userdata;

    pthread_mutex_lock(&mqtt_mutex);
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

// Cleanup operation types
typedef enum {
    MQTT_OP_LOOP_STOP,
    MQTT_OP_DESTROY,
    MQTT_OP_LIB_CLEANUP
} mqtt_cleanup_op_t;

// Thread argument structure
typedef struct {
    struct mosquitto *mosq;
    mqtt_cleanup_op_t op;
    int generation;  // Used to identify which operation completed
} mqtt_cleanup_arg_t;

/**
 * Thread function to run blocking mosquitto operations with timeout capability
 */
static void *mqtt_cleanup_thread(void *arg) {
    mqtt_cleanup_arg_t *cleanup_arg = (mqtt_cleanup_arg_t *)arg;
    int my_generation = cleanup_arg->generation;

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

    // Signal that cleanup is done for this specific generation
    pthread_mutex_lock(&cleanup_mutex);
    cleanup_completed_generation = my_generation;
    pthread_cond_signal(&cleanup_cond);
    pthread_mutex_unlock(&cleanup_mutex);

    free(cleanup_arg);
    return NULL;
}

/**
 * Run a mosquitto cleanup operation with a timeout
 * Returns true if completed within timeout, false if timed out
 */
static bool mqtt_run_with_timeout(struct mosquitto *m, mqtt_cleanup_op_t op, int timeout_sec, const char *op_name) {
    pthread_t thread;

    // Increment generation counter to uniquely identify this operation
    // This prevents race conditions where a previously timed-out operation
    // signals completion and interferes with a new operation
    pthread_mutex_lock(&cleanup_mutex);
    int my_generation = ++cleanup_generation;
    pthread_mutex_unlock(&cleanup_mutex);

    // Allocate argument structure (will be freed by thread)
    mqtt_cleanup_arg_t *arg = malloc(sizeof(mqtt_cleanup_arg_t));
    if (!arg) {
        log_warn("MQTT: Failed to allocate cleanup arg for %s", op_name);
        return false;
    }
    arg->mosq = m;
    arg->op = op;
    arg->generation = my_generation;

    // Create thread to run the operation
    if (pthread_create(&thread, NULL, mqtt_cleanup_thread, arg) != 0) {
        log_warn("MQTT: Failed to create thread for %s", op_name);
        free(arg);
        return false;
    }

    // Detach the thread so it cleans up automatically
    pthread_detach(thread);

    // Wait for completion with timeout
    struct timespec ts;
    clock_gettime(CLOCK_REALTIME, &ts);
    ts.tv_sec += timeout_sec;

    pthread_mutex_lock(&cleanup_mutex);
    while (cleanup_completed_generation != my_generation) {
        int rc = pthread_cond_timedwait(&cleanup_cond, &cleanup_mutex, &ts);
        if (rc == ETIMEDOUT) {
            pthread_mutex_unlock(&cleanup_mutex);
            log_warn("MQTT: %s timed out after %d seconds", op_name, timeout_sec);
            return false;
        }
        // Check again after waking - might have been signaled by a different operation
    }
    pthread_mutex_unlock(&cleanup_mutex);

    return true;
}

/**
 * Disconnect from the MQTT broker
 */
void mqtt_disconnect(void) {
    if (!mosq) {
        log_info("MQTT: No client to disconnect");
        return;
    }

    log_info("MQTT: Attempting to acquire mutex for disconnect...");

    // Try to lock with timeout to avoid blocking during shutdown
    struct timespec timeout;
    clock_gettime(CLOCK_REALTIME, &timeout);
    timeout.tv_sec += 2;  // 2 second timeout

    int lock_result = pthread_mutex_timedlock(&mqtt_mutex, &timeout);
    if (lock_result != 0) {
        log_warn("MQTT: Failed to acquire mutex for disconnect (timeout or error), proceeding anyway");
        // Force disconnect without mutex - risky but better than hanging
        mosquitto_disconnect(mosq);
        mqtt_run_with_timeout(mosq, MQTT_OP_LOOP_STOP, 2, "mosquitto_loop_stop");
        connected = false;
        log_info("MQTT: Forced disconnect without mutex");
        return;
    }

    log_info("MQTT: Calling mosquitto_disconnect...");
    mosquitto_disconnect(mosq);

    log_info("MQTT: Calling mosquitto_loop_stop with 2 second timeout...");
    mqtt_run_with_timeout(mosq, MQTT_OP_LOOP_STOP, 2, "mosquitto_loop_stop");

    connected = false;

    pthread_mutex_unlock(&mqtt_mutex);

    log_info("MQTT: Disconnected");
}

/**
 * Cleanup MQTT resources
 */
void mqtt_cleanup(void) {
    log_info("MQTT: Starting cleanup...");

    if (!mosq) {
        log_info("MQTT: No client to clean up");
        return;
    }

    mqtt_disconnect();

    log_info("MQTT: Attempting to acquire mutex for destroy...");

    // Try to lock with timeout to avoid blocking during shutdown
    struct timespec timeout;
    clock_gettime(CLOCK_REALTIME, &timeout);
    timeout.tv_sec += 2;  // 2 second timeout

    int lock_result = pthread_mutex_timedlock(&mqtt_mutex, &timeout);
    if (lock_result != 0) {
        log_warn("MQTT: Failed to acquire mutex for destroy (timeout or error), skipping destroy");
        // Skip destroy entirely - it's blocking and we're shutting down anyway
        mosq = NULL;
        mqtt_config = NULL;
        log_info("MQTT: Skipped destroy due to mutex timeout");
        return;
    }

    log_info("MQTT: Calling mosquitto_destroy with 2 second timeout...");
    bool destroyed = mqtt_run_with_timeout(mosq, MQTT_OP_DESTROY, 2, "mosquitto_destroy");
    if (destroyed) {
        mosq = NULL;
    } else {
        log_warn("MQTT: mosquitto_destroy timed out, setting mosq to NULL anyway");
        mosq = NULL;
    }
    mqtt_config = NULL;
    pthread_mutex_unlock(&mqtt_mutex);

    log_info("MQTT: Calling mosquitto_lib_cleanup with 2 second timeout...");
    mqtt_run_with_timeout(NULL, MQTT_OP_LIB_CLEANUP, 2, "mosquitto_lib_cleanup");
    log_info("MQTT: Cleaned up");
}

#endif /* ENABLE_MQTT */

