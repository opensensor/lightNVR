/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

#define _POSIX_C_SOURCE 200809L

#include "video/recording_path.h"

#include <errno.h>
#include <fcntl.h>
#include <sys/stat.h>
#include <unistd.h>
#include <stdio.h>
#include <string.h>

#include "core/path_utils.h"
#include "utils/strings.h"

bool mp4_directory_format_is_valid(const char *value) {
    return value &&
           (strcmp(value, MP4_DIRECTORY_FORMAT_FLAT) == 0 ||
            strcmp(value, MP4_DIRECTORY_FORMAT_YEAR_MONTH) == 0 ||
            strcmp(value, MP4_DIRECTORY_FORMAT_YEAR_MONTH_DAY) == 0);
}

int build_mp4_recording_directory(const config_t *config,
                                  const char *stream_name,
                                  time_t timestamp,
                                  char *output,
                                  size_t output_size) {
    if (!config || !stream_name || stream_name[0] == '\0' ||
        !output || output_size == 0) {
        return -1;
    }

    const char *root = config->storage_path;
    char default_root[MAX_PATH_LENGTH];
    if (config->record_mp4_directly && config->mp4_storage_path[0] != '\0') {
        root = config->mp4_storage_path;
    } else {
        int root_len = snprintf(default_root, sizeof(default_root), "%s/mp4",
                                config->storage_path);
        if (root_len < 0 || (size_t)root_len >= sizeof(default_root)) {
            return -1;
        }
        root = default_root;
    }

    return build_mp4_recording_directory_at_root(
        config, root, stream_name, timestamp, output, output_size);
}

int build_mp4_recording_directory_at_root(
    const config_t *config, const char *recording_root,
    const char *stream_name, time_t timestamp,
    char *output, size_t output_size) {
    if (!config || !recording_root || recording_root[0] != '/' ||
        !stream_name || stream_name[0] == '\0' || !output ||
        output_size == 0) return -1;
    char safe_stream[MAX_STREAM_NAME];
    sanitize_stream_name(stream_name, safe_stream, sizeof(safe_stream));
    const char *format = config->mp4_directory_format;
    if (!mp4_directory_format_is_valid(format)) {
        format = MP4_DIRECTORY_FORMAT_YEAR_MONTH_DAY;
    }

    int written;
    if (strcmp(format, MP4_DIRECTORY_FORMAT_FLAT) == 0) {
        written = snprintf(output, output_size, "%s/%s", recording_root,
                           safe_stream);
    } else {
        struct tm tm_buf;
        if (!localtime_r(&timestamp, &tm_buf)) {
            return -1;
        }
        if (strcmp(format, MP4_DIRECTORY_FORMAT_YEAR_MONTH) == 0) {
            written = snprintf(output, output_size, "%s/%s/%04d/%02d",
                               recording_root, safe_stream,
                               tm_buf.tm_year + 1900,
                               tm_buf.tm_mon + 1);
        } else {
            written = snprintf(output, output_size, "%s/%s/%04d/%02d/%02d",
                               recording_root, safe_stream,
                               tm_buf.tm_year + 1900,
                               tm_buf.tm_mon + 1, tm_buf.tm_mday);
        }
    }

    return (written < 0 || (size_t)written >= output_size) ? -1 : 0;
}

int build_recording_transcode_cache_path(const char *storage_path,
                                         uint64_t recording_id,
                                         char *output,
                                         size_t output_size) {
    if (!storage_path || storage_path[0] != '/' || recording_id == 0 ||
        !output || output_size == 0) {
        return -1;
    }

    int written = snprintf(output, output_size, "%s/transcoded/%llu.mp4",
                           storage_path, (unsigned long long)recording_id);
    return (written < 0 || (size_t)written >= output_size) ? -1 : 0;
}

static int effective_placement_root(
    const config_t *config, const storage_placement_t *placement,
    char root[MAX_PATH_LENGTH]) {
    if (!config || !placement || placement->target_root[0] != '/') return -1;
    if (placement->target_is_default && !config->record_mp4_directly) {
        int written = snprintf(root, MAX_PATH_LENGTH, "%s/mp4",
                               placement->target_root);
        return written < 0 || written >= MAX_PATH_LENGTH ? -1 : 0;
    }
    safe_strcpy(root, placement->target_root, MAX_PATH_LENGTH, 0);
    return 0;
}

int build_mp4_recording_directory_for_placement(
    const config_t *config, const storage_placement_t *placement,
    const char *stream_name, time_t timestamp,
    char *output, size_t output_size) {
    char root[MAX_PATH_LENGTH];
    if (effective_placement_root(config, placement, root) != 0) return -1;
    return build_mp4_recording_directory_at_root(
        config, root, stream_name, timestamp, output, output_size);
}

static int prepare_at_root(const config_t *config, const char *root,
                           const char *stream_name, time_t timestamp,
                           char *output, size_t output_size) {
    char directory[MAX_PATH_LENGTH];
    if (build_mp4_recording_directory_at_root(
            config, root, stream_name, timestamp, directory,
            sizeof(directory)) != 0) return -1;
    if (mkdir_recursive(directory) != 0) return -1;
    (void)chmod_path(directory, 0755);

    struct tm tm_buf;
    if (!localtime_r(&timestamp, &tm_buf)) return -1;
    char timestamp_text[32];
    if (strftime(timestamp_text, sizeof(timestamp_text),
                 "%Y%m%d_%H%M%S", &tm_buf) == 0) return -1;
    int written = snprintf(output, output_size, "%s/recording_%s.mp4",
                           directory, timestamp_text);
    return (written < 0 || (size_t)written >= output_size) ? -1 : 0;
}

