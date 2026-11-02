/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

/**
 * @file libuv_tls.h
 * @brief TLS termination for the libuv HTTP server (mbedTLS)
 *
 * Every byte that crosses a client connection goes through
 * libuv_connection_write() and libuv_connection_read_start(). On a plain HTTP
 * connection they are thin wrappers over uv_write()/uv_read_start(). On a TLS
 * connection they encrypt and decrypt with mbedTLS, using in-memory BIOs
 * layered on the same libuv stream, so the HTTP, file-serving, archive and
 * WebSocket relay code is unchanged and never sees ciphertext.
 *
 * TLS is compiled in only when CMake is configured with ENABLE_SSL=ON, which
 * defines LIGHTNVR_ENABLE_TLS. All functions are callable on the event-loop
 * thread only; mbedTLS contexts are not shared with worker threads.
 */

#ifndef LIBUV_TLS_H
#define LIBUV_TLS_H

#ifdef HTTP_BACKEND_LIBUV

#include <stdbool.h>
#include <uv.h>

struct libuv_connection;
typedef struct libuv_tls_context libuv_tls_context_t;

/**
 * @brief Whether this build contains TLS support (ENABLE_SSL=ON)
 */
bool libuv_tls_compiled_in(void);

/**
 * @brief Load a PEM certificate chain and private key into a server context
 *
 * Logs a specific error and returns NULL on any failure, including a key that
 * does not match the certificate. Key material is never logged.
 *
 * @param cert_path PEM certificate (leaf first, optionally followed by chain)
 * @param key_path  PEM private key (unencrypted)
 * @return Context, or NULL on error or when TLS is not compiled in
 */
libuv_tls_context_t *libuv_tls_context_create(const char *cert_path,
                                              const char *key_path);

/**
 * @brief Free a server context created by libuv_tls_context_create()
 */
void libuv_tls_context_free(libuv_tls_context_t *ctx);

/**
 * @brief Start a TLS session on a freshly accepted connection
 *
 * @return 0 on success, -1 on error (the caller closes the connection)
 */
int libuv_tls_session_start(struct libuv_connection *conn,
                            libuv_tls_context_t *ctx);

/**
 * @brief Free the connection's TLS session, if any (safe to call twice)
 */
void libuv_tls_session_free(struct libuv_connection *conn);

/**
 * @brief Whether the connection is carried over TLS
 */
bool libuv_connection_is_tls(const struct libuv_connection *conn);

/**
 * @brief Write to a client connection, encrypting it when TLS is active
 *
 * Same contract as uv_write(): @p cb is called exactly once with @p req after
 * the bytes have been handed to the kernel, or with an error (including
 * UV_ECANCELED when the connection closes first). Writes complete in order.
 * The caller's buffers may be released as soon as this function returns on a
 * TLS connection; on a plain connection they must stay valid until @p cb.
 */
int libuv_connection_write(struct libuv_connection *conn, uv_write_t *req,
                           const uv_buf_t bufs[], unsigned int nbufs,
                           uv_write_cb cb);

/**
 * @brief Start reading plaintext from a client connection
 *
 * Same contract as uv_read_start(); on TLS connections @p read_cb receives
 * decrypted bytes in buffers obtained from @p alloc_cb.
 */
int libuv_connection_read_start(struct libuv_connection *conn,
                                uv_alloc_cb alloc_cb, uv_read_cb read_cb);

/**
 * @brief Stop reading from a client connection
 */
int libuv_connection_read_stop(struct libuv_connection *conn);

#endif /* HTTP_BACKEND_LIBUV */

#endif /* LIBUV_TLS_H */
