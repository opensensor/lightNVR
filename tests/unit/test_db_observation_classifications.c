/* INT_01 L13: classification annotations (migration 0096) and the
 * observations.annotate action. Every contract section 10.4 rule the DB owns,
 * the C3 classification fixture outcomes, transactional revision bumps, and a
 * migration up/down/up that keeps authz bit indexes unique. */
#define _GNU_SOURCE
#include <math.h>
#include <pthread.h>
#include <sqlite3.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include "unity.h"
#include "core/authorization.h"
#include "core/event_identity.h"
#include "database/db_core.h"
#include "database/db_embedded_migrations.h"
#include "database/db_observation_classifications.h"
#include "database/db_observations.h"
#include "database/db_streams.h"
#include "database/sqlite_migrate.h"
#include "utils/strings.h"
#include "utils/uuid.h"

/* Unity is built without double support. */
#define ASSERT_NEAR(expected, actual) TEST_ASSERT_TRUE(fabs((double)(expected) - (double)(actual)) < 1e-9)

#define WRITER_ID 41
#define OTHER_WRITER_ID 42
#define ADMIN_ROLE "00000000-0000-4000-8000-000000000001"

static char db_path[256];
static char cam[OBS_UUID_SIZE];

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

static int changes_for(const char *observation_uuid, const char *cause) {
    char sql[256];
    snprintf(sql, sizeof(sql), "SELECT count(*) FROM observation_changes WHERE "
             "observation_uuid = '%s' AND cause = '%s'", observation_uuid, cause);
    return scalar(sql);
}

static int annotation_rows(const char *observation_uuid) {
    char sql[200];
    snprintf(sql, sizeof(sql), "SELECT count(*) FROM observation_classifications "
             "WHERE observation_uuid = '%s'", observation_uuid);
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

static int64_t base_time_ms = 1794000000000LL;

typedef struct {
    char observation_uuid[OBS_UUID_SIZE];
    char asset_uuid[OBS_UUID_SIZE];
} fixture_obs_t;

/* One "bird" observation; ready evidence puts it at revision 2 like the C3
 * single-ready fixture. */
static fixture_obs_t make_observation(observation_asset_state_t final_state) {
    observation_batch_input_t batch;
    memset(&batch, 0, sizeof(batch));
    TEST_ASSERT_EQUAL_INT(0, lightnvr_uuid_generate_v4(batch.batch_id));
    safe_strcpy(batch.camera_uuid, cam, sizeof(batch.camera_uuid), 0);
    batch.observed_at_ms = base_time_ms++;
    safe_strcpy(batch.engine_key, "legacy-primary", sizeof(batch.engine_key), 0);
    safe_strcpy(batch.engine_type, "object", sizeof(batch.engine_type), 0);
    safe_strcpy(batch.model, "ssd_mobilenet_v2.sod", sizeof(batch.model), 0);
    batch.object_count = 1;
    batch.objects[0].object_index = 0;
    safe_strcpy(batch.objects[0].label, "bird", sizeof(batch.objects[0].label), 0);
    batch.objects[0].confidence = 0.87;
    batch.objects[0].has_box = true;
    batch.objects[0].box_x = 0.412; batch.objects[0].box_y = 0.288;
    batch.objects[0].box_width = 0.12; batch.objects[0].box_height = 0.165;

    observation_asset_input_t asset;
    memset(&asset, 0, sizeof(asset));
    asset.state = OBS_ASSET_PENDING;
    safe_strcpy(asset.origin, "inference_input", sizeof(asset.origin), 0);
    safe_strcpy(asset.media_type, "image/jpeg", sizeof(asset.media_type), 0);

    observation_insert_result_t inserted;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_insert_batch(&batch, &asset, &inserted));
    fixture_obs_t out;
    safe_strcpy(out.observation_uuid, inserted.observation_uuids[0], OBS_UUID_SIZE, 0);
    safe_strcpy(out.asset_uuid, inserted.asset_uuid, OBS_UUID_SIZE, 0);

    if (final_state == OBS_ASSET_PENDING) return out;
    static const observation_asset_media_t media = {
        .path = "/tmp/observations/x.jpg", .bytes = 4096, .width = 640, .height = 360,
    };
    observation_transition_result_t transition;
    if (final_state == OBS_ASSET_UNAVAILABLE) {
        TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_asset_transition(
            out.asset_uuid, OBS_ASSET_PENDING, OBS_ASSET_UNAVAILABLE, "encode_failed", NULL,
            &transition));
        return out;
    }
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_asset_transition(
        out.asset_uuid, OBS_ASSET_PENDING, OBS_ASSET_READY, NULL, &media, &transition));
    if (final_state == OBS_ASSET_EXPIRED) {
        TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_asset_transition(
            out.asset_uuid, OBS_ASSET_READY, OBS_ASSET_EXPIRED, "retention_expired", NULL,
            &transition));
    }
    return out;
}

/* C3 01-create-automated request body. */
static observation_classification_input_t automated_input(const fixture_obs_t *obs) {
    observation_classification_input_t in;
    memset(&in, 0, sizeof(in));
    in.writer_namespace = "yawamf";
    in.writer_user_id = WRITER_ID;
    in.name = "species";
    in.classifier = "yawamf";
    in.model_version = "convnext-v2-2026.10";
    in.label = "Black-capped Chickadee";
    in.taxonomy_id = "inat:144815";
    in.confidence = 0.93;
    in.classified_at_ms = 1794000195002LL;
    in.input_asset_uuid = obs->asset_uuid;
    in.input_revision = 2;
    in.kind = OBS_CLS_KIND_AUTOMATED;
    in.expected_revision = 0;
    return in;
}

