#define _POSIX_C_SOURCE 200809L
#include "unity.h"
#include "video/rtsp_probe.h"

#include <arpa/inet.h>
#include <poll.h>
#include <pthread.h>
#include <stdio.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/time.h>
#include <unistd.h>

static int listener;
static char captured[2048];
static const char *reply;
static int server_error;

void setUp(void) { listener = -1; captured[0] = '\0'; server_error = 0; }
void tearDown(void) { if (listener >= 0) close(listener); }

static void *serve(void *unused) {
    (void)unused;
    struct pollfd wait = {.fd = listener, .events = POLLIN};
    if (poll(&wait, 1, 3000) != 1) { server_error = 1; return NULL; }
    int fd = accept(listener, NULL, NULL);
    if (fd < 0) { server_error = 2; return NULL; }
    struct timeval timeout = {.tv_sec = 3};
    setsockopt(fd, SOL_SOCKET, SO_RCVTIMEO, &timeout, sizeof(timeout));
    size_t used = 0;
    while (used < sizeof(captured) - 1) {
        ssize_t n = recv(fd, captured + used, sizeof(captured) - 1 - used, 0);
        if (n <= 0) { server_error = 3; break; }
        used += (size_t)n;
        captured[used] = '\0';
        if (strstr(captured, "\r\n\r\n")) break;
    }
    /* Separate writes exercise partial reads without assuming TCP boundaries. */
    for (size_t i = 0; i < strlen(reply); ++i) {
        if (send(fd, reply + i, 1, MSG_NOSIGNAL) != 1) { server_error = 4; break; }
    }
    close(fd);
    return NULL;
}

static void probe(const char *response, int expected) {
    listener = socket(AF_INET, SOCK_STREAM, 0);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, listener);
    struct sockaddr_in address = {.sin_family = AF_INET, .sin_addr.s_addr = htonl(INADDR_LOOPBACK)};
    TEST_ASSERT_EQUAL_INT(0, bind(listener, (struct sockaddr *)&address, sizeof(address)));
    TEST_ASSERT_EQUAL_INT(0, listen(listener, 1));
    socklen_t size = sizeof(address);
    TEST_ASSERT_EQUAL_INT(0, getsockname(listener, (struct sockaddr *)&address, &size));
    char url[512];
    snprintf(url, sizeof(url), "rtsp://private-user:private-password@127.0.0.1:%u/private-path?token=private-token@other-host:554#private-fragment",
             ntohs(address.sin_port));
    reply = response;
    pthread_t thread;
    TEST_ASSERT_EQUAL_INT(0, pthread_create(&thread, NULL, serve, NULL));
    char host[256]; int port;
    int result = rtsp_probe_server(url, host, sizeof(host), &port);
    TEST_ASSERT_EQUAL_INT(0, pthread_join(thread, NULL));
    TEST_ASSERT_EQUAL_INT(0, server_error);
    TEST_ASSERT_EQUAL_INT(expected, result);
    TEST_ASSERT_EQUAL_STRING("127.0.0.1", host);
    TEST_ASSERT_EQUAL_INT(ntohs(address.sin_port), port);
    TEST_ASSERT_EQUAL_STRING("OPTIONS * RTSP/1.0\r\nCSeq: 1\r\nUser-Agent: LightNVR\r\n\r\n", captured);
    close(listener); listener = -1;
}

static void test_probe_never_sends_url_data(void) {
    probe("RTSP/1.0 200 OK\r\n", 0);
}

static void test_auth_and_resource_errors_still_establish_server_availability(void) {
    probe("RTSP/1.0 401 Unauthorized\r\n", 0);
    probe("RTSP/1.0 404 Not Found\r\n", 0);
    probe("RTSP/1.0 405 Method Not Allowed\r\n", 0);
}

static void test_non_rtsp_response_is_rejected(void) {
    probe("HTTP/1.1 200 OK\r\n", -1);
    probe("RTSP/1.0 abc Invalid\r\n", -1);
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_probe_never_sends_url_data);
    RUN_TEST(test_auth_and_resource_errors_still_establish_server_availability);
    RUN_TEST(test_non_rtsp_response_is_rejected);
    return UNITY_END();
}
