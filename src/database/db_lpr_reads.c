#define _POSIX_C_SOURCE 200809L

#include "database/db_lpr_reads.h"

#include <sqlite3.h>
#include <math.h>
#include <stdio.h>
#include <string.h>
#include <strings.h>
#include <time.h>

#include "core/logger.h"
#include "database/db_core.h"
#include "utils/lpr_crypto.h"
#include "utils/memory.h"
#include "utils/uuid.h"

#define LPR_DB_CANDIDATE_LIMIT 5000
#define LPR_DB_RESULT_LIMIT 1000
#define LPR_DEDUPE_WINDOW_MS 5000

static int64_t current_time_ms(void) {
    struct timespec now;
    if (clock_gettime(CLOCK_REALTIME, &now) != 0) return (int64_t)time(NULL) * 1000;
    return (int64_t)now.tv_sec * 1000 + now.tv_nsec / 1000000;
}

static bool valid_source(const char *source) {
    static const char *sources[] = {
        "onvif_profile_m", "onvif_vendor", "vendor_api", "metadata_track"
    };
    for (size_t i = 0; i < sizeof(sources) / sizeof(sources[0]); ++i) {
        if (source && strcmp(source, sources[i]) == 0) return true;
    }
    return false;
}

static bool terminated_text(const char *value, size_t size) {
    return value && memchr(value, '\0', size) != NULL;
}

static bool contains_case_insensitive(const char *value, const char *needle) {
    if (!value || !needle || !needle[0]) return false;
    size_t length = strlen(needle);
    for (const char *cursor = value; *cursor; ++cursor) {
        if (strncasecmp(cursor, needle, length) == 0) return true;
    }
    return false;
}

static int make_aad(const char *camera_uuid, int64_t observed_at_ms,
                    const char *source, char *output, size_t output_size) {
    int written = snprintf(output, output_size, "%s|%lld|%s",
                           camera_uuid, (long long)observed_at_ms, source);
    return written > 0 && (size_t)written < output_size ? written : -1;
}

static int make_dedupe(const lpr_read_input_t *input, const char *canonical,
                       uint8_t output[LPR_CRYPTO_HMAC_SIZE]) {
    char material[1024];
    int written = snprintf(material, sizeof(material),
                           "%s|%s|%s|%s|%s|%s|%lld",
                           input->camera_uuid, canonical, input->source,
                           input->vendor_topic, input->object_id,
                           input->correlation_id,
                           (long long)(input->observed_at_ms /
                                      LPR_DEDUPE_WINDOW_MS));
    if (written < 0 || (size_t)written >= sizeof(material)) return -1;
    return lpr_crypto_fingerprint(material, (size_t)written, output);
}

static void bind_nullable_text(sqlite3_stmt *stmt, int index, const char *value) {
    if (value && value[0]) sqlite3_bind_text(stmt, index, value, -1, SQLITE_TRANSIENT);
    else sqlite3_bind_null(stmt, index);
}

