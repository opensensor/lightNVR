#define _POSIX_C_SOURCE 200809L

#include "unity.h"

#include <mosquitto.h>
#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <time.h>

#include "core/event_identity.h"
#include "core/logger.h"
#include "core/mqtt_destination_client.h"
#include "core/mqtt_presence.h"
#include "database/db_event_destinations.h"
#include "telemetry/system_health.h"
#include "telemetry/system_health_evaluator.h"
#include "utils/memory.h"

#define INSTALLATION_UUID "11111111-1111-4111-8111-111111111111"
#define DESTINATION_UUID_A "22222222-2222-4222-8222-222222222222"
#define DESTINATION_UUID_B "33333333-3333-4333-8333-333333333333"
#define RUN_UUID "44444444-4444-4444-8444-444444444444"

typedef struct {
    void *userdata;
    void (*connect_callback)(struct mosquitto *, void *, int);
    void (*disconnect_callback)(struct mosquitto *, void *, int);
    void (*publish_callback)(struct mosquitto *, void *, int);
} fake_mosquitto_t;

static fake_mosquitto_t fake_clients[EVENT_DESTINATION_MAX_COUNT];
static size_t fake_client_count;
static int call_sequence;
static int will_sequence;
static int connect_sequence;
static int will_calls;
static int publish_calls;
static char last_will_topic[MQTT_PRESENCE_TOPIC_MAX];
static char last_will_payload[MQTT_PRESENCE_PAYLOAD_MAX];
static char last_publish_topic[MQTT_PRESENCE_TOPIC_MAX];
static char last_publish_payload[MQTT_PRESENCE_PAYLOAD_MAX];
static bool last_publish_retain;

static fake_mosquitto_t *fake(struct mosquitto *mosq) {
    return (fake_mosquitto_t *)mosq;
}

static event_destination_t profile(const char *uuid, const char *name,
                                   const char *status_topic) {
    event_destination_t value;
    memset(&value, 0, sizeof(value));
    snprintf(value.uuid, sizeof(value.uuid), "%s", uuid);
    snprintf(value.name, sizeof(value.name), "%s", name);
    value.enabled = true;
    snprintf(value.destination_type, sizeof(value.destination_type), "mqtt");
    snprintf(value.broker_host, sizeof(value.broker_host), "broker.invalid");
    value.broker_port = 1883;
    snprintf(value.client_id, sizeof(value.client_id), "test-%s", name);
    snprintf(value.topic_template, sizeof(value.topic_template),
             "events/{type}/{subject_id}");
    snprintf(value.status_topic_template,
             sizeof(value.status_topic_template), "%s", status_topic);
    snprintf(value.tls_mode, sizeof(value.tls_mode), "disabled");
    value.keepalive_seconds = 60;
    value.qos = 1;
    value.revision = 1;
    return value;
}

void setUp(void) {
    memset(fake_clients, 0, sizeof(fake_clients));
    fake_client_count = 0U;
    call_sequence = 0;
    will_sequence = 0;
    connect_sequence = 0;
    will_calls = 0;
    publish_calls = 0;
    last_will_topic[0] = '\0';
    last_will_payload[0] = '\0';
    last_publish_topic[0] = '\0';
    last_publish_payload[0] = '\0';
    last_publish_retain = false;
}

void tearDown(void) {}

db_event_destination_result_t __wrap_db_event_destination_validate(
    const event_destination_t *destination, const char *password,
    bool validate_password, char *error, size_t error_size) {
    (void)destination;
    (void)password;
    (void)validate_password;
    if (error && error_size) error[0] = '\0';
    return DB_EVENT_DESTINATION_OK;
}

db_event_destination_result_t __wrap_db_event_destination_get_password(
    const char *uuid, int64_t expected_revision, char *password,
    size_t password_size) {
    (void)uuid;
    (void)expected_revision;
    if (!password || password_size == 0U)
        return DB_EVENT_DESTINATION_INVALID;
    password[0] = '\0';
    return DB_EVENT_DESTINATION_OK;
}

int __wrap_event_identity_get_source(char *output, size_t output_size) {
    int written = snprintf(output, output_size, "urn:lightnvr:%s",
                           INSTALLATION_UUID);
    return written > 0 && (size_t)written < output_size ? 0 : -1;
}

bool __wrap_system_health_evaluator_service_copy_run(
    system_health_process_run_t *run) {
    memset(run, 0, sizeof(*run));
    snprintf(run->run_id, sizeof(run->run_id), "%s", RUN_UUID);
    snprintf(run->boot_id, sizeof(run->boot_id), "boot-test");
    return true;
}

