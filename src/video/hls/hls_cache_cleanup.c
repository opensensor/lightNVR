#define _POSIX_C_SOURCE 200809L

#include <dirent.h>
#include <errno.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <unistd.h>

#include "video/hls/hls_cache_cleanup.h"

#define MAX_PLAYLIST_REFERENCES 4096
#define MAX_PLAYLIST_LINE 4096

static bool suffix(const char *name, const char *ending) {
    size_t n = strlen(name), e = strlen(ending);
    return n >= e && strcmp(name + n - e, ending) == 0;
}

static bool candidate(const char *name) {
    return suffix(name, ".ts") || suffix(name, ".m4s") ||
           suffix(name, ".ts.tmp") || suffix(name, ".m4s.tmp") ||
           suffix(name, ".m3u8.tmp");
}

static int compare_names(const void *a, const void *b) {
    return strcmp(*(const char *const *)a, *(const char *const *)b);
}

/* The generated playlists use local basenames. Fail closed on other URI
 * layouts, overlong lines, symlinks, and I/O failures instead of guessing. */
static int read_references(int fd, const char *name, char **refs, size_t *count) {
    int file = openat(fd, name, O_RDONLY | O_NOFOLLOW | O_NONBLOCK);
    if (file < 0) return -1;
    struct stat st;
    if (fstat(file, &st) || !S_ISREG(st.st_mode)) {
        close(file);
        return -1;
    }
    FILE *playlist = fdopen(file, "r");
    if (!playlist) { close(file); return -1; }
    char line[MAX_PLAYLIST_LINE];
    int status = 0;
    while (fgets(line, sizeof(line), playlist)) {
        size_t length = strlen(line);
        if (length == sizeof(line) - 1 && line[length - 1] != '\n') {
            status = -1;
            break;
        }
        line[strcspn(line, "\r\n")] = '\0';
        if (!line[0]) continue;
        char *uri = line;
        if (line[0] == '#') {
            uri = strstr(line, "URI=");
            if (!uri) continue;
            uri += strlen("URI=");
            if (*uri++ != '"') { status = -1; break; }
            char *end = strchr(uri, '"');
            if (!end) { status = -1; break; }
            *end = '\0';
        }
        uri[strcspn(uri, "?")] = '\0';
        if (!uri[0] || strchr(uri, '/') || strchr(uri, '%') ||
            !strcmp(uri, ".") || !strcmp(uri, "..") ||
            *count == MAX_PLAYLIST_REFERENCES) {
            status = -1;
            break;
        }
        refs[*count] = strdup(uri);
        if (!refs[*count]) { status = -1; break; }
        ++*count;
    }
    if (ferror(playlist)) status = -1;
    fclose(playlist);
    return status;
}

static void prune_stream(DIR *stream, time_t now, unsigned age,
                         unsigned limit, bool dry_run,
                         hls_cache_cleanup_result_t *result) {
    char **refs = calloc(MAX_PLAYLIST_REFERENCES, sizeof(*refs));
    if (!refs) { ++result->errors; return; }
    size_t count = 0;
    bool safe = true;
    int fd = dirfd(stream);
    struct dirent *entry;
    while ((entry = readdir(stream))) {
        if (suffix(entry->d_name, ".m3u8") &&
            read_references(fd, entry->d_name, refs, &count)) {
            safe = false;
            break;
        }
    }
    if (!safe) {
        ++result->skipped_directories;
        goto done;
    }
    qsort(refs, count, sizeof(*refs), compare_names);
    rewinddir(stream);
    while ((!limit || result->removed < limit) && (entry = readdir(stream))) {
        if (!candidate(entry->d_name)) continue;
        const char *name = entry->d_name;
        if (count && bsearch(&name, refs, count, sizeof(*refs), compare_names)) continue;
        struct stat st;
        if (fstatat(fd, name, &st, AT_SYMLINK_NOFOLLOW)) {
            if (errno != ENOENT) ++result->errors;
            continue;
        }
        if (!S_ISREG(st.st_mode) || st.st_mtime > now ||
            difftime(now, st.st_mtime) <= age) continue;
        if (!dry_run && unlinkat(fd, name, 0)) {
            if (errno != ENOENT) ++result->errors;
            continue;
        }
        ++result->removed;
        result->bytes += (uint64_t)st.st_size;
    }
done:
    for (size_t i = 0; i < count; ++i) free(refs[i]);
    free(refs);
}

int hls_prune_cache(const char *root, time_t now, unsigned minimum_age,
                    unsigned max_removals, bool dry_run,
                    hls_cache_cleanup_result_t *result) {
    if (!root || !root[0] || !result || minimum_age < HLS_CACHE_MIN_AGE_SECONDS) {
        errno = EINVAL;
        return -1;
    }
    memset(result, 0, sizeof(*result));
    int fd = open(root, O_RDONLY | O_DIRECTORY | O_NOFOLLOW);
    if (fd < 0) return errno == ENOENT ? 0 : -1;
    DIR *directory = fdopendir(fd);
    if (!directory) { close(fd); return -1; }
    struct dirent *entry;
    while ((!max_removals || result->removed < max_removals) &&
           (entry = readdir(directory))) {
        if (!strcmp(entry->d_name, ".") || !strcmp(entry->d_name, "..")) continue;
        struct stat st;
        if (fstatat(fd, entry->d_name, &st, AT_SYMLINK_NOFOLLOW) || !S_ISDIR(st.st_mode)) continue;
        int child = openat(fd, entry->d_name, O_RDONLY | O_DIRECTORY | O_NOFOLLOW);
        if (child < 0) { ++result->errors; continue; }
        DIR *stream = fdopendir(child);
        if (!stream) { close(child); ++result->errors; continue; }
        prune_stream(stream, now, minimum_age, max_removals, dry_run, result);
        closedir(stream);
    }
    closedir(directory);
    return result->errors ? -1 : 0;
}
