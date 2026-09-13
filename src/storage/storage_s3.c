#define _POSIX_C_SOURCE 200809L

#include "storage/storage_s3.h"

#if LIGHTNVR_ENABLE_S3
#include <cjson/cJSON.h>
#include <ctype.h>
#include <curl/curl.h>
#include <errno.h>
#include <fcntl.h>
#include <mbedtls/sha256.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <time.h>
#include <unistd.h>

#include "utils/strings.h"
#include "ezxml.h"
#include "database/db_core.h"
#include <strings.h>
#include "utils/uuid.h"

#define S3_REPLY_LIMIT (128U * 1024U)
#define S3_SECRET_LIMIT 8192

typedef struct {
    char access[256];
    char secret[256];
    char token[4096];
} s3_credentials_t;

typedef struct {
    char *body;
    size_t length;
    FILE *file;
    mbedtls_sha256_context *digest;
    uint64_t bytes;
    uint64_t limit;
    const storage_transfer_control_t *control;
    long status;
    char etag[192];
    char range[64];
    char content_range[128];
    unsigned char *data;
    bool anonymous;
} s3_io_t;

static void error_set(char *error, const char *message) {
    safe_strcpy(error, message, STORAGE_TARGET_ERROR_MAX, 0);
}

static void wipe(void *pointer, size_t length) {
    volatile unsigned char *bytes = pointer;
    while (length--) *bytes++ = 0;
}

static bool safe_identifier(const char *value, size_t maximum, bool dots) {
    size_t length = value ? strnlen(value, maximum) : 0;
    if (!length || length >= maximum) return false;
    for (size_t i = 0; i < length; i++) {
        unsigned char c = (unsigned char)value[i];
        if (!(isalnum(c) || c == '-' || c == '_' || (dots && c == '.'))) return false;
    }
    return true;
}

static bool safe_key(const char *key, bool empty) {
    if (!key || (!empty && !*key) || *key == '/' || strlen(key) >= MAX_PATH_LENGTH) return false;
    const char *segment = key;
    for (const char *p = key;; p++) {
        if (*p && ((unsigned char)*p < 32 || (unsigned char)*p == 127 || *p == '\\')) return false;
        if (*p == '/' || !*p) {
            size_t n = (size_t)(p - segment);
            if ((n == 1 && segment[0] == '.') || (n == 2 && !strncmp(segment, "..", 2))) return false;
            segment = p + 1;
        }
        if (!*p) break;
    }
    return true;
}

bool storage_s3_validate(const storage_target_t *target, char *error, size_t length) {
    const char *message = NULL;
    if (!target || !safe_identifier(target->bucket, sizeof(target->bucket), true) ||
        !safe_identifier(target->region, sizeof(target->region), false) ||
        !safe_identifier(target->credential_ref, sizeof(target->credential_ref), false)) {
        message = "S3 bucket, region and credential reference are required identifiers";
    } else {
        CURLU *url = curl_url();
        char *scheme = NULL, *host = NULL, *user = NULL, *query = NULL, *fragment = NULL, *path = NULL;
        bool valid = url && curl_url_set(url, CURLUPART_URL, target->endpoint, 0) == CURLUE_OK;
        if (valid) {
            curl_url_get(url, CURLUPART_SCHEME, &scheme, 0);
            curl_url_get(url, CURLUPART_HOST, &host, 0);
            curl_url_get(url, CURLUPART_USER, &user, 0);
            curl_url_get(url, CURLUPART_QUERY, &query, 0);
            curl_url_get(url, CURLUPART_FRAGMENT, &fragment, 0);
            curl_url_get(url, CURLUPART_PATH, &path, 0);
            const char *allow_http = getenv("LIGHTNVR_ARCHIVE_ALLOW_HTTP");
            valid = scheme && host && *host && !user && !query && !fragment &&
                (!path || !strcmp(path, "/")) &&
                (!strcmp(scheme, "https") ||
                 (!strcmp(scheme, "http") && allow_http && !strcmp(allow_http, "1")));
        }
        curl_free(scheme); curl_free(host); curl_free(user);
        curl_free(query); curl_free(fragment); curl_free(path);
        if (url) curl_url_cleanup(url);
        if (!valid) message = "S3 endpoint must be an HTTPS origin without credentials, query or path";
        char prefix[160];
        snprintf(prefix, sizeof(prefix), "s3://%s/", target->bucket);
        if (!message && (strncmp(target->root_path, prefix, strlen(prefix)) ||
                        !safe_key(target->root_path + strlen(prefix), true))) {
            message = "S3 root_path must be s3://bucket/managed-prefix without traversal";
        }
        if (!message && (target->is_default || target->mount_required))
            message = "Object storage is an archive target and cannot be the capture default or a mount";
    }
    if (message && error && length) safe_strcpy(error, message, length, 0);
    return message == NULL;
}

