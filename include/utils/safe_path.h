/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 */
#ifndef LIGHTNVR_SAFE_PATH_H
#define LIGHTNVR_SAFE_PATH_H

#include <errno.h>
#include <fcntl.h>
#include <limits.h>
#include <string.h>
#include <unistd.h>

/* Return a new descriptor beneath parent without following any symlinks.
 * The caller retains parent and owns the returned descriptor. Empty paths
 * refer to parent; a single trailing slash is allowed. */
static inline int safe_path_open_directory_at(int parent, const char *relative) {
    if (!relative || relative[0] == '/') {
        errno = EINVAL;
        return -1;
    }
    int directory = fcntl(parent, F_DUPFD_CLOEXEC, 0);
    if (directory < 0) return -1;
    while (*relative) {
        char component[NAME_MAX + 1];
        size_t length = strcspn(relative, "/");
        if (length == 0 || length >= sizeof(component)) {
            close(directory);
            errno = EINVAL;
            return -1;
        }
        memcpy(component, relative, length);
        component[length] = '\0';
        /* Each open receives exactly one name, never a path or traversal. */
        if (strchr(component, '/') || strcmp(component, ".") == 0 ||
            strcmp(component, "..") == 0) {
            close(directory);
            errno = EINVAL;
            return -1;
        }
        int next = openat(directory, component,
                          O_RDONLY | O_DIRECTORY | O_CLOEXEC | O_NOFOLLOW);
        int saved_errno = errno;
        close(directory);
        if (next < 0) {
            errno = saved_errno;
            return -1;
        }
        directory = next;
        relative += length;
        if (*relative == '/') ++relative;
    }
    return directory;
}

#endif
