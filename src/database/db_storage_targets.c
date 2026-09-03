/*
 * _XOPEN_SOURCE 700 implies _POSIX_C_SOURCE 200809L and additionally exposes
 * realpath(), which POSIX alone does not declare on either glibc or musl.
 */
#define _XOPEN_SOURCE 700

#include "database/db_storage_targets.h"

#include <ctype.h>
#include <errno.h>
#include <fcntl.h>
#include <limits.h>
#include <pthread.h>
#include <sqlite3.h>
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/statvfs.h>
#include <time.h>
#include <unistd.h>

#include "database/db_core.h"
#include "core/logger.h"
#include "utils/strings.h"

#define STORAGE_TARGET_SELECT_FIELDS \
    "t.uuid,t.name,t.target_type,t.root_path,t.enabled,t.is_default," \
    "t.storage_class,t.reserve_bytes,t.high_watermark_pct," \
    "t.low_watermark_pct,t.health_status,t.capacity_bytes," \
    "t.available_bytes,t.filesystem_device,t.last_probe_at," \
    "t.last_success_at,t.last_error,t.revision,t.created_at,t.updated_at," \
    "t.recording_count,t.recording_bytes,t.mount_required,t.mount_guard_path," \
    "t.migration_bandwidth_bps,t.archival_window_start_minute," \
    "t.archival_window_end_minute,t.replica_count,t.replica_bytes"

static void set_error(char *error, size_t error_size,
                      const char *format, ...) {
    if (!error || error_size == 0 || error[0] != '\0') return;
    va_list arguments;
    va_start(arguments, format);
    vsnprintf(error, error_size, format, arguments);
    va_end(arguments);
}

static void copy_column(char *destination, size_t destination_size,
                        sqlite3_stmt *statement, int column) {
    const char *value = (const char *)sqlite3_column_text(statement, column);
    safe_strcpy(destination, value ? value : "", destination_size, 0);
}

static void populate(sqlite3_stmt *statement, storage_target_t *target) {
    memset(target, 0, sizeof(*target));
    copy_column(target->uuid, sizeof(target->uuid), statement, 0);
    copy_column(target->name, sizeof(target->name), statement, 1);
    copy_column(target->target_type, sizeof(target->target_type), statement, 2);
    copy_column(target->root_path, sizeof(target->root_path), statement, 3);
    target->enabled = sqlite3_column_int(statement, 4) != 0;
    target->is_default = sqlite3_column_int(statement, 5) != 0;
    copy_column(target->storage_class, sizeof(target->storage_class),
                statement, 6);
    target->reserve_bytes = (uint64_t)sqlite3_column_int64(statement, 7);
    target->high_watermark_pct = sqlite3_column_double(statement, 8);
    target->low_watermark_pct = sqlite3_column_double(statement, 9);
    copy_column(target->health_status, sizeof(target->health_status),
                statement, 10);
    target->capacity_bytes = (uint64_t)sqlite3_column_int64(statement, 11);
    target->available_bytes = (uint64_t)sqlite3_column_int64(statement, 12);
    target->filesystem_device =
        (uint64_t)sqlite3_column_int64(statement, 13);
    target->last_probe_at = sqlite3_column_type(statement, 14) == SQLITE_NULL
        ? 0 : sqlite3_column_int64(statement, 14);
    target->last_success_at = sqlite3_column_type(statement, 15) == SQLITE_NULL
        ? 0 : sqlite3_column_int64(statement, 15);
    copy_column(target->last_error, sizeof(target->last_error), statement, 16);
    target->revision = sqlite3_column_int64(statement, 17);
    target->created_at = sqlite3_column_int64(statement, 18);
    target->updated_at = sqlite3_column_int64(statement, 19);
    target->recording_count = (uint64_t)sqlite3_column_int64(statement, 20);
    target->recording_bytes = (uint64_t)sqlite3_column_int64(statement, 21);
    target->mount_required = sqlite3_column_int(statement, 22) != 0;
    copy_column(target->mount_guard_path,
                sizeof(target->mount_guard_path), statement, 23);
    target->migration_bandwidth_bps =
        (uint64_t)sqlite3_column_int64(statement, 24);
    target->archival_window_start_minute = sqlite3_column_int(statement, 25);
    target->archival_window_end_minute = sqlite3_column_int(statement, 26);
    target->replica_count = (uint64_t)sqlite3_column_int64(statement, 27);
    target->replica_bytes = (uint64_t)sqlite3_column_int64(statement, 28);
}

static bool valid_text(const char *value, size_t maximum, bool required) {
    if (!value) return false;
    size_t length = strnlen(value, maximum);
    if (length == maximum || (required && length == 0)) return false;
    for (const unsigned char *cursor = (const unsigned char *)value;
         *cursor; cursor++) {
        if (iscntrl(*cursor)) return false;
    }
    return true;
}

static bool path_has_unsafe_segment(const char *path) {
    if (!path) return true;
    const char *segment = path;
    while (*segment) {
        while (*segment == '/') segment++;
        const char *end = segment;
        while (*end && *end != '/') end++;
        size_t length = (size_t)(end - segment);
        if ((length == 1 && segment[0] == '.') ||
            (length == 2 && segment[0] == '.' && segment[1] == '.')) {
            return true;
        }
        segment = end;
    }
    return false;
}

static bool normalize_root_path(char path[MAX_PATH_LENGTH]) {
    if (!valid_text(path, MAX_PATH_LENGTH, true) || path[0] != '/' ||
        strcmp(path, "/") == 0 || strstr(path, "//") ||
        path_has_unsafe_segment(path)) {
        return false;
    }
    size_t length = strlen(path);
    while (length > 1 && path[length - 1] == '/') path[--length] = '\0';
    if (length <= 1) return false;

    /*
     * Collapse symlinks and bind mounts so two targets cannot alias the same
     * directory through different spellings, which would otherwise defeat both
     * the unique index on root_path and the longest-prefix match in
     * db_storage_target_classify_path(). A root that does not resolve yet --
     * a volume that has not been mounted, say -- keeps its literal form and is
     * reported by the health probe instead of being rejected here.
     */
    char resolved[PATH_MAX];
    if (realpath(path, resolved) != NULL && resolved[0] == '/' &&
        strcmp(resolved, "/") != 0 && strlen(resolved) < MAX_PATH_LENGTH &&
        !path_has_unsafe_segment(resolved)) {
        safe_strcpy(path, resolved, MAX_PATH_LENGTH, 0);
    }
    return true;
}

static bool valid_object_key(const char *key) {
    if (!valid_text(key, STORAGE_TARGET_OBJECT_KEY_MAX, true) ||
        key[0] == '/' || strchr(key, '\\') || path_has_unsafe_segment(key)) {
        return false;
    }
    size_t length = strlen(key);
    return key[length - 1] != '/';
}

