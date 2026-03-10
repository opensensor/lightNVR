#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <errno.h>
#include <stdint.h>
#include <unistd.h>
#include <sys/stat.h>
#include <curl/curl.h>
#include <cjson/cJSON.h>
#include <pthread.h>

#include "core/logger.h"
#include "core/config.h"
#include "core/curl_init.h"
#include "core/shutdown_coordinator.h"
#include "core/mqtt_client.h"
#include "video/api_detection.h"
#include "video/detection_result.h"
#include "video/stream_manager.h"
#include "video/stream_state.h"
#include "video/zone_filter.h"
#include "video/ffmpeg_utils.h"
#include "database/db_detections.h"
#include "video/go2rtc/go2rtc_snapshot.h"
#include "video/go2rtc/go2rtc_integration.h"

// Global variables
static bool initialized = false;
static pthread_mutex_t curl_mutex = PTHREAD_MUTEX_INITIALIZER;

// Default JPEG quality used for API detection snapshots (range typically 0–100).
#define API_DETECTION_JPEG_QUALITY_DEFAULT 85

// Timeout (in seconds) for API detection HTTP requests.
#define API_DETECTION_TIMEOUT_SECONDS 10L

// Maximum number of bytes to log from the API response, including the null terminator.
#define API_DETECTION_RESPONSE_PREVIEW_LEN 64

// Initial buffer size (in bytes) for CURL responses to reduce realloc churn.
#define API_DETECTION_INITIAL_RESPONSE_BUFFER_SIZE 1024

// ASCII printable character range used when sanitizing response previews.
#define ASCII_PRINTABLE_MIN 32
#define ASCII_PRINTABLE_MAX 126

// Structure to hold memory for curl response
typedef struct {
    char *memory;
    size_t size;
    size_t capacity;
} memory_struct_t;

// Basic validation to ensure the base URL does not contain control characters or spaces.
static bool is_safe_base_url(const char *url) {
    if (url == NULL) {
        return false;
    }

    for (const unsigned char *p = (const unsigned char *)url; *p != '\0'; ++p) {
        if (*p < 32 || *p == 127 || *p == ' ') {
            return false;
        }
    }

    return true;
}

// Set common curl options used across API detection requests.
static void setup_common_curl_options(CURL *handle) {
    if (handle == NULL) {
        return;
    }

    // Prevent curl from using signals (required for multi-threaded apps)
    curl_easy_setopt(handle, CURLOPT_NOSIGNAL, 1L);
    // API detection URLs are admin-configured and commonly point to localhost/private services,
    // so we explicitly disable redirects instead of blocking private address ranges.
    curl_easy_setopt(handle, CURLOPT_FOLLOWLOCATION, 0L);
#if defined(CURLOPT_PROTOCOLS_STR) && defined(CURLOPT_REDIR_PROTOCOLS_STR)
    curl_easy_setopt(handle, CURLOPT_PROTOCOLS_STR, "http,https");
    curl_easy_setopt(handle, CURLOPT_REDIR_PROTOCOLS_STR, "http,https");
#elif defined(CURLOPT_PROTOCOLS) && defined(CURLOPT_REDIR_PROTOCOLS)
    curl_easy_setopt(handle, CURLOPT_PROTOCOLS, CURLPROTO_HTTP | CURLPROTO_HTTPS);
    curl_easy_setopt(handle, CURLOPT_REDIR_PROTOCOLS, CURLPROTO_HTTP | CURLPROTO_HTTPS);
#endif
}

static bool is_api_detection_system_initialized(void) {
    bool is_ready = false;

    pthread_mutex_lock(&curl_mutex);
    is_ready = initialized;
    pthread_mutex_unlock(&curl_mutex);

    return is_ready;
}

static float normalize_api_detection_threshold(float threshold) {
    return (threshold > 0.0f) ? threshold : 0.5f;
}

bool api_detection_should_use_go2rtc_snapshot(const unsigned char *frame_data,
                                              int width,
                                              int height,
                                              int channels,
                                              const char *stream_name) {
    bool has_decoded_frame = frame_data && width > 0 && height > 0 && channels > 0;
    return !has_decoded_frame && stream_name && stream_name[0] != '\0';
}

// Sanitize backend parameter to avoid breaking URL/query structure.
// Allows only [A-Za-z0-9_-]; falls back to "onnx" if invalid.
static const char *sanitize_backend(const char *backend) {
    static const char *default_backend = "onnx";

    if (backend == NULL || backend[0] == '\0') {
        return default_backend;
    }

    for (const char *p = backend; *p != '\0'; ++p) {
        char c = *p;
        if (!((c >= 'A' && c <= 'Z') ||
              (c >= 'a' && c <= 'z') ||
              (c >= '0' && c <= '9') ||
              c == '_' || c == '-')) {
            log_warn("API Detection: Invalid character '%c' in backend '%s', using default '%s' instead.",
                     c, backend, default_backend);
            return default_backend;
        }
    }

    return backend;
}

