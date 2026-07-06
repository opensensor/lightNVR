#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <errno.h>
#include <unistd.h>
#include <sys/stat.h>
#include <curl/curl.h>
#include <cjson/cJSON.h>
#include <pthread.h>
#include <time.h>

#include "core/logger.h"
#include "core/config.h"
#include "core/curl_init.h"
#include "core/shutdown_coordinator.h"
#include "core/mqtt_client.h"
#include "utils/strings.h"
#include "video/onvif_detection.h"
#include "video/onvif_soap.h"
#include "video/detection_result.h"
#include "video/cross_stream_motion_trigger.h"
#include "video/zone_filter.h"
#include "database/db_detections.h"
#include "ezxml.h"

/* WS-Addressing action URIs for the ONVIF operations this file emits.
 * Strict ONVIF servers (Reolink, some Vivotek) reject requests that don't
 * carry a matching wsa:Action header and `action=` Content-Type parameter —
 * see #374. */
#define ONVIF_ACTION_GET_SERVICES        "http://www.onvif.org/ver10/device/wsdl/GetServices"
#define ONVIF_ACTION_CREATE_PULLPOINT    "http://www.onvif.org/ver10/events/wsdl/EventPortType/CreatePullPointSubscriptionRequest"
#define ONVIF_ACTION_PULL_MESSAGES       "http://www.onvif.org/ver10/events/wsdl/PullPointSubscription/PullMessagesRequest"

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

// Structure to hold ONVIF subscription information
typedef struct {
    char camera_url[512];           // URL of the camera (used as the key for lookup)
    char subscription_address[512]; // Address returned by CreatePullPointSubscription (full URL)
    char username[64];              // Username for authentication
    char password[64];              // Password for authentication
    time_t creation_time;
    time_t expiration_time;
    bool active;
} onvif_subscription_t;

// Hash map to store subscriptions by URL
#define MAX_SUBSCRIPTIONS 100
static onvif_subscription_t subscriptions[MAX_SUBSCRIPTIONS];
static int subscription_count = 0;
static pthread_mutex_t subscription_mutex = PTHREAD_MUTEX_INITIALIZER;

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

/*
 * Create ONVIF SOAP request with WS-Security (if credentials provided) and
 * WS-Addressing headers. `action` and `to` are the WS-Addressing action URI
 * and destination URL respectively; both are required (strict ONVIF servers
 * such as Reolink reject requests missing either — #374).
 */
static char *create_onvif_request(const char *username, const char *password,
                                  const char *request_body,
                                  const char *action, const char *to) {
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

    /* Build a WS-Addressing fragment if an action was provided. Using
     * `urn:uuid:<v4>` for MessageID and leaving mustUnderstand off on To/Action
     * — cameras that ignore WS-Addressing shouldn't then fault because they
     * "didn't understand" an optional header. */
    char uuid[48] = {0};
    generate_uuid(uuid, sizeof(uuid));

    const char *safe_action = action ? action : "";
    const char *safe_to     = to     ? to     : "";

    /* The WS-Addressing block inflates the envelope; reserve enough for
     * MessageID (urn:uuid:… = ~45 chars), the action URI, the destination URL,
     * the security header, and the body. 2 KB of slack above those sizes is
     * plenty. */
    size_t envelope_size = strlen(request_body) + strlen(security_header)
                           + strlen(safe_action) + strlen(safe_to) + 2048;
    char *soap_request = malloc(envelope_size);
    if (!soap_request) {
        free(security_header);
        return NULL;
    }

    snprintf(soap_request, envelope_size,
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>"
        "<s:Envelope xmlns:s=\"http://www.w3.org/2003/05/soap-envelope\" "
                   "xmlns:wsa=\"http://www.w3.org/2005/08/addressing\">"
        "<s:Header>"
            "<wsa:MessageID>urn:uuid:%s</wsa:MessageID>"
            "<wsa:To>%s</wsa:To>"
            "<wsa:Action>%s</wsa:Action>"
            "%s"
        "</s:Header>"
        "<s:Body xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" "
                "xmlns:xsd=\"http://www.w3.org/2001/XMLSchema\">%s</s:Body>"
        "</s:Envelope>",
        uuid, safe_to, safe_action, security_header, request_body);

    free(security_header);
    return soap_request;
}

