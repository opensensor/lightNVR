#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <ctype.h>
#include <stdbool.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <time.h>
#include <errno.h>

#include "web/api_handlers_streaming_hls.h"
#include "web/api_handlers_common.h"
#include "web/request_response.h"
#include "web/web_server.h"
#include "core/config.h"
#include "core/logger.h"
#include "video/stream_manager.h"
#include "video/streams.h"
#include "video/hls_writer.h"

/**
 * Handle HLS manifest request
 */
void handle_hls_manifest(const http_request_t *request, http_response_t *response) {
    // Extract stream name from URL
    // URL format: /api/streaming/{stream_name}/hls/index.m3u8

    const char *path = request->path;
    const char *streams_pos = strstr(path, "/streaming/");

    if (!streams_pos) {
        create_stream_error_response(response, 400, "Invalid request path");
        return;
    }

    const char *stream_name_start = streams_pos + 11;  // Skip "/streaming/"
    const char *hls_pos = strstr(stream_name_start, "/hls/");

    if (!hls_pos) {
        create_stream_error_response(response, 400, "Invalid HLS request path");
        return;
    }

    // Extract stream name
    char stream_name[MAX_STREAM_NAME];
    size_t name_len = hls_pos - stream_name_start;

    if (name_len >= MAX_STREAM_NAME) {
        create_stream_error_response(response, 400, "Stream name too long");
        return;
    }

    memcpy(stream_name, stream_name_start, name_len);
    stream_name[name_len] = '\0';

    // URL decode the stream name
    char decoded_stream[MAX_STREAM_NAME];
    url_decode(stream_name, decoded_stream, sizeof(decoded_stream));
    strncpy(stream_name, decoded_stream, MAX_STREAM_NAME - 1);
    stream_name[MAX_STREAM_NAME - 1] = '\0';

    // Check if stream exists
    stream_handle_t stream = get_stream_by_name(stream_name);
    if (!stream) {
        create_stream_error_response(response, 404, "Stream not found");
        return;
    }
    
    // Get stream configuration to check if streaming is enabled
    stream_config_t config;
    if (get_stream_config(stream, &config) != 0) {
        create_stream_error_response(response, 500, "Failed to get stream configuration");
        return;
    }
    
    // Check if streaming is enabled for this stream
    // If streaming_enabled is false, don't start streaming
    if (config.streaming_enabled == false) {
        // Streaming is disabled for this stream
        log_info("Streaming is disabled for stream %s", stream_name);
        create_stream_error_response(response, 403, "Streaming is disabled for this stream");
        return;
    }
    
    // CRITICAL FIX: Use a per-stream mutex to prevent concurrent operations on the same stream
    // This allows different streams to be processed concurrently
    static pthread_mutex_t global_stream_mutex = PTHREAD_MUTEX_INITIALIZER;
    
    // First, get a mutex for this specific stream
    pthread_mutex_t *stream_mutex = NULL;
    
    // Lock the global mutex to safely access the stream-specific mutex
    pthread_mutex_lock(&global_stream_mutex);
    
    // Get or create a mutex for this specific stream
    static struct {
        char stream_name[MAX_STREAM_NAME];
        pthread_mutex_t mutex;
        bool initialized;
        time_t last_access;
    } stream_mutexes[MAX_STREAMS];
    
    // Find existing mutex for this stream
    bool found_mutex = false;
    for (int i = 0; i < MAX_STREAMS; i++) {
        if (stream_mutexes[i].initialized && strcmp(stream_mutexes[i].stream_name, stream_name) == 0) {
            stream_mutex = &stream_mutexes[i].mutex;
            stream_mutexes[i].last_access = time(NULL); // Update last access time
            found_mutex = true;
            break;
        }
    }
    
    // If not found, create a new mutex
    if (!found_mutex) {
        // First try to find an expired mutex slot (not accessed in the last hour)
        time_t current_time = time(NULL);
        for (int i = 0; i < MAX_STREAMS; i++) {
            if (stream_mutexes[i].initialized && 
                (current_time - stream_mutexes[i].last_access) > 3600) {
                // Clean up the old mutex
                pthread_mutex_destroy(&stream_mutexes[i].mutex);
                
                // Initialize the new mutex
                if (pthread_mutex_init(&stream_mutexes[i].mutex, NULL) != 0) {
                    log_error("Failed to initialize mutex for stream %s", stream_name);
                    pthread_mutex_unlock(&global_stream_mutex);
                    create_stream_error_response(response, 500, "Failed to initialize mutex");
                    return;
                }
                
                // Set the stream name
                strncpy(stream_mutexes[i].stream_name, stream_name, MAX_STREAM_NAME - 1);
                stream_mutexes[i].stream_name[MAX_STREAM_NAME - 1] = '\0';
                stream_mutexes[i].initialized = true;
                stream_mutexes[i].last_access = current_time;
                
                stream_mutex = &stream_mutexes[i].mutex;
                found_mutex = true;
                log_debug("Reused expired mutex slot for stream %s", stream_name);
                break;
            }
        }
        
        // If no expired slot found, try to find an uninitialized slot
        if (!found_mutex) {
            for (int i = 0; i < MAX_STREAMS; i++) {
                if (!stream_mutexes[i].initialized) {
                    // Initialize the mutex
                    if (pthread_mutex_init(&stream_mutexes[i].mutex, NULL) != 0) {
                        log_error("Failed to initialize mutex for stream %s", stream_name);
                        pthread_mutex_unlock(&global_stream_mutex);
                        create_stream_error_response(response, 500, "Failed to initialize mutex");
                        return;
                    }
                    
                    // Set the stream name
                    strncpy(stream_mutexes[i].stream_name, stream_name, MAX_STREAM_NAME - 1);
                    stream_mutexes[i].stream_name[MAX_STREAM_NAME - 1] = '\0';
                    stream_mutexes[i].initialized = true;
                    stream_mutexes[i].last_access = time(NULL);
                    
                    stream_mutex = &stream_mutexes[i].mutex;
                    found_mutex = true;
                    log_debug("Created mutex for stream %s", stream_name);
                    break;
                }
            }
        }
    }
    
    // Unlock the global mutex
    pthread_mutex_unlock(&global_stream_mutex);
    
    // If we couldn't find or create a mutex, return an error
    if (!found_mutex) {
        log_error("Failed to get or create mutex for stream %s", stream_name);
        create_stream_error_response(response, 500, "Failed to get or create mutex");
        return;
    }
    
    // Lock the stream-specific mutex
    pthread_mutex_lock(stream_mutex);
    
    // Start the stream if not already running
    if (get_stream_status(stream) != STREAM_STATUS_RUNNING) {
        log_info("Starting stream %s for HLS streaming", stream_name);
        
        // CRITICAL FIX: Reset timestamp tracker before starting the stream
        reset_timestamp_tracker(stream_name);
        
        // Set UDP flag for timestamp tracker based on stream URL
        if (strncmp(config.url, "udp://", 6) == 0 || strncmp(config.url, "rtp://", 6) == 0) {
            log_info("Setting UDP flag for stream %s based on URL: %s", stream_name, config.url);
            set_timestamp_tracker_udp_flag(stream_name, true);
        }
        
        if (start_stream(stream) != 0) {
            log_error("Failed to start stream %s", stream_name);
            pthread_mutex_unlock(stream_mutex);
            create_stream_error_response(response, 500, "Failed to start stream");
            return;
        }
        
        // Wait a moment for the stream to initialize
        usleep(500000);  // 500ms
    }
    
    // Get the stream processor
    stream_processor_t processor = get_stream_processor(stream);
    if (!processor) {
        log_error("Failed to get stream processor for %s", stream_name);
        
        // Try to restart the stream with proper cleanup
        log_info("Attempting to restart stream %s", stream_name);
        
        // Reset timestamp tracker before stopping the stream
        reset_timestamp_tracker(stream_name);
        
        // Stop the stream with proper cleanup
        stop_stream(stream);
        
        // Wait a moment for the stream to stop
        usleep(200000);  // 200ms
        
        if (start_stream(stream) != 0) {
            log_error("Failed to restart stream %s", stream_name);
            pthread_mutex_unlock(stream_mutex);
            create_stream_error_response(response, 500, "Failed to restart stream");
            return;
        }
        
        // Wait a moment for the stream to start
        usleep(800000);  // 800ms
        
        // Try to get the processor again
        processor = get_stream_processor(stream);
        if (!processor) {
            log_error("Still failed to get stream processor for %s after restart", stream_name);
            pthread_mutex_unlock(stream_mutex);
            create_stream_error_response(response, 500, "Failed to get stream processor after restart");
            return;
        }
    }
    
    // Get the HLS writer from the processor
    hls_writer_t *hls_writer = stream_processor_get_hls_writer(processor);
    
    // If HLS writer is not found in the processor, we need to add it
    if (!hls_writer) {
        log_info("HLS writer not found in stream processor for %s, adding HLS output", stream_name);
        
        // First stop the processor with proper cleanup
        if (stream_processor_stop(processor) != 0) {
            log_error("Failed to stop stream processor for %s", stream_name);
            pthread_mutex_unlock(stream_mutex);
            create_stream_error_response(response, 500, "Failed to stop stream processor");
            return;
        }
        
        // Wait a moment for the processor to fully stop
        usleep(200000);  // 200ms - increased for better cleanup
        
        // Add HLS output to the processor
        output_config_t output_config;
        memset(&output_config, 0, sizeof(output_config));
        output_config.type = OUTPUT_TYPE_HLS;
        
        // Get HLS output path from global config
        config_t *global_config = get_streaming_config();
        snprintf(output_config.hls.output_path, MAX_PATH_LENGTH, "%s/hls/%s",
                global_config->storage_path, stream_name);
        
        // Use segment duration from stream config or default to 4 seconds
        output_config.hls.segment_duration = config.segment_duration > 0 ?
                                           config.segment_duration : 4;
        
        // Create HLS directory if it doesn't exist
        char dir_path[MAX_PATH_LENGTH];
        snprintf(dir_path, sizeof(dir_path), "%s/hls/%s", global_config->storage_path, stream_name);
        
        struct stat st;
        if (stat(dir_path, &st) != 0 || !S_ISDIR(st.st_mode)) {
            // Directory doesn't exist, create it
            char mkdir_cmd[MAX_PATH_LENGTH * 2];
            snprintf(mkdir_cmd, sizeof(mkdir_cmd), "mkdir -p %s", dir_path);
            int ret = system(mkdir_cmd);
            if (ret != 0) {
                log_error("Failed to create HLS directory: %s (return code: %d)", dir_path, ret);
                pthread_mutex_unlock(stream_mutex);
                create_stream_error_response(response, 500, "Failed to create HLS directory");
                return;
            }
            
            // Set full permissions to ensure FFmpeg can write files
            snprintf(mkdir_cmd, sizeof(mkdir_cmd), "chmod -R 777 %s", dir_path);
            system(mkdir_cmd);
            
            log_info("Created HLS directory: %s", dir_path);
        }
        
        // Create a minimal valid manifest file before adding output
        char manifest_path[MAX_PATH_LENGTH];
        snprintf(manifest_path, MAX_PATH_LENGTH, "%s/index.m3u8", dir_path);
        FILE *manifest_init = fopen(manifest_path, "w");
        if (manifest_init) {
            // Write a minimal valid HLS manifest
            fprintf(manifest_init, "#EXTM3U\n");
            fprintf(manifest_init, "#EXT-X-VERSION:3\n");
            fprintf(manifest_init, "#EXT-X-TARGETDURATION:%d\n", output_config.hls.segment_duration);
            fprintf(manifest_init, "#EXT-X-MEDIA-SEQUENCE:0\n");
            fclose(manifest_init);
            
            log_info("Created initial valid HLS manifest file: %s", manifest_path);
        }
        
        if (stream_processor_add_output(processor, &output_config) != 0) {
            log_error("Failed to add HLS output to stream processor for %s", stream_name);
            pthread_mutex_unlock(stream_mutex);
            create_stream_error_response(response, 500, "Failed to add HLS output to stream processor");
            return;
        }
        
        // Restart the processor
        if (stream_processor_start(processor) != 0) {
            log_error("Failed to restart stream processor for %s", stream_name);
            pthread_mutex_unlock(stream_mutex);
            create_stream_error_response(response, 500, "Failed to restart stream processor");
            return;
        }
        
        // Wait a moment for the HLS stream to initialize
        usleep(800000);  // 800ms - increased for better initialization
        
        // Get the HLS writer again
        hls_writer = stream_processor_get_hls_writer(processor);
        if (!hls_writer) {
            log_error("Still failed to get HLS writer for %s after adding output", stream_name);
            pthread_mutex_unlock(stream_mutex);
            create_stream_error_response(response, 500, "Failed to get HLS writer after adding output");
            return;
        }
        
        log_info("Successfully added HLS output to stream processor for %s", stream_name);
    }
    
    // We can release the mutex now that we've set up the stream
    pthread_mutex_unlock(stream_mutex);

    // Get the manifest file path
    config_t *global_config = get_streaming_config();
    
    // Log the storage path for debugging
    log_info("API looking for HLS manifest in storage path: %s", global_config->storage_path);
    
    char manifest_path[MAX_PATH_LENGTH];
    // First try the standard path
    snprintf(manifest_path, MAX_PATH_LENGTH, "%s/hls/%s/index.m3u8",
             global_config->storage_path, stream_name);
    
    // Check if the file exists at this path
    if (access(manifest_path, F_OK) != 0) {
        // If not, try the recordings/hls path
        log_info("HLS manifest not found at %s, trying recordings/hls path", manifest_path);
        snprintf(manifest_path, MAX_PATH_LENGTH, "%s/recordings/hls/%s/index.m3u8",
                 global_config->storage_path, stream_name);
        log_info("Checking alternative path: %s", manifest_path);
    }
    
    // Log the full manifest path
    log_info("Full manifest path: %s", manifest_path);

    // CRITICAL FIX: Improved manifest file waiting logic
    // Wait for the manifest file to be created with a longer timeout for low-powered devices
    // Try up to 120 times with 100ms between attempts (12 seconds total)
    bool manifest_exists = false;
    for (int i = 0; i < 120; i++) {
        // Check for the final manifest file
        if (access(manifest_path, F_OK) == 0) {
            // Verify the manifest file is valid (contains EXTM3U)
            FILE *manifest_check = fopen(manifest_path, "r");
            if (manifest_check) {
                char buffer[64]; // Increased buffer size for better validation
                size_t read_size = fread(buffer, 1, sizeof(buffer) - 1, manifest_check);
                buffer[read_size] = '\0';
                fclose(manifest_check);
                
                if (strstr(buffer, "#EXTM3U") != NULL) {
                    manifest_exists = true;
                    log_info("Found valid manifest file with EXTM3U header at %s", manifest_path);
                    break;
                } else {
                    log_warn("Found manifest file but missing EXTM3U header at %s", manifest_path);
                    
                    // If the file exists but is invalid, try to fix it
                    if (read_size == 0 || read_size < 10) { // Empty or too small
                        log_info("Attempting to fix invalid manifest file: %s", manifest_path);
                        FILE *manifest_fix = fopen(manifest_path, "w");
                        if (manifest_fix) {
                            // Write a minimal valid HLS manifest
                            fprintf(manifest_fix, "#EXTM3U\n");
                            fprintf(manifest_fix, "#EXT-X-VERSION:3\n");
                            fprintf(manifest_fix, "#EXT-X-TARGETDURATION:%d\n", config.segment_duration > 0 ? config.segment_duration : 4);
                            fprintf(manifest_fix, "#EXT-X-MEDIA-SEQUENCE:0\n");
                            fclose(manifest_fix);
                            
                            manifest_exists = true;
                            log_info("Fixed invalid manifest file: %s", manifest_path);
                            break;
                        }
                    }
                    // Continue waiting for a valid file
                }
            }
        }

        // Also check for the temporary file
        char temp_manifest_path[MAX_PATH_LENGTH];
        snprintf(temp_manifest_path, MAX_PATH_LENGTH, "%s.tmp", manifest_path);
        if (access(temp_manifest_path, F_OK) == 0) {
            log_debug("Found temporary manifest file, waiting for it to be finalized");
            // Continue waiting for the final file
        }

        // Log less frequently to reduce log spam
        if (i % 10 == 0) {
            log_debug("Waiting for manifest file to be created (attempt %d/120)", i+1);
        }
        usleep(100000);  // 100ms
    }

    if (!manifest_exists) {
        log_error("Manifest file was not created in time: %s", manifest_path);
        
        // Check if the directory exists
        char dir_path[MAX_PATH_LENGTH];
        snprintf(dir_path, sizeof(dir_path), "%s/hls/%s", global_config->storage_path, stream_name);
        
        struct stat st;
        if (stat(dir_path, &st) != 0 || !S_ISDIR(st.st_mode)) {
            log_error("HLS directory does not exist: %s", dir_path);
            
            // Try to create it
            char mkdir_cmd[MAX_PATH_LENGTH * 2];
            snprintf(mkdir_cmd, sizeof(mkdir_cmd), "mkdir -p %s && chmod -R 777 %s", 
                    dir_path, dir_path);
            system(mkdir_cmd);
            
            log_info("Created HLS directory: %s", dir_path);
        }
        
        // CRITICAL FIX: Create a minimal valid manifest file before restarting
        FILE *manifest_init = fopen(manifest_path, "w");
        if (manifest_init) {
            // Write a minimal valid HLS manifest
            fprintf(manifest_init, "#EXTM3U\n");
            fprintf(manifest_init, "#EXT-X-VERSION:3\n");
            fprintf(manifest_init, "#EXT-X-TARGETDURATION:%d\n", config.segment_duration > 0 ? config.segment_duration : 4);
            fprintf(manifest_init, "#EXT-X-MEDIA-SEQUENCE:0\n");
            fclose(manifest_init);
            
            log_info("Created initial valid HLS manifest file: %s", manifest_path);
            manifest_exists = true;
        } else {
            // Try to restart the stream with proper cleanup
            log_info("Restarting stream for %s", stream_name);
            
            // CRITICAL FIX: Use mutex for stream operations
            pthread_mutex_lock(stream_mutex);
            
            // CRITICAL FIX: Reset timestamp tracker before stopping the stream
            reset_timestamp_tracker(stream_name);
            
            stop_stream(stream);
            
            // Wait a moment for the stream to stop
            usleep(200000);  // 200ms - increased from 100ms for better cleanup
            
            if (start_stream(stream) != 0) {
                log_error("Failed to restart stream %s", stream_name);
                pthread_mutex_unlock(stream_mutex);
                create_stream_error_response(response, 500, "Failed to restart stream");
                return;
            }
            
            pthread_mutex_unlock(stream_mutex);
            
            // Wait a bit longer for the manifest file to be created
            for (int i = 0; i < 40; i++) {  // Increased from 30 to 40 attempts
                if (access(manifest_path, F_OK) == 0) {
                    // Verify the manifest file is valid (contains EXTM3U)
                    FILE *manifest_check = fopen(manifest_path, "r");
                    if (manifest_check) {
                        char buffer[16];
                        size_t read_size = fread(buffer, 1, sizeof(buffer) - 1, manifest_check);
                        buffer[read_size] = '\0';
                        fclose(manifest_check);
                        
                        if (strstr(buffer, "#EXTM3U") != NULL) {
                            manifest_exists = true;
                            log_info("Found valid manifest file with EXTM3U header after restart");
                            break;
                        }
                    }
                }
                usleep(100000);  // 100ms
            }
            
            if (!manifest_exists) {
                log_error("Manifest file still not created after restart: %s", manifest_path);
                
                // CRITICAL FIX: Create a minimal valid manifest file as a last resort
                FILE *manifest_last_resort = fopen(manifest_path, "w");
                if (manifest_last_resort) {
                    // Write a minimal valid HLS manifest
                    fprintf(manifest_last_resort, "#EXTM3U\n");
                    fprintf(manifest_last_resort, "#EXT-X-VERSION:3\n");
                    fprintf(manifest_last_resort, "#EXT-X-TARGETDURATION:%d\n", config.segment_duration > 0 ? config.segment_duration : 4);
                    fprintf(manifest_last_resort, "#EXT-X-MEDIA-SEQUENCE:0\n");
                    fclose(manifest_last_resort);
                    
                    log_info("Created last-resort valid HLS manifest file: %s", manifest_path);
                    manifest_exists = true;
                } else {
                    create_stream_error_response(response, 404, "Manifest file not found, please try again");
                    return;
                }
            }
            
            log_info("Successfully restarted stream for %s", stream_name);
        }
    }

    // Read the manifest file
    FILE *fp = fopen(manifest_path, "r");
    if (!fp) {
        create_stream_error_response(response, 500, "Failed to open manifest file");
        return;
    }

    // Get file size
    fseek(fp, 0, SEEK_END);
    long file_size = ftell(fp);
    fseek(fp, 0, SEEK_SET);

    // Read file content
    char *content = malloc(file_size + 1);
    if (!content) {
        fclose(fp);
        create_stream_error_response(response, 500, "Memory allocation failed");
        return;
    }

    size_t bytes_read = fread(content, 1, file_size, fp);
    if (bytes_read != (size_t)file_size) {
        free(content);
        fclose(fp);
        create_stream_error_response(response, 500, "Failed to read manifest file");
        return;
    }

    content[file_size] = '\0';
    fclose(fp);
    
    // Check if the manifest file is empty or doesn't contain the EXTM3U delimiter
    if (file_size == 0 || strstr(content, "#EXTM3U") == NULL) {
        log_error("Manifest file is empty or missing EXTM3U delimiter: %s", manifest_path);
        
        // Try to restart the stream
        log_info("Restarting stream for %s due to invalid manifest", stream_name);
        
        // CRITICAL FIX: Use mutex for stream operations
        pthread_mutex_lock(stream_mutex);
        
        stop_stream(stream);
        
        // Wait a moment for the stream to stop
        usleep(100000);  // 100ms
        
        if (start_stream(stream) != 0) {
            log_error("Failed to restart stream %s", stream_name);
            pthread_mutex_unlock(stream_mutex);
            free(content);
            create_stream_error_response(response, 500, "Failed to restart stream");
            return;
        }
        
        pthread_mutex_unlock(stream_mutex);
        
        // Wait for the manifest file to be properly created
        bool manifest_valid = false;
        for (int i = 0; i < 30; i++) {
            // Re-read the manifest file
            FILE *fp_retry = fopen(manifest_path, "r");
            if (fp_retry) {
                fseek(fp_retry, 0, SEEK_END);
                long new_size = ftell(fp_retry);
                fseek(fp_retry, 0, SEEK_SET);
                
                if (new_size > 0) {
                    // Free old content and allocate new buffer
                    free(content);
                    content = malloc(new_size + 1);
                    if (!content) {
                        fclose(fp_retry);
                        create_stream_error_response(response, 500, "Memory allocation failed");
                        return;
                    }
                    
                    size_t new_bytes_read = fread(content, 1, new_size, fp_retry);
                    if (new_bytes_read == (size_t)new_size) {
                        content[new_size] = '\0';
                        if (strstr(content, "#EXTM3U") != NULL) {
                            file_size = new_size;
                            manifest_valid = true;
                            log_info("Successfully regenerated valid manifest file for %s", stream_name);
                            fclose(fp_retry);
                            break;
                        }
                    }
                }
                fclose(fp_retry);
            }
            
            log_debug("Waiting for valid manifest file (attempt %d/30)", i+1);
            usleep(100000);  // 100ms
        }
        
        if (!manifest_valid) {
            log_error("Failed to generate valid manifest file for %s", stream_name);
            free(content);
            create_stream_error_response(response, 500, "Failed to generate valid manifest file");
            return;
        }
    }

    // Create response
    response->status_code = 200;
    strncpy(response->content_type, "application/vnd.apple.mpegurl", sizeof(response->content_type) - 1);
    response->content_type[sizeof(response->content_type) - 1] = '\0';
    
    // Add strict cache control headers to prevent caching of HLS manifests
    set_response_header(response, "Cache-Control", "no-cache, no-store, must-revalidate, max-age=0");
    set_response_header(response, "Pragma", "no-cache");
    set_response_header(response, "Expires", "0");
    
    // Add timestamp header to help client identify the freshness of the manifest
    char timestamp_str[32];
    time_t now = time(NULL);
    snprintf(timestamp_str, sizeof(timestamp_str), "%ld", now);
    set_response_header(response, "X-Timestamp", timestamp_str);
    
    response->body = content;
    response->body_length = file_size;
}

