/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

/*
 * INT_01 L4: startup reconciliation, retention and eviction of observation
 * evidence. See include/core/observation_retention.h and
 * docs/integrations/OBSERVATIONS_CONTRACT.md sections 4.2, 7.6 and 14.
 *
 * State changes go through the L2 compare-and-set transitions, so every
 * referencing observation gets an evidence revision and a publication. Reads
 * use a private read-only connection and never hold the global DB mutex; the
 * only raw write here (deleting an unreferenced asset row) follows the
 * db_auth.c pattern with a single statement.
 */

#define _POSIX_C_SOURCE 200809L

#include "core/observation_retention.h"

#include <cjson/cJSON.h>
#include <dirent.h>
#include <errno.h>
#include <fcntl.h>
#include <limits.h>
#include <pthread.h>
#include <stdatomic.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <time.h>
#include <unistd.h>

#include "core/config.h"
#include "core/logger.h"
#include "core/path_utils.h"
#include "database/db_core.h"
#include "database/db_system_settings.h"
#include "utils/strings.h"
#include "utils/uuid.h"

#define DAY_MS 86400000LL
#define OBS_PATH_SIZE 1024
#define OBS_WALK_MAX_DEPTH 4       /* observations/<camera>/<date>/<file> */
#define OBS_PAGE 256

/* ---- module state ------------------------------------------------------ */

static pthread_mutex_t g_run_mutex = PTHREAD_MUTEX_INITIALIZER;   /* startup vs periodic */
static atomic_int g_startup_done = 0;  /* 0/1: atomic_bool breaks the C->Rust conversion */

static pthread_mutex_t g_status_mutex = PTHREAD_MUTEX_INITIALIZER;
static observation_retention_camera_status_t *g_status = NULL;
static int g_status_count = 0;
static atomic_int g_exhausted_count = 0;

static time_t g_last_tick = 0;
static bool g_more_work = false;

/* ---- paths --------------------------------------------------------------- */

static int root_dir(const char *storage_path, char *out, size_t size) {
    if (!storage_path || !*storage_path) return -1;
    size_t len = strlen(storage_path);
    while (len > 1 && storage_path[len - 1] == '/') --len;
    int n = snprintf(out, size, "%.*s/" OBS_RETENTION_DIR, (int)len, storage_path);
    return (n > 0 && (size_t)n < size) ? 0 : -1;
}

static bool has_suffix(const char *value, const char *suffix) {
    size_t a = strlen(value), b = strlen(suffix);
    return a >= b && strcmp(value + a - b, suffix) == 0;
}

/* No empty, "." or ".." segment: a stored path can never escape the tree. */
static bool safe_relative(const char *rel) {
    if (!*rel || *rel == '/') return false;
    const char *segment = rel;
    for (;;) {
        const char *end = strchr(segment, '/');
        size_t len = end ? (size_t)(end - segment) : strlen(segment);
        if (len == 0 || (len == 1 && segment[0] == '.') ||
            (len == 2 && segment[0] == '.' && segment[1] == '.'))
            return false;
        if (!end) return true;
        segment = end + 1;
    }
}

/* Map observation_assets.path to an absolute file inside
 * <storage>/observations, or fail. Only "<...>.jpg" files qualify. */
static int resolve_asset_path(const char *root, const char *stored, char *out, size_t size) {
    if (!stored || !*stored || !has_suffix(stored, ".jpg")) return -1;
    const char *rel = stored;
    size_t root_len = strlen(root);
    if (stored[0] == '/') {
        if (strncmp(stored, root, root_len) != 0 || stored[root_len] != '/') return -1;
        rel = stored + root_len + 1;
    } else if (strncmp(stored, OBS_RETENTION_DIR "/", sizeof(OBS_RETENTION_DIR)) == 0) {
        rel = stored + sizeof(OBS_RETENTION_DIR);
    }
    if (!safe_relative(rel)) return -1;
    int n = snprintf(out, size, "%s/%s", root, rel);
    return (n > 0 && (size_t)n < size) ? 0 : -1;
}

/* Resolve parents through directory descriptors, never following symlinks
 * below the trusted root. A missing path is already cleaned up. The leaf is
 * unlinked relative to its open parent, so replacing that parent's pathname
 * cannot redirect deletion to another directory. */
static int remove_file_under_root(const char *root, const char *rel, int *deleted) {
    char work[OBS_PATH_SIZE];
    int n = snprintf(work, sizeof(work), "%s", rel);
    if (n <= 0 || (size_t)n >= sizeof(work)) return -1;

    /* As in the startup sweep, the configured evidence root may be a symlink. */
    int dir_fd = open(root, O_RDONLY | O_DIRECTORY | O_CLOEXEC);
    if (dir_fd < 0) {
        if (errno == ENOENT || errno == ENOTDIR) return 0;
        log_warn("Failed to open observation root %s: %s", root, strerror(errno));
        return -1;
    }

    int rc = -1;
    char *part = work;
    for (;;) {
        char *slash = strchr(part, '/');
        if (!slash) break;
        *slash = '\0';
        int next_fd = openat(dir_fd, part, O_RDONLY | O_DIRECTORY | O_NOFOLLOW | O_CLOEXEC);
        if (next_fd < 0) {
            if (errno == ENOENT || errno == ENOTDIR) rc = 0;
            else log_warn("Failed to traverse observation evidence path %s/%s: %s",
                          root, rel, strerror(errno));
            goto done;
        }
        close(dir_fd);
        dir_fd = next_fd;
        part = slash + 1;
    }

    /* unlinkat removes a leaf symlink itself, never its target. */
    if (unlinkat(dir_fd, part, 0) == 0) {
        if (deleted) ++*deleted;
        rc = 0;
    } else if (errno == ENOENT || errno == ENOTDIR) {
        rc = 0;
    } else {
        log_warn("Failed to delete observation evidence file %s/%s: %s", root, rel, strerror(errno));
    }

done:
    close(dir_fd);
    return rc;
}

