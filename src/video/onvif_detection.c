/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2025-2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>   /* strncasecmp / strcasecmp for case-insensitive class matching */
#include <stdbool.h>
#include <ctype.h>
#include <errno.h>
#include <unistd.h>
#include <sys/stat.h>
#include <curl/curl.h>
#include "core/url_utils.h"
#include <cjson/cJSON.h>
#include <pthread.h>
#include <stdatomic.h>
#include <stdint.h>
#include <stddef.h>
#include <time.h>

#include "core/logger.h"
#include "core/config.h"
#include "core/curl_init.h"
#include "core/shutdown_coordinator.h"
#include "core/event_producers.h"
#include "utils/strings.h"
#include "video/onvif_detection.h"
#include "video/onvif_event.h"
#include "video/onvif_soap.h"
#include "video/detection_result.h"
#include "video/cross_stream_motion_trigger.h"
#include "video/mp4_recording.h"
#include "video/zone_filter.h"
#include "database/db_detections.h"
#include "database/db_lpr_reads.h"
#include "database/db_streams.h"
#include "utils/lpr_crypto.h"
#include "ezxml.h"

/* SOAP action identifiers are protocol metadata, not transport endpoints.
 * Callers select an operation; the request layer resolves its URI.
 * Keep these standardized http:// identifiers unchanged even over HTTPS. */
typedef enum {
    ONVIF_OPERATION_GET_SERVICES,
    ONVIF_OPERATION_CREATE_PULLPOINT,
    ONVIF_OPERATION_PULL_MESSAGES,
    ONVIF_OPERATION_RENEW,
    ONVIF_OPERATION_UNSUBSCRIBE,
    ONVIF_OPERATION_COUNT
} onvif_operation_t;

static const char *onvif_operation_action(onvif_operation_t operation) {
    static const char *const actions[ONVIF_OPERATION_COUNT] = {
        [ONVIF_OPERATION_GET_SERVICES] = "http://www.onvif.org/ver10/device/wsdl/GetServices",
        [ONVIF_OPERATION_CREATE_PULLPOINT] = "http://www.onvif.org/ver10/events/wsdl/EventPortType/CreatePullPointSubscriptionRequest",
        [ONVIF_OPERATION_PULL_MESSAGES] = "http://www.onvif.org/ver10/events/wsdl/PullPointSubscription/PullMessagesRequest",
        [ONVIF_OPERATION_RENEW] = "http://docs.oasis-open.org/wsn/bw-2/SubscriptionManager/RenewRequest",
        [ONVIF_OPERATION_UNSUBSCRIBE] = "http://docs.oasis-open.org/wsn/bw-2/SubscriptionManager/UnsubscribeRequest"
    };
    return (unsigned int)operation < ONVIF_OPERATION_COUNT ? actions[operation] : NULL;
}

static bool has_uri_scheme_prefix(const char *reference) {
    if (!reference || !isalpha((unsigned char)reference[0])) return false;

    for (size_t i = 1; reference[i] != '\0'; ++i) {
        unsigned char ch = (unsigned char)reference[i];
        if (ch == ':') return true;
        if (!(isalnum(ch) || ch == '+' || ch == '-' || ch == '.')) return false;
    }

    return false;
}

/* Tapo cameras accept a ten-minute PullPoint lease and reject both the old
 * one-hour request and a request with no InitialTerminationTime (#567).
 * Renew at 80% of the camera-reported lease so normal polling jitter cannot
 * push us beyond the actual expiry. */
#define ONVIF_SUBSCRIPTION_LEASE_SECONDS 600
#define ONVIF_RENEW_NUMERATOR 4
#define ONVIF_RENEW_DENOMINATOR 5

/* Tapo event services can drop individual PullMessages connections while the
 * underlying PullPoint remains valid. Keep retrying the same subscription and
 * only refresh it after a sustained outage (#567). */
#define ONVIF_PULL_FAILURE_WARN_COUNT 3
#define ONVIF_PULL_FAILURE_REFRESH_SECONDS 120

/* Some Tapo firmware (C530WS/C220 in #567/#603, C325WB elsewhere) accepts the
 * TCP connection for a PullMessages/Renew/Unsubscribe request and closes it
 * again within milliseconds without any HTTP response (libcurl: "Server
 * returned nothing"), while the PullPoint itself stays valid and keeps queueing
 * events. Clients that work with these cameras re-issue the request on the
 * same subscription a few times per second instead of backing off or
 * resubscribing, and enough of those retries get through. The budget
 * (ONVIF_PULL_DROP_MAX_ATTEMPTS x ONVIF_PULL_DROP_RETRY_DELAY_MS, declared in
 * onvif_detection.h) bounds each burst. If it is exhausted, the polling thread
 * continues after the same short delay instead of the normal five-second
 * error backoff, which can miss the camera's next accepting window. */

/* External UUID generator (used for wsa:MessageID). */
extern void generate_uuid(char *uuid, size_t size);

// Global variables
static bool initialized = false;
static CURL *curl_handle = NULL;
static pthread_mutex_t curl_mutex = PTHREAD_MUTEX_INITIALIZER;

// Structure to hold memory for curl response
typedef struct {
    char *memory;
    size_t size;
} memory_struct_t;

#define ONVIF_RESPONSE_MAX_BYTES (4U * 1024U * 1024U)

static void secure_clear(void *data, size_t size) {
    volatile unsigned char *bytes = data;
    while (bytes && size-- > 0) *bytes++ = 0;
}

// Structure to hold ONVIF subscription information
typedef struct {
    char camera_url[512];           // URL of the camera (used as the key for lookup)
    char subscription_address[512]; // Address returned by CreatePullPointSubscription (full URL)
    char reference_parameters[2048]; // SOAP header children from the endpoint reference
    char reference_namespaces[1024]; // Namespace bindings inherited by those children
    char service_url[512];          // Endpoint that created this PullPoint
    char username[64];              // Username for authentication
    char password[64];              // Password for authentication
    time_t creation_time;
    time_t renewal_time;
    time_t expiration_time;
    time_t first_pull_failure_time;
    int consecutive_pull_failures;
    atomic_bool active;             // Read under table lock, cleared after network I/O
    bool reserved;                  // Creation or renewal in progress
    CURL *curl;                     // Connection for this PullPoint (see subscription_locks)
} onvif_subscription_t;

// Hash map to store subscriptions by URL
#define MAX_SUBSCRIPTIONS 100
static onvif_subscription_t subscriptions[MAX_SUBSCRIPTIONS];
static int subscription_count = 0;
static pthread_mutex_t subscription_mutex = PTHREAD_MUTEX_INITIALIZER;
static bool subscriptions_shutting_down = false;

/* Serialize operations for one camera across table lookup and PullMessages.
 * Hash collisions only reduce concurrency; they cannot mix camera identities. */
static pthread_mutex_t camera_locks[64];
static pthread_once_t camera_locks_once = PTHREAD_ONCE_INIT;

static void init_camera_locks(void) {
    for (size_t i = 0; i < sizeof(camera_locks) / sizeof(camera_locks[0]); i++)
        pthread_mutex_init(&camera_locks[i], NULL);
}

static pthread_mutex_t *camera_lock_for_url(const char *url) {
    unsigned long hash = 5381;
    for (const unsigned char *p = (const unsigned char *)url; *p; p++)
        hash = hash * 33 + *p;
    pthread_once(&camera_locks_once, init_camera_locks);
    return &camera_locks[hash % (sizeof(camera_locks) / sizeof(camera_locks[0]))];
}

/* Each PullPoint talks to its camera over its own CURL handle, serialized by
 * the lock of the same index. A PullMessages long poll holds the camera's
 * connection for up to five seconds, so sharing one handle across cameras
 * (the previous design) queued every camera behind every other camera's poll
 * and behind the drop retries of #603: with three cameras, a Tapo that only
 * accepts a fresh connection for a short window after each response was
 * polled once every ten to twenty-five seconds and hit closed windows. The
 * locks outlive their slots (a slot is reset in place), so they are
 * initialized once and never destroyed. */
static pthread_mutex_t subscription_locks[MAX_SUBSCRIPTIONS];
static pthread_once_t subscription_locks_once = PTHREAD_ONCE_INIT;

static void init_subscription_locks(void) {
    for (int i = 0; i < MAX_SUBSCRIPTIONS; i++)
        pthread_mutex_init(&subscription_locks[i], NULL);
}

static pthread_mutex_t *subscription_lock(const onvif_subscription_t *subscription) {
    ptrdiff_t index = subscription - subscriptions;
    if (index < 0 || index >= MAX_SUBSCRIPTIONS) return NULL;
    pthread_once(&subscription_locks_once, init_subscription_locks);
    return &subscription_locks[index];
}

