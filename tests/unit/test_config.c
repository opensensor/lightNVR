/**
 * @file test_config.c
 * @brief Layer 2 unit tests — config loading and validation
 *
 * Tests load_default_config() for sane defaults and validate_config()
 * for rejection of invalid values (bad port, empty paths, bad buffer).
 * Links against lightnvr_lib so the logger is available for error paths.
 */

#define _POSIX_C_SOURCE 200809L
#define _GNU_SOURCE

#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>
#include <sys/stat.h>
#include <unistd.h>
#include <pthread.h>
#include <sched.h>
#include <stdatomic.h>
#include <errno.h>
#include <math.h>

#include "unity.h"
#include "core/config.h"
#include "core/logger.h"
#include "core/path_utils.h"

/* File-scope config used by most tests — setUp zeros it, tearDown frees
   the dynamically allocated streams array to keep ASan leak-free. */
static config_t cfg;
static bool deny_config_chmod;
static bool deny_config_rename;
static bool busy_config_rename;
static bool fail_config_pwrite;
static ino_t fail_config_pwrite_inode;
static bool monitor_secret_write_permissions;
static bool observed_config_stream;
static bool observed_config_stream_with_private_mode;
static char *saved_web_port_env;
static char *saved_audio_disabled_env;
static const char *const timeout_env_names[] = {
    "LIGHTNVR_WEBRTC_CONNECTION_TIMEOUT_MS",
    "LIGHTNVR_WEBRTC_ICE_RECOVERY_TIMEOUT_MS"
};
static char *saved_timeout_env[2];
int __real_fchmod(int, mode_t);
FILE *__real_fdopen(int, const char *);
int __wrap_fchmod(int fd, mode_t mode) {
    if (deny_config_chmod) {
        deny_config_chmod = false;
        errno = EPERM;
        return -1;
    }
    return __real_fchmod(fd, mode);
}
FILE *__wrap_fdopen(int fd, const char *mode) {
    if (monitor_secret_write_permissions) {
        observed_config_stream = true;
        struct stat st;
        bool this_stream_private =
            fstat(fd, &st) == 0 && (st.st_mode & 0777) == 0600;
        observed_config_stream_with_private_mode =
            observed_config_stream_with_private_mode && this_stream_private;
    }
    return __real_fdopen(fd, mode);
}
int __real_renameat(int, const char *, int, const char *);
int __wrap_renameat(int from_fd, const char *from, int to_fd, const char *to) {
    if (deny_config_rename) { errno = EACCES; return -1; }
    if (busy_config_rename) { busy_config_rename = false; errno = EBUSY; return -1; }
    return __real_renameat(from_fd, from, to_fd, to);
}
ssize_t __real_pwrite(int, const void *, size_t, off_t);
ssize_t __wrap_pwrite(int fd, const void *buf, size_t size, off_t offset) {
    struct stat st;
    if (fail_config_pwrite && fstat(fd, &st) == 0 && st.st_ino == fail_config_pwrite_inode) {
        fail_config_pwrite = false;
        errno = ENOSPC;
        return -1;
    }
    return __real_pwrite(fd, buf, size, offset);
}

/* ---- Unity boilerplate ---- */
void setUp(void) {
    memset(&cfg, 0, sizeof(cfg));
    observed_config_stream = false;
    observed_config_stream_with_private_mode = true;
    const char *web_port = getenv("LIGHTNVR_WEB_PORT");
    const char *audio_disabled = getenv("LIGHTNVR_AUDIO_DISABLED");
    saved_web_port_env = web_port ? strdup(web_port) : NULL;
    saved_audio_disabled_env = audio_disabled ? strdup(audio_disabled) : NULL;
    if (web_port) TEST_ASSERT_NOT_NULL(saved_web_port_env);
    if (audio_disabled) TEST_ASSERT_NOT_NULL(saved_audio_disabled_env);
    unsetenv("LIGHTNVR_WEB_PORT");
    unsetenv("LIGHTNVR_AUDIO_DISABLED");
    for (size_t i = 0; i < 2; i++) {
        const char *value = getenv(timeout_env_names[i]);
        saved_timeout_env[i] = value ? strdup(value) : NULL;
        if (value) TEST_ASSERT_NOT_NULL(saved_timeout_env[i]);
        unsetenv(timeout_env_names[i]);
    }
}
void tearDown(void) {
    deny_config_chmod = deny_config_rename = busy_config_rename = false;
    fail_config_pwrite = false;
    monitor_secret_write_permissions = false;
    observed_config_stream = false;
    observed_config_stream_with_private_mode = false;
    if (saved_web_port_env) setenv("LIGHTNVR_WEB_PORT", saved_web_port_env, 1);
    else unsetenv("LIGHTNVR_WEB_PORT");
    if (saved_audio_disabled_env) setenv("LIGHTNVR_AUDIO_DISABLED", saved_audio_disabled_env, 1);
    else unsetenv("LIGHTNVR_AUDIO_DISABLED");
    free(saved_web_port_env);
    free(saved_audio_disabled_env);
    saved_web_port_env = saved_audio_disabled_env = NULL;
    for (size_t i = 0; i < 2; i++) {
        if (saved_timeout_env[i]) setenv(timeout_env_names[i], saved_timeout_env[i], 1);
        else unsetenv(timeout_env_names[i]);
        free(saved_timeout_env[i]);
        saved_timeout_env[i] = NULL;
    }
    if (cfg.streams) {
        free(cfg.streams);
        cfg.streams = NULL;
    }
}

/* ================================================================
 * stream_config_t size sentinel (regression guard for T11 bump)
 *
 * Task T11 will raise `go2rtc_source_override` from 2048 to 8192 bytes
 * (+6144 per struct). This test documents the present size and asserts
 * that the struct stays below a 16 KB ceiling so that per-callsite
 * fan-out (stack arrays, worker-task structs) remains manageable on
 * musl/Alpine thread stacks (128 KB default).
 *
 * Expected sizes (x86_64, as of T11b):
 *   current                     ~= 6184 B
 *   projected after T11 bump   ~= 12328 B  (must remain < 16384)
 * ================================================================ */

void test_stream_config_t_size_under_16k(void) {
    /* Sentinel: struct must stay below 16 KB so that future T11 +6144-byte
       bump keeps call-site stack / task-struct cost predictable. */
    TEST_ASSERT_LESS_THAN(16 * 1024, (int)sizeof(stream_config_t));
}

void test_max_stream_ceiling_is_valid(void) {
    TEST_ASSERT_TRUE(MAX_STREAMS >= 1);
    TEST_ASSERT_TRUE(MAX_STREAMS <= 1024);
    TEST_ASSERT_EQUAL_INT(MAX_STREAMS < 32 ? MAX_STREAMS : 32,
                          DEFAULT_MAX_STREAMS);
}

/* ================================================================
 * load_default_config
 * ================================================================ */

void test_default_config_web_port(void) {
    load_default_config(&cfg);
    TEST_ASSERT_EQUAL_INT(8080, cfg.web_port);
}

void test_default_data_paths_stay_inside_persistent_volume(void) {
    load_default_config(&cfg);
    TEST_ASSERT_EQUAL_STRING("/var/lib/lightnvr/data/recordings", cfg.storage_path);
    TEST_ASSERT_EQUAL_STRING("/var/lib/lightnvr/data/recordings/mp4", cfg.mp4_storage_path);
    TEST_ASSERT_EQUAL_STRING("/var/lib/lightnvr/data/database/lightnvr.db", cfg.db_path);
    TEST_ASSERT_EQUAL_STRING("/var/lib/lightnvr/data/models", cfg.models_path);
    TEST_ASSERT_EQUAL_STRING("/var/lib/lightnvr/data/swap", cfg.swap_file);
}

void test_default_config_log_level(void) {
    load_default_config(&cfg);
    TEST_ASSERT_EQUAL_INT(LOG_LEVEL_INFO, cfg.log_level);
}

void test_default_config_retention_days(void) {
    load_default_config(&cfg);
    TEST_ASSERT_EQUAL_INT(30, cfg.retention_days);
}

void test_default_config_mp4_faststart_on(void) {
    load_default_config(&cfg);
    TEST_ASSERT_TRUE(config_get_mp4_faststart(&cfg));
}

