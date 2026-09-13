#define _POSIX_C_SOURCE 200809L
#include <sqlite3.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <time.h>
#include <unistd.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include "web/libuv_connection.h"
#include "web/recording_archive.h"

#include "core/config.h"
#include "database/db_core.h"
#include "database/db_recordings.h"
#include "database/db_storage_lifecycle.h"
#include "database/db_storage_policies.h"
#include "database/db_storage_migrations.h"
#include "database/db_storage_targets.h"
#include "storage/storage_deletion.h"
#include "storage/storage_migration.h"
#include "storage/storage_source.h"
#include "storage/storage_s3.h"
#include "utils/strings.h"
#include "unity.h"

static char root[] = "/tmp/lightnvr-archive-test-XXXXXX";
static char db_path[512], source_path[512], source_uuid[37];
static const char *payload = "a finalized recording preserved across storage tiers";

static void sql(const char *value) {
    char *error = NULL;
    int result = sqlite3_exec(get_db_handle(), value, NULL, NULL, &error);
    if (error) fprintf(stderr, "SQL: %s\n", error);
    sqlite3_free(error);
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, result);
}

static int scalar(const char *value) {
    sqlite3_stmt *statement = NULL;
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_prepare_v2(get_db_handle(), value, -1, &statement, NULL));
    TEST_ASSERT_EQUAL_INT(SQLITE_ROW, sqlite3_step(statement));
    int result = sqlite3_column_int(statement, 0);
    sqlite3_finalize(statement);
    return result;
}

static storage_target_t archive_target(const char *bucket, bool enabled) {
    storage_target_t target = {0};
    safe_strcpy(target.name, bucket, sizeof(target.name), 0);
    safe_strcpy(target.target_type, "s3", sizeof(target.target_type), 0);
    safe_strcpy(target.storage_class, "cold", sizeof(target.storage_class), 0);
    safe_strcpy(target.endpoint, getenv("LIGHTNVR_TEST_S3_ENDPOINT"), sizeof(target.endpoint), 0);
    safe_strcpy(target.region, "fixture-region", sizeof(target.region), 0);
    safe_strcpy(target.bucket, bucket, sizeof(target.bucket), 0);
    safe_strcpy(target.credential_ref, "fixture", sizeof(target.credential_ref), 0);
    snprintf(target.root_path, sizeof(target.root_path), "s3://%s/instance", bucket);
    target.enabled = enabled;
    target.high_watermark_pct = 90;
    target.low_watermark_pct = 80;
    return target;
}

static uint64_t recording(bool protect) {
    FILE *file = fopen(source_path, "wb");
    TEST_ASSERT_NOT_NULL(file);
    fwrite(payload, 1, strlen(payload), file);
    fclose(file);
    recording_metadata_t metadata = {0};
    safe_strcpy(metadata.stream_name, "archive-camera", sizeof(metadata.stream_name), 0);
    safe_strcpy(metadata.file_path, source_path, sizeof(metadata.file_path), 0);
    safe_strcpy(metadata.storage_target_uuid, source_uuid, sizeof(metadata.storage_target_uuid), 0);
    safe_strcpy(metadata.object_key, "clip.mp4", sizeof(metadata.object_key), 0);
    safe_strcpy(metadata.codec, "h264", sizeof(metadata.codec), 0);
    safe_strcpy(metadata.trigger_type, "continuous", sizeof(metadata.trigger_type), 0);
    metadata.start_time = time(NULL) - 10 * 86400;
    metadata.end_time = metadata.start_time + 60;
    metadata.is_complete = true;
    metadata.size_bytes = strlen(payload);
    metadata.retention_override_days = -1;
    metadata.retention_tier = RETENTION_TIER_STANDARD;
    uint64_t id = add_recording_metadata(&metadata);
    TEST_ASSERT_NOT_EQUAL_UINT64(0, id);
    if (protect) TEST_ASSERT_EQUAL_INT(0, set_recording_protected(id, true));
    return id;
}

