#include <cjson/cJSON.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>

#include "core/authorization.h"
#include "core/camera_collection_filter.h"
#include "database/db_camera_collections.h"
#include "database/db_fleet_query.h"

void camera_collection_filter_free(camera_collection_filter_t *filter) {
    if (!filter) return;
    fleet_selector_free(filter->smart_selector);
    free(filter->member_uuids);
    memset(filter, 0, sizeof(*filter));
}

static camera_collection_filter_result_t load_filter(
    const char *collection_uuid, const user_t *user, bool shared_only,
    camera_collection_filter_t *filter) {
    if (!filter || (!user && !shared_only)) {
        return CAMERA_COLLECTION_FILTER_DATABASE_ERROR;
    }
    memset(filter, 0, sizeof(*filter));
    if (!collection_uuid || collection_uuid[0] == '\0') {
        return CAMERA_COLLECTION_FILTER_OK;
    }

    camera_collection_t collection;
    db_camera_collection_result_t result =
        db_camera_collection_get(collection_uuid, &collection);
    bool can_manage = false;
    if (result == DB_CAMERA_COLLECTION_OK && user && !shared_only) {
        authorization_evaluation_t evaluation;
        can_manage =
            authorization_evaluate(user, AUTHZ_CAMERA_CONFIGURE, NULL,
                                   &evaluation) == 0 &&
            evaluation.decision == AUTHZ_DECISION_ALLOW;
    }
    bool visible = result == DB_CAMERA_COLLECTION_OK &&
        (shared_only ? collection.is_shared :
         (can_manage || collection.is_shared ||
          (collection.owner_user_id > 0 &&
           collection.owner_user_id == user->id)));
    if (!visible) return CAMERA_COLLECTION_FILTER_NOT_FOUND;

    filter->active = true;
    if (strcmp(collection.collection_type, "smart") == 0) {
        cJSON *selector_json = cJSON_Parse(collection.selector_json);
        char error[FLEET_SELECTOR_ERROR_MAX] = {0};
        filter->smart_selector = fleet_selector_parse(
            selector_json, error, sizeof(error));
        cJSON_Delete(selector_json);
        if (!filter->smart_selector) {
            camera_collection_filter_free(filter);
            return CAMERA_COLLECTION_FILTER_INVALID_SELECTOR;
        }
        return CAMERA_COLLECTION_FILTER_OK;
    }

    filter->member_count = collection.member_count;
    if (filter->member_count <= 0) return CAMERA_COLLECTION_FILTER_OK;
    filter->member_uuids = calloc((size_t)filter->member_count,
                                  sizeof(*filter->member_uuids));
    if (!filter->member_uuids) {
        camera_collection_filter_free(filter);
        return CAMERA_COLLECTION_FILTER_OUT_OF_MEMORY;
    }
    int count = db_camera_collection_list_members(
        collection.uuid, filter->member_uuids, filter->member_count);
    if (count < 0) {
        camera_collection_filter_free(filter);
        return CAMERA_COLLECTION_FILTER_DATABASE_ERROR;
    }
    filter->member_count = count;
    return CAMERA_COLLECTION_FILTER_OK;
}

camera_collection_filter_result_t camera_collection_filter_load(
    const char *collection_uuid, const user_t *user,
    camera_collection_filter_t *filter) {
    return load_filter(collection_uuid, user, false, filter);
}

camera_collection_filter_result_t
camera_collection_filter_load_for_authorization(
    const char *collection_uuid, camera_collection_filter_t *filter) {
    return load_filter(collection_uuid, NULL, true, filter);
}

bool camera_collection_filter_matches(
    const camera_collection_filter_t *filter, const fleet_camera_t *camera) {
    if (!filter || !filter->active) return true;
    if (!camera) return false;
    if (filter->smart_selector) {
        return fleet_selector_matches(filter->smart_selector, camera, NULL);
    }
    for (int i = 0; i < filter->member_count; i++) {
        if (strcasecmp(filter->member_uuids[i], camera->camera_uuid) == 0) {
            return true;
        }
    }
    return false;
}

