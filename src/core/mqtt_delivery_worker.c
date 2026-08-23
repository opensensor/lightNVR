#define _POSIX_C_SOURCE 200809L

#include "core/mqtt_delivery_worker.h"

#include <pthread.h>
#include <stdio.h>
#include <string.h>
#include <time.h>

#include "core/logger.h"
#include "core/mqtt_client.h"

#define MQTT_DELIVERY_POLL_SECONDS 1

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
    const char *subject_id = strrchr(event->subject, '/');
    subject_id = subject_id ? subject_id + 1 : event->subject;
    if (subject_id[0] == '\0') {
        increment(&WORKER.stats.enqueue_errors, 1);
        return EVENT_OUTBOX_ERROR;
    }

    char topic[EVENT_OUTBOX_TOPIC_MAX];
    int topic_length = snprintf(
        topic, sizeof(topic), "%s/v1/events/%s/%s",
        topic_prefix, event->type, subject_id);
    if (topic_length < 0 || (size_t)topic_length >= sizeof(topic)) {
        increment(&WORKER.stats.enqueue_errors, 1);
        return EVENT_OUTBOX_ERROR;
    }

    int shed = 0;
    event_outbox_enqueue_result_t result = db_event_outbox_enqueue(
        event, MQTT_EVENT_OUTBOX_DESTINATION, topic, NULL, row_id, &shed);
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

int mqtt_delivery_worker_process_once(int64_t now, bool broker_connected,
                                      mqtt_delivery_publish_fn publish,
                                      void *context) {
    if (!publish) return -1;
    if (now <= 0) now = (int64_t)time(NULL);

    int expired = 0;
    if (db_event_outbox_expire(now, &expired) != 0) {
        increment(&WORKER.stats.outcome_errors, 1);
        return -1;
    }
    if (expired > 0) increment(&WORKER.stats.expired, (uint64_t)expired);
    if (!broker_connected) {
        increment(&WORKER.stats.disconnected_polls, 1);
        return 0;
    }

    event_outbox_item_t item;
    int claim = db_event_outbox_claim_due(
        MQTT_EVENT_OUTBOX_DESTINATION, now,
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

#ifdef ENABLE_MQTT
static int mqtt_confirmed_publish(const char *topic, const char *payload,
                                  bool retain, int timeout_ms,
                                  void *context) {
    (void)context;
    return mqtt_publish_raw_confirmed(topic, payload, retain, timeout_ms);
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
        int processed = mqtt_delivery_worker_process_once(
            (int64_t)time(NULL), mqtt_is_connected(),
            mqtt_confirmed_publish, NULL);
        if (processed != 1) wait_for_work();
    }
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
