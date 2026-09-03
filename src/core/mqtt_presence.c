#define _POSIX_C_SOURCE 200809L

#include "core/mqtt_presence.h"

#include <ctype.h>
#include <inttypes.h>
#include <pthread.h>
#include <stdio.h>
#include <string.h>

#include "utils/uuid.h"

typedef struct {
    pthread_mutex_t lock;
    bool configured;
    bool connected;
    bool ever_connected;
    char topic[MQTT_PRESENCE_TOPIC_MAX];
    char installation_uuid[LIGHTNVR_UUID_STRING_SIZE];
    char run_id[LIGHTNVR_UUID_STRING_SIZE];
    char boot_id[64];
    char visibility[MQTT_PRESENCE_VISIBILITY_MAX];
    char version[32];
    uint64_t sequence;
    uint64_t connections;
    uint64_t reconnects;
    uint64_t disconnects;
    uint64_t publish_attempts;
    uint64_t publish_successes;
    uint64_t publish_failures;
    int64_t last_publish_at_ms;
} mqtt_presence_runtime_t;

static mqtt_presence_runtime_t runtime = {
    .lock = PTHREAD_MUTEX_INITIALIZER,
};

static bool safe_token(const char *value, size_t capacity) {
    if (!value || !value[0] || strlen(value) >= capacity) return false;
    for (const unsigned char *cursor = (const unsigned char *)value;
         *cursor; ++cursor) {
        if (*cursor < 0x20U || *cursor == 0x7fU || *cursor == '"' ||
            *cursor == '\\') return false;
    }
    return true;
}

