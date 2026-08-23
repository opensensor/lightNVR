#ifndef LIGHTNVR_UTILS_UUID_H
#define LIGHTNVR_UTILS_UUID_H

#include <stdbool.h>

#define LIGHTNVR_UUID_STRING_SIZE 37

/* Validate the canonical 8-4-4-4-12 hexadecimal UUID representation. */
bool lightnvr_uuid_is_valid(const char *value);

/* Generate a cryptographically random RFC 4122 version-4 UUID. */
int lightnvr_uuid_generate_v4(char output[LIGHTNVR_UUID_STRING_SIZE]);

#endif /* LIGHTNVR_UTILS_UUID_H */