static int credentials_read(const storage_target_t *target, s3_credentials_t *credentials) {
    const char *directory = getenv("LIGHTNVR_ARCHIVE_CREDENTIALS_DIR");
    if (!directory) directory = "/etc/lightnvr/archive-credentials";
    if (!safe_identifier(target->credential_ref, sizeof(target->credential_ref), false)) return -1;
    int dir = open(directory, O_RDONLY | O_DIRECTORY | O_CLOEXEC | O_NOFOLLOW);
    if (dir < 0) return -1;
    int descriptor = openat(dir, target->credential_ref, O_RDONLY | O_CLOEXEC | O_NOFOLLOW);
    close(dir);
    if (descriptor < 0) return -1;
    struct stat status;
    char buffer[S3_SECRET_LIMIT + 1];
    bool valid = fstat(descriptor, &status) == 0 && S_ISREG(status.st_mode) &&
        !(status.st_mode & 0077) && (status.st_uid == 0 || status.st_uid == geteuid()) &&
        status.st_size > 0 && status.st_size <= S3_SECRET_LIMIT;
    ssize_t count = valid ? read(descriptor, buffer, sizeof(buffer) - 1) : -1;
    close(descriptor);
    if (count <= 0 || count != status.st_size) return -1;
    buffer[count] = 0;
    cJSON *json = cJSON_Parse(buffer);
    wipe(buffer, sizeof(buffer));
    const cJSON *access = cJSON_GetObjectItemCaseSensitive(json, "access_key_id");
    const cJSON *secret = cJSON_GetObjectItemCaseSensitive(json, "secret_access_key");
    const cJSON *token = cJSON_GetObjectItemCaseSensitive(json, "session_token");
    valid = cJSON_IsString(access) && cJSON_IsString(secret) &&
        access->valuestring[0] && secret->valuestring[0] &&
        strlen(access->valuestring) < sizeof(credentials->access) &&
        strlen(secret->valuestring) < sizeof(credentials->secret) &&
        (!token || (cJSON_IsString(token) && strlen(token->valuestring) < sizeof(credentials->token)));
    if (valid) {
        safe_strcpy(credentials->access, access->valuestring, sizeof(credentials->access), 0);
        safe_strcpy(credentials->secret, secret->valuestring, sizeof(credentials->secret), 0);
        if (token) safe_strcpy(credentials->token, token->valuestring, sizeof(credentials->token), 0);
        for (const char *p = credentials->token; *p; p++) if ((unsigned char)*p < 32 || *p == 127) valid = false;
    }
    if (cJSON_IsString(secret)) wipe(secret->valuestring, strlen(secret->valuestring));
    if (cJSON_IsString(token)) wipe(token->valuestring, strlen(token->valuestring));
    cJSON_Delete(json);
    return valid ? 0 : -1;
}

static size_t response_header(char *buffer, size_t size, size_t count, void *context) {
    s3_io_t *io = context;
    size_t length = size * count;
    if (length > 12 && !memcmp(buffer, "HTTP/", 5)) {
        const char *space = memchr(buffer, ' ', length);
        if (space && space + 4 <= buffer + length)
            io->status = (space[1] - '0') * 100 + (space[2] - '0') * 10 + space[3] - '0';
    }
    if (length > 5 && !strncasecmp(buffer, "ETag:", 5)) {
        size_t begin = 5, end = length;
        while (begin < end && isspace((unsigned char)buffer[begin])) begin++;
        while (end > begin && isspace((unsigned char)buffer[end-1])) end--;
        if (end - begin < sizeof(io->etag)) {
            memcpy(io->etag, buffer + begin, end - begin);
            io->etag[end - begin] = 0;
        }
    }
    if (length > 14 && !strncasecmp(buffer, "Content-Range:", 14)) {
        size_t begin = 14, end = length;
        while (begin < end && isspace((unsigned char)buffer[begin])) begin++;
        while (end > begin && isspace((unsigned char)buffer[end-1])) end--;
        if (end - begin < sizeof(io->content_range)) {
            memcpy(io->content_range, buffer + begin, end - begin);
            io->content_range[end - begin] = 0;
        }
    }
    return length;
}

static size_t receive_bytes(char *buffer, size_t size, size_t count, void *context) {
    s3_io_t *io = context;
    if (size && count > SIZE_MAX / size) return 0;
    size_t length = size * count;
    if (io->status < 200 || io->status >= 300) return length; /* never persist error documents as video */
    if (io->bytes > io->limit || length > io->limit - io->bytes) return 0;
    if (io->data) memcpy(io->data + io->bytes, buffer, length);
    if (io->file && fwrite(buffer, 1, length, io->file) != length) return 0;
    if (io->digest && mbedtls_sha256_update(io->digest, (unsigned char *)buffer, length) != 0) return 0;
    if (io->body) {
        if (io->length + length >= S3_REPLY_LIMIT) return 0;
        memcpy(io->body + io->length, buffer, length);
        io->length += length;
        io->body[io->length] = 0;
    }
    io->bytes += length;
    return length;
}

static int progress(void *context, curl_off_t dt, curl_off_t dn, curl_off_t ut, curl_off_t un) {
    (void)dt; (void)dn; (void)ut; (void)un;
    const storage_transfer_control_t *control = context;
    return control && control->cancelled && control->cancelled(control->context) ? 1 : 0;
}

