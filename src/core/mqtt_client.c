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

// MQTT client state
static struct mosquitto *mosq = NULL;
static const config_t *mqtt_config = NULL;
static bool connected = false;
static volatile bool shutting_down = false;  // Flag to prevent callbacks from acquiring mutex during shutdown
static pthread_mutex_t mqtt_mutex = PTHREAD_MUTEX_INITIALIZER;

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

