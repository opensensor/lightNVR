#define _POSIX_C_SOURCE 200809L

#include <stdarg.h>
#include <stdio.h>
#include <string.h>

#include <sqlite3.h>

#include "database/db_investigation_search.h"
#include "database/db_core.h"
#define LOG_COMPONENT "InvestigationSearchDB"
#include "core/logger.h"
#include "utils/strings.h"

#define SEARCH_SQL_MAX 16384
#define EVENT_TYPE_SQL \
    "CASE WHEN d.source = 'external_motion' THEN 'motion' ELSE 'detection' END"
#define ASSOCIATED_RECORDING_ID_SQL \
    "COALESCE(d.recording_id, (SELECT candidate.id FROM recordings candidate " \
    "WHERE candidate.camera_uuid = d.camera_uuid " \
    "AND candidate.start_time <= CASE " \
    "    WHEN d.source = 'external_motion' AND d.event_end_time IS NULL " \
    "    THEN CAST(strftime('%s','now') AS INTEGER) " \
    "    ELSE COALESCE(d.event_end_time, d.timestamp) END " \
    "AND candidate.end_time >= d.timestamp " \
    "ORDER BY candidate.start_time ASC, candidate.id ASC LIMIT 1))"
#define CAPTURE_METHOD_FILTER_SQL \
    "CASE WHEN COALESCE(filter_recording.trigger_type, 'scheduled') = 'scheduled' " \
    "AND filter_recording.schedule_restricted = 0 THEN 'continuous' " \
    "ELSE COALESCE(filter_recording.trigger_type, 'scheduled') END"
#define CAPTURE_METHOD_RESULT_SQL \
    "CASE WHEN r.id IS NULL THEN '' " \
    "WHEN COALESCE(r.trigger_type, 'scheduled') = 'scheduled' " \
    "AND r.schedule_restricted = 0 THEN 'continuous' " \
    "ELSE COALESCE(r.trigger_type, 'scheduled') END"
#define CAPTURE_METHOD_FACET_SQL \
    "CASE WHEN COALESCE(facet_recording.trigger_type, 'scheduled') = 'scheduled' " \
    "AND facet_recording.schedule_restricted = 0 THEN 'continuous' " \
    "ELSE COALESCE(facet_recording.trigger_type, 'scheduled') END"

typedef struct {
    char text[SEARCH_SQL_MAX];
    size_t used;
    bool failed;
} sql_builder_t;

static void sql_append(sql_builder_t *builder, const char *format, ...) {
    if (!builder || builder->failed || !format) return;
    va_list args;
    va_start(args, format);
    int written = vsnprintf(builder->text + builder->used,
                            sizeof(builder->text) - builder->used,
                            format, args);
    va_end(args);
    if (written < 0 || (size_t)written >= sizeof(builder->text) - builder->used) {
        builder->failed = true;
        return;
    }
    builder->used += (size_t)written;
}

static void append_placeholders(sql_builder_t *builder, int count) {
    for (int i = 0; i < count; i++) {
        sql_append(builder, "%s?", i == 0 ? "" : ",");
    }
}

static int build_where(const investigation_search_query_t *query,
                       bool include_cursor, sql_builder_t *builder) {
    if (!query || !builder || query->camera_count < 1) return -1;
    sql_append(builder, " WHERE d.camera_uuid IN (");
    append_placeholders(builder, query->camera_count);
    sql_append(builder,
               ") AND d.timestamp <= ? AND (d.timestamp >= ? OR "
               "(d.source = 'external_motion' AND "
               "COALESCE(d.event_end_time, CAST(strftime('%%s','now') AS INTEGER)) >= ?))");
    if (query->label_count > 0) {
        sql_append(builder, " AND d.label IN (");
        append_placeholders(builder, query->label_count);
        sql_append(builder, ")");
    }
    if (query->zone_count > 0) {
        sql_append(builder, " AND COALESCE(NULLIF(d.zone_id, ''), 'unassigned') IN (");
        append_placeholders(builder, query->zone_count);
        sql_append(builder, ")");
    }
    if (query->source_count > 0) {
        sql_append(builder,
                   " AND CASE WHEN d.source = '' THEN 'local' ELSE d.source END IN (");
        append_placeholders(builder, query->source_count);
        sql_append(builder, ")");
    }
    if (query->event_type_count > 0) {
        sql_append(builder, " AND " EVENT_TYPE_SQL " IN (");
        append_placeholders(builder, query->event_type_count);
        sql_append(builder, ")");
    }
    if (query->capture_method_count > 0) {
        sql_append(builder, "%s",
                   " AND EXISTS (SELECT 1 FROM recordings filter_recording "
                   "WHERE filter_recording.id = " ASSOCIATED_RECORDING_ID_SQL
                   " AND " CAPTURE_METHOD_FILTER_SQL " IN (");
        append_placeholders(builder, query->capture_method_count);
        sql_append(builder, "))");
    }
    if (query->recording_tag_count > 0) {
        sql_append(builder, "%s",
                   " AND EXISTS (SELECT 1 FROM recording_tags filter_tag "
                   "WHERE filter_tag.recording_id = "
                   ASSOCIATED_RECORDING_ID_SQL " AND filter_tag.tag IN (");
        append_placeholders(builder, query->recording_tag_count);
        sql_append(builder, "))");
    }
    if (query->protected_filter >= 0) {
        sql_append(builder, "%s",
                   " AND EXISTS (SELECT 1 FROM recordings filter_recording "
                   "WHERE filter_recording.id = " ASSOCIATED_RECORDING_ID_SQL
                   " AND filter_recording.protected = ?)");
    }
    if (query->has_min_confidence) {
        sql_append(builder, " AND d.confidence >= ?");
    }
    if (query->has_max_confidence) {
        sql_append(builder, " AND d.confidence <= ?");
    }
    if (include_cursor && query->has_cursor) {
        sql_append(builder,
                   " AND (d.timestamp < ? OR (d.timestamp = ? AND d.id < ?))");
    }
    return builder->failed ? -1 : 0;
}

