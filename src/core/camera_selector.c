#define _POSIX_C_SOURCE 200809L

#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>

#include "core/camera_selector.h"

typedef enum {
    SELECTOR_ALL = 0,
    SELECTOR_AND,
    SELECTOR_OR,
    SELECTOR_NOT,
    SELECTOR_CAMERA_UUID,
    SELECTOR_LOCATION_SUBTREE,
    SELECTOR_TAG_ANY,
    SELECTOR_TAG_ALL,
    SELECTOR_TAG_NONE,
    SELECTOR_ENABLED,
    SELECTOR_RECORDING_MODE,
    SELECTOR_VENDOR,
    SELECTOR_MODEL,
    SELECTOR_CAPABILITY_ANY,
    SELECTOR_CAPABILITY_ALL,
    SELECTOR_HEALTH
} selector_node_type_t;

typedef struct selector_node {
    selector_node_type_t type;
    struct selector_node **children;
    int child_count;
    char **values;
    int value_count;
    bool bool_value;
} selector_node_t;

struct fleet_selector {
    int version;
    selector_node_t *root;
};

typedef bool (*value_validator_t)(const char *value);

static void set_error(char *error, size_t error_size, const char *format, ...) {
    if (!error || error_size == 0 || error[0] != '\0') return;
    va_list args;
    va_start(args, format);
    vsnprintf(error, error_size, format, args);
    va_end(args);
}

static bool valid_uuid(const char *value) {
    if (!value || strlen(value) != CAMERA_UUID_STRING_SIZE - 1) return false;
    for (int i = 0; i < CAMERA_UUID_STRING_SIZE - 1; i++) {
        const char c = value[i];
        if (i == 8 || i == 13 || i == 18 || i == 23) {
            if (c != '-') return false;
        } else if (!((c >= '0' && c <= '9') ||
                     (c >= 'a' && c <= 'f') ||
                     (c >= 'A' && c <= 'F'))) {
            return false;
        }
    }
    return true;
}

static bool valid_nonempty(const char *value) {
    return value && value[0] != '\0' && strlen(value) < MAX_STREAM_NAME;
}

static bool valid_recording_mode(const char *value) {
    return value && (strcmp(value, "off") == 0 ||
                     strcmp(value, "continuous") == 0 ||
                     strcmp(value, "detection") == 0);
}

static bool valid_capability(const char *value) {
    return value && (strcmp(value, "onvif") == 0 ||
                     strcmp(value, "ptz") == 0 ||
                     strcmp(value, "backchannel") == 0);
}

static bool valid_health(const char *value) {
    return value && (strcmp(value, "unknown") == 0 ||
                     strcmp(value, "up") == 0 ||
                     strcmp(value, "degraded") == 0 ||
                     strcmp(value, "down") == 0 ||
                     strcmp(value, "disabled") == 0);
}

static void selector_node_free(selector_node_t *node) {
    if (!node) return;
    for (int i = 0; i < node->child_count; i++) {
        selector_node_free(node->children[i]);
    }
    for (int i = 0; i < node->value_count; i++) free(node->values[i]);
    free(node->children);
    free(node->values);
    free(node);
}

static bool parse_values(const cJSON *json, const char *field,
                         value_validator_t validator, selector_node_t *node,
                         char *error, size_t error_size) {
    const cJSON *array = cJSON_GetObjectItemCaseSensitive(json, field);
    int count = cJSON_IsArray(array) ? cJSON_GetArraySize(array) : -1;
    if (count < 1 || count > FLEET_SELECTOR_MAX_VALUES) {
        set_error(error, error_size, "%s must contain 1-%d values", field,
                  FLEET_SELECTOR_MAX_VALUES);
        return false;
    }
    node->values = calloc((size_t)count, sizeof(*node->values));
    if (!node->values) {
        set_error(error, error_size, "Out of memory parsing selector");
        return false;
    }
    for (int i = 0; i < count; i++) {
        const cJSON *item = cJSON_GetArrayItem(array, i);
        if (!cJSON_IsString(item) || !item->valuestring ||
            strlen(item->valuestring) >= MAX_STREAM_NAME ||
            (validator && !validator(item->valuestring))) {
            set_error(error, error_size, "%s contains an invalid value", field);
            return false;
        }
        node->values[i] = strdup(item->valuestring);
        if (!node->values[i]) {
            set_error(error, error_size, "Out of memory parsing selector");
            return false;
        }
        node->value_count++;
    }
    return true;
}

