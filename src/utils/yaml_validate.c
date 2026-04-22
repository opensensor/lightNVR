/**
 * @file yaml_validate.c
 * @brief libyaml-backed parse-only YAML validator (see yaml_validate.h).
 *
 * When libyaml is available (LIGHTNVR_HAVE_LIBYAML), we run
 * `yaml_parser_parse` in event mode so we pay the price of exactly one
 * lexer+parser pass with zero document-tree construction. Events are
 * streamed until STREAM_END (or an error). Root-node classification is
 * done by peeking the first event after STREAM_START + DOCUMENT_START.
 *
 * When libyaml is NOT available, we provide stubs that return a
 * "validation disabled" answer so callers degrade gracefully.
 */

#include "utils/yaml_validate.h"

#include <stdarg.h>
#include <stddef.h>
#include <string.h>
#include <stdio.h>

#if defined(LIGHTNVR_HAVE_LIBYAML)
#include <yaml.h>
#endif

/* --------------------------------------------------------------
 * Helpers
 * -------------------------------------------------------------- */

static void set_err(char *err_buf, size_t err_size, const char *msg)
{
    if (!err_buf || err_size == 0 || !msg) {
        return;
    }
    size_t n = strlen(msg);
    if (n >= err_size) {
        n = err_size - 1;
    }
    memcpy(err_buf, msg, n);
    err_buf[n] = '\0';
}

/* --------------------------------------------------------------
 * Public API — libyaml path
 * -------------------------------------------------------------- */

#if defined(LIGHTNVR_HAVE_LIBYAML)

int yaml_validate_is_available(void)
{
    return 1;
}

int yaml_validate_str(const char *src, size_t len, char *err_buf, size_t err_size)
{
    if (err_buf && err_size > 0) {
        err_buf[0] = '\0';
    }

    /* Treat NULL/empty as a valid empty document. libyaml will happily
     * parse a zero-byte input, but we short-circuit to avoid hitting
     * any quirks. */
    if (!src || len == 0) {
        return 0;
    }

    yaml_parser_t parser;
    if (!yaml_parser_initialize(&parser)) {
        set_err(err_buf, err_size, "failed to initialize YAML parser");
        return -1;
    }

    yaml_parser_set_input_string(&parser, (const unsigned char *)src, len);

    int rc = 0;
    for (;;) {
        yaml_event_t event;
        if (!yaml_parser_parse(&parser, &event)) {
            /* Parse error — format with line/column when available. */
            const char *problem = parser.problem ? parser.problem : "YAML parse error";
            const char *context = parser.context ? parser.context : NULL;
            /* parser.problem_mark.line/column are 0-based; display 1-based. */
            size_t line = parser.problem_mark.line + 1;
            size_t column = parser.problem_mark.column + 1;

            if (err_buf && err_size > 0) {
                if (context) {
                    snprintf(err_buf, err_size,
                             "YAML parse error: %s (%s) at line %zu, column %zu",
                             problem, context, line, column);
                } else {
                    snprintf(err_buf, err_size,
                             "YAML parse error: %s at line %zu, column %zu",
                             problem, line, column);
                }
            }
            rc = -1;
            break;
        }

        int done = (event.type == YAML_STREAM_END_EVENT);
        yaml_event_delete(&event);
        if (done) {
            break;
        }
    }

    yaml_parser_delete(&parser);
    return rc;
}

int yaml_is_mapping_root(const char *src, size_t len)
{
    if (!src || len == 0) {
        return 0;
    }

    yaml_parser_t parser;
    if (!yaml_parser_initialize(&parser)) {
        return -1;
    }

    yaml_parser_set_input_string(&parser, (const unsigned char *)src, len);

    int result = 0;   /* not-a-mapping unless proven otherwise */
    int saw_stream_start = 0;
    int saw_document_start = 0;

    for (;;) {
        yaml_event_t event;
        if (!yaml_parser_parse(&parser, &event)) {
            result = -1;
            break;
        }

        yaml_event_type_t t = event.type;
        yaml_event_delete(&event);

        switch (t) {
        case YAML_STREAM_START_EVENT:
            saw_stream_start = 1;
            break;
        case YAML_DOCUMENT_START_EVENT:
            saw_document_start = 1;
            break;
        case YAML_MAPPING_START_EVENT:
            if (saw_stream_start && saw_document_start) {
                result = 1;
            }
            goto done;
        case YAML_SCALAR_EVENT:
        case YAML_SEQUENCE_START_EVENT:
        case YAML_ALIAS_EVENT:
            /* First node after DOCUMENT_START is not a mapping. */
            goto done;
        case YAML_DOCUMENT_END_EVENT:
        case YAML_STREAM_END_EVENT:
            /* Empty document — not a mapping. */
            goto done;
        default:
            /* NO_EVENT, MAPPING_END, SEQUENCE_END — keep scanning. */
            break;
        }
    }

done:
    yaml_parser_delete(&parser);
    return result;
}

