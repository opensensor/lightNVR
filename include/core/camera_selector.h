#ifndef LIGHTNVR_CAMERA_SELECTOR_H
#define LIGHTNVR_CAMERA_SELECTOR_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <cjson/cJSON.h>

#include "core/config.h"

#define FLEET_SELECTOR_VERSION 1
#define FLEET_SELECTOR_MAX_DEPTH 8
#define FLEET_SELECTOR_MAX_NODES 64
#define FLEET_SELECTOR_MAX_VALUES 64
#define FLEET_CAMERA_MAX_TAGS 64
#define FLEET_CAMERA_MAX_LOCATION_DEPTH 32
#define FLEET_LOCATION_PATH_MAX 1024
#define FLEET_INVENTORY_VALUE_MAX 128
#define FLEET_SELECTOR_ERROR_MAX 256
#define FLEET_EXPLANATION_MAX_CLAUSES 64
#define FLEET_EXPLANATION_CLAUSE_MAX 160

typedef enum {
    FLEET_HEALTH_UNKNOWN = 0,
    FLEET_HEALTH_UP,
    FLEET_HEALTH_DEGRADED,
    FLEET_HEALTH_DOWN,
    FLEET_HEALTH_DISABLED
} fleet_health_state_t;

typedef enum {
    FLEET_AVAILABILITY_LIVE = 0,
    FLEET_AVAILABILITY_OFFLINE,
    FLEET_AVAILABILITY_NEVER_CONNECTED,
    FLEET_AVAILABILITY_DISABLED,
    FLEET_AVAILABILITY_COUNT
} fleet_availability_state_t;

typedef struct {
    char uuid[CAMERA_UUID_STRING_SIZE];
    char label[256];
} fleet_camera_tag_t;

/* Credential-free camera inventory record consumed by selectors and fleet APIs. */
typedef struct {
    char camera_uuid[CAMERA_UUID_STRING_SIZE];
    char name[MAX_STREAM_NAME];
    char address[MAX_URL_LENGTH];
    char legacy_tags[256];
    char location_uuid[CAMERA_UUID_STRING_SIZE];
    char location_name[128];
    char location_path[FLEET_LOCATION_PATH_MAX];
    char location_ancestor_uuids[FLEET_CAMERA_MAX_LOCATION_DEPTH]
                                [CAMERA_UUID_STRING_SIZE];
    int location_depth;
    fleet_camera_tag_t tags[FLEET_CAMERA_MAX_TAGS];
    int tag_count;
    char manufacturer[FLEET_INVENTORY_VALUE_MAX];
    char model[FLEET_INVENTORY_VALUE_MAX];
    bool enabled;
    bool streaming_enabled;
    bool record;
    bool detection_based_recording;
    bool is_onvif;
    bool ptz_enabled;
    bool backchannel_enabled;
    fleet_health_state_t health;
    fleet_availability_state_t availability;
    int64_t health_changed_at;
    int64_t first_video_at;
    int64_t last_frame_ts;
    int64_t last_recording_at;
    double current_fps;
    bool recording_active;
} fleet_camera_t;

typedef struct fleet_selector fleet_selector_t;

typedef struct {
    char clauses[FLEET_EXPLANATION_MAX_CLAUSES]
                [FLEET_EXPLANATION_CLAUSE_MAX];
    int clause_count;
} fleet_selector_explanation_t;

/*
 * Parse a selector of the form:
 * {"version":1,"expression":{"op":"and","children":[...]}}
 *
 * Leaf operations are all, camera_uuid, location_subtree, tag_any, tag_all,
 * tag_none, enabled, recording_mode, vendor, model, capability_any,
 * capability_all, and health. Unknown fields are ignored, but every operation's
 * required typed fields are validated. The returned selector is immutable.
 */
fleet_selector_t *fleet_selector_parse(const cJSON *json,
                                       char *error, size_t error_size);
void fleet_selector_free(fleet_selector_t *selector);

bool fleet_selector_matches(const fleet_selector_t *selector,
                            const fleet_camera_t *camera,
                            fleet_selector_explanation_t *explanation);

const char *fleet_health_state_name(fleet_health_state_t state);
const char *fleet_availability_state_name(fleet_availability_state_t state);
const char *fleet_camera_recording_mode(const fleet_camera_t *camera);

#endif /* LIGHTNVR_CAMERA_SELECTOR_H */
