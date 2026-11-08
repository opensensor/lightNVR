/**
 * @file test_api_handlers_observations.c
 * @brief INT_01 observation API handlers (contract
 *        docs/integrations/OBSERVATIONS_CONTRACT.md sections 7-9, 11, 12).
 *
 * Runs the GET handlers and the P2 classification PUT (L14) against a real
 * migrated database populated through the L2/L13 DB layer. Authentication, the per-camera authorization decision,
 * list filtering, file serving and audit appends are wrapped so each test
 * controls the principal's camera grants and observes what was served or
 * audited. Response bodies are compared with the shared C3 fixtures in
 * tests/fixtures/integrations/observations.
 *
 * Set OBS_RESPONSE_DUMP_DIR to also write every JSON body the handlers
 * produce (for an ad-hoc check against web/generated/api/validators.js).
 */

#define _GNU_SOURCE

#include <errno.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <time.h>
#include <unistd.h>

#include <cjson/cJSON.h>

#include "unity.h"
#include "core/authorization.h"
#include "core/config.h"
#include "core/event_identity.h"
#include "database/db_auth.h"
#include "core/event_envelope.h"
#include "core/observation_relay.h"
#include "database/db_core.h"
#include "database/db_observation_classifications.h"
#include "database/db_observations.h"
#include "database/db_streams.h"
#include "utils/strings.h"
#include "utils/uuid.h"
#include "web/api_handlers_observations.h"
#include "web/request_response.h"

#ifndef OBS_FIXTURE_DIR
#error "OBS_FIXTURE_DIR must point at tests/fixtures/integrations/observations"
#endif

/* ---- wrapped collaborators ------------------------------------------- */

typedef struct {
    char camera[OBS_UUID_SIZE];
    bool live_view;
    bool snapshot;
    bool annotate;
} grant_t;

static bool authenticated = true;
static char principal_name[64] = "YaWamf";
static int principal_id = 42;
static grant_t grants[4];
static int grant_count;
static bool evaluation_fails;

static int serve_calls;
static char served_path[MAX_PATH_LENGTH];
static char served_type[64];
static char served_headers[512];

#define MAX_AUDITS 256
typedef struct {
    char action[64];
    char target_type[32];
    char target_uuid[OBS_UUID_SIZE];
    char outcome[16];
    char details[1024];
    char operation[48];
} audit_record_t;
static audit_record_t audits[MAX_AUDITS];
static int audit_count;

int __wrap_httpd_check_action_access(const http_request_t *req, user_t *user) {
    (void)req;
    if (user) {
        memset(user, 0, sizeof(*user));
        user->id = principal_id;
        safe_strcpy(user->username, principal_name, sizeof(user->username), 0);
    }
    return authenticated ? 1 : 0;
}

static const grant_t *find_grant(const char *camera) {
    for (int i = 0; camera && i < grant_count; i++)
        if (strcmp(grants[i].camera, camera) == 0) return &grants[i];
    return NULL;
}

static bool granted(const char *camera, authorization_action_t action) {
    const grant_t *grant = find_grant(camera);
    if (!grant) return false;
    if (action == AUTHZ_LIVE_VIEW) return grant->live_view;
    if (action == AUTHZ_SNAPSHOT_CREATE) return grant->snapshot;
    if (action == AUTHZ_OBSERVATIONS_ANNOTATE) return grant->annotate;
    return false;
}

int __wrap_httpd_evaluate_camera_identity_action(const user_t *user,
    authorization_action_t action, const char *camera_uuid,
    const char *legacy_stream_name, authorization_evaluation_t *evaluation) {
    (void)user; (void)legacy_stream_name;
    memset(evaluation, 0, sizeof(*evaluation));
    if (evaluation_fails) return -1;
    evaluation->decision = granted(camera_uuid, action) ? AUTHZ_DECISION_ALLOW
                                                        : AUTHZ_DECISION_DENY;
    return 0;
}

int __wrap_authorization_filter_cameras(const user_t *user, authorization_action_t action,
                                        fleet_camera_t *cameras, int *count) {
    (void)user;
    if (evaluation_fails) return -1;
    int kept = 0;
    for (int i = 0; i < *count; i++)
        if (granted(cameras[i].camera_uuid, action)) cameras[kept++] = cameras[i];
    *count = kept;
    return 0;
}

int __wrap_http_serve_file(const http_request_t *req, const http_response_t *res,
                           const char *file_path, const char *content_type,
                           const char *extra_headers) {
    (void)req; (void)res;
    serve_calls++;
    safe_strcpy(served_path, file_path, sizeof(served_path), 0);
    safe_strcpy(served_type, content_type ? content_type : "", sizeof(served_type), 0);
    safe_strcpy(served_headers, extra_headers ? extra_headers : "", sizeof(served_headers), 0);
    return 0;
}

void __wrap_audit_log_append(const http_request_t *req, const user_t *user,
                             const char *action, const char *target_type,
                             const char *target_uuid, const char *outcome,
                             const cJSON *details) {
    (void)req; (void)user;
    if (audit_count >= MAX_AUDITS) return;
    audit_record_t *record = &audits[audit_count++];
    memset(record, 0, sizeof(*record));
    safe_strcpy(record->action, action ? action : "", sizeof(record->action), 0);
    safe_strcpy(record->target_type, target_type ? target_type : "", sizeof(record->target_type), 0);
    safe_strcpy(record->target_uuid, target_uuid ? target_uuid : "", sizeof(record->target_uuid), 0);
    safe_strcpy(record->outcome, outcome ? outcome : "", sizeof(record->outcome), 0);
    char *text = details ? cJSON_PrintUnformatted(details) : NULL;
    if (text) safe_strcpy(record->details, text, sizeof(record->details), 0);
    cJSON_free(text);
}

/* Calls from the handler file are intercepted; the context is kept as the
 * record's details so tests can read the section 12.2 fields. */
void __wrap_audit_log_operation(const http_request_t *req, const user_t *user,
                                const char *action, const char *target_type,
                                const char *target_uuid, const char *operation,
                                const char *outcome, const cJSON *context) {
    int before = audit_count;
    __wrap_audit_log_append(req, user, action, target_type, target_uuid, outcome, context);
    if (audit_count > before)
        safe_strcpy(audits[before].operation, operation ? operation : "",
                    sizeof(audits[before].operation), 0);
}

/* ---- fixture and request helpers -------------------------------------- */

static char db_path[256];
static char storage_dir[256];
static char source[128];
static char cam_a[OBS_UUID_SIZE];
static char cam_b[OBS_UUID_SIZE];
static char cam_c[OBS_UUID_SIZE];

static cJSON *load_fixture(const char *relative) {
    char path[512];
    snprintf(path, sizeof(path), "%s/%s", OBS_FIXTURE_DIR, relative);
    FILE *file = fopen(path, "rb");
    TEST_ASSERT_NOT_NULL_MESSAGE(file, path);
    fseek(file, 0, SEEK_END);
    long size = ftell(file);
    fseek(file, 0, SEEK_SET);
    char *text = calloc(1, (size_t)size + 1);
    TEST_ASSERT_NOT_NULL(text);
    TEST_ASSERT_EQUAL_size_t((size_t)size, fread(text, 1, (size_t)size, file));
    fclose(file);
    cJSON *json = cJSON_Parse(text);
    free(text);
    TEST_ASSERT_NOT_NULL_MESSAGE(json, path);
    return json;
}

static void assert_json_equal(const cJSON *expected, const cJSON *actual, const char *what) {
    if (!cJSON_Compare(expected, actual, true)) {
        char *a = cJSON_Print(expected);
        char *b = cJSON_Print(actual);
        fprintf(stderr, "%s mismatch\nexpected: %s\nactual:   %s\n", what, a, b);
        cJSON_free(a);
        cJSON_free(b);
        TEST_FAIL_MESSAGE(what);
    }
}

/* Same keys (recursively, for objects) and compatible JSON types; nullable
 * fields may be null on either side. Detects fields the handler invents or
 * drops relative to a fixture. */
static void assert_same_shape(const cJSON *fixture, const cJSON *actual, const char *where) {
    char message[256];
    if (cJSON_IsNull(fixture) || cJSON_IsNull(actual)) return;
    snprintf(message, sizeof(message), "type at %s", where);
    if (cJSON_IsBool(fixture)) TEST_ASSERT_TRUE_MESSAGE(cJSON_IsBool(actual), message);
    else TEST_ASSERT_EQUAL_INT_MESSAGE(fixture->type & 0xFF, actual->type & 0xFF, message);
    if (!cJSON_IsObject(fixture)) return;
    const cJSON *item;
    cJSON_ArrayForEach(item, fixture) {
        snprintf(message, sizeof(message), "missing %s.%s", where, item->string);
        const cJSON *other = cJSON_GetObjectItemCaseSensitive(actual, item->string);
        TEST_ASSERT_NOT_NULL_MESSAGE(other, message);
        char child[200];
        snprintf(child, sizeof(child), "%s.%s", where, item->string);
        assert_same_shape(item, other, child);
    }
    cJSON_ArrayForEach(item, actual) {
        snprintf(message, sizeof(message), "unexpected %s.%s", where, item->string);
        TEST_ASSERT_NOT_NULL_MESSAGE(cJSON_GetObjectItemCaseSensitive(fixture, item->string),
                                     message);
    }
}

typedef void (*handler_fn)(const http_request_t *, http_response_t *);

typedef struct {
    http_response_t response;
    cJSON *json;
} result_t;

static int dump_seq;

static result_t call_with_body(handler_fn handler, http_method_t method, const char *path,
                               const char *query, const char *if_none_match,
                               const char *body) {
    http_request_t request;
    result_t result;
    memset(&result, 0, sizeof(result));
    http_request_init(&request);
    http_response_init(&result.response);
    request.method = method;
    safe_strcpy(request.method_str, method == HTTP_METHOD_PUT ? "PUT" : "GET",
                sizeof(request.method_str), 0);
    safe_strcpy(request.path, path, sizeof(request.path), 0);
    if (query) safe_strcpy(request.query_string, query, sizeof(request.query_string), 0);
    if (if_none_match) {
        safe_strcpy(request.headers[0].name, "If-None-Match", sizeof(request.headers[0].name), 0);
        safe_strcpy(request.headers[0].value, if_none_match, sizeof(request.headers[0].value), 0);
        request.num_headers = 1;
    }
    if (body) {
        request.body = (void *)body;
        request.body_len = strlen(body);
    }
    handler(&request, &result.response);
    if (result.response.body && result.response.body_length > 0) {
        result.json = cJSON_ParseWithLength((const char *)result.response.body,
                                            result.response.body_length);
        const char *dump = getenv("OBS_RESPONSE_DUMP_DIR");
        if (dump && result.json) {
            char file[512];
            snprintf(file, sizeof(file), "%s/%03d-%d.json", dump, ++dump_seq,
                     result.response.status_code);
            FILE *out = fopen(file, "w");
            if (out) {
                fwrite(result.response.body, 1, result.response.body_length, out);
                fclose(out);
            }
        }
    }
    return result;
}

static result_t call(handler_fn handler, http_method_t method, const char *path,
                     const char *query, const char *if_none_match) {
    return call_with_body(handler, method, path, query, if_none_match, NULL);
}

static void release(result_t *result) {
    cJSON_Delete(result->json);
    http_response_free(&result->response);
}

static const char *header(const http_response_t *response, const char *name) {
    for (int i = 0; i < response->num_headers; i++)
        if (strcasecmp(response->headers[i].name, name) == 0) return response->headers[i].value;
    return NULL;
}

static const char *str(const cJSON *json, const char *key) {
    const cJSON *item = cJSON_GetObjectItemCaseSensitive(json, key);
    return cJSON_IsString(item) ? item->valuestring : NULL;
}

static void expect_error(result_t *result, int status, const char *code) {
    char message[128];
    snprintf(message, sizeof(message), "status for %s", code ? code : "(none)");
    TEST_ASSERT_EQUAL_INT_MESSAGE(status, result->response.status_code, message);
    TEST_ASSERT_NOT_NULL(result->json);
    TEST_ASSERT_NOT_NULL(str(result->json, "error"));
    if (code) TEST_ASSERT_EQUAL_STRING(code, str(result->json, "code"));
}

static void make_camera(const char *name, char out[OBS_UUID_SIZE]) {
    stream_config_t *s = calloc(1, sizeof(*s));
    TEST_ASSERT_NOT_NULL(s);
    safe_strcpy(s->name, name, sizeof(s->name), 0);
    safe_strcpy(s->url, "rtsp://camera/stream", sizeof(s->url), 0);
    safe_strcpy(s->codec, "h264", sizeof(s->codec), 0);
    s->enabled = true;
    s->priority = 5;
    s->segment_duration = 60;
    s->observation_evidence_enabled = strcmp(name, "feeder-east") == 0;
    safe_strcpy(s->detection_object_filter, "none", sizeof(s->detection_object_filter), 0);
    TEST_ASSERT_GREATER_THAN(0, add_stream_config(s));
    memset(s, 0, sizeof(*s));
    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_name(name, s));
    safe_strcpy(out, s->camera_uuid, OBS_UUID_SIZE, 0);
    free(s);
}

