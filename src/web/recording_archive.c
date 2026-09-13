/* Authenticated, bounded S3 range proxy. Each next read starts only after the
 * previous socket write completes. No archive request consumes PVC space. */
#ifdef HTTP_BACKEND_LIBUV
#include "web/recording_archive.h"
#include "web/libuv_connection.h"
#include "storage/storage_source.h"
#include "storage/storage_s3.h"
#include <stdatomic.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>

#define ARCHIVE_CHUNK (1024U * 1024U)
#define ARCHIVE_STREAMS 2

typedef struct {
    uv_work_t work;
    uv_write_t write;
    uv_timer_t lease_timer;
    unsigned references; // loop-owned: one for I/O and one until the timer closes
    bool timer_initialized;
    libuv_connection_t *conn; // accessed only on the loop after start
    storage_remote_source_t source;
    atomic_bool cancelled;
    bool download, headers_sent, counted;
    uint64_t offset, remaining;
    size_t length;
    int result;
    char error[256], headers[1024];
    size_t header_length;
    unsigned char buffer[ARCHIVE_CHUNK];
} archive_stream_t;

static archive_stream_t *streams[ARCHIVE_STREAMS];
static unsigned active_streams; // loop-owned; reserve pool threads for other handlers
static void archive_next(archive_stream_t *stream);
extern int libuv_send_response_ex(libuv_connection_t *, const http_response_t *, write_complete_action_t);

static void archive_release(archive_stream_t *stream) {
    if (--stream->references == 0) free(stream);
}

static void archive_timer_closed(uv_handle_t *handle) {
    archive_release(handle->data);
}

static void archive_stop_timer(archive_stream_t *stream) {
    if (stream->timer_initialized && !uv_is_closing((uv_handle_t *)&stream->lease_timer)) {
        uv_timer_stop(&stream->lease_timer);
        uv_close((uv_handle_t *)&stream->lease_timer, archive_timer_closed);
    }
}

/* Server shutdown walks every handle; our timer must retain its own close
 * callback because outstanding work/write callbacks still reference the stream. */
bool recording_archive_close_timer(uv_handle_t *handle) {
    for (unsigned i = 0; i < ARCHIVE_STREAMS; i++) {
        if (streams[i] && handle == (uv_handle_t *)&streams[i]->lease_timer) {
            archive_stop_timer(streams[i]);
            return true;
        }
    }
    return false;
}

static void archive_free(archive_stream_t *stream) {
    if (stream->counted) {
        active_streams--;
        for (unsigned i = 0; i < ARCHIVE_STREAMS; i++)
            if (streams[i] == stream) streams[i] = NULL;
    }
    // Stop renewal only after the final write or cancelled I/O completes. The
    // shared lease retains its expiry grace for other readers of this recording.
    archive_stop_timer(stream);
    archive_release(stream);
}

static void archive_finish(archive_stream_t *stream, bool failed) {
    libuv_connection_t *conn = stream->conn;
    if (!conn) { archive_free(stream); return; }
    conn->archive_stream = NULL;
    conn->async_response_pending = false;
    bool headers_sent = stream->headers_sent;
    archive_free(stream);
    if (failed && !headers_sent) {
        http_response_set_json_error(&conn->response, 503, "Archive source is unavailable; retry later");
        http_response_add_header(&conn->response, "Retry-After", "5");
        libuv_send_response_ex(conn, &conn->response, WRITE_ACTION_CLOSE);
    } else if (!failed && conn->keep_alive && llhttp_should_keep_alive(&conn->parser) && !conn->server->shutting_down)
        libuv_connection_reset(conn);
    else libuv_connection_close(conn);
}

static bool archive_cancelled(void *context) {
    archive_stream_t *stream = context;
    return atomic_load(&stream->cancelled);
}

static void archive_renew(uv_timer_t *timer) {
    archive_stream_t *stream = timer->data;
    if (!stream->conn || archive_cancelled(stream)) { archive_stop_timer(stream); return; }
    if (!storage_source_renew_lease(stream->source.recording_id)) {
        atomic_store(&stream->cancelled, true);
        archive_stop_timer(stream);
        libuv_connection_close(stream->conn);
    }
}

static void archive_read(uv_work_t *work) {
    archive_stream_t *stream = work->data;
    stream->result = -1;
    if (!archive_cancelled(stream) && storage_source_renew_lease(stream->source.recording_id)) {
        storage_transfer_control_t control = {.cancelled = archive_cancelled, .context = stream,
            .bandwidth_bps = stream->source.target.migration_bandwidth_bps};
        stream->result = storage_s3_read_range(&stream->source.target, stream->source.key,
            stream->offset, stream->length, stream->source.size, stream->buffer, &control, stream->error);
    }
}

static void archive_written(uv_write_t *write, int status) {
    archive_stream_t *stream = write->data;
    if (!stream->conn || status < 0 || archive_cancelled(stream)) { archive_finish(stream, true); return; }
    stream->offset += stream->length;
    stream->remaining -= stream->length;
    archive_next(stream);
}

static void archive_read_done(uv_work_t *work, int status) {
    archive_stream_t *stream = work->data;
    if (!stream->conn || status < 0 || stream->result || archive_cancelled(stream)) {
        archive_finish(stream, true); return;
    }
    uv_buf_t buffers[2];
    unsigned count = 0;
    if (!stream->headers_sent) buffers[count++] = uv_buf_init(stream->headers, (unsigned)stream->header_length);
    buffers[count++] = uv_buf_init((char *)stream->buffer, (unsigned)stream->length);
    stream->write.data = stream;
    if (uv_write(&stream->write, (uv_stream_t *)&stream->conn->handle, buffers, count, archive_written)) {
        archive_finish(stream, true); return;
    }
    stream->headers_sent = true;
}