/**
 * Handle request for HLS segment
 */
void handle_hls_segment(const http_request_t *request, http_response_t *response) {
    // Extract stream name and segment from URL
    // URL format: /api/streaming/{stream_name}/hls/segment_{number}.ts or /api/streaming/{stream_name}/hls/index{number}.ts

    const char *path = request->path;
    const char *streams_pos = strstr(path, "/streaming/");

    if (!streams_pos) {
        create_stream_error_response(response, 400, "Invalid request path");
        return;
    }

    const char *stream_name_start = streams_pos + 11;  // Skip "/streaming/"
    const char *hls_pos = strstr(stream_name_start, "/hls/");

    if (!hls_pos) {
        create_stream_error_response(response, 400, "Invalid HLS request path");
        return;
    }

    // Extract stream name
    char stream_name[MAX_STREAM_NAME];
    size_t name_len = hls_pos - stream_name_start;

    if (name_len >= MAX_STREAM_NAME) {
        create_stream_error_response(response, 400, "Stream name too long");
        return;
    }

    memcpy(stream_name, stream_name_start, name_len);
    stream_name[name_len] = '\0';

    // URL decode the stream name
    char decoded_stream[MAX_STREAM_NAME];
    url_decode(stream_name, decoded_stream, sizeof(decoded_stream));
    strncpy(stream_name, decoded_stream, MAX_STREAM_NAME - 1);
    stream_name[MAX_STREAM_NAME - 1] = '\0';

    // Check if stream exists
    stream_handle_t stream = get_stream_by_name(stream_name);
    if (!stream) {
        create_stream_error_response(response, 404, "Stream not found");
        return;
    }
    
    // Get stream configuration to check if streaming is enabled
    stream_config_t config;
    if (get_stream_config(stream, &config) != 0) {
        create_stream_error_response(response, 500, "Failed to get stream configuration");
        return;
    }
    
    // Check if streaming is enabled for this stream
    if (config.streaming_enabled == false) {
        // Streaming is disabled for this stream
        log_info("Streaming is disabled for stream %s", stream_name);
        create_stream_error_response(response, 403, "Streaming is disabled for this stream");
        return;
    }
    
    // Check if the stream is running
    if (get_stream_status(stream) != STREAM_STATUS_RUNNING) {
        log_warn("Stream %s is not running, attempting to start it", stream_name);
        if (start_stream(stream) != 0) {
            create_stream_error_response(response, 500, "Failed to start stream");
            return;
        }
    }
    
    // Check if HLS streaming is active for this stream
    // Try to get the stream processor and check if it has an HLS writer
    stream_processor_t processor = get_stream_processor(stream);
    if (!processor) {
        log_error("Failed to get stream processor for %s", stream_name);
        
        // Try to restart the stream
        log_info("Attempting to restart stream %s", stream_name);
        stop_stream(stream);
        
        // Wait a moment for the stream to stop
        usleep(100000);  // 100ms
        
        if (start_stream(stream) != 0) {
            log_error("Failed to restart stream %s", stream_name);
            create_stream_error_response(response, 500, "Failed to restart stream");
            return;
        }
        
        // Wait a moment for the stream to start
        usleep(500000);  // 500ms
        
        // Try to get the processor again
        processor = get_stream_processor(stream);
        if (!processor) {
            log_error("Still failed to get stream processor for %s after restart", stream_name);
            create_stream_error_response(response, 500, "Failed to get stream processor after restart");
            return;
        }
    }
    
    // Get the HLS writer from the processor
    hls_writer_t *hls_writer = stream_processor_get_hls_writer(processor);
    
    // If HLS writer is not found in the processor, we need to add it
    if (!hls_writer) {
        log_info("HLS writer not found in stream processor for %s, adding HLS output", stream_name);
        
        // First stop the processor
        if (stream_processor_stop(processor) != 0) {
            log_error("Failed to stop stream processor for %s", stream_name);
            create_stream_error_response(response, 500, "Failed to stop stream processor");
            return;
        }
        
        // Add HLS output to the processor
        output_config_t output_config;
        memset(&output_config, 0, sizeof(output_config));
        output_config.type = OUTPUT_TYPE_HLS;
        
        // Get HLS output path from global config
        config_t *global_config = get_streaming_config();
        snprintf(output_config.hls.output_path, MAX_PATH_LENGTH, "%s/hls/%s",
                global_config->storage_path, stream_name);
        
        // Use segment duration from stream config or default to 4 seconds
        output_config.hls.segment_duration = config.segment_duration > 0 ?
                                           config.segment_duration : 4;
        
        // Create HLS directory if it doesn't exist
        char dir_cmd[MAX_PATH_LENGTH * 2];
        snprintf(dir_cmd, sizeof(dir_cmd), "mkdir -p %s", output_config.hls.output_path);
        int ret = system(dir_cmd);
        if (ret != 0) {
            log_error("Failed to create HLS directory: %s (return code: %d)", 
                     output_config.hls.output_path, ret);
            create_stream_error_response(response, 500, "Failed to create HLS directory");
            return;
        }
        
        // Set full permissions to ensure FFmpeg can write files
        snprintf(dir_cmd, sizeof(dir_cmd), "chmod -R 777 %s", output_config.hls.output_path);
        system(dir_cmd);
        
        if (stream_processor_add_output(processor, &output_config) != 0) {
            log_error("Failed to add HLS output to stream processor for %s", stream_name);
            create_stream_error_response(response, 500, "Failed to add HLS output to stream processor");
            return;
        }
        
        // Restart the processor
        if (stream_processor_start(processor) != 0) {
            log_error("Failed to restart stream processor for %s", stream_name);
            create_stream_error_response(response, 500, "Failed to restart stream processor");
            return;
        }
        
        // Wait a moment for the HLS stream to initialize
        usleep(500000);  // 500ms
        
        // Get the HLS writer again
        hls_writer = stream_processor_get_hls_writer(processor);
        if (!hls_writer) {
            log_error("Still failed to get HLS writer for %s after adding output", stream_name);
            create_stream_error_response(response, 500, "Failed to get HLS writer after adding output");
            return;
        }
        
        log_info("Successfully added HLS output to stream processor for %s", stream_name);
    }

    // Extract segment filename
    const char *segment_filename = hls_pos + 5;  // Skip "/hls/"

    log_info("Segment requested: %s", segment_filename);

    // Get the segment file path
    config_t *global_config = get_streaming_config();
    char segment_path[MAX_PATH_LENGTH];
    
    // First try the standard path
    snprintf(segment_path, MAX_PATH_LENGTH, "%s/hls/%s/%s",
             global_config->storage_path, stream_name, segment_filename);

    log_info("Looking for segment at path: %s", segment_path);
    
    // Check if the file exists at this path
    if (access(segment_path, F_OK) != 0) {
        // If not, try the recordings/hls path
        log_info("Segment not found at %s, trying recordings/hls path", segment_path);
        snprintf(segment_path, MAX_PATH_LENGTH, "%s/recordings/hls/%s/%s",
                 global_config->storage_path, stream_name, segment_filename);
        log_info("Checking alternative path: %s", segment_path);
    }

    // Check if segment file exists
    if (access(segment_path, F_OK) != 0) {
        log_debug("Segment file not found on first attempt: %s (%s)", segment_path, strerror(errno));

        // Wait for it to be created with a longer timeout for low-powered devices
        bool segment_exists = false;
        for (int i = 0; i < 40; i++) {  // Try for 4 seconds total (increased from 2 seconds)
            if (access(segment_path, F_OK) == 0) {
                log_info("Segment file found after waiting: %s (attempt %d)", segment_path, i+1);
                segment_exists = true;
                break;
            }

            log_debug("Waiting for segment file to be created: %s (attempt %d/40)", segment_path, i+1);
            usleep(100000);  // 100ms
        }

        if (!segment_exists) {
            log_error("Segment file not found after waiting: %s", segment_path);
            create_stream_error_response(response, 404, "Segment file not found");
            return;
        }
    }

    // Read the segment file
    FILE *fp = fopen(segment_path, "rb");
    if (!fp) {
        log_error("Failed to open segment file: %s (%s)", segment_path, strerror(errno));
        create_stream_error_response(response, 500, "Failed to open segment file");
        return;
    }

    // Get file size
    fseek(fp, 0, SEEK_END);
    long file_size = ftell(fp);
    fseek(fp, 0, SEEK_SET);

    log_info("Successfully opened segment file, size: %ld bytes", file_size);

    // Read file content
    char *content = malloc(file_size);
    if (!content) {
        log_error("Memory allocation failed for file content, size: %ld", file_size);
        fclose(fp);
        create_stream_error_response(response, 500, "Memory allocation failed");
        return;
    }

    size_t bytes_read = fread(content, 1, file_size, fp);
    if (bytes_read != (size_t)file_size) {
        log_error("Failed to read full file content: read %zu of %ld bytes", bytes_read, file_size);
        free(content);
        fclose(fp);
        create_stream_error_response(response, 500, "Failed to read segment file");
        return;
    }

    fclose(fp);

    // Create response
    response->status_code = 200;
    strncpy(response->content_type, "video/mp2t", sizeof(response->content_type) - 1);
    response->content_type[sizeof(response->content_type) - 1] = '\0';
    
    // Add strict cache control headers to prevent caching of HLS segments
    set_response_header(response, "Cache-Control", "no-cache, no-store, must-revalidate, max-age=0");
    set_response_header(response, "Pragma", "no-cache");
    set_response_header(response, "Expires", "0");
    
    // Add timestamp header to help client identify the freshness of the segment
    char timestamp_str[32];
    time_t now = time(NULL);
    snprintf(timestamp_str, sizeof(timestamp_str), "%ld", now);
    set_response_header(response, "X-Timestamp", timestamp_str);
    
    response->body = content;
    response->body_length = file_size;

    log_info("Successfully served segment: %s", segment_filename);
}
