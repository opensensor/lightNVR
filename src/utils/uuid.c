#define _POSIX_C_SOURCE 200809L

#include "utils/uuid.h"

#include <ctype.h>
#include <errno.h>
#include <stdio.h>
#include <string.h>
#include <sys/random.h>

bool lightnvr_uuid_is_valid(const char *value) {
    if (!value || strnlen(value, LIGHTNVR_UUID_STRING_SIZE) != 36) {
        return false;
    }
    for (int index = 0; index < 36; index++) {
        if (index == 8 || index == 13 || index == 18 || index == 23) {
            if (value[index] != '-') return false;
        } else if (!isxdigit((unsigned char)value[index])) {
            return false;
        }
    }
    return true;
}

static int random_bytes(unsigned char *buffer, size_t length) {
    size_t offset = 0;
    while (offset < length) {
        ssize_t count = getrandom(buffer + offset, length - offset, 0);
        if (count > 0) {
            offset += (size_t)count;
            continue;
        }
        if (count < 0 && errno == EINTR) continue;
        break;
    }
    if (offset == length) return 0;

    FILE *random = fopen("/dev/urandom", "rb");
    if (!random) return -1;
    size_t count = fread(buffer, 1, length, random);
    fclose(random);
    return count == length ? 0 : -1;
}

int lightnvr_uuid_generate_v4(char output[LIGHTNVR_UUID_STRING_SIZE]) {
    if (!output) return -1;
    unsigned char bytes[16];
    if (random_bytes(bytes, sizeof(bytes)) != 0) return -1;
    bytes[6] = (unsigned char)((bytes[6] & 0x0fU) | 0x40U);
    bytes[8] = (unsigned char)((bytes[8] & 0x3fU) | 0x80U);
    int written = snprintf(
        output, LIGHTNVR_UUID_STRING_SIZE,
        "%02x%02x%02x%02x-%02x%02x-%02x%02x-%02x%02x-"
        "%02x%02x%02x%02x%02x%02x",
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6],
        bytes[7], bytes[8], bytes[9], bytes[10], bytes[11], bytes[12],
        bytes[13], bytes[14], bytes[15]);
    return written == LIGHTNVR_UUID_STRING_SIZE - 1 ? 0 : -1;
}
