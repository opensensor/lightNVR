/**
 * @file test_storage_deletion_ledger.c
 * @brief Deletion ledger hot path: per-deletion scoping, derivative
 *        journaling, completed-row pruning and the indexes that serve them.
 *
 * Layer 2: real SQLite through lightnvr_lib with a temporary database built by
 * the migration runner, so index assertions exercise the shipped migrations.
 */

#define _POSIX_C_SOURCE 200809L

#include <sqlite3.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <time.h>
#include <unistd.h>

#include "core/config.h"
#include "core/path_utils.h"
#include "database/db_core.h"
#include "database/db_recordings.h"
#include "storage/storage_deletion.h"
#include "unity.h"
#include "utils/strings.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_storage_deletion_ledger.db"

static char storage_root[] = "/tmp/lightnvr-deletion-ledger-XXXXXX";
static const char *payload = "ledger payload";

static void sql(const char *statement) {
    char *error = NULL;
    pthread_mutex_lock(get_db_mutex());
    int result = sqlite3_exec(get_db_handle(), statement, NULL, NULL, &error);
    pthread_mutex_unlock(get_db_mutex());
    if (error) fprintf(stderr, "SQL: %s\n", error);
    sqlite3_free(error);
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, result);
}

static int scalar(const char *query) {
    sqlite3_stmt *statement = NULL;
    pthread_mutex_lock(get_db_mutex());
    int prepared = sqlite3_prepare_v2(get_db_handle(), query, -1, &statement, NULL);
    int step = prepared == SQLITE_OK ? sqlite3_step(statement) : prepared;
    int result = step == SQLITE_ROW ? sqlite3_column_int(statement, 0) : 0;
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(get_db_mutex());
    TEST_ASSERT_EQUAL_INT_MESSAGE(SQLITE_OK, prepared, query);
    TEST_ASSERT_EQUAL_INT_MESSAGE(SQLITE_ROW, step, query);
    return result;
}

/* Concatenate every EXPLAIN QUERY PLAN detail line for a statement. */
static void query_plan(const char *statement, char *plan, size_t size) {
    char query[2048];
    snprintf(query, sizeof(query), "EXPLAIN QUERY PLAN %s", statement);
    plan[0] = '\0';
    sqlite3_stmt *explain = NULL;
    pthread_mutex_lock(get_db_mutex());
    int prepared = sqlite3_prepare_v2(get_db_handle(), query, -1, &explain, NULL);
    if (prepared == SQLITE_OK) {
        while (sqlite3_step(explain) == SQLITE_ROW) {
            const char *detail = (const char *)sqlite3_column_text(explain, 3);
            size_t used = strlen(plan);
            snprintf(plan + used, size - used, "%s\n", detail ? detail : "");
        }
    }
    if (explain) sqlite3_finalize(explain);
    pthread_mutex_unlock(get_db_mutex());
    TEST_ASSERT_EQUAL_INT_MESSAGE(SQLITE_OK, prepared, statement);
}

static void write_file(const char *path) {
    TEST_ASSERT_EQUAL_INT(0, ensure_path(path));
    FILE *file = fopen(path, "wb");
    TEST_ASSERT_NOT_NULL(file);
    size_t length = strlen(payload);
    TEST_ASSERT_EQUAL_size_t(length, fwrite(payload, 1, length, file));
    TEST_ASSERT_EQUAL_INT(0, fclose(file));
}

static uint64_t add_recording(const char *name, char path[MAX_PATH_LENGTH]) {
    snprintf(path, MAX_PATH_LENGTH, "%s/recordings/ledger-camera/%s.mp4", storage_root, name);
    write_file(path);
    recording_metadata_t recording;
    memset(&recording, 0, sizeof(recording));
    safe_strcpy(recording.stream_name, "ledger-camera", sizeof(recording.stream_name), 0);
    safe_strcpy(recording.camera_uuid, "22222222-2222-4222-8222-222222222222",
                sizeof(recording.camera_uuid), 0);
    safe_strcpy(recording.file_path, path, sizeof(recording.file_path), 0);
    safe_strcpy(recording.codec, "h264", sizeof(recording.codec), 0);
    safe_strcpy(recording.trigger_type, "continuous", sizeof(recording.trigger_type), 0);
    recording.start_time = 100;
    recording.end_time = 200;
    recording.size_bytes = strlen(payload);
    recording.width = 1280;
    recording.height = 720;
    recording.fps = 15;
    recording.is_complete = true;
    recording.retention_override_days = -1;
    recording.retention_tier = 2;
    recording.disk_pressure_eligible = true;
    uint64_t id = add_recording_metadata(&recording);
    TEST_ASSERT_NOT_EQUAL_UINT64(0, id);
    return id;
}

