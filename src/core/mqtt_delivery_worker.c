#define _POSIX_C_SOURCE 200809L

#include "core/mqtt_delivery_worker.h"

#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#include "core/logger.h"
#include "core/mqtt_client.h"
#include "core/mqtt_destination_client.h"
#include "database/db_event_destinations.h"
#include "utils/uuid.h"

#define MQTT_DELIVERY_POLL_SECONDS 1
#define MQTT_DESTINATION_CREATE_RETRY_SECONDS 30

typedef struct {
    pthread_mutex_t mutex;
    pthread_cond_t wake;
    pthread_t thread;
    bool thread_started;
    bool shutdown_in_progress;
    bool stop_requested;
    mqtt_delivery_worker_stats_t stats;
} mqtt_delivery_worker_state_t;

static mqtt_delivery_worker_state_t WORKER = {
    .mutex = PTHREAD_MUTEX_INITIALIZER,
    .wake = PTHREAD_COND_INITIALIZER,
};

static void increment(uint64_t *counter, uint64_t amount) {
    pthread_mutex_lock(&WORKER.mutex);
    *counter += amount;
    pthread_mutex_unlock(&WORKER.mutex);
}

static uint32_t event_jitter_seed(const event_outbox_item_t *item) {
    uint32_t hash = 2166136261u;
    for (const unsigned char *cursor =
             (const unsigned char *)item->event_id;
         *cursor; cursor++) {
        hash ^= *cursor;
        hash *= 16777619u;
    }
    hash ^= (uint32_t)item->attempt_count;
    hash *= 16777619u;
    return hash;
}

static bool valid_destination_key(const char *destination) {
    if (!destination) return false;
    if (strcmp(destination, MQTT_EVENT_OUTBOX_DESTINATION) == 0) return true;
    size_t prefix_length = strlen(EVENT_DESTINATION_KEY_PREFIX);
    return strncmp(destination, EVENT_DESTINATION_KEY_PREFIX,
                   prefix_length) == 0 &&
        lightnvr_uuid_is_valid(destination + prefix_length);
}

static bool append_topic_text(char *topic, size_t topic_size, size_t *length,
                              const char *value) {
    size_t value_length = strlen(value);
    if (value_length >= topic_size || *length > topic_size - value_length - 1) {
        return false;
    }
    memcpy(topic + *length, value, value_length);
    *length += value_length;
    topic[*length] = '\0';
    return true;
}

int mqtt_delivery_topic_expand(
    const char *topic_template, const event_envelope_t *event,
    char topic[EVENT_OUTBOX_TOPIC_MAX]) {
    if (topic) topic[0] = '\0';
    if (!topic_template || !event || !topic || topic_template[0] == '\0' ||
        event_envelope_validate(event, NULL, 0) != 0) {
        return -1;
    }
    const char *subject_id = strrchr(event->subject, '/');
    subject_id = subject_id ? subject_id + 1 : event->subject;
    if (subject_id[0] == '\0') return -1;

    size_t length = 0;
    bool saw_type = false;
    bool saw_subject_id = false;
    for (size_t index = 0; topic_template[index] != '\0';) {
        const char *value = NULL;
        size_t token_length = 0;
        if (strncmp(topic_template + index, "{type}", 6) == 0) {
            value = event->type;
            token_length = 6;
            saw_type = true;
        } else if (strncmp(topic_template + index, "{subject_id}", 12) == 0) {
            value = subject_id;
            token_length = 12;
            saw_subject_id = true;
        }
        if (value) {
            if (!append_topic_text(topic, EVENT_OUTBOX_TOPIC_MAX, &length,
                                   value)) {
                topic[0] = '\0';
                return -1;
            }
            index += token_length;
        } else {
            if (topic_template[index] == '{' ||
                topic_template[index] == '}' ||
                topic_template[index] == '+' ||
                topic_template[index] == '#') {
                topic[0] = '\0';
                return -1;
            }
            char literal[2] = {topic_template[index], '\0'};
            if (!append_topic_text(topic, EVENT_OUTBOX_TOPIC_MAX, &length,
                                   literal)) {
                topic[0] = '\0';
                return -1;
            }
            index++;
        }
    }
    if (!saw_type || !saw_subject_id || length == 0 || topic[0] == '/' ||
        topic[length - 1] == '/') {
        topic[0] = '\0';
        return -1;
    }
    return 0;
}

