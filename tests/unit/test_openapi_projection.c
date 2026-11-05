/* SPDX-License-Identifier: GPL-3.0-or-later */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include "unity.h"
#include "web/openapi_projection.h"
#include "web/openapi_document_generated.h"
#include "database/db_authorization_snapshot.h"

static cJSON *input, *output;
static int mode, allocation_limit = -1, evaluation_error;
static unsigned allowed[2];
static const char *expected_resource;
static int global_calls;
static bool non_scoped_identity = true;
void setUp(void) { input = output = NULL; mode = 0; non_scoped_identity = true; evaluation_error = 0; global_calls = 0; expected_resource = NULL; allowed[0] = allowed[1] = 0; }
void tearDown(void) { allocation_limit = -1; cJSON_InitHooks(NULL); cJSON_Delete(input); cJSON_Delete(output); }
static cJSON *get(const cJSON *j, const char *key) { return cJSON_GetObjectItemCaseSensitive(j, key); }
static int access_rule(void *ctx, const cJSON *predicate) {
    (void)ctx;
    if (mode == 3) return -1;
    if (mode == 2) return 0;
    if (mode == 0) return 1;
    cJSON *requirements = get(predicate, "requirements");
    for (cJSON *req = requirements ? requirements->child : NULL; req; req = req->next)
        if (!strcmp(get(req, "action")->valuestring, "camera.configure")) return 0;
    return 1;
}
bool __wrap_authorization_snapshot_allows_non_scoped_identity(const authorization_snapshot_t *s) { return s && non_scoped_identity; }
int __wrap_authorization_snapshot_camera_count(const authorization_snapshot_t *s) { (void)s; return 2; }
int __wrap_authorization_snapshot_evaluate(authorization_snapshot_t *s, authorization_action_t action,
        int index, const char *resource, authorization_evaluation_t *result) {
    (void)s;
    if (evaluation_error) return -1;
    if (expected_resource) TEST_ASSERT_EQUAL_STRING(expected_resource, resource);
    if (index == -1) { global_calls++; result->decision = AUTHZ_DECISION_ALLOW; }
    else result->decision = (allowed[index] & (1u << action)) ? AUTHZ_DECISION_ALLOW : AUTHZ_DECISION_DENY;
    return 0;
}
static void parse(const char *json) { input = cJSON_Parse(json); TEST_ASSERT_NOT_NULL(input); }
static void canonical(void) { parse((const char *)lightnvr_openapi_document); }
static void project(void) { TEST_ASSERT_EQUAL_INT(0, openapi_project(input, access_rule, NULL, &output)); TEST_ASSERT_NOT_NULL(output); }
static void rejected(void) { TEST_ASSERT_EQUAL_INT(-1, openapi_project(input, access_rule, NULL, &output)); TEST_ASSERT_NULL(output); }
static void emit(const char *name) {
    const char *dir = getenv("LIGHTNVR_OPENAPI_OUTPUT_DIR");
    if (!dir) return;
    char path[4096]; TEST_ASSERT_TRUE(snprintf(path, sizeof(path), "%s/%s.json", dir, name) < (int)sizeof(path));
    char *json = cJSON_PrintUnformatted(output); TEST_ASSERT_NOT_NULL(json);
    FILE *file = fopen(path, "w"); TEST_ASSERT_NOT_NULL(file);
    TEST_ASSERT_TRUE(fputs(json, file) >= 0); TEST_ASSERT_EQUAL_INT(0, fclose(file)); free(json);
}
static cJSON *schemas(void) { return get(get(output, "components"), "schemas"); }
static void test_canonical_admin_immutable(void) {
    canonical(); char *before = cJSON_PrintUnformatted(input); project();
    char *after = cJSON_PrintUnformatted(input); TEST_ASSERT_EQUAL_STRING(before, after); free(before); free(after);
    TEST_ASSERT_EQUAL_INT(30, cJSON_GetArraySize(get(output, "paths")));
    TEST_ASSERT_NULL(get(output, "x-lightnvr-validators"));
    TEST_ASSERT_NOT_NULL(get(get(schemas(), "OpenApiDocument"), "properties"));
    TEST_ASSERT_NOT_NULL(get(get(get(schemas(), "OpenApiDocument"), "properties"), "x-lightnvr-coverage"));
    cJSON *full = get(get(get(output, "paths"), "/api/streams/{name}/full"), "get");
    TEST_ASSERT_NULL(get(full, "x-lightnvr-authorization")); emit("admin");
}
static void test_viewer_prunes_variants(void) {
    canonical(); mode = 1; project();
    TEST_ASSERT_NULL(get(get(output, "paths"), "/api/streams/{name}"));
    TEST_ASSERT_NOT_NULL(get(get(output, "paths"), "/api/streams/{name}/full"));
    char *text = cJSON_PrintUnformatted(output); TEST_ASSERT_NULL(strstr(text, "\"x-lightnvr-visibility\":")); free(text);
    emit("viewer");
}
static void test_empty_valid(void) { canonical(); mode = 2; project(); TEST_ASSERT_EQUAL_INT(0, cJSON_GetArraySize(get(output, "paths"))); emit("empty"); }
static void test_callback_failure(void) { canonical(); mode = 3; rejected(); }
static void test_bad_predicates_even_hidden(void) {
    const char *bad[] = {
        "{\"version\":2,\"kind\":\"authenticated\"}",
        "{\"version\":1,\"kind\":\"unknown\"}",
        "{\"version\":1,\"kind\":\"camera\",\"requirements\":[]}",
        "{\"version\":1,\"kind\":\"camera\",\"requirements\":[{\"action\":\"bad\",\"resource\":\"raw\"}]}",
        "{\"version\":1,\"kind\":\"camera\",\"requirements\":[{\"action\":\"live.view\",\"resource\":\"bad\"}]}",
        "{\"version\":1,\"kind\":\"global\",\"requirements\":[{\"action\":\"system.admin\",\"resource\":\"live\"}]}",
        "{\"version\":1,\"kind\":\"authenticated\",\"extra\":true}",
        "{\"version\":1,\"kind\":\"non_scoped_identity\",\"requirements\":[]}",
        "{\"version\":1,\"kind\":\"non_scoped_identity\",\"extra\":true}"
    };
    for (size_t i = 0; i < sizeof(bad)/sizeof(*bad); i++) {
        canonical(); cJSON *op = get(get(get(input, "paths"), "/api/openapi.json"), "get");
        cJSON_ReplaceItemInObjectCaseSensitive(op, "x-lightnvr-authorization", cJSON_Parse(bad[i]));
        mode = 2; rejected(); cJSON_Delete(input); input = NULL;
    }
}
#define SIMPLE "{\"version\":1,\"kind\":\"authenticated\"}"
#define CAMERA "{\"version\":1,\"kind\":\"camera\",\"requirements\":[{\"action\":\"camera.configure\",\"resource\":\"runtime\"}]}"
static const char fixture[] =
"{\"openapi\":\"3.1.1\",\"info\":{\"title\":\"Projection fixture\",\"version\":\"1\"},\"servers\":[{\"url\":\"https://example.invalid\"}],"
"\"paths\":{\"/read\":{\"get\":{\"operationId\":\"read\",\"summary\":\"Read\",\"x-lightnvr-authorization\":" SIMPLE ",\"security\":[{\"Session\":[]}],"
"\"responses\":{\"200\":{\"description\":\"OK\",\"content\":{\"application/json\":{\"schema\":{\"$ref\":\"#/components/schemas/Choice\"}}},"
"\"links\":{\"Write\":{\"$ref\":\"#/components/links/Write\"}}}}}},"
"\"/write\":{\"put\":{\"operationId\":\"write\",\"summary\":\"Write\",\"x-lightnvr-authorization\":" CAMERA ",\"security\":[{\"Secret\":[]}],\"responses\":{\"204\":{\"description\":\"OK\"}}}}},"
"\"components\":{\"schemas\":{\"Choice\":{\"anyOf\":[{\"$ref\":\"#/components/schemas/A\"},{\"$ref\":\"#/components/schemas/Hidden\",\"x-lightnvr-visibility\":" CAMERA "}],"
"\"discriminator\":{\"propertyName\":\"kind\",\"mapping\":{\"read\":\"#/components/schemas/A\",\"write\":\"Hidden\"}}},"
"\"A\":{\"type\":\"object\",\"properties\":{\"kind\":{\"type\":\"string\"},\"next\":{\"$ref\":\"#/components/schemas/A\"},\"A/B~C\":{\"type\":\"string\"},\"escaped\":{\"$ref\":\"#/components/schemas/A/properties/A~1B~0C\"},\"$ref\":{\"type\":\"string\"},\"x-lightnvr-visibility\":{\"type\":\"string\"}},\"example\":{\"kind\":\"read\"}},"
"\"Hidden\":{\"type\":\"object\",\"properties\":{\"kind\":{\"type\":\"string\"}},\"example\":{\"secret\":\"hidden-fixture\"}},\"Unused\":{\"type\":\"string\"}},"
"\"securitySchemes\":{\"Session\":{\"type\":\"apiKey\",\"in\":\"cookie\",\"name\":\"session\"},\"Secret\":{\"type\":\"http\",\"scheme\":\"basic\"}},"
"\"links\":{\"Write\":{\"operationRef\":\"#/paths/~1write/put\"}}}}";
static void test_cycles_pointers_literals_links_discriminator(void) {
    parse(fixture); mode = 1; project();
    TEST_ASSERT_NOT_NULL(get(schemas(), "A")); TEST_ASSERT_NULL(get(schemas(), "Hidden")); TEST_ASSERT_NULL(get(schemas(), "Unused"));
    TEST_ASSERT_NULL(get(get(get(output, "components"), "securitySchemes"), "Secret"));
    TEST_ASSERT_NULL(get(get(output, "components"), "links"));
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetArraySize(get(get(schemas(), "Choice"), "anyOf")));
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetArraySize(get(get(get(schemas(), "Choice"), "discriminator"), "mapping")));
    TEST_ASSERT_NOT_NULL(get(get(get(schemas(), "A"), "properties"), "x-lightnvr-visibility"));
    emit("references");
}
static void test_missing_external_and_bad_escaped_refs(void) {
    const char *refs[] = {"#/components/schemas/Missing", "https://example.invalid/schema", "#/components/schemas/A~2B", "#/paths/~1write/put", "#/components/schemas/Choice/anyOf/1"};
    for (size_t i = 0; i < sizeof(refs)/sizeof(*refs); i++) {
        parse(fixture); cJSON *ref = get(cJSON_GetArrayItem(get(get(get(get(input,"components"),"schemas"),"Choice"),"anyOf"),0),"$ref");
        cJSON_SetValuestring(ref, refs[i]); rejected(); cJSON_Delete(input); input = NULL;
    }
}
static void test_empty_union_fails(void) {
    parse(fixture); cJSON *choices = get(get(get(get(input,"components"),"schemas"),"Choice"),"anyOf");
    cJSON_AddItemToObject(cJSON_GetArrayItem(choices,0), "x-lightnvr-visibility", cJSON_Parse(CAMERA));
    mode = 1; rejected();
}
static void test_misplaced_or_duplicate_metadata(void) {
    canonical(); cJSON_AddItemToObject(input,"x-lightnvr-authorization",cJSON_Parse(SIMPLE)); rejected(); cJSON_Delete(input); input = NULL;
    canonical(); cJSON_AddStringToObject(input,"openapi","3.1.1"); rejected(); cJSON_Delete(input); input = NULL;
    parse(fixture); cJSON_AddItemToObject(get(get(get(input,"components"),"schemas"),"Unused"), "x-lightnvr-visibility",cJSON_Parse(CAMERA)); rejected();
}
static cJSON *add_json(cJSON *parent, const char *name, const char *json) {
    cJSON *value = cJSON_Parse(json);
    TEST_ASSERT_NOT_NULL(value);
    TEST_ASSERT_TRUE(cJSON_AddItemToObject(parent, name, value));
    return value;
}
static void test_named_encoding_and_server_variable_maps(void) {
    parse(fixture);
    cJSON *write = get(get(get(input, "paths"), "/write"), "put");
    add_json(write, "requestBody",
        "{\"content\":{\"multipart/form-data\":{\"schema\":{\"type\":\"object\",\"properties\":{\"example\":{\"type\":\"string\"}}},"
        "\"encoding\":{\"example\":{\"headers\":{\"X-Fixture\":{\"$ref\":\"#/components/headers/FixtureHeader\"}}}}}}}");
    add_json(get(input, "components"), "headers",
        "{\"FixtureHeader\":{\"description\":\"Fixture header\",\"schema\":{\"type\":\"string\"}}}");
    cJSON *server = cJSON_GetArrayItem(get(input, "servers"), 0);
    TEST_ASSERT_NOT_NULL(cJSON_SetValuestring(get(server, "url"), "https://{security}.example.invalid"));
    add_json(server, "variables", "{\"security\":{\"enum\":[\"fixture\",\"other\"],\"default\":\"fixture\"}}");
    project();
    TEST_ASSERT_NOT_NULL(get(get(get(output, "components"), "headers"), "FixtureHeader"));
    cJSON *variable = get(get(cJSON_GetArrayItem(get(output, "servers"), 0), "variables"), "security");
    TEST_ASSERT_EQUAL_STRING("fixture", get(variable, "default")->valuestring);
    TEST_ASSERT_EQUAL_INT(2, cJSON_GetArraySize(get(variable, "enum")));
    cJSON *body = get(get(get(get(output, "paths"), "/write"), "put"), "requestBody");
    cJSON *encoding = get(get(get(body, "content"), "multipart/form-data"), "encoding");
    TEST_ASSERT_EQUAL_STRING("#/components/headers/FixtureHeader", get(get(get(get(encoding, "example"), "headers"), "X-Fixture"), "$ref")->valuestring);
    emit("named-maps");
}
static void test_definitions_preserve_referenced_components(void) {
    parse(fixture);
    cJSON *schema = get(get(get(input, "components"), "schemas"), "A");
    TEST_ASSERT_NOT_NULL(cJSON_AddStringToObject(input, "jsonSchemaDialect", "https://spec.openapis.org/oas/3.1/dialect/base"));
    TEST_ASSERT_NOT_NULL(cJSON_AddStringToObject(schema, "$schema", "https://json-schema.org/draft/2020-12/schema"));
    add_json(schema, "definitions", "{\"Nested\":{\"$ref\":\"#/components/schemas/Unused\"}}");
    add_json(get(schema, "properties"), "nested", "{\"$ref\":\"#/components/schemas/A/definitions/Nested\"}");
    mode = 1; project();
    TEST_ASSERT_NOT_NULL(get(schemas(), "Unused"));
    TEST_ASSERT_EQUAL_STRING("#/components/schemas/Unused", get(get(get(get(schemas(), "A"), "definitions"), "Nested"), "$ref")->valuestring);
    emit("definitions");
}
static void test_link_parameters_and_body_are_literal_payloads(void) {
    parse(fixture);
    cJSON *link = get(get(get(input, "components"), "links"), "Write");
    cJSON *parameters = add_json(link, "parameters",
        "{\"schema\":{\"$ref\":\"literal://parameter\",\"x-lightnvr-visibility\":\"payload\"},\"example\":\"$response.body#/kind\"}");
    cJSON *body = add_json(link, "requestBody",
        "{\"$ref\":\"literal://body\",\"schema\":{\"x-lightnvr-authorization\":\"payload\"},\"number\":0}");
    project();
    cJSON *projected = get(get(get(output, "components"), "links"), "Write");
    TEST_ASSERT_TRUE(cJSON_Compare(parameters, get(projected, "parameters"), true));
    TEST_ASSERT_TRUE(cJSON_Compare(body, get(projected, "requestBody"), true));
    /* Native-only: generic external lint reference resolution can mistake the
     * literal $ref payload fields above for OpenAPI references. */
}
static void test_root_path_and_annotation_metadata_rejected(void) {
    const char *keys[] = {"x-lightnvr-authorization", "x-lightnvr-visibility"};
    for (size_t k = 0; k < sizeof(keys)/sizeof(*keys); k++) {
        for (int placement = 0; placement < 5; placement++) {
            parse(fixture);
            cJSON *target = input;
            if (placement == 1) target = get(get(input, "paths"), "/read");
            if (placement >= 2) {
                cJSON *schema = get(get(get(input, "components"), "schemas"), "Choice");
                target = placement == 2 ? get(schema, "discriminator")
                    : add_json(schema, placement == 3 ? "xml" : "externalDocs",
                        placement == 3 ? "{\"name\":\"fixture\"}" : "{\"url\":\"https://example.invalid/docs\"}");
            }
            add_json(target, keys[k], SIMPLE);
            mode = 2; rejected(); /* Malformed hidden metadata still fails. */
            cJSON_Delete(input); input = NULL;
        }
    }
}
static void test_unsupported_schema_keywords_and_dialects_rejected(void) {
    const char *keys[] = {"$schema", "$vocabulary", "unknownKeyword", "unknownKeyword"};
    const char *values[] = {"\"http://json-schema.org/draft-04/schema#\"",
        "{\"https://example.invalid/vocabulary\":true}", "true",
        "{\"$ref\":\"#/components/schemas/Hidden\"}"};
    for (size_t i = 0; i < sizeof(keys)/sizeof(*keys); i++) {
        parse(fixture);
        add_json(get(get(get(input, "components"), "schemas"), "Unused"), keys[i], values[i]);
        mode = 2; rejected();
        cJSON_Delete(input); input = NULL;
    }
    parse(fixture);
    TEST_ASSERT_NOT_NULL(cJSON_AddStringToObject(input, "jsonSchemaDialect", "http://json-schema.org/draft-04/schema#"));
    mode = 2; rejected();
}
static void test_same_camera_and_resource_adapter(void) {
    parse("{\"version\":1,\"kind\":\"camera\",\"requirements\":[{\"action\":\"live.view\",\"resource\":\"raw\"},{\"action\":\"camera.configure\",\"resource\":\"raw\"}]}");
    authorization_snapshot_t *snapshot = (authorization_snapshot_t *)&mode;
    allowed[0] = 1u << AUTHZ_LIVE_VIEW; allowed[1] = 1u << AUTHZ_CAMERA_CONFIGURE; expected_resource = "raw";
    TEST_ASSERT_EQUAL_INT(0, openapi_snapshot_access(snapshot,input));
    allowed[1] |= 1u << AUTHZ_LIVE_VIEW; TEST_ASSERT_EQUAL_INT(1, openapi_snapshot_access(snapshot,input));
    evaluation_error = 1; TEST_ASSERT_EQUAL_INT(-1, openapi_snapshot_access(snapshot,input)); evaluation_error = 0;
    cJSON_Delete(input); parse(CAMERA); expected_resource = "runtime"; TEST_ASSERT_EQUAL_INT(1, openapi_snapshot_access(snapshot,input));
    cJSON_Delete(input); parse("{\"version\":1,\"kind\":\"global\",\"requirements\":[{\"action\":\"system.admin\",\"resource\":\"raw\"}]}");
    expected_resource = "raw"; TEST_ASSERT_EQUAL_INT(1, openapi_snapshot_access(snapshot,input)); TEST_ASSERT_EQUAL_INT(1,global_calls);
    TEST_ASSERT_EQUAL_INT(-1, openapi_snapshot_access(NULL,input));
}
static void *fault_malloc(size_t size) { if (allocation_limit == 0) return NULL; if (allocation_limit > 0) allocation_limit--; return malloc(size); }
static void test_allocation_failures_cleanup(void) {
    parse(fixture); cJSON_Hooks hooks = {fault_malloc, free};
    int success = 0;
    for (int limit = 0; limit < 4000; limit++) {
        allocation_limit = limit; cJSON_InitHooks(&hooks);
        int rc = openapi_project(input,access_rule,NULL,&output);
        allocation_limit = -1; cJSON_InitHooks(NULL);
        if (rc == 0) { success = 1; cJSON_Delete(output); output = NULL; break; }
        TEST_ASSERT_NULL(output);
    }
    TEST_ASSERT_TRUE(success);
}
static void test_non_scoped_identity_adapter(void) {
    parse("{\"version\":1,\"kind\":\"non_scoped_identity\"}");
    authorization_snapshot_t *snapshot = (authorization_snapshot_t *)&mode;
    TEST_ASSERT_EQUAL_INT(1, openapi_snapshot_access(snapshot, input));
    non_scoped_identity = false;
    TEST_ASSERT_EQUAL_INT(0, openapi_snapshot_access(snapshot, input));
    TEST_ASSERT_EQUAL_INT(-1, openapi_snapshot_access(NULL, input));
    TEST_ASSERT_EQUAL_INT(0, global_calls);
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_canonical_admin_immutable); RUN_TEST(test_viewer_prunes_variants); RUN_TEST(test_empty_valid);
    RUN_TEST(test_non_scoped_identity_adapter); RUN_TEST(test_callback_failure); RUN_TEST(test_bad_predicates_even_hidden);
    RUN_TEST(test_cycles_pointers_literals_links_discriminator); RUN_TEST(test_missing_external_and_bad_escaped_refs);
    RUN_TEST(test_empty_union_fails); RUN_TEST(test_misplaced_or_duplicate_metadata);
    RUN_TEST(test_named_encoding_and_server_variable_maps); RUN_TEST(test_definitions_preserve_referenced_components);
    RUN_TEST(test_link_parameters_and_body_are_literal_payloads);
    RUN_TEST(test_root_path_and_annotation_metadata_rejected); RUN_TEST(test_unsupported_schema_keywords_and_dialects_rejected);
    RUN_TEST(test_same_camera_and_resource_adapter); RUN_TEST(test_allocation_failures_cleanup);
    return UNITY_END();
}
