/* INT_01 L7: HTTP API detector and ONVIF observation evidence.
 *
 * A synthetic decoded frame goes through the production
 * detect_on_decoded_frame() API dispatch to a real detect_objects_api_*()
 * request against a local fake detection server. The stored evidence must be
 * byte-identical to the JPEG in the request body (origin submitted_image, no
 * re-encode, no second fetch), a server-declared letterbox (C3 fixture D) is
 * mapped onto the stored image, and nothing is queued when evidence is off.
 * ONVIF object-class events become unavailable(no_matching_media)
 * observations; motion-only events never do. Real migrated DB + real L3
 * worker. */
#include "../../src/video/unified_detection_thread.c"

#include <arpa/inet.h>
#include <netinet/in.h>
#include <sys/socket.h>
#include <sys/stat.h>

#include "unity.h"
#include "core/event_identity.h"
#include "core/observation_evidence.h"
#include "database/db_core.h"
#include "database/db_observations.h"
#include "database/db_streams.h"
#include "video/api_detection.h"
#include "video/detection_evidence.h"

static char db_path[256];
static char storage_dir[256];
static char cam_uuid[OBS_UUID_SIZE];
static unified_detection_ctx_t *ctx;

int __wrap_filter_detections_by_zones(const char *name, detection_result_t *result) {
    (void)name; (void)result;
    return 0;
}
int __wrap_filter_detections_by_stream_objects(const char *name, detection_result_t *result) {
    (void)name; (void)result;
    return 0;
}

/* ---- fake detection server ---------------------------------------------- */

static struct {
    int listen_fd;
    int port;
    pthread_t thread;
    atomic_bool stop;
    pthread_mutex_t lock;
    char response[4096];
    uint8_t *last_jpeg;      /* image extracted from the last request body */
    size_t last_jpeg_size;
    int requests;
} srv;

static uint8_t *memfind(uint8_t *hay, size_t hay_len, const char *needle, size_t needle_len) {
    if (needle_len == 0 || hay_len < needle_len) return NULL;
    for (size_t i = 0; i + needle_len <= hay_len; ++i)
        if (memcmp(hay + i, needle, needle_len) == 0) return hay + i;
    return NULL;
}

static int b64_value(char c) {
    if (c >= 'A' && c <= 'Z') return c - 'A';
    if (c >= 'a' && c <= 'z') return c - 'a' + 26;
    if (c >= '0' && c <= '9') return c - '0' + 52;
    if (c == '+') return 62;
    if (c == '/') return 63;
    return -1;
}

/* Standard padded base64 -> bytes; returns the length or -1. */
static int b64_decode(const char *in, uint8_t *out) {
    int n = 0, bits = 0;
    unsigned acc = 0;
    for (const char *p = in; *p && *p != '='; ++p) {
        int v = b64_value(*p);
        if (v < 0) return -1;
        acc = (acc << 6) | (unsigned)v;
        bits += 6;
        if (bits >= 8) {
            bits -= 8;
            out[n++] = (uint8_t)((acc >> bits) & 0xFF);
        }
    }
    return n;
}

/* light-object-detect multipart "file" part, or DOODS2 JSON "data". */
static void capture_image(uint8_t *body, size_t len, const char *headers) {
    uint8_t *img = NULL;
    size_t img_len = 0;
    if (strcasestr(headers, "Content-Type: application/json")) {
        body[len] = '\0';
        cJSON *root = cJSON_Parse((const char *)body);
        const cJSON *data = root ? cJSON_GetObjectItem(root, "data") : NULL;
        if (cJSON_IsString(data)) {
            size_t cap = strlen(data->valuestring);
            img = malloc(cap + 1);
            int n = img ? b64_decode(data->valuestring, img) : -1;
            if (n > 0) img_len = (size_t)n; else { free(img); img = NULL; }
        }
        cJSON_Delete(root);
    } else {
        uint8_t *part = memfind(body, len, "Content-Type: image/jpeg", 24);
        uint8_t *start = part ? memfind(part, len - (size_t)(part - body), "\r\n\r\n", 4) : NULL;
        if (start) {
            start += 4;
            /* the part ends at the last CRLF + boundary marker */
            uint8_t *end = NULL;
            for (uint8_t *p = body + len - 4; p > start; --p)
                if (p[0] == '\r' && p[1] == '\n' && p[2] == '-' && p[3] == '-') { end = p; break; }
            if (end) {
                img_len = (size_t)(end - start);
                img = malloc(img_len);
                memcpy(img, start, img_len);
            }
        }
    }
    pthread_mutex_lock(&srv.lock);
    free(srv.last_jpeg);
    srv.last_jpeg = img;
    srv.last_jpeg_size = img_len;
    srv.requests++;
    pthread_mutex_unlock(&srv.lock);
}

