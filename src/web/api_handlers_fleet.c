#define _POSIX_C_SOURCE 200809L

#include <cjson/cJSON.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>

#include "core/camera_selector.h"
#include "core/config.h"
#include "database/db_auth.h"
#include "database/db_fleet_query.h"
#include "telemetry/stream_metrics.h"
#include "utils/strings.h"
#include "web/api_handlers_fleet.h"
#include "web/httpd_utils.h"
#include "web/request_response.h"

#define FLEET_QUERY_DEFAULT_PAGE_SIZE 50
#define FLEET_QUERY_MAX_PAGE_SIZE 200
#define FLEET_PREVIEW_MAX_PAGE_SIZE 50
#define FLEET_QUERY_SEARCH_MAX 256

typedef struct {
    int page;
    int page_size;
    char sort_by[32];
    bool descending;
    char search[FLEET_QUERY_SEARCH_MAX];
    char camera_uuid[CAMERA_UUID_STRING_SIZE];
    bool include_facets;
    bool explain;
} fleet_query_options_t;

typedef struct {
    char uuid[CAMERA_UUID_STRING_SIZE];
    char label[256];
    int count;
} facet_count_t;

static _Thread_local char comparator_sort_by[32];
static _Thread_local bool comparator_descending;

static bool valid_uuid(const char *value) {
    if (!value || strlen(value) != CAMERA_UUID_STRING_SIZE - 1) return false;
    for (int i = 0; i < CAMERA_UUID_STRING_SIZE - 1; i++) {
        char c = value[i];
        if (i == 8 || i == 13 || i == 18 || i == 23) {
            if (c != '-') return false;
        } else if (!((c >= '0' && c <= '9') ||
                     (c >= 'a' && c <= 'f') ||
                     (c >= 'A' && c <= 'F'))) {
            return false;
        }
    }
    return true;
}

static bool contains_case_insensitive(const char *haystack, const char *needle) {
    if (!needle || needle[0] == '\0') return true;
    if (!haystack) return false;
    size_t needle_length = strlen(needle);
    for (const char *candidate = haystack; *candidate; candidate++) {
        if (strncasecmp(candidate, needle, needle_length) == 0) return true;
    }
    return false;
}

static bool camera_matches_search(const fleet_camera_t *camera,
                                  const char *search) {
    if (!search || search[0] == '\0') return true;
    if (contains_case_insensitive(camera->name, search) ||
        contains_case_insensitive(camera->camera_uuid, search) ||
        contains_case_insensitive(camera->address, search) ||
        contains_case_insensitive(camera->location_path, search) ||
        contains_case_insensitive(camera->manufacturer, search) ||
        contains_case_insensitive(camera->model, search)) {
        return true;
    }
    for (int i = 0; i < camera->tag_count; i++) {
        if (contains_case_insensitive(camera->tags[i].label, search)) return true;
    }
    return false;
}

static int compare_bool(bool left, bool right) {
    return left == right ? 0 : (left ? 1 : -1);
}

static int compare_camera_pointers(const void *left_pointer,
                                   const void *right_pointer) {
    const fleet_camera_t *left = *(fleet_camera_t *const *)left_pointer;
    const fleet_camera_t *right = *(fleet_camera_t *const *)right_pointer;
    int result = 0;
    if (strcmp(comparator_sort_by, "camera_uuid") == 0) {
        result = strcmp(left->camera_uuid, right->camera_uuid);
    } else if (strcmp(comparator_sort_by, "location") == 0) {
        result = strcasecmp(left->location_path, right->location_path);
    } else if (strcmp(comparator_sort_by, "health") == 0) {
        result = (int)left->health - (int)right->health;
    } else if (strcmp(comparator_sort_by, "enabled") == 0) {
        result = compare_bool(left->enabled, right->enabled);
    } else if (strcmp(comparator_sort_by, "recording_mode") == 0) {
        result = strcmp(fleet_camera_recording_mode(left),
                        fleet_camera_recording_mode(right));
    } else if (strcmp(comparator_sort_by, "address") == 0) {
        result = strcasecmp(left->address, right->address);
    } else {
        result = strcasecmp(left->name, right->name);
    }
    if (result == 0) result = strcmp(left->camera_uuid, right->camera_uuid);
    return comparator_descending ? -result : result;
}