int mqtt_delivery_backoff_seconds(int attempt_count, uint32_t jitter_seed) {
    if (attempt_count < 1) attempt_count = 1;
    int ceiling = 2;
    for (int attempt = 1;
         attempt < attempt_count &&
             ceiling < MQTT_DELIVERY_MAX_BACKOFF_SECONDS;
         attempt++) {
        if (ceiling > MQTT_DELIVERY_MAX_BACKOFF_SECONDS / 2) {
            ceiling = MQTT_DELIVERY_MAX_BACKOFF_SECONDS;
        } else {
            ceiling *= 2;
        }
    }
    int floor = ceiling - ceiling / 4;
    int range = ceiling - floor + 1;
    return floor + (int)(jitter_seed % (uint32_t)range);
}

event_outbox_enqueue_result_t mqtt_delivery_worker_enqueue(
    const event_envelope_t *event, const char *topic_prefix,
    int64_t *row_id) {
    if (row_id) *row_id = 0;
    if (!event || !topic_prefix || topic_prefix[0] == '\0') {
        increment(&WORKER.stats.enqueue_errors, 1);
        return EVENT_OUTBOX_ERROR;
    }
    char topic_template[EVENT_OUTBOX_TOPIC_MAX];
    int template_length = snprintf(
        topic_template, sizeof(topic_template),
        "%s/v1/events/{type}/{subject_id}", topic_prefix);
    if (template_length < 0 ||
        (size_t)template_length >= sizeof(topic_template)) {
        increment(&WORKER.stats.enqueue_errors, 1);
        return EVENT_OUTBOX_ERROR;
    }
    return mqtt_delivery_worker_enqueue_destination(
        event, MQTT_EVENT_OUTBOX_DESTINATION, topic_template, row_id);
}

event_outbox_enqueue_result_t mqtt_delivery_worker_enqueue_destination(
    const event_envelope_t *event, const char *destination,
    const char *topic_template, int64_t *row_id) {
    if (row_id) *row_id = 0;
    if (!event || !valid_destination_key(destination) ||
        strlen(destination) >= EVENT_OUTBOX_DESTINATION_MAX ||
        !topic_template || topic_template[0] == '\0') {
        increment(&WORKER.stats.enqueue_errors, 1);
        return EVENT_OUTBOX_ERROR;
    }
    char topic[EVENT_OUTBOX_TOPIC_MAX];
    if (mqtt_delivery_topic_expand(topic_template, event, topic) != 0) {
        increment(&WORKER.stats.enqueue_errors, 1);
        return EVENT_OUTBOX_ERROR;
    }

    int shed = 0;
    event_outbox_enqueue_result_t result = db_event_outbox_enqueue(
        event, destination, topic, NULL, row_id, &shed);
    pthread_mutex_lock(&WORKER.mutex);
    if (result == EVENT_OUTBOX_ENQUEUED) {
        WORKER.stats.enqueued++;
        WORKER.stats.priority_shed += (uint64_t)shed;
        pthread_cond_signal(&WORKER.wake);
    } else if (result == EVENT_OUTBOX_DUPLICATE) {
        WORKER.stats.duplicates++;
    } else if (result == EVENT_OUTBOX_FULL) {
        WORKER.stats.rejected_full++;
    } else {
        WORKER.stats.enqueue_errors++;
    }
    pthread_mutex_unlock(&WORKER.mutex);
    return result;
}

