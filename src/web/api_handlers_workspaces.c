#include "web/api_handlers_workspaces.h"

#include <cjson/cJSON.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include "database/db_workspace_preferences.h"
#include "web/httpd_utils.h"

static void set_json_response(http_response_t *response, cJSON *root) {
    char *body = cJSON_PrintUnformatted(root);
    if (!body) {
        http_response_set_json_error(response, 500,
                                     "Failed to serialize workspaces");
        return;
    }
    http_response_set_json(response, 200, body);
    free(body);
}

static cJSON *workspace_json(const char *key, const char *label,
                             const char *description, bool visible,
                             bool configurable) {
    cJSON *workspace = cJSON_CreateObject();
    if (!workspace) return NULL;
    cJSON_AddStringToObject(workspace, "key", key);
    cJSON_AddStringToObject(workspace, "label", label);
    cJSON_AddStringToObject(workspace, "description", description);
    cJSON_AddBoolToObject(workspace, "visible", visible);
    cJSON_AddBoolToObject(workspace, "configurable", configurable);
    cJSON_AddBoolToObject(workspace, "authorization_unchanged", true);
    return workspace;
}

static void respond(const user_t *user,
                    const workspace_preferences_t *preferences,
                    http_response_t *response) {
    cJSON *root = cJSON_CreateObject();
    cJSON *workspaces = cJSON_CreateArray();
    bool configurable = strcmp(user->authentication_method, "demo") != 0 &&
                        !user->authenticated_via_scoped_token;
    cJSON *live = workspace_json(
        WORKSPACE_KEY_LIVE_NAVIGATOR, "Enhanced Live Navigator",
        "Location tree, availability filters, saved layouts, and recent events.",
        preferences->live_navigator_visible, configurable);
    cJSON *investigation = workspace_json(
        WORKSPACE_KEY_INVESTIGATION, "Investigation",
        "Search, correlate, protect, and export recorded evidence.",
        preferences->investigation_visible, configurable);
    if (!root || !workspaces || !live || !investigation) {
        cJSON_Delete(root);
        cJSON_Delete(workspaces);
        cJSON_Delete(live);
        cJSON_Delete(investigation);
        http_response_set_json_error(response, 500,
                                     "Failed to create workspace response");
        return;
    }
    cJSON_AddItemToArray(workspaces, live);
    cJSON_AddItemToArray(workspaces, investigation);
    cJSON_AddItemToObject(root, "workspaces", workspaces);
    cJSON_AddBoolToObject(root, "configurable", configurable);
    cJSON_AddStringToObject(
        root, "scope", user->id > 0 ? "user" : "installation");
    set_json_response(response, root);
    cJSON_Delete(root);
}

void handle_get_ui_workspaces(const http_request_t *request,
                              http_response_t *response) {
    user_t user;
    if (!httpd_check_action_access(request, &user)) {
        http_response_set_json_error(response, 401, "Unauthorized");
        return;
    }
    workspace_preferences_t preferences;
    if (strcmp(user.authentication_method, "demo") == 0) {
        db_workspace_preferences_defaults(&preferences);
    } else if (db_workspace_preferences_get(user.id, &preferences) != 0) {
        http_response_set_json_error(response, 500,
                                     "Failed to load workspace preferences");
        return;
    }
    respond(&user, &preferences, response);
}

void handle_put_ui_workspaces(const http_request_t *request,
                              http_response_t *response) {
    user_t user;
    if (!httpd_check_action_access(request, &user)) {
        http_response_set_json_error(response, 401, "Unauthorized");
        return;
    }
    if (user.authenticated_via_scoped_token ||
        strcmp(user.authentication_method, "demo") == 0) {
        http_response_set_json_error(
            response, 403, "Workspace preferences require an interactive user");
        return;
    }

    cJSON *body = httpd_parse_json_body(request);
    const cJSON *workspaces = body
        ? cJSON_GetObjectItemCaseSensitive(body, "workspaces") : NULL;
    if (!cJSON_IsObject(workspaces)) {
        cJSON_Delete(body);
        http_response_set_json_error(
            response, 400, "workspaces must be an object");
        return;
    }

    workspace_preferences_t preferences;
    if (db_workspace_preferences_get(user.id, &preferences) != 0) {
        cJSON_Delete(body);
        http_response_set_json_error(response, 500,
                                     "Failed to load workspace preferences");
        return;
    }
    bool changed = false;
    const cJSON *item = NULL;
    cJSON_ArrayForEach(item, workspaces) {
        if (!item->string || !cJSON_IsBool(item)) {
            cJSON_Delete(body);
            http_response_set_json_error(
                response, 400, "Workspace values must be booleans");
            return;
        }
        if (strcmp(item->string, WORKSPACE_KEY_LIVE_NAVIGATOR) == 0) {
            preferences.live_navigator_visible = cJSON_IsTrue(item);
        } else if (strcmp(item->string, WORKSPACE_KEY_INVESTIGATION) == 0) {
            preferences.investigation_visible = cJSON_IsTrue(item);
        } else {
            cJSON_Delete(body);
            http_response_set_json_error(response, 400,
                                         "Unknown workspace key");
            return;
        }
        changed = true;
    }
    cJSON_Delete(body);
    if (!changed) {
        http_response_set_json_error(response, 400,
                                     "At least one workspace is required");
        return;
    }
    if (db_workspace_preferences_replace(user.id, &preferences) != 0) {
        http_response_set_json_error(response, 500,
                                     "Failed to save workspace preferences");
        return;
    }
    respond(&user, &preferences, response);
}
