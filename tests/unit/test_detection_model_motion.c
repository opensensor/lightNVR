/**
 * @file test_detection_model_motion.c
 * @brief Layer 2 unit tests — built-in 'motion' detection model type (issue #93)
 *
 * Verifies that:
 *  - MODEL_TYPE_MOTION constant has the expected value
 *  - get_model_type() correctly identifies "motion" paths
 *  - get_model_type() still identifies other types correctly (regression)
 *  - load_detection_model("motion", ...) succeeds without a model file
 *  - The returned handle reports the correct type via get_model_type_from_handle()
 *  - unload_detection_model() on a motion handle does not crash
 */

#define _POSIX_C_SOURCE 200809L

#include <string.h>
#include "unity.h"
#include "core/logger.h"
#include "video/detection_model.h"

/* ---- Unity boilerplate ---- */
void setUp(void)    { init_detection_model_system(); }
void tearDown(void) {}

/* ================================================================
 * MODEL_TYPE_MOTION constant
 * ================================================================ */

void test_model_type_motion_constant_value(void) {
    TEST_ASSERT_EQUAL_STRING("motion", MODEL_TYPE_MOTION);
}

/* ================================================================
 * get_model_type — motion paths
 * ================================================================ */

void test_get_model_type_bare_motion_keyword(void) {
    /* Bare keyword as used in stream config: detection_model=motion */
    TEST_ASSERT_EQUAL_STRING(MODEL_TYPE_MOTION, get_model_type("motion"));
}

void test_get_model_type_absolute_path_ending_motion(void) {
    /* Path produced when models_path is prepended — should still be caught */
    TEST_ASSERT_EQUAL_STRING(MODEL_TYPE_MOTION,
        get_model_type("/var/lib/lightnvr/data/models/motion"));
}

/* ================================================================
 * get_model_type — regression: other special types still recognised
 * ================================================================ */

void test_get_model_type_api_detection_keyword(void) {
    TEST_ASSERT_EQUAL_STRING(MODEL_TYPE_API, get_model_type("api-detection"));
}

void test_get_model_type_http_url_is_api(void) {
    TEST_ASSERT_EQUAL_STRING(MODEL_TYPE_API,
        get_model_type("http://localhost:9001/api/v1/detect"));
}

void test_get_model_type_https_url_is_api(void) {
    TEST_ASSERT_EQUAL_STRING(MODEL_TYPE_API,
        get_model_type("https://example.com/detect"));
}

void test_get_model_type_onvif_keyword(void) {
    TEST_ASSERT_EQUAL_STRING(MODEL_TYPE_ONVIF, get_model_type("onvif"));
}

void test_get_model_type_tflite_extension(void) {
    TEST_ASSERT_EQUAL_STRING(MODEL_TYPE_TFLITE, get_model_type("/models/face.tflite"));
}

/* ================================================================
 * get_model_type — null / unknown paths
 * ================================================================ */

void test_get_model_type_null_returns_unknown(void) {
    TEST_ASSERT_EQUAL_STRING("unknown", get_model_type(NULL));
}

void test_get_model_type_unknown_extension_returns_unknown(void) {
    TEST_ASSERT_EQUAL_STRING("unknown", get_model_type("/models/model.xyz"));
}

void test_get_model_type_no_extension_returns_unknown(void) {
    /* A name with no extension and not a known keyword */
    TEST_ASSERT_EQUAL_STRING("unknown", get_model_type("/models/mymodel"));
}

/* ================================================================
 * load_detection_model — motion handle creation
 * ================================================================ */

void test_load_motion_model_returns_non_null(void) {
    detection_model_t handle = load_detection_model("motion", 0.5f);
    TEST_ASSERT_NOT_NULL(handle);
    unload_detection_model(handle);
}

void test_load_motion_model_handle_reports_correct_type(void) {
    detection_model_t handle = load_detection_model("motion", 0.5f);
    TEST_ASSERT_NOT_NULL(handle);
    TEST_ASSERT_EQUAL_STRING(MODEL_TYPE_MOTION, get_model_type_from_handle(handle));
    unload_detection_model(handle);
}

void test_load_motion_model_path_preserved_in_handle(void) {
    detection_model_t handle = load_detection_model("motion", 0.5f);
    TEST_ASSERT_NOT_NULL(handle);
    const char *path = get_model_path(handle);
    TEST_ASSERT_NOT_NULL(path);
    TEST_ASSERT_EQUAL_STRING("motion", path);
    unload_detection_model(handle);
}

/* ================================================================
 * unload_detection_model — motion handle cleanup
 * ================================================================ */

void test_unload_motion_model_does_not_crash(void) {
    detection_model_t handle = load_detection_model("motion", 0.5f);
    TEST_ASSERT_NOT_NULL(handle);
    /* Must not crash or leak — valgrind will catch leaks in CI */
    unload_detection_model(handle);
    TEST_PASS();
}

void test_unload_null_handle_does_not_crash(void) {
    /* Existing null-safety check should still hold */
    unload_detection_model(NULL);
    TEST_PASS();
}

/* ================================================================
 * main
 * ================================================================ */

int main(void) {
    init_logger();
    UNITY_BEGIN();

    RUN_TEST(test_model_type_motion_constant_value);

    RUN_TEST(test_get_model_type_bare_motion_keyword);
    RUN_TEST(test_get_model_type_absolute_path_ending_motion);

    RUN_TEST(test_get_model_type_api_detection_keyword);
    RUN_TEST(test_get_model_type_http_url_is_api);
    RUN_TEST(test_get_model_type_https_url_is_api);
    RUN_TEST(test_get_model_type_onvif_keyword);
    RUN_TEST(test_get_model_type_tflite_extension);

    RUN_TEST(test_get_model_type_null_returns_unknown);
    RUN_TEST(test_get_model_type_unknown_extension_returns_unknown);
    RUN_TEST(test_get_model_type_no_extension_returns_unknown);

    RUN_TEST(test_load_motion_model_returns_non_null);
    RUN_TEST(test_load_motion_model_handle_reports_correct_type);
    RUN_TEST(test_load_motion_model_path_preserved_in_handle);

    RUN_TEST(test_unload_motion_model_does_not_crash);
    RUN_TEST(test_unload_null_handle_does_not_crash);

    int result = UNITY_END();
    shutdown_logger();
    return result;
}

