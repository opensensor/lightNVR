/* SPDX-License-Identifier: GPL-3.0-or-later */
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include "web/openapi_projection.h"
#include "database/db_authorization_snapshot.h"

#define MAX_DEPTH 96
#define MAX_REFERENCE 2048
#define AUTH "x-lightnvr-authorization"
#define VISIBILITY "x-lightnvr-visibility"
static cJSON *item(const cJSON *node, const char *key) {
    return cJSON_GetObjectItemCaseSensitive(node, key);
}
static bool string_is(const cJSON *node, const char *value) {
    return cJSON_IsString(node) && strcmp(node->valuestring, value) == 0;
}
static bool unique_keys(const cJSON *node) {
    if (!cJSON_IsObject(node)) return true;
    for (const cJSON *a = node->child; a; a = a->next) {
        if (!a->string) return false;
        for (const cJSON *b = a->next; b; b = b->next)
            if (!b->string || strcmp(a->string, b->string) == 0) return false;
    }
    return true;
}
static bool is_method(const char *key) {
    const char *methods[] = {"get", "put", "post", "delete", "patch", "options", "head", "trace"};
    for (size_t i = 0; i < sizeof(methods)/sizeof(*methods); i++)
        if (!strcmp(key, methods[i])) return true;
    return false;
}
static int predicate_valid(const cJSON *predicate) {
    if (!cJSON_IsObject(predicate) || !unique_keys(predicate)) return -1;
    cJSON *version = item(predicate, "version"), *kind = item(predicate, "kind");
    if (!cJSON_IsNumber(version) || version->valuedouble != 1 || !cJSON_IsString(kind)) return -1;
    bool simple = string_is(kind, "authenticated") || string_is(kind, "public") || string_is(kind, "bootstrap_setup") ||
                  string_is(kind, "non_scoped_identity") || string_is(kind, "identity_management");
    bool camera = string_is(kind, "camera") || string_is(kind, "non_scoped_camera") ||
                  string_is(kind, "global_or_camera");
    bool global = string_is(kind, "global") || string_is(kind, "non_scoped_global");
    if ((!simple && !camera && !global) || cJSON_GetArraySize(predicate) != (simple ? 2 : 3)) return -1;
    if (simple) return item(predicate, "requirements") ? -1 : 0;
    cJSON *requirements = item(predicate, "requirements");
    if (!cJSON_IsArray(requirements) || !requirements->child) return -1;
    for (cJSON *req = requirements->child; req; req = req->next) {
        if (!cJSON_IsObject(req) || !unique_keys(req) || cJSON_GetArraySize(req) != 2) return -1;
        cJSON *action = item(req, "action"), *resource = item(req, "resource");
        if (!cJSON_IsString(action) || authorization_action_from_key(action->valuestring) == AUTHZ_ACTION_INVALID ||
            !(string_is(resource, "raw") || string_is(resource, "runtime") || string_is(resource, "live")) ||
            ((global || string_is(kind, "global_or_camera")) && !string_is(resource, "raw"))) return -1;
    }
    return 0;
}

