/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

/* HTTPS termination in the libuv web server (#650).
 *
 * A self-signed certificate is generated at test time, the server is started
 * with it, and an mbedTLS client that trusts exactly that certificate drives
 * real requests: API handlers, keep-alive, request bodies and responses that
 * span many TLS records, static files, the Secure cookie flag, the go2rtc
 * WebSocket relay, and fail-closed start-up on a bad certificate or key. */

#define _POSIX_C_SOURCE 200809L
#include <arpa/inet.h>
#include <errno.h>
#include <poll.h>
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/stat.h>
#include <sys/time.h>
#include <unistd.h>

#include <mbedtls/ctr_drbg.h>
#include <mbedtls/ecp.h>
#include <mbedtls/entropy.h>
#include <mbedtls/pk.h>
#include <mbedtls/ssl.h>
#include <mbedtls/version.h>
#include <mbedtls/x509_crt.h>
#if defined(MBEDTLS_PSA_CRYPTO_C)
#include <psa/crypto.h>
#endif

#include "unity.h"
#include "core/config.h"
#include "database/db_core.h"
#include "database/db_streams.h"
#ifdef USE_GO2RTC
#include "video/go2rtc/go2rtc_api.h"
#endif
#include "web/httpd_utils.h"
#include "web/libuv_server.h"
#include "web/libuv_tls.h"

#define BIG_BODY_SIZE ((size_t)300 * 1024)   // Spans ~19 TLS records
#define POST_BODY_SIZE ((size_t)200 * 1024)

static mbedtls_entropy_context entropy;
static mbedtls_ctr_drbg_context drbg;
static char work_dir[64];
static char cert_path[128], key_path[128], other_key_path[128];
static char db_path[160];
static http_server_handle_t server;
static int web_port;
static int upstream = -1;

// ---------------------------------------------------------------------------
// Certificate generation
// ---------------------------------------------------------------------------

