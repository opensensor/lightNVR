/**
 * @file test_detection_result_structures.c
 * @brief Layer 1 — validate detection_result_t / detection_t struct constants
 *
 * No heavy dependencies required; all tested code is in static header definitions.
 */

#include <string.h>
#include <stddef.h>

#include "unity.h"
#include "video/detection_result.h"

void setUp(void)    {}
void tearDown(void) {}

/* MAX_DETECTIONS is 20 */
void test_max_detections_value(void) {
    TEST_ASSERT_EQUAL_INT(20, MAX_DETECTIONS);
}

/* MAX_LABEL_LENGTH is 32 */
void test_max_label_length_value(void) {
    TEST_ASSERT_EQUAL_INT(32, MAX_LABEL_LENGTH);
}

/* MAX_ZONE_ID_LENGTH is 32 */
void test_max_zone_id_length_value(void) {
    TEST_ASSERT_EQUAL_INT(32, MAX_ZONE_ID_LENGTH);
}

/* detection_result_t with zero count */
void test_detection_result_zero_count(void) {
    detection_result_t r;
    memset(&r, 0, sizeof(r));
    TEST_ASSERT_EQUAL_INT(0, r.count);
}

/* detection_result_t with maximum count */
void test_detection_result_max_count(void) {
    detection_result_t r;
    memset(&r, 0, sizeof(r));
    r.count = MAX_DETECTIONS;
    for (int i = 0; i < MAX_DETECTIONS; i++) {
        snprintf(r.detections[i].label, MAX_LABEL_LENGTH, "obj%d", i);
        r.detections[i].confidence = 0.5f;
        r.detections[i].x         = 0.0f;
        r.detections[i].y         = 0.0f;
        r.detections[i].width     = 1.0f;
        r.detections[i].height    = 1.0f;
        r.detections[i].track_id  = i;
    }
    TEST_ASSERT_EQUAL_INT(MAX_DETECTIONS, r.count);
}

/* confidence range 0.0 to 1.0 */
void test_detection_confidence_range(void) {
    detection_t d;
    memset(&d, 0, sizeof(d));

    d.confidence = 0.0f;
    TEST_ASSERT_FLOAT_WITHIN(0.001f, 0.0f, d.confidence);

    d.confidence = 1.0f;
    TEST_ASSERT_FLOAT_WITHIN(0.001f, 1.0f, d.confidence);

    d.confidence = 0.75f;
    TEST_ASSERT_FLOAT_WITHIN(0.001f, 0.75f, d.confidence);
}

/* normalized bounding box */
void test_detection_bbox_normalized(void) {
    detection_t d;
    memset(&d, 0, sizeof(d));
    d.x = 0.1f; d.y = 0.2f; d.width = 0.3f; d.height = 0.4f;

    TEST_ASSERT_FLOAT_WITHIN(0.001f, 0.1f, d.x);
    TEST_ASSERT_FLOAT_WITHIN(0.001f, 0.2f, d.y);
    TEST_ASSERT_FLOAT_WITHIN(0.001f, 0.3f, d.width);
    TEST_ASSERT_FLOAT_WITHIN(0.001f, 0.4f, d.height);
}

/* label fits in MAX_LABEL_LENGTH - 1 chars */
void test_detection_label_max_length(void) {
    detection_t d;
    memset(&d, 0, sizeof(d));
    /* Fill MAX_LABEL_LENGTH-1 chars */
    memset(d.label, 'a', MAX_LABEL_LENGTH - 1);
    d.label[MAX_LABEL_LENGTH - 1] = '\0';
    TEST_ASSERT_EQUAL_INT(MAX_LABEL_LENGTH - 1, (int)strlen(d.label));
}

/* track_id of -1 means untracked */
void test_detection_track_id_untracked(void) {
    detection_t d;
    memset(&d, 0, sizeof(d));
    d.track_id = -1;
    TEST_ASSERT_EQUAL_INT(-1, d.track_id);
}

/* zone_id empty by default */
void test_detection_zone_id_empty(void) {
    detection_t d;
    memset(&d, 0, sizeof(d));
    TEST_ASSERT_EQUAL_INT(0, (int)strlen(d.zone_id));
}

/* zone_id fits in MAX_ZONE_ID_LENGTH - 1 chars */
void test_detection_zone_id_max_length(void) {
    detection_t d;
    memset(&d, 0, sizeof(d));
    memset(d.zone_id, 'z', MAX_ZONE_ID_LENGTH - 1);
    d.zone_id[MAX_ZONE_ID_LENGTH - 1] = '\0';
    TEST_ASSERT_EQUAL_INT(MAX_ZONE_ID_LENGTH - 1, (int)strlen(d.zone_id));
}

/* struct size sanity */
void test_detection_result_struct_size(void) {
    /* Just verifies the struct can be allocated and is non-trivial */
    TEST_ASSERT_GREATER_THAN(sizeof(int), sizeof(detection_result_t));
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_max_detections_value);
    RUN_TEST(test_max_label_length_value);
    RUN_TEST(test_max_zone_id_length_value);
    RUN_TEST(test_detection_result_zero_count);
    RUN_TEST(test_detection_result_max_count);
    RUN_TEST(test_detection_confidence_range);
    RUN_TEST(test_detection_bbox_normalized);
    RUN_TEST(test_detection_label_max_length);
    RUN_TEST(test_detection_track_id_untracked);
    RUN_TEST(test_detection_zone_id_empty);
    RUN_TEST(test_detection_zone_id_max_length);
    RUN_TEST(test_detection_result_struct_size);
    return UNITY_END();
}

