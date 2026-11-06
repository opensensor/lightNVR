/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

#define _POSIX_C_SOURCE 200809L
#include "video/rtsp_probe.h"
#include "core/url_utils.h"

#include <errno.h>
#include <fcntl.h>
#include <netdb.h>
#include <poll.h>
#include <stdio.h>
#include <string.h>
#include <sys/socket.h>
#include <time.h>
#include <unistd.h>

static long long monotonic_ms(void) {
    struct timespec now;
    if (clock_gettime(CLOCK_MONOTONIC, &now) != 0) return -1;
    return (long long)now.tv_sec * 1000 + now.tv_nsec / 1000000;
}

static int ready(int fd, short events, long long deadline) {
    for (;;) {
        long long now = monotonic_ms();
        if (now < 0 || now >= deadline) return -1;
        struct pollfd descriptor = {.fd = fd, .events = events};
        int rc = poll(&descriptor, 1, (int)(deadline - now));
        if (rc < 0 && errno == EINTR) continue;
        return rc > 0 && (descriptor.revents & events) ? 0 : -1;
    }
}

int rtsp_probe_server(const char *url, char *host, size_t host_size, int *port) {
    if (url_rtsp_endpoint(url, host, host_size, port) != 0) return -1;
    char service[6];
    snprintf(service, sizeof(service), "%d", *port);
    struct addrinfo hints = {.ai_family = AF_UNSPEC, .ai_socktype = SOCK_STREAM,
                            .ai_flags = AI_NUMERICSERV};
    struct addrinfo *addresses = NULL;
    if (getaddrinfo(host, service, &hints, &addresses) != 0) return -1;
    long long started = monotonic_ms();
    if (started < 0) { freeaddrinfo(addresses); return -1; }
    long long deadline = started + 2000;
    int fd = -1;
    for (struct addrinfo *a = addresses; a; a = a->ai_next) {
        fd = socket(a->ai_family, SOCK_STREAM | SOCK_CLOEXEC | SOCK_NONBLOCK, a->ai_protocol);
        if (fd < 0) continue;
        int rc = connect(fd, a->ai_addr, a->ai_addrlen);
        if (rc == 0) break;
        if (errno == EINPROGRESS && ready(fd, POLLOUT, deadline) == 0) {
            int error = 0;
            socklen_t size = sizeof(error);
            if (getsockopt(fd, SOL_SOCKET, SO_ERROR, &error, &size) == 0 && !error) break;
        }
        close(fd);
        fd = -1;
    }
    freeaddrinfo(addresses);
    if (fd < 0) return -1;

    static const char request[] = "OPTIONS * RTSP/1.0\r\nCSeq: 1\r\nUser-Agent: LightNVR\r\n\r\n";
    size_t sent = 0;
    int result = -1;
    while (sent < sizeof(request) - 1) {
        if (ready(fd, POLLOUT, deadline) != 0) goto done;
        ssize_t n = send(fd, request + sent, sizeof(request) - 1 - sent, MSG_NOSIGNAL);
        if (n < 0 && (errno == EINTR || errno == EAGAIN || errno == EWOULDBLOCK)) continue;
        if (n <= 0) goto done;
        sent += (size_t)n;
    }
    char response[256];
    size_t used = 0;
    while (used < sizeof(response) - 1) {
        if (ready(fd, POLLIN, deadline) != 0) goto done;
        ssize_t n = recv(fd, response + used, sizeof(response) - 1 - used, 0);
        if (n < 0 && (errno == EINTR || errno == EAGAIN || errno == EWOULDBLOCK)) continue;
        if (n <= 0) goto done;
        used += (size_t)n;
        response[used] = '\0';
        if (memchr(response, '\n', used)) {
            if (used >= 13 && !memcmp(response, "RTSP/1.0 ", 9) &&
                response[9] >= '1' && response[9] <= '5' &&
                response[10] >= '0' && response[10] <= '9' &&
                response[11] >= '0' && response[11] <= '9' && response[12] == ' ')
                result = 0;
            break;
        }
    }
done:
    close(fd);
    return result;
}
