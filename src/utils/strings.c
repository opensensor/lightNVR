#include "utils/strings.h"
#include "core/logger.h"

// Safe string duplication
char *safe_strdup(const char *str) {
    if (!str) {
        return NULL;
    }

    char *ptr = strdup(str);
    if (!ptr) {
        log_error("String duplication failed for length %zu", strlen(str));
    }

    return ptr;
}

// Safe string copy
int safe_strcpy(char *dest, const char *src, size_t dst_size, size_t src_size) {
    if (!dest || !src || dst_size == 0) {
        return -1;
    }

    size_t src_len;
    if (src_size > 0) {
        src_len = strnlen(src, src_size);
        // Note that if src_len == src_size, src may not be null-terminated
        // and we will need to account for that.
    } else {
        src_len = strlen(src);
    }

    if (src_len >= dst_size) {
        // Not enough space: truncate before null-terminating
        src_len = dst_size - 1;
    }
    memcpy(dest, src, src_len);
    dest[src_len] = '\0';

    return 0;
}

// Safe string concatenation
int safe_strcat(char *dest, const char *src, size_t size) {
    if (!dest || !src || size == 0) {
        return -1;
    }

    // Do not read beyond the destination buffer
    size_t dest_len = strnlen(dest, size);
    if (dest_len >= size) {
        // Destination already fills the buffer
        return -1;
    }

    return safe_strcpy(dest + dest_len, src, size - dest_len, size - dest_len);
}

bool ends_with(const char *str, const char *suffix) {
    if (!str || !suffix)
        return false;

    size_t str_len = strlen(str);
    size_t suffix_len = strlen(suffix);

    if (suffix_len > str_len)
        return false;

    return strcmp(str + str_len - suffix_len, suffix) == 0;
}

bool string_has_non_whitespace(const char *value) {
    if (!value) {
        return false;
    }

    while (*value) {
        if (!isspace((unsigned char)*value)) {
            return true;
        }
        value++;
    }

    return false;
}

char *trim_ascii_whitespace(char *value) {
    if (!value) {
        return NULL;
    }

    // Input is not const so output is not const
    char *end = (char *)rtrim_pos(value, 0);
    *end = '\0';

    return (char *)ltrim_pos(value);
}

size_t copy_trimmed_value(char *output, size_t output_size, const char *input, size_t input_size) {
    if (!input || !output || output_size == 0) {
        return false;
    }

    const char *start = ltrim_pos(input);

    // Allow `input_size` to limit the length of the input string. This allows us to pass
    // unterminated input strings and only copy up to the specified limit.
    const char *end = rtrim_pos(input, input_size);

    // Does not include the null terminator
    size_t len = end - start;
    if (len >= output_size) {
        len = output_size - 1;
    }
    memcpy(output, start, len);
    output[len] = '\0';
    return len;
}
