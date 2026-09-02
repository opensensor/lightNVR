#define _POSIX_C_SOURCE 200809L

#include "video/recording_transcode.h"

#include <errno.h>
#include <fcntl.h>
#include <stdio.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/wait.h>
#include <unistd.h>

#include <libavformat/avformat.h>

#include "core/path_utils.h"
#include "video/ffmpeg_utils.h"
#define LOG_COMPONENT "RecordingTranscode"
#include "core/logger.h"

#define VAAPI_RENDER_NODE "/dev/dri/renderD128"
#define FFMPEG_BINARY "/usr/bin/ffmpeg"

static bool file_exists_nonempty(const char *path) {
    struct stat st;
    return path && path[0] != '\0' && stat(path, &st) == 0 &&
           S_ISREG(st.st_mode) && st.st_size > 0;
}

bool recording_needs_hevc_transcode(const char *file_path) {
    if (!file_path || file_path[0] == '\0') {
        return false;
    }

    AVFormatContext *fmt_ctx = NULL;
    if (avformat_open_input(&fmt_ctx, file_path, NULL, NULL) != 0) {
        log_warn("recording_needs_hevc_transcode: failed to open %s, assuming no transcode needed", file_path);
        return false;
    }

    if (avformat_find_stream_info(fmt_ctx, NULL) < 0) {
        log_warn("recording_needs_hevc_transcode: failed to read stream info for %s", file_path);
        safe_avformat_cleanup(&fmt_ctx);
        return false;
    }

    bool is_hevc = false;
    for (unsigned int i = 0; i < fmt_ctx->nb_streams; i++) {
        AVStream *stream = fmt_ctx->streams[i];
        if (stream->codecpar->codec_type == AVMEDIA_TYPE_VIDEO) {
            is_hevc = (stream->codecpar->codec_id == AV_CODEC_ID_HEVC);
            break;
        }
    }

    safe_avformat_cleanup(&fmt_ctx);
    return is_hevc;
}

/* Runs argv (NULL-terminated) to completion, returns 0 on a clean exit(0),
 * -1 otherwise. Test-covered via ensure_recording_transcode_cache. */
static int run_and_wait(char *const argv[]) {
    pid_t pid = fork();
    if (pid < 0) {
        log_error("recording_transcode: fork failed: %s", strerror(errno));
        return -1;
    }

    if (pid == 0) {
        /* Child: silence ffmpeg's own stdout/stderr chatter into our log stream. */
        int devnull = open("/dev/null", O_WRONLY);
        if (devnull >= 0) {
            dup2(devnull, STDOUT_FILENO);
            dup2(devnull, STDERR_FILENO);
            close(devnull);
        }
        execv(argv[0], argv);
        _exit(127);
    }

    int status = 0;
    while (waitpid(pid, &status, 0) < 0) {
        if (errno != EINTR) {
            log_error("recording_transcode: waitpid failed: %s", strerror(errno));
            return -1;
        }
    }

    return (WIFEXITED(status) && WEXITSTATUS(status) == 0) ? 0 : -1;
}

int ensure_recording_transcode_cache(const char *original_path, const char *cache_path) {
    if (!original_path || original_path[0] == '\0' ||
        !cache_path || cache_path[0] == '\0') {
        return -1;
    }

    if (file_exists_nonempty(cache_path)) {
        return 0;
    }

    if (!file_exists_nonempty(original_path)) {
        log_error("recording_transcode: source recording missing or empty: %s", original_path);
        return -1;
    }

    char cache_dir[512];
    const char *last_slash = strrchr(cache_path, '/');
    if (!last_slash || last_slash == cache_path) {
        log_error("recording_transcode: cache path has no parent directory: %s", cache_path);
        return -1;
    }
    size_t dir_len = (size_t)(last_slash - cache_path);
    if (dir_len >= sizeof(cache_dir)) {
        log_error("recording_transcode: cache directory path too long: %s", cache_path);
        return -1;
    }
    memcpy(cache_dir, cache_path, dir_len);
    cache_dir[dir_len] = '\0';
    if (ensure_dir(cache_dir) != 0) {
        log_error("recording_transcode: failed to create cache directory %s: %s", cache_dir, strerror(errno));
        return -1;
    }

    char tmp_path[512];
    int written = snprintf(tmp_path, sizeof(tmp_path), "%s.tmp.%d", cache_path, (int)getpid());
    if (written < 0 || (size_t)written >= sizeof(tmp_path)) {
        log_error("recording_transcode: cache path too long: %s", cache_path);
        return -1;
    }

    bool have_vaapi = (access(VAAPI_RENDER_NODE, F_OK) == 0);

    /* tmp_path doesn't end in .mp4 (it carries a .tmp.<pid> suffix so a
     * concurrent request never observes a half-written cache file), so
     * ffmpeg can't infer the muxer from the extension — force it. */
    char *argv_vaapi[] = {
        (char *)FFMPEG_BINARY, "-y", "-hide_banner", "-loglevel", "error",
        "-hwaccel", "vaapi", "-hwaccel_device", VAAPI_RENDER_NODE,
        "-hwaccel_output_format", "vaapi",
        "-i", (char *)original_path,
        "-c:v", "h264_vaapi", "-an",
        "-f", "mp4", tmp_path, NULL
    };
    char *argv_software[] = {
        (char *)FFMPEG_BINARY, "-y", "-hide_banner", "-loglevel", "error",
        "-i", (char *)original_path,
        "-c:v", "libx264", "-an",
        "-f", "mp4", tmp_path, NULL
    };

    int rc = -1;
    if (have_vaapi) {
        log_info("recording_transcode: transcoding %s -> %s via VAAPI", original_path, cache_path);
        rc = run_and_wait(argv_vaapi);
        if (rc != 0) {
            log_warn("recording_transcode: VAAPI transcode failed for %s, falling back to software", original_path);
            unlink(tmp_path);
        }
    }
    if (rc != 0) {
        log_info("recording_transcode: transcoding %s -> %s via software libx264", original_path, cache_path);
        rc = run_and_wait(argv_software);
    }

    if (rc != 0 || !file_exists_nonempty(tmp_path)) {
        log_error("recording_transcode: ffmpeg failed to produce output for %s", original_path);
        unlink(tmp_path);
        return -1;
    }

    if (rename(tmp_path, cache_path) != 0) {
        log_error("recording_transcode: failed to rename %s -> %s: %s", tmp_path, cache_path, strerror(errno));
        unlink(tmp_path);
        return -1;
    }

    log_info("recording_transcode: cached playable copy at %s", cache_path);
    return 0;
}
