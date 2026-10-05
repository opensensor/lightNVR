#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>
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
#include "core/event_producers.h"
#include "utils/base64.h"
#include "utils/strings.h"
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

// Maximum length of the fully built request URL.
#define API_DETECTION_REQUEST_URL_MAX 1024

// DOODS2 request body up to (and including) the opening quote of the base64 "data" value.
#define DOODS2_BODY_PREFIX_FORMAT "{\"id\":%s,\"detector_name\":%s,\"detect\":{\"*\":%.2f},\"data\":\""

// ASCII printable character range used when sanitizing response previews.
#define ASCII_PRINTABLE_MIN 32
#define ASCII_PRINTABLE_MAX 126

// Returned by the snapshot entry point when go2rtc cannot supply a frame; the
// caller falls back to the decode path. Mirrors DETECT_SNAPSHOT_UNAVAILABLE.
#define API_DETECTION_SNAPSHOT_UNAVAILABLE -2

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

// Resolve the special "api-detection" model path to the globally configured URL.
static const char *resolve_api_url(const char *api_url, const char *context) {
    if (api_url && strcmp(api_url, "api-detection") == 0) {
        log_info("%s: Using API URL from config: %s", context,
                 g_config.api_detection_url[0] ? g_config.api_detection_url : "NULL");
        return g_config.api_detection_url;
    }
    return api_url;
}

/* ------------------------------------------------------------------------- */
/* Request options                                                           */
/* ------------------------------------------------------------------------- */

void api_detection_options_from_config(api_detection_options_t *options) {
    if (!options) {
        return;
    }
    memset(options, 0, sizeof(*options));

    api_detection_format_t format = API_DETECTION_FORMAT_LIGHT_OBJECT_DETECT;
    if (g_config.api_detection_format[0] != '\0' &&
        !api_detection_format_parse(g_config.api_detection_format, &format)) {
        log_warn("API Detection: Unknown api_detection format '%s', using %s",
                 g_config.api_detection_format,
                 api_detection_format_name(API_DETECTION_FORMAT_LIGHT_OBJECT_DETECT));
        format = API_DETECTION_FORMAT_LIGHT_OBJECT_DETECT;
    }
    options->format = format;
    safe_strcpy(options->backend, g_config.api_detection_backend, sizeof(options->backend), 0);
    safe_strcpy(options->detector_name, g_config.api_detection_detector_name,
                sizeof(options->detector_name), 0);
}

int api_detection_options_apply_json(api_detection_options_t *options,
                                     const char *config_json) {
    if (!options) {
        return -1;
    }
    if (!config_json || config_json[0] == '\0') {
        return 0;
    }

    cJSON *root = cJSON_Parse(config_json);
    if (!root) {
        log_warn("API Detection: Engine config is not valid JSON");
        return -1;
    }

    int rc = 0;
    if (!cJSON_IsObject(root)) {
        log_warn("API Detection: Engine config must be a JSON object");
        rc = -1;
    } else {
        const cJSON *format = cJSON_GetObjectItemCaseSensitive(root, "format");
        if (format && !cJSON_IsNull(format)) {
            api_detection_format_t parsed;
            if (cJSON_IsString(format) && api_detection_format_parse(format->valuestring, &parsed)) {
                options->format = parsed;
            } else {
                log_warn("API Detection: Engine config names an unknown format%s%s",
                         cJSON_IsString(format) ? ": " : "",
                         cJSON_IsString(format) ? format->valuestring : "");
                rc = -1;
            }
        }

        const cJSON *backend = cJSON_GetObjectItemCaseSensitive(root, "backend");
        if (cJSON_IsString(backend) && backend->valuestring[0] != '\0') {
            safe_strcpy(options->backend, backend->valuestring, sizeof(options->backend), 0);
        }

        const cJSON *detector = cJSON_GetObjectItemCaseSensitive(root, "detector_name");
        if (cJSON_IsString(detector) && detector->valuestring[0] != '\0') {
            safe_strcpy(options->detector_name, detector->valuestring,
                        sizeof(options->detector_name), 0);
        }
    }

    cJSON_Delete(root);
    return rc;
}