static bool valid_sort_field(const char *field) {
    return strcmp(field, "name") == 0 || strcmp(field, "camera_uuid") == 0 ||
           strcmp(field, "location") == 0 || strcmp(field, "health") == 0 ||
           strcmp(field, "enabled") == 0 ||
           strcmp(field, "recording_mode") == 0 ||
           strcmp(field, "address") == 0;
}

static bool parse_positive_int(const cJSON *body, const char *field,
                               int default_value, int maximum, int *output,
                               http_response_t *res) {
    const cJSON *item = cJSON_GetObjectItemCaseSensitive(body, field);
    if (!item) {
        *output = default_value;
        return true;
    }
    if (!cJSON_IsNumber(item) || item->valuedouble != item->valueint ||
        item->valueint < 1 || item->valueint > maximum) {
        char message[128];
        snprintf(message, sizeof(message), "%s must be an integer from 1 to %d",
                 field, maximum);
        http_response_set_json_error(res, 400, message);
        return false;
    }
    *output = item->valueint;
    return true;
}

static bool parse_options(const cJSON *body, bool preview,
                          fleet_query_options_t *options,
                          http_response_t *res) {
    memset(options, 0, sizeof(*options));
    options->include_facets = true;
    safe_strcpy(options->sort_by, "name", sizeof(options->sort_by), 0);
    int maximum_page_size = preview ? FLEET_PREVIEW_MAX_PAGE_SIZE
                                    : FLEET_QUERY_MAX_PAGE_SIZE;
    if (!parse_positive_int(body, "page", 1, 1000000, &options->page, res) ||
        !parse_positive_int(body, "page_size", FLEET_QUERY_DEFAULT_PAGE_SIZE,
                            maximum_page_size, &options->page_size, res)) {
        return false;
    }

    const cJSON *sort_by = cJSON_GetObjectItemCaseSensitive(body, "sort_by");
    if (sort_by) {
        if (!cJSON_IsString(sort_by) || !sort_by->valuestring ||
            !valid_sort_field(sort_by->valuestring)) {
            http_response_set_json_error(res, 400, "Invalid sort_by field");
            return false;
        }
        safe_strcpy(options->sort_by, sort_by->valuestring,
                    sizeof(options->sort_by), 0);
    }
    const cJSON *sort_order =
        cJSON_GetObjectItemCaseSensitive(body, "sort_order");
    if (sort_order) {
        if (!cJSON_IsString(sort_order) || !sort_order->valuestring ||
            (strcmp(sort_order->valuestring, "asc") != 0 &&
             strcmp(sort_order->valuestring, "desc") != 0)) {
            http_response_set_json_error(res, 400,
                                         "sort_order must be asc or desc");
            return false;
        }
        options->descending = strcmp(sort_order->valuestring, "desc") == 0;
    }
    const cJSON *search = cJSON_GetObjectItemCaseSensitive(body, "search");
    if (search) {
        if (!cJSON_IsString(search) || !search->valuestring ||
            strlen(search->valuestring) >= sizeof(options->search)) {
            http_response_set_json_error(res, 400, "Invalid search value");
            return false;
        }
        safe_strcpy(options->search, search->valuestring,
                    sizeof(options->search), 0);
    }
    const cJSON *camera_uuid =
        cJSON_GetObjectItemCaseSensitive(body, "camera_uuid");
    if (camera_uuid) {
        if (!cJSON_IsString(camera_uuid) ||
            !valid_uuid(camera_uuid->valuestring)) {
            http_response_set_json_error(res, 400, "Invalid camera_uuid");
            return false;
        }
        safe_strcpy(options->camera_uuid, camera_uuid->valuestring,
                    sizeof(options->camera_uuid), 0);
    }
    const cJSON *facets = cJSON_GetObjectItemCaseSensitive(body, "facets");
    if (facets) {
        if (!cJSON_IsBool(facets)) {
            http_response_set_json_error(res, 400, "facets must be boolean");
            return false;
        }
        options->include_facets = cJSON_IsTrue(facets);
    }
    const cJSON *explain_item =
        cJSON_GetObjectItemCaseSensitive(body, "explain");
    if (explain_item && !cJSON_IsBool(explain_item)) {
        http_response_set_json_error(res, 400, "explain must be boolean");
        return false;
    }
    options->explain = preview || (explain_item && cJSON_IsTrue(explain_item));
    return true;
}

