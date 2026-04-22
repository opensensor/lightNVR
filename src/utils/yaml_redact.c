/**
 * @file yaml_redact.c
 * @brief YAML-aware secret redactor (see yaml_redact.h).
 *
 * Implementation strategy: parse the document into an event stream, walk
 * the stream while tracking the (path-stack) we are at, and re-emit each
 * event back through a libyaml emitter.  When the current path matches a
 * known-secret rule, we substitute "<redacted>" for the scalar value.
 *
 * Tracking the path:
 *
 *   stack[i] is a frame describing one nested container.
 *
 *   For a mapping frame:
 *     `key`             - latest seen key (i.e., the key whose VALUE we're
 *                         currently emitting if expecting_key == 0)
 *     `expecting_key`   - 1 when the next SCALAR/MAPPING_START/SEQUENCE_START
 *                         in this mapping is a KEY; 0 when it's a VALUE.
 *
 *   For a sequence frame:
 *     `seq_index`       - the index of the next item to be emitted.
 *
 * After every emitted child the parent frame's expecting_key flips
 * (mapping) or seq_index increments (sequence).
 *
 * Note: `streams.*` values are URLs and we want to keep the host/path
 * visible while masking only the userinfo (`user:pass@`).  We mutate the
 * scalar string before emitting it.
 */

#include "utils/yaml_redact.h"

#include <stdint.h>
#include <stdlib.h>
#include <stddef.h>
#include <string.h>
#include <stdio.h>

#if defined(LIGHTNVR_HAVE_LIBYAML)
#include <yaml.h>
#endif

#define REDACTED_LITERAL "<redacted>"

#if defined(LIGHTNVR_HAVE_LIBYAML)

/* ------------------------------------------------------------------
 * Path stack
 * ------------------------------------------------------------------ */

#define STACK_MAX 32
#define KEY_LEN   128

typedef enum { FRAME_MAP, FRAME_SEQ } frame_type_t;

typedef struct {
    frame_type_t type;
    char         key[KEY_LEN];   /* mapping: current key name */
    int          expecting_key;  /* mapping: 1 if next event is a key */
    int          seq_index;      /* sequence: next index */
} stack_frame_t;

static int frame_keys_equal(const stack_frame_t *f, const char *literal)
{
    return f->type == FRAME_MAP && strncmp(f->key, literal, KEY_LEN) == 0;
}

/**
 * Decide whether the value about to be emitted at the top of @p stack should
 * be fully replaced by `<redacted>`.
 */
static int path_matches_full_redact(const stack_frame_t *stack, int depth)
{
    if (depth < 2) return 0;

    /* api.{password,username}, rtsp.{password,username}, mqtt.password */
    if (depth == 2) {
        const stack_frame_t *root = &stack[0];
        const stack_frame_t *sub  = &stack[1];
        if (root->type != FRAME_MAP || sub->type != FRAME_MAP) return 0;

        if ((frame_keys_equal(root, "api") || frame_keys_equal(root, "rtsp"))
            && (frame_keys_equal(sub, "password") || frame_keys_equal(sub, "username"))) {
            return 1;
        }
        if (frame_keys_equal(root, "mqtt") && frame_keys_equal(sub, "password")) {
            return 1;
        }
    }

    /* webrtc.ice_servers[*].credential, webrtc.ice_servers[*].username */
    if (depth == 4
        && stack[0].type == FRAME_MAP
        && stack[1].type == FRAME_MAP
        && stack[2].type == FRAME_SEQ
        && stack[3].type == FRAME_MAP
        && frame_keys_equal(&stack[0], "webrtc")
        && frame_keys_equal(&stack[1], "ice_servers")
        && (frame_keys_equal(&stack[3], "credential")
            || frame_keys_equal(&stack[3], "username"))) {
        return 1;
    }

    return 0;
}

/**
 * `streams.<name>` values are typically URLs; mask only the userinfo.
 * Returns true when the current value is a streams entry (depth==2 with
 * `streams` at root).  Sub-keys inside complex stream definitions
 * (sequences, etc.) are left alone — those rarely contain credentials.
 */
static int path_is_streams_value(const stack_frame_t *stack, int depth)
{
    return depth == 2
        && stack[0].type == FRAME_MAP
        && frame_keys_equal(&stack[0], "streams")
        && stack[1].type == FRAME_MAP;
}

