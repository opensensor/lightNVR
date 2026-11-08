/* INT_01 L5: observation publication relay. Crash-safe drain, outbox
 * duplicate and no-route handling, D8 outbox cap, route suppression, delivery
 * expiry, D9 cursor-head sidecar, and envelope equality with the C3 fixtures,
 * against a real migrated database and the real router/outbox path. */
#define _GNU_SOURCE
#include <dirent.h>
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
#include "core/config.h"
#include "core/event_envelope.h"
#include "core/event_identity.h"
#include "core/event_producers.h"
#include "core/event_router.h"
#include "core/mqtt_delivery_worker.h"
#include "core/observation_relay.h"
#include "core/observation_retention.h"
#include "database/db_core.h"
#include "database/db_event_outbox.h"
#include "database/db_event_routes.h"
#include "database/db_observations.h"
#include "database/db_streams.h"
#include "utils/strings.h"
#include "utils/uuid.h"

#ifndef LIGHTNVR_SOURCE_DIR
#error "LIGHTNVR_SOURCE_DIR must be defined"
#endif
#define FIXTURE_DIR LIGHTNVR_SOURCE_DIR "/tests/fixtures/integrations/observations/events"
#define TOPIC_PREFIX "lightnvr-test"

static char db_path[256];
static char storage_dir[256];
static char cam_a[OBS_UUID_SIZE];
static char source[128];

/* ---- helpers ---------------------------------------------------------- */

static int64_t scalar64(const char *sql) {
    sqlite3_stmt *stmt = NULL;
    int64_t value = -999;
    pthread_mutex_lock(get_db_mutex());
    if (sqlite3_prepare_v2(get_db_handle(), sql, -1, &stmt, NULL) == SQLITE_OK &&
        sqlite3_step(stmt) == SQLITE_ROW)
        value = sqlite3_column_int64(stmt, 0);
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(get_db_mutex());
    TEST_ASSERT_NOT_EQUAL_INT64_MESSAGE(-999, value, sql);
    return value;
}

static int scalar(const char *sql) { return (int)scalar64(sql); }

static void exec_sql(const char *sql) {
    char *error = NULL;
    pthread_mutex_lock(get_db_mutex());
    int rc = sqlite3_exec(get_db_handle(), sql, NULL, NULL, &error);
    pthread_mutex_unlock(get_db_mutex());
    if (rc != SQLITE_OK) fprintf(stderr, "%s: %s\n", sql, error ? error : "");
    sqlite3_free(error);
    TEST_ASSERT_EQUAL_INT_MESSAGE(SQLITE_OK, rc, sql);
}

static int publications_in(const char *state) {
    char sql[160];
    snprintf(sql, sizeof(sql),
             "SELECT count(*) FROM observation_publications WHERE state = '%s'", state);
    return scalar(sql);
}

static int outbox_rows(void) {
    return scalar("SELECT count(*) FROM event_outbox WHERE event_type = '"
                  OBSERVATION_CHANGED_EVENT_TYPE "'");
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

static int64_t now_ms(void) {
    struct timespec ts;
    clock_gettime(CLOCK_REALTIME, &ts);
    return (int64_t)ts.tv_sec * 1000 + ts.tv_nsec / 1000000;
}

/* Insert one batch of `objects` pending observations sharing one asset. */
static observation_insert_result_t insert_pending(int objects) {
    observation_batch_input_t batch;
    memset(&batch, 0, sizeof(batch));
    TEST_ASSERT_EQUAL_INT(0, lightnvr_uuid_generate_v4(batch.batch_id));
    safe_strcpy(batch.camera_uuid, cam_a, sizeof(batch.camera_uuid), 0);
    batch.observed_at_ms = now_ms();
    safe_strcpy(batch.engine_key, "legacy-primary", sizeof(batch.engine_key), 0);
    safe_strcpy(batch.engine_type, "object", sizeof(batch.engine_type), 0);
    batch.object_count = objects;
    for (int i = 0; i < objects; ++i) {
        batch.objects[i].object_index = i;
        safe_strcpy(batch.objects[i].label, "bird", sizeof(batch.objects[i].label), 0);
        batch.objects[i].confidence = 0.9;
    }
    observation_asset_input_t asset;
    memset(&asset, 0, sizeof(asset));
    asset.state = OBS_ASSET_PENDING;
    safe_strcpy(asset.origin, "inference_input", sizeof(asset.origin), 0);
    safe_strcpy(asset.media_type, "image/jpeg", sizeof(asset.media_type), 0);
    observation_insert_result_t result;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_insert_batch(&batch, &asset, &result));
    return result;
}

