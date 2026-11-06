/* SPDX-License-Identifier: GPL-3.0-or-later */
#ifndef LIGHTNVR_RECORDING_FILE_H
#define LIGHTNVR_RECORDING_FILE_H
#include <libavformat/avio.h>

/* Own a newly created regular file for the lifetime of the custom AVIO context.
 * Existing paths (including symlinks) are never opened or removed. Call only
 * recording_file_close on contexts returned by recording_file_open. */
int recording_file_open(AVIOContext **context, const char *path);
int recording_file_close(AVIOContext **context);
#endif
