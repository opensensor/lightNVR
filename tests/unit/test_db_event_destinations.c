/**
 * @file test_db_event_destinations.c
 * @brief Managed MQTT destination persistence and validation tests.
 */

#define _POSIX_C_SOURCE 200809L

#include <sqlite3.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

#include "database/db_core.h"
#include "database/db_event_destinations.h"
#include "unity.h"
#include "utils/memory.h"
#include "utils/strings.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_event_destinations.db"

static event_destination_t valid_destination(const char *name) {
    event_destination_t destination;
    memset(&destination, 0, sizeof(destination));
    safe_strcpy(destination.name, name, sizeof(destination.name), 0);
    safe_strcpy(destination.description, "Operations event bridge",
                sizeof(destination.description), 0);
    destination.enabled = true;
    safe_strcpy(destination.destination_type, "mqtt",
                sizeof(destination.destination_type), 0);
    safe_strcpy(destination.broker_host, "mqtt.example.test",
                sizeof(destination.broker_host), 0);
    destination.broker_port = 8883;
    safe_strcpy(destination.client_id, "lightnvr-sjc",
                sizeof(destination.client_id), 0);
    safe_strcpy(destination.topic_template,
                EVENT_DESTINATION_DEFAULT_TOPIC_TEMPLATE,
                sizeof(destination.topic_template), 0);
    safe_strcpy(destination.username, "event-publisher",
                sizeof(destination.username), 0);
    safe_strcpy(destination.tls_mode, "system",
                sizeof(destination.tls_mode), 0);
    destination.keepalive_seconds = 60;
    destination.qos = 1;
    return destination;
}

void setUp(void) {
    TEST_ASSERT_EQUAL_INT(
        SQLITE_OK,
        sqlite3_exec(get_db_handle(), "DELETE FROM event_routes;", NULL,
                     NULL, NULL));
    TEST_ASSERT_EQUAL_INT(
        SQLITE_OK,
        sqlite3_exec(get_db_handle(), "DELETE FROM event_destinations;", NULL,
                     NULL, NULL));
}

void tearDown(void) {}

void test_destination_crud_redacts_password_and_uses_revision(void) {
    event_destination_t destination = valid_destination("  SJC bridge  ");
    TEST_ASSERT_EQUAL_INT(
        DB_EVENT_DESTINATION_OK,
        db_event_destination_create(&destination, "top-secret-value"));
    TEST_ASSERT_EQUAL_UINT(36, strlen(destination.uuid));
    TEST_ASSERT_EQUAL_STRING("SJC bridge", destination.name);
    TEST_ASSERT_TRUE(destination.password_configured);
    TEST_ASSERT_EQUAL_INT64(1, destination.revision);
    TEST_ASSERT_EQUAL_INT(1, db_event_destination_count());

    char key[EVENT_DESTINATION_KEY_MAX];
    TEST_ASSERT_EQUAL_INT(
        0, db_event_destination_make_key(destination.uuid, key));
    TEST_ASSERT_TRUE(db_event_destination_key_exists(key));

    event_destination_t loaded;
    TEST_ASSERT_EQUAL_INT(DB_EVENT_DESTINATION_OK,
                          db_event_destination_get_by_key(key, &loaded));
    TEST_ASSERT_EQUAL_STRING(destination.uuid, loaded.uuid);
    TEST_ASSERT_TRUE(loaded.password_configured);

    char password[EVENT_DESTINATION_PASSWORD_MAX] = {0};
    TEST_ASSERT_EQUAL_INT(
        DB_EVENT_DESTINATION_OK,
        db_event_destination_get_password(loaded.uuid, loaded.revision,
                                          password, sizeof(password)));
    TEST_ASSERT_EQUAL_STRING("top-secret-value", password);
    secure_zero_memory(password, sizeof(password));

    safe_strcpy(loaded.description, "Updated bridge",
                sizeof(loaded.description), 0);
    TEST_ASSERT_EQUAL_INT(
        DB_EVENT_DESTINATION_OK,
        db_event_destination_update(&loaded, 1, NULL, false));
    TEST_ASSERT_EQUAL_INT64(2, loaded.revision);
    TEST_ASSERT_TRUE(loaded.password_configured);
    TEST_ASSERT_EQUAL_INT(
        DB_EVENT_DESTINATION_STALE,
        db_event_destination_get_password(loaded.uuid, 1, password,
                                          sizeof(password)));
    TEST_ASSERT_EQUAL_INT(
        DB_EVENT_DESTINATION_OK,
        db_event_destination_get_password(loaded.uuid, 2, password,
                                          sizeof(password)));
    TEST_ASSERT_EQUAL_STRING("top-secret-value", password);
    secure_zero_memory(password, sizeof(password));

    TEST_ASSERT_EQUAL_INT(
        DB_EVENT_DESTINATION_OK,
        db_event_destination_update(&loaded, 2, NULL, true));
    TEST_ASSERT_EQUAL_INT64(3, loaded.revision);
    TEST_ASSERT_FALSE(loaded.password_configured);
    TEST_ASSERT_EQUAL_INT(
        DB_EVENT_DESTINATION_OK,
        db_event_destination_get_password(loaded.uuid, 3, password,
                                          sizeof(password)));
    TEST_ASSERT_EQUAL_STRING("", password);

    TEST_ASSERT_EQUAL_INT(
        DB_EVENT_DESTINATION_STALE,
        db_event_destination_delete(loaded.uuid, 1));
    TEST_ASSERT_EQUAL_INT(
        DB_EVENT_DESTINATION_OK,
        db_event_destination_delete(loaded.uuid, 3));
    TEST_ASSERT_FALSE(db_event_destination_key_exists(key));
}