static observation_relay_options_t test_options(void) {
    observation_relay_options_t options;
    memset(&options, 0, sizeof(options));
    options.storage_path = storage_dir;
    return options;
}

static observation_relay_pass_t run_pass(const observation_relay_options_t *options) {
    observation_relay_pass_t pass;
    memset(&pass, 0, sizeof(pass));
    observation_relay_run_once(options, &pass);
    return pass;
}

static event_route_t route_for(const char *name, const char *type) {
    event_route_t route;
    memset(&route, 0, sizeof(route));
    safe_strcpy(route.name, name, sizeof(route.name), 0);
    route.enabled = true;
    safe_strcpy(route.destination_key, EVENT_ROUTE_DEFAULT_DESTINATION,
                sizeof(route.destination_key), 0);
    safe_strcpy(route.scope_type, "all", sizeof(route.scope_type), 0);
    safe_strcpy(route.predicate_json, "{\"version\":1}", sizeof(route.predicate_json), 0);
    safe_strcpy(route.schedule_json, "{\"version\":1,\"timezone\":\"UTC\",\"windows\":[]}",
                sizeof(route.schedule_json), 0);
    safe_strcpy(route.event_types[0], type, sizeof(route.event_types[0]), 0);
    route.event_type_count = 1;
    return route;
}

static char *read_file(const char *path) {
    FILE *file = fopen(path, "rb");
    if (!file) return NULL;
    fseek(file, 0, SEEK_END);
    long size = ftell(file);
    fseek(file, 0, SEEK_SET);
    char *buffer = calloc(1, (size_t)size + 1);
    if (buffer && fread(buffer, 1, (size_t)size, file) != (size_t)size) {
        free(buffer);
        buffer = NULL;
    }
    fclose(file);
    return buffer;
}

static int64_t parse_utc_ms(const char *text) {
    struct tm tm;
    memset(&tm, 0, sizeof(tm));
    int ms = 0;
    int n = sscanf(text, "%4d-%2d-%2dT%2d:%2d:%2d.%3dZ", &tm.tm_year, &tm.tm_mon,
                   &tm.tm_mday, &tm.tm_hour, &tm.tm_min, &tm.tm_sec, &ms);
    TEST_ASSERT_TRUE_MESSAGE(n >= 6, text);
    tm.tm_year -= 1900;
    tm.tm_mon -= 1;
    return (int64_t)timegm(&tm) * 1000 + (n == 7 ? ms : 0);
}

static void sidecar_path(char *out, size_t size) {
    snprintf(out, size, "%s/%s", storage_dir, OBSERVATION_CURSOR_HEAD_FILE);
}

/* ---- fixtures --------------------------------------------------------- */

void setUp(void) {
    observation_relay_stop();
    observation_relay_reset_stats();
    g_config.mqtt_enabled = true;
    safe_strcpy(g_config.mqtt_topic_prefix, TOPIC_PREFIX, sizeof(g_config.mqtt_topic_prefix), 0);
    exec_sql("DELETE FROM object_observations; DELETE FROM observation_assets; "
             "DELETE FROM event_outbox; DELETE FROM event_routes; "
             "DELETE FROM event_route_suppression_state;");
    exec_sql("DROP TRIGGER IF EXISTS temp.fail_outbox;");
    event_router_shutdown();
}

void tearDown(void) {
    observation_relay_stop();
    exec_sql("DROP TRIGGER IF EXISTS temp.fail_outbox;");
    event_router_shutdown();
}

/* ---- tests ------------------------------------------------------------ */

