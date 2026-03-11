/**
 * @file test_go2rtc_process_config_generation.c
 * @brief Tests for bootstrap go2rtc.yaml generation from saved settings.
 */

#define _POSIX_C_SOURCE 200809L

#include <limits.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include "unity.h"
#include "core/config.h"
#include "core/logger.h"
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

int main(void) {
    init_logger();

    UNITY_BEGIN();
    RUN_TEST(test_generate_startup_config_uses_saved_webrtc_settings);
    return UNITY_END();
}