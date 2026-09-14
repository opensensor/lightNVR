#define _POSIX_C_SOURCE 200809L
#include <sqlite3.h>
#include <errno.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <time.h>
#include <unistd.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <mbedtls/sha256.h>
#include "web/libuv_connection.h"
#include "web/recording_archive.h"

#include "core/config.h"
#include "database/db_core.h"
#include "database/db_auth.h"
#include "database/db_recordings.h"
#include "database/db_streams.h"
#include "database/db_storage_lifecycle.h"
#include "database/db_storage_policies.h"
#include "database/db_storage_migrations.h"
#include "database/db_storage_targets.h"
#include "storage/storage_deletion.h"
#include "storage/storage_migration.h"
#include "storage/storage_source.h"
#include "storage/storage_s3.h"
#include "utils/strings.h"
#include "web/api_handlers_recordings.h"
#include "web/api_handlers_recordings_download.h"
#include "web/api_handlers_recordings_batch_download.h"
#include "web/api_handlers_recordings_playback.h"
#include "web/api_handlers_recordings_thumbnail.h"
#include "web/api_handlers_storage_migrations.h"
#include "web/api_handlers_storage_targets.h"
#include "web/api_handlers_storage_policies.h"
#include "web/recording_source.h"
#include "unity.h"

static char root[] = "/tmp/lightnvr-archive-test-XXXXXX";
static char db_path[512], source_path[512], source_uuid[37];
static const char *payload = "a finalized recording preserved across storage tiers";

// Capture only the archive renewal timer to exercise elapsed leases deterministically.
static bool capture_lease_timers;
static int captured_timer_count;
static uv_timer_t *captured_timers[2];
static uv_timer_cb captured_callbacks[2];
int __real_uv_timer_start(uv_timer_t *, uv_timer_cb, uint64_t, uint64_t);
int __wrap_uv_timer_start(uv_timer_t *timer, uv_timer_cb callback, uint64_t timeout, uint64_t repeat) {
    if (capture_lease_timers && timeout == 30000 && repeat == 30000 && captured_timer_count < 2) {
        captured_timers[captured_timer_count] = timer;
        captured_callbacks[captured_timer_count++] = callback;
    }
    return __real_uv_timer_start(timer, callback, timeout, repeat);
}

static cJSON *api(void (*handler)(const http_request_t *, http_response_t *),
                  http_method_t method, const char *path, const char *query,
                  const char *body, const char *api_key, int expected) {
    http_request_t request;
    http_response_t response;
    http_request_init(&request);
    http_response_init(&response);
    request.method = method;
    safe_strcpy(request.method_str, method == HTTP_METHOD_GET ? "GET" :
        method == HTTP_METHOD_DELETE ? "DELETE" : "POST", sizeof(request.method_str), 0);
    safe_strcpy(request.path, path, sizeof(request.path), 0);
    safe_strcpy(request.uri, path, sizeof(request.uri), 0);
    safe_strcpy(request.query_string, query ? query : "", sizeof(request.query_string), 0);
    safe_strcpy(request.client_ip, "127.0.0.1", sizeof(request.client_ip), 0);
    request.body = (void *)body;
    request.body_len = body ? strlen(body) : 0;
    if (api_key) {
        safe_strcpy(request.headers[0].name, "X-API-Key", sizeof(request.headers[0].name), 0);
        safe_strcpy(request.headers[0].value, api_key, sizeof(request.headers[0].value), 0);
        request.num_headers = 1;
    }
    handler(&request, &response);
    cJSON *json = response.body ? cJSON_Parse(response.body) : NULL;
    int actual = response.status_code;
    http_response_free(&response);
    TEST_ASSERT_EQUAL_INT_MESSAGE(expected, actual, path);
    TEST_ASSERT_NOT_NULL(json);
    return json;
}

static cJSON *field(cJSON *json, const char *name) {
    cJSON *value = cJSON_GetObjectItemCaseSensitive(json, name);
    TEST_ASSERT_NOT_NULL_MESSAGE(value, name);
    return value;
}

static void assert_storage_state(uint64_t id, const char *state, int copies,
                                 bool external, const char *retention) {
    cJSON *json = cJSON_CreateObject();
    recording_source_add_status(json, id);
    TEST_ASSERT_EQUAL_STRING(state, field(json, "storage_state")->valuestring);
    TEST_ASSERT_EQUAL_INT(copies, field(json, "durable_copy_count")->valueint);
    TEST_ASSERT_EQUAL_INT(external, cJSON_IsTrue(field(json, "external_source")));
    TEST_ASSERT_EQUAL_STRING(retention, field(json, "retention_mode")->valuestring);
    TEST_ASSERT_EQUAL_INT(!strcmp(state, "deletion_pending"), cJSON_IsTrue(field(json, "deletion_pending")));
    cJSON_Delete(json);
}

static void sql(const char *value) {
    char *error = NULL;
    pthread_mutex_lock(get_db_mutex());
    int result = sqlite3_exec(get_db_handle(), value, NULL, NULL, &error);
    pthread_mutex_unlock(get_db_mutex());
    if (error) fprintf(stderr, "SQL: %s\n", error);
    sqlite3_free(error);
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, result);
}

static int scalar(const char *value) {
    sqlite3_stmt *statement = NULL;
    pthread_mutex_lock(get_db_mutex());
    int prepared = sqlite3_prepare_v2(get_db_handle(), value, -1, &statement, NULL);
    int step = prepared == SQLITE_OK ? sqlite3_step(statement) : prepared;
    int result = step == SQLITE_ROW ? sqlite3_column_int(statement, 0) : 0;
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(get_db_mutex());
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, prepared);
    TEST_ASSERT_EQUAL_INT(SQLITE_ROW, step);
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
    capture_lease_timers = false;
    captured_timer_count = 0;
    g_config.web_auth_enabled = false;
    g_config.generate_thumbnails = true;
    g_config.demo_mode = false;
    sql("DELETE FROM storage_deletion_objects;DELETE FROM storage_deletions;"
        "DELETE FROM storage_read_leases;DELETE FROM storage_retrieval_jobs;"
        "DELETE FROM storage_migration_jobs;DELETE FROM storage_recording_copies;"
        "DELETE FROM detections;DELETE FROM recordings;DELETE FROM storage_policies;DELETE FROM storage_targets;DELETE FROM audit_events;");
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

static int stop_runaway_lifecycle_query(void *context) {
    int *callbacks = context;
    // Count SQLite instructions instead of wall time so slow CI machines use
    // the same budget. A recording-by-job nested scan exceeds this by far.
    return ++*callbacks > 2000;
}

