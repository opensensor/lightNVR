/**
 * @file test_motion_trigger_parse.c
 * @brief Layer 2 — request-body parsing for POST /api/motion/trigger (#466)
 *
 * Covers motion_trigger_parse_objects() and motion_trigger_parse_tags(): the
 * `label` / `objects` / `confidence` / `tags` fields that let an external
 * detector say *what* it saw rather than just that something moved.
 *
 * These are the parts of the endpoint with real branching — mixed array forms,
 * confidence bounds, per-entry overrides, overflow caps — so they are tested
 * directly instead of through the HTTP handler, which would need a live server,
 * a configured stream and a database.
 */

#define _POSIX_C_SOURCE 200809L

#include <stdio.h>
#include <string.h>

#include <cjson/cJSON.h>

#include "unity.h"
#include "web/api_handlers_motion.h"

static char g_err[160];

/* Parse `json` and run it through motion_trigger_parse_objects(). */
static int parse_objects_str(const char *json, detection_result_t *result) {
    memset(result, 0, sizeof(*result));
    g_err[0] = '\0';
    cJSON *body = cJSON_Parse(json);
    TEST_ASSERT_NOT_NULL_MESSAGE(body, "test fixture JSON failed to parse");
    int rc = motion_trigger_parse_objects(body, result, g_err, sizeof(g_err));
    cJSON_Delete(body);
    return rc;
}

static int parse_tags_str(const char *json, char tags[][MAX_TAG_LENGTH]) {
    g_err[0] = '\0';
    cJSON *body = cJSON_Parse(json);
    TEST_ASSERT_NOT_NULL_MESSAGE(body, "test fixture JSON failed to parse");
    int rc = motion_trigger_parse_tags(body, tags, g_err, sizeof(g_err));
    cJSON_Delete(body);
    return rc;
}

void setUp(void)    {}
void tearDown(void) {}

/* =========================================================================
 * objects / label / confidence
 * ========================================================================= */

/* A plain trigger with no detection metadata stays empty — the endpoint must
 * keep working for the pre-#466 callers that only send stream/action. */
void test_no_metadata_yields_no_detections(void) {
    detection_result_t r;
    TEST_ASSERT_EQUAL_INT(0, parse_objects_str("{\"stream\":\"a\",\"action\":\"start\"}", &r));
    TEST_ASSERT_EQUAL_INT(0, r.count);
}

void test_label_only_defaults_to_full_confidence(void) {
    detection_result_t r;
    TEST_ASSERT_EQUAL_INT(0, parse_objects_str("{\"label\":\"person\"}", &r));
    TEST_ASSERT_EQUAL_INT(1, r.count);
    TEST_ASSERT_EQUAL_STRING("person", r.detections[0].label);
    TEST_ASSERT_EQUAL_FLOAT(1.0f, r.detections[0].confidence);
}

/* An external trigger has no bounding box, so the detection covers the whole
 * frame — same convention as an ONVIF smart event without coordinates. */
void test_detection_covers_whole_frame(void) {
    detection_result_t r;
    TEST_ASSERT_EQUAL_INT(0, parse_objects_str("{\"label\":\"person\"}", &r));
    TEST_ASSERT_EQUAL_FLOAT(0.0f, r.detections[0].x);
    TEST_ASSERT_EQUAL_FLOAT(0.0f, r.detections[0].y);
    TEST_ASSERT_EQUAL_FLOAT(1.0f, r.detections[0].width);
    TEST_ASSERT_EQUAL_FLOAT(1.0f, r.detections[0].height);
}

void test_top_level_confidence_applies_to_label(void) {
    detection_result_t r;
    TEST_ASSERT_EQUAL_INT(0, parse_objects_str("{\"label\":\"car\",\"confidence\":0.25}", &r));
    TEST_ASSERT_EQUAL_INT(1, r.count);
    TEST_ASSERT_EQUAL_FLOAT(0.25f, r.detections[0].confidence);
}

void test_objects_as_bare_strings(void) {
    detection_result_t r;
    TEST_ASSERT_EQUAL_INT(0, parse_objects_str("{\"objects\":[\"person\",\"vehicle\"]}", &r));
    TEST_ASSERT_EQUAL_INT(2, r.count);
    TEST_ASSERT_EQUAL_STRING("person", r.detections[0].label);
    TEST_ASSERT_EQUAL_STRING("vehicle", r.detections[1].label);
    TEST_ASSERT_EQUAL_FLOAT(1.0f, r.detections[1].confidence);
}

void test_objects_with_per_entry_confidence(void) {
    detection_result_t r;
    TEST_ASSERT_EQUAL_INT(0, parse_objects_str(
        "{\"objects\":[{\"label\":\"person\",\"confidence\":0.9},"
        "{\"label\":\"dog\",\"confidence\":0.1}]}", &r));
    TEST_ASSERT_EQUAL_INT(2, r.count);
    TEST_ASSERT_EQUAL_STRING("person", r.detections[0].label);
    TEST_ASSERT_EQUAL_FLOAT(0.9f, r.detections[0].confidence);
    TEST_ASSERT_EQUAL_STRING("dog", r.detections[1].label);
    TEST_ASSERT_EQUAL_FLOAT(0.1f, r.detections[1].confidence);
}