/* Seed a ledger entry directly: `objects` rows in `state`, completed_at as
 * given (0 leaves it NULL), and the recording flagged deletion_pending. */
static void seed_deletion(const char *uuid, uint64_t recording_id, int objects,
                          const char *state, long long completed_at) {
    char statement[1024], completed[32] = "NULL";
    if (completed_at) snprintf(completed, sizeof(completed), "%lld", completed_at);
    snprintf(statement, sizeof(statement),
             "INSERT INTO storage_deletions(uuid,recording_id,reason,completed_at) "
             "VALUES('%s',%llu,'seed',%s);", uuid, (unsigned long long)recording_id, completed);
    sql(statement);
    for (int i = 0; i < objects; i++) {
        snprintf(statement, sizeof(statement),
                 "INSERT INTO storage_deletion_objects(deletion_uuid,file_path,state,next_attempt_at) "
                 "VALUES('%s','/nonexistent/%s-%d','%s',%s);", uuid, uuid, i, state,
                 strcmp(state, "completed") ? "9999999999" : "0");
        sql(statement);
    }
    snprintf(statement, sizeof(statement),
             "UPDATE recordings SET deletion_pending=1 WHERE id=%llu;", (unsigned long long)recording_id);
    sql(statement);
}

void setUp(void) {
    sql("DELETE FROM storage_deletion_objects;DELETE FROM storage_deletions;"
        "DELETE FROM detections;DELETE FROM recordings;");
}

void tearDown(void) {}

static void test_missing_derivatives_create_one_object_row(void) {
    char path[MAX_PATH_LENGTH];
    uint64_t id = add_recording("plain", path);
    uint64_t removed = 0;
    TEST_ASSERT_EQUAL_INT(0, storage_recording_delete(id, "unit", &removed));
    TEST_ASSERT_EQUAL_UINT64(strlen(payload), removed);
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM storage_deletion_objects;"));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM storage_deletion_objects WHERE state='completed';"));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM storage_deletions;"));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM storage_deletions WHERE completed_at IS NOT NULL;"));
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT count(*) FROM recordings;"));
    TEST_ASSERT_NOT_EQUAL_INT(0, access(path, F_OK));
}

static void test_existing_derivatives_are_journaled_and_removed(void) {
    char path[MAX_PATH_LENGTH], thumbnail[MAX_PATH_LENGTH], transcode[MAX_PATH_LENGTH];
    uint64_t id = add_recording("derived", path);
    snprintf(thumbnail, sizeof(thumbnail), "%s/thumbnails/%llu_1.jpg", storage_root, (unsigned long long)id);
    snprintf(transcode, sizeof(transcode), "%s/transcoded/%llu.mp4", storage_root, (unsigned long long)id);
    write_file(thumbnail);
    write_file(transcode);
    TEST_ASSERT_EQUAL_INT(0, storage_recording_delete(id, "unit", NULL));
    TEST_ASSERT_EQUAL_INT(3, scalar("SELECT count(*) FROM storage_deletion_objects;"));
    TEST_ASSERT_EQUAL_INT(3, scalar("SELECT count(*) FROM storage_deletion_objects WHERE state='completed';"));
    TEST_ASSERT_NOT_EQUAL_INT(0, access(path, F_OK));
    TEST_ASSERT_NOT_EQUAL_INT(0, access(thumbnail, F_OK));
    TEST_ASSERT_NOT_EQUAL_INT(0, access(transcode, F_OK));
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT count(*) FROM recordings;"));
}

