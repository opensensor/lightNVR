/* SPDX-License-Identifier: GPL-3.0-or-later */
#define _POSIX_C_SOURCE 200809L
#define _XOPEN_SOURCE 700
#include <ftw.h>
#include <sqlite3.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include "unity.h"
#include "database/db_core.h"
#include "database/db_recordings.h"
#include "database/db_recording_tags.h"
#include "database/db_detections.h"
#include "web/api_handlers_timeline.h"

#define UUID_A "10000000-0000-4000-8000-000000000001"
#define UUID_B "20000000-0000-4000-8000-000000000002"
static char directory[] = "/tmp/lightnvr-recording-scope-XXXXXX";
static int variable_budget;
static const char *step_sql_match;
static int successful_steps;
static int query_budget;
static int query_progress_calls;
static int bound_query_work(void *unused) {
    (void)unused;
    return ++query_progress_calls > query_budget;
}
int __real_sqlite3_step(sqlite3_stmt *);
int __wrap_sqlite3_step(sqlite3_stmt *stmt) {
    const char *statement = sqlite3_sql(stmt);
    if (step_sql_match && statement && strstr(statement, step_sql_match) && successful_steps-- == 0)
        return SQLITE_IOERR;
    return __real_sqlite3_step(stmt);
}
int __real_db_open_readonly_connection(sqlite3 **);
int __wrap_db_open_readonly_connection(sqlite3 **db) {
    int rc = __real_db_open_readonly_connection(db);
    if (rc == 0 && variable_budget > 0) sqlite3_limit(*db, SQLITE_LIMIT_VARIABLE_NUMBER, variable_budget);
    if (rc == 0 && query_budget > 0) sqlite3_progress_handler(*db, 1000, bound_query_work, NULL);
    return rc;
}
static void sql(const char *text) {
    char *error = NULL;
    int rc = sqlite3_exec(get_db_handle(), text, NULL, NULL, &error);
    if (error) fprintf(stderr, "SQL: %s\n", error);
    sqlite3_free(error);
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, rc);
}
void setUp(void) {
    variable_budget = 0;
    query_budget = query_progress_calls = 0;
    step_sql_match = NULL;
    sql("DELETE FROM detections; DELETE FROM recording_tags; DELETE FROM recordings; DELETE FROM streams;"
        "INSERT INTO streams(name,url,camera_uuid) VALUES ('A','rtsp://fixture.invalid/a','" UUID_A "'),('B','rtsp://fixture.invalid/b','" UUID_B "');"
        "INSERT INTO recordings(id,stream_name,camera_uuid,file_path,start_time,end_time,is_complete,protected,trigger_type) VALUES "
        "(1,'A','" UUID_B "','/fixture/hidden.mp4',1000,1100,1,1,'scheduled'),"
        "(2,'B','" UUID_A "','/fixture/renamed.mp4',1000,1100,1,1,'scheduled'),"
        "(3,'A',NULL,'/fixture/legacy-null.mp4',1000,1100,1,1,'scheduled'),"
        "(4,'A','','/fixture/legacy-empty.mp4',1000,1100,1,1,'scheduled'),"
        "(5,'missing',NULL,'/fixture/orphan.mp4',1000,1100,1,1,'scheduled');"
        "INSERT INTO recording_tags(recording_id,tag) VALUES (1,'hidden'),(2,'renamed'),(3,'legacy-null'),(4,'legacy-empty'),(5,'orphan');"
        "INSERT INTO detections(stream_name,camera_uuid,timestamp,label,confidence,source,recording_id) VALUES "
        "('A','" UUID_B "',1050,'hidden',0.9,'',1),"
        "('B','" UUID_A "',1050,'visible',0.9,'',2),"
        "('A',NULL,1050,'legacy-null',0.9,'',3),"
        "('A','',1050,'legacy-empty',0.9,'',4),"
        "('A','" UUID_B "',1050,'hidden-linked',0.9,'',2);"
        "INSERT INTO detections(stream_name,camera_uuid,timestamp,event_end_time,label,confidence,source) VALUES "
        "('A','" UUID_B "',1010,1020,'hidden-motion',0.9,'external_motion'),"
        "('B','" UUID_A "',1030,1040,'visible-motion',0.9,'external_motion');");
}
void tearDown(void) { variable_budget = query_budget = 0; step_sql_match = NULL; }
static int count(const char *const *scope, int n, const char *label) {
    return get_recording_count_for_cameras(0,0,NULL,0,label,-1,scope,n,NULL,NULL,false);
}
static int page(const char *const *scope, int n, recording_metadata_t *rows, int capacity) {
    return get_recording_metadata_paginated_for_cameras(0,0,NULL,0,NULL,-1,"id","asc",rows,capacity,0,scope,n,NULL,NULL,false);
}
static void test_uuid_scope_excludes_reused_name_and_keeps_renamed_and_legacy(void) {
    const char *scope[] = {UUID_A};
    TEST_ASSERT_EQUAL_INT(3, count(scope,1,NULL));
    recording_metadata_t rows[8];
    TEST_ASSERT_EQUAL_INT(3, page(scope,1,rows,8));
    TEST_ASSERT_EQUAL_UINT64(2,rows[0].id);
    TEST_ASSERT_EQUAL_UINT64(3,rows[1].id);
    TEST_ASSERT_EQUAL_UINT64(4,rows[2].id);
    TEST_ASSERT_EQUAL_INT(0,count(NULL,0,NULL));
    TEST_ASSERT_EQUAL_INT(0,page(NULL,0,rows,8));
    TEST_ASSERT_EQUAL_INT(0,count(scope,1,"hidden"));
    /* No names are resolved from the supplied UUID after another rename. */
    sql("UPDATE streams SET name='A-renamed' WHERE camera_uuid='" UUID_A "';");
    TEST_ASSERT_EQUAL_INT(1,count(scope,1,NULL));
    TEST_ASSERT_EQUAL_INT(1,page(scope,1,rows,8));
    TEST_ASSERT_EQUAL_UINT64(2,rows[0].id);
}
static void test_generic_name_queries_keep_historical_semantics(void) {
    const char *names[] = {"A"};
    TEST_ASSERT_EQUAL_INT(3,get_recording_count(0,0,NULL,0,NULL,-1,names,1,NULL,NULL,false));
    recording_metadata_t rows[8];
    TEST_ASSERT_EQUAL_INT(3,get_recording_metadata_paginated(0,0,NULL,0,NULL,-1,"id","asc",rows,8,0,names,1,NULL,NULL,false));
    TEST_ASSERT_EQUAL_UINT64(1,rows[0].id);
}
static void test_facets_protected_count_and_empty_scope(void) {
    const char *scope[] = {UUID_A};
    char tags[16][MAX_TAG_LENGTH], labels[16][MAX_LABEL_LENGTH];
    TEST_ASSERT_EQUAL_INT(3,db_recording_tag_get_unique_for_cameras(scope,1,tags,16));
    TEST_ASSERT_EQUAL_STRING("legacy-empty",tags[0]);
    TEST_ASSERT_EQUAL_STRING("legacy-null",tags[1]);
    TEST_ASSERT_EQUAL_STRING("renamed",tags[2]);
    TEST_ASSERT_EQUAL_INT(4,get_unique_detection_labels_for_cameras(scope,1,labels,16));
    TEST_ASSERT_EQUAL_STRING("legacy-empty",labels[0]);
    TEST_ASSERT_EQUAL_STRING("legacy-null",labels[1]);
    TEST_ASSERT_EQUAL_STRING("visible",labels[2]);
    TEST_ASSERT_EQUAL_STRING("visible-motion",labels[3]);
    TEST_ASSERT_EQUAL_INT(3,get_protected_recordings_count_for_cameras(scope,1));
    TEST_ASSERT_EQUAL_INT(0,db_recording_tag_get_unique_for_cameras(NULL,0,tags,16));
    TEST_ASSERT_EQUAL_INT(0,get_unique_detection_labels_for_cameras(NULL,0,labels,16));
    TEST_ASSERT_EQUAL_INT(0,get_protected_recordings_count_for_cameras(NULL,0));
}
static void test_detection_summaries_and_timeline_do_not_leak_conflicting_uuid(void) {
    recording_metadata_t recording;
    TEST_ASSERT_EQUAL_INT(0,get_recording_metadata_by_id(2,&recording));
    recording_detection_summary_t summary;
    TEST_ASSERT_EQUAL_INT(0,get_recording_detection_summaries_for_cameras(&recording,1,&summary));
    TEST_ASSERT_TRUE(summary.has_detection);
    for(int i=0;i<summary.label_count;i++) TEST_ASSERT_NULL(strstr(summary.labels[i].label,"hidden"));
    timeline_segment_t segments[8];
    TEST_ASSERT_EQUAL_INT(3,get_timeline_segments_by_camera_uuid(UUID_A,900,1200,segments,8));
    for(int i=0;i<3;i++) TEST_ASSERT_NOT_EQUAL(1,segments[i].id);
    detection_interval_t intervals[8];
    TEST_ASSERT_EQUAL_INT(1,get_external_motion_detection_intervals_by_camera_uuid(UUID_A,900,1200,intervals,8));
    TEST_ASSERT_EQUAL_INT64(1030,intervals[0].start_time);
}
static void test_low_bind_budget_batches_facets_and_summaries_and_fails_closed_lists(void) {
    const char *scope[] = {UUID_A,"unused-1","unused-2","unused-3","unused-4","unused-5","unused-6","unused-7"};
    variable_budget = 6;
    char tags[16][MAX_TAG_LENGTH],labels[16][MAX_LABEL_LENGTH];
    TEST_ASSERT_EQUAL_INT(3,db_recording_tag_get_unique_for_cameras(scope,8,tags,16));
    TEST_ASSERT_EQUAL_INT(4,get_unique_detection_labels_for_cameras(scope,8,labels,16));
    recording_metadata_t rows[3];
    for(int i=0;i<3;i++) TEST_ASSERT_EQUAL_INT(0,get_recording_metadata_by_id((uint64_t)i+2,&rows[i]));
    recording_detection_summary_t summaries[3];
    TEST_ASSERT_EQUAL_INT(0,get_recording_detection_summaries_for_cameras(rows,3,summaries));
    TEST_ASSERT_EQUAL_INT(-1,count(scope,8,NULL));
    TEST_ASSERT_EQUAL_INT(-1,page(scope,8,rows,3));
    int previous = sqlite3_limit(get_db_handle(),SQLITE_LIMIT_VARIABLE_NUMBER,3);
    int protected_count = get_protected_recordings_count_for_cameras(scope,8);
    sqlite3_limit(get_db_handle(),SQLITE_LIMIT_VARIABLE_NUMBER,previous);
    TEST_ASSERT_EQUAL_INT(3,protected_count);
}
static void test_step_errors_never_return_partial_rows_or_successful_empty(void) {
    const char *scope[] = {UUID_A};
    recording_metadata_t rows[8];
    step_sql_match = "FROM recordings r";
    successful_steps = 0;
    TEST_ASSERT_EQUAL_INT(-1, count(scope, 1, NULL));
    successful_steps = 1;
    TEST_ASSERT_EQUAL_INT(-1, page(scope, 1, rows, 8));
    timeline_segment_t segments[8];
    successful_steps = 1;
    TEST_ASSERT_EQUAL_INT(-1, get_timeline_segments_by_camera_uuid(UUID_A,900,1200,segments,8));
    step_sql_match = "FROM detections d WHERE";
    successful_steps = 0;
    detection_interval_t intervals[8];
    TEST_ASSERT_EQUAL_INT(-1, get_external_motion_detection_intervals_by_camera_uuid(UUID_A,900,1200,intervals,8));
    step_sql_match = NULL;
}

