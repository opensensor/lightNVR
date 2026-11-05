/**
 * Multi-source registration must not send go2rtc a shortened request: it would
 * register the wrong stream instead of letting the caller fall back.
 */
#define _GNU_SOURCE

#include <arpa/inet.h>
#include <netinet/in.h>
#include <pthread.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <unistd.h>

#include "unity.h"
#include "video/go2rtc/go2rtc_api.h"

#define LONG_SOURCE_LENGTH 4000

static int listen_fd = -1;
static pthread_t server_thread;
static volatile int request_count;
static char last_request[8192];

static void *serve(void *arg) {
    (void)arg;
    for (;;) {
        int client = accept(listen_fd, NULL, NULL);
        if (client < 0) return NULL;
        ssize_t got = recv(client, last_request, sizeof(last_request) - 1, 0);
        last_request[got > 0 ? got : 0] = '\0';
        request_count++;
        static const char reply[] =
            "HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\n{}";
        if (send(client, reply, sizeof(reply) - 1, MSG_NOSIGNAL) < 0) { /* test asserts on count */ }
        close(client);
    }
}

void setUp(void) {
    request_count = 0;
    last_request[0] = '\0';
}

void tearDown(void) {
}

static void test_request_that_fits_carries_every_source_and_the_name(void) {
    const char *sources[] = {"rtsp://camera/stream", "ffmpeg:cam#video=h264"};
    TEST_ASSERT_TRUE(go2rtc_api_add_stream_multi("cam", sources, 2));
    TEST_ASSERT_EQUAL_INT(1, request_count);
    TEST_ASSERT_NOT_NULL(strstr(last_request, "src=rtsp%3A%2F%2Fcamera%2Fstream"));
    TEST_ASSERT_NOT_NULL(strstr(last_request, "&src=ffmpeg%3Acam"));
    TEST_ASSERT_NOT_NULL(strstr(last_request, "&name=cam"));
}

static void test_sources_past_the_request_buffer_send_nothing(void) {
    /* Each survives escaping whole; only together do they overflow the request. */
    static char first[2001], second[2001];
    memset(first, 'a', sizeof(first) - 1);
    memset(second, 'b', sizeof(second) - 1);
    const char *sources[] = {first, second};
    TEST_ASSERT_FALSE(go2rtc_api_add_stream_multi("cam", sources, 2));
    TEST_ASSERT_EQUAL_INT(0, request_count);
}

static void test_source_past_the_escape_buffer_sends_nothing(void) {
    /* Enough sources that the shortened text would still fit the request. */
    static char long_source[LONG_SOURCE_LENGTH + 1];
    memset(long_source, 'a', LONG_SOURCE_LENGTH);
    const char *sources[] = {long_source, "b", "c", "d"};
    TEST_ASSERT_FALSE(go2rtc_api_add_stream_multi("cam", sources, 4));
    TEST_ASSERT_EQUAL_INT(0, request_count);
}

static void test_null_source_sends_nothing(void) {
    const char *sources[] = {"rtsp://camera/stream", NULL};
    TEST_ASSERT_FALSE(go2rtc_api_add_stream_multi("cam", sources, 2));
    TEST_ASSERT_EQUAL_INT(0, request_count);
}

int main(void) {
    setenv("NO_PROXY", "127.0.0.1", 1);
    setenv("no_proxy", "127.0.0.1", 1);

    listen_fd = socket(AF_INET, SOCK_STREAM, 0);
    struct sockaddr_in address = {.sin_family = AF_INET,
                                  .sin_addr.s_addr = htonl(INADDR_LOOPBACK)};
    socklen_t length = sizeof(address);
    if (listen_fd < 0 || bind(listen_fd, (struct sockaddr *)&address, sizeof(address)) != 0 ||
        listen(listen_fd, 4) != 0 ||
        getsockname(listen_fd, (struct sockaddr *)&address, &length) != 0 ||
        pthread_create(&server_thread, NULL, serve, NULL) != 0) {
        return 1;
    }
    if (!go2rtc_api_init("127.0.0.1", ntohs(address.sin_port))) return 1;

    UNITY_BEGIN();
    RUN_TEST(test_request_that_fits_carries_every_source_and_the_name);
    RUN_TEST(test_sources_past_the_request_buffer_send_nothing);
    RUN_TEST(test_source_past_the_escape_buffer_sends_nothing);
    RUN_TEST(test_null_source_sends_nothing);
    int result = UNITY_END();

    go2rtc_api_cleanup();
    shutdown(listen_fd, SHUT_RDWR);
    close(listen_fd);
    pthread_join(server_thread, NULL);
    return result;
}
