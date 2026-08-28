#ifndef LIGHTNVR_DB_OPERATOR_FLOOR_PLANS_H
#define LIGHTNVR_DB_OPERATOR_FLOOR_PLANS_H

#include <stdint.h>

#include "core/config.h"

#define OPERATOR_FLOOR_PLAN_NAME_MAX 128
#define OPERATOR_FLOOR_PLAN_MAX_VISIBLE 256
#define OPERATOR_FLOOR_PLAN_MAX_CAMERAS 1000

typedef struct {
    char uuid[CAMERA_UUID_STRING_SIZE];
    char name[OPERATOR_FLOOR_PLAN_NAME_MAX];
    char location_uuid[CAMERA_UUID_STRING_SIZE];
    char parent_plan_uuid[CAMERA_UUID_STRING_SIZE];
    int canvas_width;
    int canvas_height;
    int64_t revision;
    int64_t created_at;
    int64_t updated_at;
} operator_floor_plan_t;

typedef struct {
    char camera_uuid[CAMERA_UUID_STRING_SIZE];
    double x;
    double y;
    double rotation;
    double fov;
} operator_floor_plan_camera_t;

typedef enum {
    DB_OPERATOR_FLOOR_PLAN_OK = 0,
    DB_OPERATOR_FLOOR_PLAN_NOT_FOUND = -1,
    DB_OPERATOR_FLOOR_PLAN_INVALID = -2,
    DB_OPERATOR_FLOOR_PLAN_CONFLICT = -3,
    DB_OPERATOR_FLOOR_PLAN_STALE = -4,
    DB_OPERATOR_FLOOR_PLAN_LIMIT = -5,
    DB_OPERATOR_FLOOR_PLAN_ERROR = -6
} db_operator_floor_plan_result_t;

int db_operator_floor_plan_list(operator_floor_plan_t *plans, int max_count);
db_operator_floor_plan_result_t db_operator_floor_plan_get(
    const char *uuid, operator_floor_plan_t *plan);
int db_operator_floor_plan_camera_list(
    const char *plan_uuid, operator_floor_plan_camera_t *cameras,
    int max_count);
db_operator_floor_plan_result_t db_operator_floor_plan_create(
    operator_floor_plan_t *plan,
    const operator_floor_plan_camera_t *cameras, int camera_count);
db_operator_floor_plan_result_t db_operator_floor_plan_update(
    operator_floor_plan_t *plan,
    const operator_floor_plan_camera_t *cameras, int camera_count,
    int64_t expected_revision);
db_operator_floor_plan_result_t db_operator_floor_plan_delete(
    const char *uuid, int64_t expected_revision);

#endif /* LIGHTNVR_DB_OPERATOR_FLOOR_PLANS_H */
