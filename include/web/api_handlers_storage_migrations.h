#ifndef LIGHTNVR_API_HANDLERS_STORAGE_MIGRATIONS_H
#define LIGHTNVR_API_HANDLERS_STORAGE_MIGRATIONS_H

#include "web/request_response.h"

void handle_get_storage_migrations(const http_request_t *req,
                                   http_response_t *res);
void handle_post_storage_migration(const http_request_t *req,
                                   http_response_t *res);
void handle_get_storage_migration(const http_request_t *req,
                                  http_response_t *res);
void handle_post_storage_migration_cancel(const http_request_t *req,
                                          http_response_t *res);
void handle_post_storage_migration_retry(const http_request_t *req,
                                         http_response_t *res);

void handle_get_storage_archive(const http_request_t *, http_response_t *);
void handle_post_storage_archive_retry(const http_request_t *, http_response_t *);

#endif /* LIGHTNVR_API_HANDLERS_STORAGE_MIGRATIONS_H */
