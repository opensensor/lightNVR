#ifndef LIGHTNVR_API_HANDLERS_CAMERA_FAVORITES_H
#define LIGHTNVR_API_HANDLERS_CAMERA_FAVORITES_H

#include "web/request_response.h"

/** GET /api/camera-favorites: the caller's favorites within their live-view scope. */
void handle_get_camera_favorites(const http_request_t *req, http_response_t *res);

/** PUT /api/camera-favorites/{camera_uuid}: add a camera (idempotent). */
void handle_put_camera_favorite(const http_request_t *req, http_response_t *res);

/** DELETE /api/camera-favorites/{camera_uuid}: remove a camera (idempotent). */
void handle_delete_camera_favorite(const http_request_t *req, http_response_t *res);

#endif /* LIGHTNVR_API_HANDLERS_CAMERA_FAVORITES_H */