static int64_t base_time_ms = 1793000000000LL;

typedef struct {
    char observation[OBS_UUID_SIZE];
    char asset[OBS_UUID_SIZE];
    char relative_path[MAX_PATH_LENGTH];
} seeded_t;

static seeded_t seed(const char *camera, const char *label, observation_asset_state_t state) {
    observation_batch_input_t batch;
    memset(&batch, 0, sizeof(batch));
    TEST_ASSERT_EQUAL_INT(0, lightnvr_uuid_generate_v4(batch.batch_id));
    safe_strcpy(batch.camera_uuid, camera, sizeof(batch.camera_uuid), 0);
    batch.observed_at_ms = base_time_ms++;
    safe_strcpy(batch.engine_key, "legacy-primary", sizeof(batch.engine_key), 0);
    safe_strcpy(batch.engine_type, "object", sizeof(batch.engine_type), 0);
    safe_strcpy(batch.model, "ssd_mobilenet_v2.sod", sizeof(batch.model), 0);
    batch.recording_id = 0;
    batch.object_count = 1;
    observation_object_input_t *object = &batch.objects[0];
    safe_strcpy(object->label, label, sizeof(object->label), 0);
    object->confidence = 0.87;
    object->has_box = true;
    object->box_x = 0.412; object->box_y = 0.288;
    object->box_width = 0.12; object->box_height = 0.165;

    observation_asset_input_t asset;
    memset(&asset, 0, sizeof(asset));
    if (state == OBS_ASSET_UNAVAILABLE) {
        asset.state = OBS_ASSET_UNAVAILABLE;
        safe_strcpy(asset.reason, "queue_full", sizeof(asset.reason), 0);
    } else {
        asset.state = OBS_ASSET_PENDING;
        safe_strcpy(asset.origin, "inference_input", sizeof(asset.origin), 0);
        safe_strcpy(asset.media_type, "image/jpeg", sizeof(asset.media_type), 0);
        safe_strcpy(asset.transform_json,
                    "{\"mode\":\"none\",\"source_width\":640,\"source_height\":360,"
                    "\"inference_width\":640,\"inference_height\":360,\"scale_x\":1,"
                    "\"scale_y\":1,\"pad_x\":0,\"pad_y\":0}",
                    sizeof(asset.transform_json), 0);
    }
    observation_insert_result_t inserted;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_insert_batch(&batch, &asset, &inserted));
    seeded_t out;
    memset(&out, 0, sizeof(out));
    safe_strcpy(out.observation, inserted.observation_uuids[0], sizeof(out.observation), 0);
    safe_strcpy(out.asset, inserted.asset_uuid, sizeof(out.asset), 0);
    if (state == OBS_ASSET_READY || state == OBS_ASSET_EXPIRED) {
        char dir[512], file[1024];
        snprintf(out.relative_path, sizeof(out.relative_path), "observations/%s/2026-11-08/%s.jpg",
                 camera, out.asset);
        snprintf(dir, sizeof(dir), "%s/observations", storage_dir);
        mkdir(dir, 0700);
        snprintf(dir, sizeof(dir), "%s/observations/%s", storage_dir, camera);
        mkdir(dir, 0700);
        snprintf(dir, sizeof(dir), "%s/observations/%s/2026-11-08", storage_dir, camera);
        mkdir(dir, 0700);
        snprintf(file, sizeof(file), "%s/%s", storage_dir, out.relative_path);
        FILE *jpeg = fopen(file, "wb");
        TEST_ASSERT_NOT_NULL(jpeg);
        fputs("\xff\xd8 not really a jpeg \xff\xd9", jpeg);
        fclose(jpeg);
        observation_asset_media_t media = {
            .path = out.relative_path, .bytes = 23, .width = 640, .height = 360,
            .expires_at_ms = base_time_ms + 7LL * 86400000LL,
        };
        TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_asset_transition(
            out.asset, OBS_ASSET_PENDING, OBS_ASSET_READY, NULL, &media, NULL));
    }
    if (state == OBS_ASSET_EXPIRED) {
        TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_asset_transition(
            out.asset, OBS_ASSET_READY, OBS_ASSET_EXPIRED, "retention_expired", NULL, NULL));
    }
    return out;
}

static void grant_all(const char *camera, bool live_view, bool snapshot, bool annotate) {
    TEST_ASSERT_LESS_THAN(4, grant_count);
    safe_strcpy(grants[grant_count].camera, camera, OBS_UUID_SIZE, 0);
    grants[grant_count].live_view = live_view;
    grants[grant_count].snapshot = snapshot;
    grants[grant_count].annotate = annotate;
    grant_count++;
}

static void grant(const char *camera, bool live_view, bool snapshot) {
    grant_all(camera, live_view, snapshot, false);
}

static void obs_path(char *out, size_t size, const char *id, const char *suffix) {
    snprintf(out, size, "/api/observations/%s%s", id, suffix ? suffix : "");
}

/* ---- setup ------------------------------------------------------------ */

void setUp(void) {
    authenticated = true;
    safe_strcpy(principal_name, "YaWamf", sizeof(principal_name), 0);
    principal_id = 42;
    grant_count = 0;
    memset(grants, 0, sizeof(grants));
    evaluation_fails = false;
    serve_calls = 0;
    served_path[0] = served_type[0] = served_headers[0] = '\0';
    audit_count = 0;
    sqlite3_exec(get_db_handle(),
                 "DELETE FROM object_observations; DELETE FROM observation_assets;",
                 NULL, NULL, NULL);
}

void tearDown(void) {}

/* ---- authentication ---------------------------------------------------- */

static void test_every_route_requires_authentication(void) {
    authenticated = false;
    handler_fn handlers[] = {handle_get_integration_capabilities, handle_get_observations,
                             handle_get_observation, handle_get_observation_snapshot,
                             handle_put_observation_classification};
    const char *paths[] = {"/api/integrations/capabilities", "/api/observations",
                           "/api/observations/55555555-5555-4555-8555-555555555555",
                           "/api/observations/55555555-5555-4555-8555-555555555555/snapshot",
                           "/api/observations/55555555-5555-4555-8555-555555555555/"
                           "classifications/yawamf/species"};
    for (size_t i = 0; i < sizeof(handlers) / sizeof(handlers[0]); i++) {
        result_t result = call(handlers[i], i == 4 ? HTTP_METHOD_PUT : HTTP_METHOD_GET,
                               paths[i], NULL, NULL);
        expect_error(&result, 401, NULL);
        release(&result);
    }
}

/* ---- serialization conformance with the C3 fixtures -------------------- */

static void time_ms(const char *text, int64_t *ms) {
    TEST_ASSERT_EQUAL_INT_MESSAGE(0, observation_parse_time(text, ms), text);
}

static void copy_str(char *out, size_t size, const cJSON *json, const char *key) {
    const char *value = str(json, key);
    safe_strcpy(out, value ? value : "", size, 0);
}

/* Build the DB record the fixture represents, as db_observation_get would
 * return it (including a path, which must never be serialized). */
static void record_from_fixture(const cJSON *fixture, observation_record_t *record) {
    memset(record, 0, sizeof(*record));
    copy_str(record->uuid, sizeof(record->uuid), fixture, "id");
    copy_str(record->camera_uuid, sizeof(record->camera_uuid), fixture, "camera_uuid");
    copy_str(record->stream_name_at_capture, sizeof(record->stream_name_at_capture), fixture,
             "stream_name_at_capture");
    copy_str(record->batch_id, sizeof(record->batch_id), fixture, "batch_id");
    record->object_index = cJSON_GetObjectItem(fixture, "object_index")->valueint;
    time_ms(str(fixture, "observed_at"), &record->observed_at_ms);
    time_ms(str(fixture, "created_at"), &record->created_at_ms);
    const cJSON *detector = cJSON_GetObjectItem(fixture, "detector");
    copy_str(record->engine_key, sizeof(record->engine_key), detector, "engine_key");
    copy_str(record->engine_type, sizeof(record->engine_type), detector, "engine_type");
    copy_str(record->model, sizeof(record->model), detector, "model");
    copy_str(record->model_version, sizeof(record->model_version), detector, "model_version");
    copy_str(record->label, sizeof(record->label), fixture, "label");
    record->confidence = cJSON_GetObjectItem(fixture, "confidence")->valuedouble;
    const cJSON *box = cJSON_GetObjectItem(fixture, "box");
    record->has_box = cJSON_IsObject(box);
    if (record->has_box) {
        record->box_x = cJSON_GetObjectItem(box, "x")->valuedouble;
        record->box_y = cJSON_GetObjectItem(box, "y")->valuedouble;
        record->box_width = cJSON_GetObjectItem(box, "width")->valuedouble;
        record->box_height = cJSON_GetObjectItem(box, "height")->valuedouble;
    }
    const cJSON *track = cJSON_GetObjectItem(fixture, "track");
    record->has_track = cJSON_IsObject(track);
    if (record->has_track) {
        copy_str(record->track_id, sizeof(record->track_id), track, "id");
        copy_str(record->track_namespace, sizeof(record->track_namespace), track, "namespace");
        copy_str(record->track_session, sizeof(record->track_session), track, "session");
    }
    const cJSON *recording = cJSON_GetObjectItem(fixture, "recording_reference");
    if (cJSON_IsObject(recording))
        record->recording_id = (int64_t)cJSON_GetObjectItem(recording, "recording_id")->valuedouble;
    record->revision = cJSON_GetObjectItem(fixture, "revision")->valueint;

    const cJSON *evidence = cJSON_GetObjectItem(fixture, "evidence");
    observation_asset_record_t *asset = &record->asset;
    copy_str(asset->uuid, sizeof(asset->uuid), evidence, "asset_id");
    safe_strcpy(asset->camera_uuid, record->camera_uuid, sizeof(asset->camera_uuid), 0);
    TEST_ASSERT_EQUAL_INT(0, observation_asset_state_parse(str(evidence, "state"), &asset->state));
    copy_str(asset->reason, sizeof(asset->reason), evidence, "reason");
    copy_str(asset->origin, sizeof(asset->origin), evidence, "origin");
    copy_str(asset->media_type, sizeof(asset->media_type), evidence, "media_type");
    safe_strcpy(asset->path, "observations/secret/never-serialized.jpg", sizeof(asset->path), 0);
    const cJSON *bytes = cJSON_GetObjectItem(evidence, "bytes");
    asset->has_bytes = cJSON_IsNumber(bytes);
    if (asset->has_bytes) asset->bytes = (int64_t)bytes->valuedouble;
    const cJSON *width = cJSON_GetObjectItem(evidence, "width");
    const cJSON *height = cJSON_GetObjectItem(evidence, "height");
    if (cJSON_IsNumber(width)) asset->width = width->valueint;
    if (cJSON_IsNumber(height)) asset->height = height->valueint;
    if (str(evidence, "captured_at")) time_ms(str(evidence, "captured_at"), &asset->captured_at_ms);
    else asset->captured_at_ms = record->observed_at_ms;  /* L2 defaults it; must stay hidden */
    if (str(evidence, "expires_at")) time_ms(str(evidence, "expires_at"), &asset->expires_at_ms);
    const cJSON *transform = cJSON_GetObjectItem(evidence, "transform");
    if (cJSON_IsObject(transform)) {
        char *text = cJSON_PrintUnformatted(transform);
        safe_strcpy(asset->transform_json, text, sizeof(asset->transform_json), 0);
        cJSON_free(text);
    }
    asset->created_at_ms = record->created_at_ms;
    asset->updated_at_ms = record->created_at_ms;
}

/* The DB row an annotation object (contract section 10.3) represents. */
static void classification_from_json(const cJSON *json, observation_classification_t *out) {
    memset(out, 0, sizeof(*out));
    copy_str(out->key, sizeof(out->key), json, "key");
    copy_str(out->writer_namespace, sizeof(out->writer_namespace), json, "writer_namespace");
    out->writer_user_id = 42;
    copy_str(out->classifier, sizeof(out->classifier), json, "classifier");
    copy_str(out->model_version, sizeof(out->model_version), json, "model_version");
    copy_str(out->label, sizeof(out->label), json, "label");
    copy_str(out->taxonomy_id, sizeof(out->taxonomy_id), json, "taxonomy_id");
    out->confidence = cJSON_GetObjectItem(json, "confidence")->valuedouble;
    time_ms(str(json, "classified_at"), &out->classified_at_ms);
    copy_str(out->input_asset_uuid, sizeof(out->input_asset_uuid), json, "input_asset_id");
    out->input_revision = cJSON_GetObjectItem(json, "input_revision")->valueint;
    TEST_ASSERT_EQUAL_INT(0, observation_classification_kind_parse(str(json, "kind"), &out->kind));
    out->annotation_revision = cJSON_GetObjectItem(json, "annotation_revision")->valueint;
    time_ms(str(json, "created_at"), &out->created_at_ms);
    time_ms(str(json, "updated_at"), &out->updated_at_ms);
}