/* C3 04-owner-confirm request body. */
static observation_classification_input_t owner_input(const fixture_obs_t *obs) {
    observation_classification_input_t in = automated_input(obs);
    in.model_version = NULL;
    in.label = "Carolina Chickadee";
    in.taxonomy_id = NULL;
    in.confidence = 1.0;
    in.classified_at_ms = 1794015641100LL;
    in.input_revision = 3;
    in.kind = OBS_CLS_KIND_OWNER_CONFIRMED;
    in.expected_revision = 1;
    return in;
}

static int put(const fixture_obs_t *obs, const observation_classification_input_t *in,
               observation_classification_result_t *result) {
    memset(result, 0x5a, sizeof(*result));
    return db_observation_classification_put(obs->observation_uuid, in, result);
}

static observation_record_t get_record(const fixture_obs_t *obs) {
    observation_record_t record;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_get(obs->observation_uuid, &record));
    return record;
}

void setUp(void) {
    exec_sql("DELETE FROM object_observations; DELETE FROM observation_assets;");
    exec_sql("DROP TRIGGER IF EXISTS temp.fail_publication;");
}
void tearDown(void) {}

/* ---- create / no-op ---------------------------------------------------- */

static void test_create_automated_bumps_revision_and_stages_publication(void) {
    fixture_obs_t obs = make_observation(OBS_ASSET_READY);
    observation_record_t before = get_record(&obs);
    TEST_ASSERT_EQUAL_INT(2, before.revision);

    observation_classification_input_t in = automated_input(&obs);
    observation_classification_result_t r;
    TEST_ASSERT_EQUAL_INT(DB_OBS_CLS_CREATED, put(&obs, &in, &r));
    TEST_ASSERT_TRUE(r.has_annotation);
    TEST_ASSERT_EQUAL_INT(3, r.observation_revision);
    TEST_ASSERT_TRUE(r.change_seq > 0);
    TEST_ASSERT_EQUAL_STRING("yawamf/species", r.annotation.key);
    TEST_ASSERT_EQUAL_STRING("yawamf", r.annotation.writer_namespace);
    TEST_ASSERT_EQUAL_INT64(WRITER_ID, r.annotation.writer_user_id);
    TEST_ASSERT_EQUAL_STRING("yawamf", r.annotation.classifier);
    TEST_ASSERT_EQUAL_STRING("convnext-v2-2026.10", r.annotation.model_version);
    TEST_ASSERT_EQUAL_STRING("Black-capped Chickadee", r.annotation.label);
    TEST_ASSERT_EQUAL_STRING("inat:144815", r.annotation.taxonomy_id);
    ASSERT_NEAR(0.93, r.annotation.confidence);
    TEST_ASSERT_EQUAL_INT64(1794000195002LL, r.annotation.classified_at_ms);
    TEST_ASSERT_EQUAL_STRING(obs.asset_uuid, r.annotation.input_asset_uuid);
    TEST_ASSERT_EQUAL_INT(2, r.annotation.input_revision);
    TEST_ASSERT_EQUAL_INT(OBS_CLS_KIND_AUTOMATED, r.annotation.kind);
    TEST_ASSERT_EQUAL_INT(1, r.annotation.annotation_revision);
    TEST_ASSERT_TRUE(r.annotation.created_at_ms > 0);
    TEST_ASSERT_EQUAL_INT64(r.annotation.created_at_ms, r.annotation.updated_at_ms);

    /* Change row snapshots the evidence state; publication pending; same tx. */
    char where[256];
    snprintf(where, sizeof(where), "SELECT count(*) FROM observation_changes c "
             "JOIN observation_publications p ON p.change_seq = c.seq WHERE c.seq = %lld "
             "AND c.revision = 3 AND c.cause = 'classification' AND c.evidence_state = 'ready' "
             "AND p.state = 'pending'", (long long)r.change_seq);
    TEST_ASSERT_EQUAL_INT(1, scalar(where));
    char message_id[OBS_UUID_SIZE];
    char source[160];
    TEST_ASSERT_EQUAL_INT(0, event_identity_get_source(source, sizeof(source)));
    TEST_ASSERT_EQUAL_INT(0, observation_message_id(source, obs.observation_uuid, 3, message_id));
    snprintf(where, sizeof(where), "SELECT count(*) FROM observation_publications "
             "WHERE change_seq = %lld AND message_id = '%s'", (long long)r.change_seq,
             message_id);
    TEST_ASSERT_EQUAL_INT(1, scalar(where));

    /* Detector fields are never modified. */
    observation_record_t after = get_record(&obs);
    TEST_ASSERT_EQUAL_INT(3, after.revision);
    TEST_ASSERT_EQUAL_STRING("bird", after.label);
    ASSERT_NEAR(0.87, after.confidence);
    TEST_ASSERT_TRUE(after.has_box);
    ASSERT_NEAR(before.box_x, after.box_x);
    ASSERT_NEAR(before.box_height, after.box_height);
    TEST_ASSERT_EQUAL_STRING(before.engine_key, after.engine_key);
    TEST_ASSERT_EQUAL_STRING(before.model, after.model);
    TEST_ASSERT_EQUAL_INT(OBS_ASSET_READY, after.asset.state);
}

