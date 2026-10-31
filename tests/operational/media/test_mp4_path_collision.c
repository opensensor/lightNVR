/* SPDX-License-Identifier: GPL-3.0-or-later
 * Drive the real recorder loop at a fixed wall-clock second. Only demux/mux,
 * configuration, metadata persistence and pacing are deterministic boundaries;
 * path allocation and actual disposable file writes are real. */
#include "../../../src/video/mp4_writer_thread.c"
#include "unity.h"
#include <dirent.h>
#include <fcntl.h>
#include <stdarg.h>

static mp4_writer_t writer;
static mp4_writer_thread_t reader;
static char directory[128], planned[MAX_PATH_LENGTH];
static char paths[2][MAX_PATH_LENGTH];
static int calls, stop_after, metadata_count;
static bool fail_first, deny_reservation, stop_on_wait, fail_empty;
int __real_open(const char *,int,...);
int __wrap_open(const char *path,int flags,...) {
    mode_t mode=0;
    if(flags & O_CREAT){va_list args;va_start(args,flags);mode=va_arg(args,int);va_end(args);}
    if(deny_reservation && (flags & O_EXCL)){errno=EROFS;return -1;}
    return __real_open(path,flags,mode);
}
static recording_metadata_t rows[2];
static const time_t instant = 1793434973;

