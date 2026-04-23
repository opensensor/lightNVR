/**
 * onvif_soap.c – shared WS-Security header generation and SOAP fault
 * parsing for ONVIF requests.
 *
 * This is the single canonical implementation of the ONVIF WS-UsernameToken
 * PasswordDigest security header.  All ONVIF subsystems (device management,
 * PTZ, detection) must use onvif_create_security_header() instead of
 * maintaining their own copies.
 */

#include "video/onvif_soap.h"
#include "core/logger.h"
#include "utils/strings.h"
#include "ezxml.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <sys/random.h>

#include <mbedtls/sha1.h>
#include <mbedtls/base64.h>

char *onvif_create_security_header(const char *username, const char *password) {
    if (!username || !password || username[0] == '\0' || password[0] == '\0') {
        log_error("onvif_create_security_header: username and password must be non-empty");
        return NULL;
    }

    /* ------------------------------------------------------------------ *
     * 1. Generate a 16-byte random nonce                                  *
     * ------------------------------------------------------------------ */
    const int nonce_len = 16;
    unsigned char nonce_bytes[16];

    if (getrandom(nonce_bytes, nonce_len, 0) < 0) {
        /* Fallback to /dev/urandom if getrandom(2) is unavailable */
        FILE *urandom = fopen("/dev/urandom", "rb");
        if (!urandom) {
            log_error("onvif_create_security_header: cannot obtain random bytes");
            return NULL;
        }
        (void)fread(nonce_bytes, 1, nonce_len, urandom);
        fclose(urandom);
    }

    /* ------------------------------------------------------------------ *
     * 2. Base64-encode the nonce                                          *
     * ------------------------------------------------------------------ */
    size_t base64_nonce_buf_len = ((4 * nonce_len) / 3) + 5; /* +5: padding + NUL */
    char *base64_nonce = malloc(base64_nonce_buf_len);
    if (!base64_nonce) {
        return NULL;
    }
    size_t base64_nonce_len = 0;
    mbedtls_base64_encode((unsigned char *)base64_nonce, base64_nonce_buf_len,
                          &base64_nonce_len, nonce_bytes, nonce_len);
    base64_nonce[base64_nonce_len] = '\0';

    /* ------------------------------------------------------------------ *
     * 3. Build the ISO-8601 UTC timestamp                                 *
     * ------------------------------------------------------------------ */
    char created[30];
    time_t now;
    struct tm tm_now_buf;
    const struct tm *tm_now;
    time(&now);
    tm_now = gmtime_r(&now, &tm_now_buf);
    strftime(created, sizeof(created), "%Y-%m-%dT%H:%M:%S.000Z", tm_now);

    /* ------------------------------------------------------------------ *
     * 4. Compute PasswordDigest = Base64(SHA-1(nonce_raw || created || password))
     *
     * Per WS-UsernameToken Profile 1.0 §3.1 the concatenation uses the
     * *raw* (decoded) nonce bytes, not the Base64 representation.
     * The null terminator is NOT included in any part of the input.
     * ------------------------------------------------------------------ */
    size_t created_len  = strlen(created);
    size_t password_len = strlen(password);
    size_t concat_len   = (size_t)nonce_len + created_len + password_len;

    char *concatenated = malloc(concat_len + 1); /* +1 to allow safe NUL write */
    if (!concatenated) {
        free(base64_nonce);
        return NULL;
    }
    memcpy(concatenated,                              nonce_bytes, nonce_len);   /* NOLINT */
    memcpy(concatenated + nonce_len,                  created,     created_len); /* NOLINT */
    memcpy(concatenated + nonce_len + created_len,    password,    password_len);/* NOLINT */

    unsigned char digest[20]; /* SHA-1 output is always 20 bytes */
    mbedtls_sha1((unsigned char *)concatenated, concat_len, digest);
    free(concatenated);

    /* ------------------------------------------------------------------ *
     * 5. Base64-encode the digest                                         *
     * ------------------------------------------------------------------ */
    size_t base64_digest_buf_len = ((4 * 20) / 3) + 5;
    char *base64_digest = malloc(base64_digest_buf_len);
    if (!base64_digest) {
        free(base64_nonce);
        return NULL;
    }
    size_t base64_digest_len = 0;
    mbedtls_base64_encode((unsigned char *)base64_digest, base64_digest_buf_len,
                          &base64_digest_len, digest, 20);
    base64_digest[base64_digest_len] = '\0';

    /* ------------------------------------------------------------------ *
     * 6. Assemble the <wsse:Security> XML element                         *
     * ------------------------------------------------------------------ */
    char *header = malloc(1024);
    if (!header) {
        free(base64_nonce);
        free(base64_digest);
        return NULL;
    }
    sprintf(header,
        "<wsse:Security s:mustUnderstand=\"1\" "
            "xmlns:wsse=\"http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-secext-1.0.xsd\" "
            "xmlns:wsu=\"http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-utility-1.0.xsd\">"
            "<wsse:UsernameToken wsu:Id=\"UsernameToken-1\">"
                "<wsse:Username>%s</wsse:Username>"
                "<wsse:Password Type=\"http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-username-token-profile-1.0#PasswordDigest\">%s</wsse:Password>"
                "<wsse:Nonce EncodingType=\"http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-soap-message-security-1.0#Base64Binary\">%s</wsse:Nonce>"
                "<wsu:Created>%s</wsu:Created>"
            "</wsse:UsernameToken>"
        "</wsse:Security>",
        username, base64_digest, base64_nonce, created);

    free(base64_nonce);
    free(base64_digest);

    return header;
}

