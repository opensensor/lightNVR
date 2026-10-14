#ifndef HLS_CACHE_CLEANUP_H
#define HLS_CACHE_CLEANUP_H

#include <stdbool.h>
#include <stdint.h>
#include <time.h>

#define HLS_CACHE_MIN_AGE_SECONDS 300
#define HLS_CACHE_MAX_REMOVALS 2048

typedef struct {
    unsigned removed;
    unsigned errors;
    unsigned skipped_directories;
    uint64_t bytes;
} hls_cache_cleanup_result_t;

/* Prune only old, unreferenced HLS segments/temporary files in immediate
 * stream subdirectories. Never follow symlinks or touch recordings/playlists.
 * Unreadable or unsupported playlists cause the stream directory to be skipped.
 * max_removals bounds work per pass; zero means unlimited (operator recovery).
 */
int hls_prune_cache(const char *root, time_t now, unsigned minimum_age,
                    unsigned max_removals, bool dry_run,
                    hls_cache_cleanup_result_t *result);

#endif
