#include <ctype.h>
#include <dirent.h>
#include <errno.h>
#include <fcntl.h>
#include <libgen.h>
#include <limits.h>
#include <stdbool.h>
#include <stdio.h>
#include <sys/stat.h>
#include <unistd.h>

#include "core/path_utils.h"
#include "core/logger.h"
#include "utils/strings.h"

void sanitize_stream_name(const char *input, char *output, size_t output_size) {
    size_t i = 0;
    for (; i < output_size - 1 && input[i] != '\0'; i++) {
        // Make sure non-ASCII values are converted to positive inputs to ctype functions
        unsigned char c = (unsigned char) input[i];
        // Allowed characters: [A-Za-z0-9_\-]
        if (isalpha(c) || isdigit(c) || c == '-') {
            output[i] = c;
        } else {
            output[i] = '_';
        }
    }
    output[i] = '\0';
}

bool is_valid_stream_name(const char *name) {
    if (!name || name[0] == '\0') {
        return false;
    }
    // Reject leading '.' so names can't produce hidden-file paths.
    if (name[0] == '.') {
        return false;
    }
    for (size_t i = 0; name[i] != '\0'; i++) {
        unsigned char c = (unsigned char) name[i];
        if (!(isalpha(c) || isdigit(c) || c == '-' || c == '_' || c == '.')) {
            return false;
        }
    }
    return true;
}

// Ensure the specified directory exists, creating it if necessary. Does not recur.
int ensure_dir(const char *path) {
    struct stat st;

    if (stat(path, &st) == 0) {
        if (!S_ISDIR(st.st_mode)) {
            // Path exists but is not a directory
            log_error("Regular file in the way of directory at %s", path);
            // Also set errno
            errno = ENOTDIR;
            return -ENOTDIR;
        }
        // If path exists and is directory, we're done
    } else {
        // Create this directory level
        if (mkdir(path, 0755) != 0 && errno != EEXIST) {
            log_error("Failed to create directory %s: %s", path, strerror(errno));
            return -errno;
        }
    }

    return 0;
}

/**
 * Create a directory recursively (like mkdir -p), internal implementation.
 * 
 * Will modify path in-place, but restores it before returning. Does not perform
 * input validation. Returns the negative error code if one is encountered.
 */
static int _mkdir_recursive(char *path) {
    // Iterate through path components and create each directory
    char *p = path;

    // Skip leading slash for absolute paths
    if (*p == '/') {
        p++;
    }

    while (*p) {
        // Find next slash
        while (*p && *p != '/') {
            p++;
        }

        // Terminate string at directory separator
        char saved = *p;
        *p = '\0';

        // Check if directory already exists
        if (ensure_dir(path)) {
            return -1;
        }

        *p = saved;
        if (*p) {
            p++;
        }
    }

    return 0;
}

/**
 * Ensure a directory exists for a file path
 */
int ensure_path(const char *path) {
    if (!path || !*path) {
        return -1;
    }

    // Use the full OS path length here since we'll be calling `dirname`. We
    // need a writable copy of path since dirname may modify the string.
    char path_buf[PATH_MAX];

    safe_strcpy(path_buf, path, PATH_MAX, 0);
    char *dir = dirname(path_buf);
    return _mkdir_recursive(dir);
}

/**
 * Create a directory recursively (like mkdir -p).
 */
int mkdir_recursive(const char *path) {
    if (!path || !*path) {
        return -1;
    }

    // Make a mutable copy of the path
    char path_copy[PATH_MAX];
    safe_strcpy(path_copy, path, PATH_MAX, 0);
    return _mkdir_recursive(path_copy);
}

/**
 * Set permissions on a file or directory (like chmod). Sets fd_out if the
 * path is a directory for use in recursive chmod.
 */
static int _chmod_path(const char *path, mode_t mode, int *fd_out) {
    // Open as a directory first (O_NOFOLLOW prevents following symlinks, eliminating
    // the TOCTOU race between the old lstat() check and chmod() on the same path).
    int fd = open(path, O_RDONLY | O_DIRECTORY | O_NOFOLLOW);
    bool is_dir = (fd >= 0);

    if (!is_dir) {
        if (errno == ENOTDIR) {
            // Not a directory — open as a regular file, still with O_NOFOLLOW
            fd = open(path, O_RDONLY | O_NOFOLLOW);
        }
        if (fd < 0) {
            if (errno == ELOOP) {
                // Path is a symlink — skip without error (don't follow)
                return 0;
            }
            log_warn("Failed to open %s: %s", path, strerror(errno));
            return -1;
        }
    }

    // Apply permissions atomically through the open fd (no TOCTOU)
    if (fchmod(fd, mode) != 0) {
        log_warn("Failed to fchmod %s: %s", path, strerror(errno));
        close(fd);
        return -1;
    }

    if (!is_dir || fd_out == NULL) {
        close(fd);
        fd = -1;
    }

    if (fd_out) {
        *fd_out = fd;
    }

    return 0;
}

/**
 * Set permissions on a file or directory (like chmod)
 */
int chmod_path(const char *path, mode_t mode) {
    if (!path || !*path) {
        return -1;
    }

    return _chmod_path(path, mode, NULL);
}

int chmod_parent(const char *path, mode_t mode) {
    if (!path || !*path) {
        return -1;
    }

    // Use the full OS path length here since we'll be calling `dirname`. We
    // need a writable copy of path since dirname may modify the string.
    char path_buf[PATH_MAX];

    safe_strcpy(path_buf, path, PATH_MAX, 0);
    char *dir = dirname(path_buf);
    return _chmod_path(dir, mode, NULL);
}

/**
 * Recursively set permissions on a directory and its contents (like chmod -R)
 */
int chmod_recursive(const char *path, mode_t mode) {
    if (!path || !*path) {
        return -1;
    }

    int fd;
    if (_chmod_path(path, mode, &fd)) {
        return -1;
    }

    if (fd < 0) {
        // Path was a file, we're done.
        return 0;
    }

    // Directory: use fdopendir so readdir operates on the already-open fd
    DIR *dir = fdopendir(fd);
    if (!dir) {
        log_warn("Failed to fdopendir %s: %s", path, strerror(errno));
        close(fd);
        return -1;
    }
    // fd is now owned by dir; do not close it separately

    const struct dirent *entry;
    char full_path[PATH_MAX];
    int result = 0;

    size_t offset = snprintf(full_path, sizeof(full_path), "%s/", path);
    if (offset >= PATH_MAX) {
        // If the path is already max length, we won't be able to append anything to
        // it. In this case emit an error and return.
        log_error("Path too long to recur for chmod");
        close(fd);
        return -1;
    }
    char *path_ptr = full_path + offset;

    while ((entry = readdir(dir)) != NULL) {
        // Skip . and ..
        if (strcmp(entry->d_name, ".") == 0 || strcmp(entry->d_name, "..") == 0) {
            continue;
        }

        // Overwrite just the file portion of the path
        safe_strcpy(path_ptr, entry->d_name, sizeof(full_path) - offset, 0);

        // Recursively chmod (opens each entry with O_NOFOLLOW)
        if (chmod_recursive(full_path, mode) != 0) {
            result = -1;
            // Continue processing other entries
        }
    }

    closedir(dir); // also closes fd
    return result;
}
