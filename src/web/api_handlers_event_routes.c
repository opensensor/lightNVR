#define _POSIX_C_SOURCE 200809L

#include "web/api_handlers_event_routes.h"

#include <cjson/cJSON.h>
#include <errno.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "core/authorization.h"
#include "core/camera_selector.h"
#include "core/event_envelope.h"
#include "database/db_event_routes.h"
#include "database/db_fleet_query.h"
#include "utils/strings.h"
#include "utils/uuid.h"
#include "web/audit_log.h"
#include "web/httpd_utils.h"

#define EVENT_ROUTE_PREVIEW_MAX_CAMERAS 20

static bool authorize_events(const http_request_t *req, http_response_t *res,
                             user_t *user) {
    authorization_evaluation_t evaluation;
    return httpd_authorize_action(req, res, AUTHZ_EVENTS_CONFIGURE, NULL, user,
                                  &evaluation) != 0;
}

static bool extract_route_uuid(const http_request_t *req,
                               char uuid[EVENT_ROUTE_UUID_MAX],
                               http_response_t *res) {
    char value[MAX_PATH_LENGTH];
    if (http_request_extract_path_param(req, "/api/event-routes/", value,
                                        sizeof(value)) != 0) {
        http_response_set_json_error(res, 400, "Invalid event route path");
        return false;
    }
    char *slash = strchr(value, '/');
    if (slash) *slash = '\0';
    if (!lightnvr_uuid_is_valid(value)) {
        http_response_set_json_error(res, 400, "Invalid event route UUID");
        return false;
    }
    safe_strcpy(uuid, value, EVENT_ROUTE_UUID_MAX, 0);
    return true;
}

static void set_db_error(http_response_t *res,
                         db_event_route_result_t result,
                         const char *validation_error) {
    switch (result) {
        case DB_EVENT_ROUTE_NOT_FOUND:
            http_response_set_json_error(res, 404, "Event route not found");
            break;
        case DB_EVENT_ROUTE_CONFLICT:
            http_response_set_json_error(res, 409,
                                         "Event route name already exists");
            break;
        case DB_EVENT_ROUTE_STALE:
            http_response_set_json_error(
                res, 409, "Event route was changed by another administrator");
            break;
        case DB_EVENT_ROUTE_LIMIT:
            http_response_set_json_error(res, 409,
                                         "Event route limit reached");
            break;
        case DB_EVENT_ROUTE_INVALID:
            http_response_set_json_error(
                res, 400, validation_error && validation_error[0]
                              ? validation_error : "Invalid event route");
            break;
        default:
            http_response_set_json_error(res, 500,
                                         "Event route operation failed");
            break;
    }
}

static cJSON *parse_stored_object(const char *json) {
    cJSON *object = cJSON_Parse(json);
    if (!cJSON_IsObject(object)) {
        cJSON_Delete(object);
        return NULL;
    }
    return object;
}

static cJSON *route_to_json(const event_route_t *route) {
    cJSON *object = cJSON_CreateObject();
    cJSON *event_types = cJSON_CreateArray();
    cJSON *scope = cJSON_CreateObject();
    cJSON *predicate = parse_stored_object(route->predicate_json);
    cJSON *schedule = parse_stored_object(route->schedule_json);
    cJSON *suppression = cJSON_CreateObject();
    if (!object || !event_types || !scope || !predicate || !schedule ||
        !suppression) {
        cJSON_Delete(object);
        cJSON_Delete(event_types);
        cJSON_Delete(scope);
        cJSON_Delete(predicate);
        cJSON_Delete(schedule);
        cJSON_Delete(suppression);
        return NULL;
    }
    cJSON_AddStringToObject(object, "uuid", route->uuid);
    cJSON_AddStringToObject(object, "name", route->name);
    cJSON_AddStringToObject(object, "description", route->description);
    cJSON_AddBoolToObject(object, "enabled", route->enabled);
    cJSON_AddStringToObject(object, "destination", route->destination_key);
    for (int index = 0; index < route->event_type_count; index++) {
        cJSON_AddItemToArray(event_types,
                             cJSON_CreateString(route->event_types[index]));
    }
    cJSON_AddItemToObject(object, "event_types", event_types);
    cJSON_AddStringToObject(scope, "type", route->scope_type);
    if (strcmp(route->scope_type, "selector") == 0) {
        cJSON *selector = parse_stored_object(route->selector_json);
        if (!selector) {
            cJSON_Delete(object);
            cJSON_Delete(scope);
            cJSON_Delete(predicate);
            cJSON_Delete(schedule);
            cJSON_Delete(suppression);
            return NULL;
        }
        cJSON_AddItemToObject(scope, "selector", selector);
    }
    cJSON_AddItemToObject(object, "camera_scope", scope);
    cJSON_AddItemToObject(object, "predicate", predicate);
    cJSON_AddItemToObject(object, "schedule", schedule);
    cJSON_AddNumberToObject(suppression, "debounce_seconds",
                            route->debounce_seconds);
    cJSON_AddNumberToObject(suppression, "cooldown_seconds",
                            route->cooldown_seconds);
    cJSON_AddNumberToObject(suppression, "grouping_window_seconds",
                            route->grouping_window_seconds);
    cJSON_AddNumberToObject(suppression, "max_events_per_minute",
                            route->max_events_per_minute);
    cJSON_AddItemToObject(object, "suppression", suppression);
    cJSON_AddNumberToObject(object, "revision", (double)route->revision);
    cJSON_AddNumberToObject(object, "created_at", (double)route->created_at);
    cJSON_AddNumberToObject(object, "updated_at", (double)route->updated_at);
    return object;
}

