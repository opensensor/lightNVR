#define _GNU_SOURCE

#include <stdio.h>
#include <string.h>

#include "web/request_response.h"
#include "web/web_server.h"
#include "core/logger.h"

#define MAX_HEADER_SIZE 8192
#define RECV_BUFFER_SIZE 4096

// URL decode function
int url_decode(const char *src, char *dst, size_t dst_size) {
    size_t src_len = strlen(src);
    size_t i, j = 0;

    for (i = 0; i < src_len && j < dst_size - 1; i++) {
        if (src[i] == '%' && i + 2 < src_len) {
            int value;
            if (sscanf(src + i + 1, "%2x", &value) == 1) {
                dst[j++] = value;
                i += 2;
            } else {
                dst[j++] = src[i];
            }
        } else if (src[i] == '+') {
            dst[j++] = ' ';
        } else {
            dst[j++] = src[i];
        }
    }

    dst[j] = '\0';
    return 0;
}