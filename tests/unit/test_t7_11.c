/* T7.11: real private-DB exposure characterization and boundary controls.
 * Plaintext characterization is evidence of an OPEN finding, not remediation. */
#define _POSIX_C_SOURCE 200809L
#include <fcntl.h>
#include <sqlite3.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <unistd.h>
#include <cjson/cJSON.h>
#include "unity.h"
#include "core/config.h"
#include "core/logger.h"
#include "database/db_auth.h"
#include "database/db_core.h"
#include "database/db_event_destinations.h"
#include "web/api_handlers_event_destinations.h"
#include "web/request_response.h"

static char directory[]="/tmp/lightnvr-t7-11-XXXXXX", database[256];
static char admin_key[128], viewer_key[128], user_key[128];
static const char *secret_create="T7_11_SYNTHETIC_CREATE_CREDENTIAL_620b";
static const char *secret_update="T7_11_SYNTHETIC_UPDATE_CREDENTIAL_98c1";

static void sql(const char *statement) {
    char *error=NULL;int status=sqlite3_exec(get_db_handle(),statement,NULL,NULL,&error);
    if(error)fprintf(stderr,"SQL: %s\n",error);
    sqlite3_free(error);TEST_ASSERT_EQUAL_INT(SQLITE_OK,status);
}
static event_destination_t destination(void) {
    event_destination_t value={0};
    strcpy(value.name,"T7.11 destination");strcpy(value.destination_type,"mqtt");
    strcpy(value.broker_host,"mqtt.synthetic.invalid");value.broker_port=8883;
    strcpy(value.client_id,"t7-11");strcpy(value.topic_template,EVENT_DESTINATION_DEFAULT_TOPIC_TEMPLATE);
    strcpy(value.username,"synthetic-publisher");strcpy(value.tls_mode,"system");
    value.keepalive_seconds=60;value.qos=1;return value;
}
static event_destination_t create(void) {
    event_destination_t value=destination();
    TEST_ASSERT_EQUAL_INT(DB_EVENT_DESTINATION_OK,db_event_destination_create(&value,secret_create));return value;
}
static void assert_private_fixture(void) {
    struct stat info;TEST_ASSERT_EQUAL_INT(0,stat(directory,&info));TEST_ASSERT_EQUAL_INT(0700,info.st_mode&0777);
    TEST_ASSERT_EQUAL_INT(0,stat(database,&info));TEST_ASSERT_EQUAL_INT(0600,info.st_mode&0777);
}
static void assert_sqlite_plaintext(const char *expected) {
    sqlite3 *reader=NULL;sqlite3_stmt *statement=NULL;
    TEST_ASSERT_EQUAL_INT(SQLITE_OK,sqlite3_open_v2(database,&reader,SQLITE_OPEN_READONLY,NULL));
    TEST_ASSERT_EQUAL_INT(SQLITE_OK,sqlite3_prepare_v2(reader,"SELECT password FROM event_destinations",-1,&statement,NULL));
    TEST_ASSERT_EQUAL_INT(SQLITE_ROW,sqlite3_step(statement));
    TEST_ASSERT_EQUAL_STRING(expected,(const char *)sqlite3_column_text(statement,0));
    sqlite3_finalize(statement);sqlite3_close(reader);
}
static void assert_plaintext_on_disk(const char *expected) {
    TEST_ASSERT_EQUAL_INT(SQLITE_OK,sqlite3_wal_checkpoint_v2(get_db_handle(),NULL,SQLITE_CHECKPOINT_TRUNCATE,NULL,NULL));
    FILE *file=fopen(database,"rb");TEST_ASSERT_NOT_NULL(file);
    TEST_ASSERT_EQUAL_INT(0,fseek(file,0,SEEK_END));long length=ftell(file);TEST_ASSERT_GREATER_THAN(0,length);
    rewind(file);unsigned char *bytes=malloc((size_t)length);TEST_ASSERT_NOT_NULL(bytes);
    TEST_ASSERT_EQUAL_size_t((size_t)length,fread(bytes,1,(size_t)length,file));fclose(file);
    bool found=false;size_t needle=strlen(expected);
    for(size_t i=0;i+needle<=(size_t)length;i++)if(memcmp(bytes+i,expected,needle)==0){found=true;break;}
    free(bytes);TEST_ASSERT_TRUE_MESSAGE(found,"Characterization must reproduce cleartext on disk; this is not a security pass");
}
static void assert_secret(const event_destination_t *value,const char *expected) {
    char password[EVENT_DESTINATION_PASSWORD_MAX];memset(password,0xa5,sizeof(password));
    TEST_ASSERT_EQUAL_INT(DB_EVENT_DESTINATION_OK,db_event_destination_get_password(value->uuid,value->revision,password,sizeof(password)));
    TEST_ASSERT_EQUAL_STRING(expected,password);memset(password,0,sizeof(password));
}
static cJSON *request(void (*handler)(const http_request_t *,http_response_t *),http_method_t method,
                      const char *path,const char *body,const char *key,int expected) {
    http_request_t req;http_response_t res;http_request_init(&req);http_response_init(&res);
    req.method=method;snprintf(req.method_str,sizeof(req.method_str),"%s",method==HTTP_METHOD_GET?"GET":method==HTTP_METHOD_POST?"POST":"PUT");
    snprintf(req.path,sizeof(req.path),"%s",path);snprintf(req.uri,sizeof(req.uri),"%s",path);
    strcpy(req.client_ip,"127.0.0.1");req.body=(void *)body;req.body_len=body?strlen(body):0;
    if(key){strcpy(req.headers[0].name,"X-API-Key");snprintf(req.headers[0].value,sizeof(req.headers[0].value),"%s",key);req.num_headers=1;}
    handler(&req,&res);TEST_ASSERT_EQUAL_INT(expected,res.status_code);
    TEST_ASSERT_NOT_NULL(res.body);TEST_ASSERT_NULL(strstr(res.body,secret_create));TEST_ASSERT_NULL(strstr(res.body,secret_update));
    cJSON *json=cJSON_Parse(res.body);TEST_ASSERT_NOT_NULL(json);http_response_free(&res);return json;
}
static void assert_zero(const unsigned char *bytes,size_t length){for(size_t i=0;i<length;i++)TEST_ASSERT_EQUAL_UINT8(0,bytes[i]);}
void setUp(void){g_config.web_auth_enabled=true;g_config.demo_mode=false;sql("DELETE FROM event_outbox;DELETE FROM event_routes;DELETE FROM event_destinations;DELETE FROM audit_events;");}
void tearDown(void){}

