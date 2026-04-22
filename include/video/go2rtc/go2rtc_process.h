/**
 * @file go2rtc_process.h
 * @brief Module for managing the go2rtc process lifecycle
 */

#ifndef GO2RTC_PROCESS_H
#define GO2RTC_PROCESS_H

#include <stdbool.h>
#include <stddef.h>

/**
 * @brief Initialize the go2rtc process manager
 * 
 * @param binary_path Path to the go2rtc binary
 * @param config_dir Directory to store go2rtc configuration files
 * @param api_port Port for the go2rtc HTTP API (used for service detection)
 * @return true if initialization was successful, false otherwise
 */
bool go2rtc_process_init(const char *binary_path, const char *config_dir, int api_port);

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
 * After the T2 refactor this emits ONLY the lightNVR-owned sections
 * (`api`, `rtsp`, `webrtc`, `ffmpeg` defaults, and `streams` with per-stream
 * overrides). The global user override from `system_settings.go2rtc_config_override`
 * is NO LONGER appended here — it is written to a separate override file by
 * go2rtc_process_generate_override_file() and passed to go2rtc via a second
 * `--config` argument.
 *
 * @param config_path Path where the configuration file should be written
 * @param api_port Port for the go2rtc HTTP API
 * @return true if configuration was generated successfully, false otherwise
 */
bool go2rtc_process_generate_config(const char *config_path, int api_port);

/**
 * @brief Write the user config override to a standalone YAML file.
 *
 * Reads `go2rtc_config_override` from `db_system_settings` and writes it to
 * @p override_path with mode 0600. If the DB value is empty or absent, any
 * existing file at @p override_path is removed so a stale override is never
 * passed to go2rtc.
 *
 * NOTE: This is a STUB declaration placed here by T2 so that T3 and T4 can
 * compile independently. T3 provides the real implementation.
 *
 * @param override_path Filesystem path where the override YAML should be written.
 * @return 0 on success (file written or confirmed absent), -1 on failure.
 */
int go2rtc_process_generate_override_file(const char *override_path);

/**
 * @brief Return the path to the user override file, or NULL if not configured.
 *
 * NOTE: This is a STUB declaration placed here by T2. T3 provides the real
 * implementation.
 *
 * @return Pointer to a stable internal string, or NULL.
 */
const char *go2rtc_process_get_override_path(void);

/**
 * @brief Generate <config_dir>/go2rtc.yaml from the currently loaded settings.
 *
 * This initializes the go2rtc process manager just long enough to write the
 * startup configuration file, then cleans up without starting go2rtc.
 *
 * @param binary_path Path to the go2rtc binary, or NULL to resolve automatically
 * @param config_dir Directory where go2rtc.yaml should be written
 * @param api_port Port for the go2rtc HTTP API
 * @return true if the startup config was generated successfully, false otherwise
 */
bool go2rtc_process_generate_startup_config(const char *binary_path,
                                            const char *config_dir,
                                            int api_port);

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

/**
 * @brief Probe a candidate go2rtc binary by running `<path> --version`.
 *
 * Spawns @p path with argument "--version" and reads up to 4 KB of its
 * standard output.  Returns 1 when the child exits 0 within 2 seconds AND
 * the collected stdout contains the substring "go2rtc version ".  Otherwise
 * returns 0.  The child is always waited for and its pipe FDs closed before
 * the call returns (no zombies).
 *
 * Exposed primarily for unit testing of the Docker binary-detection hardening
 * in T8.  Safe to call from production code as an opaque "is this really a
 * go2rtc binary?" oracle.
 *
 * @param path            Absolute or PATH-resolvable binary path.  Must be
 *                        executable by the current process.
 * @param version_out     Optional buffer to receive the matched version line
 *                        (first line of stdout that contained the signature).
 *                        May be NULL to discard.
 * @param version_out_sz  Size of @p version_out, including space for the NUL
 *                        terminator.  Ignored when @p version_out is NULL.
 * @return 1 on successful match, 0 on any failure (exec error, non-zero exit,
 *         timeout, or signature mismatch).
 */
int go2rtc_process_probe_version(const char *path,
                                 char *version_out,
                                 size_t version_out_sz);

#endif /* GO2RTC_PROCESS_H */
