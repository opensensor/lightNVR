/**
 * @file api_handlers_recordings_batch_download.c
 * @brief Handler for batch downloading recordings as a ZIP archive
 *
 * Endpoints:
 *   POST /api/recordings/batch-download        – create ZIP job, return token
 *   GET  /api/recordings/batch-download/status/#  – poll job status
 *   GET  /api/recordings/batch-download/result/#  – download the ZIP when ready
 */

#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <time.h>
#include <unistd.h>
#include <errno.h>
#include <pthread.h>
#include <sys/stat.h>
#include <sys/random.h>
#include <cjson/cJSON.h>

#include "web/request_response.h"
#include "web/httpd_utils.h"
#include "core/logger.h"
#include "core/config.h"
#include "database/db_recordings.h"
#include "database/db_auth.h"

/* ─── ZIP primitives ─────────────────────────────────────────────────── */

static uint32_t s_crc_table[256];
static int s_crc_init = 0;

static void init_crc32(void) {
    for (uint32_t i = 0; i < 256; i++) {
        uint32_t c = i;
        for (int j = 0; j < 8; j++)
            c = (c & 1) ? (0xEDB88320u ^ (c >> 1)) : (c >> 1);
        s_crc_table[i] = c;
    }
    s_crc_init = 1;
}

static uint32_t crc32_of_file(const char *path, uint64_t *size_out) {
    if (!s_crc_init) init_crc32();
    FILE *f = fopen(path, "rb");
    if (!f) { if (size_out) *size_out = 0; return 0; }
    uint32_t crc = 0xFFFFFFFFu;
    uint64_t total = 0;
    uint8_t buf[65536];
    size_t n;
    while ((n = fread(buf, 1, sizeof(buf), f)) > 0) {
        for (size_t i = 0; i < n; i++)
            crc = (crc >> 8) ^ s_crc_table[(crc ^ buf[i]) & 0xFF];
        total += n;
    }
    fclose(f);
    if (size_out) *size_out = total;
    return ~crc;
}

static void zip_write_u16(FILE *f, uint16_t v) {
    fputc(v & 0xFF, f);
    fputc((v >> 8) & 0xFF, f);
}
static void zip_write_u32(FILE *f, uint32_t v) {
    fputc((int)(v & 0xFFU), f); fputc((int)((v >> 8) & 0xFFU), f);
    fputc((int)((v >> 16) & 0xFFU), f); fputc((int)((v >> 24) & 0xFFU), f);
}

/* Write local file header; returns offset before writing */
static uint32_t zip_write_local_header(FILE *zip, const char *name,
                                        uint32_t crc, uint32_t size,
                                        uint16_t dos_date, uint16_t dos_time) {
    uint16_t namelen = (uint16_t)strlen(name);
    zip_write_u32(zip, 0x04034b50u);  /* signature */
    zip_write_u16(zip, 20);           /* version needed */
    zip_write_u16(zip, 0);            /* flags */
    zip_write_u16(zip, 0);            /* compression: STORED */
    zip_write_u16(zip, dos_time);
    zip_write_u16(zip, dos_date);
    zip_write_u32(zip, crc);
    zip_write_u32(zip, size);         /* compressed size */
    zip_write_u32(zip, size);         /* uncompressed size */
    zip_write_u16(zip, namelen);
    zip_write_u16(zip, 0);            /* extra field length */
    fwrite(name, 1, namelen, zip);
    return 30 + namelen;
}



/* Write central directory entry */
static void zip_write_central_entry(FILE *zip, const char *name,
                                     uint32_t crc, uint32_t size,
                                     uint16_t dos_date, uint16_t dos_time,
                                     uint32_t local_offset) {
    uint16_t namelen = (uint16_t)strlen(name);
    zip_write_u32(zip, 0x02014b50u);
    zip_write_u16(zip, 20); zip_write_u16(zip, 20);
    zip_write_u16(zip, 0);  zip_write_u16(zip, 0);
    zip_write_u16(zip, dos_time); zip_write_u16(zip, dos_date);
    zip_write_u32(zip, crc); zip_write_u32(zip, size); zip_write_u32(zip, size);
    zip_write_u16(zip, namelen); zip_write_u16(zip, 0); zip_write_u16(zip, 0);
    zip_write_u16(zip, 0); zip_write_u16(zip, 0); zip_write_u32(zip, 0);
    zip_write_u32(zip, local_offset);
    fwrite(name, 1, namelen, zip);
}

