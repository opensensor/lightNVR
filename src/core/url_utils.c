#include "core/url_utils.h"

#include <curl/curl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>

static int copy_string(const char *src, char *dst, size_t dst_size) {
    if (!src || !dst || dst_size == 0) {
        return -1;
    }

    int written = snprintf(dst, dst_size, "%s", src);
    return (written < 0 || (size_t)written >= dst_size) ? -1 : 0;
}

static int append_suffix(const char *base, const char *suffix,
                         char *out_url, size_t out_size) {
    if (!base || !out_url || out_size == 0) {
        return -1;
    }

    int written = snprintf(out_url, out_size, "%s%s", base, suffix ? suffix : "");
    return (written < 0 || (size_t)written >= out_size) ? -1 : 0;
}

static int split_url_base(const char *url, char **base_url, const char **suffix) {
    if (!url || !base_url) {
        return -1;
    }

    const char *fragment = strchr(url, '#');
    size_t base_len = fragment ? (size_t)(fragment - url) : strlen(url);

    char *base = malloc(base_len + 1);
    if (!base) {
        return -1;
    }

    memcpy(base, url, base_len);
    base[base_len] = '\0';

    *base_url = base;
    if (suffix) {
        *suffix = fragment;
    }

    return 0;
}

static int find_userinfo_bounds(const char *url,
                                const char **userinfo_start,
                                const char **userinfo_end) {
    const char *scheme_end = strstr(url, "://");
    if (!scheme_end) {
        return -1;
    }

    const char *authority_start = scheme_end + 3;
    const char *authority_end = strpbrk(authority_start, "/?#");
    const char *at = strchr(authority_start, '@');
    if (!at || (authority_end && at >= authority_end)) {
        return -1;
    }

    if (userinfo_start) {
        *userinfo_start = authority_start;
    }
    if (userinfo_end) {
        *userinfo_end = at;
    }

    return 0;
}

static int manual_strip_credentials(const char *url, char *out_url, size_t out_size) {
    const char *userinfo_start = NULL;
    const char *userinfo_end = NULL;

    if (find_userinfo_bounds(url, &userinfo_start, &userinfo_end) != 0) {
        return copy_string(url, out_url, out_size);
    }

    size_t prefix_len = (size_t)(userinfo_start - url);
    int written = snprintf(out_url, out_size, "%.*s%s", (int)prefix_len, url,
                           userinfo_end + 1);
    return (written < 0 || (size_t)written >= out_size) ? -1 : 0;
}

static int manual_extract_credentials(const char *url, char *username,
                                      size_t username_size, char *password,
                                      size_t password_size) {
    const char *userinfo_start = NULL;
    const char *userinfo_end = NULL;
    const char *separator = NULL;
    size_t username_len;
    size_t password_len;

    if (!username || !password || username_size == 0 || password_size == 0) {
        return -1;
    }

    username[0] = '\0';
    password[0] = '\0';

    if (find_userinfo_bounds(url, &userinfo_start, &userinfo_end) != 0) {
        return 0;
    }

    separator = memchr(userinfo_start, ':', (size_t)(userinfo_end - userinfo_start));
    username_len = separator ? (size_t)(separator - userinfo_start)
                             : (size_t)(userinfo_end - userinfo_start);
    password_len = separator ? (size_t)(userinfo_end - separator - 1) : 0;

    if (username_len >= username_size || password_len >= password_size) {
        return -1;
    }

    memcpy(username, userinfo_start, username_len);
    username[username_len] = '\0';

    if (separator && password_len > 0) {
        memcpy(password, separator + 1, password_len);
        password[password_len] = '\0';
    }

    return 0;
}

static int manual_apply_credentials(const char *url, const char *username,
                                    const char *password, char *out_url,
                                    size_t out_size) {
    const char *scheme_end = strstr(url, "://");
    const char *host_start;
    char *stripped_url = NULL;
    int written;

    if (!scheme_end) {
        return copy_string(url, out_url, out_size);
    }

    stripped_url = malloc(strlen(url) + 1);
    if (!stripped_url) {
        return -1;
    }

    if (manual_strip_credentials(url, stripped_url, strlen(url) + 1) != 0) {
        free(stripped_url);
        return -1;
    }

    scheme_end = strstr(stripped_url, "://");
    host_start = scheme_end ? scheme_end + 3 : stripped_url;
    written = snprintf(out_url, out_size, "%.*s://%s%s%s@%s",
                       (int)(scheme_end - stripped_url), stripped_url,
                       username,
                       (password && password[0] != '\0') ? ":" : "",
                       (password && password[0] != '\0') ? password : "",
                       host_start);

    free(stripped_url);
    return (written < 0 || (size_t)written >= out_size) ? -1 : 0;
}

static CURLU *parse_url_handle(const char *url, char **base_url, const char **suffix) {
    CURLU *handle = NULL;

    if (split_url_base(url, base_url, suffix) != 0) {
        return NULL;
    }

    handle = curl_url();
    if (!handle) {
        free(*base_url);
        *base_url = NULL;
        return NULL;
    }

    if (curl_url_set(handle, CURLUPART_URL, *base_url, CURLU_NON_SUPPORT_SCHEME) != CURLUE_OK) {
        curl_url_cleanup(handle);
        free(*base_url);
        *base_url = NULL;
        return NULL;
    }

    return handle;
}

static int rebuild_url(CURLU *handle, const char *suffix,
                       char *out_url, size_t out_size) {
    char *rebuilt = NULL;
    int result = -1;

    if (curl_url_get(handle, CURLUPART_URL, &rebuilt, 0) != CURLUE_OK || !rebuilt) {
        return -1;
    }

    result = append_suffix(rebuilt, suffix, out_url, out_size);
    curl_free(rebuilt);
    return result;
}

