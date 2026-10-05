/**
 * @file test_api_detection.c
 * @brief Layer 2 Unity tests for API detection URL validation, request
 *        construction, and response parsing for both wire formats.
 */

#define _POSIX_C_SOURCE 200809L

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <cjson/cJSON.h>

#include "unity.h"
#include "core/config.h"
#include "core/logger.h"
#include "video/api_detection.h"

void setUp(void) {
    TEST_ASSERT_EQUAL_INT(0, init_api_detection_system());
    snprintf(g_config.api_detection_format, sizeof(g_config.api_detection_format), "%s",
             API_DETECTION_FORMAT_NAME_LIGHT_OBJECT_DETECT);
    snprintf(g_config.api_detection_backend, sizeof(g_config.api_detection_backend), "onnx");
    snprintf(g_config.api_detection_detector_name, sizeof(g_config.api_detection_detector_name), "default");
}

void tearDown(void) {
    shutdown_api_detection_system();
}

static void assert_invalid_detection_url(const char *url) {
    detection_result_t result;
    memset(&result, 0xAB, sizeof(result));

    int rc = detect_objects_api(url, NULL, 0, 0, 0, &result, NULL, 0.5f, 0, 0);

    TEST_ASSERT_EQUAL_INT(-1, rc);
    TEST_ASSERT_EQUAL_INT(0, result.count);
}

static api_detection_options_t make_options(api_detection_format_t format,
                                            const char *backend,
                                            const char *detector_name) {
    api_detection_options_t options;
    memset(&options, 0, sizeof(options));
    options.format = format;
    if (backend) snprintf(options.backend, sizeof(options.backend), "%s", backend);
    if (detector_name) snprintf(options.detector_name, sizeof(options.detector_name), "%s", detector_name);
    return options;
}

/* ---- URL validation (pre-existing behaviour) ---------------------------- */

void test_detect_objects_api_rejects_url_with_space(void) {
    assert_invalid_detection_url("http://localhost:9001/api detect");
}

void test_detect_objects_api_rejects_url_with_userinfo(void) {
    assert_invalid_detection_url("http://user:pass@localhost:9001/detect");
}

void test_detect_objects_api_rejects_url_with_fragment(void) {
    assert_invalid_detection_url("http://localhost:9001/detect#frag");
}

void test_detect_objects_api_rejects_url_with_multiple_query_markers(void) {
    assert_invalid_detection_url("http://localhost:9001/detect?foo=1?bar=2");
}

void test_api_detection_uses_go2rtc_snapshot_only_without_decoded_frame(void) {
    TEST_ASSERT_TRUE(api_detection_should_use_go2rtc_snapshot(NULL, 0, 0, 0, "cam1"));

    const unsigned char frame_data[3] = {0, 0, 0};
    TEST_ASSERT_FALSE(api_detection_should_use_go2rtc_snapshot(frame_data, 1, 1, 3, "cam1"));
}

void test_api_detection_skips_go2rtc_snapshot_without_stream_name(void) {
    TEST_ASSERT_FALSE(api_detection_should_use_go2rtc_snapshot(NULL, 0, 0, 0, NULL));
    TEST_ASSERT_FALSE(api_detection_should_use_go2rtc_snapshot(NULL, 0, 0, 0, ""));
}

/* ---- Format names and options ------------------------------------------ */