static void serve_client(int fd) {
    size_t cap = 1 << 22, len = 0;
    uint8_t *buf = malloc(cap + 1);
    if (!buf) { close(fd); return; }
    size_t header_end = 0;
    long content_length = -1;
    bool continued = false;
    for (;;) {
        ssize_t n = recv(fd, buf + len, cap - len, 0);
        if (n <= 0) break;
        len += (size_t)n;
        buf[len] = '\0';
        if (!header_end) {
            uint8_t *e = memfind(buf, len, "\r\n\r\n", 4);
            if (!e) continue;
            header_end = (size_t)(e - buf) + 4;
            const char *cl = strcasestr((const char *)buf, "Content-Length:");
            if (cl) content_length = strtol(cl + 15, NULL, 10);
            if (!continued && strcasestr((const char *)buf, "Expect: 100-continue")) {
                const char *cont = "HTTP/1.1 100 Continue\r\n\r\n";
                (void)!send(fd, cont, strlen(cont), 0);
                continued = true;
            }
        }
        if (header_end && content_length >= 0 && len >= header_end + (size_t)content_length) break;
    }
    if (header_end && content_length >= 0) {
        char headers[4096];
        size_t hl = header_end < sizeof(headers) - 1 ? header_end : sizeof(headers) - 1;
        memcpy(headers, buf, hl);
        headers[hl] = '\0';
        capture_image(buf + header_end, (size_t)content_length, headers);
    }
    char out[8192];
    pthread_mutex_lock(&srv.lock);
    int body_len = (int)strlen(srv.response);
    int n = snprintf(out, sizeof(out),
                     "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n"
                     "Content-Length: %d\r\nConnection: close\r\n\r\n%s",
                     body_len, srv.response);
    pthread_mutex_unlock(&srv.lock);
    (void)!send(fd, out, (size_t)n, 0);
    free(buf);
    close(fd);
}

static void *server_main(void *arg) {
    (void)arg;
    while (!atomic_load(&srv.stop)) {
        fd_set fds;
        FD_ZERO(&fds);
        FD_SET(srv.listen_fd, &fds);
        struct timeval tv = {0, 100000};
        if (select(srv.listen_fd + 1, &fds, NULL, NULL, &tv) <= 0) continue;
        int fd = accept(srv.listen_fd, NULL, NULL);
        if (fd >= 0) serve_client(fd);
    }
    return NULL;
}

static int server_start(void) {
    pthread_mutex_init(&srv.lock, NULL);
    srv.listen_fd = socket(AF_INET, SOCK_STREAM, 0);
    if (srv.listen_fd < 0) return -1;
    int one = 1;
    setsockopt(srv.listen_fd, SOL_SOCKET, SO_REUSEADDR, &one, sizeof(one));
    struct sockaddr_in addr = {0};
    addr.sin_family = AF_INET;
    addr.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
    if (bind(srv.listen_fd, (struct sockaddr *)&addr, sizeof(addr)) != 0) return -1;
    if (listen(srv.listen_fd, 4) != 0) return -1;
    socklen_t alen = sizeof(addr);
    if (getsockname(srv.listen_fd, (struct sockaddr *)&addr, &alen) != 0) return -1;
    srv.port = ntohs(addr.sin_port);
    return pthread_create(&srv.thread, NULL, server_main, NULL);
}

static void server_stop(void) {
    atomic_store(&srv.stop, true);
    pthread_join(srv.thread, NULL);
    close(srv.listen_fd);
    free(srv.last_jpeg);
    srv.last_jpeg = NULL;
}

