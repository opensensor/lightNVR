/* INT_01 L2: observation DB layer. Idempotent batch writes, compare-and-set
 * evidence transitions, the section 7.3 cursor page algorithm, publication
 * staging and history pruning, against a real migrated database. */
#define _GNU_SOURCE
#include <pthread.h>
#include <sqlite3.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>

#include "unity.h"
#include "core/event_identity.h"
#include "database/db_core.h"
#include "database/db_observations.h"
#include "database/db_streams.h"
#include "utils/strings.h"
#include "utils/uuid.h"

static char db_path[256];
static char cam_a[OBS_UUID_SIZE];
static char cam_b[OBS_UUID_SIZE];
static char source[128];

/* ---- helpers ---------------------------------------------------------- */

static int scalar(const char *sql) {
    sqlite3_stmt *stmt = NULL;
    int value = -999;
    pthread_mutex_lock(get_db_mutex());
    if (sqlite3_prepare_v2(get_db_handle(), sql, -1, &stmt, NULL) == SQLITE_OK &&
        sqlite3_step(stmt) == SQLITE_ROW)
        value = sqlite3_column_int(stmt, 0);
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(get_db_mutex());
    TEST_ASSERT_NOT_EQUAL_INT_MESSAGE(-999, value, sql);
    return value;
}

static void exec_sql(const char *sql) {
    char *error = NULL;
    pthread_mutex_lock(get_db_mutex());
    int rc = sqlite3_exec(get_db_handle(), sql, NULL, NULL, &error);
    pthread_mutex_unlock(get_db_mutex());
    if (rc != SQLITE_OK) fprintf(stderr, "%s: %s\n", sql, error ? error : "");
    sqlite3_free(error);
    TEST_ASSERT_EQUAL_INT_MESSAGE(SQLITE_OK, rc, sql);
}

static int count_where(const char *table, const char *where) {
    char sql[512];
    snprintf(sql, sizeof(sql), "SELECT count(*) FROM %s WHERE %s", table, where);
    return scalar(sql);
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

static int64_t base_time_ms = 1793000000000LL;

static observation_batch_input_t make_batch(const char *camera, int objects,
                                            const char *label) {
    observation_batch_input_t batch;
    memset(&batch, 0, sizeof(batch));
    TEST_ASSERT_EQUAL_INT(0, lightnvr_uuid_generate_v4(batch.batch_id));
    safe_strcpy(batch.camera_uuid, camera, sizeof(batch.camera_uuid), 0);
    batch.observed_at_ms = base_time_ms++;
    safe_strcpy(batch.engine_key, "legacy-primary", sizeof(batch.engine_key), 0);
    safe_strcpy(batch.engine_type, "object", sizeof(batch.engine_type), 0);
    safe_strcpy(batch.model, "ssd_mobilenet_v2.sod", sizeof(batch.model), 0);
    batch.object_count = objects;
    for (int i = 0; i < objects; ++i) {
        observation_object_input_t *object = &batch.objects[i];
        object->object_index = i;
        safe_strcpy(object->label, label, sizeof(object->label), 0);
        object->confidence = 0.8;
        object->has_box = true;
        object->box_x = 0.1; object->box_y = 0.2;
        object->box_width = 0.3; object->box_height = 0.4;
    }
    return batch;
}

static observation_asset_input_t pending_asset(void) {
    observation_asset_input_t asset;
    memset(&asset, 0, sizeof(asset));
    asset.state = OBS_ASSET_PENDING;
    safe_strcpy(asset.origin, "inference_input", sizeof(asset.origin), 0);
    safe_strcpy(asset.media_type, "image/jpeg", sizeof(asset.media_type), 0);
    safe_strcpy(asset.transform_json, "{\"mode\":\"none\"}", sizeof(asset.transform_json), 0);
    return asset;
}

static observation_insert_result_t insert_ok(const char *camera, int objects,
                                             const char *label) {
    observation_batch_input_t batch = make_batch(camera, objects, label);
    observation_asset_input_t asset = pending_asset();
    observation_insert_result_t result;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_insert_batch(&batch, &asset, &result));
    return result;
}

static const observation_asset_media_t ready_media = {
    .path = "/tmp/observations/x.jpg", .bytes = 4096, .width = 640, .height = 360,
    .expires_at_ms = 0,
};

static int list_page(const observation_list_query_t *query, const char *cursor,
                     observation_page_t *page) {
    memset(page, 0, sizeof(*page));
    return db_observations_list(query, cursor, page);
}

static observation_list_query_t all_query(int limit) {
    observation_list_query_t query;
    memset(&query, 0, sizeof(query));
    query.all_cameras = true;
    query.limit = limit;
    return query;
}

void setUp(void) {
    exec_sql("DELETE FROM object_observations; DELETE FROM observation_assets;");
    exec_sql("DROP TRIGGER IF EXISTS temp.fail_publication;");
}
void tearDown(void) {}

/* ---- insert ----------------------------------------------------------- */