void test_crash_before_relay_is_drained_at_start(void) {
    /* Publications staged by a process that "crashed" before relaying. */
    observation_insert_result_t inserted = insert_pending(2);
    TEST_ASSERT_EQUAL_INT(2, publications_in("pending"));
    TEST_ASSERT_EQUAL_INT(0, outbox_rows());

    /* A fresh start drains synchronously before steady state. */
    observation_relay_options_t options = test_options();
    options.poll_interval_ms = 60000;
    TEST_ASSERT_EQUAL_INT(0, observation_relay_start(&options));
    TEST_ASSERT_EQUAL_INT(0, publications_in("pending"));
    TEST_ASSERT_EQUAL_INT(2, publications_in("relayed"));
    TEST_ASSERT_EQUAL_INT(2, outbox_rows());

    /* The outbox row carries the deterministic message ID and topic. */
    char expected_id[OBS_UUID_SIZE];
    TEST_ASSERT_EQUAL_INT(0, observation_message_id(source, inserted.observation_uuids[0], 1,
                                                    expected_id));
    char sql[512];
    snprintf(sql, sizeof(sql),
             "SELECT count(*) FROM event_outbox WHERE event_id = '%s' AND destination = '%s' "
             "AND topic = '" TOPIC_PREFIX "/v1/events/" OBSERVATION_CHANGED_EVENT_TYPE "/%s'",
             expected_id, MQTT_EVENT_OUTBOX_DESTINATION, cam_a);
    TEST_ASSERT_EQUAL_INT(1, scalar(sql));

    observation_relay_stats_t stats;
    observation_relay_get_stats(&stats);
    TEST_ASSERT_TRUE(stats.running);
    TEST_ASSERT_EQUAL_UINT64(2, stats.relayed);
    observation_relay_stop();
    observation_relay_get_stats(&stats);
    TEST_ASSERT_FALSE(stats.running);

    /* Restart is idempotent and finds nothing left. */
    TEST_ASSERT_EQUAL_INT(0, observation_relay_start(&options));
    TEST_ASSERT_EQUAL_INT(2, outbox_rows());
    observation_relay_stop();
}

void test_duplicate_relay_is_marked_relayed(void) {
    observation_insert_result_t inserted = insert_pending(1);
    observation_publication_t pending[4];
    TEST_ASSERT_EQUAL_INT(1, db_observation_pending_publications(pending, 4));

    /* Crash after the outbox accepted the row but before the mark. */
    event_envelope_t event;
    char error[256] = "";
    TEST_ASSERT_EQUAL_INT_MESSAGE(0, observation_relay_build_envelope(&pending[0], source, &event,
                                                                      error, sizeof(error)), error);
    TEST_ASSERT_EQUAL_INT(EVENT_PRODUCER_ENQUEUE_ENQUEUED,
                          event_producer_enqueue_durable(&event, error, sizeof(error)));
    TEST_ASSERT_EQUAL_INT(EVENT_PRODUCER_ENQUEUE_DUPLICATE,
                          event_producer_enqueue_durable(&event, error, sizeof(error)));
    event_envelope_clear(&event);
    TEST_ASSERT_EQUAL_INT(1, outbox_rows());

    observation_relay_options_t options = test_options();
    observation_relay_pass_t pass = run_pass(&options);
    TEST_ASSERT_EQUAL_INT(1, pass.processed);
    TEST_ASSERT_FALSE(pass.backoff);
    TEST_ASSERT_EQUAL_INT(1, publications_in("relayed"));
    TEST_ASSERT_EQUAL_INT(1, outbox_rows());

    observation_relay_stats_t stats;
    observation_relay_get_stats(&stats);
    TEST_ASSERT_EQUAL_UINT64(1, stats.relayed);
    TEST_ASSERT_EQUAL_UINT64(1, stats.duplicates);
    TEST_ASSERT_EQUAL_INT64(pending[0].change_seq, stats.last_relayed_seq);
    (void)inserted;

    /* A second pass has nothing to do; the mark is not repeated. */
    pass = run_pass(&options);
    TEST_ASSERT_EQUAL_INT(0, pass.read);
}

void test_mqtt_disabled_and_no_route_are_relayed_no_route(void) {
    g_config.mqtt_enabled = false;
    insert_pending(1);
    observation_relay_options_t options = test_options();
    observation_relay_pass_t pass = run_pass(&options);
    TEST_ASSERT_EQUAL_INT(1, pass.processed);
    TEST_ASSERT_EQUAL_INT(1, publications_in("relayed_no_route"));
    TEST_ASSERT_EQUAL_INT(0, outbox_rows());

    /* Routes exist, but none selects this type. */
    g_config.mqtt_enabled = true;
    event_route_t other = route_for("Offline only", "io.lightnvr.camera.offline.v1");
    TEST_ASSERT_EQUAL_INT(DB_EVENT_ROUTE_OK, db_event_route_create(&other));
    observation_publication_t pending[2];
    insert_pending(1);
    TEST_ASSERT_EQUAL_INT(1, db_observation_pending_publications(pending, 2));
    event_envelope_t event;
    char error[256] = "";
    TEST_ASSERT_EQUAL_INT(0, observation_relay_build_envelope(&pending[0], source, &event,
                                                              error, sizeof(error)));
    TEST_ASSERT_EQUAL_INT(EVENT_PRODUCER_ENQUEUE_NO_ROUTE,
                          event_producer_enqueue_durable(&event, error, sizeof(error)));
    event_envelope_clear(&event);
    pass = run_pass(&options);
    TEST_ASSERT_EQUAL_INT(1, pass.processed);
    TEST_ASSERT_EQUAL_INT(2, publications_in("relayed_no_route"));
    TEST_ASSERT_EQUAL_INT(0, outbox_rows());

    observation_relay_stats_t stats;
    observation_relay_get_stats(&stats);
    TEST_ASSERT_EQUAL_UINT64(2, stats.relayed_no_route);
    TEST_ASSERT_EQUAL_UINT64(0, stats.relayed);
}