static void enrich_health(fleet_camera_t *cameras, int camera_count) {
    int maximum = metrics_get_max_streams();
    if (maximum <= 0) return;
    stream_metrics_t *metrics = calloc((size_t)maximum, sizeof(*metrics));
    if (!metrics) return;
    int metric_count = metrics_snapshot_all(metrics, maximum);
    for (int i = 0; i < camera_count; i++) {
        if (!cameras[i].enabled) {
            cameras[i].health = FLEET_HEALTH_DISABLED;
            continue;
        }
        for (int j = 0; j < metric_count; j++) {
            if (strcmp(cameras[i].name, metrics[j].stream_name) != 0) continue;
            switch ((stream_health_status_t)metrics[j].health_status) {
                case STREAM_HEALTH_UP:
                    cameras[i].health = FLEET_HEALTH_UP;
                    break;
                case STREAM_HEALTH_DEGRADED:
                    cameras[i].health = FLEET_HEALTH_DEGRADED;
                    break;
                case STREAM_HEALTH_DOWN:
                    cameras[i].health = FLEET_HEALTH_DOWN;
                    break;
            }
            cameras[i].last_frame_ts = (int64_t)metrics[j].last_frame_ts;
            cameras[i].current_fps = metrics[j].current_fps;
            cameras[i].recording_active = metrics[j].recording_active != 0;
            break;
        }
    }
    free(metrics);
}

static bool facet_increment(facet_count_t **facets, int *count, int *capacity,
                            const char *uuid, const char *label) {
    for (int i = 0; i < *count; i++) {
        if (strcasecmp((*facets)[i].uuid, uuid) == 0) {
            (*facets)[i].count++;
            if ((*facets)[i].label[0] == '\0' && label && label[0] != '\0') {
                safe_strcpy((*facets)[i].label, label,
                            sizeof((*facets)[i].label), 0);
            }
            return true;
        }
    }
    if (*count == *capacity) {
        int new_capacity = *capacity == 0 ? 16 : *capacity * 2;
        facet_count_t *resized =
            realloc(*facets, (size_t)new_capacity * sizeof(**facets));
        if (!resized) return false;
        *facets = resized;
        *capacity = new_capacity;
    }
    facet_count_t *facet = &(*facets)[(*count)++];
    memset(facet, 0, sizeof(*facet));
    safe_strcpy(facet->uuid, uuid, sizeof(facet->uuid), 0);
    safe_strcpy(facet->label, label ? label : "", sizeof(facet->label), 0);
    facet->count = 1;
    return true;
}

static int compare_facets(const void *left_pointer, const void *right_pointer) {
    const facet_count_t *left = left_pointer;
    const facet_count_t *right = right_pointer;
    int result = strcasecmp(left->label, right->label);
    return result == 0 ? strcmp(left->uuid, right->uuid) : result;
}