void setUp(void) {
    sql("DELETE FROM storage_deletion_objects;DELETE FROM storage_deletions;"
        "DELETE FROM storage_read_leases;DELETE FROM storage_retrieval_jobs;"
        "DELETE FROM storage_migration_jobs;DELETE FROM storage_recording_copies;"
        "DELETE FROM detections;DELETE FROM recordings;DELETE FROM storage_policies;DELETE FROM storage_targets;");
    unlink(source_path);
    TEST_ASSERT_EQUAL_INT(0, db_storage_target_bootstrap_default(root, source_uuid));
}
void tearDown(void) { unlink(source_path); }

static storage_migration_job_t move_recording(uint64_t id, storage_target_t *target) {
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_TARGET_OK, db_storage_target_create(target));
    storage_migration_job_t job;
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_MIGRATION_OK, db_storage_migration_create(id, target->uuid, 0, &job));
    TEST_ASSERT_EQUAL_INT(1, storage_migration_process_one());
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_MIGRATION_OK, db_storage_migration_get(job.uuid, &job));
    return job;
}

static void test_verified_archive_and_retrieval(void) {
    uint64_t id = recording(false);
    storage_target_t target = archive_target("archive", true);
    storage_migration_job_t job = move_recording(id, &target);
    TEST_ASSERT_EQUAL_STRING("completed", job.state);
    TEST_ASSERT_NOT_EQUAL_INT(0, access(source_path, F_OK));
    recording_metadata_t metadata;
    TEST_ASSERT_EQUAL_INT(0, get_recording_metadata_by_id(id, &metadata));
    TEST_ASSERT_EQUAL_STRING("", metadata.file_path);
    TEST_ASSERT_EQUAL_STRING(target.uuid, metadata.storage_target_uuid);
    char path[512], error[256];
    TEST_ASSERT_EQUAL_INT(STORAGE_SOURCE_PREPARING, storage_source_resolve(id, path, error));
    TEST_ASSERT_EQUAL_INT(STORAGE_SOURCE_PREPARING, storage_source_resolve(id, path, error));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM storage_retrieval_jobs;"));
    TEST_ASSERT_EQUAL_INT(1, storage_source_process_one());
    TEST_ASSERT_EQUAL_INT(STORAGE_SOURCE_READY, storage_source_resolve(id, path, error));
    FILE *file = fopen(path, "rb");
    TEST_ASSERT_NOT_NULL(file);
    char content[128] = {0};
    TEST_ASSERT_EQUAL_UINT(strlen(payload), fread(content, 1, sizeof(content)-1, file));
    fclose(file);
    TEST_ASSERT_EQUAL_STRING(payload, content);
    unlink(path);
}

static void test_verification_failure_preserves_source(void) {
    uint64_t id = recording(false);
    storage_target_t target = archive_target("corrupt", true);
    storage_migration_job_t job = move_recording(id, &target);
    TEST_ASSERT_NOT_EQUAL_INT(0, strcmp("completed", job.state));
    TEST_ASSERT_EQUAL_INT(0, access(source_path, F_OK));
    recording_metadata_t metadata;
    TEST_ASSERT_EQUAL_INT(0, get_recording_metadata_by_id(id, &metadata));
    TEST_ASSERT_EQUAL_STRING(source_uuid, metadata.storage_target_uuid);
}

static void test_protected_recording_archives_after_ordinary_expiry(void) {
    uint64_t id = recording(true);
    storage_target_t target = archive_target("protected", true);
    TEST_ASSERT_EQUAL_INT(0, db_storage_target_create(&target));
    storage_policy_t policy = {0};
    safe_strcpy(policy.name, "protected archive", sizeof(policy.name), 0);
    safe_strcpy(policy.selector_json, "{\"version\":1,\"expression\":{\"op\":\"all\"}}", sizeof(policy.selector_json), 0);
    safe_strcpy(policy.primary_target_uuid, source_uuid, sizeof(policy.primary_target_uuid), 0);
    safe_strcpy(policy.migration_target_uuid, target.uuid, sizeof(policy.migration_target_uuid), 0);
    safe_strcpy(policy.fallback_mode, "pause", sizeof(policy.fallback_mode), 0);
    policy.enabled = true; policy.required_copy_count = 1;
    policy.maximum_retention_days = 1; policy.archive_protected = true;
    policy.archive_after_seconds = 0; policy.hot_residency_seconds = 0;
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_POLICY_OK, db_storage_policy_create(&policy));
    char query[512];
    snprintf(query, sizeof(query), "UPDATE recordings SET storage_policy_uuid='%s',placement_reason='policy-primary:%s' WHERE id=%llu;",
             policy.uuid, policy.uuid, (unsigned long long)id);
    sql(query);
    TEST_ASSERT_EQUAL_INT(1, db_storage_lifecycle_schedule(16));
    TEST_ASSERT_EQUAL_INT(1, storage_migration_process_one());
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM recordings WHERE protected=1 AND file_path='';"));
    TEST_ASSERT_EQUAL_INT(-2, storage_recording_delete(id, "retention", NULL));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM recordings;"));
}