static int request(const storage_target_t *target, const char *key, const char *query,
                   const char *method, FILE *upload, uint64_t upload_size,
                   const char *checksum, s3_io_t *io, uint64_t *object_size,
                   char error[STORAGE_TARGET_ERROR_MAX]) {
    if (!storage_s3_validate(target, error, STORAGE_TARGET_ERROR_MAX) ||
        (key && !safe_key(key, false))) return STORAGE_S3_ERROR;
    s3_credentials_t credentials = {0};
    if (!io->anonymous && credentials_read(target, &credentials)) {
        error_set(error, "Archive credential file is missing, invalid or has unsafe permissions");
        wipe(&credentials, sizeof(credentials));
        return STORAGE_S3_ERROR;
    }
    CURL *curl = curl_easy_init();
    char raw_key[2 * MAX_PATH_LENGTH];
    char root_prefix[160];
    snprintf(root_prefix, sizeof(root_prefix), "s3://%s/", target->bucket);
    const char *prefix = target->root_path + strlen(root_prefix);
    snprintf(raw_key, sizeof(raw_key), "%s%s%s", key ? prefix : "",
             key && *prefix && prefix[strlen(prefix) - 1] != '/' ? "/" : "", key ? key : "");
    char *encoded = curl ? curl_easy_escape(curl, raw_key, 0) : NULL;
    char url[4 * MAX_PATH_LENGTH + 512];
    char endpoint[MAX_PATH_LENGTH];
    safe_strcpy(endpoint, target->endpoint, sizeof(endpoint), 0);
    size_t endpoint_length = strlen(endpoint);
    while (endpoint_length && endpoint[endpoint_length - 1] == '/') endpoint[--endpoint_length] = 0;
    int length = snprintf(url, sizeof(url), "%s/%s%s%s%s%s", endpoint, target->bucket,
                         key ? "/" : "", encoded ? encoded : "", query ? "?" : "", query ? query : "");
    if (!curl || !encoded || length < 0 || (size_t)length >= sizeof(url)) {
        if (curl) curl_easy_cleanup(curl);
        curl_free(encoded);
        wipe(&credentials, sizeof(credentials));
        error_set(error, "Cannot construct archive request");
        return STORAGE_S3_ERROR;
    }
    char signature[128], hash_header[120], token_header[4200];
    snprintf(signature, sizeof(signature), "aws:amz:%s:s3", target->region);
    struct curl_slist *headers = NULL;
    headers = curl_slist_append(headers, "Expect:");
    if (checksum && *checksum) {
        snprintf(hash_header, sizeof(hash_header), "x-amz-content-sha256: %s", checksum);
        headers = curl_slist_append(headers, hash_header);
    }
    if (*credentials.token) {
        snprintf(token_header, sizeof(token_header), "x-amz-security-token: %s", credentials.token);
        headers = curl_slist_append(headers, token_header);
    }
    curl_easy_setopt(curl, CURLOPT_URL, url);
    if (!io->anonymous) {
        curl_easy_setopt(curl, CURLOPT_AWS_SIGV4, signature);
        curl_easy_setopt(curl, CURLOPT_USERNAME, credentials.access);
        curl_easy_setopt(curl, CURLOPT_PASSWORD, credentials.secret);
    }
    curl_easy_setopt(curl, CURLOPT_HTTPHEADER, headers);
    curl_easy_setopt(curl, CURLOPT_FOLLOWLOCATION, 0L);
    curl_easy_setopt(curl, CURLOPT_NOSIGNAL, 1L);
    curl_easy_setopt(curl, CURLOPT_CONNECTTIMEOUT, 10L);
    curl_easy_setopt(curl, CURLOPT_TIMEOUT, upload || io->digest || io->file ? 3600L : 20L);
    curl_easy_setopt(curl, CURLOPT_LOW_SPEED_LIMIT, 1024L);
    curl_easy_setopt(curl, CURLOPT_LOW_SPEED_TIME, 30L);
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, receive_bytes);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, io);
    curl_easy_setopt(curl, CURLOPT_HEADERFUNCTION, response_header);
    curl_easy_setopt(curl, CURLOPT_HEADERDATA, io);
    curl_easy_setopt(curl, CURLOPT_XFERINFOFUNCTION, progress);
    curl_easy_setopt(curl, CURLOPT_XFERINFODATA, io->control);
    curl_easy_setopt(curl, CURLOPT_NOPROGRESS, 0L);
    if (io->control && io->control->bandwidth_bps) {
        curl_easy_setopt(curl, CURLOPT_MAX_SEND_SPEED_LARGE, (curl_off_t)io->control->bandwidth_bps);
        curl_easy_setopt(curl, CURLOPT_MAX_RECV_SPEED_LARGE, (curl_off_t)io->control->bandwidth_bps);
    }
    if (upload) {
        curl_easy_setopt(curl, CURLOPT_UPLOAD, 1L);
        curl_easy_setopt(curl, CURLOPT_READDATA, upload);
        curl_easy_setopt(curl, CURLOPT_INFILESIZE_LARGE, (curl_off_t)upload_size);
    }
    if (!strcmp(method, "HEAD")) curl_easy_setopt(curl, CURLOPT_NOBODY, 1L);
    if (*io->range) curl_easy_setopt(curl, CURLOPT_RANGE, io->range);
    curl_easy_setopt(curl, CURLOPT_CUSTOMREQUEST, method);
    CURLcode code = curl_easy_perform(curl);
    long status = 0;
    curl_easy_getinfo(curl, CURLINFO_RESPONSE_CODE, &status);
    if (object_size && code == CURLE_OK && status == 200) {
        curl_off_t bytes = -1;
        curl_easy_getinfo(curl, CURLINFO_CONTENT_LENGTH_DOWNLOAD_T, &bytes);
        if (bytes < 0) code = CURLE_BAD_CONTENT_ENCODING;
        else *object_size = (uint64_t)bytes;
    }
    curl_easy_cleanup(curl);
    curl_slist_free_all(headers);
    curl_free(encoded);
    wipe(&credentials, sizeof(credentials));
    wipe(token_header, sizeof(token_header));
    if (code == CURLE_ABORTED_BY_CALLBACK) {
        error_set(error, "Archive operation cancelled");
        return STORAGE_S3_CANCELLED;
    }
    if (code != CURLE_OK) {
        snprintf(error, STORAGE_TARGET_ERROR_MAX, "Archive transport failed (%d)", (int)code);
        return STORAGE_S3_ERROR;
    }
    if (status == 404) return STORAGE_S3_MISSING;
    if (status < 200 || status >= 300) {
        snprintf(error, STORAGE_TARGET_ERROR_MAX, "Archive request failed (HTTP %ld)", status);
        return status == 409 || status == 412 ? STORAGE_S3_CONFLICT : STORAGE_S3_ERROR;
    }
    return STORAGE_S3_OK;
}