static void test_deletion_finalizes_only_its_own_ledger_entry(void) {
    char path[MAX_PATH_LENGTH];
    uint64_t first = add_recording("pending-1", path);
    uint64_t second = add_recording("pending-2", path);
    uint64_t third = add_recording("pending-3", path);
    uint64_t target = add_recording("target", path);
    /* Three unrelated deletions whose objects are all done but which were never
     * finalized: the old global finalize would have closed them as a side
     * effect of any other deletion. */
    seed_deletion("unrelated-1", first, 1, "completed", 0);
    seed_deletion("unrelated-2", second, 2, "completed", 0);
    seed_deletion("unrelated-3", third, 1, "completed", 0);
    TEST_ASSERT_EQUAL_INT(3, scalar("SELECT count(*) FROM recordings WHERE deletion_pending=1;"));

    TEST_ASSERT_EQUAL_INT(0, storage_recording_delete(target, "unit", NULL));

    TEST_ASSERT_EQUAL_INT(3, scalar("SELECT count(*) FROM storage_deletions "
                                    "WHERE uuid LIKE 'unrelated-%' AND completed_at IS NULL;"));
    TEST_ASSERT_EQUAL_INT(4, scalar("SELECT count(*) FROM storage_deletion_objects "
                                    "WHERE deletion_uuid LIKE 'unrelated-%';"));
    TEST_ASSERT_EQUAL_INT(3, scalar("SELECT count(*) FROM recordings WHERE deletion_pending=1;"));
    TEST_ASSERT_EQUAL_INT(3, scalar("SELECT count(*) FROM recordings;"));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM storage_deletions "
                                    "WHERE uuid NOT LIKE 'unrelated-%' AND completed_at IS NOT NULL;"));

    /* The explicit sweep is what recovers such interrupted deletions. */
    TEST_ASSERT_EQUAL_INT(0, storage_deletion_finalize_all());
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT count(*) FROM storage_deletions WHERE completed_at IS NULL;"));
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT count(*) FROM recordings;"));
}

static void test_prune_completed_removes_old_rows_only(void) {
    char path[MAX_PATH_LENGTH];
    uint64_t a = add_recording("old-a", path);
    uint64_t b = add_recording("old-b", path);
    uint64_t c = add_recording("fresh", path);
    uint64_t d = add_recording("open", path);
    long long now = (long long)time(NULL);
    seed_deletion("old-1", a, 2, "completed", now - 10 * 86400);
    seed_deletion("old-2", b, 1, "completed", now - 8 * 86400);
    seed_deletion("fresh-1", c, 1, "completed", now - 1 * 86400 - 3600);
    seed_deletion("open-1", d, 1, "pending", 0);
    TEST_ASSERT_EQUAL_INT(4, scalar("SELECT count(*) FROM storage_deletions;"));
    TEST_ASSERT_EQUAL_INT(5, scalar("SELECT count(*) FROM storage_deletion_objects;"));

    /* Bounded: only one row per call when asked for one. */
    TEST_ASSERT_EQUAL_INT(1, storage_deletion_prune_completed(7, 1));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM storage_deletions WHERE uuid LIKE 'old-%';"));
    /* Oldest first. */
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT count(*) FROM storage_deletions WHERE uuid='old-1';"));
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT count(*) FROM storage_deletion_objects WHERE deletion_uuid='old-1';"));

    TEST_ASSERT_EQUAL_INT(1, storage_deletion_prune_completed(7, 100));
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT count(*) FROM storage_deletions WHERE uuid LIKE 'old-%';"));
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT count(*) FROM storage_deletion_objects WHERE deletion_uuid LIKE 'old-%';"));
    TEST_ASSERT_EQUAL_INT(2, scalar("SELECT count(*) FROM storage_deletions;"));
    TEST_ASSERT_EQUAL_INT(2, scalar("SELECT count(*) FROM storage_deletion_objects;"));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM storage_deletions WHERE uuid='fresh-1';"));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM storage_deletions WHERE uuid='open-1' AND completed_at IS NULL;"));
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT count(*) FROM pragma_foreign_key_check('storage_deletion_objects');"));

    /* Defaults (7 days, bounded rows): nothing else is old enough. */
    TEST_ASSERT_EQUAL_INT(0, storage_deletion_prune_completed(0, 0));
    TEST_ASSERT_EQUAL_INT(2, scalar("SELECT count(*) FROM storage_deletions;"));
    /* A one-day cutoff takes the fresh completed row but never an open one. */
    TEST_ASSERT_EQUAL_INT(1, storage_deletion_prune_completed(1, 0));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM storage_deletions;"));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM storage_deletion_objects WHERE deletion_uuid='open-1';"));
}