static int bind_where(sqlite3_stmt *statement,
                      const investigation_search_query_t *query,
                      bool include_cursor, int parameter) {
    for (int i = 0; i < query->camera_count; i++) {
        sqlite3_bind_text(statement, parameter++, query->camera_uuids[i], -1,
                          SQLITE_STATIC);
    }
    sqlite3_bind_int64(statement, parameter++,
                       (sqlite3_int64)query->end_time);
    sqlite3_bind_int64(statement, parameter++,
                       (sqlite3_int64)query->start_time);
    sqlite3_bind_int64(statement, parameter++,
                       (sqlite3_int64)query->start_time);
    for (int i = 0; i < query->label_count; i++) {
        sqlite3_bind_text(statement, parameter++, query->labels[i], -1,
                          SQLITE_STATIC);
    }
    for (int i = 0; i < query->zone_count; i++) {
        sqlite3_bind_text(statement, parameter++, query->zones[i], -1,
                          SQLITE_STATIC);
    }
    for (int i = 0; i < query->source_count; i++) {
        sqlite3_bind_text(statement, parameter++, query->sources[i], -1,
                          SQLITE_STATIC);
    }
    for (int i = 0; i < query->event_type_count; i++) {
        sqlite3_bind_text(statement, parameter++, query->event_types[i], -1,
                          SQLITE_STATIC);
    }
    for (int i = 0; i < query->capture_method_count; i++) {
        sqlite3_bind_text(statement, parameter++, query->capture_methods[i],
                          -1, SQLITE_STATIC);
    }
    for (int i = 0; i < query->recording_tag_count; i++) {
        sqlite3_bind_text(statement, parameter++, query->recording_tags[i],
                          -1, SQLITE_STATIC);
    }
    if (query->protected_filter >= 0) {
        sqlite3_bind_int(statement, parameter++, query->protected_filter);
    }
    if (query->has_min_confidence) {
        sqlite3_bind_double(statement, parameter++, query->min_confidence);
    }
    if (query->has_max_confidence) {
        sqlite3_bind_double(statement, parameter++, query->max_confidence);
    }
    if (include_cursor && query->has_cursor) {
        sqlite3_bind_int64(statement, parameter++,
                           (sqlite3_int64)query->cursor_timestamp);
        sqlite3_bind_int64(statement, parameter++,
                           (sqlite3_int64)query->cursor_timestamp);
        sqlite3_bind_int64(statement, parameter++,
                           (sqlite3_int64)query->cursor_id);
    }
    return parameter;
}

static int prepare_with_where(sqlite3 *database, const char *prefix,
                              const investigation_search_query_t *query,
                              bool include_cursor, const char *suffix,
                              int first_where_parameter,
                              sqlite3_stmt **statement) {
    sql_builder_t builder = {0};
    sql_append(&builder, "%s", prefix);
    if (build_where(query, include_cursor, &builder) != 0) return -1;
    sql_append(&builder, "%s", suffix ? suffix : "");
    if (builder.failed) return -1;
    int result = sqlite3_prepare_v2(database, builder.text, -1, statement, NULL);
    if (result != SQLITE_OK) {
        log_error("Failed to prepare investigation query: %s",
                  sqlite3_errmsg(database));
        return -1;
    }
    return bind_where(*statement, query, include_cursor,
                      first_where_parameter);
}

