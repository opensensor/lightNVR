/**
 * @file test_yaml_validate.c
 * @brief Layer 1 unit tests for yaml_validate helpers (T1).
 *
 * Tests the libyaml-backed wrapper:
 *   - yaml_validate_str
 *   - yaml_is_mapping_root
 *   - yaml_validate_is_available
 *
 * When built WITHOUT libyaml (LIGHTNVR_HAVE_LIBYAML undefined), the
 * stubs return a stable "validation disabled" answer; several of the
 * error-path assertions are relaxed in that mode so the test suite
 * still passes as a compile/link smoke check.
 */

#define _POSIX_C_SOURCE 200809L

#include <stdlib.h>
#include <string.h>
#include <stdio.h>

#include "unity.h"
#include "utils/yaml_validate.h"

/* ---- Unity boilerplate ---- */
void setUp(void)    {}
void tearDown(void) {}

/* ================================================================
 * 1. Valid mapping
 * ================================================================ */
void test_valid_mapping_returns_zero(void) {
    const char *src = "key: value\n";
    char err[256] = {0};
    int rc = yaml_validate_str(src, strlen(src), err, sizeof(err));
    TEST_ASSERT_EQUAL_INT(0, rc);
}

/* ================================================================
 * 2. Malformed YAML → parse error with line/col in err_buf
 *
 * libyaml's tolerance for plain-text indentation is high, so we use
 * an unmistakably invalid construct: a flow-sequence that is never
 * closed. This is guaranteed to raise a parser error.
 * ================================================================ */
void test_invalid_indentation_returns_error(void) {
    /* Unclosed flow sequence — libyaml reliably rejects this. */
    const char *src = "key: [1, 2, 3\nkey2: value\n";
    char err[256] = {0};
    int rc = yaml_validate_str(src, strlen(src), err, sizeof(err));

    if (yaml_validate_is_available()) {
        TEST_ASSERT_EQUAL_INT(-1, rc);
        TEST_ASSERT_TRUE_MESSAGE(err[0] != '\0', "err_buf should contain a message");
        /* Error text should mention line or column info. */
        int has_line_or_col =
            (strstr(err, "line") != NULL) ||
            (strstr(err, "column") != NULL) ||
            (strstr(err, "Line") != NULL) ||
            (strstr(err, "Column") != NULL);
        TEST_ASSERT_TRUE_MESSAGE(has_line_or_col,
                                 "err_buf should contain line/column info");
    } else {
        /* Stub path — succeeds silently. */
        TEST_ASSERT_EQUAL_INT(0, rc);
    }
}

/* ================================================================
 * 3. Non-mapping root: sequence
 *    yaml_is_mapping_root returns 0 (not mapping), but
 *    yaml_validate_str returns 0 (valid YAML).
 * ================================================================ */
void test_sequence_root_is_valid_but_not_mapping(void) {
    const char *src = "- a\n- b\n";
    char err[256] = {0};
    int rc_valid = yaml_validate_str(src, strlen(src), err, sizeof(err));
    TEST_ASSERT_EQUAL_INT(0, rc_valid);

    int is_map = yaml_is_mapping_root(src, strlen(src));
    /* 0 means "not mapping root" — true for both libyaml and stub. */
    TEST_ASSERT_EQUAL_INT(0, is_map);
}

/* Sanity — mapping root IS detected when libyaml is available. */
void test_mapping_root_detected(void) {
    const char *src = "key: value\n";
    int is_map = yaml_is_mapping_root(src, strlen(src));
    if (yaml_validate_is_available()) {
        TEST_ASSERT_EQUAL_INT(1, is_map);
    } else {
        /* Stub returns 0 (unknown) — documented contract. */
        TEST_ASSERT_EQUAL_INT(0, is_map);
    }
}

/* ================================================================
 * 4. Empty string: behavior defined as "valid, empty document".
 * ================================================================ */
void test_empty_string_is_valid(void) {
    const char *src = "";
    char err[256] = {0};
    int rc = yaml_validate_str(src, 0, err, sizeof(err));
    TEST_ASSERT_EQUAL_INT(0, rc);
}

void test_empty_string_not_mapping_root(void) {
    const char *src = "";
    int is_map = yaml_is_mapping_root(src, 0);
    TEST_ASSERT_EQUAL_INT(0, is_map);
}

/* ================================================================
 * 5. CRLF line endings
 * ================================================================ */
void test_crlf_line_endings_valid(void) {
    const char *src = "key: value\r\nkey2: value2\r\n";
    char err[256] = {0};
    int rc = yaml_validate_str(src, strlen(src), err, sizeof(err));
    TEST_ASSERT_EQUAL_INT(0, rc);
}

/* ================================================================
 * 6. Huge valid document (≥ 50 KB)
 * ================================================================ */
void test_huge_valid_document(void) {
    const size_t n_entries = 5000; /* each "kNNNN: vNNNN\n" ≈ 13 bytes → ~65 KB */
    size_t cap = n_entries * 32 + 64;
    char *buf = (char *)malloc(cap);
    TEST_ASSERT_NOT_NULL(buf);

    size_t off = 0;
    for (size_t i = 0; i < n_entries; ++i) {
        int n = snprintf(buf + off, cap - off, "k%05zu: v%05zu\n", i, i);
        TEST_ASSERT_TRUE(n > 0 && (size_t)n < cap - off);
        off += (size_t)n;
    }
    TEST_ASSERT_TRUE_MESSAGE(off >= 50 * 1024, "document should be at least 50 KB");

    char err[256] = {0};
    int rc = yaml_validate_str(buf, off, err, sizeof(err));
    TEST_ASSERT_EQUAL_INT(0, rc);

    /* And the huge document is a mapping root (when libyaml available). */
    int is_map = yaml_is_mapping_root(buf, off);
    if (yaml_validate_is_available()) {
        TEST_ASSERT_EQUAL_INT(1, is_map);
    } else {
        TEST_ASSERT_EQUAL_INT(0, is_map);
    }

    free(buf);
}

/* ================================================================
 * Null-safety: null src / zero err_size should not crash.
 * ================================================================ */
void test_null_src_returns_zero(void) {
    /* NULL src with zero length is treated as empty → valid. */
    char err[256] = {0};
    int rc = yaml_validate_str(NULL, 0, err, sizeof(err));
    TEST_ASSERT_EQUAL_INT(0, rc);
}

void test_null_err_buf_does_not_crash(void) {
    const char *src = "key: value\n";
    int rc = yaml_validate_str(src, strlen(src), NULL, 0);
    TEST_ASSERT_EQUAL_INT(0, rc);
}

/* ================================================================
 * main
 * ================================================================ */
int main(void) {
    UNITY_BEGIN();

    RUN_TEST(test_valid_mapping_returns_zero);
    RUN_TEST(test_invalid_indentation_returns_error);
    RUN_TEST(test_sequence_root_is_valid_but_not_mapping);
    RUN_TEST(test_mapping_root_detected);
    RUN_TEST(test_empty_string_is_valid);
    RUN_TEST(test_empty_string_not_mapping_root);
    RUN_TEST(test_crlf_line_endings_valid);
    RUN_TEST(test_huge_valid_document);
    RUN_TEST(test_null_src_returns_zero);
    RUN_TEST(test_null_err_buf_does_not_crash);

    return UNITY_END();
}
