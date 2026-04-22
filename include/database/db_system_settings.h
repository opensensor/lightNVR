#ifndef DB_SYSTEM_SETTINGS_H
#define DB_SYSTEM_SETTINGS_H

#include <stdbool.h>
#include <stddef.h>

/**
 * @file db_system_settings.h
 * @brief Key/value helper for the system_settings table.
 *
 * The table is created by migration 0030 and holds singleton values such as
 * setup_complete, setup_completed_at, etc.
 */

/**
 * @brief Read a value from system_settings.
 *
 * @param key        Setting key (e.g. "setup_complete")
 * @param out        Buffer to write the value into
 * @param out_len    Size of the buffer
 * @return 0 on success, -1 if the key does not exist or on DB error
 */
int db_get_system_setting(const char *key, char *out, int out_len);

/**
 * @brief Read a value from system_settings into a newly malloc'd buffer.
 *
 * Unlike db_get_system_setting, this variant is safe for values larger than
 * a reasonable stack buffer (e.g. the go2rtc_config_override YAML which can
 * reach ~64 KB). On musl/Alpine or libuv worker threads the default 128 KB
 * pthread stack makes VLAs and large stack arrays risky, so callers that
 * need to tolerate large payloads should prefer this helper.
 *
 * @param key        Setting key
 * @param out        [out] On success, set to a newly malloc'd, NUL-terminated
 *                   buffer owned by the caller (must be freed with free()).
 *                   On "not found" or error, set to NULL.
 * @param out_len    [out] On success, set to strlen(*out) (excluding NUL).
 *                   On "not found" or error, set to 0.
 * @return 0 when the key was found, 1 when the key does not exist, -1 on error.
 */
int db_get_system_setting_alloc(const char *key, char **out, size_t *out_len);

/**
 * @brief Write (INSERT OR REPLACE) a value into system_settings.
 *
 * @param key    Setting key
 * @param value  Value to store
 * @return 0 on success, -1 on error
 */
int db_set_system_setting(const char *key, const char *value);

/**
 * @brief Convenience: return true when setup_complete == "1".
 */
bool db_is_setup_complete(void);

/**
 * @brief Convenience: mark setup as complete (sets setup_complete = "1"
 *        and records setup_completed_at as the current epoch second).
 * @return 0 on success, -1 on error
 */
int db_mark_setup_complete(void);

#endif /* DB_SYSTEM_SETTINGS_H */

