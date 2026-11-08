/* INT_01 L10: observation pipeline diagnostics. Metric and JSON
 * serialization, disabled-mode zeros (no queue allocated), DB-derived
 * publication/history counts, counters moving under an evidence queue
 * saturation, and the contract section 8.2 Retry-After rule. */
#define _GNU_SOURCE
#include <pthread.h>
#include <sqlite3.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <time.h>
#include <unistd.h>

#include <cjson/cJSON.h>

#include "unity.h"
#include "core/event_identity.h"
#include "core/observation_diagnostics.h"
#include "core/observation_evidence.h"
#include "database/db_core.h"
#include "database/db_observations.h"
#include "database/db_streams.h"
#include "utils/strings.h"
#include "utils/uuid.h"
#include "video/ffmpeg_utils.h"
#include "web/api_handlers_metrics.h"

static char db_path[256];
static char storage_dir[256];
static char cam_a[OBS_UUID_SIZE];

/* ---- helpers ---------------------------------------------------------- */

static void exec_sql(const char *sql) {
    char *error = NULL;
    pthread_mutex_lock(get_db_mutex());
    int rc = sqlite3_exec(get_db_handle(), sql, NULL, NULL, &error);
    pthread_mutex_unlock(get_db_mutex());
    if (rc != SQLITE_OK) fprintf(stderr, "%s: %s\n", sql, error ? error : "");
    sqlite3_free(error);
    TEST_ASSERT_EQUAL_INT_MESSAGE(SQLITE_OK, rc, sql);
}

static void make_camera(const char *name, char out[OBS_UUID_SIZE]) {
    stream_config_t *s = calloc(1, sizeof(*s));
    TEST_ASSERT_NOT_NULL(s);
    safe_strcpy(s->name, name, sizeof(s->name), 0);
    safe_strcpy(s->url, "rtsp://camera/stream", sizeof(s->url), 0);
    safe_strcpy(s->codec, "h264", sizeof(s->codec), 0);
    s->enabled = true;
    s->priority = 5;
    s->segment_duration = 60;
    safe_strcpy(s->detection_object_filter, "none", sizeof(s->detection_object_filter), 0);
    TEST_ASSERT_GREATER_THAN(0, add_stream_config(s));
    memset(s, 0, sizeof(*s));
    TEST_ASSERT_EQUAL_INT(0, get_stream_config_by_name(name, s));
    safe_strcpy(out, s->camera_uuid, OBS_UUID_SIZE, 0);
    free(s);
}

static int64_t base_time_ms = 1786000000000LL;

static observation_batch_input_t make_batch(int objects) {
    observation_batch_input_t batch;
    memset(&batch, 0, sizeof(batch));
    TEST_ASSERT_EQUAL_INT(0, lightnvr_uuid_generate_v4(batch.batch_id));
    safe_strcpy(batch.camera_uuid, cam_a, sizeof(batch.camera_uuid), 0);
    batch.observed_at_ms = base_time_ms++;
    safe_strcpy(batch.engine_key, "legacy-primary", sizeof(batch.engine_key), 0);
    safe_strcpy(batch.engine_type, "object", sizeof(batch.engine_type), 0);
    batch.object_count = objects;
    for (int i = 0; i < objects; ++i) {
        batch.objects[i].object_index = i;
        safe_strcpy(batch.objects[i].label, "bird", sizeof(batch.objects[i].label), 0);
        batch.objects[i].confidence = 0.9;
    }
    return batch;
}

static void insert_pending(int objects) {
    observation_batch_input_t batch = make_batch(objects);
    observation_asset_input_t asset;
    memset(&asset, 0, sizeof(asset));
    asset.state = OBS_ASSET_PENDING;
    safe_strcpy(asset.origin, "inference_input", sizeof(asset.origin), 0);
    safe_strcpy(asset.media_type, "image/jpeg", sizeof(asset.media_type), 0);
    observation_insert_result_t result;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_insert_batch(&batch, &asset, &result));
}