static camera_collection_filter_result_t resolve_stream_names(
    const char *collection_uuid, const user_t *user,
    authorization_action_t action, bool shared_authorization_boundary,
    char ***stream_names, int *stream_count) {
    if (!collection_uuid || !collection_uuid[0] ||
        (!user && !shared_authorization_boundary) || !stream_names ||
        !stream_count) {
        return CAMERA_COLLECTION_FILTER_DATABASE_ERROR;
    }
    *stream_names = NULL;
    *stream_count = 0;

    camera_collection_filter_t filter;
    camera_collection_filter_result_t result =
        shared_authorization_boundary
            ? camera_collection_filter_load_for_authorization(collection_uuid,
                                                               &filter)
            : camera_collection_filter_load(collection_uuid, user, &filter);
    if (result != CAMERA_COLLECTION_FILTER_OK) return result;

    fleet_camera_t *cameras = NULL;
    int camera_count = 0;
    if (db_fleet_camera_load(&cameras, &camera_count) != 0) {
        camera_collection_filter_free(&filter);
        return CAMERA_COLLECTION_FILTER_DATABASE_ERROR;
    }
    char **names = camera_count > 0
        ? calloc((size_t)camera_count, sizeof(*names)) : NULL;
    if (camera_count > 0 && !names) {
        free(cameras);
        camera_collection_filter_free(&filter);
        return CAMERA_COLLECTION_FILTER_OUT_OF_MEMORY;
    }
    /* Public resolution is filtered by the action the caller will perform.
     * Internal resolution is only allowed for shared collections after the
     * caller has already proved an all-fleet authorization boundary. */
    if (!shared_authorization_boundary &&
        authorization_filter_cameras(user, action, cameras,
                                     &camera_count) != 0) {
        free(names);
        free(cameras);
        camera_collection_filter_free(&filter);
        return CAMERA_COLLECTION_FILTER_DATABASE_ERROR;
    }
    for (int i = 0; i < camera_count; i++) {
        if (!camera_collection_filter_matches(&filter, &cameras[i])) continue;
        size_t name_size = strlen(cameras[i].name) + 1;
        names[*stream_count] = malloc(name_size);
        if (!names[*stream_count]) {
            camera_collection_filter_free_stream_names(names, *stream_count);
            free(cameras);
            camera_collection_filter_free(&filter);
            return CAMERA_COLLECTION_FILTER_OUT_OF_MEMORY;
        }
        memcpy(names[*stream_count], cameras[i].name, name_size);
        (*stream_count)++;
    }
    free(cameras);
    camera_collection_filter_free(&filter);
    *stream_names = names;
    return CAMERA_COLLECTION_FILTER_OK;
}

camera_collection_filter_result_t camera_collection_filter_resolve_stream_names(
    const char *collection_uuid, const user_t *user, char ***stream_names,
    int *stream_count) {
    return resolve_stream_names(collection_uuid, user, AUTHZ_LIVE_VIEW, false,
                                stream_names, stream_count);
}

camera_collection_filter_result_t
camera_collection_filter_resolve_stream_names_for_action(
    const char *collection_uuid, const user_t *user,
    authorization_action_t action, char ***stream_names, int *stream_count) {
    return resolve_stream_names(collection_uuid, user, action, false,
                                stream_names, stream_count);
}

camera_collection_filter_result_t
camera_collection_filter_resolve_stream_names_for_authorization(
    const char *collection_uuid, char ***stream_names, int *stream_count) {
    return resolve_stream_names(collection_uuid, NULL, AUTHZ_ACTION_INVALID,
                                true, stream_names, stream_count);
}

void camera_collection_filter_free_stream_names(char **stream_names,
                                                int stream_count) {
    if (!stream_names) return;
    for (int i = 0; i < stream_count; i++) free(stream_names[i]);
    free(stream_names);
}
