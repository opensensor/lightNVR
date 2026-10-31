#define _POSIX_C_SOURCE 200809L
/* Exercise the actual private helpers, preserving their production visibility. */
#include "../../src/telemetry/providers/linux_hardware.c"
#include "unity.h"

#include <limits.h>
#include <locale.h>
#include <unistd.h>

static char root[128], path[192];
static char *output;
static bool tracking;
static FILE *owned_file;
static unsigned opened, closed;

FILE *__real_fopen(const char *, const char *);
FILE *__wrap_fopen(const char *name, const char *mode) {
    FILE *file = __real_fopen(name, mode);
    if (tracking && file) { owned_file = file; opened++; }
    return file;
}
int __real_fclose(FILE *file);
int __wrap_fclose(FILE *file) {
    if (file == owned_file) { owned_file = NULL; closed++; }
    return __real_fclose(file);
}

static void write_input(const void *bytes, size_t length) {
    FILE *file = fopen(path, "wb");
    TEST_ASSERT_NOT_NULL(file);
    size_t written = fwrite(bytes, 1, length, file);
    int result = fclose(file);
    TEST_ASSERT_EQUAL_size_t(length, written);
    TEST_ASSERT_EQUAL_INT(0, result);
}
static bool read_input(const char *name, system_health_capability_t *capability) {
    memset(output, 0xa5, HARDWARE_TEXT_MAX);
    tracking = true;
    bool result = read_text(name, output, capability);
    tracking = false;
    TEST_ASSERT_NULL(owned_file);
    TEST_ASSERT_EQUAL_UINT(opened, closed);
    return result;
}
void setUp(void) {
    strcpy(root, "/tmp/lightnvr-t7-58-XXXXXX");
    TEST_ASSERT_NOT_NULL(mkdtemp(root));
    snprintf(path, sizeof(path), "%s/attribute", root);
    output = malloc(HARDWARE_TEXT_MAX); /* Exact-sized allocation: ASan redzones. */
    TEST_ASSERT_NOT_NULL(output);
    owned_file = NULL; opened = closed = 0; tracking = false;
}
void tearDown(void) {
    tracking = false;
    bool leaked = owned_file != NULL;
    if (owned_file) { __real_fclose(owned_file); owned_file = NULL; }
    free(output); output = NULL;
    int removed = unlink(path);
    bool file_cleanup = removed == 0 || errno == ENOENT;
    int directory_cleanup = rmdir(root);
    TEST_ASSERT_FALSE(leaked);
    TEST_ASSERT_TRUE(file_cleanup);
    TEST_ASSERT_EQUAL_INT(0, directory_cleanup);
}