static void test_lifecycle_scheduler_with_large_migration_backlog(void) {
    uint64_t id = recording(false);
    storage_target_t target = archive_target("backlog", true);
    TEST_ASSERT_EQUAL_INT(0, db_storage_target_create(&target));
    storage_policy_t policy = {0};
    safe_strcpy(policy.name, "backlog archive", sizeof(policy.name), 0);
    safe_strcpy(policy.selector_json, "{\"version\":1,\"expression\":{\"op\":\"all\"}}", sizeof(policy.selector_json), 0);
    safe_strcpy(policy.primary_target_uuid, source_uuid, sizeof(policy.primary_target_uuid), 0);
    safe_strcpy(policy.migration_target_uuid, target.uuid, sizeof(policy.migration_target_uuid), 0);
    safe_strcpy(policy.fallback_mode, "pause", sizeof(policy.fallback_mode), 0);
    policy.enabled = true;
    policy.required_copy_count = 1;
    policy.archive_after_seconds = 21600;
    policy.hot_residency_seconds = 21600;
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_POLICY_OK, db_storage_policy_create(&policy));

    char query[4096];
    snprintf(query, sizeof(query),
        "UPDATE recordings SET storage_policy_uuid='%s',storage_policy_version=%lld WHERE id=%llu;"
        "WITH RECURSIVE batch(n) AS (VALUES(1) UNION ALL SELECT n+1 FROM batch WHERE n<2000) "
        "INSERT INTO recordings(stream_name,camera_uuid,file_path,start_time,end_time,size_bytes,"
        "is_complete,storage_target_uuid,object_key,storage_policy_uuid,storage_policy_version,"
        "retention_override_days,trigger_type) "
        "SELECT stream_name,camera_uuid,file_path||'.'||n,start_time,end_time,size_bytes,1,"
        "storage_target_uuid,'backlog/'||n,storage_policy_uuid,storage_policy_version,-1,trigger_type "
        "FROM recordings,batch WHERE id=%llu;"
        "INSERT INTO storage_migration_jobs(uuid,recording_id,source_target_uuid,source_object_key,"
        "destination_target_uuid,destination_object_key,state,bytes_total) "
        "SELECT printf('00000000-0000-4000-8000-%%012d',id),id,storage_target_uuid,object_key,"
        "'%s','archive/'||id,CASE id%%3 WHEN 0 THEN 'failed' WHEN 1 THEN 'cancelled' ELSE 'queued' END,"
        "size_bytes FROM recordings WHERE id<>%llu;",
        policy.uuid, (long long)policy.revision, (unsigned long long)id,
        (unsigned long long)id, target.uuid, (unsigned long long)id);
    sql(query);
    TEST_ASSERT_EQUAL_INT(2000, scalar("SELECT count(*) FROM storage_migration_jobs;"));

    int callbacks = 0;
    sqlite3_progress_handler(get_db_handle(), 1000, stop_runaway_lifecycle_query, &callbacks);
    int scheduled = db_storage_lifecycle_schedule(16);
    sqlite3_progress_handler(get_db_handle(), 0, NULL, NULL);
    TEST_ASSERT_EQUAL_INT_MESSAGE(1, scheduled,
        "Scheduling must not scan every migration job for each recording");
    // Failed/cancelled transfers remain excluded, just as queued transfers do.
    // Only the one recording without any existing job should be scheduled.
    TEST_ASSERT_EQUAL_INT(2001, scalar("SELECT count(*) FROM storage_migration_jobs;"));
    snprintf(query, sizeof(query), "SELECT count(*) FROM storage_migration_jobs WHERE recording_id=%llu;",
             (unsigned long long)id);
    TEST_ASSERT_EQUAL_INT(1, scalar(query));
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
    target = archive_target("public", true);
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_TARGET_UNAVAILABLE, db_storage_target_create(&target));
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
    for (int mode = 0; mode < 4; mode++) {
        bool disconnect = mode == 1, download = mode == 2, unavailable = mode == 3;
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
        conn->request.num_headers = download ? 0 : 1;
        if (unavailable) {
            char error[256];
            TEST_ASSERT_EQUAL_INT(0, storage_s3_delete(&target, job.destination_object_key, error));
        }
        TEST_ASSERT_TRUE(recording_archive_serve(&conn->request, &conn->response, id, download));
        if (disconnect) {
            close(client);
            libuv_connection_close(conn);
        }
        uv_run(&loop, UV_RUN_DEFAULT);
        TEST_ASSERT_EQUAL_INT(0, uv_loop_close(&loop));
        if (!disconnect) {
            char response[2048] = {0};
            size_t total = 0;
            ssize_t count;
            while ((count = recv(client, response + total, sizeof(response)-1-total, 0)) > 0) total += (size_t)count;
            close(client);
            char *body = strstr(response, "\r\n\r\n");
            TEST_ASSERT_NOT_NULL(body);
            if (unavailable) {
                TEST_ASSERT_NOT_NULL(strstr(response, "503 Service Unavailable"));
                TEST_ASSERT_NOT_NULL(strstr(response, "Retry-After: 5"));
                TEST_ASSERT_NOT_NULL(strstr(body, "Archive source is unavailable"));
            } else if (download) {
                TEST_ASSERT_NOT_NULL(strstr(response, "200 OK"));
                TEST_ASSERT_NOT_NULL(strstr(response, "Content-Disposition: attachment;"));
                TEST_ASSERT_EQUAL_STRING(payload, body + 4);
            } else {
                TEST_ASSERT_NOT_NULL(strstr(response, "206 Partial Content"));
                TEST_ASSERT_NOT_NULL(strstr(response, "Content-Range: bytes 2-15/"));
                TEST_ASSERT_EQUAL_UINT(14, strlen(body+4));
                TEST_ASSERT_EQUAL_MEMORY(payload+2, body+4, 14);
            }
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

static void test_archive_admin_summary_retry_and_permissions(void) {
    uint64_t id = recording(false);
    storage_target_t target = archive_target("delete-retry", true);
    storage_migration_job_t job = move_recording(id, &target);
    TEST_ASSERT_EQUAL_STRING("completed", job.state);
    setenv("LIGHTNVR_STORAGE_READ_ONLY", "1", 1);
    cJSON *json = api(handle_get_storage_archive, HTTP_METHOD_GET, "/api/storage-archive", NULL, NULL, NULL, 200);
    TEST_ASSERT_TRUE(cJSON_IsTrue(field(json, "recovery_read_only")));
    TEST_ASSERT_TRUE(cJSON_IsTrue(field(json, "s3_enabled")));
    TEST_ASSERT_EQUAL_UINT(strlen(payload), field(json, "archive_bytes")->valueint);
    TEST_ASSERT_EQUAL_INT(0, field(json, "pending_deletion_count")->valueint);
    cJSON_Delete(json);
    unsetenv("LIGHTNVR_STORAGE_READ_ONLY");

    char path[128], body[128];
    snprintf(path, sizeof(path), "/api/recordings/%llu", (unsigned long long)id);
    TEST_ASSERT_EQUAL_INT(0, set_recording_protected(id, true));
    cJSON_Delete(api(handle_delete_recording, HTTP_METHOD_DELETE, path, NULL, NULL, NULL, 409));
    TEST_ASSERT_EQUAL_INT(0, set_recording_protected(id, false));
    cJSON_Delete(api(handle_delete_recording, HTTP_METHOD_DELETE, path, NULL, NULL, NULL, 202));
    TEST_ASSERT_EQUAL_INT(1, storage_deletion_process_one()); // The provider rejects the first DELETE.
    json = api(handle_get_storage_archive, HTTP_METHOD_GET, "/api/storage-archive", NULL, NULL, NULL, 200);
    TEST_ASSERT_EQUAL_INT(1, field(json, "pending_deletion_count")->valueint);
    cJSON *pending = field(json, "pending_deletions");
    TEST_ASSERT_EQUAL_INT(1, cJSON_GetArraySize(pending));
    cJSON *item = cJSON_GetArrayItem(pending, 0);
    TEST_ASSERT_EQUAL_UINT64(id, (uint64_t)field(item, "recording_id")->valuedouble);
    TEST_ASSERT_EQUAL_INT(1, field(item, "remaining_objects")->valueint);
    TEST_ASSERT_EQUAL_INT(1, field(item, "attempts")->valueint);
    TEST_ASSERT_NOT_EQUAL(0, strlen(field(item, "last_error")->valuestring));
    snprintf(body, sizeof(body), "{\"deletion_uuid\":\"%s\"}", field(item, "uuid")->valuestring);
    cJSON_Delete(json);

    const char *retry = "/api/storage-archive/retry";
    cJSON_Delete(api(handle_post_storage_archive_retry, HTTP_METHOD_POST, retry, NULL, "{}", NULL, 400));
    cJSON_Delete(api(handle_post_storage_archive_retry, HTTP_METHOD_POST, retry, NULL, "{", NULL, 400));
    cJSON_Delete(api(handle_post_storage_archive_retry, HTTP_METHOD_POST, retry, NULL,
        "{\"deletion_uuid\":\"aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa\"}", NULL, 404));

    int64_t user_id;
    TEST_ASSERT_EQUAL_INT(0, db_auth_create_user("archiveviewer", "password123", NULL, USER_ROLE_VIEWER, true, &user_id));
    char api_key[128];
    TEST_ASSERT_EQUAL_INT(0, db_auth_generate_api_key(user_id, api_key, sizeof(api_key)));
    g_config.web_auth_enabled = true;
    cJSON_Delete(api(handle_get_storage_archive, HTTP_METHOD_GET, "/api/storage-archive", NULL, NULL, NULL, 401));
    cJSON_Delete(api(handle_get_storage_archive, HTTP_METHOD_GET, "/api/storage-archive", NULL, NULL, api_key, 403));
    cJSON_Delete(api(handle_post_storage_archive_retry, HTTP_METHOD_POST, retry, NULL, body, api_key, 403));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM storage_deletion_objects WHERE next_attempt_at>0;"));
    g_config.web_auth_enabled = false;
    g_config.generate_thumbnails = true;
    cJSON_Delete(api(handle_post_storage_archive_retry, HTTP_METHOD_POST, retry, NULL, body, NULL, 202));
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT count(*) FROM storage_deletion_objects WHERE next_attempt_at>0;"));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM audit_events WHERE action='storage.configure' AND json_extract(details_json,'$.operation')='retry' AND outcome='success';"));
    TEST_ASSERT_EQUAL_INT(1, storage_deletion_process_one());
    json = api(handle_get_storage_archive, HTTP_METHOD_GET, "/api/storage-archive", NULL, NULL, NULL, 200);
    TEST_ASSERT_EQUAL_INT(0, field(json, "pending_deletion_count")->valueint);
    TEST_ASSERT_EQUAL_INT(0, field(json, "archive_bytes")->valueint);
    TEST_ASSERT_EQUAL_INT(0, cJSON_GetArraySize(field(json, "pending_deletions")));
    cJSON_Delete(json);
}

