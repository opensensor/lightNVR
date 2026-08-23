#define _POSIX_C_SOURCE 200809L

#include "database/db_event_destinations.h"

#include <ctype.h>
#include <pthread.h>
#include <sqlite3.h>
#include <stdarg.h>
#include <stdatomic.h>
#include <stdbool.h>
#include <stdio.h>
#include <string.h>
#include <strings.h>

#include "database/db_core.h"
#include "utils/memory.h"
#include "utils/strings.h"
#include "utils/uuid.h"

#define EVENT_DESTINATION_SELECT_FIELDS \
    "uuid,name,description,enabled,destination_type,broker_host," \
    "broker_port,client_id,topic_template,username,(password<>'')," \
    "tls_mode,ca_file,cert_file,key_file,keepalive_seconds,qos," \
    "revision,created_at,updated_at"

static atomic_uint_fast64_t DESTINATION_GENERATION = 1;

static void set_error(char *error, size_t error_size, const char *format, ...) {
    if (!error || error_size == 0 || error[0] != '\0') return;
    va_list arguments;
    va_start(arguments, format);
    vsnprintf(error, error_size, format, arguments);
    va_end(arguments);
}

static bool valid_text(const char *value, size_t maximum, bool required) {
    if (!value) return false;
    size_t length = strnlen(value, maximum);
    if (length == maximum || (required && length == 0)) return false;
    for (const unsigned char *cursor = (const unsigned char *)value;
         *cursor; cursor++) {
        if (iscntrl(*cursor)) return false;
    }
    return true;
}

static bool valid_host(const char *host) {
    if (!valid_text(host, EVENT_DESTINATION_HOST_MAX, true) ||
        strstr(host, "://")) {
        return false;
    }
    for (const unsigned char *cursor = (const unsigned char *)host;
         *cursor; cursor++) {
        if (isspace(*cursor) || *cursor == '/' || *cursor == '\\' ||
            *cursor == '?' || *cursor == '#') {
            return false;
        }
    }
    return true;
}

static bool valid_path(const char *path, bool required) {
    if (!valid_text(path, EVENT_DESTINATION_PATH_MAX, required)) return false;
    return path[0] == '\0' || path[0] == '/';
}

static bool valid_topic_template(const char *topic) {
    if (!valid_text(topic, EVENT_DESTINATION_TOPIC_TEMPLATE_MAX, true)) {
        return false;
    }
    bool has_type = false;
    bool has_subject_id = false;
    for (size_t index = 0; topic[index] != '\0';) {
        if (topic[index] == '+' || topic[index] == '#') return false;
        if (topic[index] == '}') return false;
        if (topic[index] != '{') {
            index++;
            continue;
        }
        if (strncmp(topic + index, "{type}", 6) == 0) {
            has_type = true;
            index += 6;
        } else if (strncmp(topic + index, "{subject_id}", 12) == 0) {
            has_subject_id = true;
            index += 12;
        } else {
            return false;
        }
    }
    return has_type && has_subject_id && topic[0] != '/' &&
        topic[strlen(topic) - 1] != '/';
}

static bool valid_password(const char *password) {
    return valid_text(password, EVENT_DESTINATION_PASSWORD_MAX, false);
}