int storage_s3_read_range(const storage_target_t *target, const char *key,
                          uint64_t offset, size_t length, uint64_t total_size,
                          void *buffer, const storage_transfer_control_t *control,
                          char error[STORAGE_TARGET_ERROR_MAX]) {
    if (!length || offset >= total_size || length > total_size - offset || length > 1024U*1024U) return -1;
    s3_io_t io = {.data = buffer, .limit = length, .control = control};
    snprintf(io.range, sizeof(io.range), "%llu-%llu", (unsigned long long)offset,
             (unsigned long long)(offset + length - 1));
    int result = request(target, key, NULL, "GET", NULL, 0, NULL, &io, NULL, error);
    char expected[128];
    snprintf(expected, sizeof(expected), "bytes %llu-%llu/%llu", (unsigned long long)offset,
             (unsigned long long)(offset + length - 1), (unsigned long long)total_size);
    if (!result && (io.status != 206 || io.bytes != length || strcmp(io.content_range, expected))) {
        error_set(error, "Archive server returned an invalid byte range");
        result = -1;
    }
    return result;
}

int storage_s3_stat(const storage_target_t *target, const char *key, uint64_t *size,
                    char error[STORAGE_TARGET_ERROR_MAX]) {
    s3_io_t io = {.limit = S3_REPLY_LIMIT};
    return request(target, key, NULL, "HEAD", NULL, 0, NULL, &io, size, error);
}

#define S3_PART_BYTES (8ULL * 1024ULL * 1024ULL)

static int upload_checkpoint(const char *uuid, char id[1024], cJSON **parts, bool save) {
    sqlite3 *db = get_db_handle();
    pthread_mutex_t *mutex = get_db_mutex();
    if (!db || !mutex || !lightnvr_uuid_is_valid(uuid)) return -1;
    char *json = save ? cJSON_PrintUnformatted(*parts) : NULL;
    if (save && !json) return -1;
    pthread_mutex_lock(mutex);
    sqlite3_stmt *stmt = NULL;
    int rc = sqlite3_prepare_v2(db, save ?
        "UPDATE storage_migration_jobs SET upload_id=?2,upload_parts=?3,"
        "bytes_copied=MIN(bytes_total,COALESCE(json_array_length(?3,'$.parts')*json_extract(?3,'$.part_size'),0)),"
        "updated_at=strftime('%s','now') WHERE uuid=?1;" :
        "SELECT upload_id,upload_parts FROM storage_migration_jobs WHERE uuid=?1;", -1, &stmt, NULL);
    if (rc == SQLITE_OK) {
        sqlite3_bind_text(stmt, 1, uuid, -1, SQLITE_TRANSIENT);
        if (save) {
            sqlite3_bind_text(stmt, 2, id, -1, SQLITE_TRANSIENT);
            sqlite3_bind_text(stmt, 3, json, -1, SQLITE_TRANSIENT);
        }
        rc = sqlite3_step(stmt);
        if (!save && rc == SQLITE_ROW) {
            safe_strcpy(id, (const char *)sqlite3_column_text(stmt, 0), 1024, 0);
            *parts = cJSON_Parse((const char *)sqlite3_column_text(stmt, 1));
        }
    }
    if (stmt) sqlite3_finalize(stmt);
    int result = save ? (rc == SQLITE_DONE && sqlite3_changes(db) == 1 ? 0 : -1) : (rc == SQLITE_ROW ? 0 : -1);
    pthread_mutex_unlock(mutex);
    free(json);
    return result;
}

static char *upload_query(const char *id, int part) {
    char *escaped = curl_easy_escape(NULL, id, 0);
    if (!escaped) return NULL;
    size_t length = strlen(escaped) + 64;
    char *query = malloc(length);
    if (query) {
        if (part) snprintf(query, length, "partNumber=%d&uploadId=%s", part, escaped);
        else snprintf(query, length, "uploadId=%s", escaped);
    }
    curl_free(escaped);
    return query;
}