static void test_batch_insert_writes_shared_asset_changes_and_publications(void) {
    observation_insert_result_t result = insert_ok(cam_a, 2, "bird");
    TEST_ASSERT_EQUAL_INT(2, result.object_count);
    TEST_ASSERT_TRUE(lightnvr_uuid_is_valid(result.asset_uuid));
    TEST_ASSERT_TRUE(result.last_seq > 0);

    char where[256];
    snprintf(where, sizeof(where), "asset_uuid = '%s' AND revision = 1", result.asset_uuid);
    TEST_ASSERT_EQUAL_INT(2, count_where("object_observations", where));
    snprintf(where, sizeof(where), "uuid = '%s' AND state = 'pending' AND reason IS NULL",
             result.asset_uuid);
    TEST_ASSERT_EQUAL_INT(1, count_where("observation_assets", where));

    for (int i = 0; i < 2; ++i) {
        observation_record_t record;
        TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_get(result.observation_uuids[i], &record));
        TEST_ASSERT_EQUAL_STRING(cam_a, record.camera_uuid);
        TEST_ASSERT_EQUAL_STRING("cam_a", record.stream_name_at_capture);
        TEST_ASSERT_EQUAL_INT(i, record.object_index);
        TEST_ASSERT_EQUAL_INT(1, record.revision);
        TEST_ASSERT_EQUAL_STRING(result.asset_uuid, record.asset.uuid);
        TEST_ASSERT_EQUAL_INT(OBS_ASSET_PENDING, record.asset.state);
        TEST_ASSERT_TRUE(record.has_box);
        TEST_ASSERT_TRUE(record.box_width > 0.2999999 && record.box_width < 0.3000001);

        snprintf(where, sizeof(where),
                 "observation_uuid = '%s' AND revision = 1 AND cause = 'created' "
                 "AND evidence_state = 'pending' AND evidence_reason IS NULL",
                 result.observation_uuids[i]);
        TEST_ASSERT_EQUAL_INT(1, count_where("observation_changes", where));

        char message_id[OBS_UUID_SIZE];
        TEST_ASSERT_EQUAL_INT(0, observation_message_id(source, result.observation_uuids[i],
                                                        1, message_id));
        snprintf(where, sizeof(where),
                 "message_id = '%s' AND state = 'pending' AND attempts = 0 AND change_seq = "
                 "(SELECT seq FROM observation_changes WHERE observation_uuid = '%s')",
                 message_id, result.observation_uuids[i]);
        TEST_ASSERT_EQUAL_INT(1, count_where("observation_publications", where));
    }
}

static void test_duplicate_batch_retry_is_a_noop_returning_existing_ids(void) {
    observation_batch_input_t batch = make_batch(cam_a, 3, "bird");
    observation_asset_input_t asset = pending_asset();
    observation_insert_result_t first, second;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_insert_batch(&batch, &asset, &first));
    int changes = scalar("SELECT count(*) FROM observation_changes");
    int assets = scalar("SELECT count(*) FROM observation_assets");

    TEST_ASSERT_EQUAL_INT(DB_OBS_DUPLICATE, db_observation_insert_batch(&batch, &asset, &second));
    TEST_ASSERT_EQUAL_INT(3, second.object_count);
    TEST_ASSERT_EQUAL_STRING(first.asset_uuid, second.asset_uuid);
    for (int i = 0; i < 3; ++i)
        TEST_ASSERT_EQUAL_STRING(first.observation_uuids[i], second.observation_uuids[i]);
    TEST_ASSERT_EQUAL_INT64(0, second.last_seq);
    TEST_ASSERT_EQUAL_INT(changes, scalar("SELECT count(*) FROM observation_changes"));
    TEST_ASSERT_EQUAL_INT(assets, scalar("SELECT count(*) FROM observation_assets"));

    /* The same batch ID claimed by another camera is a conflict, not a retry. */
    safe_strcpy(batch.camera_uuid, cam_b, sizeof(batch.camera_uuid), 0);
    TEST_ASSERT_EQUAL_INT(DB_OBS_CONFLICT, db_observation_insert_batch(&batch, &asset, &second));
}

static void test_insert_validation_rejects_contract_violations(void) {
    observation_insert_result_t result;
    observation_asset_input_t asset = pending_asset();
    observation_batch_input_t batch;

    batch = make_batch("99999999-9999-4999-8999-999999999999", 1, "bird");
    TEST_ASSERT_EQUAL_INT(DB_OBS_NOT_FOUND, db_observation_insert_batch(&batch, &asset, &result));

    batch = make_batch(cam_a, 2, "bird");
    batch.objects[1].object_index = 0;
    TEST_ASSERT_EQUAL_INT(DB_OBS_INVALID, db_observation_insert_batch(&batch, &asset, &result));
    batch = make_batch(cam_a, 1, "bird");
    batch.objects[0].object_index = 20;
    TEST_ASSERT_EQUAL_INT(DB_OBS_INVALID, db_observation_insert_batch(&batch, &asset, &result));
    batch = make_batch(cam_a, 1, "bird");
    batch.objects[0].confidence = 1.5;
    TEST_ASSERT_EQUAL_INT(DB_OBS_INVALID, db_observation_insert_batch(&batch, &asset, &result));
    batch = make_batch(cam_a, 1, "bird");
    batch.objects[0].box_x = 0.9;
    TEST_ASSERT_EQUAL_INT(DB_OBS_INVALID, db_observation_insert_batch(&batch, &asset, &result));
    batch = make_batch(cam_a, 1, "");
    TEST_ASSERT_EQUAL_INT(DB_OBS_INVALID, db_observation_insert_batch(&batch, &asset, &result));
    batch = make_batch(cam_a, 1, "bird");
    safe_strcpy(batch.engine_type, "motion", sizeof(batch.engine_type), 0);
    TEST_ASSERT_EQUAL_INT(DB_OBS_INVALID, db_observation_insert_batch(&batch, &asset, &result));
    batch = make_batch(cam_a, 1, "bird");
    safe_strcpy(batch.model, "/var/lib/models/x.sod", sizeof(batch.model), 0);
    TEST_ASSERT_EQUAL_INT(DB_OBS_INVALID, db_observation_insert_batch(&batch, &asset, &result));
    batch = make_batch(cam_a, 0, "bird");
    TEST_ASSERT_EQUAL_INT(DB_OBS_INVALID, db_observation_insert_batch(&batch, &asset, &result));

    batch = make_batch(cam_a, 1, "bird");
    asset.state = OBS_ASSET_READY;
    TEST_ASSERT_EQUAL_INT(DB_OBS_INVALID, db_observation_insert_batch(&batch, &asset, &result));
    asset.state = OBS_ASSET_UNAVAILABLE;
    TEST_ASSERT_EQUAL_INT(DB_OBS_INVALID, db_observation_insert_batch(&batch, &asset, &result));
    safe_strcpy(asset.reason, "retention_expired", sizeof(asset.reason), 0);
    TEST_ASSERT_EQUAL_INT(DB_OBS_INVALID, db_observation_insert_batch(&batch, &asset, &result));
    asset.state = OBS_ASSET_PENDING;
    safe_strcpy(asset.reason, "queue_full", sizeof(asset.reason), 0);
    TEST_ASSERT_EQUAL_INT(DB_OBS_INVALID, db_observation_insert_batch(&batch, &asset, &result));

    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT count(*) FROM object_observations"));
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT count(*) FROM observation_assets"));

    /* A dropped frame is created directly as unavailable(queue_full) with no box. */
    asset = pending_asset();
    asset.state = OBS_ASSET_UNAVAILABLE;
    safe_strcpy(asset.reason, "queue_full", sizeof(asset.reason), 0);
    batch.objects[0].has_box = false;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_insert_batch(&batch, &asset, &result));
    TEST_ASSERT_EQUAL_INT(1, count_where("observation_changes",
        "evidence_state = 'unavailable' AND evidence_reason = 'queue_full'"));
}