/* ------------------------------------------------------------------
 * URL userinfo masking
 *
 * Pattern:  scheme://[userinfo@]host[:port][/path][?query]
 * userinfo: <user>[:<pass>]
 *
 * We rewrite by replacing the userinfo span with "<redacted>" iff there is
 * a ':' inside it (suggesting a password); a username-only userinfo is
 * preserved.  No regex — single forward scan.
 *
 * Returns a malloc'd NUL-terminated copy of @p src with userinfo masked,
 * or NULL on OOM.  When no userinfo is found, returns a verbatim copy.
 * ------------------------------------------------------------------ */

static char *mask_url_userinfo_alloc(const char *src)
{
    if (!src) return NULL;

    /* Worst case the masked string is shorter than the original (because we
     * replace `user:pass` with "<redacted>"); allocate input length + the
     * literal length to be safe. */
    size_t in_len = strlen(src);
    size_t cap = in_len + sizeof(REDACTED_LITERAL) + 1;
    char *out = malloc(cap);
    if (!out) return NULL;

    const char *p = src;
    char *o = out;
    size_t remaining = cap;

    /* Find "://" */
    const char *scheme_end = strstr(p, "://");
    if (!scheme_end) {
        snprintf(out, cap, "%s", src);
        return out;
    }

    /* Copy through "://" inclusive. */
    size_t prefix_len = (size_t)(scheme_end - p) + 3;
    if (prefix_len >= remaining) { free(out); return NULL; }
    memcpy(o, p, prefix_len);
    o += prefix_len;
    remaining -= prefix_len;
    p += prefix_len;

    /* Find authority end ('/', '?', '#', or end of string). */
    const char *authority_end = p;
    while (*authority_end && *authority_end != '/' &&
           *authority_end != '?' && *authority_end != '#') {
        authority_end++;
    }

    /* Find '@' inside [p, authority_end). */
    const char *at = NULL;
    for (const char *q = p; q < authority_end; q++) {
        if (*q == '@') { at = q; break; }
    }

    if (!at) {
        /* No userinfo. */
        size_t rest_len = strlen(p);
        if (rest_len + 1 > remaining) { free(out); return NULL; }
        memcpy(o, p, rest_len);
        o[rest_len] = '\0';
        return out;
    }

    /* Check for ':' in [p, at). */
    int has_colon = 0;
    for (const char *q = p; q < at; q++) {
        if (*q == ':') { has_colon = 1; break; }
    }

    if (has_colon) {
        /* Replace the userinfo span entirely. */
        const char *replacement = REDACTED_LITERAL;
        size_t rep_len = strlen(replacement);
        if (rep_len + 1 > remaining) { free(out); return NULL; }
        memcpy(o, replacement, rep_len);
        o += rep_len;
        remaining -= rep_len;
    } else {
        /* Username only — keep it (most ops want to see "rtsp://admin@cam"
         * for diagnostics; a true secret would be in the password slot). */
        size_t user_len = (size_t)(at - p);
        if (user_len + 1 > remaining) { free(out); return NULL; }
        memcpy(o, p, user_len);
        o += user_len;
        remaining -= user_len;
    }

    /* Copy from '@' onward (inclusive). */
    size_t tail_len = strlen(at);
    if (tail_len + 1 > remaining) { free(out); return NULL; }
    memcpy(o, at, tail_len);
    o[tail_len] = '\0';
    return out;
}

/* ------------------------------------------------------------------
 * Output sink — yaml_emitter_t writing into a growable buffer
 * ------------------------------------------------------------------ */

typedef struct {
    char  *buf;
    size_t len;
    size_t cap;
    int    oom;
} growbuf_t;

static int growbuf_write(void *data, unsigned char *buffer, size_t size)
{
    growbuf_t *g = (growbuf_t *)data;
    if (g->oom) return 0;
    if (g->len + size + 1 > g->cap) {
        size_t new_cap = g->cap ? g->cap * 2 : 4096;
        while (new_cap < g->len + size + 1) new_cap *= 2;
        char *p = realloc(g->buf, new_cap);
        if (!p) { g->oom = 1; return 0; }
        g->buf = p;
        g->cap = new_cap;
    }
    memcpy(g->buf + g->len, buffer, size);
    g->len += size;
    g->buf[g->len] = '\0';
    return 1;
}

/* ------------------------------------------------------------------
 * Event walk and re-emit
 * ------------------------------------------------------------------ */

static void after_value_emitted(stack_frame_t *stack, int depth)
{
    if (depth <= 0) return;
    stack_frame_t *top = &stack[depth - 1];
    if (top->type == FRAME_MAP) {
        top->expecting_key = 1;
    } else {
        top->seq_index++;
    }
}