static int label_walk_progress(void *context) {
    int *calls = context;
    return ++*calls > 20; /* abort an archive-size scan, at 1000 VM ops/call */
}
static void test_camera_label_walk_skips_modern_rows_in_legacy_branch(void) {
    sql("WITH RECURSIVE n(i) AS (VALUES(1) UNION ALL SELECT i+1 FROM n WHERE i<20000) "
        "INSERT INTO detections(stream_name,camera_uuid,timestamp,label,confidence,source) "
        "SELECT 'A','" UUID_A "',1005,'repeated',0.9,'' FROM n;");
    int progress_calls = 0;
    sqlite3 *db = get_db_handle();
    sqlite3_progress_handler(db, 1000, label_walk_progress, &progress_calls);
    const char *scope[] = {UUID_A};
    char labels[16][MAX_LABEL_LENGTH];
    int result = get_unique_detection_labels_for_cameras_on_connection(db,scope,1,labels,16);
    sqlite3_progress_handler(db, 0, NULL, NULL);
    TEST_ASSERT_EQUAL_INT(5,result);
    TEST_ASSERT_LESS_OR_EQUAL_INT(20,progress_calls);
}

static int remove_path(const char *path,const struct stat *st,int type,struct FTW *walk) {
    (void)st;(void)type;(void)walk;return remove(path);
}
static void test_timeline_and_summaries_skip_unrelated_detection_history(void) {
    sql("DELETE FROM detections;"
        "WITH RECURSIVE n(i) AS (VALUES(1) UNION ALL SELECT i+1 FROM n WHERE i<30000) "
        "INSERT INTO detections(stream_name,camera_uuid,timestamp,label,confidence,source) "
        "SELECT 'A','" UUID_B "',i%900,'unrelated',0.9,'' FROM n;");
    /* VM work budget, independent of wall-clock speed. The old identity
     * expression scanned detection history for every empty recording. */
    query_budget = 50;
    timeline_segment_t segments[8];
    TEST_ASSERT_EQUAL_INT(3, get_timeline_segments_by_camera_uuid(UUID_A,900,1200,segments,8));
    for (int i=0;i<3;i++) TEST_ASSERT_FALSE(segments[i].has_detection);
    detection_interval_t intervals[8];
    TEST_ASSERT_EQUAL_INT(0, get_external_motion_detection_intervals_by_camera_uuid(UUID_A,900,1200,intervals,8));
    recording_metadata_t recording;
    TEST_ASSERT_EQUAL_INT(0, get_recording_metadata_by_id(2,&recording));
    recording_detection_summary_t summary;
    TEST_ASSERT_EQUAL_INT(0, get_recording_detection_summaries_for_cameras(&recording,1,&summary));
    TEST_ASSERT_FALSE(summary.has_detection);
    timeline_coverage_interval_t coverage[8];
    int sources, seconds;
    TEST_ASSERT_EQUAL_INT(1, get_timeline_coverage_by_camera_uuid(UUID_A,900,1200,coverage,8,&sources,&seconds));
    TEST_ASSERT_EQUAL_INT(3, sources);
    TEST_ASSERT_FALSE(coverage[0].has_detection);
    TEST_ASSERT_LESS_OR_EQUAL_INT(50,query_progress_calls);
}