static bool path_is_within(const char *path, const char *root) {
    if (!path || !root || path[0] != '/' || root[0] != '/') return false;
    size_t root_length = strlen(root);
    if (root_length == 1 && root[0] == '/') return true;
    return strncmp(path, root, root_length) == 0 &&
        (path[root_length] == '\0' || path[root_length] == '/');
}

static bool decode_mount_path(const char *encoded, char *decoded,
                              size_t decoded_size) {
    if (!encoded || !decoded || decoded_size == 0) return false;
    size_t encoded_length = strlen(encoded);
    size_t output = 0;
    for (size_t input = 0; input < encoded_length; input++) {
        unsigned char value = (unsigned char)encoded[input];
        if (value == '\\' && input + 3 < encoded_length &&
            encoded[input + 1] >= '0' &&
            encoded[input + 1] <= '7' && encoded[input + 2] >= '0' &&
            encoded[input + 2] <= '7' && encoded[input + 3] >= '0' &&
            encoded[input + 3] <= '7') {
            value = (unsigned char)((encoded[input + 1] - '0') * 64 +
                                    (encoded[input + 2] - '0') * 8 +
                                    (encoded[input + 3] - '0'));
            input += 3;
        }
        if (value == '\0' || output + 1 >= decoded_size) return false;
        decoded[output++] = (char)value;
    }
    decoded[output] = '\0';
    return true;
}

/*
 * Largest mount-point field find_mount() can store, as a scanf width literal.
 * Kept as a literal because the preprocessor cannot stringify arithmetic; the
 * assertion below fails the build if MAX_PATH_LENGTH ever moves.
 */
#define MOUNTINFO_FIELD_WIDTH "511"
_Static_assert(MAX_PATH_LENGTH == 512,
               "MOUNTINFO_FIELD_WIDTH must stay at MAX_PATH_LENGTH - 1");

static int find_mount(const char *root_path, const char *mountinfo_path,
                      const char *exact_path,
                      char mount_path[MAX_PATH_LENGTH]) {
    if (!root_path || root_path[0] != '/' || !mountinfo_path ||
        !mount_path) return -1;
    mount_path[0] = '\0';
    FILE *mountinfo = fopen(mountinfo_path, "r");
    if (!mountinfo) return -1;
    char *line = NULL;
    size_t line_capacity = 0;
    ssize_t line_length;
    size_t best_length = 0;
    while ((line_length = getline(&line, &line_capacity, mountinfo)) >= 0) {
        (void)line_length;
        char encoded[MAX_PATH_LENGTH];
        /*
         * Field 5 of a mountinfo line is the mount point. The scanf width must
         * track the buffer exactly -- a wider one overflows the stack on any
         * host with a long mount path, which is attacker-influenced inside a
         * container. Mount points longer than the buffer are truncated here and
         * simply fail to match a target root, which is the safe outcome.
         */
        if (sscanf(line, "%*s %*s %*s %*s %" MOUNTINFO_FIELD_WIDTH "s",
                   encoded) != 1) {
            continue;
        }
        char decoded[MAX_PATH_LENGTH];
        if (!decode_mount_path(encoded, decoded, sizeof(decoded))) continue;
        if (exact_path) {
            if (strcmp(decoded, exact_path) == 0) {
                safe_strcpy(mount_path, decoded, MAX_PATH_LENGTH, 0);
                best_length = strlen(decoded);
                break;
            }
            continue;
        }
        size_t length = strlen(decoded);
        if (strcmp(decoded, "/") == 0 || length <= best_length ||
            !path_is_within(root_path, decoded)) continue;
        safe_strcpy(mount_path, decoded, MAX_PATH_LENGTH, 0);
        best_length = length;
    }
    free(line);
    fclose(mountinfo);
    return best_length > 0 ? 0 : -1;
}

int db_storage_target_detect_mount(
    const char *root_path, const char *mountinfo_path,
    char mount_path[MAX_PATH_LENGTH]) {
    return find_mount(root_path, mountinfo_path, NULL, mount_path);
}

bool db_storage_target_mount_guard_active(const storage_target_t *target) {
    if (!target) return false;
    if (!target->mount_required) return true;
    if (target->mount_guard_path[0] == '\0') return false;
    char active_mount[MAX_PATH_LENGTH];
    return find_mount(target->root_path, "/proc/self/mountinfo",
                      target->mount_guard_path, active_mount) == 0;
}

db_storage_target_result_t db_storage_target_validate(
    storage_target_t *target, char *error, size_t error_size) {
    if (error && error_size > 0) error[0] = '\0';
    if (!target) {
        set_error(error, error_size, "storage target is required");
        return DB_STORAGE_TARGET_INVALID;
    }
    char normalized_name[STORAGE_TARGET_NAME_MAX];
    if (!valid_text(target->name, sizeof(target->name), true) ||
        copy_trimmed_value(normalized_name, sizeof(normalized_name),
                           target->name, 0) == 0) {
        set_error(error, error_size, "target name cannot be blank");
        return DB_STORAGE_TARGET_INVALID;
    }
    safe_strcpy(target->name, normalized_name, sizeof(target->name), 0);
    if (!normalize_root_path(target->root_path)) {
        set_error(error, error_size,
                  "root_path must be an absolute non-root path without traversal segments");
        return DB_STORAGE_TARGET_INVALID;
    }
    if (strcmp(target->target_type, "filesystem") != 0) {
        set_error(error, error_size,
                  "target_type must be filesystem in this release");
        return DB_STORAGE_TARGET_INVALID;
    }
    if (strcmp(target->storage_class, "hot") != 0 &&
        strcmp(target->storage_class, "warm") != 0 &&
        strcmp(target->storage_class, "cold") != 0) {
        set_error(error, error_size,
                  "storage_class must be hot, warm, or cold");
        return DB_STORAGE_TARGET_INVALID;
    }
    if (target->high_watermark_pct <= 0.0 ||
        target->high_watermark_pct >= 100.0 ||
        target->low_watermark_pct < 0.0 ||
        target->low_watermark_pct >= target->high_watermark_pct) {
        set_error(error, error_size,
                  "watermarks must satisfy 0 <= low < high < 100");
        return DB_STORAGE_TARGET_INVALID;
    }
    if (target->reserve_bytes > (uint64_t)INT64_MAX) {
        set_error(error, error_size,
                  "reserve_bytes exceeds the supported SQLite integer range");
        return DB_STORAGE_TARGET_INVALID;
    }
    if (target->migration_bandwidth_bps > (uint64_t)INT64_MAX ||
        (target->migration_bandwidth_bps > 0 &&
         target->migration_bandwidth_bps < 65536)) {
        set_error(error, error_size,
                  "migration bandwidth must be zero or at least 65536 bytes/second");
        return DB_STORAGE_TARGET_INVALID;
    }
    if (target->archival_window_start_minute < 0 ||
        target->archival_window_start_minute > 1439 ||
        target->archival_window_end_minute < 0 ||
        target->archival_window_end_minute > 1439) {
        set_error(error, error_size,
                  "archival window minutes must be between 0 and 1439");
        return DB_STORAGE_TARGET_INVALID;
    }
    if (target->mount_guard_path[0] != '\0' &&
        (target->mount_guard_path[0] != '/' ||
         strcmp(target->mount_guard_path, "/") == 0 ||
         !path_is_within(target->root_path, target->mount_guard_path))) {
        set_error(error, error_size,
                  "mount guard must be a non-root mount containing root_path");
        return DB_STORAGE_TARGET_INVALID;
    }
    if (!target->mount_required) target->mount_guard_path[0] = '\0';
    return DB_STORAGE_TARGET_OK;
}

