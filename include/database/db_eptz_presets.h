#ifndef LIGHTNVR_DB_EPTZ_PRESETS_H
#define LIGHTNVR_DB_EPTZ_PRESETS_H

#include <stdbool.h>
#include <stdint.h>

#include "core/config.h"

#define EPTZ_PRESET_NAME_MAX 128
#define EPTZ_PRESET_MAX_VISIBLE 128

typedef struct {
    char uuid[CAMERA_UUID_STRING_SIZE];
    char camera_uuid[CAMERA_UUID_STRING_SIZE];
    int64_t owner_user_id;
    char name[EPTZ_PRESET_NAME_MAX];
    bool is_shared;
    char mode[16];
    double yaw;
    double tilt;
    double view_fov;
    double secondary_yaw;
    double secondary_tilt;
    double secondary_view_fov;
    int64_t revision;
    int64_t created_at;
    int64_t updated_at;
} eptz_operator_preset_t;

typedef enum {
    DB_EPTZ_PRESET_OK = 0,
    DB_EPTZ_PRESET_NOT_FOUND = -1,
    DB_EPTZ_PRESET_INVALID = -2,
    DB_EPTZ_PRESET_CONFLICT = -3,
    DB_EPTZ_PRESET_STALE = -4,
    DB_EPTZ_PRESET_LIMIT = -5,
    DB_EPTZ_PRESET_ERROR = -6
} db_eptz_preset_result_t;

int db_eptz_preset_list_visible(const char *camera_uuid,
                                int64_t owner_user_id,
                                eptz_operator_preset_t *presets,
                                int max_count);
db_eptz_preset_result_t db_eptz_preset_get_visible(
    const char *camera_uuid, int64_t owner_user_id, const char *uuid,
    eptz_operator_preset_t *preset);
db_eptz_preset_result_t db_eptz_preset_create(
    eptz_operator_preset_t *preset);
db_eptz_preset_result_t db_eptz_preset_update(
    eptz_operator_preset_t *preset, int64_t expected_revision);
db_eptz_preset_result_t db_eptz_preset_delete(
    const char *camera_uuid, int64_t owner_user_id, const char *uuid,
    int64_t expected_revision);

#endif /* LIGHTNVR_DB_EPTZ_PRESETS_H */
