#ifndef LIGHTNVR_DB_CAMERA_FAVORITES_H
#define LIGHTNVR_DB_CAMERA_FAVORITES_H

#include <stdbool.h>
#include <stdint.h>

#include "core/config.h"

/* Upper bound on favorites returned for one user. */
#define CAMERA_FAVORITES_MAX_LIST 1024

typedef struct {
    char camera_uuid[CAMERA_UUID_STRING_SIZE];
    int64_t created_at;
} camera_favorite_t;

typedef enum {
    DB_CAMERA_FAVORITE_OK = 0,
    DB_CAMERA_FAVORITE_NOT_FOUND = -1, /* camera_uuid does not reference a stream */
    DB_CAMERA_FAVORITE_INVALID = -2,
    DB_CAMERA_FAVORITE_ERROR = -3
} db_camera_favorite_result_t;

/*
 * Favorites are scoped by user_id. A user_id <= 0 addresses the single
 * installation-wide set used while authentication is disabled (stored with a
 * NULL user_id), the same convention live_saved_layouts uses for its owner.
 */

/** List a user's favorites, oldest first. Returns the count or -1. */
int db_camera_favorites_list(int64_t user_id, camera_favorite_t *favorites,
                             int max_count);

/**
 * Add a camera to a user's favorites. Idempotent: repeating the call keeps the
 * original created_at. On success favorite (optional) receives the stored row.
 */
db_camera_favorite_result_t db_camera_favorite_add(int64_t user_id,
                                                   const char *camera_uuid,
                                                   camera_favorite_t *favorite);

/**
 * Remove a camera from a user's favorites. Idempotent; removed (optional)
 * reports whether a row existed.
 */
db_camera_favorite_result_t db_camera_favorite_remove(int64_t user_id,
                                                      const char *camera_uuid,
                                                      bool *removed);

#endif /* LIGHTNVR_DB_CAMERA_FAVORITES_H */
