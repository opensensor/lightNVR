#define _POSIX_C_SOURCE 200809L

#include "core/event_router.h"

#include <cjson/cJSON.h>
#include <limits.h>
#include <pthread.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#include "core/camera_selector.h"
#include "core/logger.h"
#include "database/db_event_route_suppression.h"
#include "database/db_event_destinations.h"
#include "database/db_event_routes.h"
#include "database/db_fleet_query.h"
#include "utils/strings.h"
#include "utils/uuid.h"

#define DETECTION_EVENT_TYPE "io.lightnvr.detection.object.v1"
#define ROUTER_FILTER_MAX_VALUES 64
#define ROUTER_FILTER_VALUE_MAX 128
#define ROUTER_SCHEDULE_MAX_WINDOWS 64
#define ROUTER_INVENTORY_TTL_SECONDS 5
#define ROUTER_SUPPRESSION_PRUNE_INTERVAL_SECONDS (24LL * 60LL * 60LL)
#define ROUTER_SUPPRESSION_PRUNE_BATCH 1000
#define ROUTER_TZ_MAX_FILE_BYTES (1024U * 1024U)
#define ROUTER_TZ_MAX_TRANSITIONS 4096
#define ROUTER_TZ_MAX_TYPES 256

typedef struct {
    char labels[ROUTER_FILTER_MAX_VALUES][ROUTER_FILTER_VALUE_MAX];
    int label_count;
    char zones[ROUTER_FILTER_MAX_VALUES][ROUTER_FILTER_VALUE_MAX];
    int zone_count;
    bool has_min_confidence;
    double min_confidence;
} compiled_detection_predicate_t;

typedef struct {
    unsigned char days;
    int start_minute;
    int end_minute;
} compiled_schedule_window_t;

typedef struct {
    int64_t *transitions;
    unsigned char *transition_types;
    int transition_count;
    int32_t offsets[ROUTER_TZ_MAX_TYPES];
    unsigned char is_dst[ROUTER_TZ_MAX_TYPES];
    int type_count;
    int default_type;
    bool utc;
} compiled_timezone_t;

typedef struct {
    event_route_t route;
    event_destination_t destination;
    fleet_selector_t *selector;
    compiled_detection_predicate_t detection;
    compiled_schedule_window_t windows[ROUTER_SCHEDULE_MAX_WINDOWS];
    int window_count;
    compiled_timezone_t timezone;
    bool valid;
} compiled_route_t;

typedef struct {
    pthread_mutex_t mutex;
    compiled_route_t *routes;
    int route_count;
    int definition_count;
    uint64_t route_generation;
    uint64_t destination_generation;
    fleet_camera_t *inventory;
    int inventory_count;
    int64_t inventory_loaded_at;
    bool inventory_loaded;
    int64_t last_suppression_prune_at;
    event_router_stats_t stats;
} event_router_state_t;

static event_router_state_t ROUTER = {
    .mutex = PTHREAD_MUTEX_INITIALIZER,
};

static uint32_t read_be32(const unsigned char *input) {
    return ((uint32_t)input[0] << 24) | ((uint32_t)input[1] << 16) |
           ((uint32_t)input[2] << 8) | (uint32_t)input[3];
}

static int32_t read_be32_signed(const unsigned char *input) {
    uint32_t value = read_be32(input);
    if (value <= INT32_MAX) return (int32_t)value;
    uint32_t magnitude = (~value) + 1U;
    if (magnitude == 0x80000000U) return INT32_MIN;
    return -(int32_t)magnitude;
}

static int64_t read_be64_signed(const unsigned char *input) {
    uint64_t value = 0;
    for (int index = 0; index < 8; index++) {
        value = (value << 8) | input[index];
    }
    if (value <= INT64_MAX) return (int64_t)value;
    uint64_t magnitude = (~value) + 1U;
    if (magnitude == (UINT64_C(1) << 63)) return INT64_MIN;
    return -(int64_t)magnitude;
}

typedef struct {
    int ttisgmt_count;
    int ttisstd_count;
    int leap_count;
    int time_count;
    int type_count;
    int character_count;
} tzif_counts_t;