static void zip_write_eocd(FILE *zip, uint16_t entry_count,
                            uint32_t cd_size, uint32_t cd_offset) {
    zip_write_u32(zip, 0x06054b50u);
    zip_write_u16(zip, 0); zip_write_u16(zip, 0);
    zip_write_u16(zip, entry_count); zip_write_u16(zip, entry_count);
    zip_write_u32(zip, cd_size); zip_write_u32(zip, cd_offset);
    zip_write_u16(zip, 0);
}

static uint64_t zip_copy_file(FILE *zip, const char *path) {
    FILE *src = fopen(path, "rb");
    if (!src) return 0;
    uint8_t buf[65536];
    size_t n; uint64_t total = 0;
    while ((n = fread(buf, 1, sizeof(buf), src)) > 0) {
        fwrite(buf, 1, n, zip); total += n;
    }
    fclose(src);
    return total;
}

/* ─── Job tracking ────────────────────────────────────────────────────── */

#define MAX_BATCH_DL_JOBS   8
#define MAX_DL_IDS         200
#define JOB_DL_RETENTION   600

typedef enum { DL_PENDING=0, DL_RUNNING, DL_COMPLETE, DL_ERROR } dl_status_t;

typedef struct {
    char        job_id[64];
    char        zip_filename[256];
    char        tmp_path[256];
    uint64_t    ids[MAX_DL_IDS];
    int         id_count;
    dl_status_t status;
    int         current;
    int         total;
    char        error[256];
    time_t      created_at;
    time_t      updated_at;
    bool        is_active;
} batch_dl_job_t;

static batch_dl_job_t  s_dl_jobs[MAX_BATCH_DL_JOBS];
static pthread_mutex_t s_dl_mutex = PTHREAD_MUTEX_INITIALIZER;
static bool            s_dl_inited = false;

static void dl_jobs_init(void) {
    if (s_dl_inited) return;
    memset(s_dl_jobs, 0, sizeof(s_dl_jobs));
    s_dl_inited = true;
}

static void gen_uuid(char *out) {
    uint8_t b[16];
    if (getrandom(b, 16, 0) < 0) {
        FILE *f = fopen("/dev/urandom", "rb");
        if (f) { (void)fread(b, 1, 16, f); fclose(f); }
    }
    b[6]=(b[6]&0x0F)|0x40; b[8]=(b[8]&0x3F)|0x80;
    snprintf(out,37,
        "%02x%02x%02x%02x-%02x%02x-%02x%02x-%02x%02x-%02x%02x%02x%02x%02x%02x",
        b[0],b[1],b[2],b[3],b[4],b[5],b[6],b[7],
        b[8],b[9],b[10],b[11],b[12],b[13],b[14],b[15]);
}

static int find_dl_slot(void) {
    time_t now = time(NULL);
    /* Prefer genuinely empty slots first */
    for (int i=0;i<MAX_BATCH_DL_JOBS;i++) if (!s_dl_jobs[i].is_active) return i;
    /* Reuse old completed/error slots; clean up their temp file first */
    for (int i=0;i<MAX_BATCH_DL_JOBS;i++)
        if ((s_dl_jobs[i].status==DL_COMPLETE||s_dl_jobs[i].status==DL_ERROR)
            && (now-s_dl_jobs[i].updated_at)>JOB_DL_RETENTION) {
            if (s_dl_jobs[i].tmp_path[0]) {
                unlink(s_dl_jobs[i].tmp_path);
                s_dl_jobs[i].tmp_path[0] = '\0';
            }
            return i;
        }
    return -1;
}

static int find_dl_job(const char *jid) {
    for (int i=0;i<MAX_BATCH_DL_JOBS;i++)
        if (s_dl_jobs[i].is_active && strcmp(s_dl_jobs[i].job_id,jid)==0) return i;
    return -1;
}


/* ─── Worker thread ───────────────────────────────────────────────────── */

typedef struct {
    char job_id[64];
} dl_thread_arg_t;

/* DOS time helpers */
static uint16_t to_dos_time(const struct tm *t) {
    return (uint16_t)(((t->tm_hour & 0x1F) << 11) | ((t->tm_min & 0x3F) << 5) | ((t->tm_sec/2) & 0x1F));
}
static uint16_t to_dos_date(const struct tm *t) {
    return (uint16_t)((((t->tm_year - 80) & 0x7F) << 9) | (((t->tm_mon+1) & 0x0F) << 5) | (t->tm_mday & 0x1F));
}