/* ---- T6: go2rtc override semantic validator -------------------- */

/* Known go2rtc top-level sections.  Anything outside this list produces a
 * warning (not an error) so a future go2rtc release adding a section still
 * works without a lightNVR change.  Keep alphabetized; mirrors the Key
 * Sections table in go2rtc-override-plan.md. */
static const char *const k_known_go2rtc_sections[] = {
    "api", "app", "echo", "ffmpeg", "hass", "hls", "homekit",
    "log", "mqtt", "ngrok", "pinggy", "preload", "publish",
    "rtsp", "srtp", "streams", "webrtc", "webtorrent",
    NULL,
};

static int section_is_known(const char *name)
{
    if (!name) return 0;
    for (int i = 0; k_known_go2rtc_sections[i]; i++) {
        if (strcmp(name, k_known_go2rtc_sections[i]) == 0) {
            return 1;
        }
    }
    return 0;
}

static void result_add_warning(yaml_validation_result_t *r, const char *fmt, ...)
{
    if (!r || r->warning_count >= YAML_VALIDATE_MAX_WARNINGS) {
        return;
    }
    va_list ap;
    va_start(ap, fmt);
    vsnprintf(r->warnings[r->warning_count],
              YAML_VALIDATE_WARN_LEN, fmt, ap);
    va_end(ap);
    r->warning_count++;
}

/* Bounded set of seen top-level keys.  go2rtc sections top out around 18; we
 * round up to 64 so a malformed input with hundreds of duplicates can't blow
 * the stack but we still record the first 64 distinctly.  Any duplicate of a
 * key already in the set sets `duplicate_keys` and records the location of
 * the second occurrence as the (sole) reported error. */
#define SEEN_KEYS_MAX 64
#define SEEN_KEY_LEN  96

