#define _POSIX_C_SOURCE 200809L

#include "web/api_handlers_event_destinations.h"

#include <cjson/cJSON.h>
#include <errno.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "core/authorization.h"
#include "database/db_event_destinations.h"
#include "utils/memory.h"
#include "utils/strings.h"
#include "utils/uuid.h"
#include "web/audit_log.h"
#include "web/httpd_utils.h"

static bool authorize_events(const http_request_t *req, http_response_t *res,
                             user_t *user) {
    authorization_evaluation_t evaluation;
    return httpd_authorize_action(req, res, AUTHZ_EVENTS_CONFIGURE, NULL, user,
                                  &evaluation) != 0;
}

static bool extract_destination_uuid(
    const http_request_t *req, char uuid[EVENT_DESTINATION_UUID_MAX],
    http_response_t *res) {
    char value[MAX_PATH_LENGTH];
    if (http_request_extract_path_param(
            req, "/api/event-destinations/", value, sizeof(value)) != 0) {
        http_response_set_json_error(res, 400,
                                     "Invalid event destination path");
        return false;
    }
    char *slash = strchr(value, '/');
    if (slash) *slash = '\0';
    if (!lightnvr_uuid_is_valid(value)) {
        http_response_set_json_error(res, 400,
                                     "Invalid event destination UUID");
        return false;
    }
    safe_strcpy(uuid, value, EVENT_DESTINATION_UUID_MAX, 0);
    return true;
}

static void set_db_error(http_response_t *res,
                         db_event_destination_result_t result,
                         const char *validation_error) {
    switch (result) {
        case DB_EVENT_DESTINATION_NOT_FOUND:
            http_response_set_json_error(res, 404,
                                         "Event destination not found");
            break;
        case DB_EVENT_DESTINATION_CONFLICT:
            http_response_set_json_error(
                res, 409,
                "Event destination name or broker client ID already exists");
            break;
        case DB_EVENT_DESTINATION_STALE:
            http_response_set_json_error(
                res, 409,
                "Event destination was changed by another administrator");
            break;
        case DB_EVENT_DESTINATION_LIMIT:
            http_response_set_json_error(res, 409,
                                         "Event destination limit reached");
            break;
        case DB_EVENT_DESTINATION_IN_USE:
            http_response_set_json_error(
                res, 409, "Event destination is referenced by an event route");
            break;
        case DB_EVENT_DESTINATION_INVALID:
            http_response_set_json_error(
                res, 400,
                validation_error && validation_error[0]
                    ? validation_error : "Invalid event destination");
            break;
        default:
            http_response_set_json_error(res, 500,
                                         "Event destination operation failed");
            break;
    }
}

static bool set_json_response(http_response_t *res, int status,
                              cJSON *object) {
    char *body = object ? cJSON_PrintUnformatted(object) : NULL;
    cJSON_Delete(object);
    if (!body) {
        http_response_set_json_error(
            res, 500, "Failed to serialize event destination response");
        return false;
    }
    http_response_set_json(res, status, body);
    free(body);
    return true;
}