int storage_s3_abort_upload(const storage_target_t *target, const char *key,
                            const char *id, char error[STORAGE_TARGET_ERROR_MAX]) {
    if (!id || !*id) return 0;
    char *query = upload_query(id, 0);
    if (!query) return -1;
    s3_io_t io = {.limit = S3_REPLY_LIMIT};
    int result = request(target, key, query, "DELETE", NULL, 0, NULL, &io, NULL, error);
    free(query);
    return result == STORAGE_S3_MISSING ? 0 : result;
}

static ezxml_t response_xml(char *body, const char *name) {
    if (!body || strstr(body, "<!")) return NULL;
    ezxml_t root = ezxml_parse_str(body, strlen(body));
    if (!root) return NULL;
    const char *local = root->name ? strrchr(root->name, ':') : NULL;
    local = local ? local + 1 : root->name;
    if (*ezxml_error(root) || !local || strcmp(local, name)) { ezxml_free(root); return NULL; }
    return root;
}

static int file_section_hash(FILE *file, uint64_t offset, uint64_t bytes, char hash[65]) {
    if (fseeko(file, (off_t)offset, SEEK_SET)) return -1;
    mbedtls_sha256_context context;
    mbedtls_sha256_init(&context);
    int rc = mbedtls_sha256_starts(&context, 0);
    unsigned char buffer[32768], digest[32];
    while (!rc && bytes) {
        size_t count = bytes > sizeof(buffer) ? sizeof(buffer) : (size_t)bytes;
        if (fread(buffer, 1, count, file) != count) { rc = -1; break; }
        rc = mbedtls_sha256_update(&context, buffer, count);
        bytes -= count;
    }
    if (!rc) rc = mbedtls_sha256_finish(&context, digest);
    mbedtls_sha256_free(&context);
    if (!rc) for (int i = 0; i < 32; i++) snprintf(hash + i*2, 3, "%02x", digest[i]);
    return rc || fseeko(file, (off_t)offset, SEEK_SET) ? -1 : 0;
}