static bool validate_api_detection_base_url(const char *base_url, const char *context) {
    if (base_url == NULL) {
        log_error("%s: API URL is NULL.", context);
        return false;
    }

    if (!is_safe_base_url(base_url)) {
        log_error("%s: Invalid API URL '%s' (contains spaces or control characters).", context, base_url);
        return false;
    }

    if (strncmp(base_url, "http://", 7) != 0 && strncmp(base_url, "https://", 8) != 0) {
        log_error("%s: Invalid URL format: %s (must start with http:// or https://)", context, base_url);
        return false;
    }

    const char *authority_start = strstr(base_url, "://");
    if (authority_start == NULL) {
        log_error("%s: Invalid API URL '%s' (missing scheme separator).", context, base_url);
        return false;
    }
    authority_start += 3;

    const char *authority_end = authority_start;
    while (*authority_end != '\0' && *authority_end != '/' && *authority_end != '?' && *authority_end != '#') {
        authority_end++;
    }
    if (authority_end == authority_start) {
        log_error("%s: Invalid API URL '%s' (missing host).", context, base_url);
        return false;
    }
    if (memchr(authority_start, '@', (size_t)(authority_end - authority_start)) != NULL) {
        log_error("%s: Invalid API URL '%s' (userinfo is not allowed).", context, base_url);
        return false;
    }

    const char *first_qmark = strchr(base_url, '?');
    const char *second_qmark = NULL;
    if (first_qmark != NULL) {
        second_qmark = strchr(first_qmark + 1, '?');
    }

    if (strchr(base_url, '#') != NULL || second_qmark != NULL) {
        log_error("%s: Invalid API URL '%s' (contains fragment or multiple '?').", context, base_url);
        return false;
    }

    return true;
}

// Helper to build the API detection URL with common query parameters.
// Returns 0 on success, -1 on error (e.g., buffer too small or invalid args).
static int build_api_detection_url(char *buffer,
                                   size_t buffer_size,
                                   const char *base_url,
                                   const char *backend,
                                   float threshold,
                                   bool return_image_flag) {
    if (buffer == NULL || buffer_size == 0 || base_url == NULL || !is_safe_base_url(base_url)) {
        return -1;
    }

    const char *backend_param = sanitize_backend(backend);
    float actual_threshold = normalize_api_detection_threshold(threshold);
    const char *return_image_value = return_image_flag ? "true" : "false";
    char separator = (strchr(base_url, '?') != NULL) ? '&' : '?';

    int url_len = snprintf(buffer,
                           buffer_size,
                           "%s%cbackend=%s&confidence_threshold=%.2f&return_image=%s",
                           base_url,
                           separator,
                           backend_param,
                           actual_threshold,
                           return_image_value);
    if (url_len < 0 || (size_t)url_len >= buffer_size) {
        return -1;
    }

    return 0;
}

// Callback function for curl to write data
static size_t write_memory_callback(void *contents, size_t size, size_t nmemb, void *userp) {
    if (nmemb != 0 && size > (SIZE_MAX / nmemb)) {
        log_error("Not enough memory for curl response (size overflow)");
        return 0;
    }

    size_t realsize = size * nmemb;
    memory_struct_t *mem = (memory_struct_t *)userp;

    if (realsize > SIZE_MAX - mem->size - 1) {
        log_error("Not enough memory for curl response (size overflow)");
        return 0;
    }

    size_t required_size = mem->size + realsize + 1;
    if (required_size > mem->capacity) {
        size_t new_capacity = mem->capacity > 0 ? mem->capacity : API_DETECTION_INITIAL_RESPONSE_BUFFER_SIZE;
        while (new_capacity < required_size) {
            if (new_capacity > (SIZE_MAX / 2)) {
                new_capacity = required_size;
                break;
            }
            new_capacity *= 2;
        }

        char *new_memory = realloc(mem->memory, new_capacity);
        if (new_memory == NULL) {
            log_error("Not enough memory for curl response");
            return 0;
        }

        mem->memory = new_memory;
        mem->capacity = new_capacity;
    }

    memcpy(&(mem->memory[mem->size]), contents, realsize);
    mem->size += realsize;
    mem->memory[mem->size] = 0;

    return realsize;
}

static bool is_tls_ca_error(CURLcode res) {
#ifdef CURLE_SSL_CACERT_BADFILE
    if (res == CURLE_SSL_CACERT_BADFILE) {
        return true;
    }
#endif
#ifdef CURLE_SSL_CACERT
    if (res == CURLE_SSL_CACERT) {
        return true;
    }
#endif
#ifdef CURLE_PEER_FAILED_VERIFICATION
    if (res == CURLE_PEER_FAILED_VERIFICATION) {
        return true;
    }
#endif
    return false;
}

