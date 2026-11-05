/**
 * POST /api/streams/:name/zones must save exactly what was drawn or reject it.
 */
#define _GNU_SOURCE

#include <cjson/cJSON.h>
#include <limits.h>
#include <sqlite3.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include "unity.h"
#include "database/db_core.h"
#include "database/db_streams.h"
#include "database/db_zones.h"
#include "utils/strings.h"
#include "web/api_handlers_zones.h"
#include "web/httpd_utils.h"
#include "web/request_response.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_api_zones.db"
#define STREAM "zone-cam"

int __wrap_httpd_authorize_stream_action(const http_request_t *req, http_response_t *res,
                                         authorization_action_t action, const char *stream_name) {
    (void)req; (void)res; (void)action; (void)stream_name;
    return 1;
}

/* Build a one-zone body whose polygon has `points` valid vertices. */
static char *zone_body(int points) {
    cJSON *root = cJSON_CreateObject();
    cJSON *zones = cJSON_AddArrayToObject(root, "zones");
    cJSON *zone = cJSON_CreateObject();
    cJSON_AddItemToArray(zones, zone);
    cJSON_AddStringToObject(zone, "id", "zone-1");
    cJSON_AddStringToObject(zone, "name", "Driveway");
    cJSON_AddBoolToObject(zone, "enabled", true);
    cJSON *polygon = cJSON_AddArrayToObject(zone, "polygon");
    for (int i = 0; i < points; i++) {
        cJSON *point = cJSON_CreateObject();
        cJSON_AddNumberToObject(point, "x", i * 0.01);
        cJSON_AddNumberToObject(point, "y", (i % 2) * 0.5);
        cJSON_AddItemToArray(polygon, point);
    }
    char *body = cJSON_PrintUnformatted(root);
    cJSON_Delete(root);
    return body;
}

static void post_zones(const char *body, int expected_status) {
    http_request_t request;
    http_response_t response;
    http_request_init(&request);
    http_response_init(&response);
    request.method = HTTP_METHOD_POST;
    safe_strcpy(request.method_str, "POST", sizeof(request.method_str), 0);
    safe_strcpy(request.path, "/api/streams/" STREAM "/zones", sizeof(request.path), 0);
    safe_strcpy(request.uri, request.path, sizeof(request.uri), 0);
    request.body = (void *)body;
    request.body_len = strlen(body);
    handle_post_zones(&request, &response);
    TEST_ASSERT_EQUAL_INT(expected_status, response.status_code);
    http_response_free(&response);
}

static int saved_point_count(void) {
    detection_zone_t zones[MAX_ZONES_PER_STREAM];
    int count = get_detection_zones(STREAM, zones, MAX_ZONES_PER_STREAM);
    TEST_ASSERT_EQUAL_INT(1, count);
    return zones[0].polygon_count;
}

static void post_points(int points, int expected_status) {
    char *body = zone_body(points);
    post_zones(body, expected_status);
    free(body);
}

void setUp(void) {
    sqlite3_exec(get_db_handle(), "DELETE FROM detection_zones;", NULL, NULL, NULL);
}

void tearDown(void) {
}

static void test_polygon_sizes_inside_the_limits_are_saved_whole(void) {
    post_points(MIN_ZONE_POINTS, 200);
    TEST_ASSERT_EQUAL_INT(MIN_ZONE_POINTS, saved_point_count());
    post_points(MAX_ZONE_POINTS, 200);
    TEST_ASSERT_EQUAL_INT(MAX_ZONE_POINTS, saved_point_count());
}

static void test_rejected_polygons_leave_saved_zones_untouched(void) {
    post_points(4, 200);

    post_points(0, 400);
    post_points(MIN_ZONE_POINTS - 1, 400);
    post_points(MAX_ZONE_POINTS + 1, 400);
    post_zones("{\"zones\":[{\"id\":\"z\",\"name\":\"n\",\"polygon\":"
               "[{\"x\":0,\"y\":0},{\"x\":1,\"y\":0},{\"x\":\"bad\",\"y\":1}]}]}", 400);
    post_zones("{\"zones\":[{\"id\":\"z\"}]}", 400);

    TEST_ASSERT_EQUAL_INT(4, saved_point_count());
}

static void test_clearing_all_zones_is_still_allowed(void) {
    post_points(4, 200);
    post_zones("{\"zones\":[]}", 200);

    detection_zone_t zones[MAX_ZONES_PER_STREAM];
    TEST_ASSERT_EQUAL_INT(0, get_detection_zones(STREAM, zones, MAX_ZONES_PER_STREAM));
}

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) return 1;
    stream_config_t stream;
    memset(&stream, 0, sizeof(stream));
    safe_strcpy(stream.name, STREAM, sizeof(stream.name), 0);
    safe_strcpy(stream.url, "rtsp://localhost/test", sizeof(stream.url), 0);
    stream.enabled = true;
    if (add_stream_config(&stream) == 0) return 1;

    UNITY_BEGIN();
    RUN_TEST(test_polygon_sizes_inside_the_limits_are_saved_whole);
    RUN_TEST(test_rejected_polygons_leave_saved_zones_untouched);
    RUN_TEST(test_clearing_all_zones_is_still_allowed);
    int result = UNITY_END();

    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}