static const api_detection_options_t *resolve_options(const api_detection_options_t *options,
                                                      api_detection_options_t *storage) {
    if (options) {
        return options;
    }
    api_detection_options_from_config(storage);
    return storage;
}

static const char *effective_detector_name(const api_detection_options_t *options) {
    return (options->detector_name[0] != '\0') ? options->detector_name : "default";
}

/* ------------------------------------------------------------------------- */
/* Request construction                                                      */
/* ------------------------------------------------------------------------- */

int api_detection_build_request_url(char *buffer, size_t buffer_size,
                                    const char *base_url, float threshold,
                                    const api_detection_options_t *options) {
    if (buffer == NULL || buffer_size == 0 || base_url == NULL || !is_safe_base_url(base_url)) {
        return -1;
    }

    api_detection_options_t storage;
    options = resolve_options(options, &storage);

    int url_len;
    if (options->format == API_DETECTION_FORMAT_DOODS2) {
        // DOODS2 carries every parameter in the JSON body; the URL is used verbatim.
        url_len = snprintf(buffer, buffer_size, "%s", base_url);
    } else {
        const char *backend_param = sanitize_backend(options->backend);
        float actual_threshold = normalize_api_detection_threshold(threshold);
        char separator = (strchr(base_url, '?') != NULL) ? '&' : '?';

        url_len = snprintf(buffer, buffer_size,
                           "%s%cbackend=%s&confidence_threshold=%.2f&return_image=false",
                           base_url, separator, backend_param, actual_threshold);
    }

    if (url_len < 0 || (size_t)url_len >= buffer_size) {
        return -1;
    }
    return 0;
}

// JSON-quote a string (including the surrounding double quotes). Caller frees.
static char *json_quote(const char *text) {
    cJSON *item = cJSON_CreateString(text ? text : "");
    if (!item) {
        return NULL;
    }
    char *quoted = cJSON_PrintUnformatted(item);
    cJSON_Delete(item);
    return quoted;
}

char *api_detection_build_doods2_body(const unsigned char *jpeg_data, size_t jpeg_size,
                                      float threshold, const char *request_id,
                                      const api_detection_options_t *options) {
    if (!jpeg_data || jpeg_size == 0) {
        return NULL;
    }

    api_detection_options_t storage;
    options = resolve_options(options, &storage);

    char *id_json = json_quote((request_id && request_id[0]) ? request_id : "lightnvr");
    char *detector_json = json_quote(effective_detector_name(options));
    char *encoded = base64_encode_alloc(jpeg_data, jpeg_size);
    char *body = NULL;

    if (id_json && detector_json && encoded) {
        // DOODS2 filters on a 0-100 confidence scale; "*" applies to every label.
        double percent = (double)normalize_api_detection_threshold(threshold) * 100.0;
        int prefix_len = snprintf(NULL, 0, DOODS2_BODY_PREFIX_FORMAT, id_json, detector_json, percent);
        size_t encoded_len = strlen(encoded);
        if (prefix_len > 0) {
            size_t total = (size_t)prefix_len + encoded_len + 3; // closing quote, brace, NUL
            body = malloc(total);
            if (body) {
                int written = snprintf(body, total, DOODS2_BODY_PREFIX_FORMAT, id_json, detector_json, percent);
                if (written == prefix_len) {
                    memcpy(body + written, encoded, encoded_len);
                    body[written + encoded_len] = '"';
                    body[written + encoded_len + 1] = '}';
                    body[written + encoded_len + 2] = '\0';
                } else {
                    free(body);
                    body = NULL;
                }
            }
        }
    }

    if (!body) {
        log_error("API Detection: Failed to build DOODS2 request body (%zu byte JPEG)", jpeg_size);
    }

    free(id_json);
    free(detector_json);
    free(encoded);
    return body;
}

/* ------------------------------------------------------------------------- */
/* Response parsing                                                          */
/* ------------------------------------------------------------------------- */

static bool number_item(const cJSON *object, const char *key, double *out) {
    const cJSON *item = object ? cJSON_GetObjectItem(object, key) : NULL;
    if (!item || !cJSON_IsNumber(item)) {
        return false;
    }
    *out = item->valuedouble;
    return true;
}

