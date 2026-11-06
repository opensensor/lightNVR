#include "unity.h"
#include "core/totp.h"
#include <string.h>
void setUp(void) {}
void tearDown(void) {}
static const char *rfc_secret = "GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ";
static void test_rfc6238_sha1_vectors_at_six_digits(void) {
    const int64_t times[] = {59,1111111109,1111111111,1234567890,2000000000,20000000000LL};
    const char *codes[] = {"287082","081804","050471","005924","279037","353130"};
    for (size_t i=0;i<sizeof(times)/sizeof(times[0]);++i) {
        int64_t step=-1;
        TEST_ASSERT_EQUAL_INT(0,totp_match_step(rfc_secret,codes[i],(time_t)times[i],&step));
        TEST_ASSERT_EQUAL_INT64(times[i]/30,step);
    }
}
static void test_skew_returns_the_consumable_step_and_rejects_outside_window(void) {
    int64_t step=-1;
    TEST_ASSERT_EQUAL_INT(0,totp_match_step(rfc_secret,"050471",1111111111-30,&step));
    TEST_ASSERT_EQUAL_INT64(1111111111/30,step);
    TEST_ASSERT_EQUAL_INT(0,totp_match_step(rfc_secret,"050471",1111111111+30,&step));
    TEST_ASSERT_EQUAL_INT64(1111111111/30,step);
    TEST_ASSERT_EQUAL_INT(-1,totp_match_step(rfc_secret,"050471",1111111111+60,&step));
    TEST_ASSERT_EQUAL_INT64(-1,step);
}
static void test_malformed_codes_and_empty_keys_are_rejected(void) {
    const char *codes[]={NULL,"","05047","0504710"," 50471","05047x","+50471"};
    int64_t step=123;
    for(size_t i=0;i<sizeof(codes)/sizeof(codes[0]);++i)
        TEST_ASSERT_EQUAL_INT(-1,totp_match_step(rfc_secret,codes[i],1111111111,&step));
    TEST_ASSERT_EQUAL_INT(-1,totp_match_step("","050471",1111111111,&step));
    TEST_ASSERT_EQUAL_INT(-1,totp_match_step("???","050471",1111111111,&step));
    TEST_ASSERT_EQUAL_INT(-1,totp_match_step(rfc_secret,"050471",(time_t)-1,&step));
}
static void test_generated_secrets_have_full_base32_length(void) {
    char secret[33];
    TEST_ASSERT_EQUAL_INT(0,totp_generate_secret(secret,sizeof(secret)));
    TEST_ASSERT_EQUAL_UINT(32,strlen(secret));
    TEST_ASSERT_EQUAL_UINT(32,strspn(secret,"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567"));
    TEST_ASSERT_EQUAL_INT(-1,totp_generate_secret(secret,32));
}
int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_rfc6238_sha1_vectors_at_six_digits);
    RUN_TEST(test_skew_returns_the_consumable_step_and_rejects_outside_window);
    RUN_TEST(test_malformed_codes_and_empty_keys_are_rejected);
    RUN_TEST(test_generated_secrets_have_full_base32_length);
    return UNITY_END();
}
