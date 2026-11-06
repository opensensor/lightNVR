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
static int fallback_requests;

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
    log_capture[0] = '\0';
}
void tearDown(void) {
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
    char body[8192] = {0}; size_t used = 0;
    while (used < sizeof(body) - 1 && !strstr(body, "</s:Envelope>")) {
        ssize_t n = recv(client, body + used, sizeof(body) - 1 - used, 0);
        if (n <= 0) { server_failed = 3; break; }
        used += (size_t)n; body[used] = '\0';
    }
    if (!strstr(body, "fixture-user") || !strstr(body, "</s:Envelope>")) server_failed = 4;
    char response[256];
    int length = snprintf(response, sizeof(response),
        "HTTP/1.1 307 Temporary Redirect\r\nLocation: http://127.0.0.1:%u/stolen\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        destination_port);
    if (send(client, response, (size_t)length, MSG_NOSIGNAL) != length) server_failed = 5;
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
        "<GetServices/>", ONVIF_ACTION_GET_SERVICES, NULL);
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
    const char *urls[] = {"file:///not-a-network-endpoint", "ftp://127.0.0.1/test",
                         "http://fixture-user:fixture-secret@127.0.0.1/test"};
    for (size_t i = 0; i < sizeof(urls)/sizeof(urls[0]); ++i) {
        TEST_ASSERT_NULL(perform_onvif_request_locked(handle, urls[i], "user", "password",
            "<GetServices/>", ONVIF_ACTION_GET_SERVICES, NULL));
    }
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
        char request[8192] = {0}; size_t used = 0;
        while (used < sizeof(request) - 1 && !strstr(request, "</s:Envelope>")) {
            ssize_t n = recv(client, request + used, sizeof(request) - 1 - used, 0);
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
        size_t sent = 0;
        while (sent < (size_t)length) {
            ssize_t n = send(client, response + sent, (size_t)length - sent, MSG_NOSIGNAL);
            if (n <= 0) { server_failed = 5; break; }
            sent += (size_t)n;
        }
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

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_soap_authentication_body_is_not_forwarded_to_redirect_target);
    RUN_TEST(test_invalid_endpoint_is_rejected_before_transfer);
    RUN_TEST(test_trace_never_logs_headers_or_curl_notes);
    RUN_TEST(test_duplicate_fallback_logs_only_fixed_service_name);
    return UNITY_END();
}
