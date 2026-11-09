/* INT_01 L4: startup reconciliation, retention and eviction of observation
 * evidence (contract sections 4.2, 4.5, 7.6 and 14), against a real migrated
 * database and a temporary storage directory. */
#define _GNU_SOURCE
#include <dirent.h>
#include <errno.h>
#include <fcntl.h>
#include <ftw.h>
#include <pthread.h>
#include <sqlite3.h>
#include <stdatomic.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <time.h>
#include <unistd.h>

#include "unity.h"
#include "core/config.h"
#include "core/event_identity.h"
#include "core/observation_retention.h"
#include "core/path_utils.h"
#include "database/db_core.h"
#include "database/db_observations.h"
#include "database/db_recordings.h"
#include "database/db_streams.h"
#include "database/db_system_settings.h"
#include "utils/strings.h"
#include "utils/uuid.h"

#define DAY_MS 86400000LL

static char db_path[256];
static char storage[256];
static char cam_a[OBS_UUID_SIZE];
static char cam_b[OBS_UUID_SIZE];
static int64_t now_ms;
static const char *failed_unlink_path;
static const char *swap_unlink_path, *swap_parent, *swap_moved, *swap_target;
static bool swap_completed;
static bool fail_sidecar_rename, fail_sidecar_dir_sync;
static bool swap_sidecar_on_rename, sidecar_dir_synced;
static const char *sidecar_root, *sidecar_moved, *sidecar_outside;
static atomic_int retention_passes;

int __real_db_observations_prune(int64_t before_ms, int batch, observation_prune_result_t *out);
int __wrap_db_observations_prune(int64_t before_ms, int batch, observation_prune_result_t *out) {
    atomic_fetch_add(&retention_passes, 1);
    return __real_db_observations_prune(before_ms, batch, out);
}

int __real_unlink(const char *path);
int __wrap_unlink(const char *path) {
    if (failed_unlink_path && strcmp(path, failed_unlink_path) == 0) {
        errno = EIO;
        return -1;
    }
    return __real_unlink(path);
}

static bool matches_unlinkat(int fd, const char *name, const char *path) {
    struct stat actual, expected;
    return path && fstatat(fd, name, &actual, AT_SYMLINK_NOFOLLOW) == 0 &&
        lstat(path, &expected) == 0 && actual.st_dev == expected.st_dev &&
        actual.st_ino == expected.st_ino;
}

int __real_unlinkat(int fd, const char *name, int flags);
int __wrap_unlinkat(int fd, const char *name, int flags) {
    if (flags == 0 && matches_unlinkat(fd, name, failed_unlink_path)) {
        errno = EIO;
        return -1;
    }
    if (flags == 0 && matches_unlinkat(fd, name, swap_unlink_path)) {
        swap_unlink_path = NULL;
        /* Swap the pathname after traversal has opened its parent. */
        if (rename(swap_parent, swap_moved) != 0 || symlink(swap_target, swap_parent) != 0)
            return -1;
        swap_completed = true;
    }
    return __real_unlinkat(fd, name, flags);
}

int __real_renameat(int oldfd, const char *oldname, int newfd, const char *newname);
int __wrap_renameat(int oldfd, const char *oldname, int newfd, const char *newname) {
    if (strcmp(newname, OBS_CURSOR_HEAD_FILE) == 0) {
        if (fail_sidecar_rename) { errno = EIO; return -1; }
        if (swap_sidecar_on_rename && !swap_completed) {
            if (rename(sidecar_root, sidecar_moved) != 0 ||
                symlink(sidecar_outside, sidecar_root) != 0) return -1;
            swap_completed = true;
        }
    }
    return __real_renameat(oldfd, oldname, newfd, newname);
}

int __real_fsync(int fd);
int __wrap_fsync(int fd) {
    struct stat actual, expected;
    if (fstat(fd, &actual) == 0 && S_ISDIR(actual.st_mode)) {
        if (fail_sidecar_dir_sync) { errno = EIO; return -1; }
        if (swap_sidecar_on_rename && swap_completed && stat(sidecar_moved, &expected) == 0)
            sidecar_dir_synced = actual.st_dev == expected.st_dev && actual.st_ino == expected.st_ino;
    }
    return __real_fsync(fd);
}

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
    TEST_ASSERT_NOT_EQUAL_INT_MESSAGE(-999, (int)value, sql);
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

static observation_insert_result_t insert_batch(const char *camera, int64_t observed_at_ms,
                                                int objects, int64_t recording_id) {
    observation_batch_input_t batch;
    memset(&batch, 0, sizeof(batch));
    TEST_ASSERT_EQUAL_INT(0, lightnvr_uuid_generate_v4(batch.batch_id));
    safe_strcpy(batch.camera_uuid, camera, sizeof(batch.camera_uuid), 0);
    batch.observed_at_ms = observed_at_ms;
    safe_strcpy(batch.engine_key, "legacy-primary", sizeof(batch.engine_key), 0);
    safe_strcpy(batch.engine_type, "object", sizeof(batch.engine_type), 0);
    batch.recording_id = recording_id;
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

/* Relative path (stored in observation_assets.path) and absolute path. */
static void asset_paths(const char *camera, const char *asset, const char *suffix,
                        char *rel, size_t rel_size, char *abs, size_t abs_size) {
    snprintf(rel, rel_size, "observations/%s/2026-11-08/%s.jpg%s", camera, asset, suffix);
    snprintf(abs, abs_size, "%s/%s", storage, rel);
}

static void write_file(const char *path, size_t bytes) {
    char dir[512];
    safe_strcpy(dir, path, sizeof(dir), 0);
    char *slash = strrchr(dir, '/');
    TEST_ASSERT_NOT_NULL(slash);
    *slash = '\0';
    TEST_ASSERT_EQUAL_INT(0, mkdir_recursive(dir));
    FILE *f = fopen(path, "wb");
    TEST_ASSERT_NOT_NULL_MESSAGE(f, path);
    for (size_t i = 0; i < bytes; ++i) fputc(0xAB, f);
    fclose(f);
}

static bool exists(const char *path) {
    struct stat st;
    return lstat(path, &st) == 0;
}

typedef struct {
    observation_insert_result_t insert;
    char rel[512];
    char abs[512];
} ready_asset_t;

static ready_asset_t make_ready(const char *camera, int64_t observed_at_ms, int64_t bytes,
                                int objects) {
    ready_asset_t r;
    memset(&r, 0, sizeof(r));
    r.insert = insert_batch(camera, observed_at_ms, objects, 0);
    asset_paths(camera, r.insert.asset_uuid, "", r.rel, sizeof(r.rel), r.abs, sizeof(r.abs));
    write_file(r.abs, (size_t)bytes);
    observation_asset_media_t media = {
        .path = r.rel, .bytes = bytes, .width = 640, .height = 360, .expires_at_ms = 0,
    };
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK,
        db_observation_asset_transition(r.insert.asset_uuid, OBS_ASSET_PENDING,
                                        OBS_ASSET_READY, NULL, &media, NULL));
    return r;
}

static observation_asset_record_t asset_of(const char *uuid) {
    observation_asset_record_t asset;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_asset_get(uuid, &asset));
    return asset;
}