char *yaml_redact_alloc(const char *src, size_t len, size_t *out_len)
{
    if (out_len) *out_len = 0;

    if (!src || len == 0) {
        char *empty = malloc(1);
        if (!empty) return NULL;
        empty[0] = '\0';
        if (out_len) *out_len = 0;
        return empty;
    }

    yaml_parser_t parser;
    yaml_emitter_t emitter;
    if (!yaml_parser_initialize(&parser)) return NULL;
    if (!yaml_emitter_initialize(&emitter)) {
        yaml_parser_delete(&parser);
        return NULL;
    }
    yaml_parser_set_input_string(&parser, (const unsigned char *)src, len);

    growbuf_t out = {0};
    yaml_emitter_set_output(&emitter, growbuf_write, &out);
    /* Match the source style as closely as we can — block style and 2-space
     * indent are the lightNVR-side conventions; the emitter will fall back
     * gracefully for inputs that need flow style. */
    yaml_emitter_set_canonical(&emitter, 0);
    yaml_emitter_set_indent(&emitter, 2);
    yaml_emitter_set_unicode(&emitter, 1);

    stack_frame_t stack[STACK_MAX];
    int depth = 0;
    int ok = 1;

    for (;;) {
        yaml_event_t event;
        if (!yaml_parser_parse(&parser, &event)) {
            ok = 0;
            break;
        }

        switch (event.type) {
        case YAML_MAPPING_START_EVENT:
            if (depth >= STACK_MAX) { ok = 0; break; }
            stack[depth].type = FRAME_MAP;
            stack[depth].key[0] = '\0';
            stack[depth].expecting_key = 1;
            depth++;
            break;
        case YAML_SEQUENCE_START_EVENT:
            if (depth >= STACK_MAX) { ok = 0; break; }
            stack[depth].type = FRAME_SEQ;
            stack[depth].seq_index = 0;
            depth++;
            break;
        case YAML_MAPPING_END_EVENT:
        case YAML_SEQUENCE_END_EVENT:
            if (depth > 0) depth--;
            after_value_emitted(stack, depth);
            break;
        case YAML_SCALAR_EVENT:
            if (depth > 0) {
                stack_frame_t *top = &stack[depth - 1];
                if (top->type == FRAME_MAP && top->expecting_key) {
                    /* This scalar IS a key. */
                    size_t klen = event.data.scalar.length;
                    if (klen >= KEY_LEN) klen = KEY_LEN - 1;
                    memcpy(top->key, event.data.scalar.value, klen);
                    top->key[klen] = '\0';
                    top->expecting_key = 0;
                } else {
                    /* This scalar is a value.  Possibly redact. */
                    if (path_matches_full_redact(stack, depth)) {
                        free(event.data.scalar.value);
                        event.data.scalar.value =
                            (yaml_char_t *)strdup(REDACTED_LITERAL);
                        event.data.scalar.length =
                            event.data.scalar.value
                                ? strlen(REDACTED_LITERAL) : 0;
                        /* Force plain style for the replacement so it looks
                         * literal in the rendered YAML. */
                        event.data.scalar.style = YAML_PLAIN_SCALAR_STYLE;
                    } else if (path_is_streams_value(stack, depth)) {
                        char *masked = mask_url_userinfo_alloc(
                            (const char *)event.data.scalar.value);
                        if (masked) {
                            free(event.data.scalar.value);
                            event.data.scalar.value = (yaml_char_t *)masked;
                            event.data.scalar.length = strlen(masked);
                        }
                    }
                    after_value_emitted(stack, depth);
                }
            }
            break;
        default:
            break;
        }

        if (!ok) {
            yaml_event_delete(&event);
            break;
        }

        int is_end = (event.type == YAML_STREAM_END_EVENT);
        if (!yaml_emitter_emit(&emitter, &event)) {
            ok = 0;
            break;
        }
        if (is_end) break;
    }

    yaml_emitter_flush(&emitter);
    yaml_emitter_delete(&emitter);
    yaml_parser_delete(&parser);

    if (!ok || out.oom) {
        free(out.buf);
        return NULL;
    }
    if (!out.buf) {
        out.buf = malloc(1);
        if (!out.buf) return NULL;
        out.buf[0] = '\0';
    }
    if (out_len) *out_len = out.len;
    return out.buf;
}

int yaml_redact_is_available(void) { return 1; }

#else /* !LIGHTNVR_HAVE_LIBYAML */

char *yaml_redact_alloc(const char *src, size_t len, size_t *out_len)
{
    char *copy = malloc(len + 1);
    if (!copy) return NULL;
    if (src && len) memcpy(copy, src, len);
    copy[len] = '\0';
    if (out_len) *out_len = len;
    return copy;
}

int yaml_redact_is_available(void) { return 0; }

#endif /* LIGHTNVR_HAVE_LIBYAML */
