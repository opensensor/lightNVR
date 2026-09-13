#ifndef LIGHTNVR_WEB_RECORDING_SOURCE_H
#define LIGHTNVR_WEB_RECORDING_SOURCE_H
#include <string.h>
#include <cjson/cJSON.h>
#include "storage/storage_source.h"
#include "web/request_response.h"

void recording_source_add_status(cJSON *object, uint64_t recording_id);

/* The caller must authorize the logical recording before reaching this helper. */
static inline bool recording_source_for_request(const http_request_t *req,
    http_response_t *res, uint64_t id, char path[MAX_PATH_LENGTH]) {
    char error[256] = {0};
    int result = storage_source_resolve(id, path, error);
    if (result == STORAGE_SOURCE_READY) return true;
    if (result == STORAGE_SOURCE_PREPARING) {
        char prepare[8];
        bool preparing = http_request_get_query_param(req, "prepare", prepare, sizeof(prepare)) > 0 &&
            !strcmp(prepare, "1");
        http_response_add_header(res, "Retry-After", "2");
        http_response_set_json(res, preparing ? 202 : 503,
            "{\"status\":\"preparing\",\"source\":\"archive\",\"error\":\"Preparing archived recording\"}");
    } else http_response_set_json_error(res,
        result == STORAGE_SOURCE_MISSING ? 404 : (result == STORAGE_SOURCE_DELETING ? 409 : 503),
        error[0] ? error : "Recording source is unavailable");
    return false;
}
#endif