static void log_tls_error_details(const char *context, CURL *curl, CURLcode res, const char *url) {
    if (!is_tls_ca_error(res)) {
        return;
    }

    long ssl_verify_result = 0;
    if (curl) {
        curl_easy_getinfo(curl, CURLINFO_SSL_VERIFYRESULT, &ssl_verify_result);
    }

    const char *ssl_cert_file = getenv("SSL_CERT_FILE");
    const char *ssl_cert_dir = getenv("SSL_CERT_DIR");

    log_error("%s: TLS certificate verification failed for %s", context, url ? url : "(unknown URL)");
    log_error("%s: libcurl SSL verify result=%ld, SSL_CERT_FILE=%s, SSL_CERT_DIR=%s",
              context,
              ssl_verify_result,
              (ssl_cert_file && ssl_cert_file[0] != '\0') ? ssl_cert_file : "(unset)",
              (ssl_cert_dir && ssl_cert_dir[0] != '\0') ? ssl_cert_dir : "(unset)");
    log_error("%s: Ensure a readable CA bundle is installed (for containers, install ca-certificates) or configure SSL_CERT_FILE/SSL_CERT_DIR to valid paths.",
              context);
}

/**
 * Initialize the API detection system
 */
int init_api_detection_system(void) {
    pthread_mutex_lock(&curl_mutex);

    if (initialized) {
        pthread_mutex_unlock(&curl_mutex);
        log_info("API detection system already initialized");
        return 0;
    }

    // Initialize curl global (thread-safe, idempotent)
    if (curl_init_global() != 0) {
        pthread_mutex_unlock(&curl_mutex);
        log_error("Failed to initialize curl global");
        return -1;
    }

    initialized = true;
    pthread_mutex_unlock(&curl_mutex);

    log_info("API detection system initialized successfully");
    return 0;
}

/**
 * Shutdown the API detection system
 */
void shutdown_api_detection_system(void) {
    bool was_initialized = false;

    pthread_mutex_lock(&curl_mutex);
    was_initialized = initialized;
    initialized = false;
    pthread_mutex_unlock(&curl_mutex);

    // Always attempt to clean up resources, even if not marked as initialized.
    log_info("Shutting down API detection system (initialized: %s)",
             was_initialized ? "yes" : "no");

    /*
     * Cleanup cached JPEG encoders used for API detection snapshots.
     *
     * jpeg_encoder_cleanup_all() releases any process-wide encoder instances
     * and associated buffers that may have been cached for performance during
     * detection. This prevents a persistent memory footprint across repeated
     * init/shutdown cycles of the API detection system.
     *
     * It is safe to call multiple times, but it must be done as part of the
     * shutdown sequence to ensure all encoder resources are freed once API
     * detection is no longer in use.
     */
    jpeg_encoder_cleanup_all();

    // Note: Don't call curl_global_cleanup() here - it's managed centrally in curl_init.c
    // The global cleanup will happen at program shutdown

    log_info("API detection system shutdown complete");
}

/**
 * Detect objects using the API with go2rtc snapshot
 */
