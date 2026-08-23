#define _POSIX_C_SOURCE 200809L
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <sqlite3.h>
#include <pthread.h>

#include "database/db_recording_tags.h"
#include "database/db_core.h"
#include "core/logger.h"
#include "utils/strings.h"

#define RECORDING_TAG_QUERY_BATCH_SIZE 500

/* ---- single-recording ops ---- */

int db_recording_tag_add(uint64_t recording_id, const char *tag) {
    if (!tag) return -1;
    char trimmed[MAX_TAG_LENGTH];
    if (copy_trimmed_value(trimmed, sizeof(trimmed), tag, 0) == 0) {
        return -1;
    }

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mtx = get_db_mutex();
    if (!db) { log_error("Database not initialized"); return -1; }

    pthread_mutex_lock(mtx);

    sqlite3_stmt *stmt = NULL;
    const char *sql = "INSERT OR IGNORE INTO recording_tags (recording_id, tag) VALUES (?, ?);";
    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare tag insert: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(mtx);
        return -1;
    }
    sqlite3_bind_int64(stmt, 1, (sqlite3_int64)recording_id);
    sqlite3_bind_text(stmt, 2, trimmed, -1, SQLITE_STATIC);
    rc = sqlite3_step(stmt);
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(mtx);

    return (rc == SQLITE_DONE) ? 0 : -1;
}

int db_recording_tag_remove(uint64_t recording_id, const char *tag) {
    if (!tag) return -1;
    char trimmed[MAX_TAG_LENGTH];
    if (copy_trimmed_value(trimmed, sizeof(trimmed), tag, 0) == 0) {
        return -1;
    }

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mtx = get_db_mutex();
    if (!db) { log_error("Database not initialized"); return -1; }

    pthread_mutex_lock(mtx);

    sqlite3_stmt *stmt = NULL;
    const char *sql = "DELETE FROM recording_tags WHERE recording_id = ? AND tag = ?;";
    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare tag delete: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(mtx);
        return -1;
    }
    sqlite3_bind_int64(stmt, 1, (sqlite3_int64)recording_id);
    sqlite3_bind_text(stmt, 2, trimmed, -1, SQLITE_STATIC);
    rc = sqlite3_step(stmt);
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(mtx);

    return (rc == SQLITE_DONE) ? 0 : -1;
}

int db_recording_tag_get(uint64_t recording_id, char tags[][MAX_TAG_LENGTH], int max_tags) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mtx = get_db_mutex();
    if (!db || !tags) return -1;

    pthread_mutex_lock(mtx);

    sqlite3_stmt *stmt = NULL;
    const char *sql = "SELECT tag FROM recording_tags WHERE recording_id = ? ORDER BY tag;";
    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare tag select: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(mtx);
        return -1;
    }
    sqlite3_bind_int64(stmt, 1, (sqlite3_int64)recording_id);

    int count = 0;
    while (sqlite3_step(stmt) == SQLITE_ROW && count < max_tags) {
        const char *t = (const char *)sqlite3_column_text(stmt, 0);
        if (t) {
            safe_strcpy(tags[count], t, MAX_TAG_LENGTH, 0);
            count++;
        }
    }
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(mtx);
    return count;
}

