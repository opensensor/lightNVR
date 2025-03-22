#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#include "web/api_handlers_onvif.h"
#include "web/api_handlers.h"
#include "web/mongoose_adapter.h"
#include "core/logger.h"
#include "core/config.h"
#include "video/onvif_discovery.h"
#include "video/stream_manager.h"
#include "../../external/mongoose/mongoose.h"
#include "cJSON.h"

// Global mutex for config access
extern pthread_mutex_t g_config_mutex;

/**
 * @brief Handle GET request for ONVIF discovery status
 */
void mg_handle_get_onvif_discovery_status(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling GET /api/onvif/discovery/status request");
    
    // Create JSON response
    cJSON *root = cJSON_CreateObject();
    if (!root) {
        log_error("Failed to create JSON response");
        mg_send_json_error(c, 500, "Failed to create JSON response");
        return;
    }
    
    // Add discovery status
    cJSON_AddBoolToObject(root, "enabled", g_config.onvif_discovery_enabled);
    cJSON_AddStringToObject(root, "network", g_config.onvif_discovery_network);
    cJSON_AddNumberToObject(root, "interval", g_config.onvif_discovery_interval);
    
    // Convert to string
    char *json_str = cJSON_PrintUnformatted(root);
    cJSON_Delete(root);
    
    if (!json_str) {
        log_error("Failed to generate JSON response");
        mg_send_json_error(c, 500, "Failed to generate JSON response");
        return;
    }
    
    // Send response
    mg_send_json_response(c, 200, json_str);
    
    // Clean up
    free(json_str);
    
    log_info("Successfully handled GET /api/onvif/discovery/status request");
}

/**
 * @brief Handle POST request to start ONVIF discovery
 */
void mg_handle_post_start_onvif_discovery(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling POST /api/onvif/discovery/start request");
    
    // Parse JSON request
    cJSON *root = mg_parse_json_body(hm);
    if (!root) {
        log_error("Invalid JSON request");
        mg_send_json_error(c, 400, "Invalid JSON request");
        return;
    }
    
    // Extract parameters
    cJSON *network_json = cJSON_GetObjectItem(root, "network");
    cJSON *interval_json = cJSON_GetObjectItem(root, "interval");
    
    if (!network_json || !cJSON_IsString(network_json) || 
        !interval_json || !cJSON_IsNumber(interval_json)) {
        cJSON_Delete(root);
        log_error("Missing or invalid parameters");
        mg_send_json_error(c, 400, "Missing or invalid parameters");
        return;
    }
    
    const char *network = network_json->valuestring;
    int interval = interval_json->valueint;
    
    // Validate parameters
    if (strlen(network) == 0 || interval <= 0) {
        cJSON_Delete(root);
        log_error("Invalid parameters");
        mg_send_json_error(c, 400, "Invalid parameters");
        return;
    }
    
    // Start ONVIF discovery
    int result = start_onvif_discovery(network, interval);
    cJSON_Delete(root);
    
    if (result != 0) {
        log_error("Failed to start ONVIF discovery");
        mg_send_json_error(c, 500, "Failed to start ONVIF discovery");
        return;
    }
    
    // Update configuration
    pthread_mutex_lock(&g_config_mutex);
    g_config.onvif_discovery_enabled = true;
    strncpy(g_config.onvif_discovery_network, network, sizeof(g_config.onvif_discovery_network) - 1);
    g_config.onvif_discovery_network[sizeof(g_config.onvif_discovery_network) - 1] = '\0';
    g_config.onvif_discovery_interval = interval;
    pthread_mutex_unlock(&g_config_mutex);
    
    // Save configuration
    save_config(&g_config, get_loaded_config_path());
    
    // Create success response
    cJSON *response_json = cJSON_CreateObject();
    if (!response_json) {
        log_error("Failed to create JSON response");
        mg_send_json_error(c, 500, "Failed to create JSON response");
        return;
    }
    
    cJSON_AddBoolToObject(response_json, "success", true);
    cJSON_AddStringToObject(response_json, "message", "ONVIF discovery started successfully");
    
    // Convert to string
    char *json_str = cJSON_PrintUnformatted(response_json);
    cJSON_Delete(response_json);
    
    if (!json_str) {
        log_error("Failed to generate JSON response");
        mg_send_json_error(c, 500, "Failed to generate JSON response");
        return;
    }
    
    // Send response
    mg_send_json_response(c, 200, json_str);
    
    // Clean up
    free(json_str);
    
    log_info("Successfully handled POST /api/onvif/discovery/start request");
}