static cJSON *build_facets(fleet_camera_t **matches, int count) {
    int health_counts[FLEET_HEALTH_DISABLED + 1] = {0};
    int enabled_counts[2] = {0};
    int recording_counts[3] = {0};
    facet_count_t *tag_facets = NULL;
    facet_count_t *location_facets = NULL;
    int tag_count = 0, tag_capacity = 0;
    int location_count = 0, location_capacity = 0;

    for (int i = 0; i < count; i++) {
        fleet_camera_t *camera = matches[i];
        if (camera->health >= FLEET_HEALTH_UNKNOWN &&
            camera->health <= FLEET_HEALTH_DISABLED) {
            health_counts[camera->health]++;
        }
        enabled_counts[camera->enabled ? 1 : 0]++;
        const char *mode = fleet_camera_recording_mode(camera);
        recording_counts[strcmp(mode, "off") == 0 ? 0 :
                         strcmp(mode, "continuous") == 0 ? 1 : 2]++;
        for (int j = 0; j < camera->tag_count; j++) {
            if (!facet_increment(&tag_facets, &tag_count, &tag_capacity,
                                 camera->tags[j].uuid,
                                 camera->tags[j].label)) goto fail;
        }
        for (int j = 0; j < camera->location_depth; j++) {
            const char *label =
                strcmp(camera->location_ancestor_uuids[j],
                       camera->location_uuid) == 0 ? camera->location_name : "";
            if (!facet_increment(&location_facets, &location_count,
                                 &location_capacity,
                                 camera->location_ancestor_uuids[j], label)) {
                goto fail;
            }
        }
    }

    if (tag_count > 1) {
        qsort(tag_facets, (size_t)tag_count, sizeof(*tag_facets),
              compare_facets);
    }
    if (location_count > 1) {
        qsort(location_facets, (size_t)location_count,
              sizeof(*location_facets), compare_facets);
    }
    cJSON *root = cJSON_CreateObject();
    cJSON *health = cJSON_CreateArray();
    cJSON *enabled = cJSON_CreateArray();
    cJSON *recording = cJSON_CreateArray();
    cJSON *tags = cJSON_CreateArray();
    cJSON *locations = cJSON_CreateArray();
    if (!root || !health || !enabled || !recording || !tags || !locations) {
        cJSON_Delete(root);
        cJSON_Delete(health);
        cJSON_Delete(enabled);
        cJSON_Delete(recording);
        cJSON_Delete(tags);
        cJSON_Delete(locations);
        goto fail;
    }
    for (int i = FLEET_HEALTH_UNKNOWN; i <= FLEET_HEALTH_DISABLED; i++) {
        cJSON *item = cJSON_CreateObject();
        cJSON_AddStringToObject(item, "value",
                                fleet_health_state_name((fleet_health_state_t)i));
        cJSON_AddNumberToObject(item, "count", health_counts[i]);
        cJSON_AddItemToArray(health, item);
    }
    for (int i = 0; i < 2; i++) {
        cJSON *item = cJSON_CreateObject();
        cJSON_AddBoolToObject(item, "value", i != 0);
        cJSON_AddNumberToObject(item, "count", enabled_counts[i]);
        cJSON_AddItemToArray(enabled, item);
    }
    const char *recording_names[] = {"off", "continuous", "detection"};
    for (int i = 0; i < 3; i++) {
        cJSON *item = cJSON_CreateObject();
        cJSON_AddStringToObject(item, "value", recording_names[i]);
        cJSON_AddNumberToObject(item, "count", recording_counts[i]);
        cJSON_AddItemToArray(recording, item);
    }
    for (int i = 0; i < tag_count; i++) {
        cJSON *item = cJSON_CreateObject();
        cJSON_AddStringToObject(item, "uuid", tag_facets[i].uuid);
        cJSON_AddStringToObject(item, "label", tag_facets[i].label);
        cJSON_AddNumberToObject(item, "count", tag_facets[i].count);
        cJSON_AddItemToArray(tags, item);
    }
    for (int i = 0; i < location_count; i++) {
        cJSON *item = cJSON_CreateObject();
        cJSON_AddStringToObject(item, "uuid", location_facets[i].uuid);
        if (location_facets[i].label[0]) {
            cJSON_AddStringToObject(item, "label", location_facets[i].label);
        }
        cJSON_AddNumberToObject(item, "count", location_facets[i].count);
        cJSON_AddItemToArray(locations, item);
    }
    cJSON_AddItemToObject(root, "health", health);
    cJSON_AddItemToObject(root, "enabled", enabled);
    cJSON_AddItemToObject(root, "recording_mode", recording);
    cJSON_AddItemToObject(root, "tags", tags);
    cJSON_AddItemToObject(root, "locations", locations);
    free(tag_facets);
    free(location_facets);
    return root;

fail:
    free(tag_facets);
    free(location_facets);
    return NULL;
}