/* An entry without its own confidence inherits the top-level default. */
void test_objects_entry_inherits_top_level_confidence(void) {
    detection_result_t r;
    TEST_ASSERT_EQUAL_INT(0, parse_objects_str(
        "{\"confidence\":0.4,\"objects\":[{\"label\":\"person\"},"
        "{\"label\":\"dog\",\"confidence\":0.8}]}", &r));
    TEST_ASSERT_EQUAL_INT(2, r.count);
    TEST_ASSERT_EQUAL_FLOAT(0.4f, r.detections[0].confidence);
    TEST_ASSERT_EQUAL_FLOAT(0.8f, r.detections[1].confidence);
}

/* Mixed forms in one array — callers should not have to normalise. */
void test_objects_mixed_string_and_object_entries(void) {
    detection_result_t r;
    TEST_ASSERT_EQUAL_INT(0, parse_objects_str(
        "{\"objects\":[\"person\",{\"label\":\"vehicle\",\"confidence\":0.5}]}", &r));
    TEST_ASSERT_EQUAL_INT(2, r.count);
    TEST_ASSERT_EQUAL_STRING("person", r.detections[0].label);
    TEST_ASSERT_EQUAL_FLOAT(1.0f, r.detections[0].confidence);
    TEST_ASSERT_EQUAL_STRING("vehicle", r.detections[1].label);
    TEST_ASSERT_EQUAL_FLOAT(0.5f, r.detections[1].confidence);
}

void test_label_and_objects_combine(void) {
    detection_result_t r;
    TEST_ASSERT_EQUAL_INT(0, parse_objects_str(
        "{\"label\":\"motion\",\"objects\":[\"person\"]}", &r));
    TEST_ASSERT_EQUAL_INT(2, r.count);
    TEST_ASSERT_EQUAL_STRING("motion", r.detections[0].label);
    TEST_ASSERT_EQUAL_STRING("person", r.detections[1].label);
}

/* Over-long object lists are capped, not rejected: the motion event still
 * matters more than the tail of the list. */
void test_objects_beyond_max_are_ignored(void) {
    char json[2048];
    int n = snprintf(json, sizeof(json), "{\"objects\":[");
    for (int i = 0; i < MAX_DETECTIONS + 5; i++) {
        n += snprintf(json + n, sizeof(json) - n, "%s\"person\"", i ? "," : "");
    }
    snprintf(json + n, sizeof(json) - n, "]}");

    detection_result_t r;
    TEST_ASSERT_EQUAL_INT(0, parse_objects_str(json, &r));
    TEST_ASSERT_EQUAL_INT(MAX_DETECTIONS, r.count);
}

/* Labels longer than the column are truncated, not overflowed. */
void test_long_label_is_truncated(void) {
    detection_result_t r;
    TEST_ASSERT_EQUAL_INT(0, parse_objects_str(
        "{\"label\":\"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\"}", &r));
    TEST_ASSERT_EQUAL_INT(1, r.count);
    TEST_ASSERT_TRUE(strlen(r.detections[0].label) < MAX_LABEL_LENGTH);
}

/* ---- rejections ---- */

void test_confidence_above_one_is_rejected(void) {
    detection_result_t r;
    TEST_ASSERT_EQUAL_INT(-1, parse_objects_str("{\"label\":\"a\",\"confidence\":1.5}", &r));
    TEST_ASSERT_TRUE(g_err[0] != '\0');
}

void test_negative_confidence_is_rejected(void) {
    detection_result_t r;
    TEST_ASSERT_EQUAL_INT(-1, parse_objects_str("{\"label\":\"a\",\"confidence\":-0.1}", &r));
}

void test_non_numeric_confidence_is_rejected(void) {
    detection_result_t r;
    TEST_ASSERT_EQUAL_INT(-1, parse_objects_str("{\"label\":\"a\",\"confidence\":\"high\"}", &r));
}

void test_empty_label_is_rejected(void) {
    detection_result_t r;
    TEST_ASSERT_EQUAL_INT(-1, parse_objects_str("{\"label\":\"\"}", &r));
}

void test_non_string_label_is_rejected(void) {
    detection_result_t r;
    TEST_ASSERT_EQUAL_INT(-1, parse_objects_str("{\"label\":42}", &r));
}

void test_objects_not_an_array_is_rejected(void) {
    detection_result_t r;
    TEST_ASSERT_EQUAL_INT(-1, parse_objects_str("{\"objects\":\"person\"}", &r));
}

void test_objects_entry_of_wrong_type_is_rejected(void) {
    detection_result_t r;
    TEST_ASSERT_EQUAL_INT(-1, parse_objects_str("{\"objects\":[42]}", &r));
}