/**
 * @brief Handle POST request to stop ONVIF discovery
 */
void mg_handle_post_stop_onvif_discovery(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling POST /api/onvif/discovery/stop request");
    
    // Stop ONVIF discovery
    int result = stop_onvif_discovery();
    
    if (result != 0) {
        log_error("Failed to stop ONVIF discovery");
        mg_send_json_error(c, 500, "Failed to stop ONVIF discovery");
        return;
    }
    
    // Update configuration
    pthread_mutex_lock(&g_config_mutex);
    g_config.onvif_discovery_enabled = false;
    pthread_mutex_unlock(&g_config_mutex);
    
    // Save configuration
    save_config(&g_config, get_loaded_config_path());
    
    // Create success response
    cJSON *response_json = cJSON_CreateObject();
    if (!response_json) {
        log_error("Failed to create JSON response");
        mg_send_json_error(c, 500, "Failed to create JSON response");
        return;
    }
    
    cJSON_AddBoolToObject(response_json, "success", true);
    cJSON_AddStringToObject(response_json, "message", "ONVIF discovery stopped successfully");
    
    // Convert to string
    char *json_str = cJSON_PrintUnformatted(response_json);
    cJSON_Delete(response_json);
    
    if (!json_str) {
        log_error("Failed to generate JSON response");
        mg_send_json_error(c, 500, "Failed to generate JSON response");
        return;
    }
    
    // Send response
    mg_send_json_response(c, 200, json_str);
    
    // Clean up
    free(json_str);
    
    log_info("Successfully handled POST /api/onvif/discovery/stop request");
}

/**
 * @brief Handle GET request for discovered ONVIF devices
 */
void mg_handle_get_discovered_onvif_devices(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling GET /api/onvif/devices request");
    
    // Get discovered devices
    onvif_device_info_t devices[32];
    int count = get_discovered_onvif_devices(devices, 32);
    
    if (count < 0) {
        log_error("Failed to get discovered ONVIF devices");
        mg_send_json_error(c, 500, "Failed to get discovered ONVIF devices");
        return;
    }
    
    // Create JSON response
    cJSON *root = cJSON_CreateObject();
    if (!root) {
        log_error("Failed to create JSON response");
        mg_send_json_error(c, 500, "Failed to create JSON response");
        return;
    }
    
    cJSON *devices_array = cJSON_AddArrayToObject(root, "devices");
    if (!devices_array) {
        log_error("Failed to create JSON response");
        cJSON_Delete(root);
        mg_send_json_error(c, 500, "Failed to create JSON response");
        return;
    }
    
    // Add devices to array
    for (int i = 0; i < count; i++) {
        cJSON *device = cJSON_CreateObject();
        if (!device) {
            log_error("Failed to create JSON response");
            cJSON_Delete(root);
            mg_send_json_error(c, 500, "Failed to create JSON response");
            return;
        }
        
        cJSON_AddStringToObject(device, "endpoint", devices[i].endpoint);
        cJSON_AddStringToObject(device, "device_service", devices[i].device_service);
        cJSON_AddStringToObject(device, "media_service", devices[i].media_service);
        cJSON_AddStringToObject(device, "ptz_service", devices[i].ptz_service);
        cJSON_AddStringToObject(device, "imaging_service", devices[i].imaging_service);
        cJSON_AddStringToObject(device, "manufacturer", devices[i].manufacturer);
        cJSON_AddStringToObject(device, "model", devices[i].model);
        cJSON_AddStringToObject(device, "firmware_version", devices[i].firmware_version);
        cJSON_AddStringToObject(device, "serial_number", devices[i].serial_number);
        cJSON_AddStringToObject(device, "hardware_id", devices[i].hardware_id);
        cJSON_AddStringToObject(device, "ip_address", devices[i].ip_address);
        cJSON_AddStringToObject(device, "mac_address", devices[i].mac_address);
        cJSON_AddNumberToObject(device, "discovery_time", (double)devices[i].discovery_time);
        cJSON_AddBoolToObject(device, "online", devices[i].online);
        
        cJSON_AddItemToArray(devices_array, device);
    }
    
    // Convert to string
    char *json_str = cJSON_PrintUnformatted(root);
    cJSON_Delete(root);
    
    if (!json_str) {
        log_error("Failed to generate JSON response");
        mg_send_json_error(c, 500, "Failed to generate JSON response");
        return;
    }
    
    // Send response
    mg_send_json_response(c, 200, json_str);
    
    // Clean up
    free(json_str);
    
    log_info("Successfully handled GET /api/onvif/devices request");
}