static int process_destination_once(const char *destination, int64_t now,
                                    bool broker_connected,
                                    mqtt_delivery_publish_fn publish,
                                    void *context, bool expire_first) {
    if (!valid_destination_key(destination) || !publish) return -1;
    if (now <= 0) now = (int64_t)time(NULL);

    if (expire_first) {
        int expired = 0;
        if (db_event_outbox_expire(now, &expired) != 0) {
            increment(&WORKER.stats.outcome_errors, 1);
            return -1;
        }
        if (expired > 0) increment(&WORKER.stats.expired, (uint64_t)expired);
    }
    if (!broker_connected) {
        increment(&WORKER.stats.disconnected_polls, 1);
        return 0;
    }

    event_outbox_item_t item;
    int claim = db_event_outbox_claim_due(
        destination, now,
        EVENT_OUTBOX_DEFAULT_LEASE_SECONDS, &item);
    if (claim <= 0) {
        if (claim < 0) increment(&WORKER.stats.outcome_errors, 1);
        return claim;
    }

    increment(&WORKER.stats.attempted, 1);
    int publish_result = publish(item.topic, item.envelope_json, false,
                                 MQTT_DELIVERY_ACK_TIMEOUT_MS, context);
    int64_t outcome_at = (int64_t)time(NULL);
    if (outcome_at < now) outcome_at = now;
    int result = 1;
    if (publish_result == 0) {
        if (db_event_outbox_mark_delivered(item.row_id, outcome_at) != 0) {
            increment(&WORKER.stats.outcome_errors, 1);
            result = -1;
        } else {
            increment(&WORKER.stats.delivered, 1);
        }
    } else {
        int delay = mqtt_delivery_backoff_seconds(
            item.attempt_count, event_jitter_seed(&item));
        int64_t next_attempt_at = outcome_at + delay;
        if (next_attempt_at >= item.expires_at) {
            if (db_event_outbox_mark_dead(
                    item.row_id, outcome_at,
                    "MQTT retry would exceed event expiry") != 0) {
                increment(&WORKER.stats.outcome_errors, 1);
                result = -1;
            } else {
                increment(&WORKER.stats.dead, 1);
            }
        } else if (db_event_outbox_mark_retry(
                       item.row_id, next_attempt_at,
                       "MQTT publish was not acknowledged") != 0) {
            increment(&WORKER.stats.outcome_errors, 1);
            result = -1;
        } else {
            increment(&WORKER.stats.retried, 1);
        }
    }
    db_event_outbox_item_clear(&item);
    return result;
}

int mqtt_delivery_worker_process_destination_once(
    const char *destination, int64_t now, bool broker_connected,
    mqtt_delivery_publish_fn publish, void *context) {
    return process_destination_once(destination, now, broker_connected,
                                    publish, context, true);
}

int mqtt_delivery_worker_process_once(int64_t now, bool broker_connected,
                                      mqtt_delivery_publish_fn publish,
                                      void *context) {
    return mqtt_delivery_worker_process_destination_once(
        MQTT_EVENT_OUTBOX_DESTINATION, now, broker_connected, publish,
        context);
}

#ifdef ENABLE_MQTT
typedef struct {
    event_destination_t profile;
    char destination_key[EVENT_DESTINATION_KEY_MAX];
    mqtt_destination_client_t *client;
    int64_t next_create_attempt_at;
} managed_destination_slot_t;

static managed_destination_slot_t *MANAGED_SLOTS = NULL;
static int MANAGED_SLOT_COUNT = 0;
static uint64_t MANAGED_GENERATION = 0;

static int mqtt_confirmed_publish(const char *topic, const char *payload,
                                  bool retain, int timeout_ms,
                                  void *context) {
    (void)context;
    return mqtt_publish_raw_confirmed(topic, payload, retain, timeout_ms);
}

static int managed_confirmed_publish(const char *topic, const char *payload,
                                     bool retain, int timeout_ms,
                                     void *context) {
    return mqtt_destination_client_publish_confirmed(
        context, topic, payload, retain, timeout_ms);
}