db_event_destination_result_t db_event_destination_validate(
    const event_destination_t *destination, const char *password,
    bool validate_password, char *error, size_t error_size) {
    if (error && error_size > 0) error[0] = '\0';
    if (!destination ||
        !valid_text(destination->name, sizeof(destination->name), true) ||
        !valid_text(destination->description,
                    sizeof(destination->description), false)) {
        set_error(error, error_size,
                  "destination name or description is invalid");
        return DB_EVENT_DESTINATION_INVALID;
    }
    char normalized_name[EVENT_DESTINATION_NAME_MAX];
    if (copy_trimmed_value(normalized_name, sizeof(normalized_name),
                           destination->name, 0) == 0) {
        set_error(error, error_size, "destination name cannot be blank");
        return DB_EVENT_DESTINATION_INVALID;
    }
    if (!valid_text(destination->destination_type,
                    sizeof(destination->destination_type), true) ||
        strcmp(destination->destination_type, "mqtt") != 0) {
        set_error(error, error_size,
                  "destination type must be mqtt");
        return DB_EVENT_DESTINATION_INVALID;
    }
    if (!valid_host(destination->broker_host) ||
        destination->broker_port < 1 || destination->broker_port > 65535) {
        set_error(error, error_size, "broker host or port is invalid");
        return DB_EVENT_DESTINATION_INVALID;
    }
    if (!valid_text(destination->client_id,
                    sizeof(destination->client_id), true) ||
        !valid_topic_template(destination->topic_template)) {
        set_error(error, error_size,
                  "client ID or topic template is invalid");
        return DB_EVENT_DESTINATION_INVALID;
    }
    if (!valid_text(destination->username,
                    sizeof(destination->username), false) ||
        (validate_password && !valid_password(password))) {
        set_error(error, error_size, "MQTT credentials are invalid");
        return DB_EVENT_DESTINATION_INVALID;
    }
    bool password_configured = validate_password
        ? password && password[0] != '\0'
        : destination->password_configured;
    if (password_configured && destination->username[0] == '\0') {
        set_error(error, error_size,
                  "a password requires an MQTT username");
        return DB_EVENT_DESTINATION_INVALID;
    }
    if (!valid_text(destination->tls_mode,
                    sizeof(destination->tls_mode), true)) {
        set_error(error, error_size, "TLS mode is invalid");
        return DB_EVENT_DESTINATION_INVALID;
    }
    bool disabled = strcmp(destination->tls_mode, "disabled") == 0;
    bool system = strcmp(destination->tls_mode, "system") == 0;
    bool custom_ca = strcmp(destination->tls_mode, "custom_ca") == 0;
    bool mutual = strcmp(destination->tls_mode, "mutual") == 0;
    if ((!disabled && !system && !custom_ca && !mutual) ||
        !valid_path(destination->ca_file, custom_ca) ||
        !valid_path(destination->cert_file, mutual) ||
        !valid_path(destination->key_file, mutual) ||
        ((disabled || system) &&
         (destination->ca_file[0] || destination->cert_file[0] ||
          destination->key_file[0])) ||
        (custom_ca &&
         (destination->cert_file[0] || destination->key_file[0]))) {
        set_error(error, error_size,
                  "TLS mode and certificate paths are inconsistent");
        return DB_EVENT_DESTINATION_INVALID;
    }
    if (destination->keepalive_seconds < 5 ||
        destination->keepalive_seconds > 3600 ||
        destination->qos < 0 || destination->qos > 2) {
        set_error(error, error_size,
                  "keepalive or QoS is out of range");
        return DB_EVENT_DESTINATION_INVALID;
    }
    return DB_EVENT_DESTINATION_OK;
}

static void copy_column(char *destination, size_t destination_size,
                        sqlite3_stmt *statement, int column) {
    const char *value = (const char *)sqlite3_column_text(statement, column);
    safe_strcpy(destination, value ? value : "", destination_size, 0);
}