// Callback function for curl to write data
static size_t write_memory_callback(void *contents, size_t size, size_t nmemb, void *userp) {
    if (size != 0 && nmemb > SIZE_MAX / size) return 0;
    size_t realsize = size * nmemb;
    memory_struct_t *mem = (memory_struct_t *)userp;

    if (mem->size > ONVIF_RESPONSE_MAX_BYTES ||
        realsize > ONVIF_RESPONSE_MAX_BYTES - mem->size) {
        log_warn("Rejected oversized ONVIF response");
        return 0;
    }

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

/* Trace only event sizes. Headers, request targets and curl's free-form notes
 * can contain URL tokens, Authorization or Set-Cookie values. */
static int curl_trace_callback(CURL *handle, curl_infotype type, char *data,
                               size_t size, void *userp) {
    (void)handle;
    (void)userp;
    (void)data;
    const char *prefix;
    switch (type) {
        case CURLINFO_HEADER_OUT: prefix = ">"; break;
        case CURLINFO_HEADER_IN: prefix = "<"; break;
        default: return 0;
    }
    log_info("ONVIF curl %s headers: %zu bytes (content omitted)", prefix, size);
    return 0;
}

static CURL *create_curl_handle(void) {
    CURL *handle = curl_easy_init();
    if (!handle) return NULL;
    /* SOAP bodies carry authentication material. Never forward them through
     * redirects or allow camera-advertised endpoints to select other protocols. */
#if LIBCURL_VERSION_NUM >= 0x075500
    CURLcode allowed = curl_easy_setopt(handle, CURLOPT_PROTOCOLS_STR, "http,https");
#else
    CURLcode allowed = curl_easy_setopt(handle, CURLOPT_PROTOCOLS, CURLPROTO_HTTP | CURLPROTO_HTTPS);
#endif
    if (allowed != CURLE_OK ||
        curl_easy_setopt(handle, CURLOPT_FOLLOWLOCATION, 0L) != CURLE_OK ||
        curl_easy_setopt(handle, CURLOPT_SSL_VERIFYPEER, 1L) != CURLE_OK ||
        curl_easy_setopt(handle, CURLOPT_SSL_VERIFYHOST, 2L) != CURLE_OK) {
        curl_easy_cleanup(handle);
        return NULL;
    }
    if (getenv("LIGHTNVR_ONVIF_CURL_TRACE")) {
        curl_easy_setopt(handle, CURLOPT_VERBOSE, 1L);
        curl_easy_setopt(handle, CURLOPT_DEBUGFUNCTION, curl_trace_callback);
    }
    return handle;
}

/* Drop a PullPoint's connection. Waits for a request in flight on it. */
static void release_subscription_curl(onvif_subscription_t *subscription) {
    pthread_mutex_t *lock = subscription_lock(subscription);
    if (!lock) return;
    pthread_mutex_lock(lock);
    if (subscription->curl) {
        curl_easy_cleanup(subscription->curl);
        subscription->curl = NULL;
    }
    pthread_mutex_unlock(lock);
}

/* Transport-level outcome of one ONVIF HTTP exchange, for callers that need to
 * tell "the camera dropped the connection" apart from every other failure. */
typedef struct {
    CURLcode curl_code;
    long http_code;
    double elapsed;
    long new_connections;
} onvif_request_status_t;

/*
 * Create ONVIF SOAP request with WS-Security (if credentials provided) and
 * WS-Addressing headers. The operation selects its action URI; `to` is the
 * destination URL; both are required (strict ONVIF servers
 * such as Reolink reject requests missing either — #374).
 */
static char *create_onvif_request(const char *username, const char *password,
                                  const char *request_body,
                                  onvif_operation_t operation, const char *to,
                                  const char *ref_params,
                                  const char *ref_namespaces) {
    const char *action = onvif_operation_action(operation);
    if (!action || !request_body) return NULL;
    bool has_credentials = (username && strlen(username) > 0 && password && strlen(password) > 0);
    char *security_header = NULL;

    if (!has_credentials) {
        log_debug("Creating ONVIF request without authentication (no credentials provided)");
        security_header = strdup("");
    } else {
        log_debug("Creating ONVIF request with WS-Security authentication");
        security_header = onvif_create_security_header(username, password);
        if (!security_header) {
            log_error("Failed to create WS-Security header");
            return NULL;
        }
    }

    if (!security_header) return NULL;

    /* Build the WS-Addressing fragment for the selected operation. Using
     * `urn:uuid:<v4>` for MessageID and leaving mustUnderstand off on To/Action
     * — cameras that ignore WS-Addressing shouldn't then fault because they
     * "didn't understand" an optional header. */
    char uuid[48] = {0};
    generate_uuid(uuid, sizeof(uuid));

    ezxml_t destination = ezxml_new("wsa:To");
    if (!destination) {
        free(security_header);
        return NULL;
    }
    ezxml_set_txt(destination, to ? to : "");
    char *destination_xml = ezxml_toxml(destination);
    ezxml_free(destination);
    if (!destination_xml) {
        free(security_header);
        return NULL;
    }

    /* The WS-Addressing block inflates the envelope; reserve enough for
     * MessageID (urn:uuid:… = ~45 chars), the action URI, the destination URL,
     * the security header, and the body. 2 KB of slack above those sizes is
     * plenty. */
    size_t envelope_size = strlen(request_body) + strlen(security_header)
                           + strlen(action) + strlen(destination_xml)
                           + (ref_params ? strlen(ref_params) : 0)
                           + (ref_namespaces ? strlen(ref_namespaces) : 0) + 2048;
    char *soap_request = malloc(envelope_size);
    if (!soap_request) {
        free(security_header);
        free(destination_xml);
        return NULL;
    }

    snprintf(soap_request, envelope_size,
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>"
        "<s:Envelope xmlns:s=\"http://www.w3.org/2003/05/soap-envelope\" "
                   "xmlns:wsa=\"http://www.w3.org/2005/08/addressing\">"
        "<s:Header%s>"
            "<wsa:MessageID>urn:uuid:%s</wsa:MessageID>"
            "%s"
            "<wsa:Action>%s</wsa:Action>"
            "%s"
            "%s"
        "</s:Header>"
        "<s:Body xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" "
                "xmlns:xsd=\"http://www.w3.org/2001/XMLSchema\">%s</s:Body>"
        "</s:Envelope>",
        ref_namespaces ? ref_namespaces : "", uuid, destination_xml, action,
        ref_params ? ref_params : "", security_header, request_body);

    free(security_header);
    free(destination_xml);
    return soap_request;
}

/*
 * Core of one ONVIF HTTP exchange with a full URL (bypassing the base URL +
 * /onvif/ + service construction). The operation resolves a WS-Addressing action URI;
 * it's also added as the `action=` parameter on the Content-Type header per
 * SOAP 1.2, which strict ONVIF servers require (#374).
 *
 * curl_mutex must be held by the caller. When `status` is provided the
 * transport outcome is returned in it and curl-level failures are logged at
 * debug level only, because the caller is retrying and reports the final
 * result itself.
 */
/* The caller holds the lock that serializes `handle` (curl_mutex for the
 * shared handle, the slot lock for a subscription's own). */
static char *perform_onvif_request_with_reference_locked(CURL *handle, const char *full_url,
                                                          const char *username,
                                                          const char *password,
                                                          const char *request_body,
                                                          onvif_operation_t operation,
                                                          onvif_request_status_t *status,
                                                          const char *ref_params,
                                                          const char *ref_namespaces) {
    if (status) memset(status, 0, sizeof(*status));
    const char *action = onvif_operation_action(operation);
    if (!action || !request_body) return NULL;
    if (!initialized || !handle) {
        log_error("ONVIF detection system not initialized");
        return NULL;
    }

    if (!url_is_http_endpoint(full_url)) {
        log_error("Rejected invalid ONVIF HTTP(S) endpoint");
        return NULL;
    }
    log_debug("ONVIF Detection: Sending SOAP request");

    /* Use full_url as the WS-Addressing To, which is exactly what the request
     * is being POSTed to. */
    char *soap_request = create_onvif_request(username, password, request_body,
                                              operation, full_url, ref_params,
                                              ref_namespaces);
    if (!soap_request) {
        log_error("Failed to create ONVIF request");
        return NULL;
    }

    // Set up curl
    curl_easy_setopt(handle, CURLOPT_URL, full_url);
    curl_easy_setopt(handle, CURLOPT_POSTFIELDS, soap_request);
    curl_easy_setopt(handle, CURLOPT_POSTFIELDSIZE, strlen(soap_request));

    /* The same operation URI is used in SOAP 1.2 Content-Type and wsa:Action. */
    char content_type[512];
    snprintf(content_type, sizeof(content_type),
             "Content-Type: application/soap+xml; charset=utf-8; action=\"%s\"",
             action);

    struct curl_slist *headers = NULL;
    headers = curl_slist_append(headers, content_type);
    curl_easy_setopt(handle, CURLOPT_HTTPHEADER, headers);

    // Set up response buffer
    memory_struct_t chunk = {0};

    curl_easy_setopt(handle, CURLOPT_WRITEFUNCTION, write_memory_callback);
    curl_easy_setopt(handle, CURLOPT_WRITEDATA, (void *)&chunk);
    curl_easy_setopt(handle, CURLOPT_TIMEOUT, 10L);
    curl_easy_setopt(handle, CURLOPT_NOSIGNAL, 1L);

    // Perform request
    CURLcode res = curl_easy_perform(handle);
    long http_code = 0;
    double elapsed = 0;
    long new_connections = 0;
    curl_easy_getinfo(handle, CURLINFO_RESPONSE_CODE, &http_code);
    curl_easy_getinfo(handle, CURLINFO_TOTAL_TIME, &elapsed);
    /* 0 means the transfer ran on a reused keep-alive connection; libcurl
     * retries a dead reused connection on a fresh one by itself, so a
     * "returned nothing" with new_connections=1 is the camera closing a fresh
     * connection, not a stale keep-alive. */
    curl_easy_getinfo(handle, CURLINFO_NUM_CONNECTS, &new_connections);
    if (status) {
        status->curl_code = res;
        status->http_code = http_code;
        status->elapsed = elapsed;
        status->new_connections = new_connections;
    }

    // Clean up request
    free(soap_request);
    curl_slist_free_all(headers);

    // Check for errors
    if (res != CURLE_OK) {
        if (status) {
            log_debug("ONVIF request %s attempt failed (curl=%d, HTTP=%ld, elapsed=%.3fs, "
                      "new connections=%ld): %s",
                      action, (int)res, http_code, elapsed,
                      new_connections, curl_easy_strerror(res));
        } else {
            log_error("ONVIF request %s failed (curl=%d, HTTP=%ld, elapsed=%.3fs, "
                      "new connections=%ld): %s",
                      action, (int)res, http_code, elapsed,
                      new_connections, curl_easy_strerror(res));
        }
        free(chunk.memory);
        return NULL;
    }

    if (http_code != 200 || chunk.size == 0) {
        log_error("ONVIF request %s failed (HTTP=%ld, response=%zu bytes, elapsed=%.3fs)",
                  action, http_code, chunk.size, elapsed);
        if (chunk.size > 0) {
            onvif_log_soap_fault(chunk.memory, chunk.size, "ONVIF Detection");
        }
        free(chunk.memory);
        return NULL;
    }

    return chunk.memory;
}

static char *perform_onvif_request_locked(CURL *handle, const char *full_url,
                                          const char *username, const char *password,
                                          const char *request_body, onvif_operation_t operation,
                                          onvif_request_status_t *status) {
    return perform_onvif_request_with_reference_locked(handle, full_url, username,
                                                       password, request_body, operation,
                                                       status, NULL, NULL);
}

/* One-shot ONVIF request to a full URL. */
static char *send_onvif_request_to_url(const char *full_url, const char *username,
                                       const char *password, const char *request_body,
                                       onvif_operation_t operation) {
    pthread_mutex_lock(&curl_mutex);
    char *response = perform_onvif_request_locked(curl_handle, full_url, username,
                                                  password, request_body, operation, NULL);
    pthread_mutex_unlock(&curl_mutex);
    return response;
}

// Send ONVIF request and get response
static char *send_onvif_request(const char *url, const char *username, const char *password,
                               const char *request_body, const char *service,
                               onvif_operation_t operation) {
    if (!initialized || !curl_handle) {
        log_error("ONVIF detection system not initialized");
        return NULL;
    }

    // Create full URL
    char full_url[512];
    int written = snprintf(full_url, sizeof(full_url), "%s/onvif/%s", url, service);
    if (written < 0 || (size_t)written >= sizeof(full_url)) {
        log_error("ONVIF request URL too long or formatting failed");
        return NULL;
    }

    return send_onvif_request_to_url(full_url, username, password, request_body, operation);
}

/* Return the local name of an ezxml element (strips any "prefix:" prefix). */
static const char *local_name(ezxml_t el) {
    if (!el || !el->name) return "";
    const char *colon = strchr(el->name, ':');
    return colon ? colon + 1 : el->name;
}

/* Find a direct child with the given local name, ignoring namespace prefix. */
static ezxml_t child_by_local_name(ezxml_t parent, const char *local) {
    if (!parent || !local) return NULL;
    for (ezxml_t c = parent->child; c; c = c->ordered) {
        if (strcmp(local_name(c), local) == 0) return c;
    }
    return NULL;
}

/* The destination belongs to SubscriptionReference, not an unrelated
 * WS-Addressing header. XML parsing also handles vendor prefixes, attributes,
 * and escaped query-string separators in the returned URL. */
static bool append_reference_namespace(char *dst, size_t capacity, const char *name,
                                       const char *value) {
    size_t used = strlen(dst);
    int written = snprintf(dst + used, capacity - used, " %s=\"", name);
    if (written < 0 || (size_t)written >= capacity - used) return false;
    used += (size_t)written;
    for (const char *p = value; *p; p++) {
        const char *escaped = *p == '&' ? "&amp;" : *p == '"' ? "&quot;" :
                              *p == '<' ? "&lt;" : NULL;
        size_t len = escaped ? strlen(escaped) : 1;
        if (len + 2 > capacity - used) return false;
        if (escaped) memcpy(dst + used, escaped, len);
        else dst[used] = *p;
        used += len;
    }
    dst[used++] = '"';
    dst[used] = '\0';
    return true;
}

static bool extract_subscription_endpoint_reference(const char *response,
                                                   char **out_address,
                                                   char **out_ref_params_xml,
                                                   char **out_ref_namespaces) {
    if (out_address) *out_address = NULL;
    if (out_ref_params_xml) *out_ref_params_xml = NULL;
    if (out_ref_namespaces) *out_ref_namespaces = NULL;
    if (!response || !out_address) return false;

    char *copy = strdup(response);
    if (!copy) return false;

    ezxml_t root = ezxml_parse_str(copy, strlen(copy));
    bool ok = false;
    if (root && !ezxml_error(root)[0]) {
        ezxml_t body = child_by_local_name(root, "Body");
        ezxml_t reply = child_by_local_name(body, "CreatePullPointSubscriptionResponse");
        ezxml_t reference = child_by_local_name(reply, "SubscriptionReference");
        ezxml_t destination = child_by_local_name(reference, "Address");
        ezxml_t ref_params = child_by_local_name(reference, "ReferenceParameters");

        if (destination && destination->txt && destination->txt[0]) {
            *out_address = strdup(destination->txt);
            if (*out_address) {
                ok = true;
                if (out_ref_params_xml && out_ref_namespaces && ref_params && ref_params->child) {
                    for (ezxml_t param = ref_params->child; param; param = param->ordered) {
                        const char *binding = ezxml_attr(param, "xmlns:wsa");
                        if (binding && strcmp(binding,
                            "http://www.w3.org/2005/08/addressing") != 0) {
                            ok = false;
                            break;
                        }
                        ezxml_set_attr(param, "wsa:IsReferenceParameter", "true");
                    }
                    if (!ok) goto endpoint_parsed;
                    char *xml = ezxml_toxml(ref_params);
                    const char *start = xml ? strchr(xml, '>') : NULL;
                    const char *end = xml ? strrchr(xml, '<') : NULL;
                    if (start && end && end > start + 1) {
                        *out_ref_params_xml = strndup(start + 1, (size_t)(end - start - 1));
                    }
                    if (xml) free(xml);

                    char namespaces[sizeof(subscriptions[0].reference_namespaces)] = {0};
                    const char *names[32] = {0};
                    size_t count = 0;
                    for (ezxml_t node = ref_params; node; node = node->parent) {
                        for (size_t i = 0; node->attr && node->attr[i]; i += 2) {
                            const char *name = node->attr[i];
                            if (strcmp(name, "xmlns") != 0 && strncmp(name, "xmlns:", 6) != 0)
                                continue;
                            /* Keep SOAP and WS-Addressing header names bound to their
                             * required namespaces. Camera-supplied bindings may only
                             * provide namespaces for reference parameter children. */
                            if (strcmp(name, "xmlns:s") == 0 ||
                                strcmp(name, "xmlns:wsa") == 0) {
                                const char *expected = strcmp(name, "xmlns:s") == 0 ?
                                    "http://www.w3.org/2003/05/soap-envelope" :
                                    "http://www.w3.org/2005/08/addressing";
                                if (strcmp(node->attr[i + 1], expected) != 0) ok = false;
                                continue;
                            }
                            if (strcmp(name, "xmlns:xml") == 0 ||
                                strcmp(name, "xmlns:xmlns") == 0) continue;
                            bool seen = false;
                            for (size_t j = 0; j < count; j++)
                                if (strcmp(name, names[j]) == 0) seen = true;
                            if (seen) continue;
                            if (count >= sizeof(names) / sizeof(names[0]) ||
                                !append_reference_namespace(namespaces, sizeof(namespaces),
                                                            name, node->attr[i + 1])) {
                                ok = false;
                                break;
                            }
                            names[count++] = name;
                        }
                        if (!ok) break;
                    }
                    if (ok) *out_ref_namespaces = strdup(namespaces);
                    if (!*out_ref_params_xml || !*out_ref_namespaces) ok = false;
                }
            }
        }
    }

endpoint_parsed:

    if (root) ezxml_free(root);
    free(copy);
    if (!ok) {
        free(*out_address);
        *out_address = NULL;
        if (out_ref_params_xml) { free(*out_ref_params_xml); *out_ref_params_xml = NULL; }
        if (out_ref_namespaces) { free(*out_ref_namespaces); *out_ref_namespaces = NULL; }
    }
    return ok;
}

/* Find the first descendant with the given local name. */
static ezxml_t descendant_by_local_name(ezxml_t node, const char *local) {
    if (!node || !local) return NULL;
    if (strcmp(local_name(node), local) == 0) return node;
    for (ezxml_t child = node->child; child; child = child->ordered) {
        ezxml_t match = descendant_by_local_name(child, local);
        if (match) return match;
    }
    return NULL;
}

/* Parse the UTC/offset xsd:dateTime values returned by ONVIF cameras. */
static bool parse_xsd_datetime(const char *text, time_t *timestamp) {
    if (!text || !timestamp) return false;

    struct tm parsed;
    memset(&parsed, 0, sizeof(parsed));
    char *suffix = strptime(text, "%Y-%m-%dT%H:%M:%S", &parsed);
    if (!suffix) return false;

    /* Fractional seconds do not affect subscription scheduling. */
    if (*suffix == '.') {
        suffix++;
        while (isdigit((unsigned char)*suffix)) suffix++;
    }

    int offset_seconds = 0;
    if (*suffix == 'Z' || *suffix == 'z') {
        suffix++;
    } else if (*suffix == '+' || *suffix == '-') {
        int sign = (*suffix++ == '+') ? 1 : -1;
        if (strlen(suffix) < 5 ||
            !isdigit((unsigned char)suffix[0]) ||
            !isdigit((unsigned char)suffix[1]) || suffix[2] != ':' ||
            !isdigit((unsigned char)suffix[3]) ||
            !isdigit((unsigned char)suffix[4])) {
            return false;
        }
        int hours = 0;
        int minutes = 0;
        if (sscanf(suffix, "%2d:%2d", &hours, &minutes) != 2 ||
            hours > 23 || minutes > 59) {
            return false;
        }
        offset_seconds = sign * (hours * 3600 + minutes * 60);
        suffix += 5;
    }

    while (isspace((unsigned char)*suffix)) suffix++;
    if (*suffix != '\0') return false;

    time_t utc = timegm(&parsed);
    if (utc == (time_t)-1) return false;
    *timestamp = utc - offset_seconds;
    return true;
}

/* Prefer the duration between the camera's CurrentTime and TerminationTime,
 * avoiding any error from a camera clock that differs from the recorder. */
static int subscription_lease_seconds(const char *response) {
    int lease = ONVIF_SUBSCRIPTION_LEASE_SECONDS;
    if (!response) return lease;

    char *copy = strdup(response);
    if (!copy) return lease;

    ezxml_t xml = ezxml_parse_str(copy, strlen(copy));
    if (xml) {
        ezxml_t current_node = descendant_by_local_name(xml, "CurrentTime");
        ezxml_t termination_node = descendant_by_local_name(xml, "TerminationTime");
        time_t current_time;
        time_t termination_time;
        bool have_current = current_node &&
            parse_xsd_datetime(ezxml_txt(current_node), &current_time);
        bool have_termination = termination_node &&
            parse_xsd_datetime(ezxml_txt(termination_node), &termination_time);

        time_t duration = 0;
        if (have_current && have_termination) {
            duration = termination_time - current_time;
        } else if (have_termination) {
            duration = termination_time - time(NULL);
        }

        /* Reject obviously broken timestamps. The requested ten-minute lease
         * remains the safe fallback for cameras that omit or mangle them. */
        if (duration > 0 && duration <= 7 * 24 * 60 * 60) {
            lease = (int)duration;
        }
        ezxml_free(xml);
    }

    free(copy);
    return lease;
}

static void apply_subscription_lease(onvif_subscription_t *subscription,
                                     int lease_seconds) {
    if (!subscription) return;
    if (lease_seconds <= 0) lease_seconds = ONVIF_SUBSCRIPTION_LEASE_SECONDS;

    time(&subscription->creation_time);
    subscription->expiration_time = subscription->creation_time + lease_seconds;

    int renewal_delay =
        (lease_seconds * ONVIF_RENEW_NUMERATOR) / ONVIF_RENEW_DENOMINATOR;
    if (renewal_delay <= 0) renewal_delay = 1;
    if (renewal_delay >= lease_seconds && lease_seconds > 1) {
        renewal_delay = lease_seconds - 1;
    }
    subscription->renewal_time = subscription->creation_time + renewal_delay;
}

/* True when the camera accepted the TCP connection but produced no HTTP
 * response at all. libcurl already retries a dead reused keep-alive connection
 * on a fresh one before reporting these, so they mean the camera itself closed
 * a fresh connection. Refused and timed-out connects are deliberately
 * excluded: a camera that is down or wedged must not be hammered. */
static bool is_transport_drop(const onvif_request_status_t *status) {
    if (!status || status->http_code != 0) return false;
    return status->curl_code == CURLE_GOT_NOTHING ||
           status->curl_code == CURLE_RECV_ERROR ||
           status->curl_code == CURLE_SEND_ERROR;
}

/* PullPoint references can use a camera-local ephemeral port (for example,
 * Tapo creates the subscription on :2020 and serves PullMessages on :1026).
 * Keep the scheme and host pinned to the configured camera while allowing a
 * different valid TCP port on that same host. */
static bool subscription_address_matches_camera_host(const char *camera_url,
                                                      const char *subscription_address) {
    if (!url_is_http_endpoint(camera_url) || !url_is_http_endpoint(subscription_address)) return false;

    CURLU *camera = curl_url();
    CURLU *sub = curl_url();
    char *camera_host = NULL, *camera_scheme = NULL, *camera_port = NULL;
    char *sub_host = NULL, *sub_scheme = NULL, *sub_port = NULL;
    bool ok = false;

    if (!camera || !sub) goto cleanup;
    if (curl_url_set(camera, CURLUPART_URL, camera_url, 0) != CURLUE_OK) goto cleanup;
    if (curl_url_set(sub, CURLUPART_URL, subscription_address, 0) != CURLUE_OK) goto cleanup;

    if (curl_url_get(camera, CURLUPART_HOST, &camera_host, 0) != CURLUE_OK) goto cleanup;
    if (curl_url_get(sub, CURLUPART_HOST, &sub_host, 0) != CURLUE_OK) goto cleanup;
    if (curl_url_get(camera, CURLUPART_SCHEME, &camera_scheme, 0) != CURLUE_OK) goto cleanup;
    if (curl_url_get(sub, CURLUPART_SCHEME, &sub_scheme, 0) != CURLUE_OK) goto cleanup;

    if (curl_url_get(camera, CURLUPART_PORT, &camera_port, CURLU_DEFAULT_PORT) != CURLUE_OK) goto cleanup;
    if (curl_url_get(sub, CURLUPART_PORT, &sub_port, CURLU_DEFAULT_PORT) != CURLUE_OK) goto cleanup;

    char *camera_end = NULL, *sub_end = NULL;
    long camera_number = strtol(camera_port, &camera_end, 10);
    long sub_number = strtol(sub_port, &sub_end, 10);
    ok = camera_end && *camera_end == '\0' && camera_number >= 1 && camera_number <= 65535 &&
         sub_end && *sub_end == '\0' && sub_number >= 1 && sub_number <= 65535 &&
         strcasecmp(camera_scheme, sub_scheme) == 0 &&
         strcasecmp(camera_host, sub_host) == 0;

cleanup:
    if (camera_host) curl_free(camera_host);
    if (camera_scheme) curl_free(camera_scheme);
    if (camera_port) curl_free(camera_port);
    if (sub_host) curl_free(sub_host);
    if (sub_scheme) curl_free(sub_scheme);
    if (sub_port) curl_free(sub_port);
    if (camera) curl_url_cleanup(camera);
    if (sub) curl_url_cleanup(sub);
    return ok;
}

static void onvif_camera_log_id(const char *url, char *out, size_t size) {
    if (url_redact_for_logging(url, out, size) != 0)
        safe_strcpy(out, "[invalid-url]", size, 0);
}

/* Send a subscription-scoped request (PullMessages, Renew, Unsubscribe) to the
 * SubscriptionReference address on the subscription's own connection.
 * Transport drops are retried on the same subscription
 * (ONVIF_PULL_DROP_MAX_ATTEMPTS); the slot lock stays held for the whole burst
 * so the retries land inside the short window in which these cameras accept
 * a connection again. Other cameras are not affected. If transport_dropped is
 * provided, report a final drop to the polling thread so it can continue at
 * the retry interval even after this burst is exhausted. */
static char *send_subscription_request(onvif_subscription_t *subscription,
                                       const char *request_body,
                                       onvif_operation_t operation,
                                       bool *transport_dropped,
                                       const char *expected_camera_url) {
    if (transport_dropped) *transport_dropped = false;
    const char *action = onvif_operation_action(operation);
    if (!subscription || !request_body || !action) return NULL;

    pthread_mutex_t *lock = subscription_lock(subscription);
    if (!lock) return NULL;
    pthread_mutex_lock(lock);
    if (expected_camera_url &&
        (!subscription->active ||
         strcmp(subscription->camera_url, expected_camera_url) != 0)) {
        pthread_mutex_unlock(lock);
        return NULL;
    }

    char *target_url = NULL;
    if (has_uri_scheme_prefix(subscription->subscription_address) ||
        strncmp(subscription->subscription_address, "//", 2) == 0) {
        if (strncmp(subscription->subscription_address, "//", 2) != 0 &&
            (strncasecmp(subscription->subscription_address, "http://", 7) == 0 ||
             strncasecmp(subscription->subscription_address, "https://", 8) == 0))
            target_url = strdup(subscription->subscription_address);
    } else {
        CURLU *resolved = curl_url();
        const char *base = subscription->service_url[0] ?
                           subscription->service_url : subscription->camera_url;
        if (resolved && curl_url_set(resolved, CURLUPART_URL, base, 0) == CURLUE_OK &&
            curl_url_set(resolved, CURLUPART_URL,
                         subscription->subscription_address, 0) == CURLUE_OK) {
            char *curl_target = NULL;
            if (curl_url_get(resolved, CURLUPART_URL, &curl_target, 0) == CURLUE_OK) {
                target_url = strdup(curl_target);
                curl_free(curl_target);
            }
        }
        if (resolved) curl_url_cleanup(resolved);
    }
    if (!target_url || !subscription_address_matches_camera_host(subscription->camera_url,
                                                                  target_url)) {
            free(target_url);
            log_warn("Rejected ONVIF subscription endpoint with a different scheme/host or invalid port");
            pthread_mutex_unlock(lock);
            return NULL;
    }
        char *response = NULL;
        onvif_request_status_t status = {0};
        int attempts = 0;
        if (!subscription->curl) subscription->curl = create_curl_handle();
        if (!subscription->curl) {
            free(target_url);
            pthread_mutex_unlock(lock);
            char camera_id[sizeof(subscription->camera_url)];
            onvif_camera_log_id(subscription->camera_url, camera_id, sizeof(camera_id));
            log_error("Failed to create a curl handle for the ONVIF subscription of %s",
                      camera_id);
            return NULL;
        }
        while (attempts < ONVIF_PULL_DROP_MAX_ATTEMPTS) {
            attempts++;
            response = perform_onvif_request_with_reference_locked(
                subscription->curl, target_url, subscription->username,
                subscription->password, request_body, operation, &status,
                subscription->reference_parameters,
                subscription->reference_namespaces);
            if (response || !is_transport_drop(&status) ||
                attempts >= ONVIF_PULL_DROP_MAX_ATTEMPTS || is_shutdown_initiated()) {
                break;
            }
            log_debug("ONVIF request %s: camera closed the connection without a "
                      "response (curl=%d); retrying on the same subscription "
                      "(attempt %d/%d)",
                      action, (int)status.curl_code, attempts + 1,
                      ONVIF_PULL_DROP_MAX_ATTEMPTS);
            usleep(ONVIF_PULL_DROP_RETRY_DELAY_MS * 1000);
        }
        free(target_url);
        pthread_mutex_unlock(lock);

        if (transport_dropped) {
            *transport_dropped = !response && is_transport_drop(&status);
        }

        /* HTTP-level failures (faults, non-200) were already reported by the
         * core. Polling continues on transport drops; note_pull_failure()
         * warns once if they persist, avoiding error spam every burst. */
        if (!response && status.curl_code != CURLE_OK) {
            log_message(transport_dropped && *transport_dropped ? LOG_LEVEL_DEBUG : LOG_LEVEL_ERROR,
                        "ONVIF request %s failed after %d attempt%s (last attempt: curl=%d, "
                        "HTTP=%ld, elapsed=%.3fs, new connections=%ld): %s",
                        action, attempts, attempts == 1 ? "" : "s",
                        (int)status.curl_code, status.http_code, status.elapsed,
                        status.new_connections, curl_easy_strerror(status.curl_code));
        }
        return response;
}

/* The caller reserves the slot or runs during shutdown. */
static bool renew_subscription_locked(onvif_subscription_t *subscription) {
    const char *request_body =
        "<Renew xmlns=\"http://docs.oasis-open.org/wsn/b-2\">"
        "<TerminationTime>PT600S</TerminationTime>"
        "</Renew>";

    char *response = send_subscription_request(subscription, request_body,
                                               ONVIF_OPERATION_RENEW, NULL, NULL);
    if (!response) return false;

    int lease_seconds = subscription_lease_seconds(response);
    free(response);
    apply_subscription_lease(subscription, lease_seconds);
    char camera_id[sizeof(subscription->camera_url)];
    onvif_camera_log_id(subscription->camera_url, camera_id, sizeof(camera_id));
    log_debug("Renewed ONVIF subscription for %s for %d seconds",
              camera_id, lease_seconds);
    return true;
}

/* Best-effort cleanup. The caller reserves the slot or runs during shutdown. */
static void unsubscribe_subscription_locked(onvif_subscription_t *subscription) {
    if (!subscription) return;
    if (!subscription->active || subscription->subscription_address[0] == '\0') {
        subscription->active = false;
        release_subscription_curl(subscription);
        return;
    }

    const char *request_body =
        "<Unsubscribe xmlns=\"http://docs.oasis-open.org/wsn/b-2\"/>";
    char *response = send_subscription_request(subscription, request_body,
                                               ONVIF_OPERATION_UNSUBSCRIBE, NULL, NULL);
    char camera_id[sizeof(subscription->camera_url)];
    onvif_camera_log_id(subscription->camera_url, camera_id, sizeof(camera_id));
    if (response) {
        free(response);
        log_debug("Unsubscribed ONVIF PullPoint for %s", camera_id);
    } else {
        log_warn("Unable to unsubscribe ONVIF PullPoint for %s",
                 camera_id);
    }
    subscription->active = false;
    release_subscription_curl(subscription);
}

/*
 * Query GetServices and return the event service URL, or NULL if not found.
 * The returned string is heap-allocated; caller must free() it.
 *
 * Previous implementation used substring matching with a 512-byte window,
 * which misses the XAddr when a Service element carries a Capabilities
 * subtree (#374, Reolink Argus). We now parse the response with ezxml and
 * walk Service elements, comparing the Namespace text to the events WSDL.
 */
static char *discover_event_service_url(const char *url, const char *username, const char *password) {
    const char *request_body =
        "<GetServices xmlns=\"http://www.onvif.org/ver10/device/wsdl\">"
            "<IncludeCapability>false</IncludeCapability>"
        "</GetServices>";

    // GetServices is always sent to the standard device management endpoint
    char *response = send_onvif_request(url, username, password, request_body,
                                        "device_service", ONVIF_OPERATION_GET_SERVICES);
    if (!response) {
        log_warn("ONVIF: GetServices to device_service endpoint failed");
        return NULL;
    }

    /* Preserve an untouched copy for the fallback substring heuristic.
     * ezxml_parse_str mutates its input buffer in place. */
    char *response_fallback = strdup(response);

    /* ezxml_parse_str modifies the buffer in place, and the returned
     * tree references it, so keep `response` alive until we copy the
     * matched URL out. */
    ezxml_t xml = ezxml_parse_str(response, strlen(response));
    char *event_url = NULL;

    if (xml) {
        ezxml_t body = child_by_local_name(xml, "Body");
        ezxml_t resp = body ? child_by_local_name(body, "GetServicesResponse") : NULL;

        if (resp) {
            const char *events_ns = "http://www.onvif.org/ver10/events/wsdl";
            for (ezxml_t svc = resp->child; svc && !event_url; svc = svc->ordered) {
                if (strcmp(local_name(svc), "Service") != 0) continue;

                ezxml_t ns    = child_by_local_name(svc, "Namespace");
                ezxml_t xaddr = child_by_local_name(svc, "XAddr");
                const char *ns_text    = ns    ? ezxml_txt(ns)    : "";
                const char *xaddr_text = xaddr ? ezxml_txt(xaddr) : "";

                if (ns_text && xaddr_text && xaddr_text[0] != '\0'
                    && strcmp(ns_text, events_ns) == 0) {
                    event_url = strdup(xaddr_text);
                }
            }
        }
        ezxml_free(xml);
    }

    /* Fallback: the old substring heuristic, for responses that ezxml
     * can't parse (malformed XML from buggy firmware). */
    if (!event_url && response_fallback) {
        const char *events_ns = "http://www.onvif.org/ver10/events/wsdl";
        const char *ns_pos = strstr(response_fallback, events_ns);
        if (ns_pos) {
            const char *xaddr_open[]  = {"<tds:XAddr>", "<XAddr>", NULL};
            const char *xaddr_close[] = {"</tds:XAddr>", "</XAddr>", NULL};
            for (int i = 0; xaddr_open[i] != NULL; i++) {
                const char *tag = strstr(ns_pos, xaddr_open[i]);
                if (tag && (tag - ns_pos) < 2048) {
                    const char *val_start = tag + strlen(xaddr_open[i]);
                    const char *val_end   = strstr(val_start, xaddr_close[i]);
                    if (val_end) {
                        event_url = strndup(val_start, (size_t)(val_end - val_start));
                        break;
                    }
                }
            }
        }
    }

    free(response_fallback);

    if (event_url) {
        log_info("ONVIF: Discovered event service endpoint");
    } else {
        log_warn("ONVIF: Could not find XAddr for events service in GetServices response");
    }

    free(response);
    return event_url;
}

// Find or create subscription for a camera
static void release_subscription_reservation(int slot) {
    pthread_mutex_lock(&subscription_mutex);
    subscriptions[slot].reserved = false;
    if (!subscriptions[slot].active) subscriptions[slot].camera_url[0] = '\0';
    pthread_mutex_unlock(&subscription_mutex);
}

static onvif_subscription_t *get_subscription(const char *url, const char *username, const char *password) {
    pthread_mutex_lock(&subscription_mutex);

    if (subscriptions_shutting_down) {
        pthread_mutex_unlock(&subscription_mutex);
        return NULL;
    }

    int slot = -1;
    // Check if we already have a subscription for this URL
    for (int i = 0; i < subscription_count; i++) {
        if (strcmp(subscriptions[i].camera_url, url) == 0) {
            // Check if subscription is still valid
            time_t now;
            time(&now);

            if (subscriptions[i].active && now < subscriptions[i].expiration_time) {
                if (now >= subscriptions[i].renewal_time) {
                    log_debug("Renewing ONVIF subscription");
                    subscriptions[i].reserved = true;
                    pthread_mutex_unlock(&subscription_mutex);
                    if (!renew_subscription_locked(&subscriptions[i])) {
                        log_warn("ONVIF subscription renewal failed; recreating it");
                        unsubscribe_subscription_locked(&subscriptions[i]);
                        pthread_mutex_lock(&subscription_mutex);
                        slot = i;
                        break;
                    }
                    pthread_mutex_lock(&subscription_mutex);
                    subscriptions[i].reserved = false;
                }

                log_debug("Reusing existing ONVIF subscription");
                pthread_mutex_unlock(&subscription_mutex);
                return &subscriptions[i];
            } else if (!subscriptions[i].active) {
                log_info("ONVIF subscription is inactive, creating new one");
                subscriptions[i].reserved = true;
                slot = i;
                break;
            } else {
                /* We missed the renewal window. Best-effort Unsubscribe avoids
                 * consuming another slot if the camera granted a longer lease
                 * than the timestamps indicated. */
                log_info("ONVIF subscription expired, replacing it");
                subscriptions[i].reserved = true;
                pthread_mutex_unlock(&subscription_mutex);
                unsubscribe_subscription_locked(&subscriptions[i]);
                pthread_mutex_lock(&subscription_mutex);
                slot = i;
                break;
            }
        }
    }

    if (slot < 0) {
        for (int i = 0; i < subscription_count; i++) {
            if (!subscriptions[i].active && !subscriptions[i].reserved) {
                slot = i;
                break;
            }
        }
        if (slot < 0 && subscription_count < MAX_SUBSCRIPTIONS)
            slot = subscription_count++;
        if (slot < 0) {
            log_error("No space for new ONVIF subscription");
            pthread_mutex_unlock(&subscription_mutex);
            return NULL;
        }
        subscriptions[slot].reserved = true;
        safe_strcpy(subscriptions[slot].camera_url, url,
                    sizeof(subscriptions[slot].camera_url), 0);
    }
    pthread_mutex_unlock(&subscription_mutex);

    log_info("Creating new ONVIF subscription");

    /* Tapo accepts a ten-minute lease but rejects both PT1H and an omitted
     * InitialTerminationTime with ter:InvalidArgVal (#567). */
    const char *request_body =
        "<CreatePullPointSubscription xmlns=\"http://www.onvif.org/ver10/events/wsdl\">"
        "<InitialTerminationTime>PT600S</InitialTerminationTime>"
        "</CreatePullPointSubscription>";

    // Dynamically discover the correct event service URL via GetServices.
    // Different vendors use different paths (e.g. Tapo uses "service", Lorex uses "event_service").
    char *discovered_url = discover_event_service_url(url, username, password);
    char *response = NULL;
    char attempted_event_url[512] = {0};

    if (discovered_url) {
        if (subscription_address_matches_camera_host(url, discovered_url)) {
            size_t discovered_len = strlen(discovered_url);
            if (discovered_len < sizeof(attempted_event_url)) {
                log_info("ONVIF: Sending CreatePullPointSubscription to discovered endpoint");
                safe_strcpy(attempted_event_url, discovered_url,
                            sizeof(attempted_event_url), 0);
                response = send_onvif_request_to_url(discovered_url, username, password, request_body,
                                                     ONVIF_OPERATION_CREATE_PULLPOINT);
            } else {
                log_warn("ONVIF: Ignoring oversized discovered event endpoint");
            }
        } else {
            log_warn("ONVIF: Ignoring discovered event endpoint outside the camera host");
        }
        free(discovered_url);
    }

    if (!response) {
        // Fall back through common event service path suffixes
        const char *fallback_services[] = {"service", "event_service", "Events", NULL};
        for (int i = 0; fallback_services[i] && !response; i++) {
            char fallback_url[512];
            int written = snprintf(fallback_url, sizeof(fallback_url),
                                   "%s/onvif/%s", url, fallback_services[i]);
            if (written < 0 || (size_t)written >= sizeof(fallback_url)) {
                continue;
            }
            if (attempted_event_url[0] != '\0' &&
                strcmp(attempted_event_url, fallback_url) == 0) {
                log_debug("ONVIF: Skipping already-tried event endpoint: onvif/%s",
                          fallback_services[i]);
                continue;
            }
            log_info("ONVIF: Trying fallback event endpoint: onvif/%s", fallback_services[i]);
            response = send_onvif_request(url, username, password, request_body,
                                          fallback_services[i], ONVIF_OPERATION_CREATE_PULLPOINT);
            if (response)
                safe_strcpy(attempted_event_url, fallback_url,
                            sizeof(attempted_event_url), 0);
        }
    }

    if (!response) {
        log_error("Failed to create subscription on any endpoint");
        release_subscription_reservation(slot);
        return NULL;
    }

    char *subscription_address = NULL;
    char *reference_parameters = NULL;
    char *reference_namespaces = NULL;
    bool parsed = extract_subscription_endpoint_reference(response, &subscription_address,
                                                          &reference_parameters,
                                                          &reference_namespaces);
    int lease_seconds = subscription_lease_seconds(response);
    free(response);

    if (!parsed) {
        log_error("Failed to extract subscription endpoint reference");
        release_subscription_reservation(slot);
        return NULL;
    }

    if (strlen(subscription_address) >= sizeof(subscriptions[0].subscription_address) ||
        (reference_parameters && strlen(reference_parameters) >=
         sizeof(subscriptions[0].reference_parameters)) ||
        (reference_namespaces && strlen(reference_namespaces) >=
         sizeof(subscriptions[0].reference_namespaces))) {
        log_warn("ONVIF subscription endpoint reference exceeds local capacity; rejecting it");
        free(subscription_address);
        free(reference_parameters);
        free(reference_namespaces);
        release_subscription_reservation(slot);
        return NULL;
    }

    pthread_mutex_lock(&subscription_mutex);

    // If we found a slot, use it
    if (slot >= 0) {
        /* An inactive slot can contain failure state and an old subscription
         * address. Start the replacement with a completely clean record. */
        pthread_mutex_t *slot_lock = subscription_lock(&subscriptions[slot]);
        if (!slot_lock) {
            free(subscription_address);
            free(reference_parameters);
            free(reference_namespaces);
            subscriptions[slot].reserved = false;
            pthread_mutex_unlock(&subscription_mutex);
            return NULL;
        }
        pthread_mutex_lock(slot_lock);
        if (subscriptions[slot].curl) {
            curl_easy_cleanup(subscriptions[slot].curl);
            subscriptions[slot].curl = NULL;
        }
        memset(&subscriptions[slot], 0, sizeof(subscriptions[slot]));

        // Store camera URL, username, and password
        safe_strcpy(subscriptions[slot].camera_url, url, sizeof(subscriptions[slot].camera_url), 0);
        
        safe_strcpy(subscriptions[slot].username, username, sizeof(subscriptions[slot].username), 0);
        
        safe_strcpy(subscriptions[slot].password, password, sizeof(subscriptions[slot].password), 0);
        
        // Store subscription address
        safe_strcpy(subscriptions[slot].subscription_address, subscription_address, 
                sizeof(subscriptions[slot].subscription_address), 0);
        if (reference_parameters)
            safe_strcpy(subscriptions[slot].reference_parameters, reference_parameters,
                        sizeof(subscriptions[slot].reference_parameters), 0);
        if (reference_namespaces)
            safe_strcpy(subscriptions[slot].reference_namespaces, reference_namespaces,
                        sizeof(subscriptions[slot].reference_namespaces), 0);
        safe_strcpy(subscriptions[slot].service_url, attempted_event_url,
                    sizeof(subscriptions[slot].service_url), 0);
        
        // Schedule renewal from the lease the camera actually granted.
        apply_subscription_lease(&subscriptions[slot], lease_seconds);
        subscriptions[slot].active = true;
        pthread_mutex_unlock(slot_lock);
        
        log_info("Successfully created ONVIF subscription (lease: %d seconds)", lease_seconds);
        free(subscription_address);
        free(reference_parameters);
        free(reference_namespaces);
        pthread_mutex_unlock(&subscription_mutex);
        return &subscriptions[slot];
    }

    log_error("No space for new ONVIF subscription");
    free(subscription_address);
    free(reference_parameters);
    free(reference_namespaces);
    pthread_mutex_unlock(&subscription_mutex);
    return NULL;
}

/* Record a failed PullMessages call without immediately discarding the
 * camera-side PullPoint. A transient HTTP disconnect does not invalidate the
 * subscription and recreating on each disconnect exhausts Tapo's low limit. */
static void note_pull_failure(const char *camera_url) {
    pthread_mutex_lock(&subscription_mutex);
    for (int i = 0; i < subscription_count; i++) {
        onvif_subscription_t *subscription = &subscriptions[i];
        if (strcmp(subscription->camera_url, camera_url) != 0 ||
            !subscription->active) {
            continue;
        }

        time_t now = time(NULL);
        if (subscription->consecutive_pull_failures == 0) {
            subscription->first_pull_failure_time = now;
        }
        subscription->consecutive_pull_failures++;
        char camera_id[sizeof(subscription->camera_url)];
        onvif_camera_log_id(camera_url, camera_id, sizeof(camera_id));

        if (subscription->consecutive_pull_failures ==
            ONVIF_PULL_FAILURE_WARN_COUNT) {
            log_warn("ONVIF PullPoint for %s has failed %d consecutive pulls; "
                     "retaining the same subscription",
                     camera_id, subscription->consecutive_pull_failures);
        }

        if (subscription->consecutive_pull_failures >=
                ONVIF_PULL_FAILURE_WARN_COUNT &&
            now - subscription->first_pull_failure_time >=
                ONVIF_PULL_FAILURE_REFRESH_SECONDS) {
            log_warn("ONVIF PullPoint for %s has not completed a pull for %d "
                     "seconds; unsubscribing before refresh",
                     camera_id, ONVIF_PULL_FAILURE_REFRESH_SECONDS);
            subscription->reserved = true;
            pthread_mutex_unlock(&subscription_mutex);
            unsubscribe_subscription_locked(subscription);
            pthread_mutex_lock(&subscription_mutex);
            subscription->reserved = false;
        }
        break;
    }
    pthread_mutex_unlock(&subscription_mutex);
}

static void note_pull_success(const char *camera_url) {
    pthread_mutex_lock(&subscription_mutex);
    for (int i = 0; i < subscription_count; i++) {
        onvif_subscription_t *subscription = &subscriptions[i];
        if (strcmp(subscription->camera_url, camera_url) != 0) continue;

        if (subscription->consecutive_pull_failures >=
            ONVIF_PULL_FAILURE_WARN_COUNT) {
            char camera_id[sizeof(subscription->camera_url)];
            onvif_camera_log_id(camera_url, camera_id, sizeof(camera_id));
            log_info("ONVIF PullPoint for %s recovered after %d failed pulls",
                     camera_id, subscription->consecutive_pull_failures);
        }
        subscription->consecutive_pull_failures = 0;
        subscription->first_pull_failure_time = 0;
        break;
    }
    pthread_mutex_unlock(&subscription_mutex);
}

/* Topics that signal motion/person detection.  "MotionDetector" also covers
 * "CellMotionDetector" (substring); "MotionAlarm" covers VideoSource/MotionAlarm.
 *
 * The second group covers "smart" object-classified detectors (Dahua SMD Plus,
 * ONVIF Profile M, Hikvision smart events, IVS rules).  These carry the object
 * class — Human / Vehicle / Face — in the topic path and/or an ObjectType
 * SimpleItem, which classify_message() extracts so the MQTT/DB label reflects
 * the real class instead of a generic "motion" (discussion #456). */
static bool topic_is_motion(const char *topic_text) {
    if (!topic_text) return false;
    return strstr(topic_text, "MotionDetector") != NULL ||
           strstr(topic_text, "VideoAnalytics/Motion") != NULL ||
           strstr(topic_text, "MotionAlarm") != NULL ||
           strstr(topic_text, "PeopleDetector") != NULL ||
           strstr(topic_text, "TPSmartEventDetector") != NULL ||
           strstr(topic_text, "SmartMotion")   != NULL ||
           strstr(topic_text, "ObjectDetect")  != NULL ||
           strstr(topic_text, "HumanDetect")   != NULL ||
           strstr(topic_text, "FaceDetect")    != NULL ||
           strstr(topic_text, "VehicleDetect") != NULL ||
           strstr(topic_text, "PeopleDetect")  != NULL ||
           strstr(topic_text, "CrossLineDetector")   != NULL ||
           strstr(topic_text, "CrossRegionDetector") != NULL ||
           strstr(topic_text, "IntrusionDetector")   != NULL ||
           strstr(topic_text, "FieldDetector")       != NULL ||
           strstr(topic_text, "LineDetector")        != NULL;
}

/* Case-insensitive substring test (strcasestr is GNU-only, avoid it). */
static bool ci_contains(const char *hay, const char *needle) {
    if (!hay || !needle) return false;
    size_t nl = strlen(needle);
    if (nl == 0) return false;
    for (; *hay; hay++) {
        if (strncasecmp(hay, needle, nl) == 0) return true;
    }
    return false;
}

/* Map a raw camera object-class token (from an ObjectType SimpleItem or a
 * topic path) to a normalized detection label consistent with the object
 * detectors ("person", "car", …).  Returns NULL when the token names no
 * recognized class so the caller can fall back to the generic "motion". */
static const char *normalize_object_class(const char *raw) {
    if (!raw || !raw[0]) return NULL;
    if (ci_contains(raw, "human") || ci_contains(raw, "people") ||
        ci_contains(raw, "person") || ci_contains(raw, "pedestrian")) {
        return "person";
    }
    /* Handle motorized two-wheelers before broad bicycle/cycle matching. */
    if (ci_contains(raw, "motorcycle") || ci_contains(raw, "motorbike")) {
        return "vehicle";
    }
    /* Check non-motorized before vehicle so "NonMotor" isn't caught by "motor". */
    if (ci_contains(raw, "nonmotor") || ci_contains(raw, "bicycle") ||
        ci_contains(raw, "bike") || ci_contains(raw, "cycle")) {
        return "bicycle";
    }
    if (ci_contains(raw, "vehicle") || ci_contains(raw, "car")) {
        return "vehicle";
    }
    if (ci_contains(raw, "face")) {
        return "face";
    }
    if (ci_contains(raw, "animal") || ci_contains(raw, "dog") ||
        ci_contains(raw, "cat") || ci_contains(raw, "pet")) {
        return "animal";
    }
    return NULL;
}

/* Recursively search a NotificationMessage for a SimpleItem that names an
 * object class inside a Data element, returning the normalized class (or NULL).
 * Alongside ObjectType-style values, Tapo reports asserted class flags such as
 * Name="IsVehicle" Value="true" on TPSmartEventDetector topics (#567). */
static bool is_recognized_class_flag_name(const char *name) {
    if (!name) return false;
    static const char *const flags[] = {
        "Human", "People", "Person", "Pedestrian", "Vehicle", "Car",
        "Motorcycle", "Motorbike", "NonMotor", "Bicycle", "Bike", "Cycle",
        "Face", "Animal", "Dog", "Cat", "Pet",
        "IsHuman", "IsPeople", "IsPerson", "IsPedestrian", "IsVehicle", "IsCar",
        "IsMotorcycle", "IsMotorbike", "IsNonMotor", "IsBicycle", "IsBike",
        "IsCycle", "IsFace", "IsAnimal", "IsDog", "IsCat", "IsPet",
        "HumanDetect", "PeopleDetect", "PersonDetect", "VehicleDetect",
        "CarDetect", "FaceDetect", "AnimalDetect", NULL
    };
    for (size_t i = 0; flags[i]; ++i) {
        if (strcasecmp(name, flags[i]) == 0) return true;
    }
    return false;
}

static const char *find_object_class(ezxml_t node, bool in_data) {
    for (ezxml_t c = node ? node->child : NULL; c; c = c->ordered) {
        bool inside = in_data || strcmp(local_name(c), "Data") == 0;
        if (inside && strcmp(local_name(c), "SimpleItem") == 0) {
            const char *name = ezxml_attr(c, "Name");
            if (name && (strcasecmp(name, "ObjectType") == 0 ||
                         strcasecmp(name, "ObjectClass") == 0 ||
                         strcasecmp(name, "Type") == 0)) {
                const char *cls = normalize_object_class(ezxml_attr(c, "Value"));
                if (cls) return cls;
            }
            const char *value = ezxml_attr(c, "Value");
            if (name && value &&
                (strcasecmp(value, "true") == 0 || strcmp(value, "1") == 0) &&
                is_recognized_class_flag_name(name)) {
                const char *cls = normalize_object_class(name);
                if (cls) return cls;
            }
        }
        const char *deep = find_object_class(c, inside);
        if (deep) return deep;
    }
    return NULL;
}

/* Best label for an asserted NotificationMessage: prefer an ObjectType class,
 * then a class named in the topic path, else the generic "motion". */
static const char *classify_message(ezxml_t msg, const char *topic_text) {
    const char *cls = find_object_class(msg, false);
    if (cls) return cls;
    cls = normalize_object_class(topic_text);
    return cls ? cls : "motion";
}

/* A property value counts as asserted only for boolean-true spellings. */
static bool value_is_true(const char *value) {
    if (!value) return false;
    return strcasecmp(value, "true") == 0 || strcmp(value, "1") == 0;
}

/* Ignore truthy metadata (for example Gain=1) in motion-topic Data sections. */
static bool is_asserted_item_name(const char *name) {
    if (!name) return false;
    static const char *const state_names[] = {
        "IsMotion", "Motion", "MotionAlarm", "State", "Active", "Detected", "IsActive",
        "Person", "People", "Human", "Vehicle", "Car", "Bicycle", "Face", "Animal", NULL
    };
    for (size_t i = 0; state_names[i]; ++i) {
        if (strcasecmp(name, state_names[i]) == 0) return true;
    }
    return is_recognized_class_flag_name(name);
}

/*
 * Return true if any SimpleItem beneath `node` that sits inside a Data
 * element has a truthy event-state Value. `in_data` tracks whether an ancestor
 * was a Data element — Source/Key sections also carry SimpleItems, but their
 * Values are configuration tokens (often literally "1"), not event state.
 */
static bool data_has_asserted_item(ezxml_t node, bool in_data) {
    for (ezxml_t c = node ? node->child : NULL; c; c = c->ordered) {
        bool inside = in_data || strcmp(local_name(c), "Data") == 0;
        if (inside && strcmp(local_name(c), "SimpleItem") == 0 &&
            is_asserted_item_name(ezxml_attr(c, "Name")) &&
            value_is_true(ezxml_attr(c, "Value"))) {
            return true;
        }
        if (data_has_asserted_item(c, inside)) return true;
    }
    return false;
}

/*
 * Recursively walk the parsed response looking for NotificationMessage
 * elements whose Topic is motion-related AND whose Data payload asserts the
 * state (Value="true"/"1").  Returns true if any such event is present and
 * fills `label` (size `label_sz`) with the best class found: a specific object
 * class (person, vehicle, …) is preferred over the generic "motion" fallback,
 * so a camera that emits both a plain motion event and a Human event is
 * reported as "person".
 */
static bool xml_find_active_motion(ezxml_t node, char *label, size_t label_sz) {
    bool any = false;
    for (ezxml_t c = node ? node->child : NULL; c; c = c->ordered) {
        if (strcmp(local_name(c), "NotificationMessage") == 0) {
            ezxml_t topic = child_by_local_name(c, "Topic");
            const char *topic_text = topic ? ezxml_txt(topic) : NULL;
            if (topic_is_motion(topic_text) && data_has_asserted_item(c, false)) {
                any = true;
                const char *cls = classify_message(c, topic_text);
                bool have_specific = label[0] != '\0' && strcmp(label, "motion") != 0;
                if (cls && strcmp(cls, "motion") != 0) {
                    if (!have_specific) safe_strcpy(label, cls, label_sz, 0);
                } else if (label[0] == '\0') {
                    safe_strcpy(label, "motion", label_sz, 0);
                }
            }
        } else if (xml_find_active_motion(c, label, label_sz)) {
            any = true;
        }
    }
    return any;
}

/* Some cameras send an unclosed Motion element. Keep that compatibility case
 * without accepting a truthy Value from an unrelated field in the payload. */
static bool malformed_motion_element_asserted(const char *response) {
    for (const char *tag = strchr(response, '<'); tag; tag = strchr(tag + 1, '<')) {
        const char *end = tag + 1;
        while (*end && *end != '<' && *end != '>') end++;
        const char *name = tag + 1;
        const char *name_end = name;
        while (name_end < end && *name_end != ' ' && *name_end != '\t' &&
               *name_end != '\r' && *name_end != '\n' && *name_end != '/') {
            name_end++;
        }
        const char *local = memchr(name, ':', (size_t)(name_end - name));
        local = local ? local + 1 : name;
        if (name_end - local != 6 || strncasecmp(local, "Motion", 6) != 0) continue;

        const char *p = name_end;
        while (p < end) {
            while (p < end && isspace((unsigned char)*p)) p++;
            if (p >= end || *p == '/' || *p == '>') break;

            const char *attr_name = p;
            while (p < end && !isspace((unsigned char)*p) && *p != '=' && *p != '/' && *p != '>') p++;
            const char *attr_name_end = p;

            while (p < end && isspace((unsigned char)*p)) p++;
            if (p >= end || *p != '=') {
                while (p < end && !isspace((unsigned char)*p) && *p != '>') p++;
                continue;
            }
            p++; /* skip '=' */
            while (p < end && isspace((unsigned char)*p)) p++;
            if (p >= end || (*p != '"' && *p != '\'')) continue;

            char quote = *p++;
            const char *attr_val = p;
            while (p < end && *p != quote) p++;
            if (p >= end) break;
            const char *attr_val_end = p;

            if ((attr_name_end - attr_name) == 5 &&
                strncasecmp(attr_name, "Value", 5) == 0) {
                size_t vlen = (size_t)(attr_val_end - attr_val);
                if ((vlen == 4 && strncasecmp(attr_val, "true", 4) == 0) ||
                    (vlen == 1 && attr_val[0] == '1')) {
                    return true;
                }
            }

            p++; /* skip closing quote */
        }
    }
    return false;
}

/*
 * Check for an ACTIVE motion event in a PullMessages response.
 *
 * Every WS-BaseNotification message names its topic and properties whether
 * the state is asserted or not — a "motion ended" payload still contains
 * <tt:SimpleItem Name="IsMotion" Value="false"/>.  The old substring checks
 * (strstr "IsMotion" etc.) therefore flagged idle/ended messages as motion
 * (issue #451).  Parse the XML and require Value="true"/"1" inside the Data
 * section of a motion-topic NotificationMessage instead.
 */
static bool has_motion_event(const char *response, char *label, size_t label_sz) {
    if (label && label_sz > 0) label[0] = '\0';
    if (!response) return false;

    /* Cheap pre-filter: skip XML parsing when the payload names no
     * motion-related topic at all (the common empty-response case).  "Detect"
     * and "ObjectType" cover the smart-detector topics (HumanDetect,
     * VehicleDetect, ObjectDetect, …) added for discussion #456. */
    if (!strstr(response, "Motion") && !strstr(response, "PeopleDetector") &&
        !strstr(response, "IsPeople") && !strstr(response, "Detect") &&
        !strstr(response, "ObjectType")) {
        return false;
    }

    /* ezxml_parse_str modifies the buffer in place; work on a copy since
     * our parameter is const. */
    char *copy = strdup(response);
    if (!copy) return false;

    ezxml_t xml = ezxml_parse_str(copy, strlen(copy));
    const char *parse_error = xml ? ezxml_error(xml) : NULL;
    if (!xml || (parse_error && parse_error[0] != '\0')) {
        /* Malformed XML from buggy firmware: accept only an asserted Motion
         * element, not an unrelated truthy metadata Value. */
        log_debug("ONVIF Detection: failed to parse PullMessages response, using fallback heuristic");
        if (xml) {
            ezxml_free(xml);
        }
        bool fallback = malformed_motion_element_asserted(response);
        if (fallback && label && label_sz > 0) safe_strcpy(label, "motion", label_sz, 0);
        free(copy);
        return fallback;
    }

    bool motion = xml_find_active_motion(xml, label, label_sz);

    ezxml_free(xml);
    free(copy);
    return motion;
}

/* Parse and persist LPR reads before the legacy motion parser reduces the
 * PullMessages envelope to detection_result_t. Plate values are never logged
 * or placed on the general detection/event path. */
static void store_lpr_notifications(const char *response, const char *stream_name) {
    if (!response || !stream_name || !stream_name[0]) return;
    if (!ci_contains(response, "LicensePlate") && !ci_contains(response, "ANPR") &&
        !ci_contains(response, "ALPR") && !ci_contains(response, "LPR")) return;

    enum { MAX_LPR_EVENTS_PER_PULL = 100 };
    const size_t events_size = MAX_LPR_EVENTS_PER_PULL * sizeof(onvif_lpr_event_t);
    onvif_lpr_event_t *events = calloc(MAX_LPR_EVENTS_PER_PULL, sizeof(*events));
    if (!events) return;
    int count = onvif_parse_lpr_events(response, events, MAX_LPR_EVENTS_PER_PULL);
    if (count <= 0) {
        secure_clear(events, events_size);
        free(events);
        return;
    }

    if (!lpr_crypto_key_available()) {
        static atomic_bool warned_missing_key = false;
        if (!atomic_exchange(&warned_missing_key, true)) {
            log_warn("LPR reads are available but protected storage is disabled: "
                     "LIGHTNVR_LPR_MASTER_KEY_HEX is not configured");
        }
        secure_clear(events, events_size);
        free(events);
        return;
    }

    stream_config_t stream;
    if (get_stream_config_by_name(stream_name, &stream) != 0 ||
        !stream.camera_uuid[0]) {
        log_warn("Cannot associate ONVIF LPR notifications with stream identity");
        secure_clear(events, events_size);
        free(events);
        return;
    }

    uint64_t recording_id = get_current_recording_id_for_stream(stream_name);
    for (int i = 0; i < count; ++i) {
        lpr_read_input_t input = {0};
        if (db_lpr_read_from_onvif(stream.camera_uuid, stream_name,
                                   &events[i], &input) != 0) {
            log_debug("Ignored incomplete ONVIF LPR notification for %s", stream_name);
            secure_clear(&input, sizeof(input));
            continue;
        }
        input.recording_id = recording_id;
        char read_uuid[LPR_READ_UUID_SIZE] = {0};
        int stored = db_lpr_read_insert(&input, read_uuid);
        if (stored < 0) {
            log_warn("Failed to persist protected ONVIF LPR read for %s", stream_name);
        } else if (stored == 0) {
            log_debug("Stored protected ONVIF LPR read for %s", stream_name);
            char event_error[256] = {0};
            if (event_producer_publish_lpr_read(
                    stream.camera_uuid, stream_name, read_uuid, input.source,
                    (time_t)(input.observed_at_ms / 1000), event_error,
                    sizeof(event_error)) != 0) {
                log_debug("Protected LPR event enqueue failed for %s: %s",
                          stream_name, event_error);
            }
        }
        secure_clear(&input, sizeof(input));
        secure_clear(read_uuid, sizeof(read_uuid));
    }
    secure_clear(events, events_size);
    free(events);
}

/**
 * Initialize the ONVIF detection system
 */
int init_onvif_detection_system(void) {
    pthread_mutex_lock(&curl_mutex);

    if (initialized && curl_handle) {
        log_info("ONVIF detection system already initialized");
        pthread_mutex_unlock(&curl_mutex);
        return 0;  // Already initialized and curl handle is present
    }

    // If we have a curl handle but initialized is false, clean it up first
    if (curl_handle) {
        log_warn("ONVIF detection system has a curl handle but is marked as uninitialized, cleaning up");
        curl_easy_cleanup(curl_handle);
        curl_handle = NULL;
    }

    // Initialize curl global (thread-safe, idempotent)
    if (curl_init_global() != 0) {
        log_error("Failed to initialize curl global");
        pthread_mutex_unlock(&curl_mutex);
        return -1;
    }

    curl_handle = create_curl_handle();
    if (!curl_handle) {
        log_error("Failed to initialize curl handle");
        // Note: Don't call curl_global_cleanup() here - it's managed centrally
        pthread_mutex_unlock(&curl_mutex);
        return -1;
    }

    if (getenv("LIGHTNVR_ONVIF_CURL_TRACE")) {
        log_info("ONVIF detection: libcurl trace enabled (LIGHTNVR_ONVIF_CURL_TRACE)");
    }

    // Initialize subscriptions
    pthread_mutex_lock(&subscription_mutex);
    subscription_count = 0;
    for (int i = 0; i < MAX_SUBSCRIPTIONS; i++)
        release_subscription_curl(&subscriptions[i]);
    memset(subscriptions, 0, sizeof(subscriptions));
    subscriptions_shutting_down = false;
    pthread_mutex_unlock(&subscription_mutex);

    initialized = true;
    pthread_mutex_unlock(&curl_mutex);
    log_info("ONVIF detection system initialized successfully");
    return 0;
}

/**
 * Shutdown the ONVIF detection system
 */
void shutdown_onvif_detection_system(void) {
    log_info("Shutting down ONVIF detection system (initialized: %s, curl_handle: %p)",
             initialized ? "yes" : "no", (void*)curl_handle);

    /* Stop new lookups and release active camera-side PullPoints while the
     * shared CURL handle is still usable. The normal shutdown sequence has
     * already joined detection threads before reaching this function. */
    pthread_mutex_lock(&subscription_mutex);
    subscriptions_shutting_down = true;
    if (initialized && curl_handle) {
        for (int i = 0; i < subscription_count; i++) {
            unsubscribe_subscription_locked(&subscriptions[i]);
        }
    }
    for (int i = 0; i < MAX_SUBSCRIPTIONS; i++)
        release_subscription_curl(&subscriptions[i]);
    subscription_count = 0;
    memset(subscriptions, 0, sizeof(subscriptions));
    pthread_mutex_unlock(&subscription_mutex);

    pthread_mutex_lock(&curl_mutex);

    // Cleanup curl handle if it exists
    if (curl_handle) {
        log_info("Cleaning up curl handle");
        curl_easy_cleanup(curl_handle);
        curl_handle = NULL;
    }

    // Note: Don't call curl_global_cleanup() here - it's managed centrally in curl_init.c
    // The global cleanup will happen at program shutdown

    initialized = false;
    pthread_mutex_unlock(&curl_mutex);
    log_info("ONVIF detection system shutdown complete");
}

/**
 * Detect motion using ONVIF events
 */
int detect_motion_onvif(const char *onvif_url, const char *username, const char *password,
                       detection_result_t *result, const char *stream_name) {
    // Check if we're in shutdown mode
    if (is_shutdown_initiated()) {
        log_info("ONVIF Detection: System shutdown in progress, skipping detection");
        return -1;
    }
    
    // Initialize result to empty at the beginning to prevent segmentation fault
    if (result) {
        memset(result, 0, sizeof(detection_result_t));
    } else {
        log_error("ONVIF Detection: NULL result pointer provided");
        return -1;
    }
    
    if (!initialized || !curl_handle) {
        log_error("ONVIF detection system not initialized");
        return -1;
    }

    // Validate parameters - allow empty credentials (empty strings) but not NULL pointers
    // cppcheck-suppress knownConditionTrueFalse
    if (!onvif_url || !username || !password || !result) {
        log_error("Invalid parameters for detect_motion_onvif (NULL pointers not allowed)");
        return -1;
    }

    // Log credential status for debugging
    if (strlen(username) == 0 || strlen(password) == 0) {
        log_debug("ONVIF Detection: Using camera without authentication (empty credentials)");
    } else {
        log_debug("ONVIF Detection: Using camera with authentication");
    }

    log_debug("ONVIF Detection: Starting detection");
    log_debug("ONVIF Detection: Stream name: %s", stream_name);

    pthread_mutex_t *camera_lock = camera_lock_for_url(onvif_url);
    pthread_mutex_lock(camera_lock);
    onvif_subscription_t *subscription = get_subscription(onvif_url, username, password);
    if (!subscription) {
        pthread_mutex_unlock(camera_lock);
        log_error("Failed to get subscription");
        return -1;
    }

    // Create pull messages request
    const char *request_body =
        "<PullMessages xmlns=\"http://www.onvif.org/ver10/events/wsdl\">\n"
        "  <Timeout>PT5S</Timeout>\n"
        "  <MessageLimit>100</MessageLimit>\n"
        "</PullMessages>";

    // Capture the event timestamp BEFORE the (potentially up-to-5-second)
    // PullMessages roundtrip and the subsequent zone-filter step.  Storing
    // this as the detection timestamp keeps detections.timestamp aligned with
    // when the motion window started, not when inference + DB write finished.
    time_t event_timestamp = time(NULL);

    // Use the same camera-host check and relative-path handling as Renew/Unsubscribe.
    bool transport_dropped = false;
    char *response = send_subscription_request(subscription, request_body,
                                               ONVIF_OPERATION_PULL_MESSAGES,
                                               &transport_dropped, onvif_url);

    if (!response) {
        if (!transport_dropped) log_error("Failed to pull messages from subscription");
        note_pull_failure(onvif_url);
        pthread_mutex_unlock(camera_lock);
        return transport_dropped ? ONVIF_DETECT_RETRY : -1;
    }

    note_pull_success(onvif_url);
    pthread_mutex_unlock(camera_lock);

    // Check for motion events, capturing the object class (person/vehicle/…)
    // when the camera reports a smart detection rather than plain motion.
    char event_label[MAX_LABEL_LENGTH] = {0};
    store_lpr_notifications(response, stream_name);
    bool motion_detected = has_motion_event(response, event_label, sizeof(event_label));
    secure_clear(response, strlen(response));
    free(response);

    if (motion_detected) {
        if (event_label[0] == '\0') safe_strcpy(event_label, "motion", sizeof(event_label), 0);
        log_info("ONVIF Detection: %s detected for %s", event_label, stream_name);

        // Create a single detection that covers the whole frame
        result->count = 1;
        safe_strcpy(result->detections[0].label, event_label, MAX_LABEL_LENGTH, 0);
        result->detections[0].confidence = 1.0f;
        result->detections[0].x = 0.0f;
        result->detections[0].y = 0.0f;
        result->detections[0].width = 1.0f;
        result->detections[0].height = 1.0f;

        // Filter detections by zones before storing
        if (stream_name && stream_name[0] != '\0') {
            log_debug("ONVIF Detection: Filtering detections by zones for stream %s", stream_name);
            int filter_ret = filter_detections_by_zones(stream_name, result);
            if (filter_ret != 0) {
                log_warn("Failed to filter detections by zones, storing all detections");
            }

            /* ONVIF detections are produced by this polling thread rather than
             * report_detections(), so they must resolve annotation-mode linkage
             * here. Passing 0 unconditionally left every ONVIF detection with a
             * NULL recording_id even while a continuous segment was active
             * (issue #547). A segment-close backfill in db_recordings covers the
             * narrow rotation window where the writer has not published its new
             * recording ID yet. */
            uint64_t recording_id =
                get_current_recording_id_for_stream(stream_name);
            store_detections_in_db(stream_name, result, event_timestamp,
                                   recording_id);

            // Enqueue the event and trigger recording if detections remain.
            if (result->count > 0) {
                char event_error[256] = {0};
                if (event_producer_publish_detection_for_stream(
                        stream_name, result, event_timestamp,
                        event_error, sizeof(event_error)) != 0) {
                    log_debug("ONVIF Detection: Event enqueue failed for %s: %s",
                              stream_name, event_error);
                }
                process_motion_event(stream_name, true, event_timestamp, false);
            }
        } else {
            log_warn("No stream name provided, skipping database storage");
        }
    } else {
        log_debug("ONVIF Detection: No asserted motion event for %s", stream_name);
        result->count = 0;

        // Notify motion recording that motion has ended
        if (stream_name && stream_name[0] != '\0') {
            process_motion_event(stream_name, false, event_timestamp, false);
        }
    }

    return 0;
}
