/* Independent helper: LD_PRELOAD must actually intercept these libc calls. */
#define _GNU_SOURCE
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/statvfs.h>
#include <unistd.h>

int main(int argc, char **argv) {
    if (argc != 2) return 2;
    struct statvfs status;
    if (statvfs(argv[1], &status)) return 3;
    char name[4096];
    snprintf(name, sizeof(name), "%s/probe-XXXXXX", argv[1]);
    errno = 0;
    int fd = mkstemp(name), error = errno;
    if (fd >= 0) { close(fd); unlink(name); }
    printf("{\"available\":%llu,\"errno\":%d,\"readonly\":%s}\n",
           (unsigned long long)status.f_bavail, fd >= 0 ? 0 : error,
           status.f_flag & ST_RDONLY ? "true" : "false");
    return 0;
}