static int load_results(sqlite3 *database,
                        const investigation_search_query_t *query,
                        investigation_search_result_t *results,
                        investigation_search_summary_t *summary) {
    static const char *prefix =
        "SELECT d.id, d.camera_uuid, d.stream_name, d.timestamp, "
        "CASE WHEN d.source = 'external_motion' AND d.event_end_time IS NULL "
        "     THEN CAST(strftime('%s','now') AS INTEGER) "
        "     ELSE COALESCE(d.event_end_time, d.timestamp) END, "
        "d.label, d.confidence, d.x, d.y, d.width, d.height, "
        "COALESCE(d.zone_id, ''), COALESCE(d.track_id, -1), "
        "CASE WHEN d.source = '' THEN 'local' ELSE d.source END, "
        "COALESCE(r.id, 0), CASE WHEN r.id IS NULL THEN 0 ELSE 1 END, "
        CAPTURE_METHOD_RESULT_SQL ", COALESCE(r.protected, 0) "
        "FROM detections d LEFT JOIN recordings r ON r.id = "
        ASSOCIATED_RECORDING_ID_SQL;
    sqlite3_stmt *statement = NULL;
    int next_parameter = prepare_with_where(
        database, prefix, query, true,
        " ORDER BY d.timestamp DESC, d.id DESC LIMIT ?;", 1, &statement);
    if (next_parameter < 0) return -1;
    sqlite3_bind_int(statement, next_parameter, query->limit + 1);

    int loaded = 0;
    int step = SQLITE_ROW;
    while ((step = sqlite3_step(statement)) == SQLITE_ROW) {
        if (loaded >= query->limit) {
            summary->has_more = true;
            break;
        }
        investigation_search_result_t *result = &results[loaded++];
        memset(result, 0, sizeof(*result));
        result->detection_id = (uint64_t)sqlite3_column_int64(statement, 0);
        const char *camera_uuid = (const char *)sqlite3_column_text(statement, 1);
        const char *stream_name = (const char *)sqlite3_column_text(statement, 2);
        const char *label = (const char *)sqlite3_column_text(statement, 5);
        const char *zone = (const char *)sqlite3_column_text(statement, 11);
        const char *source = (const char *)sqlite3_column_text(statement, 13);
        safe_strcpy(result->camera_uuid, camera_uuid ? camera_uuid : "",
                    sizeof(result->camera_uuid), 0);
        safe_strcpy(result->legacy_stream_name,
                    stream_name ? stream_name : "",
                    sizeof(result->legacy_stream_name), 0);
        result->start_time = (time_t)sqlite3_column_int64(statement, 3);
        result->end_time = (time_t)sqlite3_column_int64(statement, 4);
        safe_strcpy(result->label, label ? label : "", sizeof(result->label), 0);
        result->confidence = sqlite3_column_double(statement, 6);
        result->has_box = sqlite3_column_type(statement, 7) != SQLITE_NULL &&
                          sqlite3_column_type(statement, 8) != SQLITE_NULL &&
                          sqlite3_column_type(statement, 9) != SQLITE_NULL &&
                          sqlite3_column_type(statement, 10) != SQLITE_NULL;
        if (result->has_box) {
            result->x = sqlite3_column_double(statement, 7);
            result->y = sqlite3_column_double(statement, 8);
            result->width = sqlite3_column_double(statement, 9);
            result->height = sqlite3_column_double(statement, 10);
        }
        safe_strcpy(result->zone_uuid, zone ? zone : "",
                    sizeof(result->zone_uuid), 0);
        result->track_id = sqlite3_column_int(statement, 12);
        safe_strcpy(result->source, source ? source : "local",
                    sizeof(result->source), 0);
        result->recording_id = (uint64_t)sqlite3_column_int64(statement, 14);
        result->media_available = sqlite3_column_int(statement, 15) != 0;
        const char *capture_method =
            (const char *)sqlite3_column_text(statement, 16);
        safe_strcpy(result->capture_method,
                    capture_method ? capture_method : "",
                    sizeof(result->capture_method), 0);
        result->recording_protected =
            sqlite3_column_int(statement, 17) != 0;
    }
    if (step != SQLITE_ROW && step != SQLITE_DONE) {
        log_error("Investigation result query failed: %s",
                  sqlite3_errmsg(database));
        sqlite3_finalize(statement);
        return -1;
    }
    sqlite3_finalize(statement);
    summary->result_count = loaded;
    return 0;
}

