/* Migration 0095 (INT_01 L1): object observations, evidence assets, change log,
 * publications and the cursor epoch row. Exercises the shipped embedded SQL
 * through the real runner on a fresh database, including rollback. */
#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sqlite3.h>
#include "unity.h"
#include "database/sqlite_migrate.h"
#include "database/db_embedded_migrations.h"

#define CAM "22222222-2222-4222-8222-222222222222"
#define ASSET "88888888-8888-4888-8888-888888888888"
#define OBS "55555555-5555-4555-8555-555555555555"
#define BATCH "77777777-7777-4777-8777-777777777777"

static sqlite3 *db;
static sqlite_migrate_t *migration;

static int scalar(const char *sql) {
    sqlite3_stmt *statement = NULL;
    int prepared = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    int stepped = prepared == SQLITE_OK ? sqlite3_step(statement) : prepared;
    int value = stepped == SQLITE_ROW ? sqlite3_column_int(statement, 0) : -1;
    if (statement) sqlite3_finalize(statement);
    TEST_ASSERT_EQUAL_INT_MESSAGE(SQLITE_OK, prepared, sql);
    TEST_ASSERT_EQUAL_INT_MESSAGE(SQLITE_ROW, stepped, sql);
    return value;
}
static char *text(const char *sql) {
    sqlite3_stmt *statement = NULL;
    TEST_ASSERT_EQUAL_INT_MESSAGE(SQLITE_OK, sqlite3_prepare_v2(db, sql, -1, &statement, NULL), sql);
    TEST_ASSERT_EQUAL_INT_MESSAGE(SQLITE_ROW, sqlite3_step(statement), sql);
    const unsigned char *value = sqlite3_column_text(statement, 0);
    char *copy = strdup(value ? (const char *)value : "");
    sqlite3_finalize(statement);
    TEST_ASSERT_NOT_NULL(copy);
    return copy;
}
static void ok(const char *sql) {
    char *error = NULL;
    int rc = sqlite3_exec(db, sql, NULL, NULL, &error);
    if (rc != SQLITE_OK) fprintf(stderr, "unexpected failure: %s\n  %s\n", error ? error : "", sql);
    sqlite3_free(error);
    TEST_ASSERT_EQUAL_INT_MESSAGE(SQLITE_OK, rc, sql);
}
static void rejected(const char *sql) {
    int rc = sqlite3_exec(db, sql, NULL, NULL, NULL);
    TEST_ASSERT_EQUAL_INT_MESSAGE(SQLITE_CONSTRAINT, rc & 0xff, sql);
}
static const migration_t *find_migration(const char *version) {
    for (size_t i = 0; i < EMBEDDED_MIGRATIONS_COUNT; ++i)
        if (strcmp(embedded_migrations_data[i].version, version) == 0) return &embedded_migrations_data[i];
    return NULL;
}
static size_t index_of(const char *version) {
    for (size_t i = 0; i < EMBEDDED_MIGRATIONS_COUNT; ++i)
        if (strcmp(embedded_migrations_data[i].version, version) == 0) return i;
    TEST_FAIL_MESSAGE(version);
    return 0;
}
static void migrate_first(size_t count) {
    migrate_free(migration);
    migrate_config_t config = {.embedded_migrations = embedded_migrations_data, .embedded_count = count};
    migration = migrate_init(db, &config);
    TEST_ASSERT_NOT_NULL(migration);
    TEST_ASSERT_EQUAL_INT(0, migrate_up(migration, NULL));
}
static void migrate_through(const char *version) { migrate_first(index_of(version) + 1); }

/* Structural snapshot: every table's columns plus the full SQL of every other
 * schema object. Table SQL text is compared separately because DROP COLUMN
 * rewrites CREATE TABLE text. */
static char *structure(void) {
    return text(
        "SELECT coalesce(group_concat(line, char(10)), '') FROM ("
        " SELECT m.type || ' ' || m.name || ' ' || p.cid || ' ' || p.name || ' ' || p.type || ' ' ||"
        "   p.\"notnull\" || ' ' || coalesce(p.dflt_value, '-') || ' ' || p.pk AS line, m.type AS t, m.name AS n, p.cid AS c"
        " FROM sqlite_master m JOIN pragma_table_info(m.name) p"
        " WHERE m.type = 'table' AND m.name NOT LIKE 'sqlite_%' AND m.name <> 'schema_migrations'"
        " UNION ALL"
        " SELECT type || ' ' || name || ' ' || coalesce(sql, ''), type, name, 0 FROM sqlite_master"
        " WHERE type <> 'table' AND name NOT LIKE 'sqlite_%'"
        " ORDER BY t, n, c)");
}
static char *full_sql(void) {
    return text("SELECT coalesce(group_concat(type || ' ' || name || ' ' || coalesce(sql, ''), char(10)), '') FROM ("
                "SELECT * FROM sqlite_master WHERE name NOT LIKE 'sqlite_%' AND name <> 'schema_migrations' ORDER BY type, name)");
}
static char *settings(void) {
    return text("SELECT coalesce(group_concat(key || '=' || value, ','), '') FROM (SELECT * FROM system_settings ORDER BY key)");
}