static bool parse_tzif_header(const unsigned char *bytes, size_t size,
                              size_t offset, tzif_counts_t *counts,
                              char *version) {
    if (!bytes || !counts || offset > size || size - offset < 44 ||
        memcmp(bytes + offset, "TZif", 4) != 0) {
        return false;
    }
    if (version) *version = (char)bytes[offset + 4];
    uint32_t raw[6];
    for (int index = 0; index < 6; index++) {
        raw[index] = read_be32(bytes + offset + 20 + (size_t)index * 4);
        if (raw[index] > INT_MAX) return false;
    }
    counts->ttisgmt_count = (int)raw[0];
    counts->ttisstd_count = (int)raw[1];
    counts->leap_count = (int)raw[2];
    counts->time_count = (int)raw[3];
    counts->type_count = (int)raw[4];
    counts->character_count = (int)raw[5];
    return counts->time_count >= 0 &&
           counts->time_count <= ROUTER_TZ_MAX_TRANSITIONS &&
           counts->type_count >= 1 &&
           counts->type_count <= ROUTER_TZ_MAX_TYPES &&
           counts->character_count >= 0;
}

static bool add_size(size_t *total, size_t count, size_t width) {
    if (!total || (count > 0 && width > (SIZE_MAX - *total) / count)) {
        return false;
    }
    *total += count * width;
    return true;
}

static bool tzif_block_size(const tzif_counts_t *counts, size_t time_width,
                            size_t *size) {
    size_t result = 0;
    bool valid = add_size(&result, (size_t)counts->time_count, time_width) &&
        add_size(&result, (size_t)counts->time_count, 1) &&
        add_size(&result, (size_t)counts->type_count, 6) &&
        add_size(&result, (size_t)counts->character_count, 1) &&
        add_size(&result, (size_t)counts->leap_count, time_width + 4) &&
        add_size(&result, (size_t)counts->ttisstd_count, 1) &&
        add_size(&result, (size_t)counts->ttisgmt_count, 1);
    if (valid) *size = result;
    return valid;
}

static void timezone_clear(compiled_timezone_t *timezone) {
    if (!timezone) return;
    free(timezone->transitions);
    free(timezone->transition_types);
    memset(timezone, 0, sizeof(*timezone));
}

static bool timezone_load_tzif(const char *name,
                               compiled_timezone_t *timezone) {
    if (!name || !timezone) return false;
    memset(timezone, 0, sizeof(*timezone));
    if (strcmp(name, "UTC") == 0 || strcmp(name, "Etc/UTC") == 0 ||
        strcmp(name, "GMT") == 0) {
        timezone->utc = true;
        timezone->type_count = 1;
        return true;
    }

    char path[256];
    int path_length = snprintf(path, sizeof(path), "/usr/share/zoneinfo/%s",
                               name);
    if (path_length < 0 || (size_t)path_length >= sizeof(path)) return false;
    FILE *file = fopen(path, "rb");
    if (!file) return false;
    if (fseek(file, 0, SEEK_END) != 0) {
        fclose(file);
        return false;
    }
    long file_length = ftell(file);
    if (file_length < 44 || (unsigned long)file_length >
            ROUTER_TZ_MAX_FILE_BYTES ||
        fseek(file, 0, SEEK_SET) != 0) {
        fclose(file);
        return false;
    }
    size_t size = (size_t)file_length;
    unsigned char *bytes = malloc(size);
    if (!bytes || fread(bytes, 1, size, file) != size) {
        free(bytes);
        fclose(file);
        return false;
    }
    fclose(file);

    tzif_counts_t counts;
    char version = '\0';
    size_t header_offset = 0;
    size_t time_width = 4;
    if (!parse_tzif_header(bytes, size, 0, &counts, &version)) {
        free(bytes);
        return false;
    }
    if (version == '2' || version == '3' || version == '4') {
        size_t first_block = 0;
        if (!tzif_block_size(&counts, 4, &first_block) ||
            first_block > size - 44) {
            free(bytes);
            return false;
        }
        header_offset = 44 + first_block;
        if (!parse_tzif_header(bytes, size, header_offset, &counts, NULL)) {
            free(bytes);
            return false;
        }
        time_width = 8;
    }

    size_t block_size = 0;
    size_t data_offset = header_offset + 44;
    if (!tzif_block_size(&counts, time_width, &block_size) ||
        data_offset > size || block_size > size - data_offset) {
        free(bytes);
        return false;
    }
    if (counts.time_count > 0) {
        timezone->transitions = calloc((size_t)counts.time_count,
                                       sizeof(*timezone->transitions));
        timezone->transition_types = calloc(
            (size_t)counts.time_count, sizeof(*timezone->transition_types));
        if (!timezone->transitions || !timezone->transition_types) {
            free(bytes);
            timezone_clear(timezone);
            return false;
        }
    }
    const unsigned char *transition_data = bytes + data_offset;
    const unsigned char *type_indices = transition_data +
        (size_t)counts.time_count * time_width;
    const unsigned char *type_data = type_indices + counts.time_count;
    for (int index = 0; index < counts.time_count; index++) {
        timezone->transitions[index] = time_width == 8
            ? read_be64_signed(transition_data + (size_t)index * 8)
            : read_be32_signed(transition_data + (size_t)index * 4);
        timezone->transition_types[index] = type_indices[index];
        if (type_indices[index] >= counts.type_count) {
            free(bytes);
            timezone_clear(timezone);
            return false;
        }
    }
    timezone->type_count = counts.type_count;
    timezone->transition_count = counts.time_count;
    timezone->default_type = 0;
    bool default_found = false;
    for (int index = 0; index < counts.type_count; index++) {
        timezone->offsets[index] =
            read_be32_signed(type_data + (size_t)index * 6);
        timezone->is_dst[index] = type_data[(size_t)index * 6 + 4];
        if (!default_found && timezone->is_dst[index] == 0) {
            timezone->default_type = index;
            default_found = true;
        }
    }
    free(bytes);
    return true;
}