/* save_config must write mp4_faststart, or a settings save from the web UI
 * would silently turn faststart back on at the next start. */
void test_save_config_round_trips_mp4_faststart(void) {
    char temp_dir[] = "/tmp/lightnvr_faststart_XXXXXX";
    char *dir = mkdtemp(temp_dir);
    TEST_ASSERT_NOT_NULL(dir);
    char config_path[MAX_PATH_LENGTH];
    snprintf(config_path, sizeof(config_path), "%s/faststart.ini", dir);

    load_default_config(&cfg);
    snprintf(cfg.pid_file, sizeof(cfg.pid_file), "%s/lightnvr.pid", dir);
    snprintf(cfg.log_file, sizeof(cfg.log_file), "%s/lightnvr.log", dir);
    snprintf(cfg.storage_path, sizeof(cfg.storage_path), "%s/recordings", dir);
    snprintf(cfg.mp4_storage_path, sizeof(cfg.mp4_storage_path), "%s/mp4", dir);
    snprintf(cfg.storage_path_hls, sizeof(cfg.storage_path_hls), "%s/hls", dir);
    snprintf(cfg.models_path, sizeof(cfg.models_path), "%s/models", dir);
    snprintf(cfg.db_path, sizeof(cfg.db_path), "%s/lightnvr.db", dir);
    snprintf(cfg.web_root, sizeof(cfg.web_root), "%s/web", dir);
    TEST_ASSERT_EQUAL_INT(0, ensure_dir(cfg.web_root));
    config_set_mp4_faststart(&cfg, false);
    TEST_ASSERT_EQUAL_INT(0, save_config(&cfg, config_path));

    config_t reloaded = {0};
    set_custom_config_path(config_path);
    TEST_ASSERT_EQUAL_INT(0, load_config(&reloaded));
    TEST_ASSERT_FALSE(config_get_mp4_faststart(&reloaded));
    free(reloaded.streams);

    unlink(config_path);
    rmdir(cfg.storage_path);
    rmdir(cfg.mp4_storage_path);
    rmdir(cfg.storage_path_hls);
    rmdir(cfg.models_path);
    rmdir(cfg.web_root);
    rmdir(dir);
}

static atomic_bool faststart_writer_stop;
static atomic_int faststart_writer_updates;

static void *toggle_faststart(void *unused) {
    (void)unused;
    bool enabled = false;
    while (!atomic_load(&faststart_writer_stop)) {
        config_set_mp4_faststart(&cfg, enabled);
        enabled = !enabled;
        atomic_fetch_add(&faststart_writer_updates, 1);
    }
    return NULL;
}

/* Exercise every shared read, including the whole-config snapshot that used
 * to bypass the option accessors. This also provides a ThreadSanitizer gate. */
void test_config_copy_owns_streams_after_source_reload(void) {
    load_default_config(&cfg);
    snprintf(cfg.streams[0].name, sizeof(cfg.streams[0].name), "original-camera");
    config_t snapshot = {0};
    config_copy(&snapshot, &cfg);
    TEST_ASSERT_NOT_EQUAL_PTR(cfg.streams, snapshot.streams);
    TEST_ASSERT_EQUAL_STRING("original-camera", snapshot.streams[0].name);
    load_default_config(&cfg);
    TEST_ASSERT_EQUAL_STRING("original-camera", snapshot.streams[0].name);
    free(snapshot.streams);
}

void test_mp4_faststart_concurrent_reads_copies_and_saves(void) {
    char temp_dir[] = "/tmp/lightnvr_faststart_concurrent_XXXXXX";
    char *dir = mkdtemp(temp_dir);
    TEST_ASSERT_NOT_NULL(dir);
    char path[MAX_PATH_LENGTH];
    snprintf(path, sizeof(path), "%s/settings.ini", dir);
    load_default_config(&cfg);

    atomic_store(&faststart_writer_stop, false);
    atomic_store(&faststart_writer_updates, 0);
    pthread_t writer;
    TEST_ASSERT_EQUAL_INT(0, pthread_create(&writer, NULL, toggle_faststart, NULL));
    int save_result = 0;
    int valid_saves = 0;
    bool streams_distinct_ok = true;
    while (atomic_load(&faststart_writer_updates) == 0) {
        sched_yield();
    }
    for (int i = 0; i < 50; ++i) {
        (void)config_get_mp4_faststart(&cfg);
        config_t snapshot = {0};
        config_copy(&snapshot, &cfg);
        (void)config_get_mp4_faststart(&snapshot);
        if (cfg.streams == snapshot.streams) {
            streams_distinct_ok = false;
        }
        free(snapshot.streams);
        save_result = save_config(&cfg, path);
        if (save_result != 0) break;
        FILE *file = fopen(path, "r");
        if (!file) break;
        char line[512];
        while (fgets(line, sizeof(line), file)) {
            if (strncmp(line, "mp4_faststart = true", strlen("mp4_faststart = true")) == 0 ||
                strncmp(line, "mp4_faststart = false", strlen("mp4_faststart = false")) == 0) {
                ++valid_saves;
            }
        }
        fclose(file);
    }
    atomic_store(&faststart_writer_stop, true);
    int join_result = pthread_join(writer, NULL);
    unlink(path);
    rmdir(dir);
    TEST_ASSERT_EQUAL_INT(0, join_result);
    TEST_ASSERT_TRUE(streams_distinct_ok);
    TEST_ASSERT_EQUAL_INT(0, save_result);
    TEST_ASSERT_EQUAL_INT(50, valid_saves);

    config_set_mp4_faststart(&cfg, false);
    config_t snapshot = {0};
    config_copy(&snapshot, &cfg);
    config_set_mp4_faststart(&cfg, true);
    TEST_ASSERT_FALSE(config_get_mp4_faststart(&snapshot));
    TEST_ASSERT_TRUE(config_get_mp4_faststart(&cfg));
    TEST_ASSERT_NOT_EQUAL_PTR(cfg.streams, snapshot.streams);
    TEST_ASSERT_EQUAL_INT(cfg.max_streams, snapshot.max_streams);
    TEST_ASSERT_EQUAL_MEMORY(cfg.streams, snapshot.streams,
                             (size_t)cfg.max_streams * sizeof(*cfg.streams));
    free(snapshot.streams);
}

void test_default_config_buffer_size(void) {
    load_default_config(&cfg);
    TEST_ASSERT_GREATER_THAN_INT(0, cfg.buffer_size);
}

void test_default_config_storage_path_nonempty(void) {
    load_default_config(&cfg);
    TEST_ASSERT_GREATER_THAN(0, (int)strlen(cfg.storage_path));
}

void test_default_config_db_path_nonempty(void) {
    load_default_config(&cfg);
    TEST_ASSERT_GREATER_THAN(0, (int)strlen(cfg.db_path));
}

void test_default_config_db_backup_settings(void) {
    load_default_config(&cfg);
    TEST_ASSERT_EQUAL_INT(0, cfg.db_backup_interval_minutes);
    /* Each retained backup is a full copy of the database, so this count
     * multiplies disk usage by the database size. */
    TEST_ASSERT_EQUAL_INT(6, cfg.db_backup_retention_count);
    TEST_ASSERT_EQUAL_STRING("", cfg.db_post_backup_script);
}

void test_default_config_db_startup_check_is_off(void) {
    load_default_config(&cfg);
    /* Even quick_check scans every database page before the HTTP listener
     * binds, so consistency checks are explicit maintenance choices. */
    TEST_ASSERT_EQUAL_INT(DB_STARTUP_CHECK_OFF, cfg.db_startup_check);
}

void test_default_config_db_backup_verify_is_full(void) {
    load_default_config(&cfg);
    /* Operators opt into the cheaper quick_check (or none) for large databases
     * on slow disks; the default keeps the historical full integrity_check. */
    TEST_ASSERT_EQUAL_INT(DB_BACKUP_VERIFY_FULL, cfg.db_backup_verify);
}