int db_lpr_read_insert(const lpr_read_input_t *input,
                       char uuid_out[LPR_READ_UUID_SIZE]) {
    if (uuid_out) uuid_out[0] = '\0';
    if (!input || !terminated_text(input->camera_uuid,
                                   sizeof(input->camera_uuid)) ||
        !lightnvr_uuid_is_valid(input->camera_uuid) ||
        !terminated_text(input->stream_name, sizeof(input->stream_name)) ||
        !input->stream_name[0] ||
        !terminated_text(input->source, sizeof(input->source)) ||
        !terminated_text(input->plate, sizeof(input->plate)) ||
        !terminated_text(input->vendor_topic, sizeof(input->vendor_topic)) ||
        !terminated_text(input->country, sizeof(input->country)) ||
        !terminated_text(input->region, sizeof(input->region)) ||
        !terminated_text(input->plate_type, sizeof(input->plate_type)) ||
        !terminated_text(input->direction, sizeof(input->direction)) ||
        !terminated_text(input->lane, sizeof(input->lane)) ||
        !terminated_text(input->vehicle_type, sizeof(input->vehicle_type)) ||
        !terminated_text(input->vehicle_color, sizeof(input->vehicle_color)) ||
        !terminated_text(input->object_id, sizeof(input->object_id)) ||
        !terminated_text(input->correlation_id, sizeof(input->correlation_id)) ||
        !valid_source(input->source) || input->observed_at_ms <= 0 ||
        (input->has_confidence &&
         (!isfinite(input->confidence) || input->confidence < 0.0f ||
          input->confidence > 1.0f)) ||
        (input->has_bounding_box &&
         (!isfinite(input->bbox_left) || !isfinite(input->bbox_top) ||
          !isfinite(input->bbox_right) || !isfinite(input->bbox_bottom) ||
          input->bbox_left < -1.0f || input->bbox_left > 1.0f ||
          input->bbox_top < -1.0f || input->bbox_top > 1.0f ||
          input->bbox_right < -1.0f || input->bbox_right > 1.0f ||
          input->bbox_bottom < -1.0f || input->bbox_bottom > 1.0f ||
          input->bbox_right < input->bbox_left ||
          input->bbox_bottom < input->bbox_top))) return -1;

    char canonical[ONVIF_LPR_PLATE_MAX];
    if (lpr_canonicalize_plate(input->plate, canonical, sizeof(canonical)) < 0)
        return -1;

    char aad[256];
    int aad_len = make_aad(input->camera_uuid, input->observed_at_ms,
                           input->source, aad, sizeof(aad));
    if (aad_len < 0) {
        secure_zero_memory(canonical, sizeof(canonical));
        return -1;
    }

    uint8_t nonce[LPR_CRYPTO_NONCE_SIZE] = {0};
    uint8_t ciphertext[ONVIF_LPR_PLATE_MAX] = {0};
    uint8_t tag[LPR_CRYPTO_TAG_SIZE] = {0};
    uint8_t exact_hmac[LPR_CRYPTO_HMAC_SIZE] = {0};
    uint8_t dedupe_hmac[LPR_CRYPTO_HMAC_SIZE] = {0};
    size_t ciphertext_len = 0;
    if (lpr_crypto_encrypt(canonical, aad, (size_t)aad_len, nonce,
                           ciphertext, sizeof(ciphertext), &ciphertext_len, tag) != 0 ||
        lpr_crypto_blind_index(canonical, exact_hmac) != 0 ||
        make_dedupe(input, canonical, dedupe_hmac) != 0) {
        secure_zero_memory(canonical, sizeof(canonical));
        secure_zero_memory(nonce, sizeof(nonce));
        secure_zero_memory(ciphertext, sizeof(ciphertext));
        secure_zero_memory(tag, sizeof(tag));
        secure_zero_memory(exact_hmac, sizeof(exact_hmac));
        secure_zero_memory(dedupe_hmac, sizeof(dedupe_hmac));
        return -1;
    }
    secure_zero_memory(canonical, sizeof(canonical));

    char uuid[LPR_READ_UUID_SIZE];
    if (lightnvr_uuid_generate_v4(uuid) != 0) {
        secure_zero_memory(nonce, sizeof(nonce));
        secure_zero_memory(ciphertext, sizeof(ciphertext));
        secure_zero_memory(tag, sizeof(tag));
        secure_zero_memory(exact_hmac, sizeof(exact_hmac));
        secure_zero_memory(dedupe_hmac, sizeof(dedupe_hmac));
        return -1;
    }
    int64_t received_at_ms = current_time_ms();

    const char *sql =
        "INSERT OR IGNORE INTO lpr_reads ("
        "uuid,camera_uuid,stream_name,observed_at_ms,received_at_ms,source,vendor_topic,"
        "plate_nonce,plate_ciphertext,plate_tag,plate_exact_hmac,dedupe_hmac,confidence,"
        "country,region,plate_type,direction,lane,vehicle_type,vehicle_color,object_id,"
        "correlation_id,bbox_left,bbox_top,bbox_right,bbox_bottom,recording_id) "
        "VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?);";

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) {
        secure_zero_memory(nonce, sizeof(nonce));
        secure_zero_memory(ciphertext, sizeof(ciphertext));
        secure_zero_memory(tag, sizeof(tag));
        secure_zero_memory(exact_hmac, sizeof(exact_hmac));
        secure_zero_memory(dedupe_hmac, sizeof(dedupe_hmac));
        return -1;
    }
    pthread_mutex_lock(mutex);
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 2, input->camera_uuid, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 3, input->stream_name, -1, SQLITE_TRANSIENT);
        sqlite3_bind_int64(stmt, 4, input->observed_at_ms);
        sqlite3_bind_int64(stmt, 5, received_at_ms);
        sqlite3_bind_text(stmt, 6, input->source, -1, SQLITE_TRANSIENT);
        sqlite3_bind_text(stmt, 7, input->vendor_topic, -1, SQLITE_TRANSIENT);
        sqlite3_bind_blob(stmt, 8, nonce, sizeof(nonce), SQLITE_TRANSIENT);
        sqlite3_bind_blob(stmt, 9, ciphertext, (int)ciphertext_len, SQLITE_TRANSIENT);
        sqlite3_bind_blob(stmt, 10, tag, sizeof(tag), SQLITE_TRANSIENT);
        sqlite3_bind_blob(stmt, 11, exact_hmac, sizeof(exact_hmac), SQLITE_TRANSIENT);
        sqlite3_bind_blob(stmt, 12, dedupe_hmac, sizeof(dedupe_hmac), SQLITE_TRANSIENT);
        if (input->has_confidence) sqlite3_bind_double(stmt, 13, input->confidence);
        else sqlite3_bind_null(stmt, 13);
        bind_nullable_text(stmt, 14, input->country);
        bind_nullable_text(stmt, 15, input->region);
        bind_nullable_text(stmt, 16, input->plate_type);
        bind_nullable_text(stmt, 17, input->direction);
        bind_nullable_text(stmt, 18, input->lane);
        bind_nullable_text(stmt, 19, input->vehicle_type);
        bind_nullable_text(stmt, 20, input->vehicle_color);
        bind_nullable_text(stmt, 21, input->object_id);
        bind_nullable_text(stmt, 22, input->correlation_id);
        if (input->has_bounding_box) {
            sqlite3_bind_double(stmt, 23, input->bbox_left);
            sqlite3_bind_double(stmt, 24, input->bbox_top);
            sqlite3_bind_double(stmt, 25, input->bbox_right);
            sqlite3_bind_double(stmt, 26, input->bbox_bottom);
        } else {
            for (int i = 23; i <= 26; ++i) sqlite3_bind_null(stmt, i);
        }
        if (input->recording_id) sqlite3_bind_int64(stmt, 27, (sqlite3_int64)input->recording_id);
        else sqlite3_bind_null(stmt, 27);
        rc = sqlite3_step(stmt);
    }
    int changed = sqlite3_changes(db);
    if (stmt) sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);

    secure_zero_memory(nonce, sizeof(nonce));
    secure_zero_memory(ciphertext, sizeof(ciphertext));
    secure_zero_memory(tag, sizeof(tag));
    secure_zero_memory(exact_hmac, sizeof(exact_hmac));
    secure_zero_memory(dedupe_hmac, sizeof(dedupe_hmac));
    if (rc != SQLITE_DONE) {
        log_error("Failed to store protected LPR read: %s", sqlite3_errmsg(db));
        return -1;
    }
    if (changed == 0) return 1;
    if (uuid_out) snprintf(uuid_out, LPR_READ_UUID_SIZE, "%s", uuid);
    return 0;
}

