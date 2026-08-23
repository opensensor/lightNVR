#define _POSIX_C_SOURCE 200809L

#include "core/event_bus.h"

#include <errno.h>
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

typedef struct event_node {
    event_envelope_t event;
    size_t serialized_size;
    struct event_node *next;
} event_node_t;

typedef struct {
    char name[EVENT_BUS_SUBSCRIBER_NAME_MAX];
    event_bus_handler_t handler;
    void *context;
} event_subscription_t;

typedef struct {
    pthread_mutex_t mutex;
    pthread_cond_t wake;
    pthread_cond_t idle;
    pthread_t worker;
    bool initialized;
    bool running;
    bool drain_on_shutdown;
    event_node_t *head;
    event_node_t *tail;
    event_subscription_t subscribers[EVENT_BUS_MAX_SUBSCRIBERS];
    int subscriber_count;
    event_bus_stats_t stats;
} event_bus_state_t;

static event_bus_state_t BUS = {
    .mutex = PTHREAD_MUTEX_INITIALIZER,
    .wake = PTHREAD_COND_INITIALIZER,
    .idle = PTHREAD_COND_INITIALIZER,
};

static void set_error(char *error, size_t error_size, const char *message) {
    if (!error || error_size == 0) return;
    snprintf(error, error_size, "%s", message ? message : "event bus error");
}

static void free_node(event_node_t *node) {
    if (!node) return;
    event_envelope_clear(&node->event);
    free(node);
}

static void remove_node_locked(event_node_t *previous, event_node_t *node) {
    if (!node) return;
    if (previous) {
        previous->next = node->next;
    } else {
        BUS.head = node->next;
    }
    if (BUS.tail == node) BUS.tail = previous;
    BUS.stats.queued_events--;
    BUS.stats.queued_bytes -= node->serialized_size;
    free_node(node);
}

static bool queue_has_capacity_locked(size_t event_size) {
    return BUS.stats.queued_events < BUS.stats.max_events &&
        event_size <= BUS.stats.max_bytes - BUS.stats.queued_bytes;
}

static bool shed_lower_priority_locked(event_severity_t incoming_severity,
                                       size_t event_size) {
    while (!queue_has_capacity_locked(event_size)) {
        event_node_t *previous = NULL;
        event_node_t *candidate_previous = NULL;
        event_node_t *candidate = NULL;
        for (event_node_t *node = BUS.head; node;
             previous = node, node = node->next) {
            const event_type_definition_t *definition =
                event_registry_find(node->event.type);
            if (definition && definition->severity < incoming_severity) {
                candidate = node;
                candidate_previous = previous;
                break;
            }
        }
        if (!candidate) return false;
        remove_node_locked(candidate_previous, candidate);
        BUS.stats.dropped_events++;
        BUS.stats.priority_shed_events++;
    }
    return true;
}

static event_node_t *pop_node_locked(void) {
    event_node_t *node = BUS.head;
    if (!node) return NULL;
    BUS.head = node->next;
    if (!BUS.head) BUS.tail = NULL;
    node->next = NULL;
    BUS.stats.queued_events--;
    BUS.stats.queued_bytes -= node->serialized_size;
    BUS.stats.active_dispatches++;
    return node;
}

static void *event_bus_worker(void *unused) {
    (void)unused;
    pthread_mutex_lock(&BUS.mutex);
    for (;;) {
        while (!BUS.head && BUS.running) {
            pthread_cond_wait(&BUS.wake, &BUS.mutex);
        }
        if (!BUS.running && (!BUS.drain_on_shutdown || !BUS.head)) break;

        event_node_t *node = pop_node_locked();
        event_subscription_t subscribers[EVENT_BUS_MAX_SUBSCRIBERS];
        int subscriber_count = BUS.subscriber_count;
        if (subscriber_count > 0) {
            memcpy(subscribers, BUS.subscribers,
                   (size_t)subscriber_count * sizeof(subscribers[0]));
        }
        pthread_mutex_unlock(&BUS.mutex);

        uint64_t failures = 0;
        for (int index = 0; index < subscriber_count; index++) {
            if (subscribers[index].handler(&node->event,
                                           subscribers[index].context) != 0) {
                failures++;
            }
        }
        free_node(node);

        pthread_mutex_lock(&BUS.mutex);
        BUS.stats.dispatched_events++;
        BUS.stats.callback_deliveries += (uint64_t)subscriber_count;
        BUS.stats.handler_failures += failures;
        BUS.stats.active_dispatches--;
        if (!BUS.head && BUS.stats.active_dispatches == 0) {
            pthread_cond_broadcast(&BUS.idle);
        }
    }
    if (!BUS.head && BUS.stats.active_dispatches == 0) {
        pthread_cond_broadcast(&BUS.idle);
    }
    pthread_mutex_unlock(&BUS.mutex);
    return NULL;
}

