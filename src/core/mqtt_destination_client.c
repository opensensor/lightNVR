#define _POSIX_C_SOURCE 200809L

#include "core/mqtt_destination_client.h"

#ifdef ENABLE_MQTT

#include <errno.h>
#include <inttypes.h>
#include <limits.h>
#include <mosquitto.h>
#include <pthread.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#define LOG_COMPONENT "MQTTDestination"
#include "core/event_identity.h"
#include "core/logger.h"
#include "core/mqtt_presence.h"
#include "core/version.h"
#include "telemetry/system_health.h"
#include "telemetry/system_health_evaluator.h"
#include "utils/memory.h"
#include "utils/uuid.h"

#ifndef MANAGED_PRESENCE_INTERVAL_SECONDS
#define MANAGED_PRESENCE_INTERVAL_SECONDS 60
#endif
#define MANAGED_PRESENCE_VISIBILITY "process,container,host,filesystem"

typedef struct {
    bool occupied;
    mqtt_destination_client_stats_t value;
} managed_stats_slot_t;

static pthread_mutex_t stats_lock = PTHREAD_MUTEX_INITIALIZER;
static managed_stats_slot_t stats_slots[EVENT_DESTINATION_MAX_COUNT];

struct mqtt_destination_client {
    event_destination_t profile;
    struct mosquitto *mosq;
    pthread_mutex_t mutex;
    pthread_cond_t wake;
    pthread_cond_t heartbeat_wake;
    pthread_t heartbeat_thread;
    bool synchronization_ready;
    bool heartbeat_condition_ready;
    bool heartbeat_started;
    bool heartbeat_stop;
    bool loop_started;
    bool connected;
    bool ever_connected;
    bool shutting_down;
    bool publish_waiting;
    bool publish_acknowledged;
    bool publish_failed;
    bool presence_configured;
    int publish_mid;
    int stats_slot;
    uint64_t presence_sequence;
    char status_topic[MQTT_PRESENCE_TOPIC_MAX];
    char installation_uuid[LIGHTNVR_UUID_STRING_SIZE];
    char run_id[LIGHTNVR_UUID_STRING_SIZE];
    char boot_id[SYSTEM_HEALTH_INCIDENT_BOOT_ID_MAX];
};

static uint64_t increment_saturated(uint64_t value) {
    return value == UINT64_MAX ? value : value + 1U;
}

static int64_t wall_time_ms(void) {
    struct timespec value;
    if (clock_gettime(CLOCK_REALTIME, &value) != 0) return 0;
    return (int64_t)value.tv_sec * 1000 + value.tv_nsec / 1000000;
}

static int reserve_stats_slot(const event_destination_t *profile) {
    int index = -1;
    pthread_mutex_lock(&stats_lock);
    for (int current = 0; current < EVENT_DESTINATION_MAX_COUNT; ++current) {
        if (!stats_slots[current].occupied) {
            index = current;
            memset(&stats_slots[current], 0, sizeof(stats_slots[current]));
            stats_slots[current].occupied = true;
            snprintf(stats_slots[current].value.destination_uuid,
                     sizeof(stats_slots[current].value.destination_uuid),
                     "%s", profile->uuid);
            stats_slots[current].value.presence_configured =
                profile->status_topic_template[0] != '\0';
            break;
        }
    }
    pthread_mutex_unlock(&stats_lock);
    return index;
}

static void release_stats_slot(int index) {
    if (index < 0 || index >= EVENT_DESTINATION_MAX_COUNT) return;
    pthread_mutex_lock(&stats_lock);
    memset(&stats_slots[index], 0, sizeof(stats_slots[index]));
    pthread_mutex_unlock(&stats_lock);
}

static void stats_connection(int index, bool connected, bool reconnect) {
    if (index < 0 || index >= EVENT_DESTINATION_MAX_COUNT) return;
    pthread_mutex_lock(&stats_lock);
    if (stats_slots[index].occupied) {
        mqtt_destination_client_stats_t *stats = &stats_slots[index].value;
        stats->connected = connected;
        if (connected) {
            stats->connections = increment_saturated(stats->connections);
            if (reconnect)
                stats->reconnects = increment_saturated(stats->reconnects);
        } else {
            stats->disconnects = increment_saturated(stats->disconnects);
        }
    }
    pthread_mutex_unlock(&stats_lock);
}

