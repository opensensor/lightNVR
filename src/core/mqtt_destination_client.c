#define _POSIX_C_SOURCE 200809L

#include "core/mqtt_destination_client.h"

#ifdef ENABLE_MQTT

#include <errno.h>
#include <limits.h>
#include <mosquitto.h>
#include <pthread.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#define LOG_COMPONENT "MQTTDestination"
#include "core/logger.h"
#include "utils/memory.h"

struct mqtt_destination_client {
    event_destination_t profile;
    struct mosquitto *mosq;
    pthread_mutex_t mutex;
    pthread_cond_t wake;
    bool synchronization_ready;
    bool loop_started;
    bool connected;
    bool shutting_down;
    bool publish_waiting;
    bool publish_acknowledged;
    bool publish_failed;
    int publish_mid;
};

static void on_connect(struct mosquitto *mosq, void *userdata, int result) {
    (void)mosq;
    mqtt_destination_client_t *client = userdata;
    if (!client) return;
    pthread_mutex_lock(&client->mutex);
    if (!client->shutting_down) client->connected = result == 0;
    pthread_mutex_unlock(&client->mutex);
    if (result == 0) {
        log_info("Managed destination '%s' connected to %s:%d",
                 client->profile.name, client->profile.broker_host,
                 client->profile.broker_port);
    } else {
        log_warn("Managed destination '%s' connection refused: %s",
                 client->profile.name, mosquitto_connack_string(result));
    }
}

static void on_disconnect(struct mosquitto *mosq, void *userdata, int result) {
    (void)mosq;
    mqtt_destination_client_t *client = userdata;
    if (!client) return;
    pthread_mutex_lock(&client->mutex);
    client->connected = false;
    if (client->publish_waiting) {
        client->publish_failed = true;
        pthread_cond_broadcast(&client->wake);
    }
    bool shutting_down = client->shutting_down;
    pthread_mutex_unlock(&client->mutex);
    if (!shutting_down && result != 0) {
        log_warn("Managed destination '%s' disconnected (rc=%d); reconnecting",
                 client->profile.name, result);
    }
}

static void on_publish(struct mosquitto *mosq, void *userdata, int mid) {
    (void)mosq;
    mqtt_destination_client_t *client = userdata;
    if (!client) return;
    pthread_mutex_lock(&client->mutex);
    if (client->publish_waiting && client->publish_mid == mid) {
        client->publish_acknowledged = true;
        pthread_cond_broadcast(&client->wake);
    }
    pthread_mutex_unlock(&client->mutex);
}

static void on_log(struct mosquitto *mosq, void *userdata, int level,
                   const char *message) {
    (void)mosq;
    mqtt_destination_client_t *client = userdata;
    if (!client || !message) return;
    if (level == MOSQ_LOG_ERR) {
        log_error("Managed destination '%s': %s", client->profile.name,
                  message);
    } else if (level == MOSQ_LOG_WARNING) {
        log_warn("Managed destination '%s': %s", client->profile.name,
                 message);
    } else {
        log_debug("Managed destination '%s': %s", client->profile.name,
                  message);
    }
}

static int configure_tls(mqtt_destination_client_t *client) {
    if (strcmp(client->profile.tls_mode, "disabled") == 0) return 0;
    bool system_trust = strcmp(client->profile.tls_mode, "system") == 0;
    int result = MOSQ_ERR_SUCCESS;
    if (system_trust) {
        result = mosquitto_int_option(
            client->mosq, MOSQ_OPT_TLS_USE_OS_CERTS, 1);
    } else {
        bool mutual = strcmp(client->profile.tls_mode, "mutual") == 0;
        result = mosquitto_tls_set(
            client->mosq, client->profile.ca_file, NULL,
            mutual ? client->profile.cert_file : NULL,
            mutual ? client->profile.key_file : NULL, NULL);
    }
    if (result == MOSQ_ERR_SUCCESS) {
        result = mosquitto_tls_opts_set(client->mosq, 1, "tlsv1.2", NULL);
    }
    return result;
}

static void destroy_partial(mqtt_destination_client_t *client) {
    if (!client) return;
    if (client->loop_started && client->mosq) {
        mosquitto_disconnect(client->mosq);
        mosquitto_loop_stop(client->mosq, true);
    }
    if (client->mosq) mosquitto_destroy(client->mosq);
    if (client->synchronization_ready) {
        pthread_cond_destroy(&client->wake);
        pthread_mutex_destroy(&client->mutex);
    }
    memset(client, 0, sizeof(*client));
    free(client);
}