static void test_identical_retry_is_noop_regardless_of_expected_revision(void) {
    fixture_obs_t obs = make_observation(OBS_ASSET_READY);
    observation_classification_input_t in = automated_input(&obs);
    observation_classification_result_t created;
    TEST_ASSERT_EQUAL_INT(DB_OBS_CLS_CREATED, put(&obs, &in, &created));
    int classification_changes = changes_for(obs.observation_uuid, "classification");
    int publications = scalar("SELECT count(*) FROM observation_publications");

    const int expected[] = {0, 1, 7};
    for (size_t i = 0; i < sizeof(expected) / sizeof(expected[0]); ++i) {
        in.expected_revision = expected[i];
        observation_classification_result_t r;
        TEST_ASSERT_EQUAL_INT(DB_OBS_CLS_NOOP, put(&obs, &in, &r));
        TEST_ASSERT_TRUE(r.has_annotation);
        TEST_ASSERT_EQUAL_INT(1, r.annotation.annotation_revision);
        TEST_ASSERT_EQUAL_INT(3, r.observation_revision);
        TEST_ASSERT_EQUAL_INT64(0, r.change_seq);
        TEST_ASSERT_EQUAL_INT64(created.annotation.updated_at_ms, r.annotation.updated_at_ms);
    }
    /* Confidence is compared to 1e-6. */
    in.confidence = 0.93 + 5e-7;
    observation_classification_result_t r;
    TEST_ASSERT_EQUAL_INT(DB_OBS_CLS_NOOP, put(&obs, &in, &r));
    /* Null and empty optional strings are the same stored value. */
    in.confidence = 0.93;
    observation_classification_input_t nulls = owner_input(&obs);
    nulls.expected_revision = 0;
    nulls.name = "nulls";
    TEST_ASSERT_EQUAL_INT(DB_OBS_CLS_CREATED, put(&obs, &nulls, &r));
    TEST_ASSERT_EQUAL_STRING("", r.annotation.model_version);
    TEST_ASSERT_EQUAL_STRING("", r.annotation.taxonomy_id);
    nulls.model_version = "";
    nulls.taxonomy_id = "";
    TEST_ASSERT_EQUAL_INT(DB_OBS_CLS_NOOP, put(&obs, &nulls, &r));

    TEST_ASSERT_EQUAL_INT(classification_changes + 1,
                          changes_for(obs.observation_uuid, "classification"));
    TEST_ASSERT_EQUAL_INT(publications + 1, scalar("SELECT count(*) FROM observation_publications"));
    TEST_ASSERT_EQUAL_INT(4, get_record(&obs).revision);
}

/* ---- revision CAS ----------------------------------------------------- */

static void test_expected_revision_compare_and_set(void) {
    fixture_obs_t obs = make_observation(OBS_ASSET_READY);
    observation_classification_input_t in = automated_input(&obs);
    observation_classification_result_t r;

    /* Absent key: only 0 / omitted is accepted. */
    in.expected_revision = 1;
    TEST_ASSERT_EQUAL_INT(DB_OBS_CLS_REVISION_CONFLICT, put(&obs, &in, &r));
    TEST_ASSERT_FALSE(r.has_annotation);
    TEST_ASSERT_EQUAL_INT(0, annotation_rows(obs.observation_uuid));
    in.expected_revision = 0;
    TEST_ASSERT_EQUAL_INT(DB_OBS_CLS_CREATED, put(&obs, &in, &r));

    /* C3 03: a different result with a stale expected_revision returns current. */
    observation_classification_input_t changed = in;
    changed.model_version = "convnext-v2-2026.11";
    changed.confidence = 0.95;
    changed.classified_at_ms = 1794000242550LL;
    changed.expected_revision = 0;
    TEST_ASSERT_EQUAL_INT(DB_OBS_CLS_REVISION_CONFLICT, put(&obs, &changed, &r));
    TEST_ASSERT_TRUE(r.has_annotation);
    TEST_ASSERT_EQUAL_STRING("convnext-v2-2026.10", r.annotation.model_version);
    TEST_ASSERT_EQUAL_INT(1, r.annotation.annotation_revision);
    TEST_ASSERT_EQUAL_INT(3, r.observation_revision);
    changed.expected_revision = 2;
    TEST_ASSERT_EQUAL_INT(DB_OBS_CLS_REVISION_CONFLICT, put(&obs, &changed, &r));
    TEST_ASSERT_EQUAL_INT(3, get_record(&obs).revision);

    /* Matching revision stores it: annotation 1 -> 2, observation 3 -> 4. */
    changed.expected_revision = 1;
    TEST_ASSERT_EQUAL_INT(DB_OBS_CLS_UPDATED, put(&obs, &changed, &r));
    TEST_ASSERT_EQUAL_INT(2, r.annotation.annotation_revision);
    TEST_ASSERT_EQUAL_INT(4, r.observation_revision);
    TEST_ASSERT_EQUAL_STRING("convnext-v2-2026.11", r.annotation.model_version);
    TEST_ASSERT_EQUAL_INT(2, changes_for(obs.observation_uuid, "classification"));
}

/* ---- owner precedence ------------------------------------------------- */

