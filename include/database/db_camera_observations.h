#ifndef LIGHTNVR_DB_CAMERA_OBSERVATIONS_H
#define LIGHTNVR_DB_CAMERA_OBSERVATIONS_H

#include <stdint.h>

#include "core/config.h"

typedef struct {
    char camera_uuid[CAMERA_UUID_STRING_SIZE];
    int64_t first_video_at;
    int64_t last_video_at;
    int64_t last_recording_at;
    int64_t updated_at;
} camera_observation_t;

/* Persist non-zero timestamps for the camera currently owning stream_name.
 * Timestamps are monotonic in the database; renames retain camera history. */
int db_camera_observation_record(const char *stream_name,
                                 int64_t video_at,
                                 int64_t recording_at);
int db_camera_observation_get(const char *camera_uuid,
                              camera_observation_t *observation);

#endif /* LIGHTNVR_DB_CAMERA_OBSERVATIONS_H */
