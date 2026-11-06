/* SPDX-License-Identifier: GPL-3.0-or-later */
#define _POSIX_C_SOURCE 200809L
#include <errno.h>
#include <stdio.h>
#include <string.h>
#include <sys/random.h>
#include "unity.h"
#include "utils/uuid.h"

static unsigned char entropy_byte;
static bool use_fallback, short_fallback;
static int entropy_calls, fallback_calls;
ssize_t __wrap_getrandom(void *buffer, size_t length, unsigned int flags) {
    (void)flags;
    ++entropy_calls;
    if (use_fallback) { errno = ENOSYS; return -1; }
    memset(buffer, entropy_byte, length);
    return (ssize_t)length;
}
size_t __wrap_fread(void *buffer, size_t size, size_t count, FILE *file) {
    (void)file;
    ++fallback_calls;
    memset(buffer, entropy_byte, size * count);
    return short_fallback ? count - 1 : count;
}
void setUp(void) {
    entropy_byte = 0;
    use_fallback = short_fallback = false;
    entropy_calls = fallback_calls = 0;
}
void tearDown(void) {}

static void assert_canonical(const char *uuid) {
    TEST_ASSERT_EQUAL_UINT(36, strlen(uuid));
    TEST_ASSERT_TRUE(lightnvr_uuid_is_valid(uuid));
    TEST_ASSERT_EQUAL_CHAR('4', uuid[14]);
    TEST_ASSERT_NOT_NULL(strchr("89ab", uuid[19]));
    for (int i = 0; i < 36; ++i) {
        if (i == 8 || i == 13 || i == 18 || i == 23)
            TEST_ASSERT_EQUAL_CHAR('-', uuid[i]);
        else TEST_ASSERT_NOT_NULL(strchr("0123456789abcdef", uuid[i]));
    }
}
void test_all_entropy_values_stay_in_filename_alphabet(void) {
    for (int fallback = 0; fallback <= 1; ++fallback) {
        use_fallback = fallback;
        for (int value = 0; value <= 255; ++value) {
            entropy_byte = (unsigned char)value;
            char uuid[LIGHTNVR_UUID_STRING_SIZE];
            TEST_ASSERT_EQUAL_INT(0, lightnvr_uuid_generate_v4(uuid));
            assert_canonical(uuid);
        }
    }
    TEST_ASSERT_EQUAL_INT(512, entropy_calls);
    TEST_ASSERT_EQUAL_INT(256, fallback_calls);
}
void test_canonical_bytes_and_version_bits(void) {
    char uuid[LIGHTNVR_UUID_STRING_SIZE];
    TEST_ASSERT_EQUAL_INT(0, lightnvr_uuid_generate_v4(uuid));
    TEST_ASSERT_EQUAL_STRING("00000000-0000-4000-8000-000000000000", uuid);
    entropy_byte = 0xff;
    TEST_ASSERT_EQUAL_INT(0, lightnvr_uuid_generate_v4(uuid));
    TEST_ASSERT_EQUAL_STRING("ffffffff-ffff-4fff-bfff-ffffffffffff", uuid);
}
void test_incomplete_entropy_and_null_output_fail(void) {
    char uuid[LIGHTNVR_UUID_STRING_SIZE] = "unchanged";
    use_fallback = short_fallback = true;
    TEST_ASSERT_EQUAL_INT(-1, lightnvr_uuid_generate_v4(uuid));
    TEST_ASSERT_EQUAL_STRING("unchanged", uuid);
    TEST_ASSERT_EQUAL_INT(-1, lightnvr_uuid_generate_v4(NULL));
}
int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_all_entropy_values_stay_in_filename_alphabet);
    RUN_TEST(test_canonical_bytes_and_version_bits);
    RUN_TEST(test_incomplete_entropy_and_null_output_fail);
    return UNITY_END();
}
