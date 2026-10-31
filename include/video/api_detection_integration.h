/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2025-2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

#ifndef API_DETECTION_INTEGRATION_H
#define API_DETECTION_INTEGRATION_H

/**
 * Start API detection for all streams that have it configured
 * This function should be called during system initialization
 */
void start_api_detection_for_all_streams(void);

/**
 * Initialize API detection integration
 * This function should be called during system initialization
 * 
 * @return 0 on success, non-zero on failure
 */
int init_api_detection_integration(void);

#endif /* API_DETECTION_INTEGRATION_H */
