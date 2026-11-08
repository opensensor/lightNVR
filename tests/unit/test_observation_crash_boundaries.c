/* INT_01 L11 / A6: SIGKILL at every observation persistence boundary, then a
 * fresh process generation runs the main.c startup sequence (L4 reconcile ->
 * L3 evidence init -> L5 relay start/drain) against the same database and
 * storage tree.
 *
 * Each boundary forks a "generation 1" child that runs the real evidence
 * worker, the real relay and the real router/outbox on a migrated database,
 * submits one two-object batch and is killed with SIGKILL from a stage hook at
 * exactly one boundary. The parent first checks that the kill landed where it
 * was meant to (on-disk and DB state at the crash), then forks a
 * "generation 2" child that restarts the pipeline, and finally inspects the
 * database and tree directly with sqlite3 to assert:
 *   - no ready asset points at a missing, partial or temporary file, no
 *     pending asset survives, and no final file exists without a ready row;
 *   - every committed change has been relayed exactly once into the durable
 *     outbox under its deterministic message ID (no change lost, none
 *     duplicated), and an outbox row accepted before the crash is still
 *     pending delivery.
 *
 * The hooks (observation_evidence_set_stage_hook / observation_relay_set_
 * stage_hook) are only reachable from linked code, never from config, the
 * environment or the API, so production cannot trigger these kill points. */
#define _GNU_SOURCE
#include <dirent.h>
#include <errno.h>
#include <signal.h>
#include <sqlite3.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/wait.h>
#include <time.h>
#include <unistd.h>

#include "unity.h"
#include "core/config.h"
#include "core/event_identity.h"
#include "core/event_router.h"
#include "core/mqtt_delivery_worker.h"
#include "core/observation_evidence.h"
#include "core/observation_relay.h"
#include "core/observation_retention.h"
#include "database/db_core.h"
#include "database/db_event_routes.h"
#include "database/db_observations.h"
#include "database/db_streams.h"
#include "utils/strings.h"
#include "utils/uuid.h"

#define TOPIC_PREFIX "lightnvr-crash"
#define OBJECTS 2
#define EXIT_NOT_KILLED 77

static char root_dir[256];

/* The exact bytes the "detector" submitted. JPEG markers around a pattern so
 * any truncation or substitution is visible. */
static uint8_t submitted[4096];

typedef enum { HOOK_EVIDENCE, HOOK_RELAY } hook_kind_t;

typedef struct {
    const char *name;
    hook_kind_t kind;
    int stage;
    bool expect_ready;  /* asset outcome after restart */
} boundary_t;

static const boundary_t *g_boundary;  /* child side */

/* ---- generation helpers (run inside forked children) ------------------- */

#define CHILD_CHECK(cond, msg) do { if (!(cond)) { \
    fprintf(stderr, "[child %s] %s\n", g_boundary ? g_boundary->name : "?", msg); \
    _exit(2); } } while (0)

static void paths(const boundary_t *b, char *storage, size_t ssize, char *db, size_t dsize) {
    snprintf(storage, ssize, "%s/%s", root_dir, b->name);
    snprintf(db, dsize, "%s/%s.db", root_dir, b->name);
}

static int evidence_hook(observation_evidence_stage_t stage, const char *path, void *ctx) {
    (void)path;
    (void)ctx;
    if (g_boundary && g_boundary->kind == HOOK_EVIDENCE && (int)stage == g_boundary->stage)
        raise(SIGKILL);
    return 0;
}

static void relay_hook(observation_relay_stage_t stage, const observation_publication_t *row,
                       void *ctx) {
    (void)ctx;
    /* Only once the evidence change (pending -> ready) is committed, so the
     * kill sits strictly after the last evidence-side boundary. */
    if (g_boundary && g_boundary->kind == HOOK_RELAY && (int)stage == g_boundary->stage &&
        row->cause == OBS_CAUSE_EVIDENCE)
        raise(SIGKILL);
}

