/* INT_01 L2: UUIDv5 message IDs, base64url and the opaque history cursor
 * (contract sections 5.4 and 7.4). */
#define _POSIX_C_SOURCE 200809L
#include <stdio.h>
#include <string.h>
#include "unity.h"
#include "database/db_observations.h"
#include "utils/base64.h"
#include "utils/uuid.h"

void setUp(void) {}
void tearDown(void) {}

#define EPOCH "66666666-6666-4666-8666-666666666666"
#define CONTRACT_CURSOR \
    "eyJ2IjoxLCJzZXEiOjEwNDIsImVwb2NoIjoiNjY2NjY2NjYtNjY2Ni00NjY2LTg2NjYtNjY2NjY2NjY2NjY2In0"

static void test_uuid_v5_matches_rfc_and_contract_vectors(void) {
    char out[LIGHTNVR_UUID_STRING_SIZE];
    /* The frozen namespace is itself UUIDv5(NAMESPACE_URL, ...). */
    const char *url = "https://lightnvr.io/ns/observation-change/v1";
    TEST_ASSERT_EQUAL_INT(0, lightnvr_uuid_generate_v5(
        "6ba7b811-9dad-11d1-80b4-00c04fd430c8", url, strlen(url), out));
    TEST_ASSERT_EQUAL_STRING(OBS_MESSAGE_ID_NAMESPACE, out);
    /* Python: uuid.uuid5(uuid.NAMESPACE_DNS, "python.org") */
    TEST_ASSERT_EQUAL_INT(0, lightnvr_uuid_generate_v5(
        "6BA7B810-9DAD-11D1-80B4-00C04FD430C8", "python.org", 10, out));
    TEST_ASSERT_EQUAL_STRING("886313e1-3b8a-5372-9b90-0c9aee199e5d", out);

    TEST_ASSERT_EQUAL_INT(0, observation_message_id(
        "urn:lightnvr:11111111-1111-4111-8111-111111111111",
        "55555555-5555-4555-8555-555555555555", 2, out));
    TEST_ASSERT_EQUAL_STRING("43ebdf02-b2d5-5403-a0ef-2a4e7d58ada9", out);
}

static void test_uuid_v5_rejects_bad_namespace(void) {
    char out[LIGHTNVR_UUID_STRING_SIZE] = "unchanged";
    TEST_ASSERT_EQUAL_INT(-1, lightnvr_uuid_generate_v5("not-a-uuid", "x", 1, out));
    TEST_ASSERT_EQUAL_INT(-1, lightnvr_uuid_generate_v5(NULL, "x", 1, out));
    TEST_ASSERT_EQUAL_INT(-1, lightnvr_uuid_generate_v5(OBS_MESSAGE_ID_NAMESPACE, NULL, 1, out));
    TEST_ASSERT_EQUAL_INT(-1, observation_message_id("urn:x", "bad", 1, out));
    TEST_ASSERT_EQUAL_INT(-1, observation_message_id(
        "urn:x", "55555555-5555-4555-8555-555555555555", 0, out));
}

static void test_base64url_round_trip_and_strictness(void) {
    const unsigned char bytes[] = {0xfb, 0xff, 0xfe, 0x00, 0x3e};
    char text[16];
    size_t length = 0;
    TEST_ASSERT_EQUAL_INT(0, base64url_encode(bytes, sizeof(bytes), text, sizeof(text), &length));
    TEST_ASSERT_EQUAL_STRING("-__-AD4", text);
    TEST_ASSERT_EQUAL_UINT(7, length);
    unsigned char decoded[16];
    TEST_ASSERT_EQUAL_INT(0, base64url_decode(text, strlen(text), decoded, sizeof(decoded), &length));
    TEST_ASSERT_EQUAL_UINT(sizeof(bytes), length);
    TEST_ASSERT_EQUAL_MEMORY(bytes, decoded, sizeof(bytes));
    /* Exact-size buffer works; one byte short fails. */
    TEST_ASSERT_EQUAL_INT(0, base64url_encode(bytes, sizeof(bytes), text, 8, NULL));
    TEST_ASSERT_EQUAL_INT(-1, base64url_encode(bytes, sizeof(bytes), text, 7, NULL));
    TEST_ASSERT_EQUAL_INT(-1, base64url_decode("-__-AD4", 7, decoded, 4, NULL));
    /* Standard alphabet, padding, bad length and non-canonical tail bits. */
    TEST_ASSERT_EQUAL_INT(-1, base64url_decode("+//+", 4, decoded, sizeof(decoded), NULL));
    TEST_ASSERT_EQUAL_INT(-1, base64url_decode("AA==", 4, decoded, sizeof(decoded), NULL));
    TEST_ASSERT_EQUAL_INT(-1, base64url_decode("AAAAA", 5, decoded, sizeof(decoded), NULL));
    TEST_ASSERT_EQUAL_INT(-1, base64url_decode("AB", 2, decoded, sizeof(decoded), NULL));
    TEST_ASSERT_EQUAL_INT(-1, base64url_decode("A A=", 4, decoded, sizeof(decoded), NULL));
    TEST_ASSERT_EQUAL_INT(0, base64url_decode("", 0, decoded, sizeof(decoded), &length));
    TEST_ASSERT_EQUAL_UINT(0, length);
}

