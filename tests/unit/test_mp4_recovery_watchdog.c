/* Drive the production outer recorder loop with a scripted reader heartbeat.
 * Only reader operations and wall-clock sleeps are replaced; writer lifecycle
 * and the restart/response decisions come from mp4_recording_core.c. */
#include "../../src/video/mp4_recording_core.c"
#include "unity.h"

static mp4_recording_ctx_t ctx;
static mp4_writer_thread_t fake_reader;
static char directory[128];
static const int *health_script;
static size_t script_length, checks;
static int starts, stops, responses, response_dead_seconds;
static bool fail_restart;

unsigned __wrap_sleep(unsigned seconds) {
    TEST_ASSERT_EQUAL_UINT(1, seconds);
    TEST_ASSERT_LESS_OR_EQUAL_UINT(script_length, checks);
    return 0;
}

int __wrap_mp4_writer_start_recording_thread(mp4_writer_t *writer, const char *url) {
    TEST_ASSERT_EQUAL_STRING("watchdog-recovery", writer->stream_name);
    TEST_ASSERT_EQUAL_STRING("rtsp://camera/live", url);
    starts++;
    if (starts > 1) TEST_ASSERT_EQUAL_INT(starts - 1, stops);
    if (starts > 1 && fail_restart) return -1;
    memset(&fake_reader, 0, sizeof(fake_reader));
    fake_reader.running = 1;
    fake_reader.writer = writer;
    writer->thread_ctx = &fake_reader;
    return 0;
}

void __wrap_mp4_writer_stop_recording_thread(mp4_writer_t *writer) {
    stops++;
    writer->thread_ctx = NULL;
}

int __wrap_mp4_writer_is_recording(mp4_writer_t *writer) {
    TEST_ASSERT_EQUAL_PTR(ctx.mp4_writer, writer);
    TEST_ASSERT_LESS_THAN_UINT(script_length, checks);
    int healthy = health_script[checks++];
    if (checks == script_length) ctx.running = 0;
    return healthy;
}

bool __wrap_go2rtc_integration_is_using_go2rtc_for_recording(const char *name) {
    (void)name;
    return false;
}

void __wrap__log_message_ctx(log_level_t level, const char *component,
                            const char *stream, const char *format, ...) {
    (void)level; (void)component; (void)stream;
    // Inspect the rendered message, not just whether a logging call happened.
    char message[1024];
    va_list args;
    va_start(args, format);
    vsnprintf(message, sizeof(message), format, args);
    va_end(args);
    if (strstr(message, "recording thread responsive after")) {
        responses++;
        TEST_ASSERT_NOT_NULL(strstr(message, "packet or retry activity"));
        TEST_ASSERT_NOT_NULL(strstr(message, "not confirmation of a completed segment"));
        TEST_ASSERT_EQUAL_INT(1, sscanf(message,
            "Stream watchdog-recovery: recording thread responsive after %d dead seconds",
            &response_dead_seconds));
    }
}

void setUp(void) {
    g_config.max_streams = MAX_STREAMS;
    g_config.audio_disabled = false;
    shutdown_in_progress = 0;
    memset(&ctx, 0, sizeof(ctx));
    strcpy(directory, "/tmp/lightnvr_watchdog_recovery_XXXXXX");
    TEST_ASSERT_NOT_NULL(mkdtemp(directory));
    strcpy(ctx.config.name, "watchdog-recovery");
    strcpy(ctx.config.url, "rtsp://camera/live");
    ctx.config.record_audio = true;
    ctx.config.segment_duration = 30;
    snprintf(ctx.output_path, sizeof(ctx.output_path), "%s/recording.mp4", directory);
    ctx.running = 1;
    starts = stops = responses = response_dead_seconds = 0;
    checks = 0;
    fail_restart = false;
}

void tearDown(void) {
    unregister_mp4_writer_for_stream(ctx.config.name);
    rmdir(directory);
}

static void run_script(const int *script, size_t count) {
    health_script = script;
    script_length = count;
    mp4_recording_thread(&ctx);
    TEST_ASSERT_EQUAL_UINT(count, checks);
    TEST_ASSERT_FALSE(ctx.running);
    TEST_ASSERT_NULL(ctx.mp4_writer);
    TEST_ASSERT_NULL(get_mp4_writer_for_stream(ctx.config.name));
}

static void test_brief_inactivity_reports_responsive_without_restart(void) {
    const int script[] = {0, 0, 1, 1};
    run_script(script, sizeof(script) / sizeof(script[0]));
    TEST_ASSERT_EQUAL_INT(1, starts);
    TEST_ASSERT_EQUAL_INT(1, stops); // final exit only
    TEST_ASSERT_EQUAL_INT(1, responses);
    TEST_ASSERT_EQUAL_INT(2, response_dead_seconds);
}

static void test_response_clears_dead_count_before_a_later_outage(void) {
    const int script[] = {0, 0, 0, 0, 1, 0, 0, 0, 0, 1};
    run_script(script, sizeof(script) / sizeof(script[0]));
    TEST_ASSERT_EQUAL_INT(1, starts);
    TEST_ASSERT_EQUAL_INT(1, stops);
    TEST_ASSERT_EQUAL_INT(2, responses);
    TEST_ASSERT_EQUAL_INT(4, response_dead_seconds);
}

static void test_sustained_outage_stops_before_restart_and_recovers(void) {
    const int script[] = {0, 0, 0, 0, 0, 0, 1};
    run_script(script, sizeof(script) / sizeof(script[0]));
    TEST_ASSERT_EQUAL_INT(2, starts);
    TEST_ASSERT_EQUAL_INT(2, stops);
    TEST_ASSERT_EQUAL_INT(1, responses);
    TEST_ASSERT_EQUAL_INT(1, response_dead_seconds);
}

static void test_failed_restart_cleans_up_writer_registration(void) {
    const int script[] = {0, 0, 0, 0, 0};
    fail_restart = true;
    run_script(script, sizeof(script) / sizeof(script[0]));
    TEST_ASSERT_EQUAL_INT(2, starts);
    TEST_ASSERT_EQUAL_INT(2, stops);
    TEST_ASSERT_EQUAL_INT(0, responses);
}

int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_brief_inactivity_reports_responsive_without_restart);
    RUN_TEST(test_response_clears_dead_count_before_a_later_outage);
    RUN_TEST(test_sustained_outage_stops_before_restart_and_recovers);
    RUN_TEST(test_failed_restart_cleans_up_writer_registration);
    return UNITY_END();
}
