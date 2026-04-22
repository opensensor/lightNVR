/**
 * @file api_handlers_system_go2rtc.c
 * @brief Implementation of go2rtc system information retrieval
 */

#include "web/api_handlers_system.h"
#include "web/request_response.h"
#include "web/httpd_utils.h"
#include "video/go2rtc/go2rtc_process.h"
#include "utils/yaml_redact.h"
#define LOG_COMPONENT "SystemAPI"
#include "core/logger.h"
#include <cjson/cJSON.h>
#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <errno.h>

/**
 * @brief Get memory usage of go2rtc process
 *
 * @param memory_usage Pointer to store memory usage in bytes
 * @return true if successful, false otherwise
 */
bool get_go2rtc_memory_usage(unsigned long long *memory_usage) {
    if (!memory_usage) {
        return false;
    }

    // Initialize to 0
    *memory_usage = 0;

    // Get go2rtc process ID from the process manager
    // This is more reliable than pgrep as it tracks the actual process we started
    int pid = go2rtc_process_get_pid();
    if (pid <= 0) {
        log_warn("No go2rtc process found (PID: %d)", pid);
        return false;
    }

    // Get memory usage from /proc/{pid}/status
    char status_path[64];
    snprintf(status_path, sizeof(status_path), "/proc/%d/status", pid);

    FILE *status_file = fopen(status_path, "r");
    if (!status_file) {
        log_error("Failed to open %s: %s", status_path, strerror(errno));
        return false;
    }

    char line[256];
    unsigned long vm_rss = 0;

    while (fgets(line, sizeof(line), status_file)) {
        if (strncmp(line, "VmRSS:", 6) == 0) {
            // VmRSS is in kB - actual physical memory used
            char *endptr;
            vm_rss = strtoul(line + 6, &endptr, 10);
            break;
        }
    }

    fclose(status_file);

    // Convert kB to bytes
    *memory_usage = vm_rss * 1024;

    log_debug("go2rtc memory usage (PID %d): %llu bytes", pid, *memory_usage);
    return true;
}

/* ------------------------------------------------------------------
 * T7 — effective-config preview
 * ------------------------------------------------------------------ */

/* Hard cap on how much YAML we'll read+redact+serialize per file.  Matches
 * the T5 override save cap (64 KB) plus headroom for the much larger base
 * file (per-stream entries can grow); 1 MB is well above realistic and
 * keeps malicious inputs from blowing the response. */
#define EFFECTIVE_CONFIG_FILE_CAP (1024 * 1024)

/**
 * Read a file completely into a heap buffer (NUL-terminated).
 *
 * Returns 1 on success, 0 on missing-file (ENOENT), -1 on any other error.
 * On success @p out is malloc'd and @p out_len is set; caller frees.
 */
static int read_file_alloc(const char *path, char **out, size_t *out_len)
{
    *out = NULL;
    *out_len = 0;

    int fd = open(path, O_RDONLY | O_CLOEXEC);
    if (fd < 0) {
        if (errno == ENOENT) return 0;
        log_warn("effective-config: open %s failed: %s", path, strerror(errno));
        return -1;
    }

    struct stat st;
    if (fstat(fd, &st) != 0) {
        log_warn("effective-config: fstat %s failed: %s", path, strerror(errno));
        close(fd);
        return -1;
    }
    if ((size_t)st.st_size > EFFECTIVE_CONFIG_FILE_CAP) {
        log_warn("effective-config: %s is %lld bytes (cap %d) — refusing",
                 path, (long long)st.st_size, EFFECTIVE_CONFIG_FILE_CAP);
        close(fd);
        return -1;
    }

    char *buf = malloc((size_t)st.st_size + 1);
    if (!buf) {
        close(fd);
        return -1;
    }

    size_t total = 0;
    while (total < (size_t)st.st_size) {
        ssize_t n = read(fd, buf + total, (size_t)st.st_size - total);
        if (n < 0) {
            if (errno == EINTR) continue;
            log_warn("effective-config: read %s failed: %s",
                     path, strerror(errno));
            free(buf);
            close(fd);
            return -1;
        }
        if (n == 0) break;  /* EOF before expected size — handle below */
        total += (size_t)n;
    }
    close(fd);

    /* Detect truncation (file shrank or was rotated mid-read).  Returning
     * a partial buffer would silently render an incomplete YAML preview
     * that the operator might mistake for the real effective config. */
    if (total != (size_t)st.st_size) {
        log_warn("effective-config: %s shrank during read (got %zu of %lld) — "
                 "refusing to return truncated content",
                 path, total, (long long)st.st_size);
        free(buf);
        return -1;
    }

    buf[total] = '\0';
    *out = buf;
    *out_len = total;
    return 1;
}

/* Adds a "<file>: <error message>" entry to the warnings array. */
static void add_warning(cJSON *warnings, const char *fmt, ...)
{
    char msg[256];
    va_list ap;
    va_start(ap, fmt);
    vsnprintf(msg, sizeof(msg), fmt, ap);
    va_end(ap);
    cJSON_AddItemToArray(warnings, cJSON_CreateString(msg));
}