bool __wrap_system_health_snapshot_copy(system_health_snapshot_t *snapshot) {
    memset(snapshot, 0, sizeof(*snapshot));
    snapshot->sequence = 1U;
    snapshot->observation_count = 1U;
    return true;
}

size_t __wrap_system_health_evaluator_service_active_copy(
    system_health_incident_view_t *incidents, size_t capacity) {
    if (incidents && capacity > 0U) {
        memset(incidents, 0, sizeof(*incidents));
        incidents[0].severity = SYSTEM_HEALTH_SEVERITY_WARNING;
        return 1U;
    }
    return 0U;
}

void __wrap__log_message_ctx(log_level_t level, const char *component,
                             const char *stream, const char *format, ...) {
    (void)level;
    (void)component;
    (void)stream;
    (void)format;
}

bool __wrap_lightnvr_uuid_is_valid(const char *uuid) {
    return uuid && strlen(uuid) == 36U;
}

void __wrap_secure_zero_memory(void *pointer, size_t size) {
    volatile unsigned char *bytes = pointer;
    while (bytes && size-- > 0U) *bytes++ = 0U;
}

struct mosquitto *__wrap_mosquitto_new(const char *id, bool clean_session,
                                       void *userdata) {
    (void)id;
    (void)clean_session;
    if (fake_client_count >= EVENT_DESTINATION_MAX_COUNT) return NULL;
    fake_mosquitto_t *client = &fake_clients[fake_client_count++];
    client->userdata = userdata;
    return (struct mosquitto *)client;
}

void __wrap_mosquitto_destroy(struct mosquitto *mosq) {
    (void)mosq;
}

void __wrap_mosquitto_connect_callback_set(
    struct mosquitto *mosq,
    void (*callback)(struct mosquitto *, void *, int)) {
    fake(mosq)->connect_callback = callback;
}

void __wrap_mosquitto_disconnect_callback_set(
    struct mosquitto *mosq,
    void (*callback)(struct mosquitto *, void *, int)) {
    fake(mosq)->disconnect_callback = callback;
}

void __wrap_mosquitto_publish_callback_set(
    struct mosquitto *mosq,
    void (*callback)(struct mosquitto *, void *, int)) {
    fake(mosq)->publish_callback = callback;
}

void __wrap_mosquitto_log_callback_set(
    struct mosquitto *mosq,
    void (*callback)(struct mosquitto *, void *, int, const char *)) {
    (void)mosq;
    (void)callback;
}

int __wrap_mosquitto_username_pw_set(struct mosquitto *mosq,
                                     const char *username,
                                     const char *password) {
    (void)mosq;
    (void)username;
    (void)password;
    return MOSQ_ERR_SUCCESS;
}

int __wrap_mosquitto_int_option(struct mosquitto *mosq,
                                enum mosq_opt_t option, int value) {
    (void)mosq;
    (void)option;
    (void)value;
    return MOSQ_ERR_SUCCESS;
}

int __wrap_mosquitto_tls_set(struct mosquitto *mosq, const char *cafile,
                             const char *capath, const char *certfile,
                             const char *keyfile,
                             int (*pw_callback)(char *, int, int, void *)) {
    (void)mosq;
    (void)cafile;
    (void)capath;
    (void)certfile;
    (void)keyfile;
    (void)pw_callback;
    return MOSQ_ERR_SUCCESS;
}

int __wrap_mosquitto_tls_opts_set(struct mosquitto *mosq,
                                  int cert_reqs, const char *tls_version,
                                  const char *ciphers) {
    (void)mosq;
    (void)cert_reqs;
    (void)tls_version;
    (void)ciphers;
    return MOSQ_ERR_SUCCESS;
}

int __wrap_mosquitto_reconnect_delay_set(struct mosquitto *mosq,
                                         unsigned int delay,
                                         unsigned int delay_max,
                                         bool exponential_backoff) {
    (void)mosq;
    (void)delay;
    (void)delay_max;
    (void)exponential_backoff;
    return MOSQ_ERR_SUCCESS;
}

int __wrap_mosquitto_will_set(struct mosquitto *mosq, const char *topic,
                              int payloadlen, const void *payload, int qos,
                              bool retain) {
    (void)mosq;
    (void)qos;
    (void)retain;
    will_sequence = ++call_sequence;
    will_calls++;
    snprintf(last_will_topic, sizeof(last_will_topic), "%s", topic);
    snprintf(last_will_payload, sizeof(last_will_payload), "%.*s",
             payloadlen, (const char *)payload);
    return MOSQ_ERR_SUCCESS;
}

