#ifndef API_HANDLERS_CAMERA_TAGS_H
#define API_HANDLERS_CAMERA_TAGS_H

#include "web/request_response.h"

void handle_get_camera_tags(const http_request_t *req, http_response_t *res);
void handle_post_camera_tag(const http_request_t *req, http_response_t *res);
void handle_get_camera_tag(const http_request_t *req, http_response_t *res);
void handle_put_camera_tag(const http_request_t *req, http_response_t *res);
void handle_delete_camera_tag(const http_request_t *req, http_response_t *res);
void handle_post_camera_tag_merge(const http_request_t *req,
                                  http_response_t *res);
void handle_get_camera_tag_assignments(const http_request_t *req,
                                       http_response_t *res);
void handle_put_camera_tag_assignments(const http_request_t *req,
                                       http_response_t *res);

#endif /* API_HANDLERS_CAMERA_TAGS_H */
