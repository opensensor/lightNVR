/**
 * @file test_config.c
 * @brief Layer 2 unit tests — config loading and validation
 *
 * Tests load_default_config() for sane defaults and validate_config()
 * for rejection of invalid values (bad port, empty paths, bad buffer).
 * Links against lightnvr_lib so the logger is available for error paths.
 */

#define _POSIX_C_SOURCE 200809L
#define _GNU_SOURCE

#include <stdio.h>
#include <string.h>

#include "unity.h"
#include "core/config.h"
#include "core/logger.h"

/* ---- Unity boilerplate ---- */
void setUp(void)    {}
void tearDown(void) {}

/* ================================================================
 * load_default_config
 * ================================================================ */

void test_default_config_web_port(void) {
    config_t cfg;
    load_default_config(&cfg);
    TEST_ASSERT_EQUAL_INT(8080, cfg.web_port);
}

void test_default_config_log_level(void) {
    config_t cfg;
    load_default_config(&cfg);
    TEST_ASSERT_EQUAL_INT(LOG_LEVEL_INFO, cfg.log_level);
}

void test_default_config_retention_days(void) {
    config_t cfg;
    load_default_config(&cfg);
    TEST_ASSERT_EQUAL_INT(30, cfg.retention_days);
}

void test_default_config_buffer_size(void) {
    config_t cfg;
    load_default_config(&cfg);
    TEST_ASSERT_GREATER_THAN_INT(0, cfg.buffer_size);
}

void test_default_config_storage_path_nonempty(void) {
    config_t cfg;
    load_default_config(&cfg);
    TEST_ASSERT_GREATER_THAN(0, (int)strlen(cfg.storage_path));
}

void test_default_config_db_path_nonempty(void) {
    config_t cfg;
    load_default_config(&cfg);
    TEST_ASSERT_GREATER_THAN(0, (int)strlen(cfg.db_path));
}

void test_default_config_models_path_nonempty(void) {
    config_t cfg;
    load_default_config(&cfg);
    TEST_ASSERT_GREATER_THAN(0, (int)strlen(cfg.models_path));
}

void test_default_config_null_safe(void) {
    /* Should not crash when passed NULL */
    load_default_config(NULL);
    TEST_PASS();
}

/* ================================================================
 * validate_config
 * ================================================================ */

void test_validate_config_valid_defaults(void) {
    config_t cfg;
    load_default_config(&cfg);
    TEST_ASSERT_EQUAL_INT(0, validate_config(&cfg));
}

void test_validate_config_null(void) {
    TEST_ASSERT_NOT_EQUAL(0, validate_config(NULL));
}

void test_validate_config_empty_storage_path(void) {
    config_t cfg;
    load_default_config(&cfg);
    cfg.storage_path[0] = '\0';
    TEST_ASSERT_NOT_EQUAL(0, validate_config(&cfg));
}

void test_validate_config_empty_models_path(void) {
    config_t cfg;
    load_default_config(&cfg);
    cfg.models_path[0] = '\0';
    TEST_ASSERT_NOT_EQUAL(0, validate_config(&cfg));
}

void test_validate_config_empty_db_path(void) {
    config_t cfg;
    load_default_config(&cfg);
    cfg.db_path[0] = '\0';
    TEST_ASSERT_NOT_EQUAL(0, validate_config(&cfg));
}

void test_validate_config_empty_web_root(void) {
    config_t cfg;
    load_default_config(&cfg);
    cfg.web_root[0] = '\0';
    TEST_ASSERT_NOT_EQUAL(0, validate_config(&cfg));
}

void test_validate_config_port_zero(void) {
    config_t cfg;
    load_default_config(&cfg);
    cfg.web_port = 0;
    TEST_ASSERT_NOT_EQUAL(0, validate_config(&cfg));
}

void test_validate_config_port_too_high(void) {
    config_t cfg;
    load_default_config(&cfg);
    cfg.web_port = 99999;
    TEST_ASSERT_NOT_EQUAL(0, validate_config(&cfg));
}

void test_validate_config_port_max_valid(void) {
    config_t cfg;
    load_default_config(&cfg);
    cfg.web_port = 65535;
    TEST_ASSERT_EQUAL_INT(0, validate_config(&cfg));
}

void test_validate_config_port_min_valid(void) {
    config_t cfg;
    load_default_config(&cfg);
    cfg.web_port = 1;
    TEST_ASSERT_EQUAL_INT(0, validate_config(&cfg));
}

void test_validate_config_buffer_size_zero(void) {
    config_t cfg;
    load_default_config(&cfg);
    cfg.buffer_size = 0;
    TEST_ASSERT_NOT_EQUAL(0, validate_config(&cfg));
}

/* ================================================================
 * main
 * ================================================================ */

int main(void) {
    init_logger();

    UNITY_BEGIN();

    RUN_TEST(test_default_config_web_port);
    RUN_TEST(test_default_config_log_level);
    RUN_TEST(test_default_config_retention_days);
    RUN_TEST(test_default_config_buffer_size);
    RUN_TEST(test_default_config_storage_path_nonempty);
    RUN_TEST(test_default_config_db_path_nonempty);
    RUN_TEST(test_default_config_models_path_nonempty);
    RUN_TEST(test_default_config_null_safe);

    RUN_TEST(test_validate_config_valid_defaults);
    RUN_TEST(test_validate_config_null);
    RUN_TEST(test_validate_config_empty_storage_path);
    RUN_TEST(test_validate_config_empty_models_path);
    RUN_TEST(test_validate_config_empty_db_path);
    RUN_TEST(test_validate_config_empty_web_root);
    RUN_TEST(test_validate_config_port_zero);
    RUN_TEST(test_validate_config_port_too_high);
    RUN_TEST(test_validate_config_port_max_valid);
    RUN_TEST(test_validate_config_port_min_valid);
    RUN_TEST(test_validate_config_buffer_size_zero);

    int result = UNITY_END();
    shutdown_logger();
    return result;
}

