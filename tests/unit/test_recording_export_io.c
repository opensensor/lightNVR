/* SPDX-License-Identifier: GPL-3.0-or-later */
#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <errno.h>
#include <unistd.h>
#include <sys/stat.h>
#include "unity.h"
#include "database/db_recordings.h"
#include "storage/storage_source.h"

/* Faults affect only stdio calls made by this translation unit's production
 * worker, never global libc or another test process. Real temporary files are
 * still opened, written and closed. DB/source selection is a bounded fixture. */
static char source_path[256], archive_path[256];
static FILE *source_file, *archive_file;
static int writes, fail_write, source_opens, seeks;
static bool read_failure, short_read, source_close_failure, archive_close_failure;
static bool short_write, seek_failure, changed_source, oversized_source;
static int source_stats, write_errno = ENOSPC;
static size_t fault_fwrite(const void *ptr, size_t size, size_t count, FILE *f) {
    if (f == archive_file && ++writes == fail_write) {
        errno = write_errno;
        return short_write && count > 1 ? fwrite(ptr, size, count - 1, f) : 0;
    }
    return fwrite(ptr, size, count, f);
}
static FILE *fault_fopen(const char *path, const char *mode) {
    FILE *f = fopen(path, mode);
    if (strcmp(path, source_path) == 0) { source_file = f; ++source_opens; }
    return f;
}
static FILE *fault_fdopen(int fd, const char *mode) {
    archive_file = fdopen(fd, mode);
    return archive_file;
}
static int fault_mkstemp(char *path) {
    int fd = mkstemp(path);
    snprintf(archive_path, sizeof(archive_path), "%s", path);
    return fd;
}
static size_t fault_fread(void *ptr, size_t size, size_t count, FILE *f) {
    if (f == source_file && (read_failure || short_read)) {
        errno = EIO;
        return 0;
    }
    return fread(ptr, size, count, f);
}
static int fault_ferror(FILE *f) {
    return (f == source_file && read_failure) || ferror(f);
}
static int fault_fclose(FILE *f) {
    bool fail = (f == source_file && source_close_failure) ||
                (f == archive_file && archive_close_failure);
    if (f == source_file) source_file = NULL;
    if (f == archive_file) archive_file = NULL;
    int result = fclose(f);
    if (fail) { errno = EIO; return EOF; }
    return result;
}
static int fault_fseeko(FILE *f, off_t offset, int whence) {
    if (f == archive_file && ++seeks == 1 && seek_failure) {
        errno = EIO;
        return -1;
    }
    return fseeko(f, offset, whence);
}
static int fault_fstat(int fd, struct stat *st) {
    int result = fstat(fd, st);
    if (result == 0 && source_file && fd == fileno(source_file)) {
        ++source_stats;
        if (oversized_source) st->st_size = (off_t)UINT32_MAX;
        if (changed_source && source_stats == 2) ++st->st_mtim.tv_sec;
    }
    return result;
}
#define fwrite fault_fwrite
#define fopen fault_fopen
#define fdopen fault_fdopen
#define mkstemp fault_mkstemp
#define fread fault_fread
#define ferror fault_ferror
#define fclose fault_fclose
#define fseeko fault_fseeko
#define fstat fault_fstat
#include "../../src/web/api_handlers_recordings_batch_download.c"
#undef fwrite
#undef fopen
#undef fdopen
#undef mkstemp
#undef fread
#undef ferror
#undef fclose
#undef fseeko
#undef fstat

int __wrap_get_recording_metadata_by_id(uint64_t id, recording_metadata_t *rec) {
    memset(rec, 0, sizeof(*rec));
    rec->id = id;
    rec->start_time = 1700000000;
    rec->protected = true;
    snprintf(rec->file_path, sizeof(rec->file_path), "%s", source_path);
    return 0;
}
int __wrap_storage_source_resolve(uint64_t id, char path[MAX_PATH_LENGTH], char error[256]) {
    (void)id; (void)path; (void)error;
    return STORAGE_SOURCE_READY;
}