void test_format_parse_accepts_known_names_and_aliases(void) {
    api_detection_format_t format = API_DETECTION_FORMAT_DOODS2;

    TEST_ASSERT_TRUE(api_detection_format_parse("light-object-detect", &format));
    TEST_ASSERT_EQUAL_INT(API_DETECTION_FORMAT_LIGHT_OBJECT_DETECT, format);
    TEST_ASSERT_TRUE(api_detection_format_parse("DOODS2", &format));
    TEST_ASSERT_EQUAL_INT(API_DETECTION_FORMAT_DOODS2, format);
    TEST_ASSERT_TRUE(api_detection_format_parse("  doods ", &format));
    TEST_ASSERT_EQUAL_INT(API_DETECTION_FORMAT_DOODS2, format);
    TEST_ASSERT_TRUE(api_detection_format_parse("lod", &format));
    TEST_ASSERT_EQUAL_INT(API_DETECTION_FORMAT_LIGHT_OBJECT_DETECT, format);

    format = API_DETECTION_FORMAT_DOODS2;
    TEST_ASSERT_FALSE(api_detection_format_parse("deepstack", &format));
    TEST_ASSERT_FALSE(api_detection_format_parse("", &format));
    TEST_ASSERT_FALSE(api_detection_format_parse(NULL, &format));
    TEST_ASSERT_FALSE(api_detection_format_parse("doods2", NULL));
    TEST_ASSERT_EQUAL_INT(API_DETECTION_FORMAT_DOODS2, format);

    TEST_ASSERT_EQUAL_STRING("light-object-detect",
                             api_detection_format_name(API_DETECTION_FORMAT_LIGHT_OBJECT_DETECT));
    TEST_ASSERT_EQUAL_STRING("doods2", api_detection_format_name(API_DETECTION_FORMAT_DOODS2));
}

void test_options_from_config_reads_global_settings(void) {
    snprintf(g_config.api_detection_format, sizeof(g_config.api_detection_format), "doods2");
    snprintf(g_config.api_detection_backend, sizeof(g_config.api_detection_backend), "tflite");
    snprintf(g_config.api_detection_detector_name, sizeof(g_config.api_detection_detector_name), "tensorflow");

    api_detection_options_t options;
    api_detection_options_from_config(&options);

    TEST_ASSERT_EQUAL_INT(API_DETECTION_FORMAT_DOODS2, options.format);
    TEST_ASSERT_EQUAL_STRING("tflite", options.backend);
    TEST_ASSERT_EQUAL_STRING("tensorflow", options.detector_name);
}

void test_options_from_config_falls_back_on_unknown_format(void) {
    snprintf(g_config.api_detection_format, sizeof(g_config.api_detection_format), "bogus");

    api_detection_options_t options;
    api_detection_options_from_config(&options);

    TEST_ASSERT_EQUAL_INT(API_DETECTION_FORMAT_LIGHT_OBJECT_DETECT, options.format);
}

void test_options_apply_json_overrides_only_known_keys(void) {
    api_detection_options_t options = make_options(API_DETECTION_FORMAT_LIGHT_OBJECT_DETECT, "onnx", "default");

    TEST_ASSERT_EQUAL_INT(0, api_detection_options_apply_json(&options,
        "{\"format\":\"doods2\",\"detector_name\":\"tensorflow\",\"unrelated\":1}"));
    TEST_ASSERT_EQUAL_INT(API_DETECTION_FORMAT_DOODS2, options.format);
    TEST_ASSERT_EQUAL_STRING("onnx", options.backend);
    TEST_ASSERT_EQUAL_STRING("tensorflow", options.detector_name);

    TEST_ASSERT_EQUAL_INT(0, api_detection_options_apply_json(&options, "{\"backend\":\"opencv\"}"));
    TEST_ASSERT_EQUAL_STRING("opencv", options.backend);
    TEST_ASSERT_EQUAL_INT(API_DETECTION_FORMAT_DOODS2, options.format);

    /* Empty, absent, and {} leave everything untouched. */
    TEST_ASSERT_EQUAL_INT(0, api_detection_options_apply_json(&options, "{}"));
    TEST_ASSERT_EQUAL_INT(0, api_detection_options_apply_json(&options, ""));
    TEST_ASSERT_EQUAL_INT(0, api_detection_options_apply_json(&options, NULL));
    TEST_ASSERT_EQUAL_INT(API_DETECTION_FORMAT_DOODS2, options.format);
    TEST_ASSERT_EQUAL_STRING("tensorflow", options.detector_name);
}