void yaml_validate_go2rtc_override(const char *src, size_t len,
                                   yaml_validation_result_t *out)
{
    if (!out) return;
    memset(out, 0, sizeof(*out));
    out->valid = 1;

    if (!src || len == 0) {
        return;  /* empty override is valid */
    }

    yaml_parser_t parser;
    if (!yaml_parser_initialize(&parser)) {
        out->valid = 0;
        out->parse_error = 1;
        snprintf(out->err_message, sizeof(out->err_message),
                 "failed to initialize YAML parser");
        return;
    }
    yaml_parser_set_input_string(&parser, (const unsigned char *)src, len);

    /* Depth model:
     *   STREAM_START / DOCUMENT_START don't change depth.
     *   MAPPING_START / SEQUENCE_START increment depth.
     *   MAPPING_END   / SEQUENCE_END   decrement depth.
     *   At depth==1 (immediately inside the root mapping), a SCALAR is a
     *   key when expecting_key is true, then expecting_key flips.  Sequences
     *   and nested mappings as values flip back to expecting_key when they
     *   close (because depth returns to 1).
     *
     * We rely on libyaml emitting events in the canonical order — within a
     * mapping it always alternates key, value, key, value, ... so tracking
     * `expecting_key` at depth==1 is enough to catch every top-level key. */
    int depth = 0;
    int saw_root_mapping = 0;
    int expecting_key_at_depth_1 = 0;

    char seen[SEEN_KEYS_MAX][SEEN_KEY_LEN];
    int seen_count = 0;

    int rc = 0;
    for (;;) {
        yaml_event_t event;
        if (!yaml_parser_parse(&parser, &event)) {
            const char *problem = parser.problem ? parser.problem : "YAML parse error";
            const char *context = parser.context ? parser.context : NULL;
            size_t line = parser.problem_mark.line + 1;
            size_t column = parser.problem_mark.column + 1;
            out->valid = 0;
            out->parse_error = 1;
            out->err_line = (int)line;
            out->err_column = (int)column;
            if (context) {
                snprintf(out->err_message, sizeof(out->err_message),
                         "YAML parse error: %s (%s) at line %zu, column %zu",
                         problem, context, line, column);
            } else {
                snprintf(out->err_message, sizeof(out->err_message),
                         "YAML parse error: %s at line %zu, column %zu",
                         problem, line, column);
            }
            rc = -1;
            break;
        }

        switch (event.type) {
        case YAML_STREAM_START_EVENT:
        case YAML_DOCUMENT_START_EVENT:
            break;

        case YAML_MAPPING_START_EVENT:
            depth++;
            if (depth == 1) {
                saw_root_mapping = 1;
                expecting_key_at_depth_1 = 1;
            }
            break;

        case YAML_MAPPING_END_EVENT:
            depth--;
            if (depth == 1) {
                /* Closed a nested mapping that was a top-level value;
                 * back to expecting a key. */
                expecting_key_at_depth_1 = 1;
            }
            break;

        case YAML_SEQUENCE_START_EVENT:
            depth++;
            break;

        case YAML_SEQUENCE_END_EVENT:
            depth--;
            if (depth == 1) {
                expecting_key_at_depth_1 = 1;
            }
            break;

        case YAML_SCALAR_EVENT:
            if (depth == 1 && expecting_key_at_depth_1) {
                const char *key = (const char *)event.data.scalar.value;
                size_t key_len = event.data.scalar.length;
                size_t klen = key_len < SEEN_KEY_LEN - 1 ? key_len
                                                        : SEEN_KEY_LEN - 1;

                /* Check for duplicate. */
                int dup = 0;
                for (int i = 0; i < seen_count; i++) {
                    if (strncmp(seen[i], key, klen) == 0
                        && seen[i][klen] == '\0') {
                        dup = 1;
                        break;
                    }
                }
                if (dup) {
                    out->duplicate_keys = 1;
                    /* Only record the first duplicate as the reported error
                     * so the user sees the most actionable location. */
                    if (out->valid) {
                        out->valid = 0;
                        out->err_line =
                            (int)(event.start_mark.line + 1);
                        out->err_column =
                            (int)(event.start_mark.column + 1);
                        snprintf(out->err_message, sizeof(out->err_message),
                                 "duplicate top-level key '%.*s' at line %d, "
                                 "column %d — go2rtc's YAML parser rejects "
                                 "duplicate mapping keys",
                                 (int)klen, key,
                                 out->err_line, out->err_column);
                    }
                } else if (seen_count < SEEN_KEYS_MAX) {
                    memcpy(seen[seen_count], key, klen);
                    seen[seen_count][klen] = '\0';
                    if (!section_is_known(seen[seen_count])) {
                        result_add_warning(out,
                            "unknown top-level section '%s' "
                            "(not in known go2rtc sections; will be passed "
                            "through but may be a typo)",
                            seen[seen_count]);
                    }
                    seen_count++;
                }

                expecting_key_at_depth_1 = 0;
            } else if (depth == 1 && !expecting_key_at_depth_1) {
                /* Scalar value of a top-level key; next scalar at depth==1
                 * will be the next key. */
                expecting_key_at_depth_1 = 1;
            } else if (depth == 0) {
                /* Root is a scalar, not a mapping. */
                out->non_mapping_root = 1;
                if (out->valid) {
                    out->valid = 0;
                    snprintf(out->err_message, sizeof(out->err_message),
                             "go2rtc override must be a top-level YAML "
                             "mapping (got a scalar)");
                }
            }
            break;

        case YAML_ALIAS_EVENT:
            if (depth == 1 && expecting_key_at_depth_1) {
                /* Aliases as keys are technically legal but useless here
                 * and confuse our seen-set tracking — flag as warning. */
                result_add_warning(out,
                    "YAML alias used as a top-level key — "
                    "duplicate-key detection cannot follow this");
                expecting_key_at_depth_1 = 0;
            } else if (depth == 1) {
                expecting_key_at_depth_1 = 1;
            }
            break;

        case YAML_DOCUMENT_END_EVENT:
            /* If we never saw a root mapping AND no other error fired,
             * this is a non-mapping root (sequence, scalar, or empty). */
            if (!saw_root_mapping && out->valid) {
                out->non_mapping_root = 1;
                out->valid = 0;
                snprintf(out->err_message, sizeof(out->err_message),
                         "go2rtc override must be a top-level YAML mapping "
                         "(got a non-mapping document)");
            }
            break;

        case YAML_STREAM_END_EVENT:
            yaml_event_delete(&event);
            goto done;

        default:
            break;
        }

        yaml_event_delete(&event);
    }

done:
    yaml_parser_delete(&parser);
    (void)rc;
}

#else /* !LIGHTNVR_HAVE_LIBYAML — stub path */

int yaml_validate_is_available(void)
{
    return 0;
}

int yaml_validate_str(const char *src, size_t len, char *err_buf, size_t err_size)
{
    (void)src;
    (void)len;
    set_err(err_buf, err_size, "libyaml not available; validation disabled");
    return 0;
}

int yaml_is_mapping_root(const char *src, size_t len)
{
    (void)src;
    (void)len;
    return 0;
}

void yaml_validate_go2rtc_override(const char *src, size_t len,
                                   yaml_validation_result_t *out)
{
    (void)src;
    (void)len;
    if (!out) return;
    memset(out, 0, sizeof(*out));
    out->valid = -1;
    snprintf(out->err_message, sizeof(out->err_message),
             "libyaml not available at build time; "
             "pre-save validation skipped");
}

#endif /* LIGHTNVR_HAVE_LIBYAML */
