#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <errno.h>
#include <unistd.h>
#include <sys/stat.h>
#include <curl/curl.h>
#include <cJSON.h>
#include <pthread.h>
#include <signal.h>  // CRITICAL FIX: Added for signal handling to prevent floating point exceptions

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
static pthread_mutex_t curl_mutex = PTHREAD_MUTEX_INITIALIZER;

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

/**
 * Detect objects using the API
 */
int detect_objects_api(const char *api_url, const unsigned char *frame_data,
                      int width, int height, int channels, detection_result_t *result,
                      const char *stream_name) {
    // CRITICAL FIX: Check if we're in shutdown mode or if the stream has been stopped
    if (is_shutdown_initiated()) {
        log_info("API Detection: System shutdown in progress, skipping detection");
        return 0;
    }

    // CRITICAL FIX: Add thread safety for curl operations
    pthread_mutex_lock(&curl_mutex);
    
    // Initialize result to empty at the beginning to prevent segmentation fault
    if (result) {
        memset(result, 0, sizeof(detection_result_t));
    } else {
        log_error("API Detection: NULL result pointer provided");
        pthread_mutex_unlock(&curl_mutex);
        return 0;
    }
    
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
        pthread_mutex_unlock(&curl_mutex);
        return 0;
    }

    if (!actual_api_url || !frame_data || !result) {
        log_error("Invalid parameters for detect_objects_api");
        pthread_mutex_unlock(&curl_mutex);
        return 0;
    }

    // Check if the URL is valid (must start with http:// or https://)
    if (strncmp(actual_api_url, "http://", 7) != 0 && strncmp(actual_api_url, "https://", 8) != 0) {
        log_error("API Detection: Invalid URL format: %s (must start with http:// or https://)", actual_api_url);
        pthread_mutex_unlock(&curl_mutex);
        return 0;
    }

    // Set up curl for multipart/form-data
    struct curl_httppost *formpost = NULL;
    struct curl_httppost *lastptr = NULL;

    // Create a temporary file for the image data using mkstemp (safer than tmpnam)
    char temp_filename[] = "/tmp/lightnvr_api_detection_XXXXXX";
    int temp_fd = mkstemp(temp_filename);
    if (temp_fd < 0) {
        log_error("Failed to create temporary file: %s", strerror(errno));
        pthread_mutex_unlock(&curl_mutex);
        return 0;
    }

    char image_filename[256];
    snprintf(image_filename, sizeof(image_filename), "%s.jpg", temp_filename);

    // Convert file descriptor to FILE pointer
    FILE *temp_file = fdopen(temp_fd, "wb");
    if (!temp_file) {
        log_error("Failed to open temporary file for writing: %s", strerror(errno));
        close(temp_fd);
        unlink(temp_filename);
        pthread_mutex_unlock(&curl_mutex);
        return 0;
    }

    // Write the raw image data to the file
    size_t bytes_written = fwrite(frame_data, 1, width * height * channels, temp_file);
    fclose(temp_file);

    if (bytes_written != width * height * channels) {
        log_error("Failed to write image data to temporary file");
        remove(temp_filename);
        pthread_mutex_unlock(&curl_mutex);
        return 0;
    }

    // CRITICAL FIX: Modify the request to match the API server's expected format
    // Based on working curl examples:
    // curl -X 'POST' 'http://127.0.0.1:9001/api/v1/detect?backend=tflite&confidence_threshold=.5&return_image=false'
    //   -H 'accept: application/json' -H 'Content-Type: multipart/form-data'
    //   -F 'file=@image.jpg;type=image/jpeg'

    // CRITICAL FIX: Determine the correct ImageMagick parameters based on channels with safety checks
    const char *pixel_format = NULL;

    // Validate channels to prevent segmentation faults
    if (channels <= 0 || channels > 4) {
        log_error("API Detection: Invalid number of channels: %d (must be 1, 3, or 4)", channels);
        remove(temp_filename);
        pthread_mutex_unlock(&curl_mutex);
        return 0;
    }

    // Set pixel format based on validated channels
    if (channels == 1) {
        pixel_format = "gray";
        log_info("API Detection: Using grayscale pixel format for %d channel", channels);
    } else if (channels == 3) {
        pixel_format = "rgb";
        log_info("API Detection: Using RGB pixel format for %d channels", channels);
    } else if (channels == 4) {
        pixel_format = "rgba";
        log_info("API Detection: Using RGBA pixel format for %d channels", channels);
    }

    // Double-check that pixel_format is set
    if (!pixel_format) {
        log_error("API Detection: Failed to determine pixel format for %d channels", channels);
        remove(temp_filename);
        pthread_mutex_unlock(&curl_mutex);
        return 0;
    }

    // CRITICAL FIX: Use system command to convert the raw data to JPEG using ImageMagick with safety checks
    // The -depth 8 parameter specifies 8 bits per channel
    // The -size WxH parameter specifies the dimensions of the input image
    // The pixel_format: parameter specifies the pixel format of the input image
    char convert_cmd[1024] = {0}; // Initialize to zeros

    // Validate width and height to prevent buffer overflows
    // Format the command with safety checks