static void set_from_fixture(const cJSON *fixture, observation_classification_set_t *set) {
    memset(set, 0, sizeof(*set));
    const cJSON *lists = cJSON_GetObjectItem(fixture, "classifications");
    const cJSON *item;
    cJSON_ArrayForEach(item, cJSON_GetObjectItem(lists, "automated"))
        classification_from_json(item, &set->automated[set->automated_count++]);
    cJSON_ArrayForEach(item, cJSON_GetObjectItem(lists, "owner_confirmed"))
        classification_from_json(item, &set->owner_confirmed[set->owner_confirmed_count++]);
}

static void test_serializer_reproduces_every_observation_fixture(void) {
    const char *files[] = {
        "observations/single-pending.json", "observations/single-ready.json",
        "observations/single-unavailable.json", "observations/single-expired.json",
        "observations/two-birds-object-0.json", "observations/two-birds-object-1.json",
        "observations/http-letterbox.json", "observations/onvif-no-matching-media.json",
        "observations/single-ready-classified.json", "observations/single-owner-confirmed.json",
    };
    for (size_t i = 0; i < sizeof(files) / sizeof(files[0]); i++) {
        cJSON *fixture = load_fixture(files[i]);
        observation_record_t record;
        record_from_fixture(fixture, &record);
        observation_classification_set_t *set = calloc(1, sizeof(*set));
        TEST_ASSERT_NOT_NULL(set);
        set_from_fixture(fixture, set);
        cJSON *actual = observation_record_to_json(&record, str(fixture, "source"), set);
        free(set);
        TEST_ASSERT_NOT_NULL(actual);
        char *text = cJSON_PrintUnformatted(actual);
        TEST_ASSERT_NULL_MESSAGE(strstr(text, "never-serialized"), "asset path leaked");
        TEST_ASSERT_NULL_MESSAGE(strstr(text, "\"path\""), "path key leaked");
        TEST_ASSERT_NULL_MESSAGE(strstr(text, "writer_user_id"), "writer user id leaked");
        cJSON_free(text);
        assert_json_equal(fixture, actual, files[i]);
        cJSON_Delete(actual);
        cJSON_Delete(fixture);
    }
    /* NULL classifications: both lists present and empty. */
    cJSON *fixture = load_fixture("observations/single-ready.json");
    observation_record_t record;
    record_from_fixture(fixture, &record);
    cJSON *actual = observation_record_to_json(&record, str(fixture, "source"), NULL);
    assert_json_equal(fixture, actual, "single-ready without annotations");
    cJSON_Delete(actual);
    cJSON_Delete(fixture);
}

static void test_time_and_namespace_helpers(void) {
    char text[OBS_TIME_TEXT_SIZE];
    int64_t ms = 0;
    TEST_ASSERT_EQUAL_INT(0, observation_parse_time("2026-11-08T14:03:12.481Z", &ms));
    TEST_ASSERT_EQUAL_INT(0, observation_format_time(ms, text, sizeof(text)));
    TEST_ASSERT_EQUAL_STRING("2026-11-08T14:03:12.481Z", text);
    int64_t offset = 0;
    TEST_ASSERT_EQUAL_INT(0, observation_parse_time("2026-11-08T16:03:12.481+02:00", &offset));
    TEST_ASSERT_EQUAL_INT64(ms, offset);
    TEST_ASSERT_EQUAL_INT(0, observation_parse_time("2026-11-08T14:03:12Z", &offset));
    TEST_ASSERT_EQUAL_INT64(ms - 481, offset);
    TEST_ASSERT_NOT_EQUAL(0, observation_parse_time("2026-11-08 14:03:12Z", &offset));
    TEST_ASSERT_NOT_EQUAL(0, observation_parse_time("2026-13-08T14:03:12Z", &offset));
    TEST_ASSERT_NOT_EQUAL(0, observation_parse_time("2026-11-08T14:03:12", &offset));
    TEST_ASSERT_NOT_EQUAL(0, observation_parse_time("yesterday", &offset));

    char ns[OBS_WRITER_NAMESPACE_SIZE];
    TEST_ASSERT_EQUAL_INT(0, observation_writer_namespace("YaWamf", ns));
    TEST_ASSERT_EQUAL_STRING("yawamf", ns);
    TEST_ASSERT_EQUAL_INT(0, observation_writer_namespace("Bird Watcher@home", ns));
    TEST_ASSERT_EQUAL_STRING("bird-watcher-home", ns);
    TEST_ASSERT_EQUAL_INT(-2, observation_writer_namespace("System", ns));
    TEST_ASSERT_EQUAL_STRING("", ns);
    TEST_ASSERT_EQUAL_INT(-1, observation_writer_namespace("-leading", ns));
    TEST_ASSERT_EQUAL_INT(-1, observation_writer_namespace(
        "a-username-that-is-much-longer-than-32", ns));
    TEST_ASSERT_EQUAL_INT(-1, observation_writer_namespace("", ns));
}

/* ---- GET /api/observations/{id} --------------------------------------- */

static void test_get_returns_db_rows_in_fixture_shape(void) {
    grant(cam_a, true, false);
    struct { observation_asset_state_t state; const char *fixture; } cases[] = {
        {OBS_ASSET_PENDING, "observations/single-pending.json"},
        {OBS_ASSET_READY, "observations/single-ready.json"},
        {OBS_ASSET_UNAVAILABLE, "observations/single-unavailable.json"},
        {OBS_ASSET_EXPIRED, "observations/single-expired.json"},
    };
    for (size_t i = 0; i < sizeof(cases) / sizeof(cases[0]); i++) {
        seeded_t seeded = seed(cam_a, "bird", cases[i].state);
        char path[128];
        obs_path(path, sizeof(path), seeded.observation, NULL);
        result_t result = call(handle_get_observation, HTTP_METHOD_GET, path, NULL, NULL);
        TEST_ASSERT_EQUAL_INT(200, result.response.status_code);
        TEST_ASSERT_NOT_NULL(result.json);
        TEST_ASSERT_NULL(strstr((const char *)result.response.body, seeded.relative_path[0]
                                ? seeded.relative_path : "\"path\""));
        cJSON *fixture = load_fixture(cases[i].fixture);
        cJSON_ReplaceItemInObject(cJSON_GetObjectItem(fixture, "classifications"),
                                  "owner_confirmed", cJSON_CreateArray());
        /* recording_reference is null in the DB rows (no recording). */
        cJSON_ReplaceItemInObject(fixture, "recording_reference", cJSON_CreateNull());
        assert_same_shape(fixture, result.json, cases[i].fixture);
        TEST_ASSERT_EQUAL_STRING(seeded.observation, str(result.json, "id"));
        TEST_ASSERT_EQUAL_STRING(source, str(result.json, "source"));
        TEST_ASSERT_EQUAL_STRING(cam_a, str(result.json, "camera_uuid"));
        const cJSON *evidence = cJSON_GetObjectItem(result.json, "evidence");
        TEST_ASSERT_EQUAL_STRING(seeded.asset, str(evidence, "asset_id"));
        if (cases[i].state == OBS_ASSET_UNAVAILABLE) {
            /* Never had media: every media field null, even captured_at. */
            TEST_ASSERT_TRUE(cJSON_IsNull(cJSON_GetObjectItem(evidence, "captured_at")));
            TEST_ASSERT_TRUE(cJSON_IsNull(cJSON_GetObjectItem(evidence, "origin")));
        }
        cJSON_Delete(fixture);
        release(&result);
    }
}

static void test_get_unknown_unauthorized_and_malformed(void) {
    seeded_t other = seed(cam_b, "bird", OBS_ASSET_READY);
    grant(cam_a, true, true);
    cJSON *not_found = load_fixture("errors/observation-not-found.json");

    char path[128];
    obs_path(path, sizeof(path), other.observation, NULL);
    result_t unauthorized = call(handle_get_observation, HTTP_METHOD_GET, path, NULL, NULL);
    TEST_ASSERT_EQUAL_INT(404, unauthorized.response.status_code);
    assert_json_equal(not_found, unauthorized.json, "unauthorized get body");

    result_t unknown = call(handle_get_observation, HTTP_METHOD_GET,
                            "/api/observations/00000000-0000-4000-8000-000000000000", NULL, NULL);
    TEST_ASSERT_EQUAL_INT(404, unknown.response.status_code);
    /* Unknown and unauthorized are byte-identical. */
    TEST_ASSERT_EQUAL_size_t(unknown.response.body_length, unauthorized.response.body_length);
    TEST_ASSERT_EQUAL_MEMORY(unknown.response.body, unauthorized.response.body,
                             unknown.response.body_length);

    result_t malformed = call(handle_get_observation, HTTP_METHOD_GET,
                              "/api/observations/not-a-uuid", NULL, NULL);
    expect_error(&malformed, 400, "invalid_request");

    evaluation_fails = true;
    result_t failure = call(handle_get_observation, HTTP_METHOD_GET, path, NULL, NULL);
    expect_error(&failure, 500, "internal_error");

    cJSON_Delete(not_found);
    release(&unauthorized);
    release(&unknown);
    release(&malformed);
    release(&failure);
}

/* ---- GET /api/observations ------------------------------------------- */

static int item_count(const result_t *result) {
    return cJSON_GetArraySize(cJSON_GetObjectItem(result->json, "items"));
}

static bool page_contains(const result_t *result, const char *id) {
    const cJSON *item;
    cJSON_ArrayForEach(item, cJSON_GetObjectItem(result->json, "items"))
        if (strcmp(str(item, "id"), id) == 0) return true;
    return false;
}

static void test_list_intersects_with_authorized_cameras(void) {
    seeded_t a = seed(cam_a, "bird", OBS_ASSET_PENDING);
    seeded_t b = seed(cam_b, "bird", OBS_ASSET_PENDING);
    seeded_t c = seed(cam_c, "squirrel", OBS_ASSET_UNAVAILABLE);
    grant(cam_a, true, false);
    grant(cam_c, true, false);
    grant(cam_b, false, true);  /* snapshot without live.view: no metadata */

    result_t all = call(handle_get_observations, HTTP_METHOD_GET, "/api/observations", NULL, NULL);
    TEST_ASSERT_EQUAL_INT(200, all.response.status_code);
    TEST_ASSERT_EQUAL_INT(2, item_count(&all));
    TEST_ASSERT_TRUE(page_contains(&all, a.observation));
    TEST_ASSERT_TRUE(page_contains(&all, c.observation));
    TEST_ASSERT_FALSE(page_contains(&all, b.observation));
    TEST_ASSERT_NOT_NULL(str(all.json, "next_cursor"));
    TEST_ASSERT_TRUE(cJSON_IsFalse(cJSON_GetObjectItem(all.json, "has_more")));
    cJSON *page_fixture = load_fixture("pages/page-1.json");
    cJSON *fixture_items = cJSON_DetachItemFromObject(page_fixture, "items");
    cJSON *all_items = cJSON_DetachItemFromObject(all.json, "items");
    assert_same_shape(page_fixture, all.json, "page");
    cJSON_AddItemToObject(all.json, "items", all_items);
    cJSON_Delete(fixture_items);
    cJSON_Delete(page_fixture);

    char query[256];
    snprintf(query, sizeof(query), "camera_uuid=%s", cam_c);
    result_t filtered = call(handle_get_observations, HTTP_METHOD_GET, "/api/observations",
                             query, NULL);
    TEST_ASSERT_EQUAL_INT(200, filtered.response.status_code);
    TEST_ASSERT_EQUAL_INT(1, item_count(&filtered));
    TEST_ASSERT_TRUE(page_contains(&filtered, c.observation));

    result_t labelled = call(handle_get_observations, HTTP_METHOD_GET, "/api/observations",
                             "label=BIRD,owl", NULL);
    TEST_ASSERT_EQUAL_INT(200, labelled.response.status_code);
    TEST_ASSERT_EQUAL_INT(1, item_count(&labelled));
    TEST_ASSERT_TRUE(page_contains(&labelled, a.observation));

    /* An explicit camera without live.view is 403, even alongside allowed ones. */
    snprintf(query, sizeof(query), "camera_uuid=%s,%s", cam_a, cam_b);
    result_t forbidden = call(handle_get_observations, HTTP_METHOD_GET, "/api/observations",
                              query, NULL);
    expect_error(&forbidden, 403, "forbidden");
    /* An unknown camera UUID is treated as unauthorized. */
    result_t unknown = call(handle_get_observations, HTTP_METHOD_GET, "/api/observations",
                            "camera_uuid=00000000-0000-4000-8000-000000000000", NULL);
    expect_error(&unknown, 403, "forbidden");
    /* Uppercase input is normalized. */
    char upper[OBS_UUID_SIZE];
    safe_strcpy(upper, cam_a, sizeof(upper), 0);
    for (char *p = upper; *p; p++) if (*p >= 'a' && *p <= 'f') *p = (char)(*p - 32);
    snprintf(query, sizeof(query), "camera_uuid=%s", upper);
    result_t uppercase = call(handle_get_observations, HTTP_METHOD_GET, "/api/observations",
                              query, NULL);
    TEST_ASSERT_EQUAL_INT(200, uppercase.response.status_code);
    TEST_ASSERT_EQUAL_INT(1, item_count(&uppercase));

    release(&all);
    release(&filtered);
    release(&labelled);
    release(&forbidden);
    release(&unknown);
    release(&uppercase);
}