void test_destination_name_is_unique_case_insensitively(void) {
    event_destination_t first = valid_destination("Operations");
    event_destination_t second = valid_destination("operations");
    TEST_ASSERT_EQUAL_INT(DB_EVENT_DESTINATION_OK,
                          db_event_destination_create(&first, "secret"));
    TEST_ASSERT_EQUAL_INT(DB_EVENT_DESTINATION_CONFLICT,
                          db_event_destination_create(&second, "secret"));

    second = valid_destination("Different name");
    TEST_ASSERT_EQUAL_INT(DB_EVENT_DESTINATION_CONFLICT,
                          db_event_destination_create(&second, "secret"));
}

void test_destination_validates_topics_credentials_and_tls(void) {
    char error[EVENT_DESTINATION_VALIDATION_ERROR_MAX];
    event_destination_t destination = valid_destination("Validation");
    TEST_ASSERT_EQUAL_INT(
        DB_EVENT_DESTINATION_OK,
        db_event_destination_validate(&destination, "secret", true, error,
                                      sizeof(error)));

    safe_strcpy(destination.topic_template, "events/{type}/missing-subject",
                sizeof(destination.topic_template), 0);
    TEST_ASSERT_EQUAL_INT(
        DB_EVENT_DESTINATION_INVALID,
        db_event_destination_validate(&destination, "secret", true, error,
                                      sizeof(error)));
    TEST_ASSERT_NOT_NULL(strstr(error, "topic template"));

    destination = valid_destination("Validation");
    safe_strcpy(destination.topic_template, "events/+/{type}/{subject_id}",
                sizeof(destination.topic_template), 0);
    TEST_ASSERT_EQUAL_INT(
        DB_EVENT_DESTINATION_INVALID,
        db_event_destination_validate(&destination, "secret", true, error,
                                      sizeof(error)));

    destination = valid_destination("Validation");
    destination.username[0] = '\0';
    TEST_ASSERT_EQUAL_INT(
        DB_EVENT_DESTINATION_INVALID,
        db_event_destination_validate(&destination, "secret", true, error,
                                      sizeof(error)));
    TEST_ASSERT_NOT_NULL(strstr(error, "username"));

    destination = valid_destination("Validation");
    safe_strcpy(destination.tls_mode, "custom_ca",
                sizeof(destination.tls_mode), 0);
    TEST_ASSERT_EQUAL_INT(
        DB_EVENT_DESTINATION_INVALID,
        db_event_destination_validate(&destination, "secret", true, error,
                                      sizeof(error)));
    safe_strcpy(destination.ca_file, "/etc/lightnvr/certs/bridge-ca.pem",
                sizeof(destination.ca_file), 0);
    TEST_ASSERT_EQUAL_INT(
        DB_EVENT_DESTINATION_OK,
        db_event_destination_validate(&destination, "secret", true, error,
                                      sizeof(error)));

    safe_strcpy(destination.broker_host, "mqtts://mqtt.example.test",
                sizeof(destination.broker_host), 0);
    TEST_ASSERT_EQUAL_INT(
        DB_EVENT_DESTINATION_INVALID,
        db_event_destination_validate(&destination, "secret", true, error,
                                      sizeof(error)));
}

void test_destination_delete_is_blocked_when_a_route_references_it(void) {
    event_destination_t destination = valid_destination("In use");
    TEST_ASSERT_EQUAL_INT(
        DB_EVENT_DESTINATION_OK,
        db_event_destination_create(&destination, "secret"));
    char key[EVENT_DESTINATION_KEY_MAX];
    TEST_ASSERT_EQUAL_INT(
        0, db_event_destination_make_key(destination.uuid, key));

    sqlite3_stmt *statement = NULL;
    const char *sql =
        "INSERT INTO event_routes("
        "uuid,name,description,enabled,destination_key,scope_type,"
        "selector_json,predicate_json,schedule_json,debounce_seconds,"
        "cooldown_seconds,grouping_window_seconds,max_events_per_minute) "
        "VALUES('11111111-1111-4111-8111-111111111111','Uses profile','',1,"
        "?,'all',NULL,'{\"version\":1}',"
        "'{\"version\":1,\"timezone\":\"UTC\",\"windows\":[]}',"
        "0,0,0,0);";
    TEST_ASSERT_EQUAL_INT(
        SQLITE_OK, sqlite3_prepare_v2(get_db_handle(), sql, -1, &statement,
                                     NULL));
    sqlite3_bind_text(statement, 1, key, -1, SQLITE_TRANSIENT);
    TEST_ASSERT_EQUAL_INT(SQLITE_DONE, sqlite3_step(statement));
    sqlite3_finalize(statement);

    TEST_ASSERT_EQUAL_INT(
        DB_EVENT_DESTINATION_IN_USE,
        db_event_destination_delete(destination.uuid, destination.revision));
}

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_destination_crud_redacts_password_and_uses_revision);
    RUN_TEST(test_destination_name_is_unique_case_insensitively);
    RUN_TEST(test_destination_validates_topics_credentials_and_tls);
    RUN_TEST(test_destination_delete_is_blocked_when_a_route_references_it);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}