static void test_failed_delete_retains_inventory_until_retry(void) {
    uint64_t id = recording(false);
    storage_target_t target = archive_target("delete-retry", true);
    storage_migration_job_t job = move_recording(id, &target);
    TEST_ASSERT_EQUAL_STRING("completed", job.state);
    TEST_ASSERT_EQUAL_INT(1, storage_recording_delete(id, "retention", NULL));
    TEST_ASSERT_EQUAL_INT(1, storage_deletion_process_one());
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM recordings WHERE deletion_pending=1;"));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM storage_deletion_objects WHERE state='retry_wait';"));
    TEST_ASSERT_NOT_EQUAL_INT(0, set_recording_protected(id, true));
    sql("UPDATE storage_deletion_objects SET next_attempt_at=0;");
    TEST_ASSERT_EQUAL_INT(1, storage_deletion_process_one());
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT count(*) FROM recordings;"));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM storage_deletions WHERE completed_at IS NOT NULL;"));
}

static void test_unsafe_bucket_configuration_rejected(void) {
    storage_target_t target = archive_target("versioned", true);
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_TARGET_UNAVAILABLE, db_storage_target_create(&target));
    target = archive_target("expiry", true);
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_TARGET_UNAVAILABLE, db_storage_target_create(&target));
    target = archive_target("abort-only", true);
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_TARGET_OK, db_storage_target_create(&target));
    target = archive_target("archive", false);
    safe_strcpy(target.credential_ref, "../fixture", sizeof(target.credential_ref), 0);
    char error[256];
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_TARGET_INVALID, db_storage_target_validate(&target, error, sizeof(error)));
}

static void test_multipart_resume_after_restart(void) {
    uint64_t id = recording(false);
    FILE *file = fopen(source_path, "wb");
    TEST_ASSERT_NOT_NULL(file);
    unsigned char block[65536];
    for (size_t i = 0; i < sizeof(block); i++) block[i] = (unsigned char)(i * 13);
    for (int i = 0; i < 144; i++) TEST_ASSERT_EQUAL_UINT(sizeof(block), fwrite(block, 1, sizeof(block), file));
    fclose(file);
    sql("UPDATE recordings SET size_bytes=9437184;");
    storage_target_t target = archive_target("multipart-retry", true);
    storage_migration_job_t job = move_recording(id, &target);
    TEST_ASSERT_EQUAL_STRING("retry_wait", job.state);
    TEST_ASSERT_EQUAL_INT(0, access(source_path, F_OK));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM storage_migration_jobs WHERE upload_id<>'' AND json_array_length(upload_parts,'$.parts')=1;"));
    shutdown_database();
    TEST_ASSERT_EQUAL_INT(0, init_database_ex(db_path, DB_INIT_NO_BACKUP | DB_INIT_NO_CHECK));
    sql("UPDATE storage_migration_jobs SET next_attempt_at=0;");
    TEST_ASSERT_EQUAL_INT(1, storage_migration_process_one());
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_MIGRATION_OK, db_storage_migration_get(job.uuid, &job));
    TEST_ASSERT_EQUAL_STRING("completed", job.state);
    TEST_ASSERT_NOT_EQUAL_INT(0, access(source_path, F_OK));
}

