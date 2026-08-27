

#ifndef STRINGS_H
#define STRINGS_H

#include <ctype.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>

/**
 * Safe string duplication
 *
 * @param str String to duplicate
 * @return Pointer to duplicated string or NULL on failure
 */
char *safe_strdup(const char *str);

/**
 * Safe string copy with size checking. It is safe to pass an unterminated
 * string in `src`: the string will not be checked beyond `src_size` bytes.
 *
 * @param dest Destination buffer
 * @param src Source string
 * @param dst_size Size of destination buffer
 * @param src_size Size of source buffer if not null-terminated
 * @return 0 on success (including if the string is truncated), -1 on failure
 */
int safe_strcpy(char *dest, const char *src, size_t dst_size, size_t src_size);

/**
 * Safe string concatenation with size checking
 *
 * @param dest Destination buffer
 * @param src Source string
 * @param size Size of destination buffer
 * @return 0 on success, -1 on failure
 */
int safe_strcat(char *dest, const char *src, size_t size);

/**
 * Check if a string ends with a given suffix
 *
 * @param str The string to check
 * @param suffix The suffix to look for
 * @return true if the string ends with the suffix, false otherwise
 */
bool ends_with(const char *str, const char *suffix);

/**
 * Returns a pointer to the first printable non-space character in the input string
 * and terminates the string after the last printable non-space character.
 *
 * @param value The input string
 * @return A pointer into the original string
 */
char *trim_ascii_whitespace(char *value);

/**
 * Return true when a string contains at least one non-whitespace byte.
 *
 * This is useful for optional text fields where whitespace-only input is
 * semantically empty but non-empty values must otherwise be preserved
 * byte-for-byte (for example, indented YAML).
 */
bool string_has_non_whitespace(const char *value);

/**
 * Copies up to `output_size` bytes of the input string excluding any leading
 * and trailing whitespace or non-printing characters into the output buffer.
 * Guaranteed to null-terminate the output buffer.
 *
 * @param output The output buffer
 * @param output_size The size of the output buffer
 * @param input The input string
 * @param input_size The maximum size of the input string to check, or zero to
 *      not limit the input string size
 * @return The number of bytes copied, not counting the terminator
 */
size_t copy_trimmed_value(char *output, size_t output_size, const char *input, size_t input_size);

/**
 * Returns a pointer to the first printable character in the input string.
 */
static inline const char *ltrim_pos(const char *input) {
    if (!input) {
        return NULL;
    }

    unsigned char *start = (unsigned char *)input;
    while (*start && !isgraph(*start)) {
        start++;
    }
    return (const char *)start;
}

/**
 * Returns a pointer to the byte _after_ the last printable character
 * in the input string. Set the returned pointer to '\0' to terminate
 * the string after the last printable character.
 */
static inline const char *rtrim_pos(const char *input, size_t input_size) {
    if (!input) {
        return NULL;
    }

    const char *end;
    if (input_size > 0) {
        end = (input + strnlen(input, input_size) - 1);
    } else {
        // If input_size is zero, use the unbounded strlen
        end = (input + strlen(input) - 1);
    }
    // `end` will now point to the last non-null character. If the input is
    // empty, `end` will be (input-1), the character *before* the terminating
    // null.
    while (end > input && !isgraph((unsigned char)*end)) {
        end--;
    }
    // Point to the character after the last printable character. If input was
    // empty, this will now point to the terminating null.
    return end + 1;
}

#endif //STRINGS_H