static uint8_t *make_rgb(int width, int height) {
    size_t size = (size_t)width * (size_t)height * 3;
    uint8_t *buf = calloc(1, size + RGB24_SWS_TAIL_PADDING);
    TEST_ASSERT_NOT_NULL(buf);
    return buf;
}

static bool has_line(const char *text, const char *line) {
    size_t length = strlen(line);
    for (const char *p = strstr(text, line); p; p = strstr(p + 1, line)) {
        bool starts = p == text || p[-1] == '\n';
        bool ends = p[length] == '\n' || p[length] == '\0';
        if (starts && ends) return true;
    }
    return false;
}

/* Unity is built without double support. */
#define ASSERT_NUM(expected, actual) \
    TEST_ASSERT_EQUAL_INT64((int64_t)((expected) * 1000.0), (int64_t)((actual) * 1000.0))

#define ASSERT_LINE(text, line) TEST_ASSERT_TRUE_MESSAGE(has_line((text), (line)), (line))

static double json_number(const cJSON *object, const char *path) {
    char copy[256];
    safe_strcpy(copy, path, sizeof(copy), 0);
    const cJSON *node = object;
    for (char *save = NULL, *part = strtok_r(copy, ".", &save); part;
         part = strtok_r(NULL, ".", &save))
        node = cJSON_GetObjectItemCaseSensitive(node, part);
    TEST_ASSERT_TRUE_MESSAGE(cJSON_IsNumber(node), path);
    return node->valuedouble;
}

static bool json_bool(const cJSON *object, const char *path) {
    char copy[256];
    safe_strcpy(copy, path, sizeof(copy), 0);
    const cJSON *node = object;
    for (char *save = NULL, *part = strtok_r(copy, ".", &save); part;
         part = strtok_r(NULL, ".", &save))
        node = cJSON_GetObjectItemCaseSensitive(node, part);
    TEST_ASSERT_TRUE_MESSAGE(cJSON_IsBool(node), path);
    return cJSON_IsTrue(node);
}

/* ---- stall hook (saturation) ------------------------------------------ */

static pthread_mutex_t hook_lock = PTHREAD_MUTEX_INITIALIZER;
static pthread_cond_t hook_cond = PTHREAD_COND_INITIALIZER;
static bool stall_enabled;
static bool stall_entered;

static int stall_hook(observation_evidence_stage_t stage, const char *path, void *ctx) {
    (void)path; (void)ctx;
    if (stage != OBS_EVIDENCE_STAGE_ITEM_BEGIN) return 0;
    pthread_mutex_lock(&hook_lock);
    stall_entered = true;
    pthread_cond_broadcast(&hook_cond);
    while (stall_enabled) pthread_cond_wait(&hook_cond, &hook_lock);
    pthread_mutex_unlock(&hook_lock);
    return 0;
}

static void release_worker(void) {
    pthread_mutex_lock(&hook_lock);
    stall_enabled = false;
    pthread_cond_broadcast(&hook_cond);
    pthread_mutex_unlock(&hook_lock);
}

static void wait_stalled(void) {
    struct timespec deadline;
    clock_gettime(CLOCK_REALTIME, &deadline);
    deadline.tv_sec += 5;
    pthread_mutex_lock(&hook_lock);
    while (!stall_entered)
        if (pthread_cond_timedwait(&hook_cond, &hook_lock, &deadline) != 0) break;
    bool entered = stall_entered;
    pthread_mutex_unlock(&hook_lock);
    TEST_ASSERT_TRUE_MESSAGE(entered, "worker never reached the stall hook");
}

/* ---- fixtures --------------------------------------------------------- */

void setUp(void) {
    exec_sql("DELETE FROM object_observations; DELETE FROM observation_assets;");
    observation_relay_reset_stats();
    stall_enabled = false;
    stall_entered = false;
}

void tearDown(void) {
    release_worker();
    observation_evidence_shutdown(0);
    observation_evidence_set_stage_hook(NULL, NULL);
}

/* ---- tests ------------------------------------------------------------ */

