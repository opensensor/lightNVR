/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

#include "utils/webrtc_candidates.h"

#include <arpa/inet.h>
#include <stdbool.h>
#include <stdio.h>
#include <string.h>
#include <strings.h>

#include "utils/strings.h"

static bool candidate_port_ok(const char *p, size_t n) {
    if (n == 0 || n > 5) return false;
    long v = 0;
    for (size_t i = 0; i < n; i++) {
        if (p[i] < '0' || p[i] > '9') return false;
        v = v * 10 + (p[i] - '0');
    }
    return v >= 1 && v <= 65535;
}

/* Hostname or IPv4: dot-separated labels of [A-Za-z0-9-], no edge hyphens. */
static bool candidate_host_ok(const char *h, size_t n) {
    if (n == 0 || n > 253) return false;
    size_t label = 0;
    for (size_t i = 0; i < n; i++) {
        char c = h[i];
        if (c == '.') {
            if (label == 0 || h[i - 1] == '-') return false;
            label = 0;
            continue;
        }
        bool alnum = (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9');
        if (!alnum && c != '-') return false;
        if (c == '-' && label == 0) return false;
        if (++label > 63) return false;
    }
    return label > 0 && h[n - 1] != '-';
}

static bool candidate_ipv6_ok(const char *h, size_t n) {
    char text[INET6_ADDRSTRLEN];
    struct in6_addr address;
    if (n == 0 || n >= sizeof(text)) return false;
    memcpy(text, h, n);
    text[n] = '\0';
    return inet_pton(AF_INET6, text, &address) == 1;
}

/* host[:port] with an optional bracketed IPv6 host; writes "<host>:<port>". */
static bool candidate_address(const char *e, size_t n, int default_port,
                              char *out, size_t out_size) {
    const char *host = e, *port = NULL;
    size_t host_len = n, port_len = 0;
    bool v6 = false;
    if (n > 0 && e[0] == '[') {
        const char *close = memchr(e, ']', n);
        if (!close) return false;
        host = e + 1;
        host_len = (size_t)(close - host);
        size_t rest = n - (size_t)(close - e) - 1;
        if (rest > 0) {
            if (close[1] != ':') return false;
            port = close + 2;
            port_len = rest - 1;
            if (port_len == 0) return false;
        }
        if (!candidate_ipv6_ok(host, host_len)) return false;
        v6 = true;
    } else {
        const char *first = memchr(e, ':', n);
        if (first && memchr(first + 1, ':', n - (size_t)(first - e) - 1)) {
            if (!candidate_ipv6_ok(e, n)) return false;  /* bare IPv6, no port */
            v6 = true;
        } else if (first) {
            host_len = (size_t)(first - e);
            port = first + 1;
            port_len = n - host_len - 1;
            if (port_len == 0) return false;
        }
        if (!v6 && !(host_len == 1 && host[0] == '*') && !candidate_host_ok(host, host_len)) return false;
    }
    if (port && !candidate_port_ok(port, port_len)) return false;

    int w;
    if (port) {
        w = snprintf(out, out_size, v6 ? "[%.*s]:%.*s" : "%.*s:%.*s",
                     (int)host_len, host, (int)port_len, port);
    } else {
        w = snprintf(out, out_size, v6 ? "[%.*s]:%d" : "%.*s:%d", (int)host_len, host, default_port);
    }
    return w > 0 && (size_t)w < out_size;
}

int go2rtc_parse_external_candidates(const char *list, int port,
                                     char out[][GO2RTC_CANDIDATE_SIZE], int max) {
    if (!list) return 0;

    if (port <= 0 || port > 65535) port = 8555;
    int count = 0;
    const char *p = list;
    for (;;) {
        while (*p == ',' || *p == ';' || *p == ' ' || *p == '\t' || *p == '\n' || *p == '\r') p++;
        if (!*p) break;
        if (!out || max <= 0) return -1;
        const char *start = p;
        while (*p && *p != ',' && *p != ';' && *p != ' ' && *p != '\t' && *p != '\n' && *p != '\r') p++;
        size_t n = (size_t)(p - start);

        char candidate[GO2RTC_CANDIDATE_SIZE];
        bool ok;
        if ((n == 1 && start[0] == '*') || (n == 4 && strncasecmp(start, "auto", 4) == 0)) {
            ok = snprintf(candidate, sizeof(candidate), "*:%d", port) > 0;
        } else if (n == 4 && strncasecmp(start, "stun", 4) == 0) {
            ok = snprintf(candidate, sizeof(candidate), "stun:%d", port) > 0;
        } else if (n > 5 && strncasecmp(start, "stun:", 5) == 0) {
            /* go2rtc's special host "stun" discovers our public IP using
             * ice_servers. The port is our media port, not a STUN server. */
            ok = candidate_port_ok(start + 5, n - 5) &&
                 snprintf(candidate, sizeof(candidate), "stun:%.*s", (int)n - 5,
                          start + 5) < (int)sizeof(candidate);
        } else {
            ok = candidate_address(start, n, port, candidate, sizeof(candidate));
        }
        if (!ok) return -1;

        bool duplicate = false;
        for (int i = 0; i < count; i++) {
            if (strcmp(out[i], candidate) == 0) { duplicate = true; break; }
        }
        if (duplicate) continue;
        if (count >= max) return -1;
        safe_strcpy(out[count], candidate, GO2RTC_CANDIDATE_SIZE, 0);
        count++;
    }
    return count;
}

