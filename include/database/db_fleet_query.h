#ifndef LIGHTNVR_DB_FLEET_QUERY_H
#define LIGHTNVR_DB_FLEET_QUERY_H

#include "core/camera_selector.h"

/* Load a slim, credential-free fleet inventory snapshot. The caller owns the
 * returned array and must free it. Zero cameras returns success with NULL. */
int db_fleet_camera_load(fleet_camera_t **cameras, int *count);

/* Load one credential-free fleet camera by its current stream name. Returns
 * 0 when found, 1 when no such stream exists, and -1 on a query error. */
int db_fleet_camera_find_by_name(const char *stream_name,
                                 fleet_camera_t *camera);

/* Add the current in-process health snapshot to a loaded inventory. Cameras
 * without an active metrics slot remain unknown; disabled cameras stay disabled. */
void fleet_camera_enrich_runtime_health(fleet_camera_t *cameras, int count);

#endif /* LIGHTNVR_DB_FLEET_QUERY_H */