void test_options_apply_json_rejects_bad_input(void) {
    api_detection_options_t options = make_options(API_DETECTION_FORMAT_LIGHT_OBJECT_DETECT, "onnx", "default");

    TEST_ASSERT_EQUAL_INT(-1, api_detection_options_apply_json(&options, "{\"format\":\"deepstack\"}"));
    TEST_ASSERT_EQUAL_INT(API_DETECTION_FORMAT_LIGHT_OBJECT_DETECT, options.format);
    TEST_ASSERT_EQUAL_INT(-1, api_detection_options_apply_json(&options, "{\"format\":7}"));
    TEST_ASSERT_EQUAL_INT(-1, api_detection_options_apply_json(&options, "not json"));
    TEST_ASSERT_EQUAL_INT(-1, api_detection_options_apply_json(&options, "[1,2]"));
    TEST_ASSERT_EQUAL_INT(-1, api_detection_options_apply_json(NULL, "{}"));
}

/* ---- Request construction ---------------------------------------------- */

void test_build_request_url_appends_light_object_detect_params(void) {
    api_detection_options_t options = make_options(API_DETECTION_FORMAT_LIGHT_OBJECT_DETECT, "tflite", NULL);
    char url[512];

    TEST_ASSERT_EQUAL_INT(0, api_detection_build_request_url(url, sizeof(url),
        "http://host:8000/api/v1/detect", 0.35f, &options));
    TEST_ASSERT_EQUAL_STRING(
        "http://host:8000/api/v1/detect?backend=tflite&confidence_threshold=0.35&return_image=false", url);

    /* An existing query string is extended, not replaced. */
    TEST_ASSERT_EQUAL_INT(0, api_detection_build_request_url(url, sizeof(url),
        "http://host:8000/api/v1/detect?tiles=4", 0.35f, &options));
    TEST_ASSERT_EQUAL_STRING(
        "http://host:8000/api/v1/detect?tiles=4&backend=tflite&confidence_threshold=0.35&return_image=false", url);

    /* Non-positive thresholds fall back to 0.5 and unsafe backends to onnx. */
    api_detection_options_t unsafe = make_options(API_DETECTION_FORMAT_LIGHT_OBJECT_DETECT, "on nx&x=1", NULL);
    TEST_ASSERT_EQUAL_INT(0, api_detection_build_request_url(url, sizeof(url),
        "http://host/detect", -1.0f, &unsafe));
    TEST_ASSERT_EQUAL_STRING(
        "http://host/detect?backend=onnx&confidence_threshold=0.50&return_image=false", url);
}

void test_build_request_url_leaves_doods2_url_verbatim(void) {
    api_detection_options_t options = make_options(API_DETECTION_FORMAT_DOODS2, "tflite", "default");
    char url[512];

    TEST_ASSERT_EQUAL_INT(0, api_detection_build_request_url(url, sizeof(url),
        "http://doods:8080/detect", 0.35f, &options));
    TEST_ASSERT_EQUAL_STRING("http://doods:8080/detect", url);

    TEST_ASSERT_EQUAL_INT(0, api_detection_build_request_url(url, sizeof(url),
        "http://doods:8080/detect?x=1", 0.35f, &options));
    TEST_ASSERT_EQUAL_STRING("http://doods:8080/detect?x=1", url);
}

void test_build_request_url_uses_global_config_when_options_null(void) {
    snprintf(g_config.api_detection_format, sizeof(g_config.api_detection_format), "doods2");
    char url[512];

    TEST_ASSERT_EQUAL_INT(0, api_detection_build_request_url(url, sizeof(url),
        "http://doods:8080/detect", 0.35f, NULL));
    TEST_ASSERT_EQUAL_STRING("http://doods:8080/detect", url);
}