static void set_protected(const char *asset, int value) {
    char sql[256];
    snprintf(sql, sizeof(sql), "UPDATE observation_assets SET protected = %d WHERE uuid = '%s';",
             value, asset);
    exec_sql(sql);
}

static void set_asset_path(const char *asset, const char *path) {
    char sql[2048];
    snprintf(sql, sizeof(sql), "UPDATE observation_assets SET path = '%s' WHERE uuid = '%s';",
             path, asset);
    exec_sql(sql);
}

static void set_budget(const char *camera, int days, int64_t bytes) {
    char sql[256];
    snprintf(sql, sizeof(sql),
             "UPDATE streams SET observation_max_age_days = %d, observation_max_bytes = %lld "
             "WHERE camera_uuid = '%s';", days, (long long)bytes, camera);
    exec_sql(sql);
}

static int rm_entry(const char *path, const struct stat *st, int flag, struct FTW *ftw) {
    (void)st; (void)flag; (void)ftw;
    return remove(path);
}

static void rm_tree(const char *path) {
    nftw(path, rm_entry, 16, FTW_DEPTH | FTW_PHYS);
}

static observation_retention_result_t run_retention(int64_t at_ms) {
    observation_retention_result_t result;
    TEST_ASSERT_EQUAL_INT(0, observation_retention_run(storage, at_ms, &result));
    return result;
}

static void startup(observation_reconcile_result_t *result) {
    TEST_ASSERT_EQUAL_INT(0, observation_retention_startup(storage, result));
    TEST_ASSERT_TRUE(observation_retention_startup_done());
}

void setUp(void) {
    failed_unlink_path = NULL;
    swap_unlink_path = NULL;
    swap_completed = false;
    fail_sidecar_rename = fail_sidecar_dir_sync = false;
    swap_sidecar_on_rename = sidecar_dir_synced = false;
    atomic_store(&retention_passes, 0);
    exec_sql("DELETE FROM object_observations; DELETE FROM observation_assets; "
             "DELETE FROM observation_changes; DELETE FROM recordings;");
    char dir[512];
    snprintf(dir, sizeof(dir), "%s/observations", storage);
    rm_tree(dir);
    set_budget(cam_a, 7, 2147483648LL);
    set_budget(cam_b, 7, 2147483648LL);
    TEST_ASSERT_EQUAL_INT(0, db_set_system_setting("observation_history_retention_days", "30"));
    observation_retention_reset_for_testing();
}
void tearDown(void) {
    failed_unlink_path = NULL;
    swap_unlink_path = NULL;
    fail_sidecar_rename = fail_sidecar_dir_sync = swap_sidecar_on_rename = false;
}

/* ---- startup reconciliation ------------------------------------------- */

static void test_startup_reconciles_interrupted_missing_and_orphan_files(void) {
    /* A: interrupted write, only the temp file exists. */
    observation_insert_result_t a = insert_batch(cam_a, now_ms, 2, 0);
    char rel[512], a_tmp[512], b_final[512], orphan[512], scratch[512];
    asset_paths(cam_a, a.asset_uuid, ".tmp", rel, sizeof(rel), a_tmp, sizeof(a_tmp));
    write_file(a_tmp, 100);
    /* B: crash after rename but before the ready commit. */
    observation_insert_result_t b = insert_batch(cam_a, now_ms, 1, 0);
    asset_paths(cam_a, b.asset_uuid, "", rel, sizeof(rel), b_final, sizeof(b_final));
    write_file(b_final, 100);
    /* C: ready whose file vanished; D: healthy ready asset. */
    ready_asset_t c = make_ready(cam_a, now_ms, 200, 1);
    TEST_ASSERT_EQUAL_INT(0, unlink(c.abs));
    ready_asset_t d = make_ready(cam_b, now_ms, 300, 1);
    /* An orphan final file, an orphan temp file, and files L4 must not touch. */
    char orphan_uuid[OBS_UUID_SIZE];
    TEST_ASSERT_EQUAL_INT(0, lightnvr_uuid_generate_v4(orphan_uuid));
    asset_paths(cam_b, orphan_uuid, "", rel, sizeof(rel), orphan, sizeof(orphan));
    write_file(orphan, 50);
    char orphan_tmp[512];
    asset_paths(cam_b, orphan_uuid, ".tmp", rel, sizeof(rel), orphan_tmp, sizeof(orphan_tmp));
    write_file(orphan_tmp, 50);
    snprintf(scratch, sizeof(scratch), "%s/observations/README.txt", storage);
    write_file(scratch, 10);
    char recording[512];
    snprintf(recording, sizeof(recording), "%s/recordings/cam_a/%s.jpg", storage, orphan_uuid);
    write_file(recording, 10);

    observation_reconcile_result_t result;
    startup(&result);
    TEST_ASSERT_EQUAL_INT(2, result.tmp_files_deleted);
    TEST_ASSERT_EQUAL_INT(2, result.stale_files_deleted);  /* orphan + B's final */
    TEST_ASSERT_EQUAL_INT(2, result.pending_interrupted);
    TEST_ASSERT_EQUAL_INT(1, result.ready_missing);

    TEST_ASSERT_FALSE(exists(a_tmp));
    TEST_ASSERT_FALSE(exists(b_final));
    TEST_ASSERT_FALSE(exists(orphan));
    TEST_ASSERT_FALSE(exists(orphan_tmp));
    TEST_ASSERT_TRUE(exists(d.abs));
    TEST_ASSERT_TRUE(exists(scratch));
    TEST_ASSERT_TRUE(exists(recording));

    observation_asset_record_t asset = asset_of(a.asset_uuid);
    TEST_ASSERT_EQUAL_INT(OBS_ASSET_UNAVAILABLE, asset.state);
    TEST_ASSERT_EQUAL_STRING("interrupted", asset.reason);
    asset = asset_of(b.asset_uuid);
    TEST_ASSERT_EQUAL_INT(OBS_ASSET_UNAVAILABLE, asset.state);
    TEST_ASSERT_EQUAL_STRING("interrupted", asset.reason);
    asset = asset_of(c.insert.asset_uuid);
    TEST_ASSERT_EQUAL_INT(OBS_ASSET_UNAVAILABLE, asset.state);
    TEST_ASSERT_EQUAL_STRING("write_failed", asset.reason);
    TEST_ASSERT_EQUAL_INT(OBS_ASSET_READY, asset_of(d.insert.asset_uuid).state);

    /* Every observation sharing A got an evidence revision (published). */
    for (int i = 0; i < 2; ++i) {
        observation_record_t record;
        TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_get(a.observation_uuids[i], &record));
        TEST_ASSERT_EQUAL_INT(2, record.revision);
    }

    /* Idempotent: a second sweep finds nothing to do. */
    startup(&result);
    TEST_ASSERT_EQUAL_INT(0, result.tmp_files_deleted + result.stale_files_deleted +
                             result.pending_interrupted + result.ready_missing);
}

/* The sweep never follows a symlink below the evidence root: neither a
 * symlinked directory nor a symlinked file. Files outside the root survive. */