static db_storage_target_result_t get_locked(
    sqlite3 *db, const char *uuid, storage_target_t *target) {
    const char *sql = "SELECT " STORAGE_TARGET_SELECT_FIELDS
        " FROM storage_targets t WHERE t.uuid=? LIMIT 1;";
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, uuid, -1, SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    db_storage_target_result_t outcome = DB_STORAGE_TARGET_NOT_FOUND;
    if (result == SQLITE_ROW) {
        populate(statement, target);
        outcome = DB_STORAGE_TARGET_OK;
    } else if (result != SQLITE_DONE) {
        outcome = DB_STORAGE_TARGET_ERROR;
    }
    if (statement) sqlite3_finalize(statement);
    return outcome;
}

int db_storage_target_count(void) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int count = -1;
    if (sqlite3_prepare_v2(db, "SELECT count(*) FROM storage_targets;", -1,
                           &statement, NULL) == SQLITE_OK &&
        sqlite3_step(statement) == SQLITE_ROW) {
        count = sqlite3_column_int(statement, 0);
    }
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    return count;
}

int db_storage_target_list(storage_target_t *targets, int max_count) {
    if (!targets || max_count <= 0) return -1;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;
    const char *sql = "SELECT " STORAGE_TARGET_SELECT_FIELDS
        " FROM storage_targets t"
        " ORDER BY t.is_default DESC,t.name COLLATE NOCASE,t.uuid;";
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    int count = 0;
    if (result == SQLITE_OK) {
        while (count < max_count &&
               (result = sqlite3_step(statement)) == SQLITE_ROW) {
            populate(statement, &targets[count++]);
        }
    }
    if (result != SQLITE_DONE && result != SQLITE_ROW) count = -1;
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    return count;
}

db_storage_target_result_t db_storage_target_get(
    const char *uuid, storage_target_t *target) {
    if (!lightnvr_uuid_is_valid(uuid) || !target) {
        return DB_STORAGE_TARGET_INVALID;
    }
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return DB_STORAGE_TARGET_ERROR;
    pthread_mutex_lock(mutex);
    db_storage_target_result_t result = get_locked(db, uuid, target);
    pthread_mutex_unlock(mutex);
    return result;
}

db_storage_target_result_t db_storage_target_get_default(
    storage_target_t *target) {
    if (!target) return DB_STORAGE_TARGET_INVALID;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return DB_STORAGE_TARGET_ERROR;
    const char *sql = "SELECT " STORAGE_TARGET_SELECT_FIELDS
        " FROM storage_targets t WHERE t.is_default=1 LIMIT 1;";
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    db_storage_target_result_t outcome = DB_STORAGE_TARGET_NOT_FOUND;
    if (result == SQLITE_OK) result = sqlite3_step(statement);
    if (result == SQLITE_ROW) {
        populate(statement, target);
        outcome = DB_STORAGE_TARGET_OK;
    } else if (result != SQLITE_DONE) {
        outcome = DB_STORAGE_TARGET_ERROR;
    }
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    return outcome;
}

static db_storage_target_result_t probe_path(storage_target_t *target,
                                             bool write_test) {
    target->last_probe_at = (int64_t)time(NULL);
    target->capacity_bytes = 0;
    target->available_bytes = 0;
    target->filesystem_device = 0;
    target->last_error[0] = '\0';
    if (!target->enabled) {
        safe_strcpy(target->health_status, "disabled",
                    sizeof(target->health_status), 0);
        return DB_STORAGE_TARGET_OK;
    }

    if (target->mount_required) {
        char active_mount[MAX_PATH_LENGTH];
        int mount_result;
        if (target->mount_guard_path[0] == '\0') {
            mount_result = find_mount(target->root_path,
                                      "/proc/self/mountinfo", NULL,
                                      active_mount);
            if (mount_result == 0) {
                safe_strcpy(target->mount_guard_path, active_mount,
                            sizeof(target->mount_guard_path), 0);
            }
        } else {
            mount_result = find_mount(target->root_path,
                                      "/proc/self/mountinfo",
                                      target->mount_guard_path,
                                      active_mount);
        }
        if (mount_result != 0) {
            if (target->mount_guard_path[0]) {
                snprintf(target->last_error, sizeof(target->last_error),
                         "Required mount is absent: %.200s",
                         target->mount_guard_path);
            } else {
                safe_strcpy(
                    target->last_error,
                    "No distinct mounted filesystem contains this target",
                    sizeof(target->last_error), 0);
            }
            safe_strcpy(target->health_status, "unavailable",
                        sizeof(target->health_status), 0);
            return DB_STORAGE_TARGET_UNAVAILABLE;
        }
    }

    struct stat info;
    if (stat(target->root_path, &info) != 0 || !S_ISDIR(info.st_mode)) {
        snprintf(target->last_error, sizeof(target->last_error),
                 "Target directory is unavailable: %s", strerror(errno));
        safe_strcpy(target->health_status, "unavailable",
                    sizeof(target->health_status), 0);
        return DB_STORAGE_TARGET_UNAVAILABLE;
    }
    target->filesystem_device = (uint64_t)info.st_dev;

    struct statvfs filesystem;
    if (statvfs(target->root_path, &filesystem) != 0) {
        snprintf(target->last_error, sizeof(target->last_error),
                 "Capacity probe failed: %s", strerror(errno));
        safe_strcpy(target->health_status, "unavailable",
                    sizeof(target->health_status), 0);
        return DB_STORAGE_TARGET_UNAVAILABLE;
    }
    uint64_t fragment = (uint64_t)filesystem.f_frsize;
    target->capacity_bytes = (uint64_t)filesystem.f_blocks * fragment;
    target->available_bytes = (uint64_t)filesystem.f_bavail * fragment;

    if (access(target->root_path, W_OK | X_OK) != 0) {
        snprintf(target->last_error, sizeof(target->last_error),
                 "Target directory is not writable: %s", strerror(errno));
        safe_strcpy(target->health_status, "unavailable",
                    sizeof(target->health_status), 0);
        return DB_STORAGE_TARGET_UNAVAILABLE;
    }

    if (write_test) {
        char probe_path[MAX_PATH_LENGTH];
        int written = snprintf(probe_path, sizeof(probe_path),
                               "%s/.lightnvr-storage-probe-XXXXXX",
                               target->root_path);
        if (written < 0 || written >= (int)sizeof(probe_path)) {
            safe_strcpy(target->last_error, "Probe path is too long",
                        sizeof(target->last_error), 0);
            safe_strcpy(target->health_status, "unavailable",
                        sizeof(target->health_status), 0);
            return DB_STORAGE_TARGET_UNAVAILABLE;
        }
        int descriptor = mkstemp(probe_path);
        if (descriptor < 0 || write(descriptor, "1", 1) != 1 ||
            fsync(descriptor) != 0) {
            int saved_errno = errno;
            if (descriptor >= 0) close(descriptor);
            unlink(probe_path);
            snprintf(target->last_error, sizeof(target->last_error),
                     "Write probe failed: %s", strerror(saved_errno));
            safe_strcpy(target->health_status, "unavailable",
                        sizeof(target->health_status), 0);
            return DB_STORAGE_TARGET_UNAVAILABLE;
        }
        close(descriptor);
        if (unlink(probe_path) != 0) {
            snprintf(target->last_error, sizeof(target->last_error),
                     "Probe cleanup failed: %s", strerror(errno));
            safe_strcpy(target->health_status, "degraded",
                        sizeof(target->health_status), 0);
            return DB_STORAGE_TARGET_UNAVAILABLE;
        }
    }

    double used_pct = target->capacity_bytes > 0
        ? 100.0 * (double)(target->capacity_bytes - target->available_bytes) /
            (double)target->capacity_bytes
        : 100.0;
    const char *health = target->available_bytes <= target->reserve_bytes ||
        used_pct >= target->high_watermark_pct ? "degraded" : "healthy";
    safe_strcpy(target->health_status, health,
                sizeof(target->health_status), 0);
    target->last_success_at = target->last_probe_at;
    return DB_STORAGE_TARGET_OK;
}