static cJSON *destination_to_json(const event_destination_t *destination) {
    cJSON *object = cJSON_CreateObject();
    cJSON *broker = cJSON_CreateObject();
    cJSON *authentication = cJSON_CreateObject();
    cJSON *tls = cJSON_CreateObject();
    if (!object || !broker || !authentication || !tls) {
        cJSON_Delete(object);
        cJSON_Delete(broker);
        cJSON_Delete(authentication);
        cJSON_Delete(tls);
        return NULL;
    }
    char key[EVENT_DESTINATION_KEY_MAX] = {0};
    if (db_event_destination_make_key(destination->uuid, key) != 0) {
        cJSON_Delete(object);
        cJSON_Delete(broker);
        cJSON_Delete(authentication);
        cJSON_Delete(tls);
        return NULL;
    }
    cJSON_AddStringToObject(object, "uuid", destination->uuid);
    cJSON_AddStringToObject(object, "key", key);
    cJSON_AddStringToObject(object, "name", destination->name);
    cJSON_AddStringToObject(object, "description", destination->description);
    cJSON_AddBoolToObject(object, "enabled", destination->enabled);
    cJSON_AddStringToObject(object, "type", destination->destination_type);
    cJSON_AddBoolToObject(object, "managed", true);

    cJSON_AddStringToObject(broker, "host", destination->broker_host);
    cJSON_AddNumberToObject(broker, "port", destination->broker_port);
    cJSON_AddStringToObject(broker, "client_id", destination->client_id);
    cJSON_AddStringToObject(broker, "topic_template",
                           destination->topic_template);
    cJSON_AddNumberToObject(broker, "keepalive_seconds",
                            destination->keepalive_seconds);
    cJSON_AddNumberToObject(broker, "qos", destination->qos);
    cJSON_AddItemToObject(object, "broker", broker);

    cJSON_AddStringToObject(authentication, "username",
                           destination->username);
    cJSON_AddBoolToObject(authentication, "password_configured",
                          destination->password_configured);
    cJSON_AddItemToObject(object, "authentication", authentication);

    cJSON_AddStringToObject(tls, "mode", destination->tls_mode);
    cJSON_AddStringToObject(tls, "ca_file", destination->ca_file);
    cJSON_AddStringToObject(tls, "cert_file", destination->cert_file);
    cJSON_AddStringToObject(tls, "key_file", destination->key_file);
    cJSON_AddItemToObject(object, "tls", tls);

    cJSON_AddNumberToObject(object, "revision", (double)destination->revision);
    cJSON_AddNumberToObject(object, "created_at",
                            (double)destination->created_at);
    cJSON_AddNumberToObject(object, "updated_at",
                            (double)destination->updated_at);
    return object;
}

static bool object_has_only(const cJSON *object,
                            const char *const *fields, size_t field_count,
                            const char *context, http_response_t *res) {
    if (!cJSON_IsObject(object)) {
        char message[128];
        snprintf(message, sizeof(message), "%s must be an object", context);
        http_response_set_json_error(res, 400, message);
        return false;
    }
    for (const cJSON *item = object->child; item; item = item->next) {
        bool known = false;
        for (size_t index = 0; index < field_count; index++) {
            if (item->string && strcmp(item->string, fields[index]) == 0) {
                known = true;
                break;
            }
        }
        if (!known) {
            char message[192];
            snprintf(message, sizeof(message), "%s contains unknown field '%s'",
                     context, item->string ? item->string : "");
            http_response_set_json_error(res, 400, message);
            return false;
        }
    }
    return true;
}

static bool copy_string_field(const cJSON *object, const char *field,
                              char *destination, size_t destination_size,
                              http_response_t *res) {
    const cJSON *item = cJSON_GetObjectItemCaseSensitive(object, field);
    if (!item) return true;
    if (!cJSON_IsString(item) || !item->valuestring ||
        strlen(item->valuestring) >= destination_size) {
        char message[160];
        snprintf(message, sizeof(message), "%s must be a valid string", field);
        http_response_set_json_error(res, 400, message);
        return false;
    }
    safe_strcpy(destination, item->valuestring, destination_size, 0);
    return true;
}

static bool apply_integer_field(const cJSON *object, const char *field,
                                int minimum, int maximum, int *destination,
                                http_response_t *res) {
    const cJSON *item = cJSON_GetObjectItemCaseSensitive(object, field);
    if (!item) return true;
    if (!cJSON_IsNumber(item) || item->valuedouble != item->valueint ||
        item->valueint < minimum || item->valueint > maximum) {
        char message[160];
        snprintf(message, sizeof(message),
                 "%s must be an integer from %d to %d", field, minimum,
                 maximum);
        http_response_set_json_error(res, 400, message);
        return false;
    }
    *destination = item->valueint;
    return true;
}

