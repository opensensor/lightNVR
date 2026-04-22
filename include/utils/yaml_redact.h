/**
 * @file yaml_redact.h
 * @brief YAML-aware secret redactor for the go2rtc effective-config preview.
 *
 * The redactor walks libyaml's event stream so a value written as a block
 * scalar (`password: |\n  secret`) is caught the same as an inline scalar —
 * a regex pass over the source text would miss the block form.
 *
 * Currently masks:
 *   - api.password, api.username
 *   - rtsp.password, rtsp.username
 *   - mqtt.password
 *   - webrtc.ice_servers[*].credential
 *   - webrtc.ice_servers[*].username
 *   - streams.* URL userinfo (rtsp://user:pass@host → rtsp://<redacted>@host)
 *
 * When libyaml is unavailable at build time, returns the input verbatim
 * (the UI must show a clear "redaction skipped" indicator in that case).
 */

#ifndef LIGHTNVR_YAML_REDACT_H
#define LIGHTNVR_YAML_REDACT_H

#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Re-emit @p src with sensitive values replaced.
 *
 * Allocates and returns a NUL-terminated buffer that the caller must free().
 * On any error, returns NULL.  When libyaml is not built in, returns a
 * malloc'd copy of @p src (length @p len plus NUL).
 *
 * @param src     YAML input bytes (may be NULL iff @p len == 0).
 * @param len     Length of @p src in bytes.
 * @param out_len Optional out-parameter receiving the output length, not
 *                including the NUL terminator.
 */
char *yaml_redact_alloc(const char *src, size_t len, size_t *out_len);

/**
 * @return 1 if this build can perform structural redaction (libyaml present),
 *         0 if redaction is a no-op pass-through.
 */
int yaml_redact_is_available(void);

#ifdef __cplusplus
}
#endif

#endif /* LIGHTNVR_YAML_REDACT_H */