static void test_list_without_any_grant_is_empty(void) {
    seed(cam_a, "bird", OBS_ASSET_PENDING);
    result_t result = call(handle_get_observations, HTTP_METHOD_GET, "/api/observations",
                           NULL, NULL);
    TEST_ASSERT_EQUAL_INT(200, result.response.status_code);
    TEST_ASSERT_EQUAL_INT(0, item_count(&result));
    release(&result);
}

static void test_list_rejects_invalid_parameters(void) {
    grant(cam_a, true, false);
    const char *bad[] = {
        "limit=0", "limit=501", "limit=abc", "limit=", "limit=10x",
        "camera_uuid=not-a-uuid", "camera_uuid=", "camera_uuid=,",
        "label=", "label=a,,b", "label=a,b,c,d,e,f,g,h,i",
        "label=abcdefghijabcdefghijabcdefghijab",
        "since=yesterday", "until=2026-11-08", "cursor=",
        "since=2026-11-08T14:00:00Z&until=2026-11-08T14:00:00Z",
        "since=2026-11-08T15:00:00Z&until=2026-11-08T14:00:00Z",
    };
    for (size_t i = 0; i < sizeof(bad) / sizeof(bad[0]); i++) {
        result_t result = call(handle_get_observations, HTTP_METHOD_GET, "/api/observations",
                               bad[i], NULL);
        char message[160];
        snprintf(message, sizeof(message), "query %s", bad[i]);
        TEST_ASSERT_EQUAL_INT_MESSAGE(400, result.response.status_code, message);
        TEST_ASSERT_EQUAL_STRING_MESSAGE("invalid_request", str(result.json, "code"), message);
        release(&result);
    }
    char query[512] = "camera_uuid=";
    for (int i = 0; i < 11; i++) {
        if (i) strcat(query, ",");
        strcat(query, cam_a);
    }
    result_t too_many = call(handle_get_observations, HTTP_METHOD_GET, "/api/observations",
                             query, NULL);
    expect_error(&too_many, 400, "invalid_request");
    release(&too_many);
}

static void test_list_filters_time_and_limit_and_pages(void) {
    grant(cam_a, true, false);
    seeded_t first = seed(cam_a, "bird", OBS_ASSET_PENDING);
    seeded_t second = seed(cam_a, "bird", OBS_ASSET_PENDING);
    seeded_t third = seed(cam_a, "bird", OBS_ASSET_PENDING);
    (void)first;

    observation_record_t record;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_get(second.observation, &record));
    char since[OBS_TIME_TEXT_SIZE], query[512];
    observation_format_time(record.observed_at_ms, since, sizeof(since));
    snprintf(query, sizeof(query), "since=%s", since);
    result_t window = call(handle_get_observations, HTTP_METHOD_GET, "/api/observations",
                           query, NULL);
    TEST_ASSERT_EQUAL_INT(200, window.response.status_code);
    TEST_ASSERT_EQUAL_INT(2, item_count(&window));
    TEST_ASSERT_TRUE(page_contains(&window, second.observation));
    TEST_ASSERT_TRUE(page_contains(&window, third.observation));

    result_t page1 = call(handle_get_observations, HTTP_METHOD_GET, "/api/observations",
                          "limit=2", NULL);
    TEST_ASSERT_EQUAL_INT(200, page1.response.status_code);
    TEST_ASSERT_EQUAL_INT(2, item_count(&page1));
    TEST_ASSERT_TRUE(cJSON_IsTrue(cJSON_GetObjectItem(page1.json, "has_more")));
    snprintf(query, sizeof(query), "limit=2&cursor=%s", str(page1.json, "next_cursor"));
    result_t page2 = call(handle_get_observations, HTTP_METHOD_GET, "/api/observations",
                          query, NULL);
    TEST_ASSERT_EQUAL_INT(200, page2.response.status_code);
    TEST_ASSERT_EQUAL_INT(1, item_count(&page2));
    TEST_ASSERT_TRUE(page_contains(&page2, third.observation));
    TEST_ASSERT_TRUE(cJSON_IsFalse(cJSON_GetObjectItem(page2.json, "has_more")));

    result_t head = call(handle_get_observations, HTTP_METHOD_GET, "/api/observations",
                         "cursor=head", NULL);
    TEST_ASSERT_EQUAL_INT(200, head.response.status_code);
    TEST_ASSERT_EQUAL_INT(0, item_count(&head));
    TEST_ASSERT_EQUAL_STRING(str(page2.json, "next_cursor"), str(head.json, "next_cursor"));

    release(&window);
    release(&page1);
    release(&page2);
    release(&head);
}

static void test_list_cursor_errors(void) {
    grant(cam_a, true, false);
    seed(cam_a, "bird", OBS_ASSET_PENDING);
    cJSON *invalid_fixture = load_fixture("errors/invalid-cursor.json");
    result_t invalid = call(handle_get_observations, HTTP_METHOD_GET, "/api/observations",
                            "cursor=not-a-cursor!", NULL);
    TEST_ASSERT_EQUAL_INT(400, invalid.response.status_code);
    assert_json_equal(invalid_fixture, invalid.json, "invalid cursor body");

    /* A cursor from another DB generation (epoch mismatch) has expired. */
    char cursor[OBS_CURSOR_SIZE], query[512];
    TEST_ASSERT_EQUAL_INT(0, observation_cursor_encode(1, "0f0f0f0f-0f0f-4f0f-8f0f-0f0f0f0f0f0f",
                                                       cursor, sizeof(cursor)));
    snprintf(query, sizeof(query), "cursor=%s", cursor);
    result_t expired = call(handle_get_observations, HTTP_METHOD_GET, "/api/observations",
                            query, NULL);
    expect_error(&expired, 410, "cursor_expired");
    cJSON *expired_fixture = load_fixture("errors/cursor-expired.json");
    assert_same_shape(expired_fixture, expired.json, "cursor_expired");
    TEST_ASSERT_EQUAL_STRING(str(expired_fixture, "error"), str(expired.json, "error"));
    observation_meta_t meta;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_meta_get(&meta));
    char oldest[OBS_CURSOR_SIZE];
    observation_cursor_encode(meta.oldest_seq - 1, meta.epoch_uuid, oldest, sizeof(oldest));
    TEST_ASSERT_EQUAL_STRING(oldest, str(expired.json, "oldest_cursor"));
    TEST_ASSERT_NOT_NULL(str(expired.json, "oldest_observed_at"));

    /* The oldest cursor resumes history. */
    snprintf(query, sizeof(query), "cursor=%s", oldest);
    result_t resumed = call(handle_get_observations, HTTP_METHOD_GET, "/api/observations",
                            query, NULL);
    TEST_ASSERT_EQUAL_INT(200, resumed.response.status_code);
    TEST_ASSERT_EQUAL_INT(1, item_count(&resumed));

    cJSON_Delete(invalid_fixture);
    cJSON_Delete(expired_fixture);
    release(&invalid);
    release(&expired);
    release(&resumed);
}

/* ---- GET /api/observations/{id}/snapshot ------------------------------ */

static bool audited(const char *target, const char *outcome) {
    for (int i = 0; i < audit_count; i++)
        if (strcmp(audits[i].action, "snapshot.create") == 0 &&
            strcmp(audits[i].target_type, "observation") == 0 &&
            strcmp(audits[i].target_uuid, target) == 0 &&
            strcmp(audits[i].outcome, outcome) == 0)
            return true;
    return false;
}

static void test_snapshot_ready_streams_the_exact_asset(void) {
    grant(cam_a, true, true);
    seeded_t seeded = seed(cam_a, "bird", OBS_ASSET_READY);
    char path[128], expected_file[1024], etag[64];
    obs_path(path, sizeof(path), seeded.observation, "/snapshot");
    snprintf(expected_file, sizeof(expected_file), "%s/%s", storage_dir, seeded.relative_path);
    snprintf(etag, sizeof(etag), "\"%s\"", seeded.asset);

    result_t result = call(handle_get_observation_snapshot, HTTP_METHOD_GET, path, NULL, NULL);
    TEST_ASSERT_EQUAL_INT(1, serve_calls);
    TEST_ASSERT_EQUAL_STRING(expected_file, served_path);
    TEST_ASSERT_EQUAL_STRING("image/jpeg", served_type);
    TEST_ASSERT_NOT_NULL(strstr(served_headers, "Cache-Control: private, max-age=300\r\n"));
    char etag_header[96];
    snprintf(etag_header, sizeof(etag_header), "ETag: %s\r\n", etag);
    TEST_ASSERT_NOT_NULL(strstr(served_headers, etag_header));
    TEST_ASSERT_TRUE(audited(seeded.observation, "allowed"));
    for (int i = 0; i < audit_count; i++) {
        TEST_ASSERT_NULL_MESSAGE(strstr(audits[i].details, seeded.relative_path), "audit path");
        if (strcmp(audits[i].target_type, "observation") == 0) {
            TEST_ASSERT_NOT_NULL(strstr(audits[i].details, seeded.asset));
            TEST_ASSERT_NOT_NULL(strstr(audits[i].details, cam_a));
        }
    }
    release(&result);

    /* Conditional request: 304 without serving the file. */
    result_t cached = call(handle_get_observation_snapshot, HTTP_METHOD_GET, path, NULL, etag);
    TEST_ASSERT_EQUAL_INT(304, cached.response.status_code);
    TEST_ASSERT_EQUAL_INT(1, serve_calls);
    TEST_ASSERT_EQUAL_STRING(etag, header(&cached.response, "ETag"));
    TEST_ASSERT_EQUAL_STRING("private, max-age=300", header(&cached.response, "Cache-Control"));
    release(&cached);

    /* A different ETag is served normally. */
    result_t stale = call(handle_get_observation_snapshot, HTTP_METHOD_GET, path, NULL,
                          "\"00000000-0000-4000-8000-000000000000\"");
    TEST_ASSERT_EQUAL_INT(2, serve_calls);
    release(&stale);
}

static void test_snapshot_ready_with_missing_file_never_substitutes(void) {
    grant(cam_a, true, true);
    seeded_t seeded = seed(cam_a, "bird", OBS_ASSET_READY);
    char file[1024], path[128];
    snprintf(file, sizeof(file), "%s/%s", storage_dir, seeded.relative_path);
    TEST_ASSERT_EQUAL_INT(0, unlink(file));
    obs_path(path, sizeof(path), seeded.observation, "/snapshot");
    result_t result = call(handle_get_observation_snapshot, HTTP_METHOD_GET, path, NULL, NULL);
    expect_error(&result, 404, "evidence_unavailable");
    TEST_ASSERT_EQUAL_STRING("write_failed", str(result.json, "reason"));
    TEST_ASSERT_EQUAL_INT(0, serve_calls);
    release(&result);
}

static void test_snapshot_rejects_paths_outside_observation_storage(void) {
    grant(cam_a, true, true);
    seeded_t seeded = seed(cam_a, "bird", OBS_ASSET_READY);
    const char *paths[] = {"../etc/passwd", "observations/../../etc/passwd", "/etc/passwd",
                           "snapshots/cam/x.jpg"};
    for (size_t i = 0; i < sizeof(paths) / sizeof(paths[0]); i++) {
        char sql[512];
        snprintf(sql, sizeof(sql), "UPDATE observation_assets SET path = '%s' WHERE uuid = '%s';",
                 paths[i], seeded.asset);
        TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(get_db_handle(), sql, NULL, NULL, NULL));
        char path[128];
        obs_path(path, sizeof(path), seeded.observation, "/snapshot");
        result_t result = call(handle_get_observation_snapshot, HTTP_METHOD_GET, path, NULL, NULL);
        TEST_ASSERT_EQUAL_INT_MESSAGE(500, result.response.status_code, paths[i]);
        TEST_ASSERT_EQUAL_INT(0, serve_calls);
        release(&result);
    }
}