static void set_response(const char *json) {
    pthread_mutex_lock(&srv.lock);
    snprintf(srv.response, sizeof(srv.response), "%s", json);
    pthread_mutex_unlock(&srv.lock);
}

/* ---- helpers -------------------------------------------------------------- */

static int scalar(const char *sql) {
    sqlite3_stmt *stmt = NULL;
    int value = -999;
    pthread_mutex_lock(get_db_mutex());
    if (sqlite3_prepare_v2(get_db_handle(), sql, -1, &stmt, NULL) == SQLITE_OK &&
        sqlite3_step(stmt) == SQLITE_ROW)
        value = sqlite3_column_int(stmt, 0);
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(get_db_mutex());
    TEST_ASSERT_NOT_EQUAL_INT_MESSAGE(-999, value, sql);
    return value;
}

static double scalar_double(const char *sql) {
    sqlite3_stmt *stmt = NULL;
    double value = -999.0;
    pthread_mutex_lock(get_db_mutex());
    if (sqlite3_prepare_v2(get_db_handle(), sql, -1, &stmt, NULL) == SQLITE_OK &&
        sqlite3_step(stmt) == SQLITE_ROW)
        value = sqlite3_column_double(stmt, 0);
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(get_db_mutex());
    TEST_ASSERT_TRUE_MESSAGE(value != -999.0, sql);
    return value;
}

static void text_column(const char *sql, char *out, size_t size) {
    sqlite3_stmt *stmt = NULL;
    out[0] = '\0';
    pthread_mutex_lock(get_db_mutex());
    if (sqlite3_prepare_v2(get_db_handle(), sql, -1, &stmt, NULL) == SQLITE_OK &&
        sqlite3_step(stmt) == SQLITE_ROW && sqlite3_column_text(stmt, 0))
        snprintf(out, size, "%s", (const char *)sqlite3_column_text(stmt, 0));
    sqlite3_finalize(stmt);
    pthread_mutex_unlock(get_db_mutex());
}

static void exec_sql(const char *sql) {
    char *error = NULL;
    pthread_mutex_lock(get_db_mutex());
    int rc = sqlite3_exec(get_db_handle(), sql, NULL, NULL, &error);
    pthread_mutex_unlock(get_db_mutex());
    sqlite3_free(error);
    TEST_ASSERT_EQUAL_INT_MESSAGE(SQLITE_OK, rc, sql);
}

static void wait_idle(void) {
    for (int i = 0; i < 1000; ++i) {
        observation_evidence_stats_t s;
        observation_evidence_get_stats(&s);
        if (s.backlog_items == 0 && s.drop_ring_depth == 0) {
            usleep(20000);
            observation_evidence_get_stats(&s);
            if (s.backlog_items == 0 && s.drop_ring_depth == 0) return;
        }
        usleep(10000);
    }
    TEST_FAIL_MESSAGE("evidence worker did not go idle");
}

static AVFrame *make_frame(int w, int h, int bird_x) {
    AVFrame *f = av_frame_alloc();
    TEST_ASSERT_NOT_NULL(f);
    f->format = AV_PIX_FMT_YUV420P;
    f->width = w;
    f->height = h;
    TEST_ASSERT_EQUAL_INT(0, av_frame_get_buffer(f, 0));
    for (int y = 0; y < h; ++y)
        for (int x = 0; x < w; ++x) {
            bool bird = x >= bird_x && x < bird_x + 40 && y >= h / 2 && y < h / 2 + 40;
            f->data[0][y * f->linesize[0] + x] = bird ? 235 : (uint8_t)(32 + (x + y) / 8);
        }
    for (int y = 0; y < h / 2; ++y)
        for (int x = 0; x < w / 2; ++x) {
            f->data[1][y * f->linesize[1] + x] = (uint8_t)(96 + x / 8);
            f->data[2][y * f->linesize[2] + x] = (uint8_t)(160 - y / 8);
        }
    return f;
}

