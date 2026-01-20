#include <stdio.h>
#include <stdlib.h>

#include <string.h>
#include <sys/stat.h>
#include <fcntl.h>

#include "web/mongoose_server_static.h"
#include "web/mongoose_adapter.h"
#include "web/mongoose_server_auth.h"
#include "core/logger.h"
#include "core/config.h"
#include "video/streams.h"
#include "database/db_auth.h"

#ifdef USE_GO2RTC
#include "video/go2rtc/go2rtc_integration.h"
#endif

// Buffer size for URLs
#define URL_BUFFER_SIZE 2048

// Include Mongoose
#include "mongoose.h"

/**
 * @brief Check if client accepts gzip encoding
 * @param hm HTTP message containing headers
 * @return true if client accepts gzip, false otherwise
 */
static bool client_accepts_gzip(struct mg_http_message *hm) {
    struct mg_str *accept_encoding = mg_http_get_header(hm, "Accept-Encoding");
    if (accept_encoding != NULL && accept_encoding->len > 0) {
        // Check if "gzip" is in the Accept-Encoding header
        // Use memmem for searching within non-null-terminated buffer
        // (strstr requires null-terminated strings)
        const char *gzip_str = "gzip";
        size_t gzip_len = 4;
        if (accept_encoding->len >= gzip_len) {
            for (size_t i = 0; i <= accept_encoding->len - gzip_len; i++) {
                if (memcmp(accept_encoding->buf + i, gzip_str, gzip_len) == 0) {
                    return true;
                }
            }
        }
    }
    return false;
}

/**
 * @brief Try to serve a pre-compressed gzip file
 *
 * Checks if client accepts gzip and if a .gz version of the file exists.
 * If both conditions are met, serves the compressed file with appropriate headers.
 *
 * @param c Mongoose connection
 * @param hm HTTP message
 * @param file_path Path to the original (uncompressed) file
 * @param content_type MIME type for the Content-Type header
 * @param extra_headers Additional headers to include (can be NULL)
 * @return true if compressed file was served, false if caller should serve original
 */
static bool try_serve_gzip_file(struct mg_connection *c, struct mg_http_message *hm,
                                const char *file_path, const char *content_type,
                                const char *extra_headers) {
    // Check if client accepts gzip
    if (!client_accepts_gzip(hm)) {
        return false;
    }

    // Construct path to .gz file
    char gz_path[MAX_PATH_LENGTH * 2];
    snprintf(gz_path, sizeof(gz_path), "%s.gz", file_path);

    // Check if .gz file exists
    struct stat st;
    if (stat(gz_path, &st) != 0 || !S_ISREG(st.st_mode)) {
        return false;
    }

    log_debug("Serving gzip-compressed file: %s", gz_path);

    // Build mime_types to override the .gz extension with correct content type
    char mime_types[256];
    snprintf(mime_types, sizeof(mime_types), "gz=%s", content_type);

    // Build headers with Content-Encoding: gzip (but NOT Content-Type - let mime_types handle it)
    char headers[1024];
    snprintf(headers, sizeof(headers),
        "Content-Encoding: gzip\r\n"
        "Vary: Accept-Encoding\r\n"
        "%s",
        extra_headers ? extra_headers : "");

    // Serve the compressed file with correct MIME type
    struct mg_http_serve_opts opts = {
        .mime_types = mime_types,
        .extra_headers = headers,
    };
    mg_http_serve_file(c, hm, gz_path, &opts);
    return true;
}

/**
 * @brief Handle static file request
 */