static void test_archive_http_rejects_invalid_ranges_and_bounds_concurrency(void) {
    uint64_t id = recording(false);
    storage_target_t target = archive_target("http-limits", true);
    storage_migration_job_t job = move_recording(id, &target);
    TEST_ASSERT_EQUAL_STRING("completed", job.state);
    uv_loop_t loop;
    TEST_ASSERT_EQUAL_INT(0, uv_loop_init(&loop));
    libuv_server_t server = {.loop = &loop};
    const char *invalid[] = {"bytes=5-2", "bytes=0-1,2-3", "bytes=99999-", "bytes=-", "items=0-1"};
    for (size_t i = 0; i < sizeof(invalid) / sizeof(invalid[0]); i++) {
        libuv_connection_t *conn = libuv_connection_create(&server);
        TEST_ASSERT_NOT_NULL(conn);
        conn->request.user_data = conn;
        safe_strcpy(conn->request.headers[0].name, "Range", sizeof(conn->request.headers[0].name), 0);
        safe_strcpy(conn->request.headers[0].value, invalid[i], sizeof(conn->request.headers[0].value), 0);
        conn->request.num_headers = 1;
        TEST_ASSERT_TRUE(recording_archive_serve(&conn->request, &conn->response, id, false));
        TEST_ASSERT_EQUAL_INT(416, conn->response.status_code);
        TEST_ASSERT_NULL(conn->archive_stream);
        libuv_connection_close(conn);
        uv_run(&loop, UV_RUN_DEFAULT);
    }
    // Repeat after cancellation to verify that closed streams release capacity.
    for (int pass = 0; pass < 2; pass++) {
        libuv_connection_t *connections[3];
        for (int i = 0; i < 3; i++) {
            connections[i] = libuv_connection_create(&server);
            TEST_ASSERT_NOT_NULL(connections[i]);
            connections[i]->request.user_data = connections[i];
            TEST_ASSERT_TRUE(recording_archive_serve(&connections[i]->request, &connections[i]->response, id, false));
            if (i < 2) TEST_ASSERT_TRUE(connections[i]->async_response_pending);
            else {
                TEST_ASSERT_EQUAL_INT(503, connections[i]->response.status_code);
                TEST_ASSERT_NULL(connections[i]->archive_stream);
            }
        }
        for (int i = 0; i < 3; i++) libuv_connection_close(connections[i]);
        uv_run(&loop, UV_RUN_DEFAULT);
    }
    TEST_ASSERT_EQUAL_INT(0, uv_loop_close(&loop));
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT count(*) FROM storage_retrieval_jobs;"));
}

static void test_cache_eviction_preserves_archive_and_active_reader(void) {
    uint64_t id = recording(false);
    storage_target_t target = archive_target("cache-eviction", true);
    storage_migration_job_t job = move_recording(id, &target);
    TEST_ASSERT_EQUAL_STRING("completed", job.state);
    char path[MAX_PATH_LENGTH], error[256];
    TEST_ASSERT_EQUAL_INT(STORAGE_SOURCE_PREPARING, storage_source_resolve(id, path, error));
    TEST_ASSERT_EQUAL_INT(1, storage_source_process_one());
    TEST_ASSERT_EQUAL_INT(STORAGE_SOURCE_READY, storage_source_resolve(id, path, error));
    sql("UPDATE storage_retrieval_jobs SET last_access_at=0;");
    TEST_ASSERT_EQUAL_UINT64(0, storage_source_trim(strlen(payload)));
    TEST_ASSERT_EQUAL_INT(0, access(path, F_OK));
    sql("UPDATE storage_read_leases SET expires_at=0;");
    TEST_ASSERT_EQUAL_UINT64(strlen(payload), storage_source_trim(strlen(payload)));
    TEST_ASSERT_NOT_EQUAL_INT(0, access(path, F_OK));
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT count(*) FROM storage_retrieval_jobs;"));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM recordings WHERE deletion_pending=0;"));
    uint64_t bytes;
    TEST_ASSERT_EQUAL_INT(STORAGE_S3_OK, storage_s3_stat(&target, job.destination_object_key, &bytes, error));
    TEST_ASSERT_EQUAL_UINT64(strlen(payload), bytes);
}

static void test_corrupt_retrieval_never_publishes_cache(void) {
    uint64_t id = recording(true);
    storage_target_t target = archive_target("corrupt-retrieval", true);
    storage_migration_job_t job = move_recording(id, &target);
    TEST_ASSERT_EQUAL_STRING("completed", job.state);
    // Simulate out-of-band modification after a successfully verified archival.
    char tampered[128], hash[65];
    unsigned char digest[32];
    safe_strcpy(tampered, payload, sizeof(tampered), 0);
    tampered[0] = '!';
    TEST_ASSERT_EQUAL_INT(0, mbedtls_sha256((unsigned char *)tampered, strlen(tampered), digest, 0));
    for (size_t i = 0; i < sizeof(digest); i++) snprintf(hash + i*2, 3, "%02x", digest[i]);
    FILE *file = fopen(source_path, "wb");
    TEST_ASSERT_NOT_NULL(file);
    TEST_ASSERT_EQUAL_UINT(strlen(tampered), fwrite(tampered, 1, strlen(tampered), file));
    fclose(file);
    char path[MAX_PATH_LENGTH], error[256];
    TEST_ASSERT_EQUAL_INT(0, storage_s3_upload(&target, job.destination_object_key, source_path, hash, NULL, error));
    unlink(source_path);
    TEST_ASSERT_EQUAL_INT(STORAGE_SOURCE_PREPARING, storage_source_resolve(id, path, error));
    TEST_ASSERT_EQUAL_INT(1, storage_source_process_one());
    TEST_ASSERT_EQUAL_INT(STORAGE_SOURCE_ERROR, storage_source_resolve(id, path, error));
    TEST_ASSERT_NOT_EQUAL(0, strlen(error));
    TEST_ASSERT_NOT_EQUAL_INT(0, access(path, F_OK));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM storage_retrieval_jobs WHERE state='failed';"));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM recordings WHERE protected=1 AND deletion_pending=0;"));
}

static void test_s3_configuration_api_and_archive_budget(void) {
    char body[1400];
    snprintf(body, sizeof(body), "{\"name\":\"API archive\",\"target_type\":\"s3\",\"storage_class\":\"cold\","
        "\"enabled\":true,\"root_path\":\"s3://api-config/instance\",\"endpoint\":\"%s\","
        "\"region\":\"fixture-region\",\"bucket\":\"api-config\",\"credential_ref\":\"fixture\",\"archive_budget_bytes\":1}",
        getenv("LIGHTNVR_TEST_S3_ENDPOINT"));
    cJSON *json = api(handle_post_storage_target, HTTP_METHOD_POST, "/api/storage-targets", NULL, body, NULL, 201);
    char uuid[37];
    safe_strcpy(uuid, field(json, "uuid")->valuestring, sizeof(uuid), 0);
    TEST_ASSERT_EQUAL_STRING("s3", field(json, "target_type")->valuestring);
    TEST_ASSERT_EQUAL_STRING("fixture", field(json, "credential_ref")->valuestring);
    TEST_ASSERT_EQUAL_INT(1, field(json, "archive_budget_bytes")->valueint);
    TEST_ASSERT_NULL(cJSON_GetObjectItemCaseSensitive(json, "secret_access_key"));
    cJSON_Delete(json);
    uint64_t id = recording(false);
    storage_migration_job_t job;
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_MIGRATION_TARGET_UNAVAILABLE, db_storage_migration_create(id, uuid, 0, &job));
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT count(*) FROM storage_migration_jobs;"));
    TEST_ASSERT_EQUAL_INT(0, access(source_path, F_OK));
    snprintf(body, sizeof(body), "{\"name\":\"Archive policy\",\"selector\":{\"version\":1,\"expression\":{\"op\":\"all\"}},"
        "\"primary_target_uuid\":\"%s\",\"migration_target_uuid\":\"%s\",\"archive_after_seconds\":60,"
        "\"hot_residency_seconds\":120,\"archive_protected\":true,\"archive_on_pressure\":true}", source_uuid, uuid);
    json = api(handle_post_storage_policy, HTTP_METHOD_POST, "/api/storage-policies", NULL, body, NULL, 201);
    TEST_ASSERT_EQUAL_INT(60, field(json, "archive_after_seconds")->valueint);
    TEST_ASSERT_EQUAL_INT(120, field(json, "hot_residency_seconds")->valueint);
    TEST_ASSERT_TRUE(cJSON_IsTrue(field(json, "archive_protected")));
    TEST_ASSERT_TRUE(cJSON_IsTrue(field(json, "archive_on_pressure")));
    cJSON_Delete(json);
}

