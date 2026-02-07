#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>
#include <ctype.h>

#include "web/request_response.h"
#include "web/web_server.h"
#include "core/logger.h"

#define MAX_HEADER_SIZE 8192
#define RECV_BUFFER_SIZE 4096

// URL decode function
int url_decode(const char *src, char *dst, size_t dst_size) {
    if (!src || !dst || dst_size == 0) {
        return -1;
    }

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

void http_request_init(http_request_t *req) {
    if (!req) return;
    memset(req, 0, sizeof(http_request_t));
}

void http_response_init(http_response_t *res) {
    if (!res) return;
    memset(res, 0, sizeof(http_response_t));
    res->status_code = 200;
}

void http_response_free(http_response_t *res) {
    if (!res) return;
    if (res->body_allocated && res->body) {
        free(res->body);
        res->body = NULL;
        res->body_length = 0;
        res->body_allocated = false;
    }
}

const char* http_request_get_header(const http_request_t *req, const char *name) {
    if (!req || !name) return NULL;

    for (int i = 0; i < req->num_headers; i++) {
        if (strcasecmp(req->headers[i].name, name) == 0) {
            return req->headers[i].value;
        }
    }
    return NULL;
}

int http_request_get_query_param(const http_request_t *req, const char *name,
                                  char *value, size_t value_len) {
    if (!req || !name || !value || value_len == 0) return -1;

    const char *qs = req->query_string;
    if (!qs || qs[0] == '\0') return -1;

    size_t name_len = strlen(name);
    const char *p = qs;

    while (p && *p) {
        // Check if this segment starts with the param name
        if (strncmp(p, name, name_len) == 0 && p[name_len] == '=') {
            const char *val_start = p + name_len + 1;
            const char *val_end = strchr(val_start, '&');
            size_t val_len = val_end ? (size_t)(val_end - val_start) : strlen(val_start);

            if (val_len >= value_len) {
                val_len = value_len - 1;
            }

            // URL decode the value
            char encoded[1024] = {0};
            if (val_len < sizeof(encoded)) {
                memcpy(encoded, val_start, val_len);
                encoded[val_len] = '\0';
                url_decode(encoded, value, value_len);
            } else {
                memcpy(value, val_start, val_len);
                value[val_len] = '\0';
            }

            return (int)strlen(value);
        }

        // Move to next parameter
        p = strchr(p, '&');
        if (p) p++;
    }

    return -1;
}

int http_request_get_body_str(const http_request_t *req, char *buf, size_t buf_len) {
    if (!req || !buf || buf_len == 0) return -1;
    if (!req->body || req->body_len == 0) {
        buf[0] = '\0';
        return 0;
    }

    size_t copy_len = req->body_len < buf_len - 1 ? req->body_len : buf_len - 1;
    memcpy(buf, req->body, copy_len);
    buf[copy_len] = '\0';
    return 0;
}

int http_request_extract_path_param(const http_request_t *req, const char *prefix,
                                     char *param_buf, size_t buf_size) {
    if (!req || !prefix || !param_buf || buf_size == 0) return -1;

    const char *path = req->path;
    size_t prefix_len = strlen(prefix);

    if (strncmp(path, prefix, prefix_len) != 0) return -1;

    const char *param = path + prefix_len;
    while (*param == '/') param++;

    // Find end (query string or end of string)
    const char *end = strchr(param, '?');
    size_t param_len = end ? (size_t)(end - param) : strlen(param);

    // Strip trailing slashes
    while (param_len > 0 && param[param_len - 1] == '/') param_len--;

    if (param_len == 0 || param_len >= buf_size) return -1;

    memcpy(param_buf, param, param_len);
    param_buf[param_len] = '\0';
    return 0;
}

int http_response_add_header(http_response_t *res, const char *name, const char *value) {
    if (!res || !name || !value) return -1;
    if (res->num_headers >= MAX_HEADERS) return -1;

    strncpy(res->headers[res->num_headers].name, name,
            sizeof(res->headers[res->num_headers].name) - 1);
    strncpy(res->headers[res->num_headers].value, value,
            sizeof(res->headers[res->num_headers].value) - 1);
    res->num_headers++;
    return 0;
}

int http_response_set_body(http_response_t *res, const char *body) {
    if (!res || !body) return -1;

    // Free previous body if it was allocated by us
    if (res->body_allocated && res->body) {
        free(res->body);
    }

    size_t len = strlen(body);
    res->body = malloc(len + 1);
    if (!res->body) return -1;

    memcpy(res->body, body, len + 1);
    res->body_length = len;
    res->body_allocated = true;
    return 0;
}

void http_response_add_cors_headers(http_response_t *res) {
    if (!res) return;
    http_response_add_header(res, "Access-Control-Allow-Origin", "*");
    http_response_add_header(res, "Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS");
    http_response_add_header(res, "Access-Control-Allow-Headers", "Content-Type, Authorization, X-Requested-With");
    http_response_add_header(res, "Access-Control-Allow-Credentials", "true");
    http_response_add_header(res, "Access-Control-Max-Age", "86400");
}

int http_response_set_json(http_response_t *res, int status_code, const char *json_str) {
    if (!res || !json_str) return -1;

    res->status_code = status_code;
    strncpy(res->content_type, "application/json", sizeof(res->content_type) - 1);

    // Add standard headers
    http_response_add_cors_headers(res);
    http_response_add_header(res, "Cache-Control", "no-cache, no-store, must-revalidate");
    http_response_add_header(res, "Pragma", "no-cache");
    http_response_add_header(res, "Expires", "0");

    return http_response_set_body(res, json_str);
}

int http_response_set_json_error(http_response_t *res, int status_code, const char *error_message) {
    if (!res || !error_message) return -1;

    // Build a JSON error object: {"error": "message"}
    // Calculate needed size: {"error": ""} = 12 chars + message + null
    size_t msg_len = strlen(error_message);
    size_t buf_size = msg_len + 32; // Extra space for JSON wrapping and escaping
    char *json_buf = malloc(buf_size);
    if (!json_buf) return -1;

    snprintf(json_buf, buf_size, "{\"error\":\"%s\"}", error_message);

    int ret = http_response_set_json(res, status_code, json_buf);
    free(json_buf);
    return ret;
}