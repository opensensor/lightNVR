/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

#include "core/sentry.h"

#ifdef ENABLE_SENTRY
#include <sentry.h>
#include <stdlib.h>
#include <string.h>
#include "core/logger.h"

static int sentry_started;

void lightnvr_sentry_shutdown(void) {
    if (sentry_started) {
        sentry_started = 0;
        sentry_close();
    }
}

void lightnvr_sentry_init(void) {
    const char *dsn = getenv("SENTRY_DSN");
    const char *enabled = getenv("SENTRY_ENABLED");
    if (!dsn || !*dsn || (enabled && strcmp(enabled, "false") == 0)) return;

    const char *environment = getenv("SENTRY_ENVIRONMENT");
    const char *release = getenv("SENTRY_RELEASE");
    const char *database_path = getenv("SENTRY_DATABASE_PATH");
    sentry_options_t *options = sentry_options_new();
    if (!options) return;
    sentry_options_set_dsn(options, dsn);
    sentry_options_set_environment(options, environment ? environment : "production");
    if (release && *release) sentry_options_set_release(options, release);
    sentry_options_set_database_path(options, database_path ? database_path : "/var/lib/lightnvr/data/sentry");
    sentry_options_set_symbolize_stacktraces(options, 1);
    if (sentry_init(options) == 0) {
        sentry_started = 1;
        atexit(lightnvr_sentry_shutdown);
        sentry_set_tag("component", "recorder");
    } else {
        log_warn("Sentry initialization failed; recorder operation continues");
    }
}
#else
void lightnvr_sentry_init(void) {}
void lightnvr_sentry_shutdown(void) {}
#endif
