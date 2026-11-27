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
#include <sys/wait.h>
#include <fcntl.h>
#include <unistd.h>
#include <pthread.h>
#include <sched.h>
#include <stdatomic.h>
#include <errno.h>
#include <math.h>
#include <time.h>
#include <signal.h>

#include "unity.h"
#include "core/config.h"
#include "core/daemon.h"
#include "core/logger.h"
#include "core/path_utils.h"
#include "database/db_core.h"
#include "video/streams.h"

/* File-scope config used by most tests — setUp zeros it, tearDown frees
   the dynamically allocated streams array to keep ASan leak-free. */
static config_t cfg;
static config_t reloaded_cfg;
static FILE *saved_config_file;
static FILE *saved_baseline_file;
static void assert_files_equal_binary(FILE *expected, FILE *actual);
static char concurrent_config_dir[MAX_PATH_LENGTH];
static char stream_save_db_path[MAX_PATH_LENGTH];
static bool stream_save_db_active;
static char group_replace_dir[MAX_PATH_LENGTH];
static char group_replace_path[MAX_PATH_LENGTH];
static bool group_replace_cleanup_active;
static char short_write_dir[MAX_PATH_LENGTH];
static char short_write_path[MAX_PATH_LENGTH];
static char short_write_baseline_dir[MAX_PATH_LENGTH];
static char short_write_baseline_path[MAX_PATH_LENGTH];
static char *saved_config_text;
static stream_config_t *owned_snapshot_streams;
static int owned_pid_fd;
static char owned_pid_path[MAX_PATH_LENGTH];
static bool deny_config_chmod;
static bool deny_config_chmod_target_set;
static dev_t deny_config_chmod_target_dev;
static ino_t deny_config_chmod_target_ino;
static bool owned_bind_dir_active;
static char owned_bind_dir_path[MAX_PATH_LENGTH];
static char owned_bind_file_path[MAX_PATH_LENGTH];
static char owned_bind_baseline_path[MAX_PATH_LENGTH];
static char owned_private_config_dir[MAX_PATH_LENGTH];
static char owned_private_config_path[MAX_PATH_LENGTH];
static char owned_credential_config_dir[MAX_PATH_LENGTH];
static char owned_credential_config_path[MAX_PATH_LENGTH];
static char owned_credential_control_path[MAX_PATH_LENGTH];
static bool deny_config_rename;
static bool busy_config_rename;
static bool fail_config_pwrite;
static ino_t fail_config_pwrite_inode;
static bool short_config_pwrite;
static bool check_config_pwrite_mode;
static ino_t check_config_pwrite_inode;
static bool check_config_pwrite_saw_write;
static bool check_config_pwrite_mode_violation;
static ino_t short_config_pwrite_inode;
static size_t short_config_pwrite_max_bytes;
static bool short_config_pwrite_done;
static bool monitor_secret_write_permissions;
static bool observed_config_stream;
static bool observed_config_stream_with_private_mode;
static char *saved_web_port_env;
static char *saved_audio_disabled_env;
static char *saved_go2rtc_listen_port_env;
static const char *const timeout_env_names[] = {
    "LIGHTNVR_WEBRTC_CONNECTION_TIMEOUT_MS",
    "LIGHTNVR_WEBRTC_ICE_RECOVERY_TIMEOUT_MS"
};
static char *saved_timeout_env[2];
int __real_fchmod(int, mode_t);
FILE *__real_fdopen(int, const char *);
int __wrap_fchmod(int fd, mode_t mode) {
    if (deny_config_chmod) {
        struct stat st;
        if (deny_config_chmod_target_set &&
            fstat(fd, &st) == 0 &&
            st.st_dev == deny_config_chmod_target_dev &&
            st.st_ino == deny_config_chmod_target_ino) {
            deny_config_chmod = false;
            errno = EPERM;
            return -1;
        }
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
    if (fstat(fd, &st) == 0) {
        if (check_config_pwrite_mode && st.st_ino == check_config_pwrite_inode) {
            check_config_pwrite_saw_write = true;
            if ((st.st_mode & 0777) != 0600) {
                check_config_pwrite_mode_violation = true;
            }
        }
        if (short_config_pwrite &&
            !short_config_pwrite_done &&
            st.st_ino == short_config_pwrite_inode &&
            size > short_config_pwrite_max_bytes) {
            short_config_pwrite_done = true;
            return __real_pwrite(fd, buf, short_config_pwrite_max_bytes, offset);
        }
        if (fail_config_pwrite && st.st_ino == fail_config_pwrite_inode) {
            fail_config_pwrite = false;
            errno = ENOSPC;
            return -1;
        }
    }
    return __real_pwrite(fd, buf, size, offset);
}

/* ---- Unity boilerplate ---- */
void setUp(void) {
    memset(&cfg, 0, sizeof(cfg));
    memset(&reloaded_cfg, 0, sizeof(reloaded_cfg));
    owned_pid_fd = -1;
    owned_pid_path[0] = '\0';
    owned_bind_dir_active = false;
    owned_bind_dir_path[0] = '\0';
    owned_bind_file_path[0] = '\0';
    owned_bind_baseline_path[0] = '\0';
    owned_private_config_dir[0] = '\0';
    owned_private_config_path[0] = '\0';
    owned_credential_config_dir[0] = '\0';
    owned_credential_config_path[0] = '\0';
    owned_credential_control_path[0] = '\0';
    group_replace_dir[0] = '\0';
    group_replace_path[0] = '\0';
    group_replace_cleanup_active = false;
    short_write_dir[0] = '\0';
    short_write_path[0] = '\0';
    short_write_baseline_dir[0] = '\0';
    short_write_baseline_path[0] = '\0';
    saved_config_file = NULL;
    saved_baseline_file = NULL;
    concurrent_config_dir[0] = '\0';
    stream_save_db_path[0] = '\0';
    stream_save_db_active = false;
    saved_config_text = NULL;
    check_config_pwrite_mode = false;
    check_config_pwrite_saw_write = false;
    check_config_pwrite_mode_violation = false;
    owned_snapshot_streams = NULL;
    observed_config_stream = false;
    observed_config_stream_with_private_mode = true;
    const char *web_port = getenv("LIGHTNVR_WEB_PORT");
    const char *audio_disabled = getenv("LIGHTNVR_AUDIO_DISABLED");
    const char *go2rtc_listen_port = getenv("LIGHTNVR_GO2RTC_WEBRTC_LISTEN_PORT");
    saved_web_port_env = web_port ? strdup(web_port) : NULL;
    saved_audio_disabled_env = audio_disabled ? strdup(audio_disabled) : NULL;
    saved_go2rtc_listen_port_env = go2rtc_listen_port ? strdup(go2rtc_listen_port) : NULL;
    if (web_port) TEST_ASSERT_NOT_NULL(saved_web_port_env);
    if (audio_disabled) TEST_ASSERT_NOT_NULL(saved_audio_disabled_env);
    if (go2rtc_listen_port) TEST_ASSERT_NOT_NULL(saved_go2rtc_listen_port_env);
    unsetenv("LIGHTNVR_WEB_PORT");
    unsetenv("LIGHTNVR_AUDIO_DISABLED");
    unsetenv("LIGHTNVR_GO2RTC_WEBRTC_LISTEN_PORT");
    for (size_t i = 0; i < 2; i++) {
        const char *value = getenv(timeout_env_names[i]);
        saved_timeout_env[i] = value ? strdup(value) : NULL;
        if (value) TEST_ASSERT_NOT_NULL(saved_timeout_env[i]);
        unsetenv(timeout_env_names[i]);
    }
}
void tearDown(void) {
    if (stream_save_db_active) {
        shutdown_database();
        stream_save_db_active = false;
    }
    if (stream_save_db_path[0]) {
        char aux_path[MAX_PATH_LENGTH + 5];
        unlink(stream_save_db_path);
        snprintf(aux_path, sizeof(aux_path), "%s-wal", stream_save_db_path);
        unlink(aux_path);
        snprintf(aux_path, sizeof(aux_path), "%s-shm", stream_save_db_path);
        unlink(aux_path);
    }
    if (short_write_path[0]) unlink(short_write_path);
    if (short_write_dir[0]) rmdir(short_write_dir);
    if (short_write_baseline_path[0]) unlink(short_write_baseline_path);
    if (short_write_baseline_dir[0]) rmdir(short_write_baseline_dir);
    if (group_replace_cleanup_active) {
        unlink(group_replace_path);
        rmdir(group_replace_dir);
        group_replace_cleanup_active = false;
    }
    if (owned_private_config_path[0]) unlink(owned_private_config_path);
    if (owned_private_config_dir[0]) rmdir(owned_private_config_dir);
    if (owned_credential_config_path[0]) unlink(owned_credential_config_path);
    if (owned_credential_control_path[0]) unlink(owned_credential_control_path);
    if (owned_credential_config_dir[0]) rmdir(owned_credential_config_dir);
    if (owned_bind_dir_active) {
        unlink(owned_bind_file_path);
        unlink(owned_bind_baseline_path);
        rmdir(owned_bind_dir_path);
        owned_bind_dir_active = false;
    }
    if (concurrent_config_dir[0]) {
        char path[MAX_PATH_LENGTH];
        snprintf(path, sizeof(path), "%s/settings.ini", concurrent_config_dir);
        unlink(path);
        rmdir(concurrent_config_dir);
        concurrent_config_dir[0] = '\0';
    }
    if (owned_pid_fd >= 0) {
        remove_pid_file(owned_pid_fd, owned_pid_path);
        owned_pid_fd = -1;
    }
    if (owned_pid_path[0]) {
        unlink(owned_pid_path);
        char *last_slash = strrchr(owned_pid_path, '/');
        if (last_slash) {
            *last_slash = '\0';
            rmdir(owned_pid_path);
        }
        owned_pid_path[0] = '\0';
    }
    check_config_pwrite_mode = false;
    if (saved_config_file) fclose(saved_config_file);
    saved_config_file = NULL;
    if (saved_baseline_file) fclose(saved_baseline_file);
    saved_baseline_file = NULL;
    free(saved_config_text);
    saved_config_text = NULL;
    free(owned_snapshot_streams);
    owned_snapshot_streams = NULL;
    deny_config_chmod = deny_config_rename = busy_config_rename = false;
    deny_config_chmod_target_set = false;
    fail_config_pwrite = false;
    short_config_pwrite = false;
    short_config_pwrite_done = false;
    short_config_pwrite_max_bytes = 0;
    monitor_secret_write_permissions = false;
    observed_config_stream = false;
    observed_config_stream_with_private_mode = false;
    if (saved_web_port_env) setenv("LIGHTNVR_WEB_PORT", saved_web_port_env, 1);
    else unsetenv("LIGHTNVR_WEB_PORT");
    if (saved_audio_disabled_env) setenv("LIGHTNVR_AUDIO_DISABLED", saved_audio_disabled_env, 1);
    else unsetenv("LIGHTNVR_AUDIO_DISABLED");
    if (saved_go2rtc_listen_port_env) setenv("LIGHTNVR_GO2RTC_WEBRTC_LISTEN_PORT", saved_go2rtc_listen_port_env, 1);
    else unsetenv("LIGHTNVR_GO2RTC_WEBRTC_LISTEN_PORT");
    free(saved_web_port_env);
    free(saved_audio_disabled_env);
    free(saved_go2rtc_listen_port_env);
    saved_web_port_env = saved_audio_disabled_env = NULL;
    saved_go2rtc_listen_port_env = NULL;
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
    free(reloaded_cfg.streams);
    reloaded_cfg.streams = NULL;
    free(g_config.streams);
    g_config.streams = NULL;
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

void test_daemon_status_preserves_owned_pid_lock(void) {
    char directory[] = "/tmp/lightnvr_pid_status_XXXXXX";
    TEST_ASSERT_NOT_NULL(mkdtemp(directory));
    char path[MAX_PATH_LENGTH];
    snprintf(path, sizeof(path), "%s/lightnvr.pid", directory);

    owned_pid_fd = write_pid_file(path);
    TEST_ASSERT_TRUE(owned_pid_fd >= 0);
    snprintf(owned_pid_path, sizeof(owned_pid_path), "%s", path);
    TEST_ASSERT_EQUAL_INT(-1, write_pid_file(path));
    TEST_ASSERT_EQUAL_INT(1, daemon_status(path));
    char alias[MAX_PATH_LENGTH];
    snprintf(alias, sizeof(alias), "%s/alias.pid", directory);
    TEST_ASSERT_EQUAL_INT(0, link(path, alias));
    TEST_ASSERT_EQUAL_INT(1, daemon_status(path));
    TEST_ASSERT_EQUAL_INT(1, daemon_status(alias));
    TEST_ASSERT_EQUAL_INT(0, unlink(alias));

    pid_t child = fork();
    TEST_ASSERT_TRUE(child >= 0);
    if (child == 0) {
        close(owned_pid_fd);
        if (daemon_status(path) != 1) _exit(2);
        int probe = open(path, O_RDWR);
        if (probe < 0) _exit(3);
        int lock_result = lockf(probe, F_TLOCK, 0);
        int lock_error = errno;
        close(probe);
        _exit(lock_result < 0 && (lock_error == EACCES || lock_error == EAGAIN) ? 0 : 4);
    }
    int status = 0;
    const long wait_step_ns = 10L * 1000L * 1000L;       /* 10 ms */
    const long wait_timeout_ns = 2L * 1000L * 1000L * 1000L;  /* 2 s */
    struct timespec start_ts;
    struct timespec now_ts;
    if (clock_gettime(CLOCK_MONOTONIC, &start_ts) != 0) {
        (void)kill(child, SIGKILL);
        (void)waitpid(child, &status, 0);
        TEST_FAIL_MESSAGE("clock_gettime failed");
    }
    long long deadline_ns =
        (long long)start_ts.tv_sec * 1000000000LL + start_ts.tv_nsec + wait_timeout_ns;
    pid_t waited = -1;
    for (;;) {
        waited = waitpid(child, &status, WNOHANG);
        if (waited == child) {
            break;
        }
        if (waited < 0 && errno != EINTR) {
            (void)kill(child, SIGKILL);
            (void)waitpid(child, &status, 0);
            TEST_FAIL_MESSAGE("waitpid failed while polling child");
        }

        if (clock_gettime(CLOCK_MONOTONIC, &now_ts) != 0) {
            (void)kill(child, SIGKILL);
            (void)waitpid(child, &status, 0);
            TEST_FAIL_MESSAGE("clock_gettime failed");
        }
        long long now_ns = (long long)now_ts.tv_sec * 1000000000LL + now_ts.tv_nsec;
        if (now_ns >= deadline_ns) {
            break;
        }

        long long remaining_ns = deadline_ns - now_ns;
        long sleep_ns = (remaining_ns < wait_step_ns) ? (long)remaining_ns : wait_step_ns;
        struct timespec req = { .tv_sec = sleep_ns / 1000000000L, .tv_nsec = sleep_ns % 1000000000L };
        while (nanosleep(&req, &req) != 0 && errno == EINTR) {
        }
    }
    if (waited != child) {
        (void)kill(child, SIGKILL);
        TEST_ASSERT_EQUAL_INT_MESSAGE(child, waitpid(child, &status, 0), "Timed out waiting for child and failed to reap it");
        TEST_FAIL_MESSAGE("Timed out waiting for child process");
    }
    TEST_ASSERT_TRUE(WIFEXITED(status));
    TEST_ASSERT_EQUAL_INT(0, WEXITSTATUS(status));

    TEST_ASSERT_EQUAL_INT(0, remove_pid_file(owned_pid_fd, path));
    owned_pid_fd = -1;
    TEST_ASSERT_EQUAL_INT(0, daemon_status(path));

    int plain_fd = open(path, O_CREAT | O_TRUNC | O_WRONLY, 0644);
    TEST_ASSERT_TRUE(plain_fd >= 0);
    char pid_text[32];
    int pid_len = snprintf(pid_text, sizeof(pid_text), "%ld\n", (long)getpid());
    TEST_ASSERT_TRUE(pid_len > 0);
    TEST_ASSERT_EQUAL_INT(pid_len, write(plain_fd, pid_text, (size_t)pid_len));
    TEST_ASSERT_EQUAL_INT(0, close(plain_fd));
    TEST_ASSERT_EQUAL_INT(0, daemon_status(path));
    TEST_ASSERT_EQUAL_INT(0, unlink(path));

    pid_t supervised = fork();
    TEST_ASSERT_TRUE(supervised >= 0);
    if (supervised == 0) {
        int malformed_fd = open(path, O_CREAT | O_TRUNC | O_RDWR, 0644);
        if (malformed_fd < 0) _exit(10);
        const char *bad_pid_text = "not-a-pid\n";
        if (write(malformed_fd, bad_pid_text, strlen(bad_pid_text)) !=
            (ssize_t)strlen(bad_pid_text) || close(malformed_fd) != 0) _exit(11);
        int ready_pipe[2], release_pipe[2];
        if (pipe(ready_pipe) != 0 || pipe(release_pipe) != 0) _exit(12);
        pid_t locker = fork();
        if (locker < 0) _exit(13);
        if (locker == 0) {
            close(ready_pipe[0]);
            close(release_pipe[1]);
            int lock_fd = open(path, O_RDWR);
            if (lock_fd < 0 || lockf(lock_fd, F_LOCK, 0) != 0) _exit(2);
            if (write(ready_pipe[1], "r", 1) != 1) _exit(3);
            char release;
            if (read(release_pipe[0], &release, 1) != 1) _exit(4);
            close(lock_fd);
            _exit(0);
        }
        close(ready_pipe[1]);
        close(release_pipe[0]);
        char ready;
        int ready_read_rc = (int)read(ready_pipe[0], &ready, 1);
        int daemon_status_rc = ready_read_rc == 1 ? daemon_status(path) : -1;
        int release_write_rc = ready_read_rc == 1 ?
            (int)write(release_pipe[1], "x", 1) : -1;
        close(ready_pipe[0]);
        close(release_pipe[1]);
        pid_t waited = waitpid(locker, &status, 0);

        if (ready_read_rc != 1 || daemon_status_rc != -1 ||
            release_write_rc != 1 || waited != locker ||
            !WIFEXITED(status) || WEXITSTATUS(status) != 0 ||
            unlink(path) != 0) _exit(14);

        int second_fd = write_pid_file(path);
        if (second_fd < 0 || remove_pid_file(second_fd, path) != 0 ||
            unlink(path) != 0) _exit(15);
        _exit(0);
    } else {
        int supervised_status = 0;
        struct timespec supervised_start_ts;
        struct timespec supervised_now_ts;
        const long long timeout_ns = 3000000000LL;
        pid_t wait_rc;

        if (clock_gettime(CLOCK_MONOTONIC, &supervised_start_ts) != 0) {
            (void)kill(supervised, SIGKILL);
            (void)waitpid(supervised, &supervised_status, 0);
            TEST_FAIL_MESSAGE("clock_gettime failed before PID lock wait");
        }
        while (1) {
            wait_rc = waitpid(supervised, &supervised_status, WNOHANG);
            if (wait_rc == supervised) break;
            if (wait_rc < 0 && errno == EINTR) continue;
            if (wait_rc < 0) break;

            usleep(10000);
            if (clock_gettime(CLOCK_MONOTONIC, &supervised_now_ts) != 0) {
                (void)kill(supervised, SIGKILL);
                (void)waitpid(supervised, &supervised_status, 0);
                TEST_FAIL_MESSAGE("clock_gettime failed during PID lock wait");
            }
            long long elapsed_ns = (long long)(supervised_now_ts.tv_sec - supervised_start_ts.tv_sec) * 1000000000LL
                                 + (supervised_now_ts.tv_nsec - supervised_start_ts.tv_nsec);
            if (elapsed_ns >= timeout_ns) break;
        }

        if (wait_rc == 0) {
            kill(supervised, SIGKILL);
            (void)waitpid(supervised, &supervised_status, 0);
            TEST_FAIL_MESSAGE("lock-regression scenario timed out");
        }

        TEST_ASSERT_EQUAL_INT(supervised, wait_rc);
        TEST_ASSERT_TRUE(WIFEXITED(supervised_status));
        TEST_ASSERT_EQUAL_INT(0, WEXITSTATUS(supervised_status));
    }
    TEST_ASSERT_EQUAL_INT(0, rmdir(directory));
}

void test_daemon_status_validates_opened_pid_file(void) {
    char directory[] = "/tmp/lightnvr_pid_file_validation_XXXXXX";
    TEST_ASSERT_NOT_NULL(mkdtemp(directory));
    char path[MAX_PATH_LENGTH], alias[MAX_PATH_LENGTH];
    snprintf(path, sizeof(path), "%s/lightnvr.pid", directory);
    snprintf(alias, sizeof(alias), "%s/alias.pid", directory);

    int fd = open(path, O_WRONLY | O_CREAT | O_EXCL, 0600);
    TEST_ASSERT_TRUE(fd >= 0);
    TEST_ASSERT_EQUAL_INT(0, close(fd));
    TEST_ASSERT_EQUAL_INT(0, daemon_status(path));

    TEST_ASSERT_EQUAL_INT(0, link(path, alias));
    TEST_ASSERT_EQUAL_INT(-1, daemon_status(path));
    TEST_ASSERT_EQUAL_INT(-1, daemon_status(alias));
    TEST_ASSERT_EQUAL_INT(0, unlink(alias));

    TEST_ASSERT_EQUAL_INT(0, symlink(path, alias));
    TEST_ASSERT_EQUAL_INT(-1, daemon_status(alias));
    TEST_ASSERT_EQUAL_INT(0, unlink(alias));
    TEST_ASSERT_EQUAL_INT(0, unlink(path));
    TEST_ASSERT_EQUAL_INT(0, rmdir(directory));
}

void test_default_config_mp4_faststart_on(void) {
    load_default_config(&cfg);
    TEST_ASSERT_TRUE(config_get_mp4_faststart(&cfg));
}

/* Settings saved through the web UI must survive a config reload. */
void test_save_config_round_trips_web_cache_and_mp4_faststart(void) {
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
    snprintf(cfg.db_post_backup_script, sizeof(cfg.db_post_backup_script),
             "%s/post-backup.sh", dir);
    snprintf(cfg.web_root, sizeof(cfg.web_root), "%s/web", dir);
    TEST_ASSERT_EQUAL_INT(0, ensure_dir(cfg.web_root));
    config_set_mp4_faststart(&cfg, false);
    cfg.web_compression_enabled = false;
    cfg.web_use_minified_assets = false;
    cfg.web_cache_max_age_html = 11;
    cfg.web_cache_max_age_css = 22;
    cfg.web_cache_max_age_js = 33;
    cfg.web_cache_max_age_images = 44;
    cfg.web_cache_max_age_fonts = 55;
    cfg.web_cache_max_age_default = 66;
    TEST_ASSERT_EQUAL_INT(0, save_config(&cfg, config_path));

    memset(&reloaded_cfg, 0, sizeof(reloaded_cfg));
    set_custom_config_path(config_path);
    TEST_ASSERT_EQUAL_INT(0, load_config(&reloaded_cfg));
    TEST_ASSERT_FALSE(config_get_mp4_faststart(&reloaded_cfg));
    TEST_ASSERT_FALSE(reloaded_cfg.web_compression_enabled);
    TEST_ASSERT_FALSE(reloaded_cfg.web_use_minified_assets);
    TEST_ASSERT_EQUAL_INT(11, reloaded_cfg.web_cache_max_age_html);
    TEST_ASSERT_EQUAL_INT(22, reloaded_cfg.web_cache_max_age_css);
    TEST_ASSERT_EQUAL_INT(33, reloaded_cfg.web_cache_max_age_js);
    TEST_ASSERT_EQUAL_INT(44, reloaded_cfg.web_cache_max_age_images);
    TEST_ASSERT_EQUAL_INT(55, reloaded_cfg.web_cache_max_age_fonts);
    TEST_ASSERT_EQUAL_INT(66, reloaded_cfg.web_cache_max_age_default);
    TEST_ASSERT_EQUAL_STRING(cfg.db_post_backup_script,
                             reloaded_cfg.db_post_backup_script);

    TEST_ASSERT_EQUAL_INT(0, unlink(config_path));
    if (access(cfg.log_file, F_OK) == 0)
        TEST_ASSERT_EQUAL_INT(0, unlink(cfg.log_file));
    if (access(cfg.storage_path, F_OK) == 0)
        TEST_ASSERT_EQUAL_INT(0, rmdir(cfg.storage_path));
    if (access(cfg.mp4_storage_path, F_OK) == 0)
        TEST_ASSERT_EQUAL_INT(0, rmdir(cfg.mp4_storage_path));
    if (access(cfg.storage_path_hls, F_OK) == 0)
        TEST_ASSERT_EQUAL_INT(0, rmdir(cfg.storage_path_hls));
    if (access(cfg.models_path, F_OK) == 0)
        TEST_ASSERT_EQUAL_INT(0, rmdir(cfg.models_path));
    TEST_ASSERT_EQUAL_INT(0, rmdir(cfg.web_root));
    TEST_ASSERT_EQUAL_INT(0, rmdir(dir));
}

static atomic_bool faststart_writer_stop;
static atomic_int faststart_writer_updates;

typedef struct {
    uintptr_t config_address;
    uintptr_t streams_address;
} snapshot_addresses_t;

static void *capture_streaming_snapshot_addresses(void *argument) {
    snapshot_addresses_t *addresses = argument;
    config_t *snapshot = get_streaming_config();
    addresses->config_address = (uintptr_t)snapshot;
    addresses->streams_address = (uintptr_t)snapshot->streams;
    return NULL;
}

void test_streaming_config_snapshots_are_thread_local(void) {
    load_default_config(&g_config);
    config_t *main_snapshot = get_streaming_config();
    TEST_ASSERT_NOT_EQUAL_PTR(&g_config, main_snapshot);
    TEST_ASSERT_NOT_NULL(main_snapshot->streams);
    TEST_ASSERT_EQUAL_INT(g_config.max_streams, main_snapshot->max_streams);
    uintptr_t main_config_address = (uintptr_t)main_snapshot;
    uintptr_t main_streams_address = (uintptr_t)main_snapshot->streams;
    snapshot_addresses_t worker_addresses = {0};
    pthread_t worker;
    TEST_ASSERT_EQUAL_INT(0, pthread_create(&worker, NULL,
                         capture_streaming_snapshot_addresses, &worker_addresses));
    TEST_ASSERT_EQUAL_INT(0, pthread_join(worker, NULL));
    TEST_ASSERT_TRUE(main_config_address != worker_addresses.config_address);
    TEST_ASSERT_TRUE(main_streams_address != worker_addresses.streams_address);
    TEST_ASSERT_EQUAL_PTR(main_snapshot, get_streaming_config());
}

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

/* Verify config_copy() performs a deep copy and that snapshot-owned streams
 * remain valid after the source config is reloaded. */
void test_config_copy_owns_streams_after_source_reload(void) {
    load_default_config(&cfg);
    snprintf(cfg.streams[0].name, sizeof(cfg.streams[0].name), "original-camera");
    config_t snapshot = {0};
    TEST_ASSERT_TRUE(config_copy(&snapshot, &cfg));
    owned_snapshot_streams = snapshot.streams;
    TEST_ASSERT_NOT_EQUAL_PTR(cfg.streams, snapshot.streams);
    TEST_ASSERT_EQUAL_STRING("original-camera", snapshot.streams[0].name);
    load_default_config(&cfg);
    TEST_ASSERT_EQUAL_STRING("original-camera", snapshot.streams[0].name);
}

void test_config_reset_stream_slots_clears_stale_rows(void) {
    load_default_config(&cfg);
    snprintf(cfg.streams[0].name, sizeof(cfg.streams[0].name), "deleted-camera");
    cfg.streams[0].streaming_enabled = false;
    TEST_ASSERT_EQUAL_INT(0, config_reset_stream_slots(&cfg));
    TEST_ASSERT_EQUAL_STRING("", cfg.streams[0].name);
    TEST_ASSERT_TRUE(cfg.streams[0].streaming_enabled);
    TEST_ASSERT_EQUAL_INT(10, cfg.streams[0].detection_interval);
}

void test_save_stream_configs_preserves_identities_after_batch_reload(void) {
    if (DEFAULT_MAX_STREAMS < 2) {
        TEST_IGNORE_MESSAGE("Requires at least two default stream slots");
    }
    char db_path[] = "/tmp/lightnvr_stream_save_XXXXXX";
    int fd = mkstemp(db_path);
    TEST_ASSERT_TRUE(fd >= 0);
    close(fd);
    snprintf(stream_save_db_path, sizeof(stream_save_db_path), "%s", db_path);
    TEST_ASSERT_EQUAL_INT(0, init_database_ex(db_path, DB_INIT_NO_BACKUP));
    stream_save_db_active = true;
    char backup_path[MAX_PATH_LENGTH + 5];
    snprintf(backup_path, sizeof(backup_path), "%s.bak", db_path);
    TEST_ASSERT_EQUAL_INT(-1, access(backup_path, F_OK));

    load_default_config(&cfg);
    snprintf(cfg.streams[0].name, sizeof(cfg.streams[0].name), "camera-one");
    snprintf(cfg.streams[0].url, sizeof(cfg.streams[0].url), "rtsp://example.invalid/one");
    TEST_ASSERT_EQUAL_INT(1, save_stream_configs(&cfg));
    TEST_ASSERT_TRUE(cfg.streams[0].camera_uuid[0] != '\0');
    char original_uuid[sizeof(cfg.streams[0].camera_uuid)];
    snprintf(original_uuid, sizeof(original_uuid), "%s", cfg.streams[0].camera_uuid);

    snprintf(cfg.streams[1].name, sizeof(cfg.streams[1].name), "camera-two");
    snprintf(cfg.streams[1].url, sizeof(cfg.streams[1].url), "rtsp://example.invalid/two");
    TEST_ASSERT_EQUAL_INT(2, save_stream_configs(&cfg));
    TEST_ASSERT_EQUAL_STRING(original_uuid, cfg.streams[0].camera_uuid);
    TEST_ASSERT_TRUE(cfg.streams[1].camera_uuid[0] != '\0');
}

void test_mp4_faststart_concurrent_reads_copies_and_saves(void) {
    char temp_dir[] = "/tmp/lightnvr_faststart_concurrent_XXXXXX";
    char *dir = mkdtemp(temp_dir);
    TEST_ASSERT_NOT_NULL_MESSAGE(dir, "mkdtemp failed for concurrency timeout harness");
    snprintf(concurrent_config_dir, sizeof(concurrent_config_dir), "%s", dir);
    char path[MAX_PATH_LENGTH];
    snprintf(path, sizeof(path), "%s/settings.ini", dir);

    pid_t child = fork();
    TEST_ASSERT_TRUE_MESSAGE(child >= 0, "fork failed for concurrency timeout harness");

    if (child == 0) {
        load_default_config(&cfg);

        atomic_store(&faststart_writer_stop, false);
        atomic_store(&faststart_writer_updates, 0);
        pthread_t writer;
        if (pthread_create(&writer, NULL, toggle_faststart, NULL) != 0) {
            unlink(path);
            rmdir(dir);
            _exit(EXIT_FAILURE);
        }
        int save_result = 0;
        int valid_saves = 0;
        bool streams_distinct_ok = true;
        bool writer_progress_ok = true;
        struct timespec initial_wait_start = {0};
        if (clock_gettime(CLOCK_MONOTONIC, &initial_wait_start) != 0) {
            writer_progress_ok = false;
        }
        while (writer_progress_ok && atomic_load(&faststart_writer_updates) == 0) {
            struct timespec now = {0};
            if (clock_gettime(CLOCK_MONOTONIC, &now) != 0) {
                writer_progress_ok = false;
                break;
            }
            long elapsed_ms =
                (now.tv_sec - initial_wait_start.tv_sec) * 1000L +
                (now.tv_nsec - initial_wait_start.tv_nsec) / 1000000L;
            if (elapsed_ms > 2000L) {
                writer_progress_ok = false;
                break;
            }
            sched_yield();
        }
        for (int i = 0; i < 50 && writer_progress_ok; ++i) {
        (void)config_get_mp4_faststart(&cfg);
        config_t snapshot = {0};
        if (!config_copy(&snapshot, &cfg)) streams_distinct_ok = false;
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
        int mp4_faststart_entries = 0;
        bool file_has_valid_faststart = true;
        while (fgets(line, sizeof(line), file)) {
            if (strncmp(line, "mp4_faststart", strlen("mp4_faststart")) == 0) {
                const char *prefix = "mp4_faststart = ";
                size_t prefix_len = strlen(prefix);
                if (strncmp(line, prefix, prefix_len) != 0) {
                    file_has_valid_faststart = false;
                    break;
                }
                const char *value = line + prefix_len;
                const char *suffix = NULL;
                if (strncmp(value, "true", 4) == 0) suffix = value + 4;
                else if (strncmp(value, "false", 5) == 0) suffix = value + 5;
                if (suffix) {
                    while (*suffix == ' ' || *suffix == '\t') suffix++;
                }
                if (suffix && (*suffix == ';' || *suffix == '\n' ||
                               *suffix == '\r' || *suffix == '\0')) {
                    ++mp4_faststart_entries;
                } else {
                    file_has_valid_faststart = false;
                    break;
                }
            }
        }
        fclose(file);
        if (file_has_valid_faststart && mp4_faststart_entries == 1) {
            ++valid_saves;
        } else {
            save_result = -1;
            break;
        }
        int progress_baseline = atomic_load(&faststart_writer_updates);
        bool observed_progress = false;
        struct timespec deadline, now;
        if (clock_gettime(CLOCK_MONOTONIC, &deadline) != 0) {
            writer_progress_ok = false;
            save_result = -1;
            break;
        }
        deadline.tv_nsec += 250000000L;
        if (deadline.tv_nsec >= 1000000000L) {
            deadline.tv_sec++;
            deadline.tv_nsec -= 1000000000L;
        }
        for (;;) {
            if (atomic_load(&faststart_writer_updates) > progress_baseline) {
                observed_progress = true;
                break;
            }
            if (clock_gettime(CLOCK_MONOTONIC, &now) != 0) break;
            if (now.tv_sec > deadline.tv_sec ||
                (now.tv_sec == deadline.tv_sec && now.tv_nsec >= deadline.tv_nsec)) break;
            sched_yield();
        }
        if (!observed_progress) {
            writer_progress_ok = false;
            save_result = -1;
            break;
        }
    }
        atomic_store(&faststart_writer_stop, true);
        int join_result = pthread_join(writer, NULL);
        unlink(path);
        rmdir(dir);
        if (join_result != 0 || !writer_progress_ok || !streams_distinct_ok ||
            save_result != 0 || valid_saves != 50) {
            _exit(EXIT_FAILURE);
        }

        config_set_mp4_faststart(&cfg, false);
        config_t snapshot = {0};
        if (!config_copy(&snapshot, &cfg)) {
            _exit(EXIT_FAILURE);
        }
        owned_snapshot_streams = snapshot.streams;
        config_set_mp4_faststart(&cfg, true);
        if (config_get_mp4_faststart(&snapshot) ||
            !config_get_mp4_faststart(&cfg) ||
            cfg.streams == snapshot.streams ||
            cfg.max_streams != snapshot.max_streams ||
            memcmp(cfg.streams, snapshot.streams,
                   (size_t)cfg.max_streams * sizeof(*cfg.streams)) != 0) {
            _exit(EXIT_FAILURE);
        }
        _exit(EXIT_SUCCESS);
    }

    struct timespec start = {0};
    if (clock_gettime(CLOCK_MONOTONIC, &start) != 0) {
        kill(child, SIGKILL);
        waitpid(child, NULL, 0);
        TEST_FAIL_MESSAGE("clock_gettime failed in timeout harness");
    }
    for (;;) {
        int status = 0;
        pid_t waited = waitpid(child, &status, WNOHANG);
        if (waited == child) {
            TEST_ASSERT_TRUE_MESSAGE(WIFEXITED(status), "child did not exit normally");
            TEST_ASSERT_EQUAL_INT_MESSAGE(EXIT_SUCCESS, WEXITSTATUS(status),
                                          "child reported failure in concurrency scenario");
            break;
        }
        if (waited == -1) {
            if (errno == EINTR) {
                continue;
            }
            kill(child, SIGKILL);
            waitpid(child, NULL, 0);
            TEST_FAIL_MESSAGE("waitpid failed while monitoring child");
        }

        struct timespec now = {0};
        if (clock_gettime(CLOCK_MONOTONIC, &now) != 0) {
            kill(child, SIGKILL);
            waitpid(child, NULL, 0);
            TEST_FAIL_MESSAGE("clock_gettime failed during timeout monitoring");
        }
        long elapsed_ms =
            (now.tv_sec - start.tv_sec) * 1000L + (now.tv_nsec - start.tv_nsec) / 1000000L;
        if (elapsed_ms > 8000L) {
            kill(child, SIGKILL);
            waitpid(child, NULL, 0);
            TEST_FAIL_MESSAGE("concurrency scenario timed out (possible deadlock regression)");
        }
        struct timespec poll_delay = {.tv_sec = 0, .tv_nsec = 1000000L}; /* 1 ms */
        nanosleep(&poll_delay, NULL);
    }
    unlink(path);
    rmdir(dir);
    concurrent_config_dir[0] = '\0';
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
        cfg.storage_pressure_critical_pct = 12.0;
        cfg.storage_pressure_warning_pct = 24.0;
        cfg.storage_pressure_emergency_pct = invalid[i];
        TEST_ASSERT_EQUAL_INT(0, validate_config(&cfg));
        TEST_ASSERT_TRUE(cfg.storage_pressure_emergency_pct == 5.0);
        TEST_ASSERT_TRUE(cfg.storage_pressure_critical_pct == 10.0);
        TEST_ASSERT_TRUE(cfg.storage_pressure_warning_pct == 20.0);
    }

    for (size_t i = 0; i < sizeof(invalid) / sizeof(invalid[0]); i++) {
        load_default_config(&cfg);
        cfg.storage_pressure_emergency_pct = 6.0;
        cfg.storage_pressure_warning_pct = 24.0;
        cfg.storage_pressure_critical_pct = invalid[i];
        TEST_ASSERT_EQUAL_INT(0, validate_config(&cfg));
        TEST_ASSERT_TRUE(cfg.storage_pressure_emergency_pct == 5.0);
        TEST_ASSERT_TRUE(cfg.storage_pressure_critical_pct == 10.0);
        TEST_ASSERT_TRUE(cfg.storage_pressure_warning_pct == 20.0);
    }

    for (size_t i = 0; i < sizeof(invalid) / sizeof(invalid[0]); i++) {
        load_default_config(&cfg);
        cfg.storage_pressure_emergency_pct = 6.0;
        cfg.storage_pressure_critical_pct = 12.0;
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

    load_default_config(&cfg);
    snprintf(cfg.streams[0].name, sizeof(cfg.streams[0].name), "preserve-camera");
    config_set_mp4_faststart(&cfg, false);
    stream_config_t *original_streams = cfg.streams;
    set_custom_config_path(config_path);
    TEST_ASSERT_EQUAL_INT(-1, load_config(&cfg));
    TEST_ASSERT_EQUAL_INT(DEFAULT_MAX_STREAMS, cfg.max_streams);
    TEST_ASSERT_EQUAL_PTR(original_streams, cfg.streams);
    TEST_ASSERT_EQUAL_STRING("preserve-camera", cfg.streams[0].name);
    TEST_ASSERT_FALSE(config_get_mp4_faststart(&cfg));
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

void test_go2rtc_ini_rejects_invalid_ports(void) {
    char directory[] = "/tmp/lightnvr_go2rtc_ports_XXXXXX";
    TEST_ASSERT_NOT_NULL(mkdtemp(directory));
    snprintf(concurrent_config_dir, sizeof(concurrent_config_dir), "%s", directory);
    char path[MAX_PATH_LENGTH];
    snprintf(path, sizeof(path), "%s/settings.ini", directory);
    set_custom_config_path(path);

    const char *invalid_lines[] = {
        "api_port = -1\n",
        "rtsp_port = 65536\n",
        "webrtc_listen_port = abc\n",
    };
    for (size_t i = 0; i < sizeof(invalid_lines) / sizeof(invalid_lines[0]); i++) {
        FILE *file = fopen(path, "w");
        TEST_ASSERT_NOT_NULL(file);
        TEST_ASSERT_TRUE(fputs("[go2rtc]\n", file) >= 0);
        TEST_ASSERT_TRUE(fputs(invalid_lines[i], file) >= 0);
        TEST_ASSERT_EQUAL_INT(0, fclose(file));
        TEST_ASSERT_EQUAL_INT(-1, load_config(&cfg));
    }
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

    /* Empty and NULL inputs must not overwrite an existing stored value. */
    set_custom_config_path("");
    TEST_ASSERT_EQUAL_STRING("/tmp/test_lightnvr.ini", get_custom_config_path());
    set_custom_config_path(NULL);
    TEST_ASSERT_EQUAL_STRING("/tmp/test_lightnvr.ini", get_custom_config_path());

    set_custom_config_path("/tmp/..data/settings.ini");
    TEST_ASSERT_EQUAL_STRING("/tmp/..data/settings.ini", get_custom_config_path());
    set_custom_config_path("/tmp/../escape.ini");
    TEST_ASSERT_EQUAL_STRING("/tmp/..data/settings.ini", get_custom_config_path());
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

    saved_config_file = fopen(config_path, "r");
    TEST_ASSERT_NOT_NULL(saved_config_file);
    if (saved_config_file) {
        TEST_ASSERT_EQUAL_INT(0, fseek(saved_config_file, 0, SEEK_END));
        long file_size = ftell(saved_config_file);
        TEST_ASSERT_GREATER_THAN(0, file_size);
        rewind(saved_config_file);

        saved_config_text = malloc((size_t)file_size + 1U);
        TEST_ASSERT_NOT_NULL(saved_config_text);
        if (saved_config_text) {
            size_t bytes_read = fread(saved_config_text, 1, (size_t)file_size, saved_config_file);
            TEST_ASSERT_EQUAL_size_t((size_t)file_size, bytes_read);
            saved_config_text[bytes_read] = '\0';
            TEST_ASSERT_NOT_NULL(strstr(saved_config_text, "; LightNVR Configuration File"));
            TEST_ASSERT_NOT_NULL(strstr(saved_config_text, "backup_interval_minutes = 15"));
            TEST_ASSERT_NOT_NULL(strstr(saved_config_text, "backup_retention_count = 8"));
            TEST_ASSERT_NOT_NULL(strstr(saved_config_text, "post_backup_script = /usr/local/bin/post-backup"));
            TEST_ASSERT_NOT_NULL(strstr(saved_config_text, "mp4_directory_format = year_month_day"));
            TEST_ASSERT_NOT_NULL(strstr(saved_config_text, "audio_disabled = true"));
            TEST_ASSERT_NOT_NULL(strstr(saved_config_text, "auto_disabled = true"));
            TEST_ASSERT_NOT_NULL(strstr(saved_config_text, "[health]"));
            TEST_ASSERT_NOT_NULL(strstr(saved_config_text, "profile = conservative"));
            TEST_ASSERT_NOT_NULL(strstr(saved_config_text,
                                        "presence_interval_seconds = 120"));
            free(saved_config_text);
            saved_config_text = NULL;
        }
        fclose(saved_config_file);
        saved_config_file = NULL;
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

    saved_config_file = fopen(config_path, "r");
    TEST_ASSERT_NOT_NULL(saved_config_file);
    if (saved_config_file) {
        TEST_ASSERT_EQUAL_INT(0, fseek(saved_config_file, 0, SEEK_END));
        long file_size = ftell(saved_config_file);
        TEST_ASSERT_GREATER_THAN(0, (int)file_size);
        rewind(saved_config_file);

        saved_config_text = malloc((size_t)file_size + 1);
        TEST_ASSERT_NOT_NULL(saved_config_text);
        if (saved_config_text) {
            size_t bytes_read = fread(saved_config_text, 1, (size_t)file_size, saved_config_file);
            TEST_ASSERT_EQUAL_size_t((size_t)file_size, bytes_read);
            saved_config_text[bytes_read] = '\0';
            TEST_ASSERT_NOT_NULL(strstr(saved_config_text, "[api_detection]"));
            TEST_ASSERT_NOT_NULL(strstr(saved_config_text, "format = doods2"));
            TEST_ASSERT_NOT_NULL(strstr(saved_config_text, "detector_name = tensorflow"));
        }
        fclose(saved_config_file);
        saved_config_file = NULL;
        free(saved_config_text);
        saved_config_text = NULL;
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

    char storage_path[MAX_PATH_LENGTH];
    char hls_path[MAX_PATH_LENGTH];
    char models_path[MAX_PATH_LENGTH];
    char log_path[MAX_PATH_LENGTH];
    snprintf(storage_path, sizeof(storage_path), "%s/storage", dir);
    snprintf(hls_path, sizeof(hls_path), "%s/hls", dir);
    snprintf(models_path, sizeof(models_path), "%s/models", dir);
    snprintf(log_path, sizeof(log_path), "%s/lightnvr.log", dir);

    TEST_ASSERT_EQUAL_INT(0, unlink(config_path));
    TEST_ASSERT_EQUAL_INT(0, unlink(log_path));
    TEST_ASSERT_EQUAL_INT(0, rmdir(hls_path));
    TEST_ASSERT_EQUAL_INT(0, rmdir(storage_path));
    TEST_ASSERT_EQUAL_INT(0, rmdir(models_path));
    TEST_ASSERT_EQUAL_INT(0, rmdir(web_root));
    TEST_ASSERT_EQUAL_INT(0, rmdir(dir));
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

    TEST_ASSERT_EQUAL_INT(0, setenv("LIGHTNVR_GO2RTC_WEBRTC_LISTEN_PORT", "65536", 1));
    TEST_ASSERT_EQUAL_INT(0, load_config(&cfg));
    TEST_ASSERT_EQUAL_INT(8555, cfg.go2rtc_webrtc_listen_port);
    TEST_ASSERT_EQUAL_INT(0, setenv("LIGHTNVR_GO2RTC_WEBRTC_LISTEN_PORT", "0", 1));
    TEST_ASSERT_EQUAL_INT(0, load_config(&cfg));
    TEST_ASSERT_EQUAL_INT(0, cfg.go2rtc_webrtc_listen_port);

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
    snprintf(owned_private_config_dir, sizeof(owned_private_config_dir), "%s", directory);
    char path[MAX_PATH_LENGTH];
    snprintf(path, sizeof(path), "%s/settings.ini", directory);
    snprintf(owned_private_config_path, sizeof(owned_private_config_path), "%s", path);
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
    snprintf(group_replace_dir, sizeof(group_replace_dir), "%s", directory);
    snprintf(group_replace_path, sizeof(group_replace_path), "%s", path);
    group_replace_cleanup_active = true;
    FILE *old = fopen(path, "w");
    TEST_ASSERT_NOT_NULL(old);
    fputs("preserve-old-content", old); fclose(old);
    TEST_ASSERT_EQUAL_INT(0, chmod(path, 0660));
    struct stat before, after;
    TEST_ASSERT_EQUAL_INT(0, stat(path, &before));
    deny_config_chmod_target_dev = before.st_dev;
    deny_config_chmod_target_ino = before.st_ino;
    deny_config_chmod_target_set = true;
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
    group_replace_cleanup_active = false;
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

static void test_save_config_rejects_unsafe_credentials_before_writing(void) {
    char directory[] = "/tmp/lightnvr_config_credentials_XXXXXX";
    TEST_ASSERT_NOT_NULL(mkdtemp(directory));
    char path[MAX_PATH_LENGTH];
    char control_path[MAX_PATH_LENGTH];
    snprintf(path, sizeof(path), "%s/settings.ini", directory);
    snprintf(control_path, sizeof(control_path), "%s/control.ini", directory);
    snprintf(owned_credential_config_dir, sizeof(owned_credential_config_dir), "%s", directory);
    snprintf(owned_credential_config_path, sizeof(owned_credential_config_path), "%s", path);
    snprintf(owned_credential_control_path, sizeof(owned_credential_control_path), "%s", control_path);
    FILE *initial = fopen(path, "w");
    TEST_ASSERT_NOT_NULL(initial);
    fputs("previous configuration", initial);
    fclose(initial);

    load_default_config(&cfg);
    TEST_ASSERT_EQUAL_INT(0, save_config(&cfg, control_path));

    load_default_config(&cfg);
    snprintf(cfg.turn_username, sizeof(cfg.turn_username), "user;admin");
    TEST_ASSERT_EQUAL_INT(-1, save_config(&cfg, path));

    load_default_config(&cfg);
    snprintf(cfg.turn_password, sizeof(cfg.turn_password), "secret\n[web]");
    TEST_ASSERT_EQUAL_INT(-1, save_config(&cfg, path));

    load_default_config(&cfg);
    snprintf(cfg.mqtt_password, sizeof(cfg.mqtt_password), "secret#comment");
    TEST_ASSERT_EQUAL_INT(-1, save_config(&cfg, path));

    load_default_config(&cfg);
    snprintf(cfg.mqtt_username, sizeof(cfg.mqtt_username), "user;admin");
    TEST_ASSERT_EQUAL_INT(-1, save_config(&cfg, path));

    load_default_config(&cfg);
    snprintf(cfg.turn_password, sizeof(cfg.turn_password), " leading-space");
    TEST_ASSERT_EQUAL_INT(-1, save_config(&cfg, path));

    load_default_config(&cfg);
    snprintf(cfg.mqtt_username, sizeof(cfg.mqtt_username), "trailing-space ");
    TEST_ASSERT_EQUAL_INT(-1, save_config(&cfg, path));
    cfg.mqtt_username[0] = '\0';
    snprintf(cfg.web_username, sizeof(cfg.web_username), "admin#spoofed");
    TEST_ASSERT_EQUAL_INT(-1, save_config(&cfg, path));
    memset(cfg.web_username, 'x', sizeof(cfg.web_username));
    TEST_ASSERT_EQUAL_INT(-1, save_config(&cfg, path));
    snprintf(cfg.web_username, sizeof(cfg.web_username), "admin");
    snprintf(cfg.mqtt_client_id, sizeof(cfg.mqtt_client_id), "client#spoofed");
    TEST_ASSERT_EQUAL_INT(-1, save_config(&cfg, path));
    snprintf(cfg.mqtt_client_id, sizeof(cfg.mqtt_client_id), "lightnvr");
    snprintf(cfg.mqtt_topic_prefix, sizeof(cfg.mqtt_topic_prefix), "topic#spoofed");
    TEST_ASSERT_EQUAL_INT(-1, save_config(&cfg, path));
    snprintf(cfg.mqtt_topic_prefix, sizeof(cfg.mqtt_topic_prefix), "lightnvr");
    snprintf(cfg.db_post_backup_script, sizeof(cfg.db_post_backup_script), "/bin/true\n[web]");
    TEST_ASSERT_EQUAL_INT(-1, save_config(&cfg, path));
    cfg.db_post_backup_script[0] = '\0';
    snprintf(cfg.mqtt_ha_discovery_prefix, sizeof(cfg.mqtt_ha_discovery_prefix), "homeassistant#bad");
    TEST_ASSERT_EQUAL_INT(-1, save_config(&cfg, path));
    snprintf(cfg.mqtt_ha_discovery_prefix, sizeof(cfg.mqtt_ha_discovery_prefix), "homeassistant");
    snprintf(cfg.turn_server_url, sizeof(cfg.turn_server_url), "turn:example.com\n[web]");
    TEST_ASSERT_EQUAL_INT(-1, save_config(&cfg, path));
    cfg.turn_server_url[0] = '\0';
    snprintf(cfg.mqtt_broker_host, sizeof(cfg.mqtt_broker_host), "broker#bad");
    TEST_ASSERT_EQUAL_INT(-1, save_config(&cfg, path));
    cfg.mqtt_broker_host[0] = '\0';
    snprintf(cfg.api_detection_detector_name, sizeof(cfg.api_detection_detector_name), "model#bad");
    TEST_ASSERT_EQUAL_INT(-1, save_config(&cfg, path));
    snprintf(cfg.api_detection_detector_name, sizeof(cfg.api_detection_detector_name), "default");
    snprintf(cfg.pid_file, sizeof(cfg.pid_file), "/tmp/lightnvr.pid\n[web]");
    TEST_ASSERT_EQUAL_INT(-1, save_config(&cfg, path));
    snprintf(cfg.pid_file, sizeof(cfg.pid_file), "/tmp/lightnvr.pid");
    snprintf(cfg.api_detection_url, sizeof(cfg.api_detection_url), "http://example/#bad");
    TEST_ASSERT_EQUAL_INT(-1, save_config(&cfg, path));
    cfg.api_detection_url[0] = '\0';
    snprintf(cfg.go2rtc_config_dir, sizeof(cfg.go2rtc_config_dir), "/etc/lightnvr\n[web]");
    TEST_ASSERT_EQUAL_INT(-1, save_config(&cfg, path));
    snprintf(cfg.go2rtc_config_dir, sizeof(cfg.go2rtc_config_dir), "/etc/lightnvr");
    snprintf(cfg.go2rtc_stun_server, sizeof(cfg.go2rtc_stun_server), "stun.example#bad");
    TEST_ASSERT_EQUAL_INT(-1, save_config(&cfg, path));
    cfg.go2rtc_stun_server[0] = '\0';
    snprintf(cfg.go2rtc_ice_servers, sizeof(cfg.go2rtc_ice_servers), "stun:example\n[web]");
    TEST_ASSERT_EQUAL_INT(-1, save_config(&cfg, path));
    cfg.go2rtc_ice_servers[0] = '\0';
    snprintf(cfg.onvif_discovery_network, sizeof(cfg.onvif_discovery_network),
             "192.168.1.0/24\n[web]");
    TEST_ASSERT_EQUAL_INT(-1, save_config(&cfg, path));
    snprintf(cfg.onvif_discovery_network, sizeof(cfg.onvif_discovery_network), "auto");
    snprintf(cfg.mp4_storage_path, sizeof(cfg.mp4_storage_path), "/tmp/recordings\n[web]");
    TEST_ASSERT_EQUAL_INT(-1, save_config(&cfg, path));
    cfg.mp4_storage_path[0] = '\0';
    snprintf(cfg.swap_file, sizeof(cfg.swap_file), "/tmp/swap#bad");
    TEST_ASSERT_EQUAL_INT(-1, save_config(&cfg, path));
    cfg.swap_file[0] = '\0';
    snprintf(cfg.hw_accel_device, sizeof(cfg.hw_accel_device), "/dev/video0\n[web]");
    TEST_ASSERT_EQUAL_INT(-1, save_config(&cfg, path));
    cfg.hw_accel_device[0] = '\0';
    snprintf(cfg.web_bind_ip, sizeof(cfg.web_bind_ip), "127.0.0.1\n[web]");
    TEST_ASSERT_EQUAL_INT(-1, save_config(&cfg, path));
    cfg.web_bind_ip[0] = '\0';
    snprintf(cfg.web_ssl_cert_path, sizeof(cfg.web_ssl_cert_path), "/tmp/cert.pem#bad");
    TEST_ASSERT_EQUAL_INT(-1, save_config(&cfg, path));
    cfg.web_ssl_cert_path[0] = '\0';
    snprintf(cfg.web_ssl_key_path, sizeof(cfg.web_ssl_key_path), "/tmp/key.pem\n[web]");
    TEST_ASSERT_EQUAL_INT(-1, save_config(&cfg, path));

    char content[64] = {0};
    FILE *saved = fopen(path, "r");
    TEST_ASSERT_NOT_NULL(saved);
    saved_config_file = saved;
    TEST_ASSERT_EQUAL_UINT(strlen("previous configuration"),
                           fread(content, 1, sizeof(content) - 1, saved));
    fclose(saved);
    saved_config_file = NULL;
    TEST_ASSERT_EQUAL_STRING("previous configuration", content);
    unlink(path);
    unlink(control_path);
    TEST_ASSERT_EQUAL_INT(0, rmdir(directory));
}

static void check_bind_mount_fallback(bool fail_copy) {
    char directory[] = "/tmp/lightnvr_config_bind_XXXXXX";
    TEST_ASSERT_NOT_NULL(mkdtemp(directory));
    char path[MAX_PATH_LENGTH];
    char baseline_path[MAX_PATH_LENGTH];
    const char *original = "previous configuration";
    size_t original_len = strlen(original);
    snprintf(path, sizeof(path), "%s/settings.ini", directory);
    snprintf(baseline_path, sizeof(baseline_path), "%s/settings.baseline.ini", directory);
    owned_bind_dir_active = true;
    snprintf(owned_bind_dir_path, sizeof(owned_bind_dir_path), "%s", directory);
    snprintf(owned_bind_file_path, sizeof(owned_bind_file_path), "%s", path);
    snprintf(owned_bind_baseline_path, sizeof(owned_bind_baseline_path), "%s", baseline_path);
    FILE *initial = fopen(path, "w");
    TEST_ASSERT_NOT_NULL(initial);
    fputs(original, initial);
    fclose(initial);
    TEST_ASSERT_EQUAL_INT(0, chmod(path, 0644));
    struct stat before, after;
    struct stat baseline_stat;
    load_default_config(&cfg);

    if (!fail_copy) {
        TEST_ASSERT_EQUAL_INT(0, save_config(&cfg, baseline_path));
        TEST_ASSERT_EQUAL_INT(0, stat(baseline_path, &baseline_stat));
        int original_fd = open(path, O_WRONLY);
        TEST_ASSERT_TRUE(original_fd >= 0);
        TEST_ASSERT_EQUAL_INT(0, ftruncate(original_fd, baseline_stat.st_size + 128));
        close(original_fd);
    }
    TEST_ASSERT_EQUAL_INT(0, stat(path, &before));
    TEST_ASSERT_TRUE(((before.st_mode & 0777) & 0044) != 0);
    if (!fail_copy) TEST_ASSERT_TRUE(before.st_size > baseline_stat.st_size);

    busy_config_rename = true;
    if (fail_copy) {
        short_config_pwrite = true;
        short_config_pwrite_inode = before.st_ino;
        short_config_pwrite_max_bytes = 8;
        short_config_pwrite_done = false;
    }
    fail_config_pwrite = fail_copy;
    fail_config_pwrite_inode = before.st_ino;
    check_config_pwrite_inode = before.st_ino;
    check_config_pwrite_saw_write = false;
    check_config_pwrite_mode_violation = false;
    check_config_pwrite_mode = true;
    int result = save_config(&cfg, path);
    check_config_pwrite_mode = false;
    TEST_ASSERT_EQUAL_INT(fail_copy ? -1 : 0, result);
    if (!fail_copy) TEST_ASSERT_TRUE(check_config_pwrite_saw_write);
    TEST_ASSERT_FALSE(check_config_pwrite_mode_violation);
    if (fail_copy) TEST_ASSERT_TRUE(short_config_pwrite_done);
    TEST_ASSERT_EQUAL_INT(0, stat(path, &after));
    TEST_ASSERT_EQUAL_UINT(0600, (unsigned)(after.st_mode & 0777));
    TEST_ASSERT_EQUAL_UINT64(before.st_ino, after.st_ino);
    if (fail_copy) TEST_ASSERT_EQUAL_UINT64(before.st_size, after.st_size);
    FILE *saved = fopen(path, "rb");
    TEST_ASSERT_NOT_NULL(saved);
    saved_config_file = saved;
    if (fail_copy) {
        char content[128] = {0};
        size_t count = fread(content, 1, sizeof(content) - 1, saved);
        TEST_ASSERT_EQUAL_UINT(original_len, count);
        TEST_ASSERT_EQUAL_MEMORY(original, content, original_len);
    } else {
        TEST_ASSERT_EQUAL_UINT64((uint64_t)baseline_stat.st_size, (uint64_t)after.st_size);
        FILE *baseline_file = fopen(baseline_path, "rb");
        TEST_ASSERT_NOT_NULL(baseline_file);
        saved_baseline_file = baseline_file;
        assert_files_equal_binary(saved, baseline_file);
        fclose(baseline_file);
        saved_baseline_file = NULL;
    }
    TEST_ASSERT_FALSE(ferror(saved));
    fclose(saved);
    saved_config_file = NULL;
    if (!fail_copy) unlink(baseline_path);
    unlink(path);
    TEST_ASSERT_EQUAL_INT(0, rmdir(directory));
    owned_bind_dir_active = false;
}

static void test_save_config_bind_mount_fallback(void) {
    check_bind_mount_fallback(false);
}

static void test_save_config_bind_mount_restores_after_write_failure(void) {
    check_bind_mount_fallback(true);
}

static void assert_files_equal_binary(FILE *expected, FILE *actual) {
    unsigned char expected_buf[256];
    unsigned char actual_buf[256];
    while (true) {
        size_t expected_n = fread(expected_buf, 1, sizeof(expected_buf), expected);
        size_t actual_n = fread(actual_buf, 1, sizeof(actual_buf), actual);
        TEST_ASSERT_EQUAL_UINT64((uint64_t)expected_n, (uint64_t)actual_n);
        if (expected_n == 0) {
            break;
        }
        TEST_ASSERT_EQUAL_INT(0, memcmp(expected_buf, actual_buf, expected_n));
    }
    TEST_ASSERT_FALSE(ferror(expected));
    TEST_ASSERT_FALSE(ferror(actual));
}

static void test_save_config_bind_mount_retries_short_write(void) {
    char directory[] = "/tmp/lightnvr_config_short_write_XXXXXX";
    TEST_ASSERT_NOT_NULL(mkdtemp(directory));
    char path[MAX_PATH_LENGTH];
    snprintf(path, sizeof(path), "%s/settings.ini", directory);
    snprintf(short_write_dir, sizeof(short_write_dir), "%s", directory);
    snprintf(short_write_path, sizeof(short_write_path), "%s", path);
    FILE *initial = fopen(path, "w");
    TEST_ASSERT_NOT_NULL(initial);
    fputs("previous configuration", initial);
    fclose(initial);
    struct stat before, after;
    TEST_ASSERT_EQUAL_INT(0, stat(path, &before));
    load_default_config(&cfg);
    busy_config_rename = true;
    short_config_pwrite = true;
    short_config_pwrite_inode = before.st_ino;
    short_config_pwrite_max_bytes = 7;
    TEST_ASSERT_EQUAL_INT(0, save_config(&cfg, path));
    TEST_ASSERT_TRUE(short_config_pwrite_done);
    TEST_ASSERT_EQUAL_INT(0, stat(path, &after));
    TEST_ASSERT_EQUAL_UINT64(before.st_ino, after.st_ino);

    char baseline_directory[] = "/tmp/lightnvr_config_short_write_baseline_XXXXXX";
    TEST_ASSERT_NOT_NULL(mkdtemp(baseline_directory));
    char baseline_path[MAX_PATH_LENGTH];
    snprintf(baseline_path, sizeof(baseline_path), "%s/settings.ini", baseline_directory);
    snprintf(short_write_baseline_dir, sizeof(short_write_baseline_dir), "%s", baseline_directory);
    snprintf(short_write_baseline_path, sizeof(short_write_baseline_path), "%s", baseline_path);
    short_config_pwrite = false;
    busy_config_rename = false;
    TEST_ASSERT_EQUAL_INT(0, save_config(&cfg, baseline_path));

    struct stat short_write_stat, baseline_stat;
    TEST_ASSERT_EQUAL_INT(0, stat(path, &short_write_stat));
    TEST_ASSERT_EQUAL_INT(0, stat(baseline_path, &baseline_stat));
    TEST_ASSERT_EQUAL_UINT64((uint64_t)baseline_stat.st_size, (uint64_t)short_write_stat.st_size);

    saved_config_file = fopen(path, "rb");
    TEST_ASSERT_NOT_NULL(saved_config_file);
    saved_baseline_file = fopen(baseline_path, "rb");
    TEST_ASSERT_NOT_NULL(saved_baseline_file);

    assert_files_equal_binary(saved_baseline_file, saved_config_file);

    fclose(saved_baseline_file);
    saved_baseline_file = NULL;
    fclose(saved_config_file);
    saved_config_file = NULL;
    unlink(path);
    TEST_ASSERT_EQUAL_INT(0, rmdir(directory));
    unlink(baseline_path);
    TEST_ASSERT_EQUAL_INT(0, rmdir(baseline_directory));
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
    TEST_ASSERT_EQUAL_INT(0, chmod(path, 0644));
    TEST_ASSERT_EQUAL_INT(0, chmod(directory, 0500));
    load_default_config(&cfg);
    int result = save_config(&cfg, path);
    TEST_ASSERT_EQUAL_INT(0, chmod(directory, 0700));
    TEST_ASSERT_EQUAL_INT(0, result);
    struct stat saved_st;
    TEST_ASSERT_EQUAL_INT(0, stat(path, &saved_st));
    TEST_ASSERT_EQUAL_INT(0600, (saved_st.st_mode & 0777));

    char baseline_directory[] = "/tmp/lightnvr_config_baseline_dir_XXXXXX";
    TEST_ASSERT_NOT_NULL(mkdtemp(baseline_directory));
    char baseline_path[MAX_PATH_LENGTH];
    snprintf(baseline_path, sizeof(baseline_path), "%s/settings.ini", baseline_directory);
    TEST_ASSERT_EQUAL_INT(0, save_config(&cfg, baseline_path));

    saved_config_file = fopen(path, "rb");
    TEST_ASSERT_NOT_NULL(saved_config_file);
    saved_baseline_file = fopen(baseline_path, "rb");
    TEST_ASSERT_NOT_NULL(saved_baseline_file);

    TEST_ASSERT_EQUAL_INT(0, fseek(saved_config_file, 0, SEEK_END));
    long saved_size = ftell(saved_config_file);
    TEST_ASSERT_TRUE(saved_size >= 0);
    TEST_ASSERT_EQUAL_INT(0, fseek(saved_config_file, 0, SEEK_SET));

    TEST_ASSERT_EQUAL_INT(0, fseek(saved_baseline_file, 0, SEEK_END));
    long baseline_size = ftell(saved_baseline_file);
    TEST_ASSERT_TRUE(baseline_size >= 0);
    TEST_ASSERT_EQUAL_INT(0, fseek(saved_baseline_file, 0, SEEK_SET));

    TEST_ASSERT_EQUAL_INT64((long long)baseline_size, (long long)saved_size);

    assert_files_equal_binary(saved_baseline_file, saved_config_file);
    fclose(saved_config_file);
    saved_config_file = NULL;
    fclose(saved_baseline_file);
    saved_baseline_file = NULL;

    unlink(path);
    rmdir(directory);
    unlink(baseline_path);
    rmdir(baseline_directory);
}

int main(void) {
    init_logger();

    UNITY_BEGIN();
    RUN_TEST(test_daemon_status_preserves_owned_pid_lock);
    RUN_TEST(test_daemon_status_validates_opened_pid_file);
    RUN_TEST(test_save_config_tightens_existing_permissions_before_writing_secrets);
    RUN_TEST(test_save_config_rejects_hardlinks_without_truncation_or_mode_change);
    RUN_TEST(test_save_config_replaces_group_writable_legacy_file_privately);
    RUN_TEST(test_save_config_preserves_legacy_file_when_replacement_fails);
    RUN_TEST(test_save_config_atomically_replaces_owned_file);
    RUN_TEST(test_save_config_preserves_owned_file_when_publish_fails);
    RUN_TEST(test_save_config_rejects_unsafe_credentials_before_writing);
    RUN_TEST(test_save_config_bind_mount_fallback);
    RUN_TEST(test_save_config_bind_mount_restores_after_write_failure);
    RUN_TEST(test_save_config_bind_mount_retries_short_write);
    RUN_TEST(test_save_config_updates_file_in_read_only_directory);

    RUN_TEST(test_stream_config_t_size_under_16k);
    RUN_TEST(test_max_stream_ceiling_is_valid);

    RUN_TEST(test_default_config_web_port);
    RUN_TEST(test_default_data_paths_stay_inside_persistent_volume);
    RUN_TEST(test_default_config_log_level);
    RUN_TEST(test_default_config_retention_days);
    RUN_TEST(test_default_config_mp4_faststart_on);
    RUN_TEST(test_config_copy_owns_streams_after_source_reload);
    RUN_TEST(test_config_reset_stream_slots_clears_stale_rows);
    RUN_TEST(test_save_stream_configs_preserves_identities_after_batch_reload);
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
    RUN_TEST(test_go2rtc_ini_rejects_invalid_ports);
    /* Uses the process-wide custom config path; run last to avoid affecting later tests. */
    RUN_TEST(test_save_config_round_trips_web_cache_and_mp4_faststart);
    RUN_TEST(test_streaming_config_snapshots_are_thread_local);

    int result = UNITY_END();
    shutdown_logger();
    return result;
}
