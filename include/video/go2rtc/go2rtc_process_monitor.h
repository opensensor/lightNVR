/**
 * @file go2rtc_process_monitor.h
 * @brief Process health monitoring for go2rtc service with automatic restart
 * 
 * This module monitors the go2rtc process itself (not individual streams) and
 * automatically restarts it when it becomes unresponsive. It uses multiple
 * health indicators:
 * - API health check on port 1984
 * - Stream consensus (if all streams are down, it's likely a go2rtc issue)
 * - Process responsiveness
 */

#ifndef GO2RTC_PROCESS_MONITOR_H
#define GO2RTC_PROCESS_MONITOR_H

#include <stdbool.h>

/**
 * @brief Initialize the go2rtc process health monitor
 * 
 * Starts a background thread that periodically checks go2rtc health
 * and restarts the process if it becomes unresponsive.
 * 
 * @return true if initialization was successful, false otherwise
 */
bool go2rtc_process_monitor_init(void);

/**
 * @brief Clean up the go2rtc process health monitor
 * 
 * Stops the monitoring thread and cleans up resources.
 */
void go2rtc_process_monitor_cleanup(void);

/**
 * @brief Check if the process monitor is running
 * 
 * @return true if monitor is running, false otherwise
 */
bool go2rtc_process_monitor_is_running(void);

/**
 * @brief Get the number of times go2rtc has been restarted
 * 
 * @return Number of restarts performed by the monitor
 */
int go2rtc_process_monitor_get_restart_count(void);

/**
 * @brief Get the last restart timestamp
 * 
 * @return Unix timestamp of last restart, or 0 if never restarted
 */
time_t go2rtc_process_monitor_get_last_restart_time(void);

/**
 * @brief Force a health check (for testing/debugging)
 * 
 * @return true if go2rtc is healthy, false otherwise
 */
bool go2rtc_process_monitor_check_health(void);

#endif /* GO2RTC_PROCESS_MONITOR_H */

