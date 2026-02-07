/**
 * @file mongoose_adapter_bridge.c
 * @brief Bridge between mongoose types and backend-agnostic HTTP types
 *
 * Provides conversion functions:
 *   mg_http_message → http_request_t
 *   http_response_t → mg_http_reply
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>

#include "mongoose.h"
#include "web/http_server.h"
#include "web/request_response.h"
#include "web/mongoose_adapter.h"
#include "core/logger.h"

/**
 * Parse method string to enum
 */
static http_method_t parse_method(const char *method) {
    if (strcasecmp(method, "GET") == 0) return HTTP_METHOD_GET;
    if (strcasecmp(method, "POST") == 0) return HTTP_METHOD_POST;
    if (strcasecmp(method, "PUT") == 0) return HTTP_METHOD_PUT;
    if (strcasecmp(method, "DELETE") == 0) return HTTP_METHOD_DELETE;
    if (strcasecmp(method, "OPTIONS") == 0) return HTTP_METHOD_OPTIONS;
    if (strcasecmp(method, "HEAD") == 0) return HTTP_METHOD_HEAD;
    if (strcasecmp(method, "PATCH") == 0) return HTTP_METHOD_PATCH;
    return HTTP_METHOD_UNKNOWN;
}

int http_server_mg_to_request(struct mg_connection *conn, struct mg_http_message *msg,
                              http_request_t *request) {
    if (!conn || !msg || !request) return -1;

    http_request_init(request);

    // Method
    mg_str_copy(&msg->method, request->method_str, sizeof(request->method_str));
    request->method = parse_method(request->method_str);

    // URI (full)
    mg_str_copy(&msg->uri, request->uri, sizeof(request->uri));

    // Path (URI without query string)
    mg_str_copy(&msg->uri, request->path, sizeof(request->path));
    char *qmark = strchr(request->path, '?');
    if (qmark) *qmark = '\0';

    // Query string
    mg_str_copy(&msg->query, request->query_string, sizeof(request->query_string));

    // Headers
    request->num_headers = 0;
    for (int i = 0; i < MG_MAX_HTTP_HEADERS && msg->headers[i].name.len > 0; i++) {
        if (request->num_headers >= MAX_HEADERS) break;
        mg_str_copy(&msg->headers[i].name,
                     request->headers[request->num_headers].name,
                     sizeof(request->headers[request->num_headers].name));
        mg_str_copy(&msg->headers[i].value,
                     request->headers[request->num_headers].value,
                     sizeof(request->headers[request->num_headers].value));
        request->num_headers++;
    }

    // Content-Type from headers
    struct mg_str *ct = mg_http_get_header(msg, "Content-Type");
    if (ct) {
        mg_str_copy(ct, request->content_type, sizeof(request->content_type));
    }

    // Content-Length
    struct mg_str *cl = mg_http_get_header(msg, "Content-Length");
    if (cl) {
        char cl_buf[32] = {0};
        mg_str_copy(cl, cl_buf, sizeof(cl_buf));
        request->content_length = strtoull(cl_buf, NULL, 10);
    }

    // User-Agent
    struct mg_str *ua = mg_http_get_header(msg, "User-Agent");
    if (ua) {
        mg_str_copy(ua, request->user_agent, sizeof(request->user_agent));
    }

    // Body
    if (msg->body.len > 0 && msg->body.buf) {
        request->body = (void *)msg->body.buf;
        request->body_len = msg->body.len;
    }

    // User data (pass server pointer)
    request->user_data = conn->fn_data;

    return 0;
}

int http_server_send_response(struct mg_connection *conn, const http_response_t *response) {
    if (!conn || !response) return -1;

    // Build extra headers string
    char headers_buf[4096] = {0};
    size_t offset = 0;

    // Content-Type
    if (response->content_type[0] != '\0') {
        offset += snprintf(headers_buf + offset, sizeof(headers_buf) - offset,
                          "Content-Type: %s\r\n", response->content_type);
    }

    // Additional headers
    for (int i = 0; i < response->num_headers; i++) {
        // Skip Content-Type since we handle it above
        if (strcasecmp(response->headers[i].name, "Content-Type") == 0) continue;
        offset += snprintf(headers_buf + offset, sizeof(headers_buf) - offset,
                          "%s: %s\r\n",
                          response->headers[i].name,
                          response->headers[i].value);
    }

    // Connection: close
    offset += snprintf(headers_buf + offset, sizeof(headers_buf) - offset,
                      "Connection: close\r\n");

    // Send response
    if (response->body && response->body_length > 0) {
        mg_http_reply(conn, response->status_code, headers_buf,
                     "%.*s", (int)response->body_length, (const char *)response->body);
    } else {
        mg_http_reply(conn, response->status_code, headers_buf, "");
    }

    return 0;
}