// Extract a normalized box from either the light-object-detect shape
// (x_min/y_min/x_max/y_max, flat or nested under "bounding_box") or the
// DOODS2 shape (top/left/bottom/right).
static bool extract_box(const cJSON *detection, double *x_min, double *y_min,
                        double *x_max, double *y_max) {
    const cJSON *nested = cJSON_GetObjectItem(detection, "bounding_box");
    const cJSON *source = (nested && cJSON_IsObject(nested)) ? nested : detection;

    if (number_item(source, "x_min", x_min) && number_item(source, "y_min", y_min) &&
        number_item(source, "x_max", x_max) && number_item(source, "y_max", y_max)) {
        return true;
    }

    return number_item(detection, "left", x_min) && number_item(detection, "top", y_min) &&
           number_item(detection, "right", x_max) && number_item(detection, "bottom", y_max);
}

static void log_json_item(const char *message, const cJSON *item, bool as_error) {
    char *json_str = cJSON_Print(item);
    if (!json_str) {
        return;
    }
    if (as_error) {
        log_error("%s: %s", message, json_str);
    } else {
        log_warn("%s: %s", message, json_str);
    }
    free(json_str);
}

int api_detection_parse_response(const char *json,
                                 const api_detection_options_t *options,
                                 detection_result_t *result) {
    if (!json || !result) {
        return -1;
    }
    memset(result, 0, sizeof(*result));

    api_detection_options_t storage;
    options = resolve_options(options, &storage);
    const bool doods2 = (options->format == API_DETECTION_FORMAT_DOODS2);

    cJSON *root = cJSON_Parse(json);
    if (!root) {
        const char *error_ptr = cJSON_GetErrorPtr();
        log_error("API Detection: Failed to parse JSON response: %s", error_ptr ? error_ptr : "Unknown error");
        return -1;
    }

    if (doods2) {
        const cJSON *error = cJSON_GetObjectItem(root, "error");
        if (error && cJSON_IsString(error) && error->valuestring[0] != '\0') {
            log_error("API Detection: DOODS2 reported an error: %s", error->valuestring);
            cJSON_Delete(root);
            return -1;
        }
    }

    const cJSON *detections = cJSON_GetObjectItem(root, "detections");
    if (detections && cJSON_IsNull(detections)) {
        // An explicit null is "nothing detected", not a malformed reply.
        cJSON_Delete(root);
        return 0;
    }
    if (!detections || !cJSON_IsArray(detections)) {
        log_error("API Detection: Invalid JSON response: missing or invalid 'detections' array");
        log_json_item("API Detection: Full JSON response", root, true);
        cJSON_Delete(root);
        return -1;
    }

    int array_size = cJSON_GetArraySize(detections);
    for (int i = 0; i < array_size; i++) {
        if (result->count >= MAX_DETECTIONS) {
            log_warn("API Detection: Maximum number of detections reached (%d)", MAX_DETECTIONS);
            break;
        }

        const cJSON *detection = cJSON_GetArrayItem(detections, i);
        if (!detection || !cJSON_IsObject(detection)) {
            continue;
        }

        const cJSON *label = cJSON_GetObjectItem(detection, "label");
        double confidence = 0.0;
        double x_min = 0.0, y_min = 0.0, x_max = 0.0, y_max = 0.0;

        if (!label || !cJSON_IsString(label) ||
            !number_item(detection, "confidence", &confidence) ||
            !extract_box(detection, &x_min, &y_min, &x_max, &y_max)) {
            log_warn("API Detection: Invalid detection data in JSON response");
            log_json_item("API Detection: Detection JSON", detection, false);
            continue;
        }

        if (doods2) {
            confidence /= 100.0;
        }
        if (confidence < 0.0) {
            confidence = 0.0;
        } else if (confidence > 1.0) {
            confidence = 1.0;
        }

        detection_t *entry = &result->detections[result->count];
        safe_strcpy(entry->label, label->valuestring, MAX_LABEL_LENGTH, 0);
        entry->confidence = (float)confidence;
        entry->x = (float)x_min;
        entry->y = (float)y_min;
        entry->width = (float)(x_max - x_min);
        entry->height = (float)(y_max - y_min);

        const cJSON *track_id = cJSON_GetObjectItem(detection, "track_id");
        entry->track_id = (track_id && cJSON_IsNumber(track_id)) ? (int)track_id->valuedouble : -1;

        const cJSON *zone_id = cJSON_GetObjectItem(detection, "zone_id");
        if (zone_id && cJSON_IsString(zone_id)) {
            safe_strcpy(entry->zone_id, zone_id->valuestring, MAX_ZONE_ID_LENGTH, 0);
        } else {
            entry->zone_id[0] = '\0';
        }

        result->count++;
    }

    cJSON_Delete(root);
    return 0;
}