static void populate(sqlite3_stmt *statement,
                     event_destination_t *destination) {
    memset(destination, 0, sizeof(*destination));
    copy_column(destination->uuid, sizeof(destination->uuid), statement, 0);
    copy_column(destination->name, sizeof(destination->name), statement, 1);
    copy_column(destination->description, sizeof(destination->description),
                statement, 2);
    destination->enabled = sqlite3_column_int(statement, 3) != 0;
    copy_column(destination->destination_type,
                sizeof(destination->destination_type), statement, 4);
    copy_column(destination->broker_host, sizeof(destination->broker_host),
                statement, 5);
    destination->broker_port = sqlite3_column_int(statement, 6);
    copy_column(destination->client_id, sizeof(destination->client_id),
                statement, 7);
    copy_column(destination->topic_template,
                sizeof(destination->topic_template), statement, 8);
    copy_column(destination->username, sizeof(destination->username),
                statement, 9);
    destination->password_configured =
        sqlite3_column_int(statement, 10) != 0;
    copy_column(destination->tls_mode, sizeof(destination->tls_mode),
                statement, 11);
    copy_column(destination->ca_file, sizeof(destination->ca_file),
                statement, 12);
    copy_column(destination->cert_file, sizeof(destination->cert_file),
                statement, 13);
    copy_column(destination->key_file, sizeof(destination->key_file),
                statement, 14);
    destination->keepalive_seconds = sqlite3_column_int(statement, 15);
    destination->qos = sqlite3_column_int(statement, 16);
    destination->revision = sqlite3_column_int64(statement, 17);
    destination->created_at = sqlite3_column_int64(statement, 18);
    destination->updated_at = sqlite3_column_int64(statement, 19);
}

static db_event_destination_result_t get_locked(
    sqlite3 *db, const char *uuid, event_destination_t *destination) {
    const char *sql = "SELECT " EVENT_DESTINATION_SELECT_FIELDS
        " FROM event_destinations WHERE uuid=? LIMIT 1;";
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, uuid, -1, SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    db_event_destination_result_t outcome = DB_EVENT_DESTINATION_NOT_FOUND;
    if (result == SQLITE_ROW) {
        populate(statement, destination);
        outcome = DB_EVENT_DESTINATION_OK;
    } else if (result != SQLITE_DONE) {
        outcome = DB_EVENT_DESTINATION_ERROR;
    }
    if (statement) sqlite3_finalize(statement);
    return outcome;
}

static int count_locked(sqlite3 *db) {
    sqlite3_stmt *statement = NULL;
    int count = -1;
    if (sqlite3_prepare_v2(
            db, "SELECT count(*) FROM event_destinations;", -1,
            &statement, NULL) == SQLITE_OK &&
        sqlite3_step(statement) == SQLITE_ROW) {
        count = sqlite3_column_int(statement, 0);
    }
    if (statement) sqlite3_finalize(statement);
    return count;
}

int db_event_destination_count(void) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;
    pthread_mutex_lock(mutex);
    int count = count_locked(db);
    pthread_mutex_unlock(mutex);
    return count;
}

int db_event_destination_list(event_destination_t *destinations,
                              int max_count) {
    if (!destinations || max_count <= 0) return -1;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;
    const char *sql = "SELECT " EVENT_DESTINATION_SELECT_FIELDS
        " FROM event_destinations ORDER BY name COLLATE NOCASE,uuid;";
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    int count = 0;
    while (result == SQLITE_OK && count < max_count &&
           (result = sqlite3_step(statement)) == SQLITE_ROW) {
        populate(statement, &destinations[count++]);
    }
    if (result != SQLITE_DONE && result != SQLITE_ROW) count = -1;
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    return count;
}

db_event_destination_result_t db_event_destination_get(
    const char *uuid, event_destination_t *destination) {
    if (!lightnvr_uuid_is_valid(uuid) || !destination) {
        return DB_EVENT_DESTINATION_INVALID;
    }
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return DB_EVENT_DESTINATION_ERROR;
    pthread_mutex_lock(mutex);
    db_event_destination_result_t result = get_locked(db, uuid, destination);
    pthread_mutex_unlock(mutex);
    return result;
}

int db_event_destination_make_key(
    const char *uuid, char key[EVENT_DESTINATION_KEY_MAX]) {
    if (!lightnvr_uuid_is_valid(uuid) || !key) return -1;
    int written = snprintf(key, EVENT_DESTINATION_KEY_MAX, "%s%s",
                           EVENT_DESTINATION_KEY_PREFIX, uuid);
    return written > 0 && written < EVENT_DESTINATION_KEY_MAX ? 0 : -1;
}