static int remove_asset_file(const char *root, const char *stored, int *deleted) {
    if (!stored || !*stored) return 0;
    char path[OBS_PATH_SIZE];
    if (resolve_asset_path(root, stored, path, sizeof(path)) != 0) {
        log_warn("Observation asset path is outside the evidence tree; not deleting");
        /* A corrupt path is never a file this subsystem may remove. */
        return 0;
    }

    return remove_file_under_root(root, path + strlen(root) + 1, deleted);
}

static int64_t wall_ms(void) {
    struct timespec ts;
    clock_gettime(CLOCK_REALTIME, &ts);
    return (int64_t)ts.tv_sec * 1000 + ts.tv_nsec / 1000000;
}

static void copy_text(sqlite3_stmt *stmt, int column, char *out, size_t size) {
    const unsigned char *text = sqlite3_column_text(stmt, column);
    safe_strcpy(out, text ? (const char *)text : "", size, 0);
}

/* ---- cursor head sidecar (D9) ------------------------------------------ */

int observation_cursor_head_write(const char *storage_path, const char *epoch_uuid,
                                  int64_t head_seq) {
    char root[OBS_PATH_SIZE];
    if (!lightnvr_uuid_is_valid(epoch_uuid) || head_seq < 0 ||
        root_dir(storage_path, root, sizeof(root)) != 0)
        return -1;
    /* The reader still builds the final pathname in an OBS_PATH_SIZE buffer. */
    if (strlen(root) + sizeof("/" OBS_CURSOR_HEAD_FILE) > OBS_PATH_SIZE) return -1;
    if (mkdir_recursive(root) != 0) return -1;

    char body[160];
    int len = snprintf(body, sizeof(body), "{\"epoch\":\"%s\",\"head_seq\":%lld}\n",
                       epoch_uuid, (long long)head_seq);

    /* Each attempt owns a fresh name. A crash-left temporary file or another
     * writer must not block future checkpoints or be truncated/unlinked. */
    char uuid[LIGHTNVR_UUID_STRING_SIZE];
    char tmp[sizeof(OBS_CURSOR_HEAD_FILE) + LIGHTNVR_UUID_STRING_SIZE + 5];
    if (lightnvr_uuid_generate_v4(uuid) != 0) return -1;
    snprintf(tmp, sizeof(tmp), "%s.%s.tmp", OBS_CURSOR_HEAD_FILE, uuid);

    int dfd = open(root, O_RDONLY | O_DIRECTORY | O_CLOEXEC);
    if (dfd < 0) return -1;

    int fd = openat(dfd, tmp,
                    O_WRONLY | O_CREAT | O_EXCL | O_NOFOLLOW | O_CLOEXEC, 0640);
    if (fd < 0) {
        (void)close(dfd);
        return -1;
    }

    int rc = (write(fd, body, (size_t)len) == len && fsync(fd) == 0) ? 0 : -1;
    if (close(fd) != 0) rc = -1;
    if (rc == 0 &&
        renameat(dfd, tmp, dfd, OBS_CURSOR_HEAD_FILE) != 0)
        rc = -1;
    if (rc != 0) {
        (void)unlinkat(dfd, tmp, 0);
        log_warn("Failed to write observation cursor head sidecar");
        (void)close(dfd);
        return -1;
    }

    /* Sync the same directory used for creation and rename, even if its
     * configured pathname was replaced while writing. */
    rc = fsync(dfd);
    if (close(dfd) != 0) rc = -1;
    return rc;
}

int observation_cursor_head_read(const char *storage_path, char epoch_uuid[OBS_UUID_SIZE],
                                 int64_t *head_seq) {
    char root[OBS_PATH_SIZE], path[OBS_PATH_SIZE];
    if (!epoch_uuid || !head_seq || root_dir(storage_path, root, sizeof(root)) != 0) return -1;
    if (snprintf(path, sizeof(path), "%s/" OBS_CURSOR_HEAD_FILE, root) >= (int)sizeof(path))
        return -1;
    FILE *f = fopen(path, "rb");
    if (!f) return errno == ENOENT ? 1 : -1;
    char body[512];
    size_t n = fread(body, 1, sizeof(body) - 1, f);
    fclose(f);
    body[n] = '\0';
    cJSON *json = cJSON_Parse(body);
    int rc = -1;
    if (json) {
        const cJSON *epoch = cJSON_GetObjectItemCaseSensitive(json, "epoch");
        const cJSON *head = cJSON_GetObjectItemCaseSensitive(json, "head_seq");
        if (cJSON_IsString(epoch) && lightnvr_uuid_is_valid(epoch->valuestring) &&
            cJSON_IsNumber(head) && head->valuedouble >= 0 &&
            head->valuedouble <= 9007199254740991.0 &&
            head->valuedouble == (double)(int64_t)head->valuedouble) {
            safe_strcpy(epoch_uuid, epoch->valuestring, OBS_UUID_SIZE, 0);
            *head_seq = (int64_t)head->valuedouble;
            rc = 0;
        }
        cJSON_Delete(json);
    }
    return rc;
}