static void test_ledger_indexes_exist_and_serve_hot_path(void) {
    TEST_ASSERT_EQUAL_INT(4, scalar("SELECT count(*) FROM sqlite_master WHERE type='index' AND name IN "
                                    "('idx_recordings_deletion_pending','idx_storage_deletion_object_deletion',"
                                    "'idx_storage_deletion_object_open','idx_storage_deletion_completed');"));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM sqlite_master WHERE type='index' "
                                    "AND name='idx_recordings_deletion_pending' AND sql LIKE '%WHERE deletion_pending = 1%';"));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM sqlite_master WHERE type='index' "
                                    "AND name='idx_storage_deletion_object_open' AND sql LIKE '%WHERE state <> ''completed''%';"));

    char plan[2048];
    /* Scoped finalize: ledger entry by primary key, objects by deletion_uuid. */
    query_plan("UPDATE storage_deletions SET completed_at=strftime('%s','now') WHERE uuid=?1 AND completed_at IS NULL "
               "AND NOT EXISTS(SELECT 1 FROM storage_deletion_objects o WHERE o.deletion_uuid=?1 AND o.state<>'completed');",
               plan, sizeof(plan));
    TEST_ASSERT_NOT_NULL_MESSAGE(strstr(plan, "idx_storage_deletion_object_deletion"), plan);
    TEST_ASSERT_NULL_MESSAGE(strstr(plan, "SCAN"), plan);
    /* Sweep finalize: open ledger entries and pending recordings only. */
    query_plan("UPDATE storage_deletions SET completed_at=strftime('%s','now') WHERE completed_at IS NULL "
               "AND NOT EXISTS(SELECT 1 FROM storage_deletion_objects o "
               "WHERE o.deletion_uuid=storage_deletions.uuid AND o.state<>'completed');", plan, sizeof(plan));
    TEST_ASSERT_NOT_NULL_MESSAGE(strstr(plan, "idx_storage_deletion_completed"), plan);
    TEST_ASSERT_NOT_NULL_MESSAGE(strstr(plan, "idx_storage_deletion_object_deletion"), plan);
    query_plan("DELETE FROM recordings WHERE deletion_pending=1 AND EXISTS(SELECT 1 FROM storage_deletions d "
               "WHERE d.recording_id=recordings.id AND d.completed_at IS NOT NULL);", plan, sizeof(plan));
    TEST_ASSERT_NOT_NULL_MESSAGE(strstr(plan, "idx_recordings_deletion_pending"), plan);
    TEST_ASSERT_NOT_NULL_MESSAGE(strstr(plan, "idx_storage_deletion_recording"), plan);
    /* Worker due-object window: the partial index over open objects. */
    query_plan("SELECT o.id FROM storage_deletion_objects o WHERE o.id IN (SELECT id FROM storage_deletion_objects "
               "WHERE state<>'completed' AND next_attempt_at<=strftime('%s','now') ORDER BY next_attempt_at,id LIMIT ?1) "
               "ORDER BY o.id LIMIT ?3;", plan, sizeof(plan));
    TEST_ASSERT_NOT_NULL_MESSAGE(strstr(plan, "idx_storage_deletion_object_open"), plan);
    /* Prune: completed_at range, objects by deletion_uuid. */
    query_plan("DELETE FROM storage_deletion_objects WHERE deletion_uuid IN (SELECT uuid FROM storage_deletions "
               "WHERE completed_at IS NOT NULL AND completed_at<?1 ORDER BY completed_at,rowid LIMIT ?2);", plan, sizeof(plan));
    TEST_ASSERT_NOT_NULL_MESSAGE(strstr(plan, "idx_storage_deletion_completed"), plan);
    TEST_ASSERT_NOT_NULL_MESSAGE(strstr(plan, "idx_storage_deletion_object_deletion"), plan);
}

int main(void) {
    TEST_ASSERT_NOT_NULL(mkdtemp(storage_root));
    safe_strcpy(g_config.storage_path, storage_root, sizeof(g_config.storage_path), 0);
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: failed to initialize deletion ledger test database\n");
        return 1;
    }
    UNITY_BEGIN();
    RUN_TEST(test_missing_derivatives_create_one_object_row);
    RUN_TEST(test_existing_derivatives_are_journaled_and_removed);
    RUN_TEST(test_deletion_finalizes_only_its_own_ledger_entry);
    RUN_TEST(test_prune_completed_removes_old_rows_only);
    RUN_TEST(test_ledger_indexes_exist_and_serve_hot_path);
    int result = UNITY_END();
    shutdown_database();
    unlink(TEST_DB_PATH);
    char command[MAX_PATH_LENGTH + 16];
    snprintf(command, sizeof(command), "rm -rf %s", storage_root);
    (void)system(command);
    return result;
}