/* ---- transitions ------------------------------------------------------ */

static void test_transition_bumps_every_sharing_observation(void) {
    observation_insert_result_t inserted = insert_ok(cam_a, 2, "bird");
    observation_transition_result_t result;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_asset_transition(
        inserted.asset_uuid, OBS_ASSET_PENDING, OBS_ASSET_READY, NULL, &ready_media, &result));
    TEST_ASSERT_EQUAL_INT(2, result.observations_bumped);
    TEST_ASSERT_TRUE(result.last_seq > inserted.last_seq);

    for (int i = 0; i < 2; ++i) {
        observation_record_t record;
        TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_get(inserted.observation_uuids[i], &record));
        TEST_ASSERT_EQUAL_INT(2, record.revision);
        TEST_ASSERT_EQUAL_INT(OBS_ASSET_READY, record.asset.state);
        TEST_ASSERT_EQUAL_INT(640, record.asset.width);
        TEST_ASSERT_EQUAL_INT64(4096, record.asset.bytes);
        TEST_ASSERT_EQUAL_STRING(ready_media.path, record.asset.path);
        char where[256], message_id[OBS_UUID_SIZE];
        snprintf(where, sizeof(where),
                 "observation_uuid = '%s' AND revision = 2 AND cause = 'evidence' "
                 "AND evidence_state = 'ready'", inserted.observation_uuids[i]);
        TEST_ASSERT_EQUAL_INT(1, count_where("observation_changes", where));
        TEST_ASSERT_EQUAL_INT(0, observation_message_id(source, inserted.observation_uuids[i],
                                                        2, message_id));
        snprintf(where, sizeof(where), "message_id = '%s'", message_id);
        TEST_ASSERT_EQUAL_INT(1, count_where("observation_publications", where));
    }

    /* ready -> expired keeps metadata and records the reason at that revision. */
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_asset_transition(
        inserted.asset_uuid, OBS_ASSET_READY, OBS_ASSET_EXPIRED, "retention_expired", NULL, &result));
    observation_record_t record;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_get(inserted.observation_uuids[0], &record));
    TEST_ASSERT_EQUAL_INT(3, record.revision);
    TEST_ASSERT_EQUAL_INT(OBS_ASSET_EXPIRED, record.asset.state);
    TEST_ASSERT_EQUAL_STRING("retention_expired", record.asset.reason);
    TEST_ASSERT_EQUAL_INT(640, record.asset.width);
    TEST_ASSERT_EQUAL_INT(2, count_where("observation_changes",
        "revision = 3 AND evidence_state = 'expired' AND evidence_reason = 'retention_expired'"));
}

static void test_transition_rejects_invalid_edges_and_reasons(void) {
    observation_insert_result_t inserted = insert_ok(cam_a, 1, "bird");
    observation_transition_result_t result;
    const char *asset = inserted.asset_uuid;
    TEST_ASSERT_EQUAL_INT(DB_OBS_INVALID, db_observation_asset_transition(
        asset, OBS_ASSET_PENDING, OBS_ASSET_EXPIRED, "deleted", NULL, &result));
    TEST_ASSERT_EQUAL_INT(DB_OBS_INVALID, db_observation_asset_transition(
        asset, OBS_ASSET_PENDING, OBS_ASSET_UNAVAILABLE, "queue_full", NULL, &result));
    TEST_ASSERT_EQUAL_INT(DB_OBS_INVALID, db_observation_asset_transition(
        asset, OBS_ASSET_PENDING, OBS_ASSET_UNAVAILABLE, "bogus", NULL, &result));
    TEST_ASSERT_EQUAL_INT(DB_OBS_INVALID, db_observation_asset_transition(
        asset, OBS_ASSET_PENDING, OBS_ASSET_READY, NULL, NULL, &result));
    TEST_ASSERT_EQUAL_INT(DB_OBS_INVALID, db_observation_asset_transition(
        asset, OBS_ASSET_PENDING, OBS_ASSET_READY, "write_failed", &ready_media, &result));
    TEST_ASSERT_EQUAL_INT(DB_OBS_INVALID, db_observation_asset_transition(
        asset, OBS_ASSET_UNAVAILABLE, OBS_ASSET_READY, NULL, &ready_media, &result));
    TEST_ASSERT_EQUAL_INT(DB_OBS_NOT_FOUND, db_observation_asset_transition(
        "99999999-9999-4999-8999-999999999999", OBS_ASSET_PENDING, OBS_ASSET_READY,
        NULL, &ready_media, &result));
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_asset_transition(
        asset, OBS_ASSET_PENDING, OBS_ASSET_UNAVAILABLE, "encode_failed", NULL, &result));
    /* Terminal: a later attempt from pending loses the CAS. */
    TEST_ASSERT_EQUAL_INT(DB_OBS_CAS_LOST, db_observation_asset_transition(
        asset, OBS_ASSET_PENDING, OBS_ASSET_READY, NULL, &ready_media, &result));
    observation_record_t record;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_get(inserted.observation_uuids[0], &record));
    TEST_ASSERT_EQUAL_INT(2, record.revision);
}