static void archive_next(archive_stream_t *stream) {
    if (!stream->conn || archive_cancelled(stream)) { archive_finish(stream, true); return; }
    if (!stream->remaining) { archive_finish(stream, false); return; }
    stream->length = stream->remaining < ARCHIVE_CHUNK ? (size_t)stream->remaining : ARCHIVE_CHUNK;
    stream->work.data = stream;
    if (uv_queue_work(stream->conn->server->loop, &stream->work, archive_read, archive_read_done))
        archive_finish(stream, true);
}

bool recording_archive_serve(const http_request_t *req, http_response_t *res, uint64_t id, bool download) {
    char transcode[8], prepare[8];
    // Local consumers and compatibility encoding use the verified staging cache.
    if (!download && http_request_get_query_param(req, "transcode", transcode, sizeof(transcode)) > 0 && !strcmp(transcode, "1")) return false;
    bool preparing = http_request_get_query_param(req, "prepare", prepare, sizeof(prepare)) > 0 && !strcmp(prepare, "1");
    // Preserve legacy prepare=1 compatibility behavior when transcode is absent.
    if (!download && preparing && http_request_get_query_param(req, "transcode", transcode, sizeof(transcode)) <= 0) return false;
    storage_remote_source_t source;
    int result = storage_source_remote(id, &source);
    if (result == STORAGE_SOURCE_MISSING) return false;
    if (result != STORAGE_SOURCE_READY) {
        http_response_set_json_error(res, result == STORAGE_SOURCE_DELETING ? 409 : 503, "Recording source is unavailable");
        return true;
    }
    if (preparing) {
        http_response_set_json(res, 200, "{\"status\":\"ready\",\"source\":\"archive\"}");
        return true;
    }
    libuv_connection_t *conn = req->user_data;
    if (!conn) return false;
    archive_stream_t *stream = calloc(1, sizeof(*stream));
    if (!stream) { http_response_set_json_error(res, 503, "Archive streaming is busy"); return true; }
    stream->references = 1;
    stream->source = source;
    stream->download = download;
    atomic_init(&stream->cancelled, false);
    conn->archive_stream = stream;
    if (!conn->handler_on_worker) recording_archive_start(conn);
    return true;
}

int recording_archive_start(libuv_connection_t *conn) {
    archive_stream_t *stream = conn->archive_stream;
    if (!stream) return -1;
    stream->conn = conn;
    if (active_streams >= ARCHIVE_STREAMS || !stream->source.size || stream->source.size > SIZE_MAX) {
        conn->archive_stream = NULL;
        archive_free(stream);
        http_response_set_json_error(&conn->response, 503, "Archive streaming is busy; retry later");
        http_response_add_header(&conn->response, "Retry-After", "5");
        return -1;
    }
    size_t start = 0, end = (size_t)stream->source.size-1;
    const char *range = http_request_get_header(&conn->request, "Range");
    bool valid_range = true;
    if (range) {
        valid_range = !strncmp(range, "bytes=", 6) && range[6] && strspn(range + 6, "0123456789-") == strlen(range + 6);
        const char *dash = strchr(range, '-');
        valid_range = valid_range && dash && !strchr(dash + 1, '-') && strcmp(range + 6, "-");
    }
    if (range && (!valid_range || !libuv_parse_range_header(range, (size_t)stream->source.size, &start, &end))) {
        char value[64];
        snprintf(value, sizeof(value), "bytes */%llu", (unsigned long long)stream->source.size);
        http_response_set_json_error(&conn->response, 416, "Requested range is not satisfiable");
        http_response_add_header(&conn->response, "Content-Range", value);
        conn->archive_stream = NULL;
        archive_free(stream);
        return -1;
    }
    stream->offset = start;
    stream->remaining = end - start + 1;
    char content_range[128] = {0}, disposition[128] = {0};
    if (range) snprintf(content_range, sizeof(content_range), "Content-Range: bytes %zu-%zu/%llu\r\n", start, end,
                        (unsigned long long)stream->source.size);
    if (stream->download) snprintf(disposition, sizeof(disposition), "Content-Disposition: attachment; filename=\"recording-%llu.mp4\"\r\n",
                                   (unsigned long long)stream->source.recording_id);
    int n = snprintf(stream->headers, sizeof(stream->headers), "HTTP/1.1 %s\r\nContent-Type: video/mp4\r\n"
        "Content-Length: %llu\r\nAccept-Ranges: bytes\r\nCache-Control: private, no-store\r\n%s%s\r\n",
        range ? "206 Partial Content" : "200 OK", (unsigned long long)stream->remaining, content_range, disposition);
    stream->header_length = (size_t)n;
    stream->counted = true;
    active_streams++;
    for (unsigned i = 0; i < ARCHIVE_STREAMS; i++) {
        if (!streams[i]) { streams[i] = stream; break; }
    }
    conn->async_response_pending = true;
    if (uv_timer_init(conn->server->loop, &stream->lease_timer)) {
        archive_finish(stream, true);
        return 0;
    }
    stream->timer_initialized = true;
    stream->references++;
    stream->lease_timer.data = stream;
    if (uv_timer_start(&stream->lease_timer, archive_renew, 30000, 30000)) {
        archive_finish(stream, true);
        return 0;
    }
    archive_next(stream);
    return 0;
}

void recording_archive_disconnected(libuv_connection_t *conn) {
    archive_stream_t *stream = conn->archive_stream;
    if (!stream) return;
    conn->archive_stream = NULL;
    if (!stream->counted) { archive_free(stream); return; }
    stream->conn = NULL;
    atomic_store(&stream->cancelled, true);
    // The outstanding work/write callback releases the final reference.
}
#endif