#define GEN_CHECK(expr) do { if (!(expr)) { \
    fprintf(stderr, "certificate generation failed: %s\n", #expr); return false; } } while (0)

static bool save_file(const char *path, const void *data, size_t length) {
    FILE *file = fopen(path, "wb");
    if (!file) return false;
    bool ok = fwrite(data, 1, length, file) == length;
    return fclose(file) == 0 && ok;
}

static void write_file(const char *path, const void *data, size_t length) {
    TEST_ASSERT_TRUE(save_file(path, data, length));
}

static bool generate_key(mbedtls_pk_context *key, const char *path) {
    unsigned char pem[2048] = {0};
    GEN_CHECK(mbedtls_pk_setup(key, mbedtls_pk_info_from_type(MBEDTLS_PK_ECKEY)) == 0);
    GEN_CHECK(mbedtls_ecp_gen_key(MBEDTLS_ECP_DP_SECP256R1, mbedtls_pk_ec(*key),
                                  mbedtls_ctr_drbg_random, &drbg) == 0);
    GEN_CHECK(mbedtls_pk_write_key_pem(key, pem, sizeof(pem)) == 0);
    GEN_CHECK(save_file(path, pem, strlen((char *)pem)));
    return true;
}

/* Self-signed EC P-256 certificate for CN=localhost, plus a second,
 * unrelated key for the mismatch test. */
static bool generate_certificate(void) {
    mbedtls_pk_context key, other;
    mbedtls_x509write_cert crt;
    unsigned char pem[4096] = {0};
    bool ok = false;
    mbedtls_pk_init(&key);
    mbedtls_pk_init(&other);
    mbedtls_x509write_crt_init(&crt);
    do {
        if (!generate_key(&key, key_path) || !generate_key(&other, other_key_path)) break;
        mbedtls_x509write_crt_set_version(&crt, MBEDTLS_X509_CRT_VERSION_3);
        mbedtls_x509write_crt_set_md_alg(&crt, MBEDTLS_MD_SHA256);
        mbedtls_x509write_crt_set_subject_key(&crt, &key);
        mbedtls_x509write_crt_set_issuer_key(&crt, &key);
        if (mbedtls_x509write_crt_set_subject_name(&crt, "CN=localhost") != 0 ||
            mbedtls_x509write_crt_set_issuer_name(&crt, "CN=localhost") != 0 ||
            mbedtls_x509write_crt_set_validity(&crt, "20250101000000", "20450101000000") != 0) break;
#if MBEDTLS_VERSION_NUMBER >= 0x03040000
        unsigned char serial[] = {0x01};
        if (mbedtls_x509write_crt_set_serial_raw(&crt, serial, sizeof(serial)) != 0) break;
#else
        mbedtls_mpi serial;
        mbedtls_mpi_init(&serial);
        int serial_ret = mbedtls_mpi_lset(&serial, 1);
        if (serial_ret == 0) serial_ret = mbedtls_x509write_crt_set_serial(&crt, &serial);
        mbedtls_mpi_free(&serial);
        if (serial_ret != 0) break;
#endif
        if (mbedtls_x509write_crt_pem(&crt, pem, sizeof(pem),
                                      mbedtls_ctr_drbg_random, &drbg) != 0) break;
        ok = save_file(cert_path, pem, strlen((char *)pem));
    } while (0);
    mbedtls_x509write_crt_free(&crt);
    mbedtls_pk_free(&key);
    mbedtls_pk_free(&other);
    if (!ok) fprintf(stderr, "certificate generation failed\n");
    return ok;
}

// ---------------------------------------------------------------------------
// TLS client
// ---------------------------------------------------------------------------

typedef struct {
    int fd;
    mbedtls_ssl_context ssl;
    mbedtls_ssl_config conf;
    mbedtls_x509_crt ca;
} tls_client_t;

static int fd_send(void *ctx, const unsigned char *buf, size_t len) {
    ssize_t n;
    do n = send(*(int *)ctx, buf, len, MSG_NOSIGNAL); while (n < 0 && errno == EINTR);
    return n < 0 ? MBEDTLS_ERR_SSL_INTERNAL_ERROR : (int)n;
}

static int fd_recv(void *ctx, unsigned char *buf, size_t len) {
    ssize_t n;
    do n = recv(*(int *)ctx, buf, len, 0); while (n < 0 && errno == EINTR);
    if (n == 0) return MBEDTLS_ERR_SSL_PEER_CLOSE_NOTIFY;
    return n < 0 ? MBEDTLS_ERR_SSL_TIMEOUT : (int)n;
}

static int tcp_connect(void) {
    int fd = socket(AF_INET, SOCK_STREAM, 0);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, fd);
    struct timeval timeout = {.tv_sec = 5};
    setsockopt(fd, SOL_SOCKET, SO_RCVTIMEO, &timeout, sizeof(timeout));
    setsockopt(fd, SOL_SOCKET, SO_SNDTIMEO, &timeout, sizeof(timeout));
    struct sockaddr_in address = {.sin_family = AF_INET,
        .sin_addr.s_addr = htonl(INADDR_LOOPBACK), .sin_port = htons((uint16_t)web_port)};
    TEST_ASSERT_EQUAL_INT(0, connect(fd, (struct sockaddr *)&address, sizeof(address)));
    return fd;
}

static int tls_open(tls_client_t *client) {
    memset(client, 0, sizeof(*client));
    client->fd = tcp_connect();
    mbedtls_ssl_init(&client->ssl);
    mbedtls_ssl_config_init(&client->conf);
    mbedtls_x509_crt_init(&client->ca);
    TEST_ASSERT_EQUAL_INT(0, mbedtls_x509_crt_parse_file(&client->ca, cert_path));
    TEST_ASSERT_EQUAL_INT(0, mbedtls_ssl_config_defaults(&client->conf, MBEDTLS_SSL_IS_CLIENT,
        MBEDTLS_SSL_TRANSPORT_STREAM, MBEDTLS_SSL_PRESET_DEFAULT));
    // Trust exactly the generated certificate and check the host name.
    mbedtls_ssl_conf_authmode(&client->conf, MBEDTLS_SSL_VERIFY_REQUIRED);
    mbedtls_ssl_conf_ca_chain(&client->conf, &client->ca, NULL);
    mbedtls_ssl_conf_rng(&client->conf, mbedtls_ctr_drbg_random, &drbg);
    TEST_ASSERT_EQUAL_INT(0, mbedtls_ssl_setup(&client->ssl, &client->conf));
    TEST_ASSERT_EQUAL_INT(0, mbedtls_ssl_set_hostname(&client->ssl, "localhost"));
    mbedtls_ssl_set_bio(&client->ssl, &client->fd, fd_send, fd_recv, NULL);
    return mbedtls_ssl_handshake(&client->ssl);
}

