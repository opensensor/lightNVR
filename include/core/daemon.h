/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2025-2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

#ifndef DAEMON_H
#define DAEMON_H

/**
 * Initialize daemon mode
 *
 * @param pid_file Path to PID file, or NULL for default
 * @return Locked PID file descriptor on success, -1 on error
 */
int init_daemon(const char *pid_file);

/**
 * Get status of daemon
 *
 * @param pid_file Path to PID file, or NULL for default
 * @return 1 if running, 0 if not running, -1 on error
 */
int daemon_status(const char *pid_file);

/**
* Write PID file
*
* @param pid_file Path to PID file
* @return The (locked) PID file descriptor on success, -1 on error
*/
int write_pid_file(const char *pid_file);

/**
* Unlock and remove PID file
*
* @param pid_file Path to PID file
* @return 0 on success, -1 on error
*/
int remove_pid_file(int fd, const char *pid_file);

#endif /* DAEMON_H */