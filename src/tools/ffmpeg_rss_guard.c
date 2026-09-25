/* Limit the resident memory of go2rtc's ffmpeg children without constraining
 * FFmpeg's much larger virtual address reservations (notably libx264 stacks).
 * The guard remains the direct child of go2rtc so it can reap and signal the
 * transcoder. PR_SET_PDEATHSIG also closes the SIGKILL/orphan gap when go2rtc
 * cancels an exec.CommandContext. */
#define _GNU_SOURCE

#include <errno.h>
#include <limits.h>
#include <signal.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/prctl.h>
#include <sys/resource.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <time.h>
#include <unistd.h>

#define DEFAULT_AUDIO_MAX_RSS_MB 768U
#define DEFAULT_VIDEO_MAX_RSS_MB 1536U
#define MIN_MAX_RSS_MB 128U
#define MAX_MAX_RSS_MB 2048U

static volatile sig_atomic_t child_pid = -1;

static void forward_signal(int signal_number) {
    if (child_pid > 0) {
        kill((pid_t)child_pid, signal_number);
    }
}

static bool audio_only_command(int argc, char **argv) {
    bool audio = false;
    bool no_video = false;
    for (int i = 1; i < argc; i++) {
        if (strncmp(argv[i], "-c:a", 4) == 0 ||
            strncmp(argv[i], "-codec:a", 8) == 0) audio = true;
        if (strcmp(argv[i], "-vn") == 0) no_video = true;
        if (strncmp(argv[i], "-c:v", 4) == 0 ||
            strncmp(argv[i], "-codec:v", 8) == 0 ||
            strcmp(argv[i], "-vf") == 0) return false;
    }
    return audio && no_video;
}

static unsigned int configured_limit_mb(bool audio_only) {
    unsigned int default_limit = audio_only ? DEFAULT_AUDIO_MAX_RSS_MB
                                             : DEFAULT_VIDEO_MAX_RSS_MB;
    const char *value = getenv("LIGHTNVR_FFMPEG_MAX_RSS_MB");
    if (!value || !*value) return default_limit;
    char *end = NULL;
    errno = 0;
    unsigned long parsed = strtoul(value, &end, 10);
    if (errno || !end || *end || parsed < MIN_MAX_RSS_MB ||
        parsed > MAX_MAX_RSS_MB) {
        fprintf(stderr, "ffmpeg guard: invalid LIGHTNVR_FFMPEG_MAX_RSS_MB; "
                        "using %u MiB\n", default_limit);
        return default_limit;
    }
    return (unsigned int)parsed;
}

static bool resident_bytes(pid_t pid, uint64_t *bytes) {
    char path[64];
    snprintf(path, sizeof(path), "/proc/%ld/statm", (long)pid);
    FILE *statm = fopen(path, "r");
    if (!statm) return false;
    unsigned long long size_pages, resident_pages;
    int fields = fscanf(statm, "%llu %llu", &size_pages, &resident_pages);
    fclose(statm);
    if (fields != 2) return false;
    long page_size = sysconf(_SC_PAGESIZE);
    if (page_size <= 0 || resident_pages > UINT64_MAX / (uint64_t)page_size) {
        return false;
    }
    *bytes = (uint64_t)resident_pages * (uint64_t)page_size;
    return true;
}

int main(int argc, char **argv) {
    if (argc < 2) {
        fprintf(stderr, "usage: ffmpeg guard <ffmpeg arguments>\n");
        return 2;
    }
    bool audio_only = audio_only_command(argc, argv);
    unsigned int max_rss_mb = configured_limit_mb(audio_only);
    const uint64_t max_rss = (uint64_t)max_rss_mb * 1024U * 1024U;
    struct sigaction action = {.sa_handler = forward_signal};
    sigemptyset(&action.sa_mask);
    sigaction(SIGTERM, &action, NULL);
    sigaction(SIGINT, &action, NULL);
    sigaction(SIGHUP, &action, NULL);

    pid_t pid = fork();
    if (pid < 0) {
        perror("ffmpeg guard: fork");
        return 1;
    }
    if (pid == 0) {
        signal(SIGTERM, SIG_DFL);
        signal(SIGINT, SIG_DFL);
        signal(SIGHUP, SIG_DFL);
        if (prctl(PR_SET_PDEATHSIG, SIGKILL) != 0 || getppid() == 1) {
            _exit(1);
        }
        if (audio_only) {
            /* The AAC+OPUS worker is the reported runaway. A hard address
             * limit stops it even if it allocates faster than the RSS poll.
             * Video encoders reserve multi-gigabyte virtual stacks, so only
             * the audio worker gets this limit. */
            rlim_t address_limit = (rlim_t)(max_rss_mb + 256U) * 1024U * 1024U;
            struct rlimit limit = {.rlim_cur = address_limit,
                                   .rlim_max = address_limit};
            if (setrlimit(RLIMIT_AS, &limit) != 0) {
                perror("ffmpeg guard: setrlimit RLIMIT_AS");
                _exit(1);
            }
        }
        argv[0] = "ffmpeg";
        execvp("ffmpeg", argv);
        perror("ffmpeg guard: execvp ffmpeg");
        _exit(127);
    }

    child_pid = pid;
    bool killed_for_rss = false;
    int status = 0;
    const struct timespec interval = {.tv_sec = 0, .tv_nsec = 25000000L};
    for (;;) {
        pid_t waited = waitpid(pid, &status, WNOHANG);
        if (waited == pid) break;
        if (waited < 0 && errno != EINTR) {
            perror("ffmpeg guard: waitpid");
            kill(pid, SIGKILL);
            waitpid(pid, &status, 0);
            child_pid = -1;
            return 1;
        }
        uint64_t rss = 0;
        if (!killed_for_rss && resident_bytes(pid, &rss) && rss > max_rss) {
            fprintf(stderr, "ffmpeg guard: pid %ld RSS %llu MiB exceeds "
                            "%llu MiB; killing transcoder\n", (long)pid,
                    (unsigned long long)(rss / (1024U * 1024U)),
                    (unsigned long long)(max_rss / (1024U * 1024U)));
            kill(pid, SIGKILL);
            killed_for_rss = true;
        }
        nanosleep(&interval, NULL);
    }
    child_pid = -1;
    if (WIFEXITED(status)) return WEXITSTATUS(status);
    if (WIFSIGNALED(status)) return 128 + WTERMSIG(status);
    return 1;
}
