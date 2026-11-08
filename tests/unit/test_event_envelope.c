/**
 * @file test_event_envelope.c
 * @brief Versioned event registry, envelope, schema, and privacy tests.
 */

#define _GNU_SOURCE

#include <stdbool.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <time.h>

#include <dirent.h>

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

static cJSON *lpr_read_fixture(void) {
    cJSON *data = cJSON_CreateObject();
    cJSON_AddStringToObject(data, "read_id",
                            "44444444-4444-4444-8444-444444444444");
    cJSON_AddStringToObject(data, "stream_name", "north-drive");
    cJSON_AddStringToObject(data, "source", "onvif_profile_m");
    return data;
}

static cJSON *camera_offline_fixture(void) {
    cJSON *data = cJSON_CreateObject();
    cJSON_AddStringToObject(data, "reason", "connection_timeout");
    cJSON_AddNumberToObject(data, "consecutive_failures", 3);
    return data;
}

static cJSON *camera_recovered_fixture(void) {
    cJSON *data = cJSON_CreateObject();
    cJSON_AddStringToObject(data, "previous_state", "offline");
    cJSON_AddNumberToObject(data, "downtime_ms", 15000);
    return data;
}

static cJSON *stream_degraded_fixture(void) {
    cJSON *data = cJSON_CreateObject();
    cJSON_AddStringToObject(data, "reason", "low_fps");
    cJSON_AddNumberToObject(data, "observed_fps", 7.5);
    cJSON_AddNumberToObject(data, "expected_fps", 25.0);
    return data;
}