/* ------------------------------------------------------------------------- */
/* HTTP transport                                                            */
/* ------------------------------------------------------------------------- */

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

// Copy a printable prefix of the response body for logging.
static void make_response_preview(const memory_struct_t *chunk,
                                  char *preview, size_t preview_size) {
    size_t copy_len = 0;
    if (chunk->memory && preview_size > 0) {
        copy_len = chunk->size < (preview_size - 1) ? chunk->size : (preview_size - 1);
        memcpy(preview, chunk->memory, copy_len);
    }
    if (preview_size > 0) {
        preview[copy_len] = '\0';
    }
    for (size_t i = 0; i < copy_len; i++) {
        if (preview[i] < ASCII_PRINTABLE_MIN || preview[i] > ASCII_PRINTABLE_MAX) {
            preview[i] = '.';
        }
    }
}

// Build the light-object-detect multipart body: one "file" part holding the JPEG.
// curl_mime_data copies the bytes, so the caller may free jpeg_data afterwards.
static curl_mime *build_multipart_body(CURL *curl, const unsigned char *jpeg_data,
                                       size_t jpeg_size, const char *context) {
    curl_mime *mime = curl_mime_init(curl);
    if (!mime) {
        log_error("%s: Failed to create mime structure", context);
        return NULL;
    }

    curl_mimepart *part = curl_mime_addpart(mime);
    if (!part) {
        log_error("%s: Failed to add mime part", context);
        curl_mime_free(mime);
        return NULL;
    }

    CURLcode rc;
    if ((rc = curl_mime_name(part, "file")) != CURLE_OK ||
        (rc = curl_mime_data(part, (const char *)jpeg_data, jpeg_size)) != CURLE_OK ||
        (rc = curl_mime_filename(part, "snapshot.jpg")) != CURLE_OK ||
        (rc = curl_mime_type(part, "image/jpeg")) != CURLE_OK) {
        log_error("%s: Failed to build multipart body: %s", context, curl_easy_strerror(rc));
        curl_mime_free(mime);
        return NULL;
    }

    return mime;
}

/**
 * Send one JPEG to the detection API using the selected wire format and parse
 * the reply into result. Does not touch the database.
 *
 * Returns 0 on success, -1 on any transport, HTTP, or parse failure.
 */