static int32_t timezone_offset_at(const compiled_timezone_t *timezone,
                                  int64_t epoch) {
    if (!timezone || timezone->utc || timezone->transition_count == 0) {
        return timezone && timezone->type_count > 0
            ? timezone->offsets[timezone->default_type] : 0;
    }
    int low = 0;
    int high = timezone->transition_count;
    while (low < high) {
        int middle = low + (high - low) / 2;
        if (timezone->transitions[middle] <= epoch) {
            low = middle + 1;
        } else {
            high = middle;
        }
    }
    int type = low == 0 ? timezone->default_type
                        : timezone->transition_types[low - 1];
    return timezone->offsets[type];
}

static bool copy_filter_values(const cJSON *array,
                               char values[ROUTER_FILTER_MAX_VALUES]
                                          [ROUTER_FILTER_VALUE_MAX],
                               int *count) {
    int size = cJSON_IsArray(array) ? cJSON_GetArraySize(array) : 0;
    if (!count || size < 0 || size > ROUTER_FILTER_MAX_VALUES) return false;
    *count = 0;
    for (int index = 0; index < size; index++) {
        const cJSON *item = cJSON_GetArrayItem(array, index);
        if (!cJSON_IsString(item) || !item->valuestring ||
            strlen(item->valuestring) >= ROUTER_FILTER_VALUE_MAX) {
            return false;
        }
        safe_strcpy(values[(*count)++], item->valuestring,
                    ROUTER_FILTER_VALUE_MAX, 0);
    }
    return true;
}

static bool compile_predicate(compiled_route_t *compiled) {
    cJSON *root = cJSON_Parse(compiled->route.predicate_json);
    if (!cJSON_IsObject(root)) {
        cJSON_Delete(root);
        return false;
    }
    const cJSON *detection =
        cJSON_GetObjectItemCaseSensitive(root, "detection");
    if (!detection) {
        cJSON_Delete(root);
        return true;
    }
    const cJSON *labels =
        cJSON_GetObjectItemCaseSensitive(detection, "labels_any");
    const cJSON *zones =
        cJSON_GetObjectItemCaseSensitive(detection, "zone_ids_any");
    const cJSON *confidence =
        cJSON_GetObjectItemCaseSensitive(detection, "min_confidence");
    bool valid = copy_filter_values(labels, compiled->detection.labels,
                                    &compiled->detection.label_count) &&
        copy_filter_values(zones, compiled->detection.zones,
                           &compiled->detection.zone_count);
    if (valid && confidence) {
        if (!cJSON_IsNumber(confidence)) {
            valid = false;
        } else {
            compiled->detection.has_min_confidence = true;
            compiled->detection.min_confidence = confidence->valuedouble;
        }
    }
    cJSON_Delete(root);
    return valid;
}