static cJSON *stream_recovered_fixture(void) {
    cJSON *data = cJSON_CreateObject();
    cJSON_AddStringToObject(data, "previous_state", "degraded");
    cJSON_AddNumberToObject(data, "observed_fps", 24.5);
    cJSON_AddNumberToObject(data, "expected_fps", 25.0);
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

static cJSON *storage_recovered_fixture(void) {
    cJSON *data = cJSON_CreateObject();
    cJSON_AddStringToObject(data, "previous_level", "critical");
    cJSON_AddNumberToObject(data, "used_percent", 72.0);
    cJSON_AddNumberToObject(data, "free_bytes", 3221225472.0);
    return data;
}

static cJSON *storage_target_unavailable_fixture(void) {
    cJSON *data = cJSON_CreateObject();
    cJSON_AddStringToObject(data, "target_uuid",
                            "33333333-3333-4333-8333-333333333333");
    cJSON_AddStringToObject(data, "previous_state", "healthy");
    cJSON_AddStringToObject(data, "reason", "mount_unavailable");
    cJSON_AddBoolToObject(data, "is_default", false);
    return data;
}

static cJSON *storage_target_recovered_fixture(void) {
    cJSON *data = cJSON_CreateObject();
    cJSON_AddStringToObject(data, "target_uuid",
                            "33333333-3333-4333-8333-333333333333");
    cJSON_AddStringToObject(data, "previous_state", "unavailable");
    cJSON_AddStringToObject(data, "current_state", "healthy");
    cJSON_AddNumberToObject(data, "downtime_ms", 60000);
    cJSON_AddBoolToObject(data, "is_default", false);
    return data;
}

static cJSON *health_alert_fixture(const char *severity) {
    cJSON *data = cJSON_CreateObject();
    cJSON_AddStringToObject(data, "incident_id",
                            "44444444-4444-4444-8444-444444444444");
    cJSON_AddStringToObject(data, "code", "memory.available_low");
    cJSON_AddStringToObject(data, "scope", "container");
    cJSON_AddStringToObject(data, "resource", "memory");
    cJSON_AddStringToObject(data, "state", "open");
    cJSON_AddStringToObject(data, "severity", severity);
    cJSON *observed = cJSON_AddObjectToObject(data, "observed");
    cJSON_AddNumberToObject(observed, "value", 0.07);
    cJSON_AddStringToObject(observed, "unit", "ratio");
    cJSON *threshold = cJSON_AddObjectToObject(data, "threshold");
    cJSON_AddStringToObject(threshold, "operator", "lt");
    cJSON_AddNumberToObject(threshold, "value", 0.08);
    cJSON_AddNumberToObject(threshold, "for_ms", 30000);
    return data;
}

void test_registry_is_stable_and_versioned(void) {
    int count = 0;
    const event_type_definition_t *registry = event_registry_all(&count);
    TEST_ASSERT_NOT_NULL(registry);
    TEST_ASSERT_EQUAL_INT(14, count);
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

void test_system_severity_range_subjects_and_persisted_identity(void) {
    cJSON *data = health_alert_fixture("critical");
    event_envelope_t event;
    char error[256];
    const char *persisted = "55555555-5555-4555-8555-555555555555";
    TEST_ASSERT_EQUAL_INT(0, event_envelope_create_with_severity_and_id(
        &event, "io.lightnvr.system.health_alert.v1", INSTALLATION_SOURCE,
        "system/container", 1787466600, data, EVENT_SEVERITY_CRITICAL,
        persisted, error, sizeof(error)));
    TEST_ASSERT_EQUAL_STRING(persisted, event.id);
    TEST_ASSERT_EQUAL(EVENT_SEVERITY_CRITICAL, event.severity);

    event_envelope_t clone;
    TEST_ASSERT_EQUAL_INT(0, event_envelope_clone(
        &clone, &event, error, sizeof(error)));
    TEST_ASSERT_EQUAL(EVENT_SEVERITY_CRITICAL, clone.severity);
    char *serialized = event_envelope_serialize(&clone, error, sizeof(error));
    TEST_ASSERT_NOT_NULL(serialized);
    TEST_ASSERT_NOT_NULL(strstr(serialized, "\"severity\":\"critical\""));
    free(serialized);
    event_envelope_clear(&clone);
    event_envelope_clear(&event);

    TEST_ASSERT_EQUAL_INT(-1, event_envelope_create_with_severity_and_id(
        &event, "io.lightnvr.system.health_alert.v1", INSTALLATION_SOURCE,
        "system/container", 1787466600, data, EVENT_SEVERITY_INFO,
        NULL, error, sizeof(error)));
    TEST_ASSERT_NOT_NULL(strstr(error, "severity"));
    TEST_ASSERT_EQUAL_INT(-1, event_envelope_create_with_severity_and_id(
        &event, "io.lightnvr.system.health_alert.v1", INSTALLATION_SOURCE,
        "system/unknown", 1787466600, data, EVENT_SEVERITY_CRITICAL,
        NULL, error, sizeof(error)));
    cJSON_Delete(data);
}

void test_system_payload_severity_must_match_envelope(void) {
    cJSON *data = health_alert_fixture("warning");
    event_envelope_t event;
    char error[256];
    TEST_ASSERT_EQUAL_INT(-1, event_envelope_create_with_severity_and_id(
        &event, "io.lightnvr.system.health_alert.v1", INSTALLATION_SOURCE,
        "system/container", 1787466600, data, EVENT_SEVERITY_ERROR,
        NULL, error, sizeof(error)));
    TEST_ASSERT_NOT_NULL(strstr(error, "severity"));
    cJSON_Delete(data);
}

void test_required_fixtures_create_and_validate(void) {
    struct {
        const char *type;
        const char *subject;
        cJSON *(*fixture)(void);
    } cases[] = {
        {"io.lightnvr.detection.object.v1", CAMERA_SUBJECT,
         detection_fixture},
        {"io.lightnvr.recognition.license_plate.v1", CAMERA_SUBJECT,
         lpr_read_fixture},
        {"io.lightnvr.camera.offline.v1", CAMERA_SUBJECT,
         camera_offline_fixture},
        {"io.lightnvr.camera.recovered.v1", CAMERA_SUBJECT,
         camera_recovered_fixture},
        {"io.lightnvr.stream.degraded.v1", CAMERA_SUBJECT,
         stream_degraded_fixture},
        {"io.lightnvr.stream.recovered.v1", CAMERA_SUBJECT,
         stream_recovered_fixture},
        {"io.lightnvr.stream.recording_gap.v1", CAMERA_SUBJECT,
         recording_gap_fixture},
        {"io.lightnvr.storage.pressure.v1", "system/storage",
         storage_pressure_fixture},
        {"io.lightnvr.storage.recovered.v1", "system/storage",
         storage_recovered_fixture},
        {"io.lightnvr.storage.target_unavailable.v1", "system/storage",
         storage_target_unavailable_fixture},
        {"io.lightnvr.storage.target_recovered.v1", "system/storage",
         storage_target_recovered_fixture},
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
    cJSON_AddStringToObject(data, "license_plate", "TEST123");
    TEST_ASSERT_EQUAL_INT(
        -1, event_envelope_create(&event,
                                  "io.lightnvr.detection.object.v1",
                                  INSTALLATION_SOURCE, CAMERA_SUBJECT, 0, data,
                                  error, sizeof(error)));
    TEST_ASSERT_NOT_NULL(strstr(error, "sensitive or filesystem"));

    cJSON_DeleteItemFromObjectCaseSensitive(data, "license_plate");
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

void test_lpr_notification_is_restricted_and_plate_free(void) {
    cJSON *data = lpr_read_fixture();
    event_envelope_t event;
    char error[256] = {0};
    TEST_ASSERT_EQUAL_INT(
        0, event_envelope_create(
               &event, "io.lightnvr.recognition.license_plate.v1",
               INSTALLATION_SOURCE, CAMERA_SUBJECT, 1787466600, data,
               error, sizeof(error)));
    char *serialized = event_envelope_serialize(&event, error, sizeof(error));
    TEST_ASSERT_NOT_NULL(serialized);
    TEST_ASSERT_NOT_NULL(strstr(serialized, "\"sensitivity\":\"restricted\""));
    TEST_ASSERT_NULL(strstr(serialized, "TEST123"));
    TEST_ASSERT_NULL(strstr(serialized, "\"plate\":"));
    free(serialized);
    event_envelope_clear(&event);

    cJSON_AddStringToObject(data, "plate", "TEST123");
    TEST_ASSERT_EQUAL_INT(
        -1, event_envelope_create(
                &event, "io.lightnvr.recognition.license_plate.v1",
                INSTALLATION_SOURCE, CAMERA_SUBJECT, 1787466600, data,
                error, sizeof(error)));
    cJSON_Delete(data);
}

/* ---- INT_01 L5: io.lightnvr.detection.observation_changed.v1 ---------- */

#define OBSERVATION_TYPE "io.lightnvr.detection.observation_changed.v1"
#ifndef LIGHTNVR_SOURCE_DIR
#error "LIGHTNVR_SOURCE_DIR must be defined"
#endif
#define OBSERVATION_EVENT_FIXTURES \
    LIGHTNVR_SOURCE_DIR "/tests/fixtures/integrations/observations/events"

static char *read_text_file(const char *path) {
    FILE *file = fopen(path, "rb");
    if (!file) return NULL;
    fseek(file, 0, SEEK_END);
    long size = ftell(file);
    fseek(file, 0, SEEK_SET);
    char *buffer = calloc(1, (size_t)size + 1);
    if (buffer && fread(buffer, 1, (size_t)size, file) != (size_t)size) {
        free(buffer);
        buffer = NULL;
    }
    fclose(file);
    return buffer;
}

static time_t parse_event_time(const char *text) {
    struct tm tm;
    memset(&tm, 0, sizeof(tm));
    TEST_ASSERT_EQUAL_INT(6, sscanf(text, "%4d-%2d-%2dT%2d:%2d:%2dZ",
                                    &tm.tm_year, &tm.tm_mon, &tm.tm_mday,
                                    &tm.tm_hour, &tm.tm_min, &tm.tm_sec));
    tm.tm_year -= 1900;
    tm.tm_mon -= 1;
    return timegm(&tm);
}

/* Decode a fixture MQTT envelope through the normal constructor. */
static int create_from_fixture(const cJSON *fixture, const cJSON *data,
                               const char *subject_override,
                               event_envelope_t *event, char *error,
                               size_t error_size) {
    const cJSON *subject = cJSON_GetObjectItemCaseSensitive(fixture, "subject");
    return event_envelope_create_with_severity_and_id(
        event, cJSON_GetObjectItemCaseSensitive(fixture, "type")->valuestring,
        cJSON_GetObjectItemCaseSensitive(fixture, "source")->valuestring,
        subject_override ? subject_override : subject->valuestring,
        parse_event_time(
            cJSON_GetObjectItemCaseSensitive(fixture, "time")->valuestring),
        data, EVENT_SEVERITY_INFO,
        cJSON_GetObjectItemCaseSensitive(fixture, "id")->valuestring, error,
        error_size);
}

static cJSON *observation_fixture(const char *name) {
    char path[1024];
    snprintf(path, sizeof(path), "%s/%s", OBSERVATION_EVENT_FIXTURES, name);
    char *text = read_text_file(path);
    TEST_ASSERT_NOT_NULL_MESSAGE(text, path);
    cJSON *fixture = cJSON_Parse(text);
    free(text);
    TEST_ASSERT_NOT_NULL_MESSAGE(fixture, path);
    return fixture;
}

void test_observation_changed_registry_entry(void) {
    const event_type_definition_t *definition =
        event_registry_find(OBSERVATION_TYPE);
    TEST_ASSERT_NOT_NULL(definition);
    TEST_ASSERT_EQUAL_STRING("detection", definition->family);
    TEST_ASSERT_EQUAL_STRING("A durable object observation was created or changed",
                             definition->description);
    TEST_ASSERT_EQUAL_INT(EVENT_SEVERITY_INFO, definition->severity);
    TEST_ASSERT_FALSE(definition->dynamic_severity);
    TEST_ASSERT_EQUAL_INT(EVENT_SENSITIVITY_OPERATIONAL, definition->sensitivity);
    TEST_ASSERT_EQUAL_INT(EVENT_MEDIA_REFERENCE_ALLOWED, definition->media_policy);
    TEST_ASSERT_EQUAL_INT(EVENT_RATE_HIGH, definition->expected_rate);
    TEST_ASSERT_EQUAL_INT(EVENT_SUBJECT_CAMERA, definition->subject_kind);
    TEST_ASSERT_EQUAL_INT(3600, definition->default_expiry_seconds);
}

void test_observation_changed_validator_accepts_c3_fixtures(void) {
    DIR *dir = opendir(OBSERVATION_EVENT_FIXTURES);
    TEST_ASSERT_NOT_NULL_MESSAGE(dir, OBSERVATION_EVENT_FIXTURES);
    int checked = 0;
    struct dirent *entry;
    while ((entry = readdir(dir)) != NULL) {
        size_t length = strlen(entry->d_name);
        if (length < 6 || strcmp(entry->d_name + length - 5, ".json") != 0) {
            continue;
        }
        cJSON *fixture = observation_fixture(entry->d_name);
        event_envelope_t event;
        char error[256] = "";
        TEST_ASSERT_EQUAL_INT_MESSAGE(
            0, create_from_fixture(fixture,
                                   cJSON_GetObjectItemCaseSensitive(fixture, "data"),
                                   NULL, &event, error, sizeof(error)),
            error);
        char *serialized = event_envelope_serialize(&event, error, sizeof(error));
        char *expected = cJSON_PrintUnformatted(fixture);
        TEST_ASSERT_EQUAL_STRING_MESSAGE(expected, serialized, entry->d_name);
        free(serialized);
        free(expected);
        event_envelope_clear(&event);
        cJSON_Delete(fixture);
        checked++;
    }
    closedir(dir);
    TEST_ASSERT_EQUAL_INT(8, checked);
}

static void assert_observation_rejected(const char *fixture_name,
                                        void (*mutate)(cJSON *data),
                                        const char *subject_override,
                                        const char *label) {
    cJSON *fixture = observation_fixture(fixture_name);
    cJSON *data = cJSON_Duplicate(
        cJSON_GetObjectItemCaseSensitive(fixture, "data"), true);
    if (mutate) mutate(data);
    event_envelope_t event;
    char error[256] = "";
    TEST_ASSERT_EQUAL_INT_MESSAGE(
        -1, create_from_fixture(fixture, data, subject_override, &event,
                                error, sizeof(error)),
        label);
    TEST_ASSERT_NOT_EQUAL_MESSAGE(0, strlen(error), label);
    cJSON_Delete(data);
    cJSON_Delete(fixture);
}

static void drop_observation_id(cJSON *d) { cJSON_DeleteItemFromObject(d, "observation_id"); }
static void drop_revision(cJSON *d) { cJSON_DeleteItemFromObject(d, "revision"); }
static void drop_camera(cJSON *d) { cJSON_DeleteItemFromObject(d, "camera_uuid"); }
static void drop_label(cJSON *d) { cJSON_DeleteItemFromObject(d, "label"); }
static void drop_observed_at(cJSON *d) { cJSON_DeleteItemFromObject(d, "observed_at"); }
static void drop_cause(cJSON *d) { cJSON_DeleteItemFromObject(d, "change_cause"); }
static void drop_state(cJSON *d) { cJSON_DeleteItemFromObject(d, "evidence_state"); }
static void drop_reason(cJSON *d) { cJSON_DeleteItemFromObject(d, "evidence_reason"); }
static void drop_asset(cJSON *d) { cJSON_DeleteItemFromObject(d, "asset_id"); }
static void drop_snapshot(cJSON *d) { cJSON_DeleteItemFromObject(d, "snapshot_url"); }
static void bad_observation_id(cJSON *d) {
    cJSON_ReplaceItemInObject(d, "observation_id", cJSON_CreateString("bird-1"));
}
static void zero_revision(cJSON *d) {
    cJSON_ReplaceItemInObject(d, "revision", cJSON_CreateNumber(0));
}
static void fractional_revision(cJSON *d) {
    cJSON_ReplaceItemInObject(d, "revision", cJSON_CreateNumber(2.5));
}
static void long_label(cJSON *d) {
    cJSON_ReplaceItemInObject(d, "label",
                              cJSON_CreateString("abcdefghijklmnopqrstuvwxyz012345"));
}
static void empty_label(cJSON *d) {
    cJSON_ReplaceItemInObject(d, "label", cJSON_CreateString(""));
}
static void bad_observed_at(cJSON *d) {
    cJSON_ReplaceItemInObject(d, "observed_at", cJSON_CreateString("yesterday"));
}
static void unknown_cause(cJSON *d) {
    cJSON_ReplaceItemInObject(d, "change_cause", cJSON_CreateString("deleted"));
}
static void unknown_state(cJSON *d) {
    cJSON_ReplaceItemInObject(d, "evidence_state", cJSON_CreateString("lost"));
}
static void reason_on_ready(cJSON *d) {
    cJSON_ReplaceItemInObject(d, "evidence_reason", cJSON_CreateString("queue_full"));
}
static void null_reason(cJSON *d) {
    cJSON_ReplaceItemInObject(d, "evidence_reason", cJSON_CreateNull());
}
static void unknown_reason(cJSON *d) {
    cJSON_ReplaceItemInObject(d, "evidence_reason", cJSON_CreateString("gremlins"));
}
static void mismatched_reason(cJSON *d) {
    /* queue_full is an unavailable reason, not an expired one. */
    cJSON_ReplaceItemInObject(d, "evidence_reason", cJSON_CreateString("queue_full"));
}
static void snapshot_on_pending(cJSON *d) {
    cJSON_AddStringToObject(
        d, "snapshot_url",
        "/api/observations/55555555-5555-4555-8555-555555555555/snapshot");
}
static void wrong_snapshot(cJSON *d) {
    cJSON_ReplaceItemInObject(
        d, "snapshot_url",
        cJSON_CreateString("/api/observations/99999999-9999-4999-8999-999999999999/snapshot"));
}
static void bad_asset(cJSON *d) {
    cJSON_ReplaceItemInObject(d, "asset_id", cJSON_CreateNull());
}
static void other_camera(cJSON *d) {
    cJSON_ReplaceItemInObject(
        d, "camera_uuid", cJSON_CreateString("33333333-3333-4333-8333-333333333333"));
}

void test_observation_changed_validator_rejects_contract_violations(void) {
    const char *ready = "a-r2-evidence-ready.json";
    const char *pending = "a-r1-created-pending.json";
    const char *expired = "a-r5-evidence-expired.json";
    const char *unavailable = "b-r1-created-unavailable.json";
    assert_observation_rejected(ready, drop_observation_id, NULL, "missing observation_id");
    assert_observation_rejected(ready, drop_revision, NULL, "missing revision");
    assert_observation_rejected(ready, drop_camera, NULL, "missing camera_uuid");
    assert_observation_rejected(ready, drop_label, NULL, "missing label");
    assert_observation_rejected(ready, drop_observed_at, NULL, "missing observed_at");
    assert_observation_rejected(ready, drop_cause, NULL, "missing change_cause");
    assert_observation_rejected(ready, drop_state, NULL, "missing evidence_state");
    assert_observation_rejected(ready, drop_reason, NULL, "missing evidence_reason");
    assert_observation_rejected(ready, drop_asset, NULL, "missing asset_id");
    assert_observation_rejected(ready, drop_snapshot, NULL, "ready without snapshot_url");
    assert_observation_rejected(ready, bad_observation_id, NULL, "non-UUID observation_id");
    assert_observation_rejected(ready, zero_revision, NULL, "revision 0");
    assert_observation_rejected(ready, fractional_revision, NULL, "fractional revision");
    assert_observation_rejected(ready, long_label, NULL, "32-byte label");
    assert_observation_rejected(ready, empty_label, NULL, "empty label");
    assert_observation_rejected(ready, bad_observed_at, NULL, "non-RFC3339 observed_at");
    assert_observation_rejected(ready, unknown_cause, NULL, "unknown cause");
    assert_observation_rejected(ready, unknown_state, NULL, "unknown state");
    assert_observation_rejected(ready, reason_on_ready, NULL, "reason on ready");
    assert_observation_rejected(ready, wrong_snapshot, NULL, "snapshot_url of another observation");
    assert_observation_rejected(ready, bad_asset, NULL, "null asset_id");
    assert_observation_rejected(ready, other_camera, NULL, "camera_uuid differs from subject");
    assert_observation_rejected(ready, NULL,
                                "camera/33333333-3333-4333-8333-333333333333",
                                "subject differs from camera_uuid");
    assert_observation_rejected(pending, snapshot_on_pending, NULL, "snapshot_url on pending");
    assert_observation_rejected(expired, null_reason, NULL, "expired without reason");
    assert_observation_rejected(expired, mismatched_reason, NULL, "reason invalid for expired");
    assert_observation_rejected(unavailable, null_reason, NULL, "unavailable without reason");
    assert_observation_rejected(unavailable, unknown_reason, NULL, "unknown reason");
    assert_observation_rejected(expired, snapshot_on_pending, NULL, "snapshot_url on expired");
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
    RUN_TEST(test_lpr_notification_is_restricted_and_plate_free);
    RUN_TEST(test_system_severity_range_subjects_and_persisted_identity);
    RUN_TEST(test_system_payload_severity_must_match_envelope);
    RUN_TEST(test_observation_changed_registry_entry);
    RUN_TEST(test_observation_changed_validator_accepts_c3_fixtures);
    RUN_TEST(test_observation_changed_validator_rejects_contract_violations);
    return UNITY_END();
}