/* The main.c startup sequence (design decision 2). */
static void start_pipeline(const char *storage, const char *db) {
    CHILD_CHECK(init_database(db) == 0, "init_database");
    CHILD_CHECK(event_identity_init() == 0, "event_identity_init");
    g_config.mqtt_enabled = true;
    safe_strcpy(g_config.mqtt_topic_prefix, TOPIC_PREFIX, sizeof(g_config.mqtt_topic_prefix), 0);
    safe_strcpy(g_config.storage_path, storage, sizeof(g_config.storage_path), 0);

    observation_reconcile_result_t reconcile;
    CHILD_CHECK(observation_retention_startup(storage, &reconcile) == 0, "reconcile");
    observation_evidence_config_t evidence = {.storage_path = storage};
    CHILD_CHECK(observation_evidence_init(&evidence) == 0, "evidence init");
    observation_relay_options_t relay;
    memset(&relay, 0, sizeof(relay));
    relay.storage_path = storage;
    relay.poll_interval_ms = 50;
    CHILD_CHECK(observation_relay_start(&relay) == 0, "relay start");
}

static void stop_pipeline(void) {
    observation_relay_stop();
    observation_evidence_shutdown(OBS_EVIDENCE_DEFAULT_DRAIN_MS);
    event_router_shutdown();
    event_identity_shutdown();
    shutdown_database();
}

static void seed_camera_and_route(char camera_uuid[OBS_UUID_SIZE]) {
    stream_config_t *s = calloc(1, sizeof(*s));
    CHILD_CHECK(s != NULL, "calloc");
    safe_strcpy(s->name, "crash-cam", sizeof(s->name), 0);
    safe_strcpy(s->url, "rtsp://camera/stream", sizeof(s->url), 0);
    safe_strcpy(s->codec, "h264", sizeof(s->codec), 0);
    s->enabled = true;
    s->priority = 5;
    s->segment_duration = 60;
    s->observation_evidence_enabled = true;
    safe_strcpy(s->detection_object_filter, "none", sizeof(s->detection_object_filter), 0);
    CHILD_CHECK(add_stream_config(s) > 0, "add_stream_config");
    memset(s, 0, sizeof(*s));
    CHILD_CHECK(get_stream_config_by_name("crash-cam", s) == 0, "get_stream_config");
    safe_strcpy(camera_uuid, s->camera_uuid, OBS_UUID_SIZE, 0);
    free(s);

    event_route_t route;
    memset(&route, 0, sizeof(route));
    safe_strcpy(route.name, "Observation crash route", sizeof(route.name), 0);
    route.enabled = true;
    safe_strcpy(route.destination_key, EVENT_ROUTE_DEFAULT_DESTINATION,
                sizeof(route.destination_key), 0);
    safe_strcpy(route.scope_type, "all", sizeof(route.scope_type), 0);
    safe_strcpy(route.predicate_json, "{\"version\":1}", sizeof(route.predicate_json), 0);
    safe_strcpy(route.schedule_json, "{\"version\":1,\"timezone\":\"UTC\",\"windows\":[]}",
                sizeof(route.schedule_json), 0);
    safe_strcpy(route.event_types[0], OBSERVATION_CHANGED_EVENT_TYPE,
                sizeof(route.event_types[0]), 0);
    route.event_type_count = 1;
    CHILD_CHECK(db_event_route_create(&route) == DB_EVENT_ROUTE_OK, "route create");
}

static int64_t wall_ms(void) {
    struct timespec ts;
    clock_gettime(CLOCK_REALTIME, &ts);
    return (int64_t)ts.tv_sec * 1000 + ts.tv_nsec / 1000000;
}