void test_validate_config_clamps_out_of_range_backup_verify(void) {
    load_default_config(&cfg);
    cfg.db_backup_verify = 99;
    TEST_ASSERT_EQUAL_INT(0, validate_config(&cfg));
    TEST_ASSERT_EQUAL_INT(DB_BACKUP_VERIFY_FULL, cfg.db_backup_verify);

    load_default_config(&cfg);
    cfg.db_backup_verify = -1;
    TEST_ASSERT_EQUAL_INT(0, validate_config(&cfg));
    TEST_ASSERT_EQUAL_INT(DB_BACKUP_VERIFY_FULL, cfg.db_backup_verify);

    load_default_config(&cfg);
    cfg.db_backup_verify = DB_BACKUP_VERIFY_QUICK;
    TEST_ASSERT_EQUAL_INT(0, validate_config(&cfg));
    TEST_ASSERT_EQUAL_INT(DB_BACKUP_VERIFY_QUICK, cfg.db_backup_verify);
}

void test_default_config_health_matches_ops03(void) {
    load_default_config(&cfg);
    TEST_ASSERT_TRUE(cfg.health.enabled);
    TEST_ASSERT_EQUAL_STRING("balanced", cfg.health.profile);
    TEST_ASSERT_EQUAL_UINT32(10U, cfg.health.fast_interval_seconds);
    TEST_ASSERT_EQUAL_UINT32(60U, cfg.health.normal_interval_seconds);
    TEST_ASSERT_EQUAL_UINT32(300U, cfg.health.slow_interval_seconds);
    TEST_ASSERT_EQUAL_UINT32(900U, cfg.health.device_interval_seconds);
    TEST_ASSERT_TRUE(cfg.health.write_probe_enabled);
    TEST_ASSERT_EQUAL_STRING("auto", cfg.health.hardware_provider);
    TEST_ASSERT_EQUAL_UINT32(60U, cfg.health.presence_interval_seconds);
    TEST_ASSERT_EQUAL_UINT32(90U, cfg.health.incident_retention_days);
}

void test_validate_config_rejects_unsafe_health_values(void) {
    load_default_config(&cfg);
    cfg.health.fast_interval_seconds = 1U;
    TEST_ASSERT_NOT_EQUAL(0, validate_config(&cfg));
    TEST_ASSERT_EQUAL_UINT32(1U, cfg.health.fast_interval_seconds);

    load_default_config(&cfg);
    snprintf(cfg.health.hardware_provider,
             sizeof(cfg.health.hardware_provider), "%s", "/bin/x");
    TEST_ASSERT_NOT_EQUAL(0, validate_config(&cfg));
}

void test_validate_config_resets_nonfinite_storage_pressure(void) {
    const double invalid[] = {NAN, INFINITY, -INFINITY};
    for (size_t i = 0; i < sizeof(invalid) / sizeof(invalid[0]); i++) {
        load_default_config(&cfg);
        cfg.storage_pressure_emergency_pct = invalid[i];
        TEST_ASSERT_EQUAL_INT(0, validate_config(&cfg));
        TEST_ASSERT_TRUE(cfg.storage_pressure_emergency_pct == 5.0);
        TEST_ASSERT_TRUE(cfg.storage_pressure_critical_pct == 10.0);
        TEST_ASSERT_TRUE(cfg.storage_pressure_warning_pct == 20.0);
    }

    for (size_t i = 0; i < sizeof(invalid) / sizeof(invalid[0]); i++) {
        load_default_config(&cfg);
        cfg.storage_pressure_critical_pct = invalid[i];
        TEST_ASSERT_EQUAL_INT(0, validate_config(&cfg));
        TEST_ASSERT_TRUE(cfg.storage_pressure_emergency_pct == 5.0);
        TEST_ASSERT_TRUE(cfg.storage_pressure_critical_pct == 10.0);
        TEST_ASSERT_TRUE(cfg.storage_pressure_warning_pct == 20.0);
    }

    for (size_t i = 0; i < sizeof(invalid) / sizeof(invalid[0]); i++) {
        load_default_config(&cfg);
        cfg.storage_pressure_warning_pct = invalid[i];
        TEST_ASSERT_EQUAL_INT(0, validate_config(&cfg));
        TEST_ASSERT_TRUE(cfg.storage_pressure_emergency_pct == 5.0);
        TEST_ASSERT_TRUE(cfg.storage_pressure_critical_pct == 10.0);
        TEST_ASSERT_TRUE(cfg.storage_pressure_warning_pct == 20.0);
    }
}

void test_malformed_ini_after_stream_resize_keeps_streams_owned(void) {
    if (DEFAULT_MAX_STREAMS >= MAX_STREAMS) {
        TEST_IGNORE_MESSAGE("Resize-specific test skipped: DEFAULT_MAX_STREAMS is already at MAX_STREAMS.");
        return;
    }

    const int resize_target = DEFAULT_MAX_STREAMS + 1;
    char temp_dir[] = "/tmp/lightnvr_bad_ini_XXXXXX";
    char *dir = mkdtemp(temp_dir);
    TEST_ASSERT_NOT_NULL(dir);
    char config_path[MAX_PATH_LENGTH];
    snprintf(config_path, sizeof(config_path), "%s/bad.ini", dir);
    FILE *file = fopen(config_path, "w");
    TEST_ASSERT_NOT_NULL(file);
    char ini_contents[128];
    snprintf(ini_contents, sizeof(ini_contents),
             "[streams]\nmax_streams = %d\ninvalid line\n", resize_target);
    fputs(ini_contents, file);
    fclose(file);

    set_custom_config_path(config_path);
    TEST_ASSERT_EQUAL_INT(-1, load_config(&cfg));
    TEST_ASSERT_EQUAL_INT(DEFAULT_MAX_STREAMS, cfg.max_streams);
    TEST_ASSERT_NOT_NULL(cfg.streams);
    TEST_ASSERT_TRUE(cfg.streams[DEFAULT_MAX_STREAMS - 1].streaming_enabled);
    cfg.streams[0].name[0] = '\0';

    unlink(config_path);
    rmdir(dir);
}

void test_validate_config_clamps_out_of_range_startup_check(void) {
    load_default_config(&cfg);
    cfg.db_startup_check = 99;
    TEST_ASSERT_EQUAL_INT(0, validate_config(&cfg));
    TEST_ASSERT_EQUAL_INT(DB_STARTUP_CHECK_OFF, cfg.db_startup_check);

    load_default_config(&cfg);
    cfg.db_startup_check = -1;
    TEST_ASSERT_EQUAL_INT(0, validate_config(&cfg));
    TEST_ASSERT_EQUAL_INT(DB_STARTUP_CHECK_OFF, cfg.db_startup_check);
}

void test_validate_config_preserves_valid_startup_check(void) {
    load_default_config(&cfg);
    cfg.db_startup_check = DB_STARTUP_CHECK_FULL;
    TEST_ASSERT_EQUAL_INT(0, validate_config(&cfg));
    TEST_ASSERT_EQUAL_INT(DB_STARTUP_CHECK_FULL, cfg.db_startup_check);

    load_default_config(&cfg);
    cfg.db_startup_check = DB_STARTUP_CHECK_OFF;
    TEST_ASSERT_EQUAL_INT(0, validate_config(&cfg));
    TEST_ASSERT_EQUAL_INT(DB_STARTUP_CHECK_OFF, cfg.db_startup_check);
}

void test_default_config_models_path_nonempty(void) {
    load_default_config(&cfg);
    TEST_ASSERT_GREATER_THAN(0, (int)strlen(cfg.models_path));
}

void test_default_config_null_safe(void) {
    /* Should not crash when passed NULL */
    load_default_config(NULL);
    TEST_PASS();
}

/* ================================================================
 * validate_config
 * ================================================================ */

void test_validate_config_valid_defaults(void) {
    load_default_config(&cfg);
    TEST_ASSERT_EQUAL_INT(0, validate_config(&cfg));
}

void test_validate_config_null(void) {
    TEST_ASSERT_NOT_EQUAL(0, validate_config(NULL));
}

void test_validate_config_empty_storage_path(void) {
    load_default_config(&cfg);
    cfg.storage_path[0] = '\0';
    TEST_ASSERT_NOT_EQUAL(0, validate_config(&cfg));
}

