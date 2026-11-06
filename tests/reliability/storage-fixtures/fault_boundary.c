/* Test-only, process-local fault boundary. Never installed in LightNVR.
 * Only paths within LIGHTNVR_TEST_FAULT_ROOT are affected. A private control
 * file selects full/readonly/none; no mount, host disk or product code changes.
 */
#define _GNU_SOURCE
#include <dlfcn.h>
#include <errno.h>
#include <fcntl.h>
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/statvfs.h>
#include <sys/syscall.h>
#include <unistd.h>

static int fault(const char *path, const char *operation) {
    const char *root = getenv("LIGHTNVR_TEST_FAULT_ROOT");
    const char *control = getenv("LIGHTNVR_TEST_FAULT_CONTROL");
    const char *log = getenv("LIGHTNVR_TEST_FAULT_LOG");
    if (!path || !root || !control || !log || root[0] != '/') return 0;
    size_t size = strlen(root);
    if (strncmp(path, root, size) || (path[size] && path[size] != '/')) return 0;
    int saved = errno;
    int fd = syscall(SYS_openat, AT_FDCWD, control, O_RDONLY | O_CLOEXEC, 0);
    if (fd < 0) return 0;
    char mode[16] = {0};
    ssize_t count = syscall(SYS_read, fd, mode, sizeof(mode) - 1);
    syscall(SYS_close, fd);
    int error = count > 0 && !strncmp(mode, "full", 4) ? ENOSPC :
                count > 0 && !strncmp(mode, "readonly", 8) ? EROFS : 0;
    if (error) {
        char entry[160];
        int length = snprintf(entry, sizeof(entry), "%ld %s %d\n", (long)getpid(), operation, error);
        fd = syscall(SYS_openat, AT_FDCWD, log, O_CREAT | O_APPEND | O_WRONLY | O_CLOEXEC, 0600);
        if (fd >= 0) {
            (void)syscall(SYS_write, fd, entry, (size_t)length);
            syscall(SYS_close, fd);
        }
    }
    errno = saved;
    return error;
}

int mkstemp(char *template) {
    int error = fault(template, "mkstemp");
    if (error) { errno = error; return -1; }
    int (*next)(char *) = dlsym(RTLD_NEXT, "mkstemp");
    return next(template);
}
int mkstemp64(char *template) {
    int error = fault(template, "mkstemp64");
    if (error) { errno = error; return -1; }
    int (*next)(char *) = dlsym(RTLD_NEXT, "mkstemp64");
    return next(template);
}

int statvfs(const char *path, struct statvfs *result) {
    int (*next)(const char *, struct statvfs *) = dlsym(RTLD_NEXT, "statvfs");
    int rc = next(path, result);
    if (rc == 0) {
        int error = fault(path, "statvfs");
        if (error == ENOSPC) result->f_bfree = result->f_bavail = 0;
        if (error == EROFS) result->f_flag |= ST_RDONLY;
    }
    return rc;
}
int statvfs64(const char *path, struct statvfs64 *result) {
    int (*next)(const char *, struct statvfs64 *) = dlsym(RTLD_NEXT, "statvfs64");
    int rc = next(path, result);
    if (rc == 0) {
        int error = fault(path, "statvfs64");
        if (error == ENOSPC) result->f_bfree = result->f_bavail = 0;
        if (error == EROFS) result->f_flag |= ST_RDONLY;
    }
    return rc;
}
