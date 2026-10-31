/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2025-2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

#ifndef DNS_CLEANUP_H
#define DNS_CLEANUP_H

/**
 * @brief Clean up DNS resolver resources to prevent memory leaks
 * 
 * This function forces the DNS resolver to release any cached memory
 * that might be leaked during DNS resolution operations.
 */
void cleanup_dns_resolver(void);

#endif /* DNS_CLEANUP_H */