static bool initialize_defaults(event_destination_t *destination) {
    memset(destination, 0, sizeof(*destination));
    destination->enabled = true;
    safe_strcpy(destination->destination_type, "mqtt",
                sizeof(destination->destination_type), 0);
    destination->broker_port = 8883;
    char random_uuid[EVENT_DESTINATION_UUID_MAX];
    if (lightnvr_uuid_generate_v4(random_uuid) != 0) return false;
    int client_id_length = snprintf(
        destination->client_id, sizeof(destination->client_id),
        "lightnvr-%.8s%.4s", random_uuid, random_uuid + 9);
    if (client_id_length < 0 ||
        (size_t)client_id_length >= sizeof(destination->client_id)) {
        return false;
    }
    safe_strcpy(destination->topic_template,
                EVENT_DESTINATION_DEFAULT_TOPIC_TEMPLATE,
                sizeof(destination->topic_template), 0);
    safe_strcpy(destination->tls_mode, "system",
                sizeof(destination->tls_mode), 0);
    destination->keepalive_seconds = 60;
    destination->qos = 1;
    return true;
}

static bool apply_broker(const cJSON *broker,
                         event_destination_t *destination,
                         http_response_t *res) {
    const char *const fields[] = {
        "host", "port", "client_id", "topic_template", "keepalive_seconds",
        "qos"
    };
    return object_has_only(broker, fields, 6, "broker", res) &&
        copy_string_field(broker, "host", destination->broker_host,
                          sizeof(destination->broker_host), res) &&
        apply_integer_field(broker, "port", 1, 65535,
                            &destination->broker_port, res) &&
        copy_string_field(broker, "client_id", destination->client_id,
                          sizeof(destination->client_id), res) &&
        copy_string_field(broker, "topic_template",
                          destination->topic_template,
                          sizeof(destination->topic_template), res) &&
        apply_integer_field(broker, "keepalive_seconds", 5, 3600,
                            &destination->keepalive_seconds, res) &&
        apply_integer_field(broker, "qos", 0, 2, &destination->qos, res);
}

static bool apply_authentication(
    const cJSON *authentication, event_destination_t *destination,
    char password[EVENT_DESTINATION_PASSWORD_MAX], bool *replace_password,
    http_response_t *res) {
    const char *const fields[] = {"username", "password"};
    if (!object_has_only(authentication, fields, 2, "authentication", res) ||
        !copy_string_field(authentication, "username", destination->username,
                           sizeof(destination->username), res)) {
        return false;
    }
    const cJSON *item =
        cJSON_GetObjectItemCaseSensitive(authentication, "password");
    if (!item) return true;
    if (cJSON_IsNull(item)) {
        password[0] = '\0';
    } else if (cJSON_IsString(item) && item->valuestring &&
               strlen(item->valuestring) < EVENT_DESTINATION_PASSWORD_MAX) {
        safe_strcpy(password, item->valuestring,
                    EVENT_DESTINATION_PASSWORD_MAX, 0);
    } else {
        http_response_set_json_error(
            res, 400, "authentication.password must be a string or null");
        return false;
    }
    *replace_password = true;
    destination->password_configured = password[0] != '\0';
    return true;
}

static bool apply_tls(const cJSON *tls, event_destination_t *destination,
                      http_response_t *res) {
    const char *const fields[] = {"mode", "ca_file", "cert_file", "key_file"};
    return object_has_only(tls, fields, 4, "tls", res) &&
        copy_string_field(tls, "mode", destination->tls_mode,
                          sizeof(destination->tls_mode), res) &&
        copy_string_field(tls, "ca_file", destination->ca_file,
                          sizeof(destination->ca_file), res) &&
        copy_string_field(tls, "cert_file", destination->cert_file,
                          sizeof(destination->cert_file), res) &&
        copy_string_field(tls, "key_file", destination->key_file,
                          sizeof(destination->key_file), res);
}