void test_validate_config_empty_models_path(void) {
    load_default_config(&cfg);
    cfg.models_path[0] = '\0';
    TEST_ASSERT_NOT_EQUAL(0, validate_config(&cfg));
}

void test_validate_config_empty_db_path(void) {
    load_default_config(&cfg);
    cfg.db_path[0] = '\0';
    TEST_ASSERT_NOT_EQUAL(0, validate_config(&cfg));
}

void test_validate_config_empty_web_root(void) {
    load_default_config(&cfg);
    cfg.web_root[0] = '\0';
    TEST_ASSERT_NOT_EQUAL(0, validate_config(&cfg));
}

void test_validate_config_port_zero(void) {
    load_default_config(&cfg);
    cfg.web_port = 0;
    TEST_ASSERT_NOT_EQUAL(0, validate_config(&cfg));
}

void test_validate_config_port_too_high(void) {
    load_default_config(&cfg);
    cfg.web_port = 99999;
    TEST_ASSERT_NOT_EQUAL(0, validate_config(&cfg));
}

void test_validate_config_port_max_valid(void) {
    load_default_config(&cfg);
    cfg.web_port = 65535;
    TEST_ASSERT_EQUAL_INT(0, validate_config(&cfg));
}

void test_validate_config_port_min_valid(void) {
    load_default_config(&cfg);
    cfg.web_port = 1;
    TEST_ASSERT_EQUAL_INT(0, validate_config(&cfg));
}

void test_validate_config_buffer_size_zero(void) {
    load_default_config(&cfg);
    cfg.buffer_size = 0;
    TEST_ASSERT_NOT_EQUAL(0, validate_config(&cfg));
}

void test_validate_config_clamps_absolute_timeout_to_idle_timeout(void) {
    load_default_config(&cfg);
    cfg.auth_timeout_hours = 48;
    cfg.auth_absolute_timeout_hours = 12;

    TEST_ASSERT_EQUAL_INT(0, validate_config(&cfg));
    TEST_ASSERT_EQUAL_INT(48, cfg.auth_absolute_timeout_hours);
}

void test_validate_config_clamps_negative_db_backup_values(void) {
    load_default_config(&cfg);
    cfg.db_backup_interval_minutes = -5;
    cfg.db_backup_retention_count = -3;

    TEST_ASSERT_EQUAL_INT(0, validate_config(&cfg));
    TEST_ASSERT_EQUAL_INT(0, cfg.db_backup_interval_minutes);
    TEST_ASSERT_EQUAL_INT(0, cfg.db_backup_retention_count);
}

/* ================================================================
 * additional default field checks
 * ================================================================ */

void test_default_config_web_auth_enabled(void) {
    load_default_config(&cfg);
    TEST_ASSERT_TRUE(cfg.web_auth_enabled);
}

void test_default_config_username(void) {
    load_default_config(&cfg);
    TEST_ASSERT_EQUAL_STRING("admin", cfg.web_username);
}

void test_default_config_syslog_disabled(void) {
    load_default_config(&cfg);
    TEST_ASSERT_FALSE(cfg.syslog_enabled);
}

void test_default_config_go2rtc_enabled(void) {
    load_default_config(&cfg);
    TEST_ASSERT_TRUE(cfg.go2rtc_enabled);
}

void test_default_config_go2rtc_api_port(void) {
    load_default_config(&cfg);
    TEST_ASSERT_EQUAL_INT(1984, cfg.go2rtc_api_port);
}

void test_default_config_go2rtc_webrtc_enabled(void) {
    load_default_config(&cfg);
    TEST_ASSERT_TRUE(cfg.go2rtc_webrtc_enabled);
}

void test_default_config_go2rtc_stun_enabled(void) {
    load_default_config(&cfg);
    TEST_ASSERT_TRUE(cfg.go2rtc_stun_enabled);
}

void test_default_config_turn_disabled(void) {
    load_default_config(&cfg);
    TEST_ASSERT_FALSE(cfg.turn_enabled);
}

void test_default_config_mqtt_disabled(void) {
    load_default_config(&cfg);
    TEST_ASSERT_FALSE(cfg.mqtt_enabled);
}

void test_default_config_mqtt_port(void) {
    load_default_config(&cfg);
    TEST_ASSERT_EQUAL_INT(1883, cfg.mqtt_broker_port);
}

void test_default_config_mp4_segment_duration(void) {
    load_default_config(&cfg);
    TEST_ASSERT_EQUAL_INT(900, cfg.mp4_segment_duration);
}

void test_default_config_mp4_directory_format(void) {
    load_default_config(&cfg);
    TEST_ASSERT_EQUAL_STRING("year_month_day", cfg.mp4_directory_format);
}

void test_default_config_stream_defaults(void) {
    load_default_config(&cfg);
    TEST_ASSERT_TRUE(cfg.max_streams > 0);
    /* All streams should default to streaming enabled, no detection */
    for (int i = 0; i < cfg.max_streams; i++) {
        TEST_ASSERT_TRUE(cfg.streams[i].streaming_enabled);
        TEST_ASSERT_FALSE(cfg.streams[i].detection_based_recording);
    }
}

void test_default_config_auth_timeout(void) {
    load_default_config(&cfg);
    TEST_ASSERT_EQUAL_INT(24, cfg.auth_timeout_hours);
}

void test_default_config_auto_view_enabled(void) {
    load_default_config(&cfg);
    TEST_ASSERT_FALSE(cfg.auto_disabled);
}

void test_default_config_audio_enabled(void) {
    load_default_config(&cfg);
    TEST_ASSERT_FALSE(cfg.audio_disabled);
}

void test_default_config_web_compression_enabled(void) {
    load_default_config(&cfg);
    TEST_ASSERT_TRUE(cfg.web_compression_enabled);
}

/* ================================================================
 * validate_config — swap_size zero with use_swap true
 * ================================================================ */

void test_validate_config_swap_size_zero_with_use_swap(void) {
    load_default_config(&cfg);
    cfg.use_swap = true;
    cfg.swap_size = 0;
    TEST_ASSERT_NOT_EQUAL(0, validate_config(&cfg));
}

void test_validate_config_swap_disabled_size_zero_ok(void) {
    load_default_config(&cfg);
    cfg.use_swap = false;
    cfg.swap_size = 0;
    /* Swap size check only applies when use_swap is true */
    TEST_ASSERT_EQUAL_INT(0, validate_config(&cfg));
}

/* ================================================================
 * set_custom_config_path / get_custom_config_path
 * ================================================================ */

void test_custom_config_path_empty_not_stored(void) {
    /* set_custom_config_path ignores empty strings (does not overwrite). */
    /* At the start of the test binary, g_custom_config_path is zeroed.   */
    /* This test must run before any test that sets a non-empty path.     */
    set_custom_config_path("");
    /* getter returns NULL when nothing has been stored yet */
    const char *path = get_custom_config_path();
    TEST_ASSERT_NULL(path);
}

void test_custom_config_path_null_not_stored(void) {
    /* NULL is also silently ignored — must not crash and must not store. */
    set_custom_config_path(NULL);
    const char *path = get_custom_config_path();
    TEST_ASSERT_NULL(path);
}

void test_custom_config_path_roundtrip(void) {
    /* Run AFTER the empty/null tests so that g_custom_config_path is still
       empty when those tests run.  Here we confirm a valid path is stored. */
    set_custom_config_path("/tmp/test_lightnvr.ini");
    const char *path = get_custom_config_path();
    TEST_ASSERT_NOT_NULL(path);
    TEST_ASSERT_EQUAL_STRING("/tmp/test_lightnvr.ini", path);
}

/* ================================================================
 * get_loaded_config_path — initially NULL (no file loaded yet)
 * ================================================================ */

void test_get_loaded_config_path_initially(void) {
    /* Without calling load_config(), the loaded path should be NULL */
    const char *path = get_loaded_config_path();
    TEST_ASSERT_NULL(path);
}