int observation_cursor_head_reconcile(const char *storage_path, bool *regenerated) {
    if (regenerated) *regenerated = false;
    observation_meta_t meta;
    if (db_observation_meta_get(&meta) != DB_OBS_OK) return -1;

    char epoch[OBS_UUID_SIZE];
    int64_t head = 0;
    int rc = observation_cursor_head_read(storage_path, epoch, &head);
    if (rc < 0) log_warn("Observation cursor head sidecar is malformed; recreating it");
    if (rc == 0 && strcmp(epoch, meta.epoch_uuid) == 0 && meta.head_seq < head) {
        log_warn("Observation history rewound (DB head %lld < recorded head %lld): "
                 "database was restored; regenerating cursor epoch",
                 (long long)meta.head_seq, (long long)head);
        if (db_observation_meta_regenerate_epoch() != DB_OBS_OK ||
            db_observation_meta_get(&meta) != DB_OBS_OK)
            return -1;
        if (regenerated) *regenerated = true;
    }
    return observation_cursor_head_write(storage_path, meta.epoch_uuid, meta.head_seq);
}

/* ---- startup sweep ----------------------------------------------------- */

typedef struct {
    sqlite3 *db;
    sqlite3_stmt *lookup;
    const char *root;
    observation_reconcile_result_t *result;
    int errors;
} sweep_t;

/* Remove dir_fd/name, counting it. dir_fd-relative so a renamed or swapped
 * parent directory cannot redirect the delete outside the evidence root. */
static void remove_at(int dir_fd, const char *name, const char *path, int *deleted) {
    if (unlinkat(dir_fd, name, 0) == 0) {
        if (deleted) ++*deleted;
        return;
    }
    if (errno == ENOENT || errno == ENOTDIR) return;
    log_warn("Failed to delete observation evidence file %s: %s", path, strerror(errno));
}

/* <uuid>.jpg: keep only the file a ready asset row points at. No worker runs
 * yet, so a final file of a pending asset is an unfinished write. */
static void sweep_final(sweep_t *s, int dir_fd, const char *path, const char *name) {
    char uuid[OBS_UUID_SIZE];
    size_t len = strlen(name) - 4;  /* ".jpg" */
    if (len != OBS_UUID_SIZE - 1) return;
    memcpy(uuid, name, len);
    uuid[len] = '\0';
    if (!lightnvr_uuid_is_valid(uuid)) return;

    sqlite3_reset(s->lookup);
    sqlite3_bind_text(s->lookup, 1, uuid, -1, SQLITE_TRANSIENT);
    int step = sqlite3_step(s->lookup);
    if (step == SQLITE_ROW) {
        char state[16], stored[MAX_PATH_LENGTH], expected[OBS_PATH_SIZE];
        copy_text(s->lookup, 0, state, sizeof(state));
        copy_text(s->lookup, 1, stored, sizeof(stored));
        if (strcmp(state, "ready") == 0 &&
            resolve_asset_path(s->root, stored, expected, sizeof(expected)) == 0 &&
            strcmp(expected, path) == 0)
            return;
    } else if (step != SQLITE_DONE) {
        ++s->errors;
        return;
    }
    remove_at(dir_fd, name, path, &s->result->stale_files_deleted);
}

/* Walk an already-open directory. Every check and action is relative to the
 * directory's fd and never follows symlinks, so an entry swapped for a
 * symlink between the check and the action cannot escape the evidence root.
 * Takes ownership of dir_fd. `dir` is used for logging and path comparison. */
static void sweep_dir_fd(sweep_t *s, int dir_fd, const char *dir, int depth) {
    DIR *d = fdopendir(dir_fd);
    if (!d) {
        log_warn("Cannot scan evidence directory %s: %s", dir, strerror(errno));
        close(dir_fd);
        return;
    }
    int fd = dirfd(d);
    struct dirent *entry;
    while ((entry = readdir(d)) != NULL) {
        const char *name = entry->d_name;
        if (strcmp(name, ".") == 0 || strcmp(name, "..") == 0) continue;
        char path[OBS_PATH_SIZE];
        if (snprintf(path, sizeof(path), "%s/%s", dir, name) >= (int)sizeof(path)) continue;
        struct stat st;
        if (fstatat(fd, name, &st, AT_SYMLINK_NOFOLLOW) != 0) continue;
        if (S_ISDIR(st.st_mode)) {
            if (depth < OBS_WALK_MAX_DEPTH) {
                int child = openat(fd, name, O_RDONLY | O_DIRECTORY | O_NOFOLLOW | O_CLOEXEC);
                if (child < 0) continue;
                sweep_dir_fd(s, child, path, depth + 1);
                if (depth > 1) (void)unlinkat(fd, name, AT_REMOVEDIR);  /* only succeeds when empty */
            }
            continue;
        }
        if (!S_ISREG(st.st_mode)) continue;  /* symlinks and specials are left alone */
        if (has_suffix(name, ".jpg.tmp")) {
            remove_at(fd, name, path, &s->result->tmp_files_deleted);
        } else if (has_suffix(name, ".jpg")) {
            sweep_final(s, fd, path, name);
        }
    }
    closedir(d);
}

/* The evidence root comes from trusted configuration and may itself be a
 * symlink (for example a mounted storage path), so it is followed; nothing
 * below it is. */
static void sweep_dir(sweep_t *s, const char *dir, int depth) {
    int fd = open(dir, O_RDONLY | O_DIRECTORY | O_CLOEXEC);
    if (fd < 0) {
        if (errno != ENOENT) log_warn("Cannot scan evidence directory %s: %s", dir, strerror(errno));
        return;
    }
    sweep_dir_fd(s, fd, dir, depth);
}