static selector_node_t *parse_node(const cJSON *json, int depth,
                                   int *node_count, char *error,
                                   size_t error_size) {
    if (!cJSON_IsObject(json)) {
        set_error(error, error_size, "Selector expression must be an object");
        return NULL;
    }
    if (depth > FLEET_SELECTOR_MAX_DEPTH) {
        set_error(error, error_size, "Selector exceeds maximum depth %d",
                  FLEET_SELECTOR_MAX_DEPTH);
        return NULL;
    }
    if (++(*node_count) > FLEET_SELECTOR_MAX_NODES) {
        set_error(error, error_size, "Selector exceeds maximum node count %d",
                  FLEET_SELECTOR_MAX_NODES);
        return NULL;
    }
    const cJSON *op_item = cJSON_GetObjectItemCaseSensitive(json, "op");
    if (!cJSON_IsString(op_item) || !op_item->valuestring) {
        set_error(error, error_size, "Selector expression requires string op");
        return NULL;
    }

    const char *op = op_item->valuestring;
    selector_node_t *node = calloc(1, sizeof(*node));
    if (!node) {
        set_error(error, error_size, "Out of memory parsing selector");
        return NULL;
    }

    if (strcmp(op, "all") == 0) {
        node->type = SELECTOR_ALL;
        return node;
    }
    if (strcmp(op, "and") == 0 || strcmp(op, "or") == 0) {
        const cJSON *children =
            cJSON_GetObjectItemCaseSensitive(json, "children");
        int count = cJSON_IsArray(children) ? cJSON_GetArraySize(children) : -1;
        if (count < 1 || count > FLEET_SELECTOR_MAX_VALUES) {
            set_error(error, error_size, "%s requires 1-%d children", op,
                      FLEET_SELECTOR_MAX_VALUES);
            selector_node_free(node);
            return NULL;
        }
        node->type = strcmp(op, "and") == 0 ? SELECTOR_AND : SELECTOR_OR;
        node->children = calloc((size_t)count, sizeof(*node->children));
        if (!node->children) {
            set_error(error, error_size, "Out of memory parsing selector");
            selector_node_free(node);
            return NULL;
        }
        for (int i = 0; i < count; i++) {
            node->children[i] = parse_node(cJSON_GetArrayItem(children, i),
                                           depth + 1, node_count, error,
                                           error_size);
            if (!node->children[i]) {
                selector_node_free(node);
                return NULL;
            }
            node->child_count++;
        }
        return node;
    }
    if (strcmp(op, "not") == 0) {
        const cJSON *child = cJSON_GetObjectItemCaseSensitive(json, "child");
        node->type = SELECTOR_NOT;
        node->children = calloc(1, sizeof(*node->children));
        if (!node->children) {
            set_error(error, error_size, "Out of memory parsing selector");
            selector_node_free(node);
            return NULL;
        }
        node->children[0] = parse_node(child, depth + 1, node_count, error,
                                       error_size);
        if (!node->children[0]) {
            selector_node_free(node);
            return NULL;
        }
        node->child_count = 1;
        return node;
    }

    bool parsed = false;
    if (strcmp(op, "camera_uuid") == 0) {
        node->type = SELECTOR_CAMERA_UUID;
        parsed = parse_values(json, "values", valid_uuid, node, error, error_size);
    } else if (strcmp(op, "location_subtree") == 0) {
        const cJSON *uuid = cJSON_GetObjectItemCaseSensitive(json, "uuid");
        node->type = SELECTOR_LOCATION_SUBTREE;
        if (!cJSON_IsString(uuid) || !valid_uuid(uuid->valuestring)) {
            set_error(error, error_size, "location_subtree requires a valid uuid");
        } else {
            node->values = calloc(1, sizeof(*node->values));
            if (node->values) node->values[0] = strdup(uuid->valuestring);
            if (node->values && node->values[0]) {
                node->value_count = 1;
                parsed = true;
            } else {
                set_error(error, error_size, "Out of memory parsing selector");
            }
        }
    } else if (strcmp(op, "tag_any") == 0 ||
               strcmp(op, "tag_all") == 0 || strcmp(op, "tag_none") == 0) {
        node->type = strcmp(op, "tag_any") == 0 ? SELECTOR_TAG_ANY :
                     strcmp(op, "tag_all") == 0 ? SELECTOR_TAG_ALL :
                                                   SELECTOR_TAG_NONE;
        parsed = parse_values(json, "uuids", valid_uuid, node, error, error_size);
    } else if (strcmp(op, "enabled") == 0) {
        const cJSON *value = cJSON_GetObjectItemCaseSensitive(json, "value");
        node->type = SELECTOR_ENABLED;
        if (!cJSON_IsBool(value)) {
            set_error(error, error_size, "enabled requires boolean value");
        } else {
            node->bool_value = cJSON_IsTrue(value);
            parsed = true;
        }
    } else if (strcmp(op, "recording_mode") == 0) {
        node->type = SELECTOR_RECORDING_MODE;
        parsed = parse_values(json, "values", valid_recording_mode, node,
                              error, error_size);
    } else if (strcmp(op, "vendor") == 0 || strcmp(op, "model") == 0) {
        node->type = strcmp(op, "vendor") == 0 ? SELECTOR_VENDOR : SELECTOR_MODEL;
        parsed = parse_values(json, "values", valid_nonempty, node, error,
                              error_size);
    } else if (strcmp(op, "capability_any") == 0 ||
               strcmp(op, "capability_all") == 0) {
        node->type = strcmp(op, "capability_any") == 0 ?
                     SELECTOR_CAPABILITY_ANY : SELECTOR_CAPABILITY_ALL;
        parsed = parse_values(json, "values", valid_capability, node, error,
                              error_size);
    } else if (strcmp(op, "health") == 0) {
        node->type = SELECTOR_HEALTH;
        parsed = parse_values(json, "values", valid_health, node, error,
                              error_size);
    } else {
        set_error(error, error_size, "Unknown selector op: %s", op);
    }

    if (!parsed) {
        selector_node_free(node);
        return NULL;
    }
    return node;
}