int detect_objects_api(const char *api_url, const unsigned char *frame_data,
                      int width, int height, int channels, detection_result_t *result,
                      const char *stream_name, float threshold, uint64_t recording_id) {
    // Check if we're in shutdown mode or if the stream has been stopped.
    if (is_shutdown_initiated()) {
        log_info("API Detection: System shutdown in progress, skipping detection");
        return -1;
    }

    // Initialize result to empty at the beginning to prevent segmentation faults.
    if (result) {
        memset(result, 0, sizeof(detection_result_t));
    } else {
        log_error("API Detection: NULL result pointer provided");
        return -1;
    }

    // Check if api_url is the special "api-detection" string.
    // If so, get the actual URL from the global config.
    const char *actual_api_url = api_url;
    if (api_url && strcmp(api_url, "api-detection") == 0) {
        actual_api_url = g_config.api_detection_url;
        log_info("API Detection: Using API URL from config: %s", actual_api_url ? actual_api_url : "NULL");
    }

    log_info("API Detection: Starting detection with API URL: %s", actual_api_url);
    log_info("API Detection: Stream name: %s", stream_name ? stream_name : "NULL");

    if (!is_api_detection_system_initialized()) {
        log_error("API detection system not initialized");
        return -1;
    }

    if (!validate_api_detection_base_url(actual_api_url, "API Detection")) {
        return -1;
    }

    // Use go2rtc to get a JPEG snapshot directly only when we do not already
    // have a decoded frame. This avoids re-entering the go2rtc snapshot path
    // during fallback flows that already decoded a local frame.
    unsigned char *jpeg_data = NULL;
    size_t jpeg_size = 0;
    CURL *local_curl = NULL;
    curl_mime *mime = NULL;
    curl_mimepart *part = NULL;
    memory_struct_t chunk = {0};
    struct curl_slist *headers = NULL;
    cJSON *root = NULL;
    int ret = -1;
    bool go2rtc_initialized = false;
    bool snapshot_ok = false;

    if (api_detection_should_use_go2rtc_snapshot(frame_data, width, height, channels, stream_name)) {
        go2rtc_initialized = go2rtc_integration_is_initialized();
        if (go2rtc_initialized) {
            snapshot_ok = go2rtc_get_snapshot(stream_name, &jpeg_data, &jpeg_size);
        }
    }

    if (snapshot_ok) {
        log_info("API Detection: Successfully fetched snapshot from go2rtc: %zu bytes", jpeg_size);
    } else {
        if (!stream_name || stream_name[0] == '\0') {
            log_debug("API Detection: No stream name provided for go2rtc snapshot, using cached JPEG encoding");
        } else if (!go2rtc_initialized) {
            log_debug("API Detection: go2rtc not initialized, using cached JPEG encoding");
        } else {
            log_warn("API Detection: Failed to get snapshot from go2rtc, falling back to cached JPEG encoding");
        }

        // FALLBACK: Use cached JPEG encoder to encode raw frame to JPEG in memory.
        // The cache is keyed by (width, height, channels, quality) so encoders are only
        // reused when the frame characteristics and JPEG quality match. The underlying
        // AVCodecContext is kept alive and reused to avoid recreating it on every call.
        //
        // Thread-safety / lifetime notes:
        // - jpeg_encoder_get_cached() and jpeg_encoder_cache_encode_to_memory() are
        //   synchronized internally by the encoder cache implementation.
        // - Encoders remain cached for the lifetime of the process (or until an explicit
        //   cache-clear in the encoder module); there is no per-call teardown here.
        jpeg_encoder_cache_t *encoder = jpeg_encoder_get_cached(width, height, channels, API_DETECTION_JPEG_QUALITY_DEFAULT);
        if (!encoder) {
            log_error("API Detection: Failed to get cached JPEG encoder");
            goto cleanup;
        }

        // Encode directly to memory - no temp file needed
        int encode_result = jpeg_encoder_cache_encode_to_memory(encoder, frame_data, &jpeg_data, &jpeg_size);
        if (encode_result != 0) {
            log_error("API Detection: Failed to encode frame to JPEG using cached encoder");
            goto cleanup;
        }

        log_info("API Detection: Encoded frame to JPEG using cached encoder: %zu bytes", jpeg_size);
    }

    // Validate JPEG data.
    if (!jpeg_data || jpeg_size == 0) {
        log_error("API Detection: No JPEG data available");
        goto cleanup;
    }

    // Use a per-call curl handle so detection requests can run concurrently.
    local_curl = curl_easy_init();
    if (local_curl == NULL) {
        log_error("API Detection: Failed to initialize CURL handle");
        goto cleanup;
    }

    // Set up curl for multipart/form-data using the modern mime API.
    // Note: curl_mime_* replaced deprecated curl_formadd in libcurl 7.56.0.
    mime = curl_mime_init(local_curl);
    if (!mime) {
        log_error("API Detection: Failed to create mime structure");
        goto cleanup;
    }

    part = curl_mime_addpart(mime);
    if (!part) {
        log_error("API Detection: Failed to add mime part");
        goto cleanup;
    }

    CURLcode mime_result;
    mime_result = curl_mime_name(part, "file");
    if (mime_result != CURLE_OK) {
        log_error("API Detection: Failed to set mime name: %s", curl_easy_strerror(mime_result));
        goto cleanup;
    }

    // Use curl_mime_data to pass data directly from memory (CURL_ZERO_TERMINATED not used; we pass size).
    // curl_mime_data copies the data, so jpeg_data can be freed after this call.
    mime_result = curl_mime_data(part, (const char *)jpeg_data, jpeg_size);
    if (mime_result != CURLE_OK) {
        log_error("API Detection: Failed to set mime data: %s", curl_easy_strerror(mime_result));
        goto cleanup;
    }

    mime_result = curl_mime_filename(part, "snapshot.jpg");
    if (mime_result != CURLE_OK) {
        log_error("API Detection: Failed to set mime filename: %s", curl_easy_strerror(mime_result));
        goto cleanup;
    }

    mime_result = curl_mime_type(part, "image/jpeg");
    if (mime_result != CURLE_OK) {
        log_error("API Detection: Failed to set mime type: %s", curl_easy_strerror(mime_result));
        goto cleanup;
    }

    // Free the JPEG data now that curl has copied it.
    free(jpeg_data);
    jpeg_data = NULL;

    log_info("API Detection: Successfully added JPEG data to form (%zu bytes)", jpeg_size);

    const char *backend = g_config.api_detection_backend;
    const char *safe_backend = sanitize_backend(backend);
    float actual_threshold = normalize_api_detection_threshold(threshold);

    char url_with_params[1024];
    if (build_api_detection_url(url_with_params,
                                sizeof(url_with_params),
                                actual_api_url,
                                backend,
                                threshold,
                                false) != 0) {
        log_error("API Detection: Failed to construct URL with parameters.");
        goto cleanup;
    }
    log_info("API Detection: Using URL with parameters: %s (backend: %s, threshold: %.2f)",
             url_with_params, safe_backend, actual_threshold);

    curl_easy_setopt(local_curl, CURLOPT_URL, url_with_params);
    curl_easy_setopt(local_curl, CURLOPT_MIMEPOST, mime);

    headers = curl_slist_append(headers, "accept: application/json");
    curl_easy_setopt(local_curl, CURLOPT_HTTPHEADER, headers);

    chunk.memory = malloc(API_DETECTION_INITIAL_RESPONSE_BUFFER_SIZE);
    if (chunk.memory == NULL) {
        log_error("API Detection: Failed to allocate memory for curl response buffer");
        goto cleanup;
    }
    chunk.size = 0;
    chunk.capacity = API_DETECTION_INITIAL_RESPONSE_BUFFER_SIZE;
    chunk.memory[0] = '\0';

    curl_easy_setopt(local_curl, CURLOPT_WRITEFUNCTION, write_memory_callback);
    curl_easy_setopt(local_curl, CURLOPT_WRITEDATA, (void *)&chunk);

    curl_easy_setopt(local_curl, CURLOPT_TIMEOUT, API_DETECTION_TIMEOUT_SECONDS);
    setup_common_curl_options(local_curl);

    log_info("API Detection: Sending request to %s", url_with_params);

    CURLcode res = curl_easy_perform(local_curl);

    if (res != CURLE_OK) {
        log_error("API Detection: curl_easy_perform() failed: %s", curl_easy_strerror(res));
        log_tls_error_details("API Detection", local_curl, res, url_with_params);

        if (res == CURLE_COULDNT_CONNECT) {
            log_error("API Detection: Could not connect to server at %s. Is the API server running?", url_with_params);
        } else if (res == CURLE_OPERATION_TIMEDOUT) {
            log_error("API Detection: Connection to %s timed out. Server might be slow or unreachable.", url_with_params);
        } else if (res == CURLE_COULDNT_RESOLVE_HOST) {
            log_error("API Detection: Could not resolve host %s. Check your network connection and DNS settings.", url_with_params);
        }

        goto cleanup;
    }

    long http_code = 0;
    curl_easy_getinfo(local_curl, CURLINFO_RESPONSE_CODE, &http_code);

    if (http_code != 200) {
        log_error("API request failed with HTTP code %ld", http_code);
        goto cleanup;
    }

    if (!chunk.memory || chunk.size == 0) {
        log_error("API Detection: Empty response from server");
        goto cleanup;
    }

    char preview[API_DETECTION_RESPONSE_PREVIEW_LEN] = {0};
    int preview_len = (int)(chunk.size < (API_DETECTION_RESPONSE_PREVIEW_LEN - 1)
                                ? chunk.size
                                : (API_DETECTION_RESPONSE_PREVIEW_LEN - 1));
    memcpy(preview, chunk.memory, preview_len);
    preview[preview_len] = '\0';
    // Replace non-printable characters with dots
    for (int i = 0; i < preview_len; i++) {
        if (preview[i] < ASCII_PRINTABLE_MIN || preview[i] > ASCII_PRINTABLE_MAX) {
            preview[i] = '.';
        }
    }
    log_info("API Detection: Response preview: %s", preview);

    root = cJSON_Parse(chunk.memory);

    if (!root) {
        const char *error_ptr = cJSON_GetErrorPtr();
        log_error("Failed to parse JSON response: %s", error_ptr ? error_ptr : "Unknown error");
        log_error("API Detection: Response size: %zu bytes", chunk.size);
        log_error("API Detection: Response preview: %s", preview);
        goto cleanup;
    }

    cJSON *detections = cJSON_GetObjectItem(root, "detections");
    if (!detections || !cJSON_IsArray(detections)) {
        log_error("Invalid JSON response: missing or invalid 'detections' array");
        char *json_str = cJSON_Print(root);
        if (json_str) {
            log_error("API Detection: Full JSON response: %s", json_str);
            free(json_str);
        }
        goto cleanup;
    }

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
            x_min = cJSON_GetObjectItem(bounding_box, "x_min");
            y_min = cJSON_GetObjectItem(bounding_box, "y_min");
            x_max = cJSON_GetObjectItem(bounding_box, "x_max");
            y_max = cJSON_GetObjectItem(bounding_box, "y_max");
            log_info("API Detection: Found bounding_box object in JSON response");
        } else {
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
        result->detections[result->count].confidence = (float)confidence->valuedouble;
        result->detections[result->count].x = (float)x_min->valuedouble;
        result->detections[result->count].y = (float)y_min->valuedouble;
        result->detections[result->count].width = (float)(x_max->valuedouble - x_min->valuedouble);
        result->detections[result->count].height = (float)(y_max->valuedouble - y_min->valuedouble);

        // Parse optional track_id field
        cJSON *track_id = cJSON_GetObjectItem(detection, "track_id");
        if (track_id && cJSON_IsNumber(track_id)) {
            result->detections[result->count].track_id = (int)track_id->valuedouble;
        } else {
            result->detections[result->count].track_id = -1; // No tracking
        }

        // Parse optional zone_id field
        cJSON *zone_id = cJSON_GetObjectItem(detection, "zone_id");
        if (zone_id && cJSON_IsString(zone_id)) {
            strncpy(result->detections[result->count].zone_id, zone_id->valuestring, MAX_ZONE_ID_LENGTH - 1);
            result->detections[result->count].zone_id[MAX_ZONE_ID_LENGTH - 1] = '\0';
        } else {
            result->detections[result->count].zone_id[0] = '\0'; // Empty zone
        }

        result->count++;
    }

    // Filter detections by zones before storing
    if (stream_name && stream_name[0] != '\0') {
        log_info("API Detection: Filtering %d detections by zones for stream %s", result->count, stream_name);
        int filter_ret = filter_detections_by_zones(stream_name, result);
        if (filter_ret != 0) {
            log_error("Failed to filter detections by zones, aborting detection pipeline for this frame");
            goto cleanup;
        }

        filter_detections_by_stream_objects(stream_name, result);

        time_t timestamp = time(NULL);
        store_detections_in_db(stream_name, result, timestamp, recording_id);

        if (result->count > 0) {
            mqtt_publish_detection(stream_name, result, timestamp);
            mqtt_set_motion_state(stream_name, result);
        }
    } else {
        log_warn("No stream name provided, skipping database storage");
    }

    ret = 0;