void test_objects_entry_without_label_is_rejected(void) {
    detection_result_t r;
    TEST_ASSERT_EQUAL_INT(-1, parse_objects_str("{\"objects\":[{\"confidence\":0.5}]}", &r));
}

void test_objects_entry_with_bad_confidence_is_rejected(void) {
    detection_result_t r;
    TEST_ASSERT_EQUAL_INT(-1, parse_objects_str(
        "{\"objects\":[{\"label\":\"person\",\"confidence\":2}]}", &r));
}

/* =========================================================================
 * tags
 * ========================================================================= */

void test_no_tags_field_yields_zero(void) {
    char tags[MOTION_MAX_TAGS][MAX_TAG_LENGTH];
    TEST_ASSERT_EQUAL_INT(0, parse_tags_str("{\"action\":\"start\"}", tags));
}

void test_tags_are_parsed_in_order(void) {
    char tags[MOTION_MAX_TAGS][MAX_TAG_LENGTH];
    TEST_ASSERT_EQUAL_INT(2, parse_tags_str("{\"tags\":[\"frigate\",\"front-gate\"]}", tags));
    TEST_ASSERT_EQUAL_STRING("frigate", tags[0]);
    TEST_ASSERT_EQUAL_STRING("front-gate", tags[1]);
}

void test_empty_tags_array_yields_zero(void) {
    char tags[MOTION_MAX_TAGS][MAX_TAG_LENGTH];
    TEST_ASSERT_EQUAL_INT(0, parse_tags_str("{\"tags\":[]}", tags));
}

/* The cap must not write past the caller's fixed-size buffer. */
void test_tags_beyond_max_are_capped(void) {
    char json[2048];
    int n = snprintf(json, sizeof(json), "{\"tags\":[");
    for (int i = 0; i < MOTION_MAX_TAGS + 4; i++) {
        n += snprintf(json + n, sizeof(json) - n, "%s\"t%d\"", i ? "," : "", i);
    }
    snprintf(json + n, sizeof(json) - n, "]}");

    char tags[MOTION_MAX_TAGS][MAX_TAG_LENGTH];
    TEST_ASSERT_EQUAL_INT(MOTION_MAX_TAGS, parse_tags_str(json, tags));
}

void test_tags_not_an_array_is_rejected(void) {
    char tags[MOTION_MAX_TAGS][MAX_TAG_LENGTH];
    TEST_ASSERT_EQUAL_INT(-1, parse_tags_str("{\"tags\":\"frigate\"}", tags));
    TEST_ASSERT_TRUE(g_err[0] != '\0');
}

void test_non_string_tag_is_rejected(void) {
    char tags[MOTION_MAX_TAGS][MAX_TAG_LENGTH];
    TEST_ASSERT_EQUAL_INT(-1, parse_tags_str("{\"tags\":[\"ok\",7]}", tags));
}

void test_empty_tag_is_rejected(void) {
    char tags[MOTION_MAX_TAGS][MAX_TAG_LENGTH];
    TEST_ASSERT_EQUAL_INT(-1, parse_tags_str("{\"tags\":[\"\"]}", tags));
}

int main(void) {
    UNITY_BEGIN();

    RUN_TEST(test_no_metadata_yields_no_detections);
    RUN_TEST(test_label_only_defaults_to_full_confidence);
    RUN_TEST(test_detection_covers_whole_frame);
    RUN_TEST(test_top_level_confidence_applies_to_label);
    RUN_TEST(test_objects_as_bare_strings);
    RUN_TEST(test_objects_with_per_entry_confidence);
    RUN_TEST(test_objects_entry_inherits_top_level_confidence);
    RUN_TEST(test_objects_mixed_string_and_object_entries);
    RUN_TEST(test_label_and_objects_combine);
    RUN_TEST(test_objects_beyond_max_are_ignored);
    RUN_TEST(test_long_label_is_truncated);

    RUN_TEST(test_confidence_above_one_is_rejected);
    RUN_TEST(test_negative_confidence_is_rejected);
    RUN_TEST(test_non_numeric_confidence_is_rejected);
    RUN_TEST(test_empty_label_is_rejected);
    RUN_TEST(test_non_string_label_is_rejected);
    RUN_TEST(test_objects_not_an_array_is_rejected);
    RUN_TEST(test_objects_entry_of_wrong_type_is_rejected);
    RUN_TEST(test_objects_entry_without_label_is_rejected);
    RUN_TEST(test_objects_entry_with_bad_confidence_is_rejected);

    RUN_TEST(test_no_tags_field_yields_zero);
    RUN_TEST(test_tags_are_parsed_in_order);
    RUN_TEST(test_empty_tags_array_yields_zero);
    RUN_TEST(test_tags_beyond_max_are_capped);
    RUN_TEST(test_tags_not_an_array_is_rejected);
    RUN_TEST(test_non_string_tag_is_rejected);
    RUN_TEST(test_empty_tag_is_rejected);

    return UNITY_END();
}