/*
 * Send ONVIF request to a full URL (bypassing the base URL + /onvif/ + service
 * construction). `action` is the WS-Addressing action URI; it's also added as
 * the `action=` parameter on the Content-Type header per SOAP 1.2, which
 * strict ONVIF servers require (#374).
 */
static char *send_onvif_request_to_url(const char *full_url, const char *username,
                                       const char *password, const char *request_body,
                                       const char *action) {
    if (!initialized || !curl_handle) {
        log_error("ONVIF detection system not initialized");
        return NULL;
    }

    log_debug("ONVIF Detection: Sending request to %s", full_url);

    /* Use full_url as the WS-Addressing To, which is exactly what the request
     * is being POSTed to. */
    char *soap_request = create_onvif_request(username, password, request_body,
                                              action, full_url);
    if (!soap_request) {
        log_error("Failed to create ONVIF request");
        return NULL;
    }

    pthread_mutex_lock(&curl_mutex);

    // Set up curl
    curl_easy_setopt(curl_handle, CURLOPT_URL, full_url);
    curl_easy_setopt(curl_handle, CURLOPT_POSTFIELDS, soap_request);
    curl_easy_setopt(curl_handle, CURLOPT_POSTFIELDSIZE, strlen(soap_request));

    /* Content-Type carries the SOAP 1.2 action parameter in addition to the
     * charset. Fall back to the plain type when no action is provided. */
    char content_type[512];
    if (action && action[0] != '\0') {
        snprintf(content_type, sizeof(content_type),
                 "Content-Type: application/soap+xml; charset=utf-8; action=\"%s\"",
                 action);
    } else {
        snprintf(content_type, sizeof(content_type),
                 "Content-Type: application/soap+xml; charset=utf-8");
    }

    struct curl_slist *headers = NULL;
    headers = curl_slist_append(headers, content_type);
    curl_easy_setopt(curl_handle, CURLOPT_HTTPHEADER, headers);

    // Set up response buffer
    memory_struct_t chunk;
    chunk.memory = malloc(1);
    chunk.size = 0;

    curl_easy_setopt(curl_handle, CURLOPT_WRITEFUNCTION, write_memory_callback);
    curl_easy_setopt(curl_handle, CURLOPT_WRITEDATA, (void *)&chunk);
    curl_easy_setopt(curl_handle, CURLOPT_TIMEOUT, 10);

    // Perform request
    CURLcode res = curl_easy_perform(curl_handle);

    // Clean up request
    free(soap_request);
    curl_slist_free_all(headers);

    // Check for errors
    if (res != CURLE_OK) {
        log_error("ONVIF Detection: curl_easy_perform() failed: %s", curl_easy_strerror(res));
        free(chunk.memory);
        pthread_mutex_unlock(&curl_mutex);
        return NULL;
    }

    // Get HTTP response code
    long http_code = 0;
    curl_easy_getinfo(curl_handle, CURLINFO_RESPONSE_CODE, &http_code);

    if (http_code != 200) {
        log_error("ONVIF request to %s failed with HTTP code %ld", full_url, http_code);
        if (chunk.size > 0) {
            onvif_log_soap_fault(chunk.memory, chunk.size, "ONVIF Detection");
        }
        free(chunk.memory);
        pthread_mutex_unlock(&curl_mutex);
        return NULL;
    }

    pthread_mutex_unlock(&curl_mutex);
    return chunk.memory;
}

// Send ONVIF request and get response
static char *send_onvif_request(const char *url, const char *username, const char *password,
                               const char *request_body, const char *service,
                               const char *action) {
    if (!initialized || !curl_handle) {
        log_error("ONVIF detection system not initialized");
        return NULL;
    }

    // Create full URL
    char full_url[512];
    snprintf(full_url, sizeof(full_url), "%s/onvif/%s", url, service);

    return send_onvif_request_to_url(full_url, username, password, request_body, action);
}

