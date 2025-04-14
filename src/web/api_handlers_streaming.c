// This file provides implementations of the HLS API functions

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include "web/mongoose_adapter.h"
#include "core/logger.h"
#include "core/config.h"
#include "web/http_server.h"
#include "video/streams.h"


void mg_handle_direct_hls_request(struct mg_connection *c, struct mg_http_message *hm) {
    if (!c || !hm) {
        log_error("Invalid parameters in mg_handle_direct_hls_request");
        return;
    }

    log_info("HLS API: Handling direct HLS request");

    // Extract URI
    char uri[MAX_PATH_LENGTH];
    size_t uri_len = hm->uri.len < sizeof(uri) - 1 ? hm->uri.len : sizeof(uri) - 1;
    memcpy(uri, hm->uri.buf, uri_len);
    uri[uri_len] = '\0';

    // Extract stream name from URI
    // URI format: /hls/{stream_name}/{file}
    char stream_name[MAX_STREAM_NAME] = {0};
    char decoded_stream_name[MAX_STREAM_NAME] = {0};

    // Validate URI format
    if (uri_len < 6 || strncmp(uri, "/hls/", 5) != 0) {
        log_error("Invalid HLS URI format: %s", uri);
        mg_http_reply(c, 400, "", "{\"error\": \"Invalid HLS path\"}\n");
        return;
    }

    const char *stream_start = uri + 5; // Skip "/hls/"
    const char *file_part = strchr(stream_start, '/');

    if (!file_part) {
        log_error("Failed to extract stream name from URI: %s", uri);
        mg_http_reply(c, 400, "", "{\"error\": \"Invalid HLS path\"}\n");
        return;
    }

    // Extract stream name
    size_t name_len = file_part - stream_start;
    if (name_len == 0) {
        log_error("Empty stream name in URI: %s", uri);
        mg_http_reply(c, 400, "", "{\"error\": \"Invalid HLS path - empty stream name\"}\n");
        return;
    }

    if (name_len >= MAX_STREAM_NAME) {
        name_len = MAX_STREAM_NAME - 1;
    }
    strncpy(stream_name, stream_start, name_len);
    stream_name[name_len] = '\0';

    // URL decode the stream name
    mg_url_decode(stream_name, strlen(stream_name), decoded_stream_name, sizeof(decoded_stream_name), 0);

    // Extract file name (everything after the stream name)
    const char *file_name = file_part + 1; // Skip "/"
    if (!file_name || *file_name == '\0') {
        log_error("Empty file name in URI: %s", uri);
        mg_http_reply(c, 400, "", "{\"error\": \"Invalid HLS path - empty file name\"}\n");
        return;
    }

    // Get the config to find the storage path - make a local copy of needed values
    config_t *global_config = get_streaming_config();
    if (!global_config) {
        log_error("Failed to get streaming configuration");
        mg_http_reply(c, 500, "", "{\"error\": \"Internal server error\"}\n");
        return;
    }

    // Make local copies of the storage paths to avoid race conditions
    char storage_path[MAX_PATH_LENGTH] = {0};
    char storage_path_hls[MAX_PATH_LENGTH] = {0};

    // Copy the storage paths with bounds checking
    strncpy(storage_path, global_config->storage_path, MAX_PATH_LENGTH - 1);
    storage_path[MAX_PATH_LENGTH - 1] = '\0';

    strncpy(storage_path_hls, global_config->storage_path_hls, MAX_PATH_LENGTH - 1);
    storage_path_hls[MAX_PATH_LENGTH - 1] = '\0';

    // Construct the full path to the HLS file
    char hls_file_path[MAX_PATH_LENGTH * 2]; // Double the buffer size to avoid truncation

    // Use storage_path_hls if specified, otherwise fall back to storage_path
    if (storage_path_hls[0] != '\0') {
        snprintf(hls_file_path, sizeof(hls_file_path), "%s/hls/%s/%s",
                storage_path_hls, decoded_stream_name, file_name);
        log_info("Using HLS-specific storage path: %s", storage_path_hls);
    } else {
        snprintf(hls_file_path, sizeof(hls_file_path), "%s/hls/%s/%s",
                storage_path, decoded_stream_name, file_name);
        log_info("Using default storage path for HLS: %s", storage_path);
    }

    log_info("Serving HLS file directly: %s", hls_file_path);

    // Check if file exists
    struct stat st;
    if (stat(hls_file_path, &st) == 0 && S_ISREG(st.st_mode)) {
        // Determine content type based on file extension
        const char *content_type_header = "Content-Type: application/octet-stream\r\n";
        if (strstr(file_name, ".m3u8")) {
            content_type_header = "Content-Type: application/vnd.apple.mpegurl\r\n";
        } else if (strstr(file_name, ".ts")) {
            content_type_header = "Content-Type: video/mp2t\r\n";
        } else if (strstr(file_name, ".m4s")) {
            content_type_header = "Content-Type: video/iso.segment\r\n";
        } else if (strstr(file_name, "init.mp4")) {
            content_type_header = "Content-Type: video/mp4\r\n";
        }

        // Use more mobile-friendly cache headers with longer cache times
        char headers[512];

        // Different cache settings for different file types
        const char* cache_control;
        if (strstr(file_name, ".m3u8")) {
            // For playlist files, use a shorter cache time to ensure updates are seen
            cache_control = "Cache-Control: no-cache, no-store, must-revalidate\r\n";
        } else if (strstr(file_name, ".ts") || strstr(file_name, ".m4s")) {
            // For media segments, use a longer cache time to improve mobile performance
            cache_control = "Cache-Control: no-cache, no-store, must-revalidate\r\n";
        } else if (strstr(file_name, "init.mp4")) {
            // For initialization segments, use a longer cache time
            cache_control = "Cache-Control: no-cache, no-store, must-revalidate\r\n";
        } else {
            // Default cache time
            cache_control = "Cache-Control: no-cache, no-store, must-revalidate\r\n";
        }

        snprintf(headers, sizeof(headers),
            "%s"
            "%s"  // Dynamic cache control based on file type
            "Connection: close\r\n"
            "Access-Control-Allow-Origin: *\r\n"
            "Access-Control-Allow-Methods: GET, OPTIONS\r\n"
            "Access-Control-Allow-Headers: Origin, Content-Type, Accept, Authorization\r\n",
            content_type_header, cache_control);

        mg_http_serve_file(c, hm, hls_file_path, &(struct mg_http_serve_opts){
            .mime_types = "",
            .extra_headers = headers
        });
    } else {
        // File doesn't exist - let the client know
        log_info("HLS file not found: %s (waiting for FFmpeg to create it)", hls_file_path);

        // Return a 404 with a message that indicates the file is being generated
        mg_http_reply(c, 404, "", "{\"error\": \"HLS file not found or still being generated by FFmpeg\"}\n");
    }
}
