/**
 * @file test_go2rtc_process_config_generation.c
 * @brief Tests for bootstrap go2rtc.yaml generation from saved settings.
 */

#define _POSIX_C_SOURCE 200809L

#include <dirent.h>
#include <limits.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <unistd.h>

#include "unity.h"
#include "core/config.h"
#include "core/logger.h"
#include "database/db_core.h"
#include "database/db_system_settings.h"
#include "video/go2rtc/go2rtc_process.h"

void setUp(void) {}
void tearDown(void) {}

static char *read_text_file(const char *path) {
    FILE *fp = fopen(path, "rb");
    if (!fp) return NULL;

    if (fseek(fp, 0, SEEK_END) != 0) {
        fclose(fp);
        return NULL;
    }

    long length = ftell(fp);
    if (length < 0 || fseek(fp, 0, SEEK_SET) != 0) {
        fclose(fp);
        return NULL;
    }

    char *buffer = calloc((size_t)length + 1, 1);
    if (!buffer) {
        fclose(fp);
        return NULL;
    }

    size_t read_count = fread(buffer, 1, (size_t)length, fp);
    fclose(fp);
    if (read_count != (size_t)length) {
        free(buffer);
        return NULL;
    }

    return buffer;
}

static void remove_tree(const char *path) {
    DIR *dir = opendir(path);
    if (!dir) {
        return;
    }

    struct dirent *entry;
    while ((entry = readdir(dir)) != NULL) {
        if (strcmp(entry->d_name, ".") == 0 || strcmp(entry->d_name, "..") == 0) {
            continue;
        }

        char child[PATH_MAX];
        snprintf(child, sizeof(child), "%s/%s", path, entry->d_name);

        struct stat st;
        if (lstat(child, &st) != 0) {
            continue;
        }

        if (S_ISDIR(st.st_mode)) {
            remove_tree(child);
        } else {
            unlink(child);
        }
    }

    closedir(dir);
    rmdir(path);
}

void test_generate_startup_config_uses_saved_webrtc_settings(void) {
    load_default_config(&g_config);
    g_config.go2rtc_api_port = 31984;
    g_config.go2rtc_rtsp_port = 31554;
    g_config.go2rtc_webrtc_listen_port = 31555;
    g_config.go2rtc_stun_enabled = false;
    snprintf(g_config.go2rtc_external_ip, sizeof(g_config.go2rtc_external_ip), "192.168.50.10");
    g_config.go2rtc_ice_servers[0] = '\0';

    char dir_template[] = "/tmp/lightnvr-go2rtc-config-XXXXXX";
    char *config_dir = mkdtemp(dir_template);
    TEST_ASSERT_NOT_NULL(config_dir);

    char config_path[PATH_MAX];
    snprintf(config_path, sizeof(config_path), "%s/go2rtc.yaml", config_dir);

    TEST_ASSERT_TRUE(go2rtc_process_generate_startup_config("/bin/true",
                                                            config_dir,
                                                            g_config.go2rtc_api_port));

    char *contents = read_text_file(config_path);
    TEST_ASSERT_NOT_NULL(contents);
    TEST_ASSERT_NOT_NULL(strstr(contents, "  listen: :31984"));
    TEST_ASSERT_NOT_NULL(strstr(contents, "  listen: \":31554\""));
    TEST_ASSERT_NOT_NULL(strstr(contents, "  listen: \":31555\""));
    TEST_ASSERT_NOT_NULL(strstr(contents, "    - \"192.168.50.10:31555\""));
    TEST_ASSERT_NULL(strstr(contents, "stun.l.google.com"));

    free(contents);
    unlink(config_path);
    rmdir(config_dir);
}

