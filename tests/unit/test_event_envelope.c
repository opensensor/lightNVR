/**
 * @file test_event_envelope.c
 * @brief Versioned event registry, envelope, schema, and privacy tests.
 */

#include <stdlib.h>
#include <stdio.h>
#include <string.h>

#include <cjson/cJSON.h>

#include "core/event_envelope.h"
#include "unity.h"

#define INSTALLATION_SOURCE \
    "urn:lightnvr:11111111-1111-4111-8111-111111111111"
#define CAMERA_SUBJECT "camera/22222222-2222-4222-8222-222222222222"

void setUp(void) {}
void tearDown(void) {}

static cJSON *detection_fixture(void) {
    cJSON *data = cJSON_CreateObject();
    cJSON_AddNumberToObject(data, "count", 1);
    cJSON *detections = cJSON_AddArrayToObject(data, "detections");
    cJSON *detection = cJSON_CreateObject();
    cJSON_AddStringToObject(detection, "label", "person");
    cJSON_AddNumberToObject(detection, "confidence", 0.94);
    cJSON_AddNumberToObject(detection, "x", 0.1);
    cJSON_AddNumberToObject(detection, "y", 0.2);
    cJSON_AddNumberToObject(detection, "width", 0.3);
    cJSON_AddNumberToObject(detection, "height", 0.4);
    cJSON_AddStringToObject(detection, "zone_id", "loading-bay");
    cJSON_AddItemToArray(detections, detection);
    cJSON_AddStringToObject(data, "snapshot_url",
                            "/api/events/media/event-id");
    return data;
}

static cJSON *camera_offline_fixture(void) {
    cJSON *data = cJSON_CreateObject();
    cJSON_AddStringToObject(data, "reason", "connection_timeout");
    cJSON_AddNumberToObject(data, "consecutive_failures", 3);
    return data;
}

static cJSON *recording_gap_fixture(void) {
    cJSON *data = cJSON_CreateObject();
    cJSON_AddStringToObject(data, "started_at", "2026-08-23T06:30:00Z");
    cJSON_AddNumberToObject(data, "duration_ms", 12500);
    return data;
}

static cJSON *storage_pressure_fixture(void) {
    cJSON *data = cJSON_CreateObject();
    cJSON_AddStringToObject(data, "level", "critical");
    cJSON_AddNumberToObject(data, "used_percent", 94.5);
    cJSON_AddNumberToObject(data, "free_bytes", 1073741824.0);
    return data;
}

void test_registry_is_stable_and_versioned(void) {
    int count = 0;
    const event_type_definition_t *registry = event_registry_all(&count);
    TEST_ASSERT_NOT_NULL(registry);
    TEST_ASSERT_EQUAL_INT(4, count);
    for (int index = 0; index < count; index++) {
        TEST_ASSERT_NOT_NULL(strstr(registry[index].type, "io.lightnvr."));
        size_t length = strlen(registry[index].type);
        TEST_ASSERT_GREATER_THAN(3, length);
        TEST_ASSERT_EQUAL_STRING(".v1", registry[index].type + length - 3);
        TEST_ASSERT_GREATER_THAN(0, registry[index].default_expiry_seconds);
        TEST_ASSERT_NOT_NULL(event_registry_find(registry[index].type));
        for (int other = index + 1; other < count; other++) {
            TEST_ASSERT_NOT_EQUAL(0,
                                  strcmp(registry[index].type,
                                         registry[other].type));
        }
    }
    TEST_ASSERT_EQUAL_STRING("critical",
                             event_severity_name(EVENT_SEVERITY_CRITICAL));
    TEST_ASSERT_EQUAL_STRING(
        "reference_allowed",
        event_media_policy_name(EVENT_MEDIA_REFERENCE_ALLOWED));
    TEST_ASSERT_NULL(event_registry_find("io.lightnvr.unknown.v1"));
}