void test_save_config_accepts_hidden_ini_dotfile(void) {
    char temp_dir[] = "/tmp/lightnvr_save_config_XXXXXX";
    char *dir = mkdtemp(temp_dir);
    TEST_ASSERT_NOT_NULL(dir);

    char config_path[MAX_PATH_LENGTH];
    snprintf(config_path, sizeof(config_path), "%s/.ini", dir);

    load_default_config(&cfg);
    cfg.db_backup_interval_minutes = 15;
    cfg.db_backup_retention_count = 8;
    cfg.audio_disabled = true;
    cfg.auto_disabled = true;
    snprintf(cfg.health.profile, sizeof(cfg.health.profile), "%s",
             "conservative");
    cfg.health.presence_interval_seconds = 120U;
    snprintf(cfg.db_post_backup_script, sizeof(cfg.db_post_backup_script), "/usr/local/bin/post-backup");
    TEST_ASSERT_EQUAL_INT(0, save_config(&cfg, config_path));

    FILE *saved = fopen(config_path, "r");
    TEST_ASSERT_NOT_NULL(saved);
    if (saved) {
        TEST_ASSERT_EQUAL_INT(0, fseek(saved, 0, SEEK_END));
        long file_size = ftell(saved);
        TEST_ASSERT_GREATER_THAN(0, file_size);
        rewind(saved);

        char *file_buffer = malloc((size_t)file_size + 1U);
        TEST_ASSERT_NOT_NULL(file_buffer);
        if (file_buffer) {
            size_t bytes_read = fread(file_buffer, 1, (size_t)file_size, saved);
            TEST_ASSERT_EQUAL_size_t((size_t)file_size, bytes_read);
            file_buffer[bytes_read] = '\0';
            TEST_ASSERT_NOT_NULL(strstr(file_buffer, "; LightNVR Configuration File"));
            TEST_ASSERT_NOT_NULL(strstr(file_buffer, "backup_interval_minutes = 15"));
            TEST_ASSERT_NOT_NULL(strstr(file_buffer, "backup_retention_count = 8"));
            TEST_ASSERT_NOT_NULL(strstr(file_buffer, "post_backup_script = /usr/local/bin/post-backup"));
            TEST_ASSERT_NOT_NULL(strstr(file_buffer, "mp4_directory_format = year_month_day"));
            TEST_ASSERT_NOT_NULL(strstr(file_buffer, "audio_disabled = true"));
            TEST_ASSERT_NOT_NULL(strstr(file_buffer, "auto_disabled = true"));
            TEST_ASSERT_NOT_NULL(strstr(file_buffer, "[health]"));
            TEST_ASSERT_NOT_NULL(strstr(file_buffer, "profile = conservative"));
            TEST_ASSERT_NOT_NULL(strstr(file_buffer,
                                        "presence_interval_seconds = 120"));
            free(file_buffer);
        }
        fclose(saved);
    }

    unlink(config_path);
    rmdir(dir);
}

void test_default_config_api_detection_settings(void) {
    load_default_config(&cfg);
    TEST_ASSERT_EQUAL_STRING("http://localhost:8000/api/v1/detect", cfg.api_detection_url);
    TEST_ASSERT_EQUAL_STRING("onnx", cfg.api_detection_backend);
    TEST_ASSERT_EQUAL_STRING("light-object-detect", cfg.api_detection_format);
    TEST_ASSERT_EQUAL_STRING("default", cfg.api_detection_detector_name);
}

void test_config_set_api_detection_format_normalizes_and_rejects(void) {
    load_default_config(&cfg);

    TEST_ASSERT_TRUE(config_set_api_detection_format(&cfg, "DOODS"));
    TEST_ASSERT_EQUAL_STRING("doods2", cfg.api_detection_format);

    /* Unknown names are rejected and the previous value is kept. */
    TEST_ASSERT_FALSE(config_set_api_detection_format(&cfg, "deepstack"));
    TEST_ASSERT_EQUAL_STRING("doods2", cfg.api_detection_format);
    TEST_ASSERT_FALSE(config_set_api_detection_format(&cfg, ""));
    TEST_ASSERT_FALSE(config_set_api_detection_format(NULL, "doods2"));

    TEST_ASSERT_TRUE(config_set_api_detection_format(&cfg, " Light-Object-Detect "));
    TEST_ASSERT_EQUAL_STRING("light-object-detect", cfg.api_detection_format);
}

void test_save_config_writes_api_detection_format(void) {
    char temp_dir[] = "/tmp/lightnvr_save_config_XXXXXX";
    char *dir = mkdtemp(temp_dir);
    TEST_ASSERT_NOT_NULL(dir);

    char config_path[MAX_PATH_LENGTH];
    snprintf(config_path, sizeof(config_path), "%s/lightnvr.ini", dir);

    load_default_config(&cfg);
    TEST_ASSERT_TRUE(config_set_api_detection_format(&cfg, "doods2"));
    snprintf(cfg.api_detection_detector_name, sizeof(cfg.api_detection_detector_name), "tensorflow");
    TEST_ASSERT_EQUAL_INT(0, save_config(&cfg, config_path));

    FILE *saved = fopen(config_path, "r");
    TEST_ASSERT_NOT_NULL(saved);
    if (saved) {
        char file_buffer[8192] = {0};
        size_t bytes_read = fread(file_buffer, 1, sizeof(file_buffer) - 1, saved);
        TEST_ASSERT_GREATER_THAN(0, (int)bytes_read);
        file_buffer[bytes_read] = '\0';
        TEST_ASSERT_NOT_NULL(strstr(file_buffer, "[api_detection]"));
        TEST_ASSERT_NOT_NULL(strstr(file_buffer, "format = doods2"));
        TEST_ASSERT_NOT_NULL(strstr(file_buffer, "detector_name = tensorflow"));
        fclose(saved);
    }

    unlink(config_path);
    rmdir(dir);
}

void test_load_config_parses_api_detection_format(void) {
    char temp_dir[] = "/tmp/lightnvr_load_config_XXXXXX";
    char *dir = mkdtemp(temp_dir);
    TEST_ASSERT_NOT_NULL(dir);

    char config_path[MAX_PATH_LENGTH];
    char web_root[MAX_PATH_LENGTH];
    snprintf(config_path, sizeof(config_path), "%s/test.ini", dir);
    snprintf(web_root, sizeof(web_root), "%s/web", dir);
    TEST_ASSERT_EQUAL_INT(0, ensure_dir(web_root));

    FILE *config_file = fopen(config_path, "w");
    TEST_ASSERT_NOT_NULL(config_file);
    fprintf(config_file,
            "[general]\n"
            "pid_file = %s/lightnvr.pid\n"
            "log_file = %s/lightnvr.log\n\n"
            "[storage]\n"
            "path = %s/storage\n"
            "path_hls = %s/hls\n\n"
            "[models]\n"
            "path = %s/models\n\n"
            "[database]\n"
            "path = %s/lightnvr.db\n\n"
            "[web]\n"
            "root = %s\n\n"
            "[api_detection]\n"
            "url = http://doods:8080/detect\n"
            "format = DOODS2\n"
            "detector_name = tensorflow\n",
            dir, dir, dir, dir, dir, dir, web_root);
    fclose(config_file);

    set_custom_config_path(config_path);
    TEST_ASSERT_EQUAL_INT(0, load_config(&cfg));
    TEST_ASSERT_EQUAL_STRING("http://doods:8080/detect", cfg.api_detection_url);
    TEST_ASSERT_EQUAL_STRING("doods2", cfg.api_detection_format);
    TEST_ASSERT_EQUAL_STRING("tensorflow", cfg.api_detection_detector_name);

    /* An unknown format keeps the default rather than poisoning later requests. */
    config_file = fopen(config_path, "w");
    TEST_ASSERT_NOT_NULL(config_file);
    fprintf(config_file,
            "[general]\n"
            "pid_file = %s/lightnvr.pid\n"
            "log_file = %s/lightnvr.log\n\n"
            "[storage]\n"
            "path = %s/storage\n"
            "path_hls = %s/hls\n\n"
            "[models]\n"
            "path = %s/models\n\n"
            "[database]\n"
            "path = %s/lightnvr.db\n\n"
            "[web]\n"
            "root = %s\n\n"
            "[api_detection]\n"
            "format = deepstack\n",
            dir, dir, dir, dir, dir, dir, web_root);
    fclose(config_file);
    TEST_ASSERT_EQUAL_INT(0, load_config(&cfg));
    TEST_ASSERT_EQUAL_STRING("light-object-detect", cfg.api_detection_format);
    TEST_ASSERT_EQUAL_STRING("default", cfg.api_detection_detector_name);

    unlink(config_path);
    rmdir(web_root);
    rmdir(dir);
}