static void test_owner_confirmed_replaces_automated_and_locks_it(void) {
    fixture_obs_t obs = make_observation(OBS_ASSET_READY);
    observation_classification_input_t in = automated_input(&obs);
    observation_classification_result_t created;
    TEST_ASSERT_EQUAL_INT(DB_OBS_CLS_CREATED, put(&obs, &in, &created));

    /* C3 04: owner correction, annotation 1 -> 2, observation 3 -> 4. */
    observation_classification_input_t owner = owner_input(&obs);
    observation_classification_result_t r;
    TEST_ASSERT_EQUAL_INT(DB_OBS_CLS_UPDATED, put(&obs, &owner, &r));
    TEST_ASSERT_EQUAL_INT(OBS_CLS_KIND_OWNER_CONFIRMED, r.annotation.kind);
    TEST_ASSERT_EQUAL_STRING("Carolina Chickadee", r.annotation.label);
    TEST_ASSERT_EQUAL_STRING("", r.annotation.model_version);
    TEST_ASSERT_EQUAL_STRING("", r.annotation.taxonomy_id);
    TEST_ASSERT_EQUAL_INT(3, r.annotation.input_revision);
    TEST_ASSERT_EQUAL_INT(2, r.annotation.annotation_revision);
    TEST_ASSERT_EQUAL_INT(4, r.observation_revision);
    TEST_ASSERT_EQUAL_INT64(created.annotation.created_at_ms, r.annotation.created_at_ms);
    TEST_ASSERT_TRUE(r.annotation.updated_at_ms >= created.annotation.updated_at_ms);

    /* C3 05: automated re-run with the right revision is still locked. */
    observation_classification_input_t rerun = in;
    rerun.model_version = "convnext-v2-2026.11";
    rerun.confidence = 0.95;
    rerun.input_revision = 4;
    rerun.expected_revision = 2;
    TEST_ASSERT_EQUAL_INT(DB_OBS_CLS_OWNER_LOCKED, put(&obs, &rerun, &r));
    TEST_ASSERT_TRUE(r.has_annotation);
    TEST_ASSERT_EQUAL_STRING("Carolina Chickadee", r.annotation.label);
    TEST_ASSERT_EQUAL_INT(OBS_CLS_KIND_OWNER_CONFIRMED, r.annotation.kind);
    /* Precedence is checked before the revision CAS. */
    rerun.expected_revision = 0;
    TEST_ASSERT_EQUAL_INT(DB_OBS_CLS_OWNER_LOCKED, put(&obs, &rerun, &r));
    TEST_ASSERT_EQUAL_INT(4, get_record(&obs).revision);

    /* An identical owner retry is a no-op; another owner decision may replace it. */
    TEST_ASSERT_EQUAL_INT(DB_OBS_CLS_NOOP, put(&obs, &owner, &r));
    observation_classification_input_t again = owner;
    again.label = "Black-capped Chickadee";
    again.expected_revision = 2;
    TEST_ASSERT_EQUAL_INT(DB_OBS_CLS_UPDATED, put(&obs, &again, &r));
    TEST_ASSERT_EQUAL_INT(3, r.annotation.annotation_revision);
    TEST_ASSERT_EQUAL_INT(5, r.observation_revision);

    /* owner_confirmed may also create a key directly. */
    observation_classification_input_t direct = owner_input(&obs);
    direct.name = "owner-direct";
    direct.expected_revision = 0;
    TEST_ASSERT_EQUAL_INT(DB_OBS_CLS_CREATED, put(&obs, &direct, &r));
}

/* ---- evidence --------------------------------------------------------- */

static void test_evidence_mismatch_rejections(void) {
    fixture_obs_t obs = make_observation(OBS_ASSET_READY);
    observation_classification_input_t in = automated_input(&obs);
    observation_classification_result_t r;

    /* C3 06: another asset. */
    in.input_asset_uuid = "99999999-9999-4999-8999-999999999999";
    TEST_ASSERT_EQUAL_INT(DB_OBS_CLS_EVIDENCE_MISMATCH, put(&obs, &in, &r));
    TEST_ASSERT_EQUAL_STRING(obs.asset_uuid, r.observation_asset_uuid);
    TEST_ASSERT_EQUAL_INT(OBS_ASSET_READY, r.evidence_state);
    TEST_ASSERT_FALSE(r.has_annotation);

    /* input_revision beyond the current revision. */
    in = automated_input(&obs);
    in.input_revision = 3;
    TEST_ASSERT_EQUAL_INT(DB_OBS_CLS_EVIDENCE_MISMATCH, put(&obs, &in, &r));

    /* Asset UUIDs compare case-insensitively and are stored canonical. */
    char upper[OBS_UUID_SIZE];
    safe_strcpy(upper, obs.asset_uuid, sizeof(upper), 0);
    for (char *c = upper; *c; ++c) if (*c >= 'a' && *c <= 'f') *c = (char)(*c - 'a' + 'A');
    in = automated_input(&obs);
    in.input_asset_uuid = upper;
    TEST_ASSERT_EQUAL_INT(DB_OBS_CLS_CREATED, put(&obs, &in, &r));
    TEST_ASSERT_EQUAL_STRING(obs.asset_uuid, r.annotation.input_asset_uuid);

    /* C3 07: pending evidence. */
    fixture_obs_t pending = make_observation(OBS_ASSET_PENDING);
    in = automated_input(&pending);
    in.input_revision = 1;
    TEST_ASSERT_EQUAL_INT(DB_OBS_CLS_EVIDENCE_MISMATCH, put(&pending, &in, &r));
    TEST_ASSERT_EQUAL_STRING(pending.asset_uuid, r.observation_asset_uuid);
    TEST_ASSERT_EQUAL_INT(OBS_ASSET_PENDING, r.evidence_state);
    TEST_ASSERT_EQUAL_INT(0, annotation_rows(pending.observation_uuid));
    TEST_ASSERT_EQUAL_INT(1, get_record(&pending).revision);

    /* Unavailable evidence. */
    fixture_obs_t unavailable = make_observation(OBS_ASSET_UNAVAILABLE);
    in = automated_input(&unavailable);
    TEST_ASSERT_EQUAL_INT(DB_OBS_CLS_EVIDENCE_MISMATCH, put(&unavailable, &in, &r));
    TEST_ASSERT_EQUAL_INT(OBS_ASSET_UNAVAILABLE, r.evidence_state);

    /* Expired evidence still accepts a classification of the frame it was. */
    fixture_obs_t expired = make_observation(OBS_ASSET_EXPIRED);
    in = automated_input(&expired);
    in.input_revision = 3;
    TEST_ASSERT_EQUAL_INT(DB_OBS_CLS_CREATED, put(&expired, &in, &r));
    TEST_ASSERT_EQUAL_INT(4, r.observation_revision);
    char sql[256];
    snprintf(sql, sizeof(sql), "SELECT count(*) FROM observation_changes WHERE seq = %lld "
             "AND evidence_state = 'expired' AND evidence_reason = 'retention_expired'",
             (long long)r.change_seq);
    TEST_ASSERT_EQUAL_INT(1, scalar(sql));
}

/* ---- writer identity -------------------------------------------------- */

