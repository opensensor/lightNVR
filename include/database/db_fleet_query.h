#ifndef LIGHTNVR_DB_FLEET_QUERY_H
#define LIGHTNVR_DB_FLEET_QUERY_H

#include "core/camera_selector.h"

/* Load a slim, credential-free fleet inventory snapshot. The caller owns the
 * returned array and must free it. Zero cameras returns success with NULL. */
int db_fleet_camera_load(fleet_camera_t **cameras, int *count);

#endif /* LIGHTNVR_DB_FLEET_QUERY_H */
