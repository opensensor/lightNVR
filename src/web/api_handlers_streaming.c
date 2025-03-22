// This file provides empty implementations of the HLS API functions
// The actual HLS handling is done directly in mongoose_server_static.c

#include <stdio.h>
#include <stdlib.h>
#include "web/mongoose_adapter.h"
#include "core/logger.h"

// Empty implementations to satisfy linker references
void mg_handle_hls_master_playlist(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("HLS API: Redirecting master playlist request to direct handler");
    mg_http_reply(c, 301, "Location: /hls/\r\n", "");
}

void mg_handle_hls_media_playlist(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("HLS API: Redirecting media playlist request to direct handler");
    mg_http_reply(c, 301, "Location: /hls/\r\n", "");
}

void mg_handle_hls_segment(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("HLS API: Redirecting segment request to direct handler");
    mg_http_reply(c, 301, "Location: /hls/\r\n", "");
}