static bool safe_topic_prefix(const char *prefix) {
    if (!safe_token(prefix, MQTT_PRESENCE_TOPIC_MAX - 48U)) return false;
    for (const char *cursor = prefix; *cursor; ++cursor) {
        if (*cursor == '#' || *cursor == '+') return false;
    }
    return true;
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

static uint64_t increment_saturated(uint64_t value) {
    return value == UINT64_MAX ? value : value + 1U;
}

int mqtt_presence_configure(const char *topic_prefix,
                            const char *installation_uuid,
                            const char *run_id, const char *boot_id,
                            const char *visibility_scope,
                            const char *version) {
    if (!safe_topic_prefix(topic_prefix) ||
        !lightnvr_uuid_is_valid(installation_uuid) ||
        !lightnvr_uuid_is_valid(run_id) ||
        !safe_token(boot_id, sizeof(runtime.boot_id)) ||
        !safe_token(visibility_scope, sizeof(runtime.visibility)) ||
        !safe_token(version, sizeof(runtime.version))) return -1;

    size_t prefix_length = strlen(topic_prefix);
    while (prefix_length > 0U && topic_prefix[prefix_length - 1U] == '/')
        --prefix_length;
    if (prefix_length == 0U) return -1;

    char topic[MQTT_PRESENCE_TOPIC_MAX];
    int written = snprintf(topic, sizeof(topic), "%.*s/v1/status/%s",
                           (int)prefix_length, topic_prefix,
                           installation_uuid);
    if (written < 0 || (size_t)written >= sizeof(topic)) return -1;

    pthread_mutex_lock(&runtime.lock);
    bool same_run = runtime.configured &&
                    strcmp(runtime.run_id, run_id) == 0;
    if (!same_run) {
        runtime.connected = false;
        runtime.ever_connected = false;
        runtime.sequence = 0U;
        runtime.connections = 0U;
        runtime.reconnects = 0U;
        runtime.disconnects = 0U;
        runtime.publish_attempts = 0U;
        runtime.publish_successes = 0U;
        runtime.publish_failures = 0U;
        runtime.last_publish_at_ms = 0;
    }
    snprintf(runtime.topic, sizeof(runtime.topic), "%s", topic);
    snprintf(runtime.installation_uuid, sizeof(runtime.installation_uuid),
             "%s", installation_uuid);
    snprintf(runtime.run_id, sizeof(runtime.run_id), "%s", run_id);
    snprintf(runtime.boot_id, sizeof(runtime.boot_id), "%s", boot_id);
    snprintf(runtime.visibility, sizeof(runtime.visibility), "%s",
             visibility_scope);
    snprintf(runtime.version, sizeof(runtime.version), "%s", version);
    runtime.configured = true;
    pthread_mutex_unlock(&runtime.lock);
    return 0;
}

static int build_locked(mqtt_presence_state_t state,
                        mqtt_operational_state_t operational_state,
                        size_t active_incidents, int64_t observed_at_ms,
                        uint64_t sequence, char *topic, size_t topic_size,
                        char *payload, size_t payload_size) {
    const char *state_name = presence_state_name(state);
    const char *operational_name = operational_state_name(operational_state);
    if (!runtime.configured || !state_name || !operational_name ||
        observed_at_ms <= 0 || !topic || topic_size == 0U ||
        !payload || payload_size == 0U ||
        strlen(runtime.topic) >= topic_size) return -1;
    snprintf(topic, topic_size, "%s", runtime.topic);
    int written = snprintf(
        payload, payload_size,
        "{\"schema_version\":1,\"installation_uuid\":\"%s\","
        "\"run_id\":\"%s\",\"boot_id\":\"%s\","
        "\"visibility_scope\":\"%s\",\"version\":\"%s\","
        "\"sequence\":%" PRIu64 ",\"timestamp_ms\":%" PRId64 ","
        "\"state\":\"%s\",\"overall_state\":\"%s\","
        "\"active_incidents\":%zu}",
        runtime.installation_uuid, runtime.run_id, runtime.boot_id,
        runtime.visibility, runtime.version, sequence, observed_at_ms,
        state_name, operational_name, active_incidents);
    if (written < 0 || (size_t)written >= payload_size) {
        payload[0] = '\0';
        return -1;
    }
    return 0;
}

int mqtt_presence_build_will(int64_t configured_at_ms,
                             char *topic, size_t topic_size,
                             char *payload, size_t payload_size) {
    pthread_mutex_lock(&runtime.lock);
    int result = build_locked(MQTT_PRESENCE_OFFLINE,
                              MQTT_OPERATIONAL_UNKNOWN, 0U,
                              configured_at_ms, 0U, topic, topic_size,
                              payload, payload_size);
    pthread_mutex_unlock(&runtime.lock);
    return result;
}

int mqtt_presence_build(mqtt_presence_state_t state,
                        mqtt_operational_state_t operational_state,
                        size_t active_incidents, int64_t observed_at_ms,
                        char *topic, size_t topic_size,
                        char *payload, size_t payload_size) {
    if (state != MQTT_PRESENCE_ONLINE && state != MQTT_PRESENCE_STOPPING)
        return -1;
    pthread_mutex_lock(&runtime.lock);
    uint64_t next = increment_saturated(runtime.sequence);
    int result = build_locked(state, operational_state, active_incidents,
                              observed_at_ms, next, topic, topic_size,
                              payload, payload_size);
    if (result == 0) runtime.sequence = next;
    pthread_mutex_unlock(&runtime.lock);
    return result;
}

void mqtt_presence_record_connected(void) {
    pthread_mutex_lock(&runtime.lock);
    runtime.connections = increment_saturated(runtime.connections);
    if (runtime.ever_connected)
        runtime.reconnects = increment_saturated(runtime.reconnects);
    runtime.ever_connected = true;
    runtime.connected = true;
    pthread_mutex_unlock(&runtime.lock);
}

void mqtt_presence_record_disconnected(void) {
    pthread_mutex_lock(&runtime.lock);
    if (runtime.connected)
        runtime.disconnects = increment_saturated(runtime.disconnects);
    runtime.connected = false;
    pthread_mutex_unlock(&runtime.lock);
}

void mqtt_presence_record_publish(bool succeeded, int64_t observed_at_ms) {
    pthread_mutex_lock(&runtime.lock);
    runtime.publish_attempts = increment_saturated(runtime.publish_attempts);
    if (succeeded)
        runtime.publish_successes = increment_saturated(runtime.publish_successes);
    else
        runtime.publish_failures = increment_saturated(runtime.publish_failures);
    if (observed_at_ms > 0) runtime.last_publish_at_ms = observed_at_ms;
    pthread_mutex_unlock(&runtime.lock);
}

void mqtt_presence_get_stats(mqtt_presence_stats_t *stats) {
    if (!stats) return;
    pthread_mutex_lock(&runtime.lock);
    stats->configured = runtime.configured;
    stats->connected = runtime.connected;
    stats->sequence = runtime.sequence;
    stats->connections = runtime.connections;
    stats->reconnects = runtime.reconnects;
    stats->disconnects = runtime.disconnects;
    stats->publish_attempts = runtime.publish_attempts;
    stats->publish_successes = runtime.publish_successes;
    stats->publish_failures = runtime.publish_failures;
    stats->last_publish_at_ms = runtime.last_publish_at_ms;
    pthread_mutex_unlock(&runtime.lock);
}

int mqtt_presence_copy_topic(char *topic, size_t topic_size) {
    if (!topic || topic_size == 0U) return -1;
    pthread_mutex_lock(&runtime.lock);
    if (!runtime.configured || strlen(runtime.topic) >= topic_size) {
        topic[0] = '\0';
        pthread_mutex_unlock(&runtime.lock);
        return -1;
    }
    snprintf(topic, topic_size, "%s", runtime.topic);
    pthread_mutex_unlock(&runtime.lock);
    return 0;
}

int mqtt_presence_copy_installation_uuid(char *uuid, size_t uuid_size) {
    if (!uuid || uuid_size == 0U) return -1;
    pthread_mutex_lock(&runtime.lock);
    if (!runtime.configured || strlen(runtime.installation_uuid) >= uuid_size) {
        uuid[0] = '\0';
        pthread_mutex_unlock(&runtime.lock);
        return -1;
    }
    snprintf(uuid, uuid_size, "%s", runtime.installation_uuid);
    pthread_mutex_unlock(&runtime.lock);
    return 0;
}

void mqtt_presence_reset(void) {
    pthread_mutex_lock(&runtime.lock);
    runtime.configured = false;
    runtime.connected = false;
    runtime.ever_connected = false;
    memset(runtime.topic, 0, sizeof(runtime.topic));
    memset(runtime.installation_uuid, 0, sizeof(runtime.installation_uuid));
    memset(runtime.run_id, 0, sizeof(runtime.run_id));
    memset(runtime.boot_id, 0, sizeof(runtime.boot_id));
    memset(runtime.visibility, 0, sizeof(runtime.visibility));
    memset(runtime.version, 0, sizeof(runtime.version));
    runtime.sequence = 0U;
    runtime.connections = 0U;
    runtime.reconnects = 0U;
    runtime.disconnects = 0U;
    runtime.publish_attempts = 0U;
    runtime.publish_successes = 0U;
    runtime.publish_failures = 0U;
    runtime.last_publish_at_ms = 0;
    pthread_mutex_unlock(&runtime.lock);
}
