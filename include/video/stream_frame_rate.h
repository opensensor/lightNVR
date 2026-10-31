/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

#ifndef STREAM_FRAME_RATE_H
#define STREAM_FRAME_RATE_H

#include <stdbool.h>
#include <stdint.h>
#include <libavutil/rational.h>

/* Metadata sanity limit for surveillance streams, not a decoder limit.
 * Reject RTP clock rates (e.g. 90000) while allowing high-rate cameras. */
#define STREAM_FRAME_RATE_MAX 240.0
#define STREAM_FRAME_RATE_DEFAULT 15

/* Prefer a credible average. r_frame_rate is only a guess and must be
 * refined from arrivals. Return zero if neither rate is credible. */
double stream_frame_rate_select(AVRational average, AVRational nominal,
                                bool *provisional);
/* The persisted schema uses integer FPS: round, with a minimum of one for
 * valid sub-FPS streams. Return zero for invalid/out-of-range values. */
int stream_frame_rate_round(double fps);

typedef struct {
    bool started;
    int64_t warmup_until_us;
    int64_t window_start_us;
    uint64_t packets;
    int fps;
} stream_frame_rate_measurement_t;

/* Zero-initialize/reset on each connection. Feed only newly-read, nonempty
 * video packets with monotonic arrival time, never media PTS or replayed
 * segment-boundary packets. Discard two seconds of startup/probe backlog,
 * then measure five seconds. Returns zero until a credible rate is ready.
 * This estimates demuxed video packet arrival rate, not decoded frame rate. */
int stream_frame_rate_observe(stream_frame_rate_measurement_t *measurement,
                              int64_t now_us);

#endif