int __wrap_mosquitto_connect_async(struct mosquitto *mosq,
                                   const char *host, int port,
                                   int keepalive) {
    (void)mosq;
    (void)host;
    (void)port;
    (void)keepalive;
    connect_sequence = ++call_sequence;
    return MOSQ_ERR_SUCCESS;
}

int __wrap_mosquitto_loop_start(struct mosquitto *mosq) {
    (void)mosq;
    return MOSQ_ERR_SUCCESS;
}

int __wrap_mosquitto_disconnect(struct mosquitto *mosq) {
    (void)mosq;
    return MOSQ_ERR_SUCCESS;
}

int __wrap_mosquitto_loop_stop(struct mosquitto *mosq, bool force) {
    (void)mosq;
    (void)force;
    return MOSQ_ERR_SUCCESS;
}

int __wrap_mosquitto_publish(struct mosquitto *mosq, int *mid,
                             const char *topic, int payloadlen,
                             const void *payload, int qos, bool retain) {
    (void)mosq;
    (void)qos;
    if (mid) *mid = 7;
    publish_calls++;
    snprintf(last_publish_topic, sizeof(last_publish_topic), "%s", topic);
    snprintf(last_publish_payload, sizeof(last_publish_payload), "%.*s",
             payloadlen, (const char *)payload);
    last_publish_retain = retain;
    return MOSQ_ERR_SUCCESS;
}

const char *__wrap_mosquitto_connack_string(int connack_code) {
    (void)connack_code;
    return "mock connack";
}

const char *__wrap_mosquitto_strerror(int mosq_errno) {
    (void)mosq_errno;
    return "mock error";
}

static void connect_fake(size_t index, int result) {
    TEST_ASSERT_NOT_NULL(fake_clients[index].connect_callback);
    fake_clients[index].connect_callback(
        (struct mosquitto *)&fake_clients[index],
        fake_clients[index].userdata, result);
}

static void disconnect_fake(size_t index, int result) {
    TEST_ASSERT_NOT_NULL(fake_clients[index].disconnect_callback);
    fake_clients[index].disconnect_callback(
        (struct mosquitto *)&fake_clients[index],
        fake_clients[index].userdata, result);
}

static void test_empty_status_topic_disables_presence(void) {
    event_destination_t value = profile(DESTINATION_UUID_A, "empty", "");
    mqtt_destination_client_t *client =
        mqtt_destination_client_create(&value);
    TEST_ASSERT_NOT_NULL(client);
    TEST_ASSERT_EQUAL_INT(0, will_calls);
    connect_fake(0U, 0);
    TEST_ASSERT_EQUAL_INT(0, publish_calls);

    mqtt_destination_client_stats_t stats;
    TEST_ASSERT_TRUE(mqtt_destination_client_get_stats(client, &stats));
    TEST_ASSERT_FALSE(stats.presence_configured);
    TEST_ASSERT_TRUE(stats.connected);
    TEST_ASSERT_EQUAL_UINT64(1U, stats.connections);

    disconnect_fake(0U, 1);
    TEST_ASSERT_TRUE(mqtt_destination_client_get_stats(client, &stats));
    TEST_ASSERT_FALSE(stats.connected);
    TEST_ASSERT_EQUAL_UINT64(1U, stats.disconnects);
    TEST_ASSERT_EQUAL_UINT64(1U, stats.connection_failures);
    TEST_ASSERT_EQUAL(MQTT_DESTINATION_FAILURE_CONNECTION,
                      stats.last_failure);
    mqtt_destination_client_destroy(client);
}