static int clock_minute(const char *value) {
    if (!value || strlen(value) != 5) return -1;
    return ((value[0] - '0') * 10 + value[1] - '0') * 60 +
           (value[3] - '0') * 10 + value[4] - '0';
}

static bool compile_schedule(compiled_route_t *compiled) {
    cJSON *root = cJSON_Parse(compiled->route.schedule_json);
    const cJSON *timezone =
        cJSON_GetObjectItemCaseSensitive(root, "timezone");
    const cJSON *windows = cJSON_GetObjectItemCaseSensitive(root, "windows");
    int count = cJSON_IsArray(windows) ? cJSON_GetArraySize(windows) : -1;
    if (!cJSON_IsObject(root) || !cJSON_IsString(timezone) || count < 0 ||
        count > ROUTER_SCHEDULE_MAX_WINDOWS) {
        cJSON_Delete(root);
        return false;
    }
    compiled->window_count = count;
    for (int index = 0; index < count; index++) {
        const cJSON *window = cJSON_GetArrayItem(windows, index);
        const cJSON *days = cJSON_GetObjectItemCaseSensitive(window, "days");
        const cJSON *start = cJSON_GetObjectItemCaseSensitive(window, "start");
        const cJSON *end = cJSON_GetObjectItemCaseSensitive(window, "end");
        int day_count = cJSON_IsArray(days) ? cJSON_GetArraySize(days) : -1;
        if (!cJSON_IsString(start) || !cJSON_IsString(end) || day_count < 1) {
            cJSON_Delete(root);
            return false;
        }
        compiled_schedule_window_t *destination = &compiled->windows[index];
        destination->start_minute = clock_minute(start->valuestring);
        destination->end_minute = clock_minute(end->valuestring);
        for (int day_index = 0; day_index < day_count; day_index++) {
            const cJSON *day = cJSON_GetArrayItem(days, day_index);
            if (!cJSON_IsNumber(day) || day->valueint < 0 ||
                day->valueint > 6) {
                cJSON_Delete(root);
                return false;
            }
            destination->days |= (unsigned char)(1U << day->valueint);
        }
    }
    bool valid = count == 0 ||
        timezone_load_tzif(timezone->valuestring, &compiled->timezone);
    if (!valid) {
        log_error("Event route %s uses unavailable timezone %s",
                  compiled->route.uuid, timezone->valuestring);
    }
    cJSON_Delete(root);
    return valid;
}

static bool compile_route(compiled_route_t *compiled,
                          const event_route_t *route) {
    memset(compiled, 0, sizeof(*compiled));
    compiled->route = *route;
    if (db_event_route_validate(route, NULL, 0) != DB_EVENT_ROUTE_OK) {
        return false;
    }
    if (strcmp(route->destination_key,
               EVENT_ROUTE_DEFAULT_DESTINATION) == 0) {
        compiled->destination.enabled = true;
    } else if (db_event_destination_get_by_key(
                   route->destination_key, &compiled->destination) !=
                   DB_EVENT_DESTINATION_OK) {
        return false;
    }
    if (strcmp(route->scope_type, "selector") == 0) {
        cJSON *selector_json = cJSON_Parse(route->selector_json);
        compiled->selector = fleet_selector_parse(selector_json, NULL, 0);
        cJSON_Delete(selector_json);
        if (!compiled->selector) return false;
    }
    return compile_predicate(compiled) && compile_schedule(compiled);
}

static void compiled_route_clear(compiled_route_t *route) {
    if (!route) return;
    fleet_selector_free(route->selector);
    timezone_clear(&route->timezone);
    memset(route, 0, sizeof(*route));
}

static void clear_routes_locked(void) {
    for (int index = 0; index < ROUTER.route_count; index++) {
        compiled_route_clear(&ROUTER.routes[index]);
    }
    free(ROUTER.routes);
    ROUTER.routes = NULL;
    ROUTER.route_count = 0;
    ROUTER.definition_count = 0;
}