static void copy_sql_text(char *output, size_t size, sqlite3_stmt *stmt, int column) {
    const unsigned char *value = sqlite3_column_text(stmt, column);
    snprintf(output, size, "%s", value ? (const char *)value : "");
}

static int decode_row(sqlite3_stmt *stmt, lpr_read_t *read) {
    memset(read, 0, sizeof(*read));
    copy_sql_text(read->uuid, sizeof(read->uuid), stmt, 0);
    copy_sql_text(read->read.camera_uuid, sizeof(read->read.camera_uuid), stmt, 1);
    copy_sql_text(read->read.stream_name, sizeof(read->read.stream_name), stmt, 2);
    read->read.observed_at_ms = sqlite3_column_int64(stmt, 3);
    read->received_at_ms = sqlite3_column_int64(stmt, 4);
    copy_sql_text(read->read.source, sizeof(read->read.source), stmt, 5);
    copy_sql_text(read->read.vendor_topic, sizeof(read->read.vendor_topic), stmt, 6);

    const uint8_t *nonce = sqlite3_column_blob(stmt, 7);
    int nonce_len = sqlite3_column_bytes(stmt, 7);
    const uint8_t *ciphertext = sqlite3_column_blob(stmt, 8);
    int ciphertext_len = sqlite3_column_bytes(stmt, 8);
    const uint8_t *tag = sqlite3_column_blob(stmt, 9);
    int tag_len = sqlite3_column_bytes(stmt, 9);
    if (!nonce || nonce_len != LPR_CRYPTO_NONCE_SIZE || !ciphertext ||
        ciphertext_len <= 0 || !tag || tag_len != LPR_CRYPTO_TAG_SIZE) return -1;
    char aad[256];
    int aad_len = make_aad(read->read.camera_uuid, read->read.observed_at_ms,
                           read->read.source, aad, sizeof(aad));
    if (aad_len < 0 || lpr_crypto_decrypt(
            ciphertext, (size_t)ciphertext_len, aad, (size_t)aad_len,
            nonce, tag, read->read.plate, sizeof(read->read.plate)) != 0) return -1;

    read->read.has_confidence = sqlite3_column_type(stmt, 10) != SQLITE_NULL;
    if (read->read.has_confidence) read->read.confidence = (float)sqlite3_column_double(stmt, 10);
    copy_sql_text(read->read.country, sizeof(read->read.country), stmt, 11);
    copy_sql_text(read->read.region, sizeof(read->read.region), stmt, 12);
    copy_sql_text(read->read.plate_type, sizeof(read->read.plate_type), stmt, 13);
    copy_sql_text(read->read.direction, sizeof(read->read.direction), stmt, 14);
    copy_sql_text(read->read.lane, sizeof(read->read.lane), stmt, 15);
    copy_sql_text(read->read.vehicle_type, sizeof(read->read.vehicle_type), stmt, 16);
    copy_sql_text(read->read.vehicle_color, sizeof(read->read.vehicle_color), stmt, 17);
    copy_sql_text(read->read.object_id, sizeof(read->read.object_id), stmt, 18);
    copy_sql_text(read->read.correlation_id, sizeof(read->read.correlation_id), stmt, 19);
    read->read.has_bounding_box = sqlite3_column_type(stmt, 20) != SQLITE_NULL;
    if (read->read.has_bounding_box) {
        read->read.bbox_left = (float)sqlite3_column_double(stmt, 20);
        read->read.bbox_top = (float)sqlite3_column_double(stmt, 21);
        read->read.bbox_right = (float)sqlite3_column_double(stmt, 22);
        read->read.bbox_bottom = (float)sqlite3_column_double(stmt, 23);
    }
    if (sqlite3_column_type(stmt, 24) != SQLITE_NULL)
        read->read.recording_id = (uint64_t)sqlite3_column_int64(stmt, 24);
    return 0;
}