// Extract subscription address from response
static char *extract_subscription_address(const char *response) {
    if (!response) return NULL;

    // Try different namespace prefixes
    const char *patterns[] = {
        "<wsa:Address>", "</wsa:Address>",
        "<wsa5:Address>", "</wsa5:Address>",
        "<Address>", "</Address>"
    };

    for (int i = 0; i < 3; i++) {
        const char *start = strstr(response, patterns[(ptrdiff_t)i*2]);
        const char *end = strstr(response, patterns[(ptrdiff_t)i*2+1]);
        
        if (start && end) {
            start += strlen(patterns[(ptrdiff_t)i * 2]);
            int length = (int)(end - start);
            
            return strndup(start, length);
        }
    }

    return NULL;
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
                                        "device_service", ONVIF_ACTION_GET_SERVICES);
    if (!response) {
        log_warn("ONVIF: GetServices to device_service endpoint failed");
        return NULL;
    }

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
    if (!event_url) {
        const char *events_ns = "http://www.onvif.org/ver10/events/wsdl";
        const char *ns_pos = strstr(response, events_ns);
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

    if (event_url) {
        log_info("ONVIF: Discovered event service URL: %s", event_url);
    } else {
        log_warn("ONVIF: Could not find XAddr for events service in GetServices response");
    }

    free(response);
    return event_url;
}

// Find or create subscription for a camera
static onvif_subscription_t *get_subscription(const char *url, const char *username, const char *password) {
    pthread_mutex_lock(&subscription_mutex);

    // Check if we already have a subscription for this URL
    for (int i = 0; i < subscription_count; i++) {
        if (strcmp(subscriptions[i].camera_url, url) == 0) {
            // Check if subscription is still valid
            time_t now;
            time(&now);
            
            if (subscriptions[i].active && now < subscriptions[i].expiration_time) {
                log_debug("Reusing existing ONVIF subscription for %s", url);
                pthread_mutex_unlock(&subscription_mutex);
                return &subscriptions[i];
            } else {
                // Subscription expired, remove it
                log_info("ONVIF subscription for %s expired, creating new one", url);
                subscriptions[i].active = false;
                break;
            }
        }
    }

    log_info("Creating new ONVIF subscription for %s", url);

    // Create a new subscription
    const char *request_body =
        "<CreatePullPointSubscription xmlns=\"http://www.onvif.org/ver10/events/wsdl\">\n"
        "  <InitialTerminationTime>PT1H</InitialTerminationTime>\n"
        "</CreatePullPointSubscription>";

    // Dynamically discover the correct event service URL via GetServices.
    // Different vendors use different paths (e.g. Tapo uses "service", Lorex uses "event_service").
    char *discovered_url = discover_event_service_url(url, username, password);
    char *response = NULL;

    if (discovered_url) {
        log_info("ONVIF: Sending CreatePullPointSubscription to discovered URL: %s", discovered_url);
        response = send_onvif_request_to_url(discovered_url, username, password, request_body,
                                              ONVIF_ACTION_CREATE_PULLPOINT);
        free(discovered_url);
    }

    if (!response) {
        // Fall back through common event service path suffixes
        const char *fallback_services[] = {"service", "event_service", "Events", NULL};
        for (int i = 0; fallback_services[i] && !response; i++) {
            log_info("ONVIF: Trying fallback event endpoint: onvif/%s", fallback_services[i]);
            response = send_onvif_request(url, username, password, request_body,
                                          fallback_services[i], ONVIF_ACTION_CREATE_PULLPOINT);
        }
    }

    if (!response) {
        log_error("Failed to create subscription on any endpoint");
        pthread_mutex_unlock(&subscription_mutex);
        return NULL;
    }

    char *subscription_address = extract_subscription_address(response);
    free(response);

    if (!subscription_address) {
        log_error("Failed to extract subscription address");
        pthread_mutex_unlock(&subscription_mutex);
        return NULL;
    }

    // Find an empty slot or reuse an inactive one
    int slot = -1;
    for (int i = 0; i < subscription_count; i++) {
        if (!subscriptions[i].active) {
            slot = i;
            break;
        }
    }

    // If no empty slot found, add to the end if there's space
    if (slot == -1 && subscription_count < MAX_SUBSCRIPTIONS) {
        slot = subscription_count++;
    }

    // If we found a slot, use it
    if (slot >= 0) {
        // Store camera URL, username, and password
        safe_strcpy(subscriptions[slot].camera_url, url, sizeof(subscriptions[slot].camera_url), 0);
        
        safe_strcpy(subscriptions[slot].username, username, sizeof(subscriptions[slot].username), 0);
        
        safe_strcpy(subscriptions[slot].password, password, sizeof(subscriptions[slot].password), 0);
        
        // Store subscription address
        safe_strcpy(subscriptions[slot].subscription_address, subscription_address, 
                sizeof(subscriptions[slot].subscription_address), 0);
        
        // Set timestamps
        time(&subscriptions[slot].creation_time);
        subscriptions[slot].expiration_time = subscriptions[slot].creation_time + 3600; // 1 hour
        subscriptions[slot].active = true;
        
        log_info("Successfully created ONVIF subscription for %s", url);
        free(subscription_address);
        pthread_mutex_unlock(&subscription_mutex);
        return &subscriptions[slot];
    }

    log_error("No space for new ONVIF subscription");
    free(subscription_address);
    pthread_mutex_unlock(&subscription_mutex);
    return NULL;
}

