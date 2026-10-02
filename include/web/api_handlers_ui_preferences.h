#ifndef LIGHTNVR_API_HANDLERS_UI_PREFERENCES_H
#define LIGHTNVR_API_HANDLERS_UI_PREFERENCES_H

#include "web/request_response.h"

void handle_get_ui_preferences(const http_request_t *req, http_response_t *res);
void handle_put_ui_preferences(const http_request_t *req, http_response_t *res);
void handle_get_ui_favorites(const http_request_t *req, http_response_t *res);
void handle_post_ui_favorite(const http_request_t *req, http_response_t *res);
void handle_delete_ui_favorite(const http_request_t *req, http_response_t *res);

#endif