int event_bus_subscribe(const char *name, event_bus_handler_t handler,
                        void *context) {
    if (!name || !name[0] || strlen(name) >= EVENT_BUS_SUBSCRIBER_NAME_MAX ||
        !handler) {
        return -1;
    }
    pthread_mutex_lock(&BUS.mutex);
    if (BUS.initialized ||
        BUS.subscriber_count >= EVENT_BUS_MAX_SUBSCRIBERS) {
        pthread_mutex_unlock(&BUS.mutex);
        return -1;
    }
    for (int index = 0; index < BUS.subscriber_count; index++) {
        if (strcmp(BUS.subscribers[index].name, name) == 0) {
            pthread_mutex_unlock(&BUS.mutex);
            return -1;
        }
    }
    event_subscription_t *subscription =
        &BUS.subscribers[BUS.subscriber_count++];
    memset(subscription, 0, sizeof(*subscription));
    snprintf(subscription->name, sizeof(subscription->name), "%s", name);
    subscription->handler = handler;
    subscription->context = context;
    pthread_mutex_unlock(&BUS.mutex);
    return 0;
}

int event_bus_unsubscribe(const char *name) {
    if (!name || !name[0]) return -1;
    pthread_mutex_lock(&BUS.mutex);
    if (BUS.initialized) {
        pthread_mutex_unlock(&BUS.mutex);
        return -1;
    }
    for (int index = 0; index < BUS.subscriber_count; index++) {
        if (strcmp(BUS.subscribers[index].name, name) == 0) {
            for (int move = index + 1; move < BUS.subscriber_count; move++) {
                BUS.subscribers[move - 1] = BUS.subscribers[move];
            }
            BUS.subscriber_count--;
            memset(&BUS.subscribers[BUS.subscriber_count], 0,
                   sizeof(BUS.subscribers[0]));
            pthread_mutex_unlock(&BUS.mutex);
            return 0;
        }
    }
    pthread_mutex_unlock(&BUS.mutex);
    return -1;
}

int event_bus_init(size_t max_events, size_t max_bytes) {
    if (max_events == 0) max_events = EVENT_BUS_DEFAULT_MAX_EVENTS;
    if (max_bytes == 0) max_bytes = EVENT_BUS_DEFAULT_MAX_BYTES;
    if (max_events == 0 || max_bytes < EVENT_ENVELOPE_MAX_BYTES) return -1;

    pthread_mutex_lock(&BUS.mutex);
    if (BUS.initialized) {
        pthread_mutex_unlock(&BUS.mutex);
        return -1;
    }
    memset(&BUS.stats, 0, sizeof(BUS.stats));
    BUS.stats.max_events = max_events;
    BUS.stats.max_bytes = max_bytes;
    BUS.stats.subscriber_count = BUS.subscriber_count;
    BUS.stats.running = true;
    BUS.running = true;
    BUS.drain_on_shutdown = false;
    BUS.initialized = true;
    if (pthread_create(&BUS.worker, NULL, event_bus_worker, NULL) != 0) {
        BUS.running = false;
        BUS.initialized = false;
        BUS.stats.running = false;
        pthread_mutex_unlock(&BUS.mutex);
        return -1;
    }
    pthread_mutex_unlock(&BUS.mutex);
    return 0;
}

