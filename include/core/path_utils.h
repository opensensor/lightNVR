#ifndef LIGHTNVR_PATH_UTILS_H
#define LIGHTNVR_PATH_UTILS_H

#include <stdbool.h>
#include <stddef.h>
#include <sys/types.h>

/**
 * Sanitize a stream name for use as a path or object name.
 * Replaces non-alphanumeric characters with underscores and lowercases.
 *
 * @param input The input string
 * @param output The output buffer
 * @param output_size The size of the output buffer
 */
void sanitize_stream_name(const char *input, char *output, size_t output_size);

/**
 * Check whether a stream name is valid for use as an identifier.
 *
 * A stream name flows through filesystem paths, URLs, WebSocket query
 * parameters, and the go2rtc stream table. Allowing spaces or other
 * special characters leads to URL-encoding mismatches (e.g. "mse: stream
 * not found" when the client requests "Front%20Door%20Camera" but go2rtc
 * registered "Front Door Camera"), and also causes filesystem collisions
 * because sanitize_stream_name() collapses all non-[A-Za-z0-9-] characters
 * to underscores.
 *
 * Allowed characters: [A-Za-z0-9._-]. The name must be non-empty and may
 * not start with '.' (to avoid hidden-file paths).
 *
 * @param name NUL-terminated stream name, may be NULL
 * @return true if the name is valid, false otherwise
 */
bool is_valid_stream_name(const char *name);

/**
 * Ensure the specified directory exists, creating it if necessary. Does not recur.
 * Sets errno on failure.
 *
 * @param path Directory path to create
 * @return 0 on success, negative errno on error
 */
int ensure_dir(const char *path);

/**
 * Create a directory recursively (like mkdir -p)
 * This replaces system("mkdir -p ...") calls
 *
 * @param path Directory path to create
 * @return 0 on success, -1 on error
 */
int mkdir_recursive(const char *path);

/**
 * Ensure a directory exists for a file path by recursively creating
 * parent directories.
 *
 * @param path File path to create parent directories for
 * @return 0 on success, -1 on error
 */
int ensure_path(const char *path);

/**
 * Set permissions on a file or directory (like chmod)
 *
 * @param path Path to set permissions on
 * @param mode Permission mode (e.g., 0755)
 * @return 0 on success, -1 on error
 */
int chmod_path(const char *path, mode_t mode);

/**
 * Set permissions on a parent directory
 *
 * @param path Path to the file inside the parent directory
 * @param mode Permission mode (e.g., 0755)
 * @return 0 on success, -1 on error
 */
int chmod_parent(const char *path, mode_t mode);

/**
 * Recursively set permissions on a directory and its contents (like chmod -R)
 *
 * @param path Directory path to chmod recursively
 * @param mode Permission mode (e.g., 0755)
 * @return 0 on success, -1 on error
 */
int chmod_recursive(const char *path, mode_t mode);

#endif /* LIGHTNVR_PATH_UTILS_H */