static int persist_health(const storage_target_t *target) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;
    const char *sql =
        "UPDATE storage_targets SET health_status=?,capacity_bytes=?,"
        "available_bytes=?,filesystem_device=?,last_probe_at=?,"
        "last_success_at=CASE WHEN ?>0 THEN ? ELSE last_success_at END,"
        "last_error=?,mount_guard_path=? WHERE uuid=?;";
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, target->health_status, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_int64(statement, 2,
                           (sqlite3_int64)target->capacity_bytes);
        sqlite3_bind_int64(statement, 3,
                           (sqlite3_int64)target->available_bytes);
        sqlite3_bind_int64(statement, 4,
                           (sqlite3_int64)target->filesystem_device);
        sqlite3_bind_int64(statement, 5, target->last_probe_at);
        sqlite3_bind_int64(statement, 6, target->last_success_at);
        sqlite3_bind_int64(statement, 7, target->last_success_at);
        sqlite3_bind_text(statement, 8, target->last_error, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 9, target->mount_guard_path, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 10, target->uuid, -1, SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    return result == SQLITE_DONE ? 0 : -1;
}

static bool valid_normalized_probe_error(const char *value) {
    static const char *const allowed[] = {
        "none", "not_found", "permission", "read_only", "no_space",
        "io", "timed_out", "busy", "invalid", "other", "mount_absent"
    };
    if (!value) return false;
    for (size_t index = 0; index < sizeof(allowed) / sizeof(allowed[0]);
         ++index) {
        if (strcmp(value, allowed[index]) == 0) return true;
    }
    return false;
}

db_storage_target_result_t db_storage_target_record_health(
    const char *uuid, const storage_target_health_update_t *update,
    storage_target_t *target) {
    if (!lightnvr_uuid_is_valid(uuid) || !update || update->probed_at <= 0 ||
        update->capacity_bytes > (uint64_t)INT64_MAX ||
        update->available_bytes > update->capacity_bytes ||
        update->filesystem_device > (uint64_t)INT64_MAX ||
        (update->cleanup_failed &&
         (!update->write_checked || !update->writeable)) ||
        (!update->write_checked && update->writeable) ||
        (!update->available && update->writeable) ||
        !valid_normalized_probe_error(update->normalized_error) ||
        (update->available &&
         strcmp(update->normalized_error, "none") != 0 &&
         !(update->write_checked &&
           (!update->writeable || update->cleanup_failed))) ||
        (!update->available &&
         strcmp(update->normalized_error, "none") == 0)) {
        return DB_STORAGE_TARGET_INVALID;
    }
    storage_target_t snapshot;
    db_storage_target_result_t result = db_storage_target_get(uuid, &snapshot);
    if (result != DB_STORAGE_TARGET_OK) return result;
    snapshot.last_probe_at = update->probed_at;
    snapshot.capacity_bytes = update->capacity_bytes;
    snapshot.available_bytes = update->available_bytes;
    snapshot.filesystem_device = update->filesystem_device;
    safe_strcpy(snapshot.last_error,
                strcmp(update->normalized_error, "none") == 0
                    ? "" : update->normalized_error,
                sizeof(snapshot.last_error), 0);
    if (!snapshot.enabled) {
        safe_strcpy(snapshot.health_status, "disabled",
                    sizeof(snapshot.health_status), 0);
    } else if (!update->available ||
               (update->write_checked && !update->writeable)) {
        safe_strcpy(snapshot.health_status, "unavailable",
                    sizeof(snapshot.health_status), 0);
    } else {
        double used_pct = update->capacity_bytes > 0
            ? 100.0 * (double)(update->capacity_bytes -
                               update->available_bytes) /
                (double)update->capacity_bytes
            : 100.0;
        bool degraded = update->cleanup_failed ||
            update->available_bytes <= snapshot.reserve_bytes ||
            used_pct >= snapshot.high_watermark_pct;
        safe_strcpy(snapshot.health_status,
                    degraded ? "degraded" : "healthy",
                    sizeof(snapshot.health_status), 0);
        snapshot.last_success_at = update->probed_at;
    }
    if (persist_health(&snapshot) != 0) return DB_STORAGE_TARGET_ERROR;
    return db_storage_target_get(uuid, target ? target : &snapshot);
}

