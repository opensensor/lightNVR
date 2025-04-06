#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <errno.h>
#include <unistd.h>
#include <sys/stat.h>
#include <curl/curl.h>
#include <cJSON.h>

#include "core/logger.h"
#include "core/config.h"
#include "core/shutdown_coordinator.h"
#include "video/api_detection.h"
#include "video/detection_result.h"
#include "video/stream_manager.h"
#include "video/stream_state.h"
#include "database/db_detections.h"

// Global variables
static bool initialized = false;
static CURL *curl_handle = NULL;

// Structure to hold memory for curl response
typedef struct {
    char *memory;
    size_t size;
} memory_struct_t;

// Callback function for curl to write data
static size_t write_memory_callback(void *contents, size_t size, size_t nmemb, void *userp) {
    size_t realsize = size * nmemb;
    memory_struct_t *mem = (memory_struct_t *)userp;

    char *ptr = realloc(mem->memory, mem->size + realsize + 1);
    if (!ptr) {
        log_error("Not enough memory for curl response");
        return 0;
    }

    mem->memory = ptr;
    memcpy(&(mem->memory[mem->size]), contents, realsize);
    mem->size += realsize;
    mem->memory[mem->size] = 0;

    return realsize;
}

/**
 * Initialize the API detection system
 */
int init_api_detection_system(void) {
    if (initialized && curl_handle) {
        log_info("API detection system already initialized");
        return 0;  // Already initialized and curl handle is valid
    }

    // If we have a curl handle but initialized is false, clean it up first
    if (curl_handle) {
        log_warn("API detection system has a curl handle but is marked as uninitialized, cleaning up");
        curl_easy_cleanup(curl_handle);
        curl_handle = NULL;
    }

    // Initialize curl
    CURLcode global_init_result = curl_global_init(CURL_GLOBAL_ALL);
    if (global_init_result != CURLE_OK) {
        log_error("Failed to initialize curl global: %s", curl_easy_strerror(global_init_result));
        return -1;
    }

    curl_handle = curl_easy_init();
    if (!curl_handle) {
        log_error("Failed to initialize curl handle");
        curl_global_cleanup();
        return -1;
    }

    initialized = true;
    log_info("API detection system initialized successfully");
    return 0;
}

/**
 * Shutdown the API detection system
 */
void shutdown_api_detection_system(void) {
    // CRITICAL FIX: Always attempt to clean up resources, even if not marked as initialized
    log_info("Shutting down API detection system (initialized: %s, curl_handle: %p)",
             initialized ? "yes" : "no", (void*)curl_handle);

    // Cleanup curl handle if it exists
    if (curl_handle) {
        log_info("Cleaning up curl handle");
        curl_easy_cleanup(curl_handle);
        curl_handle = NULL;
    }

    // Only call global cleanup if we were initialized
    if (initialized) {
        log_info("Cleaning up curl global resources");
        curl_global_cleanup();
    }

    initialized = false;
    log_info("API detection system shutdown complete");
}

// Include go2rtc headers for integration
#ifdef USE_GO2RTC
#include "video/go2rtc/go2rtc_stream.h"
#include "video/go2rtc/go2rtc_api.h"

// Define GO2RTC_API_PORT if not already defined
#ifndef GO2RTC_API_PORT
#define GO2RTC_API_PORT 1984  // Default go2rtc API port
#endif
#endif

/**
 * Detect objects using the API
 */