void test_build_request_url_rejects_small_buffer_and_bad_args(void) {
    api_detection_options_t options = make_options(API_DETECTION_FORMAT_LIGHT_OBJECT_DETECT, "onnx", NULL);
    char url[32];

    TEST_ASSERT_EQUAL_INT(-1, api_detection_build_request_url(url, sizeof(url),
        "http://host:8000/api/v1/detect", 0.5f, &options));
    TEST_ASSERT_EQUAL_INT(-1, api_detection_build_request_url(NULL, 0, "http://host", 0.5f, &options));
    TEST_ASSERT_EQUAL_INT(-1, api_detection_build_request_url(url, sizeof(url), NULL, 0.5f, &options));
    TEST_ASSERT_EQUAL_INT(-1, api_detection_build_request_url(url, sizeof(url), "http://ho st", 0.5f, &options));
}

void test_doods2_body_encodes_jpeg_threshold_and_detector(void) {
    const unsigned char jpeg[] = {'h', 'e', 'l', 'l', 'o'};
    api_detection_options_t options = make_options(API_DETECTION_FORMAT_DOODS2, NULL, "tensorflow");

    char *body = api_detection_build_doods2_body(jpeg, sizeof(jpeg), 0.4f, "Front \"Door\"", &options);
    TEST_ASSERT_NOT_NULL(body);

    cJSON *root = cJSON_Parse(body);
    TEST_ASSERT_NOT_NULL(root);
    TEST_ASSERT_EQUAL_STRING("Front \"Door\"", cJSON_GetObjectItem(root, "id")->valuestring);
    TEST_ASSERT_EQUAL_STRING("tensorflow", cJSON_GetObjectItem(root, "detector_name")->valuestring);
    TEST_ASSERT_EQUAL_STRING("aGVsbG8=", cJSON_GetObjectItem(root, "data")->valuestring);
    cJSON *detect = cJSON_GetObjectItem(root, "detect");
    TEST_ASSERT_NOT_NULL(detect);
    TEST_ASSERT_FLOAT_WITHIN(0.01f, 40.0f, (float)cJSON_GetObjectItem(detect, "*")->valuedouble);
    cJSON_Delete(root);
    free(body);
}

void test_doods2_body_uses_defaults_for_missing_fields(void) {
    const unsigned char jpeg[] = {0xFF, 0xD8, 0xFF};
    api_detection_options_t options = make_options(API_DETECTION_FORMAT_DOODS2, NULL, NULL);

    char *body = api_detection_build_doods2_body(jpeg, sizeof(jpeg), -1.0f, NULL, &options);
    TEST_ASSERT_NOT_NULL(body);

    cJSON *root = cJSON_Parse(body);
    TEST_ASSERT_NOT_NULL(root);
    TEST_ASSERT_EQUAL_STRING("lightnvr", cJSON_GetObjectItem(root, "id")->valuestring);
    TEST_ASSERT_EQUAL_STRING("default", cJSON_GetObjectItem(root, "detector_name")->valuestring);
    TEST_ASSERT_EQUAL_STRING("/9j/", cJSON_GetObjectItem(root, "data")->valuestring);
    TEST_ASSERT_FLOAT_WITHIN(0.01f, 50.0f,
        (float)cJSON_GetObjectItem(cJSON_GetObjectItem(root, "detect"), "*")->valuedouble);
    cJSON_Delete(root);
    free(body);

    TEST_ASSERT_NULL(api_detection_build_doods2_body(NULL, 0, 0.5f, NULL, &options));
    TEST_ASSERT_NULL(api_detection_build_doods2_body(jpeg, 0, 0.5f, NULL, &options));
}

/* ---- Response parsing -------------------------------------------------- */

