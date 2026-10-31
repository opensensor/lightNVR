/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2025-2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

#ifndef LIGHTNVR_DB_TRANSACTION_H
#define LIGHTNVR_DB_TRANSACTION_H

/**
 * Begin a database transaction
 * 
 * @return 0 on success, non-zero on failure
 */
int begin_transaction(void);

/**
 * Commit a database transaction
 * 
 * @return 0 on success, non-zero on failure
 */
int commit_transaction(void);

/**
 * Rollback a database transaction
 * 
 * @return 0 on success, non-zero on failure
 */
int rollback_transaction(void);

#endif // LIGHTNVR_DB_TRANSACTION_H
