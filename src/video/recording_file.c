/* SPDX-License-Identifier: GPL-3.0-or-later */
#define _POSIX_C_SOURCE 200809L
#define _FILE_OFFSET_BITS 64
#include "video/recording_file.h"
#include <errno.h>
#include <fcntl.h>
#include <unistd.h>
#include <sys/stat.h>
#include <libavutil/error.h>
#include <libavutil/mem.h>

#if LIBAVFORMAT_VERSION_MAJOR >= 61
static int write_file(void *opaque, const uint8_t *buffer, int size) {
#else
static int write_file(void *opaque, uint8_t *buffer, int size) {
#endif
    int fd = *(int *)opaque;
    int done = 0;
    while (done < size) {
        ssize_t count = write(fd, buffer + done, (size_t)(size - done));
        if (count < 0 && errno == EINTR) continue;
        if (count <= 0) return AVERROR(count < 0 ? errno : EIO);
        done += (int)count;
    }
    return done;
}

static int64_t seek_file(void *opaque, int64_t offset, int whence) {
    int fd = *(int *)opaque;
    if (whence == AVSEEK_SIZE) {
        struct stat st;
        return fstat(fd, &st) == 0 ? st.st_size : AVERROR(errno);
    }
    off_t result = lseek(fd, offset, whence & ~AVSEEK_FORCE);
    return result < 0 ? AVERROR(errno) : result;
}

int recording_file_open(AVIOContext **context, const char *path) {
    if (!context || *context || !path || !*path) return AVERROR(EINVAL);
    int *fd = av_malloc(sizeof(*fd));
    unsigned char *buffer = av_malloc(32768);
    if (!fd || !buffer) {
        av_free(fd);
        av_free(buffer);
        return AVERROR(ENOMEM);
    }
    AVIOContext *io = avio_alloc_context(buffer, 32768, 1, fd, NULL,
                                         write_file, seek_file);
    if (!io) {
        av_free(fd);
        av_free(buffer);
        return AVERROR(ENOMEM);
    }
    *fd = open(path, O_WRONLY | O_CREAT | O_EXCL | O_CLOEXEC, S_IRUSR | S_IWUSR);
    if (*fd < 0) {
        int error = AVERROR(errno);
        av_freep(&io->buffer);
        avio_context_free(&io);
        av_free(fd);
        return error;
    }
    *context = io;
    return 0;
}

int recording_file_close(AVIOContext **context) {
    if (!context || !*context) return 0;
    AVIOContext *io = *context;
    avio_flush(io);
    int error = io->error;
    int *fd = io->opaque;
    /* Never retry close after EINTR: on Linux the descriptor is already freed. */
    if (close(*fd) < 0 && error >= 0) error = AVERROR(errno);
    av_free(fd);
    av_freep(&io->buffer);
    avio_context_free(context);
    return error < 0 ? error : 0;
}