static void tls_close(tls_client_t *client) {
    close(client->fd);
    mbedtls_ssl_free(&client->ssl);
    mbedtls_ssl_config_free(&client->conf);
    mbedtls_x509_crt_free(&client->ca);
}

static bool tls_send_all(tls_client_t *client, const void *data, size_t length) {
    const unsigned char *bytes = data;
    while (length) {
        int n = mbedtls_ssl_write(&client->ssl, bytes, length);
        if (n <= 0) return false;
        bytes += n;
        length -= (size_t)n;
    }
    return true;
}

static bool tls_read_all(tls_client_t *client, void *data, size_t length) {
    unsigned char *bytes = data;
    while (length) {
        int n = mbedtls_ssl_read(&client->ssl, bytes, length);
        if (n <= 0) return false;
        bytes += n;
        length -= (size_t)n;
    }
    return true;
}

static bool tls_read_headers(tls_client_t *client, char *data, size_t capacity) {
    size_t used = 0;
    while (used + 1 < capacity) {
        if (!tls_read_all(client, data + used, 1)) return false;
        data[++used] = '\0';
        if (used >= 4 && memcmp(data + used - 4, "\r\n\r\n", 4) == 0) return true;
    }
    return false;
}

/* Send one request and read the response; returns the status code. The
 * caller frees *body. */