void test_generate_startup_config_writes_db_override_file(void) {
    load_default_config(&g_config);
    g_config.go2rtc_api_port = 31984;
    g_config.go2rtc_rtsp_port = 31554;
    g_config.go2rtc_webrtc_enabled = false;

    char root_template[] = "/tmp/lightnvr-go2rtc-startup-XXXXXX";
    char *root_dir = mkdtemp(root_template);
    TEST_ASSERT_NOT_NULL(root_dir);

    char config_dir[256];
    snprintf(config_dir, sizeof(config_dir), "%s/go2rtc", root_dir);

    char db_path[256];
    snprintf(db_path, sizeof(db_path), "%s/lightnvr.db", root_dir);

    const char *override_yaml =
        "log:\n"
        "  level: trace\n";

    TEST_ASSERT_EQUAL_INT(0, init_database(db_path));
    TEST_ASSERT_EQUAL_INT(0, db_set_system_setting("go2rtc_config_override",
                                                   override_yaml));

    TEST_ASSERT_TRUE(go2rtc_process_generate_startup_config("/bin/true",
                                                            config_dir,
                                                            g_config.go2rtc_api_port));

    char config_path[PATH_MAX];
    snprintf(config_path, sizeof(config_path), "%s/go2rtc.yaml", config_dir);

    char override_path[PATH_MAX];
    snprintf(override_path, sizeof(override_path), "%s/override.yaml", config_dir);

    char *override_contents = read_text_file(override_path);
    TEST_ASSERT_NOT_NULL(override_contents);
    TEST_ASSERT_EQUAL_STRING(override_yaml, override_contents);

    char *base_contents = read_text_file(config_path);
    TEST_ASSERT_NOT_NULL(base_contents);
    TEST_ASSERT_NULL_MESSAGE(strstr(base_contents, "  level: trace"),
                             "Base config must not contain DB-backed override content");

    free(base_contents);
    free(override_contents);
    shutdown_database();
    remove_tree(root_dir);
}

/**
 * Post-T2 refactor assertion: the base go2rtc.yaml generator must NEVER append
 * the user config override as a tail block — that caused duplicate top-level
 * keys (issue #394). The override is now written to a sibling file and passed
 * to go2rtc as a second --config argument (T3/T4).
 */
void test_base_config_has_no_user_override_append_block(void) {
    load_default_config(&g_config);
    g_config.go2rtc_api_port = 31984;
    g_config.go2rtc_rtsp_port = 31554;
    g_config.go2rtc_webrtc_enabled = false;

    char dir_template[] = "/tmp/lightnvr-go2rtc-config-XXXXXX";
    char *config_dir = mkdtemp(dir_template);
    TEST_ASSERT_NOT_NULL(config_dir);

    char config_path[PATH_MAX];
    snprintf(config_path, sizeof(config_path), "%s/go2rtc.yaml", config_dir);

    TEST_ASSERT_TRUE(go2rtc_process_generate_startup_config("/bin/true",
                                                            config_dir,
                                                            g_config.go2rtc_api_port));

    char *contents = read_text_file(config_path);
    TEST_ASSERT_NOT_NULL(contents);

    // The literal tail-append marker that used to dump the raw user setting
    // into the same YAML file must no longer appear.
    TEST_ASSERT_NULL_MESSAGE(strstr(contents, "# User config override"),
                             "Base config must not contain appended user override block");

    // Exactly one top-level `ffmpeg:` stanza — never duplicates (the exact
    // symptom from issue #394). Count occurrences of "\nffmpeg:" plus a
    // leading-of-file "ffmpeg:" case to be safe.
    int ffmpeg_count = 0;
    const char *scan = contents;
    while ((scan = strstr(scan, "\nffmpeg:")) != NULL) {
        ffmpeg_count++;
        scan += strlen("\nffmpeg:");
    }
    if (strncmp(contents, "ffmpeg:", strlen("ffmpeg:")) == 0) {
        ffmpeg_count++;
    }
    TEST_ASSERT_EQUAL_INT_MESSAGE(1, ffmpeg_count,
                                  "Base config must contain exactly one ffmpeg: stanza");

    free(contents);
    unlink(config_path);
    rmdir(config_dir);
}

/**
 * Link-time assertion that T2 exposed the T3 helper stubs. This guarantees
 * downstream tasks can call them even before T3 lands, and prevents an
 * accidental removal of the declarations.
 */
void test_override_helpers_are_linkable_stubs(void) {
    // Stub currently returns 0 ("success / no-op"). When T3 lands it will
    // return 0 on success and -1 on failure; either way the function must exist
    // and be callable.
    int rc = go2rtc_process_generate_override_file("/tmp/does-not-matter.yaml");
    TEST_ASSERT_TRUE(rc == 0 || rc == -1);

    // Stub currently returns NULL; T3 will return either NULL or a stable
    // path string. Either way the symbol must exist.
    const char *path = go2rtc_process_get_override_path();
    TEST_ASSERT_TRUE(path == NULL || path[0] != '\0');
}

int main(void) {
    init_logger();

    UNITY_BEGIN();
    RUN_TEST(test_generate_startup_config_uses_saved_webrtc_settings);
    RUN_TEST(test_generate_startup_config_writes_db_override_file);
    RUN_TEST(test_base_config_has_no_user_override_append_block);
    RUN_TEST(test_override_helpers_are_linkable_stubs);
    return UNITY_END();
}