mqtt_destination_client_t *mqtt_destination_client_create(
    const event_destination_t *profile) {
    char validation_error[EVENT_DESTINATION_VALIDATION_ERROR_MAX] = {0};
    if (!profile || !profile->enabled ||
        db_event_destination_validate(profile, NULL, false, validation_error,
                                      sizeof(validation_error)) !=
            DB_EVENT_DESTINATION_OK) {
        return NULL;
    }
    mqtt_destination_client_t *client = calloc(1, sizeof(*client));
    if (!client) return NULL;
    client->profile = *profile;
    client->publish_mid = -1;
    if (pthread_mutex_init(&client->mutex, NULL) != 0) {
        free(client);
        return NULL;
    }
    if (pthread_cond_init(&client->wake, NULL) != 0) {
        pthread_mutex_destroy(&client->mutex);
        free(client);
        return NULL;
    }
    client->synchronization_ready = true;

    char password[EVENT_DESTINATION_PASSWORD_MAX] = {0};
    db_event_destination_result_t password_result =
        db_event_destination_get_password(
            profile->uuid, profile->revision, password, sizeof(password));
    if (password_result != DB_EVENT_DESTINATION_OK) {
        secure_zero_memory(password, sizeof(password));
        destroy_partial(client);
        return NULL;
    }

    client->mosq = mosquitto_new(profile->client_id, true, client);
    if (!client->mosq) {
        secure_zero_memory(password, sizeof(password));
        destroy_partial(client);
        return NULL;
    }
    mosquitto_connect_callback_set(client->mosq, on_connect);
    mosquitto_disconnect_callback_set(client->mosq, on_disconnect);
    mosquitto_publish_callback_set(client->mosq, on_publish);
    mosquitto_log_callback_set(client->mosq, on_log);

    int result = MOSQ_ERR_SUCCESS;
    if (profile->username[0]) {
        result = mosquitto_username_pw_set(
            client->mosq, profile->username,
            password[0] ? password : NULL);
    }
    secure_zero_memory(password, sizeof(password));
    if (result == MOSQ_ERR_SUCCESS) result = configure_tls(client);
    if (result == MOSQ_ERR_SUCCESS) {
        result = mosquitto_reconnect_delay_set(client->mosq, 1, 30, true);
    }
    if (result == MOSQ_ERR_SUCCESS) {
        result = mosquitto_connect_async(
            client->mosq, profile->broker_host, profile->broker_port,
            profile->keepalive_seconds);
    }
    if (result == MOSQ_ERR_SUCCESS) {
        result = mosquitto_loop_start(client->mosq);
        client->loop_started = result == MOSQ_ERR_SUCCESS;
    }
    if (result != MOSQ_ERR_SUCCESS) {
        log_error("Could not initialize managed destination '%s': %s",
                  profile->name, mosquitto_strerror(result));
        destroy_partial(client);
        return NULL;
    }
    return client;
}

void mqtt_destination_client_destroy(mqtt_destination_client_t *client) {
    if (!client) return;
    pthread_mutex_lock(&client->mutex);
    client->shutting_down = true;
    client->connected = false;
    if (client->publish_waiting) {
        client->publish_failed = true;
        pthread_cond_broadcast(&client->wake);
    }
    pthread_mutex_unlock(&client->mutex);
    destroy_partial(client);
}

bool mqtt_destination_client_is_connected(
    mqtt_destination_client_t *client) {
    if (!client) return false;
    pthread_mutex_lock(&client->mutex);
    bool connected = client->connected && !client->shutting_down;
    pthread_mutex_unlock(&client->mutex);
    return connected;
}

int mqtt_destination_client_publish_confirmed(
    mqtt_destination_client_t *client, const char *topic,
    const char *payload, bool retain, int timeout_ms) {
    if (!client || !topic || !payload || timeout_ms <= 0) return -1;
    size_t payload_length = strlen(payload);
    if (payload_length > INT_MAX) return -1;

    pthread_mutex_lock(&client->mutex);
    if (!client->connected || client->shutting_down ||
        client->publish_waiting) {
        pthread_mutex_unlock(&client->mutex);
        return -1;
    }
    client->publish_waiting = true;
    client->publish_acknowledged = false;
    client->publish_failed = false;
    client->publish_mid = -1;
    int mid = -1;
    int result = mosquitto_publish(
        client->mosq, &mid, topic, (int)payload_length, payload,
        client->profile.qos, retain);
    client->publish_mid = mid;
    if (result != MOSQ_ERR_SUCCESS) {
        client->publish_waiting = false;
        client->publish_mid = -1;
        pthread_mutex_unlock(&client->mutex);
        return -1;
    }

    struct timespec deadline;
    clock_gettime(CLOCK_REALTIME, &deadline);
    deadline.tv_sec += timeout_ms / 1000;
    deadline.tv_nsec += (long)(timeout_ms % 1000) * 1000000L;
    if (deadline.tv_nsec >= 1000000000L) {
        deadline.tv_sec++;
        deadline.tv_nsec -= 1000000000L;
    }
    while (!client->publish_acknowledged && !client->publish_failed) {
        result = pthread_cond_timedwait(
            &client->wake, &client->mutex, &deadline);
        if (result == ETIMEDOUT) break;
        if (result != 0) {
            client->publish_failed = true;
            break;
        }
    }
    bool acknowledged = client->publish_acknowledged &&
        !client->publish_failed;
    client->publish_waiting = false;
    client->publish_acknowledged = false;
    client->publish_failed = false;
    client->publish_mid = -1;
    pthread_mutex_unlock(&client->mutex);
    return acknowledged ? 0 : -1;
}

#else

struct mqtt_destination_client { int unused; };

mqtt_destination_client_t *mqtt_destination_client_create(
    const event_destination_t *profile) {
    (void)profile;
    return NULL;
}

void mqtt_destination_client_destroy(mqtt_destination_client_t *client) {
    (void)client;
}

bool mqtt_destination_client_is_connected(
    mqtt_destination_client_t *client) {
    (void)client;
    return false;
}

int mqtt_destination_client_publish_confirmed(
    mqtt_destination_client_t *client, const char *topic,
    const char *payload, bool retain, int timeout_ms) {
    (void)client;
    (void)topic;
    (void)payload;
    (void)retain;
    (void)timeout_ms;
    return -1;
}

#endif
