/**
 * @file yaml_validate.h
 * @brief libyaml wrappers for YAML validation in the go2rtc override pipeline.
 *
 * Two layers, both used by the go2rtc override-config pipeline (see
 * go2rtc-override-plan.md, tasks T1 and T6):
 *
 *   - `yaml_validate_str` / `yaml_is_mapping_root` are PARSE-ONLY checks
 *     (T1). They answer "is this syntactically valid YAML?" / "does it
 *     have a mapping at the root?" but do NOT detect duplicate top-level
 *     keys (libyaml-C silently accepts those).
 *
 *   - `yaml_validate_go2rtc_override` (T6) walks the event stream with
 *     its own bookkeeping to ALSO detect duplicate top-level keys,
 *     non-mapping roots, and unknown go2rtc sections. This is the
 *     validator the API endpoint and pre-save gate use.
 *
 * At build time, libyaml is detected via pkg-config; when available,
 * `LIGHTNVR_HAVE_LIBYAML` is defined and the real libyaml parser is used.
 * When libyaml is not available, the functions return a stable
 * "validation disabled" answer so callers can degrade gracefully.
 */

#ifndef LIGHTNVR_YAML_VALIDATE_H
#define LIGHTNVR_YAML_VALIDATE_H

#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Validate that `src` (length `len`) is syntactically valid YAML.
 *
 * @param src       Pointer to YAML bytes. May be NULL iff `len` is 0.
 * @param len       Length of `src` in bytes.
 * @param err_buf   Optional buffer for a human-readable error message.
 *                  When a parse error occurs and `err_buf` is non-NULL
 *                  with `err_size > 0`, a NUL-terminated message
 *                  (including line/column when libyaml provides them)
 *                  is written. May be NULL.
 * @param err_size  Size of `err_buf`. Ignored when `err_buf` is NULL.
 *
 * @return  0  on valid YAML (or an empty document — treated as valid).
 *          -1 on parse error (only when libyaml is available).
 *
 * When libyaml is NOT available at build time, this function returns 0
 * and writes "libyaml not available; validation disabled" into err_buf
 * (if provided).
 */
int yaml_validate_str(const char *src, size_t len, char *err_buf, size_t err_size);

/**
 * Check whether the YAML document's top-level node is a mapping.
 *
 * @param src  Pointer to YAML bytes. May be NULL iff `len` is 0.
 * @param len  Length of `src`.
 *
 * @return  1  if top-level is a mapping (STREAM_START → DOCUMENT_START → MAPPING_START).
 *          0  otherwise (including: not-a-mapping, empty document, or libyaml not available).
 *          -1 on parse error (only when libyaml is available).
 */
int yaml_is_mapping_root(const char *src, size_t len);

/**
 * @return 1 if this build has libyaml compiled in, 0 otherwise.
 */
int yaml_validate_is_available(void);

/* ------------------------------------------------------------------
 * T6 — go2rtc override semantic validation
 *
 * `yaml_validate_str` checks syntax only.  The go2rtc override pipeline
 * additionally needs to reject:
 *   • non-mapping roots (go2rtc requires a top-level mapping)
 *   • duplicate top-level keys (gopkg.in/yaml.v3 — what go2rtc actually
 *     uses — REJECTS dupes; libyaml-C accepts them silently, so we have
 *     to detect them ourselves with our own bookkeeping)
 *
 * It should also warn (not error) when an unfamiliar top-level section
 * appears, so a future go2rtc release adding a new section doesn't break
 * us — but typos still surface.
 * ------------------------------------------------------------------ */

#define YAML_VALIDATE_MAX_WARNINGS 16
#define YAML_VALIDATE_WARN_LEN     192
#define YAML_VALIDATE_ERR_LEN      256

typedef struct {
    /* 1 = valid (no errors).  Warnings may still be present.
     * 0 = invalid.  err_message describes the first / most actionable error.
     * -1 = could not validate (libyaml unavailable).  Caller should treat
     *      this as "skip pre-save validation"; the message explains why. */
    int valid;

    char err_message[YAML_VALIDATE_ERR_LEN];
    /* 1-based line/column.  0 when not applicable (e.g. non-syntactic error
     * such as "duplicate root key" still carries the location of the second
     * occurrence). */
    int err_line;
    int err_column;

    /* Independent diagnostic flags (set even when other errors trigger first). */
    int duplicate_keys;     /* 1 if any top-level key was duplicated */
    int non_mapping_root;   /* 1 if the document root is not a mapping */
    int parse_error;        /* 1 if libyaml emitted a parse error */

    /* Up to YAML_VALIDATE_MAX_WARNINGS NUL-terminated warning strings.
     * Warnings DO NOT make valid==0; they are advisory. */
    char warnings[YAML_VALIDATE_MAX_WARNINGS][YAML_VALIDATE_WARN_LEN];
    int  warning_count;
} yaml_validation_result_t;

/**
 * Validate a go2rtc override YAML document for syntax + semantic concerns
 * (duplicate top-level keys, non-mapping root) and surface unknown
 * top-level sections as warnings.
 *
 * Single-pass: runs libyaml's event parser exactly once and populates @p out
 * with everything in one walk.
 *
 * Safe to call with @p src == NULL && @p len == 0; treats empty input as
 * valid (an empty override means "no overrides").
 *
 * When libyaml is unavailable, sets @p out->valid = -1 with an explanatory
 * message and leaves all other fields zeroed.  Callers should treat this
 * as "skip pre-save validation, allow the save".
 */
void yaml_validate_go2rtc_override(const char *src, size_t len,
                                   yaml_validation_result_t *out);

#ifdef __cplusplus
}
#endif

#endif /* LIGHTNVR_YAML_VALIDATE_H */