static bool parse_positive_revision(const cJSON *revision, int64_t *value,
                                    http_response_t *res) {
    double number = cJSON_IsNumber(revision) ? revision->valuedouble : 0.0;
    if (!cJSON_IsNumber(revision) || number < 1.0 ||
        number > 9007199254740991.0 || number != (double)(int64_t)number) {
        http_response_set_json_error(res, 400,
                                     "revision must be a positive integer");
        return false;
    }
    *value = (int64_t)number;
    return true;
}

static bool apply_body(const cJSON *body, event_destination_t *destination,
                       bool creating, int64_t *expected_revision,
                       char password[EVENT_DESTINATION_PASSWORD_MAX],
                       bool *replace_password, http_response_t *res) {
    const char *const fields[] = {
        "name", "description", "enabled", "type", "broker",
        "authentication", "tls", "revision"
    };
    if (!object_has_only(body, fields, 8, "request body", res)) return false;
    if (!copy_string_field(body, "name", destination->name,
                           sizeof(destination->name), res) ||
        !copy_string_field(body, "description", destination->description,
                           sizeof(destination->description), res) ||
        !copy_string_field(body, "type", destination->destination_type,
                           sizeof(destination->destination_type), res)) {
        return false;
    }
    const cJSON *name = cJSON_GetObjectItemCaseSensitive(body, "name");
    const cJSON *enabled = cJSON_GetObjectItemCaseSensitive(body, "enabled");
    const cJSON *broker = cJSON_GetObjectItemCaseSensitive(body, "broker");
    const cJSON *authentication =
        cJSON_GetObjectItemCaseSensitive(body, "authentication");
    const cJSON *tls = cJSON_GetObjectItemCaseSensitive(body, "tls");
    const cJSON *revision = cJSON_GetObjectItemCaseSensitive(body, "revision");
    if (creating && !name) {
        http_response_set_json_error(res, 400,
                                     "Event destination name is required");
        return false;
    }
    if (enabled) {
        if (!cJSON_IsBool(enabled)) {
            http_response_set_json_error(res, 400,
                                         "enabled must be boolean");
            return false;
        }
        destination->enabled = cJSON_IsTrue(enabled);
    }
    if (creating && !broker) {
        http_response_set_json_error(res, 400, "broker is required");
        return false;
    }
    if ((broker && !apply_broker(broker, destination, res)) ||
        (authentication && !apply_authentication(
            authentication, destination, password, replace_password, res)) ||
        (tls && !apply_tls(tls, destination, res))) {
        return false;
    }
    if (creating) {
        if (revision) {
            http_response_set_json_error(res, 400,
                                         "revision is assigned by the server");
            return false;
        }
        /* Create always writes an explicit empty or supplied password. */
        *replace_password = true;
    } else if (!parse_positive_revision(revision, expected_revision, res)) {
        return false;
    }
    char validation_error[EVENT_DESTINATION_VALIDATION_ERROR_MAX] = {0};
    db_event_destination_result_t validation = db_event_destination_validate(
        destination, password, *replace_password, validation_error,
        sizeof(validation_error));
    if (validation != DB_EVENT_DESTINATION_OK) {
        set_db_error(res, validation, validation_error);
        return false;
    }
    return true;
}

static void audit_destination(const http_request_t *req, const user_t *user,
                              const char *uuid, const char *operation,
                              const char *outcome, const char *reason) {
    cJSON *context = cJSON_CreateObject();
    if (context && reason) cJSON_AddStringToObject(context, "reason", reason);
    audit_log_operation(req, user, "events.configure", "event_destination",
                        uuid, operation, outcome, context);
    cJSON_Delete(context);
}