static void test_timeline_motion_keeps_legacy_identity_and_interval_overlap(void) {
    sql("DELETE FROM detections;"
        "INSERT INTO detections(stream_name,camera_uuid,timestamp,event_end_time,label,confidence,source) VALUES "
        "('A',NULL,800,1050,'legacy-null',0.9,'external_motion'),"
        "('A','',810,1060,'legacy-empty',0.9,'external_motion'),"
        "('B','" UUID_A "',820,1070,'renamed',0.9,'external_motion'),"
        "('A','" UUID_B "',830,1080,'conflict',0.9,'external_motion'),"
        "('missing',NULL,840,1090,'orphan',0.9,'external_motion');");
    detection_interval_t intervals[8];
    TEST_ASSERT_EQUAL_INT(3,get_external_motion_detection_intervals_by_camera_uuid(UUID_A,1000,1100,intervals,8));
    TEST_ASSERT_EQUAL_INT64(800,intervals[0].start_time);
    TEST_ASSERT_EQUAL_INT64(820,intervals[2].start_time);
    timeline_segment_t segments[8];
    TEST_ASSERT_EQUAL_INT(3,get_timeline_segments_by_camera_uuid(UUID_A,1000,1100,segments,8));
    for (int i=0;i<3;i++) TEST_ASSERT_TRUE(segments[i].has_detection);
}