static int perform_detection_request(const char *context, const char *base_url,
                                     const unsigned char *jpeg_data, size_t jpeg_size,
                                     float threshold, const char *request_id,
                                     const api_detection_options_t *options,
                                     detection_result_t *result) {
    CURL *curl = NULL;
    curl_mime *mime = NULL;
    struct curl_slist *headers = NULL;
    char *body = NULL;
    memory_struct_t chunk = {0};
    char request_url[API_DETECTION_REQUEST_URL_MAX];
    char preview[API_DETECTION_RESPONSE_PREVIEW_LEN];
    int ret = -1;

    if (api_detection_build_request_url(request_url, sizeof(request_url),
                                        base_url, threshold, options) != 0) {
        log_error("%s: Failed to construct request URL for %s", context, base_url);
        return -1;
    }

    // Use a per-call curl handle so detection requests can run concurrently.
    curl = curl_easy_init();
    if (curl == NULL) {
        log_error("%s: Failed to initialize CURL handle", context);
        return -1;
    }

    if (options->format == API_DETECTION_FORMAT_DOODS2) {
        body = api_detection_build_doods2_body(jpeg_data, jpeg_size, threshold, request_id, options);
        if (!body) {
            goto cleanup;
        }
        headers = curl_slist_append(headers, "Content-Type: application/json");
        curl_easy_setopt(curl, CURLOPT_POSTFIELDS, body);
        curl_easy_setopt(curl, CURLOPT_POSTFIELDSIZE_LARGE, (curl_off_t)strlen(body));
        log_info("%s: Sending DOODS2 request to %s (detector: %s, threshold: %.2f, %zu byte JPEG)",
                 context, request_url, effective_detector_name(options),
                 normalize_api_detection_threshold(threshold), jpeg_size);
    } else {
        mime = build_multipart_body(curl, jpeg_data, jpeg_size, context);
        if (!mime) {
            goto cleanup;
        }
        curl_easy_setopt(curl, CURLOPT_MIMEPOST, mime);
        log_info("%s: Sending request to %s (backend: %s, threshold: %.2f, %zu byte JPEG)",
                 context, request_url, sanitize_backend(options->backend),
                 normalize_api_detection_threshold(threshold), jpeg_size);
    }

    headers = curl_slist_append(headers, "accept: application/json");
    curl_easy_setopt(curl, CURLOPT_URL, request_url);
    curl_easy_setopt(curl, CURLOPT_HTTPHEADER, headers);

    chunk.memory = malloc(API_DETECTION_INITIAL_RESPONSE_BUFFER_SIZE);
    if (chunk.memory == NULL) {
        log_error("%s: Failed to allocate memory for curl response buffer", context);
        goto cleanup;
    }
    chunk.size = 0;
    chunk.capacity = API_DETECTION_INITIAL_RESPONSE_BUFFER_SIZE;
    chunk.memory[0] = '\0';

    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, write_memory_callback);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, (void *)&chunk);
    curl_easy_setopt(curl, CURLOPT_TIMEOUT, API_DETECTION_TIMEOUT_SECONDS);
    setup_common_curl_options(curl);

    CURLcode res = curl_easy_perform(curl);
    if (res != CURLE_OK) {
        log_error("%s: curl_easy_perform() failed: %s", context, curl_easy_strerror(res));
        log_tls_error_details(context, curl, res, request_url);

        if (res == CURLE_COULDNT_CONNECT) {
            log_error("%s: Could not connect to server at %s. Is the API server running?", context, request_url);
        } else if (res == CURLE_OPERATION_TIMEDOUT) {
            log_error("%s: Connection to %s timed out. Server might be slow or unreachable.", context, request_url);
        } else if (res == CURLE_COULDNT_RESOLVE_HOST) {
            log_error("%s: Could not resolve host %s. Check your network connection and DNS settings.", context, request_url);
        }
        goto cleanup;
    }

    long http_code = 0;
    curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &http_code);
    if (http_code != 200) {
        make_response_preview(&chunk, preview, sizeof(preview));
        log_error("%s: API request failed with HTTP code %ld (format: %s, response: %s)",
                  context, http_code, api_detection_format_name(options->format), preview);
        goto cleanup;
    }

    if (!chunk.memory || chunk.size == 0) {
        log_error("%s: Empty response from server", context);
        goto cleanup;
    }

    make_response_preview(&chunk, preview, sizeof(preview));
    log_info("%s: Response preview: %s", context, preview);

    ret = api_detection_parse_response(chunk.memory, options, result);
    if (ret != 0) {
        log_error("%s: Response size: %zu bytes", context, chunk.size);
    }

cleanup:
    free(body);
    free(chunk.memory);
    curl_mime_free(mime);
    curl_slist_free_all(headers);
    curl_easy_cleanup(curl);
    return ret;
}

/**
 * Apply zone and object filters, persist, and publish detections for a stream.
 * Returns 0 on success, -1 when zone filtering fails.
 */