/**
 * @brief Handle POST request to manually discover ONVIF devices
 */
void mg_handle_post_discover_onvif_devices(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling POST /api/onvif/discovery/discover request");
    
    // Parse JSON request
    cJSON *root = mg_parse_json_body(hm);
    if (!root) {
        log_error("Invalid JSON request");
        mg_send_json_error(c, 400, "Invalid JSON request");
        return;
    }
    
    // Extract parameters
    cJSON *network_json = cJSON_GetObjectItem(root, "network");
    
    if (!network_json || !cJSON_IsString(network_json)) {
        cJSON_Delete(root);
        log_error("Missing or invalid parameters");
        mg_send_json_error(c, 400, "Missing or invalid parameters");
        return;
    }
    
    const char *network = network_json->valuestring;
    
    // Validate parameters
    if (strlen(network) == 0) {
        cJSON_Delete(root);
        log_error("Invalid parameters");
        mg_send_json_error(c, 400, "Invalid parameters");
        return;
    }
    
    // Discover ONVIF devices
    onvif_device_info_t devices[32];
    int count = discover_onvif_devices(network, devices, 32);
    cJSON_Delete(root);
    
    if (count < 0) {
        log_error("Failed to discover ONVIF devices");
        mg_send_json_error(c, 500, "Failed to discover ONVIF devices");
        return;
    }
    
    // Create JSON response
    cJSON *response_json = cJSON_CreateObject();
    if (!response_json) {
        log_error("Failed to create JSON response");
        mg_send_json_error(c, 500, "Failed to create JSON response");
        return;
    }
    
    cJSON *devices_array = cJSON_AddArrayToObject(response_json, "devices");
    if (!devices_array) {
        log_error("Failed to create JSON response");
        cJSON_Delete(response_json);
        mg_send_json_error(c, 500, "Failed to create JSON response");
        return;
    }
    
    // Add devices to array
    for (int i = 0; i < count; i++) {
        cJSON *device = cJSON_CreateObject();
        if (!device) {
            log_error("Failed to create JSON response");
            cJSON_Delete(response_json);
            mg_send_json_error(c, 500, "Failed to create JSON response");
            return;
        }
        
        cJSON_AddStringToObject(device, "endpoint", devices[i].endpoint);
        cJSON_AddStringToObject(device, "device_service", devices[i].device_service);
        cJSON_AddStringToObject(device, "media_service", devices[i].media_service);
        cJSON_AddStringToObject(device, "ptz_service", devices[i].ptz_service);
        cJSON_AddStringToObject(device, "imaging_service", devices[i].imaging_service);
        cJSON_AddStringToObject(device, "manufacturer", devices[i].manufacturer);
        cJSON_AddStringToObject(device, "model", devices[i].model);
        cJSON_AddStringToObject(device, "firmware_version", devices[i].firmware_version);
        cJSON_AddStringToObject(device, "serial_number", devices[i].serial_number);
        cJSON_AddStringToObject(device, "hardware_id", devices[i].hardware_id);
        cJSON_AddStringToObject(device, "ip_address", devices[i].ip_address);
        cJSON_AddStringToObject(device, "mac_address", devices[i].mac_address);
        cJSON_AddNumberToObject(device, "discovery_time", (double)devices[i].discovery_time);
        cJSON_AddBoolToObject(device, "online", devices[i].online);
        
        cJSON_AddItemToArray(devices_array, device);
    }
    
    // Convert to string
    char *json_str = cJSON_PrintUnformatted(response_json);
    cJSON_Delete(response_json);
    
    if (!json_str) {
        log_error("Failed to generate JSON response");
        mg_send_json_error(c, 500, "Failed to generate JSON response");
        return;
    }
    
    // Send response
    mg_send_json_response(c, 200, json_str);
    
    // Clean up
    free(json_str);
    
    log_info("Successfully handled POST /api/onvif/discovery/discover request");
}