void test_codeql_316_create_reproduces_plaintext_in_private_database(void){
    event_destination_t value=create();assert_private_fixture();assert_secret(&value,secret_create);
    assert_sqlite_plaintext(secret_create);assert_plaintext_on_disk(secret_create);
}
void test_codeql_307_replace_reproduces_plaintext_in_private_database(void){
    event_destination_t value=create();
    TEST_ASSERT_EQUAL_INT(DB_EVENT_DESTINATION_OK,db_event_destination_update(&value,value.revision,secret_update,true));
    assert_secret(&value,secret_update);assert_sqlite_plaintext(secret_update);assert_plaintext_on_disk(secret_update);
}
void test_admin_create_get_list_update_and_audit_are_redacted(void){
    char body[1024];snprintf(body,sizeof(body),"{\"name\":\"API credential fixture\",\"broker\":{\"host\":\"mqtt.synthetic.invalid\",\"port\":8883},\"authentication\":{\"username\":\"publisher\",\"password\":\"%s\"},\"tls\":{\"mode\":\"system\"}}",secret_create);
    cJSON *json=request(handle_post_event_destination,HTTP_METHOD_POST,"/api/event-destinations",body,admin_key,201);
    char path[128];snprintf(path,sizeof(path),"/api/event-destinations/%s",cJSON_GetObjectItemCaseSensitive(json,"uuid")->valuestring);
    cJSON *auth=cJSON_GetObjectItemCaseSensitive(json,"authentication");TEST_ASSERT_NULL(cJSON_GetObjectItemCaseSensitive(auth,"password"));
    TEST_ASSERT_TRUE(cJSON_IsTrue(cJSON_GetObjectItemCaseSensitive(auth,"password_configured")));cJSON_Delete(json);
    cJSON_Delete(request(handle_get_event_destination,HTTP_METHOD_GET,path,NULL,admin_key,200));
    cJSON_Delete(request(handle_get_event_destinations,HTTP_METHOD_GET,"/api/event-destinations",NULL,admin_key,200));
    snprintf(body,sizeof(body),"{\"revision\":1,\"authentication\":{\"password\":\"%s\"}}",secret_update);
    cJSON_Delete(request(handle_put_event_destination,HTTP_METHOD_PUT,path,body,admin_key,200));
    assert_sqlite_plaintext(secret_update);
    sqlite3_stmt *statement=NULL;TEST_ASSERT_EQUAL_INT(SQLITE_OK,sqlite3_prepare_v2(get_db_handle(),"SELECT details_json FROM audit_events",-1,&statement,NULL));
    int count=0;while(sqlite3_step(statement)==SQLITE_ROW){const char *details=(const char *)sqlite3_column_text(statement,0);TEST_ASSERT_NULL(strstr(details,secret_create));TEST_ASSERT_NULL(strstr(details,secret_update));count++;}
    sqlite3_finalize(statement);TEST_ASSERT_GREATER_THAN(0,count);
}
static void assert_denied(const char *key,int status){
    event_destination_t value=create();char path[128];snprintf(path,sizeof(path),"/api/event-destinations/%s",value.uuid);
    cJSON_Delete(request(handle_get_event_destinations,HTTP_METHOD_GET,"/api/event-destinations",NULL,key,status));
    cJSON_Delete(request(handle_get_event_destination,HTTP_METHOD_GET,path,NULL,key,status));
    cJSON_Delete(request(handle_post_event_destination,HTTP_METHOD_POST,"/api/event-destinations","{}",key,status));
    cJSON_Delete(request(handle_put_event_destination,HTTP_METHOD_PUT,path,"{}",key,status));
    TEST_ASSERT_EQUAL_INT(1,db_event_destination_count());assert_secret(&value,secret_create);
    event_destination_t current;TEST_ASSERT_EQUAL_INT(DB_EVENT_DESTINATION_OK,db_event_destination_get(value.uuid,&current));TEST_ASSERT_EQUAL_INT64(value.revision,current.revision);
}
void test_anonymous_requests_cannot_read_or_mutate(void){assert_denied(NULL,401);}
void test_viewer_requests_cannot_read_or_mutate(void){assert_denied(viewer_key,403);}
void test_standard_user_requests_cannot_read_or_mutate(void){assert_denied(user_key,403);}
void test_runtime_getter_clears_output_on_stale_invalid_missing_and_small_buffer(void){
    event_destination_t value=create();unsigned char password[EVENT_DESTINATION_PASSWORD_MAX];
    memset(password,0xa5,sizeof(password));TEST_ASSERT_EQUAL_INT(DB_EVENT_DESTINATION_STALE,db_event_destination_get_password(value.uuid,value.revision+1,(char*)password,sizeof(password)));assert_zero(password,sizeof(password));
    memset(password,0xa5,sizeof(password));TEST_ASSERT_EQUAL_INT(DB_EVENT_DESTINATION_INVALID,db_event_destination_get_password("invalid",1,(char*)password,sizeof(password)));assert_zero(password,sizeof(password));
    memset(password,0xa5,sizeof(password));TEST_ASSERT_EQUAL_INT(DB_EVENT_DESTINATION_NOT_FOUND,db_event_destination_get_password("00000000-0000-4000-8000-000000000001",1,(char*)password,sizeof(password)));assert_zero(password,sizeof(password));
    memset(password,0xa5,sizeof(password));TEST_ASSERT_EQUAL_INT(DB_EVENT_DESTINATION_ERROR,db_event_destination_get_password(value.uuid,value.revision,(char*)password,4));assert_zero(password,4);TEST_ASSERT_EQUAL_UINT8(0xa5,password[4]);
}
void test_omitted_password_preserves_and_explicit_clear_removes_logical_value(void){
    event_destination_t value=create();strcpy(value.description,"noncredential update");
    TEST_ASSERT_EQUAL_INT(DB_EVENT_DESTINATION_OK,db_event_destination_update(&value,value.revision,NULL,false));assert_secret(&value,secret_create);
    TEST_ASSERT_EQUAL_INT(DB_EVENT_DESTINATION_OK,db_event_destination_update(&value,value.revision,NULL,true));assert_secret(&value,"");TEST_ASSERT_FALSE(value.password_configured);
}
static int user_key_create(const char *name,user_role_t role,char key[128]){int64_t id=0;return db_auth_create_user(name,"synthetic-test-passphrase",NULL,role,true,&id)||db_auth_generate_api_key(id,key,128);}
int main(void){
    mode_t previous=umask(0077);if(!mkdtemp(directory))return 1;
    snprintf(database,sizeof(database),"%s/private.db",directory);init_logger();load_default_config(&g_config);
    if(init_database_ex(database,DB_INIT_NO_CHECK|DB_INIT_NO_BACKUP)||db_auth_init()||user_key_create("t7_11_admin",USER_ROLE_ADMIN,admin_key)||user_key_create("t7_11_viewer",USER_ROLE_VIEWER,viewer_key)||user_key_create("t7_11_user",USER_ROLE_USER,user_key))return 1;
    UNITY_BEGIN();
    RUN_TEST(test_codeql_316_create_reproduces_plaintext_in_private_database);
    RUN_TEST(test_codeql_307_replace_reproduces_plaintext_in_private_database);
    RUN_TEST(test_admin_create_get_list_update_and_audit_are_redacted);
    RUN_TEST(test_anonymous_requests_cannot_read_or_mutate);
    RUN_TEST(test_viewer_requests_cannot_read_or_mutate);
    RUN_TEST(test_standard_user_requests_cannot_read_or_mutate);
    RUN_TEST(test_runtime_getter_clears_output_on_stale_invalid_missing_and_small_buffer);
    RUN_TEST(test_omitted_password_preserves_and_explicit_clear_removes_logical_value);
    int result=UNITY_END();shutdown_database();unlink(database);
    char sidecar[280];snprintf(sidecar,sizeof(sidecar),"%s-wal",database);unlink(sidecar);snprintf(sidecar,sizeof(sidecar),"%s-shm",database);unlink(sidecar);
    rmdir(directory);umask(previous);return result;
}
