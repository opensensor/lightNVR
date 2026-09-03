#include "unity.h"

#include <string.h>

#include "core/mqtt_presence.h"

static const char *installation = "11111111-1111-4111-8111-111111111111";
static const char *run = "22222222-2222-4222-8222-222222222222";
static const char *boot = "33333333-3333-4333-8333-333333333333";

void setUp(void) { mqtt_presence_reset(); }
void tearDown(void) { mqtt_presence_reset(); }

static void test_builds_exact_topic_and_bounded_documents(void) {
    TEST_ASSERT_EQUAL_INT(0, mqtt_presence_configure(
        "lightnvr/", installation, run, boot,
        "process,container,host,filesystem", "0.41.4"));
    char topic[MQTT_PRESENCE_TOPIC_MAX];
    char payload[MQTT_PRESENCE_PAYLOAD_MAX];
    TEST_ASSERT_EQUAL_INT(0, mqtt_presence_build_will(
        1000, topic, sizeof(topic), payload, sizeof(payload)));
    TEST_ASSERT_EQUAL_STRING(
        "lightnvr/v1/status/11111111-1111-4111-8111-111111111111", topic);
    TEST_ASSERT_NOT_NULL(strstr(payload, "\"state\":\"offline\""));
    TEST_ASSERT_NOT_NULL(strstr(payload, "\"sequence\":0"));
    TEST_ASSERT_NOT_NULL(strstr(payload, "\"run_id\":\"22222222-"));
    TEST_ASSERT_NOT_NULL(strstr(payload,
                                "\"visibility_scope\":\"process,container,host,filesystem\""));

    TEST_ASSERT_EQUAL_INT(0, mqtt_presence_build(
        MQTT_PRESENCE_ONLINE, MQTT_OPERATIONAL_WARNING, 2U, 2000,
        topic, sizeof(topic), payload, sizeof(payload)));
    TEST_ASSERT_NOT_NULL(strstr(payload, "\"state\":\"online\""));
    TEST_ASSERT_NOT_NULL(strstr(payload, "\"overall_state\":\"warning\""));
    TEST_ASSERT_NOT_NULL(strstr(payload, "\"active_incidents\":2"));
    TEST_ASSERT_NOT_NULL(strstr(payload, "\"sequence\":1"));
}

static void test_reconnect_and_reconfigure_preserve_run_sequence(void) {
    TEST_ASSERT_EQUAL_INT(0, mqtt_presence_configure(
        "first", installation, run, boot, "host", "1.0"));
    mqtt_presence_record_connected();
    mqtt_presence_record_disconnected();
    mqtt_presence_record_connected();
    mqtt_presence_record_publish(true, 1000);
    mqtt_presence_record_publish(false, 2000);

    char topic[MQTT_PRESENCE_TOPIC_MAX];
    char payload[MQTT_PRESENCE_PAYLOAD_MAX];
    TEST_ASSERT_EQUAL_INT(0, mqtt_presence_build(
        MQTT_PRESENCE_ONLINE, MQTT_OPERATIONAL_HEALTHY, 0U, 3000,
        topic, sizeof(topic), payload, sizeof(payload)));
    TEST_ASSERT_EQUAL_INT(0, mqtt_presence_configure(
        "second", installation, run, boot, "host", "1.0"));
    TEST_ASSERT_EQUAL_INT(0, mqtt_presence_build(
        MQTT_PRESENCE_STOPPING, MQTT_OPERATIONAL_HEALTHY, 0U, 4000,
        topic, sizeof(topic), payload, sizeof(payload)));
    TEST_ASSERT_EQUAL_STRING(
        "second/v1/status/11111111-1111-4111-8111-111111111111", topic);
    TEST_ASSERT_NOT_NULL(strstr(payload, "\"sequence\":2"));

    mqtt_presence_stats_t stats;
    mqtt_presence_get_stats(&stats);
    TEST_ASSERT_TRUE(stats.configured);
    TEST_ASSERT_TRUE(stats.connected);
    TEST_ASSERT_EQUAL_UINT64(2U, stats.connections);
    TEST_ASSERT_EQUAL_UINT64(1U, stats.reconnects);
    TEST_ASSERT_EQUAL_UINT64(1U, stats.disconnects);
    TEST_ASSERT_EQUAL_UINT64(2U, stats.publish_attempts);
    TEST_ASSERT_EQUAL_UINT64(1U, stats.publish_successes);
    TEST_ASSERT_EQUAL_UINT64(1U, stats.publish_failures);
    TEST_ASSERT_EQUAL_INT64(2000, stats.last_publish_at_ms);
}

static void test_rejects_unsafe_or_truncated_contract_values(void) {
    TEST_ASSERT_EQUAL_INT(-1, mqtt_presence_configure(
        "bad/#", installation, run, boot, "host", "1.0"));
    TEST_ASSERT_EQUAL_INT(-1, mqtt_presence_configure(
        "good", "not-a-uuid", run, boot, "host", "1.0"));
    TEST_ASSERT_EQUAL_INT(0, mqtt_presence_configure(
        "good", installation, run, boot, "host", "1.0"));
    char topic[8];
    char payload[8];
    TEST_ASSERT_EQUAL_INT(-1, mqtt_presence_build_will(
        1, topic, sizeof(topic), payload, sizeof(payload)));
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_builds_exact_topic_and_bounded_documents);
    RUN_TEST(test_reconnect_and_reconfigure_preserve_run_sequence);
    RUN_TEST(test_rejects_unsafe_or_truncated_contract_values);
    return UNITY_END();
}
