/**
 * Multi-source registration must not send go2rtc a shortened request: it would
 * register the wrong stream instead of letting the caller fall back.
 */
#define _GNU_SOURCE

#include <arpa/inet.h>
#include <errno.h>
#include <netinet/in.h>
#include <pthread.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/time.h>
#include <unistd.h>

#include "unity.h"
#include "video/go2rtc/go2rtc_api.h"

#define LONG_SOURCE_LENGTH 4000

static int listen_fd = -1;
static pthread_t server_thread;
static pthread_mutex_t request_lock = PTHREAD_MUTEX_INITIALIZER;
static int request_count;
static char last_request[8192];

static int get_request_count(void) {
    int count;
    pthread_mutex_lock(&request_lock);
    count = request_count;
    pthread_mutex_unlock(&request_lock);
    return count;
}

static void get_last_request_snapshot(char *dst, size_t dst_size) {
    pthread_mutex_lock(&request_lock);
    if (dst_size > 0) {
        strncpy(dst, last_request, dst_size - 1);
        dst[dst_size - 1] = '\0';
    }
    pthread_mutex_unlock(&request_lock);
}

static void *serve(void *arg) {
    (void)arg;
    for (;;) {
        int client = accept(listen_fd, NULL, NULL);
        if (client < 0) return NULL;
        struct timeval timeout = {.tv_sec = 2};
        if (setsockopt(client, SOL_SOCKET, SO_RCVTIMEO, &timeout, sizeof(timeout)) != 0) {
            close(client);
            continue;
        }
        char request[sizeof(last_request)];
        size_t length = 0;
        request[0] = '\0';
        while (length < sizeof(request) - 1) {
            /* Force short reads so this fixture exercises header accumulation
             * even when the loopback socket already holds the whole request. */
            size_t chunk = sizeof(request) - 1 - length;
            if (chunk > 7) chunk = 7;
            ssize_t got = recv(client, request + length, chunk, 0);
            if (got < 0 && errno == EINTR) continue;
            if (got <= 0) break;
            length += (size_t)got;
            request[length] = '\0';
            if (strstr(request, "\r\n\r\n") != NULL) break;
        }
        if (strstr(request, "\r\n\r\n") == NULL) {
            close(client);
            continue;
        }
        pthread_mutex_lock(&request_lock);
        memcpy(last_request, request, length + 1);
        request_count++;
        pthread_mutex_unlock(&request_lock);
        static const char reply[] =
            "HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\n{}";
        if (send(client, reply, sizeof(reply) - 1, MSG_NOSIGNAL) < 0) { /* test asserts on count */ }
        close(client);
    }
}

void setUp(void) {
    pthread_mutex_lock(&request_lock);
    request_count = 0;
    last_request[0] = '\0';
    pthread_mutex_unlock(&request_lock);
}

void tearDown(void) {
}

static void test_request_that_fits_carries_every_source_and_the_name(void) {
    char request_snapshot[sizeof(last_request)];
    const char *sources[] = {"rtsp://camera/stream", "ffmpeg:cam#video=h264"};
    TEST_ASSERT_TRUE(go2rtc_api_add_stream_multi("cam", sources, 2));
    TEST_ASSERT_EQUAL_INT(1, get_request_count());
    get_last_request_snapshot(request_snapshot, sizeof(request_snapshot));
    TEST_ASSERT_NOT_NULL(strstr(request_snapshot, "src=rtsp%3A%2F%2Fcamera%2Fstream"));
    TEST_ASSERT_NOT_NULL(strstr(request_snapshot, "&src=ffmpeg%3Acam%23video%3Dh264"));
    TEST_ASSERT_NOT_NULL(strstr(request_snapshot, "&name=cam"));
    TEST_ASSERT_NOT_NULL(strstr(request_snapshot, "\r\n\r\n"));
}

