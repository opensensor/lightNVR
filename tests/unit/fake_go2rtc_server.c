/** Minimal native go2rtc stand-in used by process-supervisor tests. */

#define _POSIX_C_SOURCE 200809L

#include <arpa/inet.h>
#include <errno.h>
#include <fcntl.h>
#include <netinet/in.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <time.h>
#include <unistd.h>

static void record_start(void) {
    const char *path = getenv("FAKE_GO2RTC_START_LOG");
    if (!path || path[0] == '\0') return;
    int fd = open(path, O_WRONLY | O_CREAT | O_APPEND, 0600);
    if (fd < 0) return;
    dprintf(fd, "%d\n", getpid());
    close(fd);
}

int main(int argc, char **argv) {
    if (argc > 1 && strcmp(argv[1], "--version") == 0) {
        puts("go2rtc version test");
        return 0;
    }

    record_start();
    signal(SIGPIPE, SIG_IGN);
    if (getenv("FAKE_GO2RTC_FAIL_START")) {
        return 42;
    }

    const char *port_env = getenv("FAKE_GO2RTC_PORT");
    int port = port_env ? atoi(port_env) : 0;
    if (port <= 0 || port > 65535) return 43;

    int server = socket(AF_INET, SOCK_STREAM, 0);
    if (server < 0) return 44;
    int one = 1;
    setsockopt(server, SOL_SOCKET, SO_REUSEADDR, &one, sizeof(one));

    struct sockaddr_in address;
    memset(&address, 0, sizeof(address));
    address.sin_family = AF_INET;
    address.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
    address.sin_port = htons((uint16_t)port);
    if (bind(server, (struct sockaddr *)&address, sizeof(address)) != 0 ||
        listen(server, 16) != 0) {
        close(server);
        return 45;
    }

    const char *body = "{\"version\":\"test\",\"rtsp\":{\"listen\":\":18554\"}}";
    for (;;) {
        int client = accept(server, NULL, NULL);
        if (client < 0) {
            if (errno == EINTR) continue;
            /* Keep the fixture alive across transient accept failures. */
            struct timespec retry = {.tv_sec = 0, .tv_nsec = 1000000};
            nanosleep(&retry, NULL);
            continue;
        }
        char request[1024];
        ssize_t bytes_read = read(client, request, sizeof(request));
        (void)bytes_read;
        dprintf(client,
                "HTTP/1.1 200 OK\r\n"
                "Content-Type: application/json\r\n"
                "Content-Length: %zu\r\n"
                "Connection: close\r\n\r\n%s",
                strlen(body), body);
        close(client);
    }

}
