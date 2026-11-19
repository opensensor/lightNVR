/* Exercise the real request factory and exchange without camera hardware. */
#include "../../src/video/onvif_detection.c"
#include "unity.h"
#include <arpa/inet.h>
#include <poll.h>
#include <sys/socket.h>
#include <sys/time.h>

static char log_capture[8192];
static int origin, destination, server_failed;
static unsigned short destination_port;
static char advertised_endpoint[256];
static const char *operation_path;
static const char *operation_soap_path;
static int fallback_requests;

/* Force the fixture through short-write and EINTR paths during real HTTP
 * exchanges. Shared libcurl's own socket I/O is not linker-wrapped. */
static bool interrupt_send, zero_send;
static unsigned send_calls;
ssize_t __real_send(int fd, const void *buffer, size_t length, int flags);
ssize_t __wrap_send(int fd, const void *buffer, size_t length, int flags) {
    send_calls++;
    if (interrupt_send) { interrupt_send = false; errno = EINTR; return -1; }
    if (zero_send) return 0;
    if (length > 7) length = 7;
    return __real_send(fd, buffer, length, flags);
}
static bool fixture_send_all(int fd, const char *response, size_t length) {
    size_t sent = 0;
    while (sent < length) {
        ssize_t n = send(fd, response + sent, length - sent, MSG_NOSIGNAL);
        if (n < 0 && errno == EINTR) continue;
        if (n <= 0) return false;
        sent += (size_t)n;
    }
    return true;
}

void __wrap__log_message_ctx(log_level_t level, const char *component, const char *stream,
                            const char *format, ...) {
    (void)level; (void)component; (void)stream;
    size_t used = strlen(log_capture);
    va_list args; va_start(args, format);
    vsnprintf(log_capture + used, sizeof(log_capture) - used, format, args);
    va_end(args);
}

void setUp(void) {
    TEST_ASSERT_EQUAL_INT(CURLE_OK, curl_global_init(CURL_GLOBAL_DEFAULT));
    initialized = true;
    origin = destination = -1;
    server_failed = 0;
    operation_path = "/onvif/fixture?x=1&y=2";
    operation_soap_path = "/onvif/fixture?x=1&amp;y=2";
    interrupt_send = true;
    zero_send = false;
    send_calls = 0;
    log_capture[0] = '\0';
}
void tearDown(void) {
    if (subscriptions[0].curl) curl_easy_cleanup(subscriptions[0].curl);
    memset(&subscriptions[0], 0, sizeof(subscriptions[0]));
    initialized = false;
    if (curl_handle) { curl_easy_cleanup(curl_handle); curl_handle = NULL; }
    if (origin >= 0) close(origin);
    if (destination >= 0) close(destination);
    curl_global_cleanup();
}

static int listen_local(unsigned short *port) {
    int fd = socket(AF_INET, SOCK_STREAM, 0);
    if (fd < 0) return -1;
    struct sockaddr_in address = {.sin_family = AF_INET, .sin_addr.s_addr = htonl(INADDR_LOOPBACK)};
    socklen_t size = sizeof(address);
    if (bind(fd, (struct sockaddr *)&address, size) || listen(fd, 1) ||
        getsockname(fd, (struct sockaddr *)&address, &size)) { close(fd); return -1; }
    *port = ntohs(address.sin_port);
    return fd;
}