static void clear_managed_slots(void) {
    for (int index = 0; index < MANAGED_SLOT_COUNT; index++) {
        mqtt_destination_client_destroy(MANAGED_SLOTS[index].client);
    }
    free(MANAGED_SLOTS);
    MANAGED_SLOTS = NULL;
    MANAGED_SLOT_COUNT = 0;
    MANAGED_GENERATION = 0;
    pthread_mutex_lock(&WORKER.mutex);
    WORKER.stats.managed_profiles = 0;
    WORKER.stats.managed_connected = 0;
    pthread_mutex_unlock(&WORKER.mutex);
}

static void count_profile_error(void) {
    increment(&WORKER.stats.profile_errors, 1);
}

static void create_managed_client(managed_destination_slot_t *slot,
                                  int64_t now) {
    if (!slot || slot->client || now < slot->next_create_attempt_at) return;
    slot->client = mqtt_destination_client_create(&slot->profile);
    slot->next_create_attempt_at = now +
        MQTT_DESTINATION_CREATE_RETRY_SECONDS;
    if (!slot->client) {
        count_profile_error();
        log_warn("Managed MQTT destination '%s' will retry initialization",
                 slot->profile.name);
    }
}

static int reload_managed_slots(uint64_t generation, int64_t now) {
    int total = db_event_destination_count();
    if (total < 0 || total > EVENT_DESTINATION_MAX_COUNT) return -1;
    event_destination_t *profiles = total > 0
        ? calloc((size_t)total, sizeof(*profiles)) : NULL;
    if (total > 0 && !profiles) return -1;
    int count = total > 0
        ? db_event_destination_list(profiles, total) : 0;
    if (count < 0) {
        free(profiles);
        return -1;
    }
    int enabled_count = 0;
    for (int index = 0; index < count; index++) {
        if (profiles[index].enabled) enabled_count++;
    }
    managed_destination_slot_t *slots = enabled_count > 0
        ? calloc((size_t)enabled_count, sizeof(*slots)) : NULL;
    if (enabled_count > 0 && !slots) {
        free(profiles);
        return -1;
    }
    int slot_index = 0;
    bool invalid_profile = false;
    for (int index = 0; index < count; index++) {
        if (!profiles[index].enabled) continue;
        managed_destination_slot_t *slot = &slots[slot_index++];
        slot->profile = profiles[index];
        if (db_event_destination_make_key(
                profiles[index].uuid, slot->destination_key) != 0) {
            invalid_profile = true;
        }
    }
    free(profiles);
    if (invalid_profile) {
        free(slots);
        return -1;
    }

    clear_managed_slots();
    MANAGED_SLOTS = slots;
    MANAGED_SLOT_COUNT = enabled_count;
    MANAGED_GENERATION = generation;
    for (int index = 0; index < MANAGED_SLOT_COUNT; index++) {
        create_managed_client(&MANAGED_SLOTS[index], now);
    }
    pthread_mutex_lock(&WORKER.mutex);
    WORKER.stats.managed_profiles = (uint64_t)MANAGED_SLOT_COUNT;
    WORKER.stats.profile_reloads++;
    pthread_mutex_unlock(&WORKER.mutex);
    return 0;
}

static bool ensure_managed_slots(int64_t now) {
    uint64_t generation = db_event_destination_generation();
    if (MANAGED_GENERATION != generation &&
        reload_managed_slots(generation, now) != 0) {
        count_profile_error();
        return false;
    }
    return true;
}

static void update_managed_connected_stats(void) {
    uint64_t connected = 0;
    for (int index = 0; index < MANAGED_SLOT_COUNT; index++) {
        if (mqtt_destination_client_is_connected(
                MANAGED_SLOTS[index].client)) {
            connected++;
        }
    }
    pthread_mutex_lock(&WORKER.mutex);
    WORKER.stats.managed_connected = connected;
    pthread_mutex_unlock(&WORKER.mutex);
}

static bool stop_requested(void) {
    pthread_mutex_lock(&WORKER.mutex);
    bool stop = WORKER.stop_requested;
    pthread_mutex_unlock(&WORKER.mutex);
    return stop;
}

