#ifndef LIGHTNVR_URL_UTILS_H
#define LIGHTNVR_URL_UTILS_H

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
 * Build an ONVIF device-service URL from an existing stream/device URL.
 *
 * If @p onvif_port is <= 0 the stripped input URL is returned. Otherwise the
 * host is preserved, credentials are removed, the scheme is kept only when it
 * is already http/https (otherwise it becomes http), and the path becomes
 * /onvif/device_service.
 */
int url_build_onvif_device_service_url(const char *url, int onvif_port,
                                       char *out_url, size_t out_size);

/**
 * Redact credentials for safe logging.
 */
int url_redact_for_logging(const char *url, char *out_url, size_t out_size);

#endif /* LIGHTNVR_URL_UTILS_H */