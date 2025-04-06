

#ifndef STRINGS_H
#define STRINGS_H

#include <stdbool.h>

/**
 * Check if a string ends with a given suffix
 *
 * @param str The string to check
 * @param suffix The suffix to look for
 * @return true if the string ends with the suffix, false otherwise
 */
bool ends_with(const char *str, const char *suffix);

#endif //STRINGS_H