static void test_startup_sweep_does_not_follow_symlinks_below_root(void) {
    char outside[512], out_tmp[512], out_final[512], uuid[OBS_UUID_SIZE];
    snprintf(outside, sizeof(outside), "%s/outside-evidence", storage);
    rm_tree(outside);
    TEST_ASSERT_EQUAL_INT(0, lightnvr_uuid_generate_v4(uuid));
    snprintf(out_tmp, sizeof(out_tmp), "%s/%s.jpg.tmp", outside, uuid);
    snprintf(out_final, sizeof(out_final), "%s/%s.jpg", outside, uuid);
    write_file(out_tmp, 10);
    write_file(out_final, 10);

    char cam_dir[512], dir_link[512], file_link[512];
    snprintf(cam_dir, sizeof(cam_dir), "%s/observations/%s/2026-11-08", storage, cam_a);
    TEST_ASSERT_EQUAL_INT(0, mkdir_recursive(cam_dir));
    snprintf(dir_link, sizeof(dir_link), "%s/observations/%s/2026-11-09", storage, cam_a);
    TEST_ASSERT_EQUAL_INT(0, symlink(outside, dir_link));
    char other[OBS_UUID_SIZE];
    TEST_ASSERT_EQUAL_INT(0, lightnvr_uuid_generate_v4(other));
    snprintf(file_link, sizeof(file_link), "%s/%s.jpg.tmp", cam_dir, other);
    TEST_ASSERT_EQUAL_INT(0, symlink(out_tmp, file_link));

    observation_reconcile_result_t result;
    startup(&result);
    TEST_ASSERT_EQUAL_INT(0, result.tmp_files_deleted);
    TEST_ASSERT_EQUAL_INT(0, result.stale_files_deleted);
    TEST_ASSERT_TRUE(exists(out_tmp));
    TEST_ASSERT_TRUE(exists(out_final));
    TEST_ASSERT_TRUE(exists(dir_link));
    TEST_ASSERT_TRUE(exists(file_link));
    rm_tree(outside);
}

/* The evidence root itself may be a symlink (a mounted storage path); it is
 * still swept. */
static void test_startup_sweep_follows_a_symlinked_evidence_root(void) {
    char real[512], root[512], tmp[512], uuid[OBS_UUID_SIZE];
    snprintf(real, sizeof(real), "%s/observations-real", storage);
    snprintf(root, sizeof(root), "%s/observations", storage);
    rm_tree(real);
    rm_tree(root);
    TEST_ASSERT_EQUAL_INT(0, lightnvr_uuid_generate_v4(uuid));
    snprintf(tmp, sizeof(tmp), "%s/%s/2026-11-08/%s.jpg.tmp", real, cam_a, uuid);
    write_file(tmp, 10);
    TEST_ASSERT_EQUAL_INT(0, symlink(real, root));
    ready_asset_t healthy = make_ready(cam_a, now_ms, 10, 1);

    observation_reconcile_result_t result;
    startup(&result);
    TEST_ASSERT_EQUAL_INT(1, result.tmp_files_deleted);
    TEST_ASSERT_FALSE(exists(tmp));
    TEST_ASSERT_EQUAL_INT(0, result.ready_missing);
    TEST_ASSERT_EQUAL_INT(OBS_ASSET_READY, asset_of(healthy.insert.asset_uuid).state);
    TEST_ASSERT_EQUAL_INT(0, unlink(root));
    rm_tree(real);
}

static void test_startup_preserves_all_supported_asset_path_forms(void) {
    ready_asset_t assets[3];
    for (int i = 0; i < 3; ++i) assets[i] = make_ready(cam_a, now_ms, 10, 1);
    set_asset_path(assets[1].insert.asset_uuid, assets[1].rel + strlen("observations/"));
    set_asset_path(assets[2].insert.asset_uuid, assets[2].abs);
    observation_reconcile_result_t result;
    startup(&result);
    TEST_ASSERT_EQUAL_INT(0, result.ready_missing);
    TEST_ASSERT_EQUAL_INT(0, result.stale_files_deleted);
    for (int i = 0; i < 3; ++i) {
        TEST_ASSERT_EQUAL_INT(OBS_ASSET_READY, asset_of(assets[i].insert.asset_uuid).state);
        TEST_ASSERT_TRUE(exists(assets[i].abs));
    }
}

static void test_startup_rejects_symlink_parents_and_nonregular_leaves(void) {
    ready_asset_t parent = make_ready(cam_a, now_ms, 10, 1);
    ready_asset_t leaf = make_ready(cam_b, now_ms, 10, 1);
    ready_asset_t fifo = make_ready(cam_b, now_ms, 10, 1);
    ready_asset_t directory = make_ready(cam_b, now_ms, 10, 1);
    ready_asset_t wrong_extension = make_ready(cam_b, now_ms, 10, 1);
    char original_dir[512], outside_dir[512], outside_file[512], png[1024];
    snprintf(original_dir, sizeof(original_dir), "%s/observations/%s/2026-11-08", storage, cam_a);
    snprintf(outside_dir, sizeof(outside_dir), "%s/outside-reconcile", storage);
    snprintf(outside_file, sizeof(outside_file), "%s/outside-leaf.jpg", storage);
    rm_tree(outside_dir);
    TEST_ASSERT_EQUAL_INT(0, rename(original_dir, outside_dir));
    TEST_ASSERT_EQUAL_INT(0, symlink(outside_dir, original_dir));
    TEST_ASSERT_EQUAL_INT(0, rename(leaf.abs, outside_file));
    TEST_ASSERT_EQUAL_INT(0, symlink(outside_file, leaf.abs));
    TEST_ASSERT_EQUAL_INT(0, unlink(fifo.abs));
    TEST_ASSERT_EQUAL_INT(0, mkfifo(fifo.abs, 0600));
    TEST_ASSERT_EQUAL_INT(0, unlink(directory.abs));
    TEST_ASSERT_EQUAL_INT(0, mkdir(directory.abs, 0700));
    snprintf(png, sizeof(png), "%s.png", wrong_extension.abs);
    TEST_ASSERT_EQUAL_INT(0, rename(wrong_extension.abs, png));
    set_asset_path(wrong_extension.insert.asset_uuid, png);

    observation_reconcile_result_t result;
    startup(&result);  /* Must not block opening the FIFO. */
    TEST_ASSERT_EQUAL_INT(5, result.ready_missing);
    const ready_asset_t *assets[] = {&parent, &leaf, &fifo, &directory, &wrong_extension};
    for (size_t i = 0; i < sizeof(assets) / sizeof(assets[0]); ++i) {
        observation_asset_record_t asset = asset_of(assets[i]->insert.asset_uuid);
        TEST_ASSERT_EQUAL_INT(OBS_ASSET_UNAVAILABLE, asset.state);
        TEST_ASSERT_EQUAL_STRING("write_failed", asset.reason);
    }
    TEST_ASSERT_TRUE(exists(parent.abs));  /* The outside target survives. */
    TEST_ASSERT_TRUE(exists(outside_file));
    TEST_ASSERT_TRUE(exists(leaf.abs));
    TEST_ASSERT_TRUE(exists(fifo.abs));
    TEST_ASSERT_TRUE(exists(png));
    rm_tree(outside_dir);
    TEST_ASSERT_EQUAL_INT(0, unlink(outside_file));
}

static void *maintenance_tick_thread(void *arg) {
    pthread_barrier_wait(arg);
    observation_retention_maintenance_tick();
    return NULL;
}