static const char *uuid_from_key(const char *key) {
    size_t prefix_length = strlen(EVENT_DESTINATION_KEY_PREFIX);
    if (!key || strncmp(key, EVENT_DESTINATION_KEY_PREFIX,
                        prefix_length) != 0) {
        return NULL;
    }
    const char *uuid = key + prefix_length;
    return lightnvr_uuid_is_valid(uuid) ? uuid : NULL;
}

db_event_destination_result_t db_event_destination_get_by_key(
    const char *key, event_destination_t *destination) {
    const char *uuid = uuid_from_key(key);
    return uuid ? db_event_destination_get(uuid, destination)
                : DB_EVENT_DESTINATION_INVALID;
}

bool db_event_destination_key_exists(const char *key) {
    event_destination_t destination;
    return db_event_destination_get_by_key(key, &destination) ==
        DB_EVENT_DESTINATION_OK;
}

static void bind_update_fields(sqlite3_stmt *statement,
                               const event_destination_t *destination,
                               const char *normalized_name) {
    sqlite3_bind_text(statement, 1, normalized_name, -1, SQLITE_TRANSIENT);
    sqlite3_bind_text(statement, 2, destination->description, -1,
                      SQLITE_TRANSIENT);
    sqlite3_bind_int(statement, 3, destination->enabled ? 1 : 0);
    sqlite3_bind_text(statement, 4, destination->destination_type, -1,
                      SQLITE_TRANSIENT);
    sqlite3_bind_text(statement, 5, destination->broker_host, -1,
                      SQLITE_TRANSIENT);
    sqlite3_bind_int(statement, 6, destination->broker_port);
    sqlite3_bind_text(statement, 7, destination->client_id, -1,
                      SQLITE_TRANSIENT);
    sqlite3_bind_text(statement, 8, destination->topic_template, -1,
                      SQLITE_TRANSIENT);
    sqlite3_bind_text(statement, 9, destination->username, -1,
                      SQLITE_TRANSIENT);
    sqlite3_bind_text(statement, 10, destination->tls_mode, -1,
                      SQLITE_TRANSIENT);
    sqlite3_bind_text(statement, 11, destination->ca_file, -1,
                      SQLITE_TRANSIENT);
    sqlite3_bind_text(statement, 12, destination->cert_file, -1,
                      SQLITE_TRANSIENT);
    sqlite3_bind_text(statement, 13, destination->key_file, -1,
                      SQLITE_TRANSIENT);
    sqlite3_bind_int(statement, 14, destination->keepalive_seconds);
    sqlite3_bind_int(statement, 15, destination->qos);
}

