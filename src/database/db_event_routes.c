#define _POSIX_C_SOURCE 200809L

#include "database/db_event_routes.h"

#include <cjson/cJSON.h>
#include <ctype.h>
#include <pthread.h>
#include <sqlite3.h>
#include <stdarg.h>
#include <stdatomic.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>
#include <sys/stat.h>

#include "core/camera_selector.h"
#include "database/db_core.h"
#include "database/db_event_destinations.h"
#include "telemetry/system_health_types.h"
#include "utils/strings.h"

#define EVENT_ROUTE_SELECT_FIELDS \
    "uuid,name,description,enabled,destination_key,scope_type," \
    "selector_json,predicate_json,schedule_json,debounce_seconds," \
    "cooldown_seconds,grouping_window_seconds,max_events_per_minute," \
    "revision,created_at,updated_at"

static atomic_uint_fast64_t ROUTE_GENERATION = 1;

static void set_error(char *error, size_t error_size, const char *format, ...) {
    if (!error || error_size == 0 || error[0] != '\0') return;
    va_list arguments;
    va_start(arguments, format);
    vsnprintf(error, error_size, format, arguments);
    va_end(arguments);
}

static bool valid_uuid(const char *value) {
    if (!value || strlen(value) != EVENT_ROUTE_UUID_MAX - 1) return false;
    for (int index = 0; index < EVENT_ROUTE_UUID_MAX - 1; index++) {
        unsigned char character = (unsigned char)value[index];
        if (index == 8 || index == 13 || index == 18 || index == 23) {
            if (character != '-') return false;
        } else if (!isxdigit(character)) {
            return false;
        }
    }
    return true;
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

static bool key_allowed(const char *key, const char *const *allowed,
                        size_t allowed_count) {
    if (!key) return false;
    for (size_t index = 0; index < allowed_count; index++) {
        if (strcmp(key, allowed[index]) == 0) return true;
    }
    return false;
}

static bool validate_object_keys(const cJSON *object,
                                 const char *const *allowed,
                                 size_t allowed_count, const char *context,
                                 char *error, size_t error_size) {
    for (const cJSON *item = object ? object->child : NULL; item;
         item = item->next) {
        if (!key_allowed(item->string, allowed, allowed_count)) {
            set_error(error, error_size, "%s contains unknown field '%s'",
                      context, item->string ? item->string : "");
            return false;
        }
    }
    return true;
}

static bool version_is_one(const cJSON *object, const char *context,
                           char *error, size_t error_size) {
    const cJSON *version = cJSON_GetObjectItemCaseSensitive(object, "version");
    if (!cJSON_IsNumber(version) || version->valuedouble != 1.0) {
        set_error(error, error_size, "%s.version must be 1", context);
        return false;
    }
    return true;
}

static bool valid_string_array(const cJSON *array, const char *field,
                               char *error, size_t error_size) {
    int count = cJSON_IsArray(array) ? cJSON_GetArraySize(array) : -1;
    if (count < 1 || count > 64) {
        set_error(error, error_size, "%s must contain 1-64 values", field);
        return false;
    }
    for (int index = 0; index < count; index++) {
        const cJSON *item = cJSON_GetArrayItem(array, index);
        if (!cJSON_IsString(item) ||
            !valid_text(item->valuestring, 128, true)) {
            set_error(error, error_size, "%s contains an invalid value", field);
            return false;
        }
        for (int previous = 0; previous < index; previous++) {
            const cJSON *other = cJSON_GetArrayItem(array, previous);
            if (strcasecmp(item->valuestring, other->valuestring) == 0) {
                set_error(error, error_size, "%s contains a duplicate value",
                          field);
                return false;
            }
        }
    }
    return true;
}

static bool route_has_type(const event_route_t *route, const char *type) {
    for (int index = 0; index < route->event_type_count; index++) {
        if (strcmp(route->event_types[index], type) == 0) return true;
    }
    return false;
}

static bool route_types_are_family(const event_route_t *route,
                                   const char *family) {
    if (!route || !family || route->event_type_count < 1) return false;
    for (int index = 0; index < route->event_type_count; index++) {
        const event_type_definition_t *definition =
            event_registry_find(route->event_types[index]);
        if (!definition || strcmp(definition->family, family) != 0) {
            return false;
        }
    }
    return true;
}

static bool route_types_are_camera_subjects(const event_route_t *route) {
    if (!route || route->event_type_count < 1) return false;
    for (int index = 0; index < route->event_type_count; index++) {
        const event_type_definition_t *definition =
            event_registry_find(route->event_types[index]);
        if (!definition || definition->subject_kind != EVENT_SUBJECT_CAMERA) {
            return false;
        }
    }
    return true;
}

static bool valid_health_severity(const char *value) {
    return value && (strcmp(value, "warning") == 0 ||
                     strcmp(value, "error") == 0 ||
                     strcmp(value, "critical") == 0);
}

static bool valid_health_condition_array(const cJSON *array, char *error,
                                         size_t error_size) {
    const char *field = "predicate.health.condition_codes_any";
    if (!valid_string_array(array, field, error, error_size)) return false;
    int count = cJSON_GetArraySize(array);
    for (int index = 0; index < count; index++) {
        system_health_condition_t condition;
        const cJSON *item = cJSON_GetArrayItem(array, index);
        if (!system_health_condition_from_code(item->valuestring,
                                               &condition)) {
            set_error(error, error_size, "%s contains unknown value '%s'",
                      field, item->valuestring);
            return false;
        }
    }
    return true;
}

static bool valid_health_severity_array(const cJSON *array, char *error,
                                        size_t error_size) {
    const char *field = "predicate.health.severities_any";
    if (!valid_string_array(array, field, error, error_size)) return false;
    int count = cJSON_GetArraySize(array);
    for (int index = 0; index < count; index++) {
        const cJSON *item = cJSON_GetArrayItem(array, index);
        if (!valid_health_severity(item->valuestring)) {
            set_error(error, error_size, "%s contains unknown value '%s'",
                      field, item->valuestring);
            return false;
        }
    }
    return true;
}

static bool valid_predicate(const event_route_t *route, char *error,
                            size_t error_size) {
    cJSON *root = cJSON_Parse(route->predicate_json);
    const char *const root_fields[] = {"version", "detection", "health"};
    if (!cJSON_IsObject(root) ||
        !validate_object_keys(root, root_fields, 3, "predicate", error,
                              error_size) ||
        !version_is_one(root, "predicate", error, error_size)) {
        cJSON_Delete(root);
        if (error && error_size > 0 && error[0] == '\0') {
            set_error(error, error_size, "predicate must be a JSON object");
        }
        return false;
    }
    const cJSON *detection =
        cJSON_GetObjectItemCaseSensitive(root, "detection");
    bool valid = true;
    if (detection) {
        const char *const detection_fields[] = {
            "labels_any", "min_confidence", "zone_ids_any"
        };
        if (!cJSON_IsObject(detection) ||
            !route_has_type(route, "io.lightnvr.detection.object.v1") ||
            !validate_object_keys(detection, detection_fields, 3,
                                  "predicate.detection", error, error_size)) {
            if (cJSON_IsObject(detection) &&
                !route_has_type(route, "io.lightnvr.detection.object.v1")) {
                set_error(error, error_size,
                          "detection predicate requires the detection event type");
            } else if (!cJSON_IsObject(detection)) {
                set_error(error, error_size,
                          "predicate.detection must be an object");
            }
            cJSON_Delete(root);
            return false;
        }
        const cJSON *labels =
            cJSON_GetObjectItemCaseSensitive(detection, "labels_any");
        const cJSON *confidence =
            cJSON_GetObjectItemCaseSensitive(detection, "min_confidence");
        const cJSON *zones =
            cJSON_GetObjectItemCaseSensitive(detection, "zone_ids_any");
        if (!labels && !confidence && !zones) {
            set_error(error, error_size,
                      "predicate.detection requires at least one filter");
            cJSON_Delete(root);
            return false;
        }
        valid = (!labels || valid_string_array(
                          labels, "predicate.detection.labels_any", error,
                          error_size)) &&
            (!zones || valid_string_array(
                           zones, "predicate.detection.zone_ids_any", error,
                           error_size));
        if (valid && confidence &&
            (!cJSON_IsNumber(confidence) || confidence->valuedouble < 0.0 ||
             confidence->valuedouble > 1.0)) {
            set_error(error, error_size,
                      "predicate.detection.min_confidence must be from 0 to 1");
            valid = false;
        }
    }
    const cJSON *health = cJSON_GetObjectItemCaseSensitive(root, "health");
    if (valid && health) {
        const char *const health_fields[] = {
            "condition_codes_any", "severities_any"
        };
        if (!cJSON_IsObject(health)) {
            set_error(error, error_size, "predicate.health must be an object");
            valid = false;
        } else if (!route_types_are_family(route, "system_health")) {
            set_error(error, error_size,
                      "health predicate requires only system health event types");
            valid = false;
        } else if (!validate_object_keys(health, health_fields, 2,
                                         "predicate.health", error,
                                         error_size)) {
            valid = false;
        } else {
            const cJSON *conditions = cJSON_GetObjectItemCaseSensitive(
                health, "condition_codes_any");
            const cJSON *severities = cJSON_GetObjectItemCaseSensitive(
                health, "severities_any");
            if (!conditions && !severities) {
                set_error(error, error_size,
                          "predicate.health requires at least one filter");
                valid = false;
            } else {
                valid = (!conditions || valid_health_condition_array(
                            conditions, error, error_size)) &&
                    (!severities || valid_health_severity_array(
                                      severities, error, error_size));
            }
        }
    }
    if (valid && detection && health) {
        set_error(error, error_size,
                  "predicate cannot combine detection and health filters");
        valid = false;
    }
    cJSON_Delete(root);
    return valid;
}

static bool valid_timezone(const char *timezone) {
    if (!valid_text(timezone, 65, true) || timezone[0] == '/' ||
        strstr(timezone, "//") || timezone[strlen(timezone) - 1] == '/') {
        return false;
    }
    for (const unsigned char *cursor = (const unsigned char *)timezone;
         *cursor; cursor++) {
        if (!isalnum(*cursor) && *cursor != '/' && *cursor != '_' &&
            *cursor != '-' && *cursor != '+') {
            return false;
        }
    }
    return true;
}

static bool timezone_available(const char *timezone) {
    if (!valid_timezone(timezone)) return false;
    if (strcmp(timezone, "UTC") == 0 || strcmp(timezone, "GMT") == 0) {
        return true;
    }
    char path[256];
    int length = snprintf(path, sizeof(path), "/usr/share/zoneinfo/%s",
                          timezone);
    if (length < 0 || (size_t)length >= sizeof(path)) return false;
    struct stat metadata;
    return stat(path, &metadata) == 0 && S_ISREG(metadata.st_mode);
}

static bool valid_clock_time(const char *value) {
    if (!value || strlen(value) != 5 || value[2] != ':' ||
        !isdigit((unsigned char)value[0]) ||
        !isdigit((unsigned char)value[1]) ||
        !isdigit((unsigned char)value[3]) ||
        !isdigit((unsigned char)value[4])) {
        return false;
    }
    int hour = (value[0] - '0') * 10 + value[1] - '0';
    int minute = (value[3] - '0') * 10 + value[4] - '0';
    return hour >= 0 && hour <= 23 && minute >= 0 && minute <= 59;
}

static bool valid_window(const cJSON *window, char *error,
                         size_t error_size) {
    const char *const fields[] = {"days", "start", "end"};
    if (!cJSON_IsObject(window) ||
        !validate_object_keys(window, fields, 3, "schedule window", error,
                              error_size)) {
        set_error(error, error_size, "schedule windows must be objects");
        return false;
    }
    const cJSON *days = cJSON_GetObjectItemCaseSensitive(window, "days");
    const cJSON *start = cJSON_GetObjectItemCaseSensitive(window, "start");
    const cJSON *end = cJSON_GetObjectItemCaseSensitive(window, "end");
    int day_count = cJSON_IsArray(days) ? cJSON_GetArraySize(days) : -1;
    if (day_count < 1 || day_count > 7 || !cJSON_IsString(start) ||
        !cJSON_IsString(end) || !valid_clock_time(start->valuestring) ||
        !valid_clock_time(end->valuestring) ||
        strcmp(start->valuestring, end->valuestring) == 0) {
        set_error(error, error_size,
                  "schedule window requires days 0-6 and distinct HH:MM bounds");
        return false;
    }
    bool seen[7] = {false};
    for (int index = 0; index < day_count; index++) {
        const cJSON *day = cJSON_GetArrayItem(days, index);
        if (!cJSON_IsNumber(day) || day->valuedouble != day->valueint ||
            day->valueint < 0 || day->valueint > 6 || seen[day->valueint]) {
            set_error(error, error_size,
                      "schedule window days must be unique integers from 0 to 6");
            return false;
        }
        seen[day->valueint] = true;
    }
    return true;
}

static bool valid_schedule(const event_route_t *route, char *error,
                           size_t error_size) {
    cJSON *root = cJSON_Parse(route->schedule_json);
    const char *const fields[] = {"version", "timezone", "windows"};
    if (!cJSON_IsObject(root) ||
        !validate_object_keys(root, fields, 3, "schedule", error,
                              error_size) ||
        !version_is_one(root, "schedule", error, error_size)) {
        cJSON_Delete(root);
        set_error(error, error_size, "schedule must be a JSON object");
        return false;
    }
    const cJSON *timezone =
        cJSON_GetObjectItemCaseSensitive(root, "timezone");
    const cJSON *windows =
        cJSON_GetObjectItemCaseSensitive(root, "windows");
    int count = cJSON_IsArray(windows) ? cJSON_GetArraySize(windows) : -1;
    if (!cJSON_IsString(timezone) ||
        !timezone_available(timezone->valuestring) ||
        count < 0 || count > 64) {
        set_error(error, error_size,
                  "schedule requires an available timezone and at most 64 windows");
        cJSON_Delete(root);
        return false;
    }
    for (int index = 0; index < count; index++) {
        if (!valid_window(cJSON_GetArrayItem(windows, index), error,
                          error_size)) {
            cJSON_Delete(root);
            return false;
        }
    }
    cJSON_Delete(root);
    return true;
}

db_event_route_result_t db_event_route_validate(
    const event_route_t *route, char *error, size_t error_size) {
    if (error && error_size > 0) error[0] = '\0';
    if (!route || !valid_text(route->name, sizeof(route->name), true) ||
        !valid_text(route->description, sizeof(route->description), false)) {
        set_error(error, error_size, "route name or description is invalid");
        return DB_EVENT_ROUTE_INVALID;
    }
    char normalized_name[EVENT_ROUTE_NAME_MAX];
    if (copy_trimmed_value(normalized_name, sizeof(normalized_name),
                           route->name, 0) == 0) {
        set_error(error, error_size, "route name cannot be blank");
        return DB_EVENT_ROUTE_INVALID;
    }
    if (strcmp(route->destination_key, EVENT_ROUTE_DEFAULT_DESTINATION) != 0 &&
        !db_event_destination_key_exists(route->destination_key)) {
        set_error(error, error_size,
                  "destination must be mqtt:default or an existing managed "
                  "MQTT destination");
        return DB_EVENT_ROUTE_INVALID;
    }
    if (route->event_type_count < 1 ||
        route->event_type_count > EVENT_ROUTE_MAX_TYPES) {
        set_error(error, error_size, "route requires 1-%d event types",
                  EVENT_ROUTE_MAX_TYPES);
        return DB_EVENT_ROUTE_INVALID;
    }
    for (int index = 0; index < route->event_type_count; index++) {
        if (!event_registry_find(route->event_types[index])) {
            set_error(error, error_size, "route contains an unknown event type");
            return DB_EVENT_ROUTE_INVALID;
        }
        for (int previous = 0; previous < index; previous++) {
            if (strcmp(route->event_types[index],
                       route->event_types[previous]) == 0) {
                set_error(error, error_size,
                          "route contains a duplicate event type");
                return DB_EVENT_ROUTE_INVALID;
            }
        }
    }
    if (strcmp(route->scope_type, "all") == 0) {
        if (route->selector_json[0] != '\0') {
            set_error(error, error_size,
                      "all scope cannot include a camera selector");
            return DB_EVENT_ROUTE_INVALID;
        }
    } else if (strcmp(route->scope_type, "selector") == 0) {
        if (!route_types_are_camera_subjects(route)) {
            set_error(error, error_size,
                      "camera selector requires only camera event types");
            return DB_EVENT_ROUTE_INVALID;
        }
        cJSON *json = cJSON_Parse(route->selector_json);
        char selector_error[FLEET_SELECTOR_ERROR_MAX] = {0};
        fleet_selector_t *selector =
            fleet_selector_parse(json, selector_error, sizeof(selector_error));
        cJSON_Delete(json);
        if (!selector) {
            set_error(error, error_size, "%s",
                      selector_error[0] ? selector_error
                                        : "camera selector is invalid");
            return DB_EVENT_ROUTE_INVALID;
        }
        fleet_selector_free(selector);
    } else {
        set_error(error, error_size, "camera scope must be all or selector");
        return DB_EVENT_ROUTE_INVALID;
    }
    if (!valid_predicate(route, error, error_size) ||
        !valid_schedule(route, error, error_size)) {
        return DB_EVENT_ROUTE_INVALID;
    }
    if (route->debounce_seconds < 0 || route->debounce_seconds > 86400 ||
        route->cooldown_seconds < 0 ||
        route->cooldown_seconds > 604800 ||
        route->grouping_window_seconds < 0 ||
        route->grouping_window_seconds > 3600 ||
        route->max_events_per_minute < 0 ||
        route->max_events_per_minute > 60000) {
        set_error(error, error_size, "route suppression value is out of range");
        return DB_EVENT_ROUTE_INVALID;
    }
    return DB_EVENT_ROUTE_OK;
}

static bool route_transaction_begin(sqlite3 *db) {
    return sqlite3_exec(db, "BEGIN IMMEDIATE;", NULL, NULL, NULL) == SQLITE_OK;
}

static bool route_transaction_finish(sqlite3 *db, bool success) {
    if (success && sqlite3_exec(db, "COMMIT;", NULL, NULL, NULL) == SQLITE_OK) {
        return true;
    }
    sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
    return false;
}

static void copy_column(char *destination, size_t destination_size,
                        sqlite3_stmt *statement, int column) {
    const char *value = (const char *)sqlite3_column_text(statement, column);
    safe_strcpy(destination, value ? value : "", destination_size, 0);
}

static void populate_route(sqlite3_stmt *statement, event_route_t *route) {
    memset(route, 0, sizeof(*route));
    copy_column(route->uuid, sizeof(route->uuid), statement, 0);
    copy_column(route->name, sizeof(route->name), statement, 1);
    copy_column(route->description, sizeof(route->description), statement, 2);
    route->enabled = sqlite3_column_int(statement, 3) != 0;
    copy_column(route->destination_key, sizeof(route->destination_key),
                statement, 4);
    copy_column(route->scope_type, sizeof(route->scope_type), statement, 5);
    copy_column(route->selector_json, sizeof(route->selector_json), statement, 6);
    copy_column(route->predicate_json, sizeof(route->predicate_json), statement,
                7);
    copy_column(route->schedule_json, sizeof(route->schedule_json), statement, 8);
    route->debounce_seconds = sqlite3_column_int(statement, 9);
    route->cooldown_seconds = sqlite3_column_int(statement, 10);
    route->grouping_window_seconds = sqlite3_column_int(statement, 11);
    route->max_events_per_minute = sqlite3_column_int(statement, 12);
    route->revision = sqlite3_column_int64(statement, 13);
    route->created_at = sqlite3_column_int64(statement, 14);
    route->updated_at = sqlite3_column_int64(statement, 15);
}

static int load_types_locked(sqlite3 *db, event_route_t *route) {
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(
        db, "SELECT event_type FROM event_route_types WHERE route_uuid=? "
            "ORDER BY event_type;", -1, &statement, NULL);
    if (result != SQLITE_OK) return -1;
    sqlite3_bind_text(statement, 1, route->uuid, -1, SQLITE_TRANSIENT);
    int count = 0;
    while ((result = sqlite3_step(statement)) == SQLITE_ROW &&
           count < EVENT_ROUTE_MAX_TYPES) {
        copy_column(route->event_types[count], EVENT_TYPE_MAX, statement, 0);
        count++;
    }
    if (statement) sqlite3_finalize(statement);
    if (result != SQLITE_DONE) return -1;
    route->event_type_count = count;
    return count > 0 ? 0 : -1;
}

static db_event_route_result_t get_locked(sqlite3 *db, const char *uuid,
                                          event_route_t *route) {
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(
        db, "SELECT " EVENT_ROUTE_SELECT_FIELDS
            " FROM event_routes WHERE uuid=? LIMIT 1;", -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, uuid, -1, SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    if (result == SQLITE_DONE) {
        sqlite3_finalize(statement);
        return DB_EVENT_ROUTE_NOT_FOUND;
    }
    if (result != SQLITE_ROW) {
        if (statement) sqlite3_finalize(statement);
        return DB_EVENT_ROUTE_ERROR;
    }
    populate_route(statement, route);
    sqlite3_finalize(statement);
    return load_types_locked(db, route) == 0 ? DB_EVENT_ROUTE_OK
                                             : DB_EVENT_ROUTE_ERROR;
}

static int count_locked(sqlite3 *db) {
    sqlite3_stmt *statement = NULL;
    int count = -1;
    if (sqlite3_prepare_v2(db, "SELECT count(*) FROM event_routes;", -1,
                           &statement, NULL) == SQLITE_OK &&
        sqlite3_step(statement) == SQLITE_ROW) {
        count = sqlite3_column_int(statement, 0);
    }
    if (statement) sqlite3_finalize(statement);
    return count;
}

int db_event_route_count(void) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;
    pthread_mutex_lock(mutex);
    int count = count_locked(db);
    pthread_mutex_unlock(mutex);
    return count;
}

int db_event_route_list(event_route_t *routes, int max_count) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex || !routes || max_count <= 0 ||
        max_count > EVENT_ROUTE_MAX_COUNT) {
        return -1;
    }
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(
        db, "SELECT " EVENT_ROUTE_SELECT_FIELDS
            " FROM event_routes ORDER BY name COLLATE NOCASE,uuid;", -1,
        &statement, NULL);
    if (result != SQLITE_OK) {
        pthread_mutex_unlock(mutex);
        return -1;
    }
    int count = 0;
    while (count < max_count &&
           (result = sqlite3_step(statement)) == SQLITE_ROW) {
        populate_route(statement, &routes[count++]);
    }
    if (statement) sqlite3_finalize(statement);
    if (result != SQLITE_DONE && result != SQLITE_ROW) count = -1;
    for (int index = 0; count >= 0 && index < count; index++) {
        if (load_types_locked(db, &routes[index]) != 0) count = -1;
    }
    pthread_mutex_unlock(mutex);
    return count;
}