void handle_get_event_destinations(const http_request_t *req,
                                   http_response_t *res) {
    user_t user;
    if (!authorize_events(req, res, &user)) return;
    int total = db_event_destination_count();
    if (total < 0) {
        set_db_error(res, DB_EVENT_DESTINATION_ERROR, NULL);
        return;
    }
    if (total > EVENT_DESTINATION_MAX_COUNT) {
        set_db_error(res, DB_EVENT_DESTINATION_ERROR, NULL);
        return;
    }
    event_destination_t *destinations = total > 0
        ? calloc((size_t)total, sizeof(*destinations)) : NULL;
    if (total > 0 && !destinations) {
        http_response_set_json_error(res, 500, "Out of memory");
        return;
    }
    int count = total == 0 ? 0 : db_event_destination_list(
        destinations, total);
    if (count < 0) {
        free(destinations);
        set_db_error(res, DB_EVENT_DESTINATION_ERROR, NULL);
        return;
    }
    cJSON *root = cJSON_CreateObject();
    cJSON *legacy = cJSON_CreateObject();
    cJSON *items = cJSON_CreateArray();
    if (!root || !legacy || !items) {
        cJSON_Delete(root);
        cJSON_Delete(legacy);
        cJSON_Delete(items);
        free(destinations);
        set_db_error(res, DB_EVENT_DESTINATION_ERROR, NULL);
        return;
    }
    cJSON_AddStringToObject(legacy, "key", "mqtt:default");
    cJSON_AddStringToObject(legacy, "name", "Default MQTT settings");
    cJSON_AddStringToObject(legacy, "configuration_source", "settings");
    cJSON_AddBoolToObject(legacy, "managed", false);
    cJSON_AddItemToObject(root, "default_destination", legacy);
    cJSON_AddNumberToObject(root, "count", count);
    cJSON_AddItemToObject(root, "destinations", items);
    for (int index = 0; index < count; index++) {
        cJSON *item = destination_to_json(&destinations[index]);
        if (!item) {
            cJSON_Delete(root);
            free(destinations);
            set_db_error(res, DB_EVENT_DESTINATION_ERROR, NULL);
            return;
        }
        cJSON_AddItemToArray(items, item);
    }
    free(destinations);
    set_json_response(res, 200, root);
}

void handle_post_event_destination(const http_request_t *req,
                                   http_response_t *res) {
    user_t user;
    if (!authorize_events(req, res, &user)) return;
    cJSON *body = httpd_parse_json_body(req);
    event_destination_t destination;
    if (!initialize_defaults(&destination)) {
        cJSON_Delete(body);
        set_db_error(res, DB_EVENT_DESTINATION_ERROR, NULL);
        audit_destination(req, &user, NULL, "destination_create", "error",
                          "default_generation_failed");
        return;
    }
    int64_t unused_revision = 0;
    char password[EVENT_DESTINATION_PASSWORD_MAX] = {0};
    bool replace_password = false;
    if (!apply_body(body, &destination, true, &unused_revision, password,
                    &replace_password, res)) {
        cJSON_Delete(body);
        secure_zero_memory(password, sizeof(password));
        audit_destination(req, &user, NULL, "destination_create", "failure",
                          "invalid_request");
        return;
    }
    cJSON_Delete(body);
    db_event_destination_result_t result =
        db_event_destination_create(&destination, password);
    secure_zero_memory(password, sizeof(password));
    if (result != DB_EVENT_DESTINATION_OK) {
        set_db_error(res, result, NULL);
        audit_destination(req, &user, NULL, "destination_create",
                          result == DB_EVENT_DESTINATION_ERROR
                              ? "error" : "failure",
                          "persistence_failed");
        return;
    }
    audit_destination(req, &user, destination.uuid, "destination_create",
                      "success", "created");
    set_json_response(res, 201, destination_to_json(&destination));
}

