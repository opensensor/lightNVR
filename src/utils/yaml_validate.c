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

#endif /* LIGHTNVR_HAVE_LIBYAML */