static void test_writer_mismatch_on_existing_key_is_rejected(void) {
    fixture_obs_t obs = make_observation(OBS_ASSET_READY);
    observation_classification_input_t in = automated_input(&obs);
    observation_classification_result_t r;
    TEST_ASSERT_EQUAL_INT(DB_OBS_CLS_CREATED, put(&obs, &in, &r));

    /* A later user that derives the same namespace cannot touch the key, even
     * with an identical body or as owner, and learns nothing about it. */
    observation_classification_input_t other = in;
    other.writer_user_id = OTHER_WRITER_ID;
    TEST_ASSERT_EQUAL_INT(DB_OBS_CLS_WRITER_MISMATCH, put(&obs, &other, &r));
    TEST_ASSERT_FALSE(r.has_annotation);
    other = owner_input(&obs);
    other.writer_user_id = OTHER_WRITER_ID;
    TEST_ASSERT_EQUAL_INT(DB_OBS_CLS_WRITER_MISMATCH, put(&obs, &other, &r));
    /* Writer check precedes the evidence check (contract step 2 before 4). */
    other.input_asset_uuid = "99999999-9999-4999-8999-999999999999";
    TEST_ASSERT_EQUAL_INT(DB_OBS_CLS_WRITER_MISMATCH, put(&obs, &other, &r));
    TEST_ASSERT_EQUAL_INT(3, get_record(&obs).revision);

    /* The other user may still create its own key. */
    other = automated_input(&obs);
    other.writer_user_id = OTHER_WRITER_ID;
    other.name = "species-2";
    TEST_ASSERT_EQUAL_INT(DB_OBS_CLS_CREATED, put(&obs, &other, &r));
    TEST_ASSERT_EQUAL_INT64(OTHER_WRITER_ID, r.annotation.writer_user_id);
}

/* ---- validation and limit --------------------------------------------- */

static void test_validation_rejects_out_of_contract_input(void) {
    fixture_obs_t obs = make_observation(OBS_ASSET_READY);
    observation_classification_result_t r;
    char long_label[OBS_CLS_LABEL_SIZE + 1];
    memset(long_label, 'x', sizeof(long_label) - 1);
    long_label[sizeof(long_label) - 1] = '\0';
    char long_segment[OBS_CLS_SEGMENT_MAX + 2];
    memset(long_segment, 'a', sizeof(long_segment) - 1);
    long_segment[sizeof(long_segment) - 1] = '\0';
    char long_text[66];
    memset(long_text, 'm', sizeof(long_text) - 1);
    long_text[sizeof(long_text) - 1] = '\0';

    observation_classification_input_t in;
#define REJECT(mutation) do { \
        in = automated_input(&obs); mutation; \
        TEST_ASSERT_EQUAL_INT_MESSAGE(DB_OBS_CLS_INVALID, put(&obs, &in, &r), #mutation); \
    } while (0)
    REJECT(in.name = "");
    REJECT(in.name = "Species");
    REJECT(in.name = "-species");
    REJECT(in.name = "spe/cies");
    REJECT(in.name = long_segment);
    REJECT(in.writer_namespace = "YAWAMF");
    REJECT(in.writer_namespace = NULL);
    REJECT(in.writer_namespace = long_segment);
    REJECT(in.writer_user_id = 0);
    REJECT(in.classifier = "");
    REJECT(in.classifier = long_text);
    REJECT(in.model_version = long_text);
    REJECT(in.taxonomy_id = long_text);
    REJECT(in.label = "");
    REJECT(in.label = NULL);
    REJECT(in.label = long_label);
    REJECT(in.confidence = -0.01);
    REJECT(in.confidence = 1.01);
    REJECT(in.confidence = NAN);
    REJECT(in.classified_at_ms = -1);
    REJECT(in.input_asset_uuid = "not-a-uuid");
    REJECT(in.input_revision = 0);
    REJECT(in.kind = (observation_classification_kind_t)7);
    REJECT(in.expected_revision = -1);
#undef REJECT
    TEST_ASSERT_EQUAL_INT(DB_OBS_CLS_INVALID,
        db_observation_classification_put("nope", &in, &r));
    TEST_ASSERT_EQUAL_INT(DB_OBS_CLS_INVALID,
        db_observation_classification_put(obs.observation_uuid, NULL, &r));
    TEST_ASSERT_EQUAL_INT(0, annotation_rows(obs.observation_uuid));

    /* Unknown observation. */
    in = automated_input(&obs);
    TEST_ASSERT_EQUAL_INT(DB_OBS_CLS_NOT_FOUND, db_observation_classification_put(
        "12345678-1234-4234-8234-123456789abc", &in, &r));

    /* Boundaries are accepted: 128-byte label, 32-byte segments, 0 and 1. */
    long_label[OBS_CLS_LABEL_SIZE - 1] = '\0';
    long_segment[OBS_CLS_SEGMENT_MAX] = '\0';
    in = automated_input(&obs);
    in.label = long_label;
    in.name = long_segment;
    in.confidence = 0.0;
    TEST_ASSERT_EQUAL_INT(DB_OBS_CLS_CREATED, put(&obs, &in, &r));
    TEST_ASSERT_EQUAL_INT(128, (int)strlen(r.annotation.label));
    TEST_ASSERT_EQUAL_INT(7 + OBS_CLS_SEGMENT_MAX, (int)strlen(r.annotation.key));

    TEST_ASSERT_TRUE(observation_classification_segment_is_valid("a"));
    TEST_ASSERT_TRUE(observation_classification_segment_is_valid("0_a.b-c"));
    TEST_ASSERT_FALSE(observation_classification_segment_is_valid("_a"));
    TEST_ASSERT_FALSE(observation_classification_segment_is_valid(NULL));
    observation_classification_kind_t kind;
    TEST_ASSERT_EQUAL_INT(0, observation_classification_kind_parse("owner_confirmed", &kind));
    TEST_ASSERT_EQUAL_INT(OBS_CLS_KIND_OWNER_CONFIRMED, kind);
    TEST_ASSERT_EQUAL_INT(-1, observation_classification_kind_parse("owner", &kind));
    TEST_ASSERT_EQUAL_STRING("automated",
                             observation_classification_kind_name(OBS_CLS_KIND_AUTOMATED));
}