int db_recording_tag_get_batch(const uint64_t *recording_ids, int count,
                               recording_tag_list_t *tag_lists) {
    if (!recording_ids || count <= 0 || !tag_lists) return -1;

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mtx = get_db_mutex();
    if (!db) {
        log_error("Database not initialized");
        return -1;
    }

    memset(tag_lists, 0, (size_t)count * sizeof(*tag_lists));
    int total = 0;

    for (int batch_start = 0; batch_start < count;
         batch_start += RECORDING_TAG_QUERY_BATCH_SIZE) {
        int batch_count = count - batch_start;
        if (batch_count > RECORDING_TAG_QUERY_BATCH_SIZE) {
            batch_count = RECORDING_TAG_QUERY_BATCH_SIZE;
        }

        char sql[2048];
        safe_strcpy(sql,
                    "SELECT recording_id, tag FROM recording_tags "
                    "WHERE recording_id IN (",
                    sizeof(sql), 0);
        for (int i = 0; i < batch_count; i++) {
            if (i > 0) safe_strcat(sql, ",", sizeof(sql));
            safe_strcat(sql, "?", sizeof(sql));
        }
        safe_strcat(sql, ") ORDER BY recording_id, tag;", sizeof(sql));

        pthread_mutex_lock(mtx);
        sqlite3_stmt *stmt = NULL;
        int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
        if (rc != SQLITE_OK) {
            log_error("Failed to prepare batch tag select: %s", sqlite3_errmsg(db));
            pthread_mutex_unlock(mtx);
            return -1;
        }

        for (int i = 0; i < batch_count; i++) {
            sqlite3_bind_int64(stmt, i + 1,
                               (sqlite3_int64)recording_ids[batch_start + i]);
        }

        while ((rc = sqlite3_step(stmt)) == SQLITE_ROW) {
            uint64_t recording_id = (uint64_t)sqlite3_column_int64(stmt, 0);
            const char *tag = (const char *)sqlite3_column_text(stmt, 1);
            if (!tag) continue;

            for (int i = 0; i < batch_count; i++) {
                int output_index = batch_start + i;
                if (recording_ids[output_index] != recording_id) continue;

                recording_tag_list_t *list = &tag_lists[output_index];
                if (list->count < MAX_RECORDING_TAGS) {
                    safe_strcpy(list->tags[list->count], tag, MAX_TAG_LENGTH, 0);
                    list->count++;
                    total++;
                }
                break;
            }
        }

        if (rc != SQLITE_DONE) {
            log_error("Failed to fetch batch recording tags: %s", sqlite3_errmsg(db));
            sqlite3_finalize(stmt);
            pthread_mutex_unlock(mtx);
            return -1;
        }
        sqlite3_finalize(stmt);
        pthread_mutex_unlock(mtx);
    }

    return total;
}

int db_recording_tag_set(uint64_t recording_id, const char **tags, int tag_count) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mtx = get_db_mutex();
    if (!db) { log_error("Database not initialized"); return -1; }

    pthread_mutex_lock(mtx);

    /* Delete existing tags */
    sqlite3_stmt *del_stmt = NULL;
    int rc = sqlite3_prepare_v2(db,
        "DELETE FROM recording_tags WHERE recording_id = ?;", -1, &del_stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare tag delete-all: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(mtx);
        return -1;
    }
    sqlite3_bind_int64(del_stmt, 1, (sqlite3_int64)recording_id);
    sqlite3_step(del_stmt);
    sqlite3_finalize(del_stmt);

    /* Insert new tags */
    if (tag_count > 0 && tags) {
        sqlite3_stmt *ins_stmt = NULL;
        rc = sqlite3_prepare_v2(db,
            "INSERT OR IGNORE INTO recording_tags (recording_id, tag) VALUES (?, ?);",
            -1, &ins_stmt, NULL);
        if (rc != SQLITE_OK) {
            log_error("Failed to prepare tag insert: %s", sqlite3_errmsg(db));
            pthread_mutex_unlock(mtx);
            return -1;
        }
        for (int i = 0; i < tag_count; i++) {
            if (!tags[i]) continue;
            char trimmed[MAX_TAG_LENGTH];
            if (copy_trimmed_value(trimmed, sizeof(trimmed), tags[i], 0) == 0) {
                continue;
            }
            sqlite3_reset(ins_stmt);
            sqlite3_bind_int64(ins_stmt, 1, (sqlite3_int64)recording_id);
            sqlite3_bind_text(ins_stmt, 2, trimmed, -1, SQLITE_STATIC);
            sqlite3_step(ins_stmt);
        }
        sqlite3_finalize(ins_stmt);
    }

    pthread_mutex_unlock(mtx);
    return 0;
}

