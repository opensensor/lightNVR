/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2025-2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

#ifndef THREAD_UTILS_H
#define THREAD_UTILS_H

#include <pthread.h>

// Join a thread with timeout
int pthread_join_with_timeout(pthread_t thread, void **retval, int timeout_sec);

#endif // THREAD_UTILS_H
