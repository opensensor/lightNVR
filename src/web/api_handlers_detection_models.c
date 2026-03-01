#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <dirent.h>
#include <sys/stat.h>
#include <unistd.h>

#include "web/api_handlers.h"
#include "web/request_response.h"
#include "core/logger.h"
#include "core/config.h"
#include "video/detection.h"
#include "video/api_detection.h"
#include "video/onvif_detection.h"

// Default models directory
#define DEFAULT_MODELS_DIR "/var/lib/lightnvr/models"

/**
 * @brief Backend-agnostic handler for GET /api/detection/models
 *
 * This handler returns a list of available detection models
 */
void handle_get_detection_models(const http_request_t *req, http_response_t *res) {
    (void)req;
    log_info("Handling GET /api/detection/models request");

    // Get models directory from config or use default
    config_t *config = &g_config;
    const char *models_dir = config->models_path;
    if (!models_dir || strlen(models_dir) == 0) {
        models_dir = DEFAULT_MODELS_DIR;
    }
    
    log_info("Scanning models directory: %s", models_dir);
    
    // Create JSON response
    cJSON *response = cJSON_CreateObject();
    if (!response) {
        log_error("Failed to create response JSON object");
        http_response_set_json_error(res, 500, "Failed to create response JSON");
        return;
    }
    
    // Create models array
    cJSON *models_array = cJSON_CreateArray();
    if (!models_array) {
        log_error("Failed to create models JSON array");
        cJSON_Delete(response);
        http_response_set_json_error(res, 500, "Failed to create models JSON");
        return;
    }
    
    // Add models array to response
    cJSON_AddItemToObject(response, "models", models_array);
    
    // Initialize model count
    int model_count = 0;

    // Add built-in motion detection model (always available, regardless of models directory)
    cJSON *motion_model = cJSON_CreateObject();
    if (motion_model) {
        cJSON_AddStringToObject(motion_model, "id", "motion");
        cJSON_AddStringToObject(motion_model, "name", "Built-in Motion Detection");
        cJSON_AddStringToObject(motion_model, "path", "motion");
        cJSON_AddStringToObject(motion_model, "type", "builtin");
        cJSON_AddBoolToObject(motion_model, "supported", true);
        cJSON_AddStringToObject(motion_model, "description", "Built-in motion detection");

        cJSON_AddItemToArray(models_array, motion_model);
        model_count++;

        log_info("Added built-in motion detection model");
    } else {
        log_error("Failed to create motion model JSON object");
    }

    // Add API detection model (always available)
    cJSON *api_model = cJSON_CreateObject();
    if (api_model) {
        cJSON_AddStringToObject(api_model, "id", "api-detection");
        cJSON_AddStringToObject(api_model, "name", "API Detection (light-object-detect)");
        cJSON_AddStringToObject(api_model, "path", "api-detection");
        cJSON_AddStringToObject(api_model, "type", MODEL_TYPE_API);
        cJSON_AddBoolToObject(api_model, "supported", true);
        cJSON_AddStringToObject(api_model, "description", "External API-based object detection");

        cJSON_AddItemToArray(models_array, api_model);
        model_count++;

        log_info("Added API detection model");
    } else {
        log_error("Failed to create API model JSON object");
    }

    // Add ONVIF detection model (always available)
    cJSON *onvif_model = cJSON_CreateObject();
    if (onvif_model) {
        cJSON_AddStringToObject(onvif_model, "id", "onvif");
        cJSON_AddStringToObject(onvif_model, "name", "ONVIF Motion Events");
        cJSON_AddStringToObject(onvif_model, "path", "onvif");
        cJSON_AddStringToObject(onvif_model, "type", MODEL_TYPE_ONVIF);
        cJSON_AddBoolToObject(onvif_model, "supported", true);
        cJSON_AddStringToObject(onvif_model, "description", "ONVIF camera motion events detection");

        cJSON_AddItemToArray(models_array, onvif_model);
        model_count++;

        log_info("Added ONVIF detection model");
    } else {
        log_error("Failed to create ONVIF model JSON object");
    }

    // Check if models directory exists
    DIR *dir = opendir(models_dir);
    if (!dir) {
        log_warn("Models directory does not exist: %s", models_dir);
        cJSON_AddStringToObject(response, "message", "Models directory does not exist");
        // Still return the built-in models above
        cJSON_AddNumberToObject(response, "count", model_count);
        char *json_str = cJSON_PrintUnformatted(response);
        if (!json_str) {
            log_error("Failed to convert response JSON to string");
            cJSON_Delete(response);
            http_response_set_json_error(res, 500, "Failed to convert response JSON to string");
            return;
        }
        http_response_set_json(res, 200, json_str);
        free(json_str);
        cJSON_Delete(response);
        return;
    }

    // Scan models directory for file-based models
    struct dirent *entry;
    
    while ((entry = readdir(dir)) != NULL) {
        // Skip . and ..
        if (strcmp(entry->d_name, ".") == 0 || strcmp(entry->d_name, "..") == 0) {
            continue;
        }
        
        // Skip directories
        char full_path[MAX_PATH_LENGTH];
        snprintf(full_path, sizeof(full_path), "%s/%s", models_dir, entry->d_name);
        
        struct stat st;
        if (stat(full_path, &st) == 0 && S_ISDIR(st.st_mode)) {
            continue;
        }
        
        // Check if file is a supported model
        bool supported = is_model_supported(full_path);
        const char *model_type = get_model_type(full_path);
        
        // Create model object
        cJSON *model_obj = cJSON_CreateObject();
        if (!model_obj) {
            log_error("Failed to create model JSON object");
            continue;
        }
        
        // Add model properties (id = full_path so the backend can locate the file)
        cJSON_AddStringToObject(model_obj, "id", full_path);
        cJSON_AddStringToObject(model_obj, "name", entry->d_name);
        cJSON_AddStringToObject(model_obj, "path", full_path);
        cJSON_AddStringToObject(model_obj, "type", model_type);
        cJSON_AddBoolToObject(model_obj, "supported", supported);
        
        // Add file size
        if (stat(full_path, &st) == 0) {
            cJSON_AddNumberToObject(model_obj, "size", (double)st.st_size);
        }
        
        // Add model to array
        cJSON_AddItemToArray(models_array, model_obj);
        model_count++;
    }
    
    // Close directory
    closedir(dir);
    
    // Add model count to response
    cJSON_AddNumberToObject(response, "count", model_count);
    
    // Convert to string
    char *json_str = cJSON_PrintUnformatted(response);
    if (!json_str) {
        log_error("Failed to convert response JSON to string");
        cJSON_Delete(response);
        http_response_set_json_error(res, 500, "Failed to convert response JSON to string");
        return;
    }

    // Send response
    http_response_set_json(res, 200, json_str);
    
    // Clean up
    free(json_str);
    cJSON_Delete(response);
    
    log_info("Successfully handled GET /api/detection/models request, found %d models", model_count);
}