void test_env_integer_whitespace_handling(void) {
    char temp_dir[] = "/tmp/lightnvr_load_config_XXXXXX";
    char *dir = mkdtemp(temp_dir);
    TEST_ASSERT_NOT_NULL(dir);

    char config_path[MAX_PATH_LENGTH];
    char storage_path[MAX_PATH_LENGTH];
    char storage_hls_path[MAX_PATH_LENGTH];
    char models_path[MAX_PATH_LENGTH];
    char db_path[MAX_PATH_LENGTH];
    char web_root[MAX_PATH_LENGTH];
    char log_path[MAX_PATH_LENGTH];
    char pid_path[MAX_PATH_LENGTH];

    snprintf(config_path, sizeof(config_path), "%s/test.ini", dir);
    snprintf(storage_path, sizeof(storage_path), "%s/storage", dir);
    snprintf(storage_hls_path, sizeof(storage_hls_path), "%s/hls", dir);
    snprintf(models_path, sizeof(models_path), "%s/models", dir);
    snprintf(db_path, sizeof(db_path), "%s/lightnvr.db", dir);
    snprintf(web_root, sizeof(web_root), "%s/web", dir);
    snprintf(log_path, sizeof(log_path), "%s/lightnvr.log", dir);
    snprintf(pid_path, sizeof(pid_path), "%s/lightnvr.pid", dir);
    const int expanded_max = DEFAULT_MAX_STREAMS < MAX_STREAMS
        ? DEFAULT_MAX_STREAMS + 1 : DEFAULT_MAX_STREAMS;

    FILE *config_file = fopen(config_path, "w");
    TEST_ASSERT_NOT_NULL(config_file);
    fprintf(config_file,
            "[general]\n"
            "pid_file = %s\n"
            "log_file = %s\n\n"
            "[storage]\n"
            "path = %s\n"
            "path_hls = %s\n"
            "mp4_directory_format = year_month\n"
            "mp4_faststart = false\n\n"
            "[models]\n"
            "path = %s\n\n"
            "[database]\n"
            "path = %s\n"
            "backup_interval_minutes = 45\n"
            "backup_retention_count = 12\n"
            "post_backup_script = /usr/local/bin/backup-hook\n\n"
            "[health]\n"
            "enabled = true\n"
            "profile = conservative\n"
            "fast_interval_seconds = 15\n"
            "normal_interval_seconds = 90\n"
            "slow_interval_seconds = 360\n"
            "device_interval_seconds = 1200\n"
            "write_probe_enabled = false\n"
            "hardware_provider = disabled\n"
            "presence_interval_seconds = 120\n"
            "incident_retention_days = 120\n\n"
            "[web]\n"
            "root = %s\n"
            "audio_disabled = false\n"
            "auto_disabled = true\n\n"
            "[streams]\n"
            "max_streams = %d\n",
            pid_path, log_path, storage_path, storage_hls_path,
            models_path, db_path, web_root, expanded_max);
    fclose(config_file);

    TEST_ASSERT_EQUAL_INT(0, ensure_dir(web_root));

    set_custom_config_path(config_path);
    /* Trailing whitespace should be ignored. */
    TEST_ASSERT_EQUAL_INT(0, setenv("LIGHTNVR_WEB_PORT", "9099   ", 1));
    TEST_ASSERT_EQUAL_INT(0, setenv("LIGHTNVR_AUDIO_DISABLED", "true", 1));
    TEST_ASSERT_EQUAL_INT(0, load_config(&cfg));
    TEST_ASSERT_EQUAL_INT(9099, cfg.web_port);
    TEST_ASSERT_EQUAL_INT(expanded_max, cfg.max_streams);
    TEST_ASSERT_TRUE(cfg.streams[expanded_max - 1].streaming_enabled);
    TEST_ASSERT_EQUAL_INT(10, cfg.streams[expanded_max - 1].detection_interval);
    TEST_ASSERT_EQUAL_STRING("none", cfg.streams[expanded_max - 1].detection_object_filter);
    TEST_ASSERT_TRUE(cfg.streams[expanded_max - 1].tier_critical_multiplier == 3.0);
    TEST_ASSERT_EQUAL_INT(45, cfg.db_backup_interval_minutes);
    TEST_ASSERT_EQUAL_INT(12, cfg.db_backup_retention_count);
    TEST_ASSERT_EQUAL_STRING("/usr/local/bin/backup-hook", cfg.db_post_backup_script);
    TEST_ASSERT_EQUAL_STRING("year_month", cfg.mp4_directory_format);
    TEST_ASSERT_FALSE(config_get_mp4_faststart(&cfg));
    /* An explicit INI false wins even though it equals the default. */
    TEST_ASSERT_FALSE(cfg.audio_disabled);
    TEST_ASSERT_TRUE(cfg.auto_disabled);
    TEST_ASSERT_EQUAL_STRING("conservative", cfg.health.profile);
    TEST_ASSERT_EQUAL_UINT32(15U, cfg.health.fast_interval_seconds);
    TEST_ASSERT_EQUAL_UINT32(90U, cfg.health.normal_interval_seconds);
    TEST_ASSERT_EQUAL_UINT32(360U, cfg.health.slow_interval_seconds);
    TEST_ASSERT_EQUAL_UINT32(1200U, cfg.health.device_interval_seconds);
    TEST_ASSERT_FALSE(cfg.health.write_probe_enabled);
    TEST_ASSERT_EQUAL_STRING("disabled", cfg.health.hardware_provider);
    TEST_ASSERT_EQUAL_UINT32(120U, cfg.health.presence_interval_seconds);
    TEST_ASSERT_EQUAL_UINT32(120U, cfg.health.incident_retention_days);

    /* Leading whitespace should be ignored. */
    TEST_ASSERT_EQUAL_INT(0, unsetenv("LIGHTNVR_AUDIO_DISABLED"));
    TEST_ASSERT_EQUAL_INT(0, setenv("LIGHTNVR_WEB_PORT", "   9099", 1));
    TEST_ASSERT_EQUAL_INT(0, load_config(&cfg));
    TEST_ASSERT_EQUAL_INT(9099, cfg.web_port);
    TEST_ASSERT_FALSE(cfg.audio_disabled);

    /* Mixed leading and trailing whitespace should be ignored. */
    TEST_ASSERT_EQUAL_INT(0, setenv("LIGHTNVR_WEB_PORT", "   9099   ", 1));
    TEST_ASSERT_EQUAL_INT(0, load_config(&cfg));
    TEST_ASSERT_EQUAL_INT(9099, cfg.web_port);

    TEST_ASSERT_EQUAL_INT(0, setenv("LIGHTNVR_WEB_PORT", "70000", 1));
    TEST_ASSERT_EQUAL_INT(0, load_config(&cfg));
    TEST_ASSERT_EQUAL_INT(8080, cfg.web_port);

    TEST_ASSERT_EQUAL_INT(0, setenv(timeout_env_names[0], "-1", 1));
    TEST_ASSERT_EQUAL_INT(0, setenv(timeout_env_names[1], "-1", 1));
    TEST_ASSERT_EQUAL_INT(0, load_config(&cfg));
    TEST_ASSERT_EQUAL_INT(30000, cfg.webrtc_connection_timeout_ms);
    TEST_ASSERT_EQUAL_INT(5000, cfg.webrtc_ice_recovery_timeout_ms);
    TEST_ASSERT_EQUAL_INT(0, setenv(timeout_env_names[1], "0", 1));
    TEST_ASSERT_EQUAL_INT(0, load_config(&cfg));
    TEST_ASSERT_EQUAL_INT(0, cfg.webrtc_ice_recovery_timeout_ms);

    unlink(config_path);
    unlink(db_path);
    unlink(log_path);
    rmdir(storage_hls_path);
    rmdir(storage_path);
    rmdir(models_path);
    rmdir(web_root);
    rmdir(dir);
}

/* ================================================================
 * main
 * ================================================================ */

