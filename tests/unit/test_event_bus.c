/**
 * @file test_event_bus.c
 * @brief Bounded asynchronous event dispatch and pressure tests.
 */

#define _POSIX_C_SOURCE 200809L

#include <pthread.h>
#include <stdbool.h>
#include <stdio.h>
#include <string.h>
#include <time.h>

#include <cjson/cJSON.h>

#include "core/event_bus.h"
#include "unity.h"

#define INSTALLATION_SOURCE \
    "urn:lightnvr:11111111-1111-4111-8111-111111111111"
#define CAMERA_SUBJECT "camera/22222222-2222-4222-8222-222222222222"

typedef struct {
    pthread_mutex_t mutex;
    pthread_cond_t condition;
    int calls;
    bool started;
    bool block;
    bool release;
    bool saw_critical;
    char last_id[EVENT_ID_MAX];
    char last_type[EVENT_TYPE_MAX];
} capture_context_t;

static void capture_init(capture_context_t *context, bool block) {
    memset(context, 0, sizeof(*context));
    pthread_mutex_init(&context->mutex, NULL);
    pthread_cond_init(&context->condition, NULL);
    context->block = block;
}

static void capture_destroy(capture_context_t *context) {
    pthread_cond_destroy(&context->condition);
    pthread_mutex_destroy(&context->mutex);
}

static int capture_handler(const event_envelope_t *event, void *opaque) {
    capture_context_t *context = opaque;
    pthread_mutex_lock(&context->mutex);
    context->calls++;
    context->started = true;
    snprintf(context->last_id, sizeof(context->last_id), "%s", event->id);
    snprintf(context->last_type, sizeof(context->last_type), "%s",
             event->type);
    if (strcmp(event->type, "io.lightnvr.storage.pressure.v1") == 0) {
        context->saw_critical = true;
    }
    pthread_cond_broadcast(&context->condition);
    while (context->block && !context->release) {
        pthread_cond_wait(&context->condition, &context->mutex);
    }
    pthread_mutex_unlock(&context->mutex);
    return 0;
}

static int failure_handler(const event_envelope_t *event, void *context) {
    (void)event;
    (void)context;
    return -1;
}

static bool wait_for_handler_start(capture_context_t *context,
                                   unsigned int timeout_ms) {
    struct timespec deadline;
    clock_gettime(CLOCK_REALTIME, &deadline);
    deadline.tv_sec += (time_t)(timeout_ms / 1000U);
    deadline.tv_nsec += (long)(timeout_ms % 1000U) * 1000000L;
    if (deadline.tv_nsec >= 1000000000L) {
        deadline.tv_sec++;
        deadline.tv_nsec -= 1000000000L;
    }
    pthread_mutex_lock(&context->mutex);
    while (!context->started) {
        if (pthread_cond_timedwait(&context->condition, &context->mutex,
                                   &deadline) != 0) {
            pthread_mutex_unlock(&context->mutex);
            return false;
        }
    }
    pthread_mutex_unlock(&context->mutex);
    return true;
}

static void release_handler(capture_context_t *context) {
    pthread_mutex_lock(&context->mutex);
    context->release = true;
    pthread_cond_broadcast(&context->condition);
    pthread_mutex_unlock(&context->mutex);
}

static event_envelope_t detection_event(const char *label) {
    cJSON *data = cJSON_CreateObject();
    cJSON_AddNumberToObject(data, "count", 1);
    cJSON *detections = cJSON_AddArrayToObject(data, "detections");
    cJSON *detection = cJSON_CreateObject();
    cJSON_AddStringToObject(detection, "label", label);
    cJSON_AddNumberToObject(detection, "confidence", 0.9);
    cJSON_AddNumberToObject(detection, "x", 0.1);
    cJSON_AddNumberToObject(detection, "y", 0.1);
    cJSON_AddNumberToObject(detection, "width", 0.2);
    cJSON_AddNumberToObject(detection, "height", 0.2);
    cJSON_AddItemToArray(detections, detection);
    event_envelope_t event;
    char error[256];
    TEST_ASSERT_EQUAL_INT(
        0, event_envelope_create(&event,
                                 "io.lightnvr.detection.object.v1",
                                 INSTALLATION_SOURCE, CAMERA_SUBJECT, 0, data,
                                 error, sizeof(error)));
    cJSON_Delete(data);
    return event;
}