static void stats_publish(int index, bool succeeded, int64_t observed_at_ms) {
    if (index < 0 || index >= EVENT_DESTINATION_MAX_COUNT) return;
    pthread_mutex_lock(&stats_lock);
    if (stats_slots[index].occupied) {
        mqtt_destination_client_stats_t *stats = &stats_slots[index].value;
        if (succeeded) {
            stats->publish_successes =
                increment_saturated(stats->publish_successes);
            if (observed_at_ms > 0)
                stats->last_success_at_ms = observed_at_ms;
        } else {
            stats->publish_failures =
                increment_saturated(stats->publish_failures);
            stats->last_failure = MQTT_DESTINATION_FAILURE_PUBLICATION;
            if (observed_at_ms > 0)
                stats->last_failure_at_ms = observed_at_ms;
        }
    }
    pthread_mutex_unlock(&stats_lock);
}

static void stats_failure(int index, mqtt_destination_failure_t failure) {
    if (index < 0 || index >= EVENT_DESTINATION_MAX_COUNT) return;
    pthread_mutex_lock(&stats_lock);
    if (stats_slots[index].occupied) {
        mqtt_destination_client_stats_t *stats = &stats_slots[index].value;
        if (failure == MQTT_DESTINATION_FAILURE_CONNECTION) {
            stats->connection_failures =
                increment_saturated(stats->connection_failures);
        }
        stats->last_failure = failure;
        stats->last_failure_at_ms = wall_time_ms();
    }
    pthread_mutex_unlock(&stats_lock);
}

static bool replace_placeholder(char *output, size_t capacity, size_t *used,
                                const char *value) {
    size_t length = strlen(value);
    if (*used + length >= capacity) return false;
    memcpy(output + *used, value, length);
    *used += length;
    return true;
}

static int expand_status_topic(const event_destination_t *profile,
                               const char *installation_uuid,
                               char *topic, size_t topic_size) {
    if (!profile || !installation_uuid || !topic || topic_size == 0U ||
        !lightnvr_uuid_is_valid(installation_uuid) ||
        !lightnvr_uuid_is_valid(profile->uuid)) return -1;
    size_t used = 0U;
    const char *input = profile->status_topic_template;
    while (*input) {
        if (strncmp(input,
                    EVENT_DESTINATION_STATUS_INSTALLATION_PLACEHOLDER,
                    sizeof(EVENT_DESTINATION_STATUS_INSTALLATION_PLACEHOLDER)
                        - 1U) == 0) {
            if (!replace_placeholder(topic, topic_size, &used,
                                     installation_uuid)) return -1;
            input += sizeof(
                EVENT_DESTINATION_STATUS_INSTALLATION_PLACEHOLDER) - 1U;
        } else if (strncmp(
                       input,
                       EVENT_DESTINATION_STATUS_DESTINATION_PLACEHOLDER,
                       sizeof(
                           EVENT_DESTINATION_STATUS_DESTINATION_PLACEHOLDER)
                           - 1U) == 0) {
            if (!replace_placeholder(topic, topic_size, &used, profile->uuid))
                return -1;
            input += sizeof(
                EVENT_DESTINATION_STATUS_DESTINATION_PLACEHOLDER) - 1U;
        } else {
            if (used + 1U >= topic_size) return -1;
            topic[used++] = *input++;
        }
    }
    topic[used] = '\0';
    return used > 0U ? 0 : -1;
}

static const char *presence_state_name(mqtt_presence_state_t state) {
    switch (state) {
        case MQTT_PRESENCE_OFFLINE: return "offline";
        case MQTT_PRESENCE_ONLINE: return "online";
        case MQTT_PRESENCE_STOPPING: return "stopping";
        default: return NULL;
    }
}

static const char *operational_state_name(mqtt_operational_state_t state) {
    switch (state) {
        case MQTT_OPERATIONAL_UNKNOWN: return "unknown";
        case MQTT_OPERATIONAL_HEALTHY: return "healthy";
        case MQTT_OPERATIONAL_WARNING: return "warning";
        case MQTT_OPERATIONAL_ERROR: return "error";
        case MQTT_OPERATIONAL_CRITICAL: return "critical";
        default: return NULL;
    }
}