static int multipart_upload(const storage_target_t *target, const char *key, FILE *file,
                            uint64_t bytes, const char *checksum,
                            const storage_transfer_control_t *control, char error[256]) {
    char id[1024] = {0};
    cJSON *checkpoint = NULL;
    if (upload_checkpoint(control->job_uuid, id, &checkpoint, false)) return -1;
    cJSON *parts = cJSON_GetObjectItemCaseSensitive(checkpoint, "parts");
    const cJSON *hash = cJSON_GetObjectItemCaseSensitive(checkpoint, "checksum");
    const cJSON *part_size = cJSON_GetObjectItemCaseSensitive(checkpoint, "part_size");
    // Keep parts within S3's 10,000-part limit with a stable, journaled size.
    uint64_t chunk = ((bytes + 9999) / 10000 + S3_PART_BYTES - 1) / S3_PART_BYTES * S3_PART_BYTES;
    if (chunk < S3_PART_BYTES) chunk = S3_PART_BYTES;
    bool valid = cJSON_IsArray(parts) && cJSON_IsString(hash) && !strcmp(hash->valuestring, checksum) &&
                 cJSON_IsNumber(part_size) && part_size->valuedouble == (double)chunk &&
                 cJSON_GetArraySize(parts) <= (int)((bytes + chunk - 1) / chunk);
    int result = 0;
    if (!valid) {
        if (*id && storage_s3_abort_upload(target, key, id, error)) { cJSON_Delete(checkpoint); return -1; }
        id[0] = 0;
        cJSON_Delete(checkpoint);
        checkpoint = cJSON_CreateObject();
        if (!checkpoint) return -1;
        cJSON_AddStringToObject(checkpoint, "checksum", checksum);
        cJSON_AddNumberToObject(checkpoint, "part_size", (double)chunk);
        parts = cJSON_AddArrayToObject(checkpoint, "parts");
        if (!parts || upload_checkpoint(control->job_uuid, id, &checkpoint, true)) { cJSON_Delete(checkpoint); return -1; }
    }
    char *body = calloc(1, S3_REPLY_LIMIT);
    if (!body) { cJSON_Delete(checkpoint); return -1; }
    if (!*id) {
        cJSON_DeleteItemFromObjectCaseSensitive(checkpoint, "parts");
        parts = cJSON_AddArrayToObject(checkpoint, "parts");
        if (!parts) { free(body); cJSON_Delete(checkpoint); return -1; }
        s3_io_t io = {.body = body, .limit = S3_REPLY_LIMIT-1, .control = control};
        result = request(target, key, "uploads", "POST", NULL, 0, NULL, &io, NULL, error);
        ezxml_t root = result == 0 ? response_xml(body, "InitiateMultipartUploadResult") : NULL;
        const char *value = root ? ezxml_txt(ezxml_child(root, "UploadId")) : NULL;
        if (!value || !*value || strlen(value) >= sizeof(id)) result = -1;
        else safe_strcpy(id, value, sizeof(id), 0);
        if (root) ezxml_free(root);
        if (!result && upload_checkpoint(control->job_uuid, id, &checkpoint, true)) {
            // A journal failure cannot leave an upload running without ownership.
            storage_s3_abort_upload(target, key, id, error);
            result = -1;
        }
    }
    int completed = cJSON_GetArraySize(parts);
    for (uint64_t offset = (uint64_t)completed * chunk; !result && offset < bytes; offset += chunk) {
        uint64_t length = bytes - offset < chunk ? bytes - offset : chunk;
        char hash_hex[65];
        char *query = upload_query(id, completed + 1);
        if (!query || file_section_hash(file, offset, length, hash_hex)) { free(query); result = -1; break; }
        s3_io_t io = {.limit = S3_REPLY_LIMIT, .control = control};
        result = request(target, key, query, "PUT", file, length, hash_hex, &io, NULL, error);
        free(query);
        if (result == STORAGE_S3_MISSING) {
            id[0] = 0;
            cJSON_DeleteItemFromObjectCaseSensitive(checkpoint, "parts");
            cJSON_AddArrayToObject(checkpoint, "parts");
            upload_checkpoint(control->job_uuid, id, &checkpoint, true);
            result = STORAGE_S3_ERROR; // Expiry is retryable after clearing the stale upload.
            break;
        }
        if (result) break;
        if (!io.etag[0]) { result = -1; break; }
        // ETags are opaque. Only accept text safe to embed in the completion XML.
        for (const char *p = io.etag; *p; p++)
            if ((unsigned char)*p < 32 || strchr("<>&", *p)) result = -1;
        if (result) break;
        if (!cJSON_AddItemToArray(parts, cJSON_CreateString(io.etag))) { result = -1; break; }
        completed++;
        result = upload_checkpoint(control->job_uuid, id, &checkpoint, true);
    }
    if (!result) {
        FILE *manifest = tmpfile();
        if (!manifest) result = -1;
        else {
            fputs("<CompleteMultipartUpload>", manifest);
            for (int i = 0; i < completed; i++) {
                const cJSON *etag = cJSON_GetArrayItem(parts, i);
                if (!cJSON_IsString(etag)) { result = -1; break; }
                fprintf(manifest, "<Part><PartNumber>%d</PartNumber><ETag>%s</ETag></Part>", i+1, etag->valuestring);
            }
            fputs("</CompleteMultipartUpload>", manifest);
            off_t length = ftello(manifest);
            char hash_hex[65];
            char *query = upload_query(id, 0);
            memset(body, 0, S3_REPLY_LIMIT);
            s3_io_t io = {.body = body, .limit = S3_REPLY_LIMIT-1, .control = control};
            if (!result && query && length > 0 && !file_section_hash(manifest, 0, (uint64_t)length, hash_hex))
                result = request(target, key, query, "POST", manifest, (uint64_t)length, hash_hex, &io, NULL, error);
            else result = -1;
            // Completion may lose the upload, including an Error inside HTTP 200.
            bool missing_upload = result == STORAGE_S3_MISSING;
            char *failure_body = strdup(body);
            ezxml_t failure = failure_body ? response_xml(failure_body, "Error") : NULL;
            if (failure) {
                if (!strcmp(ezxml_txt(ezxml_child(failure, "Code")), "NoSuchUpload")) missing_upload = true;
                ezxml_free(failure);
            }
            free(failure_body);
            ezxml_t root = result == 0 ? response_xml(body, "CompleteMultipartUploadResult") : NULL;
            if (!root) result = -1;
            if (missing_upload) {
                id[0] = 0;
                cJSON_DeleteItemFromObjectCaseSensitive(checkpoint, "parts");
                cJSON_AddArrayToObject(checkpoint, "parts");
                upload_checkpoint(control->job_uuid, id, &checkpoint, true);
                result = -1; // Retry starts a new upload from the retained source.
            }
            if (root) ezxml_free(root);
            free(query);
            fclose(manifest);
        }
    }
    if (!result) {
        id[0] = 0;
        result = upload_checkpoint(control->job_uuid, id, &checkpoint, true);
    }
    free(body);
    cJSON_Delete(checkpoint);
    if (result && !*error) error_set(error, "Multipart archive upload could not complete; checkpoint retained for retry");
    return result;
}

int storage_s3_upload(const storage_target_t *target, const char *key, const char *path,
                      const char *checksum, const storage_transfer_control_t *control,
                      char error[STORAGE_TARGET_ERROR_MAX]) {
    int descriptor = open(path, O_RDONLY | O_CLOEXEC | O_NOFOLLOW);
    struct stat status;
    if (descriptor < 0 || fstat(descriptor, &status) || !S_ISREG(status.st_mode) || status.st_size < 0) {
        if (descriptor >= 0) close(descriptor);
        error_set(error, "Archive source is not a readable regular file");
        return STORAGE_S3_ERROR;
    }
    FILE *file = fdopen(descriptor, "rb");
    if (!file) { close(descriptor); return STORAGE_S3_ERROR; }
    s3_io_t io = {.limit = S3_REPLY_LIMIT, .control = control};
    int result = control && control->job_uuid && (uint64_t)status.st_size >= S3_PART_BYTES
        ? multipart_upload(target, key, file, (uint64_t)status.st_size, checksum, control, error)
        : request(target, key, NULL, "PUT", file, (uint64_t)status.st_size, checksum, &io, NULL, error);
    fclose(file);
    return result;
}