void handle_get_system_go2rtc_effective_config(const http_request_t *req,
                                                http_response_t *res)
{
    log_info("Handling GET /api/system/go2rtc/effective-config");

    if (!httpd_check_admin_privileges(req, res)) {
        return;
    }

    cJSON *root = cJSON_CreateObject();
    cJSON *warnings = cJSON_CreateArray();
    if (!root || !warnings) {
        cJSON_Delete(root);
        cJSON_Delete(warnings);
        http_response_set_json_error(res, 500, "Out of memory");
        return;
    }

    const char *base_path = go2rtc_process_get_config_path();
    const char *override_path = go2rtc_process_get_override_path();
    int initialized = (base_path != NULL);

    cJSON_AddBoolToObject(root, "go2rtc_initialized", initialized);
    cJSON_AddBoolToObject(root, "redaction_available",
                          yaml_redact_is_available());

    /* Track whether each file was actually read so merged_source_order
     * reflects what go2rtc would have loaded — not just what files we tried.
     * The previous always-["go2rtc.yaml","override.yaml"] shape misled the
     * UI into "override applied" even when the override didn't exist. */
    bool base_loaded = false;
    bool override_loaded = false;

    /* Base */
    char *base_yaml = NULL;
    size_t base_len = 0;
    if (base_path) {
        int rc = read_file_alloc(base_path, &base_yaml, &base_len);
        if (rc == 1) {
            base_loaded = true;
        } else if (rc == 0) {
            add_warning(warnings,
                        "go2rtc.yaml not present at %s (go2rtc not started yet?)",
                        base_path);
        } else if (rc < 0) {
            add_warning(warnings,
                        "failed to read go2rtc.yaml at %s — see server log",
                        base_path);
        }
    } else {
        add_warning(warnings,
                    "go2rtc process manager not initialized; base config unavailable");
    }

    char *base_redacted = NULL;
    if (base_yaml) {
        size_t out_len = 0;
        base_redacted = yaml_redact_alloc(base_yaml, base_len, &out_len);
        if (!base_redacted) {
            /* Fail closed: returning raw YAML on redaction failure would
             * defeat this endpoint's redaction guarantee — base.yaml
             * contains stream URLs and may include credentials. */
            add_warning(warnings,
                        "redaction failed for base config — content omitted");
            cJSON_AddStringToObject(root, "base", "[REDACTION_FAILED]");
        } else {
            cJSON_AddStringToObject(root, "base", base_redacted);
        }
    } else {
        cJSON_AddStringToObject(root, "base", "");
    }

    /* Override — may legitimately not exist if the user hasn't set one. */
    char *override_yaml = NULL;
    size_t override_len = 0;
    if (override_path) {
        int rc = read_file_alloc(override_path, &override_yaml, &override_len);
        if (rc == 1) {
            override_loaded = true;
        } else if (rc < 0) {
            add_warning(warnings,
                        "failed to read override.yaml at %s — see server log",
                        override_path);
        }
        /* rc == 0 (ENOENT) is normal — no user override configured. */
    }

    char *override_redacted = NULL;
    if (override_yaml) {
        size_t out_len = 0;
        override_redacted = yaml_redact_alloc(override_yaml, override_len,
                                              &out_len);
        if (!override_redacted) {
            /* Fail closed: override.yaml is explicitly user-supplied and
             * the most likely place to find credentials (passwords, ICE
             * creds, RTSP userinfo). Never return raw on redaction failure. */
            add_warning(warnings,
                        "redaction failed for override — content omitted");
            cJSON_AddStringToObject(root, "override", "[REDACTION_FAILED]");
        } else {
            cJSON_AddStringToObject(root, "override", override_redacted);
        }
    } else {
        cJSON_AddStringToObject(root, "override", "");
    }

    /* merged_source_order reflects what was actually loaded — empty array
     * if nothing was readable, just ["go2rtc.yaml"] when no override exists,
     * etc.  Plus an explicit override_in_use boolean for callers that want
     * a one-shot signal without parsing the array. */
    cJSON *order = cJSON_CreateArray();
    if (order) {
        if (base_loaded) {
            cJSON_AddItemToArray(order, cJSON_CreateString("go2rtc.yaml"));
        }
        if (override_loaded) {
            cJSON_AddItemToArray(order, cJSON_CreateString("override.yaml"));
        }
        cJSON_AddItemToObject(root, "merged_source_order", order);
    }
    cJSON_AddBoolToObject(root, "override_in_use", override_loaded);

    cJSON_AddItemToObject(root, "warnings", warnings);

    char *body = cJSON_PrintUnformatted(root);
    cJSON_Delete(root);
    free(base_yaml);
    free(base_redacted);
    free(override_yaml);
    free(override_redacted);
    if (!body) {
        http_response_set_json_error(res, 500, "Out of memory");
        return;
    }

    http_response_set_json(res, 200, body);
    free(body);
}