event_bus_result_t event_bus_publish(const event_envelope_t *event,
                                     char *error, size_t error_size) {
    if (error && error_size > 0) error[0] = '\0';
    pthread_mutex_lock(&BUS.mutex);
    bool running = BUS.initialized && BUS.running;
    pthread_mutex_unlock(&BUS.mutex);
    if (!running) {
        set_error(error, error_size, "event bus is not running");
        return EVENT_BUS_NOT_RUNNING;
    }

    char validation_error[256] = {0};
    char *serialized =
        event_envelope_serialize(event, validation_error,
                                 sizeof(validation_error));
    if (!serialized) {
        pthread_mutex_lock(&BUS.mutex);
        BUS.stats.rejected_events++;
        pthread_mutex_unlock(&BUS.mutex);
        set_error(error, error_size, validation_error);
        return EVENT_BUS_INVALID_EVENT;
    }
    size_t serialized_size = strlen(serialized);
    free(serialized);

    event_node_t *node = calloc(1, sizeof(*node));
    if (!node || event_envelope_clone(&node->event, event, validation_error,
                                      sizeof(validation_error)) != 0) {
        free_node(node);
        pthread_mutex_lock(&BUS.mutex);
        BUS.stats.rejected_events++;
        pthread_mutex_unlock(&BUS.mutex);
        set_error(error, error_size, "event queue allocation failed");
        return EVENT_BUS_ALLOCATION_FAILED;
    }
    node->serialized_size = serialized_size;
    const event_type_definition_t *definition =
        event_registry_find(node->event.type);

    pthread_mutex_lock(&BUS.mutex);
    if (!BUS.initialized || !BUS.running) {
        pthread_mutex_unlock(&BUS.mutex);
        free_node(node);
        set_error(error, error_size, "event bus stopped before enqueue");
        return EVENT_BUS_NOT_RUNNING;
    }
    bool has_capacity = queue_has_capacity_locked(serialized_size);
    if (!has_capacity && definition &&
        definition->severity >= EVENT_SEVERITY_ERROR) {
        has_capacity =
            shed_lower_priority_locked(definition->severity, serialized_size);
    }
    if (!has_capacity) {
        BUS.stats.dropped_events++;
        pthread_mutex_unlock(&BUS.mutex);
        free_node(node);
        set_error(error, error_size, "event bus queue is full");
        return EVENT_BUS_QUEUE_FULL;
    }
    if (BUS.tail) {
        BUS.tail->next = node;
    } else {
        BUS.head = node;
    }
    BUS.tail = node;
    BUS.stats.queued_events++;
    BUS.stats.queued_bytes += serialized_size;
    BUS.stats.accepted_events++;
    pthread_cond_signal(&BUS.wake);
    pthread_mutex_unlock(&BUS.mutex);
    return EVENT_BUS_OK;
}

int event_bus_wait_until_idle(unsigned int timeout_ms) {
    struct timespec deadline;
    if (clock_gettime(CLOCK_REALTIME, &deadline) != 0) return -1;
    deadline.tv_sec += (time_t)(timeout_ms / 1000U);
    deadline.tv_nsec += (long)(timeout_ms % 1000U) * 1000000L;
    if (deadline.tv_nsec >= 1000000000L) {
        deadline.tv_sec++;
        deadline.tv_nsec -= 1000000000L;
    }

    pthread_mutex_lock(&BUS.mutex);
    if (!BUS.initialized) {
        pthread_mutex_unlock(&BUS.mutex);
        return -1;
    }
    int result = 0;
    while (BUS.stats.queued_events > 0 || BUS.stats.active_dispatches > 0) {
        int wait_result =
            pthread_cond_timedwait(&BUS.idle, &BUS.mutex, &deadline);
        if (wait_result == ETIMEDOUT) {
            result = -1;
            break;
        }
        if (wait_result != 0) {
            result = -1;
            break;
        }
    }
    pthread_mutex_unlock(&BUS.mutex);
    return result;
}

void event_bus_get_stats(event_bus_stats_t *stats) {
    if (!stats) return;
    pthread_mutex_lock(&BUS.mutex);
    *stats = BUS.stats;
    stats->subscriber_count = BUS.subscriber_count;
    stats->running = BUS.initialized && BUS.running;
    pthread_mutex_unlock(&BUS.mutex);
}

void event_bus_shutdown(bool drain) {
    pthread_mutex_lock(&BUS.mutex);
    if (!BUS.initialized) {
        pthread_mutex_unlock(&BUS.mutex);
        return;
    }
    BUS.running = false;
    BUS.drain_on_shutdown = drain;
    BUS.stats.running = false;
    if (!drain) {
        while (BUS.head) {
            event_node_t *node = BUS.head;
            BUS.head = node->next;
            BUS.stats.queued_events--;
            BUS.stats.queued_bytes -= node->serialized_size;
            BUS.stats.dropped_events++;
            free_node(node);
        }
        BUS.tail = NULL;
    }
    pthread_cond_broadcast(&BUS.wake);
    pthread_mutex_unlock(&BUS.mutex);

    pthread_join(BUS.worker, NULL);

    pthread_mutex_lock(&BUS.mutex);
    while (BUS.head) {
        event_node_t *node = BUS.head;
        BUS.head = node->next;
        free_node(node);
    }
    BUS.tail = NULL;
    BUS.stats.queued_events = 0;
    BUS.stats.queued_bytes = 0;
    BUS.initialized = false;
    BUS.drain_on_shutdown = false;
    pthread_cond_broadcast(&BUS.idle);
    pthread_mutex_unlock(&BUS.mutex);
}