static bool set_json_response(http_response_t *res, int status,
                              cJSON *object) {
    char *body = object ? cJSON_PrintUnformatted(object) : NULL;
    cJSON_Delete(object);
    if (!body) {
        http_response_set_json_error(res, 500,
                                     "Failed to serialize event route response");
        return false;
    }
    http_response_set_json(res, status, body);
    free(body);
    return true;
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

static bool store_json_object(const cJSON *object, char *destination,
                              size_t destination_size, const char *field,
                              http_response_t *res) {
    if (!cJSON_IsObject(object)) {
        char message[128];
        snprintf(message, sizeof(message), "%s must be an object", field);
        http_response_set_json_error(res, 400, message);
        return false;
    }
    char *serialized = cJSON_PrintUnformatted(object);
    if (!serialized || strlen(serialized) >= destination_size) {
        free(serialized);
        char message[128];
        snprintf(message, sizeof(message), "%s is too large", field);
        http_response_set_json_error(res, 400, message);
        return false;
    }
    safe_strcpy(destination, serialized, destination_size, 0);
    free(serialized);
    return true;
}

static bool parse_integer(const cJSON *object, const char *field, int minimum,
                          int maximum, int *destination,
                          http_response_t *res) {
    const cJSON *item = cJSON_GetObjectItemCaseSensitive(object, field);
    if (!item) return true;
    if (!cJSON_IsNumber(item) || item->valuedouble != item->valueint ||
        item->valueint < minimum || item->valueint > maximum) {
        char message[160];
        snprintf(message, sizeof(message), "%s must be an integer from %d to %d",
                 field, minimum, maximum);
        http_response_set_json_error(res, 400, message);
        return false;
    }
    *destination = item->valueint;
    return true;
}

static bool apply_event_types(const cJSON *array, event_route_t *route,
                              http_response_t *res) {
    int count = cJSON_IsArray(array) ? cJSON_GetArraySize(array) : -1;
    if (count < 1 || count > EVENT_ROUTE_MAX_TYPES) {
        http_response_set_json_error(res, 400,
                                     "event_types must contain 1-32 values");
        return false;
    }
    memset(route->event_types, 0, sizeof(route->event_types));
    route->event_type_count = 0;
    for (int index = 0; index < count; index++) {
        const cJSON *item = cJSON_GetArrayItem(array, index);
        if (!cJSON_IsString(item) || !item->valuestring ||
            strlen(item->valuestring) >= EVENT_TYPE_MAX) {
            http_response_set_json_error(res, 400,
                                         "event_types contains an invalid value");
            return false;
        }
        safe_strcpy(route->event_types[route->event_type_count++],
                    item->valuestring, EVENT_TYPE_MAX, 0);
    }
    return true;
}

static bool apply_camera_scope(const cJSON *scope, event_route_t *route,
                               http_response_t *res) {
    const char *const fields[] = {"type", "selector"};
    if (!object_has_only(scope, fields, 2, "camera_scope", res)) return false;
    const cJSON *type = cJSON_GetObjectItemCaseSensitive(scope, "type");
    const cJSON *selector =
        cJSON_GetObjectItemCaseSensitive(scope, "selector");
    if (!cJSON_IsString(type) ||
        (strcmp(type->valuestring, "all") != 0 &&
         strcmp(type->valuestring, "selector") != 0)) {
        http_response_set_json_error(
            res, 400, "camera_scope.type must be all or selector");
        return false;
    }
    safe_strcpy(route->scope_type, type->valuestring,
                sizeof(route->scope_type), 0);
    if (strcmp(route->scope_type, "all") == 0) {
        if (selector) {
            http_response_set_json_error(
                res, 400, "all camera scope cannot include selector");
            return false;
        }
        route->selector_json[0] = '\0';
        return true;
    }
    if (!selector) {
        http_response_set_json_error(
            res, 400, "selector camera scope requires selector");
        return false;
    }
    return store_json_object(selector, route->selector_json,
                             sizeof(route->selector_json), "camera selector",
                             res);
}

static bool apply_suppression(const cJSON *suppression, event_route_t *route,
                              http_response_t *res) {
    const char *const fields[] = {
        "debounce_seconds", "cooldown_seconds", "grouping_window_seconds",
        "max_events_per_minute"
    };
    return object_has_only(suppression, fields, 4, "suppression", res) &&
        parse_integer(suppression, "debounce_seconds", 0, 86400,
                      &route->debounce_seconds, res) &&
        parse_integer(suppression, "cooldown_seconds", 0, 604800,
                      &route->cooldown_seconds, res) &&
        parse_integer(suppression, "grouping_window_seconds", 0, 3600,
                      &route->grouping_window_seconds, res) &&
        parse_integer(suppression, "max_events_per_minute", 0, 60000,
                      &route->max_events_per_minute, res);
}

static void initialize_route_defaults(event_route_t *route) {
    memset(route, 0, sizeof(*route));
    route->enabled = true;
    safe_strcpy(route->destination_key, EVENT_ROUTE_DEFAULT_DESTINATION,
                sizeof(route->destination_key), 0);
    safe_strcpy(route->scope_type, "all", sizeof(route->scope_type), 0);
    safe_strcpy(route->predicate_json, "{\"version\":1}",
                sizeof(route->predicate_json), 0);
    safe_strcpy(route->schedule_json,
                "{\"version\":1,\"timezone\":\"UTC\",\"windows\":[]}",
                sizeof(route->schedule_json), 0);
}

static bool apply_route_body(const cJSON *body, event_route_t *route,
                             bool creating, int64_t *expected_revision,
                             http_response_t *res) {
    const char *const fields[] = {
        "name", "description", "enabled", "destination", "event_types",
        "camera_scope", "predicate", "schedule", "suppression", "revision"
    };
    if (!object_has_only(body, fields, 10, "request body", res)) return false;

    const cJSON *name = cJSON_GetObjectItemCaseSensitive(body, "name");
    if (name) {
        if (!cJSON_IsString(name) || !name->valuestring ||
            strlen(name->valuestring) >= sizeof(route->name)) {
            http_response_set_json_error(res, 400, "Invalid event route name");
            return false;
        }
        safe_strcpy(route->name, name->valuestring, sizeof(route->name), 0);
    } else if (creating) {
        http_response_set_json_error(res, 400, "Event route name is required");
        return false;
    }
    const cJSON *description =
        cJSON_GetObjectItemCaseSensitive(body, "description");
    if (description) {
        if (!cJSON_IsString(description) || !description->valuestring ||
            strlen(description->valuestring) >= sizeof(route->description)) {
            http_response_set_json_error(res, 400,
                                         "Invalid event route description");
            return false;
        }
        safe_strcpy(route->description, description->valuestring,
                    sizeof(route->description), 0);
    }
    const cJSON *enabled = cJSON_GetObjectItemCaseSensitive(body, "enabled");
    if (enabled) {
        if (!cJSON_IsBool(enabled)) {
            http_response_set_json_error(res, 400, "enabled must be boolean");
            return false;
        }
        route->enabled = cJSON_IsTrue(enabled);
    }
    const cJSON *destination =
        cJSON_GetObjectItemCaseSensitive(body, "destination");
    if (destination) {
        if (!cJSON_IsString(destination) || !destination->valuestring ||
            strlen(destination->valuestring) >=
                sizeof(route->destination_key)) {
            http_response_set_json_error(res, 400, "Invalid destination");
            return false;
        }
        safe_strcpy(route->destination_key, destination->valuestring,
                    sizeof(route->destination_key), 0);
    }
    const cJSON *event_types =
        cJSON_GetObjectItemCaseSensitive(body, "event_types");
    if (event_types) {
        if (!apply_event_types(event_types, route, res)) return false;
    } else if (creating) {
        http_response_set_json_error(res, 400, "event_types is required");
        return false;
    }
    const cJSON *scope =
        cJSON_GetObjectItemCaseSensitive(body, "camera_scope");
    if (scope && !apply_camera_scope(scope, route, res)) return false;
    const cJSON *predicate =
        cJSON_GetObjectItemCaseSensitive(body, "predicate");
    if (predicate && !store_json_object(
            predicate, route->predicate_json, sizeof(route->predicate_json),
            "predicate", res)) {
        return false;
    }
    const cJSON *schedule =
        cJSON_GetObjectItemCaseSensitive(body, "schedule");
    if (schedule && !store_json_object(
            schedule, route->schedule_json, sizeof(route->schedule_json),
            "schedule", res)) {
        return false;
    }
    const cJSON *suppression =
        cJSON_GetObjectItemCaseSensitive(body, "suppression");
    if (suppression && !apply_suppression(suppression, route, res)) {
        return false;
    }
    const cJSON *revision =
        cJSON_GetObjectItemCaseSensitive(body, "revision");
    if (!creating) {
        double value = cJSON_IsNumber(revision) ? revision->valuedouble : 0.0;
        if (!cJSON_IsNumber(revision) || value < 1.0 ||
            value > 9007199254740991.0 ||
            value != (double)(int64_t)value) {
            http_response_set_json_error(res, 400,
                                         "revision must be a positive integer");
            return false;
        }
        *expected_revision = (int64_t)value;
    } else if (revision) {
        http_response_set_json_error(res, 400,
                                     "revision is assigned by the server");
        return false;
    }
    char validation_error[EVENT_ROUTE_VALIDATION_ERROR_MAX] = {0};
    db_event_route_result_t validation = db_event_route_validate(
        route, validation_error, sizeof(validation_error));
    if (validation != DB_EVENT_ROUTE_OK) {
        set_db_error(res, validation, validation_error);
        return false;
    }
    return true;
}

static void audit_route(const http_request_t *req, const user_t *user,
                        const char *uuid, const char *operation,
                        const char *outcome, const char *reason) {
    cJSON *context = cJSON_CreateObject();
    if (context && reason) cJSON_AddStringToObject(context, "reason", reason);
    audit_log_operation(req, user, "events.configure", "event_route", uuid,
                        operation, outcome, context);
    cJSON_Delete(context);
}

void handle_get_event_catalog(const http_request_t *req,
                              http_response_t *res) {
    user_t user;
    if (!authorize_events(req, res, &user)) return;
    int count = 0;
    const event_type_definition_t *definitions = event_registry_all(&count);
    cJSON *root = cJSON_CreateObject();
    cJSON *types = cJSON_CreateArray();
    if (!root || !types) {
        cJSON_Delete(root);
        cJSON_Delete(types);
        http_response_set_json_error(res, 500, "Failed to create event catalog");
        return;
    }
    cJSON_AddNumberToObject(root, "schema_version", 1);
    cJSON_AddNumberToObject(root, "count", count);
    cJSON_AddItemToObject(root, "event_types", types);
    for (int index = 0; index < count; index++) {
        cJSON *item = cJSON_CreateObject();
        if (!item) {
            cJSON_Delete(root);
            http_response_set_json_error(res, 500,
                                         "Failed to create event catalog");
            return;
        }
        cJSON_AddStringToObject(item, "type", definitions[index].type);
        cJSON_AddStringToObject(item, "family", definitions[index].family);
        cJSON_AddStringToObject(item, "description",
                                definitions[index].description);
        cJSON_AddStringToObject(item, "severity",
                                event_severity_name(definitions[index].severity));
        cJSON_AddStringToObject(
            item, "sensitivity",
            event_sensitivity_name(definitions[index].sensitivity));
        cJSON_AddStringToObject(
            item, "media_policy",
            event_media_policy_name(definitions[index].media_policy));
        cJSON_AddStringToObject(
            item, "expected_rate",
            event_expected_rate_name(definitions[index].expected_rate));
        cJSON_AddStringToObject(
            item, "subject_kind",
            definitions[index].subject_kind == EVENT_SUBJECT_CAMERA
                ? "camera" : "storage");
        cJSON_AddNumberToObject(item, "default_expiry_seconds",
                                definitions[index].default_expiry_seconds);
        cJSON_AddItemToArray(types, item);
    }
    set_json_response(res, 200, root);
}

void handle_get_event_routes(const http_request_t *req,
                             http_response_t *res) {
    user_t user;
    if (!authorize_events(req, res, &user)) return;
    int total = db_event_route_count();
    if (total < 0 || total > EVENT_ROUTE_MAX_COUNT) {
        http_response_set_json_error(res, 500, "Failed to count event routes");
        return;
    }
    event_route_t *routes = total > 0
        ? calloc((size_t)total, sizeof(*routes)) : NULL;
    if (total > 0 && !routes) {
        http_response_set_json_error(res, 500, "Out of memory");
        return;
    }
    int count = total > 0 ? db_event_route_list(routes, total) : 0;
    if (count < 0) {
        free(routes);
        http_response_set_json_error(res, 500, "Failed to list event routes");
        return;
    }
    cJSON *root = cJSON_CreateObject();
    cJSON *items = cJSON_CreateArray();
    if (!root || !items) {
        cJSON_Delete(root);
        cJSON_Delete(items);
        free(routes);
        http_response_set_json_error(res, 500, "Failed to create response");
        return;
    }
    cJSON_AddNumberToObject(root, "count", count);
    cJSON_AddItemToObject(root, "routes", items);
    for (int index = 0; index < count; index++) {
        cJSON *item = route_to_json(&routes[index]);
        if (!item) {
            cJSON_Delete(root);
            free(routes);
            http_response_set_json_error(res, 500,
                                         "Failed to create response");
            return;
        }
        cJSON_AddItemToArray(items, item);
    }
    free(routes);
    set_json_response(res, 200, root);
}

void handle_post_event_route(const http_request_t *req,
                             http_response_t *res) {
    user_t user;
    if (!authorize_events(req, res, &user)) return;
    cJSON *body = httpd_parse_json_body(req);
    event_route_t route;
    initialize_route_defaults(&route);
    int64_t unused_revision = 0;
    if (!apply_route_body(body, &route, true, &unused_revision, res)) {
        cJSON_Delete(body);
        audit_route(req, &user, NULL, "route_create", "failure",
                    "invalid_request");
        return;
    }
    cJSON_Delete(body);
    db_event_route_result_t result = db_event_route_create(&route);
    if (result != DB_EVENT_ROUTE_OK) {
        set_db_error(res, result, NULL);
        audit_route(req, &user, NULL, "route_create",
                    result == DB_EVENT_ROUTE_ERROR ? "error" : "failure",
                    "persistence_failed");
        return;
    }
    audit_route(req, &user, route.uuid, "route_create", "success",
                "created");
    set_json_response(res, 201, route_to_json(&route));
}

void handle_get_event_route(const http_request_t *req,
                            http_response_t *res) {
    user_t user;
    if (!authorize_events(req, res, &user)) return;
    char uuid[EVENT_ROUTE_UUID_MAX];
    if (!extract_route_uuid(req, uuid, res)) return;
    event_route_t route;
    db_event_route_result_t result = db_event_route_get(uuid, &route);
    if (result != DB_EVENT_ROUTE_OK) {
        set_db_error(res, result, NULL);
        return;
    }
    set_json_response(res, 200, route_to_json(&route));
}

void handle_put_event_route(const http_request_t *req,
                            http_response_t *res) {
    user_t user;
    if (!authorize_events(req, res, &user)) return;
    char uuid[EVENT_ROUTE_UUID_MAX];
    if (!extract_route_uuid(req, uuid, res)) return;
    event_route_t route;
    db_event_route_result_t result = db_event_route_get(uuid, &route);
    if (result != DB_EVENT_ROUTE_OK) {
        set_db_error(res, result, NULL);
        return;
    }
    cJSON *body = httpd_parse_json_body(req);
    int64_t expected_revision = 0;
    if (!apply_route_body(body, &route, false, &expected_revision, res)) {
        cJSON_Delete(body);
        audit_route(req, &user, uuid, "route_update", "failure",
                    "invalid_request");
        return;
    }
    cJSON_Delete(body);
    result = db_event_route_update(&route, expected_revision);
    if (result != DB_EVENT_ROUTE_OK) {
        set_db_error(res, result, NULL);
        audit_route(req, &user, uuid, "route_update",
                    result == DB_EVENT_ROUTE_ERROR ? "error" : "failure",
                    result == DB_EVENT_ROUTE_STALE ? "stale_revision"
                                                   : "persistence_failed");
        return;
    }
    audit_route(req, &user, uuid, "route_update", "success", "updated");
    set_json_response(res, 200, route_to_json(&route));
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

void handle_delete_event_route(const http_request_t *req,
                               http_response_t *res) {
    user_t user;
    if (!authorize_events(req, res, &user)) return;
    char uuid[EVENT_ROUTE_UUID_MAX];
    int64_t revision = 0;
    if (!extract_route_uuid(req, uuid, res) ||
        !parse_delete_revision(req, &revision, res)) {
        return;
    }
    db_event_route_result_t result = db_event_route_delete(uuid, revision);
    if (result != DB_EVENT_ROUTE_OK) {
        set_db_error(res, result, NULL);
        audit_route(req, &user, uuid, "route_delete",
                    result == DB_EVENT_ROUTE_ERROR ? "error" : "failure",
                    result == DB_EVENT_ROUTE_STALE ? "stale_revision"
                                                   : "persistence_failed");
        return;
    }
    audit_route(req, &user, uuid, "route_delete", "success", "deleted");
    http_response_set_json(res, 200, "{\"success\":true}");
}

void handle_post_event_route_preview(const http_request_t *req,
                                     http_response_t *res) {
    user_t user;
    if (!authorize_events(req, res, &user)) return;
    cJSON *body = httpd_parse_json_body(req);
    event_route_t route;
    initialize_route_defaults(&route);
    int64_t unused_revision = 0;
    if (!apply_route_body(body, &route, true, &unused_revision, res)) {
        cJSON_Delete(body);
        return;
    }
    cJSON_Delete(body);

    fleet_camera_t *cameras = NULL;
    int camera_count = 0;
    if (db_fleet_camera_load(&cameras, &camera_count) != 0) {
        http_response_set_json_error(res, 500,
                                     "Failed to load fleet for route preview");
        return;
    }
    fleet_camera_enrich_runtime_health(cameras, camera_count);
    fleet_selector_t *selector = NULL;
    if (strcmp(route.scope_type, "selector") == 0) {
        cJSON *selector_json = cJSON_Parse(route.selector_json);
        char error[FLEET_SELECTOR_ERROR_MAX] = {0};
        selector = fleet_selector_parse(selector_json, error, sizeof(error));
        cJSON_Delete(selector_json);
        if (!selector) {
            free(cameras);
            http_response_set_json_error(res, 400,
                                         error[0] ? error : "Invalid selector");
            return;
        }
    }
    cJSON *root = cJSON_CreateObject();
    cJSON *sample = cJSON_CreateArray();
    cJSON *types = cJSON_CreateArray();
    if (!root || !sample || !types) {
        cJSON_Delete(root);
        cJSON_Delete(sample);
        cJSON_Delete(types);
        fleet_selector_free(selector);
        free(cameras);
        http_response_set_json_error(res, 500,
                                     "Failed to create preview response");
        return;
    }
    int matched_count = 0;
    for (int index = 0; index < camera_count; index++) {
        bool matches = !selector ||
            fleet_selector_matches(selector, &cameras[index], NULL);
        if (!matches) continue;
        if (matched_count < EVENT_ROUTE_PREVIEW_MAX_CAMERAS) {
            cJSON *item = cJSON_CreateObject();
            cJSON_AddStringToObject(item, "camera_uuid",
                                    cameras[index].camera_uuid);
            cJSON_AddStringToObject(item, "name", cameras[index].name);
            cJSON_AddStringToObject(item, "location_path",
                                    cameras[index].location_path);
            cJSON_AddItemToArray(sample, item);
        }
        matched_count++;
    }
    for (int index = 0; index < route.event_type_count; index++) {
        const event_type_definition_t *definition =
            event_registry_find(route.event_types[index]);
        cJSON *item = cJSON_CreateObject();
        cJSON_AddStringToObject(item, "type", definition->type);
        cJSON_AddStringToObject(item, "expected_rate",
                                event_expected_rate_name(
                                    definition->expected_rate));
        cJSON_AddStringToObject(item, "severity",
                                event_severity_name(definition->severity));
        cJSON_AddItemToArray(types, item);
    }
    cJSON_AddNumberToObject(root, "matched_camera_count", matched_count);
    cJSON_AddNumberToObject(root, "sample_limit",
                            EVENT_ROUTE_PREVIEW_MAX_CAMERAS);
    cJSON_AddItemToObject(root, "camera_sample", sample);
    cJSON_AddItemToObject(root, "event_types", types);
    cJSON_AddBoolToObject(root, "would_publish", false);
    fleet_selector_free(selector);
    free(cameras);
    set_json_response(res, 200, root);
}
