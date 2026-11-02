/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

/**
 * @file libuv_tls.c
 * @brief mbedTLS termination layered on libuv client streams (#650)
 *
 * Design: each TLS connection owns an mbedtls_ssl_context whose BIO callbacks
 * read from and write to memory buffers instead of a socket.
 *
 *  - Inbound: libuv reads ciphertext straight into the session's input
 *    buffer (tls_alloc/tls_read). mbedTLS consumes it through bio_recv, and
 *    the decrypted bytes are handed to the connection's normal alloc/read
 *    callbacks, so llhttp and the WebSocket relay see the same plaintext
 *    stream they would see over HTTP.
 *  - Outbound: libuv_connection_write() runs the caller's buffers through
 *    mbedtls_ssl_write(); bio_send appends the records to the output buffer,
 *    which is then written with one uv_write(). The caller's callback runs
 *    when that write completes, so back-pressure and post-write actions
 *    (keep-alive reset, close) work as on a plain connection.
 *
 * Everything here runs on the event-loop thread.
 */

#ifdef HTTP_BACKEND_LIBUV

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <uv.h>

#include "utils/memory.h"
#include "web/libuv_server.h"
#include "web/libuv_tls.h"
#define LOG_COMPONENT "HTTP"
#include "core/logger.h"

#ifdef LIGHTNVR_ENABLE_TLS

#include <mbedtls/ctr_drbg.h>
#include <mbedtls/entropy.h>
#include <mbedtls/error.h>
#include <mbedtls/pk.h>
#include <mbedtls/ssl.h>
#include <mbedtls/version.h>
#include <mbedtls/x509_crt.h>
#if defined(MBEDTLS_PSA_CRYPTO_C)
#include <psa/crypto.h>
#endif

/* Ciphertext read per uv read. Kept well below the 64 KiB buffers the
 * WebSocket relay allocates, so all plaintext produced by one socket read
 * (this chunk plus at most one partially received record) fits in a single
 * delivery. A consumer that stops reading after each delivery therefore
 * never strands decrypted bytes inside mbedTLS. */
#define TLS_READ_CHUNK      ((size_t)32 * 1024)
/* Unconsumed ciphertext never exceeds one partial record plus one read
 * chunk; anything beyond this means the state machine is wedged. */
#define TLS_INPUT_MAX       ((size_t)256 * 1024)
#define TLS_PLAINTEXT_HINT  ((size_t)64 * 1024)

struct libuv_tls_context {
    mbedtls_ssl_config conf;
    mbedtls_x509_crt cert;
    mbedtls_pk_context key;
    mbedtls_entropy_context entropy;
    mbedtls_ctr_drbg_context drbg;
};

typedef struct libuv_tls_session {
    mbedtls_ssl_context ssl;

    unsigned char *in;          // Received ciphertext
    size_t in_len;              // Bytes stored in `in`
    size_t in_off;              // Bytes of `in` already consumed by mbedTLS
    size_t in_cap;

    unsigned char *out;         // Ciphertext produced, not yet written
    size_t out_len;
    size_t out_cap;

    uv_alloc_cb alloc_cb;       // Plaintext consumer
    uv_read_cb read_cb;
    bool reading;               // Consumer wants plaintext
    bool handshake_done;
    bool failed;                // Stream is unusable; never touch ssl again

    /* Delivers input buffered in the session when reading resumes (e.g. a
     * pipelined request decrypted with the previous one); no new ciphertext
     * may ever arrive to trigger it. Runs from the loop, never re-entrantly. */
    uv_idle_t drain;
    bool drain_init;
    libuv_connection_t *conn;   // NULL once the session is detached
} libuv_tls_session_t;

typedef struct {
    uv_write_t req;             // Must be first
    uv_buf_t buf;               // Owned ciphertext
    uv_write_t *user_req;       // Caller's request, or NULL for internal output
    uv_write_cb user_cb;
} tls_write_t;

static void tls_error_string(int ret, char *buf, size_t len) {
#if defined(MBEDTLS_ERROR_C)
    mbedtls_strerror(ret, buf, len);
#else
    snprintf(buf, len, "mbedTLS error -0x%04x", (unsigned int)-ret);
#endif
}

bool libuv_tls_compiled_in(void) {
    return true;
}