int openapi_snapshot_access(void *opaque, const cJSON *predicate) {
    authorization_snapshot_t *snapshot = opaque;
    if (!snapshot || predicate_valid(predicate) != 0) return -1;
    cJSON *kind = item(predicate, "kind");
    if (string_is(kind, "authenticated") || string_is(kind, "public")) return 1;
    if (string_is(kind, "bootstrap_setup")) {
        int complete = authorization_snapshot_setup_complete(snapshot);
        if (complete < 0) return -1;
        if (!complete) return 1;
        authorization_evaluation_t evaluation;
        if (authorization_snapshot_evaluate(snapshot, AUTHZ_SYSTEM_ADMIN, -1,
                                             "raw", &evaluation) != 0) return -1;
        return evaluation.decision == AUTHZ_DECISION_ALLOW ? 1 : 0;
    }
    if (string_is(kind, "non_scoped_identity"))
        return authorization_snapshot_allows_non_scoped_identity(snapshot) ? 1 : 0;
    if (string_is(kind, "identity_management")) {
        if (authorization_snapshot_allows_non_scoped_identity(snapshot)) return 1;
        authorization_evaluation_t evaluation;
        if (authorization_snapshot_evaluate(snapshot, AUTHZ_USERS_MANAGE, -1,
                                             "raw", &evaluation) != 0) return -1;
        return evaluation.decision == AUTHZ_DECISION_ALLOW ? 1 : 0;
    }
    bool global = string_is(kind, "global") || string_is(kind, "non_scoped_global");
    if ((string_is(kind, "non_scoped_global") || string_is(kind, "non_scoped_camera")) &&
        !authorization_snapshot_allows_non_scoped_identity(snapshot)) return 0;
    int count = global ? 0 : authorization_snapshot_camera_count(snapshot);
    for (int index = (global || string_is(kind, "global_or_camera")) ? -1 : 0; index < count; index++) {
        bool allowed = true;
        for (cJSON *req = item(predicate, "requirements")->child; req; req = req->next) {
            authorization_evaluation_t evaluation;
            if (authorization_snapshot_evaluate(snapshot,
                    authorization_action_from_key(item(req, "action")->valuestring),
                    index, item(req, "resource")->valuestring, &evaluation) != 0) return -1;
            if (evaluation.decision != AUTHZ_DECISION_ALLOW) { allowed = false; break; }
        }
        if (allowed) return 1;
    }
    return 0;
}

typedef struct {
    cJSON *document;
    const cJSON *canonical;
    openapi_access_fn access;
    void *access_context;
    cJSON *operations; /* kept operation IDs */
    cJSON *marks; /* component category -> marked names */
    cJSON *queue; /* [{category,name}] */
    bool filtering;
    bool collecting;
} projection_t;

/* Decode one JSON Pointer token, rejecting malformed escapes and truncation. */
static int decode_token(const char *start, size_t length, char *out, size_t capacity) {
    size_t written = 0;
    for (size_t i = 0; i < length; i++) {
        char value = start[i];
        if (value == '~') {
            if (++i >= length || (start[i] != '0' && start[i] != '1')) return -1;
            value = start[i] == '0' ? '~' : '/';
        }
        if (written + 1 >= capacity) return -1;
        out[written++] = value;
    }
    out[written] = '\0';
    return 0;
}
static const cJSON *resolve_pointer(const cJSON *root, const char *reference) {
    if (!reference || strncmp(reference, "#/", 2) || strlen(reference) >= MAX_REFERENCE) return NULL;
    const cJSON *node = root;
    const char *part = reference + 2;
    for (;;) {
        const char *slash = strchr(part, '/');
        size_t length = slash ? (size_t)(slash - part) : strlen(part);
        char key[MAX_REFERENCE];
        if (decode_token(part, length, key, sizeof(key)) != 0) return NULL;
        if (cJSON_IsObject(node)) node = item(node, key);
        /* Array positions can change during variant filtering. Supporting such
         * references requires rebasing; fail closed rather than retarget them. */
        else return NULL;
        if (!node) return NULL;
        if (!slash) return node;
        part = slash + 1;
    }
}
static bool component_category(const char *name) {
    const char *categories[] = {"schemas", "responses", "parameters", "examples",
        "requestBodies", "headers", "securitySchemes", "links"};
    for (size_t i = 0; i < sizeof(categories)/sizeof(*categories); i++)
        if (!strcmp(categories[i], name)) return true;
    return false;
}
static int mark_component(projection_t *p, const char *category, const char *name) {
    cJSON *definitions = item(item(p->document, "components"), category);
    if (!component_category(category) || !cJSON_IsObject(definitions) || !item(definitions, name)) return -1;
    if (!p->collecting) return 0;
    cJSON *marks = item(p->marks, category);
    if (!marks && !(marks = cJSON_AddObjectToObject(p->marks, category))) return -1;
    if (item(marks, name)) return 0;
    if (!cJSON_AddBoolToObject(marks, name, true)) return -1;
    cJSON *entry = cJSON_CreateObject();
    if (!entry) return -1;
    if (!cJSON_AddStringToObject(entry, "category", category) ||
        !cJSON_AddStringToObject(entry, "name", name) || !cJSON_AddItemToArray(p->queue, entry)) {
        cJSON_Delete(entry);
        return -1;
    }
    return 0;
}
static int reference(projection_t *p, const cJSON *value) {
    if (!cJSON_IsString(value) || !resolve_pointer(p->canonical, value->valuestring)) return -1;
    const char *ref = value->valuestring;
    if (strncmp(ref, "#/components/", 13)) return -1;
    const char *category_start = ref + 13, *slash = strchr(category_start, '/');
    if (!slash) return -1;
    const char *name_start = slash + 1, *end = strchr(name_start, '/');
    char category[MAX_REFERENCE], name[MAX_REFERENCE];
    if (decode_token(category_start, (size_t)(slash-category_start), category, sizeof(category)) ||
        decode_token(name_start, end ? (size_t)(end-name_start) : strlen(name_start), name, sizeof(name))) return -1;
    return mark_component(p, category, name);
}