void test_route_suppression_applies_and_keeps_history(void) {
    event_route_t route = route_for("Observation cooldown", OBSERVATION_CHANGED_EVENT_TYPE);
    route.cooldown_seconds = 600;
    TEST_ASSERT_EQUAL_INT(DB_EVENT_ROUTE_OK, db_event_route_create(&route));
    insert_pending(1);
    insert_pending(1);

    observation_relay_options_t options = test_options();
    observation_relay_pass_t pass = run_pass(&options);
    TEST_ASSERT_EQUAL_INT(2, pass.processed);
    TEST_ASSERT_EQUAL_INT(1, publications_in("relayed"));
    TEST_ASSERT_EQUAL_INT(1, publications_in("relayed_no_route"));
    TEST_ASSERT_EQUAL_INT(1, outbox_rows());

    /* Suppression is not deletion: history and the cursor API keep both. */
    TEST_ASSERT_EQUAL_INT(2, scalar("SELECT count(*) FROM object_observations"));
    TEST_ASSERT_EQUAL_INT(2, scalar("SELECT count(*) FROM observation_changes"));
    observation_list_query_t query;
    memset(&query, 0, sizeof(query));
    query.all_cameras = true;
    observation_page_t page;
    memset(&page, 0, sizeof(page));
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observations_list(&query, NULL, &page));
    TEST_ASSERT_EQUAL_UINT(2, page.count);
    db_observation_page_free(&page);
}

void test_outbox_cap_pauses_and_resumes_in_seq_order(void) {
    TEST_ASSERT_EQUAL_INT(2000, OBSERVATION_RELAY_OUTBOX_CAP);
    for (int i = 0; i < 5; ++i) insert_pending(1);

    observation_relay_options_t options = test_options();
    options.outbox_cap = 3;
    observation_relay_pass_t pass = run_pass(&options);
    TEST_ASSERT_EQUAL_INT(3, pass.processed);
    TEST_ASSERT_TRUE(pass.cap_reached);
    TEST_ASSERT_TRUE(pass.backoff);
    TEST_ASSERT_EQUAL_INT(3, outbox_rows());
    TEST_ASSERT_EQUAL_INT(2, publications_in("pending"));
    /* The relayed rows are the oldest ones. */
    TEST_ASSERT_EQUAL_INT(0, scalar(
        "SELECT count(*) FROM observation_publications p WHERE p.state = 'relayed' AND "
        "p.change_seq > (SELECT min(change_seq) FROM observation_publications "
        "WHERE state = 'pending')"));
    /* Paused rows are not charged an attempt. */
    TEST_ASSERT_EQUAL_INT(0, scalar(
        "SELECT COALESCE(sum(attempts),0) FROM observation_publications WHERE state = 'pending'"));

    /* At the cap, a pass enqueues nothing. */
    pass = run_pass(&options);
    TEST_ASSERT_EQUAL_INT(0, pass.processed);
    TEST_ASSERT_TRUE(pass.cap_reached);
    observation_relay_stats_t stats;
    observation_relay_get_stats(&stats);
    TEST_ASSERT_GREATER_OR_EQUAL_UINT64(2, stats.cap_pauses);
    TEST_ASSERT_EQUAL_INT64(3, stats.outbox_type_pending);

    /* Delivered rows no longer count toward the cap. */
    exec_sql("UPDATE event_outbox SET state = 'delivered'");
    pass = run_pass(&options);
    TEST_ASSERT_EQUAL_INT(2, pass.processed);
    TEST_ASSERT_FALSE(pass.cap_reached);
    TEST_ASSERT_EQUAL_INT(5, publications_in("relayed"));
}