void test_disabled_pipeline_reports_zeros_and_no_queue(void) {
    observation_evidence_shutdown(0);
    observation_diagnostics_t diag;
    TEST_ASSERT_EQUAL_INT(0, observation_diagnostics_collect(&diag, 0));
    TEST_ASSERT_FALSE(diag.evidence.allocated);
    TEST_ASSERT_FALSE(diag.evidence.worker_running);
    TEST_ASSERT_EQUAL_UINT64(0, diag.evidence.accepted);
    TEST_ASSERT_EQUAL_UINT32(0, diag.evidence.backlog_items);
    TEST_ASSERT_TRUE(diag.db_available);
    TEST_ASSERT_EQUAL_INT64(0, diag.db.publications_pending);
    TEST_ASSERT_EQUAL_INT64(0, diag.db.observations);
    TEST_ASSERT_FALSE(diag.db.has_oldest_observed_at);
    TEST_ASSERT_EQUAL_INT(0, diag.camera_count);
    ASSERT_NUM(0.0, observation_diagnostics_relay_lag_seconds(&diag));

    char *text = api_metrics_render_observations(&diag);
    TEST_ASSERT_NOT_NULL(text);
    ASSERT_LINE(text, "lightnvr_observation_accepted_total 0");
    ASSERT_LINE(text, "lightnvr_observation_dropped_total{reason=\"queue_full\"} 0");
    ASSERT_LINE(text, "lightnvr_observation_dropped_total{reason=\"budget_exhausted\"} 0");
    ASSERT_LINE(text, "lightnvr_observation_dropped_total{reason=\"disabled\"} 0");
    ASSERT_LINE(text, "lightnvr_observation_dropped_total{reason=\"untracked\"} 0");
    ASSERT_LINE(text, "lightnvr_observation_evidence_queue_allocated 0");
    ASSERT_LINE(text, "lightnvr_observation_evidence_worker_running 0");
    ASSERT_LINE(text, "lightnvr_observation_evidence_backlog_items 0");
    ASSERT_LINE(text, "lightnvr_observation_evidence_backlog_bytes 0");
    ASSERT_LINE(text, "lightnvr_observation_publications{state=\"pending\"} 0");
    ASSERT_LINE(text, "lightnvr_observation_relay_lag_seconds 0.000");
    ASSERT_LINE(text, "lightnvr_observation_evidence_budget_exhausted_cameras 0");
    /* No retained observation: the timestamp gauge has no sample. */
    TEST_ASSERT_NULL(strstr(text, "\nlightnvr_observation_history_oldest_observed_timestamp_seconds "));
    free(text);

    cJSON *json = observation_diagnostics_to_json(&diag);
    TEST_ASSERT_NOT_NULL(json);
    TEST_ASSERT_FALSE(json_bool(json, "active"));
    TEST_ASSERT_FALSE(json_bool(json, "evidence.queue_allocated"));
    ASSERT_NUM(0, json_number(json, "accepted"));
    ASSERT_NUM(0, json_number(json, "dropped.queue_full"));
    ASSERT_NUM(0, json_number(json, "publications.pending"));
    TEST_ASSERT_TRUE(cJSON_IsNull(cJSON_GetObjectItemCaseSensitive(
        cJSON_GetObjectItemCaseSensitive(json, "history"), "oldest_observed_at")));
    TEST_ASSERT_EQUAL_INT(0, cJSON_GetArraySize(cJSON_GetObjectItemCaseSensitive(json, "cameras")));
    cJSON_Delete(json);
    observation_diagnostics_free(&diag);
    /* Collecting never started the worker. */
    observation_evidence_stats_t after;
    observation_evidence_get_stats(&after);
    TEST_ASSERT_FALSE(after.allocated);
}

