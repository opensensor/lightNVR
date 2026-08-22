#ifndef LIGHTNVR_DB_LOCATIONS_H
#define LIGHTNVR_DB_LOCATIONS_H

#include <stdint.h>

#include "core/config.h"

#define LOCATION_NAME_MAX 128
#define LOCATION_TYPE_MAX 64
#define LOCATION_METADATA_MAX 512

typedef struct {
    char uuid[CAMERA_UUID_STRING_SIZE];
    char parent_uuid[CAMERA_UUID_STRING_SIZE];
    char name[LOCATION_NAME_MAX];
    char type[LOCATION_TYPE_MAX];
    char metadata_json[LOCATION_METADATA_MAX];
    int sort_order;
    int is_system;
    int direct_child_count;
    int direct_camera_count;
    int64_t created_at;
    int64_t updated_at;
} camera_location_t;

typedef enum {
    DB_LOCATION_OK = 0,
    DB_LOCATION_NOT_FOUND = -1,
    DB_LOCATION_CONFLICT = -2,
    DB_LOCATION_INVALID = -3,
    DB_LOCATION_ERROR = -4
} db_location_result_t;

db_location_result_t db_location_get_unassigned(camera_location_t *location);
db_location_result_t db_location_get(const char *uuid,
                                     camera_location_t *location);
int db_location_count(void);
int db_location_list(camera_location_t *locations, int max_count);
db_location_result_t db_location_create(camera_location_t *location);
db_location_result_t db_location_update(camera_location_t *location);
db_location_result_t db_location_delete(const char *uuid);
db_location_result_t db_location_assign_camera(const char *camera_uuid,
                                               const char *location_uuid);
db_location_result_t db_location_get_for_camera(const char *camera_uuid,
                                                camera_location_t *location);

#endif /* LIGHTNVR_DB_LOCATIONS_H */