void setUp(void) {
    migration = NULL;
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_open(":memory:", &db));
    ok("PRAGMA foreign_keys = ON;");
}
void tearDown(void) {
    migrate_free(migration); migration = NULL;
    sqlite3_close(db); db = NULL;
}

static void seed_valid_rows(void) {
    ok("INSERT INTO observation_assets (uuid, camera_uuid, state, reason, created_at_ms, updated_at_ms)"
       " VALUES ('" ASSET "', '" CAM "', 'pending', NULL, 1000, 1000);");
    ok("INSERT INTO object_observations (uuid, camera_uuid, stream_name_at_capture, batch_id, object_index,"
       " observed_at_ms, created_at_ms, engine_key, engine_type, model, label, confidence,"
       " box_x, box_y, box_width, box_height, track_id, track_namespace, track_session, asset_uuid, revision)"
       " VALUES ('" OBS "', '" CAM "', 'feeder-east', '" BATCH "', 0, 990, 1000, 'legacy-primary', 'object',"
       " 'ssd_mobilenet_v2.sod', 'bird', 0.87, 0.412, 0.288, 0.12, 0.165, '17', 'legacy-primary',"
       " '9b0c7d1e-5f43-4b8e-9d0e-0b6c1f2a3d4e', '" ASSET "', 1);");
    ok("INSERT INTO observation_changes (observation_uuid, revision, cause, evidence_state, evidence_reason, created_at_ms)"
       " VALUES ('" OBS "', 1, 'created', 'pending', NULL, 1000);");
    ok("INSERT INTO observation_publications (change_seq, message_id, state, attempts, updated_at_ms)"
       " SELECT seq, '43ebdf02-b2d5-5403-a0ef-2a4e7d58ada9', 'pending', 0, 1000 FROM observation_changes;");
}

static void test_fresh_database_has_observation_schema(void) {
    migrate_first(EMBEDDED_MIGRATIONS_COUNT);
    TEST_ASSERT_NOT_NULL_MESSAGE(find_migration("0095"), "0095 is not embedded");
    const char *tables[] = {"object_observations", "observation_assets", "observation_changes",
                            "observation_publications", "observation_meta"};
    for (size_t i = 0; i < sizeof(tables) / sizeof(tables[0]); ++i) {
        char sql[160];
        snprintf(sql, sizeof(sql), "SELECT count(*) FROM sqlite_master WHERE type='table' AND name='%s'", tables[i]);
        TEST_ASSERT_EQUAL_INT_MESSAGE(1, scalar(sql), tables[i]);
    }
    TEST_ASSERT_EQUAL_INT(2, scalar("SELECT count(*) FROM pragma_table_info('observation_changes')"
                                    " WHERE name IN ('evidence_state', 'evidence_reason')"));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM pragma_table_info('observation_assets') WHERE name='origin'"));
    /* AUTOINCREMENT: seq is never reused after deletion. */
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM sqlite_master WHERE name='observation_changes'"
                                    " AND sql LIKE '%AUTOINCREMENT%'"));
    const char *indexes[] = {"idx_object_observations_camera_time", "idx_object_observations_label_time",
                             "idx_object_observations_created", "idx_object_observations_asset",
                             "idx_object_observations_recording", "idx_observation_assets_state_camera",
                             "idx_observation_publications_pending"};
    for (size_t i = 0; i < sizeof(indexes) / sizeof(indexes[0]); ++i) {
        char sql[160];
        snprintf(sql, sizeof(sql), "SELECT count(*) FROM sqlite_master WHERE type='index' AND name='%s'", indexes[i]);
        TEST_ASSERT_EQUAL_INT_MESSAGE(1, scalar(sql), indexes[i]);
    }
    /* Single meta row with a lowercase UUIDv4 epoch and the initial prune boundary. */
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM observation_meta"));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT oldest_seq FROM observation_meta"));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT epoch_uuid GLOB '[0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f]-"
                                    "[0-9a-f][0-9a-f][0-9a-f][0-9a-f]-4[0-9a-f][0-9a-f][0-9a-f]-[89ab][0-9a-f][0-9a-f][0-9a-f]-"
                                    "[0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f]'"
                                    " FROM observation_meta"));
    rejected("INSERT INTO observation_meta (id, epoch_uuid, oldest_seq) VALUES (2, '66666666-6666-4666-8666-666666666666', 1);");
    /* Per-camera stream settings and global defaults (contract section 13). */
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM pragma_table_info('streams') WHERE name='observation_evidence_enabled' AND dflt_value='0' AND \"notnull\"=1"));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM pragma_table_info('streams') WHERE name='observation_labels' AND dflt_value=\"''\""));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM pragma_table_info('streams') WHERE name='observation_max_age_days' AND dflt_value='7'"));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM pragma_table_info('streams') WHERE name='observation_max_bytes' AND dflt_value='2147483648'"));
    TEST_ASSERT_EQUAL_INT(30, scalar("SELECT CAST(value AS INTEGER) FROM system_settings WHERE key='observation_history_retention_days'"));
    TEST_ASSERT_EQUAL_INT(33554432, scalar("SELECT CAST(value AS INTEGER) FROM system_settings WHERE key='observation_global_queue_bytes'"));
}