// ---------------------------------------------------------------------------
// Server context
// ---------------------------------------------------------------------------

libuv_tls_context_t *libuv_tls_context_create(const char *cert_path,
                                              const char *key_path) {
    char err[160];
    int ret;

    if (!cert_path || !cert_path[0] || !key_path || !key_path[0]) {
        log_error("HTTPS: ssl_cert_path and ssl_key_path must both be set when ssl_enabled = true");
        return NULL;
    }

#if defined(MBEDTLS_PSA_CRYPTO_C)
    // TLS 1.3 in mbedTLS 3.x runs on PSA; idempotent if already initialized.
    if (psa_crypto_init() != PSA_SUCCESS) {
        log_error("HTTPS: failed to initialize PSA crypto");
        return NULL;
    }
#endif

    libuv_tls_context_t *ctx = safe_calloc(1, sizeof(*ctx));
    if (!ctx) {
        log_error("HTTPS: out of memory creating TLS context");
        return NULL;
    }
    mbedtls_ssl_config_init(&ctx->conf);
    mbedtls_x509_crt_init(&ctx->cert);
    mbedtls_pk_init(&ctx->key);
    mbedtls_entropy_init(&ctx->entropy);
    mbedtls_ctr_drbg_init(&ctx->drbg);

    static const char personalization[] = "lightnvr-https";
    ret = mbedtls_ctr_drbg_seed(&ctx->drbg, mbedtls_entropy_func, &ctx->entropy,
                                (const unsigned char *)personalization,
                                sizeof(personalization) - 1);
    if (ret != 0) {
        tls_error_string(ret, err, sizeof(err));
        log_error("HTTPS: failed to seed the random generator: %s", err);
        goto fail;
    }

    ret = mbedtls_x509_crt_parse_file(&ctx->cert, cert_path);
    if (ret != 0) {
        if (ret > 0) {
            snprintf(err, sizeof(err), "%d certificate(s) in the file could not be parsed", ret);
        } else {
            tls_error_string(ret, err, sizeof(err));
        }
        log_error("HTTPS: cannot load certificate %s: %s", cert_path, err);
        goto fail;
    }

#if MBEDTLS_VERSION_NUMBER >= 0x03000000
    ret = mbedtls_pk_parse_keyfile(&ctx->key, key_path, NULL,
                                   mbedtls_ctr_drbg_random, &ctx->drbg);
#else
    ret = mbedtls_pk_parse_keyfile(&ctx->key, key_path, NULL);
#endif
    if (ret != 0) {
        tls_error_string(ret, err, sizeof(err));
        log_error("HTTPS: cannot load private key %s: %s", key_path, err);
        goto fail;
    }

#if MBEDTLS_VERSION_NUMBER >= 0x03000000
    ret = mbedtls_pk_check_pair(&ctx->cert.pk, &ctx->key,
                                mbedtls_ctr_drbg_random, &ctx->drbg);
#else
    ret = mbedtls_pk_check_pair(&ctx->cert.pk, &ctx->key);
#endif
    if (ret != 0) {
        log_error("HTTPS: private key %s does not match certificate %s",
                  key_path, cert_path);
        goto fail;
    }

    ret = mbedtls_ssl_config_defaults(&ctx->conf, MBEDTLS_SSL_IS_SERVER,
                                      MBEDTLS_SSL_TRANSPORT_STREAM,
                                      MBEDTLS_SSL_PRESET_DEFAULT);
    if (ret != 0) {
        tls_error_string(ret, err, sizeof(err));
        log_error("HTTPS: failed to apply TLS defaults: %s", err);
        goto fail;
    }
    mbedtls_ssl_conf_rng(&ctx->conf, mbedtls_ctr_drbg_random, &ctx->drbg);
    // Library default cipher suites; only the protocol floor is raised.
#if MBEDTLS_VERSION_NUMBER >= 0x03000000
    mbedtls_ssl_conf_min_tls_version(&ctx->conf, MBEDTLS_SSL_VERSION_TLS1_2);
#else
    mbedtls_ssl_conf_min_version(&ctx->conf, MBEDTLS_SSL_MAJOR_VERSION_3,
                                 MBEDTLS_SSL_MINOR_VERSION_3);
#endif
#if defined(MBEDTLS_SSL_ALPN)
    static const char *alpn[] = {"http/1.1", NULL};
    mbedtls_ssl_conf_alpn_protocols(&ctx->conf, alpn);
#endif

    ret = mbedtls_ssl_conf_own_cert(&ctx->conf, &ctx->cert, &ctx->key);
    if (ret != 0) {
        tls_error_string(ret, err, sizeof(err));
        log_error("HTTPS: cannot use certificate %s: %s", cert_path, err);
        goto fail;
    }

    log_info("HTTPS: loaded certificate %s", cert_path);
    return ctx;

fail:
    libuv_tls_context_free(ctx);
    return NULL;
}