void test_db_counts_publications_and_history_bounds(void) {
    insert_pending(2);
    insert_pending(1);
    /* One failed, one expired publication; the third stays pending. */
    exec_sql("UPDATE observation_publications SET state = 'failed' WHERE change_seq = "
             "(SELECT MIN(change_seq) FROM observation_publications);");
    exec_sql("UPDATE observation_publications SET state = 'expired' WHERE change_seq = "
             "(SELECT MAX(change_seq) FROM observation_publications);");
    exec_sql("UPDATE observation_changes SET created_at_ms = 1786000005000 WHERE seq = "
             "(SELECT change_seq FROM observation_publications WHERE state = 'pending');");

    observation_diagnostics_t diag;
    TEST_ASSERT_EQUAL_INT(0, observation_diagnostics_collect(&diag, 1786000065000LL));
    TEST_ASSERT_TRUE(diag.db_available);
    TEST_ASSERT_EQUAL_INT64(3, diag.db.observations);
    TEST_ASSERT_EQUAL_INT64(1, diag.db.publications_pending);
    TEST_ASSERT_EQUAL_INT64(1, diag.db.publications_failed);
    TEST_ASSERT_EQUAL_INT64(1, diag.db.publications_expired);
    TEST_ASSERT_EQUAL_INT64(1786000005000LL, diag.db.oldest_pending_created_at_ms);
    ASSERT_NUM(60.0, observation_diagnostics_relay_lag_seconds(&diag));
    TEST_ASSERT_TRUE(diag.db.has_oldest_observed_at);
    TEST_ASSERT_TRUE(diag.db.oldest_observed_at_ms > 0);
    observation_meta_t meta;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_meta_get(&meta));
    TEST_ASSERT_EQUAL_INT64(meta.oldest_seq, diag.db.meta.oldest_seq);
    TEST_ASSERT_EQUAL_INT64(meta.head_seq, diag.db.meta.head_seq);

    char *text = api_metrics_render_observations(&diag);
    TEST_ASSERT_NOT_NULL(text);
    ASSERT_LINE(text, "lightnvr_observation_publications{state=\"pending\"} 1");
    ASSERT_LINE(text, "lightnvr_observation_publications{state=\"failed\"} 1");
    ASSERT_LINE(text, "lightnvr_observation_publications{state=\"expired\"} 1");
    ASSERT_LINE(text, "lightnvr_observation_relay_lag_seconds 60.000");
    char line[160];
    snprintf(line, sizeof(line), "lightnvr_observation_history_oldest_seq %lld",
             (long long)meta.oldest_seq);
    ASSERT_LINE(text, line);
    snprintf(line, sizeof(line), "lightnvr_observation_history_head_seq %lld",
             (long long)meta.head_seq);
    ASSERT_LINE(text, line);
    TEST_ASSERT_NOT_NULL(strstr(text, "\nlightnvr_observation_history_oldest_observed_timestamp_seconds "));
    free(text);

    cJSON *json = observation_diagnostics_to_json(&diag);
    ASSERT_NUM(1, json_number(json, "publications.failed"));
    ASSERT_NUM(60.0, json_number(json, "publications.relay_lag_seconds"));
    ASSERT_NUM((double)meta.oldest_seq, json_number(json, "history.oldest_seq"));
    const cJSON *cursor = cJSON_GetObjectItemCaseSensitive(
        cJSON_GetObjectItemCaseSensitive(json, "history"), "oldest_observed_at");
    TEST_ASSERT_TRUE(cJSON_IsString(cursor));
    TEST_ASSERT_EQUAL_INT(24, (int)strlen(cursor->valuestring));  /* YYYY-MM-DDTHH:MM:SS.mmmZ */
    cJSON_Delete(json);
    observation_diagnostics_free(&diag);
}