static cJSON *camera_to_json(const fleet_camera_t *camera,
                             const fleet_selector_t *selector, bool explain_match) {
    cJSON *object = cJSON_CreateObject();
    if (!object) return NULL;
    if (!cJSON_AddStringToObject(object, "camera_uuid", camera->camera_uuid) ||
        !cJSON_AddStringToObject(object, "name", camera->name) ||
        !cJSON_AddStringToObject(object, "address", camera->address) ||
        !cJSON_AddBoolToObject(object, "enabled", camera->enabled) ||
        !cJSON_AddStringToObject(object, "recording_mode",
                                 fleet_camera_recording_mode(camera)) ||
        !cJSON_AddStringToObject(object, "health",
                                 fleet_health_state_name(camera->health)) ||
        !cJSON_AddNumberToObject(object, "last_frame_ts",
                                 (double)camera->last_frame_ts) ||
        !cJSON_AddNumberToObject(object, "current_fps", camera->current_fps) ||
        !cJSON_AddBoolToObject(object, "recording_active",
                               camera->recording_active) ||
        !cJSON_AddStringToObject(object, "manufacturer", camera->manufacturer) ||
        !cJSON_AddStringToObject(object, "model", camera->model)) {
        goto fail;
    }

    cJSON *capabilities = cJSON_CreateArray();
    if (!capabilities ||
        !cJSON_AddItemToObject(object, "capabilities", capabilities)) {
        cJSON_Delete(capabilities);
        goto fail;
    }
    if (camera->is_onvif) {
        cJSON *capability = cJSON_CreateString("onvif");
        if (!capability || !cJSON_AddItemToArray(capabilities, capability)) {
            cJSON_Delete(capability);
            goto fail;
        }
    }
    if (camera->ptz_enabled) {
        cJSON *capability = cJSON_CreateString("ptz");
        if (!capability || !cJSON_AddItemToArray(capabilities, capability)) {
            cJSON_Delete(capability);
            goto fail;
        }
    }
    if (camera->backchannel_enabled) {
        cJSON *capability = cJSON_CreateString("backchannel");
        if (!capability || !cJSON_AddItemToArray(capabilities, capability)) {
            cJSON_Delete(capability);
            goto fail;
        }
    }

    cJSON *location = cJSON_CreateObject();
    if (!location || !cJSON_AddItemToObject(object, "location", location)) {
        cJSON_Delete(location);
        goto fail;
    }
    if (!cJSON_AddStringToObject(location, "uuid", camera->location_uuid) ||
        !cJSON_AddStringToObject(location, "name", camera->location_name) ||
        !cJSON_AddStringToObject(location, "path", camera->location_path)) {
        goto fail;
    }

    cJSON *tags = cJSON_CreateArray();
    if (!tags || !cJSON_AddItemToObject(object, "tags", tags)) {
        cJSON_Delete(tags);
        goto fail;
    }
    for (int i = 0; i < camera->tag_count; i++) {
        cJSON *tag = cJSON_CreateObject();
        if (!tag ||
            !cJSON_AddStringToObject(tag, "uuid", camera->tags[i].uuid) ||
            !cJSON_AddStringToObject(tag, "label", camera->tags[i].label) ||
            !cJSON_AddItemToArray(tags, tag)) {
            cJSON_Delete(tag);
            goto fail;
        }
    }

    if (explain_match) {
        fleet_selector_explanation_t explanation;
        if (fleet_selector_matches(selector, camera, &explanation)) {
            cJSON *clauses = cJSON_CreateArray();
            if (!clauses ||
                !cJSON_AddItemToObject(object, "matched_clauses", clauses)) {
                cJSON_Delete(clauses);
                goto fail;
            }
            for (int i = 0; i < explanation.clause_count; i++) {
                cJSON *clause = cJSON_CreateString(explanation.clauses[i]);
                if (!clause || !cJSON_AddItemToArray(clauses, clause)) {
                    cJSON_Delete(clause);
                    goto fail;
                }
            }
        }
    }
    return object;

fail:
    cJSON_Delete(object);
    return NULL;
}