void test_outbox_error_backs_off_and_keeps_row_pending(void) {
    insert_pending(1);
    insert_pending(1);
    exec_sql("CREATE TEMP TRIGGER fail_outbox BEFORE INSERT ON main.event_outbox "
             "BEGIN SELECT RAISE(ABORT, 'injected outbox failure'); END;");

    observation_relay_options_t options = test_options();
    observation_relay_pass_t pass = run_pass(&options);
    TEST_ASSERT_EQUAL_INT(0, pass.processed);
    TEST_ASSERT_TRUE(pass.backoff);
    TEST_ASSERT_EQUAL_INT(2, publications_in("pending"));
    /* Only the head row was attempted, so seq order is preserved. */
    TEST_ASSERT_EQUAL_INT(1, scalar(
        "SELECT sum(attempts) FROM observation_publications"));
    TEST_ASSERT_EQUAL_INT(1, scalar(
        "SELECT count(*) FROM observation_publications WHERE last_error IS NOT NULL"));
    observation_relay_stats_t stats;
    observation_relay_get_stats(&stats);
    TEST_ASSERT_EQUAL_UINT64(1, stats.enqueue_errors);
    TEST_ASSERT_GREATER_THAN_INT64(0, stats.oldest_pending_created_at_ms);

    exec_sql("DROP TRIGGER temp.fail_outbox;");
    pass = run_pass(&options);
    TEST_ASSERT_EQUAL_INT(2, pass.processed);
    TEST_ASSERT_EQUAL_INT(2, publications_in("relayed"));
    TEST_ASSERT_EQUAL_INT(2, outbox_rows());
}

void test_publication_past_delivery_expiry_is_expired(void) {
    insert_pending(1);
    char sql[160];
    snprintf(sql, sizeof(sql), "UPDATE observation_changes SET created_at_ms = %lld",
             (long long)(now_ms() - 2LL * 3600 * 1000));
    exec_sql(sql);
    observation_relay_options_t options = test_options();
    observation_relay_pass_t pass = run_pass(&options);
    TEST_ASSERT_EQUAL_INT(1, pass.processed);
    TEST_ASSERT_EQUAL_INT(1, publications_in("expired"));
    TEST_ASSERT_EQUAL_INT(0, outbox_rows());
    observation_relay_stats_t stats;
    observation_relay_get_stats(&stats);
    TEST_ASSERT_EQUAL_UINT64(1, stats.expired);
}

void test_commit_notify_wakes_the_relay_thread(void) {
    observation_relay_options_t options = test_options();
    options.poll_interval_ms = 30000;  /* only a notify can wake it in time */
    TEST_ASSERT_EQUAL_INT(0, observation_relay_start(&options));
    insert_pending(1);  /* the DB layer's post-commit hook notifies */
    int64_t deadline = now_ms() + 3000;
    while (publications_in("relayed") == 0 && now_ms() < deadline) usleep(10000);
    TEST_ASSERT_EQUAL_INT(1, publications_in("relayed"));
    observation_relay_stop();
}

