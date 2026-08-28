#define _POSIX_C_SOURCE 200809L

#include <arpa/inet.h>
#include <pthread.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/select.h>
#include <sys/socket.h>
#include <time.h>
#include <unistd.h>
#include <sqlite3.h>

#include "unity.h"
#include "core/config.h"
#include "core/logger.h"
#include "core/shutdown_coordinator.h"
#include "database/db_core.h"
#include "database/db_recordings.h"
#include "video/detection.h"
#include "video/mp4_recording.h"
#include "video/mp4_writer.h"
#include "video/onvif_detection.h"

#define TEST_DB_PATH "/tmp/lightnvr_unit_onvif_test.db"

extern config_t g_config;

static mp4_writer_t g_annotation_writer;
static bool g_annotation_writer_registered;
static int g_saved_max_streams;

typedef struct {
    int listen_fd;
    int port;
    int connections;
    bool stop;
    pthread_t thread;
    const char *pull_body;  // Optional PullMessages response body (NULL → empty)
    bool reject_initial_termination_time;
    bool saw_initial_termination_time;
} fake_onvif_server_t;

static void send_xml_response_with_status(int client_fd, int status,
                                          const char *reason,
                                          const char *body) {
    char response[4096];
    int len = snprintf(response, sizeof(response),
                       "HTTP/1.1 %d %s\r\nContent-Type: application/soap+xml\r\n"
                       "Content-Length: %zu\r\nConnection: close\r\n\r\n%s",
                       status, reason, strlen(body), body);
    send(client_fd, response, (size_t)len, 0);
}

static void send_xml_response(int client_fd, const char *body) {
    send_xml_response_with_status(client_fd, 200, "OK", body);
}

static void read_http_request(int client_fd, char *request,
                              size_t request_size) {
    size_t used = 0;
    size_t expected = 0;
    request[0] = '\0';
    while (used + 1 < request_size) {
        ssize_t received = recv(client_fd, request + used,
                                request_size - used - 1, 0);
        if (received <= 0) break;
        used += (size_t)received;
        request[used] = '\0';

        char *body = strstr(request, "\r\n\r\n");
        if (!body) continue;
        size_t header_size = (size_t)(body + 4 - request);
        if (expected == 0) {
            const char *length = strstr(request, "Content-Length:");
            expected = header_size + (length
                ? (size_t)strtoul(length + strlen("Content-Length:"), NULL, 10)
                : 0);
        }
        if (used >= expected) break;
    }
}

static void *fake_onvif_server_main(void *arg) {
    fake_onvif_server_t *server = (fake_onvif_server_t *)arg;
    while (!server->stop && server->connections < 3) {
        fd_set readfds;
        struct timeval timeout = {.tv_sec = 0, .tv_usec = 100000};
        FD_ZERO(&readfds);
        FD_SET(server->listen_fd, &readfds);
        int ready = select(server->listen_fd + 1, &readfds, NULL, NULL, &timeout);
        if (ready <= 0) continue;

        int client_fd = accept(server->listen_fd, NULL, NULL);
        if (client_fd < 0) continue;

        char request[8192];
        read_http_request(client_fd, request, sizeof(request));
        server->connections++;

        if (server->connections == 1) {
            // GetServices response
            char body[512];
            snprintf(body, sizeof(body),
                "<Envelope><Body><GetServicesResponse>"
                "<Service><Namespace>http://www.onvif.org/ver10/events/wsdl</Namespace>"
                "<XAddr>http://127.0.0.1:%d/onvif/events</XAddr></Service>"
                "</GetServicesResponse></Body></Envelope>", server->port);
            send_xml_response(client_fd, body);
        } else if (server->connections == 2) {
            // Subscribe response
            server->saw_initial_termination_time =
                strstr(request, "InitialTerminationTime") != NULL;
            if (server->reject_initial_termination_time &&
                server->saw_initial_termination_time) {
                send_xml_response_with_status(client_fd, 400, "Bad Request",
                    "<Envelope><Body><Fault><Code><Value>SOAP-ENV:Sender</Value>"
                    "<Subcode><Value>ter:InvalidArgVal</Value></Subcode></Code>"
                    "<Reason><Text>error</Text></Reason></Fault></Body></Envelope>");
                close(client_fd);
                continue;
            }
            char body[512];
            snprintf(body, sizeof(body),
                     "<Envelope><Body><wsa:Address>http://127.0.0.1:%d/pull_service</wsa:Address>"
                     "</Body></Envelope>",
                     server->port);
            send_xml_response(client_fd, body);
        } else {
            send_xml_response(client_fd,
                server->pull_body
                    ? server->pull_body
                    : "<Envelope><Body><PullMessagesResponse/></Body></Envelope>");
        }

        close(client_fd);
    }

    close(server->listen_fd);
    server->listen_fd = -1;
    return NULL;
}