static void test_maintenance_drains_partial_age_pages_and_serializes_ticks(void) {
    observation_reconcile_result_t rec;
    startup(&rec);
    /* Cameras are visited in UUID order: the first uses one eviction, leaving
     * a 255-entry query for the second, whose last old asset needs another pass. */
    const char *first = strcmp(cam_a, cam_b) < 0 ? cam_a : cam_b;
    const char *second = first == cam_a ? cam_b : cam_a;
    (void)make_ready(first, now_ms - 8 * DAY_MS, 1, 1);
    for (int i = 0; i < OBS_RETENTION_MAX_EVICTIONS; ++i)
        (void)make_ready(second, now_ms - 8 * DAY_MS, 1, 1);
    observation_retention_maintenance_tick();
    TEST_ASSERT_EQUAL_INT(1, scalar64("SELECT count(*) FROM observation_assets WHERE state = 'ready';"));
    TEST_ASSERT_EQUAL_INT(1, atomic_load(&retention_passes));

    pthread_t threads[8];
    pthread_barrier_t barrier;
    TEST_ASSERT_EQUAL_INT(0, pthread_barrier_init(&barrier, NULL, 8));
    for (int i = 0; i < 8; ++i)
        TEST_ASSERT_EQUAL_INT(0, pthread_create(&threads[i], NULL, maintenance_tick_thread, &barrier));
    for (int i = 0; i < 8; ++i) TEST_ASSERT_EQUAL_INT(0, pthread_join(threads[i], NULL));
    TEST_ASSERT_EQUAL_INT(0, pthread_barrier_destroy(&barrier));
    TEST_ASSERT_EQUAL_INT(0, scalar64("SELECT count(*) FROM observation_assets WHERE state = 'ready';"));
    TEST_ASSERT_EQUAL_INT(2, atomic_load(&retention_passes));

    ready_asset_t later = make_ready(first, now_ms - 8 * DAY_MS, 1, 1);
    observation_retention_maintenance_tick();
    TEST_ASSERT_EQUAL_INT(OBS_ASSET_READY, asset_of(later.insert.asset_uuid).state);
    TEST_ASSERT_EQUAL_INT(2, atomic_load(&retention_passes));
}

static void test_periodic_run_waits_for_startup_reconciliation(void) {
    ready_asset_t old = make_ready(cam_a, now_ms - 30 * DAY_MS, 100, 1);
    observation_retention_result_t result;
    TEST_ASSERT_EQUAL_INT(1, observation_retention_run(storage, now_ms, &result));
    TEST_ASSERT_EQUAL_INT(OBS_ASSET_READY, asset_of(old.insert.asset_uuid).state);
    TEST_ASSERT_TRUE(exists(old.abs));
}

static void test_failed_restart_disables_retention_until_reconciled(void) {
    observation_reconcile_result_t rec;
    startup(&rec);
    ready_asset_t old = make_ready(cam_a, now_ms - 8 * DAY_MS, 10, 1);
    char blocked[512];
    snprintf(blocked, sizeof(blocked), "%s/observations/%s", storage, OBS_CURSOR_HEAD_FILE);
    TEST_ASSERT_EQUAL_INT(0, unlink(blocked));
    TEST_ASSERT_EQUAL_INT(0, mkdir(blocked, 0700));
    TEST_ASSERT_EQUAL_INT(-1, observation_retention_startup(storage, &rec));
    TEST_ASSERT_FALSE(observation_retention_startup_done());
    observation_retention_result_t result;
    TEST_ASSERT_EQUAL_INT(1, observation_retention_run(storage, now_ms, &result));
    observation_retention_maintenance_tick();
    TEST_ASSERT_EQUAL_INT(0, atomic_load(&retention_passes));
    TEST_ASSERT_TRUE(exists(old.abs));

    TEST_ASSERT_EQUAL_INT(0, rmdir(blocked));
    startup(&rec);
    TEST_ASSERT_EQUAL_INT(-1, observation_retention_startup(NULL, &rec));
    TEST_ASSERT_FALSE(observation_retention_startup_done());
    TEST_ASSERT_EQUAL_INT(1, observation_retention_run(storage, now_ms, &result));
    startup(&rec);
    result = run_retention(now_ms);
    TEST_ASSERT_EQUAL_INT(1, result.assets_expired);
    TEST_ASSERT_FALSE(exists(old.abs));
}

static void test_retention_preserves_outside_targets_of_parent_and_leaf_symlinks(void) {
    observation_reconcile_result_t rec;
    startup(&rec);
    ready_asset_t parent = make_ready(cam_a, now_ms - 8 * DAY_MS, 10, 1);
    ready_asset_t leaf = make_ready(cam_b, now_ms - 8 * DAY_MS, 10, 1);
    char original[512], outside[512], outside_file[512];
    snprintf(original, sizeof(original), "%s/observations/%s/2026-11-08", storage, cam_a);
    snprintf(outside, sizeof(outside), "%s/outside-retention", storage);
    snprintf(outside_file, sizeof(outside_file), "%s/outside-retention-leaf.jpg", storage);
    rm_tree(outside);
    TEST_ASSERT_EQUAL_INT(0, rename(original, outside));
    TEST_ASSERT_EQUAL_INT(0, symlink(outside, original));
    TEST_ASSERT_EQUAL_INT(0, rename(leaf.abs, outside_file));
    TEST_ASSERT_EQUAL_INT(0, symlink(outside_file, leaf.abs));

    observation_retention_result_t result = run_retention(now_ms);
    TEST_ASSERT_EQUAL_INT(2, result.assets_expired);
    TEST_ASSERT_EQUAL_INT(1, result.files_deleted);  /* Only the leaf link. */
    TEST_ASSERT_TRUE(exists(parent.abs));
    TEST_ASSERT_TRUE(exists(outside_file));
    TEST_ASSERT_FALSE(exists(leaf.abs));
    rm_tree(outside);
    TEST_ASSERT_EQUAL_INT(0, unlink(outside_file));
}

static void test_retention_parent_swap_cannot_redirect_unlink(void) {
    observation_reconcile_result_t rec;
    startup(&rec);
    ready_asset_t old = make_ready(cam_a, now_ms - 8 * DAY_MS, 10, 1);
    char parent[512], moved[512], outside[512], outside_file[1024], moved_file[1024];
    snprintf(parent, sizeof(parent), "%s/observations/%s/2026-11-08", storage, cam_a);
    snprintf(moved, sizeof(moved), "%s/retention-moved", storage);
    snprintf(outside, sizeof(outside), "%s/retention-swap-target", storage);
    snprintf(outside_file, sizeof(outside_file), "%s/%s.jpg", outside, old.insert.asset_uuid);
    snprintf(moved_file, sizeof(moved_file), "%s/%s.jpg", moved, old.insert.asset_uuid);
    rm_tree(moved);
    write_file(outside_file, 20);
    swap_unlink_path = old.abs;
    swap_parent = parent;
    swap_moved = moved;
    swap_target = outside;

    observation_retention_result_t result = run_retention(now_ms);
    TEST_ASSERT_TRUE(swap_completed);
    TEST_ASSERT_EQUAL_INT(1, result.files_deleted);
    TEST_ASSERT_FALSE(exists(moved_file));
    TEST_ASSERT_TRUE(exists(outside_file));
    rm_tree(moved);
    rm_tree(outside);
}