static bool reload_routes_locked(uint64_t route_generation,
                                 uint64_t destination_generation) {
    int total = db_event_route_count();
    if (total < 0 || total > EVENT_ROUTE_MAX_COUNT) return false;
    event_route_t *routes = total > 0
        ? calloc((size_t)total, sizeof(*routes)) : NULL;
    if (total > 0 && !routes) return false;
    int count = total > 0 ? db_event_route_list(routes, total) : 0;
    if (count < 0) {
        free(routes);
        return false;
    }
    int enabled_count = 0;
    for (int index = 0; index < count; index++) {
        if (routes[index].enabled) enabled_count++;
    }
    compiled_route_t *compiled = enabled_count > 0
        ? calloc((size_t)enabled_count, sizeof(*compiled)) : NULL;
    if (enabled_count > 0 && !compiled) {
        free(routes);
        return false;
    }
    int compiled_count = 0;
    for (int index = 0; index < count; index++) {
        if (!routes[index].enabled) continue;
        compiled_route_t *destination = &compiled[compiled_count++];
        destination->valid = compile_route(destination, &routes[index]);
    }
    free(routes);
    clear_routes_locked();
    ROUTER.routes = compiled;
    ROUTER.route_count = compiled_count;
    ROUTER.definition_count = count;
    ROUTER.route_generation = route_generation;
    ROUTER.destination_generation = destination_generation;
    ROUTER.stats.cache_reloads++;
    return true;
}

static bool ensure_routes_locked(void) {
    uint64_t route_generation = db_event_route_generation();
    uint64_t destination_generation = db_event_destination_generation();
    return (ROUTER.route_generation == route_generation &&
            ROUTER.destination_generation == destination_generation) ||
           reload_routes_locked(route_generation, destination_generation);
}

static bool route_has_type(const event_route_t *route, const char *type) {
    for (int index = 0; index < route->event_type_count; index++) {
        if (strcmp(route->event_types[index], type) == 0) return true;
    }
    return false;
}

static bool string_matches(const char *value,
                           const char candidates[ROUTER_FILTER_MAX_VALUES]
                                                [ROUTER_FILTER_VALUE_MAX],
                           int count) {
    if (count == 0) return true;
    for (int index = 0; index < count; index++) {
        if (strcmp(value ? value : "", candidates[index]) == 0) return true;
    }
    return false;
}

static bool predicate_matches(const compiled_route_t *route,
                              const event_envelope_t *event) {
    if (strcmp(event->type, DETECTION_EVENT_TYPE) != 0 ||
        (route->detection.label_count == 0 &&
         route->detection.zone_count == 0 &&
         !route->detection.has_min_confidence)) {
        return true;
    }
    const cJSON *detections =
        cJSON_GetObjectItemCaseSensitive(event->data, "detections");
    const cJSON *item = NULL;
    cJSON_ArrayForEach(item, detections) {
        const cJSON *label = cJSON_GetObjectItemCaseSensitive(item, "label");
        const cJSON *zone = cJSON_GetObjectItemCaseSensitive(item, "zone_id");
        const cJSON *confidence =
            cJSON_GetObjectItemCaseSensitive(item, "confidence");
        if (!cJSON_IsString(label) || !cJSON_IsNumber(confidence)) continue;
        if (!string_matches(label->valuestring, route->detection.labels,
                            route->detection.label_count) ||
            !string_matches(cJSON_IsString(zone) ? zone->valuestring : "",
                            route->detection.zones,
                            route->detection.zone_count) ||
            (route->detection.has_min_confidence &&
             confidence->valuedouble < route->detection.min_confidence)) {
            continue;
        }
        return true;
    }
    return false;
}

static bool schedule_matches(const compiled_route_t *route, int64_t epoch) {
    if (route->window_count == 0) return true;
    int32_t offset = timezone_offset_at(&route->timezone, epoch);
    if ((offset > 0 && epoch > INT64_MAX - offset) ||
        (offset < 0 && epoch < INT64_MIN - offset)) {
        return false;
    }
    time_t local_epoch = (time_t)(epoch + offset);
    if ((int64_t)local_epoch != epoch + offset) return false;
    struct tm local;
    if (!gmtime_r(&local_epoch, &local)) return false;
    int minute = local.tm_hour * 60 + local.tm_min;
    unsigned char today = (unsigned char)(1U << local.tm_wday);
    int previous_day = (local.tm_wday + 6) % 7;
    unsigned char previous = (unsigned char)(1U << previous_day);
    for (int index = 0; index < route->window_count; index++) {
        const compiled_schedule_window_t *window = &route->windows[index];
        if (window->start_minute < window->end_minute) {
            if ((window->days & today) && minute >= window->start_minute &&
                minute < window->end_minute) {
                return true;
            }
        } else if (((window->days & today) &&
                    minute >= window->start_minute) ||
                   ((window->days & previous) &&
                    minute < window->end_minute)) {
            return true;
        }
    }
    return false;
}

