/**
 * @file go2rtc_process.h
 * @brief Module for managing the go2rtc process lifecycle
 */

#ifndef GO2RTC_PROCESS_H
#define GO2RTC_PROCESS_H

#include <stdbool.h>

/**
 * @brief Initialize the go2rtc process manager
 * 
 * @param binary_path Path to the go2rtc binary
 * @param config_dir Directory to store go2rtc configuration files
 * @return true if initialization was successful, false otherwise
 */
bool go2rtc_process_init(const char *binary_path, const char *config_dir);

/**
 * @brief Start the go2rtc process
 * 
 * @param api_port Port for the go2rtc HTTP API
 * @return true if process started successfully, false otherwise
 */
bool go2rtc_process_start(int api_port);

/**
 * @brief Stop the go2rtc process
 * 
 * @return true if process stopped successfully, false otherwise
 */
bool go2rtc_process_stop(void);

/**
 * @brief Check if the go2rtc process is running
 * 
 * @return true if process is running, false otherwise
 */
bool go2rtc_process_is_running(void);

/**
 * @brief Generate a go2rtc configuration file based on system settings
 * 
 * @param config_path Path where the configuration file should be written
 * @param api_port Port for the go2rtc HTTP API
 * @return true if configuration was generated successfully, false otherwise
 */
bool go2rtc_process_generate_config(const char *config_path, int api_port);

/**
 * @brief Clean up resources used by the go2rtc process manager
 */
void go2rtc_process_cleanup(void);

/**
 * @brief Get the RTSP port used by go2rtc
 *
 * @return int The RTSP port
 */
int go2rtc_process_get_rtsp_port(void);

/**
 * @brief Get the PID of the go2rtc process
 *
 * @return pid_t The process ID, or -1 if not running
 */
int go2rtc_process_get_pid(void);

#endif /* GO2RTC_PROCESS_H */