static void test_retention_accepts_path_forms_and_symlinked_root(void) {
    observation_reconcile_result_t rec;
    startup(&rec);
    char root[512], real[512];
    snprintf(root, sizeof(root), "%s/observations", storage);
    snprintf(real, sizeof(real), "%s/retention-real-root", storage);
    rm_tree(real);
    TEST_ASSERT_EQUAL_INT(0, rename(root, real));
    TEST_ASSERT_EQUAL_INT(0, symlink(real, root));
    ready_asset_t assets[3];
    for (int i = 0; i < 3; ++i) assets[i] = make_ready(cam_a, now_ms - 8 * DAY_MS, 10, 1);
    set_asset_path(assets[1].insert.asset_uuid, assets[1].rel + strlen("observations/"));
    set_asset_path(assets[2].insert.asset_uuid, assets[2].abs);

    observation_retention_result_t result = run_retention(now_ms);
    TEST_ASSERT_EQUAL_INT(3, result.files_deleted);
    for (int i = 0; i < 3; ++i) {
        TEST_ASSERT_FALSE(exists(assets[i].abs));
        TEST_ASSERT_EQUAL_STRING("", asset_of(assets[i].insert.asset_uuid).path);
    }
    TEST_ASSERT_EQUAL_INT(0, unlink(root));
    rm_tree(real);
}

static void test_retention_missing_root_clears_completed_cleanup(void) {
    observation_reconcile_result_t rec;
    startup(&rec);
    ready_asset_t old = make_ready(cam_a, now_ms - 8 * DAY_MS, 10, 1);
    char root[512];
    snprintf(root, sizeof(root), "%s/observations", storage);
    rm_tree(root);

    observation_retention_result_t result = run_retention(now_ms);
    TEST_ASSERT_EQUAL_INT(1, result.assets_expired);
    TEST_ASSERT_EQUAL_INT(0, result.files_deleted);
    TEST_ASSERT_EQUAL_STRING("", asset_of(old.insert.asset_uuid).path);
    TEST_ASSERT_EQUAL_INT(OBS_ASSET_EXPIRED, asset_of(old.insert.asset_uuid).state);
}

/* ---- age and byte budgets ---------------------------------------------- */

static void test_age_budget_expires_media_and_keeps_metadata(void) {
    observation_reconcile_result_t rec;
    startup(&rec);
    ready_asset_t old = make_ready(cam_a, now_ms - 8 * DAY_MS, 1000, 1);
    ready_asset_t fresh = make_ready(cam_a, now_ms - 1 * DAY_MS, 1000, 1);
    ready_asset_t protected_old = make_ready(cam_a, now_ms - 9 * DAY_MS, 1000, 1);
    set_protected(protected_old.insert.asset_uuid, 1);
    observation_insert_result_t pending_old = insert_batch(cam_a, now_ms - 20 * DAY_MS, 1, 0);

    observation_retention_result_t result = run_retention(now_ms);
    TEST_ASSERT_EQUAL_INT(1, result.assets_expired);
    TEST_ASSERT_FALSE(exists(old.abs));
    TEST_ASSERT_TRUE(exists(fresh.abs));
    TEST_ASSERT_TRUE(exists(protected_old.abs));

    observation_record_t record;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_get(old.insert.observation_uuids[0], &record));
    TEST_ASSERT_EQUAL_INT(OBS_ASSET_EXPIRED, record.asset.state);
    TEST_ASSERT_EQUAL_STRING("retention_expired", record.asset.reason);
    TEST_ASSERT_EQUAL_INT(640, record.asset.width);
    TEST_ASSERT_EQUAL_INT(360, record.asset.height);
    TEST_ASSERT_EQUAL_INT64(1000, record.asset.bytes);
    TEST_ASSERT_EQUAL_INT(OBS_ASSET_READY, asset_of(fresh.insert.asset_uuid).state);
    TEST_ASSERT_EQUAL_INT(OBS_ASSET_READY, asset_of(protected_old.insert.asset_uuid).state);
    /* Pending is the worker's: never evicted. */
    TEST_ASSERT_EQUAL_INT(OBS_ASSET_PENDING, asset_of(pending_old.asset_uuid).state);
    TEST_ASSERT_FALSE(observation_retention_budget_exhausted(cam_a));
}

static void test_shared_asset_survives_until_last_reference_expires(void) {
    observation_reconcile_result_t rec;
    startup(&rec);
    ready_asset_t shared = make_ready(cam_a, now_ms - 8 * DAY_MS, 1000, 2);
    /* The second reference is newer (e.g. a later sibling re-using the frame). */
    char sql[256];
    snprintf(sql, sizeof(sql),
             "UPDATE object_observations SET observed_at_ms = %lld WHERE uuid = '%s';",
             (long long)(now_ms - 2 * DAY_MS), shared.insert.observation_uuids[1]);
    exec_sql(sql);

    run_retention(now_ms);
    TEST_ASSERT_EQUAL_INT(OBS_ASSET_READY, asset_of(shared.insert.asset_uuid).state);
    TEST_ASSERT_TRUE(exists(shared.abs));

    run_retention(now_ms + 6 * DAY_MS);
    TEST_ASSERT_EQUAL_INT(OBS_ASSET_EXPIRED, asset_of(shared.insert.asset_uuid).state);
    TEST_ASSERT_FALSE(exists(shared.abs));
    for (int i = 0; i < 2; ++i) {
        observation_record_t record;
        TEST_ASSERT_EQUAL_INT(DB_OBS_OK,
                              db_observation_get(shared.insert.observation_uuids[i], &record));
        TEST_ASSERT_EQUAL_INT(OBS_ASSET_EXPIRED, record.asset.state);
    }
}