static uint8_t *read_file(const char *path, size_t *size) {
    FILE *f = fopen(path, "rb");
    TEST_ASSERT_NOT_NULL_MESSAGE(f, path);
    fseek(f, 0, SEEK_END);
    long n = ftell(f);
    fseek(f, 0, SEEK_SET);
    uint8_t *buf = malloc((size_t)n);
    TEST_ASSERT_NOT_NULL(buf);
    TEST_ASSERT_EQUAL_size_t((size_t)n, fread(buf, 1, (size_t)n, f));
    fclose(f);
    *size = (size_t)n;
    return buf;
}

static void ready_asset_path(char *out, size_t size) {
    char rel[512];
    text_column("SELECT path FROM observation_assets WHERE state = 'ready'", rel, sizeof(rel));
    TEST_ASSERT_TRUE_MESSAGE(rel[0], "no ready asset");
    snprintf(out, size, "%s/%s", storage_dir, rel);
}

static void use_api_engine(const char *format) {
    snprintf(ctx->model_path, sizeof(ctx->model_path), "http://127.0.0.1:%d/detect", srv.port);
    snprintf(ctx->engines[0].engine_key, sizeof(ctx->engines[0].engine_key), "http-yolo");
    snprintf(ctx->engines[0].engine_type, sizeof(ctx->engines[0].engine_type), "api");
    snprintf(ctx->engines[0].model_path, sizeof(ctx->engines[0].model_path), "%s", ctx->model_path);
    snprintf(g_config.api_detection_format, sizeof(g_config.api_detection_format), "%s", format);
}

/* ---- fixtures ------------------------------------------------------------- */

void setUp(void) {
    exec_sql("DELETE FROM object_observations; DELETE FROM observation_assets; "
             "DELETE FROM detections;");
    observation_evidence_config_t cfg = { .storage_path = storage_dir };
    TEST_ASSERT_EQUAL_INT(0, observation_evidence_init(&cfg));
    TEST_ASSERT_EQUAL_INT(0, init_api_detection_system());
    snprintf(g_config.api_detection_backend, sizeof(g_config.api_detection_backend), "onnx");
    snprintf(g_config.api_detection_detector_name, sizeof(g_config.api_detection_detector_name),
             "default");

    ctx = calloc(1, sizeof(*ctx));
    TEST_ASSERT_NOT_NULL(ctx);
    pthread_mutex_init(&ctx->mutex, NULL);
    snprintf(ctx->stream_name, sizeof(ctx->stream_name), "feeder-east");
    snprintf(ctx->camera_uuid, sizeof(ctx->camera_uuid), "%s", cam_uuid);
    ctx->detection_threshold = 0.5f;
    ctx->engine_count = 1;
    ctx->engines[0].enabled = true;
    ctx->engines[0].threshold = 0.5f;
    use_api_engine("light-object-detect");
    atomic_store(&ctx->observation_evidence_enabled, true);
    ctx->observation_labels[0] = '\0';
    pthread_mutex_lock(&srv.lock);
    srv.requests = 0;
    pthread_mutex_unlock(&srv.lock);
}

void tearDown(void) {
    observation_evidence_shutdown(2000);
    shutdown_api_detection_system();
    pthread_mutex_destroy(&ctx->mutex);
    free(ctx);
    ctx = NULL;
}

/* ---- HTTP API ------------------------------------------------------------------ */

static void assert_asset_is_request_image(const char *format) {
    char path[1024];
    ready_asset_path(path, sizeof(path));
    size_t stored_size = 0;
    uint8_t *stored = read_file(path, &stored_size);
    /* Snapshot under the lock; assert outside it (a failed assertion
     * longjmps and must not leave the server thread blocked). */
    pthread_mutex_lock(&srv.lock);
    int requests = srv.requests;
    size_t sent_size = srv.last_jpeg_size;
    uint8_t *sent = sent_size ? malloc(sent_size) : NULL;
    if (sent) memcpy(sent, srv.last_jpeg, sent_size);
    pthread_mutex_unlock(&srv.lock);
    char msg[128];
    snprintf(msg, sizeof(msg), "%s: sent %zu bytes, stored %zu", format, sent_size, stored_size);
    TEST_ASSERT_EQUAL_INT_MESSAGE(1, requests, "exactly one request, no second fetch");
    TEST_ASSERT_NOT_NULL_MESSAGE(sent, msg);
    TEST_ASSERT_EQUAL_size_t_MESSAGE(sent_size, stored_size, msg);
    TEST_ASSERT_EQUAL_MEMORY_MESSAGE(sent, stored, stored_size, msg);
    free(sent);
    free(stored);
}

