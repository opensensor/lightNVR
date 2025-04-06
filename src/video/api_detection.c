#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <errno.h>
#include <unistd.h>
#include <curl/curl.h>
#include <cJSON.h>

#include "core/logger.h"
#include "core/config.h"
#include "video/api_detection.h"
#include "video/detection_result.h"
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
    if (initialized) {
        return 0;  // Already initialized
    }

    // Initialize curl
    curl_global_init(CURL_GLOBAL_ALL);
    curl_handle = curl_easy_init();
    if (!curl_handle) {
        log_error("Failed to initialize curl");
        curl_global_cleanup();
        return -1;
    }

    initialized = true;
    log_info("API detection system initialized");
    return 0;
}

/**
 * Shutdown the API detection system
 */
void shutdown_api_detection_system(void) {
    if (!initialized) {
        return;
    }

    // Cleanup curl
    if (curl_handle) {
        curl_easy_cleanup(curl_handle);
        curl_handle = NULL;
    }
    curl_global_cleanup();

    initialized = false;
    log_info("API detection system shutdown");
}

/**
 * Detect objects using the API
 */
int detect_objects_api(const char *api_url, const unsigned char *frame_data,
                      int width, int height, int channels, detection_result_t *result,
                      const char *stream_name) {
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
    
    // Construct the URL with query parameters
    char url_with_params[1024];
    snprintf(url_with_params, sizeof(url_with_params), 
             "%s?backend=tflite&confidence_threshold=0.5&return_image=false", 
             actual_api_url);
    log_info("API Detection: Using URL with parameters: %s", url_with_params);
    
    // CRITICAL FIX: We need to convert the raw image data to a proper image format
    // The API expects a proper image file (JPEG/PNG), not raw RGB/RGBA data
    
    // Create a new temporary file with a .jpg extension
    char image_filename[256];
    snprintf(image_filename, sizeof(image_filename), "%s.jpg", temp_filename);
    
    // Use system command to convert the raw data to JPEG using ImageMagick
    char convert_cmd[1024];
    snprintf(convert_cmd, sizeof(convert_cmd), 
             "convert -size %dx%d -depth 8 rgb:%s %s", 
             width, height, temp_filename, image_filename);
    log_info("API Detection: Converting raw data to JPEG: %s", convert_cmd);
    
    int convert_result = system(convert_cmd);
    if (convert_result != 0) {
        log_error("API Detection: Failed to convert raw data to JPEG (error code: %d)", convert_result);
        log_info("API Detection: Falling back to raw data with application/octet-stream MIME type");
        
        // Add the file to the form with the correct field name 'file' (not 'image')
        curl_formadd(&formpost, &lastptr,
                    CURLFORM_COPYNAME, "file",
                    CURLFORM_FILE, temp_filename,
                    CURLFORM_CONTENTTYPE, "application/octet-stream",
                    CURLFORM_END);
    } else {
        log_info("API Detection: Successfully converted raw data to JPEG: %s", image_filename);
        
        // Add the JPEG file to the form
        curl_formadd(&formpost, &lastptr,
                    CURLFORM_COPYNAME, "file",
                    CURLFORM_FILE, image_filename,
                    CURLFORM_CONTENTTYPE, "image/jpeg",
                    CURLFORM_END);
    }
    
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
    CURLcode res = curl_easy_perform(curl_handle);
    
    // Clean up the temporary files
    remove(temp_filename);
    if (convert_result == 0) {
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
        }
        
        free(chunk.memory);
        curl_formfree(formpost);
        return -1;
    }
    
    // Get the HTTP response code
    long http_code = 0;
    curl_easy_getinfo(curl_handle, CURLINFO_RESPONSE_CODE, &http_code);
    
    if (http_code != 200) {
        log_error("API request failed with HTTP code %ld", http_code);
        free(chunk.memory);
        curl_formfree(formpost);
        return -1;
    }
    
    // Parse the JSON response
    cJSON *root = cJSON_Parse(chunk.memory);
    
    if (!root) {
        const char *error_ptr = cJSON_GetErrorPtr();
        log_error("Failed to parse JSON response: %s", error_ptr ? error_ptr : "Unknown error");
        free(chunk.memory);
        curl_formfree(formpost);
        return -1;
    }
    
    // Extract the detections
    cJSON *detections = cJSON_GetObjectItem(root, "detections");
    if (!detections || !cJSON_IsArray(detections)) {
        log_error("Invalid JSON response: missing or invalid 'detections' array");
        cJSON_Delete(root);
        free(chunk.memory);
        curl_formfree(formpost);
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
        cJSON *x_min = cJSON_GetObjectItem(detection, "x_min");
        cJSON *y_min = cJSON_GetObjectItem(detection, "y_min");
        cJSON *x_max = cJSON_GetObjectItem(detection, "x_max");
        cJSON *y_max = cJSON_GetObjectItem(detection, "y_max");
        
        if (!label || !cJSON_IsString(label) ||
            !confidence || !cJSON_IsNumber(confidence) ||
            !x_min || !cJSON_IsNumber(x_min) ||
            !y_min || !cJSON_IsNumber(y_min) ||
            !x_max || !cJSON_IsNumber(x_max) ||
            !y_max || !cJSON_IsNumber(y_max)) {
            log_warn("Invalid detection data in JSON response");
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