static void assert_coverage_detection(bool expected) {
    timeline_coverage_interval_t coverage[8];
    int sources, seconds;
    TEST_ASSERT_EQUAL_INT(1, get_timeline_coverage_by_camera_uuid(UUID_A,1000,1100,coverage,8,&sources,&seconds));
    TEST_ASSERT_EQUAL_INT(3, sources);
    TEST_ASSERT_EQUAL_INT(expected, coverage[0].has_detection);
}
static void test_coverage_detection_identity_and_event_types(void) {
    sql("DELETE FROM detections;"
        "INSERT INTO detections(stream_name,camera_uuid,timestamp,label,confidence,source,recording_id) VALUES "
        "('A','" UUID_B "',1050,'conflicting-linked',0.9,'',2),"
        "('missing',NULL,1050,'orphan',0.9,'',NULL);");
    assert_coverage_detection(false);
    /* A UUID survives a stream rename, even without a recording link. */
    sql("INSERT INTO detections(stream_name,camera_uuid,timestamp,label,confidence,source) VALUES "
        "('old-name','" UUID_A "',1050,'renamed',0.9,'');");
    assert_coverage_detection(true);
    sql("DELETE FROM detections;"
        "INSERT INTO detections(stream_name,camera_uuid,timestamp,label,confidence,source) VALUES "
        "('A',NULL,1050,'legacy-null',0.9,'');");
    assert_coverage_detection(true);
    sql("UPDATE detections SET camera_uuid='';");
    assert_coverage_detection(true);
    /* Motion starts before the recording but overlaps it. */
    sql("DELETE FROM detections;"
        "INSERT INTO detections(stream_name,camera_uuid,timestamp,event_end_time,label,confidence,source) VALUES "
        "('old-name','" UUID_A "',800,1050,'motion',0.9,'external_motion');");
    assert_coverage_detection(true);
    sql("UPDATE detections SET event_end_time=900;");
    assert_coverage_detection(false);
    sql("UPDATE recordings SET trigger_type='detection' WHERE id=2;");
    assert_coverage_detection(true);
}
static void test_coverage_half_open_buckets_preserve_gaps_and_clip(void) {
    sql("DELETE FROM detections; DELETE FROM recordings;"
        "INSERT INTO recordings(id,stream_name,camera_uuid,file_path,start_time,end_time,size_bytes,is_complete,trigger_type) VALUES "
        "(10,'A','" UUID_A "','/fixture/first.mp4',990,1010,100,1,'scheduled'),"
        "(11,'A','" UUID_A "','/fixture/second.mp4',1020,1050,200,1,'scheduled'),"
        "(12,'A','" UUID_A "','/fixture/empty.mp4',1015,1015,300,1,'scheduled');");
    timeline_coverage_interval_t coverage[4];
    int sources, seconds;
    TEST_ASSERT_EQUAL_INT(2, get_timeline_coverage_by_camera_uuid(UUID_A,1000,1040,coverage,4,&sources,&seconds));
    TEST_ASSERT_EQUAL_INT(10, seconds);
    TEST_ASSERT_EQUAL_INT(3, sources); /* Includes the empty row scanned. */
    TEST_ASSERT_EQUAL_INT64(1000, coverage[0].start_time);
    TEST_ASSERT_EQUAL_INT64(1010, coverage[0].end_time);
    TEST_ASSERT_EQUAL_INT64(1020, coverage[1].start_time);
    TEST_ASSERT_EQUAL_INT64(1040, coverage[1].end_time);
    TEST_ASSERT_EQUAL_UINT64(100, coverage[0].size_bytes);
    TEST_ASSERT_EQUAL_UINT64(200, coverage[1].size_bytes);
    TEST_ASSERT_EQUAL_INT(1, coverage[0].source_segment_count);
    TEST_ASSERT_EQUAL_INT(1, coverage[1].source_segment_count);
}