static int start_fake_onvif_server(fake_onvif_server_t *server) {
    memset(server, 0, sizeof(*server));
    server->listen_fd = socket(AF_INET, SOCK_STREAM, 0);
    if (server->listen_fd < 0) return -1;

    struct sockaddr_in addr;
    memset(&addr, 0, sizeof(addr));
    addr.sin_family = AF_INET;
    addr.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
    addr.sin_port = 0;

    if (bind(server->listen_fd, (struct sockaddr *)&addr, sizeof(addr)) != 0) return -1;
    if (listen(server->listen_fd, 4) != 0) return -1;

    socklen_t addr_len = sizeof(addr);
    if (getsockname(server->listen_fd, (struct sockaddr *)&addr, &addr_len) != 0) return -1;
    server->port = ntohs(addr.sin_port);

    return pthread_create(&server->thread, NULL, fake_onvif_server_main, server);
}

static void stop_fake_onvif_server(fake_onvif_server_t *server) {
    server->stop = true;
    pthread_join(server->thread, NULL);
}

void setUp(void) {
    init_shutdown_coordinator();
    sqlite3_exec(get_db_handle(), "DELETE FROM detections;", NULL, NULL, NULL);
    sqlite3_exec(get_db_handle(), "DELETE FROM recordings;", NULL, NULL, NULL);
    memset(&g_annotation_writer, 0, sizeof(g_annotation_writer));
    g_annotation_writer_registered = false;
    g_saved_max_streams = g_config.max_streams;
}

void tearDown(void) {
    if (g_annotation_writer_registered) {
        unregister_mp4_writer_for_stream("onvif-link");
        g_annotation_writer_registered = false;
    }
    g_config.max_streams = g_saved_max_streams;
    shutdown_detection_system();
    shutdown_coordinator_cleanup();
}

void test_init_detection_system_initializes_onvif_detection(void) {
    fake_onvif_server_t server;
    TEST_ASSERT_EQUAL_INT(0, start_fake_onvif_server(&server));
    TEST_ASSERT_EQUAL_INT(0, init_detection_system());

    char url[64];
    snprintf(url, sizeof(url), "http://127.0.0.1:%d", server.port);

    detection_result_t result;
    memset(&result, 0, sizeof(result));
    TEST_ASSERT_EQUAL_INT(0, detect_motion_onvif(url, "", "", &result, ""));

    stop_fake_onvif_server(&server);
    TEST_ASSERT_EQUAL_INT(3, server.connections);
    TEST_ASSERT_EQUAL_INT(0, result.count);
}

void test_onvif_subscription_uses_pull_messages_keepalive(void) {
    fake_onvif_server_t server;
    TEST_ASSERT_EQUAL_INT(0, start_fake_onvif_server(&server));
    server.reject_initial_termination_time = true;
    TEST_ASSERT_EQUAL_INT(0, init_detection_system());

    char url[64];
    snprintf(url, sizeof(url), "http://127.0.0.1:%d", server.port);

    detection_result_t result;
    memset(&result, 0, sizeof(result));
    TEST_ASSERT_EQUAL_INT(0, detect_motion_onvif(url, "", "", &result, ""));

    stop_fake_onvif_server(&server);
    TEST_ASSERT_EQUAL_INT(3, server.connections);
    TEST_ASSERT_FALSE(server.saw_initial_termination_time);
}

// A Dahua-style SMD Plus smart-detection event carrying ObjectType=Human must
// be surfaced as a "person" detection rather than a generic "motion" (#456).
void test_onvif_smart_detection_reports_object_class(void) {
    fake_onvif_server_t server;
    TEST_ASSERT_EQUAL_INT(0, start_fake_onvif_server(&server));
    server.pull_body =
        "<Envelope><Body><PullMessagesResponse><NotificationMessage>"
        "<Topic>tns1:RuleEngine/SmartMotionDetection/Motion</Topic>"
        "<Message><Message><Source>"
        "<SimpleItem Name=\"VideoSourceToken\" Value=\"000\"/></Source>"
        "<Data>"
        "<SimpleItem Name=\"IsMotion\" Value=\"true\"/>"
        "<SimpleItem Name=\"ObjectType\" Value=\"Human\"/>"
        "</Data></Message></Message>"
        "</NotificationMessage></PullMessagesResponse></Body></Envelope>";

    TEST_ASSERT_EQUAL_INT(0, init_detection_system());

    char url[64];
    snprintf(url, sizeof(url), "http://127.0.0.1:%d", server.port);

    detection_result_t result;
    memset(&result, 0, sizeof(result));
    // Empty stream name keeps the call side-effect-free (no DB/MQTT/zone steps)
    // while still exercising the full parse + classify path.
    TEST_ASSERT_EQUAL_INT(0, detect_motion_onvif(url, "", "", &result, ""));

    stop_fake_onvif_server(&server);
    TEST_ASSERT_EQUAL_INT(1, result.count);
    TEST_ASSERT_EQUAL_STRING("person", result.detections[0].label);
}

