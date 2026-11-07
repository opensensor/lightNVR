/* Real generation registry and worker, with a small SQLite inventory fixture. */
#include "../../src/web/api_handlers_motion.c"
#include "unity.h"

static sqlite3 *inventory;
static bool fail_open;
static int opens, stop_effects, rows_seen;

int __wrap_db_open_readonly_connection(sqlite3 **connection) {
    opens++;
    if (fail_open) return -1;
    *connection = inventory;
    return 0;
}
void __wrap_db_close_readonly_connection(sqlite3 *connection) { (void)connection; }
int __wrap_close_external_motion_detections(const char *uuid, const char *name, time_t end) {
    (void)uuid; (void)name; (void)end; stop_effects++; return 0;
}
void __wrap_unified_detection_notify_motion(const char *name, bool active) {
    (void)name; (void)active; stop_effects++;
}
int __wrap_process_motion_event(const char *name, bool active, time_t now, bool propagated) {
    (void)name; (void)active; (void)now; (void)propagated; stop_effects++; return 0;
}

static void sql(const char *statement) {
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(inventory, statement, NULL, NULL, NULL));
}
void setUp(void) {
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_open(":memory:", &inventory));
    sql("CREATE TABLE streams(name TEXT PRIMARY KEY, enabled INTEGER DEFAULT 1);");
    g_pulse_gen_count = 0;
    g_pulse_gen_sequence = 0;
    memset(g_pulse_gen, 0, sizeof(g_pulse_gen));
    fail_open = false;
    opens = stop_effects = rows_seen = 0;
}
void tearDown(void) {
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_close(inventory));
}

static uint64_t increment(const char *name) {
    pthread_mutex_lock(&g_pulse_gen_mutex);
    uint64_t generation = pulse_gen_increment(name);
    pthread_mutex_unlock(&g_pulse_gen_mutex);
    return generation;
}
static void fill_registry(const char *first_name) {
    TEST_ASSERT_EQUAL_UINT64(1, increment(first_name));
    for (int i = 1; i < MAX_STREAMS; i++) {
        char name[MAX_STREAM_NAME];
        snprintf(name, sizeof(name), "retired-%d", i);
        TEST_ASSERT_NOT_EQUAL(0, increment(name));
    }
    TEST_ASSERT_EQUAL_INT(MAX_STREAMS, g_pulse_gen_count);
}
static void worker(uint64_t generation) {
    motion_pulse_task_t *task = calloc(1, sizeof(*task));
    TEST_ASSERT_NOT_NULL(task);
    safe_strcpy(task->stream_name, "camera", sizeof(task->stream_name), 0);
    safe_strcpy(task->camera_uuid, "captured-camera", sizeof(task->camera_uuid), 0);
    task->generation = generation;
    task->trigger_recording = true;
    motion_pulse_worker(task);
}

static void test_reclaim_removed_names_preserves_disabled_camera_tokens(void) {
    fill_registry("disabled-camera");
    sql("INSERT INTO streams VALUES ('disabled-camera',0),('new-camera',1);");
    uint64_t token = increment("new-camera");
    TEST_ASSERT_TRUE(token > MAX_STREAMS);
    TEST_ASSERT_EQUAL_INT(1, opens);
    TEST_ASSERT_EQUAL_INT(2, g_pulse_gen_count);
    TEST_ASSERT_EQUAL_UINT64(1, pulse_gen_get("disabled-camera"));
    TEST_ASSERT_EQUAL_UINT64(token, pulse_gen_get("new-camera"));
    TEST_ASSERT_EQUAL_UINT64(0, pulse_gen_get("retired-1"));
}