static void *redirect_server(void *unused) {
    (void)unused;
    struct pollfd wait = {.fd = origin, .events = POLLIN};
    if (poll(&wait, 1, 3000) != 1) { server_failed = 1; return NULL; }
    int client = accept(origin, NULL, NULL);
    if (client < 0) { server_failed = 2; return NULL; }
    struct timeval timeout = {.tv_sec = 3};
    setsockopt(client, SOL_SOCKET, SO_RCVTIMEO, &timeout, sizeof(timeout));
    setsockopt(client, SOL_SOCKET, SO_SNDTIMEO, &timeout, sizeof(timeout));
    char body[8192] = {0}; size_t used = 0;
    while (used < sizeof(body) - 1 && !strstr(body, "</s:Envelope>")) {
        ssize_t n = recv(client, body + used, sizeof(body) - 1 - used, 0);
        if (n < 0 && errno == EINTR) continue;
        if (n <= 0) { server_failed = 3; break; }
        used += (size_t)n; body[used] = '\0';
    }
    if (!strstr(body, "fixture-user") || !strstr(body, "</s:Envelope>")) server_failed = 4;
    char response[256];
    int length = snprintf(response, sizeof(response),
        "HTTP/1.1 307 Temporary Redirect\r\nLocation: http://127.0.0.1:%u/stolen\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        destination_port);
    if (!fixture_send_all(client, response, (size_t)length)) server_failed = 5;
    close(client);
    return NULL;
}

static void test_soap_authentication_body_is_not_forwarded_to_redirect_target(void) {
    unsigned short port;
    origin = listen_local(&port);
    destination = listen_local(&destination_port);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, origin);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, destination);
    pthread_t worker;
    TEST_ASSERT_EQUAL_INT(0, pthread_create(&worker, NULL, redirect_server, NULL));
    CURL *handle = create_curl_handle(); TEST_ASSERT_NOT_NULL(handle);
    curl_easy_setopt(handle, CURLOPT_PROXY, "");
    curl_easy_setopt(handle, CURLOPT_CONNECTTIMEOUT_MS, 500L);
    char url[128]; snprintf(url, sizeof(url), "http://127.0.0.1:%u/onvif/events?token=fixture-token", port);
    char *response = perform_onvif_request_locked(handle, url, "fixture-user", "fixture-password",
        "<GetServices/>", ONVIF_OPERATION_GET_SERVICES, NULL);
    curl_easy_cleanup(handle);
    TEST_ASSERT_EQUAL_INT(0, pthread_join(worker, NULL));
    TEST_ASSERT_EQUAL_INT(0, server_failed);
    TEST_ASSERT_NULL(response);
    free(response);
    struct pollfd pending = {.fd = destination, .events = POLLIN};
    TEST_ASSERT_EQUAL_INT(0, poll(&pending, 1, 0));
    TEST_ASSERT_NULL(strstr(log_capture, "fixture-token"));
    TEST_ASSERT_NULL(strstr(log_capture, "fixture-password"));
}

static void test_invalid_endpoint_is_rejected_before_transfer(void) {
    CURL *handle = create_curl_handle(); TEST_ASSERT_NOT_NULL(handle);
    curl_easy_setopt(handle, CURLOPT_PROXY, "");
    const char *urls[] = {"file:///not-a-network-endpoint", "ftp://127.0.0.1/test"};
    for (size_t i = 0; i < sizeof(urls)/sizeof(urls[0]); ++i) {
        TEST_ASSERT_NULL(perform_onvif_request_locked(handle, urls[i], "user", "password",
            "<GetServices/>", ONVIF_OPERATION_GET_SERVICES, NULL));
    }

    unsigned short port;
    int listener = listen_local(&port);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, listener);
    char userinfo_url[160];
    snprintf(userinfo_url, sizeof(userinfo_url),
        "http://fixture-user:fixture-secret@127.0.0.1:%u/test", port);
    TEST_ASSERT_NULL(perform_onvif_request_locked(handle, userinfo_url, "user", "password",
        "<GetServices/>", ONVIF_OPERATION_GET_SERVICES, NULL));
    struct pollfd pending = {.fd = listener, .events = POLLIN};
    TEST_ASSERT_EQUAL_INT(0, poll(&pending, 1, 0));
    close(listener);

    curl_easy_cleanup(handle);
    TEST_ASSERT_NULL(strstr(log_capture, "fixture-secret"));
}

static void test_trace_never_logs_headers_or_curl_notes(void) {
    char data[] = "Authorization: Basic secret\r\nSet-Cookie: token=secret\r\nGET /?token=secret HTTP/1.1";
    const curl_infotype types[] = {CURLINFO_TEXT, CURLINFO_HEADER_IN, CURLINFO_HEADER_OUT,
                                  CURLINFO_DATA_IN, CURLINFO_DATA_OUT};
    for (size_t i = 0; i < sizeof(types)/sizeof(types[0]); ++i)
        curl_trace_callback(NULL, types[i], data, strlen(data), NULL);
    TEST_ASSERT_NOT_NULL(strstr(log_capture, "headers:"));
    TEST_ASSERT_NULL(strstr(log_capture, "secret"));
    TEST_ASSERT_NULL(strstr(log_capture, "Authorization"));
}

