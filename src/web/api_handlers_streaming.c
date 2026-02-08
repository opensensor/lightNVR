// This file provides implementations of the HLS API functions

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include "web/request_response.h"
#include "web/httpd_utils.h"
#include "core/logger.h"
#include "core/config.h"
#include "web/http_server.h"
#include "video/streams.h"

/**
 * @brief Backend-agnostic handler for direct HLS requests
 * Endpoint: /hls/{stream_name}/{file}
 */
void handle_direct_hls_request(const http_request_t *req, http_response_t *res) {
    if (!req || !res) {
        log_error("Invalid parameters in handle_direct_hls_request");
        return;
    }

    log_info("HLS API: Handling direct HLS request");

    // Extract URI
    const char *uri = req->path;
    size_t uri_len = strlen(uri);

    // Extract stream name from URI
    // URI format: /hls/{stream_name}/{file}
    char stream_name[MAX_STREAM_NAME] = {0};
    char decoded_stream_name[MAX_STREAM_NAME] = {0};

    // Validate URI format
    if (uri_len < 6 || strncmp(uri, "/hls/", 5) != 0) {
        log_error("Invalid HLS URI format: %s", uri);
        http_response_set_json_error(res, 400, "Invalid HLS path");
        return;
    }

    const char *stream_start = uri + 5; // Skip "/hls/"
    const char *file_part = strchr(stream_start, '/');

    if (!file_part) {
        log_error("Failed to extract stream name from URI: %s", uri);
        http_response_set_json_error(res, 400, "Invalid HLS path");
        return;
    }

    // Extract stream name
    size_t name_len = file_part - stream_start;
    if (name_len == 0) {
        log_error("Empty stream name in URI: %s", uri);
        http_response_set_json_error(res, 400, "Invalid HLS path - empty stream name");
        return;
    }

    if (name_len >= MAX_STREAM_NAME) {
        name_len = MAX_STREAM_NAME - 1;
    }
    strncpy(stream_name, stream_start, name_len);
    stream_name[name_len] = '\0';

    // URL decode the stream name
    url_decode(stream_name, decoded_stream_name, sizeof(decoded_stream_name));

    // Extract file name (everything after the stream name)
    const char *file_name = file_part + 1; // Skip "/"
    if (!file_name || *file_name == '\0') {
        log_error("Empty file name in URI: %s", uri);
        http_response_set_json_error(res, 400, "Invalid HLS path - empty file name");
        return;
    }

    // Get the config to find the storage path - make a local copy of needed values
    config_t *global_config = get_streaming_config();
    if (!global_config) {
        log_error("Failed to get streaming configuration");
        http_response_set_json_error(res, 500, "Internal server error");
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
        const char *content_type = "application/octet-stream";
        if (strstr(file_name, ".m3u8")) {
            content_type = "application/vnd.apple.mpegurl";
        } else if (strstr(file_name, ".ts")) {
            content_type = "video/mp2t";
        } else if (strstr(file_name, ".m4s")) {
            content_type = "video/iso.segment";
        } else if (strstr(file_name, "init.mp4")) {
            content_type = "video/mp4";
        }

        // Build extra headers with cache control and CORS
        char extra_headers[512];
        snprintf(extra_headers, sizeof(extra_headers),
            "Cache-Control: no-cache, no-store, must-revalidate\r\n"
            "Connection: close\r\n"
            "Access-Control-Allow-Origin: *\r\n"
            "Access-Control-Allow-Methods: GET, OPTIONS\r\n"
            "Access-Control-Allow-Headers: Origin, Content-Type, Accept, Authorization\r\n");

        // Serve the file using backend-agnostic function
        http_serve_file(req, res, hls_file_path, content_type, extra_headers);
    } else {
        // File doesn't exist - let the client know
        log_info("HLS file not found: %s (waiting for FFmpeg to create it)", hls_file_path);

        // Return a 404 with a message that indicates the file is being generated
        http_response_set_json_error(res, 404, "HLS file not found or still being generated by FFmpeg");
    }
}