static void test_ninth_key_hits_the_annotation_limit(void) {
    fixture_obs_t obs = make_observation(OBS_ASSET_READY);
    observation_classification_result_t r;
    char names[OBS_CLS_MAX_PER_OBSERVATION + 1][16];
    for (int i = 0; i <= OBS_CLS_MAX_PER_OBSERVATION; ++i) {
        snprintf(names[i], sizeof(names[i]), "k%d", i);
        observation_classification_input_t in = automated_input(&obs);
        in.name = names[i];
        int expected = i < OBS_CLS_MAX_PER_OBSERVATION ? DB_OBS_CLS_CREATED : DB_OBS_CLS_LIMIT;
        TEST_ASSERT_EQUAL_INT(expected, put(&obs, &in, &r));
    }
    TEST_ASSERT_EQUAL_INT(OBS_CLS_MAX_PER_OBSERVATION, r.limit);
    TEST_ASSERT_EQUAL_INT(OBS_CLS_MAX_PER_OBSERVATION, annotation_rows(obs.observation_uuid));
    TEST_ASSERT_EQUAL_INT(2 + OBS_CLS_MAX_PER_OBSERVATION, get_record(&obs).revision);

    /* Updating an existing key at the limit still works. */
    observation_classification_input_t in = automated_input(&obs);
    in.name = names[0];
    in.label = "Tufted Titmouse";
    in.expected_revision = 1;
    TEST_ASSERT_EQUAL_INT(DB_OBS_CLS_UPDATED, put(&obs, &in, &r));
}

/* ---- atomicity -------------------------------------------------------- */

static void test_publication_failure_rolls_back_the_annotation(void) {
    fixture_obs_t obs = make_observation(OBS_ASSET_READY);
    int changes = scalar("SELECT count(*) FROM observation_changes");
    exec_sql("CREATE TEMP TRIGGER fail_publication BEFORE INSERT ON observation_publications "
             "BEGIN SELECT RAISE(ABORT, 'injected'); END;");
    observation_classification_input_t in = automated_input(&obs);
    observation_classification_result_t r;
    TEST_ASSERT_EQUAL_INT(DB_OBS_CLS_ERROR, put(&obs, &in, &r));
    exec_sql("DROP TRIGGER temp.fail_publication;");
    TEST_ASSERT_EQUAL_INT(0, annotation_rows(obs.observation_uuid));
    TEST_ASSERT_EQUAL_INT(2, get_record(&obs).revision);
    TEST_ASSERT_EQUAL_INT(changes, scalar("SELECT count(*) FROM observation_changes"));
    /* The same request then succeeds as a create. */
    TEST_ASSERT_EQUAL_INT(DB_OBS_CLS_CREATED, put(&obs, &in, &r));
    TEST_ASSERT_EQUAL_INT(3, r.observation_revision);
}

/* ---- reads ------------------------------------------------------------ */

static void test_list_separates_automated_and_owner_by_updated_at(void) {
    fixture_obs_t obs = make_observation(OBS_ASSET_READY);
    observation_classification_set_t set;
    memset(&set, 0x5a, sizeof(set));
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_classifications_list(obs.observation_uuid, &set));
    TEST_ASSERT_EQUAL_INT(0, set.automated_count);
    TEST_ASSERT_EQUAL_INT(0, set.owner_confirmed_count);

    const char *names[] = {"a1", "o1", "a2", "o2", "a3"};
    observation_classification_result_t r;
    for (size_t i = 0; i < 5; ++i) {
        observation_classification_input_t in =
            names[i][0] == 'o' ? owner_input(&obs) : automated_input(&obs);
        in.name = names[i];
        in.expected_revision = 0;
        TEST_ASSERT_EQUAL_INT(DB_OBS_CLS_CREATED, put(&obs, &in, &r));
    }
    /* Deterministic updated_at ordering for the assertion. */
    exec_sql("UPDATE observation_classifications SET updated_at_ms = CASE key "
             "WHEN 'yawamf/a1' THEN 300 WHEN 'yawamf/a2' THEN 100 WHEN 'yawamf/a3' THEN 200 "
             "WHEN 'yawamf/o1' THEN 100 WHEN 'yawamf/o2' THEN 900 END;");
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_classifications_list(obs.observation_uuid, &set));
    TEST_ASSERT_EQUAL_INT(3, set.automated_count);
    TEST_ASSERT_EQUAL_STRING("yawamf/a1", set.automated[0].key);
    TEST_ASSERT_EQUAL_STRING("yawamf/a3", set.automated[1].key);
    TEST_ASSERT_EQUAL_STRING("yawamf/a2", set.automated[2].key);
    TEST_ASSERT_EQUAL_INT(2, set.owner_confirmed_count);
    TEST_ASSERT_EQUAL_STRING("yawamf/o2", set.owner_confirmed[0].key);
    TEST_ASSERT_EQUAL_STRING("yawamf/o1", set.owner_confirmed[1].key);
    TEST_ASSERT_EQUAL_INT(OBS_CLS_KIND_OWNER_CONFIRMED, set.owner_confirmed[0].kind);
    TEST_ASSERT_EQUAL_STRING("Carolina Chickadee", set.owner_confirmed[0].label);

    /* An owner decision moves a key between lists; each key is in exactly one. */
    observation_classification_input_t promote = owner_input(&obs);
    promote.name = "a1";
    promote.expected_revision = 1;
    TEST_ASSERT_EQUAL_INT(DB_OBS_CLS_UPDATED, put(&obs, &promote, &r));
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_classifications_list(obs.observation_uuid, &set));
    TEST_ASSERT_EQUAL_INT(2, set.automated_count);
    TEST_ASSERT_EQUAL_INT(3, set.owner_confirmed_count);
    TEST_ASSERT_EQUAL_STRING("yawamf/a1", set.owner_confirmed[0].key);

    TEST_ASSERT_EQUAL_INT(DB_OBS_INVALID, db_observation_classifications_list("bad", &set));
    TEST_ASSERT_EQUAL_INT(DB_OBS_INVALID, db_observation_classifications_list(obs.observation_uuid, NULL));

    /* Observation deletion (history pruning) removes its annotations. */
    char sql[200];
    snprintf(sql, sizeof(sql), "DELETE FROM object_observations WHERE uuid = '%s';",
             obs.observation_uuid);
    exec_sql(sql);
    TEST_ASSERT_EQUAL_INT(0, annotation_rows(obs.observation_uuid));
}