static void test_byte_budget_evicts_oldest_and_protected_assets_flip_the_flag(void) {
    observation_reconcile_result_t rec;
    startup(&rec);
    ready_asset_t p1 = make_ready(cam_a, now_ms - 5 * 3600000LL, 6000, 1);
    set_protected(p1.insert.asset_uuid, 1);
    ready_asset_t u1 = make_ready(cam_a, now_ms - 4 * 3600000LL, 3000, 1);
    ready_asset_t u2 = make_ready(cam_a, now_ms - 3 * 3600000LL, 3000, 1);
    ready_asset_t other = make_ready(cam_b, now_ms - 6 * 3600000LL, 9000, 1);
    set_budget(cam_a, 7, 10000);
    set_budget(cam_b, 7, 10000);

    observation_retention_result_t result = run_retention(now_ms);
    TEST_ASSERT_EQUAL_INT(1, result.assets_expired);
    TEST_ASSERT_EQUAL_INT(OBS_ASSET_EXPIRED, asset_of(u1.insert.asset_uuid).state);
    TEST_ASSERT_EQUAL_STRING("retention_expired", asset_of(u1.insert.asset_uuid).reason);
    TEST_ASSERT_FALSE(exists(u1.abs));
    TEST_ASSERT_EQUAL_INT(OBS_ASSET_READY, asset_of(u2.insert.asset_uuid).state);
    TEST_ASSERT_EQUAL_INT(OBS_ASSET_READY, asset_of(other.insert.asset_uuid).state);
    TEST_ASSERT_FALSE(observation_retention_budget_exhausted(cam_a));
    TEST_ASSERT_EQUAL_INT(0, observation_retention_budget_exhausted_count());

    /* Shrink the budget below the protected asset alone. */
    set_budget(cam_a, 7, 5000);
    result = run_retention(now_ms);
    TEST_ASSERT_EQUAL_INT(1, result.assets_expired);
    TEST_ASSERT_FALSE(exists(u2.abs));
    TEST_ASSERT_TRUE(exists(p1.abs));
    TEST_ASSERT_EQUAL_INT(OBS_ASSET_READY, asset_of(p1.insert.asset_uuid).state);
    TEST_ASSERT_EQUAL_INT(1, result.cameras_budget_exhausted);
    TEST_ASSERT_TRUE(observation_retention_budget_exhausted(cam_a));
    TEST_ASSERT_FALSE(observation_retention_budget_exhausted(cam_b));
    TEST_ASSERT_EQUAL_INT(1, observation_retention_budget_exhausted_count());

    observation_retention_camera_status_t status[4];
    int n = observation_retention_camera_status(status, 4);
    TEST_ASSERT_EQUAL_INT(2, n);
    bool seen = false;
    for (int i = 0; i < n; ++i) {
        if (strcmp(status[i].camera_uuid, cam_a) != 0) continue;
        seen = true;
        TEST_ASSERT_TRUE(status[i].budget_exhausted);
        TEST_ASSERT_EQUAL_INT64(6000, status[i].retained_bytes);
        TEST_ASSERT_EQUAL_INT64(5000, status[i].max_bytes);
    }
    TEST_ASSERT_TRUE(seen);

    /* Unprotecting lets eviction proceed and clears the flag. */
    set_protected(p1.insert.asset_uuid, 0);
    run_retention(now_ms);
    TEST_ASSERT_EQUAL_INT(OBS_ASSET_EXPIRED, asset_of(p1.insert.asset_uuid).state);
    TEST_ASSERT_FALSE(exists(p1.abs));
    TEST_ASSERT_FALSE(observation_retention_budget_exhausted(cam_a));
    TEST_ASSERT_EQUAL_INT(0, observation_retention_budget_exhausted_count());
}

/* ---- history ----------------------------------------------------------- */

static void test_history_prune_advances_oldest_seq_and_reclaims_assets(void) {
    observation_reconcile_result_t rec;
    startup(&rec);
    ready_asset_t old = make_ready(cam_a, now_ms - 3 * DAY_MS, 500, 1);
    ready_asset_t fresh = make_ready(cam_a, now_ms, 500, 1);
    observation_meta_t before;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_meta_get(&before));

    char sql[512];
    snprintf(sql, sizeof(sql),
             "UPDATE observation_changes SET created_at_ms = %lld WHERE observation_uuid = '%s';",
             (long long)(now_ms - 3 * DAY_MS), old.insert.observation_uuids[0]);
    exec_sql(sql);
    TEST_ASSERT_EQUAL_INT(0, db_set_system_setting("observation_history_retention_days", "1"));

    observation_retention_result_t result = run_retention(now_ms);
    TEST_ASSERT_EQUAL_INT(2, result.changes_pruned);  /* created + evidence */
    TEST_ASSERT_EQUAL_INT(1, result.observations_pruned);

    observation_meta_t after;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_meta_get(&after));
    TEST_ASSERT_TRUE(after.oldest_seq > before.oldest_seq);
    TEST_ASSERT_EQUAL_INT64(after.oldest_seq, result.oldest_seq);

    /* The asset lost its last reference: file deleted and row removed. */
    observation_asset_record_t asset;
    TEST_ASSERT_EQUAL_INT(DB_OBS_NOT_FOUND, db_observation_asset_get(old.insert.asset_uuid, &asset));
    TEST_ASSERT_FALSE(exists(old.abs));
    TEST_ASSERT_EQUAL_INT(OBS_ASSET_READY, asset_of(fresh.insert.asset_uuid).state);
    TEST_ASSERT_TRUE(exists(fresh.abs));
}

/* ---- isolation from recordings ----------------------------------------- */

static void test_failed_expiry_unlink_is_retried_without_restart(void) {
    observation_reconcile_result_t rec;
    startup(&rec);
    ready_asset_t old = make_ready(cam_a, now_ms - 8 * DAY_MS, 500, 1);
    failed_unlink_path = old.abs;
    observation_retention_result_t result;
    (void)observation_retention_run(storage, now_ms, &result);
    TEST_ASSERT_EQUAL_INT(OBS_ASSET_EXPIRED, asset_of(old.insert.asset_uuid).state);
    TEST_ASSERT_TRUE(exists(old.abs));

    failed_unlink_path = NULL;
    result = run_retention(now_ms);
    TEST_ASSERT_FALSE(exists(old.abs));
    TEST_ASSERT_EQUAL_INT(1, result.files_deleted);
    TEST_ASSERT_EQUAL_INT(OBS_ASSET_EXPIRED, asset_of(old.insert.asset_uuid).state);
}

static void test_prune_preserves_failed_unlink_for_retry(void) {
    observation_reconcile_result_t rec;
    startup(&rec);
    ready_asset_t old = make_ready(cam_a, now_ms - 8 * DAY_MS, 500, 1);
    failed_unlink_path = old.abs;
    observation_retention_result_t result;
    (void)observation_retention_run(storage, now_ms, &result);
    exec_sql("UPDATE observation_changes SET created_at_ms = 1;");
    (void)observation_retention_run(storage, now_ms, &result);
    TEST_ASSERT_TRUE(exists(old.abs));
    observation_asset_record_t asset;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_asset_get(old.insert.asset_uuid, &asset));

    failed_unlink_path = NULL;
    (void)run_retention(now_ms);
    TEST_ASSERT_FALSE(exists(old.abs));
    TEST_ASSERT_EQUAL_INT(DB_OBS_NOT_FOUND,
                          db_observation_asset_get(old.insert.asset_uuid, &asset));
}