static int sweep_files(const char *root, observation_reconcile_result_t *result) {
    sweep_t s = {.root = root, .result = result};
    if (db_open_readonly_connection(&s.db) != 0) return -1;
    if (sqlite3_prepare_v2(s.db, "SELECT state, path FROM observation_assets WHERE uuid = ?;",
                           -1, &s.lookup, NULL) != SQLITE_OK) {
        db_close_readonly_connection(s.db);
        return -1;
    }
    sweep_dir(&s, root, 1);
    sqlite3_finalize(s.lookup);
    db_close_readonly_connection(s.db);
    return s.errors ? -1 : 0;
}

typedef struct {
    char uuid[OBS_UUID_SIZE];
    char path[MAX_PATH_LENGTH];
    int64_t bytes;
    int64_t ref_ms;
    char state[16];
} asset_row_t;

/* Keyset page of assets in one state, uuid > after. Returns the count or -1. */
static int page_by_state(const char *state, const char *after, asset_row_t *rows, int max) {
    sqlite3 *db = NULL;
    if (db_open_readonly_connection(&db) != 0) return -1;
    sqlite3_stmt *stmt = NULL;
    int count = -1;
    if (sqlite3_prepare_v2(db,
            "SELECT uuid, COALESCE(path, '') FROM observation_assets "
            "WHERE state = ?1 AND uuid > ?2 ORDER BY uuid LIMIT ?3;", -1, &stmt, NULL) ==
        SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, state, -1, SQLITE_STATIC);
        sqlite3_bind_text(stmt, 2, after, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int(stmt, 3, max);
        count = 0;
        int step;
        while (count < max && (step = sqlite3_step(stmt)) == SQLITE_ROW) {
            memset(&rows[count], 0, sizeof(rows[count]));
            copy_text(stmt, 0, rows[count].uuid, sizeof(rows[count].uuid));
            copy_text(stmt, 1, rows[count].path, sizeof(rows[count].path));
            ++count;
        }
        if (count < max && step != SQLITE_DONE) count = -1;
    }
    sqlite3_finalize(stmt);
    db_close_readonly_connection(db);
    return count;
}

static int asset_is_regular_nofollow(const char *root, const char *stored) {
    /* Preserve all supported stored-path forms and reject traversal, empty
     * segments, wrong extensions and truncation before walking the tree. */
    char path[OBS_PATH_SIZE];
    if (resolve_asset_path(root, stored, path, sizeof(path)) != 0) return 0;

    int dfd = open(root, O_RDONLY | O_DIRECTORY | O_CLOEXEC);
    if (dfd < 0) return 0;

    char *save = NULL;
    char *part = strtok_r(path + strlen(root) + 1, "/", &save);
    if (!part) {
        close(dfd);
        return 0;
    }

    int ok = 0;
    for (;;) {
        char *next = strtok_r(NULL, "/", &save);
        if (next) {
            int nfd = openat(dfd, part, O_RDONLY | O_DIRECTORY | O_NOFOLLOW | O_CLOEXEC);
            if (nfd < 0) break;
            close(dfd);
            dfd = nfd;
            part = next;
            continue;
        }

        /* Inspect the leaf without opening it: a corrupt row can point to a
         * FIFO or device, which must neither block startup nor be read. */
        struct stat st;
        ok = fstatat(dfd, part, &st, AT_SYMLINK_NOFOLLOW) == 0 && S_ISREG(st.st_mode);
        break;
    }

    close(dfd);
    return ok;
}

static int reconcile_state(const char *root, observation_asset_state_t from,
                           observation_reconcile_result_t *result) {
    asset_row_t *rows = calloc(OBS_PAGE, sizeof(*rows));
    if (!rows) return -1;
    char after[OBS_UUID_SIZE] = "";
    int errors = 0;
    for (;;) {
        int n = page_by_state(observation_asset_state_name(from), after, rows, OBS_PAGE);
        if (n < 0) { ++errors; break; }
        for (int i = 0; i < n; ++i) {
            const char *reason = NULL;
            if (from == OBS_ASSET_PENDING) {
                reason = "interrupted";
            } else {
                if (!asset_is_regular_nofollow(root, rows[i].path))
                    reason = "write_failed";
            }
            if (!reason) continue;
            int rc = db_observation_asset_transition(rows[i].uuid, from, OBS_ASSET_UNAVAILABLE,
                                                     reason, NULL, NULL);
            if (rc == DB_OBS_OK) {
                if (from == OBS_ASSET_PENDING) ++result->pending_interrupted;
                else ++result->ready_missing;
            } else if (rc != DB_OBS_CAS_LOST && rc != DB_OBS_NOT_FOUND) {
                ++errors;
            }
        }
        if (n < OBS_PAGE) break;
        safe_strcpy(after, rows[n - 1].uuid, sizeof(after), 0);
    }
    free(rows);
    return errors ? -1 : 0;
}