void test_save_config_tightens_existing_permissions_before_writing_secrets(void) {
    char directory[] = "/tmp/lightnvr_private_config_XXXXXX";
    TEST_ASSERT_NOT_NULL(mkdtemp(directory));
    char path[MAX_PATH_LENGTH];
    snprintf(path, sizeof(path), "%s/settings.ini", directory);
    FILE *old = fopen(path, "w");
    TEST_ASSERT_NOT_NULL(old);
    fputs("old content", old); fclose(old);
    TEST_ASSERT_EQUAL_INT(0, chmod(path, 0666));
    load_default_config(&cfg);
    snprintf(cfg.mqtt_password, sizeof(cfg.mqtt_password), "synthetic-private-password");
    monitor_secret_write_permissions = true;
    observed_config_stream = false;
    observed_config_stream_with_private_mode = true;
    mode_t previous = umask(0);
    int saved = save_config(&cfg, path);
    umask(previous);
    monitor_secret_write_permissions = false;
    struct stat info;
    TEST_ASSERT_EQUAL_INT(0, saved);
    TEST_ASSERT_TRUE(observed_config_stream);
    TEST_ASSERT_TRUE(observed_config_stream_with_private_mode);
    TEST_ASSERT_EQUAL_INT(0, stat(path, &info));
    TEST_ASSERT_EQUAL_UINT(0600, info.st_mode & 0777);
    FILE *saved_file = fopen(path, "r");
    TEST_ASSERT_NOT_NULL(saved_file);
    char saved_text[16384];
    size_t saved_size = fread(saved_text, 1, sizeof(saved_text) - 1, saved_file);
    saved_text[saved_size] = '\0';
    fclose(saved_file);
    TEST_ASSERT_NOT_NULL(strstr(saved_text, "synthetic-private-password"));
    unlink(path); rmdir(directory);
}

void test_save_config_rejects_hardlinks_without_truncation_or_mode_change(void) {
    char directory[] = "/tmp/lightnvr_config_link_XXXXXX";
    TEST_ASSERT_NOT_NULL(mkdtemp(directory));
    char path[MAX_PATH_LENGTH], original[MAX_PATH_LENGTH];
    snprintf(path, sizeof(path), "%s/settings.ini", directory);
    snprintf(original, sizeof(original), "%s/original.ini", directory);
    FILE *file = fopen(original, "w"); TEST_ASSERT_NOT_NULL(file);
    fputs("preserve-me", file); fclose(file);
    TEST_ASSERT_EQUAL_INT(0, chmod(original, 0640));
    TEST_ASSERT_EQUAL_INT(0, link(original, path));
    load_default_config(&cfg);
    TEST_ASSERT_EQUAL_INT(-1, save_config(&cfg, path));
    struct stat info;
    TEST_ASSERT_EQUAL_INT(0, stat(original, &info));
    TEST_ASSERT_EQUAL_UINT(0640, info.st_mode & 0777);
    file = fopen(original, "r"); TEST_ASSERT_NOT_NULL(file);
    char text[32] = {0}; fread(text, 1, sizeof(text)-1, file); fclose(file);
    TEST_ASSERT_EQUAL_STRING("preserve-me", text);
    unlink(path); unlink(original); rmdir(directory);
}

static void check_group_writable_config_replacement(bool fail_publish) {
    char directory[] = "/tmp/lightnvr_group_config_XXXXXX";
    TEST_ASSERT_NOT_NULL(mkdtemp(directory));
    char path[MAX_PATH_LENGTH];
    snprintf(path, sizeof(path), "%s/settings.ini", directory);
    FILE *old = fopen(path, "w");
    TEST_ASSERT_NOT_NULL(old);
    fputs("preserve-old-content", old); fclose(old);
    TEST_ASSERT_EQUAL_INT(0, chmod(path, 0660));
    struct stat before, after;
    TEST_ASSERT_EQUAL_INT(0, stat(path, &before));
    load_default_config(&cfg);
    snprintf(cfg.mqtt_password, sizeof(cfg.mqtt_password), "synthetic-replacement-secret");
    /* Reproduce chmod EPERM for an otherwise writable legacy root-owned file
     * without requiring the unit suite to run with root privileges. */
    deny_config_chmod = true;
    deny_config_rename = fail_publish;
    observed_config_stream = false;
    observed_config_stream_with_private_mode = true;
    monitor_secret_write_permissions = !fail_publish;
    mode_t prior_umask = umask(fail_publish ? 0777 : 0);
    int result = save_config(&cfg, path);
    umask(prior_umask);
    monitor_secret_write_permissions = false;
    deny_config_rename = false;
    TEST_ASSERT_EQUAL_INT(fail_publish ? -1 : 0, result);
    if (!fail_publish) {
        TEST_ASSERT_TRUE(observed_config_stream);
        TEST_ASSERT_TRUE(observed_config_stream_with_private_mode);
    }
    TEST_ASSERT_EQUAL_INT(0, stat(path, &after));
    TEST_ASSERT_EQUAL_UINT(fail_publish ? 0660 : 0600, after.st_mode & 0777);
    if (fail_publish) TEST_ASSERT_EQUAL_UINT64(before.st_ino, after.st_ino);
    else TEST_ASSERT_NOT_EQUAL(before.st_ino, after.st_ino);
    FILE *saved = fopen(path, "r");
    TEST_ASSERT_NOT_NULL(saved);
    char content[32768] = {0};
    fread(content, 1, sizeof(content)-1, saved); fclose(saved);
    if (fail_publish) TEST_ASSERT_EQUAL_STRING("preserve-old-content", content);
    else TEST_ASSERT_NOT_NULL(strstr(content, "synthetic-replacement-secret"));
    unlink(path);
    /* Also verifies no private staging file is left behind on either path. */
    TEST_ASSERT_EQUAL_INT(0, rmdir(directory));
}
static void test_save_config_replaces_group_writable_legacy_file_privately(void) {
    check_group_writable_config_replacement(false);
}
static void test_save_config_preserves_legacy_file_when_replacement_fails(void) {
    check_group_writable_config_replacement(true);
}

static void test_save_config_atomically_replaces_owned_file(void) {
    char directory[] = "/tmp/lightnvr_config_owned_XXXXXX";
    TEST_ASSERT_NOT_NULL(mkdtemp(directory));
    char path[MAX_PATH_LENGTH];
    snprintf(path, sizeof(path), "%s/settings.ini", directory);
    FILE *initial = fopen(path, "w");
    TEST_ASSERT_NOT_NULL(initial);
    fputs("old content", initial);
    fclose(initial);
    struct stat before, after;
    TEST_ASSERT_EQUAL_INT(0, stat(path, &before));
    load_default_config(&cfg);
    TEST_ASSERT_EQUAL_INT(0, save_config(&cfg, path));
    TEST_ASSERT_EQUAL_INT(0, stat(path, &after));
    TEST_ASSERT_NOT_EQUAL(before.st_ino, after.st_ino);
    TEST_ASSERT_EQUAL_UINT(0600, after.st_mode & 0777);
    unlink(path);
    rmdir(directory);
}

static void test_save_config_preserves_owned_file_when_publish_fails(void) {
    char directory[] = "/tmp/lightnvr_config_atomic_XXXXXX";
    TEST_ASSERT_NOT_NULL(mkdtemp(directory));
    char path[MAX_PATH_LENGTH];
    snprintf(path, sizeof(path), "%s/settings.ini", directory);
    FILE *initial = fopen(path, "w");
    TEST_ASSERT_NOT_NULL(initial);
    fputs("previous configuration", initial);
    fclose(initial);
    load_default_config(&cfg);
    deny_config_rename = true;
    TEST_ASSERT_EQUAL_INT(-1, save_config(&cfg, path));
    deny_config_rename = false;
    char content[64] = {0};
    FILE *saved = fopen(path, "r");
    TEST_ASSERT_NOT_NULL(saved);
    TEST_ASSERT_EQUAL_UINT(strlen("previous configuration"),
                           fread(content, 1, sizeof(content) - 1, saved));
    fclose(saved);
    TEST_ASSERT_EQUAL_STRING("previous configuration", content);
    unlink(path);
    TEST_ASSERT_EQUAL_INT(0, rmdir(directory));
}