static void test_recording_status_follows_archive_and_retrieval_lifecycle(void) {
    uint64_t id = recording(false);
    assert_storage_state(id, "hot", 1, false, "inherit");
    sql("UPDATE recordings SET retention_override_days=0;");
    assert_storage_state(id, "hot", 1, false, "indefinite");
    sql("UPDATE recordings SET retention_override_days=30;");
    storage_target_t target = archive_target("status", true);
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_TARGET_OK, db_storage_target_create(&target));
    storage_migration_job_t job;
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_MIGRATION_OK, db_storage_migration_create_operation(id, target.uuid, "copy", 0, &job));
    assert_storage_state(id, "archiving", 1, false, "finite");
    cJSON *json = api(handle_get_storage_archive, HTTP_METHOD_GET, "/api/storage-archive", NULL, NULL, NULL, 200);
    TEST_ASSERT_EQUAL_UINT(strlen(payload), field(json, "transfer_backlog_bytes")->valueint);
    cJSON_Delete(json);
    TEST_ASSERT_EQUAL_INT(1, storage_migration_process_one());
    assert_storage_state(id, "hot_and_archive", 2, false, "finite");
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_MIGRATION_OK, db_storage_migration_create(id, target.uuid, 0, &job));
    TEST_ASSERT_EQUAL_INT(1, storage_migration_process_one());
    assert_storage_state(id, "archived", 1, true, "finite");
    sql("UPDATE storage_targets SET health_status='unavailable' WHERE target_type='s3';");
    assert_storage_state(id, "unavailable", 1, true, "finite");
    sql("UPDATE storage_targets SET health_status='healthy' WHERE target_type='s3';");

    char path[128];
    snprintf(path, sizeof(path), "/api/recordings/download/%llu", (unsigned long long)id);
    json = api(handle_recordings_download, HTTP_METHOD_GET, path, "prepare=1", NULL, NULL, 200);
    TEST_ASSERT_EQUAL_STRING("ready", field(json, "status")->valuestring);
    TEST_ASSERT_EQUAL_STRING("archive", field(json, "source")->valuestring);
    cJSON_Delete(json);
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT count(*) FROM storage_retrieval_jobs;"));
    snprintf(path, sizeof(path), "/api/recordings/play/%llu", (unsigned long long)id);
    cJSON_Delete(api(handle_recordings_playback, HTTP_METHOD_GET, path, "prepare=1&transcode=0", NULL, NULL, 200));
    json = api(handle_recordings_playback, HTTP_METHOD_GET, path, "prepare=1&transcode=1", NULL, NULL, 202);
    TEST_ASSERT_EQUAL_STRING("preparing", field(json, "status")->valuestring);
    cJSON_Delete(json);
    assert_storage_state(id, "preparing", 1, true, "finite");
    char thumbnail_path[128];
    snprintf(thumbnail_path, sizeof(thumbnail_path), "/api/recordings/thumbnail/%llu/0", (unsigned long long)id);
    cJSON_Delete(api(handle_recordings_thumbnail, HTTP_METHOD_GET, thumbnail_path, "prepare=1", NULL, NULL, 202));
    json = api(handle_get_storage_archive, HTTP_METHOD_GET, "/api/storage-archive", NULL, NULL, NULL, 200);
    TEST_ASSERT_EQUAL_INT(1, field(json, "retrieval_jobs")->valueint);
    cJSON_Delete(json);
    sql("UPDATE storage_retrieval_jobs SET state='failed',last_error='Provider unavailable',next_attempt_at=0;");
    assert_storage_state(id, "unavailable", 1, true, "finite");
    cJSON_Delete(api(handle_recordings_playback, HTTP_METHOD_GET, path, "prepare=1&transcode=1", NULL, NULL, 202));
    TEST_ASSERT_EQUAL_INT(1, storage_source_process_one());
    assert_storage_state(id, "archived", 1, true, "finite");
    json = api(handle_get_storage_archive, HTTP_METHOD_GET, "/api/storage-archive", NULL, NULL, NULL, 200);
    TEST_ASSERT_EQUAL_UINT(strlen(payload), field(json, "retrieval_cache_bytes")->valueint);
    cJSON_Delete(json);
    TEST_ASSERT_EQUAL_INT(1, storage_recording_delete(id, "status test", NULL));
    assert_storage_state(id, "deletion_pending", 1, true, "finite");
    cJSON_Delete(api(handle_recordings_playback, HTTP_METHOD_GET, path, "prepare=1&transcode=0", NULL, NULL, 409));
    cJSON_Delete(api(handle_recordings_playback, HTTP_METHOD_GET, path, "prepare=1&transcode=1", NULL, NULL, 409));
    cJSON_Delete(api(handle_recordings_thumbnail, HTTP_METHOD_GET, thumbnail_path, NULL, NULL, NULL, 409));
    json = cJSON_CreateObject();
    recording_source_add_status(json, UINT64_MAX);
    TEST_ASSERT_NULL(cJSON_GetObjectItemCaseSensitive(json, "storage_state"));
    cJSON_Delete(json);
}

static void test_multipart_cancellation_aborts_owned_upload_and_preserves_source(void) {
    uint64_t id = recording(false);
    FILE *file = fopen(source_path, "wb");
    TEST_ASSERT_NOT_NULL(file);
    unsigned char block[65536] = {0};
    for (int i = 0; i < 144; i++) TEST_ASSERT_EQUAL_UINT(sizeof(block), fwrite(block, 1, sizeof(block), file));
    fclose(file);
    sql("UPDATE recordings SET size_bytes=9437184;");
    storage_target_t target = archive_target("multipart-retry", true);
    storage_migration_job_t job = move_recording(id, &target);
    TEST_ASSERT_EQUAL_STRING("retry_wait", job.state);
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM storage_migration_jobs WHERE upload_id<>'';"));
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_MIGRATION_OK, db_storage_migration_request_cancel(job.uuid, &job));
    TEST_ASSERT_EQUAL_STRING("cancelled", job.state);
    storage_migration_process_one();
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM storage_migration_jobs WHERE state='cancelled' AND artifacts_cleaned=1 AND upload_id='';"));
    TEST_ASSERT_EQUAL_INT(0, access(source_path, F_OK));
    uint64_t bytes;
    char error[256];
    TEST_ASSERT_EQUAL_INT(STORAGE_S3_MISSING, storage_s3_stat(&target, job.destination_object_key, &bytes, error));
    TEST_ASSERT_EQUAL_INT(0, storage_s3_abort_upload(&target, job.destination_object_key, "already-absent-upload", error));
}

static void test_archive_credentials_require_private_regular_files(void) {
    storage_target_t target = archive_target("credentials", true);
    char *original = strdup(getenv("LIGHTNVR_ARCHIVE_CREDENTIALS_DIR"));
    TEST_ASSERT_NOT_NULL(original);
    setenv("LIGHTNVR_ARCHIVE_CREDENTIALS_DIR", root, 1);
    char path[512], link[512], error[256];
    snprintf(path, sizeof(path), "%s/test-credential", root);
    snprintf(link, sizeof(link), "%s/test-credential-link", root);
    FILE *file = fopen(path, "w");
    TEST_ASSERT_NOT_NULL(file);
    fputs("{\"access_key_id\":\"fixture-access\",\"secret_access_key\":\"fixture-secret\"}", file);
    fclose(file);
    TEST_ASSERT_EQUAL_INT(0, chmod(path, 0644));
    safe_strcpy(target.credential_ref, "test-credential", sizeof(target.credential_ref), 0);
    uint64_t bytes;
    int unsafe_mode = storage_s3_stat(&target, "missing", &bytes, error);
    TEST_ASSERT_EQUAL_INT(0, chmod(path, 0600));
    int safe_mode = storage_s3_stat(&target, "missing", &bytes, error);
    TEST_ASSERT_EQUAL_INT(0, symlink(path, link));
    safe_strcpy(target.credential_ref, "test-credential-link", sizeof(target.credential_ref), 0);
    int symlinked = storage_s3_stat(&target, "missing", &bytes, error);
    unlink(link);
    unlink(path);
    setenv("LIGHTNVR_ARCHIVE_CREDENTIALS_DIR", original, 1);
    free(original);
    TEST_ASSERT_EQUAL_INT(STORAGE_S3_ERROR, unsafe_mode);
    TEST_ASSERT_EQUAL_INT(STORAGE_S3_MISSING, safe_mode);
    TEST_ASSERT_EQUAL_INT(STORAGE_S3_ERROR, symlinked);
}

static void write_lifecycle_attestation(const storage_target_t *target, double checked_at,
                                        const char *xml, char path[1024]) {
    snprintf(path, 1024, "%s/%s.lifecycle.json",
             getenv("LIGHTNVR_ARCHIVE_CREDENTIALS_DIR"), target->credential_ref);
    cJSON *json = cJSON_CreateObject();
    cJSON_AddStringToObject(json, "endpoint", target->endpoint);
    cJSON_AddStringToObject(json, "region", target->region);
    cJSON_AddStringToObject(json, "bucket", target->bucket);
    cJSON_AddNumberToObject(json, "checked_at", checked_at);
    cJSON_AddStringToObject(json, "lifecycle_configuration_xml", xml);
    char *encoded = cJSON_PrintUnformatted(json);
    cJSON_Delete(json);
    FILE *file = fopen(path, "w");
    TEST_ASSERT_NOT_NULL(file);
    fputs(encoded, file);
    fclose(file);
    free(encoded);
    TEST_ASSERT_EQUAL_INT(0, chmod(path, 0600));
}

static void test_scoped_credentials_use_fresh_operator_lifecycle_inspection(void) {
    storage_target_t target = archive_target("scoped-lifecycle", true);
    TEST_ASSERT_NOT_EQUAL_INT(0, storage_s3_probe(&target, true));
    TEST_ASSERT_NOT_NULL(strstr(target.last_error, "operator-verified"));
    char path[1024];
    write_lifecycle_attestation(&target, time(NULL),
        "<LifecycleConfiguration><Rule><ID>abort</ID><Status>Enabled</Status>"
        "<AbortIncompleteMultipartUpload><DaysAfterInitiation>7</DaysAfterInitiation>"
        "</AbortIncompleteMultipartUpload></Rule></LifecycleConfiguration>", path);
    uint64_t id = recording(true);
    storage_migration_job_t job = move_recording(id, &target);
    unlink(path);
    TEST_ASSERT_EQUAL_STRING("completed", job.state);
    TEST_ASSERT_NOT_EQUAL_INT(0, access(source_path, F_OK));
    assert_storage_state(id, "archived", 1, true, "inherit");
    TEST_ASSERT_NOT_EQUAL_INT(0, storage_s3_probe(&target, false));
    TEST_ASSERT_EQUAL_STRING("unavailable", target.health_status);
}

