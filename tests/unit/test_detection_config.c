/**
 * @file test_detection_config.c
 * @brief Layer 2 unit tests — detection configuration
 *
 * Tests init_detection_config, get_detection_config, set_detection_config,
 * and validates default/embedded configuration values.
 */

#define _POSIX_C_SOURCE 200809L

#include <string.h>
#include "unity.h"
#include "video/detection_config.h"
#include "core/logger.h"

/* ---- Unity boilerplate ---- */
void setUp(void)    { init_detection_config(); }
void tearDown(void) {}

/* ================================================================
 * init_detection_config
 * ================================================================ */

void test_init_detection_config_succeeds(void) {
    int rc = init_detection_config();
    TEST_ASSERT_EQUAL_INT(0, rc);
}

/* ================================================================
 * get_detection_config
 * ================================================================ */

void test_get_detection_config_returns_non_null(void) {
    detection_config_t *cfg = get_detection_config();
    TEST_ASSERT_NOT_NULL(cfg);
}

void test_get_detection_config_concurrent_detections_positive(void) {
    detection_config_t *cfg = get_detection_config();
    TEST_ASSERT_NOT_NULL(cfg);
    TEST_ASSERT_GREATER_THAN_INT(0, cfg->concurrent_detections);
}

void test_get_detection_config_cnn_threshold_valid_range(void) {
    detection_config_t *cfg = get_detection_config();
    TEST_ASSERT_NOT_NULL(cfg);
    TEST_ASSERT_GREATER_OR_EQUAL(0.0f, cfg->threshold_cnn);
    TEST_ASSERT_LESS_OR_EQUAL(1.0f, cfg->threshold_cnn);
}

void test_get_detection_config_downscale_positive(void) {
    detection_config_t *cfg = get_detection_config();
    TEST_ASSERT_NOT_NULL(cfg);
    TEST_ASSERT_GREATER_THAN_INT(0, cfg->downscale_factor_cnn);
}

/* ================================================================
 * default_config external variable
 * ================================================================ */

void test_default_config_threshold_valid(void) {
    TEST_ASSERT_GREATER_OR_EQUAL(0.0f, default_config.threshold_cnn);
    TEST_ASSERT_LESS_OR_EQUAL(1.0f, default_config.threshold_cnn);
}

void test_default_config_concurrent_detections_positive(void) {
    TEST_ASSERT_GREATER_THAN_INT(0, default_config.concurrent_detections);
}

void test_default_config_downscale_cnn_positive(void) {
    TEST_ASSERT_GREATER_THAN_INT(0, default_config.downscale_factor_cnn);
}

/* ================================================================
 * embedded_config external variable
 * ================================================================ */

void test_embedded_config_concurrent_detections_small(void) {
    /* Embedded config should have reduced concurrency */
    TEST_ASSERT_GREATER_THAN_INT(0, embedded_config.concurrent_detections);
}

void test_embedded_config_threshold_valid(void) {
    TEST_ASSERT_GREATER_OR_EQUAL(0.0f, embedded_config.threshold_cnn);
    TEST_ASSERT_LESS_OR_EQUAL(1.0f, embedded_config.threshold_cnn);
}

/* ================================================================
 * set_detection_config round-trip
 * ================================================================ */

void test_set_detection_config_round_trip(void) {
    detection_config_t custom;
    memset(&custom, 0, sizeof(custom));
    custom.concurrent_detections = 3;
    custom.downscale_factor_cnn  = 2;
    custom.threshold_cnn         = 0.5f;
    custom.threshold_realnet     = 4.0f;
    custom.save_frames_for_debug = false;
    custom.buffer_pool_size      = 8;

    int rc = set_detection_config(&custom);
    TEST_ASSERT_EQUAL_INT(0, rc);

    detection_config_t *got = get_detection_config();
    TEST_ASSERT_NOT_NULL(got);
    TEST_ASSERT_EQUAL_INT(3, got->concurrent_detections);
    TEST_ASSERT_FLOAT_WITHIN(0.001f, 0.5f, got->threshold_cnn);
}

void test_set_detection_config_null_safe(void) {
    int rc = set_detection_config(NULL);
    TEST_ASSERT_NOT_EQUAL(0, rc);
}

/* ================================================================
 * main
 * ================================================================ */

int main(void) {
    init_logger();

    UNITY_BEGIN();

    RUN_TEST(test_init_detection_config_succeeds);

    RUN_TEST(test_get_detection_config_returns_non_null);
    RUN_TEST(test_get_detection_config_concurrent_detections_positive);
    RUN_TEST(test_get_detection_config_cnn_threshold_valid_range);
    RUN_TEST(test_get_detection_config_downscale_positive);

    RUN_TEST(test_default_config_threshold_valid);
    RUN_TEST(test_default_config_concurrent_detections_positive);
    RUN_TEST(test_default_config_downscale_cnn_positive);

    RUN_TEST(test_embedded_config_concurrent_detections_small);
    RUN_TEST(test_embedded_config_threshold_valid);

    RUN_TEST(test_set_detection_config_round_trip);
    RUN_TEST(test_set_detection_config_null_safe);

    int result = UNITY_END();
    shutdown_logger();
    return result;
}

