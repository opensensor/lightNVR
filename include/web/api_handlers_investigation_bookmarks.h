#ifndef LIGHTNVR_API_HANDLERS_INVESTIGATION_BOOKMARKS_H
#define LIGHTNVR_API_HANDLERS_INVESTIGATION_BOOKMARKS_H

#include "web/request_response.h"

void handle_get_investigation_bookmarks(const http_request_t *req,
                                        http_response_t *res);
void handle_post_investigation_bookmark(const http_request_t *req,
                                        http_response_t *res);
void handle_get_investigation_bookmark(const http_request_t *req,
                                       http_response_t *res);
void handle_put_investigation_bookmark(const http_request_t *req,
                                       http_response_t *res);
void handle_delete_investigation_bookmark(const http_request_t *req,
                                          http_response_t *res);

#endif /* LIGHTNVR_API_HANDLERS_INVESTIGATION_BOOKMARKS_H */