int db_recording_tag_get_all_unique(char tags[][MAX_TAG_LENGTH], int max_tags) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mtx = get_db_mutex();
    if (!db || !tags) return -1;

    pthread_mutex_lock(mtx);

    sqlite3_stmt *stmt = NULL;
    const char *sql = "SELECT DISTINCT tag FROM recording_tags ORDER BY tag;";
    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare unique tags select: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(mtx);
        return -1;
    }

    int count = 0;
    while (sqlite3_step(stmt) == SQLITE_ROW && count < max_tags) {
        const char *t = (const char *)sqlite3_column_text(stmt, 0);
        if (t) {
            safe_strcpy(tags[count], t, MAX_TAG_LENGTH, 0);
            count++;
        }
    }
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(mtx);
    return count;
}

int db_recording_tag_batch_add(const uint64_t *recording_ids, int count, const char *tag) {
    if (!recording_ids || !tag || count <= 0) return -1;
    char trimmed[MAX_TAG_LENGTH];
    if (copy_trimmed_value(trimmed, sizeof(trimmed), tag, 0) == 0) {
        return -1;
    }

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mtx = get_db_mutex();
    if (!db) { log_error("Database not initialized"); return -1; }

    pthread_mutex_lock(mtx);

    sqlite3_stmt *stmt = NULL;
    const char *sql = "INSERT OR IGNORE INTO recording_tags (recording_id, tag) VALUES (?, ?);";
    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare batch tag insert: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(mtx);
        return -1;
    }

    int success = 0;
    for (int i = 0; i < count; i++) {
        sqlite3_reset(stmt);
        sqlite3_bind_int64(stmt, 1, (sqlite3_int64)recording_ids[i]);
        sqlite3_bind_text(stmt, 2, trimmed, -1, SQLITE_STATIC);
        if (sqlite3_step(stmt) == SQLITE_DONE) success++;
    }
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(mtx);
    return success;
}

int db_recording_tag_batch_remove(const uint64_t *recording_ids, int count, const char *tag) {
    if (!recording_ids || !tag || count <= 0) return -1;
    char trimmed[MAX_TAG_LENGTH];
    if (copy_trimmed_value(trimmed, sizeof(trimmed), tag, 0) == 0) {
        return -1;
    }

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mtx = get_db_mutex();
    if (!db) { log_error("Database not initialized"); return -1; }

    pthread_mutex_lock(mtx);

    sqlite3_stmt *stmt = NULL;
    const char *sql = "DELETE FROM recording_tags WHERE recording_id = ? AND tag = ?;";
    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare batch tag delete: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(mtx);
        return -1;
    }

    int success = 0;
    for (int i = 0; i < count; i++) {
        sqlite3_reset(stmt);
        sqlite3_bind_int64(stmt, 1, (sqlite3_int64)recording_ids[i]);
        sqlite3_bind_text(stmt, 2, trimmed, -1, SQLITE_STATIC);
        if (sqlite3_step(stmt) == SQLITE_DONE) success++;
    }
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(mtx);
    return success;
}

int db_recording_tag_get_recordings_by_tag(const char *tag, uint64_t *recording_ids, int max_ids) {
    if (!tag || !recording_ids) return -1;
    char trimmed[MAX_TAG_LENGTH];
    if (copy_trimmed_value(trimmed, sizeof(trimmed), tag, 0) == 0) {
        return -1;
    }

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mtx = get_db_mutex();
    if (!db) return -1;

    pthread_mutex_lock(mtx);

    sqlite3_stmt *stmt = NULL;
    const char *sql = "SELECT recording_id FROM recording_tags WHERE tag = ? ORDER BY recording_id DESC;";
    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc != SQLITE_OK) {
        log_error("Failed to prepare recordings-by-tag select: %s", sqlite3_errmsg(db));
        pthread_mutex_unlock(mtx);
        return -1;
    }
    sqlite3_bind_text(stmt, 1, trimmed, -1, SQLITE_STATIC);

    int count = 0;
    while (sqlite3_step(stmt) == SQLITE_ROW && count < max_ids) {
        recording_ids[count++] = (uint64_t)sqlite3_column_int64(stmt, 0);
    }
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(mtx);
    return count;
}