void test_parse_response_light_object_detect_flat_and_nested_boxes(void) {
    const char *json =
        "{\"detections\":["
        "{\"label\":\"person\",\"confidence\":0.91,\"x_min\":0.1,\"y_min\":0.2,\"x_max\":0.4,\"y_max\":0.8},"
        "{\"label\":\"car\",\"confidence\":0.5,"
        "\"bounding_box\":{\"x_min\":0.5,\"y_min\":0.5,\"x_max\":1.0,\"y_max\":1.0},"
        "\"track_id\":7,\"zone_id\":\"drive\"},"
        "{\"label\":\"broken\",\"confidence\":0.9}"
        "]}";
    api_detection_options_t options = make_options(API_DETECTION_FORMAT_LIGHT_OBJECT_DETECT, "onnx", NULL);
    detection_result_t result;
    memset(&result, 0xAB, sizeof(result));

    TEST_ASSERT_EQUAL_INT(0, api_detection_parse_response(json, &options, &result));
    TEST_ASSERT_EQUAL_INT(2, result.count);

    TEST_ASSERT_EQUAL_STRING("person", result.detections[0].label);
    TEST_ASSERT_FLOAT_WITHIN(0.001f, 0.91f, result.detections[0].confidence);
    TEST_ASSERT_FLOAT_WITHIN(0.001f, 0.1f, result.detections[0].x);
    TEST_ASSERT_FLOAT_WITHIN(0.001f, 0.2f, result.detections[0].y);
    TEST_ASSERT_FLOAT_WITHIN(0.001f, 0.3f, result.detections[0].width);
    TEST_ASSERT_FLOAT_WITHIN(0.001f, 0.6f, result.detections[0].height);
    TEST_ASSERT_EQUAL_INT(-1, result.detections[0].track_id);
    TEST_ASSERT_EQUAL_STRING("", result.detections[0].zone_id);

    TEST_ASSERT_EQUAL_STRING("car", result.detections[1].label);
    TEST_ASSERT_FLOAT_WITHIN(0.001f, 0.5f, result.detections[1].x);
    TEST_ASSERT_FLOAT_WITHIN(0.001f, 0.5f, result.detections[1].width);
    TEST_ASSERT_EQUAL_INT(7, result.detections[1].track_id);
    TEST_ASSERT_EQUAL_STRING("drive", result.detections[1].zone_id);
}

void test_parse_response_doods2_maps_boxes_and_scales_confidence(void) {
    const char *json =
        "{\"id\":\"Front Door\",\"detections\":["
        "{\"top\":0.2,\"left\":0.1,\"bottom\":0.8,\"right\":0.4,\"label\":\"dog\",\"confidence\":90.0},"
        "{\"top\":0.0,\"left\":0.0,\"bottom\":1.0,\"right\":1.0,\"label\":\"person\",\"confidence\":120.0}"
        "],\"error\":\"\"}";
    api_detection_options_t options = make_options(API_DETECTION_FORMAT_DOODS2, NULL, "default");
    detection_result_t result;

    TEST_ASSERT_EQUAL_INT(0, api_detection_parse_response(json, &options, &result));
    TEST_ASSERT_EQUAL_INT(2, result.count);

    TEST_ASSERT_EQUAL_STRING("dog", result.detections[0].label);
    TEST_ASSERT_FLOAT_WITHIN(0.001f, 0.9f, result.detections[0].confidence);
    TEST_ASSERT_FLOAT_WITHIN(0.001f, 0.1f, result.detections[0].x);
    TEST_ASSERT_FLOAT_WITHIN(0.001f, 0.2f, result.detections[0].y);
    TEST_ASSERT_FLOAT_WITHIN(0.001f, 0.3f, result.detections[0].width);
    TEST_ASSERT_FLOAT_WITHIN(0.001f, 0.6f, result.detections[0].height);
    TEST_ASSERT_EQUAL_INT(-1, result.detections[0].track_id);

    /* Out-of-range confidence is clamped to the 0-1 scale. */
    TEST_ASSERT_FLOAT_WITHIN(0.001f, 1.0f, result.detections[1].confidence);
}

void test_parse_response_doods2_error_field_fails_request(void) {
    api_detection_options_t options = make_options(API_DETECTION_FORMAT_DOODS2, NULL, "default");
    detection_result_t result;

    TEST_ASSERT_EQUAL_INT(-1, api_detection_parse_response(
        "{\"detections\":[],\"error\":\"detector not found\"}", &options, &result));
    TEST_ASSERT_EQUAL_INT(0, result.count);
}