static void test_snapshot_pending_unavailable_expired(void) {
    grant(cam_a, true, true);
    seeded_t pending = seed(cam_a, "bird", OBS_ASSET_PENDING);
    seeded_t unavailable = seed(cam_a, "bird", OBS_ASSET_UNAVAILABLE);
    seeded_t expired = seed(cam_a, "bird", OBS_ASSET_EXPIRED);
    char path[128];

    obs_path(path, sizeof(path), pending.observation, "/snapshot");
    result_t r_pending = call(handle_get_observation_snapshot, HTTP_METHOD_GET, path, NULL, NULL);
    expect_error(&r_pending, 202, "evidence_pending");
    const char *retry = header(&r_pending.response, "Retry-After");
    TEST_ASSERT_NOT_NULL(retry);
    int seconds = atoi(retry);
    TEST_ASSERT_TRUE(seconds >= 1 && seconds <= 30);
    TEST_ASSERT_EQUAL_INT(seconds,
                          cJSON_GetObjectItem(r_pending.json, "retry_after_seconds")->valueint);
    cJSON *pending_fixture = load_fixture("errors/evidence-pending.json");
    assert_same_shape(pending_fixture, r_pending.json, "evidence_pending");
    TEST_ASSERT_EQUAL_STRING(str(pending_fixture, "error"), str(r_pending.json, "error"));

    obs_path(path, sizeof(path), unavailable.observation, "/snapshot");
    result_t r_unavailable = call(handle_get_observation_snapshot, HTTP_METHOD_GET, path, NULL,
                                  NULL);
    TEST_ASSERT_EQUAL_INT(404, r_unavailable.response.status_code);
    cJSON *unavailable_fixture = load_fixture("errors/evidence-unavailable.json");
    assert_json_equal(unavailable_fixture, r_unavailable.json, "evidence_unavailable body");

    obs_path(path, sizeof(path), expired.observation, "/snapshot");
    result_t r_expired = call(handle_get_observation_snapshot, HTTP_METHOD_GET, path, NULL, NULL);
    expect_error(&r_expired, 410, "evidence_expired");
    TEST_ASSERT_EQUAL_STRING("retention_expired", str(r_expired.json, "reason"));
    cJSON *expired_fixture = load_fixture("errors/evidence-expired.json");
    assert_same_shape(expired_fixture, r_expired.json, "evidence_expired");
    TEST_ASSERT_EQUAL_STRING(str(expired_fixture, "error"), str(r_expired.json, "error"));
    int64_t expired_at = 0;
    TEST_ASSERT_EQUAL_INT(0, observation_parse_time(str(r_expired.json, "expired_at"), &expired_at));

    TEST_ASSERT_EQUAL_INT(0, serve_calls);
    cJSON_Delete(pending_fixture);
    cJSON_Delete(unavailable_fixture);
    cJSON_Delete(expired_fixture);
    release(&r_pending);
    release(&r_unavailable);
    release(&r_expired);
}

static void test_snapshot_authorization_d4(void) {
    seeded_t viewable = seed(cam_a, "bird", OBS_ASSET_READY);
    seeded_t hidden = seed(cam_b, "bird", OBS_ASSET_READY);
    grant(cam_a, true, false);  /* live.view only */
    grant(cam_b, false, false);
    char path[128];

    obs_path(path, sizeof(path), viewable.observation, "/snapshot");
    result_t forbidden = call(handle_get_observation_snapshot, HTTP_METHOD_GET, path, NULL, NULL);
    TEST_ASSERT_EQUAL_INT(403, forbidden.response.status_code);
    cJSON *forbidden_fixture = load_fixture("errors/snapshot-forbidden.json");
    assert_json_equal(forbidden_fixture, forbidden.json, "snapshot forbidden body");
    TEST_ASSERT_TRUE(audited(viewable.observation, "denied"));

    obs_path(path, sizeof(path), hidden.observation, "/snapshot");
    result_t hidden_result = call(handle_get_observation_snapshot, HTTP_METHOD_GET, path, NULL,
                                  NULL);
    TEST_ASSERT_EQUAL_INT(404, hidden_result.response.status_code);
    cJSON *not_found = load_fixture("errors/observation-not-found.json");
    assert_json_equal(not_found, hidden_result.json, "hidden snapshot body");

    result_t unknown = call(handle_get_observation_snapshot, HTTP_METHOD_GET,
                            "/api/observations/00000000-0000-4000-8000-000000000000/snapshot",
                            NULL, NULL);
    TEST_ASSERT_EQUAL_INT(404, unknown.response.status_code);
    TEST_ASSERT_EQUAL_MEMORY(unknown.response.body, hidden_result.response.body,
                             unknown.response.body_length);

    /* snapshot.create alone (without live.view) still serves the image. */
    grant_count = 0;
    grant(cam_b, false, true);
    result_t allowed = call(handle_get_observation_snapshot, HTTP_METHOD_GET, path, NULL, NULL);
    TEST_ASSERT_EQUAL_INT(1, serve_calls);

    evaluation_fails = true;
    result_t failure = call(handle_get_observation_snapshot, HTTP_METHOD_GET, path, NULL, NULL);
    expect_error(&failure, 500, "internal_error");

    TEST_ASSERT_EQUAL_INT(1, serve_calls);
    cJSON_Delete(forbidden_fixture);
    cJSON_Delete(not_found);
    release(&forbidden);
    release(&hidden_result);
    release(&unknown);
    release(&allowed);
    release(&failure);
}

/* ---- GET /api/integrations/capabilities ------------------------------ */

static const cJSON *operation(const cJSON *json, const char *name) {
    return cJSON_GetObjectItem(cJSON_GetObjectItem(json, "operations"), name);
}

static void test_capabilities_document(void) {
    grant_all(cam_a, true, true, true);
    grant(cam_b, true, false);
    grant_all(cam_c, false, false, true);  /* annotate alone still lists the camera */
    seed(cam_a, "bird", OBS_ASSET_PENDING);
    result_t result = call(handle_get_integration_capabilities, HTTP_METHOD_GET,
                           "/api/integrations/capabilities", NULL, NULL);
    TEST_ASSERT_EQUAL_INT(200, result.response.status_code);
    /* P2 (L14): observations.annotate is supported and advertised per camera. */
    cJSON *fixture = load_fixture("capabilities/p2.json");
    /* cameras is an array whose items are compared separately. */
    cJSON *fixture_cameras = cJSON_DetachItemFromObject(fixture, "cameras");
    cJSON *cameras = cJSON_DetachItemFromObject(result.json, "cameras");
    assert_same_shape(fixture, result.json, "capabilities");
    assert_json_equal(cJSON_GetObjectItem(fixture, "contract_versions"),
                      cJSON_GetObjectItem(result.json, "contract_versions"), "contract_versions");
    assert_json_equal(cJSON_GetObjectItem(fixture, "limits"),
                      cJSON_GetObjectItem(result.json, "limits"), "limits");
    const char *unsupported[] = {"observations.annotate", "track_lifecycle", "recorded_frame",
                                 "event_clip", "range_clip"};
    for (size_t i = 0; i < sizeof(unsupported) / sizeof(unsupported[0]); i++)
        assert_json_equal(operation(fixture, unsupported[i]),
                          operation(result.json, unsupported[i]), unsupported[i]);
    TEST_ASSERT_TRUE(cJSON_IsTrue(cJSON_GetObjectItem(operation(result.json,
                                                                "observations.history"),
                                                      "authorized")));
    TEST_ASSERT_TRUE(cJSON_IsTrue(cJSON_GetObjectItem(operation(result.json,
                                                                "observations.snapshot"),
                                                      "authorized")));

    const cJSON *src = cJSON_GetObjectItem(result.json, "source");
    TEST_ASSERT_EQUAL_STRING(source, str(src, "id"));
    TEST_ASSERT_EQUAL_STRING("lightnvr", str(src, "kind"));
    TEST_ASSERT_EQUAL_STRING(source + strlen("urn:lightnvr:"), str(src, "installation_uuid"));
    TEST_ASSERT_EQUAL_STRING("yawamf",
                             str(cJSON_GetObjectItem(result.json, "principal"), "writer_namespace"));
    const cJSON *history = cJSON_GetObjectItem(result.json, "history");
    TEST_ASSERT_EQUAL_INT(30, cJSON_GetObjectItem(history, "retention_days")->valueint);
    observation_meta_t meta;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_meta_get(&meta));
    char oldest[OBS_CURSOR_SIZE];
    observation_cursor_encode(meta.oldest_seq - 1, meta.epoch_uuid, oldest, sizeof(oldest));
    TEST_ASSERT_EQUAL_STRING(oldest, str(history, "oldest_cursor"));
    TEST_ASSERT_NOT_NULL(str(history, "oldest_observed_at"));
    const cJSON *delivery = cJSON_GetObjectItem(result.json, "delivery");
    TEST_ASSERT_EQUAL_STRING(
        "lightnvr/v1/events/io.lightnvr.detection.observation_changed.v1/{camera_uuid}",
        str(delivery, "topic"));

    /* Only cameras with an observation action, with the actions held. */
    TEST_ASSERT_EQUAL_INT(3, cJSON_GetArraySize(cameras));
    const cJSON *camera;
    cJSON_ArrayForEach(camera, cameras) {
        assert_same_shape(cJSON_GetArrayItem(fixture_cameras, 0), camera, "cameras[]");
        const cJSON *actions = cJSON_GetObjectItem(camera, "actions");
        if (strcmp(str(camera, "camera_uuid"), cam_a) == 0) {
            assert_json_equal(cJSON_GetObjectItem(cJSON_GetArrayItem(fixture_cameras, 0),
                                                  "actions"), actions, "cam_a actions");
            TEST_ASSERT_TRUE(cJSON_IsTrue(cJSON_GetObjectItem(camera, "evidence_enabled")));
        } else if (strcmp(str(camera, "camera_uuid"), cam_c) == 0) {
            TEST_ASSERT_EQUAL_INT(1, cJSON_GetArraySize(actions));
            TEST_ASSERT_EQUAL_STRING("observations.annotate",
                                     cJSON_GetArrayItem(actions, 0)->valuestring);
        } else {
            TEST_ASSERT_EQUAL_STRING(cam_b, str(camera, "camera_uuid"));
            TEST_ASSERT_EQUAL_INT(1, cJSON_GetArraySize(actions));
            TEST_ASSERT_EQUAL_STRING("live.view", cJSON_GetArrayItem(actions, 0)->valuestring);
            TEST_ASSERT_TRUE(cJSON_IsFalse(cJSON_GetObjectItem(camera, "evidence_enabled")));
        }
    }
    cJSON_Delete(cameras);
    cJSON_Delete(fixture_cameras);
    cJSON_Delete(fixture);
    release(&result);
}

static void test_capabilities_scope_without_grants_or_namespace(void) {
    safe_strcpy(principal_name, "system", sizeof(principal_name), 0);
    result_t result = call(handle_get_integration_capabilities, HTTP_METHOD_GET,
                           "/api/integrations/capabilities", NULL, NULL);
    TEST_ASSERT_EQUAL_INT(200, result.response.status_code);
    TEST_ASSERT_TRUE(cJSON_IsNull(cJSON_GetObjectItem(cJSON_GetObjectItem(result.json,
                                                                          "principal"),
                                                      "writer_namespace")));
    const char *ops[] = {"observations.history", "observations.snapshot"};
    for (int i = 0; i < 2; i++) {
        const cJSON *op = operation(result.json, ops[i]);
        TEST_ASSERT_TRUE(cJSON_IsTrue(cJSON_GetObjectItem(op, "supported")));
        TEST_ASSERT_TRUE(cJSON_IsFalse(cJSON_GetObjectItem(op, "authorized")));
        TEST_ASSERT_EQUAL_STRING("not_authorized", str(op, "reason"));
    }
    /* annotate is supported, but a reserved username has no writer namespace. */
    const cJSON *annotate = operation(result.json, "observations.annotate");
    TEST_ASSERT_TRUE(cJSON_IsTrue(cJSON_GetObjectItem(annotate, "supported")));
    TEST_ASSERT_TRUE(cJSON_IsFalse(cJSON_GetObjectItem(annotate, "authorized")));
    TEST_ASSERT_EQUAL_STRING("reserved_writer_namespace", str(annotate, "reason"));
    TEST_ASSERT_EQUAL_INT(0, cJSON_GetArraySize(cJSON_GetObjectItem(result.json, "cameras")));
    release(&result);

    /* A valid namespace without the action anywhere: not_authorized. */
    safe_strcpy(principal_name, "yawamf", sizeof(principal_name), 0);
    grant(cam_a, true, true);
    result = call(handle_get_integration_capabilities, HTTP_METHOD_GET,
                  "/api/integrations/capabilities", NULL, NULL);
    annotate = operation(result.json, "observations.annotate");
    TEST_ASSERT_TRUE(cJSON_IsFalse(cJSON_GetObjectItem(annotate, "authorized")));
    TEST_ASSERT_EQUAL_STRING("not_authorized", str(annotate, "reason"));
    release(&result);

    /* The action held with an invalid username: invalid_writer_namespace. */
    safe_strcpy(principal_name, "-bad", sizeof(principal_name), 0);
    grant_count = 0;
    grant_all(cam_a, true, true, true);
    result = call(handle_get_integration_capabilities, HTTP_METHOD_GET,
                  "/api/integrations/capabilities", NULL, NULL);
    annotate = operation(result.json, "observations.annotate");
    TEST_ASSERT_TRUE(cJSON_IsFalse(cJSON_GetObjectItem(annotate, "authorized")));
    TEST_ASSERT_EQUAL_STRING("invalid_writer_namespace", str(annotate, "reason"));
    release(&result);
    grant_count = 0;
    safe_strcpy(principal_name, "system", sizeof(principal_name), 0);
    result = call(handle_get_integration_capabilities, HTTP_METHOD_GET,
                  "/api/integrations/capabilities", NULL, NULL);
    /* Empty history still has the oldest cursor; oldest_observed_at is null. */
    const cJSON *history = cJSON_GetObjectItem(result.json, "history");
    TEST_ASSERT_NOT_NULL(str(history, "oldest_cursor"));
    TEST_ASSERT_TRUE(cJSON_IsNull(cJSON_GetObjectItem(history, "oldest_observed_at")));
    release(&result);
}

