/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

/**
 * @file player_telemetry.c
 * @brief Client-side playback QoE telemetry ring buffer
 *
 * Simple mutex-protected ring buffer of recent player telemetry events.
 * No persistence — events are held in memory for Prometheus exposition
 * and health API responses.
 */

#include <string.h>
#include <pthread.h>

#include "telemetry/player_telemetry.h"
#define LOG_COMPONENT "PlayerTelemetry"
#include "core/logger.h"

static player_telemetry_event_t g_events[PLAYER_TELEMETRY_RING_SIZE];
static int g_head = 0;       /* next write position */
static int g_count = 0;      /* number of valid events */
static pthread_mutex_t g_mutex = PTHREAD_MUTEX_INITIALIZER;
static bool g_initialized = false;

void player_telemetry_init(void) {
    pthread_mutex_lock(&g_mutex);
    memset(g_events, 0, sizeof(g_events));
    g_head = 0;
    g_count = 0;
    g_initialized = true;
    pthread_mutex_unlock(&g_mutex);
    log_info("Player telemetry subsystem initialized (ring size: %d)", PLAYER_TELEMETRY_RING_SIZE);
}

void player_telemetry_shutdown(void) {
    pthread_mutex_lock(&g_mutex);
    g_initialized = false;
    g_head = 0;
    g_count = 0;
    pthread_mutex_unlock(&g_mutex);
    log_info("Player telemetry subsystem shut down");
}

void player_telemetry_record(const player_telemetry_event_t *event) {
    if (!event || !g_initialized) return;

    pthread_mutex_lock(&g_mutex);
    memcpy(&g_events[g_head], event, sizeof(player_telemetry_event_t));
    if (g_events[g_head].timestamp == 0) {
        g_events[g_head].timestamp = time(NULL);
    }
    g_head = (g_head + 1) % PLAYER_TELEMETRY_RING_SIZE;
    if (g_count < PLAYER_TELEMETRY_RING_SIZE) {
        g_count++;
    }
    pthread_mutex_unlock(&g_mutex);
}

int player_telemetry_snapshot(player_telemetry_event_t *out, int max_count) {
    if (!out || max_count <= 0 || !g_initialized) return 0;

    pthread_mutex_lock(&g_mutex);
    int to_copy = g_count < max_count ? g_count : max_count;

    /* Copy most-recent first */
    for (int i = 0; i < to_copy; i++) {
        int idx = (g_head - 1 - i + PLAYER_TELEMETRY_RING_SIZE) % PLAYER_TELEMETRY_RING_SIZE;
        memcpy(&out[i], &g_events[idx], sizeof(player_telemetry_event_t));
    }
    pthread_mutex_unlock(&g_mutex);
    return to_copy;
}

int player_telemetry_count(void) {
    pthread_mutex_lock(&g_mutex);
    int c = g_count;
    pthread_mutex_unlock(&g_mutex);
    return c;
}