int url_apply_credentials(const char *url, const char *username,
                          const char *password, char *out_url,
                          size_t out_size) {
    char *base_url = NULL;
    const char *suffix = NULL;
    CURLU *handle = NULL;
    int result = -1;

    if (!url || !out_url || out_size == 0) {
        return -1;
    }

    if (!username || username[0] == '\0') {
        return copy_string(url, out_url, out_size);
    }

    handle = parse_url_handle(url, &base_url, &suffix);
    if (!handle) {
        return manual_apply_credentials(url, username, password, out_url, out_size);
    }

    if (curl_url_set(handle, CURLUPART_USER, username, CURLU_URLENCODE) != CURLUE_OK) {
        goto cleanup;
    }

    if (password && password[0] != '\0') {
        if (curl_url_set(handle, CURLUPART_PASSWORD, password, CURLU_URLENCODE) != CURLUE_OK) {
            goto cleanup;
        }
    } else if (curl_url_set(handle, CURLUPART_PASSWORD, NULL, 0) != CURLUE_OK) {
        goto cleanup;
    }

    result = rebuild_url(handle, suffix, out_url, out_size);

cleanup:
    curl_url_cleanup(handle);
    free(base_url);
    return result;
}

int url_strip_credentials(const char *url, char *out_url, size_t out_size) {
    char *base_url = NULL;
    const char *suffix = NULL;
    CURLU *handle = NULL;
    int result = -1;

    if (!url || !out_url || out_size == 0) {
        return -1;
    }

    handle = parse_url_handle(url, &base_url, &suffix);
    if (!handle) {
        return manual_strip_credentials(url, out_url, out_size);
    }

    if (curl_url_set(handle, CURLUPART_USER, NULL, 0) != CURLUE_OK ||
        curl_url_set(handle, CURLUPART_PASSWORD, NULL, 0) != CURLUE_OK) {
        goto cleanup;
    }

    result = rebuild_url(handle, suffix, out_url, out_size);

cleanup:
    curl_url_cleanup(handle);
    free(base_url);
    return result;
}

int url_extract_credentials(const char *url, char *username,
                            size_t username_size, char *password,
                            size_t password_size) {
    char *base_url = NULL;
    CURLU *handle = NULL;
    char *curl_user = NULL;
    char *curl_password = NULL;
    int result = 0;

    if (!url || !username || !password || username_size == 0 || password_size == 0) {
        return -1;
    }

    username[0] = '\0';
    password[0] = '\0';

    handle = parse_url_handle(url, &base_url, NULL);
    if (!handle) {
        return manual_extract_credentials(url, username, username_size, password, password_size);
    }

    if (curl_url_get(handle, CURLUPART_USER, &curl_user, CURLU_URLDECODE) == CURLUE_OK && curl_user) {
        if (copy_string(curl_user, username, username_size) != 0) {
            result = -1;
            goto cleanup;
        }
    }

    if (curl_url_get(handle, CURLUPART_PASSWORD, &curl_password, CURLU_URLDECODE) == CURLUE_OK && curl_password) {
        if (copy_string(curl_password, password, password_size) != 0) {
            result = -1;
            goto cleanup;
        }
    }

cleanup:
    if (curl_user) {
        curl_free(curl_user);
    }
    if (curl_password) {
        curl_free(curl_password);
    }
    curl_url_cleanup(handle);
    free(base_url);
    return result;
}

int url_build_onvif_device_service_url(const char *url, int onvif_port,
                                       char *out_url, size_t out_size) {
    char *base_url = NULL;
    CURLU *handle = NULL;
    char *scheme = NULL;
    char port_buffer[16];
    int result = -1;

    if (!url || !out_url || out_size == 0) {
        return -1;
    }

    if (onvif_port <= 0) {
        return url_strip_credentials(url, out_url, out_size);
    }

    handle = parse_url_handle(url, &base_url, NULL);
    if (!handle) {
        return -1;
    }

    if (curl_url_get(handle, CURLUPART_SCHEME, &scheme, 0) != CURLUE_OK ||
        !scheme || (strcasecmp(scheme, "http") != 0 && strcasecmp(scheme, "https") != 0)) {
        if (curl_url_set(handle, CURLUPART_SCHEME, "http", 0) != CURLUE_OK) {
            goto cleanup;
        }
    }

    if (curl_url_set(handle, CURLUPART_USER, NULL, 0) != CURLUE_OK ||
        curl_url_set(handle, CURLUPART_PASSWORD, NULL, 0) != CURLUE_OK) {
        goto cleanup;
    }

    snprintf(port_buffer, sizeof(port_buffer), "%d", onvif_port);
    if (curl_url_set(handle, CURLUPART_PORT, port_buffer, 0) != CURLUE_OK ||
        curl_url_set(handle, CURLUPART_PATH, "/onvif/device_service", 0) != CURLUE_OK ||
        curl_url_set(handle, CURLUPART_QUERY, NULL, 0) != CURLUE_OK ||
        curl_url_set(handle, CURLUPART_FRAGMENT, NULL, 0) != CURLUE_OK) {
        goto cleanup;
    }

    result = rebuild_url(handle, NULL, out_url, out_size);

cleanup:
    if (scheme) {
        curl_free(scheme);
    }
    curl_url_cleanup(handle);
    free(base_url);
    return result;
}

int url_redact_for_logging(const char *url, char *out_url, size_t out_size) {
    if (!url || !out_url || out_size == 0) {
        return -1;
    }

    if (url_strip_credentials(url, out_url, out_size) == 0) {
        return 0;
    }

    return copy_string("[invalid-url]", out_url, out_size);
}