/* ---- PUT /api/observations/{id}/classifications/{ns}/{name} (L14) ------ */

#define FIXTURE_OBSERVATION "55555555-5555-4555-8555-555555555555"
#define FIXTURE_ASSET "88888888-8888-4888-8888-888888888888"

static void put_path(char *out, size_t size, const char *id, const char *ns, const char *name) {
    snprintf(out, size, "/api/observations/%s/classifications/%s/%s", id, ns, name);
}

static result_t put(const char *path, const char *body) {
    return call_with_body(handle_put_observation_classification, HTTP_METHOD_PUT, path, NULL,
                          NULL, body);
}

static int count_for(const char *sql, const char *uuid) {
    sqlite3_stmt *stmt = NULL;
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_prepare_v2(get_db_handle(), sql, -1, &stmt, NULL));
    sqlite3_bind_text(stmt, 1, uuid, -1, SQLITE_TRANSIENT);
    TEST_ASSERT_EQUAL_INT(SQLITE_ROW, sqlite3_step(stmt));
    int count = sqlite3_column_int(stmt, 0);
    sqlite3_finalize(stmt);
    return count;
}

static int classification_rows(const char *observation) {
    return count_for("SELECT count(*) FROM observation_classifications WHERE observation_uuid = ?;",
                     observation);
}

static int publication_rows(const char *observation) {
    return count_for("SELECT count(*) FROM observation_publications p JOIN observation_changes c "
                     "ON c.seq = p.change_seq WHERE c.observation_uuid = ?;", observation);
}

static int revision_of(const char *observation) {
    observation_record_t record;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_get(observation, &record));
    return record.revision;
}

/* Replace every string equal to from (recursively). */
static void substitute(cJSON *json, const char *from, const char *to) {
    cJSON *item;
    cJSON_ArrayForEach(item, json) {
        if (cJSON_IsString(item) && strcmp(item->valuestring, from) == 0)
            cJSON_SetValuestring(item, to);
        else if (cJSON_IsObject(item) || cJSON_IsArray(item))
            substitute(item, from, to);
    }
}

/* Run a C3 classification exchange against a seeded observation: the
 * fixture's observation and asset IDs are mapped to the seeded ones in the
 * path, body and expected response. Returns the response; *expected is the
 * mapped fixture response body. */
static result_t put_exchange(const char *file, const seeded_t *seeded, cJSON **expected,
                             int *status) {
    cJSON *exchange = load_fixture(file);
    const cJSON *request = cJSON_GetObjectItem(exchange, "request");
    const cJSON *principal = cJSON_GetObjectItem(request, "principal");
    safe_strcpy(principal_name, str(principal, "username"), sizeof(principal_name), 0);
    char path[256];
    const char *fixture_path = str(request, "path");
    const char *at = strstr(fixture_path, FIXTURE_OBSERVATION);
    TEST_ASSERT_NOT_NULL(at);
    snprintf(path, sizeof(path), "%.*s%s%s", (int)(at - fixture_path), fixture_path,
             seeded->observation, at + strlen(FIXTURE_OBSERVATION));
    cJSON *body = cJSON_Duplicate(cJSON_GetObjectItem(request, "body"), true);
    substitute(body, FIXTURE_ASSET, seeded->asset);
    char *text = cJSON_PrintUnformatted(body);
    cJSON_Delete(body);
    result_t result = put(path, text);
    cJSON_free(text);
    const cJSON *response = cJSON_GetObjectItem(exchange, "response");
    *status = cJSON_GetObjectItem(response, "status")->valueint;
    *expected = cJSON_Duplicate(cJSON_GetObjectItem(response, "body"), true);
    substitute(*expected, FIXTURE_ASSET, seeded->asset);
    substitute(*expected, FIXTURE_OBSERVATION, seeded->observation);
    cJSON_Delete(exchange);
    return result;
}

/* created_at/updated_at are server times: check them, then align them with
 * the fixture so the rest compares exactly. Returns a copy of the actual
 * annotation (before alignment), or NULL when the body has none. */
static cJSON *align_annotation_times(cJSON *expected, cJSON *actual) {
    const char *keys[] = {"annotation", "current"};
    for (int k = 0; k < 2; k++) {
        cJSON *want = cJSON_GetObjectItem(expected, keys[k]);
        cJSON *got = cJSON_GetObjectItem(actual, keys[k]);
        if (!want) {
            TEST_ASSERT_NULL_MESSAGE(got, keys[k]);
            continue;
        }
        TEST_ASSERT_NOT_NULL_MESSAGE(got, keys[k]);
        cJSON *copy = cJSON_Duplicate(got, true);
        int64_t created = 0, updated = 0;
        time_ms(str(got, "created_at"), &created);
        time_ms(str(got, "updated_at"), &updated);
        TEST_ASSERT_TRUE(updated >= created);
        cJSON_ReplaceItemInObject(got, "created_at",
                                  cJSON_CreateString(str(want, "created_at")));
        cJSON_ReplaceItemInObject(got, "updated_at",
                                  cJSON_CreateString(str(want, "updated_at")));
        return copy;
    }
    return NULL;
}

static cJSON *run_exchange(const char *file, const seeded_t *seeded) {
    cJSON *expected = NULL;
    int status = 0;
    result_t result = put_exchange(file, seeded, &expected, &status);
    TEST_ASSERT_EQUAL_INT_MESSAGE(status, result.response.status_code, file);
    TEST_ASSERT_NOT_NULL_MESSAGE(result.json, file);
    cJSON *annotation = align_annotation_times(expected, result.json);
    assert_json_equal(expected, result.json, file);
    cJSON_Delete(expected);
    release(&result);
    return annotation;
}

static const audit_record_t *last_annotate_audit(void) {
    for (int i = audit_count - 1; i >= 0; i--)
        if (strcmp(audits[i].action, "observations.annotate") == 0) return &audits[i];
    return NULL;
}

static void expect_annotate_audit(const char *observation, const char *outcome,
                                  const char *result_name) {
    const audit_record_t *record = last_annotate_audit();
    TEST_ASSERT_NOT_NULL_MESSAGE(record, result_name);
    TEST_ASSERT_EQUAL_STRING("observation", record->target_type);
    TEST_ASSERT_EQUAL_STRING(observation, record->target_uuid);
    TEST_ASSERT_EQUAL_STRING_MESSAGE(outcome, record->outcome, result_name);
    cJSON *details = cJSON_Parse(record->details);
    TEST_ASSERT_NOT_NULL(details);
    TEST_ASSERT_EQUAL_STRING_MESSAGE(result_name, str(details, "result"), record->details);
    cJSON_Delete(details);
}

/* Fixtures 01-05 are one history: create, retry, stale write, owner
 * correction and the automated re-run it locks out. */
static void test_put_fixture_chain_create_retry_conflict_owner(void) {
    grant_all(cam_a, true, false, true);
    seeded_t seeded = seed(cam_a, "bird", OBS_ASSET_READY);
    TEST_ASSERT_EQUAL_INT(2, revision_of(seeded.observation));

    cJSON *created = run_exchange("classifications/01-create-automated.json", &seeded);
    TEST_ASSERT_EQUAL_STRING(str(created, "created_at"), str(created, "updated_at"));
    TEST_ASSERT_EQUAL_INT(3, revision_of(seeded.observation));
    TEST_ASSERT_EQUAL_INT(1, classification_rows(seeded.observation));
    expect_annotate_audit(seeded.observation, "success", "created");
    int publications = publication_rows(seeded.observation);

    /* 02: identical retry, nothing written and the revision is not bumped. */
    cJSON *retried = run_exchange("classifications/02-retry-noop.json", &seeded);
    assert_json_equal(created, retried, "retry returns the stored annotation");
    TEST_ASSERT_EQUAL_INT(3, revision_of(seeded.observation));
    TEST_ASSERT_EQUAL_INT(publications, publication_rows(seeded.observation));
    expect_annotate_audit(seeded.observation, "success", "noop");

    /* 03: a different result with a stale expected_revision. */
    cJSON *conflict = run_exchange("classifications/03-revision-conflict.json", &seeded);
    assert_json_equal(created, conflict, "409 current is the stored annotation");
    TEST_ASSERT_EQUAL_INT(3, revision_of(seeded.observation));
    expect_annotate_audit(seeded.observation, "failure", "revision_conflict");

    /* 04: the owner correction replaces it. */
    cJSON *owner = run_exchange("classifications/04-owner-confirm.json", &seeded);
    TEST_ASSERT_EQUAL_STRING(str(created, "created_at"), str(owner, "created_at"));
    TEST_ASSERT_EQUAL_INT(4, revision_of(seeded.observation));
    TEST_ASSERT_EQUAL_INT(1, classification_rows(seeded.observation));
    TEST_ASSERT_EQUAL_INT(publications + 1, publication_rows(seeded.observation));
    expect_annotate_audit(seeded.observation, "success", "updated");

    /* 05: an automated re-run never overrides the owner decision. */
    cJSON *locked = run_exchange("classifications/05-owner-decision-locked.json", &seeded);
    assert_json_equal(owner, locked, "409 current is the owner decision");
    TEST_ASSERT_EQUAL_INT(4, revision_of(seeded.observation));
    expect_annotate_audit(seeded.observation, "failure", "owner_decision_locked");

    /* The detector fields are untouched. */
    observation_record_t record;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_get(seeded.observation, &record));
    TEST_ASSERT_EQUAL_STRING("bird", record.label);
    TEST_ASSERT_EQUAL_FLOAT(0.87f, (float)record.confidence);

    cJSON_Delete(created);
    cJSON_Delete(retried);
    cJSON_Delete(conflict);
    cJSON_Delete(owner);
    cJSON_Delete(locked);
}

static void test_put_fixture_rejections_write_nothing(void) {
    grant_all(cam_a, true, false, true);
    struct { const char *file; observation_asset_state_t state; const char *outcome;
             const char *result; } cases[] = {
        {"classifications/06-evidence-mismatch-asset.json", OBS_ASSET_READY, "failure",
         "evidence_mismatch"},
        {"classifications/07-evidence-mismatch-pending.json", OBS_ASSET_PENDING, "failure",
         "evidence_mismatch"},
        {"classifications/08-namespace-forbidden.json", OBS_ASSET_READY, "denied",
         "namespace_forbidden"},
    };
    for (size_t i = 0; i < sizeof(cases) / sizeof(cases[0]); i++) {
        seeded_t seeded = seed(cam_a, "bird", cases[i].state);
        int revision = revision_of(seeded.observation);
        int publications = publication_rows(seeded.observation);
        cJSON *annotation = run_exchange(cases[i].file, &seeded);
        TEST_ASSERT_NULL(annotation);
        TEST_ASSERT_EQUAL_INT(0, classification_rows(seeded.observation));
        TEST_ASSERT_EQUAL_INT(revision, revision_of(seeded.observation));
        TEST_ASSERT_EQUAL_INT(publications, publication_rows(seeded.observation));
        expect_annotate_audit(seeded.observation, cases[i].outcome, cases[i].result);
    }
}

static const char *body_for(const char *asset, const char *kind, int expected_revision,
                            const char *label, const char *extra) {
    static char buffer[2048];
    snprintf(buffer, sizeof(buffer),
             "{\"classifier\":\"yawamf\",\"model_version\":\"m1\",\"label\":\"%s\","
             "\"taxonomy_id\":null,\"confidence\":0.5,"
             "\"classified_at\":\"2026-11-08T14:03:15.002Z\",\"input_asset_id\":\"%s\","
             "\"input_revision\":2,\"kind\":\"%s\",\"expected_revision\":%d%s}",
             label, asset, kind, expected_revision, extra ? extra : "");
    return buffer;
}

/* A7-annotate: a camera the caller cannot view looks like an unknown ID; a
 * viewable camera without the action is a 403; nothing is written on denial. */