int db_lpr_reads_search(const lpr_read_query_t *query,
                        lpr_read_t *reads, size_t capacity) {
    if (!query || !reads || capacity == 0 || query->start_at_ms <= 0 ||
        query->end_at_ms < query->start_at_ms || query->limit < 1 ||
        query->limit > LPR_DB_RESULT_LIMIT ||
        (query->camera_uuid[0] &&
         !lightnvr_uuid_is_valid(query->camera_uuid)) ||
        query->match_mode < LPR_MATCH_NONE ||
        query->match_mode > LPR_MATCH_PARTIAL ||
        !lpr_crypto_key_available()) return -1;

    char canonical[LPR_QUERY_MAX] = {0};
    uint8_t exact_hmac[LPR_CRYPTO_HMAC_SIZE];
    if (query->match_mode != LPR_MATCH_NONE) {
        int length = lpr_canonicalize_plate(query->plate_query, canonical,
                                            sizeof(canonical));
        if (length < 0 || (query->match_mode == LPR_MATCH_PARTIAL && length < 3))
            return -1;
        if (query->match_mode == LPR_MATCH_EXACT &&
            lpr_crypto_blind_index(canonical, exact_hmac) != 0) return -1;
    }

    const char *select =
        "SELECT uuid,camera_uuid,stream_name,observed_at_ms,received_at_ms,source,"
        "vendor_topic,plate_nonce,plate_ciphertext,plate_tag,confidence,country,region,"
        "plate_type,direction,lane,vehicle_type,vehicle_color,object_id,correlation_id,"
        "bbox_left,bbox_top,bbox_right,bbox_bottom,recording_id FROM lpr_reads "
        "WHERE observed_at_ms >= ? AND observed_at_ms <= ? ";
    char sql[1024];
    snprintf(sql, sizeof(sql), "%s%s%sORDER BY observed_at_ms DESC LIMIT ?;",
             select,
             query->camera_uuid[0] ? "AND camera_uuid = ? " : "",
             query->match_mode == LPR_MATCH_EXACT ? "AND plate_exact_hmac = ? " : "");

    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;
    pthread_mutex_lock(mutex);
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    int bind = 1;
    if (rc == SQLITE_OK) {
        sqlite3_bind_int64(stmt, bind++, query->start_at_ms);
        sqlite3_bind_int64(stmt, bind++, query->end_at_ms);
        if (query->camera_uuid[0])
            sqlite3_bind_text(stmt, bind++, query->camera_uuid, -1, SQLITE_TRANSIENT);
        if (query->match_mode == LPR_MATCH_EXACT)
            sqlite3_bind_blob(stmt, bind++, exact_hmac, sizeof(exact_hmac), SQLITE_TRANSIENT);
        int candidate_limit = query->match_mode == LPR_MATCH_PARTIAL
            ? LPR_DB_CANDIDATE_LIMIT : query->limit;
        sqlite3_bind_int(stmt, bind, candidate_limit);
    }

    int count = 0;
    while (rc == SQLITE_OK && (rc = sqlite3_step(stmt)) == SQLITE_ROW) {
        lpr_read_t decoded;
        if (decode_row(stmt, &decoded) != 0) {
            memset(&decoded, 0, sizeof(decoded));
            rc = SQLITE_ERROR;
            break;
        }
        bool match = query->match_mode != LPR_MATCH_PARTIAL ||
                     strstr(decoded.read.plate, canonical) != NULL;
        if (match && (size_t)count < capacity && count < query->limit)
            reads[count++] = decoded;
        memset(&decoded, 0, sizeof(decoded));
        if (count >= query->limit) break;
    }
    bool success = rc == SQLITE_DONE || rc == SQLITE_ROW || count >= query->limit;
    if (stmt) sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    memset(canonical, 0, sizeof(canonical));
    return success ? count : -1;
}

