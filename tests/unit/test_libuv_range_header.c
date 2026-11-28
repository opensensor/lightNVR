#include <stdint.h>
#include "unity.h"
#include "web/libuv_connection.h"

void setUp(void) {}
void tearDown(void) {}

static void test_valid_ranges(void) {
    size_t start = 0, end = 0;
    TEST_ASSERT_TRUE(libuv_parse_range_header("bytes=100-199", 1500, &start, &end));
    TEST_ASSERT_EQUAL_UINT(100, start);
    TEST_ASSERT_EQUAL_UINT(199, end);
    TEST_ASSERT_TRUE(libuv_parse_range_header("bytes=-100", 1500, &start, &end));
    TEST_ASSERT_EQUAL_UINT(1400, start);
    TEST_ASSERT_EQUAL_UINT(1499, end);
    TEST_ASSERT_TRUE(libuv_parse_range_header("bytes=1400-", 1500, &start, &end));
    TEST_ASSERT_EQUAL_UINT(1400, start);
    TEST_ASSERT_EQUAL_UINT(1499, end);
}

static void test_malformed_and_empty_ranges(void) {
    size_t start = 0, end = 0;
    const char *invalid[] = {
        "bytes=", "bytes=-", "bytes=-0", "bytes=+1-2", "bytes=1-+2",
        "bytes=1-2junk", "bytes=1-2,3-4", "bytes=9-3",
        "bytes=18446744073709551616-", "bytes=-18446744073709551616"
    };
    for (size_t i = 0; i < sizeof(invalid) / sizeof(invalid[0]); i++)
        TEST_ASSERT_FALSE(libuv_parse_range_header(invalid[i], 1500, &start, &end));
    TEST_ASSERT_FALSE(libuv_parse_range_header("bytes=0-", 0, &start, &end));
    TEST_ASSERT_FALSE(libuv_parse_range_header("bytes=0-1", 1500, NULL, &end));
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_valid_ranges);
    RUN_TEST(test_malformed_and_empty_ranges);
    return UNITY_END();
}