/**
 * @brief Handle GET request for ONVIF device profiles
 */
void mg_handle_get_onvif_device_profiles(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling GET /api/onvif/device/profiles request");
    
    // Get URL parameters
    struct mg_str *device_url_param = mg_http_get_header(hm, "X-Device-URL");
    struct mg_str *username_param = mg_http_get_header(hm, "X-Username");
    struct mg_str *password_param = mg_http_get_header(hm, "X-Password");
    
    if (!device_url_param) {
        log_error("Missing device_url parameter");
        mg_send_json_error(c, 400, "Missing device_url parameter");
        return;
    }
    
    // Extract parameters
    char device_url[MAX_URL_LENGTH];
    char username[64] = {0};
    char password[64] = {0};
    
    mg_str_copy(device_url_param, device_url, sizeof(device_url));
    
    if (username_param) {
        mg_str_copy(username_param, username, sizeof(username));
    }
    
    if (password_param) {
        mg_str_copy(password_param, password, sizeof(password));
    }
    
    // Get device profiles
    onvif_profile_t profiles[16];
    int count = get_onvif_device_profiles(device_url, 
                                         username[0] ? username : NULL, 
                                         password[0] ? password : NULL, 
                                         profiles, 16);
    
    if (count < 0) {
        log_error("Failed to get ONVIF device profiles");
        mg_send_json_error(c, 500, "Failed to get ONVIF device profiles");
        return;
    }
    
    // Create JSON response
    cJSON *root = cJSON_CreateObject();
    if (!root) {
        log_error("Failed to create JSON response");
        mg_send_json_error(c, 500, "Failed to create JSON response");
        return;
    }
    
    cJSON *profiles_array = cJSON_AddArrayToObject(root, "profiles");
    if (!profiles_array) {
        log_error("Failed to create JSON response");
        cJSON_Delete(root);
        mg_send_json_error(c, 500, "Failed to create JSON response");
        return;
    }
    
    // Add profiles to array
    for (int i = 0; i < count; i++) {
        cJSON *profile = cJSON_CreateObject();
        if (!profile) {
            log_error("Failed to create JSON response");
            cJSON_Delete(root);
            mg_send_json_error(c, 500, "Failed to create JSON response");
            return;
        }
        
        cJSON_AddStringToObject(profile, "token", profiles[i].token);
        cJSON_AddStringToObject(profile, "name", profiles[i].name);
        cJSON_AddStringToObject(profile, "snapshot_uri", profiles[i].snapshot_uri);
        cJSON_AddStringToObject(profile, "stream_uri", profiles[i].stream_uri);
        cJSON_AddNumberToObject(profile, "width", profiles[i].width);
        cJSON_AddNumberToObject(profile, "height", profiles[i].height);
        cJSON_AddStringToObject(profile, "encoding", profiles[i].encoding);
        cJSON_AddNumberToObject(profile, "fps", profiles[i].fps);
        cJSON_AddNumberToObject(profile, "bitrate", profiles[i].bitrate);
        
        cJSON_AddItemToArray(profiles_array, profile);
    }
    
    // Convert to string
    char *json_str = cJSON_PrintUnformatted(root);
    cJSON_Delete(root);
    
    if (!json_str) {
        log_error("Failed to generate JSON response");
        mg_send_json_error(c, 500, "Failed to generate JSON response");
        return;
    }
    
    // Send response
    mg_send_json_response(c, 200, json_str);
    
    // Clean up
    free(json_str);
    
    log_info("Successfully handled GET /api/onvif/device/profiles request");
}