static void test_existing_archive_promotion_and_pressure(void) {
    recording(false);
    storage_target_t target = archive_target("promotion", true);
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_TARGET_OK, db_storage_target_create(&target));
    char query[2048];
    snprintf(query, sizeof(query), "INSERT INTO storage_policies(uuid,name,selector_json,primary_target_uuid,maximum_retention_days,"
        "migration_target_uuid,archive_after_seconds,hot_residency_seconds,archive_on_pressure) "
        "VALUES('aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa','archive','{}','%s',30,'%s',0,2000000,1);"
        "UPDATE recordings SET storage_policy_uuid='aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa',"
        "placement_reason='policy-primary:aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa';", source_uuid, target.uuid);
    sql(query);
    TEST_ASSERT_EQUAL_INT(1, db_storage_lifecycle_schedule(16));
    TEST_ASSERT_EQUAL_INT(1, storage_migration_process_one());
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM storage_recording_copies;"));
    TEST_ASSERT_EQUAL_INT(0, access(source_path, F_OK));
    TEST_ASSERT_EQUAL_INT(0, db_storage_lifecycle_schedule(16)); // Do not reschedule the same copy.
    recording_metadata_t candidates[8];
    TEST_ASSERT_EQUAL_INT(0, get_recordings_for_pressure_cleanup(candidates, 8));
    sql("UPDATE storage_targets SET capacity_bytes=1000,available_bytes=1 WHERE target_type='filesystem';");
    TEST_ASSERT_EQUAL_INT(1, db_storage_lifecycle_schedule(16));
    TEST_ASSERT_EQUAL_INT(1, storage_migration_process_one());
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT count(*) FROM storage_recording_copies;"));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM recordings;"));
    TEST_ASSERT_NOT_EQUAL_INT(0, access(source_path, F_OK));
    int checked = 0;
    TEST_ASSERT_EQUAL_INT(0, get_orphaned_db_entries(candidates, 8, &checked));
}

static void test_destination_loss_preserves_cleanup_source(void) {
    uint64_t id = recording(false);
    storage_target_t target = archive_target("cleanup-loss", true);
    char path[MAX_PATH_LENGTH], error[256];
    TEST_ASSERT_EQUAL_INT(STORAGE_SOURCE_READY, storage_source_resolve(id, path, error));
    storage_migration_job_t job = move_recording(id, &target);
    TEST_ASSERT_EQUAL_STRING("cleanup_pending", job.state);
    TEST_ASSERT_EQUAL_INT(0, storage_s3_delete(&target, job.destination_object_key, error));
    sql("DELETE FROM storage_read_leases;UPDATE storage_migration_jobs SET next_attempt_at=0;");
    TEST_ASSERT_EQUAL_INT(1, storage_migration_process_one());
    TEST_ASSERT_EQUAL_INT(0, access(source_path, F_OK));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM storage_migration_jobs WHERE state='cleanup_pending';"));
}