db_event_destination_result_t db_event_destination_create(
    event_destination_t *destination, const char *password) {
    if (!password) password = "";
    char error[EVENT_DESTINATION_VALIDATION_ERROR_MAX] = {0};
    if (!destination || db_event_destination_validate(
            destination, password, true, error, sizeof(error)) !=
            DB_EVENT_DESTINATION_OK) {
        return DB_EVENT_DESTINATION_INVALID;
    }
    if (lightnvr_uuid_generate_v4(destination->uuid) != 0) {
        return DB_EVENT_DESTINATION_ERROR;
    }
    char normalized_name[EVENT_DESTINATION_NAME_MAX];
    copy_trimmed_value(normalized_name, sizeof(normalized_name),
                       destination->name, 0);
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return DB_EVENT_DESTINATION_ERROR;
    const char *sql =
        "INSERT INTO event_destinations("
        "uuid,name,description,enabled,destination_type,broker_host,"
        "broker_port,client_id,topic_template,username,password,tls_mode,"
        "ca_file,cert_file,key_file,keepalive_seconds,qos) "
        "VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?);";
    pthread_mutex_lock(mutex);
    if (count_locked(db) >= EVENT_DESTINATION_MAX_COUNT) {
        pthread_mutex_unlock(mutex);
        return DB_EVENT_DESTINATION_LIMIT;
    }
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, destination->uuid, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 2, normalized_name, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 3, destination->description, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_int(statement, 4, destination->enabled ? 1 : 0);
        sqlite3_bind_text(statement, 5, destination->destination_type, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 6, destination->broker_host, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_int(statement, 7, destination->broker_port);
        sqlite3_bind_text(statement, 8, destination->client_id, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 9, destination->topic_template, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 10, destination->username, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 11, password, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 12, destination->tls_mode, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 13, destination->ca_file, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 14, destination->cert_file, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 15, destination->key_file, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_int(statement, 16, destination->keepalive_seconds);
        sqlite3_bind_int(statement, 17, destination->qos);
        result = sqlite3_step(statement);
    }
    if (statement) sqlite3_finalize(statement);
    if (result != SQLITE_DONE) {
        pthread_mutex_unlock(mutex);
        return (result & 0xff) == SQLITE_CONSTRAINT
            ? DB_EVENT_DESTINATION_CONFLICT : DB_EVENT_DESTINATION_ERROR;
    }
    atomic_fetch_add_explicit(&DESTINATION_GENERATION, 1,
                              memory_order_relaxed);
    db_event_destination_result_t outcome =
        get_locked(db, destination->uuid, destination);
    pthread_mutex_unlock(mutex);
    return outcome;
}

db_event_destination_result_t db_event_destination_update(
    event_destination_t *destination, int64_t expected_revision,
    const char *password, bool replace_password) {
    if (replace_password && !password) password = "";
    char error[EVENT_DESTINATION_VALIDATION_ERROR_MAX] = {0};
    if (!destination || !lightnvr_uuid_is_valid(destination->uuid) ||
        expected_revision < 1 || db_event_destination_validate(
            destination, password, replace_password, error, sizeof(error)) !=
            DB_EVENT_DESTINATION_OK) {
        return DB_EVENT_DESTINATION_INVALID;
    }
    char normalized_name[EVENT_DESTINATION_NAME_MAX];
    copy_trimmed_value(normalized_name, sizeof(normalized_name),
                       destination->name, 0);
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return DB_EVENT_DESTINATION_ERROR;
    const char *sql = replace_password
        ? "UPDATE event_destinations SET name=?,description=?,enabled=?,"
          "destination_type=?,broker_host=?,broker_port=?,client_id=?,"
          "topic_template=?,username=?,tls_mode=?,ca_file=?,cert_file=?,"
          "key_file=?,keepalive_seconds=?,qos=?,password=?,"
          "revision=revision+1,updated_at=strftime('%s','now') "
          "WHERE uuid=? AND revision=?;"
        : "UPDATE event_destinations SET name=?,description=?,enabled=?,"
          "destination_type=?,broker_host=?,broker_port=?,client_id=?,"
          "topic_template=?,username=?,tls_mode=?,ca_file=?,cert_file=?,"
          "key_file=?,keepalive_seconds=?,qos=?,"
          "revision=revision+1,updated_at=strftime('%s','now') "
          "WHERE uuid=? AND revision=?;";
    pthread_mutex_lock(mutex);
    event_destination_t existing;
    db_event_destination_result_t outcome =
        get_locked(db, destination->uuid, &existing);
    if (outcome != DB_EVENT_DESTINATION_OK) {
        pthread_mutex_unlock(mutex);
        return outcome;
    }
    if (existing.revision != expected_revision) {
        pthread_mutex_unlock(mutex);
        return DB_EVENT_DESTINATION_STALE;
    }
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        bind_update_fields(statement, destination, normalized_name);
        int index = 16;
        if (replace_password) {
            sqlite3_bind_text(statement, index++, password, -1,
                              SQLITE_TRANSIENT);
        }
        sqlite3_bind_text(statement, index++, destination->uuid, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_int64(statement, index, expected_revision);
        result = sqlite3_step(statement);
    }
    int changed = result == SQLITE_DONE ? sqlite3_changes(db) : 0;
    if (statement) sqlite3_finalize(statement);
    if (result != SQLITE_DONE || changed != 1) {
        pthread_mutex_unlock(mutex);
        return (result & 0xff) == SQLITE_CONSTRAINT
            ? DB_EVENT_DESTINATION_CONFLICT : DB_EVENT_DESTINATION_ERROR;
    }
    atomic_fetch_add_explicit(&DESTINATION_GENERATION, 1,
                              memory_order_relaxed);
    outcome = get_locked(db, destination->uuid, destination);
    pthread_mutex_unlock(mutex);
    return outcome;
}

static bool route_uses_key_locked(sqlite3 *db, const char *key) {
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(
        db, "SELECT 1 FROM event_routes WHERE destination_key=? LIMIT 1;",
        -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, key, -1, SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    bool in_use = result == SQLITE_ROW;
    if (statement) sqlite3_finalize(statement);
    return in_use;
}

db_event_destination_result_t db_event_destination_delete(
    const char *uuid, int64_t expected_revision) {
    if (!lightnvr_uuid_is_valid(uuid) || expected_revision < 1) {
        return DB_EVENT_DESTINATION_INVALID;
    }
    char key[EVENT_DESTINATION_KEY_MAX];
    if (db_event_destination_make_key(uuid, key) != 0) {
        return DB_EVENT_DESTINATION_INVALID;
    }
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return DB_EVENT_DESTINATION_ERROR;
    pthread_mutex_lock(mutex);
    event_destination_t existing;
    db_event_destination_result_t outcome = get_locked(db, uuid, &existing);
    if (outcome != DB_EVENT_DESTINATION_OK) {
        pthread_mutex_unlock(mutex);
        return outcome;
    }
    if (existing.revision != expected_revision) {
        pthread_mutex_unlock(mutex);
        return DB_EVENT_DESTINATION_STALE;
    }
    if (route_uses_key_locked(db, key)) {
        pthread_mutex_unlock(mutex);
        return DB_EVENT_DESTINATION_IN_USE;
    }
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(
        db, "DELETE FROM event_destinations WHERE uuid=? AND revision=?;",
        -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int64(statement, 2, expected_revision);
        result = sqlite3_step(statement);
    }
    int changed = result == SQLITE_DONE ? sqlite3_changes(db) : 0;
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    if (result != SQLITE_DONE || changed != 1) {
        return DB_EVENT_DESTINATION_ERROR;
    }
    atomic_fetch_add_explicit(&DESTINATION_GENERATION, 1,
                              memory_order_relaxed);
    return DB_EVENT_DESTINATION_OK;
}

db_event_destination_result_t db_event_destination_get_password(
    const char *uuid, int64_t expected_revision, char *password,
    size_t password_size) {
    if (password && password_size > 0) {
        secure_zero_memory(password, password_size);
    }
    if (!lightnvr_uuid_is_valid(uuid) || expected_revision < 1 ||
        !password || password_size == 0) {
        return DB_EVENT_DESTINATION_INVALID;
    }
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return DB_EVENT_DESTINATION_ERROR;
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(
        db, "SELECT revision,password FROM event_destinations WHERE uuid=?;",
        -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, uuid, -1, SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    db_event_destination_result_t outcome = DB_EVENT_DESTINATION_ERROR;
    if (result == SQLITE_ROW) {
        if (sqlite3_column_int64(statement, 0) != expected_revision) {
            outcome = DB_EVENT_DESTINATION_STALE;
        } else {
            const char *value =
                (const char *)sqlite3_column_text(statement, 1);
            size_t length = value ? strlen(value) : 0;
            if (length < password_size) {
                safe_strcpy(password, value ? value : "", password_size, 0);
                outcome = DB_EVENT_DESTINATION_OK;
            }
        }
    } else if (result == SQLITE_DONE) {
        outcome = DB_EVENT_DESTINATION_NOT_FOUND;
    }
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    return outcome;
}

uint64_t db_event_destination_generation(void) {
    return atomic_load_explicit(&DESTINATION_GENERATION,
                                memory_order_relaxed);
}