typedef struct {
    const char *asset;
    observation_asset_state_t to;
    const char *reason;
    const observation_asset_media_t *media;
    pthread_barrier_t *barrier;
    int rc;
} race_arg_t;

static void *race_worker(void *opaque) {
    race_arg_t *arg = opaque;
    observation_transition_result_t result;
    pthread_barrier_wait(arg->barrier);
    arg->rc = db_observation_asset_transition(arg->asset, OBS_ASSET_PENDING, arg->to,
                                              arg->reason, arg->media, &result);
    return NULL;
}

static void test_cas_transition_race_has_exactly_one_winner(void) {
    for (int round = 0; round < 20; ++round) {
        observation_insert_result_t inserted = insert_ok(cam_a, 2, "bird");
        pthread_barrier_t barrier;
        pthread_barrier_init(&barrier, NULL, 2);
        race_arg_t a = {inserted.asset_uuid, OBS_ASSET_READY, NULL, &ready_media, &barrier, -99};
        race_arg_t b = {inserted.asset_uuid, OBS_ASSET_UNAVAILABLE, "interrupted", NULL, &barrier, -99};
        pthread_t ta, tb;
        pthread_create(&ta, NULL, race_worker, &a);
        pthread_create(&tb, NULL, race_worker, &b);
        pthread_join(ta, NULL);
        pthread_join(tb, NULL);
        pthread_barrier_destroy(&barrier);
        TEST_ASSERT_TRUE((a.rc == DB_OBS_OK && b.rc == DB_OBS_CAS_LOST) ||
                         (a.rc == DB_OBS_CAS_LOST && b.rc == DB_OBS_OK));
        char where[160];
        for (int i = 0; i < 2; ++i) {
            snprintf(where, sizeof(where), "observation_uuid = '%s'", inserted.observation_uuids[i]);
            TEST_ASSERT_EQUAL_INT(2, count_where("observation_changes", where));
            snprintf(where, sizeof(where), "uuid = '%s' AND revision = 2", inserted.observation_uuids[i]);
            TEST_ASSERT_EQUAL_INT(1, count_where("object_observations", where));
        }
    }
}

/* ---- rollback --------------------------------------------------------- */

static void test_injected_publication_failure_rolls_back_everything(void) {
    int observations = scalar("SELECT count(*) FROM object_observations");
    int assets = scalar("SELECT count(*) FROM observation_assets");
    int changes = scalar("SELECT count(*) FROM observation_changes");
    observation_insert_result_t survivor = insert_ok(cam_a, 1, "bird");

    /* Fires after the change row is inserted, before the publication row. */
    exec_sql("CREATE TEMP TRIGGER fail_publication BEFORE INSERT ON observation_publications "
             "BEGIN SELECT RAISE(ABORT, 'injected'); END;");
    observation_batch_input_t batch = make_batch(cam_a, 2, "bird");
    observation_asset_input_t asset = pending_asset();
    observation_insert_result_t result;
    TEST_ASSERT_EQUAL_INT(DB_OBS_ERROR, db_observation_insert_batch(&batch, &asset, &result));
    TEST_ASSERT_EQUAL_INT(observations + 1, scalar("SELECT count(*) FROM object_observations"));
    TEST_ASSERT_EQUAL_INT(assets + 1, scalar("SELECT count(*) FROM observation_assets"));
    TEST_ASSERT_EQUAL_INT(changes + 1, scalar("SELECT count(*) FROM observation_changes"));

    observation_transition_result_t transition;
    TEST_ASSERT_EQUAL_INT(DB_OBS_ERROR, db_observation_asset_transition(
        survivor.asset_uuid, OBS_ASSET_PENDING, OBS_ASSET_READY, NULL, &ready_media, &transition));
    exec_sql("DROP TRIGGER temp.fail_publication;");
    observation_record_t record;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_get(survivor.observation_uuids[0], &record));
    TEST_ASSERT_EQUAL_INT(1, record.revision);
    TEST_ASSERT_EQUAL_INT(OBS_ASSET_PENDING, record.asset.state);
    TEST_ASSERT_EQUAL_INT(changes + 1, scalar("SELECT count(*) FROM observation_changes"));

    /* The connection is usable and the retry succeeds afterwards. */
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_insert_batch(&batch, &asset, &result));
}

/* ---- cursor pages ----------------------------------------------------- */

static bool page_has(const observation_page_t *page, const char *uuid, int *revision) {
    for (size_t i = 0; i < page->count; ++i) {
        if (strcmp(page->items[i].uuid, uuid) == 0) {
            if (revision) *revision = page->items[i].revision;
            return true;
        }
    }
    return false;
}

