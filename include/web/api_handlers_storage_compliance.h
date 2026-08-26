#ifndef LIGHTNVR_API_HANDLERS_STORAGE_COMPLIANCE_H
#define LIGHTNVR_API_HANDLERS_STORAGE_COMPLIANCE_H

#include "web/request_response.h"

void handle_get_storage_compliance(const http_request_t *req,
                                   http_response_t *res);

#endif
