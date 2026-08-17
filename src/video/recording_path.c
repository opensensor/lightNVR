#define _POSIX_C_SOURCE 200809L

#include "video/recording_path.h"

#include <stdio.h>
#include <string.h>

#include "core/path_utils.h"

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

    char safe_stream[MAX_STREAM_NAME];
    sanitize_stream_name(stream_name, safe_stream, sizeof(safe_stream));

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

    const char *format = config->mp4_directory_format;
    if (!mp4_directory_format_is_valid(format)) {
        format = MP4_DIRECTORY_FORMAT_YEAR_MONTH_DAY;
    }

    int written;
    if (strcmp(format, MP4_DIRECTORY_FORMAT_FLAT) == 0) {
        written = snprintf(output, output_size, "%s/%s", root, safe_stream);
    } else {
        struct tm tm_buf;
        if (!localtime_r(&timestamp, &tm_buf)) {
            return -1;
        }
        if (strcmp(format, MP4_DIRECTORY_FORMAT_YEAR_MONTH) == 0) {
            written = snprintf(output, output_size, "%s/%s/%04d/%02d",
                               root, safe_stream, tm_buf.tm_year + 1900,
                               tm_buf.tm_mon + 1);
        } else {
            written = snprintf(output, output_size, "%s/%s/%04d/%02d/%02d",
                               root, safe_stream, tm_buf.tm_year + 1900,
                               tm_buf.tm_mon + 1, tm_buf.tm_mday);
        }
    }

    return (written < 0 || (size_t)written >= output_size) ? -1 : 0;
}

int prepare_mp4_recording_path(const config_t *config,
                               const char *stream_name,
                               time_t timestamp,
                               char *output,
                               size_t output_size) {
    char directory[MAX_PATH_LENGTH];
    if (build_mp4_recording_directory(config, stream_name, timestamp,
                                      directory, sizeof(directory)) != 0) {
        return -1;
    }
    if (mkdir_recursive(directory) != 0) {
        return -1;
    }
    (void)chmod_path(directory, 0755);

    struct tm tm_buf;
    if (!localtime_r(&timestamp, &tm_buf)) {
        return -1;
    }
    char timestamp_text[32];
    if (strftime(timestamp_text, sizeof(timestamp_text),
                 "%Y%m%d_%H%M%S", &tm_buf) == 0) {
        return -1;
    }

    int written = snprintf(output, output_size, "%s/recording_%s.mp4",
                           directory, timestamp_text);
    return (written < 0 || (size_t)written >= output_size) ? -1 : 0;
}
