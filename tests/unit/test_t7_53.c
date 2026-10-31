#define _POSIX_C_SOURCE 200809L
#include <limits.h>
#include "unity.h"
/* Include the actual implementation for the arithmetic-only static helper seam.
 * Its public definitions satisfy archive references; no replacement algorithm. */
#include "../../src/core/authorization.c"

static unsigned resize_calls, load_calls, filter_loads, filter_frees;
static bool fail_resize, deny_grants, fail_filter;
static authorization_context_t *context;
static user_t user;
static fleet_camera_t camera;
void *__real_realloc(void *, size_t);
void *__wrap_realloc(void *memory, size_t bytes) {
    resize_calls++;
    if (fail_resize || bytes > 1024 * 1024) return NULL;
    return __real_realloc(memory, bytes);
}
int __wrap_db_authorization_load_user_grants(int64_t user_id, const char *action,
        authorization_grant_t **grants, int *count, int64_t *version) {
    (void)action; load_calls++; *version = 17; *count = deny_grants ? 0 : 1;
    *grants = NULL;
    if (!*count) return 0;
    *grants = calloc(1, sizeof(**grants));
    if (!*grants) return -1;
    strcpy((*grants)->scope_type, "collection");
    snprintf((*grants)->collection_uuid, sizeof((*grants)->collection_uuid),
             "00000000-0000-4000-8000-%012lld", (long long)user_id);
    strcpy((*grants)->uuid, "11111111-1111-4111-8111-111111111111");
    strcpy((*grants)->role_name, "fixture");
    return 0;
}
camera_collection_filter_result_t __wrap_camera_collection_filter_load_for_authorization(
        const char *uuid, camera_collection_filter_t *filter) {
    (void)uuid; filter_loads++; memset(filter, 0, sizeof(*filter));
    if (fail_filter) return CAMERA_COLLECTION_FILTER_DATABASE_ERROR;
    filter->active = true;
    return CAMERA_COLLECTION_FILTER_OK;
}
bool __wrap_camera_collection_filter_matches(const camera_collection_filter_t *filter,
        const fleet_camera_t *candidate) { return filter->active && candidate != NULL; }