static void *fallback_server(void *unused) {
    (void)unused;
    const char *paths[] = {"/onvif/device_service", "/onvif/service",
                           "/onvif/event_service", "/onvif/Events"};
    for (size_t i = 0; i < sizeof(paths) / sizeof(paths[0]); ++i) {
        struct pollfd wait = {.fd = origin, .events = POLLIN};
        if (poll(&wait, 1, 3000) != 1) { server_failed = 1; return NULL; }
        int client = accept(origin, NULL, NULL);
        if (client < 0) { server_failed = 2; return NULL; }
        struct timeval timeout = {.tv_sec = 3};
        setsockopt(client, SOL_SOCKET, SO_RCVTIMEO, &timeout, sizeof(timeout));
        setsockopt(client, SOL_SOCKET, SO_SNDTIMEO, &timeout, sizeof(timeout));
        char request[8192] = {0}; size_t used = 0;
        while (used < sizeof(request) - 1 && !strstr(request, "</s:Envelope>")) {
            ssize_t n = recv(client, request + used, sizeof(request) - 1 - used, 0);
            if (n < 0 && errno == EINTR) continue;
            if (n <= 0) { server_failed = 3; break; }
            used += (size_t)n; request[used] = '\0';
        }
        char target[128]; snprintf(target, sizeof(target), "POST %s HTTP/", paths[i]);
        if (strncmp(request, target, strlen(target)) != 0) server_failed = 4;
        ++fallback_requests;
        char body[1024] = {0};
        if (i == 0) snprintf(body, sizeof(body),
            "<Envelope><Body><GetServicesResponse><Service>"
            "<Namespace>http://www.onvif.org/ver10/events/wsdl</Namespace>"
            "<XAddr>%s</XAddr></Service></GetServicesResponse></Body></Envelope>",
            advertised_endpoint);
        char response[1400];
        int length = snprintf(response, sizeof(response),
            "HTTP/1.1 %s\r\nContent-Length: %zu\r\nConnection: close\r\n\r\n%s",
            i == 0 ? "200 OK" : "500 Failed", strlen(body), body);
        if (!fixture_send_all(client, response, (size_t)length)) server_failed = 5;
        close(client);
    }
    return NULL;
}

static void test_duplicate_fallback_logs_only_fixed_service_name(void) {
    unsigned short port;
    origin = listen_local(&port);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, origin);
    char base[128]; snprintf(base, sizeof(base), "http://127.0.0.1:%u", port);
    snprintf(advertised_endpoint, sizeof(advertised_endpoint), "%s/onvif/service", base);
    fallback_requests = 0;
    curl_handle = create_curl_handle(); TEST_ASSERT_NOT_NULL(curl_handle);
    curl_easy_setopt(curl_handle, CURLOPT_PROXY, "");
    pthread_t worker;
    TEST_ASSERT_EQUAL_INT(0, pthread_create(&worker, NULL, fallback_server, NULL));
    onvif_subscription_t *subscription = get_subscription(base, "fixture-user", "fixture-password");
    TEST_ASSERT_EQUAL_INT(0, pthread_join(worker, NULL));
    TEST_ASSERT_EQUAL_INT(0, server_failed);
    TEST_ASSERT_EQUAL_INT(4, fallback_requests);
    TEST_ASSERT_NULL(subscription);
    TEST_ASSERT_NOT_NULL(strstr(log_capture, "Skipping already-tried event endpoint: onvif/service"));
    TEST_ASSERT_NULL(strstr(log_capture, "Skipping already-tried event endpoint: http"));
    TEST_ASSERT_NULL(strstr(log_capture, "fixture-password"));
}

static void test_discovered_endpoint_on_other_host_is_not_contacted(void) {
    unsigned short port;
    origin = listen_local(&port);
    destination = listen_local(&destination_port);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, origin);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, destination);
    char base[128];
    snprintf(base, sizeof(base), "http://127.0.0.1:%u", port);
    snprintf(advertised_endpoint, sizeof(advertised_endpoint),
             "http://localhost:%u/steal", destination_port);
    fallback_requests = 0;
    curl_handle = create_curl_handle();
    TEST_ASSERT_NOT_NULL(curl_handle);
    curl_easy_setopt(curl_handle, CURLOPT_PROXY, "");
    pthread_t worker;
    TEST_ASSERT_EQUAL_INT(0, pthread_create(&worker, NULL, fallback_server, NULL));
    onvif_subscription_t *subscription = get_subscription(base, "fixture-user", "fixture-password");
    TEST_ASSERT_EQUAL_INT(0, pthread_join(worker, NULL));
    TEST_ASSERT_NULL(subscription);
    TEST_ASSERT_EQUAL_INT(0, server_failed);
    TEST_ASSERT_EQUAL_INT(4, fallback_requests);
    struct pollfd pending = {.fd = destination, .events = POLLIN};
    TEST_ASSERT_EQUAL_INT(0, poll(&pending, 1, 0));
}