static int load_total(sqlite3 *database,
                      const investigation_search_query_t *query,
                      investigation_search_summary_t *summary) {
    sqlite3_stmt *statement = NULL;
    if (prepare_with_where(database, "SELECT COUNT(*) FROM detections d", query,
                           false, ";", 1, &statement) < 0) {
        return -1;
    }
    int step = sqlite3_step(statement);
    if (step == SQLITE_ROW) {
        summary->total_count = sqlite3_column_int64(statement, 0);
    }
    sqlite3_finalize(statement);
    return step == SQLITE_ROW ? 0 : -1;
}

static int load_facet(sqlite3 *database,
                      const investigation_search_query_t *query,
                      const char *expression, const char *from_clause,
                      investigation_search_facet_t *facets, int *facet_count) {
    sql_builder_t prefix = {0};
    sql_append(&prefix, "SELECT %s AS value, COUNT(*) %s", expression,
               from_clause);
    sql_builder_t suffix = {0};
    sql_append(&suffix,
               " GROUP BY value ORDER BY COUNT(*) DESC, value ASC LIMIT %d;",
               INVESTIGATION_SEARCH_MAX_FACETS);
    sqlite3_stmt *statement = NULL;
    if (prefix.failed || suffix.failed || prepare_with_where(
            database, prefix.text, query, false, suffix.text, 1,
            &statement) < 0) {
        return -1;
    }
    int count = 0;
    int step;
    while ((step = sqlite3_step(statement)) == SQLITE_ROW &&
           count < INVESTIGATION_SEARCH_MAX_FACETS) {
        const char *value = (const char *)sqlite3_column_text(statement, 0);
        safe_strcpy(facets[count].value, value ? value : "",
                    sizeof(facets[count].value), 0);
        facets[count].count = sqlite3_column_int64(statement, 1);
        count++;
    }
    sqlite3_finalize(statement);
    *facet_count = count;
    return step == SQLITE_DONE ? 0 : -1;
}

static int load_histogram(sqlite3 *database,
                          const investigation_search_query_t *query,
                          investigation_search_summary_t *summary) {
    int64_t range = (int64_t)query->end_time - (int64_t)query->start_time + 1;
    int bucket_seconds = (int)((range +
        INVESTIGATION_SEARCH_MAX_HISTOGRAM_BUCKETS - 1) /
        INVESTIGATION_SEARCH_MAX_HISTOGRAM_BUCKETS);
    if (bucket_seconds < 1) bucket_seconds = 1;
    summary->histogram_bucket_seconds = bucket_seconds;

    sqlite3_stmt *statement = NULL;
    int next_parameter = prepare_with_where(
        database,
        "SELECT CASE WHEN d.timestamp < ? THEN 0 "
        "ELSE CAST((d.timestamp - ?) / ? AS INTEGER) END AS bucket, COUNT(*) "
        "FROM detections d",
        query, false, " GROUP BY bucket ORDER BY bucket ASC;", 4,
        &statement);
    if (next_parameter < 0) return -1;
    sqlite3_bind_int64(statement, 1, (sqlite3_int64)query->start_time);
    sqlite3_bind_int64(statement, 2, (sqlite3_int64)query->start_time);
    sqlite3_bind_int(statement, 3, bucket_seconds);

    int count = 0;
    int step;
    while ((step = sqlite3_step(statement)) == SQLITE_ROW &&
           count < INVESTIGATION_SEARCH_MAX_HISTOGRAM_BUCKETS) {
        int64_t bucket = sqlite3_column_int64(statement, 0);
        time_t bucket_start = query->start_time +
                              (time_t)(bucket * bucket_seconds);
        investigation_search_histogram_bucket_t *output =
            &summary->histogram[count++];
        output->start_time = bucket_start;
        output->end_time = bucket_start + bucket_seconds;
        if (output->end_time > query->end_time) {
            output->end_time = query->end_time;
        }
        output->count = sqlite3_column_int64(statement, 1);
    }
    sqlite3_finalize(statement);
    summary->histogram_count = count;
    return step == SQLITE_DONE ? 0 : -1;
}

