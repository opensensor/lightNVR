/**
 * @file test_storage_pressure_extended.c
 * @brief Layer 1 — extended disk pressure tests
 *
 * Extends the existing test_storage_pressure.c with additional edge-case
 * coverage:
 *   - disk_pressure_level_str() for an unknown enum value
 *   - evaluate_disk_pressure_level() at exact boundary values
 *   - Negative free_pct (treated as below emergency)
 *
 * Zero heavy dependencies — only unity_lib + project headers.
 */

#define _POSIX_C_SOURCE 200809L

#include "unity.h"
#include "storage/storage_manager.h"

/* ---- Unity boilerplate ---- */
void setUp(void)    {}
void tearDown(void) {}

/* ================================================================
 * disk_pressure_level_str — unknown value
 * ================================================================ */

void test_pressure_str_unknown_enum_value(void) {
    /* Cast an out-of-range integer; the implementation should not crash */
    const char *s = disk_pressure_level_str((disk_pressure_level_t)999);
    TEST_ASSERT_NOT_NULL(s);
    /* It should not return one of the known strings */
    TEST_ASSERT_NOT_EQUAL_STRING("Normal",    s);
    TEST_ASSERT_NOT_EQUAL_STRING("Warning",   s);
    TEST_ASSERT_NOT_EQUAL_STRING("Critical",  s);
    TEST_ASSERT_NOT_EQUAL_STRING("Emergency", s);
}

/* ================================================================
 * evaluate_disk_pressure_level — exact boundary verification
 *
 * Thresholds (from storage_manager.h):
 *   EMERGENCY_PCT =  5.0   (<5  → EMERGENCY)
 *   CRITICAL_PCT  = 10.0   (<10 → CRITICAL)
 *   WARNING_PCT   = 20.0   (<20 → WARNING)
 *   ≥20           → NORMAL
 * ================================================================ */

void test_pressure_boundary_exactly_20(void) {
    /* 20.0 is NOT below the warning threshold, so it's NORMAL */
    TEST_ASSERT_EQUAL_INT(DISK_PRESSURE_NORMAL,
                          evaluate_disk_pressure_level(20.0));
}

void test_pressure_boundary_exactly_10(void) {
    /* 10.0 is NOT below the critical threshold, so it's WARNING */
    TEST_ASSERT_EQUAL_INT(DISK_PRESSURE_WARNING,
                          evaluate_disk_pressure_level(10.0));
}

void test_pressure_boundary_exactly_5(void) {
    /* 5.0 is NOT below the emergency threshold, so it's CRITICAL */
    TEST_ASSERT_EQUAL_INT(DISK_PRESSURE_CRITICAL,
                          evaluate_disk_pressure_level(5.0));
}

void test_pressure_boundary_just_above_20(void) {
    TEST_ASSERT_EQUAL_INT(DISK_PRESSURE_NORMAL,
                          evaluate_disk_pressure_level(20.1));
}

void test_pressure_boundary_just_below_5(void) {
    TEST_ASSERT_EQUAL_INT(DISK_PRESSURE_EMERGENCY,
                          evaluate_disk_pressure_level(4.99));
}

/* ================================================================
 * evaluate_disk_pressure_level — negative free_pct
 * ================================================================ */

void test_pressure_negative_free_pct(void) {
    /* A negative value means critically low → EMERGENCY */
    TEST_ASSERT_EQUAL_INT(DISK_PRESSURE_EMERGENCY,
                          evaluate_disk_pressure_level(-1.0));
}

void test_pressure_very_negative_free_pct(void) {
    TEST_ASSERT_EQUAL_INT(DISK_PRESSURE_EMERGENCY,
                          evaluate_disk_pressure_level(-100.0));
}

/* ================================================================
 * evaluate_disk_pressure_level — above 100 %
 * ================================================================ */

void test_pressure_above_100_pct(void) {
    /* A value > 100 should be treated as NORMAL (more than enough space) */
    TEST_ASSERT_EQUAL_INT(DISK_PRESSURE_NORMAL,
                          evaluate_disk_pressure_level(150.0));
}

/* ================================================================
 * main
 * ================================================================ */

int main(void) {
    UNITY_BEGIN();

    RUN_TEST(test_pressure_str_unknown_enum_value);

    RUN_TEST(test_pressure_boundary_exactly_20);
    RUN_TEST(test_pressure_boundary_exactly_10);
    RUN_TEST(test_pressure_boundary_exactly_5);
    RUN_TEST(test_pressure_boundary_just_above_20);
    RUN_TEST(test_pressure_boundary_just_below_5);

    RUN_TEST(test_pressure_negative_free_pct);
    RUN_TEST(test_pressure_very_negative_free_pct);

    RUN_TEST(test_pressure_above_100_pct);

    return UNITY_END();
}