static void test_ready_transition_is_seen_after_cursor_passed_created(void) {
    observation_insert_result_t inserted = insert_ok(cam_a, 1, "bird");
    observation_list_query_t query = all_query(100);
    observation_page_t page;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, list_page(&query, NULL, &page));
    int revision = 0;
    TEST_ASSERT_TRUE(page_has(&page, inserted.observation_uuids[0], &revision));
    TEST_ASSERT_EQUAL_INT(1, revision);
    TEST_ASSERT_FALSE(page.has_more);
    char cursor[OBS_CURSOR_SIZE];
    safe_strcpy(cursor, page.next_cursor, sizeof(cursor), 0);
    db_observation_page_free(&page);

    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, list_page(&query, cursor, &page));
    TEST_ASSERT_EQUAL_UINT(0, page.count);
    TEST_ASSERT_EQUAL_STRING(cursor, page.next_cursor);
    db_observation_page_free(&page);

    observation_transition_result_t result;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_asset_transition(
        inserted.asset_uuid, OBS_ASSET_PENDING, OBS_ASSET_READY, NULL, &ready_media, &result));
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, list_page(&query, cursor, &page));
    TEST_ASSERT_EQUAL_UINT(1, page.count);
    TEST_ASSERT_EQUAL_STRING(inserted.observation_uuids[0], page.items[0].uuid);
    TEST_ASSERT_EQUAL_INT(2, page.items[0].revision);
    TEST_ASSERT_EQUAL_INT(OBS_ASSET_READY, page.items[0].asset.state);
    db_observation_page_free(&page);
}

static void test_limit_counts_deduplicated_observations_and_cursor_tracks_changes(void) {
    observation_insert_result_t first = insert_ok(cam_a, 1, "bird");
    observation_insert_result_t second = insert_ok(cam_a, 1, "bird");
    observation_transition_result_t result;
    /* Changes: first@1, second@1, first@2, second@2. */
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_asset_transition(
        first.asset_uuid, OBS_ASSET_PENDING, OBS_ASSET_READY, NULL, &ready_media, &result));
    int64_t first_ready_seq = result.last_seq;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_asset_transition(
        second.asset_uuid, OBS_ASSET_PENDING, OBS_ASSET_READY, NULL, &ready_media, &result));

    observation_list_query_t query = all_query(1);
    observation_page_t page;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, list_page(&query, NULL, &page));
    TEST_ASSERT_EQUAL_UINT(1, page.count);
    TEST_ASSERT_EQUAL_STRING(first.observation_uuids[0], page.items[0].uuid);
    TEST_ASSERT_EQUAL_INT(2, page.items[0].revision);  /* current state at read time */
    TEST_ASSERT_TRUE(page.has_more);
    int64_t seq;
    char epoch[OBS_UUID_SIZE];
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, observation_cursor_decode(page.next_cursor, &seq, epoch));
    TEST_ASSERT_EQUAL_INT64(first.last_seq, seq);  /* stopped before second's change */
    db_observation_page_free(&page);

    query.limit = 2;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, list_page(&query, NULL, &page));
    TEST_ASSERT_EQUAL_UINT(2, page.count);
    TEST_ASSERT_FALSE(page.has_more);  /* all 4 changes scanned; 2 observations */
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, observation_cursor_decode(page.next_cursor, &seq, epoch));
    TEST_ASSERT_EQUAL_INT64(result.last_seq, seq);
    TEST_ASSERT_TRUE(seq > first_ready_seq);
    db_observation_page_free(&page);
}

static void test_filters_exclude_other_camera_and_labels(void) {
    observation_insert_result_t a = insert_ok(cam_a, 1, "Bird");
    observation_insert_result_t b = insert_ok(cam_b, 1, "bird");
    observation_insert_result_t c = insert_ok(cam_a, 1, "person");

    char allowed[1][OBS_UUID_SIZE];
    safe_strcpy(allowed[0], cam_a, OBS_UUID_SIZE, 0);

    observation_list_query_t query = all_query(100);
    query.all_cameras = false;
    query.camera_uuids = (const char (*)[OBS_UUID_SIZE])allowed;
    query.camera_count = 1;
    observation_page_t page;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, list_page(&query, NULL, &page));
    TEST_ASSERT_EQUAL_UINT(2, page.count);
    TEST_ASSERT_TRUE(page_has(&page, a.observation_uuids[0], NULL));
    TEST_ASSERT_TRUE(page_has(&page, c.observation_uuids[0], NULL));
    TEST_ASSERT_FALSE(page_has(&page, b.observation_uuids[0], NULL));
    TEST_ASSERT_FALSE(page.has_more);
    db_observation_page_free(&page);

    query.label_count = 1;
    safe_strcpy(query.labels[0], "BIRD", sizeof(query.labels[0]), 0);
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, list_page(&query, NULL, &page));
    TEST_ASSERT_EQUAL_UINT(1, page.count);
    TEST_ASSERT_EQUAL_STRING(a.observation_uuids[0], page.items[0].uuid);
    db_observation_page_free(&page);

    /* An empty authorized set yields nothing, but the cursor still advances. */
    query.label_count = 0;
    query.camera_count = 0;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, list_page(&query, NULL, &page));
    TEST_ASSERT_EQUAL_UINT(0, page.count);
    int64_t seq;
    char epoch[OBS_UUID_SIZE];
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, observation_cursor_decode(page.next_cursor, &seq, epoch));
    TEST_ASSERT_EQUAL_INT64(c.last_seq, seq);
    db_observation_page_free(&page);

    /* Time bounds: since inclusive, until exclusive. */
    observation_record_t record;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_get(b.observation_uuids[0], &record));
    query = all_query(100);
    query.has_since = true;
    query.since_ms = record.observed_at_ms;
    query.has_until = true;
    query.until_ms = record.observed_at_ms + 1;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, list_page(&query, NULL, &page));
    TEST_ASSERT_EQUAL_UINT(1, page.count);
    TEST_ASSERT_EQUAL_STRING(b.observation_uuids[0], page.items[0].uuid);
    db_observation_page_free(&page);

    /* A deleted capture camera hides its observations from list and get. */
    char cam_c[OBS_UUID_SIZE];
    make_camera("cam_c", cam_c);
    observation_insert_result_t gone = insert_ok(cam_c, 1, "bird");
    exec_sql("DELETE FROM streams WHERE name = 'cam_c';");
    TEST_ASSERT_EQUAL_INT(DB_OBS_NOT_FOUND, db_observation_get(gone.observation_uuids[0], &record));
    query = all_query(100);
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, list_page(&query, NULL, &page));
    TEST_ASSERT_FALSE(page_has(&page, gone.observation_uuids[0], NULL));
    TEST_ASSERT_TRUE(page_has(&page, b.observation_uuids[0], NULL));
    db_observation_page_free(&page);

    query.limit = 501;
    TEST_ASSERT_EQUAL_INT(DB_OBS_INVALID, list_page(&query, NULL, &page));
    query.limit = 10;
    query.has_since = query.has_until = true;
    query.since_ms = query.until_ms = 5;
    TEST_ASSERT_EQUAL_INT(DB_OBS_INVALID, list_page(&query, NULL, &page));
}

