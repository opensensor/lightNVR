#define _GNU_SOURCE
/* Include the production producer so the test controls its broker connection
 * without opening sockets; all capture/publish side effects use linker wraps. */
#ifndef ENABLE_MQTT
#define ENABLE_MQTT 1
#endif
#include "../../src/core/mqtt_client.c"
#include "unity.h"

static const char *camera_a = "11111111-1111-4111-8111-111111111111";
static const char *camera_b = "22222222-2222-4222-8222-222222222222";
static config_t test_config;
static char directory[128];
static int lookups, snapshots, binary_publishes;
static bool changed, missing_after, invalid_before;
static cJSON *published;

int __wrap_get_stream_config_by_name(const char *name, stream_config_t *stream) {
    TEST_ASSERT_EQUAL_STRING("shared-name", name);
    lookups++;
    if (missing_after && snapshots) return -1;
    memset(stream, 0, sizeof(*stream));
    snprintf(stream->camera_uuid, sizeof(stream->camera_uuid), "%s",
             invalid_before ? "invalid" : changed && snapshots ? camera_b : camera_a);
    return 0;
}
bool __wrap_go2rtc_get_snapshot(const char *name, unsigned char **data, size_t *size) {
    TEST_ASSERT_EQUAL_STRING("shared-name", name);
    snapshots++;
    *size = 4;
    *data = malloc(*size);
    TEST_ASSERT_NOT_NULL(*data);
    memcpy(*data, "JPEG", *size);
    return true;
}
int __wrap_mosquitto_publish(struct mosquitto *client, int *mid, const char *topic,
                            int length, const void *payload, int qos, bool retain) {
    (void)client; (void)mid; (void)qos; (void)retain;
    if (strstr(topic, "/snapshot")) binary_publishes++;
    else {
        TEST_ASSERT_NULL(published);
        published = cJSON_ParseWithLength(payload, (size_t)length);
        TEST_ASSERT_NOT_NULL(published);
    }
    return MOSQ_ERR_SUCCESS;
}
void setUp(void) {
    strcpy(directory, "/tmp/lightnvr-mqtt-snapshot-XXXXXX");
    TEST_ASSERT_NOT_NULL(mkdtemp(directory));
    char snapshots_dir[160];
    snprintf(snapshots_dir, sizeof(snapshots_dir), "%s/snapshots", directory);
    TEST_ASSERT_EQUAL_INT(0, mkdir(snapshots_dir, 0700));
    memset(&test_config, 0, sizeof(test_config));
    snprintf(test_config.storage_path, sizeof(test_config.storage_path), "%s", directory);
    strcpy(test_config.mqtt_topic_prefix, "fixture");
    test_config.mqtt_enabled = true;
    mqtt_config = &test_config;
    mosq = (struct mosquitto *)&test_config;
    connected = true;
    changed = missing_after = invalid_before = false;
    lookups = snapshots = binary_publishes = 0;
    published = NULL;
}
void tearDown(void) {
    if (published) {
        cJSON *file = cJSON_GetObjectItemCaseSensitive(published, "snapshot_path");
        if (cJSON_IsString(file)) unlink(file->valuestring);
        cJSON_Delete(published);
    }
    char path[MAX_PATH_LENGTH];
    snprintf(path, sizeof(path), "%s/snapshots/%s", directory, camera_a); rmdir(path);
    snprintf(path, sizeof(path), "%s/snapshots/%s", directory, camera_b); rmdir(path);
    snprintf(path, sizeof(path), "%s/snapshots", directory); rmdir(path);
    TEST_ASSERT_EQUAL_INT(0, rmdir(directory));
    mosq = NULL; mqtt_config = NULL; connected = false;
}
static void publish(void) {
    detection_result_t result = {.count=1};
    strcpy(result.detections[0].label, "fixture");
    TEST_ASSERT_EQUAL_INT(0, mqtt_publish_detection("shared-name", &result, 100));
    TEST_ASSERT_NOT_NULL(published);
}
static void test_stable_capture_has_uuid_namespace_and_actual_bytes(void) {
    publish();
    TEST_ASSERT_EQUAL_INT(1, snapshots);
    TEST_ASSERT_EQUAL_INT(1, binary_publishes);
    TEST_ASSERT_NOT_NULL(cJSON_GetObjectItemCaseSensitive(published, "snapshot_url"));
    const char *url = cJSON_GetObjectItemCaseSensitive(published, "snapshot_url")->valuestring;
    TEST_ASSERT_NOT_NULL(strstr(url, camera_a));
    const char *file = cJSON_GetObjectItemCaseSensitive(published, "snapshot_path")->valuestring;
    FILE *input = fopen(file, "rb"); TEST_ASSERT_NOT_NULL(input);
    char data[4]; TEST_ASSERT_EQUAL_INT(4, fread(data, 1, sizeof(data), input)); fclose(input);
    TEST_ASSERT_EQUAL_MEMORY("JPEG", data, sizeof(data));
}
static void assert_no_snapshot(void) {
    publish();
    TEST_ASSERT_EQUAL_INT(0, binary_publishes);
    TEST_ASSERT_NULL(cJSON_GetObjectItemCaseSensitive(published, "snapshot_url"));
    TEST_ASSERT_NULL(cJSON_GetObjectItemCaseSensitive(published, "snapshot_path"));
    TEST_ASSERT_NULL(cJSON_GetObjectItemCaseSensitive(published, "snapshot_topic"));
    char path[MAX_PATH_LENGTH]; snprintf(path, sizeof(path), "%s/snapshots/%s", directory, camera_a);
    TEST_ASSERT_NOT_EQUAL(0, access(path, F_OK));
}
static void test_name_reused_during_capture_does_not_persist_or_publish_bytes(void) {changed=true;assert_no_snapshot();}
static void test_camera_removed_during_capture_does_not_persist_or_publish_bytes(void) {missing_after=true;assert_no_snapshot();}
static void test_unresolved_capture_does_not_persist_or_publish_bytes(void) {invalid_before=true;assert_no_snapshot();}
int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_stable_capture_has_uuid_namespace_and_actual_bytes);
    RUN_TEST(test_name_reused_during_capture_does_not_persist_or_publish_bytes);
    RUN_TEST(test_camera_removed_during_capture_does_not_persist_or_publish_bytes);
    RUN_TEST(test_unresolved_capture_does_not_persist_or_publish_bytes);
    return UNITY_END();
}