static void test_asset_bytes_identical_to_request_body_image(void) {
    const char *formats[] = {"light-object-detect", "doods2"};
    for (int f = 0; f < 2; ++f) {
        exec_sql("DELETE FROM object_observations; DELETE FROM observation_assets;");
        pthread_mutex_lock(&srv.lock);
        srv.requests = 0;
        pthread_mutex_unlock(&srv.lock);
        use_api_engine(formats[f]);
        if (f == 0) {
            set_response("{\"detections\":["
                "{\"label\":\"bird\",\"confidence\":0.91,\"x_min\":0.1,\"y_min\":0.5,"
                "\"x_max\":0.25,\"y_max\":0.7,\"track_id\":17},"
                "{\"label\":\"bird\",\"confidence\":0.42,\"x_min\":0.5,\"y_min\":0.5,"
                "\"x_max\":0.6,\"y_max\":0.6},"
                "{\"label\":\"squirrel\",\"confidence\":0.8,\"x_min\":0.6,\"y_min\":0.1,"
                "\"x_max\":0.9,\"y_max\":0.4}]}");
        } else {
            set_response("{\"id\":\"feeder-east\",\"detections\":["
                "{\"top\":0.5,\"left\":0.1,\"bottom\":0.7,\"right\":0.25,"
                "\"label\":\"bird\",\"confidence\":91.0},"
                "{\"top\":0.5,\"left\":0.5,\"bottom\":0.6,\"right\":0.6,"
                "\"label\":\"bird\",\"confidence\":42.0},"
                "{\"top\":0.1,\"left\":0.6,\"bottom\":0.4,\"right\":0.9,"
                "\"label\":\"squirrel\",\"confidence\":80.0}],\"error\":\"\"}");
        }

        AVFrame *frame = make_frame(320, 240, 40);
        time_t now = time(NULL);
        detection_result_t result;
        TEST_ASSERT_TRUE(detect_on_decoded_frame(ctx, frame, now, &result));
        av_frame_free(&frame);
        TEST_ASSERT_EQUAL_INT(2, result.count);  /* 0.42 < threshold */
        wait_idle();

        assert_asset_is_request_image(formats[f]);
        TEST_ASSERT_EQUAL_INT(2, scalar("SELECT count(*) FROM object_observations"));
        TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(DISTINCT asset_uuid) FROM object_observations"));
        TEST_ASSERT_EQUAL_INT(2, scalar("SELECT count(*) FROM object_observations WHERE "
                                        "engine_type = 'api' AND engine_key = 'http-yolo' "
                                        "AND confidence >= 0.5"));
        TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM observation_assets WHERE "
                                        "state = 'ready' AND origin = 'submitted_image' AND "
                                        "media_type = 'image/jpeg' AND width = 320 AND height = 240"));
        TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM observation_assets WHERE "
                                        "transform_json LIKE '%\"mode\":\"none\"%' AND "
                                        "transform_json LIKE '%\"source_width\":320%' AND "
                                        "transform_json LIKE '%\"inference_width\":320%'"));
        char sql[256];
        snprintf(sql, sizeof(sql), "SELECT count(*) FROM object_observations WHERE "
                 "observed_at_ms / 1000 BETWEEN %lld AND %lld", (long long)now, (long long)now + 1);
        TEST_ASSERT_EQUAL_INT(2, scalar(sql));
        /* Detections are stored against the capture-time camera identity. */
        snprintf(sql, sizeof(sql), "SELECT count(*) FROM detections WHERE camera_uuid = '%s'",
                 cam_uuid);
        TEST_ASSERT_TRUE(scalar(sql) >= 2);
    }
    /* Only the DOODS2 batch remains; DOODS2 reports no track IDs. */
    TEST_ASSERT_EQUAL_INT(2, scalar("SELECT count(*) FROM object_observations WHERE track_id IS NULL"));
}