void test_serialization_of_a_populated_snapshot(void) {
    observation_diagnostics_t diag;
    memset(&diag, 0, sizeof(diag));
    diag.collected_at_ms = 1786000010000LL;
    diag.evidence.initialized = true;
    diag.evidence.allocated = true;
    diag.evidence.worker_running = true;
    diag.evidence.accepted = 41;
    diag.evidence.dropped_queue_full = 3;
    diag.evidence.dropped_budget = 2;
    diag.evidence.dropped_disabled = 1;
    diag.evidence.dropped_untracked = 4;
    diag.evidence.unavailable_submitted = 5;
    diag.evidence.ready = 30;
    diag.evidence.failed_encode = 6;
    diag.evidence.failed_write = 7;
    diag.evidence.failed_db = 8;
    diag.evidence.disabled_at_encode = 9;
    diag.evidence.interrupted = 10;
    diag.evidence.bytes_written = 123456;
    diag.evidence.backlog_items = 11;
    diag.evidence.backlog_bytes = 2048;
    diag.evidence.drop_ring_depth = 12;
    diag.evidence.high_water_items = 13;
    diag.evidence.high_water_bytes = 4096;
    diag.relay.running = true;
    diag.relay.relayed = 20;
    diag.relay.relayed_no_route = 2;
    diag.relay.epoch_regenerations = 1;
    diag.db_available = true;
    diag.db.oldest_pending_created_at_ms = 1786000000000LL;
    diag.db.publications_pending = 7;
    diag.budget_exhausted_cameras = 1;
    observation_retention_camera_status_t cameras[2];
    memset(cameras, 0, sizeof(cameras));
    safe_strcpy(cameras[0].camera_uuid, "11111111-1111-4111-8111-111111111111", OBS_UUID_SIZE, 0);
    cameras[0].retained_bytes = 5000;
    cameras[0].max_bytes = 10000;
    cameras[0].max_age_days = 7;
    safe_strcpy(cameras[1].camera_uuid, "22222222-2222-4222-8222-222222222222", OBS_UUID_SIZE, 0);
    cameras[1].retained_bytes = 9000;
    cameras[1].max_bytes = 8000;
    cameras[1].max_age_days = 3;
    cameras[1].budget_exhausted = true;
    diag.cameras = cameras;
    diag.camera_count = 2;

    char *text = api_metrics_render_observations(&diag);
    TEST_ASSERT_NOT_NULL(text);
    ASSERT_LINE(text, "# TYPE lightnvr_observation_accepted_total counter");
    ASSERT_LINE(text, "lightnvr_observation_accepted_total 41");
    ASSERT_LINE(text, "lightnvr_observation_dropped_total{reason=\"queue_full\"} 3");
    ASSERT_LINE(text, "lightnvr_observation_dropped_total{reason=\"budget_exhausted\"} 2");
    ASSERT_LINE(text, "lightnvr_observation_dropped_total{reason=\"disabled\"} 1");
    ASSERT_LINE(text, "lightnvr_observation_dropped_total{reason=\"untracked\"} 4");
    ASSERT_LINE(text, "lightnvr_observation_evidence_unavailable_total 5");
    ASSERT_LINE(text, "lightnvr_observation_evidence_ready_total 30");
    ASSERT_LINE(text, "lightnvr_observation_evidence_failures_total{reason=\"encode_failed\"} 6");
    ASSERT_LINE(text, "lightnvr_observation_evidence_failures_total{reason=\"write_failed\"} 7");
    ASSERT_LINE(text, "lightnvr_observation_evidence_failures_total{reason=\"db_error\"} 8");
    ASSERT_LINE(text, "lightnvr_observation_evidence_failures_total{reason=\"disabled\"} 9");
    ASSERT_LINE(text, "lightnvr_observation_evidence_failures_total{reason=\"interrupted\"} 10");
    ASSERT_LINE(text, "lightnvr_observation_evidence_bytes_written_total 123456");
    ASSERT_LINE(text, "lightnvr_observation_evidence_backlog_items 11");
    ASSERT_LINE(text, "lightnvr_observation_evidence_backlog_bytes 2048");
    ASSERT_LINE(text, "lightnvr_observation_evidence_drop_ring_depth 12");
    ASSERT_LINE(text, "lightnvr_observation_evidence_high_water_items 13");
    ASSERT_LINE(text, "lightnvr_observation_evidence_high_water_bytes 4096");
    ASSERT_LINE(text, "lightnvr_observation_evidence_queue_allocated 1");
    ASSERT_LINE(text, "lightnvr_observation_evidence_worker_running 1");
    ASSERT_LINE(text, "lightnvr_observation_evidence_retained_bytes{camera_uuid=\"11111111-1111-4111-8111-111111111111\"} 5000");
    ASSERT_LINE(text, "lightnvr_observation_evidence_budget_bytes{camera_uuid=\"22222222-2222-4222-8222-222222222222\"} 8000");
    ASSERT_LINE(text, "lightnvr_observation_evidence_budget_exhausted{camera_uuid=\"11111111-1111-4111-8111-111111111111\"} 0");
    ASSERT_LINE(text, "lightnvr_observation_evidence_budget_exhausted{camera_uuid=\"22222222-2222-4222-8222-222222222222\"} 1");
    ASSERT_LINE(text, "lightnvr_observation_evidence_budget_exhausted_cameras 1");
    ASSERT_LINE(text, "lightnvr_observation_publications{state=\"pending\"} 7");
    ASSERT_LINE(text, "lightnvr_observation_relay_lag_seconds 10.000");
    ASSERT_LINE(text, "lightnvr_observation_relay_running 1");
    ASSERT_LINE(text, "lightnvr_observation_relay_events_total{outcome=\"relayed\"} 20");
    ASSERT_LINE(text, "lightnvr_observation_relay_events_total{outcome=\"no_route\"} 2");
    ASSERT_LINE(text, "lightnvr_observation_history_epoch_regenerations_total 1");
    free(text);

    cJSON *json = observation_diagnostics_to_json(&diag);
    TEST_ASSERT_NOT_NULL(json);
    TEST_ASSERT_TRUE(json_bool(json, "active"));
    ASSERT_NUM(41, json_number(json, "accepted"));
    ASSERT_NUM(4, json_number(json, "dropped.untracked"));
    ASSERT_NUM(2, json_number(json, "dropped.budget_exhausted"));
    ASSERT_NUM(11, json_number(json, "evidence.backlog_items"));
    ASSERT_NUM(4096, json_number(json, "evidence.high_water_bytes"));
    ASSERT_NUM(7, json_number(json, "evidence.failures.write_failed"));
    ASSERT_NUM(7, json_number(json, "publications.pending"));
    ASSERT_NUM(10.0, json_number(json, "publications.relay_lag_seconds"));
    TEST_ASSERT_TRUE(json_bool(json, "publications.relay_running"));
    ASSERT_NUM(1, json_number(json, "budget_exhausted_cameras"));
    const cJSON *list = cJSON_GetObjectItemCaseSensitive(json, "cameras");
    TEST_ASSERT_EQUAL_INT(2, cJSON_GetArraySize(list));
    const cJSON *second = cJSON_GetArrayItem(list, 1);
    TEST_ASSERT_EQUAL_STRING("22222222-2222-4222-8222-222222222222",
                             cJSON_GetObjectItemCaseSensitive(second, "camera_uuid")->valuestring);
    ASSERT_NUM(9000, json_number(second, "retained_bytes"));
    ASSERT_NUM(8000, json_number(second, "max_bytes"));
    ASSERT_NUM(3, json_number(second, "max_age_days"));
    TEST_ASSERT_TRUE(json_bool(second, "budget_exhausted"));
    cJSON_Delete(json);
}