static void test_archive_range_proxy_and_disconnect(void) {
    uint64_t id = recording(false);
    storage_target_t target = archive_target("range-proxy", true);
    storage_migration_job_t job = move_recording(id, &target);
    TEST_ASSERT_EQUAL_STRING("completed", job.state);
    for (int disconnect = 0; disconnect < 2; disconnect++) {
        int listener = socket(AF_INET, SOCK_STREAM, 0);
        TEST_ASSERT_GREATER_OR_EQUAL_INT(0, listener);
        struct sockaddr_in address = {.sin_family=AF_INET, .sin_addr.s_addr=htonl(INADDR_LOOPBACK)};
        TEST_ASSERT_EQUAL_INT(0, bind(listener, (struct sockaddr *)&address, sizeof(address)));
        TEST_ASSERT_EQUAL_INT(0, listen(listener, 1));
        socklen_t length = sizeof(address);
        TEST_ASSERT_EQUAL_INT(0, getsockname(listener, (struct sockaddr *)&address, &length));
        int client = socket(AF_INET, SOCK_STREAM, 0);
        TEST_ASSERT_EQUAL_INT(0, connect(client, (struct sockaddr *)&address, length));
        int peer = accept(listener, NULL, NULL);
        TEST_ASSERT_GREATER_OR_EQUAL_INT(0, peer);
        close(listener);
        uv_loop_t loop;
        TEST_ASSERT_EQUAL_INT(0, uv_loop_init(&loop));
        libuv_server_t server = {.loop = &loop};
        libuv_connection_t *conn = libuv_connection_create(&server);
        TEST_ASSERT_NOT_NULL(conn);
        TEST_ASSERT_EQUAL_INT(0, uv_tcp_open(&conn->handle, peer));
        conn->keep_alive = false;
        conn->request.user_data = conn;
        conn->request.method = HTTP_METHOD_GET;
        safe_strcpy(conn->request.path, "/api/recordings/play/1", sizeof(conn->request.path), 0);
        safe_strcpy(conn->request.headers[0].name, "Range", sizeof(conn->request.headers[0].name), 0);
        safe_strcpy(conn->request.headers[0].value, "bytes=2-15", sizeof(conn->request.headers[0].value), 0);
        conn->request.num_headers = 1;
        TEST_ASSERT_TRUE(recording_archive_serve(&conn->request, &conn->response, id, false));
        if (disconnect) close(client);
        uv_run(&loop, UV_RUN_DEFAULT);
        TEST_ASSERT_EQUAL_INT(0, uv_loop_close(&loop));
        if (!disconnect) {
            char response[2048] = {0};
            size_t total = 0;
            ssize_t count;
            while ((count = recv(client, response + total, sizeof(response)-1-total, 0)) > 0) total += (size_t)count;
            close(client);
            TEST_ASSERT_NOT_NULL(strstr(response, "206 Partial Content"));
            TEST_ASSERT_NOT_NULL(strstr(response, "Content-Range: bytes 2-15/"));
            char *body = strstr(response, "\r\n\r\n");
            TEST_ASSERT_NOT_NULL(body);
            TEST_ASSERT_EQUAL_UINT(14, strlen(body+4));
            TEST_ASSERT_EQUAL_MEMORY(payload+2, body+4, 14);
        }
    }
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT count(*) FROM storage_retrieval_jobs;"));
}

static void test_retention_after_archive_and_indefinite_override(void) {
    uint64_t id = recording(false);
    storage_target_t target = archive_target("retention", true);
    storage_migration_job_t job = move_recording(id, &target);
    TEST_ASSERT_EQUAL_STRING("completed", job.state);
    recording_metadata_t expired[4];
    TEST_ASSERT_EQUAL_INT(1, get_recordings_for_retention("archive-camera", 1, 1, expired, 4));
    TEST_ASSERT_EQUAL_UINT64(id, expired[0].id);
    sql("UPDATE recordings SET retention_override_days=0;");
    TEST_ASSERT_EQUAL_INT(0, get_recordings_for_retention("archive-camera", 1, 1, expired, 4));
    sql("UPDATE recordings SET retention_override_days=-1;");
    TEST_ASSERT_EQUAL_INT(1, storage_recording_delete(id, "retention", NULL));
    TEST_ASSERT_EQUAL_INT(1, storage_deletion_process_one());
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT count(*) FROM recordings;"));
}