void test_cursor_head_sidecar_and_rewind_detection(void) {
    insert_pending(1);
    observation_relay_options_t options = test_options();
    run_pass(&options);

    char path[512];
    sidecar_path(path, sizeof(path));
    char *text = read_file(path);
    TEST_ASSERT_NOT_NULL_MESSAGE(text, path);
    cJSON *json = cJSON_Parse(text);
    free(text);
    observation_meta_t meta;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_meta_get(&meta));
    TEST_ASSERT_TRUE(cJSON_IsObject(json));
    TEST_ASSERT_EQUAL_STRING(meta.epoch_uuid,
                             cJSON_GetObjectItemCaseSensitive(json, "epoch")->valuestring);
    TEST_ASSERT_EQUAL_INT64(meta.head_seq, (int64_t)cJSON_GetObjectItemCaseSensitive(
                                               json, "head_seq")->valuedouble);
    cJSON_Delete(json);

    /* Matching sidecar: nothing to do. */
    TEST_ASSERT_EQUAL_INT(0, observation_relay_check_cursor_head(storage_dir));

    /* Missing sidecar is recreated without regenerating. */
    unlink(path);
    TEST_ASSERT_EQUAL_INT(0, observation_relay_check_cursor_head(storage_dir));
    TEST_ASSERT_EQUAL_INT(0, access(path, F_OK));
    observation_meta_t after;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_meta_get(&after));
    TEST_ASSERT_EQUAL_STRING(meta.epoch_uuid, after.epoch_uuid);

    /* Sidecar ahead of the DB with the same epoch => the DB was restored. */
    FILE *file = fopen(path, "w");
    TEST_ASSERT_NOT_NULL(file);
    fprintf(file, "{\"epoch\":\"%s\",\"head_seq\":%lld}", meta.epoch_uuid,
            (long long)meta.head_seq + 100);
    fclose(file);
    TEST_ASSERT_EQUAL_INT(1, observation_relay_check_cursor_head(storage_dir));
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_meta_get(&after));
    TEST_ASSERT_NOT_EQUAL(0, strcmp(meta.epoch_uuid, after.epoch_uuid));
    observation_relay_stats_t stats;
    observation_relay_get_stats(&stats);
    TEST_ASSERT_EQUAL_UINT64(1, stats.epoch_regenerations);

    /* The rewritten sidecar carries the new epoch; a re-check is a no-op. */
    TEST_ASSERT_EQUAL_INT(0, observation_relay_check_cursor_head(storage_dir));

    /* A different epoch (another DB generation) is not a rewind. */
    file = fopen(path, "w");
    TEST_ASSERT_NOT_NULL(file);
    fprintf(file, "{\"epoch\":\"%s\",\"head_seq\":%lld}",
            "99999999-9999-4999-8999-999999999999", (long long)after.head_seq + 100);
    fclose(file);
    TEST_ASSERT_EQUAL_INT(0, observation_relay_check_cursor_head(storage_dir));
}

void test_relay_sidecar_is_the_shared_l4_cursor_head(void) {
    /* L10 consolidation: the relay writes the D9 sidecar through L4's
     * observation_cursor_head_write(), so both modules share one path, one
     * format and one writer (which creates missing directories). */
    insert_pending(1);
    char nested[512];
    snprintf(nested, sizeof(nested), "%s/fresh-volume/storage", storage_dir);
    TEST_ASSERT_EQUAL_INT(0, observation_relay_write_cursor_head(nested));
    observation_meta_t meta;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_meta_get(&meta));
    char epoch[OBS_UUID_SIZE];
    int64_t head = -1;
    TEST_ASSERT_EQUAL_INT(0, observation_cursor_head_read(nested, epoch, &head));
    TEST_ASSERT_EQUAL_STRING(meta.epoch_uuid, epoch);
    TEST_ASSERT_EQUAL_INT64(meta.head_seq, head);

    /* The relay's startup check is L4's reconcile: a rewind written by L4's
     * writer is detected and the sidecar rewritten with the new epoch. */
    TEST_ASSERT_EQUAL_INT(0, observation_cursor_head_write(nested, meta.epoch_uuid,
                                                           meta.head_seq + 5));
    TEST_ASSERT_EQUAL_INT(1, observation_relay_check_cursor_head(nested));
    observation_meta_t after;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_meta_get(&after));
    TEST_ASSERT_NOT_EQUAL(0, strcmp(meta.epoch_uuid, after.epoch_uuid));
    TEST_ASSERT_EQUAL_INT(0, observation_cursor_head_read(nested, epoch, &head));
    TEST_ASSERT_EQUAL_STRING(after.epoch_uuid, epoch);
    TEST_ASSERT_EQUAL_INT(0, observation_relay_check_cursor_head(nested));
}