int db_lpr_read_delete(const char *uuid) {
    if (!lightnvr_uuid_is_valid(uuid)) return -1;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;
    pthread_mutex_lock(mutex);
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db, "DELETE FROM lpr_reads WHERE uuid = ?;", -1,
                                &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, uuid, -1, SQLITE_TRANSIENT);
        rc = sqlite3_step(stmt);
    }
    int changed = sqlite3_changes(db);
    if (stmt) sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    return rc == SQLITE_DONE && changed == 1 ? 0 : -1;
}

int db_lpr_read_get_camera_uuid(
    const char *uuid, char camera_uuid[CAMERA_UUID_STRING_SIZE]) {
    if (!lightnvr_uuid_is_valid(uuid) || !camera_uuid) return -1;
    camera_uuid[0] = '\0';
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;
    pthread_mutex_lock(mutex);
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(
        db, "SELECT camera_uuid FROM lpr_reads WHERE uuid=?;", -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, uuid, -1, SQLITE_TRANSIENT);
        rc = sqlite3_step(stmt);
        if (rc == SQLITE_ROW) {
            copy_sql_text(camera_uuid, CAMERA_UUID_STRING_SIZE, stmt, 0);
        }
    }
    if (stmt) sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    return rc == SQLITE_ROW && lightnvr_uuid_is_valid(camera_uuid) ? 0 : -1;
}