static void test_lifecycle_attestation_rejects_stale_mismatched_and_unsafe_files(void) {
    storage_target_t target = archive_target("scoped-lifecycle", true);
    const char *empty = "<LifecycleConfiguration/>";
    char path[1024];
    const double ages[] = {3601, -60, 86400};
    for (size_t i = 0; i < sizeof(ages)/sizeof(ages[0]); i++) {
        write_lifecycle_attestation(&target, (double)time(NULL) - ages[i], empty, path);
        int result = storage_s3_probe(&target, false);
        unlink(path);
        TEST_ASSERT_NOT_EQUAL_INT(0, result);
    }
    for (int field = 0; field < 3; field++) {
        storage_target_t other = target;
        if (field == 0) safe_strcpy(other.bucket, "another-bucket", sizeof(other.bucket), 0);
        if (field == 1) safe_strcpy(other.region, "another-region", sizeof(other.region), 0);
        if (field == 2) safe_strcpy(other.endpoint, "https://another-provider.invalid", sizeof(other.endpoint), 0);
        write_lifecycle_attestation(&other, time(NULL), empty, path);
        int result = storage_s3_probe(&target, false);
        unlink(path);
        TEST_ASSERT_NOT_EQUAL_INT(0, result);
    }
    const char *unsafe[] = {"garbage", "<WrongRoot/>",
        "<LifecycleConfiguration><Rule><Expiration><Days>1</Days></Expiration></Rule></LifecycleConfiguration>",
        "<LifecycleConfiguration><Rule><AbortIncompleteMultipartUpload><DaysAfterInitiation>8</DaysAfterInitiation></AbortIncompleteMultipartUpload></Rule></LifecycleConfiguration>"};
    for (size_t i = 0; i < sizeof(unsafe)/sizeof(unsafe[0]); i++) {
        write_lifecycle_attestation(&target, time(NULL), unsafe[i], path);
        int result = storage_s3_probe(&target, false);
        unlink(path);
        TEST_ASSERT_NOT_EQUAL_INT(0, result);
    }
    write_lifecycle_attestation(&target, time(NULL), empty, path);
    TEST_ASSERT_EQUAL_INT(0, chmod(path, 0644));
    int public_mode = storage_s3_probe(&target, false);
    TEST_ASSERT_EQUAL_INT(0, chmod(path, 0600));
    char moved[1100];
    snprintf(moved, sizeof(moved), "%s.actual", path);
    TEST_ASSERT_EQUAL_INT(0, rename(path, moved));
    TEST_ASSERT_EQUAL_INT(0, symlink(moved, path));
    int symlinked = storage_s3_probe(&target, false);
    unlink(path);
    unlink(moved);
    TEST_ASSERT_NOT_EQUAL_INT(0, public_mode);
    TEST_ASSERT_NOT_EQUAL_INT(0, symlinked);
}

static void test_s3_nested_keys_preserve_slashes_and_escape_literal_percent(void) {
    storage_target_t target = archive_target("key-encoding", true);
    char checksum[65], error[256];
    unsigned char digest[32];
    TEST_ASSERT_EQUAL_INT(0, mbedtls_sha256((const unsigned char *)payload, strlen(payload), digest, 0));
    for (int i = 0; i < 32; i++) snprintf(checksum + i * 2, 3, "%02x", digest[i]);
    recording(false);
    const char *key = "folder with spaces/literal%2F-plus+question?.mp4";
    TEST_ASSERT_EQUAL_INT(0, storage_s3_upload(&target, key, source_path, checksum, NULL, error));
    TEST_ASSERT_EQUAL_INT(0, storage_s3_verify(&target, key, strlen(payload), checksum, NULL, error));
    uint64_t bytes;
    TEST_ASSERT_EQUAL_INT(STORAGE_S3_MISSING, storage_s3_stat(&target,
        "folder with spaces/literal/-plus+question?.mp4", &bytes, error));
    TEST_ASSERT_EQUAL_INT(0, storage_s3_delete(&target, key, error));
}

static void test_lifecycle_attestation_cannot_override_provider_checks(void) {
    const char *buckets[] = {"versioned", "expiry", "lifecycle-unavailable", "public"};
    for (size_t i = 0; i < sizeof(buckets)/sizeof(buckets[0]); i++) {
        storage_target_t target = archive_target(buckets[i], true);
        char path[1024];
        write_lifecycle_attestation(&target, time(NULL), "<LifecycleConfiguration/>", path);
        int result = storage_s3_probe(&target, true);
        unlink(path);
        TEST_ASSERT_NOT_EQUAL_INT(0, result);
    }
}

static void test_legacy_age_cleanup_honors_archive_retention_override(void) {
    uint64_t id = recording(true);
    storage_target_t target = archive_target("legacy-retention", true);
    storage_migration_job_t job = move_recording(id, &target);
    TEST_ASSERT_EQUAL_STRING("completed", job.state);
    TEST_ASSERT_EQUAL_INT(0, delete_old_recording_metadata(86400));
    TEST_ASSERT_EQUAL_INT(0, set_recording_protected(id, false));
    sql("UPDATE recordings SET retention_override_days=0;");
    TEST_ASSERT_EQUAL_INT(0, delete_old_recording_metadata(86400));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM recordings WHERE deletion_pending=0;"));
    sql("UPDATE recordings SET retention_override_days=30;");
    TEST_ASSERT_EQUAL_INT(0, delete_old_recording_metadata(86400));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM recordings WHERE deletion_pending=0;"));
    sql("UPDATE recordings SET retention_override_days=-1,is_complete=0;");
    TEST_ASSERT_EQUAL_INT(-2, storage_recording_expire_age(id, time(NULL)-86400));
    TEST_ASSERT_EQUAL_INT(0, delete_old_recording_metadata(86400));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM recordings WHERE deletion_pending=0;"));
    sql("UPDATE recordings SET is_complete=1;");
    TEST_ASSERT_EQUAL_INT(0, delete_old_recording_metadata(86400));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM recordings WHERE deletion_pending=1;"));
    TEST_ASSERT_EQUAL_INT(1, storage_deletion_process_one());
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT count(*) FROM recordings;"));
}

static void test_batch_export_waits_for_verified_archive_source(void) {
    uint64_t id = recording(false);
    storage_target_t target = archive_target("batch-export", true);
    storage_migration_job_t job = move_recording(id, &target);
    TEST_ASSERT_EQUAL_STRING("completed", job.state);
    char body[128], token[64], path[160];
    snprintf(body, sizeof(body), "{\"ids\":[%llu],\"filename\":\"archive-export.zip\"}", (unsigned long long)id);
    cJSON *json = api(handle_batch_download_recordings, HTTP_METHOD_POST, "/api/recordings/batch-download", NULL, body, NULL, 202);
    safe_strcpy(token, field(json, "token")->valuestring, sizeof(token), 0);
    cJSON_Delete(json);
    snprintf(path, sizeof(path), "/api/recordings/batch-download/status/%s", token);
    bool completed = false, retrieved = false;
    for (int i = 0; i < 500; i++) {
        if (storage_source_process_one() == 1) retrieved = true;
        json = api(handle_batch_download_status, HTTP_METHOD_GET, path, NULL, NULL, NULL, 200);
        completed = !strcmp(field(json, "status")->valuestring, "complete");
        bool failed = !strcmp(field(json, "status")->valuestring, "error");
        cJSON_Delete(json);
        TEST_ASSERT_FALSE(failed);
        if (completed) break;
        struct timespec delay = {.tv_nsec = 10000000};
        nanosleep(&delay, NULL);
    }
    TEST_ASSERT_TRUE(completed);
    TEST_ASSERT_TRUE(retrieved);
    // Inspect the ZIP produced through the real handler's deferred-file contract.
    uv_loop_t loop;
    TEST_ASSERT_EQUAL_INT(0, uv_loop_init(&loop));
    libuv_server_t server = {.loop = &loop};
    libuv_connection_t *conn = libuv_connection_create(&server);
    TEST_ASSERT_NOT_NULL(conn);
    conn->handler_on_worker = true;
    conn->request.user_data = conn;
    conn->request.method = HTTP_METHOD_GET;
    snprintf(conn->request.path, sizeof(conn->request.path), "/api/recordings/batch-download/result/%s", token);
    handle_batch_download_result(&conn->request, &conn->response);
    TEST_ASSERT_EQUAL_INT(200, conn->response.status_code);
    TEST_ASSERT_TRUE(conn->deferred_file_serve);
    FILE *zip = fopen(conn->deferred_file_path, "rb");
    TEST_ASSERT_NOT_NULL(zip);
    unsigned char header[30], content[128];
    TEST_ASSERT_EQUAL_UINT(sizeof(header), fread(header, 1, sizeof(header), zip));
    TEST_ASSERT_EQUAL_MEMORY("PK\003\004", header, 4);
    unsigned name_length = header[26] | (header[27] << 8);
    unsigned extra_length = header[28] | (header[29] << 8);
    TEST_ASSERT_EQUAL_INT(0, fseek(zip, name_length + extra_length, SEEK_CUR));
    TEST_ASSERT_EQUAL_UINT(strlen(payload), fread(content, 1, strlen(payload), zip));
    TEST_ASSERT_EQUAL_MEMORY(payload, content, strlen(payload));
    fclose(zip);
    unlink(conn->deferred_file_path);
    conn->handler_on_worker = false;
    libuv_connection_close(conn);
    uv_run(&loop, UV_RUN_DEFAULT);
    TEST_ASSERT_EQUAL_INT(0, uv_loop_close(&loop));
}

