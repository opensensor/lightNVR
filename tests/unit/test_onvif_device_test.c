/* ONVIF "Test connection": failures say what failed (credentials, reachability,
 * or the stream), and a refused rtsps:// stream URI is retried as plain RTSP.
 * A loopback fake ONVIF device answers the SOAP calls. */
#define _GNU_SOURCE
#include <arpa/inet.h>
#include <netinet/in.h>
#include <pthread.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <unistd.h>

#include "unity.h"
#include "video/onvif_device_management.h"

typedef enum { FAKE_OK, FAKE_PROFILES_NOT_AUTHORIZED } fake_mode_t;

static int listen_fd = -1;
static unsigned short device_port;
static unsigned short closed_port;
static volatile fake_mode_t mode;
static pthread_t server_thread;
static volatile bool server_stop;

static const char *ENVELOPE_START =
    "<?xml version=\"1.0\" encoding=\"UTF-8\"?>"
    "<env:Envelope xmlns:env=\"http://www.w3.org/2003/05/soap-envelope\" "
    "xmlns:tds=\"http://www.onvif.org/ver10/device/wsdl\" "
    "xmlns:trt=\"http://www.onvif.org/ver10/media/wsdl\" "
    "xmlns:tt=\"http://www.onvif.org/ver10/schema\" "
    "xmlns:ter=\"http://www.onvif.org/ver10/error\"><env:Body>";
static const char *ENVELOPE_END = "</env:Body></env:Envelope>";

static unsigned short unused_port(void) {
    int fd = socket(AF_INET, SOCK_STREAM, 0);
    struct sockaddr_in addr = {.sin_family = AF_INET, .sin_addr.s_addr = htonl(INADDR_LOOPBACK)};
    TEST_ASSERT_EQUAL_INT(0, bind(fd, (struct sockaddr *)&addr, sizeof(addr)));
    socklen_t len = sizeof(addr);
    getsockname(fd, (struct sockaddr *)&addr, &len);
    close(fd);  /* nothing listens here afterwards: connections are refused */
    return ntohs(addr.sin_port);
}

static void respond(int fd, int status, const char *body) {
    char header[256];
    int n = snprintf(header, sizeof(header),
                     "HTTP/1.1 %d %s\r\nContent-Type: application/soap+xml\r\n"
                     "Content-Length: %zu\r\nConnection: close\r\n\r\n",
                     status, status == 200 ? "OK" : "Bad Request", strlen(body));
    (void)!write(fd, header, (size_t)n);
    (void)!write(fd, body, strlen(body));
}

static void handle(int fd) {
    char request[16384];
    size_t used = 0;
    for (;;) {
        ssize_t got = read(fd, request + used, sizeof(request) - 1 - used);
        if (got <= 0) break;
        used += (size_t)got;
        request[used] = '\0';
        char *end = strstr(request, "\r\n\r\n");
        if (!end) continue;
        const char *cl = strcasestr(request, "Content-Length:");
        size_t want = cl ? strtoul(cl + 15, NULL, 10) : 0;
        if (used - (size_t)(end + 4 - request) >= want) break;
    }

    char body[4096];
    if (strstr(request, "GetServices")) {
        snprintf(body, sizeof(body),
                 "%s<tds:GetServicesResponse><tds:Service>"
                 "<tds:Namespace>http://www.onvif.org/ver10/media/wsdl</tds:Namespace>"
                 "<tds:XAddr>http://127.0.0.1:%u/onvif/media_service</tds:XAddr>"
                 "</tds:Service></tds:GetServicesResponse>%s",
                 ENVELOPE_START, device_port, ENVELOPE_END);
        respond(fd, 200, body);
    } else if (strstr(request, "GetProfiles") && mode == FAKE_PROFILES_NOT_AUTHORIZED) {
        snprintf(body, sizeof(body),
                 "%s<env:Fault><env:Code><env:Value>env:Sender</env:Value>"
                 "<env:Subcode><env:Value>ter:NotAuthorized</env:Value></env:Subcode></env:Code>"
                 "<env:Reason><env:Text xml:lang=\"en\">Sender not Authorized</env:Text></env:Reason>"
                 "</env:Fault>%s",
                 ENVELOPE_START, ENVELOPE_END);
        respond(fd, 400, body);
    } else if (strstr(request, "GetProfiles")) {
        snprintf(body, sizeof(body),
                 "%s<trt:GetProfilesResponse><trt:Profiles token=\"Profile_0\">"
                 "<tt:Name>main</tt:Name></trt:Profiles></trt:GetProfilesResponse>%s",
                 ENVELOPE_START, ENVELOPE_END);
        respond(fd, 200, body);
    } else if (strstr(request, "GetStreamUri")) {
        snprintf(body, sizeof(body),
                 "%s<trt:GetStreamUriResponse><trt:MediaUri>"
                 "<tt:Uri>rtsps://127.0.0.1:%u/ch0</tt:Uri>"
                 "</trt:MediaUri></trt:GetStreamUriResponse>%s",
                 ENVELOPE_START, closed_port, ENVELOPE_END);
        respond(fd, 200, body);
    } else {
        respond(fd, 400, "");
    }
}

static void *serve(void *arg) {
    (void)arg;
    while (!server_stop) {
        int fd = accept(listen_fd, NULL, NULL);
        if (fd < 0) continue;
        handle(fd);
        close(fd);
    }
    return NULL;
}

void setUp(void) { mode = FAKE_OK; }
void tearDown(void) {}

static void device_url(char *out, size_t size) {
    snprintf(out, size, "http://127.0.0.1:%u/onvif/device_service", device_port);
}

