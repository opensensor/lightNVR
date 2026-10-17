#ifndef LIGHTNVR_URL_UTILS_H
#define LIGHTNVR_URL_UTILS_H

#include <stdbool.h>
#include <stddef.h>

/**
 * Apply username/password to a URL, replacing any embedded credentials.
 *
 * When @p username is NULL or empty the original URL is copied verbatim.
 * When @p password is NULL or empty the username is applied and any existing
 * password is cleared.
 */
int url_apply_credentials(const char *url, const char *username,
                          const char *password, char *out_url,
                          size_t out_size);

/**
 * Remove embedded credentials from a URL.
 */
int url_strip_credentials(const char *url, char *out_url, size_t out_size);

/**
 * Extract embedded credentials from a URL.
 * Missing username/password parts are returned as empty strings.
 */
int url_extract_credentials(const char *url, char *username,
                            size_t username_size, char *password,
                            size_t password_size);

/**
 * Build an ONVIF service URL from an existing stream/device URL.
 *
 * Maps transport schemes to their HTTP equivalent: rtsps → https, everything
 * else (rtsp, onvif, …) → http; existing http/https schemes are preserved.
 * Maps standard transport ports to ONVIF web ports when @p onvif_port is not
 * explicitly set: RTSP 554 → HTTP 80, RTSPS 322 → HTTPS 443.  All other
 * ports are passed through unchanged.  Credentials, query parameters, and
 * fragments are always stripped from the output.
 *
 * @param url          Source URL (rtsp, rtsps, http, https, onvif, …).
 * @param onvif_port   Explicit ONVIF port.  When > 0 it overrides any port
 *                     derived from @p url; when <= 0 the port is derived via
 *                     the mapping table above.
 * @param service_path ONVIF service path (e.g. "/onvif/device_service" or
 *                     "/onvif/ptz_service").  Must begin with '/'.
 *                     Pass NULL or an empty string to return only the
 *                     scheme + host + port (base URL without trailing slash).
 * @param out_url      Output buffer.
 * @param out_size     Size of the output buffer.
 * @return             0 on success, -1 on error.
 */
int url_build_onvif_service_url(const char *url, int onvif_port,
                                const char *service_path,
                                char *out_url, size_t out_size);

/**
 * Build an ONVIF device-service URL from an existing stream/device URL.
 *
 * Convenience wrapper around url_build_onvif_service_url() that uses
 * "/onvif/device_service" as the service path.
 *
 * If @p onvif_port is <= 0 the stripped input URL is returned unchanged
 * (backward-compatible behaviour for callers that pass a URL already
 * containing an ONVIF endpoint).  Otherwise the host is preserved,
 * credentials are removed, the scheme is mapped to http/https, and the
 * path becomes /onvif/device_service.
 */
int url_build_onvif_device_service_url(const char *url, int onvif_port,
                                       char *out_url, size_t out_size);

/**
 * Redact credentials for safe logging.
 */
int url_redact_for_logging(const char *url, char *out_url, size_t out_size);

/**
 * Format the URL for the API endpoint with query parameters (simple method)
 *
 * This is the method that works according to user feedback
 * URL encode the stream_url to handle special characters
 * TODO: it's unclear why we wouldn't use curl_easy_escape here
 *
 * @param input The input string
 * @param output The output buffer
 * @param output_size The size of the output buffer
 */
void simple_url_escape(const char *input, char *output, size_t output_size);

/* Per-destination length limit for restream URLs (matches MAX_URL_LENGTH). */
#define PUBLISH_URL_MAX_LENGTH 512

/**
 * Iterate the restream (publish) destinations in a list. Destinations are
 * separated by commas and/or whitespace.
 *
 * @param cursor   Position to scan from (start with the list itself)
 * @param out      Receives the next destination (truncated to out_size)
 * @param out_size Size of out
 * @return Position after the destination to pass back in, or NULL when no
 *         destinations remain
 */
const char *publish_url_next(const char *cursor, char *out, size_t out_size);

/**
 * Validate a restream (publish) destination list: every destination must use
 * rtmp:// or rtmps://, fit in MAX_URL_LENGTH-sized buffers and contain only
 * printable, non-space ASCII. An empty list is valid (restreaming disabled).
 */
bool publish_url_list_is_valid(const char *list);

#endif /* LIGHTNVR_URL_UTILS_H */