void libuv_tls_context_free(libuv_tls_context_t *ctx) {
    if (!ctx) return;
    mbedtls_ssl_config_free(&ctx->conf);
    mbedtls_x509_crt_free(&ctx->cert);
    mbedtls_pk_free(&ctx->key);       // zeroizes key material
    mbedtls_ctr_drbg_free(&ctx->drbg);
    mbedtls_entropy_free(&ctx->entropy);
    safe_free(ctx);
}

// ---------------------------------------------------------------------------
// Memory BIOs
// ---------------------------------------------------------------------------

static int bio_recv(void *opaque, unsigned char *buf, size_t len) {
    libuv_tls_session_t *s = opaque;
    size_t available = s->in_len - s->in_off;
    if (available == 0) return MBEDTLS_ERR_SSL_WANT_READ;
    if (len > available) len = available;
    memcpy(buf, s->in + s->in_off, len);
    s->in_off += len;
    return (int)len;
}

static int bio_send(void *opaque, const unsigned char *buf, size_t len) {
    libuv_tls_session_t *s = opaque;
    if (len > (size_t)INT32_MAX) len = (size_t)INT32_MAX;
    if (len > s->out_cap - s->out_len) {
        size_t capacity = s->out_cap ? s->out_cap : 4096;
        while (capacity - s->out_len < len) {
            if (capacity > SIZE_MAX / 2) return MBEDTLS_ERR_SSL_ALLOC_FAILED;
            capacity *= 2;
        }
        unsigned char *grown = safe_realloc(s->out, capacity);
        if (!grown) return MBEDTLS_ERR_SSL_ALLOC_FAILED;
        s->out = grown;
        s->out_cap = capacity;
    }
    memcpy(s->out + s->out_len, buf, len);
    s->out_len += len;
    return (int)len;
}

// ---------------------------------------------------------------------------
// Output
// ---------------------------------------------------------------------------

static void tls_write_done(uv_write_t *req, int status) {
    tls_write_t *write = (tls_write_t *)req;
    uv_write_t *user_req = write->user_req;
    uv_write_cb user_cb = write->user_cb;
    safe_free(write->buf.base);
    safe_free(write);
    if (user_cb) user_cb(user_req, status);
}

/* Hand all pending ciphertext to libuv as a single write. With a user
 * request the callback fires even when there is nothing to send, keeping
 * uv_write()'s "callback exactly once" contract. */
static int flush_output(libuv_connection_t *conn, uv_write_t *user_req,
                        uv_write_cb user_cb) {
    libuv_tls_session_t *s = conn->tls_session;
    if (s->out_len == 0 && !user_req) return 0;

    tls_write_t *write = safe_calloc(1, sizeof(*write));
    if (!write) return UV_ENOMEM;
    write->buf = uv_buf_init((char *)s->out, (unsigned int)s->out_len);
    write->user_req = user_req;
    write->user_cb = user_cb;
    s->out = NULL;
    s->out_len = 0;
    s->out_cap = 0;
    if (user_req) user_req->handle = (uv_stream_t *)&conn->handle;

    int r = uv_write(&write->req, (uv_stream_t *)&conn->handle, &write->buf, 1,
                     tls_write_done);
    if (r != 0) {
        safe_free(write->buf.base);
        safe_free(write);
    }
    return r;
}

static void close_after_alert(uv_write_t *req, int status) {
    (void)status;
    libuv_connection_t *conn = req->handle ? req->handle->data : NULL;
    safe_free(req);
    if (conn) libuv_connection_close(conn);
}