int observation_retention_startup(const char *storage_path,
                                  observation_reconcile_result_t *result) {
    observation_reconcile_result_t local;
    if (!result) result = &local;
    memset(result, 0, sizeof(*result));
    pthread_mutex_lock(&g_run_mutex);
    atomic_store(&g_startup_done, 0);
    char root[OBS_PATH_SIZE];
    if (root_dir(storage_path, root, sizeof(root)) != 0) {
        pthread_mutex_unlock(&g_run_mutex);
        return -1;
    }
    int rc = 0;
    /* Before any transition here advances the head (section 7.6). */
    if (observation_cursor_head_reconcile(storage_path, &result->epoch_regenerated) != 0) {
        log_warn("Observation cursor head reconciliation failed");
        rc = -1;
    }
    if (sweep_files(root, result) != 0) rc = -1;
    if (reconcile_state(root, OBS_ASSET_PENDING, result) != 0) rc = -1;
    if (reconcile_state(root, OBS_ASSET_READY, result) != 0) rc = -1;
    if (rc == 0) {
        observation_meta_t meta;
        if (db_observation_meta_get(&meta) == DB_OBS_OK)
            (void)observation_cursor_head_write(storage_path, meta.epoch_uuid, meta.head_seq);
        atomic_store(&g_startup_done, 1);
    }
    pthread_mutex_unlock(&g_run_mutex);

    if (result->tmp_files_deleted || result->stale_files_deleted ||
        result->pending_interrupted || result->ready_missing || result->epoch_regenerated)
        log_info("Observation reconciliation: %d temp and %d stale files deleted, %d pending "
                 "interrupted, %d ready missing%s", result->tmp_files_deleted,
                 result->stale_files_deleted, result->pending_interrupted,
                 result->ready_missing, result->epoch_regenerated ? ", epoch regenerated" : "");
    if (rc != 0) log_error("Observation startup reconciliation incomplete; eviction disabled");
    return rc;
}

bool observation_retention_startup_done(void) {
    return atomic_load(&g_startup_done);
}

/* ---- periodic pass ----------------------------------------------------- */

static int history_days(void) {
    char value[32];
    if (db_get_system_setting("observation_history_retention_days", value, sizeof(value)) != 0)
        return OBS_RETENTION_DEFAULT_HISTORY_DAYS;
    char *end = NULL;
    long days = strtol(value, &end, 10);
    if (end == value || days < 1 || days > 365) return OBS_RETENTION_DEFAULT_HISTORY_DAYS;
    return (int)days;
}

static int prune_history(int64_t now_ms, observation_retention_result_t *result) {
    int64_t before_ms = now_ms - (int64_t)history_days() * DAY_MS;
    while (result->changes_pruned < OBS_RETENTION_MAX_PRUNED_CHANGES) {
        observation_prune_result_t prune;
        if (db_observations_prune(before_ms, OBS_RETENTION_PRUNE_BATCH, &prune) != DB_OBS_OK)
            return -1;
        result->changes_pruned += prune.changes_deleted;
        result->observations_pruned += prune.observations_deleted;
        result->unreferenced_deleted += prune.assets_deleted;
        if (prune.changes_deleted < OBS_RETENTION_PRUNE_BATCH) return 0;
    }
    result->more_work = true;
    return 0;
}

/* Single-statement delete of an asset row nobody references (db_auth.c
 * locking: global mutex, then the connection mutex). */
static int delete_asset_row(const char *uuid) {
    pthread_mutex_t *mutex = get_db_mutex();
    if (!mutex) return -1;
    pthread_mutex_lock(mutex);
    sqlite3 *db = get_db_handle();
    int rc = -1;
    if (db) {
        sqlite3_mutex *connection = sqlite3_db_mutex(db);
        sqlite3_mutex_enter(connection);
        sqlite3_stmt *stmt = NULL;
        if (sqlite3_prepare_v2(db,
                "DELETE FROM observation_assets WHERE uuid = ?1 AND protected = 0 AND "
                "(path IS NULL OR path = '') AND "
                "state IN ('unavailable', 'expired') AND NOT EXISTS "
                "(SELECT 1 FROM object_observations WHERE asset_uuid = ?1);",
                -1, &stmt, NULL) == SQLITE_OK) {
            sqlite3_bind_text(stmt, 1, uuid, -1, SQLITE_TRANSIENT);
            if (sqlite3_step(stmt) == SQLITE_DONE) rc = sqlite3_changes(db);
        }
        sqlite3_finalize(stmt);
        sqlite3_mutex_leave(connection);
    }
    pthread_mutex_unlock(mutex);
    return rc;
}

/* Keep the path as a durable cleanup obligation until unlink succeeds. The
 * public media metadata stays intact; path is internal and is never served
 * for a terminal asset. Retrying after a crash is safe (ENOENT is success). */
static int cleanup_asset_file(const char *root, const asset_row_t *row, int *deleted) {
    if (!row->path[0]) return 0;
    if (remove_asset_file(root, row->path, deleted) != 0) return -1;
    pthread_mutex_t *mutex = get_db_mutex();
    if (!mutex) return -1;
    pthread_mutex_lock(mutex);
    sqlite3 *db = get_db_handle();
    int rc = -1;
    if (db) {
        sqlite3_mutex *connection = sqlite3_db_mutex(db);
        sqlite3_mutex_enter(connection);
        sqlite3_stmt *stmt = NULL;
        if (sqlite3_prepare_v2(db,
                "UPDATE observation_assets SET path = NULL WHERE uuid = ?1 "
                "AND path = ?2 AND state IN ('unavailable', 'expired');",
                -1, &stmt, NULL) == SQLITE_OK) {
            sqlite3_bind_text(stmt, 1, row->uuid, -1, SQLITE_TRANSIENT);
            sqlite3_bind_text(stmt, 2, row->path, -1, SQLITE_TRANSIENT);
            if (sqlite3_step(stmt) == SQLITE_DONE) rc = 0;
        }
        sqlite3_finalize(stmt);
        sqlite3_mutex_leave(connection);
    }
    pthread_mutex_unlock(mutex);
    return rc;
}

