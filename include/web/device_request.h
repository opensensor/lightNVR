/* Device command validation: reject ambiguous or lossy values before network I/O. */
#ifndef LIGHTNVR_DEVICE_REQUEST_H
#define LIGHTNVR_DEVICE_REQUEST_H
#include <cjson/cJSON.h>
#include <float.h>
#include <math.h>
#include <string.h>
#include "web/request_response.h"
static inline cJSON *device_json_body(const http_request_t *req) {
    if (!req->body || !req->body_len) return NULL;
    const char *end = NULL, *limit = req->body + req->body_len;
    cJSON *body = cJSON_ParseWithLengthOpts(req->body, req->body_len, &end, 0);
    while (end && end < limit && *end && strchr(" \t\r\n", *end)) end++;
    if (!body || !cJSON_IsObject(body) || end != limit) { cJSON_Delete(body); return NULL; }
    return body;
}
static inline bool device_keys(const cJSON *body, const char *const *keys, size_t count) {
    if (!cJSON_IsObject(body)) return false;
    const cJSON *field;
    cJSON_ArrayForEach(field, body) {
        bool known = false;
        for (size_t i = 0; i < count; ++i) if (!strcmp(field->string, keys[i])) known = true;
        if (!known) return false;
        for (const cJSON *prior = body->child; prior != field; prior = prior->next)
            if (!strcmp(prior->string, field->string)) return false;
    }
    return true;
}
static inline bool device_number(const cJSON *field) {
    return cJSON_IsNumber(field) && isfinite(field->valuedouble) && fabs(field->valuedouble) <= FLT_MAX;
}
static inline bool device_string(const cJSON *field, size_t maximum, bool empty) {
    return cJSON_IsString(field) && field->valuestring && (empty || *field->valuestring) && strlen(field->valuestring) <= maximum;
}
#endif
