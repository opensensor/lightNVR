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
    log_info("API Detection: Starting detection with API URL: %s", api_url);
    log_info("API Detection: Frame dimensions: %dx%d, channels: %d", width, height, channels);
    log_info("API Detection: Stream name: %s", stream_name ? stream_name : "NULL");
    
    if (!initialized || !curl_handle) {
        log_error("API detection system not initialized");
        return -1;
    }

    if (!api_url || !frame_data || !result) {
        log_error("Invalid parameters for detect_objects_api");
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
    
    // Add the file to the form
    curl_formadd(&formpost, &lastptr,
                CURLFORM_COPYNAME, "image",
                CURLFORM_FILE, temp_filename,
                CURLFORM_CONTENTTYPE, "application/octet-stream",
                CURLFORM_END);
    
    // Add the image dimensions to the form
    char width_str[16], height_str[16], channels_str[16];
    snprintf(width_str, sizeof(width_str), "%d", width);
    snprintf(height_str, sizeof(height_str), "%d", height);
    snprintf(channels_str, sizeof(channels_str), "%d", channels);
    
    curl_formadd(&formpost, &lastptr,
                CURLFORM_COPYNAME, "width",
                CURLFORM_COPYCONTENTS, width_str,
                CURLFORM_END);
    
    curl_formadd(&formpost, &lastptr,
                CURLFORM_COPYNAME, "height",
                CURLFORM_COPYCONTENTS, height_str,
                CURLFORM_END);
    
    curl_formadd(&formpost, &lastptr,
                CURLFORM_COPYNAME, "channels",
                CURLFORM_COPYCONTENTS, channels_str,
                CURLFORM_END);
    
    // Set up the request
    curl_easy_setopt(curl_handle, CURLOPT_URL, api_url);
    curl_easy_setopt(curl_handle, CURLOPT_HTTPPOST, formpost);
    
    // Set up the response buffer
    memory_struct_t chunk;
    chunk.memory = malloc(1);
    chunk.size = 0;
    
    curl_easy_setopt(curl_handle, CURLOPT_WRITEFUNCTION, write_memory_callback);
    curl_easy_setopt(curl_handle, CURLOPT_WRITEDATA, (void *)&chunk);
    
    // Set a timeout
    curl_easy_setopt(curl_handle, CURLOPT_TIMEOUT, 10);
    
    // Perform the request
    log_info("API Detection: Sending request to %s", api_url);
    CURLcode res = curl_easy_perform(curl_handle);
    
    // Clean up the temporary file
    remove(temp_filename);
    
    // Check for errors
    if (res != CURLE_OK) {
        log_error("API Detection: curl_easy_perform() failed: %s", curl_easy_strerror(res));
        
        // Check if it's a connection error
        if (res == CURLE_COULDNT_CONNECT) {
            log_error("API Detection: Could not connect to server at %s. Is the API server running?", api_url);
        } else if (res == CURLE_OPERATION_TIMEDOUT) {
            log_error("API Detection: Connection to %s timed out. Server might be slow or unreachable.", api_url);
        } else if (res == CURLE_COULDNT_RESOLVE_HOST) {
            log_error("API Detection: Could not resolve host %s. Check your network connection and DNS settings.", api_url);
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
    
    return 0;
}
