/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

/**
 * @file player_telemetry.h
 * @brief Client-side playback QoE telemetry ingestion
 *
 * Accepts QoE events from the web player (TTFF, rebuffer, WebRTC RTT)
 * via POST /api/telemetry/player and stores them in a ring buffer for
 * Prometheus exposition and health reporting.
 */

#ifndef LIGHTNVR_PLAYER_TELEMETRY_H
#define LIGHTNVR_PLAYER_TELEMETRY_H

#include <stdint.h>
#include <time.h>
#include "core/config.h"

#define PLAYER_TELEMETRY_RING_SIZE 256

/**
 * Single player telemetry event
 */
typedef struct {
    char stream_name[MAX_STREAM_NAME];
    char session_id[64];
    char transport[16];       /* "webrtc", "hls", "mse" */
    double ttff_ms;           /* time-to-first-frame */
    int rebuffer_count;
    double rebuffer_duration_ms;
    int resolution_switches;
    double webrtc_rtt_ms;
    time_t timestamp;
} player_telemetry_event_t;

/**
 * Initialize the player telemetry subsystem
 */
void player_telemetry_init(void);

/**
 * Shutdown the player telemetry subsystem
 */
void player_telemetry_shutdown(void);

/**
 * Record a player telemetry event
 *
 * @param event Pointer to the event to record (copied into ring buffer)
 */
void player_telemetry_record(const player_telemetry_event_t *event);

/**
 * Snapshot recent player telemetry events
 *
 * @param out       Pre-allocated array to receive events
 * @param max_count Size of out array
 * @return Number of events copied (most recent first)
 */
int player_telemetry_snapshot(player_telemetry_event_t *out, int max_count);

/**
 * Get the number of events currently stored
 */
int player_telemetry_count(void);

#endif /* LIGHTNVR_PLAYER_TELEMETRY_H */