static bool member(const cJSON *strings, const char *value) {
    for (const cJSON *n = strings ? strings->child : NULL; n; n = n->next)
        if (string_is(n, value)) return true;
    return false;
}
static int add_string(cJSON *array, const char *value) {
    if (member(array, value)) return 0;
    cJSON *string = cJSON_CreateString(value);
    if (!string) return -1;
    if (!cJSON_AddItemToArray(array, string)) { cJSON_Delete(string); return -1; }
    return 0;
}
static void erase(cJSON *parent, cJSON *child) {
    cJSON_Delete(cJSON_DetachItemViaPointer(parent, child));
}

/* Schema traversal deliberately distinguishes keyword objects from named maps
 * and literal defaults/examples. A property named "$ref" is still a property. */
static int walk_schema(projection_t *p, cJSON *node, unsigned depth, bool alternative);
static int walk_object(projection_t *p, cJSON *node, unsigned depth, bool operation);

static bool literal_schema_keyword(const char *key) {
    const char *keys[] = {"type", "enum", "const", "default", "example", "examples",
        "title", "description", "$comment", "format", "pattern", "minLength", "maxLength",
        "minimum", "maximum", "exclusiveMinimum", "exclusiveMaximum", "multipleOf",
        "minItems", "maxItems", "uniqueItems", "minContains", "maxContains",
        "minProperties", "maxProperties", "required", "dependentRequired",
        "readOnly", "writeOnly", "deprecated", "contentEncoding", "contentMediaType"};
    for (size_t i = 0; i < sizeof(keys)/sizeof(*keys); i++)
        if (!strcmp(key, keys[i])) return true;
    return !strncmp(key, "x-", 2);
}