static void test_put_authorization_on_capture_camera(void) {
    seeded_t viewable = seed(cam_a, "bird", OBS_ASSET_READY);
    seeded_t hidden = seed(cam_b, "bird", OBS_ASSET_READY);
    seeded_t writable = seed(cam_c, "bird", OBS_ASSET_READY);
    grant_all(cam_a, true, true, false);   /* live.view + snapshot, no annotate */
    grant_all(cam_c, false, false, true);  /* annotate alone is sufficient */
    char path[256];

    put_path(path, sizeof(path), viewable.observation, "yawamf", "species");
    result_t forbidden = put(path, body_for(viewable.asset, "automated", 0, "Robin", NULL));
    expect_error(&forbidden, 403, "forbidden");
    TEST_ASSERT_NULL(cJSON_GetObjectItem(forbidden.json, "current"));
    expect_annotate_audit(viewable.observation, "denied", "forbidden");

    cJSON *not_found = load_fixture("errors/observation-not-found.json");
    put_path(path, sizeof(path), hidden.observation, "yawamf", "species");
    result_t hidden_result = put(path, body_for(hidden.asset, "automated", 0, "Robin", NULL));
    TEST_ASSERT_EQUAL_INT(404, hidden_result.response.status_code);
    assert_json_equal(not_found, hidden_result.json, "hidden put body");
    expect_annotate_audit(hidden.observation, "denied", "observation_not_found");
    /* Authorization precedes namespace and body checks, so they leak nothing. */
    put_path(path, sizeof(path), hidden.observation, "someone-else", "species");
    result_t hidden_bad = put(path, "{not json");
    TEST_ASSERT_EQUAL_INT(404, hidden_bad.response.status_code);
    TEST_ASSERT_EQUAL_MEMORY(hidden_result.response.body, hidden_bad.response.body,
                             hidden_result.response.body_length);

    put_path(path, sizeof(path), "00000000-0000-4000-8000-000000000000", "yawamf", "species");
    result_t unknown = put(path, body_for(hidden.asset, "automated", 0, "Robin", NULL));
    TEST_ASSERT_EQUAL_INT(404, unknown.response.status_code);
    TEST_ASSERT_EQUAL_size_t(unknown.response.body_length, hidden_result.response.body_length);
    TEST_ASSERT_EQUAL_MEMORY(unknown.response.body, hidden_result.response.body,
                             unknown.response.body_length);

    TEST_ASSERT_EQUAL_INT(0, classification_rows(viewable.observation));
    TEST_ASSERT_EQUAL_INT(0, classification_rows(hidden.observation));
    TEST_ASSERT_EQUAL_INT(2, revision_of(viewable.observation));
    TEST_ASSERT_EQUAL_INT(2, revision_of(hidden.observation));

    put_path(path, sizeof(path), writable.observation, "yawamf", "species");
    result_t allowed = put(path, body_for(writable.asset, "automated", 0, "Robin", NULL));
    TEST_ASSERT_EQUAL_INT(201, allowed.response.status_code);
    TEST_ASSERT_EQUAL_INT(1, classification_rows(writable.observation));

    evaluation_fails = true;
    put_path(path, sizeof(path), writable.observation, "yawamf", "other");
    result_t failure = put(path, body_for(writable.asset, "automated", 0, "Robin", NULL));
    expect_error(&failure, 500, "internal_error");
    TEST_ASSERT_EQUAL_INT(1, classification_rows(writable.observation));
    expect_annotate_audit(writable.observation, "error", "internal_error");

    cJSON_Delete(not_found);
    release(&forbidden);
    release(&hidden_result);
    release(&hidden_bad);
    release(&unknown);
    release(&allowed);
    release(&failure);
}

static void test_put_namespace_rules(void) {
    grant_all(cam_a, true, false, true);
    seeded_t seeded = seed(cam_a, "bird", OBS_ASSET_READY);
    char path[256];
    put_path(path, sizeof(path), seeded.observation, "yawamf", "species");

    /* A body naming another writer is refused, before body validation. */
    const char *other[] = {",\"writer_namespace\":\"frigate-bridge\"", ",\"writer\":\"owner\""};
    for (int i = 0; i < 2; i++) {
        result_t result = put(path, body_for(seeded.asset, "automated", 0, "Robin", other[i]));
        expect_error(&result, 403, "namespace_forbidden");
        release(&result);
    }
    /* Reserved and invalid usernames have no namespace at all. */
    const char *names[][2] = {{"system", "system"}, {"Lightnvr", "lightnvr"}, {"-bad", "bad"}};
    for (int i = 0; i < 3; i++) {
        safe_strcpy(principal_name, names[i][0], sizeof(principal_name), 0);
        char own[256];
        put_path(own, sizeof(own), seeded.observation, names[i][1], "species");
        result_t result = put(own, body_for(seeded.asset, "automated", 0, "Robin", NULL));
        expect_error(&result, 403, "namespace_forbidden");
        release(&result);
    }
    TEST_ASSERT_EQUAL_INT(0, classification_rows(seeded.observation));

    /* Naming the caller's own namespace is accepted and ignored. */
    safe_strcpy(principal_name, "YaWamf", sizeof(principal_name), 0);
    result_t own = put(path, body_for(seeded.asset, "automated", 0, "Robin",
                                      ",\"writer_namespace\":\"yawamf\",\"writer\":\"yawamf\""));
    TEST_ASSERT_EQUAL_INT(201, own.response.status_code);
    const cJSON *annotation = cJSON_GetObjectItem(own.json, "annotation");
    TEST_ASSERT_EQUAL_STRING("yawamf/species", str(annotation, "key"));
    TEST_ASSERT_EQUAL_STRING("yawamf", str(annotation, "writer_namespace"));
    TEST_ASSERT_NULL(cJSON_GetObjectItem(annotation, "writer_user_id"));
    release(&own);

    /* D3: a later user reusing the username cannot touch the key, and the
     * stored values are withheld. */
    principal_id = 43;
    result_t reused = put(path, body_for(seeded.asset, "automated", 1, "Sparrow", NULL));
    expect_error(&reused, 403, "namespace_forbidden");
    TEST_ASSERT_NULL(cJSON_GetObjectItem(reused.json, "current"));
    TEST_ASSERT_NULL(strstr((const char *)reused.response.body, "Robin"));
    expect_annotate_audit(seeded.observation, "denied", "namespace_forbidden");
    release(&reused);
    TEST_ASSERT_EQUAL_INT(3, revision_of(seeded.observation));
}

static void test_put_validates_the_closed_body(void) {
    grant_all(cam_a, true, false, true);
    seeded_t seeded = seed(cam_a, "bird", OBS_ASSET_READY);
    char path[256];
    put_path(path, sizeof(path), seeded.observation, "yawamf", "species");
    char long65[66], long129[130];
    memset(long65, 'a', 65); long65[65] = '\0';
    memset(long129, 'a', 129); long129[129] = '\0';
    char long65_json[80], long129_json[150];
    snprintf(long65_json, sizeof(long65_json), "\"%s\"", long65);
    snprintf(long129_json, sizeof(long129_json), "\"%s\"", long129);
    /* {field, raw JSON value or NULL to remove the field} */
    const char *cases[][2] = {
        {"classifier", NULL}, {"classifier", "\"\""}, {"classifier", long65_json},
        {"classifier", "5"}, {"model_version", long65_json}, {"model_version", "5"},
        {"label", NULL}, {"label", "\"\""}, {"label", long129_json}, {"label", "\"\\u0000x\""},
        {"label", "\"\xff\xfe\""}, {"taxonomy_id", long65_json}, {"taxonomy_id", "true"},
        {"confidence", NULL}, {"confidence", "1.5"}, {"confidence", "-0.1"},
        {"confidence", "\"0.5\""}, {"classified_at", NULL}, {"classified_at", "\"yesterday\""},
        {"classified_at", "5"}, {"input_asset_id", NULL}, {"input_asset_id", "\"nope\""},
        {"input_revision", NULL}, {"input_revision", "0"}, {"input_revision", "1.5"},
        {"input_revision", "\"2\""}, {"input_revision", "3000000000"}, {"kind", NULL},
        {"kind", "\"manual\""}, {"expected_revision", "-1"}, {"expected_revision", "1.5"},
        {"expected_revision", "null"}, {"writer_namespace", "5"}, {"extra", "1"},
    };
    for (size_t i = 0; i < sizeof(cases) / sizeof(cases[0]); i++) {
        cJSON *body = cJSON_Parse(body_for(seeded.asset, "automated", 0, "Robin", NULL));
        TEST_ASSERT_NOT_NULL(body);
        cJSON_DeleteItemFromObject(body, cases[i][0]);
        if (cases[i][1]) {
            cJSON *value = cJSON_Parse(cases[i][1]);
            if (!value) {  /* invalid UTF-8 or embedded NUL: splice the raw text */
                char *text = cJSON_PrintUnformatted(body);
                char raw[1024];
                snprintf(raw, sizeof(raw), "%.*s,\"%s\":%s}", (int)strlen(text) - 1, text,
                         cases[i][0], cases[i][1]);
                cJSON_free(text);
                cJSON_Delete(body);
                result_t result = put(path, raw);
                char message[160];
                snprintf(message, sizeof(message), "raw %s=%s", cases[i][0], cases[i][1]);
                TEST_ASSERT_EQUAL_INT_MESSAGE(400, result.response.status_code, message);
                TEST_ASSERT_EQUAL_STRING_MESSAGE("invalid_request", str(result.json, "code"),
                                                 message);
                release(&result);
                continue;
            }
            cJSON_AddItemToObject(body, cases[i][0], value);
        }
        char *text = cJSON_PrintUnformatted(body);
        cJSON_Delete(body);
        result_t result = put(path, text);
        cJSON_free(text);
        char message[160];
        snprintf(message, sizeof(message), "%s=%s", cases[i][0],
                 cases[i][1] ? cases[i][1] : "(absent)");
        TEST_ASSERT_EQUAL_INT_MESSAGE(400, result.response.status_code, message);
        TEST_ASSERT_EQUAL_STRING_MESSAGE("invalid_request", str(result.json, "code"), message);
        TEST_ASSERT_EQUAL_STRING_MESSAGE(cases[i][0], str(result.json, "field"), message);
        release(&result);
    }
    const char *documents[] = {NULL, "", "not json", "[]", "null", "{\"classifier\":"};
    for (size_t i = 0; i < sizeof(documents) / sizeof(documents[0]); i++) {
        result_t result = put(path, documents[i]);
        TEST_ASSERT_EQUAL_INT_MESSAGE(400, result.response.status_code,
                                      documents[i] ? documents[i] : "(no body)");
        TEST_ASSERT_EQUAL_STRING("invalid_request", str(result.json, "code"));
        release(&result);
    }
    /* Oversized bodies are refused without parsing. */
    char *large = malloc(OBS_PUT_BODY_MAX + 64);
    TEST_ASSERT_NOT_NULL(large);
    snprintf(large, OBS_PUT_BODY_MAX + 64, "{\"classifier\":\"%*s\"}", OBS_PUT_BODY_MAX, "x");
    result_t oversized = put(path, large);
    free(large);
    expect_error(&oversized, 400, "invalid_request");
    release(&oversized);
    /* Path segments follow ^[a-z0-9][a-z0-9_.-]{0,31}$. */
    const char *names[] = {"Species", "-species", "a-name-that-is-longer-than-32-bytes", "sp%2Fx"};
    for (size_t i = 0; i < sizeof(names) / sizeof(names[0]); i++) {
        char bad[256];
        put_path(bad, sizeof(bad), seeded.observation, "yawamf", names[i]);
        result_t result = put(bad, body_for(seeded.asset, "automated", 0, "Robin", NULL));
        expect_error(&result, 400, "invalid_request");
        TEST_ASSERT_EQUAL_STRING_MESSAGE("name", str(result.json, "field"), names[i]);
        release(&result);
    }
    char bad_id[256];
    put_path(bad_id, sizeof(bad_id), "not-a-uuid", "yawamf", "species");
    result_t malformed = put(bad_id, body_for(seeded.asset, "automated", 0, "Robin", NULL));
    expect_error(&malformed, 400, "invalid_request");
    release(&malformed);

    TEST_ASSERT_EQUAL_INT(0, classification_rows(seeded.observation));
    TEST_ASSERT_EQUAL_INT(2, revision_of(seeded.observation));
    expect_annotate_audit(seeded.observation, "failure", "invalid_request");

    /* Optional fields may be absent; uppercase UUIDs are normalized. */
    char upper[OBS_UUID_SIZE];
    safe_strcpy(upper, seeded.asset, sizeof(upper), 0);
    for (char *p = upper; *p; p++) if (*p >= 'a' && *p <= 'f') *p = (char)(*p - 32);
    char minimal[512];
    snprintf(minimal, sizeof(minimal),
             "{\"classifier\":\"yawamf\",\"label\":\"Robin\",\"confidence\":1,"
             "\"classified_at\":\"2026-11-08T16:03:15+02:00\",\"input_asset_id\":\"%s\","
             "\"input_revision\":2,\"kind\":\"owner_confirmed\"}", upper);
    result_t created = put(path, minimal);
    TEST_ASSERT_EQUAL_INT(201, created.response.status_code);
    const cJSON *annotation = cJSON_GetObjectItem(created.json, "annotation");
    TEST_ASSERT_TRUE(cJSON_IsNull(cJSON_GetObjectItem(annotation, "model_version")));
    TEST_ASSERT_TRUE(cJSON_IsNull(cJSON_GetObjectItem(annotation, "taxonomy_id")));
    TEST_ASSERT_EQUAL_STRING(seeded.asset, str(annotation, "input_asset_id"));
    TEST_ASSERT_EQUAL_STRING("2026-11-08T14:03:15.000Z", str(annotation, "classified_at"));
    release(&created);
}

