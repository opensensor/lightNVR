#define _POSIX_C_SOURCE 200809L

#include <arpa/inet.h>
#include <pthread.h>
#include <stdbool.h>
#include <stdio.h>
#include <string.h>
#include <sys/select.h>
#include <sys/socket.h>
#include <unistd.h>

#include "unity.h"
#include "core/logger.h"
#include "core/shutdown_coordinator.h"
#include "video/detection.h"
#include "video/onvif_detection.h"

typedef struct {
    int listen_fd;
    int port;
    int connections;
    bool stop;
    pthread_t thread;
} fake_onvif_server_t;

static void send_xml_response(int client_fd, const char *body) {
    char response[1024];
    int len = snprintf(response, sizeof(response),
                       "HTTP/1.1 200 OK\r\nContent-Type: application/soap+xml\r\n"
                       "Content-Length: %zu\r\nConnection: close\r\n\r\n%s",
                       strlen(body), body);
    send(client_fd, response, (size_t)len, 0);
}

static void *fake_onvif_server_main(void *arg) {
    fake_onvif_server_t *server = (fake_onvif_server_t *)arg;
    while (!server->stop && server->connections < 2) {
        fd_set readfds;
        struct timeval timeout = {.tv_sec = 0, .tv_usec = 100000};
        FD_ZERO(&readfds);
        FD_SET(server->listen_fd, &readfds);
        int ready = select(server->listen_fd + 1, &readfds, NULL, NULL, &timeout);
        if (ready <= 0) continue;

        int client_fd = accept(server->listen_fd, NULL, NULL);
        if (client_fd < 0) continue;

        char request[512];
        (void)recv(client_fd, request, sizeof(request), 0);
        server->connections++;

        if (server->connections == 1) {
            char body[512];
            snprintf(body, sizeof(body),
                     "<Envelope><Body><wsa:Address>http://127.0.0.1:%d/pull_service</wsa:Address>"
                     "</Body></Envelope>",
                     server->port);
            send_xml_response(client_fd, body);
        } else {
            send_xml_response(client_fd, "<Envelope><Body><PullMessagesResponse/></Body></Envelope>");
        }

        close(client_fd);
    }

    close(server->listen_fd);
    server->listen_fd = -1;
    return NULL;
}

static int start_fake_onvif_server(fake_onvif_server_t *server) {
    memset(server, 0, sizeof(*server));
    server->listen_fd = socket(AF_INET, SOCK_STREAM, 0);
    if (server->listen_fd < 0) return -1;

    struct sockaddr_in addr;
    memset(&addr, 0, sizeof(addr));
    addr.sin_family = AF_INET;
    addr.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
    addr.sin_port = 0;

    if (bind(server->listen_fd, (struct sockaddr *)&addr, sizeof(addr)) != 0) return -1;
    if (listen(server->listen_fd, 4) != 0) return -1;

    socklen_t addr_len = sizeof(addr);
    if (getsockname(server->listen_fd, (struct sockaddr *)&addr, &addr_len) != 0) return -1;
    server->port = ntohs(addr.sin_port);

    return pthread_create(&server->thread, NULL, fake_onvif_server_main, server);
}

static void stop_fake_onvif_server(fake_onvif_server_t *server) {
    server->stop = true;
    pthread_join(server->thread, NULL);
}

void setUp(void) {
    init_shutdown_coordinator();
}

void tearDown(void) {
    shutdown_detection_system();
    shutdown_coordinator_cleanup();
}

void test_init_detection_system_initializes_onvif_detection(void) {
    fake_onvif_server_t server;
    TEST_ASSERT_EQUAL_INT(0, start_fake_onvif_server(&server));
    TEST_ASSERT_EQUAL_INT(0, init_detection_system());

    char url[64];
    snprintf(url, sizeof(url), "http://127.0.0.1:%d", server.port);

    detection_result_t result;
    memset(&result, 0, sizeof(result));
    TEST_ASSERT_EQUAL_INT(0, detect_motion_onvif(url, "", "", &result, ""));

    stop_fake_onvif_server(&server);
    TEST_ASSERT_EQUAL_INT(2, server.connections);
    TEST_ASSERT_EQUAL_INT(0, result.count);
}

int main(void) {
    init_logger();
    UNITY_BEGIN();
    RUN_TEST(test_init_detection_system_initializes_onvif_detection);
    int result = UNITY_END();
    shutdown_logger();
    return result;
}