/* Generation 1: submit one batch and wait to be killed at the boundary. */
static void generation_one(const boundary_t *b) {
    g_boundary = b;
    char storage[512], db[512];
    paths(b, storage, sizeof(storage), db, sizeof(db));
    start_pipeline(storage, db);
    char camera[OBS_UUID_SIZE];
    seed_camera_and_route(camera);
    observation_evidence_set_stage_hook(evidence_hook, NULL);
    observation_relay_set_stage_hook(relay_hook, NULL);

    observation_batch_input_t batch;
    memset(&batch, 0, sizeof(batch));
    CHILD_CHECK(lightnvr_uuid_generate_v4(batch.batch_id) == 0, "batch id");
    safe_strcpy(batch.camera_uuid, camera, sizeof(batch.camera_uuid), 0);
    batch.observed_at_ms = wall_ms();
    safe_strcpy(batch.engine_key, "legacy-primary", sizeof(batch.engine_key), 0);
    safe_strcpy(batch.engine_type, "api", sizeof(batch.engine_type), 0);
    batch.object_count = OBJECTS;
    for (int i = 0; i < OBJECTS; ++i) {
        batch.objects[i].object_index = i;
        safe_strcpy(batch.objects[i].label, "bird", sizeof(batch.objects[i].label), 0);
        batch.objects[i].confidence = 0.9;
    }
    observation_evidence_transform_t transform = {
        .mode = OBS_TRANSFORM_NONE, .source_width = 64, .source_height = 48,
        .inference_width = 64, .inference_height = 48, .scale_x = 1.0, .scale_y = 1.0,
    };
    uint8_t *jpeg = malloc(sizeof(submitted));
    CHILD_CHECK(jpeg != NULL, "malloc");
    memcpy(jpeg, submitted, sizeof(submitted));
    char asset[OBS_UUID_SIZE];
    CHILD_CHECK(observation_evidence_submit_jpeg(&batch, jpeg, sizeof(submitted), 64, 48,
                                                 &transform, asset) == OBS_EVIDENCE_ACCEPTED,
                "submit");
    for (int i = 0; i < 1000; ++i) usleep(10000);  /* 10 s for the hook to fire */
    fprintf(stderr, "[child %s] boundary never reached\n", b->name);
    _exit(EXIT_NOT_KILLED);
}

/* Generation 2: the next process start. Returns once nothing is pending. */
static void generation_two(const boundary_t *b) {
    g_boundary = NULL;
    char storage[512], db[512];
    paths(b, storage, sizeof(storage), db, sizeof(db));
    start_pipeline(storage, db);
    for (int i = 0; i < 500; ++i) {
        observation_publication_t rows[8];
        int pending = db_observation_pending_publications(rows, 8);
        CHILD_CHECK(pending >= 0, "pending read");
        if (pending == 0) break;
        usleep(10000);
    }
    stop_pipeline();
    _exit(0);
}

static int run_child(void (*fn)(const boundary_t *), const boundary_t *b) {
    fflush(stdout);
    fflush(stderr);
    pid_t pid = fork();
    TEST_ASSERT_TRUE_MESSAGE(pid >= 0, "fork");
    if (pid == 0) {
        fn(b);
        _exit(3);
    }
    int status = 0;
    TEST_ASSERT_EQUAL_INT(pid, waitpid(pid, &status, 0));
    return status;
}

/* ---- parent-side inspection (raw sqlite3, no lightNVR state) ----------- */

static sqlite3 *open_db(const boundary_t *b) {
    char storage[512], db[512];
    paths(b, storage, sizeof(storage), db, sizeof(db));
    sqlite3 *handle = NULL;
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_open_v2(db, &handle, SQLITE_OPEN_READONLY, NULL));
    sqlite3_busy_timeout(handle, 3000);
    return handle;
}

static int64_t q64(sqlite3 *db, const char *sql) {
    sqlite3_stmt *stmt = NULL;
    int64_t value = -999;
    if (sqlite3_prepare_v2(db, sql, -1, &stmt, NULL) == SQLITE_OK &&
        sqlite3_step(stmt) == SQLITE_ROW)
        value = sqlite3_column_int64(stmt, 0);
    sqlite3_finalize(stmt);
    TEST_ASSERT_NOT_EQUAL_INT64_MESSAGE(-999, value, sql);
    return value;
}

static int q(sqlite3 *db, const char *sql) { return (int)q64(db, sql); }

