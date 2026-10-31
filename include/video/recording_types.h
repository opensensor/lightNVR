/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2025-2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

/**
 * Header file for recording types
 */

#ifndef RECORDING_TYPES_H
#define RECORDING_TYPES_H

#include <time.h>
#include <stdint.h>
#include "../core/config.h"  // For MAX_PATH_LENGTH and MAX_STREAM_NAME

// Structure for active recording tracking
typedef struct {
    uint64_t recording_id;                // Database ID of the recording
    char stream_name[MAX_STREAM_NAME];    // Name of the stream being recorded
    char output_path[MAX_PATH_LENGTH];    // Path where recording is stored
    time_t start_time;                    // When the recording started
} active_recording_t;

#endif /* RECORDING_TYPES_H */