static int schema_map(projection_t *p, cJSON *map, unsigned depth) {
    if (!cJSON_IsObject(map) || !unique_keys(map)) return -1;
    for (cJSON *entry = map->child; entry; entry = entry->next)
        if (walk_schema(p, entry, depth + 1, false)) return -1;
    return 0;
}
static int discriminator(projection_t *p, cJSON *schema) {
    cJSON *disc = item(schema, "discriminator");
    if (!disc) return 0;
    if (!cJSON_IsObject(disc) || !unique_keys(disc) || !cJSON_IsString(item(disc, "propertyName"))) return -1;
    for (cJSON *field = disc->child; field; field = field->next) {
        if (!strcmp(field->string, AUTH) || !strcmp(field->string, VISIBILITY)) return -1;
        if (strcmp(field->string, "propertyName") && strcmp(field->string, "mapping") &&
            strncmp(field->string, "x-", 2)) return -1;
    }
    cJSON *mapping = item(disc, "mapping");
    if (!mapping) return 0;
    if (!cJSON_IsObject(mapping) || !unique_keys(mapping)) return -1;
    cJSON *choices = item(schema, "oneOf");
    if (!choices) choices = item(schema, "anyOf");
    for (cJSON *m = mapping->child, *next; m; m = next) {
        next = m->next;
        if (!cJSON_IsString(m)) return -1;
        const cJSON *target = NULL;
        if (!strncmp(m->valuestring, "#/", 2)) target = resolve_pointer(p->canonical, m->valuestring);
        else target = item(item(item(p->canonical, "components"), "schemas"), m->valuestring);
        if (!target) return -1;
        if (p->filtering && choices) {
            bool found = false;
            for (cJSON *choice = choices->child; choice; choice = choice->next) {
                cJSON *ref = item(choice, "$ref");
                if (cJSON_IsString(ref) && resolve_pointer(p->canonical, ref->valuestring) == target) found = true;
            }
            if (!found) { erase(mapping, m); continue; }
        }
        if (!strncmp(m->valuestring, "#/", 2)) {
            if (reference(p, m)) return -1;
        } else if (mark_component(p, "schemas", m->valuestring)) return -1;
    }
    return 0;
}
static int walk_schema(projection_t *p, cJSON *node, unsigned depth, bool alternative) {
    if (depth > MAX_DEPTH) return -1;
    if (cJSON_IsBool(node)) return 0;
    if (!cJSON_IsObject(node) || !unique_keys(node)) return -1;
    cJSON *visibility = item(node, VISIBILITY);
    if (visibility && (!alternative || predicate_valid(visibility))) return -1;
    if (item(node, AUTH)) return -1;
    for (cJSON *child = node->child, *next; child; child = next) {
        next = child->next;
        const char *key = child->string;
        if (!strcmp(key, VISIBILITY)) {
            if (p->filtering) erase(node, child);
        } else if (!strcmp(key, "$ref")) {
            if (reference(p, child)) return -1;
        } else if (!strcmp(key, "$id") || !strcmp(key, "$anchor") ||
                   !strcmp(key, "$dynamicRef") || !strcmp(key, "$dynamicAnchor") ||
                   !strcmp(key, "$recursiveRef") || !strcmp(key, "$recursiveAnchor")) return -1;
        else if (!strcmp(key, "properties") || !strcmp(key, "patternProperties") ||
                 !strcmp(key, "$defs") || !strcmp(key, "definitions") || !strcmp(key, "dependentSchemas")) {
            if (schema_map(p, child, depth + 1)) return -1;
        } else if (!strcmp(key, "anyOf") || !strcmp(key, "oneOf") ||
                   !strcmp(key, "allOf") || !strcmp(key, "prefixItems")) {
            bool variants = !strcmp(key, "anyOf") || !strcmp(key, "oneOf");
            if (!cJSON_IsArray(child) || !child->child) return -1;
            for (cJSON *branch = child->child, *after; branch; branch = after) {
                after = branch->next;
                cJSON *predicate = item(branch, VISIBILITY);
                if (predicate && (!variants || predicate_valid(predicate))) return -1;
                if (predicate && p->filtering) {
                    int allowed = p->access(p->access_context, predicate);
                    if (allowed != 0 && allowed != 1) return -1;
                    if (!allowed) { erase(child, branch); continue; }
                }
                if (walk_schema(p, branch, depth + 1, variants)) return -1;
            }
            if (!child->child) return -1; /* Never widen an empty required union. */
        } else if (!strcmp(key, "items") || !strcmp(key, "contains") ||
                   !strcmp(key, "additionalProperties") || !strcmp(key, "unevaluatedProperties") ||
                   !strcmp(key, "unevaluatedItems") || !strcmp(key, "propertyNames") ||
                   !strcmp(key, "not") || !strcmp(key, "if") || !strcmp(key, "then") ||
                   !strcmp(key, "else") || !strcmp(key, "contentSchema")) {
            if (walk_schema(p, child, depth + 1, false)) return -1;
        } else if (!strcmp(key, "discriminator")) {
            /* Process named mapping values after filtering all alternatives. */
        } else if (!strcmp(key, "xml") || !strcmp(key, "externalDocs")) {
            if (walk_object(p, child, depth + 1, false)) return -1;
        } else if (!strcmp(key, "$schema")) {
            if (!string_is(child, "https://json-schema.org/draft/2020-12/schema") &&
                !string_is(child, "https://spec.openapis.org/oas/3.1/dialect/base")) return -1;
        } else if (!literal_schema_keyword(key)) return -1;
    }
    return discriminator(p, node);
}
static int security(projection_t *p, cJSON *value) {
    if (!cJSON_IsArray(value)) return -1;
    for (cJSON *requirement = value->child; requirement; requirement = requirement->next) {
        if (!cJSON_IsObject(requirement) || !unique_keys(requirement)) return -1;
        for (cJSON *scheme = requirement->child; scheme; scheme = scheme->next) {
            if (!cJSON_IsArray(scheme) || mark_component(p, "securitySchemes", scheme->string)) return -1;
            for (cJSON *scope = scheme->child; scope; scope = scope->next)
                if (!cJSON_IsString(scope)) return -1;
        }
    }
    return 0;
}
static int link_allowed(projection_t *p, const cJSON *link, unsigned depth) {
    if (depth > MAX_DEPTH || !cJSON_IsObject(link)) return -1;
    cJSON *ref = item(link, "$ref");
    if (ref) {
        if (!cJSON_IsString(ref)) return -1;
        const cJSON *resolved = resolve_pointer(p->canonical, ref->valuestring);
        if (!resolved) return -1;
        return link_allowed(p, resolved, depth + 1);
    }
    cJSON *id = item(link, "operationId"), *operation_ref = item(link, "operationRef");
    if (id && operation_ref) return -1;
    if (operation_ref) {
        if (!cJSON_IsString(operation_ref) || strncmp(operation_ref->valuestring, "#/paths/", 8)) return -1;
        const cJSON *operation = resolve_pointer(p->canonical, operation_ref->valuestring);
        if (!operation) return -1;
        id = item(operation, "operationId");
    }
    if (!cJSON_IsString(id)) return -1;
    if (member(p->operations, id->valuestring)) return 1;
    return p->filtering ? 0 : -1;
}
static int object_map(projection_t *p, cJSON *map, unsigned depth, bool links) {
    if (!cJSON_IsObject(map) || !unique_keys(map)) return -1;
    for (cJSON *entry = map->child, *next; entry; entry = next) {
        next = entry->next;
        if (links) {
            int allowed = link_allowed(p, entry, 0);
            if (allowed < 0) return -1;
            if (!allowed) { erase(map, entry); continue; }
        }
        if (walk_object(p, entry, depth + 1, false)) return -1;
    }
    return 0;
}
static int walk_object(projection_t *p, cJSON *node, unsigned depth, bool operation) {
    if (depth > MAX_DEPTH) return -1;
    if (cJSON_IsArray(node)) {
        for (cJSON *child = node->child; child; child = child->next)
            if (walk_object(p, child, depth + 1, false)) return -1;
        return 0;
    }
    if (!cJSON_IsObject(node)) return 0;
    if (!unique_keys(node)) return -1;
    for (cJSON *child = node->child, *next; child; child = next) {
        next = child->next;
        const char *key = child->string;
        if (!strcmp(key, AUTH)) {
            if (!operation || predicate_valid(child)) return -1;
            if (p->filtering) erase(node, child);
        } else if (!strcmp(key, VISIBILITY) || !strcmp(key, "callbacks") ||
                   !strcmp(key, "webhooks") || !strcmp(key, "externalValue")) return -1;
        else if (!strcmp(key, "$ref")) {
            if (reference(p, child)) return -1;
        } else if (!strcmp(key, "operationRef")) {
            if (!cJSON_IsString(child) || !resolve_pointer(p->canonical, child->valuestring)) return -1;
        } else if (!strcmp(key, "schema")) {
            if (walk_schema(p, child, depth + 1, false)) return -1;
        } else if (!strcmp(key, "security")) {
            if (security(p, child)) return -1;
        } else if ((!strcmp(key, "parameters") || !strcmp(key, "requestBody")) &&
                   (item(node, "operationId") || item(node, "operationRef")) && !operation) {
            /* Link values are runtime expressions or literal payloads. */
            if (!strcmp(key, "parameters") && (!cJSON_IsObject(child) || !unique_keys(child))) return -1;
        } else if (!strcmp(key, "content") || !strcmp(key, "responses") ||
                   !strcmp(key, "encoding") || !strcmp(key, "variables") ||
                   !strcmp(key, "headers") || !strcmp(key, "examples") || !strcmp(key, "links")) {
            if (object_map(p, child, depth + 1, !strcmp(key, "links"))) return -1;
        } else if (!strcmp(key, "example") || !strcmp(key, "value") ||
                   !strcmp(key, "default") || !strncmp(key, "x-", 2)) {
            /* Literal examples and extension data are not OpenAPI objects. */
        } else if (walk_object(p, child, depth + 1, false)) return -1;
    }
    return 0;
}
static int paths(projection_t *p, bool select) {
    cJSON *map = item(p->document, "paths");
    if (!cJSON_IsObject(map) || !unique_keys(map)) return -1;
    for (cJSON *path = map->child, *next_path; path; path = next_path) {
        next_path = path->next;
        if (!cJSON_IsObject(path) || !unique_keys(path) || path->string[0] != '/') return -1;
        int methods = 0;
        for (cJSON *op = path->child, *next; op; op = next) {
            next = op->next;
            if (!is_method(op->string)) {
                if (!strcmp(op->string, "$ref") || !strcmp(op->string, AUTH) ||
                    !strcmp(op->string, VISIBILITY)) return -1;
                if (walk_object(p, op, 1, false)) return -1;
                continue;
            }
            if (!cJSON_IsObject(op)) return -1;
            cJSON *id = item(op, "operationId"), *predicate = item(op, AUTH);
            if (!cJSON_IsString(id) || !id->valuestring[0]) return -1;
            if (select) {
                if (predicate_valid(predicate)) return -1;
                int allowed = p->access(p->access_context, predicate);
                if (allowed != 0 && allowed != 1) return -1;
                if (!allowed) { erase(path, op); continue; }
                if (add_string(p->operations, id->valuestring)) return -1;
            } else if (walk_object(p, op, 1, true)) return -1;
            methods++;
        }
        if (select && methods == 0) erase(map, path);
    }
    return 0;
}
static int root_fields(projection_t *p) {
    for (cJSON *field = p->document->child; field; field = field->next) {
        if (!strcmp(field->string, "components") || !strcmp(field->string, "paths")) continue;
        if (!strcmp(field->string, "webhooks") || !strcmp(field->string, AUTH) ||
            !strcmp(field->string, VISIBILITY)) return -1;
        if (!strcmp(field->string, "jsonSchemaDialect") &&
            !string_is(field, "https://json-schema.org/draft/2020-12/schema") &&
            !string_is(field, "https://spec.openapis.org/oas/3.1/dialect/base")) return -1;
        if (!strcmp(field->string, "security")) { if (security(p, field)) return -1; }
        else if (!strncmp(field->string, "x-", 2)) continue;
        else if (walk_object(p, field, 1, false)) return -1;
    }
    return 0;
}
static int components(projection_t *p) {
    cJSON *all = item(p->document, "components");
    if (!all) return 0;
    if (!cJSON_IsObject(all) || !unique_keys(all)) return -1;
    for (cJSON *category = all->child; category; category = category->next) {
        if (!component_category(category->string) || !cJSON_IsObject(category) || !unique_keys(category)) return -1;
        for (cJSON *definition = category->child; definition; definition = definition->next) {
            int error = !strcmp(category->string, "schemas")
                ? walk_schema(p, definition, 1, false)
                : walk_object(p, definition, 1, false);
            if (error) return -1;
        }
    }
    return 0;
}
int openapi_project(const cJSON *canonical, openapi_access_fn access,
                    void *context, cJSON **result) {
    if (!result) return -1;
    *result = NULL;
    if (!cJSON_IsObject(canonical) || !unique_keys(canonical) || !access ||
        !string_is(item(canonical, "openapi"), "3.1.1")) return -1;
    projection_t p = {.canonical = canonical, .access = access, .access_context = context};
    p.document = cJSON_Duplicate(canonical, true);
    p.operations = cJSON_CreateArray(); p.marks = cJSON_CreateObject(); p.queue = cJSON_CreateArray();
    int status = -1;
    if (!p.document || !p.operations || !p.marks || !p.queue) goto done;
    cJSON *path_map = item(p.document, "paths");
    if (!cJSON_IsObject(path_map)) goto done;
    /* Full ID catalog validates links even when target operations later disappear. */
    for (cJSON *path = path_map->child; path; path = path->next) {
        for (cJSON *op = path->child; op; op = op->next) {
            if (!op->string || !is_method(op->string)) continue;
            cJSON *id = item(op, "operationId");
            if (!cJSON_IsString(id) || member(p.operations, id->valuestring) ||
                predicate_valid(item(op, AUTH)) || add_string(p.operations, id->valuestring)) goto done;
        }
    }
    if (paths(&p, false) || root_fields(&p) || components(&p)) goto done;
    cJSON_Delete(p.operations); p.operations = cJSON_CreateArray();
    if (!p.operations) goto done;
    p.filtering = true;
    /* Select all operations before pruning links, including forward links. */
    if (paths(&p, true)) goto done;
    p.collecting = true;
    if (paths(&p, false) || root_fields(&p)) goto done;
    for (int index = 0; index < cJSON_GetArraySize(p.queue); index++) {
        cJSON *entry = cJSON_GetArrayItem(p.queue, index);
        const char *category = item(entry, "category")->valuestring;
        cJSON *definition = item(item(item(p.document, "components"), category), item(entry, "name")->valuestring);
        if (!definition) goto done;
        if (!strcmp(category, "schemas")) {
            if (walk_schema(&p, definition, 1, false)) goto done;
        } else if (walk_object(&p, definition, 1, false)) goto done;
    }
    cJSON *all = item(p.document, "components");
    for (cJSON *category = all ? all->child : NULL, *next_category; category; category = next_category) {
        next_category = category->next;
        for (cJSON *definition = category->child, *next; definition; definition = next) {
            next = definition->next;
            if (!item(item(p.marks, category->string), definition->string)) erase(category, definition);
        }
        if (!category->child) erase(all, category);
    }
    cJSON *tags = item(p.document, "tags");
    if (tags) {
        if (!cJSON_IsArray(tags)) goto done;
        for (cJSON *tag = tags->child, *next; tag; tag = next) {
            next = tag->next;
            cJSON *name = item(tag, "name"); bool used = false;
            if (!cJSON_IsString(name)) goto done;
            for (cJSON *path = path_map->child; path && !used; path = path->next)
                for (cJSON *op = path->child; op; op = op->next)
                    if (is_method(op->string) && member(item(op, "tags"), name->valuestring)) used = true;
            if (!used) erase(tags, tag);
        }
    }
    cJSON_DeleteItemFromObjectCaseSensitive(p.document, "x-lightnvr-validators");
    /* Resolve against the result as well: a reference into an alternative may
     * have become invalid when that alternative was removed. Never emit it. */
    p.canonical = p.document;
    p.filtering = false;
    p.collecting = false;
    if (paths(&p, false) || root_fields(&p) || components(&p)) goto done;
    *result = p.document; p.document = NULL; status = 0;
 done:
    cJSON_Delete(p.document); cJSON_Delete(p.operations); cJSON_Delete(p.marks); cJSON_Delete(p.queue);
    return status;
}
