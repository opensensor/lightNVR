#define _POSIX_C_SOURCE 200809L
#include "unity.h"
#include <unistd.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <cjson/cJSON.h>
#include "core/config.h"
#include "database/db_core.h"
#include "video/onvif_discovery.h"
#include "web/api_handlers_onvif.h"
#include "web/request_response.h"
static char db_path[100];
static int device_calls;
int __wrap_discover_onvif_devices(const char *network,onvif_device_info_t *devices,int maximum){
 TEST_ASSERT_EQUAL_STRING("owned-interface",network);TEST_ASSERT_GREATER_THAN(0,maximum);device_calls++;
 memset(devices,0,sizeof(*devices));strcpy(devices->endpoint,"urn:uuid:owned-device");strcpy(devices->manufacturer,"Owned");strcpy(devices->claim_state,"unclaimed");return 1;
}
int __wrap_get_onvif_device_profiles(const char *url,const char *user,const char *password,onvif_profile_t *profiles,int maximum){
 (void)url;(void)user;(void)password;TEST_ASSERT_GREATER_THAN(0,maximum);device_calls++;
 memset(profiles,0,sizeof(*profiles));strcpy(profiles->token,"owned-profile");return 1;
}
int __wrap_add_onvif_device_as_stream(const onvif_device_info_t *device,const onvif_profile_t *profile,const char *user,const char *password,const char *name){
 (void)device;(void)profile;(void)user;(void)password;TEST_ASSERT_EQUAL_STRING("owned-added-camera",name);device_calls++;return 0;
}
void setUp(void){
 strcpy(db_path,"/tmp/lightnvr-device-unit-XXXXXX");int fd=mkstemp(db_path);TEST_ASSERT_GREATER_OR_EQUAL(0,fd);close(fd);
 TEST_ASSERT_EQUAL_INT(0,init_database(db_path));g_config.web_auth_enabled=false;g_config.demo_mode=false;device_calls=0;
}
void tearDown(void){shutdown_database();unlink(db_path);}
static cJSON *call(void(*handler)(const http_request_t*,http_response_t*),const char *body,int status){
 http_request_t req;http_request_init(&req);req.method=HTTP_METHOD_POST;strcpy(req.method_str,"POST");strcpy(req.client_ip,"127.0.0.1");req.body=(void*)body;req.body_len=strlen(body);
 http_response_t res;http_response_init(&res);handler(&req,&res);TEST_ASSERT_EQUAL_INT(status,res.status_code);cJSON *result=cJSON_Parse(res.body);TEST_ASSERT_NOT_NULL(result);http_response_free(&res);return result;
}
void test_discovery_success_uses_owned_network_seam(void){
 cJSON *result=call(handle_post_discover_onvif_devices,"{\"network\":\"owned-interface\"}",200);
 TEST_ASSERT_EQUAL_INT(1,device_calls);cJSON *device=cJSON_GetArrayItem(cJSON_GetObjectItem(result,"devices"),0);TEST_ASSERT_EQUAL_STRING("Owned",cJSON_GetObjectItem(device,"manufacturer")->valuestring);cJSON_Delete(result);
}
void test_add_response_retains_name_after_input_released(void){
 cJSON *result=call(handle_post_add_onvif_device_as_stream,"{\"device_url\":\"http://127.0.0.1/owned\",\"profile_token\":\"owned-profile\",\"stream_name\":\"owned-added-camera\"}",200);
 TEST_ASSERT_EQUAL_INT(2,device_calls);TEST_ASSERT_EQUAL_STRING("owned-added-camera",cJSON_GetObjectItem(result,"stream_name")->valuestring);cJSON_Delete(result);
}
void test_invalid_discovery_never_enters_transport(void){
 const char *values[]={"[]","{\"network\":4}","{\"network\":\"owned-interface\",\"network\":\"other\"}","{\"network\":\"owned-interface\"} false"};
 for(size_t i=0;i<sizeof(values)/sizeof(values[0]);i++){cJSON_Delete(call(handle_post_discover_onvif_devices,values[i],400));TEST_ASSERT_EQUAL_INT(0,device_calls);}
}
int main(void){UNITY_BEGIN();RUN_TEST(test_discovery_success_uses_owned_network_seam);RUN_TEST(test_add_response_retains_name_after_input_released);RUN_TEST(test_invalid_discovery_never_enters_transport);return UNITY_END();}