void handle_get_event_destination(const http_request_t *req,
                                  http_response_t *res) {
    user_t user;
    if (!authorize_events(req, res, &user)) return;
    char uuid[EVENT_DESTINATION_UUID_MAX];
    if (!extract_destination_uuid(req, uuid, res)) return;
    event_destination_t destination;
    db_event_destination_result_t result =
        db_event_destination_get(uuid, &destination);
    if (result != DB_EVENT_DESTINATION_OK) {
        set_db_error(res, result, NULL);
        return;
    }
    set_json_response(res, 200, destination_to_json(&destination));
}

void handle_put_event_destination(const http_request_t *req,
                                  http_response_t *res) {
    user_t user;
    if (!authorize_events(req, res, &user)) return;
    char uuid[EVENT_DESTINATION_UUID_MAX];
    if (!extract_destination_uuid(req, uuid, res)) return;
    event_destination_t destination;
    db_event_destination_result_t result =
        db_event_destination_get(uuid, &destination);
    if (result != DB_EVENT_DESTINATION_OK) {
        set_db_error(res, result, NULL);
        return;
    }
    cJSON *body = httpd_parse_json_body(req);
    int64_t expected_revision = 0;
    char password[EVENT_DESTINATION_PASSWORD_MAX] = {0};
    bool replace_password = false;
    if (!apply_body(body, &destination, false, &expected_revision, password,
                    &replace_password, res)) {
        cJSON_Delete(body);
        secure_zero_memory(password, sizeof(password));
        audit_destination(req, &user, uuid, "destination_update", "failure",
                          "invalid_request");
        return;
    }
    cJSON_Delete(body);
    result = db_event_destination_update(
        &destination, expected_revision, password, replace_password);
    secure_zero_memory(password, sizeof(password));
    if (result != DB_EVENT_DESTINATION_OK) {
        set_db_error(res, result, NULL);
        audit_destination(
            req, &user, uuid, "destination_update",
            result == DB_EVENT_DESTINATION_ERROR ? "error" : "failure",
            result == DB_EVENT_DESTINATION_STALE
                ? "stale_revision" : "persistence_failed");
        return;
    }
    audit_destination(req, &user, uuid, "destination_update", "success",
                      "updated");
    set_json_response(res, 200, destination_to_json(&destination));
}

static bool parse_delete_revision(const http_request_t *req,
                                  int64_t *revision,
                                  http_response_t *res) {
    char value[64];
    if (http_request_get_query_param(req, "revision", value,
                                     sizeof(value)) < 0) {
        http_response_set_json_error(res, 400,
                                     "revision query parameter is required");
        return false;
    }
    errno = 0;
    char *end = NULL;
    long long parsed = strtoll(value, &end, 10);
    if (errno == ERANGE || !end || *end != '\0' || parsed < 1) {
        http_response_set_json_error(res, 400,
                                     "revision must be a positive integer");
        return false;
    }
    *revision = parsed;
    return true;
}

void handle_delete_event_destination(const http_request_t *req,
                                     http_response_t *res) {
    user_t user;
    if (!authorize_events(req, res, &user)) return;
    char uuid[EVENT_DESTINATION_UUID_MAX];
    int64_t revision = 0;
    if (!extract_destination_uuid(req, uuid, res) ||
        !parse_delete_revision(req, &revision, res)) {
        return;
    }
    db_event_destination_result_t result =
        db_event_destination_delete(uuid, revision);
    if (result != DB_EVENT_DESTINATION_OK) {
        set_db_error(res, result, NULL);
        audit_destination(
            req, &user, uuid, "destination_delete",
            result == DB_EVENT_DESTINATION_ERROR ? "error" : "failure",
            result == DB_EVENT_DESTINATION_STALE
                ? "stale_revision"
                : result == DB_EVENT_DESTINATION_IN_USE
                    ? "destination_in_use" : "persistence_failed");
        return;
    }
    audit_destination(req, &user, uuid, "destination_delete", "success",
                      "deleted");
    http_response_set_json(res, 200, "{\"success\":true}");
}