static void test_constraints_accept_contract_rows_and_reject_bad_rows(void) {
    migrate_first(EMBEDDED_MIGRATIONS_COUNT);
    seed_valid_rows();
    /* Batch retry is a conflict on (batch_id, object_index). */
    rejected("INSERT INTO object_observations (uuid, camera_uuid, stream_name_at_capture, batch_id, object_index,"
             " observed_at_ms, created_at_ms, engine_key, engine_type, label, confidence, asset_uuid)"
             " VALUES ('55555555-5555-4555-8555-000000000002', '" CAM "', 'feeder-east', '" BATCH "', 0, 990, 1000,"
             " 'legacy-primary', 'object', 'bird', 0.5, '" ASSET "');");
    TEST_ASSERT_EQUAL_INT(0, sqlite3_exec(db,
        "INSERT OR IGNORE INTO object_observations (uuid, camera_uuid, stream_name_at_capture, batch_id, object_index,"
        " observed_at_ms, created_at_ms, engine_key, engine_type, label, confidence, asset_uuid)"
        " VALUES ('55555555-5555-4555-8555-000000000002', '" CAM "', 'feeder-east', '" BATCH "', 0, 990, 1000,"
        " 'legacy-primary', 'object', 'bird', 0.5, '" ASSET "');", NULL, NULL, NULL));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM object_observations"));
    /* A sibling at the next index shares the asset; a box-less, track-less row is valid. */
    ok("INSERT INTO object_observations (uuid, camera_uuid, stream_name_at_capture, batch_id, object_index,"
       " observed_at_ms, created_at_ms, engine_key, engine_type, label, confidence, asset_uuid)"
       " VALUES ('55555555-5555-4555-8555-000000000003', '" CAM "', 'feeder-east', '" BATCH "', 1, 990, 1000,"
       " 'onvif-1', 'onvif', 'bird', 1.0, '" ASSET "');");
    TEST_ASSERT_EQUAL_INT(2, scalar("SELECT count(*) FROM object_observations WHERE asset_uuid='" ASSET "'"));

#define OBS_COLS "INSERT INTO object_observations (uuid, camera_uuid, stream_name_at_capture, batch_id, object_index," \
                 " observed_at_ms, created_at_ms, engine_key, engine_type, label, confidence, asset_uuid"
#define OBS_VALS " VALUES ('55555555-5555-4555-8555-0000000000ff', '" CAM "', 'feeder-east', '" BATCH "', 5, 990, 1000," \
                 " 'legacy-primary', "
    rejected(OBS_COLS ") " OBS_VALS "'object', 'bird', 1.01, '" ASSET "');");
    rejected(OBS_COLS ") " OBS_VALS "'object', 'bird', -0.01, '" ASSET "');");
    rejected(OBS_COLS ") " OBS_VALS "'object', 'bird', NULL, '" ASSET "');");
    rejected(OBS_COLS ") " OBS_VALS "'motion', 'bird', 0.5, '" ASSET "');");
    rejected(OBS_COLS ") " OBS_VALS "'object', '', 0.5, '" ASSET "');");
    rejected(OBS_COLS ") " OBS_VALS "'object', 'bird', 0.5, '99999999-9999-4999-8999-999999999999');");
    rejected(OBS_COLS ", revision) " OBS_VALS "'object', 'bird', 0.5, '" ASSET "', 0);");
    rejected(OBS_COLS ", box_x) " OBS_VALS "'object', 'bird', 0.5, '" ASSET "', 0.1);");
    rejected(OBS_COLS ", box_x, box_y, box_width, box_height) " OBS_VALS "'object', 'bird', 0.5, '" ASSET "', 0.5, 0.1, 0.6, 0.1);");
    rejected(OBS_COLS ", box_x, box_y, box_width, box_height) " OBS_VALS "'object', 'bird', 0.5, '" ASSET "', 0.1, 0.1, 0.0, 0.1);");
    rejected(OBS_COLS ", track_id) " OBS_VALS "'object', 'bird', 0.5, '" ASSET "', '17');");
    rejected("INSERT INTO object_observations (uuid, camera_uuid, stream_name_at_capture, batch_id, object_index,"
             " observed_at_ms, created_at_ms, engine_key, engine_type, label, confidence, asset_uuid)"
             " VALUES ('55555555-5555-4555-8555-0000000000fe', '" CAM "', 'x', '" BATCH "', 20, 990, 1000,"
             " 'legacy-primary', 'object', 'bird', 0.5, '" ASSET "');");

    /* Asset state machine values and reason pairing. */
#define ASSET_ROW(id, state, reason) "INSERT INTO observation_assets (uuid, camera_uuid, state, reason, created_at_ms," \
    " updated_at_ms) VALUES ('" id "', '" CAM "', " state ", " reason ", 1, 1);"
    rejected(ASSET_ROW("a1", "'deleted'", "NULL"));
    rejected(ASSET_ROW("a2", "'ready'", "'queue_full'"));
    rejected(ASSET_ROW("a3", "'unavailable'", "NULL"));
    rejected(ASSET_ROW("a4", "'expired'", "NULL"));
    rejected(ASSET_ROW("a5", "NULL", "NULL"));
    ok(ASSET_ROW("a6", "'unavailable'", "'queue_full'"));
    ok(ASSET_ROW("a7", "'expired'", "'retention_expired'"));
    rejected("UPDATE observation_assets SET protected = 2 WHERE uuid = 'a6';");
    rejected("UPDATE observation_assets SET origin = 'recording_frame' WHERE uuid = 'a6';");
    ok("UPDATE observation_assets SET origin = 'inference_input', state = 'ready', reason = NULL WHERE uuid = '" ASSET "';");
    /* Referenced assets cannot be deleted out from under observations. */
    rejected("DELETE FROM observation_assets WHERE uuid = '" ASSET "';");

    /* Change rows: cause and evidence-state CHECKs, unique revision per observation. */
#define CHANGE(rev, cause, state, reason) "INSERT INTO observation_changes (observation_uuid, revision, cause," \
    " evidence_state, evidence_reason, created_at_ms) VALUES ('" OBS "', " rev ", " cause ", " state ", " reason ", 2000);"
    rejected(CHANGE("1", "'evidence'", "'ready'", "NULL"));
    rejected(CHANGE("2", "'updated'", "'ready'", "NULL"));
    rejected(CHANGE("2", "'evidence'", "'gone'", "NULL"));
    rejected(CHANGE("2", "'evidence'", "'unavailable'", "NULL"));
    rejected(CHANGE("2", "'evidence'", "'ready'", "'write_failed'"));
    rejected(CHANGE("0", "'evidence'", "'ready'", "NULL"));
    rejected("INSERT INTO observation_changes (observation_uuid, revision, cause, evidence_state, created_at_ms)"
             " VALUES ('00000000-0000-4000-8000-00000000dead', 1, 'created', 'pending', 1);");
    ok(CHANGE("2", "'evidence'", "'ready'", "NULL"));
    ok(CHANGE("3", "'classification'", "'unavailable'", "'write_failed'"));

    /* Publications: the five D7 states only; one row per change; FK to the change. */
    const char *states[] = {"relayed", "relayed_no_route", "expired", "failed", "pending"};
    for (size_t i = 0; i < sizeof(states) / sizeof(states[0]); ++i) {
        char sql[160];
        snprintf(sql, sizeof(sql), "UPDATE observation_publications SET state = '%s';", states[i]);
        ok(sql);
    }
    rejected("UPDATE observation_publications SET state = 'delivered';");
    rejected("UPDATE observation_publications SET state = NULL;");
    rejected("UPDATE observation_publications SET attempts = -1;");
    rejected("INSERT INTO observation_publications (change_seq, message_id, state, attempts, updated_at_ms)"
             " SELECT seq, 'dup', 'pending', 0, 1 FROM observation_changes WHERE revision = 1;");
    rejected("INSERT INTO observation_publications (change_seq, message_id, updated_at_ms) VALUES (9999, 'm-x', 1);");
    rejected("INSERT INTO observation_publications (change_seq, message_id, updated_at_ms)"
             " SELECT seq, '43ebdf02-b2d5-5403-a0ef-2a4e7d58ada9', 1 FROM observation_changes WHERE revision = 2;");
    ok("INSERT INTO observation_publications (change_seq, message_id, updated_at_ms)"
       " SELECT seq, 'm-2', 1 FROM observation_changes WHERE revision = 2;");
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM observation_publications WHERE change_seq ="
                                    " (SELECT seq FROM observation_changes WHERE revision = 2) AND state = 'pending' AND attempts = 0"));

    /* Pruning changes cascades their publications; seq is never reused. */
    int max_seq = scalar("SELECT max(seq) FROM observation_changes");
    ok("DELETE FROM observation_changes WHERE revision >= 2;");
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM observation_publications"));
    ok(CHANGE("2", "'evidence'", "'ready'", "NULL"));
    TEST_ASSERT_EQUAL_INT(max_seq + 1, scalar("SELECT max(seq) FROM observation_changes"));
    /* Deleting an observation removes its change history. */
    ok("DELETE FROM object_observations WHERE uuid = '" OBS "';");
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT count(*) FROM observation_changes"));
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT count(*) FROM observation_publications"));
}