static const char *camera_uuid_from_subject(const char *subject) {
    static const char prefix[] = "camera/";
    if (!subject || strncmp(subject, prefix, sizeof(prefix) - 1) != 0) {
        return NULL;
    }
    const char *uuid = subject + sizeof(prefix) - 1;
    return lightnvr_uuid_is_valid(uuid) ? uuid : NULL;
}

static bool refresh_inventory_locked(int64_t now) {
    if (ROUTER.inventory_loaded && now >= ROUTER.inventory_loaded_at &&
        now - ROUTER.inventory_loaded_at < ROUTER_INVENTORY_TTL_SECONDS) {
        return true;
    }
    fleet_camera_t *inventory = NULL;
    int count = 0;
    if (db_fleet_camera_load(&inventory, &count) != 0) return false;
    fleet_camera_enrich_runtime_health(inventory, count);
    free(ROUTER.inventory);
    ROUTER.inventory = inventory;
    ROUTER.inventory_count = count;
    ROUTER.inventory_loaded_at = now;
    ROUTER.inventory_loaded = true;
    return true;
}

static int find_camera_locked(const char *uuid, int64_t now,
                              const fleet_camera_t **camera) {
    if (camera) *camera = NULL;
    if (!uuid || !camera || !refresh_inventory_locked(now)) return -1;
    int low = 0;
    int high = ROUTER.inventory_count;
    while (low < high) {
        int middle = low + (high - low) / 2;
        int comparison = strcmp(ROUTER.inventory[middle].camera_uuid, uuid);
        if (comparison < 0) {
            low = middle + 1;
        } else {
            high = middle;
        }
    }
    if (low >= ROUTER.inventory_count ||
        strcmp(ROUTER.inventory[low].camera_uuid, uuid) != 0) {
        return 0;
    }
    *camera = &ROUTER.inventory[low];
    return 1;
}

static bool route_uses_suppression(const event_route_t *route) {
    return route->debounce_seconds > 0 || route->cooldown_seconds > 0 ||
        route->grouping_window_seconds > 0 ||
        route->max_events_per_minute > 0;
}

static bool delivery_plan_append(event_route_delivery_plan_t *plan,
                                 const compiled_route_t *compiled,
                                 bool suppression_pending) {
    if (!plan) return true;
    if (plan->count == plan->capacity) {
        size_t capacity = plan->capacity == 0 ? 4 : plan->capacity * 2;
        if (capacity > EVENT_ROUTE_MAX_COUNT) capacity = EVENT_ROUTE_MAX_COUNT;
        if (capacity <= plan->capacity) return false;
        event_route_delivery_plan_entry_t *entries = realloc(
            plan->entries, capacity * sizeof(*entries));
        if (!entries) return false;
        plan->entries = entries;
        plan->capacity = capacity;
    }
    event_route_delivery_plan_entry_t *entry = &plan->entries[plan->count++];
    memset(entry, 0, sizeof(*entry));
    const event_route_t *route = &compiled->route;
    safe_strcpy(entry->route_uuid, route->uuid, sizeof(entry->route_uuid), 0);
    entry->route_revision = route->revision;
    safe_strcpy(entry->destination_key, route->destination_key,
                sizeof(entry->destination_key), 0);
    if (strcmp(route->destination_key,
               EVENT_ROUTE_DEFAULT_DESTINATION) != 0) {
        safe_strcpy(entry->topic_template,
                    compiled->destination.topic_template,
                    sizeof(entry->topic_template), 0);
    }
    entry->suppression_pending = suppression_pending;
    return true;
}

static void count_suppression_locked(event_suppression_result_t result) {
    switch (result) {
        case EVENT_SUPPRESSION_DEBOUNCE:
            ROUTER.stats.debounce_suppressions++;
            break;
        case EVENT_SUPPRESSION_COOLDOWN:
            ROUTER.stats.cooldown_suppressions++;
            break;
        case EVENT_SUPPRESSION_GROUPING:
            ROUTER.stats.grouping_suppressions++;
            break;
        case EVENT_SUPPRESSION_RATE:
            ROUTER.stats.rate_suppressions++;
            break;
        default:
            break;
    }
}