// CRITICAL FIX: Add additional validation for width, height, and channels to prevent floating point exceptions
// Check for valid dimensions that won't cause arithmetic errors
if (width <= 0 || width > 8192 || height <= 0 || height > 8192) {
    log_error("API Detection: Invalid image dimensions: %dx%d (must be positive and <= 8192)", width, height);
    remove(temp_filename);
    pthread_mutex_unlock(&curl_mutex);
    return 0;
}

// Check for valid channel count
if (channels != 1 && channels != 3 && channels != 4) {
    log_error("API Detection: Invalid channel count: %d (must be 1, 3, or 4)", channels);
    remove(temp_filename);
    pthread_mutex_unlock(&curl_mutex);
    return 0;
}

// CRITICAL FIX: Verify that the raw data size is correct to prevent buffer overflows
size_t expected_size = width * height * channels;
if (expected_size == 0 || expected_size > 100000000) { // 100MB sanity check
    log_error("API Detection: Invalid data size: %zu bytes", expected_size);
    remove(temp_filename);
        pthread_mutex_unlock(&curl_mutex);
    // CRITICAL FIX: Add signal handling to prevent floating point exceptions
    // Set up a signal handler for SIGFPE (floating point exception)
    struct sigaction sa, old_sa;
    memset(&sa, 0, sizeof(sa));
    sa.sa_handler = SIG_IGN; // Ignore the signal
    sigaction(SIGFPE, &sa, &old_sa);
    
    int convert_result = system(convert_cmd);
    
    // Restore the original signal handler
    sigaction(SIGFPE, &old_sa, NULL);
    
    if (convert_result != 0) {
        log_error("API Detection: Failed to convert raw data to JPEG (error code: %d)", convert_result);
        
        // Clean up and return early
        remove(temp_filename);
        if (access(image_filename, F_OK) == 0) {
            remove(image_filename);
        }
        pthread_mutex_unlock(&curl_mutex);
        return 0;
    }
        return 0;
    }

    log_info("API Detection: Converting raw data to JPEG: %s", convert_cmd);

    int convert_result = system(convert_cmd);
    if (convert_result != 0) {
        log_error("API Detection: Failed to convert raw data to JPEG (error code: %d)", convert_result);
        
        // Clean up and return early
        remove(temp_filename);
        if (access(image_filename, F_OK) == 0) {
            remove(image_filename);
        }
        pthread_mutex_unlock(&curl_mutex);
        return 0;
    }
    
    // Verify the JPEG file exists and has content before proceeding
    struct stat jpeg_stat;
    if (stat(image_filename, &jpeg_stat) != 0 || jpeg_stat.st_size == 0) {
        log_error("API Detection: JPEG file was not created or has zero size: %s", image_filename);
        remove(temp_filename);
        if (access(image_filename, F_OK) == 0) {
            remove(image_filename);
        }
        pthread_mutex_unlock(&curl_mutex);
        return 0;
    }
    
    log_info("API Detection: Successfully converted raw data to JPEG: %s (size: %ld bytes)", 
             image_filename, (long)jpeg_stat.st_size);

    // Add the JPEG file to the form
    // CRITICAL FIX: Check if image_filename exists before adding to form
    struct stat image_stat;
    memory_struct_t chunk = {0};
    chunk.memory = NULL;
    struct curl_slist *headers = NULL;
    if (stat(image_filename, &image_stat) == 0 && image_stat.st_size > 0) {
        CURLFORMcode form_result = curl_formadd(&formpost, &lastptr,
                    CURLFORM_COPYNAME, "file",
                    CURLFORM_FILE, image_filename,
                    CURLFORM_CONTENTTYPE, "image/jpeg",
                    CURLFORM_END);
                    
        if (form_result != CURL_FORMADD_OK) {
            log_error("API Detection: Failed to add JPEG file to form (error code: %d)", form_result);
            free(chunk.memory);
            curl_formfree(formpost);
            curl_slist_free_all(headers);
            remove(temp_filename);
            remove(image_filename);
            result->count = 0;
            pthread_mutex_unlock(&curl_mutex);
            return 0;
        }
        
        log_info("API Detection: Successfully added JPEG file to form: %s", image_filename);
    } else {
        log_error("API Detection: JPEG file missing or empty: %s", image_filename);
        // Now we can safely free them
        if (chunk.memory) free(chunk.memory);
        if (formpost) curl_formfree(formpost);
        if (headers) curl_slist_free_all(headers);
        
        remove(temp_filename);
        if (stat(image_filename, &image_stat) == 0) {
            remove(image_filename);
        }
        result->count = 0;
        pthread_mutex_unlock(&curl_mutex);
        return 0;
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
    headers = curl_slist_append(headers, "accept: application/json");
    curl_easy_setopt(curl_handle, CURLOPT_HTTPHEADER, headers);

    // Set up the response buffer
    chunk.memory = malloc(1);
    chunk.size = 0;

    curl_easy_setopt(curl_handle, CURLOPT_WRITEFUNCTION, write_memory_callback);
    curl_easy_setopt(curl_handle, CURLOPT_WRITEDATA, (void *)&chunk);

    // Set a timeout
    curl_easy_setopt(curl_handle, CURLOPT_CONNECTTIMEOUT, 5L); // 5 seconds connection timeout
    curl_easy_setopt(curl_handle, CURLOPT_TIMEOUT, 10L);       // 10 seconds total timeout

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
        pthread_mutex_unlock(&curl_mutex);
        return 0;
    }

    CURLcode res = curl_easy_perform(curl_handle);

    // Clean up the temporary files first to ensure they're always removed
    remove(temp_filename);
    if (access(image_filename, F_OK) == 0) {
        log_info("API Detection: Removing temporary JPEG file: %s", image_filename);
        remove(image_filename);
    }
    
    // Check for errors with detailed logging
    if (res != CURLE_OK) {
        log_error("API Detection: curl_easy_perform() failed: %s", curl_easy_strerror(res));
        
        // Handle specific error cases
        if (res == CURLE_COULDNT_CONNECT || res == CURLE_OPERATION_TIMEDOUT) {
            log_error("API Detection: Could not connect to API server at %s. Is the server running?", actual_api_url);
        }
        
        // Clean up resources
        if (chunk.memory) free(chunk.memory);
        if (formpost) curl_formfree(formpost);
        if (headers) curl_slist_free_all(headers);
        
        // Initialize result to empty to prevent segmentation fault
        result->count = 0;
        pthread_mutex_unlock(&curl_mutex);
        return 0;
    }

    // Get the HTTP response code
    long http_code = 0;
    curl_easy_getinfo(curl_handle, CURLINFO_RESPONSE_CODE, &http_code);

    if (http_code != 200) {
        log_error("API request failed with HTTP code %ld", http_code);
        free(chunk.memory);
        curl_formfree(formpost);
        curl_slist_free_all(headers);
        pthread_mutex_unlock(&curl_mutex);
        return 0;
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
        pthread_mutex_unlock(&curl_mutex);
        return 0;
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
        pthread_mutex_unlock(&curl_mutex);
        return 0;
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
        pthread_mutex_unlock(&curl_mutex);
        return 0;
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

    pthread_mutex_unlock(&curl_mutex);
    return 0;
}