typedef struct {
    volatile int stop;
    int inserted;
} writer_arg_t;

static void *writer(void *opaque) {
    writer_arg_t *arg = opaque;
    while (!arg->stop && arg->inserted < 200) {
        observation_batch_input_t batch = make_batch(cam_a, 1, "bird");
        observation_asset_input_t asset = pending_asset();
        observation_insert_result_t result;
        if (db_observation_insert_batch(&batch, &asset, &result) == DB_OBS_OK) arg->inserted++;
    }
    return NULL;
}

static void test_watermark_pages_with_concurrent_writes_never_skip_or_loop(void) {
    char seeded[30][OBS_UUID_SIZE];
    for (int i = 0; i < 30; ++i) {
        observation_insert_result_t r = insert_ok(cam_a, 1, "bird");
        safe_strcpy(seeded[i], r.observation_uuids[0], OBS_UUID_SIZE, 0);
    }
    writer_arg_t arg = {0, 0};
    pthread_t thread;
    pthread_create(&thread, NULL, writer, &arg);

    bool seen[30] = {false};
    char cursor[OBS_CURSOR_SIZE] = "";
    int64_t previous = -1;
    int pages = 0;
    observation_list_query_t query = all_query(7);
    for (;;) {
        observation_page_t page;
        TEST_ASSERT_EQUAL_INT(DB_OBS_OK, list_page(&query, cursor[0] ? cursor : NULL, &page));
        for (int i = 0; i < 30; ++i) if (page_has(&page, seeded[i], NULL)) seen[i] = true;
        int64_t seq;
        char epoch[OBS_UUID_SIZE];
        TEST_ASSERT_EQUAL_INT(DB_OBS_OK, observation_cursor_decode(page.next_cursor, &seq, epoch));
        TEST_ASSERT_TRUE(seq > previous || (!page.has_more && page.count == 0));
        previous = seq;
        safe_strcpy(cursor, page.next_cursor, sizeof(cursor), 0);
        bool more = page.has_more;
        db_observation_page_free(&page);
        TEST_ASSERT_TRUE_MESSAGE(++pages < 200, "pages never converged");
        if (!more) {
            if (arg.stop) break;
            arg.stop = 1;
            pthread_join(thread, NULL);
        }
    }
    for (int i = 0; i < 30; ++i) TEST_ASSERT_TRUE(seen[i]);
    TEST_ASSERT_TRUE(arg.inserted > 0);
    /* The final cursor is at the head: everything committed has been paged. */
    observation_meta_t meta;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_meta_get(&meta));
    TEST_ASSERT_EQUAL_INT64(meta.head_seq, previous);
}

static void test_head_and_expired_cursors(void) {
    observation_insert_result_t old = insert_ok(cam_a, 1, "bird");
    observation_insert_result_t mid = insert_ok(cam_a, 1, "bird");
    observation_insert_result_t recent = insert_ok(cam_a, 1, "bird");
    (void)mid;
    observation_meta_t meta;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_meta_get(&meta));

    observation_list_query_t query = all_query(100);
    observation_page_t page;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, list_page(&query, "head", &page));
    TEST_ASSERT_EQUAL_UINT(0, page.count);
    TEST_ASSERT_FALSE(page.has_more);
    char expected[OBS_CURSOR_SIZE];
    TEST_ASSERT_EQUAL_INT(0, observation_cursor_encode(meta.head_seq, meta.epoch_uuid,
                                                       expected, sizeof(expected)));
    TEST_ASSERT_EQUAL_STRING(expected, page.next_cursor);
    db_observation_page_free(&page);

    char before_old[OBS_CURSOR_SIZE];
    TEST_ASSERT_EQUAL_INT(0, observation_cursor_encode(old.last_seq - 1, meta.epoch_uuid,
                                                       before_old, sizeof(before_old)));
    /* Prune the first two observations' changes. */
    char sql[256];
    snprintf(sql, sizeof(sql), "UPDATE observation_changes SET created_at_ms = 1 WHERE seq < %lld;",
             (long long)recent.last_seq);
    exec_sql(sql);
    observation_prune_result_t pruned;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observations_prune(1000, 100, &pruned));
    TEST_ASSERT_TRUE(pruned.changes_deleted >= 2);
    TEST_ASSERT_EQUAL_INT64(recent.last_seq, pruned.oldest_seq);

    TEST_ASSERT_EQUAL_INT(DB_OBS_CURSOR_EXPIRED, list_page(&query, before_old, &page));
    int64_t seq;
    char epoch[OBS_UUID_SIZE];
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, observation_cursor_decode(page.oldest_cursor, &seq, epoch));
    TEST_ASSERT_EQUAL_INT64(recent.last_seq - 1, seq);
    TEST_ASSERT_EQUAL_STRING(meta.epoch_uuid, epoch);
    TEST_ASSERT_TRUE(page.has_oldest_observed_at);
    observation_record_t record;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_get(recent.observation_uuids[0], &record));
    TEST_ASSERT_EQUAL_INT64(record.observed_at_ms, page.oldest_observed_at_ms);
    db_observation_page_free(&page);

    /* The advertised oldest cursor resumes cleanly. */
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, list_page(&query, expected, &page));
    db_observation_page_free(&page);
    char oldest[OBS_CURSOR_SIZE];
    TEST_ASSERT_EQUAL_INT(0, observation_cursor_encode(recent.last_seq - 1, meta.epoch_uuid,
                                                       oldest, sizeof(oldest)));
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, list_page(&query, oldest, &page));
    TEST_ASSERT_TRUE(page_has(&page, recent.observation_uuids[0], NULL));
    db_observation_page_free(&page);

    /* seq beyond the head (rewind without an epoch change). */
    char beyond[OBS_CURSOR_SIZE];
    TEST_ASSERT_EQUAL_INT(0, observation_cursor_encode(meta.head_seq + 5, meta.epoch_uuid,
                                                       beyond, sizeof(beyond)));
    TEST_ASSERT_EQUAL_INT(DB_OBS_CURSOR_EXPIRED, list_page(&query, beyond, &page));
    db_observation_page_free(&page);

    /* Epoch mismatch, then a regenerated epoch invalidates old cursors. */
    char foreign[OBS_CURSOR_SIZE];
    TEST_ASSERT_EQUAL_INT(0, observation_cursor_encode(meta.head_seq,
        "66666666-6666-4666-8666-666666666666", foreign, sizeof(foreign)));
    TEST_ASSERT_EQUAL_INT(DB_OBS_CURSOR_EXPIRED, list_page(&query, foreign, &page));
    db_observation_page_free(&page);
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_meta_regenerate_epoch());
    TEST_ASSERT_EQUAL_INT(DB_OBS_CURSOR_EXPIRED, list_page(&query, expected, &page));
    observation_meta_t fresh;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_meta_get(&fresh));
    TEST_ASSERT_TRUE(strcmp(fresh.epoch_uuid, meta.epoch_uuid) != 0);
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, observation_cursor_decode(page.oldest_cursor, &seq, epoch));
    TEST_ASSERT_EQUAL_STRING(fresh.epoch_uuid, epoch);
    db_observation_page_free(&page);

    TEST_ASSERT_EQUAL_INT(DB_OBS_INVALID_CURSOR, list_page(&query, "%%%", &page));
}

