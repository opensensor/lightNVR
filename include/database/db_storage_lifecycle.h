/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

#ifndef LIGHTNVR_DB_STORAGE_LIFECYCLE_H
#define LIGHTNVR_DB_STORAGE_LIFECYCLE_H

/* Schedule up to max_jobs policy-driven replication or age migration jobs. */
int db_storage_lifecycle_schedule(int max_jobs);
int db_storage_lifecycle_expire(int max_recordings);

/* Refresh persistent lifecycle violations. Returns active condition count. */
int db_storage_lifecycle_reconcile(void);

#endif