/* Regression for #547: ONVIF used to hardcode recording_id=0 when it stored
 * the parsed event, bypassing the annotation linkage used by every frame-based
 * detector. Verify the actual persisted foreign key against a live writer ID. */
void test_onvif_detection_links_to_active_continuous_recording(void) {
    fake_onvif_server_t server;
    TEST_ASSERT_EQUAL_INT(0, start_fake_onvif_server(&server));
    server.pull_body =
        "<Envelope><Body><PullMessagesResponse><NotificationMessage>"
        "<Topic>tns1:RuleEngine/CellMotionDetector/Motion</Topic>"
        "<Message><Message><Data>"
        "<SimpleItem Name=\"IsMotion\" Value=\"true\"/>"
        "</Data></Message></Message>"
        "</NotificationMessage></PullMessagesResponse></Body></Envelope>";

    TEST_ASSERT_EQUAL_INT(0, init_detection_system());

    /* The normal application configures the registry capacity before starting
     * writers. Keep init_detection_system() on this test's intentionally empty
     * config, then expose one registry slot for the writer under test. */
    g_config.max_streams = 1;

    recording_metadata_t recording;
    memset(&recording, 0, sizeof(recording));
    snprintf(recording.stream_name, sizeof(recording.stream_name),
             "%s", "onvif-link");
    snprintf(recording.file_path, sizeof(recording.file_path),
             "%s", "/tmp/onvif-link.mp4");
    snprintf(recording.codec, sizeof(recording.codec), "%s", "h264");
    snprintf(recording.trigger_type, sizeof(recording.trigger_type),
             "%s", "continuous");
    recording.start_time = time(NULL) - 10;
    recording.is_complete = false;
    recording.retention_tier = RETENTION_TIER_STANDARD;

    uint64_t recording_id = add_recording_metadata(&recording);
    TEST_ASSERT_NOT_EQUAL_UINT64(0, recording_id);
    g_annotation_writer.current_recording_id = recording_id;
    TEST_ASSERT_EQUAL_INT(0, register_mp4_writer_for_stream(
        "onvif-link", &g_annotation_writer));
    g_annotation_writer_registered = true;

    char url[64];
    snprintf(url, sizeof(url), "http://127.0.0.1:%d", server.port);

    detection_result_t result;
    memset(&result, 0, sizeof(result));
    TEST_ASSERT_EQUAL_INT(0, detect_motion_onvif(
        url, "", "", &result, "onvif-link"));
    stop_fake_onvif_server(&server);

    TEST_ASSERT_EQUAL_INT(1, result.count);

    sqlite3_stmt *stmt = NULL;
    TEST_ASSERT_EQUAL_INT(SQLITE_OK, sqlite3_prepare_v2(get_db_handle(),
        "SELECT recording_id FROM detections "
        "WHERE stream_name = 'onvif-link' LIMIT 1;", -1, &stmt, NULL));
    TEST_ASSERT_EQUAL_INT(SQLITE_ROW, sqlite3_step(stmt));
    TEST_ASSERT_EQUAL_UINT64(recording_id,
                             (uint64_t)sqlite3_column_int64(stmt, 0));
    sqlite3_finalize(stmt);
}

int main(void) {
    unlink(TEST_DB_PATH);
    if (init_database(TEST_DB_PATH) != 0) {
        fprintf(stderr, "FATAL: init_database failed\n");
        return 1;
    }
    init_logger();
    UNITY_BEGIN();
    RUN_TEST(test_init_detection_system_initializes_onvif_detection);
    RUN_TEST(test_onvif_subscription_uses_pull_messages_keepalive);
    RUN_TEST(test_onvif_smart_detection_reports_object_class);
    RUN_TEST(test_onvif_detection_links_to_active_continuous_recording);
    int result = UNITY_END();
    shutdown_logger();
    shutdown_database();
    unlink(TEST_DB_PATH);
    return result;
}