static event_envelope_t storage_event(void) {
    cJSON *data = cJSON_CreateObject();
    cJSON_AddStringToObject(data, "level", "critical");
    cJSON_AddNumberToObject(data, "used_percent", 95.0);
    event_envelope_t event;
    char error[256];
    TEST_ASSERT_EQUAL_INT(
        0, event_envelope_create(&event,
                                 "io.lightnvr.storage.pressure.v1",
                                 INSTALLATION_SOURCE, "system/storage", 0,
                                 data, error, sizeof(error)));
    cJSON_Delete(data);
    return event;
}

void setUp(void) {
    event_bus_shutdown(false);
    event_bus_unsubscribe("capture");
    event_bus_unsubscribe("failure");
}

void tearDown(void) {
    event_bus_shutdown(false);
    event_bus_unsubscribe("capture");
    event_bus_unsubscribe("failure");
}

void test_publish_dispatches_a_deep_copy_asynchronously(void) {
    capture_context_t capture;
    capture_init(&capture, false);
    TEST_ASSERT_EQUAL_INT(0,
                          event_bus_subscribe("capture", capture_handler,
                                              &capture));
    TEST_ASSERT_EQUAL_INT(0, event_bus_init(0, 0));

    event_envelope_t event = detection_event("person");
    char event_id[EVENT_ID_MAX];
    snprintf(event_id, sizeof(event_id), "%s", event.id);
    char error[256];
    TEST_ASSERT_EQUAL_INT(EVENT_BUS_OK,
                          event_bus_publish(&event, error, sizeof(error)));
    event_envelope_clear(&event);
    TEST_ASSERT_EQUAL_INT(0, event_bus_wait_until_idle(2000));

    pthread_mutex_lock(&capture.mutex);
    TEST_ASSERT_EQUAL_INT(1, capture.calls);
    TEST_ASSERT_EQUAL_STRING(event_id, capture.last_id);
    TEST_ASSERT_EQUAL_STRING("io.lightnvr.detection.object.v1",
                             capture.last_type);
    pthread_mutex_unlock(&capture.mutex);
    event_bus_stats_t stats;
    event_bus_get_stats(&stats);
    TEST_ASSERT_EQUAL_UINT64(1, stats.accepted_events);
    TEST_ASSERT_EQUAL_UINT64(1, stats.dispatched_events);
    TEST_ASSERT_EQUAL_UINT64(1, stats.callback_deliveries);
    TEST_ASSERT_EQUAL_UINT(0, stats.queued_events);

    event_bus_shutdown(true);
    capture_destroy(&capture);
}

void test_invalid_event_is_rejected_before_enqueue(void) {
    TEST_ASSERT_EQUAL_INT(0, event_bus_init(0, 0));
    event_envelope_t event = detection_event("vehicle");
    cJSON_AddStringToObject(event.data, "broker_password", "secret");
    char error[256];
    TEST_ASSERT_EQUAL_INT(
        EVENT_BUS_INVALID_EVENT,
        event_bus_publish(&event, error, sizeof(error)));
    TEST_ASSERT_NOT_NULL(strstr(error, "sensitive or filesystem"));
    event_bus_stats_t stats;
    event_bus_get_stats(&stats);
    TEST_ASSERT_EQUAL_UINT64(0, stats.accepted_events);
    TEST_ASSERT_EQUAL_UINT64(1, stats.rejected_events);
    TEST_ASSERT_EQUAL_UINT(0, stats.queued_events);
    event_envelope_clear(&event);
}