static void *zip_worker(void *arg) {
    dl_thread_arg_t *a = (dl_thread_arg_t *)arg;
    char job_id[64];
    strncpy(job_id, a->job_id, sizeof(job_id)-1);
    free(a);

    /* Grab job info */
    pthread_mutex_lock(&s_dl_mutex);
    int slot = find_dl_job(job_id);
    if (slot < 0) { pthread_mutex_unlock(&s_dl_mutex); return NULL; }
    batch_dl_job_t job = s_dl_jobs[slot]; /* local copy */
    s_dl_jobs[slot].status = DL_RUNNING;
    pthread_mutex_unlock(&s_dl_mutex);

    /* Open temp file */
    char tmp_template[] = "/tmp/lightnvr_zip_XXXXXX";
    int fd = mkstemp(tmp_template);
    if (fd < 0) {
        log_error("zip_worker: mkstemp failed: %s", strerror(errno));
        pthread_mutex_lock(&s_dl_mutex);
        slot = find_dl_job(job_id);
        if (slot>=0) { s_dl_jobs[slot].status=DL_ERROR; strncpy(s_dl_jobs[slot].error,"Failed to create temp file",255); s_dl_jobs[slot].updated_at=time(NULL); }
        pthread_mutex_unlock(&s_dl_mutex);
        return NULL;
    }
    FILE *zip = fdopen(fd, "wb");
    if (!zip) { close(fd); return NULL; }

    /* Per-entry info for central directory */
    typedef struct { char name[128]; uint32_t crc; uint32_t size; uint32_t offset; uint16_t dos_date; uint16_t dos_time; } entry_t;
    entry_t *entries = calloc(job.id_count, sizeof(entry_t));
    int entry_count = 0;
    uint32_t data_offset = 0;

    for (int i = 0; i < job.id_count; i++) {
        recording_metadata_t rec = {0};
        if (get_recording_metadata_by_id(job.ids[i], &rec) != 0) {
            log_warn("zip_worker: recording %llu not found, skipping", (unsigned long long)job.ids[i]);
            goto next;
        }
        struct stat st;
        if (stat(rec.file_path, &st) != 0) { log_warn("zip_worker: file missing: %s", rec.file_path); goto next; }

        /* Build entry name: stream_YYYY-MM-DDTHH-mm-ss.ext */
        const char *base = strrchr(rec.file_path, '/');
        base = base ? base+1 : rec.file_path;
        snprintf(entries[entry_count].name, sizeof(entries[entry_count].name), "%s", base);

        uint64_t fsize = 0;
        uint32_t crc   = crc32_of_file(rec.file_path, &fsize);
        entries[entry_count].crc  = crc;
        entries[entry_count].size = (uint32_t)(fsize & 0xFFFFFFFFu);
        entries[entry_count].offset = data_offset;

        struct tm tm_info; localtime_r(&rec.start_time, &tm_info);
        entries[entry_count].dos_date = to_dos_date(&tm_info);
        entries[entry_count].dos_time = to_dos_time(&tm_info);

        uint32_t hdr_size = zip_write_local_header(zip,
            entries[entry_count].name, crc, entries[entry_count].size,
            entries[entry_count].dos_date, entries[entry_count].dos_time);
        data_offset += hdr_size;
        uint64_t written = zip_copy_file(zip, rec.file_path);
        data_offset += (uint32_t)written;
        entry_count++;

        next:
        pthread_mutex_lock(&s_dl_mutex);
        slot = find_dl_job(job_id);
        if (slot>=0) { s_dl_jobs[slot].current=i+1; s_dl_jobs[slot].updated_at=time(NULL); }
        pthread_mutex_unlock(&s_dl_mutex);
    }

    /* Write central directory */
    uint32_t cd_start = data_offset;
    uint32_t cd_size  = 0;
    long pos_before = ftell(zip);
    for (int i = 0; i < entry_count; i++) {
        uint16_t namelen = (uint16_t)strlen(entries[i].name);
        zip_write_central_entry(zip, entries[i].name, entries[i].crc, entries[i].size,
                                entries[i].dos_date, entries[i].dos_time, entries[i].offset);
        cd_size += 46 + namelen;
    }
    (void)pos_before;
    zip_write_eocd(zip, (uint16_t)entry_count, cd_size, cd_start);
    fclose(zip);
    free(entries);

    /* Store temp path and mark complete */
    pthread_mutex_lock(&s_dl_mutex);
    slot = find_dl_job(job_id);
    if (slot>=0) {
        strncpy(s_dl_jobs[slot].tmp_path, tmp_template, sizeof(s_dl_jobs[slot].tmp_path)-1);
        s_dl_jobs[slot].status     = DL_COMPLETE;
        s_dl_jobs[slot].current    = job.id_count;
        s_dl_jobs[slot].updated_at = time(NULL);
    }
    pthread_mutex_unlock(&s_dl_mutex);
    log_info("zip_worker: completed job %s -> %s (%d entries)", job_id, tmp_template, entry_count);
    return NULL;
}