static void test_cursor_encoding_matches_contract_example(void) {
    char cursor[OBS_CURSOR_SIZE];
    TEST_ASSERT_EQUAL_INT(0, observation_cursor_encode(1042, EPOCH, cursor, sizeof(cursor)));
    TEST_ASSERT_EQUAL_STRING(CONTRACT_CURSOR, cursor);
    int64_t seq = -1;
    char epoch[OBS_UUID_SIZE];
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, observation_cursor_decode(cursor, &seq, epoch));
    TEST_ASSERT_EQUAL_INT64(1042, seq);
    TEST_ASSERT_EQUAL_STRING(EPOCH, epoch);
    TEST_ASSERT_EQUAL_INT(0, observation_cursor_encode(0, EPOCH, cursor, sizeof(cursor)));
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, observation_cursor_decode(cursor, &seq, epoch));
    TEST_ASSERT_EQUAL_INT64(0, seq);
}

static void encode_json(const char *json, char *out, size_t size) {
    TEST_ASSERT_EQUAL_INT(0, base64url_encode((const unsigned char *)json, strlen(json),
                                              out, size, NULL));
}

static void test_cursor_rejects_malformed_values(void) {
    int64_t seq;
    char epoch[OBS_UUID_SIZE];
    char cursor[1024];
    TEST_ASSERT_EQUAL_INT(DB_OBS_INVALID_CURSOR, observation_cursor_decode("", &seq, epoch));
    TEST_ASSERT_EQUAL_INT(DB_OBS_INVALID_CURSOR, observation_cursor_decode("!!!!", &seq, epoch));
    TEST_ASSERT_EQUAL_INT(DB_OBS_INVALID_CURSOR, observation_cursor_decode("head", &seq, epoch));
    encode_json("{\"v\":2,\"seq\":1,\"epoch\":\"" EPOCH "\"}", cursor, sizeof(cursor));
    TEST_ASSERT_EQUAL_INT(DB_OBS_INVALID_CURSOR, observation_cursor_decode(cursor, &seq, epoch));
    encode_json("{\"v\":1,\"seq\":-1,\"epoch\":\"" EPOCH "\"}", cursor, sizeof(cursor));
    TEST_ASSERT_EQUAL_INT(DB_OBS_INVALID_CURSOR, observation_cursor_decode(cursor, &seq, epoch));
    encode_json("{\"v\":1,\"seq\":1.5,\"epoch\":\"" EPOCH "\"}", cursor, sizeof(cursor));
    TEST_ASSERT_EQUAL_INT(DB_OBS_INVALID_CURSOR, observation_cursor_decode(cursor, &seq, epoch));
    encode_json("{\"v\":1,\"seq\":1,\"epoch\":\"nope\"}", cursor, sizeof(cursor));
    TEST_ASSERT_EQUAL_INT(DB_OBS_INVALID_CURSOR, observation_cursor_decode(cursor, &seq, epoch));
    encode_json("{\"v\":1,\"epoch\":\"" EPOCH "\"}", cursor, sizeof(cursor));
    TEST_ASSERT_EQUAL_INT(DB_OBS_INVALID_CURSOR, observation_cursor_decode(cursor, &seq, epoch));
    encode_json("[1,2]", cursor, sizeof(cursor));
    TEST_ASSERT_EQUAL_INT(DB_OBS_INVALID_CURSOR, observation_cursor_decode(cursor, &seq, epoch));
    /* Over 256 characters is invalid even when it decodes. */
    char padded[512];
    snprintf(padded, sizeof(padded), "{\"v\":1,\"seq\":1,\"epoch\":\"" EPOCH "\",\"x\":\"%0200d\"}", 0);
    encode_json(padded, cursor, sizeof(cursor));
    TEST_ASSERT_TRUE(strlen(cursor) > 256);
    TEST_ASSERT_EQUAL_INT(DB_OBS_INVALID_CURSOR, observation_cursor_decode(cursor, &seq, epoch));
}

static void test_reason_enum_is_bounded_per_state(void) {
    TEST_ASSERT_TRUE(observation_reason_is_valid(OBS_ASSET_PENDING, NULL));
    TEST_ASSERT_TRUE(observation_reason_is_valid(OBS_ASSET_READY, ""));
    TEST_ASSERT_FALSE(observation_reason_is_valid(OBS_ASSET_READY, "write_failed"));
    const char *unavailable[] = {"queue_full", "budget_exhausted", "encode_failed",
                                 "write_failed", "interrupted", "no_matching_media",
                                 "disabled"};
    for (size_t i = 0; i < sizeof(unavailable) / sizeof(unavailable[0]); ++i) {
        TEST_ASSERT_TRUE(observation_reason_is_valid(OBS_ASSET_UNAVAILABLE, unavailable[i]));
        TEST_ASSERT_FALSE(observation_reason_is_valid(OBS_ASSET_EXPIRED, unavailable[i]));
    }
    TEST_ASSERT_TRUE(observation_reason_is_valid(OBS_ASSET_EXPIRED, "retention_expired"));
    TEST_ASSERT_TRUE(observation_reason_is_valid(OBS_ASSET_EXPIRED, "deleted"));
    TEST_ASSERT_FALSE(observation_reason_is_valid(OBS_ASSET_UNAVAILABLE, "deleted"));
    TEST_ASSERT_FALSE(observation_reason_is_valid(OBS_ASSET_UNAVAILABLE, NULL));
    TEST_ASSERT_FALSE(observation_reason_is_valid(OBS_ASSET_UNAVAILABLE, "Queue_Full"));
    TEST_ASSERT_FALSE(observation_reason_is_valid(OBS_ASSET_UNAVAILABLE, "made_up"));
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_uuid_v5_matches_rfc_and_contract_vectors);
    RUN_TEST(test_uuid_v5_rejects_bad_namespace);
    RUN_TEST(test_base64url_round_trip_and_strictness);
    RUN_TEST(test_cursor_encoding_matches_contract_example);
    RUN_TEST(test_cursor_rejects_malformed_values);
    RUN_TEST(test_reason_enum_is_bounded_per_state);
    return UNITY_END();
}