static void test_recording_deletion_leaves_assets_intact(void) {
    observation_reconcile_result_t rec;
    startup(&rec);
    char rec_path[512];
    snprintf(rec_path, sizeof(rec_path), "%s/recordings/cam_a/clip.mp4", storage);
    write_file(rec_path, 64);
    recording_metadata_t meta;
    memset(&meta, 0, sizeof(meta));
    safe_strcpy(meta.stream_name, "cam_a", sizeof(meta.stream_name), 0);
    safe_strcpy(meta.file_path, rec_path, sizeof(meta.file_path), 0);
    safe_strcpy(meta.codec, "h264", sizeof(meta.codec), 0);
    safe_strcpy(meta.trigger_type, "detection", sizeof(meta.trigger_type), 0);
    meta.start_time = (time_t)(now_ms / 1000) - 60;
    meta.end_time = (time_t)(now_ms / 1000);
    meta.is_complete = true;
    meta.size_bytes = 64;
    meta.retention_override_days = -1;
    meta.retention_tier = 2;
    meta.schedule_restricted = -1;
    uint64_t recording_id = add_recording_metadata(&meta);
    TEST_ASSERT_TRUE(recording_id > 0);

    observation_insert_result_t ins = insert_batch(cam_a, now_ms, 1, (int64_t)recording_id);
    char rel[512], abs[512];
    asset_paths(cam_a, ins.asset_uuid, "", rel, sizeof(rel), abs, sizeof(abs));
    write_file(abs, 100);
    observation_asset_media_t media = {.path = rel, .bytes = 100, .width = 64, .height = 48};
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_asset_transition(
        ins.asset_uuid, OBS_ASSET_PENDING, OBS_ASSET_READY, NULL, &media, NULL));

    char sql[128];
    snprintf(sql, sizeof(sql), "DELETE FROM recordings WHERE id = %llu;",
             (unsigned long long)recording_id);
    exec_sql(sql);

    run_retention(now_ms);
    observation_record_t record;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_get(ins.observation_uuids[0], &record));
    TEST_ASSERT_EQUAL_INT64(0, record.recording_id);
    TEST_ASSERT_EQUAL_INT(OBS_ASSET_READY, record.asset.state);
    TEST_ASSERT_TRUE(exists(abs));
    /* Evidence eviction never touches recordings either, even for a path that
     * escapes the observations tree. */
    char evil[600];
    snprintf(evil, sizeof(evil),
             "UPDATE observation_assets SET path = 'observations/../recordings/cam_a/clip.mp4' "
             "WHERE uuid = '%s';", ins.asset_uuid);
    exec_sql(evil);
    run_retention(now_ms + 30 * DAY_MS);
    TEST_ASSERT_EQUAL_INT(OBS_ASSET_EXPIRED, asset_of(ins.asset_uuid).state);
    TEST_ASSERT_TRUE(exists(rec_path));
}

/* ---- D9 cursor head sidecar -------------------------------------------- */

static void test_cursor_head_write_recovers_with_stale_temporary_files(void) {
    char tmp[512];
    snprintf(tmp, sizeof(tmp), "%s/observations/%s.tmp", storage, OBS_CURSOR_HEAD_FILE);
    write_file(tmp, 7);  /* A crash before rename in an older build. */
    snprintf(tmp, sizeof(tmp), "%s/observations/%s.66666666-6666-4666-8666-666666666666.tmp",
             storage, OBS_CURSOR_HEAD_FILE);
    write_file(tmp, 9);  /* An interrupted write with a unique name. */
    observation_reconcile_result_t rec;
    startup(&rec);
    observation_meta_t meta;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_meta_get(&meta));
    TEST_ASSERT_EQUAL_INT(0, observation_cursor_head_write(storage, meta.epoch_uuid, 42));
    char epoch[OBS_UUID_SIZE];
    int64_t head = -1;
    TEST_ASSERT_EQUAL_INT(0, observation_cursor_head_read(storage, epoch, &head));
    TEST_ASSERT_EQUAL_STRING(meta.epoch_uuid, epoch);
    TEST_ASSERT_EQUAL_INT64(42, head);
}

static void assert_marker_untouched(const char *path) {
    FILE *f = fopen(path, "rb");
    TEST_ASSERT_NOT_NULL(f);
    unsigned char bytes[8];
    size_t count = fread(bytes, 1, sizeof(bytes), f);
    fclose(f);
    TEST_ASSERT_EQUAL_UINT(7, count);
    for (size_t i = 0; i < count; ++i) TEST_ASSERT_EQUAL_HEX8(0xAB, bytes[i]);
}

static void test_cursor_head_write_does_not_overwrite_link_targets(void) {
    char target[512], tmp[512], final[512];
    snprintf(target, sizeof(target), "%s/sidecar-link-target", storage);
    snprintf(tmp, sizeof(tmp), "%s/observations/%s.tmp", storage, OBS_CURSOR_HEAD_FILE);
    snprintf(final, sizeof(final), "%s/observations/%s", storage, OBS_CURSOR_HEAD_FILE);
    write_file(target, 7);
    write_file(final, 1);  /* Create the evidence root. */
    for (int hardlink = 0; hardlink < 2; ++hardlink) {
        TEST_ASSERT_EQUAL_INT(0, unlink(final));
        TEST_ASSERT_EQUAL_INT(0, hardlink ? link(target, tmp) : symlink(target, tmp));
        TEST_ASSERT_EQUAL_INT(0, hardlink ? link(target, final) : symlink(target, final));
        TEST_ASSERT_EQUAL_INT(0, observation_cursor_head_write(storage, cam_a, 42 + hardlink));
        assert_marker_untouched(target);
        char epoch[OBS_UUID_SIZE];
        int64_t head;
        TEST_ASSERT_EQUAL_INT(0, observation_cursor_head_read(storage, epoch, &head));
        TEST_ASSERT_EQUAL_STRING(cam_a, epoch);
        TEST_ASSERT_EQUAL_INT64(42 + hardlink, head);
        TEST_ASSERT_EQUAL_INT(0, unlink(tmp));
    }
    TEST_ASSERT_EQUAL_INT(0, unlink(target));
}

static void assert_only_cursor_head_remains(void) {
    char root[512];
    snprintf(root, sizeof(root), "%s/observations", storage);
    DIR *d = opendir(root);
    TEST_ASSERT_NOT_NULL(d);
    int extra = 0;
    struct dirent *entry;
    while ((entry = readdir(d))) {
        if (strcmp(entry->d_name, ".") && strcmp(entry->d_name, "..") &&
            strcmp(entry->d_name, OBS_CURSOR_HEAD_FILE)) ++extra;
    }
    closedir(d);
    TEST_ASSERT_EQUAL_INT(0, extra);
}

static void test_cursor_head_write_failure_cleans_temp_and_preserves_previous_head(void) {
    TEST_ASSERT_EQUAL_INT(0, observation_cursor_head_write(storage, cam_a, 41));
    fail_sidecar_rename = true;
    int rc = observation_cursor_head_write(storage, cam_a, 42);
    fail_sidecar_rename = false;
    TEST_ASSERT_EQUAL_INT(-1, rc);
    char epoch[OBS_UUID_SIZE];
    int64_t head;
    TEST_ASSERT_EQUAL_INT(0, observation_cursor_head_read(storage, epoch, &head));
    TEST_ASSERT_EQUAL_INT64(41, head);
    assert_only_cursor_head_remains();
    TEST_ASSERT_EQUAL_INT(0, observation_cursor_head_write(storage, cam_a, 42));
    assert_only_cursor_head_remains();
    fail_sidecar_dir_sync = true;
    rc = observation_cursor_head_write(storage, cam_a, 43);
    fail_sidecar_dir_sync = false;
    TEST_ASSERT_EQUAL_INT(-1, rc);  /* A rename alone does not prove durability. */
    assert_only_cursor_head_remains();
}