time_t __wrap_time(time_t *out) { if (out) *out = instant; return instant; }
int __wrap_get_stream_config_by_name(const char *name, stream_config_t *config) {
    (void)name; (void)config; return -1;
}
bool __wrap_storage_should_pause_recording(void) { return false; }
int __wrap_av_usleep(unsigned value) { (void)value; if(stop_on_wait)reader.running=0; return 0; }
int __wrap_prepare_placed_mp4_recording_path(const config_t *config, const char *name,
    time_t stamp, char *path, size_t size, storage_placement_t *placement) {
    (void)config; (void)name; (void)stamp;
    snprintf(path,size,"%s",planned);
    memset(placement,0,sizeof(*placement));
    snprintf(placement->target_root,sizeof(placement->target_root),"%s",directory);
    snprintf(placement->object_key,sizeof(placement->object_key),"recording_20261031_042253.mp4");
    placement->status=STORAGE_PLACEMENT_READY;
    return 0;
}
uint64_t __wrap_add_recording_metadata(const recording_metadata_t *metadata) {
    TEST_ASSERT_LESS_THAN_INT(2,metadata_count);
    rows[metadata_count]=*metadata;
    return (uint64_t)++metadata_count;
}
int __wrap_update_recording_metadata(uint64_t id,time_t end,uint64_t size,bool complete) {
    TEST_ASSERT_TRUE(id>0 && id<=2);
    rows[id-1].end_time=end; rows[id-1].size_bytes=size; rows[id-1].is_complete=complete;
    return 0;
}
int __wrap_record_segment(const char *url,const char *path,int duration,int audio,
    AVFormatContext **input,segment_info_t *segment,record_segment_started_cb started,
    record_segment_activity_cb activity,void *opaque,atomic_int *shutdown_flag) {
    (void)url;(void)duration;(void)audio;(void)input;(void)segment;(void)activity;(void)shutdown_flag;
    TEST_ASSERT_LESS_THAN_INT(2,calls);
    if(fail_empty){calls++;reader.running=0;return AVERROR(EIO);}
    snprintf(paths[calls],sizeof(paths[calls]),"%s",path);
    FILE *file=fopen(path,"wb");TEST_ASSERT_NOT_NULL(file);
    fprintf(file,"segment-%d",calls+1);fclose(file);
    started(opaque);
    calls++;
    if(calls==stop_after)reader.running=0;
    return fail_first && calls==1 ? AVERROR(EIO) : 0;
}
static void initialize_writer(void) {
    memset(&writer,0,sizeof(writer));memset(&reader,0,sizeof(reader));
    strcpy(writer.stream_name,"path-collision");
    snprintf(writer.output_path,sizeof(writer.output_path),"%s",planned);
    snprintf(writer.output_dir,sizeof(writer.output_dir),"%s",directory);
    snprintf(writer.placement.target_root,sizeof(writer.placement.target_root),"%s",directory);
    snprintf(writer.placement.object_key,sizeof(writer.placement.object_key),"recording_20261031_042253.mp4");
    writer.segment_duration=3;writer.last_rotation_time=instant-3;writer.thread_ctx=&reader;
    reader.writer=&writer;strcpy(reader.rtsp_url,"rtsp://unused/collision");
}
void setUp(void) {
    strcpy(directory,"/tmp/lightnvr-path-collision-XXXXXX");TEST_ASSERT_NOT_NULL(mkdtemp(directory));
    snprintf(planned,sizeof(planned),"%s/recording_20261031_042253.mp4",directory);
    calls=metadata_count=0;stop_after=2;fail_first=deny_reservation=stop_on_wait=fail_empty=false;
    memset(rows,0,sizeof(rows));memset(paths,0,sizeof(paths));initialize_writer();
}
void tearDown(void) {
    DIR *dir=opendir(directory);struct dirent *item;
    if(dir){while((item=readdir(dir))){if(item->d_name[0]=='.')continue;
      char path[MAX_PATH_LENGTH];snprintf(path,sizeof(path),"%s/%s",directory,item->d_name);unlink(path);}closedir(dir);}
    rmdir(directory);
}
static void assert_contents(const char *path,const char *expected) {
    char data[32]={0};FILE *file=fopen(path,"rb");TEST_ASSERT_NOT_NULL(file);
    size_t size=fread(data,1,sizeof(data)-1,file);fclose(file);
    TEST_ASSERT_EQUAL_UINT(strlen(expected),size);TEST_ASSERT_EQUAL_STRING(expected,data);
}
static void test_same_second_rotation_preserves_prior_file_and_metadata(void) {
    mp4_writer_rtsp_thread(&reader);
    TEST_ASSERT_EQUAL_INT(2,calls);TEST_ASSERT_NOT_EQUAL(0,strcmp(paths[0],paths[1]));
    assert_contents(paths[0],"segment-1");assert_contents(paths[1],"segment-2");
    TEST_ASSERT_EQUAL_INT(2,metadata_count);
    for(int n=0;n<2;n++) {
      TEST_ASSERT_EQUAL_STRING(paths[n],rows[n].file_path);
      TEST_ASSERT_EQUAL_STRING(paths[n]+strlen(directory)+1,rows[n].object_key);
    }
}
static void test_existing_completed_file_is_never_truncated(void) {
    FILE *file=fopen(planned,"wb");TEST_ASSERT_NOT_NULL(file);fputs("prior-footage",file);fclose(file);
    stop_after=1;mp4_writer_rtsp_thread(&reader);
    assert_contents(planned,"prior-footage");TEST_ASSERT_NOT_EQUAL(0,strcmp(paths[0],planned));
}
static void test_same_second_writer_restart_allocates_new_identity(void) {
    stop_after=1;mp4_writer_rtsp_thread(&reader);char first[MAX_PATH_LENGTH];strcpy(first,paths[0]);
    initialize_writer();stop_after=2;mp4_writer_rtsp_thread(&reader);
    TEST_ASSERT_NOT_EQUAL(0,strcmp(first,paths[1]));assert_contents(first,"segment-1");
}
static void test_partial_retry_preserves_old_metadata_identity(void) {
    fail_first=true;mp4_writer_rtsp_thread(&reader);
    TEST_ASSERT_EQUAL_INT(2,calls);TEST_ASSERT_NOT_EQUAL(0,strcmp(paths[0],paths[1]));
    assert_contents(paths[0],"segment-1");TEST_ASSERT_EQUAL_INT(2,metadata_count);
    TEST_ASSERT_EQUAL_STRING(paths[0],rows[0].file_path);
    TEST_ASSERT_EQUAL_STRING(paths[1],rows[1].file_path);
}
typedef struct {char path[MAX_PATH_LENGTH]; int descriptor;} allocation_t;
static void *reserve_concurrently(void *opaque) {
    allocation_t *allocation=opaque;
    snprintf(allocation->path,sizeof(allocation->path),"%s",planned);
    allocation->descriptor=reserve_mp4_recording_path(allocation->path,sizeof(allocation->path),NULL);
    return NULL;
}
static void test_concurrent_attempts_claim_distinct_files_atomically(void) {
    allocation_t allocations[8];pthread_t threads[8];
    for(int n=0;n<8;n++) TEST_ASSERT_EQUAL_INT(0,pthread_create(&threads[n],NULL,reserve_concurrently,&allocations[n]));
    for(int n=0;n<8;n++) TEST_ASSERT_EQUAL_INT(0,pthread_join(threads[n],NULL));
    for(int n=0;n<8;n++) {
        TEST_ASSERT_GREATER_OR_EQUAL(0,allocations[n].descriptor);
        for(int previous=0;previous<n;previous++) TEST_ASSERT_NOT_EQUAL(0,strcmp(allocations[n].path,allocations[previous].path));
        TEST_ASSERT_EQUAL_INT(1,write(allocations[n].descriptor,"x",1));
        TEST_ASSERT_EQUAL_INT(0,release_mp4_recording_path(allocations[n].descriptor,allocations[n].path));
        assert_contents(allocations[n].path,"x");
    }
}
static void test_unused_empty_reservation_is_removed(void) {
    int fd=reserve_mp4_recording_path(writer.output_path,sizeof(writer.output_path),&writer.placement);
    TEST_ASSERT_GREATER_OR_EQUAL(0,fd);
    TEST_ASSERT_EQUAL_INT(0,release_mp4_recording_path(fd,writer.output_path));
    TEST_ASSERT_EQUAL_INT(-1,access(writer.output_path,F_OK));
    TEST_ASSERT_EQUAL_INT(EBADF,fcntl(fd,F_GETFD)==-1?errno:0);
}
static void test_cleanup_preserves_replacement_inode(void) {
    int fd=reserve_mp4_recording_path(writer.output_path,sizeof(writer.output_path),&writer.placement);
    TEST_ASSERT_GREATER_OR_EQUAL(0,fd);TEST_ASSERT_EQUAL_INT(0,unlink(writer.output_path));
    FILE *replacement=fopen(writer.output_path,"wb");TEST_ASSERT_NOT_NULL(replacement);
    fputs("replacement",replacement);fclose(replacement);
    TEST_ASSERT_EQUAL_INT(0,release_mp4_recording_path(fd,writer.output_path));
    assert_contents(writer.output_path,"replacement");
}
static void test_short_buffer_preserves_prior_file_and_caller_state(void) {
    FILE *file=fopen(planned,"wb");TEST_ASSERT_NOT_NULL(file);fputs("prior",file);fclose(file);
    storage_placement_t original=writer.placement;
    TEST_ASSERT_EQUAL_INT(-1,reserve_mp4_recording_path(writer.output_path,strlen(planned)+1,&writer.placement));
    TEST_ASSERT_EQUAL_INT(ENAMETOOLONG,errno);
    TEST_ASSERT_EQUAL_STRING(planned,writer.output_path);
    TEST_ASSERT_EQUAL_MEMORY(&original,&writer.placement,sizeof(original));
    assert_contents(planned,"prior");
}
static void test_failed_segment_cleans_its_empty_reservation(void) {
    fail_empty=true;mp4_writer_rtsp_thread(&reader);
    TEST_ASSERT_EQUAL_INT(1,calls);TEST_ASSERT_EQUAL_INT(0,metadata_count);
    TEST_ASSERT_EQUAL_INT(-1,access(planned,F_OK));
}
static void test_reservation_error_never_calls_muxer_or_creates_metadata(void) {
    deny_reservation=stop_on_wait=true;mp4_writer_rtsp_thread(&reader);
    TEST_ASSERT_EQUAL_INT(0,calls);TEST_ASSERT_EQUAL_INT(0,metadata_count);
    TEST_ASSERT_EQUAL_INT(-1,access(planned,F_OK));
}
static void test_zero_duration_retry_uses_default_rotation_before_new_file(void) {
    writer.segment_duration=0;fail_first=true;mp4_writer_rtsp_thread(&reader);
    TEST_ASSERT_EQUAL_INT(2,metadata_count);
    TEST_ASSERT_NOT_EQUAL(0,strcmp(rows[0].file_path,rows[1].file_path));
}
static void test_negative_duration_retry_uses_default_rotation_before_new_file(void) {
    writer.segment_duration=-1;fail_first=true;mp4_writer_rtsp_thread(&reader);
    TEST_ASSERT_EQUAL_INT(2,metadata_count);
    TEST_ASSERT_NOT_EQUAL(0,strcmp(rows[0].file_path,rows[1].file_path));
}
int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_same_second_rotation_preserves_prior_file_and_metadata);
    RUN_TEST(test_existing_completed_file_is_never_truncated);
    RUN_TEST(test_same_second_writer_restart_allocates_new_identity);
    RUN_TEST(test_partial_retry_preserves_old_metadata_identity);
    RUN_TEST(test_concurrent_attempts_claim_distinct_files_atomically);
    RUN_TEST(test_unused_empty_reservation_is_removed);
    RUN_TEST(test_cleanup_preserves_replacement_inode);
    RUN_TEST(test_short_buffer_preserves_prior_file_and_caller_state);
    RUN_TEST(test_reservation_error_never_calls_muxer_or_creates_metadata);
    RUN_TEST(test_failed_segment_cleans_its_empty_reservation);
    RUN_TEST(test_zero_duration_retry_uses_default_rotation_before_new_file);
    RUN_TEST(test_negative_duration_retry_uses_default_rotation_before_new_file);
    return UNITY_END();
}