/* Stop using the session, send any alert mbedTLS queued, then close. */
static void fail_session(libuv_connection_t *conn) {
    libuv_tls_session_t *s = conn->tls_session;
    s->failed = true;
    s->reading = false;
    uv_read_stop((uv_stream_t *)&conn->handle);
    if (s->out_len > 0 && !uv_is_closing((uv_handle_t *)&conn->handle)) {
        uv_write_t *req = safe_calloc(1, sizeof(*req));
        if (req && flush_output(conn, req, close_after_alert) == 0) return;
        safe_free(req);
    }
    libuv_connection_close(conn);
}

// ---------------------------------------------------------------------------
// Input
// ---------------------------------------------------------------------------

static void deliver_status(libuv_connection_t *conn, ssize_t status) {
    libuv_tls_session_t *s = conn->tls_session;
    if (!s->reading || !s->read_cb) {
        libuv_connection_close(conn);
        return;
    }
    uv_buf_t buf = uv_buf_init(NULL, 0);
    s->alloc_cb((uv_handle_t *)&conn->handle, TLS_PLAINTEXT_HINT, &buf);
    s->read_cb((uv_stream_t *)&conn->handle, status, &buf);
}

static bool session_active(libuv_connection_t *conn, libuv_tls_session_t *s) {
    return conn->tls_session == s && !s->failed && s->reading &&
           !uv_is_closing((uv_handle_t *)&conn->handle);
}

static void process_input(libuv_connection_t *conn) {
    libuv_tls_session_t *s = conn->tls_session;
    char err[128];

    if (!s->handshake_done) {
        int ret = mbedtls_ssl_handshake(&s->ssl);
        if (ret == MBEDTLS_ERR_SSL_WANT_READ || ret == MBEDTLS_ERR_SSL_WANT_WRITE) {
            if (flush_output(conn, NULL, NULL) != 0) fail_session(conn);
            return;
        }
        if (ret != 0) {
            tls_error_string(ret, err, sizeof(err));
            log_debug("HTTPS: handshake with %s failed: %s",
                      conn->request.client_ip, err);
            fail_session(conn);
            return;
        }
        s->handshake_done = true;
        if (flush_output(conn, NULL, NULL) != 0) {
            fail_session(conn);
            return;
        }
    }

    while (session_active(conn, s)) {
        if (mbedtls_ssl_get_bytes_avail(&s->ssl) == 0 && s->in_off == s->in_len) {
            return;  // Nothing buffered; wait for more ciphertext
        }

        uv_buf_t buf = uv_buf_init(NULL, 0);
        s->alloc_cb((uv_handle_t *)&conn->handle, TLS_PLAINTEXT_HINT, &buf);
        if (!buf.base || buf.len == 0) {
            s->read_cb((uv_stream_t *)&conn->handle, UV_ENOBUFS, &buf);
            return;
        }

        size_t total = 0;
        int ret = 0;
        while (total < buf.len) {
            ret = mbedtls_ssl_read(&s->ssl, (unsigned char *)buf.base + total,
                                   buf.len - total);
            if (ret <= 0) break;
            total += (size_t)ret;
        }

        ssize_t status = 0;
        if (total < buf.len) {
            if (ret == 0 || ret == MBEDTLS_ERR_SSL_PEER_CLOSE_NOTIFY) {
                status = UV_EOF;
            } else if (ret != MBEDTLS_ERR_SSL_WANT_READ &&
                       ret != MBEDTLS_ERR_SSL_WANT_WRITE) {
                tls_error_string(ret, err, sizeof(err));
                log_debug("HTTPS: read from %s failed: %s", conn->request.client_ip, err);
                status = UV_EPROTO;
            }
        }
        // Reads can queue alerts or post-handshake messages.
        if (s->out_len > 0 && flush_output(conn, NULL, NULL) != 0) status = UV_EPROTO;

        if (total > 0) {
            s->read_cb((uv_stream_t *)&conn->handle, (ssize_t)total, &buf);
            if (status != 0 && session_active(conn, s)) deliver_status(conn, status);
        } else {
            // A zero-length read returns the buffer to its owner.
            s->read_cb((uv_stream_t *)&conn->handle, status, &buf);
        }
        if (status != 0) {
            if (status == UV_EPROTO && conn->tls_session == s) s->failed = true;
            return;
        }
        if (total < buf.len) return;  // Drained; WANT_READ
    }
}