void __wrap_camera_collection_filter_free(camera_collection_filter_t *filter) {
    filter_frees++; memset(filter, 0, sizeof(*filter));
}
void setUp(void) {
    resize_calls = load_calls = filter_loads = filter_frees = 0;
    fail_resize = deny_grants = fail_filter = false;
    context = authorization_context_create(); TEST_ASSERT_NOT_NULL(context);
    memset(&user, 0, sizeof(user)); user.id = 1; user.is_active = true;
    strcpy(user.authorization_mode, "policy");
    memset(&camera, 0, sizeof(camera));
}
void tearDown(void) {
    authorization_context_free(context); context = NULL;
    TEST_ASSERT_EQUAL_UINT(filter_loads, filter_frees);
}
static void allowed(void) {
    authorization_evaluation_t evaluation;
    TEST_ASSERT_EQUAL_INT(0, authorization_evaluate_in_context(context, &user,
        AUTHZ_LIVE_VIEW, &camera, &evaluation));
    TEST_ASSERT_EQUAL_INT(AUTHZ_DECISION_ALLOW, evaluation.decision);
    TEST_ASSERT_EQUAL_INT(AUTHZ_SOURCE_POLICY_GRANT, evaluation.source);
    TEST_ASSERT_EQUAL_INT64(17, evaluation.policy_version);
}
static void test_zero_context_and_repeated_permission_hit(void) {
    TEST_ASSERT_EQUAL_INT(0, context->grant_count);
    TEST_ASSERT_EQUAL_INT(0, context->collection_count);
    allowed(); allowed();
    TEST_ASSERT_EQUAL_UINT(1, load_calls); TEST_ASSERT_EQUAL_UINT(1, filter_loads);
    TEST_ASSERT_EQUAL_UINT(2, resize_calls);
}
static void test_both_caches_cross_first_capacity_without_permission_change(void) {
    for (int i = 1; i <= 5; i++) { user.id = i; allowed(); }
    TEST_ASSERT_EQUAL_INT(5, context->grant_count);
    TEST_ASSERT_EQUAL_INT(5, context->collection_count);
    TEST_ASSERT_EQUAL_INT(8, context->grant_capacity);
    TEST_ASSERT_EQUAL_INT(8, context->collection_capacity);
    TEST_ASSERT_EQUAL_UINT(4, resize_calls);
    for (int i = 1; i <= 5; i++) { user.id = i; allowed(); }
    TEST_ASSERT_EQUAL_UINT(5, load_calls); TEST_ASSERT_EQUAL_UINT(5, filter_loads);
}
static void test_grant_resize_failure_preserves_prior_permissions(void) {
    for (int i = 1; i <= 4; i++) { user.id = i; allowed(); }
    void *before = context->grants; fail_resize = true; user.id = 5;
    authorization_evaluation_t evaluation;
    TEST_ASSERT_EQUAL_INT(-1, authorization_evaluate_in_context(context, &user,
        AUTHZ_LIVE_VIEW, &camera, &evaluation));
    TEST_ASSERT_EQUAL_INT(AUTHZ_DECISION_DENY, evaluation.decision);
    TEST_ASSERT_EQUAL_PTR(before, context->grants);
    TEST_ASSERT_EQUAL_INT(4, context->grant_count);
    TEST_ASSERT_EQUAL_INT(4, context->grant_capacity);
    user.id = 1; allowed();
    fail_resize = false; user.id = 5; allowed();
}
static void test_collection_resize_failure_preserves_prior_filters(void) {
    for (int i = 1; i <= 4; i++) { user.id = i; allowed(); }
    /* Populate grant entry independently so failure specifically reaches the
     * collection resize, not the preceding grant resize. */
    TEST_ASSERT_NOT_NULL(context_grants(context, 5, AUTHZ_LIVE_VIEW, "live.view"));
    void *before = context->collections; fail_resize = true; user.id = 5;
    authorization_evaluation_t evaluation;
    TEST_ASSERT_EQUAL_INT(-1, authorization_evaluate_in_context(context, &user,
        AUTHZ_LIVE_VIEW, &camera, &evaluation));
    TEST_ASSERT_EQUAL_INT(AUTHZ_DECISION_DENY, evaluation.decision);
    TEST_ASSERT_EQUAL_PTR(before, context->collections);
    TEST_ASSERT_EQUAL_INT(4, context->collection_count);
    TEST_ASSERT_EQUAL_INT(4, context->collection_capacity);
    user.id = 1; allowed(); fail_resize = false; user.id = 5; allowed();
}
static void test_no_grants_default_denies_without_collection_allocation(void) {
    deny_grants = true; authorization_evaluation_t evaluation;
    TEST_ASSERT_EQUAL_INT(0, authorization_evaluate_in_context(context, &user,
        AUTHZ_LIVE_VIEW, &camera, &evaluation));
    TEST_ASSERT_EQUAL_INT(AUTHZ_DECISION_DENY, evaluation.decision);
    TEST_ASSERT_EQUAL_UINT(0, filter_loads);
}
static void test_failed_collection_stays_failed_and_cleans_up(void) {
    fail_filter = true; authorization_evaluation_t evaluation;
    for (int i = 0; i < 2; i++) {
        TEST_ASSERT_EQUAL_INT(-1, authorization_evaluate_in_context(context, &user,
            AUTHZ_LIVE_VIEW, &camera, &evaluation));
        TEST_ASSERT_EQUAL_INT(AUTHZ_DECISION_DENY, evaluation.decision);
    }
    TEST_ASSERT_EQUAL_UINT(1, filter_loads);
}
static void test_growth_zero_and_small_capacity(void) {
    int capacity = 0;
    void *memory = grow_array(NULL, &capacity, 1); TEST_ASSERT_NOT_NULL(memory);
    TEST_ASSERT_EQUAL_INT(4, capacity);
    memory = grow_array(memory, &capacity, 1); TEST_ASSERT_NOT_NULL(memory);
    TEST_ASSERT_EQUAL_INT(8, capacity); free(memory);
}
static void test_largest_nonoverflowing_double_reaches_allocator_failure_unchanged(void) {
    int capacity = INT_MAX / 2; fail_resize = true;
    TEST_ASSERT_NULL(grow_array(NULL, &capacity, 1));
    TEST_ASSERT_EQUAL_INT(INT_MAX / 2, capacity); TEST_ASSERT_EQUAL_UINT(1, resize_calls);
}
static void test_signed_doubling_overflow_rejected_before_allocator(void) {
    int capacity = INT_MAX / 2 + 1;
    TEST_ASSERT_NULL(grow_array(NULL, &capacity, 1));
    TEST_ASSERT_EQUAL_INT(INT_MAX / 2 + 1, capacity); TEST_ASSERT_EQUAL_UINT(0, resize_calls);
}
static void test_size_multiplication_overflow_rejected_before_allocator(void) {
    int capacity = 4;
    void *result = grow_array(NULL, &capacity, SIZE_MAX / 8 + 1);
    bool rejected = result == NULL;
    free(result);
    TEST_ASSERT_TRUE(rejected);
    TEST_ASSERT_EQUAL_INT(4, capacity); TEST_ASSERT_EQUAL_UINT(0, resize_calls);
}
int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_zero_context_and_repeated_permission_hit);
    RUN_TEST(test_both_caches_cross_first_capacity_without_permission_change);
    RUN_TEST(test_grant_resize_failure_preserves_prior_permissions);
    RUN_TEST(test_collection_resize_failure_preserves_prior_filters);
    RUN_TEST(test_no_grants_default_denies_without_collection_allocation);
    RUN_TEST(test_failed_collection_stays_failed_and_cleans_up);
    RUN_TEST(test_growth_zero_and_small_capacity);
    RUN_TEST(test_largest_nonoverflowing_double_reaches_allocator_failure_unchanged);
    RUN_TEST(test_signed_doubling_overflow_rejected_before_allocator);
    RUN_TEST(test_size_multiplication_overflow_rejected_before_allocator);
    return UNITY_END();
}