/* Count files under <storage>/observations whose name ends with suffix. */
static int count_files(const char *dir, const char *suffix) {
    DIR *d = opendir(dir);
    if (!d) return 0;
    int n = 0;
    struct dirent *entry;
    while ((entry = readdir(d)) != NULL) {
        if (strcmp(entry->d_name, ".") == 0 || strcmp(entry->d_name, "..") == 0) continue;
        char path[1024];
        snprintf(path, sizeof(path), "%s/%s", dir, entry->d_name);
        struct stat st;
        if (lstat(path, &st) != 0) continue;
        if (S_ISDIR(st.st_mode)) {
            n += count_files(path, suffix);
        } else {
            size_t len = strlen(entry->d_name), slen = strlen(suffix);
            if (len >= slen && strcmp(entry->d_name + len - slen, suffix) == 0) n++;
        }
    }
    closedir(d);
    return n;
}

static int evidence_files(const boundary_t *b, const char *suffix) {
    char dir[512];
    snprintf(dir, sizeof(dir), "%s/%s/observations", root_dir, b->name);
    return count_files(dir, suffix);
}

/* State at the crash proves the kill landed on the intended boundary. */
static void assert_crash_state(const boundary_t *b) {
    sqlite3 *db = open_db(b);
    int pending_assets = q(db, "SELECT count(*) FROM observation_assets WHERE state = 'pending'");
    int ready_assets = q(db, "SELECT count(*) FROM observation_assets WHERE state = 'ready'");
    TEST_ASSERT_EQUAL_INT_MESSAGE(OBJECTS, q(db, "SELECT count(*) FROM object_observations"),
                                  b->name);
    if (b->kind == HOOK_EVIDENCE) {
        TEST_ASSERT_EQUAL_INT_MESSAGE(1, pending_assets, b->name);
        TEST_ASSERT_EQUAL_INT(0, ready_assets);
        int tmp = evidence_files(b, ".jpg.tmp"), final = evidence_files(b, ".jpg");
        if (b->stage == OBS_EVIDENCE_STAGE_AFTER_INSERT) {
            TEST_ASSERT_EQUAL_INT(0, tmp);
            TEST_ASSERT_EQUAL_INT(0, final);
        } else if (b->stage == OBS_EVIDENCE_STAGE_BEFORE_RENAME) {
            TEST_ASSERT_EQUAL_INT_MESSAGE(1, tmp, "tmp file written and fsynced at the crash");
            TEST_ASSERT_EQUAL_INT(0, final);
        } else {
            TEST_ASSERT_EQUAL_INT(0, tmp);
            TEST_ASSERT_EQUAL_INT_MESSAGE(1, final, "final file renamed before the commit");
        }
        /* Only the created changes exist; nothing claims ready. */
        TEST_ASSERT_EQUAL_INT(OBJECTS, q(db, "SELECT count(*) FROM observation_changes"));
    } else {
        TEST_ASSERT_EQUAL_INT(0, pending_assets);
        TEST_ASSERT_EQUAL_INT(1, ready_assets);
        TEST_ASSERT_EQUAL_INT(2 * OBJECTS, q(db, "SELECT count(*) FROM observation_changes"));
        /* At least the killed evidence change is not fully through the relay. */
        if (b->stage == OBS_RELAY_STAGE_AFTER_MARK) {
            TEST_ASSERT_TRUE(q(db, "SELECT count(*) FROM observation_publications p JOIN "
                                   "observation_changes c ON c.seq = p.change_seq WHERE "
                                   "c.cause = 'evidence' AND p.state = 'relayed'") >= 1);
        } else {
            TEST_ASSERT_TRUE(q(db, "SELECT count(*) FROM observation_publications p JOIN "
                                   "observation_changes c ON c.seq = p.change_seq WHERE "
                                   "c.cause = 'evidence' AND p.state = 'pending'") >= 1);
        }
        int in_outbox = q(db, "SELECT count(*) FROM event_outbox e JOIN observation_publications p "
                              "ON p.message_id = e.event_id JOIN observation_changes c ON "
                              "c.seq = p.change_seq WHERE c.cause = 'evidence'");
        if (b->stage == OBS_RELAY_STAGE_BEFORE_ENQUEUE)
            TEST_ASSERT_TRUE_MESSAGE(in_outbox < OBJECTS, "killed before the outbox step");
        else
            TEST_ASSERT_TRUE_MESSAGE(in_outbox >= 1, "outbox accepted before the crash");
    }
    sqlite3_close(db);
}