static void maybe_prune_suppression_locked(int64_t now) {
    if (now <= 0 ||
        (ROUTER.last_suppression_prune_at > 0 &&
         now >= ROUTER.last_suppression_prune_at &&
         now - ROUTER.last_suppression_prune_at <
             ROUTER_SUPPRESSION_PRUNE_INTERVAL_SECONDS)) {
        return;
    }
    int deleted = 0;
    int result = db_event_route_suppression_prune(
        now - EVENT_ROUTE_SUPPRESSION_RETENTION_SECONDS,
        ROUTER_SUPPRESSION_PRUNE_BATCH, &deleted);
    if (result == 0) {
        ROUTER.last_suppression_prune_at = now;
        if (deleted > 0) {
            log_info("Pruned %d expired event route suppression states",
                     deleted);
        }
    } else {
        log_warn("Could not prune expired event route suppression states");
    }
}

event_router_result_t event_router_evaluate_delivery(
    const event_envelope_t *event, event_route_delivery_plan_t *plan) {
    if (plan) memset(plan, 0, sizeof(*plan));
    if (!event || event_envelope_validate(event, NULL, 0) != 0) {
        return EVENT_ROUTER_ERROR;
    }
    if (plan) {
        safe_strcpy(plan->event_id, event->id, sizeof(plan->event_id), 0);
        safe_strcpy(plan->event_type, event->type,
                    sizeof(plan->event_type), 0);
        safe_strcpy(plan->subject, event->subject, sizeof(plan->subject), 0);
    }
    pthread_mutex_lock(&ROUTER.mutex);
    ROUTER.stats.events_evaluated++;
    if (!ensure_routes_locked()) {
        ROUTER.stats.evaluation_errors++;
        pthread_mutex_unlock(&ROUTER.mutex);
        return EVENT_ROUTER_ERROR;
    }
    if (ROUTER.definition_count == 0) {
        ROUTER.stats.default_events++;
        pthread_mutex_unlock(&ROUTER.mutex);
        return EVENT_ROUTER_DEFAULT;
    }

    bool relevant_error = false;
    bool plan_error = false;
    bool should_enqueue = false;
    const fleet_camera_t *camera = NULL;
    bool camera_resolved = false;
    const char *camera_uuid = camera_uuid_from_subject(event->subject);
    int64_t now = (int64_t)time(NULL);
    maybe_prune_suppression_locked(now);
    for (int index = 0; index < ROUTER.route_count; index++) {
        compiled_route_t *route = &ROUTER.routes[index];
        ROUTER.stats.routes_considered++;
        if (!route_has_type(&route->route, event->type)) {
            ROUTER.stats.type_rejections++;
            continue;
        }
        if (!route->valid) {
            relevant_error = true;
            continue;
        }
        if (!route->destination.enabled) {
            ROUTER.stats.destination_disabled_rejections++;
            continue;
        }
        if (route->selector) {
            if (!camera_uuid) {
                ROUTER.stats.scope_rejections++;
                continue;
            }
            if (!camera_resolved) {
                int camera_result = find_camera_locked(camera_uuid, now,
                                                       &camera);
                if (camera_result < 0) {
                    relevant_error = true;
                }
                camera_resolved = true;
            }
            if (!camera) {
                ROUTER.stats.scope_rejections++;
                continue;
            }
            if (!fleet_selector_matches(route->selector, camera, NULL)) {
                ROUTER.stats.scope_rejections++;
                continue;
            }
        }
        if (!predicate_matches(route, event)) {
            ROUTER.stats.predicate_rejections++;
            continue;
        }
        if (!schedule_matches(route, (int64_t)event->occurred_at)) {
            ROUTER.stats.schedule_rejections++;
            continue;
        }
        if (!route_uses_suppression(&route->route)) {
            if (!delivery_plan_append(plan, route, false)) {
                relevant_error = true;
                plan_error = true;
            } else {
                should_enqueue = true;
            }
            continue;
        }
        event_suppression_result_t suppression =
            db_event_route_suppression_check(
                &route->route, event->type, event->subject, now);
        if (suppression == EVENT_SUPPRESSION_PERMIT) {
            if (!delivery_plan_append(plan, route, true)) {
                relevant_error = true;
                plan_error = true;
                ROUTER.stats.suppression_errors++;
            } else {
                should_enqueue = true;
            }
        } else if (suppression > EVENT_SUPPRESSION_PERMIT) {
            count_suppression_locked(suppression);
        } else {
            relevant_error = true;
            ROUTER.stats.suppression_errors++;
        }
    }
    if (should_enqueue && !plan_error) {
        ROUTER.stats.matched_events++;
        pthread_mutex_unlock(&ROUTER.mutex);
        return EVENT_ROUTER_MATCH;
    }
    event_router_result_t result = relevant_error
        ? EVENT_ROUTER_ERROR : EVENT_ROUTER_NO_MATCH;
    if (result == EVENT_ROUTER_ERROR) {
        ROUTER.stats.evaluation_errors++;
    } else {
        ROUTER.stats.unmatched_events++;
    }
    pthread_mutex_unlock(&ROUTER.mutex);
    return result;
}