int db_storage_target_recording_growth_bps(const char *uuid,
                                           double *bytes_per_second) {
    if (!bytes_per_second || (uuid && !lightnvr_uuid_is_valid(uuid))) return -1;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;
    const char *sql =
        "SELECT COALESCE(SUM(CASE WHEN size_bytes>0 THEN size_bytes ELSE 0 END),"
        "0),MIN(start_time),COUNT(*),"
        "strftime('%s','now') FROM recordings "
        "WHERE start_time>=strftime('%s','now')-86400 "
        "AND (? IS NULL OR storage_target_uuid=?);";
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        if (uuid) {
            sqlite3_bind_text(statement, 1, uuid, -1, SQLITE_TRANSIENT);
            sqlite3_bind_text(statement, 2, uuid, -1, SQLITE_TRANSIENT);
        } else {
            sqlite3_bind_null(statement, 1);
            sqlite3_bind_null(statement, 2);
        }
        result = sqlite3_step(statement);
    }
    if (result == SQLITE_ROW) {
        uint64_t bytes = (uint64_t)sqlite3_column_int64(statement, 0);
        int64_t first = sqlite3_column_int64(statement, 1);
        int64_t count = sqlite3_column_int64(statement, 2);
        int64_t now = sqlite3_column_int64(statement, 3);
        int64_t span = count > 0 && now > first ? now - first : 0;
        *bytes_per_second = span >= 60
            ? (double)bytes / (double)span : 0.0;
        result = SQLITE_DONE;
    }
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    return result == SQLITE_DONE ? 0 : -1;
}

db_storage_target_result_t db_storage_target_probe(
    const char *uuid, bool write_test, storage_target_t *target) {
    storage_target_t snapshot;
    db_storage_target_result_t result = db_storage_target_get(uuid, &snapshot);
    if (result != DB_STORAGE_TARGET_OK) return result;
    db_storage_target_result_t probe_result = probe_path(&snapshot, write_test);
    if (persist_health(&snapshot) != 0) return DB_STORAGE_TARGET_ERROR;
    result = db_storage_target_get(uuid, target ? target : &snapshot);
    if (result != DB_STORAGE_TARGET_OK) return result;
    return probe_result;
}

int db_storage_target_refresh_health(void) {
    int total = db_storage_target_count();
    if (total < 0 || total > STORAGE_TARGET_MAX_COUNT) return -1;
    if (total == 0) return 0;
    /*
     * storage_target_t is ~1.6 KB, so the full-count array is ~200 KB. This
     * runs on the storage controller thread, whose stack is 128 KB under musl,
     * and the same worklist reasoning applies here as to the stream-name
     * buffers in storage_manager.c.
     */
    storage_target_t *targets = calloc((size_t)total, sizeof(*targets));
    if (!targets) return -1;
    int count = db_storage_target_list(targets, total);
    if (count < 0) {
        free(targets);
        return -1;
    }
    int failures = 0;
    for (int index = 0; index < count; index++) {
        db_storage_target_result_t result = db_storage_target_probe(
            targets[index].uuid, false, NULL);
        if (result != DB_STORAGE_TARGET_OK &&
            result != DB_STORAGE_TARGET_UNAVAILABLE) {
            failures++;
        }
    }
    free(targets);
    return failures == 0 ? count : -1;
}