void test_envelopes_match_c3_fixtures(void) {
    DIR *dir = opendir(FIXTURE_DIR);
    TEST_ASSERT_NOT_NULL_MESSAGE(dir, FIXTURE_DIR);
    int checked = 0;
    struct dirent *entry;
    while ((entry = readdir(dir)) != NULL) {
        size_t length = strlen(entry->d_name);
        if (length < 6 || strcmp(entry->d_name + length - 5, ".json") != 0) continue;
        char path[1024];
        snprintf(path, sizeof(path), "%s/%s", FIXTURE_DIR, entry->d_name);
        char *text = read_file(path);
        TEST_ASSERT_NOT_NULL_MESSAGE(text, path);
        cJSON *fixture = cJSON_Parse(text);
        free(text);
        TEST_ASSERT_NOT_NULL_MESSAGE(fixture, path);
        const cJSON *data = cJSON_GetObjectItemCaseSensitive(fixture, "data");

        observation_publication_t pub;
        memset(&pub, 0, sizeof(pub));
        pub.change_seq = 1;
        safe_strcpy(pub.message_id, cJSON_GetObjectItemCaseSensitive(fixture, "id")->valuestring,
                    sizeof(pub.message_id), 0);
        safe_strcpy(pub.observation_uuid,
                    cJSON_GetObjectItemCaseSensitive(data, "observation_id")->valuestring,
                    sizeof(pub.observation_uuid), 0);
        pub.revision = cJSON_GetObjectItemCaseSensitive(data, "revision")->valueint;
        const char *cause = cJSON_GetObjectItemCaseSensitive(data, "change_cause")->valuestring;
        pub.cause = strcmp(cause, "created") == 0 ? OBS_CAUSE_CREATED
                    : strcmp(cause, "evidence") == 0 ? OBS_CAUSE_EVIDENCE
                                                     : OBS_CAUSE_CLASSIFICATION;
        TEST_ASSERT_EQUAL_INT(0, observation_asset_state_parse(
            cJSON_GetObjectItemCaseSensitive(data, "evidence_state")->valuestring,
            &pub.evidence_state));
        const cJSON *reason = cJSON_GetObjectItemCaseSensitive(data, "evidence_reason");
        if (cJSON_IsString(reason))
            safe_strcpy(pub.evidence_reason, reason->valuestring, sizeof(pub.evidence_reason), 0);
        safe_strcpy(pub.camera_uuid,
                    cJSON_GetObjectItemCaseSensitive(data, "camera_uuid")->valuestring,
                    sizeof(pub.camera_uuid), 0);
        safe_strcpy(pub.label, cJSON_GetObjectItemCaseSensitive(data, "label")->valuestring,
                    sizeof(pub.label), 0);
        pub.observed_at_ms =
            parse_utc_ms(cJSON_GetObjectItemCaseSensitive(data, "observed_at")->valuestring);
        safe_strcpy(pub.asset_uuid, cJSON_GetObjectItemCaseSensitive(data, "asset_id")->valuestring,
                    sizeof(pub.asset_uuid), 0);
        /* Any sub-second offset must truncate to the fixture's time. */
        pub.change_created_at_ms =
            parse_utc_ms(cJSON_GetObjectItemCaseSensitive(fixture, "time")->valuestring) + 999;

        /* The stored message ID is the section 5.4 derivation. */
        const char *fixture_source = cJSON_GetObjectItemCaseSensitive(fixture, "source")->valuestring;
        char derived[OBS_UUID_SIZE];
        TEST_ASSERT_EQUAL_INT(0, observation_message_id(fixture_source, pub.observation_uuid,
                                                        pub.revision, derived));
        TEST_ASSERT_EQUAL_STRING_MESSAGE(pub.message_id, derived, entry->d_name);

        event_envelope_t event;
        char error[256] = "";
        TEST_ASSERT_EQUAL_INT_MESSAGE(0, observation_relay_build_envelope(
                                             &pub, fixture_source, &event, error, sizeof(error)),
                                      error);
        char *serialized = event_envelope_serialize(&event, error, sizeof(error));
        TEST_ASSERT_NOT_NULL_MESSAGE(serialized, error);
        char *expected = cJSON_PrintUnformatted(fixture);
        /* Byte equality: same keys, values and key order as event_envelope.c. */
        TEST_ASSERT_EQUAL_STRING_MESSAGE(expected, serialized, entry->d_name);
        free(expected);
        free(serialized);
        event_envelope_clear(&event);
        cJSON_Delete(fixture);
        checked++;
    }
    closedir(dir);
    TEST_ASSERT_EQUAL_INT(8, checked);
}