event_router_result_t event_router_evaluate(const event_envelope_t *event) {
    return event_router_evaluate_delivery(event, NULL);
}

int event_router_record_enqueued(const event_envelope_t *event,
                                 const event_route_delivery_plan_t *plan) {
    if (!event || !plan || strcmp(event->id, plan->event_id) != 0 ||
        strcmp(event->type, plan->event_type) != 0 ||
        strcmp(event->subject, plan->subject) != 0) {
        return -1;
    }
    int failed = 0;
    int64_t now = (int64_t)time(NULL);
    for (size_t index = 0; index < plan->count; index++) {
        const event_route_delivery_plan_entry_t *entry =
            &plan->entries[index];
        if (!entry->suppression_pending) continue;
        event_suppression_result_t result =
            db_event_route_suppression_record_allowed(
                entry->route_uuid, entry->route_revision, event->type,
                event->subject, event->id, now);
        /* A concurrent route edit deliberately discards the old policy state. */
        if (result != EVENT_SUPPRESSION_PERMIT &&
            result != EVENT_SUPPRESSION_STALE) {
            failed = 1;
        }
    }
    return failed ? -1 : 0;
}

int event_router_record_destination_enqueued(
    const event_envelope_t *event, const event_route_delivery_plan_t *plan,
    const char *destination_key) {
    if (!event || !plan || !destination_key || destination_key[0] == '\0' ||
        strcmp(event->id, plan->event_id) != 0 ||
        strcmp(event->type, plan->event_type) != 0 ||
        strcmp(event->subject, plan->subject) != 0) {
        return -1;
    }
    int failed = 0;
    int64_t now = (int64_t)time(NULL);
    for (size_t index = 0; index < plan->count; index++) {
        const event_route_delivery_plan_entry_t *entry =
            &plan->entries[index];
        if (!entry->suppression_pending ||
            strcmp(entry->destination_key, destination_key) != 0) {
            continue;
        }
        event_suppression_result_t result =
            db_event_route_suppression_record_allowed(
                entry->route_uuid, entry->route_revision, event->type,
                event->subject, event->id, now);
        if (result != EVENT_SUPPRESSION_PERMIT &&
            result != EVENT_SUPPRESSION_STALE) {
            failed = 1;
        }
    }
    return failed ? -1 : 0;
}

void event_route_delivery_plan_clear(event_route_delivery_plan_t *plan) {
    if (!plan) return;
    free(plan->entries);
    memset(plan, 0, sizeof(*plan));
}

void event_router_get_stats(event_router_stats_t *stats) {
    if (!stats) return;
    pthread_mutex_lock(&ROUTER.mutex);
    *stats = ROUTER.stats;
    pthread_mutex_unlock(&ROUTER.mutex);
}

void event_router_shutdown(void) {
    pthread_mutex_lock(&ROUTER.mutex);
    clear_routes_locked();
    free(ROUTER.inventory);
    ROUTER.inventory = NULL;
    ROUTER.inventory_count = 0;
    ROUTER.inventory_loaded_at = 0;
    ROUTER.inventory_loaded = false;
    ROUTER.last_suppression_prune_at = 0;
    ROUTER.route_generation = 0;
    ROUTER.destination_generation = 0;
    memset(&ROUTER.stats, 0, sizeof(ROUTER.stats));
    pthread_mutex_unlock(&ROUTER.mutex);
}