static bool has_buffered_input(libuv_tls_session_t *s) {
    return s->in_off < s->in_len || mbedtls_ssl_get_bytes_avail(&s->ssl) > 0;
}

static void drain_cb(uv_idle_t *handle) {
    uv_idle_stop(handle);
    libuv_tls_session_t *s = handle->data;
    libuv_connection_t *conn = s->conn;
    // session_active() also rejects a closing handle and a stopped reader.
    if (!conn || conn->tls_session != s || !session_active(conn, s)) return;
    process_input(conn);
}

static void schedule_drain(libuv_connection_t *conn, libuv_tls_session_t *s) {
    if (!s->drain_init) {
        if (uv_idle_init(conn->handle.loop, &s->drain) != 0) return;
        s->drain.data = s;
        s->drain_init = true;
    }
    uv_idle_start(&s->drain, drain_cb);
}

static void tls_alloc(uv_handle_t *handle, size_t suggested_size, uv_buf_t *buf) {
    (void)suggested_size;
    libuv_connection_t *conn = handle->data;
    libuv_tls_session_t *s = conn ? conn->tls_session : NULL;
    buf->base = NULL;
    buf->len = 0;
    if (!s || s->failed) return;

    if (s->in_off > 0) {
        memmove(s->in, s->in + s->in_off, s->in_len - s->in_off);
        s->in_len -= s->in_off;
        s->in_off = 0;
    }
    size_t needed = s->in_len + TLS_READ_CHUNK;
    if (needed > TLS_INPUT_MAX) return;  // read_cb sees UV_ENOBUFS
    if (needed > s->in_cap) {
        unsigned char *grown = safe_realloc(s->in, needed);
        if (!grown) return;
        s->in = grown;
        s->in_cap = needed;
    }
    buf->base = (char *)s->in + s->in_len;
    buf->len = TLS_READ_CHUNK;
}

static void tls_read(uv_stream_t *stream, ssize_t nread, const uv_buf_t *buf) {
    (void)buf;  // Points into s->in
    libuv_connection_t *conn = stream->data;
    if (!conn || !conn->tls_session) return;
    libuv_tls_session_t *s = conn->tls_session;
    if (s->failed || nread == 0) return;

    if (nread < 0) {
        if (s->handshake_done) {
            deliver_status(conn, nread);
        } else {
            libuv_connection_close(conn);
        }
        return;
    }

    s->in_len += (size_t)nread;
    process_input(conn);
}

// ---------------------------------------------------------------------------
// Sessions and transport wrappers
// ---------------------------------------------------------------------------

int libuv_tls_session_start(libuv_connection_t *conn, libuv_tls_context_t *ctx) {
    if (!conn || !ctx) return -1;
    libuv_tls_session_t *s = safe_calloc(1, sizeof(*s));
    if (!s) return -1;
    mbedtls_ssl_init(&s->ssl);
    int ret = mbedtls_ssl_setup(&s->ssl, &ctx->conf);
    if (ret != 0) {
        char err[128];
        tls_error_string(ret, err, sizeof(err));
        log_error("HTTPS: cannot set up TLS session: %s", err);
        mbedtls_ssl_free(&s->ssl);
        safe_free(s);
        return -1;
    }
    mbedtls_ssl_set_bio(&s->ssl, s, bio_send, bio_recv, NULL);
    s->conn = conn;
    conn->tls_session = s;
    return 0;
}

static void session_release(libuv_tls_session_t *s) {
    mbedtls_ssl_free(&s->ssl);
    safe_free(s->in);
    safe_free(s->out);
    safe_free(s);
}

static void drain_closed(uv_handle_t *handle) {
    session_release(handle->data);
}

void libuv_tls_session_free(libuv_connection_t *conn) {
    if (!conn || !conn->tls_session) return;
    libuv_tls_session_t *s = conn->tls_session;
    conn->tls_session = NULL;
    s->conn = NULL;
    if (s->drain_init) {
        // The idle handle belongs to the loop until its close callback runs.
        uv_close((uv_handle_t *)&s->drain, drain_closed);
        return;
    }
    session_release(s);
}