/* ─── HTTP Handlers ───────────────────────────────────────────────────── */

/**
 * POST /api/recordings/batch-download
 * Body: { "ids": [1,2,3], "filename": "incident.zip" }
 * Returns: { "token": "uuid", "total": N }
 */
void handle_batch_download_recordings(const http_request_t *req, http_response_t *res) {
    /* Auth check (viewer and above can download) */
    if (g_config.web_auth_enabled) {
        user_t user;
        if (!httpd_check_viewer_access(req, &user)) {
            http_response_set_json_error(res, 401, "Unauthorized");
            return;
        }
    }

    cJSON *json = httpd_parse_json_body(req);
    if (!json) { http_response_set_json_error(res, 400, "Invalid JSON body"); return; }

    cJSON *ids_arr = cJSON_GetObjectItem(json, "ids");
    if (!ids_arr || !cJSON_IsArray(ids_arr)) {
        cJSON_Delete(json);
        http_response_set_json_error(res, 400, "Missing or invalid 'ids' array");
        return;
    }

    int count = cJSON_GetArraySize(ids_arr);
    if (count == 0 || count > MAX_DL_IDS) {
        cJSON_Delete(json);
        http_response_set_json_error(res, 400, "ids must contain 1-200 entries");
        return;
    }

    const char *filename_raw = "recordings.zip";
    cJSON *fn = cJSON_GetObjectItem(json, "filename");
    if (fn && cJSON_IsString(fn) && fn->valuestring[0]) filename_raw = fn->valuestring;

    pthread_mutex_lock(&s_dl_mutex);
    dl_jobs_init();
    int slot = find_dl_slot();
    if (slot < 0) {
        pthread_mutex_unlock(&s_dl_mutex);
        cJSON_Delete(json);
        http_response_set_json_error(res, 503, "Too many concurrent download jobs");
        return;
    }

    batch_dl_job_t *job = &s_dl_jobs[slot];
    memset(job, 0, sizeof(*job));
    gen_uuid(job->job_id);
    strncpy(job->zip_filename, filename_raw, sizeof(job->zip_filename)-1);
    job->id_count = count;
    job->total    = count;
    job->status   = DL_PENDING;
    job->created_at = job->updated_at = time(NULL);
    job->is_active  = true;

    for (int i = 0; i < count; i++) {
        cJSON *item = cJSON_GetArrayItem(ids_arr, i);
        job->ids[i] = (uint64_t)(cJSON_IsNumber(item) ? item->valuedouble : 0);
    }

    char job_id[64];
    strncpy(job_id, job->job_id, sizeof(job_id)-1);
    pthread_mutex_unlock(&s_dl_mutex);
    cJSON_Delete(json);

    /* Spawn worker thread */
    dl_thread_arg_t *targ = malloc(sizeof(dl_thread_arg_t));
    if (!targ) { http_response_set_json_error(res, 500, "Out of memory"); return; }
    strncpy(targ->job_id, job_id, sizeof(targ->job_id)-1);

    pthread_t tid;
    pthread_attr_t attr;
    pthread_attr_init(&attr);
    pthread_attr_setdetachstate(&attr, PTHREAD_CREATE_DETACHED);
    if (pthread_create(&tid, &attr, zip_worker, targ) != 0) {
        pthread_attr_destroy(&attr);
        free(targ);
        http_response_set_json_error(res, 500, "Failed to start ZIP worker");
        return;
    }
    pthread_attr_destroy(&attr);

    char body[256];
    snprintf(body, sizeof(body), "{\"token\":\"%s\",\"total\":%d}", job_id, count);
    http_response_set_json(res, 202, body);
    log_info("Batch download job started: %s (%d recordings)", job_id, count);
}