void setUp(void) {
    strcpy(source_path, "/tmp/lightnvr_export_source_XXXXXX");
    int fd = mkstemp(source_path);
    TEST_ASSERT_GREATER_OR_EQUAL(0, fd);
    TEST_ASSERT_EQUAL_INT(9, write(fd, "123456789", 9));
    close(fd);
    archive_path[0] = 0;
    source_file = archive_file = NULL;
    writes = fail_write = source_opens = seeks = 0;
    read_failure = short_read = source_close_failure = archive_close_failure = false;
    short_write = seek_failure = changed_source = oversized_source = false;
    source_stats = 0;
    write_errno = ENOSPC;
    memset(s_dl_jobs, 0, sizeof(s_dl_jobs));
}
void tearDown(void) {
    if (archive_path[0]) unlink(archive_path);
    unlink(source_path);
}
static void run_worker(bool success) {
    source_stats = 0;
    memset(s_dl_jobs, 0, sizeof(s_dl_jobs));
    s_dl_jobs[0].is_active = true;
    strcpy(s_dl_jobs[0].job_id, "export-io-test");
    s_dl_jobs[0].id_count = s_dl_jobs[0].total = 1;
    s_dl_jobs[0].ids[0] = 1;
    dl_thread_arg_t *arg = calloc(1, sizeof(*arg));
    TEST_ASSERT_NOT_NULL(arg);
    strcpy(arg->job_id, s_dl_jobs[0].job_id);
    zip_worker(arg);
    TEST_ASSERT_EQUAL_INT(success ? DL_COMPLETE : DL_ERROR, s_dl_jobs[0].status);
    TEST_ASSERT_EQUAL_INT(success, access(archive_path, F_OK) == 0);
    TEST_ASSERT_EQUAL_INT(success, s_dl_jobs[0].tmp_path[0] != 0);
    TEST_ASSERT_NULL(source_file);
    TEST_ASSERT_NULL(archive_file);
    char data[16] = {0};
    FILE *source = fopen(source_path, "rb");
    TEST_ASSERT_NOT_NULL(source);
    TEST_ASSERT_EQUAL_UINT(9, fread(data, 1, sizeof(data), source));
    TEST_ASSERT_EQUAL_STRING("123456789", data);
    fclose(source);
}
static uint32_t le32(const unsigned char *bytes) {
    return bytes[0] | (uint32_t)bytes[1] << 8 | (uint32_t)bytes[2] << 16 | (uint32_t)bytes[3] << 24;
}
void test_success_uses_one_source_open_and_patches_crc(void) {
    run_worker(true);
    TEST_ASSERT_EQUAL_INT(1, source_opens);
    unsigned char header[30];
    FILE *zip = fopen(archive_path, "rb");
    TEST_ASSERT_EQUAL_UINT(sizeof(header), fread(header, 1, sizeof(header), zip));
    fclose(zip);
    TEST_ASSERT_EQUAL_HEX32(0xcbf43926, le32(header + 14));
    TEST_ASSERT_EQUAL_UINT32(9, le32(header + 18));
    TEST_ASSERT_EQUAL_UINT32(9, le32(header + 22));
}
void test_every_archive_write_failure_prevents_publication(void) {
    run_worker(true);
    int count = writes;
    unlink(archive_path);
    for (int short_result = 0; short_result <= 1; ++short_result) {
        for (int call = 1; call <= count; ++call) {
            writes = 0;
            fail_write = call;
            short_write = short_result;
            run_worker(false);
        }
    }
}
void test_eio_prevents_publication(void) { write_errno = EIO; fail_write = 1; run_worker(false); }
void test_changed_source_prevents_publication(void) { changed_source = true; run_worker(false); }
void test_zip32_overflow_prevents_publication(void) { oversized_source = true; run_worker(false); }
void test_empty_source_is_valid(void) {
    TEST_ASSERT_EQUAL_INT(0, truncate(source_path, 0));
    FILE *zip = tmpfile();
    TEST_ASSERT_NOT_NULL(zip);
    TEST_ASSERT_EQUAL_INT(0, zip_write_local_header(zip, "empty", 0, 0));
    uint32_t size = 100, crc = 100;
    TEST_ASSERT_EQUAL_INT(0, zip_copy_file(zip, source_path, 0, 100, &size, &crc));
    TEST_ASSERT_EQUAL_UINT32(0, size);
    TEST_ASSERT_EQUAL_UINT32(0, crc);
    fclose(zip);
}
void test_source_read_error_prevents_publication(void) { read_failure = true; run_worker(false); }
void test_source_truncation_prevents_publication(void) { short_read = true; run_worker(false); }
void test_source_close_error_prevents_publication(void) { source_close_failure = true; run_worker(false); }
void test_archive_close_error_prevents_publication(void) { archive_close_failure = true; run_worker(false); }
void test_archive_seek_error_prevents_publication(void) { seek_failure = true; run_worker(false); }
int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_eio_prevents_publication);
    RUN_TEST(test_changed_source_prevents_publication);
    RUN_TEST(test_zip32_overflow_prevents_publication);
    RUN_TEST(test_empty_source_is_valid);
    RUN_TEST(test_success_uses_one_source_open_and_patches_crc);
    RUN_TEST(test_every_archive_write_failure_prevents_publication);
    RUN_TEST(test_source_read_error_prevents_publication);
    RUN_TEST(test_source_truncation_prevents_publication);
    RUN_TEST(test_source_close_error_prevents_publication);
    RUN_TEST(test_archive_close_error_prevents_publication);
    RUN_TEST(test_archive_seek_error_prevents_publication);
    return UNITY_END();
}