/**
 * @brief Handle POST request to add ONVIF device as stream
 */
void mg_handle_post_add_onvif_device_as_stream(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling POST /api/onvif/device/add request");
    
    // Parse JSON request
    cJSON *root = mg_parse_json_body(hm);
    if (!root) {
        log_error("Invalid JSON request");
        mg_send_json_error(c, 400, "Invalid JSON request");
        return;
    }
    
    // Extract parameters
    cJSON *device_url_json = cJSON_GetObjectItem(root, "device_url");
    cJSON *profile_token_json = cJSON_GetObjectItem(root, "profile_token");
    cJSON *stream_name_json = cJSON_GetObjectItem(root, "stream_name");
    cJSON *username_json = cJSON_GetObjectItem(root, "username");
    cJSON *password_json = cJSON_GetObjectItem(root, "password");
    
    if (!device_url_json || !cJSON_IsString(device_url_json) ||
        !profile_token_json || !cJSON_IsString(profile_token_json) ||
        !stream_name_json || !cJSON_IsString(stream_name_json)) {
        cJSON_Delete(root);
        log_error("Missing or invalid parameters");
        mg_send_json_error(c, 400, "Missing or invalid parameters");
        return;
    }
    
    const char *device_url = device_url_json->valuestring;
    const char *profile_token = profile_token_json->valuestring;
    const char *stream_name = stream_name_json->valuestring;
    const char *username = username_json && cJSON_IsString(username_json) ? username_json->valuestring : NULL;
    const char *password = password_json && cJSON_IsString(password_json) ? password_json->valuestring : NULL;
    
    // Validate parameters
    if (strlen(device_url) == 0 || strlen(profile_token) == 0 || strlen(stream_name) == 0) {
        cJSON_Delete(root);
        log_error("Invalid parameters");
        mg_send_json_error(c, 400, "Invalid parameters");
        return;
    }
    
    // Get device profiles
    onvif_profile_t profiles[16];
    int count = get_onvif_device_profiles(device_url, username, password, profiles, 16);
    
    if (count < 0) {
        cJSON_Delete(root);
        log_error("Failed to get ONVIF device profiles");
        mg_send_json_error(c, 500, "Failed to get ONVIF device profiles");
        return;
    }
    
    // Find the requested profile
    onvif_profile_t *profile = NULL;
    for (int i = 0; i < count; i++) {
        if (strcmp(profiles[i].token, profile_token) == 0) {
            profile = &profiles[i];
            break;
        }
    }
    
    if (!profile) {
        cJSON_Delete(root);
        log_error("Profile not found");
        mg_send_json_error(c, 404, "Profile not found");
        return;
    }
    
    // Create device info
    onvif_device_info_t device_info;
    memset(&device_info, 0, sizeof(device_info));
    strncpy(device_info.device_service, device_url, sizeof(device_info.device_service) - 1);
    
    // Add ONVIF device as stream
    int result = add_onvif_device_as_stream(&device_info, profile, username, password, stream_name);
    cJSON_Delete(root);
    
    if (result != 0) {
        log_error("Failed to add ONVIF device as stream");
        mg_send_json_error(c, 500, "Failed to add ONVIF device as stream");
        return;
    }
    
    // Create success response
    cJSON *response_json = cJSON_CreateObject();
    if (!response_json) {
        log_error("Failed to create JSON response");
        mg_send_json_error(c, 500, "Failed to create JSON response");
        return;
    }
    
    cJSON_AddBoolToObject(response_json, "success", true);
    cJSON_AddStringToObject(response_json, "message", "ONVIF device added as stream successfully");
    cJSON_AddStringToObject(response_json, "stream_name", stream_name);
    
    // Convert to string
    char *json_str = cJSON_PrintUnformatted(response_json);
    cJSON_Delete(response_json);
    
    if (!json_str) {
        log_error("Failed to generate JSON response");
        mg_send_json_error(c, 500, "Failed to generate JSON response");
        return;
    }
    
    // Send response
    mg_send_json_response(c, 200, json_str);
    
    // Clean up
    free(json_str);
    
    log_info("Successfully handled POST /api/onvif/device/add request");
}