static void test_policy_expiry_rechecks_override(void) {
    uint64_t id = recording(false);
    storage_target_t target = archive_target("policy-expiry", true);
    storage_migration_job_t archived = move_recording(id, &target);
    TEST_ASSERT_EQUAL_STRING("completed", archived.state);
    storage_policy_t policy = {0};
    safe_strcpy(policy.name, "retention", sizeof(policy.name), 0);
    safe_strcpy(policy.selector_json, "{\"version\":1,\"expression\":{\"op\":\"all\"}}", sizeof(policy.selector_json), 0);
    safe_strcpy(policy.primary_target_uuid, source_uuid, sizeof(policy.primary_target_uuid), 0);
    safe_strcpy(policy.fallback_mode, "pause", sizeof(policy.fallback_mode), 0);
    policy.enabled = true; policy.required_copy_count = 1;
    policy.maximum_retention_days = 1;
    policy.archive_after_seconds = -1; policy.hot_residency_seconds = -1;
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_POLICY_OK, db_storage_policy_create(&policy));
    char query[256];
    snprintf(query, sizeof(query), "UPDATE recordings SET storage_policy_uuid='%s',storage_policy_version=%lld,retention_override_days=0;",
             policy.uuid, (long long)policy.revision);
    sql(query);
    TEST_ASSERT_EQUAL_INT(-2, storage_recording_expire_policy(id));
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT count(*) FROM storage_deletions;"));
    sql("UPDATE recordings SET retention_override_days=30;");
    TEST_ASSERT_EQUAL_INT(-2, storage_recording_expire_policy(id));
    sql("UPDATE recordings SET retention_override_days=-1;");
    TEST_ASSERT_EQUAL_INT(1, db_storage_lifecycle_expire(16));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM recordings WHERE deletion_pending=1;"));
    TEST_ASSERT_EQUAL_INT(1, storage_deletion_process_one());
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT count(*) FROM recordings;"));
}

static void test_retrieval_wait_does_not_exhaust_migration_attempts(void) {
    uint64_t id = recording(false);
    storage_target_t origin = archive_target("waiting-origin", true);
    storage_migration_job_t archived = move_recording(id, &origin);
    TEST_ASSERT_EQUAL_STRING("completed", archived.state);
    storage_target_t destination = archive_target("waiting-destination", true);
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_TARGET_OK, db_storage_target_create(&destination));
    storage_migration_job_t job;
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_MIGRATION_OK, db_storage_migration_create(id, destination.uuid, 0, &job));
    for (int i = 0; i < 7; i++) {
        sql("UPDATE storage_migration_jobs SET next_attempt_at=0;");
        TEST_ASSERT_EQUAL_INT(1, storage_migration_process_one());
        TEST_ASSERT_EQUAL_INT(DB_STORAGE_MIGRATION_OK, db_storage_migration_get(job.uuid, &job));
        TEST_ASSERT_EQUAL_STRING("retry_wait", job.state);
        TEST_ASSERT_EQUAL_INT(0, job.attempt_count);
    }
    TEST_ASSERT_EQUAL_INT(1, storage_source_process_one());
    sql("UPDATE storage_migration_jobs SET next_attempt_at=0;");
    TEST_ASSERT_EQUAL_INT(1, storage_migration_process_one());
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_MIGRATION_OK, db_storage_migration_get(job.uuid, &job));
    TEST_ASSERT_EQUAL_STRING("cleanup_pending", job.state);
    sql("UPDATE storage_read_leases SET expires_at=0;UPDATE storage_migration_jobs SET next_attempt_at=0;");
    TEST_ASSERT_EQUAL_INT(1, storage_migration_process_one());
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_MIGRATION_OK, db_storage_migration_get(job.uuid, &job));
    TEST_ASSERT_EQUAL_STRING("completed", job.state);
}

static void test_cache_preserves_shared_target_reserve(void) {
    uint64_t id = recording(false);
    storage_target_t target = archive_target("reserve", true);
    storage_migration_job_t archived = move_recording(id, &target);
    TEST_ASSERT_EQUAL_STRING("completed", archived.state);
    sql("UPDATE storage_targets SET reserve_bytes=9223372036854775807 WHERE target_type='filesystem';");
    char path[MAX_PATH_LENGTH], error[256];
    TEST_ASSERT_EQUAL_INT(STORAGE_SOURCE_ERROR, storage_source_resolve(id, path, error));
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT count(*) FROM storage_retrieval_jobs;"));
    TEST_ASSERT_NOT_NULL(strstr(error, "reserve"));
}

