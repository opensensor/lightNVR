#include <stdio.h>
#include <string.h>
#include <time.h>

#include "video/hls/hls_cache_cleanup.h"

/* Standalone recovery/sidecar helper: same conservative rules as the NVR. */
int main(int argc, char **argv) {
    if (argc < 2 || argc > 3 || (argc == 3 && strcmp(argv[2], "--dry-run"))) {
        fprintf(stderr, "Usage: %s HLS_ROOT [--dry-run]\n", argv[0]);
        return 2;
    }
    hls_cache_cleanup_result_t result = {0};
    int status = hls_prune_cache(argv[1], time(NULL), HLS_CACHE_MIN_AGE_SECONDS,
                                HLS_CACHE_MAX_REMOVALS, argc == 3, &result);
    printf("HLS cache: %s=%u bytes=%llu errors=%u skipped_directories=%u\n",
           argc == 3 ? "candidates" : "removed", result.removed,
           (unsigned long long)result.bytes, result.errors, result.skipped_directories);
    if (status) perror("HLS cache cleanup");
    return status ? 1 : 0;
}