/* ready -> expired(retention_expired), then unlink. Returns 1 when expired,
 * 0 when another writer moved the asset first, -1 on error. */
static int expire_asset(const char *root, const asset_row_t *row,
                        observation_retention_result_t *result) {
    int rc = db_observation_asset_transition(row->uuid, OBS_ASSET_READY, OBS_ASSET_EXPIRED,
                                             "retention_expired", NULL, NULL);
    if (rc == DB_OBS_CAS_LOST || rc == DB_OBS_NOT_FOUND) return 0;
    if (rc != DB_OBS_OK) return -1;
    ++result->assets_expired;
    if (cleanup_asset_file(root, row, &result->files_deleted) != 0) return -1;
    return 1;
}

static int query_rows(const char *sql, const char *camera, int64_t bound, int max,
                      asset_row_t *rows) {
    sqlite3 *db = NULL;
    if (db_open_readonly_connection(&db) != 0) return -1;
    sqlite3_stmt *stmt = NULL;
    int count = -1;
    if (sqlite3_prepare_v2(db, sql, -1, &stmt, NULL) == SQLITE_OK) {
        int index = 1;
        if (camera) sqlite3_bind_text(stmt, index++, camera, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int64(stmt, index++, bound);
        sqlite3_bind_int(stmt, index, max);
        count = 0;
        int step = SQLITE_DONE;
        while (count < max && (step = sqlite3_step(stmt)) == SQLITE_ROW) {
            asset_row_t *row = &rows[count++];
            memset(row, 0, sizeof(*row));
            copy_text(stmt, 0, row->uuid, sizeof(row->uuid));
            copy_text(stmt, 1, row->path, sizeof(row->path));
            row->bytes = sqlite3_column_int64(stmt, 2);
            row->ref_ms = sqlite3_column_int64(stmt, 3);
            copy_text(stmt, 4, row->state, sizeof(row->state));
        }
        if (count < max && step != SQLITE_DONE) count = -1;
    }
    sqlite3_finalize(stmt);
    db_close_readonly_connection(db);
    return count;
}

static int retry_terminal_files(const char *root, int *budget,
                                observation_retention_result_t *result, asset_row_t *rows) {
    int want = *budget < OBS_PAGE ? *budget : OBS_PAGE;
    if (want <= 0) return 0;
    int n = query_rows(
        "SELECT uuid, path, COALESCE(bytes, 0), 0, state FROM observation_assets "
        "WHERE state IN ('expired', 'unavailable') AND path IS NOT NULL AND path <> '' "
        "AND ?1 > 0 ORDER BY uuid LIMIT ?2;", NULL, 1, want, rows);
    if (n < 0) return -1;
    for (int i = 0; i < n; ++i) {
        --*budget;
        if (cleanup_asset_file(root, &rows[i], &result->files_deleted) != 0) return -1;
    }
    if (n == want) result->more_work = true;
    return 0;
}

static int reclaim_unreferenced(const char *root, int *budget,
                                observation_retention_result_t *result, asset_row_t *rows) {
    if (*budget <= 0) return 0;
    int n = query_rows(
        "SELECT a.uuid, COALESCE(a.path, ''), COALESCE(a.bytes, 0), 0, a.state "
        "FROM observation_assets a WHERE a.state <> 'pending' AND a.protected = 0 "
        "AND ?1 > 0 AND NOT EXISTS "
        "(SELECT 1 FROM object_observations o WHERE o.asset_uuid = a.uuid) "
        "ORDER BY a.uuid LIMIT ?2;", NULL, 1, *budget, rows);
    if (n < 0) return -1;
    for (int i = 0; i < n; ++i) {
        --*budget;
        if (strcmp(rows[i].state, "ready") == 0) {
            if (expire_asset(root, &rows[i], result) < 0) return -1;
        } else {
            if (cleanup_asset_file(root, &rows[i], &result->files_deleted) != 0) return -1;
        }
        int deleted = delete_asset_row(rows[i].uuid);
        if (deleted < 0) return -1;
        result->unreferenced_deleted += deleted;
    }
    if (*budget <= 0) result->more_work = true;
    return 0;
}

/* An asset's age is its newest reference; captured/created time otherwise. */
#define EVICTION_CANDIDATES \
    "SELECT uuid, path, bytes, ref_ms, 'ready' FROM (" \
    "SELECT a.uuid AS uuid, COALESCE(a.path, '') AS path, COALESCE(a.bytes, 0) AS bytes, " \
    "COALESCE((SELECT MAX(o.observed_at_ms) FROM object_observations o " \
    "WHERE o.asset_uuid = a.uuid), a.captured_at_ms, a.created_at_ms) AS ref_ms " \
    "FROM observation_assets a WHERE a.camera_uuid = ?1 AND a.state = 'ready' " \
    "AND a.protected = 0) WHERE ref_ms < ?2 ORDER BY ref_ms, uuid LIMIT ?3;"

static int count_unprotected_ready(const char *camera, int64_t *count) {
    sqlite3 *db = NULL;
    if (db_open_readonly_connection(&db) != 0) return -1;
    sqlite3_stmt *stmt = NULL;
    int rc = -1;
    if (sqlite3_prepare_v2(db,
            "SELECT count(*) FROM observation_assets WHERE camera_uuid = ? AND "
            "state = 'ready' AND protected = 0;", -1, &stmt, NULL) == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, camera, -1, SQLITE_TRANSIENT);
        if (sqlite3_step(stmt) == SQLITE_ROW) {
            *count = sqlite3_column_int64(stmt, 0);
            rc = 0;
        }
    }
    sqlite3_finalize(stmt);
    db_close_readonly_connection(db);
    return rc;
}