db_storage_target_result_t db_storage_target_create(storage_target_t *target) {
    char validation_error[STORAGE_TARGET_ERROR_MAX] = {0};
    if (db_storage_target_validate(target, validation_error,
                                   sizeof(validation_error)) !=
        DB_STORAGE_TARGET_OK || target->is_default) {
        return DB_STORAGE_TARGET_INVALID;
    }
    if (target->enabled &&
        probe_path(target, true) != DB_STORAGE_TARGET_OK) {
        return DB_STORAGE_TARGET_UNAVAILABLE;
    }
    if (!target->enabled) {
        safe_strcpy(target->health_status, "disabled",
                    sizeof(target->health_status), 0);
    }
    if (lightnvr_uuid_generate_v4(target->uuid) != 0) {
        return DB_STORAGE_TARGET_ERROR;
    }
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return DB_STORAGE_TARGET_ERROR;
    const char *sql =
        "INSERT INTO storage_targets(uuid,name,target_type,root_path,enabled,"
        "is_default,storage_class,reserve_bytes,high_watermark_pct,"
        "low_watermark_pct,health_status,capacity_bytes,available_bytes,"
        "filesystem_device,last_probe_at,last_success_at,last_error,"
        "mount_required,mount_guard_path,migration_bandwidth_bps,"
        "archival_window_start_minute,archival_window_end_minute)"
        " VALUES(?,?,?,?,?,0,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?);";
    pthread_mutex_lock(mutex);
    int count = -1;
    sqlite3_stmt *count_statement = NULL;
    if (sqlite3_prepare_v2(db, "SELECT count(*) FROM storage_targets;", -1,
                           &count_statement, NULL) == SQLITE_OK &&
        sqlite3_step(count_statement) == SQLITE_ROW) {
        count = sqlite3_column_int(count_statement, 0);
    }
    if (count_statement) sqlite3_finalize(count_statement);
    if (count < 0 || count >= STORAGE_TARGET_MAX_COUNT) {
        pthread_mutex_unlock(mutex);
        return count < 0 ? DB_STORAGE_TARGET_ERROR : DB_STORAGE_TARGET_LIMIT;
    }
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, target->uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 2, target->name, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 3, target->target_type, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 4, target->root_path, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_int(statement, 5, target->enabled ? 1 : 0);
        sqlite3_bind_text(statement, 6, target->storage_class, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_int64(statement, 7,
                           (sqlite3_int64)target->reserve_bytes);
        sqlite3_bind_double(statement, 8, target->high_watermark_pct);
        sqlite3_bind_double(statement, 9, target->low_watermark_pct);
        sqlite3_bind_text(statement, 10, target->health_status, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_int64(statement, 11,
                           (sqlite3_int64)target->capacity_bytes);
        sqlite3_bind_int64(statement, 12,
                           (sqlite3_int64)target->available_bytes);
        sqlite3_bind_int64(statement, 13,
                           (sqlite3_int64)target->filesystem_device);
        if (target->last_probe_at > 0) sqlite3_bind_int64(statement, 14,
                                                          target->last_probe_at);
        else sqlite3_bind_null(statement, 14);
        if (target->last_success_at > 0) sqlite3_bind_int64(
            statement, 15, target->last_success_at);
        else sqlite3_bind_null(statement, 15);
        sqlite3_bind_text(statement, 16, target->last_error, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_int(statement, 17, target->mount_required ? 1 : 0);
        sqlite3_bind_text(statement, 18, target->mount_guard_path, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_int64(statement, 19,
                           (sqlite3_int64)target->migration_bandwidth_bps);
        sqlite3_bind_int(statement, 20,
                         target->archival_window_start_minute);
        sqlite3_bind_int(statement, 21,
                         target->archival_window_end_minute);
        result = sqlite3_step(statement);
    }
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    if (result == SQLITE_CONSTRAINT) return DB_STORAGE_TARGET_CONFLICT;
    if (result != SQLITE_DONE) return DB_STORAGE_TARGET_ERROR;
    return db_storage_target_get(target->uuid, target);
}

db_storage_target_result_t db_storage_target_update(
    storage_target_t *target, int64_t expected_revision) {
    if (!target || !lightnvr_uuid_is_valid(target->uuid) ||
        expected_revision < 1) return DB_STORAGE_TARGET_INVALID;
    storage_target_t existing;
    db_storage_target_result_t get_result =
        db_storage_target_get(target->uuid, &existing);
    if (get_result != DB_STORAGE_TARGET_OK) return get_result;
    if (existing.revision != expected_revision) return DB_STORAGE_TARGET_STALE;
    if (target->is_default != existing.is_default ||
        (existing.is_default && !target->enabled)) {
        return DB_STORAGE_TARGET_INVALID;
    }
    if (strcmp(existing.root_path, target->root_path) != 0) {
        target->mount_guard_path[0] = '\0';
    }
    char validation_error[STORAGE_TARGET_ERROR_MAX] = {0};
    if (db_storage_target_validate(target, validation_error,
                                   sizeof(validation_error)) !=
        DB_STORAGE_TARGET_OK) return DB_STORAGE_TARGET_INVALID;
    if (strcmp(existing.root_path, target->root_path) != 0 &&
        (existing.is_default || existing.recording_count > 0 ||
         existing.replica_count > 0)) {
        return DB_STORAGE_TARGET_IN_USE;
    }
    if (target->enabled && probe_path(target, true) != DB_STORAGE_TARGET_OK) {
        return DB_STORAGE_TARGET_UNAVAILABLE;
    }
    if (!target->enabled) {
        safe_strcpy(target->health_status, "disabled",
                    sizeof(target->health_status), 0);
        target->capacity_bytes = 0;
        target->available_bytes = 0;
        target->filesystem_device = 0;
        target->last_error[0] = '\0';
    }

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return DB_STORAGE_TARGET_ERROR;
    const char *sql =
        "UPDATE storage_targets SET name=?,target_type=?,root_path=?,"
        "enabled=?,storage_class=?,reserve_bytes=?,high_watermark_pct=?,"
        "low_watermark_pct=?,health_status=?,capacity_bytes=?,"
        "available_bytes=?,filesystem_device=?,last_probe_at=?,"
        "last_success_at=CASE WHEN ?>0 THEN ? ELSE last_success_at END,"
        "last_error=?,mount_required=?,mount_guard_path=?,"
        "migration_bandwidth_bps=?,archival_window_start_minute=?,"
        "archival_window_end_minute=?,"
        "revision=revision+1,updated_at=strftime('%s','now')"
        " WHERE uuid=? AND revision=?"
        " AND (root_path=? OR (is_default=0 AND recording_count=0 AND replica_count=0));";
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, target->name, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 2, target->target_type, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 3, target->root_path, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_int(statement, 4, target->enabled ? 1 : 0);
        sqlite3_bind_text(statement, 5, target->storage_class, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_int64(statement, 6,
                           (sqlite3_int64)target->reserve_bytes);
        sqlite3_bind_double(statement, 7, target->high_watermark_pct);
        sqlite3_bind_double(statement, 8, target->low_watermark_pct);
        sqlite3_bind_text(statement, 9, target->health_status, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_int64(statement, 10,
                           (sqlite3_int64)target->capacity_bytes);
        sqlite3_bind_int64(statement, 11,
                           (sqlite3_int64)target->available_bytes);
        sqlite3_bind_int64(statement, 12,
                           (sqlite3_int64)target->filesystem_device);
        if (target->last_probe_at > 0) sqlite3_bind_int64(statement, 13,
                                                          target->last_probe_at);
        else sqlite3_bind_null(statement, 13);
        sqlite3_bind_int64(statement, 14, target->last_success_at);
        sqlite3_bind_int64(statement, 15, target->last_success_at);
        sqlite3_bind_text(statement, 16, target->last_error, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_int(statement, 17, target->mount_required ? 1 : 0);
        sqlite3_bind_text(statement, 18, target->mount_guard_path, -1,
                          SQLITE_TRANSIENT);
        sqlite3_bind_int64(statement, 19,
                           (sqlite3_int64)target->migration_bandwidth_bps);
        sqlite3_bind_int(statement, 20,
                         target->archival_window_start_minute);
        sqlite3_bind_int(statement, 21,
                         target->archival_window_end_minute);
        sqlite3_bind_text(statement, 22, target->uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int64(statement, 23, expected_revision);
        sqlite3_bind_text(statement, 24, target->root_path, -1,
                          SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    int changed = sqlite3_changes(db);
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    if (result == SQLITE_CONSTRAINT) return DB_STORAGE_TARGET_CONFLICT;
    if (result != SQLITE_DONE) return DB_STORAGE_TARGET_ERROR;
    if (changed == 0) {
        storage_target_t current;
        db_storage_target_result_t current_result =
            db_storage_target_get(target->uuid, &current);
        if (current_result != DB_STORAGE_TARGET_OK) return current_result;
        if (current.revision != expected_revision) {
            return DB_STORAGE_TARGET_STALE;
        }
        if (strcmp(current.root_path, target->root_path) != 0 &&
            (current.is_default || current.recording_count > 0 ||
             current.replica_count > 0)) {
            return DB_STORAGE_TARGET_IN_USE;
        }
        return DB_STORAGE_TARGET_ERROR;
    }
    return db_storage_target_get(target->uuid, target);
}

db_storage_target_result_t db_storage_target_delete(
    const char *uuid, int64_t expected_revision) {
    if (!lightnvr_uuid_is_valid(uuid) || expected_revision < 1) {
        return DB_STORAGE_TARGET_INVALID;
    }
    storage_target_t existing;
    db_storage_target_result_t result = db_storage_target_get(uuid, &existing);
    if (result != DB_STORAGE_TARGET_OK) return result;
    if (existing.revision != expected_revision) return DB_STORAGE_TARGET_STALE;
    if (existing.is_default || existing.recording_count > 0 ||
        existing.replica_count > 0) {
        return DB_STORAGE_TARGET_IN_USE;
    }
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return DB_STORAGE_TARGET_ERROR;
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int sqlite_result = sqlite3_prepare_v2(db,
        "SELECT EXISTS(SELECT 1 FROM storage_pool_members WHERE target_uuid=?) OR "
        "EXISTS(SELECT 1 FROM storage_policies WHERE primary_target_uuid=? OR "
        "fallback_target_uuid=? OR migration_target_uuid=?) OR "
        "EXISTS(SELECT 1 FROM storage_migration_jobs WHERE source_target_uuid=? OR "
        "destination_target_uuid=?);", -1, &statement, NULL);
    if (sqlite_result == SQLITE_OK) {
        for (int parameter = 1; parameter <= 6; parameter++) {
            sqlite3_bind_text(statement, parameter, uuid, -1,
                              SQLITE_TRANSIENT);
        }
        sqlite_result = sqlite3_step(statement);
    }
    bool referenced = sqlite_result == SQLITE_ROW &&
        sqlite3_column_int(statement, 0) != 0;
    if (statement) sqlite3_finalize(statement);
    if (referenced) {
        pthread_mutex_unlock(mutex);
        return DB_STORAGE_TARGET_IN_USE;
    }
    statement = NULL;
    sqlite_result = sqlite3_prepare_v2(
        db, "DELETE FROM storage_targets WHERE uuid=? AND revision=?"
            " AND is_default=0 AND recording_count=0 AND replica_count=0;", -1,
        &statement, NULL);
    if (sqlite_result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int64(statement, 2, expected_revision);
        sqlite_result = sqlite3_step(statement);
    }
    int changed = sqlite3_changes(db);
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    if (sqlite_result == SQLITE_CONSTRAINT) return DB_STORAGE_TARGET_IN_USE;
    if (sqlite_result != SQLITE_DONE) return DB_STORAGE_TARGET_ERROR;
    if (changed == 1) return DB_STORAGE_TARGET_OK;
    storage_target_t current;
    result = db_storage_target_get(uuid, &current);
    if (result != DB_STORAGE_TARGET_OK) return result;
    if (current.revision != expected_revision) return DB_STORAGE_TARGET_STALE;
    return current.is_default || current.recording_count > 0 ||
            current.replica_count > 0
        ? DB_STORAGE_TARGET_IN_USE : DB_STORAGE_TARGET_ERROR;
}

int db_storage_target_bootstrap_default(
    const char *legacy_root, char uuid[LIGHTNVR_UUID_STRING_SIZE]) {
    if (!legacy_root || !uuid) return -1;
    storage_target_t candidate;
    memset(&candidate, 0, sizeof(candidate));
    safe_strcpy(candidate.name, "Default storage", sizeof(candidate.name), 0);
    safe_strcpy(candidate.target_type, "filesystem",
                sizeof(candidate.target_type), 0);
    safe_strcpy(candidate.root_path, legacy_root,
                sizeof(candidate.root_path), 0);
    candidate.enabled = true;
    candidate.is_default = true;
    safe_strcpy(candidate.storage_class, "hot",
                sizeof(candidate.storage_class), 0);
    candidate.high_watermark_pct = 90.0;
    candidate.low_watermark_pct = 80.0;
    char error[STORAGE_TARGET_ERROR_MAX] = {0};
    if (db_storage_target_validate(&candidate, error, sizeof(error)) !=
        DB_STORAGE_TARGET_OK) {
        log_error("Cannot bootstrap default storage target: %s", error);
        return -1;
    }

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(
        db, "SELECT uuid,root_path FROM storage_targets"
            " WHERE is_default=1 LIMIT 1;", -1, &statement, NULL);
    bool exists = false;
    char selected_root[MAX_PATH_LENGTH] = {0};
    if (result == SQLITE_OK && sqlite3_step(statement) == SQLITE_ROW) {
        const char *existing_uuid =
            (const char *)sqlite3_column_text(statement, 0);
        const char *existing_root =
            (const char *)sqlite3_column_text(statement, 1);
        safe_strcpy(uuid, existing_uuid ? existing_uuid : "",
                    LIGHTNVR_UUID_STRING_SIZE, 0);
        safe_strcpy(selected_root, existing_root ? existing_root : "",
                    sizeof(selected_root), 0);
        exists = true;
    }
    if (statement) sqlite3_finalize(statement);

    if (exists && strcmp(selected_root, candidate.root_path) != 0) {
        char previous_uuid[LIGHTNVR_UUID_STRING_SIZE];
        safe_strcpy(previous_uuid, uuid, sizeof(previous_uuid), 0);
        char next_uuid[LIGHTNVR_UUID_STRING_SIZE] = {0};

        statement = NULL;
        result = sqlite3_prepare_v2(
            db, "SELECT uuid FROM storage_targets WHERE root_path=? LIMIT 1;",
            -1, &statement, NULL);
        if (result == SQLITE_OK) {
            sqlite3_bind_text(statement, 1, candidate.root_path, -1,
                              SQLITE_TRANSIENT);
            if (sqlite3_step(statement) == SQLITE_ROW) {
                const char *matching_uuid =
                    (const char *)sqlite3_column_text(statement, 0);
                safe_strcpy(next_uuid, matching_uuid ? matching_uuid : "",
                            sizeof(next_uuid), 0);
            }
        }
        if (statement) sqlite3_finalize(statement);
        if (!next_uuid[0] && lightnvr_uuid_generate_v4(next_uuid) != 0) {
            pthread_mutex_unlock(mutex);
            return -1;
        }

        if (sqlite3_exec(db, "BEGIN IMMEDIATE;", NULL, NULL, NULL) !=
            SQLITE_OK) {
            pthread_mutex_unlock(mutex);
            return -1;
        }
        bool switched = true;
        statement = NULL;
        result = sqlite3_prepare_v2(
            db, "UPDATE storage_targets SET is_default=0,revision=revision+1,"
                "updated_at=strftime('%s','now') WHERE uuid=?;", -1,
            &statement, NULL);
        if (result == SQLITE_OK) {
            sqlite3_bind_text(statement, 1, previous_uuid, -1,
                              SQLITE_TRANSIENT);
            result = sqlite3_step(statement);
        }
        if (statement) sqlite3_finalize(statement);
        switched = result == SQLITE_DONE;

        if (switched && next_uuid[0]) {
            statement = NULL;
            result = sqlite3_prepare_v2(
                db, "SELECT count(*) FROM storage_targets WHERE uuid=?;", -1,
                &statement, NULL);
            bool target_exists = false;
            if (result == SQLITE_OK) {
                sqlite3_bind_text(statement, 1, next_uuid, -1,
                                  SQLITE_TRANSIENT);
                target_exists = sqlite3_step(statement) == SQLITE_ROW &&
                    sqlite3_column_int(statement, 0) == 1;
            }
            if (statement) sqlite3_finalize(statement);

            if (target_exists) {
                statement = NULL;
                result = sqlite3_prepare_v2(
                    db, "UPDATE storage_targets SET is_default=1,enabled=1,"
                        "revision=revision+1,updated_at=strftime('%s','now')"
                        " WHERE uuid=?;", -1, &statement, NULL);
                if (result == SQLITE_OK) {
                    sqlite3_bind_text(statement, 1, next_uuid, -1,
                                      SQLITE_TRANSIENT);
                    result = sqlite3_step(statement);
                }
                if (statement) sqlite3_finalize(statement);
                switched = result == SQLITE_DONE;
            } else {
                char generated_name[STORAGE_TARGET_NAME_MAX];
                snprintf(generated_name, sizeof(generated_name),
                         "Default storage %.8s", next_uuid);
                const char *insert_sql =
                    "INSERT INTO storage_targets(uuid,name,target_type,"
                    "root_path,enabled,is_default,storage_class,reserve_bytes,"
                    "high_watermark_pct,low_watermark_pct,health_status)"
                    " VALUES(?,?,'filesystem',?,1,1,'hot',0,90,80,'unknown');";
                statement = NULL;
                result = sqlite3_prepare_v2(db, insert_sql, -1, &statement,
                                            NULL);
                if (result == SQLITE_OK) {
                    sqlite3_bind_text(statement, 1, next_uuid, -1,
                                      SQLITE_TRANSIENT);
                    sqlite3_bind_text(statement, 2, generated_name, -1,
                                      SQLITE_TRANSIENT);
                    sqlite3_bind_text(statement, 3, candidate.root_path, -1,
                                      SQLITE_TRANSIENT);
                    result = sqlite3_step(statement);
                }
                if (statement) sqlite3_finalize(statement);
                switched = result == SQLITE_DONE;
            }
        }

        if (switched && sqlite3_exec(db, "COMMIT;", NULL, NULL, NULL) ==
                            SQLITE_OK) {
            safe_strcpy(uuid, next_uuid, LIGHTNVR_UUID_STRING_SIZE, 0);
            safe_strcpy(selected_root, candidate.root_path,
                        sizeof(selected_root), 0);
            log_info("Storage default changed from target %s to %s for root %s",
                     previous_uuid, uuid, selected_root);
        } else {
            sqlite3_exec(db, "ROLLBACK;", NULL, NULL, NULL);
            log_error("Failed to switch the default storage target: %s",
                      sqlite3_errmsg(db));
            pthread_mutex_unlock(mutex);
            return -1;
        }
    }

    if (!exists) {
        if (lightnvr_uuid_generate_v4(uuid) != 0) {
            pthread_mutex_unlock(mutex);
            return -1;
        }
        safe_strcpy(selected_root, candidate.root_path,
                    sizeof(selected_root), 0);
        const char *insert_sql =
            "INSERT INTO storage_targets(uuid,name,target_type,root_path,"
            "enabled,is_default,storage_class,reserve_bytes,"
            "high_watermark_pct,low_watermark_pct,health_status)"
            " VALUES(?,?, 'filesystem',?,1,1,'hot',0,90,80,'unknown');";
        statement = NULL;
        result = sqlite3_prepare_v2(db, insert_sql, -1, &statement, NULL);
        if (result == SQLITE_OK) {
            sqlite3_bind_text(statement, 1, uuid, -1, SQLITE_TRANSIENT);
            sqlite3_bind_text(statement, 2, candidate.name, -1,
                              SQLITE_TRANSIENT);
            sqlite3_bind_text(statement, 3, candidate.root_path, -1,
                              SQLITE_TRANSIENT);
            result = sqlite3_step(statement);
        }
        if (statement) sqlite3_finalize(statement);
        if (result != SQLITE_DONE) {
            log_error("Failed to create default storage target: %s",
                      sqlite3_errmsg(db));
            pthread_mutex_unlock(mutex);
            return -1;
        }
    }

    const char *backfill_sql =
        "UPDATE recordings SET storage_target_uuid=?,"
        "object_key=substr(file_path,length(?)+2),"
        "placement_reason='legacy-default'"
        " WHERE storage_target_uuid IS NULL"
        " AND substr(file_path,1,length(?)+1)=?||'/';";
    statement = NULL;
    result = sqlite3_prepare_v2(db, backfill_sql, -1, &statement, NULL);
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 2, selected_root, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 3, selected_root, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(statement, 4, selected_root, -1, SQLITE_TRANSIENT);
        result = sqlite3_step(statement);
    }
    int attached = result == SQLITE_DONE ? sqlite3_changes(db) : -1;
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    if (result != SQLITE_DONE) return -1;
    log_info("Storage target bootstrap: default=%s root=%s attached=%d",
             uuid, selected_root, attached);
    return 0;
}

int db_storage_target_classify_path(
    const char *absolute_path,
    char target_uuid[LIGHTNVR_UUID_STRING_SIZE],
    char object_key[STORAGE_TARGET_OBJECT_KEY_MAX]) {
    if (!absolute_path || absolute_path[0] != '/' || !target_uuid ||
        !object_key) return -1;
    target_uuid[0] = '\0';
    object_key[0] = '\0';
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;
    const char *sql = "SELECT uuid,root_path FROM storage_targets"
        " ORDER BY length(root_path) DESC,uuid;";
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(db, sql, -1, &statement, NULL);
    int outcome = -1;
    while (result == SQLITE_OK && sqlite3_step(statement) == SQLITE_ROW) {
        const char *uuid = (const char *)sqlite3_column_text(statement, 0);
        const char *root = (const char *)sqlite3_column_text(statement, 1);
        if (!uuid || !root) continue;
        size_t root_length = strlen(root);
        if (strncmp(absolute_path, root, root_length) != 0 ||
            absolute_path[root_length] != '/') continue;
        const char *relative = absolute_path + root_length + 1;
        if (!valid_object_key(relative)) continue;
        safe_strcpy(target_uuid, uuid, LIGHTNVR_UUID_STRING_SIZE, 0);
        safe_strcpy(object_key, relative, STORAGE_TARGET_OBJECT_KEY_MAX, 0);
        outcome = 0;
        break;
    }
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    return outcome;
}

int db_storage_target_resolve_path(
    const char *target_uuid, const char *object_key,
    char absolute_path[MAX_PATH_LENGTH]) {
    if (!lightnvr_uuid_is_valid(target_uuid) ||
        !valid_object_key(object_key) || !absolute_path) return -1;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;
    pthread_mutex_lock(mutex);
    sqlite3_stmt *statement = NULL;
    int result = sqlite3_prepare_v2(
        db, "SELECT root_path FROM storage_targets WHERE uuid=? LIMIT 1;", -1,
        &statement, NULL);
    int outcome = -1;
    if (result == SQLITE_OK) {
        sqlite3_bind_text(statement, 1, target_uuid, -1, SQLITE_TRANSIENT);
        if (sqlite3_step(statement) == SQLITE_ROW) {
            const char *root = (const char *)sqlite3_column_text(statement, 0);
            int written = root ? snprintf(absolute_path, MAX_PATH_LENGTH,
                                          "%s/%s", root, object_key) : -1;
            if (written > 0 && written < MAX_PATH_LENGTH) outcome = 0;
        }
    }
    if (statement) sqlite3_finalize(statement);
    pthread_mutex_unlock(mutex);
    return outcome;
}
