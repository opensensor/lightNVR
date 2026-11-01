/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

/**
 * @file signal_safe.h
 * @brief Async-signal-safe helpers shared by the process signal handlers.
 */

#ifndef LIGHTNVR_SIGNAL_SAFE_H
#define LIGHTNVR_SIGNAL_SAFE_H

#include <errno.h>
#include <string.h>
#include <unistd.h>

/**
 * Write a NUL-terminated message to stderr from a signal handler.
 *
 * Only uses functions on the POSIX async-signal-safe list (strlen() is on it
 * since POSIX.1-2008 TC2, write()). Retries on EINTR and short writes, and
 * preserves errno so the interrupted code still sees its own error value.
 */
static inline void signal_safe_write(const char *msg) {
    if (!msg) {
        return;
    }
    int saved_errno = errno;
    size_t remaining = strlen(msg);
    while (remaining > 0) {
        ssize_t written = write(STDERR_FILENO, msg, remaining);
        if (written > 0) {
            msg += written;
            remaining -= (size_t)written;
        } else if (written < 0 && errno == EINTR) {
            continue;
        } else {
            break;  // stderr closed or broken: nothing useful left to do
        }
    }
    errno = saved_errno;
}

#endif /* LIGHTNVR_SIGNAL_SAFE_H */