fleet_selector_t *fleet_selector_parse(const cJSON *json,
                                       char *error, size_t error_size) {
    if (error && error_size > 0) error[0] = '\0';
    if (!cJSON_IsObject(json)) {
        set_error(error, error_size, "selector must be an object");
        return NULL;
    }
    const cJSON *version = cJSON_GetObjectItemCaseSensitive(json, "version");
    const cJSON *expression =
        cJSON_GetObjectItemCaseSensitive(json, "expression");
    if (!cJSON_IsNumber(version) || version->valuedouble != FLEET_SELECTOR_VERSION) {
        set_error(error, error_size, "selector.version must be %d",
                  FLEET_SELECTOR_VERSION);
        return NULL;
    }
    fleet_selector_t *selector = calloc(1, sizeof(*selector));
    if (!selector) {
        set_error(error, error_size, "Out of memory parsing selector");
        return NULL;
    }
    selector->version = FLEET_SELECTOR_VERSION;
    int node_count = 0;
    selector->root = parse_node(expression, 1, &node_count, error, error_size);
    if (!selector->root) {
        free(selector);
        return NULL;
    }
    return selector;
}

void fleet_selector_free(fleet_selector_t *selector) {
    if (!selector) return;
    selector_node_free(selector->root);
    free(selector);
}

const char *fleet_health_state_name(fleet_health_state_t state) {
    switch (state) {
        case FLEET_HEALTH_UP: return "up";
        case FLEET_HEALTH_DEGRADED: return "degraded";
        case FLEET_HEALTH_DOWN: return "down";
        case FLEET_HEALTH_DISABLED: return "disabled";
        default: return "unknown";
    }
}

const char *fleet_availability_state_name(fleet_availability_state_t state) {
    switch (state) {
        case FLEET_AVAILABILITY_LIVE: return "live";
        case FLEET_AVAILABILITY_OFFLINE: return "offline";
        case FLEET_AVAILABILITY_NEVER_CONNECTED: return "never_connected";
        case FLEET_AVAILABILITY_DISABLED: return "disabled";
        default: return "never_connected";
    }
}

const char *fleet_camera_recording_mode(const fleet_camera_t *camera) {
    if (!camera || !camera->record) return "off";
    return camera->detection_based_recording ? "detection" : "continuous";
}

static void explain(fleet_selector_explanation_t *explanation,
                    const char *format, ...) {
    if (!explanation ||
        explanation->clause_count >= FLEET_EXPLANATION_MAX_CLAUSES) return;
    va_list args;
    va_start(args, format);
    vsnprintf(explanation->clauses[explanation->clause_count],
              FLEET_EXPLANATION_CLAUSE_MAX, format, args);
    va_end(args);
    explanation->clause_count++;
}

static bool string_in_values(const char *value, const selector_node_t *node) {
    for (int i = 0; i < node->value_count; i++) {
        if (strcasecmp(value ? value : "", node->values[i]) == 0) return true;
    }
    return false;
}

static bool camera_has_tag(const fleet_camera_t *camera, const char *uuid) {
    for (int i = 0; i < camera->tag_count; i++) {
        if (strcasecmp(camera->tags[i].uuid, uuid) == 0) return true;
    }
    return false;
}