static void read_file_exact(const char *path, uint8_t *out, size_t size) {
    FILE *f = fopen(path, "rb");
    TEST_ASSERT_NOT_NULL_MESSAGE(f, path);
    TEST_ASSERT_EQUAL_size_t(size, fread(out, 1, size, f));
    TEST_ASSERT_EQUAL_INT(EOF, fgetc(f));
    fclose(f);
}

static void assert_recovered(const boundary_t *b) {
    sqlite3 *db = open_db(b);
    /* No unresolved or unfinished evidence. */
    TEST_ASSERT_EQUAL_INT(0, q(db, "SELECT count(*) FROM observation_assets WHERE state = 'pending'"));
    TEST_ASSERT_EQUAL_INT(0, evidence_files(b, ".jpg.tmp"));
    int ready = q(db, "SELECT count(*) FROM observation_assets WHERE state = 'ready'");
    TEST_ASSERT_EQUAL_INT_MESSAGE(evidence_files(b, ".jpg"), ready,
                                  "every final file has a ready row and vice versa");
    if (b->expect_ready) {
        TEST_ASSERT_EQUAL_INT(1, ready);
        sqlite3_stmt *stmt = NULL;
        TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_prepare_v2(db,
            "SELECT path, bytes FROM observation_assets WHERE state = 'ready'", -1, &stmt, NULL));
        TEST_ASSERT_EQUAL_INT(SQLITE_ROW, sqlite3_step(stmt));
        char full[1024];
        snprintf(full, sizeof(full), "%s/%s/%s", root_dir, b->name,
                 (const char *)sqlite3_column_text(stmt, 0));
        TEST_ASSERT_EQUAL_INT64((int64_t)sizeof(submitted), sqlite3_column_int64(stmt, 1));
        sqlite3_finalize(stmt);
        static uint8_t stored[sizeof(submitted)];
        read_file_exact(full, stored, sizeof(stored));
        TEST_ASSERT_EQUAL_MEMORY_MESSAGE(submitted, stored, sizeof(submitted),
                                         "ready asset is the complete submitted image");
    } else {
        TEST_ASSERT_EQUAL_INT(0, ready);
        TEST_ASSERT_EQUAL_INT(1, q(db, "SELECT count(*) FROM observation_assets WHERE "
                                       "state = 'unavailable' AND reason = 'interrupted'"));
    }
    /* Every observation reached revision 2 (created + evidence outcome). */
    TEST_ASSERT_EQUAL_INT(OBJECTS, q(db, "SELECT count(*) FROM object_observations WHERE revision = 2"));
    int changes = q(db, "SELECT count(*) FROM observation_changes");
    TEST_ASSERT_EQUAL_INT(2 * OBJECTS, changes);

    /* Every committed change was relayed, exactly once, under its message ID. */
    TEST_ASSERT_EQUAL_INT(changes, q(db, "SELECT count(*) FROM observation_publications WHERE "
                                         "state = 'relayed'"));
    TEST_ASSERT_EQUAL_INT(changes, q(db, "SELECT count(*) FROM event_outbox WHERE event_type = '"
                                         OBSERVATION_CHANGED_EVENT_TYPE "' AND destination = '"
                                         MQTT_EVENT_OUTBOX_DESTINATION "'"));
    TEST_ASSERT_EQUAL_INT(changes, q(db, "SELECT count(DISTINCT e.event_id) FROM event_outbox e "
                                         "JOIN observation_publications p ON p.message_id = e.event_id"));
    /* No broker in this test: the accepted rows are still deliverable. */
    TEST_ASSERT_EQUAL_INT(changes, q(db, "SELECT count(*) FROM event_outbox WHERE state = 'pending'"));
    sqlite3_close(db);
}