/* Independent wire expectations: changing a SOAP identifier to HTTPS breaks
 * the protocol even when the transport endpoint itself uses HTTPS. */
static const char *const expected_actions[] = {
    "http://www.onvif.org/ver10/device/wsdl/GetServices",
    "http://www.onvif.org/ver10/events/wsdl/EventPortType/CreatePullPointSubscriptionRequest",
    "http://www.onvif.org/ver10/events/wsdl/PullPointSubscription/PullMessagesRequest",
    "http://docs.oasis-open.org/wsn/bw-2/SubscriptionManager/RenewRequest",
    "http://docs.oasis-open.org/wsn/bw-2/SubscriptionManager/UnsubscribeRequest"
};
static const onvif_operation_t fixture_operations[] = {
    ONVIF_OPERATION_GET_SERVICES, ONVIF_OPERATION_CREATE_PULLPOINT,
    ONVIF_OPERATION_PULL_MESSAGES, ONVIF_OPERATION_RENEW, ONVIF_OPERATION_UNSUBSCRIBE
};

static void *operation_server(void *unused) {
    (void)unused;
    for (size_t i = 0; i < sizeof(expected_actions) / sizeof(expected_actions[0]); ++i) {
        struct pollfd wait = {.fd = origin, .events = POLLIN};
        if (poll(&wait, 1, 3000) != 1) { server_failed = 1; return NULL; }
        int client = accept(origin, NULL, NULL);
        if (client < 0) { server_failed = 2; return NULL; }
        struct timeval timeout = {.tv_sec = 3};
        setsockopt(client, SOL_SOCKET, SO_RCVTIMEO, &timeout, sizeof(timeout));
        setsockopt(client, SOL_SOCKET, SO_SNDTIMEO, &timeout, sizeof(timeout));
        char request[8192] = {0}; size_t used = 0;
        while (used < sizeof(request) - 1 && !strstr(request, "</s:Envelope>")) {
            ssize_t n = recv(client, request + used, sizeof(request) - 1 - used, 0);
            if (n < 0 && errno == EINTR) continue;
            if (n <= 0) { server_failed = 3; break; }
            used += (size_t)n; request[used] = '\0';
        }
        char header[512], element[512], destination_element[512];
        snprintf(header, sizeof(header), "Content-Type: application/soap+xml; charset=utf-8; action=\"%s\"\r\n", expected_actions[i]);
        snprintf(element, sizeof(element), "<wsa:Action>%s</wsa:Action>", expected_actions[i]);
        snprintf(destination_element, sizeof(destination_element), "<wsa:To>%s%s</wsa:To>", advertised_endpoint, operation_soap_path);
        char target[256];
        snprintf(target, sizeof(target), "POST %s HTTP/", operation_path);
        if (strncmp(request, target, strlen(target)) != 0 ||
            !strstr(request, header) || !strstr(request, element) ||
            !strstr(request, destination_element) || !strstr(request, "<Fixture/>")) server_failed = 4;
        const char response[] = "HTTP/1.1 200 OK\r\nContent-Length: 11\r\nConnection: close\r\n\r\n<Envelope/>";
        if (!fixture_send_all(client, response, sizeof(response) - 1)) server_failed = 5;
        close(client);
    }
    return NULL;
}

static void test_every_operation_preserves_action_headers_and_transport_destination(void) {
    unsigned short port;
    origin = listen_local(&port);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, origin);
    snprintf(advertised_endpoint, sizeof(advertised_endpoint), "http://127.0.0.1:%u", port);
    char url[512]; snprintf(url, sizeof(url), "%s/onvif/fixture?x=1&y=2", advertised_endpoint);
    CURL *handle = create_curl_handle(); TEST_ASSERT_NOT_NULL(handle);
    curl_easy_setopt(handle, CURLOPT_PROXY, "");
    pthread_t worker;
    TEST_ASSERT_EQUAL_INT(0, pthread_create(&worker, NULL, operation_server, NULL));
    bool responses_ok = true;
    for (size_t i = 0; i < sizeof(fixture_operations) / sizeof(fixture_operations[0]); ++i) {
        char *response = perform_onvif_request_locked(handle, url, NULL, NULL,
            "<Fixture/>", fixture_operations[i], NULL);
        if (!response || strcmp(response, "<Envelope/>") != 0) responses_ok = false;
        free(response);
    }
    curl_easy_cleanup(handle);
    TEST_ASSERT_EQUAL_INT(0, pthread_join(worker, NULL));
    TEST_ASSERT_EQUAL_INT(0, server_failed);
    TEST_ASSERT_TRUE(responses_ok);
}