static int receive_object(const storage_target_t *target, const char *key, FILE *file,
                          uint64_t expected_size, const char *checksum,
                          const storage_transfer_control_t *control,
                          char error[STORAGE_TARGET_ERROR_MAX]) {
    mbedtls_sha256_context digest;
    mbedtls_sha256_init(&digest);
    if (mbedtls_sha256_starts(&digest, 0)) { mbedtls_sha256_free(&digest); return STORAGE_S3_ERROR; }
    s3_io_t io = {.file = file, .digest = &digest, .limit = expected_size, .control = control};
    int result = request(target, key, NULL, "GET", NULL, 0, NULL, &io, NULL, error);
    unsigned char hash[32];
    char hex[65];
    if (result == 0 && mbedtls_sha256_finish(&digest, hash)) result = STORAGE_S3_ERROR;
    mbedtls_sha256_free(&digest);
    if (result == 0) {
        for (int i = 0; i < 32; i++) snprintf(hex + i * 2, 3, "%02x", hash[i]);
        if (io.bytes != expected_size || (checksum && *checksum && strcmp(hex, checksum))) {
            error_set(error, "Archive content length or SHA-256 verification failed");
            result = STORAGE_S3_CONFLICT;
        }
    }
    return result;
}

int storage_s3_verify(const storage_target_t *target, const char *key, uint64_t size,
                      const char *checksum, const storage_transfer_control_t *control,
                      char error[STORAGE_TARGET_ERROR_MAX]) {
    return receive_object(target, key, NULL, size, checksum, control, error);
}

int storage_s3_download(const storage_target_t *target, const char *key, const char *path,
                        uint64_t expected_size, const char *expected_checksum,
                        const storage_transfer_control_t *control,
                        char error[STORAGE_TARGET_ERROR_MAX]) {
    int descriptor = open(path, O_WRONLY | O_CREAT | O_EXCL | O_CLOEXEC | O_NOFOLLOW, 0600);
    if (descriptor < 0) { error_set(error, "Cannot create archive staging file"); return STORAGE_S3_ERROR; }
    FILE *file = fdopen(descriptor, "wb");
    if (!file) { close(descriptor); unlink(path); return STORAGE_S3_ERROR; }
    int result = receive_object(target, key, file, expected_size, expected_checksum, control, error);
    if (result == 0 && (fflush(file) || fsync(fileno(file)))) result = STORAGE_S3_ERROR;
    if (fclose(file) && result == 0) result = STORAGE_S3_ERROR;
    if (result != 0) unlink(path);
    return result;
}

int storage_s3_delete(const storage_target_t *target, const char *key,
                      char error[STORAGE_TARGET_ERROR_MAX]) {
    s3_io_t io = {.limit = S3_REPLY_LIMIT};
    int result = request(target, key, NULL, "DELETE", NULL, 0, NULL, &io, NULL, error);
    if (result != 0 && result != STORAGE_S3_MISSING) return result;
    uint64_t size;
    result = storage_s3_stat(target, key, &size, error);
    if (result == STORAGE_S3_MISSING) return STORAGE_S3_OK;
    if (result == STORAGE_S3_OK) error_set(error, "Archive object still exists after deletion");
    return STORAGE_S3_ERROR;
}

/* Configuration checks fail closed, including unknown fields and namespaces.
 * Reject DTDs before parsing; object storage never needs entity expansion. */
static bool empty_configuration(char *body, size_t length, const char *expected) {
    if (strstr(body, "<!")) return false;
    ezxml_t root = ezxml_parse_str(body, length);
    bool valid = root && !*ezxml_error(root);
    if (valid) {
        const char *name = strrchr(root->name, ':');
        name = name ? name + 1 : root->name;
        valid = !strcmp(name, expected) && root->child == NULL;
        for (const char *p = root->txt; valid && p && *p; p++)
            if (!isspace((unsigned char)*p)) valid = false;
    }
    if (root) ezxml_free(root);
    return valid;
}

static const char *xml_local_name(ezxml_t node) {
    const char *colon = node && node->name ? strrchr(node->name, ':') : NULL;
    return colon ? colon + 1 : (node && node->name ? node->name : "");
}

static bool safe_lifecycle(char *body) {
    ezxml_t root = response_xml(body, "LifecycleConfiguration");
    if (!root) return false;
    bool safe = true;
    for (ezxml_t rule = root->child; rule && safe; rule = rule->ordered) {
        if (strcmp(xml_local_name(rule), "Rule")) { safe = false; break; }
        bool abort_rule = false;
        for (ezxml_t field = rule->child; field && safe; field = field->ordered) {
            const char *name = xml_local_name(field);
            if (!strcmp(name, "AbortIncompleteMultipartUpload")) {
                ezxml_t days = field->child;
                char *end = NULL;
                long value = days ? strtol(days->txt, &end, 10) : 0;
                abort_rule = days && !strcmp(xml_local_name(days), "DaysAfterInitiation") &&
                    !days->ordered && value >= 1 && value <= 7 && end && !*end;
                if (!abort_rule) safe = false;
            } else if (strcmp(name, "ID") && strcmp(name, "Filter") && strcmp(name, "Prefix") && strcmp(name, "Status")) safe = false;
        }
        if (!abort_rule) safe = false;
    }
    ezxml_free(root);
    return safe;
}

