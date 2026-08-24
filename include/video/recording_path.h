#ifndef LIGHTNVR_RECORDING_PATH_H
#define LIGHTNVR_RECORDING_PATH_H

#include <stdbool.h>
#include <stddef.h>
#include <time.h>

#include "core/config.h"
#include "storage/storage_placement.h"

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

/* Build beneath an explicit recording target root. */
int build_mp4_recording_directory_at_root(
    const config_t *config, const char *recording_root,
    const char *stream_name, time_t timestamp,
    char *output, size_t output_size);

/* Preserve the legacy default target's <storage>/mp4 layout. */
int build_mp4_recording_directory_for_placement(
    const config_t *config, const storage_placement_t *placement,
    const char *stream_name, time_t timestamp,
    char *output, size_t output_size);

/* Select a target, create its directory, and return auditable placement data. */
int prepare_placed_mp4_recording_path(
    const config_t *config, const char *stream_name, time_t timestamp,
    char *output, size_t output_size, storage_placement_t *placement);

#endif /* LIGHTNVR_RECORDING_PATH_H */
