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

#include "unity.h"
#include "core/config.h"
#include "core/logger.h"

/* File-scope config used by most tests — setUp zeros it, tearDown frees
   the dynamically allocated streams array to keep ASan leak-free. */
static config_t cfg;

/* ---- Unity boilerplate ---- */
void setUp(void) {
    memset(&cfg, 0, sizeof(cfg));
}
void tearDown(void) {
    if (cfg.streams) {
        free(cfg.streams);
        cfg.streams = NULL;
    }
}

/* ================================================================
 * load_default_config
 * ================================================================ */

void test_default_config_web_port(void) {
    load_default_config(&cfg);
    TEST_ASSERT_EQUAL_INT(8080, cfg.web_port);
}

void test_default_config_log_level(void) {
    load_default_config(&cfg);
    TEST_ASSERT_EQUAL_INT(LOG_LEVEL_INFO, cfg.log_level);
}

void test_default_config_retention_days(void) {
    load_default_config(&cfg);
    TEST_ASSERT_EQUAL_INT(30, cfg.retention_days);
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
    TEST_ASSERT_EQUAL_INT(60, cfg.db_backup_interval_minutes);
    TEST_ASSERT_EQUAL_INT(24, cfg.db_backup_retention_count);
    TEST_ASSERT_EQUAL_STRING("", cfg.db_post_backup_script);
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

    char config_path[512];
    snprintf(config_path, sizeof(config_path), "%s/.ini", dir);

    load_default_config(&cfg);
    cfg.db_backup_interval_minutes = 15;
    cfg.db_backup_retention_count = 8;
    snprintf(cfg.db_post_backup_script, sizeof(cfg.db_post_backup_script), "/usr/local/bin/post-backup");
    TEST_ASSERT_EQUAL_INT(0, save_config(&cfg, config_path));

    FILE *saved = fopen(config_path, "r");
    TEST_ASSERT_NOT_NULL(saved);
    if (saved) {
        char file_buffer[4096] = {0};
        size_t bytes_read = fread(file_buffer, 1, sizeof(file_buffer) - 1, saved);
        TEST_ASSERT_GREATER_THAN(0, (int)bytes_read);
        file_buffer[bytes_read] = '\0';
        TEST_ASSERT_NOT_NULL(strstr(file_buffer, "; LightNVR Configuration File"));
        TEST_ASSERT_NOT_NULL(strstr(file_buffer, "backup_interval_minutes = 15"));
        TEST_ASSERT_NOT_NULL(strstr(file_buffer, "backup_retention_count = 8"));
        TEST_ASSERT_NOT_NULL(strstr(file_buffer, "post_backup_script = /usr/local/bin/post-backup"));
        fclose(saved);
    }

    unlink(config_path);
    rmdir(dir);
}