static void test_sources_past_the_request_buffer_send_nothing(void) {
    /* Each survives escaping whole; only together do they overflow the request. */
    static char first[2001], second[2001];
    memset(first, 'a', sizeof(first) - 1);
    memset(second, 'b', sizeof(second) - 1);
    const char *sources[] = {first, second};
    TEST_ASSERT_FALSE(go2rtc_api_add_stream_multi("cam", sources, 2));
    TEST_ASSERT_EQUAL_INT(0, get_request_count());
}

static void test_source_past_the_escape_buffer_sends_nothing(void) {
    /* Enough sources that the shortened text would still fit the request. */
    static char long_source[LONG_SOURCE_LENGTH + 1];
    memset(long_source, 'a', LONG_SOURCE_LENGTH);
    const char *sources[] = {long_source, "b", "c", "d"};
    TEST_ASSERT_FALSE(go2rtc_api_add_stream_multi("cam", sources, 4));
    TEST_ASSERT_EQUAL_INT(0, get_request_count());
}

static void test_null_source_sends_nothing(void) {
    const char *sources[] = {"rtsp://camera/stream", NULL};
    TEST_ASSERT_FALSE(go2rtc_api_add_stream_multi("cam", sources, 2));
    TEST_ASSERT_EQUAL_INT(0, get_request_count());
}

static void test_percent_encoded_source_overflow_sends_nothing(void) {
    /* Escaping stops two bytes short of the buffer's end when %XX won't fit. */
    static char source[1025];
    memset(source, '%', sizeof(source) - 1);
    const char *sources[] = {source, "b", "c", "d"};
    TEST_ASSERT_FALSE(go2rtc_api_add_stream_multi("cam", sources, 4));
    TEST_ASSERT_EQUAL_INT(0, get_request_count());
}

static void test_truncated_stream_name_sends_nothing(void) {
    static char name[1025];
    memset(name, '%', sizeof(name) - 1);
    const char *sources[] = {"rtsp://camera/stream"};
    TEST_ASSERT_FALSE(go2rtc_api_add_stream_multi(name, sources, 1));
    TEST_ASSERT_EQUAL_INT(0, get_request_count());
}


static void test_incomplete_request_is_not_published(void) {
    struct sockaddr_in address;
    socklen_t length = sizeof(address);
    TEST_ASSERT_EQUAL_INT(0, getsockname(listen_fd, (struct sockaddr *)&address, &length));
    int client = socket(AF_INET, SOCK_STREAM, 0);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, client);
    int connected = connect(client, (struct sockaddr *)&address, length);
    static const char partial[] = "PUT /api/streams?src=incomplete";
    ssize_t sent = connected == 0 ? send(client, partial, sizeof(partial) - 1, MSG_NOSIGNAL) : -1;
    shutdown(client, SHUT_WR);
    char reply[128];
    ssize_t received = connected == 0 ? recv(client, reply, sizeof(reply), 0) : -1;
    close(client);
    TEST_ASSERT_EQUAL_INT(0, connected);
    TEST_ASSERT_EQUAL_INT(sizeof(partial) - 1, sent);
    TEST_ASSERT_EQUAL_INT(0, received);
    TEST_ASSERT_EQUAL_INT(0, get_request_count());
    char snapshot[sizeof(last_request)];
    get_last_request_snapshot(snapshot, sizeof(snapshot));
    TEST_ASSERT_EQUAL_STRING("", snapshot);
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
    RUN_TEST(test_incomplete_request_is_not_published);
    RUN_TEST(test_request_that_fits_carries_every_source_and_the_name);
    RUN_TEST(test_sources_past_the_request_buffer_send_nothing);
    RUN_TEST(test_source_past_the_escape_buffer_sends_nothing);
    RUN_TEST(test_null_source_sends_nothing);
    RUN_TEST(test_percent_encoded_source_overflow_sends_nothing);
    RUN_TEST(test_truncated_stream_name_sends_nothing);
    int result = UNITY_END();

    go2rtc_api_cleanup();
    shutdown(listen_fd, SHUT_RDWR);
    close(listen_fd);
    pthread_join(server_thread, NULL);
    return result;
}