cleanup:
    cJSON_Delete(root);
    free(chunk.memory);
    curl_mime_free(mime);
    curl_slist_free_all(headers);
    curl_easy_cleanup(local_curl);
    free(jpeg_data);

    if (ret != 0) {
        result->count = 0;
    }

    return ret;
}

/**
 * Detect objects using the API with go2rtc snapshot only (no frame data required)
 *
 * This function fetches a snapshot directly from go2rtc and sends it to the detection API.
 * It does NOT require decoded frame data, which saves significant memory by avoiding
 * the need to decode video segments.
 *
 * Returns: 0 on success, -1 on general failure, -2 if go2rtc snapshot failed
 */
int detect_objects_api_snapshot(const char *api_url, const char *stream_name,
                                detection_result_t *result, float threshold, uint64_t recording_id) {
    // Check if we're in shutdown mode
    if (is_shutdown_initiated()) {
        log_info("API Detection (snapshot): System shutdown in progress, skipping detection");
        return -1;
    }

    // Stream name is required for go2rtc snapshot
    if (!stream_name || stream_name[0] == '\0') {
        log_error("API Detection (snapshot): Stream name is required");
        return -1;
    }

    // Initialize result
    if (result) {
        memset(result, 0, sizeof(detection_result_t));
    } else {
        log_error("API Detection (snapshot): NULL result pointer provided");
        return -1;
    }

    // Handle "api-detection" special string
    const char *actual_api_url = api_url;
    if (api_url && strcmp(api_url, "api-detection") == 0) {
        actual_api_url = g_config.api_detection_url;
        log_info("API Detection (snapshot): Using API URL from config: %s", actual_api_url ? actual_api_url : "NULL");
    }

    if (!is_api_detection_system_initialized()) {
        log_error("API detection system not initialized");
        return -1;
    }

    if (!validate_api_detection_base_url(actual_api_url, "API Detection (snapshot)")) {
        return -1;
    }

    // Try to get snapshot from go2rtc (only if go2rtc is initialized)
    unsigned char *jpeg_data = NULL;
    size_t jpeg_size = 0;

    if (!go2rtc_integration_is_initialized()) {
        log_debug("API Detection (snapshot): go2rtc not initialized, skipping snapshot for stream %s", stream_name);
        return -2;  // Special return code: go2rtc not available, caller should fall back
    }

    if (!go2rtc_get_snapshot(stream_name, &jpeg_data, &jpeg_size)) {
        log_warn("API Detection (snapshot): Failed to get snapshot from go2rtc for stream %s", stream_name);
        return -2;  // Special return code: go2rtc failed, caller should fall back
    }

    log_info("API Detection (snapshot): Successfully fetched snapshot from go2rtc: %zu bytes", jpeg_size);

    // Validate JPEG data
    if (!jpeg_data || jpeg_size == 0) {
        log_error("API Detection (snapshot): No JPEG data available");
        if (jpeg_data) free(jpeg_data);
        return -2;
    }

    // Create a per-call curl handle to allow parallel requests from multiple threads
    // This avoids the global mutex bottleneck that was serializing all detection calls
    CURL *local_curl = curl_easy_init();
    if (!local_curl) {
        log_error("API Detection (snapshot): Failed to create curl handle");
        free(jpeg_data);
        return -1;
    }

    // Set up curl for multipart/form-data
    curl_mime *mime = NULL;
    curl_mimepart *part = NULL;
    memory_struct_t chunk = {0};
    chunk.memory = NULL;
    struct curl_slist *headers = NULL;

    // Create the mime structure
    mime = curl_mime_init(local_curl);
    if (!mime) {
        log_error("API Detection (snapshot): Failed to create mime structure");
        free(jpeg_data);
        curl_easy_cleanup(local_curl);
        return -1;
    }

    // Add the file part
    part = curl_mime_addpart(mime);
    if (!part) {
        log_error("API Detection (snapshot): Failed to add mime part");
        curl_mime_free(mime);
        free(jpeg_data);
        curl_easy_cleanup(local_curl);
        return -1;
    }

    // Set up the mime part
    CURLcode mime_result;
    mime_result = curl_mime_name(part, "file");
    if (mime_result != CURLE_OK) {
        log_error("API Detection (snapshot): curl_mime_name failed: %s",
                  curl_easy_strerror(mime_result));
        curl_mime_free(mime);
        free(jpeg_data);
        curl_easy_cleanup(local_curl);
        return -1;
    }

    mime_result = curl_mime_data(part, (const char *)jpeg_data, jpeg_size);
    if (mime_result != CURLE_OK) {
        log_error("API Detection (snapshot): curl_mime_data failed: %s",
                  curl_easy_strerror(mime_result));
        curl_mime_free(mime);
        free(jpeg_data);
        curl_easy_cleanup(local_curl);
        return -1;
    }

    mime_result = curl_mime_filename(part, "snapshot.jpg");
    if (mime_result != CURLE_OK) {
        log_error("API Detection (snapshot): curl_mime_filename failed: %s",
                  curl_easy_strerror(mime_result));
        curl_mime_free(mime);
        free(jpeg_data);
        curl_easy_cleanup(local_curl);
        return -1;
    }

    mime_result = curl_mime_type(part, "image/jpeg");
    if (mime_result != CURLE_OK) {
        log_error("API Detection (snapshot): curl_mime_type failed: %s",
                  curl_easy_strerror(mime_result));
        curl_mime_free(mime);
        free(jpeg_data);
        curl_easy_cleanup(local_curl);
        return -1;
    }

    // Free JPEG data now that curl has copied it
    free(jpeg_data);
    jpeg_data = NULL;

    const char *backend = g_config.api_detection_backend;
    const char *sanitized_backend = sanitize_backend(backend);
    float actual_threshold = normalize_api_detection_threshold(threshold);

    char url_with_params[1024];
    if (build_api_detection_url(url_with_params,
                                sizeof(url_with_params),
                                actual_api_url,
                                backend,
                                threshold,
                                false) != 0) {
        log_error("API Detection (snapshot): URL too long when constructing request");
        curl_mime_free(mime);
        curl_slist_free_all(headers);
        curl_easy_cleanup(local_curl);
        return -1;
    }

    log_info("API Detection (snapshot): Sending request to %s (backend: %s, threshold: %.2f)",
             url_with_params,
             sanitized_backend,
             actual_threshold);

    // Set up the request
    curl_easy_setopt(local_curl, CURLOPT_URL, url_with_params);
    curl_easy_setopt(local_curl, CURLOPT_MIMEPOST, mime);

    headers = curl_slist_append(headers, "accept: application/json");
    curl_easy_setopt(local_curl, CURLOPT_HTTPHEADER, headers);

    chunk.memory = malloc(API_DETECTION_INITIAL_RESPONSE_BUFFER_SIZE);
    if (chunk.memory == NULL) {
        log_error("API Detection (snapshot): Failed to allocate memory for curl response buffer");
        curl_mime_free(mime);
        curl_slist_free_all(headers);
        curl_easy_cleanup(local_curl);
        return -1;
    }
    chunk.size = 0;
    chunk.capacity = API_DETECTION_INITIAL_RESPONSE_BUFFER_SIZE;
    chunk.memory[0] = '\0';

    curl_easy_setopt(local_curl, CURLOPT_WRITEFUNCTION, write_memory_callback);
    curl_easy_setopt(local_curl, CURLOPT_WRITEDATA, (void *)&chunk);
    curl_easy_setopt(local_curl, CURLOPT_TIMEOUT, API_DETECTION_TIMEOUT_SECONDS);
    setup_common_curl_options(local_curl);

    // Perform the request
    CURLcode res = curl_easy_perform(local_curl);

    if (res != CURLE_OK) {
        log_error("API Detection (snapshot): curl_easy_perform() failed: %s", curl_easy_strerror(res));
        log_tls_error_details("API Detection (snapshot)", local_curl, res, url_with_params);
        free(chunk.memory);
        curl_mime_free(mime);
        curl_slist_free_all(headers);
        curl_easy_cleanup(local_curl);
        return -1;
    }

    // Check HTTP response code
    long http_code = 0;
    curl_easy_getinfo(local_curl, CURLINFO_RESPONSE_CODE, &http_code);

    if (http_code != 200) {
        log_error("API Detection (snapshot): HTTP error %ld", http_code);
        free(chunk.memory);
        curl_mime_free(mime);
        curl_slist_free_all(headers);
        curl_easy_cleanup(local_curl);
        return -1;
    }

    // Parse JSON response
    if (!chunk.memory || chunk.size == 0) {
        log_error("API Detection (snapshot): Empty response");
        free(chunk.memory);
        curl_mime_free(mime);
        curl_slist_free_all(headers);
        curl_easy_cleanup(local_curl);
        return -1;
    }

    cJSON *root = cJSON_Parse(chunk.memory);
    if (!root) {
        log_error("API Detection (snapshot): Failed to parse JSON response");
        free(chunk.memory);
        curl_mime_free(mime);
        curl_slist_free_all(headers);
        curl_easy_cleanup(local_curl);
        return -1;
    }

    // Extract detections
    cJSON *detections = cJSON_GetObjectItem(root, "detections");
    if (!detections || !cJSON_IsArray(detections)) {
        log_error("API Detection (snapshot): Invalid JSON response");
        cJSON_Delete(root);
        free(chunk.memory);
        curl_mime_free(mime);
        curl_slist_free_all(headers);
        curl_easy_cleanup(local_curl);
        return -1;
    }

    // Process each detection
    int array_size = cJSON_GetArraySize(detections);
    for (int i = 0; i < array_size; i++) {
        if (result->count >= MAX_DETECTIONS) {
            log_warn("API Detection (snapshot): Maximum detections reached");
            break;
        }

        cJSON *detection = cJSON_GetArrayItem(detections, i);
        if (!detection) continue;

        cJSON *label = cJSON_GetObjectItem(detection, "label");
        cJSON *confidence = cJSON_GetObjectItem(detection, "confidence");

        cJSON *bounding_box = cJSON_GetObjectItem(detection, "bounding_box");
        cJSON *x_min = NULL, *y_min = NULL, *x_max = NULL, *y_max = NULL;

        if (bounding_box) {
            x_min = cJSON_GetObjectItem(bounding_box, "x_min");
            y_min = cJSON_GetObjectItem(bounding_box, "y_min");
            x_max = cJSON_GetObjectItem(bounding_box, "x_max");
            y_max = cJSON_GetObjectItem(bounding_box, "y_max");
        } else {
            x_min = cJSON_GetObjectItem(detection, "x_min");
            y_min = cJSON_GetObjectItem(detection, "y_min");
            x_max = cJSON_GetObjectItem(detection, "x_max");
            y_max = cJSON_GetObjectItem(detection, "y_max");
        }

        if (!label || !cJSON_IsString(label) ||
            !confidence || !cJSON_IsNumber(confidence) ||
            !x_min || !cJSON_IsNumber(x_min) ||
            !y_min || !cJSON_IsNumber(y_min) ||
            !x_max || !cJSON_IsNumber(x_max) ||
            !y_max || !cJSON_IsNumber(y_max)) {
            continue;
        }

        // Add detection to result
        strncpy(result->detections[result->count].label, label->valuestring, MAX_LABEL_LENGTH - 1);
        result->detections[result->count].label[MAX_LABEL_LENGTH - 1] = '\0';
        result->detections[result->count].confidence = (float)confidence->valuedouble;
        result->detections[result->count].x = (float)x_min->valuedouble;
        result->detections[result->count].y = (float)y_min->valuedouble;
        result->detections[result->count].width = (float)(x_max->valuedouble - x_min->valuedouble);
        result->detections[result->count].height = (float)(y_max->valuedouble - y_min->valuedouble);

        cJSON *track_id = cJSON_GetObjectItem(detection, "track_id");
        result->detections[result->count].track_id = (track_id && cJSON_IsNumber(track_id))
            ? (int)track_id->valuedouble : -1;

        cJSON *zone_id = cJSON_GetObjectItem(detection, "zone_id");
        if (zone_id && cJSON_IsString(zone_id)) {
            strncpy(result->detections[result->count].zone_id, zone_id->valuestring, MAX_ZONE_ID_LENGTH - 1);
            result->detections[result->count].zone_id[MAX_ZONE_ID_LENGTH - 1] = '\0';
        } else {
            result->detections[result->count].zone_id[0] = '\0';
        }

        result->count++;
    }

    // Filter by zones and store in database
    if (stream_name && stream_name[0] != '\0') {
        log_info("API Detection (snapshot): Filtering %d detections by zones for stream %s",
                 result->count, stream_name);
        if (filter_detections_by_zones(stream_name, result) != 0) {
            log_error("API Detection (snapshot): Failed to filter detections by zones for stream %s",
                      stream_name);

            // Clean up on error to avoid leaking resources
            cJSON_Delete(root);
            free(chunk.memory);
            curl_mime_free(mime);
            curl_slist_free_all(headers);
            curl_easy_cleanup(local_curl);

            return -1;
        }

        // Filter detections by per-stream object include/exclude lists
        filter_detections_by_stream_objects(stream_name, result);

        time_t timestamp = time(NULL);
        store_detections_in_db(stream_name, result, timestamp, recording_id);

        // Publish to MQTT if enabled
        if (result->count > 0) {
            mqtt_publish_detection(stream_name, result, timestamp);
            mqtt_set_motion_state(stream_name, result);
        }
    }

    // Clean up
    cJSON_Delete(root);
    free(chunk.memory);
    curl_mime_free(mime);
    curl_slist_free_all(headers);
    curl_easy_cleanup(local_curl);

    log_info("API Detection (snapshot): Successfully detected %d objects", result->count);
    return 0;
}