bool libuv_connection_is_tls(const libuv_connection_t *conn) {
    return conn && conn->tls_session;
}

int libuv_connection_write(libuv_connection_t *conn, uv_write_t *req,
                           const uv_buf_t bufs[], unsigned int nbufs,
                           uv_write_cb cb) {
    libuv_tls_session_t *s = conn->tls_session;
    if (!s) {
        return uv_write(req, (uv_stream_t *)&conn->handle, bufs, nbufs, cb);
    }
    if (s->failed || !s->handshake_done) return UV_ENOTCONN;
    if (uv_is_closing((uv_handle_t *)&conn->handle)) return UV_EBADF;

    for (unsigned int i = 0; i < nbufs; i++) {
        const unsigned char *data = (const unsigned char *)bufs[i].base;
        size_t remaining = bufs[i].len;
        while (remaining > 0) {
            int ret = mbedtls_ssl_write(&s->ssl, data, remaining);
            if (ret < 0) {
                char err[128];
                tls_error_string(ret, err, sizeof(err));
                log_error("HTTPS: encrypting response for %s failed: %s",
                          conn->request.client_ip, err);
                // Partial records would corrupt the stream; drop them.
                s->failed = true;
                s->out_len = 0;
                return UV_EPROTO;
            }
            data += ret;
            remaining -= (size_t)ret;
        }
    }
    return flush_output(conn, req, cb);
}

int libuv_connection_read_start(libuv_connection_t *conn,
                                uv_alloc_cb alloc_cb, uv_read_cb read_cb) {
    libuv_tls_session_t *s = conn->tls_session;
    if (!s) {
        return uv_read_start((uv_stream_t *)&conn->handle, alloc_cb, read_cb);
    }
    if (s->failed) return UV_EPROTO;
    s->alloc_cb = alloc_cb;
    s->read_cb = read_cb;
    s->reading = true;
    int r = uv_read_start((uv_stream_t *)&conn->handle, tls_alloc, tls_read);
    if (r == UV_EALREADY) r = 0;
    // Input decrypted or received while paused would otherwise wait for the
    // peer to send more, which a client waiting on a response never does.
    if (r == 0 && s->handshake_done && has_buffered_input(s)) {
        schedule_drain(conn, s);
    }
    return r;
}

int libuv_connection_read_stop(libuv_connection_t *conn) {
    libuv_tls_session_t *s = conn->tls_session;
    if (s) {
        s->reading = false;
        if (s->drain_init) uv_idle_stop(&s->drain);
    }
    return uv_read_stop((uv_stream_t *)&conn->handle);
}

#else /* !LIGHTNVR_ENABLE_TLS */

bool libuv_tls_compiled_in(void) {
    return false;
}

libuv_tls_context_t *libuv_tls_context_create(const char *cert_path,
                                              const char *key_path) {
    (void)cert_path;
    (void)key_path;
    log_error("HTTPS: this build has no TLS support (rebuild with -DENABLE_SSL=ON)");
    return NULL;
}

void libuv_tls_context_free(libuv_tls_context_t *ctx) {
    (void)ctx;
}

int libuv_tls_session_start(libuv_connection_t *conn, libuv_tls_context_t *ctx) {
    (void)conn;
    (void)ctx;
    return -1;
}

void libuv_tls_session_free(libuv_connection_t *conn) {
    (void)conn;
}

bool libuv_connection_is_tls(const libuv_connection_t *conn) {
    (void)conn;
    return false;
}

int libuv_connection_write(libuv_connection_t *conn, uv_write_t *req,
                           const uv_buf_t bufs[], unsigned int nbufs,
                           uv_write_cb cb) {
    return uv_write(req, (uv_stream_t *)&conn->handle, bufs, nbufs, cb);
}

int libuv_connection_read_start(libuv_connection_t *conn,
                                uv_alloc_cb alloc_cb, uv_read_cb read_cb) {
    return uv_read_start((uv_stream_t *)&conn->handle, alloc_cb, read_cb);
}

int libuv_connection_read_stop(libuv_connection_t *conn) {
    return uv_read_stop((uv_stream_t *)&conn->handle);
}

#endif /* LIGHTNVR_ENABLE_TLS */

#endif /* HTTP_BACKEND_LIBUV */
