#include "telemetry/collectors/linux_restart.h"

#include <ctype.h>
#include <errno.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define RESTART_PATH_LENGTH 512U
#define RESTART_INPUT_LENGTH 128U

bool linux_restart_id_valid(const char *id) {
    if (!id || strlen(id) != 36U) return false;
    for (size_t i = 0; i < 36U; ++i) {
        bool hyphen = i == 8U || i == 13U || i == 18U || i == 23U;
        if ((hyphen && id[i] != '-') || (!hyphen && !isxdigit((unsigned char)id[i]))) {
            return false;
        }
    }
    return true;
}

static system_health_capability_t capability_for_errno(int error) {
    if (error == EACCES || error == EPERM) {
        return SYSTEM_HEALTH_CAPABILITY_PERMISSION_DENIED;
    }
    if (error == ENOENT || error == ENOTDIR) {
        return SYSTEM_HEALTH_CAPABILITY_UNSUPPORTED;
    }
    return SYSTEM_HEALTH_CAPABILITY_ERROR;
}

static int read_one_line(const char *path, char *output, size_t output_size,
                         system_health_capability_t *capability) {
    FILE *file = fopen(path, "r");
    if (!file) {
        *capability = capability_for_errno(errno);
        return -1;
    }
    if (!fgets(output, (int)output_size, file)) {
        int saved_errno = errno;
        fclose(file);
        *capability = capability_for_errno(saved_errno);
        return -1;
    }
    bool truncated = false;
    if (!strchr(output, '\n')) {
        int trailing = fgetc(file);
        if (trailing == '\r') trailing = fgetc(file);
        truncated = trailing != '\n' && trailing != EOF;
        if (ferror(file)) truncated = true;
    }
    fclose(file);
    if (truncated) {
        *capability = SYSTEM_HEALTH_CAPABILITY_ERROR;
        return -1;
    }
    output[strcspn(output, "\r\n")] = '\0';
    return 0;
}

int linux_restart_read_evidence(const char *proc_root, const char *run_id,
                                uint64_t process_start_monotonic_ms,
                                linux_restart_evidence_t *evidence) {
    if (!proc_root || !run_id || !evidence || !linux_restart_id_valid(run_id)) return -1;
    memset(evidence, 0, sizeof(*evidence));
    evidence->capability = SYSTEM_HEALTH_CAPABILITY_AVAILABLE;
    evidence->process_start_monotonic_ms = process_start_monotonic_ms;
    memcpy(evidence->run_id, run_id, LINUX_RESTART_ID_LENGTH);

    char path[RESTART_PATH_LENGTH];
    int written = snprintf(path, sizeof(path), "%s/sys/kernel/random/boot_id", proc_root);
    if (written < 0 || (size_t)written >= sizeof(path) ||
        read_one_line(path, evidence->boot_id, sizeof(evidence->boot_id),
                      &evidence->capability) != 0 ||
        !linux_restart_id_valid(evidence->boot_id)) {
        if (evidence->capability == SYSTEM_HEALTH_CAPABILITY_AVAILABLE) {
            evidence->capability = SYSTEM_HEALTH_CAPABILITY_ERROR;
        }
        return -1;
    }

    written = snprintf(path, sizeof(path), "%s/uptime", proc_root);
    char uptime[RESTART_INPUT_LENGTH];
    if (written < 0 || (size_t)written >= sizeof(path) ||
        read_one_line(path, uptime, sizeof(uptime), &evidence->capability) != 0) {
        return -1;
    }
    char *end = NULL;
    errno = 0;
    evidence->host_uptime_seconds = strtod(uptime, &end);
    if (errno != 0 || end == uptime ||
        !isfinite(evidence->host_uptime_seconds) ||
        evidence->host_uptime_seconds < 0.0) {
        evidence->capability = SYSTEM_HEALTH_CAPABILITY_ERROR;
        return -1;
    }
    while (*end && isspace((unsigned char)*end)) ++end;
    char *idle_end = NULL;
    errno = 0;
    double idle_seconds = strtod(end, &idle_end);
    if (errno != 0 || idle_end == end || !isfinite(idle_seconds) ||
        idle_seconds < 0.0) {
        evidence->capability = SYSTEM_HEALTH_CAPABILITY_ERROR;
        return -1;
    }
    while (*idle_end && isspace((unsigned char)*idle_end)) ++idle_end;
    if (*idle_end != '\0') {
        evidence->capability = SYSTEM_HEALTH_CAPABILITY_ERROR;
        return -1;
    }
    return 0;
}

linux_restart_kind_t linux_restart_classify(
    const linux_restart_evidence_t *previous,
    const linux_restart_evidence_t *current) {
    if (!current || current->capability != SYSTEM_HEALTH_CAPABILITY_AVAILABLE ||
        !previous || previous->capability != SYSTEM_HEALTH_CAPABILITY_AVAILABLE) {
        return LINUX_RESTART_FIRST_OBSERVATION;
    }
    if (strcmp(previous->boot_id, current->boot_id) != 0) {
        return LINUX_RESTART_HOST_REBOOT;
    }
    if (strcmp(previous->run_id, current->run_id) != 0) {
        return LINUX_RESTART_PROCESS_RESTART;
    }
    return LINUX_RESTART_SAME_RUN;
}
