#define _POSIX_C_SOURCE 200809L
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>
#include <sys/stat.h>
#include "unity.h"
#include "core/config.h"
#include "core/logger.h"
#include "database/db_core.h"
#include "database/db_recordings.h"
#include "database/db_streams.h"
#include "storage/storage_manager.h"
#include "storage/storage_deletion.h"

static char directory[] = "/tmp/lightnvr-stale-prune-XXXXXX";
static char database[256], media[256], moved[256], file[256], backup[256];
static uint64_t record_id;
static bool live_writer, live_thread, complete_after_selection;

static void sql(const char *statement) {
    char *error = NULL;
    int result = sqlite3_exec(get_db_handle(), statement, NULL, NULL, &error);
    if (error) fprintf(stderr, "SQL: %s\n", error);
    sqlite3_free(error);
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, result);
}
static int scalar(const char *query) {
    sqlite3_stmt *statement = NULL;
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_prepare_v2(get_db_handle(), query, -1, &statement, NULL));
    TEST_ASSERT_EQUAL_INT(SQLITE_ROW, sqlite3_step(statement));
    int result = sqlite3_column_int(statement, 0);
    sqlite3_finalize(statement);
    return result;
}
static void content(const char *value) {
    FILE *output = fopen(file, "wb");
    TEST_ASSERT_NOT_NULL(output);
    TEST_ASSERT_EQUAL_size_t(strlen(value), fwrite(value, 1, strlen(value), output));
    TEST_ASSERT_EQUAL_INT(0, fclose(output));
}
static void assert_payload(const char *expected) {
    char bytes[64] = {0};
    FILE *input = fopen(file, "rb"); TEST_ASSERT_NOT_NULL(input);
    size_t count = fread(bytes, 1, sizeof(bytes), input); fclose(input);
    TEST_ASSERT_EQUAL_size_t(strlen(expected), count);
    if (count) TEST_ASSERT_EQUAL_MEMORY(expected, bytes, count);
}
static void change(const char *assignment) {
    char statement[512];
    snprintf(statement, sizeof(statement), "UPDATE recordings SET %s WHERE id=%llu", assignment, (unsigned long long)record_id);
    sql(statement);
}
bool __real_stream_has_continuous_writer(const char *);
bool __real_mp4_recording_thread_alive(const char *);
bool __wrap_stream_has_continuous_writer(const char *name) { return live_writer || __real_stream_has_continuous_writer(name); }
bool __wrap_mp4_recording_thread_alive(const char *name) { return live_thread || __real_mp4_recording_thread_alive(name); }
int __real_get_stale_incomplete_recordings_page(recording_metadata_t *, int, time_t, uint64_t, uint64_t *);
int __wrap_get_stale_incomplete_recordings_page(recording_metadata_t *records, int count, time_t cutoff, uint64_t after, uint64_t *maximum) {
    int result = __real_get_stale_incomplete_recordings_page(records, count, cutoff, after, maximum);
    if (result > 0 && complete_after_selection) {
        complete_after_selection = false;
        change("is_complete=1");
    }
    return result;
}
static void seed(void) {
    stream_config_t stream = {0};
    strcpy(stream.name, "stale_camera"); strcpy(stream.url, "rtsp://127.0.0.1:1/unused");
    TEST_ASSERT_GREATER_THAN(0, add_stream_config(&stream));
    recording_metadata_t record = {0};
    strcpy(record.stream_name, "stale_camera");
    snprintf(record.file_path, sizeof(record.file_path), "%s", file);
    record.start_time = time(NULL) - 7200;
    record.retention_override_days = -1; record.retention_tier = RETENTION_TIER_STANDARD;
    record.disk_pressure_eligible = true; record.schedule_restricted = -1;
    record_id = add_recording_metadata(&record); TEST_ASSERT_NOT_EQUAL(0, record_id);
}
void setUp(void) {
    live_writer = live_thread = complete_after_selection = false;
    sql("DELETE FROM storage_deletion_objects; DELETE FROM storage_deletions; DELETE FROM storage_migration_jobs; DELETE FROM recordings; DELETE FROM streams; DELETE FROM storage_targets;");
    mkdir(media, 0700); content(""); seed();
}
void tearDown(void) {
    rename(moved, media);
    unlink(file); unlink(backup);
    rmdir(media);
}
static void assert_retained(void) {
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT COUNT(*) FROM recordings"));
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT COUNT(*) FROM storage_deletions"));
}
void test_empty_stale_recording_pruned_with_guarded_journal(void) {
    TEST_ASSERT_EQUAL_INT(1, storage_reconcile_incomplete_recordings());
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT COUNT(*) FROM recordings"));
    TEST_ASSERT_EQUAL_INT(-1, access(file, F_OK));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT COUNT(*) FROM storage_deletion_objects WHERE stale_guard_kind='empty' AND stale_guard_dev IS NOT NULL AND stale_guard_ino IS NOT NULL AND state='completed'"));
}
void test_missing_stale_recording_pruned(void) {
    unlink(file);
    TEST_ASSERT_EQUAL_INT(1, storage_reconcile_incomplete_recordings());
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT COUNT(*) FROM recordings"));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT COUNT(*) FROM storage_deletion_objects WHERE stale_guard_kind='missing' AND state='completed'"));
}
void test_protected_zero_file_preserved(void) {
    change("protected=1");
    TEST_ASSERT_EQUAL_INT(0, storage_reconcile_incomplete_recordings());
    assert_retained(); assert_payload("");
}
void test_recent_zero_file_preserved(void) {
    change("start_time=strftime('%s','now')");
    TEST_ASSERT_EQUAL_INT(0, storage_reconcile_incomplete_recordings());
    assert_retained(); assert_payload("");
}
void test_live_writer_or_thread_preserves_old_zero_file(void) {
    live_writer = true;
    TEST_ASSERT_EQUAL_INT(0, storage_reconcile_incomplete_recordings()); assert_retained(); assert_payload("");
    live_writer = false; live_thread = true;
    TEST_ASSERT_EQUAL_INT(0, storage_reconcile_incomplete_recordings()); assert_retained(); assert_payload("");
}
void test_enabled_and_unknown_streams_preserved(void) {
    sql("UPDATE streams SET enabled=1");
    TEST_ASSERT_EQUAL_INT(0, storage_reconcile_incomplete_recordings()); assert_retained(); assert_payload("");
    sql("DELETE FROM streams");
    TEST_ASSERT_EQUAL_INT(0, storage_reconcile_incomplete_recordings()); assert_retained(); assert_payload("");
}
void test_unavailable_root_preserves_metadata(void) {
    TEST_ASSERT_EQUAL_INT(0, rename(media, moved));
    TEST_ASSERT_EQUAL_INT(0, storage_reconcile_incomplete_recordings()); assert_retained();
    TEST_ASSERT_EQUAL_INT(0, rename(moved, media)); assert_payload("");
}
void test_changed_row_after_selection_preserved(void) {
    complete_after_selection = true;
    TEST_ASSERT_EQUAL_INT(0, storage_reconcile_incomplete_recordings());
    assert_retained(); assert_payload("");
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT is_complete FROM recordings"));
}
void test_migrating_recording_preserved(void) {
    sql("INSERT INTO storage_targets(uuid,name,root_path) VALUES('11111111-1111-4111-8111-111111111111','a','/synthetic/a'),('22222222-2222-4222-8222-222222222222','b','/synthetic/b')");
    char statement[1024];
    snprintf(statement, sizeof(statement), "INSERT INTO storage_migration_jobs(uuid,recording_id,source_target_uuid,source_object_key,destination_target_uuid,destination_object_key,state) VALUES('migration',%llu,'11111111-1111-4111-8111-111111111111','a','22222222-2222-4222-8222-222222222222','b','copying')", (unsigned long long)record_id);
    sql(statement);
    TEST_ASSERT_EQUAL_INT(0, storage_reconcile_incomplete_recordings()); assert_retained(); assert_payload("");
}
void test_ordinary_api_still_rejects_incomplete_recording(void) {
    TEST_ASSERT_EQUAL_INT(-2, storage_recording_delete(record_id, "ordinary API", NULL));
    assert_retained(); assert_payload("");
}
static void queue_guard(const char *kind) {
    struct stat info = {0};
    if (strcmp(kind, "empty") == 0) TEST_ASSERT_EQUAL_INT(0, lstat(file, &info));
    char statement[2048], guards[160];
    if (strcmp(kind, "empty") == 0) snprintf(guards, sizeof(guards), "'empty',%llu,%llu", (unsigned long long)info.st_dev, (unsigned long long)info.st_ino);
    else if (strcmp(kind, "missing") == 0) strcpy(guards, "'missing',NULL,NULL");
    else strcpy(guards, "NULL,NULL,NULL");
    snprintf(statement, sizeof(statement), "INSERT INTO storage_deletions(uuid,recording_id,reason) VALUES('queued',%llu,'test pending deletion'); INSERT INTO storage_deletion_objects(deletion_uuid,file_path,stale_guard_kind,stale_guard_dev,stale_guard_ino) VALUES('queued','%s',%s); UPDATE recordings SET deletion_pending=1 WHERE id=%llu;", (unsigned long long)record_id, file, guards, (unsigned long long)record_id);
    sql(statement);
}
static void reopen_and_process(void) {
    shutdown_database();
    TEST_ASSERT_EQUAL_INT(0, init_database_ex(database, DB_INIT_NO_CHECK | DB_INIT_NO_BACKUP));
    TEST_ASSERT_EQUAL_INT(1, storage_deletion_process_one());
}
void test_replaced_empty_inode_survives_delayed_deletion(void) {
    queue_guard("empty");
    TEST_ASSERT_EQUAL_INT(0, rename(file, backup)); content("");
    reopen_and_process();
    assert_payload("");
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT COUNT(*) FROM recordings"));
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT COUNT(*) FROM storage_deletion_objects WHERE state='completed'"));
}
void test_recovered_payload_survives_delayed_deletion(void) {
    queue_guard("empty"); content("recovered footage");
    reopen_and_process(); assert_payload("recovered footage");
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT COUNT(*) FROM recordings"));
}
void test_missing_guard_preserves_later_appearing_file(void) {
    unlink(file); queue_guard("missing"); content("new footage");
    reopen_and_process(); assert_payload("new footage");
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT COUNT(*) FROM recordings"));
}
void test_unchanged_guarded_empty_deletion_completes_after_restart(void) {
    queue_guard("empty"); reopen_and_process();
    TEST_ASSERT_EQUAL_INT(-1, access(file, F_OK));
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT COUNT(*) FROM recordings"));
}
void test_unchanged_missing_guard_completes_after_restart(void) {
    unlink(file); queue_guard("missing"); reopen_and_process();
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT COUNT(*) FROM recordings"));
}
void test_enabled_stream_after_queue_preserves_file(void) {
    queue_guard("empty"); sql("UPDATE streams SET enabled=1");
    reopen_and_process(); assert_payload("");
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT COUNT(*) FROM recordings"));
}
void test_live_owner_after_queue_preserves_file(void) {
    queue_guard("empty"); live_thread = true;
    reopen_and_process(); assert_payload("");
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT COUNT(*) FROM recordings"));
}
static void attribute_target(void) {
    char statement[1024];
    snprintf(statement, sizeof(statement), "INSERT INTO storage_targets(uuid,name,root_path) VALUES('11111111-1111-4111-8111-111111111111','local','%s'); UPDATE recordings SET storage_target_uuid='11111111-1111-4111-8111-111111111111',object_key='candidate.mp4'", media);
    sql(statement);
}
static void queue_attributed(void) {
    attribute_target(); queue_guard("empty");
    sql("UPDATE storage_deletion_objects SET target_uuid='11111111-1111-4111-8111-111111111111',object_key='candidate.mp4'");
}
void test_attributed_empty_stale_recording_pruned(void) {
    attribute_target();
    TEST_ASSERT_EQUAL_INT(1, storage_reconcile_incomplete_recordings());
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT COUNT(*) FROM recordings"));
    TEST_ASSERT_EQUAL_INT(-1, access(file, F_OK));
}
void test_attributed_missing_stale_recording_pruned(void) {
    attribute_target(); unlink(file);
    TEST_ASSERT_EQUAL_INT(1, storage_reconcile_incomplete_recordings());
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT COUNT(*) FROM recordings"));
}
void test_attributed_wrong_object_key_preserves_recording(void) {
    attribute_target(); change("object_key='wrong.mp4'");
    TEST_ASSERT_EQUAL_INT(0, storage_reconcile_incomplete_recordings());
    assert_retained(); assert_payload("");
}
void test_attributed_missing_mount_guard_preserves_recording(void) {
    attribute_target(); sql("UPDATE storage_targets SET mount_required=1,mount_guard_path='/not/a/mounted/storage'");
    TEST_ASSERT_EQUAL_INT(0, storage_reconcile_incomplete_recordings());
    assert_retained(); assert_payload("");
}
void test_attributed_unavailable_root_preserves_recording(void) {
    attribute_target(); TEST_ASSERT_EQUAL_INT(0, rename(media, moved));
    TEST_ASSERT_EQUAL_INT(0, storage_reconcile_incomplete_recordings()); assert_retained();
    TEST_ASSERT_EQUAL_INT(0, rename(moved, media)); assert_payload("");
}
void test_attributed_key_change_after_queue_preserves_recording(void) {
    queue_attributed(); change("object_key='different.mp4'");
    reopen_and_process(); assert_payload("");
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT COUNT(*) FROM recordings"));
}
void test_attributed_target_change_after_queue_preserves_recording(void) {
    queue_attributed(); change("storage_target_uuid=NULL");
    reopen_and_process(); assert_payload("");
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT COUNT(*) FROM recordings"));
}
void test_attributed_root_change_after_queue_preserves_recording(void) {
    queue_attributed(); sql("UPDATE storage_targets SET root_path='/tmp'");
    reopen_and_process(); assert_payload("");
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT COUNT(*) FROM recordings"));
}
void test_attributed_unchanged_queue_completes_after_restart(void) {
    queue_attributed(); reopen_and_process();
    TEST_ASSERT_EQUAL_INT(-1, access(file, F_OK));
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT COUNT(*) FROM recordings"));
}
void test_ordinary_null_guard_semantics_unchanged(void) {
    content("ordinary footage"); change("is_complete=1"); queue_guard("ordinary");
    reopen_and_process();
    TEST_ASSERT_EQUAL_INT(-1, access(file, F_OK));
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT COUNT(*) FROM recordings"));
}
int main(void) {
    init_logger(); load_default_config(&g_config);
    if (!mkdtemp(directory)) return 1;
    snprintf(database, sizeof(database), "%s/catalog.db", directory);
    snprintf(media, sizeof(media), "%s/media", directory);
    snprintf(moved, sizeof(moved), "%s/unavailable", directory);
    snprintf(file, sizeof(file), "%s/media/candidate.mp4", directory);
    snprintf(backup, sizeof(backup), "%s/media/original-empty.mp4", directory);
    snprintf(g_config.storage_path, sizeof(g_config.storage_path), "%s", media);
    if (init_database_ex(database, DB_INIT_NO_CHECK | DB_INIT_NO_BACKUP)) return 1;
    UNITY_BEGIN();
    RUN_TEST(test_empty_stale_recording_pruned_with_guarded_journal);
    RUN_TEST(test_missing_stale_recording_pruned);
    RUN_TEST(test_protected_zero_file_preserved);
    RUN_TEST(test_recent_zero_file_preserved);
    RUN_TEST(test_live_writer_or_thread_preserves_old_zero_file);
    RUN_TEST(test_enabled_and_unknown_streams_preserved);
    RUN_TEST(test_unavailable_root_preserves_metadata);
    RUN_TEST(test_changed_row_after_selection_preserved);
    RUN_TEST(test_migrating_recording_preserved);
    RUN_TEST(test_ordinary_api_still_rejects_incomplete_recording);
    RUN_TEST(test_replaced_empty_inode_survives_delayed_deletion);
    RUN_TEST(test_recovered_payload_survives_delayed_deletion);
    RUN_TEST(test_missing_guard_preserves_later_appearing_file);
    RUN_TEST(test_unchanged_guarded_empty_deletion_completes_after_restart);
    RUN_TEST(test_unchanged_missing_guard_completes_after_restart);
    RUN_TEST(test_enabled_stream_after_queue_preserves_file);
    RUN_TEST(test_live_owner_after_queue_preserves_file);
    RUN_TEST(test_attributed_empty_stale_recording_pruned);
    RUN_TEST(test_attributed_missing_stale_recording_pruned);
    RUN_TEST(test_attributed_wrong_object_key_preserves_recording);
    RUN_TEST(test_attributed_missing_mount_guard_preserves_recording);
    RUN_TEST(test_attributed_unavailable_root_preserves_recording);
    RUN_TEST(test_attributed_key_change_after_queue_preserves_recording);
    RUN_TEST(test_attributed_target_change_after_queue_preserves_recording);
    RUN_TEST(test_attributed_root_change_after_queue_preserves_recording);
    RUN_TEST(test_attributed_unchanged_queue_completes_after_restart);
    RUN_TEST(test_ordinary_null_guard_semantics_unchanged);
    int result = UNITY_END(); shutdown_database(); unlink(database); rmdir(directory); return result;
}