static void handle_fleet_query(const http_request_t *req, http_response_t *res,
                               bool preview) {
    user_t user;
    memset(&user, 0, sizeof(user));
    if (!httpd_check_viewer_access(req, &user)) {
        http_response_set_json_error(res, 401, "Unauthorized");
        return;
    }
    cJSON *body = httpd_parse_json_body(req);
    if (!cJSON_IsObject(body)) {
        cJSON_Delete(body);
        http_response_set_json_error(res, 400,
                                     "Request body must be a JSON object");
        return;
    }

    fleet_query_options_t options;
    if (!parse_options(body, preview, &options, res)) {
        cJSON_Delete(body);
        return;
    }
    cJSON *selector_json =
        cJSON_GetObjectItemCaseSensitive(body, "selector");
    cJSON *default_selector_json = NULL;
    if (!selector_json) {
        default_selector_json = cJSON_Parse(
            "{\"version\":1,\"expression\":{\"op\":\"all\"}}");
        selector_json = default_selector_json;
    }
    char selector_error[FLEET_SELECTOR_ERROR_MAX] = {0};
    fleet_selector_t *selector = fleet_selector_parse(
        selector_json, selector_error, sizeof(selector_error));
    cJSON_Delete(default_selector_json);
    if (!selector) {
        cJSON_Delete(body);
        http_response_set_json_error(
            res, 400, selector_error[0] ? selector_error : "Invalid selector");
        return;
    }

    fleet_camera_t *cameras = NULL;
    int camera_count = 0;
    if (db_fleet_camera_load(&cameras, &camera_count) != 0) {
        fleet_selector_free(selector);
        cJSON_Delete(body);
        http_response_set_json_error(res, 500, "Failed to load fleet cameras");
        return;
    }
    enrich_health(cameras, camera_count);
    fleet_camera_t **matches = camera_count > 0 ?
        calloc((size_t)camera_count, sizeof(*matches)) : NULL;
    if (camera_count > 0 && !matches) {
        free(cameras);
        fleet_selector_free(selector);
        cJSON_Delete(body);
        http_response_set_json_error(res, 500, "Out of memory");
        return;
    }

    int match_count = 0;
    for (int i = 0; i < camera_count; i++) {
        if (user.has_tag_restriction &&
            !db_auth_stream_allowed_for_user(&user, cameras[i].legacy_tags)) {
            continue;
        }
        if (options.camera_uuid[0] &&
            strcmp(options.camera_uuid, cameras[i].camera_uuid) != 0) {
            continue;
        }
        if (!camera_matches_search(&cameras[i], options.search)) continue;
        if (!fleet_selector_matches(selector, &cameras[i], NULL)) continue;
        matches[match_count++] = &cameras[i];
    }

    safe_strcpy(comparator_sort_by, options.sort_by,
                sizeof(comparator_sort_by), 0);
    comparator_descending = options.descending;
    if (match_count > 1) {
        qsort(matches, (size_t)match_count, sizeof(*matches),
              compare_camera_pointers);
    }

    cJSON *root = cJSON_CreateObject();
    cJSON *items = cJSON_CreateArray();
    if (!root || !items) {
        cJSON_Delete(root);
        cJSON_Delete(items);
        free(matches);
        free(cameras);
        fleet_selector_free(selector);
        cJSON_Delete(body);
        http_response_set_json_error(res, 500, "Failed to create response");
        return;
    }
    cJSON_AddNumberToObject(root, "selector_version", FLEET_SELECTOR_VERSION);
    cJSON_AddBoolToObject(root, "preview", preview);
    cJSON_AddNumberToObject(root, "page", options.page);
    cJSON_AddNumberToObject(root, "page_size", options.page_size);
    cJSON_AddNumberToObject(root, "total", match_count);
    int total_pages = match_count == 0 ? 0 :
        (match_count + options.page_size - 1) / options.page_size;
    cJSON_AddNumberToObject(root, "total_pages", total_pages);
    cJSON_AddStringToObject(root, "sort_by", options.sort_by);
    cJSON_AddStringToObject(root, "sort_order",
                            options.descending ? "desc" : "asc");
    cJSON_AddItemToObject(root, "cameras", items);

    if (options.include_facets) {
        cJSON *facets = build_facets(matches, match_count);
        if (!facets) {
            cJSON_Delete(root);
            free(matches);
            free(cameras);
            fleet_selector_free(selector);
            cJSON_Delete(body);
            http_response_set_json_error(res, 500,
                                         "Failed to build fleet facets");
            return;
        }
        cJSON_AddItemToObject(root, "facets", facets);
    }

    int64_t start64 = ((int64_t)options.page - 1) * options.page_size;
    int start = start64 < match_count ? (int)start64 : match_count;
    int end = start + options.page_size;
    if (end > match_count) end = match_count;
    for (int i = start; i < end; i++) {
        cJSON *item = camera_to_json(matches[i], selector, options.explain);
        if (!item) {
            cJSON_Delete(root);
            free(matches);
            free(cameras);
            fleet_selector_free(selector);
            cJSON_Delete(body);
            http_response_set_json_error(res, 500,
                                         "Failed to create camera response");
            return;
        }
        cJSON_AddItemToArray(items, item);
    }

    char *json = cJSON_PrintUnformatted(root);
    cJSON_Delete(root);
    free(matches);
    free(cameras);
    fleet_selector_free(selector);
    cJSON_Delete(body);
    if (!json) {
        http_response_set_json_error(res, 500, "Failed to serialize response");
        return;
    }
    http_response_set_json(res, 200, json);
    free(json);
}

void handle_post_fleet_camera_query(const http_request_t *req,
                                    http_response_t *res) {
    handle_fleet_query(req, res, false);
}

void handle_post_fleet_selector_preview(const http_request_t *req,
                                        http_response_t *res) {
    handle_fleet_query(req, res, true);
}