int storage_s3_probe(storage_target_t *target, bool write_test) {
    char error[STORAGE_TARGET_ERROR_MAX] = {0};
    char *body = calloc(1, S3_REPLY_LIMIT);
    if (!body) return STORAGE_S3_ERROR;
    s3_io_t io = {.body = body, .limit = S3_REPLY_LIMIT - 1};
    int result = request(target, NULL, "versioning", "GET", NULL, 0, NULL, &io, NULL, error);
    if (result == 0 && !empty_configuration(body, io.length, "VersioningConfiguration")) {
        error_set(error, "Versioned buckets require a version-aware adapter; use an unversioned dedicated bucket");
        result = STORAGE_S3_CONFLICT;
    }
    if (result == 0) {
        memset(body, 0, S3_REPLY_LIMIT);
        io.length = 0; io.bytes = 0;
        result = request(target, NULL, "lifecycle", "GET", NULL, 0, NULL, &io, NULL, error);
        if (result == STORAGE_S3_MISSING) result = 0;
        else if (result == 0 && !safe_lifecycle(body)) {
            error_set(error, "Bucket lifecycle may only abort incomplete uploads after 1-7 days; expiry and transitions are unsupported");
            result = STORAGE_S3_CONFLICT;
        }
    }
    free(body);
    if (result == 0 && write_test) {
        char uuid[LIGHTNVR_UUID_STRING_SIZE], key[96];
        FILE *payload = tmpfile();
        if (!payload || lightnvr_uuid_generate_v4(uuid)) result = STORAGE_S3_ERROR;
        else {
            snprintf(key, sizeof(key), ".lightnvr-probes/%s", uuid);
            fputs("1", payload); rewind(payload);
            s3_io_t response = {.limit = S3_REPLY_LIMIT};
            const char *checksum = "6b86b273ff34fce19d6b804eff5a3f5747ada4eaa22f1d49c01e52ddb7875b4b";
            result = request(target, key, NULL, "PUT", payload, 1, checksum, &response, NULL, error);
            if (result == 0) result = storage_s3_verify(target, key, 1, checksum, NULL, error);
            if (result == 0) {
                s3_io_t anonymous = {.limit = S3_REPLY_LIMIT, .anonymous = true};
                request(target, key, NULL, "GET", NULL, 0, NULL, &anonymous, NULL, error);
                if (anonymous.status != 401 && anonymous.status != 403 && anonymous.status != 404) {
                    error_set(error, "Archive probe is publicly readable or private access could not be verified");
                    result = STORAGE_S3_CONFLICT;
                }
            }
            int cleanup = storage_s3_delete(target, key, error);
            if (cleanup != 0) result = cleanup;
        }
        if (payload) fclose(payload);
    }
    target->capacity_bytes = 0;
    target->available_bytes = 0;
    target->filesystem_device = 0;
    target->last_probe_at = time(NULL);
    if (result == 0) target->last_success_at = target->last_probe_at;
    safe_strcpy(target->health_status, result == 0 ? "healthy" : "unavailable", sizeof(target->health_status), 0);
    safe_strcpy(target->last_error, result == 0 ? "" : (*error ? error : "Archive probe failed"), sizeof(target->last_error), 0);
    return result;
}

#else
#include <string.h>
#include <stdio.h>
static int unsupported(char *error) {
    if (error) snprintf(error, STORAGE_TARGET_ERROR_MAX, "S3 archive support is disabled in this build");
    return STORAGE_S3_ERROR;
}
bool storage_s3_validate(const storage_target_t *t, char *e, size_t n) {
    (void)t; if (e && n) snprintf(e, n, "S3 archive support is disabled in this build"); return false;
}
int storage_s3_probe(storage_target_t *t, bool write_test) {
    (void)write_test; strcpy(t->health_status, "unavailable"); return unsupported(t->last_error);
}
int storage_s3_stat(const storage_target_t *t,const char *k,uint64_t *n,char e[256]) {
    (void)t;(void)k;(void)n;return unsupported(e);
}
int storage_s3_upload(const storage_target_t *t,const char *k,const char *p,const char *h,const storage_transfer_control_t *c,char e[256]) {
    (void)t;(void)k;(void)p;(void)h;(void)c;return unsupported(e);
}
int storage_s3_download(const storage_target_t *t,const char *k,const char *p,uint64_t n,const char *h,const storage_transfer_control_t *c,char e[256]) {
    (void)t;(void)k;(void)p;(void)n;(void)h;(void)c;return unsupported(e);
}
int storage_s3_verify(const storage_target_t *t,const char *k,uint64_t n,const char *h,const storage_transfer_control_t *c,char e[256]) {
    (void)t;(void)k;(void)n;(void)h;(void)c;return unsupported(e);
}
int storage_s3_delete(const storage_target_t *t,const char *k,char e[256]) {
    (void)t;(void)k;return unsupported(e);
}
int storage_s3_abort_upload(const storage_target_t *t,const char *k,const char *id,char e[256]) {
    (void)t;(void)k;(void)id;return unsupported(e);
}
int storage_s3_read_range(const storage_target_t *t,const char *k,uint64_t o,size_t n,uint64_t total,void *b,const storage_transfer_control_t *c,char e[256]) {
    (void)t;(void)k;(void)o;(void)n;(void)total;(void)b;(void)c;return unsupported(e);
}
#endif