static mqtt_operational_state_t current_operational_state(
    size_t *active_count) {
    if (active_count) *active_count = 0U;
    system_health_snapshot_t snapshot;
    bool baseline_known = system_health_snapshot_copy(&snapshot) &&
                          snapshot.sequence > 0U &&
                          snapshot.observation_count > 0U;
    system_health_incident_view_t incidents[SYSTEM_HEALTH_MAX_INCIDENTS];
    size_t count = system_health_evaluator_service_active_copy(
        incidents, SYSTEM_HEALTH_MAX_INCIDENTS);
    if (active_count) *active_count = count;
    system_health_severity_t maximum = SYSTEM_HEALTH_SEVERITY_NONE;
    for (size_t index = 0U; index < count; ++index) {
        if (incidents[index].severity > maximum)
            maximum = incidents[index].severity;
    }
    if (maximum == SYSTEM_HEALTH_SEVERITY_CRITICAL)
        return MQTT_OPERATIONAL_CRITICAL;
    if (maximum == SYSTEM_HEALTH_SEVERITY_ERROR)
        return MQTT_OPERATIONAL_ERROR;
    if (maximum == SYSTEM_HEALTH_SEVERITY_WARNING)
        return MQTT_OPERATIONAL_WARNING;
    return baseline_known ? MQTT_OPERATIONAL_HEALTHY
                          : MQTT_OPERATIONAL_UNKNOWN;
}

static int build_presence_payload(mqtt_destination_client_t *client,
                                  mqtt_presence_state_t state,
                                  mqtt_operational_state_t operational,
                                  size_t active_incidents,
                                  int64_t observed_at_ms, uint64_t sequence,
                                  char *payload, size_t payload_size) {
    const char *state_name = presence_state_name(state);
    const char *operational_name = operational_state_name(operational);
    if (!client || !state_name || !operational_name || observed_at_ms <= 0 ||
        !payload || payload_size == 0U) return -1;
    int written = snprintf(
        payload, payload_size,
        "{\"schema_version\":1,\"installation_uuid\":\"%s\","
        "\"run_id\":\"%s\",\"boot_id\":\"%s\","
        "\"visibility_scope\":\"%s\",\"version\":\"%s\","
        "\"sequence\":%" PRIu64 ",\"timestamp_ms\":%" PRId64 ","
        "\"state\":\"%s\",\"overall_state\":\"%s\","
        "\"active_incidents\":%zu}",
        client->installation_uuid, client->run_id, client->boot_id,
        MANAGED_PRESENCE_VISIBILITY, LIGHTNVR_VERSION_STRING, sequence,
        observed_at_ms, state_name, operational_name, active_incidents);
    if (written < 0 || (size_t)written >= payload_size) {
        payload[0] = '\0';
        return -1;
    }
    return 0;
}

static int publish_presence(mqtt_destination_client_t *client,
                            mqtt_presence_state_t state) {
    if (!client || !client->presence_configured) return 0;
    size_t active_count = 0U;
    mqtt_operational_state_t operational =
        current_operational_state(&active_count);
    int64_t now = wall_time_ms();
    char payload[MQTT_PRESENCE_PAYLOAD_MAX];

    pthread_mutex_lock(&client->mutex);
    if ((!client->connected && state != MQTT_PRESENCE_OFFLINE) ||
        (client->shutting_down && state != MQTT_PRESENCE_STOPPING)) {
        pthread_mutex_unlock(&client->mutex);
        return -1;
    }
    uint64_t sequence = state == MQTT_PRESENCE_OFFLINE
        ? 0U : increment_saturated(client->presence_sequence);
    if (build_presence_payload(client, state, operational, active_count, now,
                               sequence, payload, sizeof(payload)) != 0) {
        pthread_mutex_unlock(&client->mutex);
        stats_publish(client->stats_slot, false, now);
        return -1;
    }
    int result = mosquitto_publish(
        client->mosq, NULL, client->status_topic, (int)strlen(payload),
        payload, client->profile.qos, true);
    if (result == MOSQ_ERR_SUCCESS && state != MQTT_PRESENCE_OFFLINE)
        client->presence_sequence = sequence;
    pthread_mutex_unlock(&client->mutex);
    stats_publish(client->stats_slot, result == MOSQ_ERR_SUCCESS, now);
    return result == MOSQ_ERR_SUCCESS ? 0 : -1;
}

static void on_connect(struct mosquitto *mosq, void *userdata, int result) {
    (void)mosq;
    mqtt_destination_client_t *client = userdata;
    if (!client) return;
    pthread_mutex_lock(&client->mutex);
    bool reconnect = client->ever_connected;
    bool accepted = !client->shutting_down && result == 0;
    client->connected = accepted;
    if (accepted) client->ever_connected = true;
    pthread_mutex_unlock(&client->mutex);
    if (accepted) {
        stats_connection(client->stats_slot, true, reconnect);
        (void)publish_presence(client, MQTT_PRESENCE_ONLINE);
        log_info("Managed destination '%s' connected to %s:%d",
                 client->profile.name, client->profile.broker_host,
                 client->profile.broker_port);
    } else {
        stats_failure(client->stats_slot,
                      MQTT_DESTINATION_FAILURE_CONNECTION);
        log_warn("Managed destination '%s' connection refused: %s",
                 client->profile.name, mosquitto_connack_string(result));
    }
}