static void run_boundary(const boundary_t *b) {
    int status = run_child(generation_one, b);
    if (WIFEXITED(status))
        TEST_FAIL_MESSAGE(WEXITSTATUS(status) == EXIT_NOT_KILLED ? "boundary never reached"
                                                                 : "generation 1 failed");
    TEST_ASSERT_TRUE(WIFSIGNALED(status));
    TEST_ASSERT_EQUAL_INT(SIGKILL, WTERMSIG(status));
    assert_crash_state(b);

    status = run_child(generation_two, b);
    TEST_ASSERT_TRUE_MESSAGE(WIFEXITED(status) && WEXITSTATUS(status) == 0, "restart failed");
    assert_recovered(b);

    /* A further restart changes nothing (idempotent recovery, no replays). */
    status = run_child(generation_two, b);
    TEST_ASSERT_TRUE(WIFEXITED(status) && WEXITSTATUS(status) == 0);
    assert_recovered(b);
}

/* ---- tests -------------------------------------------------------------- */

static const boundary_t AFTER_BATCH_INSERT = {
    "after_batch_insert", HOOK_EVIDENCE, OBS_EVIDENCE_STAGE_AFTER_INSERT, false};
static const boundary_t AFTER_TMP_WRITE = {
    "after_tmp_write", HOOK_EVIDENCE, OBS_EVIDENCE_STAGE_BEFORE_RENAME, false};
static const boundary_t AFTER_RENAME_BEFORE_COMMIT = {
    "after_rename_before_commit", HOOK_EVIDENCE, OBS_EVIDENCE_STAGE_BEFORE_COMMIT, false};
static const boundary_t AFTER_COMMIT_BEFORE_RELAY = {
    "after_commit_before_relay", HOOK_RELAY, OBS_RELAY_STAGE_BEFORE_ENQUEUE, true};
static const boundary_t AFTER_OUTBOX_BEFORE_MARK = {
    "after_outbox_enqueue_before_mark", HOOK_RELAY, OBS_RELAY_STAGE_AFTER_ENQUEUE, true};
static const boundary_t AFTER_RELAY_BEFORE_DELIVERY = {
    "after_relay_before_delivery", HOOK_RELAY, OBS_RELAY_STAGE_AFTER_MARK, true};

void test_kill_after_batch_insert(void) { run_boundary(&AFTER_BATCH_INSERT); }
void test_kill_after_tmp_write(void) { run_boundary(&AFTER_TMP_WRITE); }
void test_kill_after_rename_before_commit(void) { run_boundary(&AFTER_RENAME_BEFORE_COMMIT); }
void test_kill_after_commit_before_relay(void) { run_boundary(&AFTER_COMMIT_BEFORE_RELAY); }
void test_kill_after_outbox_enqueue_before_mark(void) { run_boundary(&AFTER_OUTBOX_BEFORE_MARK); }
void test_kill_after_relay_before_delivery(void) { run_boundary(&AFTER_RELAY_BEFORE_DELIVERY); }

void setUp(void) {}
void tearDown(void) {}

int main(void) {
    snprintf(root_dir, sizeof(root_dir), "/tmp/lightnvr_obs_crash_%ld", (long)getpid());
    if (mkdir(root_dir, 0700) != 0 && errno != EEXIST) return 1;
    for (size_t i = 0; i < sizeof(submitted); ++i) submitted[i] = (uint8_t)((i * 31 + 7) & 0xff);
    submitted[0] = 0xff; submitted[1] = 0xd8;
    submitted[sizeof(submitted) - 2] = 0xff; submitted[sizeof(submitted) - 1] = 0xd9;

    UNITY_BEGIN();
    RUN_TEST(test_kill_after_batch_insert);
    RUN_TEST(test_kill_after_tmp_write);
    RUN_TEST(test_kill_after_rename_before_commit);
    RUN_TEST(test_kill_after_commit_before_relay);
    RUN_TEST(test_kill_after_outbox_enqueue_before_mark);
    RUN_TEST(test_kill_after_relay_before_delivery);
    int failures = UNITY_END();

    if (failures == 0) {
        char cmd[512];
        snprintf(cmd, sizeof(cmd), "rm -rf '%s'", root_dir);
        if (system(cmd) != 0) fprintf(stderr, "cleanup failed\n");
    } else {
        fprintf(stderr, "Crash-boundary evidence retained in %s\n", root_dir);
    }
    return failures;
}