/**
 * GET /api/recordings/batch-download/status/{token}
 * Returns: { "status": "pending|running|complete|error", "current": N, "total": N, "error": "..." }
 */
void handle_batch_download_status(const http_request_t *req, http_response_t *res) {
    if (g_config.web_auth_enabled) {
        user_t user;
        if (!httpd_check_viewer_access(req, &user)) {
            http_response_set_json_error(res, 401, "Unauthorized");
            return;
        }
    }

    char token[64] = {0};
    if (http_request_extract_path_param(req, "/api/recordings/batch-download/status/", token, sizeof(token)) != 0) {
        http_response_set_json_error(res, 400, "Missing token"); return;
    }

    pthread_mutex_lock(&s_dl_mutex);
    int slot = find_dl_job(token);
    if (slot < 0) {
        pthread_mutex_unlock(&s_dl_mutex);
        http_response_set_json_error(res, 404, "Job not found");
        return;
    }
    batch_dl_job_t snap = s_dl_jobs[slot];
    pthread_mutex_unlock(&s_dl_mutex);

    const char *status_str = "pending";
    if (snap.status == DL_RUNNING)  status_str = "running";
    if (snap.status == DL_COMPLETE) status_str = "complete";
    if (snap.status == DL_ERROR)    status_str = "error";

    char body[512];
    snprintf(body, sizeof(body),
        "{\"status\":\"%s\",\"current\":%d,\"total\":%d,\"error\":\"%s\"}",
        status_str, snap.current, snap.total, snap.error);
    http_response_set_json(res, 200, body);
}

/**
 * GET /api/recordings/batch-download/result/{token}
 * Streams the ZIP file and deletes the temp file afterwards.
 */
void handle_batch_download_result(const http_request_t *req, http_response_t *res) {
    if (g_config.web_auth_enabled) {
        user_t user;
        if (!httpd_check_viewer_access(req, &user)) {
            http_response_set_json_error(res, 401, "Unauthorized");
            return;
        }
    }

    char token[64] = {0};
    if (http_request_extract_path_param(req, "/api/recordings/batch-download/result/", token, sizeof(token)) != 0) {
        http_response_set_json_error(res, 400, "Missing token"); return;
    }

    pthread_mutex_lock(&s_dl_mutex);
    int slot = find_dl_job(token);
    if (slot < 0) {
        pthread_mutex_unlock(&s_dl_mutex);
        http_response_set_json_error(res, 404, "Job not found");
        return;
    }
    if (s_dl_jobs[slot].status != DL_COMPLETE) {
        const char *st = (s_dl_jobs[slot].status == DL_ERROR) ? "error" : "not_ready";
        char err[64]; snprintf(err, sizeof(err), "Job is %s", st);
        pthread_mutex_unlock(&s_dl_mutex);
        http_response_set_json_error(res, 409, err);
        return;
    }
    char tmp_path[256], zip_filename[256];
    strncpy(tmp_path,     s_dl_jobs[slot].tmp_path,     sizeof(tmp_path)-1);
    strncpy(zip_filename, s_dl_jobs[slot].zip_filename, sizeof(zip_filename)-1);
    /*
     * Mark the slot as inactive so it can be reused, but do NOT unlink the
     * temp file here.  http_serve_file() uses libuv async I/O: it queues a
     * uv_fs_open() and returns immediately.  If we unlink before that open
     * completes the path no longer exists and the client gets a 404.
     * The temp file is cleaned up lazily when find_dl_slot() recycles this
     * slot after JOB_DL_RETENTION seconds.
     */
    s_dl_jobs[slot].is_active  = false;
    s_dl_jobs[slot].updated_at = time(NULL);   /* reset retention timer */
    pthread_mutex_unlock(&s_dl_mutex);

    /* Build Content-Disposition header */
    char disp[512];
    snprintf(disp, sizeof(disp), "Content-Disposition: attachment; filename=\"%s\"\r\n", zip_filename);

    if (http_serve_file(req, res, tmp_path, "application/zip", disp) != 0) {
        log_error("handle_batch_download_result: failed to serve %s", tmp_path);
        http_response_set_json_error(res, 500, "Failed to serve ZIP file");
        /* Safe to unlink here because http_serve_file failed before opening */
        unlink(tmp_path);
        return;
    }
    log_info("Serving batch download ZIP: %s -> %s", token, zip_filename);
}
