#ifndef LIGHTNVR_RECORDING_ARCHIVE_H
#define LIGHTNVR_RECORDING_ARCHIVE_H
#include "web/request_response.h"
#ifdef HTTP_BACKEND_LIBUV
#include "web/libuv_server.h"
/* Caller has already authorized the logical recording. Returns true when the
 * response (including errors) was handled, false to use the local resolver. */
bool recording_archive_serve(const http_request_t *req, http_response_t *res,
                              uint64_t recording_id, bool download);
int recording_archive_start(libuv_connection_t *conn);
void recording_archive_disconnected(libuv_connection_t *conn);
/* Preserve archive timer ownership during the server shutdown handle walk. */
bool recording_archive_close_timer(uv_handle_t *handle);
#else
static inline bool recording_archive_serve(const http_request_t *req, http_response_t *res,
                                           uint64_t id, bool download) {
    (void)req; (void)res; (void)id; (void)download; return false;
}
#endif
#endif
