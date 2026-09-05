#define _POSIX_C_SOURCE 200809L

#include <arpa/inet.h>
#include <netinet/in.h>
#include <pthread.h>
#include <stdbool.h>
#include <string.h>
#include <sys/socket.h>
#include <time.h>
#include <unistd.h>

#include <curl/curl.h>
#include <unity.h>

#include "video/go2rtc/go2rtc_api.h"

typedef struct {
    int listen_fd;
    bool received_request;
} delayed_server_t;

static long long monotonic_milliseconds(void) {
    struct timespec now;
    clock_gettime(CLOCK_MONOTONIC, &now);
    return (long long)now.tv_sec * 1000LL + now.tv_nsec / 1000000LL;
}

static void *delayed_server_worker(void *arg) {
    delayed_server_t *server = (delayed_server_t *)arg;
    int client = accept(server->listen_fd, NULL, NULL);
    if (client < 0) {
        return NULL;
    }

    char request[2048];
    ssize_t bytes_read = read(client, request, sizeof(request) - 1);
    if (bytes_read > 0) {
        request[bytes_read] = '\0';
        server->received_request = strstr(request, "POST ") == request;
    }

    /* Model an external ingest connection that delays go2rtc's API response. */
    struct timespec delay = {.tv_sec = 0, .tv_nsec = 700000000L};
    nanosleep(&delay, NULL);

    static const char response[] =
        "HTTP/1.1 200 OK\r\n"
        "Content-Length: 2\r\n"
        "Connection: close\r\n\r\n{}";
    (void)send(client, response, sizeof(response) - 1, MSG_NOSIGNAL);
    close(client);
    return NULL;
}

static bool start_delayed_server(delayed_server_t *server, int *port,
                                 pthread_t *thread) {
    memset(server, 0, sizeof(*server));
    server->listen_fd = socket(AF_INET, SOCK_STREAM, 0);
    if (server->listen_fd < 0) {
        return false;
    }

    int one = 1;
    setsockopt(server->listen_fd, SOL_SOCKET, SO_REUSEADDR, &one, sizeof(one));

    struct sockaddr_in address;
    memset(&address, 0, sizeof(address));
    address.sin_family = AF_INET;
    address.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
    address.sin_port = 0;
    if (bind(server->listen_fd, (struct sockaddr *)&address, sizeof(address)) != 0 ||
        listen(server->listen_fd, 1) != 0) {
        close(server->listen_fd);
        return false;
    }

    socklen_t address_size = sizeof(address);
    if (getsockname(server->listen_fd, (struct sockaddr *)&address,
                    &address_size) != 0) {
        close(server->listen_fd);
        return false;
    }
    *port = ntohs(address.sin_port);

    if (pthread_create(thread, NULL, delayed_server_worker, server) != 0) {
        close(server->listen_fd);
        return false;
    }
    return true;
}

void setUp(void) {}
void tearDown(void) {}

void test_publish_job_does_not_block_caller_and_cleanup_drains_it(void) {
    delayed_server_t server;
    pthread_t server_thread;
    int port = 0;

    TEST_ASSERT_EQUAL_INT(CURLE_OK, curl_global_init(CURL_GLOBAL_ALL));
    TEST_ASSERT_TRUE(start_delayed_server(&server, &port, &server_thread));
    TEST_ASSERT_TRUE(go2rtc_api_init("127.0.0.1", port));

    long long started = monotonic_milliseconds();
    TEST_ASSERT_TRUE(go2rtc_api_publish_stream_async(
        "back-door", "rtmp://example.invalid/live/secret"));
    long long scheduled = monotonic_milliseconds();

    /* The HTTP peer waits 700 ms, but scheduling must return immediately. */
    TEST_ASSERT_LESS_THAN_INT64(400, scheduled - started);

    go2rtc_api_cleanup();
    long long cleaned_up = monotonic_milliseconds();

    TEST_ASSERT_GREATER_THAN_INT64(500, cleaned_up - started);
    TEST_ASSERT_EQUAL_INT(0, pthread_join(server_thread, NULL));
    TEST_ASSERT_TRUE(server.received_request);

    close(server.listen_fd);
    curl_global_cleanup();
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_publish_job_does_not_block_caller_and_cleanup_drains_it);
    return UNITY_END();
}
