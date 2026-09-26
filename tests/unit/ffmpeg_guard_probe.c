#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <sys/resource.h>
#include <unistd.h>

int main(int argc, char **argv) {
    unsigned long allocation_mib = 0;
    unsigned long expected_as = 0;

    for (int i = 1; i < argc; i++) {
        if (strcmp(argv[i], "-version") == 0) return 0;
        if (strncmp(argv[i], "--allocate-mib=", 15) == 0) {
            allocation_mib = strtoul(argv[i] + 15, NULL, 10);
        }
        if (strncmp(argv[i], "--expect-as=", 12) == 0) {
            expected_as = strtoul(argv[i] + 12, NULL, 10);
        }
    }

    if (expected_as) {
        struct rlimit limit;
        if (getrlimit(RLIMIT_AS, &limit) != 0 ||
            limit.rlim_cur != (rlim_t)expected_as ||
            limit.rlim_max != (rlim_t)expected_as) return 3;
    }

    if (allocation_mib) {
        size_t bytes = (size_t)allocation_mib * 1024U * 1024U;
        volatile unsigned char *memory = malloc(bytes);
        if (!memory) return 4;
        for (size_t offset = 0; offset < bytes; offset += 4096U) {
            memory[offset] = 1;
        }
        sleep(2);
        free((void *)memory);
    }
    return 0;
}