static int load_unresolved_legacy_count(
    sqlite3 *database, const investigation_search_query_t *query,
    investigation_search_summary_t *summary) {
    sql_builder_t builder = {0};
    sql_append(&builder,
               "SELECT COUNT(*) FROM detections WHERE camera_uuid IS NULL "
               "AND stream_name IN (");
    append_placeholders(&builder, query->camera_count);
    sql_append(&builder, ") AND timestamp >= ? AND timestamp <= ?;");
    if (builder.failed) return -1;
    sqlite3_stmt *statement = NULL;
    if (sqlite3_prepare_v2(database, builder.text, -1, &statement, NULL) !=
        SQLITE_OK) {
        return -1;
    }
    int parameter = 1;
    for (int i = 0; i < query->camera_count; i++) {
        sqlite3_bind_text(statement, parameter++, query->legacy_stream_names[i],
                          -1, SQLITE_STATIC);
    }
    sqlite3_bind_int64(statement, parameter++,
                       (sqlite3_int64)query->start_time);
    sqlite3_bind_int64(statement, parameter,
                       (sqlite3_int64)query->end_time);
    int step = sqlite3_step(statement);
    if (step == SQLITE_ROW) {
        summary->unresolved_legacy_count = sqlite3_column_int64(statement, 0);
    }
    sqlite3_finalize(statement);
    return step == SQLITE_ROW ? 0 : -1;
}

int db_investigation_search(
    const investigation_search_query_t *query,
    investigation_search_result_t *results,
    investigation_search_summary_t *summary) {
    if (!query || !results || !summary || query->camera_count < 1 ||
        query->camera_count > INVESTIGATION_SEARCH_MAX_CAMERAS ||
        query->label_count < 0 ||
        query->label_count > INVESTIGATION_SEARCH_MAX_FILTER_VALUES ||
        query->zone_count < 0 ||
        query->zone_count > INVESTIGATION_SEARCH_MAX_FILTER_VALUES ||
        query->source_count < 0 ||
        query->source_count > INVESTIGATION_SEARCH_MAX_FILTER_VALUES ||
        query->event_type_count < 0 ||
        query->event_type_count > INVESTIGATION_SEARCH_MAX_FILTER_VALUES ||
        query->capture_method_count < 0 ||
        query->capture_method_count > INVESTIGATION_SEARCH_MAX_FILTER_VALUES ||
        query->recording_tag_count < 0 ||
        query->recording_tag_count > INVESTIGATION_SEARCH_MAX_FILTER_VALUES ||
        query->protected_filter < -1 || query->protected_filter > 1 ||
        query->limit < 1 || query->limit > INVESTIGATION_SEARCH_MAX_RESULTS) {
        return -1;
    }
    sqlite3 *database = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!database || !mutex) return -1;
    memset(summary, 0, sizeof(*summary));

    pthread_mutex_lock(mutex);
    int result = load_results(database, query, results, summary);
    if (result == 0) result = load_total(database, query, summary);
    if (result == 0) result = load_facet(
        database, query, "d.camera_uuid", "FROM detections d",
        summary->facets.cameras,
        &summary->facets.camera_count);
    if (result == 0) result = load_facet(
        database, query, "d.label", "FROM detections d",
        summary->facets.labels,
        &summary->facets.label_count);
    if (result == 0) result = load_facet(
        database, query,
        "COALESCE(NULLIF(d.zone_id, ''), 'unassigned')",
        "FROM detections d",
        summary->facets.zones, &summary->facets.zone_count);
    if (result == 0) result = load_facet(
        database, query,
        "CASE WHEN d.source = '' THEN 'local' ELSE d.source END",
        "FROM detections d",
        summary->facets.sources, &summary->facets.source_count);
    if (result == 0) result = load_facet(
        database, query, EVENT_TYPE_SQL, "FROM detections d",
        summary->facets.event_types,
        &summary->facets.event_type_count);
    if (result == 0) result = load_facet(
        database, query, CAPTURE_METHOD_FACET_SQL,
        "FROM detections d JOIN recordings facet_recording "
        "ON facet_recording.id = " ASSOCIATED_RECORDING_ID_SQL,
        summary->facets.capture_methods,
        &summary->facets.capture_method_count);
    if (result == 0) result = load_facet(
        database, query, "facet_tag.tag",
        "FROM detections d JOIN recording_tags facet_tag "
        "ON facet_tag.recording_id = " ASSOCIATED_RECORDING_ID_SQL,
        summary->facets.recording_tags,
        &summary->facets.recording_tag_count);
    if (result == 0) result = load_facet(
        database, query,
        "CASE WHEN facet_recording.protected = 1 "
        "THEN 'protected' ELSE 'unprotected' END",
        "FROM detections d JOIN recordings facet_recording "
        "ON facet_recording.id = " ASSOCIATED_RECORDING_ID_SQL,
        summary->facets.protection,
        &summary->facets.protection_count);
    if (result == 0) result = load_histogram(database, query, summary);
    if (result == 0) {
        result = load_unresolved_legacy_count(database, query, summary);
    }
    pthread_mutex_unlock(mutex);
    return result;
}