static void test_empty_and_every_all_whitespace_length_are_bounded_errors(void) {
    char bytes[HARDWARE_TEXT_MAX]; memset(bytes, ' ', sizeof(bytes));
    for (size_t n = 0; n < HARDWARE_TEXT_MAX; ++n) {
        write_input(bytes, n);
        system_health_capability_t capability;
        TEST_ASSERT_FALSE(read_input(path, &capability));
        TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_ERROR, capability);
        TEST_ASSERT_EQUAL_CHAR('\0', output[0]);
    }
}
static void test_every_leading_whitespace_length_trims_to_last_payload(void) {
    char bytes[HARDWARE_TEXT_MAX];
    for (size_t n = 1; n < HARDWARE_TEXT_MAX; ++n) {
        memset(bytes, ' ', n); bytes[n - 1] = '7';
        write_input(bytes, n);
        system_health_capability_t capability;
        TEST_ASSERT_TRUE(read_input(path, &capability));
        TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_AVAILABLE, capability);
        TEST_ASSERT_EQUAL_STRING("7", output);
    }
}
static void test_every_trailing_whitespace_length_preserves_first_payload(void) {
    char bytes[HARDWARE_TEXT_MAX];
    for (size_t n = 1; n < HARDWARE_TEXT_MAX; ++n) {
        memset(bytes, '\t', n); bytes[0] = '8';
        write_input(bytes, n);
        system_health_capability_t capability;
        TEST_ASSERT_TRUE(read_input(path, &capability));
        TEST_ASSERT_EQUAL_STRING("8", output);
    }
}
static void test_exact_capacity_minus_one_payload_is_terminated(void) {
    char bytes[HARDWARE_TEXT_MAX - 1]; memset(bytes, '9', sizeof(bytes));
    write_input(bytes, sizeof(bytes));
    system_health_capability_t capability;
    TEST_ASSERT_TRUE(read_input(path, &capability));
    TEST_ASSERT_EQUAL_MEMORY(bytes, output, sizeof(bytes));
    TEST_ASSERT_EQUAL_CHAR('\0', output[sizeof(bytes)]);
}
static void test_overlong_and_one_past_capacity_inputs_are_rejected(void) {
    char bytes[HARDWARE_TEXT_MAX + 1]; memset(bytes, '9', sizeof(bytes));
    for (size_t n = HARDWARE_TEXT_MAX; n <= sizeof(bytes); ++n) {
        write_input(bytes, n);
        system_health_capability_t capability;
        TEST_ASSERT_FALSE(read_input(path, &capability));
        TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_ERROR, capability);
        /* False callers discard output: no terminator is promised on failure. */
    }
}
static void test_all_unsigned_character_values_use_valid_ctype_indices(void) {
    for (unsigned byte = 1; byte <= UCHAR_MAX; ++byte) {
        unsigned char value = (unsigned char)byte;
        write_input(&value, 1);
        system_health_capability_t capability;
        bool whitespace = byte == ' ' || byte == '\t' || byte == '\n' ||
                          byte == '\r' || byte == '\v' || byte == '\f';
        TEST_ASSERT_EQUAL_INT(!whitespace, read_input(path, &capability));
        TEST_ASSERT_EQUAL_CHAR(whitespace ? 0 : value, output[0]);
        if (!whitespace) TEST_ASSERT_EQUAL_CHAR(0, output[1]);
    }
}
static void test_missing_and_actual_read_error_close_owned_stream(void) {
    system_health_capability_t capability;
    TEST_ASSERT_FALSE(read_input(path, &capability));
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED, capability);
    TEST_ASSERT_EQUAL_UINT(0, opened);
    /* Linux permits fopen(directory); fread then reports EISDIR. */
    TEST_ASSERT_FALSE(read_input(root, &capability));
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_ERROR, capability);
    TEST_ASSERT_EQUAL_UINT(1, opened);
    TEST_ASSERT_EQUAL_UINT(1, closed);
}
static void test_integer_caller_consumes_trimmed_value_and_rejects_overflow(void) {
    const char valid[] = " \t18446744073709551615\r\n";
    write_input(valid, sizeof(valid) - 1);
    hardware_integer_t reading = read_integer(path, 10);
    TEST_ASSERT_TRUE(reading.valid);
    TEST_ASSERT_EQUAL_UINT64(UINT64_MAX, reading.value);
    const char invalid[] = "18446744073709551616";
    write_input(invalid, sizeof(invalid) - 1);
    reading = read_integer(path, 10);
    TEST_ASSERT_FALSE(reading.valid);
    TEST_ASSERT_EQUAL(SYSTEM_HEALTH_CAPABILITY_ERROR, reading.capability);
}
static void test_literal_separator_and_empty_parts_match_known_hash(void) {
    char *empty = malloc(1);
    TEST_ASSERT_NOT_NULL(empty); empty[0] = 0;
    uint64_t actual = scoped_hash(empty, empty, empty, 0);
    free(empty);
    TEST_ASSERT_EQUAL_HEX64(UINT64_C(0xbc28bea9bf3e71a7), actual);
    TEST_ASSERT_EQUAL_HEX64(UINT64_C(0x7b58946416c9a930),
        scoped_hash("installation-A", "fan", "hwmon0", 1));
    TEST_ASSERT_EQUAL_HEX64(UINT64_C(0x75386f57550426fb),
        scoped_hash("installation-B", "fan", "hwmon0", 1));
}
static void test_hash_exact_size_longest_scope_and_channel_inputs(void) {
    char *scope = malloc(LINUX_HARDWARE_INSTALLATION_SCOPE_LENGTH);
    char *name = malloc(sizeof("hwmon999"));
    TEST_ASSERT_NOT_NULL(scope); TEST_ASSERT_NOT_NULL(name);
    memset(scope, 'A', LINUX_HARDWARE_INSTALLATION_SCOPE_LENGTH - 1);
    scope[LINUX_HARDWARE_INSTALLATION_SCOPE_LENGTH - 1] = 0;
    memcpy(name, "hwmon999", sizeof("hwmon999"));
    uint64_t actual = scoped_hash(scope, "fan", name, 999);
    free(scope); free(name);
    TEST_ASSERT_EQUAL_HEX64(UINT64_C(0x84594dec82b8f0f8), actual);
    TEST_ASSERT_EQUAL_UINT32(UINT32_MAX, UINT_MAX);
    TEST_ASSERT_EQUAL_HEX64(UINT64_C(0xfa4fb3faf2c57446),
        scoped_hash("installation-A", "fan", "hwmon0", UINT_MAX));
}
static void test_scope_initialization_bounds_terminated_hash_input(void) {
    char source[192]; memset(source, 'A', sizeof(source)); source[191] = 0;
    linux_hardware_state_t state;
    linux_hardware_state_init(&state, source);
    TEST_ASSERT_EQUAL_size_t(95, strlen(state.installation_scope));
    TEST_ASSERT_EQUAL_CHAR(0, state.installation_scope[95]);
    TEST_ASSERT_EQUAL_HEX64(UINT64_C(0x84594dec82b8f0f8),
        scoped_hash(state.installation_scope, "fan", "hwmon999", 999));
}