static bool camera_has_capability(const fleet_camera_t *camera,
                                  const char *capability) {
    if (strcmp(capability, "onvif") == 0) return camera->is_onvif;
    if (strcmp(capability, "ptz") == 0) return camera->ptz_enabled;
    if (strcmp(capability, "backchannel") == 0) {
        return camera->backchannel_enabled;
    }
    return false;
}

static bool evaluate_node(const selector_node_t *node,
                          const fleet_camera_t *camera,
                          fleet_selector_explanation_t *explanation) {
    int original_clause_count = explanation ? explanation->clause_count : 0;
    switch (node->type) {
        case SELECTOR_ALL:
            explain(explanation, "all cameras");
            return true;
        case SELECTOR_AND:
            for (int i = 0; i < node->child_count; i++) {
                if (!evaluate_node(node->children[i], camera, explanation)) {
                    if (explanation) explanation->clause_count = original_clause_count;
                    return false;
                }
            }
            return true;
        case SELECTOR_OR:
            for (int i = 0; i < node->child_count; i++) {
                if (explanation) explanation->clause_count = original_clause_count;
                if (evaluate_node(node->children[i], camera, explanation)) return true;
            }
            if (explanation) explanation->clause_count = original_clause_count;
            return false;
        case SELECTOR_NOT: {
            bool child_match = evaluate_node(node->children[0], camera, explanation);
            if (explanation) explanation->clause_count = original_clause_count;
            if (!child_match) explain(explanation, "not predicate");
            return !child_match;
        }
        case SELECTOR_CAMERA_UUID:
            if (string_in_values(camera->camera_uuid, node)) {
                explain(explanation, "camera_uuid=%s", camera->camera_uuid);
                return true;
            }
            return false;
        case SELECTOR_LOCATION_SUBTREE:
            for (int i = 0; i < camera->location_depth; i++) {
                if (strcasecmp(camera->location_ancestor_uuids[i],
                               node->values[0]) == 0) {
                    explain(explanation, "location_subtree=%s", node->values[0]);
                    return true;
                }
            }
            return false;
        case SELECTOR_TAG_ANY:
            for (int i = 0; i < node->value_count; i++) {
                if (camera_has_tag(camera, node->values[i])) {
                    explain(explanation, "tag_any=%s", node->values[i]);
                    return true;
                }
            }
            return false;
        case SELECTOR_TAG_ALL:
            for (int i = 0; i < node->value_count; i++) {
                if (!camera_has_tag(camera, node->values[i])) return false;
            }
            explain(explanation, "tag_all (%d tags)", node->value_count);
            return true;
        case SELECTOR_TAG_NONE:
            for (int i = 0; i < node->value_count; i++) {
                if (camera_has_tag(camera, node->values[i])) return false;
            }
            explain(explanation, "tag_none (%d tags)", node->value_count);
            return true;
        case SELECTOR_ENABLED:
            if (camera->enabled == node->bool_value) {
                explain(explanation, "enabled=%s", node->bool_value ? "true" : "false");
                return true;
            }
            return false;
        case SELECTOR_RECORDING_MODE: {
            const char *mode = fleet_camera_recording_mode(camera);
            if (string_in_values(mode, node)) {
                explain(explanation, "recording_mode=%s", mode);
                return true;
            }
            return false;
        }
        case SELECTOR_VENDOR:
            if (string_in_values(camera->manufacturer, node)) {
                explain(explanation, "vendor=%s", camera->manufacturer);
                return true;
            }
            return false;
        case SELECTOR_MODEL:
            if (string_in_values(camera->model, node)) {
                explain(explanation, "model=%s", camera->model);
                return true;
            }
            return false;
        case SELECTOR_CAPABILITY_ANY:
            for (int i = 0; i < node->value_count; i++) {
                if (camera_has_capability(camera, node->values[i])) {
                    explain(explanation, "capability_any=%s", node->values[i]);
                    return true;
                }
            }
            return false;
        case SELECTOR_CAPABILITY_ALL:
            for (int i = 0; i < node->value_count; i++) {
                if (!camera_has_capability(camera, node->values[i])) return false;
            }
            explain(explanation, "capability_all (%d capabilities)",
                    node->value_count);
            return true;
        case SELECTOR_HEALTH: {
            const char *health = fleet_health_state_name(camera->health);
            if (string_in_values(health, node)) {
                explain(explanation, "health=%s", health);
                return true;
            }
            return false;
        }
    }
    return false;
}

bool fleet_selector_matches(const fleet_selector_t *selector,
                            const fleet_camera_t *camera,
                            fleet_selector_explanation_t *explanation) {
    if (!selector || !selector->root || !camera) return false;
    if (explanation) memset(explanation, 0, sizeof(*explanation));
    return evaluate_node(selector->root, camera, explanation);
}