static void test_track_and_model_from_response(void) {
    set_response("{\"model\":\"yolov8n\",\"model_version\":\"8.3.0\",\"detections\":["
        "{\"label\":\"bird\",\"confidence\":0.9,\"x_min\":0.1,\"y_min\":0.1,"
        "\"x_max\":0.2,\"y_max\":0.2,\"track_id\":17}]}");
    AVFrame *frame = make_frame(320, 240, 40);
    detection_result_t result;
    TEST_ASSERT_TRUE(detect_on_decoded_frame(ctx, frame, time(NULL), &result));
    av_frame_free(&frame);
    wait_idle();
    char sql[384];
    snprintf(sql, sizeof(sql), "SELECT count(*) FROM object_observations WHERE track_id = '17' "
             "AND track_namespace = 'http-yolo' AND track_session = '%s' AND "
             "model = 'yolov8n' AND model_version = '8.3.0'", detection_evidence_boot_session());
    TEST_ASSERT_EQUAL_INT(1, scalar(sql));
}

/* C3 fixture D: 1280x720 submitted, letterboxed server-side to 640x640
 * (scale 0.5, pad_y 140); the inference-space box u=256 v=185 128x72 is
 * stored as x 0.4 y 0.125 0.2x0.2 on the submitted image. */
static void test_letterboxed_response_maps_box_to_stored_image(void) {
    set_response("{\"letterbox\":{\"width\":640,\"height\":640,\"scale\":0.5,\"pad_x\":0,"
                 "\"pad_y\":140,\"box_units\":\"pixels\"},\"detections\":["
                 "{\"label\":\"bird\",\"confidence\":0.84,\"x_min\":256,\"y_min\":185,"
                 "\"x_max\":384,\"y_max\":257,\"track_id\":17}]}");
    AVFrame *frame = make_frame(1280, 720, 512);
    detection_result_t result;
    TEST_ASSERT_TRUE(detect_on_decoded_frame(ctx, frame, time(NULL), &result));
    av_frame_free(&frame);
    TEST_ASSERT_EQUAL_INT(1, result.count);
    /* The mapped box is what every consumer (detections table, MQTT) sees. */
    TEST_ASSERT_FLOAT_WITHIN(1e-5f, 0.4f, result.detections[0].x);
    TEST_ASSERT_FLOAT_WITHIN(1e-5f, 0.125f, result.detections[0].y);
    wait_idle();

    assert_asset_is_request_image("letterbox");
    TEST_ASSERT_FLOAT_WITHIN(1e-6, 0.4, scalar_double("SELECT box_x FROM object_observations"));
    TEST_ASSERT_FLOAT_WITHIN(1e-6, 0.125, scalar_double("SELECT box_y FROM object_observations"));
    TEST_ASSERT_FLOAT_WITHIN(1e-6, 0.2, scalar_double("SELECT box_width FROM object_observations"));
    TEST_ASSERT_FLOAT_WITHIN(1e-6, 0.2, scalar_double("SELECT box_height FROM object_observations"));
    char tjson[512];
    text_column("SELECT transform_json FROM observation_assets", tjson, sizeof(tjson));
    const char *expect[] = {"\"mode\":\"letterbox\"", "\"source_width\":1280",
                            "\"source_height\":720", "\"inference_width\":640",
                            "\"inference_height\":640", "\"pad_y\":140"};
    for (size_t i = 0; i < sizeof(expect) / sizeof(expect[0]); ++i)
        TEST_ASSERT_NOT_NULL_MESSAGE(strstr(tjson, expect[i]), tjson);
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM observation_assets WHERE "
                                    "width = 1280 AND height = 720"));
}