static void test_public_entry_points_reject_null_required_inputs(void) {
    linux_hardware_state_t state;
    linux_hardware_state_init(&state, "fixture");
    system_health_collect_context_t context = {.sys_root = root};
    system_health_provider_inventory_t inventory;
    system_health_observation_sink_t sink = {0};
    TEST_ASSERT_EQUAL_INT(-1, linux_hardware_discover(NULL, &context, &inventory));
    TEST_ASSERT_EQUAL_INT(-1, linux_hardware_discover(&state, NULL, &inventory));
    TEST_ASSERT_EQUAL_INT(-1, linux_hardware_discover(&state, &context, NULL));
    TEST_ASSERT_EQUAL_INT(-1, linux_hardware_collect(NULL, &context, &sink));
    TEST_ASSERT_EQUAL_INT(-1, linux_hardware_collect(&state, NULL, &sink));
    TEST_ASSERT_EQUAL_INT(-1, linux_hardware_collect(&state, &context, NULL));
    context.sys_root = NULL;
    TEST_ASSERT_EQUAL_INT(-1, linux_hardware_discover(&state, &context, &inventory));
    TEST_ASSERT_EQUAL_INT(-1, linux_hardware_collect(&state, &context, &sink));
}

int main(void) {
    if (!setlocale(LC_CTYPE, "C")) return 1;
    UNITY_BEGIN();
    RUN_TEST(test_empty_and_every_all_whitespace_length_are_bounded_errors);
    RUN_TEST(test_every_leading_whitespace_length_trims_to_last_payload);
    RUN_TEST(test_every_trailing_whitespace_length_preserves_first_payload);
    RUN_TEST(test_exact_capacity_minus_one_payload_is_terminated);
    RUN_TEST(test_overlong_and_one_past_capacity_inputs_are_rejected);
    RUN_TEST(test_all_unsigned_character_values_use_valid_ctype_indices);
    RUN_TEST(test_missing_and_actual_read_error_close_owned_stream);
    RUN_TEST(test_integer_caller_consumes_trimmed_value_and_rejects_overflow);
    RUN_TEST(test_literal_separator_and_empty_parts_match_known_hash);
    RUN_TEST(test_hash_exact_size_longest_scope_and_channel_inputs);
    RUN_TEST(test_scope_initialization_bounds_terminated_hash_input);
    RUN_TEST(test_public_entry_points_reject_null_required_inputs);
    return UNITY_END();
}