void test_required_fixtures_create_and_validate(void) {
    struct {
        const char *type;
        const char *subject;
        cJSON *(*fixture)(void);
    } cases[] = {
        {"io.lightnvr.detection.object.v1", CAMERA_SUBJECT,
         detection_fixture},
        {"io.lightnvr.camera.offline.v1", CAMERA_SUBJECT,
         camera_offline_fixture},
        {"io.lightnvr.stream.recording_gap.v1", CAMERA_SUBJECT,
         recording_gap_fixture},
        {"io.lightnvr.storage.pressure.v1", "system/storage",
         storage_pressure_fixture},
    };
    for (size_t index = 0; index < sizeof(cases) / sizeof(cases[0]); index++) {
        cJSON *data = cases[index].fixture();
        event_envelope_t event;
        char error[256];
        TEST_ASSERT_EQUAL_INT(
            0, event_envelope_create(&event, cases[index].type,
                                     INSTALLATION_SOURCE, cases[index].subject,
                                     1787466600, data, error, sizeof(error)));
        TEST_ASSERT_EQUAL_INT(0, event_envelope_validate(
                                     &event, error, sizeof(error)));
        TEST_ASSERT_EQUAL_UINT(36, strlen(event.id));
        TEST_ASSERT_EQUAL_STRING("2026-08-23T06:30:00Z", event.time);
        TEST_ASSERT_GREATER_THAN(event.occurred_at, event.expires_at);
        event_envelope_clear(&event);
        cJSON_Delete(data);
    }
}

void test_serialization_is_cloudevents_shaped_and_identity_is_stable(void) {
    cJSON *data = detection_fixture();
    event_envelope_t event;
    char error[256];
    TEST_ASSERT_EQUAL_INT(
        0, event_envelope_create(&event,
                                 "io.lightnvr.detection.object.v1",
                                 INSTALLATION_SOURCE, CAMERA_SUBJECT,
                                 1787466600, data, error, sizeof(error)));
    char original_id[EVENT_ID_MAX];
    snprintf(original_id, sizeof(original_id), "%s", event.id);

    cJSON_ReplaceItemInObjectCaseSensitive(data, "count",
                                            cJSON_CreateNumber(99));
    char *first = event_envelope_serialize(&event, error, sizeof(error));
    char *second = event_envelope_serialize(&event, error, sizeof(error));
    TEST_ASSERT_NOT_NULL(first);
    TEST_ASSERT_NOT_NULL(second);
    TEST_ASSERT_EQUAL_STRING(first, second);
    TEST_ASSERT_EQUAL_STRING(original_id, event.id);

    cJSON *serialized = cJSON_Parse(first);
    TEST_ASSERT_TRUE(cJSON_IsObject(serialized));
    TEST_ASSERT_EQUAL_STRING(
        "1.0", cJSON_GetObjectItemCaseSensitive(serialized,
                                                 "specversion")->valuestring);
    TEST_ASSERT_EQUAL_STRING(
        "application/json",
        cJSON_GetObjectItemCaseSensitive(serialized,
                                         "datacontenttype")->valuestring);
    TEST_ASSERT_EQUAL_STRING(
        "operational",
        cJSON_GetObjectItemCaseSensitive(serialized,
                                         "sensitivity")->valuestring);
    TEST_ASSERT_EQUAL_INT(
        1, cJSON_GetObjectItemCaseSensitive(
               cJSON_GetObjectItemCaseSensitive(serialized, "data"),
               "count")->valueint);
    cJSON_Delete(serialized);
    free(first);
    free(second);
    event_envelope_clear(&event);
    cJSON_Delete(data);
}

void test_identity_is_unique_across_events(void) {
    cJSON *data = camera_offline_fixture();
    event_envelope_t first;
    event_envelope_t second;
    char error[256];
    TEST_ASSERT_EQUAL_INT(
        0, event_envelope_create(&first, "io.lightnvr.camera.offline.v1",
                                 INSTALLATION_SOURCE, CAMERA_SUBJECT, 0, data,
                                 error, sizeof(error)));
    TEST_ASSERT_EQUAL_INT(
        0, event_envelope_create(&second, "io.lightnvr.camera.offline.v1",
                                 INSTALLATION_SOURCE, CAMERA_SUBJECT, 0, data,
                                 error, sizeof(error)));
    TEST_ASSERT_NOT_EQUAL(0, strcmp(first.id, second.id));
    event_envelope_clear(&first);
    event_envelope_clear(&second);
    cJSON_Delete(data);
}