static void test_cursor_head_rename_and_sync_stay_in_opened_root(void) {
    char root[512], moved[512], outside[512], outside_file[1024], moved_file[1024];
    snprintf(root, sizeof(root), "%s/observations", storage);
    snprintf(moved, sizeof(moved), "%s/sidecar-moved", storage);
    snprintf(outside, sizeof(outside), "%s/sidecar-outside", storage);
    snprintf(outside_file, sizeof(outside_file), "%s/%s", outside, OBS_CURSOR_HEAD_FILE);
    snprintf(moved_file, sizeof(moved_file), "%s/%s", moved, OBS_CURSOR_HEAD_FILE);
    write_file(outside_file, 7);
    sidecar_root = root;
    sidecar_moved = moved;
    sidecar_outside = outside;
    swap_sidecar_on_rename = true;
    int rc = observation_cursor_head_write(storage, cam_a, 42);
    swap_sidecar_on_rename = false;
    TEST_ASSERT_EQUAL_INT(0, rc);
    TEST_ASSERT_TRUE(swap_completed);
    TEST_ASSERT_TRUE(sidecar_dir_synced);
    TEST_ASSERT_TRUE(exists(moved_file));
    assert_marker_untouched(outside_file);
    TEST_ASSERT_EQUAL_INT(0, unlink(root));
    /* A trusted root configured as a symlink remains supported. */
    TEST_ASSERT_EQUAL_INT(0, symlink(moved, root));
    TEST_ASSERT_EQUAL_INT(0, observation_cursor_head_write(storage, cam_a, 43));
    char epoch[OBS_UUID_SIZE];
    int64_t head;
    TEST_ASSERT_EQUAL_INT(0, observation_cursor_head_read(storage, epoch, &head));
    TEST_ASSERT_EQUAL_INT64(43, head);
    TEST_ASSERT_EQUAL_INT(0, unlink(root));
    rm_tree(moved);
    rm_tree(outside);
}

static void test_cursor_head_sidecar_detects_rewind(void) {
    insert_batch(cam_a, now_ms, 1, 0);
    observation_meta_t meta;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_meta_get(&meta));

    /* Missing sidecar: created, epoch kept. */
    bool regenerated = true;
    TEST_ASSERT_EQUAL_INT(0, observation_cursor_head_reconcile(storage, &regenerated));
    TEST_ASSERT_FALSE(regenerated);
    char epoch[OBS_UUID_SIZE];
    int64_t head = -1;
    TEST_ASSERT_EQUAL_INT(0, observation_cursor_head_read(storage, epoch, &head));
    TEST_ASSERT_EQUAL_STRING(meta.epoch_uuid, epoch);
    TEST_ASSERT_EQUAL_INT64(meta.head_seq, head);

    /* Sidecar is ahead of the DB in the same epoch: the DB was restored. */
    TEST_ASSERT_EQUAL_INT(0, observation_cursor_head_write(storage, meta.epoch_uuid,
                                                           meta.head_seq + 100));
    TEST_ASSERT_EQUAL_INT(0, observation_cursor_head_reconcile(storage, &regenerated));
    TEST_ASSERT_TRUE(regenerated);
    observation_meta_t now_meta;
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_meta_get(&now_meta));
    TEST_ASSERT_NOT_EQUAL(0, strcmp(meta.epoch_uuid, now_meta.epoch_uuid));
    TEST_ASSERT_EQUAL_INT(0, observation_cursor_head_read(storage, epoch, &head));
    TEST_ASSERT_EQUAL_STRING(now_meta.epoch_uuid, epoch);
    TEST_ASSERT_EQUAL_INT64(now_meta.head_seq, head);

    /* A different epoch (already regenerated in-process): no regeneration. */
    TEST_ASSERT_EQUAL_INT(0, observation_cursor_head_write(
        storage, "66666666-6666-4666-8666-666666666666", now_meta.head_seq + 100));
    TEST_ASSERT_EQUAL_INT(0, observation_cursor_head_reconcile(storage, &regenerated));
    TEST_ASSERT_FALSE(regenerated);
    TEST_ASSERT_EQUAL_INT(DB_OBS_OK, db_observation_meta_get(&meta));
    TEST_ASSERT_EQUAL_STRING(now_meta.epoch_uuid, meta.epoch_uuid);

    /* A corrupt sidecar is treated as missing. */
    char path[512];
    snprintf(path, sizeof(path), "%s/observations/.cursor_head.json", storage);
    write_file(path, 5);
    TEST_ASSERT_EQUAL_INT(0, observation_cursor_head_reconcile(storage, &regenerated));
    TEST_ASSERT_FALSE(regenerated);
    TEST_ASSERT_EQUAL_INT(0, observation_cursor_head_read(storage, epoch, &head));
    TEST_ASSERT_EQUAL_STRING(meta.epoch_uuid, epoch);
}

int main(void) {
    const char *tmp = getenv("TMPDIR");
    snprintf(storage, sizeof(storage), "%s/lightnvr_obs_retention_XXXXXX",
             tmp && *tmp ? tmp : "/tmp");
    if (!mkdtemp(storage)) return 1;
    snprintf(db_path, sizeof(db_path), "%s/test.db", storage);
    if (init_database(db_path) != 0) return 1;
    if (event_identity_init() != 0) return 1;
    now_ms = (int64_t)time(NULL) * 1000;
    safe_strcpy(g_config.storage_path, storage, sizeof(g_config.storage_path), 0);

    UNITY_BEGIN();
    make_camera("cam_a", cam_a);
    make_camera("cam_b", cam_b);
    RUN_TEST(test_startup_reconciles_interrupted_missing_and_orphan_files);
    RUN_TEST(test_startup_sweep_does_not_follow_symlinks_below_root);
    RUN_TEST(test_startup_sweep_follows_a_symlinked_evidence_root);
    RUN_TEST(test_startup_preserves_all_supported_asset_path_forms);
    RUN_TEST(test_startup_rejects_symlink_parents_and_nonregular_leaves);
    RUN_TEST(test_maintenance_drains_partial_age_pages_and_serializes_ticks);
    RUN_TEST(test_periodic_run_waits_for_startup_reconciliation);
    RUN_TEST(test_failed_restart_disables_retention_until_reconciled);
    RUN_TEST(test_retention_preserves_outside_targets_of_parent_and_leaf_symlinks);
    RUN_TEST(test_retention_parent_swap_cannot_redirect_unlink);
    RUN_TEST(test_retention_accepts_path_forms_and_symlinked_root);
    RUN_TEST(test_retention_missing_root_clears_completed_cleanup);
    RUN_TEST(test_age_budget_expires_media_and_keeps_metadata);
    RUN_TEST(test_shared_asset_survives_until_last_reference_expires);
    RUN_TEST(test_byte_budget_evicts_oldest_and_protected_assets_flip_the_flag);
    RUN_TEST(test_history_prune_advances_oldest_seq_and_reclaims_assets);
    RUN_TEST(test_failed_expiry_unlink_is_retried_without_restart);
    RUN_TEST(test_prune_preserves_failed_unlink_for_retry);
    RUN_TEST(test_recording_deletion_leaves_assets_intact);
    RUN_TEST(test_cursor_head_write_recovers_with_stale_temporary_files);
    RUN_TEST(test_cursor_head_write_does_not_overwrite_link_targets);
    RUN_TEST(test_cursor_head_write_failure_cleans_temp_and_preserves_previous_head);
    RUN_TEST(test_cursor_head_rename_and_sync_stay_in_opened_root);
    RUN_TEST(test_cursor_head_sidecar_detects_rewind);
    int result = UNITY_END();

    observation_retention_reset_for_testing();
    event_identity_shutdown();
    shutdown_database();
    rm_tree(storage);
    return result;
}