int detect_objects_api(const char *api_url, const unsigned char *frame_data,
                      int width, int height, int channels, detection_result_t *result,
                      const char *stream_name) {
    // CRITICAL FIX: Check if we're in shutdown mode or if the stream has been stopped
    if (is_shutdown_initiated()) {
        log_info("API Detection: System shutdown in progress, skipping detection");
        return -1;
    }

    // Check if the stream still exists - but only in non-test builds
    // This avoids undefined references in test builds that don't link with stream_manager.c
#ifndef BUILDING_TEST
    if (stream_name && stream_name[0] != '\0') {
        stream_handle_t stream = get_stream_by_name(stream_name);
        if (!stream) {
            log_info("API Detection: Stream %s no longer exists, skipping detection", stream_name);
            return -1;
        }

        // Check if the stream is still running
        stream_status_t status = get_stream_status(stream);
        if (status != STREAM_STATUS_RUNNING && status != STREAM_STATUS_STARTING) {
            log_info("API Detection: Stream %s is not running (status: %d), but will continue with detection",
                    stream_name, status);
            // Don't return -1 here, continue with detection
            // This allows detection to work with go2rtc even if the stream status isn't RUNNING
        }
    }
#endif // BUILDING_TEST
    // CRITICAL FIX: Check if api_url is the special "api-detection" string
    // If so, get the actual URL from the global config
    const char *actual_api_url = api_url;
    if (api_url && strcmp(api_url, "api-detection") == 0) {
        // Get the API URL from the global config
        extern config_t g_config;
        actual_api_url = g_config.api_detection_url;
        log_info("API Detection: Using API URL from config: %s", actual_api_url ? actual_api_url : "NULL");
    }

    log_info("API Detection: Starting detection with API URL: %s", actual_api_url);
    log_info("API Detection: Frame dimensions: %dx%d, channels: %d", width, height, channels);
    log_info("API Detection: Stream name: %s", stream_name ? stream_name : "NULL");

    if (!initialized || !curl_handle) {
        log_error("API detection system not initialized");
        return -1;
    }

    if (!actual_api_url || !frame_data || !result) {
        log_error("Invalid parameters for detect_objects_api");
        return -1;
    }

    // Check if the URL is valid (must start with http:// or https://)
    if (strncmp(actual_api_url, "http://", 7) != 0 && strncmp(actual_api_url, "https://", 8) != 0) {
        log_error("API Detection: Invalid URL format: %s (must start with http:// or https://)", actual_api_url);
        return -1;
    }

    // Initialize result
    result->count = 0;

    // Set up curl for multipart/form-data
    struct curl_httppost *formpost = NULL;
    struct curl_httppost *lastptr = NULL;

    // Create a temporary file for the image data using mkstemp (safer than tmpnam)
    char temp_filename[] = "/tmp/lightnvr_api_detection_XXXXXX";
    int temp_fd = mkstemp(temp_filename);
    if (temp_fd < 0) {
        log_error("Failed to create temporary file: %s", strerror(errno));
        return -1;
    }

    // Flag to track if we're using a go2rtc snapshot
    bool using_go2rtc_snapshot = false;
    char image_filename[256];
    snprintf(image_filename, sizeof(image_filename), "%s.jpg", temp_filename);

    // Try to get a snapshot from go2rtc if enabled and we have a stream name
    #ifdef USE_GO2RTC
    if (stream_name && stream_name[0] != '\0') {
        log_info("API Detection: Checking if go2rtc is available for stream %s", stream_name);

        // Check if go2rtc is ready
        if (go2rtc_stream_is_ready()) {
            log_info("API Detection: go2rtc is available, trying to get snapshot for stream %s", stream_name);

            // Get the go2rtc API port
            int api_port = GO2RTC_API_PORT; // Default from CMake

            // Try to get the port from the go2rtc API
            int rtsp_port = 0;
            if (go2rtc_api_get_server_info(&rtsp_port)) {
                // If we can get the RTSP port, we're using the configured API port
                log_info("API Detection: Using go2rtc API port from config: %d", api_port);
            } else {
                log_warn("API Detection: Could not get server info, using default API port: %d", api_port);
            }

            // Construct the go2rtc snapshot URL
            char go2rtc_url[1024];
            snprintf(go2rtc_url, sizeof(go2rtc_url), "http://localhost:%d/api/frame.jpeg?src=%s",
                    api_port, stream_name);
            log_info("API Detection: Using go2rtc snapshot URL: %s", go2rtc_url);

            // Set up a new curl handle for downloading the snapshot
            CURL *snapshot_curl = curl_easy_init();
            if (snapshot_curl) {
                // Open the image file for writing
                FILE *image_file = fopen(image_filename, "wb");
                if (image_file) {
                    // Set up curl options
                    curl_easy_setopt(snapshot_curl, CURLOPT_URL, go2rtc_url);
                    curl_easy_setopt(snapshot_curl, CURLOPT_WRITEFUNCTION, NULL); // Use default write function
                    curl_easy_setopt(snapshot_curl, CURLOPT_WRITEDATA, image_file);
                    curl_easy_setopt(snapshot_curl, CURLOPT_TIMEOUT, 5); // 5 second timeout

                    // Perform the request
                    log_info("API Detection: Downloading go2rtc snapshot from %s", go2rtc_url);
                    CURLcode res = curl_easy_perform(snapshot_curl);

                    // Close the file
                    fclose(image_file);

                    // Check if the download was successful
                    if (res == CURLE_OK) {
                        // Check if the file has content
                        struct stat st;
                        if (stat(image_filename, &st) == 0 && st.st_size > 0) {
                            log_info("API Detection: Successfully downloaded go2rtc snapshot: %s (size: %ld bytes)",
                                    image_filename, (long)st.st_size);
                            using_go2rtc_snapshot = true;
                        } else {
                            log_warn("API Detection: go2rtc snapshot file is empty, falling back to frame conversion");
                        }
                    } else {
                        log_warn("API Detection: Failed to download go2rtc snapshot: %s, falling back to frame conversion",
                                curl_easy_strerror(res));
                    }

                    // Clean up curl
                    curl_easy_cleanup(snapshot_curl);
                } else {
                    log_warn("API Detection: Failed to open image file for writing: %s", image_filename);
                    curl_easy_cleanup(snapshot_curl);
                }
            } else {
                log_warn("API Detection: Failed to initialize curl for go2rtc snapshot");
            }
        } else {
            log_info("API Detection: go2rtc is not available, using frame conversion");
        }
    }
    #endif

    // If we didn't get a go2rtc snapshot, convert the raw frame data
    if (!using_go2rtc_snapshot) {
        // Convert file descriptor to FILE pointer
        FILE *temp_file = fdopen(temp_fd, "wb");
        if (!temp_file) {
            log_error("Failed to open temporary file for writing: %s", strerror(errno));
            close(temp_fd);
            unlink(temp_filename);
            return -1;
        }

        // Write the raw image data to the file
        size_t bytes_written = fwrite(frame_data, 1, width * height * channels, temp_file);
        fclose(temp_file);

        if (bytes_written != width * height * channels) {
            log_error("Failed to write image data to temporary file");
            remove(temp_filename);
            return -1;
        }

        // CRITICAL FIX: Modify the request to match the API server's expected format
        // Based on working curl examples:
        // curl -X 'POST' 'http://127.0.0.1:9001/api/v1/detect?backend=tflite&confidence_threshold=.5&return_image=false'
        //   -H 'accept: application/json' -H 'Content-Type: multipart/form-data'
        //   -F 'file=@image.jpg;type=image/jpeg'

        // Determine the correct ImageMagick parameters based on channels
        const char *pixel_format;
        if (channels == 1) {
            pixel_format = "gray";
        } else if (channels == 3) {
            pixel_format = "rgb";
        } else if (channels == 4) {
            pixel_format = "rgba";
        } else {
            log_error("API Detection: Unsupported number of channels: %d", channels);
            remove(temp_filename);
            return -1;
        }

        // Use system command to convert the raw data to JPEG using ImageMagick
        // The -depth 8 parameter specifies 8 bits per channel
        // The -size WxH parameter specifies the dimensions of the input image
        // The pixel_format: parameter specifies the pixel format of the input image
        char convert_cmd[1024];
        snprintf(convert_cmd, sizeof(convert_cmd),
                 "convert -size %dx%d -depth 8 %s:%s -quality 90 %s",
                 width, height, pixel_format, temp_filename, image_filename);
        log_info("API Detection: Converting raw data to JPEG: %s", convert_cmd);

        int convert_result = system(convert_cmd);
        if (convert_result != 0) {
            log_error("API Detection: Failed to convert raw data to JPEG (error code: %d)", convert_result);

            // Try an alternative approach using ffmpeg
            log_info("API Detection: Trying alternative conversion with ffmpeg");
            char ffmpeg_cmd[1024];
            snprintf(ffmpeg_cmd, sizeof(ffmpeg_cmd),
                    "ffmpeg -f rawvideo -pixel_format %s -video_size %dx%d -i %s -y %s",
                    channels == 3 ? "rgb24" : (channels == 4 ? "rgba" : "gray"),
                    width, height, temp_filename, image_filename);
            log_info("API Detection: Running ffmpeg command: %s", ffmpeg_cmd);

            int ffmpeg_result = system(ffmpeg_cmd);
            if (ffmpeg_result != 0) {
                log_error("API Detection: Failed to convert with ffmpeg (error code: %d)", ffmpeg_result);
                log_info("API Detection: Falling back to raw data with application/octet-stream MIME type");

                // Add the file to the form with the correct field name 'file' (not 'image')
                curl_formadd(&formpost, &lastptr,
                            CURLFORM_COPYNAME, "file",
                            CURLFORM_FILE, temp_filename,
                            CURLFORM_CONTENTTYPE, "application/octet-stream",
                            CURLFORM_END);
            } else {
                log_info("API Detection: Successfully converted raw data to JPEG with ffmpeg: %s", image_filename);
                using_go2rtc_snapshot = true; // We're using the converted image
            }
        } else {
            log_info("API Detection: Successfully converted raw data to JPEG with ImageMagick: %s", image_filename);

            // Verify the file was created and has a non-zero size
            struct stat st;
            if (stat(image_filename, &st) == 0 && st.st_size > 0) {
                log_info("API Detection: JPEG file size: %ld bytes", (long)st.st_size);
                using_go2rtc_snapshot = true; // We're using the converted image
            } else {
                log_error("API Detection: JPEG file was not created or has zero size");
                log_info("API Detection: Falling back to raw data with application/octet-stream MIME type");

                // Add the file to the form with the correct field name 'file' (not 'image')
                curl_formadd(&formpost, &lastptr,
                            CURLFORM_COPYNAME, "file",
                            CURLFORM_FILE, temp_filename,
                            CURLFORM_CONTENTTYPE, "application/octet-stream",
                            CURLFORM_END);
            }
        }
    }

    // If we have a valid JPEG image (either from go2rtc or conversion), use it
    if (using_go2rtc_snapshot) {
        // Add the JPEG file to the form
        curl_formadd(&formpost, &lastptr,
                    CURLFORM_COPYNAME, "file",
                    CURLFORM_FILE, image_filename,
                    CURLFORM_CONTENTTYPE, "image/jpeg",
                    CURLFORM_END);
    }

    // Construct the URL with query parameters
    char url_with_params[1024];
    snprintf(url_with_params, sizeof(url_with_params),
             "%s?backend=tflite&confidence_threshold=0.5&return_image=false",
             actual_api_url);
    log_info("API Detection: Using URL with parameters: %s", url_with_params);

    // Set up the request with the URL including query parameters
    curl_easy_setopt(curl_handle, CURLOPT_URL, url_with_params);
    curl_easy_setopt(curl_handle, CURLOPT_HTTPPOST, formpost);

    // Add the accept header
    struct curl_slist *headers = NULL;
    headers = curl_slist_append(headers, "accept: application/json");
    curl_easy_setopt(curl_handle, CURLOPT_HTTPHEADER, headers);

    // Set up the response buffer
    memory_struct_t chunk;
    chunk.memory = malloc(1);
    chunk.size = 0;

    curl_easy_setopt(curl_handle, CURLOPT_WRITEFUNCTION, write_memory_callback);
    curl_easy_setopt(curl_handle, CURLOPT_WRITEDATA, (void *)&chunk);

    // Set a timeout
    curl_easy_setopt(curl_handle, CURLOPT_TIMEOUT, 10);

    // Perform the request
    log_info("API Detection: Sending request to %s", url_with_params);

    // CRITICAL FIX: Check if curl handle is still valid before performing the request
    if (!curl_handle || !initialized) {
        log_error("API Detection: curl handle is invalid or API detection system not initialized");
        free(chunk.memory);
        curl_formfree(formpost);
        curl_slist_free_all(headers);

        // Clean up the temporary files
        remove(temp_filename);
        // Remove the JPEG file if it was created by either ImageMagick or ffmpeg
        struct stat st;
        if (stat(image_filename, &st) == 0) {
            log_info("API Detection: Removing temporary JPEG file: %s", image_filename);
            remove(image_filename);
        }

        // Initialize result to empty to prevent segmentation fault
        result->count = 0;
        return -1;
    }

    CURLcode res = curl_easy_perform(curl_handle);

    // Clean up the temporary files
    remove(temp_filename);
    // Remove the JPEG file if it was created by either ImageMagick or ffmpeg
    struct stat st;
    if (stat(image_filename, &st) == 0) {
        log_info("API Detection: Removing temporary JPEG file: %s", image_filename);
        remove(image_filename);
    }

    // Check for errors
    if (res != CURLE_OK) {
        log_error("API Detection: curl_easy_perform() failed: %s", curl_easy_strerror(res));

        // Check if it's a connection error
        if (res == CURLE_COULDNT_CONNECT) {
            log_error("API Detection: Could not connect to server at %s. Is the API server running?", url_with_params);
        } else if (res == CURLE_OPERATION_TIMEDOUT) {
            log_error("API Detection: Connection to %s timed out. Server might be slow or unreachable.", url_with_params);
        } else if (res == CURLE_COULDNT_RESOLVE_HOST) {
            log_error("API Detection: Could not resolve host %s. Check your network connection and DNS settings.", url_with_params);
        } else if (res == CURLE_FAILED_INIT) {
            log_error("API Detection: Curl initialization failed. Reinitializing curl handle...");

            // Attempt to reinitialize curl
            if (curl_handle) {
                curl_easy_cleanup(curl_handle);
                curl_handle = NULL;
            }

            curl_handle = curl_easy_init();
            if (!curl_handle) {
                log_error("API Detection: Failed to reinitialize curl handle");
                initialized = false; // Mark as uninitialized to force reinitialization next time
            } else {
                log_info("API Detection: Successfully reinitialized curl handle");
            }
        }

        free(chunk.memory);
        curl_formfree(formpost);
        curl_slist_free_all(headers);

        // Initialize result to empty to prevent segmentation fault
        result->count = 0;
        return -1;
    }

    // Get the HTTP response code
    long http_code = 0;
    curl_easy_getinfo(curl_handle, CURLINFO_RESPONSE_CODE, &http_code);

    if (http_code != 200) {
        log_error("API request failed with HTTP code %ld", http_code);
        free(chunk.memory);
        curl_formfree(formpost);
        curl_slist_free_all(headers);
        return -1;
    }

    // Parse the JSON response
    // CRITICAL FIX: Add additional logging and validation for JSON parsing
    if (!chunk.memory || chunk.size == 0) {
        log_error("API Detection: Empty response from server");
        free(chunk.memory);
        curl_formfree(formpost);
        curl_slist_free_all(headers);
        // Initialize result to empty to prevent segmentation fault
        result->count = 0;
        return -1;
    }

    // Log the first few bytes of the response for debugging
    char preview[64] = {0};
    int preview_len = chunk.size < 63 ? chunk.size : 63;
    memcpy(preview, chunk.memory, preview_len);
    preview[preview_len] = '\0';
    // Replace non-printable characters with dots
    for (int i = 0; i < preview_len; i++) {
        if (preview[i] < 32 || preview[i] > 126) {
            preview[i] = '.';
        }
    }
    log_info("API Detection: Response preview: %s", preview);

    cJSON *root = cJSON_Parse(chunk.memory);

    if (!root) {
        const char *error_ptr = cJSON_GetErrorPtr();
        log_error("Failed to parse JSON response: %s", error_ptr ? error_ptr : "Unknown error");
        // Log more details about the response
        log_error("API Detection: Response size: %zu bytes", chunk.size);
        log_error("API Detection: Response preview: %s", preview);
        free(chunk.memory);
        curl_formfree(formpost);
        curl_slist_free_all(headers);
        // Initialize result to empty to prevent segmentation fault
        result->count = 0;
        return -1;
    }

    // Extract the detections
    cJSON *detections = cJSON_GetObjectItem(root, "detections");
    if (!detections || !cJSON_IsArray(detections)) {
        log_error("Invalid JSON response: missing or invalid 'detections' array");
        // Log the JSON for debugging
        char *json_str = cJSON_Print(root);
        if (json_str) {
            log_error("API Detection: Full JSON response: %s", json_str);
            free(json_str);
        }
        cJSON_Delete(root);
        free(chunk.memory);
        curl_formfree(formpost);
        curl_slist_free_all(headers);
        // Initialize result to empty to prevent segmentation fault
        result->count = 0;
        return -1;
    }

    // Process each detection
    int array_size = cJSON_GetArraySize(detections);
    for (int i = 0; i < array_size; i++) {
        if (result->count >= MAX_DETECTIONS) {
            log_warn("Maximum number of detections reached (%d)", MAX_DETECTIONS);
            break;
        }

        cJSON *detection = cJSON_GetArrayItem(detections, i);
        if (!detection) continue;

        // Extract the detection data
        cJSON *label = cJSON_GetObjectItem(detection, "label");
        cJSON *confidence = cJSON_GetObjectItem(detection, "confidence");

        // The bounding box coordinates might be in a nested object
        cJSON *bounding_box = cJSON_GetObjectItem(detection, "bounding_box");
        cJSON *x_min = NULL;
        cJSON *y_min = NULL;
        cJSON *x_max = NULL;
        cJSON *y_max = NULL;

        if (bounding_box) {
            // Get coordinates from the nested bounding_box object
            x_min = cJSON_GetObjectItem(bounding_box, "x_min");
            y_min = cJSON_GetObjectItem(bounding_box, "y_min");
            x_max = cJSON_GetObjectItem(bounding_box, "x_max");
            y_max = cJSON_GetObjectItem(bounding_box, "y_max");
            log_info("API Detection: Found bounding_box object in JSON response");
        } else {
            // Try to get coordinates directly from the detection object (old format)
            x_min = cJSON_GetObjectItem(detection, "x_min");
            y_min = cJSON_GetObjectItem(detection, "y_min");
            x_max = cJSON_GetObjectItem(detection, "x_max");
            y_max = cJSON_GetObjectItem(detection, "y_max");
            log_info("API Detection: Using direct coordinates from JSON response");
        }

        if (!label || !cJSON_IsString(label) ||
            !confidence || !cJSON_IsNumber(confidence) ||
            !x_min || !cJSON_IsNumber(x_min) ||
            !y_min || !cJSON_IsNumber(y_min) ||
            !x_max || !cJSON_IsNumber(x_max) ||
            !y_max || !cJSON_IsNumber(y_max)) {
            log_warn("Invalid detection data in JSON response");
            // Log the actual JSON for debugging
            char *json_str = cJSON_Print(detection);
            if (json_str) {
                log_warn("Detection JSON: %s", json_str);
                free(json_str);
            }
            continue;
        }

        // Add the detection to the result
        strncpy(result->detections[result->count].label, label->valuestring, MAX_LABEL_LENGTH - 1);
        result->detections[result->count].label[MAX_LABEL_LENGTH - 1] = '\0';
        result->detections[result->count].confidence = confidence->valuedouble;
        result->detections[result->count].x = x_min->valuedouble;
        result->detections[result->count].y = y_min->valuedouble;
        result->detections[result->count].width = x_max->valuedouble - x_min->valuedouble;
        result->detections[result->count].height = y_max->valuedouble - y_min->valuedouble;

        result->count++;
    }

    // Store the detections in the database if we have a valid stream name
    if (stream_name && stream_name[0] != '\0') {
        store_detections_in_db(stream_name, result, 0); // 0 means use current time
    } else {
        log_warn("No stream name provided, skipping database storage");
    }

    // Clean up
    cJSON_Delete(root);
    free(chunk.memory);
    curl_formfree(formpost);
    curl_slist_free_all(headers);

    return 0;
}