static void test_expired_multipart_upload_restarts_from_source(void) {
    const char *buckets[] = {"multipart-expired", "multipart-expired-200", "multipart-part-expired"};
    for (int mode = 0; mode < 3; mode++) {
        uint64_t id = recording(false);
        FILE *file = fopen(source_path, "wb");
        TEST_ASSERT_NOT_NULL(file);
        unsigned char block[65536] = {0};
        for (int i = 0; i < 144; i++) TEST_ASSERT_EQUAL_UINT(sizeof(block), fwrite(block, 1, sizeof(block), file));
        fclose(file);
        char query[128];
        snprintf(query, sizeof(query), "UPDATE recordings SET size_bytes=9437184 WHERE id=%llu;", (unsigned long long)id);
        sql(query);
        storage_target_t target = archive_target(buckets[mode], true);
        storage_migration_job_t job = move_recording(id, &target);
        TEST_ASSERT_EQUAL_STRING("retry_wait", job.state);
        TEST_ASSERT_EQUAL_INT(0, access(source_path, F_OK));
        TEST_ASSERT_EQUAL_INT(0, scalar("SELECT count(*) FROM storage_migration_jobs WHERE upload_id<>'';"));
        TEST_ASSERT_EQUAL_INT(0, scalar("SELECT count(*) FROM storage_migration_jobs WHERE state='retry_wait' AND json_array_length(upload_parts,'$.parts')<>0;"));
        shutdown_database();
        TEST_ASSERT_EQUAL_INT(0, init_database_ex(db_path, DB_INIT_NO_BACKUP | DB_INIT_NO_CHECK));
        sql("UPDATE storage_migration_jobs SET next_attempt_at=0;");
        TEST_ASSERT_EQUAL_INT(1, storage_migration_process_one());
        TEST_ASSERT_EQUAL_INT(DB_STORAGE_MIGRATION_OK, db_storage_migration_get(job.uuid, &job));
        TEST_ASSERT_EQUAL_STRING("completed", job.state);
        TEST_ASSERT_NOT_EQUAL_INT(0, access(source_path, F_OK));
    }
}

static void test_unpublished_corrupt_destination_is_repaired(void) {
    for (int same_size = 0; same_size < 3; same_size++) {
        uint64_t id = recording(false);
        storage_target_t target = archive_target(same_size == 2 ? "retained-corrupt" : (same_size ? "stale-hash" : "stale-size"), true);
        TEST_ASSERT_EQUAL_INT(DB_STORAGE_TARGET_OK, db_storage_target_create(&target));
        storage_migration_job_t job;
        TEST_ASSERT_EQUAL_INT(DB_STORAGE_MIGRATION_OK, db_storage_migration_create(id, target.uuid, 0, &job));
        char corrupt[512], error[256], hash[65];
        unsigned char digest[32];
        snprintf(corrupt, sizeof(corrupt), "%s/stale.mp4", root);
        FILE *file = fopen(corrupt, "wb");
        TEST_ASSERT_NOT_NULL(file);
        size_t bytes = same_size ? strlen(payload) : 3;
        char data[128];
        memset(data, '!', bytes);
        TEST_ASSERT_EQUAL_UINT(bytes, fwrite(data, 1, bytes, file));
        fclose(file);
        TEST_ASSERT_EQUAL_INT(0, mbedtls_sha256((unsigned char *)data, bytes, digest, 0));
        for (int i = 0; i < 32; i++) snprintf(hash + i * 2, 3, "%02x", digest[i]);
        TEST_ASSERT_EQUAL_INT(0, storage_s3_upload(&target, job.destination_object_key, corrupt, hash, NULL, error));
        unlink(corrupt);
        if (same_size == 2) {
            char query[1500];
            snprintf(query, sizeof(query), "INSERT INTO storage_recording_copies(recording_id,target_uuid,object_key,checksum,size_bytes) VALUES(%llu,'%s','%s','%s',%zu);",
                     (unsigned long long)id, target.uuid, job.destination_object_key, hash, bytes);
            sql(query);
        }
        TEST_ASSERT_EQUAL_INT(1, storage_migration_process_one());
        TEST_ASSERT_EQUAL_INT(DB_STORAGE_MIGRATION_OK, db_storage_migration_get(job.uuid, &job));
        if (same_size == 2) {
            TEST_ASSERT_EQUAL_STRING("retry_wait", job.state);
            TEST_ASSERT_EQUAL_INT(0, access(source_path, F_OK));
            TEST_ASSERT_EQUAL_INT(0, storage_s3_verify(&target, job.destination_object_key, bytes, hash, NULL, error));
            continue;
        }
        TEST_ASSERT_EQUAL_STRING("completed", job.state);
        TEST_ASSERT_EQUAL_INT(0, storage_s3_verify(&target, job.destination_object_key, strlen(payload), job.checksum, NULL, error));
    }
}

static void test_archive_replica_failover_and_local_status(void) {
    uint64_t id = recording(false);
    storage_target_t primary = archive_target("failover-primary", true);
    storage_target_t replica = archive_target("failover-replica", true);
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_TARGET_OK, db_storage_target_create(&primary));
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_TARGET_OK, db_storage_target_create(&replica));
    storage_migration_job_t copy, job;
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_MIGRATION_OK, db_storage_migration_create_operation(id, replica.uuid, "copy", 0, &copy));
    TEST_ASSERT_EQUAL_INT(1, storage_migration_process_one());
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_MIGRATION_OK, db_storage_migration_create(id, primary.uuid, 0, &job));
    TEST_ASSERT_EQUAL_INT(1, storage_migration_process_one());
    TEST_ASSERT_EQUAL_INT(DB_STORAGE_MIGRATION_OK, db_storage_migration_get(job.uuid, &job));
    TEST_ASSERT_EQUAL_STRING("completed", job.state);
    char path[MAX_PATH_LENGTH], error[256], query[1600];
    TEST_ASSERT_EQUAL_INT(0, storage_s3_delete(&primary, job.destination_object_key, error));
    TEST_ASSERT_EQUAL_INT(STORAGE_SOURCE_PREPARING, storage_source_resolve(id, path, error));
    TEST_ASSERT_EQUAL_INT(1, storage_source_process_one());
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM storage_retrieval_jobs WHERE state='failed';"));
    // A failed job must not mask the other healthy, verified replica.
    assert_storage_state(id, "archived", 2, true, "inherit");
    storage_remote_source_t remote;
    TEST_ASSERT_EQUAL_INT(STORAGE_SOURCE_READY, storage_source_remote(id, &remote));
    TEST_ASSERT_EQUAL_STRING(replica.uuid, remote.target.uuid);
    TEST_ASSERT_EQUAL_INT(STORAGE_SOURCE_PREPARING, storage_source_resolve(id, path, error));
    snprintf(query, sizeof(query), "SELECT count(*) FROM storage_retrieval_jobs WHERE target_uuid='%s' AND state='queued';", replica.uuid);
    TEST_ASSERT_EQUAL_INT(1, scalar(query));
    TEST_ASSERT_EQUAL_INT(1, storage_source_process_one());
    TEST_ASSERT_EQUAL_INT(STORAGE_SOURCE_READY, storage_source_resolve(id, path, error));
    sql("UPDATE storage_targets SET health_status='unavailable' WHERE target_type='s3';");
    // A completed verified cache can serve playback even with both providers down.
    TEST_ASSERT_EQUAL_INT(STORAGE_SOURCE_MISSING, storage_source_remote(id, &remote));
    assert_storage_state(id, "archived", 2, true, "inherit");
    TEST_ASSERT_EQUAL_INT(0, unlink(path));
    sql("DELETE FROM storage_retrieval_jobs;");
    FILE *local = fopen(source_path, "wb");
    TEST_ASSERT_NOT_NULL(local);
    fwrite(payload, 1, strlen(payload), local);
    fclose(local);
    snprintf(query, sizeof(query), "INSERT INTO storage_recording_copies(recording_id,target_uuid,object_key,checksum,size_bytes) VALUES(%llu,'%s','clip.mp4','%s',%zu);",
             (unsigned long long)id, source_uuid, job.checksum, strlen(payload));
    sql(query);
    assert_storage_state(id, "archived", 3, true, "inherit");
    TEST_ASSERT_EQUAL_INT(STORAGE_SOURCE_MISSING, storage_source_remote(id, &remote));
    TEST_ASSERT_EQUAL_INT(STORAGE_SOURCE_READY, storage_source_resolve(id, path, error));
    TEST_ASSERT_EQUAL_STRING(source_path, path);
}