static void test_will_online_reconnect_and_stopping_use_explicit_topic(void) {
    event_destination_t value = profile(
        DESTINATION_UUID_A, "presence",
        "fleet/status/{installation_uuid}/{destination_uuid}");
    snprintf(value.username, sizeof(value.username), "secret-user");
    mqtt_destination_client_t *client =
        mqtt_destination_client_create(&value);
    TEST_ASSERT_NOT_NULL(client);
    TEST_ASSERT_EQUAL_INT(1, will_calls);
    TEST_ASSERT_TRUE(will_sequence < connect_sequence);
    TEST_ASSERT_EQUAL_STRING(
        "fleet/status/" INSTALLATION_UUID "/" DESTINATION_UUID_A,
        last_will_topic);
    TEST_ASSERT_NOT_NULL(strstr(last_will_payload,
                                "\"state\":\"offline\""));
    TEST_ASSERT_NOT_NULL(strstr(last_will_payload,
                                "\"sequence\":0"));
    TEST_ASSERT_NULL(strstr(last_will_payload, "secret-user"));
    TEST_ASSERT_NULL(strstr(last_will_payload, "broker.invalid"));

    connect_fake(0U, 0);
    TEST_ASSERT_EQUAL_INT(1, publish_calls);
    TEST_ASSERT_EQUAL_STRING(last_will_topic, last_publish_topic);
    TEST_ASSERT_TRUE(last_publish_retain);
    TEST_ASSERT_NOT_NULL(strstr(last_publish_payload,
                                "\"state\":\"online\""));
    TEST_ASSERT_NOT_NULL(strstr(last_publish_payload,
                                "\"sequence\":1"));
    TEST_ASSERT_NOT_NULL(strstr(last_publish_payload,
                                "\"overall_state\":\"warning\""));
    TEST_ASSERT_NOT_NULL(strstr(last_publish_payload,
                                "\"active_incidents\":1"));

    disconnect_fake(0U, 1);
    connect_fake(0U, 0);
    mqtt_destination_client_stats_t stats;
    TEST_ASSERT_TRUE(mqtt_destination_client_get_stats(client, &stats));
    TEST_ASSERT_TRUE(stats.presence_configured);
    TEST_ASSERT_EQUAL_UINT64(2U, stats.connections);
    TEST_ASSERT_EQUAL_UINT64(1U, stats.reconnects);
    TEST_ASSERT_EQUAL_UINT64(2U, stats.publish_successes);
    TEST_ASSERT_TRUE(stats.last_success_at_ms > 0);

    int before_stopping = publish_calls;
    mqtt_destination_client_destroy(client);
    TEST_ASSERT_EQUAL_INT(before_stopping + 1, publish_calls);
    TEST_ASSERT_NOT_NULL(strstr(last_publish_payload,
                                "\"state\":\"stopping\""));
    mqtt_destination_client_stats_t listed[EVENT_DESTINATION_MAX_COUNT];
    TEST_ASSERT_EQUAL_UINT64(
        0U, mqtt_destination_client_list_stats(
                listed, EVENT_DESTINATION_MAX_COUNT));
}

static void test_heartbeat_runs_on_the_client_worker(void) {
    event_destination_t value = profile(
        DESTINATION_UUID_A, "heartbeat", "status/{installation_uuid}");
    mqtt_destination_client_t *client =
        mqtt_destination_client_create(&value);
    TEST_ASSERT_NOT_NULL(client);
    connect_fake(0U, 0);
    TEST_ASSERT_EQUAL_INT(1, publish_calls);
    struct timespec delay = {.tv_sec = 1, .tv_nsec = 300000000L};
    nanosleep(&delay, NULL);
    TEST_ASSERT_TRUE(publish_calls >= 2);
    TEST_ASSERT_NOT_NULL(strstr(last_publish_payload,
                                "\"state\":\"online\""));
    mqtt_destination_client_destroy(client);
}

static void test_destinations_keep_independent_bounded_stats(void) {
    event_destination_t second = profile(
        DESTINATION_UUID_B, "second", "status/{installation_uuid}");
    event_destination_t first = profile(
        DESTINATION_UUID_A, "first", "status/{installation_uuid}");
    mqtt_destination_client_t *second_client =
        mqtt_destination_client_create(&second);
    mqtt_destination_client_t *first_client =
        mqtt_destination_client_create(&first);
    TEST_ASSERT_NOT_NULL(second_client);
    TEST_ASSERT_NOT_NULL(first_client);
    connect_fake(0U, 2);
    connect_fake(1U, 0);

    mqtt_destination_client_stats_t listed[2];
    TEST_ASSERT_EQUAL_UINT64(
        2U, mqtt_destination_client_list_stats(listed, 2U));
    TEST_ASSERT_EQUAL_STRING(DESTINATION_UUID_A, listed[0].destination_uuid);
    TEST_ASSERT_TRUE(listed[0].connected);
    TEST_ASSERT_EQUAL_STRING(DESTINATION_UUID_B, listed[1].destination_uuid);
    TEST_ASSERT_FALSE(listed[1].connected);
    TEST_ASSERT_EQUAL(MQTT_DESTINATION_FAILURE_CONNECTION,
                      listed[1].last_failure);

    mqtt_destination_client_destroy(first_client);
    mqtt_destination_client_destroy(second_client);
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_empty_status_topic_disables_presence);
    RUN_TEST(test_will_online_reconnect_and_stopping_use_explicit_topic);
    RUN_TEST(test_heartbeat_runs_on_the_client_worker);
    RUN_TEST(test_destinations_keep_independent_bounded_stats);
    return UNITY_END();
}
