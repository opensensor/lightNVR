#define _POSIX_C_SOURCE 200809L
#include <arpa/inet.h>
#include <pthread.h>
#include <stdio.h>
#include <string.h>
#include <sys/socket.h>
#include <unistd.h>
#include <curl/curl.h>
#include "unity.h"
#include "video/go2rtc/go2rtc_api.h"

void setUp(void) { curl_global_init(CURL_GLOBAL_ALL); }
void tearDown(void) { go2rtc_api_cleanup(); curl_global_cleanup(); }

typedef struct { int fd; const char *address; } fixture_t;
static void *serve_info(void *arg) {
    fixture_t *fixture = arg;
    int client = accept(fixture->fd, NULL, NULL);
    if (client < 0) return NULL;
    char request[1024];
    ssize_t received = read(client, request, sizeof(request));
    if (received > 0) {
        char response[512];
        int length = snprintf(response, sizeof(response),
            "HTTP/1.1 200 OK\r\nConnection: close\r\n\r\n"
            "{\"version\":\"fixture\",\"rtsp\":{\"listen\":\"%s\"}}", fixture->address);
        (void)send(client, response, (size_t)length, MSG_NOSIGNAL);
    }
    close(client);
    return NULL;
}

void test_rtsp_listener_port_is_read_from_wildcard_ipv4_and_ipv6_addresses(void) {
    const struct { const char *address; int port; } cases[] = {
        {":18554", 18554}, {"127.0.0.1:31554", 31554}, {"[::1]:41554", 41554},
        {"localhost:1", 1}, {"[::]:65535", 65535}, {"", 8554},
        {"127.0.0.1:0", 8554}, {":65536", 8554}, {":-1", 8554},
        {":+123", 8554}, {":123junk", 8554}, {":99999999999999999999", 8554},
    };
    for (size_t i = 0; i < sizeof(cases) / sizeof(cases[0]); i++) {
        fixture_t fixture = {.fd = socket(AF_INET, SOCK_STREAM, 0), .address = cases[i].address};
        TEST_ASSERT_GREATER_OR_EQUAL_INT(0, fixture.fd);
        struct sockaddr_in address = {.sin_family = AF_INET,
            .sin_addr.s_addr = htonl(INADDR_LOOPBACK)};
        TEST_ASSERT_EQUAL_INT(0, bind(fixture.fd, (struct sockaddr *)&address, sizeof(address)));
        TEST_ASSERT_EQUAL_INT(0, listen(fixture.fd, 1));
        socklen_t size = sizeof(address);
        TEST_ASSERT_EQUAL_INT(0, getsockname(fixture.fd, (struct sockaddr *)&address, &size));
        TEST_ASSERT_TRUE(go2rtc_api_init("127.0.0.1", ntohs(address.sin_port)));
        pthread_t thread;
        TEST_ASSERT_EQUAL_INT(0, pthread_create(&thread, NULL, serve_info, &fixture));
        int port = 0;
        bool ok = go2rtc_api_get_server_info(&port);
        pthread_join(thread, NULL);
        close(fixture.fd);
        go2rtc_api_cleanup();
        TEST_ASSERT_TRUE(ok);
        TEST_ASSERT_EQUAL_INT_MESSAGE(cases[i].port, port, cases[i].address);
    }
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_rtsp_listener_port_is_read_from_wildcard_ipv4_and_ipv6_addresses);
    return UNITY_END();
}