int main(void) {
    if(!mkdtemp(directory))return 1;
    char path[256];snprintf(path,sizeof(path),"%s/db.sqlite",directory);
    if(init_database(path)!=0)return 1;
    UNITY_BEGIN();
    RUN_TEST(test_uuid_scope_excludes_reused_name_and_keeps_renamed_and_legacy);
    RUN_TEST(test_generic_name_queries_keep_historical_semantics);
    RUN_TEST(test_facets_protected_count_and_empty_scope);
    RUN_TEST(test_detection_summaries_and_timeline_do_not_leak_conflicting_uuid);
    RUN_TEST(test_low_bind_budget_batches_facets_and_summaries_and_fails_closed_lists);
    RUN_TEST(test_step_errors_never_return_partial_rows_or_successful_empty);
    RUN_TEST(test_camera_label_walk_skips_modern_rows_in_legacy_branch);
    RUN_TEST(test_timeline_and_summaries_skip_unrelated_detection_history);
    RUN_TEST(test_timeline_motion_keeps_legacy_identity_and_interval_overlap);
    RUN_TEST(test_coverage_detection_identity_and_event_types);
    RUN_TEST(test_coverage_half_open_buckets_preserve_gaps_and_clip);
    int result=UNITY_END();shutdown_database();nftw(directory,remove_path,16,FTW_DEPTH|FTW_PHYS);return result;
}