static void test_https_destination_preserves_standard_soap_action_identifiers(void) {
    for (size_t i = 0; i < sizeof(fixture_operations) / sizeof(fixture_operations[0]); ++i) {
        char *soap = create_onvif_request(NULL, NULL, "<Fixture/>", fixture_operations[i],
                                         "https://127.0.0.1/onvif/fixture");
        TEST_ASSERT_NOT_NULL(soap);
        char element[512];
        snprintf(element, sizeof(element), "<wsa:Action>%s</wsa:Action>", expected_actions[i]);
        bool correct_action = strstr(soap, element) != NULL;
        bool correct_destination = strstr(soap, "<wsa:To>https://127.0.0.1/onvif/fixture</wsa:To>") != NULL;
        free(soap);
        TEST_ASSERT_TRUE(correct_action);
        TEST_ASSERT_TRUE(correct_destination);
    }
}

static void test_invalid_operation_is_rejected_before_transfer(void) {
    unsigned short port;
    origin = listen_local(&port);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, origin);
    char url[128]; snprintf(url, sizeof(url), "http://127.0.0.1:%u/unused", port);
    CURL *handle = create_curl_handle(); TEST_ASSERT_NOT_NULL(handle);
    const onvif_operation_t invalid[] = {(onvif_operation_t)-1, ONVIF_OPERATION_COUNT, (onvif_operation_t)(ONVIF_OPERATION_COUNT + 1)};
    for (size_t i = 0; i < sizeof(invalid) / sizeof(invalid[0]); ++i) {
        TEST_ASSERT_NULL(create_onvif_request(NULL, NULL, "<Fixture/>", invalid[i], url));
        TEST_ASSERT_NULL(perform_onvif_request_locked(handle, url, NULL, NULL, "<Fixture/>", invalid[i], NULL));
    }
    curl_easy_cleanup(handle);
    struct pollfd pending = {.fd = origin, .events = POLLIN};
    TEST_ASSERT_EQUAL_INT(0, poll(&pending, 1, 0));
}

static void test_subscription_origin_allows_camera_local_port(void) {
    const struct { const char *camera; const char *endpoint; bool allowed; } cases[] = {
        {"http://camera", "http://CAMERA:80/pull", true},
        {"https://camera:443", "https://camera/pull", true},
        {"http://[::1]:8080", "http://[::1]:1026/pull", true},
        {"http://192.168.1.123:2020", "http://192.168.1.123:1026/pull", true},
        {"https://camera:443", "http://camera:443/pull", false},
        {"http://camera:80", "https://camera:80/pull", false},
        {"http://camera", "http://camera:8080/pull", true},
        {"http://camera", "http://camera:0/pull", false},
        {"http://camera", "http://camera:65536/pull", false},
        {"http://camera", "http://other/pull", false},
        {"http://camera", "http://user:secret@camera/pull", false},
        {"http://camera", "http://camera/pull\r\nInjected: x", false},
        {"http://camera", "ftp://camera:80/pull", false},
        {"http://camera", "//camera/pull", false},
    };
    for (size_t i = 0; i < sizeof(cases) / sizeof(cases[0]); i++)
        TEST_ASSERT_EQUAL_INT_MESSAGE(cases[i].allowed,
            subscription_address_matches_camera_host(cases[i].camera, cases[i].endpoint), cases[i].endpoint);
}