void test_env_integer_whitespace_handling(void) {
    char temp_dir[] = "/tmp/lightnvr_load_config_XXXXXX";
    char *dir = mkdtemp(temp_dir);
    TEST_ASSERT_NOT_NULL(dir);

    char config_path[512];
    char storage_path[512];
    char storage_hls_path[512];
    char models_path[512];
    char db_path[512];
    char web_root[512];
    char log_path[512];
    char pid_path[512];

    snprintf(config_path, sizeof(config_path), "%s/test.ini", dir);
    snprintf(storage_path, sizeof(storage_path), "%s/storage", dir);
    snprintf(storage_hls_path, sizeof(storage_hls_path), "%s/hls", dir);
    snprintf(models_path, sizeof(models_path), "%s/models", dir);
    snprintf(db_path, sizeof(db_path), "%s/lightnvr.db", dir);
    snprintf(web_root, sizeof(web_root), "%s/web", dir);
    snprintf(log_path, sizeof(log_path), "%s/lightnvr.log", dir);
    snprintf(pid_path, sizeof(pid_path), "%s/lightnvr.pid", dir);

    FILE *config_file = fopen(config_path, "w");
    TEST_ASSERT_NOT_NULL(config_file);
    fprintf(config_file,
            "[general]\n"
            "pid_file = %s\n"
            "log_file = %s\n\n"
            "[storage]\n"
            "path = %s\n"
            "path_hls = %s\n\n"
            "[models]\n"
            "path = %s\n\n"
            "[database]\n"
            "path = %s\n"
            "backup_interval_minutes = 45\n"
            "backup_retention_count = 12\n"
            "post_backup_script = /usr/local/bin/backup-hook\n\n"
            "[web]\n"
            "root = %s\n",
            pid_path, log_path, storage_path, storage_hls_path,
            models_path, db_path, web_root);
    fclose(config_file);

    TEST_ASSERT_EQUAL_INT(0, mkdir(web_root, 0755));

    const char *previous_env = getenv("LIGHTNVR_WEB_PORT");
    char *saved_env = previous_env ? strdup(previous_env) : NULL;
    if (previous_env) {
        TEST_ASSERT_NOT_NULL(saved_env);
    }

    set_custom_config_path(config_path);
    /* Trailing whitespace should be ignored. */
    TEST_ASSERT_EQUAL_INT(0, setenv("LIGHTNVR_WEB_PORT", "9099   ", 1));
    TEST_ASSERT_EQUAL_INT(0, load_config(&cfg));
    TEST_ASSERT_EQUAL_INT(9099, cfg.web_port);
    TEST_ASSERT_EQUAL_INT(45, cfg.db_backup_interval_minutes);
    TEST_ASSERT_EQUAL_INT(12, cfg.db_backup_retention_count);
    TEST_ASSERT_EQUAL_STRING("/usr/local/bin/backup-hook", cfg.db_post_backup_script);

    /* Leading whitespace should be ignored. */
    TEST_ASSERT_EQUAL_INT(0, setenv("LIGHTNVR_WEB_PORT", "   9099", 1));
    TEST_ASSERT_EQUAL_INT(0, load_config(&cfg));
    TEST_ASSERT_EQUAL_INT(9099, cfg.web_port);

    /* Mixed leading and trailing whitespace should be ignored. */
    TEST_ASSERT_EQUAL_INT(0, setenv("LIGHTNVR_WEB_PORT", "   9099   ", 1));
    TEST_ASSERT_EQUAL_INT(0, load_config(&cfg));
    TEST_ASSERT_EQUAL_INT(9099, cfg.web_port);

    if (saved_env) {
        TEST_ASSERT_EQUAL_INT(0, setenv("LIGHTNVR_WEB_PORT", saved_env, 1));
        free(saved_env);
    } else {
        TEST_ASSERT_EQUAL_INT(0, unsetenv("LIGHTNVR_WEB_PORT"));
    }

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

int main(void) {
    init_logger();

    UNITY_BEGIN();

    RUN_TEST(test_default_config_web_port);
    RUN_TEST(test_default_config_log_level);
    RUN_TEST(test_default_config_retention_days);
    RUN_TEST(test_default_config_buffer_size);
    RUN_TEST(test_default_config_storage_path_nonempty);
    RUN_TEST(test_default_config_db_path_nonempty);
    RUN_TEST(test_default_config_db_backup_settings);
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
    RUN_TEST(test_default_config_stream_defaults);
    RUN_TEST(test_default_config_auth_timeout);
    RUN_TEST(test_default_config_web_compression_enabled);

    RUN_TEST(test_validate_config_swap_size_zero_with_use_swap);
    RUN_TEST(test_validate_config_swap_disabled_size_zero_ok);

    RUN_TEST(test_custom_config_path_empty_not_stored);
    RUN_TEST(test_custom_config_path_null_not_stored);
    RUN_TEST(test_custom_config_path_roundtrip);
    RUN_TEST(test_get_loaded_config_path_initially);
    RUN_TEST(test_save_config_accepts_hidden_ini_dotfile);
    RUN_TEST(test_env_integer_whitespace_handling);

    int result = UNITY_END();
    shutdown_logger();
    return result;
}