// Extract service name from subscription address
static char *extract_service_name(const char *subscription_address) {
    if (!subscription_address) return NULL;

    // Find the last slash
    const char *last_slash = strrchr(subscription_address, '/');
    if (!last_slash) return NULL;

    // Extract the service name
    char *service = strdup(last_slash + 1);
    return service;
}

/* Topics that signal motion/person detection.  "MotionDetector" also covers
 * "CellMotionDetector" (substring); "MotionAlarm" covers VideoSource/MotionAlarm. */
static bool topic_is_motion(const char *topic_text) {
    if (!topic_text) return false;
    return strstr(topic_text, "MotionDetector") != NULL ||
           strstr(topic_text, "VideoAnalytics/Motion") != NULL ||
           strstr(topic_text, "MotionAlarm") != NULL ||
           strstr(topic_text, "PeopleDetector") != NULL;
}

/* A property value counts as asserted only for boolean-true spellings. */
static bool value_is_true(const char *value) {
    if (!value) return false;
    return strcasecmp(value, "true") == 0 || strcmp(value, "1") == 0;
}

/*
 * Return true if any SimpleItem beneath `node` that sits inside a Data
 * element has a truthy Value.  `in_data` tracks whether an ancestor was a
 * Data element — Source/Key sections also carry SimpleItems, but their
 * Values are configuration tokens (often literally "1"), not event state.
 */