static void test_rejected_subscription_never_contacts_target_or_legacy_fallback(void) {
    unsigned short port;
    origin = listen_local(&port);
    destination = listen_local(&destination_port);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, origin);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, destination);
    onvif_subscription_t *sub = &subscriptions[0];
    memset(sub, 0, sizeof(*sub));
    snprintf(sub->camera_url, sizeof(sub->camera_url), "http://127.0.0.1:%u", port);
    strcpy(sub->username, "fixture-user");
    strcpy(sub->password, "fixture-password");
    const onvif_operation_t operations[] = {ONVIF_OPERATION_PULL_MESSAGES, ONVIF_OPERATION_RENEW, ONVIF_OPERATION_UNSUBSCRIBE};
    for (int variant = 0; variant < 4; variant++) {
        snprintf(sub->subscription_address, sizeof(sub->subscription_address),
                 variant == 0 ? "http://127.0.0.2:%u/stolen" :
                 variant == 1 ? "https://127.0.0.1:%u/stolen" :
                 variant == 2 ? "ftp://127.0.0.1:%u/stolen" : "//127.0.0.1:%u/stolen",
                 variant == 1 ? port : destination_port);
        for (size_t i = 0; i < sizeof(operations) / sizeof(operations[0]); i++) {
            bool dropped = true;
            TEST_ASSERT_NULL(send_subscription_request(sub, "<Fixture/>", operations[i], &dropped, NULL));
            TEST_ASSERT_FALSE(dropped);
        }
    }
    struct pollfd pending[] = {{.fd = origin, .events = POLLIN}, {.fd = destination, .events = POLLIN}};
    TEST_ASSERT_EQUAL_INT(0, poll(pending, 2, 0));
    TEST_ASSERT_NULL(sub->curl);
}

static void assert_subscription_request_target(int relative, bool different_port) {
    unsigned short port;
    origin = listen_local(&port);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, origin);
    snprintf(advertised_endpoint, sizeof(advertised_endpoint), "http://127.0.0.1:%u", port);
    onvif_subscription_t *sub = &subscriptions[0];
    memset(sub, 0, sizeof(*sub));
    sub->curl = create_curl_handle();
    TEST_ASSERT_NOT_NULL(sub->curl);
    curl_easy_setopt(sub->curl, CURLOPT_PROXY, "");
    if (different_port) {
        destination = listen_local(&destination_port);
        TEST_ASSERT_GREATER_OR_EQUAL_INT(0, destination);
        snprintf(sub->camera_url, sizeof(sub->camera_url),
                 "http://127.0.0.1:%u", destination_port);
    } else {
        strcpy(sub->camera_url, advertised_endpoint);
    }
    snprintf(sub->subscription_address, sizeof(sub->subscription_address), "%s/onvif/fixture?x=1&y=2", advertised_endpoint);
    if (relative == 1) {
        strcpy(sub->subscription_address, "fixture?x=1&y=2");
        snprintf(sub->service_url, sizeof(sub->service_url),
                 "%s/onvif/service", advertised_endpoint);
        curl_handle = create_curl_handle();
        TEST_ASSERT_NOT_NULL(curl_handle);
        curl_easy_setopt(curl_handle, CURLOPT_PROXY, "");
    } else if (relative == 2) {
        strcpy(sub->subscription_address, "/onvif/subscriptions/fixture?x=1&y=2");
        snprintf(sub->service_url, sizeof(sub->service_url),
                 "%s/onvif/service", advertised_endpoint);
        operation_path = "/onvif/subscriptions/fixture?x=1&y=2";
        operation_soap_path = "/onvif/subscriptions/fixture?x=1&amp;y=2";
    }
    pthread_t worker;
    TEST_ASSERT_EQUAL_INT(0, pthread_create(&worker, NULL, operation_server, NULL));
    bool ok = true;
    for (size_t i = 0; i < sizeof(fixture_operations) / sizeof(fixture_operations[0]); i++) {
        char *response = send_subscription_request(sub, "<Fixture/>", fixture_operations[i], NULL, NULL);
        if (!response || strcmp(response, "<Envelope/>") != 0) ok = false;
        free(response);
    }
    pthread_join(worker, NULL);
    TEST_ASSERT_EQUAL_INT(0, server_failed);
    TEST_ASSERT_TRUE(ok);
    if (different_port) {
        struct pollfd pending = {.fd = destination, .events = POLLIN};
        TEST_ASSERT_EQUAL_INT(0, poll(&pending, 1, 0));
    }
}

static void test_same_origin_subscription_keeps_full_request_target(void) {
    assert_subscription_request_target(false, false);
}

static void test_camera_local_ephemeral_port_receives_subscription_requests(void) {
    assert_subscription_request_target(false, true);
}

static void test_relative_subscription_preserves_camera_local_fallback(void) {
    assert_subscription_request_target(1, false);
}

static void test_relative_subscription_preserves_full_path(void) {
    assert_subscription_request_target(2, false);
}