/* ---- authorization action and migration ------------------------------- */

static void test_annotate_action_is_bit_19_and_administrator_only(void) {
    TEST_ASSERT_EQUAL_INT(19, (int)AUTHZ_OBSERVATIONS_ANNOTATE);
    TEST_ASSERT_EQUAL_INT(AUTHZ_OBSERVATIONS_ANNOTATE,
                          authorization_action_from_key("observations.annotate"));
    const authorization_action_metadata_t *meta =
        authorization_action_metadata(AUTHZ_OBSERVATIONS_ANNOTATE);
    TEST_ASSERT_NOT_NULL(meta);
    TEST_ASSERT_EQUAL_STRING("Integrations", meta->category);
    TEST_ASSERT_TRUE(meta->camera_scoped);
    TEST_ASSERT_FALSE(meta->destructive);

    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM authz_actions WHERE "
        "action_key = 'observations.annotate' AND bit_index = 19 AND sort_order = 200 "
        "AND camera_scoped = 1 AND destructive = 0 AND category = 'Integrations'"));
    TEST_ASSERT_EQUAL_INT(19, scalar("SELECT max(bit_index) FROM authz_actions"));
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT count(*) FROM (SELECT bit_index FROM authz_actions "
                                    "GROUP BY bit_index HAVING count(*) > 1)"));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM authz_role_actions "
                                    "WHERE action_key = 'observations.annotate'"));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM authz_role_actions WHERE "
        "action_key = 'observations.annotate' AND role_uuid = '" ADMIN_ROLE "'"));
}

static sqlite3 *mem_db;
static sqlite_migrate_t *mem_migration;

static int mem_scalar(const char *sql) {
    sqlite3_stmt *stmt = NULL;
    int value = -999;
    if (sqlite3_prepare_v2(mem_db, sql, -1, &stmt, NULL) == SQLITE_OK &&
        sqlite3_step(stmt) == SQLITE_ROW)
        value = sqlite3_column_int(stmt, 0);
    sqlite3_finalize(stmt);
    TEST_ASSERT_NOT_EQUAL_INT_MESSAGE(-999, value, sql);
    return value;
}

static char *mem_text(const char *sql) {
    sqlite3_stmt *stmt = NULL;
    TEST_ASSERT_EQUAL_INT_MESSAGE(SQLITE_OK, sqlite3_prepare_v2(mem_db, sql, -1, &stmt, NULL), sql);
    TEST_ASSERT_EQUAL_INT_MESSAGE(SQLITE_ROW, sqlite3_step(stmt), sql);
    const unsigned char *value = sqlite3_column_text(stmt, 0);
    char *copy = strdup(value ? (const char *)value : "");
    sqlite3_finalize(stmt);
    TEST_ASSERT_NOT_NULL(copy);
    return copy;
}

static void mem_exec(const char *sql, int expected) {
    int rc = sqlite3_exec(mem_db, sql, NULL, NULL, NULL);
    TEST_ASSERT_EQUAL_INT_MESSAGE(expected, rc & 0xff, sql);
}

static size_t index_of(const char *version) {
    for (size_t i = 0; i < EMBEDDED_MIGRATIONS_COUNT; ++i)
        if (strcmp(embedded_migrations_data[i].version, version) == 0) return i;
    TEST_FAIL_MESSAGE(version);
    return 0;
}

static void mem_migrate_first(size_t count) {
    migrate_free(mem_migration);
    migrate_config_t config = {.embedded_migrations = embedded_migrations_data,
                               .embedded_count = count};
    mem_migration = migrate_init(mem_db, &config);
    TEST_ASSERT_NOT_NULL(mem_migration);
    TEST_ASSERT_EQUAL_INT(0, migrate_up(mem_migration, NULL));
}

static char *mem_schema(void) {
    return mem_text("SELECT coalesce(group_concat(type || ' ' || name || ' ' || coalesce(sql, ''), "
                    "char(10)), '') FROM (SELECT * FROM sqlite_master WHERE name NOT LIKE 'sqlite_%' "
                    "AND name <> 'schema_migrations' ORDER BY type, name)");
}

static char *mem_authz(void) {
    return mem_text("SELECT coalesce(group_concat(line, ','), '') FROM (SELECT action_key || ':' || "
                    "bit_index AS line FROM authz_actions ORDER BY bit_index) UNION ALL "
                    "SELECT coalesce(group_concat(role_uuid || ':' || action_key, ','), '') FROM "
                    "(SELECT * FROM authz_role_actions ORDER BY role_uuid, action_key)");
}

