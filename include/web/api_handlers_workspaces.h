#ifndef LIGHTNVR_API_HANDLERS_WORKSPACES_H
#define LIGHTNVR_API_HANDLERS_WORKSPACES_H

#include "web/request_response.h"

void handle_get_ui_workspaces(const http_request_t *request,
                              http_response_t *response);
void handle_put_ui_workspaces(const http_request_t *request,
                              http_response_t *response);

#endif /* LIGHTNVR_API_HANDLERS_WORKSPACES_H */
