/**
 * @file go2rtc_health_monitor.h
 * @brief Health monitoring for go2rtc streams to handle camera disconnections
 */

#ifndef GO2RTC_HEALTH_MONITOR_H
#define GO2RTC_HEALTH_MONITOR_H

#include <stdbool.h>

/**
 * @brief Initialize the go2rtc health monitor
 * 
 * This starts a background thread that periodically checks the health of
 * streams registered with go2rtc and automatically re-registers them when
 * cameras disconnect and reconnect.
 * 
 * @return true if initialization was successful, false otherwise
 */
bool go2rtc_health_monitor_init(void);

/**
 * @brief Clean up the go2rtc health monitor
 * 
 * This stops the health monitor thread and cleans up resources.
 */
void go2rtc_health_monitor_cleanup(void);

/**
 * @brief Check if the health monitor is running
 * 
 * @return true if the health monitor is running, false otherwise
 */
bool go2rtc_health_monitor_is_running(void);

#endif /* GO2RTC_HEALTH_MONITOR_H */