static void test_disabled_or_ineligible_submits_nothing(void) {
    set_response("{\"detections\":[{\"label\":\"bird\",\"confidence\":0.9,"
                 "\"x_min\":0.1,\"y_min\":0.1,\"x_max\":0.2,\"y_max\":0.2}]}");
    atomic_store(&ctx->observation_evidence_enabled, false);
    AVFrame *frame = make_frame(320, 240, 40);
    detection_result_t result;
    TEST_ASSERT_TRUE(detect_on_decoded_frame(ctx, frame, time(NULL), &result));

    atomic_store(&ctx->observation_evidence_enabled, true);
    snprintf(ctx->observation_labels, sizeof(ctx->observation_labels), "cat, dog");
    TEST_ASSERT_TRUE(detect_on_decoded_frame(ctx, frame, time(NULL), &result));
    av_frame_free(&frame);

    observation_evidence_stats_t s;
    observation_evidence_get_stats(&s);
    TEST_ASSERT_FALSE(s.allocated);
    TEST_ASSERT_FALSE(s.worker_running);
    TEST_ASSERT_EQUAL_UINT64(0, s.accepted);
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT count(*) FROM object_observations"));
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT count(*) FROM observation_assets"));
    /* Legacy detection storage is unaffected. */
    TEST_ASSERT_EQUAL_INT(2, scalar("SELECT count(*) FROM detections"));
}

/* ---- ONVIF -------------------------------------------------------------------- */

static detection_result_t onvif_result(const char *label) {
    detection_result_t r;
    memset(&r, 0, sizeof(r));
    r.count = 1;
    snprintf(r.detections[0].label, sizeof(r.detections[0].label), "%s", label);
    r.detections[0].confidence = 1.0f;
    r.detections[0].width = r.detections[0].height = 1.0f;
    r.detections[0].track_id = -1;
    return r;
}

static void use_onvif_engine(void) {
    snprintf(ctx->model_path, sizeof(ctx->model_path), "onvif");
    snprintf(ctx->engines[0].engine_key, sizeof(ctx->engines[0].engine_key), "onvif-events");
    snprintf(ctx->engines[0].engine_type, sizeof(ctx->engines[0].engine_type), "onvif");
}

static void test_onvif_object_event_records_no_matching_media(void) {
    use_onvif_engine();
    detection_result_t r = onvif_result("animal");
    int64_t observed = detection_evidence_observed_at_ms(0);
    udt_record_onvif_observations(ctx, &r, observed);
    wait_idle();

    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM object_observations WHERE "
                                    "label = 'animal' AND engine_type = 'onvif' AND "
                                    "engine_key = 'onvif-events' AND box_x IS NULL AND "
                                    "track_id IS NULL AND model IS NULL AND confidence = 1.0"));
    TEST_ASSERT_EQUAL_INT(1, scalar("SELECT count(*) FROM observation_assets WHERE "
                                    "state = 'unavailable' AND reason = 'no_matching_media' AND "
                                    "origin IS NULL AND path IS NULL AND transform_json IS NULL"));
    char sql[256];
    snprintf(sql, sizeof(sql), "SELECT count(*) FROM object_observations WHERE observed_at_ms = %lld",
             (long long)observed);
    TEST_ASSERT_EQUAL_INT(1, scalar(sql));
}

static void test_onvif_motion_only_and_disabled_record_nothing(void) {
    use_onvif_engine();
    detection_result_t motion = onvif_result("motion");
    udt_record_onvif_observations(ctx, &motion, detection_evidence_observed_at_ms(0));
    detection_result_t upper = onvif_result("Motion");
    udt_record_onvif_observations(ctx, &upper, detection_evidence_observed_at_ms(0));

    atomic_store(&ctx->observation_evidence_enabled, false);
    detection_result_t person = onvif_result("person");
    udt_record_onvif_observations(ctx, &person, detection_evidence_observed_at_ms(0));

    atomic_store(&ctx->observation_evidence_enabled, true);
    snprintf(ctx->observation_labels, sizeof(ctx->observation_labels), "bird");
    udt_record_onvif_observations(ctx, &person, detection_evidence_observed_at_ms(0));

    observation_evidence_stats_t s;
    observation_evidence_get_stats(&s);
    TEST_ASSERT_FALSE(s.allocated);
    TEST_ASSERT_EQUAL_UINT64(0, s.unavailable_submitted);
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT count(*) FROM object_observations"));
    TEST_ASSERT_EQUAL_INT(0, scalar("SELECT count(*) FROM observation_assets"));
}