/**
 * @brief Handle POST request to test ONVIF connection
 */
void mg_handle_post_test_onvif_connection(struct mg_connection *c, struct mg_http_message *hm) {
    log_info("Handling POST /api/onvif/device/test request");
    
    // Parse JSON request
    cJSON *root = mg_parse_json_body(hm);
    if (!root) {
        log_error("Invalid JSON request");
        mg_send_json_error(c, 400, "Invalid JSON request");
        return;
    }
    
    // Extract parameters
    cJSON *url_json = cJSON_GetObjectItem(root, "url");
    cJSON *username_json = cJSON_GetObjectItem(root, "username");
    cJSON *password_json = cJSON_GetObjectItem(root, "password");
    
    if (!url_json || !cJSON_IsString(url_json)) {
        cJSON_Delete(root);
        log_error("Missing or invalid parameters");
        mg_send_json_error(c, 400, "Missing or invalid parameters");
        return;
    }
    
    const char *url = url_json->valuestring;
    const char *username = username_json && cJSON_IsString(username_json) ? username_json->valuestring : NULL;
    const char *password = password_json && cJSON_IsString(password_json) ? password_json->valuestring : NULL;
    
    // Validate parameters
    if (strlen(url) == 0) {
        cJSON_Delete(root);
        log_error("Invalid parameters");
        mg_send_json_error(c, 400, "Invalid parameters");
        return;
    }
    
    // Test ONVIF connection
    int result = test_onvif_connection(url, username, password);
    cJSON_Delete(root);
    
    if (result != 0) {
        log_error("Failed to connect to ONVIF device");
        mg_send_json_error(c, 500, "Failed to connect to ONVIF device");
        return;
    }
    
    // Create success response
    cJSON *response_json = cJSON_CreateObject();
    if (!response_json) {
        log_error("Failed to create JSON response");
        mg_send_json_error(c, 500, "Failed to create JSON response");
        return;
    }
    
    cJSON_AddBoolToObject(response_json, "success", true);
    cJSON_AddStringToObject(response_json, "message", "Successfully connected to ONVIF device");
    
    // Convert to string
    char *json_str = cJSON_PrintUnformatted(response_json);
    cJSON_Delete(response_json);
    
    if (!json_str) {
        log_error("Failed to generate JSON response");
        mg_send_json_error(c, 500, "Failed to generate JSON response");
        return;
    }
    
    // Send response
    mg_send_json_response(c, 200, json_str);
    
    // Clean up
    free(json_str);
    
    log_info("Successfully handled POST /api/onvif/device/test request");
}

/**
 * @brief Register all ONVIF API handlers
 */
void register_onvif_api_handlers(void) {
    // Register GET handlers
    mg_register_http_endpoint("GET", "/api/onvif/discovery/status", mg_handle_get_onvif_discovery_status);
    mg_register_http_endpoint("GET", "/api/onvif/devices", mg_handle_get_discovered_onvif_devices);
    mg_register_http_endpoint("GET", "/api/onvif/device/profiles", mg_handle_get_onvif_device_profiles);
    
    // Register POST handlers
    mg_register_http_endpoint("POST", "/api/onvif/discovery/start", mg_handle_post_start_onvif_discovery);
    mg_register_http_endpoint("POST", "/api/onvif/discovery/stop", mg_handle_post_stop_onvif_discovery);
    mg_register_http_endpoint("POST", "/api/onvif/discovery/discover", mg_handle_post_discover_onvif_devices);
    mg_register_http_endpoint("POST", "/api/onvif/device/add", mg_handle_post_add_onvif_device_as_stream);
    mg_register_http_endpoint("POST", "/api/onvif/device/test", mg_handle_post_test_onvif_connection);
    
    log_info("Registered ONVIF API handlers");
}