static void test_retrieval_queue_counts_only_active_work(void) {
    uint64_t id = recording(false);
    storage_target_t target = archive_target("queue", true);
    storage_migration_job_t job = move_recording(id, &target);
    TEST_ASSERT_EQUAL_STRING("completed", job.state);
    for (int i = 0; i < 16; i++) {
        uint64_t other = recording(false);
        char query[1500];
        snprintf(query, sizeof(query), "INSERT INTO storage_retrieval_jobs(recording_id,target_uuid,object_key,checksum,size_bytes,file_path) VALUES(%llu,'%s','unused','%s',1,'%s/unused');",
                 (unsigned long long)other, target.uuid, job.checksum, root);
        sql(query);
    }
    char path[MAX_PATH_LENGTH], error[256];
    TEST_ASSERT_EQUAL_INT(STORAGE_SOURCE_ERROR, storage_source_resolve(id, path, error));
    sql("UPDATE storage_retrieval_jobs SET state=CASE WHEN recording_id%2=0 THEN 'ready' ELSE 'failed' END;");
    TEST_ASSERT_EQUAL_INT(STORAGE_SOURCE_PREPARING, storage_source_resolve(id, path, error));
    TEST_ASSERT_EQUAL_INT(1, storage_source_process_one());
    TEST_ASSERT_EQUAL_INT(STORAGE_SOURCE_READY, storage_source_resolve(id, path, error));
}

static void test_catalog_failure_only_reclaims_idle_owned_cache(void) {
    uint64_t id = recording(false);
    storage_target_t target = archive_target("cache-fallback", true);
    storage_migration_job_t job = move_recording(id, &target);
    char path[MAX_PATH_LENGTH], error[256], recent[MAX_PATH_LENGTH], other[MAX_PATH_LENGTH], link[MAX_PATH_LENGTH];
    TEST_ASSERT_EQUAL_INT(STORAGE_SOURCE_PREPARING, storage_source_resolve(id, path, error));
    TEST_ASSERT_EQUAL_INT(1, storage_source_process_one());
    TEST_ASSERT_EQUAL_INT(STORAGE_SOURCE_READY, storage_source_resolve(id, path, error));
    snprintf(recent, sizeof(recent), "%s/archive-cache/999-%s.mp4", root, job.checksum);
    snprintf(link, sizeof(link), "%s/archive-cache/998-%s.mp4", root, job.checksum);
    snprintf(other, sizeof(other), "%s/archive-cache/operator-file.mp4", root);
    FILE *file = fopen(recent, "wb"); TEST_ASSERT_NOT_NULL(file); fputs("recent", file); fclose(file);
    file = fopen(other, "wb"); TEST_ASSERT_NOT_NULL(file); fputs("preserve", file); fclose(file);
    TEST_ASSERT_EQUAL_INT(0, symlink(other, link));
    struct timespec old[2] = {{.tv_sec = time(NULL)-7200}, {.tv_sec = time(NULL)-7200}};
    TEST_ASSERT_EQUAL_INT(0, utimensat(AT_FDCWD, path, old, 0));
    TEST_ASSERT_EQUAL_INT(0, utimensat(AT_FDCWD, other, old, 0));
    shutdown_database();
    uint64_t removed = storage_source_trim(strlen(payload));
    TEST_ASSERT_EQUAL_INT(0, init_database_ex(db_path, DB_INIT_NO_BACKUP | DB_INIT_NO_CHECK));
    TEST_ASSERT_EQUAL_UINT(strlen(payload), removed);
    TEST_ASSERT_NOT_EQUAL_INT(0, access(path, F_OK));
    TEST_ASSERT_EQUAL_INT(0, access(recent, F_OK));
    TEST_ASSERT_EQUAL_INT(0, access(other, F_OK));
    TEST_ASSERT_EQUAL_INT(0, access(link, F_OK));
    TEST_ASSERT_EQUAL_INT(0, storage_s3_verify(&target, job.destination_object_key, strlen(payload), job.checksum, NULL, error));
    unlink(recent); unlink(other); unlink(link);
    TEST_ASSERT_EQUAL_INT(STORAGE_SOURCE_PREPARING, storage_source_resolve(id, path, error));
    TEST_ASSERT_EQUAL_INT(1, storage_source_process_one());
    TEST_ASSERT_EQUAL_INT(0, utimensat(AT_FDCWD, path, old, 0));
    // An open database with an unusable cache catalog must use the same safe scan.
    sql("ALTER TABLE storage_retrieval_jobs RENAME TO inaccessible_retrieval_jobs;");
    removed = storage_source_trim(strlen(payload));
    sql("ALTER TABLE inaccessible_retrieval_jobs RENAME TO storage_retrieval_jobs;");
    TEST_ASSERT_EQUAL_UINT(strlen(payload), removed);
    TEST_ASSERT_NOT_EQUAL_INT(0, access(path, F_OK));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM recordings WHERE deletion_pending=0;"));
    TEST_ASSERT_EQUAL_INT(1, delete_recording_metadata(id));
    TEST_ASSERT_EQUAL_INT(1, delete_recording_metadata(id));
    sql("UPDATE storage_read_leases SET expires_at=0;");
    TEST_ASSERT_EQUAL_INT(1, storage_deletion_process_one());
    for (int i = 0; i < 16 && scalar("SELECT count(*) FROM recordings;"); i++) storage_deletion_process_one();
    TEST_ASSERT_EQUAL_INT(0, delete_recording_metadata(id));
}

static void test_missing_source_mount_defers_archive_and_filesystem_moves(void) {
    for (int filesystem = 0; filesystem < 2; filesystem++) {
        uint64_t id = recording(false);
        storage_target_t target = archive_target(filesystem ? "mount-fs" : "mount-s3", true);
        if (filesystem) {
            safe_strcpy(target.target_type, "filesystem", sizeof(target.target_type), 0);
            snprintf(target.root_path, sizeof(target.root_path), "%s/mounted-destination", root);
            TEST_ASSERT_EQUAL_INT(0, mkdir(target.root_path, 0700));
        }
        TEST_ASSERT_EQUAL_INT(DB_STORAGE_TARGET_OK, db_storage_target_create(&target));
        storage_migration_job_t job;
        TEST_ASSERT_EQUAL_INT(DB_STORAGE_MIGRATION_OK, db_storage_migration_create(id, target.uuid, 0, &job));
        char query[1600], path[MAX_PATH_LENGTH], error[256];
        // Simulate an unmounted target whose underlying directory/file remain.
        snprintf(query, sizeof(query), "UPDATE storage_targets SET mount_required=1,mount_guard_path='%s' WHERE uuid='%s';", root, source_uuid);
        sql(query);
        TEST_ASSERT_EQUAL_INT(1, storage_migration_process_one());
        TEST_ASSERT_EQUAL_INT(DB_STORAGE_MIGRATION_OK, db_storage_migration_get(job.uuid, &job));
        TEST_ASSERT_EQUAL_STRING("retry_wait", job.state);
        TEST_ASSERT_EQUAL_UINT64(0, job.bytes_copied);
        TEST_ASSERT_EQUAL_INT(0, access(source_path, R_OK));
        if (filesystem) {
            TEST_ASSERT_EQUAL_INT(0, db_storage_target_resolve_path(target.uuid, job.destination_object_key, path));
            TEST_ASSERT_NOT_EQUAL_INT(0, access(path, F_OK));
        } else {
            uint64_t bytes;
            TEST_ASSERT_EQUAL_INT(STORAGE_S3_MISSING, storage_s3_stat(&target, job.destination_object_key, &bytes, error));
        }
        recording_metadata_t metadata;
        TEST_ASSERT_EQUAL_INT(0, get_recording_metadata_by_id(id, &metadata));
        TEST_ASSERT_EQUAL_STRING(source_uuid, metadata.storage_target_uuid);
        // Restore the fixture's original local target. CI containers need not
        // have a separate mount beneath /, and mount detection excludes /.
        snprintf(query, sizeof(query), "UPDATE storage_targets SET mount_required=0,mount_guard_path='' WHERE uuid='%s';UPDATE storage_migration_jobs SET next_attempt_at=0;", source_uuid);
        sql(query);
        TEST_ASSERT_EQUAL_INT(1, storage_migration_process_one());
        TEST_ASSERT_EQUAL_INT(DB_STORAGE_MIGRATION_OK, db_storage_migration_get(job.uuid, &job));
        TEST_ASSERT_EQUAL_STRING("completed", job.state);
    }
}

static void test_storage_workers_do_not_rollback_an_unowned_transaction(void) {
    sql("BEGIN IMMEDIATE;INSERT INTO storage_read_leases(recording_id,expires_at) SELECT id,0 FROM recordings;");
    int reconciled = db_storage_lifecycle_reconcile();
    bool reconcile_preserved = !sqlite3_get_autocommit(get_db_handle());
    storage_deletion_process_one();
    bool deletion_preserved = !sqlite3_get_autocommit(get_db_handle());
    sql("ROLLBACK;");
    TEST_ASSERT_EQUAL_INT(-1, reconciled);
    TEST_ASSERT_TRUE(reconcile_preserved);
    TEST_ASSERT_TRUE(deletion_preserved);
}