void test_counters_move_under_queue_saturation(void) {
    observation_evidence_config_t config;
    memset(&config, 0, sizeof(config));
    config.storage_path = storage_dir;
    config.per_camera_max_items = 2;  /* the item being encoded counts toward the depth */
    TEST_ASSERT_EQUAL_INT(0, observation_evidence_init(&config));
    observation_evidence_set_stage_hook(stall_hook, NULL);
    stall_enabled = true;

    char asset[OBS_UUID_SIZE];
    observation_batch_input_t batch = make_batch(1);
    TEST_ASSERT_EQUAL_INT(OBS_EVIDENCE_ACCEPTED,
                          observation_evidence_submit(&batch, make_rgb(32, 24), 32, 24, NULL, asset));
    wait_stalled();  /* the worker holds item 1 */
    batch = make_batch(1);
    TEST_ASSERT_EQUAL_INT(OBS_EVIDENCE_ACCEPTED,
                          observation_evidence_submit(&batch, make_rgb(32, 24), 32, 24, NULL, asset));
    for (int i = 0; i < 3; ++i) {
        batch = make_batch(1);
        TEST_ASSERT_EQUAL_INT(OBS_EVIDENCE_DROPPED_QUEUE_FULL,
                              observation_evidence_submit(&batch, make_rgb(32, 24), 32, 24, NULL,
                                                          asset));
    }

    observation_diagnostics_t diag;
    TEST_ASSERT_EQUAL_INT(0, observation_diagnostics_collect(&diag, 0));
    char *text = api_metrics_render_observations(&diag);
    TEST_ASSERT_NOT_NULL(text);
    ASSERT_LINE(text, "lightnvr_observation_accepted_total 2");
    ASSERT_LINE(text, "lightnvr_observation_dropped_total{reason=\"queue_full\"} 3");
    ASSERT_LINE(text, "lightnvr_observation_evidence_queue_allocated 1");
    ASSERT_LINE(text, "lightnvr_observation_evidence_worker_running 1");
    TEST_ASSERT_TRUE(diag.evidence.backlog_items >= 1);
    TEST_ASSERT_TRUE(diag.evidence.high_water_items >= diag.evidence.backlog_items);
    char line[128];
    snprintf(line, sizeof(line), "lightnvr_observation_evidence_backlog_items %u",
             (unsigned)diag.evidence.backlog_items);
    ASSERT_LINE(text, line);
    free(text);
    observation_diagnostics_free(&diag);
    release_worker();
}