static void test_full_registry_rejects_before_remote_create(void) {
    subscription_count = MAX_SUBSCRIPTIONS;
    for (int i = 0; i < MAX_SUBSCRIPTIONS; i++) {
        subscriptions[i].active = true;
        snprintf(subscriptions[i].camera_url, sizeof(subscriptions[i].camera_url),
                 "http://occupied-%d", i);
    }
    TEST_ASSERT_NULL(get_subscription("http://new-camera", "", ""));
    subscription_count = 0;
    memset(subscriptions, 0, sizeof(subscriptions));
}

static void test_pull_rejects_reassigned_slot(void) {
    onvif_subscription_t *sub = &subscriptions[0];
    sub->active = true;
    strcpy(sub->camera_url, "http://different-camera");
    strcpy(sub->subscription_address, "http://different-camera/onvif/pull");
    TEST_ASSERT_NULL(send_subscription_request(sub, "<Fixture/>",
                     ONVIF_OPERATION_PULL_MESSAGES, NULL, "http://original-camera"));
}

typedef struct {
    char url[128];
    onvif_subscription_t *subscription;
} subscription_lookup_fixture_t;

static void *lookup_subscription_worker(void *arg) {
    subscription_lookup_fixture_t *fixture = arg;
    fixture->subscription = get_subscription(fixture->url, "", "");
    return NULL;
}

static void test_discovery_does_not_hold_subscription_table_lock(void) {
    unsigned short port;
    origin = listen_local(&port);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, origin);
    curl_handle = create_curl_handle();
    TEST_ASSERT_NOT_NULL(curl_handle);
    curl_easy_setopt(curl_handle, CURLOPT_PROXY, "");

    subscription_lookup_fixture_t fixture = {0};
    snprintf(fixture.url, sizeof(fixture.url), "http://127.0.0.1:%u", port);
    pthread_t worker;
    TEST_ASSERT_EQUAL_INT(0, pthread_create(&worker, NULL, lookup_subscription_worker, &fixture));
    struct pollfd pending = {.fd = origin, .events = POLLIN};
    TEST_ASSERT_EQUAL_INT(1, poll(&pending, 1, 3000));
    int client = accept(origin, NULL, NULL);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, client);
    int lock_result = pthread_mutex_trylock(&subscription_mutex);
    if (lock_result == 0) pthread_mutex_unlock(&subscription_mutex);
    close(client);
    close(origin);
    origin = -1;
    pthread_join(worker, NULL);
    TEST_ASSERT_EQUAL_INT(0, lock_result);
    TEST_ASSERT_NULL(fixture.subscription);
    TEST_ASSERT_FALSE(subscriptions[0].reserved);
    subscription_count = 0;
    memset(subscriptions, 0, sizeof(subscriptions));
}

static void test_fixture_send_stops_on_zero_progress_or_closed_peer(void) {
    int pair[2];
    TEST_ASSERT_EQUAL_INT(0, socketpair(AF_UNIX, SOCK_STREAM, 0, pair));
    interrupt_send = false;
    zero_send = true;
    bool zero_result = fixture_send_all(pair[0], "response", 8);
    unsigned zero_calls = send_calls;
    zero_send = false;
    close(pair[1]);
    bool closed_result = fixture_send_all(pair[0], "response", 8);
    close(pair[0]);
    TEST_ASSERT_FALSE(zero_result);
    TEST_ASSERT_EQUAL_UINT(1, zero_calls);
    TEST_ASSERT_FALSE(closed_result);
    TEST_ASSERT_EQUAL_UINT(2, send_calls);
}

static void test_motorcycle_class_precedes_bicycle_substrings(void) {
    TEST_ASSERT_EQUAL_STRING("vehicle", normalize_object_class("Motorcycle"));
    TEST_ASSERT_EQUAL_STRING("vehicle", normalize_object_class("motorbike"));
    TEST_ASSERT_EQUAL_STRING("bicycle", normalize_object_class("bicycle"));
}

static void test_malformed_motion_message_uses_truthy_fallback(void) {
    char label[32];
    TEST_ASSERT_TRUE(has_motion_event("<Motion Value=\"true\"", label, sizeof(label)));
    TEST_ASSERT_EQUAL_STRING("motion", label);
    TEST_ASSERT_FALSE(has_motion_event("<Motion Value=\"false\"", label, sizeof(label)));
    TEST_ASSERT_FALSE(has_motion_event(
        "<Motion Value=\"false\"><SimpleItem Name=\"Gain\" Value=\"1\"",
        label, sizeof(label)));
}

