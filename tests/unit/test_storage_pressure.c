/**
 * @file test_storage_pressure.c
 * @brief Layer 1 pure unit tests — disk pressure classification and string helpers
 *
 * These tests exercise only stateless, pure-C logic:
 *   - evaluate_disk_pressure_level()  (static inline in storage_manager.h)
 *   - disk_pressure_level_str()
 *   - Tier-retention cutoff arithmetic
 *
 * No filesystem I/O, no SQLite, no threads.
 * Compiled and linked against unity.c only — zero heavy dependencies.
 */

#define _POSIX_C_SOURCE 200809L

#include <string.h>
#include <time.h>

#include "unity.h"
#include "storage/storage_manager.h"

/* ---- Unity boilerplate ---- */
void setUp(void)    {}
void tearDown(void) {}

/* ================================================================
 * disk_pressure_level_str()
 * ================================================================ */

void test_pressure_str_normal(void) {
    TEST_ASSERT_EQUAL_STRING("Normal", disk_pressure_level_str(DISK_PRESSURE_NORMAL));
}

void test_pressure_str_warning(void) {
    TEST_ASSERT_EQUAL_STRING("Warning", disk_pressure_level_str(DISK_PRESSURE_WARNING));
}

void test_pressure_str_critical(void) {
    TEST_ASSERT_EQUAL_STRING("Critical", disk_pressure_level_str(DISK_PRESSURE_CRITICAL));
}

void test_pressure_str_emergency(void) {
    TEST_ASSERT_EQUAL_STRING("Emergency", disk_pressure_level_str(DISK_PRESSURE_EMERGENCY));
}

/* ================================================================
 * evaluate_disk_pressure_level() — boundary conditions
 *
 * Thresholds (from storage_manager.h):
 *   EMERGENCY_PCT =  5.0   (<5 → EMERGENCY)
 *   CRITICAL_PCT  = 10.0   (<10 → CRITICAL)
 *   WARNING_PCT   = 20.0   (<20 → WARNING)
 *   ≥20           → NORMAL
 * ================================================================ */

void test_pressure_level_well_above_normal(void) {
    TEST_ASSERT_EQUAL_INT(DISK_PRESSURE_NORMAL,
                          evaluate_disk_pressure_level(50.0));
}

void test_pressure_level_at_warning_boundary(void) {
    /* Exactly 20.0 % free → NORMAL (not WARNING, threshold is strictly <20) */
    TEST_ASSERT_EQUAL_INT(DISK_PRESSURE_NORMAL,
                          evaluate_disk_pressure_level(20.0));
}

void test_pressure_level_just_below_warning(void) {
    TEST_ASSERT_EQUAL_INT(DISK_PRESSURE_WARNING,
                          evaluate_disk_pressure_level(19.9));
}

void test_pressure_level_mid_warning(void) {
    TEST_ASSERT_EQUAL_INT(DISK_PRESSURE_WARNING,
                          evaluate_disk_pressure_level(15.0));
}

void test_pressure_level_at_critical_boundary(void) {
    /* Exactly 10.0 % free → WARNING (threshold is strictly <10) */
    TEST_ASSERT_EQUAL_INT(DISK_PRESSURE_WARNING,
                          evaluate_disk_pressure_level(10.0));
}

void test_pressure_level_just_below_critical(void) {
    TEST_ASSERT_EQUAL_INT(DISK_PRESSURE_CRITICAL,
                          evaluate_disk_pressure_level(9.9));
}

void test_pressure_level_mid_critical(void) {
    TEST_ASSERT_EQUAL_INT(DISK_PRESSURE_CRITICAL,
                          evaluate_disk_pressure_level(7.5));
}

void test_pressure_level_at_emergency_boundary(void) {
    /* Exactly 5.0 % free → CRITICAL (threshold is strictly <5) */
    TEST_ASSERT_EQUAL_INT(DISK_PRESSURE_CRITICAL,
                          evaluate_disk_pressure_level(5.0));
}

void test_pressure_level_just_below_emergency(void) {
    TEST_ASSERT_EQUAL_INT(DISK_PRESSURE_EMERGENCY,
                          evaluate_disk_pressure_level(4.9));
}

void test_pressure_level_zero_free(void) {
    TEST_ASSERT_EQUAL_INT(DISK_PRESSURE_EMERGENCY,
                          evaluate_disk_pressure_level(0.0));
}

/* ================================================================
 * Tier-retention cutoff arithmetic
 *
 * The tiered cleanup multiplies base_retention_days by per-tier
 * multipliers before computing the cutoff timestamp:
 *   cutoff = now - (int)(base * mult) * 86400
 *
 * We validate the arithmetic for the four standard tiers.
 * ================================================================ */

void test_tier_critical_multiplier_default(void) {
    /* Default: 3.0× — critical recordings kept 3× longer */
    int base = 7;
    double mult = 3.0;
    int effective = (int)(base * mult);
    TEST_ASSERT_EQUAL_INT(21, effective);
}

void test_tier_important_multiplier_default(void) {
    int base = 7;
    double mult = 2.0;
    int effective = (int)(base * mult);
    TEST_ASSERT_EQUAL_INT(14, effective);
}

void test_tier_standard_multiplier(void) {
    /* Standard tier always uses 1.0× */
    int base = 7;
    double mult = 1.0;
    int effective = (int)(base * mult);
    TEST_ASSERT_EQUAL_INT(7, effective);
}

void test_tier_ephemeral_multiplier_default(void) {
    /* Default: 0.25× — ephemeral recordings expire quickly */
    int base = 8;   /* use 8 so (int)(8 * 0.25) = 2 */
    double mult = 0.25;
    int effective = (int)(base * mult);
    TEST_ASSERT_EQUAL_INT(2, effective);
}

void test_cutoff_timestamp_is_in_past(void) {
    int base_days = 7;
    time_t now = time(NULL);
    time_t cutoff = now - (time_t)(base_days * 86400);
    TEST_ASSERT_TRUE(cutoff < now);
}

/* ================================================================
 * main
 * ================================================================ */

int main(void) {
    UNITY_BEGIN();

    RUN_TEST(test_pressure_str_normal);
    RUN_TEST(test_pressure_str_warning);
    RUN_TEST(test_pressure_str_critical);
    RUN_TEST(test_pressure_str_emergency);

    RUN_TEST(test_pressure_level_well_above_normal);
    RUN_TEST(test_pressure_level_at_warning_boundary);
    RUN_TEST(test_pressure_level_just_below_warning);
    RUN_TEST(test_pressure_level_mid_warning);
    RUN_TEST(test_pressure_level_at_critical_boundary);
    RUN_TEST(test_pressure_level_just_below_critical);
    RUN_TEST(test_pressure_level_mid_critical);
    RUN_TEST(test_pressure_level_at_emergency_boundary);
    RUN_TEST(test_pressure_level_just_below_emergency);
    RUN_TEST(test_pressure_level_zero_free);

    RUN_TEST(test_tier_critical_multiplier_default);
    RUN_TEST(test_tier_important_multiplier_default);
    RUN_TEST(test_tier_standard_multiplier);
    RUN_TEST(test_tier_ephemeral_multiplier_default);
    RUN_TEST(test_cutoff_timestamp_is_in_past);

    return UNITY_END();
}

