#ifndef LIGHTNVR_RECORDING_PATH_H
#define LIGHTNVR_RECORDING_PATH_H

#include <stdbool.h>
#include <stddef.h>
#include <time.h>

#include "core/config.h"

#define MP4_DIRECTORY_FORMAT_FLAT "flat"
#define MP4_DIRECTORY_FORMAT_YEAR_MONTH "year_month"
#define MP4_DIRECTORY_FORMAT_YEAR_MONTH_DAY "year_month_day"

/** Return true when value is one of the supported safe directory presets. */
bool mp4_directory_format_is_valid(const char *value);

/**
 * Build the directory for a stream recording at timestamp.
 *
 * Examples:
 *   flat:           <root>/<stream>
 *   year_month:     <root>/<stream>/2026/08
 *   year_month_day: <root>/<stream>/2026/08/16
 */
int build_mp4_recording_directory(const config_t *config,
                                  const char *stream_name,
                                  time_t timestamp,
                                  char *output,
                                  size_t output_size);

/** Build a timestamped MP4 path and create its directory when needed. */
int prepare_mp4_recording_path(const config_t *config,
                               const char *stream_name,
                               time_t timestamp,
                               char *output,
                               size_t output_size);

#endif /* LIGHTNVR_RECORDING_PATH_H */