static int finalize_detections(const char *context, const char *stream_name,
                               detection_result_t *result, time_t frame_timestamp,
                               uint64_t recording_id) {
    if (!stream_name || stream_name[0] == '\0') {
        log_warn("%s: No stream name provided, skipping database storage", context);
        return 0;
    }

    log_info("%s: Filtering %d detections by zones for stream %s", context, result->count, stream_name);
    if (filter_detections_by_zones(stream_name, result) != 0) {
        log_error("%s: Failed to filter detections by zones for stream %s, aborting detection pipeline for this frame",
                  context, stream_name);
        return -1;
    }

    filter_detections_by_stream_objects(stream_name, result);

    time_t timestamp = (frame_timestamp != 0) ? frame_timestamp : time(NULL);
    store_detections_in_db(stream_name, result, timestamp, recording_id);

    if (result->count > 0) {
        char event_error[256] = {0};
        if (event_producer_publish_detection_for_stream(
                stream_name, result, timestamp,
                event_error, sizeof(event_error)) != 0) {
            log_debug("%s: Event enqueue failed for %s: %s", context, stream_name, event_error);
        }
    }

    return 0;
}

/* ------------------------------------------------------------------------- */
/* Lifecycle                                                                 */
/* ------------------------------------------------------------------------- */

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

/* ------------------------------------------------------------------------- */
/* Entry points                                                              */
/* ------------------------------------------------------------------------- */

int detect_objects_api_with_options(const char *api_url, const unsigned char *frame_data,
                                    int width, int height, int channels,
                                    detection_result_t *result, const char *stream_name,
                                    float threshold, uint64_t recording_id,
                                    time_t frame_timestamp,
                                    const api_detection_options_t *options) {
    static const char *context = "API Detection";

    // Check if we're in shutdown mode or if the stream has been stopped.
    if (is_shutdown_initiated()) {
        log_info("%s: System shutdown in progress, skipping detection", context);
        return -1;
    }

    // Initialize result to empty at the beginning to prevent segmentation faults.
    if (!result) {
        log_error("%s: NULL result pointer provided", context);
        return -1;
    }
    memset(result, 0, sizeof(detection_result_t));

    const char *actual_api_url = resolve_api_url(api_url, context);
    log_info("%s: Starting detection with API URL: %s", context, actual_api_url ? actual_api_url : "NULL");
    log_info("%s: Stream name: %s", context, stream_name ? stream_name : "NULL");

    if (!is_api_detection_system_initialized()) {
        log_error("API detection system not initialized");
        return -1;
    }

    if (!validate_api_detection_base_url(actual_api_url, context)) {
        return -1;
    }

    api_detection_options_t storage;
    options = resolve_options(options, &storage);

    // Use go2rtc to get a JPEG snapshot directly only when we do not already
    // have a decoded frame. This avoids re-entering the go2rtc snapshot path
    // during fallback flows that already decoded a local frame.
    unsigned char *jpeg_data = NULL;
    size_t jpeg_size = 0;
    bool go2rtc_initialized = false;
    bool snapshot_ok = false;

    if (api_detection_should_use_go2rtc_snapshot(frame_data, width, height, channels, stream_name)) {
        go2rtc_initialized = go2rtc_integration_is_initialized();
        if (go2rtc_initialized) {
            snapshot_ok = go2rtc_get_snapshot(stream_name, &jpeg_data, &jpeg_size);
        }
    }

    if (snapshot_ok) {
        log_info("%s: Successfully fetched snapshot from go2rtc: %zu bytes", context, jpeg_size);
    } else {
        if (!stream_name || stream_name[0] == '\0') {
            log_debug("%s: No stream name provided for go2rtc snapshot, using cached JPEG encoding", context);
        } else if (!go2rtc_initialized) {
            log_debug("%s: go2rtc not initialized, using cached JPEG encoding", context);
        } else {
            log_warn("%s: Failed to get snapshot from go2rtc, falling back to cached JPEG encoding", context);
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
            log_error("%s: Failed to get cached JPEG encoder", context);
            return -1;
        }

        // Encode directly to memory - no temp file needed
        if (jpeg_encoder_cache_encode_to_memory(encoder, frame_data, &jpeg_data, &jpeg_size) != 0) {
            log_error("%s: Failed to encode frame to JPEG using cached encoder", context);
            return -1;
        }

        log_info("%s: Encoded frame to JPEG using cached encoder: %zu bytes", context, jpeg_size);
    }

    // Validate JPEG data.
    if (!jpeg_data || jpeg_size == 0) {
        log_error("%s: No JPEG data available", context);
        free(jpeg_data);
        return -1;
    }

    int ret = perform_detection_request(context, actual_api_url, jpeg_data, jpeg_size,
                                        threshold, stream_name, options, result);
    free(jpeg_data);

    if (ret == 0) {
        ret = finalize_detections(context, stream_name, result, frame_timestamp, recording_id);
    }

    if (ret != 0) {
        result->count = 0;
        return ret;
    }

    log_info("%s: Successfully detected %d objects", context, result->count);
    return 0;
}