void mongoose_server_handle_static_file(struct mg_connection *c, struct mg_http_message *hm, http_server_t *server) {
    // Note: No mutex locking needed as each connection is handled by a single thread
    // Extract URI
    char uri[MAX_PATH_LENGTH];
    size_t uri_len = hm->uri.len < sizeof(uri) - 1 ? hm->uri.len : sizeof(uri) - 1;
    memcpy(uri, hm->uri.buf, uri_len);
    uri[uri_len] = '\0';
    
    // Check if this is a static asset that should bypass authentication
    bool is_static_asset = false;
    if (strncmp(uri, "/js/", 4) == 0 || 
        strncmp(uri, "/css/", 5) == 0 || 
        strncmp(uri, "/img/", 5) == 0 || 
        strncmp(uri, "/fonts/", 7) == 0 ||
        strstr(uri, ".js") != NULL ||
        strstr(uri, ".css") != NULL ||
        strstr(uri, ".map") != NULL ||
        strstr(uri, ".ico") != NULL) {
        is_static_asset = true;
    }
    
    // Debug log to check URI
    log_info("Processing request for URI: %s, is_static_asset: %d", uri, is_static_asset);
    
    // Always allow login page without authentication
    if (strcmp(uri, "/login") == 0 || strcmp(uri, "/login.html") == 0) {
        log_info("Login page requested, bypassing authentication");
        // Continue processing without authentication check
    }
    // Skip authentication for static assets
    else if (is_static_asset) {
        log_debug("Bypassing authentication for static asset: %s", uri);
        // Continue processing without authentication check
    }
    // Authentication is already checked in the main event handler
    // No need to check it again here

    // Check if this is an API request
    if (strncmp(uri, "/api/", 5) == 0) {
        // API endpoint not found
        mg_http_reply(c, 404, "", "{\"error\": \"API Endpoint Not Found\"}\n");
        return;
    }
    
    // Special case for HLS streaming files
    if (strncmp(uri, "/hls/", 5) == 0) {
        // This is an HLS streaming request, serve it directly from the filesystem
        // Use the global configuration variable directly
        config_t *global_config = &g_config;
        
        // Check for authentication
        log_info("Processing HLS request: %s", uri);
        
        // Log all headers for debugging
        for (int i = 0; i < MG_MAX_HTTP_HEADERS; i++) {
            if (hm->headers[i].name.len == 0) break;
            log_info("HLS request header: %.*s: %.*s", 
                    (int)hm->headers[i].name.len, hm->headers[i].name.buf,
                    (int)hm->headers[i].value.len, hm->headers[i].value.buf);
        }
        
        // Check authentication using the common auth function
        if (server->config.auth_enabled && mongoose_server_basic_auth_check(hm, server) != 0) {
            log_info("Authentication required for HLS request but authentication failed");
            mg_printf(c, "HTTP/1.1 401 Unauthorized\r\n");
            mg_printf(c, "Content-Type: application/json\r\n");
            mg_printf(c, "Content-Length: 26\r\n");
            mg_printf(c, "Connection: close\r\n");
            mg_printf(c, "\r\n");
            mg_printf(c, "{\"error\": \"Unauthorized\"}\n");
            return;
        }
    
        // Extract stream name from URI
        // URI format: /hls/{stream_name}/{file}
        char stream_name[MAX_STREAM_NAME];
        char decoded_stream_name[MAX_STREAM_NAME];
        const char *stream_start = uri + 5; // Skip "/hls/"
        const char *file_part = strchr(stream_start, '/');

        if (!file_part) {
            mg_http_reply(c, 404, "", "{\"error\": \"Invalid HLS path\"}\n");
            return;
        }

        // Extract stream name
        size_t name_len = file_part - stream_start;
        if (name_len >= MAX_STREAM_NAME) {
            name_len = MAX_STREAM_NAME - 1;
        }
        strncpy(stream_name, stream_start, name_len);
        stream_name[name_len] = '\0';

        // URL decode the stream name
        mg_url_decode(stream_name, strlen(stream_name), decoded_stream_name, sizeof(decoded_stream_name), 0);

        // Use decoded_stream_name for file path construction
        #ifdef USE_GO2RTC
        char go2rtc_hls_url[URL_BUFFER_SIZE];
        if (go2rtc_integration_get_hls_url(decoded_stream_name, go2rtc_hls_url, sizeof(go2rtc_hls_url))) {
            // Stream is using go2rtc for HLS, but we'll serve the files directly
            log_info("Stream %s is using go2rtc for HLS, but serving files directly from filesystem", decoded_stream_name);
            // No redirection needed as go2rtc writes HLS segments to our HLS directory
        }
        #endif
        
        // Extract file name (everything after the stream name)
        const char *file_name = file_part + 1; // Skip "/"
        
        // Construct the full path to the HLS file
        char hls_file_path[MAX_PATH_LENGTH * 2]; // Double the buffer size to avoid truncation
        
        // Use storage_path_hls if specified, otherwise fall back to storage_path
        if (global_config->storage_path_hls[0] != '\0') {
            snprintf(hls_file_path, sizeof(hls_file_path), "%s/hls/%s/%s", 
                    global_config->storage_path_hls, decoded_stream_name, file_name);
            log_info("Using HLS-specific storage path: %s", global_config->storage_path_hls);
        } else {
            snprintf(hls_file_path, sizeof(hls_file_path), "%s/hls/%s/%s", 
                    global_config->storage_path, decoded_stream_name, file_name);
            log_info("Using default storage path for HLS: %s", global_config->storage_path);
        }
        
        log_info("Serving HLS file directly: %s", hls_file_path);
        
        // Check if file exists
        struct stat st;
        if (stat(hls_file_path, &st) == 0 && S_ISREG(st.st_mode)) {
            // Determine content type based on file extension
            const char *content_type = "application/octet-stream"; // Default
            if (strstr(file_name, ".m3u8")) {
                content_type = "application/vnd.apple.mpegurl";
            } else if (strstr(file_name, ".ts")) {
                content_type = "video/mp2t";
            } else if (strstr(file_name, ".m4s")) {
                content_type = "video/iso.segment";
            } else if (strstr(file_name, "init.mp4")) {
                content_type = "video/mp4";
            }
            
            // Serve the file with appropriate headers
            // Use a more efficient approach for HLS files to reduce overhead
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
            return;
        } else {
            // File doesn't exist - let the client know
            // We don't need to create dummy files since FFmpeg integration 
            // is responsible for creating the actual HLS files
            log_info("HLS file not found: %s (waiting for FFmpeg to create it)", hls_file_path);
            
            // Return a 404 with a message that indicates the file is being generated
            mg_http_reply(c, 404, "", "{\"error\": \"HLS file not found or still being generated by FFmpeg\"}\n");
            return;
        }
    }

    // Special handling for login page - redirect /login to /login.html
    if (strcmp(uri, "/login") == 0) {
        log_info("Redirecting /login to /login.html");
        mg_printf(c, "HTTP/1.1 302 Found\r\n");
        mg_printf(c, "Location: /login.html\r\n");
        mg_printf(c, "Connection: close\r\n");
        mg_printf(c, "Content-Length: 0\r\n");
        mg_printf(c, "\r\n");
        return;
    }

    // Special handling for logout - redirect to login page
    if (strcmp(uri, "/logout") == 0) {
        log_info("Redirecting /logout to /login.html");
        mg_printf(c, "HTTP/1.1 302 Found\r\n");
        mg_printf(c, "Location: /login.html?logout=1\r\n");
        mg_printf(c, "Set-Cookie: session=; Path=/; Max-Age=0\r\n");  // Clear session cookie
        mg_printf(c, "Connection: close\r\n");
        mg_printf(c, "Content-Length: 0\r\n");
        mg_printf(c, "\r\n");
        return;
    }

    // Special handling for root path or index.html
    if (strcmp(uri, "/") == 0 || strcmp(uri, "/index.html") == 0) {
        // Check if WebRTC is disabled in the configuration
        config_t *global_config = &g_config;
        if (global_config->webrtc_disabled) {
            // WebRTC is disabled, serve hls.html directly
            log_info("WebRTC is disabled, serving hls.html instead of index.html");

            // Use hls.html path instead
            char index_path[MAX_PATH_LENGTH * 2];
            snprintf(index_path, sizeof(index_path), "%s/hls.html", server->config.web_root);

            // Log the path we're trying to serve
            log_info("Serving hls.html: %s", index_path);

            // Try to serve gzip-compressed version first
            if (try_serve_gzip_file(c, hm, index_path, "text/html", "Connection: close\r\n")) {
                return;
            }

            // Fall back to uncompressed file
            struct mg_http_serve_opts opts = {
                .root_dir = server->config.web_root,
                .mime_types = "html=text/html",
                .extra_headers = "Connection: close\r\n"
            };

            mg_http_serve_file(c, hm, index_path, &opts);
            return;
        }

        // WebRTC is enabled, serve index.html as normal
        char index_path[MAX_PATH_LENGTH * 2];

        // Add debug logging to help diagnose the issue
        log_info("Root path requested, web_root: %s", server->config.web_root);

        // Use a direct path to index.html
        snprintf(index_path, sizeof(index_path), "%s/index.html", server->config.web_root);

        // Log the path we're trying to serve
        log_info("Serving root path with index file: %s", index_path);

        // Check if index.html exists
        struct stat st;
        if (stat(index_path, &st) == 0 && S_ISREG(st.st_mode)) {
            // Try to serve gzip-compressed version first
            if (try_serve_gzip_file(c, hm, index_path, "text/html", "Connection: close\r\n")) {
                return;
            }

            // Fall back to uncompressed file
            struct mg_http_serve_opts opts = {
                .root_dir = server->config.web_root,
                .mime_types = "html=text/html,htm=text/html,css=text/css,js=application/javascript,"
                             "json=application/json,jpg=image/jpeg,jpeg=image/jpeg,png=image/png,"
                             "gif=image/gif,svg=image/svg+xml,ico=image/x-icon,mp4=video/mp4,"
                             "webm=video/webm,ogg=video/ogg,mp3=audio/mpeg,wav=audio/wav,"
                             "txt=text/plain,xml=application/xml,pdf=application/pdf",
                .extra_headers = "Connection: close\r\n"
            };

            log_info("Serving index file for root path using mg_http_serve_file: %s", index_path);
            mg_http_serve_file(c, hm, index_path, &opts);
            return;
        } else {
            log_error("Index file not found for root path: %s", index_path);
            mg_http_reply(c, 404, "", "404 Not Found - Index file missing\n");
            return;
        }
    } else {
        // For non-root paths, construct file path
        char file_path[MAX_PATH_LENGTH * 2];
        snprintf(file_path, sizeof(file_path), "%s%s", server->config.web_root, uri);

        // Check if file exists (either uncompressed or gzip-only)
        struct stat st;
        char gz_path[MAX_PATH_LENGTH * 2];
        snprintf(gz_path, sizeof(gz_path), "%s.gz", file_path);
        bool file_exists = (stat(file_path, &st) == 0);
        bool gz_only = (!file_exists && stat(gz_path, &st) == 0 && S_ISREG(st.st_mode));

        if (file_exists || gz_only) {
            // Check if it's a directory
            if (S_ISDIR(st.st_mode)) {
                // Try to serve index.html as the index
                strncat(file_path, "index.html", sizeof(file_path) - strlen(file_path) - 1);
                if (stat(file_path, &st) != 0 || !S_ISREG(st.st_mode)) {
                    mg_http_reply(c, 403, "", "403 Forbidden\n");
                    return;
                }
            }

            // Serve the file without any locks
            // This is a critical optimization for static content

            // Common extra headers for static assets
            const char *common_extra_headers =
                "Cache-Control: no-store\r\n"
                "Connection: close\r\n"
                "Access-Control-Allow-Origin: *\r\n"
                "Access-Control-Allow-Methods: GET, OPTIONS\r\n"
                "Access-Control-Allow-Headers: Origin, Content-Type, Accept, Authorization\r\n";

            // Add special handling for JavaScript files to improve Firefox compatibility
            if (strstr(file_path, ".js") != NULL) {
                // Try to serve gzip-compressed version first
                if (try_serve_gzip_file(c, hm, file_path, "application/javascript", common_extra_headers)) {
                    return;
                }

                // Fall back to uncompressed file (only if it exists)
                if (gz_only) {
                    // Only .gz exists but client doesn't accept gzip - return 404
                    log_info("Client doesn't accept gzip and only .gz file exists: %s", file_path);
                    mg_http_reply(c, 404, "", "404 Not Found\n");
                    return;
                }

                const char js_headers[] =
                    "Content-Type: application/javascript\r\n"
                    "Cache-Control: no-store\r\n"
                    "Connection: close\r\n"
                    "Access-Control-Allow-Origin: *\r\n"
                    "Access-Control-Allow-Methods: GET, OPTIONS\r\n"
                    "Access-Control-Allow-Headers: Origin, Content-Type, Accept, Authorization\r\n";

                struct mg_http_serve_opts js_opts = {
                    .mime_types = "",
                    .extra_headers = js_headers,
                    .root_dir = server->config.web_root
                };

                log_debug("Serving JavaScript file: %s", file_path);
                mg_http_serve_file(c, hm, file_path, &js_opts);
            }
            // Add special handling for CSS files to improve Firefox compatibility
            else if (strstr(file_path, ".css") != NULL) {
                // Try to serve gzip-compressed version first
                if (try_serve_gzip_file(c, hm, file_path, "text/css", common_extra_headers)) {
                    return;
                }

                // Fall back to uncompressed file (only if it exists)
                if (gz_only) {
                    // Only .gz exists but client doesn't accept gzip - return 404
                    log_info("Client doesn't accept gzip and only .gz file exists: %s", file_path);
                    mg_http_reply(c, 404, "", "404 Not Found\n");
                    return;
                }

                const char css_headers[] =
                    "Content-Type: text/css\r\n"
                    "Cache-Control: no-store\r\n"
                    "Connection: close\r\n"
                    "Access-Control-Allow-Origin: *\r\n"
                    "Access-Control-Allow-Methods: GET, OPTIONS\r\n"
                    "Access-Control-Allow-Headers: Origin, Content-Type, Accept, Authorization\r\n";

                struct mg_http_serve_opts css_opts = {
                    .mime_types = "",
                    .extra_headers = css_headers,
                    .root_dir = server->config.web_root
                };

                log_debug("Serving CSS file: %s", file_path);
                mg_http_serve_file(c, hm, file_path, &css_opts);
            }
            // Handle HTML files with gzip support
            else if (strstr(file_path, ".html") != NULL) {
                // Try to serve gzip-compressed version first
                if (try_serve_gzip_file(c, hm, file_path, "text/html", common_extra_headers)) {
                    return;
                }

                // Fall back to uncompressed file
                struct mg_http_serve_opts html_opts = {
                    .mime_types = "html=text/html",
                    .root_dir = server->config.web_root,
                    .extra_headers = "Connection: close\r\n"
                };

                mg_http_serve_file(c, hm, file_path, &html_opts);
            }
            // Handle JSON files with gzip support
            else if (strstr(file_path, ".json") != NULL) {
                // Try to serve gzip-compressed version first
                if (try_serve_gzip_file(c, hm, file_path, "application/json", common_extra_headers)) {
                    return;
                }

                // Fall back to uncompressed file
                struct mg_http_serve_opts json_opts = {
                    .mime_types = "json=application/json",
                    .root_dir = server->config.web_root,
                    .extra_headers = "Connection: close\r\n"
                };

                mg_http_serve_file(c, hm, file_path, &json_opts);
            }
            // Handle SVG files with gzip support
            else if (strstr(file_path, ".svg") != NULL) {
                // Try to serve gzip-compressed version first
                if (try_serve_gzip_file(c, hm, file_path, "image/svg+xml", common_extra_headers)) {
                    return;
                }

                // Fall back to uncompressed file
                struct mg_http_serve_opts svg_opts = {
                    .mime_types = "svg=image/svg+xml",
                    .root_dir = server->config.web_root,
                    .extra_headers = "Connection: close\r\n"
                };

                mg_http_serve_file(c, hm, file_path, &svg_opts);
            } else {
                // For other files, use standard options (no gzip for binary files)
                struct mg_http_serve_opts std_opts = {
                    .mime_types = "html=text/html,htm=text/html,css=text/css,js=application/javascript,"
                                "json=application/json,jpg=image/jpeg,jpeg=image/jpeg,png=image/png,"
                                "gif=image/gif,svg=image/svg+xml,ico=image/x-icon,mp4=video/mp4,"
                                "webm=video/webm,ogg=video/ogg,mp3=audio/mpeg,wav=audio/wav,"
                                "txt=text/plain,xml=application/xml,pdf=application/pdf",
                    .root_dir = server->config.web_root,
                    .extra_headers = "Connection: close\r\n"
                };

                mg_http_serve_file(c, hm, file_path, &std_opts);
            }
            return;
        }
    }

    // File not found
    mg_http_reply(c, 404, "", "404 Not Found\n");
}

/**
 * @brief Set maximum connections
 */
int http_server_set_max_connections(http_server_handle_t server, int max_connections) {
    if (!server || max_connections <= 0) {
        return -1;
    }

    server->config.max_connections = max_connections;
    log_info("Maximum connections set to %d", max_connections);
    return 0;
}

/**
 * @brief Set connection timeout
 */
int http_server_set_connection_timeout(http_server_handle_t server, int timeout_seconds) {
    if (!server || timeout_seconds <= 0) {
        return -1;
    }

    server->config.connection_timeout = timeout_seconds;
    log_info("Connection timeout set to %d seconds", timeout_seconds);
    return 0;
}