static void test_no_media_helper_rejects_wrong_engine_types(void) {
    detection_result_t r = onvif_result("person");
    detection_evidence_camera_t camera = { .camera_uuid = cam_uuid, .stream_name = "feeder-east",
                                           .evidence_enabled = true };
    detection_evidence_engine_t object = { .engine_key = "k", .engine_type = "object" };
    detection_evidence_engine_t api = { .engine_key = "k", .engine_type = "api" };
    TEST_ASSERT_EQUAL_INT(DETECTION_EVIDENCE_NOT_OBJECT_ENGINE,
        detection_evidence_record_no_media(&camera, &object, r.detections, 1,
                                           detection_evidence_observed_at_ms(0), false, NULL, NULL));
    TEST_ASSERT_EQUAL_INT(DETECTION_EVIDENCE_NOT_OBJECT_ENGINE,
        detection_evidence_record_no_media(&camera, &api, r.detections, 1,
                                           detection_evidence_observed_at_ms(0), false, NULL, NULL));
    /* The JPEG path accepts only api engines and always consumes the bytes. */
    uint8_t *jpeg = malloc(16);
    TEST_ASSERT_NOT_NULL(jpeg);
    TEST_ASSERT_EQUAL_INT(DETECTION_EVIDENCE_NOT_OBJECT_ENGINE,
        detection_evidence_capture_jpeg(&camera, &object, r.detections, 1,
                                        detection_evidence_observed_at_ms(0), jpeg, 16, 4, 4,
                                        NULL, NULL, NULL));
    TEST_ASSERT_TRUE(detection_evidence_is_motion_label("MOTION"));
    TEST_ASSERT_FALSE(detection_evidence_is_motion_label("person"));
}

int main(void) {
    init_logger();
    set_log_level(LOG_LEVEL_ERROR);
    snprintf(db_path, sizeof(db_path), "/tmp/lightnvr_api_evidence_%ld.db", (long)getpid());
    snprintf(storage_dir, sizeof(storage_dir), "/tmp/lightnvr_api_evidence_%ld", (long)getpid());
    unlink(db_path);
    mkdir(storage_dir, 0750);
    if (init_database(db_path) != 0) return 1;
    if (event_identity_init() != 0) return 1;
    if (server_start() != 0) return 1;

    stream_config_t *s = calloc(1, sizeof(*s));
    if (!s) return 1;
    safe_strcpy(s->name, "feeder-east", sizeof(s->name), 0);
    safe_strcpy(s->url, "rtsp://camera/stream", sizeof(s->url), 0);
    safe_strcpy(s->codec, "h264", sizeof(s->codec), 0);
    s->enabled = true;
    s->priority = 5;
    s->segment_duration = 60;
    s->observation_evidence_enabled = true;
    safe_strcpy(s->detection_object_filter, "none", sizeof(s->detection_object_filter), 0);
    if (add_stream_config(s) <= 0) return 1;
    memset(s, 0, sizeof(*s));
    if (get_stream_config_by_name("feeder-east", s) != 0) return 1;
    safe_strcpy(cam_uuid, s->camera_uuid, sizeof(cam_uuid), 0);
    free(s);

    UNITY_BEGIN();
    RUN_TEST(test_disabled_or_ineligible_submits_nothing);
    RUN_TEST(test_asset_bytes_identical_to_request_body_image);
    RUN_TEST(test_track_and_model_from_response);
    RUN_TEST(test_letterboxed_response_maps_box_to_stored_image);
    RUN_TEST(test_onvif_motion_only_and_disabled_record_nothing);
    RUN_TEST(test_onvif_object_event_records_no_matching_media);
    RUN_TEST(test_no_media_helper_rejects_wrong_engine_types);
    int failures = UNITY_END();

    server_stop();
    event_identity_shutdown();
    shutdown_database();
    unlink(db_path);
    char cmd[600];
    snprintf(cmd, sizeof(cmd), "rm -rf '%s'", storage_dir);
    if (system(cmd) != 0) fprintf(stderr, "cleanup failed: %s\n", storage_dir);
    return failures;
}