static bool data_has_asserted_item(ezxml_t node, bool in_data) {
    for (ezxml_t c = node ? node->child : NULL; c; c = c->ordered) {
        bool inside = in_data || strcmp(local_name(c), "Data") == 0;
        if (inside && strcmp(local_name(c), "SimpleItem") == 0 &&
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
 * state (Value="true"/"1").
 */
static bool xml_has_active_motion(ezxml_t node) {
    for (ezxml_t c = node ? node->child : NULL; c; c = c->ordered) {
        if (strcmp(local_name(c), "NotificationMessage") == 0) {
            ezxml_t topic = child_by_local_name(c, "Topic");
            if (topic_is_motion(topic ? ezxml_txt(topic) : NULL) &&
                data_has_asserted_item(c, false)) {
                return true;
            }
        } else if (xml_has_active_motion(c)) {
            return true;
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
static bool has_motion_event(const char *response) {
    if (!response) return false;

    /* Cheap pre-filter: skip XML parsing when the payload names no
     * motion-related topic at all (the common empty-response case). */
    if (!strstr(response, "Motion") && !strstr(response, "PeopleDetector") &&
        !strstr(response, "IsPeople")) {
        return false;
    }

    /* ezxml_parse_str modifies the buffer in place; work on a copy since
     * our parameter is const. */
    char *copy = strdup(response);
    if (!copy) return false;

    ezxml_t xml = ezxml_parse_str(copy, strlen(copy));
    if (!xml) {
        /* Malformed XML from buggy firmware: fall back to substring matching,
         * but at least require a truthy Value somewhere in the payload so a
         * plain "motion ended" message doesn't trigger. */
        log_debug("ONVIF Detection: failed to parse PullMessages response, using fallback heuristic");
        bool fallback =
            (strstr(response, "Value=\"true\"") || strstr(response, "Value='true'") ||
             strstr(response, "Value=\"1\"")    || strstr(response, "Value='1'"));
        free(copy);
        return fallback;
    }

    bool motion = xml_has_active_motion(xml);

    ezxml_free(xml);
    free(copy);
    return motion;
}

/**
 * Initialize the ONVIF detection system
 */
int init_onvif_detection_system(void) {
    if (initialized && curl_handle) {
        log_info("ONVIF detection system already initialized");
        return 0;  // Already initialized and curl handle is valid
    }

    pthread_mutex_lock(&curl_mutex);

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

    curl_handle = curl_easy_init();
    if (!curl_handle) {
        log_error("Failed to initialize curl handle");
        // Note: Don't call curl_global_cleanup() here - it's managed centrally
        pthread_mutex_unlock(&curl_mutex);
        return -1;
    }

    pthread_mutex_unlock(&curl_mutex);

    // Initialize subscriptions
    pthread_mutex_lock(&subscription_mutex);
    subscription_count = 0;
    memset(subscriptions, 0, sizeof(subscriptions));
    pthread_mutex_unlock(&subscription_mutex);

    initialized = true;
    log_info("ONVIF detection system initialized successfully");
    return 0;
}

/**
 * Shutdown the ONVIF detection system
 */
void shutdown_onvif_detection_system(void) {
    log_info("Shutting down ONVIF detection system (initialized: %s, curl_handle: %p)",
             initialized ? "yes" : "no", (void*)curl_handle);

    // Cleanup curl handle if it exists
    pthread_mutex_lock(&curl_mutex);
    if (curl_handle) {
        log_info("Cleaning up curl handle");
        curl_easy_cleanup(curl_handle);
        curl_handle = NULL;
    }
    pthread_mutex_unlock(&curl_mutex);

    // Note: Don't call curl_global_cleanup() here - it's managed centrally in curl_init.c
    // The global cleanup will happen at program shutdown

    initialized = false;
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
        log_debug("ONVIF Detection: Using camera with authentication (username: %s)", username);
    }

    log_debug("ONVIF Detection: Starting detection with URL: %s", onvif_url);
    log_debug("ONVIF Detection: Stream name: %s", stream_name);

    onvif_subscription_t *subscription = get_subscription(onvif_url, username, password);
    if (!subscription) {
        log_error("Failed to get subscription for %s", onvif_url);
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

    // Send PullMessages directly to the subscription address (the full URL returned by
    // CreatePullPointSubscription per ONVIF spec).  Fall back to the legacy path-extraction
    // approach only when the stored address is not an absolute HTTP URL.
    char *response = NULL;
    if (strncmp(subscription->subscription_address, "http://", 7) == 0 ||
        strncmp(subscription->subscription_address, "https://", 8) == 0) {
        log_debug("ONVIF Detection: Sending PullMessages to %s", subscription->subscription_address);
        response = send_onvif_request_to_url(subscription->subscription_address,
                                             subscription->username,
                                             subscription->password,
                                             request_body,
                                             ONVIF_ACTION_PULL_MESSAGES);
    } else {
        // Legacy fallback: extract last path component and re-append under /onvif/
        log_warn("ONVIF Detection: subscription_address is not a full URL ('%s'), using legacy path extraction",
                 subscription->subscription_address);
        char *service = extract_service_name(subscription->subscription_address);
        if (service) {
            response = send_onvif_request(subscription->camera_url,
                                         subscription->username,
                                         subscription->password,
                                         request_body,
                                         service,
                                         ONVIF_ACTION_PULL_MESSAGES);
            free(service);
        }
    }

    if (!response) {
        log_error("Failed to pull messages from subscription");
        
        // If pulling messages fails, the subscription might be invalid
        // Mark it as inactive so we'll create a new one next time
        pthread_mutex_lock(&subscription_mutex);
        subscription->active = false;
        pthread_mutex_unlock(&subscription_mutex);
        
        return -1;
    }

    // Check for motion events
    bool motion_detected = has_motion_event(response);
    free(response);

    if (motion_detected) {
        log_info("ONVIF Detection: Motion detected for %s", stream_name);

        // Create a single detection that covers the whole frame
        result->count = 1;
        safe_strcpy(result->detections[0].label, "motion", MAX_LABEL_LENGTH, 0);
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

            // Store the detection in the database (no recording_id linkage for ONVIF)
            store_detections_in_db(stream_name, result, event_timestamp, 0);

            // Publish to MQTT and trigger motion recording if detections remain after filtering
            if (result->count > 0) {
                mqtt_publish_detection(stream_name, result, event_timestamp);
                process_motion_event(stream_name, true, event_timestamp, false);
            }
        } else {
            log_warn("No stream name provided, skipping database storage");
        }
    } else {
        log_debug("ONVIF Detection: No motion detected for %s", stream_name);
        result->count = 0;

        // Notify motion recording that motion has ended
        if (stream_name && stream_name[0] != '\0') {
            process_motion_event(stream_name, false, event_timestamp, false);
        }
    }

    return 0;
}