static void test_ambiguous_multipart_completion_recovers(void) {
    uint64_t id = recording(false);
    FILE *file = fopen(source_path, "wb");
    TEST_ASSERT_NOT_NULL(file);
    unsigned char block[65536] = {0};
    for (int i = 0; i < 144; i++) TEST_ASSERT_EQUAL_UINT(sizeof(block), fwrite(block, 1, sizeof(block), file));
    fclose(file);
    sql("UPDATE recordings SET size_bytes=9437184;");
    storage_target_t target = archive_target("multipart-ambiguous", true);
    storage_migration_job_t job = move_recording(id, &target);
    TEST_ASSERT_EQUAL_STRING("retry_wait", job.state);
    TEST_ASSERT_EQUAL_INT(0, access(source_path, F_OK));
    shutdown_database();
    TEST_ASSERT_EQUAL_INT(0, init_database_ex(db_path, DB_INIT_NO_BACKUP | DB_INIT_NO_CHECK));
    sql("UPDATE storage_migration_jobs SET next_attempt_at=0;");
    TEST_ASSERT_EQUAL_INT(1, storage_migration_process_one());
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_MIGRATION_OK, db_storage_migration_get(job.uuid, &job));
    TEST_ASSERT_EQUAL_STRING("completed", job.state);
    TEST_ASSERT_NOT_EQUAL_INT(0, access(source_path, F_OK));
}

static void test_archive_restore_without_retrieval_cache(void) {
    uint64_t id = recording(true);
    storage_target_t target = archive_target("restore", true);
    storage_migration_job_t archived = move_recording(id, &target);
    TEST_ASSERT_EQUAL_STRING("completed", archived.state);
    setenv("LIGHTNVR_ARCHIVE_CACHE_MB", "0", 1);
    storage_migration_job_t restore;
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_MIGRATION_OK, db_storage_migration_create(id, source_uuid, 0, &restore));
    TEST_ASSERT_EQUAL_INT(1, storage_migration_process_one());
    unsetenv("LIGHTNVR_ARCHIVE_CACHE_MB");
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_MIGRATION_OK, db_storage_migration_get(restore.uuid, &restore));
    TEST_ASSERT_EQUAL_STRING("completed", restore.state);
    recording_metadata_t metadata;
    TEST_ASSERT_EQUAL_INT(0, get_recording_metadata_by_id(id, &metadata));
    TEST_ASSERT_TRUE(metadata.protected);
    TEST_ASSERT_EQUAL_STRING(source_uuid, metadata.storage_target_uuid);
    TEST_ASSERT_EQUAL_INT(0, access(metadata.file_path, F_OK));
    uint64_t size;
    char error[256];
    TEST_ASSERT_EQUAL_INT(STORAGE_S3_MISSING, storage_s3_stat(&target, archived.destination_object_key, &size, error));
    unlink(metadata.file_path);
}

int main(void) {
    if (!getenv("LIGHTNVR_TEST_S3_ENDPOINT") || !mkdtemp(root)) return 2;
    snprintf(db_path, sizeof(db_path), "%s/catalog.sqlite", root);
    snprintf(source_path, sizeof(source_path), "%s/clip.mp4", root);
    safe_strcpy(g_config.storage_path, root, sizeof(g_config.storage_path), 0);
    if (init_database_ex(db_path, DB_INIT_NO_BACKUP | DB_INIT_NO_CHECK)) return 2;
    UNITY_BEGIN();
    RUN_TEST(test_verified_archive_and_retrieval);
    RUN_TEST(test_verification_failure_preserves_source);
    RUN_TEST(test_protected_recording_archives_after_ordinary_expiry);
    RUN_TEST(test_failed_delete_retains_inventory_until_retry);
    RUN_TEST(test_unsafe_bucket_configuration_rejected);
    RUN_TEST(test_multipart_resume_after_restart);
    RUN_TEST(test_existing_archive_promotion_and_pressure);
    RUN_TEST(test_destination_loss_preserves_cleanup_source);
    RUN_TEST(test_archive_range_proxy_and_disconnect);
    RUN_TEST(test_retention_after_archive_and_indefinite_override);
    RUN_TEST(test_archive_restore_without_retrieval_cache);
    RUN_TEST(test_policy_expiry_rechecks_override);
    RUN_TEST(test_retrieval_wait_does_not_exhaust_migration_attempts);
    RUN_TEST(test_cache_preserves_shared_target_reserve);
    RUN_TEST(test_ambiguous_multipart_completion_recovers);
    int result = UNITY_END();
    shutdown_database();
    return result;
}