static int enforce_camera(const char *root, int64_t now_ms,
                          observation_retention_camera_status_t *camera, int *budget,
                          observation_retention_result_t *result, asset_row_t *rows) {
    /* Age budget. */
    int64_t cutoff = now_ms - (int64_t)camera->max_age_days * DAY_MS;
    if (*budget > 0) {
        int want = *budget < OBS_PAGE ? *budget : OBS_PAGE;
        int n = query_rows(EVICTION_CANDIDATES, camera->camera_uuid, cutoff, want, rows);
        if (n < 0) return -1;
        for (int i = 0; i < n; ++i) {
            --*budget;
            int rc = expire_asset(root, &rows[i], result);
            if (rc < 0) return -1;
            if (rc > 0) camera->retained_bytes -= rows[i].bytes;
        }
        if (n == want) result->more_work = true;
    }

    /* Byte budget: oldest unprotected first. */
    while (camera->retained_bytes > camera->max_bytes && *budget > 0) {
        int want = *budget < OBS_PAGE ? *budget : OBS_PAGE;
        int n = query_rows(EVICTION_CANDIDATES, camera->camera_uuid, INT64_MAX, want, rows);
        if (n < 0) return -1;
        if (n == 0) break;
        for (int i = 0; i < n && camera->retained_bytes > camera->max_bytes; ++i) {
            --*budget;
            int rc = expire_asset(root, &rows[i], result);
            if (rc < 0) return -1;
            if (rc > 0) camera->retained_bytes -= rows[i].bytes;
        }
        if (n < want) break;
    }

    camera->budget_exhausted = false;
    if (camera->retained_bytes > camera->max_bytes) {
        int64_t evictable = 0;
        if (count_unprotected_ready(camera->camera_uuid, &evictable) != 0) return -1;
        if (evictable == 0) {
            camera->budget_exhausted = true;
            ++result->cameras_budget_exhausted;
        } else {
            result->more_work = true;
        }
    }
    return 0;
}

static int load_cameras(observation_retention_camera_status_t **out, int *count) {
    *out = NULL;
    *count = 0;
    sqlite3 *db = NULL;
    if (db_open_readonly_connection(&db) != 0) return -1;
    sqlite3_stmt *stmt = NULL;
    int rc = -1, capacity = 0;
    if (sqlite3_prepare_v2(db,
            "SELECT c.camera_uuid, s.observation_max_age_days, s.observation_max_bytes, "
            "(SELECT COALESCE(SUM(x.bytes), 0) FROM observation_assets x "
            " WHERE x.camera_uuid = c.camera_uuid AND x.state = 'ready') "
            "FROM (SELECT DISTINCT camera_uuid FROM observation_assets WHERE state = 'ready') c "
            "LEFT JOIN streams s ON s.camera_uuid = c.camera_uuid ORDER BY c.camera_uuid;",
            -1, &stmt, NULL) == SQLITE_OK) {
        rc = 0;
        int step;
        while ((step = sqlite3_step(stmt)) == SQLITE_ROW) {
            if (*count == capacity) {
                int next = capacity ? capacity * 2 : 16;
                void *grown = realloc(*out, (size_t)next * sizeof(**out));
                if (!grown) { rc = -1; break; }
                *out = grown;
                capacity = next;
            }
            observation_retention_camera_status_t *camera = &(*out)[(*count)++];
            memset(camera, 0, sizeof(*camera));
            copy_text(stmt, 0, camera->camera_uuid, sizeof(camera->camera_uuid));
            camera->max_age_days = sqlite3_column_int(stmt, 1);
            camera->max_bytes = sqlite3_column_int64(stmt, 2);
            camera->retained_bytes = sqlite3_column_int64(stmt, 3);
            if (camera->max_age_days <= 0)
                camera->max_age_days = OBS_RETENTION_DEFAULT_MAX_AGE_DAYS;
            if (camera->max_bytes <= 0) camera->max_bytes = OBS_RETENTION_DEFAULT_MAX_BYTES;
        }
        if (rc == 0 && step != SQLITE_DONE) rc = -1;
    }
    sqlite3_finalize(stmt);
    db_close_readonly_connection(db);
    if (rc != 0) {
        free(*out);
        *out = NULL;
        *count = 0;
    }
    return rc;
}

/* L3 admission flag (include/core/observation_evidence.h). Weak so this
 * module links without the evidence worker (tools, tests); once L3's object
 * is linked the flags are pushed to it on every pass. */
extern void observation_evidence_set_budget_exhausted(const char *camera_uuid, bool exhausted)
    __attribute__((weak));

static const observation_retention_camera_status_t *find_status(
        const observation_retention_camera_status_t *cameras, int count, const char *uuid) {
    for (int i = 0; i < count; ++i)
        if (strcmp(cameras[i].camera_uuid, uuid) == 0) return &cameras[i];
    return NULL;
}