static libuv_connection_t *stalled_archive_connection(libuv_server_t *server, int *client, uint64_t id) {
    int listener = socket(AF_INET, SOCK_STREAM, 0);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, listener);
    struct sockaddr_in address = {.sin_family=AF_INET, .sin_addr.s_addr=htonl(INADDR_LOOPBACK)};
    TEST_ASSERT_EQUAL_INT(0, bind(listener, (struct sockaddr *)&address, sizeof(address)));
    TEST_ASSERT_EQUAL_INT(0, listen(listener, 1));
    socklen_t length = sizeof(address);
    TEST_ASSERT_EQUAL_INT(0, getsockname(listener, (struct sockaddr *)&address, &length));
    *client = socket(AF_INET, SOCK_STREAM, 0);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, *client);
    TEST_ASSERT_EQUAL_INT(0, connect(*client, (struct sockaddr *)&address, length));
    int peer = accept(listener, NULL, NULL);
    close(listener);
    TEST_ASSERT_GREATER_OR_EQUAL_INT(0, peer);
    int small = 4096;
    TEST_ASSERT_EQUAL_INT(0, setsockopt(peer, SOL_SOCKET, SO_SNDBUF, &small, sizeof(small)));
    libuv_connection_t *conn = libuv_connection_create(server);
    TEST_ASSERT_NOT_NULL(conn);
    TEST_ASSERT_EQUAL_INT(0, uv_tcp_open(&conn->handle, peer));
    conn->keep_alive = false;
    conn->request.user_data = conn;
    conn->request.method = HTTP_METHOD_GET;
    TEST_ASSERT_TRUE(recording_archive_serve(&conn->request, &conn->response, id, false));
    return conn;
}

static void test_stalled_archive_writes_renew_lease_until_completion(void) {
    uint64_t id = recording(false);
    FILE *file = fopen(source_path, "wb");
    TEST_ASSERT_NOT_NULL(file);
    unsigned char block[65536] = {0};
    for (int i = 0; i < 48; i++) TEST_ASSERT_EQUAL_UINT(sizeof(block), fwrite(block, 1, sizeof(block), file));
    fclose(file);
    sql("UPDATE recordings SET size_bytes=3145728;");
    storage_target_t target = archive_target("stalled-reader", true);
    storage_migration_job_t job = move_recording(id, &target);
    TEST_ASSERT_EQUAL_STRING("completed", job.state);
    uv_loop_t loop;
    TEST_ASSERT_EQUAL_INT(0, uv_loop_init(&loop));
    libuv_server_t server = {.loop = &loop};
    int clients[2];
    capture_lease_timers = true;
    libuv_connection_t *first = stalled_archive_connection(&server, &clients[0], id);
    libuv_connection_t *second = stalled_archive_connection(&server, &clients[1], id);
    capture_lease_timers = false;
    TEST_ASSERT_EQUAL_INT(2, captured_timer_count);
    bool stalled = false;
    for (int i = 0; i < 5000; i++) {
        uv_run(&loop, UV_RUN_NOWAIT);
        stalled = uv_stream_get_write_queue_size((uv_stream_t *)&first->handle) > 0 &&
                  uv_stream_get_write_queue_size((uv_stream_t *)&second->handle) > 0;
        if (stalled) break;
        struct timespec pause = {.tv_nsec = 1000000}; nanosleep(&pause, NULL);
    }
    TEST_ASSERT_TRUE(stalled);
    // Advance the database's lease clock without a two-minute wall-clock sleep.
    sql("UPDATE storage_read_leases SET expires_at=0;");
    captured_callbacks[0](captured_timers[0]);
    TEST_ASSERT_TRUE(storage_source_has_lease(id));
    TEST_ASSERT_EQUAL_INT(1, storage_recording_delete(id, "delete during stalled playback", NULL));
    TEST_ASSERT_EQUAL_INT(0, storage_deletion_process_one());
    // Exercise server shutdown's timer close callback while a write is pending.
    TEST_ASSERT_FALSE(recording_archive_close_timer((uv_handle_t *)&first->handle));
    TEST_ASSERT_TRUE(recording_archive_close_timer((uv_handle_t *)captured_timers[0]));
    close(clients[0]);
    libuv_connection_close(first);
    for (int i = 0; i < 10; i++) uv_run(&loop, UV_RUN_NOWAIT);
    sql("UPDATE storage_read_leases SET expires_at=0;");
    captured_callbacks[1](captured_timers[1]);
    TEST_ASSERT_TRUE(storage_source_has_lease(id));
    TEST_ASSERT_EQUAL_INT(0, storage_deletion_process_one());
    storage_remote_source_t rejected;
    TEST_ASSERT_EQUAL_INT(STORAGE_SOURCE_DELETING, storage_source_remote(id, &rejected));

    // The remaining reader may finish even though new readers are excluded.
    TEST_ASSERT_EQUAL_INT(0, fcntl(clients[1], F_SETFL, O_NONBLOCK));
    const size_t capacity = 3145728 + 2048;
    char *response = calloc(1, capacity);
    TEST_ASSERT_NOT_NULL(response);
    size_t total = 0;
    bool ended = false;
    for (int i = 0; i < 15000; i++) {
        uv_run(&loop, UV_RUN_NOWAIT);
        ssize_t count = recv(clients[1], response + total, capacity - total - 1, 0);
        if (count > 0) total += (size_t)count;
        else if (count == 0) { ended = true; break; }
        else TEST_ASSERT_TRUE(errno == EAGAIN || errno == EWOULDBLOCK || errno == EINTR);
        struct timespec pause = {.tv_nsec = 1000000}; nanosleep(&pause, NULL);
    }
    close(clients[1]);
    uv_run(&loop, UV_RUN_DEFAULT);
    TEST_ASSERT_EQUAL_INT(0, uv_loop_close(&loop));
    TEST_ASSERT_TRUE(ended);
    char *body = strstr(response, "\r\n\r\n");
    TEST_ASSERT_NOT_NULL(body);
    body += 4;
    TEST_ASSERT_EQUAL_UINT(3145728, total - (size_t)(body - response));
    unsigned char digest[32]; char hash[65];
    TEST_ASSERT_EQUAL_INT(0, mbedtls_sha256((unsigned char *)body, 3145728, digest, 0));
    for (int i = 0; i < 32; i++) snprintf(hash + i * 2, 3, "%02x", digest[i]);
    free(response);
    TEST_ASSERT_EQUAL_STRING(job.checksum, hash);
    // Once both responses finish, renewal stops and normal expiry permits cleanup.
    sql("UPDATE storage_read_leases SET expires_at=0;");
    for (int i = 0; i < 16 && scalar("SELECT count(*) FROM recordings;"); i++) storage_deletion_process_one();
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT count(*) FROM recordings;"));
}

int main(void) {
    if (!getenv("LIGHTNVR_TEST_S3_ENDPOINT") || !mkdtemp(root)) return 2;
    snprintf(db_path, sizeof(db_path), "%s/catalog.sqlite", root);
    snprintf(source_path, sizeof(source_path), "%s/clip.mp4", root);
    safe_strcpy(g_config.storage_path, root, sizeof(g_config.storage_path), 0);
    if (init_database_ex(db_path, DB_INIT_NO_BACKUP | DB_INIT_NO_CHECK)) return 2;
    stream_config_t camera = {0};
    safe_strcpy(camera.name, "archive-camera", sizeof(camera.name), 0);
    safe_strcpy(camera.url, "rtsp://fixture/live", sizeof(camera.url), 0);
    camera.enabled = true;
    if (!add_stream_config(&camera)) return 2;
    UNITY_BEGIN();
    RUN_TEST(test_verified_archive_and_retrieval);
    RUN_TEST(test_verification_failure_preserves_source);
    RUN_TEST(test_protected_recording_archives_after_ordinary_expiry);
    RUN_TEST(test_lifecycle_scheduler_with_large_migration_backlog);
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
    RUN_TEST(test_archive_admin_summary_retry_and_permissions);
    RUN_TEST(test_recording_status_follows_archive_and_retrieval_lifecycle);
    RUN_TEST(test_archive_http_rejects_invalid_ranges_and_bounds_concurrency);
    RUN_TEST(test_cache_eviction_preserves_archive_and_active_reader);
    RUN_TEST(test_corrupt_retrieval_never_publishes_cache);
    RUN_TEST(test_s3_configuration_api_and_archive_budget);
    RUN_TEST(test_multipart_cancellation_aborts_owned_upload_and_preserves_source);
    RUN_TEST(test_archive_credentials_require_private_regular_files);
    RUN_TEST(test_scoped_credentials_use_fresh_operator_lifecycle_inspection);
    RUN_TEST(test_lifecycle_attestation_rejects_stale_mismatched_and_unsafe_files);
    RUN_TEST(test_lifecycle_attestation_cannot_override_provider_checks);
    RUN_TEST(test_s3_nested_keys_preserve_slashes_and_escape_literal_percent);
    RUN_TEST(test_batch_export_waits_for_verified_archive_source);
    RUN_TEST(test_legacy_age_cleanup_honors_archive_retention_override);
    RUN_TEST(test_expired_multipart_upload_restarts_from_source);
    RUN_TEST(test_unpublished_corrupt_destination_is_repaired);
    RUN_TEST(test_archive_replica_failover_and_local_status);
    RUN_TEST(test_retrieval_queue_counts_only_active_work);
    RUN_TEST(test_catalog_failure_only_reclaims_idle_owned_cache);
    RUN_TEST(test_missing_source_mount_defers_archive_and_filesystem_moves);
    RUN_TEST(test_storage_workers_do_not_rollback_an_unowned_transaction);
    RUN_TEST(test_stalled_archive_writes_renew_lease_until_completion);
    int result = UNITY_END();
    shutdown_database();
    return result;
}
