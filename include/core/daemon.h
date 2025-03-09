#ifndef DAEMON_H
#define DAEMON_H

/**
 * Initialize daemon mode
 *
 * @param pid_file Path to PID file, or NULL for default
 * @return 0 on success, -1 on error
 */
int init_daemon(const char *pid_file);

/**
 * Cleanup daemon resources
 *
 * @return 0 on success, -1 on error
 */
int cleanup_daemon(void);

/**
 * Stop running daemon
 *
 * @param pid_file Path to PID file, or NULL for default
 * @return 0 on success, -1 on error
 */
int stop_daemon(const char *pid_file);

/**
 * Get status of daemon
 *
 * @param pid_file Path to PID file, or NULL for default
 * @return 1 if running, 0 if not running, -1 on error
 */
int daemon_status(const char *pid_file);


/**
* Remove Daemon PID file
*
* @param pid_file Path to PID file
* @return 0 on success, -1 on error
*/
int remove_daemon_pid_file(const char *pid_file);

#endif /* DAEMON_H */