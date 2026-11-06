/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

#ifndef LIGHTNVR_DB_FLEET_QUERY_H
#define LIGHTNVR_DB_FLEET_QUERY_H

#include "core/camera_selector.h"
#include <sqlite3.h>

/* Load a slim, credential-free fleet inventory snapshot. The caller owns the
 * returned array and must free it. Zero cameras returns success with NULL. */
int db_fleet_camera_load(fleet_camera_t **cameras, int *count);

/* Read the inventory on a private WAL snapshot without the writer mutex. */
int db_fleet_camera_load_readonly(fleet_camera_t **cameras, int *count);
/* Use the caller's already-established read snapshot. */
int db_fleet_camera_load_on_connection(sqlite3 *db,
    fleet_camera_t **cameras, int *count);

/* Load one credential-free fleet camera by its current stream name. Returns
 * 0 when found, 1 when no such stream exists, and -1 on a query error. */
int db_fleet_camera_find_by_name(const char *stream_name,
                                 fleet_camera_t *camera);
int db_fleet_camera_find_by_name_readonly(const char *stream_name,
                                         fleet_camera_t *camera);

/* Resolve immutable identity in one read snapshot, without a name roundtrip. */
int db_fleet_camera_find_by_uuid_readonly(const char *camera_uuid,
                                         fleet_camera_t *camera);

/* Add the current in-process health snapshot to a loaded inventory. Cameras
 * without an active metrics slot remain unknown; disabled cameras stay disabled. */
void fleet_camera_enrich_runtime_health(fleet_camera_t *cameras, int count);
/* Same observation, but report allocation/capture errors to fail-closed callers.
 * Uninitialized metrics are an available UNKNOWN observation, not an error. */
int fleet_camera_capture_runtime_health(fleet_camera_t *cameras, int count);

#endif /* LIGHTNVR_DB_FLEET_QUERY_H */