static void test_put_update_limit_and_absent_key_conflict(void) {
    grant_all(cam_a, true, false, true);
    seeded_t seeded = seed(cam_a, "bird", OBS_ASSET_READY);
    char path[256];

    /* expected_revision on a key that does not exist: 409 without current. */
    put_path(path, sizeof(path), seeded.observation, "yawamf", "species");
    result_t absent = put(path, body_for(seeded.asset, "automated", 3, "Robin", NULL));
    expect_error(&absent, 409, "revision_conflict");
    TEST_ASSERT_NULL(cJSON_GetObjectItem(absent.json, "current"));
    release(&absent);

    result_t created = put(path, body_for(seeded.asset, "automated", 0, "Robin", NULL));
    TEST_ASSERT_EQUAL_INT(201, created.response.status_code);
    release(&created);
    /* An automated update with the right revision: 200 changed=true. */
    result_t updated = put(path, body_for(seeded.asset, "automated", 1, "American Robin", NULL));
    TEST_ASSERT_EQUAL_INT(200, updated.response.status_code);
    TEST_ASSERT_TRUE(cJSON_IsTrue(cJSON_GetObjectItem(updated.json, "changed")));
    TEST_ASSERT_EQUAL_INT(4, cJSON_GetObjectItem(updated.json, "observation_revision")->valueint);
    const cJSON *annotation = cJSON_GetObjectItem(updated.json, "annotation");
    TEST_ASSERT_EQUAL_INT(2, cJSON_GetObjectItem(annotation, "annotation_revision")->valueint);
    TEST_ASSERT_EQUAL_STRING("American Robin", str(annotation, "label"));
    release(&updated);

    for (int i = 1; i < OBS_CLS_MAX_PER_OBSERVATION; i++) {
        char name[16];
        snprintf(name, sizeof(name), "k%d", i);
        put_path(path, sizeof(path), seeded.observation, "yawamf", name);
        result_t result = put(path, body_for(seeded.asset, "automated", 0, "Robin", NULL));
        TEST_ASSERT_EQUAL_INT(201, result.response.status_code);
        release(&result);
    }
    put_path(path, sizeof(path), seeded.observation, "yawamf", "ninth");
    result_t limited = put(path, body_for(seeded.asset, "automated", 0, "Robin", NULL));
    expect_error(&limited, 409, "annotation_limit");
    TEST_ASSERT_EQUAL_INT(8, cJSON_GetObjectItem(limited.json, "limit")->valueint);
    expect_annotate_audit(seeded.observation, "failure", "annotation_limit");
    release(&limited);
    TEST_ASSERT_EQUAL_INT(8, classification_rows(seeded.observation));
}

/* Section 10.5: GET and history items carry the annotations split by kind. */
static void test_get_and_list_include_classifications(void) {
    grant_all(cam_a, true, false, true);
    seeded_t seeded = seed(cam_a, "bird", OBS_ASSET_READY);
    seeded_t plain = seed(cam_a, "bird", OBS_ASSET_READY);
    char path[256];
    put_path(path, sizeof(path), seeded.observation, "yawamf", "species");
    result_t automated = put(path, body_for(seeded.asset, "automated", 0, "Robin", NULL));
    TEST_ASSERT_EQUAL_INT(201, automated.response.status_code);
    put_path(path, sizeof(path), seeded.observation, "yawamf", "owner");
    result_t owner = put(path, body_for(seeded.asset, "owner_confirmed", 0, "Wren", NULL));
    TEST_ASSERT_EQUAL_INT(201, owner.response.status_code);

    obs_path(path, sizeof(path), seeded.observation, NULL);
    result_t got = call(handle_get_observation, HTTP_METHOD_GET, path, NULL, NULL);
    TEST_ASSERT_EQUAL_INT(200, got.response.status_code);
    TEST_ASSERT_EQUAL_INT(4, cJSON_GetObjectItem(got.json, "revision")->valueint);
    const cJSON *lists = cJSON_GetObjectItem(got.json, "classifications");
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetArraySize(cJSON_GetObjectItem(lists, "automated")));
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetArraySize(cJSON_GetObjectItem(lists, "owner_confirmed")));
    assert_json_equal(cJSON_GetObjectItem(automated.json, "annotation"),
                      cJSON_GetArrayItem(cJSON_GetObjectItem(lists, "automated"), 0),
                      "GET automated");
    assert_json_equal(cJSON_GetObjectItem(owner.json, "annotation"),
                      cJSON_GetArrayItem(cJSON_GetObjectItem(lists, "owner_confirmed"), 0),
                      "GET owner_confirmed");
    /* The original detector label and confidence stay visible. */
    TEST_ASSERT_EQUAL_STRING("bird", str(got.json, "label"));

    result_t page = call(handle_get_observations, HTTP_METHOD_GET, "/api/observations", NULL,
                         NULL);
    TEST_ASSERT_EQUAL_INT(200, page.response.status_code);
    TEST_ASSERT_EQUAL_INT(2, item_count(&page));
    const cJSON *item;
    cJSON_ArrayForEach(item, cJSON_GetObjectItem(page.json, "items")) {
        const cJSON *item_lists = cJSON_GetObjectItem(item, "classifications");
        if (strcmp(str(item, "id"), seeded.observation) == 0) {
            assert_json_equal(lists, item_lists, "history item classifications");
        } else {
            TEST_ASSERT_EQUAL_STRING(plain.observation, str(item, "id"));
            TEST_ASSERT_EQUAL_INT(0, cJSON_GetArraySize(cJSON_GetObjectItem(item_lists,
                                                                            "automated")));
            TEST_ASSERT_EQUAL_INT(0, cJSON_GetArraySize(cJSON_GetObjectItem(item_lists,
                                                                            "owner_confirmed")));
        }
    }
    release(&automated);
    release(&owner);
    release(&got);
    release(&page);
}

/* The classification change is staged for the relay (MQTT) with the
 * section 5.4 message ID, which a retry neither changes nor duplicates. */
static void test_put_stages_a_classification_change_with_stable_message_id(void) {
    grant_all(cam_a, true, false, true);
    seeded_t seeded = seed(cam_a, "bird", OBS_ASSET_READY);
    char path[256];
    put_path(path, sizeof(path), seeded.observation, "yawamf", "species");
    const char *body = body_for(seeded.asset, "automated", 0, "Robin", NULL);
    char body_copy[2048];
    safe_strcpy(body_copy, body, sizeof(body_copy), 0);
    result_t created = put(path, body_copy);
    TEST_ASSERT_EQUAL_INT(201, created.response.status_code);
    TEST_ASSERT_EQUAL_INT(3, cJSON_GetObjectItem(created.json, "observation_revision")->valueint);

    observation_publication_t *pending = calloc(64, sizeof(*pending));
    TEST_ASSERT_NOT_NULL(pending);
    int count = db_observation_pending_publications(pending, 64);
    TEST_ASSERT_GREATER_OR_EQUAL(1, count);
    const observation_publication_t *publication = NULL;
    for (int i = 0; i < count; i++)
        if (strcmp(pending[i].observation_uuid, seeded.observation) == 0 &&
            pending[i].revision == 3)
            publication = &pending[i];
    TEST_ASSERT_NOT_NULL(publication);
    TEST_ASSERT_EQUAL_INT(OBS_CAUSE_CLASSIFICATION, publication->cause);
    char expected_id[OBS_UUID_SIZE];
    TEST_ASSERT_EQUAL_INT(0, observation_message_id(source, seeded.observation, 3, expected_id));
    TEST_ASSERT_EQUAL_STRING(expected_id, publication->message_id);

    char error[256] = "";
    event_envelope_t first, second;
    TEST_ASSERT_EQUAL_INT(0, observation_relay_build_envelope(publication, source, &first, error,
                                                              sizeof(error)));
    TEST_ASSERT_EQUAL_INT(0, observation_relay_build_envelope(publication, source, &second, error,
                                                              sizeof(error)));
    TEST_ASSERT_EQUAL_STRING(expected_id, first.id);
    TEST_ASSERT_EQUAL_STRING(first.id, second.id);
    TEST_ASSERT_EQUAL_STRING("classification",
                             str(first.data, "change_cause"));
    event_envelope_clear(&first);
    event_envelope_clear(&second);

    /* A retry after a lost response stages nothing new. */
    int publications = publication_rows(seeded.observation);
    result_t retried = put(path, body_copy);
    TEST_ASSERT_EQUAL_INT(200, retried.response.status_code);
    TEST_ASSERT_TRUE(cJSON_IsFalse(cJSON_GetObjectItem(retried.json, "changed")));
    TEST_ASSERT_EQUAL_INT(publications, publication_rows(seeded.observation));
    TEST_ASSERT_EQUAL_INT(3, revision_of(seeded.observation));

    /* Audit context: camera, key, kind, result and the annotation revisions;
     * never the request body beyond those fields. */
    const audit_record_t *record = last_annotate_audit();
    TEST_ASSERT_NOT_NULL(record);
    TEST_ASSERT_EQUAL_STRING("annotate", record->operation);
    cJSON *details = cJSON_Parse(record->details);
    TEST_ASSERT_EQUAL_STRING(cam_a, str(details, "camera_uuid"));
    TEST_ASSERT_EQUAL_STRING("yawamf/species", str(details, "key"));
    TEST_ASSERT_EQUAL_STRING("automated", str(details, "kind"));
    TEST_ASSERT_EQUAL_STRING("noop", str(details, "result"));
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetObjectItem(details, "annotation_revision_before")->valueint);
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetObjectItem(details, "annotation_revision_after")->valueint);
    TEST_ASSERT_NULL(strstr(record->details, "Robin"));
    cJSON_Delete(details);

    free(pending);
    release(&created);
    release(&retried);
}

int main(void) {
    snprintf(db_path, sizeof(db_path), "/tmp/lightnvr_observation_api_%ld.db", (long)getpid());
    unlink(db_path);
    snprintf(storage_dir, sizeof(storage_dir), "/tmp/lightnvr_observation_api_%ld",
             (long)getpid());
    mkdir(storage_dir, 0700);
    memset(&g_config, 0, sizeof(g_config));
    safe_strcpy(g_config.storage_path, storage_dir, sizeof(g_config.storage_path), 0);
    safe_strcpy(g_config.mqtt_topic_prefix, "lightnvr", sizeof(g_config.mqtt_topic_prefix), 0);
    g_config.mqtt_enabled = true;
    if (init_database(db_path) != 0) return 1;
    if (event_identity_init() != 0 || event_identity_get_source(source, sizeof(source)) != 0)
        return 1;

    UNITY_BEGIN();
    make_camera("feeder-east", cam_a);
    make_camera("feeder-west", cam_b);
    make_camera("yard", cam_c);
    RUN_TEST(test_every_route_requires_authentication);
    RUN_TEST(test_time_and_namespace_helpers);
    RUN_TEST(test_serializer_reproduces_every_observation_fixture);
    RUN_TEST(test_capabilities_scope_without_grants_or_namespace);
    RUN_TEST(test_get_returns_db_rows_in_fixture_shape);
    RUN_TEST(test_get_unknown_unauthorized_and_malformed);
    RUN_TEST(test_list_intersects_with_authorized_cameras);
    RUN_TEST(test_list_without_any_grant_is_empty);
    RUN_TEST(test_list_rejects_invalid_parameters);
    RUN_TEST(test_list_filters_time_and_limit_and_pages);
    RUN_TEST(test_list_cursor_errors);
    RUN_TEST(test_snapshot_ready_streams_the_exact_asset);
    RUN_TEST(test_snapshot_ready_with_missing_file_never_substitutes);
    RUN_TEST(test_snapshot_rejects_paths_outside_observation_storage);
    RUN_TEST(test_snapshot_pending_unavailable_expired);
    RUN_TEST(test_snapshot_authorization_d4);
    RUN_TEST(test_capabilities_document);
    RUN_TEST(test_put_fixture_chain_create_retry_conflict_owner);
    RUN_TEST(test_put_fixture_rejections_write_nothing);
    RUN_TEST(test_put_authorization_on_capture_camera);
    RUN_TEST(test_put_namespace_rules);
    RUN_TEST(test_put_validates_the_closed_body);
    RUN_TEST(test_put_update_limit_and_absent_key_conflict);
    RUN_TEST(test_get_and_list_include_classifications);
    RUN_TEST(test_put_stages_a_classification_change_with_stable_message_id);
    int result = UNITY_END();

    event_identity_shutdown();
    shutdown_database();
    unlink(db_path);
    char sidecar[300];
    snprintf(sidecar, sizeof(sidecar), "%s-wal", db_path); unlink(sidecar);
    snprintf(sidecar, sizeof(sidecar), "%s-shm", db_path); unlink(sidecar);
    char command[400];
    snprintf(command, sizeof(command), "rm -rf '%s'", storage_dir);
    if (system(command) != 0) fprintf(stderr, "failed to remove %s\n", storage_dir);
    return result;
}
