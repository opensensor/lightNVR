#include <string.h>
#include <stdbool.h>


bool ends_with(const char *str, const char *suffix) {
    if (!str || !suffix)
        return false;

    size_t str_len = strlen(str);
    size_t suffix_len = strlen(suffix);

    if (suffix_len > str_len)
        return false;

    return strcmp(str + str_len - suffix_len, suffix) == 0;
}