int prepare_mp4_recording_path(const config_t *config,
                               const char *stream_name,
                               time_t timestamp,
                               char *output,
                               size_t output_size) {
    if (!config) return -1;
    const char *root = config->storage_path;
    char default_root[MAX_PATH_LENGTH];
    if (config->record_mp4_directly && config->mp4_storage_path[0]) {
        root = config->mp4_storage_path;
    } else {
        int written = snprintf(default_root, sizeof(default_root), "%s/mp4",
                               config->storage_path);
        if (written < 0 || (size_t)written >= sizeof(default_root)) return -1;
        root = default_root;
    }
    return prepare_at_root(config, root, stream_name, timestamp, output,
                           output_size);
}

int prepare_placed_mp4_recording_path(
    const config_t *config, const char *stream_name, time_t timestamp,
    char *output, size_t output_size, storage_placement_t *placement) {
    if (!placement) return -1;
    /*
     * Callers log placement->reason on failure, and storage_placement_select()
     * rejects a blank stream name before it clears the struct, so seed the
     * whole thing here rather than letting them read an uninitialised stack
     * buffer.
     */
    memset(placement, 0, sizeof(*placement));
    placement->status = STORAGE_PLACEMENT_ERROR;
    safe_strcpy(placement->reason, "invalid-request",
                sizeof(placement->reason), 0);
    if (!config || storage_placement_select(stream_name, placement) != 0 ||
        placement->status != STORAGE_PLACEMENT_READY) return -1;
    char root[MAX_PATH_LENGTH];
    if (effective_placement_root(config, placement, root) != 0 ||
        prepare_at_root(config, root, stream_name,
                        timestamp, output, output_size) != 0) return -1;
    size_t root_length = strlen(placement->target_root);
    if (strncmp(output, placement->target_root, root_length) != 0 ||
        output[root_length] != '/') return -1;
    safe_strcpy(placement->object_key, output + root_length + 1,
                sizeof(placement->object_key), 0);
    return placement->object_key[0] ? 0 : -1;
}

/* Planning a timestamped name and claiming it are separate operations. A real
 * O_EXCL reservation closes the same-second/restart race without changing the
 * historical directory or timestamp format for the ordinary first segment. */
int reserve_mp4_recording_path(char *path, size_t path_size,
                               storage_placement_t *placement) {
    if (!path || !path_size || path[0] != '/') { errno = EINVAL; return -1; }
    size_t length = strnlen(path, path_size);
    if (length == path_size || length < 4 || strcmp(path + length - 4, ".mp4")) {
        errno = EINVAL;
        return -1;
    }
    if (length >= MAX_PATH_LENGTH) { errno = ENAMETOOLONG; return -1; }
    size_t root_length = 0;
    if (placement) {
        root_length = strlen(placement->target_root);
        if (!root_length || strncmp(path, placement->target_root, root_length) ||
            path[root_length] != '/') { errno = EINVAL; return -1; }
    }
    char candidate[MAX_PATH_LENGTH];
    for (unsigned attempt = 0; attempt < 1000; attempt++) {
        int size = attempt == 0 ? snprintf(candidate, sizeof(candidate), "%s", path)
            : snprintf(candidate, sizeof(candidate), "%.*s_%03u.mp4", (int)(length - 4), path, attempt);
        if (size < 0 || (size_t)size >= sizeof(candidate) || (size_t)size >= path_size ||
            (placement && (size_t)size - root_length >= sizeof(placement->object_key))) {
            errno = ENAMETOOLONG;
            return -1;
        }
        int fd = open(candidate, O_RDWR | O_CREAT | O_EXCL | O_CLOEXEC | O_NOFOLLOW, 0644);
        if (fd < 0) {
            if (errno == EEXIST) continue;
            return -1;
        }
        /* All lengths and placement constraints were validated before create;
         * no allocation/copy operation after reservation can fail. */
        memcpy(path, candidate, (size_t)size + 1);
        if (placement) strcpy(placement->object_key, candidate + root_length + 1);
        return fd;
    }
    errno = EEXIST;
    return -1;
}

int release_mp4_recording_path(int descriptor, const char *path) {
    if (descriptor < 0) { errno = EINVAL; return -1; }
    struct stat held, current;
    int result = 0, saved_errno = 0;
    if (fstat(descriptor, &held) != 0) {
        result = -1; saved_errno = errno;
    } else if (S_ISREG(held.st_mode) && held.st_size == 0 && path) {
        if (lstat(path, &current) == 0) {
            if (S_ISREG(current.st_mode) && current.st_dev == held.st_dev &&
                current.st_ino == held.st_ino && current.st_size == 0 && unlink(path) != 0) {
                result = -1; saved_errno = errno;
            }
        } else if (errno != ENOENT) {
            result = -1; saved_errno = errno;
        }
    }
    if (close(descriptor) != 0 && result == 0) { result = -1; saved_errno = errno; }
    if (result != 0) errno = saved_errno;
    return result;
}
