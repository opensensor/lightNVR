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
#include "database/db_core.h"
#include "database/db_zones.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_zones_test.db"

static detection_zone_t make_zone(const char *id, const char *stream,
                                  const char *name, bool enabled) {
    detection_zone_t z;
    memset(&z, 0, sizeof(z));
    strncpy(z.id,          id,     sizeof(z.id)     - 1);
    strncpy(z.stream_name, stream, sizeof(z.stream_name) - 1);
    strncpy(z.name,        name,   sizeof(z.name)   - 1);
    strncpy(z.color,       "#ff0000", sizeof(z.color) - 1);
    z.enabled = enabled;
    /* Triangle polygon */
    z.polygon[0].x = 0.0f; z.polygon[0].y = 0.0f;
    z.polygon[1].x = 1.0f; z.polygon[1].y = 0.0f;
    z.polygon[2].x = 0.5f; z.polygon[2].y = 1.0f;
    z.polygon_count = 3;
    z.min_confidence = 0.5f;
    strncpy(z.filter_classes, "person,car", sizeof(z.filter_classes) - 1);
    return z;
}

static void clear_zones(void) {
    sqlite3_exec(get_db_handle(), "DELETE FROM detection_zones;", NULL, NULL, NULL);
}

void setUp(void)    { clear_zones(); }
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
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}

