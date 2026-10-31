#define _POSIX_C_SOURCE 200809L
#include <limits.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <unistd.h>
#include <cjson/cJSON.h>
#include "unity.h"
#include "core/event_router.h"
#include "database/db_event_route_suppression.h"

#define FIXTURE_ZONE "/usr/share/zoneinfo/Etc/T7_28"
#define MAX_TRANSITIONS 4096U /* Existing production contract, not a new limit. */
#define MAX_FILE_BYTES (1024U * 1024U)
#define EVENT_TYPE "io.lightnvr.camera.offline.v1"
static char directory[128], fixture[256];
static event_route_t route;
static event_envelope_t event;
static event_route_delivery_plan_t plan;
static uint64_t generation = 1;
static bool monitor;
static unsigned transition_calls, oversized_calls, file_buffer_calls;
static unsigned fail_transition_call;
static size_t allocation_counts[2], allocation_widths[2];
static void *transition_blocks[2];
static int live_blocks;

int __wrap_db_event_route_count(void) { return 1; }
int __wrap_db_event_route_list(event_route_t *output, int maximum) {
    if (maximum < 1) return -1;
    *output = route;
    return 1;
}
uint64_t __wrap_db_event_route_generation(void) { return generation; }
uint64_t __wrap_db_event_destination_generation(void) { return 1; }
int __wrap_db_event_route_suppression_prune(int64_t before, int maximum, int *deleted) {
    (void)before; (void)maximum; *deleted = 0; return 0;
}
FILE *__real_fopen(const char *, const char *);
FILE *__wrap_fopen(const char *name, const char *mode) {
    return __real_fopen(strcmp(name, FIXTURE_ZONE) == 0 ? fixture : name, mode);
}
int __real_stat(const char *, struct stat *);
int __wrap_stat(const char *name, struct stat *info) {
    return __real_stat(strcmp(name, FIXTURE_ZONE) == 0 ? fixture : name, info);
}
void *__real_calloc(size_t, size_t);
void *__wrap_calloc(size_t count, size_t width) {
    if (monitor && (width == sizeof(int64_t) || width == sizeof(unsigned char))) {
        unsigned index = transition_calls++;
        if (count > MAX_TRANSITIONS || count > SIZE_MAX / width) {
            oversized_calls++;
            return NULL; /* Never attempt an unbounded allocation in a probe. */
        }
        if (index < 2) { allocation_counts[index] = count; allocation_widths[index] = width; }
        if (fail_transition_call == index + 1) return NULL;
        void *result = __real_calloc(count, width);
        if (result && index < 2) { transition_blocks[index] = result; live_blocks++; }
        return result;
    }
    return __real_calloc(count, width);
}
void *__real_malloc(size_t);
void *__wrap_malloc(size_t size) {
    if (monitor && size >= MAX_FILE_BYTES) {
        file_buffer_calls++;
        if (size > MAX_FILE_BYTES) { oversized_calls++; return NULL; }
    }
    return __real_malloc(size);
}
void __real_free(void *);
void __wrap_free(void *memory) {
    if (memory) for (unsigned i = 0; i < 2; i++) {
        if (transition_blocks[i] == memory) { transition_blocks[i] = NULL; live_blocks--; }
    }
    __real_free(memory);
}
static void be32(FILE *file, uint32_t value) {
    unsigned char bytes[] = {(unsigned char)(value >> 24), (unsigned char)(value >> 16),
                            (unsigned char)(value >> 8), (unsigned char)value};
    TEST_ASSERT_EQUAL_UINT(4, fwrite(bytes, 1, 4, file));
}
static void header(FILE *file, char version, uint32_t transitions, uint32_t types,
                   uint32_t leaps, uint32_t characters) {
    unsigned char prefix[20] = {'T','Z','i','f',0}; prefix[4] = (unsigned char)version;
    TEST_ASSERT_EQUAL_UINT(sizeof(prefix), fwrite(prefix, 1, sizeof(prefix), file));
    be32(file, 0); be32(file, 0); be32(file, leaps);
    be32(file, transitions); be32(file, types); be32(file, characters);
}
static void block(FILE *file, uint32_t count, uint32_t types, unsigned width,
                  unsigned char index) {
    /* Invalid raw counts deliberately have no oversized backing body. */
    uint32_t actual_count = count <= MAX_TRANSITIONS ? count : 0;
    uint32_t actual_types = types <= 256 ? types : 0;
    for (uint32_t i = 0; i < actual_count; i++) {
        if (width == 8) be32(file, 0);
        be32(file, i);
    }
    for (uint32_t i = 0; i < actual_count; i++) fputc(index, file);
    for (uint32_t i = 0; i < actual_types; i++) {
        be32(file, 0); fputc(0, file); fputc(0, file);
    }
    TEST_ASSERT_EQUAL_UINT(4, fwrite("UTC", 1, 4, file));
}
static void write_fixture(uint32_t count, uint32_t types, bool extended,
                          unsigned char index) {
    FILE *file = fopen(fixture, "wb"); TEST_ASSERT_NOT_NULL(file);
    if (extended) {
        header(file, '2', 0, 1, 0, 4); block(file, 0, 1, 4, 0);
    }
    header(file, extended ? '2' : '\0', count, types, 0, 4);
    block(file, count, types, extended ? 8 : 4, index);
    TEST_ASSERT_EQUAL_INT(0, fclose(file));
}
void setUp(void) {
    event_router_shutdown();
    memset(&plan, 0, sizeof(plan)); memset(&event, 0, sizeof(event));
    monitor = false; transition_calls = oversized_calls = file_buffer_calls = 0;
    fail_transition_call = 0; live_blocks = 0;
    memset(transition_blocks, 0, sizeof(transition_blocks));
    memset(allocation_counts, 0, sizeof(allocation_counts));
    memset(allocation_widths, 0, sizeof(allocation_widths));
    strcpy(directory, "/tmp/lightnvr_t7_28_XXXXXX"); TEST_ASSERT_NOT_NULL(mkdtemp(directory));
    snprintf(fixture, sizeof(fixture), "%s/timezone", directory);
    write_fixture(1, 1, false, 0);
    memset(&route, 0, sizeof(route));
    strcpy(route.uuid, "11111111-1111-4111-8111-111111111111");
    strcpy(route.name, "TZif boundary"); route.enabled = true; route.revision = 1;
    strcpy(route.destination_key, EVENT_ROUTE_DEFAULT_DESTINATION);
    strcpy(route.scope_type, "all"); strcpy(route.predicate_json, "{\"version\":1}");
    strcpy(route.event_types[0], EVENT_TYPE); route.event_type_count = 1;
    strcpy(route.schedule_json, "{\"version\":1,\"timezone\":\"Etc/T7_28\",\"windows\":[{\"days\":[0,1,2,3,4,5,6],\"start\":\"00:00\",\"end\":\"23:59\"}]}");
    generation++;
    cJSON *data = cJSON_CreateObject(); TEST_ASSERT_NOT_NULL(data);
    cJSON_AddStringToObject(data, "stream_name", "camera");
    cJSON_AddStringToObject(data, "reason", "fixture");
    cJSON_AddNumberToObject(data, "consecutive_failures", 1);
    int result = event_envelope_create(&event, EVENT_TYPE,
        "urn:lightnvr:11111111-1111-4111-8111-111111111111",
        "camera/22222222-2222-4222-8222-222222222222", 1786991400,
        data, NULL, 0);
    cJSON_Delete(data); TEST_ASSERT_EQUAL_INT(0, result);
}
void tearDown(void) {
    monitor = false;
    event_route_delivery_plan_clear(&plan); event_envelope_clear(&event);
    event_router_shutdown();
    unlink(fixture); rmdir(directory);
    TEST_ASSERT_EQUAL_INT(0, live_blocks);
}
static void evaluate(event_router_result_t expected) {
    event_route_t before = route;
    event_envelope_t original_event = event;
    monitor = true;
    event_router_result_t result = event_router_evaluate_delivery(&event, &plan);
    monitor = false;
    TEST_ASSERT_EQUAL_INT(expected, result);
    TEST_ASSERT_EQUAL_MEMORY(&before, &route, sizeof(route));
    TEST_ASSERT_EQUAL_MEMORY(&original_event, &event, sizeof(event));
    TEST_ASSERT_EQUAL_UINT(0, oversized_calls);
    if (expected == EVENT_ROUTER_MATCH) {
        TEST_ASSERT_EQUAL_UINT(1, plan.count);
        TEST_ASSERT_EQUAL_STRING(EVENT_ROUTE_DEFAULT_DESTINATION, plan.entries[0].destination_key);
        TEST_ASSERT_EQUAL_STRING(route.uuid, plan.entries[0].route_uuid);
    } else {
        TEST_ASSERT_EQUAL_UINT(0, plan.count); TEST_ASSERT_NULL(plan.entries);
    }
}
static void reject_without_transition_allocation(void) {
    evaluate(EVENT_ROUTER_ERROR);
    TEST_ASSERT_EQUAL_UINT(0, transition_calls); TEST_ASSERT_EQUAL_INT(0, live_blocks);
}
static void test_ordinary_delivery_allocates_two_bounded_arrays(void) {
    evaluate(EVENT_ROUTER_MATCH);
    TEST_ASSERT_EQUAL_UINT(2, transition_calls);
    TEST_ASSERT_EQUAL_UINT(1, allocation_counts[0]); TEST_ASSERT_EQUAL_UINT(8, allocation_widths[0]);
    TEST_ASSERT_EQUAL_UINT(1, allocation_counts[1]); TEST_ASSERT_EQUAL_UINT(1, allocation_widths[1]);
}
static void test_zero_transitions_needs_no_arrays(void) {
    write_fixture(0, 1, false, 0); evaluate(EVENT_ROUTER_MATCH);
    TEST_ASSERT_EQUAL_UINT(0, transition_calls);
}
static void test_maximum_transitions_retains_delivery(void) {
    write_fixture(MAX_TRANSITIONS, 1, false, 0); evaluate(EVENT_ROUTER_MATCH);
    TEST_ASSERT_EQUAL_UINT(2, transition_calls);
    TEST_ASSERT_EQUAL_UINT(MAX_TRANSITIONS, allocation_counts[0]);
    TEST_ASSERT_EQUAL_UINT(MAX_TRANSITIONS, allocation_counts[1]);
}
static void test_one_past_transition_limit_rejected_before_arrays(void) {
    write_fixture(MAX_TRANSITIONS + 1, 1, false, 0); reject_without_transition_allocation();
}
static void test_maximum_raw_count_rejected_before_arrays(void) {
    write_fixture(UINT32_MAX, 1, false, 0); reject_without_transition_allocation();
}
static void test_signed_boundary_count_rejected_before_arrays(void) {
    write_fixture((uint32_t)INT_MAX + 1U, 1, false, 0); reject_without_transition_allocation();
}
static void test_zero_types_rejected_before_arrays(void) {
    write_fixture(1, 0, false, 0); reject_without_transition_allocation();
}
static void test_maximum_type_and_index_are_valid(void) {
    write_fixture(1, 256, false, 255); evaluate(EVENT_ROUTER_MATCH);
}
static void test_one_past_type_limit_rejected_before_arrays(void) {
    write_fixture(1, 257, false, 0); reject_without_transition_allocation();
}
static void test_out_of_range_transition_index_frees_both_arrays(void) {
    write_fixture(1, 1, false, 1); evaluate(EVENT_ROUTER_ERROR);
    TEST_ASSERT_EQUAL_UINT(2, transition_calls); TEST_ASSERT_EQUAL_INT(0, live_blocks);
}
static void test_extended_second_header_maximum_is_bounded(void) {
    write_fixture(MAX_TRANSITIONS, 1, true, 0); evaluate(EVENT_ROUTER_MATCH);
    TEST_ASSERT_EQUAL_UINT(2, transition_calls);
    TEST_ASSERT_EQUAL_UINT(MAX_TRANSITIONS, allocation_counts[0]);
}
static void test_extended_second_header_one_past_rejected(void) {
    write_fixture(MAX_TRANSITIONS + 1, 1, true, 0); reject_without_transition_allocation();
}
static void test_truncated_data_block_rejected_before_arrays(void) {
    TEST_ASSERT_EQUAL_INT(0, truncate(fixture, 54)); reject_without_transition_allocation();
}
static void test_huge_leap_block_rejected_before_arrays(void) {
    FILE *file = fopen(fixture, "wb"); TEST_ASSERT_NOT_NULL(file);
    header(file, '\0', 1, 1, INT_MAX, INT_MAX); block(file, 1, 1, 4, 0);
    TEST_ASSERT_EQUAL_INT(0, fclose(file)); reject_without_transition_allocation();
}
static void test_existing_file_size_limit_accepts_maximum_buffer(void) {
    /* A valid v1 data prefix padded to the parser's existing file-size limit. */
    TEST_ASSERT_EQUAL_INT(0, truncate(fixture, MAX_FILE_BYTES)); evaluate(EVENT_ROUTER_MATCH);
    TEST_ASSERT_EQUAL_UINT(1, file_buffer_calls);
}
static void test_file_one_past_limit_rejected_before_buffer_allocation(void) {
    TEST_ASSERT_EQUAL_INT(0, truncate(fixture, MAX_FILE_BYTES + 1));
    reject_without_transition_allocation(); TEST_ASSERT_EQUAL_UINT(0, file_buffer_calls);
}
static void test_first_array_allocation_failure_releases_partial_state(void) {
    fail_transition_call = 1; evaluate(EVENT_ROUTER_ERROR);
    TEST_ASSERT_EQUAL_INT(0, live_blocks);
}
static void test_second_array_allocation_failure_releases_partial_state(void) {
    fail_transition_call = 2; evaluate(EVENT_ROUTER_ERROR);
    TEST_ASSERT_EQUAL_INT(0, live_blocks);
}
static void test_invalid_timezone_recovery_after_definition_reload(void) {
    write_fixture(MAX_TRANSITIONS + 1, 1, false, 0); reject_without_transition_allocation();
    event_route_delivery_plan_clear(&plan);
    write_fixture(1, 1, false, 0); generation++;
    evaluate(EVENT_ROUTER_MATCH);
    TEST_ASSERT_EQUAL_UINT(2, transition_calls);
}
int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_ordinary_delivery_allocates_two_bounded_arrays);
    RUN_TEST(test_zero_transitions_needs_no_arrays);
    RUN_TEST(test_maximum_transitions_retains_delivery);
    RUN_TEST(test_one_past_transition_limit_rejected_before_arrays);
    RUN_TEST(test_maximum_raw_count_rejected_before_arrays);
    RUN_TEST(test_signed_boundary_count_rejected_before_arrays);
    RUN_TEST(test_zero_types_rejected_before_arrays);
    RUN_TEST(test_maximum_type_and_index_are_valid);
    RUN_TEST(test_one_past_type_limit_rejected_before_arrays);
    RUN_TEST(test_out_of_range_transition_index_frees_both_arrays);
    RUN_TEST(test_extended_second_header_maximum_is_bounded);
    RUN_TEST(test_extended_second_header_one_past_rejected);
    RUN_TEST(test_truncated_data_block_rejected_before_arrays);
    RUN_TEST(test_huge_leap_block_rejected_before_arrays);
    RUN_TEST(test_existing_file_size_limit_accepts_maximum_buffer);
    RUN_TEST(test_file_one_past_limit_rejected_before_buffer_allocation);
    RUN_TEST(test_first_array_allocation_failure_releases_partial_state);
    RUN_TEST(test_second_array_allocation_failure_releases_partial_state);
    RUN_TEST(test_invalid_timezone_recovery_after_definition_reload);
    return UNITY_END();
}
