#include "web/api_handlers_zones.h"
#include "web/api_handlers.h"
#include "web/request_response.h"
#include "web/httpd_utils.h"
#include "core/config.h"
#include "database/db_zones.h"
#include "core/logger.h"
#include <cjson/cJSON.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

/**
 * Handler for GET /api/streams/:stream_name/zones
 */
void handle_get_zones(const http_request_t *req, http_response_t *res) {
    // Check authentication
    if (g_config.web_auth_enabled) {
        user_t user;
        if (!httpd_get_authenticated_user(req, &user)) {
            log_error("Authentication failed for GET zones request");
            http_response_set_json_error(res, 401, "Unauthorized");
            return;
        }
    }

    char stream_name[MAX_STREAM_NAME];

    // Extract stream name from URL
    if (http_request_extract_path_param(req, "/api/streams/", stream_name, sizeof(stream_name)) != 0) {
        http_response_set_json_error(res, 400, "Invalid stream name");
        return;
    }

    // Remove "/zones" suffix from stream_name if present
    char *zones_suffix = strstr(stream_name, "/zones");
    if (zones_suffix) {
        *zones_suffix = '\0';
    }

    log_info("GET /api/streams/%s/zones", stream_name);

    // Get zones from database
    detection_zone_t zones[MAX_ZONES_PER_STREAM];
    int count = get_detection_zones(stream_name, zones, MAX_ZONES_PER_STREAM);

    if (count < 0) {
        http_response_set_json_error(res, 500, "Failed to get detection zones");
        return;
    }

    // Create JSON response
    cJSON *response = cJSON_CreateObject();
    if (!response) {
        http_response_set_json_error(res, 500, "Failed to create JSON response");
        return;
    }

    cJSON *zones_array = cJSON_CreateArray();
    if (!zones_array) {
        cJSON_Delete(response);
        http_response_set_json_error(res, 500, "Failed to create zones array");
        return;
    }

    for (int i = 0; i < count; i++) {
        cJSON *zone_obj = cJSON_CreateObject();
        if (!zone_obj) continue;

        cJSON_AddStringToObject(zone_obj, "id", zones[i].id);
        cJSON_AddStringToObject(zone_obj, "stream_name", zones[i].stream_name);
        cJSON_AddStringToObject(zone_obj, "name", zones[i].name);
        cJSON_AddBoolToObject(zone_obj, "enabled", zones[i].enabled);
        cJSON_AddStringToObject(zone_obj, "color", zones[i].color);

        // Add polygon as array of points
        cJSON *polygon_array = cJSON_CreateArray();
        if (polygon_array) {
            for (int j = 0; j < zones[i].polygon_count; j++) {
                cJSON *point = cJSON_CreateObject();
                if (point) {
                    cJSON_AddNumberToObject(point, "x", zones[i].polygon[j].x);
                    cJSON_AddNumberToObject(point, "y", zones[i].polygon[j].y);
                    cJSON_AddItemToArray(polygon_array, point);
                }
            }
            cJSON_AddItemToObject(zone_obj, "polygon", polygon_array);
        }

        cJSON_AddStringToObject(zone_obj, "filter_classes", zones[i].filter_classes);
        cJSON_AddNumberToObject(zone_obj, "min_confidence", zones[i].min_confidence);

        cJSON_AddItemToArray(zones_array, zone_obj);
    }

    cJSON_AddItemToObject(response, "zones", zones_array);

    char *json_str = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);

    if (json_str) {
        http_response_set_json(res, 200, json_str);
        free(json_str);
    } else {
        http_response_set_json_error(res, 500, "Failed to serialize JSON");
    }
}

/**
 * Handler for POST /api/streams/:stream_name/zones
 */