void test_plain_rtsp_fallback_url(void) {
    char out[512];
    TEST_ASSERT_EQUAL_INT(0, onvif_plain_rtsp_fallback_url("rtsps://192.168.52.4:322/ch0", out, sizeof(out)));
    TEST_ASSERT_EQUAL_STRING("rtsp://192.168.52.4:554/ch0", out);
    TEST_ASSERT_EQUAL_INT(0, onvif_plain_rtsp_fallback_url("RTSPS://user:p%40ss@cam.local/stream?x=1", out, sizeof(out)));
    TEST_ASSERT_EQUAL_STRING("rtsp://user:p%40ss@cam.local:554/stream?x=1", out);
    TEST_ASSERT_EQUAL_INT(0, onvif_plain_rtsp_fallback_url("rtsps://[fd00::4]:322/ch0", out, sizeof(out)));
    TEST_ASSERT_EQUAL_STRING("rtsp://[fd00::4]:554/ch0", out);
    TEST_ASSERT_EQUAL_INT(0, onvif_plain_rtsp_fallback_url("rtsps://cam:322", out, sizeof(out)));
    TEST_ASSERT_EQUAL_STRING("rtsp://cam:554", out);
    /* Only rtsps:// has a plain fallback. */
    TEST_ASSERT_EQUAL_INT(-1, onvif_plain_rtsp_fallback_url("rtsp://cam:554/ch0", out, sizeof(out)));
    TEST_ASSERT_EQUAL_INT(-1, onvif_plain_rtsp_fallback_url("http://cam/ch0", out, sizeof(out)));
    TEST_ASSERT_EQUAL_INT(-1, onvif_plain_rtsp_fallback_url("rtsps://[fd00::4/ch0", out, sizeof(out)));
    TEST_ASSERT_EQUAL_INT(-1, onvif_plain_rtsp_fallback_url("rtsps://cam/ch0", out, 8));
}

void test_rejected_credentials_are_reported_as_auth_failure(void) {
    mode = FAKE_PROFILES_NOT_AUTHORIZED;
    char url[256], detail[512];
    device_url(url, sizeof(url));
    int result = test_onvif_connection_detail(url, "admin", "wrong", detail, sizeof(detail));
    TEST_ASSERT_EQUAL_INT(ONVIF_TEST_AUTH_FAILED, result);
    TEST_ASSERT_NOT_NULL_MESSAGE(strstr(detail, "credentials"), detail);
    TEST_ASSERT_NULL_MESSAGE(strstr(detail, "wrong"), detail);  /* never echo the password */
}

void test_unreachable_device_is_reported_as_unreachable(void) {
    char url[256], detail[512];
    snprintf(url, sizeof(url), "http://127.0.0.1:%u/onvif/device_service", closed_port);
    int result = test_onvif_connection_detail(url, "admin", "secret", detail, sizeof(detail));
    TEST_ASSERT_EQUAL_INT(ONVIF_TEST_UNREACHABLE, result);
    TEST_ASSERT_NOT_NULL_MESSAGE(strstr(detail, "reach"), detail);
}

void test_refused_rtsps_stream_retries_plain_rtsp_and_reports_stream_failure(void) {
    char url[256], detail[512], expected[128];
    device_url(url, sizeof(url));
    int result = test_onvif_connection_detail(url, "admin", "secret", detail, sizeof(detail));
    TEST_ASSERT_EQUAL_INT(ONVIF_TEST_STREAM_FAILED, result);
    snprintf(expected, sizeof(expected), "rtsps://127.0.0.1:%u/ch0", closed_port);
    TEST_ASSERT_NOT_NULL_MESSAGE(strstr(detail, expected), detail);
    TEST_ASSERT_NOT_NULL_MESSAGE(strstr(detail, "rtsp://127.0.0.1:554/ch0"), detail);
    TEST_ASSERT_NULL_MESSAGE(strstr(detail, "secret"), detail);
    /* The legacy entry point keeps its 0 / non-zero contract. */
    TEST_ASSERT_NOT_EQUAL(0, test_onvif_connection(url, "admin", "secret"));
}

int main(void) {
    listen_fd = socket(AF_INET, SOCK_STREAM, 0);
    int one = 1;
    setsockopt(listen_fd, SOL_SOCKET, SO_REUSEADDR, &one, sizeof(one));
    struct sockaddr_in addr = {.sin_family = AF_INET, .sin_addr.s_addr = htonl(INADDR_LOOPBACK)};
    if (bind(listen_fd, (struct sockaddr *)&addr, sizeof(addr)) != 0 || listen(listen_fd, 8) != 0) {
        perror("fake ONVIF server");
        return 1;
    }
    socklen_t len = sizeof(addr);
    getsockname(listen_fd, (struct sockaddr *)&addr, &len);
    device_port = ntohs(addr.sin_port);
    closed_port = unused_port();
    pthread_create(&server_thread, NULL, serve, NULL);

    UNITY_BEGIN();
    RUN_TEST(test_plain_rtsp_fallback_url);
    RUN_TEST(test_rejected_credentials_are_reported_as_auth_failure);
    RUN_TEST(test_unreachable_device_is_reported_as_unreachable);
    RUN_TEST(test_refused_rtsps_stream_retries_plain_rtsp_and_reports_stream_failure);
    int failures = UNITY_END();

    server_stop = true;
    shutdown(listen_fd, SHUT_RDWR);
    close(listen_fd);
    pthread_join(server_thread, NULL);
    return failures;
}