void test_build_envelope_rejects_inconsistent_publications(void) {
    observation_publication_t pub;
    memset(&pub, 0, sizeof(pub));
    pub.change_seq = 7;
    safe_strcpy(pub.message_id, "43ebdf02-b2d5-5403-a0ef-2a4e7d58ada9", sizeof(pub.message_id), 0);
    safe_strcpy(pub.observation_uuid, "55555555-5555-4555-8555-555555555555",
                sizeof(pub.observation_uuid), 0);
    pub.revision = 2;
    pub.cause = OBS_CAUSE_EVIDENCE;
    pub.evidence_state = OBS_ASSET_READY;
    safe_strcpy(pub.camera_uuid, "22222222-2222-4222-8222-222222222222", sizeof(pub.camera_uuid), 0);
    safe_strcpy(pub.label, "bird", sizeof(pub.label), 0);
    pub.observed_at_ms = 1793000000481LL;
    pub.change_created_at_ms = 1793000000700LL;
    safe_strcpy(pub.asset_uuid, "88888888-8888-4888-8888-888888888888", sizeof(pub.asset_uuid), 0);

    event_envelope_t event;
    char error[256] = "";
    TEST_ASSERT_EQUAL_INT(0, observation_relay_build_envelope(&pub, source, &event, error,
                                                              sizeof(error)));
    event_envelope_clear(&event);

    observation_publication_t bad = pub;
    safe_strcpy(bad.evidence_reason, "queue_full", sizeof(bad.evidence_reason), 0);
    TEST_ASSERT_EQUAL_INT(-1, observation_relay_build_envelope(&bad, source, &event, error,
                                                               sizeof(error)));
    bad = pub;
    bad.label[0] = '\0';
    TEST_ASSERT_EQUAL_INT(-1, observation_relay_build_envelope(&bad, source, &event, error,
                                                               sizeof(error)));
    bad = pub;
    bad.revision = 0;
    TEST_ASSERT_EQUAL_INT(-1, observation_relay_build_envelope(&bad, source, &event, error,
                                                               sizeof(error)));
    TEST_ASSERT_EQUAL_INT(-1, observation_relay_build_envelope(NULL, source, &event, error,
                                                               sizeof(error)));
}

void test_invalid_staged_row_is_marked_failed_without_blocking(void) {
    insert_pending(1);
    insert_pending(1);
    /* Corrupt the head row so its envelope cannot validate. */
    exec_sql("UPDATE observation_publications SET message_id = 'not-a-uuid' "
             "WHERE change_seq = (SELECT min(change_seq) FROM observation_publications)");
    observation_relay_options_t options = test_options();
    observation_relay_pass_t pass = run_pass(&options);
    TEST_ASSERT_EQUAL_INT(2, pass.processed);
    TEST_ASSERT_EQUAL_INT(1, publications_in("failed"));
    TEST_ASSERT_EQUAL_INT(1, publications_in("relayed"));
    observation_relay_stats_t stats;
    observation_relay_get_stats(&stats);
    TEST_ASSERT_EQUAL_UINT64(1, stats.failed);
}

int main(void) {
    snprintf(db_path, sizeof(db_path), "/tmp/lightnvr_observation_relay_%ld.db", (long)getpid());
    snprintf(storage_dir, sizeof(storage_dir), "/tmp/lightnvr_observation_relay_%ld",
             (long)getpid());
    unlink(db_path);
    mkdir(storage_dir, 0700);
    if (init_database(db_path) != 0) return 1;
    if (event_identity_init() != 0 || event_identity_get_source(source, sizeof(source)) != 0)
        return 1;

    UNITY_BEGIN();
    make_camera("relay_cam", cam_a);
    RUN_TEST(test_crash_before_relay_is_drained_at_start);
    RUN_TEST(test_duplicate_relay_is_marked_relayed);
    RUN_TEST(test_mqtt_disabled_and_no_route_are_relayed_no_route);
    RUN_TEST(test_route_suppression_applies_and_keeps_history);
    RUN_TEST(test_outbox_cap_pauses_and_resumes_in_seq_order);
    RUN_TEST(test_outbox_error_backs_off_and_keeps_row_pending);
    RUN_TEST(test_publication_past_delivery_expiry_is_expired);
    RUN_TEST(test_commit_notify_wakes_the_relay_thread);
    RUN_TEST(test_cursor_head_sidecar_and_rewind_detection);
    RUN_TEST(test_relay_sidecar_is_the_shared_l4_cursor_head);
    RUN_TEST(test_envelopes_match_c3_fixtures);
    RUN_TEST(test_build_envelope_rejects_inconsistent_publications);
    RUN_TEST(test_invalid_staged_row_is_marked_failed_without_blocking);
    int result = UNITY_END();

    observation_relay_stop();
    event_router_shutdown();
    event_identity_shutdown();
    shutdown_database();
    unlink(db_path);
    char sidecar[512];
    snprintf(sidecar, sizeof(sidecar), "%s-wal", db_path); unlink(sidecar);
    snprintf(sidecar, sizeof(sidecar), "%s-shm", db_path); unlink(sidecar);
    sidecar_path(sidecar, sizeof(sidecar)); unlink(sidecar);
    snprintf(sidecar, sizeof(sidecar), "%s/observations", storage_dir); rmdir(sidecar);
    rmdir(storage_dir);
    return result;
}