void test_snapshot_retry_after_rule(void) {
    /* Under 5 s old: always 1. */
    TEST_ASSERT_EQUAL_INT(1, observation_snapshot_retry_after_seconds(0, 0));
    TEST_ASSERT_EQUAL_INT(1, observation_snapshot_retry_after_seconds(4999, 100));
    /* Then min(30, 2 x ahead), at least 1. */
    TEST_ASSERT_EQUAL_INT(1, observation_snapshot_retry_after_seconds(5000, 0));
    TEST_ASSERT_EQUAL_INT(2, observation_snapshot_retry_after_seconds(5000, 1));
    TEST_ASSERT_EQUAL_INT(6, observation_snapshot_retry_after_seconds(60000, 3));
    TEST_ASSERT_EQUAL_INT(30, observation_snapshot_retry_after_seconds(60000, 15));
    TEST_ASSERT_EQUAL_INT(30, observation_snapshot_retry_after_seconds(60000, 16));
    TEST_ASSERT_EQUAL_INT(30, observation_snapshot_retry_after_seconds(60000, UINT64_MAX));
    /* Clock skew (asset in the future) counts as fresh. */
    TEST_ASSERT_EQUAL_INT(1, observation_snapshot_retry_after_seconds(-1000, 50));
}

int main(void) {
    snprintf(db_path, sizeof(db_path), "/tmp/lightnvr_observation_diag_%ld.db", (long)getpid());
    snprintf(storage_dir, sizeof(storage_dir), "/tmp/lightnvr_observation_diag_%ld",
             (long)getpid());
    unlink(db_path);
    mkdir(storage_dir, 0700);
    if (init_database(db_path) != 0) return 1;
    if (event_identity_init() != 0) return 1;

    UNITY_BEGIN();
    make_camera("diag_cam", cam_a);
    RUN_TEST(test_disabled_pipeline_reports_zeros_and_no_queue);
    RUN_TEST(test_db_counts_publications_and_history_bounds);
    RUN_TEST(test_serialization_of_a_populated_snapshot);
    RUN_TEST(test_counters_move_under_queue_saturation);
    RUN_TEST(test_snapshot_retry_after_rule);
    int result = UNITY_END();

    observation_evidence_shutdown(0);
    event_identity_shutdown();
    shutdown_database();
    unlink(db_path);
    char command[512];
    snprintf(command, sizeof(command), "rm -rf '%s'", storage_dir);
    (void)system(command);
    return result;
}