/* ----------------------------------------------------------------------- *
 * SOAP Fault parsing                                                       *
 * ----------------------------------------------------------------------- */

/**
 * Try to find a child element under multiple namespace prefixes.
 * Returns the first match, or NULL if none found.
 */
static ezxml_t find_with_ns(ezxml_t parent, const char *local_name,
                            const char **ns_prefixes, int ns_count) {
    if (!parent) return NULL;
    for (int i = 0; i < ns_count; i++) {
        char qname[256];
        snprintf(qname, sizeof(qname), "%s:%s", ns_prefixes[i], local_name);
        ezxml_t child = ezxml_child(parent, qname);
        if (child) return child;
    }
    /* Try without namespace prefix as a last resort */
    return ezxml_child(parent, local_name);
}

void onvif_log_soap_fault(const char *response, size_t response_len, const char *context) {
    if (!response || response_len == 0) return;

    const char *ctx = context ? context : "ONVIF";

    /* ezxml_parse_str modifies the buffer in-place, so make a copy */
    char *buf = malloc(response_len + 1);
    if (!buf) {
        /* Fallback: just log raw truncated response */
        char snippet[256];
        size_t n = response_len < sizeof(snippet) - 1 ? response_len : sizeof(snippet) - 1;
        memcpy(snippet, response, n);
        snippet[n] = '\0';
        log_error("[%s] SOAP fault (raw): %s", ctx, snippet);
        return;
    }
    memcpy(buf, response, response_len);
    buf[response_len] = '\0';

    ezxml_t xml = ezxml_parse_str(buf, response_len);
    if (!xml) {
        /* Could not parse XML at all — log raw snippet */
        char snippet[512];
        size_t n = response_len < sizeof(snippet) - 1 ? response_len : sizeof(snippet) - 1;
        memcpy(snippet, response, n);
        snippet[n] = '\0';
        log_error("[%s] SOAP fault response (unparseable): %s", ctx, snippet);
        free(buf);
        return;
    }

    /* Namespace prefixes cameras may use for the SOAP envelope */
    const char *env_ns[] = {"s", "S", "SOAP-ENV", "soap", "env"};
    int env_ns_count = sizeof(env_ns) / sizeof(env_ns[0]);

    /* Find Body */
    ezxml_t body = find_with_ns(xml, "Body", env_ns, env_ns_count);

    /* Find Fault */
    ezxml_t fault = body ? find_with_ns(body, "Fault", env_ns, env_ns_count) : NULL;

    if (!fault) {
        /* No Fault element found — log raw snippet */
        char snippet[512];
        size_t n = response_len < sizeof(snippet) - 1 ? response_len : sizeof(snippet) - 1;
        memcpy(snippet, response, n);
        snippet[n] = '\0';
        log_error("[%s] SOAP error response (no Fault element): %s", ctx, snippet);
        ezxml_free(xml);
        free(buf);
        return;
    }

    /* Extract Code > Value */
    const char *code_str = NULL;
    const char *subcode_str = NULL;
    ezxml_t code_elem = find_with_ns(fault, "Code", env_ns, env_ns_count);
    if (code_elem) {
        ezxml_t value_elem = find_with_ns(code_elem, "Value", env_ns, env_ns_count);
        if (value_elem) code_str = ezxml_txt(value_elem);

        ezxml_t subcode_elem = find_with_ns(code_elem, "Subcode", env_ns, env_ns_count);
        if (subcode_elem) {
            ezxml_t sub_value = find_with_ns(subcode_elem, "Value", env_ns, env_ns_count);
            if (sub_value) subcode_str = ezxml_txt(sub_value);
        }
    }

    /* Extract Reason > Text */
    const char *reason_str = NULL;
    ezxml_t reason_elem = find_with_ns(fault, "Reason", env_ns, env_ns_count);
    if (reason_elem) {
        ezxml_t text_elem = find_with_ns(reason_elem, "Text", env_ns, env_ns_count);
        if (text_elem) reason_str = ezxml_txt(text_elem);
    }

    /* Also try faultstring (SOAP 1.1 style) if no Reason was found */
    if (!reason_str || reason_str[0] == '\0') {
        ezxml_t faultstring = ezxml_child(fault, "faultstring");
        if (faultstring) reason_str = ezxml_txt(faultstring);
    }

    bool have_reason = (reason_str && reason_str[0] != '\0');

    /* Build and log the message */
    if (code_str && code_str[0] != '\0') {
        if (subcode_str && subcode_str[0] != '\0') {
            log_error("[%s] SOAP Fault: Code=%s, Subcode=%s, Reason=%s",
                      ctx,
                      code_str,
                      subcode_str,
                      have_reason ? reason_str : "(none)");
        } else {
            log_error("[%s] SOAP Fault: Code=%s, Reason=%s",
                      ctx,
                      code_str,
                      have_reason ? reason_str : "(none)");
        }
    } else if (have_reason) {
        log_error("[%s] SOAP Fault: %s", ctx, reason_str);
    } else {
        /* Could not extract anything useful — log raw */
        char snippet[512];
        size_t n = response_len < sizeof(snippet) - 1 ? response_len : sizeof(snippet) - 1;
        memcpy(snippet, response, n);
        snippet[n] = '\0';
        log_error("[%s] SOAP Fault (could not extract details): %s", ctx, snippet);
    }

    /* Extra diagnostic: when the structured fault lacks a Reason — as happens
     * with strict ONVIF servers like Reolink (#374) — dump a snippet of the
     * raw XML so future bug reports include enough for us to spot vendor
     * quirks (Subcode values, Detail elements, etc.) without needing a pcap. */
    if (!have_reason && code_str && code_str[0] != '\0') {
        char snippet[768];
        size_t n = response_len < sizeof(snippet) - 1 ? response_len : sizeof(snippet) - 1;
        memcpy(snippet, response, n);
        snippet[n] = '\0';
        log_error("[%s] SOAP Fault raw XML (no Reason/Text): %s", ctx, snippet);
    }

    ezxml_free(xml);
    free(buf);
}
