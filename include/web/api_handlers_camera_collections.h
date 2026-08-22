#ifndef LIGHTNVR_API_HANDLERS_CAMERA_COLLECTIONS_H
#define LIGHTNVR_API_HANDLERS_CAMERA_COLLECTIONS_H

#include "web/request_response.h"

void handle_get_camera_collections(const http_request_t *req,
                                   http_response_t *res);
void handle_post_camera_collection(const http_request_t *req,
                                   http_response_t *res);
void handle_get_camera_collection(const http_request_t *req,
                                  http_response_t *res);
void handle_put_camera_collection(const http_request_t *req,
                                  http_response_t *res);
void handle_delete_camera_collection(const http_request_t *req,
                                     http_response_t *res);
void handle_get_camera_collection_members(const http_request_t *req,
                                          http_response_t *res);
void handle_put_camera_collection_members(const http_request_t *req,
                                          http_response_t *res);
void handle_post_camera_collection_preview(const http_request_t *req,
                                           http_response_t *res);

#endif /* LIGHTNVR_API_HANDLERS_CAMERA_COLLECTIONS_H */
