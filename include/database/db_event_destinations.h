#ifndef LIGHTNVR_DB_EVENT_DESTINATIONS_H
#define LIGHTNVR_DB_EVENT_DESTINATIONS_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#define EVENT_DESTINATION_UUID_MAX 37
#define EVENT_DESTINATION_KEY_MAX 64
#define EVENT_DESTINATION_NAME_MAX 128
#define EVENT_DESTINATION_DESCRIPTION_MAX 512
#define EVENT_DESTINATION_TYPE_MAX 16
#define EVENT_DESTINATION_HOST_MAX 256
#define EVENT_DESTINATION_CLIENT_ID_MAX 128
#define EVENT_DESTINATION_TOPIC_TEMPLATE_MAX 512
#define EVENT_DESTINATION_STATUS_TOPIC_TEMPLATE_MAX 512
#define EVENT_DESTINATION_USERNAME_MAX 128
#define EVENT_DESTINATION_PASSWORD_MAX 256
#define EVENT_DESTINATION_TLS_MODE_MAX 16
#define EVENT_DESTINATION_PATH_MAX 512
#define EVENT_DESTINATION_VALIDATION_ERROR_MAX 256
#define EVENT_DESTINATION_MAX_COUNT 64
#define EVENT_DESTINATION_KEY_PREFIX "mqtt:"
#define EVENT_DESTINATION_DEFAULT_TOPIC_TEMPLATE \
    "lightnvr/v1/events/{type}/{subject_id}"
#define EVENT_DESTINATION_STATUS_INSTALLATION_PLACEHOLDER \
    "{installation_uuid}"
#define EVENT_DESTINATION_STATUS_DESTINATION_PLACEHOLDER \
    "{destination_uuid}"

typedef struct {
    char uuid[EVENT_DESTINATION_UUID_MAX];
    char name[EVENT_DESTINATION_NAME_MAX];
    char description[EVENT_DESTINATION_DESCRIPTION_MAX];
    bool enabled;
    char destination_type[EVENT_DESTINATION_TYPE_MAX];
    char broker_host[EVENT_DESTINATION_HOST_MAX];
    int broker_port;
    char client_id[EVENT_DESTINATION_CLIENT_ID_MAX];
    char topic_template[EVENT_DESTINATION_TOPIC_TEMPLATE_MAX];
    char status_topic_template[EVENT_DESTINATION_STATUS_TOPIC_TEMPLATE_MAX];
    char username[EVENT_DESTINATION_USERNAME_MAX];
    bool password_configured;
    char tls_mode[EVENT_DESTINATION_TLS_MODE_MAX];
    char ca_file[EVENT_DESTINATION_PATH_MAX];
    char cert_file[EVENT_DESTINATION_PATH_MAX];
    char key_file[EVENT_DESTINATION_PATH_MAX];
    int keepalive_seconds;
    int qos;
    int64_t revision;
    int64_t created_at;
    int64_t updated_at;
} event_destination_t;

typedef enum {
    DB_EVENT_DESTINATION_OK = 0,
    DB_EVENT_DESTINATION_NOT_FOUND = -1,
    DB_EVENT_DESTINATION_CONFLICT = -2,
    DB_EVENT_DESTINATION_INVALID = -3,
    DB_EVENT_DESTINATION_STALE = -4,
    DB_EVENT_DESTINATION_LIMIT = -5,
    DB_EVENT_DESTINATION_IN_USE = -6,
    DB_EVENT_DESTINATION_ERROR = -7
} db_event_destination_result_t;

db_event_destination_result_t db_event_destination_validate(
    const event_destination_t *destination, const char *password,
    bool validate_password, char *error, size_t error_size);

int db_event_destination_count(void);
int db_event_destination_list(event_destination_t *destinations,
                              int max_count);
db_event_destination_result_t db_event_destination_get(
    const char *uuid, event_destination_t *destination);
db_event_destination_result_t db_event_destination_get_by_key(
    const char *key, event_destination_t *destination);

db_event_destination_result_t db_event_destination_create(
    event_destination_t *destination, const char *password);
db_event_destination_result_t db_event_destination_update(
    event_destination_t *destination, int64_t expected_revision,
    const char *password, bool replace_password);
db_event_destination_result_t db_event_destination_delete(
    const char *uuid, int64_t expected_revision);

/* Runtime-only credential load. Callers must zero the returned buffer. */
db_event_destination_result_t db_event_destination_get_password(
    const char *uuid, int64_t expected_revision, char *password,
    size_t password_size);

bool db_event_destination_key_exists(const char *key);
int db_event_destination_make_key(
    const char *uuid, char key[EVENT_DESTINATION_KEY_MAX]);
uint64_t db_event_destination_generation(void);

#endif /* LIGHTNVR_DB_EVENT_DESTINATIONS_H */