static int tls_request(tls_client_t *client, const char *method, const char *path,
                       const char *body, size_t body_length,
                       char *headers, size_t headers_size, char **response_body,
                       size_t *response_length) {
    char request[512];
    int length = snprintf(request, sizeof(request),
        "%s %s HTTP/1.1\r\nHost: localhost:%d\r\nContent-Length: %zu\r\n\r\n",
        method, path, web_port, body_length);
    TEST_ASSERT_TRUE(tls_send_all(client, request, (size_t)length));
    if (body_length) TEST_ASSERT_TRUE(tls_send_all(client, body, body_length));
    TEST_ASSERT_TRUE(tls_read_headers(client, headers, headers_size));
    int status = 0;
    TEST_ASSERT_EQUAL_INT(1, sscanf(headers, "HTTP/1.1 %d", &status));
    const char *content_length = strstr(headers, "Content-Length: ");
    TEST_ASSERT_NOT_NULL(content_length);
    size_t size = strtoul(content_length + 16, NULL, 10);
    char *data = malloc(size + 1);
    TEST_ASSERT_NOT_NULL(data);
    TEST_ASSERT_TRUE(tls_read_all(client, data, size));
    data[size] = '\0';
    *response_body = data;
    *response_length = size;
    return status;
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

static void hello_handler(const http_request_t *req, http_response_t *res) {
    (void)req;
    http_response_set_json(res, 200, "{\"hello\":\"tls\"}");
}

static void big_handler(const http_request_t *req, http_response_t *res) {
    (void)req;
    char *body = malloc(BIG_BODY_SIZE + 1);
    for (size_t i = 0; i < BIG_BODY_SIZE; i++) body[i] = (char)('a' + i % 26);
    body[BIG_BODY_SIZE] = '\0';
    http_response_set_body(res, body);
    free(body);
    res->status_code = 200;
}

static void echo_length_handler(const http_request_t *req, http_response_t *res) {
    size_t checksum = 0;
    for (size_t i = 0; i < req->body_len; i++) checksum += ((const unsigned char *)req->body)[i];
    char json[128];
    snprintf(json, sizeof(json), "{\"length\":%zu,\"sum\":%zu}", req->body_len, checksum);
    http_response_set_json(res, 200, json);
}

static void cookie_handler(const http_request_t *req, http_response_t *res) {
    (void)req;
    httpd_add_session_cookie(res, "test-token");
    http_response_set_json(res, 200, "{}");
}

// ---------------------------------------------------------------------------
// Fixtures
// ---------------------------------------------------------------------------

static http_server_handle_t start_server(const char *cert, const char *key) {
    http_server_config_t config = {0};
    config.bind_ip = "127.0.0.1";
    config.web_root = work_dir;
    config.max_connections = 16;
    config.ssl_enabled = true;
    snprintf(config.cert_path, sizeof(config.cert_path), "%s", cert);
    snprintf(config.key_path, sizeof(config.key_path), "%s", key);
    return libuv_server_init(&config);
}

void setUp(void) {
    load_default_config(&g_config);
    g_config.web_auth_enabled = false;
    g_config.demo_mode = false;
    g_config.web_compression_enabled = false;

    snprintf(db_path, sizeof(db_path), "%s/test.db", work_dir);
    TEST_ASSERT_EQUAL_INT(0, init_database(db_path));
    stream_config_t camera = {0};
    snprintf(camera.name, sizeof(camera.name), "Camera 0");
    strcpy(camera.url, "rtsp://test/live");
    camera.enabled = true;
    TEST_ASSERT_NOT_EQUAL(0, add_stream_config(&camera));

    upstream = socket(AF_INET, SOCK_STREAM, 0);
    struct sockaddr_in address = {.sin_family = AF_INET, .sin_addr.s_addr = htonl(INADDR_LOOPBACK)};
    TEST_ASSERT_EQUAL_INT(0, bind(upstream, (struct sockaddr *)&address, sizeof(address)));
    TEST_ASSERT_EQUAL_INT(0, listen(upstream, 4));
    socklen_t size = sizeof(address);
    getsockname(upstream, (struct sockaddr *)&address, &size);
    g_config.go2rtc_api_port = ntohs(address.sin_port);
#ifdef USE_GO2RTC
    go2rtc_api_apply_base_path_override(NULL, 0);
#endif

    server = start_server(cert_path, key_path);
    TEST_ASSERT_NOT_NULL(server);
    TEST_ASSERT_EQUAL_INT(0, http_server_register_handler(server, "/api/tls/hello", "GET", hello_handler));
    TEST_ASSERT_EQUAL_INT(0, http_server_register_handler(server, "/api/tls/big", "GET", big_handler));
    TEST_ASSERT_EQUAL_INT(0, http_server_register_handler(server, "/api/tls/echo", "POST", echo_length_handler));
    TEST_ASSERT_EQUAL_INT(0, http_server_register_handler(server, "/api/tls/cookie", "GET", cookie_handler));
    TEST_ASSERT_EQUAL_INT(0, http_server_start(server));
    int address_size = sizeof(address);
    uv_tcp_getsockname(&((libuv_server_t *)server)->listener, (struct sockaddr *)&address, &address_size);
    web_port = ntohs(address.sin_port);
}

void tearDown(void) {
    if (server) {
        http_server_stop(server);
        http_server_destroy(server);
        server = NULL;
    }
    if (upstream >= 0) close(upstream);
    upstream = -1;
    shutdown_database();
    unlink(db_path);
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

void test_handshake_api_request_and_keep_alive(void) {
    tls_client_t client;
    TEST_ASSERT_EQUAL_INT(0, tls_open(&client));
    TEST_ASSERT_NOT_NULL(strstr(mbedtls_ssl_get_version(&client.ssl), "TLSv1."));
    TEST_ASSERT_NULL(strstr(mbedtls_ssl_get_version(&client.ssl), "TLSv1.1"));

    char headers[2048];
    char *body = NULL;
    size_t length = 0;
    for (int i = 0; i < 3; i++) {  // Same TLS session: keep-alive works
        TEST_ASSERT_EQUAL_INT(200, tls_request(&client, "GET", "/api/tls/hello", NULL, 0,
                                               headers, sizeof(headers), &body, &length));
        TEST_ASSERT_EQUAL_STRING("{\"hello\":\"tls\"}", body);
        free(body);
    }
    tls_close(&client);
}

void test_large_response_and_request_body_span_many_records(void) {
    tls_client_t client;
    TEST_ASSERT_EQUAL_INT(0, tls_open(&client));
    char headers[2048];
    char *body = NULL;
    size_t length = 0;

    TEST_ASSERT_EQUAL_INT(200, tls_request(&client, "GET", "/api/tls/big", NULL, 0,
                                           headers, sizeof(headers), &body, &length));
    TEST_ASSERT_EQUAL_size_t(BIG_BODY_SIZE, length);
    for (size_t i = 0; i < length; i++) {
        if (body[i] != (char)('a' + i % 26)) TEST_FAIL_MESSAGE("response body corrupted");
    }
    free(body);

    char *upload = malloc(POST_BODY_SIZE);
    size_t checksum = 0;
    for (size_t i = 0; i < POST_BODY_SIZE; i++) {
        upload[i] = (char)(i * 7);
        checksum += (unsigned char)upload[i];
    }
    TEST_ASSERT_EQUAL_INT(200, tls_request(&client, "POST", "/api/tls/echo", upload, POST_BODY_SIZE,
                                           headers, sizeof(headers), &body, &length));
    char expected[128];
    snprintf(expected, sizeof(expected), "{\"length\":%zu,\"sum\":%zu}", POST_BODY_SIZE, checksum);
    TEST_ASSERT_EQUAL_STRING(expected, body);
    free(body);
    free(upload);
    tls_close(&client);
}

void test_static_file_is_served_over_tls(void) {
    char path[160];
    snprintf(path, sizeof(path), "%s/asset.bin", work_dir);
    size_t size = 150 * 1024;  // Several 64 KiB file-serve chunks
    unsigned char *data = malloc(size);
    for (size_t i = 0; i < size; i++) data[i] = (unsigned char)(i * 13 + 5);
    write_file(path, data, size);

    tls_client_t client;
    TEST_ASSERT_EQUAL_INT(0, tls_open(&client));
    char headers[2048];
    char *body = NULL;
    size_t length = 0;
    TEST_ASSERT_EQUAL_INT(200, tls_request(&client, "GET", "/asset.bin", NULL, 0,
                                           headers, sizeof(headers), &body, &length));
    TEST_ASSERT_EQUAL_size_t(size, length);
    TEST_ASSERT_EQUAL_MEMORY(data, body, size);
    free(body);
    free(data);
    tls_close(&client);
    unlink(path);
}

void test_session_cookie_is_secure_while_tls_is_on(void) {
    TEST_ASSERT_TRUE(httpd_secure_cookies());
    tls_client_t client;
    TEST_ASSERT_EQUAL_INT(0, tls_open(&client));
    char headers[2048];
    char *body = NULL;
    size_t length = 0;
    TEST_ASSERT_EQUAL_INT(200, tls_request(&client, "GET", "/api/tls/cookie", NULL, 0,
                                           headers, sizeof(headers), &body, &length));
    free(body);
    TEST_ASSERT_NOT_NULL(strstr(headers, "Set-Cookie: session=test-token;"));
    TEST_ASSERT_NOT_NULL(strstr(headers, "HttpOnly; SameSite=Lax; Secure\r\n"));
    tls_close(&client);

    http_server_stop(server);
    http_server_destroy(server);
    server = NULL;
    TEST_ASSERT_FALSE(httpd_secure_cookies());
}

void test_plain_http_and_garbage_do_not_break_the_listener(void) {
    int fd = tcp_connect();
    const char plain[] = "GET /api/tls/hello HTTP/1.1\r\nHost: localhost\r\n\r\n";
    TEST_ASSERT_EQUAL_INT((int)sizeof(plain) - 1, (int)send(fd, plain, sizeof(plain) - 1, 0));
    char reply[512];
    ssize_t n;
    // The server answers with at most a TLS alert and closes; never HTTP.
    while ((n = recv(fd, reply, sizeof(reply) - 1, 0)) > 0) {
        reply[n] = '\0';
        TEST_ASSERT_NULL(strstr(reply, "HTTP/1.1"));
    }
    TEST_ASSERT_TRUE(n <= 0);
    close(fd);

    // A client that disconnects mid-handshake.
    fd = tcp_connect();
    const unsigned char partial_hello[] = {0x16, 0x03, 0x01, 0x00, 0x80, 0x01};
    send(fd, partial_hello, sizeof(partial_hello), 0);
    close(fd);

    tls_client_t client;
    TEST_ASSERT_EQUAL_INT(0, tls_open(&client));
    char headers[2048];
    char *body = NULL;
    size_t length = 0;
    TEST_ASSERT_EQUAL_INT(200, tls_request(&client, "GET", "/api/tls/hello", NULL, 0,
                                           headers, sizeof(headers), &body, &length));
    free(body);
    tls_close(&client);
}

#ifdef USE_GO2RTC
static const unsigned char ws_request_frame[] = {
    0x81, 0x8e, 0, 0, 0, 0, '{', '"', 't', 'y', 'p', 'e', '"', ':', '"', 'm', 's', 'e', '"', '}'
};
static const unsigned char ws_media_frame[] = {0x82, 0x08, 0, 0, 0, 8, 'f', 't', 'y', 'p'};
static const char ws_upgrade_response[] =
    "HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\n"
    "Sec-WebSocket-Accept: s3pPLMBiTxaQ9kYGzzhZRbK+xOo=\r\n\r\n";

typedef struct {
    char request[4096];
    bool received_frame;
} ws_peer_t;

static void *ws_upstream(void *arg) {
    ws_peer_t *peer = arg;
    struct pollfd ready = {.fd = upstream, .events = POLLIN};
    if (poll(&ready, 1, 5000) <= 0) return NULL;
    int fd = accept(upstream, NULL, NULL);
    if (fd < 0) return NULL;
    struct timeval timeout = {.tv_sec = 5};
    setsockopt(fd, SOL_SOCKET, SO_RCVTIMEO, &timeout, sizeof(timeout));
    size_t used = 0;
    while (used + 1 < sizeof(peer->request) &&
           recv(fd, peer->request + used, 1, 0) == 1) {
        peer->request[++used] = '\0';
        if (used >= 4 && memcmp(peer->request + used - 4, "\r\n\r\n", 4) == 0) break;
    }
    send(fd, ws_upgrade_response, strlen(ws_upgrade_response), MSG_NOSIGNAL);
    send(fd, ws_media_frame, sizeof(ws_media_frame), MSG_NOSIGNAL);
    unsigned char frame[sizeof(ws_request_frame)];
    size_t got = 0;
    while (got < sizeof(frame)) {
        ssize_t n = recv(fd, frame + got, sizeof(frame) - got, 0);
        if (n <= 0) break;
        got += (size_t)n;
    }
    if (got == sizeof(frame) && memcmp(frame, ws_request_frame, sizeof(frame)) == 0) {
        peer->received_frame = true;
        send(fd, ws_media_frame, sizeof(ws_media_frame), MSG_NOSIGNAL);
    }
    char byte;
    recv(fd, &byte, 1, 0);  // Hold until the relay closes
    close(fd);
    return NULL;
}

void test_go2rtc_websocket_relays_over_tls(void) {
    ws_peer_t peer = {0};
    pthread_t thread;
    TEST_ASSERT_EQUAL_INT(0, pthread_create(&thread, NULL, ws_upstream, &peer));

    tls_client_t client;
    TEST_ASSERT_EQUAL_INT(0, tls_open(&client));
    char request[1024];
    int length = snprintf(request, sizeof(request),
        "GET /go2rtc/api/ws?src=Camera%%200_sub HTTP/1.1\r\nHost: localhost:%d\r\n"
        "Origin: https://localhost:%d\r\nUpgrade: websocket\r\n"
        "Connection: keep-alive, Upgrade\r\nSec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\n"
        "Sec-WebSocket-Version: 13\r\n\r\n", web_port, web_port);
    TEST_ASSERT_TRUE(tls_send_all(&client, request, (size_t)length));

    char headers[1024];
    TEST_ASSERT_TRUE(tls_read_headers(&client, headers, sizeof(headers)));
    TEST_ASSERT_EQUAL_STRING(ws_upgrade_response, headers);
    unsigned char media[sizeof(ws_media_frame)];
    TEST_ASSERT_TRUE(tls_read_all(&client, media, sizeof(media)));
    TEST_ASSERT_EQUAL_MEMORY(ws_media_frame, media, sizeof(media));
    TEST_ASSERT_TRUE(tls_send_all(&client, ws_request_frame, sizeof(ws_request_frame)));
    TEST_ASSERT_TRUE(tls_read_all(&client, media, sizeof(media)));
    TEST_ASSERT_EQUAL_MEMORY(ws_media_frame, media, sizeof(media));

    tls_close(&client);
    pthread_join(thread, NULL);
    TEST_ASSERT_TRUE(peer.received_frame);
    TEST_ASSERT_NULL(strstr(peer.request, "Origin:"));
}
#endif /* USE_GO2RTC */

void test_bad_certificate_or_key_refuses_to_start(void) {
    http_server_stop(server);
    http_server_destroy(server);
    server = NULL;

    char missing[160];
    snprintf(missing, sizeof(missing), "%s/missing.pem", work_dir);
    TEST_ASSERT_NULL(start_server(missing, key_path));
    TEST_ASSERT_NULL(start_server(cert_path, missing));
    TEST_ASSERT_NULL(start_server("", ""));
    TEST_ASSERT_NULL(start_server(cert_path, other_key_path));  // Key/cert mismatch
    TEST_ASSERT_NULL(start_server(key_path, key_path));         // Not a certificate
    TEST_ASSERT_FALSE(httpd_secure_cookies());
}

int main(void) {
    if (!libuv_tls_compiled_in()) {
        fprintf(stderr, "built without LIGHTNVR_ENABLE_TLS\n");
        return 1;
    }
#if defined(MBEDTLS_PSA_CRYPTO_C)
    psa_crypto_init();
#endif
    mbedtls_entropy_init(&entropy);
    mbedtls_ctr_drbg_init(&drbg);
    if (mbedtls_ctr_drbg_seed(&drbg, mbedtls_entropy_func, &entropy, NULL, 0) != 0) return 1;

    snprintf(work_dir, sizeof(work_dir), "/tmp/lightnvr-tls-test-XXXXXX");
    if (!mkdtemp(work_dir)) return 1;
    snprintf(cert_path, sizeof(cert_path), "%s/cert.pem", work_dir);
    snprintf(key_path, sizeof(key_path), "%s/key.pem", work_dir);
    snprintf(other_key_path, sizeof(other_key_path), "%s/other-key.pem", work_dir);

    if (!generate_certificate()) return 1;

    UNITY_BEGIN();
    RUN_TEST(test_handshake_api_request_and_keep_alive);
    RUN_TEST(test_large_response_and_request_body_span_many_records);
    RUN_TEST(test_static_file_is_served_over_tls);
    RUN_TEST(test_session_cookie_is_secure_while_tls_is_on);
    RUN_TEST(test_plain_http_and_garbage_do_not_break_the_listener);
#ifdef USE_GO2RTC
    RUN_TEST(test_go2rtc_websocket_relays_over_tls);
#endif
    RUN_TEST(test_bad_certificate_or_key_refuses_to_start);
    int result = UNITY_END();

    unlink(cert_path);
    unlink(key_path);
    unlink(other_key_path);
    rmdir(work_dir);
    mbedtls_ctr_drbg_free(&drbg);
    mbedtls_entropy_free(&entropy);
    return result;
}
