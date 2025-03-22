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

// Include Mongoose
#include "mongoose.h"

/**
 * @brief Handle static file request
 */
void mongoose_server_handle_static_file(struct mg_connection *c, struct mg_http_message *hm, http_server_t *server) {
    // Note: The mutex is already locked in the calling function (process_request_task or handle_http_request)
    // Extract URI
    char uri[MAX_PATH_LENGTH];
    size_t uri_len = hm->uri.len < sizeof(uri) - 1 ? hm->uri.len : sizeof(uri) - 1;
    memcpy(uri, hm->uri.buf, uri_len);
    uri[uri_len] = '\0';
    
    // Special case: handle "/%s" path which is causing redirection issues
    // This is likely coming from a client-side formatting issue
    if (strcmp(uri, "/%s") == 0) {
        log_info("Detected problematic '/%%s' path, redirecting to /live.html");
        
        // Get Authorization header if present
        struct mg_str *auth_header = mg_http_get_header(hm, "Authorization");
        
        mg_printf(c, "HTTP/1.1 302 Found\r\n");
        mg_printf(c, "Location: /live.html\r\n");
        
        // If there's an Authorization header, include it in the redirect
        if (auth_header != NULL) {
            mg_printf(c, "Authorization: %.*s\r\n", (int) auth_header->len, auth_header->buf);
        }
        
        mg_printf(c, "Content-Length: 0\r\n");
        mg_printf(c, "\r\n");
        return;
    }
    
    // Special case: handle root path "/" directly
    if (strcmp(uri, "/") == 0) {
        log_info("Handling root path '/', redirecting to /live.html");
        mg_printf(c, "HTTP/1.1 302 Found\r\n");
        mg_printf(c, "Location: /live.html\r\n");
        mg_printf(c, "Content-Length: 0\r\n");
        mg_printf(c, "\r\n");
        return;
    }
    
    // Check if this is a static asset that should bypass authentication
    bool is_static_asset = false;
    if (strncmp(uri, "/js/", 4) == 0 || 
        strncmp(uri, "/css/", 5) == 0 || 
        strncmp(uri, "/img/", 5) == 0 || 
        strncmp(uri, "/fonts/", 7) == 0 ||
        strstr(uri, ".js.map") != NULL ||
        strstr(uri, ".css.map") != NULL ||
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
        config_t *global_config = get_streaming_config();
        
    // Check for authentication headers for HLS requests
    log_info("Processing HLS request: %s", uri);
    
    // Log all headers for debugging
    for (int i = 0; i < MG_MAX_HTTP_HEADERS; i++) {
        if (hm->headers[i].name.len == 0) break;
        log_info("HLS request header: %.*s: %.*s", 
                (int)hm->headers[i].name.len, hm->headers[i].name.buf,
                (int)hm->headers[i].value.len, hm->headers[i].value.buf);
    }
    
    // Extract stream name from URI
    // URI format: /hls/{stream_name}/{file}
    char stream_name[MAX_STREAM_NAME];
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
        
        // Extract file name (everything after the stream name)
        const char *file_name = file_part + 1; // Skip "/"
        
        // Construct the full path to the HLS file
        char hls_file_path[MAX_PATH_LENGTH * 2]; // Double the buffer size to avoid truncation
        snprintf(hls_file_path, sizeof(hls_file_path), "%s/hls/%s/%s", 
                global_config->storage_path, stream_name, file_name);
        
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
            }
            
            // Serve the file with appropriate headers
            // Use a more efficient approach for HLS files to reduce overhead
            mg_http_serve_file(c, hm, hls_file_path, &(struct mg_http_serve_opts){
                .mime_types = "",
                .extra_headers = "Cache-Control: no-cache, no-store, must-revalidate\r\n"
                                "Pragma: no-cache\r\n"
                                "Expires: 0\r\n"
                                "Access-Control-Allow-Origin: *\r\n"
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

    // Construct file path
    char file_path[MAX_PATH_LENGTH * 2];
    snprintf(file_path, sizeof(file_path), "%s%s", server->config.web_root, uri);

    // If path ends with '/', append 'live.html' (which serves as our index)
    size_t path_len = strlen(file_path);
    if (path_len > 0 && file_path[path_len - 1] == '/') {
        strncat(file_path, "live.html", sizeof(file_path) - path_len - 1);
    }

    // Check if file exists
    struct stat st;
    if (stat(file_path, &st) == 0) {
        // Check if it's a directory
        if (S_ISDIR(st.st_mode)) {
            // Redirect to add trailing slash if needed
            if (uri[strlen(uri) - 1] != '/') {
                // Create a fixed redirect URL with the trailing slash
                char redirect_path[MAX_PATH_LENGTH * 2];
                snprintf(redirect_path, sizeof(redirect_path), "%s/", uri);
                
                // Use a complete HTTP response with fixed headers to avoid any string formatting issues
                mg_printf(c, "HTTP/1.1 301 Moved Permanently\r\n");
                mg_printf(c, "Location: %s\r\n", redirect_path);
                mg_printf(c, "Content-Length: 0\r\n");
                mg_printf(c, "\r\n");
                return;
            } else {
                // Try to serve live.html as the index
                strncat(file_path, "live.html", sizeof(file_path) - strlen(file_path) - 1);
                if (stat(file_path, &st) != 0 || !S_ISREG(st.st_mode)) {
                    mg_http_reply(c, 403, "", "403 Forbidden\n");
                    return;
                }
            }
        }

        // Serve the file
        struct mg_http_serve_opts opts = {
            .root_dir = server->config.web_root,
            .mime_types = "html=text/html,htm=text/html,css=text/css,js=application/javascript,"
                         "json=application/json,jpg=image/jpeg,jpeg=image/jpeg,png=image/png,"
                         "gif=image/gif,svg=image/svg+xml,ico=image/x-icon,mp4=video/mp4,"
                         "webm=video/webm,ogg=video/ogg,mp3=audio/mpeg,wav=audio/wav,"
                         "txt=text/plain,xml=application/xml,pdf=application/pdf"
        };
        mg_http_serve_file(c, hm, file_path, &opts);
        return;
    }

    // File doesn't exist - check SPA routes
    // List of known SPA routes
    const char *spa_routes[] = {
        "/",
        "/recordings",
        "/streams",
        "/settings",
        "/system",
        "/debug",
        NULL  // Terminator
    };

    // Check if the path matches a known SPA route
    bool is_spa_route = false;
    for (int i = 0; spa_routes[i] != NULL; i++) {
        if (strcmp(uri, spa_routes[i]) == 0) {
            is_spa_route = true;
            break;
        }
    }

    // If it's a known SPA route or path with assumed dynamic segments (/recordings/123)
    // Serve the index.html file
    if (is_spa_route ||
        strncmp(uri, "/recordings/", 12) == 0 ||
        strncmp(uri, "/streams/", 9) == 0) {

        // If authentication is enabled and this is not the login page, redirect to login
        if (server->config.auth_enabled && strcmp(uri, "/login") != 0 && strcmp(uri, "/login.html") != 0) {
            // Check if the request has valid authentication
            struct mg_str *auth_header = mg_http_get_header(hm, "Authorization");
            if (auth_header == NULL) {
                // No auth header, redirect to login page
                log_info("No authentication, redirecting to login page");
                mg_printf(c, "HTTP/1.1 302 Found\r\n");
                mg_printf(c, "Location: /login.html\r\n");
                mg_printf(c, "Content-Length: 0\r\n");
                mg_printf(c, "\r\n");
                return;
            }
            
            // Validate the authentication credentials
            const char *auth_str = auth_header->buf;
            if (auth_header->len > 6 && strncmp(auth_str, "Basic ", 6) == 0) {
                // Extract credentials
                char user[64] = {0}, pass[64] = {0};
                char decoded[128] = {0};
                
                // Skip "Basic " prefix and decode base64
                const char *b64 = auth_str + 6;
                size_t b64_len = auth_header->len - 6;
                mg_base64_decode(b64, b64_len, decoded, sizeof(decoded));
                
                // Find the colon separator
                char *colon = strchr(decoded, ':');
                if (colon != NULL) {
                    size_t user_len = colon - decoded;
                    if (user_len < sizeof(user)) {
                        strncpy(user, decoded, user_len);
                        user[user_len] = '\0';
                        
                        // Get password (everything after the colon)
                        strncpy(pass, colon + 1, sizeof(pass) - 1);
                        pass[sizeof(pass) - 1] = '\0';
                    }
                }
                
                if (user[0] != '\0') {
                    // Check credentials
                    if (strcmp(user, server->config.username) == 0 && 
                        strcmp(pass, server->config.password) == 0) {
                        // Authentication successful, continue
                        log_debug("Authentication successful for web page request");
                    } else {
                        // Authentication failed, redirect to login page
                        log_info("Authentication failed for web page request");
                        mg_printf(c, "HTTP/1.1 302 Found\r\n");
                        mg_printf(c, "Location: /login.html\r\n");
                        mg_printf(c, "Content-Length: 0\r\n");
                        mg_printf(c, "\r\n");
                        return;
                    }
                } else {
                    // Invalid format, redirect to login page
                    log_info("Invalid authentication format for web page request");
                    mg_printf(c, "HTTP/1.1 302 Found\r\n");
                    mg_printf(c, "Location: /login.html\r\n");
                    mg_printf(c, "Content-Length: 0\r\n");
                    mg_printf(c, "\r\n");
                    return;
                }
            } else {
                // Not Basic authentication, redirect to login page
                log_info("Not Basic authentication for web page request");
                mg_printf(c, "HTTP/1.1 302 Found\r\n");
                mg_printf(c, "Location: /login.html\r\n");
                mg_printf(c, "Content-Length: 0\r\n");
                mg_printf(c, "\r\n");
                return;
            }
        }

        // For SPA routes, directly serve live.html without redirection
        char index_path[MAX_PATH_LENGTH * 2];
        snprintf(index_path, sizeof(index_path), "%s/live.html", server->config.web_root);
        
        // Log the path we're trying to serve
        log_info("Serving SPA route %s with index file: %s", uri, index_path);

        // Check if live.html exists
        if (stat(index_path, &st) == 0 && S_ISREG(st.st_mode)) {
            // Serve live.html as the index
            struct mg_http_serve_opts opts = {
                .root_dir = server->config.web_root,
                .mime_types = "html=text/html"
            };
            
            // Use direct file serving instead of mg_http_serve_file
            // to avoid any potential redirection issues
            FILE *fp = fopen(index_path, "rb");
            if (fp != NULL) {
                // Get file size
                fseek(fp, 0, SEEK_END);
                long size = ftell(fp);
                fseek(fp, 0, SEEK_SET);
                
                // Read file content
                char *content = malloc(size);
                if (content) {
                    size_t bytes_read = fread(content, 1, size, fp);
                    if (bytes_read == size) {
                        // Send HTTP response with file content
                        mg_printf(c, "HTTP/1.1 200 OK\r\n");
                        mg_printf(c, "Content-Type: text/html\r\n");
                        mg_printf(c, "Content-Length: %ld\r\n", size);
                        mg_printf(c, "\r\n");
                        mg_send(c, content, size);
                    } else {
                        // Error reading file
                        log_error("Error reading file: %s (read %zu of %ld bytes)", index_path, bytes_read, size);
                        mg_http_reply(c, 500, "", "Internal Server Error - File read error\n");
                    }
                    free(content);
                } else {
                    mg_http_reply(c, 500, "", "Internal Server Error\n");
                }
                fclose(fp);
            } else {
                mg_http_reply(c, 500, "", "Internal Server Error\n");
            }
            return;
        } else {
            log_error("SPA index file not found: %s", index_path);
            mg_http_reply(c, 404, "", "404 Not Found - SPA index file missing\n");
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
