/**
 * @file yaml_validate.h
 * @brief Thin libyaml wrapper for parse-only YAML validation.
 *
 * This helper is used by the go2rtc override-config pipeline (see
 * go2rtc-override-plan.md, task T1) to pre-flight validate user-supplied
 * YAML before it is passed to go2rtc as a second `--config` file.
 *
 * At build time, libyaml is detected via pkg-config; when available,
 * `LIGHTNVR_HAVE_LIBYAML` is defined and the real libyaml parser is used.
 * When libyaml is not available, the functions return a stable
 * "validation disabled" answer so callers can degrade gracefully.
 *
 * Duplicate-key detection is intentionally NOT implemented here — that
 * is scheduled for a later task (T6) which walks the event stream with
 * its own bookkeeping.
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

#ifdef __cplusplus
}
#endif

#endif /* LIGHTNVR_YAML_VALIDATE_H */