void test_critical_event_sheds_older_lower_priority_when_full(void) {
    capture_context_t capture;
    capture_init(&capture, true);
    TEST_ASSERT_EQUAL_INT(0,
                          event_bus_subscribe("capture", capture_handler,
                                              &capture));
    TEST_ASSERT_EQUAL_INT(0, event_bus_init(2, 0));
    char error[256];

    event_envelope_t first = detection_event("first");
    TEST_ASSERT_EQUAL_INT(EVENT_BUS_OK,
                          event_bus_publish(&first, error, sizeof(error)));
    event_envelope_clear(&first);
    TEST_ASSERT_TRUE(wait_for_handler_start(&capture, 2000));

    event_envelope_t second = detection_event("second");
    event_envelope_t third = detection_event("third");
    TEST_ASSERT_EQUAL_INT(EVENT_BUS_OK,
                          event_bus_publish(&second, error, sizeof(error)));
    TEST_ASSERT_EQUAL_INT(EVENT_BUS_OK,
                          event_bus_publish(&third, error, sizeof(error)));
    event_envelope_clear(&second);
    event_envelope_clear(&third);

    event_envelope_t critical = storage_event();
    TEST_ASSERT_EQUAL_INT(EVENT_BUS_OK,
                          event_bus_publish(&critical, error, sizeof(error)));
    event_envelope_clear(&critical);

    event_envelope_t overflow = detection_event("overflow");
    TEST_ASSERT_EQUAL_INT(
        EVENT_BUS_QUEUE_FULL,
        event_bus_publish(&overflow, error, sizeof(error)));
    event_envelope_clear(&overflow);

    release_handler(&capture);
    TEST_ASSERT_EQUAL_INT(0, event_bus_wait_until_idle(2000));
    event_bus_stats_t stats;
    event_bus_get_stats(&stats);
    TEST_ASSERT_EQUAL_UINT64(4, stats.accepted_events);
    TEST_ASSERT_EQUAL_UINT64(3, stats.dispatched_events);
    TEST_ASSERT_EQUAL_UINT64(2, stats.dropped_events);
    TEST_ASSERT_EQUAL_UINT64(1, stats.priority_shed_events);
    pthread_mutex_lock(&capture.mutex);
    TEST_ASSERT_TRUE(capture.saw_critical);
    pthread_mutex_unlock(&capture.mutex);

    event_bus_shutdown(true);
    capture_destroy(&capture);
}

void test_multiple_subscribers_and_failures_are_observable(void) {
    capture_context_t capture;
    capture_init(&capture, false);
    TEST_ASSERT_EQUAL_INT(0,
                          event_bus_subscribe("capture", capture_handler,
                                              &capture));
    TEST_ASSERT_EQUAL_INT(
        0, event_bus_subscribe("failure", failure_handler, NULL));
    TEST_ASSERT_EQUAL_INT(0, event_bus_init(0, 0));
    event_envelope_t event = storage_event();
    char error[256];
    TEST_ASSERT_EQUAL_INT(EVENT_BUS_OK,
                          event_bus_publish(&event, error, sizeof(error)));
    event_envelope_clear(&event);
    TEST_ASSERT_EQUAL_INT(0, event_bus_wait_until_idle(2000));
    event_bus_stats_t stats;
    event_bus_get_stats(&stats);
    TEST_ASSERT_EQUAL_UINT64(1, stats.dispatched_events);
    TEST_ASSERT_EQUAL_UINT64(2, stats.callback_deliveries);
    TEST_ASSERT_EQUAL_UINT64(1, stats.handler_failures);
    TEST_ASSERT_EQUAL_INT(2, stats.subscriber_count);

    event_bus_shutdown(true);
    capture_destroy(&capture);
}

void test_shutdown_with_drain_delivers_all_accepted_events(void) {
    capture_context_t capture;
    capture_init(&capture, false);
    TEST_ASSERT_EQUAL_INT(0,
                          event_bus_subscribe("capture", capture_handler,
                                              &capture));
    TEST_ASSERT_EQUAL_INT(0, event_bus_init(16, 0));
    char error[256];
    for (int index = 0; index < 10; index++) {
        event_envelope_t event = detection_event("person");
        TEST_ASSERT_EQUAL_INT(EVENT_BUS_OK,
                              event_bus_publish(&event, error,
                                                sizeof(error)));
        event_envelope_clear(&event);
    }
    event_bus_shutdown(true);
    event_bus_stats_t stats;
    event_bus_get_stats(&stats);
    TEST_ASSERT_EQUAL_UINT64(10, stats.accepted_events);
    TEST_ASSERT_EQUAL_UINT64(10, stats.dispatched_events);
    TEST_ASSERT_FALSE(stats.running);
    capture_destroy(&capture);
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_publish_dispatches_a_deep_copy_asynchronously);
    RUN_TEST(test_invalid_event_is_rejected_before_enqueue);
    RUN_TEST(test_critical_event_sheds_older_lower_priority_when_full);
    RUN_TEST(test_multiple_subscribers_and_failures_are_observable);
    RUN_TEST(test_shutdown_with_drain_delivers_all_accepted_events);
    return UNITY_END();
}