void test_parse_response_handles_null_missing_and_garbage(void) {
    api_detection_options_t options = make_options(API_DETECTION_FORMAT_DOODS2, NULL, "default");
    detection_result_t result;

    TEST_ASSERT_EQUAL_INT(0, api_detection_parse_response("{\"detections\":null}", &options, &result));
    TEST_ASSERT_EQUAL_INT(0, result.count);
    TEST_ASSERT_EQUAL_INT(0, api_detection_parse_response("{\"detections\":[]}", &options, &result));
    TEST_ASSERT_EQUAL_INT(0, result.count);
    TEST_ASSERT_EQUAL_INT(-1, api_detection_parse_response("{\"id\":\"x\"}", &options, &result));
    TEST_ASSERT_EQUAL_INT(-1, api_detection_parse_response("{\"detections\":\"nope\"}", &options, &result));
    TEST_ASSERT_EQUAL_INT(-1, api_detection_parse_response("garbage", &options, &result));
    TEST_ASSERT_EQUAL_INT(-1, api_detection_parse_response(NULL, &options, &result));
    TEST_ASSERT_EQUAL_INT(-1, api_detection_parse_response("{}", &options, NULL));
}

void test_parse_response_caps_at_max_detections(void) {
    char json[8192];
    size_t len = snprintf(json, sizeof(json), "{\"detections\":[");
    for (int i = 0; i < MAX_DETECTIONS + 5; i++) {
        len += snprintf(json + len, sizeof(json) - len,
                        "%s{\"label\":\"p\",\"confidence\":0.9,\"x_min\":0,\"y_min\":0,\"x_max\":1,\"y_max\":1}",
                        i ? "," : "");
        TEST_ASSERT_LESS_THAN(sizeof(json) - 8, len);
    }
    snprintf(json + len, sizeof(json) - len, "]}");

    api_detection_options_t options = make_options(API_DETECTION_FORMAT_LIGHT_OBJECT_DETECT, "onnx", NULL);
    detection_result_t result;
    TEST_ASSERT_EQUAL_INT(0, api_detection_parse_response(json, &options, &result));
    TEST_ASSERT_EQUAL_INT(MAX_DETECTIONS, result.count);
}

int main(void) {
    init_logger();
    UNITY_BEGIN();
    RUN_TEST(test_detect_objects_api_rejects_url_with_space);
    RUN_TEST(test_detect_objects_api_rejects_url_with_userinfo);
    RUN_TEST(test_detect_objects_api_rejects_url_with_fragment);
    RUN_TEST(test_detect_objects_api_rejects_url_with_multiple_query_markers);
    RUN_TEST(test_api_detection_uses_go2rtc_snapshot_only_without_decoded_frame);
    RUN_TEST(test_api_detection_skips_go2rtc_snapshot_without_stream_name);
    RUN_TEST(test_format_parse_accepts_known_names_and_aliases);
    RUN_TEST(test_options_from_config_reads_global_settings);
    RUN_TEST(test_options_from_config_falls_back_on_unknown_format);
    RUN_TEST(test_options_apply_json_overrides_only_known_keys);
    RUN_TEST(test_options_apply_json_rejects_bad_input);
    RUN_TEST(test_build_request_url_appends_light_object_detect_params);
    RUN_TEST(test_build_request_url_leaves_doods2_url_verbatim);
    RUN_TEST(test_build_request_url_uses_global_config_when_options_null);
    RUN_TEST(test_build_request_url_rejects_small_buffer_and_bad_args);
    RUN_TEST(test_doods2_body_encodes_jpeg_threshold_and_detector);
    RUN_TEST(test_doods2_body_uses_defaults_for_missing_fields);
    RUN_TEST(test_parse_response_light_object_detect_flat_and_nested_boxes);
    RUN_TEST(test_parse_response_doods2_maps_boxes_and_scales_confidence);
    RUN_TEST(test_parse_response_doods2_error_field_fails_request);
    RUN_TEST(test_parse_response_handles_null_missing_and_garbage);
    RUN_TEST(test_parse_response_caps_at_max_detections);
    return UNITY_END();
}
