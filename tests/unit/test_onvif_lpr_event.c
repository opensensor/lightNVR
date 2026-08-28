#include "unity.h"

#include <stdio.h>
#include <stdlib.h>

#include "video/onvif_event.h"

#ifndef TEST_SOURCE_DIR
#define TEST_SOURCE_DIR "."
#endif

void setUp(void) {}
void tearDown(void) {}

static char *read_fixture(const char *name) {
    char path[1024];
    snprintf(path, sizeof(path), "%s/fixtures/onvif/%s", TEST_SOURCE_DIR, name);
    FILE *file = fopen(path, "rb");
    TEST_ASSERT_NOT_NULL(file);
    TEST_ASSERT_EQUAL_INT(0, fseek(file, 0, SEEK_END));
    long size = ftell(file);
    TEST_ASSERT_GREATER_THAN(0, size);
    rewind(file);
    char *data = malloc((size_t)size + 1);
    TEST_ASSERT_NOT_NULL(data);
    TEST_ASSERT_EQUAL_size_t((size_t)size, fread(data, 1, (size_t)size, file));
    data[size] = '\0';
    fclose(file);
    return data;
}

static void test_parses_profile_m_license_plate_info(void) {
    char *xml = read_fixture("profile_m_lpr.xml");
    onvif_lpr_event_t events[2];
    int count = onvif_parse_lpr_events(xml, events, 2);
    free(xml);

    TEST_ASSERT_EQUAL_INT(1, count);
    TEST_ASSERT_EQUAL_INT(ONVIF_LPR_SOURCE_PROFILE_M, events[0].source);
    TEST_ASSERT_TRUE(events[0].asserted);
    TEST_ASSERT_EQUAL_STRING("TEST123", events[0].plate);
    TEST_ASSERT_TRUE(events[0].has_confidence);
    TEST_ASSERT_FLOAT_WITHIN(0.001f, 0.94f, events[0].confidence);
    TEST_ASSERT_EQUAL_STRING("USA", events[0].country);
    TEST_ASSERT_EQUAL_STRING("ZZ", events[0].region);
    TEST_ASSERT_EQUAL_STRING("private", events[0].plate_type);
    TEST_ASSERT_EQUAL_STRING("inbound", events[0].direction);
    TEST_ASSERT_EQUAL_STRING("2", events[0].lane);
    TEST_ASSERT_EQUAL_STRING("car", events[0].vehicle_type);
    TEST_ASSERT_EQUAL_STRING("blue", events[0].vehicle_color);
    TEST_ASSERT_EQUAL_STRING("object-42", events[0].object_id);
    TEST_ASSERT_EQUAL_STRING("frame-redacted-1", events[0].correlation_id);
    TEST_ASSERT_EQUAL_INT64(1787920496789LL, events[0].observed_at_ms);
    TEST_ASSERT_TRUE(events[0].has_bounding_box);
    TEST_ASSERT_FLOAT_WITHIN(0.001f, 0.10f, events[0].bbox_left);
    TEST_ASSERT_FLOAT_WITHIN(0.001f, 0.50f, events[0].bbox_bottom);
}

static void test_parses_vendor_simple_items_and_ignores_clear(void) {
    char *xml = read_fixture("vendor_lpr_simple_items.xml");
    onvif_lpr_event_t events[2];
    int count = onvif_parse_lpr_events(xml, events, 2);
    free(xml);

    TEST_ASSERT_EQUAL_INT(1, count);
    TEST_ASSERT_EQUAL_INT(ONVIF_LPR_SOURCE_VENDOR_TOPIC, events[0].source);
    TEST_ASSERT_EQUAL_STRING("SAMPLE9", events[0].plate);
    TEST_ASSERT_FLOAT_WITHIN(0.001f, 0.87f, events[0].confidence);
    TEST_ASSERT_EQUAL_STRING("US", events[0].country);
    TEST_ASSERT_EQUAL_STRING("YY", events[0].region);
    TEST_ASSERT_EQUAL_STRING("north", events[0].lane);
}

static void test_non_lpr_and_malformed_xml_do_not_produce_reads(void) {
    const char *motion =
        "<Envelope><NotificationMessage><Topic>tns1:RuleEngine/CellMotionDetector/Motion</Topic>"
        "<Message><Data><SimpleItem Name=\"Plate\" Value=\"NOPE1\"/>"
        "</Data></Message></NotificationMessage></Envelope>";
    onvif_lpr_event_t event;
    TEST_ASSERT_EQUAL_INT(0, onvif_parse_lpr_events(motion, &event, 1));
    const char *cleared =
        "<NotificationMessage><Topic>tns1:RuleEngine/Recognition/LicensePlate</Topic>"
        "<Message PropertyOperation=\"Deleted\"><Data>"
        "<SimpleItem Name=\"PlateNumber\" Value=\"CLEARED1\"/>"
        "</Data></Message></NotificationMessage>";
    TEST_ASSERT_EQUAL_INT(0, onvif_parse_lpr_events(cleared, &event, 1));
    TEST_ASSERT_EQUAL_INT(-1, onvif_parse_lpr_events("<broken", &event, 1));
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_parses_profile_m_license_plate_info);
    RUN_TEST(test_parses_vendor_simple_items_and_ignores_clear);
    RUN_TEST(test_non_lpr_and_malformed_xml_do_not_produce_reads);
    return UNITY_END();
}
