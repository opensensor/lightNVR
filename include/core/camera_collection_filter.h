#ifndef LIGHTNVR_CAMERA_COLLECTION_FILTER_H
#define LIGHTNVR_CAMERA_COLLECTION_FILTER_H

#include <stdbool.h>

#include "core/camera_selector.h"
#include "database/db_auth.h"

typedef enum {
    CAMERA_COLLECTION_FILTER_OK = 0,
    CAMERA_COLLECTION_FILTER_NOT_FOUND = -1,
    CAMERA_COLLECTION_FILTER_INVALID_SELECTOR = -2,
    CAMERA_COLLECTION_FILTER_OUT_OF_MEMORY = -3,
    CAMERA_COLLECTION_FILTER_DATABASE_ERROR = -4
} camera_collection_filter_result_t;

typedef struct {
    bool active;
    fleet_selector_t *smart_selector;
    char (*member_uuids)[CAMERA_UUID_STRING_SIZE];
    int member_count;
} camera_collection_filter_t;

camera_collection_filter_result_t camera_collection_filter_load(
    const char *collection_uuid, const user_t *user,
    camera_collection_filter_t *filter);
void camera_collection_filter_free(camera_collection_filter_t *filter);
bool camera_collection_filter_matches(
    const camera_collection_filter_t *filter, const fleet_camera_t *camera);
camera_collection_filter_result_t camera_collection_filter_resolve_stream_names(
    const char *collection_uuid, const user_t *user, char ***stream_names,
    int *stream_count);
void camera_collection_filter_free_stream_names(char **stream_names,
                                                int stream_count);

#endif /* LIGHTNVR_CAMERA_COLLECTION_FILTER_H */