/* ---- publications ----------------------------------------------------- */

static void test_pending_publications_in_seq_order_and_mark(void) {
    exec_sql("DELETE FROM observation_publications;");
    observation_insert_result_t inserted = insert_ok(cam_b, 2, "bird");
    observation_transition_result_t result;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_asset_transition(
        inserted.asset_uuid, OBS_ASSET_PENDING, OBS_ASSET_UNAVAILABLE, "write_failed", NULL, &result));

    observation_publication_t pubs[8];
    int count = db_observation_pending_publications(pubs, 8);
    TEST_ASSERT_EQUAL_INT(4, count);
    for (int i = 1; i < count; ++i) TEST_ASSERT_TRUE(pubs[i].change_seq > pubs[i - 1].change_seq);
    TEST_ASSERT_EQUAL_INT(OBS_CAUSE_CREATED, pubs[0].cause);
    TEST_ASSERT_EQUAL_INT(OBS_ASSET_PENDING, pubs[0].evidence_state);
    TEST_ASSERT_EQUAL_STRING(cam_b, pubs[0].camera_uuid);
    TEST_ASSERT_EQUAL_STRING("bird", pubs[0].label);
    TEST_ASSERT_EQUAL_STRING(inserted.asset_uuid, pubs[0].asset_uuid);
    TEST_ASSERT_EQUAL_INT(OBS_CAUSE_EVIDENCE, pubs[3].cause);
    TEST_ASSERT_EQUAL_INT(2, pubs[3].revision);
    TEST_ASSERT_EQUAL_INT(OBS_ASSET_UNAVAILABLE, pubs[3].evidence_state);
    TEST_ASSERT_EQUAL_STRING("write_failed", pubs[3].evidence_reason);
    char message_id[OBS_UUID_SIZE];
    TEST_ASSERT_EQUAL_INT(0, observation_message_id(source, pubs[3].observation_uuid, 2, message_id));
    TEST_ASSERT_EQUAL_STRING(message_id, pubs[3].message_id);
    TEST_ASSERT_EQUAL_INT(2, db_observation_pending_publications(pubs, 2));

    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_publication_mark(
        pubs[0].change_seq, OBS_PUBLICATION_PENDING, "outbox full"));
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_publication_mark(
        pubs[0].change_seq, OBS_PUBLICATION_RELAYED, NULL));
    TEST_ASSERT_EQUAL_INT(DB_OBS_CAS_LOST, db_observation_publication_mark(
        pubs[0].change_seq, OBS_PUBLICATION_FAILED, "late"));
    TEST_ASSERT_EQUAL_INT(DB_OBS_NOT_FOUND, db_observation_publication_mark(
        999999, OBS_PUBLICATION_RELAYED, NULL));
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_publication_mark(
        pubs[1].change_seq, OBS_PUBLICATION_RELAYED_NO_ROUTE, NULL));
    char where[128];
    snprintf(where, sizeof(where), "change_seq = %lld AND state = 'relayed' AND attempts = 2 "
             "AND last_error IS NULL", (long long)pubs[0].change_seq);
    TEST_ASSERT_EQUAL_INT(1, count_where("observation_publications", where));
    TEST_ASSERT_EQUAL_INT(2, db_observation_pending_publications(pubs, 8));
}

/* ---- prune and bump --------------------------------------------------- */

