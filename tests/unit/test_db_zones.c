/**
 * @file test_db_zones.c
 * @brief Layer 2 — detection zone CRUD via SQLite
 *
 * Tests save_detection_zones, get_detection_zones,
 * delete_detection_zones, delete_detection_zone, update_zone_enabled.
 */

#define _POSIX_C_SOURCE 200809L
#define _GNU_SOURCE

#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <sqlite3.h>

#include "unity.h"
#include "utils/strings.h"
#include "database/db_core.h"
#include "database/db_zones.h"
#include "database/db_streams.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_zones_test.db"

static detection_zone_t make_zone(const char *id, const char *stream,
                                  const char *name, bool enabled) {
    detection_zone_t z;
    memset(&z, 0, sizeof(z));
    safe_strcpy(z.id,          id,     sizeof(z.id),          0);
    safe_strcpy(z.stream_name, stream, sizeof(z.stream_name), 0);
    safe_strcpy(z.name,        name,   sizeof(z.name),        0);
    safe_strcpy(z.color,       "#ff0000", sizeof(z.color), 0);
    z.enabled = enabled;
    /* Triangle polygon */
    z.polygon[0].x = 0.0f; z.polygon[0].y = 0.0f;
    z.polygon[1].x = 1.0f; z.polygon[1].y = 0.0f;
    z.polygon[2].x = 0.5f; z.polygon[2].y = 1.0f;
    z.polygon_count = 3;
    z.min_confidence = 0.5f;
    safe_strcpy(z.filter_classes, "person,car", sizeof(z.filter_classes), 0);
    return z;
}

static void clear_zones(void) {
    sqlite3_exec(get_db_handle(), "DELETE FROM detection_zones;", NULL, NULL, NULL);
}

/* Insert a stream row so FK constraints are satisfied for zone tests */
static void ensure_test_stream(const char *stream_name) {
    stream_config_t cfg;
    memset(&cfg, 0, sizeof(cfg));
    safe_strcpy(cfg.name, stream_name, sizeof(cfg.name), 0);
    safe_strcpy(cfg.url, "rtsp://localhost/test", sizeof(cfg.url), 0);
    cfg.enabled = true;
    cfg.width   = 1920;
    cfg.height  = 1080;
    cfg.fps     = 30;
    cfg.protocol = STREAM_PROTOCOL_TCP;
    add_stream_config(&cfg);
}

void setUp(void) {
    sqlite3_exec(get_db_handle(), "DELETE FROM detection_zones;", NULL, NULL, NULL);
    sqlite3_exec(get_db_handle(), "DELETE FROM streams;",         NULL, NULL, NULL);
    ensure_test_stream("cam1");
}
void tearDown(void) {}

/* save and get round-trip */
void test_save_and_get_zones(void) {
    detection_zone_t z = make_zone("zone1", "cam1", "Front Gate", true);
    int rc = save_detection_zones("cam1", &z, 1);
    TEST_ASSERT_EQUAL_INT(0, rc);

    detection_zone_t out[16];
    int n = get_detection_zones("cam1", out, 16);
    TEST_ASSERT_EQUAL_INT(1, n);
    TEST_ASSERT_EQUAL_STRING("Front Gate", out[0].name);
    TEST_ASSERT_TRUE(out[0].enabled);
}

/* polygon points preserved */
void test_polygon_points_preserved(void) {
    detection_zone_t z = make_zone("zone2", "cam1", "Driveway", true);
    save_detection_zones("cam1", &z, 1);

    detection_zone_t out[16];
    get_detection_zones("cam1", out, 16);
    TEST_ASSERT_EQUAL_INT(3, out[0].polygon_count);
    TEST_ASSERT_FLOAT_WITHIN(0.001f, 0.5f, out[0].polygon[2].x);
    TEST_ASSERT_FLOAT_WITHIN(0.001f, 1.0f, out[0].polygon[2].y);
}

/* multiple zones per stream */
void test_multiple_zones_per_stream(void) {
    detection_zone_t z[2];
    z[0] = make_zone("mz1", "cam1", "Zone A", true);
    z[1] = make_zone("mz2", "cam1", "Zone B", false);
    int rc = save_detection_zones("cam1", z, 2);
    TEST_ASSERT_EQUAL_INT(0, rc);

    detection_zone_t out[16];
    int n = get_detection_zones("cam1", out, 16);
    TEST_ASSERT_EQUAL_INT(2, n);
}

/* delete_detection_zones removes all for stream */
void test_delete_detection_zones(void) {
    detection_zone_t z = make_zone("dz1", "cam1", "To Delete", true);
    save_detection_zones("cam1", &z, 1);

    int rc = delete_detection_zones("cam1");
    TEST_ASSERT_EQUAL_INT(0, rc);

    detection_zone_t out[16];
    int n = get_detection_zones("cam1", out, 16);
    TEST_ASSERT_EQUAL_INT(0, n);
}

/* delete_detection_zone removes single */
void test_delete_detection_zone_single(void) {
    detection_zone_t z[2];
    z[0] = make_zone("single1", "cam1", "Keep",   true);
    z[1] = make_zone("single2", "cam1", "Delete", true);
    save_detection_zones("cam1", z, 2);

    int rc = delete_detection_zone("single2");
    TEST_ASSERT_EQUAL_INT(0, rc);

    detection_zone_t out[16];
    int n = get_detection_zones("cam1", out, 16);
    TEST_ASSERT_EQUAL_INT(1, n);
    TEST_ASSERT_EQUAL_STRING("Keep", out[0].name);
}

/* update_zone_enabled */
void test_update_zone_enabled(void) {
    detection_zone_t z = make_zone("tog1", "cam1", "Toggle Zone", true);
    save_detection_zones("cam1", &z, 1);

    int rc = update_zone_enabled("tog1", false);
    TEST_ASSERT_EQUAL_INT(0, rc);

    detection_zone_t out[16];
    get_detection_zones("cam1", out, 16);
    TEST_ASSERT_FALSE(out[0].enabled);
}

/* empty zone list */
void test_get_zones_empty_stream(void) {
    detection_zone_t out[16];
    int n = get_detection_zones("no_such_stream", out, 16);
    TEST_ASSERT_EQUAL_INT(0, n);
}

void test_invalid_polygon_keeps_saved_zones(void) {
    clear_zones();
    ensure_test_stream("cam1");
    detection_zone_t zones[2] = {make_zone("zone-a", "cam1", "A", true),
                                 make_zone("zone-b", "cam1", "B", true)};
    TEST_ASSERT_EQUAL_INT(0, save_detection_zones("cam1", zones, 1));

    zones[1].polygon_count = MIN_ZONE_POINTS - 1;
    TEST_ASSERT_EQUAL_INT(-1, save_detection_zones("cam1", zones, 2));
    zones[1].polygon_count = MAX_ZONE_POINTS + 1;
    TEST_ASSERT_EQUAL_INT(-1, save_detection_zones("cam1", zones, 2));

    detection_zone_t out[4];
    TEST_ASSERT_EQUAL_INT(1, get_detection_zones("cam1", out, 4));
    TEST_ASSERT_EQUAL_STRING("zone-a", out[0].id);
}

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_save_and_get_zones);
    RUN_TEST(test_polygon_points_preserved);
    RUN_TEST(test_multiple_zones_per_stream);
    RUN_TEST(test_delete_detection_zones);
    RUN_TEST(test_delete_detection_zone_single);
    RUN_TEST(test_update_zone_enabled);
    RUN_TEST(test_get_zones_empty_stream);
    RUN_TEST(test_invalid_polygon_keeps_saved_zones);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}