void test_subject_and_per_type_schema_fail_closed(void) {
    cJSON *data = camera_offline_fixture();
    event_envelope_t event;
    char error[256];
    TEST_ASSERT_EQUAL_INT(
        -1, event_envelope_create(&event, "io.lightnvr.camera.offline.v1",
                                  INSTALLATION_SOURCE, "camera/mutable-name",
                                  0, data, error, sizeof(error)));
    TEST_ASSERT_NOT_NULL(strstr(error, "camera/<uuid>"));
    cJSON_DeleteItemFromObjectCaseSensitive(data, "consecutive_failures");
    TEST_ASSERT_EQUAL_INT(
        -1, event_envelope_create(&event, "io.lightnvr.camera.offline.v1",
                                  INSTALLATION_SOURCE, CAMERA_SUBJECT, 0, data,
                                  error, sizeof(error)));
    TEST_ASSERT_NOT_NULL(strstr(error, "consecutive_failures"));
    cJSON_Delete(data);
}

void test_detection_bounding_box_is_optional_but_atomic(void) {
    cJSON *data = detection_fixture();
    cJSON *detection = cJSON_GetArrayItem(
        cJSON_GetObjectItemCaseSensitive(data, "detections"), 0);
    cJSON_DeleteItemFromObjectCaseSensitive(detection, "x");
    cJSON_DeleteItemFromObjectCaseSensitive(detection, "y");
    cJSON_DeleteItemFromObjectCaseSensitive(detection, "width");
    cJSON_DeleteItemFromObjectCaseSensitive(detection, "height");

    event_envelope_t event;
    char error[256] = {0};
    TEST_ASSERT_EQUAL_INT(
        0, event_envelope_create(&event,
                                 "io.lightnvr.detection.object.v1",
                                 INSTALLATION_SOURCE, CAMERA_SUBJECT, 0,
                                 data, error, sizeof(error)));
    event_envelope_clear(&event);

    cJSON_AddNumberToObject(detection, "x", 0.2);
    TEST_ASSERT_EQUAL_INT(
        -1, event_envelope_create(&event,
                                  "io.lightnvr.detection.object.v1",
                                  INSTALLATION_SOURCE, CAMERA_SUBJECT, 0,
                                  data, error, sizeof(error)));
    TEST_ASSERT_NOT_NULL(strstr(error, "bounding boxes"));
    cJSON_Delete(data);
}

void test_sensitive_and_filesystem_fields_are_rejected_recursively(void) {
    cJSON *data = detection_fixture();
    cJSON *context = cJSON_AddObjectToObject(data, "context");
    cJSON_AddStringToObject(context, "mqtt_password", "do-not-publish");
    event_envelope_t event;
    char error[256];
    TEST_ASSERT_EQUAL_INT(
        -1, event_envelope_create(&event,
                                  "io.lightnvr.detection.object.v1",
                                  INSTALLATION_SOURCE, CAMERA_SUBJECT, 0, data,
                                  error, sizeof(error)));
    TEST_ASSERT_NOT_NULL(strstr(error, "sensitive or filesystem"));

    cJSON_DeleteItemFromObjectCaseSensitive(data, "context");
    cJSON_AddStringToObject(data, "snapshot_path",
                            "/var/lib/lightnvr/snapshot.jpg");
    TEST_ASSERT_EQUAL_INT(
        -1, event_envelope_create(&event,
                                  "io.lightnvr.detection.object.v1",
                                  INSTALLATION_SOURCE, CAMERA_SUBJECT, 0, data,
                                  error, sizeof(error)));
    TEST_ASSERT_NOT_NULL(strstr(error, "sensitive or filesystem"));

    cJSON_DeleteItemFromObjectCaseSensitive(data, "snapshot_path");
    cJSON_AddStringToObject(data, "diagnostic", "/var/lib/lightnvr/raw.jpg");
    TEST_ASSERT_EQUAL_INT(
        -1, event_envelope_create(&event,
                                  "io.lightnvr.detection.object.v1",
                                  INSTALLATION_SOURCE, CAMERA_SUBJECT, 0, data,
                                  error, sizeof(error)));
    TEST_ASSERT_NOT_NULL(strstr(error, "raw filesystem path"));
    cJSON_Delete(data);
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_registry_is_stable_and_versioned);
    RUN_TEST(test_required_fixtures_create_and_validate);
    RUN_TEST(test_serialization_is_cloudevents_shaped_and_identity_is_stable);
    RUN_TEST(test_identity_is_unique_across_events);
    RUN_TEST(test_subject_and_per_type_schema_fail_closed);
    RUN_TEST(test_detection_bounding_box_is_optional_but_atomic);
    RUN_TEST(test_sensitive_and_filesystem_fields_are_rejected_recursively);
    return UNITY_END();
}