static void test_motion_requires_asserted_state_not_truthy_metadata(void) {
    char label[32];
    const char *metadata_only =
        "<Envelope><NotificationMessage><Topic>RuleEngine/CellMotionDetector/Motion</Topic>"
        "<Message><Data><SimpleItem Name=\"Gain\" Value=\"1\"/>"
        "<SimpleItem Name=\"Camera\" Value=\"1\"/>"
        "<SimpleItem Name=\"IsMotion\" Value=\"false\"/>"
        "</Data></Message></NotificationMessage></Envelope>";
    TEST_ASSERT_FALSE(has_motion_event(metadata_only, label, sizeof(label)));

    const char *asserted =
        "<Envelope><NotificationMessage><Topic>RuleEngine/CellMotionDetector/Motion</Topic>"
        "<Message><Data><SimpleItem Name=\"Gain\" Value=\"1\"/>"
        "<SimpleItem Name=\"IsMotion\" Value=\"true\"/>"
        "</Data></Message></NotificationMessage></Envelope>";
    TEST_ASSERT_TRUE(has_motion_event(asserted, label, sizeof(label)));
    TEST_ASSERT_EQUAL_STRING("motion", label);
}

static void test_class_flags_ignore_camera_metadata(void) {
    char label[32];
    const char *metadata_only =
        "<Envelope><NotificationMessage><Topic>RuleEngine/CellMotionDetector/Motion</Topic>"
        "<Message><Data><SimpleItem Name=\"IsCamera\" Value=\"true\"/>"
        "<SimpleItem Name=\"IsMotion\" Value=\"false\"/>"
        "</Data></Message></NotificationMessage></Envelope>";
    TEST_ASSERT_FALSE(has_motion_event(metadata_only, label, sizeof(label)));

    const char *asserted =
        "<Envelope><NotificationMessage><Topic>RuleEngine/CellMotionDetector/Motion</Topic>"
        "<Message><Data><SimpleItem Name=\"IsCamera\" Value=\"true\"/>"
        "<SimpleItem Name=\"IsMotion\" Value=\"true\"/>"
        "</Data></Message></NotificationMessage></Envelope>";
    TEST_ASSERT_TRUE(has_motion_event(asserted, label, sizeof(label)));
    TEST_ASSERT_EQUAL_STRING("motion", label);

    const char *vehicle =
        "<Envelope><NotificationMessage><Topic>RuleEngine/CellMotionDetector/Motion</Topic>"
        "<Message><Data><SimpleItem Name=\"IsVehicle\" Value=\"true\"/>"
        "</Data></Message></NotificationMessage></Envelope>";
    TEST_ASSERT_TRUE(has_motion_event(vehicle, label, sizeof(label)));
    TEST_ASSERT_EQUAL_STRING("vehicle", label);
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_every_operation_preserves_action_headers_and_transport_destination);
    RUN_TEST(test_invalid_operation_is_rejected_before_transfer);
    RUN_TEST(test_https_destination_preserves_standard_soap_action_identifiers);
    RUN_TEST(test_soap_authentication_body_is_not_forwarded_to_redirect_target);
    RUN_TEST(test_invalid_endpoint_is_rejected_before_transfer);
    RUN_TEST(test_trace_never_logs_headers_or_curl_notes);
    RUN_TEST(test_duplicate_fallback_logs_only_fixed_service_name);
    RUN_TEST(test_discovered_endpoint_on_other_host_is_not_contacted);
    RUN_TEST(test_subscription_origin_allows_camera_local_port);
    RUN_TEST(test_rejected_subscription_never_contacts_target_or_legacy_fallback);
    RUN_TEST(test_same_origin_subscription_keeps_full_request_target);
    RUN_TEST(test_camera_local_ephemeral_port_receives_subscription_requests);
    RUN_TEST(test_relative_subscription_preserves_camera_local_fallback);
    RUN_TEST(test_relative_subscription_preserves_full_path);
    RUN_TEST(test_full_registry_rejects_before_remote_create);
    RUN_TEST(test_pull_rejects_reassigned_slot);
    RUN_TEST(test_discovery_does_not_hold_subscription_table_lock);
    RUN_TEST(test_fixture_send_stops_on_zero_progress_or_closed_peer);
    RUN_TEST(test_motorcycle_class_precedes_bicycle_substrings);
    RUN_TEST(test_malformed_motion_message_uses_truthy_fallback);
    RUN_TEST(test_motion_requires_asserted_state_not_truthy_metadata);
    RUN_TEST(test_class_flags_ignore_camera_metadata);
    return UNITY_END();
}