uint64_t db_event_route_generation(void) {
    return atomic_load_explicit(&ROUTE_GENERATION, memory_order_relaxed);
}

db_event_route_result_t db_event_route_get(const char *uuid,
                                           event_route_t *route) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex || !route || !valid_uuid(uuid)) {
        return DB_EVENT_ROUTE_INVALID;
    }
    pthread_mutex_lock(mutex);
    db_event_route_result_t result = get_locked(db, uuid, route);
    pthread_mutex_unlock(mutex);
    return result;
}

static int insert_types_locked(sqlite3 *db, const event_route_t *route) {
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(
        db, "INSERT INTO event_route_types(route_uuid,event_type) VALUES(?,?);",
        -1, &statement, NULL);
    for (int index = 0; result == SQLITE_OK &&
         index < route->event_type_count; index++) {
        sqlite3_reset(statement);
        sqlite3_clear_bindings(statement);
        sqlite3_bind_text(statement, 1, route->uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 2, route->event_types[index], -1,
                          SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
        if (result == SQLITE_DONE) result = SQLITE_OK;
    }
    if (statement) sqlite3_finalize(statement);
    return result == SQLITE_OK ? 0 : -1;
}

static int clear_suppression_locked(sqlite3 *db, const char *route_uuid) {
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(
        db, "DELETE FROM event_route_suppression_state WHERE route_uuid=?;",
        -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, route_uuid, -1, SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    if (statement) sqlite3_finalize(statement);
    return result == SQLITE_DONE ? 0 : -1;
}

static void bind_route_fields(sqlite3_stmt *statement,
                              const event_route_t *route,
                              const char *normalized_name) {
    sqlite3_bind_text(statement, 1, normalized_name, -1, SQLITE_TRANSIENT);
    sqlite3_bind_text(statement, 2, route->description, -1, SQLITE_TRANSIENT);
    sqlite3_bind_int(statement, 3, route->enabled ? 1 : 0);
    sqlite3_bind_text(statement, 4, route->destination_key, -1,
                      SQLITE_TRANSIENT);
    sqlite3_bind_text(statement, 5, route->scope_type, -1, SQLITE_TRANSIENT);
    if (strcmp(route->scope_type, "selector") == 0) {
        sqlite3_bind_text(statement, 6, route->selector_json, -1,
                          SQLITE_TRANSIENT);
    } else {
        sqlite3_bind_null(statement, 6);
    }
    sqlite3_bind_text(statement, 7, route->predicate_json, -1,
                      SQLITE_TRANSIENT);
    sqlite3_bind_text(statement, 8, route->schedule_json, -1,
                      SQLITE_TRANSIENT);
    sqlite3_bind_int(statement, 9, route->debounce_seconds);
    sqlite3_bind_int(statement, 10, route->cooldown_seconds);
    sqlite3_bind_int(statement, 11, route->grouping_window_seconds);
    sqlite3_bind_int(statement, 12, route->max_events_per_minute);
}

db_event_route_result_t db_event_route_create(event_route_t *route) {
    char error[EVENT_ROUTE_VALIDATION_ERROR_MAX] = {0};
    if (db_event_route_validate(route, error, sizeof(error)) !=
        DB_EVENT_ROUTE_OK) {
        return DB_EVENT_ROUTE_INVALID;
    }
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return DB_EVENT_ROUTE_ERROR;
    char normalized_name[EVENT_ROUTE_NAME_MAX];
    copy_trimmed_value(normalized_name, sizeof(normalized_name), route->name, 0);

    pthread_mutex_lock(mutex);
    if (!route_transaction_begin(db)) {
        pthread_mutex_unlock(mutex);
        return DB_EVENT_ROUTE_ERROR;
    }
    if (count_locked(db) >= EVENT_ROUTE_MAX_COUNT) {
        route_transaction_finish(db, false);
        pthread_mutex_unlock(mutex);
        return DB_EVENT_ROUTE_LIMIT;
    }
    const char *sql =
        "INSERT INTO event_routes(uuid,name,description,enabled,destination_key,"
        "scope_type,selector_json,predicate_json,schedule_json,"
        "debounce_seconds,cooldown_seconds,grouping_window_seconds,"
        "max_events_per_minute) VALUES("
        "lower(hex(randomblob(4))||'-'||hex(randomblob(2))||'-4'||"
        "substr(hex(randomblob(2)),2)||'-'||"
        "substr('89ab',(abs(random())%4)+1,1)||"
        "substr(hex(randomblob(2)),2)||'-'||hex(randomblob(6))),"
        "?,?,?,?,?,?,?,?,?,?,?,?);";
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        bind_route_fields(statement, route, normalized_name);
        result = sqlite3_step(statement);
    }
    if (statement) sqlite3_finalize(statement);
    if (result != SQLITE_DONE) {
        route_transaction_finish(db, false);
        pthread_mutex_unlock(mutex);
        return (result & 0xff) == SQLITE_CONSTRAINT
            ? DB_EVENT_ROUTE_CONFLICT : DB_EVENT_ROUTE_ERROR;
    }
    statement = NULL;
    result = sqlite3_prepare_v2(
        db, "SELECT uuid FROM event_routes WHERE id=last_insert_rowid();", -1,
        &statement, NULL);
    if (result == SQLITE_OK && sqlite3_step(statement) == SQLITE_ROW) {
        copy_column(route->uuid, sizeof(route->uuid), statement, 0);
        result = SQLITE_OK;
    } else {
        result = SQLITE_ERROR;
    }
    if (statement) sqlite3_finalize(statement);
    bool ready_to_commit = result == SQLITE_OK &&
        insert_types_locked(db, route) == 0;
    if (!route_transaction_finish(db, ready_to_commit)) {
        pthread_mutex_unlock(mutex);
        return DB_EVENT_ROUTE_ERROR;
    }
    atomic_fetch_add_explicit(&ROUTE_GENERATION, 1, memory_order_relaxed);
    db_event_route_result_t route_result = get_locked(db, route->uuid, route);
    pthread_mutex_unlock(mutex);
    return route_result;
}

db_event_route_result_t db_event_route_update(event_route_t *route,
                                              int64_t expected_revision) {
    char error[EVENT_ROUTE_VALIDATION_ERROR_MAX] = {0};
    if (!route || !valid_uuid(route->uuid) || expected_revision < 1 ||
        db_event_route_validate(route, error, sizeof(error)) !=
            DB_EVENT_ROUTE_OK) {
        return DB_EVENT_ROUTE_INVALID;
    }
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return DB_EVENT_ROUTE_ERROR;
    char normalized_name[EVENT_ROUTE_NAME_MAX];
    copy_trimmed_value(normalized_name, sizeof(normalized_name), route->name, 0);

    pthread_mutex_lock(mutex);
    if (!route_transaction_begin(db)) {
        pthread_mutex_unlock(mutex);
        return DB_EVENT_ROUTE_ERROR;
    }
    event_route_t existing;
    db_event_route_result_t route_result = get_locked(db, route->uuid, &existing);
    if (route_result != DB_EVENT_ROUTE_OK) {
        route_transaction_finish(db, false);
        pthread_mutex_unlock(mutex);
        return route_result;
    }
    if (existing.revision != expected_revision) {
        route_transaction_finish(db, false);
        pthread_mutex_unlock(mutex);
        return DB_EVENT_ROUTE_STALE;
    }
    const char *sql =
        "UPDATE event_routes SET name=?,description=?,enabled=?,"
        "destination_key=?,scope_type=?,selector_json=?,predicate_json=?,"
        "schedule_json=?,debounce_seconds=?,cooldown_seconds=?,"
        "grouping_window_seconds=?,max_events_per_minute=?,"
        "revision=revision+1,updated_at=strftime('%s','now') "
        "WHERE uuid=? AND revision=?;";
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        bind_route_fields(statement, route, normalized_name);
        sqlite3_bind_text(statement, 13, route->uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int64(statement, 14, expected_revision);
        result = sqlite3_step(statement);
    }
    int changed = result == SQLITE_DONE ? sqlite3_changes(db) : 0;
    if (statement) sqlite3_finalize(statement);
    if (result != SQLITE_DONE || changed != 1) {
        route_transaction_finish(db, false);
        pthread_mutex_unlock(mutex);
        return (result & 0xff) == SQLITE_CONSTRAINT
            ? DB_EVENT_ROUTE_CONFLICT : DB_EVENT_ROUTE_ERROR;
    }
    statement = NULL;
    result = sqlite3_prepare_v2(
        db, "DELETE FROM event_route_types WHERE route_uuid=?;", -1,
        &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, route->uuid, -1, SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    if (statement) sqlite3_finalize(statement);
    bool ready_to_commit = result == SQLITE_DONE &&
        clear_suppression_locked(db, route->uuid) == 0 &&
        insert_types_locked(db, route) == 0;
    if (!route_transaction_finish(db, ready_to_commit)) {
        pthread_mutex_unlock(mutex);
        return DB_EVENT_ROUTE_ERROR;
    }
    atomic_fetch_add_explicit(&ROUTE_GENERATION, 1, memory_order_relaxed);
    route_result = get_locked(db, route->uuid, route);
    pthread_mutex_unlock(mutex);
    return route_result;
}

db_event_route_result_t db_event_route_delete(const char *uuid,
                                              int64_t expected_revision) {
    if (!valid_uuid(uuid) || expected_revision < 1) {
        return DB_EVENT_ROUTE_INVALID;
    }
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return DB_EVENT_ROUTE_ERROR;
    pthread_mutex_lock(mutex);
    if (!route_transaction_begin(db)) {
        pthread_mutex_unlock(mutex);
        return DB_EVENT_ROUTE_ERROR;
    }
    event_route_t existing;
    db_event_route_result_t result = get_locked(db, uuid, &existing);
    if (result != DB_EVENT_ROUTE_OK) {
        route_transaction_finish(db, false);
        pthread_mutex_unlock(mutex);
        return result;
    }
    if (existing.revision != expected_revision) {
        route_transaction_finish(db, false);
        pthread_mutex_unlock(mutex);
        return DB_EVENT_ROUTE_STALE;
    }
    sqlite3_stmt *statement = NULL;
    int sqlite_result = sqlite3_prepare_v2(
        db, "DELETE FROM event_routes WHERE uuid=? AND revision=?;", -1,
        &statement, NULL);
    if (sqlite_result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int64(statement, 2, expected_revision);
        sqlite_result = sqlite3_step(statement);
    }
    int changed = sqlite_result == SQLITE_DONE ? sqlite3_changes(db) : 0;
    if (statement) sqlite3_finalize(statement);
    bool committed = route_transaction_finish(
        db, sqlite_result == SQLITE_DONE && changed == 1);
    if (committed) {
        atomic_fetch_add_explicit(&ROUTE_GENERATION, 1,
                                  memory_order_relaxed);
    }
    pthread_mutex_unlock(mutex);
    return committed ? DB_EVENT_ROUTE_OK : DB_EVENT_ROUTE_ERROR;
}