static void test_recording_delete_detaches_observation(void) {
    migrate_first(EMBEDDED_MIGRATIONS_COUNT);
    seed_valid_rows();
    ok("INSERT INTO recordings (id, stream_name, file_path, start_time) VALUES (81234, 'feeder-east', '/tmp/x.mp4', 1);");
    ok("UPDATE object_observations SET recording_id = 81234;");
    ok("DELETE FROM recordings WHERE id = 81234;");
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM object_observations WHERE recording_id IS NULL"));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM observation_assets"));
}

static void test_upgrade_down_and_reapply_reach_same_schema(void) {
    /* Schema and settings of a database at 0094. */
    migrate_through("0094");
    char *before = structure();
    char *before_settings = settings();
    /* Upgrade the 0094 database to 0095 (later migrations are tested on their own). */
    migrate_through("0095");
    char *upgraded = full_sql();
    char *upgraded_structure = structure();
    TEST_ASSERT_TRUE(strcmp(before, upgraded_structure) != 0);
    seed_valid_rows();
    /* Down restores the 0094 structure and settings, dropping observation data. */
    TEST_ASSERT_EQUAL_INT(1, migrate_down(migration));
    char *after_down = structure();
    char *after_down_settings = settings();
    TEST_ASSERT_EQUAL_STRING(before, after_down);
    TEST_ASSERT_EQUAL_STRING(before_settings, after_down_settings);
    /* Up again reaches the same schema as an upgrade, and the same as a fresh database. */
    migrate_through("0095");
    char *reapplied = full_sql();
    TEST_ASSERT_EQUAL_STRING(upgraded, reapplied);
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT count(*) FROM object_observations"));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM observation_meta"));

    sqlite3 *upgraded_db = db; db = NULL;
    sqlite_migrate_t *upgraded_migration = migration; migration = NULL;
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_open(":memory:", &db));
    ok("PRAGMA foreign_keys = ON;");
    migrate_through("0095");
    char *fresh = full_sql();
    TEST_ASSERT_EQUAL_STRING(upgraded, fresh);
    migrate_free(migration); sqlite3_close(db);
    migration = upgraded_migration; db = upgraded_db;

    free(before); free(before_settings); free(upgraded); free(upgraded_structure);
    free(after_down); free(after_down_settings); free(reapplied); free(fresh);
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_fresh_database_has_observation_schema);
    RUN_TEST(test_constraints_accept_contract_rows_and_reject_bad_rows);
    RUN_TEST(test_recording_delete_detaches_observation);
    RUN_TEST(test_upgrade_down_and_reapply_reach_same_schema);
    return UNITY_END();
}