static void on_disconnect(struct mosquitto *mosq, void *userdata, int result) {
    (void)mosq;
    mqtt_destination_client_t *client = userdata;
    if (!client) return;
    pthread_mutex_lock(&client->mutex);
    bool was_connected = client->connected;
    client->connected = false;
    if (client->publish_waiting) {
        client->publish_failed = true;
        pthread_cond_broadcast(&client->wake);
    }
    bool shutting_down = client->shutting_down;
    pthread_mutex_unlock(&client->mutex);
    if (was_connected)
        stats_connection(client->stats_slot, false, false);
    if (!shutting_down && result != 0) {
        stats_failure(client->stats_slot,
                      MQTT_DESTINATION_FAILURE_CONNECTION);
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

static int configure_presence(mqtt_destination_client_t *client) {
    if (!client->profile.status_topic_template[0]) return 0;
    char source[EVENT_SOURCE_MAX];
    static const char prefix[] = "urn:lightnvr:";
    system_health_process_run_t run;
    memset(&run, 0, sizeof(run));
    if (event_identity_get_source(source, sizeof(source)) != 0 ||
        strncmp(source, prefix, sizeof(prefix) - 1U) != 0 ||
        !system_health_evaluator_service_copy_run(&run)) return -1;
    const char *installation_uuid = source + sizeof(prefix) - 1U;
    if (expand_status_topic(&client->profile, installation_uuid,
                            client->status_topic,
                            sizeof(client->status_topic)) != 0) return -1;
    snprintf(client->installation_uuid, sizeof(client->installation_uuid),
             "%s", installation_uuid);
    snprintf(client->run_id, sizeof(client->run_id), "%s", run.run_id);
    snprintf(client->boot_id, sizeof(client->boot_id), "%s", run.boot_id);
    client->presence_configured = true;

    int64_t now = wall_time_ms();
    char payload[MQTT_PRESENCE_PAYLOAD_MAX];
    if (build_presence_payload(client, MQTT_PRESENCE_OFFLINE,
                               MQTT_OPERATIONAL_UNKNOWN, 0U, now, 0U,
                               payload, sizeof(payload)) != 0) return -1;
    return mosquitto_will_set(
        client->mosq, client->status_topic, (int)strlen(payload), payload,
        client->profile.qos, true);
}

static void *heartbeat_main(void *argument) {
    mqtt_destination_client_t *client = argument;
    pthread_mutex_lock(&client->mutex);
    while (!client->heartbeat_stop) {
        struct timespec deadline;
        clock_gettime(CLOCK_REALTIME, &deadline);
        deadline.tv_sec += MANAGED_PRESENCE_INTERVAL_SECONDS;
        int result = 0;
        while (!client->heartbeat_stop && result != ETIMEDOUT) {
            result = pthread_cond_timedwait(
                &client->heartbeat_wake, &client->mutex, &deadline);
            if (result != 0 && result != ETIMEDOUT) break;
        }
        bool publish = !client->heartbeat_stop && client->connected &&
                       result == ETIMEDOUT;
        pthread_mutex_unlock(&client->mutex);
        if (publish) (void)publish_presence(client, MQTT_PRESENCE_ONLINE);
        pthread_mutex_lock(&client->mutex);
    }
    pthread_mutex_unlock(&client->mutex);
    return NULL;
}

static void destroy_partial(mqtt_destination_client_t *client) {
    if (!client) return;
    if (client->loop_started && client->mosq) {
        mosquitto_disconnect(client->mosq);
        mosquitto_loop_stop(client->mosq, true);
    }
    if (client->mosq) mosquitto_destroy(client->mosq);
    if (client->heartbeat_condition_ready)
        pthread_cond_destroy(&client->heartbeat_wake);
    if (client->synchronization_ready) {
        pthread_cond_destroy(&client->wake);
        pthread_mutex_destroy(&client->mutex);
    }
    release_stats_slot(client->stats_slot);
    memset(client, 0, sizeof(*client));
    free(client);
}

mqtt_destination_client_t *mqtt_destination_client_create(
    const event_destination_t *profile) {
    char validation_error[EVENT_DESTINATION_VALIDATION_ERROR_MAX] = {0};
    if (!profile || !profile->enabled ||
        db_event_destination_validate(profile, NULL, false, validation_error,
                                      sizeof(validation_error)) !=
            DB_EVENT_DESTINATION_OK) return NULL;

    mqtt_destination_client_t *client = calloc(1, sizeof(*client));
    if (!client) return NULL;
    client->profile = *profile;
    client->publish_mid = -1;
    client->stats_slot = -1;
    client->stats_slot = reserve_stats_slot(profile);
    if (client->stats_slot < 0) {
        free(client);
        return NULL;
    }
    if (pthread_mutex_init(&client->mutex, NULL) != 0) {
        destroy_partial(client);
        return NULL;
    }
    if (pthread_cond_init(&client->wake, NULL) != 0) {
        pthread_mutex_destroy(&client->mutex);
        release_stats_slot(client->stats_slot);
        free(client);
        return NULL;
    }
    client->synchronization_ready = true;
    if (pthread_cond_init(&client->heartbeat_wake, NULL) != 0) {
        destroy_partial(client);
        return NULL;
    }
    client->heartbeat_condition_ready = true;

    char password[EVENT_DESTINATION_PASSWORD_MAX] = {0};
    db_event_destination_result_t password_result =
        db_event_destination_get_password(
            profile->uuid, profile->revision, password, sizeof(password));
    if (password_result != DB_EVENT_DESTINATION_OK) {
        secure_zero_memory(password, sizeof(password));
        stats_failure(client->stats_slot,
                      MQTT_DESTINATION_FAILURE_CONFIGURATION);
        destroy_partial(client);
        return NULL;
    }

    client->mosq = mosquitto_new(profile->client_id, true, client);
    if (!client->mosq) {
        secure_zero_memory(password, sizeof(password));
        stats_failure(client->stats_slot,
                      MQTT_DESTINATION_FAILURE_CONFIGURATION);
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
    if (result == MOSQ_ERR_SUCCESS) result = configure_presence(client);
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
    if (result == MOSQ_ERR_SUCCESS && client->presence_configured) {
        result = pthread_create(&client->heartbeat_thread, NULL,
                                heartbeat_main, client) == 0
            ? MOSQ_ERR_SUCCESS : MOSQ_ERR_ERRNO;
        client->heartbeat_started = result == MOSQ_ERR_SUCCESS;
    }
    if (result != MOSQ_ERR_SUCCESS) {
        stats_failure(client->stats_slot,
                      MQTT_DESTINATION_FAILURE_CONFIGURATION);
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
    client->heartbeat_stop = true;
    pthread_cond_broadcast(&client->heartbeat_wake);
    pthread_mutex_unlock(&client->mutex);
    if (client->heartbeat_started) {
        pthread_join(client->heartbeat_thread, NULL);
        client->heartbeat_started = false;
    }
    if (mqtt_destination_client_is_connected(client))
        (void)publish_presence(client, MQTT_PRESENCE_STOPPING);

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

bool mqtt_destination_client_get_stats(
    mqtt_destination_client_t *client,
    mqtt_destination_client_stats_t *stats) {
    if (!client || !stats || client->stats_slot < 0 ||
        client->stats_slot >= EVENT_DESTINATION_MAX_COUNT) return false;
    pthread_mutex_lock(&stats_lock);
    bool available = stats_slots[client->stats_slot].occupied;
    if (available) *stats = stats_slots[client->stats_slot].value;
    pthread_mutex_unlock(&stats_lock);
    return available;
}

size_t mqtt_destination_client_list_stats(
    mqtt_destination_client_stats_t *stats, size_t capacity) {
    if (!stats || capacity == 0U) return 0U;
    size_t count = 0U;
    pthread_mutex_lock(&stats_lock);
    for (size_t index = 0U; index < EVENT_DESTINATION_MAX_COUNT &&
         count < capacity; ++index) {
        if (stats_slots[index].occupied)
            stats[count++] = stats_slots[index].value;
    }
    pthread_mutex_unlock(&stats_lock);
    for (size_t index = 1U; index < count; ++index) {
        mqtt_destination_client_stats_t value = stats[index];
        size_t position = index;
        while (position > 0U &&
               strcmp(stats[position - 1U].destination_uuid,
                      value.destination_uuid) > 0) {
            stats[position] = stats[position - 1U];
            --position;
        }
        stats[position] = value;
    }
    return count;
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
        stats_publish(client->stats_slot, false, wall_time_ms());
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
        stats_publish(client->stats_slot, false, wall_time_ms());
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
    stats_publish(client->stats_slot, acknowledged, wall_time_ms());
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

bool mqtt_destination_client_get_stats(
    mqtt_destination_client_t *client,
    mqtt_destination_client_stats_t *stats) {
    (void)client;
    (void)stats;
    return false;
}

size_t mqtt_destination_client_list_stats(
    mqtt_destination_client_stats_t *stats, size_t capacity) {
    (void)stats;
    (void)capacity;
    return 0U;
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