static void test_recreated_name_cannot_match_sleeping_worker_token(void) {
    fill_registry("camera");
    sql("INSERT INTO streams(name) VALUES ('replacement');");
    TEST_ASSERT_NOT_EQUAL(0, increment("replacement"));
    TEST_ASSERT_EQUAL_UINT64(0, pulse_gen_get("camera"));
    sql("INSERT INTO streams(name) VALUES ('camera');");
    uint64_t new_token = increment("camera");
    TEST_ASSERT_TRUE(new_token > MAX_STREAMS);
    worker(1);
    TEST_ASSERT_EQUAL_INT(0, stop_effects);
    worker(new_token);
    TEST_ASSERT_EQUAL_INT(3, stop_effects);
}

static void test_full_live_inventory_preserves_all_entries(void) {
    fill_registry("camera");
    for (int i = 0; i < g_pulse_gen_count; i++) {
        char statement[160];
        snprintf(statement, sizeof(statement), "INSERT INTO streams(name) VALUES ('%s');", g_pulse_gen[i].name);
        sql(statement);
    }
    TEST_ASSERT_EQUAL_UINT64(0, increment("overflow"));
    TEST_ASSERT_EQUAL_INT(MAX_STREAMS, g_pulse_gen_count);
    TEST_ASSERT_EQUAL_UINT64(1, pulse_gen_get("camera"));
    opens = 0;
    TEST_ASSERT_TRUE(increment("camera") > MAX_STREAMS);
    TEST_ASSERT_EQUAL_INT(0, opens);
}

static void test_inventory_errors_never_prune_existing_tokens(void) {
    fill_registry("camera");
    fail_open = true;
    TEST_ASSERT_EQUAL_UINT64(0, increment("new-camera"));
    TEST_ASSERT_EQUAL_INT(MAX_STREAMS, g_pulse_gen_count);
    fail_open = false;
    sql("DROP TABLE streams;");
    TEST_ASSERT_EQUAL_UINT64(0, increment("new-camera"));
    TEST_ASSERT_EQUAL_INT(MAX_STREAMS, g_pulse_gen_count);
    TEST_ASSERT_EQUAL_UINT64(1, pulse_gen_get("camera"));
}

static void fail_second_row(sqlite3_context *context, int argc, sqlite3_value **argv) {
    (void)argc;
    if (++rows_seen == 2) sqlite3_result_error(context, "fixture read failure", -1);
    else sqlite3_result_value(context, argv[0]);
}
static void test_partial_inventory_read_never_prunes(void) {
    fill_registry("camera");
    sql("INSERT INTO streams(name) VALUES ('camera'),('retired-1'); ALTER TABLE streams RENAME TO names;");
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_create_function(inventory, "fixture_name", 1,
        SQLITE_UTF8, NULL, fail_second_row, NULL, NULL));
    sql("CREATE VIEW streams AS SELECT fixture_name(name) AS name FROM names;");
    TEST_ASSERT_EQUAL_UINT64(0, increment("new-camera"));
    TEST_ASSERT_EQUAL_INT(2, rows_seen);
    TEST_ASSERT_EQUAL_INT(MAX_STREAMS, g_pulse_gen_count);
    TEST_ASSERT_EQUAL_UINT64(1, pulse_gen_get("camera"));
}

static void test_generation_exhaustion_never_recycles_a_token(void) {
    TEST_ASSERT_EQUAL_UINT64(1, increment("camera"));
    g_pulse_gen_sequence = UINT64_MAX;
    TEST_ASSERT_EQUAL_UINT64(0, increment("camera"));
    TEST_ASSERT_EQUAL_UINT64(0, increment("another-camera"));
    TEST_ASSERT_EQUAL_UINT64(1, pulse_gen_get("camera"));
    TEST_ASSERT_EQUAL_INT(1, g_pulse_gen_count);
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_reclaim_removed_names_preserves_disabled_camera_tokens);
    RUN_TEST(test_recreated_name_cannot_match_sleeping_worker_token);
    RUN_TEST(test_full_live_inventory_preserves_all_entries);
    RUN_TEST(test_inventory_errors_never_prune_existing_tokens);
    RUN_TEST(test_partial_inventory_read_never_prunes);
    RUN_TEST(test_generation_exhaustion_never_recycles_a_token);
    return UNITY_END();
}
