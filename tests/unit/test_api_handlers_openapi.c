/* SPDX-License-Identifier: GPL-3.0-or-later */
#define _POSIX_C_SOURCE 200809L
#define _XOPEN_SOURCE 700
#include <ftw.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>
#include "unity.h"
#include "core/config.h"
#include "database/db_core.h"
#include "database/db_auth.h"
#include "database/db_streams.h"
#include "database/db_authorization_snapshot.h"
#include "web/api_handlers_openapi.h"
#include "web/openapi_projection.h"
#include "web/openapi_document_generated.h"
#include "utils/strings.h"

extern config_t g_config;
static char directory[] = "/tmp/lightnvr-openapi-XXXXXX";
static char admin_key[64], viewer_key[64];
static int64_t admin_id;
static http_request_t request;
static http_response_t response;
static cJSON *document;
static int fault;
int __real_authorization_snapshot_create(const user_t *, authorization_snapshot_t **);
int __real_openapi_project(const cJSON *,openapi_access_fn,void *,cJSON **);
int __real_http_response_set_body(http_response_t *,const char *);
char *__real_cJSON_PrintUnformatted(const cJSON *);
int __wrap_authorization_snapshot_create(const user_t *u, authorization_snapshot_t **out) {
    if (fault == 1 || fault == 2) { *out = NULL; return fault == 1 ? -1 : -2; }
    return __real_authorization_snapshot_create(u,out);
}
int __wrap_openapi_project(const cJSON *j,openapi_access_fn access,void *ctx,cJSON **out) {
    if (fault == 3) { *out = NULL; return -1; }
    return __real_openapi_project(j,access,ctx,out);
}
int __wrap_http_response_set_body(http_response_t *r,const char *body) {
    if (fault == 4) return -1;
    return __real_http_response_set_body(r,body);
}
char *__wrap_cJSON_PrintUnformatted(const cJSON *j) { return fault == 5 ? NULL : __real_cJSON_PrintUnformatted(j); }
void setUp(void) {
    g_config.web_auth_enabled = true; g_config.demo_mode = false;
    fault = 0; document = NULL;
    http_request_init(&request); http_response_init(&response);
    request.method = HTTP_METHOD_GET;
    safe_strcpy(request.path,"/api/openapi.json",sizeof(request.path),0);
    safe_strcpy(request.client_ip,"127.0.0.1",sizeof(request.client_ip),0);
    request.num_headers = 1;
    safe_strcpy(request.headers[0].name,"X-API-Key",sizeof(request.headers[0].name),0);
    safe_strcpy(request.headers[0].value,admin_key,sizeof(request.headers[0].value),0);
}
void tearDown(void) { fault = 0; cJSON_Delete(document); http_response_free(&response); }
static void invoke(int status) {
    handle_get_openapi(&request,&response);
    TEST_ASSERT_EQUAL_INT(status,response.status_code);
    TEST_ASSERT_EQUAL_STRING("application/json",response.content_type);
    int caches = 0;
    for (int i = 0; i < response.num_headers; i++) {
        if (!strcasecmp(response.headers[i].name,"Cache-Control")) {
            caches++; TEST_ASSERT_EQUAL_STRING("private, no-store",response.headers[i].value);
        }
        TEST_ASSERT_NOT_EQUAL(0,strcasecmp(response.headers[i].name,"ETag"));
    }
    TEST_ASSERT_EQUAL_INT(1,caches);
    TEST_ASSERT_NOT_NULL(response.body);
    document = cJSON_Parse(response.body); TEST_ASSERT_NOT_NULL(document);
}
static cJSON *get(const cJSON *node,const char *key) { return cJSON_GetObjectItemCaseSensitive(node,key); }
static void test_admin_embedded_revision_and_headers(void) {
    http_response_add_header(&response,"Cache-Control","public");
    http_response_add_header(&response,"ETag","obsolete");
    invoke(200);
    TEST_ASSERT_EQUAL_INT(30,cJSON_GetArraySize(get(document,"paths")));
    TEST_ASSERT_EQUAL_STRING(lightnvr_openapi_document_revision,get(get(document,"x-lightnvr-coverage"),"canonical_revision")->valuestring);
    TEST_ASSERT_TRUE(cJSON_IsTrue(get(get(document,"x-lightnvr-coverage"),"partial")));
    TEST_ASSERT_NULL(get(document,"x-lightnvr-validators"));
}
static void test_viewer_cannot_override_identity(void) {
    safe_strcpy(request.headers[0].value,viewer_key,sizeof(request.headers[0].value),0);
    safe_strcpy(request.query_string,"user_id=1&role=admin",sizeof(request.query_string),0);
    invoke(200);
    TEST_ASSERT_NULL(get(get(document,"paths"),"/api/streams/{name}"));
    TEST_ASSERT_NOT_NULL(get(get(document,"paths"),"/api/streams/{name}/full"));
}
static void test_anonymous(void) { request.num_headers = 0; invoke(401); TEST_ASSERT_NOT_NULL(get(document,"error")); }
static void test_synthetic_modes_rejected(void) {
    g_config.web_auth_enabled = false; invoke(403); cJSON_Delete(document); document = NULL; http_response_free(&response); http_response_init(&response);
    g_config.web_auth_enabled = true; g_config.demo_mode = true; request.num_headers = 0; invoke(403);
}
static void test_authenticated_demo_retains_real_permissions(void) { g_config.demo_mode = true; invoke(200); TEST_ASSERT_NOT_NULL(get(get(document,"paths"),"/api/streams/{name}")); }
static void test_failure_never_returns_canonical(void) {
    for (int failure = 1; failure <= 5; failure++) {
        fault = failure; invoke(failure == 2 ? 401 : 500);
        TEST_ASSERT_NOT_NULL(get(document,"error")); TEST_ASSERT_NULL(get(document,"paths"));
        cJSON_Delete(document); document = NULL; http_response_free(&response); http_response_init(&response);
    }
}
static void test_password_gate(void) {
    TEST_ASSERT_EQUAL_INT(SQLITE_OK,sqlite3_exec(get_db_handle(),"UPDATE users SET must_change_password=1 WHERE username='contract_admin';",NULL,NULL,NULL));
    /* Existing API key/token credentials are exempt; Basic/session are gated. */
    invoke(200); cJSON_Delete(document); document = NULL; http_response_free(&response); http_response_init(&response);
    safe_strcpy(request.headers[0].name,"Authorization",sizeof(request.headers[0].name),0);
    safe_strcpy(request.headers[0].value,"Basic Y29udHJhY3RfYWRtaW46dGVzdC1wYXNzd29yZC0xMjM=",sizeof(request.headers[0].value),0);
    invoke(401);
    TEST_ASSERT_EQUAL_INT(SQLITE_OK,sqlite3_exec(get_db_handle(),"UPDATE users SET must_change_password=0 WHERE username='contract_admin';",NULL,NULL,NULL));
}
static int remove_path(const char *name,const struct stat *st,int type,struct FTW *walk) { (void)st; (void)type; (void)walk; return remove(name); }
int main(void) {
    if (!mkdtemp(directory)) return 1;
    char path[256]; snprintf(path,sizeof(path),"%s/db.sqlite",directory);
    if (init_database(path) || db_auth_init()) return 1;
    int64_t viewer_id;
    if (db_auth_create_user("contract_admin","test-password-123",NULL,USER_ROLE_ADMIN,true,&admin_id) ||
        db_auth_create_user("contract_viewer","test-password-123",NULL,USER_ROLE_VIEWER,true,&viewer_id) ||
        db_auth_generate_api_key(admin_id,admin_key,sizeof(admin_key)) ||
        db_auth_generate_api_key(viewer_id,viewer_key,sizeof(viewer_key))) return 1;
    stream_config_t camera = {0};
    safe_strcpy(camera.name,"A",sizeof(camera.name),0);
    safe_strcpy(camera.camera_uuid,"10000000-0000-4000-8000-000000000001",sizeof(camera.camera_uuid),0);
    safe_strcpy(camera.url,"rtsp://fixture.invalid/live",sizeof(camera.url),0);
    safe_strcpy(camera.codec,"h264",sizeof(camera.codec),0);
    if (!add_stream_config(&camera)) return 1;
    UNITY_BEGIN();
    RUN_TEST(test_admin_embedded_revision_and_headers); RUN_TEST(test_viewer_cannot_override_identity);
    RUN_TEST(test_anonymous); RUN_TEST(test_synthetic_modes_rejected);
    RUN_TEST(test_authenticated_demo_retains_real_permissions); RUN_TEST(test_failure_never_returns_canonical);
    RUN_TEST(test_password_gate);
    int result = UNITY_END(); shutdown_database();
    if (nftw(directory,remove_path,16,FTW_DEPTH|FTW_PHYS)) return 1;
    return result;
}