static void check_bind_mount_fallback(bool fail_copy) {
    char directory[] = "/tmp/lightnvr_config_bind_XXXXXX";
    TEST_ASSERT_NOT_NULL(mkdtemp(directory));
    char path[MAX_PATH_LENGTH];
    snprintf(path, sizeof(path), "%s/settings.ini", directory);
    FILE *initial = fopen(path, "w");
    TEST_ASSERT_NOT_NULL(initial);
    fputs("previous configuration", initial);
    fclose(initial);
    struct stat before, after;
    TEST_ASSERT_EQUAL_INT(0, stat(path, &before));
    load_default_config(&cfg);
    busy_config_rename = true;
    fail_config_pwrite = fail_copy;
    fail_config_pwrite_inode = before.st_ino;
    int result = save_config(&cfg, path);
    TEST_ASSERT_EQUAL_INT(fail_copy ? -1 : 0, result);
    TEST_ASSERT_EQUAL_INT(0, stat(path, &after));
    TEST_ASSERT_EQUAL_UINT64(before.st_ino, after.st_ino);
    char content[128] = {0};
    FILE *saved = fopen(path, "r");
    TEST_ASSERT_NOT_NULL(saved);
    size_t count = fread(content, 1, sizeof(content) - 1, saved);
    fclose(saved);
    if (fail_copy) TEST_ASSERT_EQUAL_STRING("previous configuration", content);
    else TEST_ASSERT_TRUE(count > 0 && strstr(content, "LightNVR Configuration File") != NULL);
    unlink(path);
    TEST_ASSERT_EQUAL_INT(0, rmdir(directory));
}

static void test_save_config_bind_mount_fallback(void) {
    check_bind_mount_fallback(false);
}

static void test_save_config_bind_mount_restores_after_write_failure(void) {
    check_bind_mount_fallback(true);
}

static void test_save_config_updates_file_in_read_only_directory(void) {
    if (geteuid() == 0) {
        TEST_IGNORE_MESSAGE("Skipping read-only directory permission test when running as root.");
        return;
    }

    char directory[] = "/tmp/lightnvr_config_read_only_dir_XXXXXX";
    TEST_ASSERT_NOT_NULL(mkdtemp(directory));
    char path[MAX_PATH_LENGTH];
    snprintf(path, sizeof(path), "%s/settings.ini", directory);
    FILE *initial = fopen(path, "w");
    TEST_ASSERT_NOT_NULL(initial);
    fputs("old content", initial);
    fclose(initial);
    TEST_ASSERT_EQUAL_INT(0, chmod(directory, 0500));
    load_default_config(&cfg);
    int result = save_config(&cfg, path);
    TEST_ASSERT_EQUAL_INT(0, chmod(directory, 0700));
    TEST_ASSERT_EQUAL_INT(0, result);
    unlink(path);
    rmdir(directory);
}

int main(void) {
    init_logger();

    UNITY_BEGIN();
    RUN_TEST(test_save_config_tightens_existing_permissions_before_writing_secrets);
    RUN_TEST(test_save_config_rejects_hardlinks_without_truncation_or_mode_change);
    RUN_TEST(test_save_config_replaces_group_writable_legacy_file_privately);
    RUN_TEST(test_save_config_preserves_legacy_file_when_replacement_fails);
    RUN_TEST(test_save_config_atomically_replaces_owned_file);
    RUN_TEST(test_save_config_preserves_owned_file_when_publish_fails);
    RUN_TEST(test_save_config_bind_mount_fallback);
    RUN_TEST(test_save_config_bind_mount_restores_after_write_failure);
    RUN_TEST(test_save_config_updates_file_in_read_only_directory);

    RUN_TEST(test_stream_config_t_size_under_16k);
    RUN_TEST(test_max_stream_ceiling_is_valid);

    RUN_TEST(test_default_config_web_port);
    RUN_TEST(test_default_data_paths_stay_inside_persistent_volume);
    RUN_TEST(test_default_config_log_level);
    RUN_TEST(test_default_config_retention_days);
    RUN_TEST(test_default_config_mp4_faststart_on);
    RUN_TEST(test_config_copy_owns_streams_after_source_reload);
    RUN_TEST(test_mp4_faststart_concurrent_reads_copies_and_saves);
    RUN_TEST(test_default_config_buffer_size);
    RUN_TEST(test_default_config_storage_path_nonempty);
    RUN_TEST(test_default_config_db_path_nonempty);
    RUN_TEST(test_default_config_db_backup_settings);
    RUN_TEST(test_default_config_health_matches_ops03);
    RUN_TEST(test_validate_config_rejects_unsafe_health_values);
    RUN_TEST(test_validate_config_resets_nonfinite_storage_pressure);
    RUN_TEST(test_default_config_models_path_nonempty);
    RUN_TEST(test_default_config_null_safe);

    RUN_TEST(test_validate_config_valid_defaults);
    RUN_TEST(test_validate_config_null);
    RUN_TEST(test_validate_config_empty_storage_path);
    RUN_TEST(test_validate_config_empty_models_path);
    RUN_TEST(test_validate_config_empty_db_path);
    RUN_TEST(test_validate_config_empty_web_root);
    RUN_TEST(test_validate_config_port_zero);
    RUN_TEST(test_validate_config_port_too_high);
    RUN_TEST(test_validate_config_port_max_valid);
    RUN_TEST(test_validate_config_port_min_valid);
    RUN_TEST(test_validate_config_buffer_size_zero);
    RUN_TEST(test_validate_config_clamps_absolute_timeout_to_idle_timeout);
    RUN_TEST(test_validate_config_clamps_negative_db_backup_values);
    RUN_TEST(test_default_config_db_startup_check_is_off);
    RUN_TEST(test_validate_config_clamps_out_of_range_startup_check);
    RUN_TEST(test_default_config_db_backup_verify_is_full);
    RUN_TEST(test_validate_config_clamps_out_of_range_backup_verify);
    RUN_TEST(test_validate_config_preserves_valid_startup_check);

    RUN_TEST(test_default_config_web_auth_enabled);
    RUN_TEST(test_default_config_username);
    RUN_TEST(test_default_config_syslog_disabled);
    RUN_TEST(test_default_config_go2rtc_enabled);
    RUN_TEST(test_default_config_go2rtc_api_port);
    RUN_TEST(test_default_config_go2rtc_webrtc_enabled);
    RUN_TEST(test_default_config_go2rtc_stun_enabled);
    RUN_TEST(test_default_config_turn_disabled);
    RUN_TEST(test_default_config_mqtt_disabled);
    RUN_TEST(test_default_config_mqtt_port);
    RUN_TEST(test_default_config_mp4_segment_duration);
    RUN_TEST(test_default_config_mp4_directory_format);
    RUN_TEST(test_default_config_stream_defaults);
    RUN_TEST(test_default_config_auth_timeout);
    RUN_TEST(test_default_config_audio_enabled);
    RUN_TEST(test_default_config_auto_view_enabled);
    RUN_TEST(test_default_config_web_compression_enabled);

    RUN_TEST(test_validate_config_swap_size_zero_with_use_swap);
    RUN_TEST(test_validate_config_swap_disabled_size_zero_ok);

    RUN_TEST(test_custom_config_path_empty_not_stored);
    RUN_TEST(test_custom_config_path_null_not_stored);
    RUN_TEST(test_custom_config_path_roundtrip);
    RUN_TEST(test_get_loaded_config_path_initially);
    RUN_TEST(test_save_config_accepts_hidden_ini_dotfile);
    RUN_TEST(test_env_integer_whitespace_handling);
    RUN_TEST(test_default_config_api_detection_settings);
    RUN_TEST(test_config_set_api_detection_format_normalizes_and_rejects);
    RUN_TEST(test_save_config_writes_api_detection_format);
    RUN_TEST(test_load_config_parses_api_detection_format);
    RUN_TEST(test_malformed_ini_after_stream_resize_keeps_streams_owned);
    /* Uses the process-wide custom config path; run last to avoid affecting later tests. */
    RUN_TEST(test_save_config_round_trips_mp4_faststart);

    int result = UNITY_END();
    shutdown_logger();
    return result;
}