void handle_post_zones(const http_request_t *req, http_response_t *res) {
    // Check authentication
    if (g_config.web_auth_enabled) {
        user_t user;
        if (!httpd_get_authenticated_user(req, &user)) {
            log_error("Authentication failed for POST zones request");
            http_response_set_json_error(res, 401, "Unauthorized");
            return;
        }
    }

    char stream_name[MAX_STREAM_NAME];

    // Extract stream name from URL
    if (http_request_extract_path_param(req, "/api/streams/", stream_name, sizeof(stream_name)) != 0) {
        http_response_set_json_error(res, 400, "Invalid stream name");
        return;
    }

    // Remove "/zones" suffix from stream_name if present
    char *zones_suffix = strstr(stream_name, "/zones");
    if (zones_suffix) {
        *zones_suffix = '\0';
    }

    log_info("POST /api/streams/%s/zones", stream_name);

    // Parse JSON body
    cJSON *json = httpd_parse_json_body(req);
    if (!json) {
        http_response_set_json_error(res, 400, "Invalid JSON in request body");
        return;
    }

    // Get zones array from JSON
    cJSON *zones_array = cJSON_GetObjectItem(json, "zones");
    if (!zones_array || !cJSON_IsArray(zones_array)) {
        cJSON_Delete(json);
        http_response_set_json_error(res, 400, "Missing or invalid 'zones' array");
        return;
    }

    int zone_count = cJSON_GetArraySize(zones_array);
    if (zone_count > MAX_ZONES_PER_STREAM) {
        cJSON_Delete(json);
        http_response_set_json_error(res, 400, "Too many zones");
        return;
    }

    // Parse zones from JSON
    detection_zone_t zones[MAX_ZONES_PER_STREAM];
    memset(zones, 0, sizeof(zones));

    for (int i = 0; i < zone_count; i++) {
        cJSON *zone_obj = cJSON_GetArrayItem(zones_array, i);
        if (!zone_obj) continue;

        detection_zone_t *zone = &zones[i];

        // Parse zone fields
        cJSON *id = cJSON_GetObjectItem(zone_obj, "id");
        cJSON *name = cJSON_GetObjectItem(zone_obj, "name");
        cJSON *enabled = cJSON_GetObjectItem(zone_obj, "enabled");
        cJSON *color = cJSON_GetObjectItem(zone_obj, "color");
        cJSON *polygon = cJSON_GetObjectItem(zone_obj, "polygon");
        cJSON *filter_classes = cJSON_GetObjectItem(zone_obj, "filter_classes");
        cJSON *min_confidence = cJSON_GetObjectItem(zone_obj, "min_confidence");

        if (!id || !cJSON_IsString(id) || !name || !cJSON_IsString(name) || 
            !polygon || !cJSON_IsArray(polygon)) {
            log_warn("Invalid zone data at index %d", i);
            continue;
        }

        // Copy basic fields
        strncpy(zone->id, id->valuestring, MAX_ZONE_ID - 1);
        strncpy(zone->stream_name, stream_name, MAX_STREAM_NAME - 1);
        strncpy(zone->name, name->valuestring, MAX_ZONE_NAME - 1);
        zone->enabled = enabled && cJSON_IsTrue(enabled);

        if (color && cJSON_IsString(color)) {
            strncpy(zone->color, color->valuestring, 7);
            zone->color[7] = '\0';
        } else {
            strcpy(zone->color, "#3b82f6");
        }

        // Parse polygon points
        int point_count = cJSON_GetArraySize(polygon);
        if (point_count > MAX_ZONE_POINTS) {
            point_count = MAX_ZONE_POINTS;
        }

        zone->polygon_count = 0;
        for (int j = 0; j < point_count; j++) {
            cJSON *point = cJSON_GetArrayItem(polygon, j);
            if (!point) continue;

            cJSON *x = cJSON_GetObjectItem(point, "x");
            cJSON *y = cJSON_GetObjectItem(point, "y");

            if (x && cJSON_IsNumber(x) && y && cJSON_IsNumber(y)) {
                zone->polygon[zone->polygon_count].x = (float)x->valuedouble;
                zone->polygon[zone->polygon_count].y = (float)y->valuedouble;
                zone->polygon_count++;
            }
        }

        // Parse optional fields
        if (filter_classes && cJSON_IsString(filter_classes)) {
            strncpy(zone->filter_classes, filter_classes->valuestring, 255);
            zone->filter_classes[255] = '\0';
        } else {
            zone->filter_classes[0] = '\0';
        }

        if (min_confidence && cJSON_IsNumber(min_confidence)) {
            zone->min_confidence = (float)min_confidence->valuedouble;
        } else {
            zone->min_confidence = 0.0f;
        }
    }

    cJSON_Delete(json);

    // Save zones to database
    if (save_detection_zones(stream_name, zones, zone_count) != 0) {
        http_response_set_json_error(res, 500, "Failed to save detection zones");
        return;
    }

    // Return success
    cJSON *response = cJSON_CreateObject();
    cJSON_AddBoolToObject(response, "success", true);
    cJSON_AddStringToObject(response, "message", "Zones saved successfully");
    cJSON_AddNumberToObject(response, "count", zone_count);

    char *json_str = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);

    if (json_str) {
        http_response_set_json(res, 200, json_str);
        free(json_str);
    } else {
        http_response_set_json_error(res, 500, "Failed to serialize JSON");
    }
}

/**
 * Handler for DELETE /api/streams/:stream_name/zones
 */
void handle_delete_zones(const http_request_t *req, http_response_t *res) {
    // Check authentication
    if (g_config.web_auth_enabled) {
        user_t user;
        if (!httpd_get_authenticated_user(req, &user)) {
            log_error("Authentication failed for DELETE zones request");
            http_response_set_json_error(res, 401, "Unauthorized");
            return;
        }
    }

    char stream_name[MAX_STREAM_NAME];

    // Extract stream name from URL
    if (http_request_extract_path_param(req, "/api/streams/", stream_name, sizeof(stream_name)) != 0) {
        http_response_set_json_error(res, 400, "Invalid stream name");
        return;
    }

    // Remove "/zones" suffix from stream_name if present
    char *zones_suffix = strstr(stream_name, "/zones");
    if (zones_suffix) {
        *zones_suffix = '\0';
    }

    log_info("DELETE /api/streams/%s/zones", stream_name);

    // Delete zones from database
    if (delete_detection_zones(stream_name) != 0) {
        http_response_set_json_error(res, 500, "Failed to delete detection zones");
        return;
    }

    // Return success
    cJSON *response = cJSON_CreateObject();
    cJSON_AddBoolToObject(response, "success", true);
    cJSON_AddStringToObject(response, "message", "Zones deleted successfully");

    char *json_str = cJSON_PrintUnformatted(response);
    cJSON_Delete(response);

    if (json_str) {
        http_response_set_json(res, 200, json_str);
        free(json_str);
    } else {
        http_response_set_json_error(res, 500, "Failed to serialize JSON");
    }
}