int db_lpr_reads_prune(int64_t received_before_ms, int batch_limit) {
    if (received_before_ms <= 0 || batch_limit < 1 || batch_limit > 10000) return -1;
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex) return -1;
    const char *sql =
        "DELETE FROM lpr_reads WHERE uuid IN (SELECT uuid FROM lpr_reads "
        "WHERE received_at_ms < ? ORDER BY received_at_ms LIMIT ?);";
    pthread_mutex_lock(mutex);
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db, sql, -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_int64(stmt, 1, received_before_ms);
        sqlite3_bind_int(stmt, 2, batch_limit);
        rc = sqlite3_step(stmt);
    }
    int changed = sqlite3_changes(db);
    if (stmt) sqlite3_finalize(stmt);
    pthread_mutex_unlock(mutex);
    return rc == SQLITE_DONE ? changed : -1;
}

int db_lpr_read_from_onvif(const char *camera_uuid, const char *stream_name,
                           const onvif_lpr_event_t *event,
                           lpr_read_input_t *input) {
    if (!lightnvr_uuid_is_valid(camera_uuid) || !stream_name ||
        !stream_name[0] || strlen(stream_name) >= MAX_STREAM_NAME ||
        !event || !event->asserted || !event->plate[0] || !input) return -1;
    memset(input, 0, sizeof(*input));
    snprintf(input->camera_uuid, sizeof(input->camera_uuid), "%s", camera_uuid);
    snprintf(input->stream_name, sizeof(input->stream_name), "%s", stream_name);
    input->observed_at_ms = event->observed_at_ms > 0
        ? event->observed_at_ms : current_time_ms();
    snprintf(input->source, sizeof(input->source), "%s",
             event->source == ONVIF_LPR_SOURCE_PROFILE_M
                 ? "onvif_profile_m" : "onvif_vendor");
    snprintf(input->vendor_topic, sizeof(input->vendor_topic), "%s",
             contains_case_insensitive(event->topic, event->plate)
                 ? "[plate-redacted-vendor-topic]" : event->topic);
    snprintf(input->plate, sizeof(input->plate), "%s", event->plate);
    input->has_confidence = event->has_confidence;
    input->confidence = event->confidence;
#define COPY_FIELD(field) snprintf(input->field, sizeof(input->field), "%s", event->field)
    COPY_FIELD(country);
    COPY_FIELD(region);
    COPY_FIELD(plate_type);
    COPY_FIELD(direction);
    COPY_FIELD(lane);
    COPY_FIELD(vehicle_type);
    COPY_FIELD(vehicle_color);
    COPY_FIELD(object_id);
    COPY_FIELD(correlation_id);
#undef COPY_FIELD
    input->has_bounding_box = event->has_bounding_box;
    input->bbox_left = event->bbox_left;
    input->bbox_top = event->bbox_top;
    input->bbox_right = event->bbox_right;
    input->bbox_bottom = event->bbox_bottom;
    return 0;
}