static void test_prune_deletes_history_in_order_and_advances_boundary(void) {
    observation_insert_result_t dropped = insert_ok(cam_a, 1, "bird");
    observation_transition_result_t result;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_asset_transition(
        dropped.asset_uuid, OBS_ASSET_PENDING, OBS_ASSET_UNAVAILABLE, "encode_failed", NULL, &result));
    observation_insert_result_t ready = insert_ok(cam_a, 1, "bird");
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_asset_transition(
        ready.asset_uuid, OBS_ASSET_PENDING, OBS_ASSET_READY, NULL, &ready_media, &result));
    observation_insert_result_t kept = insert_ok(cam_a, 1, "bird");

    char sql[256];
    snprintf(sql, sizeof(sql), "UPDATE observation_changes SET created_at_ms = 10 WHERE seq < %lld;",
             (long long)kept.last_seq);
    exec_sql(sql);
    observation_meta_t before;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_meta_get(&before));

    /* Bounded batch: one change per call. */
    observation_prune_result_t pruned;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observations_prune(100, 1, &pruned));
    TEST_ASSERT_EQUAL_INT(1, pruned.changes_deleted);
    TEST_ASSERT_TRUE(pruned.oldest_seq > before.oldest_seq);
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observations_prune(100, 100, &pruned));
    TEST_ASSERT_EQUAL_INT64(kept.last_seq, pruned.oldest_seq);

    observation_record_t record;
    TEST_ASSERT_EQUAL_INT(DB_OBS_NOT_FOUND, db_observation_get(dropped.observation_uuids[0], &record));
    TEST_ASSERT_EQUAL_INT(DB_OBS_NOT_FOUND, db_observation_get(ready.observation_uuids[0], &record));
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_get(kept.observation_uuids[0], &record));
    /* Unavailable asset row is gone; the ready one stays for L4 to delete its file. */
    observation_asset_record_t asset;
    TEST_ASSERT_EQUAL_INT(DB_OBS_NOT_FOUND, db_observation_asset_get(dropped.asset_uuid, &asset));
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_asset_get(ready.asset_uuid, &asset));
    TEST_ASSERT_EQUAL_INT(OBS_ASSET_READY, asset.state);

    observation_meta_t after;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_meta_get(&after));
    TEST_ASSERT_EQUAL_INT64(kept.last_seq, after.oldest_seq);
    TEST_ASSERT_EQUAL_STRING(before.epoch_uuid, after.epoch_uuid);

    /* Nothing old left: a no-op that never moves the boundary back. */
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observations_prune(100, 100, &pruned));
    TEST_ASSERT_EQUAL_INT(0, pruned.changes_deleted);
    TEST_ASSERT_EQUAL_INT64(kept.last_seq, pruned.oldest_seq);
}

static void test_bump_revision_locked_for_classification(void) {
    observation_insert_result_t inserted = insert_ok(cam_a, 1, "bird");
    sqlite3 *db = get_db_handle();
    pthread_mutex_lock(get_db_mutex());
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_exec(db, "BEGIN IMMEDIATE;", NULL, NULL, NULL));
    int64_t seq = 0;
    int revision = 0;
    int rc = db_observation_bump_revision_locked(db, source, inserted.observation_uuids[0],
                                                 OBS_CAUSE_CLASSIFICATION, 1234, &seq, &revision);
    sqlite3_exec(db, rc == DB_OBS_OK ? "COMMIT;" : "ROLLBACK;", NULL, NULL, NULL);
    pthread_mutex_unlock(get_db_mutex());
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, rc);
    TEST_ASSERT_EQUAL_INT(2, revision);
    TEST_ASSERT_TRUE(seq > inserted.last_seq);
    char where[200];
    snprintf(where, sizeof(where), "seq = %lld AND cause = 'classification' AND "
             "evidence_state = 'pending'", (long long)seq);
    TEST_ASSERT_EQUAL_INT(1, count_where("observation_changes", where));
    snprintf(where, sizeof(where), "change_seq = %lld", (long long)seq);
    TEST_ASSERT_EQUAL_INT(1, count_where("observation_publications", where));
}

int main(void) {
    snprintf(db_path, sizeof(db_path), "/tmp/lightnvr_observations_%ld.db", (long)getpid());
    unlink(db_path);
    if (init_database(db_path) != 0) return 1;
    if (event_identity_init() != 0 || event_identity_get_source(source, sizeof(source)) != 0)
        return 1;

    UNITY_BEGIN();
    make_camera("cam_a", cam_a);
    make_camera("cam_b", cam_b);
    RUN_TEST(test_batch_insert_writes_shared_asset_changes_and_publications);
    RUN_TEST(test_duplicate_batch_retry_is_a_noop_returning_existing_ids);
    RUN_TEST(test_insert_validation_rejects_contract_violations);
    RUN_TEST(test_transition_bumps_every_sharing_observation);
    RUN_TEST(test_transition_rejects_invalid_edges_and_reasons);
    RUN_TEST(test_cas_transition_race_has_exactly_one_winner);
    RUN_TEST(test_injected_publication_failure_rolls_back_everything);
    RUN_TEST(test_ready_transition_is_seen_after_cursor_passed_created);
    RUN_TEST(test_limit_counts_deduplicated_observations_and_cursor_tracks_changes);
    RUN_TEST(test_filters_exclude_other_camera_and_labels);
    RUN_TEST(test_watermark_pages_with_concurrent_writes_never_skip_or_loop);
    RUN_TEST(test_head_and_expired_cursors);
    RUN_TEST(test_pending_publications_in_seq_order_and_mark);
    RUN_TEST(test_prune_deletes_history_in_order_and_advances_boundary);
    RUN_TEST(test_bump_revision_locked_for_classification);
    int result = UNITY_END();

    event_identity_shutdown();
    shutdown_database();
    unlink(db_path);
    char sidecar[300];
    snprintf(sidecar, sizeof(sidecar), "%s-wal", db_path); unlink(sidecar);
    snprintf(sidecar, sizeof(sidecar), "%s-shm", db_path); unlink(sidecar);
    return result;
}
