#define _POSIX_C_SOURCE 200809L

#include "unity.h"

#include <sqlite3.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include "database/db_core.h"
#include "database/db_lpr_reads.h"
#include "utils/lpr_crypto.h"

static char db_path[256];
static const char *test_key =
    "000102030405060708090a0b0c0d0e0f"
    "101112131415161718191a1b1c1d1e1f";

void setUp(void) {
    sqlite3_exec(get_db_handle(), "DELETE FROM lpr_reads;", NULL, NULL, NULL);
    setenv("LIGHTNVR_LPR_MASTER_KEY_HEX", test_key, 1);
}
void tearDown(void) {}

static lpr_read_input_t sample_read(int64_t timestamp) {
    lpr_read_input_t input;
    memset(&input, 0, sizeof(input));
    snprintf(input.camera_uuid, sizeof(input.camera_uuid), "%s",
             "11111111-1111-4111-8111-111111111111");
    snprintf(input.stream_name, sizeof(input.stream_name), "%s", "drive");
    input.observed_at_ms = timestamp;
    snprintf(input.source, sizeof(input.source), "%s", "onvif_profile_m");
    snprintf(input.vendor_topic, sizeof(input.vendor_topic), "%s",
             "tns1:RuleEngine/Recognition/LicensePlate");
    snprintf(input.plate, sizeof(input.plate), "%s", "test-123");
    input.has_confidence = true;
    input.confidence = 0.91f;
    snprintf(input.country, sizeof(input.country), "%s", "USA");
    snprintf(input.direction, sizeof(input.direction), "%s", "inbound");
    snprintf(input.object_id, sizeof(input.object_id), "%s", "object-1");
    return input;
}

static lpr_read_query_t query(lpr_match_mode_t mode, const char *plate) {
    lpr_read_query_t value;
    memset(&value, 0, sizeof(value));
    value.start_at_ms = 1700000000000LL;
    value.end_at_ms = 1800000000000LL;
    value.match_mode = mode;
    if (plate) snprintf(value.plate_query, sizeof(value.plate_query), "%s", plate);
    value.limit = 20;
    return value;
}

static void test_crypto_round_trip_and_tamper_detection(void) {
    const char *plaintext = "TEST123";
    const char *aad = "camera|time|source";
    uint8_t nonce[LPR_CRYPTO_NONCE_SIZE];
    uint8_t ciphertext[64];
    uint8_t tag[LPR_CRYPTO_TAG_SIZE];
    size_t length = 0;
    char output[64];
    TEST_ASSERT_EQUAL_INT(0, lpr_crypto_encrypt(
        plaintext, aad, strlen(aad), nonce, ciphertext, sizeof(ciphertext),
        &length, tag));
    TEST_ASSERT_EQUAL_INT(0, lpr_crypto_decrypt(
        ciphertext, length, aad, strlen(aad), nonce, tag, output, sizeof(output)));
    TEST_ASSERT_EQUAL_STRING(plaintext, output);
    tag[0] ^= 0x01;
    TEST_ASSERT_EQUAL_INT(-1, lpr_crypto_decrypt(
        ciphertext, length, aad, strlen(aad), nonce, tag, output, sizeof(output)));
}

static void test_insert_exact_partial_dedupe_and_no_plaintext_column(void) {
    lpr_read_input_t input = sample_read(1787920496789LL);
    char uuid[LPR_READ_UUID_SIZE];
    TEST_ASSERT_EQUAL_INT(0, db_lpr_read_insert(&input, uuid));
    TEST_ASSERT_EQUAL_INT(36, (int)strlen(uuid));
    TEST_ASSERT_EQUAL_INT(1, db_lpr_read_insert(&input, NULL));
    input.observed_at_ms += 1000;
    TEST_ASSERT_EQUAL_INT(1, db_lpr_read_insert(&input, NULL));

    char owner[CAMERA_UUID_STRING_SIZE];
    TEST_ASSERT_EQUAL_INT(0, db_lpr_read_get_camera_uuid(uuid, owner));
    TEST_ASSERT_EQUAL_STRING(input.camera_uuid, owner);

    lpr_read_t reads[20];
    lpr_read_query_t exact = query(LPR_MATCH_EXACT, "TEST 123");
    TEST_ASSERT_EQUAL_INT(1, db_lpr_reads_search(&exact, reads, 20));
    TEST_ASSERT_EQUAL_STRING("TEST123", reads[0].read.plate);
    TEST_ASSERT_FLOAT_WITHIN(0.001f, 0.91f, reads[0].read.confidence);

    lpr_read_query_t partial = query(LPR_MATCH_PARTIAL, "st-12");
    TEST_ASSERT_EQUAL_INT(1, db_lpr_reads_search(&partial, reads, 20));
    TEST_ASSERT_EQUAL_STRING("TEST123", reads[0].read.plate);

    sqlite3_stmt *stmt = NULL;
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_prepare_v2(
        get_db_handle(),
        "SELECT COUNT(*) FROM lpr_reads WHERE "
        "instr(CAST(plate_ciphertext AS TEXT), 'TEST123') > 0;",
        -1, &stmt, NULL));
    TEST_ASSERT_EQUAL_INT(SQLITE_ROW, sqlite3_step(stmt));
    TEST_ASSERT_EQUAL_INT(0, sqlite3_column_int(stmt, 0));
    sqlite3_finalize(stmt);
}

static void test_time_scope_key_failure_delete_and_prune(void) {
    lpr_read_input_t input = sample_read(1787920496789LL);
    char uuid[LPR_READ_UUID_SIZE];
    TEST_ASSERT_EQUAL_INT(0, db_lpr_read_insert(&input, uuid));

    lpr_read_t reads[2];
    lpr_read_query_t bad = query(LPR_MATCH_PARTIAL, "TE");
    TEST_ASSERT_EQUAL_INT(-1, db_lpr_reads_search(&bad, reads, 2));
    bad = query(LPR_MATCH_NONE, NULL);
    bad.start_at_ms = 0;
    TEST_ASSERT_EQUAL_INT(-1, db_lpr_reads_search(&bad, reads, 2));

    unsetenv("LIGHTNVR_LPR_MASTER_KEY_HEX");
    lpr_read_query_t all = query(LPR_MATCH_NONE, NULL);
    TEST_ASSERT_EQUAL_INT(-1, db_lpr_reads_search(&all, reads, 2));
    setenv("LIGHTNVR_LPR_MASTER_KEY_HEX", test_key, 1);

    TEST_ASSERT_EQUAL_INT(0, db_lpr_read_delete(uuid));
    input.observed_at_ms++;
    TEST_ASSERT_EQUAL_INT(0, db_lpr_read_insert(&input, NULL));
    TEST_ASSERT_EQUAL_INT(1, db_lpr_reads_prune(2000000000000LL, 10));
}

int main(void) {
    snprintf(db_path, sizeof(db_path), "/tmp/lightnvr_lpr_%ld.db", (long)getpid());
    unlink(db_path);
    setenv("LIGHTNVR_MIGRATIONS_DIR", "./db/migrations", 1);
    setenv("LIGHTNVR_LPR_MASTER_KEY_HEX", test_key, 1);
    if (init_database(db_path) != 0) return 1;

    UNITY_BEGIN();
    RUN_TEST(test_crypto_round_trip_and_tamper_detection);
    RUN_TEST(test_insert_exact_partial_dedupe_and_no_plaintext_column);
    RUN_TEST(test_time_scope_key_failure_delete_and_prune);
    int result = UNITY_END();

    shutdown_database();
    unlink(db_path);
    return result;
}