static void publish_status(observation_retention_camera_status_t *cameras, int count) {
    int exhausted = 0;
    for (int i = 0; i < count; ++i)
        if (cameras[i].budget_exhausted) ++exhausted;
    pthread_mutex_lock(&g_status_mutex);
    observation_retention_camera_status_t *old = g_status;
    int old_count = g_status_count;
    g_status = cameras;
    g_status_count = count;
    atomic_store(&g_exhausted_count, exhausted);
    pthread_mutex_unlock(&g_status_mutex);

    /* Only this thread replaces g_status, so cameras stays valid here. */
    if (observation_evidence_set_budget_exhausted) {
        for (int i = 0; i < count; ++i) {
            const observation_retention_camera_status_t *before =
                find_status(old, old_count, cameras[i].camera_uuid);
            if (cameras[i].budget_exhausted || (before && before->budget_exhausted))
                observation_evidence_set_budget_exhausted(cameras[i].camera_uuid,
                                                          cameras[i].budget_exhausted);
        }
        for (int i = 0; i < old_count; ++i)
            if (old[i].budget_exhausted && !find_status(cameras, count, old[i].camera_uuid))
                observation_evidence_set_budget_exhausted(old[i].camera_uuid, false);
    }
    free(old);
}

/* Caller holds g_run_mutex, including admission and scheduling state for ticks. */
static int retention_run_locked(const char *storage_path, int64_t now_ms,
                                observation_retention_result_t *result) {
    observation_retention_result_t local;
    if (!result) result = &local;
    memset(result, 0, sizeof(*result));
    char root[OBS_PATH_SIZE];
    if (root_dir(storage_path, root, sizeof(root)) != 0) return -1;
    if (!atomic_load(&g_startup_done)) return 1;

    asset_row_t *rows = calloc(OBS_PAGE, sizeof(*rows));
    if (!rows) return -1;
    int budget = OBS_RETENTION_MAX_EVICTIONS;
    int rc = prune_history(now_ms, result);
    if (rc == 0) rc = retry_terminal_files(root, &budget, result, rows);
    if (rc == 0) rc = reclaim_unreferenced(root, &budget, result, rows);

    observation_retention_camera_status_t *cameras = NULL;
    int camera_count = 0;
    if (rc == 0) rc = load_cameras(&cameras, &camera_count);
    for (int i = 0; rc == 0 && i < camera_count; ++i)
        rc = enforce_camera(root, now_ms, &cameras[i], &budget, result, rows);
    if (rc == 0) {
        publish_status(cameras, camera_count);
        cameras = NULL;
    }
    free(cameras);
    free(rows);

    observation_meta_t meta;
    if (db_observation_meta_get(&meta) == DB_OBS_OK) result->oldest_seq = meta.oldest_seq;
    if (result->assets_expired || result->unreferenced_deleted || result->changes_pruned)
        log_info("Observation retention: %d media expired, %d files deleted, %d unreferenced "
                 "assets reclaimed, %d changes and %d observations pruned (oldest seq %lld)",
                 result->assets_expired, result->files_deleted, result->unreferenced_deleted,
                 result->changes_pruned, result->observations_pruned,
                 (long long)result->oldest_seq);
    if (result->cameras_budget_exhausted)
        log_warn("Observation evidence budget exhausted on %d camera(s): only protected media "
                 "remains", result->cameras_budget_exhausted);
    if (rc != 0) log_error("Observation retention pass failed");
    return rc == 0 ? 0 : -1;
}

int observation_retention_run(const char *storage_path, int64_t now_ms,
                              observation_retention_result_t *result) {
    pthread_mutex_lock(&g_run_mutex);
    int rc = retention_run_locked(storage_path, now_ms, result);
    pthread_mutex_unlock(&g_run_mutex);
    return rc;
}

void observation_retention_maintenance_tick(void) {
    if (!atomic_load(&g_startup_done)) return;
    pthread_mutex_lock(&g_run_mutex);
    time_t now = time(NULL);
    if (!g_more_work && g_last_tick != 0 && now - g_last_tick < OBS_RETENTION_INTERVAL_SEC) {
        pthread_mutex_unlock(&g_run_mutex);
        return;
    }
    g_last_tick = now;

    observation_retention_result_t result;
    int rc = retention_run_locked(g_config.storage_path, wall_ms(), &result);

    g_more_work = rc == 0 && result.more_work;
    pthread_mutex_unlock(&g_run_mutex);
}

/* ---- budget flag -------------------------------------------------------- */

bool observation_retention_budget_exhausted(const char *camera_uuid) {
    if (!camera_uuid || atomic_load(&g_exhausted_count) == 0) return false;
    if (pthread_mutex_trylock(&g_status_mutex) != 0) return false;
    bool exhausted = false;
    for (int i = 0; i < g_status_count; ++i) {
        if (strcmp(g_status[i].camera_uuid, camera_uuid) == 0) {
            exhausted = g_status[i].budget_exhausted;
            break;
        }
    }
    pthread_mutex_unlock(&g_status_mutex);
    return exhausted;
}

int observation_retention_budget_exhausted_count(void) {
    return atomic_load(&g_exhausted_count);
}

int observation_retention_camera_status(observation_retention_camera_status_t *out, int max) {
    if (!out || max <= 0) return 0;
    pthread_mutex_lock(&g_status_mutex);
    int n = g_status_count < max ? g_status_count : max;
    if (n > 0) memcpy(out, g_status, (size_t)n * sizeof(*out));
    pthread_mutex_unlock(&g_status_mutex);
    return n;
}

void observation_retention_reset_for_testing(void) {
    pthread_mutex_lock(&g_run_mutex);
    atomic_store(&g_startup_done, 0);
    publish_status(NULL, 0);
    g_last_tick = 0;
    g_more_work = false;
    pthread_mutex_unlock(&g_run_mutex);
}