static void test_migration_0096_up_down_up(void) {
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_open(":memory:", &mem_db));
    mem_exec("PRAGMA foreign_keys = ON;", SQLITE_OK);
    size_t through_0095 = index_of("0095") + 1;
    size_t through_0096 = index_of("0096") + 1;
    TEST_ASSERT_EQUAL_size_t(through_0095 + 1, through_0096);

    mem_migrate_first(through_0095);
    /* Bit 19 must still be free before 0096 (no later migration took it). */
    TEST_ASSERT_EQUAL_INT(18, mem_scalar("SELECT max(bit_index) FROM authz_actions"));
    TEST_ASSERT_EQUAL_INT(0, mem_scalar("SELECT count(*) FROM authz_actions WHERE bit_index = 19"));
    char *before = mem_schema();
    char *before_authz = mem_authz();

    mem_migrate_first(through_0096);
    char *upgraded = mem_schema();
    TEST_ASSERT_TRUE(strcmp(before, upgraded) != 0);
    TEST_ASSERT_EQUAL_INT(1, mem_scalar("SELECT count(*) FROM sqlite_master WHERE type = 'table' "
                                        "AND name = 'observation_classifications'"));
    TEST_ASSERT_EQUAL_INT(0, mem_scalar("SELECT count(*) FROM (SELECT bit_index FROM authz_actions "
                                        "GROUP BY bit_index HAVING count(*) > 1)"));
    TEST_ASSERT_EQUAL_INT(19, mem_scalar("SELECT bit_index FROM authz_actions "
                                         "WHERE action_key = 'observations.annotate'"));
    /* The unique index rejects any second action at bit 19. */
    mem_exec("INSERT INTO authz_actions (action_key, category, description, camera_scoped, "
             "destructive, sort_order, bit_index) VALUES ('x.y', 'X', 'x', 1, 0, 210, 19);",
             SQLITE_CONSTRAINT);

    /* Constraints on a seeded row. */
    mem_exec("INSERT INTO observation_assets (uuid, camera_uuid, state, created_at_ms, updated_at_ms) "
             "VALUES ('88888888-8888-4888-8888-888888888888', 'c', 'ready', 1, 1);", SQLITE_OK);
    mem_exec("INSERT INTO object_observations (uuid, camera_uuid, stream_name_at_capture, batch_id, "
             "object_index, observed_at_ms, created_at_ms, engine_key, engine_type, label, confidence, "
             "asset_uuid, revision) VALUES ('55555555-5555-4555-8555-555555555555', 'c', 's', 'b', 0, 1, "
             "1, 'e', 'object', 'bird', 0.8, '88888888-8888-4888-8888-888888888888', 2);", SQLITE_OK);
#define CLS_ROW(key, ns, kind, conf) \
    "INSERT INTO observation_classifications (observation_uuid, key, writer_namespace, " \
    "writer_user_id, classifier, label, confidence, classified_at_ms, input_asset_uuid, " \
    "input_revision, kind, annotation_revision, created_at_ms, updated_at_ms) VALUES " \
    "('55555555-5555-4555-8555-555555555555', '" key "', '" ns "', 1, 'yawamf', 'Chickadee', " \
    conf ", 1, '88888888-8888-4888-8888-888888888888', 2, '" kind "', 1, 1, 1);"
    mem_exec(CLS_ROW("yawamf/species", "yawamf", "automated", "0.9"), SQLITE_OK);
    mem_exec(CLS_ROW("yawamf/species", "yawamf", "automated", "0.9"), SQLITE_CONSTRAINT);
    mem_exec(CLS_ROW("other/species", "yawamf", "automated", "0.9"), SQLITE_CONSTRAINT);
    mem_exec(CLS_ROW("yawamf/s2", "yawamf", "owner", "0.9"), SQLITE_CONSTRAINT);
    mem_exec(CLS_ROW("yawamf/s3", "yawamf", "automated", "1.5"), SQLITE_CONSTRAINT);
    mem_exec(CLS_ROW("yawamf/s4", "yawamf", "owner_confirmed", "1.0"), SQLITE_OK);
#undef CLS_ROW

    /* Down restores the 0095 schema and authz catalog, dropping annotations. */
    TEST_ASSERT_EQUAL_INT(1, migrate_down(mem_migration));
    char *after_down = mem_schema();
    char *after_down_authz = mem_authz();
    TEST_ASSERT_EQUAL_STRING(before, after_down);
    TEST_ASSERT_EQUAL_STRING(before_authz, after_down_authz);

    /* Up again reaches the same schema and catalog. */
    mem_migrate_first(through_0096);
    char *reapplied = mem_schema();
    TEST_ASSERT_EQUAL_STRING(upgraded, reapplied);
    TEST_ASSERT_EQUAL_INT(0, mem_scalar("SELECT count(*) FROM observation_classifications"));
    TEST_ASSERT_EQUAL_INT(1, mem_scalar("SELECT count(*) FROM authz_role_actions "
                                        "WHERE action_key = 'observations.annotate'"));

    free(before); free(before_authz); free(upgraded); free(after_down);
    free(after_down_authz); free(reapplied);
    migrate_free(mem_migration); mem_migration = NULL;
    sqlite3_close(mem_db); mem_db = NULL;
}

int main(void) {
    snprintf(db_path, sizeof(db_path), "/tmp/lightnvr_obs_classifications_%ld.db", (long)getpid());
    unlink(db_path);
    if (init_database(db_path) != 0) return 1;
    if (event_identity_init() != 0) return 1;

    UNITY_BEGIN();
    make_camera("cls_cam", cam);
    RUN_TEST(test_create_automated_bumps_revision_and_stages_publication);
    RUN_TEST(test_identical_retry_is_noop_regardless_of_expected_revision);
    RUN_TEST(test_expected_revision_compare_and_set);
    RUN_TEST(test_owner_confirmed_replaces_automated_and_locks_it);
    RUN_TEST(test_evidence_mismatch_rejections);
    RUN_TEST(test_writer_mismatch_on_existing_key_is_rejected);
    RUN_TEST(test_validation_rejects_out_of_contract_input);
    RUN_TEST(test_ninth_key_hits_the_annotation_limit);
    RUN_TEST(test_publication_failure_rolls_back_the_annotation);
    RUN_TEST(test_list_separates_automated_and_owner_by_updated_at);
    RUN_TEST(test_annotate_action_is_bit_19_and_administrator_only);
    RUN_TEST(test_migration_0096_up_down_up);
    int result = UNITY_END();

    event_identity_shutdown();
    shutdown_database();
    unlink(db_path);
    char sidecar[300];
    snprintf(sidecar, sizeof(sidecar), "%s-wal", db_path); unlink(sidecar);
    snprintf(sidecar, sizeof(sidecar), "%s-shm", db_path); unlink(sidecar);
    return result;
}