int detect_objects_api(const char *api_url, const unsigned char *frame_data,
                      int width, int height, int channels, detection_result_t *result,
                      const char *stream_name, float threshold, uint64_t recording_id,
                      time_t frame_timestamp) {
    return detect_objects_api_with_options(api_url, frame_data, width, height, channels,
                                           result, stream_name, threshold, recording_id,
                                           frame_timestamp, NULL);
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
int detect_objects_api_snapshot_with_options(const char *api_url, const char *stream_name,
                                             detection_result_t *result, float threshold,
                                             uint64_t recording_id, time_t frame_timestamp,
                                             const api_detection_options_t *options) {
    static const char *context = "API Detection (snapshot)";

    // Check if we're in shutdown mode
    if (is_shutdown_initiated()) {
        log_info("%s: System shutdown in progress, skipping detection", context);
        return -1;
    }

    // Stream name is required for go2rtc snapshot
    if (!stream_name || stream_name[0] == '\0') {
        log_error("%s: Stream name is required", context);
        return -1;
    }

    // Initialize result
    if (!result) {
        log_error("%s: NULL result pointer provided", context);
        return -1;
    }
    memset(result, 0, sizeof(detection_result_t));

    const char *actual_api_url = resolve_api_url(api_url, context);

    if (!is_api_detection_system_initialized()) {
        log_error("API detection system not initialized");
        return -1;
    }

    if (!validate_api_detection_base_url(actual_api_url, context)) {
        return -1;
    }

    api_detection_options_t storage;
    options = resolve_options(options, &storage);

    // Try to get snapshot from go2rtc (only if go2rtc is initialized)
    unsigned char *jpeg_data = NULL;
    size_t jpeg_size = 0;

    if (!go2rtc_integration_is_initialized()) {
        log_debug("%s: go2rtc not initialized, skipping snapshot for stream %s", context, stream_name);
        return API_DETECTION_SNAPSHOT_UNAVAILABLE;  // caller should fall back
    }

    if (!go2rtc_get_snapshot(stream_name, &jpeg_data, &jpeg_size)) {
        log_warn("%s: Failed to get snapshot from go2rtc for stream %s", context, stream_name);
        return API_DETECTION_SNAPSHOT_UNAVAILABLE;  // caller should fall back
    }

    log_info("%s: Successfully fetched snapshot from go2rtc: %zu bytes", context, jpeg_size);

    // Validate JPEG data
    if (!jpeg_data || jpeg_size == 0) {
        log_error("%s: No JPEG data available", context);
        free(jpeg_data);
        return API_DETECTION_SNAPSHOT_UNAVAILABLE;
    }

    int ret = perform_detection_request(context, actual_api_url, jpeg_data, jpeg_size,
                                        threshold, stream_name, options, result);
    free(jpeg_data);

    if (ret == 0) {
        ret = finalize_detections(context, stream_name, result, frame_timestamp, recording_id);
    }

    if (ret != 0) {
        result->count = 0;
        return -1;
    }

    log_info("%s: Successfully detected %d objects", context, result->count);
    return 0;
}

int detect_objects_api_snapshot(const char *api_url, const char *stream_name,
                                detection_result_t *result, float threshold,
                                uint64_t recording_id, time_t frame_timestamp) {
    return detect_objects_api_snapshot_with_options(api_url, stream_name, result, threshold,
                                                    recording_id, frame_timestamp, NULL);
}