static void wait_for_work(void) {
    struct timespec deadline;
    clock_gettime(CLOCK_REALTIME, &deadline);
    deadline.tv_sec += MQTT_DELIVERY_POLL_SECONDS;
    pthread_mutex_lock(&WORKER.mutex);
    if (!WORKER.stop_requested) {
        int rc = pthread_cond_timedwait(
            &WORKER.wake, &WORKER.mutex, &deadline);
        (void)rc;
    }
    pthread_mutex_unlock(&WORKER.mutex);
}

static void *mqtt_delivery_thread(void *unused) {
    (void)unused;
    log_set_thread_context("MQTTDelivery", NULL);
    while (!stop_requested()) {
        int64_t now = (int64_t)time(NULL);
        int processed_count = 0;
        if (!ensure_managed_slots(now)) {
            wait_for_work();
            continue;
        }
        int expired = 0;
        if (db_event_outbox_expire(now, &expired) != 0) {
            increment(&WORKER.stats.outcome_errors, 1);
            wait_for_work();
            continue;
        }
        if (expired > 0) increment(&WORKER.stats.expired, (uint64_t)expired);

        int processed = process_destination_once(
            MQTT_EVENT_OUTBOX_DESTINATION, now, mqtt_is_connected(),
            mqtt_confirmed_publish, NULL, false);
        if (processed > 0) processed_count += processed;
        for (int index = 0; index < MANAGED_SLOT_COUNT; index++) {
            managed_destination_slot_t *slot = &MANAGED_SLOTS[index];
            create_managed_client(slot, now);
            processed = process_destination_once(
                slot->destination_key, now,
                mqtt_destination_client_is_connected(slot->client),
                managed_confirmed_publish, slot->client, false);
            if (processed > 0) processed_count += processed;
        }
        update_managed_connected_stats();
        if (processed_count == 0) wait_for_work();
    }
    clear_managed_slots();
    return NULL;
}
#endif

int mqtt_delivery_worker_start(void) {
#ifndef ENABLE_MQTT
    log_warn("MQTT delivery worker unavailable: MQTT support is disabled");
    return -1;
#else
    pthread_mutex_lock(&WORKER.mutex);
    while (WORKER.shutdown_in_progress) {
        pthread_cond_wait(&WORKER.wake, &WORKER.mutex);
    }
    if (WORKER.thread_started) {
        pthread_mutex_unlock(&WORKER.mutex);
        return 0;
    }
    WORKER.stop_requested = false;
    if (pthread_create(&WORKER.thread, NULL,
                       mqtt_delivery_thread, NULL) != 0) {
        pthread_mutex_unlock(&WORKER.mutex);
        return -1;
    }
    WORKER.thread_started = true;
    WORKER.stats.running = true;
    pthread_mutex_unlock(&WORKER.mutex);
    log_info("MQTT durable delivery worker started");
    return 0;
#endif
}

void mqtt_delivery_worker_shutdown(void) {
    pthread_mutex_lock(&WORKER.mutex);
    while (WORKER.shutdown_in_progress) {
        pthread_cond_wait(&WORKER.wake, &WORKER.mutex);
    }
    if (!WORKER.thread_started) {
        WORKER.stats.running = false;
        pthread_mutex_unlock(&WORKER.mutex);
        return;
    }
    WORKER.shutdown_in_progress = true;
    WORKER.stop_requested = true;
    pthread_cond_broadcast(&WORKER.wake);
    pthread_t thread = WORKER.thread;
    pthread_mutex_unlock(&WORKER.mutex);

    pthread_join(thread, NULL);
    pthread_mutex_lock(&WORKER.mutex);
    WORKER.thread_started = false;
    WORKER.shutdown_in_progress = false;
    WORKER.stats.running = false;
    pthread_cond_broadcast(&WORKER.wake);
    pthread_mutex_unlock(&WORKER.mutex);
    log_info("MQTT durable delivery worker stopped");
}

void mqtt_delivery_worker_get_stats(mqtt_delivery_worker_stats_t *stats) {
    if (!stats) return;
    pthread_mutex_lock(&WORKER.mutex);
    *stats = WORKER.stats;
    pthread_mutex_unlock(&WORKER.mutex);
}
